
## Linux/x86 寮曞鍗忚


鍦?x86 骞冲彴涓婏紝Linux 鍐呮牳閲囩敤浜嗕竴濂楃浉褰撳鏉傜殑寮曞绾﹀畾銆傝繖涓€绾﹀畾閮ㄥ垎婧愪簬鍘嗗彶鍘熷洜锛屼篃婧愯嚜鏃╂湡甯屾湜鍐呮牳鏈韩鎴愪负鍙紩瀵兼槧鍍忕殑鎯虫硶銆佸鏉傜殑 PC 鍐呭瓨妯″瀷锛屼互鍙婇殢鐫€瀹炴ā寮?DOS 浣滀负涓绘祦鎿嶄綔绯荤粺閫愭笎閫€鍑鸿垶鍙帮紝PC 琛屼笟鏈熸湜鍙戠敓鍙樺寲鎵€鑷淬€?

鐩墠锛孡inux/x86 寮曞鍗忚瀛樺湪浠ヤ笅鐗堟湰銆?

=============	============================================================
Old kernels	浠呮敮鎸?zImage/Image銆備竴浜涢潪甯告棭鏈熺殑鍐呮牳鐢氳嚦鍙兘涓嶆敮鎸佸懡浠よ銆?

Protocol 2.00	锛堝唴鏍?1.3.73锛夋柊澧?bzImage 鍜?initrd 鏀寔锛屼互鍙婂紩瀵煎姞杞界▼搴?
		涓庡唴鏍镐箣闂存寮忕殑閫氫俊鏂瑰紡銆俿etup.S 鍙樹负鍙噸瀹氫綅锛屼絾浼犵粺鐨?
		setup 鍖哄煙浠嶅亣瀹氬彲鍐欍€?

Protocol 2.01	锛堝唴鏍?1.3.76锛夋柊澧炲爢婧㈠嚭璀﹀憡銆?

Protocol 2.02	锛堝唴鏍?2.4.0-test3-pre3锛夋柊鐨勫懡浠よ鍗忚銆傞檷浣庡父瑙勫唴瀛樹笂闄愩€?
		涓嶅啀瑕嗙洊浼犵粺鐨?setup 鍖哄煙锛屼粠鑰岃閭ｄ簺閫氳繃 SMM 鎴?32 浣?BIOS
		鍏ュ彛鐐逛娇鐢?EBDA 鐨勭郴缁熷紩瀵兼洿鍔犲畨鍏ㄣ€倆Image 宸插純鐢ㄤ絾浠嶅彈鏀寔銆?

Protocol 2.03	锛堝唴鏍?2.4.18-pre1锛夋樉寮忓湴鍚戝紩瀵煎姞杞界▼搴忔彁渚涘彲鑳界殑鏈€楂?
		initrd 鍦板潃銆?

Protocol 2.04	锛堝唴鏍?2.6.14锛夊皢 syssize 瀛楁鎵╁睍鍒板洓涓瓧鑺傘€?

Protocol 2.05	锛堝唴鏍?2.6.20锛変娇淇濇姢妯″紡鍐呮牳鍙噸瀹氫綅銆傚紩鍏?relocatable_kernel
		鍜?kernel_alignment 瀛楁銆?

Protocol 2.06	锛堝唴鏍?2.6.22锛夋柊澧炰竴涓瓧娈碉紝鐢ㄤ簬淇濆瓨寮曞鍛戒护琛岀殑澶у皬銆?

Protocol 2.07	锛堝唴鏍?2.6.24锛夋柊澧炲崐铏氭嫙鍖栫殑寮曞鍗忚銆傚紩鍏?hardware_subarch銆?
		hardware_subarch_data 浠ュ強 load_flags 涓殑 KEEP_SEGMENTS 鏍囧織銆?

Protocol 2.08	锛堝唴鏍?2.6.26锛夋柊澧?crc32 鏍￠獙鍜屼笌 ELF 鏍煎紡鐨勬湁鏁堣浇鑽枫€?
		寮曞叆 payload_offset 鍜?payload_length 瀛楁浠ヨ緟鍔╁畾浣嶆湁鏁堣浇鑽枫€?

Protocol 2.09	锛堝唴鏍?2.6.26锛夋柊澧炰竴涓?64 浣嶇墿鐞嗘寚閽堝瓧娈碉紝鎸囧悜
		struct setup_data 鐨勫崟鍚戦摼琛ㄣ€?

Protocol 2.10	锛堝唴鏍?2.6.31锛夊湪宸叉湁 kernel_alignment 鐨勫熀纭€涓婃柊澧炲鏉惧榻?
		鍗忚锛屾柊澧?init_size 鍜?pref_address 瀛楁銆傛柊澧炴墿灞曠殑寮曞
		鍔犺浇绋嬪簭 ID銆?

Protocol 2.11	锛堝唴鏍?3.6锛夋柊澧炰竴涓瓧娈碉紝鐢ㄤ簬淇濆瓨 EFI 浜ゆ帴鍗忚鍏ュ彛鐐圭殑鍋忕Щ閲忋€?

Protocol 2.12	锛堝唴鏍?3.8锛夋柊澧?xloadflags 瀛楁鍙?struct boot_params 鐨勬墿灞?
		瀛楁锛岀敤浜庡湪 64 浣嶇幆澧冧笅灏?bzImage 鍜?ramdisk 鍔犺浇鍒?4G 浠ヤ笂銆?

Protocol 2.13	锛堝唴鏍?3.14锛夋敮鎸佸湪 xloadflags 涓缃?32 浣嶄笌 64 浣嶆爣蹇楋紝
		浠ユ敮鎸佷粠 32 浣?EFI 寮曞 64 浣嶅唴鏍?

Protocol 2.14	鍥犻敊璇彁浜よ€屼綔搴?
                ae7e1238e68f2a472a125673ab506d49158c1889
		锛?x86/boot: Add ACPI RSDP address to setup_header"锛?
		涓嶈浣跨敤锛侊紒锛佽浣滀笌 2.13 鐩稿悓銆?

Protocol 2.15	锛堝唴鏍?5.5锛夋柊澧?kernel_info 涓?kernel_info.setup_type_max銆?
=============	============================================================

     鍙湁鍦?setup header 鍙戠敓鍙樻洿鏃舵墠搴旀洿鏀瑰崗璁増鏈彿銆傚鏋?boot_params 鎴?
     kernel_info 鍙戠敓鍙樻洿锛屽垯鏃犻渶鏇存柊鐗堟湰鍙枫€傛澶栵紝寤鸿浣跨敤 xloadflags锛堣繖绉?
     鎯呭喌涓嬪崗璁増鏈彿涔熶笉搴旀洿鏂帮級鎴?kernel_info 鏉ュ悜寮曞鍔犺浇绋嬪簭浼犺揪 Linux
     鍐呮牳鎵€鏀寔鐨勭壒鎬с€傜敱浜庡師濮?setup header 涓殑鍙敤绌洪棿闈炲父鏈夐檺锛屽鍏剁殑浠讳綍
     鏇存柊閮藉簲鏋佷负璋ㄦ厧銆備粠鍗忚 2.15 寮€濮嬶紝鍚戝紩瀵煎姞杞界▼搴忎紶杈句俊鎭殑涓昏鏂瑰紡鏄?
     kernel_info銆?


## 鍐呭瓨甯冨眬


鐢ㄤ簬 Image 鎴栦互涓嬪唴鏍稿姞杞藉櫒鐨勪紶缁熷唴瀛樻槧灏勶細

```
		|  			 |
  0A0000	+------------------------+
		|  Reserved for BIOS	 |	Do not use.  Reserved for BIOS EBDA.
  09A000	+------------------------+
		|  Command line		 |
		|  Stack/heap		 |	For use by the kernel real-mode code.
  098000	+------------------------+
		|  Kernel setup		 |	The kernel real-mode code.
  090200	+------------------------+
		|  Kernel boot sector	 |	The kernel legacy boot sector.
  090000	+------------------------+
		|  Protected-mode kernel |	The bulk of the kernel image.
  010000	+------------------------+
		|  Boot loader		 |	<- Boot sector entry point 0000:7C00
  001000	+------------------------+
		|  Reserved for MBR/BIOS |
  000800	+------------------------+
		|  Typically used by MBR |
  000600	+------------------------+
		|  BIOS use only	 |
  000000	+------------------------+

```

褰撲娇鐢?bzImage 鏃讹紝淇濇姢妯″紡鍐呮牳琚噸瀹氫綅鍒?0x100000锛?楂樼鍐呭瓨"锛夛紝鑰屽唴鏍稿疄妯″紡鍧楋紙寮曞鎵囧尯銆乻etup 浠ュ強鏍?鍫嗭級琚缃负鍙噸瀹氫綅鍒?0x10000 鍒颁綆鍐呭瓨鏈熬涔嬮棿鐨勪换鎰忓湴鍧€銆傞仐鎲剧殑鏄紝鍦?2.00 鍜?2.01 鍗忚涓紝0x90000 浠ヤ笂鍐呭瓨鑼冨洿浠嶈鍐呮牳鍐呴儴浣跨敤锛?.02 鍗忚瑙ｅ喅浜嗚繖涓€闂銆?

鏈€濂藉皢"鍐呭瓨涓婇檺"锛堝嵆寮曞鍔犺浇绋嬪簭瑙﹀強鐨勪綆鍐呭瓨鏈€楂樹綅缃級淇濇寔寰楀敖鍙兘浣庯紝鍥犱负涓€浜涜緝鏂扮殑 BIOS 宸茬粡寮€濮嬪湪浣庡唴瀛橀《閮ㄩ檮杩戝垎閰嶅ぇ閲忚绉颁负鎵╁睍 BIOS 鏁版嵁鍖猴紙Extended BIOS Data Area锛夌殑鍐呭瓨銆傚紩瀵煎姞杞界▼搴忓簲浣跨敤 "INT 12h" BIOS 璋冪敤鏉ョ‘璁ゆ湁澶氬皯浣庡唴瀛樺彲鐢ㄣ€?

閬楁喚鐨勬槸锛屽鏋?INT 12h 鎶ュ憡鍙敤鍐呭瓨杩囦綆锛屽紩瀵煎姞杞界▼搴忛€氬父鏃犺兘涓哄姏锛屽彧鑳藉悜鐢ㄦ埛鎶ュ憡閿欒銆傚洜姝わ紝寮曞鍔犺浇绋嬪簭鐨勮璁″簲灏藉彲鑳藉皯鍗犵敤浣庡唴瀛樼┖闂淬€傚浜庨渶瑕佸皢鏁版嵁鍐欏叆 0x90000 娈电殑 zImage 鎴栨棫鐗?bzImage 鍐呮牳锛屽紩瀵煎姞杞界▼搴忓簲纭繚涓嶄娇鐢?0x9A000 浠ヤ笂鐨勫唴瀛橈紱鏈夊お澶?BIOS 鍦ㄨ浣嶇疆浠ヤ笂浼氬嚭閿欍€?

瀵逛簬寮曞鍗忚鐗堟湰 >= 2.02 鐨勭幇浠?bzImage 鍐呮牳锛屽唴瀛樺竷灞€濡備笅锛?

