
## DPAA2 MAC / PHY 支持


:Copyright: |copy| 2019 NXP

### 概述


DPAA2 MAC / PHY 支持由一API 组成，这API 帮助 DPAA2 网络驱动（dpaa2-ethdpaa2-ethsw）与 PHY 库交互
### DPAA2 软件架构


在其DPAA2 对象之中，fsl-mc 总线导出 DPNI 对象（抽象网络接口）DPMAC 对象
（抽MAC）。dpaa2-eth 驱动DPNI 对象上探测，并借助 phylink 连接并配置一DPMAC 对象
可以DPNI DPMAC 之间，或两个 DPNI 之间建立数据连接。根据连接类型的不同netif_carrier_[on/off] dpaa2-eth 驱动phylink 直接处理

  MC 固件呈现的抽象链路状态信息的来源

                                               +--------------------------------------+
  +------------+                  +---------+  |                           xgmac_mdio |
  | net_device |                  | phylink |--|  +-----+  +-----+  +-----+  +-----+  |
  +------------+                  +---------+  |  | PHY |  | PHY |  | PHY |  | PHY |  |
        |                             |        |  +-----+  +-----+  +-----+  +-----+  |
      +------------------------------------+   |                    External MDIO bus |
      |            dpaa2-eth               |   +--------------------------------------+
      +------------------------------------+
#         |                             |                                           Linux

        |                             |                                     MC firmware
        |              /|             V
  +----------+        / |       +----------+
  |          |       /  |       |          |
  |          |       |  |       |          |
  |   DPNI   |<------|  |<------|   DPMAC  |
  |          |       |  |       |          |
  |          |       \  |<---+  |          |
  +----------+        \ |    |  +----------+
                       \|    |
                             |
           +--------------------------------------+
           | MC firmware polling MAC PCS for link |
           |  +-----+  +-----+  +-----+  +-----+  |
           |  | PCS |  | PCS |  | PCS |  | PCS |  |
           |  +-----+  +-----+  +-----+  +-----+  |
           |                    Internal MDIO bus |
           +--------------------------------------+


根据 MC 固件配置设置的不同，每个 MAC 可能处于两种模式之一
- DPMAC_LINK_TYPE_FIXED：链路状态管理完全由 MC 固件通过轮询 MAC PCS 来处理。无需
  注册 phylink 实例，dpaa2-eth 驱动根本不会绑定到所连接dpmac 对象
- DPMAC_LINK_TYPE_PHY：MC 固件处于等待链路状态更新事件的状态，但这些事件实际上
  严格dpaa2-mac（基phylink）与其所连接net_device 驱动（dpaa2-eth  dpaa2-ethsw）之间传递，有效地绕过了固件
### 实现


在探测时或当 DPNI 的端点被动态更改时，dpaa2-eth 负责查明对端对象是否DPMAC如果是，则使dpaa2_mac_connect() API 将其PHYLINK 集成，该 API 将执行以操作
 - 在设备树中查找与 PHYLINK 兼容的绑定（phy-handle - 将创建一个与所接收 net_device 关联PHYLINK 实例
 - 使用 phylink_of_phy_connect() 连接PHY

实现了以phylink_mac_ops 回调
 - .validate() 将用 MAC 能力填充受支持的链路模式，仅phy_interface_t    RGMII_* 时（目前，这是驱动支持的唯一种链路类型）
 - .mac_config() 将使dpmac_set_link_state() MC 固件 API 以新配置配置 MAC
 - .mac_link_up() / .mac_link_down() 将使用上述相同的 API 更新 MAC 链路
在驱unbind() 时或DPNI 对象DPMAC 断开连接时，dpaa2-eth 驱动调用
dpaa2_mac_disconnect()，后者反过来会断开PHY 的连接并销PHYLINK 实例
DPNI-DPMAC 连接的情况下ip link set dev eth0 up' 将启动以下操作序列：

(1) .dev_open() 调用 phylink_start()(2) .mac_config() .mac_link_up() 回调PHYLINK 调用(3) 为了配置硬件 MAC，调MC 固件 API dpmac_set_link_state()(4) 固件最终会将硬MAC 设置为新配置(5) 直接PHYLINK 在关联的 net_device 上调netif_carrier_on()(6) dpaa2-eth 驱动处理 LINK_STATE_CHANGE 中断，以根据暂停帧设置启禁用 Rx
    taildrop銆。

  +---------+               +---------+
  | PHYLINK |-------------->|  eth0   |
  +---------+           (5) +---------+
  (1) ^  |
      |  |
      |  v (2)
  +-----------------------------------+
  |             dpaa2-eth             |
  +-----------------------------------+
         |                    ^ (6)
         |                    |
         v (3)                |
  +---------+---------------+---------+
  |  DPMAC  |               |  DPNI   |
  +---------+               +---------+
  |            MC Firmware            |
  +-----------------------------------+
         |
         |
         v (4)
  +-----------------------------------+
  |             HW MAC                |
  +-----------------------------------+

DPNI-DPNI 连接的情况下，通常的操作序列如下所示：

(1) ip link set dev eth0 up
(2) 在所关联fsl_mc_device 上调dpni_enable() MC API(3) ip link set dev eth1 up
(4) 在所关联fsl_mc_device 上调dpni_enable() MC API(5) LINK_STATE_CHANGED 中断dpaa2-eth 驱动的两个实例接收，因为现在操作链路状    up(6) link_state_update() 在导出的 net_device 上调netif_carrier_on()

  +---------+               +---------+
  |  eth0   |               |  eth1   |
  +---------+               +---------+
      |  ^                     ^  |
      |  |                     |  |
  (1) v  | (6)             (6) |  v (3)
  +---------+               +---------+
  |dpaa2-eth|               |dpaa2-eth|
  +---------+               +---------+
      |  ^                     ^  |
      |  |                     |  |
  (2) v  | (5)             (5) |  v (4)
  +---------+---------------+---------+
  |  DPNI   |               |  DPNI   |
  +---------+               +---------+
  |            MC Firmware            |
  +-----------------------------------+


### 导出API


任何驱动 DPMAC 对象端点DPAA2 驱动都应当处理其 _EVENT_ENDPOINT_CHANGED 中断，并
与关联的 DPMAC 连接/断开
```

 - int dpaa2_mac_connect(struct dpaa2_mac *mac);
 - void dpaa2_mac_disconnect(struct dpaa2_mac *mac);

```
只有当对DPMAC 不是 `TYPE_FIXED` 时，才需phylink 集成。这意味着它要么是
`TYPE_PHY`，要么是 `TYPE_BACKPLANE`（二者的区别在于，在 `TYPE_BACKPLANE` 模式下，
MC 固件不访PCS 寄存器）。可以检```

 - static inline bool dpaa2_mac_is_type_phy(struct dpaa2_mac *mac);

```
在连接到 MAC 之前，调用者必须分配并用关联的 net_device、要使用MC portal 指针
以及 DPMAC 的实fsl_mc_device 结构填充 dpaa2_mac 结构