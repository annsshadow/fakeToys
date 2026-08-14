## usbmon


## 简介


小写的名称 “usbmon” 指代内核中的一项设施，用于收集 USB 总线上的 I/O 追踪信息。该功能类似于 tcpdump(1) 或 Ethereal 等网络监控工具所使用的包套接字。类似地，预期会使用一个诸如 usbdump 或 USBMon（大写字母）之类的工具来检查由 usbmon 产生的原始追踪数据。

usbmon 报告的是外设特定驱动向主机控制器驱动（HCD）发出的请求。因此，如果 HCD 存在缺陷，usbmon 报告的追踪可能并不精确对应总线上的事务。这与 tcpdump 的情况相同。

目前实现了两套 API：“text”（文本）与 “binary”（二进制）。二进制 API 通过 /dev 命名空间下的字符设备提供，是一个 ABI。文本 API 自 2.6.35 起已弃用，但出于便利性仍可使用。

## 如何使用 usbmon 收集原始文本追踪


与包套接字不同，usbmon 提供了一个以文本格式提供追踪的接口。这用于两个目的。首先，在更完善的格式最终确定之前，它作为工具之间通用的追踪交换格式。其次，在工具不可用时，人类也可以阅读它。

要收集原始文本追踪，请执行以下步骤。

### 1. 准备


挂载 debugfs（必须在你的内核配置中启用），并加载 usbmon 模块（如果以内核模块方式构建）。第二步在模块内置时会被跳过
```
	# mount -t debugfs none_debugs /sys/kernel/debug
	# modprobe usbmon
	#
```
```
	# ls /sys/kernel/debug/usb/usbmon
	0s  0u  1s  1t  1u  2s  2t  2u  3s  3t  3u  4s  4t  4u
	#
```
现在你可以选择使用套接字 '0u'（捕获所有总线上的包），并跳到步骤 #3；或者使用步骤 #2 找出你的设备所使用的总线。这样可以过滤掉那些持续通信的烦人设备。

### 2. 找出连接目标设备的总线


运行 “cat /sys/kernel/debug/usb/devices”，并找到与该设备对应的 T 行。通常你通过查找厂商字符串来做到这一点。如果你有许多相似的设备，可以拔掉其中一个并对比两次
/sys/kernel/debug/usb/devices 的输出。T 行会带有一个总线号。

```
  T:  Bus=03 Lev=01 Prnt=01 Port=00 Cnt=01 Dev#=  2 Spd=12  MxCh= 0
  D:  Ver= 1.10 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
  P:  Vendor=0557 ProdID=2004 Rev= 1.00
  S:  Manufacturer=ATEN
  S:  Product=UC100KM V2.00
```
“Bus=03” 表示这是总线 3。或者，你也可以查看 “lsusb” 的输出，并从相应行获取总线号。例如：

Bus 003 Device 002: ID 0557:2004 ATEN UC100KM V2.00

### 3. 启动 'cat'


```
	# cat /sys/kernel/debug/usb/usbmon/3u > /tmp/1.mon.out
```
```
	# cat /sys/kernel/debug/usb/usbmon/0u > /tmp/1.mon.out
```
该进程会一直读取，直到被杀死。自然，输出可以重定向到期望的位置。这是推荐做法，因为输出会相当长。

### 4. 在 USB 总线上执行期望的操作


在这里你执行某些会产生流量的操作：插入 U 盘、复制文件、控制摄像头等。

### 5. 杀死 cat


通常这通过键盘中断（Control-C）完成。

此时输出文件（本例中为 /tmp/1.mon.out）可以被保存、通过电子邮件发送，或用文本编辑器查看。在后一种情况下，请确保文件大小对你的常用编辑器来说不是过大。

## 原始文本数据格式


目前支持两种格式：原始的 '1t' 格式与 '1u' 格式。'1t' 格式在 2.6.21 内核中已弃用。'1u' 格式增加了若干字段，例如 ISO 帧描述符、间隔等。它产生的行稍长一些，但在其他方面是 '1t' 格式的完美超集。

如果希望在程序中区分二者，可以查看 “address” 字段（见下文），其中 '1u' 格式会附加一个总线号。如果出现两个冒号，则为 '1t' 格式，否则为 '1u'。

任何文本格式数据都由一串事件组成，例如 URB 提交、URB 回调、提交错误。每个事件都是一行文本，由空白分隔的单词组成。单词的数量或位置可能取决于事件类型，但有一组对于所有类型都通用的单词。

以下是单词的列表，从左到右：

- URB 标签（URB Tag）。用于标识 URB，通常是 URB 结构在内核中的十六进制地址，但也可以是一个序号或任何其他合理的唯一字符串。

- 以微秒为单位的时间戳，一个十进制数字。时间戳的分辨率取决于可用的时钟，因此它可能远差于一微秒（例如，如果实现使用 jiffies）。

- 事件类型（Event Type）。该类型指的是事件的格式，而非 URB 类型。可用类型有：S - 提交（submission），C - 回调（callback），E - 提交错误（submission error）。

