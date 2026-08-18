
## 閲嶅畾鍚?

# XDP_REDIRECT


### 鏀寔鐨勬槧灏?

XDP_REDIRECT 閫傜敤浜庝互涓嬫槧灏勭被鍨嬶細

- `BPF_MAP_TYPE_DEVMAP`
- `BPF_MAP_TYPE_DEVMAP_HASH`
- `BPF_MAP_TYPE_CPUMAP`
- `BPF_MAP_TYPE_XSKMAP`

鍏充簬杩欎簺鏄犲皠鐨勬洿澶氫俊鎭紝璇峰弬闃呭搴旂殑鏄犲皠鏂囨。銆?
### 澶勭悊杩囩▼


   :doc: xdp redirect

    骞堕潪鎵€鏈夐┍鍔ㄩ兘鏀寔閲嶅畾鍚戝悗鍙戦€佸抚锛岃€屽浜庢敮鎸佺殑椹卞姩锛屼篃骞堕潪鍏ㄩ儴鏀寔闈炵嚎鎬у抚銆傞潪绾挎€?xdp buf/frame 鏄寚鍖呭惈澶氫釜鐗囨鐨?buf/frame銆?
### 璋冭瘯涓㈠寘


XDP_REDIRECT 鐨勯潤榛樹涪鍖呭彲閫氳繃浠ヤ笅鏂瑰紡璋冭瘯锛?
- bpf_trace
- perf_record

##### bpf_trace


浠ヤ笅 bpftrace 鍛戒护鍙敤浜庢崟鑾峰苟缁熻鎵€鏈?XDP 璺熻釜鐐癸細


    sudo bpftrace -e 'tracepoint:xdp:* { @cnt[probe] = count(); }'
    Attaching 12 probes...
    ^C

    @cnt[tracepoint:xdp:mem_connect]: 18
    @cnt[tracepoint:xdp:mem_disconnect]: 18
    @cnt[tracepoint:xdp:xdp_exception]: 19605
    @cnt[tracepoint:xdp:xdp_devmap_xmit]: 1393604
    @cnt[tracepoint:xdp:xdp_redirect]: 22292200

    Various xdp tracepoints can be found in `source/include/trace/events/xdp.h`

浠ヤ笅 bpftrace 鍛戒护鍙敤浜庢彁鍙栦綔涓?err 鍙傛暟涓€閮ㄥ垎杩斿洖鐨?`ERRNO`锛?

    sudo bpftrace -e \
    'tracepoint:xdp:xdp_redirect*_err {@redir_errno[-args->err] = count();}
    tracepoint:xdp:xdp_devmap_xmit {@devmap_errno[-args->err] = count();}'

##### perf record


perf 宸ュ叿涔熸敮鎸佽褰曡窡韪偣锛?

    perf record -a -e xdp:xdp_redirect_err \
        -e xdp:xdp_redirect_map_err \
        -e xdp:xdp_exception \
        -e xdp:xdp_devmap_xmit

## 鍙傝€?

- https://github.com/xdp-project/xdp-tutorial/tree/master/tracing02-xdp-monitor
