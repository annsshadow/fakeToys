## ARM Marvell SoC

鏈枃妗ｆ⒊鐞?Linux 鍐呮牳涓荤嚎鎵€鏀寔鐨?ARM Marvell SoC锛堝惈 Orion銆並irkwood銆丏iscovery 绛夌郴鍒楋級锛屽垪鍑哄悇鍨嬪彿銆佸搴斿唴鏍?mach/plat 鐩綍鍙婂叕寮€鏁版嵁鎵嬪唽閾炬帴锛屽府鍔╁紑鍙戣€呯‘璁ゅ叿浣撹姱鐗囩殑鏀寔鎯呭喌銆?


鏈枃妗ｅ垪鍑轰簡褰撳墠鐢?Linux 鍐呮牳涓荤嚎鎵€鏀寔鐨勫叏閮?ARM Marvell SoC銆傜敱浜?Marvell 鍚?SoC 绯诲垪瑙勬ā搴炲ぇ涓斿鏉傦紝寰堥毦寮勬竻鏌愪釜鐗瑰畾 SoC 鍦?Linux 鍐呮牳涓殑鏀寔鎯呭喌銆傛湰鏂囨。璇曞浘甯姪鐞嗚В杩欎簺 SoC 鍦ㄤ綍澶勫緱鍒版敮鎸侊紝骞跺湪鏈夊叕寮€鏁版嵁鎵嬪唽鏃跺皢鍏朵笌瀵瑰簲鐨勫叕寮€鏁版嵁鎵嬪唽鐩稿尮閰嶃€?

### Orion 绯诲垪


  鍨嬪彿锛?
        - 88F5082
        - 88F5181  a.k.a Orion-1
        - 88F5181L a.k.a Orion-VoIP
        - 88F5182  a.k.a Orion-NAS

               - 鏁版嵁鎵嬪唽锛歨ttps://web.archive.org/web/20210124231420/http://csclub.uwaterloo.ca/~board/ts7800/MV88F5182-datasheet.pdf
               - 缂栫▼鑰呯敤鎴锋寚鍗楋細https://web.archive.org/web/20210124231536/http://csclub.uwaterloo.ca/~board/ts7800/MV88F5182-opensource-manual.pdf
               - 鐢ㄦ埛鎵嬪唽锛歨ttps://web.archive.org/web/20210124231631/http://csclub.uwaterloo.ca/~board/ts7800/MV88F5182-usermanual.pdf
               - 鍔熻兘鍕樿锛歨ttps://web.archive.org/web/20210704165540/https://www.digriz.org.uk/ts78xx/88F5182_Functional_Errata.pdf
        - 88F5281  a.k.a Orion-2

               - 鏁版嵁鎵嬪唽锛歨ttps://web.archive.org/web/20131028144728/http://www.ocmodshop.com/images/reviews/networking/qnap_ts409u/marvel_88f5281_data_sheet.pdf
        - 88F6183  a.k.a Orion-1-90
  涓婚〉锛?
        https://web.archive.org/web/20080607215437/http://www.marvell.com/products/media/index.jsp
  鍐呮牳锛?
	Feroceon 88fr331 (88f51xx) or 88fr531-vd (88f52xx) ARMv5 compatible
  Linux 鍐呮牳 mach 鐩綍锛?
	arch/arm/mach-orion5x
  Linux 鍐呮牳 plat 鐩綍锛?
	arch/arm/plat-orion

