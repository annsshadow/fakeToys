
## 缓存后端 API


FS-Cache 系统提供了一API，实际的缓存可以通过它提供给 FS-Cache，由后者进而服务于网络文件系统
及其他感兴趣的方

```
	#include <linux/fscache-cache.h>.
```

## 概述


API 的交互在三个层级上进行：缓存（cache）、卷（volume）与数据存储（data storage），每个层级
都有自己cookie 对象类型
	=======================	=======================
	COOKIE			C TYPE
	=======================	=======================
	Cache cookie		struct fscache_cache
	Volume cookie		struct fscache_volume
	Data storage cookie	struct fscache_cookie
	=======================	=======================

Cookie 用于向缓存提供一些文件系统数据、管理状态并在访问期间固定缓存，此外还充API 函数引用点。每cookie 都有一个调ID，它被包含在 tracepoint 中，以便更容易地关联跟踪记录。不请注意，调试 ID 只是从递增计数器中分配出来的，最终会回绕
缓存后端与网络文件系统都可以请求缓存 cookie——如果它们请求了同一个名称，就会得到同一cookie而卷与数cookie 则仅由文件系统按需要创建

## 缓存 Cookie


缓存API 中由缓存 cookie 表示。它们是如下对象

```
	struct fscache_cache {
		void		*cache_priv;
		unsigned int	debug_id;
		char		*name;
		...
	};
```

缓存后端可能会感兴趣的字段有几个。`debug_id` 可用于跟踪中以匹配引用同一缓存的行，`name` 是该
缓存注册时使用的名称。`cache_priv` 成员是缓存上线时由缓存提供的私有数据。其余字段供内部使用

## 注册一个缓

当缓存后端想要让一个缓存上线时，它应当先注
```
	struct fscache_cache *fscache_acquire_cache(const char *name);
```

这会查找并可能创建一个缓cookie。该缓存 cookie 可能已经被某个正在寻找它的网络文件系统创建，
在这种情形下就会使用那个缓存 cookie。如果该缓存 cookie 没有被另一个缓存使用，它将被移preparing（准备中）状态，否则会返busy（忙）
如果成功，缓存后端随后就可以开始搭建缓存。在

```
	void fscache_relinquish_cache(struct fscache_cache *cache);
```

中可以重置并丢弃cookie

## 使缓存上

```
	int fscache_add_cache(struct fscache_cache *cache,
			      const struct fscache_cache_ops *ops,
			      void *cache_priv);
```

这将把缓存操作表指针与缓存私有数据存储进缓存 cookie，并将缓存移active（活动）状态，从允许访问发生

## 灏嗙紦瀛樻挙鍑烘湇鍔。

```
	void fscache_withdraw_cache(struct fscache_cache *cache);
```

这将把缓存移withdrawn（已撤出）状态，以阻止新的缓存级与卷级访问启动，然后等待未完成的缓存访问完成
随后缓存必须遍历它所拥有的数据存储对象，并对每个对象所属的 cookie 调用

```
	void fscache_withdraw_cookie(struct fscache_cookie *cookie);
```

这会将该指定 cookie 安排撤出。它被卸载到一个工作队列上。在

```
	void fscache_wait_for_objects(struct fscache_cache *cache);
```

之后，缓存后端可以撤出所有的

```
	void fscache_withdraw_volume(struct fscache_volume *volume);
```

以告fscache 某个卷已被撤出。它会在返回之前等待该卷上所有未完成的访问完成
当缓存被完全撤出时，应当通过

```
	void fscache_relinquish_cache(struct fscache_cache *cache);
```

通知 fscache，以清除 cookie 中的字段并丢弃调用方对其的引用

## 鍗?Cookie


在一个缓存内部，数据存储对象被组织成逻辑卷
```
	struct fscache_volume {
		struct fscache_cache		*cache;
		void				*cache_priv;
		unsigned int			debug_id;
		char				*key;
		unsigned int			key_hash;
		...
		u8				coherency_len;
		u8				coherency[];
	};
```

这里有一些对缓存后端而言感兴趣的字段
   - `cache` - 鐖剁紦瀛?cookie銆。
   - `cache_priv` - 缓存用来存放私有数据的地方
   - `debug_id` - 用于 tracepoint 日志记录的调ID
   - `key` - 一个可打印字符串，其中不包含任'/' 字符，表示卷的索引键。该键以 NUL 结尾，并
     被填充到 4 字节的倍数
   - `key_hash` - 索引键的哈希。无CPU 架构与字节序如何，它都应当是一样的
   - `coherency` - 一段一致性数据，在卷被绑定到缓存中时应当被检查
   - `coherency_len` - 一致性数据缓冲区中的数据量

## 数据存储 Cookie


