
## Linux I2C 的 Sysfs


## 概述


由于存在 I2C MUX（I2C 多路复用器），I2C 拓扑可能十分复杂。Linux
内核将 MUX 通道抽象为逻辑 I2C 总线编号。然而，从 I2C 总线物理编号和 MUX
拓扑映射到逻辑 I2C 总线编号之间存在知识鸿沟。本文档旨在填补这一鸿沟，使
读者（例如硬件工程师和新的软件开发人员）能够通过了解物理 I2C
拓扑并在 Linux shell 中浏览 I2C sysfs，掌握内核中逻辑 I2C 总线的概念。这一知识
对于使用 `i2c-tools` 进行开发和调试十分有用且必不可少。

### 目标读者


需要使用 Linux shell 与运行 Linux 的系统上的 I2C 子系统进行交互的人员。

### 前提条件


1. 了解 Linux shell 文件系统命令和操作的一般知识。

2. 了解 I2C、I2C MUX 和 I2C 拓扑的一般知识。

## I2C Sysfs 的位置


通常，Linux Sysfs 文件系统挂载在 `/sys` 目录下，因此可以在
`/sys/bus/i2c/devices` 下找到 I2C Sysfs，你可以直接 `cd` 到该目录。
该目录下有一系列符号链接。以 `i2c-` 开头的链接是 I2C 总线，可能是物理的，
也可能是逻辑的。其他以数字开头并以数字结尾的链接是 I2C 设备，其中第一个数字是
I2C 总线编号，第二个数字是 I2C 地址。

```

  blueline:/sys/bus/i2c/devices $ ls
  0-0008  0-0061  1-0028  3-0043  4-0036  4-0041  i2c-1  i2c-3
  0-000c  0-0066  2-0049  4-000b  4-0040  i2c-0   i2c-2  i2c-4

```
`i2c-2` 是编号为 2 的 I2C 总线，`2-0049` 是总线 2 上地址为 0x49、已绑定内核驱动的
I2C 设备。

## 术语


首先，我们定义一些术语，以避免后续章节中的混淆。

### （物理）I2C 总线控制器


运行 Linux 内核的硬件系统可能拥有多个物理 I2C 总线控制器。这些控制器是硬件且
物理的，系统可能在内存空间中定义多个寄存器来操作这些控制器。Linux 内核在源码
目录 `drivers/i2c/busses` 下有 I2C 总线驱动，用于将内核 I2C API 转换为针对不同
系统的寄存器操作。此术语并不局限于 Linux 内核。

### I2C 总线物理编号


对于每个物理 I2C 总线控制器，系统厂商可能会为每个控制器分配一个物理编号。
例如，具有最低寄存器地址的第一个 I2C 总线控制器可能被称为 `I2C-0`。

### 逻辑 I2C 总线


你在 Linux I2C Sysfs 中看到的每个 I2C 总线编号都是一个被分配了编号的逻辑 I2C
总线。这类似于软件代码通常编写在虚拟内存空间之上，而非物理内存空间。

每个逻辑 I2C 总线可能是某个物理 I2C 总线控制器的抽象，也可能是某个 I2C MUX
之后的通道的抽象。如果它是 MUX 通道的抽象，那么每当我们通过此类逻辑总线访问
I2C 设备时，内核会作为抽象的一部分为你把 I2C MUX 切换到相应的通道。

### 物理 I2C 总线


如果逻辑 I2C 总线是某个物理 I2C 总线控制器的直接抽象，我们称之为物理 I2C 总线。

### 注意事项


对于只了解电路板物理 I2C 设计的人来说，这部分可能令人困惑。实际上，在设备树
源文件（DTS）的 `aliases` 段下，可以将 I2C 总线物理编号重命名为逻辑 I2C 总线
级别上的不同编号。相关 DTS 文件示例参见
`arch/arm/boot/dts/nuvoton-npcm730-gsj.dts`。

最佳实践：**（针对内核软件开发人员）** 最好让 I2C 总线物理编号与其对应的逻辑 I2C
总线编号保持一致，而不是重命名或映射它们，这样可以减少给其他用户的困惑。这些
物理 I2C 总线可以作为 I2C MUX 扇出的良好起点。在后续示例中，我们将假设物理 I2C
总线的编号与其 I2C 总线物理编号相同。

## 遍历逻辑 I2C 总线


