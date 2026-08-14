## VIA 闆嗘垚鍥惧舰鑺墖鎺у埗鍙板抚缂撳啿椹卞姩


### 骞冲彴


    鎺у埗鍙板抚缂撳啿椹卞姩閫傜敤浜?VIA UniChrome 瀹舵棌鐨勫浘褰㈣姱鐗?    锛圕LE266銆丳M800 / CN400 / CN300銆?    P4M800CE / P4M800Pro / CN700 / VN800銆?    CX700 / VX700銆並8M890銆丳4M890銆?    CN896 / P4M900銆乂X800銆乂X855锛?
### 椹卞姩鐗规€?

    璁惧锛欳RT銆丩CD銆丏VI

```
	CRT:
	    640x480(60, 75, 85, 100, 120 Hz), 720x480(60 Hz),
	    720x576(60 Hz), 800x600(60, 75, 85, 100, 120 Hz),
	    848x480(60 Hz), 856x480(60 Hz), 1024x512(60 Hz),
	    1024x768(60, 75, 85, 100 Hz), 1152x864(75 Hz),
	    1280x768(60 Hz), 1280x960(60 Hz), 1280x1024(60, 75, 85 Hz),
	    1440x1050(60 Hz), 1600x1200(60, 75 Hz), 1280x720(60 Hz),
	    1920x1080(60 Hz), 1400x1050(60 Hz), 800x480(60 Hz)

    color depth: 8 bpp, 16 bpp, 32 bpp supports.

    Support 2D hardware accelerator.
```

### 浣跨敤 viafb 妯″潡


