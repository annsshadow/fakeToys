## 纭欢闅忔満鏁扮敓鎴愬櫒


## 绠€浠?

hw_random 妗嗘灦鏄竴涓埄鐢ㄤ綘鐨?CPU 鎴栦富鏉夸笂鐗规畩纭欢鐗规€р€斺€旈殢鏈烘暟鐢熸垚鍣紙RNG锛夆€斺€?鐨勮蒋浠躲€傝杞欢鍖呭惈涓ら儴鍒嗭細鎻愪緵涓€涓?/dev/hwrng 瀛楃璁惧鍙婂叾 sysfs 鏀寔鐨?鏍稿績閮ㄥ垎锛屼互鍙婁竴涓彃鍏ヨ鏍稿績鐨勭‖浠朵笓鐢ㄩ┍鍔ㄣ€?
涓轰簡鏈€鏈夋晥鍦板埄鐢ㄨ繖浜涙満鍒讹紝浣犺繕搴斿綋涓嬭浇鐩稿簲鐨勬敮鎸佽蒋浠躲€傝浠庝互涓嬪湴鍧€涓嬭浇
鏈€鏂扮殑 "rng-tools" 杞欢鍖咃細

	https://github.com/nhorman/rng-tools

杩欎簺宸ュ叿浣跨敤 /dev/hwrng 鏉ュ～鍏呭唴鏍哥喌姹狅紝璇ョ喌姹犲湪鍐呴儴浣跨敤锛屽苟閫氳繃 /dev/urandom 涓?/dev/random 鐗规畩鏂囦欢瀵煎嚭銆?
## 宸ヤ綔鍘熺悊


瀛楃璁惧銆備娇鐢ㄦ爣鍑嗙殑 open() 涓?read() 绯荤粺璋冪敤锛屼綘鍙互浠庣‖浠?RNG 璁惧璇诲彇
闅忔満鏁版嵁銆傝繖浜涙暟鎹?*鏈粡**浠讳綍閫傜敤鎬ф娴嬫鏌ワ紝骞朵笖鍙兘涓嶅彲闈狅紙濡傛灉纭欢瀛樺湪鏁呴殰鎴?閬埌绡℃敼锛夈€備粎褰撶‖浠剁殑 "has-data" 鏍囧織缃綅鏃舵墠浼氳緭鍑烘暟鎹紝灏界濡傛锛屾敞閲嶅畨鍏ㄦ€х殑浜?浼氬湪鍋囧畾鏁版嵁鐪熸闅忔満涔嬪墠锛屽厛瀵硅繖浜涙暟鎹繍琛岄€傜敤鎬ф娴嬨€?
rng-tools 杞欢鍖呭湪 "rngd" 涓娇鐢ㄦ绫绘娴嬶紝骞跺厑璁镐綘閫氳繃 "rngtest" 宸ュ叿鎵嬪姩杩愯瀹冧滑銆?
/dev/hwrng 鏄富璁惧鍙?10銆佹璁惧鍙?183 鐨勫瓧绗﹁澶囥€?
绫昏澶囥€傚瓨鍦ㄤ竴涓?/sys/class/misc/hw_random 鑺傜偣锛屽叿鏈変袱涓嫭鐗瑰睘鎬э細"rng_available"
涓?"rng_current"銆?rng_available" 灞炴€у垪鍑哄彲鐢ㄧ殑纭欢涓撶敤椹卞姩锛岃€?"rng_current" 鍒楀嚭
褰撳墠杩炴帴鍒?/dev/hwrng 鐨勯偅涓€傚鏋滀綘鐨勭郴缁熸湁澶氫釜鍙敤鐨?RNG锛屽彲浠ラ€氳繃灏?"rng_available"
鍒楄〃涓殑鏌愪釜鍚嶇О鍐欏叆 "rng_current" 鏉ユ洿鏀规墍浣跨敤鐨?RNG銆?
==========================================================================

Intel/AMD/VIA 闅忔満鏁扮敓鎴愬櫒锛圧NG锛夌‖浠堕┍鍔? - 鐗堟潈鎵€鏈?2000,2001 Jeff Garzik <jgarzik@pobox.com>
 - 鐗堟潈鎵€鏈?2000,2001 Philipp Rumpf <prumpf@mandrakesoft.com>

## 鍏充簬 Intel RNG 纭欢锛堟憳鑷浐浠?hub 鏁版嵁鎵嬪唽锛?

鍥轰欢 Hub 闆嗘垚浜嗕竴涓殢鏈烘暟鐢熸垚鍣紙RNG锛夛紝鍒╃敤纭呮潗鏂欏浐鏈夌殑銆佹湰璐ㄤ笂闅忔満鐨勯噺瀛愬姏瀛?鐗规€ф墍浜х敓鐨勭儹鍣０銆傚綋涓嶇敓鎴愭柊鐨勯殢鏈烘瘮鐗规椂锛孯NG 鐢佃矾浼氳繘鍏ヤ綆鍔熻€楃姸鎬併€侷ntel 灏嗘彁渚?涓€涓簩杩涘埗杞欢椹卞姩锛屼娇绗笁鏂硅蒋浠惰兘澶熻闂垜浠殑 RNG锛屼綔涓哄畨鍏ㄧ壒鎬т娇鐢ㄣ€傜洰鍓嶏紝RNG
浠呭彲鍦ㄧ郴缁熷浜?OS-present 鐘舵€佹椂浣跨敤銆?
## Intel RNG 椹卞姩璇存槑


FIXME锛氭敮鎸?poll(2)

	request_mem_region 宸茶绉婚櫎锛屽師鍥犳湁涓夛細

 1) 璇ラ┍鍔ㄤ粎鏀寔涓€涓?RNG锛? 2) RNG 浣跨敤鐨勪綅缃槸 MMIO 鍙鍧€鍐呭瓨涓殑涓€涓浐瀹氫綅缃紱
 3) 瀵逛簬姝ｇ‘宸ヤ綔鐨?BIOS e820 澶勭悊鐨勭敤鎴凤紝RNG 鎵€鍦ㄥ尯鍩熸€绘槸琚繚鐣欙紝鍥犳
	   request_mem_region 璋冪敤瀵逛簬姝ｇ‘閰嶇疆鎬绘槸澶辫触銆傜劧鑰岋紝瀵逛簬浣跨敤 mem=XX 鐨?	   鐢ㄦ埛锛孊IOS e820 淇℃伅**涓嶅湪** /proc/iomem 涓紝姝ゆ椂 request_mem_region(RNG_ADDR)
	   鍙互鎴愬姛銆?
## 椹卞姩缁嗚妭


鍩轰簬锛?	Intel 82802AB/82802AC Firmware Hub (FWH) Datasheet
	1999 骞?5 鏈?璁㈠崟鍙凤細290658-002 R

Intel 82802 Firmware Hub锛?	Random Number Generator
	Programmer's Reference Manual
	1999 骞?12 鏈?璁㈠崟鍙凤細298029-001 R

Intel 82802 Firmware HUB Random Number Generator Driver
	鐗堟潈鎵€鏈?(c) 2000 Matt Sottek <msottek@quiknet.com>

鐗瑰埆鎰熻阿 Matt Sottek銆傛垜鍋氫簡 "guts"锛堝簳灞傚疄鐜帮級锛屼粬鍋氫簡 "brains"锛堟牳蹇冭璁★級浠ュ強
鍏ㄩ儴娴嬭瘯銆?