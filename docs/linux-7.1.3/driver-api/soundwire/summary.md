## SoundWire 瀛愮郴缁熸杩?

SoundWire 鏄?MIPI 鑱旂洘浜?2015 骞存壒鍑嗙殑涓€绉嶆柊鎺ュ彛銆係oundWire 鐢ㄤ簬浼犺緭閫氬父涓庨煶棰?鍔熻兘鐩稿叧鐨勬暟鎹€係oundWire 鎺ュ彛缁忚繃浼樺寲锛岄€傜敤浜庡湪绉诲姩鎴栧彈绉诲姩璁惧鍚彂鐨勭郴缁熶腑
闆嗘垚闊抽璁惧銆?
SoundWire 鏄竴绉嶅弻寮曡剼鐨勫鐐癸紙multi-drop锛夋帴鍙ｏ紝鍖呭惈鏁版嵁绾垮拰鏃堕挓绾裤€傚畠鏈夊姪浜?寮€鍙戜綆鎴愭湰銆侀珮鏁堛€侀珮鎬ц兘鐨勭郴缁熴€係oundWire 鎺ュ彛鐨勯珮灞傜骇鍏抽敭鐗规€у寘鎷細

 (1) 閫氳繃鍗曚竴鐨勫弻寮曡剼鎺ュ彛浼犺緭鎵€鏈夋湁鏁堣浇鑽锋暟鎹€氶亾銆佹帶鍒朵俊鎭拰寤虹珛鍛戒护銆?
 (2) 閫氳繃浣跨敤 DDR锛堝弻鏁版嵁閫熺巼锛夋暟鎹紶杈擄紝闄嶄綆鏃堕挓棰戠巼锛屼粠鑰岄檷浣庡姛鑰椼€?
 (3) 鏃堕挓缂╂斁鍜屽彲閫夌殑澶氭潯鏁版嵁閫氶亾锛屼互鏋佸ぇ鐨勭伒娲绘€у尮閰嶇郴缁熼渶姹傜殑鏁版嵁閫熺巼銆?
 (4) 璁惧鐘舵€佺洃鎺э紝鍖呮嫭瀵?Master 鐨勪腑鏂紡鍛婅銆?
SoundWire 鍗忚鏈€澶氭敮鎸佸崄涓€涓?Slave 鎺ュ彛銆傛墍鏈夋帴鍙ｅ叡浜寘鍚暟鎹嚎涓庢椂閽熺嚎鐨勫叕鍏辨€荤嚎銆?姣忎釜 Slave 鏈€澶氬彲鏀寔 14 涓暟鎹鍙ｃ€傚叾涓?13 涓暟鎹鍙ｄ笓鐢ㄤ簬闊抽浼犺緭銆?鏁版嵁绔彛 0 涓撶敤浜庝紶杈撴壒閲忔帶鍒朵俊鎭紝姣忎釜闊抽鏁版嵁绔彛锛?..14锛夊湪鍙戦€佹垨鎺ユ敹妯″紡涓?鏈€澶氬彲鏀寔 8 涓€氶亾锛堥€氬父涓哄浐瀹氭柟鍚戯紝浣嗚鑼冧篃鍏佽鍙厤缃柟鍚戯級銆備笉杩囷紝绾?19.2..24.576Mbits/s 鐨勫甫瀹介檺鍒朵笉鍏佽鍚屾椂浼犺緭 11**13**8 涓€氶亾銆?
涓嬪浘灞曠ず浜嗕竴涓?SoundWire Master 涓?```

        +---------------+                                       +---------------+
        |               |                       Clock Signal    |               |
        |    Master     |-------+-------------------------------|    Slave      |
        |   Interface   |       |               Data Signal     |  Interface 1  |
        |               |-------|-------+-----------------------|               |
        +---------------+       |       |                       +---------------+
                                |       |
                                |       |
                                |       |
                             +--+-------+--+
                             |             |
                             |   Slave     |
                             | Interface 2 |
                             |             |
                             +-------------+


```
## 鏈


