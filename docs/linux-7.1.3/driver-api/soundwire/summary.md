## SoundWire 子系统概

SoundWire MIPI 联盟2015 年批准的一种新接口。SoundWire 用于传输通常与音功能相关的数据。SoundWire 接口经过优化，适用于在移动或受移动设备启发的系统中
集成音频设备
SoundWire 是一种双引脚的多点（multi-drop）接口，包含数据线和时钟线。它有助开发低成本、高效、高性能的系统。SoundWire 接口的高层级关键特性包括：

 (1) 通过单一的双引脚接口传输所有有效载荷数据通道、控制信息和建立命令
 (2) 通过使用 DDR（双数据速率）数据传输，降低时钟频率，从而降低功耗
 (3) 时钟缩放和可选的多条数据通道，以极大的灵活性匹配系统需求的数据速率
 (4) 设备状态监控，包括Master 的中断式告警
SoundWire 协议最多支持十一Slave 接口。所有接口共享包含数据线与时钟线的公共总线每个 Slave 最多可支持 14 个数据端口。其13 个数据端口专用于音频传输数据端口 0 专用于传输批量控制信息，每个音频数据端口..14）在发送或接收模式最多可支持 8 个通道（通常为固定方向，但规范也允许可配置方向）。不过，19.2..24.576Mbits/s 的带宽限制不允许同时传输 11**13**8 个通道
下图展示了一SoundWire Master ```

        +---------------+                                       +---------------+
        |               |                       Clock Signal    |               |
        |    Master     |-------+-------------------------------|    Slave      |
        |   Interface   |       |               Data Signal     |  Interface 1  |
        |               |-------|-------+-----------------------|               |
        +---------------+       |       |                       +---------------+
                                |       |
                                |       |
                                |       |
                             +--+-------+--+
                             |             |
                             |   Slave     |
                             | Interface 2 |
                             |             |
                             +-------------+


```
## 术语


MIPI SoundWire 规范使用术语 'device' 来指Master Slave 接口，这当然容易引起混淆在本概述和代码中，我们仅使用术语 interface 来指代硬件。我们遵Linux 设备模型将总线上连接的每个 Slave 接口映射为由特定驱动管理device。Linux SoundWire 子系提供了一个框架来实现 SoundWire Slave 驱动，并提供一API，允许第三方厂商实现
自定义的、规范定义之外的功能，而通用的建配置任务由总线处理
Bus（总线）：
实现处理 SoundWire 协议SoundWire Linux 总线。对所有的 MIPI 定义Slave 寄存进行编程。代表一SoundWire Master。系统中可能存在总线的多个实例
Slave（从设备）：
注册SoundWire Slave 设备（Linux 设备）。多Slave 设备可以注册到一个总线实例
Slave driver（从设备驱动）：
控制 Slave 设备的驱动。MIPI 规定的寄存器由总线直接控制（并通过 Master 驱动/接口传输）任何规范定义之外Slave 寄存器都Slave 驱动控制。实践中，预Slave 驱动依赖
regmap，而不直接请求寄存器访问
## 编程接口（SoundWire 主接口驱动）


SoundWire 总线SoundWire Master 实现SoundWire Slave 设备提供编程接口。所有代都使SoC 设计人员和第三方厂商常用"sdw" 前缀
每个 SoundWire Master 接口都需要注册到总线上。总线实现了用于读取标Master MIPI
属性的 API，并Master ops 中提供回调，Master 驱动实现其自身提供能力信息的函数目前尚未实现 DT 支持，但由于能力是通过 `device_property_` API 启用的，添加起来应该
很简单
Master 接口及其能力基于 board 文件、DT ACPI 进行注册
以下是用于注SoundWire 总线的总线 API

	int sdw_bus_master_add(struct sdw_bus *bus,
				struct device *parent,
				struct fwnode_handle)
	{
		sdw_master_device_add(bus, parent, fwnode);

		mutex_init(&bus->lock);
		INIT_LIST_HEAD(&bus->slaves);

		/** Check ACPI for Slave devices **/
		sdw_acpi_find_slaves(bus);

		/** Check DT for Slave devices **/
		sdw_of_find_slaves(bus);

		return 0;
	}

这将Master 设备初始sdw_bus 对象。向总线提供 "sdw_master_ops" "sdw_master_port_ops" 回调函数
"sdw_master_ops" 由总线用于以硬件特定的方式控制总线。它包括总线控制函数，例在总线上发SoundWire 写消息，设置时钟频率和流同步点（SSP）sdw_master_ops"
结构体将 Master 的硬件细节从总线中抽象出来
"sdw_master_port_ops" 由总线用于设置 Master 接口端口的端口参数。Master 接口端口寄存器映射并未由 MIPI 规范定义，因此总线调用 "sdw_master_port_ops" 回调函数来执端口操作，例"Port Prepare"Port Transport params set"Port enable and disable"然后 Master 驱动的实现可以执行硬件特定的配置
## 编程接口（SoundWire 从设备驱动）


MIPI 规范要求每个 Slave 接口暴露一个唯一48 位标识符，存储在 6 个只dev_id
寄存器中。该 dev_id 标识符包含厂商和部件信息，以及一个用于区分相同组件的字段额外class 字段目前未使用。Slave 驱动针对特定的厂商和部件标识符编写，总线根据
这两id 枚举 Slave 设备。设备与驱动的匹配基于这两个 id 完成。当设备与驱id
成功匹配时，总线调用 Slave 驱动Probe。Master Slave 设备之间强制建立父子关系
（逻辑表示与物理连接保持一致）
Master/Slave 依赖关系的信息存储在平台数据、board 文件、ACPI DT 中。MIPI 软件规范
为拥有多Master 接口的控制器定义了额外的 link_id 参数。dev_id 寄存器仅link 范围内唯一，link_id 在控制器的范围内唯一。dev_id link_id 在系统级别上都不一定唯一但父子信息用于避免歧义

	static const struct sdw_device_id slave_id[] = {
	        SDW_SLAVE_ENTRY(0x025d, 0x700, 0),
	        {},
	};
	MODULE_DEVICE_TABLE(sdw, slave_id);

	static struct sdw_driver slave_sdw_driver = {
	        .driver = {
	                   .name = "slave_xxx",
	                   .pm = &slave_runtime_pm,
	                   },
		.probe = slave_sdw_probe,
		.remove = slave_sdw_remove,
		.ops = &slave_slave_ops,
		.id_table = slave_id,
	};


对于能力，总线实现了用于读取标Slave MIPI 属性的 API，并Slave ops 中提供回调，
Slave 驱动实现提供能力信息的自身函数。总线需要知道一Slave 能力，以便对 Slave
寄存器进行编程并控制总线的重新配置
## 链接


SoundWire MIPI 规范 1.1 可在以下地址获取https://members.mipi.org/wg/All-Members/document/70290

SoundWire MIPI DisCo（Discover and Configuration，发现与配置）规范可在以下地址获取https://www.mipi.org/specifications/mipi-disco-soundwire

（注册后可公开访问，MIPI 成员可直接访问）

MIPI 联盟厂商 ID 页面：mid.mipi.org
