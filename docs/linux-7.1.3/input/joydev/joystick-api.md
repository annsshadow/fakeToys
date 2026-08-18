
## Programming Interface


:浣滆€? Ragnar Hojland Espinosa <ragnar@macula.net> - 1998 骞?8 鏈?7 鏃?

## Introduction


   鏈枃妗ｆ弿杩版棫鐨?`js` 鎺ュ彛銆傚缓璁柊瀹㈡埛绔垏鎹㈠埌閫氱敤鐨勪簨浠讹紙`evdev`锛夋帴鍙ｃ€?

1.0 鐗堥┍鍔ㄩ噰鐢ㄤ竴绉嶅叏鏂扮殑銆佸熀浜庝簨浠剁殑鎽囨潌椹卞姩鏂瑰紡銆傜敤鎴风▼搴忎笉鍐嶄富鍔ㄨ疆璇㈡憞鏉?
鏁板€硷紝鑰屾槸鐢辨憞鏉嗛┍鍔ㄤ粎鍦ㄧ姸鎬佸彂鐢熷彉鍖栨椂涓婃姤銆傛洿澶氫俊鎭鍙傞槄鎽囨潌杞欢鍖呬腑闄勫甫鐨?
joystick-api.txt銆乯oystick.h 浠ュ強 jstest.c銆傛憞鏉嗚澶囧彲鍦ㄩ樆濉炴垨闈為樆濉炴ā寮忎笅浣跨敤锛?
骞舵敮鎸?select() 璋冪敤銆?

涓轰繚鎸佸悜鍚庡吋瀹癸紝鏃х殑锛坴0.x锛夋帴鍙ｄ緷鐒朵繚鐣欍€備换浣曚娇鐢ㄦ棫鎺ュ彛瀵规憞鏉嗛┍鍔ㄧ殑璋冪敤閮戒細
杩斿洖涓庢棫鎺ュ彛鍏煎鐨勬暟鍊笺€傝鎺ュ彛浠嶅眬闄愪簬 2 涓酱锛屼笖浣跨敤瀹冪殑搴旂敤绋嬪簭閫氬父鍙В鐮?
2 涓寜閿紝灏界椹卞姩鏈€澶氬彲鎻愪緵 32 涓€?

## Initialization


鎸夊父瑙勮涔夛紙鍗充娇鐢?open锛夋墦寮€鎽囨潌璁惧銆傜敱浜庨┍鍔ㄧ幇鍦ㄤ笂鎶ヤ簨浠惰€岄潪杞鍙樺寲锛屽湪
open 涔嬪悗瀹冧細绔嬪嵆鍙戝嚭涓€绯诲垪鍚堟垚浜嬩欢锛圝S_EVENT_INIT锛夛紝浣犲彲浠ラ€氳繃璇诲彇瀹冧滑鏉ヨ幏寰?
鎽囨潌鐨勫垵濮嬬姸鎬併€?

```

	int fd = open ("/dev/input/js0", O_RDONLY);


```
## Event Reading


```

	struct js_event e;
	read (fd, &e, sizeof(e));

```
```

	struct js_event {
		__u32 time;     /* event timestamp in milliseconds */
		__s16 value;    /* value */
		__u8 type;      /* event type */
		__u8 number;    /* axis/button number */
	};

```
濡傛灉璇诲彇鎴愬姛锛岄櫎闈炰綘甯屾湜鍍?3.1 鑺傛墍杩伴偅鏍峰湪涓€娆?read 涓鍙栧涓簨浠讹紝鍚﹀垯瀹冨皢
杩斿洖 sizeof(e)銆?


### js_event.type


```

	#define JS_EVENT_BUTTON         0x01    /* button pressed/released */
	#define JS_EVENT_AXIS           0x02    /* joystick moved */
	#define JS_EVENT_INIT           0x80    /* initial state of device */

```
濡備笂鎵€杩帮紝椹卞姩鍦?open 鏃朵細鍙戝嚭甯︽湁 JS_EVENT_INIT 鏍囧織锛圤Red锛夌殑鍚堟垚浜嬩欢銆備篃灏辨槸璇达紝
褰撳畠鍙戝嚭涓€涓?INIT BUTTON 浜嬩欢鏃讹紝
```

	int type = JS_EVENT_BUTTON | JS_EVENT_INIT;	/* 0x81 */

```
濡傛灉浣犻€夋嫨涓嶅尯鍒嗗悎鎴愪簨浠朵笌鐪熷疄浜嬩欢
```

	type &= ~JS_EVENT_INIT;				/* 0x01 */


```
### js_event.number


