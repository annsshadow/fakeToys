## MIPI CCS 鎽勫儚澶翠紶鎰熷櫒椹卞姩


MIPI CCS 鎽勫儚澶翠紶鎰熷櫒椹卞姩鏄竴涓潰鍚?`MIPI CCS <https://www.mipi.org/specifications/camera-command-set>`_
鍏煎鎽勫儚澶翠紶鎰熷櫒鐨勯€氱敤椹卞姩銆?
鍙﹁鍙傞槄 CCS 椹卞姩 UAPI 鏂囨。 <media-ccs-uapi>銆?
### CCS 闈欐€佹暟鎹?

MIPI CCS 椹卞姩鏀寔鎵€鏈夊吋瀹硅澶囩殑 CCS 闈欐€佹暟鎹紝涓嶄粎鍖呮嫭鍏煎 CCS 1.1 鐨勮澶囷紝涔熷寘鎷?CCS 1.0 鍜?SMIA(++)銆傚浜?CCS锛屾枃浠跺悕鏋勬垚涓?
	ccs/ccs-sensor-vvvv-mmmm-rrrr.fw锛堜紶鎰熷櫒锛変互鍙?	ccs/ccs-module-vvvv-mmmm-rrrr.fw锛堟ā鍧楋級銆?
瀵逛簬鍏煎 SMIA++ 鐨勮澶囷紝鐩稿簲鐨勬枃浠跺悕涓?
	ccs/smiapp-sensor-vv-mmmm-rr.fw锛堜紶鎰熷櫒锛変互鍙?	ccs/smiapp-module-vv-mmmm-rrrr.fw锛堟ā鍧楋級銆?
瀵逛簬鍏煎 SMIA锛堥潪 ++锛夌殑璁惧锛岄潤鎬佹暟鎹枃浠跺悕涓?
	ccs/smia-sensor-vv-mmmm-rr.fw锛堜紶鎰熷櫒锛夈€?
vvvv 鎴?vv 鍒嗗埆琛ㄧず MIPI 鍜?SMIA 鍘傚晢 ID锛宮mmm 涓哄瀷鍙?ID锛宺rrr 鎴?rr 涓虹増鏈彿銆?
#### CCS 宸ュ叿


`CCS tools <https://github.com/MIPI-Alliance/ccs-tools/>`_ 鏄竴缁勭敤浜庡鐞?CCS 闈欐€佹暟鎹枃浠剁殑
宸ュ叿銆侰CS tools 鍖呭惈浜虹被鍙鐨?CCS 闈欐€佹暟鎹?YAML 鏍煎紡鐨勫畾涔夛紝骞跺寘鍚竴涓皢鍏惰浆鎹负浜岃繘鍒剁殑
绋嬪簭銆?
### 瀵勫瓨鍣ㄥ畾涔夌敓鎴愬櫒


ccs-regs.asc 鏂囦欢鍖呭惈 MIPI CCS 瀵勫瓨鍣ㄥ畾涔夛紝鐢ㄤ簬鐢熸垚鏇翠究浜?C 璇█绋嬪簭浣跨敤鐨?C 婧愪唬鐮佸畾涔夋枃浠躲€?鐢变簬鐢熸垚鐨勬枃浠朵箣闂村瓨鍦ㄨ澶氫緷璧栧叧绯伙紝璇蜂笉瑕佹墜鍔ㄤ慨鏀瑰畠浠紝鍥犱负閭ｅ鏄撳嚭閿欎笖寰掑姵鏃犲姛锛岃€屽簲淇敼
鐢熸垚瀹冧滑鐨勮剼鏈€?
#### 鐢ㄦ硶


鎸夌収鎯緥锛岃剼鏈互濡備笅鏂瑰紡璋冪敤鏉ユ洿鏂?CCS 椹卞姩瀹氫箟锛?

	$ Documentation/driver-api/media/drivers/ccs/mk-ccs-regs -k \
		-e drivers/media/i2c/ccs/ccs-regs.h \
		-L drivers/media/i2c/ccs/ccs-limits.h \
		-l drivers/media/i2c/ccs/ccs-limits.c \
		-c Documentation/driver-api/media/drivers/ccs/ccs-regs.asc

## CCS PLL 璁＄畻鍣?

CCS PLL 璁＄畻鍣ㄧ敤浜庡湪缁欏畾浼犳劅鍣ㄨ兘鍔涖€佹澘閰嶇疆浠ュ強鐢ㄦ埛鎸囧畾閰嶇疆鐨勬儏鍐典笅璁＄畻 PLL 閰嶇疆銆傜敱浜庢兜鐩栨墍鏈?杩欎簺閰嶇疆鐨勯厤缃┖闂撮潪甯稿簽澶э紝PLL 璁＄畻鍣ㄥ苟闈炲畬鍏ㄧ畝鍗曘€備絾瀵逛簬椹卞姩鑰岃█瀹冪浉瀵规槗浜庝娇鐢ㄣ€?
PLL 璁＄畻鍣ㄥ疄鐜扮殑 PLL 妯″瀷瀵瑰簲浜?MIPI CCS 1.1銆?

**Copyright** |copy| 2020 Intel Corporation
