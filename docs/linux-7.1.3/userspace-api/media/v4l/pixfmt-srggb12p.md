


######## V4L2_PIX_FMT_SRGGB12P ('pRCC')銆乂4L2_PIX_FMT_SGRBG12P ('pgCC')銆乂4L2_PIX_FMT_SGBRG12P ('pGCC')銆乂4L2_PIX_FMT_SBGGR12P ('pBCC')


### 12 浣嶆墦鍖?Bayer 鏍煎紡



## 鎻忚堪


杩欏洓绉嶅儚绱犳牸寮忔槸姣忛鑹?12 浣嶇殑鎵撳寘鍘熷 sRGB / Bayer 鏍煎紡銆傛瘡涓や釜杩炵画鐨勯噰鏍疯
鎵撳寘杩涗笁涓瓧鑺傘€傚墠涓や釜瀛楄妭鍚勫寘鍚儚绱犵殑 8 涓珮浣嶏紝绗笁涓瓧鑺傚寘鍚瘡涓儚绱犵殑 4 涓?鏈€浣庢湁鏁堜綅锛岄『搴忕浉鍚屻€?
姣忎釜 n 鍍忕礌琛屽寘鍚?n/2 涓豢鑹查噰鏍峰拰 n/2 涓摑鑹叉垨绾㈣壊閲囨牱锛岀豢鑹?绾㈣壊涓庣豢鑹?钃濊壊琛?浜ゆ浛鎺掑垪銆傚畠浠€氬父鎻忚堪涓?GRGR... BGBG...銆丷GRG... GBGB... 绛夈€備笅闈㈡槸涓€涓皬鐨?V4L2_PIX_FMT_SBGGR12P 鍥惧儚绀轰緥锛?
**瀛楄妭椤哄簭銆?*
姣忎釜鍗曞厓鏍间负涓€涓瓧鑺傘€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       2 1 1 1 1 1 1


    - -  start + 0:
       - B\ `00high`
       - G\ `01high`
       - G\ `01low`\ (bits 7--4)

          B\ `00low`\ (bits 3--0)
       - B\ `02high`
       - G\ `03high`
       - G\ `03low`\ (bits 7--4)

          B\ `02low`\ (bits 3--0)

    - -  start + 6:
       - G\ `10high`
       - R\ `11high`
       - R\ `11low`\ (bits 7--4)

          G\ `10low`\ (bits 3--0)
       - G\ `12high`
       - R\ `13high`
       - R\ `13low`\ (bits 7--4)

          G\ `12low`\ (bits 3--0)
    - -  start + 12:
       - B\ `20high`
       - G\ `21high`
       - G\ `21low`\ (bits 7--4)

          B\ `20low`\ (bits 3--0)
       - B\ `22high`
       - G\ `23high`
       - G\ `23low`\ (bits 7--4)

          B\ `22low`\ (bits 3--0)
    - -  start + 18:
       - G\ `30high`
       - R\ `31high`
       - R\ `31low`\ (bits 7--4)

          G\ `30low`\ (bits 3--0)
       - G\ `32high`
       - R\ `33high`
       - R\ `33low`\ (bits 7--4)

          G\ `32low`\ (bits 3--0)
