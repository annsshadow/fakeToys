## PHY 抽象层


## 目的


大多数网络设备都由一组寄存器构成，这些寄存器提供了与 MAC 层的接口，而 MAC 层通过 PHY 与物理连接通信。PHY 负责与网络连接（通常是以太网线缆）另一端的链路伙伴协商链路参数，并提供一个寄存器接口，让驱动能够确定选择了哪些设置、以及配置允许哪些设置。

虽然这些设备与网络设备不同，并且寄存器遵循标准布局，但把 PHY 管理代码与网络驱动集成在一起是一种常见做法。这导致了大量冗余代码。此外，在带有多个（有时差异很大）连接到同一管理总线的以太网控制器的嵌入式系统上，很难确保对总线的安全使用。

由于 PHY 是设备，而访问它们的管理总线实际上也是总线，PHY 抽象层（PAL）就按此对待它们。这样做时，它有以下目标：

#. 提高代码复用
#. 提高整体代码的可维护性
#. 加快新网络驱动以及新系统的开发时间

基本上，这一层旨在为 PHY 设备提供一个接口，让网络驱动编写者尽可能少写代码，同时仍能提供完整的功能集。

## MDIO 总线


大多数网络设备通过一条管理总线连接到 PHY。不同的设备使用不同的总线（尽管有些共享通用接口）。为了利用 PAL，每个总线接口都需要作为一个独立的设备注册。

```

	int write(struct mii_bus *bus, int mii_id, int regnum, u16 value);
	int read(struct mii_bus *bus, int mii_id, int regnum);

   mii_id is the address on the bus for the PHY, and regnum is the register
   number.  These functions are guaranteed not to be called from interrupt
   time, so it is safe for them to block, waiting for an interrupt to signal
   the operation is complete

```
#. 复位函数是可选的。它用于将总线返回到初始化状态。

#. 需要一个 probe 函数。这个函数应当设置总线驱动所需的任何东西，设置 mii_bus 结构体，并使用 mdiobus_register 向 PAL 注册。类似地，还有一个 remove 函数用于撤销所有这些（使用 mdiobus_unregister）。

#. 像任何驱动一样，必须配置 device_driver 结构体，并使用 init 和 exit 函数来注册该驱动。

#. 该总线还必须在某处被声明为一个设备，并注册。

关于一个驱动如何实现 mdio 总线驱动的示例，请参见 drivers/net/ethernet/freescale/fsl_pq_mdio.c 以及其中一个用户对应的 DTS 文件。（例如 "git grep fsl,.*-mdio arch/powerpc/boot/dts/"）

## (RG)MII/电气接口考量


Reduced Gigabit Medium Independent Interface（RGMII，精简千兆媒体独立接口）是一个 12 针的电信号接口，使用同步的 125Mhz 时钟信号和若干数据线。由于这一设计决定，必须在时钟线（RXC 或 TXC）与数据线之间加入 1.5ns 到 2ns 的延迟，以便 PHY（时钟接收端）有足够大的建立与保持时间来正确采样数据线。PHY 库提供了不同类型的 PHY_INTERFACE_MODE_RGMII* 值，让 PHY 驱动（以及可选的 MAC 驱动）实现所需的延迟。phy_interface_t 的取值必须从 PHY 设备自身的角度来理解，由此得出以下内容：

- PHY_INTERFACE_MODE_RGMII：PHY 不负责自行插入任何内部延迟，它假设要么是以太网 MAC（如果有能力）要么 PCB 走线插入了正确的 1.5-2ns 延迟

- PHY_INTERFACE_MODE_RGMII_TXID：PHY 应当为 PHY 设备处理的发送数据线（TXD[3:0]）插入内部延迟

- PHY_INTERFACE_MODE_RGMII_RXID：PHY 应当为 PHY 设备处理的接收数据线（RXD[3:0]）插入内部延迟

