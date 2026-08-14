
## ``amd-pstate`` CPU 鎬ц兘缂╂斁椹卞姩


:Copyright: |copy| 2021 Advanced Micro Devices, Inc.

:Author: Huang Rui <ray.huang@amd.com>


## 寮曡█


`amd-pstate` 鏄?AMD CPU 鎬ц兘缂╂斁椹卞姩锛屽畠鍦ㄧ幇浠?AMD APU 鍜?CPU 绯诲垪涓?
涓?Linux 鍐呮牳寮曞叆浜嗕竴绉嶆柊鐨?CPU 棰戠巼鎺у埗鏈哄埗銆傛柊鏈哄埗鍩轰簬鍗忎綔澶勭悊鍣ㄦ€ц兘鎺у埗
锛圕ollaborative Processor Performance Control锛孋PPC锛夛紝瀹冩彁渚涙瘮浼犵粺 ACPI
纭欢 P-States 鏇寸粏绮掑害鐨勯鐜囩鐞嗐€傚綋鍓?AMD CPU/APU 骞冲彴浣跨敤 ACPI P-states
椹卞姩锛屼粎鑳藉湪 3 涓?P-state 涔嬮棿鍒囨崲鏉ョ鐞?CPU 棰戠巼鍜屾椂閽熴€侰PPC 鍙栦唬浜?ACPI
P-states 鎺у埗锛屽苟涓?Linux 鍐呮牳鎻愪緵浜嗕竴涓伒娲汇€佷綆寤惰繜鐨勬帴鍙ｏ紝鐢ㄤ簬鐩存帴涓庣‖浠?
閫氫俊鎬ц兘鎻愮ず銆?

`amd-pstate` 鍒╃敤 Linux 鍐呮牳鐨勮皟鎺у櫒锛坓overnor锛夛紝渚嬪 `schedutil`銆?
`ondemand` 绛夛紝鏉ョ鐞嗙敱 CPPC 纭欢鍔熻兘鎻愪緵鐨勬€ц兘鎻愮ず锛屽悗鑰呭湪鍐呴儴閬靛惊纭欢
瑙勮寖锛堣鎯呭弬瑙?AMD64 鏋舵瀯绋嬪簭鍛樻墜鍐岀 2 鍗凤細绯荤粺缂栫▼ [^1^]_锛夈€傜洰鍓嶏紝
`amd-pstate` 宸插湪涓€浜?Zen2 鍜?Zen3 澶勭悊鍣ㄤ笂锛屾牴鎹唴鏍歌皟鎺у櫒鏀寔鍩烘湰鐨勯鐜?
鎺у埗鍔熻兘锛屽湪鎴戜滑鍦ㄧ‖浠跺拰 SBIOS 涓婇獙璇佷箣鍚庯紝鏈潵浼氬疄鐜版洿澶?AMD 鐗规湁鐨勫姛鑳姐€?

## AMD CPPC 姒傝堪


鍗忎綔澶勭悊鍣ㄦ€ц兘鎺у埗锛圕ollaborative Processor Performance Control锛孋PPC锛夋帴鍙?
鏋氫妇涓€涓繛缁殑銆佹娊璞＄殑銆佹棤鍗曚綅锛坲nit-less锛夌殑鎬ц兘鍊硷紝鍏跺埢搴﹀苟涓嶇粦瀹氬埌鐗瑰畾鐨?
鎬ц兘鐘舵€?/ 棰戠巼銆傝繖鏄竴椤?ACPI 鏍囧噯 [^2^]_锛岃蒋浠跺彲浠ユ嵁姝ゅ皢搴旂敤鎬ц兘鐩爣鍜屾彁绀?
浣滀负鐩稿鐩爣鎸囧畾缁欏熀纭€璁炬柦鐨勯檺鍒躲€侫MD 澶勭悊鍣ㄦ彁渚涗綆寤惰繜鐨勫瘎瀛樺櫒妯″瀷锛圡SR锛夛紝
鑰屼笉鏄敤 AML 浠ｇ爜瑙ｉ噴鍣ㄦ潵杩涜鎬ц兘璋冩暣銆俙amd-pstate` 浼氱敤鍥炶皟鍒濆鍖栦竴涓?
`struct cpufreq_driver` 瀹炰緥 `amd_pstate_driver`锛?

```

 Highest Perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |          Max Perf  ---->|                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
 Nominal Perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |      Desired Perf  ---->|                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
  Lowest non-        |                       |                         |                       |
  linear perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |          Min perf  ---->|                       |
                     |                       |                         |                       |
  Lowest perf ------>+-----------------------+                         +-----------------------+
                     |                       |                         |                       |
                     |                       |                         |                       |
                     |                       |                         |                       |
          0   ------>+-----------------------+                         +-----------------------+

                                     AMD P-States Performance Scale


