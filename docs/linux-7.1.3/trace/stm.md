

## 绯荤粺杩借釜妯″潡锛圫ystem Trace Module锛?


绯荤粺杩借釜妯″潡锛圫ystem Trace Module锛孲TM锛夋槸 MIPI STP 瑙勮寖涓弿杩扮殑涓€绉嶈澶囷紝浣滀负 STP
杩借釜娴佺敓鎴愬櫒銆係TP锛圫ystem Trace Protocol锛岀郴缁熻拷韪崗璁級鏄竴绉嶅皢鏉ヨ嚜澶氫釜杩借釜婧愮殑鏁版嵁
杩涜澶氳矾澶嶇敤鐨勮拷韪崗璁紝鍏朵腑姣忎釜杩借釜婧愰兘琚垎閰嶄竴瀵瑰敮涓€鐨?master 涓?channel銆傝櫧鐒惰繖浜?
master 涓?channel 涓殑涓€閮ㄥ垎琚潤鎬佸垎閰嶇粰鏌愪簺纭欢杩借釜婧愶紝浣嗗叾浣欓儴鍒嗗彲渚涜蒋浠朵娇鐢ㄣ€傝蒋浠?
杩借釜婧愰€氬父鍙互鑷浠庤姹犲瓙涓换鎰忛€夊彇 master/channel 缁勫悎銆?

鍦?STP 娴佺殑鎺ユ敹绔紙瑙ｇ爜渚э級锛岃拷韪簮鍙兘鐢?master/channel 缁勫悎鏉ヨ瘑鍒紝鍥犳涓轰簡璁╄В鐮佸櫒
鑳藉鐞嗚В娑夊強澶氫釜杩借釜婧愮殑杩借釜鍐呭锛屽畠闇€瑕佽兘澶熷皢杩欎簺 master/channel 瀵规槧灏勫埌瀹冩墍璁よ瘑鐨?
杩借釜婧愩€?

渚嬪锛岀煡閬?syslog 娑堟伅鏉ヨ嚜 master 7 channel 15 鏄緢鏈夊府鍔╃殑锛岃€屼换鎰忕敤鎴峰簲鐢ㄧ▼搴忓彲浠ヤ娇鐢?
master 48 鍒?63 浠ュ強 channel 0 鍒?127銆?

涓轰簡瑙ｅ喅杩欎釜鏄犲皠闂锛宻tm 绫婚€氳繃 configfs 鎻愪緵浜嗕竴绉嶇瓥鐣ョ鐞嗘満鍒讹紝鍏佽瀹氫箟灏嗗瓧绗︿覆
鏍囪瘑绗︽槧灏勫埌 master 涓?channel 鑼冨洿鐨勮鍒欍€傚鏋滆繖浜涜鍒欙紙绛栫暐锛変笌瑙ｇ爜鍣ㄧ殑鏈熸湜涓€鑷达紝瀹?
灏辫兘姝ｇ‘澶勭悊杩借釜鏁版嵁銆?

璇ョ瓥鐣ユ槸涓€涓爲褰㈢粨鏋勶紝鍖呭惈瑙勫垯锛坧olicy_node锛夛紝姣忔潯瑙勫垯閮芥湁涓€涓悕绉帮紙瀛楃涓叉爣璇嗙锛変互鍙?
涓庝箣鍏宠仈鐨勪竴缁?master 涓?channel 鑼冨洿锛屼綅浜?configfs 鐨?"stp-policy" 瀛愮郴缁熺洰褰曚腑銆傛渶椤跺眰
鐩綍鐨勫悕绉帮紙鍗崇瓥鐣ワ級鏍煎紡涓猴細鍏舵墍閫傜敤鐨?STM 璁惧鍚嶏紝鍚庤窡涓€涓敱鍙ョ偣鍒嗛殧鐨勪换鎰忓瓧绗︿覆鏍囪瘑绗︺€?
浠ヤ笂闈㈢殑渚嬪瓙鏉ヨ锛屼竴鏉¤鍒?

```

	$ ls /config/stp-policy/dummy_stm.my-policy/user
	channels masters
	$ cat /config/stp-policy/dummy_stm.my-policy/user/masters
	48 63
	$ cat /config/stp-policy/dummy_stm.my-policy/user/channels
	0 127

```
杩欐剰鍛崇潃璇ヨ鍒欑殑 master 鍒嗛厤姹犲寘鍚?master 48 鍒?63锛宑hannel 鍒嗛厤姹犲寘鍚?channel 0 鍒?127銆?
鐜板湪锛屼换浣曚互 "user" 鏍囪瘑瀛楃涓茶嚜鎴戞爣璇嗙殑鐢熶骇鑰咃紙杩借釜婧愶級閮藉皢琚粠杩欎簺鑼冨洿鍐呭垎閰嶄竴涓?master 涓?channel銆?

