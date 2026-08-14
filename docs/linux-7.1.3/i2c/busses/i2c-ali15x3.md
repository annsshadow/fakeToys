## 鍐呮牳椹卞姩 i2c-ali15x3


鏀寔鐨勯€傞厤鍣細
  - Acer Labs, Inc. ALI 1533 涓?1543C锛堝崡妗ワ級

    Datasheet: 鐩墠椤荤缃?NDA
	http://www.ali.com.tw/

浣滆€咃細
 - Frodo Looijaard <frodol@dds.nl>,
 - Philip Edelbrock <phil@netroedge.com>,
 - Mark D. Studebaker <mdsxyz123@yahoo.com>

### 妯″潡鍙傛暟


- force_addr: int
    鍒濆鍖?i2c 鎺у埗鍣ㄧ殑鍩哄湴鍧€

### 璇存槑


force_addr 鍙傛暟瀵逛簬 BIOS 涓病鏈夎缃湴鍧€鐨勪富鏉垮緢鏈夌敤銆傚畠涓嶄細鎵ц PCI 寮哄埗鎿嶄綔锛涜澶囦粛鐒跺繀椤诲瓨鍦ㄤ簬
lspci 涓€傞櫎闈為┍鍔ㄦ彁绀哄熀鍦板潃鏈璁剧疆锛屽惁鍒欎笉瑕佷娇鐢ㄦ鍙傛暟銆?
```

    modprobe i2c-ali15x3 force_addr=0xe800

```
鍦?ASUS P5A 涓绘澘涓婏紝SMBus 浼氬懆鏈熸€ф寕璧凤紝鍙兘閫氳繃鏂數閲嶅惎鏉ユ竻闄ゃ€傚師鍥犳湭鐭ワ紙瑙佷笅鏂団€滈棶棰樷€濓級銆?
### 鎻忚堪


杩欐槸 Acer Labs Inc. (ALI) M1541 涓?M1543C 鍗楁ˉ涓?SMB 涓绘満鎺у埗鍣ㄧ殑椹卞姩銆?
M1543C 鏄潰鍚戞闈㈢郴缁熺殑鍗楁ˉ銆?
M1541 鏄潰鍚戜究鎼虹郴缁熺殑鍗楁ˉ銆?
瀹冧滑灞炰簬浠ヤ笅 ALI 鑺墖缁勶細

 - 鈥淎laddin Pro 2鈥?鍖呭惈 M1621 Slot 1 鍖楁ˉ锛屽甫 AGP 涓?   100MHz CPU 鍓嶇鎬荤嚎
 - 鈥淎laddin V鈥?鍖呭惈 M1541 Socket 7 鍖楁ˉ锛屽甫 AGP 涓?100MHz
   CPU 鍓嶇鎬荤嚎

   涓€浜?Aladdin V 涓绘澘锛? - Asus P5A
 - Atrend ATC-5220
 - BCM/GVC VP1541
 - Biostar M5ALA
 - Gigabyte GA-5AX锛堥€氬父鏃犳硶宸ヤ綔锛屽洜涓?BIOS 娌℃湁
	  鍚敤 7101 璁惧锛侊級
 - Iwill XA100 Plus
 - Micronics C200
 - Microstar (MSI) MS-5169

  - 鈥淎laddin IV鈥?鍖呭惈 M1541 Socket 7 鍖楁ˉ锛?    鍏?host bus 鏈€楂?83.3 MHz銆?
鏈夊叧杩欎簺鑺墖鐨勬瑙堬紝璇峰弬瑙?http://www.acerlabs.com銆傜洰鍓嶇綉绔欎笂瀹屾暣鐨勬暟鎹墜鍐屽彈瀵嗙爜淇濇姢锛屼絾濡傛灉鑱旂郴
ALI 浣嶄簬鍦ｄ綍濉炵殑鍔炲叕瀹わ紝浠栦滑鍙兘浼氭彁渚涘瘑鐮併€?
M1533/M1543C 璁惧鍦?PCI 鎬荤嚎涓婅〃鐜颁负鍥涗釜鐙珛鐨勮澶囥€備竴涓?```

  00:02.0 USB Controller: Acer Laboratories Inc. M5237 (rev 03)
  00:03.0 Bridge: Acer Laboratories Inc. M7101      <= 杩欐槸鎴戜滑闇€瑕佺殑閭ｄ釜
  00:07.0 ISA bridge: Acer Laboratories Inc. M1533 (rev c3)
  00:0f.0 IDE interface: Acer Laboratories Inc. M5229 (rev c1)

```

   濡傛灉浣犵殑鏉垮瓙涓婅鏈?M1533 鎴?M1543C锛屽苟涓斾綘鐪嬪埌
   鈥渁li15x3: Error: Can't detect ali15x3!鈥?   閭ｄ箞璇疯繍琛?lspci銆?
   濡傛灉浣犵湅鍒?1533 鍜?5229 璁惧浣嗘病鏈?7101 璁惧锛?   閭ｄ箞浣犲繀椤诲湪 BIOS 涓惎鐢?ACPI銆丳MU銆丼MB 鎴栫被浼奸€夐」銆?
   濡傛灉鎵句笉鍒?M7101 璁惧锛岄┍鍔ㄥ皢鏃犳硶宸ヤ綔銆?
SMB 鎺у埗鍣ㄦ槸 M7101 璁惧鐨勪竴閮ㄥ垎锛孧7101 鏄竴涓鍚?ACPI 瑙勮寖鐨?鐢垫簮绠＄悊鍗曞厓锛圥MU锛夈€?
鏁翠釜 M7101 璁惧閮藉繀椤昏鍚敤锛孲MB 鎵嶈兘宸ヤ綔銆備綘涓嶈兘
鍙崟鐙惎鐢?SMB銆係MB 鍜?ACPI 鎷ユ湁鐙珛鐨?I/O 绌洪棿銆?鎴戜滑浼氱‘淇?SMB 琚惎鐢紝鑰?ACPI 鍒欎繚鎸佷笉鍔ㄣ€?
### 鐗规€?

璇ラ┍鍔ㄤ粎鎺у埗 SMB 涓绘満銆侻15X3 涓婄殑 SMB 浠庢満
鎺у埗鍣ㄦ湭琚惎鐢ㄣ€傝椹卞姩涓嶄娇鐢ㄤ腑鏂€?
### 闂


璇ラ┍鍔ㄤ粎涓?SMB 瀵勫瓨鍣ㄨ姹?I/O 绌洪棿銆?瀹冧笉浣跨敤 ACPI 鍖哄煙銆?
鍦?ASUS P5A 涓绘澘涓婏紝鏈夊浠芥姤鍛婄О
SMBus 浼氭寕璧凤紝涓斿彧鑳介€氳繃
鍏抽棴璁＄畻鏈虹數婧愭潵瑙ｅ喅銆傚湪涓绘澘娓╁害鍗囬珮鏃讹紙渚嬪 CPU 楂樿礋杞斤紝鎴栧瀛ｏ級鎯呭喌浼间箮鏇翠弗閲嶃€?璇ヤ富鏉垮彲鑳藉瓨鍦ㄧ數姘旈棶棰樸€?鍦?P5A 涓婏紝W83781D 浼犳劅鍣ㄨ姱鐗囧悓鏃朵綅浜?ISA 涓?SMBus 涓娿€傚洜姝わ紝浠呴€氳繃 ISA 鎬荤嚎璁块棶 W83781D 閫氬父鍙互
閬垮厤 SMBus 鎸傝捣銆?