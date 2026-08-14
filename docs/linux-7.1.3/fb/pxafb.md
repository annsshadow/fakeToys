## PXA25x LCD 鎺у埗鍣ㄩ┍鍔?

璇ラ┍鍔ㄦ敮鎸佷互涓嬮€夐」锛屾ā鍧楁柟寮忎笅閫氳繃 `options=<OPTIONS>`锛屽唴寤烘柟寮忎笅閫氳繃 `video=pxafb:<OPTIONS>`銆?
```

	modprobe pxafb options=vmem:2M,mode:640x480-8,passive

```
```

	video=pxafb:vmem:2M,mode:640x480-8,passive

```
vmem: VIDEO_MEM_SIZE

	瑕佸垎閰嶇殑鏄惧瓨澶у皬锛堝彲鍔犲悗缂€ K 鎴?M 琛ㄧず鍗冨瓧鑺傛垨鍏嗗瓧鑺傦級

mode:XRESxYRES[-BPP]

	XRES == LCCR1_PPL + 1

	YRES == LLCR2_LPP + 1

		浠ュ儚绱犱负鍗曚綅鐨勬樉绀哄垎杈ㄧ巼

	BPP == 浣嶆繁銆傚悎娉曞€间负 1銆?銆?銆? 鍜?16銆?
pixclock:PIXCLOCK

	鍍忕礌鏃堕挓锛屽崟浣嶄负鐨

left:LEFT == LCCR1_BLW + 1

right:RIGHT == LCCR1_ELW + 1

hsynclen:HSYNC == LCCR1_HSW + 1

upper:UPPER == LCCR2_BFW

lower:LOWER == LCCR2_EFR

vsynclen:VSYNC == LCCR2_VSW + 1

	鏄剧ず杈硅窛涓庡悓姝ユ椂闂?
color | mono => LCCR0_CMS

	鍡€︹€?
active | passive => LCCR0_PAS

	涓诲姩锛圱FT锛夋垨琚姩锛圫TN锛夋樉绀?
single | dual => LCCR0_SDS

	鍗曢潰鏉挎垨鍙岄潰鏉胯鍔ㄦ樉绀?
4pix | 8pix => LCCR0_DPD

	4 鎴?8 鍍忕礌鍗曡壊鍗曢潰鏉挎暟鎹?
hsync:HSYNC, vsync:VSYNC

	姘村钩涓庡瀭鐩村悓姝ャ€? => 浣庣數骞虫湁鏁堬紝1 => 楂樼數骞虫湁鏁堛€?
dpc:DPC

	鍙屽€嶅儚绱犳椂閽熴€?=>鐪燂紝0=>鍋?
outputen:POLARITY

	杈撳嚭浣胯兘鏋佹€с€? => 浣庣數骞虫湁鏁堬紝1 => 楂樼數骞虫湁鏁?
pixclockpol:POLARITY

	鍍忕礌鏃堕挓鏋佹€?	0 => 涓嬮檷娌匡紝1 => 涓婂崌娌?

## PXA27x 鍙婃洿楂樼増鏈?LCD 鎺у埗鍣ㄧ殑鍙犲姞灞傛敮鎸?

  PXA27x 鍙婃洿楂樼増鏈殑澶勭悊鍣ㄥ湪鍩虹甯х紦鍐蹭箣涓婃敮鎸?overlay1 涓?overlay2锛堝綋鐒朵篃鍙互浣嶄簬鍩虹灞備箣涓嬶級銆傚畠浠敮鎸佸甫璋冭壊鏉夸笌鏃犺皟鑹叉澘鐨?RGB 鏍煎紡锛屼互鍙?YUV 鏍煎紡锛堜粎鍦?overlay2 涓婂彲鐢級銆傝繖浜涘彔鍔犲眰鎷ユ湁涓撶敤鐨?DMA 閫氶亾锛岃涓烘柟寮忎笌甯х紦鍐茬被浼笺€?
  鐒惰€岋紝杩欎簺鍙犲姞灞傚抚缂撳啿涓庢櫘閫氬抚缂撳啿涔嬮棿瀛樺湪涓€浜涘樊寮傦紝濡備笅鎵€绀猴細

  1. 鍙犲姞灞傚彲浠ヨ捣濮嬩簬鍩虹甯х紦鍐蹭腑 32 浣嶅瓧瀵归綈鐨勪綅缃紝杩欐剰鍛崇潃瀹冧滑鍏锋湁涓€涓捣濮嬪潗鏍?(x, y)銆傝淇℃伅琚紪鐮佽繘 `var->nonstd`锛堟敞鎰忥紝`var->xoffset` 鍜?`var->yoffset` 骞堕潪鐢ㄤ簬姝ょ洰鐨勶級銆?
  2. 鍙犲姞灞傚抚缂撳啿鏍规嵁鎸囧畾鐨勫唴瀹瑰姩鎬佸垎閰?