- PHY_INTERFACE_MODE_RGMII_ID：PHY 应当为进出 PHY 设备的发送****和****接收数据线都插入内部延迟

只要可能，出于以下原因应使用 PHY 侧的 RGMII 延迟：

- PHY 设备可能在接收/发送侧延迟的指定上提供亚纳秒级粒度（例如：0.5、1.0、1.5ns）。这种精度可能是处理 PCB 走线长度差异所必需的

- PHY 设备通常适用于大范围的应用（工业、医疗、汽车……），并且它们在温度/压力/电压范围内提供恒定且可靠的延迟

- PHYLIB 中的 PHY 设备驱动本质上可复用，能够正确配置指定的延迟，可使更多具有类似延迟需求的设计正确工作

对于 PHY 无法提供该延迟、但以以太网 MAC 驱动能够提供的情况，正确的 phy_interface_t 值应当是 PHY_INTERFACE_MODE_RGMII，并且以太网 MAC 驱动应当被正确配置，以便从 PHY 设备的角度提供所需的发送和/或接收侧延迟。反之，如果以太网 MAC 驱动查看 phy_interface_t 值，对于除 PHY_INTERFACE_MODE_RGMII 之外的任何模式，它应当确保 MAC 级的延迟被禁用。

如果以太网 MAC 和 PHY 都无法按 RGMII 标准的定义提供所需的延迟，可能有以下几种选择：

- 某些 SoC 可能提供一个引脚焊盘/mux/控制器，能够配置给定一组引脚的驱动强度、延迟和电压；它可能是插入预期 2ns RGMII 延迟的合适选项。

- 修改 PCB 设计以包含固定延迟（例如：使用专门设计的蛇形走线），这可能完全不需要软件配置。

### RGMII 延迟不匹配的常见问题
当以太网 MAC 与 PHY 之间存在 RGMII 延迟不匹配时，这极可能导致时钟与数据线信号在 PHY 或 MAC 对这些信号采样、将其转换为逻辑 1 或 0 状态并重建收发数据时不稳定。典型症状包括：

- 发送/接收部分工作，并且观察到频繁或偶发的丢包

- 以太网 MAC 可能报告部分或全部入向数据包带有 FCS/CRC 错误，或者直接把它们全部丢弃

- 切换到较低速率（如 10/100Mbits/sec）时问题消失（因为此时有足够的建立/保持时间）

## 连接到 PHY


在启动过程中的某个时刻，网络驱动需要在 PHY 设备与网络设备之间建立连接。此时，PHY 的总线和驱动都需要已经加载好，以便为连接做好准备。此时有几种连接到 PHY 的方式：

#. PAL 处理一切，并且只在链路状态变化时调用网络驱动，以便它能够做出反应。

#. PAL 处理除中断之外的一切（通常是因为控制器拥有中断寄存器）。

#. PAL 处理一切，但每秒与驱动核对一次，让网络驱动在 PAL 之前先对任何变化做出反应。

#. PAL 仅作为函数库使用，由网络设备手动调用函数来更新状态、并配置 PHY。

## 让 PHY 抽象层处理一切


如果你选择选项 1（希望每个驱动都能如此，但对不能的驱动仍有用处），连接到 PHY 很简单：

首先，你需要一个函数来对链路状态的变化做出反应。这个
```

	static void adjust_link(struct net_device *dev);

```
接下来，你需要知道连接到此设备的 PHY 的设备名。名字看起来类似 "0:00"，其中第一个数字是总线 id，第二个是该总线上 PHY 的地址。通常，总线负责使其 ID 唯一。

```

	phydev = phy_connect(dev, phy_name, &adjust_link, interface);

```
**phydev** 是一个指向代表 PHY 的 phy_device 结构体的指针。如果 phy_connect 成功，它会返回该指针。这里的 dev 是指向你的 net_device 的指针。一旦完成，这个函数就会启动 PHY 的软件状态机，并在 PHY 有中断时为其注册中断。phydev 结构体会被填入关于当前状态的信息，尽管此时 PHY 尚未真正可运行。

