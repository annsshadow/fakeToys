## dm-log-writes


该 target 接收 2 个设备，一个用于正常转发所有 IO，另一个用于记录所有
写操作。其面向希望在文件系统写入过程中验证元数据或数据完整性的
文件系统开发者。每次 WRITE 请求都会写入一个 log_write_entry，并且该
target 能够从用户空间获取任意数据插入到日志中。WRITE 请求中的数据会被
复制到日志里，从而使重放能够完全按照原始发生的顺序进行。

## Log Ordering


我们按照完成的顺序记录，前提是我们确认该写操作已不在缓存中。
这意味着普通的 WRITE 请求实际上要等到下一次 REQ_PREFLUSH 请求
出现时才会被记录。这样做是为了让用户空间能够以与磁盘上（而非缓存中）
一致的方式重放日志，从而更容易发现不正确的等待/刷新行为。

其工作方式是将所有 WRITE 请求在写完成后挂到一个链表上。一旦看到
REQ_PREFLUSH 请求，我们就把该链表拼接进请求，待 FLUSH 请求完成后，
我们记录所有 WRITE 以及随后的 FLUSH。只有在 REQ_PREFLUSH 发起时
已经完成的 WRITE 才会被按顺序加入，以模拟断电情况下的最坏场景。
考虑下面这个例子（W 表示写入，C 表示完成）：

	W1,W2,W3,C3,C2,Wflush,C1,Cflush

日志中显示的将是：

	W3,W2,flush,W1....

同样，这也是为了模拟磁盘上的真实情况，从而让我们能够检测
在某个特定时刻发生断电会导致文件系统不一致的情况。

任何 REQ_FUA 请求会绕过该刷新机制，并在其完成后立即被记录，
因为这些请求显然会绕过设备缓存。

任何 REQ_OP_DISCARD 请求都被当作 WRITE 请求处理。否则我们就会
先记录所有的 DISCARD 请求，然后是 WRITE 请求，最后才是 FLUSH
请求。考虑下面的例子：

	WRITE block 1, DISCARD block 1, FLUSH

如果我们按 DISCARD 完成时记录，重放看起来会是这样：

	DISCARD 1, WRITE 1, FLUSH

这与实际发生的情况并不完全相符，也会在日志重放中被漏掉。

## Target interface


i) 构造函数

   log-writes <dev_path> <log_dev_path>

   ============= ==============================================
   dev_path	 所有 IO 正常转发到的设备。
   log_dev_path  日志条目写入到的设备。
   ============= ==============================================

ii) 状态

    <#logged entries> <highest allocated sector>

    =========================== ========================
    #logged entries	         已记录的条目数量
    highest allocated sector    已分配的最高扇区
    =========================== ========================

iii) 消息

    mark <description>

	你可以使用 dmsetup message 在日志中设置一个任意标记。
	例如，假设你想在每次写入后都对文件系统进行 fsck，但首先
	需要重放到 mkfs 以确保我们 fsck 的对象是合理的，你可以
	做类似这样的事情
```

	  mkfs.btrfs -f /dev/mapper/log
	  dmsetup message log 0 mark mkfs
	  <run test>

	This would allow you to replay the log up to the mkfs mark and
	then replay from that point on doing the fsck check in the
	interval that you want.

	Every log has a mark at the end labeled "dm-log-writes-end".

```
## Userspace component


有一个用户空间工具可以用多种方式为你重放日志。
它可以在这里找到：https://github.com/josefbacik/log-writes

## Example usage


假设你想测试文件系统上的 fsync。你会做类似这样的事情
```

  TABLE="0 $(blockdev --getsz /dev/sdb) log-writes /dev/sdb /dev/sdc"
  dmsetup create log --table "$TABLE"
  mkfs.btrfs -f /dev/mapper/log
  dmsetup message log 0 mark mkfs

  mount /dev/mapper/log /mnt/btrfs-test
  <some test that does fsync at the end>
  dmsetup message log 0 mark fsync
  md5sum /mnt/btrfs-test/foo
  umount /mnt/btrfs-test

  dmsetup remove log
  replay-log --log /dev/sdc --replay /dev/sdb --end-mark fsync
  mount /dev/sdb /mnt/btrfs-test
  md5sum /mnt/btrfs-test/foo
  <verify md5sum's are correct>

  Another option is to do a complicated file system operation and verify the file
  system is consistent during the entire operation.  You could do this with:

  TABLE="0 $(blockdev --getsz /dev/sdb) log-writes /dev/sdb /dev/sdc"
  dmsetup create log --table "$TABLE"
  mkfs.btrfs -f /dev/mapper/log
  dmsetup message log 0 mark mkfs

  mount /dev/mapper/log /mnt/btrfs-test
  <fsstress to dirty the fs>
  btrfs filesystem balance /mnt/btrfs-test
  umount /mnt/btrfs-test
  dmsetup remove log

  replay-log --log /dev/sdc --replay /dev/sdb --end-mark mkfs
  btrfsck /dev/sdb
  replay-log --log /dev/sdc --replay /dev/sdb --start-mark mkfs \
	--fsck "btrfsck /dev/sdb" --check fua

```
它会一直重放日志直到遇到一个 FUA 请求，运行 fsck 命令，如果
fsck 通过，则重放到下一个 FUA，直到全部完成或 fsck 命令异常退出。
