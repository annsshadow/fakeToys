## Charger Manager锛堝厖鐢电鐞嗗櫒锛?

	(C) 2011 MyungJoo Ham <myungjoo.ham@samsung.com>, GPL

Charger Manager 鎻愪緵鍐呮牳鍐呯殑鐢垫睜鍏呯數绠＄悊锛屽畠闇€瑕佸湪鎸傝捣鑷?RAM锛坰uspend-to-RAM锛夌姸鎬佷笅杩涜娓╁害鐩戞帶锛屽苟涓旀瘡鍧楃數姹犲彲鑳芥寕鎺ュ涓厖鐢靛櫒锛岃€岀敤鎴风┖闂村笇鏈涙煡鐪嬭繖澶氫釜鍏呯數鍣ㄧ殑鑱氬悎淇℃伅銆?
Charger Manager 鏄竴涓甫鏈?power-supply-class 鏉＄洰鐨?platform_driver銆侰harger Manager 鐨勪竴涓疄渚嬶紙鐢?Charger-Manager 鍒涘缓鐨?platform-device锛変唬琛ㄤ竴鍧楀甫鏈夊厖鐢靛櫒鐨勭嫭绔嬬數姹犮€傚鏋滀竴涓郴缁熶腑鏈夊鍧楃數姹犲悇鑷甫鏈夌嫭绔嬪伐浣滅殑鍏呯數鍣紝璇ョ郴缁熷彲鑳介渶瑕佸涓?Charger Manager 瀹炰緥銆?
## 1. 绠€浠?

Charger Manager 鏀寔浠ヤ笅鍔熻兘锛?
- 鏀寔澶氫釜鍏呯數鍣紙渚嬪锛屽甫鏈?USB銆丄C 鍜屽お闃宠兘鏉跨殑璁惧锛?	涓€涓郴缁熷彲鑳芥湁澶氫釜鍏呯數鍣紙鎴栫數婧愶級锛屽叾涓儴鍒嗗彲鑳藉悓鏃舵縺娲汇€傛瘡涓厖鐢靛櫒鍙互鎷ユ湁鑷繁鐨?power-supply-class锛岃€屾瘡涓?power-supply-class 鍙互鎻愪緵鍏充簬鐢垫睜鐘舵€佺殑涓嶅悓淇℃伅銆傝妗嗘灦浠庡涓潵婧愯仛鍚堜笌鍏呯數鍣ㄧ浉鍏崇殑淇℃伅锛屽苟浠ュ崟涓€ power-supply-class 鐨勫舰寮忓睍绀哄悎骞跺悗鐨勪俊鎭€?
- 鏀寔鎸傝捣鑷?RAM 鏈熼棿鐨勮疆璇紙鍊熷姪 suspend_again 鍥炶皟锛?	鍦ㄧ數姹犲厖鐢典笖绯荤粺澶勪簬 suspend-to-RAM 鏃讹紝鎴戜滑鍙兘闇€瑕侀€氳繃鏌ョ湅鐜鎴栫數姹犳俯搴︽潵鐩戞帶鐢垫睜鍋ュ悍銆傛垜浠彲浠ラ€氳繃鍛ㄦ湡鎬у敜閱掔郴缁熸潵瀹炵幇銆傜劧鑰岋紝杩欑鏂规硶浼氫负鐩戞帶鐢垫睜鍋ュ悍鍜屼换鍔¤€屽敜閱掍笉蹇呰鐨勮澶囷紝浠ュ強鏈簲淇濇寔鎸傝捣鐨勭敤鎴疯繘绋嬨€傝繖鍙嶈繃鏉ヤ細瀵艰嚧涓嶅繀瑕佺殑鍔熻€楋紝骞舵嫋鎱㈠厖鐢佃繃绋嬨€傜敋鑷筹紝杩欑宄板€煎姛鑰楀彲鑳藉湪鍏呯數涓€斿仠姝㈠厖鐢靛櫒锛堝閮ㄨ緭鍏ュ姛鐜?< 璁惧鍔熻€楋級锛岃繖涓嶄粎褰卞搷鍏呯數鏃堕棿锛屼篃褰卞搷鐢垫睜瀵垮懡銆?
	Charger Manager 鎻愪緵涓€涓嚱鏁?鈥渃m_suspend_again鈥濓紝鍙敤浣?platform_suspend_ops 鐨?suspend_again 鍥炶皟銆傚鏋滃钩鍙伴渶瑕侀櫎 cm_suspend_again 涔嬪鐨勫叾浠栦换鍔★紝瀹冨彲浠ュ疄鐜拌嚜宸辩殑 suspend_again 鍥炶皟锛屽湪涓棿璋冪敤 cm_suspend_again銆傞€氬父锛屽钩鍙伴渶瑕佹仮澶嶅苟鎸傝捣 Charger Manager 浣跨敤鐨勪竴浜涜澶囥€?
