## The Linux Kernel API


## Basic C Library Functions


缂栧啓椹卞姩鏃讹紝涓€鑸笉鑳戒娇鐢ㄦ潵鑷?C 搴撶殑渚嬬▼銆傚叾涓竴浜涘嚱鏁拌鍙戠幇鍏锋湁鏅亶鐨勫疄鐢ㄦ€э紝涓嬮潰鍒楀嚭瀹冧滑銆傝繖浜涘嚱鏁扮殑琛屼负鍙兘涓?ANSI 瀹氫箟鐨勭暐鏈変笉鍚岋紝杩欎簺宸紓鍦ㄦ枃涓凡娉ㄦ槑銆?
### String Conversions


   :export:

   :functions: kstrtol kstrtoul

   :export:

   :export:

### String Manipulation


   :internal:

   :export:

   :internal:

   :functions: kstrdup kstrdup_const kstrndup kmemdup kmemdup_nul memdup_user
               vmemdup_user strndup_user memdup_user_nul

## Basic Kernel Library Functions


Linux 鍐呮牳鎻愪緵浜嗘洿澶氬熀纭€鐨勫疄鐢ㄥ嚱鏁般€?
### Bit Operations


   :internal:

   :internal:

   :internal:

### Bitmap Operations


   :doc: bitmap introduction

   :doc: declare bitmap

   :doc: bitmap overview

   :doc: bitmap bitops

   :export:

   :internal:

   :internal:

### Command-line Parsing


   :export:

### Error Pointers


   :internal:

### Sorting


   :export:

   :export:

### Text Searching


   :doc: ts_intro

   :export:

   :functions: textsearch_find textsearch_next \
               textsearch_get_pattern textsearch_get_pattern_len

## CRC and Math Functions in Linux


### Arithmetic Overflow Checking


   :internal:

### CRC Functions


   :export:

   :export:

   :export:

   :export:

   :export:

   :export:

   :export:


### Base 2 log and power Functions


   :internal:

### Integer log and power Functions



   :export:

   :export:

### Division Functions


   :functions: do_div

   :internal:

   :export:

### UUID/GUID


   :export:

## Kernel IPC facilities


### IPC utilities


   :internal:

## FIFO Buffer


### kfifo interface


   :internal:

## relay interface support


Relay 鎺ュ彛鏀寔鏃ㄥ湪涓哄伐鍏峰拰璁炬柦鎻愪緵涓€绉嶉珮鏁堟満鍒讹紝灏嗗ぇ閲忔暟鎹粠鍐呮牳绌洪棿浼犻€佸埌鐢ㄦ埛绌洪棿銆?
### relay interface


   :export:

   :internal:

## Module Support


### Kernel module auto-loading


   :export:

### Module debugging


   :doc: module debugging statistics overview

######## dup_failed_modules - tracks duplicate failed modules


   :doc: dup_failed_modules - tracks duplicate failed modules

######## module statistics debugfs counters


   :doc: module statistics debugfs counters

### Inter Module support


鏇村淇℃伅璇峰弬闃?kernel/module/ 涓嬬殑鏂囦欢銆?
## Hardware Interfaces


### DMA Channels


   :export:

### Resources Management


   :internal:

   :export:

### MTRR Handling


   :export:

## Security Framework


   :internal:

   :export:

## Audit Interfaces


   :export:

   :internal:

   :internal:

## Accounting Framework


   :internal:

## Block Devices


   :export:

   :internal:

   :export:

   :internal:

   :export:

   :export:

   :export:

   :export:

   :internal:

   :internal:

   :export:

   :export:

## Char devices


   :export:

## Clock Framework


