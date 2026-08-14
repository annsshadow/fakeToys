

## BPF 鍐呮牳鍑芥暟锛坘funcs锛?


## 1. 绠€浠?


BPF 鍐呮牳鍑芥暟锛屾洿甯歌鐨勫彨娉曟槸 kfuncs锛屾槸 Linux 鍐呮牳涓毚闇茬粰 BPF 绋嬪簭浣跨敤鐨勫嚱鏁般€備笌鏅€氱殑 BPF 杈呭姪鍑芥暟锛坔elper锛変笉鍚岋紝kfuncs 娌℃湁绋冲畾鐨勬帴鍙ｏ紝鍙兘浠庝竴涓唴鏍哥増鏈彉鍒板彟涓€涓増鏈€傚洜姝わ紝BPF 绋嬪簭闇€瑕侀殢鐫€鍐呮牳鐨勫彉鍖栬€屾洿鏂般€傛洿澶氫俊鎭鍙傞槄 BPF_kfunc_lifecycle_expectations銆?

## 2. 瀹氫箟涓€涓?kfunc


鏈変袱绉嶆柟寮忓皢鍐呮牳鍑芥暟鏆撮湶缁?BPF 绋嬪簭锛氳涔堣鍐呮牳涓凡鏈夌殑鍑芥暟鍙锛岃涔堜负 BPF 鏂板涓€涓寘瑁呭嚱鏁般€傚湪杩欎袱绉嶆儏鍐典笅锛岄兘蹇呴』娉ㄦ剰 BPF 绋嬪簭鍙兘鍦ㄦ湁鏁堢殑涓婁笅鏂囦腑璋冪敤姝ょ被鍑芥暟銆備负浜嗗己鍒舵墽琛屾绾︽潫锛宬func 鐨勫彲瑙佹€у彲浠ユ槸鎸夌▼搴忕被鍨嬪垝鍒嗙殑銆?

濡傛灉浣犱笉鏄负宸叉湁鐨勫唴鏍稿嚱鏁板垱寤?BPF 鍖呰鍑芥暟锛岃璺冲埌 BPF_kfunc_nodef銆?

### 2.1 鍒涘缓涓€涓寘瑁?kfunc


瀹氫箟鍖呰 kfunc 鏃讹紝鍖呰鍑芥暟搴斿綋鍏锋湁 extern 閾炬帴銆傝繖鍙互闃绘缂栬瘧鍣ㄤ紭鍖栨帀姝讳唬鐮侊紝鍥犱负杩欎釜鍖呰 kfunc 鍦ㄥ唴鏍歌嚜韬腑骞朵笉浼氳浠讳綍鍦版柟璋冪敤銆備负鍖呰 kfunc 鍦ㄥご鏂囦欢涓彁渚涘師鍨嬪苟涓嶆槸蹇呴渶鐨勩€?

```

        /* Disables missing prototype warnings */
        __bpf_kfunc_start_defs();

        __bpf_kfunc struct task_struct *bpf_find_get_task_by_vpid(pid_t nr)
        {
                return find_get_task_by_vpid(nr);
        }

        __bpf_kfunc_end_defs();

```
褰撴垜浠渶瑕佸 kfunc 鐨勫弬鏁拌繘琛屾敞瑙ｆ椂锛岄€氬父灏遍渶瑕佷竴涓寘瑁?kfunc銆傚惁鍒欙紝鍙互鐩存帴閫氳繃鍚?BPF 瀛愮郴缁熸敞鍐屾潵璁?kfunc 瀵?BPF 绋嬪簭鍙銆傝鍙傞槄 BPF_kfunc_nodef銆?

### 2.2 kfunc 鍙傛暟


鐜板湪鎵€鏈?kfunc 榛樿閮借姹傚彈淇′换锛坱rusted锛夌殑鍙傛暟銆傝繖鎰忓懗鐫€鎵€鏈夋寚閽堝弬鏁伴兘蹇呴』鏈夋晥锛屽苟涓旀墍鏈夋寚鍚?BTF 瀵硅薄鐨勬寚閽堥兘蹇呴』浠ユ湭淇敼鐨勫舰寮忥紙闆跺亸绉伙紝涓斾笉鏄€氳繃閬嶅巻鍙︿竴涓寚閽堣幏寰楃殑锛屼笅鏂囨弿杩扮殑渚嬪闄ゅ锛変紶鍏ャ€?

鏈変袱绉嶇被鍨嬬殑鎸囧悜鍐呮牳瀵硅薄鐨勬寚閽堣璁や负鏄€滃彈淇′换鐨勨€濓細

1. 浣滀负 tracepoint 鎴?struct_ops 鍥炶皟鍙傛暟浼犲叆鐨勬寚閽堛€?
2. 浠?KF_ACQUIRE kfunc 杩斿洖鐨勬寚閽堛€?

鎸囧悜闈?BTF 瀵硅薄锛堜緥濡傛爣閲忔寚閽堬級鐨勬寚閽堜篃鍙互浼犵粰 kfuncs锛屽苟涓斿彲浠ュ叿鏈夐潪闆跺亸绉汇€?

鈥滄湁鏁堚€濇寚閽堢殑瀹氫箟闅忔椂鍙兘鏀瑰彉锛岀粷瀵规病鏈変换浣?ABI 绋冲畾鎬т繚璇併€?

濡備笂鎵€杩帮紝浠庨亶鍘嗗彈淇′换鎸囬拡寰楀埌鐨勫祵濂楁寚閽堜笉鍐嶅彈淇′换锛屽彧鏈変竴涓緥澶栥€傚鏋滀竴涓粨鏋勪綋绫诲瀷鏈変竴涓瓧娈碉紝鍙鍏剁埗鎸囬拡鏈夋晥锛岃瀛楁灏变繚璇佹湁鏁堬紙鍙椾俊浠绘垨 rcu锛屽涓嬫枃 KF_RCU 鎻忚堪锛夛紝鍙互浣跨敤浠ヤ笅瀹忓悜楠岃瘉鍣紙verifier锛夎〃杈捐繖涓€鐐癸細

- `BTF_TYPE_SAFE_TRUSTED`
- `BTF_TYPE_SAFE_RCU`
- `BTF_TYPE_SAFE_RCU_OR_NULL`

