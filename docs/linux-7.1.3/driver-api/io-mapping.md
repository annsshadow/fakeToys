## io_mapping 鍑芥暟


## API


linux/io-mapping.h 涓殑 io_mapping 鍑芥暟鎻愪緵浜嗕竴绉嶆娊璞★紝鐢ㄤ簬楂樻晥鍦板皢 I/O 璁惧鐨勫皬鍧楀尯鍩熸槧灏勫埌 CPU銆傚叾鏈€鍒濈敤閫旀槸鏀寔 32 浣嶅鐞嗗櫒涓婅緝澶х殑鍥惧舰 aperture锛屽洜涓哄湪杩欎簺澶勭悊鍣ㄤ笂鏃犳硶浣跨敤 ioremap_wc 灏嗘暣涓?aperture 闈欐€佹槧灏勫埌 CPU锛堥偅鏍蜂細娑堣€楄繃澶氱殑鍐呮牳鍦板潃绌洪棿锛夈€?
```

	struct io_mapping *io_mapping_create_wc(unsigned long base,
						unsigned long size)

```

'base' 鏄浣垮叾鍙槧灏勭殑鍖哄煙鐨勬€荤嚎鍦板潃锛岃€?'size' 琛ㄧず瑕佸惎鐢ㄧ殑鏄犲皠鍖哄煙澶у皬銆備袱鑰呭潎浠ュ瓧鑺備负鍗曚綅銆?
姝?_wc 鍙樹綋鎻愪緵鐨勬槧灏勫彧鑳戒笌 io_mapping_map_atomic_wc()銆乮o_mapping_map_local_wc() 鎴?io_mapping_map_wc() 涓€璧蜂娇鐢ㄣ€?
鍊熷姪姝ゆ槧灏勫璞★紝鍙互鏍规嵁闇€姹備互涓存椂鎴栭暱鏈熸柟寮忔槧灏勫崟涓〉銆傚綋鐒讹紝涓存椂鏄犲皠鏄?
```

	void *io_mapping_map_local_wc(struct io_mapping *mapping,
				      unsigned long offset)

	void *io_mapping_map_atomic_wc(struct io_mapping *mapping,
				       unsigned long offset)

```

'offset' 鏄墍瀹氫箟鏄犲皠鍖哄煙鍐呯殑鍋忕Щ閲忋€傝闂垱寤哄嚱鏁颁腑鎸囧畾鍖哄煙涔嬪鐨勫湴鍧€浼氫骇鐢熸湭瀹氫箟鐨勭粨鏋溿€備娇鐢ㄦ湭鎸夐〉瀵归綈鐨勫亸绉婚噺涔熶細浜х敓鏈畾涔夌殑缁撴灉銆傝繑鍥炲€兼寚鍚?CPU 鍦板潃绌洪棿涓殑鍗曚釜椤点€?
姝?_wc 鍙樹綋浼氳繑鍥炶椤电殑涓€涓啓鍏ュ悎骞讹紙write-combining锛夋槧灏勶紝涓斿彧鑳界敤浜庣敱 io_mapping_create_wc() 鍒涘缓鐨勬槧灏勩€?
涓存椂鏄犲皠浠呭湪璋冪敤鑰呯殑涓婁笅鏂囦腑鏈夋晥銆傝鏄犲皠涓嶄繚璇佸鎵€鏈?CPU 鍏ㄥ眬鍙銆?
io_mapping_map_local_wc() 鍦?X86 32 浣嶄笂鏈夊壇浣滅敤锛氬畠浼氱鐢ㄨ縼绉讳互浣挎槧灏勪唬鐮佹甯稿伐浣溿€備换浣曡皟鐢ㄨ€呴兘涓嶅緱渚濊禆杩欎竴鍓綔鐢ㄣ€?
io_mapping_map_atomic_wc() 鐨勫壇浣滅敤鏄鐢ㄦ姠鍗狅紙preemption锛夊拰缂洪〉锛坧agefaults锛夈€備笉瑕佸湪鏂颁唬鐮佷腑浣跨敤瀹冿紝璇锋敼鐢?io_mapping_map_local_wc()銆?
宓屽鏄犲皠蹇呴』浠ョ浉鍙嶉『搴忔挙閿€锛屽洜涓烘槧灏?
```

 addr1 = io_mapping_map_local_wc(map1, offset1);
 addr2 = io_mapping_map_local_wc(map2, offset2);
 ...
 io_mapping_unmap_local(addr2);
 io_mapping_unmap_local(addr1);

```

```

	void io_mapping_unmap_local(void *vaddr)
	void io_mapping_unmap_atomic(void *vaddr)

```

'vaddr' 蹇呴』鏄渶鍚庝竴娆?io_mapping_map_local_wc() 鎴?io_mapping_map_atomic_wc() 璋冪敤杩斿洖鐨勫€笺€傝繖浼氬彇娑堟槧灏勬寚瀹氱殑鏄犲皠锛屽苟鎾ら攢鏄犲皠鍑芥暟鐨勫壇浣滅敤銆?
濡傛灉浣犲湪鎸佹湁涓€涓槧灏勬湡闂撮渶瑕佺潯鐪狅紝鍙互浣跨敤甯歌鐨?
```

	void *io_mapping_map_wc(struct io_mapping *mapping,
				unsigned long offset)

```

鍏跺伐浣滄柟寮忕被浼间簬 io_mapping_map_atomic/local_wc()锛屽彧鏄病鏈夊壇浣滅敤锛屼笖鎸囬拡鍏ㄥ眬鍙銆?
```

	void io_mapping_unmap(void *vaddr)

```

鐢ㄤ簬瑙ｉ櫎鐢?io_mapping_map_wc() 鏄犲皠鐨勯〉銆?
```

	void io_mapping_free(struct io_mapping *mapping)

```
