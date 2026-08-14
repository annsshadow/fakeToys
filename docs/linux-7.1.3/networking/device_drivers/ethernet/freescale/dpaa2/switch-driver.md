
## DPAA2 交换机驱动


:Copyright: |copy| 2021 NXP

DPAA2 交换机驱动在 Datapath Switch（DPSW）对象上探测（probe），该对象可在以下 DPAA2 SoC 及其变体上实例化：LS2088A 和 LX2160A。

驱动使用交换机设备驱动模型，并把每个交换机端口作为一个网络接口暴露出来，既可以被纳入网桥，也可以作为独立接口使用。端口之间交换的流量会被卸载到硬件中。

DPSW 可以有连接到 DPNI 或连接到 DPMAC 以实现外部访问的端口。
```

         [ethA]     [ethB]      [ethC]     [ethD]     [ethE]     [ethF]
            :          :          :          :          :          :
            :          :          :          :          :          :
       [dpaa2-eth]  [dpaa2-eth]  [              dpaa2-switch              ]
            :          :          :          :          :          :        kernel
       =============================================================================
            :          :          :          :          :          :        hardware
         [DPNI]      [DPNI]     [============= DPSW =================]
            |          |          |          |          |          |
            |           ----------           |       [DPMAC]    [DPMAC]
             -------------------------------            |          |
                                                        |          |
                                                      [PHY]      [PHY]

```
## 创建一个以太网交换机


dpaa2-switch 驱动在 fsl-mc 总线上发现的 DPSW 设备上探测。这些设备既可以通过启动时配置文件——DataPath Layout（DPL）——静态创建，也可以在运行时使用 DPAA2 对象 API（已集成到 restool 用户空间工具中）创建。

目前，dpaa2-switch 驱动对它要探测的 DPSW 对象施加了以下限制：

 - FDB 的最小数量应至少等于交换机接口的数量。这是为了实现交换机端口的隔离所必需的，即当不在网桥下时，每个交换机端口将拥有自己的 FDB。
```

        fsl_dpaa2_switch dpsw.0: The number of FDBs is lower than the number of ports, cannot probe

 * 广播和洪泛（flooding）配置都应是每个 FDB 独立的。这使得驱动能够根据共享该 FDB 的交换机端口（即处于同一网桥下）来限制每个 FDB 的广播和洪泛域。
   ::

        fsl_dpaa2_switch dpsw.0: Flooding domain is not per FDB, cannot probe
        fsl_dpaa2_switch dpsw.0: Broadcast domain is not per FDB, cannot probe

 * 交换机的控制接口不应被禁用（创建时选项不应传入 DPSW_OPT_CTRL_IF_DIS）。没有控制接口，驱动就无法在交换机端口 netdevices 上提供正确的 Rx/Tx 流量支持。
   ::

        fsl_dpaa2_switch dpsw.0: Control Interface is disabled, cannot probe

```
除了实际 DPSW 对象的配置外，dpaa2-switch 驱动还需要以下 DPAA2 对象：

 - 1 个 DPMCP - 任何与 MC 固件的交互都需要一个 Management Command Portal 对象。

 - 1 个 DPBP - 一个 Buffer Pool 用于为控制接口上 Rx 路径准备的缓冲区播种。

 - 需要访问至少一个 DPIO 对象（Software Portal）才能对控制接口队列执行任何入队/出队操作。DPIO 对象将被共享，无需私有的。

## 交换特性


驱动支持在硬件中配置 L2 转发规则，用于端口桥接以及独立交换机接口的独立使用。

硬件在 VLAN 感知方面不可配置，因此任何 DPAA2
```

        $ ip link add dev br0 type bridge vlan_filtering 1

        $ ip link add dev br1 type bridge
        $ ip link set dev ethX master br1
        Error: fsl_dpaa2_switch: Cannot join a VLAN-unaware bridge

```
当设置 `stp_state 1` 时，支持通过 STP 进行拓扑和环路检测
```

        $ ip link add dev br0 type bridge vlan_filtering 1 stp_state 1

```
支持 L2 FDB 操作（添加/删除/转储）。

