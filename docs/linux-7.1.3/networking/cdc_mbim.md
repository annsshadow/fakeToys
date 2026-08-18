
## cdc_mbim - 鐢ㄤ簬 CDC MBIM 绉诲姩瀹藉甫璋冨埗瑙ｈ皟鍣ㄧ殑椹卞姩

cdc_mbim 椹卞姩鏀寔绗﹀悎 鈥淯niversal Serial Bus Communications Class Subclass Specification for Mobile Broadband Interface Model鈥濓紙绉诲姩瀹藉甫鎺ュ彛妯″瀷鐨?USB 閫氫俊绫诲瓙绫昏鑼冿級[^1^] 鐨?USB 璁惧锛岃瑙勮寖鏄?鈥淯niversal Serial Bus Communications Class Subclass Specifications for Network Control Model Devices鈥濓紙缃戠粶鎺у埗妯″瀷璁惧鐨?USB 閫氫俊绫诲瓙绫昏鑼冿級[^2^] 閽堝绉诲姩瀹藉甫璁惧锛堝嵆 鈥?G/LTE 璋冨埗瑙ｈ皟鍣ㄢ€濓級鐨勮繘涓€姝ヤ紭鍖栫増鏈€?

## 鍛戒护琛屽弬鏁?
cdc_mbim 椹卞姩鏈韩娌℃湁鑷繁鐨勫弬鏁般€備絾鏄 NCM 1.0 鍚戝悗鍏煎鐨?MBIM 鍔熻兘锛堝嵆 [^1^] 绗?3.2 鑺備腑瀹氫箟鐨?鈥淣CM/MBIM 鍔熻兘鈥濓級鐨勬帰娴嬭涓哄彈 cdc_ncm 椹卞姩鍙傛暟鐨勫奖鍝嶏細

### prefer_mbim

:Type:          Boolean
:Valid Range:   N/Y (0-1)
:Default Value: Y锛堝亸濂?MBIM锛?
璇ュ弬鏁拌缃簡閽堝 NCM/MBIM 鍔熻兘鐨勭郴缁熺瓥鐣ャ€傛绫诲姛鑳藉皢鐢?cdc_ncm 椹卞姩鎴?cdc_mbim 椹卞姩澶勭悊锛屽彇鍐充簬 prefer_mbim 璁剧疆銆傚皢 prefer_mbim 璁句负 N 浼氳 cdc_mbim 椹卞姩蹇界暐杩欎簺鍔熻兘锛岃€屾敼鐢?cdc_ncm 椹卞姩澶勭悊瀹冧滑銆?
璇ュ弬鏁版槸鍙啓鐨勶紝鍙互闅忔椂鏇存敼銆傞渶瑕佹墜鍔ㄦ墽琛?unbind/bind 鎵嶈兘浣挎洿鏀瑰缁戝畾鍒扳€滈敊璇€濋┍鍔ㄧ殑 NCM/MBIM 鍔熻兘鐢熸晥銆?

## 鍩烘湰鐢ㄦ硶

MBIM 鍔熻兘鍦ㄦ湭鍙楃鐞嗘椂澶勪簬闈炴椿鍔ㄧ姸鎬併€俢dc_mbim 椹卞姩浠呮彁渚涘埌 MBIM 鎺у埗閫氶亾鐨勭敤鎴风┖闂存帴鍙ｏ紝骞朵笖涓嶄細鍙備笌璇ュ姛鑳界殑鏃ュ父绠＄悊銆傝繖鎰忓懗鐫€濮嬬粓闇€瑕佷竴涓敤鎴风┖闂?MBIM 绠＄悊搴旂敤鏉ュ惎鐢ㄤ竴涓?MBIM 鍔熻兘銆?
姝ょ被鐢ㄦ埛绌洪棿搴旂敤鍖呮嫭浣嗕笉闄愪簬锛?
 - mbimcli锛堝寘鍚湪 libmbim [^3^] 搴撲腑锛夛紝浠ュ強
 - ModemManager [^4^]

