
灏?Propeller 鐢ㄤ簬 Linux 鍐呮牳


鍚敤鍚庯紝鍦ㄤ娇鐢?Clang 缂栬瘧鍣ㄦ椂涓哄唴鏍告彁渚?Propeller 鏋勫缓鏀寔銆侾ropeller 鏄竴绉嶅熀浜庢€ц兘鍓栨瀽鐨勪紭鍖栵紙PGO锛夋柟娉曪紝鐢ㄤ簬浼樺寲浜岃繘鍒跺彲鎵ц鏂囦欢銆備笌 AutoFDO 绫讳技锛屽畠鍒╃敤纭欢閲囨牱鏀堕泦浜岃繘鍒朵腑涓嶅悓浠ｇ爜璺緞鐨勬墽琛岄鐜囦俊鎭€備笌 AutoFDO 涓嶅悓锛岃淇℃伅闅忓悗浼氬湪閾炬帴闃舵涔嬪墠琚敤浜庝紭鍖栵紙鍏朵腑鍖呮嫭锛夊嚱鏁板唴閮ㄥ強璺ㄥ嚱鏁扮殑鍩烘湰鍧楀竷灞€銆?

閲囩敤 Propeller 浼樺寲鏃剁殑涓€浜涢噸瑕佹敞鎰忎簨椤癸細

#. 灏界瀹冨彲浣滀负鐙珛鐨勪紭鍖栨楠や娇鐢紝浣嗗己鐑堝缓璁湪 AutoFDO銆丄utoFDO+ThinLTO 鎴?Instrument FDO 涔嬩笂搴旂敤 Propeller銆傛湰鏂囨。鐨勫叾浣欓儴鍒嗗潎浠ユ鑼冨紡涓哄墠鎻愩€?

#. Propeller 鍦?AutoFDO/AutoFDO+ThinLTO/iFDO 涔嬩笂鍐嶈繘琛屼竴杞€ц兘鍓栨瀽銆傛暣涓瀯寤鸿繃绋嬪寘鎷€渂uild-afdo - train-afdo - build-propeller - train-propeller - build-optimized鈥濄€?

#. Propeller 闇€瑕?Clang/Clang++ 涓庨摼鎺ュ櫒锛坙d.lld锛変负 LLVM 19 鎴栨洿楂樼増鏈€?

#. 闄?LLVM 宸ュ叿閾惧锛孭ropeller 杩橀渶瑕佷竴涓€ц兘鍓栨瀽杞崲宸ュ叿锛歨ttps://github.com/google/autofdo锛岀増鏈渶鍦?v0.30.1 涔嬪悗锛歨ttps://github.com/google/autofdo/releases/tag/v0.30.1銆?

Propeller 浼樺寲杩囩▼鍖呭惈浠ヤ笅姝ラ锛?

#. 鍒濆鏋勫缓锛氬儚閫氬父閭ｆ牱鏋勫缓 AutoFDO 鎴?AutoFDO+ThinLTO 浜岃繘鍒舵枃浠讹紝浣嗛渶甯︿笂涓€缁勭紪璇戞湡/閾炬帴鏈熸爣蹇楋紝浠ヤ究鍦ㄥ唴鏍镐簩杩涘埗鏂囦欢涓垱寤轰竴涓壒娈婄殑鍏冩暟鎹銆傝鐗规畩娈典粎鐢ㄤ簬鎬ц兘鍓栨瀽宸ュ叿锛屽畠涓嶆槸杩愯鏃舵槧鍍忕殑涓€閮ㄥ垎锛屼篃涓嶄細鏀瑰彉鍐呮牳杩愯鏃剁殑鏂囨湰娈点€?

#. 鎬ц兘鍓栨瀽锛氶殢鍚庝娇鐢ㄥ叿鏈変唬琛ㄦ€х殑宸ヤ綔璐熻浇杩愯涓婅堪鍐呮牳锛屼互鏀堕泦鎵ц棰戠巼鏁版嵁銆傝繖浜涙暟鎹€氳繃 perf 鍒╃敤纭欢閲囨牱鏀堕泦銆侾ropeller 鍦ㄦ敮鎸侀珮绾?PMU 鐗规€э紙濡?Intel 鏈哄櫒涓婄殑 LBR锛夌殑骞冲彴涓婃渶涓烘湁鏁堛€傛姝ラ涓庝负 AutoFDO 鍓栨瀽鍐呮牳鐨勮繃绋嬬浉鍚岋紙鍏蜂綋鐨?perf 鍙傛暟鍙兘涓嶅悓锛夈€?

#. Propeller 鍓栨瀽鏂囦欢鐢熸垚锛氶€氳繃绂荤嚎宸ュ叿灏?perf 杈撳嚭鏂囦欢杞崲涓轰竴瀵?Propeller 鍓栨瀽鏂囦欢銆?