```
		~  			 ~
		|  Protected-mode kernel |
  100000	+------------------------+
		|  I/O memory hole	 |
  0A0000	+------------------------+
		|  Reserved for BIOS	 |	Leave as much as possible unused
		~  			 ~
		|  Command line		 |	(Can also be below the X+10000 mark)
  X+10000	+------------------------+
		|  Stack/heap		 |	For use by the kernel real-mode code.
  X+08000	+------------------------+
		|  Kernel setup		 |	The kernel real-mode code.
		|  Kernel boot sector	 |	The kernel legacy boot sector.
  X		+------------------------+
		|  Boot loader		 |	<- Boot sector entry point 0000:7C00
  001000	+------------------------+
		|  Reserved for MBR/BIOS |
  000800	+------------------------+
		|  Typically used by MBR |
  000600	+------------------------+
		|  BIOS use only	 |
  000000	+------------------------+

  ... 鍏朵腑鍦板潃 X 鍙栧紩瀵煎姞杞界▼搴忚璁℃墍鍏佽鐨勬渶浣庡€笺€?


```

## 瀹炴ā寮忓唴鏍稿ご


鍦ㄤ笅闈㈢殑鏂囧瓧浠ュ強鍐呮牳寮曞杩囩▼鐨勪换浣曞湴鏂癸紝"鎵囧尯"鎸囩殑鏄?512 瀛楄妭銆傚畠涓庡簳灞備粙璐ㄧ殑瀹為檯鎵囧尯澶у皬鏃犲叧銆?

鍔犺浇 Linux 鍐呮牳鐨勭涓€姝ュ簲璇ユ槸鍔犺浇瀹炴ā寮忎唬鐮侊紙寮曞鎵囧尯鍜?setup 浠ｇ爜锛夛紝鐒跺悗妫€鏌ヤ綅浜庡亸绉?0x01f1 澶勭殑浠ヤ笅澶淬€傚疄妯″紡浠ｇ爜鏈€澶氬彲杈?32K锛屼笉杩囧紩瀵煎姞杞界▼搴忓彲浠ラ€夋嫨鍙姞杞藉墠涓や釜鎵囧尯锛?K锛夛紝鐒跺悗妫€鏌ュ紩瀵兼墖鍖哄ぇ灏忋€?

璇ュご鐨勭粨鏋勫涓嬶細

===========	========	=====================	============================================
Offset/Size	Proto		Name			Meaning
===========	========	=====================	============================================
01F1/1		ALL(1)		setup_sects		setup 鐨勫ぇ灏忥紙浠ユ墖鍖轰负鍗曚綅锛?
01F2/2		ALL		root_flags		鑻ヨ缃紝鍒欐牴鏂囦欢绯荤粺浠ュ彧璇绘柟寮忔寕杞?
01F4/4		2.04+(2)	syssize			32 浣嶄唬鐮佺殑澶у皬锛屼互 16 瀛楄妭娈佃惤涓哄崟浣?
01F8/2		ALL		ram_size		鍕跨敤 - 浠呬緵 bootsect.S 浣跨敤
01FA/2		ALL		vid_mode		瑙嗛妯″紡鎺у埗
01FC/2		ALL		root_dev		榛樿鏍硅澶囧彿
01FE/2		ALL		boot_flag		榄旀暟 0xAA55
0200/2		2.00+		jump			璺宠浆鎸囦护
0202/4		2.00+		header			榄旀暟绛惧悕 "HdrS"
0206/2		2.00+		version			鎵€鏀寔鐨勫紩瀵煎崗璁増鏈?
0208/4		2.00+		realmode_swtch		寮曞鍔犺浇绋嬪簭閽╁瓙锛堣涓嬫枃锛?
020C/2		2.00+		start_sys_seg		浣庝綅鍔犺浇娈碉紙0x1000锛夛紙宸插簾寮冿級
020E/2		2.00+		kernel_version		鎸囧悜鍐呮牳鐗堟湰瀛楃涓茬殑鎸囬拡
0210/1		2.00+		type_of_loader		寮曞鍔犺浇绋嬪簭鏍囪瘑绗?
0211/1		2.00+		loadflags		寮曞鍗忚閫夐」鏍囧織
0212/2		2.00+		setup_move_size		绉诲姩鍒伴珮绔唴瀛樼殑澶у皬锛堜笌閽╁瓙閰嶅悎浣跨敤锛?
0214/4		2.00+		code32_start		寮曞鍔犺浇绋嬪簭閽╁瓙锛堣涓嬫枃锛?
0218/4		2.00+		ramdisk_image		initrd 鍔犺浇鍦板潃锛堢敱寮曞鍔犺浇绋嬪簭璁剧疆锛?
021C/4		2.00+		ramdisk_size		initrd 澶у皬锛堢敱寮曞鍔犺浇绋嬪簭璁剧疆锛?
0220/4		2.00+		bootsect_kludge		鍕跨敤 - 浠呬緵 bootsect.S 浣跨敤
0224/2		2.01+		heap_end_ptr		setup 缁撴潫鍚庣殑绌洪棽鍐呭瓨
0226/1		2.02+(3)	ext_loader_ver		鎵╁睍鐨勫紩瀵煎姞杞界▼搴忕増鏈?
0227/1		2.02+(3)	ext_loader_type		鎵╁睍鐨勫紩瀵煎姞杞界▼搴?ID
0228/4		2.02+		cmd_line_ptr		鎸囧悜鍐呮牳鍛戒护琛岀殑 32 浣嶆寚閽?
022C/4		2.03+		initrd_addr_max		鍚堟硶鐨勬渶楂?initrd 鍦板潃
0230/4		2.05+		kernel_alignment	鍐呮牳鎵€闇€鐨勭墿鐞嗗湴鍧€瀵归綈
0234/1		2.05+		relocatable_kernel	鍐呮牳鏄惁鍙噸瀹氫綅
0235/1		2.10+		min_alignment		鏈€灏忓榻愶紝浠?2 鐨勫箓琛ㄧず
0236/2		2.12+		xloadflags		寮曞鍗忚閫夐」鏍囧織
0238/4		2.06+		cmdline_size		鍐呮牳鍛戒护琛岀殑鏈€澶уぇ灏?
023C/4		2.07+		hardware_subarch	纭欢瀛愭灦鏋?
0240/8		2.07+		hardware_subarch_data	鐗瑰畾浜庡瓙鏋舵瀯鐨勬暟鎹?
0248/4		2.08+		payload_offset		鍐呮牳鏈夋晥杞借嵎鐨勫亸绉婚噺
024C/4		2.08+		payload_length		鍐呮牳鏈夋晥杞借嵎鐨勯暱搴?
0250/8		2.09+		setup_data		鎸囧悜 struct setup_data 閾捐〃鐨?64 浣嶇墿鐞嗘寚閽?
0258/8		2.10+		pref_address		鍋忓ソ鐨勫姞杞藉湴鍧€
0260/4		2.10+		init_size		鍒濆鍖栨湡闂存墍闇€鐨勭嚎鎬у唴瀛?
0264/4		2.11+		handover_offset		浜ゆ帴鍏ュ彛鐐圭殑鍋忕Щ閲?
0268/4		2.15+		kernel_info_offset	kernel_info 鐨勫亸绉婚噺
===========	========	=====================	============================================

     锛?锛変负淇濇寔鍚戝悗鍏煎锛屽鏋?setup_sects 瀛楁涓?0锛屽垯鐪熷疄鍊间负 4銆?

     锛?锛夊浜?2.04 涔嬪墠鐨勫紩瀵煎崗璁紝syssize 瀛楁鐨勯珮涓や釜瀛楄妭涓嶅彲鐢紝杩欐剰鍛崇潃
         鏃犳硶纭畾 bzImage 鍐呮牳鐨勫ぇ灏忋€?

     锛?锛夊浜?2.02-2.09 寮曞鍗忚锛岃瀛楁琚拷鐣ワ紝浣嗚缃畠鏄畨鍏ㄧ殑銆?

濡傛灉鍦ㄥ亸绉?0x202 澶勬病鏈夋壘鍒?"HdrS"锛?x53726448锛夐瓟鏁帮紝鍒欏紩瀵煎崗璁増鏈负 "old"锛堟棫鐗堬級銆傚姞杞芥棫鍐呮牳鏃讹紝鎯呭喌濡備笅锛?

```
  Image type = zImage
  initrd not supported
  Real-mode kernel must be located at 0x90000.

```

鍚﹀垯锛?version" 瀛楁鍖呭惈鍗忚鐗堟湰锛屼緥濡傚崗璁増鏈?2.01 鍦ㄨ瀛楁涓皢鍖呭惈 0x0201銆傚湪璁剧疆澶翠腑鐨勫瓧娈垫椂锛屼綘蹇呴』纭繚鍙缃綋鍓嶆墍鐢ㄥ崗璁増鏈墍鏀寔鐨勫瓧娈点€?


## 澶村瓧娈佃瑙?


瀵逛簬姣忎釜瀛楁锛屾湁浜涙槸鍐呮牳鎻愪緵缁欏紩瀵煎姞杞界▼搴忕殑淇℃伅锛?read"/璇伙級锛屾湁浜涢渶瑕佺敱寮曞鍔犺浇绋嬪簭濉啓锛?write"/鍐欙級锛岃繕鏈変簺闇€瑕佺敱寮曞鍔犺浇绋嬪簭璇诲彇骞朵慨鏀癸紙"modify"/淇敼锛夈€傛墍鏈夐€氱敤寮曞鍔犺浇绋嬪簭閮藉簲鍐欏叆鏍囪涓猴紙obligatory/蹇呭～锛夌殑瀛楁銆傚笇鏈涘皢鍐呮牳鍔犺浇鍒伴潪鏍囧噯鍦板潃鐨勫紩瀵煎姞杞界▼搴忓簲濉啓鏍囪涓猴紙reloc/鍙噸瀹氫綅锛夌殑瀛楁锛涘叾浠栧紩瀵煎姞杞界▼搴忓彲浠ュ拷鐣ヨ繖浜涘瓧娈点€?

鎵€鏈夊瓧娈电殑瀛楄妭搴忓潎涓哄皬绔紙姣曠珶杩欐槸 x86锛夈€?

============	===========
Field name:	setup_sects
Type:		read
Offset/size:	0x1f1/1
Protocol:	ALL
============	===========

  setup 浠ｇ爜鐨勫ぇ灏忥紝浠?512 瀛楄妭鎵囧尯涓哄崟浣嶃€傚鏋滆瀛楁涓?0锛屽垯鐪熷疄鍊间负 4銆傚疄妯″紡浠ｇ爜鐢卞紩瀵兼墖鍖猴紙濮嬬粓涓轰竴涓?512 瀛楄妭鎵囧尯锛夊姞涓?setup 浠ｇ爜缁勬垚銆?

============	=================
Field name:	root_flags
Type:		modify (optional)
Offset/size:	0x1f2/2
Protocol:	ALL
============	=================

  濡傛灉璇ュ瓧娈甸潪闆讹紝鍒欐牴鏂囦欢绯荤粺榛樿涓哄彧璇汇€傝瀛楁鐨勪娇鐢ㄥ凡琚純鐢紱璇锋敼鐢ㄥ懡浠よ涓婄殑 "ro" 鎴?"rw" 閫夐」銆?

