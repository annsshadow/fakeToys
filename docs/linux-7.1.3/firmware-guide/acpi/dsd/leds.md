
## 在 ACPI 中描述与引用 LED


单个 LED 由设备节点（即 LED 驱动芯片）下的分层数据扩展 [^5^] 节点描述。LED 特定节点中的 "reg" 属性告知每个连接到 LED 的 LED 输出的数字 ID。[leds] 分层数据节点命名为 "led@X"，其中 X 是 LED 输出的编号。

在 Device Tree 中引用 LED 的文档位于 [video-interfaces] 的 "flash-leds" 属性文档中。简而言之，LED 是通过使用 phandle 直接引用的。

ACPI（与 DT 一样）允许在引用后使用整数参数。LED 驱动设备引用与一个整数参数（引用相关 LED 的 "reg" 属性）的组合，用于标识单个 LED。"reg" 属性的值是固件与软件之间的约定，它唯一地标识 LED 驱动输出。

在 LED 驱动设备下，第一个分层数据扩展包列表条目应包含字符串 "led@" 后跟 LED 的编号，再后跟被引用对象的名称。该对象应命名为 "LED" 后跟 LED 的编号。

## 示例


下面展示了一个相机传感器设备与一个带两个 LED 的 LED 驱动设备的 ASL 示例。与 LED 或对其的引用无关的对象已被
```

	Device (LED)
	{
		Name (_DSD, Package () {
			ToUUID("dbb8e3e6-5886-4ba6-8795-1319f52a966b"),
			Package () {
				Package () { "led@0", LED0 },
				Package () { "led@1", LED1 },
			}
		})
		Name (LED0, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () { "reg", 0 },
				Package () { "flash-max-microamp", 1000000 },
				Package () { "flash-timeout-us", 200000 },
				Package () { "led-max-microamp", 100000 },
				Package () { "label", "white:flash" },
			}
		})
		Name (LED1, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () { "reg", 1 },
				Package () { "led-max-microamp", 10000 },
				Package () { "label", "red:indicator" },
			}
		})
	}

	Device (SEN)
	{
		Name (_DSD, Package () {
			ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
			Package () {
				Package () {
					"flash-leds",
					Package () { "^LED.LED0", "^LED.LED1" },
				}
			}
		})
	}

```

其中
```

	LED	LED 驱动设备
	LED0	第一个 LED
	LED1	第二个 LED
	SEN	相机传感器设备（或 LED 相关的另一个设备）

```

## 参考


[acpi] Advanced Configuration and Power Interface Specification。
    https://uefi.org/specifications/ACPI/6.4/，引用日期 2021-11-30。

[data-node-ref] Documentation/firmware-guide/acpi/dsd/data-node-references.rst

[devicetree] Devicetree。https://www.devicetree.org，引用日期 2019-02-21。

[dsd-guide] DSD Guide。
    https://github.com/UEFI/DSD-Guide/blob/main/dsd-guide.adoc，引用
    日期 2021-11-30。

[leds] Documentation/devicetree/bindings/leds/common.yaml

[video-interfaces] Documentation/devicetree/bindings/media/video-interfaces.yaml
