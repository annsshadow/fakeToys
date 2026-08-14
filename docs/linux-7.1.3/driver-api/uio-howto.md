## 用户空间 I/O HOWTO


:Author: Hans-Jürgen Koch Linux developer, Linutronix
:Date:   2006-12-11

## 关于本文档


### 翻译


如果你知道本文档的任何翻译版本，或者你有兴趣翻译它，请给我发邮件 hjk@hansjkoch.de。

### 前言


对于许多类型的设备，创建一个 Linux 内核驱动是杀鸡用牛刀。
真正需要的只是某种处理中断并提供对设备内存空间访问的方式。控制设备的逻辑并不一定非要在内核之中，因为设备不需要利用内核提供的任何其他资源。
这样的一类常见设备是工业 I/O 卡。

为了应对这种情况，设计了用户空间 I/O 系统（UIO）。对于典型的工业 I/O 卡，只需要一个非常小的内核模块。驱动的主要部分将在用户空间中运行。
这简化了开发，并降低了内核模块内部出现严重 bug 的风险。

请注意，UIO 并非一个通用的驱动接口。那些已经被其他内核子系统（如网络、串口或 USB）处理得很好的设备，不适合作为 UIO 驱动。
理想适用于 UIO 驱动硬件必须满足以下所有条件：

- 设备拥有可被映射的内存。该设备可以通过写入这块内存被完全控制。

- 设备通常会产生中断。

- 设备不适合归入任何一个标准的内核子系统。

### 致谢


我要感谢 Linutronix 的 Thomas Gleixner 与 Benedikt Spranger，他们不仅编写了 UIO 的大部分代码，还通过向我提供各种背景信息，在编写本 HOWTO 时给予了极大的帮助。

### 反馈


发现本文档有错误？（或者也许有正确的地方？）我很乐意听到你的意见。请给我发邮件 hjk@hansjkoch.de。

## 关于 UIO


如果你为你的卡使用 UIO 驱动，你会得到：

- 只需编写并维护一个小的内核模块。

- 在用户空间中使用你惯用的所有工具和库来开发驱动的主要部分。

- 你驱动中的 bug 不会使内核崩溃。

- 更新你的驱动无需重新编译内核。

### UIO 如何工作


每个 UIO 设备都通过一个设备文件以及若干 sysfs 属性文件来访问。
第一个设备的设备文件将被称为 `/dev/uio0`，后续设备为 `/dev/uio1`、`/dev/uio2`，以此类推。

`/dev/uioX` 用于访问卡的地址空间。只需使用 `mmap()` 来访问你卡上的寄存器或 RAM 位置。

中断通过从 `/dev/uioX` 读取来处理。对 `/dev/uioX` 的一个阻塞式 `read()` 会在中断一发生时立即返回。
你也可以使用 `/dev/uioX` 上的 `select()` 来等待一个中断。
从 `/dev/uioX` 读取的整数值代表中断的累计计数。你可以用这个数字来判断你是否错过了某些中断。

对于一些内部有多个中断源、但没有独立的 IRQ 掩码与状态寄存器的硬件，可能存在这样的情况：如果内核处理例程通过写入芯片的 IRQ 寄存器禁用了它们，用户空间就无法确定中断源是什么。
在这种情况下，内核必须完全禁用 IRQ，以保留芯片的寄存器不被改动。现在用户空间部分可以确定中断的原因，但它无法重新启用中断。
另一个边界情况是那些将重新启用中断作为一个对组合 IRQ 状态/确认寄存器的读-修改-写操作的芯片。
如果恰好一个新中断同时发生，这将存在竞争。

为了解决这些问题，UIO 还实现了一个 write() 函数。它通常不被使用，对于只有单个中断源、或拥有独立 IRQ 掩码与状态寄存器的硬件可以忽略它。
但如果你需要它，向 `/dev/uioX` 写入将调用由驱动实现的 `irqcontrol()` 函数。
你必须写入一个通常是 0 或 1 的 32 位值，以禁用或启用中断。
如果驱动没有实现 `irqcontrol()`，`write()` 将返回 `-ENOSYS`。

