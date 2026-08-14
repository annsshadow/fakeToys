## Broadcom Starfighter 2 以太网交换机驱动


Broadcom 的 Starfighter 2 以太网交换硬件模块常见于并部署于以下产品中：

- xDSL 网关，例如 BCM63138
- 流媒体/多媒体机顶盒，例如 BCM7445
- 电缆调制解调器/住宅网关，例如 BCM7145/BCM3390

该交换机通常部署在包含 5 到 13 个端口的配置中，提供一系列内置和可定制接口：

- 单个集成千兆 PHY
- 四口集成千兆 PHY
- 带 MDIO 多路复用器的四口外部千兆 PHY
- 集成 MoCA PHY
- 多个外部 MII/RevMII/GMII/RGMII 接口

该交换机还支持特定的拥塞控制特性，允许 MoCA 在主角色重选期间
不丢失数据包，以及在下行接口以较低速度连接时，向主机 CPU 网络接口
提供带外（out of band）背压。

交换机硬件模块通常通过 MMIO 访问连接，并包含一组子模块/寄存器：

- `SWITCH_CORE`：通用交换机寄存器
- `SWITCH_REG`：外部接口交换机寄存器
- `SWITCH_MDIO`：外部 MDIO 总线控制器（SWITCH_CORE 中还有一个，
  用于间接 PHY 访问）
- `SWITCH_INDIR_RW`：64 位宽的寄存器辅助模块
- `SWITCH_INTRL2_0/1`：二级中断控制器
- `SWITCH_ACB`：准入控制块（Admission control block）
- `SWITCH_FCB`：故障转移控制块（Fail-over control block）

## 实现细节


该驱动位于 `drivers/net/dsa/bcm_sf2.c`，并作为一个 DSA
驱动实现；有关该子系统及其提供的功能详情，请参见
`Documentation/networking/dsa/dsa.rst`。

SF2 交换机被配置为启用 Broadcom 特有的 4 字节交换标签
（switch tag），该标签由交换机为每一个转发到 CPU 接口的数据包
插入；相应地，CPU 网络接口也应为进入 CPU 端口的数据包插入类似的标签。
标签格式在 `net/dsa/tag_brcm.c` 中有说明。

总体而言，SF2 驱动是一个相当常规的 DSA 驱动；下面介绍一些
具体细节。

### 设备树探测（Device Tree probing）


DSA 平台设备驱动使用 `net/dsa/dsa.c` 中提供的一个特定
兼容（compatible）字符串进行探测。这样做的原因是目前 DSA 子系统
被注册为一个平台设备驱动。DSA 将提供所需的 device_node 指针，
随后可由交换机驱动的设置函数访问，以设置诸如寄存器范围和中断等资源。
目前这工作得很好，因为驱动所使用的 of_* 函数都不要求将 struct device
绑定到 struct device_node，但未来情况可能会改变。

### MDIO 间接访问


由于 Broadcom 交换机设计上的一个限制，连接到 SF2 的外部 Broadcom 交换机
需要使用 DSA 用户 MDIO 总线才能正确配置它们。默认情况下，SF2 伪 PHY 地址
和外部交换机伪 PHY 地址都会侦听传入的 MDIO 事务，因为它们位于同一地址（30），
导致某种“双重”编程。使用 DSA 并相应地设置 `ds->phys_mii_mask`，我们
有选择地将读取和写入转向外部 Broadcom 交换机的伪 PHY 地址。较新版本的
SF2 硬件引入了可配置的伪 PHY 地址，从而规避了最初的设计限制。

### 同轴多媒体（MoCA）接口


MoCA 接口相当特殊，需要使用一个固件 blob，该固件会被加载到 MoCA 处理器上
用于数据包处理。交换机硬件包含逻辑，每当 MoCA 同轴电缆断开连接或固件
重新加载时，会相应地断言/解除断言 MoCA 接口的链路状态。SF2 驱动依赖此类事件
来正确设置其 MoCA 接口的载波状态，并向网络栈 proper 报告。

MoCA 接口通过 PHY 库的固定 PHY/仿真 PHY 设备支持，交换机驱动为这些 PHY
注册一个 `fixed_link_update` 回调，以反映从中断处理程序获取的链路状态。

### 电源管理


只要有可能，SF2 驱动就会尝试通过组合以下方式来最小化整体交换机功耗：

- 关闭内部缓冲/内存
- 禁用数据包处理逻辑
- 将集成 PHY 置于 IDDQ/低功耗
- 根据活动端口数量降低交换机核心时钟
- 启用并通告 EEE
- 在链路断开时关闭 RGMII 数据处理逻辑

### 局域网唤醒（Wake-on-LAN）


局域网唤醒目前通过利用主机处理器以太网 MAC 控制器的
唤醒逻辑实现。每当请求局域网唤醒时，会对用户请求与受支持的主机以太网
接口 WoL 能力取交集，并配置交集结果。在系统级挂起/恢复期间，
只有不参与局域网唤醒的端口会被禁用。
