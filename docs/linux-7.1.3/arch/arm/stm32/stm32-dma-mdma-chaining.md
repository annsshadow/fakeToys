
## STM32 DMA-MDMA 閾惧紡浼犺緭锛圫TM32 DMA-MDMA chaining锛?

鏈枃妗ｈ鏄?STM32 寰鐞嗗櫒涓?STM32 DMA 涓?STM32 MDMA 鎺у埗鍣ㄩ€氳繃 DMAMUX 瀹炵幇鐨勯摼寮忎紶杈撴満鍒讹紝浠嬬粛鐩稿叧澶栬鍙婂叾鍗忓悓宸ヤ綔鏂瑰紡锛岄€傜敤浜庨渶瑕佸湪涓嶅崰鐢?CPU 鐨勫墠鎻愪笅鍦ㄥ唴瀛樹笌澶栬闂存惉杩愭暟鎹殑寮€鍙戣€呫€?


### 绠€浠嬶紙Introduction锛?


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


### 鍘熺悊锛圥rinciples锛?


  STM32 DMA-MDMA 閾惧紡浼犺緭鐗规€т緷璧栦簬 STM32 DMA 鍜?STM32 MDMA 鎺у埗鍣ㄧ殑浼樺娍銆?

  STM32 DMA 鍏锋湁寰幆鍙岀紦鍐叉ā寮忥紙DBM锛夈€傚湪姣忔浜嬪姟缁撴潫鏃讹紙褰?DMA 鏁版嵁璁℃暟鍣?- DMA_SxNDTR - 杈惧埌 0锛夛紝鍐呭瓨鎸囬拡锛堥€氳繃 DMA_SxSM0AR 鍜?DMA_SxM1AR 閰嶇疆锛夎浜ゆ崲锛孌MA 鏁版嵁璁℃暟鍣ㄨ鑷姩閲嶈浇銆傝繖浣垮緱杞欢鎴?STM32 MDMA 鍙互鍦ㄧ浜屼釜鍐呭瓨鍖哄煙姝ｈ STM32 DMA 浼犺緭濉厖/浣跨敤鏃讹紝澶勭悊鍏朵腑涓€涓唴瀛樺尯鍩熴€?

  鍦?STM32 MDMA 閾捐〃妯″紡涓嬶紝鍗曚釜璇锋眰鍚姩瑕佷紶杈撶殑鏁版嵁鏁扮粍锛堣妭鐐圭殑闆嗗悎锛夛紝鐩村埌璇ラ€氶亾鐨勯摼琛ㄦ寚閽堜负绌恒€傛渶鍚庝竴涓妭鐐圭殑閫氶亾浼犺緭瀹屾垚鍗充负浼犺緭缁撴潫锛岄櫎闈炵涓€涓拰鏈€鍚庝竴涓妭鐐圭浉浜掗摼鎺ワ紝姝ゆ椂閾捐〃寰幆浠ュ垱寤哄惊鐜殑 MDMA 浼犺緭銆?

  STM32 MDMA 涓?STM32 DMA 鏈夌洿鎺ヨ繛鎺ャ€傝繖浣垮緱澶栬涔嬮棿鑳藉瀹炵幇鑷富閫氫俊涓庡悓姝ワ紝浠庤€岃妭鐪?CPU 璧勬簮鍜屾€荤嚎鎷ュ銆係TM32 DMA 閫氶亾鐨勪紶杈撳畬鎴愪俊鍙峰彲浠ヨЕ鍙?STM32 MDMA 浼犺緭銆係TM32 MDMA 鍙互閫氳繃鍐欏叆鍏朵腑鏂竻闄ゅ瘎瀛樺櫒锛堝湴鍧€淇濆瓨鍦?MDMA_CxMAR 涓紝浣嶆帺鐮佸湪 MDMA_CxMDR 涓級鏉ユ竻闄?STM32 DMA 浜х敓鐨勮姹傘€?

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

  STM32 DMA-MDMA 閾惧紡浼犺緭鐗规€ч殢鍚庝娇鐢ㄤ竴涓?SRAM 缂撳啿鍖恒€係TM32MP1 SoC 鍐呭祵涓変釜涓嶅悓澶у皬鐨勫揩閫熻闂潤鎬佸唴閮?RAM锛岀敤浜庢暟鎹瓨鍌ㄣ€傜敱浜?STM32 DMA 鐨勯仐鐣欒璁★紙鍦ㄥ井鎺у埗鍣ㄤ腑锛夛紝STM32 DMA 鍦?DDR 涓婄殑鎬ц兘杈冨樊锛岃€屽湪 SRAM 涓婃€ц兘鏈€浣炽€傚洜姝や娇鐢?STM32 DMA 涓?STM32 MDMA 涔嬮棿鐨?SRAM 缂撳啿鍖恒€傝缂撳啿鍖鸿鍒嗘垚鐩哥瓑鐨勪袱涓懆鏈燂紝STM32 DMA 浣跨敤鍏朵腑涓€涓懆鏈燂紝鑰?STM32 MDMA 鍚屾椂浣跨敤鍙︿竴涓懆鏈熴€?
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
### 鐢ㄤ簬 STM32 DMA-MDMA 閾惧紡浼犺緭鏀寔鐨勮澶囨爲鏇存柊锛圖evice Tree updates锛?


  **1. 鍒嗛厤涓€涓?SRAM 缂撳啿鍖猴紙Allocate a SRAM buffer锛?*

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
### 鍦?foo 椹卞姩涓敤浜?STM32 DMA-MDMA 閾惧紡浼犺緭鏀寔鐨勯┍鍔ㄦ洿鏂帮紙Driver updates锛?


  **0.锛堝彲閫夛級濡傛灉浣跨敤 dmaengine_prep_slave_sg()锛岄噸鏋勫師濮嬬殑 sg_table**

    鍦ㄤ娇鐢?dmaengine_prep_slave_sg() 鐨勬儏鍐典笅锛屽師濮嬬殑 sg_table 涓嶈兘鍘熸牱浣跨敤銆傚繀椤讳粠鍘熷琛ㄥ垱寤轰袱涓柊鐨?sg_table銆備竴涓敤浜?STM32 DMA 浼犺緭锛堝叾涓唴瀛樺湴鍧€鐜板湪鎸囧悜 SRAM 缂撳啿鍖鸿€岄潪 DDR 缂撳啿鍖猴級锛屽彟涓€涓敤浜?STM32 MDMA 浼犺緭锛堝叾涓唴瀛樺湴鍧€鎸囧悜 DDR 缂撳啿鍖猴級銆?

    鏂扮殑 sg_list 椤瑰繀椤婚€傞厤 SRAM 鍛ㄦ湡闀垮害銆備互涓嬫槸 DMA_DEV_TO_MEM 鐨勭ず渚嬶細
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
### 璧勬簮锛圧esources锛?


  Application note, datasheet and reference manual are available on ST website
  (STM32MP1_).

  Dedicated focus on three application notes (AN5224_, AN4031_ & AN5001_)
  dealing with STM32 DMAMUX, STM32 DMA and STM32 MDMA.


:Authors:

- Amelie Delaunay <amelie.delaunay@foss.st.com>
