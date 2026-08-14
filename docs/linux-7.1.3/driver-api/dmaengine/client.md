## DMA Engine（DMA 引擎）API 指南


Vinod Koul <vinod dot koul at intel.com>

          `Documentation/crypto/async-tx-api.rst`


以下是面向设备驱动开发者的指南，介绍如何使用 DMA Engine 的 Slave-DMA API。该指南仅适用于 slave DMA 用法。

## DMA 用法


slave DMA 的用法包含以下步骤：

- 分配一个 DMA slave 通道

- 设置 slave 与控制器相关的特定参数

- 获取事务的描述符

- 提交事务

- 发出待处理请求并等待回调通知

这些操作的细节如下：

1. 分配一个 DMA slave 通道

   在 slave DMA 场景下，通道分配略有不同，客户端驱动通常只需要来自某个特定 DMA 控制器的通道，某些情况下甚至需要某个特定通道。请求通道时使用 dma_request_chan() API。

   接口：

   .. code-block:: c

      struct dma_chan **dma_request_chan(struct device **dev, const char *name);

   该函数会查找并返回与 'dev' 设备关联的 `name` DMA 通道。关联通过 DT、ACPI 或基于板级文件的 dma_slave_map 匹配表完成。

   通过该接口分配的通道对调用者是独占的，直到调用 dma_release_channel()。

2. 设置 slave 与控制器相关特定参数

   下一步总是向 DMA 驱动传递一些特定信息。slave DMA 可用的大部分通用信息位于 struct dma_slave_config 中。这允许客户端为外设指定 DMA 方向、DMA 地址、总线宽度、DMA 突发长度等。

   如果某些 DMA 控制器需要传递更多参数，它们应尝试将 struct dma_slave_config 嵌入其控制器特定的结构体中。这样在需要时客户端可灵活地传递更多参数。

   接口：

   .. code-block:: c

      int dmaengine_slave_config(struct dma_chan *chan,
			struct dma_slave_config *config)

   有关结构体成员的详细说明，请参见 dmaengine.h 中的 dma_slave_config 结构定义。请注意，'direction' 成员即将被移除，因为它与 prepare 调用中给出的方向重复。

3. 获取事务的描述符

  对于 slave 用法，DMA engine 支持的各类 slave 传输模式如下：

  - slave_sg：将一组分散/聚集（scatter gather）缓冲区与/从外设进行 DMA

  - peripheral_dma_vec：将一组分散/聚集缓冲区数组与/从外设进行 DMA。类似 slave_sg，但使用 dma_vec 结构数组而非 scatterlist。

  - dma_cyclic：执行从/到外设的循环 DMA 操作，直到该操作被显式停止。

  - interleaved_dma：这同时适用于 Slave 与 M2M 客户端。对于 slave，设备 fifo 的地址驱动可能已经知晓。通过设置 'dma_interleaved_template' 成员的适当取值，可以表达多种类型的操作。如果通道支持，还可通过设置 DMA_PREP_REPEAT 传输标志实现循环交错 DMA 传输。

  该传输 API 返回非 NULL 即表示给定事务的一个“描述符”。

  接口：

  .. code-block:: c

     struct dma_async_tx_descriptor *dmaengine_prep_slave_sg(
		struct dma_chan **chan, struct scatterlist **sgl,
		unsigned int sg_len, enum dma_data_direction direction,
		unsigned long flags);

     struct dma_async_tx_descriptor *dmaengine_prep_peripheral_dma_vec(
		struct dma_chan **chan, const struct dma_vec **vecs,
		size_t nents, enum dma_data_direction direction,
		unsigned long flags);

     struct dma_async_tx_descriptor *dmaengine_prep_dma_cyclic(
		struct dma_chan *chan, dma_addr_t buf_addr, size_t buf_len,
		size_t period_len, enum dma_data_direction direction);

     struct dma_async_tx_descriptor *dmaengine_prep_interleaved_dma(
		struct dma_chan **chan, struct dma_interleaved_template **xt,
		unsigned long flags);

  外设驱动应在调用 dmaengine_prep_slave_sg() 之前已完成 scatterlist 的映射，并且必须保持 scatterlist 的映射状态直到 DMA 操作完成。scatterlist 必须使用 DMA struct device 进行映射。如果后续需要同步映射，也必须使用 DMA struct device 调用 dma_sync_**_for_**()。因此，正常的设置应如下所示：

  .. code-block:: c

     struct device *dma_dev = dmaengine_get_dma_device(chan);

     nr_sg = dma_map_sg(dma_dev, sgl, sg_len);
	if (nr_sg == 0)
		/** error **/

	desc = dmaengine_prep_slave_sg(chan, sgl, nr_sg, direction, flags);

  一旦获得描述符，就可以添加回调信息，随后必须提交该描述符。某些 DMA engine 驱动可能在成功 prepare 与提交之间持有一个自旋锁，因此将这两个操作紧密配对非常重要。

