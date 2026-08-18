## TI Keystone Linux 姒傝堪


### 绠€浠?


Keystone 绯诲垪 SoC 鍩轰簬 ARM Cortex-A15 MPCore 澶勭悊鍣?
鍜?c66x DSP 鍐呮牳銆傛湰鏂囨。鎻忚堪浜嗙敤鎴峰湪寰峰窞浠櫒锛圱I锛夌殑
Keystone 璇勪及妯″潡锛圗VM锛変笂杩愯 Linux 鎵€闇€鐨勫熀鏈俊鎭€?

鐩墠鏀寔浠ヤ笅 SoC 鍜?EVM锛?

## K2HK SoC 涓?EVM


a.k.a Keystone 2 Hawking/Kepler SoC
TCI6636K2H 涓?TCI6636K2K锛氭枃妗ｅ弬瑙?

	http://www.ti.com/product/tci6638k2k
	http://www.ti.com/product/tci6638k2h

EVM锛?
  http://www.advantech.com/Support/TI-EVM/EVMK2HX_sd.aspx

## K2E SoC 涓?EVM


a.k.a Keystone 2 Edison SoC

K2E  -  66AK2E05:

鏂囨。鍙傝

	http://www.ti.com/product/66AK2E05/technicaldocuments

EVM锛?
   https://www.einfochips.com/index.php/partnerships/texas-instruments/k2e-evm.html

## K2L SoC 涓?EVM


a.k.a Keystone 2 Lamarr SoC

K2L  -  TCI6630K2L:

鏂囨。鍙傝
	http://www.ti.com/product/TCI6630K2L/technicaldocuments

EVM锛?
  https://www.einfochips.com/index.php/partnerships/texas-instruments/k2l-evm.html

### 閰嶇疆


鎵€鏈?K2 SoC/EVM 鍏变韩涓€涓€氱敤 defconfig锛坘eystone_defconfig锛夛紝涓斾娇鐢?
鐩稿悓鐨勯暅鍍忓湪鍚?EVM 涓婂惎鍔ㄣ€傚钩鍙伴厤缃€氳繃
DTS 鎸囧畾銆備娇鐢ㄧ殑 DTS 濡備笅锛?

	K2HK EVM:
		k2hk-evm.dts
	K2E EVM:
		k2e-evm.dts
	K2L EVM:
		k2l-evm.dts

Keystone 璁惧鐨勮澶囨爲鏂囨。浣嶄簬

        Documentation/devicetree/bindings/arm/ti/ti,keystone.yaml

### 鏂囨。浣滆€?


Murali Karicheri <m-karicheri2@ti.com>

鐗堟潈鎵€鏈?2015 寰峰窞浠櫒
