## 使用 kgdb、kdb 以及内核调试器内部机制


:Author: Jason Wessel

## 简介


内核有两个与调试核心（debug core）接口的不同调试器前端（kdb 和 kgdb）。如果你在编译和运行时正确配置内核，就可以使用其中任一调试器前端，并在它们之间动态切换。

Kdb 是一个简单的 shell 风格接口，你可以在带有键盘或串行控制台的系统控制台上使用它。你可以用它来检查内存、寄存器、进程列表、dmesg，甚至可以设置断点以在特定位置停止。Kdb 不是源码级调试器，尽管你可以设置断点并执行一些基本的内核运行控制。Kdb 主要旨在进行一些分析，以辅助开发或诊断内核问题。如果代码是用 `CONFIG_KALLSYMS` 构建的，你可以通过名称访问内核内置或内核模块中的一些符号。

Kgdb 旨在用作 Linux 内核的源码级调试器。它与 gdb 配合使用来调试 Linux 内核。期望是 gdb 可以“闯入”内核，以检查内存、变量并查看调用栈信息，类似于应用程序开发者使用 gdb 调试应用程序的方式。可以在内核代码中放置断点并执行一些有限的单步执行。

使用 kgdb 需要两台机器。其中一台是开发机，另一台是目标机。待调试的内核运行在目标机上。开发机运行一个针对 vmlinux 文件（包含符号，而非 bzImage、zImage、uImage 等引导镜像）的 gdb 实例。在 gdb 中，开发者指定连接参数并连接到 kgdb。开发者用 gdb 建立的连接类型取决于测试机内核中是否将 kgdb I/O 模块编译为内置或可加载内核模块。

## 编译内核


- 为了启用 kdb 的编译，你必须先启用 kgdb。

- kgdb 测试编译选项在 kgdb 测试套件一章中描述。

### kgdb 的内核配置选项


要启用 `CONFIG_KGDB`，你应在 `Kernel hacking --> Kernel debugging` 下查找并选择 `KGDB: kernel debugger`。

虽然并不强制要求你的 vmlinux 文件中有符号，但没有符号数据 gdb 往往没什么用处，因此你会想要开启 `CONFIG_DEBUG_INFO`，它在配置菜单中称为 `Compile the kernel with debug info`。

建议（但非必需）开启 `CONFIG_FRAME_POINTER` 内核选项，它在配置菜单中称为 :menuselection:`Compile the kernel with frame pointers`。该选项向编译后的可执行文件中插入代码，在不同位置将帧信息保存到寄存器或栈上，从而允许诸如 gdb 之类的调试器在调试内核时更准确地构造栈回溯。

如果你使用的架构支持内核选项 `CONFIG_STRICT_KERNEL_RWX`，你应该考虑将其关闭。该选项会将内核内存空间的某些区域标记为只读，从而阻止使用软件断点。如果你使用的架构支持，你可以在开启 `CONFIG_STRICT_KERNEL_RWX` 选项的情况下使用硬件断点，否则你需要关闭该选项。

接下来，你应选择一个或多个 I/O 驱动来连接调试主机和被调试的目标。早期启动调试需要一个支持早期调试的 KGDB I/O 驱动，且该驱动必须直接编译进内核。Kgdb I/O 驱动的配置通过内核或模块参数进行，你可以在描述 kgdboc 参数的章节中了解更多。

```

  # CONFIG_STRICT_KERNEL_RWX is not set
  CONFIG_FRAME_POINTER=y
  CONFIG_KGDB=y
  CONFIG_KGDB_SERIAL_CONSOLE=y

```
### kdb 的内核配置选项


Kdb 比位于内核调试核心之上的简单 gdbstub 要复杂得多。Kdb 必须实现一个 shell，并且还在内核的其他部分添加一些辅助函数，负责打印出有趣的数据，例如你运行 `lsmod` 或 `ps` 时会看到的内容。要将 kdb 构建进内核，你遵循与 kgdb 相同的步骤。

kdb 的主要配置选项是 `CONFIG_KGDB_KDB`，它在配置菜单中称为 `KGDB_KDB: include kdb frontend for kgdb`。理论上，如果你打算在串行端口上使用 kdb，那么在你配置 kgdb 时就应该已经选择了诸如 `CONFIG_KGDB_SERIAL_CONSOLE` 接口这样的 I/O 驱动。

