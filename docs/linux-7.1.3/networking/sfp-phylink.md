
## phylink


## 概述


phylink 是一种支持热插拔网络模块直接连接MAC 的机制，无需在热插拔事件
重新初始化适配器

目前 phylink 支持传统的基phylib 的设置、固定链路设置以SFP（Small
Formfactor Pluggable，小尺寸可插拔）模块

## 操作模式


phylink 有多种操作模式，取决于固件设置

1. PHY 模式

   PHY 模式下，我们使用 phylib PHY 读取当前链路设置，并将其传递给 MAC
   驱动。我们期MAC 驱动精确地配置所指定的模式，而链路上不启用任何协商

2. 固定模式

   MAC 驱动而言，固定模式与 PHY 模式相同

3. 带内（in-band）模

   带内模式用于 802.3z、SGMII 以及类似的接口模式，我们期望使用并遵循跨 serdes
   通道发送的带内协商或控制字

举例来说，这意味着

  &eth {
    phy = <&phy>;
    phy-mode = "sgmii";
  };

不使用带SGMII 信号。PHY 应当严格遵循`mac_config` 函数中给定的设置
链路应在 `mac_link_up` `mac_link_down` 函数中被适当地强制为 up down

  &eth {
    managed = "in-band-status";
    phy = <&phy>;
    phy-mode = "sgmii";
  };

使用带内模式，PHY 协商的结果通过 SGMII 控制字传递给 MAC，且 MAC 应当确认
控制字。`mac_link_up` `mac_link_down` 函数不得强制 MAC 侧的链路 up down

## 将网络驱动转换为 sfp/phylink 的粗略指


本指南简要描述如何将网络驱动phylib 转换sfp/phylink 支持。欢迎提交补
来改进本文档

1. 可选地，将网络驱动phylib 更新函数拆分为处link-down link-up 
   两部分。这可以作为一个独立的准备提交来完成

   这种准备的一个较早示例可git 提交 fc548b991fb0 中找到，尽管当时的拆分是
   三部分；link-up 部分现在已包含为链路设置配置 MAC。更多相关信息请参见
   `mac_link_up`銆。

```
	select FIXED_PHY
	select PHYLIB

   with::

	select PHYLINK

   in the driver's Kconfig stanza.
```

```
	#include <linux/phylink.h>

   to the driver's list of header files.
```

```
	struct phylink *phylink;
	struct phylink_config phylink_config;

   to the driver's private data structure.  We shall refer to the
   driver's private data pointer as ``priv`` below, and the driver's
   private data structure as ``struct foo_priv``.
```

5. 替换以下函数

```
    :header-rows: 1
    :widths: 1 1
    :stub-columns: 0

    * - Original function
      - Replacement function
    * - phy_start(phydev)
      - phylink_start(priv->phylink)
    * - phy_stop(phydev)
      - phylink_stop(priv->phylink)
    * - phy_mii_ioctl(phydev, ifr, cmd)
      - phylink_mii_ioctl(priv->phylink, ifr, cmd)
    * - phy_ethtool_get_wol(phydev, wol)
      - phylink_ethtool_get_wol(priv->phylink, wol)
    * - phy_ethtool_set_wol(phydev, wol)
      - phylink_ethtool_set_wol(priv->phylink, wol)
    * - phy_disconnect(phydev)
      - phylink_disconnect_phy(priv->phylink)

   Please note that some of these functions must be called under the
   rtnl lock, and will warn if not. This will normally be the case,
   except if these are called from the driver suspend/resume paths.
```

6. 用以下方法添替换 ksettings get/set

   .. code-block:: c

	static int foo_ethtool_set_link_ksettings(struct net_device *dev,
						  const struct ethtool_link_ksettings *cmd)
	{
		struct foo_priv *priv = netdev_priv(dev);

		return phylink_ethtool_ksettings_set(priv->phylink, cmd);
	}

	static int foo_ethtool_get_link_ksettings(struct net_device *dev,
						  struct ethtool_link_ksettings *cmd)
	{
		struct foo_priv *priv = netdev_priv(dev);

		return phylink_ethtool_ksettings_get(priv->phylink, cmd);
	}
	phy_dev = of_phy_connect(dev, node, link_func, flags, phy_interface);

   以及将相关代码替换为对以下函数的调用

	err = phylink_of_phy_connect(priv->phylink, node, flags);

   在大多数情况下，``flags`` 可以为零；如DT 节点 ``node`` 中指定了 PHY，这
   flags 会被传入该函数调用内部的 phy_attach_direct()

   ``node`` 应当是包network phy 属性、fixed link 属性，并且也将包含 sfp
   属性的 DT 节点

   固定链路的设置也应被移除；这些由 phylink 在内部处理

   of_phy_connect() 还传入了一个用于链路更新的函数指针。该函数被替换为下文
   (8) 中描述的一种不同形式的 MAC 更新

   PHY supported/advertised 的操控发生在 phylink 内部，基validate 回调
   见下(8)

   注意，驱动不再需要存``phy_interface``，同时也要注``phy_interface``
   变成了一个动态属性，就像 speed、duplex 等设置一样

   最后，注意 MAC 驱动不再能直接访PHY；这是因为在 phylink 模型中，PHY 可以
   动态的

