## Linux CD-ROM 标准


:作? David van Leeuwen <david@ElseWare.cistron.nl>
:日期: 1999 ?3 ?12 ?
:更新? Erik Andersen (andersee@debian.org)
:更新? Jens Axboe (axboe@image.dk)


## 引言


Linux 大概是支持硬件设备种类最广泛的类 Unix 操作系统。其原因大概是：

- Linux 目前支持的众多平台（?i386-PC、Sparc Sun 等）上有大量可用的硬件设备。
- 操作系统采用开放设计，任何人都可以?Linux 编写驱动。
- 有大量源代码可作为如何编写驱动的示例。

Linux 的开放性，以及种类繁多的可用硬件，?Linux 得以支持许多不同的硬件设备。遗憾的是，正是这种允许 Linux 支持所有这些不同设备的开放性，也导致了每个设备驱动的行为彼此之间存在显著差异。这种行为的差异?CD-ROM 设备上表现得尤为明显；某个特定光驱对一?`standard` **ioctl()** 调用的反应，在不同设备驱动之间差异巨大。为了避免让自己的驱动完全不一致，Linux CD-ROM 驱动的编写者通常会通过理解、复制、再修改一个已有驱动来创建新的设备驱动。遗憾的是，这种做法并没有在所?Linux CD-ROM 驱动之间维持统一的行为。

本文档描述了?Linux 所有不同的 CD-ROM 设备驱动建立统一行为的努力。本文档还定义了各种 **ioctl()**，以及底?CD-ROM 设备驱动应当如何实现它们。当前（?Linux 2.1.\ **x** 开发内核中）已有若干底?CD-ROM 设备驱动（包?IDE/ATAPI ?SCSI）开始使用这种统一接口。

?CD-ROM 被开发出来时，CD-ROM 驱动器与计算机之间的接口并未在标准中规定。结果便是出现了许多不同?CD-ROM 接口。其中一些拥有自己的专有设计（Sony、Mitsumi、Panasonic、Philips），其他制造商则采用了已有的电气接口并改变了功能（CreativeLabs/SoundBlaster、Teac、Funai），或者干脆让自家的驱动器适配一种或多种已有的电气接口（Aztech、Sanyo、Funai、Vertos、Longshine、Optics Storage 以及大多?`NoName` 制造商）。在某种新驱动器确实带来了自己的接口、或使用了自己的命令集和流控方案的情况下，要么必须编写一个独立的驱动，要么必须增强一个已有的驱动。历史已经为我们提供了针对许多这类不同接口的 CD-ROM 支持。如今，几乎所有新出的 CD-ROM 驱动器都?IDE/ATAPI ?SCSI，制造商再创建新接口的可能性微乎其微。甚至连为旧的专有接口寻找驱动器都变得困难了。

