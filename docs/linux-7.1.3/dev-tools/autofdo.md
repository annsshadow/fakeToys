
灏?AutoFDO 鐢ㄤ簬 Linux 鍐呮牳


鍚敤鍚庯紝鍦ㄤ娇鐢?Clang 缂栬瘧鍣ㄦ椂涓哄唴鏍告彁渚?AutoFDO 鏋勫缓鏀寔銆侫utoFDO锛圓utomatic Feedback-Directed Optimization锛岃嚜鍔ㄥ弽棣堝鍚戜紭鍖栵級鏄竴绉嶅熀浜庢€ц兘鍓栨瀽鐨勪紭鍖栵紙PGO锛夛紝鐢ㄤ簬鎻愬崌浜岃繘鍒跺彲鎵ц鏂囦欢鐨勬€ц兘銆傚畠鍒╃敤纭欢閲囨牱鏀堕泦浜岃繘鍒朵腑鍚勭浠ｇ爜璺緞鐨勬墽琛岄鐜囦俊鎭€傞殢鍚庤繖浜涙暟鎹鐢ㄤ簬鎸囧缂栬瘧鍣ㄧ殑浼樺寲鍐崇瓥锛屼粠鑰岀敓鎴愭洿楂樻晥鐨勪簩杩涘埗鏂囦欢銆侫utoFDO 鏄竴绉嶅己澶х殑浼樺寲鎶€鏈紝鏁版嵁琛ㄦ槑瀹冨彲浠ユ樉钁楁彁鍗囧唴鏍告€ц兘銆傚浜庡彈鍓嶇鍋滈】褰卞搷鐨勫伐浣滆礋杞藉挨涓烘湁鐩娿€?

涓庨潪 FDO 鏋勫缓涓嶅悓锛孉utoFDO 鏋勫缓瑕佹眰鐢ㄦ埛鎻愪緵涓€涓€ц兘鍓栨瀽鏂囦欢锛坧rofile锛夈€傝幏鍙?AutoFDO 鍓栨瀽鏂囦欢鏈夊绉嶆柟寮忋€侫utoFDO 鍓栨瀽鏂囦欢鏄€氳繃 "perf" 宸ュ叿杞崲纭欢閲囨牱鑰屽垱寤虹殑銆傜敤浜庣敓鎴愯繖浜?perf 鏂囦欢鐨勫伐浣滆礋杞藉繀椤诲叿鏈変唬琛ㄦ€э紝瀹冧滑蹇呴』琛ㄧ幇鍑轰笌鎷熶紭鍖栧伐浣滆礋杞界浉浼肩殑杩愯鏃剁壒寰併€傚惁鍒欏皢瀵艰嚧缂栬瘧鍣ㄩ拡瀵归敊璇殑鐩爣杩涜浼樺寲銆?

AutoFDO 鍓栨瀽鏂囦欢閫氬父灏佽浜嗙▼搴忕殑琛屼负銆傚鏋滄€ц兘鍏抽敭浠ｇ爜鏄笌浣撶郴缁撴瀯鏃犲叧鐨勶紝鍒欒鍓栨瀽鏂囦欢鍙法骞冲彴搴旂敤浠ヨ幏寰楁€ц兘鎻愬崌銆備緥濡傦紝浣跨敤鍦?Intel 浣撶郴缁撴瀯涓婄敓鎴愮殑鍓栨瀽鏂囦欢鏉ユ瀯寤洪潰鍚?AMD 浣撶郴缁撴瀯鐨勫唴鏍革紝鍚屾牱鍙互甯︽潵鎬ц兘鏀硅繘銆?

鑾峰彇鍏锋湁浠ｈ〃鎬х殑鍓栨瀽鏂囦欢鏈変袱绉嶆柟娉曪細
(1) 浣跨敤鐢熶骇鐜瀵圭湡瀹炲伐浣滆礋杞借繘琛岄噰鏍枫€?
(2) 浣跨敤鍏锋湁浠ｈ〃鎬х殑璐熻浇娴嬭瘯鐢熸垚鍓栨瀽鏂囦欢銆?
濡傛灉鍦ㄥ惎鐢?AutoFDO 鏋勫缓閰嶇疆鏃舵湭鎻愪緵 AutoFDO 鍓栨瀽鏂囦欢锛岀紪璇戝櫒鍙細淇敼鍐呮牳涓殑 dwarf 淇℃伅锛岃€屼笉浼氬奖鍝嶈繍琛屾椂鎬ц兘銆傚缓璁娇鐢ㄤ互鐩稿悓 AutoFDO 閰嶇疆鏋勫缓鐨勫唴鏍镐簩杩涘埗鏂囦欢鏉ユ敹闆?perf 鍓栨瀽鏂囦欢銆傝櫧鐒朵篃鍙互浣跨敤浠ヤ笉鍚岄€夐」鏋勫缓鐨勫唴鏍革紝浣嗗彲鑳戒細瀵艰嚧鎬ц兘涓嬮檷銆?

鍙互浣跨敤涓婁竴鐗堝唴鏍哥殑 AutoFDO 鏋勫缓鏉ユ敹闆嗗墫鏋愭枃浠躲€侫utoFDO 閲囩敤鐩稿琛屽彿鏉ュ尮閰嶅墫鏋愭枃浠讹紝瀵规簮鐮佸彉鏇存湁涓€瀹氬蹇嶅害銆傝繖绉嶆ā寮忓父鐢ㄤ簬鐢熶骇鐜涓敹闆嗗墫鏋愭枃浠躲€?

鍦ㄥ熀浜庤礋杞芥祴璇曠殑鍓栨瀽鏂囦欢鏀堕泦涓紝AutoFDO 鏀堕泦杩囩▼鍖呭惈浠ヤ笅姝ラ锛?