```
	#modprobe viafb

    Start viafb with user options::

	#modprobe viafb viafb_mode=800x600 viafb_bpp=16 viafb_refresh=60
		  viafb_active_dev=CRT+DVI viafb_dvi_port=DVP1
		  viafb_mode1=1024x768 viafb_bpp=16 viafb_refresh1=60
		  viafb_SAMM_ON=1

    viafb_mode:
	- 640x480 (榛樿)
	- 720x480
	- 800x600
	- 1024x768

    viafb_bpp:
	- 8, 16, 32 (榛樿:32)

    viafb_refresh:
	- 60, 75, 85, 100, 120 (榛樿:60)

    viafb_lcd_dsp_method:
	- 0 : 鎵╁睍锛堥粯璁わ級
	- 1 : 灞呬腑

    viafb_lcd_mode:
	0 : LSB 鏁版嵁鏍煎紡杈撳叆鐨?LCD 闈㈡澘锛堥粯璁わ級
	1 : MSB 鏁版嵁鏍煎紡杈撳叆鐨?LCD 闈㈡澘

    viafb_lcd_panel_id:
	- 0 : 鍒嗚鲸鐜? 640x480, 閫氶亾: 鍗? 鎶栧姩: 鍚敤
	- 1 : 鍒嗚鲸鐜? 800x600, 閫氶亾: 鍗? 鎶栧姩: 鍚敤
	- 2 : 鍒嗚鲸鐜? 1024x768, 閫氶亾: 鍗? 鎶栧姩: 鍚敤锛堥粯璁わ級
	- 3 : 鍒嗚鲸鐜? 1280x768, 閫氶亾: 鍗? 鎶栧姩: 鍚敤
	- 4 : 鍒嗚鲸鐜? 1280x1024, 閫氶亾: 鍙? 鎶栧姩: 鍚敤
	- 5 : 鍒嗚鲸鐜? 1400x1050, 閫氶亾: 鍙? 鎶栧姩: 鍚敤
	- 6 : 鍒嗚鲸鐜? 1600x1200, 閫氶亾: 鍙? 鎶栧姩: 鍚敤

	- 8 : 鍒嗚鲸鐜? 800x480, 閫氶亾: 鍗? 鎶栧姩: 鍚敤
	- 9 : 鍒嗚鲸鐜? 1024x768, 閫氶亾: 鍙? 鎶栧姩: 鍚敤
	- 10: 鍒嗚鲸鐜? 1024x768, 閫氶亾: 鍗? 鎶栧姩: 绂佺敤
	- 11: 鍒嗚鲸鐜? 1024x768, 閫氶亾: 鍙? 鎶栧姩: 绂佺敤
	- 12: 鍒嗚鲸鐜? 1280x768, 閫氶亾: 鍗? 鎶栧姩: 绂佺敤
	- 13: 鍒嗚鲸鐜? 1280x1024, 閫氶亾: 鍙? 鎶栧姩: 绂佺敤
	- 14: 鍒嗚鲸鐜? 1400x1050, 閫氶亾: 鍙? 鎶栧姩: 绂佺敤
	- 15: 鍒嗚鲸鐜? 1600x1200, 閫氶亾: 鍙? 鎶栧姩: 绂佺敤
	- 16: 鍒嗚鲸鐜? 1366x768, 閫氶亾: 鍗? 鎶栧姩: 绂佺敤
	- 17: 鍒嗚鲸鐜? 1024x600, 閫氶亾: 鍗? 鎶栧姩: 鍚敤
	- 18: 鍒嗚鲸鐜? 1280x768, 閫氶亾: 鍙? 鎶栧姩: 鍚敤
	- 19: 鍒嗚鲸鐜? 1280x800, 閫氶亾: 鍗? 鎶栧姩: 鍚敤

    viafb_accel:
	- 0 : 鏃?2D 纭欢鍔犻€?	- 1 : 2D 纭欢鍔犻€燂紙榛樿锛?
    viafb_SAMM_ON:
	- 0 : viafb_SAMM_ON 绂佺敤锛堥粯璁わ級
	- 1 : viafb_SAMM_ON 鍚敤

    viafb_mode1:锛堝壇鏄剧ず璁惧锛?	- 640x480锛堥粯璁わ級
	- 720x480
	- 800x600
	- 1024x768

    viafb_bpp1:锛堝壇鏄剧ず璁惧锛?	- 8, 16, 32锛堥粯璁?32锛?
    viafb_refresh1:锛堝壇鏄剧ず璁惧锛?	- 60, 75, 85, 100, 120锛堥粯璁?60锛?
    viafb_active_dev:
	姝ら€夐」鐢ㄤ簬鎸囧畾娲诲姩璁惧銆傦紙CRT銆丏VI銆丆RT+LCD鈥︹€︼級
	DVI 浠ｈ〃 DVI 鎴?HDMI锛屼緥濡傦紝鑻ユ兂鍚敤 HDMI锛?	璁剧疆 viafb_active_dev=DVI銆傚湪 SAMM 鎯呭喌涓嬶紝viafb_active_dev
	涔嬪墠鐨勬槸涓昏澶囷紝涔嬪悗鐨勬槸鍓澶囥€?
	渚嬪锛?
	瑕佸惎鐢ㄤ竴涓澶囷紝渚嬪浠?DVI锛屾垜浠彲浠ヤ娇鐢?:

	    modprobe viafb viafb_active_dev=DVI

	瑕佸惎鐢ㄤ袱涓澶囷紝渚嬪 CRT+DVI::

	    modprobe viafb viafb_active_dev=CRT+DVI;

	瀵逛簬 DuoView 鎯呭喌锛屾垜浠彲浠ヤ娇鐢?:

	    modprobe viafb viafb_active_dev=CRT+DVI

	鎴?:

	    modprobe viafb viafb_active_dev=DVI+CRT...

	瀵逛簬 SAMM 鎯呭喌锛?
	鑻?CRT 涓轰富銆丏VI 涓哄壇锛屾垜浠簲璇ヤ娇鐢?:

	    modprobe viafb viafb_active_dev=CRT+DVI viafb_SAMM_ON=1...

	鑻?DVI 涓轰富銆丆RT 涓哄壇锛屾垜浠簲璇ヤ娇鐢?:

	    modprobe viafb viafb_active_dev=DVI+CRT viafb_SAMM_ON=1...

    viafb_display_hardware_layout:
	姝ら€夐」鐢ㄤ簬鎸囧畾 CX700 鑺墖鐨勬樉绀虹‖浠跺竷灞€銆?
	- 1 : 浠?LCD
	- 2 : 浠?DVI
	- 3 : LCD+DVI锛堥粯璁わ級
	- 4 : LCD1+LCD2锛堝唴閮?+ 鍐呴儴锛?	- 16: LCD1+ExternalLCD2锛堝唴閮?+ 澶栭儴锛?
    viafb_second_size:
	姝ら€夐」鐢ㄤ簬璁剧疆 SAMM 鎯呭喌涓嬬浜岃澶囩殑鍐呭瓨澶у皬锛圡B锛夈€?	鏈€灏忓ぇ灏忎负 16銆?
    viafb_platform_epia_dvi:
	姝ら€夐」鐢ㄤ簬鍚敤 EPIA - M 涓婄殑 DVI

	- 0 : EPIA - M 涓婃棤 DVI锛堥粯璁わ級
	- 1 : EPIA - M 涓婃湁 DVI

    viafb_bus_width:
	褰撲娇鐢?24 浣嶆€荤嚎瀹藉害鐨勬暟瀛楁帴鍙ｆ椂锛?	搴旇缃閫夐」銆?
	- 12: 12 浣?LVDS 鎴?12 浣?TMDS锛堥粯璁わ級
	- 24: 24 浣?LVDS 鎴?24 浣?TMDS

    viafb_device_lcd_dualedge:
	褰撲娇鐢ㄥ弻杈圭紭闈㈡澘鏃讹紝搴旇缃閫夐」銆?
	- 0 : 鏃犲弻杈圭紭闈㈡澘锛堥粯璁わ級
	- 1 : 鍙岃竟缂橀潰鏉?
    viafb_lcd_port:
	姝ら€夐」鐢ㄤ簬鎸囧畾 LCD 杈撳嚭绔彛锛?	鍙敤鍊间负 "DVP0" "DVP1" "DFP_HIGHLOW" "DFP_HIGH" "DFP_LOW"銆?
	瀵逛簬 CX700 涓婄殑澶栭儴 LCD + 澶栭儴 DVI锛堝閮?LCD 鍦?DVP0 涓婏級锛?	鎴戜滑搴旇浣跨敤::

	    modprobe viafb viafb_lcd_port=DVP0...
```