```
### AMD CPPC 鎬ц兘鑳藉姏


Highest Performance (RO)
.........................

杩欐槸鍗曚釜澶勭悊鍣ㄥ湪鐞嗘兂鏉′欢涓嬪彲鑳借揪鍒扮殑缁濆鏈€澶ф€ц兘銆傝鎬ц兘姘村钩鍙兘鏃犳硶闀挎椂闂?
鎸佺画锛屽苟涓斿彲鑳戒粎褰撳叾浠栧钩鍙扮粍浠跺浜庣壒瀹氱姸鎬佹椂鎵嶅彲瀹炵幇锛涗緥濡傦紝瀹冨彲鑳借姹傚叾浠?
澶勭悊鍣ㄥ浜庣┖闂茬姸鎬併€傝繖鐩稿綋浜庡鐞嗗櫒鏀寔鐨勬渶楂橀鐜囥€?

Nominal (Guaranteed) Performance (RO)
......................................

杩欐槸澶勭悊鍣ㄥ湪鐞嗘兂杩愯鏉′欢涓嬬殑鏈€澶ф寔缁€ц兘姘村钩銆傚湪娌℃湁澶栭儴绾︽潫锛堝姛鑰椼€佹俯搴︾瓑锛?
鐨勬儏鍐典笅锛岃繖鏄鐞嗗櫒棰勬湡鑳藉鎸佺画缁存寔鐨勬€ц兘姘村钩銆傛墍鏈夋牳 / 澶勭悊鍣ㄩ兘搴旇兘澶熷悓鏃?
缁存寔鍏舵爣绉版€ц兘鐘舵€併€?

Lowest non-linear Performance (RO)
...................................

杩欐槸瀹炵幇闈炵嚎鎬ц妭鑳界殑鏈€浣庢€ц兘姘村钩锛屼緥濡傜敱浜庣數鍘嬪拰棰戠巼缂╂斁鐨勭患鍚堟晥搴斻€傞珮浜庢
闃堝€兼椂锛岃緝浣庣殑鎬ц兘姘村钩閫氬父搴旀瘮杈冮珮鎬ц兘姘村钩鏇磋妭鑳姐€傝瀵勫瓨鍣ㄦ湁鏁堝湴鍚?`amd-pstate`
浼犺揪浜嗘渶楂樻晥鐨勬€ц兘姘村钩銆?

Lowest Performance (RO)
........................

杩欐槸澶勭悊鍣ㄧ殑缁濆鏈€浣庢€ц兘姘村钩銆傞€夋嫨浣庝簬鏈€浣庨潪绾挎€ф€ц兘姘村钩鐨勬€ц兘鍙兘瀵艰嚧鏁堢巼
鎹熷け锛屼絾搴斾細闄嶄綆澶勭悊鍣ㄧ殑鐬椂鍔熻€椼€?

### AMD CPPC 鎬ц兘鎺у埗


`amd-pstate` 閫氳繃杩欎簺瀵勫瓨鍣ㄤ紶閫掓€ц兘鐩爣銆傝瀵勫瓨鍣ㄩ┍鍔ㄦ湡鏈涙€ц兘鐩爣鐨勮涓恒€?

Minimum requested performance (RW)
...................................

`amd-pstate` 鎸囧畾鍏佽鐨勬渶灏忔€ц兘姘村钩銆?

Maximum requested performance (RW)
...................................

`amd-pstate` 鎸囧畾纭欢棰勬湡鎻愪緵鐨勬渶澶ф€ц兘鐨勯檺鍒躲€?

Desired performance target (RW)
...................................

`amd-pstate` 鍦?CPPC 鎬ц兘鍒诲害涓互涓€涓浉瀵规暟瀛楁寚瀹氭湡鏈涚洰鏍囥€傝繖鍙互琛ㄧず涓烘爣绉?
鎬ц兘锛堝熀纭€璁炬柦鏈€澶у€硷級鐨勭櫨鍒嗘瘮銆傚湪鏍囩О鎸佺画鎬ц兘姘村钩浠ヤ笅锛屾湡鏈涙€ц兘琛ㄧず鍙楃‖浠?
绾︽潫鐨勫鐞嗗櫒骞冲潎鎬ц兘姘村钩銆傚湪鏍囩О鎬ц兘姘村钩浠ヤ笂锛屽鐞嗗櫒蹇呴』鑷冲皯鎻愪緵鎵€璇锋眰鐨勬爣绉?
鎬ц兘锛屽苟鍦ㄥ綋鍓嶈繍琛屾潯浠跺厑璁告椂杩涗竴姝ユ彁楂樸€?

Energy Performance Preference (EPP) (RW)
.........................................

璇ュ睘鎬у悜纭欢鎻愪緵涓€涓彁绀猴紝琛ㄧず杞欢甯屾湜鍋忓悜鎬ц兘锛?x0锛夎繕鏄兘鏁堬紙0xff锛夈€?


## 鍏抽敭璋冩帶鍣ㄦ敮鎸?


`amd-pstate` 鍙互涓?`sysfs` 涓?`scaling_available_governors` 绛栫暐灞炴€у垪鍑虹殑
鎵€鏈夛紙閫氱敤锛夌缉鏀捐皟鎺у櫒涓€璧蜂娇鐢ㄣ€傜劧鍚庯紝瀹冭礋璐ｉ厤缃笌 CPU 瀵瑰簲鐨勭瓥鐣ュ璞★紝骞跺悜
`CPUFreq` 鏍稿績锛堜互鍙婇檮鍔犲埌绛栫暐瀵硅薄鐨勭缉鏀捐皟鎺у櫒锛夋彁渚涚‖浠舵敮鎸佺殑鏈€澶у拰鏈€灏忚繍琛?
棰戠巼鐨勫噯纭俊鎭€傜敤鎴峰彲浠ユ煡鐪嬫潵鑷?`CPUFreq` 鏍稿績鐨?`scaling_cur_freq` 淇℃伅銆?

`amd-pstate` 涓昏鏀寔 `schedutil` 鍜?`ondemand` 鐢ㄤ簬鍔ㄦ€侀鐜囨帶鍒躲€傚畠鏄皢
澶勭悊鍣ㄩ厤缃€氳繃 `amd-pstate` 寰皟鍒板甫 CPU CFS 璋冨害鍣ㄧ殑 `schedutil`銆俙amd-pstate`
娉ㄥ唽 adjust_perf 鍥炶皟锛屼互瀹炵幇绫讳技浜?CPPC 鐨勬€ц兘鏇存柊琛屼负銆傚畠鐢?`sugov_start`
鍒濆鍖栵紝鐒跺悗濉厖 CPU 鐨?update_util_data 鎸囬拡锛屽皢 `sugov_update_single_perf`
璧嬪€间负 CPU 璋冨害鍣ㄤ腑鐨勫埄鐢ㄧ巼鏇存柊鍥炶皟鍑芥暟銆侰PU 璋冨害鍣ㄥ皢璋冪敤 `cpufreq_update_util`锛?
骞舵牴鎹鍒╃敤鐜囨洿鏂版墍灞炵殑 `struct sugov_cpu` 鍒嗛厤鐩爣鎬ц兘銆傜劧鍚庯紝`amd-pstate`
鏍规嵁 CPU 璋冨害鍣ㄥ垎閰嶇殑鍊兼洿鏂版湡鏈涙€ц兘銆?


## 澶勭悊鍣ㄦ敮鎸?


濡傛灉妫€娴嬪埌鐨勫鐞嗗櫒涓?ACPI SBIOS 閲屼笉瀛樺湪 `_CPC` 鏉＄洰锛宍amd-pstate` 鐨勫垵濮嬪寲
灏嗗け璐ャ€傚畠浣跨敤 `acpi_cpc_valid` 鏉ユ鏌?`_CPC` 鏄惁瀛樺湪銆傛墍鏈夊熀浜?Zen 鐨勫鐞嗗櫒
閮芥敮鎸佷紶缁熺殑 ACPI 纭欢 P-States 鍔熻兘锛屽洜姝ゅ綋 `amd-pstate` 鍒濆鍖栧け璐ユ椂锛?
鍐呮牳浼氬洖閫€鍘诲垵濮嬪寲 `acpi-cpufreq` 椹卞姩銆?

`amd-pstate` 鏈変袱绉嶇‖浠跺疄鐜帮細涓€绉嶆槸 `Full MSR Support <perf_cap_>`_锛屽彟涓€绉嶆槸
`Shared Memory Support <perf_cap_>`_銆傚畠鍙互浣跨敤 `X86_FEATURE_CPPC` 鐗规€ф爣蹇?
鏉ユ寚绀轰笉鍚岀殑绫诲瀷銆傦紙璇︽儏鍙傝 AMD Family 19h Model 51h Revision A1 澶勭悊鍣ㄧ紪绋?
鍙傝€冩墜鍐岋紙PPR锛塠^3^]_銆傦級`amd-pstate` 浼氫负涓嶅悓鐨勭‖浠跺疄鐜版敞鍐屼笉鍚岀殑 `static_call`
瀹炰緥銆?

鐩墠锛屼竴浜?Zen2 鍜?Zen3 澶勭悊鍣ㄦ敮鎸?`amd-pstate`銆傛湭鏉ワ紝瀹冨皢鍦ㄨ秺鏉ヨ秺澶氱殑 AMD
澶勭悊鍣ㄤ笂寰楀埌鏀寔銆?

### 瀹屾暣 MSR 鏀寔锛團ull MSR Support锛?


涓€浜涙柊鐨?Zen3 澶勭悊鍣紙濡?Cezanne锛夊湪 `X86_FEATURE_CPPC` CPU 鐗规€ф爣蹇楄璁剧疆鏃讹紝
鐩存帴鎻愪緵 MSR 瀵勫瓨鍣ㄣ€俙amd-pstate` 鍙互澶勭悊 MSR 瀵勫瓨鍣紝鍦?`CPUFreq` 涓疄鐜板揩閫?
鍒囨崲锛坒ast switch锛夊姛鑳斤紝浠庤€岄檷浣庝腑鏂笂涓嬫枃涓鐜囨帶鍒剁殑寤惰繜銆傚甫鏈?`pstate_xxx`
鍓嶇紑鐨勫嚱鏁拌〃绀哄 MSR 瀵勫瓨鍣ㄧ殑鎿嶄綔銆?

### 鍏变韩鍐呭瓨鏀寔锛圫hared Memory Support锛?