============	===============================================
Field name:	syssize
Type:		read
Offset/size:	0x1f4/4 (protocol 2.04+) 0x1f4/2 (protocol ALL)
Protocol:	2.04+
============	===============================================

  淇濇姢妯″紡浠ｇ爜鐨勫ぇ灏忥紝浠?16 瀛楄妭娈佃惤涓哄崟浣嶃€傚浜?2.04 涔嬪墠鐨勫崗璁増鏈紝璇ュ瓧娈靛彧鏈変袱涓瓧鑺傚锛屽洜姝ゅ湪璁剧疆浜?LOAD_HIGH 鏍囧織鏃讹紝涓嶈兘鎹纭畾鍐呮牳鐨勫ぇ灏忋€?

============	===============
Field name:	ram_size
Type:		kernel internal
Offset/size:	0x1f8/2
Protocol:	ALL
============	===============

  璇ュ瓧娈靛凡搴熷純銆?

============	===================
Field name:	vid_mode
Type:		modify (obligatory)
Offset/size:	0x1fa/2
============	===================

  璇峰弬闃?鐗规畩鍛戒护琛岄€夐」"灏忚妭銆?

============	=================
Field name:	root_dev
Type:		modify (optional)
Offset/size:	0x1fc/2
Protocol:	ALL
============	=================

  榛樿鏍硅澶囧彿銆傝瀛楁鐨勪娇鐢ㄥ凡琚純鐢紝璇锋敼鐢ㄥ懡浠よ涓婄殑 "root=" 閫夐」銆?

============	=========
Field name:	boot_flag
Type:		read
Offset/size:	0x1fe/2
Protocol:	ALL
============	=========

  鍖呭惈 0xAA55銆傝繖鏄棫鐗?Linux 鍐呮牳鏈€鎺ヨ繎榄旀暟鐨勪笢瑗裤€?

============	=======
Field name:	jump
Type:		read
Offset/size:	0x200/2
Protocol:	2.00+
============	=======

  鍖呭惈涓€涓?x86 璺宠浆鎸囦护锛屽嵆 0xEB 鍚庤窡涓€涓浉瀵逛簬瀛楄妭 0x202 鐨勬湁绗﹀彿鍋忕Щ閲忋€傝繖鍙敤浜庣‘瀹氬ご鐨勫ぇ灏忋€?

============	=======
Field name:	header
Type:		read
Offset/size:	0x202/4
Protocol:	2.00+
============	=======

  鍖呭惈榄旀暟 "HdrS"锛?x53726448锛夈€?

============	=======
Field name:	version
Type:		read
Offset/size:	0x206/2
Protocol:	2.00+
============	=======

  鍖呭惈寮曞鍗忚鐗堟湰锛屾牸寮忎负 (major << 8) + minor锛屼緥濡傜増鏈?2.04 涓?0x0204锛屽亣璁剧殑鐗堟湰 10.17 涓?0x0a11銆?

============	=================
Field name:	realmode_swtch
Type:		modify (optional)
Offset/size:	0x208/4
Protocol:	2.00+
============	=================

  寮曞鍔犺浇绋嬪簭閽╁瓙锛堣涓嬫枃"楂樼骇寮曞鍔犺浇绋嬪簭閽╁瓙"锛夈€?

============	=============
Field name:	start_sys_seg
Type:		read
Offset/size:	0x20c/2
Protocol:	2.00+
============	=============

  浣庝綅鍔犺浇娈碉紙0x1000锛夈€傚凡搴熷純銆?

============	==============
Field name:	kernel_version
Type:		read
Offset/size:	0x20e/2
Protocol:	2.00+
============	==============

  鑻ヨ缃负闈為浂鍊硷紝鍒欏寘鍚竴涓寚鍚戜互 NUL 缁撳熬銆佷汉绫诲彲璇荤殑鍐呮牳鐗堟湰鍙峰瓧绗︿覆鐨勬寚閽堬紝鍑忓幓 0x200銆傝繖鍙敤浜庡悜鐢ㄦ埛鏄剧ず鍐呮牳鐗堟湰銆傝鍊煎簲灏忎簬 (0x200 * setup_sects)銆備緥濡傦紝濡傛灉璇ュ€艰涓?0x1c00锛屽垯鍐呮牳鐗堟湰鍙峰瓧绗︿覆鍙湪鍐呮牳鏂囦欢鍋忕Щ 0x1e00 澶勬壘鍒般€傚綋涓斾粎褰?"setup_sects" 瀛楁

```
  0x1c00  < 15 * 0x200 (= 0x1e00) but
  0x1c00 >= 14 * 0x200 (= 0x1c00)

  0x1c00 >> 9 = 14, So the minimum value for setup_secs is 15.

```

============	==================
Field name:	type_of_loader
Type:		write (obligatory)
Offset/size:	0x210/1
Protocol:	2.00+
============	==================

  濡傛灉浣犵殑寮曞鍔犺浇绋嬪簭鏈変竴涓凡鍒嗛厤鐨?ID锛堣涓嬭〃锛夛紝鍒欏湪姝ゅ～鍏?0xTV锛屽叾涓?T 鏄紩瀵煎姞杞界▼搴忕殑鏍囪瘑绗︼紝V 鏄増鏈彿銆傚惁鍒欙紝鍦ㄦ濉叆 0xFF銆傚浜?T = 0xD 浠ヤ笂鐨勫紩瀵煎姞杞界▼搴?ID锛屽皢 T = 0xE 鍐欏叆璇ュ瓧娈碉紝骞跺皢鎵╁睍 ID 鍑忓幓 0x10 鍚庡啓鍏?ext_loader_type 瀛楁銆傜被浼煎湴锛宔xt_loader_ver 瀛楁鍙敤浜庝负寮曞鍔犺浇绋嬪簭鐗堟湰鎻愪緵瓒呰繃 4 浣嶇殑淇℃伅銆?

```
   type_of_loader  <- 0xE4
   ext_loader_type <- 0x05
   ext_loader_ver  <- 0x23

  Assigned boot loader IDs:

	==== =======================================
	0x0  LILO
	     (0x00 reserved for pre-2.00 bootloader)
	0x1  Loadlin
	0x2  bootsect-loader
	     (0x20, all other values reserved)
	0x3  Syslinux
	0x4  Etherboot/gPXE/iPXE
	0x5  ELILO
	0x7  GRUB
	0x8  U-Boot
	0x9  Xen
	0xA  Gujin
	0xB  Qemu
	0xC  Arcturus Networks uCbootloader
	0xD  kexec-tools
	0xE  Extended (see ext_loader_type)
	0xF  Special (0xFF = undefined)
	0x10 Reserved
	0x11 Minimal Linux Bootloader
	     <http://sebastian-plotz.blogspot.de>
	0x12 OVMF UEFI virtualization stack
	0x13 barebox
	==== =======================================

  Please contact <hpa@zytor.com> if you need a bootloader ID value assigned.

```

============	===================
Field name:	loadflags
Type:		modify (obligatory)
Offset/size:	0x211/1
Protocol:	2.00+
============	===================

  璇ュ瓧娈垫槸涓€涓綅鎺╃爜銆?

  Bit 0 (read):	LOADED_HIGH

 - 濡傛灉涓?0锛屼繚鎶ゆā寮忎唬鐮佸姞杞藉湪 0x10000銆?
 - 濡傛灉涓?1锛屼繚鎶ゆā寮忎唬鐮佸姞杞藉湪 0x100000銆?

  Bit 1 (kernel internal): KASLR_FLAG

 - 琚帇缂╁唴鏍稿唴閮ㄤ娇鐢紝鐢ㄤ簬鍚戠湡姝ｇ殑鍐呮牳浼犺揪 KASLR 鐘舵€併€?

     - 濡傛灉涓?1锛屽垯鍚敤 KASLR銆?
     - 濡傛灉涓?0锛屽垯绂佺敤 KASLR銆?

  Bit 5 (write): QUIET_FLAG

 - 濡傛灉璇ヤ綅涓?0锛屽垯鎵撳嵃鏃╂湡娑堟伅銆?
 - 濡傛灉涓?1锛屽垯鎶戝埗鏃╂湡娑堟伅銆?

		杩欏悜鍐呮牳锛堣В鍘嬬▼搴忓拰鏃╂湡鍐呮牳锛夎姹備笉瑕佸啓鍏ラ渶瑕佺洿鎺ヨ闂樉绀虹‖浠剁殑鏃╂湡娑堟伅銆?

  Bit 6 (obsolete): KEEP_SEGMENTS

	Protocol: 2.07+

        - 璇ユ爣蹇楀凡搴熷純銆?

  Bit 7 (write): CAN_USE_HEAP

	灏嗚浣嶈涓?1 琛ㄧず heap_end_ptr 涓～鍏ョ殑鍊兼槸鏈夋晥鐨勩€傚鏋滆浣嶆竻闆讹紝閮ㄥ垎 setup 浠ｇ爜鍔熻兘灏嗚绂佺敤銆?


============	===================
Field name:	setup_move_size
Type:		modify (obligatory)
Offset/size:	0x212/2
Protocol:	2.00-2.01
============	===================

  褰撲娇鐢?2.00 鎴?2.01 鍗忚鏃讹紝濡傛灉瀹炴ā寮忓唴鏍告湭鍔犺浇鍦?0x90000锛屽垯浼氬湪鍔犺浇杩囩▼鐨勫悗缁楠よ绉诲姩鍒伴偅閲屻€傚鏋滀綘甯屾湜闄ゅ疄妯″紡鍐呮牳鏈韩涔嬪杩樼Щ鍔ㄥ叾浠栨暟鎹紙渚嬪鍐呮牳鍛戒护琛岋級锛屽垯濉啓璇ュ瓧娈点€?

  鍗曚綅鏄互寮曞鎵囧尯璧峰澶勭畻璧风殑瀛楄妭鏁般€?

  褰撳崗璁负 2.02 鎴栨洿楂橈紝鎴栧疄妯″紡浠ｇ爜鍔犺浇鍦?0x90000 鏃讹紝鍙互蹇界暐璇ュ瓧娈点€?

============	========================
Field name:	code32_start
Type:		modify (optional, reloc)
Offset/size:	0x214/4
Protocol:	2.00+
============	========================

  淇濇姢妯″紡涓嬭烦杞埌鐨勫湴鍧€銆傞粯璁ゅ€间负鍐呮牳鐨勫姞杞藉湴鍧€锛屽紩瀵煎姞杞界▼搴忓彲鐢ㄥ畠鏉ョ‘瀹氭纭殑鍔犺浇鍦板潃銆?

  璇ュ瓧娈靛彲鍑轰簬涓や釜鐩殑琚慨鏀癸細

    1. 浣滀负寮曞鍔犺浇绋嬪簭閽╁瓙锛堣涓嬫枃"楂樼骇寮曞鍔犺浇绋嬪簭閽╁瓙"锛夈€?

    2. 濡傛灉涓€涓笉瀹夎閽╁瓙鐨勫紩瀵煎姞杞界▼搴忓皢鍙噸瀹氫綅鍐呮牳鍔犺浇鍒伴潪鏍囧噯鍦板潃锛屽垯蹇呴』淇敼璇ュ瓧娈典互鎸囧悜鍔犺浇鍦板潃銆?

