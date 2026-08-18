
## Qualcomm 鎽勫儚澶村瓙绯荤粺椹卞姩


### 绠€浠?

鏈枃浠惰褰曚簡浣嶄簬 drivers/media/platform/qcom/camss 涓嬬殑 Qualcomm 鎽勫儚澶?瀛愮郴缁熼┍鍔ㄣ€?
褰撳墠鐗堟湰鐨勯┍鍔ㄦ敮鎸佸湪 Qualcomm MSM8916/APQ8016 浠ュ強 MSM8996/APQ8096
澶勭悊鍣ㄤ笂鍙戠幇鐨勬憚鍍忓ご瀛愮郴缁熴€?
璇ラ┍鍔ㄥ疄鐜颁簡 V4L2銆丮edia controller锛堝獟浣撴帶鍒跺櫒锛変互鍙?V4L2 subdev 鎺ュ彛銆?鏀寔鍦ㄥ唴鏍镐腑浣跨敤 V4L2 subdev 鎺ュ彛鐨勬憚鍍忓ご浼犳劅鍣ㄣ€?
璇ラ┍鍔ㄤ互 Code Linaro 涓殑 Qualcomm 鎽勫儚澶村瓙绯荤粺 Android 椹卞姩 [#f1]_ [#f2]_
浣滀负鍙傝€冨疄鐜般€?

### Qualcomm 鎽勫儚澶村瓙绯荤粺纭欢


椹卞姩鎵€鏀寔鐨?8x16 / 8x96 澶勭悊鍣ㄤ笂鐨勬憚鍍忓ご瀛愮郴缁熺‖浠剁敱浠ヤ笅閮ㄥ垎缁勬垚锛?
- 2 / 3 涓?CSIPHY 妯″潡銆傚畠浠鐞?CSI2 鎺ユ敹鍣ㄧ殑鐗╃悊灞傘€傛瘡涓?CSIPHY 妯″潡
  鍙繛鎺ヤ竴涓嫭绔嬬殑鎽勫儚澶翠紶鎰熷櫒锛?- 2 / 4 涓?CSID锛圕SI 瑙ｇ爜鍣級妯″潡銆傚畠浠鐞?CSI2 鎺ユ敹鍣ㄧ殑鍗忚灞備笌搴旂敤
  灞傘€備竴涓?CSID 鍙互瑙ｇ爜鏉ヨ嚜浠绘剰 CSIPHY 鐨勬暟鎹祦銆傛瘡涓?CSID 杩樺寘鍚竴涓?  TG锛堟祴璇曠敓鎴愬櫒锛夊潡锛屽彲鐢ㄤ簬鐢熸垚浜哄伐杈撳叆鏁版嵁浠ヨ繘琛屾祴璇曪紱
- ISPIF锛圛SP 鎺ュ彛锛夋ā鍧椼€傝礋璐ｅ皢鏁版嵁娴佷粠 CSID 璺敱鍒?VFE 鐨勮緭鍏ョ锛?- 1 / 2 涓?VFE锛堣棰戝墠绔級妯″潡銆傚寘鍚竴鏉″浘鍍忓鐞嗙殑纭欢鍧楁祦姘寸嚎銆俈FE
  鍏锋湁澶氱杈撳叆鎺ュ彛銆侾IX锛堝儚绱狅級杈撳叆鎺ュ彛灏嗚緭鍏ユ暟鎹€佸叆鍥惧儚澶勭悊娴佹按绾裤€?  鍥惧儚澶勭悊娴佹按绾挎湯绔繕鍖呭惈涓€涓缉鏀句笌瑁佸壀妯″潡銆備笁涓?RDI锛圧aw Dump
  Interface锛屽師濮嬭浆鍌ㄦ帴鍙ｏ級杈撳叆鎺ュ彛浼氱粫杩囧浘鍍忓鐞嗘祦姘寸嚎銆俈FE 杩樺寘鍚?  灏嗚緭鍑烘暟鎹啓鍏ュ唴瀛樼殑 AXI 鎬荤嚎鎺ュ彛銆?

### 鏀寔鐨勫姛鑳?

褰撳墠鐗堟湰鐨勯┍鍔ㄦ敮鎸侊細

- 閫氳繃 CSIPHY 鏉ヨ嚜鎽勫儚澶翠紶鎰熷櫒鐨勮緭鍏ワ紱
- 鐢?CSID 涓殑 TG 鐢熸垚娴嬭瘯杈撳叆鏁版嵁锛?- VFE 鐨?RDI 鎺ュ彛

  - 灏嗚緭鍏ユ暟鎹師濮嬭浆鍌ㄥ埌鍐呭瓨銆?
    鏀寔鐨勬牸寮忥細

    - YUYV/UYVY/YVYU/VYUY锛堟墦鍖?YUV 4:2:2 - V4L2_PIX_FMT_YUYV /
      V4L2_PIX_FMT_UYVY / V4L2_PIX_FMT_YVYU / V4L2_PIX_FMT_VYUY锛夛紱
    - MIPI RAW8锛? 浣?Bayer RAW - V4L2_PIX_FMT_SRGGB8 /
      V4L2_PIX_FMT_SGRBG8 / V4L2_PIX_FMT_SGBRG8 / V4L2_PIX_FMT_SBGGR8锛夛紱
    - MIPI RAW10锛?0 浣嶆墦鍖?Bayer RAW - V4L2_PIX_FMT_SBGGR10P /
      V4L2_PIX_FMT_SGBRG10P / V4L2_PIX_FMT_SGRBG10P / V4L2_PIX_FMT_SRGGB10P /
      V4L2_PIX_FMT_Y10P锛夛紱
    - MIPI RAW12锛?2 浣嶆墦鍖?Bayer RAW - V4L2_PIX_FMT_SRGGB12P /
      V4L2_PIX_FMT_SGBRG12P / V4L2_PIX_FMT_SGRBG12P / V4L2_PIX_FMT_SRGGB12P锛夈€?    - 锛堜粎 8x96锛塎IPI RAW14锛?4 浣嶆墦鍖?Bayer RAW - V4L2_PIX_FMT_SRGGB14P /
      V4L2_PIX_FMT_SGBRG14P / V4L2_PIX_FMT_SGRBG14P / V4L2_PIX_FMT_SRGGB14P锛夈€?
  - 锛堜粎 8x96锛夎緭鍏ユ暟鎹殑鏍煎紡杞崲銆?
    鏀寔鐨勮緭鍏ユ牸寮忥細

    - MIPI RAW10锛?0 浣嶆墦鍖?Bayer RAW - V4L2_PIX_FMT_SBGGR10P / V4L2_PIX_FMT_Y10P锛夈€?
    鏀寔鐨勮緭鍑烘牸寮忥細

    - Plain16 RAW10锛?0 浣嶉潪鎵撳寘 Bayer RAW - V4L2_PIX_FMT_SBGGR10 / V4L2_PIX_FMT_Y10锛夈€?
- VFE 鐨?PIX 鎺ュ彛

  - 杈撳叆鏁版嵁鐨勬牸寮忚浆鎹€?
    鏀寔鐨勮緭鍏ユ牸寮忥細

    - YUYV/UYVY/YVYU/VYUY锛堟墦鍖?YUV 4:2:2 - V4L2_PIX_FMT_YUYV /
      V4L2_PIX_FMT_UYVY / V4L2_PIX_FMT_YVYU / V4L2_PIX_FMT_VYUY锛夈€?
    鏀寔鐨勮緭鍑烘牸寮忥細

    - NV12/NV21锛堝弻骞抽潰 YUV 4:2:0 - V4L2_PIX_FMT_NV12 / V4L2_PIX_FMT_NV21锛夛紱
    - NV16/NV61锛堝弻骞抽潰 YUV 4:2:2 - V4L2_PIX_FMT_NV16 / V4L2_PIX_FMT_NV61锛夈€?    - 锛堜粎 8x96锛塝UYV/UYVY/YVYU/VYUY锛堟墦鍖?YUV 4:2:2 - V4L2_PIX_FMT_YUYV /
      V4L2_PIX_FMT_UYVY / V4L2_PIX_FMT_YVYU / V4L2_PIX_FMT_VYUY锛夈€?
  - 缂╂斁鏀寔銆傞厤缃?VFE Encoder Scale 妯″潡浠ヨ繘琛屾渶楂?16x 鐨勭缉灏忋€?
  - 瑁佸壀鏀寔銆傞厤缃?VFE Encoder Crop 妯″潡銆?
- 涓や釜锛?x96锛氫笁涓級鏁版嵁杈撳叆鐨勫苟鍙戜笌鐙珛浣跨敤鈥斺€斿彲浠ユ槸鎽勫儚澶翠紶鎰熷櫒
  鍜?鎴?TG銆?

### 椹卞姩鏋舵瀯涓庤璁?

璇ラ┍鍔ㄥ疄鐜颁簡 V4L2 subdev 鎺ュ彛銆備负浜嗗妯″潡涔嬮棿鐨勭‖浠惰繛鎺ヨ繘琛屽缓妯★紝骞?鏆撮湶涓€涓共鍑€銆佸悎涔庨€昏緫涓斿彲鐢ㄧ殑鎺ュ彛锛岄┍鍔ㄦ寜濡備笅鏂瑰紡鎷嗗垎涓?V4L2 瀛愯澶?锛?x16 / 8x96锛夛細

- 2 / 3 涓?CSIPHY 瀛愯澶団€斺€旀瘡涓?CSIPHY 鐢变竴涓嫭绔嬬殑瀛愯澶囪〃绀猴紱
- 2 / 4 涓?CSID 瀛愯澶団€斺€旀瘡涓?CSID 鐢变竴涓嫭绔嬬殑瀛愯澶囪〃绀猴紱
- 2 / 4 涓?ISPIF 瀛愯澶団€斺€擨SPIF 鐢辨暟閲忎笌 CSID 瀛愯澶囩浉绛夌殑瀛愯澶囪〃绀猴紱
- 4 / 8 涓?VFE 瀛愯澶団€斺€擵FE 鐢辨暟閲忎笌杈撳叆鎺ュ彛鏁扮浉绛夌殑瀛愯澶囪〃绀猴紙姣忎釜
  VFE 鏈?3 涓?RDI 鍜?1 涓?PIX锛夈€?
浠ユ鐗瑰畾鏂瑰紡鎷嗗垎椹卞姩鐨勭悊鐢卞涓嬶細

- 灏?CSIPHY 鍜?CSID 妯″潡鍚勮嚜琛ㄧず涓轰竴涓嫭绔嬬殑瀛愯澶囷紝鍙互瀵硅繖浜涙ā鍧椾箣闂?  鐨勭‖浠惰繛鎺ヨ繘琛屽缓妯★紱
- 灏?VFE 鐨勬瘡涓緭鍏ユ帴鍙ｈ〃绀轰负鐙珛鐨勫瓙璁惧锛屽彲浠ュ苟鍙戜笖鐙珛鍦颁娇鐢ㄨ繖浜?  杈撳叆鎺ュ彛锛屾濡傜‖浠舵墍鏀寔鐨勯偅鏍凤紱
- 灏?ISPIF 琛ㄧず涓烘暟閲忎笌 CSID 瀛愯澶囩浉绛夌殑瀛愯澶囷紝鍙互鍦ㄥ悓鏃朵娇鐢ㄤ袱涓?  鎽勫儚澶存椂鍒涘缓绾挎€х殑濯掍綋鎺у埗鍣ㄦ祦姘寸嚎銆傝繖閬垮厤浜嗘祦姘寸嚎涓殑鍒嗘敮锛屽惁鍒?  鍒嗘敮浼氳姹?a) 鐢ㄦ埛绌洪棿浠ュ強 b) 濯掍綋妗嗘灦锛堜緥濡備笂鐢?涓嬬數鎿嶄綔锛夊浠庡崟涓?  濯掍綋瀹炰綋鐨?sink pad 鍒?source pad 鐨勬暟鎹祦鍋氬嚭鍋囪銆?
姣忎釜 VFE 瀛愯澶囬兘杩炴帴鍒颁竴涓嫭绔嬬殑瑙嗛璁惧鑺傜偣銆?
濯掍綋鎺у埗鍣ㄦ祦姘寸嚎鍥惧涓嬶紙杩炴帴浜嗕袱涓?/ 涓変釜 OV5645 鎽勫儚澶翠紶鎰熷櫒锛夛細


    :alt:   qcom_camss_graph.dot
    :align: center

    Media pipeline graph 8x16

    :alt:   qcom_camss_8x96_graph.dot
    :align: center

    Media pipeline graph 8x96