### Kirkwood 绯诲垪


  鍨嬪彿锛?
        - 88F6282 a.k.a Armada 300

                - 浜у搧绠€浠? : https://web.archive.org/web/20111027032509/http://www.marvell.com/embedded-processors/armada-300/assets/armada_310.pdf
        - 88F6283 a.k.a Armada 310

                - 浜у搧绠€浠? : https://web.archive.org/web/20111027032509/http://www.marvell.com/embedded-processors/armada-300/assets/armada_310.pdf
        - 88F6190

                - 浜у搧绠€浠? : https://web.archive.org/web/20130730072715/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6190-003_WEB.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20121021182835/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F619x_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6192

                - 浜у搧绠€浠? : https://web.archive.org/web/20131113121446/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6192-003_ver1.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20121021182835/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F619x_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6182
        - 88F6180

                - 浜у搧绠€浠? : https://web.archive.org/web/20120616201621/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6180-003_ver1.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20130730091654/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F6180_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6280

                - 浜у搧绠€浠? : https://web.archive.org/web/20130730091058/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6280_SoC_PB-001.pdf
        - 88F6281

                - 浜у搧绠€浠? : https://web.archive.org/web/20120131133709/http://www.marvell.com/embedded-processors/kirkwood/assets/88F6281-004_ver1.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20120620073511/http://www.marvell.com/embedded-processors/kirkwood/assets/HW_88F6281_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20130730091033/http://www.marvell.com/embedded-processors/kirkwood/assets/FS_88F6180_9x_6281_OpenSource.pdf
        - 88F6321
        - 88F6322
        - 88F6323

                - 浜у搧绠€浠? : https://web.archive.org/web/20120616201639/http://www.marvell.com/embedded-processors/kirkwood/assets/88f632x_pb.pdf
  涓婚〉锛?
	https://web.archive.org/web/20160513194943/http://www.marvell.com/embedded-processors/kirkwood/
  鍐呮牳锛?
	Feroceon 88fr131 ARMv5 compatible
  Linux 鍐呮牳 mach 鐩綍锛?
	arch/arm/mach-mvebu
  Linux 鍐呮牳 plat 鐩綍锛?
	none

### Discovery 绯诲垪


  鍨嬪彿锛?
        - MV78100

                - 浜у搧绠€浠? : https://web.archive.org/web/20120616194711/http://www.marvell.com/embedded-processors/discovery-innovation/assets/MV78100-003_WEB.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20141005120451/http://www.marvell.com/embedded-processors/discovery-innovation/assets/HW_MV78100_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20111110081125/http://www.marvell.com/embedded-processors/discovery-innovation/assets/FS_MV76100_78100_78200_OpenSource.pdf
        - MV78200

                - 浜у搧绠€浠? : https://web.archive.org/web/20140801121623/http://www.marvell.com/embedded-processors/discovery-innovation/assets/MV78200-002_WEB.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20141005120458/http://www.marvell.com/embedded-processors/discovery-innovation/assets/HW_MV78200_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20111110081125/http://www.marvell.com/embedded-processors/discovery-innovation/assets/FS_MV76100_78100_78200_OpenSource.pdf

        - MV76100

                - 浜у搧绠€浠? : https://web.archive.org/web/20140722064429/http://www.marvell.com/embedded-processors/discovery-innovation/assets/MV76100-002_WEB.pdf
                - 纭欢瑙勬牸  : https://web.archive.org/web/20140722064425/http://www.marvell.com/embedded-processors/discovery-innovation/assets/HW_MV76100_OpenSource.pdf
                - 鍔熻兘瑙勬牸: https://web.archive.org/web/20111110081125/http://www.marvell.com/embedded-processors/discovery-innovation/assets/FS_MV76100_78100_78200_OpenSource.pdf

                Linux 鍐呮牳涓嶆敮鎸併€?

  涓婚〉锛?
        https://web.archive.org/web/20110924171043/http://www.marvell.com/embedded-processors/discovery-innovation/
  鍐呮牳锛?
	Feroceon 88fr571-vd ARMv5 compatible

  Linux 鍐呮牳 mach 鐩綍锛?
	arch/arm/mach-mv78xx0
  Linux 鍐呮牳 plat 鐩綍锛?
	arch/arm/plat-orion

