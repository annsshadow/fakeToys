
## C2 端口支持


(C) Copyright 2007 Rodolfo Giometti <giometti@enneenne.com>

本程序是自由软件；你可以在自由软件基金会发布GNU 通用公共许可证条款下重新
分发或修改它；无论是许可证的2 版，还是（由你选择）任何更晚的版本
本程序的分发希望它有用，但没有任何担保；甚至不隐含任何适销性或特定用途适用的担保。更多细节请参阅 GNU 通用公共许可证
### 概述


本驱动实现了 Linux Silicon Labs（Silabs）C2 接口的支持，该接口用于微控制器的
在系统编程（in-system programming）
通过使用本驱动，你可以在没有 EC2 EC3 调试适配器的情况下对在系统闪存进行重编程。该方案在那些微控制器通过特殊 GPIO 引脚连接的系统中也很有用
### 参考资

C2 接口的主要参考资料位(https://www.silabs.com) Silicon Laboratories 网站参见
- AN127: FLASH Programming via the C2 Interface，地址  https://www.silabs.com/Support Documents/TechnicalDocs/an127.pdf

- C2 Specification，地址  https://www.silabs.com/pages/DownloadDoc.aspxFILEURL=Support%20Documents/TechnicalDocs/an127.pdf&src=SearchResults

它实现了一个双线串行通信协议（bit banging），旨在为低引脚数的 Silicon Labs 设备
实现在系统编程、调试和边界扫描测试。目前这段代码仅支持闪存编程，但扩展很容添加
### 使用驱动


一旦驱动被加载，你就可以使sysfs 支持来获C2port 的：

```

  # ls /sys/class/c2port/c2port0/
  access            flash_block_size  flash_erase       rev_id
  dev_id            flash_blocks_num  flash_size        subsystem/
  flash_access      flash_data        reset             uevent

```
最C2port 访问是被禁用的，因为你的硬件可能将这些线路与其他设备复用，因此要
获取访问权限需执行
```

  # echo 1 > /sys/class/c2port/c2port0/access

```
此后你应该读取设ID 与版ID
```

  # cat /sys/class/c2port/c2port0/dev_id
  8
  # cat /sys/class/c2port/c2port0/rev_id
  1

```
然而出于安全原因，在系统闪存访问默认是不：

```

  # echo 1 > /sys/class/c2port/c2port0/flash_access

```
```

  # cat /sys/class/c2port/c2port0/flash_data > image

```
```

  # echo 1 > /sys/class/c2port/c2port0/flash_erase

```
```

  # cat image > /sys/class/c2port/c2port0/flash_data

```
```

  # echo 1 > /sys/class/c2port/c2port0/reset

```
