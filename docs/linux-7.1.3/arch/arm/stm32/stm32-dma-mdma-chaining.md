
## STM32 DMA-MDMA 链式传输（STM32 DMA-MDMA chaining）

本文档说明 STM32 微处理器上 STM32 DMA 与 STM32 MDMA 控制器通过 DMAMUX 实现的链式传输机制，介绍相关外设及其协同工作方式，适用于需要在不占用 CPU 的前提下在内存与外设间搬运数据的开发者。


### 简介（Introduction）


  This document describes the STM32 DMA-MDMA chaining feature. But before going
  further, let's introduce the peripherals involved.

  To offload data transfers from the CPU, STM32 microprocessors (MPUs) embed
  direct memory access controllers (DMA).

  STM32MP1 SoCs embed both STM32 DMA and STM32 MDMA controllers. STM32 DMA
  request routing capabilities are enhanced by a DMA request multiplexer
  (STM32 DMAMUX).

  **STM32 DMAMUX**

  STM32 DMAMUX routes any DMA request from a given peripheral to any STM32 DMA
  controller (STM32MP1 counts two STM32 DMA controllers) channels.

  **STM32 DMA**

  STM32 DMA is mainly used to implement central data buffer storage (usually in
  the system SRAM) for different peripheral. It can access external RAMs but
  without the ability to generate convenient burst transfer ensuring the best
  load of the AXI.

  **STM32 MDMA**

  STM32 MDMA (Master DMA) is mainly used to manage direct data transfers between
  RAM data buffers without CPU intervention. It can also be used in a
  hierarchical structure that uses STM32 DMA as first level data buffer
  interfaces for AHB peripherals, while the STM32 MDMA acts as a second level
  DMA with better performance. As a AXI/AHB master, STM32 MDMA can take control
  of the AXI/AHB bus.


### 原理（Principles）


  STM32 DMA-MDMA 链式传输特性依赖于 STM32 DMA 和 STM32 MDMA 控制器的优势。

  STM32 DMA 具有循环双缓冲模式（DBM）。在每次事务结束时（当 DMA 数据计数器 - DMA_SxNDTR - 达到 0），内存指针（通过 DMA_SxSM0AR 和 DMA_SxM1AR 配置）被交换，DMA 数据计数器被自动重载。这使得软件或 STM32 MDMA 可以在第二个内存区域正被 STM32 DMA 传输填充/使用时，处理其中一个内存区域。

  在 STM32 MDMA 链表模式下，单个请求启动要传输的数据数组（节点的集合），直到该通道的链表指针为空。最后一个节点的通道传输完成即为传输结束，除非第一个和最后一个节点相互链接，此时链表循环以创建循环的 MDMA 传输。

  STM32 MDMA 与 STM32 DMA 有直接连接。这使得外设之间能够实现自主通信与同步，从而节省 CPU 资源和总线拥塞。STM32 DMA 通道的传输完成信号可以触发 STM32 MDMA 传输。STM32 MDMA 可以通过写入其中断清除寄存器（地址保存在 MDMA_CxMAR 中，位掩码在 MDMA_CxMDR 中）来清除 STM32 DMA 产生的请求。

  .. table:: STM32 MDMA interconnect table with STM32 DMA

    +--------------+----------------+-----------+------------+
    | STM32 DMAMUX | STM32 DMA      | STM32 DMA | STM32 MDMA |
    | channels     | channels       | Transfer  | request    |
    |              |                | complete  |            |
    |              |                | signal    |            |
    +==============+================+===========+============+
    | Channel **0**  | DMA1 channel 0 | dma1_tcf0 | **0x00**     |
    +--------------+----------------+-----------+------------+
    | Channel **1**  | DMA1 channel 1 | dma1_tcf1 | **0x01**     |
    +--------------+----------------+-----------+------------+
    | Channel **2**  | DMA1 channel 2 | dma1_tcf2 | **0x02**     |
    +--------------+----------------+-----------+------------+
    | Channel **3**  | DMA1 channel 3 | dma1_tcf3 | **0x03**     |
    +--------------+----------------+-----------+------------+
    | Channel **4**  | DMA1 channel 4 | dma1_tcf4 | **0x04**     |
    +--------------+----------------+-----------+------------+
    | Channel **5**  | DMA1 channel 5 | dma1_tcf5 | **0x05**     |
    +--------------+----------------+-----------+------------+
    | Channel **6**  | DMA1 channel 6 | dma1_tcf6 | **0x06**     |
    +--------------+----------------+-----------+------------+
    | Channel **7**  | DMA1 channel 7 | dma1_tcf7 | **0x07**     |
    +--------------+----------------+-----------+------------+
    | Channel **8**  | DMA2 channel 0 | dma2_tcf0 | **0x08**     |
    +--------------+----------------+-----------+------------+
    | Channel **9**  | DMA2 channel 1 | dma2_tcf1 | **0x09**     |
    +--------------+----------------+-----------+------------+
    | Channel **10** | DMA2 channel 2 | dma2_tcf2 | **0x0A**     |
    +--------------+----------------+-----------+------------+
    | Channel **11** | DMA2 channel 3 | dma2_tcf3 | **0x0B**     |
    +--------------+----------------+-----------+------------+
    | Channel **12** | DMA2 channel 4 | dma2_tcf4 | **0x0C**     |
    +--------------+----------------+-----------+------------+
    | Channel **13** | DMA2 channel 5 | dma2_tcf5 | **0x0D**     |
    +--------------+----------------+-----------+------------+
    | Channel **14** | DMA2 channel 6 | dma2_tcf6 | **0x0E**     |
    +--------------+----------------+-----------+------------+
    | Channel **15** | DMA2 channel 7 | dma2_tcf7 | **0x0F**     |
    +--------------+----------------+-----------+------------+

  STM32 DMA-MDMA 链式传输特性随后使用一个 SRAM 缓冲区。STM32MP1 SoC 内嵌三个不同大小的快速访问静态内部 RAM，用于数据存储。由于 STM32 DMA 的遗留设计（在微控制器中），STM32 DMA 在 DDR 上的性能较差，而在 SRAM 上性能最佳。因此使用 STM32 DMA 与 STM32 MDMA 之间的 SRAM 缓冲区。该缓冲区被分成相等的两个周期，STM32 DMA 使用其中一个周期，而 STM32 MDMA 同时使用另一个周期。
