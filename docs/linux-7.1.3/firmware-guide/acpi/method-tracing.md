
## ACPICA 璺熻釜鏈哄埗锛圱race Facility锛?

:Copyright: |copy| 2015, Intel Corporation
:Author: Lv Zheng <lv.zheng@intel.com>


## 鎽樿

鏈枃妗ｆ弿杩版柟娉曡窡韪紙method tracing锛夋満鍒剁殑鍔熻兘涓庢帴鍙ｃ€?
## 鍔熻兘涓庝娇鐢ㄧず渚?

ACPICA 鎻愪緵浜嗘柟娉曡窡韪兘鍔涖€傜洰鍓嶅熀浜庤鑳藉姏瀹炵幇浜嗕袱涓姛鑳姐€?
### 鏃ュ織缂╁噺鍣紙Log reducer锛?

褰撳惎鐢?CONFIG_ACPI_DEBUG 鏃讹紝ACPICA 瀛愮郴缁熶細杈撳嚭璋冭瘯淇℃伅銆傞€氳繃 ACPI_DEBUG_PRINT() 瀹忚緭鍑虹殑璋冭瘯娑堟伅鍙互鍦ㄤ袱涓眰绾т笂琚缉鍑忊€斺€旀寜缁勪欢灞傜骇锛堢О涓?debug layer锛岄€氳繃 /sys/module/acpi/parameters/debug_layer 閰嶇疆锛夊拰鎸夌被鍨嬪眰绾э紙绉颁负 debug level锛岄€氳繃 /sys/module/acpi/parameters/debug_level 閰嶇疆锛夈€?
浣嗘槸锛屽綋鎶婄壒瀹?layer/level 搴旂敤浜庢帶鍒舵柟娉曟眰鍊兼椂锛岃皟璇曡緭鍑虹殑鏁伴噺浠嶅彲鑳藉ぇ鍒版棤娉曟斁鍏ュ唴鏍告棩蹇楃紦鍐插尯銆傚洜姝や骇鐢熶簡杩欐牱鐨勬€濊矾锛氫粎鍦ㄦ帶鍒舵柟娉曟眰鍊煎紑濮嬫椂鍚敤鐗瑰畾 debug layer/level锛堥€氬父鏇磋缁嗭級鐨勬棩蹇楋紝骞跺湪鎺у埗鏂规硶姹傚€煎仠姝㈡椂绂佺敤璇︾粏鏃ュ織銆?
浠ヤ笅鍛戒护绀轰緥璇存槑浜嗏€滄棩蹇楃缉鍑忓櫒鈥濆姛鑳界殑浣跨敤锛?
a. 褰撴帶鍒舵柟娉曟眰鍊兼椂锛岃繃婊ゆ帀鍖归厤 debug layer/level 鐨勬棩蹇?```

      # cd /sys/module/acpi/parameters
      # echo "0xXXXXXXXX" > trace_debug_layer
      # echo "0xYYYYYYYY" > trace_debug_level
      # echo "enable" > trace_state

```
b. 褰撴寚瀹氱殑鎺у埗鏂规硶姹傚€兼椂锛岃繃婊ゆ帀鍖归厤 debug layer/level 鐨勬棩蹇?```

      # cd /sys/module/acpi/parameters
      # echo "0xXXXXXXXX" > trace_debug_layer
      # echo "0xYYYYYYYY" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method" > /sys/module/acpi/parameters/trace_state

```
c. 褰撴寚瀹氱殑鎺у埗鏂规硶姹傚€兼椂锛岃繃婊ゆ帀鍖归厤 debug layer/level 鐨勬棩蹇楋紙浠呬竴娆★級
```

      # cd /sys/module/acpi/parameters
      # echo "0xXXXXXXXX" > trace_debug_layer
      # echo "0xYYYYYYYY" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method-once" > /sys/module/acpi/parameters/trace_state

```
鍏朵腑锛?   0xXXXXXXXX/0xYYYYYYYY
     鏈夊叧鍙兘鐨?debug layer/level 鎺╃爜鍙栧€硷紝璇峰弬瑙?Documentation/firmware-guide/acpi/debug.rst銆?   \PPPP.AAAA.TTTT.HHHH
     ACPI 鍛藉悕绌洪棿涓煇涓帶鍒舵柟娉曠殑瀹屾暣璺緞銆?     瀹冧笉蹇呮槸鎺у埗鏂规硶姹傚€肩殑鍏ュ彛銆?
### AML 璺熻釜鍣紙AML tracer锛?