8. 向驱动中添加一`struct phylink_mac_ops <phylink_mac_ops>` 实例，它是一
   函数指针表，并实现这些函数。针`of_phy_connect` 的旧链路更新函数变成
   三个方法：`mac_link_up`、`mac_link_down` `mac_config`。如果执行了1 步，
   那么相关功能应当已经在那里被拆分了

   重要的是，如果使用了带内协商，则 `mac_link_up` `mac_link_down` 不得阻止
   带内协商完成，因为这些函数是在带内链路状态改变时被调用的——否则链路将永远
   无法建立

   `mac_get_caps` 方法是可选的，如果提供，应返回所传入 `interface` 模式所支持
   phylink MAC capabilities。一般来说，没有必要实现此方法。Phylink 会将这些
   capabilities `interface` 的允capabilities 结合，以确定允许ethtool
   链路模式

   `mac_link_state` 方法用于MAC 读取链路状态，并回MAC 当前正在使用的设置
   这对1000base-X SGMII 等带内协商方法尤为重要

   `mac_link_up` 方法用于通知 MAC 链路已经建立。该调用包含协商模式与接口，仅供
   参考。同时也会提供最终确定的链路参数（speed、duplex 与流控制/pause 使能设置），
   MAC PCS 不是紧密集成，或者设置不是来自带内协商时，应当用这些参数来配
   MAC銆。

   `mac_config` 方法用于以请求的状态更MAC，并且在MAC 配置做改动时必须避免
   不必要地让链down。这意味着该函数应当修改状态，并且仅在绝对必须改变 MAC
   配置时才让链down。关于如何做到这一点的示例，可以参
   `drivers/net/ethernet/marvell/mvneta.c` 中的 `mvneta_mac_config`

   关于这些方法的更多信息，请参`struct phylink_mac_ops <phylink_mac_ops>` 中的
   内联文档

9. 用与你的 `struct net_device <net_device>` 关联`struct device <device>`
   引用填充 `struct phylink_config <phylink_config>` 的字段：

   .. code-block:: c

	priv->phylink_config.dev = &dev.dev;
	priv->phylink_config.type = PHYLINK_NETDEV;

   填充你的 MAC 能够处理的速度、pause duplex 模式

   .. code-block:: c

        priv->phylink_config.mac_capabilities = MAC_SYM_PAUSE | MAC_10 | MAC_100 | MAC_1000FD;

10. 一些以太网控制器与 PCS（Physical Coding Sublayer，物理编码子层）块配对工作，
    PCS 除其他外还能处理编码/解码、链路建立检测与自协商。虽然某MAC 具有内部
    PCS 且其操作是透明的，但另一些则需要专门的 PCS 配置才能使链路正常工作。在那种
    情况下，phylink 通过 `struct phylink_pcs <phylink_pcs>` 提供了一PCS 抽象

    确认你的驱动是否有一个或多个内部 PCS 块，以及/或者你的控制器是否可以使用可能
    在内部连接到你控制器的外PCS 块

    如果你的控制器没有任何内PCS，可以跳到步11

    如果你的以太网控制器包含一个或多个 PCS 块，在你的驱动私有数据结构中为每PCS
    块创建一`struct phylink_pcs <phylink_pcs>` 实例

    .. code-block:: c

        struct phylink_pcs pcs;

    填充相关`struct phylink_pcs_ops <phylink_pcs_ops>` 来配置你PCS。创建一
    `pcs_get_state` 函数来报告带内链路状态、一`pcs_config` 函数来根phylink
    提供的参数配置你PCS，以及一`pcs_validate` 函数来向 phylink 报告你的 PCS
    所能接受的所有配置参数：

    .. code-block:: c

        struct phylink_pcs_ops foo_pcs_ops = {
                .pcs_validate = foo_pcs_validate,
                .pcs_get_state = foo_pcs_get_state,
                .pcs_config = foo_pcs_config,
        };

    安排PCS 链路状态中断转发进 phylink，方法是

    .. code-block:: c

        phylink_pcs_change(pcs, link_is_up);

    其中 `link_is_up` 在链路当前为 up 时为 true，否则为 false。如果某PCS 无法
    提供这些中断，那么它应在创建 PCS 时设`pcs->pcs_poll = true;`

