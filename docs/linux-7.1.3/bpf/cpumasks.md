
## BPF cpumask kfuncs

## 1. 寮曡█

`struct cpumask` 鏄唴鏍镐腑鐨勪竴涓綅鍥炬暟鎹粨鏋勶紝鍏剁储寮曞弽鏄犵郴缁熶笂鐨?CPU銆傞€氬父锛宑pumask 鐢ㄤ簬璺熻釜涓€涓换鍔¤浜插拰鍒板摢浜?CPU锛屼絾瀹冧滑涔熷彲浠ョ敤浜庝緥濡傝窡韪摢浜涙牳蹇冧笌鏌愪釜璋冨害鍩熺浉鍏宠仈銆佹満鍣ㄤ笂鍝簺鏍稿績鏄┖闂茬殑锛岀瓑绛夈€?
BPF 涓虹▼搴忔彁渚涗簡涓€缁?kfuncs锛屽彲鐢ㄤ簬鍒嗛厤銆佸彉鏇淬€佹煡璇㈠拰閲婃斁 cpumask銆?
## 2. BPF cpumask 瀵硅薄

BPF 绋嬪簭鍙互浣跨敤涓ょ涓嶅悓绫诲瀷鐨?cpumask銆?
### 2.1 ``struct bpf_cpumask *``

`struct bpf_cpumask *` 鏄敱 BPF 浠ｈ〃鏌愪釜 BPF 绋嬪簭鍒嗛厤鐨?cpumask锛屽叾鐢熷懡鍛ㄦ湡瀹屽叏鐢?BPF 鎺у埗銆傝繖浜?cpumask 鍙?RCU 淇濇姢锛屽彲浠ヨ鍙樻洿锛屽彲浠ョ敤浣?kptr锛屽苟涓斿彲浠ュ畨鍏ㄥ湴杞崲涓?`struct cpumask *`銆?
### 2.1.1 ``struct bpf_cpumask *`` 鐢熷懡鍛ㄦ湡

`struct bpf_cpumask *` 浣跨敤浠ヤ笅鍑芥暟杩涜鍒嗛厤銆佽幏鍙栧拰閲婃斁锛?
  :identifiers: bpf_cpumask_create

  :identifiers: bpf_cpumask_acquire

  :identifiers: bpf_cpumask_release

渚嬪锛?

        struct cpumask_map_value {
                struct bpf_cpumask __kptr * cpumask;
        };

        struct array_map {
                __uint(type, BPF_MAP_TYPE_ARRAY);
                __type(key, int);
                __type(value, struct cpumask_map_value);
                __uint(max_entries, 65536);
        } cpumask_map SEC(".maps");

        static int cpumask_map_insert(struct bpf_cpumask *mask, u32 pid)
        {
                struct cpumask_map_value local, *v;
                long status;
                struct bpf_cpumask *old;
                u32 key = pid;

                local.cpumask = NULL;
                status = bpf_map_update_elem(&cpumask_map, &key, &local, 0);
                if (status) {
                        bpf_cpumask_release(mask);
                        return status;
                }

                v = bpf_map_lookup_elem(&cpumask_map, &key);
                if (!v) {
                        bpf_cpumask_release(mask);
                        return -ENOENT;
                }

                old = bpf_kptr_xchg(&v->cpumask, mask);
                if (old)
                        bpf_cpumask_release(old);

                return 0;
        }

        /**
         - 涓€涓ず渚?tracepoint锛屽睍绀哄浣曟煡璇换鍔＄殑 cpumask 骞?         - 灏嗗叾璁板綍涓?kptr銆?         */
        SEC("tp_btf/task_newtask")
        int BPF_PROG(record_task_cpumask, struct task_struct *task, u64 clone_flags)
        {
                struct bpf_cpumask *cpumask;
                int ret;

                cpumask = bpf_cpumask_create();
                if (!cpumask)
                        return -ENOMEM;

                if (!bpf_cpumask_full(task->cpus_ptr))
                        bpf_printk("task %s has CPU affinity", task->comm);

                bpf_cpumask_copy(cpumask, task->cpus_ptr);
                return cpumask_map_insert(cpumask, task->pid);
        }

----

### 2.1.1 ``struct bpf_cpumask *`` 浣滀负 kptr

