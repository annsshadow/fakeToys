## 使用通用设备的动DMA 映射


:Author: James E.J. Bottomley <James.Bottomley@HansenPartnership.com>

本文档描DMA API。如果你想更平缓地了解该 API（以及实际示例），请参阅
Documentation/core-api/dma-api-howto.rst銆。
API 分为两部分。第一部分描述基础 API。第二部分描述用于支持非一致性内机器的扩展。除非你明确知道你的驱动必须支持非一致性平台（通常只有遗留平台），
否则你只应使用第一部分描述API
### 第一部分 - DMA API

要获DMA API，你必须 #include <linux/dma-mapping.h>。这提供dma_addr_t
以及下面描述的接口
dma_addr_t 可以持有该平台上任何有效DMA 地址。它可以被交给设备，用作 DMA
源或目标。CPU 不能直接引用 dma_addr_t，因为在其物理地址空间DMA 地址空间之间
可能存在转换
### 第一部分 a - 使用大型 DMA 一致性缓冲区

```

	void *
	dma_alloc_coherent(struct device *dev, size_t size,
			   dma_addr_t *dma_handle, gfp_t flag)

```
一致性内存（coherent memory）是指设备或处理器任意一方写入后，处理器或设备都立即读取、而无需担心缓存影响的内存。（不过在告诉设备去读该内存之前，你可能仍需确保刷新处理器的写缓冲区。）

该例程分配一<size> 字节的一致性内存区域
它返回一个指向所分配区域（在处理器虚拟地址空间中）的指针，如果分配失败则返NULL
它还会返回一<dma_handle>，它可以被转换为与总线同宽的无符号整数，并作为该区域的
DMA 地址基地址交给设备
注意：在某些平台上一致性内存可能很昂贵，且最小分配长度可能和一个页一样大，因此你
应尽可能合并对一致性内存的请求。最简单的方法是使dma_pool 调用（见下文）
flag 参数允许调用者指定分配的 `GFP_` 标志（见 kmalloc()）（实现可能会忽略影响返内存位置的标志，GFP_DMA）
```

	void
	dma_free_coherent(struct device *dev, size_t size, void *cpu_addr,
			  dma_addr_t dma_handle)

```
释放先前分配的一致性内存区域。dev、size dma_handle 必须全部与传dma_alloc_coherent() 的一致。cpu_addr 必须dma_alloc_coherent() 返回的虚拟地址
注意，与该分配的兄弟调用不同，此例程只能IRQ 启用时被调用

### 第一部分 b - 使用小型 DMA 一致性缓冲区

要获DMA API 的这一部分，你必须 #include <linux/dmapool.h>

许多驱动需要大量小DMA 一致性内存区域来存放 DMA 描述符或 I/O 缓冲区。与其用
dma_alloc_coherent() 以页或更大的单位分配，你可以使用 DMA 池（pool）。它们的
工作方式很像 struct kmem_cache，只是它们使DMA 一致性分配器，而不__get_free_pages()。此外，它们理解常见的硬件对齐约束，比如队列头需要对齐到 N 字节
边界
   :export:



### 第一部分 c - DMA 寻址限制

