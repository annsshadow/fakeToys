######## ioctl MEDIA_IOC_G_TOPOLOGY


## 鍚嶇О锛圢ame锛?

MEDIA_IOC_G_TOPOLOGY - 鏋氫妇鍥炬嫇鎵戝拰鍥惧厓绱犲睘鎬?
## 姒傝锛圫ynopsis锛?

`int ioctl(int fd, MEDIA_IOC_G_TOPOLOGY, struct media_v2_topology *argp)`

## 鍙傛暟锛圓rguments锛?

`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `media_v2_topology` 鐨勬寚閽堛€?
## 鎻忚堪锛圖escription锛?

璇?ioctl 鐨勫吀鍨嬬敤娉曟槸璋冪敤涓ゆ銆傜涓€娆¤皟鐢ㄦ椂锛宻truct
`media_v2_topology` 瀹氫箟鐨勭粨鏋勪綋搴旇娓呴浂銆傝繑鍥炴椂锛屽鏋滄病鏈夐敊璇彂鐢燂紝璇?ioctl 灏嗚繑鍥?`topology_version` 浠ュ強瀹炰綋銆佹帴鍙ｃ€乸ad 鍜岄摼鎺ョ殑鎬绘暟銆?
鍦ㄧ浜屾璋冪敤涔嬪墠锛岀敤鎴风┖闂村簲鍒嗛厤鏁扮粍鏉ュ瓨鍌ㄦ墍闇€鐨勫浘鍏冪礌锛屽皢鎸囧悜瀹冧滑鐨勬寚閽堟斁鍒?ptr_entities銆乸tr_interfaces銆乸tr_links 鍜?鎴?ptr_pads锛屽叾浣欏€间繚鎸佷笉鍙樸€?
濡傛灉 `topology_version` 淇濇寔涓嶅彉锛岃 ioctl 搴斾娇鐢ㄥ獟浣撳浘鍏冪礌濉厖鎵€闇€鐨勬暟缁勩€?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u64
       - `topology_version`
       - 濯掍綋鍥炬嫇鎵戠殑鐗堟湰銆傚綋鍥捐鍒涘缓鏃讹紝璇ュ瓧娈典粠闆跺紑濮嬨€傛瘡褰撴湁鍥惧厓绱犺
	  娣诲姞鎴栫Щ闄わ紝璇ュ瓧娈甸兘浼氶€掑銆?
    - -  __u32
       - `num_entities`
       - 鍥句腑瀹炰綋鐨勬暟閲?
    - -  __u32
       - `reserved1`
       - 搴旂敤绋嬪簭鍜岄┍鍔ㄥ簲灏嗗叾璁句负 0銆?
    - -  __u64
       - `ptr_entities`
       - 鎸囧悜灏嗗瓨鍌?entities 鏁扮粍鐨勫唴瀛樺尯鍩熺殑鎸囬拡锛岃浆鎹负 64 浣嶆暣鏁般€?	  瀹冨彲浠ヤ负闆躲€傚鏋滀负闆讹紝璇?ioctl 灏嗕笉浼氬瓨鍌?entities锛岃€屽彧浼氭洿鏂?	  `num_entities`

    - -  __u32
       - `num_interfaces`
       - 鍥句腑鎺ュ彛鐨勬暟閲?
    - -  __u32
       - `reserved2`
       - 搴旂敤绋嬪簭鍜岄┍鍔ㄥ簲灏嗗叾璁句负 0銆?
    - -  __u64
       - `ptr_interfaces`
       - 鎸囧悜灏嗗瓨鍌?interfaces 鏁扮粍鐨勫唴瀛樺尯鍩熺殑鎸囬拡锛岃浆鎹负 64 浣嶆暣鏁般€?	  瀹冨彲浠ヤ负闆躲€傚鏋滀负闆讹紝璇?ioctl 灏嗕笉浼氬瓨鍌?interfaces锛岃€屽彧浼氭洿鏂?	  `num_interfaces`

    - -  __u32
       - `num_pads`
       - 鍥句腑 pad 鐨勬€绘暟

    - -  __u32
       - `reserved3`
       - 搴旂敤绋嬪簭鍜岄┍鍔ㄥ簲灏嗗叾璁句负 0銆?
    - -  __u64
       - `ptr_pads`
       - 鎸囧悜灏嗗瓨鍌?pads 鏁扮粍鐨勫唴瀛樺尯鍩熺殑鎸囬拡锛岃浆鎹负 64 浣嶆暣鏁般€?	  瀹冨彲浠ヤ负闆躲€傚鏋滀负闆讹紝璇?ioctl 灏嗕笉浼氬瓨鍌?pads锛岃€屽彧浼氭洿鏂?	  `num_pads`

    - -  __u32
       - `num_links`
       - 鍥句腑鏁版嵁涓庢帴鍙ｉ摼鎺ョ殑鎬绘暟

    - -  __u32
       - `reserved4`
       - 搴旂敤绋嬪簭鍜岄┍鍔ㄥ簲灏嗗叾璁句负 0銆?
    - -  __u64
       - `ptr_links`
       - 鎸囧悜灏嗗瓨鍌?links 鏁扮粍鐨勫唴瀛樺尯鍩熺殑鎸囬拡锛岃浆鎹负 64 浣嶆暣鏁般€?	  瀹冨彲浠ヤ负闆躲€傚鏋滀负闆讹紝璇?ioctl 灏嗕笉浼氬瓨鍌?links锛岃€屽彧浼氭洿鏂?	  `num_links`



    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - 瀹炰綋鐨勫敮涓€ ID銆備笉瑕佹湡鏈涜 ID 瀵硅澶囩殑姣忎釜瀹炰緥閮?	  濮嬬粓鐩稿悓銆傛崲鍙ヨ瘽璇达紝涓嶈鍦ㄥ簲鐢ㄧ▼搴忎腑纭紪鐮佸疄浣?ID銆?
    - -  char
       - `name`\ [^64^]
       - 瀹炰綋鍚嶇О锛屼綔涓轰互 UTF-8 NULL 缁撳熬鐨勫瓧绗︿覆銆傝鍚嶇О鍦ㄥ獟浣撴嫇鎵戝唴
	  蹇呴』鍞竴銆?
    - -  __u32
       - `function`
       - 瀹炰綋鐨勪富鍔熻兘锛岃瑙?media-entity-functions銆?
    - -  __u32
       - `flags`
       - 瀹炰綋鏍囧織锛岃瑙?media-entity-flag銆?	  浠呭綋 `MEDIA_V2_ENTITY_HAS_FLAGS(media_version)` 杩斿洖
	  true 鏃舵湁鏁堛€俙media_version` 瀹氫箟浜?struct
	  `media_device_info` 涓紝鍙€氳繃
	  MEDIA_IOC_DEVICE_INFO 鑾峰彇銆?
    - -  __u32
       - `reserved`\ [^5^]
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍璁句负闆躲€?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - 鎺ュ彛鐨勫敮涓€ ID銆備笉瑕佹湡鏈涜 ID 瀵硅澶囩殑姣忎釜瀹炰緥閮?	  濮嬬粓鐩稿悓銆傛崲鍙ヨ瘽璇达紝涓嶈鍦ㄥ簲鐢ㄧ▼搴忎腑纭紪鐮佹帴鍙?ID銆?
    - -  __u32
       - `intf_type`
       - 鎺ュ彛绫诲瀷锛岃瑙?media-intf-type銆?
    - -  __u32
       - `flags`
       - 鎺ュ彛鏍囧織銆傚綋鍓嶆湭浣跨敤銆?
    - -  __u32
       - `reserved`\ [^9^]
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍璁句负闆躲€?
    - -  struct media_v2_intf_devnode
       - `devnode`
       - 浠呯敤浜庤澶囪妭鐐规帴鍙ｃ€傝瑙?	  `media_v2_intf_devnode`銆?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `major`
       - 璁惧鑺傜偣涓昏澶囧彿銆?
    - -  __u32
       - `minor`
       - 璁惧鑺傜偣娆¤澶囧彿銆?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - pad 鐨勫敮涓€ ID銆備笉瑕佹湡鏈涜 ID 瀵硅澶囩殑姣忎釜瀹炰緥閮?	  濮嬬粓鐩稿悓銆傛崲鍙ヨ瘽璇达紝涓嶈鍦ㄥ簲鐢ㄧ▼搴忎腑纭紪鐮?pad ID銆?
    - -  __u32
       - `entity_id`
       - 姝?pad 鎵€灞炲疄浣撶殑鍞竴 ID銆?
    - -  __u32
       - `flags`
       - pad 鏍囧織锛岃瑙?media-pad-flag銆?
    - -  __u32
       - `index`
       - pad 绱㈠紩锛屼粠 0 寮€濮嬨€備粎褰?`MEDIA_V2_PAD_HAS_INDEX(media_version)`
	  杩斿洖 true 鏃舵湁鏁堛€俙media_version` 瀹氫箟浜?struct
	  `media_device_info` 涓紝鍙€氳繃 MEDIA_IOC_DEVICE_INFO 鑾峰彇銆?
    - -  __u32
       - `reserved`\ [^4^]
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍璁句负闆躲€?


    :header-rows:  0
    :stub-columns: 0
    :widths: 1 2 8

    - -  __u32
       - `id`
       - 閾炬帴鐨勫敮涓€ ID銆備笉瑕佹湡鏈涜 ID 瀵硅澶囩殑姣忎釜瀹炰緥閮?	  濮嬬粓鐩稿悓銆傛崲鍙ヨ瘽璇达紝涓嶈鍦ㄥ簲鐢ㄧ▼搴忎腑纭紪鐮侀摼鎺?ID銆?
    - -  __u32
       - `source_id`
       - 瀵逛簬 pad 鍒?pad 鐨勯摼鎺ワ細婧?pad 鐨勫敮涓€ ID銆?
	  瀵逛簬鎺ュ彛鍒板疄浣撶殑閾炬帴锛氭帴鍙ｇ殑鍞竴 ID銆?
    - -  __u32
       - `sink_id`
       - 瀵逛簬 pad 鍒?pad 鐨勯摼鎺ワ細sink pad 鐨勫敮涓€ ID銆?
	  瀵逛簬鎺ュ彛鍒板疄浣撶殑閾炬帴锛氬疄浣撶殑鍞竴 ID銆?
    - -  __u32
       - `flags`
       - 閾炬帴鏍囧織锛岃瑙?media-link-flag銆?
    - -  __u32
       - `reserved`\ [^6^]
       - 涓烘湭鏉ユ墿灞曚繚鐣欍€傞┍鍔ㄥ拰搴旂敤绋嬪簭蹇呴』灏嗚鏁扮粍璁句负闆躲€?
## 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞惰缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 涓€绔犱腑鎻忚堪銆?
ENOSPC
    褰?num_entities銆乶um_interfaces銆乶um_links 鎴?num_pads 涓湁涓€涓垨澶氫釜闈為浂锛屼笖
    灏忎簬鍥句腑瀹為檯鍏冪礌鏁伴噺鏃惰繑鍥炪€傚鏋?`topology_version` 涓庝笂娆¤皟鐢ㄦ ioctl 鏃剁浉姣?    鍙戠敓浜嗗彉鍖栵紝灏卞彲鑳藉彂鐢熻繖绉嶆儏鍐点€傜敤鎴风┖闂撮€氬父搴旈噴鏀炬寚閽堟墍鎸囧悜鐨勫尯鍩燂紝灏嗙粨鏋勪綋鍏冪礌
    娓呴浂锛岀劧鍚庡啀娆¤皟鐢ㄦ ioctl銆?