
## Chrome OS ACPI 璁惧


Chrome OS 鐗规湁鐨勭‖浠跺姛鑳介€氳繃 Chrome OS ACPI 璁惧鏆撮湶鍑烘潵銆侰hrome OS ACPI 璁惧鐨?鍗虫彃鍗崇敤 ID 涓?GGL0001锛岀‖浠?ID 涓?GOOG0016銆傛敮鎸佷互涓?ACPI 瀵硅薄锛?
   :widths: 1 2
   :header-rows: 1

   - - Object
     - 鎻忚堪

   - - CHSW
     - Chrome OS 寮€鍏充綅缃?
   - - HWID
     - Chrome OS 纭欢 ID

   - - FWID
     - Chrome OS 鍥轰欢鐗堟湰

   - - FRID
     - Chrome OS 鍙鍥轰欢鐗堟湰

   - - BINF
     - Chrome OS 鍚姩淇℃伅

   - - GPIO
     - Chrome OS GPIO 鍒嗛厤

   - - VBNV
     - Chrome OS NVRAM 浣嶇疆

   - - VDTA
     - Chrome OS 宸查獙璇佸惎鍔ㄦ暟鎹?
   - - FMAP
     - Chrome OS flashmap 鍩哄湴鍧€

   - - MLST
     - Chrome OS 鏂规硶鍒楄〃

## CHSW锛圕hrome OS 寮€鍏充綅缃級

姝ゆ帶鍒舵柟娉曡繑鍥?Chrome OS 鐗瑰畾纭欢寮€鍏崇殑寮€鍏充綅缃€?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓寘鍚互浣嶅煙褰㈠紡琛ㄧず鐨勫紑鍏充綅缃殑鏁存暟锛?
   :widths: 1 2

   - - 0x00000002
     - x86 鍥轰欢鍚姩鏃舵寜涓嬩簡鎭㈠鎸夐挳銆?
   - - 0x00000004
     - EC 鍥轰欢鍚姩鏃舵寜涓嬩簡鎭㈠鎸夐挳銆傦紙濡傛灉 EC EEPROM 鍙噸鍐欏垯涓哄繀濉紱鍚﹀垯鍙€夛級

   - - 0x00000020
     - x86 鍥轰欢鍚姩鏃跺惎鐢ㄤ簡寮€鍙戣€呭紑鍏炽€?
   - - 0x00000200
     - x86 鍥轰欢鍚姩鏃剁鐢ㄤ簡鍥轰欢鍐欎繚鎶ゃ€傦紙濡傛灉鍥轰欢鍐欎繚鎶ょ敱 x86 BIOS 鎺у埗鍒欎负蹇呭～锛?       鍚﹀垯鍙€夛級