如果你想像使用 PS/2 风格的键盘与 kdb 配合，你应选择 `CONFIG_KDB_KEYBOARD`，它在配置菜单中称为 :menuselection:`KGDB_KDB: keyboard as input device`。`CONFIG_KDB_KEYBOARD` 选项在 kgdb 的 gdb 接口中没有任何用途。`CONFIG_KDB_KEYBOARD` 选项仅与 kdb 配合工作。

```

  # CONFIG_STRICT_KERNEL_RWX is not set
  CONFIG_FRAME_POINTER=y
  CONFIG_KGDB=y
  CONFIG_KGDB_SERIAL_CONSOLE=y
  CONFIG_KGDB_KDB=y
  CONFIG_KDB_KEYBOARD=y

```
## 内核调试器引导参数


本节描述影响内核调试器配置的各种运行时内核参数。下一章涵盖 kdb 和 kgdb 的使用，并提供一些配置参数的示例。

### 内核参数：kgdboc


kgdboc 驱动最初是一个缩写，意为“kgdb over console（通过控制台使用 kgdb）”。如今它是配置如何从 gdb 与 kgdb 通信，以及你想用来与 kdb shell 交互的设备的主要机制。

对于 kgdb/gdb，kgdboc 设计用于与单个串行端口一起工作。它旨在覆盖你想将串行控制台用作主控制台并用它执行内核调试的情况。也可以在未被指定为系统控制台的串行端口上使用 kgdb。Kgdboc 可以配置为内核内置或内核可加载模块。只有将 kgdboc 作为内置编译进内核，你才能使用 `kgdbwait` 和早期调试。

可选地，你可以选择激活 kms（Kernel Mode Setting，内核模式设置）集成。当你将 kms 与 kgdboc 一起使用，并且你有一个具有原子模式设置钩子的视频驱动时，就可以在图形控制台上进入调试器。当内核执行恢复时，先前的图形模式将被恢复。这种集成可以作为一个有用的工具，在允许完整图形控制台应用程序运行的同时，辅助诊断崩溃或用 kdb 对内存进行分析。

#### kgdboc 参数


```

	kgdboc=[kms][[,]kbd][[,]serial_device][,baud]

```
上面列出的顺序必须遵守，如果你同时使用任何可选配置的话。

缩写：

- kms = 内核模式设置（Kernel Mode Setting）

- kbd = 键盘（Keyboard）

你可以根据是否使用 kdb 和/或 kgdb，在以下场景之一中配置 kgdboc 使用键盘和/或串行设备。如果你同时使用上述任何可选配置，必须遵守上面列出的顺序。使用 kms + 仅 gdb 通常不是一个有用的组合。

##### 使用可加载模块或内置


1. 作为内核内置：

```

	kgdboc=<tty-device>,[baud]

```
2. 作为内核可加载模块：

```

	modprobe kgdboc kgdboc=<tty-device>,[baud]

   Here are two examples of how you might format the kgdboc string. The
   first is for an x86 target using the first serial port. The second
   example is for the ARM Versatile AB using the second serial port.

   1. ``kgdboc=ttyS0,115200``

   2. ``kgdboc=ttyAMA1,115200``

```
##### 用 sysfs 在运行时配置 kgdboc


在运行时，你可以通过向 sysfs 写入参数来启用或禁用 kgdboc。这里有两个示例：

```

	echo ttyS0 > /sys/module/kgdboc/parameters/kgdboc

```
```

	echo "" > /sys/module/kgdboc/parameters/kgdboc

```

   如果你正在配置已经配置好或已打开的 tty 上的控制台，则无需指定波特率。

##### 更多示例


你可以根据是否使用 kdb 和/或 kgdb，在以下场景之一中配置 kgdboc 使用键盘和/或串行设备。