濡傛灉鏈缃?`X86_FEATURE_CPPC` CPU 鐗规€ф爣蹇楋紝鍒欏鐞嗗櫒鏀寔鍏变韩鍐呭瓨鏂规銆傚湪杩欑
鎯呭喌涓嬶紝`amd-pstate` 浣跨敤 `cppc_acpi` 杈呭姪鏂规硶鏉ュ疄鐜板畾涔夊湪 `static_call` 涓婄殑
鍥炶皟鍑芥暟銆傚甫鏈?`cppc_xxx` 鍓嶇紑鐨勫嚱鏁拌〃绀哄鍏变韩鍐呭瓨鏂规鐨?ACPI CPPC 杈呭姪鏂规硶鐨?
鎿嶄綔銆?


AMD P-States 鍜?ACPI 纭欢 P-States 濮嬬粓鍙互鍦ㄥ悓涓€涓鐞嗗櫒涓婂緱鍒版敮鎸併€備絾 AMD
P-States 鍏锋湁鏇撮珮鐨勪紭鍏堢骇锛屽鏋滃畠閫氳繃 `MSR_AMD_CPPC_ENABLE` 鎴?`cppc_set_enable`
琚惎鐢紝瀹冨皢鍝嶅簲鏉ヨ嚜 AMD P-States 鐨勮姹傘€?


## 鐢ㄦ埛绌洪棿鎺ュ彛锛坄`sysfs``锛夆€斺€?姣忕瓥鐣ユ帶鍒?


`amd-pstate` 鍦?`sysfs` 涓毚闇蹭簡鍑犱釜鍏ㄥ眬灞炴€э紙鏂囦欢锛夋潵鍦ㄧ郴缁熺骇鍒帶鍒跺叾鍔熻兘銆?
瀹冧滑浣嶄簬

```

 root@hr-test1:/home/ray# ls /sys/devices/system/cpu/cpufreq/policy0/*amd*
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_highest_perf
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_hw_prefcore
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_lowest_nonlinear_freq
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_max_freq
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_floor_freq
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_floor_count
 /sys/devices/system/cpu/cpufreq/policy0/amd_pstate_prefcore_ranking


```
`amd_pstate_highest_perf / amd_pstate_max_freq`

椹卞姩鍏佽璁剧疆鐨勬渶澶?CPPC 鎬ц兘鍜?CPU 棰戠巼锛屼互鏈€澶ф敮鎸佺殑 CPPC 鎬ц兘姘村钩
锛堝湪 `AMD CPPC Performance Capability <perf_cap_>`_ 涓殑鏈€楂樻€ц兘锛夌殑鐧惧垎姣旇〃绀恒€?
鍦ㄦ煇浜?ASIC 涓紝鏈€楂樼殑 CPPC 鎬ц兘骞朵笉鍦?`_CPC` 琛ㄤ腑锛屽洜姝ゆ垜浠渶瑕佹妸瀹冩毚闇插埌
sysfs銆傚鏋?boost 鏈縺娲讳絾浠嶅彈鏀寔锛岃鏈€澶ч鐜囧皢澶т簬 `cpuinfo` 涓殑閭ｄ釜銆?
璇ュ睘鎬ф槸鍙鐨勩€?

`amd_pstate_lowest_nonlinear_freq`

椹卞姩鍏佽璁剧疆鐨勬渶浣庨潪绾挎€?CPPC CPU 棰戠巼锛屼互鏈€澶ф敮鎸佺殑 CPPC 鎬ц兘姘村钩鐨勭櫨鍒嗘瘮琛ㄧず銆?
锛堣鍙傝 `AMD CPPC Performance Capability <perf_cap_>`_ 涓殑鏈€浣庨潪绾挎€ф€ц兘銆傦級
璇ュ睘鎬ф槸鍙鐨勩€?

`amd_pstate_hw_prefcore`

骞冲彴鏄惁鏀寔棣栭€夋牳锛坧referred core锛夌壒鎬у苟涓斿凡鍚敤銆傝灞炴€ф槸鍙鐨勩€傝繖涓枃浠?
鍙湪鏀寔棣栭€夋牳鐗规€х殑骞冲彴涓婂彲瑙併€?

`amd_pstate_prefcore_ranking`

璇ユ牳鐨勬€ц兘鎺掑悕銆傝繖涓暟瀛楁病鏈変换浣曞崟浣嶏紝浣嗗湪璇诲彇鏃舵暟鍊艰秺澶ц秺琚紭鍏堛€傚畠浼氭牴鎹?
骞冲彴鏉′欢鍦ㄨ繍琛屾椂鍙樺寲銆傝灞炴€ф槸鍙鐨勩€傝繖涓枃浠跺彧鍦ㄦ敮鎸侀閫夋牳鐗规€х殑骞冲彴涓婂彲瑙併€?

`amd_pstate_floor_freq`

涓庢瘡涓?CPU 鍏宠仈鐨勫湴鏉块鐜囷紙floor frequency锛夈€傜敤鎴风┖闂村彲浠ュ悜璇ユ枃浠跺啓鍏?
`cpuinfo_min_freq` 鍜?`scaling_max_freq` 涔嬮棿鐨勪换鎰忓€笺€傚綋绯荤粺澶勪簬鍔熻€楁垨娓╁害
绾︽潫涓嬫椂锛屽钩鍙板浐浠朵細灏濊瘯鍏堝皢 CPU 棰戠巼闄愬埗鍒?`amd_pstate_floor_freq` 涓寚瀹氱殑
鍊硷紝鐒跺悗鍐嶈繘涓€姝ラ檺鍒躲€傝繖鍏佽鐢ㄦ埛绌洪棿涓轰笉鍚岀殑 CPU 鎸囧畾涓嶅悓鐨勫湴鏉块鐜囥€備负浜嗚幏寰?
鏈€浣崇粨鏋滐紝鍚屼竴鏍哥殑绾跨▼搴斿叿鏈夌浉鍚岀殑鍦版澘棰戠巼鍊笺€傝繖涓枃浠跺彧鍦ㄦ敮鎸?CPPC 鎬ц兘浼樺厛绾?
锛圥erformance Priority锛夌壒鎬х殑骞冲彴涓婂彲瑙併€?


`amd_pstate_floor_count`

骞冲彴鏀寔鐨勪笉鍚岀殑鍦版澘鎬ц兘锛團loor Performance锛夌骇鍒殑鏁伴噺銆備緥濡傦紝濡傛灉璇ュ€间负 2锛?
閭ｄ箞浠庡懡浠?``cat
/sys/devices/system/cpu/cpufreq/policy*/amd_pstate_floor_freq |
sort -n | uniq`` 鑾峰緱鐨勫敮涓€鍊肩殑鏁伴噺锛屽浜?`amd_pstate_floor_freq` 涓弿杩扮殑琛屼负
鐢熸晥鑰岃█锛屾渶澶氬簲涓鸿鏁板瓧銆傞浂鍊艰〃绀哄钩鍙版敮鎸佹棤闄愬鐨勫湴鏉挎€ц兘绾у埆銆傝繖涓枃浠跺彧鍦?
鏀寔 CPPC 鎬ц兘浼樺厛绾х壒鎬х殑骞冲彴涓婂彲瑙併€?

**娉ㄦ剰**锛氬綋 `amd_pstate_floor_count` 闈為浂鏃讹紝濡傛灉鍦ㄥ姛鐜囨垨娓╁害绾︽潫涓嬪 CPU 杩涜
闄愬埗鐨勯鐐癸紝鍦ㄧ郴缁熶腑鎵€鏈?CPU 鐨?`amd_pstate_floor_freq` 鍞竴鍊兼暟閲忚秴杩?
`amd_pstate_floor_count` 鏃讹紝鏄湭瀹氫箟鐨勩€?

`energy_performance_available_preferences`

