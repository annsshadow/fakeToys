
## idpf Linux* 基础驱动，用Intel(R) 基础设施数据路径功能（Infrastructure Data Path Function

Intel idpf Linux 驱动Copyright(C) 2023 Intel Corporation

idpf 驱动同时作为 Intel(R) 基础设施数据路径功能（Infrastructure Data Path
Function）的物理功能（PF）驱动和虚拟功能（VF）驱动
可以使用 ethtool、lspci ip 获取驱动信息
关于硬件要求的相关问题，请参阅随你的 Intel 适配器提供的文档。所列出的所硬件要求均适用于在 Linux 下使用
## 识别你的适配
关于如何识别你的适配器，以及获取最新的 Intel 网络驱动，请参阅 Intel 支持网站http://www.intel.com/support

## 附加特性与配置


### ethtool

驱动利用 ethtool 接口进行驱动配置和诊断，以及显示统计信息。此功能需要最新的
ethtool 版本。如果你还没有，可以在以下地址获取https://kernel.org/pub/software/network/ethtool/

### 查看链路消息

如果发行版限制系统消息，链路消息将不会显示到控制台。为了看到网络驱动的链路
消息，执```

  # dmesg -n 8

```
   该设置不会在重启后保留
### 巨帧（Jumbo Frames
通过把最大传输单元（MTU）改为大于默认1500 的值来启用巨帧支持
使用 ip 命令增大 MTU 大小。例如，输入以下命令
```

  # ip link set mtu 9000 dev <ethX>
  # ip link set up dev <ethX>

```
   巨帧的最MTU 设置9706。这对应9728 字节的最大巨帧大小
   该驱动将尝试使用多个页大小的缓冲区来接收每个巨帧数据包。这应有助于在分   接收数据包时避免缓冲区匮乏问题
   当你使用巨帧时，丢包可能对吞吐量有更大的影响。如果在启用巨帧后观察到性能
   下降，启用流控可能会缓解该问题
## 性能优化

驱动默认值旨在适应各种各样的工作负载，但如果需要进一步优化，我们建议对以设置进行试验
### 中断速率限制

该驱动支持一种为通用工作负载调优的自适应中断节流速率（ITR）机制。用户可以通过
ethtool 自定义特定工作负载的中断速率控制，调整中断之间的微秒数
```
  # ethtool -C <ethX> adaptive-rx off adaptive-tx off

```
为了更低CPU 占用 - 禁用自适应 ITR 并降Rx Tx 中断。下面的示例影响指定接口的每个队列
 - rx-usecs tx-usecs 设为 80 会把中断限制在大```
     # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 80
     tx-usecs 80

```
为了更低的延迟：
 - 通过rx-usecs tx-usecs 设为 0 来禁用自适应 ITR ITR
```
     # ethtool -C <ethX> adaptive-rx off adaptive-tx off rx-usecs 0
     tx-usecs 0

```
每队列中断速率设置 - 以下示例针对队列 1 3，但你可以调整其它队列
 - 要禁Rx 自适应 ITR 并将静Rx ITR 设为 10 微秒，执```
     # ethtool --per-queue <ethX> queue_mask 0xa --coalesce adaptive-rx off
     rx-usecs 10

 - 要显示队1 3 当前的合并（coalesce）设置：
     # ethtool --per-queue <ethX> queue_mask 0xa --show-coalesce



```
### 虚拟化环
除本节中的其它建议外，以下内容可能有助于优化虚拟机中的性能
 - VM 中使用适当的机制（vcpupin），CPU 固定到各LCPU，确保使用包含在
   设备 local_cpulist 中的一CPUsys/class/net/<ethX>/device/local_cpulist
 - VM 中配置尽可能多的 Rx/Tx 队列（参idpf 驱动
```
     # ethtool -L <virt_interface> rx <max> tx <max>


```
## 支持

关于一般信息，请访Intel 支持网站http://www.intel.com/support/

如果发现已发布源代码在受支持的内核和受支持的适配器上存在问题，请将与该问相关的具体信息发送至 intel-wired-lan@lists.osuosl.org
## 商标

Intel Intel Corporation 或其子公司在美国或其它国地区的商标或注册商标
- 其它名称和品牌可能被视为他人的财产