
## LoongArch 鐨?IRQ 鑺墖妯″瀷锛堝眰绾х粨鏋勶級


鐩墠锛屽熀浜?LoongArch 鐨勫鐞嗗櫒锛堜緥濡?Loongson-3A5000锛夊彧鑳戒笌 LS7A 鑺墖缁勯厤鍚堜娇鐢ㄣ€侺oongArch 璁＄畻鏈轰腑鐨勪腑鏂姱鐗囧寘鎷?CPUINTC锛圕PU 鏍稿績涓柇鎺у埗鍣級銆丩IOINTC锛堜紶缁?I/O 涓柇鎺у埗鍣級銆丒IOINTC锛堟墿灞?I/O 涓柇鎺у埗鍣級銆丠TVECINTC锛圚yper-Transport 鍚戦噺涓柇鎺у埗鍣級銆丳CH-PIC锛圠S7A 鑺墖缁勪腑鐨勪富涓柇鎺у埗鍣級銆丳CH-LPC锛圠S7A 鑺墖缁勪腑鐨?LPC 涓柇鎺у埗鍣級浠ュ強 PCH-MSI锛圡SI 涓柇鎺у埗鍣級銆?
CPUINTC 鏄瘡涓牳蹇冪殑鎺у埗鍣紙浣嶄簬 CPU 鍐咃級锛孡IOINTC/EIOINTC/HTVECINTC 鏄瘡涓皝瑁呯殑鎺у埗鍣紙浣嶄簬 CPU 鍐咃級锛岃€?PCH-PIC/PCH-LPC/PCH-MSI 鏄綅浜?CPU 涔嬪锛堝嵆鑺墖缁勶級鐨勬帶鍒跺櫒銆傝繖浜涙帶鍒跺櫒锛堟崲瑷€涔嬪嵆 irqchip锛変互灞傜骇鏂瑰紡杩炴帴锛屽叡鏈変袱绉嶅眰绾фā鍨嬶紙浼犵粺妯″瀷鍜屾墿灞曟ā鍨嬶級銆?
## 浼犵粺 IRQ 妯″瀷


鍦ㄨ妯″瀷涓紝IPI锛堝鐞嗗櫒闂翠腑鏂級鍜?CPU 鏈湴瀹氭椂鍣ㄤ腑鏂洿鎺ラ€佸線 CPUINTC锛孋PU UARTS 涓柇閫佸線 LIOINTC锛岃€屾墍鏈夊叾浠栬澶囦腑鏂垯閫佸線 PCH-PIC/PCH-LPC/PCH-MSI锛屽苟鐢?HTVECINTC 姹囨€诲悗閫佸線
```
     +-----+     +---------+     +-------+
     | IPI | --> | CPUINTC | <-- | Timer |
     +-----+     +---------+     +-------+
                      ^
                      |
                 +---------+     +-------+
                 | LIOINTC | <-- | UARTs |
                 +---------+     +-------+
                      ^
                      |
                +-----------+
                | HTVECINTC |
                +-----------+
                 ^         ^
                 |         |
           +---------+ +---------+
           | PCH-PIC | | PCH-MSI |
           +---------+ +---------+
             ^     ^           ^
             |     |           |
     +---------+ +---------+ +---------+
     | PCH-LPC | | Devices | | Devices |
     +---------+ +---------+ +---------+
          ^
          |
     +---------+
     | Devices |
     +---------+
```
## 鎵╁睍 IRQ 妯″瀷


鍦ㄨ妯″瀷涓紝IPI锛堝鐞嗗櫒闂翠腑鏂級鍜?CPU 鏈湴瀹氭椂鍣ㄤ腑鏂洿鎺ラ€佸線 CPUINTC锛孋PU UARTS 涓柇閫佸線 LIOINTC锛岃€屾墍鏈夊叾浠栬澶囦腑鏂垯閫佸線 PCH-PIC/PCH-LPC/PCH-MSI锛屽苟鐢?EIOINTC 姹囨€诲悗閫佸線
```
          +-----+     +---------+     +-------+
          | IPI | --> | CPUINTC | <-- | Timer |
          +-----+     +---------+     +-------+
                       ^       ^
                       |       |
                +---------+ +---------+     +-------+
                | EIOINTC | | LIOINTC | <-- | UARTs |
                +---------+ +---------+     +-------+
                 ^       ^
                 |       |
          +---------+ +---------+
          | PCH-PIC | | PCH-MSI |
          +---------+ +---------+
            ^     ^           ^
            |     |           |
    +---------+ +---------+ +---------+
    | PCH-LPC | | Devices | | Devices |
    +---------+ +---------+ +---------+
         ^
         |
    +---------+
    | Devices |
    +---------+
```
## 铏氭嫙鎵╁睍 IRQ 妯″瀷