============	==================
Field name:	ramdisk_image
Type:		write (obligatory)
Offset/size:	0x218/4
Protocol:	2.00+
============	==================

  鍒濆 ramdisk 鎴?ramfs 鐨?32 浣嶇嚎鎬у湴鍧€銆傚鏋滄病鏈夊垵濮?ramdisk/ramfs锛屽垯淇濇寔涓洪浂銆?

============	==================
Field name:	ramdisk_size
Type:		write (obligatory)
Offset/size:	0x21c/4
Protocol:	2.00+
============	==================

  鍒濆 ramdisk 鎴?ramfs 鐨勫ぇ灏忋€傚鏋滄病鏈夊垵濮?ramdisk/ramfs锛屽垯淇濇寔涓洪浂銆?

============	===============
Field name:	bootsect_kludge
Type:		kernel internal
Offset/size:	0x220/4
Protocol:	2.00+
============	===============

  璇ュ瓧娈靛凡搴熷純銆?

============	==================
Field name:	heap_end_ptr
Type:		write (obligatory)
Offset/size:	0x224/2
Protocol:	2.01+
============	==================

  灏嗚瀛楁璁句负 setup 鏍?鍫嗘湯灏撅紙浠庡疄妯″紡浠ｇ爜璧峰澶勭畻璧凤級鐨勫亸绉婚噺锛屽噺鍘?0x0200銆?

============	================
Field name:	ext_loader_ver
Type:		write (optional)
Offset/size:	0x226/1
Protocol:	2.02+
============	================

  璇ュ瓧娈电敤浣?type_of_loader 瀛楁涓増鏈彿鐨勬墿灞曘€傛€荤増鏈彿瑙嗕负 (type_of_loader & 0x0f) + (ext_loader_ver << 4)銆?

  璇ュ瓧娈电殑浣跨敤鍙栧喅浜庡紩瀵煎姞杞界▼搴忋€傚鏋滄湭鍐欏叆锛屽垯涓洪浂銆?

  2.6.31 涔嬪墠鐨勫唴鏍镐笉璇嗗埆璇ュ瓧娈碉紝浣嗗浜?2.02 鎴栨洿楂樼増鏈殑鍗忚锛屽啓鍏ュ畠鏄畨鍏ㄧ殑銆?

============	=====================================================
Field name:	ext_loader_type
Type:		write (obligatory if (type_of_loader & 0xf0) == 0xe0)
Offset/size:	0x227/1
Protocol:	2.02+
============	=====================================================

  璇ュ瓧娈电敤浣?type_of_loader 瀛楁涓被鍨嬪彿鐨勬墿灞曘€傚鏋?type_of_loader 涓殑绫诲瀷涓?0xE锛屽垯瀹為檯绫诲瀷涓?(ext_loader_type + 0x10)銆?

  濡傛灉 type_of_loader 涓殑绫诲瀷涓嶆槸 0xE锛屽垯蹇界暐璇ュ瓧娈点€?

  2.6.31 涔嬪墠鐨勫唴鏍镐笉璇嗗埆璇ュ瓧娈碉紝浣嗗浜?2.02 鎴栨洿楂樼増鏈殑鍗忚锛屽啓鍏ュ畠鏄畨鍏ㄧ殑銆?

============	==================
Field name:	cmd_line_ptr
Type:		write (obligatory)
Offset/size:	0x228/4
Protocol:	2.02+
============	==================

  灏嗚瀛楁璁句负鍐呮牳鍛戒护琛岀殑绾挎€у湴鍧€銆傚唴鏍稿懡浠よ鍙互浣嶄簬 setup 鍫嗘湯灏惧埌 0xA0000 涔嬮棿鐨勪换鎰忎綅缃紱瀹冧笉蹇呬笌瀹炴ā寮忎唬鐮佹湰韬綅浜庡悓涓€涓?64K 娈典腑銆傚嵆浣夸綘鐨勫紩瀵煎姞杞界▼搴忎笉鏀寔鍛戒护琛岋紝涔熻濉啓璇ュ瓧娈碉紝姝ゆ椂鍙互鎸囧悜涓€涓┖瀛楃涓诧紙鎴栬€呮洿濂斤紝鎸囧悜瀛楃涓?"auto"銆傦級濡傛灉璇ュ瓧娈典繚鎸佷负闆讹紝鍐呮牳灏嗗亣瀹氫綘鐨勫紩瀵煎姞杞界▼搴忎笉鏀寔 2.02+ 鍗忚銆?

============	===============
Field name:	initrd_addr_max
Type:		read
Offset/size:	0x22c/4
Protocol:	2.03+
============	===============

  鍒濆 ramdisk/ramfs 鍐呭鍙兘鍗犵敤鐨勬渶澶у湴鍧€銆傚浜?2.02 鎴栨洿鏃╃殑寮曞鍗忚锛屼笉瀛樺湪璇ュ瓧娈碉紝鏈€澶у湴鍧€涓?0x37FFFFFF銆傦紙璇ュ湴鍧€琚畾涔変负鏈€楂樺畨鍏ㄥ瓧鑺傜殑鍦板潃锛屽洜姝ゅ鏋滀綘鐨?ramdisk 鎭板ソ涓?131072 瀛楄妭闀匡紝涓旇瀛楁涓?0x37FFFFFF锛屽垯鍙互浠?0x37FE0000 寮€濮嬩綘鐨?ramdisk銆傦級

============	============================
Field name:	kernel_alignment
Type:		read/modify (reloc)
Offset/size:	0x230/4
Protocol:	2.05+ (read), 2.10+ (modify)
============	============================

  鍐呮牳鎵€闇€鐨勫榻愬崟浣嶏紙濡傛灉 relocatable_kernel 涓虹湡锛夈€備互涓庤瀛楁鍊间笉鍏煎鐨勫榻愭柟寮忓姞杞界殑鍙噸瀹氫綅鍐呮牳锛屼細鍦ㄥ唴鏍稿垵濮嬪寲鏈熼棿琚噸鏂板榻愩€?

  浠庡崗璁増鏈?2.10 寮€濮嬶紝杩欏弽鏄犱簡鍐呮牳涓鸿幏寰楁渶浣虫€ц兘鎵€鍋忓ソ鐨勫榻愶紱鍔犺浇鍣ㄥ彲浠ヤ慨鏀硅瀛楁浠ュ厑璁歌緝灏忕殑瀵归綈銆傝鍙傝涓嬫枃鐨?min_alignment 鍜?pref_address 瀛楁銆?

============	==================
Field name:	relocatable_kernel
Type:		read (reloc)
Offset/size:	0x234/1
Protocol:	2.05+
============	==================

  濡傛灉璇ュ瓧娈甸潪闆讹紝鍒欏唴鏍哥殑淇濇姢妯″紡閮ㄥ垎鍙互鍔犺浇鍒颁换浣曟弧瓒?kernel_alignment 瀛楁鐨勫湴鍧€銆傚姞杞藉悗锛屽紩瀵煎姞杞界▼搴忓繀椤诲皢 code32_start 瀛楁璁剧疆涓烘寚鍚戝凡鍔犺浇鐨勪唬鐮侊紝鎴栨寚鍚戜竴涓紩瀵煎姞杞界▼搴忛挬瀛愩€?

============	=============
Field name:	min_alignment
Type:		read (reloc)
Offset/size:	0x235/1
Protocol:	2.10+
============	=============

  濡傛灉璇ュ瓧娈甸潪闆讹紝鍒欎互 2 鐨勫箓琛ㄧず鍐呮牳鍚姩鎵€闇€鐨勶紙涓庡亸濂界浉瀵圭殑锛夋渶灏忓榻愩€傚鏋滃紩瀵煎姞杞界▼搴忎娇鐢ㄤ簡璇ュ瓧娈碉紝鍒欏簲鏇存柊濡備笅

```
   kernel_alignment = 1 << min_alignment;

  杩囧害鏈榻愮殑鍐呮牳鍙兘浼氬甫鏉ョ浉褰撳ぇ鐨勬€ц兘浠ｄ环銆傚洜姝わ紝鍔犺浇鍣ㄩ€氬父搴斿皾璇曚粠 kernel_alignment 鍒拌瀵归綈涔嬮棿鐨勬瘡涓€涓?2 鐨勫箓瀵归綈銆?

```

============	==========
Field name:	xloadflags
Type:		read
Offset/size:	0x236/2
Protocol:	2.12+
============	==========

  璇ュ瓧娈垫槸涓€涓綅鎺╃爜銆?

  Bit 0 (read):	XLF_KERNEL_64

 - 濡傛灉涓?1锛屽垯鍐呮牳鍦?0x200 澶勫叿鏈変紶缁熺殑 64 浣嶅叆鍙ｇ偣銆?

  Bit 1 (read): XLF_CAN_BE_LOADED_ABOVE_4G

        - 濡傛灉涓?1锛屽垯 kernel/boot_params/cmdline/ramdisk 鍙互浣嶄簬 4G 浠ヤ笂銆?

  Bit 2 (read):	XLF_EFI_HANDOVER_32

 - 濡傛灉涓?1锛屽垯鍐呮牳鏀寔浣嶄簬 handover_offset 鐨?32 浣?EFI 鍒囨崲鍏ュ彛鐐广€?

  Bit 3 (read): XLF_EFI_HANDOVER_64

 - 濡傛灉涓?1锛屽垯鍐呮牳鏀寔浣嶄簬 handover_offset + 0x200 鐨?64 浣?EFI 鍒囨崲鍏ュ彛鐐广€?

  Bit 4 (read): XLF_EFI_KEXEC

 - 濡傛灉涓?1锛屽垯鍐呮牳鏀寔甯︽湁 EFI 杩愯鏃舵敮鎸佺殑 kexec EFI 寮曞銆?


============	============
Field name:	cmdline_size
Type:		read
Offset/size:	0x238/4
Protocol:	2.06+
============	============

  鍛戒护琛岀殑鏈€澶уぇ灏忥紙涓嶅惈缁撳熬鐨勯浂锛夈€傝繖鎰忓懗鐫€鍛戒护琛屾渶澶氬彲鍖呭惈 cmdline_size 涓瓧绗︺€傚浜?2.05 鍙婃洿鏃╃殑鍗忚鐗堟湰锛屾渶澶уぇ灏忎负 255銆?

============	====================================
Field name:	hardware_subarch
Type:		write (optional, defaults to x86/PC)
Offset/size:	0x23c/4
Protocol:	2.07+
============	====================================

  鍦ㄥ崐铏氭嫙鍖栫幆澧冧腑锛屼腑鏂鐞嗐€侀〉琛ㄥ鐞嗕互鍙婅闂繘绋嬫帶鍒跺瘎瀛樺櫒绛夊簳灞傜‖浠舵灦鏋勯儴鍒嗛渶瑕佷互涓嶅悓鏂瑰紡瀹屾垚銆傝瀛楁鍏佽寮曞鍔犺浇绋嬪簭鍛婄煡鍐呮牳鎴戜滑姝ｅ浜庤繖浜涚幆澧冧箣涓€涓€?

  ==========	==============================
  0x00000000	榛樿鐨?x86/PC 鐜
  0x00000001	lguest
  0x00000002	Xen
  0x00000003	Intel MID (Moorestown, CloverTrail, Merrifield, Moorefield)
  0x00000004	CE4100 TV Platform
  ==========	==============================

