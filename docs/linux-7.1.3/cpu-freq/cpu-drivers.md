
## 濡備綍瀹炵幇涓€涓柊鐨?CPUFreq 澶勭悊鍣ㄩ┍鍔?

Authors:

 - Dominik Brodowski  <linux@brodo.de>
 - Rafael J. Wysocki <rafael.j.wysocki@intel.com>
 - Viresh Kumar <viresh.kumar@linaro.org>


   1. 璇ュ仛浠€涔堬紵
   1.1  鍒濆鍖?   1.2  姣?CPU 鍒濆鍖?   1.3  verify
   1.4  target/target_index 杩樻槸 setpolicy锛?   1.5  target/target_index
   1.6  setpolicy
   1.7  get_intermediate 涓?target_intermediate
   2. 棰戠巼琛ㄨ緟鍔╁伐鍏?


## 1. 璇ュ仛浠€涔堬紵


閭ｄ箞锛屼綘鍒氭嬁鍒颁竴鍧楀叏鏂扮殑 CPU / 鑺墖缁勪互鍙婂畠鐨勬暟鎹墜鍐岋紝骞舵兂涓鸿繖棰?CPU /
鑺墖缁勬坊鍔?cpufreq 鏀寔锛熷お濂戒簡銆備笅闈㈡槸涓€浜涘叧浜庡繀瑕佸伐浣滅殑鎻愮ず锛?

### 1.1 鍒濆鍖?

棣栧厛锛屽湪涓€涓?__initcall 绗?7 绾э紙module_init()锛夋垨鏇存櫄鐨勫嚱鏁颁腑锛屾鏌ュ綋鍓?鍐呮牳鏄惁杩愯鍦ㄦ纭殑 CPU 鍜屾纭殑鑺墖缁勪笂銆傚鏋滄槸锛屽垯浣跨敤
cpufreq_register_driver() 鍚?CPUfreq 鏍稿績娉ㄥ唽涓€涓?struct cpufreq_driver銆?
杩欎釜 struct cpufreq_driver 搴斿綋鍖呭惈浠€涔堬紵

 .name - 璇ラ┍鍔ㄧ殑鍚嶇О銆?
 .init - 鎸囧悜姣忕瓥鐣ワ紙per-policy锛夊垵濮嬪寲鍑芥暟鐨勬寚閽堛€?
 .verify - 鎸囧悜涓€涓€滈獙璇佲€濆嚱鏁扮殑鎸囬拡銆?
 .setpolicy _鎴朹 .fast_switch _鎴朹 .target _鎴朹 .target_index - 鍏充簬宸紓
 瑙佷笅鏂囥€?
浠ュ強鍙€夌殑

 .flags - 鎻愪緵缁?cpufreq 鏍稿績鐨勬彁绀恒€?
 .driver_data - cpufreq 椹卞姩鐗规湁鐨勬暟鎹€?
 .get_intermediate 鍜?target_intermediate - 鐢ㄤ簬鍦ㄦ敼鍙?CPU 棰戠巼鏃跺垏鎹㈠埌
 绋冲畾棰戠巼銆?
 .get - 杩斿洖 CPU 鐨勫綋鍓嶉鐜囥€?
 .bios_limit - 杩斿洖 CPU 鐨勭‖浠?BIOS 鏈€澶ч鐜囬檺鍒躲€?
 .exit - 鎸囧悜涓€涓瘡绛栫暐娓呯悊鍑芥暟鐨勬寚閽堬紝鍦?CPU 鐑彃鎷旇繃绋嬬殑 CPU_POST_DEAD
 闃舵琚皟鐢ㄣ€?
 .suspend - 鎸囧悜涓€涓瘡绛栫暐鎸傝捣鍑芥暟鐨勬寚閽堬紝鍦ㄥ叧闂腑鏂€佷笖璋冭妭鍣紙governor锛? 涓鸿绛栫暐鍋滄_涔嬪悗_琚皟鐢ㄣ€?
 .resume - 鎸囧悜涓€涓瘡绛栫暐鎭㈠鍑芥暟鐨勬寚閽堬紝鍦ㄥ叧闂腑鏂€佷笖璋冭妭鍣ㄤ负璇ョ瓥鐣? 閲嶆柊鍚姩_涔嬪墠_琚皟鐢ㄣ€?
 .ready - 鎸囧悜涓€涓瘡绛栫暐灏辩华鍑芥暟鐨勬寚閽堬紝鍦ㄧ瓥鐣ュ畬鍏ㄥ垵濮嬪寲涔嬪悗琚皟鐢ㄣ€?
 .attr - 鎸囧悜涓€涓?NULL 缁撳熬鐨?"struct freq_attr" 鍒楄〃鐨勬寚閽堬紝鐢ㄤ簬灏嗗€? 瀵煎嚭鍒?sysfs銆?
 .boost_enabled - 鑻ョ疆浣嶏紝鍒欏惎鐢?boost 棰戠巼銆?
 .set_boost - 鎸囧悜涓€涓瘡绛栫暐鍑芥暟鐨勬寚閽堬紝鐢ㄤ簬鍚敤/绂佺敤 boost 棰戠巼銆?

