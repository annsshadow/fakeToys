
## 虚拟加速器交换机（Virtual Accelerator Switchboard，VAS）用户空API


## 简

Power9 处理器引入了虚拟加速器交换机（VAS），它允许用户空间和内核与被称为
Nest Accelerator（NX）的协处理器（硬件加速器）进行通信。NX 单元由一个或多个
硬件引擎或协处理器类型组成，例如 842 压缩、GZIP 压缩和加密。在 power9 上，
用户空间应用程序只能访问支持硬件ZLIB GZIP 压缩算法GZIP 压缩引擎
为了NX 通信，内核必须建立一个通道或窗口，然后请求就可以直接提交，而无需
内核参与。发往 GZIP 引擎的请求必须被格式化为协处理器请求块（CRB），并且这些
CRB 必须使用 COPY/PASTE 指令提交NX，把 CRB 粘贴到与该引擎请求队列相关联硬件地址上
GZIP 引擎提供两个优先级的请求：普通（Normal）和高（High）。目前从用户空间
只支持普通请求
本文档解释了用于与内核交互以建立通道/窗口的用户空API，该窗口可用于直接向
NX 加速器发送压缩请求

## 概述


通过VAS/NX 设备驱动实现/dev/crypto/nx-gzip 设备节点，提供对 GZIP 引擎
的访问。应用程序必须打开 /dev/crypto/nx-gzip 设备以获得一个文件描述符（fd）然后应当用这fd 发出 VAS_TX_WIN_OPEN ioctl 来建立与引擎的连接。这意味着该进程在 GZIP 引擎上打开了一个发送窗口。一旦建立连接，应用程序就应当使mmap() 系统调用把引擎请求队列的硬件地址映射到应用程序的虚拟地址空间
然后，应用程序可以通过使用 copy/paste 指令CRB 粘贴mmap() 返回的虚拟地址
（即 paste_address）来向引擎提交一个或多个请求。用户空间可以通过关闭文件描述（close(fd)）或在进程退出时关闭已建立的连接或发送窗口
注意，应用程序可以用同一个窗口发送多个请求，也可以建立多个窗口，但每个文描述符对应一个窗口
以下各节提供关于各个步骤的更多细节和参考

## NX-GZIP 设备节点


系统中有一/dev/crypto/nx-gzip 节点，它提供对系统中所GZIP 引擎的访问/dev/crypto/nx-gzip 唯一有效的操作是
 - 以读写方open() 该设备 - 发出 VAS_TX_WIN_OPEN ioctl
 - 把引擎的请求队列 mmap() 到应用程序的虚拟地址空间（即获得协处理器引擎   paste_address） - 关闭该设备节点
该设备节点上的其他文件操作是未定义的
注意，copy paste 操作直接发往硬件，并不经过该设备。更多细节请参COPY/PASTE 文档
尽管一个系统可能拥有多NX 协处理器引擎的实例（通常每个 P9 芯片一个），但
系统中只有一/dev/crypto/nx-gzip 设备节点。当打开 nx-gzip 设备节点时，内核
在一个合适的 NX 加速器实例上打开发送窗口。它会找到用户进程正在其上执行的 CPU并确定该 CPU 所属的相应芯片上的 NX 实例
应用程序可以使用 VAS_TX_WIN_OPEN ioctl 中的 vas_id 字段来选择特定NX 协处理器
实例，详见下文
一个名libnxz 的用户空间库可在此获取，但仍在开发中
	 https://github.com/abalib/power-gzip

使用 inflate / deflate 调用的应用程序可以链libnxz 而非 libz，从而无需任何
修改即可使用 NX GZIP 压缩

## 打开 /dev/crypto/nx-gzip


nx-gzip 设备应当以读写方式打开。打开该设备不需要特殊权限。每个窗口对应一个文描述符。所以如果用户空间进程需要多个窗口，就必须发出多open 调用
关于返回值、错误码和限制等其他细节，请参阅 open(2) 系统调用手册页

## VAS_TX_WIN_OPEN ioctl


应用程序应当如下使用 VAS_TX_WIN_OPEN ioctl 来与 NX 协处理器引擎建立连接
```
		struct vas_tx_win_open_attr {
			__u32   version;
			__s16   vas_id; /* specific instance of vas or -1
						for default */
			__u16   reserved1;
			__u64   flags;	/* For future use */
			__u64   reserved2[6];
		};

	version:
		version 字段目前必须设置1	vas_id:
		如果传入 '-1'，内核将尽最大努力为进程分配一个最优的 NX
		实例。要选择特定VAS 实例，请参考下方的“可VAS 引擎		发现”一节
	flags、reserved1 reserved2[6] 字段用于未来的扩展，必须设置0
	VAS_TX_WIN_OPEN ioctl 的属attr 定义如下::

		#define VAS_MAGIC 'v'
		#define VAS_TX_WIN_OPEN _IOW(VAS_MAGIC, 1,
						struct vas_tx_win_open_attr)

		struct vas_tx_win_open_attr attr;
		rc = ioctl(fd, VAS_TX_WIN_OPEN, &attr);

	VAS_TX_WIN_OPEN ioctl 成功时返0。出错时，返-1 并设errno
	变量以指示错误
	错误条件
		======	================================================
		EINVAL	fd 不指向一个有效的 VAS 设备		EINVAL	无效vas ID
		EINVAL	version 未设置为正确的		EEXIST	给定fd 已经打开了窗		ENOMEM	没有可用内存来分配窗		ENOSPC	系统已打开的活跃窗口（连接）过		EINVAL	保留字段未被设置0		======	================================================

	关于更多细节、错误码和限制，请参ioctl(2) 手册页
```
## mmap() NX-GZIP 设备


