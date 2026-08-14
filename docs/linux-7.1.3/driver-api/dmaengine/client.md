## DMA Engine锛圖MA 寮曟搸锛堿PI 鎸囧崡


Vinod Koul <vinod dot koul at intel.com>

          `Documentation/crypto/async-tx-api.rst`


浠ヤ笅鏄潰鍚戣澶囬┍鍔ㄥ紑鍙戣€呯殑鎸囧崡锛屼粙缁嶅浣曚娇鐢?DMA Engine 鐨?Slave-DMA API銆傝鎸囧崡浠呴€傜敤浜?slave DMA 鐢ㄦ硶銆?
## DMA 鐢ㄦ硶


slave DMA 鐨勭敤娉曞寘鍚互涓嬫楠わ細

- 鍒嗛厤涓€涓?DMA slave 閫氶亾

- 璁剧疆 slave 涓庢帶鍒跺櫒鐩稿叧鐨勭壒瀹氬弬鏁?
- 鑾峰彇浜嬪姟鐨勬弿杩扮

- 鎻愪氦浜嬪姟

- 鍙戝嚭寰呭鐞嗚姹傚苟绛夊緟鍥炶皟閫氱煡

杩欎簺鎿嶄綔鐨勭粏鑺傚涓嬶細

1. 鍒嗛厤涓€涓?DMA slave 閫氶亾

   鍦?slave DMA 鍦烘櫙涓嬶紝閫氶亾鍒嗛厤鐣ユ湁涓嶅悓锛屽鎴风椹卞姩閫氬父鍙渶瑕佹潵鑷煇涓壒瀹?DMA 鎺у埗鍣ㄧ殑閫氶亾锛屾煇浜涙儏鍐典笅鐢氳嚦闇€瑕佹煇涓壒瀹氶€氶亾銆傝姹傞€氶亾鏃朵娇鐢?dma_request_chan() API銆?
   鎺ュ彛锛?
   .. code-block:: c

      struct dma_chan **dma_request_chan(struct device **dev, const char *name);

   璇ュ嚱鏁颁細鏌ユ壘骞惰繑鍥炰笌 'dev' 璁惧鍏宠仈鐨?`name` DMA 閫氶亾銆傚叧鑱旈€氳繃 DT銆丄CPI 鎴栧熀浜庢澘绾ф枃浠剁殑 dma_slave_map 鍖归厤琛ㄥ畬鎴愩€?
   閫氳繃璇ユ帴鍙ｅ垎閰嶇殑閫氶亾瀵硅皟鐢ㄨ€呮槸鐙崰鐨勶紝鐩村埌璋冪敤 dma_release_channel()銆?
2. 璁剧疆 slave 涓庢帶鍒跺櫒鐩稿叧鐗瑰畾鍙傛暟

   涓嬩竴姝ユ€绘槸鍚?DMA 椹卞姩浼犻€掍竴浜涚壒瀹氫俊鎭€俿lave DMA 鍙敤鐨勫ぇ閮ㄥ垎閫氱敤淇℃伅浣嶄簬 struct dma_slave_config 涓€傝繖鍏佽瀹㈡埛绔负澶栬鎸囧畾 DMA 鏂瑰悜銆丏MA 鍦板潃銆佹€荤嚎瀹藉害銆丏MA 绐佸彂闀垮害绛夈€?
   濡傛灉鏌愪簺 DMA 鎺у埗鍣ㄩ渶瑕佷紶閫掓洿澶氬弬鏁帮紝瀹冧滑搴斿皾璇曞皢 struct dma_slave_config 宓屽叆鍏舵帶鍒跺櫒鐗瑰畾鐨勭粨鏋勪綋涓€傝繖鏍峰湪闇€瑕佹椂瀹㈡埛绔彲鐏垫椿鍦颁紶閫掓洿澶氬弬鏁般€?
   鎺ュ彛锛?
   .. code-block:: c

      int dmaengine_slave_config(struct dma_chan *chan,
			struct dma_slave_config *config)

   鏈夊叧缁撴瀯浣撴垚鍛樼殑璇︾粏璇存槑锛岃鍙傝 dmaengine.h 涓殑 dma_slave_config 缁撴瀯瀹氫箟銆傝娉ㄦ剰锛?direction' 鎴愬憳鍗冲皢琚Щ闄わ紝鍥犱负瀹冧笌 prepare 璋冪敤涓粰鍑虹殑鏂瑰悜閲嶅銆?
