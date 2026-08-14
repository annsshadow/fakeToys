
## 网络文件系统缓存 API

fscache 提供一个 API，网络文件系统可借此利用本地缓存设施。该 API 围绕若干原则组织：

 (1) 一个缓存从逻辑上被组织为若干卷（volume）以及这些卷内的数据存储对象。

 (2) 卷和数据存储对象由各种类型的 cookie 表示。

 (3) cookie 拥有将其与同类区分开来的键（key）。

 (4) cookie 拥有一致性数据，使缓存能够判断缓存的数据是否仍然有效。

 (5) 在可能的情况下，I/O 是异步进行的。

```

	#include <linux/fscache.h>.

```

	 (1) 概述
	 (2) 卷注册
	 (3) 数据文件注册
	 (4) 声明一个 cookie 在使用中
	 (5) 调整数据文件大小（截断）
	 (6) 数据 I/O API
	 (7) 数据文件一致性
	 (8) 数据文件失效
	 (9) 回写资源管理
	(10) 本地修改的缓存
	(11) 页释放与失效


## 概述

从网络文件系统的角度看，fscache 的层级分为两个级别组织。上级代表“卷”，下级代表“数据存储对象”。
它们由两类 cookie 表示，以下分别称为“卷 cookie”和“cookie”。

网络文件系统使用卷键（volume key）为某个卷获取一个卷 cookie，卷键代表定义该卷的全部信息
（例如 cell 名称或服务器地址、卷 ID 或共享名）。它必须被渲染为一个可用作目录名的可打印字符串
（即不含 '/' 字符，且不应以 '.' 开头）。最大名称长度比文件名分量的最大大小小一（为缓存后端留出一个
字符供其自用）。

一个文件系统通常会为每个超级块拥有一个卷 cookie。

随后，文件系统使用该卷内的对象键（object key）为该卷中的每个文件获取一个 cookie。对象键是二进制
blob，只需要在其父卷内唯一。缓存后端负责将二进制 blob 渲染为它可用的形式，并可能采用哈希表、树或
任何其它结构来提升其查找对象的能力。这对网络文件系统是透明的。

一个文件系统通常会为每个 inode 拥有一个 cookie，并在 iget 中获取它，在驱逐该 cookie 时 relinquish。

一旦拥有 cookie，文件系统需要将 cookie 标记为在使用中。这会导致 fscache 在后台派缓存后端去查找/创建
该 cookie 的资源，检查其一致性，并在必要时将该对象标记为处于修改中。

文件系统通常会在其文件打开例程中“使用”该 cookie，并在文件释放时取消使用，并且它需要在对 cookie
进行本地截断的调用前后使用该 cookie。它**还**需要在 pagecache 变脏时使用该 cookie，并在回写完成时
取消使用。这有些棘手，但我们为此做了相应安排。

在对 cookie 执行读、写或调整大小操作时，文件系统必须首先开始一个操作。这会将资源复制到一个持有结构
（holding struct）中，并对缓存加额外的 pin，以阻止缓存撤销拆毁正在使用的结构。随后可以发起实际操作，
并在完成时检测到冲突的失效。

文件系统应当使用 netfslib 来访问缓存，但这并非真正强制，它也可以直接使用 fscache I/O API。


## 卷注册

网络文件系统的第一步是为卷获取一个卷 cookie：

```

	struct fscache_volume *
	fscache_acquire_volume(const char *volume_key,
			       const char *cache_name,
			       const void *coherency_data,
			       size_t coherency_len);

```
此函数创建一个以指定卷键作为名称的卷 cookie，并记录一致性数据。

卷键必须是可打印字符串，且其中不含 '/' 字符。它应以文件系统的名称开头，且长度不超过 254 个字符。
它应当唯一地代表该卷，并将与缓存中存储的内容进行匹配。

