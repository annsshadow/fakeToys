## 通过 configfs 配置的 Linux USB gadget


25th April 2013



## 概述


一个 Linux USB Gadget 是拥有 UDC（USB Device Controller，USB 设备控制器）的设备，可以
连接到 USB 主机（Host），以扩展其功能，例如串口或大容量存储能力。

从主机的角度看，一个 gadget 是一组配置（configuration），每个配置包含若干接口（interface），
从 gadget 的角度看，这些接口被称为功能（function），每个功能代表例如一个串行连接或一块
SCSI 磁盘。

Linux 为 gadget 提供了一系列可供使用的功能。

创建一个 gadget 意味着决定会有哪些配置，以及每个配置会提供哪些功能。

Configfs（请参见 `Documentation/filesystems/configfs.rst`）非常适合用于向内核告知上述决策。
本文档讲述如何做到这一点。

它还描述了 configfs 与 gadget 的集成是如何设计的。



## 要求


为了让这一切工作，configfs 必须可用，因此在 .config 中 CONFIGFS_FS 必须为 'y' 或 'm'。
截至本文撰写时，USB_LIBCOMPOSITE 会自动选择 CONFIGFS_FS。



## 用法


（描述首个通过 configfs 可用的功能的原始帖子可以在这里看到：
http://www.spinics.net/lists/linux-usb/msg76388.html）

```

	$ modprobe libcomposite
	$ mount none $CONFIGFS_HOME -t configfs

```
其中 CONFIGFS_HOME 是 configfs 的挂载点

### 1. 创建 gadget


```

	$ mkdir $CONFIGFS_HOME/usb_gadget/<gadget name>

```
```

	$ mkdir $CONFIGFS_HOME/usb_gadget/g1

	...
	...
	...

	$ cd $CONFIGFS_HOME/usb_gadget/g1

```
```

	$ echo <VID> > idVendor
	$ echo <PID> > idProduct

```
一个 gadget 还需要它的序列号、厂商名与产品名字符串。为了有地方存放它们，必须创建一个
strings 子目录
```

	$ mkdir strings/0x409

```
```

	$ echo <serial number> > strings/0x409/serialnumber
	$ echo <manufacturer> > strings/0x409/manufacturer
	$ echo <product> > strings/0x409/product

```
进一步的自定义字符串描述符可以作为该语言目录下的子目录创建，字符串文本被写入 "s" 属性
```

	$ mkdir strings/0x409/xu.0
	$ echo <string text> > strings/0x409/xu.0/s

```
在功能驱动支持的情况下，功能可以允许创建指向这些自定义字符串描述符的符号链接，以将这些
字符串与类描述符关联起来。

### 2. 创建配置


每个 gadget 由若干配置组成，它们相应的
```

        $ mkdir configs/<name>.<number>

```
其中 <name> 可以是文件系统中合法的任意字符串，而
```

	$ mkdir configs/c.1

	...
	...
	...

```
每个配置也需要它自己的字符串，因此必须创建一个子目录
```

	$ mkdir configs/c.1/strings/0x409

```
```

	$ echo <configuration> > configs/c.1/strings/0x409/configuration

```
```

	$ echo 120 > configs/c.1/MaxPower

```
### 3. 创建功能


该 gadget 将提供一些功能，每个功能对应的
```

	$ mkdir functions/<name>.<instance name>

```
其中 <name> 对应于某个允许的功能名，instance name（实例名）
```

  $ mkdir functions/ncm.usb0 # usb_f_ncm.ko gets loaded with request_module()

  ...
  ...
  ...

```
每个功能提供其特定的一组属性，可以是只读或读写访问。在适用的情况下，需要以适当的方式
写入它们。更多信息请参考 Documentation/ABI/testing/configfs-usb-gadget。

### 4. 将功能与配置关联


此刻已经创建了若干 gadget，每个 gadget 都指定了若干配置并提供了若干可用功能。剩下的就是
指定哪个功能在哪个配置中可用（同一个功能可以在多个配置中使用）。这通过以下方式实现
```

	$ ln -s functions/<name>.<instance name> configs/<name>.<number>

```
```

	$ ln -s functions/ncm.usb0 configs/c.1

	...
	...
	...

```
### 5. 启用 gadget


以上所有步骤的目的都是组合出由配置与功能构成的 gadget。

```

  .
  ./strings
  ./strings/0x409
  ./strings/0x409/serialnumber
  ./strings/0x409/product
  ./strings/0x409/manufacturer
  ./configs
  ./configs/c.1
  ./configs/c.1/ncm.usb0 -> ../../../../usb_gadget/g1/functions/ncm.usb0
  ./configs/c.1/strings
  ./configs/c.1/strings/0x409
  ./configs/c.1/strings/0x409/configuration
  ./configs/c.1/bmAttributes
  ./configs/c.1/MaxPower
  ./functions
  ./functions/ncm.usb0
  ./functions/ncm.usb0/ifname
  ./functions/ncm.usb0/qmult
  ./functions/ncm.usb0/host_addr
  ./functions/ncm.usb0/dev_addr
  ./UDC
  ./bcdUSB
  ./bcdDevice
  ./idProduct
  ./idVendor
  ./bMaxPacketSize0
  ./bDeviceProtocol
  ./bDeviceSubClass
  ./bDeviceClass


```
这样一个 gadget 最终必须被启用，这样 USB 主机才能枚举它。

