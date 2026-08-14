## Linux 鍔涘弽棣堬紙Force feedback锛?

:Author: Johann Deneux <johann.deneux@gmail.com>锛?001/04/22銆?:Updated: Anssi Hannula <anssi.hannula@gmail.com>锛?006/04/09銆?
浣犲彲浠ラ噸鏂板垎鍙戞湰鏂囦欢銆傝璁板緱鍚屾椂鍖呭惈 shape.svg 鍜?interactive.svg銆?
#### 绠€浠嬶紙Introduction锛?

鏈枃妗ｆ弿杩板浣曞湪 Linux 涓嬩娇鐢ㄥ姏鍙嶉璁惧銆傜洰鏍囦笉鏄儚瀵瑰緟绠€鍗曠殑浠呰緭鍏ヨ澶囬偅鏍锋敮鎸佽繖浜涜澶囷紙鍘熸湰宸茬粡濡傛锛夛紝鑰屾槸鐪熸鍚敤鍔涙晥鏋滐紙force effects锛夌殑娓叉煋銆?鏈枃妗ｄ粎鎻忚堪 Linux 杈撳叆鎺ュ彛鐨勫姏鍙嶉閮ㄥ垎銆傚湪杩涗竴姝ラ槄璇绘湰鏂囨。涔嬪墠锛岃鍏堥槄璇?joydev/joystick.rst 鍜?input.rst銆?
#### 缁欑敤鎴风殑浣跨敤璇存槑锛圛nstructions to the user锛?

瑕佸惎鐢ㄥ姏鍙嶉锛屼綘蹇呴』锛?
1. 灏嗗唴鏍搁厤缃负鍖呭惈 evdev 浠ュ強鏀寔浣犺澶囩殑椹卞姩銆?2. 纭繚宸插姞杞?evdev 妯″潡锛屽苟涓斿凡鍒涘缓 /dev/input/event* 璁惧鏂囦欢銆?
鍦ㄥ紑濮嬩箣鍓嶏紝鍏堟彁閱掍綘锛氭煇浜涜澶囧湪鍒濆鍖栭樁娈典細鍓х儓闇囧姩銆備緥濡傛垜鐨?"AVB Top Shot Pegasus" 灏变細鍑虹幇杩欑鎯呭喌銆傝鍋滄杩欑鎭间汉琛屼负锛屽皢浣犵殑鎽囨潌绉诲埌鏋侀檺浣嶇疆銆傛棤璁哄浣曪紝浣犻兘搴斿綋鐢ㄦ墜鎵朵綇璁惧锛屼互渚垮湪鍑虹幇闂鏃堕伩鍏嶅叾鎹熷潖銆?
濡傛灉浣犳湁涓€涓覆琛岀殑 iforce 璁惧锛屼綘闇€瑕佸惎鍔?inputattach銆傝瑙?joydev/joystick.rst銆?
### 瀹冨伐浣滃悧锛燂紙Does it work ?锛?

```
    % fftest /dev/input/eventXX
```

#### 缁欏紑鍙戣€呯殑浣跨敤璇存槑锛圛nstructions to the developer锛?

鎵€鏈変氦浜掗兘浣跨敤 event API 瀹屾垚銆備篃灏辨槸璇达紝浣犲彲浠ュ湪 /dev/input/eventXX 涓婁娇鐢?ioctl() 鍜?write()銆?姝や俊鎭彲鑳戒細鍙戠敓鍙樺寲銆?
### 鏌ヨ璁惧鑳藉姏锛圦uerying device capabilities锛?

```
    #include <linux/input.h>
    #include <sys/ioctl.h>

    #define BITS_TO_LONGS(x) \
	    (((x) + 8 * sizeof (unsigned long) - 1) / (8 * sizeof (unsigned long)))
    unsigned long features[BITS_TO_LONGS(FF_CNT)];
    int ioctl(int file_descriptor, int request, unsigned long *features);

```
"request" 蹇呴』涓?EVIOCGBIT(EV_FF, features 鏁扮粍鐨勫瓧鑺傚ぇ灏?

杩斿洖璁惧鏀寔鐨勭壒鎬с€俧eatures 鏄竴涓綅鍩燂紝鍖呭惈浠ヤ笅浣嶏細

- FF_CONSTANT	鍙覆鏌撴亽瀹氬姏鏁堟灉
- FF_PERIODIC	鍙覆鏌撳叿鏈変互涓嬫尝褰㈢殑鍛ㄦ湡鏁堟灉锛?
  - FF_SQUARE	  鏂规尝娉㈠舰
  - FF_TRIANGLE	  涓夎娉㈡尝褰?  - FF_SINE	  姝ｅ鸡娉㈡尝褰?  - FF_SAW_UP	  涓婂崌閿娇娉㈡尝褰?  - FF_SAW_DOWN	  涓嬮檷閿娇娉㈡尝褰?  - FF_CUSTOM	  鑷畾涔夋尝褰?
- FF_RAMP       鍙覆鏌撴枩鍧℃晥鏋?- FF_SPRING	鍙ā鎷熷脊绨х殑瀛樺湪
- FF_FRICTION	鍙ā鎷熸懇鎿﹀姏
- FF_DAMPER	鍙ā鎷熼樆灏兼晥鏋?- FF_RUMBLE	闇囧姩锛坮umble锛夋晥鏋?- FF_INERTIA    鍙ā鎷熸儻鎬?- FF_GAIN	澧炵泭鍙皟
- FF_AUTOCENTER	鑷姩灞呬腑鍙皟


    - 鍦ㄥぇ澶氭暟鎯呭喌涓嬶紝浣犲簲褰撲娇鐢?FF_PERIODIC 鑰岄潪 FF_RUMBLE銆傛墍鏈夋敮鎸?FF_RUMBLE 鐨勮澶囬兘鏀寔 FF_PERIODIC锛堟柟娉€佷笁瑙掓尝銆佹寮︽尝锛夛紝鍙嶄箣浜︾劧銆?
    - 鐩墠 FF_CUSTOM 鐨勭‘鍒囪娉曞皻鏈畾涔夛紝鍥犱负杩樻病鏈夐┍鍔ㄦ敮鎸佸畠銆?
```
    int ioctl(int fd, EVIOCGEFFECTS, int *n);
```
杩斿洖璁惧鍐呭瓨涓彲淇濆瓨鐨勬晥鏋滄暟閲忋€?
### 灏嗘晥鏋滀笂浼犲埌璁惧锛圲ploading effects to the device锛?

```
    #include <linux/input.h>
    #include <sys/ioctl.h>

    int ioctl(int file_descriptor, int request, struct ff_effect *effect);
```
"request" 蹇呴』涓?EVIOCSFF銆?
"effect" 鎸囧悜涓€涓弿杩板緟涓婁紶鏁堟灉鐨勭粨鏋勪綋銆傝鏁堟灉琚笂浼狅紝浣嗕笉浼氭挱鏀俱€?effect 鐨勫唴瀹瑰彲鑳借淇敼銆傜壒鍒槸锛屽叾 "id" 瀛楁浼氳璁句负椹卞姩鍒嗛厤鐨勫敮涓€ id銆傚湪鎵ц鏌愪簺鎿嶄綔锛堢Щ闄ゆ晥鏋溿€佹帶鍒舵挱鏀撅級鏃堕渶瑕佹鏁版嵁銆?鐢ㄦ埛蹇呴』灏?"id" 瀛楁璁句负 -1锛屼互鍛婄煡椹卞姩鍒嗛厤涓€涓柊鏁堟灉銆?
鏁堟灉鏄笌鏂囦欢鎻忚堪绗︾浉鍏崇殑銆?
鍏充簬 ff_effect 缁撴瀯浣撶殑璇存槑锛岃鍙傝 <uapi/linux/input.h>銆備綘涔熷彲浠ヤ粠 shape.svg 鍜?interactive.svg 杩欎袱涓枃浠跺寘鍚殑绀烘剰鍥句腑鑾峰緱甯姪锛?

    Shape


    Interactive


### 浠庤澶囦腑绉婚櫎鏁堟灉锛圧emoving an effect from the device锛?

```
    int ioctl(int fd, EVIOCRMFF, effect.id);
```
杩欎负璁惧鍐呭瓨涓殑鏂版晥鏋滆吘鍑虹┖闂淬€傛敞鎰忥紝濡傛灉璇ユ晥鏋滄鍦ㄦ挱鏀撅紝杩欎篃浼氬仠姝㈠畠銆?
### 鎺у埗鏁堟灉鐨勬挱鏀撅紙Controlling the playback of effects锛?

鎾斁鎺у埗閫氳繃 write() 瀹屾垚銆備笅闈㈡槸涓€涓ず渚嬶細

```
    #include <linux/input.h>
    #include <unistd.h>

	struct input_event play;
	struct input_event stop;
	struct ff_effect effect;
	int fd;
   ...
	fd = open("/dev/input/eventXX", O_RDWR);
   ...
	/* 鎾斁涓夋 */
	play.type = EV_FF;
	play.code = effect.id;
	play.value = 3;

	write(fd, (const void*) &play, sizeof(play));
   ...
	/* 鍋滄涓€涓晥鏋?*/
	stop.type = EV_FF;
	stop.code = effect.id;
	stop.value = 0;

	write(fd, (const void*) &stop, sizeof(stop));

```
### 璁剧疆澧炵泭锛圫etting the gain锛?

骞堕潪鎵€鏈夎澶囩殑鍔涘害閮界浉鍚屻€傚洜姝わ紝鐢ㄦ埛搴旀牴鎹笇鏈涙晥鏋滅殑寮哄害鏉ヨ缃竴涓鐩婂洜瀛愩€傝璁剧疆鍦ㄥ娆¤闂┍鍔ㄦ湡闂翠繚鎸佹湁鏁堛€?
```
    /* 璁剧疆璁惧鐨勫鐩?    int gain;		/* 浠嬩簬 0 鍒?100 涔嬮棿 */
    struct input_event ie;	/* 鐢ㄤ簬涓庨┍鍔ㄩ€氫俊鐨勭粨鏋勪綋 */

    ie.type = EV_FF;
    ie.code = FF_GAIN;
    ie.value = 0xFFFFUL * gain / 100;

    if (write(fd, &ie, sizeof(ie)) == -1)
	perror("set gain");

```
### 鍚敤/绂佺敤鑷姩灞呬腑锛圗nabling/Disabling autocenter锛?

鍦ㄦ垜鐪嬫潵锛岃嚜鍔ㄥ眳涓壒鎬х浉褰撳共鎵版晥鏋滅殑娓叉煋锛屾垜璁や负瀹冨簲褰撴槸涓€绉嶆晥鏋滐紝鍏惰绠楀彇鍐充簬娓告垙绫诲瀷銆備絾濡傛灉浣犳効鎰忥紝鍙互鍚敤瀹冦€?
```
    int autocenter;		/* 浠嬩簬 0 鍒?100 涔嬮棿 */
    struct input_event ie;

    ie.type = EV_FF;
    ie.code = FF_AUTOCENTER;
    ie.value = 0xFFFFUL * autocenter / 100;

    if (write(fd, &ie, sizeof(ie)) == -1)
	perror("set auto-center");

```
鍊间负 0 琛ㄧず鈥滄棤鑷姩灞呬腑鈥濄€?
### 鍔ㄦ€佹洿鏂版晥鏋滐紙Dynamic update of an effect锛?

杩囩▼涓庝笂浼犳柊鏁堟灉鐩稿悓锛屽彧鏄笉灏?id 瀛楁璁句负 -1锛岃€屾槸灏嗗叾璁句负鎵€闇€鐨勬晥鏋?id銆?閫氬父锛屾晥鏋滀笉浼氬仠姝㈠苟閲嶆柊鍚姩銆傜劧鑰岋紝鍙栧喅浜庤澶囩被鍨嬶紝骞堕潪鎵€鏈夊弬鏁伴兘鑳藉姩鎬佹洿鏂般€備緥濡傦紝瀵逛簬 iforce 璁惧锛屾晥鏋滅殑鏂瑰悜鏃犳硶鏇存柊銆傚湪杩欑鎯呭喌涓嬶紝椹卞姩浼氬仠姝㈣鏁堟灉銆侀噸鏂颁笂浼犲苟閲嶆柊鍚姩瀹冦€?
鍥犳锛屽缓璁粎鍦ㄤ互閲嶆斁娆℃暟涓?1 閲嶅惎鏁堟灉鍙帴鍙楃殑鎯呭喌涓嬶紝鍦ㄦ晥鏋滄挱鏀炬椂鍔ㄦ€佹洿鏀瑰叾鏂瑰悜銆?
### 鍏充簬鏁堟灉鐘舵€佺殑淇℃伅锛圛nformation about the status of effects锛?

姣忔鏁堟灉鐨勭姸鎬佸彂鐢熷彉鍖栨椂锛岄兘浼氬彂閫佷竴涓簨浠躲€傚叾鍊?```
    struct input_event {
    /* 褰撴晥鏋滅殑鐘舵€佸彂鐢熷彉鍖栨椂 */
	    struct timeval time;

    /* 璁句负 EV_FF_STATUS */
	    unsigned short type;

    /* 鍖呭惈鏁堟灉鐨?id */
	    unsigned short code;

    /* 鎸囩ず鐘舵€?*/
	    unsigned int value;
    };

    FF_STATUS_STOPPED	鏁堟灉宸插仠姝㈡挱鏀?    FF_STATUS_PLAYING	鏁堟灉宸插紑濮嬫挱鏀?
```

    - 鐘舵€佸弽棣堜粎鐢?iforce 椹卞姩鏀寔銆傚鏋滀綘纭湁鍏呭垎鐞嗙敱浣跨敤瀹冿紝璇疯仈绯?      linux-joystick@atrey.karlin.mff.cuni.cz 鎴?anssi.hannula@gmail.com锛?      浠ヤ究涓哄叾浣欓┍鍔ㄦ坊鍔犲姝ょ殑鏀寔銆?