11. 如果你的控制器依赖或接受通过自身驱动控制的外PCS 的存在，在你的驱动私
    数据结构中添加一个指phylink_pcs 实例的指针：

    .. code-block:: c

        struct phylink_pcs *pcs;

    获取实际 PCS 实例的方式取决于平台，某PCS 位于 MDIO 总线上，通过传入指向
    相应 `struct mii_bus <mii_bus>` 的指针以及该 PCS 在该总线上的地址来取得。在
    例中，我们假设控制器连接到一Lynx PCS 实例

    .. code-block:: c

        priv->pcs = lynx_pcs_create_mdiodev(bus, 0);

    某些 PCS 可以基于固件信息取得

    .. code-block:: c

        priv->pcs = lynx_pcs_create_fwnode(of_fwnode_handle(node));

12. 填充 `mac_select_pcs` 回调，并将其加入你的 `struct phylink_mac_ops
    <phylink_mac_ops>` 操作集。该函数必须返回一个指向将用于所请求链路配置的相
    `struct phylink_pcs <phylink_pcs>` 的指针：

    .. code-block:: c

        static struct phylink_pcs **foo_select_pcs(struct phylink_config **config,
                                                  phy_interface_t interface)
        {
                struct foo_priv *priv = container_of(config, struct foo_priv,
                                                     phylink_config);

                if ( /** 'interface' needs a PCS to function **/ )
                        return priv->pcs;

                return NULL;
        }

    参见 `mvpp2_select_pcs` 作为一个拥有多个内PCS 的驱动示例

13. 填充你的 MAC 能够输出的所`phy_interface_t <phy_interface_t>`（即所MAC 
    PHY 的链路模式）。下面的示例展示了针对一个能够处理所RGMII 模式、SGMII 
    1000BaseX MAC 的配置。你必须根据MAC 以及所有关PCS 的能力进行调整，
    不仅仅是你希望使用的接口

    .. code-block:: c

       phy_interface_set_rgmii(priv->phylink_config.supported_interfaces);
        __set_bit(PHY_INTERFACE_MODE_SGMII,
                  priv->phylink_config.supported_interfaces);
        __set_bit(PHY_INTERFACE_MODE_1000BASEX,
                  priv->phylink_config.supported_interfaces);

14. probe 函数中移除对 PHY of_parse_phandle()、对固定链路
    of_phy_register_fixed_link() 等调用，并替换为

    .. code-block:: c

	struct phylink *phylink;

	phylink = phylink_create(&priv->phylink_config, node, phy_mode, &phylink_ops);
	if (IS_ERR(phylink)) {
		err = PTR_ERR(phylink);
		fail probe;
	}

	priv->phylink = phylink;

    并适当安排销phylink：在 probe 失败路径以及移除路径中都通过调用以下函数
    销毁：

    .. code-block:: c

	phylink_destroy(priv->phylink);

15. 安排MAC 链路状态中断转发进 phylink，方法是

    .. code-block:: c

	phylink_mac_change(priv->phylink, link_is_up);

    其中 `link_is_up` 在链路当前为 up 时为 true，否则为 false

```
	netif_carrier_on()
	netif_carrier_off()

    as these will interfere with phylink's tracking of the link state,
    and cause phylink to omit calls via the :c:func:`mac_link_up` and
    :c:func:`mac_link_down` methods.
```

网络驱动应通过它们suspend/resume 路径调用 phylink_stop() phylink_start()
这确保了在必要时调用相应`struct phylink_mac_ops <phylink_mac_ops>` 方法

关于DT 中描SFP 笼（cage）的信息，请参阅内核源码树中的绑定文
`Documentation/devicetree/bindings/net/sff,sfp.yaml`銆。
