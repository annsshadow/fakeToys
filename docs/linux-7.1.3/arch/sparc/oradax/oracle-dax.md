## Oracle 数据分析加速器 (DAX)


DAX 是一个协处理器，位于 SPARC M7（DAX1）和 M8（DAX2）处理器芯片上，能够直接访问 CPU L3 缓存以及物理内存。它可以对具有各种输入和输出格式的数据流执行多种操作。驱动程序提供一种传输机制，对各种操作码和数据格式的了解有限。用户空间库提供高层服务，并将其转换为底层命令，随后传递给驱动程序，进而传Hypervisor 和协处理器。应用程序推荐使用此库来使用协处理器，驱动程序接口并不面向一般用途。本文档描述驱动程序的总体流程、其结构以及编程接口。同时提供了足够的示例代码，用于编写使用 DAX 功能的用户或内核应用程序

用户库是开源的，可从以下地址获取

    https://oss.oracle.com/git/gitweb.cgi?p=libdax.git

Hypervisor 与协处理器的接口在配套文dax-hv-api.txt 中有详细描述，该文档是（Oracle 内部UltraSPARC 虚拟机规 3.0.20+15 版的纯文本摘录，日期2017-09-25


## 高层概览


协处理器请求由命令控制块（CCB）描述。CCB 包含一个操作码和若干参数。操作码指定要执行的操作，参数指定选项、标志、大小和地址。CCB（或一CCB）被传递给 Hypervisor，由它负责将请求排队并调度给可用的协处理器执行单元。返回的状态码指示请求是否成功提交，或者是否发生错误。每CCB 中给出的一个地址是指完成的指针，完成区是一128 字节的内存，由协处理器写入以提供执行状态。完成时不会生成中断；必须由软件轮询完成区以获知事务何时结束，但 M7 及后续处理器提供了一种机制，可暂停虚拟处理器直至协处理器更新完完成状态。这通过被监控加载（monitored load）和 mwait 指令实现，后文将详细说明。DAX 协处理器的设计使得在请求提交后，内核不再参与其处理过程。轮询在用户层完成，从而请求完成与请求线程恢复执行之间的延迟几乎为零


## 内存寻址


Sun4v 架构中，内核无法直接访问物理内存，因为存在额外一级的内存虚拟化。这一中间层级称为"内存（real memory），内核将其当作物理内存对待。Hypervisor 负责实内存与物理内存之间的转换，使得每个逻辑域（LDOM）都能拥有与其他 LDOM 隔离的物理内存分区。当内核建立虚拟映射时，它指定一个虚拟地址以及要映射到的实地址

DAX 协处理器只能操作物理内存，因此在将请求送入协处理器之前，CCB 中的所有地址都必须转换为物理地址。内核无法执行此转换，因为它看不到物理地址。因此一CCB 可以包含缓冲区的虚拟或实地址，或是二者的组合。CCB 中可能给出的每个地址都有一地址类型"字段。在所有情况下，Hypervisor 都会在分派到硬件之前将所有地址转换为物理地址。地址转换使用发起请求的进程的上下文执行


## 驱动程序 API


应用程序通过 write() 系统调用向驱动程序发出请求，并通过 read() 获取结果（如果有）。完成区通过 mmap() 访问，对应用程序是只读的

请求可以是一条立即命令，或是要提交给硬件的一CCB

设备的每个打开实例都专属于打开它的线程，并且该线程必须将其用于所有后续操作。驱动程序的 open 函数会为线程创建一个新的上下文并进行初始化以供使用。该上下文包含驱动程序内部用于跟踪已提交请求的指针和数值。完成区缓冲区也会被分配，其大小足以容纳多个并发请求的完成区。设备关闭时，所有未完成的事务都会被刷新，上下文被清理

DAX1 系统（M7）上，设备名"oradax1"，而在 DAX2 系统（M8）上名为 "oradax2"。如果应用程序需要其中之一，只需尝试打开相应的设备即可。任意给定系统上只会存在其中一个设备，因此可用名称判断平台所支持的是什么