#. 鍒濆鏋勫缓锛氫娇鐢?AutoFDO 閫夐」鏋勫缓鍐呮牳锛屼絾涓嶅甫鍓栨瀽鏂囦欢銆?

#. 鎬ц兘鍓栨瀽锛氶殢鍚庝娇鐢ㄥ叿鏈変唬琛ㄦ€х殑宸ヤ綔璐熻浇杩愯涓婅堪鍐呮牳锛屼互鏀堕泦鎵ц棰戠巼鏁版嵁銆傝繖浜涙暟鎹€氳繃 perf 鍒╃敤纭欢閲囨牱鏀堕泦銆侫utoFDO 鍦ㄦ敮鎸侀珮绾?PMU 鐗规€э紙濡?Intel 鏈哄櫒涓婄殑 LBR锛夌殑骞冲彴涓婃渶涓烘湁鏁堛€?

#. AutoFDO 鍓栨瀽鏂囦欢鐢熸垚锛氶€氳繃绂荤嚎宸ュ叿灏?perf 杈撳嚭鏂囦欢杞崲涓?AutoFDO 鍓栨瀽鏂囦欢銆?

璇ユ敮鎸侀渶瑕?Clang 缂栬瘧鍣?LLVM 17 鎴栨洿楂樼増鏈€?

鍑嗗宸ヤ綔


```

   CONFIG_AUTOFDO_CLANG=y

```
鑷畾涔?


榛樿鐨?CONFIG_AUTOFDO_CLANG 璁剧疆瑕嗙洊 AutoFDO 鏋勫缓鐨勫唴鏍哥┖闂寸洰鏍囥€備笉杩囷紝鍙互閫氳繃鍦ㄧ浉搴旂殑鍐呮牳 Makefile 涓坊鍔犵被浼间笅闈㈢殑涓€琛岋紝鏉ヤ负鍗曚釜鏂囦欢鎴栫洰褰曞惎鐢ㄦ垨绂佺敤 AutoFDO 鏋勫缓锛?

```

   AUTOFDO_PROFILE_foo.o := y

```
```

   AUTOFDO_PROFILE := y

```
```

   AUTOFDO_PROFILE_foo.o := n

```
```

   AUTOFDO_PROFILE := n

```
宸ヤ綔娴佺▼


浠ヤ笅鏄?AutoFDO 鍐呮牳鐨勭ず渚嬪伐浣滄祦绋嬶細

1) 鍦ㄥ惎鐢ㄤ簡 LLVM 鐨勪富鏈轰笂鏋勫缓鍐呮牳锛?
```

      $ make menuconfig LLVM=1

    Turn on AutoFDO build config::

      CONFIG_AUTOFDO_CLANG=y

    With a configuration that with LLVM enabled, use the following command::

      $ scripts/config -e AUTOFDO_CLANG

    After getting the config, build with ::

      $ make LLVM=1

```
2) 鍦ㄦ祴璇曟満鍣ㄤ笂瀹夎璇ュ唴鏍搞€?

3) 杩愯璐熻浇娴嬭瘯銆俻erf 涓殑 '-c' 閫夐」鎸囧畾閲囨牱浜嬩欢鍛ㄦ湡銆傚缓璁负姝や娇鐢ㄤ竴涓悎閫傜殑绱犳暟锛屼緥濡?500009銆?

```

      $ perf record -e BR_INST_RETIRED.NEAR_TAKEN:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

   - For AMD platforms:

     The supported systems are: Zen3 with BRS, or Zen4 with amd_lbr_v2. To check,

     For Zen3::

      $ cat /proc/cpuinfo | grep " brs"

     For Zen4::

      $ cat /proc/cpuinfo | grep amd_lbr_v2

     The following command generated the perf data file::

      $ perf record --pfm-events RETIRED_TAKEN_BRANCH_INSTRUCTIONS:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

```
4) 锛堝彲閫夛級灏嗗師濮?perf 鏂囦欢涓嬭浇鍒颁富鏈恒€?

5) 瑕佺敓鎴?AutoFDO 鍓栨瀽鏂囦欢锛屾湁涓や釜绂荤嚎宸ュ叿鍙敤锛歝reate_llvm_prof 鍜?llvm_profgen銆俢reate_llvm_prof 宸ュ叿鏄?AutoFDO 椤圭洰鐨勪竴閮ㄥ垎锛屽彲鍦?GitHub锛坔ttps://github.com/google/autofdo锛変笂鎵惧埌锛岀増鏈负 v0.30.1 鎴栨洿楂樸€俵lvm_profgen 宸ュ叿鍖呭惈鍦?LLVM 缂栬瘧鍣ㄦ湰韬腑銆傞渶瑕佹敞鎰忕殑鏄紝llvm_profgen 鐨勭増鏈棤闇€涓?Clang 鐨勭増鏈尮閰嶃€傞渶瑕佺殑鏄?Clang 鐨?LLVM 19 鐗堟湰鍙戝竷銆?
```

      $ llvm-profgen --kernel --binary=<vmlinux> --perfdata=<perf_file> -o <profile_file>

   or ::

      $ create_llvm_prof --binary=<vmlinux> --profile=<perf_file> --format=extbinary --out=<profile_file>

   Note that multiple AutoFDO profile files can be merged into one via::

      $ llvm-profdata merge -o <profile_file> <profile_1> <profile_2> ... <profile_n>

```
6) 浣跨敤涓庢楠?1 鐩稿悓鐨勯厤缃拰 AutoFDO 鍓栨瀽鏂囦欢閲嶆柊鏋勫缓鍐呮牳锛?
```

      $ make LLVM=1 CLANG_AUTOFDO_PROFILE=<profile_file>

```