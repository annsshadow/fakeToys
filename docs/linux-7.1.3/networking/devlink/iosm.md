
## iosm devlink 鏀寔


鏈枃妗ｆ弿杩扮敱 `iosm` 璁惧椹卞姩瀹炵幇鐨?devlink 鐗规€с€?
## 鍙傛暟


`iosm` 椹卞姩瀹炵幇浜嗕互涓嬮┍鍔ㄧ壒瀹氱殑鍙傛暟銆?
   :widths: 5 5 5 85

   - - 鍚嶇О
     - 绫诲瀷
     - 妯″紡
     - 鎻忚堪
   - - `erase_full_flash`
     - u8
     - runtime
     - erase_full_flash 鍙傛暟鐢ㄤ簬妫€鏌ュ湪鍥轰欢鍒峰啓鏈熼棿璁惧鏄惁闇€瑕佸畬鍏ㄦ摝闄ゃ€?       濡傛灉璁剧疆锛屽皢鍚戣澶囧彂閫佸畬鏁寸殑 nand 鎿﹂櫎鍛戒护銆傞粯璁ゆ儏鍐典笅锛?       浠呭惎鐢ㄦ潯浠舵摝闄ゆ敮鎸併€?
## 闂瓨鏇存柊锛團lash Update锛?

`iosm` 椹卞姩瀹炵幇浜嗕娇鐢?`devlink-flash` 鎺ュ彛杩涜闂瓨鏇存柊鐨勬敮鎸併€?
瀹冩敮鎸佷娇鐢ㄥ寘鍚?Bootloader 闀滃儚鍜屽叾浠栬皟鍒惰В璋冨櫒杞欢闀滃儚鐨勭粍鍚堥棯瀛橀暅鍍?鏉ユ洿鏂拌澶囬棯瀛樸€?
椹卞姩浣跨敤 DEVLINK_SUPPORT_FLASH_UPDATE_COMPONENT 鏉ヨ瘑鍒渶瑕佺敱鐢ㄦ埛绌洪棿搴旂敤绋嬪簭
璇锋眰鐨勯棯瀛樺埛鍐欑被鍨嬨€傛敮鎸佺殑鍥轰欢闀滃儚绫诲瀷锛?
    :widths: 15 85

    - - 鍚嶇О
      - 鎻忚堪
    - - `PSI RAM`
      - Primary Signed Image锛堜富绛惧悕闀滃儚锛?    - - `EBL`
      - External Bootloader锛堝閮ㄥ紩瀵煎姞杞界▼搴忥級
    - - `FLS`
      - Modem Software Image锛堣皟鍒惰В璋冨櫒杞欢闀滃儚锛?
PSI RAM 鍜?EBL 鏄?RAM 闀滃儚锛屽綋璁惧澶勪簬 BOOT ROM 闃舵鏃惰娉ㄥ叆鍒拌澶囥€備竴鏃︽垚鍔燂紝
瀹為檯鐨勮皟鍒惰В璋冨櫒鍥轰欢闀滃儚灏嗚鍒峰啓鍒拌澶囥€傝皟鍒惰В璋冨櫒杞欢闀滃儚鍖呭惈澶氫釜鏂囦欢锛?姣忎釜鏂囦欢鏈変竴涓畨鍏?bin 鏂囦欢浠ュ強鑷冲皯涓€涓?Loadmap/Region 鏂囦欢銆備负浜嗗埛鍐欒繖浜?鏂囦欢锛岄渶瑕佸悜璋冨埗瑙ｈ皟鍣ㄨ澶囧彂閫侀€傚綋鐨勫懡浠や互鍙婂埛鍐欐墍闇€鐨勬暟鎹€傝濡傚尯鍩熻鏁板拰
姣忎釜鍖哄煙鐨勫湴鍧€杩欐牱鐨勬暟鎹繀椤讳娇鐢?devlink param 鍛戒护浼犻€掔粰椹卞姩銆?
濡傛灉璁惧闇€瑕佸湪鍥轰欢鍒峰啓鍓嶈瀹屽叏鎿﹂櫎锛岀敤鎴峰簲鐢ㄧ▼搴忛渶瑕佷娇鐢?devlink param 鍛戒护
璁剧疆 erase_full_flash 鍙傛暟銆傞粯璁ゆ儏鍐典笅锛屾敮鎸佹潯浠舵摝闄ょ壒鎬с€?
## 闂瓨鍛戒护锛?

1) 褰撹皟鍒惰В璋冨櫒澶勪簬 Boot ROM 闃舵鏃讹紝鐢ㄦ埛鍙互浣跨敤浠ヤ笅鍛戒护閫氳繃 devlink flash
   鍛戒护娉ㄥ叆 PSI RAM 闀滃儚銆?
