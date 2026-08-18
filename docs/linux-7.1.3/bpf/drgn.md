## BPF drgn 宸ュ叿


drgn 鑴氭湰鏄竴绉嶆柟渚挎槗鐢ㄧ殑鏈哄埗锛岀敤浜庢绱换鎰忓唴鏍告暟鎹粨鏋勩€俤rgn 骞朵笉渚濊禆鍐呮牳 UAPI 鏉ヨ鍙栨暟鎹€?鐩稿弽锛屽畠鐩存帴浠?`/proc/kcore` 鎴?vmcore 璇诲彇鏁版嵁锛屽苟鍩轰簬 vmlinux 涓殑 DWARF 璋冭瘯淇℃伅婕備寒鍦版墦鍗版暟鎹€?
鏈枃妗ｆ弿杩颁笌 BPF 鐩稿叧鐨?drgn 宸ュ叿銆?
鏈夊叧褰撳墠鎵€鏈夊彲鐢ㄥ伐鍏凤紝鍙傝 `drgn/tools`_锛涙湁鍏?drgn 鏈韩鐨勬洿澶氱粏鑺傦紝鍙傝 `drgn/doc`_銆?
### bpf_inspect.py


## 鎻忚堪


`bpf_inspect.py`_ 鏄竴涓敤浜庢鏌?BPF 绋嬪簭涓庢槧灏勶紙map锛夌殑宸ュ叿銆傚畠鍙互閬嶅巻绯荤粺涓墍鏈夌殑绋嬪簭鍜屾槧灏勶紝
骞舵墦鍗拌繖浜涘璞＄殑鍩烘湰淇℃伅锛屽寘鎷?id銆乼ype 鍜?name銆?
`bpf_inspect.py`_ 瑕嗙洊鐨勪富瑕佺敤渚嬫槸鏄剧ず绫诲瀷涓?`BPF_PROG_TYPE_EXT` 涓?`BPF_PROG_TYPE_TRACING`銆侀€氳繃 `freplace`/`fentry`/`fexit`/`fsession`
鏈哄埗闄勫姞鍒板叾浠?BPF 绋嬪簭涓婄殑 BPF 绋嬪簭锛屽洜涓虹洰鍓嶆病鏈夌敤鎴风┖闂?API 鑳借幏鍙栨淇℃伅銆?
## 鍏ラ棬


```
    % sudo bpf_inspect.py prog
        27: BPF_PROG_TYPE_TRACEPOINT         tracepoint__tcp__tcp_send_reset
      4632: BPF_PROG_TYPE_CGROUP_SOCK_ADDR   tw_ipt_bind
     49464: BPF_PROG_TYPE_RAW_TRACEPOINT     raw_tracepoint__sched_process_exit
```
```
      % sudo bpf_inspect.py map
        2577: BPF_MAP_TYPE_HASH                tw_ipt_vips
        4050: BPF_MAP_TYPE_STACK_TRACE         stack_traces
        4069: BPF_MAP_TYPE_PERCPU_ARRAY        ned_dctcp_cntr
```
```
      % sudo bpf_inspect.py p | grep test_pkt_access
         650: BPF_PROG_TYPE_SCHED_CLS          test_pkt_access
         654: BPF_PROG_TYPE_TRACING            test_main                        linked:[650->25: BPF_TRAMP_FEXIT test_pkt_access->test_pkt_access()]
         655: BPF_PROG_TYPE_TRACING            test_subprog1                    linked:[650->29: BPF_TRAMP_FEXIT test_pkt_access->test_pkt_access_subprog1()]
         656: BPF_PROG_TYPE_TRACING            test_subprog2                    linked:[650->31: BPF_TRAMP_FEXIT test_pkt_access->test_pkt_access_subprog2()]
         657: BPF_PROG_TYPE_TRACING            test_subprog3                    linked:[650->21: BPF_TRAMP_FEXIT test_pkt_access->test_pkt_access_subprog3()]
         658: BPF_PROG_TYPE_EXT                new_get_skb_len                  linked:[650->16: BPF_TRAMP_REPLACE test_pkt_access->get_skb_len()]
         659: BPF_PROG_TYPE_EXT                new_get_skb_ifindex              linked:[650->23: BPF_TRAMP_REPLACE test_pkt_access->get_skb_ifindex()]
         660: BPF_PROG_TYPE_EXT                new_get_constant                 linked:[650->19: BPF_TRAMP_REPLACE test_pkt_access->get_constant()]
```
鍙互鐪嬪埌锛屽瓨鍦ㄤ竴涓▼搴?`test_pkt_access`锛宨d 涓?650锛屽苟涓旀湁澶氫釜鍏朵粬鐨?tracing 涓?ext 绋嬪簭闄勫姞鍒?`test_pkt_access` 涓殑鍑芥暟涓娿€?
```
         658: BPF_PROG_TYPE_EXT                new_get_skb_len                  linked:[650->16: BPF_TRAMP_REPLACE test_pkt_access->get_skb_len()]
```
琛ㄧず BPF 绋嬪簭 id 涓?658锛岀被鍨嬩负 `BPF_PROG_TYPE_EXT`锛屽悕绉颁负
`new_get_skb_len`锛屾浛鎹紙`BPF_TRAMP_REPLACE`锛変簡鍦?BPF 绋嬪簭 id 650锛堝悕绉?`test_pkt_access`锛変腑銆?BTF id 涓?16 鐨勫嚱鏁?`get_skb_len()`銆?
鑾峰彇甯姪锛?

    % sudo bpf_inspect.py
    usage: bpf_inspect.py [-h] {prog,p,map,m} ...

    drgn 鑴氭湰锛岀敤浜庡垪鍑?BPF 绋嬪簭鎴栨槧灏勫強鍏跺睘鎬?    锛堝唴鏍?API 鏃犳硶鑾峰彇鐨勯偅浜涳級銆?
    See https://github.com/osandov/drgn/ for more details on drgn.

    optional arguments:
      -h, --help      show this help message and exit

    subcommands:
      {prog,p,map,m}
        prog (p)      list BPF programs
        map (m)       list BPF maps