============	=========================
Field name:	hardware_subarch_data
Type:		write (subarch-dependent)
Offset/size:	0x240/8
Protocol:	2.07+
============	=========================

  鎸囧悜鐗瑰畾浜庣‖浠跺瓙鏋舵瀯鐨勬暟鎹殑鎸囬拡銆傝瀛楁鍦ㄩ粯璁ょ殑 x86/PC 鐜涓洰鍓嶆湭浣跨敤锛岃鍕夸慨鏀广€?

============	==============
Field name:	payload_offset
Type:		read
Offset/size:	0x248/4
Protocol:	2.08+
============	==============

  濡傛灉闈為浂锛屽垯璇ュ瓧娈靛寘鍚粠淇濇姢妯″紡浠ｇ爜璧峰澶勫埌鏈夋晥杞借嵎鐨勫亸绉婚噺銆?

  鏈夋晥杞借嵎鍙兘琚帇缂┿€傚帇缂╀笌鏈帇缂╂暟鎹殑鏍煎紡閮藉簲浣跨敤鏍囧噯榄旀暟鏉ョ‘瀹氥€傚綋鍓嶆敮鎸佺殑鍘嬬缉鏍煎紡鏈?gzip锛堥瓟鏁?1F 8B 鎴?1F 9E锛夈€乥zip2锛堥瓟鏁?42 5A锛夈€丩ZMA锛堥瓟鏁?5D 00锛夈€乆Z锛堥瓟鏁?FD 37锛夈€丩Z4锛堥瓟鏁?02 21锛夊拰 ZSTD锛堥瓟鏁?28 B5锛夈€傛湭鍘嬬缉鐨勬湁鏁堣浇鑽风洰鍓嶅缁堟槸 ELF锛堥瓟鏁?7F 45 4C 46锛夈€?

============	==============
Field name:	payload_length
Type:		read
Offset/size:	0x24c/4
Protocol:	2.08+
============	==============

  鏈夋晥杞借嵎鐨勯暱搴︺€?

============	===============
Field name:	setup_data
Type:		write (special)
Offset/size:	0x250/8
Protocol:	2.09+
============	===============

  鎸囧悜浠?NULL 缁撳熬鐨?struct setup_data 鍗曞悜閾捐〃鐨?64 浣嶇墿鐞嗘寚閽堛€傝繖鐢ㄤ簬瀹氫箟鏇村叿鎵╁睍鎬х殑寮曞鍙傛暟浼犻€掓満鍒躲€俿truct setup_data 鐨勫畾涔変负

```
   struct setup_data {
	__u64 next;
	__u32 type;
	__u32 len;
	__u8 data[];
   }

```

  鍏朵腑锛宯ext 鏄寚鍚戦摼琛ㄤ笅涓€涓妭鐐圭殑 64 浣嶇墿鐞嗘寚閽堬紝鏈€鍚庝竴涓妭鐐圭殑 next 瀛楁涓?0锛泃ype 鐢ㄤ簬鏍囪瘑 data 鐨勫唴瀹癸紱len 鏄?data 瀛楁鐨勯暱搴︼紱data 淇濆瓨鐪熸鐨勬湁鏁堣浇鑽枫€?

  璇ラ摼琛ㄥ彲鑳藉湪寮曞杩囩▼鐨勫涓幆鑺傝淇敼銆傚洜姝わ紝淇敼璇ラ摼琛ㄦ椂锛屽簲濮嬬粓鑰冭檻閾捐〃宸茬粡鍖呭惈鑺傜偣鐨勬儏褰€?

  setup_data 鐢ㄤ簬鏋佸ぇ鏁版嵁瀵硅薄鏃舵湁浜涗笉渚匡紝杩欐棦鏄洜涓?setup_data 澶撮儴蹇呴』涓庢暟鎹璞＄浉閭伙紝涔熸槸鍥犱负瀹冨彧鏈変竴涓?32 浣嶉暱搴﹀瓧娈点€傜劧鑰岋紝寮曞杩囩▼鐨勪腑闂撮樁娈甸渶瑕佹湁鍔炴硶璇嗗埆鍝簺鍐呭瓨鍧楄鍐呮牳鏁版嵁鍗犵敤锛岃繖涓€鐐瑰緢閲嶈銆?

  鍥犳锛屽崗璁?2.15 寮曞叆浜?setup_indirect 缁撴瀯浣撳拰 SETUP_INDIRECT 绫诲瀷锛?

```
   struct setup_indirect {
	__u32 type;
	__u32 reserved;		/* Reserved, must be set to zero. */
	__u64 len;
	__u64 addr;
   };

```

  type 鎴愬憳鏄?SETUP_INDIRECT | SETUP_* 绫诲瀷銆備絾瀹冧笉鑳芥槸 SETUP_INDIRECT 鑷韩锛屽洜涓哄皢 setup_indirect 鍋氭垚鏍戝舰缁撴瀯鍙兘浼氬湪闇€瑕佽В鏋愬畠鐨勫湴鏂规秷鑰楀ぇ閲忔爤绌洪棿锛岃€屽湪寮曞涓婁笅鏂囦腑鏍堢┖闂村彲鑳芥湁闄愩€?

  涓嬮潰涓句緥璇存槑濡備綍浣跨敤 setup_indirect 鎸囧悜 SETUP_E820_EXT 鏁版嵁銆傛鏃?setup_data 鍜?setup_indirect 灏嗗涓嬫墍绀猴細

```
   struct setup_data {
	.next = 0,	/* or <addr_of_next_setup_data_struct> */
	.type = SETUP_INDIRECT,
	.len = sizeof(setup_indirect),
	.data[sizeof(setup_indirect)] = (struct setup_indirect) {
		.type = SETUP_INDIRECT | SETUP_E820_EXT,
		.reserved = 0,
		.len = <len_of_SETUP_E820_EXT_data>,
		.addr = <addr_of_SETUP_E820_EXT_data>,
	},
   }

```

     SETUP_INDIRECT | SETUP_NONE 瀵硅薄鏃犳硶涓?SETUP_INDIRECT 鏈韩鏄庣‘鍖哄垎銆傚洜姝わ紝寮曞鍔犺浇绋嬪簭涓嶈兘鎻愪緵姝ょ被瀵硅薄銆?

============	============
Field name:	pref_address
Type:		read (reloc)
Offset/size:	0x258/8
Protocol:	2.10+
============	============

  濡傛灉璇ュ瓧娈甸潪闆讹紝鍒欒〃绀哄唴鏍稿亸濂界殑鍔犺浇鍦板潃銆傚彲閲嶅畾浣嶇殑寮曞鍔犺浇绋嬪簭搴斿敖鍙兘灏濊瘯鍦ㄨ鍦板潃鍔犺浇銆?

  涓嶅彲閲嶅畾浣嶇殑鍐呮牳灏嗘棤鏉′欢鍦扮Щ鍔ㄨ嚜韬苟鍦ㄨ鍦板潃杩愯銆傚彲閲嶅畾浣嶅唴鏍稿鏋滃姞杞藉湪璇ュ湴鍧€浠ヤ笅锛屽垯浼氬皢鑷韩绉诲姩鍒拌鍦板潃銆?

============	=======
Field name:	init_size
Type:		read
Offset/size:	0x260/4
============	=======

  璇ュ瓧娈垫寚绀轰粠鍐呮牳杩愯鏃惰捣濮嬪湴鍧€寮€濮嬨€佸唴鏍稿湪鑳藉妫€鏌ュ叾鍐呭瓨鏄犲皠涔嬪墠鎵€闇€鐨勭嚎鎬ц繛缁唴瀛樺ぇ灏忋€傝繖涓庡唴鏍稿惎鍔ㄦ墍闇€鐨勬€诲唴瀛樹笉鏄悓涓€鍥炰簨锛屼絾鍙噸瀹氫綅鐨勫紩瀵煎姞杞界▼搴忓彲鐢ㄥ畠鏉ュ府鍔╀负鍐呮牳閫夋嫨涓€涓畨鍏ㄧ殑鍔犺浇鍦板潃銆?

```
   if (relocatable_kernel) {
	if (load_address < pref_address)
		load_address = pref_address;
	runtime_start = align_up(load_address, kernel_alignment);
   } else {
	runtime_start = pref_address;
   }

```

鍥犳锛屾墍闇€鍐呭瓨绐楀彛鐨勪綅缃拰澶у皬鍙€氳繃浠ヤ笅鏂瑰紡浼扮畻锛?

```
   memory_window_start = runtime_start;
   memory_window_size = init_size;

```

============	===============
Field name:	handover_offset
Type:		read
Offset/size:	0x264/4
============	===============

  璇ュ瓧娈垫槸浠庡唴鏍告槧鍍忚捣濮嬪鍒?EFI 浜ゆ帴鍗忚鍏ュ彛鐐圭殑鍋忕Щ閲忋€備娇鐢?EFI 浜ゆ帴鍗忚寮曞鍐呮牳鐨勫紩瀵煎姞杞界▼搴忓簲璺宠浆鍒拌鍋忕Щ閲忋€?

  璇﹁涓嬫枃"EFI 浜ゆ帴鍗忚"銆?

============	==================
Field name:	kernel_info_offset
Type:		read
Offset/size:	0x268/4
Protocol:	2.15+
============	==================

  璇ュ瓧娈垫槸浠庡唴鏍告槧鍍忚捣濮嬪鍒?kernel_info 鐨勫亸绉婚噺銆俴ernel_info 缁撴瀯宓屽叆鍦?Linux 鏄犲儚鐨勬湭鍘嬬缉淇濇姢妯″紡鍖哄煙涓€?


## kernel_info


鍚勫ご閮ㄤ箣闂寸殑鍏崇郴绫讳技浜庡悇绉嶆暟鎹?

```
  setup_header = .data
  boot_params/setup_data = .bss

```

```
  kernel_info = .rodata

```

闀挎湡浠ユ潵锛岀敱浜庣己涔忔浛浠ｆ柟妗堚€斺€斿挨鍏舵槸鍦ㄦ棭鏈熲€斺€斾互鍙婃儻鎬э紝鎴戜滑涓€鐩村湪锛堟互鐢級.data 瀛樻斁鏈彲鏀惧叆 .rodata 鎴?.bss 鐨勫唴瀹广€傛澶栵紝BIOS stub 璐熻矗鍒涘缓 boot_params锛屽洜姝ゅ畠瀵逛簬鍩轰簬 BIOS 鐨勫姞杞藉櫒骞朵笉鍙敤锛堜笉杩?setup_data 鍙敤锛夈€?

setup_header 鍥?2 瀛楄妭璺宠浆瀛楁鐨勫鍧€鑼冨洿锛堝畠鍚屾椂鍏呭綋缁撴瀯鐨勯暱搴﹀瓧娈碉級浠ュ強 struct boot_params 涓繀椤荤敱淇濇姢妯″紡鍔犺浇鍣ㄦ垨 BIOS stub 灏嗗叾澶嶅埗杩涘幓鐨?绌烘礊"澶у皬锛岃姘镐箙闄愬埗鍦?144 瀛楄妭銆傚畠鐩墠闀?119 瀛楄妭锛岀暀缁欐垜浠殑鍙湁闈炲父瀹濊吹鐨?25 涓瓧鑺傘€傝嫢涓嶅畬鍏ㄤ慨璁㈠紩瀵煎崗璁€佺牬鍧忓悜鍚庡吋瀹癸紝杩欐槸鏃犳硶淇鐨勩€?