DMA 掩码（mask）是该设备可寻址区域的位掩码。换句话说，如果对某一内存区域DMA 地址
应用 DMA 掩码（按位与操作）不会清除地址中的任何位，那么该设备就能对该内存区域执DMA
下面所有设DMA 掩码的函数，如果所请求的掩码无法用于该设备，或者该设备不具备执DMA
的能力，都可能失败
```

	int
	dma_set_mask_and_coherent(struct device *dev, u64 mask)

```
同时更新流式（streaming）和一致性（coherent）DMA 掩码
返回：成功返0，失败返回负的错误码
```

	int
	dma_set_mask(struct device *dev, u64 mask)

```
仅更新流DMA 掩码
返回：成功返0，失败返回负的错误码
```

	int
	dma_set_coherent_mask(struct device *dev, u64 mask)

```
仅更新一致DMA 掩码
返回：成功返0，失败返回负的错误码
```

	u64
	dma_get_required_mask(struct device *dev)

```
API 返回平台为了高效运行所需的掩码。通常这意味着返回的掩码是覆盖全部内存所需最小掩码。检查所需掩码可以让具有可变描述符大小的驱动有机会在必要时使用更小的描述符
请求所需掩码不会改变当前掩码。如果你想利用它，应该调dma_set_mask() 将掩码设返回的值
```

	size_t
	dma_max_mapping_size(struct device *dev);

```
返回该设备映射的最大大小。dma_map_single()、dma_map_page() 等映射函数的 size 参数
不应大于返回值
```

	size_t
	dma_opt_mapping_size(struct device *dev);

```
返回该设备映射的最大最优大小
映射更大的缓冲区在某些场景下可能花费长得多的时间。此外，对于高速、短生命周期的流映射，映射所花费的前期时间可能占整个请求生命周期中相当可观的一部分。因此，如果拆分
更大的请求不会带来明显的性能损失，建议设备驱动将 DMA 流式映射的总长度限制在返回以内
```

	bool
	dma_need_sync(struct device *dev, dma_addr_t dma_addr);

```
如果转移内存所有权需dma_sync_single_for_{device,cpu} 调用，则返回 %true。如可以跳过这些调用，则返回 %false
```

	unsigned long
	dma_get_merge_boundary(struct device *dev);

```
返回 DMA 合并边界。如果设备无法合并任DMA 地址段，该函数返0
### 第一部分 d - 流式 DMA 映射

流式 DMA 允许映射一个已有的缓冲区用DMA 传输，并在完成后解除映射。映射函数不保证
成功，因此必须检查返回值

	特别地，对于设备不可寻址的内存，映射可能会失败，例如它不在设备的 DMA 掩码	连接的总线桥的寻址范围内。流DMA 函数试图克服这样的寻址约束，要么通过使用
	IOMMU（一个将 I/O DMA 地址映射到物理内存地址的设备），要么在配置	[SWIOTLB <swiotlb>](SWIOTLB <swiotlb>) 的内核中，把数据复制到弹跳缓冲区（bounce
	buffer）或从弹跳缓冲区复制出来。然而，这些方法并不总是可用，而且即便可用，也可能
	因为多种原因失败
	简而言之，设备驱动可能需要警惕缓冲区在物理内存中的位置，尤其是当 DMA 掩码小于 32
	位时
```

	dma_addr_t
	dma_map_single(struct device *dev, void *cpu_addr, size_t size,
		       enum dma_data_direction direction)

```
映射一块处理器虚拟内存，使其能被设备访问，并返回该内存DMA 地址
DMA API 对其方向使用强类型枚举：

======================= =============================================
DMA_NONE		无方向（用于调试DMA_TO_DEVICE		数据正从内存发往设备
DMA_FROM_DEVICE		数据正从设备发往内存
DMA_BIDIRECTIONAL	方向未知
======================= =============================================


	连续的内核虚拟空间在物理内存上未必连续。由于此 API 不提供任何分聚集
	（scatter/gather）能力，如果用户试图映射一块物理上不连续的内存，它会失败。因此，
	要由API 映射的内存应当来自能保证其物理上连续的地方（kmalloc）

	内存一致性（coherency）以称为缓存行宽度的粒度运作。为了让API 映射的内存正	工作，被映射区域必须恰好起始于一个缓存行边界、并恰好结束于一个缓存行边界（以防止
	两个分别映射的区域共享同一个缓存行）。由于缓存行大小在编译时可能未知，该 API 不会
	强制这一要求。因此，建议那些不特别去确定运行时缓存行大小的驱动作者，只映射起始和
	结束都在页边界上的虚拟区域（页边界保证也是缓存行边界）
	DMA_TO_DEVICE 同步必须在软件最后一次修改内存区域之后、且在把它交给设备之前完成	一旦使用了这一原语，该原语所覆盖的内存应当被设备视为只读。如果设备可能在任何时刻
	写入它，那它应该DMA_BIDIRECTIONAL（见下文）
	DMA_FROM_DEVICE 同步必须在驱动访问可能被设备改变的数据之前完成。这块内存应当被
	驱动视为只读。如果驱动需要在任何时刻写入它，那它应该DMA_BIDIRECTIONAL（见
	下文）
	DMA_BIDIRECTIONAL 需要特殊处理：它意味着驱动既不确定内存在交给设备之前是否被修改
	过，也不确定设备是否也会修改它。因此，你必须总是同步双向内存两次：一次在把内存交	设备之前（以确保所有内存修改都已从处理器刷新），一次在内存被设备使用之后、数据可	被访问之前（以确保任何处理器缓存行都更新为设备可能已修改的数据）
