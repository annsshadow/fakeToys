## DNS 瑙ｆ瀽鍣ㄦā鍧?


 - 姒傝堪銆?
 - 缂栬瘧銆?
 - 璁剧疆銆?
 - 鐢ㄦ硶銆?
 - 鏈哄埗銆?
 - 璋冭瘯銆?


## 姒傝堪


DNS 瑙ｆ瀽鍣ㄦā鍧椾负鍐呮牳鏈嶅姟鎻愪緵浜嗕竴绉嶉€氳繃璇锋眰 key 绫诲瀷涓?dns_resolver 鐨?
瀵嗛挜鏉ヨ繘琛?DNS 鏌ヨ鐨勬柟娉曘€傝繖浜涙煡璇㈤€氳繃 /sbin/request-key 鍚戜笂璋冪敤
锛坲pcall锛夊埌鐢ㄦ埛绌洪棿銆?

杩欎簺渚嬬▼蹇呴』鐢辩敤鎴风┖闂村伐鍏?dns.upcall銆乧ifs.upcall 鍜?request-key 鏀寔銆?
瀹冧粛鍦ㄥ紑鍙戜腑锛屽皻鏈彁渚涘畬鏁寸殑鍔熻兘闆嗐€傚畠鏀寔鐨勫姛鑳藉寘鎷細

 - 瀹炵幇浜?dns_resolver key_type 浠ヨ仈绯荤敤鎴风┖闂淬€?

瀹冨皻涓嶆敮鎸佷互涓?AFS 鍔熻兘锛?

 - 瀵?AFSDB 璧勬簮璁板綍鐨?DNS 鏌ヨ鏀寔銆?

姝や唬鐮佷粠 CIFS 鏂囦欢绯荤粺涓彁鍙栥€?


## 缂栬瘧


```

	CONFIG_DNS_RESOLVER	- tristate "DNS Resolver support"


```
## 璁剧疆


瑕佽缃鍔熻兘锛屽繀椤讳慨鏀?/etc/request-key.conf 鏂囦欢锛屼互渚?/sbin/request-key
鑳藉閫傚綋鍦板紩瀵煎悜涓婅皟鐢ㄣ€備緥濡傦紝涓轰簡澶勭悊鍩烘湰鐨?dname 鍒?IPv4/IPv6 鍦板潃鐨?
瑙ｆ瀽锛屽簲娣诲姞濡備笅涓€琛岋細
```

	#OP	TYPE		DESC	CO-INFO	PROGRAM ARG1 ARG2 ARG3 ...
	#======	============	=======	=======	==========================
	create	dns_resolver  	*	*	/usr/sbin/cifs.upcall %k

```
涓轰簡寮曞瀵规煡璇㈢被鍨?'foo' 鐨勬煡璇紝搴旀坊鍔犲涓嬩竴琛岋細
```

	create	dns_resolver  	foo:*	*	/usr/sbin/dns.foo %k


```
## 鐢ㄦ硶


```

	#include <linux/dns_resolver.h>

```
```

	int dns_query(const char *type, const char *name, size_t namelen,
		     const char *options, char **_result, time_t *_expiry);

```
杩欐槸鍩烘湰鐨勮闂嚱鏁般€傚畠鏌ユ壘缂撳瓨鐨?DNS 鏌ヨ锛屽鏋滄病鎵惧埌锛屽垯鍚戜笂璋冪敤鐢ㄦ埛绌洪棿
鍙戣捣涓€涓柊鐨?DNS 鏌ヨ锛岃鏌ヨ闅忓悗鍙兘琚紦瀛樸€傚瘑閽ユ弿杩拌鏋勯€犱负濡備笅瀛楃涓诧細
```

		[<type>:]<name>

```
鍏朵腑 <type> 鍙€夊湴鎸囧畾瑕佽皟鐢ㄧ殑鐗瑰畾鍚戜笂璋冪敤绋嬪簭锛屽苟鍥犳鎸囧畾鏌ヨ鐨勭被鍨嬶紝
<name> 鎸囧畾瑕佹煡鎵剧殑瀛楃涓层€傞粯璁ゆ煡璇㈢被鍨嬫槸鐩存帴鐨勪富鏈哄悕鍒?IP 鍦板潃闆嗗悎鏌ユ壘銆?

name 鍙傛暟涓嶈姹傛槸 NUL 缁撳熬鐨勫瓧绗︿覆锛屽叾闀垮害搴旂敱 namelen 鍙傛暟缁欏嚭銆?

options 鍙傛暟鍙互涓?NULL锛屼篃鍙互鏄竴缁勯€傚悎璇ユ煡璇㈢被鍨嬬殑閫夐」銆?

杩斿洖鍊兼槸涓€涓€傚悎璇ユ煡璇㈢被鍨嬬殑瀛楃涓层€備緥濡傦紝瀵逛簬榛樿鏌ヨ绫诲瀷锛屽畠鍙槸涓€涓?
閫楀彿鍒嗛殧鐨?IPv4 鍜?IPv6 鍦板潃鍒楄〃銆傝皟鐢ㄨ€呭繀椤婚噴鏀捐缁撴灉銆?