以下内容将使用一个更复杂的 I2C 拓扑作为示例。下面是该 I2C 拓扑的简要图示。如果
你第一眼没看懂，不必担心，继续阅读本文档，读完后再回看即可。

```

  i2c-7 (physical I2C bus controller 7)
  `-- 7-0071 (4-channel I2C MUX at 0x71)
      |-- i2c-60 (channel-0)
      |-- i2c-73 (channel-1)
      |   |-- 73-0040 (I2C sensor device with hwmon directory)
      |   |-- 73-0070 (I2C MUX at 0x70, exists in DTS, but failed to probe)
      |   `-- 73-0072 (8-channel I2C MUX at 0x72)
      |       |-- i2c-78 (channel-0)
      |       |-- ... (channel-1...6, i2c-79...i2c-84)
      |       `-- i2c-85 (channel-7)
      |-- i2c-86 (channel-2)
      `-- i2c-203 (channel-3)

```
### 区分物理 I2C 总线与逻辑 I2C 总线


区分物理 I2C 总线和逻辑 I2C 总线的一个简单方法，是使用 `ls -l` 或 `readlink`
命令读取 I2C 总线目录下名为 `device` 的符号链接。

另一个可检查的符号链接是 `mux_device`。该链接只存在于从另一条 I2C 总线扇出的
逻辑 I2C 总线目录中。读取此链接还能告诉你是哪个 I2C MUX 设备创建了这个逻辑 I2C
总线。

如果符号链接指向以 `.i2c` 结尾的目录，那么它应该是一个物理 I2C 总线，直接抽象了
某个物理 I2C 总线控制器。例如：
```

  $ readlink /sys/bus/i2c/devices/i2c-7/device
  ../../f0087000.i2c
  $ ls /sys/bus/i2c/devices/i2c-7/mux_device
  ls: /sys/bus/i2c/devices/i2c-7/mux_device: No such file or directory

```
在此例中，`i2c-7` 是一条物理 I2C 总线，因此其目录下没有 `mux_device` 符号链接。
如果内核软件开发人员遵循不重命名物理 I2C 总线的惯例，这也意味着它对应系统中编号为
7 的物理 I2C 总线控制器。

另一方面，如果符号链接指向另一条 I2C 总线，则当前目录所表示的 I2C 总线必定是一条
逻辑总线。该链接指向的 I2C 总线是父总线，可能是物理 I2C 总线，也可能是逻辑 I2C
总线。在这种情况下，当前目录所表示的 I2C 总线抽象的是父总线下的某个 I2C MUX 通道。

```

  $ readlink /sys/bus/i2c/devices/i2c-73/device
  ../../i2c-7
  $ readlink /sys/bus/i2c/devices/i2c-73/mux_device
  ../7-0071

```
`i2c-73` 是由 `i2c-7` 下某个 I2C MUX 扇出的逻辑总线，该 MUX 的 I2C 地址为
0x71。每当我们访问总线 73 上的某个 I2C 设备时，内核总会作为抽象的一部分，把地址为
0x71 的 I2C MUX 为你切换到相应通道。

### 查找逻辑 I2C 总线编号


本节将描述如何基于物理硬件 I2C 拓扑的知识，找出表示特定 I2C MUX 通道的逻辑 I2C
总线编号。

在此示例中，我们有一个系统，其物理 I2C 总线 7 在 DTS 中未被重命名。该总线上有一个
地址为 0x71 的 4 通道 MUX。在 0x71 这个 MUX 的通道 1 之后，还有一个地址为 0x72 的
8 通道 MUX。让我们浏览 Sysfs，找出 0x72 MUX 的通道 3 的逻辑 I2C 总线编号。