濡備笂鎵€杩板苟涓句緥璇存槑锛岃繖浜?`struct bpf_cpumask *` 瀵硅薄涔熷彲浠ュ瓨鍌ㄥ湪鏄犲皠涓苟鐢ㄤ綔 kptr銆傚鏋滀竴涓?`struct bpf_cpumask *` 鍦ㄦ槧灏勪腑锛岃寮曠敤鍙互浣跨敤 bpf_kptr_xchg() 浠庢槧灏勪腑绉婚櫎锛屾垨鑰呬娇鐢?RCU 鏈轰細鎬у湴鑾峰彇锛?

	/** 鍖呭惈瀛樺偍鍦ㄦ槧灏勪腑鐨?struct bpf_cpumask kptr 鐨勭粨鏋勪綋銆?**/
	struct cpumasks_kfunc_map_value {
		struct bpf_cpumask __kptr * bpf_cpumask;
	};

	/** 鍖呭惈 struct cpumasks_kfunc_map_value 琛ㄩ」鐨勬槧灏勩€?**/
	struct {
		__uint(type, BPF_MAP_TYPE_ARRAY);
		__type(key, int);
		__type(value, struct cpumasks_kfunc_map_value);
		__uint(max_entries, 1);
	} cpumasks_kfunc_map SEC(".maps");

	/** ... **/

	/**
  - 涓€涓畝鍗曠殑绀轰緥 tracepoint 绋嬪簭锛屽睍绀哄瓨鍌ㄥ湪鏄犲皠涓殑
  - struct bpf_cpumask * kptr 濡備綍
  - 鍙互鍦?RCU 淇濇姢涓嬩紶閫掔粰 kfuncs銆?	 */
	SEC("tp_btf/cgroup_mkdir")
	int BPF_PROG(cgrp_ancestor_example, struct cgroup **cgrp, const char **path)
	{
		struct bpf_cpumask *kptr;
		struct cpumasks_kfunc_map_value *v;
		u32 key = 0;

		/** 鍋囪涔嬪墠宸插湪鏄犲皠涓瓨鍌ㄤ簡涓€涓?bpf_cpumask ** kptr銆?*/
		v = bpf_map_lookup_elem(&cpumasks_kfunc_map, &key);
		if (!v)
			return -ENOENT;

		bpf_rcu_read_lock();
		/** 鑾峰彇瀵瑰凡瀛樺偍鍦ㄦ槧灏勪腑鐨?bpf_cpumask ** kptr 鐨勫紩鐢ㄣ€?*/
		kptr = v->cpumask;
		if (!kptr) {
			/* 濡傛灉鏄犲皠涓病鏈?bpf_cpumask锛岄偅鏄洜涓?    - 鎴戜滑涓庡彟涓€涓湪 bpf_map_lookup_elem()
    - 涔嬪悗銆佷互鍙婃垜浠粠鏄犲皠鍔犺浇鎸囬拡涔嬪墠
    - 鐢?bpf_kptr_xchg() 绉婚櫎瀹冪殑 CPU 鍙戠敓浜嗙珵浜夈€?			 */
			bpf_rcu_read_unlock();
			return -EBUSY;
		}

		bpf_cpumask_setall(kptr);
		bpf_rcu_read_unlock();

		return 0;
	}

----

### 2.2 ``struct cpumask``

`struct cpumask` 鏄疄闄呭寘鍚鏌ヨ銆佸彉鏇寸瓑鐨?cpumask 浣嶅浘鐨勫璞°€備竴涓?`struct bpf_cpumask` 鍖呰浜嗕竴涓?``struct cpumask``锛岃繖灏辨槸涓轰粈涔堝皢鍏跺姝よ浆鎹㈡槸瀹夊叏鐨勶紙浣嗚娉ㄦ剰锛屽皢 `struct cpumask **` 杞崲涓?`struct bpf_cpumask **` 鏄?*涓?*瀹夊叏鐨勶紝楠岃瘉鍣ㄤ細鎷掔粷浠讳綍灏濊瘯杩欐牱鍋氱殑绋嬪簭锛夈€?
姝ｅ鎴戜滑灏嗗湪涓嬮潰鐪嬪埌鐨勶紝浠讳綍鍙樻洿鍏?cpumask 鍙傛暟鐨?kfunc 閮戒細灏?`struct bpf_cpumask *` 浣滀负璇ュ弬鏁般€備换浣曞彧鏄煡璇?cpumask 鐨勫弬鏁板垯浼氬彇涓€涓?`struct cpumask *`銆?
## 3. cpumask kfuncs

涓婇潰鎴戜滑鎻忚堪浜嗗彲鐢ㄤ簬鍒嗛厤銆佽幏鍙栥€侀噴鏀剧瓑 `struct bpf_cpumask *` 鐨?kfunc銆傛湰鏂囨。鐨勮繖涓€鑺傚皢鎻忚堪鐢ㄤ簬鍙樻洿鍜屾煡璇?cpumask 鐨?kfunc銆?
### 3.1 鍙樻洿 cpumask