寤虹珛涓€涓?MBIM IP 浼氳瘽鑷冲皯闇€瑕佺鐞嗗簲鐢ㄦ墽琛屼互涓嬫搷浣滐細

 - 鎵撳紑鎺у埗閫氶亾
 - 閰嶇疆缃戠粶杩炴帴璁剧疆
 - 杩炴帴鍒扮綉缁? - 閰嶇疆 IP 鎺ュ彛

### 绠＄悊搴旂敤寮€鍙?
椹卞姩 <-> 鐢ㄦ埛绌洪棿鐨勬帴鍙ｅ涓嬫枃鎵€杩般€侻BIM 鎺у埗閫氶亾鍗忚鍦?[^1^] 涓弿杩般€?

## MBIM 鎺у埗閫氶亾鐢ㄦ埛绌洪棿 ABI


### /dev/cdc-wdmX 瀛楃璁惧

椹卞姩浣跨敤 cdc-wdm 椹卞姩浣滀负瀛愰┍鍔紝鍒涘缓涓€涓埌 MBIM 鍔熻兘鎺у埗閫氶亾鐨勫弻鍚戠閬撱€傛帶鍒堕€氶亾绠￠亾鐨勭敤鎴风┖闂寸鏄竴涓?/dev/cdc-wdmX 瀛楃璁惧銆?
cdc_mbim 椹卞姩涓嶅鐞嗘垨瀹℃煡鎺у埗閫氶亾涓婄殑娑堟伅銆傝閫氶亾瀹屽叏濮旀墭缁欑敤鎴风┖闂寸鐞嗗簲鐢ㄣ€傚洜姝わ紝纭繚绗﹀悎 [^1^] 涓墍鏈夋帶鍒堕€氶亾瑕佹眰鐨勮矗浠诲湪浜庤搴旂敤銆?
cdc-wdmX 璁惧鏄綔涓?MBIM 鎺у埗鎺ュ彛 USB 璁惧鐨勫瓙璁惧鍒涘缓鐨勩€備笌鐗瑰畾璁惧鍏宠仈鐨勫瓧绗﹁澶?
```
 bjorn@nemi:~$ ls /sys/bus/usb/drivers/cdc_mbim/2-4:2.12/usbmisc
 cdc-wdm0

 bjorn@nemi:~$ grep . /sys/bus/usb/drivers/cdc_mbim/2-4:2.12/usbmisc/cdc-wdm0/dev
 180:0

```
### USB 閰嶇疆鎻忚堪绗?
CDC MBIM 鍔熻兘鎻忚堪绗︾殑 wMaxControlMessage 瀛楁闄愬埗浜嗘渶澶х殑鎺у埗娑堟伅澶у皬銆傜鐞嗗簲鐢ㄨ礋璐ｅ崗鍟嗕竴涓鍚?[^1^] 绗?9.3.1 鑺傝姹傜殑鎺у埗娑堟伅澶у皬锛屽悓鏃惰€冭檻姝ゆ弿杩扮瀛楁銆?
鐢ㄦ埛绌洪棿搴旂敤鍙互浣跨敤 [^6^] 鎴?[^7^] 涓弿杩扮殑涓ょ USB 閰嶇疆鎻忚堪绗﹀唴鏍告帴鍙ｄ箣涓€鏉ヨ闂?MBIM 鍔熻兘鐨?CDC MBIM 鍔熻兘鎻忚堪绗︺€?
鍙﹁涓嬫枃鍏充簬 ioctl 鐨勬枃妗ｃ€?

### 鍒嗙墖锛團ragmentation锛?
鐢ㄦ埛绌洪棿搴旂敤璐熻矗鎵€鏈夌殑鎺у埗娑堟伅鍒嗙墖鍜屽幓鍒嗙墖锛屽 [^1^] 绗?9.5 鑺傛墍杩般€?

### /dev/cdc-wdmX write()