```

  ~$ cd /sys/bus/i2c/devices/i2c-7
  /sys/bus/i2c/devices/i2c-7$ ls
  7-0071         i2c-60         name           subsystem
  delete_device  i2c-73         new_device     uevent
  device         i2c-86         of_node
  i2c-203        i2c-dev        power

```
```

  /sys/bus/i2c/devices/i2c-7$ cd 7-0071/
  /sys/bus/i2c/devices/i2c-7/7-0071$ ls -l
  channel-0   channel-3   modalias    power
  channel-1   driver      name        subsystem
  channel-2   idle_state  of_node     uevent

```
```

  /sys/bus/i2c/devices/i2c-7/7-0071$ readlink channel-1
  ../i2c-73

```
我们发现 `i2c-7` 上 0x71 MUX 的通道 1 被分配了逻辑 I2C 总线编号 73。
```

  # cd to i2c-73 under I2C Sysfs root
  /sys/bus/i2c/devices/i2c-7/7-0071$ cd /sys/bus/i2c/devices/i2c-73
  /sys/bus/i2c/devices/i2c-73$

  # cd the channel symbolic link
  /sys/bus/i2c/devices/i2c-7/7-0071$ cd channel-1
  /sys/bus/i2c/devices/i2c-7/7-0071/channel-1$

  # cd the link content
  /sys/bus/i2c/devices/i2c-7/7-0071$ cd ../i2c-73
  /sys/bus/i2c/devices/i2c-7/i2c-73$

```
无论哪种方式，最终都会进入 `i2c-73` 的目录。类似地，我们现在可以找出 0x72 MUX
及其对应的逻辑 I2C 总线编号：
```

  /sys/bus/i2c/devices/i2c-73$ ls
  73-0040        device         i2c-83         new_device
  73-004e        i2c-78         i2c-84         of_node
  73-0050        i2c-79         i2c-85         power
  73-0070        i2c-80         i2c-dev        subsystem
  73-0072        i2c-81         mux_device     uevent
  delete_device  i2c-82         name
  /sys/bus/i2c/devices/i2c-73$ cd 73-0072
  /sys/bus/i2c/devices/i2c-73/73-0072$ ls
  channel-0   channel-4   driver      of_node
  channel-1   channel-5   idle_state  power
  channel-2   channel-6   modalias    subsystem
  channel-3   channel-7   name        uevent
  /sys/bus/i2c/devices/i2c-73/73-0072$ readlink channel-3
  ../i2c-81

```
在这里，我们得知 0x72 MUX 的通道 3 的逻辑 I2C 总线编号是 81。之后我们可以用这个
编号切换到它自己的 I2C Sysfs 目录，或发出 `i2c-tools` 命令。

提示：一旦你理解了带 MUX 的 I2C 拓扑，如果你的系统上可用，`I2C Tools
<https://i2c.wiki.kernel.org/index.php/I2C_Tools>`_ 中的命令
`i2cdetect -l
<https://manpages.debian.org/unstable/i2c-tools/i2cdetect.8.en.html>`_
可以让你轻松了解 I2C 拓扑概览。例如：
```

  $ i2cdetect -l | grep -e '\-73' -e _7 | sort -V
  i2c-7   i2c             npcm_i2c_7                              I2C adapter
  i2c-73  i2c             i2c-7-mux (chan_id 1)                   I2C adapter
  i2c-78  i2c             i2c-73-mux (chan_id 0)                  I2C adapter
  i2c-79  i2c             i2c-73-mux (chan_id 1)                  I2C adapter
  i2c-80  i2c             i2c-73-mux (chan_id 2)                  I2C adapter
  i2c-81  i2c             i2c-73-mux (chan_id 3)                  I2C adapter
  i2c-82  i2c             i2c-73-mux (chan_id 4)                  I2C adapter
  i2c-83  i2c             i2c-73-mux (chan_id 5)                  I2C adapter
  i2c-84  i2c             i2c-73-mux (chan_id 6)                  I2C adapter
  i2c-85  i2c             i2c-73-mux (chan_id 7)                  I2C adapter

```
### 固定的逻辑 I2C 总线编号


如果在 DTS 中未指定，当 I2C MUX 驱动被应用且 MUX 设备成功 probe 时，内核将基于
当前最大的逻辑总线编号，递增地为 MUX 通道分配逻辑总线编号。例如，如果系统中
`i2c-15` 是最高的逻辑总线编号，且一个 4 通道 MUX 被成功应用，那么 MUX 通道 0 将
获得 `i2c-16`，一直到 MUX 通道 3 获得 `i2c-19`。

内核软件开发人员能够在 DTS 中将扇出的 MUX 通道固定到静态的逻辑 I2C 总线编号。本文档
不会深入讲解如何在 DTS 中实现这一点，但我们可以在以下示例中看到：
`arch/arm/boot/dts/aspeed-bmc-facebook-wedge400.dts`

