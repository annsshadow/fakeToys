
## SSDT Overlays


涓轰簡鏀寔 ACPI 寮€鏀惧紡鐨勭‖浠堕厤缃紙渚嬪寮€鍙戞澘锛夛紝鎴戜滑闇€瑕佷竴绉嶆柟寮忔潵澧炲己鍥轰欢闀滃儚
鎵€鎻愪緵鐨?ACPI 閰嶇疆銆備竴涓父瑙佺殑渚嬪瓙鏄湪寮€鍙戞澘鐨?I2C / SPI 鎬荤嚎涓婅繛鎺ヤ紶鎰熷櫒銆?
铏界劧杩欏彲浠ラ€氳繃鍒涘缓鍐呮牳骞冲彴椹卞姩鎴栫敤鏇存柊鍚庣殑 ACPI 琛ㄩ噸鏂扮紪璇戝浐浠堕暅鍍忔潵瀹炵幇锛屼絾涓?鑰呴兘涓嶅疄鐢細鍓嶈€呬細瀵艰嚧澶ч噺鐗瑰畾浜庢澘鍗＄殑鍐呮牳浠ｇ爜锛岃€屽悗鑰呴渶瑕佽闂€氬父涓嶅叕寮€鎻愪緵鐨?鍥轰欢宸ュ叿銆?
鐢变簬 ACPI 鍦?AML 浠ｇ爜涓敮鎸佸閮ㄥ紩鐢紝涓€绉嶆洿瀹炵敤鐨勫寮哄浐浠?ACPI 閰嶇疆鐨勬柟娉曪紝鏄?鍔ㄦ€佸姞杞藉寘鍚澘鍗＄壒瀹氫俊鎭殑鐢ㄦ埛瀹氫箟 SSDT 琛ㄣ€?
渚嬪锛岃鍦ㄩ€氳繃 LSE 杩炴帴鍣?[^1^] 鏆撮湶鐨?Minnowboard MAX 寮€鍙戞澘鐨?I2C 鎬荤嚎涓婃灇涓?涓€涓?Bosch BMA222E 鍔犻€熷害璁★紝鍙娇鐢?```

    DefinitionBlock ("minnowmax.aml", "SSDT", 1, "Vendor", "Accel", 0x00000003)
    {
        External (\_SB.I2C6, DeviceObj)

        Scope (\_SB.I2C6)
        {
            Device (STAC)
            {
                Name (_HID, "BMA222E")
                Name (RBUF, ResourceTemplate ()
                {
                    I2cSerialBus (0x0018, ControllerInitiated, 0x00061A80,
                                AddressingMode7Bit, "\\_SB.I2C6", 0x00,
                                ResourceConsumer, ,)
                    GpioInt (Edge, ActiveHigh, Exclusive, PullDown, 0x0000,
                            "\\_SB.GPO2", 0x00, ResourceConsumer, , )
                    { // Pin list
                        0
                    }
                })

                Method (_CRS, 0, Serialized)
                {
                    Return (RBUF)
                }
            }
        }
    }

```
```

    $ iasl minnowmax.asl

    Intel ACPI Component Architecture
    ASL Optimizing Compiler version 20140214-64 [Mar 29 2014]
    Copyright (c) 2000 - 2014 Intel Corporation

    ASL Input:     minnomax.asl - 30 lines, 614 bytes, 7 keywords
    AML Output:    minnowmax.aml - 165 bytes, 6 named objects, 1 executable opcodes

```
[^1^] https://www.elinux.org/Minnowboard:MinnowMax#Low_Speed_Expansion_.28Top.29

鐢熸垚鐨?AML 浠ｇ爜闅忓悗鍙敱鍐呮牳浣跨敤浠ヤ笅浠讳竴鏂规硶鍔犺浇銆?
## 浠?initrd 鍔犺浇 ACPI SSDT


璇ラ€夐」鍏佽浠?initrd 鍔犺浇鐢ㄦ埛瀹氫箟鐨?SSDT锛屽湪绯荤粺涓嶆敮鎸?EFI 鎴?EFI 瀛樺偍绌洪棿涓嶈冻鏃?寰堟湁鐢ㄣ€?
瀹冪殑宸ヤ綔鏂瑰紡涓庡熀浜?initrd 鐨?ACPI 琛ㄨ鐩?鍗囩骇绫讳技锛歋SDT 鐨?AML 浠ｇ爜蹇呴』鏀惧湪绗竴
涓湭鍘嬬缉鐨?initrd 涓紝浣嶄簬 "kernel/firmware/acpi" 璺緞涓嬨€傚彲浠ヤ娇鐢ㄥ涓枃浠讹紝杩?灏嗚浆鍖栦负鍔犺浇澶氫釜琛ㄣ€備粎鍏佽 SSDT 鍜?OEM 琛ㄣ€傛洿澶氱粏鑺傝鍙傞槄 initrd_table_override.txt銆?
```

    # 灏嗗師濮?ACPI 琛ㄦ坊鍔犲埌鏈帇缂╃殑 cpio 褰掓。涓€?    # 瀹冧滑蹇呴』鏀惧湪 cpio 褰掓。鍐呯殑 /kernel/firmware/acpi 鐩綍涓嬨€?    # 鏈帇缂╃殑 cpio 褰掓。蹇呴』鏄涓€涓€?    # 鍏跺畠锛堥€氬父鏄帇缂╃殑锛塩pio 褰掓。蹇呴』
    # 鎷兼帴鍦ㄦ湭鍘嬬缉鐨勫綊妗ｄ箣涓娿€?    mkdir -p kernel/firmware/acpi
    cp ssdt.aml kernel/firmware/acpi

    # 鍒涘缓鏈帇缂╃殑 cpio 褰掓。锛屽苟灏嗗師濮?initrd 鎷兼帴鍦ㄥ叾涓婏細
    find kernel | cpio -H newc --create > /boot/instrumented_initrd
    cat /boot/initrd >>/boot/instrumented_initrd

```
## 浠?EFI 鍙橀噺鍔犺浇 ACPI SSDT