### EBU Armada 绯诲垪


  Armada 370 鍨嬪彿锛?
        - 88F6710
        - 88F6707
        - 88F6W11

    - 浜у搧淇℃伅锛?  https://web.archive.org/web/20141002083258/http://www.marvell.com/embedded-processors/armada-370/
    - 浜у搧绠€浠嬶細   https://web.archive.org/web/20121115063038/http://www.marvell.com/embedded-processors/armada-300/assets/Marvell_ARMADA_370_SoC.pdf
    - 纭欢瑙勬牸锛?  https://web.archive.org/web/20140617183747/http://www.marvell.com/embedded-processors/armada-300/assets/ARMADA370-datasheet.pdf
    - 鍔熻兘瑙勬牸锛?  https://web.archive.org/web/20140617183701/http://www.marvell.com/embedded-processors/armada-300/assets/ARMADA370-FunctionalSpec-datasheet.pdf

  鍐呮牳锛?
	Sheeva ARMv7 compatible PJ4B

  Armada XP 鍨嬪彿锛?
        - MV78230
        - MV78260
        - MV78460

    娉ㄦ剰锛?
	涓嶈涓庨潪 SMP 鐨?78xx0 SoC 娣锋穯

    - 浜у搧淇℃伅锛?  https://web.archive.org/web/20150101215721/http://www.marvell.com/embedded-processors/armada-xp/
    - 浜у搧绠€浠嬶細   https://web.archive.org/web/20121021173528/http://www.marvell.com/embedded-processors/armada-xp/assets/Marvell-ArmadaXP-SoC-product%20brief.pdf
    - 鍔熻兘瑙勬牸锛?  https://web.archive.org/web/20180829171131/http://www.marvell.com/embedded-processors/armada-xp/assets/ARMADA-XP-Functional-SpecDatasheet.pdf
    - 纭欢瑙勬牸锛?
        - https://web.archive.org/web/20141127013651/http://www.marvell.com/embedded-processors/armada-xp/assets/HW_MV78230_OS.PDF
        - https://web.archive.org/web/20141222000224/http://www.marvell.com/embedded-processors/armada-xp/assets/HW_MV78260_OS.PDF
        - https://web.archive.org/web/20141222000230/http://www.marvell.com/embedded-processors/armada-xp/assets/HW_MV78460_OS.PDF

  鍐呮牳锛?
	Sheeva ARMv7 compatible Dual-core or Quad-core PJ4B-MP

  Armada 375 鍨嬪彿锛?
 - 88F6720

    - 浜у搧淇℃伅锛?https://web.archive.org/web/20140108032402/http://www.marvell.com/embedded-processors/armada-375/
    - 浜у搧绠€浠嬶細 https://web.archive.org/web/20131216023516/http://www.marvell.com/embedded-processors/armada-300/assets/ARMADA_375_SoC-01_product_brief.pdf

  鍐呮牳锛?
	ARM Cortex-A9

  Armada 38x 鍨嬪彿锛?
 - 88F6810	Armada 380
 - 88F6811 Armada 381
 - 88F6821 Armada 382
 - 88F6W21 Armada 383
 - 88F6820 Armada 385
 - 88F6825
 - 88F6828 Armada 388

    - 浜у搧淇℃伅锛?  https://web.archive.org/web/20181006144616/http://www.marvell.com/embedded-processors/armada-38x/
    - 鍔熻兘瑙勬牸锛?  https://web.archive.org/web/20200420191927/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-38x-functional-specifications-2015-11.pdf
    - 纭欢瑙勬牸锛?  https://web.archive.org/web/20180713105318/https://www.marvell.com/docs/embedded-processors/assets/marvell-embedded-processors-armada-38x-hardware-specifications-2017-03.pdf
    - 璁捐鎸囧崡锛?  https://web.archive.org/web/20180712231737/https://www.marvell.com/docs/embedded-processors/assets/marvell-embedded-processors-armada-38x-hardware-design-guide-2017-08.pdf

  鍐呮牳锛?
	ARM Cortex-A9

  Armada 39x 鍨嬪彿锛?
 - 88F6920 Armada 390
 - 88F6925 Armada 395
 - 88F6928 Armada 398

    - 浜у搧淇℃伅锛?https://web.archive.org/web/20181020222559/http://www.marvell.com/embedded-processors/armada-39x/

  鍐呮牳锛?
	ARM Cortex-A9

  Linux 鍐呮牳 mach 鐩綍锛?
	arch/arm/mach-mvebu
  Linux 鍐呮牳 plat 鐩綍锛?
	none

