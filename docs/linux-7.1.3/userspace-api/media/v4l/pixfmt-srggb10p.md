

######## V4L2_PIX_FMT_SRGGB10P ('pRAA'), V4L2_PIX_FMT_SGRBG10P ('pgAA'), V4L2_PIX_FMT_SGBRG10P ('pGAA'), V4L2_PIX_FMT_SBGGR10P ('pBAA'),



V4L2_PIX_FMT_SGRBG10P
V4L2_PIX_FMT_SGBRG10P
V4L2_PIX_FMT_SBGGR10P
10 浣嶆墦鍖?Bayer 鏍煎紡


## 鎻忚堪


杩欏洓绉嶅儚绱犳牸寮忔槸姣忔牱鏈?10 浣嶇殑鎵撳寘鍘熷 sRGB / Bayer 鏍煎紡銆傛瘡鍥涗釜杩炵画鐨勬牱鏈鎵撳寘杩?5 涓瓧鑺傘€傚墠 4 涓瓧鑺備腑鐨勬瘡涓€涓寘鍚儚绱犵殑 8 涓珮浣嶏紝绗?5 涓瓧鑺備互鐩稿悓椤哄簭鍖呭惈姣忎釜鍍忕礌鐨?2 涓渶浣庢湁鏁堜綅銆?
姣忎釜 n 鍍忕礌琛屽寘鍚?n/2 涓豢鑹叉牱鏈拰 n/2 涓摑鑹叉垨绾㈣壊鏍锋湰锛岀豢-绾㈣涓庣豢-钃濊浜ゆ浛銆傚畠浠€氬父琚弿杩颁负 GRGR... BGBG...銆丷GRG... GBGB... 绛夈€備笅闈㈡槸涓€涓皬鐨?V4L2_PIX_FMT_SBGGR10P 鍥惧儚绀轰緥锛?
**瀛楄妭椤哄簭銆?*
姣忎釜鍗曞厓鏍间负涓€涓瓧鑺傘€?

    :header-rows:  0
    :stub-columns: 0
    :widths: 12 8 8 8 8 68

    - - start + 0:
      - B\ `00high`
      - G\ `01high`
      - B\ `02high`
      - G\ `03high`
      - G\ `03low`\ (bits 7--6) B\ `02low`\ (bits 5--4)

	G\ `01low`\ (bits 3--2) B\ `00low`\ (bits 1--0)
    - - start + 5:
      - G\ `10high`
      - R\ `11high`
      - G\ `12high`
      - R\ `13high`
      - R\ `13low`\ (bits 7--6) G\ `12low`\ (bits 5--4)

	R\ `11low`\ (bits 3--2) G\ `10low`\ (bits 1--0)
    - - start + 10:
      - B\ `20high`
      - G\ `21high`
      - B\ `22high`
      - G\ `23high`
      - G\ `23low`\ (bits 7--6) B\ `22low`\ (bits 5--4)

	G\ `21low`\ (bits 3--2) B\ `20low`\ (bits 1--0)
    - - start + 15:
      - G\ `30high`
      - R\ `31high`
      - G\ `32high`
      - R\ `33high`
      - R\ `33low`\ (bits 7--6) G\ `32low`\ (bits 5--4)

	R\ `31low`\ (bits 3--2) G\ `30low`\ (bits 1--0)
