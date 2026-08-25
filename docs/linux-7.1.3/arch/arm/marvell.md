## ARM Marvell SoC

本文档梳Linux 内核主线所支持ARM Marvell SoC（含 Orion、Kirkwood、Discovery 等系列），列出各型号、对应内mach/plat 目录及公开数据手册链接，帮助开发者确认具体芯片的支持情况


本文档列出了当前Linux 内核主线所支持的全ARM Marvell SoC。由Marvell SoC 系列规模庞大且复杂，很难弄清某个特定 SoC Linux 内核中的支持情况。本文档试图帮助理解这些 SoC 在何处得到支持，并在有公开数据手册时将其与对应的公开数据手册相匹配

### Orion 系列


  型号
        - 88F5082
        - 88F5181  a.k.a Orion-1
        - 88F5181L a.k.a Orion-VoIP
        - 88F5182  a.k.a Orion-NAS

               - 数据手册：https://web.archive.org/web/20210124231420/http://csclub.uwaterloo.ca/~board/ts7800/MV88F5182-datasheet.pdf
               - 编程者用户指南：https://web.archive.org/web/20210124231536/http://csclub.uwaterloo.ca/~board/ts7800/MV88F5182-opensource-manual.pdf
               - 用户手册：https://web.archive.org/web/20210124231631/http://csclub.uwaterloo.ca/~board/ts7800/MV88F5182-usermanual.pdf
               - 功能勘误：https://web.archive.org/web/20210704165540/https://www.digriz.org.uk/ts78xx/88F5182_Functional_Errata.pdf
        - 88F5281  a.k.a Orion-2

               - 数据手册：https://web.archive.org/web/20131028144728/http://www.ocmodshop.com/images/reviews/networking/qnap_ts409u/marvel_88f5281_data_sheet.pdf
        - 88F6183  a.k.a Orion-1-90
  主页
        https://web.archive.org/web/20080607215437/http://www.marvell.com/products/media/index.jsp
  内核
	Feroceon 88fr331 (88f51xx) or 88fr531-vd (88f52xx) ARMv5 compatible
  Linux 内核 mach 目录
	arch/arm/mach-orion5x
  Linux 内核 plat 目录
	arch/arm/plat-orion

### Kirkwood 系列


  型号
        - 88F6282 a.k.a Armada 300

                - 产品简 : https://web.archive.org/web/20111027032509/http://www.marvell.com/embedded-processors/armada-300/assets/armada_310.pdf
        - 88F6283 a.k.a Armada 310

                - 产品简 : https://web.archive.org/web/20111027032509/http://www.marvell.com/embedded-processors/armada-300/assets/armada_310.pdf
        - 88F6190

                - 产品简 : https://web.archive.org/web/20130730072715/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6190-003_WEB.pdf
                - 硬件规格  : https://web.archive.org/web/20121021182835/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F619x_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6192

                - 产品简 : https://web.archive.org/web/20131113121446/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6192-003_ver1.pdf
                - 硬件规格  : https://web.archive.org/web/20121021182835/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F619x_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6182
        - 88F6180

                - 产品简 : https://web.archive.org/web/20120616201621/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6180-003_ver1.pdf
                - 硬件规格  : https://web.archive.org/web/20130730091654/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F6180_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6280

                - 产品简 : https://web.archive.org/web/20130730091058/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6280_SoC_PB-001.pdf
        - 88F6281

                - 产品简 : https://web.archive.org/web/20120131133709/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6281-004_ver1.pdf
                - 硬件规格  : https://web.archive.org/web/20120620073511/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F6281_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6321
        - 88F6322
        - 88F6323

                - 产品简 : https://web.archive.org/web/20120616201639/http://www.marvell.com/embedded-processors/kirkwood/assets/88f632x_pb.pdf
  主页
	https://web.archive.org/web/20160513194943/http://www.marvell.com/embedded-processors/kirkwood/
  内核
	Feroceon 88fr131 ARMv5 compatible
  Linux 内核 mach 目录
	arch/arm/mach-mvebu
  Linux 内核 plat 目录
	none