PHY 特定的标志应当在调用 phy_connect() 之前设置到 phydev->dev_flags 中，以便底层 PHY 驱动能够检查这些标志并据此执行特定操作。如果系统对 PHY/控制器施加了硬件限制、而 PHY 需要知道这些限制，这就很有用。

**interface** 是一个 u32，指定控制器与 PHY 之间使用的连接类型。例如 GMII、MII、RGMII 和 SGMII。参见下文的“PHY 接口模式”。完整列表见 include/linux/phy.h。

现在只需确保从 phydev->supported 和 phydev->advertising 中剪除对你的控制器无意义的值（一个 10/100 控制器可能连接到一个支持千兆的 PHY，因此你需要屏蔽掉 SUPPORTED_1000baseT*）。这些位域的定义见 include/linux/ethtool.h。注意你不应 SET 任何位，除非是 SUPPORTED_Pause 和 SUPPORTED_AsymPause 位（见下文），否则 PHY 可能进入不受支持的状态。

最后，一旦控制器准备好处理网络流量，你就调用 phy_start(phydev)。这告诉 PAL 你已经就绪，并配置 PHY 连接到网络。如果你网络驱动的 MAC 中断也处理 PHY 状态变化，只需在调用 phy_start 之前把 phydev->irq 设为 PHY_MAC_INTERRUPT，并在网络驱动中使用 phy_mac_interrupt()。如果你不想使用中断，把 phydev->irq 设为 PHY_POLL。phy_start() 会启用 PHY 中断（若适用）并启动 phylib 状态机。

当你想断开与网络的连接时（即便只是短暂断开），你调用 phy_stop(phydev)。这个函数也会停止 phylib 状态机并禁用 PHY 中断。

## PHY 接口模式


phy_connect() 系列函数所提供的 PHY 接口模式，定义了 PHY 接口的初始运行模式。这并不保证保持不变；有些 PHY 会根据协商结果、在无需软件介入的情况下动态改变其接口模式。

下面描述其中一些接口模式：

`PHY_INTERFACE_MODE_SMII`
    这是串行 MII，以 125MHz 时钟运行，支持 100M 和 10M 速率。
    部分细节可参见
    https://opencores.org/ocsvn/smii/smii/trunk/doc/SMII.pdf

`PHY_INTERFACE_MODE_1000BASEX`
    这定义了 802.3 标准第 36 节所规定的 1000BASE-X 单通道 serdes 链路。该链路以 1.25Gbaud 的固定比特率运行，使用 10B/8B 编码方案，从而得到 1Gbps 的底层数据率。数据流中嵌入了一个 16 位控制字，用于与远端协商双工和暂停模式。这不包括“升频”变体（如 2.5Gbps 速率，见下文）。

`PHY_INTERFACE_MODE_2500BASEX`
    这定义了 1000BASE-X 的一个变体，其时钟速度是 802.3 标准的 2.5 倍，得到 3.125Gbaud 的固定比特率。

`PHY_INTERFACE_MODE_SGMII`
    这用于 Cisco SGMII，它是 802.3 标准所定义的 1000BASE-X 的一种修改。SGMII 链路由单条以 1.25Gbaud 固定比特率运行、采用 10B/8B 编码的 serdes 通道组成。底层数据率为 1Gbps，更慢的 100Mbps 和 10Mbps 速率通过对每个数据符号进行复制来实现。802.3 控制字被改作他用，用于把协商得到的速度和双工信息从 PHY 发送给 MAC，并由 MAC 确认收到。这不包括“升频”变体（如 2.5Gbps 速率）。

    注意：在某些情形下，链路上的 SGMII 与 1000BASE-X 配置不匹配仍能成功传输数据，但 16 位控制字不会被正确解释，这可能导致双工、暂停或其他设置的不匹配。这取决于 MAC 和/或 PHY 的行为。

