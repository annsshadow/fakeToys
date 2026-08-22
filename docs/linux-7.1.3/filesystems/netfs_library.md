
## 网络文件系统服务库（Network Filesystem Services Library

 - 概述   - 请求与流   - 子请求   - 结果收集与重试   - 本地缓存   - 内容加密（fscrypt） - inode 上下文   - inode 上下文辅助函数   - inode 锁   - inode 回写 - 高层 VFS API   - 未加锁的 read/write iter   - 预加锁的 read/write iter   - 整体文件 API   - 内存映射 I/O API - 高层 VM API   - 已废弃的 PG_private_2 API - I/O 请求 API   - 请求结构   - 流结构   - 子请求结构   - 文件系统方法   - 终止子请求   - 本地缓存 API - API 函数参考

## 概述


网络文件系统服务库（netfslib）是一组函数，旨在帮助网络文件系统实现 VM/VFS API 操作。它接管了常规的缓冲读、预读、写和回写，同时也处理非缓冲和直I/O
该库提供I/O 大小（重新）协商、重试失败的 I/O 以及本地缓存的支持，并且未来还将提供内容加密
它尽可能地将文件系统VM 接口的变化隔离开来，并处理诸如大型多folio 之类VM 特性。文件系统基本上只需要提供一种执行读RPC 调用的方法
netfslib 内部组织 I/O 的方式由若干对象构成
 - 一*请求（request*。请求用于跟I/O 的整体进度并持有资源。结果的收集在请求层进行。请求内I/O 被划分为若干并行的子请求流
 - 一*流（stream*。一组互不重叠的子请求序列。流内的子请求不必是连续的
 - 一*子请求（subrequest*。这I/O 的基本单元。它代表一次单独的 RPC 调用或一次单独的缓存 I/O 操作。库将这些传递给文件系统和缓存去执行
### 请求与流


当真正执I/O 时（与仅仅复制到 pagecache 相对），netfslib 会创建一个或多个请求来跟I/O 的进度并持有资源
读操作将只有一个流，该流内的子请求可能来自混合的来源，例如混合 RPC 子请求和缓存子请求
另一方面，写操作可能有多个流，其中每个流面向不同的目标。例如，可能有一个流写入本地缓存，另一个流写入服务器。目前只允许两个流，但如果需要对多个服务器进行并行写入，可以增加
写流中的子请求不需要与另一个写流中的子请求对齐或大小一致，netfslib 会独立地将每个流中的子请求平铺到源缓冲区上。此外，每个流可能包含与另一个流中的空洞不对应的空洞
另外，子请求不需要与目标缓冲区中 folio vector 的边界对应。库负责结果的收集以folio 标志与引用的处理
### 子请

子请求是 netfslib 与使用它的文件系统之间交互的核心。每个子请求应当对应于一次单独的读或RPC 或缓存操作。库会将一组子请求的结果拼接起来，以提供更高层次的操作
netfslib 在建立子请求时，与文件系统或缓存有两次交互。首先，有一个可选的准备步骤，允许文件系统协商子请求的限制，包括最大字节数和最vector 数（例如用于 RDMA）。这可能涉及与服务器协商（例cifs 需要获取信用额度）
其次，是分发步骤，在此步骤中子请求被移交给文件系统执行
注意，读和写之间这两个步骤的做法略有不同
 - 对于读，VM/VFS 会预先告知我们要请求多少数据，因此库可以预设最大值，然后缓存以及之后的文件系统可以逐步减小该值。缓存也会首先被咨询是否想要执行读，然后才咨询文件系统
 - 对于回写，在遍历 pagecache 之前，无法知道将要写入多少数据，因此库不设置任何限制
一旦子请求完成，文件系统或缓存会通知库完成，然后调用结果收集。根据请求是同步还是异步，结果的收集将在应用程序线程或工作队列中进行
### 结果收集与重