鏉ヨ嚜绠＄悊搴旂敤鐨?MBIM 鎺у埗娑堟伅**涓嶅緱**瓒呰繃鍗忓晢鐨勬帶鍒舵秷鎭ぇ灏忋€?

### /dev/cdc-wdmX read()

绠＄悊搴旂敤**蹇呴』**鎺ュ彈鏈€澶т负鍗忓晢鎺у埗娑堟伅澶у皬鐨勬帶鍒舵秷鎭€?

### /dev/cdc-wdmX ioctl()

IOCTL_WDM_MAX_COMMAND锛氳幏鍙栨渶澶у懡浠ゅぇ灏?姝?ioctl 杩斿洖 MBIM 璁惧鐨?CDC MBIM 鍔熻兘鎻忚堪绗︿腑鐨?wMaxControlMessage 瀛楁銆傝繖浣滀负涓€绉嶄究鍒╄€屾彁渚涳紝娑堥櫎浜嗕粠鐢ㄦ埛绌洪棿瑙ｆ瀽 USB 鎻忚堪绗︾殑闇€瑕併€?
```
	#include <stdio.h>
	#include <fcntl.h>
	#include <sys/ioctl.h>
	#include <linux/types.h>
	#include <linux/usb/cdc-wdm.h>
	int main()
	{
		__u16 max;
		int fd = open("/dev/cdc-wdm0", O_RDWR);
		if (!ioctl(fd, IOCTL_WDM_MAX_COMMAND, &max))
			printf("wMaxControlMessage is %d\n", max);
	}

```
### 鑷畾涔夎澶囨湇鍔?
MBIM 瑙勮寖鍏佽渚涘簲鍟嗚嚜鐢卞畾涔夐澶栫殑鏈嶅姟銆俢dc_mbim 椹卞姩瀹屽叏鏀寔杩欎竴鐐广€?
瀵规柊鐨?MBIM 鏈嶅姟锛堝寘鎷緵搴斿晢鎸囧畾鐨勬湇鍔★級鐨勬敮鎸侊紝涓?MBIM 鎺у埗鍗忚鐨勫叾浣欓儴鍒嗕竴鏍凤紝瀹屽叏鍦ㄧ敤鎴风┖闂村疄鐜般€?
鏂扮殑鏈嶅姟搴斿湪 MBIM Registry [^5^] 涓敞鍐屻€?

## MBIM 鏁版嵁閫氶亾鐢ㄦ埛绌洪棿 ABI


### wwanY 缃戠粶璁惧

cdc_mbim 椹卞姩灏?MBIM 鏁版嵁閫氶亾琛ㄧず涓轰竴涓?鈥渨wan鈥?绫诲瀷鐨勫崟涓€缃戠粶璁惧銆傝缃戠粶璁惧鏈€鍒濇槧灏勫埌 MBIM IP 浼氳瘽 0銆?

### 澶氳矾澶嶇敤鐨?IP 浼氳瘽锛圛PS锛?
MBIM 鍏佽鍦ㄥ崟涓?USB 鏁版嵁閫氶亾涓婂璺鐢ㄥ杈?256 涓?IP 浼氳瘽銆俢dc_mbim 椹卞姩灏嗚繖浜?IP 浼氳瘽寤烘ā涓轰富 wwanY 璁惧鐨?802.1q VLAN 瀛愯澶囷紝灏嗘墍鏈夊ぇ浜?0 鐨?Z 鍊兼槧灏勫埌 MBIM IP 浼氳瘽 Z 鍒?VLAN ID Z銆?
璁惧鏈€澶?Z 鍊煎湪 [^1^] 绗?10.5.1 鑺傛弿杩扮殑 MBIM_DEVICE_CAPS_INFO 缁撴瀯涓粰鍑恒€?
鐢ㄦ埛绌洪棿绠＄悊搴旂敤璐熻矗鍦ㄥ缓绔?SessionId 澶т簬 0 鐨?MBIM IP 浼氳瘽涔嬪墠娣诲姞鏂扮殑 VLAN 閾捐矾銆傝繖浜涢摼璺彲浠ヤ娇鐢ㄦ櫘閫氱殑 VLAN 鍐呮牳鎺ュ彛锛坕octl 鎴?netlink锛夋潵娣诲姞銆?
```
  ip link add link wwan0 name wwan0.3 type vlan id 3

```
椹卞姩灏嗚嚜鍔ㄦ妸 鈥渨wan0.3鈥?缃戠粶璁惧鏄犲皠鍒?MBIM IP 浼氳瘽 3銆?