渚嬪锛?


	BTF_TYPE_SAFE_TRUSTED(struct socket) {
		struct sock *sk;
	};

鎴栬€?


	BTF_TYPE_SAFE_RCU(struct task_struct) {
		const cpumask_t *cpus_ptr;
		struct css_set __rcu *cgroups;
		struct task_struct __rcu *real_parent;
		struct task_struct *group_leader;
	};

鎹㈠彞璇濊锛屼綘蹇呴』锛?

1. 灏嗘湁鏁堟寚閽堢被鍨嬪寘瑁瑰湪 `BTF_TYPE_SAFE_*` 瀹忎腑銆?

2. 鎸囧畾鏈夋晥宓屽瀛楁鐨勭被鍨嬪拰鍚嶇О銆傝瀛楁蹇呴』涓庡師濮嬬被鍨嬪畾涔変腑鐨勫瓧娈靛畬鍏ㄤ竴鑷淬€?

鐢?`BTF_TYPE_SAFE_*` 瀹忓０鏄庣殑鏂扮被鍨嬩篃闇€瑕佽鍙戝嚭锛坋mit锛夛紝浠ヤ究瀹冨嚭鐜板湪 BTF 涓€備緥濡傦紝`BTF_TYPE_SAFE_TRUSTED(struct socket)`
鍦?`type_is_trusted()` 鍑芥暟涓寜濡備笅鏂瑰紡鍙戝嚭锛?


	BTF_TYPE_EMIT(BTF_TYPE_SAFE_TRUSTED(struct socket));

### 2.3 娉ㄨВ kfunc 鍙傛暟


涓?BPF 杈呭姪鍑芥暟绫讳技锛屾湁鏃堕獙璇佸櫒闇€瑕侀澶栫殑涓婁笅鏂囨潵浣垮唴鏍稿嚱鏁扮殑浣跨敤鏇村畨鍏ㄣ€佹洿鏈夌敤銆傚洜姝わ紝鎴戜滑鍙互閫氳繃鍦?kfunc 鐨勫弬鏁板悕鍚庡姞涓婁竴涓?__tag 鍚庣紑鏉ユ敞瑙ｈ鍙傛暟锛屽叾涓?tag 鍙互鏄彈鏀寔鐨勬敞瑙ｄ箣涓€銆?

### 2.3.1 __sz 娉ㄨВ


姝ゆ敞瑙ｇ敤浜庢寚绀哄弬鏁板垪琛ㄤ腑鐨勪竴涓€滃唴瀛樹笌澶у皬鈥濆銆?
```

        __bpf_kfunc void bpf_memzero(void *mem, int mem__sz)
        {
        ...
        }

```
杩欓噷锛岄獙璇佸櫒浼氬皢绗竴涓弬鏁拌涓?PTR_TO_MEM锛屽皢绗簩涓弬鏁拌涓哄叾澶у皬銆傞粯璁ゆ儏鍐典笅锛屽湪娌℃湁 __sz 娉ㄨВ鏃讹紝浣跨敤鎸囬拡鎵€鎸囧悜绫诲瀷鐨勫ぇ灏忋€傛病鏈?__sz 娉ㄨВ鏃讹紝kfunc 涓嶈兘鎺ュ彈 void 鎸囬拡銆?

### 2.3.2 __k 娉ㄨВ


姝ゆ敞瑙ｄ粎鐢ㄤ簬鏍囬噺鍙傛暟锛岃〃绀洪獙璇佸櫒蹇呴』妫€鏌ヨ鏍囬噺鍙傛暟鏄竴涓凡鐭ョ殑甯搁噺锛屽畠涓嶈〃绀哄ぇ灏忓弬鏁帮紝骞朵笖璇ュ父閲忕殑鍊间笌绋嬪簭鐨勫畨鍏ㄦ€х浉鍏炽€?

```

        __bpf_kfunc void *bpf_obj_new(u32 local_type_id__k, ...)
        {
        ...
        }

```
杩欓噷锛宐pf_obj_new 浣跨敤 local_type_id 鍙傛暟鏉ユ煡鎵捐绋嬪簭 BTF 涓绫诲瀷 ID 鐨勫ぇ灏忥紝骞惰繑鍥炴寚鍚戝畠鐨勫ぇ灏忓寲鎸囬拡銆傛瘡涓被鍨?ID 閮芥湁涓嶅悓鐨勫ぇ灏忥紝鍥犳鍦ㄩ獙璇佸櫒鐘舵€佽鍓紙pruning锛夋鏌ヤ腑锛屽綋鍊间笉鍖归厤鏃讹紝蹇呴』灏嗘瘡娆¤繖鏍风殑璋冪敤瑙嗕负涓嶅悓鐨勮皟鐢ㄣ€?

鍥犳锛屽彧瑕?kfunc 鎺ュ彈闈炲父閲忔爣閲忓弬鏁颁笖璇ュ弬鏁颁笉鏄ぇ灏忓弬鏁帮紝骞朵笖甯搁噺鐨勫€煎绋嬪簭瀹夊叏鑷冲叧閲嶈锛屽氨搴斿綋浣跨敤 __k 鍚庣紑銆?

### 2.3.3 __uninit 娉ㄨВ


姝ゆ敞瑙ｇ敤浜庢寚绀鸿鍙傛暟灏嗚瑙嗕负鏈垵濮嬪寲銆?

```

        __bpf_kfunc int bpf_dynptr_from_skb(..., struct bpf_dynptr_kern *ptr__uninit)
        {
        ...
        }

```
杩欓噷锛岃 dynptr 灏嗚瑙嗕负涓€涓湭鍒濆鍖栫殑 dynptr銆傛病鏈夋娉ㄨВ鏃讹紝濡傛灉浼犲叆鐨?dynptr 鏈垵濮嬪寲锛岄獙璇佸櫒灏嗘嫆缁濊绋嬪簭銆?

### 2.3.4 __nullable 娉ㄨВ


姝ゆ敞瑙ｇ敤浜庢寚绀鸿鎸囬拡鍙傛暟鍙兘涓?NULL銆傞獙璇佸櫒灏嗗厑璁镐负姝ょ被鍙傛暟浼犲叆 NULL銆?

