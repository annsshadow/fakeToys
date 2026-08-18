## 鍐呮牳椹卞姩 occ-hwmon


鏀寔鑺墖锛?
  - POWER8
  - POWER9

Author: Eddie James <eajames@linux.ibm.com>

### 鎻忚堪


鏈┍鍔ㄦ敮鎸佸宓屽叆鍦?POWER 澶勭悊鍣ㄤ笂鐨勭墖涓婃帶鍒跺櫒锛圤CC锛夎繘琛岀‖浠剁洃鎺с€侽CC 鏄竴涓粠
澶勭悊鍣ㄥ拰绯荤粺鏀堕泦骞惰仛鍚堜紶鎰熷櫒鏁版嵁鐨勮澶囥€侽CC 鏃㈠彲浠ユ彁渚涘師濮嬩紶鎰熷櫒鏁版嵁锛屼篃鍙互鍦ㄧ郴缁熶笂
鎵ц鏁ｇ儹涓庣數婧愮鐞嗐€?
鏈┍鍔ㄧ殑 P8 鐗堟湰鏄?I2C 鐨勪竴涓鎴风椹卞姩銆傚鏋滃湪璁惧鏍戠殑鐩稿簲 I2C 鎬荤嚎鑺傜偣涓嬫壘鍒?"ibm,p8-occ-hwmon" 鍏煎璁惧锛屽彲浠ユ墜鍔ㄦ帰娴嬪畠銆?
鏈┍鍔ㄧ殑 P9 鐗堟湰鏄熀浜?FSI 鐨?OCC 椹卞姩鐨勫鎴风椹卞姩銆傚畠灏嗙敱鍩轰簬 FSI 鐨?OCC 椹卞姩鑷姩
鎺㈡祴銆?
### Sysfs 鏉＄洰


鏀寔浠ヤ笅灞炴€с€傞櫎闈炵壒鍒鏄庯紝鎵€鏈夊睘鎬ч兘鏄彧璇荤殑銆?
OCC 浼犳劅鍣?ID 鏄竴涓暣鏁帮紝琛ㄧず鐩稿浜?OCC 鐨勪紶鎰熷櫒鐨勫敮涓€鏍囪瘑绗︺€備緥濡傦紝绯荤粺涓涓変釜
DIMM 鎻掓Ы鐨勬俯搴︿紶鎰熷櫒鍙兘鍏锋湁浼犳劅鍣?ID 7銆傝澶囬┍鍔ㄦ棤娉曡幏鍙栨鏄犲皠锛屽洜姝ゅ繀椤诲師鏍峰鍑?浼犳劅鍣?ID銆?
鏌愪簺鏉＄洰浠呭湪鏌愪簺 OCC 浼犳劅鍣ㄧ増鏈笅鍑虹幇锛屾垨浠呭嚭鐜板湪绯荤粺涓殑鏌愪簺 OCC 涓娿€傜増鏈彿涓嶅鍑?缁欑敤鎴凤紝浣嗗彲浠ユ帹鏂€?
temp[1-n]_label
	OCC 浼犳劅鍣?ID銆?
[with temperature sensor version 1]

    temp[1-n]_input
			浠ュ崈鍒嗗害鎽勬皬搴︽祴閲忕殑缁勪欢娓╁害銆?
[with temperature sensor version >= 2]

    temp[1-n]_type
			FRU锛堢幇鍦哄彲鏇存崲鍗曞厓锛夌被鍨?			锛堢敱鏁存暟琛ㄧず锛夛紝琛ㄧず姝や紶鎰熷櫒鎵€娴嬮噺鐨勭粍浠躲€?    temp[1-n]_fault
			娓╁害浼犳劅鍣ㄦ晠闅滃竷灏斿€硷紱1 琛ㄧず瀛樺湪鏁呴殰锛?			0 琛ㄧず涓嶅瓨鍦ㄦ晠闅溿€?
    [with type == 3 (FRU type is VRM)]

	temp[1-n]_alarm
			VRM 娓╁害鍛婅甯冨皵鍊硷紱1 琛ㄧず鍛婅锛? 琛ㄧず鏃犲憡璀︺€?
    [else]

	temp[1-n]_input
			浠ュ崈鍒嗗害鎽勬皬搴︽祴閲忕殑缁勪欢娓╁害銆?
freq[1-n]_label
			OCC 浼犳劅鍣?ID銆?freq[1-n]_input
			浠?MHz 娴嬮噺鐨勭粍浠堕鐜囥€?power[1-n]_input
			缁勪欢鏈€鏂版祴閲忕殑鍔熺巼璇绘暟锛屽崟浣?microwatts銆?power[1-n]_average
			缁勪欢鐨勫钩鍧囧姛鐜囷紝鍗曚綅 microwatts銆?power[1-n]_average_interval
			鍙栧姛鐜囧钩鍧囧€兼墍缁忚繃鐨勬椂闂达紝鍗曚綅寰銆?
