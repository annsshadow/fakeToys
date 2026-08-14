## GCC 鎻掍欢鍩虹璁炬柦


## 绠€浠?

GCC 鎻掍欢鏄彲鍔犺浇妯″潡锛屼负缂栬瘧鍣?[^1^]_ 鎻愪緵棰濆鐗规€с€傚畠浠杩愯鏃舵彃妗╀笌闈欐€佸垎鏋愬緢鏈夌敤銆?鎴戜滑鍙互鍦ㄧ紪璇戞湡闂撮€氳繃鍥炶皟 [^2^]_銆丟IMPLE [^3^]_銆両PA [^4^]_ 涓?RTL passes [^5^]_ 鏉ュ垎鏋愩€?淇敼骞舵坊鍔犳洿澶氫唬鐮併€?
鍐呮牳鐨?GCC 鎻掍欢鍩虹璁炬柦鏀寔鏋勫缓鏍戝妯″潡銆佷氦鍙夌紪璇戜互鍙婂湪鐙珛鐩綍涓瀯寤恒€傛彃浠舵簮鏂囦欢蹇呴』
鑳藉琚?C++ 缂栬瘧鍣ㄧ紪璇戙€?
鐩墠 GCC 鎻掍欢鍩虹璁炬柦浠呮敮鎸侀儴鍒嗘灦鏋勩€傝 grep "select HAVE_GCC_PLUGINS" 鏉ユ煡鏄庡摢浜涙灦鏋?鏀寔 GCC 鎻掍欢銆?
姝ゅ熀纭€璁炬柦绉绘鑷?grsecurity [^6^]_ 涓?PaX [^7^]_銆?
--

## 鐩殑


GCC 鎻掍欢鏃ㄥ湪鎻愪緵涓€涓敤浜庤瘯楠屾綔鍦ㄧ紪璇戝櫒鐗规€х殑鍦烘墍锛岃繖浜涚壒鎬у湪 GCC 涓?Clang 涓婃父涓兘涓嶅瓨鍦ㄣ€?涓€鏃﹁瘉鏄庡叾瀹炵敤鎬э紝鐩爣灏辨槸灏嗚鐗规€у苟鍏?GCC锛堜笌 Clang锛変笂娓革紝鐒跺悗鏈€缁堝湪鍙楁敮鎸佺殑鎵€鏈?GCC
鐗堟湰閮芥彁渚涜鐗规€у悗锛屽皢鍏朵粠鍐呮牳涓Щ闄ゃ€?
鍏蜂綋鑰岃█锛屾柊鎻掍欢搴斿彧瀹炵幇鍦ㄤ笂娓哥紪璇戝櫒涓紙鏃犺鏄?GCC 杩樻槸 Clang锛夋病鏈夋敮鎸佺殑鐗规€с€?
褰撲竴涓壒鎬у瓨鍦ㄤ簬 Clang 鑰屼笉瀛樺湪浜?GCC 鏃讹紝搴斿姫鍔涘皢璇ョ壒鎬у紩鍏ヤ笂娓?GCC锛堣€屼笉鏄粎浠呬綔涓轰竴涓?鍐呮牳涓撶敤鐨?GCC 鎻掍欢锛夛紝浠ヤ究鏁翠釜鐢熸€侀兘鑳戒粠涓彈鐩娿€?
绫讳技鍦帮紝鍗充究鏌愪釜鐢?GCC 鎻掍欢鎻愪緵鐨勭壒鎬у湪 Clang 涓?*涓?*瀛樺湪锛屼絾鍙璇ョ壒鎬ц璇佹槑鏈夌敤锛屼篃搴?鎶曞叆绮惧姏灏嗗叾骞跺叆 GCC锛堜笌 Clang锛変笂娓搞€?
鍦ㄦ煇涓壒鎬т簬涓婃父 GCC 涓彲鐢ㄥ悗锛岃鎻掍欢灏嗗彉寰楁棤娉曞搴?GCC 鐗堟湰锛堝強涔嬪悗鐗堟湰锛夋瀯寤恒€備竴鏃︽墍鏈?鍐呮牳鏀寔鐨?GCC 鐗堟湰閮芥彁渚涗簡璇ョ壒鎬э紝璇ユ彃浠跺皢浠庡唴鏍镐腑绉婚櫎銆?
## 鏂囦欢


**$(src)/scripts/gcc-plugins**

	杩欐槸 GCC 鎻掍欢鐨勭洰褰曘€?
**$(src)/scripts/gcc-plugins/gcc-common.h**

	杩欐槸涓€涓?GCC 鎻掍欢鐨勫吋瀹瑰ご鏂囦欢銆傚簲濮嬬粓鍖呭惈瀹冿紝鑰屼笉鏄悇涓嫭绔嬬殑 gcc 澶存枃浠躲€?
**$(src)/scripts/gcc-plugins/gcc-generate-gimple-pass.h,
$(src)/scripts/gcc-plugins/gcc-generate-ipa-pass.h,
$(src)/scripts/gcc-plugins/gcc-generate-simple_ipa-pass.h,
$(src)/scripts/gcc-plugins/gcc-generate-rtl-pass.h**

	杩欎簺澶存枃浠惰嚜鍔ㄧ敓鎴?GIMPLE銆丼IMPLE_IPA銆両PA 涓?RTL passes 鐨勬敞鍐岀粨鏋勩€?	搴斾紭鍏堜娇鐢ㄥ畠浠紝鑰岄潪鎵嬪伐鍒涘缓杩欎簺缁撴瀯銆?
## 鐢ㄦ硶


浣犲繀椤讳负浣犵殑 gcc 鐗堟湰瀹夎 gcc 鎻掍欢澶存枃浠讹紝
```

	apt-get install gcc-10-plugin-dev

```
```

	dnf install gcc-plugin-devel libmpc-devel

```
```

	dnf install libmpc-devel

```
鍚敤 GCC 鎻掍欢鍩虹璁炬柦浠ュ強浣犳兂浣跨敤鐨勬煇浜涙彃浠?```

	CONFIG_GCC_PLUGINS=y
	CONFIG_GCC_PLUGIN_LATENT_ENTROPY=y
	...

```
```

	gcc -print-file-name=plugin
	CROSS_COMPILE=arm-linux-gnu- ${CROSS_COMPILE}gcc -print-file-name=plugin

```
```

	plugin

```
```

       /usr/lib/gcc/x86_64-redhat-linux/12/plugin

```
```

	make scripts

```
鎴栬€呯洿鎺ヨ繍琛屽唴鏍?make锛屽苟浣跨敤鐜矾澶嶆潅搴︼紙cyclomatic complexity锛塆CC 鎻掍欢缂栬瘧鏁翠釜鍐呮牳銆?
## 4. 濡備綍娣诲姞涓€涓柊鐨?GCC 鎻掍欢


GCC 鎻掍欢浣嶄簬 scripts/gcc-plugins/ 涓€備綘闇€瑕佸皢鎻掍欢婧愭枃浠剁洿鎺ユ斁鍦?scripts/gcc-plugins/ 涓嬨€?涓嶆敮鎸佸垱寤哄瓙鐩綍銆傚畠蹇呴』琚坊鍔犲埌 scripts/gcc-plugins/Makefile銆乻cripts/Makefile.gcc-plugins
浠ュ強涓€涓浉鍏崇殑 Kconfig 鏂囦欢涓€?