鍙敤浜庢湰绯荤粺 `energy_performance_preference` 鐨勬墍鏈夊彈鏀寔鐨?EPP 鍋忓ソ鐨勫垪琛ㄣ€?
杩欎簺閰嶇疆鏂囦欢浠ｈ〃浜嗘彁渚涚粰搴曞眰鍥轰欢鐨勪笉鍚屾彁绀猴紝鍏充簬鐢ㄦ埛鎵€鏈熸湜鐨勮兘鏁堜笌鎬ц兘鏉冭　銆?
`default` 琛ㄧず epp 鍊肩敱骞冲彴鍥轰欢璁剧疆銆俙custom` 琛ㄧず涔熷彲浠ュ啓鍏?0-255 鐨勬暣鏁板€笺€?
璇ュ睘鎬ф槸鍙鐨勩€?

`energy_performance_preference`

褰撳墠鐨勮兘鏁堟€ц兘鍋忓ソ鍙互浠庤灞炴€ц鍙栵紝鐢ㄦ埛鍙互鏍规嵁鑳芥晥鎴栨€ц兘闇€姹傛洿鏀瑰綋鍓嶅亸濂姐€?
璇ュ睘鎬т腑鎻愪緵浜嗙矖绮掑害鐨勫懡鍚嶉厤缃枃浠?
`energy_performance_available_preferences`銆?
鐢ㄦ埛涔熷彲浠ュ啓鍏?0 鍒?255 涔嬮棿鐨勫崟涓暣鏁板€笺€?
褰撳惎鐢ㄤ簡鍔ㄦ€?EPP 鏃讹紝鍗充娇骞冲彴鍥轰欢宸插惎鐢?EPP 鐗规€э紝瀵?energy_performance_preference
鐨勫啓鍏ヤ篃浼氳闃绘銆傝緝浣庣殑 epp 鍊间細灏嗗亸鍚戣浆鍚戞敼杩涙€ц兘锛岃€岃緝楂樼殑 epp 鍊间細灏嗗亸鍚?
杞悜鑺傝兘銆傜‘鍒囩殑褰卞搷浼氬洜骞冲彴鑰屽紓銆?
濡傛灉鏈€鍚庝竴娆″啓鍏ョ殑鏄湁鏁堟暣鏁帮紝鍒欐湭鏉ヨ鍙栨椂灏嗚繑鍥炰竴涓暟瀛椼€?
濡傛灉鏈€鍚庝竴娆″啓鍏ョ殑鏄湁鏁堝瓧绗︿覆锛屽垯鏈潵璇诲彇鏃跺皢杩斿洖涓€涓瓧绗︿覆銆?
璇ュ睘鎬ф槸鍙鍐欑殑銆?

`boost`
`boost` sysfs 灞炴€ф彁渚涘 CPU 鏍告€ц兘 boost 鐨勬帶鍒讹紝鍏佽鐢ㄦ埛绠＄悊 CPU 鐨勬渶澶?
棰戠巼闄愬埗銆傝灞炴€у彲鐢ㄤ簬鍦ㄥ崟涓?CPU 涓婂惎鐢ㄦ垨绂佺敤 boost 鐗规€с€?

褰?boost 鐗规€у惎鐢ㄦ椂锛孋PU 鍙互鍔ㄦ€佸湴灏嗛鐜囨彁鍗囧埌鍩虹棰戠巼涔嬩笂锛屼负瑕佹眰鑻涘埢鐨?
宸ヤ綔璐熻浇鎻愪緵澧炲己鐨勬€ц兘銆傚彟涓€鏂归潰锛岀鐢?boost 鐗规€т細灏?CPU 闄愬埗鍦ㄥ熀纭€棰戠巼涓?
杩愯锛屽湪鏌愪簺鍦烘櫙涓嬩负浜嗕紭鍏堣€冭檻鑳芥晥鎴栫鐞嗘俯搴︼紝杩欏彲鑳芥槸鍙彇鐨勩€?

瑕佹搷浣?`boost` 灞炴€э紝鐢ㄦ埛鍙互浣跨敤 sysfs 璺緞
`/sys/devices/system/cpu/cpuX/cpufreq/boost`锛堝叾涓?`X` 琛ㄧず CPU 缂栧彿锛夛紝鍚?
鐩稿簲鐨?CPU 鍐欏叆鍊?`0` 鏉ョ鐢?boost锛屾垨鍐欏叆 `1` 鏉ュ惎鐢?boost銆?

鍏朵粬鎬ц兘鍜岄鐜囧€煎彲浠ヤ粠
`/sys/devices/system/cpu/cpuX/acpi_cppc/` 璇诲洖锛屽弬瑙?cppc_sysfs銆?

## 鍔ㄦ€佽兘鏁堟€ц兘閰嶇疆鏂囦欢


amd-pstate 椹卞姩鏀寔鏍规嵁鏈哄櫒鏄繍琛屽湪浜ゆ祦锛圓C锛夎繕鏄洿娴侊紙DC锛夌數婧愪笂锛屽姩鎬佸湴
閫夋嫨鑳芥晥鎬ц兘閰嶇疆鏂囦欢銆?

姝よ涓烘槸鍚﹂粯璁ゅ惎鐢ㄥ彇鍐充簬鍐呮牳鍛戒护琛岄€夐」 `amd_dynamic_epp` 鏄惁琚缃€傝琛屼负
涔熷彲浠ュ湪杩愯鏃堕€氳繃 sysfs 鏂囦欢
`/sys/devices/system/cpu/amd_pstate/dynamic_epp` 琚鐩栥€?

褰撹缃负鍚敤鏃讹紝椹卞姩浼氬湪鏈哄櫒杩愯鍦ㄧ數姹犳垨浜ゆ祦鐢垫簮涓婃椂閫夋嫨涓嶅悓鐨勮兘鏁堟€ц兘閰嶇疆鏂囦欢銆?
椹卞姩杩樹細鍚戝钩鍙伴厤缃枃浠跺鐞嗙▼搴忥紙platform profile handler锛夋敞鍐岋紝浠ユ帴鏀剁敤鎴?
鏈熸湜鐨勭數婧愮姸鎬侀€氱煡骞跺仛鍑哄弽搴斻€傚綋璁剧疆涓虹鐢ㄦ椂锛岄┍鍔ㄤ笉浼氭牴鎹數婧愭潵婧愭敼鍙樿兘鏁?
鎬ц兘閰嶇疆鏂囦欢锛屼篃涓嶄細瀵圭敤鎴风殑鏈熸湜鐢垫簮鐘舵€佸仛鍑哄弽搴斻€?

褰?`dynamic_epp` 鍚敤鏃讹紝灏濊瘯鎵嬪姩鍐欏叆 `energy_performance_preference` sysfs
鏂囦欢灏嗕細澶辫触銆?

## ``amd-pstate`` 涓?``acpi-cpufreq`` 瀵规瘮


鍦?`acpi-cpufreq` 鏀寔鐨勫ぇ澶氭暟 AMD 骞冲彴涓婏紝骞冲彴鍥轰欢鎻愪緵鐨?ACPI 琛ㄧ敤浜?CPU
鎬ц兘缂╂斁锛屼絾鍦?AMD 澶勭悊鍣ㄤ笂浠呮彁渚?3 涓?P-state銆傜劧鑰岋紝鍦ㄧ幇浠?AMD APU 鍜?CPU
绯诲垪涓婏紝纭欢鏍规嵁 ACPI 鍗忚鎻愪緵鍗忎綔澶勭悊鍣ㄦ€ц兘鎺у埗锛屽苟閽堝 AMD 骞冲彴杩涜浜嗗畾鍒躲€?
涔熷氨鏄锛屾槸缁嗙矑搴︿笖杩炵画鐨勯鐜囪寖鍥达紝鑰屼笉鏄紶缁熺殑纭欢 P-states銆俙amd-pstate`
鏄敮鎸佹湭鏉ュぇ澶氭暟 AMD 骞冲彴涓婃柊鐨?AMD P-States 鏈哄埗鐨勫唴鏍告ā鍧椼€侫MD P-States
鏈哄埗鏄?AMD 澶勭悊鍣ㄤ笂鎬ц兘鍜岃兘鏁堟洿楂樼殑棰戠巼绠＄悊鏂规硶銆?