在上述示例中，物理 I2C 总线 2 上有一个地址为 0x70 的 8 通道 I2C MUX。该 MUX 的
通道 2 在 DTS 中被定义为 `imux18`，并通过 `aliases` 段中的
`i2c18 = &imux18;` 固定到逻辑 I2C 总线编号 18。

更进一步，可以设计一套便于人类记忆或通过算术计算得出的逻辑 I2C 总线编号方案。例如，
我们可以将总线 3 上 MUX 的扇出通道固定为从 30 开始。于是 30 将是总线 3 上 MUX 通道 0
的逻辑总线编号，而 37 将是总线 3 上 MUX 通道 7 的逻辑总线编号。

## I2C 设备


在之前的章节中，我们主要介绍的是 I2C 总线。本节让我们看看从链接名为 `${bus}-${addr}`
格式的 I2C 设备目录中可以了解到什么。名称中的 `${bus}` 部分是逻辑 I2C 总线的十进制
编号，而 `${addr}` 部分是每个设备 I2C 地址的十六进制编号。

### I2C 设备目录内容


在每个 I2C 设备目录内部，有一个名为 `name` 的文件。该文件说明内核驱动用来
匹配该设备的设备名称是什么：
```

  /sys/bus/i2c/devices/i2c-73$ cat 73-0040/name
  ina230
  /sys/bus/i2c/devices/i2c-73$ cat 73-0070/name
  pca9546
  /sys/bus/i2c/devices/i2c-73$ cat 73-0072/name
  pca9547

```
有一个名为 `driver` 的符号链接，用于说明使用了哪个 Linux 内核驱动：
```

  /sys/bus/i2c/devices/i2c-73$ readlink -f 73-0040/driver
  /sys/bus/i2c/drivers/ina2xx
  /sys/bus/i2c/devices/i2c-73$ readlink -f 73-0072/driver
  /sys/bus/i2c/drivers/pca954x

```
但如果 `driver` 链接一开始就不存在，则可能意味着内核驱动由于
以下原因未能成功 probe 该设备：
```

  /sys/bus/i2c/devices/i2c-73$ ls 73-0070/driver
  ls: 73-0070/driver: No such file or directory
  /sys/bus/i2c/devices/i2c-73$ dmesg | grep 73-0070
  pca954x 73-0070: probe failed
  pca954x 73-0070: probe failed

```
根据 I2C 设备的不同以及用于 probe 该设备的内核驱动的不同，设备目录中的内容也可能
不同。

### I2C MUX 设备


虽然你可能在前面的章节已经有所了解，I2C MUX 设备在其设备目录中会包含名为
`channel-*` 的符号链接。
```

  /sys/bus/i2c/devices/i2c-73$ ls -l 73-0072/channel-*
  lrwxrwxrwx ... 73-0072/channel-0 -> ../i2c-78
  lrwxrwxrwx ... 73-0072/channel-1 -> ../i2c-79
  lrwxrwxrwx ... 73-0072/channel-2 -> ../i2c-80
  lrwxrwxrwx ... 73-0072/channel-3 -> ../i2c-81
  lrwxrwxrwx ... 73-0072/channel-4 -> ../i2c-82
  lrwxrwxrwx ... 73-0072/channel-5 -> ../i2c-83
  lrwxrwxrwx ... 73-0072/channel-6 -> ../i2c-84
  lrwxrwxrwx ... 73-0072/channel-7 -> ../i2c-85

```
### I2C 传感器设备 / Hwmon


I2C 传感器设备也很常见。如果它们被某个内核 hwmon（硬件监控）驱动成功绑定，你将
在 I2C 设备目录中看到一个名为 `hwmon` 的目录。继续深入，你会找到 Hwmon
sysfs 接口：
```

  /sys/bus/i2c/devices/i2c-73/73-0040/hwmon/hwmon17$ ls
  curr1_input        in0_lcrit_alarm    name               subsystem
  device             in1_crit           power              uevent
  in0_crit           in1_crit_alarm     power1_crit        update_interval
  in0_crit_alarm     in1_input          power1_crit_alarm
  in0_input          in1_lcrit          power1_input
  in0_lcrit          in1_lcrit_alarm    shunt_resistor

```
关于 Hwmon Sysfs 的更多信息，请参阅该文档：

../hwmon/sysfs-interface.rst

### 在 I2C Sysfs 中实例化 I2C 设备


请参阅 instantiating-devices.rst 中的“方法 4：从用户空间实例化”一节。