鍦ㄨ妯″瀷涓紝IPI锛堝鐞嗗櫒闂翠腑鏂級鍜?CPU 鏈湴瀹氭椂鍣ㄤ腑鏂洿鎺ラ€佸線 CPUINTC锛孋PU UARTS 涓柇閫佸線 PCH-PIC锛岃€屾墍鏈夊叾浠栬澶囦腑鏂垯閫佸線 PCH-PIC/PCH-MSI锛屽苟鐢?V-EIOINTC锛堣櫄鎷?```
       +-----+    +-------------------+     +-------+
       | IPI |--> | CPUINTC(0-255vcpu)| <-- | Timer |
       +-----+    +-------------------+     +-------+
                            ^
                            |
                      +-----------+
                      | V-EIOINTC |
                      +-----------+
                       ^         ^
                       |         |
                +---------+ +---------+
                | PCH-PIC | | PCH-MSI |
                +---------+ +---------+
                  ^      ^          ^
                  |      |          |
           +--------+ +---------+ +---------+
           | UARTs  | | Devices | | Devices |
           +--------+ +---------+ +---------+
```
### 璇存槑

V-EIOINTC锛堣櫄鎷熸墿灞?I/O 涓柇鎺у埗鍣級鏄?EIOINTC 鐨勬墿灞曪紝浠呭湪杩愯浜?KVM hypervisor 鐨?VM 妯″紡涓嬪伐浣溿€傞€氳繃鏍囧噯 EIOINTC锛屼腑鏂渶澶氬彲璺敱鍒?4 涓?vCPU锛岃€屽€熷姪 V-EIOINTC锛屼腑鏂渶澶氬彲璺敱鍒?256 涓櫄鎷?CPU銆?
鍦ㄦ爣鍑?EIOINTC 涓紝涓柇璺敱璁剧疆鍖呭惈涓や釜閮ㄥ垎锛? 浣嶇敤浜?CPU 閫夋嫨锛? 浣嶇敤浜?CPU IP锛堜腑鏂紩鑴氾級閫夋嫨銆侰PU 閫夋嫨涓寘鍚?4 浣嶇敤浜?EIOINTC 鑺傜偣閫夋嫨銆? 浣嶇敤浜?EIOINTC CPU 閫夋嫨銆侰PU 閫夋嫨鍜?CPU IP 閫夋嫨鍧囬噰鐢ㄤ綅鍥炬柟娉曪紝鍥犳鍦ㄤ竴涓?EIOINTC 鑺傜偣涓紝涓柇鍙兘璺敱鍒?CPU0鈥揅PU3 浠ュ強 IP0鈥揑P3銆?
鍊熷姪 V-EIOINTC锛屽彲璺敱鍒版洿澶?CPU 浠ュ強 CPU IP锛堜腑鏂紩鑴氾級锛孷-EIOINTC 鏂板浜嗕袱涓瘎瀛樺櫒銆?
### EXTIOI_VIRT_FEATURES

璇ュ瘎瀛樺櫒涓哄彧璇诲瘎瀛樺櫒锛屾寚绀?V-EIOINTC 鎵€鏀寔鐨勭壒鎬с€傛柊澧炰簡 EXTIOI_HAS_INT_ENCODE 鍜?EXTIOI_HAS_CPU_ENCODE 鐗规€с€?
EXTIOI_HAS_INT_ENCODE 灞炰簬鏍囧噯 EIOINTC 鐨勪竴閮ㄥ垎銆傝嫢鍏朵负 1锛岃〃绀?CPU 涓柇寮曡剼閫夋嫨鍙噰鐢ㄥ父瑙勬柟娉曡€岄潪浣嶅浘鏂规硶锛屽洜姝や腑鏂彲璺敱鍒?IP0鈥揑P15銆?
EXTIOI_HAS_CPU_ENCODE 鏄?V-EIOINTC 鐨勬墿灞曘€傝嫢鍏朵负 1锛岃〃绀?CPU 閫夋嫨鍙噰鐢ㄥ父瑙勬柟娉曡€岄潪浣嶅浘鏂规硶锛屽洜姝や腑鏂彲璺敱鍒?CPU0鈥揅PU255銆?
### EXTIOI_VIRT_CONFIG

璇ュ瘎瀛樺櫒涓鸿鍐欏瘎瀛樺櫒锛屼负鍏煎璧疯锛屼腑鏂矾鐢遍粯璁ら噰鐢ㄤ笌鏍囧噯 EIOINTC 鐩稿悓鐨勬柟娉曘€傝嫢灏嗚浣嶈涓?1锛屽垯鎸囩ず纭欢浣跨敤甯歌鏂规硶鑰岄潪浣嶅浘鏂规硶銆?
## 楂樼骇鎵╁睍 IRQ 妯″瀷