璇存槑锛?    1. 瀵逛簬 DuoView CRT 涓?DVI 鏄剧ず锛屽湪鍚敤浜?DVI 杩囨壂鎻忕殑 鈥?40x480鈥?PAL 妯″紡涓嬶紝CRT 鍙兘鏄剧ず涓嶆甯搞€?    2. SAMM 浠ｈ〃鍗曢€傞厤鍣ㄥ鏄剧ず鍣紙single adapter multi monitors锛夈€傚畠涓庡鏄剧ず澶达紙multi-head锛変笉鍚岋紝鍥犱负 SAMM 鍦ㄩ┍鍔ㄥ眰鏀寔澶氭樉绀哄櫒锛屽洜姝?fbcon 灞傜敋鑷充笉鐭ラ亾瀹冪殑瀛樺湪锛汼AMM 鐨勭浜屼釜灞忓箷娌℃湁璁惧鑺傜偣鏂囦欢锛屽洜姝ょ敤鎴锋€佸簲鐢ㄧ▼搴忔棤娉曠洿鎺ヨ闂畠銆傚綋 SAMM 鍚敤鏃讹紝viafb_mode 涓?viafb_mode1銆乿iafb_bpp 涓?viafb_bpp1銆乿iafb_refresh 涓?viafb_refresh1 鍙互涓嶅悓銆?    3. 褰撴帶鍒跺彴渚濊禆浜?viafbinfo1 鏃讹紝鍔ㄦ€佹洿鏀瑰垎杈ㄧ巼鍜?bpp锛岄渶瑕佽皟鐢?VIAFB 鎸囧畾鐨?ioctl 鎺ュ彛 VIAFB_SET_DEVICE锛岃€屼笉鏄皟鐢ㄩ€氱敤鐨?ioctl 鍑芥暟 FBIOPUT_VSCREENINFO锛屽洜涓?viafb 瀵瑰鏄剧ず澶寸殑鏀寔涓嶅お濂斤紝鍚﹀垯浼氬鑷村睆骞曞穿婧冦€?
### 鐢?鈥渇bset鈥?宸ュ叿閰嶇疆 viafb


    鈥渇bset鈥?鏄?Linux 鐨勪竴涓唴缃疄鐢ㄥ伐鍏枫€?
```
	   # fbset -i

    2. 璁剧疆鍚勭鍒嗚鲸鐜囧拰 viafb_refresh 閫熺巼::

	   # fbset <resolution-vertical_sync>

       example::

	   # fbset "1024x768-75"

       or::

	   # fbset -g 1024 768 1024 768 32

       Check the file "/etc/fb.modes" to find display modes available.

    3. Set the color depth::

	   # fbset -depth <value>

       example::

	   # fbset -depth 16
```