```

     尽管 async_tx API 规定完成回调函数不能提交任何新操作，但 slave/cyclic DMA 并非如此。

     对于 slave DMA，在回调函数被调用之前，后续事务可能尚不可提交，因此允许 slave DMA 回调准备并提交一个新的事务。

     对于 cyclic DMA，回调函数可能希望通过 dmaengine_terminate_async() 终止 DMA。

     因此，DMA engine 驱动必须在调用回调函数之前释放任何锁，否则可能导致死锁。

     注意，回调总是从 DMA engine 的 tasklet 中调用，绝不会在中断上下文中调用。

  **可选：每个描述符的元数据**

  DMAengine 提供两种方式支持元数据。

  DESC_METADATA_CLIENT

    元数据缓冲区由客户端驱动分配/提供，并附加到描述符上。

  .. code-block:: c

     int dmaengine_desc_attach_metadata(struct dma_async_tx_descriptor *desc,
				   void *data, size_t len);

  DESC_METADATA_ENGINE

    元数据缓冲区由 DMA 驱动分配/管理。客户端驱动可以查询元数据的指针、最大大小与当前已使用大小，并可直接更新或读取它。

    由于 DMA 驱动管理包含元数据的内存区域，客户端必须确保在描述符的传输完成回调运行后，不再尝试访问或获取该指针。如果传输未定义完成回调，则在 issue_pending 之后不得访问元数据。换言之：如果目的是在传输完成后读回元数据，则客户端必须使用完成回调。

  .. code-block:: c

     void *dmaengine_desc_get_metadata_ptr(struct dma_async_tx_descriptor *desc,
		size_t *payload_len, size_t *max_len);

     int dmaengine_desc_set_metadata_len(struct dma_async_tx_descriptor *desc,
		size_t payload_len);

  客户端驱动可通过以下方式查询给定模式是否受支持：

  .. code-block:: c

     bool dmaengine_is_metadata_mode_supported(struct dma_chan *chan,
		enum dma_desc_metadata_mode mode);

  根据所用模式的不同，客户端驱动必须遵循不同的流程。

  DESC_METADATA_CLIENT

    - DMA_MEM_TO_DEV / DEV_MEM_TO_MEM:

      1. 准备描述符（dmaengine_prep_*）
         在客户端缓冲区中构造元数据
      2. 使用 dmaengine_desc_attach_metadata() 将缓冲区附加到描述符
      3. 提交传输

    - DMA_DEV_TO_MEM:

      1. 准备描述符（dmaengine_prep_*）
      2. 使用 dmaengine_desc_attach_metadata() 将缓冲区附加到描述符
      3. 提交传输
      4. 传输完成时，元数据应在附加的缓冲区中可用

  DESC_METADATA_ENGINE

    - DMA_MEM_TO_DEV / DEV_MEM_TO_MEM:

      1. 准备描述符（dmaengine_prep_*）
      2. 使用 dmaengine_desc_get_metadata_ptr() 获取指向引擎元数据区的指针
      3. 在该指针处更新元数据
      4. 使用 dmaengine_desc_set_metadata_len() 告知 DMA engine 客户端已放入元数据缓冲区的字节数
      5. 提交传输

    - DMA_DEV_TO_MEM:

      1. 准备描述符（dmaengine_prep_*）
      2. 提交传输
      3. 传输完成时，使用 dmaengine_desc_get_metadata_ptr() 获取指向引擎元数据区的指针
      4. 从该指针读取元数据

  .. note::

     当使用 DESC_METADATA_ENGINE 模式时，描述符的元数据区在传输完成后不再有效（若使用回调，则有效至回调返回为止）。

     不允许混合使用 DESC_METADATA_CLIENT / DESC_METADATA_ENGINE，客户端驱动每个描述符必须只使用其中一种模式。

```
4. 提交事务

   一旦描述符已准备好并添加了回调信息，就必须将其放入 DMA engine 驱动的待处理队列。

   接口：

   .. code-block:: c

      dma_cookie_t dmaengine_submit(struct dma_async_tx_descriptor *desc)

   这会返回一个 cookie，可用于通过本文档未涵盖的其他 DMA engine 调用来检查 DMA engine 活动的进度。

   dmaengine_submit() 不会启动 DMA 操作，它只是将其加入待处理队列。为此，请参见第 5 步 dma_async_issue_pending。