```

                    dma[1:2]-tcf[0:7]
                   .----------------.
     ____________ '    _________     V____________
    | STM32 DMA  |    /  __|>_  \    | STM32 MDMA |
    |------------|   |  /     \  |   |------------|
    | DMA_SxM0AR |<=>| | SRAM  | |<=>| []-[]...[] |
    | DMA_SxM1AR |   |  \_____/  |   |            |
    |____________|    \___<|____/    |____________|

  STM32 DMA-MDMA chaining uses (struct dma_slave_config).peripheral_config to
  exchange the parameters needed to configure MDMA. These parameters are
  gathered into a u32 array with three values:

  * the STM32 MDMA request (which is actually the DMAMUX channel ID),
  * the address of the STM32 DMA register to clear the Transfer Complete
    interrupt flag,
  * the mask of the Transfer Complete interrupt flag of the STM32 DMA channel.

```
### 用于 STM32 DMA-MDMA 链式传输支持的设备树更新（Device Tree updates）


  **1. 分配一个 SRAM 缓冲区（Allocate a SRAM buffer）**

    SRAM device tree node is defined in SoC device tree. You can refer to it in
    your board device tree to define your SRAM pool.
```

          &sram {
                  my_foo_device_dma_pool: dma-sram@0 {
                          reg = <0x0 0x1000>;
                  };
          };

    Be careful of the start index, in case there are other SRAM consumers.
    Define your pool size strategically: to optimise chaining, the idea is that
    STM32 DMA and STM32 MDMA can work simultaneously, on each buffer of the
    SRAM.
    If the SRAM period is greater than the expected DMA transfer, then STM32 DMA
    and STM32 MDMA will work sequentially instead of simultaneously. It is not a
    functional issue but it is not optimal.

    Don't forget to refer to your SRAM pool in your device node. You need to
    define a new property.
    ::

          &my_foo_device {
                  ...
                  my_dma_pool = &my_foo_device_dma_pool;
          };

    Then get this SRAM pool in your foo driver and allocate your SRAM buffer.

  **2. Allocate a STM32 DMA channel and a STM32 MDMA channel**

    You need to define an extra channel in your device tree node, in addition to
    the one you should already have for "classic" DMA operation.

    This new channel must be taken from STM32 MDMA channels, so, the phandle of
    the DMA controller to use is the MDMA controller's one.
    ::

          &my_foo_device {
                  [...]
                  my_dma_pool = &my_foo_device_dma_pool;
                  dmas = <&dmamux1 ...>,                // STM32 DMA channel
                         <&mdma1 0 0x3 0x1200000a 0 0>; // + STM32 MDMA channel
          };

    Concerning STM32 MDMA bindings:

    1. The request line number : whatever the value here, it will be overwritten
    by MDMA driver with the STM32 DMAMUX channel ID passed through
    (struct dma_slave_config).peripheral_config

    2. The priority level : choose Very High (0x3) so that your channel will
    take priority other the other during request arbitration

    3. A 32bit mask specifying the DMA channel configuration : source and
    destination address increment, block transfer with 128 bytes per single
    transfer

    4. The 32bit value specifying the register to be used to acknowledge the
    request: it will be overwritten by MDMA driver, with the DMA channel
    interrupt flag clear register address passed through
    (struct dma_slave_config).peripheral_config

    5. The 32bit mask specifying the value to be written to acknowledge the
    request: it will be overwritten by MDMA driver, with the DMA channel
    Transfer Complete flag passed through
    (struct dma_slave_config).peripheral_config

```
### 在 foo 驱动中用于 STM32 DMA-MDMA 链式传输支持的驱动更新（Driver updates）


  **0.（可选）如果使用 dmaengine_prep_slave_sg()，重构原始的 sg_table**

    在使用 dmaengine_prep_slave_sg() 的情况下，原始的 sg_table 不能原样使用。必须从原始表创建两个新的 sg_table。一个用于 STM32 DMA 传输（其中内存地址现在指向 SRAM 缓冲区而非 DDR 缓冲区），另一个用于 STM32 MDMA 传输（其中内存地址指向 DDR 缓冲区）。

    新的 sg_list 项必须适配 SRAM 周期长度。以下是 DMA_DEV_TO_MEM 的示例：