为了正确处理中断，你的自定义内核模块可以提供它自己的中断处理例程。它会被内置的处理例程自动调用。

对于不产生中断但需要被轮询的卡，可以设置一个定时器，以可配置的时间间隔触发中断处理例程。
这个中断模拟是通过从定时器的事件处理例程中调用 `uio_event_notify()` 来完成的。

每个驱动都提供用于读取或写入变量的属性。这些属性可通过 sysfs 文件访问。
一个自定义的内核驱动模块可以向 uio 驱动所拥有的设备添加它自己的属性，但此时不会添加到 UIO 设备本身。
如果发现这有用，将来这可能会改变。

UIO 框架提供以下标准属性：

- `name`：你的设备名。建议为此使用你的内核模块的名字。

- `version`：由你的驱动定义的一个版本字符串。这使得你驱动的
  用户空间部分能够处理内核模块的不同版本。

- `event`：自上次读取设备节点以来，由驱动处理的中断总数。

这些属性出现在 `/sys/class/uio/uioX` 目录下。
请注意，这个目录可能是一个符号链接，而不是一个真实的目录。任何访问它的用户空间代码都必须能够处理这一点。

每个 UIO 设备可以使一个或更多内存区域可用于内存映射。这是必要的，因为一些工业 I/O 卡在驱动中需要访问多于一个 PCI 内存区域。

每个映射在 sysfs 中都有自己的目录，第一个映射显示为 `/sys/class/uio/uioX/maps/map0/`。
后续的映射创建目录 `map1/`、`map2/`，依此类推。这些目录只有在映射的大小不为 0 时才会出现。

每个 `mapX/` 目录包含四个只读文件，显示内存的属性：

- `name`：这个映射的一个字符串标识符。这是可选的，字符串可以为空。驱动可以设置它，以便用户空间更容易找到正确的映射。

- `addr`：可以被映射的内存地址。

- `size`：由 addr 指向的内存的大小，以字节计。

- `offset`：必须加到由 `mmap()` 返回的指针上，以到达实际设备内存的偏移量，以字节计。
  如果设备的内存不是页对齐的，这就很重要。记住 `mmap()` 返回的指针总是页对齐的，所以总是加上这个偏移量是个好习惯。

在用户空间，不同的映射通过调整 `mmap()` 调用的 `offset` 参数来区分。
要映射映射 N 的内存，你必须使用 N 倍的页大小作为你的
```
    offset = N * getpagesize();
```
有时会有一些带有类内存区域、却无法用这里描述的技术映射的硬件，但仍有办法从用户空间访问它们。
最常见的例子是 x86 的 ioport。在 x86 系统上，用户空间可以使用 `ioperm()`、`iopl()`、`inb()`、`outb()` 以及类似的函数来访问这些 ioport。

由于这些 ioport 区域无法被映射，它们不会像上面描述的普通内存那样出现在 `/sys/class/uio/uioX/maps/` 下。
在没有关于端口区域信息的情况下，驱动的 user 空间部分很难弄清哪些端口属于哪个 UIO 设备。

为了应对这种情况，新增了目录 `/sys/class/uio/uioX/portio/`。
只有当驱动想要将一个或多个端口区域的信息传递给用户空间时，它才存在。如果是这样，名为 `port0`、`port1` 等等的子目录将出现在 `/sys/class/uio/uioX/portio/` 之下。

每个 `portX/` 目录包含四个只读文件，显示端口区域的 name、start、size 和 type：

- `name`：这个端口区域的一个字符串标识符。字符串是可选的，可以为空。驱动可以设置它，以便用户空间更容易找到某个端口区域。

- `start`：这个区域的第一个端口。

- `size`：这个区域中端口的数量。

- `porttype`：描述端口类型的一个字符串。

