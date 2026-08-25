

## AMD Sensor Fusion Hub


AMD Sensor Fusion Hub（SFH）是从基Ryzen 的平台开始、作SoC 一部分存在的组件该方案在多个 OEM 产品上运行良好。AMD SFH 使用 HID over PCIe 总线。在架构上它类似
ISH，但主要区别在于所HID 报告都是在核心驱动中生成的
### 框图


```

	---------------------------------
	|  HID User Space Applications  |
	- -------------------------------

    ---------------------------------------------
	 ---------------------------------
	|		HID Core          |
	 ---------------------------------

	 ---------------------------------
	|     AMD HID Transport           |
	 ---------------------------------

	 --------------------------------
	|             AMD HID Client     |
	|	with HID Report Generator|
	 --------------------------------

	 --------------------------------
	|     AMD MP2 PCIe Driver        |
	 --------------------------------
    OS
    ---------------------------------------------
    Hardware + Firmware
         --------------------------------
         |     SFH MP2 Processor         |
         --------------------------------


```
### AMD HID 浼犺緭灞。

AMD SFH 传输也被实现为一个总线。在 AMD MP2 中执行的每个客户端应用程序都被注册为总线上的一个设备。这里，MP2 是一个连接到 x86 用于处理传感器数据的 ARM 核心。将每个
设备（AMD SFH HID 驱动）绑定、识别设备类型并注册HID 核心的层，会将一个常“struct hid_ll_driver”对象与每个设备关联。一旦设备注册到 HID 核心，HID 核心便使通过该结构体提供的回调来与设备通信。AMD HID 传输层实现了同步调用
### AMD HID 客户端层


该层负责实现 HID 请求和描述符。由于固件是与操作系统无关的，HID 客户端层填充 HID
请求结构和描述符。HID 客户端层比较复杂，因为它MP2 PCIe 层与 HID 之间的接口。HID
客户端层初始MP2 PCIe 层并持有 MP2 层的实例。它使用 MP2-PCIe 层识别连接的传感器数量据此为每个传感器分配 DRAM 地址，并将其传递给 MP2-PCIe 驱动。在枚举每个传感器时，客户端
层填HID 描述符结构和 HID 输入报告结构。HID 特性报告结构是可选的。报告描述符结构传感器而异
### AMD MP2 PCIe 灞。

MP2 PCIe 层负责通过 PCIe 与固件进行所有事务。固件与 PCIe 之间的连接建立就发生在这里
X86 MP2 之间的通信分为三部分1. 通过 C2P 邮箱寄存器进行命令传输2. 通过 DRAM 进行数据传输3. 通过 P2C 寄存器获取受支持的传感器信息
命令通过 C2P 邮箱寄存器发送给 MP2。写C2P 消息寄存器会产生一个向 MP2 的中断。客户端
层分配物理内存，并通过 PCI 层将其发送给 MP2。MP2 固件将命令输出写入客户端层已分配访问 DRAM 内存。固件总是DRAM 写入至少 32 字节。因此，协议驱动应分配至32 字节DRAM 空间
### 枚举与探测流

```

       HID             AMD            AMD                       AMD -PCIe             MP2
       Core         Transport      Client layer                   layer                FW
        |		|	       |                           |                 |
        |		|              |                 on Boot Driver Loaded       |
        |		|	       |                           |                 |
        |		|	       |                        MP2-PCIe Int         |
        |		|              |			   |                 |
        |		|	       |---Get Number of sensors-> |                 |
        |		|              |                       Read P2C              |
        |		|	       |			Register             |
        |		|              |                           |                 |
        |               |              | Loop(for No of Sensors)   |                 |
        |		|	       |----------------------|    |                 |
        |		|              | Create HID Descriptor|    |                 |
        |		|	       | Create Input  report |    |                 |
        |		|              |  Descriptor Map      |    |                 |
        |		|	       |  the MP2 FW Index to |    |                 |
        |		|              |   HID Index          |    |                 |
        |		|	       | Allocate the DRAM    |  Enable              |
        |		|	       |	address       |  Sensors             |
        |		|              |----------------------|    |                 |
        |		| HID transport|                           |    Enable       |
        |	        |<--Probe------|                           |---Sensor CMD--> |
        |		| Create the   |			   |                 |
        |		| HID device   |                           |                 |
        |               |    (MFD)     |                           |                 |
        |		| by Populating|			   |                 |
        |               |  the HID     |                           |                 |
        |               |  ll_driver   |                           |                 |
        | HID           |	       |			   |                 |
        |  add          |              |                           |                 |
        |Device         |              |                           |                 |
        |<------------- |	       |			   |                 |

```
### 从应用程序到 AMD SFH 驱动的数据流


```

	        |	       |              |	  	 	          |		    |
                |	       |	      |			          |                 |
                |	       |	      |			          |                 |
                |              |              |                           |                 |
                |              |              |                           |                 |
                |HID_req       |              |                           |                 |
                |get_report    |              |                           |                 |
                |------------->|              |                           |                 |
	        |              | HID_get_input|                           |                 |
	        |              |  report      |                           |                 |
	        |              |------------->|------------------------|  |                 |
	        |              |              |  Read the DRAM data for|  |                 |
	        |              |              |  requested sensor and  |  |                 |
	        |              |              |  create the HID input  |  |                 |
	        |              |              |  report                |  |                 |
	        |              |              |------------------------|  |                 |
	        |              |Data received |                           |                 |
	        |              | in HID report|                           |                 |
    To	        |<-------------|<-------------|                           |                 |
    Applications|              |              |                           |                 |
        <-------|              |              |                           |                 |

```