```

	void
	dma_unmap_single(struct device *dev, dma_addr_t dma_addr, size_t size,
			 enum dma_data_direction direction)

```
解除先前映射的区域。传入的所有参数必须与传入（和由）dma_map_single() 的（返回的）完全一致
```

	dma_addr_t
	dma_map_page(struct device *dev, struct page *page,
		     unsigned long offset, size_t size,
		     enum dma_data_direction direction)

	void
	dma_unmap_page(struct device *dev, dma_addr_t dma_address, size_t size,
		       enum dma_data_direction direction)

```
用于页映射和解除映射API。其他映API 的所有注意事项和警告都适用于这里。此外，
虽然提供<offset> <size> 参数用于做部分页映射，但建议你除非确实知道缓存宽是什么，否则绝不要使用它们
```

	dma_addr_t
	dma_map_resource(struct device *dev, phys_addr_t phys_addr, size_t size,
			 enum dma_data_direction dir, unsigned long attrs)

	void
	dma_unmap_resource(struct device *dev, dma_addr_t addr, size_t size,
			   enum dma_data_direction dir, unsigned long attrs)

```
用于 MMIO 资源映射和解除映射的 API。其他映API 的所有注意事项和警告都适用于这里API 只应用于映射设备 MMIO 资源，不允许映射 RAM
```

	int
	dma_mapping_error(struct device *dev, dma_addr_t dma_addr)

```
在某些情况下 dma_map_single()、dma_map_page() dma_map_resource() 会创建映射失败驱动可以通过dma_mapping_error() 测试返回DMA 地址来检查这些错误。非零返回值意味着
无法创建映射，驱动应当采取适当措施（例如减少当DMA 映射的使用量，或延迟稍后重试）
```

	int
	dma_map_sg(struct device *dev, struct scatterlist *sg,
		   int nents, enum dma_data_direction direction)

```
DMA 映射一个分聚集列表。返回被映射DMA 地址段数量，如果若干连续sglist 条目
被合并（例如通过 IOMMU，或者某些相邻的段刚好碰巧物理上连续），该数量可能小于传入的
<nents>銆。
请注意，sg 一旦被映射就不能再次映射。映射过程允许破sg 中的信息
与其他映射接口一样，dma_map_sg() 可能失败。当它失败时，返0，驱动必须采取适当措施驱动做点什么至关重要，对于块设备来说，中止请求甚至触发 oops 都比什么都不做、进而损文件系统要好
```

	int i, count = dma_map_sg(dev, sglist, nents, direction);
	struct scatterlist *sg;

	for_each_sg(sglist, sg, count, i) {
		hw_address[i] = sg_dma_address(sg);
		hw_len[i] = sg_dma_len(sg);
	}

```
其中 nents sglist 中的条目数量
实现可以自由地将若干连续sglist 条目合并为一个。返回的数量是它实际映射到的 sg 条目
数。失败时返回 0
然后你应该循count 次（注意：这可能少于 nents 次），并在你先前访问 sg->address sg->length 的地方使sg_dma_address() sg_dma_len() 宏，如上所示
```

	void
	dma_unmap_sg(struct device *dev, struct scatterlist *sg,
		     int nents, enum dma_data_direction direction)

```
解除先前映射的分聚集列表。所有参数必须与传入分散/聚集映射 API 的相同
注意nents> 必须是你传入的数量，**不是** 返回DMA 地址条目数量
```

	void
	dma_sync_single_for_cpu(struct device *dev, dma_addr_t dma_handle,
				size_t size,
				enum dma_data_direction direction)

	void
	dma_sync_single_for_device(struct device *dev, dma_addr_t dma_handle,
				   size_t size,
				   enum dma_data_direction direction)

	void
	dma_sync_sg_for_cpu(struct device *dev, struct scatterlist *sg,
			    int nents,
			    enum dma_data_direction direction)

	void
	dma_sync_sg_for_device(struct device *dev, struct scatterlist *sg,
			       int nents,
			       enum dma_data_direction direction)

```
CPU 和设备同步一个连续的或分聚集的映射。对sync_sg API，所有参数必须与传入
sg 映射 API 的相同。对sync_single API，你可以使用与传入单次映API 不完全相同的
dma_handle size 参数，以进行部分同步


   你必须这样做
   - 在读取由设备通过 DMA 写入的值之前（使用 DMA_FROM_DEVICE 方向   - 在写入将通过 DMA 写入设备的值之后（使用 DMA_TO_DEVICE 方向   - 在把内存交给设备**之前和之*，如果内存是 DMA_BIDIRECTIONAL