- 鏀寔鎻愬墠鐨勬弧鐢典簨浠跺鐞?	濡傛灉鍦ㄦ弧鐢典簨浠朵箣鍚庣粡杩?鈥渇ullbatt_vchkdrop_ms鈥濓紝鐢垫睜鐢靛帇涓嬮檷浜?鈥渇ullbatt_vchkdrop_uV鈥濓紝妗嗘灦灏嗛噸鏂板惎鍔ㄥ厖鐢点€傝妫€鏌ヤ篃浼氬湪鎸傝捣鏈熼棿閫氳繃璁剧疆鐩稿簲鐨勫敜閱掓椂闂村苟鍊熷姪 suspend_again 鏉ユ墽琛屻€?
- 鏀寔 uevent 閫氱煡
	鍦ㄥ厖鐢靛櫒鐩稿叧浜嬩欢鍙戠敓鏃讹紝璁惧鍚戠敤鎴峰彂閫?UEVENT 閫氱煡銆?
## 2. 涓?suspend_again 鐩稿叧鐨勫叏灞€ Charger-Manager 鏁版嵁

涓轰簡涓?Charger Manager 閰嶇疆 suspend_again 鐗规€э紙鎸傝捣涓洃鎺э級锛岀敤鎴峰簲鎻愪緵 charger_global_desc锛屽苟閫氳繃 setup_charger_manager(`struct charger_global_desc *`) 杩涜璁剧疆銆傞【鍚嶆€濅箟锛岃繖涓敤浜庢寕璧蜂腑鐩戞帶鐨?charger_global_desc 鏁版嵁鏄叏灞€鐨勩€傚洜姝わ紝鍗充娇鏈夊鍧楃數姹狅紝鐢ㄦ埛涔熷彧闇€鎻愪緵涓€娆°€傚鏋滄湁澶氫釜鐢垫睜锛屽涓?Charger Manager 瀹炰緥鍏变韩鍚屼竴涓?charger_global_desc锛屽畠灏嗕负鎵€鏈?Charger Manager 瀹炰緥绠＄悊鎸傝捣涓洃鎺с€?
鐢ㄦ埛闇€瑕佹纭湴涓?`struct charger_global_desc` 鎻愪緵鍏ㄩ儴涓変釜鏉＄洰锛屾墠鑳芥縺娲绘寕璧蜂腑鐩戞帶锛?
`char *rtc_name;`
	鐢ㄤ簬浠庢寕璧蜂腑鍞ら啋绯荤粺鐨?rtc 鍚嶇О锛堜緥濡?鈥渞tc0鈥濓級銆俽tc 鐨勯椆閽熶腑鏂紙AIE锛夊簲褰撹兘澶熷敜閱掔郴缁熴€侰harger Manager 浼氫繚瀛樺苟鎭㈠闂归挓鍊硷紝骞跺湪闂归挓灏嗘瘮 Charger Manager 璁惧畾鐨勬洿鏃╄Е鍙戞椂浣跨敤鍏堝墠瀹氫箟鐨勯椆閽燂紝浠庤€屼笉骞叉壈鍏堝墠瀹氫箟鐨勯椆閽熴€?
`bool (*rtc_only_wakeup)(void);`
	璇ュ洖璋冨簲璁?CM 鐭ラ亾浠庢寕璧蜂腑鍞ら啋鏄惁浠呯敱鍚屼竴缁撴瀯浣撲腑鐨?鈥渞tc鈥?闂归挓寮曡捣銆傚鏋滄湁浠讳綍鍏朵粬鍞ら啋婧愯Е鍙戜簡鍞ら啋锛屽畠搴旇繑鍥?false銆傚鏋?鈥渞tc鈥?鏄敮涓€鐨勫敜閱掑師鍥狅紝瀹冨簲杩斿洖 true銆?
