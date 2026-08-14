#### USB DMA


鍦?Linux 2.5 鍐呮牳锛堝強鏇撮珮鐗堟湰锛変腑锛孶SB 璁惧椹卞姩瀵瑰浣曚娇鐢?DMA 鏉ユ墽琛?I/O 鎿嶄綔鏈変簡
鏇村鐨勬帶鍒躲€傝繖浜?API 鍦ㄥ唴鏍?USB 缂栫▼鎸囧崡锛坘erneldoc锛屾潵鑷簮浠ｇ爜锛変腑鏈夎缁嗚鏄庛€?
## API 姒傝


鎬讳綋鎯呭喌鏄紝USB 椹卞姩鍙互缁х画蹇界暐澶у鏁?DMA 闂锛屽敖绠″畠浠粛鐒跺繀椤绘彁渚?DMA 灏辩华鐨?缂撳啿鍖猴紙鍙傝 Documentation/core-api/dma-api-howto.rst锛夈€傝繖灏辨槸瀹冧滑鍦?2.4锛堝強鏇存棭锛?鍐呮牳涓殑宸ヤ綔鏂瑰紡锛屾垨鑰呭畠浠幇鍦ㄤ篃鍙互鎰熺煡 DMA銆?
鎰熺煡 DMA 鐨?USB 椹卞姩锛?
- 鏂板鐨勮皟鐢ㄤ娇鎰熺煡 DMA 鐨勯┍鍔ㄨ兘澶熷垎閰?dma 缂撳啿鍖猴紝骞朵负宸叉湁鐨?dma 灏辩华缂撳啿鍖虹鐞?  dma 鏄犲皠锛堣涓嬫枃锛夈€?
- URB 鏈変竴涓澶栫殑 "transfer_dma" 瀛楁锛屼互鍙婁竴涓寚绀哄叾鏄惁鏈夋晥鐨?transfer_flags
  浣嶃€傦紙鎺у埗璇锋眰涔熸湁 "setup_dma"锛屼絾椹卞姩涓嶅緱浣跨敤瀹冦€傦級

- 濡傛灉鎰熺煡 DMA 鐨勯┍鍔ㄦ病鏈夋姠鍏堝畬鎴愭槧灏勫苟璁剧疆 `URB_NO_TRANSFER_DMA_MAP`锛屽垯
  "usbcore" 浼氭槧灏勬 DMA 鍦板潃銆侶CD 涓嶄负 URB 绠＄悊 dma 鏄犲皠銆?
- 鏈変竴涓柊鐨勨€滈€氱敤 DMA API鈥濓紝鍏朵腑閮ㄥ垎鍙緵 USB 璁惧椹卞姩浣跨敤銆傜粷涓嶈鍦ㄤ换浣?USB 鎺ュ彛
  鎴栬澶囦笂浣跨敤 dma_set_mask()锛涢偅鍙兘浼氱牬鍧忓叡浜鎬荤嚎鐨勬墍鏈夎澶囥€?
## 娑堥櫎鎷疯礉