`PHY_INTERFACE_MODE_5GBASER`
    这是 IEEE 802.3 第 129 条定义的 5GBASE-R 协议。它与第 49 条定义的 10GBASE-R 协议相同，唯一例外是它以一半的频率运行。定义请参阅 IEEE 标准。

`PHY_INTERFACE_MODE_10GBASER`
    这是 IEEE 802.3 第 49 条定义的 10GBASE-R 协议，用于各种不同的介质。定义请参阅 IEEE 标准。

    注意：10GBASE-R 只是可以与 XFI 和 SFI 一起使用的协议之一。XFI 和 SFI 允许在单条 SERDES 通道上使用多种协议，并且在主机 XFP/SFP 连接器插入主机合规板时，还定义了信号的电气特性。因此，XFI 和 SFI 本身并不是 PHY 接口类型。

`PHY_INTERFACE_MODE_10GKR`
    这是 IEEE 802.3 第 49 条定义的、带有第 73 条自动协商的 10GBASE-R。更多信息请参阅 IEEE 标准。

    注意：由于历史用法，一些 10GBASE-R 用法错误地使用了这个定义。

`PHY_INTERFACE_MODE_25GBASER`
    这是 IEEE 802.3 PCS 第 107 条定义的 25GBASE-R 协议。其 PCS 与 10GBASE-R 相同，即以 2.5 倍速度运行的 64B/66B 编码，得到 25.78125 Gbaud 的固定比特率。更多信息请参阅 IEEE 标准。

`PHY_INTERFACE_MODE_100BASEX`
    这定义了 IEEE 802.3 第 24 条。该链路以 125Mpbs 的固定数据率运行，使用 4B/5B 编码方案，得到 100Mpbs 的底层数据率。

`PHY_INTERFACE_MODE_QUSGMII`
    这定义了 Cisco 的 Quad USGMII 模式，即 USGMII（Universal SGMII）链路的 Quad 变体。它与 QSGMII 非常相似，但使用 Packet Control Header（PCH，数据包控制头）而非 7 字节前导码，不仅携带端口 id，还携带所谓的“扩展”。规范中迄今为止唯一有文档记载的扩展是包含时间戳，用于支持 PTP 的 PHY。这种模式与 QSGMII 不兼容，但在链路速率和协商方面提供相同的能力。

`PHY_INTERFACE_MODE_1000BASEKX`
    这是 IEEE 802.3 第 36 条定义的、带有第 73 条自动协商的 1000BASE-X。通常它会与第 70 条 PMD 一起使用。与用于第 38 和 39 条 PMD 的 1000BASE-X phy 模式相比，这种接口模式具有不同的自动协商，并且只支持全双工。

`PHY_INTERFACE_MODE_PSGMII`
    这是 Penta SGMII 模式，类似于 QSGMII，但它把 5 条 SGMII 线合并成单条链路，而 QSGMII 是 4 条。

`PHY_INTERFACE_MODE_10G_QXGMII`
    表示 Cisco USXGMII 多端口铜接口文档所定义的 10G-QXGMII PHY-MAC 接口。它在一条 10.3125 GHz 的 SerDes 通道上支持 4 个端口，每个端口的速率为 2.5G / 1G / 100M / 10M，通过符号复制实现。PCS 期望标准的 USXGMII 码字。

`PHY_INTERFACE_MODE_MIILITE`
    非标准的、简化的 MII 模式，没有为 MII 定义的 TXER、RXER、CRS 和 COL 信号。缺少 COL 信号使得半双工链路模式不可能，但并不会干扰 Broadcom（以及其他两线以太网）PHY 上的 BroadR-Reach 链路模式，因为它们只支持全双工。

## 暂停帧 / 流控