杩欎簺瑙勫垯鍙互宓屽锛屼緥濡傦紝鍙互鍦ㄤ笂闈緥瀛愪腑鐨?"user" 鐩綍涓嬪畾涔変竴鏉″悕涓?"dummy" 鐨勮鍒欙紝
杩欐潯鏂拌鍒欏皢鐢ㄤ簬鏍囪瘑瀛楃涓蹭负 "user/dummy" 鐨勮拷韪簮銆?

杩借釜婧愬繀椤绘墦寮€ stm 绫昏澶囩殑鑺傜偣锛屽苟灏嗗畠浠殑杩借釜鏁版嵁鍐欏叆鍏舵枃浠舵弿杩扮銆?

涓轰簡缁欑粰瀹氱殑杩借釜婧愭壘鍒板悎閫傜殑绛栫暐鑺傜偣锛屽彲浠ヤ娇鐢ㄨ嫢骞叉満鍒躲€傞鍏堬紝杩借釜婧愬彲浠ュ湪鍚戝瓧绗﹁澶囩殑
鏂囦欢鎻忚堪绗﹀啓鍏ヤ换浣曟暟鎹箣鍓嶏紝閫氳繃鍦ㄥ叾涓婅皟鐢?STP_POLICY_ID_SET ioctl 鏄惧紡鍦拌嚜鎴戞爣璇嗭紝
鎻愪緵瀹冧滑鐨?id 瀛楃涓层€傚叾娆★紝濡傛灉瀹冧滑閫夋嫨涓嶈繘琛屾樉寮忔爣璇嗭紙鍥犱负浣犲彲鑳戒笉鎯充负姝や慨琛ョ幇鏈夎蒋浠讹級锛?
瀹冧滑鍙互鐩存帴寮€濮嬪啓鍏ユ暟鎹紝姝ゆ椂 stm 鏍稿績浼氬皾璇曟煡鎵惧悕绉颁笌浠诲姟鍚嶏紙渚嬪 "syslogd"锛夊尮閰嶇殑绛栫暐
鑺傜偣锛屽鏋滃瓨鍦ㄥ垯浣跨敤瀹冦€傜涓夛紝濡傛灉鍦ㄧ瓥鐣ヨ妭鐐逛腑鎵句笉鍒颁换鍔″悕锛屽垯浼氫娇鐢ㄥ厹搴曟潯鐩?"default"锛?
濡傛灉瀹冨瓨鍦級銆傝鏉＄洰鍚屾牱闇€瑕佺敱绯荤粺绠＄悊鍛樻垨璐熻矗绛栫暐閰嶇疆鐨勭浉鍏冲伐鍏峰垱寤哄拰閰嶇疆銆傛渶鍚庯紝濡傛灉涓婅堪
鎵€鏈夋楠ら兘澶辫触锛屽 stm 鏂囦欢鎻忚堪绗︾殑 write() 灏嗚繑鍥炰竴涓敊璇紙EINVAL锛夈€?

姝ゅ墠锛屽鏋滀负鏌愪釜杩借釜婧愭壘涓嶅埌绛栫暐鑺傜偣锛宻tm 绫讳細榛橀粯鍦板洖閫€鍒颁粠璁惧 master/channel 鑼冨洿鐨?
寮€澶村垎閰嶇涓€鍧楀彲鐢ㄧ殑杩炵画 master/channel 鑼冨洿銆傜幇鍦ㄨ姹傚繀椤诲瓨鍦ㄧ瓥鐣ヨ妭鐐癸紝杩欏皢甯姪绋嬪簭鍛樺拰
绯荤粺绠＄悊鍛樺彂鐜伴厤缃腑鐨勭己鍙ｏ紝骞舵洿濂藉湴鎺у埗鏈爣璇嗙殑婧愩€?