一个卷是数据存储对象的逻辑分组，其中每个对象都由一cookie 向网络文件系统表示。Cookie 
```
	struct fscache_cookie {
		struct fscache_volume		*volume;
		void				*cache_priv;
		unsigned long			flags;
		unsigned int			debug_id;
		unsigned int			inval_counter;
		loff_t				object_size;
		u8				advice;
		u32				key_hash;
		u8				key_len;
		u8				aux_len;
		...
	};
```

中表示
cookie 中对缓存后端而言感兴趣的字段有：

   - `volume` - 鐖跺嵎 cookie銆。
   - `cache_priv` - 缓存用来存放私有数据的地方
   - `flags` - 一组位标志，包括：

      - FSCACHE_COOKIE_NO_DATA_TO_READ - 缓存中没有可供读取的数据，因为该 cookie 已被创建	失效
      - FSCACHE_COOKIE_NEEDS_UPDATE - 一致性数据和/或对象大小已被更改，需要提交
      - FSCACHE_COOKIE_LOCAL_WRITE - netfs 的数据已被本地修改，因此缓存对象相对于服务器可能处于
	不一致状态
      - FSCACHE_COOKIE_HAVE_DATA - 如果后端成功将数据存入缓存，则应当设置此标志
      - FSCACHE_COOKIE_RETIRED - cookie 在被放弃时已被失效，缓存数据应当被丢弃
   - `debug_id` - 用于 tracepoint 日志记录的调ID
   - `inval_counter` - 对该 cookie 执行的失效次数
   - `advice` - 关于cookie 将如何被使用的信息
   - `key_hash` - 索引键的哈希。无CPU 架构与字节序如何，它都应当是一样的
   - `key_len` - 索引键的长度
   - `aux_len` - 一致性数据缓冲区的长度
每个 cookie 都有一个索引键，它可以内联存储cookie 中，也可以通过

```
	void *fscache_get_key(struct fscache_cookie *cookie);
```

获取。索引键是一段二进制数据块，其存储会被填充到 4 字节的倍数
每个 cookie 还有一个用于一致性数据的缓冲区。它也可以是内联的，或通过

```
	void *fscache_get_aux(struct fscache_cookie *cookie);
```

获取

## Cookie 记账


数据存储 cookie 会被计数，这用于在所有对象都被销毁之前阻塞缓存撤出完成。以下函
```
	void fscache_count_object(struct fscache_cache *cache);
	void fscache_uncount_object(struct fscache_cache *cache);
	void fscache_wait_for_objects(struct fscache_cache *cache);
```

count 函数记录缓存中一个对象的分配，uncount 函数记录其销毁。警告：uncount 函数返回时，缓存
可能已经被销毁
wait 函数可在撤出过程中使用，以等fscache 完成撤出缓存中的所有对象。当它通过时，将不再有引用
该缓存对象或任何卷对象的剩余对象

## 缓存管理 API


缓存后端通过提供一个操作表来实现缓存管API，fscache 可以利用这些操作来管理缓存的各个方面该表`struct fscache_cache_ops` 表示
```
	struct fscache_cache_ops {
		const char *name;
		...
	};
```

它包含一个供缓存后端驱动打印的名称，以及若干指向方法的指针，fscache 能够请求对缓存的管理
```
	void (*acquire_volume)(struct fscache_volume *volume);
```

     该方法在一个卷 cookie 正在创建时被调用。调用方持有一个缓存级别的访问引脚（access pin），
     以防止缓存在此期间被销毁。该方法应当建立访问缓存中某个卷所需的资源，并且在完成之前不应返回
     如果成功，它可以``cache_priv`` 设为它自己的数据
   * 清理cookie [可选]
```
	void (*free_volume)(struct fscache_volume *volume);
```

     当某个卷 cookie 被释放时，如``cache_priv`` 已被设置，则会调用此方法
   * 在缓存中查找一cookie [强制]
```
	bool (*lookup_cookie)(struct fscache_cookie *cookie);
```

     调用此方法以查找/创建访问某个 cookie 的数据存储所需的资源。它从一个工作线程中调用，并带有
     缓存中的一个卷级访问引脚，以防止该卷被撤出
     成功时应当返true，否则返false。如果返false，则会调withdraw_cookie 操作（见下文）
     如果查找失败，但该对象仍可被创建（例如此前尚未被缓存），则可以调
```
	void fscache_cookie_lookup_negative(
			struct fscache_cookie *cookie);
```

     以让网络文件系统继续运行，并在缓存后端着手创建相关资源的同时开始下载内容
     如果成功，可以设``cookie->cache_priv``
   * 在没有持有任cookie 访问计数地情况下撤出一个对[强制]
