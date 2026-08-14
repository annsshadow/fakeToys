



######## V4L2_PIX_FMT_SRGGB16 ('RG16')銆乂4L2_PIX_FMT_SGRBG16 ('GR16')銆乂4L2_PIX_FMT_SGBRG16 ('GB16')銆乂4L2_PIX_FMT_SBGGR16 ('BYR2')


## 16 浣?Bayer 鏍煎紡


## 鎻忚堪


杩欏洓绉嶅儚绱犳牸寮忔槸鍘熷 sRGB / Bayer 鏍煎紡锛屾瘡涓牱鏈?16 浣嶃€傛瘡涓牱鏈瓨鍌ㄥ湪涓€涓?16 浣嶅瓧涓€傛瘡涓?n 鍍忕礌琛屽寘鍚?n/2 涓豢鑹叉牱鏈拰 n/2 涓摑鑹叉垨绾㈣壊鏍锋湰锛岀孩鑹插拰钃濊壊琛屼氦鏇裤€傚瓧鑺備互灏忕椤哄簭瀛樺偍鍦ㄥ唴瀛樹腑銆傚畠浠€氬父琚弿杩颁负 GRGR... BGBG...銆丷GRG... GBGB... 绛夈€備笅闈㈡槸涓€涓皬绀轰緥 V4L2_PIX_FMT_SBGGR16 鍥惧儚锛?

**瀛楄妭搴忋€?*
姣忎釜鍗曞厓涓轰竴涓瓧鑺傘€?

    :header-rows:  0
    :stub-columns: 0

    - - start + 0:
      - B\ `00low`
      - B\ `00high`
      - G\ `01low`
      - G\ `01high`
      - B\ `02low`
      - B\ `02high`
      - G\ `03low`
      - G\ `03high`
    - - start + 8:
      - G\ `10low`
      - G\ `10high`
      - R\ `11low`
      - R\ `11high`
      - G\ `12low`
      - G\ `12high`
      - R\ `13low`
      - R\ `13high`
    - - start + 16:
      - B\ `20low`
      - B\ `20high`
      - G\ `21low`
      - G\ `21high`
      - B\ `22low`
      - B\ `22high`
      - G\ `23low`
      - G\ `23high`
    - - start + 24:
      - G\ `30low`
      - G\ `30high`
      - R\ `31low`
      - R\ `31high`
      - G\ `32low`
      - G\ `32high`
      - R\ `33low`
      - R\ `33high`