随着子请求的完成，库会收集和整理结果，并逐步执行 folio 解锁（如果合适）。一旦请求完成，将调用异步完成（同样地，如果合适）。文件系统可以向库提供临时的进度报告，以便在可能的情况下folio 解锁更早发生
如果有任何子请求失败，netfslib 可以重试它们。它会等待所有子请求完成，给文件系统机会去调整请求持有的资源/状态，并在重新准备和重新分发子请求之前对其进行处理
这允许改变流中一组连续失败子请求的平铺方式，根据需要增加子请求或丢弃多余的部分（例如，如果网络大小发生变化，或者服务器决定需要更小的块）
此外，如果一个或多个连续的缓存读子请求失败，库会将其交给文件系统执行，并根据文件系统的参数（而非缓存的参数）重新协商并重新平铺它们
### 本地缓存


netfslib 通过 `fscache` 提供的服务之一，是选择将来写入网络文件系统的数据副本缓存在本地磁盘上。如果有一cookie 附加`netfs_inode` 上，库将自动代表文件系统管理数据的存储、检索和部分失效
注意，本地缓存过去使PG_private_2（别名为 PG_fscache）来跟踪正在写入缓存的页，但现在已经废弃，因PG_private_2 将被移除
相反，从服务器读取的、而缓存中没有数据folio 将被标记为脏，并`folio->private` 设置为一个特殊值（`NETFS_FOLIO_COPY_TO_CACHE`），并留待回写写入。如果在该操作发生之folio 被修改，该特殊值将被清除，该写入将变为普通的脏状态
当回写发生时，如此标记的 folio 将只写入缓存而不写入服务器。回写通过使用两个流来处理混合的仅缓存写入和服务器与缓存写入，一个发往缓存，一个发往服务器。服务器流中将包含与这些 folio 对应的间隙
### 内容加密（fscrypt

尽管目前还没有这样做，但 netfslib 终将获得代表网络文件系统（例Ceph）执行客户端内容加密的能力。如果合适（也可能不合适，例如 cifs），可以使用 fscrypt
数据将使用与写入服务器的数据相同的加密方式加密后存储在本地缓存中，库将在必要时实施反弹缓冲和 RMW 周期

## inode 上下

网络文件系统辅助库需要为每个由其帮助管理netfs inode 存储一些状态。为此，提供了一个上下文
```

	struct netfs_inode {
		struct inode inode;
		const struct netfs_request_ops *ops;
		struct fscache_cookie * cache;
		loff_t remote_i_size;
		unsigned long flags;
		...
	};

```
想要使用 netfslib 的网络文件系统必须将此结构放入其 inode 封装结构体中，以替代 VFS `struct inode`。这可以通过以下方式完成
```

	struct my_inode {
		struct netfs_inode netfs; /* Netfslib context and vfs inode */
		...
	};

```
这使netfslib 能够通过 `container_of()` inode 指针找到其状态，从而允netfslib 辅助函数VFS/VM 操作表直接指向
该结构中包含文件系统感兴趣的以下字段
 - `inode`

   VFS inode 结构
 - `ops`

   网络文件系统提供netfslib 的一组操作
 - `cache`

   本地缓存 cookie，如果未启用缓存则为 NULL。如果禁用了 fscache，该字段不存在
 - `remote_i_size`

   服务器上文件的大小。如果已进行了本地修改但尚未写回，则该值与 inode->i_size 不同
 - `flags`

   一组标志，其中一些文件系统可能感兴趣
   - `NETFS_ICTX_MODIFIED_ATTR`

     如果 netfslib 修改mtime/ctime 则设置。文件系统可以自由忽略或清除它
   - `NETFS_ICTX_UNBUFFERED`

     对该文件执行非缓I/O。类似于直接 I/O，但没有对齐限制。必要时将执RMW。除非同时使mmap()，否则不会使pagecache
   - `NETFS_ICTX_WRITETHROUGH`

     对该文件执行直写（writethrough）缓存。当向页缓存进行缓冲写时，I/O 将被建立和分发。mmap() 执行正常的回写操作
   - `NETFS_ICTX_SINGLE_NO_UPLOAD`

     如果该文件的内容是整块（monolithic）的，必须一次性整体读取且不得写回服务器，则设置此标志，但可以缓存（例AFS 目录）
### inode 上下文辅助函