[with power sensor version < 2]

    power[1-n]_label
			OCC 浼犳劅鍣?ID銆?
[with power sensor version >= 2]

    power[1-n]_label
			OCC 浼犳劅鍣?ID + 鍔熻兘 ID + 閫氶亾锛屽舰寮忎负瀛楃涓诧紝
			浠ヤ笅鍒掔嚎鍒嗛殧锛屽嵆 "0_15_1"銆傚姛鑳?ID 鍜岄€氶亾閮芥槸
			鏁存暟锛岀敤浜庤繘涓€姝ユ爣璇嗗姛鐜囦紶鎰熷櫒銆?
[with power sensor version 0xa0]

    power[1-n]_label
			OCC 浼犳劅鍣?ID + 浼犳劅鍣ㄧ被鍨嬶紝褰㈠紡涓哄瓧绗︿覆锛?			浠ヤ笅鍒掔嚎鍒嗛殧锛屽嵆 "0_system"銆備紶鎰熷櫒绫诲瀷灏嗘槸
			"system"銆?proc"銆?vdd" 鎴?"vdn" 涔嬩竴銆傚浜庢
			浼犳劅鍣ㄧ増鏈紝鎵€鏈夊姛鐜囦紶鎰熷櫒鐨?OCC 浼犳劅鍣?ID 閮界浉鍚屻€?
[浠呭湪 "master" OCC 涓婂嚭鐜帮紱琛ㄧず鏁翠釜绯荤粺鐨勫姛鐜囷紱姝ょ被鍔熺巼浼犳劅鍣ㄥ彧浼氭湁涓€涓猐

    power[1-n]_label
			"system"
    power[1-n]_input
			鏈€鏂扮殑绯荤粺杈撳嚭鍔熺巼锛屽崟浣?microwatts銆?    power[1-n]_cap
			褰撳墠绯荤粺鍔熺巼涓婇檺锛屽崟浣?microwatts銆?    power[1-n]_cap_not_redundant
			鏃犲啑浣欑數婧愭椂鐨勭郴缁熷姛鐜囦笂闄愶紝鍗曚綅 microwatts銆?    power[1-n]_cap_max
			OCC 鍙互寮哄埗鎵ц鐨勬渶澶у姛鐜囦笂闄愶紝鍗曚綅 microwatts銆?    power[1-n]_cap_min		OCC 鍙互寮哄埗鎵ц鐨勬渶灏忓姛鐜囦笂闄愶紝鍗曚綅
			microwatts銆?    power[1-n]_cap_user		鐢ㄦ埛璁剧疆鐨勫姛鐜囦笂闄愶紝鍗曚綅 microwatts銆?			濡傛灉娌℃湁璁剧疆鐢ㄦ埛鍔熺巼涓婇檺锛屾灞炴€у皢杩斿洖 0銆傛灞炴€?			鏄彲璇诲啓鐨勶紝浣嗕綆浜庣摝鐗圭殑浠讳綍绮惧害鍐欏叆閮藉皢琚拷鐣ワ紝
			鍗宠姹?500900000 microwatts 鐨勫姛鐜囦笂闄愬皢瀵艰嚧
			涓€涓?500 鐡︾殑鍔熺巼涓婇檺璇锋眰銆?
    [with caps sensor version > 1]

	power[1-n]_cap_user_source
				鎸囩ず鐢ㄦ埛鍔熺巼涓婇檺鏄浣曡缃殑銆傝繖鏄竴涓?				鏁存暟锛屾槧灏勫埌鍙互璁剧疆鐢ㄦ埛鍔熺巼涓婇檺鐨?				绯荤粺鎴栧浐浠剁粍浠躲€?
浠ヤ笅 "extn" 浼犳劅鍣ㄨ瀵煎嚭锛屼綔涓?OCC 鎻愪緵涓嶉€傚悎浠讳綍鍏朵粬鍦版柟鐨勬暟鎹殑涓€绉嶆柟寮忋€傝繖浜?浼犳劅鍣ㄧ殑鍚箟瀹屽叏鍙栧喅浜庡叾鏁版嵁锛屾棤娉曢潤鎬佸畾涔夈€?
extn[1-n]_label
			ASCII ID 鎴?OCC 浼犳劅鍣?ID銆?extn[1-n]_flags
			杩欐槸涓€涓崟瀛楄妭鍗佸叚杩涘埗鍊笺€備綅 7 鎸囩ず label 灞炴€?			鐨勭被鍨嬶紱1 琛ㄧず浼犳劅鍣?ID锛? 琛ㄧず ASCII ID銆傚叾浠栦綅淇濈暀銆?extn[1-n]_input
			6 瀛楄妭鍗佸叚杩涘埗鏁版嵁锛屽惈涔夌敱浼犳劅鍣?ID 瀹氫箟銆?