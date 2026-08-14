
## Amiga 文件系统概述


并非所有种类的 Amiga 文件系统都支持读写。Amiga 目前有六种不同文件系统：

==============	===============================================================
DOS\0		原始（最早期）文件系统，并不适合硬盘，通常也不在硬盘上使用。
		支持读/写。

DOS\1		原始快速文件系统（Fast File System）。支持读/写。

DOS\2		旧版“国际化”文件系统。所谓“国际化”是指修复了一个缺陷，使
		文件名中带重音的（“国际化”）字母不区分大小写（本应如此）。
		支持读/写。

DOS\3		“国际化”快速文件系统。支持读/写。

DOS\4		带目录缓存的原始文件系统。目录缓存可显著加快软盘上的目录访问，
		但会拖慢文件的创建/删除。在硬盘上意义不大。仅支持读。

DOS\5		带目录缓存的快速文件系统。仅支持读。
==============	===============================================================

上述所有文件系统允许的块大小为 512 至 32K 字节。支持的块大小为：512、1024、
2048 和 4096 字节。更大的块几乎能加速一切，代价是浪费磁盘空间。4K 以上的速度
提升似乎并不值得付出该代价，因此你在这里也不会损失太多。

上述文件系统的 muFS（多用户文件系统，multi user File System）等价形式也同样受支持。

## AFFS 的挂载选项


protect
		若设置此选项，保护位不可更改。

setuid[=uid]
		将文件系统中所有文件和目录的属主设为 uid 或当前用户的 uid。

setgid[=gid]
		同上，但针对 gid。

mode=mode
		将模式标志设为给定的（八进制）值，忽略原有权限。若相应的 r
		位置位，目录将获得 x 权限。这很有用，因为大多数普通 AmigaOS
		文件会映射为 600。

nofilenametruncate
		当文件名超过标准最大文件名长度（30 个字符）时，文件系统将返回错误。

reserved=num
		将分区起始处的保留块数量设为 num。通常不需要此选项。默认值为 2。

root=block
		设置根块的块号。通常无需此选项。

bs=blksize
		将块大小设为 blksize。合法的块大小为 512、1024、2048 和 4096。
		与 root 选项类似，通常无需此选项，因为 affs 可自行判断。

quiet
		对于不允许的模式更改，文件系统不会返回错误。

verbose
		文件系统挂载时，卷名、文件系统类型与块大小将被写入 syslog。

mufs
		该文件系统实际上是一个 muFS，尽管它并未以此自标识。若文件系统
		并非以 muFS 格式化却被当作 muFS 使用，则必须指定此选项。

prefix=path
		path 将被加在 AFFS 分区上每个符号链接的绝对路径名之前。
		默认 = “/”。（见下文。）

volume=name
		在 AFFS 分区上创建带绝对路径的符号链接时，name 将作为卷名前缀。
		默认 = “”（空字符串）。（见下文。）

## 用户/组与保护标志的处理


Amiga → Linux：

Amiga 保护标志 RWEDRWEDHSPARWED 按下述方式处理：

  - R 映射为用户、组和其他人的 r。在目录上，R 隐含 x。

  - W 映射为 w。

  - E 映射为 x。

  - D 被忽略。

  - H、S 和 P 始终保留，但在 Linux 下被忽略。

  - A 在写入文件时被清除。

除非通过挂载选项指定 set[gu]id，否则将使用用户 id 和组 id。由于大多数 Amiga
文件系统都是单用户系统，它们将由 root 拥有。Amiga 文件系统的根目录（挂载点）
将由实际挂载该文件系统的用户拥有（根目录没有 uid/gid 字段）。

Linux → Amiga：

Linux 的 rwxrwxrwx 文件模式按下述方式处理：

  - r 权限将允许用户、组和其他人的 R。

  - w 权限将允许用户、组和其他人的 W。

  - 用户的 x 权限将允许普通文件的 E。

  - D 将允许用户、组和其他人。

  - 所有其他标志（suid、sgid 等）被忽略，且不会被保留。