可以通过网桥命令在每个交换机端口上独立配置 HW FDB 学习。当禁用 HW 学习时，会运行一个快速老化（fast age）过程，任何先前学习到的地址都会被移除。
```

        $ bridge link set dev ethX learning off
        $ bridge link set dev ethX learning on

```
支持限制未知单播和组播洪泛域，但
```

        $ ip link set dev ethX type bridge_slave flood off mcast_flood off
        $ ip link set dev ethX type bridge_slave flood off mcast_flood on
        Error: fsl_dpaa2_switch: Cannot configure multicast flooding independently of unicast.

```
```

        $ echo 0 > /sys/bus/fsl-mc/devices/dpsw.Y/net/ethX/brport/broadcast_flood

```
## 卸载（Offloads）


### 路由动作（重定向、trap、drop）


DPAA2 交换机能够利用 ACL 表卸载基于流的包重定向。通过在多个端口间共享单个 ACL 表来支持共享过滤块。

支持以下流关键字：

 - Ethernet：dst_mac/src_mac
 - IPv4：dst_ip/src_ip/ip_proto/tos
 - VLAN：vlan_id/vlan_prio/vlan_tpid/vlan_dei
 - L4：dst_port/src_port

此外，matchall 过滤器可用于重定向端口上接收到的全部流量。

就流动作而言，支持以下动作：

 - drop
 - mirred egress redirect
 - trap

每个 ACL 表项（过滤器）只能配置所列动作中的一个。

示例 1：把 eth4 上接收到的、SA 为 00:01:02:03:04:05 的帧发送到
```

        $ tc qdisc add dev eth4 clsact
        $ tc filter add dev eth4 ingress flower src_mac 00:01:02:03:04:05 skip_sw action trap

```
```

        $ tc filter add dev eth4 ingress protocol 802.1q flower skip_sw vlan_id 100 vlan_prio 3 action drop

```
```

        $ tc filter add dev eth4 ingress matchall action mirred egress redirect dev eth1

```
```

        $ tc qdisc add dev eth5 ingress_block 1 clsact
        $ tc qdisc add dev eth6 ingress_block 1 clsact
        $ tc filter add block 1 ingress flower dst_mac 00:01:02:03:04:04 skip_sw \
                action trap
        $ tc filter add block 1 ingress protocol ipv4 flower src_ip 192.168.1.1 skip_sw \
                action mirred egress redirect dev eth3

```
#### 镜像（Mirroring）


DPAA2 交换机仅支持每端口镜像和每 VLAN 镜像。也支持在共享块中添加镜像过滤器。

当使用带有 802.1q 协议的 tc-flower 分类器时，只接受 ‘’vlan_id‘’ 关键字。基于任何其他字段的镜像
```

        $ tc qdisc add dev eth8 ingress_block 1 clsact
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_prio 3 action mirred egress mirror dev eth6
        Error: fsl_dpaa2_switch: Only matching on VLAN ID supported.
        We have an error talking to the kernel

```
如果在端口上请求了某个 VLAN 的镜像过滤器，则该 VLAN 必须已安装在相关交换机端口上，可以使用 ‘’bridge‘’ 或
```

        $ tc qdisc add dev eth8 ingress_block 1 clsact
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6
        Error: VLAN must be installed on the switch port.
        We have an error talking to the kernel

        $ bridge vlan add vid 200 dev eth8
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6

        $ ip link add link eth8 name eth8.200 type vlan id 200
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6

```
此外，应注意镜像流量将受到与其他任何流量相同的出口限制。这意味着当镜像数据包到达镜像端口时，如果包中发现的 VLAN 未安装在该端口上，它将被丢弃。

DPAA2 交换机只支持单一镜像目的地，因此多个
```

        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 200 action mirred egress mirror dev eth6
        $ tc filter add block 1 ingress protocol 802.1q flower skip_sw vlan_id 100 action mirred egress mirror dev eth7
        Error: fsl_dpaa2_switch: Multiple mirror ports not supported.
        We have an error talking to the kernel

```