```

        __bpf_kfunc void bpf_task_release(struct task_struct *task__nullable)
        {
        ...
        }

```
杩欓噷锛岃 task 鎸囬拡鍙兘涓?NULL銆俴func 璐熻矗鍦ㄨВ寮曠敤璇ユ寚閽堜箣鍓嶆鏌ュ畠鏄惁涓?NULL銆?

__nullable 娉ㄨВ鍙互涓庡叾浠栨敞瑙ｇ粍鍚堜娇鐢ㄣ€備緥濡傦紝褰撲笌鐢ㄤ簬鈥滃唴瀛樹笌澶у皬鈥濆鐨?__sz 鎴?__szk 娉ㄨВ涓€璧蜂娇鐢ㄦ椂锛岄獙璇佸櫒浼氬湪浼犲叆 NULL 鎸囬拡鏃惰烦杩囧ぇ灏忔牎楠岋紝浣嗗湪鎻愬彇甯搁噺澶у皬淇℃伅鏃朵粛浼氬鐞嗗ぇ灏忓弬鏁帮紙褰?
```

        __bpf_kfunc void *bpf_dynptr_slice(..., void *buffer__nullable,
                                           u32 buffer__szk)

```
杩欓噷锛岃 buffer 鍙兘涓?NULL銆傚鏋?buffer 涓嶄负 NULL锛屽畠鐨勫ぇ灏忓繀椤昏嚦灏戜负 buffer__szk 瀛楄妭銆俴func 璐熻矗鍦ㄤ娇鐢?buffer 涔嬪墠妫€鏌ュ畠鏄惁涓?NULL銆?

### 2.3.5 __str 娉ㄨВ


姝ゆ敞瑙ｇ敤浜庢寚绀鸿鍙傛暟鏄竴涓父閲忓瓧绗︿覆銆?

```

        __bpf_kfunc bpf_get_file_xattr(..., const char *name__str, ...)
        {
        ...
        }

```
```

        bpf_get_file_xattr(..., "xattr_name", ...);

```
```

        const char name[] = "xattr_name";  /* This need to be global */
        int BPF_PROG(...)
        {
                ...
                bpf_get_file_xattr(..., name, ...);
                ...
        }

```

### 2.4 浣跨敤宸叉湁鐨勫唴鏍稿嚱鏁?


褰撳唴鏍镐腑宸叉湁鐨勫嚱鏁伴€傚悎琚?BPF 绋嬪簭浣跨敤鏃讹紝瀹冨彲浠ョ洿鎺ュ悜 BPF 瀛愮郴缁熸敞鍐屻€備絾鏄紝浠嶇劧蹇呴』娉ㄦ剰瀹℃煡璇ュ嚱鏁板湪琚?BPF 绋嬪簭璋冪敤鏃剁殑涓婁笅鏂囷紝浠ュ強杩欐牱鍋氭槸鍚﹀畨鍏ㄣ€?

### 2.5 娉ㄨВ kfuncs


闄や簡 kfunc 鐨勫弬鏁板锛岄獙璇佸櫒鍙兘杩橀渶瑕佹洿澶氬叧浜庢敞鍐屽埌 BPF 瀛愮郴缁熺殑 kfunc 绫诲瀷鐨勪俊鎭€備负姝わ紝鎴戜滑瀹氫箟
```

        BTF_KFUNCS_START(bpf_task_set)
        BTF_ID_FLAGS(func, bpf_get_task_pid, KF_ACQUIRE | KF_RET_NULL)
        BTF_ID_FLAGS(func, bpf_put_pid, KF_RELEASE)
        BTF_KFUNCS_END(bpf_task_set)

```
杩欎釜闆嗗悎缂栫爜浜嗕笂闈㈠垪鍑虹殑姣忎釜 kfunc 鐨?BTF ID锛屽苟杩炲悓鏍囧織涓€璧风紪鐮併€傚綋鐒讹紝涔熷厑璁告寚瀹氭病鏈夋爣蹇椼€?

kfunc 鐨勫畾涔変篃搴斿綋濮嬬粓鐢?`__bpf_kfunc` 瀹忚繘琛屾敞瑙ｃ€傝繖鍙互闃叉璇稿缂栬瘧鍣ㄥ湪 kfunc 鏄潤鎬佸唴鏍稿嚱鏁版椂灏嗗叾鍐呰仈锛屾垨鑰呭湪 LTO 鏋勫缓涓洜涓哄畠鍦ㄥ唴鏍稿叾浣欓儴鍒嗘湭琚娇鐢ㄨ€屽皢鍏跺墧闄ょ瓑闂銆傚紑鍙戣€呬笉搴旀墜鍔ㄤ负鑷繁鐨?kfunc 娣诲姞娉ㄨВ鏉ラ槻姝㈣繖浜涢棶棰樸€傚鏋滀负浜嗛槻姝㈡绫婚棶棰樿€岄渶瑕佸浣犵殑 kfunc 娣诲姞娉ㄨВ锛岄偅鏄竴涓?bug锛屽簲褰撴坊鍔犲埌璇ュ畯鐨勫畾涔変腑锛屼互渚垮叾浠?kfunc 涔熷緱鍒板悓鏍风殑澶勭悊
```

        __bpf_kfunc struct task_struct *bpf_get_task_pid(s32 pid)
        {
        ...
        }

```
### 2.5.1 KF_ACQUIRE 鏍囧織


KF_ACQUIRE 鏍囧織鐢ㄤ簬鎸囩ず璇?kfunc 杩斿洖涓€涓寚鍚戝甫寮曠敤璁℃暟锛坮efcounted锛夊璞＄殑鎸囬拡銆傞獙璇佸櫒闅忓悗灏嗙‘淇濊瀵硅薄鎸囬拡鏈€缁堣涓€涓?release kfunc 閲婃斁锛屾垨鑰呴€氳繃琚紩鐢ㄧ殑 kptr锛堣皟鐢?bpf_kptr_xchg锛夎浆绉诲埌鏄犲皠涓€傚惁鍒欙紝楠岃瘉鍣ㄤ細鎷掔粷鍔犺浇璇?BPF 绋嬪簭锛岀洿鍒扮▼搴忔墍鏈夊彲鑳界殑鎺㈢储鐘舵€佷腑閮芥病鏈夐仐鐣欑殑寮曠敤銆?