新创建的文件和目录将获得当前用户的用户 ID 与组 ID，以及依据 umask 设置的模式。

## 符号链接


尽管 Amiga 与 Linux 文件系统彼此相似，但仍存在某些（往往并不微妙）的差异。
其中之一在符号链接上表现得尤为明显。Linux 的文件系统只有一个根目录，而 Amiga
为每个文件系统（例如分区、软盘……）各设一个独立的根目录。在 Amiga 中，这些
实体被称为“卷（volumes）”。它们具有可用于访问的符号名。因此，符号链接可以
指向不同的卷。AFFS 将卷名转换为目录名，并在其前加上前缀路径（见 prefix 选项）。

示例：
你将所有 Amiga 分区挂载到 /amiga/<volume>（其中 <volume> 为卷名），并在挂载
所有 AFFS 分区时指定选项 “prefix=/amiga/”。（它们可能是 “User”、“WB” 和
“Graphics”，挂载点为 /amiga/User、/amiga/WB 和 /amiga/Graphics）。指向
“User:sc/include/dos/dos.h” 的符号链接将被解析为 “/amiga/User/sc/include/dos/dos.h”。

## 示例


```

    mount  Archive/Amiga/Workbench3.1.adf /mnt -t affs -o loop,verbose
    mount  /dev/sda3 /Amiga -t affs

```
```
    /dev/sdb5	/amiga/Workbench    affs    noauto,user,exec,verbose 0 0

```
## 重要提示


若你在 PC 连接 Amiga 硬盘的情况下启动 Windows 95（3.x、98 与 NT 未知），它会用
垃圾数据覆盖块 0 的字节 0x00dc..0x00df，从而使 Rigid Disk Block 失效。所幸这是
RDB 中未使用的区域，因此仅是校验和不匹配。Linux 会忽略这些垃圾数据并照常识别
RDB，但在你将该驱动器重新连接到 Amiga 之前，必须恢复或修复你的 RDB。因此在启动
Windows 之前，请务必为其制作一份备份！

若损害已经发生，以下命令应当能修复 RDB（其中 <disk> 为设备名）。

```

  dd if=/dev/<disk> of=rdb.tmp count=1
  cp rdb.tmp rdb.fixed
  dd if=/dev/zero of=rdb.fixed bs=1 seek=220 count=4
  dd if=rdb.fixed of=/dev/<disk>

```
## 缺陷、限制与注意事项


有不少功能可能并不如其所宣称的那样工作。并非所有内容都经过测试，尽管已用此 fs
读写过数百 MB 数据。最新的缺陷列表请参阅 fs/affs/Changes。

默认情况下，文件名会被无声地截断为 30 个字符。‘nofilenametruncate’ 挂载选项
可改变这一行为。

affs 在文件名匹配时忽略大小写，但 Linux shell
```

    rm /wb/WRONGCASE

```
```

    rm /wb/WR*

```
则不会，因为名称是由 shell 匹配的。

块分配是为硬盘分区设计的。若多于 1 个进程写入（小）软盘，块的分配方式会很不
理想（不过真正的 AFFS 也好不到哪去）。当空间变得紧张时同样如此。

无法在 OFS（旧文件系统，Old File System）上执行程序，因为由于 488 字节的块，
程序文件无法被内存映射。出于同样的原因，你也无法通过回环设备在这样的文件系统
上挂载映像。

当 affs 分区挂载时系统崩溃，根块中的位图有效标志可能不准确。目前若没有 Amiga
（磁盘校验器）或手动操作（谁会这么做呢？），无法修复损坏的文件系统。也许以后
会有办法。

若你在系统启动时挂载 affs 分区，可能需要告诉 fsck 不要检查该 fs（在 /etc/fstab
的第六字段填入 ‘0’）。

由于与 Amiga 软盘控制器不兼容，无法用普通 PC 或工作站读取软盘。

若你对 Linux 上的 Amiga 模拟器感兴趣，请参阅

http://web.archive.org/web/%2E/http://www.freiburg.linux.de/~uae/