鍦ㄨ妯″瀷涓紝IPI锛堝鐞嗗櫒闂翠腑鏂級鍜?CPU 鏈湴瀹氭椂鍣ㄤ腑鏂洿鎺ラ€佸線 CPUINTC锛孋PU UARTS 涓柇閫佸線 LIOINTC锛孭CH-MSI 涓柇閫佸線 AVECINTC锛岀劧鍚庣洿鎺ラ€佸線 CPUINTC锛岃€屾墍鏈夊叾浠栬澶囦腑鏂?```
 +-----+     +-----------------------+     +-------+
 | IPI | --> |        CPUINTC        | <-- | Timer |
 +-----+     +-----------------------+     +-------+
              ^          ^          ^
              |          |          |
       +---------+ +----------+ +---------+     +-------+
       | EIOINTC | | AVECINTC | | LIOINTC | <-- | UARTs |
       +---------+ +----------+ +---------+     +-------+
            ^            ^
            |            |
       +---------+  +---------+
       | PCH-PIC |  | PCH-MSI |
       +---------+  +---------+
         ^     ^           ^
         |     |           |
 +---------+ +---------+ +---------+
 | Devices | | PCH-LPC | | Devices |
 +---------+ +---------+ +---------+
                  ^
                  |
             +---------+
             | Devices |
             +---------+
```
## ACPI 鐩稿叧瀹氫箟


```
  ACPI_MADT_TYPE_CORE_PIC;
  struct acpi_madt_core_pic;
  enum acpi_madt_core_pic_version;
```
```
  ACPI_MADT_TYPE_LIO_PIC;
  struct acpi_madt_lio_pic;
  enum acpi_madt_lio_pic_version;
```
```
  ACPI_MADT_TYPE_EIO_PIC;
  struct acpi_madt_eio_pic;
  enum acpi_madt_eio_pic_version;
```
```
  ACPI_MADT_TYPE_HT_PIC;
  struct acpi_madt_ht_pic;
  enum acpi_madt_ht_pic_version;
```
```
  ACPI_MADT_TYPE_BIO_PIC;
  struct acpi_madt_bio_pic;
  enum acpi_madt_bio_pic_version;
```
```
  ACPI_MADT_TYPE_MSI_PIC;
  struct acpi_madt_msi_pic;
  enum acpi_madt_msi_pic_version;
```
```
  ACPI_MADT_TYPE_LPC_PIC;
  struct acpi_madt_lpc_pic;
  enum acpi_madt_lpc_pic_version;
```
## 鍙傝€冭祫鏂?

Loongson-3A5000 鏂囨。锛?
  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-3A5000-usermanual-1.02-CN.pdf 锛堜腑鏂囷級

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-3A5000-usermanual-1.02-EN.pdf 锛堣嫳鏂囷級

Loongson LS7A 鑺墖缁勬枃妗ｏ細

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-7A1000-usermanual-2.00-CN.pdf 锛堜腑鏂囷級

  https://github.com/loongson/LoongArch-Documentation/releases/latest/download/Loongson-7A1000-usermanual-2.00-EN.pdf 锛堣嫳鏂囷級

    - CPUINTC 鏄?CSR.ECFG/CSR.ESTAT 鍙婂叾涓柇鎺у埗鍣紝鎻忚堪浜庛€奓oongArch 鍙傝€冩墜鍐岀 1 鍗枫€嬬 7.4 鑺傦紱
    - LIOINTC 鏄€奓oongson 3A5000 澶勭悊鍣ㄥ弬鑰冩墜鍐屻€嬬 11.1 鑺傛弿杩扮殑鈥滀紶缁?I/O 涓柇鈥濓紱
    - EIOINTC 鏄€奓oongson 3A5000 澶勭悊鍣ㄥ弬鑰冩墜鍐屻€嬬 11.2 鑺傛弿杩扮殑鈥滄墿灞?I/O 涓柇鈥濓紱
    - HTVECINTC 鏄€奓oongson 3A5000 澶勭悊鍣ㄥ弬鑰冩墜鍐屻€嬬 14.3 鑺傛弿杩扮殑鈥淗yperTransport 涓柇鈥濓紱
    - PCH-PIC/PCH-MSI 鏄€奓oongson 7A1000 妗ユ帴鍣ㄧ敤鎴锋墜鍐屻€嬬 5 鑺傛弿杩扮殑鈥滀腑鏂帶鍒跺櫒鈥濓紱
    - PCH-LPC 鏄€奓oongson 7A1000 妗ユ帴鍣ㄧ敤鎴锋墜鍐屻€嬬 24.3 鑺傛弿杩扮殑鈥淟PC 涓柇鈥濄€?