### 璁惧鏈嶅姟娴侊紙DSS锛?
MBIM 杩樺厑璁稿湪鍚屼竴涓叡浜?USB 鏁版嵁閫氶亾涓婂璺鐢ㄥ杈?256 涓潪 IP 鏁版嵁娴併€俢dc_mbim 椹卞姩灏嗚繖浜涗細璇濆缓妯′负涓?wwanY 璁惧鐨勫彟涓€缁?802.1q VLAN 瀛愯澶囷紝灏嗘墍鏈?A 鍊兼槧灏勫埌 MBIM DSS 浼氳瘽 A 鍒?VLAN ID锛?56 + A锛夈€?
璁惧鏈€澶?A 鍊煎湪 [^1^] 绗?10.5.29 鑺傛弿杩扮殑 MBIM_DEVICE_SERVICES_INFO 缁撴瀯涓粰鍑恒€?
DSS VLAN 瀛愯澶囩敤浣滃叡浜?MBIM 鏁版嵁閫氶亾涓庢劅鐭?MBIM DSS 鐨勭敤鎴风┖闂村簲鐢ㄤ箣闂寸殑涓€涓疄鐢ㄦ帴鍙ｃ€傚畠涓嶆墦绠楀師鏍峰憟鐜扮粰鏈€缁堢敤鎴枫€傚亣璁惧彂璧?DSS 浼氳瘽鐨勭敤鎴风┖闂村簲鐢ㄤ篃浼氳礋璐?DSS 鏁版嵁鐨勫繀瑕佹垚甯э紝骞朵互閫傚悎璇ユ祦绫诲瀷鐨勬柟寮忓皢娴佸憟鐜扮粰鏈€缁堢敤鎴枫€?
缃戠粶璁惧 ABI 瑕佹眰涓烘瘡涓浼犺緭鐨?DSS 鏁版嵁甯ч檮鍔犱竴涓吉浠ュお缃戝ご銆傝澶寸殑鍐呭鏄换鎰忕殑锛屼絾鏈変互涓嬩緥澶栵細

 - 浣跨敤 IP 鍗忚锛?x0800 鎴?0x86dd锛夌殑 TX 甯у皢琚涪寮? - RX 甯х殑鍗忚瀛楁灏嗚璁句负 ETH_P_802_3锛堜絾涓嶄細琚纭牸寮忓寲涓?802.3 甯э級
 - RX 甯х殑鐩殑鍦板潃灏嗚璁句负涓昏澶囩殑纭欢鍦板潃

鏀寔 DSS 鐨勭敤鎴风┖闂寸鐞嗗簲鐢ㄨ礋璐ｅ湪 TX 鏃舵坊鍔犱吉浠ュお缃戝ご骞跺湪 RX 鏃跺墺绂诲畠銆?
杩欐槸涓€涓娇鐢ㄥ父鐢ㄥ伐鍏风殑绠€鍗曠ず渚嬶紝灏?DssSessionId 5 瀵煎嚭涓烘寚鍚?/dev/nmea 鐨?pty 瀛楃璁惧