3. 鑾峰彇浜嬪姟鐨勬弿杩扮

  瀵逛簬 slave 鐢ㄦ硶锛孌MA engine 鏀寔鐨勫悇绫?slave 浼犺緭妯″紡濡備笅锛?
  - slave_sg锛氬皢涓€缁勫垎鏁?鑱氶泦锛坰catter gather锛夌紦鍐插尯涓?浠庡璁捐繘琛?DMA

  - peripheral_dma_vec锛氬皢涓€缁勫垎鏁?鑱氶泦缂撳啿鍖烘暟缁勪笌/浠庡璁捐繘琛?DMA銆傜被浼?slave_sg锛屼絾浣跨敤 dma_vec 缁撴瀯鏁扮粍鑰岄潪 scatterlist銆?
  - dma_cyclic锛氭墽琛屼粠/鍒板璁剧殑寰幆 DMA 鎿嶄綔锛岀洿鍒拌鎿嶄綔琚樉寮忓仠姝€?
  - interleaved_dma锛氳繖鍚屾椂閫傜敤浜?Slave 涓?M2M 瀹㈡埛绔€傚浜?slave锛岃澶?fifo 鐨勫湴鍧€椹卞姩鍙兘宸茬粡鐭ユ檽銆傞€氳繃璁剧疆 'dma_interleaved_template' 鎴愬憳鐨勯€傚綋鍙栧€硷紝鍙互琛ㄨ揪澶氱绫诲瀷鐨勬搷浣溿€傚鏋滈€氶亾鏀寔锛岃繕鍙€氳繃璁剧疆 DMA_PREP_REPEAT 浼犺緭鏍囧織瀹炵幇寰幆浜ら敊 DMA 浼犺緭銆?
  璇ヤ紶杈?API 杩斿洖闈?NULL 鍗宠〃绀虹粰瀹氫簨鍔＄殑涓€涓€滄弿杩扮鈥濄€?
  鎺ュ彛锛?
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

  澶栬椹卞姩搴斿湪璋冪敤 dmaengine_prep_slave_sg() 涔嬪墠宸插畬鎴?scatterlist 鐨勬槧灏勶紝骞朵笖蹇呴』淇濇寔 scatterlist 鐨勬槧灏勭姸鎬佺洿鍒?DMA 鎿嶄綔瀹屾垚銆俿catterlist 蹇呴』浣跨敤 DMA struct device 杩涜鏄犲皠銆傚鏋滃悗缁渶瑕佸悓姝ユ槧灏勶紝涔熷繀椤讳娇鐢?DMA struct device 璋冪敤 dma_sync_**_for_**()銆傚洜姝わ紝姝ｅ父鐨勮缃簲濡備笅鎵€绀猴細

  .. code-block:: c

     struct device *dma_dev = dmaengine_get_dma_device(chan);

     nr_sg = dma_map_sg(dma_dev, sgl, sg_len);
	if (nr_sg == 0)
		/** error **/

	desc = dmaengine_prep_slave_sg(chan, sgl, nr_sg, direction, flags);

  涓€鏃﹁幏寰楁弿杩扮锛屽氨鍙互娣诲姞鍥炶皟淇℃伅锛岄殢鍚庡繀椤绘彁浜よ鎻忚堪绗︺€傛煇浜?DMA engine 椹卞姩鍙兘鍦ㄦ垚鍔?prepare 涓庢彁浜や箣闂存寔鏈変竴涓嚜鏃嬮攣锛屽洜姝ゅ皢杩欎袱涓搷浣滅揣瀵嗛厤瀵归潪甯搁噸瑕併€?
