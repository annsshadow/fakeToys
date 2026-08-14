## 内核驱动 i2c-viapro


支持的适配器：
  - VIA Technologies, Inc. VT82C596A/B
    Datasheet: 有时可在 VIA 网站获取

  - VIA Technologies, Inc. VT82C686A/B
    Datasheet: 有时可在 VIA 网站获取

  - VIA Technologies, Inc. VT8231、VT8233、VT8233A
    Datasheet: 可向 VIA 索取

  - VIA Technologies, Inc. VT8235、VT8237R、VT8237A、VT8237S、VT8251
    Datasheet: 可向 VIA 索取，且需签署 NDA

  - VIA Technologies, Inc. CX700
    Datasheet: 可向 VIA 索取，且需签署 NDA

  - VIA Technologies, Inc. VX800/VX820
    Datasheet: 可在 http://linux.via.com.tw 获取

  - VIA Technologies, Inc. VX855/VX875
    Datasheet: 可在 http://linux.via.com.tw 获取

  - VIA Technologies, Inc. VX900
    Datasheet: 可在 http://linux.via.com.tw 获取

Authors:
 - Kyösti Mälkki <kmalkki@cc.hut.fi>,
 - Mark D. Studebaker <mdsxyz123@yahoo.com>,
 - Jean Delvare <jdelvare@suse.de>

### 模块参数


- force: int
  强制启用 SMBus 控制器。危险！
- force_addr: int
  强制在给定地址启用 SMBus。极度危险！

### 描述


i2c-viapro 是一个真正的 SMBus 主控制器驱动，适用于搭载所支持 VIA 南桥
的主板。

你的 `lspci -n` 列表必须显示以下之一：

 ================   ======================
 device 1106:3050   (VT82C596A function 3)
 device 1106:3051   (VT82C596B function 3)
 device 1106:3057   (VT82C686 function 4)
 device 1106:3074   (VT8233)
 device 1106:3147   (VT8233A)
 device 1106:8235   (VT8231 function 4)
 device 1106:3177   (VT8235)
 device 1106:3227   (VT8237R)
 device 1106:3337   (VT8237A)
 device 1106:3372   (VT8237S)
 device 1106:3287   (VT8251)
 device 1106:8324   (CX700)
 device 1106:8353   (VX800/VX820)
 device 1106:8409   (VX855/VX875)
 device 1106:8410   (VX900)
 ================   ======================

如果这些都没有出现，你应该在 BIOS 中查找诸如启用 ACPI / SMBus 甚至 USB
之类的设置。

除最老的芯片（VT82C596A/B、VT82C686A，以及极可能是 VT8231）外，本驱动
支持 I2C 块事务。这类事务主要用于读写 EEPROM。

CX700/VX800/VX820 似乎还支持 SMBus PEC，尽管本驱动尚未实现它。
