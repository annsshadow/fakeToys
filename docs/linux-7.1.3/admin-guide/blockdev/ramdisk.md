## Linux 中使RAM 磁盘块设

 1) 概述
 2) 内核命令行参 3) 使用 "rdev"
 4) 创建压缩 RAM 磁盘的示

### 1) 概述


RAM 磁盘驱动是一种将主系统内存用作块设备的方法。它对于 initrd 是必需的——如果你需要加载模块才能访根文件系统，就会用到 initrd 这个初始文件系统（参`Documentation/admin-guide/initrd.rst`）。它也可用于加密等工作的临时文件系统，因为其内容在重启时会被清除
RAM 磁盘会随着需要更多空间而动态增长。它通过使用来自缓冲区缓存的 RAM 来实现。驱动将其正在使用的缓冲标记为脏，以VM 子系统之后不会尝试回收它们
RAM 磁盘默认支持最16 RAM 磁盘，并且可以重新配置为支持无限数量RAM 磁盘（风险自负）。只需块设备驱动配置菜单中更改配置符号 BLK_DEV_RAM_COUNT 并（重新）构建内核即可
要在你的系统中使RAM 磁盘支持，请/dev 目录运行 `./MAKEDEV ram`。RAM 磁盘的主设备号均1并以 /dev/ram0 的次设备0 开始，依此类推。如果使用，现代内核使用 /dev/ram0 作为 initrd
新的 RAM 磁盘还能够加载压缩的 RAM 磁盘映像，从而可以在普通的安装或救援软盘上塞入更多程序

### 2) 参数


2a) 内核命令行参
	ramdisk_size=N
		 ramdisk 的大小
该参数告RAM 磁盘驱动建立大小N k RAM 磁盘。默认为 4096 MB）
2b) 模块参数

	rd_nr
		/dev/ramX 设备被创建的数量
	max_part
		最大分区号
	rd_size
		参见 ramdisk_size
### 3) 使用 "rdev"


"rdev" 是一个过时、已弃用、老旧的实用工具，曾可用于Linux 内核映像中设置启动设备
与其使用 rdev，不如直接将启动设备信息放在内核命令行上，并由引导加载程序传递给内核
你还可以通过arch/x86/boot/Makefile 中设FDARGS 来向内核传递参数，并通过arch/x86/boot/Makefile
中设FDINITRD 来指initrd 映像
```

  ramdisk_start=N
  ramdisk_size=M

```
```

	append = "ramdisk_start=N ramdisk_size=M"

```
### 4) 创建压缩 RAM 磁盘的示

要创RAM 磁盘映像，你需要一个空闲的块设备来构建它。可以是 RAM 磁盘设备本身，也可以是一个未使用磁盘分区（例如一个未挂载的交换分区）。在本例中，我们将使RAM 磁盘设备dev/ram0”
注意：此技术不应在内存少于 8 MB 的机器上进行。如果使用空闲磁盘分区而非 /dev/ram0，则此限制不适用
a) 确定你想要的 RAM 磁盘大小。本例设2 MB。通过写入 RAM 磁盘设备来创建它。（此步骤当前不是必需的，
   但将来可能需要。）将区域清零是明智的（尤其是对磁盘），以便对文件实现最大程度的压缩
```

	dd if=/dev/zero of=/dev/ram0 bs=1k count=2048

```
```

	mke2fs -vm0 /dev/ram0 2048

```
c) 挂载它，将你需要的文件复制进去（例如：/etc/** /dev/** ...），然后再次卸载它
d) 压缩 RAM 磁盘的内容。压缩级别大约为文件所占用空间50%。未使用的：

```

	dd if=/dev/ram0 bs=1k count=2048 | gzip -v9 > /tmp/ram_image.gz

```
```

	dd if=zImage of=/dev/fd0 bs=1k

```
f) RAM 磁盘映像放到软盘上，放在内核之后。使用一个比内核略大的偏移量，这样以后你可以在同一张软盘上
   放置另一个（可能更大的）内核而不RAM 磁盘映像重叠。对于约 350 kB 大小的内核，400 kB 的偏移量   合理的。确ram_image.gz 的偏移量+大小
```

	dd if=/tmp/ram_image.gz of=/dev/fd0 bs=1k seek=400

```
g) 确保你已经在 FDARGS FDINITRD 中指定了启动信息，或者使用引导加载程序将内核命令行启动选项传递给内核
就是这样。你现在拥有了你的启根压RAM 磁盘软盘。一些用户可能希望通过管道合并步骤（d）和（f）

					Paul Gortmaker 12/95

### 更新日志

SEPT-2020锛。
                移除"rdev" 的用
10-22-04		更新以反映命令行选项的变化，移除过时的引用，总体清理		James Nelson (james4765@gmail.com)

12-95		原始文档