```
  ip link add link wwan0 name wwan0.dss5 type vlan id 261
  ip link set dev wwan0.dss5 up
  socat INTERFACE:wwan0.dss5,type=2 PTY:,echo=0,link=/dev/nmea

```
杩欏彧鏄竴涓ず渚嬶紝鏈€閫傚悎鐢ㄦ潵娴嬭瘯 DSS 鏈嶅姟銆傛敮鎸佺壒瀹?MBIM DSS 鏈嶅姟鐨勭敤鎴风┖闂村簲鐢ㄥ簲褰撲娇鐢ㄨ鏈嶅姟鎵€闇€鐨勫伐鍏峰拰缂栫▼鎺ュ彛銆?
娉ㄦ剰锛屼负 DSS 浼氳瘽娣诲姞 VLAN 閾捐矾瀹屽叏鏄彲閫夌殑銆傜鐞嗗簲鐢ㄤ篃鍙互閫夋嫨灏嗗寘濂楁帴瀛楃洿鎺ョ粦瀹氬埌涓荤綉缁滆澶囷紝浣跨敤鎺ユ敹鍒扮殑 VLAN 鏍囩灏嗗抚鏄犲皠鍒版纭殑 DSS 浼氳瘽锛屽苟鍦?TX 鏃舵坊鍔犲甫鏈夐€傚綋鏍囩鐨?18 瀛楄妭 VLAN 浠ュお缃戝ご銆傚湪杩欑鎯呭喌涓嬶紝寤鸿浣跨敤濂楁帴瀛楄繃婊ゅ櫒锛屽彧鍖归厤 DSS VLAN 瀛愰泦銆傝繖閬垮厤灏嗘棤鍏崇殑 IP 浼氳瘽鏁版嵁涓嶅繀瑕佸湴澶嶅埗鍒扮敤鎴风┖闂淬€傚浜?
```
  static struct sock_filter dssfilter[] = {
	/* 浣跨敤鐗规畩鐨勮礋鍋忕Щ鏉ヨ幏鍙?VLAN 鏍囩 */
	BPF_STMT(BPF_LD|BPF_B|BPF_ABS, SKF_AD_OFF + SKF_AD_VLAN_TAG_PRESENT),
	BPF_JUMP(BPF_JMP|BPF_JEQ|BPF_K, 1, 0, 6), /* true */

	/* 楠岃瘉 DSS VLAN 鑼冨洿 */
	BPF_STMT(BPF_LD|BPF_H|BPF_ABS, SKF_AD_OFF + SKF_AD_VLAN_TAG),
	BPF_JUMP(BPF_JMP|BPF_JGE|BPF_K, 256, 0, 4),	/* 256 鏄涓€涓?DSS VLAN */
	BPF_JUMP(BPF_JMP|BPF_JGE|BPF_K, 512, 3, 0),	/* 511 鏄渶鍚庝竴涓?DSS VLAN */

	/* 楠岃瘉浠ュお绫诲瀷 */
	BPF_STMT(BPF_LD|BPF_H|BPF_ABS, 2 * ETH_ALEN),
	BPF_JUMP(BPF_JMP|BPF_JEQ|BPF_K, ETH_P_802_3, 0, 1),

	BPF_STMT(BPF_RET|BPF_K, (u_int)-1),	/* 鎺ュ彈 */
	BPF_STMT(BPF_RET|BPF_K, 0),		/* 蹇界暐 */
  };



```
### 甯︽爣绛剧殑 IP 浼氳瘽 0 VLAN

濡備笂鎵€杩帮紝MBIM IP 浼氳瘽 0 琚┍鍔ㄥ綋浣滅壒娈婄殑鏉ュ鐞嗐€傚畠鏈€鍒濇槧灏勫埌 wwanY 缃戠粶璁惧涓婃湭鎵撴爣绛剧殑甯с€?
杩欑鏄犲皠瀵瑰璺鐢ㄧ殑 IPS 鍜?DSS 浼氳瘽鏂藉姞浜嗕竴浜涢檺鍒讹紝杩欎簺闄愬埗鍙兘骞朵笉鎬绘槸瀹炵敤锛?
 - 浠讳綍 IPS 鎴?DSS 浼氳瘽閮戒笉鑳戒娇鐢ㄥぇ浜?IP 浼氳瘽 0 涓?MTU 鐨勫抚澶у皬
 - 闄ら潪琛ㄧず IP 浼氳瘽 0 鐨勭綉缁滆澶囦篃澶勪簬 up 鐘舵€侊紝鍚﹀垯浠讳綍 IPS 鎴?DSS 浼氳瘽閮戒笉鑳藉浜?up 鐘舵€?