### 閫氳繃 /proc 閰嶇疆 viafb


    浠ヤ笅鏂囦欢瀛樺湪浜?/proc/viafb 涓?
    supported_output_devices
	杩欎釜鍙鏂囦欢鍖呭惈涓€涓畬鏁寸殑銆佷互 鈥?鈥?鍒嗛殧鐨勫垪琛紝鍖呭惈浣犵殑骞冲彴涓婂彲鑳藉彲鐢ㄧ殑鎵€鏈夎緭鍑鸿澶囥€傚緢鍙兘骞堕潪鎵€鏈夎繖浜涜澶囧湪浣犵殑纭欢涓婇兘鏈夎繛鎺ュ櫒锛屼絾瀹冨簲鑳芥彁渚涜壇濂界殑璧风偣锛屼互寮勬竻杩欎簺鍚嶇О涓摢浜涘搴旂湡瀹炵殑杩炴帴鍣ㄣ€?
```
		# cat /proc/viafb/supported_output_devices

    iga1/output_devices, iga2/output_devices
	杩欎袱涓枃浠跺彲璇诲彲鍐欍€俰ga1 鍜?iga2 鏄骇鐢熷睆骞曞浘鍍忕殑涓や釜鐙珛鍗曞厓銆傝繖浜涘浘鍍忓彲浠ヨ杞彂鍒颁竴涓垨澶氫釜杈撳嚭璁惧銆傝鍙栬繖浜涙枃浠舵槸鏌ヨ鏌愪釜 iga 褰撳墠姝ｅ湪浣跨敤鍝簺杈撳嚭璁惧鐨勪竴绉嶆柟寮忋€?
	绀轰緥::

		# cat /proc/viafb/iga1/output_devices

	濡傛灉鏈墦鍗颁换浣曡緭鍑鸿澶囷紝鍒欒 iga 鐨勮緭鍑轰涪澶便€備緥濡傦紝濡傛灉鍙娇鐢ㄤ簡涓€涓紙鍙︿竴涓級iga锛屽氨鍙兘鍙戠敓杩欑鎯呭喌銆傚啓鍏ヨ繖浜涙枃浠跺厑璁稿湪杩愯鏃惰皟鏁磋緭鍑鸿澶囥€傚彲浠ユ坊鍔犳柊璁惧銆佺Щ闄ゅ凡鏈夎澶囷紝鎴栧湪 iga 涔嬮棿鍒囨崲銆傛湰璐ㄤ笂锛屼綘鍙互鍐欏叆涓€涓互 鈥?鈥?鍒嗛殧鐨勮澶囧悕鍒楄〃锛堟垨鍗曚釜璁惧鍚嶏級锛屾牸寮忎笌杩欎簺鏂囦欢鐨勮緭鍑虹浉鍚屻€備綘鍙互娣诲姞 鈥?鈥?鎴?鈥?鈥?浣滀负鍓嶇紑锛屼互渚跨畝鍗曞湴娣诲姞鍜岀Щ闄よ澶囥€傚洜姝ゅ墠缂€ 鈥?鈥?灏嗕綘鍒楄〃涓殑璁惧娣诲姞鍒板凡鏈夎澶囦箣涓婏紝鈥?鈥?浠庡凡鏈夎澶囦腑绉婚櫎鍒楀嚭鐨勮澶囷紝濡傛灉娌℃湁鍓嶇紑锛屽垯鐢ㄥ垪鍑虹殑璁惧鏇挎崲鎵€鏈夊凡鏈夎澶囥€傚鏋滀綘绉婚櫎璁惧锛屽畠浠簲褰撹鍏抽棴銆傚鏋滀綘娣诲姞鐨勮澶囧凡缁忔槸鍙︿竴涓?iga 鐨勪竴閮ㄥ垎锛屽垯浼氫粠閭ｉ噷绉婚櫎骞舵坊鍔犲埌鏂扮殑 iga 涓€?
	绀轰緥锛?
	灏?CRT 娣诲姞涓?iga1 鐨勮緭鍑鸿澶?:

		# echo +CRT > /proc/viafb/iga1/output_devices

	绉婚櫎锛堝叧闂級DVP1 鍜?LVDS1 浣滀负 iga2 鐨勮緭鍑鸿澶?:

		# echo -DVP1,LVDS1 > /proc/viafb/iga2/output_devices

	鐢?CRT 鏇挎崲 iga1 鐨勬墍鏈夎緭鍑鸿澶?:

		# echo CRT > /proc/viafb/iga1/output_devices
```

### 鐢?viafb 鍚姩


```
    append = "video=viafb:viafb_mode=1024x768,viafb_bpp=32,viafb_refresh=85"
```

## VIA 甯х紦鍐叉ā寮?

   :literal:
