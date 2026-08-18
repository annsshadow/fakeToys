
## 楂樺眰 CI API


   鏈枃妗ｅ凡杩囨椂銆?
鏈枃妗ｆ牴鎹?Linux DVB API 鎻忚堪楂樺眰 CI API銆?
閫氳繃楂樺眰 CI 鏂规硶锛屼换浣曞叿鏈夊嚑涔庝换鎰忛殢鏈烘灦鏋勭殑鏂板崱閮藉彲浠ョ敤杩欑椋庢牸瀹炵幇锛宻witch 璇彞涓殑瀹氫箟鍙互杞绘澗閫傞厤
浠讳綍鍗★紝浠庤€屾棤闇€浠讳綍棰濆鐨?ioctl銆?
缂虹偣鍦ㄤ簬椹卞姩/纭欢蹇呴』绠＄悊鍏朵綑閮ㄥ垎銆傚浜庡簲鐢ㄧ▼搴忓憳鏉ヨ锛岃繖灏卞儚鍚?Linux DVB API 涓畾涔夌殑 CI ioctl
鍙戦€?鎺ユ敹鏁扮粍涓€鏍风畝鍗曘€備负浜嗗绾虫鐗规€э紝API 娌℃湁鍋氫换浣曟敼鍔ㄣ€?

#### 涓轰綍闇€瑕佸彟涓€涓?CI 鎺ュ彛锛?

杩欐槸鏈€甯搁棶鐨勯棶棰樹箣涓€銆傚棷锛岃繖鏄釜濂介棶棰樸€備弗鏍兼潵璇达紝杩欎笉鏄竴涓柊鎺ュ彛銆?
CI 鎺ュ彛鍦?DVB API 鐨?ca.h 涓畾涔変负锛?
	typedef struct ca_slot_info {
		int num;               /** 妲戒綅鍙?**/

		int type;              /** 姝ゆЫ浣嶆敮鎸佺殑 CA 鎺ュ彛 **/
	#define CA_CI            1     /** CI 楂樺眰鎺ュ彛 **/
	#define CA_CI_LINK       2     /** CI 閾捐矾灞傛帴鍙?**/
	#define CA_CI_PHYS       4     /** CI 鐗╃悊灞傛帴鍙?**/
	#define CA_DESCR         8     /** 鍐呯疆瑙ｆ壈鍣?**/
	#define CA_SC          128     /** 绠€鍗曟櫤鑳藉崱鎺ュ彛 **/

		unsigned int flags;
	#define CA_CI_MODULE_PRESENT 1 /** 妯″潡锛堟垨鍗★級宸叉彃鍏?**/
	#define CA_CI_MODULE_READY   2
	} ca_slot_info_t;

姝?CI 鎺ュ彛閬靛惊 CI 楂樺眰鎺ュ彛锛岃€屽ぇ澶氭暟搴旂敤绋嬪簭骞舵湭瀹炵幇瀹冦€傚洜姝ら噸鏂板瑙嗕簡杩欎竴棰嗗煙銆?
姝?CI 鎺ュ彛鐩稿綋涓嶅悓锛屽洜涓哄畠璇曞浘瀹圭撼鎵€鏈夎惤鍏ュ叾浠栫被鍒殑銆佸熀浜?CI 鐨勫叾浠栬澶囥€?
杩欐剰鍛崇潃姝?CI 鎺ュ彛浠呭湪搴旂敤灞傚鐞?EN50221 椋庢牸鏍囩锛屼細璇濈鐞嗕笉鐢卞簲鐢ㄧ▼搴忚礋璐ｃ€傞┍鍔?纭欢灏嗚礋璐ｆ墍鏈夎繖浜涖€?
姝ゆ帴鍙ｇ函绮规槸涓€涓氦鎹?APDU 鐨?EN50221 鎺ュ彛銆傝繖鎰忓懗鐫€鍦ㄥ簲鐢ㄧ▼搴忓埌椹卞姩鐨勯€氫俊涓笉瀛樺湪浼氳瘽绠＄悊銆侀摼璺眰鎴?浼犺緭灞傘€傚氨杩欎箞绠€鍗曘€傞┍鍔?纭欢蹇呴』璐熻矗杩欎簺銆?
閫氳繃姝ら珮灞?CI 鎺ュ彛锛屽彲浠ヤ娇鐢ㄥ父瑙?ioctl 鏉ュ畾涔夋帴鍙ｃ€?
鎵€鏈夎繖浜?ioctl 瀵归珮灞?CI 鎺ュ彛鍚屾牱鏈夋晥

