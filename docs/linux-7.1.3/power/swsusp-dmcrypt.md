## How to use dm-crypt and swsusp together


作者：Andreas Steinmetz <ast@domdv.de>



一些先决条件：
你了解 dm-crypt 的工作原理。如果不了解，请访问以下网页：
http://www.saout.de/misc/dm-crypt/
你已阅读 Documentation/power/swsusp.rst 并理解了它。
你确实阅读过 Documentation/admin-guide/initrd.rst 并了解 initrd 的工作原理。
你知道如何创建或修改一个 initrd。

现在你的系统已正确设置，除了交换设备以及可能包含用于加密设置和/或救援用途的
迷你系统的引导分区外，你的磁盘已加密。你甚至可能已经有一个会完成当前加密设置的 initrd。

此时你也想加密你的交换分区。同时你仍希望能够通过 swsusp 挂起。然而，这意味着你
必须能够在恢复之前输入口令，或者从外部设备（如 pcmcia 闪存盘或 usb 存储棒）读取密钥。
因此你需要一个 initrd，它先设置 dm-crypt，然后让 swsusp 从加密的交换设备恢复。

最重要的一点是，你设置 dm-crypt 的方式必须使得你挂起/恢复到的交换设备在 initrd 内
以及运行中的系统内始终具有相同的主/次设备号。最简单的实现方式是始终首先用 dmsetup
设置该交换设备，这样
```

  brw-------  1 root root 254, 0 Jul 28 13:37 /dev/mapper/swap0

```
现在将你的内核设置为使用 /dev/mapper/swap0 作为默认的
```

  CONFIG_PM_STD_PARTITION="/dev/mapper/swap0"

```
准备好你的引导加载程序以使用你将创建或修改的 initrd。对于 lilo，最简单的设置如下所示
```

  image=/boot/vmlinuz
  initrd=/boot/initrd.gz
  label=linux
  append="root=/dev/ram0 init=/linuxrc rw"

```
最后你需要创建或修改你的 initrd。假设你要创建一个从 pcmcia 闪存卡读取所需 dm-crypt
设置的 initrd。该卡格式化为 ext2 文件系统，插入时位于 /dev/hde1。该卡至少包含一个名为
“swapkey”的文件，其中存放着加密的交换设置。你 initrd 的 /etc/fstab 中含有类似如下内容
```

  /dev/hda1   /mnt    ext3      ro                            0 0
  none        /proc   proc      defaults,noatime,nodiratime   0 0
  none        /sys    sysfs     defaults,noatime,nodiratime   0 0

```
/dev/hda1 包含一个未加密的迷你系统，它同样通过从 pcmcia 闪存盘读取设置来配置你所有的
加密设备。以下是你的 initrd 的一个 /linuxrc，它允许你从加密交换恢复，并在恢复失败时
继续用 /dev/hda1 上的迷你系统引导
```

  #!/bin/sh
  PATH=/sbin:/bin:/usr/sbin:/usr/bin
  mount /proc
  mount /sys
  mapped=0
  noresume=`grep -c noresume /proc/cmdline`
  if [ "$*" != "" ]
  then
    noresume=1
  fi
  dmesg -n 1
  /sbin/cardmgr -q
  for i in 1 2 3 4 5 6 7 8 9 0
  do
    if [ -f /proc/ide/hde/media ]
    then
      usleep 500000
      mount -t ext2 -o ro /dev/hde1 /mnt
      if [ -f /mnt/swapkey ]
      then
        dmsetup create swap0 /mnt/swapkey > /dev/null 2>&1 && mapped=1
      fi
      umount /mnt
      break
    fi
    usleep 500000
  done
  killproc /sbin/cardmgr
  dmesg -n 6
  if [ $mapped = 1 ]
  then
    if [ $noresume != 0 ]
    then
      mkswap /dev/mapper/swap0 > /dev/null 2>&1
    fi
    echo 254:0 > /sys/power/resume
    dmsetup remove swap0
  fi
  umount /sys
  mount /mnt
  umount /proc
  cd /mnt
  pivot_root . mnt
  mount /proc
  umount -l /mnt
  umount /proc
  exec chroot . /sbin/init $* < dev/console > dev/console 2>&1

```
请不要介意上面这个奇怪的循环，busybox 的 msh 不认识 let 语句。那么，这个脚本里发生了什么？
首先我们必须决定是否要尝试恢复。如果我们以“noresume”或任何给 init 的参数（如“single”
或“emergency”）作为引导参数启动，我们将不恢复。

然后我们需要用来自 pcmcia 闪存盘的设置数据设置 dmcrypt。如果成功，且我们不想恢复，
则需要重置交换设备。随后“echo 254:0 > /sys/power/resume”这一行尝试从第一个设备映射
设备恢复。注意，无论是否恢复，在 /sys/power/resume 中设置设备都很重要，否则后续挂起会失败。
如果恢复开始，脚本执行到此终止。

否则我们只是移除加密的交换设备，并将其留给 /dev/hda1 上的迷你系统来完成整个加密的设置
（你可以根据需要自行修改）。

接下来就是众所周知的切换根文件系统并从中继续引导的过程。我倾向于在继续引导之前卸载
initrd，但这由你自行修改。