### EBU Armada 绯诲垪锛圓RMv8锛?


  Armada 3710/3720 鍨嬪彿锛?
 - 88F3710
 - 88F3720

  鍐呮牳锛?
	ARM Cortex A53 (ARMv8)

  涓婚〉锛?
	https://web.archive.org/web/20181103003602/http://www.marvell.com/embedded-processors/armada-3700/

  浜у搧绠€浠嬶細
	https://web.archive.org/web/20210121194810/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-37xx-product-brief-2016-01.pdf

  纭欢瑙勬牸锛?
	https://web.archive.org/web/20210202162011/http://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-37xx-hardware-specifications-2019-09.pdf

  璁惧鏍戞枃浠讹細
	arch/arm64/boot/dts/marvell/armada-37*

  Armada 7K 鍨嬪彿锛?
   - 88F6040 (AP806 Quad 600 MHz + one CP110)
   - 88F7020 (AP806 Dual + one CP110)
   - 88F7040 (AP806 Quad + one CP110)

  鍐呮牳锛?ARM Cortex A72

  涓婚〉锛?
	https://web.archive.org/web/20181020222606/http://www.marvell.com/embedded-processors/armada-70xx/

  浜у搧绠€浠嬶細
   - https://web.archive.org/web/20161010105541/http://www.marvell.com/embedded-processors/assets/Armada7020PB-Jan2016.pdf
   - https://web.archive.org/web/20160928154533/http://www.marvell.com/embedded-processors/assets/Armada7040PB-Jan2016.pdf

  璁惧鏍戞枃浠讹細
	arch/arm64/boot/dts/marvell/armada-70*

  Armada 8K 鍨嬪彿锛?
 - 88F8020 (AP806 Dual + two CP110)
 - 88F8040 (AP806 Quad + two CP110)
  鍐呮牳锛?
	ARM Cortex A72

  涓婚〉锛?
	https://web.archive.org/web/20181022004830/http://www.marvell.com/embedded-processors/armada-80xx/

  浜у搧绠€浠嬶細
   - https://web.archive.org/web/20210124233728/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-embedded-processors-armada-8020-product-brief-2017-12.pdf
   - https://web.archive.org/web/20161010105532/http://www.marvell.com/embedded-processors/assets/Armada8040PB-Jan2016.pdf

  璁惧鏍戞枃浠讹細
	arch/arm64/boot/dts/marvell/armada-80*

  Octeon TX2 CN913x 鍨嬪彿锛?
 - CN9130 (AP807 Quad + one internal CP115)
 - CN9131 (AP807 Quad + one internal CP115 + one external CP115 / 88F8215)
 - CN9132 (AP807 Quad + one internal CP115 + two external CP115 / 88F8215)

  鍐呮牳锛?
	ARM Cortex A72

  涓婚〉锛?
	https://web.archive.org/web/20200803150818/https://www.marvell.com/products/infrastructure-processors/multi-core-processors/octeon-tx2/octeon-tx2-cn9130.html

  浜у搧绠€浠嬶細
	https://web.archive.org/web/20200803150818/https://www.marvell.com/content/dam/marvell/en/public-collateral/embedded-processors/marvell-infrastructure-processors-octeon-tx2-cn913x-product-brief-2020-02.pdf

  璁惧鏍戞枃浠讹細
	arch/arm64/boot/dts/marvell/cn913*

### Avanta 绯诲垪


  鍨嬪彿锛?
       - 88F6500
       - 88F6510
       - 88F6530P
       - 88F6550
       - 88F6560
       - 88F6601

  涓婚〉锛?
	https://web.archive.org/web/20181005145041/http://www.marvell.com/broadband/

  浜у搧绠€浠嬶細
	https://web.archive.org/web/20180829171057/http://www.marvell.com/broadband/assets/Marvell_Avanta_88F6510_305_060-001_product_brief.pdf

  鏃犲叕寮€鏁版嵁鎵嬪唽鍙敤銆?

  鍐呮牳锛?
	ARMv5 compatible

  Linux 鍐呮牳 mach 鐩綍锛?
	涓荤嚎涓皻鏃犱唬鐮侊紝璁″垝鍦ㄦ湭鏉ユ敮鎸?
  Linux 鍐呮牳 plat 鐩綍锛?
	涓荤嚎涓皻鏃犱唬鐮侊紝璁″垝鍦ㄦ湭鏉ユ敮鎸?

### Storage 绯诲垪


  Armada SP锛?
 - 88RC1580

  浜у搧淇℃伅锛?
	https://web.archive.org/web/20191129073953/http://www.marvell.com/storage/armada-sp/

  鍐呮牳锛?
	Sheeva ARMv7 compatible Quad-core PJ4C

  锛堜笂娓?Linux 鍐呮牳涓嶆敮鎸侊級

