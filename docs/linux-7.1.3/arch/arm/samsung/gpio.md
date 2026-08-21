## Samsung GPIO 实现


### 简


本文概述 Samsung GPIO 实现，以及随 drivers/gpio 核心一同提供的
架构相关调用

### GPIOLIB 集成


gpio 实现尽可能使gpiolib，仅为那些需Samsung 特定处理
项目（如引脚特殊功能或上拉电阻控制）提供特定调用

GPIO 编号Samsung gpiolib 系统之间保持同步

### 引脚配置


引脚配置特定Samsung 架构，每SoC 注册必要信息
供核gpio 配置实现按需配置引脚

s3c_gpio_cfgpin() s3c_gpio_setpull() 为驱动或机器提供
更改 gpio 配置的手段

更多信息请参arch/arm/mach-s3c/gpio-cfg.h
