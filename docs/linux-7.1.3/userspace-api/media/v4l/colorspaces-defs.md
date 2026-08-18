
######## Defining Colorspaces in V4L2


鍦?V4L2 涓紝鑹插僵绌洪棿鐢卞洓涓€煎畾涔夈€傜涓€涓槸鑹插僵绌洪棿鏍囪瘑绗︼紙enum `v4l2_pix_format_mplane`锛夛紝
瀹冨畾涔変簡鑹插害銆侀粯璁や紶閫掑嚱鏁般€侀粯璁?Y'CbCr 缂栫爜鍜岄粯璁ら噺鍖栨柟娉曘€?
绗簩涓槸浼犻€掑嚱鏁版爣璇嗙锛坋num `v4l2_pix_format_mplane`锛夛紝鐢ㄤ簬鎸囧畾闈炴爣鍑嗕紶閫掑嚱鏁般€?
绗笁涓槸 Y'CbCr 缂栫爜鏍囪瘑绗︼紙enum `v4l2_pix_format_mplane`锛夛紝鐢ㄤ簬鎸囧畾闈炴爣鍑?Y'CbCr 缂栫爜锛?
绗洓涓槸閲忓寲鏍囪瘑绗︼紙enum `v4l2_pix_format_mplane`锛夛紝鐢ㄤ簬鎸囧畾闈炴爣鍑嗛噺鍖栨柟娉曘€?
澶у鏁版儏鍐典笅鍙渶瑕佸～鍐?struct `v4l2_pix_format_mplane` 鎴?struct `v4l2_pix_format_mplane` 鐨?colorspace 瀛楁銆?


鍦?HSV 鏍煎紡 <hsv-formats> 涓婏紝**Hue锛堣壊鐩革級** 琚畾涔変负鍦嗘煴棰滆壊琛ㄧず涓婄殑瑙掑害銆?
閫氬父璇ヨ搴︿互搴︿负鍗曚綅搴﹂噺锛屽嵆 0-360銆傚綋灏嗚瑙掑害鍊兼槧灏勫埌 8 浣嶆椂锛屾湁涓ょ鍩烘湰鏂瑰紡锛?
灏嗚搴﹀€奸櫎浠?2锛?-179锛夛紝鎴栦娇鐢ㄦ暣涓寖鍥?0-255锛屽皢瑙掑害鍊奸櫎浠?1.41銆?
enum `v4l2_hsv_encoding` 鎸囧畾浣跨敤鍝缂栫爜銆?

   鑹插僵绌洪棿銆侶SV 鏍煎紡濮嬬粓涓哄叏鑼冨洿銆?



    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_COLORSPACE_RAW`
      - 榛樿鑹插僵绌洪棿銆傚簲鐢ㄧ▼搴忓彲浣跨敤瀹冭椹卞姩濉厖鑹插僵绌洪棿銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-smpte-170m銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-rec709銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-srgb銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-oprgb銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-bt2020銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-dcip3銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-smpte-240m銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-sysm銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-sysbg銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍙傝 col-jpeg銆?
    - - `V4L2_COLORSPACE_RAW`
      - 鍘熷鑹插僵绌洪棿銆傜敤浜庡師濮嬪浘鍍忛噰闆嗭紝姝ゆ椂鍥惧儚缁忚繃鏈€灏戝鐞嗗苟浣跨敤璁惧鍐呴儴鐨勮壊褰╃┖闂淬€?
      浣跨敤姝も€滆壊褰╃┖闂粹€濆鐞嗗浘鍍忕殑杞欢蹇呴』浜嗚В閲囬泦璁惧鐨勫唴閮ㄧ粏鑺傘€?





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤鑹插僵绌洪棿瀹氫箟鐨勯粯璁や紶閫掑嚱鏁般€?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤 Rec. 709 浼犻€掑嚱鏁般€?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤 sRGB 浼犻€掑嚱鏁般€?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤 opRGB 浼犻€掑嚱鏁般€?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤 SMPTE 240M 浼犻€掑嚱鏁般€?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 涓嶄娇鐢ㄤ紶閫掑嚱鏁帮紙鍗充娇鐢ㄧ嚎鎬?RGB 鍊硷級銆?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤 DCI-P3 浼犻€掑嚱鏁般€?
    - - `V4L2_XFER_FUNC_SMPTE2084`
      - 浣跨敤 SMPTE 2084 浼犻€掑嚱鏁般€傚弬瑙?xf-smpte-2084銆?





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤鑹插僵绌洪棿瀹氫箟鐨勯粯璁?Y'CbCr 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤 BT.601 Y'CbCr 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤 Rec. 709 Y'CbCr 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤鎵╁睍鑹插煙 xvYCC BT.601 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤鎵╁睍鑹插煙 xvYCC Rec. 709 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤榛樿闈炲父閲忎寒搴?BT.2020 Y'CbCr 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤甯搁噺浜害 BT.2020 Yc'CbcCrc 缂栫爜銆?
    - - `V4L2_YCBCR_ENC_SMPTE_240M`
      - 浣跨敤 SMPTE 240M Y'CbCr 缂栫爜銆?





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_HSV_ENC_256`
      - 瀵逛簬鑹茬浉锛屾瘡涓?LSB 涓轰袱搴︺€?
    - - `V4L2_HSV_ENC_256`
      - 瀵逛簬鑹茬浉锛?60 搴︽槧灏勫埌 8 浣嶏紝鍗虫瘡涓?LSB 绾︿负 1.41 搴︺€?





    :header-rows:  1
    :stub-columns: 0

    - - Identifier
      - Details
    - - `V4L2_QUANTIZATION_LIM_RANGE`
      - 浣跨敤鑹插僵绌洪棿瀹氫箟鐨勯粯璁ら噺鍖栫紪鐮併€傚浜?R'G'B' 鍜?HSV 杩欏缁堟槸鍏ㄨ寖鍥淬€?
      瀵逛簬 Y'CbCr 閫氬父涓哄彈闄愯寖鍥淬€?
    - - `V4L2_QUANTIZATION_LIM_RANGE`
      - 浣跨敤鍏ㄨ寖鍥撮噺鍖栫紪鐮併€傚嵆鑼冨洿 [0鈥?] 鏄犲皠鍒?[0鈥?55]锛堝彲鑳借鍒囧埌 [1鈥?54] 浠ラ伩鍏?
      0x00 鍜?0xff 鍊硷級銆侰b 鍜?Cr 浠?[-0.5鈥?.5] 鏄犲皠鍒?[0鈥?55]锛堝彲鑳借鍒囧埌 [1鈥?54] 浠ラ伩鍏?
      0x00 鍜?0xff 鍊硷級銆?
    - - `V4L2_QUANTIZATION_LIM_RANGE`
      - 浣跨敤鍙楅檺鑼冨洿閲忓寲缂栫爜銆傚嵆鑼冨洿 [0鈥?] 鏄犲皠鍒?[16鈥?35]銆侰b 鍜?Cr 浠?[-0.5鈥?.5] 鏄犲皠鍒?
      [16鈥?40]銆傚彈闄愯寖鍥翠笉鑳戒笌 HSV 涓€璧蜂娇鐢ㄣ€?

