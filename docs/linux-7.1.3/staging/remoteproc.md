## 远程处理器框架（Remote Processor Framework

## 简

现代 SoC 通常在非对称多处理（AMP）配置中包含异构的远程处理器设备，这些设备可能运行不同的操作系统实例，无论是 Linux 还是任何其它风格的实时操作系统
例如，OMAP4 拥有双核 Cortex-A9、双Cortex-M3 以及一C64x+ DSP。在典型配置中，双核 Cortex-A9 SMP 配置运行 Linux，而其它三个核心（两个 M3 核心和一DSP）各自以 AMP 配置运行自己RTOS 实例
remoteproc 框架允许不同的平架构控制（上电、加载固件、断电）这些远程处理器，同时抽象掉硬件差异，因此无需重复编写整个驱动。此外，该框架还会为支持这种通信方式的远程处理器添加 rpmsg virtio 设备。这样，特定于平台的 remoteproc 驱动只需要提供少量底层处理函数，然后所rpmsg 驱动就都能正常工作（关于基于 virtio rpmsg 总线及其驱动的更多信息，请参Documentation/staging/rpmsg.rst）。现在也可以注册其它类型virtio 设备。固件只需公布它们支持哪种 virtio 设备，然remoteproc 就会添加这些设备。这使得以最小的开发成本，将现有的 virtio 驱动与远程处理器后端复用成为可能

## 用户 API


```
  int rproc_boot(struct rproc *rproc)

```
启动一个远程处理器（即加载其固件、为其上电……）
如果该远程处理器已经上电，该函数会立即（成功）返回
成功时返0，否则返回相应的错误值。注意：要使用该函数，你应该已经拥有一个有效的 rproc 句柄。有几种干净的方式可以获得它（devres、pdata、remoteproc_rpmsg.c 的做法，或者如果这种方式变得普遍，我们也可能会考虑使用 dev_archdata）
```
  int rproc_shutdown(struct rproc *rproc)

```
关闭一个远程处理器（先前由 rproc_boot() 启动）。如@rproc 仍被其它用户使用，那么该函数只会递减电源引用计数并退出，而不会真正给设备断电
成功时返0，否则返回相应的错误值。每一次对 rproc_boot() 的调用都必须（最终）伴随一次对 rproc_shutdown() 的调用。冗余地调用 rproc_shutdown() 是一bug

```
  we're not decrementing the rproc's refcount, only the power refcount.
  which means that the @rproc handle stays valid even after
  rproc_shutdown() returns, and users can still use it with a subsequent
  rproc_boot(), if needed.

```
```
  struct rproc *rproc_get_by_phandle(phandle phandle)

```
使用设备phandle 查找一rproc 句柄。成功时返回 rproc 句柄，失败时返回 NULL。该函数会递增远程处理器的引用计数，因此当不再需rproc 时，务必使用 rproc_put() 将其递减回去

## 典型用法


```
  #include <linux/remoteproc.h>

  /* in case we were given a valid 'rproc' handle */
  int dummy_rproc_example(struct rproc *my_rproc)
  {
	int ret;

	/* let's power on and boot our remote processor */
	ret = rproc_boot(my_rproc);
	if (ret) {
		/*
		 * something went wrong. handle it and leave.
		 */
	}

	/*
	 * our remote processor is now powered on... give it some work
	 */

	/* let's shut it down now */
	rproc_shutdown(my_rproc);
  }

```
## 供实现者使用的 API


```
  struct rproc *rproc_alloc(struct device *dev, const char *name,
				const struct rproc_ops *ops,
				const char *firmware, int len)

```
分配一个新的远程处理器句柄，但暂不注册它。必需的参数有：底层设备、该远程处理器的名称、特定于平台的操作处理函数、用于启动该 rproc 的固件名称，以及分配rproc 的驱动所需的私有数据长度（以字节计）
该函数应rproc 实现在远程处理器初始化期间使用
使用该函数创rproc 句柄之后，在准备就绪时，实现者应调用 rproc_add() 来完成远程处理器的注册
成功时返回新rproc，失败时返回 NULL

  **never** 即使rproc 尚未注册，也绝不能直接释@rproc。相反，当你需要回退 rproc_alloc() 时，应使rproc_free()