```

     灏界 async_tx API 瑙勫畾瀹屾垚鍥炶皟鍑芥暟涓嶈兘鎻愪氦浠讳綍鏂版搷浣滐紝浣?slave/cyclic DMA 骞堕潪濡傛銆?
     瀵逛簬 slave DMA锛屽湪鍥炶皟鍑芥暟琚皟鐢ㄤ箣鍓嶏紝鍚庣画浜嬪姟鍙兘灏氫笉鍙彁浜わ紝鍥犳鍏佽 slave DMA 鍥炶皟鍑嗗骞舵彁浜や竴涓柊鐨勪簨鍔°€?
     瀵逛簬 cyclic DMA锛屽洖璋冨嚱鏁板彲鑳藉笇鏈涢€氳繃 dmaengine_terminate_async() 缁堟 DMA銆?
     鍥犳锛孌MA engine 椹卞姩蹇呴』鍦ㄨ皟鐢ㄥ洖璋冨嚱鏁颁箣鍓嶉噴鏀句换浣曢攣锛屽惁鍒欏彲鑳藉鑷存閿併€?
     娉ㄦ剰锛屽洖璋冩€绘槸浠?DMA engine 鐨?tasklet 涓皟鐢紝缁濅笉浼氬湪涓柇涓婁笅鏂囦腑璋冪敤銆?
  **鍙€夛細姣忎釜鎻忚堪绗︾殑鍏冩暟鎹?*

  DMAengine 鎻愪緵涓ょ鏂瑰紡鏀寔鍏冩暟鎹€?
  DESC_METADATA_CLIENT

    鍏冩暟鎹紦鍐插尯鐢卞鎴风椹卞姩鍒嗛厤/鎻愪緵锛屽苟闄勫姞鍒版弿杩扮涓娿€?
  .. code-block:: c

     int dmaengine_desc_attach_metadata(struct dma_async_tx_descriptor *desc,
				   void *data, size_t len);

  DESC_METADATA_ENGINE

    鍏冩暟鎹紦鍐插尯鐢?DMA 椹卞姩鍒嗛厤/绠＄悊銆傚鎴风椹卞姩鍙互鏌ヨ鍏冩暟鎹殑鎸囬拡銆佹渶澶уぇ灏忎笌褰撳墠宸蹭娇鐢ㄥぇ灏忥紝骞跺彲鐩存帴鏇存柊鎴栬鍙栧畠銆?
    鐢变簬 DMA 椹卞姩绠＄悊鍖呭惈鍏冩暟鎹殑鍐呭瓨鍖哄煙锛屽鎴风蹇呴』纭繚鍦ㄦ弿杩扮鐨勪紶杈撳畬鎴愬洖璋冭繍琛屽悗锛屼笉鍐嶅皾璇曡闂垨鑾峰彇璇ユ寚閽堛€傚鏋滀紶杈撴湭瀹氫箟瀹屾垚鍥炶皟锛屽垯鍦?issue_pending 涔嬪悗涓嶅緱璁块棶鍏冩暟鎹€傛崲瑷€涔嬶細濡傛灉鐩殑鏄湪浼犺緭瀹屾垚鍚庤鍥炲厓鏁版嵁锛屽垯瀹㈡埛绔繀椤讳娇鐢ㄥ畬鎴愬洖璋冦€?
  .. code-block:: c

     void *dmaengine_desc_get_metadata_ptr(struct dma_async_tx_descriptor *desc,
		size_t *payload_len, size_t *max_len);

     int dmaengine_desc_set_metadata_len(struct dma_async_tx_descriptor *desc,
		size_t payload_len);

  瀹㈡埛绔┍鍔ㄥ彲閫氳繃浠ヤ笅鏂瑰紡鏌ヨ缁欏畾妯″紡鏄惁鍙楁敮鎸侊細

  .. code-block:: c

     bool dmaengine_is_metadata_mode_supported(struct dma_chan *chan,
		enum dma_desc_metadata_mode mode);

  鏍规嵁鎵€鐢ㄦā寮忕殑涓嶅悓锛屽鎴风椹卞姩蹇呴』閬靛惊涓嶅悓鐨勬祦绋嬨€?
  DESC_METADATA_CLIENT

    - DMA_MEM_TO_DEV / DEV_MEM_TO_MEM:

      1. 鍑嗗鎻忚堪绗︼紙dmaengine_prep_*锛?         鍦ㄥ鎴风缂撳啿鍖轰腑鏋勯€犲厓鏁版嵁
      2. 浣跨敤 dmaengine_desc_attach_metadata() 灏嗙紦鍐插尯闄勫姞鍒版弿杩扮
      3. 鎻愪氦浼犺緭

    - DMA_DEV_TO_MEM:

      1. 鍑嗗鎻忚堪绗︼紙dmaengine_prep_*锛?      2. 浣跨敤 dmaengine_desc_attach_metadata() 灏嗙紦鍐插尯闄勫姞鍒版弿杩扮
      3. 鎻愪氦浼犺緭
      4. 浼犺緭瀹屾垚鏃讹紝鍏冩暟鎹簲鍦ㄩ檮鍔犵殑缂撳啿鍖轰腑鍙敤

  DESC_METADATA_ENGINE

    - DMA_MEM_TO_DEV / DEV_MEM_TO_MEM:

      1. 鍑嗗鎻忚堪绗︼紙dmaengine_prep_*锛?      2. 浣跨敤 dmaengine_desc_get_metadata_ptr() 鑾峰彇鎸囧悜寮曟搸鍏冩暟鎹尯鐨勬寚閽?      3. 鍦ㄨ鎸囬拡澶勬洿鏂板厓鏁版嵁
      4. 浣跨敤 dmaengine_desc_set_metadata_len() 鍛婄煡 DMA engine 瀹㈡埛绔凡鏀惧叆鍏冩暟鎹紦鍐插尯鐨勫瓧鑺傛暟
      5. 鎻愪氦浼犺緭

    - DMA_DEV_TO_MEM:

      1. 鍑嗗鎻忚堪绗︼紙dmaengine_prep_*锛?      2. 鎻愪氦浼犺緭
      3. 浼犺緭瀹屾垚鏃讹紝浣跨敤 dmaengine_desc_get_metadata_ptr() 鑾峰彇鎸囧悜寮曟搸鍏冩暟鎹尯鐨勬寚閽?      4. 浠庤鎸囬拡璇诲彇鍏冩暟鎹?
  .. note::

     褰撲娇鐢?DESC_METADATA_ENGINE 妯″紡鏃讹紝鎻忚堪绗︾殑鍏冩暟鎹尯鍦ㄤ紶杈撳畬鎴愬悗涓嶅啀鏈夋晥锛堣嫢浣跨敤鍥炶皟锛屽垯鏈夋晥鑷冲洖璋冭繑鍥炰负姝級銆?
     涓嶅厑璁告贩鍚堜娇鐢?DESC_METADATA_CLIENT / DESC_METADATA_ENGINE锛屽鎴风椹卞姩姣忎釜鎻忚堪绗﹀繀椤诲彧浣跨敤鍏朵腑涓€绉嶆ā寮忋€?
```
4. 鎻愪氦浜嬪姟

   涓€鏃︽弿杩扮宸插噯澶囧ソ骞舵坊鍔犱簡鍥炶皟淇℃伅锛屽氨蹇呴』灏嗗叾鏀惧叆 DMA engine 椹卞姩鐨勫緟澶勭悊闃熷垪銆?
   鎺ュ彛锛?
   .. code-block:: c

      dma_cookie_t dmaengine_submit(struct dma_async_tx_descriptor *desc)

   杩欎細杩斿洖涓€涓?cookie锛屽彲鐢ㄤ簬閫氳繃鏈枃妗ｆ湭娑电洊鐨勫叾浠?DMA engine 璋冪敤鏉ユ鏌?DMA engine 娲诲姩鐨勮繘搴︺€?
   dmaengine_submit() 涓嶄細鍚姩 DMA 鎿嶄綔锛屽畠鍙槸灏嗗叾鍔犲叆寰呭鐞嗛槦鍒椼€備负姝わ紝璇峰弬瑙佺 5 姝?dma_async_issue_pending銆?
