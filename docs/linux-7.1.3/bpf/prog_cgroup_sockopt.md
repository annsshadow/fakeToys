
## BPF_PROG_TYPE_CGROUP_SOCKOPT


`BPF_PROG_TYPE_CGROUP_SOCKOPT` 绋嬪簭绫诲瀷鍙互闄勫姞鍒颁袱涓?cgroup 閽╁瓙锛坔ook锛変笂锛?
- `BPF_CGROUP_GETSOCKOPT` - 鍦ㄨ繘绋嬫瘡娆℃墽琛?`getsockopt`
  绯荤粺璋冪敤鏃惰皟鐢ㄣ€?- `BPF_CGROUP_SETSOCKOPT` - 鍦ㄨ繘绋嬫瘡娆℃墽琛?`setsockopt`
  绯荤粺璋冪敤鏃惰皟鐢ㄣ€?
涓婁笅鏂囷紙`struct bpf_sockopt`锛夊叧鑱斾簡濂楁帴瀛楋紙`sk`锛変互鍙?鎵€鏈夎緭鍏ュ弬鏁帮細`level`銆乣optname`銆乣optval` 鍜?`optlen`銆?
## BPF_CGROUP_SETSOCKOPT


`BPF_CGROUP_SETSOCKOPT` 鍦ㄥ唴鏍稿鐞?sockopt **涔嬪墠**琚Е鍙戯紝
骞朵笖鍏朵笂涓嬫枃鏄彲鍐欑殑锛氬畠鍙互鍦ㄥ皢鎻愪緵鐨勫弬鏁板悜涓嬩紶閫掔粰鍐呮牳涔嬪墠
淇敼杩欎簺鍙傛暟銆傝閽╁瓙鍙互璁块棶 cgroup
涓庡鎺ュ瓧鏈湴瀛樺偍锛坰ocket local storage锛夈€?
濡傛灉 BPF 绋嬪簭灏?`optlen` 璁剧疆涓?-1锛屽垯鍦?cgroup 閾句腑
鎵€鏈夊叾浠?BPF 绋嬪簭鎵ц瀹屾瘯鍚庯紝鎺у埗灏嗚繑鍥炵敤鎴风┖闂?锛堝嵆鍐呮牳鐨?`setsockopt` 澶勭悊灏?*涓嶄細**琚墽琛岋級銆?
娉ㄦ剰锛宍optlen` 涓嶈兘澧炲姞鍒拌秴杩囩敤鎴锋彁渚涚殑
鍊笺€傚畠鍙兘琚噺灏忔垨璁剧疆涓?-1銆備换浣曞叾浠栧€奸兘浼?瑙﹀彂 `EFAULT`銆?
### 杩斿洖绫诲瀷


- `0` - 鎷掔粷璇?syscall锛屽皢鍚戠敤鎴风┖闂磋繑鍥?`EPERM`銆?- `1` - 鎴愬姛锛岀户缁墽琛?cgroup 閾句腑鐨勪笅涓€涓?BPF 绋嬪簭銆?
## BPF_CGROUP_GETSOCKOPT


`BPF_CGROUP_GETSOCKOPT` 鍦ㄥ唴鏍稿鐞?sockopt **涔嬪悗**琚Е鍙戙€?濡傛灉 BPF 閽╁瓙瀵瑰唴鏍歌繑鍥炵殑浠讳綍鍐呭鎰熷叴瓒ｏ紝瀹冨彲浠ヨ瀵?`optval`銆乣optlen` 鍜?`retval`銆侭PF 閽╁瓙鍙互瑕嗙洊
涓婅堪鍊硷紝璋冩暣 `optlen` 骞跺皢 `retval` 閲嶇疆涓?0銆傚鏋?`optlen`
琚鍔犲埌瓒呰繃鍒濆鐨?`getsockopt` 鍊硷紙鍗崇敤鎴风┖闂寸紦鍐插尯澶皬锛夛紝
鍒欎細杩斿洖 `EFAULT`銆?
璇ラ挬瀛愬彲浠ヨ闂?cgroup 涓庡鎺ュ瓧鏈湴瀛樺偍銆?
娉ㄦ剰锛屽彲浠ヨ缃粰 `retval` 鐨勫敮涓€鍙帴鍙楀€兼槸 0 浠ュ強
鍐呮牳杩斿洖鐨勫師濮嬪€笺€備换浣曞叾浠栧€奸兘浼氳Е鍙?`EFAULT`銆?
### 杩斿洖绫诲瀷


- `0` - 鎷掔粷璇?syscall锛屽皢鍚戠敤鎴风┖闂磋繑鍥?`EPERM`銆?- `1` - 鎴愬姛锛氬皢 `optval` 鍜?`optlen` 澶嶅埗鍒扮敤鎴风┖闂达紝骞朵粠
  syscall 杩斿洖 `retval`锛堟敞鎰忚繖鍙兘浼氳鐖?cgroup 鐨?  BPF 绋嬪簭瑕嗙洊锛夈€?
## Cgroup 缁ф壙


鍋囪瀛樺湪濡備笅 cgroup 灞傜骇锛屽叾涓瘡涓?cgroup 鍦ㄦ瘡涓眰绾ч兘闄勫姞浜?`BPF_CGROUP_GETSOCKOPT`锛屽叾涓?```

  A (root, parent)
   \
    B (child)

```

