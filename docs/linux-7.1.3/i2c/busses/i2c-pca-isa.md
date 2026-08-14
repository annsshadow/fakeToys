## 内核驱动 i2c-pca-isa


支持的适配器：

该驱动支持使用 Philips PCA 9564 并行总线到 I2C 总线控制器的 ISA 板卡

作者：Ian Campbell <icampbell@arcom.com>、Arcom Control Systems

### 模块参数


- base int
    I/O 基地址
- irq int
    IRQ 中断
- clock int
    时钟频率，如 PCA9564 数据手册表 1 所述

### 描述


该驱动支持使用 Philips PCA 9564 并行总线到 I2C 总线控制器的 ISA 板卡