閬垮厤璁?CPU 涓嶅繀瑕佸湴鎷疯礉鏁版嵁鏄ソ浜嬨€備唬浠蜂細绱Н锛岃€屽儚缂撳瓨棰犵案锛坈ache-trashing锛夎繖绫?褰卞搷浼氭柦鍔犲井濡欑殑鎯╃綒銆?
- 濡傛灉浣犱竴鐩翠粠鍚屼竴涓紦鍐插尯杩涜澶ч噺灏忔暟鎹紶杈擄紝鍦ㄤ娇鐢?IOMMU 绠＄悊 DMA 鏄犲皠鐨勭郴缁熶笂锛?  杩欑湡鐨勪細娑堣€楀ぇ閲忚祫婧愩€備笌鎵ц I/O 鐩告瘮锛屼负姣忎釜璇锋眰寤虹珛鍜屾媶闄?IOMMU 鏄犲皠鐨勪唬浠峰彲鑳?  瑕侀珮寰楀锛?
  瀵逛簬杩欎簺鐗瑰畾鎯呭喌锛孶SB 鎻愪緵浜嗗垎閰嶅紑閿€鏇翠綆鐨勫唴瀛樼殑鍘熻銆傚畠浠殑宸ヤ綔鏂瑰紡绫讳技浜?  kmalloc 鍜?kfree 鐗堟湰锛屼负浣犳彁渚涘彲瀛樺叆 urb->transfer_buffer 鍜?urb->transfer_dma
  鐨勬纭被鍨嬬殑鍦板潃銆?```

	void *usb_alloc_coherent (struct usb_device *dev, size_t size,
		int mem_flags, dma_addr_t *dma);

	void usb_free_coherent (struct usb_device *dev, size_t size,
		void *addr, dma_addr_t dma);

  澶у鏁伴┍鍔?*涓嶅簲**浣跨敤杩欎簺鍘熻锛涘畠浠笉闇€瑕佷娇鐢ㄨ繖绫诲唴瀛橈紙鈥渄ma-coherent鈥濓級锛岃€屼粠
  :c:func:`kmalloc` 杩斿洖鐨勫唴瀛樹篃鑳芥甯稿伐浣溿€?
  杩斿洖鐨勫唴瀛樼紦鍐插尯鏄€渄ma-coherent鈥濈殑锛涙湁鏃朵綘鍙兘闇€瑕侀€氳繃浣跨敤鍐呭瓨灞忛殰鏉ュ己鍒朵竴鑷寸殑
  鍐呭瓨璁块棶椤哄簭銆傚畠娌℃湁浣跨敤娴佸紡锛坰treaming锛塂MA 鏄犲皠锛屽洜姝ら€傜敤浜庡湪鍚﹀垯浼氶绨?IOMMU
  鏄犲皠鐨勭郴缁熶笂杩涜灏忎紶杈撱€傦紙鏈夊叧鈥渃oherent鈥濆拰鈥渟treaming鈥滵MA 鏄犲皠鐨勫畾涔夛紝璇峰弬闃?  Documentation/core-api/dma-api-howto.rst銆傦級

  鐢宠 1/N 椤碉紙浠ュ強鐢宠 N 椤碉級鍦ㄧ┖闂翠笂鏄浉褰撻珮鏁堢殑銆?
  鍦ㄥぇ澶氭暟绯荤粺涓婏紝杩斿洖鐨勫唴瀛樺皢鏄湭缂撳瓨鐨勶紝鍥犱负 dma-coherent 鍐呭瓨鐨勮涔夎姹傝涔堢粫杩?  CPU 缂撳瓨锛岃涔堜娇鐢ㄥ甫鏈夋€荤嚎渚﹀惉锛坆us-snooping锛夋敮鎸佺殑缂撳瓨纭欢銆傝櫧鐒?x86 纭欢鍏锋湁
  杩欑鎬荤嚎渚﹀惉鑳藉姏锛屼絾璁稿鍏朵粬绯荤粺浣跨敤杞欢鏉ュ埛鏂扮紦瀛樿浠ラ槻姝?DMA 鍐茬獊銆?
```
- 鏌愪簺 EHCI 鎺у埗鍣ㄤ笂鐨勮澶囧彲浠ュ鐞嗗楂樼鍐呭瓨锛坔igh memory锛夌殑 DMA 杈撳叆杈撳嚭銆?
  閬楁喚鐨勬槸锛屽綋鍓嶇殑 Linux DMA 鍩虹璁炬柦娌℃湁鍚堢悊鐨勬柟寮忔潵鏆撮湶杩欎簺鑳藉姏鈥︹€﹁€屼笖鏃犺濡備綍锛?  HIGHMEM 鍦ㄥ緢澶х▼搴︿笂鏄?x86_32 鐗规湁鐨勪竴涓璁＄己闄枫€傛墍浠ヤ綘鏈€濂界殑鍔炴硶鏄‘淇濈粷涓嶅皢
  楂樼鍐呭瓨缂撳啿鍖轰紶鍏?USB 椹卞姩銆傝繖寰堝鏄擄紱瀹冩槸榛樿琛屼负銆傚彧鏄笉瑕佽鐩栧畠锛屼緥濡備娇鐢?  `NETIF_F_HIGHDMA`銆?
  杩欏彲鑳戒細杩娇浣犵殑璋冪敤鑰呭仛涓€浜涘弽寮圭紦鍐诧紙bounce buffering锛夛紝浠庨珮绔唴瀛樺鍒跺埌鈥滄櫘閫氣€?  DMA 鍐呭瓨銆傚鏋滀綘鑳芥兂鍑鸿В鍐虫闂锛堥拡瀵瑰唴瀛樿秴杩?1 GByte 鐨?x86_32 鏈哄櫒锛夌殑濂藉姙娉曪紝
  娆㈣繋鎻愪氦琛ヤ竵銆?
## 浣跨敤宸叉湁缂撳啿鍖?

宸叉湁缂撳啿鍖哄湪棣栧厛琚槧灏勫埌璁惧鐨?DMA 鍦板潃绌洪棿涔嬪墠锛屼笉鑳界敤浜?DMA銆傜劧鑰岋紝浼犻€掔粰浣犵殑
椹卞姩鐨勫ぇ澶氭暟缂撳啿鍖洪兘鍙互瀹夊叏鍦扮敤浜庤繖鏍风殑 DMA 鏄犲皠銆傦紙璇峰弬闃?Documentation/core-api/dma-api-howto.rst 鐨勭涓€鑺傦紝鏍囬涓衡€滃摢浜涘唴瀛樺彲鐢ㄤ簬 DMA锛熲€濓級

- 褰撲綘鎷ユ湁宸蹭负 USB 鎺у埗鍣ㄦ槧灏勫ソ鐨?scatterlist 鏃讹紝鍙互浣跨敤鏂扮殑 `usb_sg_*()` 璋冪敤锛?  瀹冧細灏?scatterlist 杞崲涓?```

	int usb_sg_init(struct usb_sg_request *io, struct usb_device *dev,
		unsigned pipe, unsigned	period, struct scatterlist *sg,
		int nents, size_t length, gfp_t mem_flags);

	void usb_sg_wait(struct usb_sg_request *io);

	void usb_sg_cancel(struct usb_sg_request *io);

  褰?USB 鎺у埗鍣ㄤ笉鏀寔 DMA 鏃讹紝鍙 scatterlist 涓殑椤典笉鍦?Highmem 涓紝``usb_sg_init()``
  灏变細灏濊瘯浠?PIO 鏂瑰紡鎻愪氦 URB锛岃€屽湪鐜颁唬鏋舵瀯涓婅繖绉嶆儏鍐甸潪甯哥綍瑙併€?
```