为了帮助处理inode 上下文，提供了一组辅助函数。首先，一个用于对上下文执行基本初始化的函```

	void netfs_inode_init(struct netfs_inode *ctx,
			      const struct netfs_request_ops *ops);

```
```

	struct netfs_inode *netfs_inode(struct inode *inode);

```
最后，一个用于从上下文获取缓cookie 指针的函```

	struct fscache_cookie *netfs_i_cookie(struct netfs_inode *ctx);

```
### inode 閿。

提供了一组函数用于管I/O mmap i_rwsem ```

	int netfs_start_io_read(struct inode *inode);
	void netfs_end_io_read(struct inode *inode);
	int netfs_start_io_write(struct inode *inode);
	void netfs_end_io_write(struct inode *inode);
	int netfs_start_io_direct(struct inode *inode);
	void netfs_end_io_direct(struct inode *inode);

```
排他性分为四个独立的类别
 1) 缓冲读和写
    缓冲读可以彼此并发运行，也可以与缓冲写并发运行，但缓冲写彼此之间不能并发运行
 2) 直接读和写
    直接（与非缓冲）读和写可以并发运行，因为它们不共享本地缓冲（pagecache），并且在网络文件系统中，预期排他性由服务器管理（尽管对于 Ceph 等情况可能并非如此）
 3) 其他主要inode 修改操作（例truncate、fallocate）
    这些应直接访i_rwsem
 4) mmap()銆?
    mmap 映射的访问可能与其他任何类别并发运行。它们可能构成文件内环回 DIO 写的缓冲区。它们可能被允许出现在非缓冲文件上
### inode 回写


inode 被弄脏时，netfslib 会为未来的回写固inode 上的资源（例如固fscache cookie 的使用）。然而，这种固定需要谨慎管理。为了管理固定，会发生以下序列：

 1) 当固定开始时（例如当某个 folio 被弄脏时），如果缓存处于活动状态，netfslib 会设置一inode 状态标`I_PINNING_NETFS_WB`，以阻止缓存结构被丢弃以及缓存空间被回收。如果该标志已设置，这也可以防止重新获取缓存资源
 2) 该标志随后在 VM 中的 inode 锁内、inode 回写期间被清除——并且其已被设置的事实被转移`struct writeback_control` 中的 `->unpinned_netfs_wb`
 3) 如果现在设置`->unpinned_netfs_wb`，则强制调用 write_inode 过程
 4) 调用文件系统`->write_inode()` 函数进行清理
 5) 文件系统调用 netfs 进行清理```

	int netfs_unpin_writeback(struct inode *inode, struct writeback_control *wbc);

```
如果文件系统不需要做其他事情，可以将其设置为它的 `.write_inode` 方法
此外，如果一inode 被删除，文件系统write_inode 方法可能不会
```

	void netfs_clear_inode_writeback(struct inode *inode, const void *aux);

```
必须`->evict_inode()` 中、在调用 `clear_inode()` **之前**调用

## 高层 VFS API


netfslib 提供多组 API 调用，供文件系统VFS 操作委托给它。netfslib 反过来会调用文件系统和缓存来协商 I/O 大小、发RPC，并在不同时机提供其介入的位置
### 未加锁的 Read/Write Iter


第一API 用于在文件系统通过标准 VFS 方法被调用、但需要先或后做其他事情、同时仍处于加锁区段内时，将操作委托netfslib
```

	ssize_t netfs_file_read_iter(struct kiocb *iocb, struct iov_iter *iter);
	ssize_t netfs_file_write_iter(struct kiocb *iocb, struct iov_iter *from);
	ssize_t netfs_buffered_read_iter(struct kiocb *iocb, struct iov_iter *iter);
	ssize_t netfs_unbuffered_read_iter(struct kiocb *iocb, struct iov_iter *iter);
	ssize_t netfs_unbuffered_write_iter(struct kiocb *iocb, struct iov_iter *from);

```
它们可以直接赋给 `.read_iter` `.write_iter`。它们自己执inode 锁，前两个会在缓I/O DIO 之间按需切换
### 预加锁的 Read/Write Iter