```

	kgdboc=<serial_device>[,baud]

   Example::

	kgdboc=ttyS0,115200

```
```

	kgdboc=kbd,<serial_device>[,baud]

   Example::

	kgdboc=kbd,ttyS0,115200

```
```

	kgdboc=kbd

```
```

	kgdboc=kms,kbd

```
```

	kgdboc=kms,kbd,ttyS0,115200

```

   Kgdboc 不支持通过 gdb 远程协议中断目标。你必须手动发送 `SysRq-G`，除非你有一个将控制台输出分流到终端程序的代理。控制台代理为调试器提供一个独立的 TCP 端口，为“人类”控制台提供另一个独立的 TCP 端口。该代理可以替你发送 `SysRq-G`。

当在没有调试器代理的情况下使用 kgdboc 时，你最终可能会在两个入口点之一连接调试器。如果在加载 kgdboc 后发生异常，控制台应打印一条消息，说明它正在等待调试器。在这种情况下，你断开终端程序，然后连接调试器取而代之。如果你想中断目标系统并强制进入调试会话，你必须发出 `Sysrq` 序列，然后键入字母 `g`。然后你断开终端会话并连接 gdb。如果你不喜欢这样，你的选择是修改 gdb 让它在初始连接时也替你发送 `SysRq-G`，或者使用允许未修改的 gdb 进行调试的调试器代理。

### 内核参数：`kgdboc_earlycon`


如果你指定了内核参数 `kgdboc_earlycon`，并且你的串行驱动注册了一个支持轮询（不需要中断并实现非阻塞 read() 函数）的引导控制台，kgdb 将尝试使用引导控制台工作，直到它可以切换到 `kgdboc` 参数指定的常规 tty 驱动。

通常只有一个引导控制台（尤其是实现了 read() 函数的那个），因此仅添加 `kgdboc_earlycon` 本身就足以使其工作。如果你有多个引导控制台，可以添加引导控制台的名称以区分。注意，通过引导控制台层和 tty 层注册的同一端口的名称并不相同。

```

   kgdboc_earlycon=qcom_geni kgdboc=ttyMSM0

```
```

   kgdboc_earlycon kgdboc=ttyMSM0

```
### 内核参数：`kgdbwait`


内核命令行选项 `kgdbwait` 使 kgdb 在内核启动期间等待调试器连接。只有当你将 kgdb I/O 驱动编译进内核，并将该 I/O 驱动配置指定为内核命令行选项时，才能使用此选项。`kgdbwait` 参数应始终位于内核命令行中 kgdb I/O 驱动的配置参数之后，否则在要求内核使用它来等待之前，该 I/O 驱动将不会被配置。

当你使用此选项时，内核会在 I/O 驱动和架构允许的最早时机停止并等待。如果你将 kgdb I/O 驱动构建为可加载内核模块，`kgdbwait` 将不起任何作用。

### 内核参数：`kgdbcon`


`kgdbcon` 特性允许你在 gdb 连接到内核时，在 gdb 内部看到 printk() 消息。Kdb 不使用 kgdbcon 特性。

Kgdb 支持在调试器已连接并运行时，使用 gdb 串行协议向调试器发送控制台消息。有两种方式激活此特性。

```

	kgdbcon

```
```

	echo 1 > /sys/module/debug_core/parameters/kgdb_use_con

```

   如果你在配置 kgdb I/O 驱动之后执行此操作，该设置要到下一次重新配置 I/O 时才会生效。


   你不能在作为

```

	console=ttyS0,115200 kgdboc=ttyS0 kgdbcon

```
系统控制台的 tty 上同时使用 kgdboc + kgdbcon。可以将此选项与 kgdboc 一起用于不是系统控制台的 tty 上。

### 运行时参数：`kgdbreboot`


kgdbreboot 特性允许你更改调试器处理重启通知的方式。行为有 3 种选择。默认行为始终设为 0。


  :widths: 1 10 8

  - - 1
    - `echo -1 > /sys/module/debug_core/parameters/kgdbreboot`
    - 完全忽略重启通知。

  - - 2
    - `echo 0 > /sys/module/debug_core/parameters/kgdbreboot`
    - 向任何已连接的调试器客户端发送分离消息。

  - - 3
    - `echo 1 > /sys/module/debug_core/parameters/kgdbreboot`
    - 在重启通知时进入调试器。

