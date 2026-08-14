
## Raspberry Pi PiSP 鐩告満鍓嶇锛坮p1-cfe锛?

## PiSP 鐩告満鍓嶇锛圕amera Front End锛?

PiSP 鐩告満鍓嶇锛圕FE锛夋槸涓€涓ā鍧楋紝瀹冨皢涓€涓?CSI-2 鎺ユ敹鍣ㄤ笌涓€涓О涓哄墠绔紙FE锛夌殑绠€鍗?ISP 缁勫悎鍦ㄤ竴璧枫€?
CFE 鏈夊洓涓?DMA 寮曟搸锛屽彲浠ュ皢浠?CSI-2 鎺ユ敹鍒扮殑鍥涗釜鐙珛娴佷腑鐨勫抚鍐欏叆鍐呭瓨銆傚叾涓煇涓€涓祦涔熷彲浠ョ洿鎺ヨ矾鐢卞埌 FE锛孎E 鍙互杩涜鏈€灏戠殑鍥惧儚澶勭悊锛屽皢鎺ユ敹鍒扮殑甯х殑涓や釜鐗堟湰锛堜緥濡傛湭缂╂斁鐗堟湰涓庣缉灏忕増鏈級鍐欏叆鍐呭瓨锛屽苟鎻愪緵鎵€鎺ユ敹甯х殑缁熻淇℃伅銆?
FE 瀵勫瓨鍣ㄨ褰曞湪 `Raspberry Pi Image Signal Processor (ISP) Specification document <https://datasheets.raspberrypi.com/camera/raspberry-pi-image-signal-processor-specification.pdf>`_ 涓紝FE 鐨勭ず渚嬩唬鐮佸彲浠ュ湪 `libpisp <https://github.com/raspberrypi/libpisp>`_ 涓壘鍒般€?
## rp1-cfe 椹卞姩


Raspberry Pi PiSP 鐩告満鍓嶇锛坮p1-cfe锛夐┍鍔ㄤ綅浜?drivers/media/platform/raspberrypi/rp1-cfe銆傚畠浣跨敤 `V4L2 API` 娉ㄥ唽鑻ュ共瑙嗛鎹曡幏涓庤緭鍑鸿澶囷紝浣跨敤 `V4L2 subdev API` 涓?CSI-2 鎺ユ敹绔笌杩炴帴瑙嗛璁惧鐨?FE 娉ㄥ唽瀛愯澶囷紝杩欎簺璁惧鐢变竴涓娇鐢?`Media Controller (MC) API` 瀹炵幇鐨勫崟涓€濯掍綋鍥捐繛鎺ャ€?
鐢?`rp1-cfe` 椹卞姩娉ㄥ唽鐨勫獟浣撴嫇鎵戯紝鍦ㄨ繖涓繛鎺ュ埌 imx219 浼犳劅鍣ㄧ殑鐗瑰畾绀轰緥涓紝濡備笅鎵€杩帮細


    :alt:   涓€涓ず渚嬪獟浣撴祦姘寸嚎鎷撴墤鍥?    :align: center

璇ュ獟浣撳浘鍖呭惈浠ヤ笅瑙嗛璁惧鑺傜偣锛?
- rp1-cfe-csi2-ch0锛氱涓€涓?CSI-2 娴佺殑鎹曡幏璁惧
- rp1-cfe-csi2-ch1锛氱浜屼釜 CSI-2 娴佺殑鎹曡幏璁惧
- rp1-cfe-csi2-ch2锛氱涓変釜 CSI-2 娴佺殑鎹曡幏璁惧
- rp1-cfe-csi2-ch3锛氱鍥涗釜 CSI-2 娴佺殑鎹曡幏璁惧
- rp1-cfe-fe-image0锛氱涓€涓?FE 杈撳嚭鐨勬崟鑾疯澶?- rp1-cfe-fe-image1锛氱浜屼釜 FE 杈撳嚭鐨勬崟鑾疯澶?- rp1-cfe-fe-stats锛欶E 缁熻淇℃伅鐨勬崟鑾疯澶?- rp1-cfe-fe-config锛欶E 閰嶇疆鐨勮緭鍑鸿澶?
### rp1-cfe-csi2-chX


rp1-cfe-csi2-chX 鎹曡幏璁惧鏄櫘閫氱殑 V4L2 鎹曡幏璁惧锛屽彲鐢ㄤ簬鎹曡幏浠?CSI-2 鎺ユ敹鍒扮殑瑙嗛甯ф垨鍏冩暟鎹€?
### rp1-cfe-fe-image0, rp1-cfe-fe-image1


rp1-cfe-fe-image0 涓?rp1-cfe-fe-image1 鎹曡幏璁惧鐢ㄤ簬灏嗗鐞嗗悗鐨勫抚鍐欏叆鍐呭瓨銆?
### rp1-cfe-fe-stats


FE 缁熻淇℃伅缂撳啿鍖虹殑鏍煎紡鐢?`pisp_statistics` C 缁撴瀯浣撳畾涔夛紝姣忎釜鍙傛暟鐨勫惈涔夊湪 `PiSP specification` 鏂囨。涓弿杩般€?
### rp1-cfe-fe-config


FE 閰嶇疆缂撳啿鍖虹殑鏍煎紡鐢?`pisp_fe_config` C 缁撴瀯浣撳畾涔夛紝姣忎釜鍙傛暟鐨勫惈涔夊湪 `PiSP specification` 鏂囨。涓弿杩般€?