- “Address” 字段（原称 “pipe”）。它由四个以冒号分隔的字段组成：URB 类型与方向、总线号、设备地址、端点号。类型与方向由两个字节按如下方式编码：

    == ==   =============================
    Ci Co   控制输入与输出
    Zi Zo   等时输入与输出
    Ii Io   中断输入与输出
    Bi Bo   批量输入与输出
    == ==   =============================

  总线号、设备地址和端点都是十进制数字，但为了便于人类阅读，它们可能带有前导零。

- URB 状态字段（URB Status word）。这是一个字母，或是若干以冒号分隔的数字：URB 状态、间隔、起始帧与错误计数。与 “address” 字段不同，除状态外的所有字段都是可选的。间隔仅对中断和等时 URB 打印。起始帧仅对等时 URB 打印。错误计数仅对等时回调事件打印。

  状态字段是一个十进制数字，有时为负，表示 URB 的 “status” 字段。该字段对提交没有意义，但无论如何都会存在以帮助脚本解析。当发生错误时，该字段包含错误码。

  在提交控制包的情况下，该字段包含的是 Setup 标签（Setup Tag），而非一组数字。很容易判断 Setup 标签是否存在，因为它永远不会是数字。因此，如果脚本在该字段中发现一组数字，它们会继续读取数据长度（等时 URB 除外）。如果发现其他内容，例如字母，它们会在读取数据长度或等时描述符之前先读取 setup 包。

- Setup 包（如果存在）由 5 个单词组成：分别对应 bmRequestType、bRequest、wValue、wIndex、wLength 各一个，如 USB Specification 2.0 所规定。如果 Setup 标签为 's'，这些单词可以安全解码。否则，setup 包曾存在但未被捕获，字段中包含填充值。

- 等时帧描述符的数量以及描述符本身。如果一个等时传输事件带有一组描述符，会先打印一个 URB 中的描述符总数，然后每个描述符一个单词，最多 5 个。单词由 3 个以冒号分隔的十进制数字组成，分别对应状态、偏移与长度。对提交而言，报告的是初始长度。对回调而言，报告的是实际长度。

- 数据长度（Data Length）。对提交而言，这是请求的长度。对回调而言，这是实际长度。

- 数据标签（Data tag）。即使长度非零，usbmon 也可能并不总是捕获数据。仅当该标签为 '=' 时数据单词才存在。

- 其后的数据单词，采用大端十六进制格式。注意它们并非机器字，而只是被拆分成单词的字节流，以便于阅读。因此，最后一个单词可能包含 1 到 4 个字节。收集到的数据长度是受限的，可能小于数据长度字段中报告的长度。在等时输入（Zi）完成、且接收数据在缓冲区中稀疏的情况下，收集到的数据长度可能大于数据长度值（因为数据长度只统计已接收的字节，而数据单词包含整个传输缓冲区）。

示例：

```
  d5ea89a0 3575914555 S Ci:1:001:0 s a3 00 0000 0003 0004 4 <
  d5ea89a0 3575914560 C Ci:1:001:0 0 4 = 01050000
```
一个向发送 SCSI 命令 0x28（READ_10）的 31 字节输出批量传输
```
  dd65f0e8 4128379752 S Bo:1:005:2 -115 31 = 55534243 ad000000 00800000 80010a28 20000000 20000040 00000000 000000
  dd65f0e8 4128379808 C Bo:1:005:2 0 31 >
```
## 原始二进制格式与 API


该 API 的整体架构与上述基本相同，只是事件以二进制格式交付。每个事件在
```
  struct usbmon_packet {
	u64 id;			/*  0: URB ID - from submission to callback */
	unsigned char type;	/*  8: Same as text; extensible. */
	unsigned char xfer_type; /*    ISO (0), Intr, Control, Bulk (3) */
	unsigned char epnum;	/*     Endpoint number and transfer direction */
	unsigned char devnum;	/*     Device address */
	u16 busnum;		/* 12: Bus number */
	char flag_setup;	/* 14: Same as text */
	char flag_data;		/* 15: Same as text; Binary zero is OK. */
	s64 ts_sec;		/* 16: gettimeofday */
	s32 ts_usec;		/* 24: gettimeofday */
	int status;		/* 28: */
	unsigned int length;	/* 32: Length of data (submitted or actual) */
	unsigned int len_cap;	/* 36: Delivered length */
	union {			/* 40: */
		unsigned char setup[SETUP_LEN];	/* Only for Control S-type */
		struct iso_rec {		/* Only for ISO */
			int error_count;
			int numdesc;
		} iso;
	} s;
	int interval;		/* 48: Only for Interrupt and ISO */
	int start_frame;	/* 52: For ISO */
	unsigned int xfer_flags; /* 56: copy of URB's transfer_flags */
	unsigned int ndesc;	/* 60: Actual number of ISO descriptors */
  };				/* 64 total length */
```
这些事件可以通过 read(2) 读取、通过 ioctl(2) 调用，或通过 mmap 访问缓冲区来从字符设备接收。不过出于兼容性原因，read(2) 只返回前 48 字节。