### 内核参数：`nokaslr`


如果你使用的架构默认启用了 KASLR，你应该考虑将其关闭。KASLR 会随机化内核映像映射的虚拟地址，并使从 vmlinux 的符号表解析内核符号地址的 gdb 感到困惑。

### 内核参数：`rodata`


`CONFIG_STRICT_KERNEL_RWX` 默认开启，并且在一些架构（例如 arm64）上对 menuconfig 不可见，在这种情况下你可以向内核传递 `rodata=off`。

## 使用 kdb


### 串行端口上 kdb 的快速入门


这是一个如何使用 kdb 的简短示例。

```

	console=ttyS0,115200 kgdboc=ttyS0,115200 nokaslr

   OR

   Configure kgdboc after the kernel has booted; assuming you are using
   a serial port console::

	echo ttyS0 > /sys/module/kgdboc/parameters/kgdboc

```
2. 手动进入内核调试器，或者等待 oops 或故障。有几种方式可以手动进入内核调试器；它们都涉及使用 `SysRq-G`，这意味着你必须在内核配置中启用了 `CONFIG_MAGIC_SYSRQ=y`。

```

	echo g > /proc/sysrq-trigger

   -  Example using minicom 2.2

      Press: `CTRL-A` `f` `g`

   -  When you have telneted to a terminal server that supports sending
      a remote break

      Press: `CTRL-]`

      Type in: ``send break``

      Press: `Enter` `g`

```
3. 在 kdb 提示符下，你可以运行 `help` 命令来查看可用命令的完整列表。

   kdb 中一些有用的命令包括：

   =========== =================================================================
   `lsmod`   显示内核模块加载的位置
   `ps`      仅显示活动进程
   `ps A`    显示所有进程
   `summary` 显示内核版本信息和内存使用情况
   `bt`      使用 dump_stack() 获取当前进程的回溯
   `dmesg`   查看内核 syslog 缓冲区
   `go`      继续系统运行
   =========== =================================================================

4. 当你使用完 kdb 后，需要考虑重启系统，或者使用 `go` 命令恢复正常的内核执行。如果你让内核暂停了较长时间，依赖及时联网或任何与真实墙上时钟时间相关的事务的应用程序可能会受到不利影响，因此在使用内核调试器时你应考虑到这一点。

### 使用连接键盘的控制台的 kdb 快速入门


这是一个如何使用键盘配合 kdb 的简短示例。

```

	kgdboc=kbd

   OR

   Configure kgdboc after the kernel has booted::

	echo kbd > /sys/module/kgdboc/parameters/kgdboc

```
2. 手动进入内核调试器，或者等待 oops 或故障。有几种方式可以手动进入内核调试器；它们都涉及使用 `SysRq-G`，这意味着你必须在内核配置中启用了 `CONFIG_MAGIC_SYSRQ=y`。

```

	echo g > /proc/sysrq-trigger

   -  Example using a laptop keyboard:

      Press and hold down: `Alt`

      Press and hold down: `Fn`

      Press and release the key with the label: `SysRq`

      Release: `Fn`

      Press and release: `g`

      Release: `Alt`

   -  Example using a PS/2 101-key keyboard

      Press and hold down: `Alt`

      Press and release the key with the label: `SysRq`

      Press and release: `g`

      Release: `Alt`

```
3. 现在键入一个 kdb 命令，例如 `help`、`dmesg`、`bt` 或 `go` 来继续内核执行。

## 使用 kgdb / gdb


为了使用 kgdb，你必须通过向某个 kgdb I/O 驱动传递配置信息来激活它。如果你不传递任何配置信息，kgdb 将什么也不做。只有当 kgdb I/O 驱动被加载并配置后，kgdb 才会主动挂接到内核陷阱钩子上。如果你取消配置某个 kgdb I/O 驱动，kgdb 将注销所有内核钩子点。

如果启用了 `CONFIG_SYSFS` 和 `CONFIG_MODULES`，所有 kgdb I/O 驱动都可以在运行时通过向 `/sys/module/<driver>/parameter/<option>` echo 新的配置字符串来重新配置。通过传递空字符串可以取消配置该驱动。在调试器连接时不能更改配置。在尝试取消配置 kgdb I/O 驱动之前，务必使用 `detach` 命令分离调试器。

