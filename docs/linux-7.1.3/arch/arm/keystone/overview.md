## TI Keystone Linux 概述


### 简介


Keystone 系列 SoC 基于 ARM Cortex-A15 MPCore 处理器
和 c66x DSP 内核。本文档描述了用户在德州仪器（TI）的
Keystone 评估模块（EVM）上运行 Linux 所需的基本信息。

目前支持以下 SoC 和 EVM：-

## K2HK SoC 与 EVM


a.k.a Keystone 2 Hawking/Kepler SoC
TCI6636K2H 与 TCI6636K2K：文档参见

	http://www.ti.com/product/tci6638k2k
	http://www.ti.com/product/tci6638k2h

EVM：
  http://www.advantech.com/Support/TI-EVM/EVMK2HX_sd.aspx

## K2E SoC 与 EVM


a.k.a Keystone 2 Edison SoC

K2E  -  66AK2E05:

文档参见

	http://www.ti.com/product/66AK2E05/technicaldocuments

EVM：
   https://www.einfochips.com/index.php/partnerships/texas-instruments/k2e-evm.html

## K2L SoC 与 EVM


a.k.a Keystone 2 Lamarr SoC

K2L  -  TCI6630K2L:

文档参见
	http://www.ti.com/product/TCI6630K2L/technicaldocuments

EVM：
  https://www.einfochips.com/index.php/partnerships/texas-instruments/k2l-evm.html

### 配置


所有 K2 SoC/EVM 共享一个通用 defconfig（keystone_defconfig），且使用
相同的镜像在各 EVM 上启动。平台配置通过
DTS 指定。使用的 DTS 如下：

	K2HK EVM:
		k2hk-evm.dts
	K2E EVM:
		k2e-evm.dts
	K2L EVM:
		k2l-evm.dts

Keystone 设备的设备树文档位于

        Documentation/devicetree/bindings/arm/ti/ti,keystone.yaml

### 文档作者


Murali Karicheri <m-karicheri2@ti.com>

版权所有 2015 德州仪器