第二API 用于在文件系统通过标准 VFS 方法被调用、但需要先或后做其他事情、同时仍处于加锁区段内时，将操作委托netfslib
```

	ssize_t netfs_unbuffered_read_iter_locked(struct kiocb *iocb, struct iov_iter *iter);

```
它不能直接赋`.read_iter`，文件系统负责在调用它之前执inode 锁。对于缓冲读，文件系统应使用 `filemap_read()````

	ssize_t netfs_buffered_write_iter_locked(struct kiocb *iocb, struct iov_iter *from,
					 struct netfs_group *netfs_group);
	ssize_t netfs_perform_write(struct kiocb *iocb, struct iov_iter *iter,
				    struct netfs_group *netfs_group);
	ssize_t netfs_unbuffered_write_iter_locked(struct kiocb *iocb, struct iov_iter *iter,
						   struct netfs_group *netfs_group);

```
这些不能直接赋给 `.write_iter`，文件系统负责在调用它们之前执行 inode 锁
前两个函数用于缓冲写；第一个只是添加一些标准写检查并跳转到第二个，但如果文件系统想要自己做检查，它可以直接使用第二个。第三个函数用于非缓冲或 DIO 写
在这三个写函数上，都有一个回写组指针（如果文件系统不使用则为 NULL）。回写组folio 被修改时设置folio 上。如果要修改folio 已经标记了不同的组，则先将其刷出。回API 允许写回特定的组
### 内存映射 I/O API


```

	vm_fault_t netfs_page_mkwrite(struct vm_fault *vmf, struct netfs_group *netfs_group);

```
这使得文件系统可以将 `.page_mkwrite` 委托netfslib。文件系统不应在调用它之前获inode 锁，但与上面的加锁写函数一样，它确实带有一个回写组指针。如果要变为可写的页属于不同的组，则会先将其刷出
### 整体文件 API


还有一组特殊的 API，用于那些内容必须通过单次 RPC 读取（且不写回）、并作为整体块维护的文件
```

	ssize_t netfs_read_single(struct inode *inode, struct file *file, struct iov_iter *iter);
	void netfs_single_mark_inode_dirty(struct inode *inode);
	int netfs_writeback_single(struct address_space *mapping,
				   struct writeback_control *wbc,
				   struct iov_iter *iter);

```
第一个函数从文件读取到给定缓冲区，如果数据已在缓存中则优先从缓存读取；第二个函数允许inode 标记为脏，从而引发后续的回写；第三个函数可由回写代码调用，以将数据写入缓存（如果存在）
如果使用API，inode 应标记为 `NETFS_ICTX_SINGLE_NO_UPLOAD`。回写函数要求缓冲区ITER_FOLIOQ 类型
## 高层 VM API


netfslib 还提供多API 调用，供文件系统VM 操作委托给它。同样地，netfslib 反过来会调用文件系统和缓存来协商 I/O 大小、发RPC 并提供介入位```

	void netfs_readahead(struct readahead_control *);
	int netfs_read_folio(struct file *, struct folio *);
	int netfs_writepages(struct address_space *mapping,
			     struct writeback_control *wbc);
	bool netfs_dirty_folio(struct address_space *mapping, struct folio *folio);
	void netfs_invalidate_folio(struct folio *folio, size_t offset, size_t length);
	bool netfs_release_folio(struct folio *folio, gfp_t gfp);

```
这些`address_space_operations` 方法，可以直接设置在操作表中
### 已废弃的 PG_private_2 API


还有一个用于仍使用已废PG_private_2 标志的文件系统的废弃函数
```

	int netfs_write_begin(struct netfs_inode *inode, struct file *file,
			      struct address_space *mapping, loff_t pos, unsigned int len,
			      struct folio **_folio, void **_fsdata);

```
它使用了已废弃的 PG_private_2 标志，因此不应被使用

## I/O 请求 API


I/O 请求 API 包含若干结构以及文件系统可能需要使用的若干函数
### 请求结构