### Dove 绯诲垪锛堝簲鐢ㄥ鐞嗗櫒锛?


  鍨嬪彿锛?
        - 88AP510 a.k.a Armada 510

   浜у搧绠€浠嬶細
	https://web.archive.org/web/20111102020643/http://www.marvell.com/application-processors/armada-500/assets/Marvell_Armada510_SoC.pdf

   纭欢瑙勬牸锛?
	https://web.archive.org/web/20160428160231/http://www.marvell.com/application-processors/armada-500/assets/Armada-510-Hardware-Spec.pdf

  鍔熻兘瑙勬牸锛?
	https://web.archive.org/web/20120130172443/http://www.marvell.com/application-processors/armada-500/assets/Armada-510-Functional-Spec.pdf

  涓婚〉锛?
	https://web.archive.org/web/20160822232651/http://www.marvell.com/application-processors/armada-500/

  鍐呮牳锛?
	ARMv7 compatible

  鐩綍锛?
 - arch/arm/mach-mvebu (DT 鍚敤骞冲彴)
        - arch/arm/mach-dove (闈?DT 鍚敤骞冲彴)

### PXA 2xx/3xx/93x/95x 绯诲垪


  鍨嬪彿锛?
        - PXA21x, PXA25x, PXA26x
             - 浠呭簲鐢ㄥ鐞嗗櫒
             - 鍐呮牳锛欰RMv5 XScale1 core
        - PXA270, PXA271, PXA272
             - 浜у搧绠€浠?        : https://web.archive.org/web/20150927135510/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_pb.pdf
             - 璁捐鎸囧崡          : https://web.archive.org/web/20120111181937/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_design_guide.pdf
             - 寮€鍙戣€呮墜鍐?    : https://web.archive.org/web/20150927164805/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_dev_man.pdf
             - 瑙勬牸         : https://web.archive.org/web/20140211221535/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_emts.pdf
             - 瑙勬牸鏇存柊  : https://web.archive.org/web/20120111104906/http://www.marvell.com/application-processors/pxa-family/assets/pxa_27x_spec_update.pdf
             - 浠呭簲鐢ㄥ鐞嗗櫒
             - 鍐呮牳锛欰RMv5 XScale2 core
        - PXA300, PXA310, PXA320
             - PXA 300 浜у搧绠€浠?: https://web.archive.org/web/20120111121203/http://www.marvell.com/application-processors/pxa-family/assets/PXA300_PB_R4.pdf
             - PXA 310 浜у搧绠€浠?: https://web.archive.org/web/20120111104515/http://www.marvell.com/application-processors/pxa-family/assets/PXA310_PB_R4.pdf
             - PXA 320 浜у搧绠€浠?: https://web.archive.org/web/20121021182826/http://www.marvell.com/application-processors/pxa-family/assets/PXA320_PB_R4.pdf
             - 璁捐鎸囧崡          : https://web.archive.org/web/20130727144625/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_Design_Guide.pdf
             - 寮€鍙戣€呮墜鍐?    : https://web.archive.org/web/20130727144605/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_Developers_Manual.zip
             - 瑙勬牸        : https://web.archive.org/web/20130727144559/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_EMTS.pdf
             - 瑙勬牸鏇存柊  : https://web.archive.org/web/20150927183411/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_Spec_Update.zip
             - 鍙傝€冩墜鍐?     : https://web.archive.org/web/20120111103844/http://www.marvell.com/application-processors/pxa-family/assets/PXA3xx_TavorP_BootROM_Ref_Manual.pdf
             - 浠呭簲鐢ㄥ鐞嗗櫒
             - 鍐呮牳锛欰RMv5 XScale3 core
        - PXA930, PXA935
             - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
             - 鍐呮牳锛欰RMv5 XScale3 core
        - PXA955
             - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
             - 鍐呮牳锛欰RMv7 compatible Sheeva PJ4 core

   璇存槑锛?

    - 杩欎竴绯诲垪 SoC 婧愯嚜 Intel 寮€鍙戠殑 XScale 绯诲垪锛屼簬 2006 骞村乏鍙宠 Marvell 鏀惰喘銆侾XA21x銆丳XA25x銆丳XA26x銆丳XA27x銆丳XA3xx 涓?PXA93x 鐢?Intel 寮€鍙戯紝鑰屽悗鏉ョ殑 PXA95x 鐢?Marvell 寮€鍙戙€?

    - 鐢变簬鍏?XScale 娓婃簮锛岃繖浜?SoC 涓?Marvell 鐨勫叾浠栵紙Kirkwood銆丏ove 绛夛級SoC 绯诲垪鍑犱箮姣棤鍏卞悓鐐癸紝浠呬笌 MMP/MMP2 绯诲垪 SoC 渚嬪銆?

   Linux 鍐呮牳 mach 鐩綍锛?
	arch/arm/mach-pxa

