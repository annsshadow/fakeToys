## Texas Instruments Keystone Navigator 闃熷垪绠＄悊瀛愮郴缁燂紙QMSS锛夐┍鍔?

椹卞姩婧愪唬鐮佽矾寰?  drivers/soc/ti/knav_qmss.c
  drivers/soc/ti/knav_qmss_acc.c

Keystone SoC 涓婄殑 QMSS锛堥槦鍒楃鐞嗗瓙绯荤粺锛孮ueue Manager Sub System锛夋槸鏋勬垚 Keystone 澶氭牳 Navigator 楠ㄥ共鐨勪富瑕佺‖浠跺瓙绯荤粺涔嬩竴銆俀MSS 鐢遍槦鍒楃鐞嗗櫒銆佹墦鍖呮暟鎹粨鏋勫鐞嗗櫒锛圥DSP锛夈€侀摼鎺?RAM銆佹弿杩扮姹犱互鍙婂熀纭€璁炬柦 Packet DMA 缁勬垚銆?闃熷垪绠＄悊鍣ㄦ槸涓€涓‖浠舵ā鍧楋紝璐熻矗鍔犻€熸暟鎹寘闃熷垪鐨勭鐞嗐€傛暟鎹寘閫氳繃鍚戠壒瀹氱殑鍐呭瓨鏄犲皠鍦板潃鍐欏叆鎴栬鍙栨弿杩扮鍦板潃鏉ヨ繘琛屽叆闃?鍑洪槦鎿嶄綔銆侾DSP 鎵ц涓?QMSS 鐩稿叧鐨勫姛鑳斤紝濡傜疮绉紙accumulation锛夈€丵oS 鎴栦簨浠剁鐞嗐€傞摼鎺?RAM 瀵勫瓨鍣ㄧ敤浜庨摼鎺ュ瓨鍌ㄥ湪鎻忚堪绗?RAM 涓殑鎻忚堪绗︺€傛弿杩扮 RAM 鍙厤缃负鍐呴儴鎴栧閮ㄥ唴瀛樸€俀MSS 椹卞姩璐熻矗绠＄悊 PDSP 鐨勮缃€侀摼鎺?RAM 鍖哄煙銆侀槦鍒楁睜绠＄悊锛堝垎閰嶃€佸帇鍏ャ€佸脊鍑轰笌閫氱煡锛変互鍙婃弿杩扮姹犵鐞嗐€?
knav qmss 椹卞姩鍚戝叾浠栭┍鍔ㄦ彁渚涗竴缁?API锛岀敤浜庢墦寮€/鍏抽棴 qmss 闃熷垪銆佸垎閰嶆弿杩扮姹犮€佹槧灏勬弿杩扮銆佸悜闃熷垪鍘嬪叆/寮瑰嚭绛夈€傛湁鍏冲彲鐢?API 鐨勮缁嗕俊鎭紝璇峰弬闃?include/linux/soc/ti/knav_qmss.h

DT 鏂囨。浣嶄簬
Documentation/devicetree/bindings/soc/ti/keystone-navigator-qmss.txt

## 浣跨敤 PDSP 鍥轰欢鐨勭疮绉櫒 QMSS 闃熷垪

QMSS PDSP 鍥轰欢鏀寔绱Н鍣ㄩ€氶亾锛屽彲鐩戣鍗曚釜闃熷垪鎴栧涓繛缁殑闃熷垪銆俤rivers/soc/ti/knav_qmss_acc.c 鏄笌绱Н鍣?PDSP 浜や簰鐨勯┍鍔ㄣ€傚畠浼氶厤缃?DTS锛堝弬瑙?DT 鏂囨。涓殑绀轰緥锛変腑瀹氫箟鐨勭疮绉櫒閫氶亾锛屼互姣忎釜閫氶亾鐩戣 1 鎴?32 涓槦鍒椼€傛湁鍏宠鍥轰欢鐨勬洿澶氳鏄庯紝鍙煡闃?CPPI/QMSS 浣庡眰椹卞姩鏂囨。锛坉ocs/CPPI_QMSS_LLD_SDS.pdf锛夛紝浣嶄簬

	git://git.ti.com/keystone-rtos/qmss-lld.git

k2_qmss_pdsp_acc48_k2_le_1_0_0_9.bin 鍥轰欢鏈€澶氭敮鎸?48 涓疮绉櫒閫氶亾銆傝鍥轰欢浣嶄簬 firmware.git 鐨?ti-keystone 鐩綍涓嬶紝鍦板潃涓?
   git://git.kernel.org/pub/scm/linux/kernel/git/firmware/linux-firmware.git

浣跨敤鏃讹紝璇峰皢鍥轰欢闀滃儚澶嶅埗鍒?initramfs 鎴?ubifs 鏂囦欢绯荤粺鐨?lib/firmware 鐩綍锛屽苟鍦ㄦ枃浠剁郴缁熶腑涓?k2_qmss_pdsp_acc48_k2_le_1_0_0_9.bin 鎻愪緵绗﹀彿閾炬帴锛岀劧鍚庡惎鍔ㄥ唴鏍搞€傝嫢鍥轰欢鎴愬姛鍔犺浇鍒?PDSP锛岀敤鎴峰皢鍦ㄥ惎鍔ㄦ棩蹇椾腑鐪嬪埌

 "firmware file ks2_qmss_pdsp_acc48.bin downloaded for PDSP"

浣跨敤绱Н闃熷垪瑕佹眰鍥轰欢闀滃儚瀛樺湪浜庢枃浠剁郴缁熶腑銆傚鏋?SoC 涓殑 PDSP 鏈繍琛岋紝椹卞姩涓嶄細灏嗙疮绉槦鍒楀姞鍏ュ彈鏀寔鐨勯槦鍒楄寖鍥淬€傚鏋滃绱Н闃熷垪鍙戣捣闃熷垪鎵撳紑璇锋眰鑰?PDSP 鏈繍琛岋紝鍒?API 璋冪敤浼氬け璐ャ€傚洜姝わ紝鍦ㄤ娇鐢ㄨ繖浜涢槦鍒楃被鍨嬩箣鍓嶏紝鍔″繀灏嗗浐浠跺鍒跺埌鏂囦欢绯荤粺涓€?