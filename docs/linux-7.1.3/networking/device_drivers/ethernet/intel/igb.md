
## 适用于 Intel(R) 以太网网络连接的 Linux 基础驱动


Intel 千兆 Linux 驱动。
Copyright(c) 1999-2018 Intel Corporation.

## 目录


- 识别你的适配器
- 命令行参数
- 附加配置
- 支持

## 识别你的适配器


有关如何识别你的适配器以及获取最新 Intel 网络驱动的信息，请参阅 Intel 支持网站：
https://www.intel.com/support


## 命令行参数


如果驱动被构建为模块，可以通过以下方式在命令行上使用 modprobe 命令输入以下可选参数：
```

    modprobe igb [<option>=<VAL1>,<VAL2>,...]

```
对于本驱动支持的系统中的每个网络端口，都需要有一个 <VAL#>。这些值将按功能顺序应用到每个实例。
```

    modprobe igb max_vfs=2,4

```
在这种情况下，系统中由 igb 支持的网络端口有两个。

注意：描述符（descriptor）描述了一个数据缓冲区以及与该数据缓冲区相关的属性。这些信息由硬件访问。

### max_vfs


:Valid Range: 0-7

此参数添加对 SR-IOV 的支持。它使驱动生成最多 max_vfs 个虚拟功能（VF）。如果该值大于 0，它还会强制 VMDq 参数至少为 1。

驱动的参数按位置引用。因此，如果你的系统中有双端口适配器或多个适配器，并且希望每个端口有 N 个虚拟功能，则必须为每个端口用每个参数指定一个数字
```

    modprobe igb max_vfs=4

```
这将在第一个端口上生成 4 个 VF。
```

    modprobe igb max_vfs=2,4

```
这将在第一个端口上生成 2 个 VF，在第二个端口上生成 4 个 VF。

注意：使用这些参数加载驱动时必须谨慎。取决于你的系统配置、插槽数量等，无法在所有情况下预测命令行上的位置。

注意：设备和驱动都不控制 VF 如何映射到配置空间。总线布局会因操作系统而异。在支持的操作系统上，你可以检查 sysfs 来查找映射。

注意：当 SR-IOV 模式或 VMDq 模式启用时，硬件 VLAN 过滤和 VLAN 标签剥离/插入将保持启用。请移除旧的
```

    ip link set eth0 vf 0 vlan 100	// set vlan 100 for VF 0
    ip link set eth0 vf 0 vlan 0	// Delete vlan 100
    ip link set eth0 vf 0 vlan 200	// set a new vlan 200 for VF 0

```
### Debug


:Valid Range: 0-16 (0=none,...,16=all)
:Default Value: 0

此参数调整系统日志中显示的调试消息级别。


## 附加特性与配置


### Jumbo Frames


Jumbo Frames 支持通过更改最大传输单元（MTU）为大于默认值 1500 的值来启用。

使用 ifconfig 命令增大 MTU 大小。例如，输入
```

    ifconfig eth<x> mtu 9000 up

```
```

    ip link set mtu 9000 dev eth<x>
    ip link set up dev eth<x>

```
此设置不会在重启后保留。可以通过在文件中添加 'MTU=9000' 使设置更改永久生效：

- 对于 RHEL：/etc/sysconfig/network-scripts/ifcfg-eth<x>
- 对于 SLES：/etc/sysconfig/network/<config_file>

注意：Jumbo Frames 的最大 MTU 设置为 9216。该值与 9234 字节的最大 Jumbo Frames 大小一致。

注意：不支持在 10 或 100 Mbps 下使用 Jumbo 帧，可能会导致性能下降或链路丢失。


### ethtool


驱动利用 ethtool 接口进行驱动配置和诊断，以及显示统计信息。此功能需要最新版本的 ethtool。可在此处下载：

https://www.kernel.org/pub/software/network/ethtool/


### 启用局域网唤醒（WoL）


WoL 通过 ethtool 实用程序配置。

在下一次关机或重启期间，系统上将启用 WoL。对于此驱动版本，要启用 WoL，必须在关闭或挂起系统之前加载 igb 驱动。

注意：Wake on LAN 仅在多端口设备的端口 A 上受支持。此外，以下设备不支持 Wake On LAN：
- Intel(R) Gigabit VT Quad Port Server Adapter


### 多队列（Multiqueue）


在此模式下，为每个队列分配一个单独的 MSI-X 向量，并为“其他”中断（如链路状态变化和错误）分配一个。所有中断都通过中断调节（interrupt moderation）进行节流。必须使用中断调节，以避免在驱动处理一个中断时发生中断风暴。调节值至少应不小于驱动处理一个中断的预期时间。默认情况下多队列是关闭的。

要求：多队列需要 MSI-X 支持。如果未找到 MSI-X，系统将回退到 MSI 或传统（Legacy）中断。本驱动在所有支持 MSI-X 的内核上都支持接收多队列。

注意：在某些内核上，需要在单队列模式和多队列模式之间切换时重启。


### MAC 和 VLAN 防欺骗特性


当恶意驱动尝试发送欺骗（spoofed）数据包时，它会被硬件丢弃而不被发送。

会向 PF 驱动发送一个中断，通知其发生了欺骗尝试。当检测到欺骗数据包时，PF 驱动会向系统日志（由 "dmesg" 命令显示）发送以下消息：Spoof event(s) detected on VF(n)，其中 n = 尝试进行欺骗的 VF。


### 使用 IProute2 工具设置 MAC 地址、VLAN 和速率限制


你可以使用 IProute2 工具设置虚拟功能（VF）的 MAC 地址、默认 VLAN 和速率限制。如果你的版本没有你需要的全部特性，请从 Sourceforge 下载最新版本的 IProute2 工具。


### 基于信用的整形器（CBS，Qav 模式）


在硬件卸载（hardware offload）模式下启用 CBS qdisc 时，使用 CBS 算法（在 IEEE 802.1Q-2018 第 8.6.8.2 节描述并在附录 L 中讨论）的流量整形将在 i210 控制器中运行，因此更准确且占用更少的 CPU。

当使用卸载的 CBS 且流量速率遵守配置的速率（不超过它）时，CBS 对延迟应几乎没有影响。

该算法的卸载版本有一些限制，这是由于空闲斜率（idle slope）在适配器寄存器中的表示方式造成的。它只能以 16.38431 kbps 为单位表示空闲斜率，这意味着如果请求 2576kbps 的空闲斜率，控制器将被配置为使用约 2589 kbps 的空闲斜率，因为驱动会将该值向上取整。更多细节，请参见 `igb_config_tx_modes()` 的注释。

注意：此特性仅限 i210 型号。


## 支持


有关通用信息，请访问 Intel 支持网站：
https://www.intel.com/support/

如果在受支持的内核和受支持的适配器上，已发布的源代码中发现问题，请将与该问题相关的具体信息发送到 intel-wired-lan@lists.osuosl.org。