### 2.5.2 KF_RET_NULL 鏍囧織


KF_RET_NULL 鏍囧織鐢ㄤ簬鎸囩ず璇?kfunc 杩斿洖鐨勬寚閽堝彲鑳戒负 NULL銆傚洜姝わ紝瀹冨己鍒剁敤鎴峰湪浣跨敤锛堣В寮曠敤鎴栦紶閫掔粰鍙︿竴涓緟鍔╁嚱鏁帮級璇?kfunc 杩斿洖鐨勬寚閽堜箣鍓嶏紝瀵瑰叾鍋氫竴娆?NULL 妫€鏌ャ€傛鏍囧織甯镐笌 KF_ACQUIRE 鏍囧織閰嶅浣跨敤锛屼絾浜岃€呭郊姝ゆ浜ゃ€?

### 2.5.3 KF_RELEASE 鏍囧織


KF_RELEASE 鏍囧織鐢ㄤ簬鎸囩ず璇?kfunc 閲婃斁浼犲叆瀹冪殑鎸囬拡銆傚彧鑳戒紶鍏ヤ竴涓寮曠敤鐨勬寚閽堛€傝皟鐢ㄥ甫鏈夋鏍囧織鐨?kfunc 浼氬鑷磋閲婃斁鎸囬拡鐨勬墍鏈夊壇鏈兘澶辨晥銆?

### 2.5.4 KF_SLEEPABLE 鏍囧織


KF_SLEEPABLE 鏍囧織鐢ㄤ簬鍙兘浼戠湢锛坰leep锛夌殑 kfuncs銆傛绫?kfunc 鍙兘鐢卞彲浼戠湢鐨?BPF 绋嬪簭锛圔PF_F_SLEEPABLE锛夎皟鐢ㄣ€?

### 2.5.5 KF_DESTRUCTIVE 鏍囧織


KF_DESTRUCTIVE 鏍囧織鐢ㄤ簬鎸囩ず璋冪敤瀹冧細鐮村潖绯荤粺銆備緥濡傦紝杩欐牱鐨勮皟鐢ㄥ彲鑳藉鑷寸郴缁熼噸鍚垨宕╂簝銆傚洜姝わ紝瀵规绫昏皟鐢ㄦ湁棰濆鐨勯檺鍒躲€傜洰鍓嶅畠浠彧闇€瑕?CAP_SYS_BOOT 鑳藉姏锛屼絾浠ュ悗鍙兘浼氬鍔犳洿澶氥€?

### 2.5.6 KF_RCU 鏍囧織


KF_RCU 鏍囧織鍏佽 kfunc 閫夋嫨閫€鍑洪粯璁ゅ彈淇′换鍙傛暟鐨勮姹傦紝骞舵帴鍙楀叿鏈夎緝寮变繚璇佺殑 RCU 鎸囬拡銆傛爣璁颁簡 KF_RCU 鐨?kfunc 鏈熸湜 PTR_TRUSTED 鎴?MEM_RCU 鍙傛暟銆傞獙璇佸櫒淇濊瘉瀵硅薄鏄湁鏁堢殑涓斾笉瀛樺湪 use-after-free銆傛寚閽堜笉涓?NULL锛屼絾瀵硅薄鐨勫紩鐢ㄨ鏁板彲鑳藉凡杈惧埌闆躲€俴func 闇€瑕佽€冭檻鍋?refcnt != 0 鐨勬鏌ワ紝灏ゅ叾鏄湪杩斿洖 KF_ACQUIRE 鎸囬拡鏃躲€傝繕瑕佹敞鎰忕殑鏄紝涓€涓?KF_RCU 鐨?KF_ACQUIRE kfunc 闈炲父鍙兘涔熷簲褰撳悓鏃舵槸 KF_RET_NULL銆?

### 2.5.7 KF_RCU_PROTECTED 鏍囧織


KF_RCU_PROTECTED 鏍囧織鐢ㄤ簬鎸囩ず璇?kfunc 蹇呴』鍦?RCU 涓寸晫鍖轰腑璋冪敤銆傝繖瀵逛笉鍙紤鐪犵殑绋嬪簭榛樿鏄垚绔嬬殑锛屽浜庡彲浼戠湢鐨勭▼搴忥紝鍒欏繀椤婚€氳繃璋冪敤 `bpf_rcu_read_lock` 鏉ユ樉寮忎繚璇併€?

濡傛灉璇?kfunc 杩斿洖涓€涓寚閽堝€硷紝姝ゆ爣蹇楄繕寮哄埗瑕佹眰杩斿洖鐨勬寚閽堝彈鍒?RCU 淇濇姢锛屽苟涓斿彧鑳藉湪 RCU 涓寸晫鍖哄浜庢椿鍔ㄧ姸鎬佹椂浣跨敤銆?

璇ユ爣蹇椾笉鍚屼簬 `KF_RCU` 鏍囧織锛屽悗鑰呭彧淇濊瘉鍏跺弬鏁版槸鑷冲皯鍙?RCU 淇濇姢鐨勬寚閽堛€傝繖鍙兘浼氫紶閫掓€у湴鏆楃ず RCU 淇濇姢寰楀埌淇濊瘉锛屼絾瀵逛簬閭ｄ簺闇€瑕?RCU 淇濇姢浣嗕笉鎺ュ彈鍙?RCU 淇濇姢鍙傛暟鐨?kfunc 鏉ヨ骞朵笉閫傜敤銆?

### 2.5.8 KF_DEPRECATED 鏍囧織


KF_DEPRECATED 鏍囧織鐢ㄤ簬閭ｄ簺璁″垝鍦ㄥ悗缁唴鏍哥増鏈腑琚慨鏀规垨绉婚櫎鐨?kfuncs銆傝鏍囪浜?KF_DEPRECATED 鐨?kfunc 杩樺簲褰撳湪鍏跺唴鏍告枃妗ｄ腑璁板綍浠讳綍鐩稿叧淇℃伅銆傛绫讳俊鎭€氬父鍖呮嫭璇?kfunc 棰勬湡鐨勫墿浣欏鍛姐€佸鍙浛浠ｅ畠鐨勬柊鍔熻兘鐨勫缓璁紙濡傛灉鏈夌殑璇濓級锛屼互鍙婂彲鑳藉叧浜庝负浣曡绉婚櫎瀹冪殑鐞嗙敱銆?

