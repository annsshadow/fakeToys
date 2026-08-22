
## 海PCIe 调优与追踪设备（PTT

## 简

海PCIe 调优与追踪设备（PTT，PCIe Tune and Trace device）是一个集成在
PCIe 根复合体中的端点设备（RCiEP，Root Complex integrated Endpoint），能够
动态地监控并调PCIe 链路上的事件（调tune），以及追踪 TLP 头（追踪/trace）这两个功能相互独立，但建议配合使用，以分析和提升 PCIe 链路的性能
在鲲930 SoC 上，PCIe 根复合体由多PCIe 核（core）组成。每PCIe 核包若干个根端口（Root Port）以及一PTT RCiEP，如下图所示。PTT 设备能够对其所PCIe 核的链路进行调优和追踪```

          +--------------Core 0-------+
          |       |       [   PTT   ] |
          |       |       [Root Port]---[Endpoint]
          |       |       [Root Port]---[Endpoint]
          |       |       [Root Port]---[Endpoint]
    Root Complex  |------Core 1-------+
          |       |       [   PTT   ] |
          |       |       [Root Port]---[ Switch ]---[Endpoint]
          |       |       [Root Port]---[Endpoint] `-[Endpoint]
          |       |       [Root Port]---[Endpoint]
          +---------------------------+

```
每个 PTT 设备驱动会为对应PTT 设备注册一PMU 设备。每PTT 设备的名称由
'hisi_ptt' 前缀加上其所在的 SICL Core id 组成。鲲930 SoC 封装了多CPU 晶片（SCCL，Super CPU Cluster，超CPU 簇）IO 晶片（SICL，Super I/O
Cluster，超IO 簇），每SICL 对应一PCIe 根复合体```

    /sys/bus/event_source/devices/hisi_ptt<sicl_id>_<core_id>

```
## 调优（Tune

PTT 调优（tune）用于监控并调整 PCIe 链路参数（事件）。目前我们支2 类事件事件的作用范围覆PTT 设备所属的 PCIe 核
每个事件$(PTT PMU dir)/tune 目录下的一个文件形式呈现，通过一个简单的
open/read/write/close 流程即可对该事件进行调优```

    $ cd /sys/bus/event_source/devices/hisi_ptt<sicl_id>_<core_id>/tune
    $ ls
    qos_tx_cpl    qos_tx_np    qos_tx_p
    tx_path_rx_req_alloc_buf_level
    tx_path_tx_req_alloc_buf_level
    $ cat qos_tx_dp
    1
    $ echo 2 > qos_tx_dp
    $ cat qos_tx_dp
    2

```
事件的当前值（数值）可以直接从文件中读取，而期望的值写入文件即可完成调优
### 1. 发送路QoS 控制

以下文件用于调优 PCIe 核发送（tx）路径的 QoS
- qos_tx_cpl：发送完成（Tx completion）TLP 的权- qos_tx_np：发送非发布（Tx non-posted）TLP 的权- qos_tx_p：发送发布（Tx posted）TLP 的权
该权重影响特定类型报文在 PCIe 链路上的占比。例如，在存储场景下，增大链路上
完成报文的占比可以提升性能，因为会消耗更多的完成报文
这些事件可调的数据为 [0, 1, 2]。写入负值会返回错误，超出范围的值会被转换为 2注意，事件值仅表示一个大致的级别，并非精确值
### 2. 发送路径缓冲控
以下文件用于调优 PCIe 核发送（tx）路径的缓冲
- rx_alloc_buf_level：接收请求（Rx requested）的水位- tx_alloc_buf_level：发送请求（Tx requested）的水位
这些事件影响为每种类型分配的缓冲水位线。Rx 表示入站（inbound），Tx 表示出站
（outbound）。报文会先存入缓冲，然后在达到水位线或超时时被发送出去。对于繁忙的
方向，应增大对应的缓冲水位线，以避免频繁发送，从而提升性能。大多数情况下保默认值即可
上述事件可调的数据为 [0, 1, 2]。写入负值会返回错误，超出范围的值会被转换为 2注意，事件值仅表示一个大致的级别，并非精确值
## 追踪（Trace

PTT 追踪（trace）用于将 TLP 头转储到内存中，可用于分PCIe 链路上的事务使用情况。你可以选择按请求ID（Requester ID）过滤被追踪的头，也可以选择追踪
位于 PTT 设备同一核上的一组根端口的下游流量。同时也支持追踪特定类型和特定方向的
报文头
你可以使perf 命令 `perf record` 来设置参数、启动追踪并获取数据。同时支持使`perf report` 对追踪数据进行解码。追踪的控制参数以各事件对应的事件编码（event
code）形式输入，稍后会进一步说明。一个使用示例为```

    $ perf record -e hisi_ptt0_2/filter=0x80001,type=1,direction=1,
      format=1/ -- sleep 5