另见 dma_map_single()
```

	dma_addr_t
	dma_map_single_attrs(struct device *dev, void *cpu_addr, size_t size,
			     enum dma_data_direction dir,
			     unsigned long attrs)

	void
	dma_unmap_single_attrs(struct device *dev, dma_addr_t dma_addr,
			       size_t size, enum dma_data_direction dir,
			       unsigned long attrs)

	int
	dma_map_sg_attrs(struct device *dev, struct scatterlist *sgl,
			 int nents, enum dma_data_direction dir,
			 unsigned long attrs)

	void
	dma_unmap_sg_attrs(struct device *dev, struct scatterlist *sgl,
			   int nents, enum dma_data_direction dir,
			   unsigned long attrs)

```
上面这四个函数与不带 _attrs 后缀的对应函数类似，只是它们传入一个可选的 dma_attrs
DMA 属性的解释是架构相关的，每个属性都应在
Documentation/core-api/dma-attributes.rst 中记录
如果 dma_attrs 0，这些函数中每一个的语义都与不带 _attrs 后缀的对应函数相同。因dma_map_single_attrs() 通常可以替代 dma_map_single() 等
作为使用 `*_attrs` 函数的一个例子，下面是你如何在映射内存时传入属DMA_ATTR_FOO
```

	#include <linux/dma-mapping.h>
	/* DMA_ATTR_FOO 应当定义linux/dma-mapping.h 中，并在
	* Documentation/core-api/dma-attributes.rst 中记*/
	...

		unsigned long attr;
		attr |= DMA_ATTR_FOO;
		....
		n = dma_map_sg_attrs(dev, sg, nents, DMA_TO_DEVICE, attr);
		....

```
关心 DMA_ATTR_FOO 的架构会在它们映射和解除映射的实现中检查它的存```

	void whizco_dma_map_sg_attrs(struct device *dev, dma_addr_t dma_addr,
				     size_t size, enum dma_data_direction dir,
				     unsigned long attrs)
	{
		....
		if (attrs & DMA_ATTR_FOO)
			/* twizzle the frobnozzle */
		....
	}

```
### 第一部分 e - 基于 IOVA DMA 映射

这些 API 在使IOMMU 时允许非常高效的映射。它们是一条可选路径，需要额外的代码，仅
推荐用于 DMA 映射性能、或用于存储 DMA 地址的空间占用很重要的驱动。上一节的所有注事项同样适用于这里
```

    bool dma_iova_try_alloc(struct device *dev, struct dma_iova_state *state,
		phys_addr_t phys, size_t size);

```
用于尝试分配用于映射操作IOVA 空间。如果返false，则API 不能用于给定设备应当使用正常的流DMA 映射 API。`struct dma_iova_state` 由驱动分配，并且必须保留
到解除映射时
```

    static inline bool dma_use_iova(struct dma_iova_state *state)

```
可由驱动用来在调dma_iova_try_alloc 之后检查是否使用了基于 IOVA API。这解除映射路径上可能很有用
```

    int dma_iova_link(struct device *dev, struct dma_iova_state *state,
		phys_addr_t phys, size_t offset, size_t size,
		enum dma_data_direction dir, unsigned long attrs);

```
用于将范围链接到先前分配IOVA。对于给定的 state，除第一次调用外所dma_iova_link
调用的起始地址必须对齐`dma_get_merge_boundary()` 返回DMA 合并边界，并且除最一个范围外的所有范围的大小也必须对齐到 DMA 合并边界
```

    int dma_iova_sync(struct device *dev, struct dma_iova_state *state,
		size_t offset, size_t size);

```
必须被调用，以同步由一个或多个 `dma_iova_link()` 调用所映射IOVA 范围IOMMU
页表
对于使用一次性映射的驱动，所有范围都可以被解除映射，并且通过调用以下函数释放 IOVA
```

   void dma_iova_destroy(struct device *dev, struct dma_iova_state *state,
		size_t mapped_len, enum dma_data_direction dir,
                unsigned long attrs);

```
或者，驱动可以通过解除映射和映射单独的区域来动态管IOVA 空间。在那种情况
```

    void dma_iova_unlink(struct device *dev, struct dma_iova_state *state,
		size_t offset, size_t size, enum dma_data_direction dir,
		unsigned long attrs);

```
用于解除映射先前映射的范围，以及

```

   void dma_iova_free(struct device *dev, struct dma_iova_state *state);

```
用于释放 IOVA 空间。在调用 `dma_iova_free()` 之前，所有区域必须已经用
`dma_iova_unlink()` 解除映射
### 第二部分 - 非一致DMA 分配

这些 API 允许分配保证能被传入设备通过 DMA 寻址的页，但这些页需要由内核与设备显式地
管理内存所有权
如果你不理解缓存行一致性在处理器与 I/O 设备之间如何工作，你不应该使用这部分 API
```

	struct page *
	dma_alloc_pages(struct device *dev, size_t size, dma_addr_t *dma_handle,
			enum dma_data_direction dir, gfp_t gfp)

```
该例程分配一<size> 字节的非一致性内存。它返回指向该区域第一struct page 的指针，
如果分配失败则返NULL。得到的 struct page 可用struct page 适用的一切场合
它还会返回一<dma_handle>，它可以被转换为与总线同宽的无符号整数，并作为该区域的
DMA 地址基地址交给设备
dir 参数指定数据是否被设备读取和/或写入，详见 dma_map_single()
gfp 参数允许调用者指定分配的 `GFP_` 标志（见 kmalloc()），但拒绝用于指定内存区域（GFP_DMA GFP_HIGHMEM）的标志
在把内存交给设备之前，需要调dma_sync_single_for_device()，而在读取由设备写入的内存
之前，需要调dma_sync_single_for_cpu()，就像被复用的流DMA 映射一样
```

	void
	dma_free_pages(struct device *dev, size_t size, struct page *page,
			dma_addr_t dma_handle, enum dma_data_direction dir)

```
释放先前使用 dma_alloc_pages() 分配的内存区域。dev、size、dma_handle dir 必须
全部与传dma_alloc_pages() 的一致。page 必须dma_alloc_pages() 返回的指针
```

	int
	dma_mmap_pages(struct device *dev, struct vm_area_struct *vma,
		       size_t size, struct page *page)

```
dma_alloc_pages() 返回的分配映射到用户地址空间。dev size 必须与传dma_alloc_pages() 的一致。page 必须dma_alloc_pages() 返回的指针
```

	void *
	dma_alloc_noncoherent(struct device *dev, size_t size,
			dma_addr_t *dma_handle, enum dma_data_direction dir,
			gfp_t gfp)

```
该例程是 dma_alloc_pages 的一个便捷包装，返回所分配内存的内核虚拟地址，而不是页结构
```

	void
	dma_free_noncoherent(struct device *dev, size_t size, void *cpu_addr,
			dma_addr_t dma_handle, enum dma_data_direction dir)

```
释放先前使用 dma_alloc_noncoherent() 分配的内存区域。dev、size、dma_handle dir
必须全部与传dma_alloc_noncoherent() 的一致。cpu_addr 必须dma_alloc_noncoherent()
返回的虚拟地址
```

	struct sg_table *
	dma_alloc_noncontiguous(struct device *dev, size_t size,
				enum dma_data_direction dir, gfp_t gfp,
				unsigned long attrs);

```
该例程分<size> 字节的非一致、且可能非连续的内存。它返回一个指struct sg_table 指针，描述已分配并已完成 DMA 映射的内存，如果分配失败则返NULL。得到的内存可用struct page 映射到分散列表所适用的场合
返回sg_table 保证只有一个单一DMA 映射段，sgt->nents 指示，但它可能有多个
CPU 侧段，由 sgt->orig_nents 指示
dir 参数指定数据是否被设备读取和/或写入，详见 dma_map_single()
gfp 参数允许调用者指定分配的 `GFP_` 标志（见 kmalloc()），但拒绝用于指定内存区域（GFP_DMA GFP_HIGHMEM）的标志
attrs 参数必须0 DMA_ATTR_ALLOC_SINGLE_PAGES
在把内存交给设备之前，需要调dma_sync_sgtable_for_device()，而在读取由设备写入的内存
之前，需要调dma_sync_sgtable_for_cpu()，就像被复用的流DMA 映射一样
```

	void
	dma_free_noncontiguous(struct device *dev, size_t size,
			       struct sg_table *sgt,
			       enum dma_data_direction dir)

```
释放先前使用 dma_alloc_noncontiguous() 分配的内存。dev、size dir 必须全部与传dma_alloc_noncontiguous() 的一致。sgt 必须dma_alloc_noncontiguous() 返回的指针
```

	void *
	dma_vmap_noncontiguous(struct device *dev, size_t size,
		struct sg_table *sgt)

```
dma_alloc_noncontiguous() 返回的分配返回一块连续的内核映射。dev size 必须与传dma_alloc_noncontiguous() 的一致。sgt 必须dma_alloc_noncontiguous() 返回的指针
一旦一个非连续分配被此函数映射，就必须使用 flush_kernel_vmap_range() invalidate_kernel_vmap_range() API 来管理内核映射、设备与用户空间映射（如果有）之间的
一致性
```

	void
	dma_vunmap_noncontiguous(struct device *dev, void *vaddr)

```
解除dma_vmap_noncontiguous() 返回的内核映射。dev 必须与传dma_alloc_noncontiguous()
的一致。vaddr 必须dma_vmap_noncontiguous() 返回的指针

```

	int
	dma_mmap_noncontiguous(struct device *dev, struct vm_area_struct *vma,
			       size_t size, struct sg_table *sgt)

```
dma_alloc_noncontiguous() 返回的分配映射到用户地址空间。dev size 必须与传dma_alloc_noncontiguous() 的一致。sgt 必须dma_alloc_noncontiguous() 返回的指针
```

	int
	dma_get_cache_alignment(void)

```
返回处理器缓存对齐。这是你在映射内存或进行部分刷新*必须**遵守的绝对最小对**
宽度

	API 可能返回一个比实际缓存*更大**的数字，但它保证一个或多个缓存行恰	适配到此次调用返回的宽度中。它也将始终2 的幂，便于对齐

### 第三部分 - 调试驱动DMA API 的使
如上所述的 DMA API 有一些约束。例如，DMA 地址必须用相同大小、相应的函数释放。随着
硬件 IOMMU 的出现，驱动不违反这些约束变得越来越重要。在最坏情况下，此类违规可能导数据损坏，直至摧毁文件系统
为了调试驱动并发DMA API 使用中的缺陷，可以把检查代码编译进内核，它会把这类违规
告诉开发者。如果你的架构支持，你可以在内核配置中选择 "Enable debugging of DMA API
usage" 选项。启用此选项会有性能影响。不要在生产内核中启用它
如果你启动，得到的内核将包含一些对账代码，记录为哪个设备分配了哪些 DMA 内存。如这段代码检测到错误，它会将一条带有一些细节的警告消息打印到你的内核日志中。一```

	WARNING: at /data2/repos/linux-2.6-iommu/lib/dma-debug.c:448
		check_unmap+0x203/0x490()
	Hardware name:
	forcedeth 0000:00:08.0: DMA-API: device driver frees DMA memory with wrong
		function [device address=0x00000000640444be] [size=66 bytes] [mapped as
		single] [unmapped as page]
	Modules linked in: nfsd exportfs bridge stp llc r8169
	Pid: 0, comm: swapper Tainted: G        W  2.6.28-dmatest-09289-g8bb99c0 #1
	Call Trace:
	<IRQ>  [<ffffffff80240b22>] warn_slowpath+0xf2/0x130
	[<ffffffff80647b70>] _spin_unlock+0x10/0x30
	[<ffffffff80537e75>] usb_hcd_link_urb_to_ep+0x75/0xc0
	[<ffffffff80647c22>] _spin_unlock_irqrestore+0x12/0x40
	[<ffffffff8055347f>] ohci_urb_enqueue+0x19f/0x7c0
	[<ffffffff80252f96>] queue_work+0x56/0x60
	[<ffffffff80237e10>] enqueue_task_fair+0x20/0x50
	[<ffffffff80539279>] usb_hcd_submit_urb+0x379/0xbc0
	[<ffffffff803b78c3>] cpumask_next_and+0x23/0x40
	[<ffffffff80235177>] find_busiest_group+0x207/0x8a0
	[<ffffffff8064784f>] _spin_lock_irqsave+0x1f/0x50
	[<ffffffff803c7ea3>] check_unmap+0x203/0x490
	[<ffffffff803c8259>] debug_dma_unmap_phys+0x49/0x50
	[<ffffffff80485f26>] nv_tx_done_optimized+0xc6/0x2c0
	[<ffffffff80486c13>] nv_nic_irq_optimized+0x73/0x2b0
	[<ffffffff8026df84>] handle_IRQ_event+0x34/0x70
	[<ffffffff8026ffe9>] handle_edge_irq+0xc9/0x150
	[<ffffffff8020e3ab>] do_IRQ+0xcb/0x1c0
	[<ffffffff8020c093>] ret_from_intr+0x0/0xa
	<EOI> <4>---[ end trace f6435a98e2a38c0e ]---

```
驱动开发者可以找到驱动和设备，包括导致此警告DMA API 调用的栈回溯
默认情况下，只有第一个错误会导致警告消息。所有其他错误只会默默计数。存在这一限制是为防止代码淹没你的内核日志。为了支持调试设备驱动，可以通过 debugfs 禁用它。详见下文的
debugfs 接口文档
用于 DMA API 调试代码debugfs 目录名为 dma-api/。在该目录下，目前可以找到以下文件：

=============================== ===============================================
dma-api/all_errors		此文件含一个数值。如果该值不0，调试代码会				它发现的每个错误向内核日志打印一条警告。小心使用此
				选项，因为它很容易淹没你的日志
dma-api/disabled		此只读文件在调试代码被禁用时包含字符 'Y'。这可能
				发生在它耗尽内存时，或在启动时就被禁用时
dma-api/dump			此只读文件包含当前的 DMA 映射
dma-api/error_count		此文件是只读的，显示发现的错误总数
dma-api/num_errors		此文件中的数字显示在停止之前会向内核日志打印多少				警告。该数字在系统启动时被初始化1，并可通过写入
				此文件来设置
dma-api/min_free_entries	此只读文件可读出分配器曾见过的最少空				dma_debug_entries 数量。如果该值降0，代码会尝试
				增加 nr_total_entries 来补偿
dma-api/num_free_entries	分配器中当前的空dma_debug_entries 数量
dma-api/nr_total_entries	分配器中 dma_debug_entries 的总数，包括空闲和已用
dma-api/driver_filter		你可以把一个驱动的名字写入此文件，将调试输出限制为
				来自那个特定驱动的请求。向该文件写入空字符串以禁用
				过滤器并再次看到所有错误=============================== ===============================================

如果你把这段代码编译进了内核，它将默认被启用。如果你想无论如何都不带对账启动，可提供 'dma_debug=off' 作为启动参数。这会禁DMA API 调试。注意你无法在运行时再次启用它你必须重启才能做到
如果你只想看到某个特定设备驱动的调试消息，可以指dma_debug_driver=<drivername> 参数这会在启动时启用驱动过滤器。此后调试代码只会打印该驱动的错误。此过滤器稍后可以使debugfs 禁用或更改
当代码在运行时禁用自身时，最可能是因为它耗尽dma_debug_entries，并且无法按需分配更多启动时预分配65536 个条目——如果这对你太低，请'dma_debug_entries=<你期望的数字>'
启动以覆盖默认值。注意代码是批量分配条目的，因此预分配条目的确切数量可能大于实际请求数量。每当代码动态分配的条目数达到最初预分配的数量时，它会向内核日志打印一条消息。这为了表明可能需要更大的预分配大小，或者如果这种情况持续发生，则表明某个驱动可能正在泄映射
```

	void
	debug_dma_mapping_error(struct device *dev, dma_addr_t dma_addr);

```
dma-debug 接口 debug_dma_mapping_error() 用于调试那些未能检dma_map_single() dma_map_page() 接口返回地址DMA 映射错误的驱动。该接口清除debug_dma_map_phys()
设置的一个标志，以表明驱动已经调用了 dma_mapping_error()。当驱动解除映射时，
debug_dma_unmap() 检查该标志，如果它仍被设置，则打印一条包含通往解除映射处的调用栈的
警告消息。此接口可以dma_mapping_error() 例程中调用，以启DMA 映射错误检查调试
## 函数与结构体