杩欎簺闂鍙互閫氳繃鍙€夊湴璁╅┍鍔ㄥ皢 IP 浼氳瘽 0 鏄犲皠鍒颁竴涓?VLAN 瀛愯澶囨潵閬垮厤锛岀被浼间簬鎵€鏈夊叾浠?IP 浼氳瘽銆傝繖绉嶈涓洪€氳繃涓洪瓟鏈?VLAN ID 4094 娣诲姞 VLAN 閾捐矾鏉ヨЕ鍙戙€傜劧鍚庨┍鍔ㄥ皢绔嬪嵆寮€濮嬪皢 MBIM IP 浼氳瘽 0 鏄犲皠鍒拌 VLAN锛屽苟灏嗕涪寮冧富 wwanY 璁惧涓婄殑鏈墦鏍囩甯с€?
鎻愮ず锛氬皢璇?VLAN 瀛愯澶囦互 MBIM SessionID 鑰屼笉鏄?VLAN ID 鍛藉悕锛屽鏈€缁堢敤鎴锋潵璇村彲鑳戒笉閭ｄ箞浠や汉鍥版儜銆傚浜?
```
  ip link add link wwan0 name wwan0.0 type vlan id 4094


```
### VLAN 鏄犲皠

鎬荤粨涓婅堪鎻忚堪鐨?cdc_mbim 椹卞姩鏄犲皠锛屾垜浠湁 wwanY 缃戠粶璁惧涓婄殑 VLAN 鏍囩涓?MBIM 涔嬮棿鐨勫叧绯?
```
  VLAN ID       MBIM type   MBIM SessionID           Notes
  ---------------------------------------------------------
  untagged      IPS         0                        a)
  1 - 255       IPS         1 - 255 <VLANID>
  256 - 511     DSS         0 - 255 <VLANID - 256>
  512 - 4093                                         b)
  4094          IPS         0                        c)

    a) 濡傛灉涓嶅瓨鍦?VLAN ID 4094 閾捐矾锛屽垯涓㈠純锛屽惁鍒欒涓㈠純
    b) 涓嶆敮鎸佺殑 VLAN 鑼冨洿锛屾棤鏉′欢涓㈠純
    c) 濡傛灉瀛樺湪 VLAN ID 4094 閾捐矾锛屽惁鍒欎涪寮?


```
## 鍙傝€冩枃鐚?
 1) USB Implementers Forum, Inc. - "Universal Serial Bus
    Communications Class Subclass Specification for Mobile Broadband
    Interface Model", Revision 1.0 (Errata 1), May 1, 2013

      - http://www.usb.org/developers/docs/devclass_docs/

 2) USB Implementers Forum, Inc. - "Universal Serial Bus
    Communications Class Subclass Specifications for Network Control
    Model Devices", Revision 1.0 (Errata 1), November 24, 2010

      - http://www.usb.org/developers/docs/devclass_docs/

 3) libmbim - "a glib-based library for talking to WWAN modems and
    devices which speak the Mobile Interface Broadband Model (MBIM)
    protocol"

      - http://www.freedesktop.org/wiki/Software/libmbim/

 4) ModemManager - "a DBus-activated daemon which controls mobile
    broadband (2G/3G/4G) devices and connections"

      - http://www.freedesktop.org/wiki/Software/ModemManager/

 5) "MBIM (Mobile Broadband Interface Model) Registry"

       - http://compliance.usb.org/mbim/

 6) "/sys/kernel/debug/usb/devices output format"

       - Documentation/driver-api/usb/usb.rst

 7) "/sys/bus/usb/devices/.../descriptors"

       - Documentation/ABI/stable/sysfs-bus-usb
