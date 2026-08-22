
## ID 分配


:Author: Matthew Wilcox

## 概述


一个常见的问题是分配标识符（ID）；通常是用来标识某个事物的一小段数字。例子包括文件描述符、进ID、网络协议中的数据包标识符、SCSI 标签以及设备实例号。IDR IDA 为该问题提供了合理的解决方案，以避免每个人都自己发明一套。IDR 提供了将 ID 映射到指针的能力，IDA 仅提ID 分配，因此内存效率要高得多
IDR 接口已被弃用；请改用 [XArray <xarray>](XArray <xarray>)
## IDR 用法


首先初始化一IDR，对于静态分配的 IDR 使用 DEFINE_IDR()，对于动态分配的 IDR 使用 idr_init()
你可以调idr_alloc() 来分配一个未使用ID。通过调用 idr_find() 查找与该 ID 关联的指针，并通过调用 idr_remove() 释放ID
如果你需要更改与某个 ID 关联的指针，可以调用 idr_replace()。这样做的一个常见原因，是向分配函数传入一`NULL` 指针来预留一ID；用预留ID 初始化对象，最后将初始化好的对象插IDR
有些用户需要分配大`INT_MAX` ID。到目前为止，所有这些用户都满足`UINT_MAX` 的限制，他们使用 idr_alloc_u32()。如果你需要无法放u32 ID，我们会与你合作以满足你的需求
如果你需要顺序分ID，可以使idr_alloc_cyclic()。IDR 在处理较ID 时效率会降低，因此使用该函数要付出轻微的代价
要对 IDR 使用的所有指针执行某项操作，你可以使用基于回调的 idr_for_each()，或迭代器风格的 idr_for_each_entry()。你可能需要使idr_for_each_entry_continue() 来继续一次迭代。如果迭代器不满足你的需求，也可以使idr_get_next()
当你使用完一IDR 后，可以调用 idr_destroy() 释放 IDR 使用的内存。这不会释放 IDR 所指向的对象；如果你想这样做，请使用某个迭代器来完成
你可以使idr_is_empty() 来查明当前是否分配了任何 ID
如果你需要在IDR 分配ID 时加锁，可能需要传入一组受限的 GFP 标志，这可能导致 IDR 无法分配内存。为规避此问题，你可以在加锁前调idr_preload()，并在分配后调用 idr_preload_end()
   :doc: idr sync

## IDA 用法


   :doc: IDA description

## 函数与结构体


   :functions:
   :functions:
