## spi_lm70llp锛歀M70-LLP 骞跺彛杞?SPI 閫傞厤鍣?

鏀寔鐨勬澘鍗?鑺墖锛?
  - National Semiconductor LM70 LLP 璇勪及鏉?
    鏁版嵁鎵嬪唽: https://www.ti.com/lit/gpn/lm70

浣滆€?
        Kaiwan N Billimoria <kaiwan@designergraphix.com>

### 鎻忚堪

璇ラ┍鍔ㄦ彁渚涚矘鍚堜唬鐮侊紝灏?National Semiconductor LM70 LLP 娓╁害浼犳劅鍣ㄨ瘎浼版澘杩炴帴鍒板唴鏍哥殑 SPI 鏍稿績瀛愮郴缁熴€?
杩欐槸涓€涓?SPI 涓绘帶鍒跺櫒锛坢aster controller锛夐┍鍔ㄣ€傚畠鍙互涓庯紙浣滀负涓嬪眰锛塋M70 閫昏緫椹卞姩锛堜竴涓€淪PI 鍗忚椹卞姩鈥濓級閰嶅悎浣跨敤銆傚疄闄呬笂锛岃椹卞姩灏嗚瘎浼版澘涓婄殑骞惰绔彛鎺ュ彛杞崲涓轰竴鏉″甫鍗曚釜璁惧鐨?SPI 鎬荤嚎锛岃璁惧灏嗙敱閫氱敤 LM70 椹卞姩锛坉rivers/hwmon/lm70.c锛夐┍鍔ㄣ€?
### 纭欢鎺ュ彛

姝ょ壒瀹氭澘鍗★紙LM70EVAL-LLP锛夌殑鍘熺悊鍥撅紙绗?4 椤碉級鍙湪姝ゅ鑾峰彇锛?
  https://download.datasheets.com/pdfs/documentation/nat/kit&board/lm70llpevalmanual.pdf

LM70 LLP 璇勪及鏉夸笂鐨勭‖浠舵帴鍙ｅ涓嬶細

   ======== == =========   ==========
   骞惰绔彛              LM70 LLP
     绔彛    .  鏂瑰悜      JP2 鎺掗拡
   ======== == =========   ==========
      D0     2      -         -
      D1     3     -->      V+   5
      D2     4     -->      V+   5
      D3     5     -->      V+   5
      D4     6     -->      V+   5
      D5     7     -->      nCS  8
      D6     8     -->      SCLK 3
      D7     9     -->      SI/O 5
     GND    25      -       GND  7
   Select  13     <--      SI/O 1
   ======== == =========   ==========

娉ㄦ剰锛岀敱浜?LM70 浣跨敤 SPI 鐨勨€? 绾库€濆彉浣擄紝SI/SO 寮曡剼閫氳繃涓€绉嶈骞跺彛鎴?LM70 浠讳竴鎷変綆璇ュ紩鑴氱殑鎺ユ硶锛屽悓鏃惰繛鎺ュ埌寮曡剼 D7锛堜綔涓轰富鍑?Master Out锛夊拰 Select锛堜綔涓轰富鍏?Master In锛夈€傝繖涓嶈兘涓庢櫘閫?SPI 璁惧鍏变韩锛屼絾鍏朵粬 3 绾胯澶囧彲鑳藉叡浜悓涓€涓?SI/SO 寮曡剼銆?
璇ラ┍鍔ㄤ腑鐨?bitbanger 渚嬬▼锛坙m70_txrx锛夌敱鍏剁粦瀹氱殑鈥渉wmon/lm70鈥濆崗璁┍鍔ㄩ€氳繃 sysfs 閽╁瓙锛屼娇鐢?spi_write_then_read() 璋冪敤鍥炶皟銆傚畠鎵ц Mode 0锛圫PI/Microwire锛変綅鑴夊啿锛坆itbanging锛夈€傜劧鍚?lm70 椹卞姩瑙ｉ噴鎵€寰楃殑鏁板瓧娓╁害鍊煎苟閫氳繃 sysfs 瀵煎嚭銆?
涓€涓€滈櫡闃憋紙gotcha锛夆€濓細National Semiconductor 鐨?LM70 LLP 璇勪及鏉跨數璺師鐞嗗浘鏄剧ず锛屾潵鑷?LM70 鑺墖鐨?SI/O 绾胯繛鎺ュ埌鏅朵綋绠?Q1 鐨勫熀鏋侊紙杩樻湁涓€涓笂鎷夌數闃伙紝浠ュ強涓€涓埌 D7 鐨勯綈绾充簩鏋佺锛夛紱鑰岄泦鐢垫瀬鎺ュ埌 VCC銆?
瑙ｉ噴璇ョ數璺細褰?LM70 SI/O 绾夸负楂樼數骞筹紙鎴栦笁鎬佷笖鏈涓绘満閫氳繃 D7 鎷変綆锛夋椂锛屾櫠浣撶瀵奸€氬苟灏嗛泦鐢垫瀬鍒囨崲涓洪浂锛岃繖鍙嶆槧鍦?DB25 骞跺彛杩炴帴鍣ㄧ殑寮曡剼 13 涓娿€傚彟涓€鏂归潰锛屽綋 SI/O 涓轰綆鐢靛钩锛堢敱 LM70 鎴栦富鏈洪┍鍔級鏃讹紝鏅朵綋绠℃埅姝紝鎺ュ湪鍏堕泦鐢垫瀬涓婄殑鐢靛帇浣滀负楂樼數骞冲弽鏄犲湪寮曡剼 13 涓娿€?
鍥犳锛氳椹卞姩涓殑 getmiso 鍐呰仈渚嬬▼鑰冭檻浜嗚繖涓€浜嬪疄锛屽寮曡剼 13 璇诲彇鐨勫€艰繘琛屽彇鍙嶃€?
### 鑷磋阿


- David Brownell锛屾劅璋㈠叾鍦?SPI 渚ч┍鍔ㄥ紑鍙戜笂鐨勬寚瀵笺€?- Dr.Craig Hollabaugh锛屾劅璋㈠叾锛堟棭鏈燂級鐨勨€滄墜鍔ㄢ€濅綅鑴夊啿椹卞姩鐗堟湰銆?- Nadir Billimoria锛屾劅璋㈠叾鍦ㄨВ閲婄數璺師鐞嗗浘涓婄殑甯姪銆?