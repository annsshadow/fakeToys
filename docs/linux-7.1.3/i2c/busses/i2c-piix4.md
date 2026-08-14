## 鍐呮牳椹卞姩 i2c-piix4


鏀寔鐨勯€傞厤鍣細
  - Intel 82371AB PIIX4 鍜?PIIX4E
  - Intel 82443MX (440MX)
    Datasheet锛氬湪 Intel 缃戠珯涓婂叕寮€鎻愪緵
  - ServerWorks OSB4銆丆SB5銆丆SB6銆丠T-1000 鍜?HT-1100 鍗楁ˉ
    Datasheet锛氫粎鍙€氳繃涓?ServerWorks 绛剧讲鐨?NDA 鑾峰彇
  - ATI IXP200銆両XP300銆両XP400銆丼B600銆丼B700 鍜?SB800 鍗楁ˉ
    Datasheet锛氭湭鍏紑鎻愪緵
    SB700 瀵勫瓨鍣ㄥ弬鑰冨彲鍦ㄤ互涓嬩綅缃幏鍙栵細
    http://support.amd.com/us/Embedded_TechDocs/43009_sb7xx_rrg_pub_1.00.pdf
  - AMD SP5100锛堣浜庢煇浜涙湇鍔″櫒涓绘澘鐨?SB700 琛嶇敓鍨嬪彿锛?    Datasheet锛氬湪 AMD 缃戠珯涓婂叕寮€鎻愪緵
    http://support.amd.com/us/Embedded_TechDocs/44413.pdf
  - AMD Hudson-2銆丮L銆丆Z
    Datasheet锛氭湭鍏紑鎻愪緵
  - Hygon CZ
    Datasheet锛氭湭鍏紑鎻愪緵
  - Standard Microsystems (SMSC) SLC90E66 (Victory66) 鍗楁ˉ
    Datasheet锛氬湪 SMSC 缃戠珯 http://www.smsc.com 鍏紑鎻愪緵

浣滆€咃細
 - Frodo Looijaard <frodol@dds.nl>
 - Philip Edelbrock <phil@netroedge.com>


### 妯″潡鍙傛暟


- force: int
  寮哄埗鍚敤 PIIX4銆傚緢鍗遍櫓锛?- force_addr: int
  寮哄埗鍦ㄧ粰瀹氱殑鍦板潃涓婂惎鐢?PIIX4銆傛瀬搴﹀嵄闄╋紒

### 鎻忚堪