### 通过串行端口用 gdb 连接


1. 配置 kgdboc

```

	kgdboc=ttyS0,115200

   OR

   Configure kgdboc after the kernel has booted::

	echo ttyS0 > /sys/module/kgdboc/parameters/kgdboc

```
2. 停止内核执行（闯入调试器）

   为了通过 kgdboc 连接到 gdb，内核必须先被停止。有几种方式可以停止内核，包括使用 kgdbwait 作为引导参数、通过 `SysRq-G`，或者让内核一直运行直到它发生异常并在该处等待调试器连接。

```

	echo g > /proc/sysrq-trigger

   -  Example using minicom 2.2

      Press: `CTRL-A` `f` `g`

   -  When you have telneted to a terminal server that supports sending
      a remote break

      Press: `CTRL-]`

      Type in: ``send break``

      Press: `Enter` `g`

```
3. 从 gdb 连接

```

           % gdb ./vmlinux
           (gdb) set serial baud 115200
           (gdb) target remote /dev/ttyS0


   Example (kgdb to a terminal server on TCP port 2012)::

           % gdb ./vmlinux
           (gdb) target remote 192.168.2.2:2012


   Once connected, you can debug a kernel the way you would debug an
   application program.

   If you are having problems connecting or something is going seriously
   wrong while debugging, it will most often be the case that you want
   to enable gdb to be verbose about its target communications. You do
   this prior to issuing the ``target remote`` command by typing in::

	set debug remote 1

```
记住，如果你在 gdb 中继续运行，并且需要再次“闯入”，你需要再发出一个 `SysRq-G`。很容易创建一个简单的入口点：在 `sys_sync` 处放置一个断点，然后你可以从 shell 或脚本运行 `sync` 来闯入调试器。

## kgdb 与 kdb 的互操作性


可以在 kdb 和 kgdb 之间动态切换。调试核心会记住你上次使用的是哪一个，并自动以相同模式启动。

### 在 kdb 和 kgdb 之间切换


#### 从 kgdb 切换到 kdb


有两种方式可以从 kgdb 切换到 kdb：你可以使用 gdb 发出一个维护包（maintenance packet），或者盲目地键入命令 `$3#33`。每当内核调试器在 kgdb 模式下停止时，它会打印消息 `KGDB or $3#33 for KDB`。需要注意的是，你必须一次性正确地键入该序列。你不能键入退格或删除，因为 kgdb 会将其解释为调试流的一部分。

```

	$3#33

```
```

	maintenance packet 3

   .. note::

     Now you must kill gdb. Typically you press `CTRL-Z` and issue
     the command::

	kill -9 %

```
#### 从 kdb 切换到 kgdb


有两种方式可以从 kdb 切换到 kgdb。你可以从 kdb shell 提示符发出 kgdb 命令来手动进入 kgdb 模式，或者在 kdb shell 提示符处于活动状态时连接 gdb。kdb shell 会查找 gdb 通过 gdb 远程协议发出的典型首条命令，如果它看到其中一条命令，就会自动切换到 kgdb 模式。

```

	kgdb

```
2. 在 kdb 提示符下，断开终端程序，然后连接 gdb 取而代之。

### 从 gdb 运行 kdb 命令


可以使用 gdb 的 monitor 命令，从 gdb 运行一组受限的 kdb 命令。你不应执行任何运行控制或断点操作，因为这会扰乱内核调试器的状态。如果你已连接 gdb，应该使用 gdb 来进行断点和运行控制操作。更有用的命令是诸如 lsmod、dmesg、ps 或可能的一些内存信息命令。要查看所有可运行的 kdb 命令，你可以运行 `monitor help`。

```

    (gdb) monitor ps
    1 idle process (state I) and
    27 sleeping system daemon (state M) processes suppressed,
    use 'ps A' to see all.
    Task Addr       Pid   Parent [*] cpu State Thread     Command

    0xc78291d0        1        0  0    0   S  0xc7829404  init
    0xc7954150      942        1  0    0   S  0xc7954384  dropbear
    0xc78789c0      944        1  0    0   S  0xc7878bf4  sh
    (gdb)

```
## kgdb 测试套件


