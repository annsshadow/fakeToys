## 缂栧啓杈撳叆璁惧椹卞姩


#### 鏈€绠€鍗曠殑绀轰緥


涓嬮潰鏄竴涓瀬鍏剁畝鍗曠殑杈撳叆璁惧椹卞姩绀轰緥銆傝璁惧鍙湁涓€涓寜閽紝涓旀寜閽彲鍦?i/o 绔彛 BUTTON_PORT 澶勮闂€傚綋

```

    #include <linux/input.h>
    #include <linux/module.h>
    #include <linux/init.h>

    #include <asm/irq.h>
    #include <asm/io.h>

    static struct input_dev *button_dev;

    static irqreturn_t button_interrupt(int irq, void *dummy)
    {
	    input_report_key(button_dev, BTN_0, inb(BUTTON_PORT) & 1);
	    input_sync(button_dev);
	    return IRQ_HANDLED;
    }

    static int __init button_init(void)
    {
	    int error;

	    if (request_irq(BUTTON_IRQ, button_interrupt, 0, "button", NULL)) {
		    printk(KERN_ERR "button.c: Can't allocate irq %d\n", button_irq);
		    return -EBUSY;
	    }

	    button_dev = input_allocate_device();
	    if (!button_dev) {
		    printk(KERN_ERR "button.c: Not enough memory\n");
		    error = -ENOMEM;
		    goto err_free_irq;
	    }

	    button_dev->evbit[0] = BIT_MASK(EV_KEY);
	    button_dev->keybit[BIT_WORD(BTN_0)] = BIT_MASK(BTN_0);

	    error = input_register_device(button_dev);
	    if (error) {
		    printk(KERN_ERR "button.c: Failed to register device\n");
		    goto err_free_dev;
	    }

	    return 0;

    err_free_dev:
	    input_free_device(button_dev);
    err_free_irq:
	    free_irq(BUTTON_IRQ, button_interrupt);
	    return error;
    }

    static void __exit button_exit(void)
    {
	    input_unregister_device(button_dev);
	    free_irq(BUTTON_IRQ, button_interrupt);
    }

    module_init(button_init);
    module_exit(button_exit);

```
#### 绀轰緥鐨勪綔鐢?

棣栧厛瀹冨繀椤诲寘鍚?<linux/input.h> 鏂囦欢锛岃鏂囦欢鏄緭鍏ュ瓙绯荤粺鐨勬帴鍙ｃ€傚畠鎻愪緵浜嗘墍闇€鐨勬墍鏈夊畾涔夈€?
鍦?_init 鍑芥暟涓紙鏃犺鏄湪鍔犺浇妯″潡鏃惰繕鏄湪鍚姩鍐呮牳鏃惰皟鐢級锛屽畠浼氳幏鍙栨墍闇€鐨勮祫婧愶紙涔熷簲褰撴鏌ヨ澶囨槸鍚﹀瓨鍦級銆?
鐒跺悗瀹冮€氳繃 input_allocate_device() 鍒嗛厤涓€涓柊鐨勮緭鍏ヨ澶囩粨鏋勪綋锛屽苟璁剧疆杈撳叆浣嶅煙銆傝繖鏍疯澶囬┍鍔ㄥ氨鍛婅瘔杈撳叆绯荤粺鐨勫叾浠栭儴鍒嗗畠鏄粈涔堚€斺€旇杈撳叆璁惧鑳界敓鎴愭垨鎺ュ彈鍝簺浜嬩欢銆傛垜浠殑绀轰緥璁惧鍙兘鐢熸垚 EV_KEY 绫诲瀷鐨勪簨浠讹紝鑰屼笖鍦ㄨ繖浜涗簨浠朵腑浠?BTN_0 浜嬩欢鐮併€傚洜姝ゆ垜浠彧璁剧疆杩欎簺浣嶏紝