```

      璋冪敤 ``dmaengine_submit()`` 鍚庯紝宸叉彁浜ょ殑浼犺緭鎻忚堪绗︼紙``struct dma_async_tx_descriptor``锛夊綊 DMA engine 鎵€鏈夈€傚洜姝わ紝瀹㈡埛绔繀椤昏涓烘寚鍚戣鎻忚堪绗︾殑鎸囬拡宸插け鏁堛€?
```
5. 鍙戝嚭寰呭鐞?DMA 璇锋眰骞剁瓑寰呭洖璋冮€氱煡

   鍙互閫氳繃璋冪敤 issue_pending API 鏉ユ縺娲诲緟澶勭悊闃熷垪涓殑浜嬪姟銆傚鏋滈€氶亾绌洪棽锛屽垯闃熷垪涓殑绗竴涓簨鍔¤鍚姩锛屽悗缁簨鍔′緷娆℃帓闃熴€?
   姣忔 DMA 鎿嶄綔瀹屾垚鏃讹紝闃熷垪涓殑涓嬩竴涓簨鍔¤鍚姩锛屽苟瑙﹀彂涓€涓?tasklet銆傞殢鍚庤 tasklet 浼氳皟鐢ㄥ鎴风椹卞姩鐨勫畬鎴愬洖璋冨嚱鏁颁互鍙戝嚭閫氱煡锛堣嫢宸茶缃級銆?
   鎺ュ彛锛?
   .. code-block:: c

      void dma_async_issue_pending(struct dma_chan *chan);