```

      /*
        * Assuming sgl and nents, respectively the initial scatterlist and its
        * length.
        * Assuming sram_dma_buf and sram_period, respectively the memory
        * allocated from the pool for DMA usage, and the length of the period,
        * which is half of the sram_buf size.
        */
      struct sg_table new_dma_sgt, new_mdma_sgt;
      struct scatterlist *s, *_sgl;
      dma_addr_t ddr_dma_buf;
      u32 new_nents = 0, len;
      int i;

      /* Count the number of entries needed */
      for_each_sg(sgl, s, nents, i)
              if (sg_dma_len(s) > sram_period)
                      new_nents += DIV_ROUND_UP(sg_dma_len(s), sram_period);
              else
                      new_nents++;

      /* Create sg table for STM32 DMA channel */
      ret = sg_alloc_table(&new_dma_sgt, new_nents, GFP_ATOMIC);
      if (ret)
              dev_err(dev, "DMA sg table alloc failed\n");

      for_each_sg(new_dma_sgt.sgl, s, new_dma_sgt.nents, i) {
              _sgl = sgl;
              sg_dma_len(s) = min(sg_dma_len(_sgl), sram_period);
              /* Targets the beginning = first half of the sram_buf */
              s->dma_address = sram_buf;
              /*
                * Targets the second half of the sram_buf
                * for odd indexes of the item of the sg_list
                */
              if (i & 1)
                      s->dma_address += sram_period;
      }

      /* Create sg table for STM32 MDMA channel */
      ret = sg_alloc_table(&new_mdma_sgt, new_nents, GFP_ATOMIC);
      if (ret)
              dev_err(dev, "MDMA sg_table alloc failed\n");

      _sgl = sgl;
      len = sg_dma_len(sgl);
      ddr_dma_buf = sg_dma_address(sgl);
      for_each_sg(mdma_sgt.sgl, s, mdma_sgt.nents, i) {
              size_t bytes = min_t(size_t, len, sram_period);

              sg_dma_len(s) = bytes;
              sg_dma_address(s) = ddr_dma_buf;
              len -= bytes;

              if (!len && sg_next(_sgl)) {
                      _sgl = sg_next(_sgl);
                      len = sg_dma_len(_sgl);
                      ddr_dma_buf = sg_dma_address(_sgl);
              } else {
                      ddr_dma_buf += bytes;
              }
      }

    Don't forget to release these new sg_tables after getting the descriptors
    with dmaengine_prep_slave_sg().

  **1. Set controller specific parameters**

    First, use dmaengine_slave_config() with a struct dma_slave_config to
    configure STM32 DMA channel. You just have to take care of DMA addresses,
    the memory address (depending on the transfer direction) must point on your
    SRAM buffer, and set (struct dma_slave_config).peripheral_size != 0.

    STM32 DMA driver will check (struct dma_slave_config).peripheral_size to
    determine if chaining is being used or not. If it is used, then STM32 DMA
    driver fills (struct dma_slave_config).peripheral_config with an array of
    three u32 : the first one containing STM32 DMAMUX channel ID, the second one
    the channel interrupt flag clear register address, and the third one the
    channel Transfer Complete flag mask.

    Then, use dmaengine_slave_config with another struct dma_slave_config to
    configure STM32 MDMA channel. Take care of DMA addresses, the device address
    (depending on the transfer direction) must point on your SRAM buffer, and
    the memory address must point to the buffer originally used for "classic"
    DMA operation. Use the previous (struct dma_slave_config).peripheral_size
    and .peripheral_config that have been updated by STM32 DMA driver, to set
    (struct dma_slave_config).peripheral_size and .peripheral_config of the
    struct dma_slave_config to configure STM32 MDMA channel.
    ::

      struct dma_slave_config dma_conf;
      struct dma_slave_config mdma_conf;

      memset(&dma_conf, 0, sizeof(dma_conf));
      [...]
      config.direction = DMA_DEV_TO_MEM;
      config.dst_addr = sram_dma_buf;        // SRAM buffer
      config.peripheral_size = 1;            // peripheral_size != 0 => chaining

      dmaengine_slave_config(dma_chan, &dma_config);

      memset(&mdma_conf, 0, sizeof(mdma_conf));
      config.direction = DMA_DEV_TO_MEM;
      mdma_conf.src_addr = sram_dma_buf;     // SRAM buffer
      mdma_conf.dst_addr = rx_dma_buf;       // original memory buffer
      mdma_conf.peripheral_size = dma_conf.peripheral_size;       // <- dma_conf
      mdma_conf.peripheral_config = dma_config.peripheral_config; // <- dma_conf

      dmaengine_slave_config(mdma_chan, &mdma_conf);

  **2. Get a descriptor for STM32 DMA channel transaction**

    In the same way you get your descriptor for your "classic" DMA operation,
    you just have to replace the original sg_list (in case of
    dmaengine_prep_slave_sg()) with the new sg_list using SRAM buffer, or to
    replace the original buffer address, length and period (in case of
    dmaengine_prep_dma_cyclic()) with the new SRAM buffer.

  **3. Get a descriptor for STM32 MDMA channel transaction**

    If you previously get descriptor (for STM32 DMA) with

    * dmaengine_prep_slave_sg(), then use dmaengine_prep_slave_sg() for
      STM32 MDMA;
    * dmaengine_prep_dma_cyclic(), then use dmaengine_prep_dma_cyclic() for
      STM32 MDMA.

    Use the new sg_list using SRAM buffer (in case of dmaengine_prep_slave_sg())
    or, depending on the transfer direction, either the original DDR buffer (in
    case of DMA_DEV_TO_MEM) or the SRAM buffer (in case of DMA_MEM_TO_DEV), the
    source address being previously set with dmaengine_slave_config().

  **4. Submit both transactions**

    Before submitting your transactions, you may need to define on which
    descriptor you want a callback to be called at the end of the transfer
    (dmaengine_prep_slave_sg()) or the period (dmaengine_prep_dma_cyclic()).
    Depending on the direction, set the callback on the descriptor that finishes
    the overall transfer:

    * DMA_DEV_TO_MEM: set the callback on the "MDMA" descriptor
    * DMA_MEM_TO_DEV: set the callback on the "DMA" descriptor

    Then, submit the descriptors whatever the order, with dmaengine_tx_submit().

  **5. Issue pending requests (and wait for callback notification)**

  As STM32 MDMA channel transfer is triggered by STM32 DMA, you must issue
  STM32 MDMA channel before STM32 DMA channel.

  If any, your callback will be called to warn you about the end of the overall
  transfer or the period completion.

  Don't forget to terminate both channels. STM32 DMA channel is configured in
  cyclic Double-Buffer mode so it won't be disabled by HW, you need to terminate
  it. STM32 MDMA channel will be stopped by HW in case of sg transfer, but not
  in case of cyclic transfer. You can terminate it whatever the kind of transfer.

  **STM32 DMA-MDMA chaining DMA_MEM_TO_DEV special case**

  STM32 DMA-MDMA chaining in DMA_MEM_TO_DEV is a special case. Indeed, the
  STM32 MDMA feeds the SRAM buffer with the DDR data, and the STM32 DMA reads
  data from SRAM buffer. So some data (the first period) have to be copied in
  SRAM buffer when the STM32 DMA starts to read.

  A trick could be pausing the STM32 DMA channel (that will raise a Transfer
  Complete signal, triggering the STM32 MDMA channel), but the first data read
  by the STM32 DMA could be "wrong". The proper way is to prepare the first SRAM
  period with dmaengine_prep_dma_memcpy(). Then this first period should be
  "removed" from the sg or the cyclic transfer.

  Due to this complexity, rather use the STM32 DMA-MDMA chaining for
  DMA_DEV_TO_MEM and keep the "classic" DMA usage for DMA_MEM_TO_DEV, unless
  you're not afraid.

```
### 资源（Resources）


  Application note, datasheet and reference manual are available on ST website
  (STM32MP1_).

  Dedicated focus on three application notes (AN5224_, AN4031_ & AN5001_)
  dealing with STM32 DMAMUX, STM32 DMA and STM32 MDMA.


:Authors:

- Amelie Delaunay <amelie.delaunay@foss.st.com>