```

	set_bit(EV_KEY, button_dev->evbit);
	set_bit(BTN_0, button_dev->keybit);

```
鍚屾牱濡傛锛屼絾褰撴秹鍙婄殑浣嶄笉姝竴涓椂锛岀涓€绉嶅啓娉曞線寰€鏇寸畝鐭€?
```

	input_register_device(button_dev);

```
杩欏皢 button_dev 缁撴瀯浣撳姞鍏ヨ緭鍏ラ┍鍔ㄧ殑閾捐〃涓紝骞惰皟鐢ㄨ澶囧鐞嗘ā鍧楃殑 _connect 鍑芥暟锛屼互鍛婄煡瀹冧滑鍑虹幇浜嗕竴涓柊杈撳叆璁惧銆俰nput_register_device() 鍙兘浼氱潯鐪狅紝鍥犳涓嶈兘浠庝腑鏂笂涓嬫枃鎴栨寔鏈夎嚜鏃嬮攣鏃惰皟鐢ㄣ€?
```

	button_interrupt()

```
瀹冧細鍦ㄦ寜閽瘡娆′骇鐢熶腑鏂椂妫€鏌ュ叾鐘舵€侊紝骞舵姤鍛?
```

	input_report_key()

```
缁欒緭鍏ョ郴缁熴€傛棤闇€妫€鏌ヤ腑鏂緥绋嬫槸鍚﹀悜杈撳叆绯荤粺鎶ュ憡浜嗕袱涓浉鍚屽€肩殑浜嬩欢锛堜緥濡備袱娆℃寜涓嬶級锛屽洜涓?input_report_* 鍑芥暟鑷韩浼氬仛杩欎釜妫€鏌ャ€?
```

	input_sync()

```
璋冪敤鐢ㄤ簬鍛婅瘔閭ｄ簺鎺ユ敹浜嬩欢鐨勫璞★細鎴戜滑宸茬粡鍙戦€佷簡涓€浠藉畬鏁寸殑鎶ュ憡銆傚湪鍙湁涓€涓寜閽殑鎯呭喌涓嬭繖浼间箮鏃犲叧绱ц锛屼絾瀵逛簬榧犳爣绉诲姩绛夋儏鍐靛垯鐩稿綋閲嶈鈥斺€斾綘涓嶄細甯屾湜 X 鍜?Y 鍊艰鍒嗗埆瑙ｉ噴锛屽洜涓洪偅鏍蜂細瀵艰嚧涓嶅悓鐨勭Щ鍔ㄣ€?
#### dev->open() 涓?dev->close()


濡傛灉椹卞姩蹇呴』鍙嶅杞璁惧锛屽洜涓鸿澶囦笉浼氫骇鐢熶腑鏂紝鑰岃疆璇㈢殑寮€閿€鍙堣繃澶ф棤娉曚竴鐩磋繘琛岋紱鎴栬€呰澶囦娇鐢ㄤ簡瀹濊吹璧勬簮锛堜緥濡備腑鏂級锛岄偅涔堝畠鍙互鍒╃敤 open 鍜?close 鍥炶皟鏉ュ緱鐭ヤ綍鏃跺彲浠ュ仠姝㈣疆璇㈡垨閲婃斁涓柇锛屼互鍙婁綍鏃跺繀椤绘仮澶嶈疆璇㈡垨鑾峰彇涓柇銆?
```

    static int button_open(struct input_dev *dev)
    {
	    if (request_irq(BUTTON_IRQ, button_interrupt, 0, "button", NULL)) {
		    printk(KERN_ERR "button.c: Can't allocate irq %d\n", button_irq);
		    return -EBUSY;
	    }

	    return 0;
    }

    static void button_close(struct input_dev *dev)
    {
	    free_irq(IRQ_AMIGA_VERTB, button_interrupt);
    }

    static int __init button_init(void)
    {
	    ...
	    button_dev->open = button_open;
	    button_dev->close = button_close;
	    ...
    }

```
娉ㄦ剰锛岃緭鍏ユ牳蹇冧細璁板綍璁惧鐨勭敤鎴锋暟閲忥紝骞剁‘淇濆彧鏈夊湪绗竴涓敤鎴疯繛鎺ュ埌璁惧鏃舵墠璋冪敤 dev->open()锛屼互鍙婂彧鍦ㄦ渶鍚庝竴涓敤鎴锋柇寮€杩炴帴鏃舵墠璋冪敤 dev->close()銆傚杩欎袱涓洖璋冪殑璋冪敤鏄覆琛屽寲鐨勩€?
open() 鍥炶皟鎴愬姛鏃跺簲杩斿洖 0锛屽け璐ユ椂搴旇繑鍥炰换鎰忛潪闆跺€笺€俢lose() 鍥炶皟锛堝叾杩斿洖绫诲瀷涓?void锛夊繀椤诲缁堟垚鍔熴€?
#### 鎶戝埗锛坕nhibit锛夎緭鍏ヨ澶?