`bool assume_timer_stops_in_suspend;`
	濡傛灉涓?true锛孋harger Manager 鍋囧畾瀹氭椂鍣紙CM 浣跨敤 jiffies 浣滀负瀹氭椂鍣級鍦ㄦ寕璧锋湡闂村仠姝€傞偅涔堬紝CM 鍋囧畾鎸傝捣鏃堕暱涓庨椆閽熼暱搴︾浉鍚屻€?
## 3. 濡備綍閰嶇疆 suspend_again

Charger Manager 鎻愪緵鍑芥暟 鈥渆xtern bool cm_suspend_again(void)鈥濄€傚綋璋冪敤 cm_suspend_again 鏃讹紝瀹冧細鐩戞帶姣忎竴鍧楃數姹犮€傜郴缁?platform_suspend_ops 鐨?suspend_ops 鍥炶皟鍙互璋冪敤 cm_suspend_again 鍑芥暟锛屼互浜嗚В Charger Manager 鏄惁甯屾湜鍐嶆鎸傝捣銆傚鏋滄病鏈夊叾浠栬澶囨垨浠诲姟鎯充娇鐢?suspend_again 鐗规€э紝platform_suspend_ops 鍙互鐩存帴灏嗗叾 suspend_again 鍥炶皟鎸囧悜 cm_suspend_again銆?
濡傛灉绯荤粺鐢?Charger Manager 鍞ら啋涓旇疆璇紙鎸傝捣涓洃鎺э級缁撴灉涓?鈥渘ormal鈥濓紝cm_suspend_again() 杩斿洖 true锛堟剰涓衡€滄垜甯屾湜鍐嶆鎸傝捣鈥濓級銆?
## 4. Charger-Manager 鏁版嵁锛坰truct charger_desc锛?
瀵逛簬姣忓潡鐙珛鍏呯數鐨勭數姹狅紙濡傛灉涓€绯诲垪鐢垫睜鐢卞崟涓厖鐢靛櫒鍏呯數锛屽垯瀹冧滑绠椾綔涓€鍧楃嫭绔嬬數姹狅級锛屼細鎸傛帴涓€涓?Charger Manager 瀹炰緥銆備笅鍒?
struct charger_desc 鍏冪礌锛?
`char *psy_name;`
	鐢垫睜鐨?power-supply-class 鍚嶇О銆傝嫢 psy_name 涓?NULL锛岄粯璁や负 鈥渂attery鈥濄€傜敤鎴峰彲鍦?鈥?sys/class/power_supply/[psy_name]/鈥?璁块棶 psy 鏉＄洰銆?
`enum polling_modes polling_mode;`
	  CM_POLL_DISABLE:
		涓嶈疆璇㈣鐢垫睜銆?	  CM_POLL_ALWAYS:
		濮嬬粓杞璇ョ數姹犮€?	  CM_POLL_EXTERNAL_POWER_ONLY:
		褰撲笖浠呭綋鎸傛帴浜嗗閮ㄧ數婧愭椂鎵嶈疆璇㈣鐢垫睜銆?	  CM_POLL_CHARGING_ONLY:
		褰撲笖浠呭綋鐢垫睜姝ｅ湪鍏呯數鏃舵墠杞璇ョ數姹犮€?
`unsigned int fullbatt_vchkdrop_ms; / unsigned int fullbatt_vchkdrop_uV;`
	鑻ヤ袱鑰呴兘鍏锋湁闈為浂鍊硷紝Charger Manager 浼氬湪鐢垫睜鍏呮弧鍚庣粡杩?fullbatt_vchkdrop_ms 妫€鏌ョ數姹犵數鍘嬩笅闄嶃€傚鏋滅數鍘嬩笅闄嶈秴杩?fullbatt_vchkdrop_uV锛孋harger Manager 灏嗗皾璇曢€氳繃绂佺敤骞堕噸鏂板惎鐢ㄥ厖鐢靛櫒鏉ュ鐢垫睜閲嶆柊鍏呯數銆備粎鏍规嵁鐢靛帇涓嬮檷鏉′欢锛堜笉甯﹀欢杩熸潯浠讹級閲嶆柊鍏呯數锛岄渶瑕佸€熷姪鏉ヨ嚜鐢甸噺璁℃垨鍏呯數鍣ㄨ澶?鑺墖鐨勭‖浠朵腑鏂潵瀹炵幇銆?
