## kcopyd


kcopyd 提供将一段扇区范围从一个块设备复制到一个或多个其他块设备的能力，并带有异步完成通知。它dm-snapshot dm-mirror 使用
kcopyd 的用户必须先创建一个客户端，并指明要为其复制作业预留多少内存页。这是通过调用
```

   int kcopyd_client_create(unsigned int num_pages,
                            struct kcopyd_client **result);

```
来完成的
要启动一个复制作业，用户必须设置 io_region 结构体来描述复制的源和目的地。每io_region 表示一个块设备以及该区域的起始扇区和大小。复制的源以一io_region 结构体给出，目的地以
```

   struct io_region {
      struct block_device *bdev;
      sector_t sector;
      sector_t count;
   };

```
给出
要启动复制，用户调用 kcopyd_copy()，传入客户端指针、指向源和目io_region 的指针、名```

   int kcopyd_copy(struct kcopyd_client *kc, struct io_region *from,
                   unsigned int num_dests, struct io_region *dests,
                   unsigned int flags, kcopyd_notify_fn fn, void *context);

   typedef void (*kcopyd_notify_fn)(int read_err, unsigned int write_err,
				    void *context);

```
当复制完成时，kcopyd 将调用用户的完成例程，传回用户的 context 指针。它还会指示复制过程中是否发生了读或写错误
当用户完成所有复制作业后，应调用 kcopyd_client_destroy() 来删kcopyd 客户端，这将释放
```

   void kcopyd_client_destroy(struct kcopyd_client *kc);

```
