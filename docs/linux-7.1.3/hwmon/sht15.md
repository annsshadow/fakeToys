## 鍐呮牳椹卞姩 sht15


Authors:

  - Wouter Horre
  - Jonathan Cameron
  - Vivien Didelot <vivien.didelot@savoirfairelinux.com>
  - Jerome Oufella <jerome.oufella@savoirfairelinux.com>

鏀寔鐨勮澶囷細

  - Sensirion SHT10

    Prefix: 'sht10'

  - Sensirion SHT11

    Prefix: 'sht11'

  - Sensirion SHT15

    Prefix: 'sht15'

  - Sensirion SHT71

    Prefix: 'sht71'

  - Sensirion SHT75

    Prefix: 'sht75'

Datasheet: 鍙湪 Sensirion 缃戠珯鍏紑鑾峰彇

	http://www.sensirion.ch/en/pdf/product_information/Datasheet-humidity-sensor-SHT1x.pdf

### 鎻忚堪


SHT10銆丼HT11銆丼HT15銆丼HT71 涓?SHT75 鏄箍搴︿笌娓╁害浼犳劅鍣ㄣ€?
杩欎簺鍣ㄤ欢浣跨敤涓ゆ潯 GPIO 绾胯繘琛岄€氫俊銆?
鏀寔鐨勬祴閲忓垎杈ㄧ巼涓烘俯搴?14 浣嶃€佹箍搴?12 浣嶏紝鎴栨俯搴?12 浣嶃€佹箍搴?8 浣嶃€?
婀垮害鏍″噯绯绘暟琚儳褰曞湪鑺墖鐨?OTP 瀛樺偍鍣ㄤ腑銆傝繖浜涚郴鏁扮敤浜庡鍐呮潵鑷紶鎰熷櫒鐨?淇″彿杩涜鍐呴儴鏍″噯銆傜鐢ㄨ繖浜涚郴鏁扮殑閲嶆柊鍔犺浇鍙互涓烘瘡娆℃祴閲忚妭鐪?10ms 骞堕檷浣?鍔熻€楋紝浣嗕細鎹熷け绮惧害銆?
涓€浜涢€夐」鍙互閫氳繃 sysfs 灞炴€ц缃€?
娉ㄦ剰锛?  - 璋冭妭鍣ㄧ數婧愬悕绉拌璁剧疆涓?鈥渧cc鈥濄€?  - 濡傛灉 CRC 鏍￠獙澶辫触锛屼細鍙戦€佷竴涓蒋澶嶄綅鍛戒护锛屽皢鐘舵€佸瘎瀛樺櫒閲嶇疆涓哄叾纭欢
    榛樿鍊硷紝浣嗛┍鍔ㄤ細灏濊瘯鎭㈠鍏堝墠鐨勮澶囬厤缃€?
### 骞冲彴鏁版嵁


- checksum锛?  璁句负 true 浠ュ惎鐢ㄨ鏁扮殑 CRC 鏍￠獙锛堥粯璁や负 false锛夈€?- no_otp_reload锛?  鎸囩ず涓嶄粠 OTP 閲嶆柊鍔犺浇鐨勬爣蹇楋紙榛樿涓?false锛夈€?- low_resolution锛?  鎸囩ず瑕佷娇鐢ㄧ殑娓╁害/婀垮害鍒嗚鲸鐜囩殑鏍囧織锛堥粯璁や负 false锛夈€?
### Sysfs 鎺ュ彛


================== ==========================================================
temp1_input        娓╁害杈撳叆
humidity1_input    婀垮害杈撳叆
heater_enable      鍚戣灞炴€у啓鍏?1 浠ュ惎鐢ㄧ墖鍐呭姞鐑櫒锛屽啓鍏?0 浠ョ鐢ㄣ€?		    娉ㄦ剰涓嶈灏嗗姞鐑櫒鍚敤杩囦箙銆?temp1_fault        鑻ヤ负 1锛岃〃绀虹數鍘嬭繃浣庯紙浣庝簬 2.47V锛夛紝娴嬮噺鍙兘鏃犳晥銆?humidity1_fault    鍚?temp1_fault銆?================== ==========================================================