`unsigned int fullbatt_uV;`
	濡傛灉鎸囧畾浜嗛潪闆跺€硷紝Charger Manager 鍋囧畾褰撶數姹犳湭琚厖鐢典笖鐢垫睜鐢靛帇绛変簬鎴栧ぇ浜?fullbatt_uV 鏃讹紝鐢垫睜宸插厖婊★紙瀹归噺 = 100锛夈€?
`unsigned int polling_interval_ms;`
	鎵€闇€鐨勮疆璇㈤棿闅旓紙姣锛夈€侰harger Manager 浼氭瘡 polling_interval_ms 鎴栨洿棰戠箒鍦拌疆璇㈣鐢垫睜銆?
`enum data_source battery_present;`
	CM_BATTERY_PRESENT:
		鍋囧畾鐢垫睜瀛樺湪銆?	CM_NO_BATTERY:
		鍋囧畾鐢垫睜涓嶅瓨鍦ㄣ€?	CM_FUEL_GAUGE:
		浠庣數閲忚鑾峰彇鐢垫睜瀛樺湪淇℃伅銆?	CM_CHARGER_STAT:
		浠庡厖鐢靛櫒鑾峰彇鐢垫睜瀛樺湪淇℃伅銆?
`char **psy_charger_stat;`
	浠?NULL 缁撳熬鐨勬暟缁勶紝鍖呭惈鍏呯數鍣ㄧ殑 power-supply-class 鍚嶇О銆傛瘡涓?power-supply-class 搴旀彁渚?鈥淧RESENT鈥濓紙鑻?battery_present 涓?鈥淐M_CHARGER_STAT鈥濓級銆佲€淥NLINE鈥濓紙鏄剧ず鏄惁鎸傛帴浜嗗閮ㄧ數婧愶級鍜?鈥淪TATUS鈥濓紙鏄剧ず鐢垫睜鏄惁 {鈥淔ULL鈥?鎴?鏈弧} 鎴?{鈥淔ULL鈥濄€佲€淐harging鈥濄€佲€淒ischarging鈥濄€佲€淣otCharging鈥潁锛夈€?
`int num_charger_regulators; / struct regulator_bulk_data *charger_regulators;`
	浠?regulator 妗嗘灦鎵归噺鍑芥暟褰㈠紡琛ㄧず鍏呯數鍣ㄧ殑璋冭妭鍣ㄣ€?
`char *psy_fuel_gauge;`
	鐢甸噺璁＄殑 power-supply-class 鍚嶇О銆?
`int (**temperature_out_of_range)(int **mC); / bool measure_battery_temp;`
	濡傛灉娓╁害瀵瑰厖鐢垫槸瀹夊叏鐨勶紝璇ュ洖璋冭繑鍥?0锛涘鏋滆繃鐑棤娉曞厖鐢碉紝杩斿洖姝ｆ暟锛涘鏋滆繃鍐锋棤娉曞厖鐢碉紝杩斿洖璐熸暟銆傚€熷姪鍙橀噺 mC锛岃鍥炶皟浠ユ憚姘忓害鐨勫崈鍒嗕箣涓€杩斿洖娓╁害銆傛牴鎹?measure_battery_temp 鐨勫€硷紝娓╁害鏉ユ簮鍙互鏄數姹犳俯搴︽垨鐜娓╁害銆?
## 5. 鍏朵粬娉ㄦ剰浜嬮」


鍦ㄥ厖鐢靛櫒/鐢垫睜鐩稿叧浜嬩欢锛堝鐢垫睜鎷斿嚭銆佸厖鐢靛櫒鎷斿嚭銆佸厖鐢靛櫒鎻掑叆銆丏CIN 杩囧帇/娆犲帇銆佸厖鐢靛櫒鍋滄锛変互鍙婂叾浠栧鍏呯數鍣ㄨ嚦鍏抽噸瑕佺殑鎯呭喌涓嬶紝绯荤粺搴旇閰嶇疆涓哄敜閱掋€傝嚦灏戜互涓嬩簨浠跺簲灏嗙郴缁熶粠鎸傝捣涓敜閱掞細a) 鍏呯數鍣ㄥ紑/鍏?b) 澶栭儴鐢垫簮鎺ュ叆/鏂紑 c) 鐢垫睜瑁呭叆/鍙栧嚭锛堝厖鐢垫湡闂达級

杩欓€氬父閫氳繃灏?PMIC 閰嶇疆涓哄敜閱掓簮鏉ュ疄鐜般€?