### 鏇村 API


1. 缁堟 API

   .. code-block:: c

      int dmaengine_terminate_sync(struct dma_chan *chan)
      int dmaengine_terminate_async(struct dma_chan *chan)
      int dmaengine_terminate_all(struct dma_chan **chan) /** DEPRECATED */

   杩欎細瀵艰嚧璇?DMA 閫氶亾涓婄殑鎵€鏈夋椿鍔ㄥ仠姝紝骞跺彲鑳戒涪寮?DMA FIFO 涓皻鏈畬鍏ㄤ紶杈撶殑鏁版嵁銆傚浜庝换浣曟湭瀹屾垚鐨勪紶杈擄紝涓嶄細璋冪敤浠讳綍鍥炶皟鍑芥暟銆?
   璇ュ嚱鏁版湁涓ょ鍙樹綋銆?
   dmaengine_terminate_async() 鍙兘涓嶄細绛夊緟 DMA 瀹屽叏鍋滄锛屼篃涓嶄細绛夊緟浠讳綍姝ｅ湪杩愯鐨勫畬鎴愬洖璋冪粨鏉熴€備絾鍙互鍦ㄥ師瀛愪笂涓嬫枃鎴栧畬鎴愬洖璋冨唴閮ㄨ皟鐢?dmaengine_terminate_async()銆傚湪鍙互瀹夊叏閲婃斁 DMA 浼犺緭鎵€璁块棶鐨勫唴瀛樻垨閲婃斁瀹屾垚鍥炶皟鍐呴儴鎵€璁块棶鐨勮祫婧愪箣鍓嶏紝蹇呴』鍏堣皟鐢?dmaengine_synchronize()銆?
   dmaengine_terminate_sync() 浼氬湪杩斿洖鍓嶇瓑寰呬紶杈撲互鍙婁换浣曟鍦ㄨ繍琛岀殑瀹屾垚鍥炶皟缁撴潫銆備絾璇ュ嚱鏁颁笉寰楀湪鍘熷瓙涓婁笅鏂囨垨瀹屾垚鍥炶皟鍐呴儴璋冪敤銆?
   dmaengine_terminate_all() 宸茶寮冪敤锛屼笉搴斿湪鏂颁唬鐮佷腑浣跨敤銆?