立即命令包括 CCB_DEQUEUE、CCB_KILL CCB_INFO。所有这些命令成功与否，write() 返回值等于调用中给定的字节数来指示。否则返-1 并设errno

### CCB_DEQUEUE


告知驱动程序清理与过往请求相关的资源。由于请求完成时不会生成中断，必须通知驱动程序何时可以回收资源。不会返回进一步的状态信息，因此用户不应随后调用 read()

### CCB_KILL


在执行过程中终止一CCB。一旦此调用成功返回，即可保证该 CCB 不会再继续执行。成功时，必须调read() 以取回该操作的结果

### CCB_INFO


取回关于当前正在执行CCB 的信息。注意，某些 Hypervisor CCB 处于 'inprogress' 状态时可能返回 'notfound'。为确保处于 'notfound' 状态的 CCB 永远不会被执行，必须对该 CCB 调用 CCB_KILL。成功后，必须调read() 以取回该操作的详细信息

### 提交一CCB 以供执行


长度正好CCB 大小倍数write() 被视为一次提交操作。文件偏移被视为要使用的完成区索引，可通过 lseek() pwrite() 系统调用设置。如果返-1，则 errno 被设置以指示错误。否则，返回值是协处理器实际接受的数组长度。如果接受的长度等于请求的长度，则提交完全成功，无需进一步状态；因此用户不应随后调用 read()。对 CCB 数组的部分接受以返回值小于请求长度来指示，必须调read() 以获取进一步的状态信息。状态将反映第一个未被接受的 CCB 所导致的错误，某些情况status_data 会提供额外数据

### MMAP


mmap() 函数提供对驱动程序中分配的完成区的访问。注意用户进程不可写入完成区，且 mmap 调用不得指定 PROT_WRITE


## 请求完成


每个完成区的第一个字节是命令状态，由协处理器硬件更新。软件可利用新的 M7/M8 处理器能力来高效地轮询该状态字节。首先，通过 Load from Alternate Space（ldxa、lduba 等）带上 ASI 0x84（ASI_MONITOR_PRIMARY）实被监控加。其次，通过 mwait 指令（对 %asr28 的写操作）实被监控等。该指令类似pause，可让虚拟处理器在给定的纳秒数内暂停执行，但此外还会在某些事件发生时提前终止。如果包含被监控位置的数据块被修改，mwait 终止。这会使软件在事务完成后立即恢复执行（无需上下文切换或从内核到用户的切换）。因此事务完成与恢复执行之间的延迟可能只有几纳秒


## DAX 提交的应用程序生命周


 - 打开 dax 设备
 - 调用 mmap() 获取完成区地址
 - 分配一CCB 并填入操作码、标志、参数、地址
 - 通过 write() pwrite() 提交 CCB
 - 进入循环执行被监控加+ 被监控等待，并在命令状态指示请求完成时终止
   （CCB_KILL CCB_INFO 可随时按需使用
 - 执行一CCB_DEQUEUE
 - 对完成区调用 munmap()
 - 关闭 dax 设备


## 内存约束


DAX 硬件只操作物理地址。因此它不知道虚拟内存映射以及虚拟缓冲区所映射的物理内存中可能存在的不连续性。没I/O TLB 或任何分聚集机制。所有缓冲区，无论是输入还是输出，都必须位于物理连续的内存区域内

Hypervisor 在将 CCB 交予 DAX 之前，会CCB 内的所有地址转换为物理地址。Hypervisor 会确定所给每个虚拟地址的虚拟页大小，并据此为每个地址设定一个大小上限。这可防止协处理器越过虚拟页的边界进行读或写，即便它直接访问的是物理内存。简而言之，DAX 操作永远不会"跨越"虚拟页边界。如果使8k 虚拟页，则数据被严格限制8k。如果用户的缓冲区大8k，则必须使用更大的页大小，否则事务大小会被截断到 8k

