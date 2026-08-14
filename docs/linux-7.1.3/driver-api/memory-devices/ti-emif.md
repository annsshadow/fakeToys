## TI EMIF SDRAM 控制器驱动


## 作者

Aneesh V <aneesh@ti.com>

## 位置

driver/memory/emif.c

## 支持的 SoC：

TI OMAP44xx
TI OMAP54xx

## Menuconfig 选项：

Device Drivers
	Memory devices
		Texas Instruments EMIF driver

## 描述

该驱动用于 Texas Instruments SoC 中可用的 EMIF 模块。EMIF 是一个 SDRAM
控制器，根据其版本支持 DDR2、DDR3 和 LPDDR2 SDRAM 协议中的一个或多个。目前
该驱动仅处理 LPDDR2 存储器。驱动的功能包括在频率、电压和温度变化期间重新
配置 AC 时序参数及其他设置。

## 平台数据（见 include/linux/platform_data/emif_plat.h）

DDR 设备细节以及其他依赖板和依赖 SoC 的信息可以通过平台数据
（struct emif_platform_data）传递。

- DDR 设备细节：'struct ddr_device_info'
- 设备 AC 时序：'struct lpddr2_timings' 和 'struct lpddr2_min_tck'
- 自定义配置：通过 'struct emif_custom_configs' 的可定制策略选项
- IP 版本
- PHY 类型

## 与外部世界的接口

EMIF 驱动为影响 EMIF 的电压和频率变化注册通知器，并在它们被调用时采取适当的
操作。

- freq_pre_notify_handling()
- freq_post_notify_handling()
- volt_notify_handling()

## Debugfs

该驱动为每个设备创建两个 debugfs 条目。

- regcache_dump：到目前为止所有使用过的频率计算并保存的寄存器值转储。
- mr4：LPDDR2 设备中 MR4 寄存器的最后轮询值。MR4 指示设备当前的温度等级。