2. 鏆傚仠 API

   .. code-block:: c

      int dmaengine_pause(struct dma_chan *chan)

   杩欎細鏆傚仠 DMA 閫氶亾涓婄殑娲诲姩涓斾笉浼氶€犳垚鏁版嵁涓㈠け銆?
3. 鎭㈠ API

   .. code-block:: c

       int dmaengine_resume(struct dma_chan *chan)

   鎭㈠涔嬪墠宸叉殏鍋滅殑 DMA 閫氶亾銆傛仮澶嶄竴涓綋鍓嶅苟鏈浜庢殏鍋滅姸鎬佺殑閫氶亾鏄棤鏁堢殑銆?
4. 妫€鏌ヤ簨鍔℃槸鍚﹀畬鎴?
   .. code-block:: c

      enum dma_status dma_async_is_tx_complete(struct dma_chan *chan,
		dma_cookie_t cookie, dma_cookie_t **last, dma_cookie_t **used)

   杩欏彲鐢ㄤ簬妫€鏌ラ€氶亾鐨勭姸鎬併€傛湁鍏宠 API 鏇村畬鏁寸殑鎻忚堪锛岃鍙傝 include/linux/dmaengine.h 涓殑鏂囨。銆?
   杩欏彲涓?dma_async_is_complete() 浠ュ強 dmaengine_submit() 杩斿洖鐨?cookie 閰嶅悎浣跨敤锛屼互妫€鏌ョ壒瀹?DMA 浜嬪姟鏄惁瀹屾垚銆?
```

      骞堕潪鎵€鏈?DMA engine 椹卞姩閮借兘涓烘鍦ㄨ繍琛岀殑 DMA 閫氶亾杩斿洖鍙潬淇℃伅銆傚缓璁?DMA engine 鐢ㄦ埛鍦ㄤ娇鐢ㄨ API 涔嬪墠鍏堟殏鍋滄垨鍋滄锛堥€氳繃 dmaengine_terminate_all()锛夎閫氶亾銆?
```
5. 鍚屾缁堟 API

   .. code-block:: c

      void dmaengine_synchronize(struct dma_chan *chan)

   灏?DMA 閫氶亾鐨勭粓姝㈠悓姝ュ埌褰撳墠涓婁笅鏂囥€?
   璇ュ嚱鏁板簲鍦?dmaengine_terminate_async() 涔嬪悗浣跨敤锛屼互灏?DMA 閫氶亾鐨勭粓姝㈠悓姝ュ埌褰撳墠涓婁笅鏂囥€傝鍑芥暟浼氬湪杩斿洖鍓嶇瓑寰呬紶杈撲互鍙婁换浣曟鍦ㄨ繍琛岀殑瀹屾垚鍥炶皟缁撴潫銆?
   濡傛灉浣跨敤 dmaengine_terminate_async() 鍋滄 DMA 閫氶亾锛屽垯鍦ㄥ彲浠ュ畨鍏ㄩ噴鏀句箣鍓嶆彁浜ょ殑鎻忕鎵€璁块棶鐨勫唴瀛橈紝鎴栭噴鏀捐繖浜涙弿绗﹀畬鎴愬洖璋冨唴閮ㄦ墍璁块棶鐨勪换浣曡祫婧愪箣鍓嶏紝蹇呴』鍏堣皟鐢ㄦ鍑芥暟銆?
   濡傛灉鍦?dmaengine_terminate_async() 涓庢鍑芥暟涔嬮棿璋冪敤浜?dma_async_issue_pending()锛屽垯姝ゅ嚱鏁扮殑琛屼负鏈畾涔夈€?