当在内核配置中启用了 kgdb 时，你也可以选择启用配置参数 `KGDB_TESTS`。打开它会启用一个特殊的 kgdb I/O 模块，该模块旨在测试 kgdb 的内部函数。

kgdb 测试主要面向开发者，用于测试 kgdb 内部机制，以及作为开发新的 kgdb 架构特定实现的工具。这些测试并不是真正给 Linux 内核的终端用户用的。主要的文档来源是查看 `drivers/misc/kgdbts.c` 文件。

kgdb 测试套件也可以在编译时配置为运行核心测试集，方法是设置内核配置参数 `KGDB_TESTS_ON_BOOT`。这个特定选项面向自动化回归测试，不需要修改内核引导配置参数。如果开启了它，可以通过指定 `kgdbts=` 作为内核引导参数来禁用 kgdb 测试套件。

## 内核调试器内部机制


### 架构相关细节


内核调试器被组织为若干组件：

1. 调试核心

   调试核心位于 `kernel/debugger/debug_core.c`。它包含：

   - 一个通用的 OS 异常处理程序，包括在多 CPU 系统上将处理器同步到停止状态。

   - 与 kgdb I/O 驱动通信的 API

   - 调用架构特定的 kgdb 实现的 API

   - 在使用调试器时对内存执行安全读写的逻辑

   - 软件断点的完整实现，除非被架构覆盖

   - 调用 kdb 或 kgdb 前端到调试核心的 API。

   - 用于原子内核模式设置的结构和回调 API。

      .. note:: kgdboc 是调用 kms 回调的地方。

2. kgdb 架构特定实现

   该实现通常位于 `arch/*/kernel/kgdb.c`。例如，`arch/x86/kernel/kgdb.c` 包含了实现硬件断点的细节，以及在本架构上动态注册和注销陷阱处理程序的初始化。架构特定的部分实现了：

   - 包含一个架构特定的陷阱捕获器，它调用 kgdb_handle_exception() 来启动 kgdb 工作

   - 在 gdb 特定包格式与 struct pt_regs 之间的转换

   - 架构特定陷阱钩子的注册和注销

   - 任何特殊的异常处理和清理

   - NMI 异常处理和清理

   - （可选）硬件断点

3. gdbstub 前端（即 kgdb）

   gdbstub 位于 `kernel/debug/gdbstub.c`。它包含：

   - 实现 gdb 串行协议的全部逻辑

4. kdb 前端

   kdb 调试器 shell 被拆分为若干组件。kdb 核心位于 kernel/debug/kdb。在其他一些内核组件中有若干辅助函数，使 kdb 能够在不获取可能导致内核死锁的锁的情况下检查和报告内核信息。kdb 核心实现了以下功能。

   - 一个简单的 shell

   - kdb 核心命令集

   - 用于注册额外 kdb shell 命令的注册 API。

      - 一个自包含 kdb 模块的好例子是用于转储 ftrace 缓冲区的 `ftdump` 命令。参见：`kernel/trace/trace_kdb.c`

      - 关于如何动态注册新 kdb 命令的示例，你可以从 `samples/kdb/kdb_hello.c` 构建 kdb_hello.ko 内核模块。要构建此示例，你可以在内核配置中设置 `CONFIG_SAMPLES=y` 和 `CONFIG_SAMPLE_KDB=m`。之后运行 `modprobe kdb_hello`，下次进入 kdb shell 时，你就可以运行 `hello` 命令。

   - kdb_printf() 的实现，它直接将消息发送到 I/O 驱动，绕过内核日志。

   - kdb shell 的软件/硬件断点管理

5. kgdb I/O 驱动

   每个 kgdb I/O 驱动必须为实现以下内容提供实现：

   - 通过内置或模块进行配置

   - 动态配置和 kgdb 钩子注册调用

   - 读写字符接口

   - 用于从 kgdb 核心取消配置的清理处理程序

   - （可选）早期调试方法

   任何给定的 kgdb I/O 驱动都必须与硬件非常紧密地配合工作，并且必须以不启用中断或改变系统上下文其他部分而不完全恢复它们的方式来进行。kgdb 核心在需要输入时会反复“轮询”kgdb I/O 驱动以获取字符。如果没有可用数据，I/O 驱动应立即返回。这样做为将来以某种方式接触看门狗硬件提供了可能，使得在启用这些硬件时目标系统不会重置。