#define CA_RESET          _IO('o', 128)
#define CA_GET_CAP        _IOR('o', 129, ca_caps_t)
#define CA_GET_SLOT_INFO  _IOR('o', 130, ca_slot_info_t)
#define CA_GET_DESCR_INFO _IOR('o', 131, ca_descr_info_t)
#define CA_GET_MSG        _IOR('o', 132, ca_msg_t)
#define CA_SEND_MSG       _IOW('o', 133, ca_msg_t)
#define CA_SET_DESCR      _IOW('o', 134, ca_descr_t)


鏌ヨ璁惧鏃讹紝璁惧浜х敓濡備笅淇℃伅锛?

# 	CA_GET_SLOT_INFO

	Command = [info]
	APP: Number=[^1^]
	APP: Type=[^1^]
	APP: flags=[^1^]
	APP: CI 楂樺眰鎺ュ彛
	APP: CA/CI 妯″潡宸叉彃鍏?
# 	CA_GET_CAP

	Command = [caps]
	APP: Slots=[^1^]
	APP: Type=[^1^]
	APP: 瑙ｆ壈鍣ㄥ瘑閽?[^16^]
	APP: Type=[^1^]

# 	CA_SEND_MSG

	Descriptors(Program Level)=[ 09 06 06 04 05 50 ff f1]
	Found CA descriptor @ program level

	(20) ES type=[^2^] ES pid=[^201^]  ES length =[0 (0x0)]
	(25) ES type=[^4^] ES pid=[^301^]  ES length =[0 (0x0)]
	ca_message length is 25 (0x19) bytes
	EN50221 CA MSG=[ 9f 80 32 19 03 01 2d d1 f0 08 01 09 06 06 04 05 50 ff f1 02 e0 c9 00 00 04 e1 2d 00 00]


骞堕潪 API 涓殑鎵€鏈?ioctl 閮藉湪椹卞姩涓疄鐜帮紝閭ｄ簺鏃犳硶閫氳繃 API 瀹炵幇鐨勭‖浠跺叾浠栫壒鎬у垯浣跨敤 CA_GET_MSG 涓?CA_SEND_MSG ioctl 鏉ュ疄鐜般€備娇鐢ㄤ竴涓?EN50221 椋庢牸鐨勫寘瑁呭櫒鏉ヤ氦鎹㈡暟鎹紝浠ヤ笌鍏朵粬纭欢淇濇寔鍏煎銆?

	/** 鏉ヨ嚜/鍙戝線 CI-CAM 鐨勬秷鎭?**/
	typedef struct ca_msg {
		unsigned int index;
		unsigned int type;
		unsigned int length;
		unsigned char msg[^256^];
	} ca_msg_t;


鏁版嵁鐨勬祦鍚戝彲浠ユ弿杩板涓嬶細


	App (User)
	-----
	parse
	  |
	  |
	  v
# 	en50221 APDU锛堟墦鍖咃級

   |	  |				| High Level CI driver
   |	  |				|
   |	  v				|
   |	en50221 APDU锛堣В鍖咃級	|
   |	  |				|
   |	  |				|
   |	  v				|
   |	瀹屾暣鎬ф鏌?		|
   |	  |				|
   |	  |				|
   |	  v				|
#    |	do锛堜緷璧栫‖浠讹級		|

	  |    Hardware
	  |
	  v

楂樺眰 CI 鎺ュ彛浣跨敤 EN50221 DVB 鏍囧噯锛岄伒寰爣鍑嗙‘淇濅簡闈㈠悜鏈潵銆?