鎶戝埗涓€涓澶囨剰鍛崇潃蹇界暐鏉ヨ嚜瀹冪殑杈撳叆浜嬩欢銆傚洜姝わ紝瀹冨叧涔庝笌杈撳叆澶勭悊绋嬪簭涔嬮棿鍏崇郴鐨勭淮鎶も€斺€旀棤璁烘槸宸叉湁鐨勫叧绯伙紝杩樻槸鍦ㄨ澶囧浜庢姂鍒剁姸鎬佹湡闂村皢瑕佸缓绔嬬殑鍏崇郴銆?
濡傛灉涓€涓澶囪鎶戝埗锛屼换浣曡緭鍏ュ鐞嗙▼搴忛兘涓嶄細鏀跺埌鏉ヨ嚜瀹冪殑浜嬩欢銆?
閫氳繃鍒╃敤鈥滄病鏈変汉闇€瑕佽璁惧鐨勪簨浠垛€濊繖涓€浜嬪疄锛屽湪鎶戝埗锛坕nhibit锛夊拰瑙ｆ姂鍒讹紙uninhibit锛夋搷浣滄椂锛屽垎鍒皟鐢ㄨ澶囩殑 close()锛堝鏋滄湁鐢ㄦ埛锛夊拰 open()锛堝鏋滄湁鐢ㄦ埛锛夛紝鍙互杩涗竴姝ュ姞浠ュ埄鐢ㄣ€傜殑纭紝close() 鐨勫惈涔夋槸鍋滄鍚戣緭鍏ユ牳蹇冩彁渚涗簨浠讹紝鑰?open() 鐨勫惈涔夋槸寮€濮嬪悜杈撳叆鏍稿績鎻愪緵浜嬩欢銆?
鍦ㄦ姂鍒舵椂璋冪敤璁惧鐨?close() 鏂规硶锛堝鏋滄湁鐢ㄦ埛锛夊彲浠ヨ椹卞姩鑺傜渷鍔熻€椼€傛棤璁烘槸鐩存帴鍏抽棴璁惧鐢垫簮锛岃繕鏄湪椹卞姩浣跨敤杩愯鏃?PM 鏃堕噴鏀惧畠鍦?open() 涓幏鍙栫殑杩愯鏃?PM 寮曠敤閮藉彲浠ャ€?
鎶戝埗涓庤В鎶戝埗锛屼笌杈撳叆澶勭悊绋嬪簭鎵撳紑鍜屽叧闂澶囨槸姝ｄ氦鐨勩€傜敤鎴风┖闂村彲鑳藉笇鏈涘湪浠讳綍涓€涓鐞嗙▼搴忚姝ｅ悜鍖归厤涔嬪墠锛屽氨鎻愬墠鎶戝埗鏌愪釜璁惧銆?
鎶戝埗涓庤В鎶戝埗锛屼篃涓庤澶囨槸鍚︿负鍞ら啋婧愭槸姝ｄ氦鐨勩€傛槸鍚︿负鍞ら啋婧愬湪绯荤粺鐫＄湢鏃惰捣浣滅敤锛岃€屼笉鏄湪绯荤粺杩愯鏃惰捣浣滅敤銆傞┍鍔ㄥ簲褰撳浣曠紪绋嬪叾鎶戝埗銆佺潯鐪犱笌浣滀负鍞ら啋婧愪箣闂寸殑浜や簰锛屾槸椹卞姩鐗瑰畾鐨勪簨鎯呫€?
鎵撲釜姣旀柟锛岀綉鍗♀€斺€旀妸缃戠粶鎺ュ彛 down 鎺夛紝骞朵笉鎰忓懗鐫€涓嶅簲鑳介€氳繃姝ゆ帴鍙ｅ湪 LAN 涓婂敜閱掔郴缁熴€傚洜姝わ紝鍙兘瀛樺湪鍗充娇琚姂鍒朵篃搴旇瑙嗕负鍞ら啋婧愮殑杈撳叆椹卞姩銆傚疄闄呬笂锛屽湪璁稿 I2C 杈撳叆璁惧涓紝瀹冧滑鐨勪腑鏂澹版槑涓哄敜閱掍腑鏂紝鍏跺鐞嗗彂鐢熷湪椹卞姩鏍稿績涓紝鑰岄┍鍔ㄦ牳蹇冨苟涓嶇煡閬撹緭鍏ョ壒瀹氱殑鎶戝埗锛堜篃涓嶅簲璇ョ煡閬擄級銆傚寘鍚涓帴鍙ｇ殑澶嶅悎璁惧鍙互鍩轰簬姣忎釜鎺ュ彛琚姂鍒讹紝渚嬪鎶戝埗鏌愪竴涓帴鍙ｄ笉搴斿奖鍝嶈璁惧浣滀负鍞ら啋婧愮殑鑳藉姏銆?
濡傛灉涓€涓澶囧湪琚姂鍒舵湡闂磋琚涓哄敜閱掓簮锛屽垯鍦ㄥ瀹冪殑 suspend() 缂栫▼鏃跺繀椤绘牸澶栧皬蹇冿紝鍥犱负瀹冨彲鑳介渶瑕佽皟鐢ㄨ澶囩殑 open()銆傚彇鍐充簬 close() 瀵圭浉搴旇澶囩殑鍚箟锛屽湪鐫＄湢鍓嶄笉璋冪敤 open() 鍙兘浣垮畠鏃犳硶鎻愪緵浠讳綍鍞ら啋浜嬩欢銆傛棤璁哄浣曡澶囬兘瑕佽繘鍏ョ潯鐪犮€?
#### 鍩烘湰浜嬩欢绫诲瀷