涓€浜?cpumask kfunc 鏄€滃彧璇烩€濈殑锛屽洜涓哄畠浠笉鍙樻洿浠讳綍鍙傛暟锛岃€屽彟涓€浜涘垯鍙樻洿鑷冲皯涓€涓弬鏁帮紙杩欐剰鍛崇潃璇ュ弬鏁板繀椤绘槸 `struct bpf_cpumask *`锛屽涓婃墍杩帮級銆?
鏈妭灏嗘弿杩版墍鏈夊彉鏇磋嚦灏戜竴涓弬鏁扮殑 cpumask kfunc銆備笅闈?cpumasks-querying-label 鎻忚堪鍙 kfunc銆?
### 3.1.1 璁剧疆鍜屾竻闄?CPU

bpf_cpumask_set_cpu() 鍜?bpf_cpumask_clear_cpu() 鍙垎鍒敤浜庡湪 `struct bpf_cpumask` 涓缃拰娓呴櫎涓€涓?CPU锛?
   :identifiers: bpf_cpumask_set_cpu bpf_cpumask_clear_cpu

杩欎簺 kfunc 鐩稿綋鐩存帴锛屼緥濡傚彲浠ユ寜濡備笅鏂瑰紡浣跨敤锛?

        /**
         - 涓€涓ず渚?tracepoint锛屽睍绀哄浣曟煡璇?cpumask銆?         */
        SEC("tp_btf/task_newtask")
        int BPF_PROG(test_set_clear_cpu, struct task_struct *task, u64 clone_flags)
        {
                struct bpf_cpumask *cpumask;

                cpumask = bpf_cpumask_create();
                if (!cpumask)
                        return -ENOMEM;

                bpf_cpumask_set_cpu(0, cpumask);
                if (!bpf_cpumask_test_cpu(0, cast(cpumask)))
                        /** 涓嶅簲鍙戠敓銆?**/
                        goto release_exit;

                bpf_cpumask_clear_cpu(0, cpumask);
                if (bpf_cpumask_test_cpu(0, cast(cpumask)))
                        /** 涓嶅簲鍙戠敓銆?**/
                        goto release_exit;

                /** 鍍?task->cpus_ptr 杩欐牱鐨?struct cpumask ** 鎸囬拡涔熷彲浠ヨ鏌ヨ銆?*/
                if (bpf_cpumask_test_cpu(0, task->cpus_ptr))
                        bpf_printk("task %s can use CPU %d", task->comm, 0);

        release_exit:
                bpf_cpumask_release(cpumask);
                return 0;
        }

----

bpf_cpumask_test_and_set_cpu() 鍜?bpf_cpumask_test_and_clear_cpu() 鏄簰琛ョ殑 kfunc锛屽厑璁歌皟鐢ㄨ€呭師瀛愬湴娴嬭瘯鍜岃缃紙鎴栨竻闄わ級CPU锛?
   :identifiers: bpf_cpumask_test_and_set_cpu bpf_cpumask_test_and_clear_cpu

----

鎴戜滑涔熷彲浠ヤ娇鐢?bpf_cpumask_setall() 鍜?bpf_cpumask_clear() 鍦ㄤ竴娆℃搷浣滀腑璁剧疆鍜屾竻闄ゆ暣涓?`struct bpf_cpumask *` 瀵硅薄锛?
   :identifiers: bpf_cpumask_setall bpf_cpumask_clear

### 3.1.2 cpumask 涔嬮棿鐨勬搷浣?
闄や簡鍦ㄥ崟涓?cpumask 涓缃拰娓呴櫎鍗曚釜 CPU 涔嬪锛岃皟鐢ㄨ€呰繕鍙互浣跨敤 bpf_cpumask_and()銆乥pf_cpumask_or() 鍜?bpf_cpumask_xor() 鍦ㄥ涓?cpumask 涔嬮棿鎵ц鎸変綅鎿嶄綔锛?
   :identifiers: bpf_cpumask_and bpf_cpumask_or bpf_cpumask_xor