boot_params 鏈韩闄愪簬 4096 瀛楄妭锛屼絾鍙互閫氳繃娣诲姞 setup_data 鏉＄洰浠绘剰鎵╁睍銆傚畠涓嶈兘鐢ㄤ簬浼犺揪鍐呮牳鏄犲儚鐨勫睘鎬э紝鍥犱负瀹冩槸 .bss 涓旀病鏈夋槧鍍忔彁渚涚殑鍐呭銆?

kernel_info 閫氳繃涓哄唴鏍告槧鍍忎俊鎭彁渚涗竴涓彲鎵╁睍鐨勪綅缃潵瑙ｅ喅杩欎竴闂銆傚畠鏄彧璇荤殑锛屽洜涓哄唴鏍镐笉鑳戒緷璧栧紩瀵煎姞杞界▼搴忓皢鍏跺唴瀹瑰鍒跺埌浠讳綍鍦版柟锛屼絾娌″叧绯伙紱濡傛灉纭湁蹇呰锛屽畠浠嶇劧鍙互鍖呭惈閭ｄ簺鍚敤鐨勫紩瀵煎姞杞界▼搴忓簲澶嶅埗鍒?setup_data 鍧椾腑鐨勬暟鎹」銆?

鎵€鏈?kernel_info 鏁版嵁閮藉簲鏄缁撴瀯鐨勪竴閮ㄥ垎銆傚畾闀挎暟鎹繀椤绘斁鍦?kernel_info_var_len_data 鏍囩涔嬪墠銆傚彉闀挎暟鎹繀椤绘斁鍦?kernel_info_var_len_data 鏍囩涔嬪悗銆傛瘡涓彉闀挎暟鎹潡閮藉繀椤?

```
  kernel_info:
	.ascii  "LToP"		/* Header, Linux top (structure). */
	.long   kernel_info_var_len_data - kernel_info
	.long   kernel_info_end - kernel_info
	.long   0x01234567	/* Some fixed size data for the bootloaders. */
  kernel_info_var_len_data:
  example_struct:		/* Some variable size data for the bootloaders. */
	.ascii  "0123"		/* Header/Magic. */
	.long   example_struct_end - example_struct
	.ascii  "Struct"
	.long   0x89012345
  example_struct_end:
  example_strings:		/* Some variable size data for the bootloaders. */
	.ascii  "ABCD"		/* Header/Magic. */
	.long   example_strings_end - example_strings
	.asciz  "String_0"
	.asciz  "String_1"
  example_strings_end:
  kernel_info_end:

```

杩欐牱锛宬ernel_info 灏辨槸涓€涓嚜鍖呭惈鐨?blob銆?

     姣忎釜鍙橀暱鏁版嵁澶撮儴/榄旀暟鍙互鏄换鎰?4 瀛楃瀛楃涓诧紙瀛楃涓叉湯灏句笉甯?\0锛夛紝涓斾笉寰椾笌鐜版湁鐨勫彉闀挎暟鎹ご閮?榄旀暟鍐茬獊銆?


## kernel_info 瀛楁璇﹁В


============	========
Field name:	header
Offset/size:	0x0000/4
============	========

  鍖呭惈榄旀暟 "LToP"锛?x506f544c锛夈€?

============	========
Field name:	size
Offset/size:	0x0004/4
============	========

  璇ュ瓧娈靛寘鍚?kernel_info 鐨勫ぇ灏忥紙鍚?kernel_info.header锛夈€傚畠涓嶈鍏?kernel_info.kernel_info_var_len_data 鐨勫ぇ灏忋€傚紩瀵煎姞杞界▼搴忓簲浣跨敤璇ュ瓧娈垫潵妫€娴?kernel_info 涓彈鏀寔鐨勫畾闀垮瓧娈典互鍙?kernel_info.kernel_info_var_len_data 鐨勮捣濮嬩綅缃€?

============	========
Field name:	size_total
Offset/size:	0x0008/4
============	========

  璇ュ瓧娈靛寘鍚?kernel_info 鐨勫ぇ灏忥紙鍚?kernel_info.header 鍜?kernel_info.kernel_info_var_len_data锛夈€?

============	==============
Field name:	setup_type_max
Offset/size:	0x000c/4
============	==============

  璇ュ瓧娈靛寘鍚?setup_data 鍜?setup_indirect 缁撴瀯浣撴墍鍏佽鐨勬渶澶х被鍨嬨€?


## 鍐呮牳鍛戒护琛?


鍐呮牳鍛戒护琛屽凡鎴愪负寮曞鍔犺浇绋嬪簭涓庡唴鏍搁€氫俊鐨勯噸瑕佹柟寮忋€傚叾涓竴浜涢€夐」涔熶笌寮曞鍔犺浇绋嬪簭鏈韩鐩稿叧锛岃瑙佷笅鏂?鐗规畩鍛戒护琛岄€夐」"銆傚唴鏍稿懡浠よ鏄竴涓互 NUL 缁撳熬鐨勫瓧绗︿覆銆傛渶澶ч暱搴﹀彲浠?cmdline_size 瀛楁鑾峰彇銆傚湪 2.06 鍗忚鐗堟湰涔嬪墠锛屾渶澶ч暱搴︿负 255 涓瓧绗︺€傝繃闀跨殑瀛楃涓蹭細琚唴鏍歌嚜鍔ㄦ埅鏂€?

濡傛灉寮曞鍗忚鐗堟湰涓?2.02 鎴栨洿楂橈紝鍒欏唴鏍稿懡浠よ鐨勫湴鍧€鐢卞ご瀛楁 cmd_line_ptr 缁欏嚭锛堣涓婃枃锛夈€傝鍦板潃鍙互浣嶄簬 setup 鍫嗘湯灏惧埌 0xA0000 涔嬮棿鐨勪换鎰忎綅缃€?

濡傛灉鍗忚鐗堟湰**涓嶆槸** 2.02 鎴栨洿楂橈紝鍒欎娇鐢ㄤ互涓嬪崗璁緭鍏ュ唴鏍稿懡浠よ锛?

  - 鍦ㄥ亸绉?0x0020锛堝瓧锛夊鐨?"cmd_line_magic" 涓紝濉叆榄旀暟 0xA33F銆?

  - 鍦ㄥ亸绉?0x0022锛堝瓧锛夊鐨?"cmd_line_offset" 涓紝濉叆鍐呮牳鍛戒护琛岀殑鍋忕Щ閲忥紙鐩稿浜庡疄妯″紡鍐呮牳璧峰澶勶級銆?

  - 鍐呮牳鍛戒护琛?*蹇呴』**浣嶄簬 setup_move_size 鎵€瑕嗙洊鐨勫唴瀛樺尯鍩熷唴锛屽洜姝や綘鍙兘闇€瑕佽皟鏁磋瀛楁銆?


## 瀹炴ā寮忎唬鐮佺殑鍐呭瓨甯冨眬


瀹炴ā寮忎唬鐮侀渶瑕佽缃爤/鍫嗭紝骞跺垎閰嶇敤浜庡唴鏍稿懡浠よ鐨勫唴瀛樸€傝繖闇€瑕佸湪浣庡厗瀛楄妭涓疄妯″紡鍙闂殑鍐呭瓨涓畬鎴愩€?

闇€瑕佹敞鎰忕殑鏄紝鐜颁唬鏈哄櫒閫氬父鏈変竴涓浉褰撳ぇ鐨勬墿灞?BIOS 鏁版嵁鍖猴紙EBDA锛夈€傚洜姝わ紝寤鸿灏藉彲鑳藉皯鍦颁娇鐢ㄤ綆鍏嗗瓧鑺傚唴瀛樸€?

閬楁喚鐨勬槸锛屽湪浠ヤ笅鎯呭喌涓嬪繀椤讳娇鐢?0x90000 鍐呭瓨娈碉細

 - 鍔犺浇 zImage 鍐呮牳鏃讹紙(loadflags & 0x01) == 0锛夈€?
 - 鍔犺浇 2.01 鎴栨洿鏃╁紩瀵煎崗璁殑鍐呮牳鏃躲€?

     瀵逛簬 2.00 鍜?2.01 寮曞鍗忚锛屽疄妯″紡浠ｇ爜鍙互鍔犺浇鍒板彟涓€涓湴鍧€锛屼絾浼氬湪鍐呴儴閲嶅畾浣嶅埌 0x90000銆傚浜?"old"锛堟棫鐗堬級鍗忚锛屽疄妯″紡浠ｇ爜蹇呴』鍔犺浇鍦?0x90000銆?

鍦?0x90000 鍔犺浇鏃讹紝閬垮厤浣跨敤 0x9a000 浠ヤ笂鐨勫唴瀛樸€?

瀵逛簬 2.02 鎴栨洿楂樼増鏈殑寮曞鍗忚锛屽懡浠よ涓嶅繀涓庡疄妯″紡 setup 浠ｇ爜浣嶄簬鍚屼竴涓?64K 娈典腑锛涘洜姝ゅ彲浠ュ皢鏁翠釜 64K 娈甸兘缁欐爤/鍫嗭紝骞跺皢鍛戒护琛屾斁鍦ㄥ畠涓婇潰銆?

鍐呮牳鍛戒护琛屼笉搴斾綅浜庡疄妯″紡浠ｇ爜涓嬫柟锛屼篃涓嶅簲浣嶄簬楂樼鍐呭瓨涓€?


## 寮曞閰嶇疆绀轰緥


浣滀负绀轰緥閰嶇疆锛屽亣璁惧疄妯″紡娈靛叿鏈変互涓嬪竷灞€銆?

    褰撳姞杞藉湪 0x90000 浠ヤ笅鏃讹紝浣跨敤鏁翠釜娈碉細

        =============	===================
	0x0000-0x7fff	Real mode kernel
	0x8000-0xdfff	Stack and heap
	0xe000-0xffff	Kernel command line
	=============	===================

    褰撳姞杞藉湪 0x90000 鎴栧崗璁増鏈负 2.01 鎴栨洿鏃╂椂锛?

	=============	===================
	0x0000-0x7fff	Real mode kernel
	0x8000-0x97ff	Stack and heap
	0x9800-0x9fff	Kernel command line
	=============	===================

```
  unsigned long base_ptr;	/* base address for real-mode segment */

  if (setup_sects == 0)
	setup_sects = 4;

  if (protocol >= 0x0200) {
	type_of_loader = <type code>;
	if (loading_initrd) {
		ramdisk_image = <initrd_address>;
		ramdisk_size = <initrd_size>;
	}

	if (protocol >= 0x0202 && loadflags & 0x01)
		heap_end = 0xe000;
	else
		heap_end = 0x9800;

	if (protocol >= 0x0201) {
		heap_end_ptr = heap_end - 0x200;
		loadflags |= 0x80;		/* CAN_USE_HEAP */
	}

	if (protocol >= 0x0202) {
		cmd_line_ptr = base_ptr + heap_end;
		strcpy(cmd_line_ptr, cmdline);
	} else {
		cmd_line_magic	= 0xA33F;
		cmd_line_offset = heap_end;
		setup_move_size = heap_end + strlen(cmdline) + 1;
		strcpy(base_ptr + cmd_line_offset, cmdline);
	}
  } else {
	/* Very old kernel */

	heap_end = 0x9800;

	cmd_line_magic	= 0xA33F;
	cmd_line_offset = heap_end;

	/* A very old kernel MUST have its real-mode code loaded at 0x90000 */
	if (base_ptr != 0x90000) {
		/* Copy the real-mode kernel */
		memcpy(0x90000, base_ptr, (setup_sects + 1) * 512);
		base_ptr = 0x90000;		 /* Relocated */
	}

	strcpy(0x90000 + cmd_line_offset, cmdline);

	/* It is recommended to clear memory up to the 32K mark */
	memset(0x90000 + (setup_sects + 1) * 512, 0, (64 - (setup_sects + 1)) * 512);
  }


```

## 鍔犺浇鍐呮牳鐨勫叾浣欓儴鍒?


32 浣嶏紙闈炲疄妯″紡锛夊唴鏍镐粠鍐呮牳鏂囦欢涓亸绉?(setup_sects + 1) * 512 澶勫紑濮嬶紙鍐嶆寮鸿皟锛屽鏋?setup_sects == 0锛岀湡瀹炲€间负 4锛夈€傚浜?Image/zImage 鍐呮牳锛屽畠搴斿姞杞藉湪鍦板潃 0x10000锛涘浜?bzImage 鍐呮牳锛屽簲鍔犺浇鍦?0x100000銆傚鏋滃崗璁?>= 2.00 涓?0x01

```
  is_bzImage = (protocol >= 0x0200) && (loadflags & 0x01);
  load_address = is_bzImage ? 0x100000 : 0x10000;

```

Image/zImage 鍐呮牳鏈€澶у彲杈?512K锛屽洜姝や細浣跨敤鏁翠釜 0x10000-0x90000 鍐呭瓨鑼冨洿銆傝繖鎰忓懗鐫€杩欎簺鍐呮牳鍑犱箮蹇呴』灏嗗疄妯″紡閮ㄥ垎鍔犺浇鍦?0x90000銆俠zImage 鍐呮牳鍒欏厑璁告洿澶х殑鐏垫椿鎬с€?

## 鐗规畩鍛戒护琛岄€夐」


濡傛灉寮曞鍔犺浇绋嬪簭鎻愪緵鐨勫懡浠よ鐢辩敤鎴疯緭鍏ワ紝鐢ㄦ埛鍙兘鏈熸湜浠ヤ笅鍛戒护琛岄€夐」鑳芥甯稿伐浣溿€傚嵆浣垮苟闈炴墍鏈夐€夐」瀵瑰唴鏍搁兘鐪熸鏈夋剰涔夛紝閫氬父涔熶笉搴斾粠鍐呮牳鍛戒护琛屼腑鍒犻櫎瀹冧滑銆傞渶瑕佷负寮曞鍔犺浇绋嬪簭鏈韩娣诲姞棰濆鍛戒护琛岄€夐」鐨勫紩瀵煎姞杞界▼搴忎綔鑰咃紝搴斿湪 Documentation/admin-guide/kernel-parameters.rst 涓敞鍐屽畠浠紝浠ョ‘淇濆畠浠幇鍦ㄦ垨灏嗘潵閮戒笉浼氫笌瀹為檯鍐呮牳閫夐」鍐茬獊銆?

  vga=<mode>
	<mode> 鍙互鏄暣鏁帮紙閲囩敤 C 琛ㄧず娉曪紝鍙负鍗佽繘鍒躲€佸叓杩涘埗鎴栧崄鍏繘鍒讹級锛屼篃鍙互鏄瓧绗︿覆 "normal"锛堝嵆 0xFFFF锛夈€?ext"锛堝嵆 0xFFFE锛夋垨 "ask"锛堝嵆 0xFFFD锛変箣涓€銆傝鍊煎簲濉叆 vid_mode 瀛楁锛屽洜涓哄唴鏍稿湪瑙ｆ瀽鍛戒护琛屼箣鍓嶅氨浼氫娇鐢ㄥ畠銆?

  mem=<size>
	<size> 鏄噰鐢?C 琛ㄧず娉曘€佸悗闈㈠彲閫夋嫨鎬у湴璺燂紙澶у皬鍐欎笉鏁忔劅锛塊銆丮銆丟銆乀銆丳 鎴?E锛堝垎鍒〃绀?<< 10銆?< 20銆?< 30銆?< 40銆?< 50 鎴?<< 60锛夌殑鏁存暟銆傝繖鍚戝唴鏍告寚瀹氬唴瀛樼殑鏈銆傝繖浼氬奖鍝?initrd 鍙兘鐨勬斁缃綅缃紝鍥犱负 initrd 搴旀斁鍦ㄥ唴瀛樻湯绔檮杩戙€傛敞鎰忥紝杩欏悓鏃舵槸鍐呮牳**鍜?*寮曞鍔犺浇绋嬪簭鐨勪竴涓€夐」锛?

  initrd=<file>
	搴斿姞杞戒竴涓?initrd銆?file> 鐨勫惈涔夋樉鐒跺彇鍐充簬寮曞鍔犺浇绋嬪簭锛屽苟涓旀煇浜涘紩瀵煎姞杞界▼搴忥紙渚嬪 LILO锛夋病鏈夎繖鏍风殑鍛戒护銆?

姝ゅ锛屼竴浜涘紩瀵煎姞杞界▼搴忎細鍚戠敤鎴锋寚瀹氱殑鍛戒护琛屾坊鍔犱互涓嬮€夐」锛?

  BOOT_IMAGE=<file>
	琚姞杞界殑寮曞鏄犲儚銆傚悓鏍凤紝<file> 鐨勫惈涔夋樉鐒跺彇鍐充簬寮曞鍔犺浇绋嬪簭銆?

  auto
	鍐呮牳鍦ㄦ病鏈夌敤鎴锋槑纭共棰勭殑鎯呭喌涓嬪惎鍔ㄣ€?

濡傛灉杩欎簺閫夐」鐢卞紩瀵煎姞杞界▼搴忔坊鍔狅紝寮虹儓寤鸿灏嗗畠浠斁鍦?*鏈€鍓嶉潰**锛屼綅浜庣敤鎴锋寚瀹氭垨閰嶇疆鎸囧畾鐨勫懡浠よ涔嬪墠銆傚惁鍒欙紝"init=/bin/sh" 浼氳 "auto" 閫夐」骞叉壈銆?


## 杩愯鍐呮牳


鍐呮牳閫氳繃璺宠浆鍒板唴鏍稿叆鍙ｇ偣鏉ュ惎鍔紝璇ュ叆鍙ｇ偣浣嶄簬璺濆疄妯″紡鍐呮牳璧峰澶勭殑**娈?*鍋忕Щ 0x20銆傝繖鎰忓懗鐫€濡傛灉浣犲皢瀹炴ā寮忓唴鏍镐唬鐮佸姞杞藉湪 0x90000锛屽唴鏍稿叆鍙ｇ偣灏辨槸 9020:0000銆傝繘鍏ユ椂锛宒s = es = ss 搴旀寚鍚戝疄妯″紡鍐呮牳浠ｇ爜鐨勮捣濮嬪锛堝鏋滀唬鐮佸姞杞藉湪 0x90000锛屽垯涓?0x9000锛夛紝sp 搴旀纭缃紝閫氬父鎸囧悜鍫嗛《锛屽苟涓斾腑鏂簲琚鐢ㄣ€傛澶栵紝涓洪槻姝㈠唴鏍镐腑鐨?bug锛屽缓璁紩瀵煎姞杞界▼搴忚缃?fs = gs = ds = es = ss銆?

```
  /*
   * Note: in the case of the "old" kernel protocol, base_ptr must
   * be == 0x90000 at this point; see the previous sample code.
   */
  seg = base_ptr >> 4;

  cli();			/* Enter with interrupts disabled! */

  /* Set up the real-mode kernel stack */
  _SS = seg;
  _SP = heap_end;

  _DS = _ES = _FS = _GS = seg;
  jmp_far(seg + 0x20, 0);	/* Run the kernel */

```

濡傛灉浣犵殑寮曞鎵囧尯璁块棶杞洏椹卞姩鍣紝寤鸿鍦ㄥ唴鏍歌繍琛屼箣鍓嶅叧闂蒋鐩橀┈杈撅紝鍥犱负鍐呮牳寮曞浼氫娇涓柇淇濇寔鍏抽棴锛屼粠鑰岄┈杈句笉浼氳鍏抽棴锛岀壒鍒槸褰撹鍔犺浇鐨勫唴鏍稿皢杞洏椹卞姩浣滀负鎸夐渶鍔犺浇妯″潡鏃讹紒


## 楂樼骇寮曞鍔犺浇绋嬪簭閽╁瓙


濡傛灉寮曞鍔犺浇绋嬪簭杩愯鍦ㄧ壒鍒伓鍔ｇ殑鐜涓紙渚嬪杩愯鍦?DOS 涓嬬殑 LOADLIN锛夛紝鍙兘鏃犳硶閬靛惊鏍囧噯鐨勫唴瀛樹綅缃姹傘€傝繖鏍风殑寮曞鍔犺浇绋嬪簭鍙互浣跨敤浠ヤ笅閽╁瓙锛屽畠浠鏋滆璁剧疆锛屼細鍦ㄩ€傚綋鏃舵満鐢卞唴鏍歌皟鐢ㄣ€備娇鐢ㄨ繖浜涢挬瀛愬ぇ姒傚簲琚涓虹粷瀵圭殑鏈€鍚庢墜娈碉紒閲嶈锛氭墍鏈夐挬瀛愬湪琚皟鐢ㄦ椂閮藉繀椤讳繚鐣?%esp銆?ebp銆?esi 鍜?%edi銆?

  realmode_swtch:
	A 16-bit real mode far subroutine invoked immediately before entering protected mode. The default routine disables NMI, so your routine should probably do so, too.

  code32_start:
	A 32-bit flat-mode routine **jumped** to immediately after the transition to protected mode, but before the kernel is uncompressed. No segments, except CS, are guaranteed to be set up (current kernels do, but older ones do not); you should set them up to BOOT_DS (0x18) yourself. After completing your hook, you should jump to the address that was in this field before your boot loader overwrote it (relocated, if appropriate.)


## 32 浣嶅紩瀵煎崗璁?


瀵逛簬浣跨敤鏌愪簺闈炰紶缁?BIOS 鐨勬柊鍨?BIOS锛堝 EFI銆丩inuxBIOS 绛夛級鐨勬満鍣ㄤ互鍙?kexec锛屽熀浜庝紶缁?BIOS 鐨?16 浣嶅疄妯″紡 setup 浠ｇ爜鏃犳硶浣跨敤锛屽洜姝ら渶瑕佸畾涔変竴涓?32 浣嶅紩瀵煎崗璁€?

