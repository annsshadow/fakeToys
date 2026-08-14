
## Chrome OS ACPI 设备


Chrome OS 特有的硬件功能通过 Chrome OS ACPI 设备暴露出来。Chrome OS ACPI 设备的
即插即用 ID 为 GGL0001，硬件 ID 为 GOOG0016。支持以下 ACPI 对象：

   :widths: 1 2
   :header-rows: 1

   - - Object
     - 描述

   - - CHSW
     - Chrome OS 开关位置

   - - HWID
     - Chrome OS 硬件 ID

   - - FWID
     - Chrome OS 固件版本

   - - FRID
     - Chrome OS 只读固件版本

   - - BINF
     - Chrome OS 启动信息

   - - GPIO
     - Chrome OS GPIO 分配

   - - VBNV
     - Chrome OS NVRAM 位置

   - - VDTA
     - Chrome OS 已验证启动数据

   - - FMAP
     - Chrome OS flashmap 基地址

   - - MLST
     - Chrome OS 方法列表

## CHSW（Chrome OS 开关位置）

此控制方法返回 Chrome OS 特定硬件开关的开关位置。

### 参数：

None

### 结果码：

一个包含以位域形式表示的开关位置的整数：

   :widths: 1 2

   - - 0x00000002
     - x86 固件启动时按下了恢复按钮。

   - - 0x00000004
     - EC 固件启动时按下了恢复按钮。（如果 EC EEPROM 可重写则为必填；否则可选）

   - - 0x00000020
     - x86 固件启动时启用了开发者开关。

   - - 0x00000200
     - x86 固件启动时禁用了固件写保护。（如果固件写保护由 x86 BIOS 控制则为必填；
       否则可选）

所有其他位均保留，应设为 0。

## HWID（Chrome OS 硬件 ID）

此控制方法返回 Chromebook 的硬件 ID。

### 参数：

None

### 结果码：

一个以 null 结尾的 ASCII 字符串，包含来自 EEPROM 的型号特定数据（Model-Specific Data）
区域的硬件 ID。

注意硬件 ID 最长可达 256 个字符，包含结尾的 null。

## FWID（Chrome OS 固件版本）

此控制方法返回主处理器固件可重写部分的固件版本。

### 参数：

None

### 结果码：

一个以 null 结尾的 ASCII 字符串，包含主处理器固件可重写部分的完整固件版本。

## FRID（Chrome OS 只读固件版本）

此控制方法返回主处理器固件只读部分的固件版本。

### 参数：

None

### 结果码：

一个以 null 结尾的 ASCII 字符串，包含主处理器固件只读（引导 + 恢复）部分的完整固件版本。

## BINF（Chrome OS 启动信息）

此控制方法返回关于当前启动的信息。

### 参数：

None

### 结果码：



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
     - 格式
     - 描述

   - - Reserved1
     - DWORD
     - 设为 256（0x100）。表示该字段已不再使用。

   - - Reserved2
     - DWORD
     - 设为 256（0x100）。表示该字段已不再使用。

   - - Active EC firmware
     - DWORD
     - 启动时使用的 EC 固件。

       - 0 - 只读（恢复）固件
       - 1 - 可重写固件。

       如果 EC 固件始终为只读，则设为 0。

   - - Active Main Firmware Type
     - DWORD
     - 启动时使用的主要固件类型。

       - 0 - 恢复（Recovery）
       - 1 - 正常（Normal）
       - 2 - 开发者（Developer）
       - 3 - 网络启动（netboot，仅工厂安装）

       其他值为保留值。

   - - Reserved5
     - DWORD
     - 设为 256（0x100）。表示该字段已不再使用。

## GPIO（Chrome OS GPIO 分配）

此控制方法返回关于 Chrome OS 硬件上 Chrome OS 特定 GPIO 分配的信息，
以便内核可以直接控制该硬件。

### 参数：

None

### 结果码：


        Package {
                Package {
                        // 第一个 GPIO 分配
                        Signal Type        //DWORD
                        Attributes         //DWORD
                        Controller Offset  //DWORD
                        Controller Name    //ASCIIZ
                },
                ...
                Package {
                        // 最后一个 GPIO 分配
                        Signal Type        //DWORD
                        Attributes         //DWORD
                        Controller Offset  //DWORD
                        Controller Name    //ASCIIZ
                }
        }

其中 ASCIIZ 表示以 null 结尾的 ASCII 字符串。

   :widths: 1 1 2
   :header-rows: 1

   - - Field
     - 格式
     - 描述

   - - Signal Type
     - DWORD
     - GPIO 信号的类型

       - 0x00000001 - 恢复按钮
       - 0x00000002 - 开发者模式开关
       - 0x00000003 - 固件写保护开关
       - 0x00000100 - 调试排针 GPIO 0
       - ...
       - 0x000001FF - 调试排针 GPIO 255

       其他值为保留值。

   - - Attributes
     - DWORD
     - 以位域表示的信号属性：

       - 0x00000001 - 信号为高电平有效（对于按钮，GPIO 值为 1 表示按钮被按下；
         对于开关，GPIO 值为 1 表示开关已启用）。如果该位为 0，则信号为低电平有效。
         调试排针 GPIO 设为 0。

   - - Controller Offset
     - DWORD
     - 指定控制器上的 GPIO 编号。

   - - Controller Name
     - ASCIIZ
     - GPIO 所属控制器的名称。
       目前支持的命名：
       "NM10" - Intel NM10 芯片

## VBNV（Chrome OS NVRAM 位置）

此控制方法返回关于用于与 BIOS 通信的 NVRAM（CMOS）位置的信息。

### 参数：

None

### 结果码：


        Package {
                NV Storage Block Offset  //DWORD
                NV Storage Block Size    //DWORD
        }

   :widths: 1 1 2
   :header-rows: 1

   - - Field
     - 格式
     - 描述

   - - NV Storage Block Offset
     - DWORD
     - 已验证启动非易失性存储块在 CMOS bank 0 中的偏移，从第一个可写 CMOS 字节
       开始计数（即 offset=0 是紧随 14 字节时钟数据之后的字节）。

   - - NV Storage Block Size
     - DWORD
     - 已验证启动非易失性存储块的大小（字节数）。

## FMAP（Chrome OS flashmap 地址）

此控制方法返回主处理器固件 flashmap 起始位置的物理内存地址。

### 参数：

None

### NoneResult code：

一个 DWORD，包含主处理器固件 flashmap 起始位置的物理内存地址。

## VDTA（Chrome OS 已验证启动数据）

此控制方法返回在固件验证步骤与内核验证步骤之间共享的已验证启动数据块。

### 参数：

None

### 结果码：

一个包含已验证启动数据块的缓冲区。

## MECK（管理引擎校验和）

此控制方法返回在启动期间从管理引擎（Management Engine）扩展寄存器读出的
SHA-1 或 SHA-256 哈希。该哈希通过 ACPI 导出，以便操作系统可以验证 ME 固件
是否发生了变更。如果不存在管理引擎，或者固件无法读取扩展寄存器，
此缓冲区可以为零。

### 参数：

None

### 结果码：

一个包含 ME 哈希的缓冲区。

## MLST（Chrome OS 方法列表）

此控制方法返回 Chrome OS 硬件设备支持的其他控制方法的列表。

### 参数：

None

### 结果码：

一个包（package），包含以 null 结尾的 ASCII 字符串列表，每个字符串对应 Chrome OS
硬件设备支持的一个控制方法，不包括 MLST 方法本身。对于本版本的规范，结果为：


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