## ``amd-pstate`` 椹卞姩杩愯妯″紡


`amd_pstate` CPPC 鏈?3 绉嶈繍琛屾ā寮忥細鑷富锛坅ctive锛夋ā寮忋€侀潪鑷富锛坧assive锛夋ā寮?
鍜屽紩瀵艰嚜涓伙紙guided锛夋ā寮忋€傚彲浠ラ€氳繃涓嶅悓鐨勫唴鏍稿弬鏁伴€夋嫨 active/passive/guided 妯″紡銆?

- 鍦ㄨ嚜涓绘ā寮忎笅锛屽钩鍙板拷鐣ユ湡鏈涙€ц兘姘村钩鐨勮姹傦紝鍙€冭檻璁剧疆鍒版渶灏忓€笺€佹渶澶у€煎拰
  鑳芥晥鎬ц兘鍋忓ソ瀵勫瓨鍣ㄤ腑鐨勫€笺€?
- 鍦ㄩ潪鑷富妯″紡涓嬶紝骞冲彴閫氳繃鏈熸湜鎬ц兘瀵勫瓨鍣紙Desired Performance Register锛夌洿鎺ヤ粠
  OS 鑾峰彇鏈熸湜鎬ц兘姘村钩銆?
- 鍦ㄥ紩瀵艰嚜涓绘ā寮忎笅锛屽钩鍙版牴鎹綋鍓嶅伐浣滆礋杞斤紝骞跺湪 OS 閫氳繃鏈€灏忓拰鏈€澶ф€ц兘瀵勫瓨鍣ㄨ瀹氱殑
  闄愬埗鑼冨洿鍐咃紝鑷富鍦拌缃繍琛屾€ц兘姘村钩銆?

### 涓诲姩妯″紡锛圓ctive Mode锛?


`amd_pstate=active`

杩欐槸鐢?`amd_pstate_epp` 椹卞姩瀹炵幇鐨勫簳灞傚浐浠舵帶鍒舵ā寮忥紝閫氳繃鍦ㄥ懡浠よ鍚戝唴鏍镐紶閫?
`amd_pstate=active` 鏉ュ惎鐢ㄣ€傚湪姝ゆā寮忎笅锛宍amd_pstate_epp` 椹卞姩鍚戠‖浠舵彁渚涗竴涓?
鎻愮ず锛岃〃绀鸿蒋浠舵兂瑕佸亸鍚戞€ц兘锛?x0锛夎繕鏄兘鏁堬紙0xff锛夊埌 CPPC 鍥轰欢銆傜劧鍚?CPPC 鐢垫簮
绠楁硶灏嗘牴鎹數婧愪緵搴斿拰娓╁害銆佹牳鐢靛帇浠ュ強鍏朵粬涓€浜涚‖浠舵潯浠惰绠楄繍琛屾椂宸ヤ綔璐熻浇骞惰皟鏁?
瀹炴椂鏍搁鐜囥€?

### 琚姩妯″紡锛圥assive Mode锛?


`amd_pstate=passive`

濡傛灉鍦ㄥ懡浠よ鍚戝唴鏍镐紶閫掍簡 `amd_pstate=passive`锛屽垯浼氬惎鐢ㄦ妯″紡銆傚湪姝ゆā寮忎笅锛?
`amd_pstate` 椹卞姩杞欢鍦?CPPC 鎬ц兘鍒诲害涓互涓€涓浉瀵规暟瀛楁寚瀹氭湡鏈涚殑 QoS 鐩爣銆?
杩欏彲浠ヨ〃绀轰负鏍囩О鎬ц兘锛堝熀纭€璁炬柦鏈€澶у€硷級鐨勭櫨鍒嗘瘮銆傚湪鏍囩О鎸佺画鎬ц兘姘村钩浠ヤ笅锛?
鏈熸湜鎬ц兘琛ㄧず鍙楁€ц兘闄嶄綆瀹瑰樊锛圥erformance Reduction Tolerance锛夊瘎瀛樺櫒绾︽潫鐨?
澶勭悊鍣ㄥ钩鍧囨€ц兘姘村钩銆傚湪鏍囩О鎬ц兘姘村钩浠ヤ笂锛屽鐞嗗櫒蹇呴』鑷冲皯鎻愪緵鎵€璇锋眰鐨勬爣绉版€ц兘锛?
骞跺湪褰撳墠杩愯鏉′欢鍏佽鏃惰繘涓€姝ユ彁楂樸€?

### 寮曞妯″紡锛圙uided Mode锛?


`amd_pstate=guided`

濡傛灉鍦ㄥ唴鏍稿懡浠よ閫夐」涓紶閫掍簡 `amd_pstate=guided`锛屽垯浼氭縺娲绘妯″紡銆傚湪姝ゆā寮忎笅锛?
椹卞姩璇锋眰鏈€灏忓拰鏈€澶ф€ц兘姘村钩锛屽钩鍙板湪璇ヨ寖鍥村唴鑷富閫夋嫨涓€涓€傚悎褰撳墠宸ヤ綔璐熻浇鐨?
鎬ц兘姘村钩銆?

## ``amd-pstate`` 棣栭€夋牳锛圥referred Core锛?


鏍搁鐜囧彈鍒朵簬鍗婂浣撲腑鐨勫伐鑹哄樊寮傘€傚苟闈炴墍鏈夋牳閮借兘鍦ㄩ伒瀹堝熀纭€璁炬柦闄愬埗鐨勬儏鍐典笅杈惧埌
鏈€澶ч鐜囥€傚洜姝わ紝AMD 閲嶆柊瀹氫箟浜嗛儴浠舵渶澶ч鐜囩殑姒傚康銆傝繖鎰忓懗鐫€涓€閮ㄥ垎鏍稿彲浠ヨ揪鍒?
鏈€澶ч鐜囥€備负浜嗙粰缁欏畾鍦烘櫙鎵惧埌鏈€浣崇殑杩涚▼璋冨害绛栫暐锛孫S 闇€瑕侀€氳繃 CPPC 鎺ュ彛鐨勬渶楂?
鎬ц兘鑳藉姏瀵勫瓨鍣ㄨ幏鐭ュ钩鍙板憡鐭ョ殑鏍告帓搴忋€?

`amd-pstate` 棣栭€夋牳浣胯皟搴﹀櫒浼樺厛璋冨害鍦ㄨ兘澶熶互鏇翠綆鐢靛帇杈惧埌鏇撮珮棰戠巼鐨勬牳涓娿€傞閫夋牳
鎺掑悕鍙互鏍规嵁宸ヤ綔璐熻浇銆佸钩鍙版潯浠躲€佹俯搴﹀拰鑰佸寲鑰屽姩鎬佸彉鍖栥€?

浼樺厛绾у害閲忓皢鐢?`amd-pstate` 椹卞姩鍒濆鍖栥€俙amd-pstate` 椹卞姩杩樺皢纭畾骞冲彴鏄惁鏀寔
`amd-pstate` 棣栭€夋牳銆?