鏂规硶璺熻釜鏈哄埗浼氬湪 AML 瑙ｉ噴鍣ㄥ紑濮?鍋滄鎵ц鏌愪釜鎺у埗鏂规硶鎴栨煇涓?AML 鎿嶄綔鐮侊紙opcode锛夌殑鈥滆窡韪偣鈥濆娣诲姞鐗规畩鐨勬棩蹇楁潯鐩€傛敞鎰忚繖浜涙棩蹇楁潯鐩殑鏍煎紡涓?```

   [    0.186427]   exdebug-0398 ex_trace_point        : Method Begin [0xf58394d8:\_SB.PCI0.LPCB.ECOK] execution.
   [    0.186630]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905c88:If] execution.
   [    0.186820]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905cc0:LEqual] execution.
   [    0.187010]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905a20:-NamePath-] execution.
   [    0.187214]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905a20:-NamePath-] execution.
   [    0.187407]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905f60:One] execution.
   [    0.187594]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905f60:One] execution.
   [    0.187789]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905cc0:LEqual] execution.
   [    0.187980]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905cc0:Return] execution.
   [    0.188146]   exdebug-0398 ex_trace_point        : Opcode Begin [0xf5905f60:One] execution.
   [    0.188334]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905f60:One] execution.
   [    0.188524]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905cc0:Return] execution.
   [    0.188712]   exdebug-0398 ex_trace_point        : Opcode End [0xf5905c88:If] execution.
   [    0.188903]   exdebug-0398 ex_trace_point        : Method End [0xf58394d8:\_SB.PCI0.LPCB.ECOK] execution.

```
寮€鍙戣€呭彲浠ュ埄鐢ㄨ繖浜涚壒娈婃棩蹇楁潯鐩潵杩借釜 AML 瑙ｉ噴杩囩▼锛屼粠鑰屾湁鍔╀簬闂璋冭瘯鍜屾€ц兘璋冧紭銆傛敞鎰忥紝鐢变簬鈥淎ML tracer鈥濇棩蹇楁槸閫氳繃 ACPI_DEBUG_PRINT() 瀹忓疄鐜扮殑锛屽惎鐢ㄢ€淎ML tracer鈥濇棩蹇楀悓鏍烽渶瑕佸紑鍚?CONFIG_ACPI_DEBUG銆?
浠ヤ笅鍛戒护绀轰緥璇存槑浜嗏€淎ML tracer鈥濆姛鑳界殑浣跨敤锛?
a. 褰撴帶鍒舵柟娉曞紑濮?鍋滄鏃讹紝杩囨护鍑烘柟娉曞紑濮?鍋滄鐨勨€淎ML tracer鈥濇棩蹇?```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "enable" > trace_state

```
b. 褰撴寚瀹氱殑鎺у埗鏂规硶寮€濮?鍋滄鏃讹紝杩囨护鍑衡€淎ML tracer鈥濇棩蹇?```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method" > trace_state

```
c. 褰撴寚瀹氱殑鎺у埗鏂规硶寮€濮?鍋滄鏃讹紝杩囨护鍑衡€淎ML tracer鈥濇棩蹇楋紙浠呬竴娆★級
```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "method-once" > trace_state

```
d. 褰撴寚瀹氱殑鏂规硶/鎿嶄綔鐮佸紑濮?鍋滄鏃讹紝杩囨护鍑衡€淎ML tracer鈥濇棩蹇?```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "opcode" > trace_state

```
e. 褰撴寚瀹氱殑鏂规硶/鎿嶄綔鐮佸紑濮?鍋滄鏃讹紝杩囨护鍑衡€淎ML tracer鈥濇棩蹇?```

      # cd /sys/module/acpi/parameters
      # echo "0x80" > trace_debug_layer
      # echo "0x10" > trace_debug_level
      # echo "\PPPP.AAAA.TTTT.HHHH" > trace_method_name
      # echo "opcode-opcode" > trace_state

```
娉ㄦ剰锛屼笂杩版墍鏈夋柟娉曡窡韪満鍒剁浉鍏崇殑妯″潡鍙傛暟涔熷彲閫氳繃鍐呮牳鍚姩鍙傛暟鎸囧畾
```

   acpi.trace_debug_layer=0x80 acpi.trace_debug_level=0x10 \
   acpi.trace_method_name=\_SB.LID0._LID acpi.trace_state=opcode-once


```
## 鎺ュ彛鎻忚堪


鎵€鏈夋柟娉曡窡韪姛鑳介兘鍙€氳繃 ACPI 妯″潡鍙傛暟閰嶇疆锛岃繖浜涘弬鏁颁綅浜?/sys/module/acpi/parameters/锛?
trace_method_name
  鐢ㄦ埛鎯宠璺熻釜鐨?AML 鏂规硶鐨勫畬鏁磋矾寰勩€?
  娉ㄦ剰瀹屾暣璺緞鐨勫悕绉版涓笉搴斿寘鍚粨灏剧殑 鈥淿鈥濓紝浣嗗彲浠ュ寘鍚?鈥淺鈥?浠ユ瀯鎴愮粷瀵硅矾寰勩€?
