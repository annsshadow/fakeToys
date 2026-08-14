
## ACPI 中的 MDIO 总线与 PHY


MDIO 总线 [phy] 上的 PHY 使用 fwnode_mdiobus_register_phy() 进行探测和注册。

之后，为了将这些 PHY 连接到它们各自的 MAC，必须引用注册在
MDIO 总线上的 PHY。

本文档介绍了两个 _DSD 属性，用于将 MDIO 总线 [dsd-properties-rules] 上的
PHY 连接到 MAC 层。

这些属性是根据“用于 _DSD 的设备属性 UUID”[dsd-guide] 文档定义的，并且
包含它们的设备数据描述符（Device Data Descriptors）中必须使用
daffd814-6eba-4d8c-8a91-bc9bbf4aa301 UUID。

### phy-handle


对于每个 MAC 节点，使用设备属性 "phy-handle" 来引用
注册在 MDIO 总线上的 PHY。对于通过 MDIO 总线将 PHY 连接到 MAC 的
网络接口，这是强制性的。

在 MDIO 总线驱动初始化期间，使用该总线上的 PHY 通过如下所示的 _ADR 对象
进行探测，并注册到 MDIO 总线上。


      Scope(\_SB.MDI0)
      {
        Device(PHY1) {
          Name (_ADR, 0x1)
        } // end of PHY1

        Device(PHY2) {
          Name (_ADR, 0x2)
        } // end of PHY2
      }

之后，在 MAC 驱动初始化期间，必须从 MDIO 总线检索已注册的 PHY 设备。为此，
MAC 驱动需要引用先前注册的 PHY，这些引用作为设备对象引用提供
（例如 \_SB.MDI0.PHY1）。

### phy-mode


"phy-mode" _DSD 属性用于描述与 PHY 的连接。"phy-mode" 的有效值在
[ethernet-controller] 中定义。

### managed


可选属性，指定 PHY 管理类型。"managed" 的有效值在 [ethernet-controller] 中
定义。

### fixed-link


"fixed-link" 由 MAC 端口的一个仅数据（data-only）子节点描述，该子节点通过
分层数据扩展（UUID dbb8e3e6-5886-4ba6-8795-1319f52a966b，依据 [dsd-guide]
“_DSD 实现指南”文档）链接到 _DSD 包中。该子节点应包含一个必需属性
（"speed"）以及可能的可选属性——参数及其值的完整列表在 [ethernet-controller]
中指定。

以下 ASL 示例说明了这些属性的用法。

### MDIO 节点的 DSDT 条目


MDIO 总线有一个 SoC 组件（MDIO 控制器）和一个平台组件（MDIO 总线上的 PHY）。

a) 硅组件
### 该节点描述 MDIO 控制器，MDI0



	Scope(_SB)
	{
	  Device(MDI0) {
	    Name(_HID, "NXP0006")
	    Name(_CCA, 1)
	    Name(_UID, 0)
	    Name(_CRS, ResourceTemplate() {
	      Memory32Fixed(ReadWrite, MDI0_BASE, MDI_LEN)
	      Interrupt(ResourceConsumer, Level, ActiveHigh, Shared)
	       {
		 MDI0_IT
	       }
	    }) // end of _CRS for MDI0
	  } // end of MDI0
	}

b) 平台组件
### PHY1 和 PHY2 节点表示连接到 MDIO 总线 MDI0 的 PHY



	Scope(\_SB.MDI0)
	{
	  Device(PHY1) {
	    Name (_ADR, 0x1)
	  } // end of PHY1

	  Device(PHY2) {
	    Name (_ADR, 0x2)
	  } // end of PHY2
	}

### 表示 MAC 节点的 DSDT 条目


以下是引用了 PHY 节点的 MAC 节点。
### phy-mode 和 phy-handle 的使用如前所述。



	Scope(\_SB.MCE0.PR17)
	{
	  Name (_DSD, Package () {
	     ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package (2) {"phy-mode", "rgmii-id"},
		     Package (2) {"phy-handle", \_SB.MDI0.PHY1}
	      }
	   })
	}

	Scope(\_SB.MCE0.PR18)
	{
	  Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package (2) {"phy-mode", "rgmii-id"},
		    Package (2) {"phy-handle", \_SB.MDI0.PHY2}}
	    }
	  })
	}

### 指定了 "managed" 属性的 MAC 节点示例。



	Scope(\_SB.PP21.ETH0)
	{
	  Name (_DSD, Package () {
	     ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package () {"phy-mode", "sgmii"},
		     Package () {"managed", "in-band-status"}
		 }
	   })
	}

### 带有 "fixed-link" 子节点的 MAC 节点示例。



	Scope(\_SB.PP21.ETH1)
	{
	  Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package () {"phy-mode", "sgmii"},
		 },
	    ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		 Package () {
		     Package () {"fixed-link", "LNK0"}
		 }
	  })
	  Name (LNK0, Package(){ // Data-only subnode of port
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		 Package () {
		     Package () {"speed", 1000},
		     Package () {"full-duplex", 1}
		 }
	  })
	}

## 参考


[phy] Documentation/networking/phy.rst

[dsd-properties-rules]
    Documentation/firmware-guide/acpi/DSD-properties-rules.rst

[ethernet-controller]
    Documentation/devicetree/bindings/net/ethernet-controller.yaml

[dsd-guide] DSD Guide.
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc, 引用日期
    2021-11-30。
