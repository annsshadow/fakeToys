## dm-io


dm-io 提供同步与异I/O 服务。共有三I/O 服务，每类都有同步和异步两个版本
用户必须建立一io_region 结构体来描述期望I/O 位置。每io_region 表示一块设备以及起始位```

   struct io_region {
      struct block_device *bdev;
      sector_t sector;
      sector_t count;
   };

```
dm-io 可以从一io_region 读取，或写入一个或多个 io_region。对多个区域的写入由
io_region 结构体数组指定
第一I/O 服务将一组内存页作为 I/O 的数据缓冲区
```

   struct page_list {
      struct page_list *next;
      struct page *page;
   };

   int dm_io_sync(unsigned int num_regions, struct io_region *where, int rw,
                  struct page_list *pl, unsigned int offset,
                  unsigned long *error_bits);
   int dm_io_async(unsigned int num_regions, struct io_region *where, int rw,
                   struct page_list *pl, unsigned int offset,
                   io_notify_fn fn, void *context);

```
第二I/O 服务将一bio 向量数组作为 I/O 的数据缓冲区。如果调用方已经预先组装一bio，该服务会非常方```

   int dm_io_sync_bvec(unsigned int num_regions, struct io_region *where,
                       int rw, struct bio_vec *bvec,
                       unsigned long *error_bits);
   int dm_io_async_bvec(unsigned int num_regions, struct io_region *where,
                        int rw, struct bio_vec *bvec,
                        io_notify_fn fn, void *context);

```
第三I/O 服务将一个指vmalloc 分配的内存缓冲区的指针作I/O 的数据缓冲区如果调用方需要对一个大区域执行 I/O，但又不想分配大量独立的
```

   int dm_io_sync_vm(unsigned int num_regions, struct io_region *where, int rw,
                     void *data, unsigned long *error_bits);
   int dm_io_async_vm(unsigned int num_regions, struct io_region *where, int rw,
                      void *data, io_notify_fn fn, void *context);

```
异步 I/O 服务的调用方必须包含一个完成回调的名称
```

   typedef void (*io_notify_fn)(unsigned long error, void *context);

```
此回调中"error" 参数，以及所有同步版本中`*error` 参数，都是一个位集（而非
简单的错误值）。在写入多个区域的写 I/O 情况下，该位集使 dm-io 能够指示每个单独
区域的成功或失败
在使用任dm-io 服务之前，用户应调用 dm_io_get() 并指定其期望并发执行 I/O 页数。dm-io 会尝试调整其内存池的大小，以确保始终有足够的页可用，从而在执行 I/O
时避免不必要的等待
当用户使用完 dm-io 服务后，应调dm_io_put() 并指定与 dm_io_get() 调用时相同的
页数