## 编写你自己的内核模块


请以 `uio_cif.c` 为例。以下段落解释了这个文件的不同部分。

### struct uio_info


这个结构告诉框架你的驱动的详细信息，其中一些成员是必需的，其他是可选的。

- `const char *name`：必需。你的驱动在 sysfs 中显示的名字。我建议为此使用你的模块名。

- `const char *version`：必需。这个字符串出现在 `/sys/class/uio/uioX/version` 中。

- `struct uio_mem mem[ MAX_UIO_MAPS ]`：如果你有可以用 `mmap()` 映射的内存，则必需。对于每个映射，你需要填充一个 `uio_mem` 结构。详见下面的描述。

- `struct uio_port port[ MAX_UIO_PORTS_REGIONS ]`：如果你想将有关 ioport 的信息传递给用户空间，则必需。对于每个端口区域，你需要填充一个 `uio_port` 结构。详见下面的描述。

- `long irq`：必需。如果你的硬件产生中断，确定 irq 号是你模块在初始化期间的任务。如果你没有硬件产生的中断，但想以其他方式触发中断处理例程，将 `irq` 设为 `UIO_IRQ_CUSTOM`。如果你根本没有中断，你可以将 `irq` 设为 `UIO_IRQ_NONE`，尽管这很少有意义。

- `unsigned long irq_flags`：如果你已将 `irq` 设为某个硬件中断号，则必需。这里给出的标志将用于调用 `request_irq()`。

- `int (**mmap)(struct uio_info **info, struct vm_area_struct *vma)`：可选。如果你需要一个特殊的 `mmap()` 函数，你可以在这里设置它。如果这个指针不是 NULL，你的 `mmap()` 将被调用，而不是内置的那个。

- `int (**open)(struct uio_info **info, struct inode *inode)`：可选。你也许想要有自己的 `open()`，例如仅当你的设备实际被使用时才启用中断。

- `int (**release)(struct uio_info **info, struct inode *inode)`：可选。如果你定义了自己的 `open()`，你可能也想要一个自定义的 `release()` 函数。

- `int (**irqcontrol)(struct uio_info **info, s32 irq_on)`：可选。如果你需要通过写入 `/dev/uioX` 来从用户空间启用或禁用中断，你可以实现这个函数。参数 `irq_on` 为 0 时禁用中断，为 1 时启用它们。

通常，你的设备会有一个或多个可以映射到用户空间的内存区域。对于每个区域，你必须在 `mem[]` 数组中建立一个 `struct uio_mem`。
以下是 `struct uio_mem` 字段的描述：

- `const char *name`：可选。设置它以帮助识别内存区域，它会显示在对应的 sysfs 节点中。

- `int memtype`：如果使用了映射则必需。如果你卡上有要被映射的物理内存，将其设为 `UIO_MEM_PHYS`。对于逻辑内存（例如用 `__get_free_pages()` 而非 kmalloc() 分配的）使用 `UIO_MEM_LOGICAL`。还有用于虚拟内存的 `UIO_MEM_VIRTUAL`。

- `phys_addr_t addr`：如果使用了映射则必需。填入你的内存块的地址。这个地址就是在 sysfs 中出现的那个。

- `resource_size_t size`：填入 `addr` 指向的内存块的大小。如果 `size` 为零，该映射被视为未使用。注意你**必须**将所有未使用的映射的 `size` 初始化为零。

- `void *internal_addr`：如果你必须从你的内核模块内部访问这个内存区域，你会想通过使用类似 `ioremap()` 的方式来在内部映射它。这个函数返回的地址不能被映射到用户空间，所以你绝不可将它存入 `addr`。改用 `internal_addr` 来记住这样的地址。

请不要触碰 `struct uio_mem` 的 `map` 元素！它由 UIO 框架用来为这个映射建立 sysfs 文件。就让它原样不动。

