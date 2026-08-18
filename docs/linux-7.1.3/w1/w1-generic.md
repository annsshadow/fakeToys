## 1-wire锛坵1锛夊瓙绯荤粺绠€浠?

1-wire 鎬荤嚎鏄竴绉嶇畝鍗曠殑涓讳粠鎬荤嚎锛屽畠閫氳繃涓€鏍逛俊鍙风嚎锛堝鍔犲湴绾匡紝鎵€浠ユ槸涓ゆ牴绾匡級杩涜閫氫俊銆?

璁惧閫氳繃鍦ㄦ€荤嚎涓婇€氳繃寮€婕忚緭鍑烘妸淇″彿鎷変綆鍒板湴锛屽苟瀵逛俊鍙风嚎鐨勯€昏緫鐢靛钩杩涜閲囨牱鏉ヨ繘琛岄€氫俊銆?

w1 瀛愮郴缁熸彁渚涗簡绠＄悊 w1 涓昏澶囷紙master锛変互鍙婁笌鍏朵粠璁惧锛坰lave锛夐€氫俊鐨勬鏋躲€?

鎵€鏈?w1 浠庤澶囬兘蹇呴』杩炴帴鍒颁竴涓?w1 鎬荤嚎涓昏澶囥€?

w1 涓昏澶囩ず渚嬶細

    - DS9490 usb device
    - W1-over-GPIO
    - DS2482 (i2c to w1 bridge)
    - Emulated devices, such as a RS232 converter, parallel port adapter, etc

### w1 瀛愮郴缁熷仛浜嗕粈涔堬紵

褰撲竴涓?w1 涓昏澶囬┍鍔ㄥ悜 w1 瀛愮郴缁熸敞鍐屾椂锛屼細鍙戠敓浠ヤ笅浜嬫儏锛?

 - 涓鸿 w1 涓昏澶囧垱寤?sysfs 鏉＄洰
 - 鍛ㄦ湡鎬у湴鍦?w1 鎬荤嚎涓婃悳绱㈡柊鐨勪粠璁惧

褰撳湪鎬荤嚎涓婂彂鐜颁竴涓澶囨椂锛寃1 鏍稿績浼氬皾璇曚负瀹冪殑鏃忥紙family锛夊姞杞介┍鍔ㄥ苟妫€鏌ュ畠鏄惁宸插姞杞姐€傚鏋滃凡鍔犺浇锛屽垯璇ユ棌椹卞姩琚粦瀹氬埌璇ヤ粠璁惧銆傚鏋滆鏃忔病鏈夐┍鍔紝鍒欎細鍒嗛厤涓€涓粯璁ら┍鍔紝瀹冨嚑涔庡彲浠ユ墽琛屼换浣曠绫荤殑鎿嶄綔銆傛瘡涓€昏緫鎿嶄綔鏈川涓婇兘鏄竴涓簨鍔★紝鍏朵腑鍙互鍖呭惈鑻ュ共涓紙涓や釜鎴栦竴涓級搴曞眰鎿嶄綔銆傛垜浠潵鐪嬩竴涓嬪浣曡鍙?EEPROM 鍐呭锛?
1. 蹇呴』鍐欏叆鎺у埗缂撳啿鍖猴紝鍗冲寘鍚懡浠ゅ瓧鑺傚拰涓や釜瀛楄妭鍦板潃鐨勭紦鍐插尯銆傚湪杩欎竴姝ヤ腑锛屾€荤嚎琚浣嶅苟浣跨敤 W1_SKIP_ROM 鎴?W1_MATCH_ROM 鍛戒护閫変腑鐩稿簲鐨勮澶囥€傜劧鍚庢彁渚涚殑鎺у埗缂撳啿鍖鸿鍐欏叆鎬荤嚎銆?
2. 璇诲彇銆傝繖灏嗗彂鍑鸿鍙?eeprom 鐨勫搷搴斻€?

鍦?1. 鍜?2. 涔嬮棿锛寃1 涓昏澶囩嚎绋嬪彲鑳戒細澶嶄綅鎬荤嚎浠ヨ繘琛屾悳绱紝浠庤澶囩敋鑷充細琚Щ闄わ紝浣嗗湪杩欑鎯呭喌涓嬩細璇诲埌 0xff锛屽洜涓烘病鏈夐€変腑浠讳綍璁惧銆?

### w1 璁惧鏃?

浠庤澶囩敱涓烘煇涓?w1 璁惧鏃忕紪鍐欑殑椹卞姩鏉ュ鐞嗐€?

涓€涓棌椹卞姩濉厖涓€涓?struct w1_family_ops锛堣 w1_family.h锛夊苟鍚?w1 瀛愮郴缁熸敞鍐屻€?

褰撳墠鐨勬棌椹卞姩锛?

w1_therm
  - 锛坉s18?20 娓╁害浼犳劅鍣ㄦ棌椹卞姩锛?
    鎻愪緵娓╁害璇诲彇鍑芥暟锛屽畠琚粦瀹氬埌涓婅堪 w1_family_ops 缁撴瀯鐨?->rbin() 鏂规硶涓娿€?