### MMP/MMP2/MMP3 绯诲垪锛堥€氫俊澶勭悊鍣級


  鍨嬪彿锛?
        - PXA168, a.k.a Armada 168
             - 涓婚〉             : https://web.archive.org/web/20110926014256/http://www.marvell.com/application-processors/armada-100/armada-168.jsp
             - 浜у搧绠€浠?       : https://web.archive.org/web/20111102030100/http://www.marvell.com/application-processors/armada-100/assets/pxa_168_pb.pdf
             - 纭欢鎵嬪唽      : https://web.archive.org/web/20160428165359/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_datasheet.pdf
             - 杞欢鎵嬪唽      : https://web.archive.org/web/20160428154454/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_software_manual.pdf
             - 瑙勬牸鏇存柊 : https://web.archive.org/web/20150927160338/http://www.marvell.com/application-processors/armada-100/assets/ARMADA16x_Spec_update.pdf
             - Boot ROM 鎵嬪唽      : https://web.archive.org/web/20130727205559/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_ref_manual.pdf
             - 搴旂敤鑺傜偣鍖?    : https://web.archive.org/web/20141005090706/http://www.marvell.com/application-processors/armada-100/assets/armada_16x_app_note_package.pdf
             - 浠呭簲鐢ㄥ鐞嗗櫒
             - 鍐呮牳锛欰RMv5 compatible Marvell PJ1 88sv331 (Mohawk)
        - PXA910/PXA920
             - 涓婚〉             : https://web.archive.org/web/20150928121236/http://www.marvell.com/communication-processors/pxa910/
             - 浜у搧绠€浠?       : https://archive.org/download/marvell-pxa910-pb/Marvell_PXA910_Platform-001_PB.pdf
             - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
             - 鍐呮牳锛欰RMv5 compatible Marvell PJ1 88sv331 (Mohawk)
        - PXA688, a.k.a. MMP2, a.k.a Armada 610 (OLPC XO-1.75)
             - 浜у搧绠€浠?       : https://web.archive.org/web/20111102023255/http://www.marvell.com/application-processors/armada-600/assets/armada610_pb.pdf
             - 浠呭簲鐢ㄥ鐞嗗櫒
             - 鍐呮牳锛欰RMv7 compatible Sheeva PJ4 88sv581x core
 - PXA2128, a.k.a. MMP3, a.k.a Armada 620 (OLPC XO-4)
      - 浜у搧绠€浠?    : https://web.archive.org/web/20120824055155/http://www.marvell.com/application-processors/armada/pxa2128/assets/Marvell-ARMADA-PXA2128-SoC-PB.pdf
      - 浠呭簲鐢ㄥ鐞嗗櫒
      - 鍐呮牳锛欴ual-core ARMv7 compatible Sheeva PJ4C core
 - PXA960/PXA968/PXA978 (Linux 鏀寔涓嶅湪涓婃父)
      - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
      - 鍐呮牳锛欰RMv7 compatible Sheeva PJ4 core
 - PXA986/PXA988 (Linux 鏀寔涓嶅湪涓婃父)
      - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
      - 鍐呮牳锛欴ual-core ARMv7 compatible Sheeva PJ4B-MP core
 - PXA1088/PXA1920 (Linux 鏀寔涓嶅湪涓婃父)
      - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
      - 鍐呮牳锛歲uad-core ARMv7 Cortex-A7
 - PXA1908/PXA1928/PXA1936
      - 甯﹂€氫俊澶勭悊鍣ㄧ殑搴旂敤澶勭悊鍣?
      - 鍐呮牳锛歮ulti-core ARMv8 Cortex-A53

   璇存槑锛?

    - 杩欎竴绯诲垪 SoC 婧愯嚜 Intel 寮€鍙戠殑 XScale 绯诲垪锛屼簬 2006 骞村乏鍙宠 Marvell 鏀惰喘銆侻MP/MMP2 绯诲垪鐨勬墍鏈夊鐞嗗櫒鍧囩敱 Marvell 寮€鍙戙€?

    - 鐢变簬鍏?XScale 娓婃簮锛岃繖浜?SoC 涓?Marvell 鐨勫叾浠栵紙Kirkwood銆丏ove 绛夛級SoC 绯诲垪鍑犱箮姣棤鍏卞悓鐐癸紝浠呬笌涓婃枃鎵€鍒楃殑 PXA 绯诲垪 SoC 渚嬪銆?

   Linux 鍐呮牳 mach 鐩綍锛?
	arch/arm/mach-mmp

