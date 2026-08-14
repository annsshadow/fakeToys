## Mellanox 绯荤粺 LED 鐨勫唴鏍搁┍鍔?

涓轰互涓?Mellanox 绯荤粺鎻愪緵绯荤粺 LED 鏀寔锛?"msx6710"銆?msx6720"銆?msb7700"銆?msn2700"銆?msx1410"銆?"msn2410"銆?msb7800"銆?msn2740"銆?msn2100"銆?
### 鎻忚堪


椹卞姩涓虹郴缁?"msx6710"銆?msx6720"銆?msb7700"銆?msn2700"銆?"msx1410"銆?msn2410"銆?msb7800"銆?msn2740" 鎻愪緵浠ヤ笅 LED锛?
  - mlxcpld:fan1:green
  - mlxcpld:fan1:red
  - mlxcpld:fan2:green
  - mlxcpld:fan2:red
  - mlxcpld:fan3:green
  - mlxcpld:fan3:red
  - mlxcpld:fan4:green
  - mlxcpld:fan4:red
  - mlxcpld:psu:green
  - mlxcpld:psu:red
  - mlxcpld:status:green
  - mlxcpld:status:red

 "status"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x20
  - 浣?[3:0]

 "psu"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x20
  - 浣?[7:4]

 "fan1"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x21
  - 浣?[3:0]

 "fan2"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x21
  - 浣?[7:4]

 "fan3"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x22
  - 浣?[3:0]

 "fan4"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x22
  - 浣?[7:4]

 涓婅堪鎵€鏈?LED 鐨勯鑹叉帺鐮侊細

  [bit3,bit2,bit1,bit0] 鎴?  [bit7,bit6,bit5,bit4]锛?
 - [0,0,0,0] = LED 鍏抽棴
 - [0,1,0,1] = 绾㈣壊甯镐寒
 - [1,1,0,1] = 缁胯壊甯镐寒
 - [0,1,1,0] = 绾㈣壊闂儊 3Hz
 - [1,1,1,0] = 缁胯壊闂儊 3Hz
 - [0,1,1,1] = 绾㈣壊闂儊 6Hz
 - [1,1,1,1] = 缁胯壊闂儊 6Hz

椹卞姩涓虹郴缁?"msn2100" 鎻愪緵浠ヤ笅 LED锛?
  - mlxcpld:fan:green
  - mlxcpld:fan:red
  - mlxcpld:psu1:green
  - mlxcpld:psu1:red
  - mlxcpld:psu2:green
  - mlxcpld:psu2:red
  - mlxcpld:status:green
  - mlxcpld:status:red
  - mlxcpld:uid:blue

 "status"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x20
  - 浣?[3:0]

 "fan"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x21
  - 浣?[3:0]

 "psu1"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x23
  - 浣?[3:0]

 "psu2"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x23
  - 浣?[7:4]

 "uid"
  - CPLD 瀵勫瓨鍣ㄥ亸绉伙細0x24
  - 浣?[3:0]

 闄?uid 澶栵紝涓婅堪鎵€鏈?LED 鐨勯鑹叉帺鐮侊細

  [bit3,bit2,bit1,bit0] 鎴?  [bit7,bit6,bit5,bit4]锛?
 - [0,0,0,0] = LED 鍏抽棴
 - [0,1,0,1] = 绾㈣壊甯镐寒
 - [1,1,0,1] = 缁胯壊甯镐寒
 - [0,1,1,0] = 绾㈣壊闂儊 3Hz
 - [1,1,1,0] = 缁胯壊闂儊 3Hz
 - [0,1,1,1] = 绾㈣壊闂儊 6Hz
 - [1,1,1,1] = 缁胯壊闂儊 6Hz

 uid LED 鐨勯鑹叉帺鐮侊細
  [bit3,bit2,bit1,bit0]锛?
 - [0,0,0,0] = LED 鍏抽棴
 - [1,1,0,1] = 钃濊壊甯镐寒
 - [1,1,1,0] = 钃濊壊闂儊 3Hz
 - [1,1,1,1] = 钃濊壊闂儊 6Hz

椹卞姩鏀寔 3Hz 涓?6Hz 棰戠巼锛?0% 鍗犵┖姣旓級鐨勭‖浠堕棯鐑併€?3Hz 鏃跺崰绌烘瘮鍛ㄦ湡绾︿负 167 姣锛?Hz 鏃剁害涓?83 姣銆?