如果你打算为新的架构添加 kgdb 架构特定支持，该架构应在其架构特定的 Kconfig 文件中定义 `HAVE_ARCH_KGDB`。这将为该架构启用 kgdb，此时你必须创建一个架构特定的 kgdb 实现。

在每个架构的 `asm/kgdb.h` 文件中必须设置一些标志。它们是：

- `NUMREGBYTES`：
     所有寄存器的字节大小，以便我们确保它们都能放入一个包中。

- `BUFMAX`：
     GDB 将读入的缓冲区的字节大小。它必须大于 NUMREGBYTES。

- `CACHE_FLUSH_IS_SAFE`：
     如果调用 flush_cache_range 或 flush_icache_range 始终安全，则设为 1。在某些架构上，由于我们将其他 CPU 保持在等待状态，这些函数在 SMP 上调用可能不安全。

在 `kernel/kgdb.c` 中还有些用于公共后端的以下函数，必须由架构特定的后端提供，除非标记为（可选），在这种情况下，如果架构不需要提供特定实现，可以使用默认函数。

   :internal:

### kgdboc 内部机制


#### kgdboc 与 uart


kgdboc 驱动实际上是一个非常薄的驱动，它依赖于底层到硬件驱动的“轮询钩子（polling hooks）”，tty 驱动就挂载在这些钩子上。在 kgdboc 的最初实现中，serial_core 被修改为暴露一个低级 UART 钩子，用于在原子上下文中以轮询模式读写单个字符。当 kgdb 向调试器发出 I/O 请求时，kgdboc 调用 serial_core 中的回调，该回调进而使用 UART 驱动中的回调。

当将 kgdboc 与 UART 配合使用时，UART 驱动必须在 struct uart_ops 中实现两个回调。

```

    #ifdef CONFIG_CONSOLE_POLL
        .poll_get_char = serial8250_get_poll_char,
        .poll_put_char = serial8250_put_poll_char,
    #endif


```
围绕创建轮询驱动的任何实现细节都使用 `#ifdef CONFIG_CONSOLE_POLL`，如上所示。请记住，轮询钩子必须以可以从原子上下文调用，并在返回时恢复 UART 芯片状态的方式实现，以便系统能在调试器分离时恢复正常。对任何你考虑的锁都要非常小心，因为这里的失败很可能意味着要按下复位按钮。

#### kgdboc 与键盘


kgdboc 驱动包含配置与已连接键盘通信的逻辑。键盘基础设施只有在内核配置中设置了 `CONFIG_KDB_KEYBOARD=y` 时才会编译进内核。

PS/2 类型键盘的核心轮询键盘驱动位于 `drivers/char/kdb_keyboard.c`。当 kgdboc 在名为 :c`kdb_poll_funcs[]` 的数组中填充回调时，该驱动会被挂接到调试核心。kdb_get_kbd_char() 是轮询硬件以获取单个字符输入的顶层函数。

#### kgdboc 与 kms


kgdboc 驱动包含逻辑，在你使用 `kgdboc=kms,kbd` 时请求图形显示切换到文本上下文，前提是你有一个带有帧缓冲控制台和原子内核模式设置支持的视频驱动。

每次进入内核调试器时，它会调用 kgdboc_pre_exp_handler()，该函数进而调用虚拟控制台层中的 con_debug_enter()。在恢复内核执行时，内核调试器调用 kgdboc_post_exp_handler()，该函数进而调用 con_debug_leave()。


## 致谢


以下人员对本文档做出了贡献：

1. Amit Kale <amitkale@linsyssoft.com>

2. Tom Rini <trini@kernel.crashing.org>

2008 年 3 月，本文档由以下人员完全重写：

- Jason Wessel <jason.wessel@windriver.com>

2010 年 1 月，本文档更新以包含 kdb。

- Jason Wessel <jason.wessel@windriver.com>