大页。用户可以使用标准接口分配大页。位于大页上的内存缓冲区可用于实现大得多DAX 事务尺寸，但仍须遵守这些规则，且任何事务都不会跨越页边界，即便大页也是一样。一个主要注意点是，Sparc 上的 Linux 8Mb 作为大页尺寸之一。Sparc 实际上并不提8Mb 的硬件页大小，这一尺寸是通过把两4Mb 页拼接在一起合成的。这出于历史原因，并且会造成一个问题：在任意给定的 DAX 请求中，8Mb 页中实际只有一半可用于某个缓冲区，且必须是前一半或后一半；不能是中间的一4Mb 块，因为那会越过（硬件）页边界。注意，这个整体问题可能被高层库所隐藏


### CCB 结构


一CCB 是由 8 64 位字组成的数组。其中若干字提供命令操作码、参数、标志等，其余的是地址
```

   struct ccb {
       u64   control;
       u64   completion;
       u64   input0;
       u64   access;
       u64   input1;
       u64   op_data;
       u64   output;
       u64   table;
   };

```
关于每个字段的详细说明，请参libdax/common/sys/dax1/dax1_ccb.h，关于可供客户机操作系统（即 Linux 内核）使用的 Hypervisor API 的完整描述，请参dax-hv-api.txt

第一个字（control）会被驱动程序检查以下内容：
 - CCB 版本，必须与硬件版本一
 - 操作码，必须是文档允许的命令之一
 - 地址类型，对用户提供所有地址必须设为 "virtual"，从而确保应用程序只能访问它拥有的内


## 示例代码


DAX 对用户代码和内核代码都可访问。内核代码可以直接发起超调用（hypercall），而用户代码必须使用驱动程序提供的封装函数。CCB 的设置对二者几乎相同；唯一的区别在于完成区的准备。下面先给出用户代码示例，之后是内核代码

要使用驱动程API 编程，必须包含文arch/sparc/include/uapi/asm/oradax.h

首先，必须打开正确的设备。对M7 /dev/oradax1，对M8 /dev/oradax2。最简单的
```

	fd = open("/dev/oradax1", O_RDWR);
	if (fd < 0)
		fd = open("/dev/oradax2", O_RDWR);
	if (fd < 0)
	       /* 未找DAX */

```
```

      completion_area = mmap(NULL, DAX_MMAP_LEN, PROT_READ, MAP_SHARED, fd, 0);

```
所有输入和输出缓冲区必须完整地包含在一个硬件页内，因为如前所述，DAX 严格受虚拟页边界约束。此外，输出缓冲区必须按 64 字节对齐，且大小必须64 字节的倍数，因为协处理器以缓存行为单位写入

本例演示 DAX Scan 命令，它以一个向量和一个匹配值作为输入，并产生一个位图作为输出。每个与匹配值相同的输入元素会在输出中对应位置置位

在本例中，输入向量由一连串单比特组成，匹配值为 0。因此输入中的每0 比特会在输出中产生一1，反之亦然，从而得到的输出位图就是输入位图的反转