褰撳簲鐢ㄧ▼搴忎粠 cgroup B 璋冪敤 `getsockopt` syscall 鏃讹紝
绋嬪簭鑷簳鍚戜笂鎵ц锛欱銆丄銆傜涓€涓▼搴?锛圔锛夌湅鍒板唴鏍?`getsockopt` 鐨勭粨鏋溿€傚畠鍙互閫夋嫨鎬у湴
璋冩暣 `optval`銆乣optlen` 骞跺皢 `retval` 閲嶇疆涓?0銆備箣鍚?鎺у埗灏嗕紶閫掔粰绗簩涓紙A锛夌▼搴忥紝璇ョ▼搴忓皢鐪嬪埌
涓?B 鐩稿悓鐨勪笂涓嬫枃锛屽寘鎷换浣曟綔鍦ㄧ殑淇敼銆?
`BPF_CGROUP_SETSOCKOPT` 鍚岀悊锛氬鏋滅▼搴忚闄勫姞鍒?A 鍜?B锛岃Е鍙戦『搴忔槸 B锛岀劧鍚?A銆傚鏋?B 瀵硅緭鍏ュ弬鏁?锛坄level`銆乣optname`銆乣optval`銆乣optlen`锛夊仛浜嗕换浣曚慨鏀癸紝
閭ｄ箞閾句腑鐨勪笅涓€涓▼搴忥紙A锛夊皢鐪嬪埌閭ｄ簺淇敼锛?**鑰岄潪**鍘熷鐨勮緭鍏?`setsockopt` 鍙傛暟銆傝繖浜涘彲鑳借
淇敼鐨勫€奸殢鍚庝細琚悜涓嬩紶閫掔粰鍐呮牳銆?
## 杈冨ぇ鐨?optval


褰?`optval` 澶т簬 `PAGE_SIZE` 鏃讹紝BPF 绋嬪簭
鍙兘璁块棶璇ユ暟鎹殑绗竴涓?`PAGE_SIZE`銆傚洜姝ゅ畠鏈変袱涓€夋嫨锛?
- 灏?`optlen` 璁剧疆涓洪浂锛岃繖琛ㄧず鍐呮牳搴斾娇鐢?  鏉ヨ嚜鐢ㄦ埛绌洪棿鐨勫師濮嬬紦鍐插尯銆侭PF 绋嬪簭瀵?`optval` 鎵€鍋氱殑浠讳綍淇敼
  閮藉皢琚拷鐣ャ€?- 灏?`optlen` 璁剧疆涓哄皬浜?`PAGE_SIZE` 鐨勫€硷紝杩欒〃绀?  鍐呮牳搴斾娇鐢?BPF 瑁佸壀鍚庣殑 `optval`銆?
褰?BPF 绋嬪簭浠ュぇ浜?`PAGE_SIZE` 鐨?`optlen` 杩斿洖鏃讹紝
鐢ㄦ埛绌洪棿灏嗘敹鍒板師濮嬬殑鍐呮牳缂撳啿鍖猴紝鑰?BPF 绋嬪簭鍙兘鏂藉姞鐨?浠讳綍淇敼閮戒笉浼氳搴旂敤銆?
## 绀轰緥


澶勭悊 BPF 绋嬪簭鐨勬帹鑽愭柟寮忓涓嬶細


	SEC("cgroup/getsockopt")
	int getsockopt(struct bpf_sockopt *ctx)
	{
		/** 鑷畾涔夊鎺ュ瓧閫夐」銆?**/
		if (ctx->level == MY_SOL && ctx->optname == MY_OPTNAME) {
			ctx->retval = 0;
			optval[^0^] = ...;
			ctx->optlen = 1;
			return 1;
		}

		/** 淇敼鍐呮牳鐨勫鎺ュ瓧閫夐」銆?**/
		if (ctx->level == SOL_IP && ctx->optname == IP_FREEBIND) {
			ctx->retval = 0;
			optval[^0^] = ...;
			ctx->optlen = 1;
			return 1;
		}

		/** optval 澶т簬 PAGE_SIZE 鏃朵娇鐢ㄥ唴鏍哥紦鍐插尯銆?**/
		if (ctx->optlen > PAGE_SIZE)
			ctx->optlen = 0;

		return 1;
	}

	SEC("cgroup/setsockopt")
	int setsockopt(struct bpf_sockopt *ctx)
	{
		/** 鑷畾涔夊鎺ュ瓧閫夐」銆?**/
		if (ctx->level == MY_SOL && ctx->optname == MY_OPTNAME) {
			/** 鎵ц鏌愪簺鎿嶄綔 **/
			ctx->optlen = -1;
			return 1;
		}

		/** 淇敼鍐呮牳鐨勫鎺ュ瓧閫夐」銆?**/
		if (ctx->level == SOL_IP && ctx->optname == IP_FREEBIND) {
			optval[^0^] = ...;
			return 1;
		}

		/** optval 澶т簬 PAGE_SIZE 鏃朵娇鐢ㄥ唴鏍哥紦鍐插尯銆?**/
		if (ctx->optlen > PAGE_SIZE)
			ctx->optlen = 0;

		return 1;
	}

鏈夊叧澶勭悊濂楁帴瀛楅€夐」鐨?BPF 绋嬪簭绀轰緥锛岃鍙傝
`tools/testing/selftests/bpf/progs/sockopt_sk.c`銆?