## 嵌入式设备命令行分区解析


“blkdevparts”命令行选项添加了对从内核命令行读取块设备分区表的支持
它通常用于固定块（eMMC）嵌入式设备。它没有 MBR，因此节省存储空间。引导加程序可以通过块设备上数据的绝对地址轻松访问。用户可以轻松更改分区
命令行的格式mtdparts 类似
blkdevparts=<blkdev-def>[;<blkdev-def>]
  <blkdev-def> := <blkdev-id>:<partdef>[,<partdef>]
    <partdef> := <size>[@<offset>](part-name)

<blkdev-id>
    块设备磁盘名。嵌入式设备使用固定块设备。其磁盘名也是固定的，例如：
    mmcblk0、mmcblk1、mmcblk0boot0
<size>
    分区大小，以字节为单位，例如12mG。大小可包含可选后缀
    （大写或小写）：

      K, M, G, T, P, E銆。
    表示所有剩余空间
<offset>
    分区起始地址，以字节为单位。偏移可包含可选后缀（大写或小写）：

      K, M, G, T, P, E銆。
(part-name)
    分区名。内核发送带有“PARTNAME”的 uevent。应用程序可以创建指向该名称
    “PARTNAME”的块设备分区的链接。用户空间应用程序可以通过分区名访问分区
ro
    只读。将分区标记为只读
示例
    eMMC 磁盘名为 "mmcblk0" "mmcblk0boot0"
```
    'blkdevparts=mmcblk0:1G(data0),1G(data1),-;mmcblk0boot0:1m(boot)ro,-(kernel)'

  dmesg::

    mmcblk0: p1(data0) p2(data1) p3()
    mmcblk0boot0: p1(boot) p2(kernel)
```
