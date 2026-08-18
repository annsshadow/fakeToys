


######## V4L2_PIX_FMT_SRGGB14P ('pREE'), V4L2_PIX_FMT_SGRBG14P ('pgEE'), V4L2_PIX_FMT_SGBRG14P ('pGEE'), V4L2_PIX_FMT_SBGGR14P ('pBEE'),

鏈〉鎻忚堪 V4L2 涓洓绉?14 浣嶆墦鍖呯殑 sRGB/Bayer 鍘熷鍍忕礌鏍煎紡锛圫RGGB14P銆丼GRBG14P銆丼GBRG14P銆丼BGGR14P锛夛紝璇存槑鍏跺瓧鑺傛墦鍖呮帓鍒楁柟寮忎笌鍐呭瓨甯冨眬锛屼緵鎽勫儚澶翠笌鍥惧儚澶勭悊搴旂敤姝ｇ‘瑙ｈ鍘熷甯ф暟鎹€?


**man V4L2_PIX_FMT_SRGGB14P(2)**

V4L2_PIX_FMT_SGRBG14P
V4L2_PIX_FMT_SGBRG14P
V4L2_PIX_FMT_SBGGR14P
14 浣嶆墦鍖呯殑 Bayer 鏍煎紡

## 鎻忚堪


杩欏洓绉嶅儚绱犳牸寮忔槸姣忚壊 14 浣嶇殑鎵撳寘鍘熷 sRGB / Bayer 鏍煎紡銆傛瘡鍥涗釜杩炵画鐨勯噰鏍疯
鎵撳寘杩涗竷涓瓧鑺傘€傚墠鍥涗釜瀛楄妭鍚勫寘鍚儚绱犵殑鍏釜楂樹綅锛岄殢鍚庣殑涓変釜瀛楄妭浠ョ浉鍚岄『搴忓寘鍚?
姣忎釜鍍忕礌鐨勫叚涓綆浣嶃€?

姣忎釜 n 鍍忕礌琛屽寘鍚?n/2 涓豢鑹查噰鏍峰拰 n/2 涓摑鑹叉垨绾㈣壊閲囨牱锛岀豢鑹?绾㈣壊涓庣豢鑹?钃濊壊
琛屼氦鏇挎帓鍒椼€傚畠浠€氬父鎻忚堪涓?GRGR... BGBG...銆丷GRG... GBGB... 绛夈€備笅闈㈡槸鍏朵腑涓€绉?
鏍煎紡鐨勭ず渚嬶細

**瀛楄妭椤哄簭銆?* 姣忎釜鍗曞厓鏍间负涓€涓瓧鑺傘€?


    \begingroup
    \footnotesize
    \setlength{\tabcolsep}{2pt}


    :header-rows:  0
    :stub-columns: 0
    :widths:       2 1 1 1 1 3 3 3


    - .. row 1

       - start + 0

       - B\ `00high`

       - G\ `01high`

       - B\ `02high`

       - G\ `03high`

       - G\ `01low bits 1--0`\ (bits 7--6)

	  B\ `00low bits 5--0`\ (bits 5--0)

       - B\ `02low bits 3--0`\ (bits 7--4)

	  G\ `01low bits 5--2`\ (bits 3--0)

       - G\ `03low bits 5--0`\ (bits 7--2)

	  B\ `02low bits 5--4`\ (bits 1--0)

    - .. row 2

       - start + 7

       - G\ `10high`

       - R\ `11high`

       - G\ `12high`

       - R\ `13high`

       - R\ `11low bits 1--0`\ (bits 7--6)

	  G\ `10low bits 5--0`\ (bits 5--0)

       - G\ `12low bits 3--0`\ (bits 7--4)

	  R\ `11low bits 5--2`\ (bits 3--0)

       - R\ `13low bits 5--0`\ (bits 7--2)

	  G\ `12low bits 5--4`\ (bits 1--0)

    - .. row 3

       - start + 14

       - B\ `20high`

       - G\ `21high`

       - B\ `22high`

       - G\ `23high`

       - G\ `21low bits 1--0`\ (bits 7--6)

	  B\ `20low bits 5--0`\ (bits 5--0)

       - B\ `22low bits 3--0`\ (bits 7--4)

	  G\ `21low bits 5--2`\ (bits 3--0)

       - G\ `23low bits 5--0`\ (bits 7--2)

	  B\ `22low bits 5--4`\ (bits 1--0)

    - .. row 4

       - start + 21

       - G\ `30high`

       - R\ `31high`

       - G\ `32high`

       - R\ `33high`

       - R\ `31low bits 1--0`\ (bits 7--6)
	  G\ `30low bits 5--0`\ (bits 5--0)

       - G\ `32low bits 3--0`\ (bits 7--4)
	  R\ `31low bits 5--2`\ (bits 3--0)

       - R\ `33low bits 5--0`\ (bits 7--2)
	  G\ `32low bits 5--4`\ (bits 1--0)


    \endgroup