鎵€鏈夊叾浠栦綅鍧囦繚鐣欙紝搴旇涓?0銆?
## HWID锛圕hrome OS 纭欢 ID锛?
姝ゆ帶鍒舵柟娉曡繑鍥?Chromebook 鐨勭‖浠?ID銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓互 null 缁撳熬鐨?ASCII 瀛楃涓诧紝鍖呭惈鏉ヨ嚜 EEPROM 鐨勫瀷鍙风壒瀹氭暟鎹紙Model-Specific Data锛?鍖哄煙鐨勭‖浠?ID銆?
娉ㄦ剰纭欢 ID 鏈€闀垮彲杈?256 涓瓧绗︼紝鍖呭惈缁撳熬鐨?null銆?
## FWID锛圕hrome OS 鍥轰欢鐗堟湰锛?
姝ゆ帶鍒舵柟娉曡繑鍥炰富澶勭悊鍣ㄥ浐浠跺彲閲嶅啓閮ㄥ垎鐨勫浐浠剁増鏈€?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓互 null 缁撳熬鐨?ASCII 瀛楃涓诧紝鍖呭惈涓诲鐞嗗櫒鍥轰欢鍙噸鍐欓儴鍒嗙殑瀹屾暣鍥轰欢鐗堟湰銆?
## FRID锛圕hrome OS 鍙鍥轰欢鐗堟湰锛?
姝ゆ帶鍒舵柟娉曡繑鍥炰富澶勭悊鍣ㄥ浐浠跺彧璇婚儴鍒嗙殑鍥轰欢鐗堟湰銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓互 null 缁撳熬鐨?ASCII 瀛楃涓诧紝鍖呭惈涓诲鐞嗗櫒鍥轰欢鍙锛堝紩瀵?+ 鎭㈠锛夐儴鍒嗙殑瀹屾暣鍥轰欢鐗堟湰銆?
## BINF锛圕hrome OS 鍚姩淇℃伅锛?
姝ゆ帶鍒舵柟娉曡繑鍥炲叧浜庡綋鍓嶅惎鍔ㄧ殑淇℃伅銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細



   Package {
           Reserved1
           Reserved2
           Active EC Firmware
           Active Main Firmware Type
           Reserved5
   }

   :widths: 1 1 2
   :header-rows: 1

   - - Field
     - 鏍煎紡
     - 鎻忚堪

   - - Reserved1
     - DWORD
     - 璁句负 256锛?x100锛夈€傝〃绀鸿瀛楁宸蹭笉鍐嶄娇鐢ㄣ€?
   - - Reserved2
     - DWORD
     - 璁句负 256锛?x100锛夈€傝〃绀鸿瀛楁宸蹭笉鍐嶄娇鐢ㄣ€?
   - - Active EC firmware
     - DWORD
     - 鍚姩鏃朵娇鐢ㄧ殑 EC 鍥轰欢銆?
       - 0 - 鍙锛堟仮澶嶏級鍥轰欢
       - 1 - 鍙噸鍐欏浐浠躲€?
       濡傛灉 EC 鍥轰欢濮嬬粓涓哄彧璇伙紝鍒欒涓?0銆?
   - - Active Main Firmware Type
     - DWORD
     - 鍚姩鏃朵娇鐢ㄧ殑涓昏鍥轰欢绫诲瀷銆?
       - 0 - 鎭㈠锛圧ecovery锛?       - 1 - 姝ｅ父锛圢ormal锛?       - 2 - 寮€鍙戣€咃紙Developer锛?       - 3 - 缃戠粶鍚姩锛坣etboot锛屼粎宸ュ巶瀹夎锛?
       鍏朵粬鍊间负淇濈暀鍊笺€?
   - - Reserved5
     - DWORD
     - 璁句负 256锛?x100锛夈€傝〃绀鸿瀛楁宸蹭笉鍐嶄娇鐢ㄣ€?
## GPIO锛圕hrome OS GPIO 鍒嗛厤锛?
姝ゆ帶鍒舵柟娉曡繑鍥炲叧浜?Chrome OS 纭欢涓?Chrome OS 鐗瑰畾 GPIO 鍒嗛厤鐨勪俊鎭紝
浠ヤ究鍐呮牳鍙互鐩存帴鎺у埗璇ョ‖浠躲€?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細


        Package {
                Package {
                        // 绗竴涓?GPIO 鍒嗛厤
                        Signal Type        //DWORD
                        Attributes         //DWORD
                        Controller Offset  //DWORD
                        Controller Name    //ASCIIZ
                },
                ...
                Package {
                        // 鏈€鍚庝竴涓?GPIO 鍒嗛厤
                        Signal Type        //DWORD
                        Attributes         //DWORD
                        Controller Offset  //DWORD
                        Controller Name    //ASCIIZ
                }
        }

鍏朵腑 ASCIIZ 琛ㄧず浠?null 缁撳熬鐨?ASCII 瀛楃涓层€?
   :widths: 1 1 2
   :header-rows: 1

   - - Field
     - 鏍煎紡
     - 鎻忚堪

   - - Signal Type
     - DWORD
     - GPIO 淇″彿鐨勭被鍨?
       - 0x00000001 - 鎭㈠鎸夐挳
       - 0x00000002 - 寮€鍙戣€呮ā寮忓紑鍏?       - 0x00000003 - 鍥轰欢鍐欎繚鎶ゅ紑鍏?       - 0x00000100 - 璋冭瘯鎺掗拡 GPIO 0
       - ...
       - 0x000001FF - 璋冭瘯鎺掗拡 GPIO 255

       鍏朵粬鍊间负淇濈暀鍊笺€?
   - - Attributes
     - DWORD
     - 浠ヤ綅鍩熻〃绀虹殑淇″彿灞炴€э細

       - 0x00000001 - 淇″彿涓洪珮鐢靛钩鏈夋晥锛堝浜庢寜閽紝GPIO 鍊间负 1 琛ㄧず鎸夐挳琚寜涓嬶紱
         瀵逛簬寮€鍏筹紝GPIO 鍊间负 1 琛ㄧず寮€鍏冲凡鍚敤锛夈€傚鏋滆浣嶄负 0锛屽垯淇″彿涓轰綆鐢靛钩鏈夋晥銆?         璋冭瘯鎺掗拡 GPIO 璁句负 0銆?
   - - Controller Offset
     - DWORD
     - 鎸囧畾鎺у埗鍣ㄤ笂鐨?GPIO 缂栧彿銆?
   - - Controller Name
     - ASCIIZ
     - GPIO 鎵€灞炴帶鍒跺櫒鐨勫悕绉般€?       鐩墠鏀寔鐨勫懡鍚嶏細
       "NM10" - Intel NM10 鑺墖

