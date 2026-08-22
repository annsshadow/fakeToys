
## 重定

# XDP_REDIRECT


### 支持的映

XDP_REDIRECT 适用于以下映射类型：

- `BPF_MAP_TYPE_DEVMAP`
- `BPF_MAP_TYPE_DEVMAP_HASH`
- `BPF_MAP_TYPE_CPUMAP`
- `BPF_MAP_TYPE_XSKMAP`

关于这些映射的更多信息，请参阅对应的映射文档
### 处理过程


   :doc: xdp redirect

    并非所有驱动都支持重定向后发送帧，而对于支持的驱动，也并非全部支持非线性帧。非线xdp buf/frame 是指包含多个片段buf/frame
### 调试丢包


XDP_REDIRECT 的静默丢包可通过以下方式调试
- bpf_trace
- perf_record

##### bpf_trace


以下 bpftrace 命令可用于捕获并统计所XDP 跟踪点：


    sudo bpftrace -e 'tracepoint:xdp:* { @cnt[probe] = count(); }'
    Attaching 12 probes...
    ^C

    @cnt[tracepoint:xdp:mem_connect]: 18
    @cnt[tracepoint:xdp:mem_disconnect]: 18
    @cnt[tracepoint:xdp:xdp_exception]: 19605
    @cnt[tracepoint:xdp:xdp_devmap_xmit]: 1393604
    @cnt[tracepoint:xdp:xdp_redirect]: 22292200

    Various xdp tracepoints can be found in `source/include/trace/events/xdp.h`

以下 bpftrace 命令可用于提取作err 参数一部分返回`ERRNO`

    sudo bpftrace -e \
    'tracepoint:xdp:xdp_redirect*_err {@redir_errno[-args->err] = count();}
    tracepoint:xdp:xdp_devmap_xmit {@devmap_errno[-args->err] = count();}'

##### perf record


perf 工具也支持记录跟踪点

    perf record -a -e xdp:xdp_redirect_err \
        -e xdp:xdp_redirect_map_err \
        -e xdp:xdp_exception \
        -e xdp:xdp_devmap_xmit

## 参

- https://github.com/xdp-project/xdp-tutorial/tree/master/tracing02-xdp-monitor