### 1.2 姣?CPU 鍒濆鍖?

姣忓綋涓€涓柊鐨?CPU 琚敞鍐屽埌璁惧妯″瀷锛屾垨鑰呭湪 cpufreq 椹卞姩娉ㄥ唽鑷韩涔嬪悗锛屽鏋?璇?CPU 杩樹笉瀛樺湪 cpufreq 绛栫暐锛屽氨浼氳皟鐢ㄦ瘡绛栫暐鍒濆鍖栧嚱鏁?cpufreq_driver.init銆?娉ㄦ剰锛?init() 鍜?.exit() 渚嬬▼鍙拡瀵圭瓥鐣ヨ璋冪敤涓€娆★紝鑰屼笉鏄拡瀵硅绛栫暐绠＄悊鐨?姣忎釜 CPU 璋冪敤銆傚畠鎺ュ彈涓€涓?``struct cpufreq_policy *policy`` 浣滀负鍙傛暟銆傜幇鍦ㄨ
鍋氫粈涔堬紵

濡傛灉鏈夊繀瑕侊紝鍦ㄤ綘鐨?CPU 涓婃縺娲?CPUfreq 鏀寔銆?
鎺ョ潃锛岄┍鍔ㄥ繀椤诲～鍏ヤ互涓嬪€硷細

+-----------------------------------+--------------------------------------+
|policy->cpuinfo.min_freq _浠ュ強_    |					   |
|policy->cpuinfo.max_freq	    | 璇?CPU 鏀寔鐨勬渶灏忓拰鏈€澶ч鐜?   |
|				    | 锛堝崟浣?kHz锛?		   |
+-----------------------------------+--------------------------------------+
|policy->cpuinfo.transition_latency | 璇?CPU 鍦ㄤ袱绉嶉鐜囦箣闂村垏鎹㈡墍闇€鐨?   |
|				    | 鏃堕棿锛屽崟浣嶇撼绉?		   |
+-----------------------------------+--------------------------------------+
|policy->cur			    | 璇?CPU 鐨勫綋鍓嶈繍琛岄鐜?	   |
|				    | 锛堝閫傜敤锛?		   |
+-----------------------------------+--------------------------------------+
|policy->min,			    |					   |
|policy->max,			    |					   |
|policy->policy 浠ュ強蹇呰鏃?    |					   |
|policy->governor		    | 蹇呴』鍖呭惈璇?CPU 鐨勨€滈粯璁ょ瓥鐣モ€濄€傜◢鍚?  |
|				    | cpufreq_driver.verify 浠ュ強浜岃€呬箣涓€  |
|				    | cpufreq_driver.setpolicy 鎴?   |
|				    | cpufreq_driver.target/target_index  |
|				    | 浼氫互杩欎簺鍊艰璋冪敤銆?	   |
+-----------------------------------+--------------------------------------+
|policy->cpus			    | 鐢紙鍦ㄧ嚎 + 绂荤嚎锛塁PU 鐨勬帺鐮佹洿鏂板畠锛寍
|				    | 杩欎簺 CPU 涓庤 CPU 涓€璧疯繘琛?DVFS	   |
|				    | 锛堝嵆涓庡畠鍦ㄥ悓涓€鏃堕挓/鐢靛帇杞ㄤ笂锛夈€?   |
+-----------------------------------+--------------------------------------+