## VBNV锛圕hrome OS NVRAM 浣嶇疆锛?
姝ゆ帶鍒舵柟娉曡繑鍥炲叧浜庣敤浜庝笌 BIOS 閫氫俊鐨?NVRAM锛圕MOS锛変綅缃殑淇℃伅銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細


        Package {
                NV Storage Block Offset  //DWORD
                NV Storage Block Size    //DWORD
        }

   :widths: 1 1 2
   :header-rows: 1

   - - Field
     - 鏍煎紡
     - 鎻忚堪

   - - NV Storage Block Offset
     - DWORD
     - 宸查獙璇佸惎鍔ㄩ潪鏄撳け鎬у瓨鍌ㄥ潡鍦?CMOS bank 0 涓殑鍋忕Щ锛屼粠绗竴涓彲鍐?CMOS 瀛楄妭
       寮€濮嬭鏁帮紙鍗?offset=0 鏄揣闅?14 瀛楄妭鏃堕挓鏁版嵁涔嬪悗鐨勫瓧鑺傦級銆?
   - - NV Storage Block Size
     - DWORD
     - 宸查獙璇佸惎鍔ㄩ潪鏄撳け鎬у瓨鍌ㄥ潡鐨勫ぇ灏忥紙瀛楄妭鏁帮級銆?
## FMAP锛圕hrome OS flashmap 鍦板潃锛?
姝ゆ帶鍒舵柟娉曡繑鍥炰富澶勭悊鍣ㄥ浐浠?flashmap 璧峰浣嶇疆鐨勭墿鐞嗗唴瀛樺湴鍧€銆?
### 鍙傛暟锛?
None

### NoneResult code锛?
涓€涓?DWORD锛屽寘鍚富澶勭悊鍣ㄥ浐浠?flashmap 璧峰浣嶇疆鐨勭墿鐞嗗唴瀛樺湴鍧€銆?
## VDTA锛圕hrome OS 宸查獙璇佸惎鍔ㄦ暟鎹級

姝ゆ帶鍒舵柟娉曡繑鍥炲湪鍥轰欢楠岃瘉姝ラ涓庡唴鏍搁獙璇佹楠や箣闂村叡浜殑宸查獙璇佸惎鍔ㄦ暟鎹潡銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓寘鍚凡楠岃瘉鍚姩鏁版嵁鍧楃殑缂撳啿鍖恒€?
## MECK锛堢鐞嗗紩鎿庢牎楠屽拰锛?
姝ゆ帶鍒舵柟娉曡繑鍥炲湪鍚姩鏈熼棿浠庣鐞嗗紩鎿庯紙Management Engine锛夋墿灞曞瘎瀛樺櫒璇诲嚭鐨?SHA-1 鎴?SHA-256 鍝堝笇銆傝鍝堝笇閫氳繃 ACPI 瀵煎嚭锛屼互渚挎搷浣滅郴缁熷彲浠ラ獙璇?ME 鍥轰欢
鏄惁鍙戠敓浜嗗彉鏇淬€傚鏋滀笉瀛樺湪绠＄悊寮曟搸锛屾垨鑰呭浐浠舵棤娉曡鍙栨墿灞曞瘎瀛樺櫒锛?姝ょ紦鍐插尯鍙互涓洪浂銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓寘鍚?ME 鍝堝笇鐨勭紦鍐插尯銆?
## MLST锛圕hrome OS 鏂规硶鍒楄〃锛?
姝ゆ帶鍒舵柟娉曡繑鍥?Chrome OS 纭欢璁惧鏀寔鐨勫叾浠栨帶鍒舵柟娉曠殑鍒楄〃銆?
### 鍙傛暟锛?
None

### 缁撴灉鐮侊細

涓€涓寘锛坧ackage锛夛紝鍖呭惈浠?null 缁撳熬鐨?ASCII 瀛楃涓插垪琛紝姣忎釜瀛楃涓插搴?Chrome OS
纭欢璁惧鏀寔鐨勪竴涓帶鍒舵柟娉曪紝涓嶅寘鎷?MLST 鏂规硶鏈韩銆傚浜庢湰鐗堟湰鐨勮鑼冿紝缁撴灉涓猴細


        Package {
                "CHSW",
                "FWID",
                "HWID",
                "FRID",
                "BINF",
                "GPIO",
                "VBNV",
                "FMAP",
                "VDTA",
                "MECK"
        }
