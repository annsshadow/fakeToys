


######## 鎵撳寘锛圥acked锛塝UV 鏍煎紡


涓庢墦鍖?RGB 鏍煎紡绫讳技锛屾墦鍖?YUV 鏍煎紡灏?Y銆丆b 鍜?Cr 鍒嗛噺鍦ㄥ唴瀛樹腑杩炵画瀛樺偍銆傚畠浠彲鑳藉鑹插害鍒嗛噺搴旂敤瀛愰噰鏍凤紝鍥犳鍦ㄤ氦閿欒繖涓変釜鍒嗛噺鐨勬柟寮忎笂鏈夋墍涓嶅悓銆?
   - 鍦ㄤ互涓嬫墍鏈夎〃鏍间腑锛屼綅 7 鏄竴涓瓧鑺備腑鏈€楂樻湁鏁堜綅銆?   - 鈥淵鈥濄€佲€淐b鈥濆拰鈥淐r鈥濆垎鍒〃绀轰寒搴︺€佽摑鑹茶壊搴︼紙涔熺О涓衡€淯鈥濓級鍜岀孩鑹茶壊搴︼紙涔熺О涓衡€淰鈥濓級鍒嗛噺鐨勪綅銆傗€淎鈥濊〃绀?alpha 鍒嗛噺鐨勪綅锛堝鏋滄牸寮忔敮鎸侊級锛屸€淴鈥濊〃绀哄～鍏呬綅銆?
## 4:4:4 瀛愰噰鏍?

杩欎簺鏍煎紡涓嶅鑹插害鍒嗛噺杩涜瀛愰噰鏍凤紝骞跺皢姣忎釜鍍忕礌瀛樺偍涓轰竴涓敱 Y銆丆b 鍜?Cr 鍊肩粍鎴愮殑瀹屾暣涓夊厓缁勩€?
涓嬩竴涓〃鏍煎垪鍑轰簡姣忎釜鍒嗛噺灏戜簬 8 浣嶇殑鎵撳寘 YUV 4:4:4 鏍煎紡銆傚畠浠牴鎹湪 16 浣嶅瓧涓湅鍒扮殑 Y銆丆b 鍜?Cr 鍒嗛噺椤哄簭锛堥殢鍚庝互 little endian 瀛楄妭搴忓瓨鍏ュ唴瀛橈級浠ュ強姣忎釜鍒嗛噺鐨勪綅鏁版潵鍛藉悕銆備緥濡傦紝YUV565 鏍煎紡灏嗕竴涓儚绱犲瓨鍌ㄥ湪涓€涓?16 浣嶅瓧 [15:0] 涓紝甯冨眬涓?[Y'\ `4-0` Cb\ `5-0` Cr\ `4-0`]锛屽苟浠ヤ袱涓瓧鑺傚瓨鍏ュ唴瀛橈紝[Cb\ `2-0` Cr\ `4-0`] 鍦ㄥ墠锛屾帴鐫€鏄?[Y'\ `4-0` Cb\ `5-3`]銆?
    \begingroup
    \scriptsize
    \setlength{\tabcolsep}{2pt}


    :header-rows:  2
    :stub-columns: 0

    - - Identifier
      - Code

      - `7` Byte 0 in memory

      - `7` Byte 1

#     * -

      - 7
      - 6
      - 5
      - 4
      - 3
      - 2
      - 1
      - 0

      - 7
      - 6
      - 5
      - 4
      - 3
      - 2
      - 1
      - 0

    - .. _V4L2-PIX-FMT-YUV444:

      - `V4L2_PIX_FMT_YUV444`
      - 'Y444'

      - Cb\ `3`
      - Cb\ `2`
      - Cb\ `1`
      - Cb\ `0`
      - Cr\ `3`
      - Cr\ `2`
      - Cr\ `1`
      - Cr\ `0`

      - a\ `3`
      - a\ `2`
      - a\ `1`
      - a\ `0`
      - Y'\ `3`
      - Y'\ `2`
      - Y'\ `1`
      - Y'\ `0`

    - .. _V4L2-PIX-FMT-YUV555:

      - `V4L2_PIX_FMT_YUV555`
      - 'YUVO'

      - Cb\ `2`
      - Cb\ `1`
      - Cb\ `0`
      - Cr\ `4`
      - Cr\ `3`
      - Cr\ `2`
      - Cr\ `1`
      - Cr\ `0`

      - a
      - Y'\ `4`
      - Y'\ `3`
      - Y'\ `2`
      - Y'\ `1`
      - Y'\ `0`
      - Cb\ `4`
      - Cb\ `3`

    - .. _V4L2-PIX-FMT-YUV565:

      - `V4L2_PIX_FMT_YUV565`
      - 'YUVP'

      - Cb\ `2`
      - Cb\ `1`
      - Cb\ `0`
      - Cr\ `4`
      - Cr\ `3`
      - Cr\ `2`
      - Cr\ `1`
      - Cr\ `0`

      - Y'\ `4`
      - Y'\ `3`
      - Y'\ `2`
      - Y'\ `1`
      - Y'\ `0`
      - Cb\ `5`
      - Cb\ `4`
      - Cb\ `3`


    \endgroup


    瀵逛簬 YUV444 鍜?YUV555 鏍煎紡锛宎lpha 浣嶇殑鍊煎湪浠庨┍鍔ㄨ鍙栨椂鏈畾涔夛紝鍦ㄥ啓鍏ラ┍鍔ㄦ椂琚拷鐣ワ紝闄ら潪閽堝 :ref:`瑙嗛鍙犲姞 <overlay>` 鎴栬棰戣緭鍑哄彔鍔?<osd> 鍗忓晢浜?alpha 娣峰悎銆?