### Berlin 绯诲垪锛堝濯掍綋瑙ｅ喅鏂规锛?


  - 鍨嬪彿锛?
 - 88DE3010, Armada 1000 (鏃?Linux 鏀寔)
  - 鍐呮牳锛?	Marvell PJ1 (ARMv5TE), Dual-core
  - 浜у搧绠€浠嬶細	https://web.archive.org/web/20131103162620/http://www.marvell.com/digital-entertainment/assets/armada_1000_pb.pdf
 - 88DE3005, Armada 1500 Mini
  - 璁捐鍚嶇О锛?BG2CD
  - 鍐呮牳锛?	ARM Cortex-A9, PL310 L2CC
 - 88DE3006, Armada 1500 Mini Plus
  - 璁捐鍚嶇О锛?BG2CDP
  - 鍐呮牳锛?	Dual Core ARM Cortex-A7
 - 88DE3100, Armada 1500
  - 璁捐鍚嶇О锛?BG2
  - 鍐呮牳锛?	Marvell PJ4B-MP (ARMv7), Tauros3 L2CC
 - 88DE3114, Armada 1500 Pro
  - 璁捐鍚嶇О锛?BG2Q
  - 鍐呮牳锛?	Quad Core ARM Cortex-A9, PL310 L2CC
 - 88DE3214, Armada 1500 Pro 4K
  - 璁捐鍚嶇О锛?BG3
  - 鍐呮牳锛?	ARM Cortex-A15, CA15 integrated L2CC
 - 88DE3218, ARMADA 1500 Ultra
  - 鍐呮牳锛?	ARM Cortex-A53

  涓婚〉锛歨ttps://www.synaptics.com/products/multimedia-solutions
  鐩綍锛歛rch/arm/mach-berlin

  璇存槑锛?

   - 杩欎竴绯诲垪 SoC 鍩轰簬 Marvell Sheeva 鎴?ARM Cortex CPU锛屽苟閲囩敤 Synopsys DesignWare锛圛RQ銆丟PIO銆乀imers 绛夛級涓?PXA IP锛圫DHCI銆乁SB銆丒TH 绛夛級銆?

   - Berlin 绯诲垪浜?2017 骞寸敱 Synaptics 浠?Marvell 鏀惰喘銆?

### CPU 鍐呮牳


XScale 鍐呮牳鐢?Intel 璁捐锛屽苟鍦ㄨ緝鑰佺殑 PXA 澶勭悊鍣ㄤ腑鐢?Marvell 鍑鸿揣銆侳eroceon 鏄?Marvell 鍐呴儴寮€鍙戠殑涓撴湁鍐呮牳锛屽悗鏉ユ紨杩涗负 Sheeva銆俋Scale 涓?Feroceon 鍐呮牳闅忔椂闂磋閫愭娣樻卑锛屽湪鍚庣画浜у搧涓 Sheeva 鍐呮牳鍙栦唬锛岄殢鍚庡張琚幏寰楁巿鏉冪殑 ARM Cortex-A 鍐呮牳鍙栦唬銆?

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

### 闀挎湡璁″垝


 - 灏?mach-dove/銆乵ach-mv78xx0/銆乵ach-orion5x/ 缁熶竴鍒?mach-mvebu/ 涓紝浠ュ湪鍗曚竴鐨?mach-<foo> 鐩綍涓嬫敮鎸佹潵鑷?Marvell EBU锛圗ngineering Business Unit锛屽伐绋嬩笟鍔″崟鍏冿級鐨勫叏閮?SoC銆傚洜姝?plat-orion/ 灏嗕笉澶嶅瓨鍦ㄣ€?

### 鑷磋阿


- Maen Suleiman <maen@marvell.com>
- Lior Amsalem <alior@marvell.com>
- Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
- Andrew Lunn <andrew@lunn.ch>
- Nicolas Pitre <nico@fluxnic.net>
- Eric Miao <eric.y.miao@gmail.com>