浠ヤ笅鏄畠浠浣曚娇鐢ㄧ殑绀轰緥銆傝娉ㄦ剰锛屾绀轰緥涓樉绀虹殑涓€浜?kfunc 灏嗗湪涓嬫枃涓洿璇︾粏鍦颁粙缁嶃€?

        /**
         - 涓€涓ず渚?tracepoint锛屽睍绀哄浣曚娇鐢?           鎸変綅杩愮畻绗﹀彉鏇达紙骞舵煡璇級cpumask銆?         */
        SEC("tp_btf/task_newtask")
        int BPF_PROG(test_and_or_xor, struct task_struct *task, u64 clone_flags)
        {
                struct bpf_cpumask **mask1, **mask2, **dst1, **dst2;

                mask1 = bpf_cpumask_create();
                if (!mask1)
                        return -ENOMEM;

                mask2 = bpf_cpumask_create();
                if (!mask2) {
                        bpf_cpumask_release(mask1);
                        return -ENOMEM;
                }

                // ...瀹夊叏鍦板垱寤哄彟澶栦袱涓?mask... */

                bpf_cpumask_set_cpu(0, mask1);
                bpf_cpumask_set_cpu(1, mask2);
                bpf_cpumask_and(dst1, (const struct cpumask **)mask1, (const struct cpumask **)mask2);
                if (!bpf_cpumask_empty((const struct cpumask *)dst1))
                        /** 涓嶅簲鍙戠敓銆?**/
                        goto release_exit;

                bpf_cpumask_or(dst1, (const struct cpumask **)mask1, (const struct cpumask **)mask2);
                if (!bpf_cpumask_test_cpu(0, (const struct cpumask *)dst1))
                        /** 涓嶅簲鍙戠敓銆?**/
                        goto release_exit;

                if (!bpf_cpumask_test_cpu(1, (const struct cpumask *)dst1))
                        /** 涓嶅簲鍙戠敓銆?**/
                        goto release_exit;

                bpf_cpumask_xor(dst2, (const struct cpumask **)mask1, (const struct cpumask **)mask2);
                if (!bpf_cpumask_equal((const struct cpumask *)dst1,
                                       (const struct cpumask *)dst2))
                        /** 涓嶅簲鍙戠敓銆?**/
                        goto release_exit;

         release_exit:
                bpf_cpumask_release(mask1);
                bpf_cpumask_release(mask2);
                bpf_cpumask_release(dst1);
                bpf_cpumask_release(dst2);
                return 0;
        }

----

鍙互浣跨敤 bpf_cpumask_copy() 灏嗘暣涓?cpumask 鐨勫唴瀹瑰鍒跺埌鍙︿竴涓細

   :identifiers: bpf_cpumask_copy

----


### 3.2 鏌ヨ cpumask

闄や簡涓婅堪 kfunc 涔嬪锛岃繕鏈変竴缁勫彧璇?kfunc 鍙敤浜庢煡璇?cpumask 鐨勫唴瀹广€?
   :identifiers: bpf_cpumask_first bpf_cpumask_first_zero bpf_cpumask_first_and
                 bpf_cpumask_test_cpu bpf_cpumask_weight

   :identifiers: bpf_cpumask_equal bpf_cpumask_intersects bpf_cpumask_subset
                 bpf_cpumask_empty bpf_cpumask_full

   :identifiers: bpf_cpumask_any_distribute bpf_cpumask_any_and_distribute

----

涓婇潰宸茬粡灞曠ず浜嗚繖浜涙煡璇?kfunc 鐨勪竴浜涚ず渚嬬敤娉曘€傛垜浠笉浼氬湪姝ら噸澶嶉偅浜涚ず渚嬨€備絾鏄紝璇锋敞鎰忥紝鎵€鏈変笂杩?kfunc 閮藉湪 `tools/testing/selftests/bpf/progs/cpumask_success.c`_ 涓繘琛屼簡娴嬭瘯锛屾墍浠ュ鏋滀綘鍦ㄥ鎵炬洿澶氬浣曚娇鐢ㄥ畠浠殑绀轰緥锛岃鐪嬬湅閭ｉ噷銆?
   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/tools/testing/selftests/bpf/progs/cpumask_success.c


## 4. 娣诲姞 BPF cpumask kfunc

鍙楁敮鎸佺殑 BPF cpumask kfunc 闆嗗悎锛堢洰鍓嶏級涓?include/linux/cpumask.h 涓殑 cpumask 鎿嶄綔骞朵笉鏄?1 瀵?1 鐨勫尮閰嶃€傞偅浜?cpumask 鎿嶄綔涓殑浠讳綍涓€涓兘鍙互鍦ㄩ渶瑕佹椂杞绘澗鍦板皝瑁呭埌涓€涓柊鐨?kfunc 涓€傚鏋滀綘鎯虫敮鎸佷竴涓柊鐨?cpumask 鎿嶄綔锛岃闅忔椂鎻愪氦琛ヤ竵銆傚鏋滀綘娣诲姞浜嗕竴涓柊鐨?cpumask kfunc锛岃鍦ㄦ澶勮褰曞畠锛屽苟灏嗕换浣曠浉鍏崇殑鑷祴璇曠敤渚嬫坊鍔犲埌 cpumask 鑷祴璇曞浠朵腑銆?