`amd-pstate` 椹卞姩灏嗗湪绯荤粺鍚姩鏃舵彁渚涗竴涓垵濮嬬殑鏍告帓搴忋€傚钩鍙颁娇鐢?CPPC 鎺ュ彛灏嗘牳
鎺掑悕浼犺揪缁欐搷浣滅郴缁熷拰璋冨害鍣紝浠ョ‘淇?OS 浼樺厛閫夋嫨鍏锋湁鏈€楂樻€ц兘鐨勬牳鏉ヨ皟搴﹁繘绋嬨€傚綋
`amd-pstate` 椹卞姩鏀跺埌鏈€楂樻€ц兘鍙樺寲鐨勬秷鎭椂锛屽畠灏嗘洿鏂版牳鎺掑悕骞惰缃?cpu 鐨勪紭鍏堢骇銆?

## ``amd-pstate`` 棣栭€夋牳寮€鍏?


### 鍐呮牳鍙傛暟


`amd-pstate` peferred core`` 鏈変袱绉嶇姸鎬侊細鍚敤鍜岀鐢ㄣ€?
鍙互閫氳繃涓嶅悓鐨勫唴鏍稿弬鏁伴€夋嫨鍚敤 / 绂佺敤鐘舵€併€?
榛樿鍚敤 `amd-pstate` 棣栭€夋牳銆?

`amd_prefcore=disable`

瀵逛簬鏀寔 `amd-pstate` 棣栭€夋牳鐨勭郴缁燂紝鏍告帓鍚嶅皢濮嬬粓鐢卞钩鍙伴€氬憡銆備絾 OS 鍙互閫氳繃
鍐呮牳鍙傛暟 `amd_prefcore=disable` 閫夋嫨蹇界暐瀹冦€?

`amd_dynamic_epp`

褰?AMD pstate 澶勪簬鑷姩妯″紡鏃讹紝鍔ㄦ€?EPP 灏嗘帶鍒跺唴鏍告槸鍚﹁嚜涓绘洿鏀?EPP 妯″紡銆傞粯璁?
涓虹鐢ㄣ€傚彲浠ラ€氳繃鍐呮牳鍙傛暟 `amd_dynamic_epp=enable` 鍚敤銆?

## 鐢ㄦ埛绌洪棿鎺ュ彛锛坄`sysfs``锛夆€斺€?閫氱敤


### 鍏ㄥ眬灞炴€?


`amd-pstate` 鍦?`sysfs` 涓毚闇蹭簡鍑犱釜鍏ㄥ眬灞炴€э紙鏂囦欢锛夋潵鍦ㄧ郴缁熺骇鍒帶鍒跺叾鍔熻兘銆?
瀹冧滑浣嶄簬 `/sys/devices/system/cpu/amd_pstate/` 鐩綍锛屽苟褰卞搷鎵€鏈?CPU銆?

`status`
	椹卞姩鐨勮繍琛屾ā寮忥細"active"銆?passive"銆?guided" 鎴?"disable"銆?

	"active"
		椹卞姩澶勪簬鍙敤鐘舵€侊紝骞跺浜?`active mode`

	"passive"
		椹卞姩澶勪簬鍙敤鐘舵€侊紝骞跺浜?`passive mode`

	"guided"
		椹卞姩澶勪簬鍙敤鐘舵€侊紝骞跺浜?`guided mode`

	"disable"
		椹卞姩宸叉敞閿€锛屽綋鍓嶄笉鍙敤銆?

        鍙互鍐欏叆璇ュ睘鎬т互鏇存敼椹卞姩鐨勮繍琛屾ā寮忔垨娉ㄩ攢瀹冦€傚啓鍏ョ殑瀛楃涓插繀椤绘槸鍏跺彲鑳藉€?
        涔嬩竴锛屽鏋滄垚鍔燂紝鍚戣 sysfs 鏂囦欢鍐欏叆杩欎簺鍊间箣涓€灏嗗鑷撮┍鍔ㄥ垏鎹㈠埌璇ュ瓧绗︿覆
        鎵€浠ｈ〃鐨勮繍琛屾ā寮忊€斺€旀垨鍦?"disable" 鎯呭喌涓嬭娉ㄩ攢銆?

`prefcore`
	椹卞姩鐨勯閫夋牳鐘舵€侊細"enabled" 鎴?"disabled"銆?

	"enabled"
		鍚敤 `amd-pstate` 棣栭€夋牳銆?

	"disabled"
		绂佺敤 `amd-pstate` 棣栭€夋牳


        璇ュ睘鎬ф槸鍙鐨勶紝鐢ㄤ簬妫€鏌ョ敱鍐呮牳鍙傛暟璁剧疆鐨勯閫夋牳鐘舵€併€?

## ``cpupower`` 宸ュ叿瀵?``amd-pstate`` 鐨勬敮鎸?


`amd-pstate` 鐢?`cpupower` 宸ュ叿鏀寔锛岃宸ュ叿鍙敤浜庤浆鍌ㄩ鐜囦俊鎭€傜洰鍓嶆鍦ㄥ紑鍙戜腑锛?
浠ユ敮鎸佽秺鏉ヨ秺澶氱殑

```

 root@hr-test1:/home/ray# cpupower frequency-info
 analyzing CPU 0:
   driver: amd-pstate
   CPUs which run at the same hardware frequency: 0
   CPUs which need to have their frequency coordinated by software: 0
   maximum transition latency: 131 us
   hardware limits: 400 MHz - 4.68 GHz
   available cpufreq governors: ondemand conservative powersave userspace performance schedutil
   current policy: frequency should be within 400 MHz and 4.68 GHz.
                   The governor "schedutil" may decide which speed to use
                   within this range.
   current CPU frequency: Unable to call hardware
   current CPU frequency: 4.02 GHz (asserted by call to kernel)
   boost state support:
     Supported: yes
     Active: yes
     AMD PSTATE Highest Performance: 166. Maximum Frequency: 4.68 GHz.
     AMD PSTATE Nominal Performance: 117. Nominal Frequency: 3.30 GHz.
     AMD PSTATE Lowest Non-linear Performance: 39. Lowest Non-linear Frequency: 1.10 GHz.
     AMD PSTATE Lowest Performance: 15. Lowest Frequency: 400 MHz.


```
## 璇婃柇涓庤皟浼?


### 璺熻釜浜嬩欢锛圱race Events锛?


鏈変袱涓潤鎬佽窡韪簨浠跺彲鐢ㄤ簬 `amd-pstate` 鐨勮瘖鏂€傚叾涓竴涓槸 `cpu_frequency` 璺熻釜
浜嬩欢锛岄€氬父鐢?`CPUFreq` 浣跨敤锛涘彟涓€涓槸 `amd_pstate_perf` 璺熻釜浜嬩欢锛岀壒瀹氫簬
`amd-pstate`銆傚彲浠ヤ娇鐢ㄤ互涓?shell 鍛戒护搴忓垪鏉ュ惎鐢ㄥ畠浠苟鏌ョ湅鍏惰緭鍑猴紙濡傛灉鍐呮牳鏄?

```

 root@hr-test1:/home/ray# cd /sys/kernel/tracing/
 root@hr-test1:/home/ray# echo 1 > events/amd_cpu/enable
 root@hr-test1:/home/ray# cat trace
 # tracer: nop
 #
 # entries-in-buffer/entries-written: 47827/42233061   #P:2
 #
 #                                _-----=> irqs-off
 #                               / _----=> need-resched
 #                              | / _---=> hardirq/softirq
 #                              || / _--=> preempt-depth
 #                              ||| /     delay
 #           TASK-PID     CPU#  ||||   TIMESTAMP  FUNCTION
 #              | |         |   ||||      |         |
          <idle>-0       [015] dN...  4995.979886: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=15 changed=false fast_switch=true
          <idle>-0       [007] d.h..  4995.979893: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=7 changed=false fast_switch=true
             cat-2161    [000] d....  4995.980841: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=0 changed=false fast_switch=true
            sshd-2125    [004] d.s..  4995.980968: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=4 changed=false fast_switch=true
          <idle>-0       [007] d.s..  4995.980968: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=7 changed=false fast_switch=true
          <idle>-0       [003] d.s..  4995.980971: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=3 changed=false fast_switch=true
          <idle>-0       [011] d.s..  4995.980996: amd_pstate_perf: amd_min_perf=85 amd_des_perf=85 amd_max_perf=166 cpu_id=11 changed=false fast_switch=true

```
`cpu_frequency` 璺熻釜浜嬩欢浼氱敱 `schedutil` 缂╂斁璋冩帶鍣紙瀵逛簬瀹冩墍闄勫姞鐨勭瓥鐣ワ級鎴?
鐢?`CPUFreq` 鏍稿績锛堝浜庝娇鐢ㄥ叾浠栫缉鏀捐皟鎺у櫒鐨勭瓥鐣ワ級瑙﹀彂銆?