请求结构管理整个请求，持有一些资```

	struct netfs_io_request {
		enum netfs_io_origin	origin;
		struct inode		*inode;
		struct address_space	*mapping;
		struct netfs_group	*group;
		struct netfs_io_stream	io_streams[];
		void			*netfs_priv;
		void			*netfs_priv2;
		unsigned long long	start;
		unsigned long long	len;
		unsigned long long	i_size;
		unsigned int		debug_id;
		unsigned long		flags;
		...
	};

```
许多字段供内部使用，但此处显示的字段是文件系统感兴趣的：

 - `origin`

   请求的来源（预读、read_folio、DIO 读、回写等）
 - `inode`
 - `mapping`

   被读取文件的 inode 和地址空间。mapping 可能指向也可能不指向 inode->i_data
 - `group`

   此请求正在处理的回写组，NULL。这持有对该组的一个引用
 - `io_streams`

   请求可用的并行子请求流。目前有两个可用，但未来可能做成可扩展的。`NR_IO_STREAMS` 指示该数组的大小
 - `netfs_priv`
 - `netfs_priv2`

   网络文件系统的私有数据。该值可以在调用辅助函数时传入，也可以在请求期间设置
 - `start`
 - `len`

   读请求起始位置的文件偏移和长度。这些可能被 ->expand_readahead() 操作修改
 - `i_size`

   请求开始时文件的大小
 - `debug_id`

   为此操作分配的一个编号，可在 trace 行中显示以供参考
 - `flags`

   用于管理和控制请求操作的标志。其中一些可能引起文件系统的兴趣
   - `NETFS_RREQ_RETRYING`

     netfslib 在生成重试时设置此标志
   - `NETFS_RREQ_PAUSE`

     文件系统可以设置此标志以请求暂停库的分发子请求循环——但需要注意，因为 netfslib 也可能设置它
   - `NETFS_RREQ_NONBLOCK`
   - `NETFS_RREQ_BLOCKED`

     netfslib 设置第一个以指示调用者设置了非阻塞模式，文件系统可以设置第二个以指示它本应阻塞
   - `NETFS_RREQ_USE_PGPRIV2`

     如果文件系统想要使用 PG_private_2 来跟踪某folio 是否正在写入缓存，则可以设置此标志。这已被废弃，因PG_private_2 即将消失
如果文件系统需要比此结构提供的更多的私有数据，则应该封装它并提供自己的分配器
### 流结

一个请求由一个或多个并行流组成，每个流可能面向不同的目标
对于读请求，只使用流 0。它可以包含面向不同来源的、混合的子请求。对于写请求，流 0 用于服务器，1 用于缓存。对于缓冲回写，除非遇到正常的脏 folio，否则流 0 不会启用，此时将调用 ->begin_writeback()，文件系统可以将该流标记为可用```

	struct netfs_io_stream {
		unsigned char		stream_nr;
		bool			avail;
		size_t			sreq_max_len;
		unsigned int		sreq_max_segs;
		unsigned int		submit_extendable_to;
		...
	};

```
文件系统可以访问/使用若干成员
 - `stream_nr`

   请求内流的编号
 - `avail`

   如果流可用则true。文件系统应在流零上、在 ->begin_writeback() 中设置它
 - `sreq_max_len`
 - `sreq_max_segs`

   这些由文件系统或缓存->prepare_read() ->prepare_write() 中为每个子请求设置，以指示该子请求支持的最大字节数，以及可选的最大段数（如果不为 0）
 - `submit_extendable_to`

   在给定可用缓冲区的情况下，子请求可以向上舍入超出 EOF 的大小。这使得缓存能够判断它是否能执行跨越 EOF 标记DIO 读或写
### 子请求结