娉ㄦ剰锛屽敖绠″湪鏌愪簺鎯呭喌涓嬶紝涓€涓?KF_DEPRECATED 鐨?kfunc 鍙兘缁х画寰楀埌鏀寔骞剁Щ闄ゅ叾 KF_DEPRECATED 鏍囧織锛屼絾娣诲姞涔嬪悗瑕佺Щ闄?KF_DEPRECATED 鏍囧織锛屽緢鍙兘姣斾竴寮€濮嬪氨闃绘娣诲姞瀹冭鍥伴毦寰楀銆傚 BPF_kfunc_lifecycle_expectations 鎵€杩帮紝渚濊禆鐗瑰畾 kfunc 鐨勭敤鎴疯榧撳姳灏芥棭璁╀粬浜虹煡鏅撲粬浠殑浣跨敤鍦烘櫙锛屽苟鍦ㄦ绫昏璁哄彂鐢?upstream 鏃跺弬涓庡叧浜庢槸鍚︿繚鐣欍€佷慨鏀广€佸純鐢ㄦ垨绉婚櫎杩欎簺 kfunc 鐨勮璁恒€?

### 2.5.9 KF_IMPLICIT_ARGS 鏍囧織


KF_IMPLICIT_ARGS 鏍囧織鐢ㄤ簬鎸囩ず璇?kfunc 鐨?BPF 绛惧悕涓庡叾鍐呮牳绛惧悕涓嶅悓锛岄殣寮忓弬鏁扮殑鍊煎湪鍔犺浇鏃剁敱楠岃瘉鍣ㄦ彁渚涖€?

鍙湁鐗瑰畾绫诲瀷鐨勫弬鏁版槸闅愬紡鐨勩€傜洰鍓嶅彧鏀寔 `struct bpf_prog_aux *` 绫诲瀷銆?

鍥犳甯︽湁 KF_IMPLICIT_ARGS 鏍囧織鐨?kfunc 鍦?BTF 涓湁涓ょ绫诲瀷锛氫竴绉嶅尮閰嶅唴鏍稿０鏄庯紙鎸夋儻渚嬪悕绉板甫鏈?_impl 鍚庣紑锛夛紝鍙︿竴绉嶅尮閰嶉鏈熺殑 BPF API銆?

楠岃瘉鍣ㄥ彧鍏佽璋冪敤 kfunc 鐨勯潪 _impl 鐗堟湰锛屽嵆浣跨敤涓嶅甫闅愬紡鍙傛暟鐨勭鍚嶃€?

绀轰緥澹版槑锛?


	__bpf_kfunc int bpf_task_work_schedule_signal(struct task_struct **task, struct bpf_task_work **tw,
						      void *map__map, bpf_task_work_callback_t callback,
						      struct bpf_prog_aux *aux) { ... }

BPF 绋嬪簭涓殑绀轰緥鐢ㄦ硶锛?


	/** note that the last argument is omitted **/
        bpf_task_work_schedule_signal(task, &work->tw, &arrmap, task_work_callback);

### 2.6 娉ㄥ唽 kfuncs


涓€鏃?kfunc 鍑嗗濂戒娇鐢紝浣垮叾鍙鐨勬渶鍚庝竴姝ュ氨鏄悜 BPF 瀛愮郴缁熸敞鍐屽畠銆傛敞鍐屾槸鎸?BPF 绋嬪簭绫诲瀷杩涜鐨?
```

        BTF_KFUNCS_START(bpf_task_set)
        BTF_ID_FLAGS(func, bpf_get_task_pid, KF_ACQUIRE | KF_RET_NULL)
        BTF_ID_FLAGS(func, bpf_put_pid, KF_RELEASE)
        BTF_KFUNCS_END(bpf_task_set)

        static const struct btf_kfunc_id_set bpf_task_kfunc_set = {
                .owner = THIS_MODULE,
                .set   = &bpf_task_set,
        };

        static int init_subsystem(void)
        {
                return register_btf_kfunc_id_set(BPF_PROG_TYPE_TRACING, &bpf_task_kfunc_set);
        }
        late_initcall(init_subsystem);

```
### 2.7 浣跨敤 ___init 鎸囧畾鏃犵被鍨嬭浆鎹㈢殑鍒悕


楠岃瘉鍣ㄦ€绘槸寮哄埗瑕佹眰 BPF 绋嬪簭浼犵粰 kfunc 鐨勬寚閽堢殑 BTF 绫诲瀷锛屼笌 kfunc 瀹氫箟涓寚瀹氱殑鎸囬拡绫诲瀷鐩稿尮閰嶃€備笉杩囷紝楠岃瘉鍣ㄥ厑璁搁偅浜涙牴鎹?C 鏍囧噯绛変环銆佷絾 BTF_ID 涓嶅悓鐨勭被鍨嬭浼犵粰鍚屼竴涓?kfunc 鍙傛暟銆?

渚嬪锛屽浜庝互涓嬬被鍨嬪畾涔夛細


	struct bpf_cpumask {
		cpumask_t cpumask;
		refcount_t usage;
	};

楠岃瘉鍣ㄤ細鍏佽灏?`struct bpf_cpumask *` 浼犵粰涓€涓帴鍙?`cpumask_t **`锛堝畠鏄?`struct cpumask **` 鐨勪竴涓?typedef锛夌殑 kfunc銆備緥濡傦紝`struct cpumask **` 鍜?`struct bpf_cpmuask **` 閮藉彲浠ヤ紶缁?bpf_cpumask_test_cpu()銆?

鍦ㄦ煇浜涙儏鍐典笅锛岃繖绉嶇被鍨嬪埆鍚嶈涓烘槸涓嶆湡鏈涚殑銆俙`struct
nf_conn___init`` 灏辨槸杩欐牱涓€涓緥瀛愶細


	struct nf_conn___init {
		struct nf_conn ct;
	};

C 鏍囧噯浼氳涓鸿繖浜涚被鍨嬫槸绛変环鐨勶紝浣嗗皢杩欎袱绉嶇被鍨嬩腑鐨勪换浣曚竴绉嶄紶缁欎竴涓彈淇′换鐨?kfunc 骞朵笉鎬绘槸瀹夊叏鐨勩€俙`struct
nf_conn___init` 琛ㄧず涓€涓凡鍒嗛厤浣?*灏氭湭鍒濆鍖?*鐨?`struct nf_conn`` 瀵硅薄锛屽洜姝ゅ皢涓€涓?``struct
nf_conn___init *` 浼犵粰涓€涓湡鏈涘凡瀹屽叏鍒濆鍖栫殑 `struct
nf_conn *` 鐨?kfunc锛堜緥濡?`bpf_ct_change_timeout()``锛夋槸涓嶅畨鍏ㄧ殑銆?