MIPI SoundWire 瑙勮寖浣跨敤鏈 'device' 鏉ユ寚浠?Master 鎴?Slave 鎺ュ彛锛岃繖褰撶劧瀹规槗寮曡捣娣锋穯銆?鍦ㄦ湰姒傝堪鍜屼唬鐮佷腑锛屾垜浠粎浣跨敤鏈 interface 鏉ユ寚浠ｇ‖浠躲€傛垜浠伒寰?Linux 璁惧妯″瀷锛?灏嗘€荤嚎涓婅繛鎺ョ殑姣忎釜 Slave 鎺ュ彛鏄犲皠涓虹敱鐗瑰畾椹卞姩绠＄悊鐨?device銆侺inux SoundWire 瀛愮郴缁?鎻愪緵浜嗕竴涓鏋舵潵瀹炵幇 SoundWire Slave 椹卞姩锛屽苟鎻愪緵涓€涓?API锛屽厑璁哥涓夋柟鍘傚晢瀹炵幇
鑷畾涔夌殑銆佽鑼冨畾涔変箣澶栫殑鍔熻兘锛岃€岄€氱敤鐨勫缓绔?閰嶇疆浠诲姟鐢辨€荤嚎澶勭悊銆?
Bus锛堟€荤嚎锛夛細
瀹炵幇澶勭悊 SoundWire 鍗忚鐨?SoundWire Linux 鎬荤嚎銆傚鎵€鏈夌殑 MIPI 瀹氫箟鐨?Slave 瀵勫瓨鍣?杩涜缂栫▼銆備唬琛ㄤ竴涓?SoundWire Master銆傜郴缁熶腑鍙兘瀛樺湪鎬荤嚎鐨勫涓疄渚嬨€?
Slave锛堜粠璁惧锛夛細
娉ㄥ唽涓?SoundWire Slave 璁惧锛圠inux 璁惧锛夈€傚涓?Slave 璁惧鍙互娉ㄥ唽鍒颁竴涓€荤嚎瀹炰緥銆?
Slave driver锛堜粠璁惧椹卞姩锛夛細
鎺у埗 Slave 璁惧鐨勯┍鍔ㄣ€侻IPI 瑙勫畾鐨勫瘎瀛樺櫒鐢辨€荤嚎鐩存帴鎺у埗锛堝苟閫氳繃 Master 椹卞姩/鎺ュ彛浼犺緭锛夈€?浠讳綍瑙勮寖瀹氫箟涔嬪鐨?Slave 瀵勫瓨鍣ㄩ兘鐢?Slave 椹卞姩鎺у埗銆傚疄璺典腑锛岄鏈?Slave 椹卞姩渚濊禆
regmap锛岃€屼笉鐩存帴璇锋眰瀵勫瓨鍣ㄨ闂€?
## 缂栫▼鎺ュ彛锛圫oundWire 涓绘帴鍙ｉ┍鍔級


SoundWire 鎬荤嚎涓?SoundWire Master 瀹炵幇鍜?SoundWire Slave 璁惧鎻愪緵缂栫▼鎺ュ彛銆傛墍鏈変唬鐮?閮戒娇鐢?SoC 璁捐浜哄憳鍜岀涓夋柟鍘傚晢甯哥敤鐨?"sdw" 鍓嶇紑銆?
姣忎釜 SoundWire Master 鎺ュ彛閮介渶瑕佹敞鍐屽埌鎬荤嚎涓娿€傛€荤嚎瀹炵幇浜嗙敤浜庤鍙栨爣鍑?Master MIPI
灞炴€х殑 API锛屽苟鍦?Master ops 涓彁渚涘洖璋冿紝渚?Master 椹卞姩瀹炵幇鍏惰嚜韬彁渚涜兘鍔涗俊鎭殑鍑芥暟銆?鐩墠灏氭湭瀹炵幇 DT 鏀寔锛屼絾鐢变簬鑳藉姏鏄€氳繃 `device_property_` API 鍚敤鐨勶紝娣诲姞璧锋潵搴旇
寰堢畝鍗曘€?
Master 鎺ュ彛鍙婂叾鑳藉姏鍩轰簬 board 鏂囦欢銆丏T 鎴?ACPI 杩涜娉ㄥ唽銆?
浠ヤ笅鏄敤浜庢敞鍐?SoundWire 鎬荤嚎鐨勬€荤嚎 API锛?

	int sdw_bus_master_add(struct sdw_bus *bus,
				struct device *parent,
				struct fwnode_handle)
	{
		sdw_master_device_add(bus, parent, fwnode);

		mutex_init(&bus->lock);
		INIT_LIST_HEAD(&bus->slaves);

		/** Check ACPI for Slave devices **/
		sdw_acpi_find_slaves(bus);

		/** Check DT for Slave devices **/
		sdw_of_find_slaves(bus);

		return 0;
	}

杩欏皢涓?Master 璁惧鍒濆鍖?sdw_bus 瀵硅薄銆傚悜鎬荤嚎鎻愪緵 "sdw_master_ops" 鍜?"sdw_master_port_ops" 鍥炶皟鍑芥暟銆?
"sdw_master_ops" 鐢辨€荤嚎鐢ㄤ簬浠ョ‖浠剁壒瀹氱殑鏂瑰紡鎺у埗鎬荤嚎銆傚畠鍖呮嫭鎬荤嚎鎺у埗鍑芥暟锛屼緥濡?鍦ㄦ€荤嚎涓婂彂閫?SoundWire 璇?鍐欐秷鎭紝璁剧疆鏃堕挓棰戠巼鍜屾祦鍚屾鐐癸紙SSP锛夈€?sdw_master_ops"
缁撴瀯浣撳皢 Master 鐨勭‖浠剁粏鑺備粠鎬荤嚎涓娊璞″嚭鏉ャ€?
"sdw_master_port_ops" 鐢辨€荤嚎鐢ㄤ簬璁剧疆 Master 鎺ュ彛绔彛鐨勭鍙ｅ弬鏁般€侻aster 鎺ュ彛绔彛鐨?瀵勫瓨鍣ㄦ槧灏勫苟鏈敱 MIPI 瑙勮寖瀹氫箟锛屽洜姝ゆ€荤嚎璋冪敤 "sdw_master_port_ops" 鍥炶皟鍑芥暟鏉ユ墽琛?绔彛鎿嶄綔锛屼緥濡?"Port Prepare"銆?Port Transport params set"銆?Port enable and disable"銆?鐒跺悗 Master 椹卞姩鐨勫疄鐜板彲浠ユ墽琛岀‖浠剁壒瀹氱殑閰嶇疆銆?
## 缂栫▼鎺ュ彛锛圫oundWire 浠庤澶囬┍鍔級