#. 浼樺寲鏋勫缓锛氬儚閫氬父閭ｆ牱鏋勫缓 AutoFDO 鎴?AutoFDO+ThinLTO 浼樺寲浜岃繘鍒舵枃浠讹紝浣嗛渶甯︿笂缂栬瘧鏈?閾炬帴鏈熸爣蹇椾互浣跨敤 Propeller 鐨勭紪璇戞湡涓庨摼鎺ユ湡鍓栨瀽鏂囦欢銆傛鏋勫缓姝ラ浣跨敤 3 涓墫鏋愭枃浠垛€斺€擜utoFDO 鍓栨瀽鏂囦欢銆丳ropeller 缂栬瘧鏈熷墫鏋愭枃浠跺拰 Propeller 閾炬帴鏈熷墫鏋愭枃浠躲€?

#. 閮ㄧ讲锛氫紭鍖栧悗鐨勫唴鏍镐簩杩涘埗鏂囦欢琚儴缃插苟鐢ㄤ簬鐢熶骇鐜锛屼粠鑰屾彁渚涙洿楂樼殑鎬ц兘鍜屾洿浣庣殑寤惰繜銆?

鍑嗗宸ヤ綔


```

   CONFIG_AUTOFDO_CLANG=y
   CONFIG_PROPELLER_CLANG=y

```
鑷畾涔?


榛樿鐨?CONFIG_PROPELLER_CLANG 璁剧疆瑕嗙洊 Propeller 鏋勫缓鐨勫唴鏍哥┖闂寸洰鏍囥€備笉杩囷紝鍙互閫氳繃鍦ㄧ浉搴旂殑鍐呮牳 Makefile 涓坊鍔犵被浼间笅闈㈢殑涓€琛岋紝鏉ヤ负鍗曚釜鏂囦欢鎴栫洰褰曞惎鐢ㄦ垨绂佺敤 Propeller 鏋勫缓锛?

```

   PROPELLER_PROFILE_foo.o := y

```
```

   PROPELLER_PROFILE := y

```
```

   PROPELLER_PROFILE_foo.o := n

```
```

   PROPELLER__PROFILE := n


```
宸ヤ綔娴佺▼


浠ヤ笅鏄瀯寤?AutoFDO+Propeller 鍐呮牳鐨勭ず渚嬪伐浣滄祦绋嬶細

1) 鍋囪宸叉寜鐓?AutoFDO 鏂囨。涓殑璇存槑鏀堕泦浜?AutoFDO 鍓栨瀽鏂囦欢锛屽湪涓绘満涓婃瀯寤哄唴鏍?
```

      CONFIG_AUTOFDO_CLANG=y
      CONFIG_PROPELLER_CLANG=y

   and ::

      $ make LLVM=1 CLANG_AUTOFDO_PROFILE=<autofdo-profile-name>

```
2) 鍦ㄦ祴璇曟満鍣ㄤ笂瀹夎璇ュ唴鏍搞€?

3) 杩愯璐熻浇娴嬭瘯銆俻erf 涓殑 '-c' 閫夐」鎸囧畾閲囨牱浜嬩欢鍛ㄦ湡銆傚缓璁负姝や娇鐢ㄤ竴涓悎閫傜殑绱犳暟锛屼緥濡?500009銆?

```

      $ perf record -e BR_INST_RETIRED.NEAR_TAKEN:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

   - For AMD platforms::

      $ perf record --pfm-event RETIRED_TAKEN_BRANCH_INSTRUCTIONS:k -a -N -b -c <count> -o <perf_file> -- <loadtest>

   Note you can repeat the above steps to collect multiple <perf_file>s.

```
4) 锛堝彲閫夛級灏嗗師濮?perf 鏂囦欢涓嬭浇鍒颁富鏈恒€?

5) 浣跨敤 create_llvm_prof 宸ュ叿锛坔ttps://github.com/google/autofdo锛夋潵
```

      $ create_llvm_prof --binary=<vmlinux> --profile=<perf_file>
                         --format=propeller --propeller_output_module_name
                         --out=<propeller_profile_prefix>_cc_profile.txt
                         --propeller_symorder=<propeller_profile_prefix>_ld_profile.txt

   "<propeller_profile_prefix>" can be something like "/home/user/dir/any_string".

   This command generates a pair of Propeller profiles:
   "<propeller_profile_prefix>_cc_profile.txt" and
   "<propeller_profile_prefix>_ld_profile.txt".

   If there are more than 1 perf_file collected in the previous step,
   you can create a temp list file "<perf_file_list>" with each line
   containing one perf file name and run::

      $ create_llvm_prof --binary=<vmlinux> --profile=@<perf_file_list>
                         --format=propeller --propeller_output_module_name
                         --out=<propeller_profile_prefix>_cc_profile.txt
                         --propeller_symorder=<propeller_profile_prefix>_ld_profile.txt

```
6) 浣跨敤 AutoFDO 涓?Propeller 閲嶆柊鏋勫缓鍐呮牳
```

      CONFIG_AUTOFDO_CLANG=y
      CONFIG_PROPELLER_CLANG=y

   and ::

      $ make LLVM=1 CLANG_AUTOFDO_PROFILE=<profile_file> CLANG_PROPELLER_PROFILE_PREFIX=<propeller_profile_prefix>

```