
## padata 骞惰鎵ц鏈哄埗


:Date: May 2020

Padata 鏄竴绉嶆満鍒讹紝鍐呮牳鍙互鍊熷姪瀹冨皢浠诲姟鍒嗘淳鍒板涓?CPU 涓婂苟琛屾墽琛岋紝鍚屾椂锛堝彲閫夊湴锛変繚鎸佸畠浠殑椤哄簭銆?
瀹冩渶鍒濇槸涓?IPsec 寮€鍙戠殑锛孖Psec 闇€瑕佸澶ч噺鏁版嵁鍖呮墽琛屽姞瀵嗗拰瑙ｅ瘑锛岃€屼笉瀵硅繖浜涙暟鎹寘閲嶆柊鎺掑簭銆傜洰鍓嶈繖鏄?padata 搴忓垪鍖栦换鍔℃敮鎸佺殑鍞竴浣跨敤鑰呫€?
Padata 杩樻敮鎸佸绾跨▼浠诲姟锛屽湪璐熻浇鍧囪　鍜屽悇绾跨▼涔嬮棿鍗忚皟鐨勫悓鏃讹紝灏嗕换鍔″潎鍖€鎷嗗垎銆?
## 杩愯搴忓垪鍖栦换鍔?

### 鍒濆鍖?

浣跨敤 padata 杩愯搴忓垪鍖栦换鍔＄殑绗竴姝ユ槸寤虹珛涓€涓?
```

    #include <linux/padata.h>

    struct padata_instance *padata_alloc(const char *name);

```
'name' 鍙槸鐢ㄤ簬鏍囪瘑璇ュ疄渚嬨€?
```

   struct padata_shell *padata_alloc_shell(struct padata_instance *pinst);

```
padata_shell 鐢ㄤ簬鍚?padata 鎻愪氦涓€涓换鍔★紝骞跺厑璁镐竴绯诲垪杩欐牱鐨勪换鍔¤鐙珛鍦板簭鍒楀寲銆備竴涓?padata_instance 鍙互鍏宠仈涓€涓垨澶氫釜 padata_shell锛屾瘡涓兘鍏佽涓€绯诲垪鐙珛鐨勪换鍔°€?
### 淇敼 cpumask


鐢ㄤ簬杩愯浠诲姟鐨?CPU 鍙互閫氳繃涓ょ鏂瑰紡鏇存敼锛屼竴绉嶆槸閫氳繃缂栫▼鏂瑰紡锛?
```

    int padata_set_cpumask(struct padata_instance *pinst, int cpumask_type,
			   cpumask_var_t cpumask);

```
杩欓噷 cpumask_type 鏄?PADATA_CPU_PARALLEL 鎴?PADATA_CPU_SERIAL 涔嬩竴锛屽叾涓?parallel cpumask 鎻忚堪灏嗙敤浜庡苟琛屾墽琛屾彁浜ゅ埌璇ュ疄渚嬬殑浠诲姟鐨勫鐞嗗櫒锛岃€?serial cpumask 瀹氫箟鍏佽鐢ㄤ綔搴忓垪鍖栧洖璋冨鐞嗗櫒鐨勫鐞嗗櫒銆俢pumask 鎸囧畾瑕佷娇鐢ㄧ殑鏂?cpumask銆?
涓€涓疄渚嬬殑 cpumask 鍙兘鏈夊搴旂殑 sysfs 鏂囦欢銆備緥濡傦紝pcrypt 鐨勬枃浠朵綅浜?/sys/kernel/pcrypt/<instance-name>銆傚湪涓€涓疄渚嬬殑鐩綍涓湁涓や釜鏂囦欢锛宲arallel_cpumask 鍜?serial_cpumask锛屼换涓€ cpumask

```

    echo f > /sys/kernel/pcrypt/pencrypt/parallel_cpumask

```
璇诲彇杩欎簺鏂囦欢涔嬩竴浼氭樉绀虹敤鎴锋彁渚涚殑 cpumask锛屽畠鍙兘涓?鍙敤"鐨?cpumask 涓嶅悓銆?
padata 鍦ㄥ唴閮ㄧ淮鎶や袱瀵?cpumask锛屽嵆鐢ㄦ埛鎻愪緵鐨?cpumask 鍜?鍙敤"鐨?cpumask銆傦紙姣忓閮界敱涓€涓?parallel 鍜屼竴涓?serial cpumask 缁勬垚銆傦級鐢ㄦ埛鎻愪緵鐨?cpumask 鍦ㄥ疄渚嬪垎閰嶆椂榛樿涓烘墍鏈夊彲鑳界殑 CPU锛屽苟鍙互濡備笂鎵€杩版洿鏀广€傚彲鐢ㄧ殑 cpumask 濮嬬粓鏄敤鎴锋彁渚涚殑 cpumask 鐨勫瓙闆嗭紝骞朵笖鍙寘鍚敤鎴锋彁渚涚殑鎺╃爜涓湪绾跨殑 CPU锛涜繖浜涙墠鏄?padata 瀹為檯浣跨敤鐨?cpumask銆傚洜姝わ紝鍚?padata 鎻愪緵涓€涓寘鍚绾?CPU 鐨?cpumask 鏄悎娉曠殑銆備竴鏃︾敤鎴锋彁渚涚殑 cpumask 涓殑鏌愪釜绂荤嚎 CPU 涓婄嚎锛宲adata 灏变細浣跨敤瀹冦€?
鏇存敼 CPU 鎺╃爜鏄唬浠烽珮鏄傜殑鎿嶄綔锛屽洜姝や笉搴旇繃浜庨绻佸湴杩涜銆?
### 杩愯涓€涓换鍔?