当（?1.3.70 时代）我查看通过 `cdrom.h` 表达的现有软件接口时，它看起来是一组相当杂乱的命令和数据格?[#f1]_。似乎软件接口的许多特性都是以一?**ad hoc**（临时应付）的方式被添加进来，以迁就某个特定驱动器的能力。更重要的是，大多数不同驱动?`standard` 命令的行为似乎各不相同：例如，有些驱动在托盘打开时若发生一?**open()** 调用就会合上托盘，而另一些则不会。有些驱动在打开设备时会锁上门，以防止文件系统不一致，而另一些则不会，以便允许软件弹出。毫无疑问，不同驱动器的能力存在差异，但即便是两个拥有相同能力的驱动器，其驱动的行为通常也不同。

   我记不清当时看的是哪个内核版本了，大概是 1.2.13 ?1.3.34 —?我间接参与过的最后一个内核。

我决定就如何让所?Linux CD-ROM 驱动的行为更加统一展开一次讨论。我首先联系?Linux 内核中众?CD-ROM 驱动的开发者。他们的反应鼓舞了我去编写本文档旨在描述的统一 CD-ROM 驱动（Uniform CD-ROM Driver）。统一 CD-ROM 驱动的实现位于文?`cdrom.c` 中。该驱动意在成为位于每个 CD-ROM 驱动器底层设备驱动之上的一层附加软件层。通过增加这一层，便有可能让所有不同的 CD-ROM 设备表现?**完全** 一致（在底层硬件允许的范围内）。

统一 CD-ROM 驱动的目?**并非** 疏远那些尚未采取措施支持该努力的驱动开发者。统一 CD-ROM 驱动的目标仅仅是，为编写面向 CD-ROM 驱动器的应用程序的人提供 **一?* 对全?CD-ROM 设备行为一致的 Linux CD-ROM 接口。此外，这也为底层设备驱动代码与 Linux 内核之间提供了一致的接口。我们确保与 `cdrom.h` 中定义的数据结构和程序员接口保持 100% 兼容。本指南的编写是为了帮助 CD-ROM 驱动开发者调整他们的代码以使?`cdrom.c` 中定义的统一 CD-ROM 驱动代码。

就个人而言，我认为最重要的硬件接口是 IDE/ATAPI 驱动器，当然还有 SCSI 驱动器，但随着硬件价格持续下降，人们也可能拥有多台 CD-ROM 驱动器、甚至可能类型混杂，这同样很可能发生。重要的是，这些驱动器的行为应当一致?994 ?12 月，最便宜?CD-ROM 驱动器之一?Philips cm206，一台双倍速专有驱动器。在我忙于为它编?Linux 驱动的那些月份里，专有驱动器变得过时，?IDE/ATAPI 驱动器成了标准。在本文档最后一次更新时?997 ?11 月），要找到低于 16 倍速的 CD-ROM 驱动器甚至都变得困难，?24 倍速驱动器已经很常见了。


## 通过另一软件层实现标准化


在构思本文档之时，所有驱动都直接通过各自的例程实?CD-ROM ?**ioctl()** 调用。这导致了一种风险：不同驱动可能会忘记做诸如检查用户是否向驱动提供了有效数据这类重要事情。更重要的是，这导致了前面已经讨论过的行为分歧。

出于这一原因，创建了统一 CD-ROM 驱动，以强制实现一致的 CD-ROM 驱动器行为，并为各种底层 CD-ROM 设备驱动提供一组通用服务。统一 CD-ROM 驱动现在提供了另一软件层，它把 **ioctl()** ?**open()** 的实现与实际的硬件实现分离开来。请注意，这一努力极少改动会影响用户应用程序的地方。最大的改动是把各种底层 CD-ROM 驱动的头文件内容移到了内核的 cdrom 目录中。这样做是为了帮助用户只面对唯一?cdrom 接口，即 `cdrom.h` 中定义的接口。

CD-ROM 驱动器的特性足够特殊（即不同于软盘或硬盘等其他块设备），因此可以定义一组通用?**CD-ROM 设备操作**，即 **<cdrom-device>_dops**。这些操作不同于经典的块设备文件操作 **<block-device>_fops**?

统一 CD-ROM 驱动接口层的例程实现在文?`cdrom.c` 中。在该文件中，统一 CD-ROM 驱动通过注册以下通用内容，以内核块设备的方式与内核交互：

```
	struct file_operations cdrom_fops = {
		NULL,			/* lseek */
		block _read ,		/* read--general block-dev read */
		block _write,		/* write--general block-dev write */
		NULL,			/* readdir */
		NULL,			/* select */
		cdrom_ioctl,		/* ioctl */
		NULL,			/* mmap */
		cdrom_open,		/* open */
		cdrom_release,		/* release */
		NULL,			/* fsync */
		NULL,			/* fasync */
		NULL			/* revalidate */
	};

```
每个活跃?CD-ROM 设备都共享这一 **struct**。上面声明的例程全部实现?`cdrom.c` 中，因为该文件正是定义和标准化所?CD-ROM 设备行为的地方。对各种类型 CD-ROM 硬件的实际接口仍然由各种底层 CD-ROM 设备驱动执行。这些例程只是实现了所?CD-ROM（事实上，所有可移动介质设备）通用的某?**capabilities**?

底层 CD-ROM 设备驱动的注册现在通过 `cdrom.c` 中的通用例程完成，而不再经由虚拟文件系统（VFS）。`cdrom.c` 中实现的接口通过两个通用结构来执行，这两个结构包含了关于驱动能力、以及驱动所操作的特定驱动器的信息。这两个结构是：

cdrom_device_ops
  该结构包含关于某?CD-ROM 设备的底层驱动的信息。该结构在概念上连接到设备的主设备号（尽管某些驱动可能拥有不同的主设备号，IDE 驱动便是如此）。

cdrom_device_info
  该结构包含关于某个特?CD-ROM 驱动器的信息，例如它的设备名、速度等。该结构在概念上连接到设备的次设备号。

用统一 CD-ROM 驱动注册某个特定?CD-ROM 驱动器：

```
	register_cdrom(struct cdrom_device_info * <device>_info)

```
设备信息结构 **<device>_info** 包含了内核与底层 CD-ROM 设备驱动交互所需的全部信息。该结构中最重要的条目之一，是指向底层驱动?**cdrom_device_ops** 结构的指针。

设备操作结构 **cdrom_device_ops** 包含一组指向底层设备驱动中所实现函数的指针。当 `cdrom.c` 访问一?CD-ROM 设备时，它通过该结构中的函数来进行。未来的 CD-ROM 驱动器的全部能力无法预知，因此预计随着新技术被开发出来，这一列表可能需要不时扩展。例如，CD-R ?CD-R/W 驱动器正开始变得流行，很快就需要为它们添加支持。目前：

```
	struct cdrom_device_ops {
		int (*open)(struct cdrom_device_info *, int)
		void (*release)(struct cdrom_device_info *);
		int (*drive_status)(struct cdrom_device_info *, int);
		unsigned int (*check_events)(struct cdrom_device_info *,
					     unsigned int, int);
		int (*media_changed)(struct cdrom_device_info *, int);
		int (*tray_move)(struct cdrom_device_info *, int);
		int (*lock_door)(struct cdrom_device_info *, int);
		int (*select_speed)(struct cdrom_device_info *, unsigned long);
		int (*get_last_session) (struct cdrom_device_info *,
					 struct cdrom_multisession *);
		int (*get_mcn)(struct cdrom_device_info *, struct cdrom_mcn *);
		int (*reset)(struct cdrom_device_info *);
		int (*audio_ioctl)(struct cdrom_device_info *,
				   unsigned int, void *);
		const int capability;		/* capability flags */
		int (*generic_packet)(struct cdrom_device_info *,
				      struct packet_command *);
	};

```
当底层设备驱动实现了这些能力中的某一个时，它应当在该 **struct** 中加入一个函数指针。而当某个特定函数未被实现时，?**struct** 中应包含 NULL。在把一?CD-ROM 驱动器向统一 CD-ROM 驱动注册时，**capability** 标志指明?CD-ROM 硬件，或底?CD-ROM 驱动的能力。

请注意，大多数函数的参数都比它们?**blkdev_fops** 对应项要少。这是因?**inode** ?**file** 结构中的信息很少被用到。对大多数驱动而言，主要参数是 **struct** **cdrom_device_info**，从中可以提取出主设备号和次设备号。（不过大多数底?CD-ROM 驱动甚至不看主设备号和次设备号，因为它们中许多只支持一个设备。）这一点将通过下面描述?**cdrom_device_info** 中的 **dev** 可用。

与驱动器相关的、类似次设备号的信息在注册时通过以下结构。

```
  struct cdrom_device_info {
	const struct cdrom_device_ops * ops;	/* device operations for this major */
	struct list_head list;			/* linked list of all device_info */
	struct gendisk * disk;			/* matching block layer disk */
	void *  handle;				/* driver-dependent data */

	int mask;				/* mask of capability: disables them */
	int speed;				/* maximum speed for reading data */
	int capacity;				/* number of discs in a jukebox */

	unsigned int options:30;		/* options flags */
	unsigned mc_flags:2;			/*  media-change buffer flags */
	unsigned int vfs_events;		/*  cached events for vfs path */
	unsigned int ioctl_events;		/*  cached events for ioctl path */
	int use_count;				/*  number of times device is opened */
	char name[20];				/*  name of the device type */

	__u8 sanyo_slot : 2;			/*  Sanyo 3-CD changer support */
	__u8 keeplocked : 1;			/*  CDROM_LOCKDOOR status */
	__u8 reserved : 5;			/*  not used yet */
	int cdda_method;			/*  see CDDA_* flags */
	__u8 last_sense;			/*  saves last sense key */
	__u8 media_written;			/*  dirty flag, DVD+RW bookkeeping */
	unsigned short mmc3_profile;		/*  current MMC3 profile */
	int for_data;				/*  unknown:TBD */
	int mrw_mode_page;			/*  which MRW mode page is in use */
  };

```
使用这一 **struct**，借助 **next** 字段，构建出一个已注册次设备号的链表。设备号、设备操作结构以及驱动器属性的规格都存储在该结构中。

**mask** 标志可用于屏蔽掉 **ops->capability** 中列出的某些能力，如果某个特定驱动器不支持驱动的某项特性。数?**speed** 指明了驱动器的最大磁头速率，以正常音频速度为单位（176kB/sec 原始数据?150kB/sec 文件系统数据）。这些参数被声明?**const**，因为它们描述驱动器的属性，在注册之后不会改变。

少数寄存器包含专属于 CD-ROM 驱动器的变量?*options** 标志用于指定通用 CD-ROM 例程应当如何表现。这些不同的标志寄存器应当提供足够的灵活性，以适应不同用户的意愿（?**不是** 像旧方案那样迁就底层设备驱动作者的 `arbitrary`（任意）意愿）。寄存器 **mc_flags** 用于把来?**media_changed()** 的信息缓冲到两个独立的队列。其他专属于某个次设备的、特定的数据，可以通过 **handle** 访问?*handle** 可以指向一个底层驱动特有的数据结构。字?**use_count**?*next**?*options** ?**mc_flags** 无需初始化。

`cdrom.c` 构成的中间软件层将执行一些额外的簿记工作。设备的使用计数（打开了该设备的进程数）登记在 **use_count** 中。函?**cdrom_ioctl()** 将验证供读写的适当用户内存区域，并且在需要传?CD 上某个位置的数据时，它会通过以标准格式向底层驱动发出请求?`sanitize`（规整）格式，并在用户软件与底层驱动之间翻译所有格式。这免去了驱动大量的内存检查、格式检查和翻译工作。同时，所需的结构将在程序栈上声明。

函数的实现应如后续各节所定义。有两个函数 **必须** 实现，即 **open()** ?**release()**。其他函数可以忽略，它们对应的能力标志会在注册时被清除。通常，函数成功时返回零，出错时返回负值。函数调用应当只在命令完成之后才返回，但当然，等待设备时不应占用处理器时间。

```
	int open(struct cdrom_device_info *cdi, int purpose)

```
**Open()** 应当尝试为特定的 **purpose**（目的）打开设备，该目的可以是：

- 为读取数据而打开，如 `mount()` (2) 或用户命?`dd`、`cat` 所做。
- 为执?**ioctl** 命令而打开，如播放音频 CD 的程序所做。

注意，任何策略性代码（?**open()** 时合上托盘等）都?`cdrom.c` 中的调用例程完成，因此底层例程只需关注适当的初始化，例如让盘片转起来等。

```
	void release(struct cdrom_device_info *cdi)

```
应当执行设备相关的动作，例如让设备减速停止。不过，策略性动作如弹出托盘或解锁舱门，应留给通用例程 **cdrom_release()** 处理。这是唯一一个返回类型为 **void** 的函数。


```
	int drive_status(struct cdrom_device_info *cdi, int slot_nr)

```
如果实现了函?**drive_status**，它应当提供关于驱动器状态（而不是盘片的状态，盘片可能在也可能不在驱动器中）的信息。如果驱动器不是换片器（changer）：


	CDS_NO_INFO		/* no information available */
	CDS_NO_DISC		/* no disc is inserted, tray is closed */
	CDS_TRAY_OPEN		/* tray is opened */
	CDS_DRIVE_NOT_READY	/* something is wrong, tray is moving? */
	CDS_DISC_OK		/* a disc is loaded and everything is fine */

```
```
	int tray_move(struct cdrom_device_info *cdi, int position)

```
如果实现了该函数，它应当控制托盘的运动。（没有其他函数应当控制这个。）参数 **position** 控制期望的运动方向：

- 0 合上托盘
- 1 打开托盘

该函数在成功时返?0，出错时返回非零值。注意，如果托盘已经处于期望的位置，则无需采取任何动作，返回值应?0?

```
	int lock_door(struct cdrom_device_info *cdi, int lock)

```
如果驱动器允许，该函数（且没有其他代码）控制舱门的锁定。数?**lock** 控制期望的锁定状态：

- 0 解锁舱门，允许手动打开
- 1 锁定舱门，托盘无法被手动弹出

该函数在成功时返?0，出错时返回非零值。注意，如果舱门已经处于所请求的状态，则无需采取任何动作，返回值应?0?

```
	int select_speed(struct cdrom_device_info *cdi, unsigned long speed)

```
某些 CD-ROM 驱动器能够改变其磁头速度。改?CD-ROM 驱动器速度有若干原因。压制得不好?CD-ROM 可能会从低于最大的磁头速率中受益。现?CD-ROM 驱动器可以获得非常高的磁头速率（最高达?**24x** 是很常见的）。有报告称这些驱动器在如此高速下可能产生读取错误，降低速度可以在这种情况下防止数据丢失。最后，这些驱动器中的某些会发出恼人的巨大噪声，降低速度可能会减弱这种噪声?

该函数指定读取数据或播放音频时的速度。数?**speed** 以标?cdrom 速度为单位（176kB/sec 原始数据?150kB/sec 文件系统数据）指明驱动器的磁头速度。因此，若要请求 CD-ROM 驱动器以 300kB/sec 运行，你将使?**speed=2** 调用 CDROM_SELECT_SPEED **ioctl**。特殊?`0` 表示 `auto-selection`（自动选择），即最大数据速率或实时音频速率。如果驱动器没有这种 `auto-selection` 能力，则应当根据当前装入的盘片做出决定，并且返回值应为正。负的返回值表示出错?

```
	int get_last_session(struct cdrom_device_info *cdi,
			     struct cdrom_multisession *ms_info)

```
该函数应当实现旧的对?**ioctl()**。对于设?**cdi->dev**，当前盘片最后一个会话的起点应当通过指针参数 **ms_info** 返回。注意，`cdrom.c` 中的例程已经对该参数做了规整：无论调用软件请求何种格式，其请求的格式?**始终** ?**CDROM_LBA** 类型（线性块寻址模式）。但规整还更进一步：`cdrom.c` 中的例程如果必要会做转换，而底层实现如果愿意，可以?**CDROM_MSF** 格式返回所请求的信息（当然要适当地设?**ms_info->addr_format** 字段）。成功时返回值为 0?

```
	int get_mcn(struct cdrom_device_info *cdi,
		    struct cdrom_mcn *mcn)

```
某些盘片带有 `Media Catalog Number`（媒体目录号，MCN），也称?`Universal Product Code`（通用产品代码，UPC）。该编号应当反映通常印在产品条形码上的编号。遗憾的是，盘片上带有这种编号的少数盘片甚至没有使用相同的格式。该函数的返回参数是一个指向预先声明的内存区域的指针，区域类型?**struct cdrom_mcn**。MCN 应为一?13 字符的字符串，以 null 字符结尾?

```
	int reset(struct cdrom_device_info *cdi)

```
该调用应当对驱动器执行一次硬复位（尽管在确实需要硬复位的情况下，驱动器很可能已经不再听从命令了）。最好是在驱动器完成复位之后才把控制权返回给调用者。如果驱动器不再听从，底层的底层 cdrom 驱动明智的做法是超时退出?

```
	int audio_ioctl(struct cdrom_device_info *cdi,
			unsigned int cmd, void *arg)

```
`cdrom.h` 中定义的某些 CD-ROM-\ **ioctl()** 可以由上面描述的例程实现，因此函?**cdrom_ioctl** 将使用它们。然而，大多?**ioctl()** 处理音频控制。我们决定把它们留作通过单一函数访问，并重复参数 **cmd** ?**arg**。注意后者类型为 **void**，而不?**unsigned long int**?*cdrom_ioctl()** 例程确实做了一些有用的事情：它为所有音频调用把地址格式类型规整?**CDROM_MSF**（分、秒、帧）。它还验?**arg** 的内存位置，并为参数保留栈内存。这使得 **audio_ioctl()** 的实现比旧驱动方案简单得多。例如，你可以查看函?**cm206_audio_ioctl()**（`cm206.c` 中）以配合本文档更新?

未实现的 ioctl 应当返回 **-ENOSYS**，但无害的请求（例如 **CDROMSTART**）可以通过返回 0（成功）来忽略。其他错误应遵循相应的标准。当底层驱动返回错误时，统一 CD-ROM 驱动在可能时尽量把错误码返回给调用程序。（不过我们也可能决定在 **cdrom_ioctl()** 中规整返回值，以保证对音频播放器软件提供统一接口。）

```
	int dev_ioctl(struct cdrom_device_info *cdi,
		      unsigned int cmd, unsigned long arg)

```
某些 **ioctl()** 似乎是特定于某些 CD-ROM 驱动器的。也就是说，它们被引入是为了服务于某些驱动器的某些能力。事实上，有 6 种不同的 **ioctl()** 用于读取数据，要么以某种特定格式，要么是音频数据。我认为支持把音轨作为数据读取的驱动器不多，这是因为要保护艺术家的版权。此外，我认为如果支持音轨，应当通过 VFS 而非 **ioctl()** 来实现。这里的一个问题可能是音频帧长 2352 字节，因此音频文件系统要么应当一次性请?75264 字节?12 ?2352 的最小公倍数），要么驱动应当费劲去应对这种不一致（我对此持反对态度）。再者，硬件很难找到精确的帧边界，因为音频帧中没有同步头。一旦解决了这些问题，这段代码就应当?`cdrom.c` 中标准化?

因为有如此多?**ioctl()** 似乎是出于迁就某些驱动而引入的 [#f2]_，任何非标准?**ioctl()** 都通过调用 **dev_ioctl()** 路由。原则上，`private`（私有）**ioctl()** 的编号应当按照设备的主设备号来定，而不是通用?CD-ROM **ioctl** 编号 `0x53`。目前不被支持的 **ioctl()** 有：

	CDROMREADMODE1, CDROMREADMODE2, CDROMREADAUDIO, CDROMREADRAW,
	CDROMREADCOOKED, CDROMSEEK, CDROMPLAY-BLK and CDROM-READALL


   有真正使用这些接口的软件吗？我很感兴趣！


### CD-ROM 能力


除了仅仅实现某些 **ioctl** 调用之外，`cdrom.c` 中的接口还提供了表明 CD-ROM 驱动?**capabilities**（能力）的可能性。这可以通过在注册时?`cdrom.h` 中定义的任意数量的能力常量做 OR（或）运算来实现?

```
	CDC_CLOSE_TRAY		/* can close tray by software control */
	CDC_OPEN_TRAY		/* can open tray */
	CDC_LOCK		/* can lock and unlock the door */
	CDC_SELECT_SPEED	/* can select speed, in units of * sim*150 ,kB/s */
	CDC_SELECT_DISC		/* drive is juke-box */
	CDC_MULTI_SESSION	/* can read sessions *> rm1* */
	CDC_MCN			/* can read Media Catalog Number */
	CDC_MEDIA_CHANGED	/* can report if disc has changed */
	CDC_PLAY_AUDIO		/* can perform audio-functions (play, pause, etc) */
	CDC_RESET		/* hard reset device */
	CDC_IOCTLS		/* driver has non-standard ioctls */
	CDC_DRIVE_STATUS	/* driver implements drive status */

```
能力标志被声明为 **const**，以防止驱动意外篡改其内容。能力标志实际上告诉 `cdrom.c` 驱动能做什么。如果驱动找到的驱动器不具备该能力，可以通过 **cdrom_device_info** 变量 **mask** 把它屏蔽掉。例如，SCSI CD-ROM 驱动已经实现了装入和弹出 CD-ROM 的代码，因此它的 **capability** 中相应的标志会被设置。但一?SCSI CD-ROM 驱动器可能是匣式（caddy）系统，无法装入托盘，因此对于这个驱动器?*cdrom_device_info** 结构会在 **mask** 中设?**CDC_CLOSE_TRAY** 位?

```
	if (cdo->capability & ~cdi->mask & CDC _<capability>) ...

```
没有用于设置 mask ?**ioctl**……原因是我认为控?**behavior**（行为）比控?**capabilities**（能力）更好?

### 选项


最后一个标志寄存器控制 CD-ROM 驱动器的 **behavior**（行为），以满足不同用户的意愿，希望这能独立于相应作者（碰巧让该驱动器的支持进入 Linux 社区的人）的想法。该寄存器初始值为?

```
	CDO_AUTO_CLOSE	/* try to close tray upon device open() */
	CDO_AUTO_EJECT	/* try to open tray on last device close() */
	CDO_USE_FFLAGS	/* use file_pointer->f_flags to indicate purpose for open() */
	CDO_LOCK	/* try to lock door if device is opened */
	CDO_CHECK_TYPE	/* ensure disc type is data if opened for data */

```
该寄存器的初始值为 `CDO_AUTO_CLOSE | CDO_USE_FFLAGS | CDO_LOCK`，反映我个人对用户界面和软件标准的看法。在你抗议之前，`cdrom.c` 中实现了两个新的 **ioctl()**，允许你控制?

```
	CDROM_SET_OPTIONS	/* set options specified in (int)arg */
	CDROM_CLEAR_OPTIONS	/* clear options specified in (int)arg */

```
有一个选项需要更多解释：**CDO_USE_FFLAGS**。在下一节中我们将解释为什么需要这个选项?

一个名?`setcd` 的软件包，可?Debian 发行版和 `sunsite.unc.edu` 获取，允许用户级控制这些标志?


## 了解打开 CD-ROM 设备之目的的需?


传统上，Unix 设备可以以两种不同的 `modes`（模式）使用，要么通过对设备文件进行读/写，要么通过对设备发出控制命令，即设备的 **ioctl()** 调用。CD-ROM 驱动器的问题在于，它们可以用于两个完全不同的目的。其一是挂载可移动文件系统，即 CD-ROM；其二是播放音频 CD。音频命令完全通过 **ioctl()** 实现，大概是因为最初的实现（SUN?）就是如此。原则上这没什么问题，但对 `CD player`（CD 播放器）的良好控制要求设?**始终** 能够被打开，以便发?**ioctl** 命令，而不管驱动器处于什么状态?

另一方面，当用作可移动介质磁盘驱动器（这正是 CD-ROM 的最初目的）时，我们希望确保在打开设备时磁盘驱动器已准备好操作。在旧方案中，某?CD-ROM 驱动不做任何完整性检查，导致在试图在一个空驱动器上挂载 CD-ROM 时，VFS 向内核报告若?i/o 错误。这不是一种发现没有插?CD-ROM 的特别优雅的方式；它多少有点像老的 IBM-PC 试图读一个空软驱几秒钟，然后系统抱怨无法读取。如今我们可?**sense**（感知）驱动器中是否存在可移动介质，我们相信我们应当利用这一事实。在打开设备时进行一次完整性检查，验证 CD-ROM 是否可用及其正确类型（数据），将是可取的?

这两种使?CD-ROM 驱动器的方式——主要用于数据，其次用于播放音频盘——对 **open()** 调用的行为有不同要求。音频用途只是想打开设备以获得用于发?**ioctl** 命令所需的文件句柄，而数据用途想为正确可靠的数据传输而打开?

用户程序能够表明其打开设备?**purpose**（目的）的唯一方式，是通过 **flags**（标志）参数（见 `open(2)`）。对 CD-ROM 设备而言，这些标志并未实现（某些驱动实现了对写相关标志的检查，但如果设备文件拥有正确的权限标志，这并非严格必要）。大多数选项标志?CD-ROM 设备根本没有意义?*O_CREAT**?*O_NOCTTY**?*O_TRUNC**?*O_APPEND** ?**O_SYNC** ?CD-ROM 毫无意义?

因此我们提议使用标志 **O_NONBLOCK** 来表明设备被打开仅仅是为了发?**ioctl** 命令。严格地说，**O_NONBLOCK** 的含义是打开及随后对设备的调用不会导致调用进程等待。我们可以把它理解为：不要等待有人插入某个有效的数据 CD-ROM。因此，我们?CD-ROM ?**open()** 调用的实现提议如下：

- 如果没有设置?**O_RDONLY** 之外的其他标志，设备被打开用于数据传输，并且只有在传输成功初始化后才返?0。该调用甚至可能?CD-ROM 上引发一些动作，例如合上托盘?
- 如果设置了选项标志 **O_NONBLOCK**，除非整个设备不存在，否则打开将总是成功。驱动器不会采取任何动作?

### 那么标准呢？


你可能会犹豫是否接受这一提议，因为它来自 Linux 社区，而不是来自某个标准化机构。SUN、SGI、HP 以及所有那些其?Unix 和硬件厂商怎么说？嗯，这些公司处于一种幸运的位置：它们通常同时控制所支持产品的硬件和软件，并且规模足够大以设定自己的标准。它们不必应付十几种或更多不同且相互竞争的硬件配置\ [#f3]_?


   顺便说一句，我认?SUN 挂载 CD-ROM 的方式在根源上是很好的：?Solaris 下，一个卷守护进程自动把新插入?CD-ROM 挂载?`/cdrom/**<volume-name>**`?

   在我看来，它们本应把这一点推得更远，让局域网上的 **每个** CD-ROM 都挂载在类似的位置，即无论你?CD-ROM 插入哪台特定机器，它总是出现在目录树中的相同位置，在每个系统上都是如此。当我想?Linux 实现这样一个用户程序时，我遇到了各种驱动在行为上的差异，以及对一个报告介质变更的 **ioctl** 的需要?

我们相信，使?**O_NONBLOCK** 来表明设备被打开仅用?**ioctl** 命令，可以轻易地?Linux 社区中引入。所?CD 播放器的作者都必须被告知，我们甚至可以自己给这些程序发送补丁?*O_NONBLOCK** 的使用对其他操作系统（Linux 之外）上?CD 播放器的行为极有可能没有影响。最后，用户总能通过调用 **ioctl(file_descriptor, CDROM_CLEAR_OPTIONS, CDO_USE_FFLAGS)** 恢复到旧的行为?

### *open()* 的推荐策?


`cdrom.c` 中的例程被设计成可以通过 **CDROM_SET/CLEAR_OPTIONS** **ioctls** 在运行时配置 CD-ROM 设备?*任何** 类型）的行为。因此，可以设置多种操作模式?

`CDO_AUTO_CLOSE | CDO_USE_FFLAGS | CDO_LOCK`
   这是默认设置。（将来加上 **CDO_CHECK_TYPE** 会更好。）如果没有其他进程打开该设备，并且设备被打开用于数据（未设置 **O_NONBLOCK**）且发现托盘是打开的，则会尝试合上托盘。然后，验证驱动器中有一张盘片，并且如果设置?**CDO_CHECK_TYPE**，验证其中包?`data mode 1` 类型的轨道。只有当所有测试都通过时，返回值才为零。门被锁定以防止文件系统损坏。如果驱动器被打开用于音频（设置了 **O_NONBLOCK**），则不采取任何动作，返回值为 0?

`CDO_AUTO_CLOSE | CDO_AUTO_EJECT | CDO_LOCK`
   这模仿了当前 sbpcd-driver 的行为。选项标志被忽略，必要时在第一次打开时合上托盘。类似地，在最后一?release 时打开托盘，即如果卸载了一?CD-ROM，它会自动弹出，以便用户更换?

我们希望这些选项能够说服所有人（驱动维护者和用户程序开发者）采纳新的 CD-ROM 驱动方案和选项标志解释?

## `cdrom.c` 中例程的描述


`cdrom.c` 中只有少数例程导出给了驱动。在这一新节中我们将讨论这些例程，以及那?`take over`（接管）对内核的 CD-ROM 接口的的函数。`cdrom.c` 所属的头文件叫?`cdrom.h`。以前，这个文件的部分内容放在文?`ucdrom.h` 中，但现在该文件已经合并回了 `cdrom.h`?

```
	struct file_operations cdrom_fops

```
该结构的内容已在 cdrom_api_ 中描述。指向该结构的指针被赋给 **struct gendisk** ?**fops** 字段?

```
	int register_cdrom(struct cdrom_device_info *cdi)

```
该函数的使用方式，大致上与把 **cdrom_fops** 注册到内核的方式相同——设备操作和信息结构，如 cdrom_api_ 中所述，应当用以下方式注册：

```
	register_cdrom(&<device>_info);

```
该函数在成功时返回零，失败时返回非零。结?**<device>_info** 应当有一个指向以下内容的指针?

```
	struct cdrom_device_info <device>_info = {
		<device>_dops;
		...
	}

```
注意，一个驱动必须有一个静态结?**<device>_dops**，而它可以根据活跃的次设备数量拥有任意多个结构 **<device>_info**?*Register_cdrom()** 用这些构建一个链表?


```
	void unregister_cdrom(struct cdrom_device_info *cdi)

```
把次设备号为 **MINOR(cdi->dev)** 的设?**cdi** 注销，会从列表中移除该次设备。如果它是该底层驱动注册的最后一个次设备，则断开已注册的设备操作例程?CD-ROM 接口的连接。该函数在成功时返回零，失败时返回非零?

```
	int cdrom_open(struct inode * ip, struct file * fp)

```
该函数不会被底层驱动直接调用，它列在标准 **cdrom_fops** 中。如?VFS 打开一个文件，该函数被激活。该例程中实现了一种策略，处理连接到该设备?**cdrom_device_ops** 中设置的所有能力和选项。然后，程序流程转移到设备相关的 **open()** 调用?

```
	void cdrom_release(struct inode *ip, struct file *fp)

```
该函数实现了 **cdrom_open()** 的逆逻辑，然后调用设备相关的 **release()** 例程。当使用计数达到 0 时，通过调用 **sync_dev(dev)** ?**invalidate_buffers(dev)** 刷新已分配的缓冲区?



```
	int cdrom_ioctl(struct inode *ip, struct file *fp,
			unsigned int cmd, unsigned long arg)

```
该函数以统一的方式处?CD-ROM 设备的所有标?**ioctl** 请求。这些不同的调用分为三类：可以直接由设备操作实现?**ioctl()**、通过调用 **audio_ioctl()** 路由的，以及其余那些大概是设备相关的。通常，负的返回值表示出错?

### 直接实现?*ioctl()*


下列 `old`（旧的）CD-ROM **ioctl()** ?**cdrom_device_ops** 中实现且未被屏蔽的情况下，通过直接调用设备操作来实现：

`CDROMMULTISESSION`
	请求 CD-ROM 上的最后一个会话?
`CDROMEJECT`
	打开托盘?
`CDROMCLOSETRAY`
	合上托盘?
`CDROMEJECT_SW`
	如果 **arg\not=0**，设置行为为自动合上（第一次打开时合上托盘）和自动弹出（最后一次释放时弹出），否则设置行为为在 **open()** ?**release()** 调用时不移动?
`CDROM_GET_MCN`
	?CD 获取媒体目录号?

### 通过 *audio_ioctl()* 路由?*ioctl*


下面这组 **ioctl()** 都通过调用 **cdrom_fops** 函数 **audio_ioctl()** 实现。内存检查和分配?**cdrom_ioctl()** 中执行，地址格式?*CDROM_LBA**/**CDROM_MSF**）的规整也在其中完成?

`CDROMSUBCHNL`
	在类型为 `struct cdrom_subchnl *` 的参?**arg** 中获取子通道数据?
`CDROMREADTOCHDR`
	读取目录（Table of Contents）头，在类型?`struct cdrom_tochdr *` ?**arg** 中?
`CDROMREADTOCENTRY`
	?**arg** 中读取一个目录项，并由类型为 `struct cdrom_tocentry *` ?**arg** 指定?
`CDROMPLAYMSF`
	播放以分、秒、帧格式指定的音频片段，由类型为 `struct cdrom_msf *` ?**arg** 界定?
`CDROMPLAYTRKIND`
	以轨-索引格式播放音频片段，由类型?`struct cdrom_ti *` ?**arg** 界定?
`CDROMVOLCTRL`
	设置由类型为 `struct cdrom_volctrl *` ?**arg** 指定的音量?
`CDROMVOLREAD`
	把音量读入由类型?`struct cdrom_volctrl *` ?**arg** 指定的位置?
`CDROMSTART`
	使盘片加速旋转?
`CDROMSTOP`
	停止播放音频片段?
`CDROMPAUSE`
	暂停播放音频片段?
`CDROMRESUME`
	恢复播放?

### `cdrom.c` 中新?*ioctl()*


下列 **ioctl()** 被引入，以允许用户程序控制各?CD-ROM 设备的行为。新?**ioctl** 命令可以通过其名称中的下划线来识别?

`CDROM_SET_OPTIONS`
	设置?**arg** 指定的选项。返回修改后的选项标志寄存器。使?**arg = \rm0** 读取当前标志?
`CDROM_CLEAR_OPTIONS`
	清除?**arg** 指定的选项。返回修改后的选项标志寄存器?
`CDROM_SELECT_SPEED`
	选择盘片磁头速率，由 **arg** 以标?cdrom 速度为单位（176\,kB/sec 原始数据?150kB/sec 文件系统数据）指定。?0 表示 `auto-select`（自动选择），即音频盘以实时速度播放，数据盘以最大速度读取。数?**arg** 会对照在 **cdrom_dops** 中找到的驱动器最大磁头速率进行检查?
`CDROM_SELECT_DISC`
	从换片器（juke-box）中选择编号?**arg** 的盘片?

	第一张盘片编号为 0?*arg** 会对照在 **cdrom_dops** 中找到的换片器中盘片的最大数量进行检查?
`CDROM_MEDIA_CHANGED`
	如果自上次调用以来盘片已更换则返?1。对于换片器，额外的参数 **arg** 指定了提供信息的槽位。特殊?**CDSL_CURRENT** 请求返回关于当前选中槽位的信息?
`CDROM_TIMED_MEDIA_CHANGE`
	检查自用户提供的某个时间以来盘片是否已更换，并返回最后一次盘片更换的时间?

	**arg** 是指?**cdrom_timed_media_change_info** 结构的指针?*arg->last_media_change** 可由调用代码设置，以表示已知的最后一次介质变更的时间戳（由调用者给出）。成功返回时，该 ioctl 调用会把 **arg->last_media_change** 设为内核/驱动所知的最新介质变更时间戳（以毫秒计），并?**arg->has_changed** 设为 1（如果该时间戳比调用者设置的时间戳更新）?
`CDROM_DRIVE_STATUS`
	通过调用 **drive_status()** 返回驱动器的状态。返回值在 cdrom_drive_status_ 中定义。注意，该调用不返回驱动器当前播放活动的信息；这可通过?**CDROMSUBCHNL** 发出 **ioctl** 调用来轮询。对于换片器，额外的参数 **arg** 指定了提供（可能受限的）信息的槽位。特殊?**CDSL_CURRENT** 请求返回关于当前选中槽位的信息?
`CDROM_DISC_STATUS`
	返回驱动器中当前盘片的类型。它应当被看作是?**CDROM_DRIVE_STATUS** 的补充。该 **ioctl** 可以提供关于驱动器中插入的当前盘片的 **某些** 信息。这一功能过去由底层驱动实现，但现在完全在统一 CD-ROM 驱动中执行?

	CD 作为各种数字信息载体介质的使用发展史，导致了许多不同的盘片类型。该 **ioctl** 仅在 CD 上只?**一?* 类型的数据时才有用。虽然这经常是事实，?CD 同时拥有一些数据轨道和一些音频轨道也非常常见。因为这是一个已有的接口，而不是通过改变其所基于的假设来修复该接口（从而破坏所有使用该功能的用户程序），统一 CD-ROM 驱动按如下方式实现该 **ioctl**：如果所讨论?CD 上有音频轨道，并且它上面绝对没有 CD-I、XA 或数据轨道，它将被报告为 **CDS_AUDIO**。如果它同时有音频和数据轨道，它将返?**CDS_MIXED**。如果盘片上没有音频轨道，并且所讨论?CD 上有任何 CD-I 轨道，它将被报告?**CDS_XA_2_2**。如果还不行，如果所讨论?CD 上有任何 XA 轨道，它将被报告?**CDS_XA_2_1**。最后，如果所讨论?CD 上有任何数据轨道，它将被报告为数?CD?*CDS_DATA_1**）?

```
		CDS_NO_INFO	/* no information available */
		CDS_NO_DISC	/* no disc is inserted, or tray is opened */
		CDS_AUDIO	/* Audio disc (2352 audio bytes/frame) */
		CDS_DATA_1	/* data disc, mode 1 (2048 user bytes/frame) */
		CDS_XA_2_1	/* mixed data (XA), mode 2, form 1 (2048 user bytes) */
		CDS_XA_2_2	/* mixed data (XA), mode 2, form 1 (2324 user bytes) */
		CDS_MIXED	/* mixed audio/data disc */

	For some information concerning frame layout of the various disc
	types, see a recent version of `cdrom.h`.

```
`CDROM_CHANGER_NSLOTS`
	返回换片器中的槽位数量?
`CDROMRESET`
	复位驱动器?
`CDROM_GET_CAPABILITY`
	返回驱动器的 **capability** 标志。关于这些标志的更多信息，参?cdrom_capabilities_ 一节?
`CDROM_LOCKDOOR`
	锁定驱动器的门。`arg == 0` 解锁门，任何其他值锁定它?
`CDROM_DEBUG`
	打开调试信息。只允许 root 这样做。语义与 CDROM_LOCKDOOR 相同?


### 设备相关?*ioctl()*


最后，所有其?**ioctl()** 都被传给函数 **dev_ioctl()**（如果已实现）。不执行内存分配或验证?

## 如何更新你的驱动


- 备份你当前的驱动?
- 获取文件 `cdrom.c` ?`cdrom.h`，它们应当位于随本文档一同提供的目录树中?
- 确保你包含了 `cdrom.h`?
- ?**register_blkdev** 的第 3 个参数从 `&<your-drive>_fops` 改为 `&cdrom_fops`?
- 就在该行之后，添加以下内容以向统一 CD-ROM 驱动注册?

```
	register_cdrom(&<your-drive>_info);*

  Similarly, add a call to *unregister_cdrom()* at the appropriate place.
```
- 把设备操?**struct** 的一个例子复制到你的源码中，例如来自 `cm206.c` ?**cm206_dops**，并把所有条目改成与你驱动对应的名字，或你碰巧喜欢的名字。如果你的驱动不支持某个函数，把该条目设?**NULL**。在 **capability** 条目处，你应列出你的驱动当前支持的所有能力。如果你的驱动拥有某个未列出的能力，请给我发消息?
- 从同一个示例驱动复?**cdrom_device_info** 声明，并根据你的需要修改条目。如果你的驱动动态确定硬件的能力，该结构也应当动态声明?
- 根据 `cdrom.h` 中列出的原型?cdrom_api_ 中给出的规格，实现你?`<device>_dops` 结构中的所有函数。你很可能已经实现了其中很大一部分代码，并且你几乎肯定需要调整原型和返回值?
- 把你?`<device>_ioctl()` 函数重命名为 **audio_ioctl** 并稍微改动原型。移?cdrom_ioctl_ 第一部分中列出的条目，如果你的代码没问题，这些只是对你在前一步调整的例程的调用?
- 你可以移?**audio_ioctl()** 函数中所有处理音频命令的内存检查代码（这些列在 cdrom_ioctl_ 的第二部分）。也不需要内存分配，因此 **switch** 中的大多?**case** 形如?

```
	case CDROMREADTOCENTRY:
		get_toc_entry\bigl((struct cdrom_tocentry *) arg);

- 所有剩余的 **ioctl** case 必须移到一个独立的函数 **<device>_ioctl** 中，即设备相关的 **ioctl()**。注意，内存检查和分配必须保留在这段代码中。
- 改变 **<device>_open()** ?**<device>_release()** 的原型，并移除任何策略性代码（即托盘运动、舱门锁定等）。
- 尝试重新编译驱动。我们建议你使用模块，无论是 `cdrom.o` 还是你的驱动，因为这样调试要容易得多。

## 致谢


感谢所有参与的人。首先感?Erik Andersen，他接过了维?`cdrom.c` 并在 2.1 内核中整合大?CD-ROM 相关代码的火炬。感?Scott Snyder ?Gerd Knorr，他们是率先?SCSI ?IDE-CD 驱动实现这一接口、并相对?kernel~2.0 为数据结构扩展提出许多想法的人。进一步感?Heiko Eißfeldt、Thomas Quinot、Jon Tombs、Ken Pizzini、Eberhard Mönkeberg ?Andrew Kroll，这?Linux CD-ROM 设备驱动开发者在撰写过程中善意地给出了建议和批评。最后当然要感谢 Linus Torvalds，是他首先让这一切成为可能。