$ devlink dev flash pci/0000:02:00.0 file <PSI_RAM_File_name>

2) 濡傛灉鐢ㄦ埛鎯宠杩涜瀹屽叏鎿﹂櫎锛岄渶瑕佸彂鍑轰互涓嬪懡浠ゆ潵璁剧疆 erase full flash 鍙傛暟
   锛堜粎鍦ㄩ渶瑕佸畬鍏ㄦ摝闄ゆ椂璁剧疆锛夈€?
$ devlink dev param set pci/0000:02:00.0 name erase_full_flash value true cmode runtime

3) 鍦ㄨ皟鍒惰В璋冨櫒杩涘叆 PSI 闃舵鍚庢敞鍏?EBL銆?
$ devlink dev flash pci/0000:02:00.0 file <EBL_File_name>

4) 涓€鏃?EBL 娉ㄥ叆鎴愬姛锛屽氨浼氳繘琛屽疄闄呯殑鍥轰欢鍒峰啓銆備互涓嬫槸鐢ㄤ簬姣忎釜鍥轰欢闀滃儚鐨勫懡浠?   搴忓垪銆?
a) 鍒峰啓瀹夊叏 bin 鏂囦欢銆?
$ devlink dev flash pci/0000:02:00.0 file <Secure_bin_file_name>

b) 鍒峰啓 Loadmap/Region 鏂囦欢銆?
$ devlink dev flash pci/0000:02:00.0 file <Load_map_file_name>

## 鍖哄煙锛圧egions锛?

`iosm` 椹卞姩鏀寔杞偍锛坉ump锛塩oredump 鏃ュ織銆?
濡傛灉鍥轰欢閬囧埌寮傚父锛岄┍鍔ㄥ皢鑾峰彇涓€涓揩鐓с€備互涓嬪尯鍩熺敤浜庤闂澶囧唴閮ㄦ暟鎹€?
    :widths: 15 85

    - - 鍚嶇О
      - 鎻忚堪
    - - `report.json`
      - 浣滀负璇ュ尯鍩熶竴閮ㄥ垎璁板綍鐨勫紓甯歌鎯呮憳瑕併€?    - - `coredump.fcd`
      - 璇ュ尯鍩熷寘鍚笌璁惧涓彂鐢熺殑寮傚父鐩稿叧鐨勮鎯咃紙RAM 杞偍锛夈€?    - - `cdd.log`
      - 璇ュ尯鍩熷寘鍚笌璋冨埗瑙ｈ皟鍣?CDD 椹卞姩鐩稿叧鐨勬棩蹇椼€?    - - `eeprom.bin`
      - 璇ュ尯鍩熷寘鍚?eeprom 鏃ュ織銆?    - - `bootcore_trace.bin`
      - 璇ュ尯鍩熷寘鍚綋鍓嶅疄渚嬬殑 bootloader 鏃ュ織銆?    - - `bootcore_prev_trace.bin`
      - 璇ュ尯鍩熷寘鍚笂涓€涓疄渚嬬殑 bootloader 鏃ュ織銆?
## 鍖哄煙鍛戒护


$ devlink region show

$ devlink region new pci/0000:02:00.0/report.json

$ devlink region dump pci/0000:02:00.0/report.json snapshot 0

$ devlink region del pci/0000:02:00.0/report.json snapshot 0

$ devlink region new pci/0000:02:00.0/coredump.fcd

$ devlink region dump pci/0000:02:00.0/coredump.fcd snapshot 1

$ devlink region del pci/0000:02:00.0/coredump.fcd snapshot 1

$ devlink region new pci/0000:02:00.0/cdd.log

$ devlink region dump pci/0000:02:00.0/cdd.log snapshot 2

$ devlink region del pci/0000:02:00.0/cdd.log snapshot 2

$ devlink region new pci/0000:02:00.0/eeprom.bin

$ devlink region dump pci/0000:02:00.0/eeprom.bin snapshot 3

$ devlink region del pci/0000:02:00.0/eeprom.bin snapshot 3

$ devlink region new pci/0000:02:00.0/bootcore_trace.bin

$ devlink region dump pci/0000:02:00.0/bootcore_trace.bin snapshot 4

$ devlink region del pci/0000:02:00.0/bootcore_trace.bin snapshot 4

$ devlink region new pci/0000:02:00.0/bootcore_prev_trace.bin

$ devlink region dump pci/0000:02:00.0/bootcore_prev_trace.bin snapshot 5

$ devlink region del pci/0000:02:00.0/bootcore_prev_trace.bin snapshot 5