### Discovery 系列


  型号
        - MV78100

                - 产品简 : https://web.archive.org/web/20120616194711/http://www.marvell.com/embedded-processors/discovery-innovation/assets/MV78100-003_WEB.pdf
                - 硬件规格  : https://web.archive.org/web/20141005120451/http://www.marvell.com/embedded-processors/discovery-innovation/assets/HW_MV78100_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20111110081125/http://www.marvell.com/embedded-processors/discovery-innovation/assets/FS_MV76100_78100_78200_OpenSource.pdf
        - MV78200

                - 产品简 : https://web.archive.org/web/20140801121623/http://www.marvell.com/embedded-processors/discovery-innovation/assets/MV78200-002_WEB.pdf
                - 硬件规格  : https://web.archive.org/web/20141005120458/http://www.marvell.com/embedded-processors/discovery-innovation/assets/HW_MV78200_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20111110081125/http://www.marvell.com/embedded-processors/discovery-innovation/assets/FS_MV76100_78100_78200_OpenSource.pdf

        - MV76100

                - 产品简 : https://web.archive.org/web/20140722064429/http://www.marvell.com/embedded-processors/discovery-innovation/assets/MV76100-002_WEB.pdf
                - 硬件规格  : https://web.archive.org/web/20140722064425/http://www.marvell.com/embedded-processors/discovery-innovation/assets/HW_MV76100_OpenSource.pdf
                - 功能规格: https://web.archive.org/web/20111110081125/http://www.marvell.com/embedded-processors/discovery-innovation/assets/FS_MV76100_78100_78200_OpenSource.pdf

                Linux 内核不支持

  主页
        https://web.archive.org/web/20110924171043/http://www.marvell.com/embedded-processors/discovery-innovation/
  内核
	Feroceon 88fr571-vd ARMv5 compatible

  Linux 内核 mach 目录
	arch/arm/mach-mv78xx0
  Linux 内核 plat 目录
	arch/arm/plat-orion

### EBU Armada 系列


  Armada 370 型号
        - 88F6710
        - 88F6707
        - 88F6W11

    - 产品信息  https://web.archive.org/web/20141002083258/http://www.marvell.com/embedded-processors/armada-370/
    - 产品简介：   https://web.archive.org/web/20121115063038/http://www.marvell.com/embedded-processors/armada-300/assets/Marvell_ARMADA_370_SoC.pdf
    - 硬件规格  https://web.archive.org/web/20140617183747/http://www.marvell.com/embedded-processors/armada-300/assets/ARMADA370-datasheet.pdf
    - 功能规格  https://web.archive.org/web/20140617183701/http://www.marvell.com/embedded-processors/armada-300/assets/ARMADA370-FunctionalSpec-datasheet.pdf

  内核
	Sheeva ARMv7 compatible PJ4B

  Armada XP 型号
        - MV78230
        - MV78260
        - MV78460

    注意
	不要与非 SMP 78xx0 SoC 混淆

    - 产品信息  https://web.archive.org/web/20150101215721/http://www.marvell.com/embedded-processors/armada-xp/
    - 产品简介：   https://web.archive.org/web/20121021173528/http://www.marvell.com/embedded-processors/armada-xp/assets/Marvell-ArmadaXP-SoC-product%20brief.pdf
    - 功能规格  https://web.archive.org/web/20180829171131/http://www.marvell.com/embedded-processors/armada-xp/assets/ARMADA-XP-Functional-SpecDatasheet.pdf
    - 硬件规格
        - https://web.archive.org/web/20141127013651/http://www.marvell.com/embedded-processors/armada-xp/assets/HW_MV78230_OS.PDF
        - https://web.archive.org/web/20141222000224/http://www.marvell.com/embedded-processors/armada-xp/assets/HW_MV78260_OS.PDF
        - https://web.archive.org/web/20141222000230/http://www.marvell.com/embedded-processors/armada-xp/assets/HW_MV78460_OS.PDF

  内核
	Sheeva ARMv7 compatible Dual-core or Quad-core PJ4B-MP

  Armada 375 型号
 - 88F6720

    - 产品信息https://web.archive.org/web/20140108032402/http://www.marvell.com/embedded-processors/armada-375/
    - 产品简介： https://web.archive.org/web/20131216023516/http://www.marvell.com/embedded-processors/armada-300/assets/ARMADA_375_SoC-01_product_brief.pdf

  内核
	ARM Cortex-A9

  Armada 38x 型号
 - 88F6810	Armada 380
 - 88F6811 Armada 381
 - 88F6821 Armada 382
 - 88F6W21 Armada 383
 - 88F6820 Armada 385
 - 88F6825
 - 88F6828 Armada 388

    - 产品信息  https://web.archive.org/web/20181006144616/http://www.marvell.com/embedded-processors/armada-38x/
    - 功能规格  https://web.archive.org/web/20200420191927/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-38x-functional-specifications-2015-11.pdf
    - 硬件规格  https://web.archive.org/web/20180713105318/https://www.marvell.com/docs/embedded-processors/assets/marvell-embedded-processors-armada-38x-hardware-specifications-2017-03.pdf
    - 设计指南  https://web.archive.org/web/20180712231737/https://www.marvell.com/docs/embedded-processors/assets/marvell-embedded-processors-armada-38x-hardware-design-guide-2017-08.pdf

  内核
	ARM Cortex-A9

  Armada 39x 型号
 - 88F6920 Armada 390
 - 88F6925 Armada 395
 - 88F6928 Armada 398

    - 产品信息https://web.archive.org/web/20181020222559/http://www.marvell.com/embedded-processors/armada-39x/

  内核
	ARM Cortex-A9

  Linux 内核 mach 目录
	arch/arm/mach-mvebu
  Linux 内核 plat 目录
	none