除了确保在 MII_ADVERTISE 中设置 SUPPORTED_Pause 和 SUPPORTED_AsymPause 位、以向链路伙伴表明以太网 MAC 控制器支持此类功能之外，PHY 并不直接参与流控/暂停帧。由于流控/暂停帧的生成涉及以太网 MAC 驱动，建议该驱动通过相应地设置 SUPPORTED_Pause 和 SUPPORTED_AsymPause 位，来妥善指示对此类特性的通告与支持。这可以在 phy_connect() 之前或之后完成，也可以是实现 **ethtool** 的 set_pauseparam 特性的结果。

## 密切关注 PAL


有可能 PAL 内置的状态机需要一点帮助，才能让你的网络设备和 PHY 保持正确同步。如果是这样，你可以在连接到 PHY 时注册一个辅助函数，它会在状态机对任何变化做出反应之前每秒被调用。要做到这一点，你需要手动调用 phy_attach() 和 phy_prepare_link()，然后把 phy_start_machine() 的第二个参数设为指向你的特殊处理函数。

目前还没有关于如何使用这一功能的示例，并且由于作者没有任何使用它的驱动（它们都使用选项 1），对它的测试也很有限。因此 Caveat Emptor（买者自负）。

## 全部自己动手


PAL 内置的状态机有可能无法跟踪
PHY 与你的网络设备之间的复杂交互。如果是这样，你可以简单地调用 phy_attach()，而不调用 phy_start_machine 或 phy_prepare_link()。这意味着 phydev->state 完全由你来处理（phy_start 和 phy_stop 会在某些状态之间切换，所以你可能需要避开它们）。

已经做出了努力，以确保在没有状态机运行的情况下也能访问有用的功能，并且这些函数大多源自那些不与复杂状态机交互的函数。然而，同样，目前还没有做出在不运行状态机的情况下进行测试的努力，所以使用者当心。

```

 int phy_read(struct phy_device *phydev, u16 regnum);
 int phy_write(struct phy_device *phydev, u16 regnum, u16 val);

```
简单的读/写原语。它们调用总线的读/写函数指针。
```

 void phy_print_status(struct phy_device *phydev);

```
一个整洁地打印 PHY 状态的便捷函数。
```

 void phy_request_interrupt(struct phy_device *phydev);

```
请求 PHY 中断的 IRQ。
```

 struct phy_device * phy_attach(struct net_device *dev, const char *phy_id,
		                phy_interface_t interface);

```
把一个网络设备连接到一个特定的 PHY，如果在总线初始化期间没有找到驱动，就把 PHY 绑定到一个通用驱动。
```

 int phy_start_aneg(struct phy_device *phydev);

```
使用 phydev 结构体内部的变量，要么配置通告并重置自动协商，要么禁用自动协商，并配置强制设置。
```

 static inline int phy_read_status(struct phy_device *phydev);

```
用 PHY 中当前设置的最新信息填充 phydev 结构体。
```

 int phy_ethtool_ksettings_set(struct phy_device *phydev,
                               const struct ethtool_link_ksettings *cmd);

```
Ethtool 便捷函数。
```

 int phy_mii_ioctl(struct phy_device *phydev,
                   struct mii_ioctl_data *mii_data, int cmd);

```
MII ioctl。注意，如果你写入像 BMCR、BMSR、ADVERTISE 等寄存器，这个函数会彻底搞乱状态机。最好只把它用于写入非标准、且不会触发重新协商的寄存器。

## PHY 设备驱动


有了 PHY 抽象层，为新的 PHY 添加支持就相当容易。在某些情况下，根本不需要做任何工作！然而，许多 PHY 需要一点“手把手”引导才能启动运行。

### 通用 PHY 驱动


如果目标 PHY 没有任何你想要支持的勘误、怪癖或特殊特性，那么最好不要添加支持，而是让 PHY 抽象层的通用 PHY 驱动来完成所有工作。

### 编写 PHY 驱动