瀵逛簬璁剧疆鍏朵腑鏌愪簺鍊硷紙cpuinfo.min[max]_freq銆乸olicy->min[max]锛夛紝棰戠巼琛ㄨ緟鍔╁伐鍏?鍙兘浼氭湁甯姪銆傚叧浜庡畠浠殑鏇村淇℃伅锛岃鍙傞槄绗?2 鑺傘€?

### 1.3 verify


褰撶敤鎴峰喅瀹氳缃竴涓柊鐨勭瓥鐣ワ紙鐢?"policy銆乬overnor銆乵in銆乵ax" 缁勬垚锛夋椂锛屽繀椤?瀵硅繖涓瓥鐣ヨ繘琛屾牎楠岋紝浠ヤ究鎶婁笉鍏煎鐨勫€肩籂姝ｈ繃鏉ャ€傜敤浜庢牎楠岃繖浜涘€硷紝鍑芥暟
cpufreq_verify_within_limits(`struct cpufreq_policy *policy`,
`unsigned int min_freq`, `unsigned int max_freq`) 鍙兘浼氭湁甯姪銆傚叧浜庨鐜囪〃
杈呭姪宸ュ叿鐨勭粏鑺傦紝璇峰弬闃呯 2 鑺傘€?
浣犻渶瑕佺‘淇濊嚦灏戞湁涓€涓湁鏁堢殑棰戠巼锛堟垨宸ヤ綔鑼冨洿锛夎惤鍦?policy->min 鍜?policy->max
涔嬮棿銆傚鏈夊繀瑕侊紝鍏堝澶?policy->max锛屽彧鏈夊湪杩欎篃鏃犳硶瑙ｅ喅鏃讹紝鎵嶉檷浣?policy->min銆?

### 1.4 target 杩樻槸 target_index 杩樻槸 setpolicy 杩樻槸 fast_switch锛?

澶у鏁?cpufreq 椹卞姩锛岀敋鑷冲ぇ澶氭暟 CPU 棰戠巼璋冭妭绠楁硶锛屽彧鍏佽灏?CPU 棰戠巼璁剧疆涓?棰勫畾涔夌殑鍥哄畾鍊笺€傚浜庤繖浜涳紝浣犱娇鐢?->target()銆?>target_index() 鎴?->fast_switch() 鍥炶皟銆?
涓€浜涙敮鎸?cpufreq 鐨勫鐞嗗櫒浼氬湪鏌愪簺闄愬埗涔嬮棿鑷鍒囨崲棰戠巼銆傝繖浜涘簲褰撲娇鐢?->setpolicy() 鍥炶皟銆?

### 1.5. target/target_index


target_index 璋冪敤鏈変袱涓弬鏁帮細`struct cpufreq_policy *policy` 鍜?`unsigned
int` index锛堢储寮曞埌鎵€鏆撮湶鐨勯鐜囪〃涓級銆?
CPUfreq 椹卞姩蹇呴』鍦ㄨ繖閲岃璋冪敤鏃惰缃柊鐨勯鐜囥€傚疄闄呴鐜囧繀椤荤敱
freq_table[index].frequency 纭畾銆?
鍗充娇鍦ㄤ箣鍓嶅垏鎹㈠埌浜嗕腑闂撮鐜囷紝涔熷簲褰撳湪鍑洪敊鏃舵仮澶嶅埌鏇存棭鐨勯鐜囷紙鍗?policy->restore_freq锛夈€?
### 宸插簾寮?

target 璋冪敤鏈変笁涓弬鏁帮細`struct cpufreq_policy *policy`銆乽nsigned int
target_frequency銆乽nsigned int relation銆?
CPUfreq 椹卞姩蹇呴』鍦ㄨ繖閲岃璋冪敤鏃惰缃柊鐨勯鐜囥€傚疄闄呴鐜囧繀椤讳緷鎹互涓嬭鍒欑‘瀹氾細

- 灏介噺鎺ヨ繎 "target_freq"
- policy->min <= new_freq <= policy->max锛堣繖蹇呴』鎴愮珛锛侊紒锛侊級
- 鑻?relation==CPUFREQ_REL_L锛屽皾璇曢€夋嫨涓€涓ぇ浜庣瓑浜?target_freq 鐨?new_freq銆?  锛堚€淟 琛ㄧず lowest锛屼絾涓嶄綆浜庘€濓級
- 鑻?relation==CPUFREQ_REL_H锛屽皾璇曢€夋嫨涓€涓皬浜庣瓑浜?target_freq 鐨?new_freq銆?  锛堚€淗 琛ㄧず highest锛屼絾涓嶉珮浜庘€濓級

杩欓噷棰戠巼琛ㄨ緟鍔╁伐鍏峰悓鏍峰彲浠ュ府鍒颁綘 鈥斺€?璇︽儏瑙佺 2 鑺傘€?

### 1.6. fast_switch


杩欎釜鍑芥暟鐢ㄤ簬浠庤皟搴﹀櫒涓婁笅鏂囦腑杩涜棰戠巼鍒囨崲銆傚苟闈炴墍鏈夐┍鍔ㄩ兘瑕佹眰瀹炵幇瀹冿紝鍥犱负
鍦ㄨ繖涓洖璋冨唴閮ㄤ笉鍏佽鐫＄湢銆傝繖涓洖璋冨繀椤昏楂樺害浼樺寲锛屼互灏藉揩瀹屾垚鍒囨崲銆?
杩欎釜鍑芥暟鏈変袱涓弬鏁帮細`struct cpufreq_policy *policy` 鍜?`unsigned int
target_frequency`銆?

### 1.7 setpolicy


setpolicy 璋冪敤鍙帴鍙椾竴涓?`struct cpufreq_policy *policy` 浣滀负鍙傛暟銆備綘闇€瑕佹妸
澶勭悊鍣ㄥ唴鎴栬姱鐗囩粍鍐呭姩鎬侀鐜囧垏鎹㈢殑涓嬮檺璁句负 policy->min锛屼笂闄愯涓?policy->max锛?骞朵笖鈥斺€斿鏋滄敮鎸佺殑璇濃€斺€斿湪 policy->policy 涓?CPUFREQ_POLICY_PERFORMANCE 鏃堕€夋嫨
闈㈠悜鎬ц兘鐨勮缃紝鍦?CPUFREQ_POLICY_POWERSAVE 鏃堕€夋嫨闈㈠悜鑺傝兘鐨勮缃€傚悓鏃惰鍙傝€?drivers/cpufreq/longrun.c 涓殑鍙傝€冨疄鐜般€?

### 1.8 get_intermediate 涓?target_intermediate


浠呴€傜敤浜庢湭璁剧疆 target_index() 鍜?CPUFREQ_ASYNC_NOTIFICATION 鐨勯┍鍔ㄣ€?
get_intermediate 搴斿綋杩斿洖涓€涓钩鍙版兂鍒囨崲鍒扮殑绋冲畾涓棿棰戠巼锛岃€?target_intermediate()
搴斿綋鍦ㄨ烦杞埌涓?'index' 瀵瑰簲鐨勯鐜囦箣鍓嶏紝鎶?CPU 璁剧疆鍒伴偅涓鐜囥€傛牳蹇冧細璐熻矗鍙戦€?閫氱煡锛岄┍鍔ㄤ笉蹇呭湪 target_intermediate() 鎴?target_index() 涓鐞嗗畠浠€?
濡傛灉椹卞姩涓嶅笇鏈涗负鏌愪釜鐩爣棰戠巼鍒囨崲鍒颁腑闂撮鐜囷紝鍙互浠?get_intermediate() 杩斿洖
'0'銆傝繖绉嶆儏鍐典笅锛屾牳蹇冧細鐩存帴璋冪敤 ->target_index()銆?
娉ㄦ剰锛?>target_index() 鍦ㄥけ璐ユ椂搴斿綋鎭㈠鍒?policy->restore_freq锛屽洜涓烘牳蹇冧細
涓哄畠鍙戦€侀€氱煡銆?

## 2. 棰戠巼琛ㄨ緟鍔╁伐鍏?

鐢变簬澶у鏁?cpufreq 澶勭悊鍣ㄥ彧鍏佽琚缃负灏戞暟鍑犱釜鐗瑰畾棰戠巼锛屽甫鏈変竴浜涘嚱鏁扮殑
鈥滈鐜囪〃鈥濆彲浠ュ湪澶勭悊鍣ㄩ┍鍔ㄧ殑鏌愪簺宸ヤ綔涓彁渚涘府鍔┿€傝繖鏍蜂竴涓€滈鐜囪〃鈥濈敱涓€涓?struct cpufreq_frequency_table 鏉＄洰鏁扮粍缁勬垚锛屽叾涓湪 "driver_data" 涓繚瀛橀┍鍔?鐗瑰畾鐨勫€硷紝鍦?"frequency" 涓繚瀛樺搴旂殑棰戠巼锛屽苟璁剧疆 flags銆傚湪琛ㄧ殑鏈熬锛屼綘闇€瑕?娣诲姞涓€涓?frequency 璁句负 CPUFREQ_TABLE_END 鐨?cpufreq_frequency_table 鏉＄洰銆傝€?濡傛灉浣犳兂璺宠繃琛ㄤ腑鐨勬煇涓潯鐩紝灏辨妸棰戠巼璁句负 CPUFREQ_ENTRY_INVALID銆傛潯鐩笉闇€瑕佹寜
浠讳綍鐗瑰畾椤哄簭鎺掑垪锛屼絾濡傛灉鎺掍簡搴忥紝cpufreq 鏍稿績瀵瑰畠浠仛 DVFS 浼氬揩涓€浜涳紝鍥犱负鏌ユ壘
鏈€浣冲尮閰嶆洿蹇€?
濡傛灉绛栫暐鍦ㄥ叾 policy->freq_table 瀛楁涓寘鍚湁鏁堟寚閽堬紝cpufreq 琛ㄤ細鐢辨牳蹇冭嚜鍔?鏍￠獙銆?
cpufreq_frequency_table_verify() 纭繚鑷冲皯鏈変竴涓湁鏁堥鐜囪惤鍦?policy->min 鍜?policy->max 涔嬮棿锛屽苟涓旀弧瓒虫墍鏈夊叾浠栨爣鍑嗐€傝繖瀵?->verify 璋冪敤寰堟湁甯姪銆?
cpufreq_frequency_table_target() 鏄搴斾簬 ->target 闃舵鐨勯鐜囪〃杈呭姪宸ュ叿銆?鍙渶鎶婂€间紶閫掔粰杩欎釜鍑芥暟锛屽畠灏变細杩斿洖鍖呭惈 CPU 搴旇璁剧疆鍒扮殑棰戠巼鐨勯鐜囪〃鏉＄洰銆?
浠ヤ笅瀹忓彲鐢ㄤ綔閬嶅巻 cpufreq_frequency_table 鐨勮凯浠ｅ櫒锛?
cpufreq_for_each_entry(pos, table) - 閬嶅巻棰戠巼琛ㄧ殑鎵€鏈夋潯鐩€?
cpufreq_for_each_valid_entry(pos, table) - 閬嶅巻鎵€鏈夋潯鐩紝浣嗘帓闄?CPUFREQ_ENTRY_INVALID 棰戠巼銆?浣跨敤鍙傛暟 "pos" 鈥斺€?浣滀负寰幆娓告爣鐨?`cpufreq_frequency_table *`锛屼互鍙?"table" 鈥斺€?浣犳兂瑕侀亶鍘嗙殑 `cpufreq_frequency_table *`銆?
```
	struct cpufreq_frequency_table *pos, *driver_freq_table;

	cpufreq_for_each_entry(pos, driver_freq_table) {
		/* Do something with pos */
		pos->frequency = ...
	}
```
濡傛灉浣犻渶瑕佷娇鐢?pos 鍦?driver_freq_table 涓殑浣嶇疆锛屼笉瑕佸鎸囬拡鍋氱浉鍑忥紝鍥犱负杩?鐩稿綋鑰楄垂璧勬簮銆傜浉鍙嶏紝璇蜂娇鐢ㄥ畯 cpufreq_for_each_entry_idx() 鍜?cpufreq_for_each_valid_entry_idx()銆?