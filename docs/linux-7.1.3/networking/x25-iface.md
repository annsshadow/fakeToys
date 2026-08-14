## X.25 璁惧椹卞姩鎺ュ彛


鐗堟湰 1.1

			   Jonathan Naylor 26.12.96

鏈枃鎻忚堪浜嗗湪 X.25 鍒嗙粍灞傦紙Packet Layer锛変笌 X.25 璁惧椹卞姩涔嬮棿浼犻€掔殑娑堟伅銆傚畠浠?琚璁℃垚渚夸簬浠庡垎缁勫眰鍐呴儴杞绘澗璁剧疆 LAPB 妯″紡銆?
X.25 璁惧椹卞姩灏嗘寜鐓?Linux 璁惧椹卞姩鏍囧噯姝ｅ父缂栧啓銆傚ぇ澶氭暟 X.25 璁惧椹卞姩涓庡凡鏈夌殑
浠ュお缃戣澶囬┍鍔ㄥぇ浣撶浉浼笺€備絾涓庨偅浜涢┍鍔ㄤ笉鍚岋紝X.25 璁惧椹卞姩甯︽湁涓庝箣鍏宠仈鐨勭姸鎬侊紝
涓旇繖浜涗俊鎭渶瑕佸湪鍒嗙粍灞備箣闂翠紶鍏ヤ紶鍑猴紝浠ヤ繚璇佹甯歌繍琛屻€?
鎵€鏈夋秷鎭兘瀛樻斁鍦?sk_buff 涓紝灏卞儚瑕侀€氳繃 LAPB 閾捐矾浼犺緭鐨勭湡瀹炴暟鎹竴鏍枫€俿kbuff
鐨勭涓€涓瓧鑺傛寚绀哄叾浣欓儴鍒嗙殑鍚箟锛堝鏋滆繕瀛樺湪鏇村淇℃伅锛夈€?

### 鍒嗙粍灞傚埌璁惧椹卞姩


First Byte = 0x00 (X25_IFACE_DATA)

琛ㄧず skbuff 鐨勫叾浣欓儴鍒嗗寘鍚閫氳繃 LAPB 閾捐矾浼犺緭鐨勬暟鎹€傚湪浼犻€掍换浣曟暟鎹箣鍓嶏紝
LAPB 閾捐矾搴斿綋宸茬粡寤虹珛銆?
First Byte = 0x01 (X25_IFACE_CONNECT)

寤虹珛 LAPB 閾捐矾銆傚鏋滈摼璺凡缁忓缓绔嬶紝鍒欒繛鎺ョ‘璁ゆ秷鎭簲灏藉揩杩斿洖銆?
First Byte = 0x02 (X25_IFACE_DISCONNECT)

缁堟 LAPB 閾捐矾銆傚鏋滃凡缁忔柇寮€锛屽垯鏂紑纭娑堟伅搴斿敖蹇繑鍥炪€?
First Byte = 0x03 (X25_IFACE_PARAMS)

LAPB 鍙傛暟銆傚緟瀹氫箟銆?

### 璁惧椹卞姩鍒板垎缁勫眰


First Byte = 0x00 (X25_IFACE_DATA)

琛ㄧず skbuff 鐨勫叾浣欓儴鍒嗗寘鍚凡閫氳繃 LAPB 閾捐矾鎺ユ敹鐨勬暟鎹€?
First Byte = 0x01 (X25_IFACE_CONNECT)

LAPB 閾捐矾宸插缓绔嬨€傚悓涓€鏉℃秷鎭棦鐢ㄤ簬 LAPB 閾捐矾鐨?connect_confirmation锛堣繛鎺ョ‘璁わ級锛?涔熺敤浜?connect_indication锛堣繛鎺ユ寚绀猴級銆?
First Byte = 0x02 (X25_IFACE_DISCONNECT)

LAPB 閾捐矾宸茬粓姝€傚悓涓€鏉℃秷鎭棦鐢ㄤ簬 LAPB 閾捐矾鐨?disconnect_confirmation锛堟柇寮€
纭锛夛紝涔熺敤浜?disconnect_indication锛堟柇寮€鎸囩ず锛夈€?
First Byte = 0x03 (X25_IFACE_PARAMS)

LAPB 鍙傛暟銆傚緟瀹氫箟銆?

### 瀵硅澶囬┍鍔ㄧ殑瑕佹眰


鍦ㄥ垎缁勫眰涓庤澶囬┍鍔ㄤ箣闂翠紶閫掓暟鎹寘鏃讹紝涓嶅簲閲嶆帓搴忔垨涓㈠純銆?
涓洪伩鍏嶄粠璁惧椹卞姩鍚戝垎缁勫眰浼犻€掓暟鎹寘鏃跺彂鐢熼噸鎺掑簭鎴栦涪寮冿紝璁惧椹卞姩涓嶅簲璋冪敤
"netif_rx" 鏉ラ€掍氦鎺ユ敹鍒扮殑鏁版嵁鍖咃紝鑰屽簲浠?softirq 涓婁笅鏂囪皟鐢?"netif_receive_skb_core" 鏉ラ€掍氦瀹冧滑銆?