瀹為檯鍚?padata 瀹炰緥鎻愪氦宸ヤ綔锛岄渶瑕佸垱寤?
```

    struct padata_priv {
        /* Other stuff here... */
	void                    (*parallel)(struct padata_priv *padata);
	void                    (*serial)(struct padata_priv *padata);
    };

```
璇ョ粨鏋勪綋鍑犱箮鑲畾浼氳宓屽叆鍒版煇涓壒瀹氫簬寰呭畬鎴愬伐浣滅殑鏇村ぇ缁撴瀯浣撲腑銆傚畠鐨勫鏁板瓧娈靛 padata 鏄鏈夌殑锛屼絾璇ョ粨鏋勪綋搴斿湪鍒濆鍖栨椂琚竻闆讹紝骞朵笖搴旀彁渚?parallel() 鍜?serial() 鍑芥暟銆傝繖浜涘嚱鏁板皢鍦ㄥ畬鎴愬伐浣滅殑杩囩▼涓璋冪敤锛屾垜浠◢鍚庡氨浼氱湅鍒般€?
```

    int padata_do_parallel(struct padata_shell *ps,
		           struct padata_priv *padata, int *cb_cpu);

```
ps 鍜?padata 缁撴瀯浣撳繀椤绘寜涓婅堪鏂瑰紡璁剧疆锛沜b_cpu 鎸囧悜浠诲姟瀹屾垚鏃剁敤浜庢渶缁堝洖璋冪殑棣栭€?CPU锛涘畠蹇呴』浣嶄簬褰撳墠瀹炰緥鐨?CPU 鎺╃爜涓紙鍚﹀垯 cb_cpu 鎸囬拡浼氳鏇存柊涓烘寚鍚戝疄闄呰閫変腑鐨?CPU锛夈€俻adata_do_parallel() 鐨勮繑鍥炲€间负 0 琛ㄧず鎴愬姛锛岃〃鏄庝换鍔℃鍦ㄨ繘琛屼腑銆?EBUSY 鎰忓懗鐫€鍏朵粬鍦版柟鐨勬煇涓汉姝ｅ湪骞叉壈璇ュ疄渚嬬殑 CPU 鎺╃爜锛岃€?-EINVAL 琛ㄧず鎶辨€?cb_cpu 涓嶅湪 serial cpumask 涓€乸arallel 鎴?serial cpumask 涓病鏈夊湪绾?CPU锛屾垨鑰呭疄渚嬪凡鍋滄銆?
鎻愪氦缁?padata_do_parallel() 鐨勬瘡涓换鍔★紝灏嗕緷娆¤浼犻€掔粰鎭板ソ涓€娆′笂杩?parallel() 鍑芥暟鐨勮皟鐢紝鍦ㄤ竴涓?CPU 涓婏紝鍥犳鐪熸鐨勫苟琛屾€ф槸閫氳繃鎻愪氦澶氫釜浠诲姟鏉ュ疄鐜扮殑銆俻arallel() 鍦ㄨ蒋浠朵腑鏂绂佺敤鐨勬儏鍐典笅杩愯锛屽洜姝や笉鑳界潯鐪犮€俻arallel() 鍑芥暟浠?padata_priv 缁撴瀯浣撴寚閽堜綔涓哄叾鍞竴鍙傛暟锛涘叧浜庡疄闄呰瀹屾垚鐨勫伐浣滅殑淇℃伅锛屽彲鑳芥槸閫氳繃浣跨敤 container_of() 鎵惧埌澶栧眰缁撴瀯浣撴潵鑾峰緱鐨勩€?
娉ㄦ剰 parallel() 娌℃湁杩斿洖鍊硷紱padata 瀛愮郴缁熷亣瀹?parallel() 浠庤繖涓€鐐硅捣灏嗚礋璐ｈ浠诲姟銆傝浠诲姟涓嶅繀鍦ㄨ繖娆¤皟鐢ㄦ湡闂村畬鎴愶紝浣嗗鏋?parallel() 鐣欎笅浜嗘湭瀹屾垚鐨勫伐浣滐紝瀹冨簲璇ュ仛濂藉噯澶囷紝鍦ㄥ墠涓€涓换鍔″畬鎴愪箣鍓嶈鍐嶆璋冪敤浠ュ鐞嗕竴涓柊浠诲姟銆?
### 搴忓垪鍖栦换鍔?