调用者还可以指定要使用的缓存的名称。如果指定了，fscache 将查找或创建一个具有该名称的缓存 cookie，
并在该名称的缓存上线时使用它。如果未指定缓存名称，它将使用手边第一个缓存，并将名称设为该缓存。

指定的一致性数据存储在 cookie 中，并将与磁盘上存储的一致性数据匹配。如果没有提供数据，数据指针可以是
NULL。如果一致性数据不匹配，整个缓存卷将被失效。

此函数可能返回诸如 EBUSY（如果卷键已被一个已获取的卷使用）或 ENOMEM（如果发生分配失败）之类的错误。
如果 fscache 未启用，它也可能返回 NULL 卷 cookie。将 NULL cookie 传递给任何接受卷 cookie 的函数是安全的，
这将导致该函数什么都不做。


当网络文件系统用完一个卷时，它应当 relinquish 它：

```

	void fscache_relinquish_volume(struct fscache_volume *volume,
				       const void *coherency_data,
				       bool invalidate);

```
这将导致该卷被提交或移除，并且如果被 seal，一致性数据将被设为提供的值。一致性数据的大小必须与获取该卷时
指定的长度匹配。注意，在该卷被 relinquish 之前，必须 relinquish 在该卷中获取的所有数据 cookie。


## 数据文件注册

一旦拥有了卷 cookie，网络文件系统就可以用它来获取一个 cookie：

```

	struct fscache_cookie *
	fscache_acquire_cookie(struct fscache_volume *volume,
			       u8 advice,
			       const void *index_key,
			       size_t index_key_len,
			       const void *aux_data,
			       size_t aux_data_len,
			       loff_t object_size)

```
这使用指定的索引键在卷中创建 cookie。索引键是给定长度的二进制 blob，且对该卷必须唯一。它被保存到 cookie 中。
其内容没有限制，但其长度不应超过最大文件名长度的大约四分之三，以便进行编码。

调用者还应传入一段位于 aux_data 中的一致性数据。将分配一个大小为 aux_data_len 的缓冲区并复制一致性数据。
假定其大小随时间不变。一致性数据用于检查缓存中数据的有效性。提供了可更新一致性数据的函数。

还应提供被缓存对象的大小。这可能用于裁剪数据，并将与一致性数据一同存储。

此函数从不返回错误，尽管它可能在分配失败或 fscache 未启用时返回 NULL cookie。传入 NULL 卷 cookie 并将返回的
NULL cookie 传递给任何接受它的函数都是安全的。这将导致该函数什么都不做。


当网络文件系统用完一个 cookie 时，它应当 relinquish 它：

```

	void fscache_relinquish_cookie(struct fscache_cookie *cookie,
				       bool retire);

```
这将导致 fscache 提交或删除支撑该 cookie 的存储。


## 标记一个 Cookie 在使用中

一旦网络文件系统获取了 cookie，文件系统应在它打算使用该 cookie 时（通常在文件打开时）告知 fscache：

```

	void fscache_use_cookie(struct fscache_cookie *cookie,
				bool will_modify);
	void fscache_unuse_cookie(struct fscache_cookie *cookie,
				  const void *aux_data,
				  const loff_t *object_size);

```
**use** 函数告知 fscache 它将使用该 cookie，并额外指示用户是否打算在本地修改内容。如果尚未完成，这将触发
缓存后端去收集它访问/存储缓存中数据所需的资源。这是在后台完成的，因此在函数返回时可能尚未完成。

**unuse** 函数指示文件系统已用完一个 cookie。它可选地更新存储的一致性数据和对象大小，然后递减使用中计数。
当最后一个用户取消使用该 cookie 时，它将被安排进行垃圾回收。如果在短时间内未被复用，资源将被释放以减少系统
资源消耗。

在能够访问 cookie 进行读、写或调整大小之前，必须将该 cookie 标记为在使用中——并且在 pagecache 中存在脏数据期间
必须保持在使用中标记，以避免在进程退出期间尝试打开文件而导致 oops。