涓轰簡婊¤冻姝ょ被闇€姹傦紝濡傛灉涓や釜绫诲瀷鍏锋湁瀹屽叏鐩稿悓鐨勫悕绉帮紝涓斿叾涓竴涓甫鏈?`___init` 鍚庣紑锛岄獙璇佸櫒灏嗗己鍒惰繘琛屼弗鏍肩殑 PTR_TO_BTF_ID 绫诲瀷鍖归厤銆?


## 3. kfunc 鐢熷懡鍛ㄦ湡棰勬湡


kfuncs 鎻愪緵鐨勬槸鍐呮牳 <-> 鍐呮牳 API锛屽洜姝や笉鍙椾换浣曚笌鍐呮牳 <-> 鐢ㄦ埛 UAPI 鐩稿叧鐨勪弗鏍肩ǔ瀹氭€ч檺鍒剁害鏉熴€傝繖鎰忓懗鐫€瀹冧滑鍙互琚涓虹被浼间簬 EXPORT_SYMBOL_GPL锛屽洜姝ゅ綋瀹冧滑鎵€鍦ㄥ瓙绯荤粺鐨勭淮鎶よ€呰涓烘湁蹇呰鏃讹紝鍙互瀵瑰叾杩涜淇敼鎴栫Щ闄ゃ€?

涓庡唴鏍哥殑浠讳綍鍏朵粬鍙樻洿涓€鏍凤紝缁存姢鑰呬笉浼氬湪娌℃湁鍚堢悊鐞嗙敱鐨勬儏鍐典笅鏇存敼鎴栫Щ闄や竴涓?kfunc銆備粬浠槸鍚︿細閫夋嫨鏇存敼涓€涓?kfunc锛屾渶缁堝彇鍐充簬澶氱鍥犵礌锛屼緥濡傝 kfunc 鐨勪娇鐢ㄥ箍娉涚▼搴︺€佸畠鍦ㄥ唴鏍镐腑瀛樺湪鐨勬椂闂撮暱鐭€佹槸鍚﹀瓨鍦ㄦ浛浠ｇ殑 kfunc銆佺浉鍏冲瓙绯荤粺鍦ㄧǔ瀹氭€ф柟闈㈢殑鎯緥锛屽綋鐒惰繕鏈夌户缁敮鎸佽 kfunc 鐨勬妧鏈唬浠枫€?

杩欐湁鍑犱釜鍚箟锛?

a) 琚箍娉涗娇鐢ㄦ垨鍦ㄥ唴鏍镐腑瀛樺湪宸蹭箙鐨?kfunc锛岀淮鎶よ€呮洿闅捐瘉鏄庡叾鏇存敼鎴栫Щ闄ょ殑鍚堢悊鎬с€傛崲鍙ヨ瘽璇达紝宸茬煡鏈夊ぇ閲忕敤鎴峰苟鎻愪緵鏄捐憲浠峰€肩殑 kfunc锛屽缁存姢鑰呮姇鍏ユ椂闂村拰绮惧姏鍘绘敮鎸佸畠浠彁渚涗簡鏇村己鐨勬縺鍔便€傚洜姝わ紝鍦?BPF 绋嬪簭涓娇鐢?kfuncs 鐨勫紑鍙戣€咃紝鍚戜粬浜烘矡閫氬拰瑙ｉ噴杩欎簺 kfunc 鏄浣曚互鍙婁负浣曡浣跨敤鐨勶紝骞跺湪瀹冧滑浜?upstream 琚璁烘椂鍙備笌鍏朵腑锛屾槸寰堥噸瑕佺殑銆?

b) 涓庣敤 EXPORT_SYMBOL_GPL 鏍囪鐨勬櫘閫氬唴鏍哥鍙蜂笉鍚岋紝璋冪敤 kfuncs 鐨?BPF 绋嬪簭閫氬父涓嶅睘浜庡唴鏍镐唬鐮佹爲銆傝繖鎰忓懗鐫€褰?kfunc 鍙戠敓鍙樺寲鏃讹紝閲嶆瀯閫氬父鏃犳硶灏卞湴淇敼璋冪敤鑰咃紝鑰屽儚涓婃父椹卞姩鍦ㄥ唴鏍哥鍙峰彉鍖栨椂琚氨鍦版洿鏂伴偅鏍风殑鍋氭硶鍒欎笉鍙銆?

   涓庢櫘閫氬唴鏍哥鍙蜂笉鍚岋紝杩欏 BPF 绗﹀彿鏉ヨ鏄鏈熺殑琛屼负锛屼娇鐢?kfuncs 鐨勬爲澶?BPF 绋嬪簭搴斿綋琚涓轰笌淇敼鍜岀Щ闄よ繖浜?kfuncs 鐩稿叧鐨勮璁哄拰鍐崇瓥鐨勭浉鍏虫柟銆侭PF 绀惧尯灏嗗湪蹇呰鏃剁Н鏋佹壆婕斿弬涓?upstream 璁ㄨ鐨勮鑹诧紝浠ョ‘淇濇绫荤敤鎴风殑瑙傜偣琚撼鍏ヨ€冭檻銆?