关于CCB 中使用的所有参数和位的细节，请参阅 DAX Hypervisor API 文档的第 36.2.1.3 节，
```

	ccb->control =       /* Table 36.1, CCB Header Format */
		  (2L << 48)     /* command = Scan Value */
		| (3L << 40)     /* output address type = primary virtual */
		| (3L << 34)     /* primary input address type = primary virtual */
		             /* Section 36.2.1, Query CCB Command Formats */
		| (1 << 28)     /* 36.2.1.1.1 primary input format = fixed width bit packed */
		| (0 << 23)     /* 36.2.1.1.2 primary input element size = 0 (1 bit) */
		| (8 << 10)     /* 36.2.1.1.6 output format = bit vector */
		| (0 <<  5)	/* 36.2.1.3 First scan criteria size = 0 (1 byte) */
		| (31 << 0);	/* 36.2.1.3 Disable second scan criteria */

	ccb->completion = 0;    /* Completion area address, to be filled in by driver */

	ccb->input0 = (unsigned long) input; /* primary input address */

	ccb->access =       /* Section 36.2.1.2, Data Access Control */
		  (2 << 24)    /* Primary input length format = bits */
		| (nbits - 1); /* number of bits in primary input stream, minus 1 */

	ccb->input1 = 0;       /* secondary input address, unused */

	ccb->op_data = 0;      /* scan criteria (value to be matched) */

	ccb->output = (unsigned long) output;	/* output address */

	ccb->table = 0;	       /* table address, unused */

```
CCB 的提交是对驱动程序的一write() pwrite() 系统调用。如果调用失败，则必须使read() 来取
```

	if (pwrite(fd, ccb, 64, 0) != 64) {
		struct ccb_exec_result status;
		read(fd, &status, sizeof(status));
		/* 退*/
	}

```
CCB 成功提交后，可轮询完成区以确DAX 何时结束。关于完成区内容的详细信息，可在36.2.2 节找
```

	while (1) {
		/* Monitored Load */
		__asm__ __volatile__("lduba [%1] 0x84, %0\n"
				     : "=r" (status)
				     : "r"  (completion_area));

		if (status)	     /* 0 表示命令执行*/
			break;

		/* MWAIT */
		__asm__ __volatile__("wr %%g0, 1000, %%asr28\n" ::);    /* 1000 ns */
	}

```
完成区状态为 1 表示 CCB 成功完成且输出位图有效，可立即使用。所有其他非零值表示错误条件，
```

	if (completion_area[0] != 1) {	/* section 36.2.2, 1 = command ran and succeeded */
		/* completion_area[0] 包含完成状*/
		/* completion_area[1] 包含错误码，36.2.2 */
	}

```
完成区处理完毕后，必须通知驱动程序它可以释放与
```

	struct dax_command cmd;
	cmd.command = CCB_DEQUEUE;
	if (write(fd, &cmd, sizeof(cmd)) != sizeof(cmd)) {
		/* 退*/
	}

```
最后，应进行正常的程序清理，即解除完成区映射、关dax 设备、释放内存等

### 内核示例


在内核代码中使用 DAX 的唯一区别在于完成区的处理。与通过 mmap 驱动程序分配的完成区的用户应用程序不同，内核代码必须分配自己的内存用作完成区，其地址以及
```

	ccb->control |=      /* Table 36.1, CCB Header Format */
	        (3L << 32);     /* completion area address type = primary virtual */

	ccb->completion = (unsigned long) completion_area;   /* Completion area address */

```
dax 提交超调用是直接发起的。ccb_submit 调用中使用的标志记录DAX HV API 的第 36.3.1 节中

```

  #include <asm/hypervisor.h>

	hv_rv = sun4v_ccb_submit((unsigned long)ccb, 64,
				 HV_CCB_QUERY_CMD |
				 HV_CCB_ARG0_PRIVILEGED | HV_CCB_ARG0_TYPE_PRIMARY |
				 HV_CCB_VA_PRIVILEGED,
				 0, &bytes_accepted, &status_data);

	if (hv_rv != HV_EOK) {
		/* hv_rv 是错误码，status_data 包含 */
		/* 可能的额外状态，36.3.1.1 */
	}

```
提交之后，完成区轮询代码
```

	while (1) {
		/* Monitored Load */
		__asm__ __volatile__("lduba [%1] 0x84, %0\n"
				     : "=r" (status)
				     : "r"  (completion_area));

		if (status)	     /* 0 表示命令执行*/
			break;

		/* MWAIT */
		__asm__ __volatile__("wr %%g0, 1000, %%asr28\n" ::);    /* 1000 ns */
	}

	if (completion_area[0] != 1) {	/* section 36.2.2, 1 = command ran and succeeded */
		/* completion_area[0] 包含完成状*/
		/* completion_area[1] 包含错误码，36.2.2 */
	}

```
一旦完成状态指示成功，输出位图即可立即使用

## UltraSPARC 虚拟机规范摘


 .. include:: dax-hv-api.txt
    :literal:
