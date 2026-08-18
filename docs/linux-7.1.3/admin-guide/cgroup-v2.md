
## 鎺у埗缁?v2


:Date: October, 2015
:Author: Tejun Heo <tj@kernel.org>

鏈枃浠舵槸鍏充簬 cgroup v2 鐨勮璁°€佹帴鍙ｄ笌绾﹀畾鐨勬潈濞佹枃妗ｃ€傚畠鎻忚堪浜?cgroup 鎵€鏈夊鐢ㄦ埛绌洪棿鍙鐨勬柟闈紝鍖呮嫭鏍稿績涓庡悇鍏蜂綋鎺у埗鍣ㄧ殑琛屼负銆備粖鍚庢墍鏈夌殑鍙樻洿閮藉繀椤诲弽鏄犲埌鏈枃浠朵腑銆傚叧浜?v1 鐨勬枃妗ｅ彲鍦?Documentation/admin-guide/cgroup-v1/index.rst <cgroup-v1> 涓壘鍒般€?


   [鏃犺浣曟椂鍚戞湰鏂囦欢鏂板浠讳綍绔犺妭锛岃鍚屾椂鍦ㄦ澶勬坊鍔犲搴旀潯鐩€俔

   1. 绠€浠?
     1-1. 鏈
     1-2. 浠€涔堟槸 cgroup锛?
   2. 鍩烘湰鎿嶄綔
     2-1. 鎸傝浇
     2-2. 缁勭粐杩涚▼涓庣嚎绋?
       2-2-1. 杩涚▼
       2-2-2. 绾跨▼
     2-3. [瑙ｉ櫎]濉厖閫氱煡
     2-4. 鎺у埗鎺у埗鍣?
       2-4-1. 鍙敤鎬?
       2-4-2. 鍚敤涓庣鐢?
       2-4-3. 鑷笂鑰屼笅绾︽潫
       2-4-4. 鏃犲唴閮ㄨ繘绋嬬害鏉?
     2-5. 濮旀墭
       2-5-1. 濮旀墭妯″瀷
       2-5-2. 濮旀墭 containment锛堥殧绂伙級
     2-6. 鍑嗗垯
       2-6-1. 缁勭粐涓€娆″苟鎺у埗
       2-6-2. 閬垮厤鍚嶇О鍐茬獊
   3. 璧勬簮鍒嗛厤妯″瀷
     3-1. 鏉冮噸
     3-2. 闄愰
     3-3. 淇濇姢
     3-4. 鍒嗛厤
   4. 鎺ュ彛鏂囦欢
     4-1. 鏍煎紡
     4-2. 绾﹀畾
     4-3. 鏍稿績鎺ュ彛鏂囦欢
   5. 鎺у埗鍣?
     5-1. CPU
       5-1-1. CPU 鎺ュ彛鏂囦欢
     5-2. 鍐呭瓨
       5-2-1. 鍐呭瓨鎺ュ彛鏂囦欢
       5-2-2. 浣跨敤鍑嗗垯
       5-2-3. 鍥炴敹淇濇姢
       5-2-4. 鍐呭瓨鎵€鏈夋潈
     5-3. IO
       5-3-1. IO 鎺ュ彛鏂囦欢
       5-3-2. 鍥炲啓
       5-3-3. IO 寤惰繜
         5-3-3-1. IO 寤惰繜鑺傛祦濡備綍宸ヤ綔
         5-3-3-2. IO 寤惰繜鎺ュ彛鏂囦欢
       5-3-4. IO 浼樺厛绾?
     5-4. PID
       5-4-1. PID 鎺ュ彛鏂囦欢
     5-5. Cpuset
       5.5-1. Cpuset 鎺ュ彛鏂囦欢
     5-6. 璁惧鎺у埗鍣?
     5-7. RDMA
       5-7-1. RDMA 鎺ュ彛鏂囦欢
     5-8. DMEM
       5-8-1. DMEM 鎺ュ彛鏂囦欢
     5-9. HugeTLB
       5.9-1. HugeTLB 鎺ュ彛鏂囦欢
     5-10. Misc
       5.10-1 Misc 鎺ュ彛鏂囦欢
       5.10-2 杩佺Щ涓庢墍鏈夋潈
     5-11. 鍏朵粬
       5-11-1. perf_event
     5-N. 闈炶鑼冩€т俊鎭?
       5-N-1. CPU 鎺у埗鍣ㄦ牴 cgroup 杩涚▼琛屼负
       5-N-2. IO 鎺у埗鍣ㄦ牴 cgroup 杩涚▼琛屼负
   6. 鍛藉悕绌洪棿
     6-1. 鍩虹
     6-2. 鏍逛笌瑙嗗浘
     6-3. 杩佺Щ涓?setns(2)
     6-4. 涓庡叾浠栧懡鍚嶇┖闂寸殑浜や簰
   P. 鍐呮牳缂栫▼鐩稿叧淇℃伅
     P-1. 鍥炲啓鐩稿叧鐨勬枃浠剁郴缁熸敮鎸?
   D. 宸插純鐢ㄧ殑 v1 鏍稿績鐗规€?
   R. v1 瀛樺湪鐨勯棶棰樺強 v2 鐨勮璁＄悊鐢?
     R-1. 澶氶噸灞傜骇
     R-2. 绾跨▼绮掑害
     R-3. 鍐呴儴鑺傜偣涓庣嚎绋嬩箣闂寸殑绔炰簤
     R-4. 鍏朵粬鎺ュ彛闂
     R-5. 鎺у埗鍣ㄩ棶棰樺強瀵圭瓥
       R-5-1. 鍐呭瓨


## 绠€浠?


### 鏈


鈥渃group鈥?鏄?鈥渃ontrol group锛堟帶鍒剁粍锛夆€濈殑缂╁啓锛屾案杩滀笉澶у啓銆傚崟鏁板舰寮忔棦鐢ㄦ潵鎸囨暣涓壒鎬э紝涔熶綔涓洪檺瀹氳浣跨敤锛屽 鈥渃group controllers锛坈group 鎺у埗鍣級鈥濄€傚綋鏄庣‘鎸囦唬澶氫釜鐙珛鐨勬帶鍒剁粍鏃讹紝浣跨敤澶嶆暟褰㈠紡 鈥渃groups鈥濄€?


### 浠€涔堟槸 cgroup锛?


cgroup 鏄竴绉嶅皢杩涚▼鎸夊眰娆＄粍缁囥€佸苟浠ュ彲鎺т笖鍙厤缃殑鏂瑰紡娌胯灞傛鍒嗛厤绯荤粺璧勬簮鐨勬満鍒躲€?

cgroup 澶т綋涓婄敱涓ら儴鍒嗙粍鎴愨€斺€旀牳蹇冿紙core锛変笌鎺у埗鍣紙controllers锛夈€俢group 鏍稿績涓昏璐熻矗鎸夊眰娆＄粍缁囪繘绋嬨€俢group 鎺у埗鍣ㄩ€氬父璐熻矗娌垮眰娆″垎閰嶆煇涓€鐗瑰畾绫诲瀷鐨勭郴缁熻祫婧愶紝涓嶈繃涔熷瓨鍦ㄤ竴浜涚敤浜庤祫婧愬垎閰嶄箣澶栫敤閫旂殑瀹炵敤鍨嬫帶鍒跺櫒锛坲tility controllers锛夈€?

cgroups 鏋勬垚鏍戝舰缁撴瀯锛岀郴缁熶腑鐨勬瘡涓繘绋嬮兘涓斾粎灞炰簬涓€涓?cgroup銆備竴涓繘绋嬬殑鎵€鏈夌嚎绋嬮兘灞炰簬鍚屼竴涓?cgroup銆傝繘绋嬪湪鍒涘缓鏃朵細琚斁鍏ュ叾鐖惰繘绋嬪綋鏃舵墍灞炵殑 cgroup銆傝繘绋嬪彲浠ヨ杩佺Щ鍒板彟涓€涓?cgroup銆傝縼绉讳竴涓繘绋嬩笉浼氬奖鍝嶅凡缁忓瓨鍦ㄧ殑鍚庝唬杩涚▼銆?

閬靛惊鐗瑰畾鐨勭粨鏋勬€х害鏉燂紝鎺у埗鍣ㄥ彲浠ュ湪鏌愪釜 cgroup 涓婃湁閫夋嫨鍦板惎鐢ㄦ垨绂佺敤銆傛墍鏈夋帶鍒跺櫒鐨勮涓洪兘鏄垎灞傜殑鈥斺€斿鏋滄煇涓帶鍒跺櫒鍦ㄦ煇涓?cgroup 涓婅鍚敤锛屽畠浼氬奖鍝嶅睘浜庢瀯鎴愯 cgroup 鍖呭惈鎬у瓙灞傛锛坕nclusive sub-hierarchy锛夌殑鎵€鏈?cgroup 鐨勮繘绋嬨€傚綋鏌愪釜鎺у埗鍣ㄥ湪宓屽鐨?cgroup 涓婅鍚敤鏃讹紝瀹冩€绘槸杩涗竴姝ラ檺鍒惰祫婧愬垎閰嶃€傚湪灞傛涓洿闈犺繎鏍硅妭鐐规墍璁剧疆鐨勭害鏉燂紝鏃犳硶琚洿杩滅殑鑺傜偣瑕嗙洊銆?


## 鍩烘湰鎿嶄綔


### 鎸傝浇


涓?v1 涓嶅悓锛宑group v2 鍙湁鍗曚竴鐨勫眰绾с€俢group v2
```

  # mount -t cgroup2 none $MOUNT_POINT

```
cgroup2 鏂囦欢绯荤粺鐨勯瓟鏁颁负 0x63677270锛堚€渃grp鈥濓級銆傛墍鏈夋敮鎸?v2銆佷笖鏈粦瀹氬埌鏌愪釜 v1 灞傜骇鐨勬帶鍒跺櫒閮戒細鑷姩缁戝畾鍒?v2 灞傜骇骞跺嚭鐜板湪鏍硅妭鐐广€傛湭鍦?v2 灞傜骇涓浜庢椿鍔ㄤ娇鐢ㄧ殑鎺у埗鍣ㄥ彲浠ョ粦瀹氬埌鍏朵粬灞傜骇銆傝繖浣垮緱浠ュ畬鍏ㄥ悜鍚庡吋瀹圭殑鏂瑰紡灏?v2 灞傜骇涓庨仐鐣欑殑 v1 澶氶噸灞傜骇娣峰悎浣跨敤鎴愪负鍙兘銆?

鍙湁褰撴煇涓帶鍒跺櫒鍦ㄥ叾褰撳墠灞傜骇涓笉鍐嶈寮曠敤鍚庯紝瀹冩墠鑳借法灞傜骇绉诲姩銆傜敱浜庢瘡涓?cgroup 鐨勬帶鍒跺櫒鐘舵€佹槸寮傛閿€姣佺殑锛屼笖鎺у埗鍣ㄥ彲鑳芥寔鏈夋粸鐣欏紩鐢紝鍥犳鍦ㄤ笂涓€灞傜骇鏈€缁?umount 涔嬪悗锛岃鎺у埗鍣ㄥ彲鑳戒笉浼氱珛鍗冲嚭鐜板湪 v2 灞傜骇涓娿€傜被浼煎湴锛屼竴涓帶鍒跺櫒蹇呴』鍏堣瀹屽叏绂佺敤鎵嶈兘绉诲嚭缁熶竴灞傜骇锛岃€屽畠鍙兘闇€瑕佷竴浜涙椂闂存墠鑳藉鍏跺畠灞傜骇鍙樹负鍙敤锛涙澶栵紝鐢变簬鎺у埗鍣ㄤ箣闂寸殑鐩镐簰渚濊禆鍏崇郴锛屽彲鑳借繕闇€瑕佺鐢ㄥ叾浠栨帶鍒跺櫒銆?

铏界劧鍦ㄥ紑鍙戝拰鎵嬪姩閰嶇疆鏃跺緢鏈夌敤锛屼絾鍦ㄧ敓浜х幆澧冧腑寮虹儓涓嶅缓璁湪 v2 涓庡叾瀹冨眰绾т箣闂村姩鎬佺Щ鍔ㄦ帶鍒跺櫒銆傚缓璁湪绯荤粺鍚姩鍚庛€佸紑濮嬩娇鐢ㄦ帶鍒跺櫒涔嬪墠灏卞喅瀹氬ソ灞傜骇缁撴瀯涓庢帶鍒跺櫒鍏宠仈銆?

鍦ㄥ悜 v2 杩囨浮鏈熼棿锛岀郴缁熺鐞嗚蒋浠跺彲鑳戒粛浼氳嚜鍔ㄦ寕杞?v1 鐨?cgroup 鏂囦欢绯荤粺锛屼粠鑰屽湪鎵嬪姩骞查鎴愪负鍙兘涔嬪墠浜庡惎鍔ㄩ樁娈靛姭鎸佹墍鏈夋帶鍒跺櫒銆備负浜嗚娴嬭瘯鍜岃瘯楠屾洿鏂逛究锛屽唴鏍稿弬鏁?cgroup_no_v1= 鍏佽鍦?v1 涓鐢ㄦ帶鍒跺櫒锛屽苟浣垮畠浠湪 v2 涓缁堝彲鐢ㄣ€?

cgroup v2 鐩墠鏀寔浠ヤ笅鎸傝浇閫夐」銆?

  nsdelegate
	灏?cgroup 鍛藉悕绌洪棿瑙嗕负濮旀墭杈圭晫銆傛閫夐」鏄郴缁熺骇鐨勶紝鍙兘鍦ㄦ寕杞芥椂璁剧疆锛屾垨閫氳繃浠?init 鍛藉悕绌洪棿杩涜閲嶆柊鎸傝浇鏉ヤ慨鏀广€傝鎸傝浇閫夐」鍦ㄩ潪 init 鍛藉悕绌洪棿鐨勬寕杞戒笂浼氳蹇界暐銆傝鎯呰鍙傞槄鈥滃鎵樷€濅竴鑺傘€?

  favordynmods
       闄嶄綆璇稿浠诲姟杩佺Щ鍜屾帶鍒跺櫒寮€鍏崇瓑鍔ㄦ€?cgroup 淇敼鐨勫欢杩燂紝浠ｄ环鏄娇 fork 鍜?exit 绛夌儹璺緞鎿嶄綔鐨勪唬浠锋洿楂樸€備互鈥滃垱寤?cgroup銆佸惎鐢ㄦ帶鍒跺櫒銆佺劧鍚庣敤 CLONE_INTO_CGROUP 濉厖瀹冣€濅负鐗瑰緛鐨勯潤鎬佷娇鐢ㄦā寮忎笉鍙楁閫夐」褰卞搷銆?

  memory_localevents
       浠呯敤褰撳墠 cgroup 鐨勬暟鎹紙鑰岄潪浠讳綍瀛愭爲锛夊～鍏?memory.events銆傝繖鏄仐鐣欒涓猴紝涓嶅甫姝ら€夐」鐨勯粯璁よ涓烘槸鍖呭惈瀛愭爲璁℃暟銆傛閫夐」鏄郴缁熺骇鐨勶紝鍙兘鍦ㄦ寕杞芥椂璁剧疆锛屾垨閫氳繃浠?init 鍛藉悕绌洪棿杩涜閲嶆柊鎸傝浇鏉ヤ慨鏀广€傝鎸傝浇閫夐」鍦ㄩ潪 init 鍛藉悕绌洪棿鐨勬寕杞戒笂浼氳蹇界暐銆?

  memory_recursiveprot
        灏?memory.min 鍜?memory.low 鐨勪繚鎶や互閫掑綊鏂瑰紡搴旂敤鍒版暣涓瓙鏍戯紝鑰屾棤闇€鏄惧紡鍚戜笅浼犳挱鍒板彾瀛?cgroup銆傝繖鍏佽灏嗘暣涓瓙鏍戠浉浜掗殧绂讳繚鎶わ紝鍚屾椂鍦ㄥ瓙鏍戝唴閮ㄤ繚鐣欒嚜鐢辩珵浜夈€傝繖鏈簲鏄粯璁よ涓猴紝浣嗕负浜嗛伩鍏嶈渚濊禆鍘熷璇箟锛堜緥濡傚湪鏍戠殑鏇撮珮灞傜骇鎸囧畾铏氶珮鐨勨€渂ypass鈥濅繚鎶ゅ€硷級鐨勯厤缃彂鐢熷洖褰掞紝瀹冭鍋氭垚浜嗕竴涓寕杞介€夐」銆?

  memory_hugetlb_accounting
        灏?HugeTLB 鍐呭瓨浣跨敤璁″叆 cgroup 閽堝鍐呭瓨鎺у埗鍣ㄧ殑鏁翠綋鍐呭瓨浣跨敤锛堢敤浜庣粺璁′笂鎶ヤ笌鍐呭瓨淇濇姢锛夈€傝繖鏄竴绉嶅彲鑳藉奖鍝嶇幇鏈夐厤缃殑鏂拌涓猴紝鍥犳蹇呴』閫氳繃姝ゆ寕杞介€夐」鏄惧紡閫夋嫨鍔犲叆銆?

        闇€瑕佺墷璁颁互涓嬪嚑鐐规敞鎰忎簨椤癸細

        - 鍐呭瓨鎺у埗鍣ㄤ笉娑夊強浠讳綍 HugeTLB 姹犵鐞嗐€傞鍒嗛厤鐨勬睜涓嶅睘浜庝换浣曚汉銆傚叿浣撹€岃█锛屽綋涓€涓柊鐨?HugeTLB folio 琚垎閰嶈繘姹犳椂锛屼粠鍐呭瓨鎺у埗鍣ㄧ殑瑙掑害鐪嬪畠涓嶈璁″叆銆傚彧鏈夊綋瀹冪湡姝ｈ浣跨敤锛堜緥濡傚湪澶勭悊椤甸敊璇椂锛夋椂锛屾墠浼氬悜鏌愪釜 cgroup 璁拌处銆備富鏈哄唴瀛樿繃閲忎娇鐢ㄧ鐞嗗湪閰嶇疆纭檺棰濇椂蹇呴』鑰冭檻鍒拌繖涓€鐐广€備竴鑸€岃█锛孒ugeTLB 姹犵鐞嗗簲閫氳繃鍏朵粬鏈哄埗锛堜緥濡?HugeTLB 鎺у埗鍣級瀹屾垚銆?
        - 鍚戝唴瀛樻帶鍒跺櫒璁拌处鏌愪釜 HugeTLB folio 澶辫触浼氬鑷?SIGBUS銆傚嵆浣?HugeTLB 姹犱粛鏈夊彲鐢ㄩ〉锛堜絾 cgroup 闄愰宸叉弧涓斿洖鏀跺皾璇曞け璐ワ級锛屼篃鍙兘鍙戠敓杩欑鎯呭喌銆?
        - 灏?HugeTLB 鍐呭瓨璁″叆鍐呭瓨鎺у埗鍣ㄤ細褰卞搷鍐呭瓨淇濇姢涓庡洖鏀剁殑鍔ㄦ€佽涓恒€備换浣曠敤鎴风┖闂寸殑璋冧紭锛堜緥濡傚 low銆乵in 闄愰鐨勮皟浼橈級閮介渶瑕佸皢姝よ€冭檻鍦ㄥ唴銆?
        - 鍦ㄦ湭閫夋嫨姝ら€夐」鏃朵娇鐢ㄧ殑 HugeTLB 椤典笉浼氳鍐呭瓨鎺у埗鍣ㄨ窡韪紙鍗充娇涔嬪悗鍐嶉噸鏂版寕杞?cgroup v2 涔熶笉浼氾級銆?

  pids_localevents
        姝ら€夐」鎭㈠ pids.events:max 绫讳技 v1 鐨勮涓猴紝鍗冲彧缁熻鏈湴鐨勶紙cgroup 鍐呴儴鐨勶級fork 澶辫触銆備笉甯︽閫夐」鏃讹紝pids.events.max 琛ㄧず cgroup 瀛愭爲涓婁换浣?pids.max 鐨勫己鍒舵墽琛屾儏鍐点€?


### 缁勭粐杩涚▼涓庣嚎绋?


#### 杩涚▼


鏈€鍒濓紝鍙瓨鍦ㄦ牴 cgroup锛屾墍鏈夎繘绋嬮兘灞炰簬瀹冦€?
```

  # mkdir $CGROUP_NAME

```
涓€涓粰瀹氱殑 cgroup 鍙互鏈夊涓瓙 cgroup锛屽舰鎴愭爲褰㈢粨鏋勩€傛瘡涓?cgroup 閮芥湁涓€涓彲璇诲啓鐨勬帴鍙ｆ枃浠?鈥渃group.procs鈥濄€傝鍙栨椂锛屽畠閫愯鍒楀嚭灞炰簬璇?cgroup 鐨勬墍鏈夎繘绋嬬殑 PID銆侾ID 娌℃湁鐗瑰畾椤哄簭锛屽鏋滄煇涓繘绋嬭绉诲姩鍒板彟涓€涓?cgroup 鍚庡張绉诲洖锛屾垨鑰呭湪璇诲彇杩囩▼涓?PID 琚洖鏀讹紝鍚屼竴涓?PID 鍙兘浼氬嚭鐜板娆°€?

閫氳繃灏嗘煇涓繘绋嬬殑 PID 鍐欏叆鐩爣 cgroup 鐨?鈥渃group.procs鈥?鏂囦欢锛屽彲浠ユ妸璇ヨ繘绋嬭縼绉昏繘璇?cgroup銆傚崟娆?write(2) 璋冪敤鍙兘杩佺Щ涓€涓繘绋嬨€傚鏋滀竴涓繘绋嬬敱澶氫釜绾跨▼缁勬垚锛屽啓鍏ュ叾涓换鎰忎竴涓嚎绋嬬殑 PID 閮戒細杩佺Щ璇ヨ繘绋嬬殑鎵€鏈夌嚎绋嬨€?

褰撲竴涓繘绋?fork 鍑哄瓙杩涚▼鏃讹紝鏂拌繘绋嬭癁鐢熶簬鎵ц fork 鎿嶄綔鐨勮繘绋嬪綋鏃舵墍灞炵殑 cgroup銆傝繘绋嬮€€鍑哄悗锛屼細涓€鐩村叧鑱斿埌瀹冮€€鍑烘椂鎵€灞炵殑 cgroup锛岀洿鍒拌鍥炴敹锛坮eaped锛夛紱涓嶈繃锛屽兊灏歌繘绋嬩笉浼氬嚭鐜板湪 鈥渃group.procs鈥?涓紝鍥犳鏃犳硶琚Щ鍔ㄥ埌鍙︿竴涓?cgroup銆?

涓€涓病鏈変换浣曞瓙 cgroup 鎴栨椿鍔ㄨ繘绋嬬殑 cgroup 鍙互閫氳繃鍒犻櫎鐩綍鏉ラ攢姣併€傛敞鎰忥紝涓€涓病鏈夊瓙 cgroup銆佷笖鍙笌鍍靛案杩涚▼鍏宠仈鐨?cgroup 鏄?
```

  # rmdir $CGROUP_NAME

```
鈥?proc/$PID/cgroup鈥?鍒楀嚭浜嗚繘绋嬬殑 cgroup 褰掑睘銆傚鏋滅郴缁熶腑姝ｅ湪浣跨敤閬楃暀 cgroup锛岃繖涓枃浠跺彲鑳藉寘鍚琛岋紝姣忎釜灞傜骇涓€琛屻€俢group v2 鐨勬潯鐩缁堜綅浜?
```

  # cat /proc/842/cgroup
  ...
  0::/test-cgroup/test-cgroup-nested

```
濡傛灉杩涚▼鍙樻垚浜嗗兊灏革紝涓斿畠鍏宠仈鐨?cgroup 宸茶
```

  # cat /proc/842/cgroup
  ...
  0::/test-cgroup/test-cgroup-nested (deleted)


```
#### 绾跨▼


cgroup v2 瀵逛竴閮ㄥ垎鎺у埗鍣ㄦ敮鎸佺嚎绋嬬矑搴︼紝浠ユ弧瓒抽渶瑕佸湪杩涚▼缁勫悇绾跨▼涔嬮棿杩涜鍒嗗眰璧勬簮鍒嗛厤鐨勪娇鐢ㄥ満鏅€傞粯璁ゆ儏鍐典笅锛屼竴涓繘绋嬬殑鎵€鏈夌嚎绋嬮兘灞炰簬鍚屼竴涓?cgroup锛岃 cgroup 鍚屾椂涔熶綔涓鸿祫婧愬煙锛坮esource domain锛夛紝鎵胯浇涓嶅睘浜庢煇涓壒瀹氳繘绋嬫垨绾跨▼鐨勮祫婧愭秷鑰椼€傜嚎绋嬫ā寮忓厑璁哥嚎绋嬫暎甯冨湪瀛愭爲涓婏紝鍚屾椂浠嶇劧涓哄畠浠淮鎶ゅ叡鍚岀殑璧勬簮鍩熴€?

鏀寔绾跨▼妯″紡鐨勬帶鍒跺櫒绉颁负绾跨▼鍖栨帶鍒跺櫒锛坱hreaded controllers锛夈€備笉鏀寔鐨勭О涓哄煙鎺у埗鍣紙domain controllers锛夈€?

灏嗕竴涓?cgroup 鏍囪涓虹嚎绋嬪寲浼氫娇瀹冧綔涓虹嚎绋嬪寲 cgroup 鍔犲叆鍏剁埗鑺傜偣鐨勮祫婧愬煙銆傜埗鑺傜偣鍙互鏄彟涓€涓嚎绋嬪寲 cgroup锛屽叾璧勬簮鍩熷湪灞傛涓洿闈犱笂銆傜嚎绋嬪寲瀛愭爲鐨勬牴鈥斺€斿嵆鏈€杩戠殑銆侀潪绾跨▼鍖栫殑绁栧厛鈥斺€斿彲浜掓崲鍦扮О涓虹嚎绋嬪寲鍩燂紙threaded domain锛夋垨绾跨▼鏍癸紙thread root锛夛紝骞朵綔涓烘暣涓瓙鏍戠殑璧勬簮鍩熴€?

鍦ㄧ嚎绋嬪寲瀛愭爲鍐呴儴锛屼竴涓繘绋嬬殑绾跨▼鍙互琚斁鍏ヤ笉鍚岀殑 cgroup锛屼笖涓嶅彈鈥滄棤鍐呴儴杩涚▼鈥濈害鏉熺殑闄愬埗鈥斺€旂嚎绋嬪寲鎺у埗鍣ㄥ彲浠ュ湪闈炲彾瀛?cgroup 涓婂惎鐢紝鏃犺鍏朵腑鏄惁鏈夌嚎绋嬨€?

鐢变簬绾跨▼鍖栧煙 cgroup 鎵胯浇浜嗘暣妫靛瓙鏍戠殑鎵€鏈夊煙璧勬簮娑堣€楋紝鏃犺鍏朵腑鏄惁鏈夎繘绋嬶紝閮借璁や负鍏锋湁鍐呴儴璧勬簮娑堣€楋紝鍥犳涓嶈兘鎷ユ湁闈炵嚎绋嬪寲鐨勫凡濉厖瀛?cgroup銆傜敱浜庢牴 cgroup 涓嶅彈鈥滄棤鍐呴儴杩涚▼鈥濈害鏉熼檺鍒讹紝瀹冨彲浠ュ悓鏃跺厖褰撶嚎绋嬪寲鍩熷拰鍩?cgroup 鐨勭埗鑺傜偣銆?

cgroup 鐨勫綋鍓嶆搷浣滄ā寮忔垨绫诲瀷鏄剧ず鍦?鈥渃group.type鈥?鏂囦欢涓紝瀹冭〃鏄庤 cgroup 鏄櫘閫氬煙銆佷綔涓烘煇绾跨▼鍖栧瓙鏍戜箣鍩熺殑鍩燂紝杩樻槸涓€涓嚎绋嬪寲 cgroup銆?

鍦ㄥ垱寤烘椂锛宑group 濮嬬粓鏄煙 cgroup锛屽彲浠ラ€氳繃鍚?鈥渃group.type鈥?鏂囦欢鍐欏叆 鈥渢hreaded鈥?鍙樹负绾跨▼鍖栥€?
```

  # echo threaded > cgroup.type

```
涓€鏃︾嚎绋嬪寲锛岃 cgroup 灏辨棤娉曞啀鍙樺洖鍩熴€傝鍚敤绾跨▼妯″紡锛屽繀椤绘弧瓒充互涓嬫潯浠躲€?

- 鐢变簬璇?cgroup 灏嗗姞鍏ョ埗鑺傜偣鐨勮祫婧愬煙銆傜埗鑺傜偣蹇呴』鏄湁鏁堢殑锛堢嚎绋嬪寲锛夊煙鎴栫嚎绋嬪寲 cgroup銆?

- 褰撶埗鑺傜偣鏄湭绾跨▼鍖栫殑鍩熸椂锛屽畠涓嶈兘鍚敤浠讳綍鍩熸帶鍒跺櫒锛屼篃涓嶈兘鏈夊凡濉厖鐨勫煙瀛愯妭鐐广€傛牴鑺傜偣涓嶅彈姝よ姹傞檺鍒躲€?

浠庢嫇鎵戣搴︾湅锛屼竴涓?cgroup 鍙兘澶勪簬鏃犳晥鐘舵€併€傝鐪嬩笅渚?
```

  A (threaded domain) - B (threaded) - C (domain, just created)

```
C 琚垱寤轰负鍩燂紝浣嗗苟鏈繛鎺ュ埌涓€涓兘澶熸壙杞藉瓙鍩熺殑鐖惰妭鐐广€傚湪鎶?C 鍙樹负绾跨▼鍖?cgroup 涔嬪墠锛屽畠鏃犳硶琚娇鐢ㄣ€傚湪杩欎簺鎯呭喌涓嬶紝鈥渃group.type鈥?鏂囦欢浼氭姤鍛?鈥渄omain (invalid)鈥濄€傚洜鏃犳晥鎷撴墤鑰屽け璐ョ殑鎿嶄綔浣跨敤 EOPNOTSUPP 浣滀负 errno銆?

褰撲竴涓?cgroup 鐨勬煇涓瓙 cgroup 鍙樹负绾跨▼鍖栵紝鎴栬€呭湪 cgroup 涓粛鏈夎繘绋嬫椂浜?鈥渃group.subtree_control鈥?鏂囦欢涓惎鐢ㄧ嚎绋嬪寲鎺у埗鍣紝璇ュ煙 cgroup 浼氬彉涓虹嚎绋嬪寲鍩熴€傚綋杩欎簺鏉′欢娓呴櫎鍚庯紝绾跨▼鍖栧煙浼氭仮澶嶄负鏅€氬煙銆?

璇诲彇鏃讹紝鈥渃group.threads鈥?鍖呭惈璇?cgroup 涓墍鏈夌嚎绋嬬殑绾跨▼ ID 鍒楄〃銆傞櫎浜嗘搷浣滄槸姣忕嚎绋嬭€岄潪姣忚繘绋嬩箣澶栵紝鈥渃group.threads鈥?涓?鈥渃group.procs鈥?鍏锋湁鐩稿悓鐨勬牸寮忓拰琛屼负銆傝櫧鐒?鈥渃group.threads鈥?鍙互鍦ㄤ换浣?cgroup 涓啓鍏ワ紝浣嗙敱浜庡畠鍙兘鍦ㄥ悓涓€绾跨▼鍖栧煙鍐呯Щ鍔ㄧ嚎绋嬶紝鍏舵搷浣滆闄愬埗鍦ㄦ瘡涓嚎绋嬪寲瀛愭爲涔嬪唴銆?

绾跨▼鍖栧煙 cgroup 浣滀负鏁存５瀛愭爲鐨勮祫婧愬煙锛岃櫧鐒剁嚎绋嬪彲浠ユ暎甯冨湪瀛愭爲涓紝浣嗘墍鏈夎繘绋嬮兘琚涓轰綅浜庣嚎绋嬪寲鍩?cgroup 涓€傜嚎绋嬪寲鍩?cgroup 涓殑 鈥渃group.procs鈥?鍖呭惈瀛愭爲涓墍鏈夎繘绋嬬殑 PID锛屼笖涓嶈兘鍦ㄥ瓙鏍戝唴閮ㄨ璇诲彇銆備笉杩囷紝鈥渃group.procs鈥?鍙互浠庡瓙鏍戜腑鐨勪换浣曚綅缃啓鍏ワ紝浠ュ皢鍖归厤杩涚▼鐨勬墍鏈夌嚎绋嬭縼绉诲埌璇?cgroup銆?

鍙湁绾跨▼鍖栨帶鍒跺櫒鎵嶈兘鍦ㄧ嚎绋嬪寲瀛愭爲涓惎鐢ㄣ€傚綋鏌愪釜绾跨▼鍖栨帶鍒跺櫒鍦ㄧ嚎绋嬪寲瀛愭爲鍐呴儴琚惎鐢ㄦ椂锛屽畠鍙璐﹀苟鎺у埗涓庤 cgroup 鍙婂叾鍚庝唬涓殑绾跨▼鐩稿叧鐨勮祫婧愭秷鑰椼€傛墍鏈変笉缁戝畾鍒扮壒瀹氱嚎绋嬬殑娑堣€楅兘灞炰簬绾跨▼鍖栧煙 cgroup銆?

鐢变簬绾跨▼鍖栧瓙鏍戜笉鍙椻€滄棤鍐呴儴杩涚▼鈥濈害鏉熼檺鍒讹紝绾跨▼鍖栨帶鍒跺櫒蹇呴』鑳藉澶勭悊闈炲彾瀛?cgroup 涓嚎绋嬩笌鍏跺瓙 cgroup 涔嬮棿鐨勭珵浜夈€傛瘡涓嚎绋嬪寲鎺у埗鍣ㄥ悇鑷畾涔変簡杩欑绔炰簤濡備綍澶勭悊銆?

鐩墠锛屼互涓嬫帶鍒跺櫒鏄嚎绋嬪寲鐨勶紝鍙互鍦ㄥ叾涓惎鐢?
```

```
- cpu
- cpuset
- perf_event
- pids

### [瑙ｉ櫎]濉厖閫氱煡


姣忎釜闈炴牴 cgroup 閮芥湁涓€涓?鈥渃group.events鈥?鏂囦欢锛屽叾涓寘鍚?鈥減opulated鈥?瀛楁锛屾寚绀鸿 cgroup 鐨勫瓙灞傛涓槸鍚︽湁娲诲姩杩涚▼銆傚鏋?cgroup 鍙婂叾鍚庝唬涓病鏈夋椿鍔ㄨ繘绋嬶紝鍏跺€间负 0锛涘惁鍒欎负 1銆傚綋鍊煎彂鐢熷彉鍖栨椂浼氳Е鍙?poll 鍜?[id]notify 浜嬩欢銆備緥濡傦紝杩欏彲鐢ㄤ簬鍦ㄦ煇涓瓙灞傛鐨勬墍鏈夎繘绋嬮€€鍑哄悗鍚姩娓呯悊鎿嶄綔銆傚～鍏呯姸鎬佺殑鏇存柊涓庨€氱煡鏄€掑綊鐨勩€傝€冭檻浠ヤ笅瀛愬眰娆★紝鍏朵腑鎷彿鍐呯殑鏁板瓧琛ㄧず杩涚▼鏁?
```

  A(4) - B(0) - C(1)
              \ D(0)

```
A銆丅 鍜?C 鐨?鈥減opulated鈥?瀛楁涓?1锛岃€?D 鐨勪负 0銆侰 涓殑閭ｄ竴涓繘绋嬮€€鍑哄悗锛孊 鍜?C 鐨?鈥減opulated鈥?瀛楁浼氱炕杞负 鈥?鈥濓紝骞跺湪杩欎袱涓?cgroup 鐨?鈥渃group.events鈥?鏂囦欢涓婄敓鎴愭枃浠朵慨鏀逛簨浠躲€?


### 鎺у埗鎺у埗鍣?


#### 鍙敤鎬?


褰撴煇涓帶鍒跺櫒琚唴鏍告敮鎸侊紙鍗冲凡缂栬瘧杩涘唴鏍搞€佹湭琚鐢ㄣ€佷篃鏈寕鎺ュ埌 v1 灞傜骇锛夛紝骞朵笖鍒楀湪 鈥渃group.controllers鈥?鏂囦欢涓椂锛屽畠鍦ㄨ cgroup 涓氨鏄彲鐢ㄧ殑銆傚彲鐢ㄦ€ф剰鍛崇潃璇ユ帶鍒跺櫒鐨勬帴鍙ｆ枃浠朵細琚毚闇插湪 cgroup 鐨勭洰褰曚腑锛屼粠鑰岃兘澶熷湪璇?cgroup 鍐呰瀵熸垨鎺у埗鐩爣璧勬簮鐨勫垎閰嶃€?


#### 鍚敤涓庣鐢?


姣忎釜 cgroup 閮芥湁涓€涓?鈥渃group.controllers鈥?鏂囦欢锛屽畠鍒楀嚭浜嗘墍鏈?
```

  # cat cgroup.controllers
  cpu io memory

```
榛樿涓嶅惎鐢ㄤ换浣曟帶鍒跺櫒銆傛帶鍒跺櫒鍙互閫氳繃濡備笅鏂瑰紡鍚敤鍜岀鐢?
```

  # echo "+cpu +memory -io" > cgroup.subtree_control

```
鍙湁鍒楀湪 鈥渃group.controllers鈥?涓殑鎺у埗鍣ㄦ墠鑳借鍚敤銆傚綋鍍忎笂闈㈤偅鏍锋寚瀹氬涓搷浣滄椂锛屽畠浠涔堝叏閮ㄦ垚鍔燂紝瑕佷箞鍏ㄩ儴澶辫触銆傚鏋滃鍚屼竴涓帶鍒跺櫒鎸囧畾浜嗗涓搷浣滐紝鏈€鍚庝竴涓敓鏁堛€?

鍦ㄦ煇涓?cgroup 涓婂惎鐢ㄦ帶鍒跺櫒锛屾剰鍛崇潃鍏剁洿鎺ュ瓙鑺傜偣涔嬮棿鐨勭洰鏍囪祫婧愬垎閰嶅皢鍙楀埌鎺у埗銆傝€冭檻浠ヤ笅瀛愬眰娆°€傚凡鍚敤鐨勬帶鍒跺櫒涓?
```

  A(cpu,memory) - B(memory) - C()
                            \ D()

```
鐢变簬 A 鍚敤浜?鈥渃pu鈥?鍜?鈥渕emory鈥濓紝A 灏嗘帶鍒跺鍏跺瓙鑺傜偣锛堟湰渚嬩腑鍗?B锛夌殑 CPU 鍛ㄦ湡涓庡唴瀛樺垎閰嶃€傜敱浜?B 鍚敤浜?鈥渕emory鈥?浣嗘湭鍚敤 鈥淐PU鈥濓紝C 鍜?D 灏嗗湪 CPU 鍛ㄦ湡涓婅嚜鐢辩珵浜夛紝浣嗗畠浠粠 B 鍙幏寰楃殑鍐呭瓨鍒掑垎灏嗗彈鎺с€?

鐢变簬鎺у埗鍣ㄨ皟鑺傜洰鏍囪祫婧愬悜鍏?cgroup 瀛愯妭鐐圭殑鍒嗛厤锛屽惎鐢ㄥ畠浼氬湪瀛?cgroup 涓垱寤鸿鎺у埗鍣ㄧ殑鎺ュ彛鏂囦欢銆傚湪涓婁緥涓紝鍦?B 涓婂惎鐢?鈥渃pu鈥?浼氬湪 C 鍜?D 涓垱寤轰互 鈥渃pu.鈥?涓哄墠缂€鐨勬帶鍒跺櫒鎺ュ彛鏂囦欢銆傚悓鏍峰湴锛屼粠 B 绂佺敤 鈥渕emory鈥?浼氫粠 C 鍜?D 涓Щ闄や互 鈥渕emory.鈥?涓哄墠缂€鐨勬帶鍒跺櫒鎺ュ彛鏂囦欢銆傝繖鎰忓懗鐫€鎺у埗鍣ㄦ帴鍙ｆ枃浠垛€斺€斾换浣曚笉浠?鈥渃group.鈥?寮€澶寸殑鏂囦欢鈥斺€旂敱鐖惰妭鐐硅€岄潪 cgroup 鑷韩鎷ユ湁銆?


#### 鑷笂鑰屼笅绾︽潫


璧勬簮鏄嚜涓婅€屼笅鍒嗛厤鐨勶紝涓€涓?cgroup 鍙湁鍦ㄧ埗鑺傜偣宸插悜瀹冨垎閰嶄簡鏌愯祫婧愪箣鍚庯紝鎵嶈兘杩涗竴姝ュ垎閰嶈璧勬簮銆傝繖鎰忓懗鐫€鎵€鏈夐潪鏍圭殑 鈥渃group.subtree_control鈥?鏂囦欢鍙兘鍖呭惈鍦ㄥ叾鐖惰妭鐐圭殑 鈥渃group.subtree_control鈥?鏂囦欢涓惎鐢ㄧ殑鎺у埗鍣ㄣ€傚彧鏈夊綋鐖惰妭鐐瑰惎鐢ㄤ簡鏌愪釜鎺у埗鍣ㄦ椂锛岃鎺у埗鍣ㄦ墠鑳借鍚敤锛涜€屽鏋滄湁涓€涓垨澶氫釜瀛愯妭鐐瑰惎鐢ㄤ簡鏌愭帶鍒跺櫒锛屽垯璇ユ帶鍒跺櫒涓嶈兘琚鐢ㄣ€?


#### 鏃犲唴閮ㄨ繘绋嬬害鏉?


闈炴牴 cgroup 鍙湁鍦ㄨ嚜韬病鏈変换浣曡繘绋嬫椂锛屾墠鑳藉悜瀛愯妭鐐瑰垎閰嶅煙璧勬簮銆傛崲瑷€涔嬶紝鍙湁涓嶅寘鍚换浣曡繘绋嬬殑鍩?cgroup 鎵嶈兘鍦ㄥ叾 鈥渃group.subtree_control鈥?鏂囦欢涓惎鐢ㄥ煙鎺у埗鍣ㄣ€?

杩欎繚璇佷簡锛氬綋鏌愪釜鍩熸帶鍒跺櫒瑙傚療宸插惎鐢ㄥ畠鐨勯偅閮ㄥ垎灞傛鏃讹紝杩涚▼姘歌繙鍙綅浜庡彾瀛愯妭鐐广€傝繖灏辨帓闄や簡瀛?cgroup 涓庣埗鑺傜偣鍐呴儴杩涚▼鐩镐簰绔炰簤鐨勬儏鍐点€?

鏍?cgroup 涓嶅彈姝ら檺鍒躲€傛牴鑺傜偣鍖呭惈杩涚▼浠ュ強鏃犳硶涓庝换浣曞叾浠?cgroup 鍏宠仈鐨勫尶鍚嶈祫婧愭秷鑰楋紝闇€瑕佸ぇ澶氭暟鎺у埗鍣ㄧ殑鐗规畩澶勭悊銆傛牴 cgroup 涓殑璧勬簮娑堣€楀浣曟不鐞嗗彇鍐充簬鍚勪釜鎺у埗鍣紙鏈夊叧姝や富棰樼殑鏇村淇℃伅锛岃鍙傞槄鈥滄帶鍒跺櫒鈥濅竴绔犱腑鐨勨€滈潪瑙勮寖鎬т俊鎭€濅竴鑺傦級銆?

娉ㄦ剰锛屽鏋?cgroup 鐨?鈥渃group.subtree_control鈥?涓病鏈夊惎鐢ㄤ换浣曟帶鍒跺櫒锛岃闄愬埗骞朵笉浼氶€犳垚闃荤銆傝繖涓€鐐瑰緢閲嶈锛屽惁鍒欏皢鏃犳硶鍒涘缓宸插～鍏?cgroup 鐨勫瓙鑺傜偣銆傝鎺у埗鏌愪釜 cgroup 鐨勮祫婧愬垎閰嶏紝璇?cgroup 蹇呴』鍏堝垱寤哄瓙鑺傜偣锛屽苟灏嗚嚜韬殑鍏ㄩ儴杩涚▼杞Щ鍒拌繖浜涘瓙鑺傜偣锛岀劧鍚庢墠鑳藉湪鑷繁鐨?鈥渃group.subtree_control鈥?鏂囦欢涓惎鐢ㄦ帶鍒跺櫒銆?


### 濮旀墭


#### 濮旀墭妯″瀷


cgroup 鍙互閫氳繃涓ょ鏂瑰紡琚鎵樸€傜涓€绉嶏紝閫氳繃鎺堜簣鏌愪綆鐗规潈鐢ㄦ埛瀵圭洰褰曞強鍏?鈥渃group.procs鈥濄€佲€渃group.threads鈥?鍜?鈥渃group.subtree_control鈥?鏂囦欢鐨勫啓鏉冮檺锛屽鎵樼粰璇ョ敤鎴枫€傜浜岀锛屽鏋滆缃簡 鈥渘sdelegate鈥?鎸傝浇閫夐」锛屽垯鍦ㄥ垱寤哄懡鍚嶇┖闂存椂鑷姩濮旀墭缁欐煇涓?cgroup 鍛藉悕绌洪棿銆?

鐢变簬缁欏畾鐩綍涓殑璧勬簮鎺у埗鎺ュ彛鏂囦欢鎺у埗鐨勬槸鐖惰妭鐐硅祫婧愮殑鍒嗛厤锛屼笉搴斿厑璁歌濮旀墭鏂瑰啓鍏ヨ繖浜涙枃浠躲€傚浜庣涓€绉嶆柟寮忥紝杩欓€氳繃涓嶆巿浜堝杩欎簺鏂囦欢鐨勮闂潈闄愭潵瀹炵幇銆傚浜庣浜岀鏂瑰紡锛屽懡鍚嶇┖闂翠箣澶栫殑鏂囦欢搴旈€氳繃鑷冲皯鎸傝浇鍛藉悕绌洪棿鍖栫殑鎵嬫瀵瑰鎵樻柟闅愯棌锛屽苟涓斿唴鏍镐細鎷掔粷浠?cgroup 鍛藉悕绌洪棿鍐呴儴瀵瑰懡鍚嶇┖闂存牴涓婄殑鎵€鏈夋枃浠惰繘琛屽啓鍏ワ紝浣?鈥?sys/kernel/cgroup/delegate鈥?涓垪鍑虹殑鏂囦欢锛堝寘鎷?鈥渃group.procs鈥濄€佲€渃group.threads鈥濄€佲€渃group.subtree_control鈥?绛夛級闄ゅ銆?

涓ょ濮旀墭绫诲瀷鐨勬渶缁堢粨鏋滄槸绛変环鐨勩€備竴鏃﹁濮旀墭锛岀敤鎴峰氨鍙互鍦ㄨ鐩綍涓嬫瀯寤哄瓙灞傛銆佹寜鐓ц嚜宸辩殑闇€瑕佺粍缁囧叾涓殑杩涚▼锛屽苟杩涗竴姝ュ垎閰嶄粠鐖惰妭鐐硅幏寰楃殑璧勬簮銆傛墍鏈夎祫婧愭帶鍒跺櫒鐨勯檺棰濆強鍏朵粬璁剧疆閮芥槸鍒嗗眰鐨勶紝鏃犺琚鎵樼殑瀛愬眰娆′腑鍙戠敓浠€涔堬紝閮芥病鏈変换浣曚笢瑗胯兘澶熼€冭劚鐖惰妭鐐规柦鍔犵殑璧勬簮闄愬埗銆?

鐩墠锛宑group 骞舵湭瀵瑰鎵樺瓙灞傛涓殑 cgroup 鏁伴噺鎴栧叾宓屽娣卞害鏂藉姞浠讳綍闄愬埗锛涗笉杩囧皢鏉ュ彲鑳戒細鏄惧紡鍦板姞浠ラ檺鍒躲€?


#### 濮旀墭 containment锛堥殧绂伙級


琚鎵樼殑瀛愬眰娆℃槸鍙?containment 绾︽潫鐨勶紝鍗宠繘绋嬩笉鑳借濮旀墭鏂圭Щ鍏ユ垨绉诲嚭璇ュ瓙灞傛銆?

瀵逛簬濮旀墭缁欎綆鐗规潈鐢ㄦ埛鐨勬儏鍐碉紝杩欓€氳繃瑕佹眰浠ヤ笅鏉′欢鏉ュ疄鐜帮細涓€涓叿鏈夐潪鏍?euid 鐨勮繘绋嬶紝鑻ヨ閫氳繃鍚?鈥渃group.procs鈥?鏂囦欢鍐欏叆 PID 鏉ュ皢鐩爣杩涚▼杩佺Щ杩涙煇涓?cgroup锛屽垯蹇呴』婊¤冻锛?

- 鍐欏叆鑰呭繀椤诲 鈥渃group.procs鈥?鏂囦欢鍏锋湁鍐欐潈闄愩€?

- 鍐欏叆鑰呭繀椤诲婧?cgroup 涓庣洰鐨?cgroup 鐨勫叡鍚岀鍏堢殑 鈥渃group.procs鈥?鏂囦欢鍏锋湁鍐欐潈闄愩€?

涓婅堪涓や釜绾︽潫纭繚锛氳櫧鐒跺鎵樻柟鍙互鍦ㄨ濮旀墭鐨勫瓙灞傛涓嚜鐢辫縼绉昏繘绋嬶紝浣嗗畠鏃犳硶浠庡瓙灞傛涔嬪鎷夊叆杩涚▼锛屼篃鏃犳硶灏嗚繘绋嬫帹鍒板瓙灞傛涔嬪銆?

涓句緥鏉ヨ锛屽亣璁?cgroup C0 鍜?C1 宸茶濮旀墭缁欑敤鎴?U0锛孶0 鍦?C0 涓嬪垱寤轰簡 C00銆丆01锛屽湪 C1 涓嬪垱寤轰簡 C10锛屽涓嬫墍绀?
```

  ~~~~~~~~~~~~~ - C0 - C00
  ~ cgroup    ~      \ C01
  ~ hierarchy ~
  ~~~~~~~~~~~~~ - C1 - C10

```
鍐嶅亣璁?U0 鎯虫妸褰撳墠浣嶄簬 C10 鐨勬煇涓繘绋嬬殑 PID 鍐欏叆 鈥淐00/cgroup.procs鈥濄€俇0 瀵硅鏂囦欢鏈夊啓鏉冮檺锛涚劧鑰岋紝婧?cgroup C10 涓庣洰鐨?cgroup C00 鐨勫叡鍚岀鍏堜綅浜庡鎵樼偣涔嬩笂锛孶0 瀵瑰叾 鈥渃group.procs鈥?鏂囦欢娌℃湁鍐欐潈闄愶紝鍥犳璇ュ啓鍏ュ皢浠?-EACCES 琚嫆缁濄€?

瀵逛簬濮旀墭缁欏懡鍚嶇┖闂寸殑鎯呭喌锛宑ontainment 閫氳繃瑕佹眰婧?cgroup 鍜岀洰鐨?cgroup 閮借兘浠庡皾璇曡縼绉荤殑杩涚▼鎵€鍦ㄧ殑鍛藉悕绌洪棿鍒拌揪鏉ュ疄鐜般€傚鏋滃叾涓换浣曚竴涓笉鍙揪锛屽垯璇ヨ縼绉讳互 -ENOENT 琚嫆缁濄€?


### 鍑嗗垯


#### 缁勭粐涓€娆″苟鎺у埗


璺?cgroup 杩佺Щ杩涚▼鏄竴涓浉瀵规槀璐电殑鎿嶄綔锛岃€屽唴瀛樼瓑鏈夌姸鎬佺殑璧勬簮涓嶄細闅忚繘绋嬩竴璧风Щ鍔ㄣ€傝繖鏄竴涓樉寮忕殑璁捐鍐崇瓥锛屽洜涓哄湪鍚屾浠ｄ环鏂归潰锛岃縼绉讳笌鍚勭鐑矾寰勪箣闂村線寰€瀛樺湪鍥烘湁鐨勬潈琛°€?

鍥犳锛屼笉榧撳姳灏嗛绻佽法 cgroup 杩佺Щ杩涚▼浣滀负涓€绉嶆柦鍔犱笉鍚岃祫婧愰檺鍒剁殑鎵嬫銆傚伐浣滆礋杞藉簲鍦ㄥ惎鍔ㄦ椂鏍规嵁绯荤粺鐨勯€昏緫涓庤祫婧愮粨鏋勪竴娆℃€у垎閰嶅埌鏌愪釜 cgroup銆傚彲浠ラ€氳繃鎺ュ彛鏂囦欢鏇存敼鎺у埗鍣ㄩ厤缃潵瀵硅祫婧愬垎閰嶈繘琛屽姩鎬佽皟鏁淬€?


#### 閬垮厤鍚嶇О鍐茬獊


涓€涓?cgroup 涓庡叾瀛?cgroup 鐨勬帴鍙ｆ枃浠跺崰鎹悓涓€鐩綍锛屽洜姝ゆ湁鍙兘鍒涘缓鍑轰笌鎺ュ彛鏂囦欢鍐茬獊鐨勫瓙 cgroup銆?

鎵€鏈?cgroup 鏍稿績鎺ュ彛鏂囦欢閮戒互 鈥渃group.鈥?涓哄墠缂€锛屾瘡涓帶鍒跺櫒鐨勬帴鍙ｆ枃浠堕兘浠ユ帶鍒跺櫒鍚嶅姞涓€涓偣浣滀负鍓嶇紑銆傛帶鍒跺櫒鐨勫悕绉扮敱灏忓啓瀛楁瘝鍜?鈥淿鈥?缁勬垚锛屼絾缁濅笉浠?鈥淿鈥?寮€澶达紝鍥犳瀹冨彲浠ヤ綔涓哄墠缂€瀛楃鐢ㄤ簬閬垮厤鍐茬獊銆傛澶栵紝鎺ュ彛鏂囦欢鍚嶄笉浼氫互甯哥敤浜庡宸ヤ綔璐熻浇鍒嗙被鐨勬湳璇紑澶存垨缁撳熬锛屼緥濡?job銆乻ervice銆乻lice銆乽nit 鎴?workload銆?

cgroup 涓嶅仛浠讳綍浜嬫儏鏉ラ槻姝㈠悕绉板啿绐侊紝閬垮厤鍐茬獊鏄敤鎴风殑璐ｄ换銆?


## 璧勬簮鍒嗛厤妯″瀷


cgroup 鎺у埗鍣ㄦ牴鎹祫婧愮被鍨嬩笌棰勬湡浣跨敤鍦烘櫙瀹炵幇浜嗚嫢骞茬璧勬簮鍒嗛厤鏂规銆傛湰鑺傛弿杩版鍦ㄤ娇鐢ㄧ殑涓昏鏂规鍙婂叾棰勬湡琛屼负銆?


### 鏉冮噸


鐖惰妭鐐圭殑璧勬簮閫氳繃鎶婃墍鏈夋椿鍔ㄥ瓙鑺傜偣鐨勬潈閲嶇浉鍔犮€佸苟鎸夊悇鑷潈閲嶅崰鏉冮噸涔嬪拰鐨勬瘮渚嬭繘琛屽垎閰嶃€傜敱浜庡彧鏈夊綋鍓嶈兘鍒╃敤璇ヨ祫婧愮殑瀛愯妭鐐规墠鍙備笌鍒嗛厤锛岃繖鏄竴绉嶅伐浣滀繚鎸侊紙work-conserving锛夌殑鏂瑰紡銆傜敱浜庤繖绉嶅姩鎬佺壒鎬э紝璇ユā鍨嬮€氬父鐢ㄤ簬鏃犵姸鎬佺殑璧勬簮銆?

鎵€鏈夋潈閲嶉兘鍦?[1, 10000] 鑼冨洿鍐咃紝榛樿鍊间负 100銆傝繖鍏佽鍦ㄤ袱涓柟鍚戜笂浠ヨ冻澶熺簿缁嗙殑绮掑害杩涜瀵圭О鐨勪箻娉曞亸缃紝鍚屾椂淇濇寔鍦ㄧ洿瑙傜殑鑼冨洿鍐呫€?

鍙鏉冮噸鍦ㄨ寖鍥村唴锛屾墍鏈夐厤缃粍鍚堥兘鏄湁鏁堢殑锛屾病鏈夌悊鐢辨嫆缁濋厤缃彉鏇存垨杩涚▼杩佺Щ銆?

鈥渃pu.weight鈥?鎸夋瘮渚嬪皢 CPU 鍛ㄦ湡鍒嗛厤缁欐椿鍔ㄥ瓙鑺傜偣锛屽氨鏄繖绫荤殑涓€涓緥瀛愩€?


### 闄愰


瀛愯妭鐐规渶澶氬彧鑳芥秷璐归厤缃噺鐨勮祫婧愩€傞檺棰濆彲浠ヨ杩囧害鎵胯锛坥ver-committed锛夆€斺€斿瓙鑺傜偣闄愰涔嬪拰鍙互瓒呰繃鐖惰妭鐐瑰彲鐢ㄧ殑璧勬簮閲忋€?

闄愰鑼冨洿鏄?[0, max]锛岄粯璁ゅ€间负 鈥渕ax鈥濓紝鍗崇┖鎿嶄綔锛坣oop锛夈€?

鐢变簬闄愰鍙互琚繃搴︽壙璇猴紝鎵€鏈夐厤缃粍鍚堥兘鏄湁鏁堢殑锛屾病鏈夌悊鐢辨嫆缁濋厤缃彉鏇存垨杩涚▼杩佺Щ銆?

鈥渋o.max鈥?闄愬埗涓€涓?cgroup 鍦ㄦ煇涓€ IO 璁惧涓婂彲娑堣垂鐨勬渶澶?BPS 鍜?鎴?IOPS锛屽氨鏄繖绫荤殑涓€涓緥瀛愩€?


### 淇濇姢


鍙鏌?cgroup 鐨勬墍鏈夌鍏堢殑浣跨敤閲忛兘浣庝簬鍏跺彈淇濇姢绾у埆锛岃 cgroup 灏变細寰楀埌閰嶇疆閲忚祫婧愮殑淇濇姢銆備繚鎶ゅ彲浠ユ槸纭繚璇侊紝涔熷彲浠ユ槸灏藉姏鑰屼负鐨勮蒋杈圭晫銆備繚鎶や篃鍙互琚繃搴︽壙璇猴紝杩欑鎯呭喌涓嬪瓙鑺傜偣涔嬮棿鍙湁鐖惰妭鐐瑰彲鐢ㄩ噺鑼冨洿鍐呯殑涓€閮ㄥ垎鍙楀埌淇濇姢銆?

淇濇姢鑼冨洿鏄?[0, max]锛岄粯璁ゅ€间负 0锛屽嵆绌烘搷浣滐紙noop锛夈€?

鐢变簬淇濇姢鍙互琚繃搴︽壙璇猴紝鎵€鏈夐厤缃粍鍚堥兘鏄湁鏁堢殑锛屾病鏈夌悊鐢辨嫆缁濋厤缃彉鏇存垨杩涚▼杩佺Щ銆?

鈥渕emory.low鈥?瀹炵幇浜嗗敖鍔涜€屼负鐨勫唴瀛樹繚鎶わ紝灏辨槸杩欑被鐨勪竴涓緥瀛愩€?


### 鍒嗛厤


涓€涓?cgroup 琚嫭鍗犲湴鍒嗛厤鏌愭湁闄愯祫婧愮殑涓€瀹氭暟閲忋€傚垎閰嶄笉鑳借杩囧害鎵胯鈥斺€斿瓙鑺傜偣鍒嗛厤涔嬪拰涓嶈兘瓒呰繃鐖惰妭鐐瑰彲鐢ㄧ殑璧勬簮閲忋€?

鍒嗛厤鑼冨洿鏄?[0, max]锛岄粯璁ゅ€间负 0锛屽嵆鏃犺祫婧愩€?

鐢变簬鍒嗛厤涓嶈兘琚繃搴︽壙璇猴紝鏌愪簺閰嶇疆缁勫悎鏄棤鏁堢殑锛屽簲琚嫆缁濄€傛澶栵紝濡傛灉璇ヨ祫婧愭槸杩涚▼鎵ц鎵€蹇呴渶鐨勶紝杩涚▼杩佺Щ鍙兘浼氳鎷掔粷銆?


## 鎺ュ彛鏂囦欢


### 鏍煎紡


鎵€鏈夋帴鍙ｆ枃浠跺湪鍙兘鐨勬儏鍐典笅閮藉簲閲囩敤浠ヤ笅鏍煎紡涔嬩竴
```

  New-line separated values
  (when only one value can be written at once)

	VAL0\n
	VAL1\n
	...

  Space separated values
  (when read-only or multiple values can be written at once)

	VAL0 VAL1 ...\n

  Flat keyed

	KEY0 VAL0\n
	KEY1 VAL1\n
	...

  Nested keyed

	KEY0 SUB_KEY0=VAL00 SUB_KEY1=VAL01...
	KEY1 SUB_KEY0=VAL10 SUB_KEY1=VAL11...
	...

```
瀵逛簬鍙啓鏂囦欢锛屽啓鍏ョ殑鏍煎紡閫氬父搴斾笌璇诲彇鏍煎紡鍖归厤锛涗笉杩囷紝鎺у埗鍣ㄥ彲鑳藉厑璁哥渷鐣ュ悗闈㈢殑瀛楁锛屾垨閽堝鏈€甯歌鐨勪娇鐢ㄥ満鏅疄鐜板彈闄愮殑蹇嵎鏂瑰紡銆?

瀵逛簬鎵佸钩閿€硷紙flat keyed锛夊拰宓屽閿€硷紙nested keyed锛夋枃浠讹紝姣忔鍙兘鍐欏叆鍗曚釜閿搴旂殑鍊笺€傚浜庡祵濂楅敭鍊兼枃浠讹紝瀛愰敭瀵瑰彲浠ヤ互浠绘剰椤哄簭鎸囧畾锛屼笖涓嶅繀鎸囧畾鎵€鏈夐敭鍊煎銆?


### 绾﹀畾


- 鍗曚竴鐗规€х殑璁剧疆搴斿綋鍖呭惈鍦ㄤ竴涓崟涓€鏂囦欢涓€?

- 鏍?cgroup 搴斾笉鍙楄祫婧愭帶鍒剁害鏉燂紝鍥犳涓嶅簲鏈夎祫婧愭帶鍒舵帴鍙ｆ枃浠躲€?

- 榛樿鏃堕棿鍗曚綅鏄井绉掋€傚鏋滀娇鐢ㄤ簡涓嶅悓鐨勫崟浣嶏紝蹇呴』甯︽樉寮忕殑鍗曚綅鍚庣紑銆?

- 浠モ€滄瘡鍗曚綅涓墍鍗犳瘮渚嬶紙parts-per锛夆€濊〃绀虹殑閲忓簲浣跨敤甯︽湁鑷冲皯涓や綅灏忔暟閮ㄥ垎鐨勭櫨鍒嗘瘮灏忔暟鈥斺€斾緥濡?13.40銆?

- 濡傛灉鏌愪釜鎺у埗鍣ㄥ疄鐜颁簡鍩轰簬鏉冮噸鐨勮祫婧愬垎閰嶏紝鍏舵帴鍙ｆ枃浠跺簲鍛藉悕涓?鈥渨eight鈥濓紝鑼冨洿 [1, 10000]锛岄粯璁ゅ€?100銆傝繖浜涘彇鍊肩殑閫夋嫨鏄负浜嗗湪涓や釜鏂瑰悜涓婇兘鑳芥彁渚涜冻澶熶笖瀵圭О鐨勫亸缃紝鍚屾椂淇濇寔鐩磋锛堥粯璁ゆ槸 100%锛夈€?

- 濡傛灉鏌愪釜鎺у埗鍣ㄥ疄鐜颁簡缁濆鐨勮祫婧愪繚璇佸拰/鎴栭檺鍒讹紝鎺ュ彛鏂囦欢搴斿垎鍒懡鍚嶄负 鈥渕in鈥?鍜?鈥渕ax鈥濄€傚鏋滄煇涓帶鍒跺櫒瀹炵幇浜嗗敖鍔涜€屼负鐨勮祫婧愪繚璇佸拰/鎴栭檺鍒讹紝鎺ュ彛鏂囦欢搴斿垎鍒懡鍚嶄负 鈥渓ow鈥?鍜?鈥渉igh鈥濄€?

  鍦ㄤ笂杩板洓涓帶鍒舵枃浠朵腑锛岀壒娈婃爣璁?鈥渕ax鈥?搴旂敤鏉ヨ〃绀鸿鍙栧拰鍐欏叆鏃剁殑鍚戜笂鏃犵┓澶с€?

- 濡傛灉涓€涓缃叿鏈夊彲閰嶇疆鐨勯粯璁ゅ€间互鍙婃寜閿殑鐗瑰畾瑕嗙洊鍊硷紝榛樿鏉＄洰搴斾互 鈥渄efault鈥?涓洪敭锛屽苟浣滀负鏂囦欢涓殑绗竴涓潯鐩嚭鐜般€?

  榛樿鍊煎彲浠ラ€氳繃鍐欏叆 鈥渄efault $VAL鈥?鎴?鈥?VAL鈥?鏉ユ洿鏂般€?

  鍐欏叆浠ユ洿鏂版煇涓壒瀹氳鐩栧€兼椂锛屽彲浠ヤ娇鐢?鈥渄efault鈥?浣滀负鍊硷紝琛ㄧず绉婚櫎璇ヨ鐩栥€傚€间负 鈥渄efault鈥?鐨勮鐩栨潯鐩湪璇诲彇鏃朵笉寰楀嚭鐜般€?

  渚嬪锛屼竴涓互涓?娆¤澶囧彿锛坢ajor:minor锛変负閿殑璁剧疆
```

    # cat cgroup-example-interface-file
    default 150
    8:0 300

  The default value can be updated by::

    # echo 125 > cgroup-example-interface-file

  or::

    # echo "default 125" > cgroup-example-interface-file

  An override can be set by::

    # echo "8:16 170" > cgroup-example-interface-file

  and cleared by::

    # echo "8:0 default" > cgroup-example-interface-file
    # cat cgroup-example-interface-file
    default 125
    8:16 170

```
- 瀵逛簬棰戠巼涓嶅お楂樼殑浜嬩欢锛屽簲鍒涘缓涓€涓帴鍙ｆ枃浠?鈥渆vents鈥濓紝鍏朵腑鍒楀嚭浜嬩欢鐨勯敭鍊煎銆傛瘡褰撳彂鐢熷彲閫氱煡鐨勪簨浠舵椂锛屽簲鍦ㄨ鏂囦欢涓婄敓鎴愭枃浠朵慨鏀逛簨浠躲€?


### 鏍稿績鎺ュ彛鏂囦欢


鎵€鏈?cgroup 鏍稿績鏂囦欢閮戒互 鈥渃group.鈥?涓哄墠缂€銆?

  cgroup.type
	A read-write single value file which exists on non-root
	cgroups.

	When read, it indicates the current type of the cgroup, which
	can be one of the following values.

 - "domain" : 涓€涓櫘閫氫笖鏈夋晥鐨勫煙 cgroup銆?

 - "domain threaded" : 浣滀负涓€涓嚎绋嬪寲瀛愭爲涔嬫牴鐨勭嚎绋嬪寲鍩?cgroup銆?

 - "domain invalid" : 澶勪簬鏃犳晥鐘舵€佺殑 cgroup銆?
	  瀹冧笉鑳借濉厖锛屼篃涓嶈兘鍚敤鎺у埗鍣ㄣ€傚畠鍙兘
	  琚厑璁告垚涓虹嚎绋嬪寲 cgroup銆?

 - "threaded" : 浣滀负绾跨▼鍖栧瓙鏍戞垚鍛樼殑绾跨▼鍖?cgroup銆?

	A cgroup can be turned into a threaded cgroup by writing
	"threaded" to this file.

  cgroup.procs
	A read-write new-line separated values file which exists on
	all cgroups.

	When read, it lists the PIDs of all processes which belong to
	the cgroup one-per-line.  The PIDs are not ordered and the
	same PID may show up more than once if the process got moved
	to another cgroup and then back or the PID got recycled while
	reading.

	A PID can be written to migrate the process associated with
	the PID to the cgroup.  The writer should match all of the
	following conditions.

 - It must have write access to the "cgroup.procs" file.

 - It must have write access to the "cgroup.procs" file of the
	  common ancestor of the source and destination cgroups.

	When delegating a sub-hierarchy, write access to this file
	should be granted along with the containing directory.

	In a threaded cgroup, reading this file fails with EOPNOTSUPP
	as all the processes belong to the thread root.  Writing is
	supported and moves every thread of the process to the cgroup.

  cgroup.threads
	A read-write new-line separated values file which exists on
	all cgroups.

	When read, it lists the TIDs of all threads which belong to
	the cgroup one-per-line.  The TIDs are not ordered and the
	same TID may show up more than once if the thread got moved to
	another cgroup and then back or the TID got recycled while
	reading.

	A TID can be written to migrate the thread associated with the
	TID to the cgroup.  The writer should match all of the
	following conditions.

 - It must have write access to the "cgroup.threads" file.

 - The cgroup that the thread is currently in must be in the
          same resource domain as the destination cgroup.

 - It must have write access to the "cgroup.procs" file of the
	  common ancestor of the source and destination cgroups.

	When delegating a sub-hierarchy, write access to this file
	should be granted along with the containing directory.

  cgroup.controllers
	A read-only space separated values file which exists on all
	cgroups.

	It shows space separated list of all controllers available to
	the cgroup.  The controllers are not ordered.

  cgroup.subtree_control
	A read-write space separated values file which exists on all
	cgroups.  Starts out empty.

	When read, it shows space separated list of the controllers
	which are enabled to control resource distribution from the
	cgroup to its children.

	Space separated list of controllers prefixed with '+' or '-'
	can be written to enable or disable controllers.  A controller
	name prefixed with '+' enables the controller and '-'
	disables.  If a controller appears more than once on the list,
	the last one is effective.  When multiple enable and disable
	operations are specified, either all succeed or all fail.

  cgroup.events
	A read-only flat-keyed file which exists on non-root cgroups.
	The following entries are defined.  Unless specified
	otherwise, a value change in this file generates a file
	modified event.

	  populated
		濡傛灉璇?cgroup 鎴栧叾鍚庝唬涓寘鍚换浣曟椿鍔ㄨ繘绋嬪垯涓?1锛涘惁鍒欎负 0銆?
	  frozen
		濡傛灉璇?cgroup 琚喕缁撳垯涓?1锛涘惁鍒欎负 0銆?

  cgroup.max.descendants
	A read-write single value files.  The default is "max".

	鍏佽鐨勬渶澶у悗浠?cgroup 鏁伴噺銆?
	濡傛灉瀹為檯鐨勫悗浠ｆ暟閲忕瓑浜庢垨澶т簬姝ゅ€硷紝
	鍦ㄨ灞傛涓垱寤烘柊 cgroup 鐨勫皾璇曞皢澶辫触銆?

  cgroup.max.depth
	A read-write single value files.  The default is "max".

	褰撳墠 cgroup 涔嬩笅鍏佽鐨勬渶澶у悗浠ｆ繁搴︺€?
	濡傛灉瀹為檯鐨勫悗浠ｆ繁搴︾瓑浜庢垨澶т簬姝ゅ€硷紝
	鍒涘缓鏂板瓙 cgroup 鐨勫皾璇曞皢澶辫触銆?

  cgroup.stat
	A read-only flat-keyed file with the following entries:

	  nr_descendants
		鍙鍚庝唬 cgroup 鐨勬€绘暟銆?

	  nr_dying_descendants
		澶勪簬娑堜骸锛坉ying锛夌姸鎬佺殑 descendant cgroup 鎬绘暟銆備竴涓?cgroup 鍦ㄨ鐢ㄦ埛鍒犻櫎鍚庤繘鍏ユ秷浜＄姸鎬併€傝 cgroup 鍦ㄥ畬鍏ㄨ閿€姣佷箣鍓嶏紝浼氬湪娑堜骸鐘舵€佷繚鎸佷竴娈垫湭瀹氫箟鐨勬椂闂达紙鍙兘鍙栧喅浜庣郴缁熻礋杞斤級銆?

		A process can't enter a dying cgroup under any circumstances,
		a dying cgroup can't revive.

		A dying cgroup can consume system resources not exceeding
		limits, which were active at the moment of cgroup deletion.

	  nr_subsys_<cgroup_subsys>
		褰撳墠 cgroup 鍙婂叾涔嬩笅澶勪簬娲诲姩鐘舵€佺殑 cgroup 瀛愮郴缁燂紙渚嬪 memory cgroup锛夋€绘暟銆?

	  nr_dying_subsys_<cgroup_subsys>
		褰撳墠 cgroup 鍙婂叾涔嬩笅澶勪簬娑堜骸鐘舵€佺殑 cgroup 瀛愮郴缁燂紙渚嬪 memory cgroup锛夋€绘暟銆?

  cgroup.stat.local
	A read-only flat-keyed file which exists in non-root cgroups.
	The following entry is defined:

	  frozen_usec
		Cumulative time that this cgroup has spent between freezing and
		thawing, regardless of whether by self or ancestor groups.
		NB: (not) reaching "frozen" state is not accounted here.

		Using the following ASCII representation of a cgroup's freezer
```

			       1    _____
			frozen 0 __/     \__
			          ab    cd

		the duration being measured is the span between a and c.

  cgroup.freeze
	A read-write single value file which exists on non-root cgroups.
	Allowed values are "0" and "1". The default is "0".

	Writing "1" to the file causes freezing of the cgroup and all
	descendant cgroups. This means that all belonging processes will
	be stopped and will not run until the cgroup will be explicitly
	unfrozen. Freezing of the cgroup may take some time; when this action
	is completed, the "frozen" value in the cgroup.events control file
	will be updated to "1" and the corresponding notification will be
	issued.

	A cgroup can be frozen either by its own settings, or by settings
	of any ancestor cgroups. If any of ancestor cgroups is frozen, the
	cgroup will remain frozen.

	Processes in the frozen cgroup can be killed by a fatal signal.
	They also can enter and leave a frozen cgroup: either by an explicit
	move by a user, or if freezing of the cgroup races with fork().
	If a process is moved to a frozen cgroup, it stops. If a process is
	moved out of a frozen cgroup, it becomes running.

	Frozen status of a cgroup doesn't affect any cgroup tree operations:
	it's possible to delete a frozen (and empty) cgroup, as well as
	create new sub-cgroups.

  cgroup.kill
	A write-only single value file which exists in non-root cgroups.
	The only allowed value is "1".

	Writing "1" to the file causes the cgroup and all descendant cgroups to
	be killed. This means that all processes located in the affected cgroup
	tree will be killed via SIGKILL.

	Killing a cgroup tree will deal with concurrent forks appropriately and
	is protected against migrations.

	In a threaded cgroup, writing this file fails with EOPNOTSUPP as
	killing cgroups is a process directed operation, i.e. it affects
	the whole thread-group.

  cgroup.pressure
	A read-write single value file that allowed values are "0" and "1".
	The default is "1".

	Writing "0" to the file will disable the cgroup PSI accounting.
	Writing "1" to the file will re-enable the cgroup PSI accounting.

	This control attribute is not hierarchical, so disable or enable PSI
	accounting in a cgroup does not affect PSI accounting in descendants
	and doesn't need pass enablement via ancestors from root.

	The reason this control attribute exists is that PSI accounts stalls for
	each cgroup separately and aggregates it at each level of the hierarchy.
	This may cause non-negligible overhead for some workloads when under
	deep level of the hierarchy, in which case this control attribute can
	be used to disable PSI accounting in the non-leaf cgroups.

  irq.pressure
	A read-write nested-keyed file.

	Shows pressure stall information for IRQ/SOFTIRQ. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.

```
## 鎺у埗鍣?



### CPU


鈥渃pu鈥?鎺у埗鍣ㄨ皟鑺?CPU 鍛ㄦ湡鐨勫垎閰嶃€傝鎺у埗鍣ㄥ鏅€氳皟搴︾瓥鐣ュ疄鐜颁簡鏉冮噸涓庣粷瀵瑰甫瀹介檺鍒舵ā鍨嬶紝瀵瑰疄鏃惰皟搴︾瓥鐣ュ疄鐜颁簡缁濆甯﹀鍒嗛厤妯″瀷銆?

鍦ㄤ笂杩版墍鏈夋ā鍨嬩腑锛屽懆鏈熷垎閰嶄粎鍩轰簬鏃堕棿瀹氫箟锛屽苟涓嶈€冭檻浠诲姟琚墽琛岀殑棰戠巼銆傚彲閫夌殑鍒╃敤鐜囬挸鍒讹紙utilization clamping锛夋敮鎸佸厑璁稿悜 schedutil cpufreq 璋冮€熷櫒鏆楃ず鏌愪釜 CPU 搴斿缁堟彁渚涚殑鏈€灏忔湡鏈涢鐜囷紝浠ュ強涓嶅簲瓒呰繃鐨勬渶澶ф湡鏈涢鐜囥€?

璀﹀憡锛歝group2 鐨?cpu 鎺у埗鍣ㄥ皻涓嶆敮鎸佸瀹炴椂杩涚▼鐨勶紙甯﹀锛夋帶鍒躲€傚浜庣紪璇戞椂鍚敤浜?CONFIG_RT_GROUP_SCHED 閫夐」浠ユ敮鎸佸疄鏃惰繘绋嬪垎缁勮皟搴︾殑鍐呮牳锛屽彧鏈夊綋鎵€鏈?RT 杩涚▼閮戒綅浜庢牴 cgroup 鏃讹紝鎵嶈兘鍚敤 cpu 鎺у埗鍣ㄣ€傝娉ㄦ剰锛岀郴缁熺鐞嗚蒋浠跺彲鑳藉湪绯荤粺鍚姩杩囩▼涓凡缁忓皢 RT 杩涚▼鏀惧叆浜嗛潪鏍?cgroup锛屽湪鍚敤 CONFIG_RT_GROUP_SCHED 鐨勫唴鏍镐笂鍚敤 cpu 鎺у埗鍣ㄤ箣鍓嶏紝鍙兘闇€瑕佸厛灏嗚繖浜涜繘绋嬬Щ鍔ㄥ埌鏍?cgroup銆?

鍦ㄧ鐢?CONFIG_RT_GROUP_SCHED 鐨勬儏鍐典笅锛屾闄愬埗涓嶉€傜敤锛岄儴鍒嗘帴鍙ｆ枃浠惰涔堝奖鍝嶅疄鏃惰繘绋嬶紝瑕佷箞瀵瑰畠浠璐︺€傝瑙佷笅涓€鑺傘€傚彧鏈?cpu 鎺у埗鍣ㄥ彈 CONFIG_RT_GROUP_SCHED 褰卞搷銆傚叾浠栨帶鍒跺櫒鏃犺 CONFIG_RT_GROUP_SCHED 濡備綍锛岄兘鍙敤浜庡疄鏃惰繘绋嬬殑璧勬簮鎺у埗銆?


#### CPU 鎺ュ彛鏂囦欢


杩涚▼涓?cpu 鎺у埗鍣ㄧ殑浜や簰鍙栧喅浜庡叾璋冨害绛栫暐涓庡簳灞傝皟搴﹀櫒銆備粠 cpu 鎺у埗鍣ㄧ殑瑙掑害鐪嬶紝杩涚▼鍙垎绫诲涓嬶細

- 澶勪簬鍏钩绫伙紙fair-class锛夎皟搴﹀櫒涓嬬殑杩涚▼
- 浣跨敤甯︽湁 `cgroup_set_weight` 鍥炶皟鐨?BPF 璋冨害鍣ㄤ笅鐨勮繘绋?
- 鍏朵粬涓€鍒囷細`SCHED_{FIFO,RR,DEADLINE}` 浠ュ強浣跨敤涓嶅甫 `cgroup_set_weight` 鍥炶皟鐨?BPF 璋冨害鍣ㄤ笅鐨勮繘绋?

鍏充簬杩涚▼浣曟椂澶勪簬鍏钩绫昏皟搴﹀櫒鎴?BPF 璋冨害鍣ㄤ箣涓嬶紝璇峰弬闃?Documentation/scheduler/sched-ext.rst <sched-ext>銆?

瀵逛簬浠ヤ笅姣忎釜鎺ュ彛鏂囦欢锛岄兘浼氬紩鐢ㄤ笂杩板垎绫汇€傛墍鏈夋椂闂存椂闀垮潎浠ュ井绉掍负鍗曚綅銆?

  cpu.stat
	A read-only flat-keyed file.
	This file exists whether the controller is enabled or not.

	It always reports the following three stats, which account for all the
	processes in the cgroup:

 - usage_usec
 - user_usec
 - system_usec

	and the following five when the controller is enabled, which account for
	only the processes under the fair-class scheduler:

 - nr_periods
 - nr_throttled
 - throttled_usec
 - nr_bursts
 - burst_usec

  cpu.weight
	A read-write single value file which exists on non-root
	cgroups.  The default is "100".

	For non idle groups (cpu.idle = 0), the weight is in the
	range [1, 10000].

	If the cgroup has been configured to be SCHED_IDLE (cpu.idle = 1),
	then the weight will show as a 0.

	This file affects only processes under the fair-class scheduler and a BPF
	scheduler with the `cgroup_set_weight` callback depending on what the
	callback actually does.

  cpu.weight.nice
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	The nice value is in the range [-20, 19].

	This interface file is an alternative interface for
	"cpu.weight" and allows reading and setting weight using the
	same values used by nice(2).  Because the range is smaller and
	granularity is coarser for the nice values, the read value is
	the closest approximation of the current weight.

	This file affects only processes under the fair-class scheduler and a BPF
	scheduler with the `cgroup_set_weight` callback depending on what the
	callback actually does.

  cpu.max
	A read-write two value file which exists on non-root cgroups.
	The default is "max 100000".

```

	  $MAX $PERIOD

	which indicates that the group may consume up to $MAX in each
	$PERIOD duration.  "max" for $MAX indicates no limit.  If only
	one number is written, $MAX is updated.

	This file affects only processes under the fair-class scheduler.

  cpu.max.burst
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	The burst in the range [0, $MAX].

	This file affects only processes under the fair-class scheduler.

  cpu.pressure
	A read-write nested-keyed file.

	Shows pressure stall information for CPU. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.

	This file accounts for all the processes in the cgroup.

  cpu.uclamp.min
	A read-write single value file which exists on non-root cgroups.
	The default is "0", i.e. no utilization boosting.

	The requested minimum utilization (protection) as a percentage
	rational number, e.g. 12.34 for 12.34%.

	This interface allows reading and setting minimum utilization clamp
	values similar to the sched_setattr(2). This minimum utilization
	value is used to clamp the task specific minimum utilization clamp,
	including those of realtime processes.

	The requested minimum utilization (protection) is always capped by
	the current value for the maximum utilization (limit), i.e.
	`cpu.uclamp.max`.

	This file affects all the processes in the cgroup.

  cpu.uclamp.max
	A read-write single value file which exists on non-root cgroups.
	The default is "max". i.e. no utilization capping

	The requested maximum utilization (limit) as a percentage rational
	number, e.g. 98.76 for 98.76%.

	This interface allows reading and setting maximum utilization clamp
	values similar to the sched_setattr(2). This maximum utilization
	value is used to clamp the task specific maximum utilization clamp,
	including those of realtime processes.

	This file affects all the processes in the cgroup.

  cpu.idle
	A read-write single value file which exists on non-root cgroups.
	The default is 0.

	This is the cgroup analog of the per-task SCHED_IDLE sched policy.
	Setting this value to a 1 will make the scheduling policy of the
	cgroup SCHED_IDLE. The threads inside the cgroup will retain their
	own relative priorities, but the cgroup itself will be treated as
	very low priority relative to its peers.

	This file affects only processes under the fair-class scheduler.

```
### 鍐呭瓨


鈥渕emory鈥?鎺у埗鍣ㄨ皟鑺傚唴瀛樼殑鍒嗛厤銆傚唴瀛樻槸鏈夌姸鎬佺殑锛屽悓鏃跺疄鐜颁簡闄愬埗涓庝繚鎶ゆā鍨嬨€傜敱浜庡唴瀛樹娇鐢ㄤ笌鍥炴敹鍘嬪姏涔嬮棿閿欑患澶嶆潅鐨勫叧绯伙紝浠ュ強鍐呭瓨鐨勬湁鐘舵€佺壒鎬э紝鍏跺垎閰嶆ā鍨嬬浉瀵瑰鏉傘€?

铏界劧骞堕潪瀹屽叏婊存按涓嶆紡锛屼絾涓€涓粰瀹?cgroup 鐨勬墍鏈変富瑕佸唴瀛樼敤閲忛兘鍙楀埌璺熻釜锛屼粠鑰屽彲浠ュ湪鍚堢悊绋嬪害涓婂鎬诲唴瀛樻秷鑰楄繘琛岃璐︿笌鎺у埗銆傜洰鍓嶏紝璺熻釜浠ヤ笅鍑犵被鍐呭瓨鐢ㄩ噺銆?

- 鐢ㄦ埛绌洪棿鍐呭瓨鈥斺€旈〉缂撳瓨涓庡尶鍚嶅唴瀛樸€?

- 鍐呮牳鏁版嵁缁撴瀯锛屼緥濡?dentries 涓?inodes銆?

- TCP 濂楁帴瀛楃紦鍐插尯銆?

涓婅堪鍒楄〃鏈潵鍙兘浼氭墿灞曚互鑾峰緱鏇村ソ鐨勮鐩栧害銆?


#### 鍐呭瓨鎺ュ彛鏂囦欢


鎵€鏈夊唴瀛橀噺閮戒互瀛楄妭涓哄崟浣嶃€傚鏋滃啓鍏ョ殑鍊兼湭瀵归綈鍒?PAGE_SIZE锛岃鍥炴椂璇ュ€煎彲鑳戒細琚悜涓婂彇鏁村埌鏈€鎺ヨ繎鐨?PAGE_SIZE 鍊嶆暟銆?

  memory.current
	A read-only single value file which exists on non-root
	cgroups.

	璇?cgroup 鍙婂叾鍚庝唬褰撳墠姝ｅ湪浣跨敤鐨勬€诲唴瀛橀噺銆?

  memory.min
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	Hard memory protection.  If the memory usage of a cgroup
	is within its effective min boundary, the cgroup's memory
	won't be reclaimed under any conditions. If there is no
	unprotected reclaimable memory available, OOM killer
	is invoked. Above the effective min boundary (or
	effective low boundary if it is higher), pages are reclaimed
	proportionally to the overage, reducing reclaim pressure for
	smaller overages.

	Effective min boundary is limited by memory.min values of
	ancestor cgroups. If there is memory.min overcommitment
	(child cgroup or cgroups are requiring more protected memory
	than parent will allow), then each child cgroup will get
	the part of parent's protection proportional to its
	actual memory usage below memory.min.

	Putting more memory than generally available under this
	protection is discouraged and may lead to constant OOMs.

  memory.low
	A read-write single value file which exists on non-root
	cgroups.  The default is "0".

	Best-effort memory protection.  If the memory usage of a
	cgroup is within its effective low boundary, the cgroup's
	memory won't be reclaimed unless there is no reclaimable
	memory available in unprotected cgroups.
	Above the effective low	boundary (or 
	effective min boundary if it is higher), pages are reclaimed
	proportionally to the overage, reducing reclaim pressure for
	smaller overages.

	Effective low boundary is limited by memory.low values of
	ancestor cgroups. If there is memory.low overcommitment
	(child cgroup or cgroups are requiring more protected memory
	than parent will allow), then each child cgroup will get
	the part of parent's protection proportional to its
	actual memory usage below memory.low.

	Putting more memory than generally available under this
	protection is discouraged.

  memory.high
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Memory usage throttle limit.  If a cgroup's usage goes
	over the high boundary, the processes of the cgroup are
	throttled and put under heavy reclaim pressure.

	Going over the high limit never invokes the OOM killer and
	under extreme conditions the limit may be breached. The high
	limit should be used in scenarios where an external process
	monitors the limited cgroup to alleviate heavy reclaim
	pressure.

	If memory.high is opened with O_NONBLOCK then the synchronous
	reclaim is bypassed. This is useful for admin processes that
	need to dynamically adjust the job's memory limits without
	expending their own CPU resources on memory reclamation. The
	job will trigger the reclaim and/or get throttled on its
	next charge request.

	Please note that with O_NONBLOCK, there is a chance that the
	target memory cgroup may take indefinite amount of time to
	reduce usage below the limit due to delayed charge request or
	busy-hitting its memory to slow down reclaim.

  memory.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Memory usage hard limit.  This is the main mechanism to limit
	memory usage of a cgroup.  If a cgroup's memory usage reaches
	this limit and can't be reduced, the OOM killer is invoked in
	the cgroup. Under certain circumstances, the usage may go
	over the limit temporarily.

	In default configuration regular 0-order allocations always
	succeed unless OOM killer chooses current task as a victim.

	Some kinds of allocations don't invoke the OOM killer.
	Caller could retry them differently, return into userspace
	as -ENOMEM or silently ignore in cases like disk readahead.

	If memory.max is opened with O_NONBLOCK, then the synchronous
	reclaim and oom-kill are bypassed. This is useful for admin
	processes that need to dynamically adjust the job's memory limits
	without expending their own CPU resources on memory reclamation.
	The job will trigger the reclaim and/or oom-kill on its next
	charge request.

	Please note that with O_NONBLOCK, there is a chance that the
	target memory cgroup may take indefinite amount of time to
	reduce usage below the limit due to delayed charge request or
	busy-hitting its memory to slow down reclaim.

  memory.reclaim
	A write-only nested-keyed file which exists for all cgroups.

	This is a simple interface to trigger memory reclaim in the
	target cgroup.

```

	  echo "1G" > memory.reclaim

	Please note that the kernel can over or under reclaim from
	the target cgroup. If less bytes are reclaimed than the
	specified amount, -EAGAIN is returned.

	Please note that the proactive reclaim (triggered by this
	interface) is not meant to indicate memory pressure on the
	memory cgroup. Therefore socket memory balancing triggered by
	the memory reclaim normally is not exercised in this case.
	This means that the networking layer will not adapt based on
	reclaim induced by memory.reclaim.

```
The following nested keys are defined.

	  ==========            ================================
	  swappiness            Swappiness value to reclaim with
	  ==========            ================================

	Specifying a swappiness value instructs the kernel to perform
	the reclaim with that swappiness value. Note that this has the
	same semantics as vm.swappiness applied to memcg reclaim with
	all the existing limitations and potential future extensions.

	The valid range for swappiness is [0-200, max], setting
	swappiness=max exclusively reclaims anonymous memory.

  memory.peak
	A read-write single value file which exists on non-root cgroups.

	The max memory usage recorded for the cgroup and its descendants since
	either the creation of the cgroup or the most recent reset for that FD.

	A write of any non-empty string to this file resets it to the
	current memory usage for subsequent reads through the same
	file descriptor.

  memory.oom.group
	A read-write single value file which exists on non-root
	cgroups.  The default value is "0".

	Determines whether the cgroup should be treated as
	an indivisible workload by the OOM killer. If set,
	all tasks belonging to the cgroup or to its descendants
	(if the memory cgroup is not a leaf cgroup) are killed
	together or not at all. This can be used to avoid
	partial kills to guarantee workload integrity.

	Tasks with the OOM protection (oom_score_adj set to -1000)
	are treated as an exception and are never killed.

	If the OOM killer is invoked in a cgroup, it's not going
	to kill any tasks outside of this cgroup, regardless
	memory.oom.group values of ancestor cgroups.

  memory.events
	A read-only flat-keyed file which exists on non-root cgroups.
	The following entries are defined.  Unless specified
	otherwise, a value change in this file generates a file
	modified event.

	Note that all fields in this file are hierarchical and the
	file modified event can be generated due to an event down the
	hierarchy. For the local events at the cgroup level see
	memory.events.local.

	  low
		The number of times the cgroup is reclaimed due to
		high memory pressure even though its usage is under
		the low boundary.  This usually indicates that the low
		boundary is over-committed.

	  high
		The number of times processes of the cgroup are
		throttled and routed to perform direct memory reclaim
		because the high memory boundary was exceeded.  For a
		cgroup whose memory usage is capped by the high limit
		rather than global memory pressure, this event's
		occurrences are expected.

	  max
		The number of times the cgroup's memory usage was
		about to go over the max boundary.  If direct reclaim
		fails to bring it down, the cgroup goes to OOM state.

	  oom
		The number of time the cgroup's memory usage was
		reached the limit and allocation was about to fail.

		This event is not raised if the OOM killer is not
		considered as an option, e.g. for failed high-order
		allocations or if caller asked to not retry attempts.

	  oom_kill
		The number of processes belonging to this cgroup
		killed by any kind of OOM killer.

          oom_group_kill
                The number of times a group OOM has occurred.

          sock_throttled
                The number of times network sockets associated with
                this cgroup are throttled.

  memory.events.local
	Similar to memory.events but the fields in the file are local
	to the cgroup i.e. not hierarchical. The file modified event
	generated on this file reflects only the local events.

  memory.stat
	A read-only flat-keyed file which exists on non-root cgroups.

	This breaks down the cgroup's memory footprint into different
	types of memory, type-specific details, and other information
	on the state and past events of the memory management system.

	All memory amounts are in bytes.

	The entries are ordered to be human readable, and new entries
	can show up in the middle. Don't rely on items remaining in a
	fixed position; use the keys to look up specific values!

	If the entry has no per-node counter (or not show in the
	memory.numa_stat). We use 'npn' (non-per-node) as the tag
	to indicate that it will not show in the memory.numa_stat.

	  anon
		Amount of memory used in anonymous mappings such as
		brk(), sbrk(), and mmap(MAP_ANONYMOUS). Note that
		some kernel configurations might account complete larger
		allocations (e.g., THP) if only some, but not all the
		memory of such an allocation is mapped anymore.

	  file
		鐢ㄤ簬缂撳瓨鏂囦欢绯荤粺鏁版嵁鐨勫唴瀛橀噺锛屽寘鎷?tmpfs 涓庡叡浜唴瀛樸€?

	  kernel (npn)
		鍐呮牳鍐呭瓨鎬婚噺锛屽寘鎷?
		(kernel_stack, pagetables, percpu, vmalloc, slab) 浠ュ強
		鍏跺畠鍐呮牳鍐呭瓨浣跨敤鍦烘櫙銆?

	  kernel_stack
		鍒嗛厤缁欏唴鏍告爤鐨勫唴瀛橀噺銆?

	  pagetables
                Amount of memory allocated for page tables.

	  sec_pagetables
		鐢ㄤ簬浜岀骇椤佃〃锛坰econdary page tables锛夌殑鍐呭瓨閲忥紝
		鐩墠鍖呮嫭 x86 涓?arm64 涓婄殑 KVM mmu 鍒嗛厤锛屼互鍙?IOMMU 椤佃〃銆?

	  percpu (npn)
		鐢ㄤ簬瀛樺偍姣?CPU 鍐呮牳鏁版嵁缁撴瀯鐨勫唴瀛橀噺銆?

	  sock (npn)
		鐢ㄤ簬缃戠粶浼犺緭缂撳啿鍖虹殑鐨勫唴瀛橀噺銆?

	  vmalloc (npn)
		鐢ㄤ簬 vmap 鍚庡鍐呭瓨鐨勫唴瀛橀噺銆?

	  shmem
		琚氦鎹紙swap锛夋敮鎸佺殑宸茬紦瀛樻枃浠剁郴缁熸暟鎹噺锛?
		渚嬪 tmpfs銆乻hm 娈点€佸叡浜尶鍚?mmap()銆?

	  zswap
		琚?zswap 鍘嬬缉鍚庣娑堣€楃殑鍐呭瓨閲忋€?

	  zswapped
		琚氦鎹㈠埌 zswap 鐨勫簲鐢ㄧ▼搴忓唴瀛橀噺銆?

	  file_mapped
		閫氳繃 mmap() 鏄犲皠鐨勫凡缂撳瓨鏂囦欢绯荤粺鏁版嵁閲忋€傛敞鎰?
		鏌愪簺鍐呮牳閰嶇疆鍙兘灏嗘暣涓洿澶х殑鍒嗛厤锛堜緥濡?THP锛夎璐︼紝
		濡傛灉姝ょ被鍒嗛厤涓彧鏈夐儴鍒嗭紙鑰岄潪鍏ㄩ儴锛夊唴瀛樹粛琚槧灏勩€?

	  file_dirty
		琚慨鏀逛絾灏氭湭鍐欏洖纾佺洏鐨勫凡缂撳瓨鏂囦欢绯荤粺鏁版嵁閲忋€?

	  file_writeback
		宸蹭慨鏀逛笖褰撳墠姝ｅ湪鍐欏洖纾佺洏鐨勫凡缂撳瓨鏂囦欢绯荤粺鏁版嵁閲忋€?

	  swapcached
		缂撳瓨鍦ㄥ唴瀛樹腑鐨勪氦鎹㈤噺銆俿wapcache 鍚屾椂璁″叆鍐呭瓨浣跨敤涓庝氦鎹娇鐢ㄣ€?

	  anon_thp
		鐢遍€忔槑澶ч〉锛坱ransparent hugepages锛夋敮鎸佺殑鍖垮悕鏄犲皠鎵€浣跨敤鐨勫唴瀛橀噺銆?

	  file_thp
		鐢遍€忔槑澶ч〉鏀寔鐨勫凡缂撳瓨鏂囦欢绯荤粺鏁版嵁閲忋€?

	  shmem_thp
		鐢遍€忔槑澶ч〉鏀寔鐨?shm銆乼mpfs銆佸叡浜尶鍚?mmap() 閲忋€?

	  inactive_anon, active_anon, inactive_file, active_file, unevictable
		鍩轰簬鍐呴儴鍐呭瓨绠＄悊鍒楄〃锛堣椤靛洖鏀剁畻娉曚娇鐢級鐨勫唴瀛樹笌浜ゆ崲銆佹枃浠剁郴缁熸敮鎸佺殑浜ゆ崲閲忋€?

		As these represent internal list state (eg. shmem pages are on anon
		memory management lists), inactive_foo + active_foo may not be equal to
		the value for the foo counter, since the foo counter is type-based, not
		list-based.

	  slab_reclaimable
		鈥渟lab鈥?涓彲鑳借鍥炴敹鐨勯儴鍒嗭紝渚嬪 dentries 涓?inodes銆?

	  slab_unreclaimable
		鍦ㄥ唴瀛樺帇鍔涗笅鏃犳硶琚洖鏀剁殑 鈥渟lab鈥?閮ㄥ垎銆?

	  slab (npn)
		鐢ㄤ簬瀛樺偍鍐呮牳鍐呴儴鏁版嵁缁撴瀯鐨勫唴瀛橀噺銆?

	  workingset_refault_anon
		涔嬪墠琚┍閫愮殑鍖垮悕椤靛彂鐢熷啀娆＄己椤碉紙refault锛夌殑娆℃暟銆?

	  workingset_refault_file
		涔嬪墠琚┍閫愮殑鏂囦欢椤靛彂鐢熷啀娆＄己椤电殑娆℃暟銆?

	  workingset_activate_anon
		琚珛鍗虫縺娲荤殑鍐嶆缂洪〉鍖垮悕椤垫暟閲忋€?

	  workingset_activate_file
		琚珛鍗虫縺娲荤殑鍐嶆缂洪〉鏂囦欢椤垫暟閲忋€?

	  workingset_restore_anon
		鍦ㄨ鍥炴敹涔嬪墠琚娴嬩负娲诲姩 workingset 鐨勩€佸凡鎭㈠鐨勫尶鍚嶉〉鏁伴噺銆?

	  workingset_restore_file
		鍦ㄨ鍥炴敹涔嬪墠琚娴嬩负娲诲姩 workingset 鐨勩€佸凡鎭㈠鐨勬枃浠堕〉鏁伴噺銆?

	  workingset_nodereclaim
		褰卞瓙鑺傜偣锛坰hadow node锛夎鍥炴敹鐨勬鏁般€?

	  pswpin (npn)
		鎹㈠叆鍐呭瓨鐨勯〉鏁般€?

	  pswpout (npn)
		鎹㈠嚭鍐呭瓨鐨勯〉鏁般€?

	  pgscan (npn)
		宸叉壂鎻忕殑椤垫暟锛堝湪闈炴椿鍔?LRU 鍒楄〃涓級銆?

	  pgsteal (npn)
		宸插洖鏀剁殑椤垫暟銆?

	  pgscan_kswapd (npn)
		kswapd 鎵弿鐨勯〉鏁帮紙鍦ㄩ潪娲诲姩 LRU 鍒楄〃涓級銆?

	  pgscan_direct (npn)
		鐩存帴鎵弿鐨勯〉鏁帮紙鍦ㄩ潪娲诲姩 LRU 鍒楄〃涓級銆?

	  pgscan_khugepaged (npn)
		khugepaged 鎵弿鐨勯〉鏁帮紙鍦ㄩ潪娲诲姩 LRU 鍒楄〃涓級銆?

	  pgscan_proactive (npn)
		涓诲姩鎵弿鐨勯〉鏁帮紙鍦ㄩ潪娲诲姩 LRU 鍒楄〃涓級銆?

	  pgsteal_kswapd (npn)
		kswapd 鍥炴敹鐨勯〉鏁般€?

	  pgsteal_direct (npn)
		鐩存帴鍥炴敹鐨勯〉鏁般€?

	  pgsteal_khugepaged (npn)
		khugepaged 鍥炴敹鐨勯〉鏁般€?

	  pgsteal_proactive (npn)
		涓诲姩鍥炴敹鐨勯〉鏁般€?

	  pgfault (npn)
		鍙戠敓鐨勬€荤己椤垫鏁般€?

	  pgmajfault (npn)
		鍙戠敓鐨勪富瑕佺己椤碉紙major page fault锛夋鏁般€?

	  pgrefill (npn)
		宸叉壂鎻忕殑椤垫暟锛堝湪娲诲姩 LRU 鍒楄〃涓級銆?

	  pgactivate (npn)
		绉诲姩鍒版椿鍔?LRU 鍒楄〃鐨勯〉鏁般€?

	  pgdeactivate (npn)
		绉诲姩鍒伴潪娲诲姩 LRU 鍒楄〃鐨勯〉鏁般€?

	  pglazyfree (npn)
		鍦ㄥ唴瀛樺帇鍔涗笅琚帹杩熼噴鏀剧殑椤垫暟銆?

	  pglazyfreed (npn)
		宸插洖鏀剁殑 lazyfree 椤垫暟銆?

	  swpin_zero
		鎹㈠叆鍐呭瓨骞跺～鍏呬负闆剁殑椤垫暟锛屽叾涓敱浜庝氦鎹㈠嚭鏃舵娴嬪埌椤靛唴瀹逛负闆惰€屼紭鍖栦簡 I/O銆?

	  swpout_zero
		鍥犲唴瀹硅妫€娴嬩负闆惰€岃烦杩?I/O 鐨勩€佽濉厖涓洪浂骞舵崲鍑虹殑椤垫暟銆?

	  zswpin
		浠?zswap 绉诲叆鍐呭瓨鐨勯〉鏁般€?

	  zswpout
		浠庡唴瀛樼Щ鍑哄埌 zswap 鐨勯〉鏁般€?

	  zswpwb
		浠?zswap 鍐欏叆浜ゆ崲鐨勯〉鏁般€?

	  zswap_incomp
		褰撳墠鏈粡鍘嬬缉瀛樺偍鍦?zswap 涓殑涓嶅彲鍘嬬缉椤垫暟銆?
		杩欎簺椤垫棤娉曡鍘嬬缉鍒板皬浜?PAGE_SIZE 鐨勫昂瀵革紝鍥犳鎸夊師鏍峰瓨鍌ㄣ€?

	  thp_fault_alloc (npn)
		涓烘弧瓒充竴娆＄己椤佃€屽垎閰嶇殑閫忔槑澶ч〉鏁伴噺銆傚湪鏈缃?CONFIG_TRANSPARENT_HUGEPAGE
                鏃朵笉鍑虹幇姝よ鏁板櫒銆?

	  thp_collapse_alloc (npn)
		涓哄厑璁稿皢涓€娈电幇鏈夐〉鑼冨洿鎶樺彔锛坈ollapse锛夎€屽垎閰嶇殑閫忔槑澶ч〉鏁伴噺銆傚湪鏈缃?
		CONFIG_TRANSPARENT_HUGEPAGE 鏃朵笉鍑虹幇姝よ鏁板櫒銆?

	  thp_swpout (npn)
		涓嶇粡鎷嗗垎銆佷竴娆℃€ф暣浣撲氦鎹㈠嚭鐨勯€忔槑澶ч〉鏁伴噺銆?

	  thp_swpout_fallback (npn)
		鍦ㄤ氦鎹㈠嚭涔嬪墠琚媶鍒嗙殑閫忔槑澶ч〉鏁伴噺銆?
		閫氬父鏄洜涓烘湭鑳戒负杩欎釜澶ч〉鍒嗛厤鏌愪簺杩炵画鐨勪氦鎹㈢┖闂淬€?

	  numa_pages_migrated (npn)
		NUMA 骞宠　杩佺Щ鐨勯〉鏁般€?

	  numa_pte_updates (npn)
		鍏堕〉琛ㄩ」琚?NUMA 骞宠　淇敼浠ュ湪璁块棶鏃朵骇鐢?NUMA 鎻愮ず缂洪〉锛坔inting fault锛夌殑椤垫暟銆?

	  numa_hint_faults (npn)
		NUMA 鎻愮ず缂洪〉鐨勬鏁般€?

	  pgdemote_kswapd
		kswapd 闄嶇骇锛坉emote锛夌殑椤垫暟銆?

	  pgdemote_direct
		鐩存帴闄嶇骇鐨勯〉鏁般€?

	  pgdemote_khugepaged
		khugepaged 闄嶇骇鐨勯〉鏁般€?

	  pgdemote_proactive
		涓诲姩闄嶇骇鐨勯〉鏁般€?

	  hugetlb
		鐢?hugetlb 椤典娇鐢ㄧ殑鍐呭瓨閲忋€備粎褰?hugetlb 鐢ㄩ噺鍦?memory.current 涓璁拌处鏃?
		锛堝嵆 cgroup 浠?memory_hugetlb_accounting 閫夐」鎸傝浇锛夋墠浼氬嚭鐜版鎸囨爣銆?

  memory.numa_stat
	A read-only nested-keyed file which exists on non-root cgroups.

	This breaks down the cgroup's memory footprint into different
	types of memory, type-specific details, and other information
	per node on the state of the memory management system.

	This is useful for providing visibility into the NUMA locality
	information within an memcg since the pages are allowed to be
	allocated from any physical node. One of the use case is evaluating
	application performance by combining this information with the
	application's CPU allocation.

	All memory amounts are in bytes.

```

	  type N0=<bytes in node 0> N1=<bytes in node 1> ...

	The entries are ordered to be human readable, and new entries
	can show up in the middle. Don't rely on items remaining in a
	fixed position; use the keys to look up specific values!

	The entries can refer to the memory.stat.

  memory.swap.current
	A read-only single value file which exists on non-root
	cgroups.

	璇?cgroup 鍙婂叾鍚庝唬褰撳墠姝ｅ湪浣跨敤鐨勪氦鎹㈡€婚噺銆?

  memory.swap.high
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Swap usage throttle limit.  If a cgroup's swap usage exceeds
	this limit, all its further allocations will be throttled to
	allow userspace to implement custom out-of-memory procedures.

	This limit marks a point of no return for the cgroup. It is NOT
	designed to manage the amount of swapping a workload does
	during regular operation. Compare to memory.swap.max, which
	prohibits swapping past a set amount, but lets the cgroup
	continue unimpeded as long as other memory can be reclaimed.

	Healthy workloads are not expected to reach this limit.

  memory.swap.peak
	A read-write single value file which exists on non-root cgroups.

	The max swap usage recorded for the cgroup and its descendants since
	the creation of the cgroup or the most recent reset for that FD.

	A write of any non-empty string to this file resets it to the
	current memory usage for subsequent reads through the same
	file descriptor.

  memory.swap.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Swap usage hard limit.  If a cgroup's swap usage reaches this
	limit, anonymous memory of the cgroup will not be swapped out.

  memory.swap.events
	A read-only flat-keyed file which exists on non-root cgroups.
	The following entries are defined.  Unless specified
	otherwise, a value change in this file generates a file
	modified event.

	  high
		The number of times the cgroup's swap usage was over
		the high threshold.

	  max
		The number of times the cgroup's swap usage was about
		to go over the max boundary and swap allocation
		failed.

	  fail
		The number of times swap allocation failed either
		because of running out of swap system-wide or max
		limit.

	When reduced under the current usage, the existing swap
	entries are reclaimed gradually and the swap usage may stay
	higher than the limit for an extended period of time.  This
	reduces the impact on the workload and memory management.

  memory.zswap.current
	A read-only single value file which exists on non-root
	cgroups.

	The total amount of memory consumed by the zswap compression
	backend.

  memory.zswap.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	Zswap usage hard limit. If a cgroup's zswap pool reaches this
	limit, it will refuse to take any more stores before existing
	entries fault back in or are written out to disk.

  memory.zswap.writeback
	A read-write single value file. The default value is "1".
	Note that this setting is hierarchical, i.e. the writeback would be
	implicitly disabled for child cgroups if the upper hierarchy
	does so.

	When this is set to 0, all swapping attempts to swapping devices
	are disabled. This included both zswap writebacks, and swapping due
	to zswap store failures. If the zswap store failures are recurring
	(for e.g if the pages are incompressible), users can observe
	reclaim inefficiency after disabling writeback (because the same
	pages might be rejected again and again).

	Note that this is subtly different from setting memory.swap.max to
	0, as it still allows for pages to be written to the zswap pool.
	This setting has no effect if zswap is disabled, and swapping
	is allowed unless memory.swap.max is set to 0.

  memory.pressure
	A read-only nested-keyed file.

	Shows pressure stall information for memory. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.


```
#### 浣跨敤鍑嗗垯


鈥渕emory.high鈥?鏄帶鍒跺唴瀛樹娇鐢ㄧ殑涓昏鏈哄埗銆傚 high 闄愰杩涜杩囧害鎵胯锛坔igh 闄愰涔嬪拰 > 鍙敤鍐呭瓨锛夛紝骞惰鍏ㄥ眬鍐呭瓨鍘嬪姏鏍规嵁鐢ㄩ噺鏉ュ垎閰嶅唴瀛橈紝鏄竴绉嶅彲琛岀殑绛栫暐銆?

鐢变簬绐佺牬 high 闄愰涓嶄細瑙﹀彂 OOM killer锛岃€屽彧鏄妭娴佽繚瑙勭殑 cgroup锛岀鐞嗕唬鐞嗘湁鍏呭垎鐨勬満浼氳繘琛岀洃鎺у苟閲囧彇閫傚綋琛屽姩锛屼緥濡傛巿浜堟洿澶氬唴瀛樻垨缁堟宸ヤ綔璐熻浇銆?

纭畾涓€涓?cgroup 鏄惁鏈夎冻澶熷唴瀛樺苟闈炴槗浜嬶紝鍥犱负鍐呭瓨鐢ㄩ噺骞朵笉鑳借〃鏄庤宸ヤ綔璐熻浇鏄惁鍙互浠庢洿澶氬唴瀛樹腑鍙楃泭銆備緥濡傦紝涓€涓皢缃戠粶鎺ユ敹鍒扮殑鏁版嵁鍐欏叆鏂囦欢鐨勫伐浣滆礋杞藉彲浠ヤ娇鐢ㄦ墍鏈夊彲鐢ㄥ唴瀛橈紝浣嗕篃鍙互鍦ㄥ彧鏈夊皯閲忓唴瀛樻椂鍚屾牱楂樻晥鍦拌繍琛屻€傞渶瑕佷竴绉嶅唴瀛樺帇鍔涘害閲忊€斺€斿嵆宸ヤ綔璐熻浇鍥犵己涔忓唴瀛樿€屽彈鍒板澶у奖鍝嶁€斺€旀潵纭畾宸ヤ綔璐熻浇鏄惁闇€瑕佹洿澶氬唴瀛橈紱閬楁喚鐨勬槸锛屽唴瀛樺帇鍔涚洃鎺ф満鍒跺皻鏈疄鐜般€?


#### 鍥炴敹淇濇姢


閫氳繃 鈥渕emory.low鈥?鎴?鈥渕emory.min鈥?閰嶇疆鐨勪繚鎶わ紝鐩稿鍦板簲鐢ㄤ簬鍥炴敹鐩爣锛堝嵆浠讳綍鍐呭瓨 cgroup 闄愰銆佷富鍔ㄧ殑 memory.reclaim锛屾垨鏄剧劧浣嶄簬鏍?cgroup 涓殑鍏ㄥ眬鍥炴敹锛夈€備负 B 閰嶇疆鐨勪繚鎶ゅ€煎師鏍烽€傜敤浜庡洖鏀?
```

		root - ... - A - B - C
		              \    ` D
		               ` E

```
When the reclaim targets ancestors of A, the effective protection of B is
capped by the protection value configured for A (and any other intermediate
ancestors between A and the target).

To express indifference about relative sibling protection, it is suggested to
use memory_recursiveprot. Configuring all descendants of a parent with finite
protection to "max" works but it may unnecessarily skew memory.events:low
field.


#### 鍐呭瓨鎵€鏈夋潈


涓€涓唴瀛樺尯鍩熶細琚璐﹀埌瀹炰緥鍖栧畠鐨?cgroup锛屽苟涓€鐩翠繚鎸佽璐﹀埌璇ュ尯鍩熻閲婃斁涓烘銆傚皢杩涚▼杩佺Щ鍒颁笉鍚岀殑 cgroup 骞朵笉浼氭妸瀹冨湪鍓嶄竴涓?cgroup 涓疄渚嬪寲鐨勫唴瀛樼敤閲忕Щ鍔ㄥ埌鏂扮殑 cgroup銆?

涓€涓唴瀛樺尯鍩熷彲鑳借灞炰簬涓嶅悓 cgroup 鐨勮繘绋嬩娇鐢ㄣ€傝鍖哄煙浼氳璁拌处鍒板摢涓?cgroup 鏄笉纭畾鐨勶紱涓嶈繃锛岄殢鐫€鏃堕棿鐨勬帹绉伙紝璇ュ唴瀛樺尯鍩熷緢鍙兘鏈€缁堣惤鍦ㄦ煇涓嫢鏈夎冻澶熷唴瀛橀厤棰濄€佷互閬垮厤楂樻槀鍥炴敹鍘嬪姏鐨?cgroup 涓€?

濡傛灉涓€涓?cgroup 娓呴櫎浜嗗ぇ閲忛鏈熶細琚叾浠?cgroup 鍙嶅璁块棶鐨勫唴瀛橈紝浣跨敤 POSIX_FADV_DONTNEED 鏉?relinquish 灞炰簬鐩稿叧鏂囦欢鐨勫唴瀛樺尯鍩熺殑鎵€鏈夋潈锛屼互纭繚姝ｇ‘鐨勫唴瀛樻墍鏈夋潈锛屽彲鑳芥槸鏈夋剰涔夌殑銆?


### IO


鈥渋o鈥?鎺у埗鍣ㄨ皟鑺?IO 璧勬簮鐨勫垎閰嶃€傝鎺у埗鍣ㄥ悓鏃跺疄鐜颁簡鍩轰簬鏉冮噸浠ュ強缁濆甯﹀鎴?IOPS 闄愰鐨勫垎閰嶏紱涓嶈繃锛屽熀浜庢潈閲嶇殑鍒嗛厤浠呭湪浣跨敤浜?cfq-iosched 鏃跺彲鐢紝涓斾袱绉嶆柟妗堝 blk-mq 璁惧閮戒笉鍙敤銆?


#### IO 鎺ュ彛鏂囦欢


  io.stat
	A read-only nested-keyed file.

	Lines are keyed by $MAJ:$MIN device numbers and not ordered.
	The following nested keys are defined.

	  ======	=====================
	  rbytes	璇诲彇鐨勫瓧鑺傛暟
	  wbytes	鍐欏叆鐨勫瓧鑺傛暟
	  rios		璇?IO 娆℃暟
	  wios		鍐?IO 娆℃暟
	  dbytes	涓㈠純鐨勫瓧鑺傛暟
	  dios		涓㈠純 IO 娆℃暟
	  ======	=====================

```

	  8:16 rbytes=1459200 wbytes=314773504 rios=192 wios=353 dbytes=0 dios=0
	  8:0 rbytes=90430464 wbytes=299008000 rios=8950 wios=1252 dbytes=50331648 dios=3021

  io.cost.qos
	A read-write nested-keyed file which exists only on the root
	cgroup.

	This file configures the Quality of Service of the IO cost
	model based controller (CONFIG_BLK_CGROUP_IOCOST) which
	currently implements "io.weight" proportional control.  Lines
	are keyed by $MAJ:$MIN device numbers and not ordered.  The
	line for a given device is populated on the first write for
	the device on "io.cost.qos" or "io.cost.model".  The following
	nested keys are defined.

	  ======	=====================================
	  enable	鍩轰簬鏉冮噸鐨勬帶鍒跺紑鍏?
	  ctrl		"auto" 鎴?"user"
	  rpct		璇诲欢杩熺櫨鍒嗕綅    [0, 100]
	  rlat		璇诲欢杩熼槇鍊?
	  wpct		鍐欏欢杩熺櫨鍒嗕綅   [0, 100]
	  wlat		鍐欏欢杩熼槇鍊?
	  min		鏈€灏忕缉鏀剧櫨鍒嗘瘮 [1, 10000]
	  max		鏈€澶х缉鏀剧櫨鍒嗘瘮 [1, 10000]
	  ======	=====================================

	The controller is disabled by default and can be enabled by
	setting "enable" to 1.  "rpct" and "wpct" parameters default
	to zero and the controller uses internal device saturation
	state to adjust the overall IO rate between "min" and "max".

	When a better control quality is needed, latency QoS
	parameters can be configured.  For example::

	  8:16 enable=1 ctrl=auto rpct=95.00 rlat=75000 wpct=95.00 wlat=150000 min=50.00 max=150.0

	shows that on sdb, the controller is enabled, will consider
	the device saturated if the 95th percentile of read completion
	latencies is above 75ms or write 150ms, and adjust the overall
	IO issue rate between 50% and 150% accordingly.

	The lower the saturation point, the better the latency QoS at
	the cost of aggregate bandwidth.  The narrower the allowed
	adjustment range between "min" and "max", the more conformant
	to the cost model the IO behavior.  Note that the IO issue
	base rate may be far off from 100% and setting "min" and "max"
	blindly can lead to a significant loss of device capacity or
	control quality.  "min" and "max" are useful for regulating
	devices which show wide temporary behavior changes - e.g. a
	ssd which accepts writes at the line speed for a while and
	then completely stalls for multiple seconds.

	When "ctrl" is "auto", the parameters are controlled by the
	kernel and may change automatically.  Setting "ctrl" to "user"
	or setting any of the percentile and latency parameters puts
	it into "user" mode and disables the automatic changes.  The
	automatic mode can be restored by setting "ctrl" to "auto".

  io.cost.model
	A read-write nested-keyed file which exists only on the root
	cgroup.

	This file configures the cost model of the IO cost model based
	controller (CONFIG_BLK_CGROUP_IOCOST) which currently
	implements "io.weight" proportional control.  Lines are keyed
	by $MAJ:$MIN device numbers and not ordered.  The line for a
	given device is populated on the first write for the device on
	"io.cost.qos" or "io.cost.model".  The following nested keys
	are defined.

	  =====		================================
	  ctrl		"auto" 鎴?"user"
	  model		姝ｅ湪浣跨敤鐨勬垚鏈ā鍨?- "linear"
	  =====		================================

	When "ctrl" is "auto", the kernel may change all parameters
	dynamically.  When "ctrl" is set to "user" or any other
	parameters are written to, "ctrl" become "user" and the
	automatic changes are disabled.

	When "model" is "linear", the following model parameters are
	defined.

	  =============	========================================
	  [r|w]bps	鏈€澶ч『搴?IO 鍚炲悙
	  [r|w]seqiops	鏈€澶?4k 椤哄簭 IO 姣忕娆℃暟
	  [r|w]randiops	鏈€澶?4k 闅忔満 IO 姣忕娆℃暟
	  =============	========================================

	From the above, the builtin linear model determines the base
	costs of a sequential and random IO and the cost coefficient
	for the IO size.  While simple, this model can cover most
	common device classes acceptably.

	The IO cost model isn't expected to be accurate in absolute
	sense and is scaled to the device behavior dynamically.

	If needed, tools/cgroup/iocost_coef_gen.py can be used to
	generate device-specific coefficients.

  io.weight
	A read-write flat-keyed file which exists on non-root cgroups.
	The default is "default 100".

	The first line is the default weight applied to devices
	without specific override.  The rest are overrides keyed by
	$MAJ:$MIN device numbers and not ordered.  The weights are in
	the range [1, 10000] and specifies the relative amount IO time
	the cgroup can use in relation to its siblings.

	The default weight can be updated by writing either "default
	$WEIGHT" or simply "$WEIGHT".  Overrides can be set by writing
	"$MAJ:$MIN $WEIGHT" and unset by writing "$MAJ:$MIN default".

	An example read output follows::

	  default 100
	  8:16 200
	  8:0 50

  io.max
	A read-write nested-keyed file which exists on non-root
	cgroups.

	BPS and IOPS based IO limit.  Lines are keyed by $MAJ:$MIN
	device numbers and not ordered.  The following nested keys are
	defined.

	  =====		==================================
	  rbps		姣忕鏈€澶ц鍙栧瓧鑺傛暟
	  wbps		姣忕鏈€澶у啓鍏ュ瓧鑺傛暟
	  riops		姣忕鏈€澶ц IO 鎿嶄綔鏁?
	  wiops		姣忕鏈€澶у啓 IO 鎿嶄綔鏁?
	  =====		==================================

	When writing, any number of nested key-value pairs can be
	specified in any order.  "max" can be specified as the value
	to remove a specific limit.  If the same key is specified
	multiple times, the outcome is undefined.

	BPS and IOPS are measured in each IO direction and IOs are
	delayed if limit is reached.  Temporary bursts are allowed.

	Setting read limit at 2M BPS and write at 120 IOPS for 8:16::

	  echo "8:16 rbps=2097152 wiops=120" > io.max

	Reading returns the following::

	  8:16 rbps=2097152 wbps=max riops=max wiops=120

	Write IOPS limit can be removed by writing the following::

	  echo "8:16 wiops=max" > io.max

	Reading now returns the following::

	  8:16 rbps=2097152 wbps=max riops=max wiops=max

  io.pressure
	A read-only nested-keyed file.

	Shows pressure stall information for IO. See
	:ref:`Documentation/accounting/psi.rst <psi>` for details.


```
#### 鍥炲啓


椤电紦瀛橀€氳繃缂撳啿鍐欙紙buffered writes锛変笌鍏变韩 mmap 琚紕鑴忥紙dirtied锛夛紝骞剁敱鍥炲啓锛坵riteback锛夋満鍒跺紓姝ュ啓鍏ュ悗澶囨枃浠剁郴缁熴€傚洖鍐欎綅浜庡唴瀛樺煙涓?IO 鍩熶箣闂达紝閫氳繃骞宠　寮勮剰涓庡啓 IO 鏉ヨ皟鑺傝剰鍐呭瓨鐨勬瘮渚嬨€?

io 鎺у埗鍣ㄤ笌鍐呭瓨鎺у埗鍣ㄥ崗鍚岋紝瀹炵幇瀵归〉缂撳瓨鍥炲啓 IO 鐨勬帶鍒躲€傚唴瀛樻帶鍒跺櫒瀹氫箟浜嗚绠楀苟缁存姢鑴忓唴瀛樻瘮渚嬬殑鍐呭瓨鍩燂紝io 鎺у埗鍣ㄥ畾涔変簡涓鸿鍐呭瓨鍩熷啓鍑鸿剰椤电殑 io 鍩熴€傜郴缁熺骇涓庢瘡 cgroup 鐨勮剰鍐呭瓨鐘舵€侀兘浼氳妫€鏌ワ紝浜岃€呬腑鏇翠弗鏍肩殑閭ｄ釜浼氳寮哄埗鎵ц銆?

cgroup 鍥炲啓闇€瑕佸簳灞傛枃浠剁郴缁熺殑鏄惧紡鏀寔銆傜洰鍓嶏紝cgroup 鍥炲啓瀹炵幇浜?ext2銆乪xt4銆乥trfs銆乫2fs 鍜?xfs 涓娿€傚湪鍏跺畠鏂囦欢绯荤粺涓婏紝鎵€鏈夊洖鍐?IO 閮借褰掔畻鍒版牴 cgroup銆?

鍐呭瓨涓庡洖鍐欑鐞嗕箣闂村瓨鍦ㄥ浐鏈夊樊寮傦紝杩欏奖鍝嶄簡 cgroup 鎵€鏈夋潈鐨勮窡韪柟寮忋€傚唴瀛樻槸姣忛〉璺熻釜鐨勶紝鑰屽洖鍐欐槸姣?inode 璺熻釜鐨勩€傚嚭浜庡洖鍐欑殑鐩殑锛屼竴涓?inode 琚垎閰嶇粰涓€涓?cgroup锛屾墍鏈変粠璇?inode 鍐欏嚭鑴忛〉鐨?IO 璇锋眰閮借褰掔畻鍒拌 cgroup銆?

鐢变簬鍐呭瓨鐨?cgroup 鎵€鏈夋潈鏄瘡椤佃窡韪殑锛屽彲鑳藉瓨鍦ㄤ竴浜涢〉闈笌 inode 鎵€鍏宠仈鐨?cgroup 涓嶅悓銆傝繖浜涜绉颁负澶栨潵椤碉紙foreign pages锛夈€傚洖鍐欐寔缁窡韪鏉ラ〉锛屽鏋滄煇涓壒瀹氱殑澶栨潵 cgroup 鍦ㄤ竴娈垫椂闂村唴鎴愪负澶氭暟锛屽氨灏?inode 鐨勬墍鏈夋潈鍒囨崲鍒拌 cgroup銆?

铏界劧瀵逛簬澶у鏁颁娇鐢ㄥ満鏅€岃█锛屾妯″瀷宸茬粡瓒冲鈥斺€斿嵆浣夸富鍐欏叆 cgroup 闅忔椂闂村彉鍖栵紝缁欏畾涓€涓?inode 澶ч儴鍒嗘椂鍊欑敱鍗曚竴 cgroup 寮勮剰鈥斺€斾絾澶氫釜 cgroup 鍚屾椂鍐欏叆鍗曚釜 inode 鐨勪娇鐢ㄥ満鏅敮鎸佸緱骞朵笉濂姐€傚湪杩欑鎯呭喌涓嬶紝鐩稿綋澶т竴閮ㄥ垎 IO 寰堝彲鑳借閿欒褰掔畻銆傜敱浜庡唴瀛樻帶鍒跺櫒鍦ㄩ娆′娇鐢ㄦ椂鍒嗛厤椤垫墍鏈夋潈锛屽苟涓斿湪椤佃閲婃斁涔嬪墠涓嶄細鏇存柊锛屽嵆浣垮洖鍐欎弗鏍奸伒寰〉鎵€鏈夋潈锛屽涓?cgroup 寮勮剰閲嶅彔鍖哄煙涔熸棤娉曞棰勬湡閭ｆ牱宸ヤ綔銆傚缓璁伩鍏嶆绫讳娇鐢ㄦā寮忋€?

褰卞搷鍥炲啓琛屼负鐨?sysctl 鏃嬮挳鎸夊涓嬫柟寮忓簲鐢ㄤ簬 cgroup 鍥炲啓銆?

  vm.dirty_background_ratio, vm.dirty_ratio
	These ratios apply the same to cgroup writeback with the
	amount of available memory capped by limits imposed by the
	memory controller and system-wide clean memory.

  vm.dirty_background_bytes, vm.dirty_bytes
	For cgroup writeback, this is calculated into ratio against
	total available memory and applied the same way as
	vm.dirty[_background]_ratio.


#### IO 寤惰繜


杩欐槸涓€涓敤浜?IO 宸ヤ綔璐熻浇淇濇姢鐨?cgroup v2 鎺у埗鍣ㄣ€備綘涓烘煇涓粍鎻愪緵涓€涓欢杩熺洰鏍囷紝濡傛灉骞冲潎寤惰繜瓒呰繃浜嗚鐩爣锛屾帶鍒跺櫒灏变細瀵规墍鏈夊叿鏈夋瘮鍙椾繚鎶ゅ伐浣滆礋杞芥洿浣庡欢杩熺洰鏍囩殑鍚岃緢锛坧eer锛夎繘琛岃妭娴併€?

闄愬埗鍙簲鐢ㄤ簬灞傛涓殑鍚岃緢灞傜骇銆傝繖鎰忓懗鐫€鍦ㄤ笅闈㈢殑鍥句腑锛屽彧鏈夌粍 A銆丅 鍜?C 浼氱浉浜掑奖鍝嶏紝鑰?
```

			[root]
		/	   |		\
		A	   B		C
	       /  \        |
	      D    F	   G


```
So the ideal way to configure this is to set io.latency in groups A, B, and C.
Generally you do not want to set a value lower than the latency your device
supports.  Experiment to find the value that works best for your workload.
Start at higher than the expected latency for your device and watch the
avg_lat value in io.stat for your workload group to get an idea of the
latency you see during normal operation.  Use the avg_lat value as a basis for
your real setting, setting at 10-15% higher than the value in io.stat.

#### IO 寤惰繜鑺傛祦濡備綍宸ヤ綔


io.latency 鏄伐浣滀繚鎸侊紙work conserving锛夌殑锛涘洜姝ゅ彧瑕佹瘡涓粍閮芥弧瓒冲叾寤惰繜鐩爣锛屾帶鍒跺櫒灏变笉鍋氫换浣曚簨銆備竴鏃︽煇涓粍寮€濮嬫湭杈惧埌鍏剁洰鏍囷紝瀹冨氨寮€濮嬪浠讳綍鍏锋湁姣旇嚜韬洿楂樼洰鏍囩殑鍚岃緢缁勮繘琛岃妭娴併€傝繖绉嶈妭娴佹湁涓ょ褰㈠紡锛?

- 闃熷垪娣卞害锛圦ueue depth锛夎妭娴併€傝繖鏄竴涓粍鍏佽鎷ユ湁鐨勬湭瀹屾垚 IO 鏁伴噺銆傛垜浠細鐩稿蹇€熷湴鏀剁揣瀹冿紝浠庢棤闄愬埗寮€濮嬶紝涓€鐩撮檷鍒版瘡娆″彧鍏佽 1 涓?IO銆?

- 浜轰负寤惰繜璇卞锛圓rtificial delay induction锛夈€傛煇浜涚被鍨嬬殑 IO 鏃犳硶鍦ㄤ笉瀵规洿楂樹紭鍏堢骇缁勪骇鐢熶笉鍒╁奖鍝嶇殑鎯呭喌涓嬭鑺傛祦銆傝繖鍖呮嫭浜ゆ崲锛坰wapping锛変笌鍏冩暟鎹?IO銆傝繖浜涚被鍨嬬殑 IO 鍏佽姝ｅ父鍙戠敓锛屼絾瀹冧滑浼氳鈥滆璐︹€濆埌鍙戣捣缁勩€傚鏋滃彂璧风粍姝ｅ湪琚妭娴侊紝浣犱細鐪嬪埌 io.stat 涓殑 use_delay 鍜?delay 瀛楁澧炲姞銆俤elay 鍊兼槸琚姞鍒板湪璇ョ粍涓繍琛岀殑浠讳綍杩涚▼涓婄殑寰鏁般€傜敱浜庡鏋滃彂鐢熶簡澶ч噺浜ゆ崲鎴栧厓鏁版嵁 IO锛岃繖涓暟瀛楀彲鑳戒細鍙樺緱鐩稿綋澶э紝鎴戜滑灏嗗崟涓欢杩熶簨浠堕檺鍒朵负姣忔鏈€澶?1 绉掋€?

涓€鏃﹀彈瀹崇粍鍐嶆寮€濮嬫弧瓒冲叾寤惰繜鐩爣锛屽畠灏变細寮€濮嬭В闄や箣鍓嶈鑺傛祦鐨勫悓杈堢粍鐨勮妭娴併€傚鏋滃彈瀹崇粍骞茶剢鍋滄杩涜 IO锛屽叏灞€璁℃暟鍣ㄤ細閫傚綋鍦拌В闄よ妭娴併€?


#### IO 寤惰繜鎺ュ彛鏂囦欢


  io.latency
	This takes a similar format as the other controllers.

		"MAJOR:MINOR target=<target time in microseconds>"

  io.stat
	If the controller is enabled you will see extra stats in io.stat in
	addition to the normal ones.

	  depth
		璇ョ粍褰撳墠鐨勯槦鍒楁繁搴︺€?

	  avg_lat
		杩欐槸涓€涓“鍑忕巼涓?1/exp銆佺敱閲囨牱闂撮殧闄愬畾鐨勬寚鏁扮Щ鍔ㄥ钩鍧囥€?
		琛板噺鐜囬棿闅斿彲浠ラ€氳繃灏?io.stat 涓殑 win 鍊间箻浠ュ熀浜?win 鍊肩殑鐩稿簲閲囨牱鏁版潵璁＄畻銆?

	  win
		閲囨牱绐楀彛澶у皬锛屼互姣涓哄崟浣嶃€傝繖鏄袱娆¤瘎浼颁簨浠朵箣闂寸殑鏈€鐭寔缁椂闂淬€?
		绐楀彛鍙湪鏈?IO 娲诲姩鏃舵墠浼氭祦閫濄€傜┖闂叉椂娈典細寤堕暱鏈€杩戠殑绐楀彛銆?

#### IO 浼樺厛绾?


A single attribute controls the behavior of the I/O priority cgroup policy,
namely the io.prio.class attribute. The following values are accepted for
that attribute:

  no-change
	涓嶄慨鏀?I/O 浼樺厛绾х被銆?

  promote-to-rt
	瀵逛簬鍏锋湁闈?RT I/O 浼樺厛绾х被鐨勮姹傦紝灏嗗叾鏀逛负 RT銆傚悓鏃跺皢杩欎簺璇锋眰鐨勪紭鍏堢骇绾у埆鏀逛负 4銆備笉淇敼鍏锋湁 RT 浼樺厛绾х被鐨勮姹傜殑 I/O 浼樺厛绾с€?

  restrict-to-be
	瀵逛簬娌℃湁 I/O 浼樺厛绾х被鎴栧叿鏈?I/O 浼樺厛绾х被 RT 鐨勮姹傦紝灏嗗叾鏀逛负 BE銆傚悓鏃跺皢杩欎簺璇锋眰鐨勪紭鍏堢骇绾у埆鏀逛负 0銆備笉淇敼鍏锋湁 IDLE 浼樺厛绾х被鐨勮姹傜殑 I/O 浼樺厛绾х被銆?

  idle
	灏嗘墍鏈夎姹傜殑 I/O 浼樺厛绾х被鏀逛负 IDLE锛堟渶浣庣殑 I/O 浼樺厛绾х被锛夈€?

  none-to-rt
	宸插純鐢ㄣ€傚彧鏄?promote-to-rt 鐨勫埆鍚嶃€?

The following numerical values are associated with the I/O priority policies:

+----------------+---+
| no-change      | 0 |
+----------------+---+
| promote-to-rt  | 1 |
+----------------+---+
| restrict-to-be | 2 |
+----------------+---+
| idle           | 3 |
+----------------+---+

The numerical value that corresponds to each I/O priority class is as follows:

+-------------------------------+---+
| IOPRIO_CLASS_NONE             | 0 |
+-------------------------------+---+
| IOPRIO_CLASS_RT (real-time)   | 1 |
+-------------------------------+---+
| IOPRIO_CLASS_BE (best effort) | 2 |
+-------------------------------+---+
| IOPRIO_CLASS_IDLE             | 3 |
+-------------------------------+---+

The algorithm to set the I/O priority class for a request is as follows:

- If I/O priority class policy is promote-to-rt, change the request I/O
  priority class to IOPRIO_CLASS_RT and change the request I/O priority
  level to 4.
- If I/O priority class policy is not promote-to-rt, translate the I/O priority
  class policy into a number, then change the request I/O priority class
  into the maximum of the I/O priority class policy number and the numerical
  I/O priority class.

### PID


杩涚▼鏁伴噺鎺у埗鍣ㄧ敤浜庡厑璁告煇涓?cgroup 鍦ㄨ揪鍒版寚瀹氶檺鍒跺悗锛岄樆姝换浣曟柊浠诲姟琚?fork() 鎴?clone()銆?

涓€涓?cgroup 涓殑浠诲姟鏁伴噺鍙兘浠ュ叾瀹冩帶鍒跺櫒鏃犳硶闃叉鐨勬柟寮忚鑰楀敖锛屽洜姝ら渶瑕佹湁鑷繁鐨勬帶鍒跺櫒銆備緥濡傦紝fork 鐐稿脊锛坒ork bomb锛夊緢鍙兘鍏堣€楀敖浠诲姟鏁伴噺锛岀劧鍚庢墠瑙﹀強鍐呭瓨闄愬埗銆?

娉ㄦ剰锛屾鎺у埗鍣ㄤ腑浣跨敤鐨?PID 鎸囩殑鏄?TID锛屽嵆鍐呮牳鎵€浣跨敤鐨勮繘绋?ID銆?


#### PID 鎺ュ彛鏂囦欢


  pids.max
	A read-write single value file which exists on non-root
	cgroups.  The default is "max".

	杩涚▼鏁伴噺鐨勭‖闄愬埗銆?

  pids.current
	A read-only single value file which exists on non-root cgroups.

	褰撳墠浣嶄簬璇?cgroup 鍙婂叾鍚庝唬涓殑杩涚▼鏁伴噺銆?

  pids.peak
	A read-only single value file which exists on non-root cgroups.

	璇?cgroup 鍙婂叾鍚庝唬涓繘绋嬫暟閲忔浘缁忚揪鍒拌繃鐨勬渶澶у€笺€?

  pids.events
	A read-only flat-keyed file which exists on non-root cgroups. Unless
	specified otherwise, a value change in this file generates a file
	modified event. The following entries are defined.

	  max
		The number of times the cgroup's total number of processes hit the pids.max
		limit (see also pids_localevents).

  pids.events.local
	Similar to pids.events but the fields in the file are local
	to the cgroup i.e. not hierarchical. The file modified event
	generated on this file reflects only the local events.

Organisational operations are not blocked by cgroup policies, so it is
possible to have pids.current > pids.max.  This can be done by either
setting the limit to be smaller than pids.current, or attaching enough
processes to the cgroup such that pids.current is larger than
pids.max.  However, it is not possible to violate a cgroup PID policy
through fork() or clone(). These will return -EAGAIN if the creation
of a new process would cause a cgroup policy to be violated.


### Cpuset


鈥渃puset鈥?鎺у埗鍣ㄦ彁渚涗簡涓€绉嶆満鍒讹紝鐢ㄤ簬灏嗕换鍔℃墍鏀剧疆鐨?CPU 鍜屽唴瀛樿妭鐐归檺鍒朵负浠诲姟褰撳墠 cgroup 涓?cpuset 鎺ュ彛鏂囦欢鎵€鎸囧畾鐨勮祫婧愩€傝繖鍦ㄥぇ鍨?NUMA 绯荤粺涓婂挨鍏舵湁浠峰€硷紝鍥犱负灏嗕綔涓氭斁缃湪缁忚繃鍚堢悊璋冩暣澶у皬鐨勮祫婧愬瓙闆嗕笂銆佸苟杈呬互璋ㄦ厧鐨勫鐞嗗櫒鍜屽唴瀛樻斁缃紝浠ュ噺灏戣法鑺傜偣鍐呭瓨璁块棶涓庝簤鐢紝鍙互鎻愬崌鏁翠綋绯荤粺鎬ц兘銆?

鈥渃puset鈥?鎺у埗鍣ㄦ槸鍒嗗眰鐨勩€傝繖鎰忓懗鐫€鎺у埗鍣ㄤ笉鑳戒娇鐢ㄥ叾鐖惰妭鐐逛笉鍏佽鐨?CPU 鎴栧唴瀛樿妭鐐广€?


#### Cpuset 鎺ュ彛鏂囦欢


  cpuset.cpus
	A read-write multiple values file which exists on non-root
	cpuset-enabled cgroups.

	It lists the requested CPUs to be used by tasks within this
	cgroup.  The actual list of CPUs to be granted, however, is
	subjected to constraints imposed by its parent and can differ
	from the requested CPUs.

	The CPU numbers are comma-separated numbers or ranges.
```

	  # cat cpuset.cpus
	  0-4,6,8-10

	An empty value indicates that the cgroup is using the same
	setting as the nearest cgroup ancestor with a non-empty
	"cpuset.cpus" or all the available CPUs if none is found.

	The value of "cpuset.cpus" stays constant until the next update
	and won't be affected by any CPU hotplug events.

  cpuset.cpus.effective
	A read-only multiple values file which exists on all
	cpuset-enabled cgroups.

	It lists the onlined CPUs that are actually granted to this
	cgroup by its parent.  These CPUs are allowed to be used by
	tasks within the current cgroup.

	If "cpuset.cpus" is empty, the "cpuset.cpus.effective" file shows
	all the CPUs from the parent cgroup that can be available to
	be used by this cgroup.  Otherwise, it should be a subset of
	"cpuset.cpus" unless none of the CPUs listed in "cpuset.cpus"
	can be granted.  In this case, it will be treated just like an
	empty "cpuset.cpus".

	Its value will be affected by CPU hotplug events.

  cpuset.mems
	A read-write multiple values file which exists on non-root
	cpuset-enabled cgroups.

	It lists the requested memory nodes to be used by tasks within
	this cgroup.  The actual list of memory nodes granted, however,
	is subjected to constraints imposed by its parent and can differ
	from the requested memory nodes.

	The memory node numbers are comma-separated numbers or ranges.
	For example::

	  # cat cpuset.mems
	  0-1,3

	An empty value indicates that the cgroup is using the same
	setting as the nearest cgroup ancestor with a non-empty
	"cpuset.mems" or all the available memory nodes if none
	is found.

	The value of "cpuset.mems" stays constant until the next update
	and won't be affected by any memory nodes hotplug events.

	Setting a non-empty value to "cpuset.mems" causes memory of
	tasks within the cgroup to be migrated to the designated nodes if
	they are currently using memory outside of the designated nodes.

	There is a cost for this memory migration.  The migration
	may not be complete and some memory pages may be left behind.
	So it is recommended that "cpuset.mems" should be set properly
	before spawning new tasks into the cpuset.  Even if there is
	a need to change "cpuset.mems" with active tasks, it shouldn't
	be done frequently.

  cpuset.mems.effective
	A read-only multiple values file which exists on all
	cpuset-enabled cgroups.

	It lists the onlined memory nodes that are actually granted to
	this cgroup by its parent. These memory nodes are allowed to
	be used by tasks within the current cgroup.

	If "cpuset.mems" is empty, it shows all the memory nodes from the
	parent cgroup that will be available to be used by this cgroup.
	Otherwise, it should be a subset of "cpuset.mems" unless none of
	the memory nodes listed in "cpuset.mems" can be granted.  In this
	case, it will be treated just like an empty "cpuset.mems".

	Its value will be affected by memory nodes hotplug events.

  cpuset.cpus.exclusive
	A read-write multiple values file which exists on non-root
	cpuset-enabled cgroups.

	It lists all the exclusive CPUs that are allowed to be used
	to create a new cpuset partition.  Its value is not used
	unless the cgroup becomes a valid partition root.  See the
	"cpuset.cpus.partition" section below for a description of what
	a cpuset partition is.

	When the cgroup becomes a partition root, the actual exclusive
	CPUs that are allocated to that partition are listed in
	"cpuset.cpus.exclusive.effective" which may be different
	from "cpuset.cpus.exclusive".  If "cpuset.cpus.exclusive"
	has previously been set, "cpuset.cpus.exclusive.effective"
	is always a subset of it.

	Users can manually set it to a value that is different from
	"cpuset.cpus".	One constraint in setting it is that the list of
	CPUs must be exclusive with respect to "cpuset.cpus.exclusive"
	and "cpuset.cpus.exclusive.effective" of its siblings.	Another
	constraint is that it cannot be a superset of "cpuset.cpus"
	of its sibling in order to leave at least one CPU available to
	that sibling when the exclusive CPUs are taken away.

	For a parent cgroup, any one of its exclusive CPUs can only
	be distributed to at most one of its child cgroups.  Having an
	exclusive CPU appearing in two or more of its child cgroups is
	not allowed (the exclusivity rule).  A value that violates the
	exclusivity rule will be rejected with a write error.

	The root cgroup is a partition root and all its available CPUs
	are in its exclusive CPU set.

  cpuset.cpus.exclusive.effective
	A read-only multiple values file which exists on all non-root
	cpuset-enabled cgroups.

	This file shows the effective set of exclusive CPUs that
	can be used to create a partition root.  The content
	of this file will always be a subset of its parent's
	"cpuset.cpus.exclusive.effective" if its parent is not the root
	cgroup.  It will also be a subset of "cpuset.cpus.exclusive"
	if it is set.  This file should only be non-empty if either
	"cpuset.cpus.exclusive" is set or when the current cpuset is
	a valid partition root.

  cpuset.cpus.isolated
	A read-only and root cgroup only multiple values file.

	This file shows the set of all isolated CPUs used in existing
	isolated partitions. It will be empty if no isolated partition
	is created.

  cpuset.cpus.partition
	A read-write single value file which exists on non-root
	cpuset-enabled cgroups.  This flag is owned by the parent cgroup
	and is not delegatable.

	It accepts only the following input values when written to.

	  ==========	=====================================
	  "member"	鍒嗗尯鐨勯潪鏍规垚鍛?
	  "root"		鍒嗗尯鏍?
	  "isolated"	鏃犺礋杞藉潎琛＄殑鍒嗗尯鏍?
	  ==========	=====================================

	A cpuset partition is a collection of cpuset-enabled cgroups with
	a partition root at the top of the hierarchy and its descendants
	except those that are separate partition roots themselves and
	their descendants.  A partition has exclusive access to the
	set of exclusive CPUs allocated to it.	Other cgroups outside
	of that partition cannot use any CPUs in that set.

	There are two types of partitions - local and remote.  A local
	partition is one whose parent cgroup is also a valid partition
	root.  A remote partition is one whose parent cgroup is not a
	valid partition root itself.

	Writing to "cpuset.cpus.exclusive" is optional for the creation
	of a local partition as its "cpuset.cpus.exclusive" file will
	assume an implicit value that is the same as "cpuset.cpus" if it
	is not set.  Writing the proper "cpuset.cpus.exclusive" values
	down the cgroup hierarchy before the target partition root is
	mandatory for the creation of a remote partition.

	Not all the CPUs requested in "cpuset.cpus.exclusive" can be
	used to form a new partition.  Only those that were present
	in its parent's "cpuset.cpus.exclusive.effective" control
	file can be used.  For partitions created without setting
	"cpuset.cpus.exclusive", exclusive CPUs specified in sibling's
	"cpuset.cpus.exclusive" or "cpuset.cpus.exclusive.effective"
	also cannot be used.

	Currently, a remote partition cannot be created under a local
	partition.  All the ancestors of a remote partition root except
	the root cgroup cannot be a partition root.

	The root cgroup is always a partition root and its state cannot
	be changed.  All other non-root cgroups start out as "member".
	Even though the "cpuset.cpus.exclusive*" and "cpuset.cpus"
	control files are not present in the root cgroup, they are
	implicitly the same as the "/sys/devices/system/cpu/possible"
	sysfs file.

	When set to "root", the current cgroup is the root of a new
	partition or scheduling domain.  The set of exclusive CPUs is
	determined by the value of its "cpuset.cpus.exclusive.effective".

	When set to "isolated", the CPUs in that partition will be in
	an isolated state without any load balancing from the scheduler
	and excluded from the unbound workqueues.  Tasks placed in such
	a partition with multiple CPUs should be carefully distributed
	and bound to each of the individual CPUs for optimal performance.

	A partition root ("root" or "isolated") can be in one of the
	two possible states - valid or invalid.  An invalid partition
	root is in a degraded state where some state information may
	be retained, but behaves more like a "member".

	All possible state transitions among "member", "root" and
	"isolated" are allowed.

	On read, the "cpuset.cpus.partition" file can show the following
	values.

	  =============================	=====================================
	  "member"		鍒嗗尯鐨勯潪鏍规垚鍛?
	  "root"		鍒嗗尯鏍?
	  "isolated"		鏃犺礋杞藉潎琛＄殑鍒嗗尯鏍?
	  "root invalid (<reason>)"	鏃犳晥鐨勫垎鍖烘牴
	  "isolated invalid (<reason>)"	鏃犳晥鐨勯殧绂诲垎鍖烘牴
	  =============================	=====================================

	In the case of an invalid partition root, a descriptive string on
	why the partition is invalid is included within parentheses.

	For a local partition root to be valid, the following conditions
	must be met.

	1) The parent cgroup is a valid partition root.
	2) The "cpuset.cpus.exclusive.effective" file cannot be empty,
	   though it may contain offline CPUs.
	3) The "cpuset.cpus.effective" cannot be empty unless there is
	   no task associated with this partition.

	For a remote partition root to be valid, all the above conditions
	except the first one must be met.

	External events like hotplug or changes to "cpuset.cpus" or
	"cpuset.cpus.exclusive" can cause a valid partition root to
	become invalid and vice versa.	Note that a task cannot be
	moved to a cgroup with empty "cpuset.cpus.effective".

	A valid non-root parent partition may distribute out all its CPUs
	to its child local partitions when there is no task associated
	with it.

	Care must be taken to change a valid partition root to "member"
	as all its child local partitions, if present, will become
	invalid causing disruption to tasks running in those child
	partitions. These inactivated partitions could be recovered if
	their parent is switched back to a partition root with a proper
	value in "cpuset.cpus" or "cpuset.cpus.exclusive".

	Poll and inotify events are triggered whenever the state of
	"cpuset.cpus.partition" changes.  That includes changes caused
	by write to "cpuset.cpus.partition", cpu hotplug or other
	changes that modify the validity status of the partition.
	This will allow user space agents to monitor unexpected changes
	to "cpuset.cpus.partition" without the need to do continuous
	polling.

	A user can pre-configure certain CPUs to an isolated state
	with load balancing disabled at boot time with the "isolcpus"
	kernel boot command line option.  If those CPUs are to be put
	into a partition, they have to be used in an isolated partition.


```
### 璁惧鎺у埗鍣?


璁惧鎺у埗鍣ㄧ鐞嗗璁惧鏂囦欢鐨勮闂€傚畠鏃㈠寘鎷垱寤烘柊鐨勮澶囨枃浠讹紙浣跨敤 mknod锛夛紝涔熷寘鎷鐜版湁璁惧鏂囦欢鐨勮闂€?

Cgroup v2 璁惧鎺у埗鍣ㄦ病鏈夋帴鍙ｆ枃浠讹紝鏋勫缓浜?cgroup BPF 涔嬩笂銆傝鎺у埗瀵硅澶囨枃浠剁殑璁块棶锛岀敤鎴峰彲浠ュ垱寤虹被鍨嬩负 BPF_PROG_TYPE_CGROUP_DEVICE 鐨?bpf 绋嬪簭锛屽苟浣跨敤 BPF_CGROUP_DEVICE 鏍囧織灏嗗畠浠寕鎺ュ埌 cgroup銆傚綋灏濊瘯璁块棶璁惧鏂囦欢鏃讹紝鐩稿簲鐨?BPF 绋嬪簭浼氳鎵ц锛屽苟鏍规嵁杩斿洖鍊煎喅瀹氳灏濊瘯鎴愬姛鎴栦互 -EPERM 澶辫触銆?

涓€涓?BPF_PROG_TYPE_CGROUP_DEVICE 绋嬪簭鎺ュ彈涓€涓寚鍚?bpf_cgroup_dev_ctx 缁撴瀯鐨勬寚閽堬紝璇ョ粨鏋勬弿杩颁簡璁惧璁块棶灏濊瘯锛氳闂被鍨嬶紙mknod/read/write锛変笌璁惧锛堢被鍨嬨€佷富璁惧鍙峰拰娆¤澶囧彿锛夈€傚鏋滅▼搴忚繑鍥?0锛岃灏濊瘯浠?-EPERM 澶辫触锛屽惁鍒欐垚鍔熴€?

BPF_PROG_TYPE_CGROUP_DEVICE 绋嬪簭鐨勪竴涓ず渚嬪彲浠ュ湪鍐呮牳婧愮爜鏍戠殑 tools/testing/selftests/bpf/progs/dev_cgroup.c 涓壘鍒般€?


### RDMA


鈥渞dma鈥?鎺у埗鍣ㄨ皟鑺?RDMA 璧勬簮鐨勫垎閰嶄笌璁拌处銆?


#### RDMA 鎺ュ彛鏂囦欢


  rdma.max
	A readwrite nested-keyed file that exists for all the cgroups
	except root that describes current configured resource limit
	for a RDMA/IB device.

	Lines are keyed by device name and are not ordered.
	Each line contains space separated resource name and its configured
	limit that can be distributed.

	The following nested keys are defined.

	  ==========	=============================
	  hca_handle	HCA 鍙ユ焺鐨勬渶澶ф暟閲?
	  hca_object 	HCA 瀵硅薄鐨勬渶澶ф暟閲?
	  ==========	=============================

```

	  mlx4_0 hca_handle=2 hca_object=2000
	  ocrdma1 hca_handle=3 hca_object=max

  rdma.current
	A read-only file that describes current resource usage.
	It exists for all the cgroup except root.

	An example for mlx4 and ocrdma device follows::

	  mlx4_0 hca_handle=1 hca_object=20
	  ocrdma1 hca_handle=1 hca_object=23

```
### DMEM


鈥渄mem鈥?鎺у埗鍣ㄨ皟鑺傝澶囧唴瀛樺尯鍩熺殑鍒嗛厤涓庤璐︺€傜敱浜庢瘡涓唴瀛樺尯鍩熷彲鑳芥嫢鏈夎嚜宸辩殑椤靛ぇ灏忥紝涓斾笉蹇呯瓑浜庣郴缁熼〉澶у皬锛屽崟浣嶅缁堜负瀛楄妭銆?


#### DMEM 鎺ュ彛鏂囦欢


  dmem.max, dmem.min, dmem.low
	A readwrite nested-keyed file that exists for all the cgroups
	except root that describes current configured resource limit
	for a region.

```

	  drm/0000:03:00.0/vram0 1073741824
	  drm/0000:03:00.0/stolen max

	The semantics are the same as for the memory cgroup controller, and are
	calculated in the same way.

  dmem.capacity
	A read-only file that describes maximum region capacity.
	It only exists on the root cgroup. Not all memory can be
	allocated by cgroups, as the kernel reserves some for
	internal use.

	An example for xe follows::

	  drm/0000:03:00.0/vram0 8514437120
	  drm/0000:03:00.0/stolen 67108864

  dmem.current
	A read-only file that describes current resource usage.
	It exists for all the cgroup except root.

	An example for xe follows::

	  drm/0000:03:00.0/vram0 12550144
	  drm/0000:03:00.0/stolen 8650752

```
### HugeTLB


HugeTLB 鎺у埗鍣ㄥ厑璁搁檺鍒舵瘡涓帶鍒剁粍鐨?HugeTLB 鐢ㄩ噺锛屽苟鍦ㄥ彂鐢熼〉閿欒鏃跺己鍒舵墽琛屾帶鍒跺櫒闄愰銆?


#### HugeTLB 鎺ュ彛鏂囦欢


  hugetlb.<hugepagesize>.current
	Show current usage for "hugepagesize" hugetlb.  It exists for all
	the cgroup except root.

  hugetlb.<hugepagesize>.max
	Set/show the hard limit of "hugepagesize" hugetlb usage.
	The default value is "max".  It exists for all the cgroup except root.

  hugetlb.<hugepagesize>.events
	A read-only flat-keyed file which exists on non-root cgroups.

	  max
		鍥?HugeTLB 闄愰鑰屽鑷村垎閰嶅け璐ョ殑娆℃暟銆?

  hugetlb.<hugepagesize>.events.local
	Similar to hugetlb.<hugepagesize>.events but the fields in the file
	are local to the cgroup i.e. not hierarchical. The file modified event
	generated on this file reflects only the local events.

  hugetlb.<hugepagesize>.numa_stat
	Similar to memory.numa_stat, it shows the numa information of the
        hugetlb pages of <hugepagesize> in this cgroup.  Only active in
        use hugetlb pages are included.  The per-node values are in bytes.

### Misc


Miscellaneous cgroup 涓洪偅浜涙棤娉曞儚鍏跺畠 cgroup 璧勬簮閭ｆ牱琚娊璞＄殑鏍囬噺璧勬簮鎻愪緵璧勬簮闄愬埗涓庤窡韪満鍒躲€傝鎺у埗鍣ㄩ€氳繃 CONFIG_CGROUP_MISC 閰嶇疆閫夐」鍚敤銆?

鍙互閫氳繃 include/linux/misc_cgroup.h 鏂囦欢涓殑 enum misc_res_type{} 鍚戞帶鍒跺櫒娣诲姞璧勬簮锛屽苟閫氳繃 kernel/cgroup/misc.c 鏂囦欢涓殑 misc_res_name[] 娣诲姞鐩稿簲鍚嶇О銆傝祫婧愮殑鎻愪緵鏂瑰繀椤诲湪浣跨敤璇ヨ祫婧愪箣鍓嶈皟鐢?misc_cg_set_capacity() 璁剧疆鍏跺閲忋€?

涓€鏃﹁缃簡瀹归噺锛屽氨鍙互閫氳繃 charge 涓?uncharge API 鏇存柊璧勬簮鐢ㄩ噺銆傛墍鏈変笌 misc 鎺у埗鍣ㄤ氦浜掔殑 API 閮藉湪 include/linux/misc_cgroup.h 涓€?


#### Misc 鎺ュ彛鏂囦欢


Miscellaneous controller provides 3 interface files. If two misc resources (res_a and res_b) are registered then:

  misc.capacity
        A read-only flat-keyed file shown only in the root cgroup.  It shows
        miscellaneous scalar resources available on the platform along with
```

	  $ cat misc.capacity
	  res_a 50
	  res_b 10

  misc.current
        A read-only flat-keyed file shown in the all cgroups.  It shows
        the current usage of the resources in the cgroup and its children.::

	  $ cat misc.current
	  res_a 3
	  res_b 0

  misc.peak
        A read-only flat-keyed file shown in all cgroups.  It shows the
        historical maximum usage of the resources in the cgroup and its
        children.::

	  $ cat misc.peak
	  res_a 10
	  res_b 8

  misc.max
        A read-write flat-keyed file shown in the non root cgroups. Allowed
        maximum usage of the resources in the cgroup and its children.::

	  $ cat misc.max
	  res_a max
	  res_b 4

	Limit can be set by::

	  # echo res_a 1 > misc.max

	Limit can be set to max by::

	  # echo res_a max > misc.max

        Limits can be set higher than the capacity value in the misc.capacity
        file.

  misc.events
	A read-only flat-keyed file which exists on non-root cgroups. The
	following entries are defined. Unless specified otherwise, a value
	change in this file generates a file modified event. All fields in
	this file are hierarchical.

	  max
		璇?cgroup 鐨勮祫婧愮敤閲忓嵆灏嗚秴杩?max 杈圭晫鐨勬鏁般€?

  misc.events.local
        Similar to misc.events but the fields in the file are local to the
        cgroup i.e. not hierarchical. The file modified event generated on
        this file reflects only the local events.

```
#### 杩佺Щ涓庢墍鏈夋潈


涓€涓潅椤规爣閲忚祫婧愪細琚璐﹀埌棣栨浣跨敤瀹冪殑 cgroup锛屽苟涓€鐩翠繚鎸佽璐﹀埌璇ヨ祫婧愯閲婃斁涓烘銆傚皢杩涚▼杩佺Щ鍒颁笉鍚岀殑 cgroup 骞朵笉浼氭妸璁拌处杞Щ鍒拌繘绋嬫墍绉诲姩鍒扮殑鐩殑 cgroup銆?


### 鍏朵粬


#### perf_event


perf_event 鎺у埗鍣紝濡傛灉鏈寕杞藉埌閬楃暀灞傜骇锛屼細鑷姩鍦?v2 灞傜骇涓婂惎鐢紝浠ヤ究 perf 浜嬩欢濮嬬粓鑳芥寜 cgroup v2 璺緞杩涜杩囨护銆傚湪 v2 灞傜骇琚～鍏呬箣鍚庯紝璇ユ帶鍒跺櫒浠嶅彲琚Щ鍔ㄥ埌閬楃暀灞傜骇銆?


### 闈炶鑼冩€т俊鎭?


鏈妭鍖呭惈涓嶈瑙嗕负绋冲畾鍐呮牳 API 涓€閮ㄥ垎銆佸洜鑰屽彲鑳藉彂鐢熷彉鏇寸殑淇℃伅銆?


#### CPU 鎺у埗鍣ㄦ牴 cgroup 杩涚▼琛屼负


鍦ㄦ牴 cgroup 涓垎閰?CPU 鍛ㄦ湡鏃讹紝璇?cgroup 涓殑姣忎釜绾跨▼閮借褰撲綔鏄敱鏍?cgroup 鐨勪竴涓嫭绔嬪瓙 cgroup 鎵胯浇鐨勩€傝繖涓瓙 cgroup 鐨勬潈閲嶅彇鍐充簬鍏剁嚎绋嬬殑 nice 绾у埆銆?

鍏充簬杩欑鏄犲皠鐨勭粏鑺傦紝璇峰弬闃?kernel/sched/core.c 鏂囦欢涓殑 sched_prio_to_weight 鏁扮粍锛堣鏁扮粍涓殑鍊煎簲閫傚綋缂╂斁锛屼娇寰椾腑鎬х殑鈥斺€攏ice 0鈥斺€斿€间负 100 鑰岄潪 1024锛夈€?


#### IO 鎺у埗鍣ㄦ牴 cgroup 杩涚▼琛屼负


鏍?cgroup 涓殑杩涚▼鎵胯浇浜庝竴涓殣寮忕殑鍙跺瓙瀛愯妭鐐逛腑銆傚湪鍒嗛厤 IO 璧勬簮鏃讹紝浼氭妸杩欎釜闅愬紡瀛愯妭鐐瑰綋浣滄槸鏍?cgroup 鐨勪竴涓櫘閫氬瓙 cgroup 鏉ヨ€冭檻锛屽叾鏉冮噸鍊间负 200銆?


## 鍛藉悕绌洪棿


### 鍩虹


cgroup 鍛藉悕绌洪棿鎻愪緵浜嗕竴绉嶆満鍒讹紝鐢ㄤ簬铏氭嫙鍖?鈥?proc/$PID/cgroup鈥?鏂囦欢涓?cgroup 鎸傝浇鐨勮鍥俱€侰LONE_NEWCGROUP clone 鏍囧織鍙互涓?clone(2) 鍜?unshare(2) 涓€璧蜂娇鐢紝浠ュ垱寤轰竴涓柊鐨?cgroup 鍛藉悕绌洪棿銆傝繍琛屽湪 cgroup 鍛藉悕绌洪棿鍐呴儴鐨勮繘绋嬶紝鍏?鈥?proc/$PID/cgroup鈥?杈撳嚭浼氳闄愬埗涓?cgroupns 鏍广€俢groupns 鏍规槸鍒涘缓 cgroup 鍛藉悕绌洪棿鏃惰繘绋嬬殑 cgroup銆?

鍦ㄦ病鏈?cgroup 鍛藉悕绌洪棿鐨勬儏鍐典笅锛屸€?proc/$PID/cgroup鈥?鏂囦欢鏄剧ず杩涚▼ cgroup 鐨勫畬鏁磋矾寰勩€傚湪瀹瑰櫒璁剧疆涓紝涓€缁?cgroup 鍜屽懡鍚嶇┖闂存棬鍦ㄩ殧绂昏繘绋嬶紝鈥?proc/$PID/cgroup鈥?鏂囦欢鍙兘浼氭硠婕忔綔鍦ㄧ殑绯荤粺绾т俊鎭?
```

  # cat /proc/self/cgroup
  0::/batchjobs/container_id1

```
璺緞 鈥?batchjobs/container_id1鈥?鍙瑙嗕负绯荤粺鏁版嵁锛屼笉甯屾湜鏆撮湶缁欒闅旂鐨勮繘绋嬨€俢group 鍛藉悕绌洪棿鍙敤浜庨檺鍒舵璺緞鐨勫彲瑙佹€с€備緥濡傦紝鍦ㄤ箣鍓?
```

  # ls -l /proc/self/ns/cgroup
  lrwxrwxrwx 1 root root 0 2014-07-15 10:37 /proc/self/ns/cgroup -> cgroup:[4026531835]
  # cat /proc/self/cgroup
  0::/batchjobs/container_id1

```
```

  # ls -l /proc/self/ns/cgroup
  lrwxrwxrwx 1 root root 0 2014-07-15 10:35 /proc/self/ns/cgroup -> cgroup:[4026532183]
  # cat /proc/self/cgroup
  0::/

```
When some thread from a multi-threaded process unshares its cgroup
namespace, the new cgroupns gets applied to the entire process (all
the threads).  This is natural for the v2 hierarchy; however, for the
legacy hierarchies, this may be unexpected.

A cgroup namespace is alive as long as there are processes inside or
mounts pinning it.  When the last usage goes away, the cgroup
namespace is destroyed.  The cgroupns root and the actual cgroups
remain.


### 鏍逛笌瑙嗗浘


cgroup 鍛藉悕绌洪棿鐨?鈥榗groupns root鈥?鏄皟鐢?unshare(2) 鐨勮繘绋嬫墍杩愯鐨?cgroup銆備緥濡傦紝濡傛灉浣嶄簬 /batchjobs/container_id1 cgroup 涓殑涓€涓繘绋嬭皟鐢?unshare锛宑group /batchjobs/container_id1 灏辨垚涓?cgroupns 鏍广€傚浜?init_cgroup_ns锛岃繖灏辨槸鐪熸鐨勬牴锛堚€?鈥欙級cgroup銆?

鍗充娇鍛藉悕绌洪棿鍒涘缓鑰?
```

  #~/unshare -c # unshare cgroupns in some cgroup
  # cat /proc/self/cgroup
  0::/
  # mkdir sub_cgrp_1
  # echo 0 > sub_cgrp_1/cgroup.procs
  # cat /proc/self/cgroup
  0::/sub_cgrp_1

```
Each process gets its namespace-specific view of "/proc/$PID/cgroup"

Processes running inside the cgroup namespace will be able to see
cgroup paths (in /proc/self/cgroup) only inside their root cgroup.
```

  # sleep 100000 &
  [1] 7353
  # echo 7353 > sub_cgrp_1/cgroup.procs
  # cat /proc/7353/cgroup
  0::/sub_cgrp_1

```
From the initial cgroup namespace, the real cgroup path will be
```

  $ cat /proc/7353/cgroup
  0::/batchjobs/container_id1/sub_cgrp_1

```
From a sibling cgroup namespace (that is, a namespace rooted at a
different cgroup), the cgroup path relative to its own cgroup
namespace root will be shown.  For instance, if PID 7353's cgroup
```

  # cat /proc/7353/cgroup
  0::/../container_id2/sub_cgrp_1

```
Note that the relative path always starts with '/' to indicate that
its relative to the cgroup namespace root of the caller.


### 杩佺Щ涓?setns(2)


Processes inside a cgroup namespace can move into and out of the
namespace root if they have proper access to external cgroups.  For
example, from inside a namespace with cgroupns root at
/batchjobs/container_id1, and assuming that the global hierarchy is
```

  # cat /proc/7353/cgroup
  0::/sub_cgrp_1
  # echo 7353 > batchjobs/container_id2/cgroup.procs
  # cat /proc/7353/cgroup
  0::/../container_id2

```
Note that this kind of setup is not encouraged.  A task inside cgroup
namespace should only be exposed to its own cgroupns hierarchy.

setns(2) to another cgroup namespace is allowed when:

(a) the process has CAP_SYS_ADMIN against its current user namespace
(b) the process has CAP_SYS_ADMIN against the target cgroup
    namespace's userns

No implicit cgroup changes happen with attaching to another cgroup
namespace.  It is expected that the someone moves the attaching
process under the target cgroup namespace root.


### 涓庡叾浠栧懡鍚嶇┖闂寸殑浜や簰


Namespace specific cgroup hierarchy can be mounted by a process
```

  # mount -t cgroup2 none $MOUNT_POINT

```
This will mount the unified cgroup hierarchy with cgroupns root as the
filesystem root.  The process needs CAP_SYS_ADMIN against its user and
mount namespaces.

The virtualization of /proc/self/cgroup file combined with restricting
the view of cgroup hierarchy by namespace-private cgroupfs mount
provides a properly isolated cgroup view inside the container.


## 鍐呮牳缂栫▼鐩稿叧淇℃伅


鏈妭鍖呭惈鍦ㄤ笌 cgroup 浜や簰涓嶅彲閬垮厤涔嬪鐨勫唴鏍哥紪绋嬩俊鎭€俢group 鏍稿績涓庢帶鍒跺櫒涓嶅湪瑕嗙洊鑼冨洿鍐呫€?


### 鍥炲啓鐩稿叧鐨勬枃浠剁郴缁熸敮鎸?


涓€涓枃浠剁郴缁熷彲浠ラ€氳繃鏇存柊 address_space_operations->writepages() 鏉ユ敞閲?bio锛屼粠鑰屾敮鎸?cgroup 鍥炲啓锛屼娇鐢ㄤ互涓嬩袱涓嚱鏁般€?

  wbc_init_bio(@wbc, @bio)
	Should be called for each bio carrying writeback data and
	associates the bio with the inode's owner cgroup and the
	corresponding request queue.  This must be called after
	a queue (device) has been associated with the bio and
	before submission.

  wbc_account_cgroup_owner(@wbc, @folio, @bytes)
	Should be called for each data segment being written out.
	While this function doesn't care exactly when it's called
	during the writeback session, it's the easiest and most
	natural to call it as data segments are added to a bio.

With writeback bio's annotated, cgroup support can be enabled per
super_block by setting SB_I_CGROUPWB in ->s_iflags.  This allows for
selective disabling of cgroup writeback support which is helpful when
certain filesystem features, e.g. journaled data mode, are
incompatible.

wbc_init_bio() binds the specified bio to its cgroup.  Depending on
the configuration, the bio may be executed at a lower priority and if
the writeback session is holding shared resources, e.g. a journal
entry, may lead to priority inversion.  There is no one easy solution
for the problem.  Filesystems can try to work around specific problem
cases by skipping wbc_init_bio() and using bio_associate_blkg()
directly.


## 宸插純鐢ㄧ殑 v1 鏍稿績鐗规€?


- 涓嶆敮鎸佸寘鎷懡鍚嶅眰绾у湪鍐呯殑澶氶噸灞傜骇銆?

- 涓嶆敮鎸佹墍鏈?v1 鎸傝浇閫夐」銆?

- 绉婚櫎浜?鈥渢asks鈥?鏂囦欢锛屸€渃group.procs鈥?涔熶笉鎺掑簭銆?

- 绉婚櫎浜?鈥渃group.clone_children鈥濄€?

- 瀵逛簬 v2锛?proc/cgroups 娌℃湁鎰忎箟銆傝鏀圭敤鏍硅妭鐐逛笂鐨?鈥渃group.controllers鈥?鎴?鈥渃group.stat鈥?鏂囦欢銆?


## v1 瀛樺湪鐨勯棶棰樺強 v2 鐨勮璁＄悊鐢?


### 澶氶噸灞傜骇


cgroup v1 鍏佽浠绘剰鏁伴噺鐨勫眰绾э紝涓旀瘡涓眰绾у彲浠ユ壙杞戒换鎰忔暟閲忕殑鎺у埗鍣ㄣ€傝櫧鐒惰繖鐪嬩技鎻愪緵浜嗛珮搴︾殑鐏垫椿鎬э紝浣嗗湪瀹炶返涓苟鏃犵敤澶勩€?

渚嬪锛岀敱浜庢瘡涓帶鍒跺櫒鍙湁涓€涓疄渚嬶紝鍍?freezer 杩欑被鍦ㄦ墍鏈夊眰绾ч兘鍙兘鏈夌敤鐨勫疄鐢ㄥ瀷鎺у埗鍣ㄥ彧鑳界敤浜庡叾涓竴涓€傜敱浜庢帶鍒跺櫒涓€鏃﹀眰绾ц濉厖灏辨棤娉曠Щ鍔ㄥ埌鍙︿竴涓眰绾э紝杩欎釜闂鏇村姞涓ラ噸銆傚彟涓€涓棶棰樻槸锛岀粦瀹氬埌鏌愪釜灞傜骇鐨勬墍鏈夋帶鍒跺櫒琚揩鎷ユ湁瀹屽叏鐩稿悓鐨勫眰娆¤鍥俱€備笉鍙兘鏍规嵁鐗瑰畾鎺у埗鍣ㄦ潵鏀瑰彉绮掑害銆?

鍦ㄥ疄璺典腑锛岃繖浜涢棶棰樹弗閲嶉檺鍒朵簡鍝簺鎺у埗鍣ㄨ兘琚斁鍦ㄥ悓涓€涓眰绾т笂锛屽ぇ澶氭暟閰嶇疆鏈€缁堥兘閫夋嫨灏嗘瘡涓帶鍒跺櫒鏀惧湪鑷繁鐨勫眰绾т笂銆傚彧鏈夌揣瀵嗙浉鍏崇殑閭ｄ簺锛屼緥濡?cpu 鍜?cpuacct 鎺у埗鍣紝鏀惧湪鍚屼竴灞傜骇鎵嶆湁鎰忎箟銆傝繖閫氬父鎰忓懗鐫€姣忓綋闇€瑕佽繘琛屽眰绾х鐞嗘搷浣滄椂锛岀敤鎴风┖闂翠笉寰椾笉鍦ㄥ涓浉浼肩殑灞傜骇涓婇噸澶嶇浉鍚岀殑姝ラ銆?

姝ゅ锛屽澶氶噸灞傜骇鐨勬敮鎸佷唬浠烽珮鏄傘€傚畠鏋佸ぇ鍦板鏉傚寲浜?cgroup 鏍稿績瀹炵幇锛屼絾鏇撮噸瑕佺殑鏄紝瀵瑰閲嶅眰绾х殑鏀寔闄愬埗浜?cgroup 鐨勪竴鑸敤娉曚互鍙婃帶鍒跺櫒鑳藉鍋氱殑浜嬫儏銆?

灞傜骇鏁伴噺娌℃湁涓婇檺锛岃繖鎰忓懗鐫€涓€涓嚎绋嬬殑 cgroup 褰掑睘鏃犳硶鐢ㄦ湁闄愰暱搴︽潵鎻忚堪銆傞敭鍙兘鍖呭惈浠绘剰鏁伴噺鐨勬潯鐩紝涓旈暱搴︿笉闄愶紝杩欎娇鍏舵搷浣滆捣鏉ラ潪甯哥鎷欙紝骞跺鑷翠簡涓撻棬涓轰簡鏍囪瘑褰掑睘鑰屽瓨鍦ㄧ殑鎺у埗鍣ㄥ鍔狅紝鑰岃繖鍙堝弽杩囨潵鍔犲墽浜嗗眰绾ф暟閲忔縺澧炵殑鍘熷闂銆?

鍙﹀锛岀敱浜庝竴涓帶鍒跺櫒鏃犳硶瀵瑰叾瀹冩帶鍒跺櫒鍙兘鎵€鍦ㄧ殑灞傜骇鎷撴墤鏈変换浣曢鏈燂紝姣忎釜鎺у埗鍣ㄩ兘涓嶅緱涓嶅亣璁炬墍鏈夊叾瀹冩帶鍒跺櫒閮芥寕鎺ュ湪瀹屽叏姝ｄ氦鐨勫眰绾т笂銆傝繖浣垮緱鎺у埗鍣ㄤ箣闂存棤娉曞崗浣滐紝鎴栬€呰嚦灏戦潪甯哥鎷欍€?

鍦ㄥぇ澶氭暟浣跨敤鍦烘櫙涓紝灏嗘帶鍒跺櫒鏀惧湪褰兼瀹屽叏姝ｄ氦鐨勫眰绾т笂骞堕潪蹇呰銆傞€氬父鎵€闇€瑕佺殑鏄牴鎹壒瀹氭帶鍒跺櫒鎷ユ湁涓嶅悓绮掑害绾у埆鐨勮兘鍔涖€傛崲瑷€涔嬶紝浠庡彾瀛愬悜鏍规柟鍚戠湅锛屽眰绾у彲鑳借鎶樺彔銆備緥濡傦紝鏌愪釜閰嶇疆鍙兘涓嶅叧蹇冭秴杩囨煇涓€灞備箣鍚庡唴瀛樺浣曞垎閰嶏紝浣嗕粛鎯虫帶鍒?CPU 鍛ㄦ湡濡備綍鍒嗛厤銆?


### 绾跨▼绮掑害


cgroup v1 鍏佽涓€涓繘绋嬬殑绾跨▼灞炰簬涓嶅悓鐨?cgroup銆傝繖瀵规煇浜涙帶鍒跺櫒鑰岃█娌℃湁鎰忎箟锛岃繖浜涙帶鍒跺櫒鏈€缁堝疄鐜颁簡涓嶅悓鐨勬柟寮忔潵蹇界暐杩欑鎯呭喌锛涗絾鏇撮噸瑕佺殑鏄紝瀹冩ā绯婁簡鏆撮湶缁欏崟涓簲鐢ㄧ▼搴忕殑 API 涓庣郴缁熺鐞嗘帴鍙ｄ箣闂寸殑鐣岄檺銆?

涓€鑸€岃█锛岃繘绋嬪唴閮ㄧ煡璇嗗彧鏈夎繘绋嬭嚜韬彲鐢紱鍥犳锛屼笌杩涚▼鐨勬湇鍔＄骇缁勭粐涓嶅悓锛屽杩涚▼绾跨▼杩涜鍒嗙被闇€瑕佹嫢鏈夌洰鏍囪繘绋嬬殑銆佽搴旂敤绋嬪簭鐨勭Н鏋佸弬涓庛€?

cgroup v1 鏈変竴涓畾涔夋ā绯婄殑濮旀墭妯″瀷锛屽畠涓庣嚎绋嬬矑搴︾粨鍚堣婊ョ敤銆俢group 琚鎵樼粰鍗曚釜搴旂敤绋嬪簭锛屼互渚垮畠浠兘澶熷垱寤哄苟绠＄悊鑷繁鐨勫瓙灞傛锛屽苟鎺у埗娌胯繖浜涘瓙灞傛鐨勮祫婧愬垎閰嶃€傝繖瀹為檯涓婂皢 cgroup 鎻愬崌鍒颁簡绫讳技浜庣郴缁熻皟鐢ㄧ殑 API 鐨勫湴浣嶏紝鏆撮湶缁欐櫘閫氱▼搴忋€?

棣栧厛锛宑group 浣滀负涓€涓帴鍙ｏ紝鍏舵湰璐ㄤ笉瓒充互浠ユ绉嶆柟寮忔毚闇层€備竴涓繘绋嬭璁块棶鑷繁鐨勬棆閽紝蹇呴』浠?/proc/self/cgroup 涓彁鍙栫洰鏍囧眰绾т笂鐨勮矾寰勶紝灏嗘棆閽悕闄勫姞鍒拌矾寰勫悗鏋勯€犲嚭瀹屾暣璺緞锛屾墦寮€鐒跺悗璇诲彇鍜?鎴栧啓鍏ャ€傝繖涓嶄粎鏋佸叾绗ㄦ嫏涓斾笉甯歌锛岃€屼笖鏈川涓婃槸瀛樺湪绔炴€佺殑銆傛病鏈夊父瑙勬柟娉曡兘瀹氫箟璺ㄦ墍闇€姝ラ鐨勪簨鍔★紝涔熸病鏈変换浣曚笢瑗胯兘淇濊瘉璇ヨ繘绋嬪疄闄呬笂鏄湪鎿嶄綔鑷繁鐨勫瓙灞傛銆?

cgroup 鎺у埗鍣ㄥ疄鐜颁簡澶ч噺姘歌繙涓嶄細琚帴鍙椾负鍏叡 API 鐨勬棆閽紝鍥犱负瀹冧滑鍙槸鍚戠郴缁熺鐞嗕吉鏂囦欢绯荤粺娣诲姞鎺у埗鏃嬮挳銆俢group 鏈€缁堟湁浜嗘湭琚纭娊璞℃垨绮剧偧銆佺洿鎺ユ毚闇插唴鏍稿唴閮ㄧ粏鑺傜殑鎺ュ彛鏃嬮挳銆傝繖浜涙棆閽€氳繃瀹氫箟涓嶆竻鐨勫鎵樻満鍒舵毚闇茬粰鍗曚釜搴旂敤绋嬪簭锛屽疄闄呬笂灏?cgroup 婊ョ敤涓哄疄鐜板叕鍏?API 鐨勬嵎寰勶紝鑰岀粫杩囦簡鎵€闇€鐨勪弗鏍煎鏌ャ€?

杩欏鐢ㄦ埛绌洪棿鍜屽唴鏍搁兘鏄棝鑻︾殑銆傜敤鎴风┖闂存渶缁堝緱鍒颁簡琛屼负寮傚父涓旀娊璞′笉鑹殑鎺ュ彛锛岃€屽唴鏍告毚闇插苟閿佸畾浜嗘棤鎰忎腑鐨勬瀯閫犮€?


### 鍐呴儴鑺傜偣涓庣嚎绋嬩箣闂寸殑绔炰簤


cgroup v1 鍏佽绾跨▼浣嶄簬浠讳綍 cgroup 涓紝杩欓€犳垚浜嗕竴涓湁瓒ｇ殑闂锛氬睘浜庣埗 cgroup 鍙婂叾瀛?cgroup 鐨勭嚎绋嬬浉浜掔珵浜夎祫婧愩€傝繖寰堢碂绯曪紝鍥犱负涓ょ涓嶅悓绫诲瀷鐨勫疄浣撳湪绔炰簤锛岃€屼笖娌℃湁鏄庢樉鐨勬柟娉曟潵瑙ｅ喅瀹冦€備笉鍚岀殑鎺у埗鍣ㄥ仛娉曚笉鍚屻€?

cpu 鎺у埗鍣ㄥ皢绾跨▼鍜?cgroup 瑙嗕负绛変环锛屽苟鎶?nice 绾у埆鏄犲皠鍒?cgroup 鏉冮噸銆傝繖鍦ㄦ煇浜涙儏鍐典笅琛屽緱閫氾紝浣嗗綋瀛愯妭鐐规兂瑕佽鍒嗛厤鐗瑰畾姣斾緥鐨?CPU 鍛ㄦ湡銆佽€屽唴閮ㄧ嚎绋嬫暟閲忔尝鍔ㄦ椂灏变細澶辨晥鈥斺€旀瘮渚嬮殢鐫€绔炰簤瀹炰綋鏁伴噺鐨勬尝鍔ㄨ€屼笉鏂彉鍖栥€傝繕鏈夊叾瀹冮棶棰樸€備粠 nice 绾у埆鍒版潈閲嶇殑鏄犲皠鏃笉鏄庢樉涔熶笉閫氱敤锛岃€屼笖鏈夊悇绉嶅叾瀹冩棆閽绾跨▼鏍规湰涓嶅彲鐢ㄣ€?

io 鎺у埗鍣ㄤ负姣忎釜 cgroup 闅愬紡鍒涘缓浜嗕竴涓殣钘忕殑鍙跺瓙鑺傜偣鏉ユ壙杞界嚎绋嬨€傝繖涓殣钘忓彾瀛愭嫢鏈夎嚜宸辨墍鏈変互 `leaf_` 涓哄墠缂€鐨勬棆閽壇鏈€傝櫧鐒惰繖鍏佽瀵瑰唴閮ㄧ嚎绋嬭繘琛岀瓑浠风殑鎺у埗锛屼絾甯︽湁涓ラ噸鐨勭己闄枫€傚畠鎬绘槸澧炲姞浜嗕竴灞傚師鏈笉蹇呰鐨勫祵濂楋紝浣挎帴鍙ｅ彉寰楁贩涔憋紝骞舵樉钁楀鏉傚寲浜嗗疄鐜般€?

鍐呭瓨鎺у埗鍣ㄦ病鏈夊姙娉曟帶鍒跺唴閮ㄤ换鍔′笌瀛?cgroup 涔嬮棿鍙戠敓鐨勬儏鍐碉紝鍏惰涓轰篃娌℃湁琚竻鏅板畾涔夈€傛浘鏈変汉灏濊瘯娣诲姞涓存椂琛屼负涓庢棆閽潵閽堝鐗瑰畾宸ヤ綔璐熻浇瑁佸壀琛屼负锛屼絾杩欎細瀵艰嚧闀挎湡鏋侀毦瑙ｅ喅鐨勯棶棰樸€?

澶氫釜鎺у埗鍣ㄩ兘鍦ㄤ笌鍐呴儴浠诲姟浣滄枟浜夛紝骞舵兂鍑轰簡涓嶅悓鐨勫簲瀵规柟娉曪紱涓嶅垢鐨勬槸锛屾墍鏈夎繖浜涙柟娉曢兘鏈変弗閲嶇己闄凤紝鑰屼笖锛屽樊寮傚法澶х殑琛屼负浣?cgroup 浣滀负涓€涓暣浣撻珮搴︿笉涓€鑷淬€?

杩欐樉鐒舵槸涓€涓渶瑕佷粠 cgroup 鏍稿績浠ョ粺涓€鏂瑰紡瑙ｅ喅鐨勯棶棰樸€?


### 鍏朵粬鎺ュ彛闂


cgroup v1 鍦ㄦ病鏈夌洃鐫ｇ殑鎯呭喌涓嬪彂灞曪紝浜х敓浜嗗ぇ閲忕壒娈婅涓轰笌涓嶄竴鑷淬€俢group 鏍稿績渚х殑涓€涓棶棰樻槸绌?cgroup 濡備綍琚€氱煡鈥斺€斾负姣忎釜浜嬩欢 fork 骞舵墽琛屼竴涓敤鎴风┖闂磋緟鍔╀簩杩涘埗銆備簨浠舵姇閫掓棦闈為€掑綊涔熶笉鍙鎵樸€傝鏈哄埗鐨勫眬闄愭€ц繕瀵艰嚧浜嗗唴鏍稿唴鐨勪簨浠舵姇閫掕繃婊ゆ満鍒讹紝杩涗竴姝ュ鏉傚寲浜嗘帴鍙ｃ€?

鎺у埗鍣ㄦ帴鍙ｄ篃鏈夐棶棰樸€備竴涓瀬绔殑渚嬪瓙鏄帶鍒跺櫒瀹屽叏蹇界暐灞傛缁勭粐锛屾妸鎵€鏈?cgroup 閮藉綋浣滅洿鎺ヤ綅浜庢牴 cgroup 涔嬩笅銆備竴浜涙帶鍒跺櫒鍚戠敤鎴风┖闂存毚闇蹭簡澶ч噺涓嶄竴鑷寸殑瀹炵幇缁嗚妭銆?

璺ㄦ帶鍒跺櫒涔嬮棿涔熺己涔忎竴鑷存€с€傚綋鍒涘缓涓€涓柊鐨?cgroup 鏃讹紝涓€浜涙帶鍒跺櫒榛樿涓嶆柦鍔犻澶栭檺鍒讹紝鑰屽彟涓€浜涘垯鍦ㄨ鏄惧紡閰嶇疆涔嬪墠绂佹浠讳綍璧勬簮浣跨敤銆傚悓涓€绫诲瀷鎺у埗鐨勯厤缃棆閽娇鐢ㄤ簡宸紓宸ㄥぇ鐨勫懡鍚嶆柟妗堜笌鏍煎紡銆傜粺璁′笌淇℃伅鏃嬮挳鍛藉悕闅忔剰锛屽嵆浣垮湪鍚屼竴鎺у埗鍣ㄥ唴涔熶娇鐢ㄤ簡涓嶅悓鏍煎紡涓庡崟浣嶃€?

cgroup v2 鍦ㄩ€傚綋涔嬪寤虹珛浜嗛€氱敤绾﹀畾锛屽苟鏇存柊鎺у埗鍣紝浣垮畠浠毚闇叉渶灏忎笖涓€鑷寸殑鎺ュ彛銆?


### 鎺у埗鍣ㄩ棶棰樺強瀵圭瓥


#### 鍐呭瓨


鍘熷鐨勮緝浣庤竟鐣屸€斺€旇蒋闄愬埗锛坰oft limit锛夆€斺€旇瀹氫箟涓轰竴涓粯璁ゆ湭璁剧疆鐨勯檺棰濄€傜粨鏋滐紝鍏ㄥ眬鍥炴敹浼樺厛閫夋嫨鐨?cgroup 闆嗗悎鏄€夋嫨鍔犲叆锛坥pt-in锛夌殑锛岃€岄潪閫夋嫨閫€鍑猴紙opt-out锛夈€備紭鍖栬繖浜涘ぇ澶氫负璐熼潰鐨勬煡鎵剧殑浠ｄ环濡傛涔嬮珮锛屼互鑷充簬璇ュ疄鐜板敖绠¤妯″簽澶э紝鍗磋繛鍩烘湰鐨勭悊鎯宠涓洪兘鏃犳硶鎻愪緵銆傞鍏堬紝杞檺鍒舵病鏈夊眰娆″惈涔夈€傛墍鏈夊凡閰嶇疆鐨勭粍琚粍缁囧湪涓€涓叏灞€ rbtree 涓紝骞惰褰撲綔骞崇瓑鐨勫悓浼村寰咃紝鏃犺瀹冧滑浣嶄簬灞傛涓殑浣曞銆傝繖浣垮緱瀛愭爲濮旀墭鍙樺緱涓嶅彲鑳姐€傚叾娆★紝杞檺鍒跺洖鏀惰繃绋嬭繃浜庢縺杩涳紝涓嶄粎缁欑郴缁熷紩鍏ヤ簡楂樺垎閰嶅欢杩燂紝杩樺洜杩囧害鍥炴敹褰卞搷浜嗙郴缁熸€ц兘锛屼互鑷充簬璇ョ壒鎬у彉寰楅€傚緱鍏跺弽銆?

鍙︿竴鏂归潰锛宮emory.low 杈圭晫鏄竴涓嚜涓婅€屼笅鍒嗛厤鐨勫偍澶囥€傚綋涓€涓?cgroup 澶勪簬鍏?effective low 涔嬪唴鏃讹紝瀹冧韩鏈夊洖鏀朵繚鎶わ紝杩欎娇寰楀瓙鏍戝鎵樻垚涓哄彲鑳姐€傚綋瀹冮珮浜庡叾 effective low 鏃讹紝瀹冭繕浜湁涓庡叾瓒呭嚭閲忔垚姣斾緥鐨勫洖鏀跺帇鍔涖€?

鍘熷鐨勮緝楂樿竟鐣屸€斺€旂‖闄愬埗锛坔ard limit锛夆€斺€旇瀹氫箟涓轰竴涓弗鏍肩殑闄愬埗锛屽嵆浣垮繀椤昏皟鐢?OOM killer 涔熶笉寰楅€€璁┿€備絾杩欐€讳綋涓婅繚鑳屼簡鍏呭垎鍒╃敤鍙敤鍐呭瓨鐨勭洰鏍囥€傚伐浣滆礋杞界殑鍐呭瓨娑堣€楀湪杩愯鏈熼棿鏄彉鍖栫殑锛岃繖瑕佹眰鐢ㄦ埛杩涜杩囧害鎵胯銆備絾浣跨敤涓ユ牸鐨勭‖涓婇檺杩涜杩囧害鎵胯锛岃涔堥渶瑕佺浉褰撳噯纭湴棰勬祴宸ヤ綔闆嗗ぇ灏忥紝瑕佷箞闇€瑕佸湪闄愰涓婄暀鏈変綑閲忋€傜敱浜庡伐浣滈泦澶у皬浼扮畻鏃㈠洶闅惧張瀹规槗鍑洪敊锛岃€屼及绠楅敊璇細瀵艰嚧 OOM kill锛屽ぇ澶氭暟鐢ㄦ埛鍊惧悜浜庨€夋嫨杈冧负瀹芥澗鐨勯檺棰濓紝鏈€缁堟氮璐逛簡瀹濊吹璧勬簮銆?

鍙︿竴鏂归潰锛宮emory.high 杈圭晫鍙互璁剧疆寰椾繚瀹堝緱澶氥€傚綋琚Е鍙婏紝瀹冮€氳繃寮哄埗鍒嗛厤杩涘叆鐩存帴鍥炴敹浠ユ秷瑙ｈ秴棰濋儴鍒嗭紝浣嗕粠涓嶈皟鐢?OOM killer銆傚洜姝わ紝涓€涓缃緱杩囦簬婵€杩涚殑 high 杈圭晫涓嶄細缁堟杩涚▼锛岃€屾槸瀵艰嚧鎬ц兘閫愭笎涓嬮檷銆傜敤鎴峰彲浠ョ洃鎺ц繖涓€鐐瑰苟鍋氬嚭淇锛岀洿鍒版壘鍒颁粛鑳芥彁渚涘彲鎺ュ彈鎬ц兘鐨勬渶灏忓唴瀛樺崰鐢ㄣ€?

鍦ㄦ瀬绔儏鍐典笅锛屽瓨鍦ㄥぇ閲忓苟鍙戝垎閰嶄笖缁勫唴鍥炴敹杩涘睍瀹屽叏鍋滄粸鏃讹紝high 杈圭晫鍙兘琚獊鐮淬€備絾鍗充究濡傛锛屾弧瓒冲垎閰嶉渶姹備篃澶у浼樹簬浠庡叾瀹冪粍鎴栫郴缁熷叾浣欓儴鍒嗗彲鐢ㄧ殑浣欓噺涓弧瓒筹紝鑰岄潪鏉€姝昏缁勩€傚惁鍒欙紝memory.max 灏卞湪閭ｉ噷闄愬埗杩欑被婧㈠嚭锛屽苟鏈€缁?containment 鏈?bug 鐢氳嚦鎭舵剰鐨勫簲鐢ㄧ▼搴忋€?

灏嗗師濮嬬殑 memory.limit_in_bytes 璁剧疆鍒颁綆浜庡綋鍓嶇敤閲忎細閬亣绔炴€佹潯浠讹紝骞跺彂鐨勮璐﹀彲鑳藉鑷撮檺棰濊缃け璐ャ€傝€?memory.max 鍒欎細鍏堣缃檺棰濅互闃绘鏂扮殑璁拌处锛岀劧鍚庡洖鏀跺苟 OOM kill锛岀洿鍒拌揪鍒版柊闄愰鈥斺€旀垨鑰呭啓鍏?memory.max 鐨勪换鍔¤鏉€姝汇€?

鍚堝苟鐨勫唴瀛?浜ゆ崲璁拌处涓庨檺鍒讹紝琚浛鎹负瀵逛氦鎹㈢┖闂寸殑鐪熸鎺у埗銆?

鍘熷 cgroup 璁捐涓娇鐢ㄥ悎骞跺唴瀛?浜ゆ崲鏈哄埗鐨勪富瑕佽鐐规槸锛氬叏灞€鎴栫埗绾у帇鍔涙€昏兘浜ゆ崲鍑哄瓙缁勭殑鍏ㄩ儴鍖垮悕鍐呭瓨锛屾棤璁哄瓙缁勮嚜韬殑锛堝彲鑳芥槸涓嶅彲淇＄殑锛夐厤缃浣曘€傜劧鑰岋紝涓嶅彲淇＄殑缁勫彲浠ラ€氳繃鍏跺畠鏂瑰紡鐮村潖浜ゆ崲鈥斺€斾緥濡傚湪涓€涓揣鍑戝惊鐜腑寮曠敤鍏跺尶鍚嶅唴瀛樷€斺€旇€岀鐞嗗憳鍦ㄨ繃搴︽壙璇轰笉鍙俊浣滀笟鏃讹紝涓嶈兘鍋囧畾瀹屽叏鐨勪氦鎹㈠彲琛屾€с€?

鍙︿竴鏂归潰锛屽浜庡彲淇′綔涓氾紝鍚堝苟璁℃暟鍣ㄥ苟涓嶆槸涓€涓洿瑙傜殑鐢ㄦ埛绌洪棿鎺ュ彛锛屽苟涓斿畠杩濊儗浜?cgroup 鎺у埗鍣ㄥ簲褰撹璐﹀苟闄愬埗鐗瑰畾鐗╃悊璧勬簮鐨勭悊蹇点€備氦鎹㈢┖闂翠笌绯荤粺涓殑鍏朵粬璧勬簮涓€鏍凤紝姝ｅ洜濡傛锛岀粺涓€灞傜骇鍏佽鍗曠嫭鍒嗛厤瀹冦€?