单个 I/O 单元由子请求结构管理。这```

	struct netfs_io_subrequest {
		struct netfs_io_request *rreq;
		struct iov_iter		io_iter;
		unsigned long long	start;
		size_t			len;
		size_t			transferred;
		unsigned long		flags;
		short			error;
		unsigned short		debug_index;
		unsigned char		stream_nr;
		...
	};

```
每个子请求应当访问单一来源，不过库会处理从一种来源类型回退到另一种来源类型。各成员如下
 - `rreq`

   指向读请求的指针
 - `io_iter`

   一I/O 迭代器，表示要读入或写出的缓冲区片段
 - `start`
 - `len`

   此读请求片段起始位置的文件偏移和长度
 - `transferred`

   此子请求到目前为止已传输的数据量。应在本次分发子请求所完成的传输长度上累加。如果此值小`len`，则子请求可能会被重新分发以继续
 - `flags`

   用于管理子请求的若干标志。文件系统或缓存对其中一些感兴趣
   - `NETFS_SREQ_MADE_PROGRESS`

     由文件系统设置，表示一个或多个字节的数据已被读取或写入
   - `NETFS_SREQ_HIT_EOF`

     如果读命中了文件 EOF，文件系统应设置此标志（在这种情况下 `transferred` 应停EOF 处）。netfslib 可能会将子请求扩展到包含 EOF folio 的大小，以防第三方发生了改变，或DIO 读可能请求了比可用数据更多的数据。库将清除任何多余的 pagecache
   - `NETFS_SREQ_CLEAR_TAIL`

     文件系统可以设置此标志，以指示从 transferred len 的片段剩余部分应被清零。如果设置了 HIT_EOF，请勿设置
   - `NETFS_SREQ_NEED_RETRY`

     文件系统可以设置此标志，以告netfslib 重试该子请求
   - `NETFS_SREQ_BOUNDARY`

     文件系统可以在子请求上设置此标志，以指示它在文件系统结构的边界处结束（例如在一Ceph 对象的末尾）。它告诉 netfslib 不要跨它重新平铺子请求
 - `error`

   供文件系统存储子请求的结果。成功时设为 0，否则设为负的错误码
 - `debug_index`
 - `stream_nr`

   为此片段分配的、可trace 行中显示以供参考的编号，以及它所属的请求流的编号
如有必要，文件系统可以对它正在使用的子请求获取和释放额外的引```

	void netfs_get_subrequest(struct netfs_io_subrequest *subreq,
				  enum netfs_sreq_ref_trace what);
	void netfs_put_subrequest(struct netfs_io_subrequest *subreq,
				  enum netfs_sreq_ref_trace what);

```
使用 netfs trace 码来指示原因。但必须小心，因为一旦子请求的控制权返回netfslib，同一个子请求可能会被重新分发/重试
### 文件系统方法