```
这将追踪根端0000:00:10.1 下游TLP 头（事件 'filter' 的事件编码为 0x80001），
类型为发布型 TLP 请求（posted TLP requests），方向为入站（inbound），追踪数据格式
涓?8DW銆。
### 1. 过滤（Filter
要追踪的 TLP 头可按根端口或端点的请求ID 进行过滤，二者均位于 PTT 设备所在的
同一核上。可以通过指定 `filter` 参数来设置过滤器，该参数是启动追踪所必需的。参值为 20 位。bit 19 表示过滤器类型，1 表示根端口过滤器 表示请求者过滤器bit[15:0] 表示过滤值。根端口的过滤值是其核端口 id 的掩码，PCI 插槽 ID 计算
得到slotid & 7) * 2。请求者的过滤值即为请求ID（PCIe 功能的设ID）bit[18:16] 当前保留以备扩展
例如，若期望的过滤器为端点功0000:01:00.1，则过滤值为 0x00101。若期望的过滤器
为根端口 0000:00:10.0，则过滤值计算为 0x80001
驱动还会通过 sysfs 呈现每个受支持的根端口和请求者过滤器。每个过滤器以与其相PCIe 设备名（domain:bus:device.function）同名的独立文件呈现。根端口过滤器的文件
位于 $(PTT PMU dir)/root_port_filters，请求者过滤器的文件位$(PTT PMU dir)/requester_filters
注意，一次可以指定多个根端口，但一次追踪只能指定一个端点功能。不支持同时指定
根端口和功能。驱动维护一个可用过滤器列表，并对非法输入进行检查
可用过滤器会动态更新，这意味着在发生热插拔事件，或手动移除/重新扫描设备时，始终能获取正确的过滤器信息
### 2. 类型（Type
可以通过指定 `type` 参数来追踪特定类型的 TLP 头，该参数是启动追踪所必需的参数值为 8 位。当前支持的类型及对应取值如下：

- 8'b00000001：发布型请求（P，posted- 8'b00000010：非发布型请求（NP，non-posted- 8'b00000100：完成报文（CPL，completions
追踪入站（inbound）TLP 头时可以指定多种类型，但追踪出站（outbound）TLP 头时
只能指定一种类型
### 3. 方向（Direction
可以通过指定 `direction` 参数来追踪特定方向的 TLP 头，方向是相对于根端口或
PCIe 核而言的。该参数可选，默认参数为入站（inbound）。参数值为 4 位。当期望格式4DW 时，支持的方向及对应取值如下：

- 4'b0000：入TLP（P, NP, CPL- 4'b0001：出TLP（P, NP, CPL- 4'b0010：出TLP（P, NP, CPL）及入站 TLP（P, NP, CPL B- 4'b0011：出TLP（P, NP, CPL）及入站 TLP（CPL A
当期望的格式8DW 时，支持的方向及对应取值如下：

- 4'b0000：保- 4'b0001：出TLP（P, NP, CPL- 4'b0010：入TLP（P, NP, CPL B- 4'b0011：入TLP（CPL A
入站完成报文分为两类
- 完成 A（CPL A）：CHI/DMA/原生非发布请求的完成，CPL B 除外
- 完成 B（CPL B）：DMA remote2local P2P 非发布请求的完成

### 4. 格式（Format
可以通过指定 `format` 参数来改变被追踪TLP 头格式。默认格式为 4DW。参数值为
4 位。当前支持的格式及对应取值如下：

- 4'b0000：每TLP 头长度为 4DW
- 4'b0001：每TLP 头长度为 8DW

被追踪的 TLP 头格式与 PCIe 标准不同
使用 8DW 数据格式时，会记录完整的 TLP 头（如下所示的 Header DW0-3）。例如，
64 位地址的内存读请求TLP 头见 PCIe r5.0 2-17；配置请求的 TLP 头见
2.20，等等
此外DW 追踪缓冲条目包含一个时间戳，以及可能包含的 PASID TLP 前缀
（参PCIe r5.0 6-20）。否则该字段全为 0
DW0 bit[31:11] 恒为 0x1fffff，可用于区分数据格式DW 格式如下```

    bits [                 31:11                 ][       10:0       ]
         |---------------------------------------|-------------------|
     DW0 [                0x1fffff               ][ Reserved (0x7ff) ]
     DW1 [                       Prefix                              ]
     DW2 [                     Header DW0                            ]
     DW3 [                     Header DW1                            ]
     DW4 [                     Header DW2                            ]
     DW5 [                     Header DW3                            ]
     DW6 [                   Reserved (0x0)                          ]
     DW7 [                        Time                               ]

