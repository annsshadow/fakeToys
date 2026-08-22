
## 通过 sysfs 访问 PCI 设备资源


sysfs 通常挂载/sys，在平台上提供对 PCI 资源的访
```

     /sys/devices/pci0000:17
     |-- 0000:17:00.0
     |   |-- class
     |   |-- config
     |   |-- device
     |   |-- enable
     |   |-- irq
     |   |-- local_cpus
     |   |-- remove
     |   |-- resource
     |   |-- resource0
     |   |-- resource1
     |   |-- resource2
     |   |-- revision
     |   |-- rom
     |   |-- subsystem_device
     |   |-- subsystem_vendor
     |   `-- vendor
     `-- ...

```
最顶层的元素描PCI 域和总线号。在本例中，域号0000，总线号为 17（两个值均为十六进制）。该总线上有一个位于插0 的单功能设备。域号和总线号会为了方便而重复给出。设备目录下有几个文件，每个文件各有其功能

       =================== =====================================================
       file		   function
       =================== =====================================================
       class		   PCI class (ascii, ro)
       config		   PCI config space (binary, rw)
       device		   PCI device (ascii, ro)
       enable	           Whether the device is enabled (ascii, rw)
       irq		   IRQ number (ascii, ro)
       local_cpus	   nearby CPU mask (cpumask, ro)
       remove		   remove device from kernel's list (ascii, wo)
       resource		   PCI resource host addresses (ascii, ro)
       resource0..N	   PCI resource N, if present (binary, mmap, rw\ [^1^]_)
       resource0_wc..N_wc  PCI WC map resource N, if prefetchable (binary, mmap)
       revision		   PCI revision (ascii, ro)
       rom		   PCI ROM resource, if present (binary, ro)
       subsystem_device	   PCI subsystem device (ascii, ro)
       subsystem_vendor	   PCI subsystem vendor (ascii, ro)
       vendor		   PCI vendor (ascii, ro)
       =================== =====================================================

```

  ro - read only file
  rw - file is readable and writable
  wo - write only file
  mmap - file is mmapable
  ascii - file contains ascii text
  binary - file contains binary data
  cpumask - file contains a cpumask type

```

只读文件是信息性的，对它们的写入将被忽略，'rom' 文件除外。可写文件可用于对设备执行操作（例如更改配置空间、卸载设备）。可通过在偏移量 0 处对文件进行 mmap 来获得可映射文件，并可用于从用户空间实际对设备进行编程。注意，某些平台不支持对某些资源进行 mmap，因此务必检查任何一次尝mmap 的返回值。其中最值得注意的是 I/O 端口资源，它们也提供写访问

'enable' 文件提供一个计数器，指示设备被启用的次数。如'enable' 文件当前返回 '4'，并且向其中写入一'1'，它将返'5'。向其中写入 '0' 会将计数减少。不过，即使它回0，某些初始化操作也可能不会被撤销

'rom' 文件特殊之处在于，如果可用，它提供对设备 ROM 文件的只读访问。不过它默认是禁用的，因此应用程序应当在尝试读取调用之前向该文件写入字符"1" 来启用它，并在访问之后通过向该文件写入 "0" 来禁用它。注意，设备必须处于启用状态，ROM 读取才能成功返回数据。在没有驱动绑定到该设备的情况下，可以使用上文记载的 'enable' 文件将其启用

'remove' 文件用于移除 PCI 设备，方法是向该文件写入一个非零整数。这不涉及任何类型的热插拔功能，例如关闭设备电源。该设备会从内核PCI 设备列表中被移除，其对应sysfs 目录被删除，并且该设备会从任何附加到它的驱动中被移除。不允许移除 PCI 根总线

### 通过 sysfs 访问传统资源


如果底层平台支持，传I/O 端口ISA 内存资源也会sysfs 中提供。它们位PCI 类层级结构中
```

	/sys/class/pci_bus/0000:17/
	|-- bridge -> ../../../devices/pci0000:17
	|-- cpuaffinity
	|-- legacy_io
	`-- legacy_mem

```
legacy_io 文件是一个读/写文件，应用程序可用它来进行传统端口 I/O。应用程序应当打开该文件，定位到期望的端口（例0x3e8）并进行 1 4 字节的读或写。legacy_mem 文件应当以对应于期望内存偏移量的偏移量进mmap，例VGA 帧缓冲的 0xa0000。然后应用程序可以（在检查过错误之后）直接解引用返回的指针来访问传统内存空间

### 在新平台上支PCI 访问


为了支持如上所述的 PCI 资源映射，Linux 平台代码理想情况下应当定ARCH_GENERIC_PCI_MMAP_RESOURCE 并使用该功能特性的通用实现。为了支/proc/bus/pci 中通过文件进行 mmap() 的历史接口，平台也可以设HAVE_PCI_MMAP

或者，设置HAVE_PCI_MMAP 的平台可以提供它们自己的 pci_mmap_resource_range() 实现，而不是定ARCH_GENERIC_PCI_MMAP_RESOURCE

支持 PCI 资源写合并映射的平台必须定义 arch_can_pci_mmap_wc()，当允许写合并时，它在运行时应求值为非零。类似地，支I/O 资源映射的平台定arch_can_pci_mmap_io()

传统资源HAVE_PCI_LEGACY 定义保护。希望支持传统功能的平台应当定义它，并提pci_legacy_read、pci_legacy_write pci_mmap_legacy_page_range 函数