### 璺熻釜宸ュ叿锛圱racer Tool锛?


`amd_pstate_tracer.py` 鍙互璁板綍鍜岃В鏋?`amd-pstate` 璺熻釜鏃ュ織锛岀劧鍚庣敓鎴愭€ц兘鍥俱€?
璇ュ伐鍏峰彲鐢ㄤ簬璋冭瘯鍜岃皟浼?`amd-pstate` 椹卞姩鐨勬€ц兘銆傝璺熻釜宸ュ叿闇€瑕佸鍏?intel pstate
璺熻釜鍣ㄣ€?

璺熻釜宸ュ叿浣嶄簬 `linux/tools/power/x86/amd_pstate_tracer`銆傚畠鏈変袱绉嶄娇鐢ㄦ柟寮忋€傚鏋?
璺熻釜鏂囦欢鍙敤锛屽垯鐩存帴瑙ｆ瀽璇ユ枃浠?

```

 ./amd_pstate_trace.py [-c cpus] -t <trace_file> -n <test_name>


```
```

 sudo ./amd_pstate_trace.py [-c cpus] -n <test_name> -i <interval> [-m kbytes]


```
娴嬭瘯缁撴灉鍙互鍦?`results/test_name` 涓壘鍒般€備互涓嬫槸绀轰緥

```

 common_cpu  common_secs  common_usecs  min_perf  des_perf  max_perf  freq    mperf   apef    tsc       load   duration_ms  sample_num  elapsed_time  common_comm
 CPU_005     712          116384        39        49        166       0.7565  9645075 2214891 38431470  25.1   11.646       469         2.496         kworker/5:0-40
 CPU_006     712          116408        39        49        166       0.6769  8950227 1839034 37192089  24.06  11.272       470         2.496         kworker/6:0-1264


```
### amd-pstate 鐨勫崟鍏冩祴璇?


`amd-pstate-ut` 鏄竴涓敤浜庢祴璇?`amd-pstate` 椹卞姩鐨勬祴璇曟ā鍧椼€?

 - 瀹冨彲浠ュ府鍔╂墍鏈夌敤鎴烽獙璇佷粬浠殑澶勭悊鍣ㄦ敮鎸侊紙SBIOS/鍥轰欢鎴栫‖浠讹級銆?

 - 鍐呮牳鍙互鏈変竴涓熀鏈殑鍔熻兘娴嬭瘯锛屼互閬垮厤鍦ㄦ洿鏂版湡闂村彂鐢熷唴鏍稿洖褰掋€?

 - 鎴戜滑鍙互寮曞叆鏇村鐨勫姛鑳芥垨鎬ц兘娴嬭瘯鏉ュ榻愮粨鏋滐紝杩欏皢鏈夊埄浜庡姛鑰楀拰鎬ц兘瑙勬ā鐨勪紭鍖栥€?

1. 娴嬭瘯鐢ㄤ緥鎻忚堪

    1). 鍩烘湰娴嬭瘯

        鐢ㄤ簬 `amd-pstate` 椹卞姩鐨勫墠缃潯浠跺拰鍩烘湰鍔熻兘銆?

        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | Index   | Functions                      | Description                                                                        |
        +=========+================================+====================================================================================+
        | 1       | amd_pstate_ut_acpi_cpc_valid   || Check whether the _CPC object is present in SBIOS.                                |
        |         |                                ||                                                                                   |
        |         |                                || The detail refer to `Processor Support <processor_support_>`_.                    |
        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | 2       | amd_pstate_ut_check_enabled    || Check whether AMD P-State is enabled.                                             |
        |         |                                ||                                                                                   |
        |         |                                || AMD P-States and ACPI hardware P-States always can be supported in one processor. |
        |         |                                | But AMD P-States has the higher priority and if it is enabled with                 |
        |         |                                | `MSR_AMD_CPPC_ENABLE` or `cppc_set_enable`, it will respond to the      |
        |         |                                | request from AMD P-States.                                                         |
        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | 3       | amd_pstate_ut_check_perf       || Check if the each performance values are reasonable.                              |
        |         |                                || highest_perf >= nominal_perf > lowest_nonlinear_perf > lowest_perf > 0.           |
        +---------+--------------------------------+------------------------------------------------------------------------------------+
        | 4       | amd_pstate_ut_check_freq       || Check if the each frequency values and max freq when set support boost mode       |
        |         |                                | are reasonable.                                                                    |
        |         |                                || max_freq >= nominal_freq > lowest_nonlinear_freq > min_freq > 0                   |
        |         |                                || If boost is not active but supported, this maximum frequency will be larger than  |
        |         |                                | the one in `cpuinfo`.                                                            |
        +---------+--------------------------------+------------------------------------------------------------------------------------+

    2). Tbench 娴嬭瘯

        鍦ㄦ寚瀹氳皟鎺у櫒涓嬭繍琛?tbench 鍩哄噯娴嬭瘯鏃讹紝娴嬭瘯骞剁洃鎺?cpu 鐨勫彉鍖栥€?
        杩欎簺鍙樺寲鍖呮嫭鏈熸湜鎬ц兘銆侀鐜囥€佽礋杞姐€佹€ц兘銆佽兘鑰楃瓑銆?
        鎸囧畾鐨勮皟鎺у櫒鏄?ondemand 鎴?schedutil銆?
        Tbench 涔熷彲浠ュ湪 `acpi-cpufreq` 鍐呮牳椹卞姩涓婅繘琛屾祴璇曚互浣滄瘮杈冦€?

    3). Gitsource 娴嬭瘯

        鍦ㄦ寚瀹氳皟鎺у櫒涓嬭繍琛?gitsource 鍩哄噯娴嬭瘯鏃讹紝娴嬭瘯骞剁洃鎺?cpu 鐨勫彉鍖栥€?
        杩欎簺鍙樺寲鍖呮嫭鏈熸湜鎬ц兘銆侀鐜囥€佽礋杞姐€佹椂闂淬€佽兘鑰楃瓑銆?
        鎸囧畾鐨勮皟鎺у櫒鏄?ondemand 鎴?schedutil銆?
        Gitsource 涔熷彲浠ュ湪 `acpi-cpufreq` 鍐呮牳椹卞姩涓婅繘琛屾祴璇曚互浣滄瘮杈冦€?

#. 濡備綍鎵ц娴嬭瘯

   鎴戜滑浣跨敤 kselftest 妗嗘灦涓殑娴嬭瘯妯″潡鏉ュ疄鐜板畠銆?
   鎴戜滑鍒涘缓 `amd-pstate-ut` 妯″潡骞跺皢鍏剁粦瀹氬埌 kselftest銆傦紙璇︽儏鍙傝 Linux 鍐呮牳
   鑷祴璇?[^4^]_锛夈€?

    1). 鏋勫缓

        - 鎵撳紑 `CONFIG_X86_AMD_PSTATE` 閰嶇疆閫夐」銆?
        - 灏?`CONFIG_X86_AMD_PSTATE_UT` 閰嶇疆閫夐」璁剧疆涓?M銆?
        - 鏋勫缓宸ョ▼
```

            $ cd linux
            $ make -C tools/testing/selftests

        + make perf ::

            $ cd tools/perf/
            $ make


    2). Installation & Steps ::

        $ make -C tools/testing/selftests install INSTALL_PATH=~/kselftest
        $ cp tools/perf/perf /usr/bin/perf
        $ sudo ./kselftest/run_kselftest.sh -c amd-pstate

    3). Specified test case ::

        $ cd ~/kselftest/amd-pstate
        $ sudo ./run.sh -t basic
        $ sudo ./run.sh -t tbench
        $ sudo ./run.sh -t tbench -m acpi-cpufreq
        $ sudo ./run.sh -t gitsource
        $ sudo ./run.sh -t gitsource -m acpi-cpufreq
        $ ./run.sh --help
        ./run.sh: illegal option -- -
        Usage: ./run.sh [OPTION...]
                [-h <help>]
                [-o <output-file-for-dump>]
                [-c <all: All testing,
                     basic: Basic testing,
                     tbench: Tbench testing,
                     gitsource: Gitsource testing.>]
                [-t <tbench time limit>]
                [-p <tbench process number>]
                [-l <loop times for tbench>]
                [-i <amd tracer interval>]
                [-m <comparative test: acpi-cpufreq>]


    4). Results

        + basic

         When you finish test, you will get the following log info ::

          $ dmesg | grep "amd_pstate_ut" | tee log.txt
          [12977.570663] amd_pstate_ut: 1    amd_pstate_ut_acpi_cpc_valid  success!
          [12977.570673] amd_pstate_ut: 2    amd_pstate_ut_check_enabled   success!
          [12977.571207] amd_pstate_ut: 3    amd_pstate_ut_check_perf      success!
          [12977.571212] amd_pstate_ut: 4    amd_pstate_ut_check_freq      success!

        + tbench

         When you finish test, you will get selftest.tbench.csv and png images.
         The selftest.tbench.csv file contains the raw data and the drop of the comparative test.
         The png images shows the performance, energy and performan per watt of each test.
         Open selftest.tbench.csv :

         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + Governor                                        | Round        | Des-perf | Freq    | Load     | Performance | Energy  | Performance Per Watt |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + Unit                                            |              |          | GHz     |          | MB/s        | J       | MB/J                 |
         +=================================================+==============+==========+=========+==========+=============+=========+======================+
         + amd-pstate-ondemand                             | 1            |          |         |          | 2504.05     | 1563.67 | 158.5378             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 2            |          |         |          | 2243.64     | 1430.32 | 155.2941             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 3            |          |         |          | 2183.88     | 1401.32 | 154.2860             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | Average      |          |         |          | 2310.52     | 1465.1  | 156.1268             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 1            | 165.329  | 1.62257 | 99.798   | 2136.54     | 1395.26 | 151.5971             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 2            | 166      | 1.49761 | 99.9993  | 2100.56     | 1380.5  | 150.6377             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 3            | 166      | 1.47806 | 99.9993  | 2084.12     | 1375.76 | 149.9737             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | Average      | 165.776  | 1.53275 | 99.9322  | 2107.07     | 1383.84 | 150.7399             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 1            |          |         |          | 2529.9      | 1564.4  | 160.0997             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 2            |          |         |          | 2249.76     | 1432.97 | 155.4297             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 3            |          |         |          | 2181.46     | 1406.88 | 153.5060             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | Average      |          |         |          | 2320.37     | 1468.08 | 156.4741             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 1            |          |         |          | 2137.64     | 1385.24 | 152.7723             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 2            |          |         |          | 2107.05     | 1372.23 | 152.0138             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 3            |          |         |          | 2085.86     | 1365.35 | 151.2433             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | Average      |          |         |          | 2110.18     | 1374.27 | 152.0136             |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS acpi-cpufreq-schedutil | Comprison(%) |          |         |          | -9.0584     | -6.3899 | -2.8506              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand VS amd-pstate-schedutil     | Comprison(%) |          |         |          | 8.8053      | -5.5463 | -3.4503              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS amd-pstate-ondemand    | Comprison(%) |          |         |          | -0.4245     | -0.2029 | -0.2219              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil VS amd-pstate-schedutil  | Comprison(%) |          |         |          | -0.1473     | 0.6963  | -0.8378              |
         +-------------------------------------------------+--------------+----------+---------+----------+-------------+---------+----------------------+

        + gitsource

         When you finish test, you will get selftest.gitsource.csv and png images.
         The selftest.gitsource.csv file contains the raw data and the drop of the comparative test.
         The png images shows the performance, energy and performan per watt of each test.
         Open selftest.gitsource.csv :

         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + Governor                                        | Round        | Des-perf | Freq     | Load     | Time        | Energy  | Performance Per Watt |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + Unit                                            |              |          | GHz      |          | s           | J       | 1/J                  |
         +=================================================+==============+==========+==========+==========+=============+=========+======================+
         + amd-pstate-ondemand                             | 1            | 50.119   | 2.10509  | 23.3076  | 475.69      | 865.78  | 0.001155027          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 2            | 94.8006  | 1.98771  | 56.6533  | 467.1       | 839.67  | 0.001190944          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | 3            | 76.6091  | 2.53251  | 43.7791  | 467.69      | 855.85  | 0.001168429          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand                             | Average      | 73.8429  | 2.20844  | 41.2467  | 470.16      | 853.767 | 0.001171279          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 1            | 165.919  | 1.62319  | 98.3868  | 464.17      | 866.8   | 0.001153668          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 2            | 165.97   | 1.31309  | 99.5712  | 480.15      | 880.4   | 0.001135847          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | 3            | 165.973  | 1.28448  | 99.9252  | 481.79      | 867.02  | 0.001153375          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-schedutil                            | Average      | 165.954  | 1.40692  | 99.2944  | 475.37      | 871.407 | 0.001147569          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 1            |          |          |          | 2379.62     | 742.96  | 0.001345967          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 2            |          |          |          | 441.74      | 817.49  | 0.001223256          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | 3            |          |          |          | 455.48      | 820.01  | 0.001219497          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand                           | Average      |          |          |          | 425.613     | 793.487 | 0.001260260          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 1            |          |          |          | 459.69      | 838.54  | 0.001192548          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 2            |          |          |          | 466.55      | 830.89  | 0.001203528          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | 3            |          |          |          | 470.38      | 837.32  | 0.001194286          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil                          | Average      |          |          |          | 465.54      | 835.583 | 0.001196769          |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS acpi-cpufreq-schedutil | Comprison(%) |          |          |          | 9.3810      | 5.3051  | -5.0379              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + amd-pstate-ondemand VS amd-pstate-schedutil     | Comprison(%) | 124.7392 | -36.2934 | 140.7329 | 1.1081      | 2.0661  | -2.0242              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-ondemand VS amd-pstate-ondemand    | Comprison(%) |          |          |          | 10.4665     | 7.5968  | -7.0605              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+
         + acpi-cpufreq-schedutil VS amd-pstate-schedutil  | Comprison(%) |          |          |          | 2.1115      | 4.2873  | -4.1110              |
         +-------------------------------------------------+--------------+----------+----------+----------+-------------+---------+----------------------+

```
## Reference


       https://docs.amd.com/v/u/en-US/24593_3.44_APM_Vol2

       https://uefi.org/sites/default/files/resources/ACPI_Spec_6_4_Jan22.pdf

       https://docs.amd.com/v/u/en-US/56569-A1-PUB_3.03

       https://www.kernel.org/doc/html/latest/dev-tools/kselftest.html