有时，你的设备会有一个或多个无法映射到用户空间的端口区域。但如果用户空间还有其他方式访问这些端口，那么让关于这些端口的信息在 sysfs 中可用是有意义的。
对于每个区域，你必须在 `port[]` 数组中建立一个 `struct uio_port`。以下是 `struct uio_port` 字段的描述：

- `char *porttype`：必需。将其设为预定义常量之一。对 x86 架构中发现的 ioport 使用 `UIO_PORT_X86`。

- `unsigned long start`：如果使用了端口区域则必需。填入这个区域第一个端口的编号。

- `unsigned long size`：填入这个区域中端口的数量。如果 `size` 为零，该区域被视为未使用。注意你**必须**将所有未使用区域的 `size` 初始化为零。

请不要触碰 `struct uio_port` 的 `portio` 元素！它由 UIO 框架在内部用来为这个区域建立 sysfs 文件。就让它原样不动。

### 添加中断处理例程


你需要在你的中断处理例程中做什么，取决于你的硬件以及你想如何处理它。你应当尽量让你的内核中断处理例程中的代码量保持最小。
如果你的硬件在每次中断后不需要你**必须**执行的任何动作，那么你的处理例程可以为空。

另一方面，如果你的硬件**需要**在每次中断后执行某些动作，那么你**必须**在你的内核模块中完成它。
注意你不能依赖你驱动的 user 空间部分。你的 user 空间程序可能在任何时候终止，可能留下你的硬件处于仍需要正确处理中断的状态。

也可能有这样的情况：你想在每次中断时从硬件读取数据，并将其缓冲在你为此目的分配的一块内核内存中。
借助这种技术，如果你的 user 空间程序错过了一次中断，你可以避免数据丢失。

关于共享中断的一点说明：你的驱动应当尽可能支持中断共享。这当且仅当你的驱动能够检测你的硬件是否触发了中断时才可能。这通常通过查看一个中断状态寄存器来完成。
如果你的驱动看到 IRQ 位确实被置位，它将执行它的动作，并且处理例程返回 IRQ_HANDLED。
如果驱动检测到不是你的硬件引起了中断，它将什么都不做并返回 IRQ_NONE，让内核得以调用下一个可能的中断处理例程。

如果你决定不支持共享中断，你的卡将无法在没有空闲中断的计算机上工作。由于这在 PC 平台上经常发生，你可以通过支持中断共享来省去很多麻烦。

### 对平台设备使用 uio_pdrv


在许多情况下，平台设备的 UIO 驱动可以以通用的方式处理。在你定义 `struct platform_device` 的同一个地方，你只需同时实现你的中断处理例程并填充你的 `struct uio_info`。
一个指向这个 `struct uio_info` 的指针随后被用作你的平台设备的 `platform_data`。

你还需要建立一个 `struct resource` 数组，包含你的内存映射的地址与大小。这些信息使用 `struct platform_device` 的 `.resource` 与 `.num_resources` 元素传递给驱动。

你现在必须将 `struct platform_device` 的 `.name` 元素设为 `"uio_pdrv"`，以使用通用的 UIO 平台设备驱动。
这个驱动会根据给定的资源填充 `mem[]` 数组，并注册该设备。

这种方法的优点是你只需编辑一个你无论如何都需要编辑的文件。你不必创建一个额外的驱动。

### 对平台设备使用 uio_pdrv_genirq


特别是在嵌入式设备中，你经常会发现中断引脚连接到它自己专用的中断线的芯片。
在这种情况下，你可以真正确定中断不被共享，我们可以把 `uio_pdrv` 的概念再推进一步，使用一个通用的中断处理例程。这就是 `uio_pdrv_genirq` 所做的。

这个驱动的建立与上述 `uio_pdrv` 相同，只是你不实现中断处理例程。`struct uio_info` 的 `.handler` 元素必须保持为 `NULL`。`.irq_flags` 元素不得包含 `IRQF_SHARED`。

你将 `struct platform_device` 的 `.name` 元素设为 `"uio_pdrv_genirq"` 来使用这个驱动。