鎴愬姛鏃惰繑鍥炵粨鏋滃瓧绗︿覆鐨勯暱搴︼紝鍚﹀垯杩斿洖璐熺殑閿欒鐮併€傚鏋?DNS 鏌ユ壘澶辫触锛屽皢
杩斿洖 -EKEYREJECTED銆?

濡傛灉 _expiry 闈?NULL锛屽垯缁撴灉鐨勫埌鏈熸椂闂达紙TTL锛変篃浼氳杩斿洖銆?

鍐呯淮鎸佹湁涓€涓唴閮ㄥ瘑閽ョ幆锛坘eyring锛夛紝鍦ㄥ叾涓紦瀛樺凡鏌ユ壘鐨勫瘑閽ャ€備换浣曞叿鏈?
CAP_SYS_ADMIN 鑳藉姏鐨勮繘绋嬮兘鍙互閫氳繃瀵硅瀵嗛挜鐜?ID 浣跨敤 KEYCTL_KEYRING_CLEAR
鏉ユ竻闄ゅ畠銆?


## 浠庣敤鎴风┖闂磋鍙?DNS 瀵嗛挜


dns_resolver 绫诲瀷鐨勫瘑閽ュ彲浠ヤ娇鐢?keyctl_read() 鎴?"keyctl read/print/pipe"
浠庣敤鎴风┖闂磋鍙栥€?


## 鏈哄埗


dns_resolver 妯″潡娉ㄥ唽浜嗕竴涓悕涓?"dns_resolver" 鐨勫瘑閽ョ被鍨嬨€傛绫诲瀷鐨勫瘑閽?
鐢ㄤ簬鍦ㄧ敤鎴风┖闂翠箣闂翠紶杈撳拰缂撳瓨 DNS 鏌ユ壘缁撴灉銆?

褰撹皟鐢?dns_query() 鏃讹紝瀹冭皟鐢?request_key() 鍦ㄦ湰鍦板瘑閽ョ幆涓悳绱㈢紦瀛樼殑 DNS
缁撴灉銆傚鏋滄病鎵惧埌锛屽畠浼氬悜涓婅皟鐢ㄧ敤鎴风┖闂翠互鑾峰彇鏂扮粨鏋溿€?

鍚戠敤鎴风┖闂寸殑鍚戜笂璋冪敤鏄€氳繃 request_key() 鍚戜笂璋冪敤鍚戦噺杩涜鐨勶紝骞剁敱
/etc/request-key.conf 涓殑閰嶇疆琛屽紩瀵硷紝杩欎簺閰嶇疆琛屽憡璇?/sbin/request-key
杩愯浠€涔堢▼搴忔潵瀹炰緥鍖栵紙instantiate锛夎瀵嗛挜銆?

鍚戜笂璋冪敤澶勭悊绋嬪簭绋嬪簭璐熻矗鏌ヨ DNS锛屽皢缁撴灉澶勭悊涓洪€傚悎浼犻€掔粰
keyctl_instantiate_key() 渚嬬▼鐨勫舰寮忋€傜劧鍚庡畠灏嗘暟鎹紶閫掔粰
dns_resolver_instantiate()锛屽悗鑰呭墺绂诲苟澶勭悊鏁版嵁涓换浣曞寘鍚殑閫夐」锛岀劧鍚庡皢
瀛楃涓茬殑鍓╀綑閮ㄥ垎浣滀负杞借嵎锛坧ayload锛夐檮鍔犲埌瀵嗛挜涓娿€?

鍚戜笂璋冪敤澶勭悊绋嬪簭绋嬪簭搴斿皢瀵嗛挜鐨勫埌鏈熸椂闂磋涓哄畠浠庝腑鎻愬彇缁撴灉鐨勬墍鏈夎褰曚腑
鏈€浣庣殑 TTL銆傝繖鎰忓懗鐫€褰撹瀵嗛挜鎸佹湁鐨勬暟鎹埌鏈熸椂锛屽瘑閽ュ皢琚涪寮冨苟閲嶆柊鍒涘缓銆?

dns_query() 杩斿洖闄勫姞鍒板瘑閽ョ殑鍊肩殑鍓湰锛屾垨鑰呭鏋滄寚绀轰簡閿欒鍒欒繑鍥炶閿欒銆?

鏈夊叧 request-key 鍑芥暟鐨勬洿澶氫俊鎭紝璇峰弬瑙?Documentation/security/keys/request-key.rst銆?


## 璋冭瘯


鍙互閫氳繃灏?1 鍐欏叆浠ヤ笅鍐呭鏉ュ姩鎬佸紑鍚皟璇曟秷鎭細
```

	/sys/module/dns_resolver/parameters/debug
```