字符设备通常称为 /dev/usbmonN，其中 N 是 USB 总线号。零号（/dev/usbmon0）是特殊的，表示 “所有总线”。注意，具体的命名策略由你的 Linux 发行版决定。

如果你手动创建 /dev/usbmon0，请确保它由 root 拥有且权限模式为 0600。否则，非特权用户将能够窥探键盘流量。

以下是可用的 ioctl 调用，其 MON_IOC_MAGIC 为 0x92：

 MON_IOCQ_URB_LEN, 定义为 _IO(MON_IOC_MAGIC, 1)

该调用返回下一个事件中数据的长度。注意大多数事件不包含数据，因此如果该调用返回零，并不意味着没有事件可用。

 MON_IOCG_STATS, 定义为 _IOR(MON_IOC_MAGIC, 3, struct mon_bin_stats)

```
  struct mon_bin_stats {
	u32 queued;
	u32 dropped;
  };
```
成员 “queued” 指的是当前在缓冲区中排队（queued）的事件数（而非自上次重置以来已处理的事件数）。

成员 “dropped” 是自上次调用 MON_IOCG_STATS 以来丢失的事件数。

 MON_IOCT_RING_SIZE, 定义为 _IO(MON_IOC_MAGIC, 4)

该调用设置缓冲区大小。参数是以字节为单位的大小。该大小可能被向下取整到下一个块（或页）。如果请求的大小超出此内核的 [未指定] 边界，调用将以 -EINVAL 失败。

 MON_IOCQ_RING_SIZE, 定义为 _IO(MON_IOC_MAGIC, 5)

该调用返回缓冲区当前的字节大小。

 MON_IOCX_GET, 定义为 _IOW(MON_IOC_MAGIC, 6, struct mon_get_arg)
 MON_IOCX_GETX, 定义为 _IOW(MON_IOC_MAGIC, 10, struct mon_get_arg)

如果内核缓冲区中没有事件，这些调用会等待事件到达，然后返回第一个事件。参数是指向如下结构的指针
```
  struct mon_get_arg {
	struct usbmon_packet *hdr;
	void *data;
	size_t alloc;		/* Length of data (can be zero) */
  };
```
在调用之前，hdr、data 与 alloc 应被填充。返回时，hdr 所指向的区域包含下一个事件结构，data 缓冲区包含数据（如果有）。该事件会从内核缓冲区中移除。

MON_IOCX_GET 向 hdr 区域复制 48 字节，MON_IOCX_GETX 复制 64 字节。

 MON_IOCX_MFETCH, 定义为 _IOWR(MON_IOC_MAGIC, 7, struct mon_mfetch_arg)

该 ioctl 主要在应用程序通过以下方式访问缓冲区时使用
```
  struct mon_mfetch_arg {
	uint32_t *offvec;	/* Vector of events fetched */
	uint32_t nfetch;	/* Number of events to fetch (out: fetched) */
	uint32_t nflush;	/* Number of events to flush */
  };
```
该 ioctl 分 3 个阶段运行。

首先，它从内核缓冲区中移除并丢弃最多 nflush 个事件。实际丢弃的事件数返回在 nflush 中。

其次，除非伪设备以 O_NONBLOCK 打开，否则它会等待缓冲区中出现一个事件。

第三，它将最多 nfetch 个偏移提取到 mmap 缓冲区中，并存入 offvec。实际的事件偏移数量存入 nfetch。

 MON_IOCH_MFLUSH, 定义为 _IO(MON_IOC_MAGIC, 8)

该调用从内核缓冲区中移除一定数量的事件。其参数是要移除的事件数。如果缓冲区中已有的事件少于请求数量，则移除所有存在的事件，且不报告错误。在没有事件可用时它同样有效。

 FIONBIO

如果有需要，未来可能会实现 ioctl FIONBIO。

除了 ioctl(2) 和 read(2)，二进制 API 的特殊文件还可以用 select(2) 和 poll(2) 进行轮询。但 lseek(2) 无法工作。

- 二进制 API 内核缓冲区的内存映射访问

基本思路很简单：

先获取当前大小，然后使用 mmap(2) 映射缓冲区以做准备。
```
   struct mon_mfetch_arg fetch;
   struct usbmon_packet *hdr;
   int nflush = 0;
   for (;;) {
      fetch.offvec = vec; // Has N 32-bit words
      fetch.nfetch = N;   // Or less than N
      fetch.nflush = nflush;
      ioctl(fd, MON_IOCX_MFETCH, &fetch);   // Process errors, too
      nflush = fetch.nfetch;       // This many packets to flush when done
      for (i = 0; i < nflush; i++) {
         hdr = (struct ubsmon_packet *) &mmap_area[vec[i]];
         if (hdr->type == '@')     // Filler packet
            continue;
         caddr_t data = &mmap_area[vec[i]] + 64;
         process_packet(hdr, data);
      }
   }
```
因此，主要思想是每 N 个事件仅执行一次 ioctl。

尽管缓冲区是环形的，返回的头部和数据不会跨越缓冲区末尾，因此上面的伪代码不需要任何聚集操作。
