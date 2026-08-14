
## 缃戠粶椹卞姩鐨?Netmem 鏀寔


鏈枃妗ｆ杩颁簡缃戠粶椹卞姩鏀寔 netmem 鐨勮姹傦紝netmem 鏄竴绉嶆娊璞＄殑鍐呭瓨绫诲瀷锛岃兘澶熸敮鎸佽濡傝澶囧唴瀛?TCP 绛?鐗规€с€傞€氳繃鏀寔 netmem锛岄┍鍔ㄥ彲浠ヤ互寰堝皯鐢氳嚦鏃犻渶淇敼鐨勬柟寮忥紝閰嶅悎鍚勭搴曞眰鍐呭瓨绫诲瀷宸ヤ綔銆?
Netmem 鐨勫ソ澶勶細

- 鐏垫椿鎬э細Netmem 鍙互鐢变笉鍚岀殑鍐呭瓨绫诲瀷锛堜緥濡?struct page銆丏MA-buf锛変綔涓哄悗绔紝浣块┍鍔ㄨ兘澶熸敮鎸佸悇绉?  鐢ㄤ緥锛屼緥濡傝澶囧唴瀛?TCP銆?- 闈㈠悜鏈潵锛氬甫鏈?netmem 鏀寔鐨勯┍鍔ㄥ凡涓轰緷璧栧畠鐨勫悗缁壒鎬у仛濂藉噯澶囥€?- 绠€鍖栧紑鍙戯細鏃犺搴曞眰鍐呭瓨瀹炵幇濡備綍锛岄┍鍔ㄩ兘閫氳繃涓€鑷寸殑 API 杩涜浜や簰銆?
## 椹卞姩 RX 瑕佹眰


1. 椹卞姩蹇呴』鏀寔 page_pool銆?
2. 椹卞姩蹇呴』鏀寔 tcp-data-split ethtool 閫夐」銆?
3. 椹卞姩蹇呴』涓烘湁鏁堣浇鑽峰唴瀛樹娇鐢?page_pool 鐨?netmem API銆俷etmem API 褰撳墠涓?page API 涓€涓€瀵瑰簲銆?   杞崲涓?netmem 搴斿綋鍙互閫氳繃灏?page API 鍒囨崲涓?netmem API锛屽苟鍦ㄩ┍鍔ㄤ腑閫氳繃 netmem_refs 鑰岄潪
   struct page * 鏉ヨ窡韪唴瀛樻潵瀹炵幇锛?
   - page_pool_alloc -> page_pool_alloc_netmem
   - page_pool_get_dma_addr -> page_pool_get_dma_addr_netmem
   - page_pool_put_page -> page_pool_put_netmem

   鐩墠骞堕潪鎵€鏈?page API 閮芥湁瀵瑰簲鐨?netmem 鐗堟湰銆傚鏋滀綘鐨勯┍鍔ㄤ緷璧栨煇涓己澶辩殑 netmem API锛屾杩?   鑷娣诲姞骞舵彁浜ゅ埌 netdev@锛屾垨鑰呰仈绯荤淮鎶よ€呭拰/鎴?almasrymina@google.com 浠ュ姹傚府鍔╂坊鍔犺 netmem API銆?
4. 椹卞姩蹇呴』浣跨敤浠ヤ笅 PP_FLAGS锛?
   - PP_FLAG_DMA_MAP锛歯etmem 涓嶈兘琚┍鍔ㄨ繘琛?dma 鏄犲皠銆傞┍鍔ㄥ繀椤诲皢 dma 鏄犲皠濮旀墭缁?page_pool锛屽畠鐭ラ亾
     浣曟椂锛堟垨涓嶏級閫傚悎杩涜 dma 鏄犲皠銆?   - PP_FLAG_DMA_SYNC_DEV锛歯etmem 鐨?dma 鍦板潃涓嶄竴瀹氳兘琚┍鍔ㄨ繘琛?dma 鍚屾銆傞┍鍔ㄥ繀椤诲皢 dma 鍚屾濮旀墭缁?     page_pool锛屽畠鐭ラ亾浣曟椂锛堟垨涓嶏級閫傚悎杩涜 dma 鍚屾銆?   - PP_FLAG_ALLOW_UNREADABLE_NETMEM銆備粎褰撳惎鐢ㄤ簡 tcp-data-split 鏃讹紝椹卞姩鎵嶅繀椤绘寚瀹氭鏍囧織銆?