```
  void rproc_free(struct rproc *rproc)

```
释放一个由 rproc_alloc 分配rproc 句柄
该函数本质上是通过递减 rproc 的引用计数来回退 rproc_alloc()。它不会直接释放 rproc；只有当rproc 没有其它引用、且其引用计数现在降为零时，才会真正释放
```
  int rproc_add(struct rproc *rproc)

```
在通过 rproc_alloc() 分配之后，向 remoteproc 框架注册 @rproc
当探测到一个新的远程处理器设备时，由特定于平台rproc 实现调用
成功时返0，否则返回相应的错误码。注意：该函数会启动一个异步的固件加载上下文，它将查找rproc 的固件所支持virtio 设备
如果找到，这virtio 设备将被创建并添加，因此作为注册该远程处理器的结果，可能会有额外virtio 驱动被探测到
```
  int rproc_del(struct rproc *rproc)

```
回退 rproc_add()
当特定于平台rproc 实现决定移除rproc 设备时，应当调用此函数。它应当仅在先前rproc_add() 的调用已成功完成时才被调用
rproc_del() 返回之后，@rproc 仍然有效，其最后的引用计数应当通过调用 rproc_free() 来递减
成功时返0，如@rproc 无效则返-EINVAL
```
  void rproc_report_crash(struct rproc *rproc, enum rproc_crash_type type)

```
报告 remoteproc 中发生了一次崩溃
每次特定于平台的 rproc 实现检测到一次崩溃时，都必须调用此函数。它不应被非 remoteproc 驱动调用。该函数可以在原中断上下文中调用

## 实现回调


这些回调应由特定于平台的 remoteproc 提供
```
  /**
   * struct rproc_ops - platform-specific device handlers
   * @start:	power on the device and boot it
   * @stop:	power off the device
   * @kick:	kick a virtqueue (virtqueue id given as a parameter)
   */
  struct rproc_ops {
	int (*start)(struct rproc *rproc);
	int (*stop)(struct rproc *rproc);
	void (*kick)(struct rproc *rproc, int vqid);
  };

```
每一remoteproc 实现至少应当提供 ->start ->stop 处理函数。如果还希望rpmsg/virtio 功能，那么也应当提供 ->kick 处理函数
->start() 处理函数接受一rproc 句柄，并应当为设备上电并启动它（使用 rproc->priv 来访问特定于平台的私有数据）。启动地址（如果需要的话）可以rproc->bootaddr 中找到（remoteproc 核心ELF 入口点放在那里）。成功时应当返回 0，失败时返回相应的错误码
->stop() 处理函数接受一rproc 句柄并为设备断电。成功时返回 0，失败时返回相应的错误码
->kick() 处理函数接受一rproc 句柄，以及放置了新消息的 virtqueue 索引。实现应当中断远程处理器，让它知道自己有待处理的消息。通知远程处理器具体要查看哪个 virtqueue 索引是可选的：遍历现有的 virtqueue 并在 used 环中查找新的缓冲区是容易的（且代价不高）

## 二进制固件结

