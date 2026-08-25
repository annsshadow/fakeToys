
## Texas Instruments TPS6594 PFSM 驱动


Author: Julien Panis (jpanis@baylibre.com)

## 概述


严格来说，PFSM（预配置有限状态机，Pre-configurable Finite State Machine）并非硬件。它是一段代码
TPS6594 PMIC（电源管IC，Power Management IC）集成了一个管理运行模式的状态机。根据当前的运行模式，某些电压域保持上电，而其他域可以关闭
PFSM 驱动可用于触发已配置状态之间的转换。它还提供对设备寄存器的写访问
### 支持的设

- tps6594-q1
- tps6593-q1
- lp8764-q1

## 驱动位置


drivers/misc/tps6594-pfsm.c

## 驱动类型定义


include/uapi/linux/tps6594_pfsm.h

## 驱动 IOCTL


`PMIC_GOTO_STANDBY`
所有设备资源均断电。处理器关闭，没有任何电压域上电
`PMIC_GOTO_LP_STANDBY`
PMIC 中不需要常开的数字与模拟功能被关闭（低功耗）
`PMIC_UPDATE_PGM`
触发固件更新
`PMIC_SET_ACTIVE_STATE`
运行模式之一PMIC 完全正常工作，并向所PDN 负载供电MCU 与主处理器两个部分的电压域均上电
`PMIC_SET_MCU_ONLY_STATE`
运行模式之一仅有分配MCU Safety Island 的电源资源开启
`PMIC_SET_RETENTION_STATE`
运行模式之一根据所设置的触发器，部DDR/GPIO 电压域可保持上电，而所有其他域关闭，以最小化系统总功耗
## 驱动使用


```

    # ls /dev/pfsm*

```
```

    # hexdump -C /dev/pfsm-0-0x48

```
```

    # cat /proc/interrupts

```
### 用户空间代码示例


samples/pfsm/pfsm-wakeup.c