### EBU Armada 系列（ARMv8


  Armada 3710/3720 型号
 - 88F3710
 - 88F3720

  内核
	ARM Cortex A53 (ARMv8)

  主页
	https://web.archive.org/web/20181103003602/http://www.marvell.com/embedded-processors/armada-3700/

  产品简介：
	https://web.archive.org/web/20210121194810/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-37xx-product-brief-2016-01.pdf

  硬件规格
	https://web.archive.org/web/20210202162011/http://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-37xx-hardware-specifications-2019-09.pdf

  设备树文件：
	arch/arm64/boot/dts/marvell/armada-37*

  Armada 7K 型号
   - 88F6040 (AP806 Quad 600 MHz + one CP110)
   - 88F7020 (AP806 Dual + one CP110)
   - 88F7040 (AP806 Quad + one CP110)

  内核ARM Cortex A72

  主页
	https://web.archive.org/web/20181020222606/http://www.marvell.com/embedded-processors/armada-70xx/

  产品简介：
   - https://web.archive.org/web/20161010105541/http://www.marvell.com/embedded-processors/assets/Armada7020PB-Jan2016.pdf
   - https://web.archive.org/web/20160928154533/http://www.marvell.com/embedded-processors/assets/Armada7040PB-Jan2016.pdf

  设备树文件：
	arch/arm64/boot/dts/marvell/armada-70*

  Armada 8K 型号
 - 88F8020 (AP806 Dual + two CP110)
 - 88F8040 (AP806 Quad + two CP110)
  内核
	ARM Cortex A72

  主页
	https://web.archive.org/web/20181022004830/http://www.marvell.com/embedded-processors/armada-80xx/

  产品简介：
   - https://web.archive.org/web/20210124233728/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-8020-product-brief-2017-12.pdf
   - https://web.archive.org/web/20161010105532/http://www.marvell.com/embedded-processors/assets/Armada8040PB-Jan2016.pdf

  设备树文件：
	arch/arm64/boot/dts/marvell/armada-80*

  Octeon TX2 CN913x 型号
 - CN9130 (AP807 Quad + one internal CP115)
 - CN9131 (AP807 Quad + one internal CP115 + one external CP115 / 88F8215)
 - CN9132 (AP807 Quad + one internal CP115 + two external CP115 / 88F8215)

  内核
	ARM Cortex A72

  主页
	https://web.archive.org/web/20200803150818/https://www.marvell.com/products/infrastructure-processors/multi-core-processors/octeon-tx2/octeon-tx2-cn9130.html

  产品简介：
	https://web.archive.org/web/20200803150818/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-infrastructure-processors-octeon-tx2-cn913x-product-brief-2020-02.pdf

  设备树文件：
	arch/arm64/boot/dts/marvell/cn913*