褰撳钩鍙版敮鎸?EFI 鏃讹紝杩欐槸棣栭€夋柟娉曪紝鍥犱负瀹冩彁渚涗簡涓€绉嶆寔涔呯殑銆佷笌鎿嶄綔绯荤粺鏃犲叧鐨勬柟寮忔潵
瀛樺偍鐢ㄦ埛瀹氫箟鐨?SSDT銆傜洰鍓嶄篃鏈夊伐浣滄鍦ㄨ繘琛岋紝浠ュ疄鐜扮敤浜庡姞杞界敤鎴峰畾涔?SSDT 鐨?EFI
鏀寔锛屼娇鐢ㄦ湰鏂规硶灏嗕娇鏈潵杞崲鍒?EFI 鍔犺浇鏈哄埗鏇村姞瀹规槗銆傝鍚敤瀹冿紝搴斿皢
CONFIG_EFI_CUSTOM_SSDT_OVERLAYS 閫夋嫨涓?y銆?
涓轰簡浠?EFI 鍙橀噺鍔犺浇 SSDT锛屽彲浠ヤ娇鐢?`"efivar_ssdt=..."` 鍐呮牳鍛戒护琛屽弬鏁帮紙鍚嶇О闄愬埗
涓?16 涓瓧绗︼級銆傝閫夐」鐨勫弬鏁版槸瑕佷娇鐢ㄧ殑鍙橀噺鍚嶃€傚鏋滃瓨鍦ㄥ涓悓鍚嶄絾鍘傚晢 GUID 涓嶅悓鐨?鍙橀噺锛屽畠浠兘灏嗚鍔犺浇銆?
涓轰簡灏?AML 浠ｇ爜瀛樺叆 EFI 鍙橀噺锛屽彲浠ヤ娇鐢?efivarfs 鏂囦欢绯荤粺銆傚畠鍦ㄦ墍鏈夎繎鏈熷彂琛岀増涓?榛樿鍚敤骞舵寕杞戒簬 /sys/firmware/efi/efivars銆?
鍦?/sys/firmware/efi/efivars 涓垱寤轰竴涓柊鏂囦欢灏嗚嚜鍔ㄥ垱寤轰竴涓柊鐨?EFI 鍙橀噺銆傛洿鏂?璇ョ洰褰曚腑鐨勬枃浠跺皢鏇存柊瀵瑰簲鐨?EFI 鍙橀噺銆傝娉ㄦ剰锛屾枃浠跺悕闇€瑕佷互 "Name-GUID" 鐨勭壒娈婃牸寮?鍛藉悕锛屽苟涓旀枃浠剁殑鍓?4 涓瓧鑺傦紙灏忕鏍煎紡锛夎〃绀?EFI 鍙橀噺鐨勫睘鎬э紙鍙傝 include/linux/efi.h
涓殑 EFI_VARIABLE_MASK锛夈€傚啓鍏ユ枃浠朵篃蹇呴』浠ヤ竴娆″啓鎿嶄綔瀹屾垚銆?
渚嬪锛屼綘鍙互浣跨敤浠ヤ笅 bash 鑴氭湰鏉ュ垱寤?鏇存柊涓€涓?EFI
```

    #!/bin/sh -e

    while [ -n "$1" ]; do
            case "$1" in
            "-f") filename="$2"; shift;;
            "-g") guid="$2"; shift;;
            *) name="$1";;
            esac
            shift
    done

    usage()
    {
            echo "Syntax: ${0##*/} -f filename [ -g guid ] name"
            exit 1
    }

    [ -n "$name" -a -f "$filename" ] || usage

    EFIVARFS="/sys/firmware/efi/efivars"

    [ -d "$EFIVARFS" ] || exit 2

    if stat -tf $EFIVARFS | grep -q -v de5e81e4; then
            mount -t efivarfs none $EFIVARFS
    fi

    # 灏濊瘯鎷惧彇涓€涓凡鏈夌殑 GUID
    [ -n "$guid" ] || guid=$(find "$EFIVARFS" -name "$name-*" | head -n1 | cut -f2- -d-)

    # 浣跨敤涓€涓殢鏈虹敓鎴愮殑 GUID
    [ -n "$guid" ] || guid="$(cat /proc/sys/kernel/random/uuid)"

    # efivarfs 鏈熸湜鎵€鏈夋暟鎹湪涓€娆″啓鍏ヤ腑瀹屾垚
    tmp=$(mktemp)
    /bin/echo -ne "\007\000\000\000" | cat - $filename > $tmp
    dd if=$tmp of="$EFIVARFS/$name-$guid" bs=$(stat -c %s $tmp)
    rm $tmp

```
## 浠?configfs 鍔犺浇 ACPI SSDT


璇ラ€夐」鍏佽閫氳繃 configfs 鎺ュ彛浠庣敤鎴风┖闂村姞杞界敤鎴峰畾涔夌殑 SSDT銆傚繀椤婚€夋嫨 CONFIG_ACPI_CONFIGFS
閫夐」锛屽苟涓?configfs 蹇呴』宸叉寕杞姐€傚湪浠ヤ笅绀轰緥涓紝鎴戜滑鍋囪 configfs 宸叉寕杞戒簬 /sys/kernel/config銆?
鍙互閫氳繃鍦?/sys/kernel/config/acpi/table 涓垱寤烘柊鐩綍鏉ュ姞杞芥柊琛?```

    cd /sys/kernel/config/acpi/table
    mkdir my_ssdt
    cat ~/ssdt.aml > my_ssdt/aml

```
