## 鍐呮牳椹卞姩 i2c-viapro


鏀寔鐨勯€傞厤鍣細
  - VIA Technologies, Inc. VT82C596A/B
    Datasheet: 鏈夋椂鍙湪 VIA 缃戠珯鑾峰彇

  - VIA Technologies, Inc. VT82C686A/B
    Datasheet: 鏈夋椂鍙湪 VIA 缃戠珯鑾峰彇

  - VIA Technologies, Inc. VT8231銆乂T8233銆乂T8233A
    Datasheet: 鍙悜 VIA 绱㈠彇

  - VIA Technologies, Inc. VT8235銆乂T8237R銆乂T8237A銆乂T8237S銆乂T8251
    Datasheet: 鍙悜 VIA 绱㈠彇锛屼笖闇€绛剧讲 NDA

  - VIA Technologies, Inc. CX700
    Datasheet: 鍙悜 VIA 绱㈠彇锛屼笖闇€绛剧讲 NDA

  - VIA Technologies, Inc. VX800/VX820
    Datasheet: 鍙湪 http://linux.via.com.tw 鑾峰彇

  - VIA Technologies, Inc. VX855/VX875
    Datasheet: 鍙湪 http://linux.via.com.tw 鑾峰彇

  - VIA Technologies, Inc. VX900
    Datasheet: 鍙湪 http://linux.via.com.tw 鑾峰彇

Authors:
 - Ky枚sti M盲lkki <kmalkki@cc.hut.fi>,
 - Mark D. Studebaker <mdsxyz123@yahoo.com>,
 - Jean Delvare <jdelvare@suse.de>

### 妯″潡鍙傛暟


- force: int
  寮哄埗鍚敤 SMBus 鎺у埗鍣ㄣ€傚嵄闄╋紒
- force_addr: int
  寮哄埗鍦ㄧ粰瀹氬湴鍧€鍚敤 SMBus銆傛瀬搴﹀嵄闄╋紒

### 鎻忚堪


i2c-viapro 鏄竴涓湡姝ｇ殑 SMBus 涓绘帶鍒跺櫒椹卞姩锛岄€傜敤浜庢惌杞芥墍鏀寔 VIA 鍗楁ˉ
鐨勪富鏉裤€?
浣犵殑 `lspci -n` 鍒楄〃蹇呴』鏄剧ず浠ヤ笅涔嬩竴锛?
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

濡傛灉杩欎簺閮芥病鏈夊嚭鐜帮紝浣犲簲璇ュ湪 BIOS 涓煡鎵捐濡傚惎鐢?ACPI / SMBus 鐢氳嚦 USB
涔嬬被鐨勮缃€?
闄ゆ渶鑰佺殑鑺墖锛圴T82C596A/B銆乂T82C686A锛屼互鍙婃瀬鍙兘鏄?VT8231锛夊锛屾湰椹卞姩
鏀寔 I2C 鍧椾簨鍔°€傝繖绫讳簨鍔′富瑕佺敤浜庤鍐?EEPROM銆?
CX700/VX800/VX820 浼间箮杩樻敮鎸?SMBus PEC锛屽敖绠℃湰椹卞姩灏氭湭瀹炵幇瀹冦€?