鏃堕挓妗嗘灦锛坈lock framework锛夊畾涔変簡缂栫▼鎺ュ彛锛屼互鏀寔瀵圭郴缁熸椂閽熸爲锛坈lock tree锛夌殑杞欢绠＄悊銆傝妗嗘灦骞挎硾鐢ㄤ簬鐗囦笂绯荤粺锛圫OC锛夊钩鍙帮紝浠ユ敮鎸佺數婧愮鐞嗕互鍙婂悇绉嶅彲鑳介渶瑕佽嚜瀹氫箟鏃堕挓棰戠巼鐨勮澶囥€傝娉ㄦ剰锛岃繖浜?鏃堕挓"涓庢椂闂翠繚鎸佹垨瀹炴椂鏃堕挓锛圧TC锛夋棤鍏筹紝鍚庝袱鑰呭悇鏈夌嫭绔嬬殑妗嗘灦銆傝繖浜?`struct clk <clk>` 瀹炰緥鍙敤浜庣鐞嗕緥濡備竴涓?96 MHz 鐨勪俊鍙凤紝璇ヤ俊鍙风敤浜庡皢鏁版嵁浣嶇Щ鍏ュ拰绉诲嚭澶栬鎴栨€荤嚎锛屾垨浠ュ叾浠栨柟寮忚Е鍙戠郴缁熺‖浠朵腑鐨勫悓姝ョ姸鎬佹満杞崲銆?
鐢垫簮绠＄悊閫氳繃鏄惧紡鐨勮蒋浠舵椂閽熼棬鎺э紙software clock gating锛夋潵鏀寔锛氭湭浣跨敤鐨勬椂閽熻绂佺敤锛岃繖鏍风郴缁熷氨涓嶄細娴垂鍔熻€楀幓鏀瑰彉鏈涓诲姩浣跨敤鐨勬櫠浣撶鐘舵€併€傚湪鏌愪簺绯荤粺涓婏紝杩欏彲鑳界敱纭欢鏃堕挓闂ㄦ帶浣滀负鏀拺锛屽嵆鏃堕挓鍦ㄨ蒋浠朵腑鏈绂佺敤鐨勬儏鍐典笅琚棬鎺с€傚凡涓婄數浣嗘湭鎻愪緵鏃堕挓鐨勮姱鐗囬儴鍒嗗彲鑳借兘澶熶繚鐣欏叾鏈€鍚庣姸鎬併€傝繖绉嶄綆鍔熻€楃姸鎬侀€氬父绉颁负**淇濇寔妯″紡锛坮etention mode锛?*銆傝妯″紡浠嶇劧浼氫骇鐢熸硠婕忕數娴侊紝灏ゅ叾鏄湪鏇寸簿缁嗙殑鐢佃矾鍑犱綍灏哄涓嬶紝浣嗗浜?CMOS 鐢佃矾鑰岃█锛屽姛鑰椾富瑕佺敱鏃堕挓椹卞姩鐨勭姸鎬佸彉鍖栨秷鑰椼€?
娉ㄩ噸鍔熻€楃殑椹卞姩浠呭湪鍏剁鐞嗙殑璁惧澶勪簬娲诲姩浣跨敤鐘舵€佹椂鍚敤鍏舵椂閽熴€傛澶栵紝绯荤粺鐫＄湢鐘舵€侀€氬父鏍规嵁鍝簺鏃堕挓鍩熷浜庢椿鍔ㄧ姸鎬佽€屼笉鍚岋細铏界劧"standby"鐘舵€佸彲鑳藉厑璁告潵鑷涓椿鍔ㄥ煙鐨勫敜閱掞紝浣?mem"锛堟寕璧峰埌 RAM锛夌姸鎬佸彲鑳介渶瑕佹洿鍏ㄩ潰鍦板叧闂簮鑷珮閫?PLL 鍜屾尟鑽″櫒鐨勬椂閽燂紝浠庤€岄檺鍒跺彲鑳界殑鍞ら啋浜嬩欢婧愭暟閲忋€傞┍鍔ㄧ殑 suspend 鏂规硶鍙兘闇€瑕佷簡瑙ｇ洰鏍囩潯鐪犵姸鎬佷笂涓庣郴缁熺浉鍏崇殑鏃堕挓绾︽潫銆?
鏌愪簺骞冲彴鏀寔鍙紪绋嬫椂閽熷彂鐢熷櫒銆傝繖浜涘彲琚悇绉嶅閮ㄨ姱鐗囦娇鐢紝渚嬪鍏朵粬 CPU銆佸濯掍綋缂栬В鐮佸櫒锛屼互鍙婂鎺ュ彛鏃堕挓鏈変弗鏍艰姹傜殑璁惧銆?
   :internal:

## Synchronization Primitives


### Read-Copy Update (RCU)




















