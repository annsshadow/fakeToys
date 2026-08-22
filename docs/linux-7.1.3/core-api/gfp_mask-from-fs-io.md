
## FS/IO 上下文中使用GFP 掩码


:Date: May, 2018
:Author: Michal Hocko <mhocko@kernel.org>

## 简

文件系统（filesystem）和 IO 栈中的代码路径在分配内存时必须小心，以防止由直接内存回收（direct memory reclaim）回调到 FS IO 路径、并阻塞在已持有资源（例如锁——最常见的是用于事务上下文的那些锁）上所引起的递归死锁
传统的避免此死锁问题的方式是，在调用分配器时清除 gfp 掩码中的 __GFP_FS 或相应地 __GFP_IO（注意后者也隐含了清除前者）。GFP_NOFS 或相应地 GFP_NOIO 可用作快捷方式。但事实证明，上述方法已被滥用：受限gfp 掩码以防万一"地使用，而没有更深入的考量，这会导致问题，因为过度使用 GFP_NOFS/GFP_NOIO 可能导致内存过度回收（over-reclaim）或其他内存回收问题
## 鏂?API


4.12 起，我们有了用于 NOFS NOIO 上下文的通用作用域（scope）API：`memalloc_nofs_save`、`memalloc_nofs_restore` 以及相应`memalloc_noio_save`、`memalloc_noio_restore`，它们允许将某个作用域标记为从文件系统或 I/O 角度看的临界区。该作用域内的任何分配都会自动从给定的掩码中去掉 __GFP_FS 或相应的 __GFP_IO，因此没有任何内存分配能够递归回到 FS/IO 中
   :functions: memalloc_nofs_save memalloc_nofs_restore
   :functions: memalloc_noio_save memalloc_noio_restore

FS/IO 代码随后只需在开启任何相对于回收而言的临界区之前——例如与回收上下文共享的锁，或者可能通过回收发生事务上下文嵌套时——调用相应的 save 函数。当临界区结束时应当调用 restore 函数。理想情况下，所有这些都应附带一段解释，说明回收上下文是什么，以便维护
请注意，save/restore 函数的正确配对允许嵌套，因此从已有的 NOIO NOFS 作用域中调用 `memalloc_noio_save` 或相应的 `memalloc_noio_restore` 是安全的
## __vmalloc(GFP_NOFS) 鍛。

v5.17 起，特别是在提交 451769ebb7e79mm/vmalloc: alloc GFP_NO{FS,IO} for vmalloc"）之后，GFP_NOFS/GFP_NOIO 现在已通过隐式使用作用API `[k]vmalloc` 中得到支持
在早期内核中，`vmalloc` 不支GFP_NOFS 语义，因为分配器内部深处有硬编码GFP_KERNEL 分配。这意味着使用 GFP_NOFS/GFP_NOIO 调用 `vmalloc` 几乎总是一bug
在理想情况下，上层应该已经标记了危险上下文，因此无需特别小心，`vmalloc` 应可毫无问题地被调用。有时，如果上下文并不清晰或存在分层违规，那么（v5.17 之前的内核上）推荐的变通方法是由作用域 API 包裹 `vmalloc`，并附上注释说明问题