## 甯х紦鍐茶澶?API

鏈€鍚庝慨璁細2011 骞?6 鏈?21 鏃?

### 0. 寮曡█

鏈枃妗ｆ弿杩颁簡搴旂敤鐢ㄦ潵涓庡抚缂撳啿璁惧浜や簰鐨勫抚缂撳啿 API銆傝澶囬┍鍔ㄤ笌甯х紦鍐叉牳蹇冧箣闂寸殑鍐呮牳鍐?API 涓嶅湪鎻忚堪鑼冨洿鍐呫€?
鐢变簬鍘熷甯х紦鍐?API 缂轰箯鏂囨。锛岄┍鍔ㄧ殑琛屼负鍦ㄧ粏寰紙浠ュ強涓嶉偅涔堢粏寰級鐨勬柟闈㈠瓨鍦ㄥ樊寮傘€傛湰鏂囨。鎻忚堪浜嗘帹鑽愮殑 API 瀹炵幇锛屼絾搴旂敤搴斿綋鍑嗗濂藉簲瀵逛笉鍚岀殑琛屼负銆?

### 1. 鑳藉姏锛圕apabilities锛?
璁惧鍜岄┍鍔ㄧ殑鑳藉姏鍦ㄥ浐瀹氱殑灞忓箷淇℃伅涓姤鍛?
```
  struct fb_fix_screeninfo {
	...
	__u16 capabilities;		/* see FB_CAP_*			*/
	...
  };

```
搴旂敤搴斿綋浣跨敤杩欎簺鑳藉姏鏉ユ煡鏄庡畠浠彲浠ヤ粠璁惧鍜岄┍鍔ㄦ湡寰呭摢浜涚壒鎬с€?
- FB_CAP_FOURCC

椹卞姩鏀寔鍩轰簬鍥涘瓧绗︾爜锛團OURCC锛夌殑鏍煎紡璁剧疆 API銆傚綋鏀寔鏃讹紝鏍煎紡浣跨敤 FOURCC 閰嶇疆锛岃€屼笉鏄墜鍔ㄦ寚瀹氶鑹插垎閲忕殑甯冨眬銆?

### 2. 绫诲瀷涓庤瑙夛紙Types and visuals锛?
鍍忕礌浠ヤ緷璧栦簬纭欢鐨勬牸寮忓瓨鍌ㄥ湪鍐呭瓨涓€傚簲鐢ㄩ渶瑕佷簡瑙ｅ儚绱犲瓨鍌ㄦ牸寮忥紝浠ヤ究浠ョ‖浠舵湡鏈涚殑鏍煎紡灏嗗浘鍍忔暟鎹啓鍏ュ抚缂撳啿鍐呭瓨銆?
鏍煎紡鐢卞抚缂撳啿绫诲瀷鍜岃瑙夛紙visual锛夋弿杩般€傛煇浜涜瑙夐渶瑕侀澶栫殑淇℃伅锛岃繖浜涗俊鎭瓨鍌ㄥ湪鍙彉鐨勫睆骞曚俊鎭?bits_per_pixel銆乬rayscale銆乺ed銆乬reen銆乥lue 鍜?transp 瀛楁涓€?
瑙嗚鎻忚堪棰滆壊淇℃伅濡備綍琚紪鐮佸苟缁勮浠ュ垱寤哄畯鍍忕礌锛坢acropixel锛夈€傜被鍨嬫弿杩板畯鍍忕礌濡備綍瀛樺偍鍦ㄥ唴瀛樹腑銆傛敮鎸佷互涓嬬被鍨嬪拰瑙嗚銆?
- FB_TYPE_PACKED_PIXELS

瀹忓儚绱犲湪鍗曚竴骞抽潰涓繛缁瓨鍌ㄣ€傚鏋滄瘡涓畯鍍忕礌鐨勪綅鏁颁笉鏄?8 鐨勫€嶆暟锛岄偅涔堝畯鍍忕礌鏄～鍏呭埌涓嬩竴涓?8 浣嶇殑鍊嶆暟杩樻槸鎵撳寘杩涘瓧鑺傦紝鍙栧喅浜庤瑙夈€?
琛屾湯鍙兘瀛樺湪濉厖锛屽苟閫氳繃鍥哄畾灞忓箷淇℃伅鐨?line_length 瀛楁鎶ュ憡銆?
- FB_TYPE_PLANES

瀹忓儚绱犺鎷嗗垎鍒板涓钩闈腑銆傚钩闈㈡暟绛変簬姣忎釜瀹忓儚绱犵殑浣嶆暟锛岀 i 涓钩闈㈠瓨鍌ㄦ墍鏈夊畯鍍忕礌鐨勭 i 浣嶃€?
骞抽潰鍦ㄥ唴瀛樹腑杩炵画瀛樻斁銆?
- FB_TYPE_INTERLEAVED_PLANES

瀹忓儚绱犺鎷嗗垎鍒板涓钩闈腑銆傚钩闈㈡暟绛変簬姣忎釜瀹忓儚绱犵殑浣嶆暟锛岀 i 涓钩闈㈠瓨鍌ㄦ墍鏈夊畯鍍忕礌鐨勭 i 浣嶃€?
骞抽潰鍦ㄥ唴瀛樹腑浜ら敊瀛樻斁銆備氦閿欏洜瀛愶紙瀹氫箟涓哄睘浜庝笉鍚屽钩闈㈢殑涓や釜杩炵画浜ら敊鍧楄捣鐐逛箣闂寸殑瀛楄妭璺濈锛夊瓨鍌ㄥ湪鍥哄畾灞忓箷淇℃伅鐨?type_aux 瀛楁涓€?
- FB_TYPE_FOURCC

瀹忓儚绱犳寜鐓у瓨鍌ㄥ湪鍙彉灞忓箷淇℃伅 grayscale 瀛楁涓殑鏍煎紡 FOURCC 鏍囪瘑绗︽墍鎻忚堪鐨勯偅鏍峰瓨鍌ㄥ湪鍐呭瓨涓€?
- FB_VISUAL_MONO01

鍍忕礌涓洪粦鎴栫櫧锛屽苟瀛樺偍鍦ㄧ敱鍙彉灞忓箷淇℃伅 bpp 瀛楁鎸囧畾鐨勮嫢骞蹭綅锛堥€氬父鏄竴浣嶏級涓娿€?
榛戝儚绱犵敱鎵€鏈変綅璁句负 1 琛ㄧず锛岀櫧鍍忕礌鐢辨墍鏈変綅璁句负 0 琛ㄧず銆傚綋姣忓儚绱犱綅鏁板皬浜?8 鏃讹紝澶氫釜鍍忕礌琚墦鍖呰繘涓€涓瓧鑺傘€?
FB_VISUAL_MONO01 鐩墠浠呬笌 FB_TYPE_PACKED_PIXELS 涓€璧蜂娇鐢ㄣ€?
- FB_VISUAL_MONO10

鍍忕礌涓洪粦鎴栫櫧锛屽苟瀛樺偍鍦ㄧ敱鍙彉灞忓箷淇℃伅 bpp 瀛楁鎸囧畾鐨勮嫢骞蹭綅锛堥€氬父鏄竴浣嶏級涓娿€?
榛戝儚绱犵敱鎵€鏈変綅璁句负 0 琛ㄧず锛岀櫧鍍忕礌鐢辨墍鏈変綅璁句负 1 琛ㄧず銆傚綋姣忓儚绱犱綅鏁板皬浜?8 鏃讹紝澶氫釜鍍忕礌琚墦鍖呰繘涓€涓瓧鑺傘€?
FB_VISUAL_MONO10 鐩墠浠呬笌 FB_TYPE_PACKED_PIXELS 涓€璧蜂娇鐢ㄣ€?
- FB_VISUAL_TRUECOLOR

鍍忕礌琚垎瑙ｄ负绾€佺豢銆佽摑鍒嗛噺锛屾瘡涓垎閲忕储寮曚竴涓彧璇绘煡鎵捐〃浠ヨ幏寰楀搴旂殑鍊笺€傛煡鎵捐〃渚濊禆浜庤澶囷紝骞舵彁渚涚嚎鎬ф垨闈炵嚎鎬ф枩鍧°€?
姣忎釜鍒嗛噺鏍规嵁鍙彉灞忓箷淇℃伅鐨?red銆乬reen銆乥lue 鍜?transp 瀛楁瀛樺偍鍦ㄤ竴涓畯鍍忕礌涓€?
- FB_VISUAL_PSEUDOCOLOR 涓?FB_VISUAL_STATIC_PSEUDOCOLOR

鍍忕礌鍊艰缂栫爜涓虹储寮曪紝瀛樺叆瀛樺偍绾€佺豢銆佽摑鍒嗛噺鐨勯鑹叉槧灏勮〃锛坈olormap锛夈€傚浜?FB_VISUAL_STATIC_PSEUDOCOLOR 棰滆壊鏄犲皠琛ㄦ槸鍙鐨勶紝瀵逛簬 FB_VISUAL_PSEUDOCOLOR 鏄彲璇诲啓鐨勩€?
姣忎釜鍍忕礌鍊煎瓨鍌ㄥ湪鐢卞彲鍙樺睆骞曚俊鎭?bits_per_pixel 瀛楁鎶ュ憡鐨勪綅鏁颁腑銆?
- FB_VISUAL_DIRECTCOLOR

鍍忕礌琚垎瑙ｄ负绾€佺豢銆佽摑鍒嗛噺锛屾瘡涓垎閲忕储寮曚竴涓彲缂栫▼鐨勬煡鎵捐〃浠ヨ幏寰楀搴旂殑鍊笺€?
姣忎釜鍒嗛噺鏍规嵁鍙彉灞忓箷淇℃伅鐨?red銆乬reen銆乥lue 鍜?transp 瀛楁瀛樺偍鍦ㄤ竴涓畯鍍忕礌涓€?
- FB_VISUAL_FOURCC

鍍忕礌鎸夌収瀛樺偍鍦ㄥ彲鍙樺睆骞曚俊鎭?grayscale 瀛楁涓殑鏍煎紡 FOURCC 鏍囪瘑绗︽墍鎻忚堪鐨勯偅鏍疯繘琛岀紪鐮佸拰瑙ｉ噴銆?

### 3. 灞忓箷淇℃伅

灞忓箷淇℃伅鐢卞簲鐢ㄤ娇鐢?FBIOGET_FSCREENINFO 鍜?FBIOGET_VSCREENINFO ioctl 鏌ヨ銆傝繖浜?ioctl 鍒嗗埆鎺ュ彈涓€涓寚鍚?fb_fix_screeninfo 鍜?fb_var_screeninfo 缁撴瀯鐨勬寚閽堛€?
struct fb_fix_screeninfo 瀛樺偍鍏充簬甯х紦鍐茶澶囧強鍏跺綋鍓嶆牸寮忕殑銆佷笌璁惧鏃犲叧涓斾笉鍙洿鏀圭殑淇℃伅銆傝繖浜涗俊鎭笉鑳借搴旂敤鐩存帴淇敼锛屼絾鍙互鍦ㄩ┍鍔ㄦ墽琛?
```
  struct fb_fix_screeninfo {
	char id[16];			/* identification string eg "TT Builtin" */
	unsigned long smem_start;	/* Start of frame buffer mem */
					/* (physical address) */
	__u32 smem_len;			/* Length of frame buffer mem */
	__u32 type;			/* see FB_TYPE_*		*/
	__u32 type_aux;			/* Interleave for interleaved Planes */
	__u32 visual;			/* see FB_VISUAL_*		*/
	__u16 xpanstep;			/* zero if no hardware panning  */
	__u16 ypanstep;			/* zero if no hardware panning  */
	__u16 ywrapstep;		/* zero if no hardware ywrap    */
	__u32 line_length;		/* length of a line in bytes    */
	unsigned long mmio_start;	/* Start of Memory Mapped I/O   */
					/* (physical address) */
	__u32 mmio_len;			/* Length of Memory Mapped I/O  */
	__u32 accel;			/* Indicate to driver which	*/
					/*  specific chip/card we have	*/
	__u16 capabilities;		/* see FB_CAP_*			*/
	__u16 reserved[2];		/* Reserved for future compatibility */
  };

```
struct fb_var_screeninfo 瀛樺偍鍏充簬甯х紦鍐茶澶囥€佸叾褰撳墠鏍煎紡鍜岃棰戞ā寮忎互鍙婁笌璁惧鏃犲叧涓斿彲鏇存敼鐨勪俊鎭紝浠ュ強

```
  struct fb_var_screeninfo {
	__u32 xres;			/* visible resolution		*/
	__u32 yres;
	__u32 xres_virtual;		/* virtual resolution		*/
	__u32 yres_virtual;
	__u32 xoffset;			/* offset from virtual to visible */
	__u32 yoffset;			/* resolution			*/

	__u32 bits_per_pixel;		/* guess what			*/
	__u32 grayscale;		/* 0 = color, 1 = grayscale,	*/
					/* >1 = FOURCC			*/
	struct fb_bitfield red;		/* bitfield in fb mem if true color, */
	struct fb_bitfield green;	/* else only length is significant */
	struct fb_bitfield blue;
	struct fb_bitfield transp;	/* transparency			*/

	__u32 nonstd;			/* != 0 Non standard pixel format */

	__u32 activate;			/* see FB_ACTIVATE_*		*/

	__u32 height;			/* height of picture in mm    */
	__u32 width;			/* width of picture in mm     */

	__u32 accel_flags;		/* (OBSOLETE) see fb_info.flags */

	/* Timing: All values in pixclocks, except pixclock (of course) */
	__u32 pixclock;			/* pixel clock in ps (pico seconds) */
	__u32 left_margin;		/* time from sync to picture	*/
	__u32 right_margin;		/* time from picture to sync	*/
	__u32 upper_margin;		/* time from sync to picture	*/
	__u32 lower_margin;
	__u32 hsync_len;		/* length of horizontal sync	*/
	__u32 vsync_len;		/* length of vertical sync	*/
	__u32 sync;			/* see FB_SYNC_*		*/
	__u32 vmode;			/* see FB_VMODE_*		*/
	__u32 rotate;			/* angle we rotate counter clockwise */
	__u32 colorspace;		/* colorspace for FOURCC-based modes */
	__u32 reserved[4];		/* Reserved for future compatibility */
  };

```
瑕佷慨鏀瑰彲鍙樹俊鎭紝搴旂敤璋冪敤 FBIOPUT_VSCREENINFO ioctl锛屽苟浼犲叆涓€涓寚鍚?fb_var_screeninfo 缁撴瀯鐨勬寚閽堛€傚鏋滆皟鐢ㄦ垚鍔燂紝椹卞姩灏嗙浉搴斿湴鏇存柊鍥哄畾灞忓箷淇℃伅銆?
搴旂敤涓嶅簲鎵嬪姩濉厖鏁翠釜 fb_var_screeninfo 缁撴瀯锛岃€屽簲璋冪敤 FBIOGET_VSCREENINFO ioctl 骞朵粎淇敼瀹冧滑鍏冲績鐨勫瓧娈点€?

### 4. 鏍煎紡閰嶇疆

甯х紦鍐茶澶囨彁渚涗袱绉嶆柟寮忔潵閰嶇疆甯х紦鍐叉牸寮忥細浼犵粺 API 鍜屽熀浜?FOURCC 鐨?API銆?

浼犵粺 API 闀挎湡浠ユ潵涓€鐩存槸鍞竴鐨勫抚缂撳啿鏍煎紡閰嶇疆 API锛屽洜姝よ搴旂敤骞挎硾浣跨敤銆傚浜?RGB 鍜岀伆搴︽牸寮忎互鍙婁紶缁熺殑闈炴爣鍑嗘牸寮忥紝瀹冩槸鎺ㄨ崘缁欏簲鐢ㄤ娇鐢ㄧ殑 API銆?
瑕侀€夋嫨涓€绉嶆牸寮忥紝搴旂敤灏?fb_var_screeninfo 鐨?bits_per_pixel 瀛楁璁句负鎵€闇€鐨勫抚缂撳啿娣卞害銆傛渶澶т负 8 鐨勫€奸€氬父浼氭槧灏勫埌鍗曡壊銆佺伆搴︽垨浼僵鑹茶瑙夛紝浣嗚繖骞朵笉寮哄埗瑕佹眰銆?
- 瀵逛簬鐏板害鏍煎紡锛屽簲鐢ㄥ皢 grayscale 瀛楁璁句负 1銆俽ed銆乥lue銆乬reen 鍜?transp 瀛楁蹇呴』鐢卞簲鐢ㄨ涓?0锛屽苟琚┍鍔ㄥ拷鐣ャ€傞┍鍔ㄥ繀椤诲皢 red銆乥lue 鍜?green 鐨勫亸绉诲～涓?0锛岄暱搴﹀～涓?bits_per_pixel 鐨勫€笺€?
- 瀵逛簬浼僵鑹叉牸寮忥紝搴旂敤灏?grayscale 瀛楁璁句负 0銆俽ed銆乥lue銆乬reen 鍜?transp 瀛楁蹇呴』鐢卞簲鐢ㄨ涓?0锛屽苟琚┍鍔ㄥ拷鐣ャ€傞┍鍔ㄥ繀椤诲皢 red銆乥lue 鍜?green 鐨勫亸绉诲～涓?0锛岄暱搴﹀～涓?bits_per_pixel 鐨勫€笺€?
- 瀵逛簬鐪熷僵鑹诧紙truecolor锛夊拰鐩存帴褰╄壊锛坉irectcolor锛夋牸寮忥紝搴旂敤灏?grayscale 瀛楁璁句负 0锛屽苟灏?red銆乥lue銆乬reen 鍜?transp 瀛楁璁句负鎻忚堪

```
    struct fb_bitfield {
	__u32 offset;			/* beginning of bitfield	*/
	__u32 length;			/* length of bitfield		*/
	__u32 msb_right;		/* != 0 : Most significant bit is */
					/* right */
    };

  鍍忕礌鍊间负 bits_per_pixel 瀹斤紝骞惰鎷嗗垎涓轰笉閲嶅彔鐨勭孩銆佺豢銆佽摑鍜?alpha锛堥€忔槑搴︼級鍒嗛噺銆傛瘡涓垎閲忓湪鍍忕礌鍊间腑鐨勪綅缃拰澶у皬鐢?fb_bitfield 鐨?offset 鍜?length 瀛楁鎻忚堪銆傚亸绉讳粠鍙充晶璁＄畻銆?
  鍍忕礌鎬绘槸瀛樺偍鍦ㄦ暣鏁颁釜瀛楄妭涓€傚鏋滄瘡鍍忕礌浣嶆暟涓嶆槸 8 鐨勫€嶆暟锛屽儚绱犲€艰濉厖鍒颁笅涓€涓?8 浣嶇殑鍊嶆暟銆?
```
鏍煎紡閰嶇疆鎴愬姛鍚庯紝椹卞姩鏍规嵁鎵€閫夋牸寮忔洿鏂?fb_fix_screeninfo 鐨?type銆乿isual 鍜?line_length 瀛楁銆?

鍩轰簬 FOURCC 鐨?API 鐢ㄥ洓瀛楃鐮侊紙FOURCC锛夋浛浠ｆ牸寮忔弿杩般€侳OURCC 鏄娊璞℃爣璇嗙锛屽湪涓嶆樉寮忔弿杩版牸寮忕殑鎯呭喌涓嬪敮涓€鍦板畾涔変竴涓牸寮忋€傝繖鏄敮涓€鏀寔 YUV 鏍煎紡鐨?API銆備篃榧撳姳椹卞姩涓?RGB 鍜岀伆搴︽牸寮忓疄鐜板熀浜?FOURCC 鐨?API銆?
鏀寔鍩轰簬 FOURCC 鐨?API 鐨勯┍鍔ㄩ€氳繃鍦?fb_fix_screeninfo 鐨?capabilities 瀛楁涓缃?FB_CAP_FOURCC 浣嶆潵鎶ュ憡姝よ兘鍔涖€?
FOURCC 瀹氫箟浣嶄簬 linux/videodev2.h 澶存枃浠朵腑銆傜劧鑰岋紝灏界浠?V4L2_PIX_FMT_ 鍓嶇紑寮€澶达紝瀹冧滑骞朵笉灞€闄愪簬 V4L2锛屼篃涓嶈姹備娇鐢?V4L2 瀛愮郴缁熴€侳OURCC 鏂囨。鍙湪 Documentation/userspace-api/media/v4l/pixfmt.rst 涓幏鍙栥€?
瑕侀€夋嫨涓€绉嶆牸寮忥紝搴旂敤灏?grayscale 瀛楁璁句负鎵€闇€鐨?FOURCC銆傚浜?YUV 鏍煎紡锛屽畠浠繕搴旈€氳繃灏?colorspace 瀛楁璁句负 linux/videodev2.h 涓垪鍑哄苟鍦?Documentation/userspace-api/media/v4l/colorspaces.rst 涓褰曠殑鏌愪釜鑹插僵绌洪棿鏉ラ€夋嫨閫傚綋鐨?colorspace銆?
鍩轰簬 FOURCC 鐨?API 涓嶄娇鐢?red銆乬reen銆乥lue 鍜?transp 瀛楁銆傚嚭浜庡悜鍓嶅吋瀹圭殑鍘熷洜锛屽簲鐢ㄥ繀椤诲皢閭ｄ簺瀛楁娓呴浂锛岄┍鍔ㄥ繀椤诲拷鐣ュ畠浠€傞櫎 0 浠ュ鐨勫€煎彲鑳藉湪鏈潵鐨勬墿灞曚腑鑾峰緱鍚箟銆?
鏍煎紡閰嶇疆鎴愬姛鍚庯紝椹卞姩鏍规嵁鎵€閫夋牸寮忔洿鏂?fb_fix_screeninfo 鐨?type銆乿isual 鍜?line_length 瀛楁銆倀ype 鍜?visual 瀛楁鍒嗗埆璁句负 FB_TYPE_FOURCC 鍜?FB_VISUAL_FOURCC銆?