```

      调用 ``dmaengine_submit()`` 后，已提交的传输描述符（``struct dma_async_tx_descriptor``）归 DMA engine 所有。因此，客户端必须认为指向该描述符的指针已失效。

```
5. 发出待处理 DMA 请求并等待回调通知

   可以通过调用 issue_pending API 来激活待处理队列中的事务。如果通道空闲，则队列中的第一个事务被启动，后续事务依次排队。

   每次 DMA 操作完成时，队列中的下一个事务被启动，并触发一个 tasklet。随后该 tasklet 会调用客户端驱动的完成回调函数以发出通知（若已设置）。

   接口：

   .. code-block:: c

      void dma_async_issue_pending(struct dma_chan *chan);

### 更多 API


1. 终止 API

   .. code-block:: c

      int dmaengine_terminate_sync(struct dma_chan *chan)
      int dmaengine_terminate_async(struct dma_chan *chan)
      int dmaengine_terminate_all(struct dma_chan **chan) /** DEPRECATED */

   这会导致该 DMA 通道上的所有活动停止，并可能丢弃 DMA FIFO 中尚未完全传输的数据。对于任何未完成的传输，不会调用任何回调函数。

   该函数有两种变体。

   dmaengine_terminate_async() 可能不会等待 DMA 完全停止，也不会等待任何正在运行的完成回调结束。但可以在原子上下文或完成回调内部调用 dmaengine_terminate_async()。在可以安全释放 DMA 传输所访问的内存或释放完成回调内部所访问的资源之前，必须先调用 dmaengine_synchronize()。

   dmaengine_terminate_sync() 会在返回前等待传输以及任何正在运行的完成回调结束。但该函数不得在原子上下文或完成回调内部调用。

   dmaengine_terminate_all() 已被弃用，不应在新代码中使用。

2. 暂停 API

   .. code-block:: c

      int dmaengine_pause(struct dma_chan *chan)

   这会暂停 DMA 通道上的活动且不会造成数据丢失。

3. 恢复 API

   .. code-block:: c

       int dmaengine_resume(struct dma_chan *chan)

   恢复之前已暂停的 DMA 通道。恢复一个当前并未处于暂停状态的通道是无效的。

4. 检查事务是否完成

   .. code-block:: c

      enum dma_status dma_async_is_tx_complete(struct dma_chan *chan,
		dma_cookie_t cookie, dma_cookie_t **last, dma_cookie_t **used)

   这可用于检查通道的状态。有关该 API 更完整的描述，请参见 include/linux/dmaengine.h 中的文档。

   这可与 dma_async_is_complete() 以及 dmaengine_submit() 返回的 cookie 配合使用，以检查特定 DMA 事务是否完成。

```

      并非所有 DMA engine 驱动都能为正在运行的 DMA 通道返回可靠信息。建议 DMA engine 用户在使用该 API 之前先暂停或停止（通过 dmaengine_terminate_all()）该通道。

```
5. 同步终止 API

   .. code-block:: c

      void dmaengine_synchronize(struct dma_chan *chan)

   将 DMA 通道的终止同步到当前上下文。

   该函数应在 dmaengine_terminate_async() 之后使用，以将 DMA 通道的终止同步到当前上下文。该函数会在返回前等待传输以及任何正在运行的完成回调结束。

   如果使用 dmaengine_terminate_async() 停止 DMA 通道，则在可以安全释放之前提交的描符所访问的内存，或释放这些描符完成回调内部所访问的任何资源之前，必须先调用此函数。

   如果在 dmaengine_terminate_async() 与此函数之间调用了 dma_async_issue_pending()，则此函数的行为未定义。