为了启用 gadget，必须将它绑定到一个 UDC（USB Device Controller）
```

	$ echo <udc name> > UDC

```
其中 <udc name> 是 /sys/class/udc/* 中找到的名字之一
```

	$ echo s3c-hsotg > UDC


```
### 6. 禁用 gadget


```

	$ echo "" > UDC

```
### 7. 清理


```

	$ rm configs/<config name>.<number>/<function>

```
其中 <config name>.<number> 指定配置，<function> 是
```

	$ rm configs/c.1/ncm.usb0

	...
	...
	...

```
```

	$ rmdir configs/<config name>.<number>/strings/<lang>

```
```

	$ rmdir configs/c.1/strings/0x409

	...
	...
	...

```
```

	$ rmdir configs/<config name>.<number>

```
```

	rmdir configs/c.1

	...
	...
	...

```
```

	$ rmdir functions/<name>.<instance name>

```
```

	$ rmdir functions/ncm.usb0

	...
	...
	...

```
```

	$ rmdir strings/<lang>

```
```

	$ rmdir strings/0x409

```
```

	$ cd ..
	$ rmdir <gadget name>

```
```

	$ rmdir g1



```
## 实现设计


下面介绍 configfs 是如何工作的。在 configfs 中有 item（项）与 group（组），两者都表示为
目录。item 与 group 的区别在于，group 可以包含其它的 group。下面的图中只显示了一个 item。
item 与 group 都可以有属性（attribute），它们表示为文件。用户可以创建和删除目录，但不能
删除文件，文件可以是只读或读写的，取决于它们所代表的内容。

configfs 的文件系统部分操作的是 config_items/groups 与 configfs_attributes，它们对于
所有被配置的元素都是通用的、同一类型的。然而，它们被内嵌于特定用途的更大结构中。在下面的
图中有一个 “cs”，它包含一个 config_item，以及一个 “sa”，它包含一个 configfs_attribute。

```

  ./
  ./cs        (directory)
     |
     +--sa    (file)
     |
     .
     .
     .

```
每当用户读取/写入 “sa” 文件时，会调用一个函数，该函数接受一个 struct config_item 与
一个 struct configfs_attribute。在该函数中，使用众所周知的 container_of 技术取回 “cs”
与 “sa”，并调用相应的 sa 函数（show 或 store），将 “cs” 与一个字符缓冲区传给它。“show”
用于显示文件的内容（将数据从 cs 复制到缓冲区），而 “store” 用于修改文件的内容（将数据从
缓冲区复制到 cs），但这两个函数实际做什么由实现者决定。

```

  typedef struct configured_structure cs;
  typedef struct specific_attribute sa;

                                         sa
                         +----------------------------------+
          cs             |  (*show)(cs *, buffer);          |
  +-----------------+    |  (*store)(cs *, buffer, length); |
  |                 |    |                                  |
  | +-------------+ |    |       +------------------+       |
  | | struct      |-|----|------>|struct            |       |
  | | config_item | |    |       |configfs_attribute|       |
  | +-------------+ |    |       +------------------+       |
  |                 |    +----------------------------------+
  | data to be set  |                .
  |                 |                .
  +-----------------+                .

```
文件名由 config item/group 的设计者决定，而目录一般可以随意命名。一个 group 可以有若干
默认子组被自动创建。

有关 configfs 的更多信息，请参见 `Documentation/filesystems/configfs.rst`。

上述概念映射到 USB gadget 上如下：

1. 一个 gadget 有它的 config group，它有一些属性（idVendor、idProduct 等）以及默认子组
   （configs、functions、strings）。写入这些属性会使信息被存储到适当的位置。在 configs、
   functions 与 strings 子组中，用户可以创建他们自己的子组，以表示给定语言下的配置、功能
   与字符串组。

2. 用户创建配置与功能，并在配置中创建指向功能的符号链接。这些信息在写入 gadget 的 UDC
   属性时被使用，这意味着将 gadget 绑定到 UDC。drivers/usb/gadget/configfs.c 中的代码遍历
   所有配置，并在每个配置中遍历所有功能并将它们绑定。这样整个 gadget 就被绑定了。

3. drivers/usb/gadget/configfs.c 文件中包含用于以下用途的代码：

 - gadget 的 config_group
 - gadget 的默认组（configs、functions、strings）
 - 将功能与配置关联（符号链接）

4. 每个 USB 功能自然有它自己想要配置的内容的视图，因此特定功能的 config_groups 定义在各
   功能的实现文件 drivers/usb/gadget/f_*.c 中。

5. 功能的代码编写方式使得它使用 usb_get_function_instance()，而后者又会调用 request_module。
   因此，只要 modprobe 能正常工作，特定功能的模块就会被自动加载。请注意反之不成立：在一个
   gadget 被禁用并拆除之后，模块仍然保持加载状态。