针对 NX-GZIP 设备 fd mmap() 系统调用返回一paste_address，应用程序可
用它CRB 复制/粘贴到硬件引擎
```
		paste_addr = mmap(addr, size, prot, flags, fd, offset);

	NX-GZIP 设备 fd 进行 mmap 的唯一限制是：

		* size 应为 PAGE_SIZE
		* offset 参数应为 0ULL

	关于更多细节/限制，请参阅 mmap(2) 手册页。除mmap(2) 手册页上
	列出的错误条件之外，也可能因以下某个错误码而失败：

		======	=============================================
		EINVAL	fd 没有关联一个已打开的窗			（即 mmap() 没有跟在成功VAS_TX_WIN_OPEN
			ioctl 调用之后）		EINVAL	offset 字段不是 0ULL		======	=============================================

```
## 可用 VAS 引擎的发

系统中的每个可用 VAS 实例都会有一个设备树节点，例/proc/device-tree/vas@** /proc/device-tree/xscom@**/vas@*。确定芯片或 VAS
实例，并使用该节点中ibm,vas-id 属性值来选择特定VAS 实例

## Copy/Paste 操作


应用程序应当使用 copy paste 指令来向 NX 发CRB。关Copy/Paste 指令请参PowerISA 的第 4.4 节：
https://openpowerfoundation.org/?resource_lib=power-isa-version-3-0


## CRB 规范与使NX


应用程序应当使用协处理器请求块（CRB）来格式化发往协处理器的请求。关CRB 格式以及从用户空间使NX（例如发送请求和检查请求状态），请参阅 NX-GZIP 用户
手册

## NX 错误处理


应用程序NX 发送请求，并通过轮询协处理器状态块（CSB）标志来等待状态。NX 每个请求处理完成后更CSB 中的状态。关CSB 的格式和状态标志，请参NX-GZIP
用户手册
如果 NX CSB 地址或任何请求缓冲区上遇到转换错误（称为 NX 页错误），就会在
CPU 上引发一个中断来处理该错误。如果应用程序传入了无效地址，或者请求缓冲区
不在内存中，就可能发生页错误。操作系统通过以下方式处理该错误：
```
	csb.flags = CSB_V;
	csb.cc = CSB_CC_FAULT_ADDRESS;
	csb.ce = CSB_CE_TERMINATION;
	csb.address = fault_address;
```
当应用程序收到转换错误时，它可以触及或访问带有错误地址的页，使其位于内存中然后应用程序可以重新NX 发送该请求
如果操作系统由于无效CSB 地址而无法更CSB，就会向打开发送窗口的进程发SEGV 信号，原始请求正是通过该窗口发出的```
	siginfo.si_signo = SIGSEGV;
	siginfo.si_errno = EFAULT;
	siginfo.si_code = SEGV_MAPERR;
	siginfo.si_addr = CSB address;
```
对于多线程应用程序，NX 发送窗口可以在所有线程之间共享。例如，一个子线程可以
打开一个发送窗口，但其他线程可以使用这个窗口向 NX 发送请求。只CSB 地址有效这些请求即使在操作系统处理错误的情形下也会成功。如NX 请求包含无效CSB 地址信号将被发送给打开该窗口的子线程。但如果该线程在没有关闭窗口的情况下退出，并且
请求是使用这个窗口发出的，信号将被发给线程组组长（tgid）。应用程序可以忽略或
处理这些信号，由应用程序自行决定
NX-GZIP 用户手册https://github.com/libnxz/power-gzip/blob/master/doc/power_nx_gzip_um.pdf


## 简单示

```
		int use_nx_gzip()
		{
			int rc, fd;
			void *addr;
			struct vas_setup_attr txattr;

			fd = open("/dev/crypto/nx-gzip", O_RDWR);
			if (fd < 0) {
				fprintf(stderr, "open nx-gzip failed\n");
				return -1;
			}
			memset(&txattr, 0, sizeof(txattr));
			txattr.version = 1;
			txattr.vas_id = -1
			rc = ioctl(fd, VAS_TX_WIN_OPEN,
					(unsigned long)&txattr);
			if (rc < 0) {
				fprintf(stderr, "ioctl() n %d, error %d\n",
						rc, errno);
				return rc;
			}
			addr = mmap(NULL, 4096, PROT_READ|PROT_WRITE,
					MAP_SHARED, fd, 0ULL);
			if (addr == MAP_FAILED) {
				fprintf(stderr, "mmap() failed, errno %d\n",
						errno);
				return -errno;
			}
			do {
				//Format CRB request with compression or
				//uncompression
				// Refer tests for vas_copy/vas_paste
				vas_copy((&crb, 0, 1);
				vas_paste(addr, 0, 1);
				// Poll on csb.flags with timeout
				// csb address is listed in CRB
			} while (true)
			close(fd) or window can be closed upon process exit
		}

	Refer https://github.com/libnxz/power-gzip for tests or more
	use cases.

```
