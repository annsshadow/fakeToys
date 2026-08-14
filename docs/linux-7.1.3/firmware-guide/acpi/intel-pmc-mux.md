## Intel North Mux-Agent


## 简介


North Mux-Agent 是 Intel PMC 固件的一项功能，在大多数带有 PMC 微控制器的
Intel 平台上是受支持的。它用于配置系统上的各种 USB 多路复用器/解复用器
（Multiplexer/DeMultiplexer）。允许从操作系统配置 mux-agent 的平台有一个
ACPI 设备对象（节点），其 HID 为 "INTC105C"，代表它。

North Mux-Agent（又称 Intel PMC Mux Control，或简称 mux-agent）驱动通过
使用 PMC IPC 方法（drivers/platform/x86/intel_scu_ipc.c）与 PMC 微控制器
通信。该驱动向 USB Type-C Mux Class 注册，从而允许 USB Type-C 控制器和
接口驱动配置线缆插头方向和模式（及交替模式，Alternate Modes）。该驱动也
向 USB Role Class 注册，以支持 USB Host 和 Device 两种模式。该驱动位于：
drivers/usb/typec/mux/intel_pmc_mux.c。

## 端口节点


### 概述


对于系统上受 mux-agent 控制的每个 USB Type-C 连接器，在 PMC mux-agent
设备节点下都有一个独立的子节点。这些节点不代表实际的连接器，而是 mux-agent
中的“通道（channel）”
```

	Scope (_SB.PCI0.PMC.MUX)
	{
	    Device (CH0)
	    {
		Name (_ADR, 0)
	    }

	    Device (CH1)
	    {
		Name (_ADR, 1)
	    }
	}

```
### _PLD（设备的物理位置，Physical Location of Device）

可选的 _PLD 对象可以与端口（通道）节点一起使用。如果 _PLD
```

	Scope (_SB.PCI0.PMC.MUX)
	{
	    Device (CH0)
	    {
		Name (_ADR, 0)
	        Method (_PLD, 0, NotSerialized)
                {
		    /* 将此视为伪代码。 */
		    Return (\_SB.USBC.CON0._PLD())
		}
	    }
	}

```
### mux-agent 专用的 _DSD 设备属性


#### 端口号

为了配置 USB Type-C 连接器背后的 mux，PMC 固件需要知道与该连接器关联的
USB2 端口和 USB3 端口。驱动通过读取名为 "usb2-port-number" 和
"usb3-port-number" 的特定 _DSD 设备属性来提取正确的端口号。这些属性具有
表示端口索引的整数值。端口索引编号是基于 1 的，值 0 是非法的。驱动在向
mux-agent 发送特定消息时，原样使用从这些设备属性中提取的数字
```

	Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	    Package() {
	        Package () {"usb2-port-number", 6},
	        Package () {"usb3-port-number", 3},
	    },
	})

```
#### 方向

根据平台的不同，来自连接器的数据线和 SBU 线从 mux-agent 的角度看可能是
“固定的（fixed）”，这意味着 mux-agent 驱动不应根据线缆插头方向配置它们。
例如，当平台上的重定时器（retimer）处理线缆插头方向时，就会发生这种情况。
驱动使用特定的设备属性 "sbu-orientation"（SBU）和 "hsl-orientation"（数据）
来了解这些线是否“固定”，以及固定到哪个方向。这些属性具有的值是字符串值，
它可以是为 USB Type-C 连接器方向定义的值之一："normal"
```

	Name (_DSD, Package () {
	    ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	    Package() {
	        Package () {"sbu-orientation", "normal"},
	        Package () {"hsl-orientation", "normal"},
	    },
	})

```
## 示例 ASL


以下 ASL 是一个示例，展示了 mux-agent 节点以及两个
```

	Scope (_SB.PCI0.PMC)
	{
	    Device (MUX)
	    {
	        Name (_HID, "INTC105C")

	        Device (CH0)
	        {
	            Name (_ADR, 0)

	            Name (_DSD, Package () {
	                ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	                Package() {
	                    Package () {"usb2-port-number", 6},
	                    Package () {"usb3-port-number", 3},
	                    Package () {"sbu-orientation", "normal"},
	                    Package () {"hsl-orientation", "normal"},
	                },
	            })
	        }

	        Device (CH1)
	        {
	            Name (_ADR, 1)

	            Name (_DSD, Package () {
	                ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
	                Package() {
	                    Package () {"usb2-port-number", 5},
	                    Package () {"usb3-port-number", 2},
	                    Package () {"sbu-orientation", "normal"},
	                    Package () {"hsl-orientation", "normal"},
	                },
	            })
	        }
	    }
	}

```