```

	var->xres_virtual * var->yres_virtual * bpp

     bpp = 16 -- for RGB565 or RGBT555

     bpp = 24 -- for YUV444 packed

     bpp = 24 -- for YUV444 planar

     bpp = 16 -- for YUV422 planar (1 pixel = 1 Y + 1/2 Cb + 1/2 Cr)

     bpp = 12 -- for YUV420 planar (1 pixel = 1 Y + 1/4 Cb + 1/4 Cr)

     NOTE:

     a. 鍙犲姞灞備笉鏀寔 x 鏂瑰悜骞崇Щ锛屽洜姝?	var->xres_virtual 灏嗗缁堢瓑浜?var->xres

     b. 鍙犲姞灞傜殑琛岄暱搴﹀繀椤讳綅浜?32 浣嶅瓧杈圭晫涓婏紝
	瀵逛簬 YUV planar 妯″紡锛岃繖鏄拡瀵规瘡鍍忕礌浣嶆暟鏈€灏戠殑
	鍒嗛噺鑰岃█鐨勮姹傦紝渚嬪瀵逛簬 YUV420锛屼竴涓儚绱犵殑 Cr 鍒嗛噺
	瀹為檯涓?2 浣嶏紝杩欐剰鍛崇潃琛岄暱搴﹀簲涓?16 鍍忕礌鐨勬暣鏁板€?
     c. 璧峰姘村钩浣嶇疆锛圶POS锛夊簲浣嶄簬 32 浣嶅瓧杈圭晫涓婏紝
	鍚﹀垯 fb_check_var() 灏嗙洿鎺ュけ璐ャ€?
     d. 鍙犲姞灞傜殑鐭╁舰鍖哄煙搴斾綅浜庡熀纭€骞抽潰涔嬪唴锛?	鍚﹀垯澶辫触

     Applications should follow the sequence below to operate an overlay
     framebuffer:

	 a. open("/dev/fb[1-2]", ...)
	 b. ioctl(fd, FBIOGET_VSCREENINFO, ...)
	 c. modify 'var' with desired parameters:

	    1) var->xres and var->yres
	    2) 濡傛灉闇€瑕佹洿澶氬唴瀛橈紙閫氬父鐢ㄤ簬鍙岀紦鍐诧級锛?	       澧炲ぇ var->yres_virtual
	    3) var->nonstd 鐢ㄤ簬璧峰 (x, y) 涓庨鑹叉牸寮?	    4) 鑻ヤ娇鐢?RGB 妯″紡锛屽垯璁剧疆 var->{red, green, blue, transp}

	 d. ioctl(fd, FBIOPUT_VSCREENINFO, ...)
	 e. ioctl(fd, FBIOGET_FSCREENINFO, ...)
	 f. mmap
	 g. ...

  3. 瀵逛簬 YUV planar 鏍煎紡锛屽抚缂撳啿妗嗘灦瀹為檯涓婂苟涓嶆敮鎸侊紝搴旂敤绋嬪簭蹇呴』鑷澶勭悊鍚勫垎閲忓湪甯х紦鍐蹭腑鐨勫亸绉讳笌闀垮害銆?
  4. `var->nonstd` 鐢ㄤ簬浼犻€掕捣濮?(x, y) 浣嶇疆涓庨鑹叉牸寮忥紝璇︾粏鐨勪綅鍩熷涓嬫墍绀?:

      31                23  20         10          0
       +-----------------+---+----------+----------+
       |  ... unused ... |FOR|   XPOS   |   YPOS   |
       +-----------------+---+----------+----------+

     FOR  - 棰滆壊鏍煎紡锛岀敱 pxafb.h 涓殑 OVERLAY_FORMAT_* 瀹氫箟

	  - 0 - RGB
	  - 1 - YUV444 PACKED
	  - 2 - YUV444 PLANAR
	  - 3 - YUV422 PLANAR
	  - 4 - YUR420 PLANAR

     XPOS - 璧峰姘村钩浣嶇疆

     YPOS - 璧峰鍨傜洿浣嶇疆

```
