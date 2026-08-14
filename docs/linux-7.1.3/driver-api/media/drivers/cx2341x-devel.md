锘?
## cx2341x 椹卞姩


### cx2341x 鑺墖涓婄殑鍐呭瓨


鏈妭鎻忚堪 cx2341x 鐨勫唴瀛樻槧灏勶紝骞惰褰曢儴鍒嗗瘎瀛樺櫒绌洪棿銆?


	杩欎簺淇℃伅鏄€氳繃鎼滅储鍐呭瓨鍜屽瘎瀛樺櫒寰楀埌鐨勶紝鍙兘涓嶆纭€佷篃鑲畾涓嶅畬鏁达紝涓斾粎鏄€氳繃鐢ㄥ涓嬪懡浠ゆ悳绱㈠唴瀛樼┖闂村緱鍑猴細

	.. code-block:: none

		ivtvctl -O min=0x02000000,max=0x020000ff

	鍥犳璇锋寜鍘熸牱鐪嬪緟锛屾垜涓€鐩村湪瀵绘壘鏇村鍐呭锛屽瘎瀛樺櫒绌洪棿寰堝ぇ :-)銆?

#### 鍐呭瓨鏄犲皠


cx2341x 閫氳繃 PCI BAR0锛堝熀鍦板潃瀵勫瓨鍣?0锛夋妸鍏舵暣涓?64M 鍐呭瓨绌洪棿鏆撮湶缁?PCI 涓绘満銆傝繖閲岀殑鍦板潃鏄浉瀵逛簬 BAR0 涓墍鎸佸湴鍧€鐨勫亸绉汇€?


	0x00000000-0x00ffffff Encoder memory space
	0x00000000-0x0003ffff Encode.rom
	???-???         MPEG buffer(s)
	???-???         Raw video capture buffer(s)
	???-???         Raw audio capture buffer(s)
	???-???         Display buffers (6 or 9)

	0x01000000-0x01ffffff Decoder memory space
	0x01000000-0x0103ffff Decode.rom
	???-???         MPEG buffers(s)
	0x0114b000-0x0115afff Audio.rom (deprecated锛?

	0x02000000-0x0200ffff Register Space

#### 瀵勫瓨鍣?


瀵勫瓨鍣ㄥ崰鎹粠 BAR0 鍋忕Щ 0x02000000 寮€濮嬬殑 64k 绌洪棿銆傛墍鏈夎繖浜涘瘎瀛樺櫒鍧囦负 32 浣嶅銆?


	DMA Registers 0x000-0xff:

	0x00 - Control:
		0=reset/cancel, 1=read, 2=write, 4=stop
	0x04 - DMA status:
		1=read busy, 2=write busy, 4=read error, 8=write error, 16=link list error
	0x08 - pci DMA pointer for read link list
	0x0c - pci DMA pointer for write link list
	0x10 - read/write DMA enable:
		1=read enable, 2=write enable
	0x14 - always 0xffffffff, if set any lower instability occurs, 0x00 crashes
	0x18 - ??
	0x1c - always 0x20 or 32, smaller values slow down DMA transactions
	0x20 - always value of 0x780a010a
	0x24-0x3c - usually just random values???
	0x40 - Interrupt status
	0x44 - Write a bit here and shows up in Interrupt status 0x40
	0x48 - Interrupt Mask
	0x4C - always value of 0xfffdffff,
		if changed to 0xffffffff DMA write interrupts break.
	0x50 - always 0xffffffff
	0x54 - always 0xffffffff (0x4c, 0x50, 0x54 seem like interrupt masks, are
		3 processors on chip, Java ones, VPU, SPU, APU, maybe these are the
		interrupt masks??锛?
	0x60-0x7C - random values
	0x80 - first write linked list reg, for Encoder Memory addr
	0x84 - first write linked list reg, for pci memory addr
	0x88 - first write linked list reg, for length of buffer in memory addr
		(|0x80000000 or this for last link)
	0x8c-0xdc - rest of write linked list reg, 8 sets of 3 total, DMA goes here
		from linked list addr in reg 0x0c, firmware must push through or
		something.
	0xe0 - first (and only) read linked list reg, for pci memory addr
	0xe4 - first (and only) read linked list reg, for Decoder memory addr
	0xe8 - first (and only) read linked list reg, for length of buffer
	0xec-0xff - Nothing seems to be in these registers, 0xec-f4 are 0x00000000.

Encoder 缂撳啿鍖虹殑鍐呭瓨浣嶇疆 0x700-0x7ff锛?

杩欎簺瀵勫瓨鍣ㄦ樉绀虹敤浜庣紪鐮佺殑鍚勪釜缂撳啿鍖哄尯鍩熺浉鍏冲唴瀛樹綅缃殑鍋忕Щ锛岄渶鍏堝乏绉?<<1銆?

- 0x07F8锛氱紪鐮佸櫒 SDRAM 鍒锋柊
- 0x07FC锛氱紪鐮佸櫒 SDRAM 棰勫厖鐢?

Decoder 缂撳啿鍖虹殑鍐呭瓨浣嶇疆 0x800-0x8ff锛?

杩欎簺瀵勫瓨鍣ㄦ樉绀虹敤浜庤В鐮佺殑鍚勪釜缂撳啿鍖哄尯鍩熺浉鍏冲唴瀛樹綅缃殑鍋忕Щ锛岄渶鍏堝乏绉?<<1銆?

- 0x08F8锛氳В鐮佸櫒 SDRAM 鍒锋柊
- 0x08FC锛氳В鐮佸櫒 SDRAM 棰勫厖鐢?

鍏朵粬鍐呭瓨浣嶇疆锛?

- 0x2800锛氳棰戞樉绀烘ā鍧楁帶鍒?
- 0x2D00锛欰O锛堥煶棰戣緭鍑猴紵锛夋帶鍒?
- 0x2D24锛氬凡鍒锋柊瀛楄妭鏁?
- 0x7000锛歀SB I2C 鍐欐椂閽熶綅锛堝彇鍙嶏級
- 0x7004锛歀SB I2C 鍐欐暟鎹綅锛堝彇鍙嶏級
- 0x7008锛歀SB I2C 璇绘椂閽熶綅
- 0x700c锛歀SB I2C 璇绘暟鎹綅
- 0x9008锛欸PIO 鑾峰彇杈撳叆鐘舵€?
- 0x900c锛欸PIO 璁剧疆杈撳嚭鐘舵€?
- 0x9020锛欸PIO 鏂瑰悜锛圔it7锛圙PIO 0..7锛夆€斺€?锛氳緭鍏ワ紝1锛氳緭鍑猴級
- 0x9050锛歋PU 鎺у埗
- 0x9054锛氬浣嶇‖浠舵ā鍧?
- 0x9058锛歏PU 鎺у埗
- 0xA018锛欱it6锛氫腑鏂寕璧凤紵
- 0xA064锛欰PU 鍛戒护


#### 涓柇鐘舵€佸瘎瀛樺櫒


涓柇鐘舵€佸瘎瀛樺櫒 0x0040 涓庝腑鏂帺鐮?0x0048 涓悇姣旂壒鐨勫畾涔夈€傚鏋滃湪鎺╃爜涓煇姣旂壒琚竻闆讹紝鍒欐垜浠笇鏈涙垜浠殑 ISR 鎵ц銆?

- bit 31 Encoder Start Capture
- bit 30 Encoder EOS
- bit 29 Encoder VBI capture
- bit 28 Encoder Video Input Module reset event
- bit 27 Encoder DMA complete
- bit 24 Decoder audio mode change detection event (through event notification)
- bit 22 Decoder data request
- bit 20 Decoder DMA complete
- bit 19 Decoder VBI re-insertion
- bit 18 Decoder DMA err (linked-list bad)

### 缂哄け鐨勬枃妗?


- Encoder API post锛?
- Decoder API post锛?
- Decoder VTRACE event


### cx2341x 鍥轰欢涓婁紶


鏈枃鎻忚堪濡備綍鎶?cx2341x 鍥轰欢涓婁紶鍒板崱涓娿€?

#### 濡備綍鎵惧埌


鏈夊叧濡備綍鑾峰彇鍥轰欢鐨勪俊鎭紝璇峰弬瑙佷娇鐢ㄦ鑺墖鐨勫悇椤圭洰鐨勭綉椤点€?

瀛樺偍鍦?Windows 椹卞姩涓殑鍥轰欢鍙涓嬫娴嬶細

- Each firmware image is 256k bytes.
- The 1st 32-bit word of the Encoder image is 0x0000da7
- The 1st 32-bit word of the Decoder image is 0x00003a7
- The 2nd 32-bit word of both images is 0xaa55bb66

#### 濡備綍鍔犺浇


- Issue the FWapi command to stop the encoder if it is running. Wait for the command to complete.
- Issue the FWapi command to stop the decoder if it is running. Wait for the command to complete.
- Issue the I2C command to the digitizer to stop emitting VSYNC events.
- Issue the FWapi command to halt the encoder's firmware.
- Sleep for 10ms.
- Issue the FWapi command to halt the decoder's firmware.
- Sleep for 10ms.
- Write 0x00000000 to register 0x2800 to stop the Video Display Module.
- Write 0x00000005 to register 0x2D00 to stop the AO (audio output锛?
- Write 0x00000000 to register 0xA064 to ping? the APU.
- Write 0xFFFFFFFE to register 0x9058 to stop the VPU.
- Write 0xFFFFFFFF to register 0x9054 to reset the HW blocks.
- Write 0x00000001 to register 0x9050 to stop the SPU.
- Sleep for 10ms.
- Write 0x0000001A to register 0x07FC to init the Encoder SDRAM's pre-charge.
- Write 0x80000640 to register 0x07F8 to init the Encoder SDRAM's refresh to 1us.
- Write 0x0000001A to register 0x08FC to init the Decoder SDRAM's pre-charge.
- Write 0x80000640 to register 0x08F8 to init the Decoder SDRAM's refresh to 1us.
- Sleep for 512ms. (600ms is recommended)
- Transfer the encoder's firmware image to offset 0 in Encoder memory space.
- Transfer the decoder's firmware image to offset 0 in Decoder memory space.
- Use a read-modify-write operation to Clear bit 0 of register 0x9050 to re-enable the SPU.
- Sleep for 1 second.
- Use a read-modify-write operation to Clear bits 3 and 0 of register 0x9058 to re-enable the VPU.
- Sleep for 1 second.
- Issue status API commands to both firmware images to verify.


### 濡備綍璋冪敤鍥轰欢 API


棣栭€夌殑璋冪敤绾﹀畾绉颁负鍥轰欢閭锛坒irmware mailbox锛夈€傞偖绠辨湰璐ㄤ笂鏄竴涓浐瀹氶暱搴︾殑鏁扮粍锛屽厖褰撹皟鐢ㄦ爤銆?

鍥轰欢閭鍙互閫氳繃鍦ㄧ紪鐮佸櫒鍜岃В鐮佸櫒鍐呭瓨涓悳绱竴涓?16 瀛楄妭鐨勭鍚嶆潵瀹氫綅銆傝绛惧悕浼氫綅浜?256 瀛楄妭杈圭晫涓娿€?

绛惧悕锛?


	0x78, 0x56, 0x34, 0x12, 0x12, 0x78, 0x56, 0x34,
	0x34, 0x12, 0x78, 0x56, 0x56, 0x34, 0x12, 0x78

鍥轰欢瀹炵幇浜?20 涓偖绠憋紝姣忎釜 20 涓?32 浣嶅瓧銆傚墠 10 涓繚鐣欑粰 API 璋冪敤銆傚悗 10 涓敱鍥轰欢鐢ㄤ簬浜嬩欢閫氱煡銆?

  ====== =================
  绱㈠紩   鍚嶇О
  ====== =================
  0      鏍囧織
  1      鍛戒护
  2      杩斿洖鍊?
  3      瓒呮椂
  4-19   鍙傛暟/缁撴灉
  ====== =================


鏍囧織鍦ㄤ笅闈㈢殑琛ㄤ腑瀹氫箟銆傛柟鍚戞槸绔欏湪鍥轰欢鐨勮搴︺€?

  ==== ========== ============================================
  浣?  鏂瑰悜       鐢ㄩ€?
  ==== ========== ============================================
  2    O          鍥轰欢宸插鐞嗚鍛戒护銆?
  1    I          椹卞姩宸插畬鎴愬弬鏁拌缃€?
  0    I          椹卞姩姝ｅ湪浣跨敤姝ら偖绠便€?
  ==== ========== ============================================

鍛戒护鏄竴涓?32 浣嶆灇涓惧€笺€侫PI 缁嗚妭鍙湪鏈珷鎵惧埌銆?

杩斿洖鍊兼槸涓€涓?32 浣嶆灇涓惧€笺€傜洰鍓嶄粎瀹氫箟浜嗕袱涓€硷細

- 0=success
- -1=command undefined.

鍏辨湁 16 涓弬鏁?缁撴灉 32 浣嶅瓧娈点€傞┍鍔ㄧ敤璋冪敤鎵€闇€鐨勫叏閮ㄥ弬鏁板€煎～鍏呰繖浜涘瓧娈点€傞┍鍔ㄥ啀鐢ㄨ皟鐢ㄨ繑鍥炵殑绲愭灉鍊艰鐩栬繖浜涘瓧娈点€?

瓒呮椂鍊间繚鎶ゅ崱鍏嶅彈鎸傝捣鐨勯┍鍔ㄧ嚎绋嬪奖鍝嶃€傚鏋滈┍鍔ㄦ病鏈夊湪鎸囧畾鐨勮秴鏃跺唴澶勭悊瀹岃璋冪敤锛屽浐浠跺皢澶嶄綅璇ラ偖绠便€?

瑕佽繘琛屼竴娆?API 璋冪敤锛岄┍鍔ㄩ亶鍘嗘瘡涓偖绠憋紝瀵绘壘绗竴涓彲鐢ㄧ殑锛坆it 0 宸茶娓呴浂锛夈€傞┍鍔ㄧ疆浣嶈姣旂壒锛屽～鍏ュ懡浠ゆ灇涓惧€笺€佽秴鏃跺€间互鍙婁换浣曟墍闇€鍙傛暟銆傞┍鍔ㄩ殢鍚庣疆浣嶅弬鏁板氨缁瘮鐗癸紙bit 1锛夈€傚浐浠舵壂鎻忛偖绠变互瀵绘壘寰呭鐞嗗懡浠わ紝澶勭悊瀹冧滑锛岃缃粨鏋滅爜锛岀敤璇ヨ皟鐢ㄧ殑杩斿洖鍊煎～鍏呯粨鏋滃€兼暟缁勶紝骞剁疆浣嶈皟鐢ㄥ畬鎴愭瘮鐗癸紙bit 2锛夈€備竴鏃?bit 2 琚疆浣嶏紝椹卞姩搴斿彇鍥炵粨鏋滃苟娓呴櫎鎵€鏈夋爣蹇椼€傚鏋滈┍鍔ㄦ病鏈夊湪瓒呮椂瀵勫瓨鍣ㄨ瀹氱殑鏃堕棿鍐呭畬鎴愭浠诲姟锛屽浐浠跺皢澶嶄綅璇ラ偖绠便€?

浜嬩欢閫氱煡鐢卞浐浠跺彂閫佺粰涓绘満銆備富鏈洪€氳繃涓€涓?API 璋冪敤鍛婅瘔鍥轰欢瀹冩劅鍏磋叮鐨勪簨浠躲€傝璋冪敤鍛婅瘔鍥轰欢浣跨敤鍝釜閫氱煡閭銆傚浐浠堕€氳繃涓€涓腑鏂悜涓绘満鍙戜俊鍙枫€備粎浣跨敤 16 涓粨鏋滃瓧娈碉紝鏍囧織銆佸懡浠ゃ€佽繑鍥炲€煎拰瓒呮椂瀛椾笉琚娇鐢ㄣ€?


### OSD 鍥轰欢 API 鎻忚堪



#### CX2341X_OSD_GET_FRAMEBUFFER


Enum: 65/0x41

##### 鎻忚堪


杩斿洖杩炵画 OSD 鍐呭瓨鐨勫熀鍧€鍜岄暱搴︺€?

##### 缁撴灉[0]


OSD 鍩哄潃

##### 缁撴灉[1]


OSD 闀垮害



#### CX2341X_OSD_GET_PIXEL_FORMAT


Enum: 66/0x42

##### 鎻忚堪


鏌ヨ OSD 鏍煎紡

##### 缁撴灉[0]


0=8bit index
1=16bit RGB 5:6:5
2=16bit ARGB 1:5:5:5
3=16bit ARGB 1:4:4:4
4=32bit ARGB 8:8:8:8



#### CX2341X_OSD_SET_PIXEL_FORMAT


Enum: 67/0x43

##### 鎻忚堪


璁剧疆鍍忕礌鏍煎紡

##### 鍙傛暟[0]


- 0=8bit index
- 1=16bit RGB 5:6:5
- 2=16bit ARGB 1:5:5:5
- 3=16bit ARGB 1:4:4:4
- 4=32bit ARGB 8:8:8:8



#### CX2341X_OSD_GET_STATE


Enum: 68/0x44

##### 鎻忚堪


鏌ヨ OSD 鐘舵€?

##### 缁撴灉[0]


- Bit  0   0=off, 1=on
- Bits 1:2 alpha control
- Bits 3:5 pixel format



#### CX2341X_OSD_SET_STATE


Enum: 69/0x45

##### 鎻忚堪


OSD 寮€鍏?

##### 鍙傛暟[0]


0=off, 1=on



#### CX2341X_OSD_GET_OSD_COORDS


Enum: 70/0x46

##### 鎻忚堪


鍙栧洖涓庤棰戞贩鍚堢殑 OSD 鍖哄煙鍧愭爣

##### 缁撴灉[0]


OSD 缂撳啿鍖哄湴鍧€

##### 缁撴灉[1]


姝ラ暱锛堝儚绱狅級

##### 缁撴灉[2]


OSD 缂撳啿鍖轰腑鐨勮鏁?

##### 缁撴灉[3]


缂撳啿鍖轰腑鐨勬按骞冲亸绉?

##### 缁撴灉[4]


缂撳啿鍖轰腑鐨勫瀭鐩村亸绉?



#### CX2341X_OSD_SET_OSD_COORDS


Enum: 71/0x47

##### 鎻忚堪


璁剧疆瑕佷笌瑙嗛娣峰悎鐨?OSD 鍖哄煙鍧愭爣

##### 鍙傛暟[0]


缂撳啿鍖哄湴鍧€

##### 鍙傛暟[1]


缂撳啿鍖烘闀匡紙鍍忕礌锛?

##### 鍙傛暟[2]


缂撳啿鍖轰腑鐨勮鏁?

##### 鍙傛暟[3]


姘村钩鍋忕Щ

##### 鍙傛暟[4]


鍨傜洿鍋忕Щ



#### CX2341X_OSD_GET_SCREEN_COORDS


Enum: 72/0x48

##### 鎻忚堪


鍙栧洖 OSD 灞忓箷鍖哄煙鍧愭爣

##### 缁撴灉[0]


宸︿笂瑙掓按骞冲亸绉?

##### 缁撴灉[1]


宸︿笂瑙掑瀭鐩村亸绉?

##### 缁撴灉[2]


鍙充笅瑙掓按骞冲亸绉?

##### 缁撴灉[3]


鍙充笅瑙掑瀭鐩村亸绉?



#### CX2341X_OSD_SET_SCREEN_COORDS


Enum: 73/0x49

##### 鎻忚堪


璁剧疆瑕佷笌瑙嗛娣峰悎鐨勫睆骞曞尯鍩熷潗鏍?

##### 鍙傛暟[0]


宸︿笂瑙掓按骞冲亸绉?

##### 鍙傛暟[1]


宸︿笂瑙掑瀭鐩村亸绉?

##### 鍙傛暟[2]


宸︿笅瑙掓按骞冲亸绉?

##### 鍙傛暟[3]


宸︿笅瑙掑瀭鐩村亸绉?



#### CX2341X_OSD_GET_GLOBAL_ALPHA


Enum: 74/0x4A

##### 鎻忚堪


鍙栧洖 OSD 鍏ㄥ眬 alpha

##### 缁撴灉[0]


鍏ㄥ眬 alpha锛?=off, 1=on

##### 缁撴灉[1]


bits 0:7 global alpha



#### CX2341X_OSD_SET_GLOBAL_ALPHA


Enum: 75/0x4B

##### 鎻忚堪


鏇存柊鍏ㄥ眬 alpha

##### 鍙傛暟[0]


鍏ㄥ眬 alpha锛?=off, 1=on

##### 鍙傛暟[1]


鍏ㄥ眬 alpha锛? 浣嶏級

##### 鍙傛暟[2]


灞€閮?alpha锛?=on, 1=off



#### CX2341X_OSD_SET_BLEND_COORDS


Enum: 78/0x4C

##### 鎻忚堪


鍦ㄦ樉绀虹紦鍐插尯鍐呯Щ鍔ㄦ贩鍚堝尯鍩熺殑璧风偣

##### 鍙傛暟[0]


缂撳啿鍖轰腑鐨勬按骞冲亸绉?

##### 鍙傛暟[1]


缂撳啿鍖轰腑鐨勫瀭鐩村亸绉?



#### CX2341X_OSD_GET_FLICKER_STATE


Enum: 79/0x4F

##### 鎻忚堪


鍙栧洖闂儊鎶戝埗妯″潡鐘舵€?

##### 缁撴灉[0]


闂儊鐘舵€侊細0=off, 1=on



#### CX2341X_OSD_SET_FLICKER_STATE


Enum: 80/0x50

##### 鎻忚堪


璁剧疆闂儊鎶戝埗妯″潡鐘舵€?

##### 鍙傛暟[0]


鐘舵€侊細0=off, 1=on



#### CX2341X_OSD_BLT_COPY


Enum: 82/0x52

##### 鎻忚堪


BLT 澶嶅埗

##### 鍙傛暟[0]



	'0000'  zero
	'0001' ~destination AND ~source
	'0010' ~destination AND  source
	'0011' ~destination
	'0100'  destination AND ~source
	'0101'                  ~source
	'0110'  destination XOR  source
	'0111' ~destination OR  ~source
	'1000' ~destination AND ~source
	'1001'  destination XNOR source
	'1010'                   source
	'1011' ~destination OR   source
	'1100'  destination
	'1101'  destination OR  ~source
	'1110'  destination OR   source
	'1111'  one


##### 鍙傛暟[1]


缁撴灉 alpha 娣峰悎

- '01' source_alpha
- '10' destination_alpha
- '11' source_alpha*destination_alpha+1
  (zero if both source and destination alpha are zero)

##### 鍙傛暟[2]



	'00' output_pixel = source_pixel

	'01' if source_alpha=0:
		 output_pixel = destination_pixel
	     if 256 > source_alpha > 1:
		 output_pixel = ((source_alpha + 1)*source_pixel +
				 (255 - source_alpha)*destination_pixel)/256

	'10' if destination_alpha=0:
		 output_pixel = source_pixel
	      if 255 > destination_alpha > 0:
		 output_pixel = ((255 - destination_alpha)*source_pixel +
				 (destination_alpha + 1)*destination_pixel)/256

	'11' if source_alpha=0:
		 source_temp = 0
	     if source_alpha=255:
		 source_temp = source_pixel*256
	     if 255 > source_alpha > 0:
		 source_temp = source_pixel*(source_alpha + 1)
	     if destination_alpha=0:
		 destination_temp = 0
	     if destination_alpha=255:
		 destination_temp = destination_pixel*256
	     if 255 > destination_alpha > 0:
		 destination_temp = destination_pixel*(destination_alpha + 1)
	     output_pixel = (source_temp + destination_temp)/256

##### 鍙傛暟[3]


瀹藉害

##### 鍙傛暟[4]


楂樺害

##### 鍙傛暟[5]


鐩爣鍍忕礌鎺╃爜

##### 鍙傛暟[6]


鐩爣鐭╁舰璧峰鍦板潃

##### 鍙傛暟[7]


鐩爣姝ラ暱锛坉words锛?

##### 鍙傛暟[8]


婧愭闀匡紙dwords锛?

##### 鍙傛暟[9]


婧愮煩褰㈣捣濮嬪湴鍧€



#### CX2341X_OSD_BLT_FILL


Enum: 83/0x53

##### 鎻忚堪


BLT 濉厖棰滆壊

##### 鍙傛暟[0]


Same as Param[^0^] on API 0x52

##### 鍙傛暟[1]


Same as Param[^1^] on API 0x52

##### 鍙傛暟[2]


Same as Param[^2^] on API 0x52

##### 鍙傛暟[3]


瀹藉害

##### 鍙傛暟[4]


楂樺害

##### 鍙傛暟[5]


鐩爣鍍忕礌鎺╃爜

##### 鍙傛暟[6]


鐩爣鐭╁舰璧峰鍦板潃

##### 鍙傛暟[7]


鐩爣姝ラ暱锛坉words锛?

##### 鍙傛暟[8]


棰滆壊濉厖鍊?



#### CX2341X_OSD_BLT_TEXT


Enum: 84/0x54

##### 鎻忚堪


鐢ㄤ簬 8 浣?alpha 鏂囨湰婧愮殑 BLT

##### 鍙傛暟[0]


Same as Param[^0^] on API 0x52

##### 鍙傛暟[1]


Same as Param[^1^] on API 0x52

##### 鍙傛暟[2]


Same as Param[^2^] on API 0x52

##### 鍙傛暟[3]


瀹藉害

##### 鍙傛暟[4]


楂樺害

##### 鍙傛暟[5]


鐩爣鍍忕礌鎺╃爜

##### 鍙傛暟[6]


鐩爣鐭╁舰璧峰鍦板潃

##### 鍙傛暟[7]


鐩爣姝ラ暱锛坉words锛?

##### 鍙傛暟[8]


婧愭闀匡紙dwords锛?

##### 鍙傛暟[9]


婧愮煩褰㈣捣濮嬪湴鍧€

##### 鍙傛暟[10]


棰滆壊濉厖鍊?



#### CX2341X_OSD_SET_FRAMEBUFFER_WINDOW


Enum: 86/0x56

##### 鎻忚堪


鍦ㄥ睆骞曚笂瀹氫綅涓昏緭鍑虹獥鍙ｃ€傚潗鏍囧繀椤讳娇寰楁暣涓獥鍙ｈ兘钀藉叆灞忓箷鍐呫€?

##### 鍙傛暟[0]


绐楀彛瀹藉害

##### 鍙傛暟[1]


绐楀彛楂樺害

##### 鍙傛暟[2]


宸︿笂瑙掔獥鍙ｆ按骞冲亸绉?

##### 鍙傛暟[3]


宸︿笂瑙掔獥鍙ｅ瀭鐩村亸绉?



#### CX2341X_OSD_SET_CHROMA_KEY


Enum: 96/0x60

##### 鎻忚堪


鑹插害閿紑鍏充笌棰滆壊

##### 鍙傛暟[0]


鐘舵€侊細0=off, 1=on

##### 鍙傛暟[1]


棰滆壊



#### CX2341X_OSD_GET_ALPHA_CONTENT_INDEX


Enum: 97/0x61

##### 鎻忚堪


鍙栧洖 alpha 鍐呭绱㈠紩

##### 缁撴灉[0]


alpha 鍐呭绱㈠紩锛岃寖鍥?0:15



#### CX2341X_OSD_SET_ALPHA_CONTENT_INDEX


Enum: 98/0x62

##### 鎻忚堪


璁剧疆 alpha 鍐呭绱㈠紩

##### 鍙傛暟[0]


alpha 鍐呭绱㈠紩锛岃寖鍥?0:15


### 缂栫爜鍣ㄥ浐浠?API 鎻忚堪


#### CX2341X_ENC_PING_FW


Enum: 128/0x80

##### 鎻忚堪


绌烘搷浣溿€傚彲鐢ㄤ簬妫€鏌ュ浐浠舵槸鍚﹀湪鍝嶅簲銆?



#### CX2341X_ENC_START_CAPTURE


Enum: 129/0x81

##### 鎻忚堪


寮€濮嬫崟鑾疯棰戙€侀煶棰戝拰/鎴?VBI 鏁版嵁銆傛墍鏈夌紪鐮佸弬鏁板繀椤诲湪姝?API 璋冪敤涔嬪墠鍒濆鍖栥€傛寔缁崟鑾凤紝鐩村埌鎹曡幏浜嗛瀹氫箟鏁伴噺鐨勫抚銆?

##### 鍙傛暟[0]


鎹曡幏娴佺被鍨嬶細

 - 0=MPEG
 - 1=Raw
 - 2=Raw passthrough
 - 3=VBI

##### 鍙傛暟[1]


浣嶆帺鐮侊細

 - Bit 0 缃綅鏃讹紝鎹曡幏 YUV
 - Bit 1 缃綅鏃讹紝鎹曡幏 PCM 闊抽
 - Bit 2 缃綅鏃讹紝鎹曡幏 VBI锛堝悓 param[^0^]=3锛?
 - Bit 3 缃綅鏃讹紝鎹曡幏鐩爣鏄В鐮佸櫒锛堝悓 param[^0^]=2锛?
 - Bit 4 缃綅鏃讹紝鎹曡幏鐩爣鏄富鏈?



#### CX2341X_ENC_STOP_CAPTURE


Enum: 130/0x82

##### 鎻忚堪


缁撴潫姝ｅ湪杩涜鐨勬崟鑾?

##### 鍙傛暟[0]


- 0=鍦?GOP 缁撴潫鏃跺仠姝紙浜х敓 IRQ锛?
- 1=绔嬪嵆鍋滄锛堟棤 IRQ锛?

##### 鍙傛暟[1]


瑕佸仠姝㈢殑娴佺被鍨嬶紝瑙?API 0x81 鐨?param[^0^]

##### 鍙傛暟[2]


瀛愮被鍨嬶紝瑙?API 0x81 鐨?param[^1^]



#### CX2341X_ENC_SET_AUDIO_ID


Enum: 137/0x89

##### 鎻忚堪


璁剧疆缂栫爜鍚庨煶棰戞祦鐨勪紶杈撴祦 ID

##### 鍙傛暟[0]


闊抽娴?ID



#### CX2341X_ENC_SET_VIDEO_ID


Enum: 139/0x8B

##### 鎻忚堪


璁剧疆瑙嗛浼犺緭娴?ID

##### 鍙傛暟[0]


瑙嗛娴?ID



#### CX2341X_ENC_SET_PCR_ID


Enum: 141/0x8D

##### 鎻忚堪


璁剧疆 PCR 鍖呯殑浼犺緭娴?ID

##### 鍙傛暟[0]


PCR 娴?ID



#### CX2341X_ENC_SET_FRAME_RATE


Enum: 143/0x8F

##### 鎻忚堪


璁剧疆瑙嗛姣忕甯ф暟銆傛洿鏀瑰湪涓嬩竴涓?GOP 寮€濮嬫椂鐢熸晥銆?

##### 鍙傛暟[0]


- 0=30fps
- 1=25fps



#### CX2341X_ENC_SET_FRAME_SIZE


Enum: 145/0x91

##### 鎻忚堪


閫夋嫨瑙嗛娴佺紪鐮佸垎杈ㄧ巼銆?

##### 鍙傛暟[0]


楂樺害锛堣鏁帮級銆傞粯璁?480

##### 鍙傛暟[1]


瀹藉害锛堝儚绱狅級銆傞粯璁?720



#### CX2341X_ENC_SET_BIT_RATE


Enum: 149/0x95

##### 鎻忚堪


璁剧疆瑙嗛娴佸钩鍧囩爜鐜囥€?

##### 鍙傛暟[0]


0=鍙彉鐮佺巼, 1=鎭掑畾鐮佺巼

##### 鍙傛暟[1]


鐮佺巼锛堜綅姣忕锛?

##### 鍙傛暟[2]


宄板€肩爜鐜囷紙浣嶆瘡绉掞級锛岄櫎浠?400

##### 鍙傛暟[3]


澶嶇敤鐮佺巼锛堜綅姣忕锛夛紝闄や互 400銆傚彲涓?0锛堥粯璁わ級銆?

##### 鍙傛暟[4]


鐮佺巼鎺у埗 VBR 濉厖

##### 鍙傛暟[5]


缂栫爜鍣ㄤ娇鐢ㄧ殑 VBV 缂撳啿鍖?

	#) Param\[3\] and Param\[4\] seem to be always 0
	#) Param\[5\] doesn't seem to be used.



#### CX2341X_ENC_SET_GOP_PROPERTIES


Enum: 151/0x97

##### 鎻忚堪


璁剧疆 GOP 缁撴瀯

##### 鍙傛暟[0]


GOP 澶у皬锛堟渶澶?34锛?

##### 鍙傛暟[1]


I 甯т笌 P 甯т箣闂寸殑 B 甯ф暟锛屽姞 1銆?
渚嬪锛欼BBPBBPBBPBB --> GOP 澶у皬锛?2锛孊 甯ф暟锛?+1 = 3

	GOP 澶у皬蹇呴』鏄紙B 甯ф暟 + 1锛夌殑鍊嶆暟銆?



#### CX2341X_ENC_SET_ASPECT_RATIO


Enum: 153/0x99

##### 鎻忚堪


璁剧疆缂栫爜瀹介珮姣斻€傚楂樻瘮鐨勬敼鍙樺湪涓嬩竴涓?GOP 寮€濮嬫椂鐢熸晥銆?

##### 鍙傛暟[0]


- '0000' forbidden
- '0001' 1:1 square
- '0010' 4:3
- '0011' 16:9
- '0100' 2.21:1
- '0101' to '1111' reserved



#### CX2341X_ENC_SET_DNR_FILTER_MODE


Enum: 155/0x9B

##### 鎻忚堪


璁剧疆鍔ㄦ€侀檷鍣紙Dynamic Noise Reduction锛夊伐浣滄ā寮?

##### 鍙傛暟[0]


Bit0锛氱┖闂存护娉㈠櫒锛岀疆浣?鑷姩锛屾竻闄?鎵嬪姩
Bit1锛氭椂闂存护娉㈠櫒锛岀疆浣?鑷姩锛屾竻闄?鎵嬪姩

##### 鍙傛暟[1]


涓€兼护娉㈠櫒锛?

- 0=Disabled
- 1=Horizontal
- 2=Vertical
- 3=Horiz/Vert
- 4=Diagonal



#### CX2341X_ENC_SET_DNR_FILTER_PROPS


Enum: 157/0x9D

##### 鎻忚堪


杩欎簺鍔ㄦ€侀檷鍣腑鍊兼护娉㈠櫒鐨勫€间粎褰撶浉搴旀护娉㈠櫒琚涓?鎵嬪姩"锛堣 API 0x9B锛夋椂鎵嶆湁鎰忎箟

##### 鍙傛暟[0]


绌洪棿婊ゆ尝鍣細榛樿 0锛岃寖鍥?0:15

##### 鍙傛暟[1]


鏃堕棿婊ゆ尝鍣細榛樿 0锛岃寖鍥?0:31



#### CX2341X_ENC_SET_CORING_LEVELS


Enum: 159/0x9F

##### 鎻忚堪


璁剧疆鍔ㄦ€侀檷鍣腑鍊兼护娉㈠櫒灞炴€с€?

##### 鍙傛暟[0]


浜害涓€兼护娉㈠櫒鍚敤鎵€渚濇嵁鐨勯槇鍊间笂闄愩€?
榛樿锛?锛岃寖鍥?0:255

##### 鍙傛暟[1]


浜害涓€兼护娉㈠櫒鍚敤鎵€渚濇嵁鐨勯槇鍊间笅闄愩€?
榛樿锛?55锛岃寖鍥?0:255

##### 鍙傛暟[2]


鑹插害涓€兼护娉㈠櫒鍚敤鎵€渚濇嵁鐨勯槇鍊间笂闄愩€?
榛樿锛?锛岃寖鍥?0:255

##### 鍙傛暟[3]


鑹插害涓€兼护娉㈠櫒鍚敤鎵€渚濇嵁鐨勯槇鍊间笅闄愩€?
榛樿锛?55锛岃寖鍥?0:255



#### CX2341X_ENC_SET_SPATIAL_FILTER_TYPE


Enum: 161/0xA1

##### 鎻忚堪


璁剧疆绌洪棿棰勬护娉㈠弬鏁?

##### 鍙傛暟[0]


浜害婊ゆ尝鍣?

- 0=Off
- 1=1D Horizontal
- 2=1D Vertical
- 3=2D H/V Separable (default)
- 4=2D Symmetric non-separable

##### 鍙傛暟[1]


鑹插害婊ゆ尝鍣?

- 0=Off
- 1=1D Horizontal (default)



#### CX2341X_ENC_SET_VBI_LINE


Enum: 183/0xB7

##### 鎻忚堪


閫夋嫨 VBI 琛屽彿銆?

##### 鍙傛暟[0]


- Bits 0:4 	line number
- Bit  31		0=top_field, 1=bottom_field
- Bits 0:31 	all set specifies "all lines"

##### 鍙傛暟[1]


VBI 琛屼俊鎭壒鎬э細0=disabled, 1=enabled

##### 鍙傛暟[2]


鍒囩墖锛?=None, 1=Closed Caption
鍑犱箮鍙互纭畾鏈疄鐜般€傝涓?0銆?

##### 鍙傛暟[3]


鏈涓殑浜害閲囨牱鏁般€?
鍑犱箮鍙互纭畾鏈疄鐜般€傝涓?0銆?

##### 鍙傛暟[4]


鏈涓殑鑹插害閲囨牱鏁?
鍑犱箮鍙互纭畾鏈疄鐜般€傝涓?0銆?



#### CX2341X_ENC_SET_STREAM_TYPE


Enum: 185/0xB9

##### 鎻忚堪


璁剧疆娴佺被鍨?


	Transport stream is not working in recent firmwares.
	And in older firmwares the timestamps in the TS seem to be
	unreliable.

##### 鍙傛暟[0]


- 0=Program stream
- 1=Transport stream
- 2=MPEG1 stream
- 3=PES A/V stream
- 5=PES Video stream
- 7=PES Audio stream
- 10=DVD stream
- 11=VCD stream
- 12=SVCD stream
- 13=DVD_S1 stream
- 14=DVD_S2 stream



#### CX2341X_ENC_SET_OUTPUT_PORT


Enum: 187/0xBB

##### 鎻忚堪


璁剧疆娴佽緭鍑虹鍙ｃ€傚綋鏁版嵁閫氳繃 PCI 鎬荤嚎锛圖MA锛夊鍒舵椂閫氬父涓?0锛屽綋鏁版嵁娴佸悜鍙︿竴棰楄姱鐗囷紙pvrusb 鍜?cx88-blackbird锛夋椂涓?1銆?

##### 鍙傛暟[0]


- 0=Memory (default)
- 1=Streaming
- 2=Serial

##### 鍙傛暟[1]


鏈煡锛屼絾鎶婂畠鐣欎负 0 浼间箮鏁堟灉鏈€濂姐€傛湁杩硅薄琛ㄦ槑杩欏彲鑳戒笌 USB 鏀寔鏈夊叧锛屼笉杩囦紶鍏ラ潪 0 鐨勪换浣曞€煎彧浼氭妸浜嬫儏寮勭碂銆?



#### CX2341X_ENC_SET_AUDIO_PROPERTIES


Enum: 189/0xBD

##### 鎻忚堪


璁剧疆闊抽娴佸睘鎬э紝鍙湪缂栫爜杩涜涓皟鐢ㄣ€?


	All bitfields are consistent with ISO11172 documentation except
	bits 2:3 which ISO docs define as:

 - '11' Layer I
 - '10' Layer II
 - '01' Layer III
 - '00' Undefined

	This discrepancy may indicate a possible error in the documentation.
	Testing indicated that only Layer II is actually working, and that
	the minimum bitrate should be 192 kbps.

##### 鍙傛暟[0]


浣嶆帺鐮侊細


	   0:1  '00' 44.1Khz
		'01' 48Khz
		'10' 32Khz
		'11' reserved

	   2:3  '01'=Layer I
		'10'=Layer II

	   4:7  Bitrate:
		     Index | Layer I     | Layer II
		     ------+-------------+------------
		    '0000' | free format | free format
		    '0001' |  32 kbit/s  |  32 kbit/s
		    '0010' |  64 kbit/s  |  48 kbit/s
		    '0011' |  96 kbit/s  |  56 kbit/s
		    '0100' | 128 kbit/s  |  64 kbit/s
		    '0101' | 160 kbit/s  |  80 kbit/s
		    '0110' | 192 kbit/s  |  96 kbit/s
		    '0111' | 224 kbit/s  | 112 kbit/s
		    '1000' | 256 kbit/s  | 128 kbit/s
		    '1001' | 288 kbit/s  | 160 kbit/s
		    '1010' | 320 kbit/s  | 192 kbit/s
		    '1011' | 352 kbit/s  | 224 kbit/s
		    '1100' | 384 kbit/s  | 256 kbit/s
		    '1101' | 416 kbit/s  | 320 kbit/s
		    '1110' | 448 kbit/s  | 384 kbit/s

```
			For Layer II, not all combinations of total bitrate
			and mode are allowed. See ISO11172-3 3-Annex B,
			Table 3-B.2

	   8:9  '00'=Stereo
		'01'=JointStereo
		'10'=Dual
		'11'=Mono

		.. note::

			The cx23415 cannot decode Joint Stereo properly.

	  10:11 Mode Extension used in joint_stereo mode.
		In Layer I and II they indicate which subbands are in
		intensity_stereo. All other subbands are coded in stereo.
		    '00' subbands 4-31 in intensity_stereo, bound==4
		    '01' subbands 8-31 in intensity_stereo, bound==8
		    '10' subbands 12-31 in intensity_stereo, bound==12
		    '11' subbands 16-31 in intensity_stereo, bound==16

	  12:13 Emphasis:
		    '00' None
		    '01' 50/15uS
		    '10' reserved
		    '11' CCITT J.17

	  14 	CRC:
		    '0' off
		    '1' on

	  15    Copyright:
		    '0' off
		    '1' on

	  16    Generation:
		    '0' copy
		    '1' original

```

#### CX2341X_ENC_HALT_FW


Enum: 195/0xC3

##### 鎻忚堪


鍥轰欢琚仠姝紝鍦ㄥ浐浠惰閲嶆柊涓婁紶涔嬪墠涓嶅啀鏈嶅姟浠讳綍 API 璋冪敤銆?



#### CX2341X_ENC_GET_VERSION


Enum: 196/0xC4

##### 鎻忚堪


杩斿洖缂栫爜鍣ㄥ浐浠剁殑鐗堟湰銆?

##### 缁撴灉[0]


鐗堟湰浣嶆帺鐮侊細
- Bits  0:15 build
- Bits 16:23 minor
- Bits 24:31 major



#### CX2341X_ENC_SET_GOP_CLOSURE


Enum: 197/0xC5

##### 鎻忚堪


璁剧疆 GOP 寮€鏀?闂悎灞炴€с€?

##### 鍙傛暟[0]


- 0=Open
- 1=Closed



#### CX2341X_ENC_GET_SEQ_END


Enum: 198/0xC6

##### 鎻忚堪


鑾峰彇缂栫爜鍣ㄧ紦鍐插尯涓殑搴忓垪缁撴潫鐮併€傚綋涓€娆℃崟鑾峰紑濮嬫椂浠嶄細浜х敓鑻ュ共涓柇锛屽叾涓渶鍚庝竴涓腑鏂殑 Result[^0^] 灏嗚缃负 1锛岃€?Result[^1^] 灏嗗寘鍚紦鍐插尯鐨勫ぇ灏忋€?

##### 缁撴灉[0]


浼犺緭鐘舵€侊紙鑻ヤ负鏈€鍚庝竴涓紦鍐插尯鍒欎负 1锛?

##### 缁撴灉[1]


鑻?Result[^0^] 涓?1锛屽垯姝ゅ鍖呭惈鏈€鍚庝竴涓紦鍐插尯鐨勫ぇ灏忥紝鍚﹀垯鏈畾涔夈€?



#### CX2341X_ENC_SET_PGM_INDEX_INFO


Enum: 199/0xC7

##### 鎻忚堪


璁剧疆鑺傜洰绱㈠紩淇℃伅锛圥rogram Index Information锛夈€?
淇℃伅鎸夊涓嬫柟寮忓瓨鍌細


	struct info {
		u32 length;		// Length of this frame
		u32 offset_low;		// Offset in the file of the
		u32 offset_high;	// start of this frame
		u32 mask1;		// Bits 0-2 are the type mask:
					// 1=I, 2=P, 4=B
					// 0=End of Program Index, other fields
					//   are invalid.
		u32 pts;		// The PTS of the frame
		u32 mask2;		// Bit 0 is bit 32 of the pts.
	};
	u32 table_ptr;
	struct info index[^400^];

table_ptr 鏄〃涓皢鍐欏叆**鏂?*鏉＄洰鐨勭紪鐮佸櫒鍐呭瓨鍦板潃銆?

##### 鍙傛暟[0]


鍥惧儚鎺╃爜锛?
- 0=涓嶆崟鑾风储寮?
- 1=I 甯?
- 3=I, P 甯?
- 7=I, P, B 甯?

锛堜技涔庤蹇界暐锛屽畠鎬绘槸绱㈠紩 I銆丳 鍜?B 甯э級

##### 鍙傛暟[1]


璇锋眰鐨勫厓绱犳暟锛堟渶澶?400锛?

##### 缁撴灉[0]


琛ㄨ捣濮嬪鍦ㄧ紪鐮佸櫒鍐呭瓨涓殑鍋忕Щ銆?

##### 缁撴灉[1]


宸插垎閰嶇殑鍏冪礌鏁帮紝鏈€澶氬埌 Param[^1^]



#### CX2341X_ENC_SET_VBI_CONFIG


Enum: 200/0xC8

##### 鎻忚堪


閰嶇疆 VBI 璁剧疆

##### 鍙傛暟[0]


浣嶅浘锛?


	    0    Mode '0' Sliced, '1' Raw
	    1:3  Insertion:
		     '000' insert in extension & user data
		     '001' insert in private packets
		     '010' separate stream and user data
		     '111' separate stream and private data
	    8:15 Stream ID (normally 0xBD)

##### 鍙傛暟[1]


姣忎釜涓柇鐨勫抚鏁帮紙鏈€澶?8锛夈€備粎鍦?raw 妯″紡涓嬫湁鏁堛€?

##### 鍙傛暟[2]


raw VBI 鎬诲抚鏁般€備粎鍦?raw 妯″紡涓嬫湁鏁堛€?

##### 鍙傛暟[3]


璧峰鐮?

##### 鍙傛暟[4]


鍋滄鐮?

##### 鍙傛暟[5]


姣忓抚琛屾暟

##### 鍙傛暟[6]


姣忚瀛楄妭鏁?

##### 缁撴灉[0]


浠呭湪 raw 妯″紡涓嬭瀵熷埌鐨勬瘡涓腑鏂抚鏁般€傝寖鍥?1 鍒?Param[^1^]

##### 缁撴灉[1]


raw 妯″紡涓嬭瀵熷埌鐨勫抚鏁般€傝寖鍥?1 鍒?Param[^2^]

##### 缁撴灉[2]


raw VBI 鏁版嵁鐨勮捣濮嬪唴瀛樺亸绉?



#### CX2341X_ENC_SET_DMA_BLOCK_SIZE


Enum: 201/0xC9

##### 鎻忚堪


璁剧疆 DMA 浼犺緭鍧楀ぇ灏?

##### 鍙傛暟[0]


DMA 浼犺緭鍧楀ぇ灏忥紙瀛楄妭鎴栧抚锛夈€傚綋鍗曚綅涓哄瓧鑺傛椂锛屾敮鎸佺殑鍧楀ぇ灏忎负 2^7銆?^8 鍜?2^9 瀛楄妭銆?

##### 鍙傛暟[1]


鍗曚綅锛?=瀛楄妭, 1=甯?



#### CX2341X_ENC_GET_PREV_DMA_INFO_MB_10


Enum: 202/0xCA

##### 鎻忚堪


缁撳悎涓柇鎺╃爜 bit 27 杩斿洖鍓嶄竴娆?DMA 浼犺緭鐨勪俊鎭€備娇鐢ㄩ偖绠?10銆?

##### 缁撴灉[0]


娴佺被鍨?

##### 缁撴灉[1]


鍦板潃鍋忕Щ

##### 缁撴灉[2]


浼犺緭鐨勬渶澶уぇ灏?



#### CX2341X_ENC_GET_PREV_DMA_INFO_MB_9


Enum: 203/0xCB

##### 鎻忚堪


缁撳悎涓柇鎺╃爜 bit 27 鎴?bit 18 杩斿洖鍓嶄竴娆?DMA 浼犺緭鐨勪俊鎭€備娇鐢ㄩ偖绠?9銆?

##### 缁撴灉[0]


鐘舵€佷綅锛?
- 0   read completed
- 1   write completed
- 2   DMA read error
- 3   DMA write error
- 4   Scatter-Gather array error

##### 缁撴灉[1]


DMA 绫诲瀷

##### 缁撴灉[2]


鍛堢幇鏃堕棿鎴筹紙Presentation Time Stamp锛変綅 0..31

##### 缁撴灉[3]


鍛堢幇鏃堕棿鎴充綅 32



#### CX2341X_ENC_SCHED_DMA_TO_HOST


Enum: 204/0xCC

##### 鎻忚堪


璁剧疆鍒颁富鏈虹殑 DMA 鎿嶄綔

##### 鍙傛暟[0]


閾捐〃鐨勫唴瀛樺湴鍧€

##### 鍙傛暟[1]


閾捐〃闀垮害锛坵tf锛氫粈涔堝崟浣????锛?

##### 鍙傛暟[2]


DMA 绫诲瀷锛?=MPEG锛?



#### CX2341X_ENC_INITIALIZE_INPUT


Enum: 205/0xCD

##### 鎻忚堪


鍒濆鍖栬棰戣緭鍏?



#### CX2341X_ENC_SET_FRAME_DROP_RATE


Enum: 208/0xD0

##### 鎻忚堪


瀵规瘡涓€甯ц鎹曡幏鐨勫抚锛岃烦杩囨寚瀹氭暟閲忕殑甯с€?

##### 鍙傛暟[0]


瑕佽烦杩囩殑甯ф暟



#### CX2341X_ENC_PAUSE_ENCODER


Enum: 210/0xD2

##### 鎻忚堪


鍦ㄦ殏鍋滅姸鎬佷笅锛屾墍鏈夊抚閮借涓㈠純鑰屼笉鏄缂栫爜銆?

##### 鍙傛暟[0]


- 0=鏆傚仠缂栫爜
- 1=缁х画缂栫爜



#### CX2341X_ENC_REFRESH_INPUT


Enum: 211/0xD3

##### 鎻忚堪


鍒锋柊瑙嗛杈撳叆



#### CX2341X_ENC_SET_COPYRIGHT


Enum: 212/0xD4

##### 鎻忚堪


璁剧疆娴佺殑鐗堟潈灞炴€?

##### 鍙傛暟[0]


- 0=娴佷笉鍙楃増鏉冧繚鎶?
- 1=娴佸彈鐗堟潈淇濇姢



#### CX2341X_ENC_SET_EVENT_NOTIFICATION


Enum: 213/0xD5

##### 鎻忚堪


璁剧疆鍥轰欢浠ュ氨鏌愪釜鐗瑰畾浜嬩欢閫氱煡涓绘満銆備富鏈哄繀椤诲彇娑堝璇ヤ腑鏂綅鐨勫睆钄姐€?

##### 鍙傛暟[0]


浜嬩欢锛?=鍒锋柊缂栫爜鍣ㄨ緭鍏ワ級

##### 鍙傛暟[1]


閫氱煡 0=绂佺敤 1=鍚敤

##### 鍙傛暟[2]


涓柇浣?

##### 鍙傛暟[3]


閭妲戒綅锛?1 琛ㄧず涓嶉渶瑕侀偖绠便€?



#### CX2341X_ENC_SET_NUM_VSYNC_LINES


Enum: 214/0xD6

##### 鎻忚堪


鏍规嵁鎵€鐢ㄧ殑妯℃嫙瑙嗛瑙ｇ爜鍣紝璁剧疆鍦?1 鍜屽満 2 鐨勮鏁般€?

##### 鍙傛暟[0]


鍦?1 琛屾暟锛?
- 0x00EF for SAA7114
- 0x00F0 for SAA7115
- 0x0105 for Micronas

##### 鍙傛暟[1]


鍦?2 琛屾暟锛?
- 0x00EF for SAA7114
- 0x00F0 for SAA7115
- 0x0106 for Micronas



#### CX2341X_ENC_SET_PLACEHOLDER


Enum: 215/0xD7

##### 鎻忚堪


鎻愪緵涓€绉嶅湪 MPEG 娴佷腑鎻掑叆鑷畾涔夌敤鎴锋暟鎹殑鏈哄埗銆?

##### 鍙傛暟[0]


- 0=extension & user data
- 1=甯︽祦 ID 0xBD 鐨勭鏈夊寘

##### 鍙傛暟[1]


鎻掑叆鏁版嵁鐨勯€熺巼锛屽崟浣嶄负甯э紙瀵圭鏈夊寘锛夋垨 GOP锛堝 ext. & user data锛?

##### 鍙傛暟[2]


瑕佹彃鍏ョ殑鏁版嵁 DWORD 鏁帮紙濡備笅锛?

##### 鍙傛暟[3]


鑷畾涔夋暟鎹?0

##### 鍙傛暟[4]


鑷畾涔夋暟鎹?1

##### 鍙傛暟[5]


鑷畾涔夋暟鎹?2

##### 鍙傛暟[6]


鑷畾涔夋暟鎹?3

##### 鍙傛暟[7]


鑷畾涔夋暟鎹?4

##### 鍙傛暟[8]


鑷畾涔夋暟鎹?5

##### 鍙傛暟[9]


鑷畾涔夋暟鎹?6

##### 鍙傛暟[10]


鑷畾涔夋暟鎹?7

##### 鍙傛暟[11]


鑷畾涔夋暟鎹?8



#### CX2341X_ENC_MUTE_VIDEO


Enum: 217/0xD9

##### 鎻忚堪


瑙嗛闈欓煶

##### 鍙傛暟[0]


浣嶇敤娉曪細


	 0    	'0'=video not muted
		'1'=video muted, creates frames with the YUV color defined below
	 1:7  	Unused
	 8:15 	V chrominance information
	16:23 	U chrominance information
	24:31 	Y luminance information



#### CX2341X_ENC_MUTE_AUDIO


Enum: 218/0xDA

##### 鎻忚堪


闊抽闈欓煶

##### 鍙傛暟[0]


- 0=audio not muted
- 1=audio muted (produces silent mpeg audio stream)



#### CX2341X_ENC_SET_VERT_CROP_LINE


Enum: 219/0xDB

##### 鎻忚堪


涓庘€淰ertical Crop Line鈥濈浉鍏崇殑涓€浜涙搷浣?

##### 鍙傛暟[0]


鑻ヤ负 saa7114 涓?raw VBI 鎹曡幏涓?60 Hz锛屽垯璁句负 10001銆?
鍚﹀垯涓?0銆?



#### CX2341X_ENC_MISC


Enum: 220/0xDC

##### 鎻忚堪


鏉傞」鎿嶄綔銆備笉鑳?100% 纭畾瀹冪殑浣滅敤銆傚畠鏇村儚鏄竴绉?ioctl 璋冪敤銆傜涓€涓弬鏁版槸鍛戒护鍙凤紝绗簩涓槸鍊笺€?

##### 鍙傛暟[0]


鍛戒护鍙凤細


	 1=set initial SCR value when starting encoding (works).
	 2=set quality mode (apparently some test setting).
	 3=setup advanced VIM protection handling.
	   Always 1 for the cx23416 and 0 for cx23415.
	 4=generate DVD compatible PTS timestamps
	 5=USB flush mode
	 6=something to do with the quantization matrix
	 7=set navigation pack insertion for DVD: adds 0xbf (private stream 2)
	   packets to the MPEG. The size of these packets is 2048 bytes (including
	   the header of 6 bytes: 0x000001bf + length). The payload is zeroed and
	   it is up to the application to fill them in. These packets are apparently
	   inserted every four frames.
	 8=enable scene change detection (seems to be a failure)
	 9=set history parameters of the video input module
	10=set input field order of VIM
	11=set quantization matrix
	12=reset audio interface after channel change or input switch (has no argument).
	   Needed for the cx2584x, not needed for the mspx4xx, but it doesn't seem to
	   do any harm calling it regardless.
	13=set audio volume delay
	14=set audio delay

##### 鍙傛暟[1]


鍛戒护鍊笺€?

### 瑙ｇ爜鍣ㄥ浐浠?API 鎻忚堪



#### CX2341X_DEC_PING_FW


Enum: 0/0x00

##### 鎻忚堪


姝?API 璋冪敤涓嶅仛浠讳綍浜嬨€傚彲鐢ㄤ簬妫€鏌ュ浐浠舵槸鍚﹀湪鍝嶅簲銆?



#### CX2341X_DEC_START_PLAYBACK


Enum: 1/0x01

##### 鎻忚堪


寮€濮嬫垨鎭㈠鎾斁銆?

##### 鍙傛暟[0]


浠?GOP 涓紑濮嬫挱鏀剧殑銆佷粠 0 璁℃暟鐨勫抚鍙枫€?

##### 鍙傛暟[1]


鎸囧畾鍦ㄦ甯搁煶棰戞仮澶嶄箣鍓嶆挱鏀剧殑闈欓煶闊抽甯ф暟銆傦紙鍥轰欢鏈疄鐜版鍔熻兘锛屼繚鐣欎负 0锛?



#### CX2341X_DEC_STOP_PLAYBACK


Enum: 2/0x02

##### 鎻忚堪


缁撴潫鎾斁骞舵竻绌烘墍鏈夎В鐮佸櫒缂撳啿鍖恒€傝嫢 PTS 闈為浂锛屽垯鍦ㄦ寚瀹氱殑 PTS 澶勫仠姝㈡挱鏀俱€?

##### 鍙傛暟[0]


鏄剧ず 0=鏈€鍚庝竴甯? 1=榛戝睆

		this takes effect immediately, so if you want to wait for a PTS,
		then use '0', otherwise the screen goes to black at once.
		You can call this later (even if there is no playback) with a 1 value
		to set the screen to black.

##### 鍙傛暟[1]


PTS 浣庝綅

##### 鍙傛暟[2]


PTS 楂樹綅



#### CX2341X_DEC_SET_PLAYBACK_SPEED


Enum: 3/0x03

##### 鎻忚堪


浠ラ潪姝ｅ父閫熷害鎾斁娴併€傛湁涓ょ鎿嶄綔妯″紡锛?

 - Smooth锛氫富鏈轰紶杈撴暣涓祦锛屽浐浠朵涪寮冩湭浣跨敤鐨勫抚銆?
 - Coarse锛氫富鏈烘牴鎹储寮曟寜闇€涓㈠純甯т互杈惧埌鎵€闇€閫熷害銆?

##### 鍙傛暟[0]



	Bitmap:
	    0:7  0 normal
		 1 fast only "1.5 times"
		 n nX fast, 1/nX slow
	    30   Framedrop:
		     '0' during 1.5 times play, every other B frame is dropped
		     '1' during 1.5 times play, stream is unchanged (bitrate
			 must not exceed 8mbps)
	    31   Speed:
		     '0' slow
		     '1' fast


	n is limited to 2. Anything higher does not result in
	faster playback. Instead the host should start dropping frames.

##### 鍙傛暟[1]


鏂瑰悜锛?=forward, 1=reverse


	to make reverse playback work you have to write full GOPs in
	reverse order.

##### 鍙傛暟[2]



	Picture mask:
	    1=I frames
	    3=I, P frames
	    7=I, P, B frames

##### 鍙傛暟[3]


B frames per GOP (for reverse play only)


	for reverse playback the Picture Mask should be set to I or I, P.
	Adding B frames to the mask will result in corrupt video. This field
	has to be set to the correct value in order to keep the timing correct.

##### 鍙傛暟[4]


Mute audio: 0=disable, 1=enable

##### 鍙傛暟[5]


Display 0=frame, 1=field

##### 鍙傛暟[6]


鎸囧畾鍦ㄦ甯搁煶棰戞仮澶嶄箣鍓嶆挱鏀剧殑闈欓煶闊抽甯ф暟銆傦紙鍥轰欢鏈疄鐜版鍔熻兘锛屼繚鐣欎负 0锛?



#### CX2341X_DEC_STEP_VIDEO


Enum: 5/0x05

##### 鎻忚堪


姝?API 鐨勬瘡娆¤皟鐢ㄩ兘浼氬皢鎾斁姝ヨ繘鍒颁笅闈㈠畾涔夌殑銆佸綋鍓嶆挱鏀炬柟鍚戜笂鐨勪笅涓€涓崟鍏冦€?

##### 鍙傛暟[0]


0=frame, 1=top field, 2=bottom field



#### CX2341X_DEC_SET_DMA_BLOCK_SIZE


Enum: 8/0x08

##### 鎻忚堪


璁剧疆 DMA 浼犺緭鍧楀ぇ灏忋€侫PI 0xC9 鐨勫搴旈」銆?

##### 鍙傛暟[0]


DMA 浼犺緭鍧楀ぇ灏忥紙瀛楄妭锛夈€傚彂鍑?DMA 浼犺緭鍛戒护鏃跺彲鎸囧畾涓嶅悓澶у皬銆?



#### CX2341X_DEC_GET_XFER_INFO


Enum: 9/0x09

##### 鎻忚堪


姝?API 璋冪敤鍙敤浜庢娴嬫祦缁撴潫锛坋nd of stream锛夋潯浠躲€?

##### 缁撴灉[0]


娴佺被鍨?

##### 缁撴灉[1]


鍦板潃鍋忕Щ

##### 缁撴灉[2]


鏈€澶т紶杈撳瓧鑺傛暟

##### 缁撴灉[3]


缂撳啿鍖哄厖鐩堝害



#### CX2341X_DEC_GET_DMA_STATUS


Enum: 10/0x0A

##### 鎻忚堪


涓婁竴娆?DMA 浼犺緭鐨勭姸鎬?

##### 缁撴灉[0]


Bit 1 set means transfer complete
Bit 2 set means DMA error
Bit 3 set means linked list error

##### 缁撴灉[1]


DMA type: 0=MPEG, 1=OSD, 2=YUV



#### CX2341X_DEC_SCHED_DMA_FROM_HOST


Enum: 11/0x0B

##### 鎻忚堪


璁剧疆浠庝富鏈虹殑 DMA 鎿嶄綔銆侫PI 0xCC 鐨勫搴旈」銆?

##### 鍙傛暟[0]


閾捐〃鐨勫唴瀛樺湴鍧€

##### 鍙傛暟[1]


瑕佷紶杈撶殑鎬诲瓧鑺傛暟

##### 鍙傛暟[2]


DMA 绫诲瀷锛?=MPEG, 1=OSD, 2=YUV锛?



#### CX2341X_DEC_PAUSE_PLAYBACK


Enum: 13/0x0D

##### 鎻忚堪


绔嬪嵆鍐荤粨鎾斁銆傚湪姝ゆā寮忎笅锛屽綋鍐呴儴缂撳啿鍖烘弧鏃讹紝涓嶅啀鎺ユ敹鏇村鏁版嵁锛屾暟鎹姹?IRQ 涔熶細琚睆钄姐€?

##### 鍙傛暟[0]


鏄剧ず锛?=鏈€鍚庝竴甯? 1=榛戝睆



#### CX2341X_DEC_HALT_FW


Enum: 14/0x0E

##### 鎻忚堪


鍥轰欢琚仠姝紝鍦ㄥ浐浠惰閲嶆柊涓婁紶涔嬪墠涓嶅啀鏈嶅姟浠讳綍 API 璋冪敤銆?



#### CX2341X_DEC_SET_STANDARD


Enum: 16/0x10

##### 鎻忚堪


閫夋嫨鏄剧ず鏍囧噯

##### 鍙傛暟[0]


0=NTSC, 1=PAL



#### CX2341X_DEC_GET_VERSION


Enum: 17/0x11

##### 鎻忚堪


杩斿洖瑙ｇ爜鍣ㄥ浐浠剁増鏈俊鎭?

##### 缁撴灉[0]


鐗堟湰浣嶆帺鐮侊細
 - Bits  0:15 build
 - Bits 16:23 minor
 - Bits 24:31 major



#### CX2341X_DEC_SET_STREAM_INPUT


Enum: 20/0x14

##### 鎻忚堪


閫夋嫨瑙ｇ爜鍣ㄦ祦杈撳叆绔彛

##### 鍙傛暟[0]


0=memory (default), 1=streaming



#### CX2341X_DEC_GET_TIMING_INFO


Enum: 21/0x15

##### 鎻忚堪


杩斿洖浠庢挱鏀惧紑濮嬭捣鐨勬椂搴忎俊鎭?

##### 缁撴灉[0]


鎸夎В鐮侀『搴忕殑甯ц鏁?

##### 缁撴灉[1]


鎸夋樉绀洪『搴忕殑瑙嗛 PTS 浣?0:31

##### 缁撴灉[2]


鎸夋樉绀洪『搴忕殑瑙嗛 PTS 浣?32

##### 缁撴灉[3]


鎸夋樉绀洪『搴忕殑 SCR 浣?0:31

##### 缁撴灉[4]


鎸夋樉绀洪『搴忕殑 SCR 浣?32



#### CX2341X_DEC_SET_AUDIO_MODE


Enum: 22/0x16

##### 鎻忚堪


閫夋嫨闊抽妯″紡

##### 鍙傛暟[0]


鍙屽０閬撳崟澹版ā寮忓姩浣?
	0=Stereo, 1=Left, 2=Right, 3=Mono, 4=Swap, -1=Unchanged

##### 鍙傛暟[1]


绔嬩綋澹版ā寮忓姩浣滐細
	0=Stereo, 1=Left, 2=Right, 3=Mono, 4=Swap, -1=Unchanged



#### CX2341X_DEC_SET_EVENT_NOTIFICATION


Enum: 23/0x17

##### 鎻忚堪


璁剧疆鍥轰欢浠ュ氨鏌愪釜鐗瑰畾浜嬩欢閫氱煡涓绘満銆?
API 0xD5 鐨勫搴旈」銆?

##### 鍙傛暟[0]


浜嬩欢锛?
 - 0=闊抽妯″紡鍦?mono銆?joint) stereo 鍜?dual channel 涔嬮棿鍙樺寲銆?
 - 3=瑙ｇ爜鍣ㄥ凡鍚姩
 - 4=鏈煡锛氳В鐮佹椂姣忕瑙﹀彂 10-15 娆°€?
 - 5=鏌愪釜鍚屾浜嬩欢锛氭瘡甯цЕ鍙戜竴娆°€?

##### 鍙傛暟[1]


閫氱煡 0=绂佺敤, 1=鍚敤

##### 鍙傛暟[2]


涓柇浣?

##### 鍙傛暟[3]


閭妲戒綅锛?1 琛ㄧず涓嶉渶瑕侀偖绠便€?



#### CX2341X_DEC_SET_DISPLAY_BUFFERS


Enum: 24/0x18

##### 鎻忚堪


鏄剧ず缂撳啿鍖烘暟閲忋€傝鍦ㄥ€掓斁涓В鐮佹墍鏈夊抚锛屽繀椤讳娇鐢ㄤ節涓紦鍐插尯銆?

##### 鍙傛暟[0]


0=six buffers, 1=nine buffers



#### CX2341X_DEC_EXTRACT_VBI


Enum: 25/0x19

##### 鎻忚堪


鎻愬彇 VBI 鏁版嵁

##### 鍙傛暟[0]


0=浠?extension & user data 鎻愬彇, 1=浠庣鏈夊寘鎻愬彇

##### 缁撴灉[0]


VBI 琛ㄤ綅缃?

##### 缁撴灉[1]


VBI 琛ㄥぇ灏?



#### CX2341X_DEC_SET_DECODER_SOURCE


Enum: 26/0x1A

##### 鎻忚堪


閫夋嫨瑙ｇ爜鍣ㄦ簮銆傜‘淇濅紶缁欐 API 鐨勫弬鏁颁笌缂栫爜鍣ㄨ缃浉鍖归厤銆?

##### 鍙傛暟[0]


妯″紡锛?=MPEG from host, 1=YUV from encoder, 2=YUV from host

##### 鍙傛暟[1]


YUV 鍥惧儚瀹藉害

##### 鍙傛暟[2]


YUV 鍥惧儚楂樺害

##### 鍙傛暟[3]


浣嶅浘锛氳 API 0xBD 鐨?Param[^0^]



#### CX2341X_DEC_SET_PREBUFFERING


Enum: 30/0x1E

##### 鎻忚堪


瑙ｇ爜鍣ㄩ缂撳啿锛屽惎鐢ㄦ椂锛屽浜?<8mbps 鐨勬祦缂撳啿鏈€澶?128KB锛屽浜?>8mbps 鐨勬祦缂撳啿鏈€澶?640KB銆?

##### 鍙傛暟[0]


0=off, 1=on

### PVR350 瑙嗛瑙ｇ爜鍣ㄥ瘎瀛樺櫒 0x02002800 -> 0x02002B00


Author: Ian Armstrong <ian@iarmst.demon.co.uk>

Version: v0.4

Date: 12 March 2007


姝ゅ垪琛ㄦ槸閫氳繃鍙嶅璇曢獙寰楀嚭鐨勩€傚叾涓細鏈夐敊璇拰閬楁紡銆傛湁浜涘瘎瀛樺櫒娌℃湁鏄庢樉鐨勬晥鏋滐紝鎵€浠ュ緢闅捐瀹冧滑鍋氫粈涔堬紱鑰屽彟涓€浜涗細鐩镐簰褰卞搷锛屾垨闇€瑕佺壒瀹氱殑鍔犺浇椤哄簭銆傛按骞虫护娉㈠櫒璁剧疆灏辨槸涓€涓緥瀛愶細鏈夊叚涓瘎瀛樺櫒鍗忓悓宸ヤ綔锛屽苟闇€瑕佺壒瀹氱殑鍔犺浇椤哄簭鎵嶈兘姝ｇ‘閰嶇疆銆傜储寮曡壊褰╄皟鑹叉澘鍙渶涓や釜瀵勫瓨鍣ㄥ氨鑳芥洿瀹规槗鍦拌缃紝浣嗗悓鏍烽渶瑕佺壒瀹氱殑鍔犺浇椤哄簭銆?

鏈変簺瀵勫瓨鍣ㄥ瀹冧滑鐨勮缃€煎緢鎸戝墧銆傝浇鍏ラ敊璇殑鍊硷紝瑙ｇ爜鍣ㄥ氨浼氬け鏁堛€傞噸鏂板姞杞藉浐浠堕€氬父鑳芥仮澶嶏紝浣嗘湁鏃堕渶瑕佸浣嶃€傚浜庡寘鍚ぇ灏忎俊鎭殑瀵勫瓨鍣紝鎶婂畠浠殑鍦板潃璁句负 0 閫氬父鏄釜鍧忎富鎰忋€傚浜庡叾浠栨帶鍒跺瘎瀛樺櫒锛堝 2878锛夛紝鍙湁瀹冩寕璧锋椂浣犳墠浼氬彂鐜板摢浜涘€兼槸鍧忕殑銆?


	--------------------------------------------------------------------------------
	2800
	bit 0
		Decoder enable
		0 = disable
# 		1 = enable

	2804
	bits 0:31
		Decoder horizontal Y alias register 1
	---------------
	2808
	bits 0:31
		Decoder horizontal Y alias register 2
	---------------
	280C
	bits 0:31
		Decoder horizontal Y alias register 3
	---------------
	2810
	bits 0:31
		Decoder horizontal Y alias register 4
	---------------
	2814
	bits 0:31
		Decoder horizontal Y alias register 5
	---------------
	2818
	bits 0:31
		Decoder horizontal Y alias trigger

	These six registers control the horizontal aliasing filter for the Y plane.
	The first five registers must all be loaded before accessing the trigger
	(2818), as this register actually clocks the data through for the first
	five.

	To correctly program set the filter, this whole procedure must be done 16
	times. The actual register contents are copied from a lookup-table in the
	firmware which contains 4 different filter settings.

	--------------------------------------------------------------------------------
	281C
	bits 0:31
		Decoder horizontal UV alias register 1
	---------------
	2820
	bits 0:31
		Decoder horizontal UV alias register 2
	---------------
	2824
	bits 0:31
		Decoder horizontal UV alias register 3
	---------------
	2828
	bits 0:31
		Decoder horizontal UV alias register 4
	---------------
	282C
	bits 0:31
		Decoder horizontal UV alias register 5
	---------------
	2830
	bits 0:31
		Decoder horizontal UV alias trigger

	These six registers control the horizontal aliasing for the UV plane.
	Operation is the same as the Y filter, with 2830 being the trigger
	register.

	--------------------------------------------------------------------------------
	2834
	bits 0:15
		Decoder Y source width in pixels

	bits 16:31
		Decoder Y destination width in pixels
	---------------
	2838
	bits 0:15
		Decoder UV source width in pixels

	bits 16:31
		Decoder UV destination width in pixels

	NOTE: For both registers, the resulting image must be fully visible on
	screen. If the image exceeds the right edge both the source and destination
	size must be adjusted to reflect the visible portion. For the source width,
# 	you must take into account the scaling when calculating the new value.


	283C
	bits 0:31
		Decoder Y horizontal scaling
			Normally = Reg 2854 >> 2
	---------------
	2840
	bits 0:31
		Decoder ?? unknown - horizontal scaling
		Usually 0x00080514
	---------------
	2844
	bits 0:31
		Decoder UV horizontal scaling
		Normally = Reg 2854 >> 2
	---------------
	2848
	bits 0:31
		Decoder ?? unknown - horizontal scaling
		Usually 0x00100514
	---------------
	284C
	bits 0:31
		Decoder ?? unknown - Y plane
		Usually 0x00200020
	---------------
	2850
	bits 0:31
		Decoder ?? unknown - UV plane
		Usually 0x00200020
	---------------
	2854
	bits 0:31
		Decoder 'master' value for horizontal scaling
	---------------
	2858
	bits 0:31
		Decoder ?? unknown
# 		Usually 0

	285C
	bits 0:31
		Decoder ?? unknown
		Normally = Reg 2854 >> 1
	---------------
	2860
	bits 0:31
		Decoder ?? unknown
# 		Usually 0

	2864
	bits 0:31
		Decoder ?? unknown
		Normally = Reg 2854 >> 1
	---------------
	2868
	bits 0:31
		Decoder ?? unknown
		Usually 0

	Most of these registers either control horizontal scaling, or appear linked
	to it in some way. Register 2854 contains the 'master' value & the other
	registers can be calculated from that one. You must also remember to
	correctly set the divider in Reg 2874.

	To enlarge:
		Reg 2854 = (source_width * 0x00200000) / destination_width
		Reg 2874 = No divide

	To reduce from full size down to half size:
		Reg 2854 = (source_width/2 * 0x00200000) / destination width
		Reg 2874 = Divide by 2

	To reduce from half size down to quarter size:
		Reg 2854 = (source_width/4 * 0x00200000) / destination width
		Reg 2874 = Divide by 4

	The result is always rounded up.

	--------------------------------------------------------------------------------
	286C
	bits 0:15
		Decoder horizontal Y buffer offset

	bits 15:31
		Decoder horizontal UV buffer offset

	Offset into the video image buffer. If the offset is gradually incremented,
	the on screen image will move left & wrap around higher up on the right.

	--------------------------------------------------------------------------------
	2870
	bits 0:15
		Decoder horizontal Y output offset

	bits 16:31
		Decoder horizontal UV output offset

	Offsets the actual video output. Controls output alignment of the Y & UV
	planes. The higher the value, the greater the shift to the left. Use
	reg 2890 to move the image right.

	--------------------------------------------------------------------------------
	2874
	bits 0:1
		Decoder horizontal Y output size divider
		00 = No divide
		01 = Divide by 2
		10 = Divide by 3

	bits 4:5
		Decoder horizontal UV output size divider
		00 = No divide
		01 = Divide by 2
		10 = Divide by 3

	bit 8
		Decoder ?? unknown
		0 = Normal
		1 = Affects video output levels

	bit 16
		Decoder ?? unknown
		0 = Normal
		1 = Disable horizontal filter

	--------------------------------------------------------------------------------
	2878
	bit 0
		?? unknown

	bit 1
		osd on/off
		0 = osd off
		1 = osd on

	bit 2
		Decoder + osd video timing
		0 = NTSC
		1 = PAL

	bits 3:4
		?? unknown

	bit 5
		Decoder + osd
		Swaps upper & lower fields

	--------------------------------------------------------------------------------
	287C
	bits 0:10
		Decoder & osd ?? unknown
		Moves entire screen horizontally. Starts at 0x005 with the screen
		shifted heavily to the right. Incrementing in steps of 0x004 will
		gradually shift the screen to the left.

	bits 11:31
		?? unknown

	Normally contents are 0x00101111 (NTSC) or 0x1010111d (PAL)

	--------------------------------------------------------------------------------
	2880  --------    ?? unknown
# 	2884  --------    ?? unknown

	2888
	bit 0
		Decoder + osd ?? unknown
		0 = Normal
		1 = Misaligned fields (Correctable through 289C & 28A4)

	bit 4
		?? unknown

	bit 8
		?? unknown

	Warning: Bad values will require a firmware reload to recover.
# 			Known to be bad are 0x000,0x011,0x100,0x111

	288C
	bits 0:15
		osd ?? unknown
		Appears to affect the osd position stability. The higher the value the
		more unstable it becomes. Decoder output remains stable.

	bits 16:31
		osd ?? unknown
		Same as bits 0:15

	--------------------------------------------------------------------------------
	2890
	bits 0:11
		Decoder output horizontal offset.

	Horizontal offset moves the video image right. A small left shift is
	possible, but it's better to use reg 2870 for that due to its greater
	range.

	NOTE: Video corruption will occur if video window is shifted off the right
# 	edge. To avoid this read the notes for 2834 & 2838.

	2894
	bits 0:23
		Decoder output video surround colour.

	Contains the colour (in yuv) used to fill the screen when the video is
# 	running in a window.

	2898
	bits 0:23
		Decoder video window colour
		Contains the colour (in yuv) used to fill the video window when the
		video is turned off.

	bit 24
		Decoder video output
		0 = Video on
		1 = Video off

	bit 28
		Decoder plane order
		0 = Y,UV
		1 = UV,Y

	bit 29
		Decoder second plane byte order
		0 = Normal (UV)
		1 = Swapped (VU)

	In normal usage, the first plane is Y & the second plane is UV. Though the
	order of the planes can be swapped, only the byte order of the second plane
	can be swapped. This isn't much use for the Y plane, but can be useful for
	the UV plane.

	--------------------------------------------------------------------------------
	289C
	bits 0:15
		Decoder vertical field offset 1

	bits 16:31
		Decoder vertical field offset 2

	Controls field output vertical alignment. The higher the number, the lower
	the image on screen. Known starting values are 0x011E0017 (NTSC) &
# 	0x01500017 (PAL)

	28A0
	bits 0:15
		Decoder & osd width in pixels

	bits 16:31
		Decoder & osd height in pixels

	All output from the decoder & osd are disabled beyond this area. Decoder
	output will simply go black outside of this region. If the osd tries to
# 	exceed this area it will become corrupt.

	28A4
	bits 0:11
		osd left shift.

	Has a range of 0x770->0x7FF. With the exception of 0, any value outside of
# 	this range corrupts the osd.

	28A8
	bits 0:15
		osd vertical field offset 1

	bits 16:31
		osd vertical field offset 2

	Controls field output vertical alignment. The higher the number, the lower
	the image on screen. Known starting values are 0x011E0017 (NTSC) &
# 	0x01500017 (PAL)

	28AC  --------    ?? unknown
	|
	V
# 	28BC  --------    ?? unknown

	28C0
	bit 0
		Current output field
		0 = first field
		1 = second field

	bits 16:31
		Current scanline
		The scanline counts from the top line of the first field
# 		through to the last line of the second field.

	28C4  --------    ?? unknown
	|
	V
# 	28F8  --------    ?? unknown

	28FC
	bit 0
		?? unknown
		0 = Normal
# 		1 = Breaks decoder & osd output

	2900
	bits 0:31
		Decoder vertical Y alias register 1
	---------------
	2904
	bits 0:31
		Decoder vertical Y alias register 2
	---------------
	2908
	bits 0:31
		Decoder vertical Y alias trigger

	These three registers control the vertical aliasing filter for the Y plane.
	Operation is similar to the horizontal Y filter (2804). The only real
	difference is that there are only two registers to set before accessing
	the trigger register (2908). As for the horizontal filter, the values are
	taken from a lookup table in the firmware, and the procedure must be
# 	repeated 16 times to fully program the filter.

	290C
	bits 0:31
		Decoder vertical UV alias register 1
	---------------
	2910
	bits 0:31
		Decoder vertical UV alias register 2
	---------------
	2914
	bits 0:31
		Decoder vertical UV alias trigger

	These three registers control the vertical aliasing filter for the UV
# 	plane. Operation is the same as the Y filter, with 2914 being the trigger.

	2918
	bits 0:15
		Decoder Y source height in pixels

	bits 16:31
		Decoder Y destination height in pixels
	---------------
	291C
	bits 0:15
		Decoder UV source height in pixels divided by 2

	bits 16:31
		Decoder UV destination height in pixels

	NOTE: For both registers, the resulting image must be fully visible on
	screen. If the image exceeds the bottom edge both the source and
	destination size must be adjusted to reflect the visible portion. For the
	source height, you must take into account the scaling when calculating the
# 	new value.

	2920
	bits 0:31
		Decoder Y vertical scaling
		Normally = Reg 2930 >> 2
	---------------
	2924
	bits 0:31
		Decoder Y vertical scaling
		Normally = Reg 2920 + 0x514
	---------------
	2928
	bits 0:31
		Decoder UV vertical scaling
		When enlarging = Reg 2930 >> 2
		When reducing = Reg 2930 >> 3
	---------------
	292C
	bits 0:31
		Decoder UV vertical scaling
		Normally = Reg 2928 + 0x514
	---------------
	2930
	bits 0:31
		Decoder 'master' value for vertical scaling
	---------------
	2934
	bits 0:31
		Decoder ?? unknown - Y vertical scaling
	---------------
	2938
	bits 0:31
		Decoder Y vertical scaling
		Normally = Reg 2930
	---------------
	293C
	bits 0:31
		Decoder ?? unknown - Y vertical scaling
	---------------
	2940
	bits 0:31
		Decoder UV vertical scaling
		When enlarging = Reg 2930 >> 1
		When reducing = Reg 2930
	---------------
	2944
	bits 0:31
		Decoder ?? unknown - UV vertical scaling
	---------------
	2948
	bits 0:31
		Decoder UV vertical scaling
		Normally = Reg 2940
	---------------
	294C
	bits 0:31
		Decoder ?? unknown - UV vertical scaling

	Most of these registers either control vertical scaling, or appear linked
	to it in some way. Register 2930 contains the 'master' value & all other
	registers can be calculated from that one. You must also remember to
	correctly set the divider in Reg 296C

	To enlarge:
		Reg 2930 = (source_height * 0x00200000) / destination_height
		Reg 296C = No divide

	To reduce from full size down to half size:
		Reg 2930 = (source_height/2 * 0x00200000) / destination height
		Reg 296C = Divide by 2

	To reduce from half down to quarter.
		Reg 2930 = (source_height/4 * 0x00200000) / destination height
		Reg 296C = Divide by 4

	--------------------------------------------------------------------------------
	2950
	bits 0:15
		Decoder Y line index into display buffer, first field

	bits 16:31
# 		Decoder Y vertical line skip, first field

	2954
	bits 0:15
		Decoder Y line index into display buffer, second field

	bits 16:31
# 		Decoder Y vertical line skip, second field

	2958
	bits 0:15
		Decoder UV line index into display buffer, first field

	bits 16:31
# 		Decoder UV vertical line skip, first field

	295C
	bits 0:15
		Decoder UV line index into display buffer, second field

	bits 16:31
# 		Decoder UV vertical line skip, second field

	2960
	bits 0:15
		Decoder destination height minus 1

	bits 16:31
# 		Decoder destination height divided by 2

	2964
	bits 0:15
		Decoder Y vertical offset, second field

	bits 16:31
		Decoder Y vertical offset, first field

	These two registers shift the Y plane up. The higher the number, the
# 	greater the shift.

	2968
	bits 0:15
		Decoder UV vertical offset, second field

	bits 16:31
		Decoder UV vertical offset, first field

	These two registers shift the UV plane up. The higher the number, the
# 	greater the shift.

	296C
	bits 0:1
		Decoder vertical Y output size divider
		00 = No divide
		01 = Divide by 2
		10 = Divide by 4

	bits 8:9
		Decoder vertical UV output size divider
		00 = No divide
		01 = Divide by 2
# 		10 = Divide by 4

	2970
	bit 0
		Decoder ?? unknown
		0 = Normal
		1 = Affect video output levels

	bit 16
		Decoder ?? unknown
		0 = Normal
		1 = Disable vertical filter

	--------------------------------------------------------------------------------
	2974  --------   ?? unknown
	|
	V
# 	29EF  --------   ?? unknown

	2A00
	bits 0:2
		osd colour mode
		000 = 8 bit indexed
		001 = 16 bit (565)
		010 = 15 bit (555)
		011 = 12 bit (444)
		100 = 32 bit (8888)

	bits 4:5
		osd display bpp
		01 = 8 bit
		10 = 16 bit
		11 = 32 bit

	bit 8
		osd global alpha
		0 = Off
		1 = On

	bit 9
		osd local alpha
		0 = Off
		1 = On

	bit 10
		osd colour key
		0 = Off
		1 = On

	bit 11
		osd ?? unknown
		Must be 1

	bit 13
		osd colour space
		0 = ARGB
		1 = AYVU

	bits 16:31
		osd ?? unknown
		Must be 0x001B (some kind of buffer pointer 锛?

	When the bits-per-pixel is set to 8, the colour mode is ignored and
	assumed to be 8 bit indexed. For 16 & 32 bits-per-pixel the colour depth
	is honoured, and when using a colour depth that requires fewer bytes than
	allocated the extra bytes are used as padding. So for a 32 bpp with 8 bit
	index colour, there are 3 padding bytes per pixel. It's also possible to
	select 16bpp with a 32 bit colour mode. This results in the pixel width
	being doubled, but the color key will not work as expected in this mode.

	Colour key is as it suggests. You designate a colour which will become
	completely transparent. When using 565, 555 or 444 colour modes, the
	colour key is always 16 bits wide. The colour to key on is set in Reg 2A18.

	Local alpha works differently depending on the colour mode. For 32bpp & 8
	bit indexed, local alpha is a per-pixel 256 step transparency, with 0 being
	transparent and 255 being solid. For the 16bpp modes 555 & 444, the unused
	bit(s) act as a simple transparency switch, with 0 being solid & 1 being
	fully transparent. There is no local alpha support for 16bit 565.

	Global alpha is a 256 step transparency that applies to the entire osd,
	with 0 being transparent & 255 being solid.

# 	It's possible to combine colour key, local alpha & global alpha.

	2A04
	bits 0:15
		osd x coord for left edge

	bits 16:31
		osd y coord for top edge
	---------------
	2A08
	bits 0:15
		osd x coord for right edge

	bits 16:31
		osd y coord for bottom edge

	For both registers, (0,0) = top left corner of the display area. These
	registers do not control the osd size, only where it's positioned & how
	much is visible. The visible osd area cannot exceed the right edge of the
	display, otherwise the osd will become corrupt. See reg 2A10 for
# 	setting osd width.

	2A0C
	bits 0:31
		osd buffer index

	An index into the osd buffer. Slowly incrementing this moves the osd left,
# 	wrapping around onto the right edge

	2A10
	bits 0:11
		osd buffer 32 bit word width

	Contains the width of the osd measured in 32 bit words. This means that all
# 	colour modes are restricted to a byte width which is divisible by 4.

	2A14
	bits 0:15
		osd height in pixels

	bits 16:32
		osd line index into buffer
# 		osd will start displaying from this line.

	2A18
	bits 0:31
		osd colour key

# 	Contains the colour value which will be transparent.

	2A1C
	bits 0:7
		osd global alpha

# 	Contains the global alpha value (equiv ivtvfbctl --alpha XX)

	2A20  --------    ?? unknown
	|
	V
# 	2A2C  --------    ?? unknown

	2A30
	bits 0:7
		osd colour to change in indexed palette
	---------------
	2A34
	bits 0:31
		osd colour for indexed palette

	To set the new palette, first load the index of the colour to change into
	2A30, then load the new colour into 2A34. The full palette is 256 colours,
# 	so the index range is 0x00-0xFF

	2A38  --------    ?? unknown
# 	2A3C  --------    ?? unknown

	2A40
	bits 0:31
		osd ?? unknown

# 	Affects overall brightness, wrapping around to black

	2A44
	bits 0:31
		osd ?? unknown

# 	Green tint

	2A48
	bits 0:31
		osd ?? unknown

# 	Red tint

	2A4C
	bits 0:31
		osd ?? unknown

# 	Affects overall brightness, wrapping around to black

	2A50
	bits 0:31
		osd ?? unknown

# 	Colour shift

	2A54
	bits 0:31
		osd ?? unknown

# 	Colour shift

	2A58  --------    ?? unknown
	|
	V
# 	2AFC  --------    ?? unknown

	2B00
	bit 0
		osd filter control
		0 = filter off
		1 = filter on

	bits 1:4
		osd ?? unknown

	--------------------------------------------------------------------------------

### The cx231xx DMA engine



鏈〉鎻忚堪 cx2341x DMA 寮曟搸鎵€浣跨敤鐨勭粨鏋勪笌娴佺▼銆?

#### 绠€浠?


cx2341x 鐨?PCI 鎺ュ彛鍏峰鎬荤嚎涓绘帶锛坆usmaster锛夎兘鍔涖€傝繖鎰忓懗鐫€瀹冩湁涓€涓?DMA 寮曟搸锛屽彲浠ュ湪鍗′笌涓诲瓨涔嬮棿楂樻晥鍦颁紶杈撳ぇ閲忔暟鎹紝鑰屾棤闇€ CPU 鍗忓姪銆傚拰澶у鏁扮‖浠朵竴鏍凤紝瀹冨繀椤诲湪杩炵画鐨勭墿鐞嗗唴瀛樹笂鎿嶄綔銆傚湪铏氭嫙鍐呭瓨鏈哄櫒涓婏紝澶у潡鐨勮繛缁墿鐞嗗唴瀛樺緢闅捐幏寰椼€?

鍥犳锛屽畠涔熸敮鎸佷竴绉嶇О涓衡€渟catter-gather鈥濓紙鍒嗘暎/鑱氶泦锛夌殑鎶€鏈€傝鍗″彲浠ュ湪涓€娆℃搷浣滀腑浼犺緭澶氫釜缂撳啿鍖恒€傞┍鍔ㄦ棤闇€鍒嗛厤涓€涓ぇ鐨勮繛缁紦鍐插尯锛岃€屽彲浠ュ垎閰嶅涓緝灏忕殑缂撳啿鍖恒€?

鍦ㄥ疄璺典腑锛屾垜瑙佽繃骞冲潎浼犺緭閲忓ぇ绾︽槸 80K锛屼絾瓒呰繃 128K 鐨勪紶杈撳苟涓嶅皯瑙侊紝灏ゅ叾鏄湪鍚姩鏃躲€?28K 杩欎釜鏁板瓧寰堥噸瑕侊紝鍥犱负瀹冩槸鍐呮牳閫氬父鑳藉垎閰嶇殑鏈€澶у潡銆傚嵆渚垮姝わ紝128K 鐨勫潡涔熷緢闅捐幏寰楋紝鍥犳寮虹儓寤鸿椹卞姩缂栧啓鑰呴€夋嫨鏇村皬鐨勫潡澶у皬锛屽苟瀛︿範 scatter-gather 鎶€鏈€?

閭 #10 淇濈暀缁?DMA 浼犺緭淇℃伅浣跨敤銆?

娉ㄦ剰锛氱‖浠舵湡鏈涘皬绔暟鎹紙'intel format'锛夈€?

#### 娴佺▼


鏈妭鎬讳綋涓婃弿杩板鐞?DMA 浼犺緭鏃朵簨浠剁殑椤哄簭銆傝缁嗕俊鎭湪鏈妭涔嬪悗缁欏嚭銆?

- 鍗¤Е鍙?Encoder 涓柇銆?
- 椹卞姩浠庨偖绠?#10 璇诲彇浼犺緭绫诲瀷銆佸亸绉诲拰澶у皬銆?
- 椹卞姩鐢ㄨ冻澶熷鐨勭┖闂?dma 缂撳啿鍖烘瀯閫?scatter-gather 鏁扮粍浠ヨ鐩栬澶у皬銆?
- 椹卞姩閫氳繃 ScheduleDMAtoHost API 璋冪敤璋冨害 DMA 浼犺緭銆?
- 鍗¤Е鍙?DMA Complete 涓柇銆?
- 椹卞姩妫€鏌?DMA 鐘舵€佸瘎瀛樺櫒浠ュ彂鐜颁换浣曢敊璇€?
- 椹卞姩瀵瑰垰浼犺緭鐨勭紦鍐插尯杩涜鍚庡鐞嗐€?

娉ㄦ剰锛丒ncoder 鍜?DMA Complete 涓柇鏈夊彲鑳藉悓鏃惰瑙﹀彂銆傦紙涓婁竴娆＄殑缁撴潫銆佷笅涓€娆＄殑寮€濮嬶紝绛夌瓑銆傦級

#### 閭 #10


Flags銆丆ommand銆丷eturn Value 鍜?Timeout 瀛楁琚拷鐣ャ€?

- Name:       Mailbox #10
- Results[^0^]: Type: 0: MPEG.
- Results[^1^]: Offset: 鐩稿浜庡崱鍐呭瓨绌洪棿鐨勪綅缃€?
- Results[^2^]: Size: 瑕佷紶杈撶殑纭垏瀛楄妭鏁般€?

鎴戞帹娴嬶紝鏃㈢劧 StartCapture API 鏈変竴涓彲鐢ㄧ殑鎹曡幏绫诲瀷 "RAW"锛岄偅涔?type 瀛楁灏嗕細鏈夊搴?YUV 鍜?PCM 鏁版嵁鐨勫叾浠栧€笺€?

#### Scatter-Gather 鏁扮粍


scatter-gather 鏁扮粍鏄竴鍧楄繛缁垎閰嶇殑鍐呭瓨锛屽畠鍛婅瘔鍗℃瘡涓緟浼犺緭鏁版嵁鍧楃殑婧愬拰鐩殑銆傚崱鐨勨€滃湴鍧€鈥濈敱閭 #10 鎻愪緵鐨勫亸绉绘帹瀵艰€屾潵銆備富鏈哄湴鍧€鏄洰鏍?DMA 缂撳啿鍖虹殑鐗╃悊鍐呭瓨浣嶇疆銆?

姣忎釜 S-G 鏁扮粍鍏冪礌鏄竴涓敱涓変釜 32 浣嶅瓧缁勬垚鐨勭粨鏋勪綋銆傜涓€涓瓧鏄簮鍦板潃锛岀浜屼釜鏄洰鐨勫湴鍧€銆備袱鑰呭悇鍗犳弧 32 浣嶃€傜涓変釜瀛楃殑浣?18 浣嶆槸浼犺緭瀛楄妭璁℃暟銆傜涓変釜瀛楃殑鏈€楂樹綅鏄€渓ast鈥濇爣蹇椼€俵ast 鏍囧織鍛婅瘔鍗¤Е鍙?DMA_DONE 涓柇銆傛牴鎹垜鐥涜嫤鐨勪釜浜虹粡楠岋紝濡傛灉浣犲繕浜嗚缃繖涓瘮鐗癸紝鍗′粛浼氣€滃伐浣溾€濓紝浣嗘祦鏋佹湁鍙兘琚崯鍧忋€?

浼犺緭璁℃暟蹇呴』鏄?256 鐨勫€嶆暟銆傚洜姝わ紝椹卞姩闇€瑕佽窡韪洰鏍囩紦鍐插尯涓湁澶氬皯鏁版嵁鏄湁鏁堢殑锛屽苟鐩稿簲鍦板鐞嗐€?

鏁扮粍鍏冪礌锛?

- 32 浣嶆簮鍦板潃
- 32 浣嶇洰鐨勫湴鍧€
- 14 浣嶄繚鐣欙紙鏈€楂樹綅鏄?last 鏍囧織锛?
- 18 浣嶅瓧鑺傝鏁?

#### DMA 浼犺緭鐘舵€?


瀵勫瓨鍣?0x0004 淇濆瓨 DMA 浼犺緭鐘舵€侊細

- bit 0:   read completed
- bit 1:   write completed
- bit 2:   DMA read error
- bit 3:   DMA write error
- bit 4:   Scatter-Gather array error