注意，使用中标记是累积的。每将 cookie 标记为在使用中一次，就必须取消使用一次。


## 调整数据文件大小（截断）

如果网络文件系统文件通过截断在本地被调整大小，则使用以下函数：

```

	void fscache_resize_cookie(struct fscache_cookie *cookie,
				   loff_t new_size);

```
调用者必须首先将该 cookie 标记为在使用中。cookie 和新的大小被传入，缓存被同步地调整大小。这预期在 inode 锁下
从 `->setattr()` inode 操作中调用。


## 数据 I/O API

要直接通过一个 cookie 执行数据 I/O 操作，使用以下函数：

```

	int fscache_begin_read_operation(struct netfs_cache_resources *cres,
					 struct fscache_cookie *cookie);
	int fscache_read(struct netfs_cache_resources *cres,
			 loff_t start_pos,
			 struct iov_iter *iter,
			 enum netfs_read_from_hole read_hole,
			 netfs_io_terminated_t term_func,
			 void *term_func_priv);
	int fscache_write(struct netfs_cache_resources *cres,
			  loff_t start_pos,
			  struct iov_iter *iter,
			  netfs_io_terminated_t term_func,
			  void *term_func_priv);

```
**begin** 函数设置一个操作，将访问缓存所需资源附到 cookie 的缓存资源块上。假设它未返回错误（例如，如果给定 NULL
cookie 它将返回 -ENOBUFS，否则什么都不做），那么可以发起另外两个函数之一。

**read** 和 **write** 函数发起一个直接 I/O（direct-IO）操作。两者都接受先前设置好的缓存资源块、起始文件位置的指示，
以及一个描述缓冲区并指明数据量的 I/O 迭代器。

read 函数还接受一个参数，指示它应如何处理磁盘内容中部分填充的区域（空洞，hole）。这可以是忽略它、跳过初始空洞
并在缓冲区中填入零，或者给出错误。

read 和 write 函数可以给定一个可选的终止函数：

```

	typedef
	void (*netfs_io_terminated_t)(void *priv, ssize_t transferred_or_error,
				      bool was_async);

```
如果给定了终止函数，操作将异步运行，并在完成时调用终止函数。如果未给定，操作将同步运行。注意，在异步情况下，
操作有可能在函数返回之前就已完成。

read 和 write 函数都会在完成时结束操作，detach 任何被 pin 的资源。

如果操作进行期间发生了失效，read 操作将以 ESTALE 失败。


## 数据文件一致性

要请求更新 cookie 上的一致性数据和文件大小，使用：

```

	void fscache_update_cookie(struct fscache_cookie *cookie,
				   const void *aux_data,
				   const loff_t *object_size);

```
这将更新 cookie 的一致性数据和/或文件大小。


## 数据文件失效

有时有必要使包含数据的对象失效。通常，当服务器通知网络文件系统发生了远程第三方更改时这是必要的——此时文件
系统必须丢弃它为该文件持有的状态和缓存数据，并从服务器重新加载。

要指示一个缓存对象应当失效，应使用以下函数：

```

	void fscache_invalidate(struct fscache_cookie *cookie,
				const void *aux_data,
				loff_t size,
				unsigned int flags);

```
这会增加 cookie 中的失效计数器，导致未完成的读操作以 -ESTALE 失败，从提供的信息设置一致性数据和文件大小，阻止
对该 cookie 的新 I/O，并派缓存去清除旧数据。

失效在一个工作线程中异步运行，以免阻塞过多。


## 回写资源管理

要将数据从网络文件系统回写写入缓存，所需的缓存资源需要在修改发生之时（例如当页被标记为脏时）被 pin 住，因为在
正在退出的线程中无法打开文件。

提供了以下设施来管理这一点：

 - 提供了一个 inode 标志 `I_PINNING_FSCACHE_WB`，用于指示该 inode 的 cookie 上持有一个使用中标记。
   只有在持有 inode 锁时才能更改它。

 - 一个标志 `unpinned_fscache_wb` 被放入 `writeback_control` 结构，当 `__writeback_single_inode()`
   因为所有脏页都已清除而清除 `I_PINNING_FSCACHE_WB` 时设置它。

```

	bool fscache_dirty_folio(struct address_space *mapping,
				 struct folio *folio,
				 struct fscache_cookie *cookie);
	void fscache_unpin_writeback(struct writeback_control *wbc,
				     struct fscache_cookie *cookie);
	void fscache_clear_inode_writeback(struct fscache_cookie *cookie,
					   struct inode *inode,
					   const void *aux);

```
**set** 函数旨在从文件系统的 `dirty_folio` 地址空间操作调用。如果 `I_PINNING_FSCACHE_WB` 未设置，它设置该标志
并递增 cookie 的使用计数（调用者必须已经调用过 `fscache_use_cookie()`）。

**unpin** 函数旨在从文件系统的 `write_inode` 超级块操作调用。如果在 writeback_control 结构中设置了
unpinned_fscache_wb，它通过取消使用该 cookie 来进行写后的清理。

**clear** 函数旨在从 netfs 的 `evict_inode` 超级块操作调用。它必须在 `truncate_inode_pages_final()`
**之后**、但在 `clear_inode()` **之前**调用。这会清理任何悬挂的 `I_PINNING_FSCACHE_WB`。它也允许更新一致性数据。


## 本地修改的缓存

如果网络文件系统有想要写入缓存的本地修改数据，它需要标记这些页以指示写操作正在进行，并且如果标记已经存在，
它需要先等待其被移除（大概是由于已经在进行中的操作）。这防止了对缓存中同一存储的多个互相竞争的 DIO 写。

首先，netfs 应通过以下方式确定缓存是否可用：

```

	bool caching = fscache_cookie_enabled(cookie);

```
如果要尝试缓存，应等待页，然后用以下方式标记：

```

	void set_page_fscache(struct page *page);
	void wait_on_page_fscache(struct page *page);
	int wait_on_page_fscache_killable(struct page *page);

```
一旦跨度内的所有页都被标记，netfs 就可以请求 fscache：

```

	void fscache_write_to_cache(struct fscache_cookie *cookie,
				    struct address_space *mapping,
				    loff_t start, size_t len, loff_t i_size,
				    netfs_io_terminated_t term_func,
				    void *term_func_priv,
				    bool caching)

```
如果在到达该点之前发生错误，可以移除标记：

```

	void fscache_clear_page_bits(struct address_space *mapping,
				     loff_t start, size_t len,
				     bool caching)

```
在这些函数中，传入指向源页所附加映射的指针，start 和 len 指示将要写入的区域大小（它不一定需要对齐到页边界，
但必须对后端文件系统中的 DIO 边界对齐）。caching 参数指示是否应跳过缓存，若为 false，这些函数什么都不做。

write 函数接受一些附加参数：代表要写入的缓存对象的 cookie、i_size 指示 netfs 文件的大小，term_func 指示一个
可选的完成函数，term_func_priv 将与错误或写入量一同传给它。

注意，write 函数将始终异步运行，并在完成时调用 term_func 之前取消对所有页的标记。


## 页释放与失效

fscache 跟踪我们刚刚创建的缓存对象在缓存中是否已有任何数据。它知道在写完且写入所来源的页被 VM 释放之前无需进行
任何读取，在那之后它**必须**去缓存中查找。

要告知 fscache 一个页现在可能已在缓存中，使用以下函数：

```

	void fscache_note_page_release(struct fscache_cookie *cookie);

```
如果页已被释放（即 release_folio 返回 true）。

页释放和页失效也应等待留在页上的任何标记：

```

	void wait_on_page_fscache(struct page *page);
	int wait_on_page_fscache_killable(struct page *page);


```
## API 函数参考