trace_debug_layer
  鍚敤璺熻釜鍔熻兘鏃朵娇鐢ㄧ殑涓存椂 debug_layer銆?
  榛樿浣跨敤 ACPI_EXECUTER (0x80)锛屽嵆鐢ㄤ簬鍖归厤鎵€鏈夆€淎ML tracer鈥濇棩蹇楃殑 debug_layer銆?
trace_debug_level
  鍚敤璺熻釜鍔熻兘鏃朵娇鐢ㄧ殑涓存椂 debug_level銆?
  榛樿浣跨敤 ACPI_LV_TRACE_POINT (0x10)锛屽嵆鐢ㄤ簬鍖归厤鎵€鏈夆€淎ML tracer鈥濇棩蹇楃殑 debug_level銆?
trace_state
  璺熻釜鍔熻兘鐨勭姸鎬併€?
  鐢ㄦ埛鍙€氳繃鎵ц
```

   # echo string > /sys/module/acpi/parameters/trace_state

```
鏉ュ惎鐢?绂佺敤璇ヨ皟璇曡窡韪姛鑳姐€傚叾涓?鈥渟tring鈥?搴斾负浠ヤ笅涔嬩竴锛?
"disable"
  绂佺敤鏂规硶璺熻釜鍔熻兘銆?
"enable"
  鍚敤鏂规硶璺熻釜鍔熻兘銆?
  鍦ㄤ换鎰忔柟娉曟墽琛屾湡闂达紝鍖归厤 鈥渢race_debug_layer/trace_debug_level鈥?鐨?ACPICA 璋冭瘯娑堟伅閮戒細琚褰曘€?
"method"
  鍚敤鏂规硶璺熻釜鍔熻兘銆?
  鍦?鈥渢race_method_name鈥?鐨勬柟娉曟墽琛屾湡闂达紝鍖归厤 鈥渢race_debug_layer/trace_debug_level鈥?鐨?ACPICA 璋冭瘯娑堟伅浼氳璁板綍銆?
"method-once"
  鍚敤鏂规硶璺熻釜鍔熻兘銆?
  鍦?鈥渢race_method_name鈥?鐨勬柟娉曟墽琛屾湡闂达紝鍖归厤 鈥渢race_debug_layer/trace_debug_level鈥?鐨?ACPICA 璋冭瘯娑堟伅浠呬細琚褰曚竴娆°€?
"opcode"
  鍚敤鏂规硶璺熻釜鍔熻兘銆?
  鍦?鈥渢race_method_name鈥?鐨勬柟娉?鎿嶄綔鐮佹墽琛屾湡闂达紝鍖归厤 鈥渢race_debug_layer/trace_debug_level鈥?鐨?ACPICA 璋冭瘯娑堟伅浼氳璁板綍銆?
"opcode-once"
  鍚敤鏂规硶璺熻釜鍔熻兘銆?
  鍦?鈥渢race_method_name鈥?鐨勬柟娉?鎿嶄綔鐮佹墽琛屾湡闂达紝鍖归厤 鈥渢race_debug_layer/trace_debug_level鈥?鐨?ACPICA 璋冭瘯娑堟伅浠呬細琚褰曚竴娆°€?
娉ㄦ剰锛屸€渆nable鈥?涓庡叾浠栧姛鑳藉惎鐢ㄩ€夐」鐨勫尯鍒湪浜庯細

1. 鎸囧畾 鈥渆nable鈥?鏃讹紝鐢变簬 鈥渢race_debug_layer/trace_debug_level鈥?浼氬簲鐢ㄤ簬鎵€鏈夋帶鍒舵柟娉曟眰鍊硷紝鍥犳鍦ㄥ皢 鈥渢race_state鈥?閰嶇疆涓?鈥渆nable鈥?鍚庯紝鈥渢race_method_name鈥?浼氳閲嶇疆涓?NULL銆?2. 鎸囧畾 鈥渕ethod/opcode鈥?鏃讹紝濡傛灉鍦ㄥ皢杩欎簺閫夐」閰嶇疆鍒?鈥渢race_state鈥?鏃?鈥渢race_method_name鈥?涓?NULL锛屽垯 鈥渢race_debug_layer/trace_debug_level鈥?浼氬簲鐢ㄤ簬鎵€鏈夋帶鍒舵柟娉曟眰鍊笺€?