```
	void (*withdraw_cookie)(struct fscache_cookie *cookie);
```

     调用此方法以将一cookie 撤出服务。当cookie netfs 放弃、被缓存后端撤出或剔除，或被
     fscache 在非使用一段时间后关闭时，都会调用它
     调用方不持有任何访问引脚，但它从一个不可重入的工作项中被调用，以管理撤出可能发生的各种方式
     之间的竞争
     如果相关联的数据要从缓存中移除，cookie 上会设置 ``FSCACHE_COOKIE_RETIRED`` 标志
   * 改变一个数据存储对象的大小 [强制]
```
	void (*resize_cookie)(struct netfs_cache_resources *cres,
			      loff_t new_size);
```

     调用此方法以告知缓存后端，由于本地截断，netfs 文件的大小发生了变化。缓存后端应当在返回之前
     完成它需要做的所有改动，因为这发生在 netfs inode 互斥锁之下
     调用方持有一cookie 级别的访问引脚，以防止与撤出发生竞争，并netfs 必须已将cookie
     标记为使用中，以防止垃圾回收或剔除移除任何资源
   * 使一个数据存储对象失[强制]
```
	bool (*invalidate_cookie)(struct fscache_cookie *cookie);
```

     当网络文件系统检测到第三方修改，或进行了一次本O_DIRECT 写入时，会调用此方法。它请求缓存
     后端丢弃该对象在缓存中的所有数据并重新开始。成功时应当返回 true，否则返false
     在进入时，新I/O 操作会被阻塞。一旦缓存处于可以再次接I/O 的状态，后端应当通过调用

```
	void fscache_resume_after_invalidation(struct fscache_cookie *cookie);
```

     来释放该阻塞
     如果该方法返false，则会针对此 cookie 撤出缓存
   * 准备对缓存进行本地修[强制]
```
	void (*prepare_to_write)(struct fscache_cookie *cookie);
```

     当网络文件系统发现它将需要因本地写入或截断而修改缓存的内容时，会调用此方法。这给缓存一     机会，记下某个缓存对象相对于服务器可能处于不一致状态，并可能需要在稍后写回。如果未能正     提交，这也可能导致缓存数据在后续重新绑定时被丢弃
   * netfs 库开始一个操[强制]
```
	bool (*begin_operation)(struct netfs_cache_resources *cres,
				enum fscache_want_state want_state);
```

     当正在建立一I/O 操作（读、写或调整大小）时，会调用此方法。调用方持有cookie 上的一     访问引脚，并且必须已将该 cookie 标记为使用中
     如果可以，后端应当把需要保留的任何资源附加netfs_cache_resources 对象上，并返true
     如果它无法完成设置，则应当返false
     want_state 参数指示调用方需要缓存对象处于什么状态，以及它想在该操作期间做什么：

	* ``FSCACHE_WANT_PARAMS`` - 调用方只是想访问缓存对象的参数；它还不需要进行数I/O
	* ``FSCACHE_WANT_READ`` - 调用方想要读取数据
	* ``FSCACHE_WANT_WRITE`` - 调用方想要写入或调整缓存对象的大小
     注意，如cookie 仍在创建中，``cache_priv`` 上未必已经附加了任何内容

## 数据 I/O API


缓存后端通过 netfs 库的 ``struct netfs_cache_ops`` 提供数据 I/O API，该结构由上`begin_operation` 方法附加到一`struct netfs_cache_resources` 上
相关说明请参Documentation/filesystems/netfs_library.rst

## 杂项函数


FS-Cache 提供了一些缓存后端可以使用的实用函数
```
	void fscache_io_error(struct fscache_cache *cache);
```

     这告FS-Cache 缓存中发生了一I/O 错误。这会阻止在该缓存上启动任何新的 I/O
     这并不会实际撤出缓存。那必须单独进行
   * 记录因失败而停止在某个 cookie 上的缓存
```
	void fscache_caching_failed(struct fscache_cookie *cookie);
```

     这记录在某个 cookie 上进行的缓存以某种方式失败了，例如后备存储创建失败或失效失败，并且在
     缓存被重置之前不应在其上进行进一步的 I/O 操作
   * 统计 I/O 请求
```
	void fscache_count_read(void);
	void fscache_count_write(void);
```

     这些记录对缓存的读与写。这些数字显示在 /proc/fs/fscache/stats 中
   * 统计空间不足错误
```
	void fscache_count_no_write_space(void);
	void fscache_count_no_create_space(void);
```

     这些记录缓存中的 ENOSPC 错误，分为数据写入失败与文件系统对象创建失败（例mkdir）
   * 统计被剔除的对象
```
	void fscache_count_culled(void);
```

     这记录一个对象被剔除
   * 从一组缓存资源中取出 cookie
```
	struct fscache_cookie *fscache_cres_cookie(struct netfs_cache_resources *cres)
```

     从缓存资源中拉出一个指cookie 的指针。如果没有设cookie，则可能返回一NULL cookie

## API 函数参