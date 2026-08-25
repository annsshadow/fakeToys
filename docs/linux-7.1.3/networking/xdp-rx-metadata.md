
## XDP RX 元数
本文档描述了 XDP（eXpress Data Path，快速数据路径）程序如何使用一组辅助函数来
访问与数据包相关的硬件元数据，以及如何将该元数据传递给其他消费者
## 总体设计

XDP 可以访问一kfunc 来操XDP 帧中的元数据。每个希望暴露额外数据包元数据的
设备驱动都可以实现这kfunc。这kfunc 通过 `XDP_METADATA_KFUNC_xxx` `include/net/xdp.h` 中声明
目前支持以下 kfunc。未来，随着支持更多元数据，这组集合将会扩大
   :identifiers: bpf_xdp_metadata_rx_timestamp

   :identifiers: bpf_xdp_metadata_rx_hash

   :identifiers: bpf_xdp_metadata_rx_vlan_tag

XDP 程序可以使用这些 kfunc 将元数据读入栈上的变量供自身使用。或者，为了将元数据
传递给其他消费者，XDP 程序可以将其存储到数据包前面携带的元数据区域中。并非所数据包都必然会有所请求的元数据可用，在这种情况下驱动会返回 `-ENODATA`
并非所kfunc 都必须由设备驱动实现；当未实现时，将使用返回 `-EOPNOTSUPP` 默认版本，以表明设备驱动尚未实现kfunc
XDP 帧中，元数据布局（通过 `xdp_buff` 访问）为
```

  +----------+-----------------+------+
  | headroom | custom metadata | data |
  +----------+-----------------+------+
             ^                 ^
             |                 |
   xdp_buff->data_meta   xdp_buff->data

```
XDP 程序可以以它选择的任何格式将各个元数据项存储到这`data_meta` 区域中。后续的
元数据消费者必须通过某种带外约定来就格式达成一致（例如对于 AF_XDP 用例，见下文）
## AF_XDP

[af_xdp](af_xdp) 用例意味着，将 XDP 帧重定向`AF_XDP` 套接字（`XSK`）的 BPF
程序与最终消费者之间存在一个约定。因此，BPF 程序通过 `bpf_xdp_adjust_meta` 元数据中手动分配固定数量的字节，并调用部kfunc 来填充它。用户空`XSK` 消费计算 `xsk_umem__get_data() - METADATA_SIZE` 来定位该元数据。注意，`xsk_umem__get_data`
定义`libxdp` 中，`METADATA_SIZE` 是一个应用特定的常量（`AF_XDP` 接收描述并_不_显式携带元数据的大小）```

  +----------+-----------------+------+
  | headroom | custom metadata | data |
  +----------+-----------------+------+
                               ^
                               |
                        rx_desc->address

```
## XDP_PASS

这是XDP 程序处理过的包传入内核的路径。内核根`xdp_buff` 的内容创`skb`目前，每个驱动都有自定义的核内代码，在进`xdp_buff->skb` 转换时解析描述符填充 `skb` 元数据，而在构建 `skbs` 时内核并不会使用 XDP 元数据。不过，TC-BPF
程序可以使用 `data_meta` 指针访问 XDP 元数据区域
未来，我们希望支持这样一种情况：XDP 程序可以覆盖用于构建 `skbs` 的部分元数据
## bpf_redirect_map

`bpf_redirect_map` 可以将帧重定向到另一个设备。某些设备（如虚拟以太网链路）支在重定向后运行第二个 XDP 程序。但是，最终的消费者无法访问原始的硬件描述符，无法访问任何原始元数据。这同样适用于安装到 devmap cpumap 中的 XDP 程序
这意味着对于重定向后的数据包，目前只支持自定义元数据，且必须由初始的 XDP 程序
在重定向之前准备好。如果该帧最终被传入内核，那么由该帧创建`skb` 中将不会
填充任何硬件元数据。如果这样的数据包后来被重定向到 `XSK`，它也只会访问到自定元数据
## bpf_tail_call

目前不支持将访问元数kfunc 的程序添加到 `BPF_MAP_TYPE_PROG_ARRAY` 中
## 支持的设
可以通过 netlink 查询特定 netdev 实现了哪kfunc。参`Documentation/netlink/specs/netdev.yaml` 中的 `xdp-rx-metadata-features` 属性集
## 驱动实现

某些设备可能会在被接收的数据包前面添加元数据。但是截至目前，`AF_XDP` 缺乏`data_meta` 区域的大小传递给消费者的能力。因此，驱动有责任将设备保留的任何元数据
从元数据区域中复制出来，并确保在将帧呈现XDP 程序之前，`xdp_buff->data_meta`
指向 `xdp_buff->data`。这是必要的，这样在 XDP 程序调整元数据区域之后，消费者才可靠地使`METADATA_SIZE` 偏移量检索到元数据地址
下面的示意图展示了自定义元数据相对于
```

              |<-- bpf_xdp_adjust_meta(xdp_buff, -METADATA_SIZE) --|
  new xdp_buff->data_meta                              old xdp_buff->data_meta
              |                                                    |
              |                                            xdp_buff->data
              |                                                    |
   +----------+----------------------------------------------------+------+
   | headroom |                  custom metadata                   | data |
   +----------+----------------------------------------------------+------+
              |                                                    |
              |                                            xdp_desc->addr
              |<------ xsk_umem__get_data() - METADATA_SIZE -------|

```
`bpf_xdp_adjust_meta` 确保 `METADATA_SIZE` 4 字节对齐，不超过 252 字节，并构建 xdp_frame 留出足够空间。如果不满足这些条件，它会返回一个负的错误码。在这种
情况下，BPF 程序不应继续`data_meta` 区域填充数据
## 示例

有关处理 XDP 元数据的 BPF 程序示例，请参见
`tools/testing/selftests/bpf/progs/xdp_metadata.c` 鍜?`tools/testing/selftests/bpf/prog_tests/xdp_metadata.c`銆?