### Avanta 系列


  型号
       - 88F6500
       - 88F6510
       - 88F6530P
       - 88F6550
       - 88F6560
       - 88F6601

  主页
	https://web.archive.org/web/20181005145041/http://www.marvell.com/broadband/

  产品简介：
	https://web.archive.org/web/20180829171057/http://www.marvell.com/broadband/assets/Marvell_Avanta_88F6510_305_060-001_product_brief.pdf

  无公开数据手册可用

  内核
	ARMv5 compatible

  Linux 内核 mach 目录
	主线中尚无代码，计划在未来支
  Linux 内核 plat 目录
	主线中尚无代码，计划在未来支

### Storage 系列


  Armada SP锛。
 - 88RC1580

  产品信息
	https://web.archive.org/web/20191129073953/http://www.marvell.com/storage/armada-sp/

  内核
	Sheeva ARMv7 compatible Quad-core PJ4C

  （上Linux 内核不支持）

### Dove 系列（应用处理器


  型号
        - 88AP510 a.k.a Armada 510

   产品简介：
	https://web.archive.org/web/20111102020643/http://www.marvell.com/application-processors/armada-500/assets/Marvell_Armada510_SoC.pdf

   硬件规格
	https://web.archive.org/web/20160428160231/http://www.marvell.com/application-processors/armada-500/assets/Armada-510-Hardware-Spec.pdf

  功能规格
	https://web.archive.org/web/20120130172443/http://www.marvell.com/application-processors/armada-500/assets/Armada-510-Functional-Spec.pdf

  主页
	https://web.archive.org/web/20160822232651/http://www.marvell.com/application-processors/armada-500/

  内核
	ARMv7 compatible

  目录
 - arch/arm/mach-mvebu (DT 启用平台)
        - arch/arm/mach-dove (DT 启用平台)

### PXA 2xx/3xx/93x/95x 系列


  型号
        - PXA21x, PXA25x, PXA26x
             - 仅应用处理器
             - 内核：ARMv5 XScale1 core
        - PXA270, PXA271, PXA272
             - 产品简        : https://web.archive.org/web/20150927135510/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_pb.pdf
             - 设计指南          : https://web.archive.org/web/20120111181937/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_design_guide.pdf
             - 开发者手    : https://web.archive.org/web/20150927164805/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_dev_man.pdf
             - 规格         : https://web.archive.org/web/20140211221535/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_emts.pdf
             - 规格更新  : https://web.archive.org/web/20120111104906/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_spec_update.pdf
             - 仅应用处理器
             - 内核：ARMv5 XScale2 core
        - PXA300, PXA310, PXA320
             - PXA 300 产品简: https://web.archive.org/web/20120111121203/http://www.marvell.com/application-processors/pxa-family/assets/PXA300_PB_R4.pdf
             - PXA 310 产品简: https://web.archive.org/web/20120111104515/http://www.marvell.com/application-processors/pxa-family/assets/PXA310_PB_R4.pdf
             - PXA 320 产品简: https://web.archive.org/web/20121021182826/http://www.marvell.com/application-processors/pxa-family/assets/PXA320_PB_R4.pdf
             - 设计指南          : https://web.archive.org/web/20130727144625/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_Design_Guide.pdf
             - 开发者手    : https://web.archive.org/web/20130727144605/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_Developers_Manual.zip
             - 规格        : https://web.archive.org/web/20130727144559/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_EMTS.pdf
             - 规格更新  : https://web.archive.org/web/20150927183411/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_Spec_Update.zip
             - 参考手     : https://web.archive.org/web/20120111103844/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_TavorP_BootROM_Ref_Manual.pdf
             - 仅应用处理器
             - 内核：ARMv5 XScale3 core
        - PXA930, PXA935
             - 带通信处理器的应用处理
             - 内核：ARMv5 XScale3 core
        - PXA955
             - 带通信处理器的应用处理
             - 内核：ARMv7 compatible Sheeva PJ4 core

   说明

    - 这一系列 SoC 源自 Intel 开发的 XScale 系列，于 2006 年左右被 Marvell 收购。PXA21x、PXA25x、PXA26x、PXA27x、PXA3xx PXA93x Intel 开发，而后来的 PXA95x Marvell 开发

    - 由于XScale 渊源，这SoC Marvell 的其他（Kirkwood、Dove 等）SoC 系列几乎毫无共同点，仅与 MMP/MMP2 系列 SoC 例外

   Linux 内核 mach 目录
	arch/arm/mach-pxa