鍦?32 浣嶅紩瀵煎崗璁腑锛屽姞杞?Linux 鍐呮牳鐨勭涓€姝ュ簲璇ユ槸璁剧疆寮曞鍙傛暟锛坰truct boot_params锛屼紶缁熶笂绉颁负 "zero page"/闆堕〉锛夈€俿truct boot_params 鐨勫唴瀛樺簲琚垎閰嶅苟鍒濆鍖栦负闆躲€傜劧鍚庯紝搴斿皢鍐呮牳鏄犲儚涓粠鍋忕Щ 0x01f1 寮€濮嬬殑 setup 澶村姞杞藉埌 struct boot_params 涓苟妫€鏌ャ€俿etup 澶寸殑鏈熬鍙寜浠ヤ笅鏂瑰紡璁＄畻锛?

```
  0x0202 + byte value at offset 0x0201

```

闄や簡鍍?16 浣嶅紩瀵煎崗璁偅鏍峰 struct boot_params 鐨?setup 澶磋繘琛岃/淇敼/鍐欎箣澶栵紝寮曞鍔犺浇绋嬪簭杩樺簲鎸夌収 Documentation/arch/x86/zero-page.rst 涓€绔犵殑鎻忚堪濉啓 struct boot_params 鐨勯檮鍔犲瓧娈点€傝缃ソ struct boot_params 鍚庯紝寮曞鍔犺浇绋嬪簭鍙互鍍?16 浣嶅紩瀵煎崗璁偅鏍峰姞杞?32/64 浣嶅唴鏍搞€傚湪 32 浣嶅紩瀵煎崗璁腑锛屽唴鏍搁€氳繃璺宠浆鍒?32 浣嶅唴鏍稿叆鍙ｇ偣鏉ュ惎鍔紝璇ュ叆鍙ｇ偣灏辨槸宸插姞杞界殑 32/64 浣嶅唴鏍哥殑璧峰鍦板潃銆傝繘鍏ユ椂锛孋PU 蹇呴』澶勪簬绂佺敤鍒嗛〉鐨?32 浣嶄繚鎶ゆā寮忥紱蹇呴』鍔犺浇涓€涓?GDT锛屽叾涓寘鍚€夋嫨鍣?__BOOT_CS(0x10) 鍜?__BOOT_DS(0x18) 鐨勬弿杩扮锛涗袱涓弿杩扮閮藉繀椤绘槸 4G 骞抽潰娈碉紱__BOOT_CS 蹇呴』鍏锋湁鎵ц/璇绘潈闄愶紝__BOOT_DS 蹇呴』鍏锋湁璇?鍐欐潈闄愶紱CS 蹇呴』鏄?__BOOT_CS锛孌S銆丒S銆丼S 蹇呴』鏄?__BOOT_DS锛涗腑鏂繀椤昏绂佺敤锛?esi 蹇呴』淇濆瓨 struct boot_params 鐨勫熀鍦板潃锛?ebp銆?edi 鍜?%ebx 蹇呴』涓洪浂銆?

## 64 浣嶅紩瀵煎崗璁?


瀵逛簬閰嶅 64 浣?CPU 鍜?64 浣嶅唴鏍哥殑鏈哄櫒锛屾垜浠彲浠ヤ娇鐢?64 浣嶅紩瀵煎姞杞界▼搴忥紝骞朵笖闇€瑕佷竴涓?64 浣嶅紩瀵煎崗璁€?

鍦?64 浣嶅紩瀵煎崗璁腑锛屽姞杞?Linux 鍐呮牳鐨勭涓€姝ュ簲璇ユ槸璁剧疆寮曞鍙傛暟锛坰truct boot_params锛屼紶缁熶笂绉颁负 "zero page"/闆堕〉锛夈€俿truct boot_params 鐨勫唴瀛樺彲浠ュ垎閰嶅湪浠绘剰浣嶇疆锛堢敋鑷?4G 浠ヤ笂锛夊苟鍒濆鍖栦负闆躲€傜劧鍚庯紝搴斿皢鍐呮牳鏄犲儚涓亸绉?0x01f1 澶勭殑 setup 澶村姞杞藉埌 struct boot_params 涓苟妫€鏌ャ€俿etup 澶寸殑鏈熬

```
  0x0202 + byte value at offset 0x0201

```

闄や簡鍍?16 浣嶅紩瀵煎崗璁偅鏍峰 struct boot_params 鐨?setup 澶磋繘琛岃/淇敼/鍐欎箣澶栵紝寮曞鍔犺浇绋嬪簭杩樺簲鎸夌収 Documentation/arch/x86/zero-page.rst 涓€绔犵殑鎻忚堪濉啓 struct boot_params 鐨勯檮鍔犲瓧娈点€傝缃ソ struct boot_params 鍚庯紝寮曞鍔犺浇绋嬪簭鍙互鍍?16 浣嶅紩瀵煎崗璁偅鏍峰姞杞?64 浣嶅唴鏍革紝浣嗗唴鏍稿彲浠ュ姞杞藉埌 4G 浠ヤ笂銆傚湪 64 浣嶅紩瀵煎崗璁腑锛屽唴鏍搁€氳繃璺宠浆鍒?64 浣嶅唴鏍稿叆鍙ｇ偣鏉ュ惎鍔紝璇ュ叆鍙ｇ偣鏄凡鍔犺浇鐨?64 浣嶅唴鏍哥殑璧峰鍦板潃鍔犱笂 0x200銆傝繘鍏ユ椂锛孋PU 蹇呴』澶勪簬鍚敤鍒嗛〉鐨?64 浣嶆ā寮忋€備粠宸插姞杞藉唴鏍哥殑璧峰鍦板潃璧枫€佸ぇ灏忎负 setup_header.init_size 鐨勮寖鍥达紝浠ュ強闆堕〉鍜屽懡浠よ缂撳啿鍖猴紝閮戒細鑾峰緱涓€鑷存€ф槧灏勶紙ident mapping锛夛紱蹇呴』鍔犺浇涓€涓?GDT锛屽叾涓寘鍚€夋嫨鍣?__BOOT_CS(0x10) 鍜?__BOOT_DS(0x18) 鐨勬弿杩扮锛涗袱涓弿杩扮閮藉繀椤绘槸 4G 骞抽潰娈碉紱__BOOT_CS 蹇呴』鍏锋湁鎵ц/璇绘潈闄愶紝__BOOT_DS 蹇呴』鍏锋湁璇?鍐欐潈闄愶紱CS 蹇呴』鏄?__BOOT_CS锛孌S銆丒S銆丼S 蹇呴』鏄?__BOOT_DS锛涗腑鏂繀椤昏绂佺敤锛?rsi 蹇呴』淇濆瓨 struct boot_params 鐨勫熀鍦板潃銆?

## EFI 浜ゆ帴鍗忚锛堝凡寮冪敤锛?


璇ュ崗璁厑璁稿紩瀵煎姞杞界▼搴忓皢鍒濆鍖栨帹杩熷埌 EFI boot stub銆傚紩瀵煎姞杞界▼搴忛渶瑕佷粠寮曞浠嬭川鍔犺浇鍐呮牳/initrd锛屽苟璺宠浆鍒?EFI 浜ゆ帴鍗忚鍏ュ彛鐐癸紝璇ュ叆鍙ｇ偣璺濈 startup_{32,64} 璧峰澶?hdr->handover_offset 瀛楄妭銆傚紩瀵煎姞杞界▼搴忓湪澶勭悊鑺傚榻愩€佸彲鎵ц鏄犲儚瓒呭嚭鏂囦欢鏈韩澶у皬鐨勫唴瀛樺崰鐢紝浠ュ強鍙兘褰卞搷璇ユ槧鍍忓湪 EFI 鍥轰欢鎻愪緵鐨勬墽琛屼笂涓嬫枃涓綔涓?PE/COFF 浜岃繘鍒舵纭繍琛岀殑 PE/COFF 澶翠换浣曞叾浠栨柟闈㈡椂锛屽繀椤婚伒瀹堝唴鏍哥殑 PE/COFF 鍏冩暟鎹€?

```
  void efi_stub_entry(void *handle, efi_system_table_t *table, struct boot_params *bp);

```

'handle' 鏄?EFI 鍥轰欢浼犻€掔粰寮曞鍔犺浇绋嬪簭鐨?EFI 鏄犲儚鍙ユ焺锛?table' 鏄?EFI 绯荤粺琛ㄢ€斺€斿畠浠槸 UEFI 瑙勮寖绗?2.3 鑺傛墍鎻忚堪鐨?浜ゆ帴鐘舵€?鐨勫墠涓や釜鍙傛暟銆?bp' 鏄紩瀵煎姞杞界▼搴忓垎閰嶇殑 boot params銆?

```
  - hdr.cmd_line_ptr
  - hdr.ramdisk_image (if applicable)
  - hdr.ramdisk_size  (if applicable)

```

鎵€鏈夊叾浠栧瓧娈靛簲涓洪浂銆侲FI 浜ゆ帴鍗忚宸插純鐢紝鍙栬€屼唬涔嬬殑鏄笅鏂囨弿杩扮殑鏅€?PE/COFF 鍏ュ彛鐐广€?


## PE/COFF 鍏ュ彛鐐?


褰撲娇鐢?`CONFIG_EFI_STUB=y` 缂栬瘧鏃讹紝鍐呮牳鍙互浣滀负鏅€氱殑 PE/COFF 浜岃繘鍒舵墽琛屻€傚疄鐜扮粏鑺傝鍙傞槄 Documentation/admin-guide/efi-stub.rst銆俿tub 鍔犺浇鍣ㄥ彲浠ラ€氳繃 UEFI 鍗忚璇锋眰 initrd銆傝浣挎鍔熻兘宸ヤ綔锛屽浐浠舵垨寮曞鍔犺浇绋嬪簭闇€瑕佹敞鍐屼竴涓彞鏌勶紝璇ュ彞鏌勬惡甯?`EFI_LOAD_FILE2` 鍗忚鐨勫疄鐜帮紝浠ュ強鏆撮湶 `LINUX_EFI_INITRD_MEDIA_GUID` 鍘傚晢濯掍綋璁惧璺緞鐨勮澶囪矾寰勫崗璁€傚湪杩欑鎯呭喌涓嬶紝閫氳繃 EFI stub 寮曞鐨勫唴鏍稿皢鍦ㄥ凡娉ㄥ唽鐨勫崗璁笂璋冪敤 **``LoadFile2``: LoadFile()** 鏂规硶锛屾寚绀哄浐浠跺皢 initrd 鍔犺浇鍒板唴鏍?EFI stub 閫夋嫨鐨勫唴瀛樹綅缃€傝繖绉嶆柟寮忎娇寰?EFI 寮曞鍔犺浇绋嬪簭鏃犻渶浜嗚В boot_params 鐨勫唴閮ㄨ〃绀猴紝涔熸棤闇€浜嗚В鍛戒护琛屼笌 ramdisk 鍦ㄥ唴瀛樹腑鏀剧疆浣嶇疆銆佹垨鍐呮牳鏄犲儚鏈韩鏀剧疆浣嶇疆鐨勪换浣曡姹?闄愬埗銆傛湁鍏崇ず渚嬪疄鐜帮紝璇峰弬闃?`the original u-boot implementation`_ 鎴?`the OVMF implementation`_銆?