5. 椹卞姩涓嶅緱鍋囧畾 netmem 鏄彲璇荤殑涓?鎴栫敱椤典綔涓哄悗绔€俻age_pool 杩斿洖鐨?netmem 鍙兘鏄笉鍙鐨勶紝姝ゆ椂
   netmem_address() 灏嗚繑鍥?NULL銆傞┍鍔ㄥ繀椤绘纭鐞嗕笉鍙鐨?netmem锛屽嵆褰?netmem_address() 涓?NULL 鏃讹紝
   涓嶈灏濊瘯澶勭悊鍏跺唴瀹广€?
   鐞嗘兂鎯呭喌涓嬶紝椹卞姩涓嶅繀閫氳繃鍍?netmem_is_net_iov() 杩欐牱鐨勮緟鍔╁嚱鏁版鏌ュ簳灞?netmem 绫诲瀷锛屼篃涓嶅繀閫氳繃
   netmem_to_page() 鎴?netmem_to_net_iov() 灏?netmem 杞崲涓哄畠鐨勪换浣曞簳灞傜被鍨嬨€傚湪澶у鏁版儏鍐典笅锛屾彁渚涗簡
   鎶借薄浜嗚繖绉嶅鏉傛€х殑 netmem 鎴?page_pool 杈呭姪鍑芥暟锛堣€屼笖杩樺彲浠ユ坊鍔犳洿澶氾級銆?
6. 椹卞姩蹇呴』浣跨敤 page_pool_dma_sync_netmem_for_cpu() 鏉ヤ唬鏇?dma_sync_single_range_for_cpu()銆傚浜庢煇浜?   鍐呭瓨鎻愪緵鏂癸紝闈㈠悜 CPU 鐨?dma 鍚屾灏嗙敱 page_pool 瀹屾垚锛涘浜庡叾浠栨彁渚涙柟锛堢壒鍒槸 dmabuf 鍐呭瓨鎻愪緵鏂癸級锛?   闈㈠悜 CPU 鐨?dma 鍚屾鐢变娇鐢?dmabuf API 鐨勭敤鎴风┖闂磋礋璐ｃ€傞┍鍔ㄥ繀椤诲皢鏁翠釜 dma 鍚屾鎿嶄綔濮旀墭缁?page_pool锛?   瀹冧細姝ｇ‘鍦板畬鎴愩€?
7. 閬垮厤鍩轰簬 page_pool 瀹炵幇椹卞姩鐗瑰畾鐨勫洖鏀躲€傞┍鍔ㄤ笉鑳芥寔鏈変竴涓?struct page 鏉ュ仛鑷繁鐨勫洖鏀讹紝鍥犱负 netmem
   鍙兘涓嶆槸鐢?struct page 浣滀负鍚庣鐨勩€備笉杩囷紝浣犲彲浠ヤ负姝ょ洰鐨勯€氳繃 page_pool_fragment_netmem() 鎴?   page_pool_ref_netmem() 鎸佹湁涓€涓?page_pool 寮曠敤锛屼絾瑕佹敞鎰忔煇浜?netmem 绫诲瀷鍙兘鏈夋洿闀跨殑娴佽浆鏃堕棿锛?   渚嬪鍦ㄩ浂鎷疯礉鍦烘櫙涓敤鎴风┖闂存寔鏈夊紩鐢ㄦ椂銆?
## 椹卞姩 TX 瑕佹眰


1. 椹卞姩涓嶅緱鐩存帴灏?netmem 鐨?dma_addr 浼犻€掔粰浠讳綍 dma-mapping API銆傝繖鏄洜涓?netmem 鐨?dma_addr 鍙兘鏉ヨ嚜
   鍍?dma-buf 杩欐牱涓?dma-mapping API 涓嶅吋瀹圭殑婧愩€?
   搴斾娇鐢ㄥ儚 netmem_dma_unmap_page_attrs() 涓?netmem_dma_unmap_addr_set() 杩欐牱鐨勮緟鍔╁嚱鏁帮紝鏉ヤ唬鏇?   dma_unmap_page[_attrs]()銆乨ma_unmap_addr_set()銆傛棤璁烘潵婧愬浣曪紝netmem 鍙樹綋閮戒細姝ｇ‘澶勭悊 netmem 鐨?   dma_addr锛屽苟鍦ㄩ€傚綋鏃跺鎵樼粰 dma-mapping API銆?
   鐩墠骞堕潪鎵€鏈?dma-mapping API 閮芥湁瀵瑰簲鐨?netmem 鐗堟湰銆傚鏋滀綘鐨勯┍鍔ㄤ緷璧栨煇涓己澶辩殑 netmem API锛屾杩?   鑷娣诲姞骞舵彁浜ゅ埌 netdev@锛屾垨鑰呰仈绯荤淮鎶よ€呭拰/鎴?almasrymina@google.com 浠ュ姹傚府鍔╂坊鍔犺 netmem API銆?
2. 椹卞姩搴旈€氳繃璁剧疆 `netdev->netmem_tx = true` 鏉ュ０鏄庢敮鎸併€?