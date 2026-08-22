
## 图（Graphs

## _DSD


_DSD（Device Specific Data，设备特定数据）[dsd-guide] 是一个预定义ACPI 设备
配置对象，可用于传达 ACPI 规范 [acpi] 未专门涵盖的硬件特性信息。与图相关的
_DSD 扩展有两种：property（属性）[dsd-guide] 扩展hierarchical data（分层数据）
扩展。property 扩展提供通用的键值对，hierarchical data 扩展支持带有指向其他
节点引用的节点，从而形成一棵树。树中的节点可以包含property 扩展定义的属性这两种扩展一起提供了一个类树结构，树的每个节点带有零个或多个属性（键值对）
该数据结构可以在运行时通过 include/linux/fwnode.h 中定义的 device_* fwnode_*
函数来访问
fwnode 表示一个通用的固件节点对象。它独立于固件类型。在 ACPI 中，fwnode _DSD 分层数据扩展对象。一个设备的 _DSD 对象由一fwnode 表示
该数据结构可以在 ACPI 表的其它位置被引用，方法是使用对设备本身的硬引用，以在每一深度上对分层数据扩展数组的索引

## 端口（Ports）与端点（endpoints

端口和端点的概念Devicetree [devicetree, graph-bindings] 中的非常相似。端表示一个设备中的接口，端点表示对该接口的连接。另请参[data-node-ref] 了解
通用的数据节点引用
所有端口节点都位于设备 _DSD 节点下的分层数据扩展树中。与每个端口节点相关的数扩展必须"port" 开头，其后必须跟随 "@" 字符以及端口编号作为键。它引用的目对象应命名为 "PRTX"，其```

    Package() { "port@4", "PRT4" }

```
此外，端点位于端口节点之下。端点节点的分层数据扩展键必须以 "endpoint" 开头，
其后必须跟随 "@" 字符以及端点编号。它引用的对象应命名"EPXY"，其"X" 为端编号Y" 为端点编号。此类示例为
```

    Package() { "endpoint@0", "EP40" }

```
每个端口节点含有一property 扩展"port"，其值为端口编号。每个端点也类似地用
property 扩展"reg" 编号，其值为端点编号。端口编号在一个设备内必须唯一，端编号在一个端口内必须唯一。如果一个设备对象可能只有一个端口，则该端口的编号应零。类似地，如果一个端口可能只有一个端点，则该端点的编号应为零
端点引用使用"remote-endpoint" 属性的 property 扩展
```

    "device.datanode"

```
在上例中X" 为端口编号，"Y" 为端点编号
对端点的引用必须总是双向进行，既要指向远端端点，也要从被引用的远端端点节点指来
```

    Scope (\_SB.PCI0.I2C2)
    {
	Device (CAM0)
	{
	    Name (_DSD, Package () {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "compatible", Package () { "nokia,smia" } },
		},
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "port@0", "PRT0" },
		}
	    })
	    Name (PRT0, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		},
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "endpoint@0", "EP00" },
		}
	    })
	    Name (EP00, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		    Package () { "remote-endpoint", "\\_SB.PCI0.ISP.EP40" },
		}
	    })
	}
    }

    Scope (\_SB.PCI0)
    {
	Device (ISP)
	{
	    Name (_DSD, Package () {
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "port@4", "PRT4" },
		}
	    })

	    Name (PRT4, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 4 }, /* CSI-2 port number */
		},
		ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
		Package () {
		    Package () { "endpoint@0", "EP40" },
		}
	    })

	    Name (EP40, Package() {
		ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
		Package () {
		    Package () { "reg", 0 },
		    Package () { "remote-endpoint", "\\_SB.PCI0.I2C2.CAM0.EP00" },
		}
	    })
	}
    }

```
这里CAM0" 设备的端0 连接到了 "ISP" 设备的端4，反之亦然

## 参考（References

[acpi] Advanced Configuration and Power Interface 规范    https://uefi.org/specifications/ACPI/6.4/，引用日2021-11-30
[data-node-ref] Documentation/firmware-guide/acpi/dsd/data-node-references.rst

[devicetree] Devicetree。https://www.devicetree.org，引用日2016-10-03
[dsd-guide] DSD Guide    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc，引用日    2021-11-30
[dsd-rules] _DSD Device Properties Usage Rules銆?    Documentation/firmware-guide/acpi/DSD-properties-rules.rst

[graph-bindings] Common bindings for device graphs（Devicetree 设备图的通用绑定）    https://github.com/devicetree-org/dt-schema/blob/main/schemas/graph.yaml    引用日期 2021-11-30