鏌愪簺 STM 璁惧鍙兘鍏佽灏?channel 鐨?mmio 鍖哄煙鐩存帴鏄犲皠鍒扮敤鎴风┖闂翠互瀹炵幇闆舵嫹璐濆啓鍏ャ€備竴涓彲
鏄犲皠鐨勯〉锛堝氨 mmu 鑰岃█锛夐€氬父鍖呭惈澶氫釜 channel 鐨?mmio锛屽洜姝ょ敤鎴烽渶瑕佷负鑷繁鍒嗛厤閭ｄ箞澶?channel
锛堥€氳繃涓婅堪 ioctl() 璋冪敤锛夋墠鑳藉仛鍒拌繖涓€鐐广€備篃灏辨槸璇达紝濡傛灉浣犵殑 stm 璁惧鐨?channel mmio 鍖哄煙
涓?64 瀛楄妭锛岃€岀‖浠堕〉澶у皬涓?4096 瀛楄妭锛岄偅涔堝湪鎴愬姛璋冪敤 width==64 鐨?STP_POLICY_ID_SET ioctl()
涔嬪悗锛屼綘搴旇鑳藉鍦ㄦ鏂囦欢鎻忚堪绗︿笂 mmap() 涓€椤碉紝骞惰幏寰楀 64 涓?channel 鐨?mmio 鍖哄煙鐨勭洿鎺ヨ闂€?

STM 璁惧鐨勪緥瀛愭湁 Intel(R) Trace Hub [^1^] 涓?Coresight STM [^2^]銆?

## stm_source


瀵逛簬鍩轰簬鍐呮牳鐨勮拷韪簮锛屽瓨鍦?"stm_source" 璁惧绫汇€傝绫荤殑璁惧鍙互鍦ㄨ繍琛屾椂閫氳繃鍚嶄负
"stm_source_link" 鐨?sysfs 灞炴€ц繛鎺ュ埌 stm 璁惧鎴栦粠 stm 璁惧鏂紑

```

	$ echo dummy_stm.0 > /sys/class/stm_source/console/stm_source_link

```
鍏充簬濡備綍鍦ㄥ唴鏍镐腑浣跨敤 stm_source 鎺ュ彛鐨勭ず渚嬶紝璇峰弬鑰?stm_console銆乻tm_heartbeat 鎴?
stm_ftrace 椹卞姩銆?

姣忎釜 stm_source 璁惧閮介渶瑕佹牴鎹畠鎵€闇€鐨?channel 鏁伴噺锛屽崰鐢ㄤ竴涓?master 浠ュ強涓€娈?channel 鑼冨洿銆?
杩欎簺浼氭牴鎹瓥鐣ラ厤缃负璁惧鍒嗛厤銆傚鏋滅瓥鐣ョ洰褰曠殑鏍逛笅瀛樺湪涓€涓笌 stm_source 璁惧鍚嶇О锛堜緥濡?
"console"锛夊尮閰嶇殑鑺傜偣锛屽垯浣跨敤璇ヨ妭鐐规潵鍒嗛厤 master 涓?channel 鍙枫€傚鏋滀笉瀛樺湪杩欐牱鐨勭瓥鐣ヨ妭鐐癸紝
stm 鏍稿績灏嗕娇鐢ㄥ厹搴曟潯鐩?"default"锛堝鏋滃瓨鍦級銆傚鏋滀袱绉嶇瓥鐣ヨ妭鐐归兘涓嶅瓨鍦紝瀵?stm_source_link
鐨?write() 灏嗚繑鍥炰竴涓敊璇€?

## stm_console


涓婇潰渚嬪瓙涓娇鐢ㄧ殑璇ユ帴鍙ｇ殑鍙︿竴绉嶅疄鐜版槸 "stm_console" 椹卞姩锛屽畠鍩烘湰涓婇€氳繃 stm 璁惧涓哄唴鏍告秷鎭?
鎻愪緵涓€涓崟鍚戞帶鍒跺彴銆?

瑕侀厤缃皢鍦?STP 娴佷腑鍒嗛厤缁欒鎺у埗鍙扮殑 master/channel 瀵癸紝璇峰垱寤轰竴涓?"console" 绛栫暐鏉＄洰
锛堝浣曞垱寤鸿鍙傝鏈枃寮€澶达級銆傚垵濮嬪寲鏃讹紝瀹冨皢鍗犵敤涓€涓?channel銆?

## stm_ftrace


杩欐槸鍙︿竴涓?"stm_source" 璁惧锛屼竴鏃?stm_ftrace 涓庢煇涓?stm 璁惧寤虹珛閾炬帴锛屽苟涓斿惎鐢ㄤ簡 "function"
杩借釜鍣紝Ftrace 瀛愮郴缁熸湰搴斿瓨鍏ョ幆褰㈢紦鍐插尯鐨勫嚱鏁板湴鍧€涓庣埗鍑芥暟鍦板潃锛屽皢鍚屾椂閫氳繃 stm 璁惧瀵煎嚭銆?

鐩墠浠呮敮鎸?Ftrace 鐨?"function" 杩借釜鍣ㄣ€?

- [^1^] https://software.intel.com/sites/default/files/managed/d3/3c/intel-th-developer-manual.pdf
- [^2^] http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0444b/index.html