如果你确实要编写 PHY 驱动，首先要做的是确保它能与合适的 PHY 设备匹配。这是在总线初始化期间，通过读取设备的 UID（存储在寄存器 2 和 3 中），然后把它与每个驱动的 phy_id 字段按位与（AND）每个驱动的
```

   static struct phy_driver dm9161_driver = {
         .phy_id         = 0x0181b880,
	 .name           = "Davicom DM9161E",
	 .phy_id_mask    = 0x0ffffff0,
	 ...
   }

```
来进行比较。

接下来，你需要指定你的 PHY 设备和驱动支持哪些特性（速率、双工、自协商等）。大多数 PHY 支持 PHY_BASIC_FEATURES，但你可以在 include/mii.h 中查找其他特性。

每个驱动由若干函数指针组成，这些在 include/linux/phy.h 的 phy_driver 结构体下有文档说明。

其中，只有 config_aneg 和 read_status 必须由驱动代码赋值。其余都是可选的。此外，应尽可能使用通用 PHY 驱动的这两个函数的版本：genphy_read_status 和 genphy_config_aneg。如果做不到，很可能你只需要在调用这些函数之前和之后执行一些操作，因此你的函数会包装这些通用函数。

欢迎查看 drivers/net/phy/ 中的 Marvell、Cicada 和 Davicom 驱动作为示例（在撰写本文时，lxt 和 qsemi 驱动尚未被测试）。

PHY 的 MMD 寄存器访问默认由 PAL 框架处理，但如果有需要，也可以被特定的 PHY 驱动覆盖。如果一个 PHY 在 MMD PHY 寄存器定义被 IEEE 标准化之前就发布用于生产，就可能出现这种情况。大多数现代 PHY 都能使用通用的 PAL 框架来访问 PHY 的 MMD 寄存器。这种用法的一个例子是 PHY 抽象层实现的节能以太网（Energy Efficient Ethernet）支持。如果 PHY 支持 IEEE 标准访问机制，该支持使用 PAL 访问 MMD 寄存器以进行 EEE 查询和配置；如果被特定 PHY 驱动覆盖，也可以使用 PHY 特定的访问接口。参见 drivers/net/phy/ 中的 Micrel 驱动，了解如何实现这一点。

## 板级修复（Board Fixups）


有时，平台与 PHY 之间的特定交互需要特殊处理。例如，改变 PHY 时钟输入的位置，或者为数据路径中的延迟问题增加延迟。为了支持此类意外情况，PHY 层允许平台代码注册在 PHY 被拉起（或随后重置）时运行的修复程序。

当 PHY 层拉起一个 PHY 时，它会检查是否为它注册了任何修复程序，匹配依据是 UID（包含在 PHY 设备的 phy_id 字段中）和总线标识符（包含在 phydev->dev.bus_id 中）。两者都必须匹配，不过提供了两个常量 PHY_ANY_ID 和 PHY_ANY_UID，分别作为总线 ID 和 UID 的通配符。

当找到匹配时，PHY 层会调用与该修复程序关联的 run 函数。这个函数会传入一个指向相关 phy_device 的指针。因此它应当只操作那个 PHY。

```

 int phy_register_fixup_for_uid(u32 phy_uid, u32 phy_uid_mask,
		int (*run)(struct phy_device *));
 int phy_register_fixup_for_id(const char *phy_id,
		int (*run)(struct phy_device *));

```
## 标准


IEEE 标准 802.3：CSMA/CD 访问方法与物理层规范，第二部分：
http://standards.ieee.org/getieee802/download/802.3-2008_section2.pdf

RGMII v1.3:
http://web.archive.org/web/20160303212629/http://www.hp.com/rnd/pdfs/RGMIIv1_3.pdf

RGMII v2.0:
http://web.archive.org/web/20160303171328/http://www.hp.com/rnd/pdfs/RGMIIv2_0_final_hp.pdf
