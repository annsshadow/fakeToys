
## SSDT Overlays


为了支持 ACPI 开放式的硬件配置（例如开发板），我们需要一种方式来增强固件镜像
所提供ACPI 配置。一个常见的例子是在开发板I2C / SPI 总线上连接传感器
虽然这可以通过创建内核平台驱动或用更新后的 ACPI 表重新编译固件镜像来实现，但者都不实用：前者会导致大量特定于板卡的内核代码，而后者需要访问通常不公开提供固件工具
由于 ACPI AML 代码中支持外部引用，一种更实用的增强固ACPI 配置的方法，动态加载包含板卡特定信息的用户定义 SSDT 表
例如，要在通过 LSE 连接[^1^] 暴露Minnowboard MAX 开发板I2C 总线上枚一Bosch BMA222E 加速度计，可使```

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

生成AML 代码随后可由内核使用以下任一方法加载
## initrd 加载 ACPI SSDT


该选项允许initrd 加载用户定义SSDT，在系统不支EFI EFI 存储空间不足很有用
它的工作方式与基initrd ACPI 表覆升级类似：SSDT AML 代码必须放在第一
个未压缩initrd 中，位于 "kernel/firmware/acpi" 路径下。可以使用多个文件，将转化为加载多个表。仅允许 SSDT OEM 表。更多细节请参阅 initrd_table_override.txt
```

    # 将原ACPI 表添加到未压缩的 cpio 归档中    # 它们必须放在 cpio 归档内的 /kernel/firmware/acpi 目录下    # 未压缩的 cpio 归档必须是第一个    # 其它（通常是压缩的）cpio 归档必须
    # 拼接在未压缩的归档之上    mkdir -p kernel/firmware/acpi
    cp ssdt.aml kernel/firmware/acpi

    # 创建未压缩的 cpio 归档，并将原initrd 拼接在其上：
    find kernel | cpio -H newc --create > /boot/instrumented_initrd
    cat /boot/initrd >>/boot/instrumented_initrd

```
## EFI 变量加载 ACPI SSDT


当平台支EFI 时，这是首选方法，因为它提供了一种持久的、与操作系统无关的方式来
存储用户定义SSDT。目前也有工作正在进行，以实现用于加载用户定SSDT EFI
支持，使用本方法将使未来转换EFI 加载机制更加容易。要启用它，应将
CONFIG_EFI_CUSTOM_SSDT_OVERLAYS 选择y
为了EFI 变量加载 SSDT，可以使`"efivar_ssdt=..."` 内核命令行参数（名称限制
16 个字符）。该选项的参数是要使用的变量名。如果存在多个同名但厂商 GUID 不同变量，它们都将被加载
为了AML 代码存入 EFI 变量，可以使efivarfs 文件系统。它在所有近期发行版默认启用并挂载于 /sys/firmware/efi/efivars
/sys/firmware/efi/efivars 中创建一个新文件将自动创建一个新EFI 变量。更该目录中的文件将更新对应EFI 变量。请注意，文件名需要以 "Name-GUID" 的特殊格命名，并且文件的4 个字节（小端格式）表EFI 变量的属性（参见 include/linux/efi.h
中的 EFI_VARIABLE_MASK）。写入文件也必须以一次写操作完成
例如，你可以使用以下 bash 脚本来创更新一EFI
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

    # 尝试拾取一个已有的 GUID
    [ -n "$guid" ] || guid=$(find "$EFIVARFS" -name "$name-*" | head -n1 | cut -f2- -d-)

    # 使用一个随机生成的 GUID
    [ -n "$guid" ] || guid="$(cat /proc/sys/kernel/random/uuid)"

    # efivarfs 期望所有数据在一次写入中完成
    tmp=$(mktemp)
    /bin/echo -ne "\007\000\000\000" | cat - $filename > $tmp
    dd if=$tmp of="$EFIVARFS/$name-$guid" bs=$(stat -c %s $tmp)
    rm $tmp

```
## configfs 加载 ACPI SSDT


该选项允许通过 configfs 接口从用户空间加载用户定义的 SSDT。必须选择 CONFIG_ACPI_CONFIGFS
选项，并configfs 必须已挂载。在以下示例中，我们假设 configfs 已挂载于 /sys/kernel/config
可以通过/sys/kernel/config/acpi/table 中创建新目录来加载新```

    cd /sys/kernel/config/acpi/table
    mkdir my_ssdt
    cat ~/ssdt.aml > my_ssdt/aml

```