`uio_pdrv_genirq` 的通用中断处理例程将简单地使用 `disable_irq_nosync()` 禁用中断线。
在完成它的工作之后，用户空间可以通过向 UIO 设备文件写入 0x00000001 来重新启用中断。
这个驱动已经实现了一个 `irq_control()` 来使之成为可能，你绝不能实现你自己的。

使用 `uio_pdrv_genirq` 不仅省去了几行中断处理例程的代码。你也不需要知道芯片内部寄存器的任何信息来创建驱动的内核部分。
你只需要知道芯片所连接到的引脚的 irq 号。

当使用在启用了设备树的系统中时，该驱动需要使用设为该驱动应当处理的节点的 `"compatible"` 字符串的 `"of_id"` 模块参数来进行探测。
默认情况下，节点的名字（不含单元地址）被暴露为用户空间中 UIO 设备的名字。
要设置一个自定义名字，可以在 DT 节点中指定一个名为 `"linux,uio-name"` 的属性。

### 对平台设备使用 uio_dmem_genirq


除了静态分配的内存范围，也可能有在用户空间驱动中使用动态分配区域的愿望。特别是，能够访问通过 dma-mapping API 提供的、可能特别有用的内存。
`uio_dmem_genirq` 驱动提供了一种实现这一点的方式。

就中断配置与处理而言，这个驱动的使用方式与 `"uio_pdrv_genirq"` 驱动类似。

将 `struct platform_device` 的 `.name` 元素设为 `"uio_dmem_genirq"` 来使用这个驱动。

使用这个驱动时，填充 `struct platform_device` 的 `.platform_data` 元素，其类型为 `struct uio_dmem_genirq_pdata`，包含以下元素：

- `struct uio_info uioinfo`：与 `uio_pdrv_genirq` 平台数据使用的相同结构

- `unsigned int *dynamic_region_sizes`：要映射到用户空间的动态内存区域大小列表的指针。

- `unsigned int num_dynamic_regions`：`dynamic_region_sizes` 数组中的元素数量。

平台数据中定义的动态区域将被追加到 ` mem[] ` 数组中的平台设备资源之后，这意味着静态和动态内存区域的总数不能超过 `MAX_UIO_MAPS`。

动态内存区域将在 UIO 设备文件 `/dev/uioX` 被打开时分配。与静态内存资源类似，动态区域的内存区域信息随后通过 sysfs 在 `/sys/class/uio/uioX/maps/mapY/*` 可见。
动态内存区域将在 UIO 设备文件被关闭时释放。当没有进程保持设备文件打开时，返回给用户空间的地址是 ~0。

## 在用户空间编写驱动


一旦你有了一个可用于你的硬件的内核模块，你就可以编写驱动的用户空间部分了。你不需要任何特殊的库，你的驱动可以用任何合理的语言编写，你可以使用浮点数等等。
简而言之，你可以使用你平时编写用户空间应用所用的所有工具和库。

### 获取关于你的 UIO 设备的信息


关于所有 UIO 设备的信息都可在 sysfs 中获得。在你的驱动中你应该做的第一件事是检查 `name` 与 `version`，以确保你正在与正确的设备对话，并且它的内核驱动具有你期望的版本。

你还应该确保你需要的那个内存映射存在，并且具有你期望的大小。

有一个叫做 `lsuio` 的工具，它列出 UIO 设备及其属性。它可在此处获取：

http://www.osadl.org/projects/downloads/UIO/user/

通过 `lsuio`，你可以快速检查你的内核模块是否已加载，以及它导出了哪些属性。详情请看它的 manpage。

`lsuio` 的源代码可以作为获取 UIO 设备信息的一个例子。`uio_helper.c` 文件包含了许多你可以在你的用户空间驱动代码中使用的函数。

### mmap() 设备内存


在你确认你有了带有所需内存映射的正确设备之后，你所要做的全部就是调用 `mmap()` 将设备的内存映射到用户空间。