```
使用 4DW 数据格式时，追踪缓冲条目DW0 包含 TLP DW0 中选中的字段以及时间戳追踪缓冲条目DW1-DW3 直接包含 TLP 头的 DW1-DW3
4DW 格式如下```

    bits [31:30] [ 29:25 ][24][23][22][21][    20:11   ][    10:0    ]
         |-----|---------|---|---|---|---|-------------|-------------|
     DW0 [ Fmt ][  Type  ][T9][T8][TH][SO][   Length   ][    Time    ]
     DW1 [                     Header DW1                            ]
     DW2 [                     Header DW2                            ]
     DW3 [                     Header DW3                            ]

```
### 5. 内存管理

被追踪的 TLP 头会被写入由驱动分配的内存中。硬件接4 个大小相同的 DMA 地址并按如下顺序依次写入缓冲。如DMA 地址 3 写入完成且追踪仍在进行，则会回到地址 0```

    +->[DMA addr 0]->[DMA addr 1]->[DMA addr 2]->[DMA addr 3]-+
    +---------------------------------------------------------+

```
驱动会为每个 DMA 缓冲分配 4MiB。已写入完成的缓冲会被复制到perf 核心分配perf AUX 缓冲中。一AUX 缓冲在追踪进行期间被填满，驱动会先提交该 AUX 缓冲然后再申请一个同样大小的新缓冲。AUX 缓冲大小默认16MiB。用户可以通过指定
perf 命令`-m` 参数来调整该大小
### 6. 解码（Decoding
你可以使`perf report -D` 命令对追踪数据进行解码（目前仅支持转储原始追踪数据）追踪数据会按照前面描述的格式进行解码（以 8DW 为例）：
```

    [...perf headers and other information]
    . ... HISI PTT data: size 4194304 bytes
    .  00000000: 00 00 00 00                                 Prefix
    .  00000004: 01 00 00 60                                 Header DW0
    .  00000008: 0f 1e 00 01                                 Header DW1
    .  0000000c: 04 00 00 00                                 Header DW2
    .  00000010: 40 00 81 02                                 Header DW3
    .  00000014: 33 c0 04 00                                 Time
    .  00000020: 00 00 00 00                                 Prefix
    .  00000024: 01 00 00 60                                 Header DW0
    .  00000028: 0f 1e 00 01                                 Header DW1
    .  0000002c: 04 00 00 00                                 Header DW2
    .  00000030: 40 00 81 02                                 Header DW3
    .  00000034: 02 00 00 00                                 Time
    .  00000040: 00 00 00 00                                 Prefix
    .  00000044: 01 00 00 60                                 Header DW0
    .  00000048: 0f 1e 00 01                                 Header DW1
    .  0000004c: 04 00 00 00                                 Header DW2
    .  00000050: 40 00 81 02                                 Header DW3
    [...]

```