文件系统`netfs_inode` 中设置一个操作表netfslib
```

	struct netfs_request_ops {
		mempool_t *request_pool;
		mempool_t *subrequest_pool;
		int (*init_request)(struct netfs_io_request *rreq, struct file *file);
		void (*free_request)(struct netfs_io_request *rreq);
		void (*free_subrequest)(struct netfs_io_subrequest *rreq);
		void (*expand_readahead)(struct netfs_io_request *rreq);
		int (*prepare_read)(struct netfs_io_subrequest *subreq);
		void (*issue_read)(struct netfs_io_subrequest *subreq);
		void (*done)(struct netfs_io_request *rreq);
		void (*update_i_size)(struct inode *inode, loff_t i_size);
		void (*post_modify)(struct inode *inode);
		void (*begin_writeback)(struct netfs_io_request *wreq);
		void (*prepare_write)(struct netfs_io_subrequest *subreq);
		void (*issue_write)(struct netfs_io_subrequest *subreq);
		void (*retry_request)(struct netfs_io_request *wreq,
				      struct netfs_io_stream *stream);
		void (*invalidate_cache)(struct netfs_io_request *wreq);
	};

```
该表以一对可选的内存池指针开头，请求和子请求可从中分配。如果未提供，netfslib 有默认的池来替代使用。如果文件系统将自己的更大结构体封装netfs 结构体之外，则需要使用自己的池。netfslib 将直接从池中分配
表中定义的方法有
 - `init_request()`
 - `free_request()`
 - `free_subrequest()`

   [可选] 文件系统可以实现这些方法来初始化或清理其附加到请求或子请求上的任何资源
 - `expand_readahead()`

   [可选] 调用此方法以允许文件系统扩展预读请求的大小。文件系统可以在两个方向上扩展请求，但必须保留初始区域，因为它可能代表已经完成的分配。如果启用了本地缓存，则它率先扩展请求
   扩展通过修改请求结构中的 ->start ->len 来传达。注意，如果进行了任何修改，->len 的增加量至少应与 ->start 的减少量一样多
 - `prepare_read()`

   [可选] 调用此方法以允许文件系统限制子请求的大小。它也可以限制迭代器中单独区域的数量```

	rreq->io_streams[0].sreq_max_len
	rreq->io_streams[0].sreq_max_segs

   文件系统可以利用它，例如，将一个必须跨多个服务器拆分的请求切片，或者将多个读操作同时派发
   成功时返0，否则返回错误码
 * ``issue_read()``

   [必需] netfslib 调用此函数将子请求分派到服务器进行读取。在子请求中>start>len ->transferred 指示应从服务器读取哪些数据，->io_iter 指示要使用的缓冲区
   没有返回值；应调``netfs_read_subreq_terminated()`` 函数来指示子请求已完成（无论哪种结果）>error>transferred ->flags 应在完成前更新。终止可以是异步的
   注意：文件系统不得处理设folio uptodate、解锁它们或丢弃它们的引用——库会处理这些，因为它可能需要将多个子请求的结果拼接起来，这些子请求以各种方式重叠于一folio
 * ``done()``

   [可选] 在读请求中的 folio 全部解锁（并在适用时标记为 uptodate）之后调用
 * ``update_i_size()``

   [可选] 在写路径的各个时机由 netfslib 调用，以请求文件系统更新其对文件大小的认知。如果未提供，netfslib 将设i_size i_blocks 并更新本地缓cookie
 * ``post_modify()``

   [可选] netfslib 写入 pagecache 时，或当它允许一mmap 映射的页被标记为可写时调用
 * ``begin_writeback()``

   [可选] netfslib 在处理回写请求时，如果发现一个不仅仅是标记为 NETFS_FOLIO_COPY_TO_CACHE 的脏页，表明它必须写入服务器，则调用此函数。这使得文件系统只有在知道自己将要执行一次写操作时，才建立回写资源
 * ``prepare_write()``

   [可选] 调用此方法以允许文件系统限制子请求的大小。它也可以限制迭代器中单独区域的数量，例RDMA 所要求的。此信息应设置在子请求所属流:

	rreq->io_streams[subreq->stream_nr].sreq_max_len
	rreq->io_streams[subreq->stream_nr].sreq_max_segs

   文件系统可以利用它，例如，将一个必须跨多个服务器拆分的请求切片，或者将多个写操作同时派发
   不允许返回错误。相反，在失败的情况下，必须调用 ``netfs_prepare_write_failed()``
 * ``issue_write()``

   [必需] 用于将子请求分派到服务器进行写入。在子请求中>start>len ->transferred 指示应写入服务器的数据，->io_iter 指示要使用的缓冲区
   没有返回值；应调``netfs_write_subreq_terminated()`` 函数来指示子请求已完成（无论哪种结果）>error>transferred ->flags 应在完成前更新。终止可以是异步的
   注意：文件系统不得处理清除操作中涉及folio 上的脏或回写标记，也不应对它们获取引用或固定，而应将保留交netfslib
 * ``retry_request()``

   [可选] netfslib 在重试周期开始时调用此函数。这使得文件系统能够检查请求的状态、指定流中的子请求以及其自身数据的状态，并进行调整或重新协商资源
 * ``invalidate_cache()``

   [可选] 当写入本地缓存失败时，netfslib 调用此函数以使存储在本地缓存中的数据失效，提netfs 无法提供的更新一致性数据
```
### 终止子请

当子请求完成时，缓存或子请求可以调用若干函数来通知 netfslib 状态变化。提供一个函数在准备阶段同步地终止一个写子请求：

 - `void netfs_prepare_write_failed(struct netfs_io_subrequest *subreq);`

   指示 ->prepare_write() 调用失败。`error` 字段应已更新
注意>prepare_read() 可以返回错误，因为读可以简单地中止。处理回写失败则更棘手
其他函数用于已经分发到执行阶段的子请求：

 - `void netfs_read_subreq_terminated(struct netfs_io_subrequest *subreq);`

   告诉 netfslib 一个读子请求已终止。`error`、`flags` `transferred` 字段应已更新
 - `void netfs_write_subreq_terminated(void *_op, ssize_t transferred_or_error);`

   告诉 netfslib 一个写子请求已终止。可以传入已处理的数据量或负的错误码。这可以用作 kiocb 完成函数
 - `void netfs_read_subreq_progress(struct netfs_io_subrequest *subreq);`

   提供此函数以可选地netfslib 更新读的增量进度，允许某folio 提前解锁，但实际上并不终止子请求。`transferred` 字段应已更新