鏈€绠€鍗曠殑浜嬩欢绫诲瀷鏄?EV_KEY锛岀敤浜庢寜閿拰鎸夐挳銆?
```

	input_report_key(struct input_dev *dev, int code, int value)

```
鍙傝 uapi/linux/input-event-codes.h 浜嗚В code 鐨勫彲鍏佽鍙栧€硷紙浠?0 鍒?KEY_MAX锛夈€倂alue 琚В閲婁负鐪熷€硷紝鍗充换鎰忛潪闆跺€艰〃绀烘寜閿寜涓嬶紝闆跺€艰〃绀烘寜閿澗寮€銆傝緭鍏ヤ唬鐮佷粎鍦?value 涓庝箣鍓嶄笉鍚岀殑鎯呭喌涓嬫墠鐢熸垚浜嬩欢銆?
闄や簡 EV_KEY锛岃繕鏈変袱绉嶆洿鍩烘湰鐨勪簨浠剁被鍨嬶細EV_REL 鍜?EV_ABS銆傚畠浠敤浜庤澶囨彁渚涚殑鐩稿鍊煎拰缁濆鍊笺€傜浉瀵瑰€间緥濡傚彲浠ユ槸榧犳爣鍦?X 杞翠笂鐨勭Щ鍔ㄣ€傞紶鏍囧皢鍏舵姤鍛婁负鐩稿涓婃浣嶇疆鐨勪綅绉伙紝鍥犱负瀹冩病鏈変换浣曞彲宸ヤ綔鐨勭粷瀵瑰潗鏍囩郴缁熴€傜粷瀵逛簨浠跺垯鐢ㄤ簬鎽囨潌鍜屾暟瀛楀寲浠€斺€旈偅浜涚‘瀹炲伐浣滃湪缁濆鍧愭爣绯荤粺涓殑璁惧銆?
璁╄澶囨姤鍛?EV_REL 鎸夐挳涓?EV_KEY 涓€鏍风畝鍗曪紱鍙渶

```

	input_report_rel(struct input_dev *dev, int code, int value)

```
鍑芥暟銆備粎瀵归潪闆跺€肩敓鎴愪簨浠躲€?
鐒惰€?EV_ABS 闇€瑕佷竴鐐圭壒鍒暀鎰忋€傚湪璋冪敤 input_register_device 涔嬪墠锛屼綘瑕佷负璁惧鐨勬瘡涓粷瀵硅酱鍦?input_dev 缁撴瀯浣撲腑濉厖棰濆瀛楁銆傚鏋滄垜浠殑鎸夐挳璁惧杩樻湁

```

	button_dev.absmin[ABS_X] = 0;
	button_dev.absmax[ABS_X] = 255;
	button_dev.absfuzz[ABS_X] = 4;
	button_dev.absflat[ABS_X] = 8;

```

```

	input_set_abs_params(button_dev, ABS_X, 0, 255, 4, 8);

```
杩欎釜璁剧疆閫傜敤浜庢憞鏉嗙殑 X 杞达紝鏈€灏忓€间负 0锛屾渶澶у€间负 255锛堟憞鏉?*蹇呴』**鑳藉杈惧埌锛屽嵆浣挎湁鏃舵姤鍛婃洿澶у€间篃娌￠棶棰橈紝浣嗗畠蹇呴』濮嬬粓鑳借揪鍒版渶灏忓€煎拰鏈€澶у€硷級锛屾暟鎹櫔澹版渶澶т负 卤4锛屼腑蹇冨钩鍧﹀尯澶у皬涓?8銆?
濡傛灉浣犱笉闇€瑕?absfuzz 鍜?absflat锛屽彲浠ュ皢瀹冧滑璁句负闆讹紝杩欒〃绀鸿璁惧绮剧‘涓旀€绘槸绮剧‘鍥炲埌涓績浣嶇疆锛堝鏋滄湁鐨勮瘽锛夈€?
#### BITS_TO_LONGS()銆丅IT_WORD()銆丅IT_MASK()


```

	BITS_TO_LONGS(x) - 杩斿洖 x 涓瘮鐗瑰搴旂殑浣嶅煙鏁扮粍锛堜互 long 璁★級鐨勯暱搴?	BIT_WORD(x)	 - 杩斿洖姣旂壒 x 鍦ㄦ暟缁勪腑鐨?long 绱㈠紩
	BIT_MASK(x)	 - 杩斿洖姣旂壒 x 鍦ㄤ竴涓?long 涓殑绱㈠紩

```
#### id* 涓?name 瀛楁


dev->name 搴旂敱杈撳叆璁惧椹卞姩鍦ㄦ敞鍐岃緭鍏ヨ澶囦箣鍓嶈缃€傚畠鏄竴涓舰濡?'Generic button device' 鐨勫瓧绗︿覆锛屽寘鍚澶囧鐢ㄦ埛鍙嬪ソ鐨勫悕绉般€?
id* 瀛楁鍖呭惈璁惧鐨勬€荤嚎 ID锛圥CI銆乁SB 绛夛級銆佸巶鍟?ID 鍜岃澶?ID銆傛€荤嚎 ID 瀹氫箟浜?input.h 涓€傚巶鍟嗗拰璁惧 ID 瀹氫箟浜?pci_ids.h銆乽sb_ids.h 鍙婄被浼肩殑澶存枃浠朵腑銆傝繖浜涘瓧娈靛簲鐢辫緭鍏ヨ澶囬┍鍔ㄥ湪娉ㄥ唽涔嬪墠璁剧疆銆?
idtype 瀛楁鍙敤浜庤緭鍏ヨ澶囬┍鍔ㄧ殑鐗瑰畾淇℃伅銆?
id 鍜?name 瀛楁鍙互閫氳繃 evdev 鎺ュ彛浼犻€掔粰鐢ㄦ埛绌洪棿銆?
#### keycode銆乲eycodemax銆乲eycodesize 瀛楁


杩欎笁涓瓧娈靛簲鐢卞叿鏈夊瘑闆嗛敭鏄犲皠鐨勮緭鍏ヨ澶囦娇鐢ㄣ€俴eycode 鏄竴涓暟缁勶紝鐢ㄤ簬浠庢壂鎻忕爜鏄犲皠鍒拌緭鍏ョ郴缁熺殑閿爜銆俴eycode max 搴斿寘鍚暟缁勭殑澶у皬锛宬eycodesize 鍒欏寘鍚叾涓瘡涓潯鐩殑澶у皬锛堜互瀛楄妭璁★級銆?
鐢ㄦ埛绌洪棿鍙互浣跨敤瀵瑰簲 evdev 鎺ュ彛涓婄殑 EVIOCGKEYCODE 鍜?EVIOCSKEYCODE ioctl 鏉ユ煡璇㈠拰淇敼褰撳墠鐨勬壂鎻忕爜鍒伴敭鐮佹槧灏勩€傚綋涓€涓澶囧～濂戒簡涓婅堪鍏ㄩ儴涓変釜瀛楁锛岄┍鍔ㄥ彲浠ヤ緷璧栧唴鏍搁粯璁ゅ疄鐜扮殑閿爜鏄犲皠璁剧疆涓庢煡璇€?
#### dev->getkeycode() 涓?dev->setkeycode()