褰撲竴涓换鍔＄‘瀹炲畬鎴愭椂锛宲arallel()锛堟垨浠讳綍瀹為檯瀹屾垚璇ヤ换鍔＄殑鍑芥暟锛?
```

    void padata_do_serial(struct padata_priv *padata);

```
鍦ㄦ湭鏉ョ殑鏌愪釜鏃跺埢锛宲adata_do_serial() 灏嗚Е鍙戝 padata_priv 缁撴瀯浣撲腑 serial() 鍑芥暟鐨勮皟鐢ㄣ€傝璋冪敤灏嗗彂鐢熷湪鏈€鍒濊皟鐢?padata_do_parallel() 鏃舵墍璇锋眰鐨?CPU 涓婏紱瀹冨悓鏍峰湪鏈湴杞欢涓柇琚鐢ㄧ殑鎯呭喌涓嬭繍琛屻€傛敞鎰忚繖涓皟鐢ㄥ彲鑳戒細琚帹杩熶竴娈垫椂闂达紝鍥犱负 padata 浠ｇ爜浼氫笉閬椾綑鍔涘湴纭繚浠诲姟鎸夋彁浜ょ殑椤哄簭瀹屾垚銆?
### 閿€姣?

娓呯悊涓€涓?padata 瀹炰緥锛岄『鐞嗘垚绔犲湴娑夊強璋冪敤涓や釜 free

```

    void padata_free_shell(struct padata_shell *ps);
    void padata_free(struct padata_instance *pinst);

```
鐢ㄦ埛鏈夎矗浠荤‘淇濆湪璋冪敤涓婅堪浠讳綍鍑芥暟涔嬪墠锛屾墍鏈夋湭瀹屾垚鐨勪换鍔￠兘宸茬粨鏉熴€?
## 杩愯澶氱嚎绋嬩换鍔?

涓€涓绾跨▼浠诲姟鏈変竴涓富绾跨▼鍜岄浂涓垨澶氫釜杈呭姪绾跨▼锛屼富绾跨▼鍙備笌璇ヤ换鍔★紝鐒跺悗绛夊緟鎵€鏈夎緟鍔╃嚎绋嬪畬鎴愩€俻adata 灏嗕换鍔℃媶鍒嗕负绉颁负 chunk 鐨勫崟鍏冿紝鍏朵腑 chunk 鏄竴涓嚎绋嬪湪涓€娆″绾跨▼鍑芥暟鐨勮皟鐢ㄤ腑鎵€瀹屾垚鐨勪竴閮ㄥ垎浠诲姟銆?
鐢ㄦ埛瑕佽繍琛屼竴涓绾跨▼浠诲姟闇€瑕佸仛鍒颁笁浠朵簨銆傞鍏堬紝閫氳繃瀹氫箟涓€涓?padata_mt_job 缁撴瀯浣撴潵鎻忚堪璇ヤ换鍔★紝杩欏皢鍦ㄦ帴鍙ｄ竴鑺備腑瑙ｉ噴銆傝繖鍖呮嫭涓€涓寚鍚戠嚎绋嬪嚱鏁扮殑鎸囬拡锛宲adata 姣忔灏嗕竴涓换鍔?chunk 鍒嗛厤缁欎竴涓嚎绋嬫椂閮戒細璋冪敤璇ュ嚱鏁般€傜劧鍚庯紝瀹氫箟绾跨▼鍑芥暟锛屽畠鎺ュ彈涓変釜鍙傛暟锛宍start`銆乣end` 鍜?`arg`锛屽叾涓墠涓や釜闄愬畾绾跨▼鎵€鎿嶄綔鐨勮寖鍥达紝鏈€鍚庝竴涓槸锛堝鏋滄湁鐨勮瘽锛夋寚鍚戜换鍔″叡浜姸鎬佺殑鎸囬拡銆傚噯澶囧叡浜姸鎬侊紝瀹冮€氬父鍦ㄤ富绾跨▼鐨勬爤涓婂垎閰嶃€傛渶鍚庯紝璋冪敤 padata_do_multithreaded()锛屽畠浼氬湪浠诲姟瀹屾垚鏃惰繑鍥炪€?
## 鎺ュ彛