`number` 鐨勫€煎搴斾簬浜х敓璇ヤ簨浠剁殑杞存垨鎸夐敭銆傛敞鎰忓畠浠娇鐢ㄥ悇鑷嫭绔嬬殑缂栧彿锛堝嵆浣犲悓鏃?
鎷ユ湁杞?0 鍜屾寜閿?0锛夈€傞€氬父锛?

        =============== =======
	杞?	缂栧彿
        =============== =======
	绗竴杞?X	0
	绗竴杞?Y	1
	绗簩杞?X	2
	绗簩杞?Y	3
	鈥︹€︿互姝ょ被鎺?
        =============== =======

鏂瑰悜甯斤紙hat锛夊洜鎽囨潌绫诲瀷鑰屽紓銆傛湁浜涘彲鏈?8 涓柟鍚戠Щ鍔紝鏈変簺鍙兘鏈?4 涓柟鍚戙€備絾鏃犺
纭欢鏄惁鍏佽鐙珛绉诲姩锛岄┍鍔ㄥ缁堝皢鏂瑰悜甯戒笂鎶ヤ负涓や釜鐙珛鐨勮酱銆?


### js_event.value


瀵逛簬涓€涓酱锛宍value` 鏄粙浜?-32767 涓?+32767 涔嬮棿鐨勬湁绗﹀彿鏁存暟锛岃〃绀鸿鎽囨潌娌胯杞寸殑
浣嶇疆銆傚鏋滀綘鍦ㄦ憞鏉嗗浜?`dead`锛堟鍖猴級鏃舵病鏈夎鍒?0锛屾垨鑰呭畠涓嶈兘瑕嗙洊瀹屾暣鑼冨洿锛屽垯搴?
閲嶆柊鏍″噯锛堜緥濡備娇鐢?jscal锛夈€?

瀵逛簬涓€涓寜閿紝鎸変笅浜嬩欢鐨?`value` 涓?1锛岄噴鏀句簨浠剁殑 `value` 涓?0銆?

```

	if (js_event.type == JS_EVENT_BUTTON) {
		buttons_state ^= (1 << js_event.number);
	}

```
濡傛灉浣犲崟鐙鐞?JS_EVENT_INIT 浜嬩欢锛屽彲鑳戒細宸ヤ綔寰楀緢濂斤紝

```

	if ((js_event.type & ~JS_EVENT_INIT) == JS_EVENT_BUTTON) {
		if (js_event.value)
			buttons_state |= (1 << js_event.number);
		else
			buttons_state &= ~(1 << js_event.number);
	}

```
杩欐牱鍋氳瀹夊叏寰楀锛屽洜涓哄畠涓嶄細涓庨┍鍔ㄥけ鍘诲悓姝ャ€傜敱浜庝綘涓嶅緱涓嶅湪绗竴娈典唬鐮佷腑涓?
JS_EVENT_INIT 浜嬩欢缂栧啓鍗曠嫭鐨勫鐞嗗嚱鏁帮紝杩欑鏂瑰紡鏈€缁堝弽鑰屾洿绠€鐭€?


### js_event.time


浜嬩欢浜х敓鐨勬椂闂翠繚瀛樺湪 `js_event.time` 涓€傚畠鏄嚜鈥︹€︽煇涓繃鍘绘椂鍒讳互鏉ョ殑姣鏁般€傝繖
鏂逛究浜嗘娴嬪弻鍑汇€佸垽鏂酱鐨勭Щ鍔ㄤ笌鎸夐敭鎸変笅鏄惁鍚屾椂鍙戠敓绛夌被浼间换鍔°€?


## Reading


濡傛灉浣犱互闃诲妯″紡鎵撳紑璁惧锛屼竴娆?read 灏嗕竴鐩撮樆濉烇紙鍗崇瓑寰咃級锛岀洿鍒版湁浜嬩欢浜х敓骞惰
瀹為檯璇诲彇銆傚鏋滀綘涓嶈兘鏃犻檺绛夊緟锛堣瘹鐒讹紝閭ｆ槸寰堥暱鐨勪竴娈垫椂闂达紱锛夛紝鏈変袱绉嶆浛浠ｆ柟妗?

	a) 浣跨敤 select 绛夊緟 fd 涓婃湁鏁版嵁鍙锛屾垨鐩村埌瓒呮椂銆俿elect(2) 鐨勬墜鍐岄〉涓婃湁涓€涓?
	   寰堝ソ鐨勭ず渚嬨€?

	b) 浠ラ潪闃诲妯″紡锛圤_NONBLOCK锛夋墦寮€璁惧


### O_NONBLOCK


濡傛灉鍦?O_NONBLOCK 妯″紡涓?read 杩斿洖 -1锛岃繖涓嶄竴瀹氭槸鈥滅湡瀹炩€濋敊璇紙璇锋鏌?errno(3)锛夛紱
瀹冨彲鑳藉彧鏄〃绀洪┍鍔ㄩ槦鍒椾腑灏氭棤鍙鍙栫殑浜嬩欢銆備綘搴斿綋璇诲彇闃熷垪涓殑鎵€鏈変簨浠讹紙鍗充竴鐩?
璇诲埌杩斿洖 -1 涓烘锛夈€?

渚嬪锛?

```

	while (1) {
		while (read (fd, &e, sizeof(e)) > 0) {
			process_event (e);
		}
		/* EAGAIN is returned when the queue is empty */
		if (errno != EAGAIN) {
			/* error */
		}
		/* do something interesting with processed events */
	}

```
娓呯┖闃熷垪鐨勪竴涓師鍥犳槸锛屽鏋滈槦鍒楀彉婊★紝鐢变簬闃熷垪瀹归噺鏈夐檺锛屼綘灏嗗紑濮嬩涪澶变簨浠讹紝杈冩棫鐨?
浜嬩欢浼氳瑕嗙洊銆?

鍙︿竴涓師鍥犳槸浣犳兂鐭ラ亾鍙戠敓鐨勬墍鏈変簨鎯咃紝鑰屼笉鏄妸澶勭悊鎺ㄨ繜鍒颁互鍚庛€?

闃熷垪涓轰綍浼氬彉婊★紵鍥犱负浣犳病鏈夊涓婃墍杩版竻绌洪槦鍒楋紝鎴栬€呭洜涓轰袱娆¤鍙栦箣闂撮棿闅旇繃闀匡紝浠庤€?
鍦ㄩ槦鍒椾腑浜х敓浜嗚繃澶氫簨浠躲€傛敞鎰忛珮绯荤粺璐熻浇鍙兘浼氳繘涓€姝ユ媺澶ц繖浜涜鍙栫殑闂撮殧銆?

濡傛灉璇诲彇涔嬮棿鐨勬椂闂磋冻浠ュ～婊￠槦鍒楀苟涓㈠け浜嬩欢锛岄┍鍔ㄥ皢鍒囨崲鍒板惎鍔ㄦā寮忥紝涓嬫浣犺鍙栨椂
浼氱敓鎴愬悎鎴愪簨浠讹紙JS_EVENT_INIT锛夋潵鍛婄煡浣犳憞鏉嗙殑瀹為檯鐘舵€併€?



 鑷?1.2.8 鐗堟湰璧凤紝闃熷垪涓虹幆褰紝鍙绾?64 涓簨浠躲€備綘鍙互閫氳繃璋冨ぇ joystick.h 涓殑
 JS_BUFF_SIZE 骞堕噸鏂扮紪璇戦┍鍔ㄦ潵澧炲姞姝ゅぇ灏忋€?


鍦ㄤ笂闈㈢殑浠ｇ爜涓紝浣犲彲鑳借繕鎯冲埄鐢ㄥ吀鍨嬬殑 read(2) 鍔熻兘涓€娆℃€ц鍙栧涓簨浠躲€備负姝わ紝浣?
```

	struct js_event mybuffer[0xff];
	int i = read (fd, mybuffer, sizeof(mybuffer));

```
杩欑鎯呭喌涓嬶紝濡傛灉闃熷垪涓虹┖锛宺ead 灏嗚繑鍥?-1锛屾垨鑰呰繑鍥炲彟涓€涓€硷紝鍏朵腑璇诲彇鍒扮殑浜嬩欢
鏁伴噺涓?i / sizeof(js_event)銆傚悓鏍凤紝濡傛灉缂撳啿鍖哄凡婊★紝鏈€濂藉鐞嗚繖浜涗簨浠跺苟缁х画璇诲彇锛?
鐩村埌娓呯┖椹卞姩闃熷垪銆?


## IOCTLs


```

				/* function			3rd arg  */
	#define JSIOCGAXES	/* get number of axes		char	 */
	#define JSIOCGBUTTONS	/* get number of buttons	char	 */
	#define JSIOCGVERSION	/* get driver version		int	 */
	#define JSIOCGNAME(len) /* get identifier string	char	 */
	#define JSIOCSCORR	/* set correction values	&js_corr */
	#define JSIOCGCORR	/* get correction values	&js_corr */

```
```

	char number_of_axes;
	ioctl (fd, JSIOCGAXES, &number_of_axes);


```
### JSIOGCVERSION


JSIOGCVERSION 鏄湪杩愯鏃舵鏌ユ鍦ㄨ繍琛岀殑椹卞姩鏄惁涓?1.0+ 骞舵敮鎸佷簨浠舵帴鍙ｇ殑濂芥柟娉曘€?
濡傛灉涓嶆槸锛岃 IOCTL 灏嗗け璐ャ€傚浜庣紪璇戞湡鍐冲畾锛屼綘鍙互娴嬭瘯
```

	#ifdef JS_VERSION
	#if JS_VERSION > 0xsomething


```
### JSIOCGNAME


JSIOCGNAME(len) 鍏佽浣犺幏鍙栨憞鏉嗙殑鍚嶇О瀛楃涓测€斺€斾笌鍚姩鏃舵墦鍗扮殑鐩稿悓銆?len' 鍙傛暟鏄?
璇锋眰鍚嶇О鐨勫簲鐢ㄧ▼搴忔墍鎻愪緵鐨勭紦鍐插尯闀垮害锛岀敤浜庨伩鍏?
```

	char name[128];
	if (ioctl(fd, JSIOCGNAME(sizeof(name)), name) < 0)
		strscpy(name, "Unknown", sizeof(name));
	printf("Name: %s\n", name);


```
### JSIOC[SG]CORR


鍏充簬 JSIOC[SG]CORR 鐨勭敤娉曪紝寤鸿鍙傝€?jscal.c銆傛甯哥▼搴忎腑骞朵笉闇€瑕佸畠浠紝浠呭湪鏍″噯
杞欢锛堝 jscal 鎴?kcmjoy锛変腑鎵嶉渶瑕併€傝繖浜?IOCTL 鍙婃暟鎹被鍨嬩笉琚涓?API 鐨勭ǔ瀹氶儴鍒嗭紝
鍥犳鍙兘鍦ㄩ┍鍔ㄥ悗缁増鏈腑涓嶇粡璀﹀憡鍦板彂鐢熷彉鍖栥€?

JSIOCSCORR 鍜?JSIOCGCORR 閮芥湡鏈?&js_corr 鑳藉瀹圭撼鎵€鏈夎酱鐨勪俊鎭€傚嵆 struct
js_corr corr[MAX_AXIS];

```

	struct js_corr {
		__s32 coef[8];
		__u16 prec;
		__u16 type;
	};

```
```

	#define JS_CORR_NONE            0x00    /* returns raw values */
	#define JS_CORR_BROKEN          0x01    /* broken line */


```
## Backward compatibility


0.x 鐗堟憞鏉嗛┍鍔?API 鐩稿綋鍙楅檺锛屽叾鐢ㄦ硶宸茶搴熷純銆?
```

	struct JS_DATA_TYPE js;
	while (1) {
		if (read (fd, &js, JS_RETURN) != JS_RETURN) {
			/* error */
		}
		usleep (1000);
	}

```
濡傜ず渚嬫墍绀猴紝read 浼氱珛鍗宠繑鍥烇紝
```

	struct JS_DATA_TYPE {
		int buttons;    /* immediate button state */
		int x;          /* immediate x axis value */
		int y;          /* immediate y axis value */
	};

```
```

	#define JS_RETURN       sizeof(struct JS_DATA_TYPE)

```
瑕佹祴璇曟寜閿殑鐘舵€侊紝

```

	first_button_state  = js.buttons & 1;
	second_button_state = js.buttons & 2;

```
鍦ㄥ師濮嬬殑 0.x 椹卞姩涓紝杞村€兼病鏈夊畾涔夌殑鑼冨洿锛屽彧瑕佹眰鍊间负闈炶礋銆?.2.8+ 鐗堥┍鍔ㄤ娇鐢ㄥ浐瀹?
鑼冨洿涓婃姤鏁板€硷紝1 涓烘渶灏忓€硷紝128 涓哄眳涓紝255 涓烘渶澶у€笺€?

v0.8.0.2 鐗堥┍鍔ㄨ繕鎻愪緵浜嗕竴涓€滄暟瀛楁憞鏉嗏€濓紙鍦ㄦ湰椹卞姩涓幇绉?Multisystem 鎽囨潌锛夋帴鍙ｏ紝
浣嶄簬 /dev/djsX 涓嬨€傛湰椹卞姩涓嶈瘯鍥句笌璇ユ帴鍙ｄ繚鎸佸吋瀹广€?


## Final Notes


```

  ____/|	Comments, additions, and specially corrections are welcome.
  \ o.O|	Documentation valid for at least version 1.2.8 of the joystick
   =(_)=	driver and as usual, the ultimate source for documentation is
     U		to "Use The Source Luke" or, at your convenience, Vojtech ;)

```