`mmap()` 调用的 `offset` 参数对 UIO 设备有一个特殊的含义：它用于选择你想要映射的设备的哪个映射。
要映射映射 N 的内存，你必须使用
```
        offset = N * getpagesize();
```
N 从零开始，所以如果你只有一个要映射的内存范围，设置 `offset = 0`。这种技术的一个缺点是，内存总是从它的起始地址开始被映射。

### 等待中断


在你成功映射了你的设备内存之后，你可以像访问一个普通数组一样访问它。通常，你会执行一些初始化。
之后，你的硬件开始工作，并会在一完成、有一些数据可用、或因为发生错误需要你关注时立即产生一个中断。

`/dev/uioX` 是一个只读文件。一个 `read()` 将总是阻塞，直到一个中断发生。
`read()` 的 `count` 参数只有一个合法值，那就是一个带符号 32 位整数（4）的大小。
`count` 的任何其他值都会导致 `read()` 失败。读取的带符号 32 位整数是你的设备的中断计数。
如果这个值比你上次读取的值大一，那么一切都正常。如果差值大于一，你错过了中断。

你也可以在 `/dev/uioX` 上使用 `select()`。

## 通用 PCI UIO 驱动


这个通用驱动是一个名为 uio_pci_generic 的内核模块。它可以与任何兼容 PCI 2.3（约 2002 年）的设备以及任何兼容的 PCI Express 设备协同工作。
使用它，你只需要编写用户空间驱动，不再需要编写硬件特定的内核模块。

### 让驱动识别设备


由于该驱动不声明任何设备 id，它不会被自动加载，也不会自动绑定到任何设备，你必须
```
     modprobe uio_pci_generic
     echo "8086 10f5" > /sys/bus/pci/drivers/uio_pci_generic/new_id
```
如果你的设备已经有一个硬件特定的内核驱动，这个通用驱动仍然不会绑定到它，这种情况下如果你想使用通用驱动（你为什么想这样？）你将不得不手动解绑
```
        echo -n 0000:00:19.0 > /sys/bus/pci/drivers/e1000e/unbind
        echo -n 0000:00:19.0 > /sys/bus/pci/drivers/uio_pci_generic/bind
```
你可以通过查看
```
        ls -l /sys/bus/pci/devices/0000:00:19.0/driver
```
来验证设备是否已绑定到驱动，
```
      .../0000:00:19.0/driver -> ../../../bus/pci/drivers/uio_pci_generic
```
请注意，这个通用驱动不会绑定到旧的 PCI 2.2 设备。如果
```
      dmesg
```
并在输出中查找失败原因。

### 关于 uio_pci_generic 需要知道的事情


中断是通过 PCI 命令寄存器中的 Interrupt Disable 位以及 PCI 状态寄存器中的 Interrupt Status 位来处理的。
所有兼容 PCI 2.3（约 2002 年）的设备以及所有兼容的 PCI Express 设备都应当支持这些位。uio_pci_generic 检测这种支持，并且不会绑定到不支持命令寄存器中 Interrupt Disable 位的设备。

在每次中断时，uio_pci_generic 设置 Interrupt Disable 位。这防止设备产生进一步的中断，直到该位被清除。用户空间驱动应当在阻塞并等待更多中断之前清除这个位。

### 使用 uio_pci_generic 编写用户空间驱动


用户空间驱动可以使用 pci sysfs 接口，或者包装它的 libpci 库，来与设备对话并通过写入命令寄存器重新启用中断。

### 使用 uio_pci_generic 的示例代码