涓嬩竴涓〃鏍煎垪鍑轰簡姣忎釜鍒嗛噺 8 浣嶇殑鎵撳寘 YUV 4:4:4 鏍煎紡銆傚畠浠牴鎹湪鍐呭瓨涓瓨鍌ㄧ殑 Y銆丆b 鍜?Cr 鍒嗛噺椤哄簭浠ュ強姣忓儚绱犵殑鎬讳綅鏁版潵鍛藉悕銆備緥濡傦紝VUYX32 鏍煎紡灏嗗儚绱犵殑 Cr\ `7-0` 瀛樺偍鍦ㄧ涓€涓瓧鑺傘€丆b\ `7-0` 瀛樺偍鍦ㄧ浜屼釜瀛楄妭銆乊'\ `7-0` 瀛樺偍鍦ㄧ涓変釜瀛楄妭銆?
    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 0
      - Byte 1
      - Byte 2
      - Byte 3

    - .. _V4L2-PIX-FMT-YUV32:

      - `V4L2_PIX_FMT_YUV32`
      - 'YUV4'

      - A\ `7-0`
      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`

    - .. _V4L2-PIX-FMT-AYUV32:

      - `V4L2_PIX_FMT_AYUV32`
      - 'AYUV'

      - A\ `7-0`
      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`

    - .. _V4L2-PIX-FMT-XYUV32:

      - `V4L2_PIX_FMT_XYUV32`
      - 'XYUV'

      - X\ `7-0`
      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`

    - .. _V4L2-PIX-FMT-VUYA32:

      - `V4L2_PIX_FMT_VUYA32`
      - 'VUYA'

      - Cr\ `7-0`
      - Cb\ `7-0`
      - Y'\ `7-0`
      - A\ `7-0`

    - .. _V4L2-PIX-FMT-VUYX32:

      - `V4L2_PIX_FMT_VUYX32`
      - 'VUYX'

      - Cr\ `7-0`
      - Cb\ `7-0`
      - Y'\ `7-0`
      - X\ `7-0`

    - .. _V4L2-PIX-FMT-YUVA32:

      - `V4L2_PIX_FMT_YUVA32`
      - 'YUVA'

      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`
      - A\ `7-0`

    - .. _V4L2-PIX-FMT-YUVX32:

      - `V4L2_PIX_FMT_YUVX32`
      - 'YUVX'

      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`
      - X\ `7-0`

    - .. _V4L2-PIX-FMT-YUV24:

      - `V4L2_PIX_FMT_YUV24`
      - 'YUV3'

      - Y'\ `7-0`
      - Cb\ `7-0`
      - Cr\ `7-0`
      - -\

    - alpha 鍒嗛噺搴斿寘鍚竴涓椹卞姩鍜屽簲鐢ㄧ▼搴忔湁鎰忎箟鐨勫€笺€?    - 濉厖浣嶅寘鍚湭瀹氫箟鐨勫€硷紝蹇呴』琚墍鏈夊簲鐢ㄧ▼搴忓拰椹卞姩蹇界暐銆?
涓嬩竴涓〃鏍煎垪鍑轰簡姣忎釜鍒嗛噺 12 浣嶇殑鎵撳寘 YUV 4:4:4 鏍煎紡銆傚皢姣忎釜鍒嗛噺鎵╁睍鍒?16 浣嶏紝鏁版嵁鏀惧湪楂樺瓧鑺傦紝浣庡瓧鑺傝ˉ闆讹紝鎸?little endian 椤哄簭鎺掑垪锛岀敤 6 涓瓧鑺傚瓨鍌?1 涓儚绱犮€?
    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 1-0
      - Byte 3-2
      - Byte 5-4
      - Byte 7-6
      - Byte 9-8
      - Byte 11-10

    - .. _V4L2-PIX-FMT-YUV48-12:

      - `V4L2_PIX_FMT_YUV48_12`
      - 'Y312'

      - Y'\ `0`
      - Cb\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `1`
      - Cr\ `1`

## 4:2:2 瀛愰噰鏍?

杩欎簺閫氬父绉颁负 YUYV 鎴?YUY2 鐨勬牸寮忥紝灏嗚壊搴﹀垎閲忔按骞冲瓙閲囨牱 2 鍊嶏紝鍦ㄥ鍣ㄤ腑瀛樺偍 2 涓儚绱犮€? 浣嶆牸寮忕殑瀹瑰櫒涓?32 浣嶏紝10 浣嶅強浠ヤ笂鏍煎紡鐨勫鍣ㄤ负 64 浣嶃€?
姣忎釜鍒嗛噺澶氫簬 8 浣嶇殑鎵撳寘 YUYV 鏍煎紡瀛樺偍涓哄洓涓?16 浣嶇殑 little endian 瀛椼€傛瘡涓瓧鐨勬渶楂樻湁鏁堜綅鍖呭惈涓€涓垎閲忥紝鏈€浣庢湁鏁堜綅涓洪浂濉厖銆?

    \footnotesize


    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 0
      - Byte 1
      - Byte 2
      - Byte 3
      - Byte 4
      - Byte 5
      - Byte 6
      - Byte 7
    - .. _V4L2-PIX-FMT-UYVY:

      - `V4L2_PIX_FMT_UYVY`
      - 'UYVY'

      - Cb\ `0`
      - Y'\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `2`
      - Y'\ `2`
      - Cr\ `2`
      - Y'\ `3`
    - .. _V4L2-PIX-FMT-VYUY:

      - `V4L2_PIX_FMT_VYUY`
      - 'VYUY'

      - Cr\ `0`
      - Y'\ `0`
      - Cb\ `0`
      - Y'\ `1`
      - Cr\ `2`
      - Y'\ `2`
      - Cb\ `2`
      - Y'\ `3`
    - .. _V4L2-PIX-FMT-YUYV:

      - `V4L2_PIX_FMT_YUYV`
      - 'YUYV'

      - Y'\ `0`
      - Cb\ `0`
      - Y'\ `1`
      - Cr\ `0`
      - Y'\ `2`
      - Cb\ `2`
      - Y'\ `3`
      - Cr\ `2`
    - .. _V4L2-PIX-FMT-YVYU:

      - `V4L2_PIX_FMT_YVYU`
      - 'YVYU'

      - Y'\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `0`
      - Y'\ `2`
      - Cr\ `2`
      - Y'\ `3`
      - Cb\ `2`


    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Word 0
      - Word 1
      - Word 2
      - Word 3
    - .. _V4L2-PIX-FMT-Y210:

      - `V4L2_PIX_FMT_Y210`
      - 'Y210'

      - Y'\ `0` (bits 15-6)
      - Cb\ `0` (bits 15-6)
      - Y'\ `1` (bits 15-6)
      - Cr\ `0` (bits 15-6)
    - .. _V4L2-PIX-FMT-Y212:

      - `V4L2_PIX_FMT_Y212`
      - 'Y212'

      - Y'\ `0` (bits 15-4)
      - Cb\ `0` (bits 15-4)
      - Y'\ `1` (bits 15-4)
      - Cr\ `0` (bits 15-4)
    - .. _V4L2-PIX-FMT-Y216:

      - `V4L2_PIX_FMT_Y216`
      - 'Y216'

      - Y'\ `0` (bits 15-0)
      - Cb\ `0` (bits 15-0)
      - Y'\ `1` (bits 15-0)
      - Cr\ `0` (bits 15-0)


    \normalsize

**棰滆壊閲囨牱浣嶇疆锛?*
鑹插害鏍锋湰鍦ㄦ按骞虫柟鍚戜笂灞呬腑鎻掍簬鍏堕棿<yuv-chroma-centered>銆?
## 4:1:1 瀛愰噰鏍?

姝ゆ牸寮忓皢鑹插害鍒嗛噺姘村钩瀛愰噰鏍?4 鍊嶏紝鐢?12 涓瓧鑺傚瓨鍌?8 涓儚绱犮€?

    \scriptsize


    :header-rows: 1
    :stub-columns: 0

    - - Identifier
      - Code
      - Byte 0
      - Byte 1
      - Byte 2
      - Byte 3
      - Byte 4
      - Byte 5
      - Byte 6
      - Byte 7
      - Byte 8
      - Byte 9
      - Byte 10
      - Byte 11
    - .. _V4L2-PIX-FMT-Y41P:

      - `V4L2_PIX_FMT_Y41P`
      - 'Y41P'

      - Cb\ `0`
      - Y'\ `0`
      - Cr\ `0`
      - Y'\ `1`
      - Cb\ `4`
      - Y'\ `2`
      - Cr\ `4`
      - Y'\ `3`
      - Y'\ `4`
      - Y'\ `5`
      - Y'\ `6`
      - Y'\ `7`


    \normalsize


    涓嶈灏?`V4L2_PIX_FMT_Y41P` 涓?    V4L2_PIX_FMT_YUV411P <V4L2-PIX-FMT-YUV411P> 娣锋穯銆俌41P 娲剧敓鑷?    鈥淵UV 4:1:1 **packed**鈥濓紙鎵撳寘锛夛紝鑰?YUV411P 浠ｈ〃 鈥淵UV 4:1:1 **planar**鈥濓紙骞抽潰锛夈€?
**棰滆壊閲囨牱浣嶇疆锛?*
鑹插害鏍锋湰鍦ㄦ按骞虫柟鍚戜笂灞呬腑鎻掍簬鍏堕棿<yuv-chroma-centered>銆?