### MMP/MMP2/MMP3 系列（通信处理器）


  型号
        - PXA168, a.k.a Armada 168
             - 主页             : https://web.archive.org/web/20110926014256/http://www.marvell.com/application-processors/armada-100/armada-168.jsp
             - 产品简       : https://web.archive.org/web/20111102030100/http://www.marvell.com/application-processors/armada-100/assets/pxa_168_pb.pdf
             - 硬件手册      : https://web.archive.org/web/20160428165359/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_datasheet.pdf
             - 软件手册      : https://web.archive.org/web/20160428154454/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_software_manual.pdf
             - 规格更新 : https://web.archive.org/web/20150927160338/http://www.marvell.com/application-processors/armada-100/assets/ARMADA16x_Spec_update.pdf
             - Boot ROM 手册      : https://web.archive.org/web/20130727205559/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_ref_manual.pdf
             - 应用节点    : https://web.archive.org/web/20141005090706/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_app_note_package.pdf
             - 仅应用处理器
             - 内核：ARMv5 compatible Marvell PJ1 88sv331 (Mohawk)
        - PXA910/PXA920
             - 主页             : https://web.archive.org/web/20150928121236/http://www.marvell.com/communication-processors/pxa910/
             - 产品简       : https://archive.org/download/marvell-pxa910-pb/Marvell_PXA910_Platform-001_PB.pdf
             - 带通信处理器的应用处理
             - 内核：ARMv5 compatible Marvell PJ1 88sv331 (Mohawk)
        - PXA688, a.k.a. MMP2, a.k.a Armada 610 (OLPC XO-1.75)
             - 产品简       : https://web.archive.org/web/20111102023255/http://www.marvell.com/application-processors/armada-600/assets/armada610_pb.pdf
             - 仅应用处理器
             - 内核：ARMv7 compatible Sheeva PJ4 88sv581x core
 - PXA2128, a.k.a. MMP3, a.k.a Armada 620 (OLPC XO-4)
      - 产品简    : https://web.archive.org/web/20120824055155/http://www.marvell.com/application-processors/armada/pxa2128/assets/Marvell-ARMADA-PXA2128-SoC-PB.pdf
      - 仅应用处理器
      - 内核：Dual-core ARMv7 compatible Sheeva PJ4C core
 - PXA960/PXA968/PXA978 (Linux 支持不在上游)
      - 带通信处理器的应用处理
      - 内核：ARMv7 compatible Sheeva PJ4 core
 - PXA986/PXA988 (Linux 支持不在上游)
      - 带通信处理器的应用处理
      - 内核：Dual-core ARMv7 compatible Sheeva PJ4B-MP core
 - PXA1088/PXA1920 (Linux 支持不在上游)
      - 带通信处理器的应用处理
      - 内核：quad-core ARMv7 Cortex-A7
 - PXA1908/PXA1928/PXA1936
      - 带通信处理器的应用处理
      - 内核：multi-core ARMv8 Cortex-A53

   说明

    - 这一系列 SoC 源自 Intel 开发的 XScale 系列，于 2006 年左右被 Marvell 收购。MMP/MMP2 系列的所有处理器均由 Marvell 开发

    - 由于XScale 渊源，这SoC Marvell 的其他（Kirkwood、Dove 等）SoC 系列几乎毫无共同点，仅与上文所列的 PXA 系列 SoC 例外

   Linux 内核 mach 目录
	arch/arm/mach-mmp