```
    #include <stdlib.h>
    #include <stdio.h>
    #include <unistd.h>
    #include <sys/types.h>
    #include <sys/stat.h>
    #include <fcntl.h>
    #include <errno.h>

    int main()
    {
        int uiofd;
        int configfd;
        int err;
        int i;
        unsigned icount;
        unsigned char command_high;

        uiofd = open("/dev/uio0", O_RDONLY);
        if (uiofd < 0) {
            perror("uio open:");
            return errno;
        }
        configfd = open("/sys/class/uio/uio0/device/config", O_RDWR);
        if (configfd < 0) {
            perror("config open:");
            return errno;
        }

        /* Read and cache command value */
        err = pread(configfd, &command_high, 1, 5);
        if (err != 1) {
            perror("command config read:");
            return errno;
        }
        command_high &= ~0x4;

        for(i = 0;; ++i) {
            /* Print out a message, for debugging. */
            if (i == 0)
                fprintf(stderr, "Started uio test driver.\n");
            else
                fprintf(stderr, "Interrupts: %d\n", icount);

            /****************************************/
            /* Here we got an interrupt from the
               device. Do something to it. */
            /****************************************/

            /* Re-enable interrupts. */
            err = pwrite(configfd, &command_high, 1, 5);
            if (err != 1) {
                perror("config write:");
                break;
            }

            /* Wait for next interrupt. */
            err = read(uiofd, &icount, 4);
            if (err != 4) {
                perror("uio read:");
                break;
            }

        }
        return errno;
    }
```
## 通用 Hyper-V UIO 驱动


这个通用驱动是一个名为 uio_hv_generic 的内核模块。它支持 Hyper-V VMBus 上的设备，类似于 PCI 总线上的 uio_pci_generic。

### 让驱动识别设备


由于该驱动不声明任何设备 GUID，它不会被自动加载，也不会自动绑定到任何设备，你必须自己加载它并为驱动分配 id。例如，要使用
```
     modprobe uio_hv_generic
     echo "f8615163-df3e-46c5-913f-f2d2f965ed0e" > /sys/bus/vmbus/drivers/uio_hv_generic/new_id
```
如果你的设备已经有一个硬件特定的内核驱动，这个通用驱动仍然不会绑定到它，这种情况下如果你想为一个用户空间库使用通用驱动，你将不得不手动解绑硬件特定驱动并绑定通用驱动，使用设备特定的 GUID
```
          echo -n ed963694-e847-4b2a-85af-bc9cfc11d6f3 > /sys/bus/vmbus/drivers/hv_netvsc/unbind
          echo -n ed963694-e847-4b2a-85af-bc9cfc11d6f3 > /sys/bus/vmbus/drivers/uio_hv_generic/bind
```
你可以通过查看
```
        ls -l /sys/bus/vmbus/devices/ed963694-e847-4b2a-85af-bc9cfc11d6f3/driver
```
来验证设备是否已绑定到驱动，
```
      .../ed963694-e847-4b2a-85af-bc9cfc11d6f3/driver -> ../../../bus/vmbus/drivers/uio_hv_generic
```
### 关于 uio_hv_generic 需要知道的事情


在每次中断时，uio_hv_generic 设置 Interrupt Disable 位。这防止设备产生进一步的中断，直到该位被清除。用户空间驱动应当在阻塞并等待更多中断之前清除这个位。

当宿主回收一个设备时，中断文件描述符被标记为关闭，并且对中断文件描述符的任何读取都将返回 -EIO。类似于一个已关闭的套接字或断开连接的串口设备。

vmbus 设备区域被映射到 uio 设备资源中：
    0) 通道环形缓冲区：客户机到宿主以及宿主到客户机
    1) 客户机到宿主的中断信令页
    2) 客户机到宿主的监视页
    3) 网络接收缓冲区区域
    4) 网络发送缓冲区区域

如果由对宿主的请求创建了一个子通道，那么 uio_hv_generic 设备驱动将为每通道环形缓冲区创建一个 sysfs 二进制文件。
```
	/sys/bus/vmbus/devices/3811fe4d-0fa0-4b62-981a-74fc1084c757/channels/21/ring
```
## 更多信息


- `OSADL 主页。 <http://www.osadl.org>`_

- `Linutronix 主页。 <http://www.linutronix.de>`_
