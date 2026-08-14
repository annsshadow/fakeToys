## MIPI CCS 摄像头传感器驱动


MIPI CCS 摄像头传感器驱动是一个面向 `MIPI CCS <https://www.mipi.org/specifications/camera-command-set>`_
兼容摄像头传感器的通用驱动。

另请参阅 CCS 驱动 UAPI 文档 <media-ccs-uapi>。

### CCS 静态数据


MIPI CCS 驱动支持所有兼容设备的 CCS 静态数据，不仅包括兼容 CCS 1.1 的设备，也包括 CCS 1.0 和
SMIA(++)。对于 CCS，文件名构成为

	ccs/ccs-sensor-vvvv-mmmm-rrrr.fw（传感器）以及
	ccs/ccs-module-vvvv-mmmm-rrrr.fw（模块）。

对于兼容 SMIA++ 的设备，相应的文件名为

	ccs/smiapp-sensor-vv-mmmm-rr.fw（传感器）以及
	ccs/smiapp-module-vv-mmmm-rrrr.fw（模块）。

对于兼容 SMIA（非 ++）的设备，静态数据文件名为

	ccs/smia-sensor-vv-mmmm-rr.fw（传感器）。

vvvv 或 vv 分别表示 MIPI 和 SMIA 厂商 ID，mmmm 为型号 ID，rrrr 或 rr 为版本号。

#### CCS 工具


`CCS tools <https://github.com/MIPI-Alliance/ccs-tools/>`_ 是一组用于处理 CCS 静态数据文件的
工具。CCS tools 包含人类可读的 CCS 静态数据 YAML 格式的定义，并包含一个将其转换为二进制的
程序。

### 寄存器定义生成器


ccs-regs.asc 文件包含 MIPI CCS 寄存器定义，用于生成更便于 C 语言程序使用的 C 源代码定义文件。
由于生成的文件之间存在许多依赖关系，请不要手动修改它们，因为那容易出错且徒劳无功，而应修改
生成它们的脚本。

#### 用法


按照惯例，脚本以如下方式调用来更新 CCS 驱动定义：


	$ Documentation/driver-api/media/drivers/ccs/mk-ccs-regs -k \
		-e drivers/media/i2c/ccs/ccs-regs.h \
		-L drivers/media/i2c/ccs/ccs-limits.h \
		-l drivers/media/i2c/ccs/ccs-limits.c \
		-c Documentation/driver-api/media/drivers/ccs/ccs-regs.asc

## CCS PLL 计算器


CCS PLL 计算器用于在给定传感器能力、板配置以及用户指定配置的情况下计算 PLL 配置。由于涵盖所有
这些配置的配置空间非常庞大，PLL 计算器并非完全简单。但对于驱动而言它相对易于使用。

PLL 计算器实现的 PLL 模型对应于 MIPI CCS 1.1。


**Copyright** |copy| 2020 Intel Corporation