PIIX4锛堟纭悕绉颁负 82371AB锛夋槸涓€涓姛鑳戒赴瀵岀殑 Intel 鑺墖銆傞櫎鍏跺畠鍔熻兘澶栵紝瀹冭繕瀹炵幇浜?PCI 鎬荤嚎銆傚畠鐨勪竴涓瑕佸姛鑳芥槸瀹炵幇涓€涓郴缁熺鐞嗘€荤嚎锛圫ystem Management Bus锛夈€傝繖鏄竴涓?鐪熸鐨?SMBus鈥斺€斾綘鏃犳硶鍦?I2C 灞傞潰璁块棶瀹冦€傚ソ娑堟伅鏄畠鍘熺敓鐞嗚В SMBus 鍛戒护锛屼綘涓嶅繀鎷呭績
鏃跺簭闂銆傚潖娑堟伅鏄繛鎺ュ埌瀹冪殑闈?SMBus 璁惧鍙兘浼氳瀹冩瀬搴︽贩涔便€傛槸鐨勶紝杩欑‘瀹炰細鍙戠敓鈥︹€?
```

  0000:00:02.3 Bridge: Intel Corp. 82371AB/EB/MB PIIX4 ACPI (rev 02)
	       Flags: medium devsel, IRQ 9

```
鎬荤嚎鍜岃澶囧彿鍙兘涓嶅悓锛屼絾鍔熻兘鍙峰繀椤荤浉鍚岋紙鍍忚澶?PCI 璁惧涓€鏍凤紝PIIX4 鍖呭惈鑻ュ共涓笉鍚?鐨勨€滃姛鑳解€濓紝鍙瑙嗕负鐙珛璁惧锛夈€傚鏋滀綘鎵惧埌杩欐牱鐨勬潯鐩紝浣犲氨鎷ユ湁涓€涓?PIIX4 SMBus 鎺у埗鍣ㄣ€?
鍦ㄦ煇浜涜绠楁満涓婏紙鏈€钁楀悕鐨勬槸鏌愪簺 Dell 鏈哄櫒锛夛紝SMBus 榛樿琚鐢ㄣ€傚鏋滀綘浣跨敤 insmod
鍙傛暟 鈥榝orce=1鈥欙紝鍐呮牳妯″潡灏嗗皾璇曞惎鐢ㄥ畠銆傝繖闈炲父鍗遍櫓锛佸鏋?BIOS 娌℃湁涓烘妯″潡璁剧疆姝ｇ‘鐨?鍦板潃锛屼綘鍙兘浼氶櫡鍏ュぇ楹荤儲锛堣锛氬穿婧冦€佹暟鎹崯鍧忕瓑锛夈€備粎鍦ㄤ竾涓嶅緱宸叉椂鎵嶅皾璇曪紙渚嬪鍏堝皾璇?鏇存柊 BIOS锛夛紝骞朵笖鍏堝仛濂藉浠斤紒涓€涓洿鍗遍櫓鐨勯€夐」鏄?鈥榝orce_addr=<IOPORT>鈥欍€傝繖涓嶄粎浼氬儚
鈥榝orce鈥?閭ｆ牱鍚敤 PIIX4锛岃繕浼氳缃竴涓柊鐨勫熀鍦板潃 I/O 绔彛銆侾IIX4 鐨?SMBus 閮ㄥ垎闇€瑕?杩炵画 8 涓繖鏍风殑鍦板潃鎵嶈兘姝ｅ父宸ヤ綔銆傚鏋滆繖浜涘湴鍧€宸茬粡琚叾瀹冭澶囦繚鐣欙紝浣犲皢浼氶櫡鍏ュぇ楹荤儲锛?濡傛灉浣犱笉闈炲父纭畾鑷繁鍦ㄥ仛浠€涔堬紝涓嶈浣跨敤瀹冿紒

PIIX4E 鍙槸 PIIX4 鐨勪竴涓柊鐗堟湰锛涘畠鍚屾牱鍙楁敮鎸併€侾IIX/PIIX3 娌℃湁瀹炵幇 SMBus 鎴?I2C 鎬荤嚎锛?鍥犳浣犱笉鑳藉湪杩欎簺涓绘澘涓婁娇鐢ㄦ椹卞姩銆?
ServerWorks 鍗楁ˉ銆両ntel 440MX 鍜?Victory66 鍦?I2C/SMBus 鏀寔涓婁笌 PIIX4 瀹屽叏鐩稿悓銆?
AMD SB700銆丼B800銆丼P5100 鍜?Hudson-2 鑺墖缁勫疄鐜颁簡涓や釜涓?PIIX4 鍏煎鐨?SMBus 鎺у埗鍣ㄣ€?濡傛灉浣犵殑 BIOS 鍒濆鍖栦簡杈呭姪鎺у埗鍣紝瀹冨皢琚椹卞姩妫€娴嬩负鈥淎uxiliary SMBus Host Controller鈥?锛堣緟鍔?SMBus 涓绘帶鍒跺櫒锛夈€?
濡傛灉浣犳嫢鏈?Force CPCI735 涓绘澘鎴栧叾瀹冨熀浜?OSB4 鐨勭郴缁燂紝浣犲彲鑳介渶瑕佹洿鏀?SMBus 涓柇閫夋嫨
瀵勫瓨鍣紝浣?SMBus 鎺у埗鍣ㄤ娇鐢?SMI 妯″紡銆?
1) 浣跨敤 `lspci` 鍛戒护骞跺畾浣嶅甫鏈?SMBus 鎺у埗鍣ㄧ殑 PCI 璁惧锛?   00:0f.0 ISA bridge: ServerWorks OSB4 South Bridge (rev 4f)
   涓嶅悓鑺墖缁勭殑杩欎竴琛屽彲鑳芥湁鎵€涓嶅悓銆傝鏌ラ槄椹卞姩婧愮爜浜嗚В鎵€鏈夊彲鑳界殑 PCI id锛堝苟鐢?   `lspci -n` 鏉ュ尮閰嶅畠浠級銆傚亣璁捐璁惧浣嶄簬 00:0f.0銆?2) 鐜板湪浣犲彧闇€鏇存敼 0xD2 瀵勫瓨鍣ㄤ腑鐨勫€笺€傞鍏堢敤浠ヤ笅鍛戒护鑾峰彇瀹冿細
   `lspci -xxx -s 00:0f.0`
   濡傛灉鍊间负 0x3锛屽垯闇€瑕佸皢鍏舵敼涓?0x1锛?   `setpci  -s 00:0f.0 d2.b=1`