### Berlin 系列（多媒体解决方案


  - 型号
 - 88DE3010, Armada 1000 (Linux 支持)
  - 内核	Marvell PJ1 (ARMv5TE), Dual-core
  - 产品简介：	https://web.archive.org/web/20131103162620/http://www.marvell.com/digital-entertainment/assets/armada_1000_pb.pdf
 - 88DE3005, Armada 1500 Mini
  - 设计名称BG2CD
  - 内核	ARM Cortex-A9, PL310 L2CC
 - 88DE3006, Armada 1500 Mini Plus
  - 设计名称BG2CDP
  - 内核	Dual Core ARM Cortex-A7
 - 88DE3100, Armada 1500
  - 设计名称BG2
  - 内核	Marvell PJ4B-MP (ARMv7), Tauros3 L2CC
 - 88DE3114, Armada 1500 Pro
  - 设计名称BG2Q
  - 内核	Quad Core ARM Cortex-A9, PL310 L2CC
 - 88DE3214, Armada 1500 Pro 4K
  - 设计名称BG3
  - 内核	ARM Cortex-A15, CA15 integrated L2CC
 - 88DE3218, ARMADA 1500 Ultra
  - 内核	ARM Cortex-A53

  主页：https://www.synaptics.com/products/multimedia-solutions
  目录：arch/arm/mach-berlin

  说明

   - 这一系列 SoC 基于 Marvell Sheeva ARM Cortex CPU，并采用 Synopsys DesignWare（IRQ、GPIO、Timers 等）PXA IP（SDHCI、USB、ETH 等）

   - Berlin 系列2017 年由 Synaptics Marvell 收购

### CPU 内核


XScale 内核Intel 设计，并在较老的 PXA 处理器中Marvell 出货。Feroceon Marvell 内部开发的专有内核，后来演进为 Sheeva。XScale Feroceon 内核随时间被逐步淘汰，在后续产品中被 Sheeva 内核取代，随后又被获得授权的 ARM Cortex-A 内核取代

  XScale 1
	CPUID 0x69052xxx
	ARMv5, iWMMXt
  XScale 2
	CPUID 0x69054xxx
	ARMv5, iWMMXt
  XScale 3
	CPUID 0x69056xxx or 0x69056xxx
	ARMv5, iWMMXt
  Feroceon-1850 88fr331 "Mohawk"
	CPUID 0x5615331x or 0x41xx926x
	ARMv5TE, single issue
  Feroceon-2850 88fr531-vd "Jolteon"
	CPUID 0x5605531x or 0x41xx926x
	ARMv5TE, VFP, dual-issue
  Feroceon 88fr571-vd "Jolteon"
	CPUID 0x5615571x
	ARMv5TE, VFP, dual-issue
  Feroceon 88fr131 "Mohawk-D"
	CPUID 0x5625131x
	ARMv5TE, single-issue in-order
  Sheeva PJ1 88sv331 "Mohawk"
	CPUID 0x561584xx
	ARMv5, single-issue iWMMXt v2
  Sheeva PJ4 88sv581x "Flareon"
	CPUID 0x560f581x
	ARMv7, idivt, optional iWMMXt v2
  Sheeva PJ4B 88sv581x
	CPUID 0x561f581x
	ARMv7, idivt, optional iWMMXt v2
  Sheeva PJ4B-MP / PJ4C
	CPUID 0x562f584x
	ARMv7, idivt/idiva, LPAE, optional iWMMXt v2 and/or NEON

### 长期计划


 - mach-dove/、mach-mv78xx0/、mach-orion5x/ 统一mach-mvebu/ 中，以在单一mach-<foo> 目录下支持来Marvell EBU（Engineering Business Unit，工程业务单元）的全SoC。因plat-orion/ 将不复存在

### 致谢


- Maen Suleiman <maen@marvell.com>
- Lior Amsalem <alior@marvell.com>
- Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
- Andrew Lunn <andrew@lunn.ch>
- Nicolas Pitre <nico@fluxnic.net>
- Eric Miao <eric.y.miao@gmail.com>