MIPI 瑙勮寖瑕佹眰姣忎釜 Slave 鎺ュ彛鏆撮湶涓€涓敮涓€鐨?48 浣嶆爣璇嗙锛屽瓨鍌ㄥ湪 6 涓彧璇?dev_id
瀵勫瓨鍣ㄤ腑銆傝 dev_id 鏍囪瘑绗﹀寘鍚巶鍟嗗拰閮ㄤ欢淇℃伅锛屼互鍙婁竴涓敤浜庡尯鍒嗙浉鍚岀粍浠剁殑瀛楁銆?棰濆鐨?class 瀛楁鐩墠鏈娇鐢ㄣ€係lave 椹卞姩閽堝鐗瑰畾鐨勫巶鍟嗗拰閮ㄤ欢鏍囪瘑绗︾紪鍐欙紝鎬荤嚎鏍规嵁
杩欎袱涓?id 鏋氫妇 Slave 璁惧銆傝澶囦笌椹卞姩鐨勫尮閰嶅熀浜庤繖涓や釜 id 瀹屾垚銆傚綋璁惧涓庨┍鍔?id
鎴愬姛鍖归厤鏃讹紝鎬荤嚎璋冪敤 Slave 椹卞姩鐨?Probe銆侻aster 涓?Slave 璁惧涔嬮棿寮哄埗寤虹珛鐖跺瓙鍏崇郴
锛堥€昏緫琛ㄧず涓庣墿鐞嗚繛鎺ヤ繚鎸佷竴鑷达級銆?
Master/Slave 渚濊禆鍏崇郴鐨勪俊鎭瓨鍌ㄥ湪骞冲彴鏁版嵁銆乥oard 鏂囦欢銆丄CPI 鎴?DT 涓€侻IPI 杞欢瑙勮寖
涓烘嫢鏈夊涓?Master 鎺ュ彛鐨勬帶鍒跺櫒瀹氫箟浜嗛澶栫殑 link_id 鍙傛暟銆俤ev_id 瀵勫瓨鍣ㄤ粎鍦?link 鐨?鑼冨洿鍐呭敮涓€锛宭ink_id 鍦ㄦ帶鍒跺櫒鐨勮寖鍥村唴鍞竴銆俤ev_id 鍜?link_id 鍦ㄧ郴缁熺骇鍒笂閮戒笉涓€瀹氬敮涓€锛?浣嗙埗瀛愪俊鎭敤浜庨伩鍏嶆涔夈€?

	static const struct sdw_device_id slave_id[] = {
	        SDW_SLAVE_ENTRY(0x025d, 0x700, 0),
	        {},
	};
	MODULE_DEVICE_TABLE(sdw, slave_id);

	static struct sdw_driver slave_sdw_driver = {
	        .driver = {
	                   .name = "slave_xxx",
	                   .pm = &slave_runtime_pm,
	                   },
		.probe = slave_sdw_probe,
		.remove = slave_sdw_remove,
		.ops = &slave_slave_ops,
		.id_table = slave_id,
	};


瀵逛簬鑳藉姏锛屾€荤嚎瀹炵幇浜嗙敤浜庤鍙栨爣鍑?Slave MIPI 灞炴€х殑 API锛屽苟鍦?Slave ops 涓彁渚涘洖璋冿紝
渚?Slave 椹卞姩瀹炵幇鎻愪緵鑳藉姏淇℃伅鐨勮嚜韬嚱鏁般€傛€荤嚎闇€瑕佺煡閬撲竴缁?Slave 鑳藉姏锛屼互渚垮 Slave
瀵勫瓨鍣ㄨ繘琛岀紪绋嬪苟鎺у埗鎬荤嚎鐨勯噸鏂伴厤缃€?
## 閾炬帴


SoundWire MIPI 瑙勮寖 1.1 鍙湪浠ヤ笅鍦板潃鑾峰彇锛?https://members.mipi.org/wg/All-Members/document/70290

SoundWire MIPI DisCo锛圖iscover and Configuration锛屽彂鐜颁笌閰嶇疆锛夎鑼冨彲鍦ㄤ互涓嬪湴鍧€鑾峰彇锛?https://www.mipi.org/specifications/mipi-disco-soundwire

锛堟敞鍐屽悗鍙叕寮€璁块棶锛孧IPI 鎴愬憳鍙洿鎺ヨ闂級

MIPI 鑱旂洘鍘傚晢 ID 椤甸潰锛歮id.mipi.org