c) kfunc 姘歌繙涓嶄細鏈変换浣曠‖鎬хǔ瀹氭€т繚璇併€侭PF API 涓嶈兘涔熶笉浼氱函绮瑰嚭浜庣ǔ瀹氭€у師鍥犺€岀‖鎬ч樆姝㈠唴鏍镐腑鐨勫彉鏇淬€傝瘽铏藉姝わ紝kfuncs 鏄敤鏉ヨВ鍐抽棶棰樺苟涓虹敤鎴锋彁渚涗环鍊肩殑鍔熻兘銆傛槸鍚︽洿鏀规垨绉婚櫎涓€涓?kfunc 鏄竴涓鍙橀噺鐨勬妧鏈喅绛栵紝闇€瑙嗗叿浣撴儏鍐佃€屽畾锛屽苟鍙傝€冨涓婃墍杩扮殑鏁版嵁鐐广€傞鏈熶竴涓?kfunc 鍦ㄦ棤璀﹀憡鐨勬儏鍐典笅琚Щ闄ゆ垨鏇存敼涓嶄細鏄父瑙佺幇璞★紝涔熶笉浼氬湪娌℃湁鍏呭垎鐞嗙敱鐨勬儏鍐典笅鍙戠敓锛屼絾浣跨敤 kfuncs 灏卞繀椤绘帴鍙楄繖绉嶅彲鑳芥€с€?

### 3.1 kfunc 寮冪敤


濡備笂鎵€杩帮紝铏界劧鏈夋椂缁存姢鑰呭彲鑳藉彂鐜板繀椤荤珛鍗虫洿鏀规垨绉婚櫎涓€涓?kfunc 浠ラ€傚簲鍏跺瓙绯荤粺涓殑鏌愪簺鍙樻洿锛屼絾閫氬父 kfuncs 鑳藉瀹圭撼鏇撮暱銆佹洿瀹℃厧鐨勫純鐢ㄨ繃绋嬨€備緥濡傦紝濡傛灉鍑虹幇浜嗕竴涓柊鐨勩€佹瘮鐜版湁 kfunc 鍔熻兘鏇翠紭鐨?kfunc锛岀幇鏈夌殑 kfunc 鍙兘浼氳寮冪敤涓€娈垫椂闂达紝浠ュ厑璁哥敤鎴峰皢浠栦滑鐨?BPF 绋嬪簭杩佺Щ鍒版柊鐨?kfunc 涓娿€傛垨鑰咃紝濡傛灉涓€涓?kfunc 娌℃湁宸茬煡鐢ㄦ埛锛屽彲鑳戒細鍐冲畾鍦ㄦ煇涓純鐢ㄦ湡涔嬪悗绉婚櫎璇?kfunc锛堜笉鎻愪緵鏇夸唬 API锛夛紝浠ヤ究涓虹敤鎴锋彁渚涗竴涓獥鍙ｏ紝浠ヤ究鍦ㄥ叾瀹為檯琚娇鐢ㄧ殑鎯呭喌涓嬮€氱煡 kfunc 缁存姢鑰呫€?

棰勬湡甯歌鐨勬儏鍐垫槸 kfuncs 浼氱粡鍘嗕竴涓純鐢ㄦ湡锛岃€屼笉鏄湪鏃犺鍛婄殑鎯呭喌涓嬭鏇存敼鎴栫Щ闄ゃ€傚 KF_deprecated_flag 鎵€杩帮紝kfunc 妗嗘灦鎻愪緵浜?KF_DEPRECATED 鏍囧織锛屼緵 kfunc 寮€鍙戣€呭悜鐢ㄦ埛鍙戝嚭鏌?kfunc 宸茶寮冪敤鐨勪俊鍙枫€備竴鏃︿竴涓?kfunc 琚爣璁颁簡 KF_DEPRECATED锛岀Щ闄ゆ椂閬靛惊浠ヤ笅娴佺▼锛?

1. 浠讳綍涓庡凡寮冪敤 kfunc 鐩稿叧鐨勪俊鎭兘璁板綍鍦ㄨ kfunc 鐨勫唴鏍告枃妗ｄ腑銆傝鏂囨。閫氬父鍖呮嫭璇?kfunc 棰勬湡鐨勫墿浣欏鍛姐€佸鍙浛浠ｅ凡寮冪敤鍑芥暟鐢ㄦ硶鐨勬柊鍔熻兘鐨勫缓璁紙鎴栬В閲婁负浣曚笉瀛樺湪杩欐牱鐨勬浛浠ｅ搧锛夌瓑銆?

2. 宸插純鐢ㄧ殑 kfunc 浼氬湪棣栨琚爣璁颁负寮冪敤鍚庯紝鍦ㄥ唴鏍镐腑淇濈暀涓€娈垫椂闂淬€傝繖娈垫椂闂寸殑闀跨煭灏嗚鍏蜂綋鎯呭喌鑰屽畾锛岄€氬父鍙栧喅浜庤 kfunc 鐨勪娇鐢ㄥ箍娉涚▼搴︺€佸畠鍦ㄥ唴鏍镐腑瀛樺湪鐨勬椂闂撮暱鐭紝浠ュ強杩佺Щ鍒版浛浠ｆ柟妗堢殑闅惧害銆傝繖涓純鐢ㄦ湡鏄€滃敖鍔涜€屼负鈥濈殑锛屽苟涓斿涓婃墍杩?<BPF_kfunc_lifecycle_expectations>锛屾湁鏃舵儏鍐靛彲鑳借姹傚湪瀹屾暣鐨勯鏈熷純鐢ㄦ湡缁撴潫涔嬪墠灏辩Щ闄よ kfunc銆?

3. 寮冪敤鏈熺粨鏉熷悗锛岃 kfunc 灏嗚绉婚櫎銆傛鏃讹紝璋冪敤璇?kfunc 鐨?BPF 绋嬪簭灏嗚楠岃瘉鍣ㄦ嫆缁濄€?

## 4. 鏍稿績 kfuncs


BPF 瀛愮郴缁熸彁渚涗簡涓€鎵光€滄牳蹇冣€漦funcs锛屽畠浠彲鑳介€傜敤浜庡悇绉嶅悇鏍风殑涓嶅悓娼滃湪浣跨敤鍦烘櫙鍜岀▼搴忋€傝繖浜?kfuncs 鍦ㄦ澶勮褰曘€?

### 4.1 struct task_struct * kfuncs


鏈変竴浜?kfuncs 鍏佽灏?`struct task_struct *` 瀵硅薄鐢ㄤ綔 kptr锛?

   :identifiers: bpf_task_acquire bpf_task_release

