## EP93xx LCD 鎺у埗鍣ㄩ┍鍔?
EP93xx LCD 鎺у埗鍣ㄥ彲浠ュ悓鏃堕┍鍔ㄦ爣鍑嗙殑妗岄潰鏄剧ず鍣ㄥ拰宓屽叆寮?LCD 鏄剧ず灞忋€傚鏋滀綘鎷ユ湁鏍囧噯鐨?妗岄潰鏄剧ず鍣紝閭ｄ箞
```

	static struct ep93xxfb_mach_info some_board_fb_info = {
		.num_modes	= EP93XXFB_USE_MODEDB,
		.bpp		= 16,
	};

```
濡傛灉浣犳嫢鏈夊祵鍏ュ紡 LCD 鏄剧ず灞忥紝鍒欓渶瑕佸畾涔変竴娈佃棰?```

	static struct fb_videomode some_board_video_modes[] = {
		{
			.name		= "some_lcd_name",
			/* Pixel clock, porches, etc */
		},
	};

```
娉ㄦ剰鍍忕礌鏃堕挓鍊间互鐨锛坧ico-seconds锛変负鍗曚綅銆備綘鍙互浣跨敤 KHZ2PICOS 瀹忔潵杞崲鍍忕礌鏃堕挓
鍊笺€傚ぇ澶氭暟鍏跺畠鍊间互鍍忕礌鏃堕挓涓哄崟浣嶃€傛洿澶氱粏鑺傚弬瑙?Documentation/fb/framebuffer.rst銆?
浣犳澘鍗＄殑 ep93xxfb_mach_info 缁撴瀯搴旂被浼间簬
```

	static struct ep93xxfb_mach_info some_board_fb_info = {
		.num_modes	= ARRAY_SIZE(some_board_video_modes),
		.modes		= some_board_video_modes,
		.default_mode	= &some_board_video_modes[0],
		.bpp		= 16,
	};

```
鍙互閫氳繃鍦ㄤ笅闈㈡坊鍔犱互涓嬪唴瀹规潵娉ㄥ唽甯х紦鍐茶澶?```

	ep93xx_register_fb(&some_board_fb_info);

```
## 瑙嗛灞炴€ф爣蹇?
ep93xxfb_mach_info 缁撴瀯鏈変竴涓?flags 瀛楁锛屽彲鐢ㄤ簬閰嶇疆鎺у埗鍣ㄣ€傝棰戝睘鎬ф爣蹇楀湪 EP93xx
鐢ㄦ埛鎸囧崡鐨勭 7 鑺備腑鏈夊畬鏁磋鏄庛€傚彲鐢ㄧ殑鏍囧織濡備笅锛?
=============================== ==========================================
EP93XXFB_PCLK_FALLING		Clock data on the falling edge of the
				pixel clock. The default is to clock
				data on the rising edge.

EP93XXFB_SYNC_BLANK_HIGH	Blank signal is active high. By
				default the blank signal is active low.

EP93XXFB_SYNC_HORIZ_HIGH	Horizontal sync is active high. By
				default the horizontal sync is active low.

EP93XXFB_SYNC_VERT_HIGH		Vertical sync is active high. By
				default the vertical sync is active high.
=============================== ==========================================

甯х紦鍐插尯鐨勭墿鐞嗗湴鍧€鍙互浣跨敤浠ヤ笅鏍囧織鏉ユ帶鍒讹細

=============================== ======================================
EP93XXFB_USE_SDCSN0		Use SDCSn[^0^] for the framebuffer. This
				is the default setting.

EP93XXFB_USE_SDCSN1		Use SDCSn[^1^] for the framebuffer.

EP93XXFB_USE_SDCSN2		Use SDCSn[^2^] for the framebuffer.

EP93XXFB_USE_SDCSN3		Use SDCSn[^3^] for the framebuffer.
=============================== ======================================

## 骞冲彴鍥炶皟

EP93xx 甯х紦鍐查┍鍔ㄦ敮鎸佷笁涓彲閫夌殑骞冲彴鍥炶皟锛歴etup銆乼eardown 鍜?blank銆俿etup 鍜?teardown
鍑芥暟鍒嗗埆鍦ㄥ抚缂撳啿椹卞姩琚畨瑁呭拰绉婚櫎鏃惰皟鐢ㄣ€俠lank 鍑芥暟鍦ㄦ樉绀哄櫒琚秷闅愶紙blank锛夋垨鍙栨秷娑堥殣
锛坲nblank锛夋椂璋冪敤銆?
setup 鍜?teardown 璁惧灏?platform_device 缁撴瀯浣滀负鍙傛暟浼犲叆銆俧b_info 鍜?ep93xxfb_mach_info 缁撴瀯鍙互浠?```

	static int some_board_fb_setup(struct platform_device *pdev)
	{
		struct ep93xxfb_mach_info *mach_info = pdev->dev.platform_data;
		struct fb_info *fb_info = platform_get_drvdata(pdev);

		/* Board specific framebuffer setup */
	}

```
## 璁剧疆瑙嗛妯″紡

```

	video=XRESxYRES[-BPP][@REFRESH]

```
濡傛灉 EP93xx 瑙嗛椹卞姩鏄唴寤虹殑锛屽垯瑙嗛妯″紡鍦?```

	video=ep93xx-fb:800x600-16@60

```
涓缃€傚鏋?EP93xx 瑙嗛椹卞姩鏄綔涓烘ā鍧楁瀯寤虹殑锛屽垯瑙嗛妯″紡鍦?```

	modprobe ep93xx-fb video=320x240

```
涓缃€?## Screenpage 缂洪櫡锛坆ug锛?
鑷冲皯鍦?EP9315 涓婂瓨鍦ㄤ竴涓缂洪櫡锛屼細瀵艰嚧 VIDSCRNPAGE锛堝抚缂撳啿鐗╃悊鍋忕Щ锛夌殑绗?27 浣嶈
鍥哄畾涓轰綆鐢靛钩銆傚瓨鍦?```

	https://marc.info/?l=linux-arm-kernel&m=110061245502000&w=2

```
榛樿鎯呭喌涓嬶紝EP93xx 甯х紦鍐查┍鍔ㄤ細妫€鏌ュ凡鍒嗛厤鐨勭墿鐞嗗湴鍧€鐨勭 27 浣嶆槸鍚﹁璁剧疆銆傚鏋滆缃簡锛?鍒欓噴鏀捐鍐呭瓨骞惰繑鍥為敊璇€傚彲浠ラ€氳繃灏嗕互涓嬪唴瀹规坊鍔犳潵绂佺敤璇ユ鏌?```

      ep93xx-fb.check_screenpage_bug=0

```
鍦ㄦ煇浜涙儏鍐典笅锛屽彲浠ラ噸鏂伴厤缃綘鐨?SDRAM 甯冨眬鏉ヨ閬挎缂洪櫡銆傝瑙?EP93xx 鐢ㄦ埛鎸囧崡绗?13 鑺傘€?