璇锋敞鎰忥紝浣犲苟闈炲湪鎵€鏈夋儏鍐典笅閮介渶瑕佽繖鏍峰仛锛屼粎褰?SMBus 宸ヤ綔涓嶆甯告椂銆?
### 纭欢鐩稿叧闂


姝ら┍鍔ㄥ皢鎷掔粷鍦ㄥ甫鏈?Intel PIIX4 SMBus 鐨?IBM 绯荤粺涓婂姞杞姐€傚叾涓竴浜涙満鍣ㄦ湁涓€涓繛鎺ュ埌
SMBus 鐨?RFID EEPROM锛?4RF08锛夛紝鐢变簬鐘舵€佹満缂洪櫡寰堝鏄撹鎹熷潖銆傝繖浜涗富瑕佹槸 Thinkpad 绗旇鏈紝
浣嗗彴寮忔満绯荤粺涔熷彲鑳藉彈褰卞搷銆傛垜浠病鏈夋墍鏈夊彈褰卞搷绯荤粺鐨勫垪琛紝鍥犳鍞竴瀹夊叏鐨勮В鍐虫柟妗堟槸闃绘
璁块棶鎵€鏈?IBM 绯荤粺涓婄殑 SMBus锛堥€氳繃 DMI 鏁版嵁妫€娴嬶級銆?
### ACPI 浠ｇ爜涓殑鎻忚堪


PIIX4 鑺墖鐨勮澶囬┍鍔ㄤ负鍏舵瘡涓?```

    $ i2cdetect -l
    ...
    i2c-7   unknown         SMBus PIIX4 adapter port 0 at 0b00      N/A
    i2c-8   unknown         SMBus PIIX4 adapter port 2 at 0b00      N/A
    i2c-9   unknown         SMBus PIIX4 adapter port 1 at 0b20      N/A
    ...

```
鍥犳锛屽鏋滀綘鎯冲湪 ACPI 浠ｇ爜涓闂叾涓竴涓€荤嚎锛岀鍙?```

    Scope (\_SB_.PCI0.SMBS)
    {
        Name (_ADR, 0x00140000)

        Device (SMB0) {
            Name (_ADR, 0)
        }
        Device (SMB1) {
            Name (_ADR, 1)
        }
        Device (SMB2) {
            Name (_ADR, 2)
        }
    }

```
濡傛灉浣犵殑 UEFI 鍥轰欢骞堕潪濡傛锛屼笖浣犳棤娉曡闂簮浠ｇ爜锛屼綘鍙互浣跨敤 ACPI SSDT Overlays 鏉?鎻愪緵缂哄け鐨勯儴鍒嗐€傚彧闇€璁颁綇锛屽湪杩欑鎯呭喌涓嬩綘闇€瑕佸湪 piix4 椹卞姩鍚姩涔嬪墠鍔犺浇棰濆鐨?SSDT
琛紝鍗充綘搴旇閫氳繃 initrd 鎴?EFI 鍙橀噺鐨勬柟寮忔彁渚?SSDT锛岃€屼笉鏄€氳繃 configfs銆?
浣滀负鐢ㄦ硶绀轰緥锛屼笅闈㈡槸涓€涓皢涓?jc42 鍒嗛厤鐨?ACPI 浠ｇ爜鐗囨浠ｇ爜
```

    Device (JC42) {
        Name (_HID, "PRP0001")
        Name (_DDN, "JC42 Temperature sensor")
        Name (_CRS, ResourceTemplate () {
            I2cSerialBusV2 (
                0x001c,
                ControllerInitiated,
                100000,
                AddressingMode7Bit,
                "\\_SB.PCI0.SMBS.SMB0",
                0
            )
        })

        Name (_DSD, Package () {
            ToUUID("daffd814-6eba-4d8c-8a91-bc9bbf4aa301"),
            Package () {
                Package () { "compatible", Package() { "jedec,jc-42.4-temp" } },
            }
        })
    }

```
