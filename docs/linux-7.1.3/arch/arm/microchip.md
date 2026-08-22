## ARM Microchip SoC（即 AT91

### 简
本文档提供了关于目前 Linux 主线（也就是 kernel.org 上的那个）所支持ARM
Microchip SoC 的有用信息
需要注意的是，Microchip（原 Atmel）基ARM MPU 产品线在整个 Linux 内核开过程中历史上一直被称为 "AT91" "at91"，即使这个产品前缀已经Microchip 官方产品名称中完全消失。无论如何，文件、目录、git 树、git 分支/标签以及邮件主题
始终包含这个 "at91" 子串

### AT91 SoC

每款产品的文档与详细数据手册均可Microchip 网站获取http://www.microchip.com
  类别    - 基于 ARM 920 SoC
      - at91rm9200

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-1768-32-bit-ARM920T-Embedded-Microprocessor-AT91RM9200_Datasheet.pdf

    - 基于 ARM 926 SoC
      - at91sam9260

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6221-32-bit-ARM926EJ-S-Embedded-Microprocessor-SAM9260_Datasheet.pdf

      - at91sam9xe

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6254-32-bit-ARM926EJ-S-Embedded-Microprocessor-SAM9XE_Datasheet.pdf

      - at91sam9261

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6062-ARM926EJ-S-Microprocessor-SAM9261_Datasheet.pdf

      - at91sam9263

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6249-32-bit-ARM926EJ-S-Embedded-Microprocessor-SAM9263_Datasheet.pdf

      - at91sam9rl

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/doc6289.pdf

      - at91sam9g20

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/DS60001516A.pdf

      - at91sam9g45 family
        - at91sam9g45
        - at91sam9g46
        - at91sam9m10
        - at91sam9m11 (device superset)

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-6437-32-bit-ARM926-Embedded-Microprocessor-SAM9M11_Datasheet.pdf

      - at91sam9x5 family (aka "The 5 series")
        - at91sam9g15
        - at91sam9g25
        - at91sam9g35
        - at91sam9x25
        - at91sam9x35

          - 数据手册（可视为覆盖整个系列
          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11055-32-bit-ARM926EJ-S-Microcontroller-SAM9X35_Datasheet.pdf

      - at91sam9n12

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/DS60001517A.pdf

      - sam9x60

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/SAM9X60-Data-Sheet-DS60001579A.pdf

    - 基于 ARM Cortex-A5 SoC
      - sama5d3 family

        - sama5d31
        - sama5d33
        - sama5d34
        - sama5d35
        - sama5d36 (device superset)

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-11121-32-bit-Cortex-A5-Microcontroller-SAMA5D3_Datasheet_B.pdf

    - 基于 ARM Cortex-A5 + NEON SoC
      - sama5d4 family

        - sama5d41
        - sama5d42
        - sama5d43
        - sama5d44 (device superset)

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/60001525A.pdf

      - sama5d2 family

        - sama5d21
        - sama5d22
        - sama5d23
        - sama5d24
        - sama5d26
        - sama5d27 (device superset)
        - sama5d28 (device superset + environmental monitors)

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/DS60001476B.pdf

    - 基于 ARM Cortex-A7 SoC
      - sama7g5 family

        - sama7g51
        - sama7g52
        - sama7g53
        - sama7g54 (device superset)

          - 数据手册

          即将推出

      - lan966 family
        - lan9662
        - lan9668

          - 数据手册

          即将推出

    - ARM Cortex-M7 MCUs
      - sams70 family

        - sams70j19
        - sams70j20
        - sams70j21
        - sams70n19
        - sams70n20
        - sams70n21
        - sams70q19
        - sams70q20
        - sams70q21

      - samv70 family

        - samv70j19
        - samv70j20
        - samv70n19
        - samv70n20
        - samv70q19
        - samv70q20

      - samv71 family

        - samv71j19
        - samv71j20
        - samv71j21
        - samv71n19
        - samv71n20
        - samv71n21
        - samv71q19
        - samv71q20
        - samv71q21

          - 数据手册

          http://ww1.microchip.com/downloads/en/DeviceDoc/SAM-E70-S70-V70-V71-Family-Data-Sheet-DS60001527D.pdf


### Linux 内核相关信息

Linux 内核mach 目录为：arch/arm/mach-at91
MAINTAINERS 条目为："ARM/Microchip (AT91) SoC support"


### AT91 SoC 与开发板的设备树

所AT91 SoC 均已转换为使用设备树。自 Linux 3.19 起，这些产品必须使用此方法来
引导 Linux 内核
进行中声明：
适用AT91 SoC 与开发板的设备树文件以及设备树绑定被视为"不稳。说得再清楚些，
任何 at91 绑定都可能在任何时候发生变更。因此，请务必使用由同一源码树生成的设备二进制文件和内核映像有关"稳定"绑定/ABI 的定义，请参Documentation/devicetree/bindings/ABI.rst 文件当情况合适时，该声明将由 AT91 的维护者移除
命名约定与最佳实践：

- SoC 的设备树源包含文件（.dtsi）以产品的官方名称命名（例如 at91sam9g20.dtsi
  sama5d33.dtsi）- 设备树源包含文件dtsi）用于收集可在多SoC 或开发板之间共享的通用节点
  （例sama5d3.dtsi at91sam9x5cm.dtsi）。当为某个特定外设或主题收集节点时，
  标识符必须放在文件名的末尾，并以 "_" 分隔（例at91sam9x5_can.dtsi   sama5d3_gmac.dtsi）- 开发板设备树源文件dts）以字符"at91-" 作为前缀，以便于识别。请注意，部  文件属于此规则的历史例外（例sama5d3[^13456^]ek.dts、usb_a9g20.dts   animeo_ip.dts）