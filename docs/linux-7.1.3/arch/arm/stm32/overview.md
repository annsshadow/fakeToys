## STM32 ARM Linux 概述


### 简


意法半导体（STMicroelectronics）的 STM32 系列 Cortex-A 微处理器（MPU）和 Cortex-M 微控制器（MCU）由 ARM Linux 'STM32' 平台提供支持

### 配置


对于 MCU，使用提供的默认配置
        make stm32_defconfig
对于 MPU，使multi_v7 配置
        make multi_v7_defconfig

### 布局


多个机器系列的所有文件都位于 arch/arm/mach-stm32 内的平台代码中

mach 文件夹中有一个通用board-dt.c，支持扁平设备树（Flattened Device Tree），这意味着它可以与任何兼容的设备树板卡配合工作

:Authors:

- Maxime Coquelin <mcoquelin.stm32@gmail.com>
- Ludovic Barre <ludovic.barre@st.com>
- Gerald Baeza <gerald.baeza@st.com>