w1_smem
  - 鐢ㄤ簬绠€鍗?64 浣嶅瓨鍌ㄥ崟鍏冪殑椹卞姩锛屾彁渚?ID 璇诲彇鏂规硶銆?

浣犲彲浠ラ€氳繃璇诲彇鐩稿簲鐨?sysfs 鏂囦欢鏉ヨ皟鐢ㄤ笂杩版柟娉曘€?

### w1 涓昏澶囬┍鍔ㄩ渶瑕佸疄鐜颁粈涔堬紵

w1 鎬荤嚎涓昏澶囩殑椹卞姩鑷冲皯蹇呴』鎻愪緵涓や釜鍑芥暟銆?

妯℃嫙璁惧蹇呴』鎻愪緵璁剧疆杈撳嚭淇″彿鐢靛钩锛坵rite_bit锛夊拰閲囨牱淇″彿鐢靛钩锛坮ead_bit锛夌殑鑳藉姏銆?

鍘熺敓鏀寔 1-wire 鐨勮澶囧繀椤绘彁渚涘啓鍏ュ拰閲囨牱涓€涓瘮鐗癸紙touch_bit锛変互鍙婂浣嶆€荤嚎锛坮eset_bus锛夌殑鑳藉姏銆?

澶у鏁扮‖浠舵彁渚涙洿楂樺眰鐨勫嚱鏁帮紝灏?w1 澶勭悊宸ヤ綔鍗歌浇鎺夈€傝瑙?w1.h 涓殑 struct w1_bus_master 瀹氫箟銆?

### w1 涓昏澶?sysfs 鎺ュ彛

========================= =====================================================
<xx-xxxxxxxxxxxx>         A directory for a found device. The format is
                          family-serial
bus                       (standard) symlink to the w1 bus
driver                    (standard) symlink to the w1 driver
w1_master_add             (rw) manually register a slave device
w1_master_attempts        (ro) the number of times a search was attempted
w1_master_max_slave_count (rw) maximum number of slaves to search for at a time
w1_master_name            (ro) the name of the device (w1_bus_masterX)
w1_master_pullup          (rw) 5V strong pullup 0 enabled, 1 disabled
w1_master_remove          (rw) manually remove a slave device
w1_master_search          (rw) the number of searches left to do,
                          -1=continual (default)
w1_master_slave_count     (ro) the number of slaves found
w1_master_slaves          (ro) the names of the slaves, one per line
w1_master_timeout         (ro) the delay in seconds between searches
w1_master_timeout_us      (ro) the delay in microseconds between searches
========================= =====================================================

濡傛灉浣犳湁涓€涓粠涓嶅彉鍖栫殑 w1 鎬荤嚎锛堜綘涓嶆坊鍔犳垨绉婚櫎璁惧锛夛紝鍙互鎶婃ā鍧楀弬鏁?search_count 璁句负涓€涓緝灏忕殑姝ｆ暣鏁帮紝浠庤€屽湪鍒濆闃舵鍙繘琛屽皯閲忕殑鎬荤嚎鎼滅储銆傛垨鑰呬篃鍙互鎶婂畠璁句负 0锛岀劧鍚庨€氳繃 w1_master_add 璁惧鏂囦欢鎵嬪姩娣诲姞浠庤澶囧簭鍒楀彿銆倃1_master_add 鍜?w1_master_remove 鏂囦欢涓€鑸彧鍦ㄦ悳绱㈣绂佺敤鏃舵墠鏈夋剰涔夛紝鍥犱负涓€娆℃悳绱細閲嶆柊妫€娴嬪埌鎵嬪姩绉婚櫎浣嗕粛鍦ㄦ€荤嚎涓婂瓨鍦ㄧ殑璁惧锛屽苟璁╄秴鏃惰鍔犱笂鍒版墜鍔ㄦ坊鍔犱絾瀹為檯涓嶅湪鎬荤嚎涓婄殑璁惧銆?

鎬荤嚎鎼滅储浠ヤ竴瀹氶棿闅斿彂鐢燂紝璇ラ棿闅旀寚瀹氫负 timeout 鍜?timeout_us 妯″潡鍙傛暟涔嬪拰锛堜袱鑰呬换涓€鍙互涓?0锛夛紝鍙 w1_master_search 浠嶅ぇ浜?0 鎴栦负 -1銆傛瘡娆℃悳绱㈠皾璇曚細鎶?w1_master_search 鍑?1锛堝噺鍒?0锛夛紝骞舵妸 w1_master_attempts 鍔?1銆?

### w1 浠庤澶?sysfs 鎺ュ彛

=================== ============================================================
bus                 (standard) symlink to the w1 bus
driver              (standard) symlink to the w1 driver
name                the device name, usually the same as the directory name
w1_slave            (optional) a binary file whose meaning depends on the
                    family driver
rw		    (optional) created for slave devices which do not have
		    appropriate family driver. Allows to read/write binary data.
=================== ============================================================