### 本地缓存 API


netfslib 提供了一个独立的 API 供本地缓存实现，尽管它提供了一些与文件系统请求 API 相当类似的过程
首先，netfs_io_request 对象包含一个供缓存挂载```

	struct netfs_cache_resources {
		const struct netfs_cache_ops	*ops;
		void				*cache_priv;
		void				*cache_priv2;
		unsigned int			debug_id;
		unsigned int			inval_counter;
	};

```
这包含一个操作表指针和两个私有指针，加上用于追踪fscache cookie 的调ID，以及一个由 `fscache_invalidate()` 调用递增的失效计数器，允许缓存子请求在完成后被失效```

	struct netfs_cache_ops {
		void (*end_operation)(struct netfs_cache_resources *cres);
		void (*expand_readahead)(struct netfs_cache_resources *cres,
					 loff_t *_start, size_t *_len, loff_t i_size);
		enum netfs_io_source (*prepare_read)(struct netfs_io_subrequest *subreq,
						     loff_t i_size);
		int (*read)(struct netfs_cache_resources *cres,
			    loff_t start_pos,
			    struct iov_iter *iter,
			    bool seek_data,
			    netfs_io_terminated_t term_func,
			    void *term_func_priv);
		void (*prepare_write_subreq)(struct netfs_io_subrequest *subreq);
		void (*issue_write)(struct netfs_io_subrequest *subreq);
	};

```
```

	typedef void (*netfs_io_terminated_t)(void *priv,
					      ssize_t transferred_or_error,
					      bool was_async);

```
表中定义的方法有
 - `end_operation()`

   [必需] 在读取请求结束时调用，以清理资源
 - `expand_readahead()`

   [可选] 在预读操作开始时调用，以允许缓存向任一方向扩展请求。这使得缓存能够针对其粒度对请求进行适当的大小调整
 - `prepare_read()`

   [必需] 调用以配置请求的下一个片段。子请求中的 ->start ->len 指示下一个片段的位置和大小；缓存可以将长度减小以匹配其粒度要求
   该函数在其参数中传入指向起始位置和长度的指针，加上文件大小供参考，并适当地调整起始位置和长度。它应返回以下之一
   - `NETFS_FILL_WITH_ZEROES`
   - `NETFS_DOWNLOAD_FROM_SERVER`
   - `NETFS_READ_FROM_CACHE`
   - `NETFS_INVALID_READ`

   以指示该片段应仅被清零，还是应从服务器下载或从缓存读取——或者是否应在当前位置放弃切片
 - `read()`

   [必需] 调用以从缓存读取。给定起始文件偏移量以及一个要读入的迭代器（它也给出长度）。可以给定一个提示，请求从那个起始位置向前查找数据
   还提供了一个指向终止处理函数的指针以及要传递给该函数的私有数据。应以传输的字节数或错误码，加上一个指示终止是否肯定发生在调用者上下文中的标志来调用该终止函数
 - `prepare_write_subreq()`

   [必需] 调用以允许缓存限制子请求的大小。它也可以限制迭代器中单独区域的数量，例DIO/DMA 所要求的。此信息应设置在子请求所属的流上
```

	rreq->io_streams[subreq->stream_nr].sreq_max_len
	rreq->io_streams[subreq->stream_nr].sreq_max_segs

   文件系统可以利用它，例如，将一个必须跨多个服务器拆分的请求切片，或者将多个写操作同时派发
   不允许返回错误。在失败的情况下，必须调``netfs_prepare_write_failed()``
 * ``issue_write()``

   [必需] 用于将子请求分派到缓存进行写入。在子请求中>start>len ->transferred 指示应写入缓存的数据>io_iter 指示要使用的缓冲区
   没有返回值；应调``netfs_write_subreq_terminated()`` 函数来指示子请求已完成（无论哪种结果）>error>transferred ->flags 应在完成前更新。终止可以是异步的

```
## API 函数参