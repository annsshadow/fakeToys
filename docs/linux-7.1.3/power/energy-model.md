
## 璁惧鐨勮兘閲忔ā鍨?

### 1. 姒傝堪


鑳介噺妯″瀷锛圗nergy Model锛孍M锛夋鏋跺厖褰撹繖鏍蜂竴绉嶆帴鍙ｏ細涓€渚ф槸浜嗚В璁惧鍦ㄥ悇绉嶆€ц兘绛夌骇涓?鍔熻€楃殑椹卞姩锛屽彟涓€渚ф槸甯屾湜鍒╃敤杩欎簺淇℃伅鍋氬嚭鑳介噺鎰熺煡鍐崇瓥鐨勫収鏍稿瓙绯荤粺銆?
鍏充簬璁惧鍔熻€楃殑淇℃伅鏉ユ簮鍦ㄤ笉鍚屽钩鍙颁箣闂村樊寮傚緢澶с€傚湪鏌愪簺鎯呭喌涓嬶紝杩欎簺鍔熻€楀紑閿€鍙互浣跨敤
devicetree 鏁版嵁杩涜浼扮畻銆傚湪鍏朵粬鎯呭喌涓嬶紝鍥轰欢浼氭洿娓呮銆傛垨鑰咃紝鐢ㄦ埛绌洪棿鍙兘澶勪簬鏈€鏈夊埄鐨?浣嶇疆銆備负浜嗛伩鍏嶆瘡涓鎴风瀛愮郴缁熷悇鑷噸鏂板疄鐜板姣忎竴绉嶅彲鑳戒俊鎭簮鐨勬敮鎸侊紝EM 妗嗘灦浣滀负涓€涓?鎶借薄灞備粙鍏ワ紝瀹冩爣鍑嗗寲浜嗗唴鏍镐腑鍔熻€楄〃鐨勬牸寮忥紝浠庤€岄伩鍏嶉噸澶嶅伐浣溿€?
鍔熻€楀€煎彲浠ョ敤寰摝琛ㄧず锛屼篃鍙互鐢ㄢ€滄娊璞″埢搴︹€濊〃绀恒€傚涓瓙绯荤粺鍙兘浣跨敤 EM锛岀敱绯荤粺闆嗘垚鍛?璐熻矗妫€鏌ュ姛鑰楀€煎埢搴︾被鍨嬬殑鍚勯」瑕佹眰鏄惁寰楀埌婊¤冻銆備竴涓緥瀛愬彲浠ュ湪鑳介噺鎰熺煡璋冨害鍣?锛圗nergy-Aware Scheduler锛夋枃妗?Documentation/scheduler/sched-energy.rst 涓壘鍒般€傚浜?thermal 鎴?powercap 绛夋煇浜涘瓙绯荤粺锛屼互鈥滄娊璞″埢搴︹€濊〃绀虹殑鍔熻€楀€煎彲鑳戒細寮曞彂闂銆傝繖浜涘瓙绯荤粺
鏇村叧娉ㄥ杩囧幓鎵€鐢ㄥ姛鑰楃殑浼扮畻锛屽洜姝ゅ彲鑳介渶瑕佺湡瀹炵殑寰摝鍊笺€傝繖浜涜姹傜殑涓€涓緥瀛愬彲浠ュ湪
Documentation/driver-api/thermal/power_allocator.rst 涓殑鏅鸿兘鍔熻€楀垎閰嶏紙Intelligent Power
Allocation锛夐儴鍒嗘壘鍒般€傚唴鏍稿瓙绯荤粺鍙兘瀹炵幇鑷姩妫€娴嬶紝浠ユ鏌ュ凡娉ㄥ唽鍒?EM 鐨勮澶囨槸鍚﹀叿鏈?涓嶄竴鑷寸殑鍒诲害锛堝熀浜?EM 鍐呴儴鏍囧織锛夈€傞渶瑕佺墷璁扮殑涓€鐐规槸锛屽綋鍔熻€楀€间互鈥滄娊璞″埢搴︹€濊〃绀烘椂锛屾棤娉?鎺ㄥ鍑轰互寰劍鑰充负鍗曚綅鐨勭湡瀹炶兘閲忋€?
涓嬪浘灞曠ず浜嗕竴涓┍鍔紙姝ゅ涓?Arm 涓撶敤锛屼絾璇ユ柟娉曢€傜敤浜庝换浣曟灦鏋勶級鍚?EM 鎻愪緵鍔熻€楀紑閿€鐨?绀轰緥锛?
```
       +---------------+  +-----------------+  +---------------+
       | Thermal (IPA) |  | Scheduler (EAS) |  |     Other     |
       +---------------+  +-----------------+  +---------------+
               |                   | em_cpu_energy()   |
               |                   | em_cpu_get()      |
               +---------+         |         +---------+
                         |         |         |
                         v         v         v
                        +---------------------+
                        |    Energy Model     |
                        |     Framework       |
                        +---------------------+
                           ^       ^       ^
                           |       |       | em_dev_register_perf_domain()
                +----------+       |       +---------+
                |                  |                 |
        +---------------+  +---------------+  +--------------+
        |  cpufreq-dt   |  |   arm_scmi    |  |    Other     |
        +---------------+  +---------------+  +--------------+
                ^                  ^                 ^
                |                  |                 |
        +--------------+   +---------------+  +--------------+
        | Device Tree  |   |   Firmware    |  |      ?       |
        +--------------+   +---------------+  +--------------+

```
瀵逛簬 CPU 璁惧锛孍M 妗嗘灦绠＄悊绯荤粺涓殑姣忎釜鈥滄€ц兘鍩熲€濓紙performance domain锛夌殑鍔熻€楄〃銆傛€ц兘鍩?鏄竴缁勬€ц兘琚竴璧风缉鏀剧殑 CPU銆傛€ц兘鍩熼€氬父涓?CPUFreq 绛栫暐鍏锋湁涓€涓€鏄犲皠鍏崇郴銆傛€ц兘鍩熶腑鐨勬墍鏈?CPU 蹇呴』鍏锋湁鐩稿悓鐨勫井鏋舵瀯銆備笉鍚屾€ц兘鍩熶腑鐨?CPU 鍙互鍏锋湁涓嶅悓鐨勫井鏋舵瀯銆?
涓轰簡鏇村ソ鍦板弽鏄犵敱浜庨潤鎬佸姛鑰楋紙娉勬紡锛夊紩璧风殑鍔熻€楀彉鍖栵紝EM 鏀寔鍦ㄨ繍琛屾椂淇敼鍔熻€楀€笺€傝鏈哄埗
渚濊禆 RCU 鏉ラ噴鏀惧彲淇敼鐨?EM perf_state 琛ㄥ唴瀛樸€傚叾鐢ㄦ埛鈥斺€斾换鍔¤皟搴﹀櫒鈥斺€斾篃浣跨敤 RCU 鏉ヨ闂?璇ュ唴瀛樸€侲M 妗嗘灦鎻愪緵鐢ㄤ簬鍒嗛厤/閲婃斁鍙慨鏀?EM 琛ㄦ柊鍐呭瓨鐨?API銆傚綋缁欏畾鐨?EM 杩愯鏃惰〃瀹炰緥涓嶅啀
鏈夋嫢鏈夎€呮椂锛屾棫鍐呭瓨浼氶€氳繃 RCU 鍥炶皟鏈哄埗鑷姩閲婃斁銆傝繖閫氳繃 kref 鏈哄埗杩涜璺熻釜銆傚湪杩愯鏃舵彁渚?鏂?EM 鐨勮澶囬┍鍔ㄥ簲鍦ㄤ笉鍐嶉渶瑕佹椂璋冪敤 EM API 瀹夊叏鍦伴噴鏀惧畠銆侲M 妗嗘灦浼氬湪鍙兘鏃惰礋璐ｆ竻鐞嗗伐浣溿€?
甯屾湜淇敼 EM 鍊肩殑鍐呮牳浠ｇ爜鍙椾簰鏂ヤ綋淇濇姢锛屼互鍏嶅苟鍙戣闂€傚洜姝わ紝璁惧椹卞姩浠ｇ爜鍦ㄥ皾璇曚慨鏀?EM 鏃?蹇呴』鍦ㄥ彲鐫＄湢涓婁笅鏂囷紙sleeping context锛変腑杩愯銆?
鍊熷姪杩愯鏃跺彲淇敼鐨?EM锛屾垜浠皢璁捐浠庘€滃湪鏁翠釜杩愯鏈熼棿鍗曚竴涓旈潤鎬佺殑 EM鈥濓紙绯荤粺灞炴€э級杞彉涓?鈥滃彲鍦ㄨ繍琛屾湡闂存牴鎹緥濡傚伐浣滆礋杞借€屾敼鍙樼殑鍗曚竴 EM鈥濓紙绯荤粺涓庡伐浣滆礋杞藉睘鎬э級銆?
杩樺彲浠ヤ慨鏀规瘡涓?EM 鎬ц兘鐘舵€佺殑 CPU 鎬ц兘鍊笺€傚洜姝わ紝瀹屾暣鐨勫姛鑰椾笌鎬ц兘鏇茬嚎锛堝憟鎸囨暟鏇茬嚎锛夊彲浠?鏍规嵁渚嬪宸ヤ綔璐熻浇鎴栫郴缁熷睘鎬ц€屾敼鍙樸€?

### 2. 鏍稿績 API


##### 2.1 閰嶇疆閫夐」


蹇呴』浣跨敤 CONFIG_ENERGY_MODEL 鎵嶈兘浣跨敤 EM 妗嗘灦銆?

##### 2.2 鎬ц兘鍩熺殑娉ㄥ唽


#### 'advanced' EM 鐨勬敞鍐?

鈥渁dvanced鈥?EM 涔嬫墍浠ュ緱鍚嶏紝鏄洜涓哄厑璁搁┍鍔ㄦ彁渚涙洿绮剧‘鐨勫姛鑰楁ā鍨嬨€傚畠涓嶅眬闄愪簬妗嗘灦涓疄鐜扮殑
鏌愪簺鏁板鍏紡锛堝鍚屸€渟imple鈥?EM 鐨勬儏鍐碉級銆傚畠鍙互鏇村ソ鍦板弽鏄犱负姣忎釜鎬ц兘鐘舵€佹墽琛岀殑鐪熷疄鍔熻€?娴嬮噺銆傚洜姝わ紝鍦ㄨ€冭檻 EM 闈欐€佸姛鑰楋紙娉勬紡锛夊緢閲嶈鐨勬儏鍐典笅锛屽簲浼樺厛浣跨敤杩欑娉ㄥ唽鏂规硶銆?
椹卞姩搴旈€氳繃浠ヤ笅鏂瑰紡灏嗘€ц兘鍩熸敞鍐屽埌 EM 妗嗘灦锛?
```

  int em_dev_register_perf_domain(struct device *dev, unsigned int nr_states,
		struct em_data_callback *cb, cpumask_t *cpus, bool microwatts);

```
椹卞姩蹇呴』鎻愪緵涓€涓洖璋冨嚱鏁帮紝涓烘瘡涓€ц兘鐘舵€佽繑鍥?<棰戠巼, 鍔熻€? 鍏冪粍銆傞┍鍔ㄦ彁渚涚殑鍥炶皟鍑芥暟鍙?鑷敱鍦颁粠浠讳綍鐩稿叧浣嶇疆锛圖T銆佸浐浠垛€︹€︼級骞朵互浠讳綍蹇呰鐨勬柟寮忚幏鍙栨暟鎹€備粎瀵逛簬 CPU 璁惧锛岄┍鍔?蹇呴』浣跨敤 cpumask 鎸囧畾鎬ц兘鍩熺殑 CPU銆傚浜庨潪 CPU 鐨勫叾浠栬澶囷紝鏈€鍚庝竴涓弬鏁板繀椤昏涓?NULL銆?鏈€鍚庝竴涓弬鏁?'microwatts' 蹇呴』浠ユ纭殑鍊艰缃紝杩欎竴鐐瑰緢閲嶈銆備娇鐢?EM 鐨勫唴鏍稿瓙绯荤粺鍙兘
渚濊禆姝ゆ爣蹇楁潵妫€鏌ユ墍鏈?EM 璁惧鏄惁浣跨敤鐩稿悓鐨勫埢搴︺€傚鏋滃瓨鍦ㄤ笉鍚岀殑鍒诲害锛岃繖浜涘瓙绯荤粺鍙兘浼?杩斿洖璀﹀憡/閿欒銆佸仠姝㈠伐浣滅敋鑷?panic銆傛湁鍏冲疄鐜版鍥炶皟鐨勯┍鍔ㄧず渚嬶紝璇峰弬瑙佺 3 鑺傦紱鏈夊叧姝?API
鐨勬洿澶氭枃妗ｏ紝璇峰弬瑙佺 2.4 鑺傘€?
#### 浣跨敤 DT 娉ㄥ唽 EM


EM 涔熷彲浠ヤ娇鐢?OPP 妗嗘灦浠ュ強 DT 涓殑 "operating-points-v2" 淇℃伅鏉ユ敞鍐屻€侱T 涓殑姣忎釜 OPP
鏉＄洰閮藉彲浠ョ敤鍖呭惈寰摝鍔熻€楀€肩殑灞炴€?"opp-microwatt" 杩涜鎵╁睍銆傝繖涓?OPP DT 灞炴€у厑璁稿钩鍙版敞鍐?鍙嶆槧鎬诲姛鑰楋紙闈欐€?+ 鍔ㄦ€侊級鐨?EM 鍔熻€楀€笺€傝繖浜涘姛鑰楀€煎彲鑳界洿鎺ユ潵鑷疄楠屽拰娴嬮噺銆?
#### 'artificial' EM 鐨勬敞鍐?

瀵逛簬缂哄皯姣忎釜鎬ц兘鐘舵€佸姛鑰楀€艰缁嗕俊鎭殑椹卞姩锛屽彲浠ラ€夋嫨鎻愪緵涓€涓嚜瀹氫箟鍥炶皟銆傚洖璋?.get_cost()
鏄彲閫夌殑锛屾彁渚?EAS 浣跨敤鐨勨€渃ost鈥濆€笺€傝繖瀵逛簬浠呮彁渚?CPU 绫诲瀷涔嬮棿鐩稿鏁堢巼淇℃伅鐨勫钩鍙板緢鏈夌敤锛?鍒╃敤杩欎簺淇℃伅鍙互鍒涘缓鎶借薄鍔熻€楁ā鍨嬨€備絾鍗充娇鎶借薄鍔熻€楁ā鍨嬶紝鑰冭檻鍒拌緭鍏ュ姛鑰楀€肩殑澶у皬闄愬埗锛屾湁鏃?涔熼毦浠ラ€傞厤銆?get_cost() 鍏佽鎻愪緵鍙嶆槧 CPU 鏁堢巼鐨勨€渃ost鈥濆€笺€傝繖鏍峰彲浠ユ彁渚涗笌 EM 鍐呴儴璁＄畻
鈥渃ost鈥濆€肩殑鍏紡鎵€寮哄埗鐨勫叧绯讳笉鍚岀殑 EAS 淇℃伅銆傝涓鸿繖鏍风殑骞冲彴娉ㄥ唽 EM锛岄┍鍔ㄥ繀椤诲皢鏍囧織
'microwatts' 璁句负 0锛屾彁渚?.get_power() 鍥炶皟骞舵彁渚?.get_cost() 鍥炶皟銆侲M 妗嗘灦浼氬湪娉ㄥ唽
鏈熼棿姝ｇ‘澶勭悊姝ょ被骞冲彴銆傚姝ょ被骞冲彴浼氳缃?EM_PERF_DOMAIN_ARTIFICIAL 鏍囧織銆備娇鐢?EM 鐨勫叾浠?妗嗘灦搴旀牸澶栨敞鎰忥紝姝ｇ‘娴嬭瘯鍜屽鐞嗘鏍囧織銆?
#### 'simple' EM 鐨勬敞鍐?

鈥渟imple鈥?EM 浣跨敤妗嗘灦杈呭姪鍑芥暟 cpufreq_register_em_with_opp() 娉ㄥ唽銆傚畠瀹炵幇鐨勫姛鑰楁ā鍨嬩笌
浠ヤ笅寮忓瓙鐩稿叧锛?
```

	Power = C * V^2 * f

```
浣跨敤姝ゆ柟娉曟敞鍐岀殑 EM 鍙兘鏃犳硶姝ｇ‘鍙嶆槧鐪熷疄璁惧鐨勭墿鐞嗙壒鎬э紝渚嬪褰撻潤鎬佸姛鑰楋紙娉勬紡锛夊緢閲嶈鏃躲€?

##### 2.3 璁块棶鎬ц兘鍩?

鏈変袱涓?API 鍑芥暟鎻愪緵瀵硅兘閲忔ā鍨嬬殑璁块棶锛歟m_cpu_get() 浠?CPU id 浣滀负鍙傛暟锛宔m_pd_get() 浠?璁惧鎸囬拡浣滀负鍙傛暟銆備娇鐢ㄥ摢涓帴鍙ｅ彇鍐充簬瀛愮郴缁燂紝浣嗗浜?CPU 璁惧锛岃繖涓や釜鍑芥暟杩斿洖鐩稿悓鐨勬€ц兘
鍩熴€?
瀵?CPU 鑳介噺妯″瀷鎰熷叴瓒ｇ殑瀛愮郴缁熷彲浠ヤ娇鐢?em_cpu_get() API 鑾峰彇瀹冦€傝兘閲忔ā鍨嬭〃鍦ㄦ€ц兘鍩熷垱寤烘椂
鍒嗛厤涓€娆★紝骞跺師鏍蜂繚鐣欏湪鍐呭瓨涓€?
鎬ц兘鍩熸秷鑰楃殑鑳借€楀彲浠ヤ娇鐢?em_cpu_energy() API 浼扮畻銆傝浼扮畻鍋囪鍦?CPU 璁惧鐨勬儏鍐典笅浣跨敤
schedutil CPUfreq 璋冨害鍣ㄣ€傜洰鍓嶆湭閽堝鍏朵粬绫诲瀷鐨勮澶囨彁渚涙璁＄畻銆?
鏈夊叧涓婅堪 API 鐨勬洿澶氳鎯呭彲鍦?`<linux/energy_model.h>` 鎴栫 2.5 鑺備腑鎵惧埌銆?

##### 2.4 杩愯鏃朵慨鏀?

甯屾湜鍦ㄨ繍琛屾椂鏇存柊 EM 鐨勯┍鍔ㄥ簲浣跨敤浠ヤ笅涓撶敤鍑芥暟鏉ュ垎閰嶅凡淇敼 EM 鐨勬柊瀹炰緥銆傝 API 濡備笅锛?
```

  struct em_perf_table __rcu *em_table_alloc(struct em_perf_domain *pd);

```
杩欏厑璁稿垎閰嶄竴涓粨鏋勶紝鍏朵腑鍖呭惈鏂扮殑 EM 琛紝浠ュ強 EM 妗嗘灦鎵€闇€鐨?RCU 鍜?kref銆?struct
em_perf_table' 鍖呭惈鏁扮粍 'struct em_perf_state state[]'锛屽嵆鎸夊崌搴忔帓鍒楃殑鎬ц兘鐘舵€佸垪琛ㄣ€傝
鍒楄〃蹇呴』鐢卞笇鏈涙洿鏂?EM 鐨勮澶囬┍鍔ㄥ～鍏呫€傞鐜囧垪琛ㄥ彲浠ヤ粠鐜版湁鐨?EM锛堝湪鍚姩鏃跺垱寤猴級鑾峰彇銆?'struct em_perf_state' 涓殑鍐呭涔熷繀椤荤敱椹卞姩濉厖銆?
```

  int em_dev_update_perf_domain(struct device *dev,
			struct em_perf_table __rcu *new_table);

```
椹卞姩蹇呴』鎻愪緵鎸囧悜宸插垎閰嶅苟鍒濆鍖栫殑鏂?EM 'struct em_perf_table' 鐨勬寚閽堛€傝鏂?EM 灏嗗湪 EM
妗嗘灦鍐呰瀹夊叏浣跨敤锛屽苟瀵瑰唴鏍镐腑鐨勫叾浠栧瓙绯荤粺锛坱hermal銆乸owercap锛夊彲瑙併€傛 API 鐨勪富瑕佽璁?鐩爣鏄揩閫燂紝骞堕伩鍏嶅湪杩愯鏃惰繘琛岄澶栫殑璁＄畻鎴栧唴瀛樺垎閰嶃€傚綋璁惧椹卞姩涓凡鏈夐璁＄畻鐨?EM 鏃讹紝
搴斿綋鍙互绠€鍗曞湴澶嶇敤瀹冧滑锛屼笖鎬ц兘寮€閿€寰堜綆銆?
涓轰簡閲婃斁椹卞姩鍏堝墠鎻愪緵鐨?EM锛堜緥濡傚綋妯″潡

```

  void em_table_free(struct em_perf_table __rcu *table);

```
褰撴病鏈夊叾浠栧瓙绯荤粺锛堜緥濡?EAS锛変娇鐢ㄥ畠鏃讹紝杩欏皢鍏佽 EM 妗嗘灦瀹夊叏鍦扮Щ闄よ鍐呭瓨銆?
瑕佸湪鍏朵粬瀛愮郴缁燂紙濡?thermal銆乸owercap锛変腑浣跨敤鍔熻€楀€硷紝闇€瑕佽皟鐢ㄨ兘澶熶繚鎶よ鍙栬€呭苟淇濊瘉 EM
涓€鑷存€х殑 API锛?
```

  struct em_perf_state *em_perf_state_from_pd(struct em_perf_domain *pd);

```
瀹冭繑鍥?'struct em_perf_state' 鎸囬拡锛屽嵆鎸夊崌搴忔帓鍒楃殑鎬ц兘鐘舵€佹暟缁勩€傛鍑芥暟蹇呴』鍦?RCU 璇婚攣
鍖洪棿锛坮cu_read_lock() 涔嬪悗锛夎皟鐢ㄣ€傚綋涓嶅啀闇€瑕?EM 琛ㄦ椂锛岄渶瑕佽皟鐢?rcu_read_unlock()銆傝繖鏍?EM 鍙互瀹夊叏鍦颁娇鐢?RCU 璇诲尯闂村苟淇濇姢鐢ㄦ埛銆傚畠涔熷厑璁?EM 妗嗘灦绠＄悊鍐呭瓨骞堕噴鏀惧畠銆傛湁鍏冲浣曚娇鐢?瀹冪殑鏇村璇︽儏锛岃鍙傝绗?3.2 鑺備腑鐨勭ず渚嬮┍鍔ㄣ€?
**鎻愪緵浜嗕笓鐢?API 渚涜澶囬┍鍔ㄨ绠?em_perf_state** : cost

```

  int em_dev_compute_costs(struct device *dev, struct em_perf_state *table,
                           int nr_states);

```
EM 涓殑杩欎簺鈥渃ost鈥濆€肩敤浜?EAS銆傛柊鐨?EM 琛ㄥ簲涓庢潯鐩暟閲忓拰璁惧鎸囬拡涓€璧蜂紶鍏ャ€傚綋 cost 鍊肩殑璁＄畻
姝ｇ‘瀹屾垚鏃讹紝鍑芥暟杩斿洖鍊间负 0銆傝鍑芥暟杩樿礋璐ｄ负姣忎釜鎬ц兘**鐘舵€?*姝ｇ‘璁剧疆浣庢晥鍊硷紝骞剁浉搴斿湴鏇存柊
em_perf_state : flags銆?
闅忓悗锛岃繖鏍峰噯澶囧ソ鐨勬柊 EM 鍙互浼犻€掔粰 em_dev_update_perf_domain() 鍑芥暟锛屼粠鑰屼娇鍏跺彲鐢ㄣ€?
鏈夊叧涓婅堪 API 鐨勬洿澶氳鎯呭彲鍦?`<linux/energy_model.h>` 鎴栫 3.2 鑺備腑鎵惧埌锛屽叾涓寘鍚竴涓?绀轰緥锛屽睍绀轰簡璁惧椹卞姩涓洿鏂版満鍒剁殑绠€鍗曞疄鐜般€?

##### 2.5 姝?API 鐨勮缁嗘弿杩?
   :internal:

   :export:


### 3. 绀轰緥


##### 3.1 娉ㄥ唽 EM 鐨勭ず渚嬮┍鍔?

CPUFreq 妗嗘灦鏀寔涓撶敤鍥炶皟锛岀敤浜庢敞鍐?*缁欏畾 CPU 鐨?'policy' 瀵硅薄鐨?EM锛歝pufreq_driver**
: register_em()銆傚繀椤婚拡瀵圭壒瀹氶┍鍔ㄦ纭疄鐜拌鍥炶皟锛屽洜涓烘鏋朵細鍦ㄨ缃湡闂寸殑閫傚綋鏃舵満璋冪敤瀹冦€?鏈妭鎻愪緵浜嗕竴涓畝鍗曠ず渚嬶紝灞曠ず涓€涓?CPUFreq 椹卞姩浣跨敤锛堣櫄鏋勭殑锛?foo' 鍗忚鍦ㄨ兘閲忔ā鍨嬫鏋朵腑
娉ㄥ唽鎬ц兘鍩熴€傝椹卞姩瀹炵幇浜?est_power() 鍑芥暟锛屾彁渚涚粰

```

  -> drivers/cpufreq/foo_cpufreq.c

  01	static int est_power(struct device *dev, unsigned long *mW,
  02			unsigned long *KHz)
  03	{
  04		long freq, power;
  05
  06		/* Use the 'foo' protocol to ceil the frequency */
  07		freq = foo_get_freq_ceil(dev, *KHz);
  08		if (freq < 0)
  09			return freq;
  10
  11		/* Estimate the power cost for the dev at the relevant freq. */
  12		power = foo_estimate_power(dev, freq);
  13		if (power < 0)
  14			return power;
  15
  16		/* Return the values to the EM framework */
  17		*mW = power;
  18		*KHz = freq;
  19
  20		return 0;
  21	}
  22
  23	static void foo_cpufreq_register_em(struct cpufreq_policy *policy)
  24	{
  25		struct em_data_callback em_cb = EM_DATA_CB(est_power);
  26		struct device *cpu_dev;
  27		int nr_opp;
  28
  29		cpu_dev = get_cpu_device(cpumask_first(policy->cpus));
  30
  31     	/* Find the number of OPPs for this policy */
  32     	nr_opp = foo_get_nr_opp(policy);
  33
  34     	/* And register the new performance domain */
  35     	em_dev_register_perf_domain(cpu_dev, nr_opp, &em_cb, policy->cpus,
  36					    true);
  37	}
  38
  39	static struct cpufreq_driver foo_cpufreq_driver = {
  40		.register_em = foo_cpufreq_register_em,
  41	};


```
##### 3.2 淇敼 EM 鐨勭ず渚嬮┍鍔?

鏈妭鎻愪緵浜嗕竴涓畝鍗曠殑鐑鐞嗛┍鍔ㄤ慨鏀?EM 鐨勭ず渚嬨€傝椹卞姩瀹炵幇浜?foo_thermal_em_update()
鍑芥暟銆傞┍鍔ㄨ鍞ら啋

```

  -> drivers/soc/example/example_em_mod.c

  01	static void foo_get_new_em(struct foo_context *ctx)
  02	{
  03		struct em_perf_table __rcu *em_table;
  04		struct em_perf_state *table, *new_table;
  05		struct device *dev = ctx->dev;
  06		struct em_perf_domain *pd;
  07		unsigned long freq;
  08		int i, ret;
  09
  10		pd = em_pd_get(dev);
  11		if (!pd)
  12			return;
  13
  14		em_table = em_table_alloc(pd);
  15		if (!em_table)
  16			return;
  17
  18		new_table = em_table->state;
  19
  20		rcu_read_lock();
  21		table = em_perf_state_from_pd(pd);
  22		for (i = 0; i < pd->nr_perf_states; i++) {
  23			freq = table[i].frequency;
  24			foo_get_power_perf_values(dev, freq, &new_table[i]);
  25		}
  26		rcu_read_unlock();
  27
  28		/* Calculate 'cost' values for EAS */
  29		ret = em_dev_compute_costs(dev, new_table, pd->nr_perf_states);
  30		if (ret) {
  31			dev_warn(dev, "EM: compute costs failed %d\n", ret);
  32			em_table_free(em_table);
  33			return;
  34		}
  35
  36		ret = em_dev_update_perf_domain(dev, em_table);
  37		if (ret) {
  38			dev_warn(dev, "EM: update failed %d\n", ret);
  39			em_table_free(em_table);
  40			return;
  41		}
  42
  43		/*
  44		 * Since it's one-time-update drop the usage counter.
  45		 * The EM framework will later free the table when needed.
  46		 */
  47		em_table_free(em_table);
  48	}
  49
  50	/*
  51	 * Function called periodically to check the temperature and
  52	 * update the EM if needed
  53	 */
  54	static void foo_thermal_em_update(struct foo_context *ctx)
  55	{
  56		struct device *dev = ctx->dev;
  57		int cpu;
  58
  59		ctx->temperature = foo_get_temp(dev, ctx);
  60		if (ctx->temperature < FOO_EM_UPDATE_TEMP_THRESHOLD)
  61			return;
  62
  63		foo_get_new_em(ctx);
  64	}


```
