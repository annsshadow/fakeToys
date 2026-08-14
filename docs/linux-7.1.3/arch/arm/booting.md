## Booting ARM Linux


Author:	Russell King

Date  : 18 鍙?2002

The 浠ヤ笅 documentation 鏄?relevant 鍒?2.4.18-rmk6 鍜?beyond.

涓轰簡 boot ARM Linux, 鎮?闇€瑕?涓€涓?boot loader, 鍏?鏄?涓€涓?small
program 璇?runs 涔嬪墠 the 涓昏 鍐呮牳.  The boot loader 鏄?expected
鍒?initialise 鍚勭 璁惧, 鍜?eventually call the Linux 鍐呮牳,
passing information 鍒?the 鍐呮牳.

Essentially, the boot loader 搴斿綋 鎻愪緵 (浣滀负 涓€涓?鏈€灏? the
浠ヤ笅:

1. Setup 鍜?initialise the RAM.
2. Initialise one 涓茶 绔彛.
3. Detect the machine 绫诲瀷.
4. Setup the 鍐呮牳 tagged 鍒楀嚭.
5. 鍔犺浇 initramfs.
6. Call the 鍐呮牳 image.


### 1. Setup 鍜?initialise RAM


Existing boot loaders:
	MANDATORY
鏂?boot loaders:
	MANDATORY

The boot loader 鏄?expected 鍒?find 鍜?initialise 鍏ㄩ儴 RAM 璇?the
鍐呮牳 灏?浣跨敤 鐢ㄤ簬 volatile 鏁版嵁 storage 鍦?the 绯荤粺.  瀹?performs
姝?鍦?涓€涓?machine dependent manner.  (瀹?鍙?浣跨敤 鍐呴儴 algorithms
鍒?automatically locate 鍜?澶у皬 鍏ㄩ儴 RAM, 鎴?瀹?鍙?浣跨敤 knowledge 鐨?
the RAM 鍦?the machine, 鎴?浠讳綍 鍏朵粬 鏂规硶 the boot loader designer
sees fit.)


### 2. Initialise one 涓茶 绔彛


Existing boot loaders:
	鍙€? RECOMMENDED
鏂?boot loaders:
	鍙€? RECOMMENDED

The boot loader 搴斿綋 initialise 鍜?鍚敤 one 涓茶 绔彛 鍦?the
target.  姝?allows the 鍐呮牳 涓茶 椹卞姩 鍒?automatically detect
鍏?涓茶 绔彛 瀹?搴斿綋 浣跨敤 鐢ㄤ簬 the 鍐呮牳 console (generally
浣跨敤 鐢ㄤ簬 debugging purposes, 鎴?communication 涓?the target.)

浣滀负 涓€涓?alternative, the boot loader 鍙?pass the relevant 'console='
閫夐」 鍒?the 鍐呮牳 閫氳繃 the tagged 鍒楄〃 specifying the 绔彛, 鍜?
涓茶 鏍煎紡 閫夐」 浣滀负 鎻忚堪 鍦?

       Documentation/admin-guide/kernel-parameters.rst.


### 3. Detect the machine 绫诲瀷


Existing boot loaders:
	鍙€?
鏂?boot loaders:
	MANDATORY except 鐢ㄤ簬 DT-浠?platforms

The boot loader 搴斿綋 detect the machine 绫诲瀷 鍏?杩愯涓?鍦?鐢?涓€浜?
鏂规硶.  鏄惁 杩欐槸 涓€涓?hard coded 鍊?鎴?涓€浜?algorithm 璇?
looks 鍦?the connected 纭欢 鏄?beyond the scope 鐨?姝?document.
The boot loader 蹇呴』 ultimately 涓?able 鍒?鎻愪緵 涓€涓?MACH_绫诲瀷_xxx
鍊?鍒?the 鍐呮牳. (鍙傝 linux/arch/arm/tools/mach-types).  姝?
搴斿綋 涓?passed 鍒?the 鍐呮牳 鍦?娉ㄥ唽 r1.

鐢ㄤ簬 DT-浠?platforms, the machine 绫诲瀷 灏?涓?determined 鐢?璁惧
tree.  set the machine 绫诲瀷 鍒?鍏ㄩ儴 ones (~0).  杩欐槸 涓?strictly
蹇呰, 浣?assures 璇?瀹?灏?涓?match 浠讳綍 existing types.

### 4. Setup boot 鏁版嵁


Existing boot loaders:
	鍙€? HIGHLY RECOMMENDED
鏂?boot loaders:
	MANDATORY

The boot loader 蹇呴』 鎻愪緵 浠讳竴涓?涓€涓?tagged 鍒楀嚭 鎴?涓€涓?dtb image 鐢ㄤ簬
passing 閰嶇疆 鏁版嵁 鍒?the 鍐呮牳.  The 鐗╃悊 鍦板潃 鐨?the
boot 鏁版嵁 鏄?passed 鍒?the 鍐呮牳 鍦?娉ㄥ唽 r2.

### 4涓€涓? Setup the 鍐呮牳 tagged 鍒楀嚭


The boot loader 蹇呴』 鍒涘缓 鍜?initialise the 鍐呮牳 tagged 鍒楀嚭.
涓€涓?valid tagged 鍒楀嚭 starts 涓?ATAG_鏍稿績 鍜?ends 涓?ATAG_NONE.
The ATAG_鏍稿績 tag 鍙?鎴?鍙?涓?涓?empty.  涓€涓?empty ATAG_鏍稿績 tag
鍏锋湁 the 澶у皬 瀛楁 set 鍒?'2' (0x00000002).  The ATAG_NONE 蹇呴』 set
the 澶у皬 瀛楁 鍒?zero.

浠讳綍 鏁板瓧 鐨?tags 鍙?涓?placed 鍦?the 鍒楀嚭.  瀹冩槸 undefined
鏄惁 涓€涓?repeated tag appends 鍒?the information carried 鐢?the
鍓嶄竴涓?tag, 鎴?鏄惁 瀹?replaces the information 鍦?鍏?
entirety; 涓€浜?tags behave 浣滀负 the former, others the latter.

The boot loader 蹇呴』 pass 鍦?涓€涓?鏈€灏?the 澶у皬 鍜?location 鐨?
the 绯荤粺 鍐呭瓨, 鍜?root 鏂囦欢绯荤粺 location.  鍥犳, the
```

		+-----------+
  base ->	| ATAG_CORE |  |
		+-----------+  |
		| ATAG_MEM  |  | increasing address
		+-----------+  |
		| ATAG_NONE |  |
		+-----------+  v

```
The tagged 鍒楀嚭 搴斿綋 涓?stored 鍦?绯荤粺 RAM.

The tagged 鍒楀嚭 蹇呴』 涓?placed 鍦?涓€涓?region 鐨?鍐呭瓨 浣曞 涓よ€呴兘涓?
the 鍐呮牳 decompressor nor initrd 'bootp' program 灏?overwrite
瀹?  The recommended placement 鏄?鍦?the 绗竴 16KiB 鐨?RAM.

### 4b. Setup the 璁惧鏍?


The boot loader 蹇呴』 鍔犺浇 涓€涓?璁惧鏍?image (dtb) 杩涘叆 绯荤粺 ram
鍦?涓€涓?64浣?aligned 鍦板潃 鍜?initialize 瀹?涓?the boot 鏁版嵁.  The
dtb 鏍煎紡 鏄?documented 鍦?https://www.devicetree.org/specifications/.
The 鍐呮牳 灏?look 鐢ㄤ簬 the dtb magic 鍊?鐨?0xd00dfeed 鍦?the dtb
鐗╃悊 鍦板潃 鍒?determine 鑻?涓€涓?dtb 鍏锋湁 宸茬粡 passed 鑰岄潪 涓€涓?
tagged 鍒楀嚭.

The boot loader 蹇呴』 pass 鍦?涓€涓?鏈€灏?the 澶у皬 鍜?location 鐨?the
绯荤粺 鍐呭瓨, 鍜?the root 鏂囦欢绯荤粺 location.  The dtb 蹇呴』 涓?
placed 鍦?涓€涓?region 鐨?鍐呭瓨 浣曞 the 鍐呮牳 decompressor 灏?涓?
overwrite 瀹? 鍚屾椂 remaining 涔嬪唴 the region 鍏?灏?涓?covered
鐢?the 鍐呮牳's low-memory 鏄犲皠.

涓€涓?safe location 鏄?just 涓婃枃 the 128MiB boundary 鏉ヨ嚜 鍚姩 鐨?RAM.

### 5. 鍔犺浇 initramfs.


Existing boot loaders:
	鍙€?
鏂?boot loaders:
	鍙€?

鑻?涓€涓?initramfs 鏄?鍦?浣跨敤 鐒跺悗, 浣滀负 涓?the dtb, 瀹?蹇呴』 涓?placed 鍦?
涓€涓?region 鐨?鍐呭瓨 浣曞 the 鍐呮牳 decompressor 灏?涓?overwrite 瀹?
鍚屾椂 涔?涓?the region 鍏?灏?涓?covered 鐢?the 鍐呮牳's
low-memory 鏄犲皠.

涓€涓?safe location 鏄?just 涓婃枃 the 璁惧鏍?blob 鍏?itself 灏?
涓?loaded just 涓婃枃 the 128MiB boundary 鏉ヨ嚜 the 鍚姩 鐨?RAM 浣滀负
recommended 涓婃枃.

### 6. Calling the 鍐呮牳 image


Existing boot loaders:
	MANDATORY
鏂?boot loaders:
	MANDATORY

瀛樺湪 two 閫夐」 鐢ㄤ簬 calling the 鍐呮牳 zImage.  鑻?the zImage
鏄?stored 鍦?flash, 鍜?鏄?linked correctly 鍒?涓?杩愯 鏉ヨ嚜 flash,
鐒跺悗 瀹冩槸 legal 鐢ㄤ簬 the boot loader 鍒?call the zImage 鍦?flash
directly.

The zImage 鍙?涔?涓?placed 鍦?绯荤粺 RAM 鍜?called 閭ｉ噷.  The
鍐呮牳 搴斿綋 涓?placed 鍦?the 绗竴 128MiB 鐨?RAM.  瀹冩槸 recommended
璇?瀹冩槸 loaded 涓婃枃 32MiB 涓轰簡 avoid the 闇€瑕?鍒?relocate
prior 鍒?decompression, 鍏?灏?make the boot 杩涚▼ slightly
faster.

褰?booting 涓€涓?raw (non-zImage) 鍐呮牳 the constraints 鏄?tighter.
鍦?姝?case the 鍐呮牳 蹇呴』 涓?loaded 鍦?涓€涓?鍋忕Щ 杩涘叆 绯荤粺 equal
鍒?TEXT_鍋忕Щ - 椤礯鍋忕Щ.

鍦?浠讳綍 case, the 浠ヤ笅 conditions 蹇呴』 涓?met:

- Quiesce 鍏ㄩ儴 DMA capable 璁惧 鍥犳 璇?鍐呭瓨 鎵ц 涓?get
  corrupted 鐢?bogus 缃戠粶 packets 鎴?disk 鏁版嵁. 姝?灏?save
  鎮?璁稿 hours 鐨?debug.

- CPU 娉ㄥ唽 璁剧疆

  - r0 = 0,
  - r1 = machine 绫诲瀷 鏁板瓧 discovered 鍦?(3) 涓婃枃.
  - r2 = 鐗╃悊 鍦板潃 鐨?tagged 鍒楀嚭 鍦?绯荤粺 RAM, 鎴?
    鐗╃悊 鍦板潃 鐨?璁惧鏍?鍧?(dtb) 鍦?绯荤粺 RAM

- CPU 妯″紡

  鍏ㄩ儴 forms 鐨?涓柇 蹇呴』 涓?宸茬鐢?(IRQs 鍜?FIQs)

  鐢ㄤ簬 CPUs 鍏?鎵ц 涓?鍖呭惈 the ARM virtualization extensions, the
  CPU 蹇呴』 涓?鍦?SVC 妯″紡.  (涓€涓?鐗规畩 寮傚父 exists 鐢ㄤ簬 Angel)

  CPUs 鍏?鍖呭惈 鏀寔 鐢ㄤ簬 the virtualization extensions 鍙?涓?
  entered 鍦?HYP 妯″紡 涓轰簡 鍚敤 the 鍐呮牳 鍒?make full 浣跨敤 鐨?
  杩欎簺 extensions.  杩欐槸 the recommended boot 鏂规硶 鐢ㄤ簬 姝ょ被 CPUs,
  闄ら潪 the virtualisations 鏄?宸茬粡 鍦?浣跨敤 鐢?涓€涓?pre-installed
  hypervisor.

  鑻?the 鍐呮牳 鏄?涓?entered 鍦?HYP 妯″紡 鐢ㄤ簬 浠讳綍 reason, 瀹?蹇呴』 涓?
  entered 鍦?SVC 妯″紡.

- Caches, MMUs

  The MMU 蹇呴』 涓?off.

  Instruction 缂撳瓨 鍙?涓?鍦?鎴?off.

  鏁版嵁 缂撳瓨 蹇呴』 涓?off.

  鑻?the 鍐呮牳 鏄?entered 鍦?HYP 妯″紡, the 涓婃枃 requirements apply 鍒?
  the HYP 妯″紡 閰嶇疆 姝ゅ 鍒?the ordinary PL1 (privileged
  鍐呮牳 modes) 閰嶇疆.  姝ゅ, 鍏ㄩ儴 traps 杩涘叆 the
  hypervisor 蹇呴』 涓?宸茬鐢? 鍜?PL1 access 蹇呴』 涓?granted 鐢ㄤ簬 鍏ㄩ儴
  peripherals 鍜?CPU resources 鐢ㄤ簬 鍏?杩欐槸 architecturally
  鍙兘.  Except 鐢ㄤ簬 entering 鍦?HYP 妯″紡, the 绯荤粺 閰嶇疆
  搴斿綋 涓?姝ょ被 璇?涓€涓?鍐呮牳 鍏?鎵ц 涓?鍖呭惈 鏀寔 鐢ㄤ簬 the
  virtualization extensions 鍙?boot correctly 鏃?extra help.

- The boot loader 鏄?expected 鍒?call the 鍐呮牳 image 鐢?jumping
  directly 鍒?the 绗竴 instruction 鐨?the 鍐呮牳 image.

  鍦?CPUs supporting the ARM instruction set, the 鏉＄洰 蹇呴』 涓?
  made 鍦?ARM 鐘舵€? even 鐢ㄤ簬 涓€涓?Thumb-2 鍐呮牳.

  鍦?CPUs supporting 浠?the Thumb instruction set 渚嬪
  Cortex-M 绫?CPUs, the 鏉＄洰 蹇呴』 涓?made 鍦?Thumb 鐘舵€?
