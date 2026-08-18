
## ACPI CA 璋冭瘯杈撳嚭


ACPI CA 鑳藉鐢熸垚璋冭瘯杈撳嚭銆傛湰鏂囨。浠嬬粛濡備綍浣跨敤璇ユ満鍒躲€?
## 缂栬瘧鏃堕厤缃?

ACPI CA 鐨勮皟璇曡緭鍑虹敱 `CONFIG_ACPI_DEBUG` 鍏ㄥ眬寮€鍚€傝嫢鏈缃閰嶇疆閫夐」锛岃皟璇曟秷鎭敋鑷充笉浼氳缂栬瘧杩涘唴鏍搞€?
## 鍚姩涓庤繍琛屾椂閰嶇疆


褰?`CONFIG_ACPI_DEBUG=y` 鏃讹紝浣犲彲浠ラ€夋嫨鎰熷叴瓒ｇ殑缁勪欢涓庢秷鎭骇鍒€傚湪鍚姩闃舵锛屼娇鐢?`acpi.debug_layer` 鍜?`acpi.debug_level` 鍐呮牳鍛戒护琛岄€夐」銆傚惎鍔ㄤ箣鍚庯紝鍙互浣跨敤 `/sys/module/acpi/parameters/` 涓嬬殑 `debug_layer` 涓?`debug_level` 鏂囦欢鏉ユ帶鍒惰皟璇曟秷鎭€?
## debug_layer锛堢粍浠讹級


`debug_layer` 鏄竴涓敤浜庨€夋嫨鎰熷叴瓒ｇ粍浠剁殑鎺╃爜锛屼緥濡?ACPI 瑙ｉ噴鍣ㄧ殑鏌愪釜鐗瑰畾閮ㄥ垎銆傝鏋勯€?`debug_layer` 浣嶆帺鐮侊紝璇峰湪 ACPI 婧愭枃浠朵腑鏌ユ壘 `#define _COMPONENT`銆?
浣犲彲浠ュ湪鍚姩闃舵浣跨敤 `acpi.debug_layer` 鍛戒护琛屽弬鏁拌缃?`debug_layer` 鎺╃爜锛屽惎鍔ㄤ箣鍚庝篃鍙互閫氳繃鍚?`/sys/module/acpi/parameters/debug_layer` 鍐欏叆鏁板€兼潵鏇存敼瀹冦€?
鍙兘鐨勭粍浠跺畾涔夊湪 `include/acpi/acoutput.h` 涓€?
```

    ACPI_UTILITIES                  0x00000001
    ACPI_HARDWARE                   0x00000002
    ACPI_EVENTS                     0x00000004
    ACPI_TABLES                     0x00000008
    ACPI_NAMESPACE                  0x00000010
    ACPI_PARSER                     0x00000020
    ACPI_DISPATCHER                 0x00000040
    ACPI_EXECUTER                   0x00000080
    ACPI_RESOURCES                  0x00000100
    ACPI_CA_DEBUGGER                0x00000200
    ACPI_OS_SERVICES                0x00000400
    ACPI_CA_DISASSEMBLER            0x00000800
    ACPI_COMPILER                   0x00001000
    ACPI_TOOLS                      0x00002000

```
## debug_level


`debug_level` 鏄竴涓敤浜庨€夋嫨涓嶅悓绫诲瀷娑堟伅鐨勬帺鐮侊紝渚嬪涓庡垵濮嬪寲銆佹柟娉曟墽琛屻€佷俊鎭€ф秷鎭瓑鐩稿叧鐨勬秷鎭€傝鏋勯€?`debug_level`锛岃鏌ョ湅 `ACPI_DEBUG_PRINT()` 璇彞涓寚瀹氱殑绾у埆銆?
ACPI 瑙ｉ噴鍣ㄤ娇鐢ㄥ涓笉鍚岀殑绾у埆锛屼絾 Linux 鐨?ACPI 鏍稿績涓?ACPI 椹卞姩閫氬父鍙娇鐢?`ACPI_LV_INFO`銆?
浣犲彲浠ュ湪鍚姩闃舵浣跨敤 `acpi.debug_level` 鍛戒护琛屽弬鏁拌缃?`debug_level` 鎺╃爜锛屽惎鍔ㄤ箣鍚庝篃鍙互閫氳繃鍚?`/sys/module/acpi/parameters/debug_level` 鍐欏叆鏁板€兼潵鏇存敼瀹冦€?
鍙兘鐨勭骇鍒畾涔夊湪 `include/acpi/acoutput.h` 涓€傝鍙?`/sys/module/acpi/parameters/debug_level` 浼氭樉绀烘敮鎸佺殑鎺╃爜鍊硷紝

```

    ACPI_LV_INIT                    0x00000001
    ACPI_LV_DEBUG_OBJECT            0x00000002
    ACPI_LV_INFO                    0x00000004
    ACPI_LV_INIT_NAMES              0x00000020
    ACPI_LV_PARSE                   0x00000040
    ACPI_LV_LOAD                    0x00000080
    ACPI_LV_DISPATCH                0x00000100
    ACPI_LV_EXEC                    0x00000200
    ACPI_LV_NAMES                   0x00000400
    ACPI_LV_OPREGION                0x00000800
    ACPI_LV_BFIELD                  0x00001000
    ACPI_LV_TABLES                  0x00002000
    ACPI_LV_VALUES                  0x00004000
    ACPI_LV_OBJECTS                 0x00008000
    ACPI_LV_RESOURCES               0x00010000
    ACPI_LV_USER_REQUESTS           0x00020000
    ACPI_LV_PACKAGE                 0x00040000
    ACPI_LV_ALLOCATIONS             0x00100000
    ACPI_LV_FUNCTIONS               0x00200000
    ACPI_LV_OPTIMIZATIONS           0x00400000
    ACPI_LV_MUTEX                   0x01000000
    ACPI_LV_THREADS                 0x02000000
    ACPI_LV_IO                      0x04000000
    ACPI_LV_INTERRUPTS              0x08000000
    ACPI_LV_AML_DISASSEMBLE         0x10000000
    ACPI_LV_VERBOSE_INFO            0x20000000
    ACPI_LV_FULL_TABLES             0x40000000
    ACPI_LV_EVENTS                  0x80000000

```
## 绀轰緥


```

    #define _COMPONENT          ACPI_EVENTS
    ...
    ACPI_DEBUG_PRINT((ACPI_DB_INIT, "ACPI mode disabled\n"));

```
瑕佹墦寮€璇ユ秷鎭紝璇峰湪 `acpi.debug_layer` 涓缃?`ACPI_EVENTS` 浣嶏紝骞跺湪 `acpi.debug_level` 涓缃?`ACPI_LV_INIT` 浣嶃€傦紙`ACPI_DEBUG_PRINT` 璇彞浣跨敤 `ACPI_DB_INIT`锛屽畠鏄熀浜?`ACPI_LV_INIT` 瀹氫箟鐨勪竴涓畯銆傦級

寮€鍚墍鏈?AML "Debug" 杈撳嚭锛堝湪瑙ｉ噴杩囩▼涓瓨鍌ㄥ埌 Debug 瀵硅薄锛夛紝

```

    acpi.debug_layer=0xffffffff acpi.debug_level=0x2

```

```

    acpi.debug_layer=0x2 acpi.debug_level=0xffffffff

```

```

    # echo 0x4 > /sys/module/acpi/parameters/debug_level

```

```

    # cat /sys/module/acpi/parameters/debug_layer

```