### 瀹炵幇


褰撳墠鎵€鏀寔鐨勫姛鑳藉苟涓嶉渶瑕佺‖浠剁殑杩愯鏃堕厤缃紙鍦ㄦ祦浼犺緭杩囩▼涓洿鏂拌缃級銆?姣忎釜纭欢妯″潡鐨勫畬鏁撮厤缃兘鍦?STREAMON ioctl 鏃讹紝鏍规嵁褰撳墠婵€娲荤殑濯掍綋閾捐矾銆?鏍煎紡鍜屽凡璁剧疆鐨勬帶鍒堕」杩涜搴旂敤銆?
VFE 涓缉鏀惧櫒妯″潡鐨勮緭鍑哄昂瀵革紝鐢?'msm_vfe0_pix' 瀹炰綋鐨?sink pad 涓婂疄闄呯殑
compose 閫夊尯鐭╁舰鏉ラ厤缃€?
VFE 涓鍓ā鍧楃殑杈撳嚭瑁佸壀鍖哄煙锛岀敱 'msm_vfe0_pix' 瀹炰綋鐨?source pad 涓?瀹為檯鐨?crop 閫夊尯鐭╁舰鏉ラ厤缃€?

### 鏂囨。


APQ8016 瑙勬牸锛?https://developer.qualcomm.com/download/sd410/snapdragon-410-processor-device-specification.pdf
寮曠敤鏃ユ湡 2016-11-24銆?
APQ8096 瑙勬牸锛?https://developer.qualcomm.com/download/sd820e/qualcomm-snapdragon-820e-processor-apq8096sge-device-specification.pdf
寮曠敤鏃ユ湡 2018-06-22銆?
### 鍙傝€?