## 鑷畾涔?

璇ヨ剼鏈棬鍦ㄤ緵寮€鍙戣€呰嚜瀹氫箟锛屼互鎵撳嵃鍏充簬 BPF 绋嬪簭銆佹槧灏勫強鍏朵粬瀵硅薄鐨勭浉鍏充俊鎭€?
渚嬪锛岃鎵撳嵃 BPF 绋嬪簭 id 53077 鐨?`struct bpf_prog_aux`锛?

    % git diff
    diff --git a/tools/bpf_inspect.py b/tools/bpf_inspect.py
    index 650e228..aea2357 100755
    --- a/tools/bpf_inspect.py
    +++ b/tools/bpf_inspect.py
    @@ -112,7 +112,9 @@ def list_bpf_progs(args):
             if linked:
                 linked = f" linked:[{linked}]"

    - print(f"{id_:>6}: {type_:32} {name:32} {linked}")
    - if id_ == 53077:
    - print(f"{id_:>6}: {type_:32} {name:32}")
    - print(f"{bpf_prog.aux}")


     def list_bpf_maps(args):

```
    % sudo bpf_inspect.py p
     53077: BPF_PROG_TYPE_XDP                tw_xdp_policer
    *(struct bpf_prog_aux *)0xffff8893fad4b400 = {
            .refcnt = (atomic64_t){
                    .counter = (long)58,
            },
            .used_map_cnt = (u32)1,
            .max_ctx_offset = (u32)8,
            .max_pkt_offset = (u32)15,
            .max_tp_access = (u32)0,
            .stack_depth = (u32)8,
            .id = (u32)53077,
            .func_cnt = (u32)0,
            .func_idx = (u32)0,
            .attach_btf_id = (u32)0,
            .linked_prog = (struct bpf_prog *)0x0,
            .verifier_zext = (bool)0,
            .offload_requested = (bool)0,
            .attach_btf_trace = (bool)0,
            .func_proto_unreliable = (bool)0,
            .trampoline_prog_type = (enum bpf_tramp_prog_type)BPF_TRAMP_FENTRY,
            .trampoline = (struct bpf_trampoline *)0x0,
            .tramp_hlist = (struct hlist_node){
                    .next = (struct hlist_node *)0x0,
                    .pprev = (struct hlist_node **)0x0,
            },
            .attach_func_proto = (const struct btf_type *)0x0,
            .attach_func_name = (const char *)0x0,
            .func = (struct bpf_prog **)0x0,
            .jit_data = (void *)0x0,
            .poke_tab = (struct bpf_jit_poke_descriptor *)0x0,
            .size_poke_tab = (u32)0,
            .ksym_tnode = (struct latch_tree_node){
                    .node = (struct rb_node [2]){
                            {
                                    .__rb_parent_color = (unsigned long)18446612956263126665,
                                    .rb_right = (struct rb_node *)0x0,
                                    .rb_left = (struct rb_node *)0xffff88a0be3d0088,
                            },
                            {
                                    .__rb_parent_color = (unsigned long)18446612956263126689,
                                    .rb_right = (struct rb_node *)0x0,
                                    .rb_left = (struct rb_node *)0xffff88a0be3d00a0,
                            },
                    },
            },
            .ksym_lnode = (struct list_head){
                    .next = (struct list_head *)0xffff88bf481830b8,
                    .prev = (struct list_head *)0xffff888309f536b8,
            },
            .ops = (const struct bpf_prog_ops *)xdp_prog_ops+0x0 = 0xffffffff820fa350,
            .used_maps = (struct bpf_map **)0xffff889ff795de98,
            .prog = (struct bpf_prog *)0xffffc9000cf2d000,
            .user = (struct user_struct *)root_user+0x0 = 0xffffffff82444820,
            .load_time = (u64)2408348759285319,
            .cgroup_storage = (struct bpf_map *[2]){},
            .name = (char [16])"tw_xdp_policer",
            .security = (void *)0xffff889ff795d548,
            .offload = (struct bpf_prog_offload *)0x0,
            .btf = (struct btf *)0xffff8890ce6d0580,
            .func_info = (struct bpf_func_info *)0xffff889ff795d240,
            .func_info_aux = (struct bpf_func_info_aux *)0xffff889ff795de20,
            .linfo = (struct bpf_line_info *)0xffff888a707afc00,
            .jited_linfo = (void **)0xffff8893fad48600,
            .func_info_cnt = (u32)1,
            .nr_linfo = (u32)37,
            .linfo_idx = (u32)0,
            .num_exentries = (u32)0,
            .extable = (struct exception_table_entry *)0xffffffffa032d950,
            .stats = (struct bpf_prog_stats *)0x603fe3a1f6d0,
            .work = (struct work_struct){
                    .data = (atomic_long_t){
                            .counter = (long)0,
                    },
                    .entry = (struct list_head){
                            .next = (struct list_head *)0x0,
                            .prev = (struct list_head *)0x0,
                    },
                    .func = (work_func_t)0x0,
            },
            .rcu = (struct callback_head){
                    .next = (struct callback_head *)0x0,
                    .func = (void (*)(struct callback_head *))0x0,
            },
    }


```
   https://github.com/osandov/drgn/blob/master/tools/bpf_inspect.py