getkeycode() 鍜?setkeycode() 鍥炶皟鍏佽椹卞姩瑕嗙洊鐢辫緭鍏ユ牳蹇冩彁渚涚殑榛樿 keycode/keycodesize/keycodemax 鏄犲皠鏈哄埗锛屽苟瀹炵幇绋€鐤忛敭鐮佹槧灏勩€?
#### 鎸夐敭鑷姩閲嶅


鈥︹€﹀緢绠€鍗曘€傚畠鐢?input.c 妯″潡澶勭悊銆備笉浣跨敤纭欢鑷姩閲嶅锛屽洜涓哄畠鍦ㄨ澶氳澶囦腑骞朵笉瀛樺湪锛屽嵆浣垮湪瀛樺湪鐨勫湴鏂规湁鏃朵篃鏄潖鐨勶紙渚嬪閿洏锛氫笢鑺濈瑪璁版湰锛夈€傝涓轰綘鐨勮澶囧惎鐢ㄨ嚜鍔ㄩ噸澶嶏紝鍙渶鍦?dev->evbit 涓缃?EV_REP 鍗冲彲銆傚叾浣欏叏閮ㄧ敱杈撳叆绯荤粺澶勭悊銆?
#### 鍏朵粬浜嬩欢绫诲瀷銆佸鐞嗚緭鍑轰簨浠?

鍒扮洰鍓嶄负姝㈢殑鍏朵粬浜嬩欢绫诲瀷鏈夛細

- EV_LED - 鐢ㄤ簬閿洏 LED銆?- EV_SND - 鐢ㄤ簬閿洏铚傞福銆?
瀹冧滑涓庝緥濡傛寜閿簨浠堕潪甯哥浉浼硷紝浣嗘柟鍚戠浉鍙嶁€斺€斾粠绯荤粺鍒拌緭鍏ヨ澶囬┍鍔ㄣ€傚鏋滀綘鐨勮緭鍏ヨ澶囬┍鍔ㄨ兘澶勭悊杩欎簺浜嬩欢锛屽畠蹇呴』鍦?evbit 涓缃浉搴旂殑浣嶏紝

```

    button_dev->event = button_event;

    int button_event(struct input_dev *dev, unsigned int type,
		     unsigned int code, int value)
    {
	    if (type == EV_SND && code == SND_BELL) {
		    outb(value, BUTTON_BELL);
		    return 0;
	    }
	    return -1;
    }

```
璇ュ洖璋冧緥绋嬪彲浠ヤ粠涓柇鎴?BH锛堜笅鍗婇儴锛変腑璋冪敤锛堝敖绠¤繖涓嶆槸纭€ц瀹氾級锛屽洜姝ょ粷涓嶈兘鐫＄湢锛屼笖蹇呴』灏藉揩瀹屾垚銆?
#### 杞寮忚緭鍏ヨ澶?

杈撳叆杞閫氳繃浼犲叆涓€涓緭鍏ヨ澶囩粨鏋勪綋鍜屽洖璋冩潵璁剧疆锛?
```

    int input_setup_polling(struct input_dev *dev,
        void (*poll_fn)(struct input_dev *dev))

```
鍦ㄥ洖璋冨唴閮紝璁惧搴斿綋浣跨敤鍏朵粬璁惧鎵€浣跨敤鐨勫父瑙?input_report_* 鍑芥暟鍜?input_sync銆?
```

    void input_set_poll_interval(struct input_dev *dev, unsigned int interval)

```
瀹冪敤浜庨厤缃澶囪杞鐨勯棿闅旓紝浠ユ绉掍负鍗曚綅銆?