目前 remoteproc 支持 ELF32 ELF64 固件二进制文件。不过，我们很可能会希望用该框架支持的其它平设备将基于不同的二进制格式
当这些用例出现时，我们必须将二进制格式与框架核心解耦，以便在不重复通用代码的情况下支持多种二进制格式
当固件被解析时，它的各个段会根据指定的设备地址（如果远程处理器直接访问内存，则可能是物理地址）被加载到内存中
除了标准ELF 段之外，大多数远程处理器还会包含一个我们称之为“资源表（resource table）”的特殊段
资源表包含远程处理器在上电之前所需的系统资源，例如分配物理上连续的内存，或对某些片上外设进iommu 映射。Remotecore 只有在资源表的所有要求都满足之后才会给设备上电
除了系统资源之外，资源表还可能包含用于公布远程处理器所支持的特性和配置的资源条目，例如跟踪缓冲区（trace buffer）以及受支持virtio 设备（及其配置）
```
  /**
   * struct resource_table - firmware resource table header
   * @ver: version number
   * @num: number of resource entries
   * @reserved: reserved (must be zero)
   * @offset: array of offsets pointing at the various resource entries
   *
   * The header of the resource table, as expressed by this structure,
   * contains a version number (should we need to change this format in the
   * future), the number of available resource entries, and their offsets
   * in the table.
   */
  struct resource_table {
	u32 ver;
	u32 num;
	u32 reserved[2];
	u32 offset[0];
  } __packed;

```
紧接在该头部之后的是资源条目本身```
  /**
   * struct fw_rsc_hdr - firmware resource entry header
   * @type: resource type
   * @data: resource data
   *
   * Every resource entry begins with a 'struct fw_rsc_hdr' header providing
   * its @type. The content of the entry itself will immediately follow
   * this header, and it should be parsed according to the resource type.
   */
  struct fw_rsc_hdr {
	u32 type;
	u8 data[0];
  } __packed;

```
有些资源条目仅仅是通告，告知主机某些特定的 remoteproc 配置。其它条目则要求主机做某些事情（例如分配一个系统资源）。有时还需要协商：固件请求一个资源，一旦分配完成，主机应当将其细节（例如已分配内存区域的地址）反馈回去
```
  /**
   * enum fw_resource_type - types of resource entries
   *
   * @RSC_CARVEOUT:   request for allocation of a physically contiguous
   *		    memory region.
   * @RSC_DEVMEM:     request to iommu_map a memory-based peripheral.
   * @RSC_TRACE:	    announces the availability of a trace buffer into which
   *		    the remote processor will be writing logs.
   * @RSC_VDEV:       declare support for a virtio device, and serve as its
   *		    virtio header.
   * @RSC_LAST:       just keep this one at the end
   * @RSC_VENDOR_START:	start of the vendor specific resource types range
   * @RSC_VENDOR_END:	end of the vendor specific resource types range
   *
   * Please note that these values are used as indices to the rproc_handle_rsc
   * lookup table, so please keep them sane. Moreover, @RSC_LAST is used to
   * check the validity of an index before the lookup table is accessed, so
   * please update it as needed.
   */
  enum fw_resource_type {
	RSC_CARVEOUT		= 0,
	RSC_DEVMEM		= 1,
	RSC_TRACE		= 2,
	RSC_VDEV		= 3,
	RSC_LAST		= 4,
	RSC_VENDOR_START	= 128,
	RSC_VENDOR_END		= 512,
  };

```
关于特定资源类型的更多细节，请参include/linux/remoteproc.h 中其专门的结构
我们也预期在将来会出现特定于平台的资源条目。当这种情况发生时，我们可以轻松地添加一个新RSC_PLATFORM 类型，并将这些资源交给特定于平台rproc 驱动去处理

## Virtio 涓?remoteproc


固件应当remoteproc 提供它所支持virtio 设备及其配置的信息：一RSC_VDEV 资源条目应当指定 virtio 设备 id（如 virtio_ids.h 中）、virtio 特性、virtio 配置空间、vring 信息等
当一个新的远程处理器被注册时，remoteproc 框架会查找它的资源表，并注册它所支持virtio 设备。一个固件可以支持任意数量的 virtio 设备，且可以是任意类型（如果需要，单个远程处理器也可以轻松地通过这种方式支持多个 rpmsg virtio 设备）
当然，RSC_VDEV 资源条目仅足以用virtio 设备的静态分配。动态分配也可以通过 rpmsg 总线实现（类似于我们已经rpmsg 通道进行的动态分配；更多信息请参rpmsg.txt）