褰撲綘鎯宠鑾峰彇鎴栭噴鏀惧涓€涓綔涓轰緥濡?tracepoint 鍙傛暟鎴?struct_ops 鍥炶皟鍙傛暟浼犲叆鐨?`struct task_struct *` 鐨勫紩鐢ㄦ椂锛岃繖浜?kfuncs 寰堟湁鐢ㄣ€備緥濡傦細


	/**
  - A trivial example tracepoint program that shows how to
  - acquire and release a struct task_struct * pointer.
	 */
	SEC("tp_btf/task_newtask")
	int BPF_PROG(task_acquire_release_example, struct task_struct *task, u64 clone_flags)
	{
		struct task_struct *acquired;

		acquired = bpf_task_acquire(task);
		if (acquired)
			/*
    - In a typical program you'd do something like store
    - the task in a map, and the map will automatically
    - release it later. Here, we release it manually.
			 */
			bpf_task_release(acquired);
		return 0;
	}


鍦?`struct task_struct *` 瀵硅薄涓婅幏鍙栫殑寮曠敤鏄彈 RCU 淇濇姢鐨勩€傚洜姝わ紝鍦?RCU 璇诲尯鍩熷唴锛屼綘鍙互鑾峰緱鎸囧悜宓屽叆鍦ㄦ槧灏勫€间腑鐨?task 鐨勬寚閽堬紝鑰屾棤闇€鑾峰彇寮曠敤锛?


	#define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
	private(TASK) static struct task_struct *global;

	/**
  - A trivial example showing how to access a task stored
  - in a map using RCU.
	 */
	SEC("tp_btf/task_newtask")
	int BPF_PROG(task_rcu_read_example, struct task_struct *task, u64 clone_flags)
	{
		struct task_struct *local_copy;

		bpf_rcu_read_lock();
		local_copy = global;
		if (local_copy)
			/*
    - We could also pass local_copy to kfuncs or helper functions here,
    - as we're guaranteed that local_copy will be valid until we exit
    - the RCU read region below.
			 */
			bpf_printk("Global task %s is valid", local_copy->comm);
		else
			bpf_printk("No global task found");
		bpf_rcu_read_unlock();

		/** At this point we can no longer reference local_copy. **/

		return 0;
	}

----

涓€涓?BPF 绋嬪簭涔熷彲浠ヤ粠 pid 鏌ユ壘涓€涓?task銆傚鏋滆皟鐢ㄨ€呮病鏈夊彲浠ヨ幏鍙栧紩鐢ㄧ殑銆佹寚鍚?`struct task_struct *` 瀵硅薄鐨勫彈淇′换鎸囬拡锛岃繖浼氬緢鏈夌敤銆?

   :identifiers: bpf_task_from_pid

涓嬮潰鏄竴涓娇鐢ㄥ畠鐨勪緥瀛愶細


	SEC("tp_btf/task_newtask")
	int BPF_PROG(task_get_pid_example, struct task_struct *task, u64 clone_flags)
	{
		struct task_struct *lookup;

		lookup = bpf_task_from_pid(task->pid);
		if (!lookup)
			/** A task should always be found, as %task is a tracepoint arg. **/
			return -ENOENT;

		if (lookup->pid != task->pid) {
			/* bpf_task_from_pid() looks up the task via its
    - globally-unique pid from the init_pid_ns. Thus,
    - the pid of the lookup task should always be the
    - same as the input task.
			 */
			bpf_task_release(lookup);
			return -EINVAL;
		}

		/* bpf_task_from_pid() returns an acquired reference,
   - so it must be dropped before returning from the
   - tracepoint handler.
		 */
		bpf_task_release(lookup);
		return 0;
	}

### 4.2 struct cgroup * kfuncs


`struct cgroup *` 瀵硅薄涔熸湁鑾峰彇鍜岄噴鏀惧嚱鏁帮細

   :identifiers: bpf_cgroup_acquire bpf_cgroup_release

杩欎簺 kfuncs 鐨勪娇鐢ㄦ柟寮忎笌 bpf_task_acquire() 鍜?bpf_task_release() 瀹屽叏鐩稿悓锛屽洜姝ゆ垜浠笉鍐嶄负瀹冧滑鎻愪緵绀轰緥銆?

----

鍏朵粬鍙敤浜庝笌 `struct cgroup *` 瀵硅薄浜や簰鐨?kfuncs 鏈?bpf_cgroup_ancestor() 鍜?bpf_cgroup_from_id()锛屽垎鍒厑璁歌皟鐢ㄨ€呰闂竴涓?cgroup 鐨勭鍏堜互鍙婇€氳繃鍏?ID 鏌ユ壘涓€涓?cgroup銆備簩鑰呴兘杩斿洖涓€涓?cgroup kptr銆?

   :identifiers: bpf_cgroup_ancestor

   :identifiers: bpf_cgroup_from_id

鏈€缁堬紝搴斿綋鏇存柊 BPF锛屼互鍏佽鍦ㄧ▼搴忚嚜韬腑閫氳繃涓€娆℃櫘閫氱殑鍐呭瓨鍔犺浇鏉ュ畬鎴愯繖浠朵簨銆傜洰鍓嶆病鏈夐獙璇佸櫒鏂归潰鏇村鐨勬敮鎸侊紝杩欐槸涓嶅彲鑳界殑銆俠pf_cgroup_ancestor() 鐨勭敤娉曞涓嬶細


	/**
  - Simple tracepoint example that illustrates how a cgroup's
  - ancestor can be accessed using bpf_cgroup_ancestor().
	 */
	SEC("tp_btf/cgroup_mkdir")
	int BPF_PROG(cgrp_ancestor_example, struct cgroup **cgrp, const char **path)
	{
		struct cgroup *parent;

		/** The parent cgroup resides at the level before the current cgroup's level. **/
		parent = bpf_cgroup_ancestor(cgrp, cgrp->level - 1);
		if (!parent)
			return -ENOENT;

		bpf_printk("Parent id is %d", parent->self.id);

		/** Return the parent cgroup that was acquired above. **/
		bpf_cgroup_release(parent);
		return 0;
	}

### 4.3 struct cpumask * kfuncs


BPF 鎻愪緵浜嗕竴缁勫彲鐢ㄤ簬鏌ヨ銆佸垎閰嶃€佸彉鏇村拰閿€姣?struct cpumask * 瀵硅薄鐨?kfuncs銆傛洿澶氱粏鑺傝鍙傞槄 cpumasks-header-label銆?
