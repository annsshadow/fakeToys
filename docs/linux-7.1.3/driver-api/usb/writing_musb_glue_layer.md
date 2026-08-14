## 缂栧啓 MUSB Glue Layer锛堢矘鍚堝眰锛?

:Author: Apelete Seketeli

## 绠€浠?

Linux MUSB 瀛愮郴缁熸槸鏇村ぇ鐨?Linux USB 瀛愮郴缁熺殑涓€閮ㄥ垎銆傚畠涓洪偅浜涗笉
浣跨敤 Universal Host Controller Interface (UHCI) 鎴?Open Host
Controller Interface (OHCI) 鐨勫祵鍏ュ紡 USB Device Controller (UDC)
鎻愪緵鏀寔銆?

鐩稿弽锛岃繖浜涘祵鍏ュ紡 UDC 渚濊禆浜?USB On-the-Go (OTG) 瑙勮寖锛屽苟涓旇嚦灏?
閮ㄥ垎鍦板疄鐜颁簡璇ヨ鑼冦€傚ぇ澶氭暟鎯呭喌涓嬩娇鐢ㄧ殑纭呯墖鍙傝€冭璁℃槸 Mentor
Graphics Inventra鈩?璁捐涓殑 Multipoint USB Highspeed
Dual-Role Controller (MUSB HDRC)銆?

浣滀负涓€娆¤嚜瀛︾粌涔狅紝鎴戜负 Ingenic JZ4740 SoC 缂栧啓浜嗕竴涓?MUSB glue
layer锛屽叾妯″瀷鍙傜収浜嗗唴鏍告簮鐮佹爲涓殑浼楀 MUSB glue layer銆傝灞備綅浜?
`drivers/usb/musb/jz4740.c`銆傚湪鏈枃妗ｄ腑锛屾垜灏嗛€愭璁茶В `jz4740.c`
杩欎釜 glue layer 鐨勫熀纭€鐭ヨ瘑锛岃В閲婂叾涓悇涓粍鎴愰儴鍒嗭紝浠ュ強缂栧啓涓€涓?
灞炰簬浣犺嚜宸辩殑璁惧 glue layer 闇€瑕佸仛浜涗粈涔堛€?

## Linux MUSB 鍩虹

瑕佸紑濮嬩簡瑙ｈ繖涓富棰橈紝璇烽槄璇?USB On-the-Go Basics锛堝弬瑙?
Resources锛夛紝瀹冧粠纭欢灞傞潰浠嬬粛浜?USB OTG 鐨勬搷浣溿€俆exas Instruments
鍜?Analog Devices 鐨勫嚑绡?wiki 椤甸潰涔熸杩颁簡 Linux 鍐呮牳 MUSB 鐨勯厤缃紝
灏界瀹冧滑渚ч噸浜庤繖涓ゅ鍏徃鎻愪緵鐨勬煇浜涚壒瀹氳澶囥€傛渶鍚庯紝閫氳繃 USB home
page 鏉ョ啛鎮?USB 瑙勮寖鍙兘浼氭湁鎵€甯姪锛屽苟涓斿彲浠ラ€氳繃 Writing USB
Device Drivers 鏂囨。锛堝悓鏍峰弬瑙?Resources锛夎幏寰楀疄鐢ㄧ殑瀹炰緥銆?

Linux USB 鍗忚鏍堟槸涓€涓垎灞傛灦鏋勶紝MUSB 鎺у埗鍣ㄧ‖浠朵綅浜庢渶搴曞眰銆侻USB
鎺у埗鍣ㄩ┍鍔ㄥ浠ヤ笅閮ㄥ垎杩涜浜嗘娊璞★細

```
	  ------------------------
	  |                      | <------- drivers/usb/gadget
	  | Linux USB Core Stack | <------- drivers/usb/host
	  |                      | <------- drivers/usb/core
	  ------------------------
		     猬?
	 --------------------------
	 |                        | <------ drivers/usb/musb/musb_gadget.c
	 | MUSB Controller driver | <------ drivers/usb/musb/musb_host.c
	 |                        | <------ drivers/usb/musb/musb_core.c
	 --------------------------
		     猬?
      ---------------------------------
      | MUSB Platform Specific Driver |
      |                               | <-- drivers/usb/musb/jz4740.c
      |       aka "Glue Layer"        |
      ---------------------------------
		     猬?
      ---------------------------------
      |   MUSB Controller Hardware    |
      ---------------------------------
```

濡備笂鎵€杩帮紝glue layer 瀹為檯涓婃槸浣嶄簬鎺у埗鍣ㄩ┍鍔ㄤ笌鎺у埗鍣ㄧ‖浠朵箣闂寸殑
骞冲彴鐩稿叧浠ｇ爜銆?

灏卞儚 Linux USB 椹卞姩闇€瑕佸悜 Linux USB 瀛愮郴缁熸敞鍐岃嚜宸变竴鏍凤紝MUSB glue
layer 闇€瑕佸厛鍚?MUSB 鎺у埗鍣ㄩ┍鍔ㄦ敞鍐岃嚜宸便€傝繖鏍锋帶鍒跺櫒椹卞姩灏辫兘鐭ラ亾
璇?glue layer 鏀寔鍝簺璁惧锛屼互鍙婂湪妫€娴嬪埌鎴栭噴鏀惧彈鏀寔鐨勮澶囨椂璇?
璋冪敤鍝簺鍑芥暟锛涜璁颁綇锛屾垜浠繖閲岃璁虹殑鏄竴涓祵鍏ュ紡鎺у埗鍣ㄨ姱鐗囷紝
鍥犳涓嶅瓨鍦ㄨ繍琛屾椂鐨勬彃鍏ユ垨绉婚櫎銆?

鎵€鏈夎繖浜涚浉鍏充俊鎭兘閫氳繃浠ヤ笅鏂瑰紡浼犻€掔粰 MUSB 鎺у埗鍣ㄩ┍鍔細

```
    static struct platform_driver jz4740_driver = {
	.probe      = jz4740_probe,
	.remove     = jz4740_remove,
	.driver     = {
	    .name   = "musb-jz4740",
	},
    };
```

probe 鍜?remove 鍑芥暟鎸囬拡鍒嗗埆鍦ㄦ娴嬪埌鍖归厤鐨勮澶囧拰锛堢浉搴斿湴锛夐噴鏀?
璁惧鏃惰璋冪敤銆俷ame 瀛楃涓叉弿杩颁簡璇?glue layer 鎵€鏀寔鐨勮澶囥€傚湪褰撳墠
鎯呭喌涓嬶紝瀹冧笌 `arch/mips/jz4740/platform.c` 涓０鏄庣殑 platform_device
缁撴瀯鐩稿尮閰嶃€傛敞鎰忥紝鎴戜滑杩欓噷娌℃湁浣跨敤 device tree bindings銆?

涓轰簡鍚戞帶鍒跺櫒椹卞姩瀹屾垚娉ㄥ唽锛実lue layer 瑕佺粡鍘嗗嚑涓楠わ紝鍩烘湰涓婃槸
鍒嗛厤鎺у埗鍣ㄧ‖浠惰祫婧愬苟鍒濆鍖栬嫢骞叉ā鍧椼€備负姝わ紝瀹冮渶瑕佽窡韪繖浜涙楠や腑
鎵€浣跨敤鐨勭浉鍏充俊鎭€傝繖鏄€氳繃浠ヤ笅缁撴瀯瀹屾垚鐨勶細

```
    struct jz4740_glue {
	struct device           *dev;
	struct platform_device  *musb;
	struct clk      *clk;
    };
```

dev 鍜?musb 鎴愬憳閮芥槸 device 缁撴瀯鍙橀噺銆傜涓€涓垚鍛樹繚瀛樺叧浜庤璁惧鐨?
閫氱敤淇℃伅锛屽洜涓哄畠鏄渶鍩虹鐨勮澶囩粨鏋勶紱鑰屽悗鑰呬繚瀛樹笌璁惧鎵€娉ㄥ唽鍒扮殑
瀛愮郴缁熸洿瀵嗗垏鐩稿叧鐨勪俊鎭€俢lk 鍙橀噺淇濆瓨涓庤澶囨椂閽熸搷浣滅浉鍏崇殑淇℃伅銆?

璁╂垜浠潵鐪嬬湅 probe 鍑芥暟涓偅浜涗娇 glue layer 鍚戞帶鍒跺櫒椹卞姩瀹屾垚娉ㄥ唽
鐨勫悇涓楠ゃ€?

   鍑轰簬鍙鎬ц€冭檻锛屾瘡涓嚱鏁板皢琚媶鍒嗘垚閫昏緫涓婄殑鑻ュ共閮ㄥ垎锛屾瘡涓€閮ㄥ垎
   閮藉儚褰兼鐙珛涓€鏍峰睍绀恒€?

    :emphasize-lines: 8,12,18

    static int jz4740_probe(struct platform_device *pdev)
    {
	struct platform_device      *musb;
	struct jz4740_glue      *glue;
	struct clk                      *clk;
	int             ret;

	glue = devm_kzalloc(&pdev->dev, sizeof(*glue), GFP_KERNEL);
	if (!glue)
	    return -ENOMEM;

	musb = platform_device_alloc("musb-hdrc", PLATFORM_DEVID_AUTO);
	if (!musb) {
	    dev_err(&pdev->dev, "failed to allocate musb device\n");
	    return -ENOMEM;
	}

	clk = devm_clk_get(&pdev->dev, "udc");
	if (IS_ERR(clk)) {
	    dev_err(&pdev->dev, "failed to get clock\n");
	    ret = PTR_ERR(clk);
	    goto err_platform_device_put;
	}

	ret = clk_prepare_enable(clk);
	if (ret) {
	    dev_err(&pdev->dev, "failed to enable clock\n");
	    goto err_platform_device_put;
	}

	musb->dev.parent        = &pdev->dev;

	glue->dev           = &pdev->dev;
	glue->musb          = musb;
	glue->clk           = clk;

	return 0;

    err_platform_device_put:
	platform_device_put(musb);
	return ret;
    }

probe 鍑芥暟鐨勫墠鍑犺鍒嗛厤骞惰祴鍊?glue銆乵usb 鍜?clk 鍙橀噺銆俙GFP_KERNEL`
鏍囧織锛堢 8 琛岋級鍏佽鍒嗛厤杩囩▼鐫＄湢骞剁瓑寰呭唴瀛橈紝鍥犳鍙敤浜庡姞閿佺殑
鎯呭舰銆俙PLATFORM_DEVID_AUTO` 鏍囧織锛堢 12 琛岋級鍏佽鑷姩鍒嗛厤鍜岀鐞?
璁惧 ID锛屼互閬垮厤涓庢樉寮?ID 浜х敓璁惧鍛藉悕绌洪棿鍐茬獊銆傞€氳繃
`devm_clk_get`锛堢 18 琛岋級锛実lue layer 鍒嗛厤鏃堕挓鈥斺€擿devm_` 鍓嶇紑琛ㄧず
`clk_get` 鏄彈绠＄悊鐨勶細褰撹澶囪閲婃斁鏃跺畠浼氳嚜鍔ㄩ噴鏀炬墍鍒嗛厤鐨勬椂閽?
璧勬簮鏁版嵁鈥斺€斿苟鍚敤瀹冦€?

鎺ヤ笅鏉ユ槸娉ㄥ唽姝ラ锛?

    :emphasize-lines: 3,5,7,9,16

    static int jz4740_probe(struct platform_device *pdev)
    {
	struct musb_hdrc_platform_data  *pdata = &jz4740_musb_platform_data;

	pdata->platform_ops     = &jz4740_musb_ops;

	platform_set_drvdata(pdev, glue);

	ret = platform_device_add_resources(musb, pdev->resource,
			    pdev->num_resources);
	if (ret) {
	    dev_err(&pdev->dev, "failed to add resources\n");
	    goto err_clk_disable;
	}

	ret = platform_device_add_data(musb, pdata, sizeof(*pdata));
	if (ret) {
	    dev_err(&pdev->dev, "failed to add platform_data\n");
	    goto err_clk_disable;
	}

	return 0;

    err_clk_disable:
	clk_disable_unprepare(clk);
    err_platform_device_put:
	platform_device_put(musb);
	return ret;
    }

绗竴姝ユ槸閫氳繃 `platform_set_drvdata`锛堢 7 琛岋級灏?glue layer 绉佹湁
鎸佹湁鐨勮澶囨暟鎹紶閫掔粰鎺у埗鍣ㄩ┍鍔ㄣ€傛帴涓嬫潵鏄€氳繃
`platform_device_add_resources`锛堢 9 琛岋級浼犻€掕澶囪祫婧愪俊鎭紝姝ゆ椂
杩欎簺淇℃伅鍚屾牱涓虹鏈夋寔鏈夈€?

鏈€鍚庢槸鍚戞帶鍒跺櫒椹卞姩浼犻€掑钩鍙扮浉鍏虫暟鎹紙绗?16 琛岋級銆侾latform data
灏嗗湪 musb-dev-platform-data 涓璁猴紝浣嗚繖閲屾垜浠鐪嬬殑鏄?
`musb_hdrc_platform_data` 缁撴瀯锛堢 3 琛岋級涓殑 `platform_ops` 鍑芥暟
鎸囬拡锛堢 5 琛岋級銆傝繖涓嚱鏁版寚閽堝厑璁?MUSB 鎺у埗鍣ㄩ┍鍔ㄥ湪闇€瑕佹椂璋冪敤
浠ヤ笅鍑芥暟锛?

```
    static const struct musb_platform_ops jz4740_musb_ops = {
	.init       = jz4740_musb_init,
	.exit       = jz4740_musb_exit,
    };
```

杩欓噷鏄渶绮剧畝鐨勬儏鍐碉紝鎺у埗鍣ㄩ┍鍔ㄤ粎鍦ㄩ渶瑕佹椂璋冪敤 init 鍜?exit 鍑芥暟銆?
浜嬪疄涓?JZ4740 MUSB 鎺у埗鍣ㄦ槸涓€涓熀纭€鍨嬫帶鍒跺櫒锛岀己灏戝叾浠栨帶鍒跺櫒涓?
鍏峰鐨勪竴浜涚壒鎬э紝鍚﹀垯鎴戜滑鍙兘杩橀渶瑕佹寚鍚戝叾浠栦竴浜涘嚱鏁扮殑鎸囬拡锛屼緥濡?
鐢垫簮绠＄悊鍑芥暟锛屾垨鍦?OTG 涓庨潪 OTG 妯″紡涔嬮棿鍒囨崲鐨勫嚱鏁扮瓑绛夈€?

鍦ㄦ敞鍐岀殑閭ｄ釜鏃跺埢锛屾帶鍒跺櫒椹卞姩浼氬疄闄呰皟鐢?init 鍑芥暟锛?

   .. code-block:: c
    :emphasize-lines: 12,14

    static int jz4740_musb_init(struct musb *musb)
    {
	musb->xceiv = usb_get_phy(USB_PHY_TYPE_USB2);
	if (!musb->xceiv) {
	    pr_err("HS UDC: no transceiver configured\n");
	    return -ENODEV;
	}

	/* Silicon does not implement ConfigData register.
  - Set dyn_fifo to avoid reading EP config from hardware.
	 */
	musb->dyn_fifo = true;

	musb->isr = jz4740_musb_interrupt;

	return 0;
    }

`jz4740_musb_init()` 鐨勭洰鏍囨槸鑾峰彇 MUSB 鎺у埗鍣ㄧ‖浠剁殑 transceiver
椹卞姩鏁版嵁锛屽苟鍍忓線甯镐竴鏍峰皢鍏朵紶閫掔粰 MUSB 鎺у埗鍣ㄩ┍鍔ㄣ€倀ransceiver 鏄?
鎺у埗鍣ㄧ‖浠跺唴閮ㄨ礋璐ｅ彂閫?鎺ユ敹 USB 鏁版嵁鐨勭數璺€傜敱浜庡畠鏄?OSI 妯″瀷
鐗╃悊灞傜殑瀹炵幇锛宼ransceiver 涔熷父琚О涓?PHY銆?

鑾峰彇 `MUSB PHY` 椹卞姩鏁版嵁鏄€氳繃 `usb_get_phy()` 瀹屾垚鐨勶紝瀹冭繑鍥炴寚鍚?
鍖呭惈椹卞姩瀹炰緥鏁版嵁鐨勭粨鏋勭殑鎸囬拡銆傛帴涓嬫潵鐨勫嚑鏉℃寚浠わ紙绗?12 琛屽拰绗?14 琛岋級
鍒嗗埆鐢ㄤ綔涓€涓?quirk 浠ュ強鐢ㄤ簬璁剧疆 IRQ 澶勭悊銆俀uirks 鍜?IRQ 澶勭悊灏?
鍦?musb-dev-quirks 涓◢鍚庤璁恒€?

```
    static int jz4740_musb_exit(struct musb *musb)
    {
	usb_put_phy(musb->xceiv);

	return 0;
    }
```

浣滀负 init 鐨勫搴旈儴鍒嗭紝exit 鍑芥暟鍦ㄦ帶鍒跺櫒纭欢鏈韩鍗冲皢琚噴鏀炬椂
閲婃斁 MUSB PHY 椹卞姩銆?

鍐嶆娉ㄦ剰锛岀敱浜?JZ4740 鎺у埗鍣ㄧ‖浠剁殑鐗规€ч泦杈冧负鍩虹锛宨nit 鍜?exit
鍦ㄦ澶勭浉褰撶畝鍗曘€傚湪涓烘洿澶嶆潅鐨勬帶鍒跺櫒纭欢缂栧啓 musb glue layer 鏃讹紝
浣犲彲鑳介渶瑕佸湪杩欎袱涓嚱鏁颁腑澶勭悊鏇村浜嬪姟銆?

浠?init 鍑芥暟杩斿洖鍚庯紝MUSB 鎺у埗鍣ㄩ┍鍔ㄨ烦鍥炲埌锛?

```
    static int jz4740_probe(struct platform_device *pdev)
    {
	ret = platform_device_add(musb);
	if (ret) {
	    dev_err(&pdev->dev, "failed to register musb device\n");
	    goto err_clk_disable;
	}

	return 0;

    err_clk_disable:
	clk_disable_unprepare(clk);
    err_platform_device_put:
	platform_device_put(musb);
	return ret;
    }
```

杩欐槸璁惧娉ㄥ唽杩囩▼鐨勬渶鍚庝竴閮ㄥ垎锛実lue layer 灏嗘帶鍒跺櫒纭欢璁惧娣诲姞鍒?
Linux 鍐呮牳璁惧灞傜骇缁撴瀯涓細鍦ㄦ闃舵锛屾墍鏈夊凡鐭ョ殑鍏充簬璇ヨ澶囩殑淇℃伅
閮借浼犻€掔粰 Linux USB core 鍗忚鏍堬細

   .. code-block:: c
    :emphasize-lines: 5,6

    static int jz4740_remove(struct platform_device *pdev)
    {
	struct jz4740_glue  *glue = platform_get_drvdata(pdev);

	platform_device_unregister(glue->musb);
	clk_disable_unprepare(glue->clk);

	return 0;
    }

浣滀负 probe 鐨勫搴旈儴鍒嗭紝remove 鍑芥暟娉ㄩ攢 MUSB 鎺у埗鍣ㄧ‖浠讹紙绗?5 琛岋級
骞剁鐢ㄦ椂閽燂紙绗?6 琛岋級锛屼娇鍏跺彲浠ヨ闂ㄦ帶鍏抽棴銆?

## 澶勭悊 IRQ

闄や簡 MUSB 鎺у埗鍣ㄧ‖浠剁殑鍩烘湰璁剧疆鍜屾敞鍐屼箣澶栵紝glue layer 杩樿礋璐ｅ鐞?
IRQ锛?

   .. code-block:: c
    :emphasize-lines: 7,9-11,14,24

    static irqreturn_t jz4740_musb_interrupt(int irq, void *__hci)
    {
	unsigned long   flags;
	irqreturn_t     retval = IRQ_NONE;
	struct musb     *musb = __hci;

	spin_lock_irqsave(&musb->lock, flags);

	musb->int_usb = musb_readb(musb->mregs, MUSB_INTRUSB);
	musb->int_tx = musb_readw(musb->mregs, MUSB_INTRTX);
	musb->int_rx = musb_readw(musb->mregs, MUSB_INTRRX);

	/*
  - The controller is gadget only, the state of the host mode IRQ bits is
  - undefined. Mask them to make sure that the musb driver core will
  - never see them set
	 */
	musb->int_usb &= MUSB_INTR_SUSPEND | MUSB_INTR_RESUME |
	    MUSB_INTR_RESET | MUSB_INTR_SOF;

	if (musb->int_usb || musb->int_tx || musb->int_rx)
	    retval = musb_interrupt(musb);

	spin_unlock_irqrestore(&musb->lock, flags);

	return retval;
    }

杩欓噷 glue layer 涓昏闇€瑕佽鍙栫浉鍏崇殑纭欢瀵勫瓨鍣紝骞跺皢鍏跺€间紶閫掔粰
鎺у埗鍣ㄩ┍鍔紝鐢辨帶鍒跺櫒椹卞姩鏉ュ鐞嗗疄闄呰Е鍙戣 IRQ 鐨勪簨浠躲€?

涓柇澶勭悊绋嬪簭鐨勫叧閿尯娈电敱 `spin_lock_irqsave` 鍙婂叾瀵瑰簲鍑芥暟
`spin_unlock_irqrestore`锛堝垎鍒槸绗?7 琛屽拰绗?24 琛岋級淇濇姢锛屽畠浠?
闃叉涓柇澶勭悊绋嬪簭浠ｇ爜琚袱涓笉鍚岀殑绾跨▼鍚屾椂杩愯銆?

闅忓悗璇诲彇鐩稿叧鐨勪腑鏂瘎瀛樺櫒锛堢 9 鑷?11 琛岋級锛?

- `MUSB_INTRUSB`锛氭寚绀哄綋鍓嶅摢浜?USB 涓柇澶勪簬婵€娲荤姸鎬侊紝

- `MUSB_INTRTX`锛氭寚绀哄綋鍓?TX 绔偣涓摢浜涗腑鏂浜庢縺娲荤姸鎬侊紝

- `MUSB_INTRRX`锛氭寚绀哄綋鍓?TX 绔偣涓摢浜涗腑鏂浜庢縺娲荤姸鎬併€?

娉ㄦ剰锛宍musb_readb` 鏈€澶氱敤浜庤鍙?8 浣嶅瘎瀛樺櫒锛岃€?`musb_readw` 鍏佽
鎴戜滑璇诲彇鏈€澶?16 浣嶇殑瀵勫瓨鍣ㄣ€傛牴鎹澶囧瘎瀛樺櫒澶у皬鐨勪笉鍚岋紝杩樺彲浠ヤ娇鐢?
鍏朵粬鍑芥暟銆傛洿澶氫俊鎭鍙傝 `musb_io.h`銆?

绗?18 琛岀殑鎸囦护鏄?JZ4740 USB 璁惧鎺у埗鍣ㄧ壒鏈夌殑鍙︿竴涓?quirk锛屽皢鍦?
musb-dev-quirks 涓◢鍚庤璁恒€?

涓嶈繃锛実lue layer 浠嶇劧闇€瑕佹敞鍐岃 IRQ 澶勭悊绋嬪簭銆傝繕璁板緱锛?

```
    static int jz4740_musb_init(struct musb *musb)
    {
	musb->isr = jz4740_musb_interrupt;

	return 0;
    }
```

璇ユ寚浠よ缃簡涓€涓寚鍚?glue layer IRQ 澶勭悊鍑芥暟鐨勬寚閽堬紝浠ヤ究褰?
鎺у埗鍣ㄧ‖浠朵骇鐢?IRQ 鏃舵帶鍒跺櫒纭欢鑳藉鍥炶皟璇ュ鐞嗙▼搴忋€備腑鏂鐞嗙▼搴?
鐜板凡瀹炵幇骞舵敞鍐屽畬鎴愩€?

## 璁惧 Platform Data

涓轰簡缂栧啓涓€涓?MUSB glue layer锛屼綘闇€瑕佹湁涓€浜涙弿杩版帶鍒跺櫒纭欢鑳藉姏鐨?
鏁版嵁锛岃繖琚О涓?platform data銆?

Platform data 鏄壒瀹氫簬浣犵殑纭欢鐨勶紝灏界瀹冨彲鑳戒細瑕嗙洊涓€澶х被璁惧锛?
骞朵笖閫氬父浣嶄簬 `arch/` 鐩綍涓殑鏌愪釜浣嶇疆锛屽叿浣撳彇鍐充簬浣犵殑璁惧鏋舵瀯銆?

渚嬪锛孞Z4740 SoC 鐨?platform data 浣嶄簬 `arch/mips/jz4740/platform.c`銆?
鍦?`platform.c` 鏂囦欢涓紝JZ4740 SoC 鐨勬瘡涓澶囬兘閫氳繃涓€缁勭粨鏋勬潵鎻忚堪銆?

浠ヤ笅鏄?`arch/mips/jz4740/platform.c` 涓鐩?USB Device Controller (UDC)
鐨勯儴鍒嗭細

   .. code-block:: c
    :emphasize-lines: 2,7,14-17,21,22,25,26,28,29

    /** USB Device Controller **/
    struct platform_device jz4740_udc_xceiv_device = {
	.name = "usb_phy_gen_xceiv",
	.id   = 0,
    };

    static struct resource jz4740_udc_resources[] = {
	[^0^] = {
	    .start = JZ4740_UDC_BASE_ADDR,
	    .end   = JZ4740_UDC_BASE_ADDR + 0x10000 - 1,
	    .flags = IORESOURCE_MEM,
	},
	[^1^] = {
	    .start = JZ4740_IRQ_UDC,
	    .end   = JZ4740_IRQ_UDC,
	    .flags = IORESOURCE_IRQ,
	    .name  = "mc",
	},
    };

    struct platform_device jz4740_udc_device = {
	.name = "musb-jz4740",
	.id   = -1,
	.dev  = {
	    .dma_mask          = &jz4740_udc_device.dev.coherent_dma_mask,
	    .coherent_dma_mask = DMA_BIT_MASK(32),
	},
	.num_resources = ARRAY_SIZE(jz4740_udc_resources),
	.resource      = jz4740_udc_resources,
    };

`jz4740_udc_xceiv_device` platform device 缁撴瀯锛堢 2 琛岋級閫氳繃鍚嶇О鍜?
id 鍙锋弿杩颁簡 UDC transceiver銆?

鍦ㄦ挵鍐欐湰鏂囨椂锛岃娉ㄦ剰 `usb_phy_gen_xceiv` 鏄敤浜庢墍鏈夊唴缃湪鍙傝€?USB
IP 涓€佹垨鑷富涓斾笉闇€瑕佷换浣?PHY 缂栫▼鐨?transceiver 鐨勪笓鐢ㄥ悕绉般€備綘闇€瑕?
鍦ㄥ唴鏍搁厤缃腑璁剧疆 `CONFIG_NOP_USB_XCEIV=y` 鎵嶈兘浣跨敤鐩稿簲鐨?transceiver
椹卞姩銆俰d 瀛楁鍙互璁剧疆涓?-1锛堢浉褰撲簬 `PLATFORM_DEVID_NONE`锛夈€?2锛堢浉褰撲簬
`PLATFORM_DEVID_AUTO`锛夛紝鎴栬€呭鏋滆鎸囧畾 id 鍙凤紝鍒欎粠 0 寮€濮嬩綔涓烘绫?
璁惧鐨勭涓€涓澶囥€?

`jz4740_udc_resources` 璧勬簮缁撴瀯锛堢 7 琛岋級瀹氫箟浜?UDC 瀵勫瓨鍣ㄥ熀鍦板潃銆?

绗竴涓暟缁勶紙绗?9 鑷?11 琛岋級瀹氫箟浜?UDC 瀵勫瓨鍣ㄥ熀鍦板潃鍐呭瓨鍦板潃锛歴tart
鎸囧悜绗竴涓瘎瀛樺櫒鍐呭瓨鍦板潃锛宔nd 鎸囧悜鏈€鍚庝竴涓瘎瀛樺櫒鍐呭瓨鍦板潃锛岃€?flags
鎴愬憳瀹氫箟浜嗘垜浠墍澶勭悊璧勬簮鐨勭被鍨嬨€傚洜姝?`IORESOURCE_MEM` 鐢ㄤ簬瀹氫箟
瀵勫瓨鍣ㄥ唴瀛樺湴鍧€銆傜浜屼釜鏁扮粍锛堢 14 鑷?17 琛岋級瀹氫箟浜?UDC IRQ 瀵勫瓨鍣?
鍦板潃銆傜敱浜?JZ4740 UDC 鍙湁涓€涓彲鐢ㄧ殑 IRQ 瀵勫瓨鍣紝start 鍜?end 鎸囧悜
鐩稿悓鐨勫湴鍧€銆俙IORESOURCE_IRQ` 鏍囧織琛ㄦ槑鎴戜滑澶勭悊鐨勬槸 IRQ 璧勬簮锛岃€屽悕绉?
`mc` 瀹為檯涓婃槸纭紪鐮佸湪 MUSB core 涓殑锛屼互渚挎帶鍒跺櫒椹卞姩鑳藉閫氳繃鎸夊悕绉?
鏌ヨ鏉ヨ幏鍙栬繖涓?IRQ 璧勬簮銆?

鏈€鍚庯紝`jz4740_udc_device` platform device 缁撴瀯锛堢 21 琛岋級鎻忚堪浜?
UDC 鏈韩銆?

`musb-jz4740` 鍚嶇О锛堢 22 琛岋級瀹氫箟浜嗙敤浜庤璁惧鐨?MUSB 椹卞姩锛涜璁颁綇
杩欏疄闄呬笂姝ｆ槸鎴戜滑鍦?musb-basics 涓?`jz4740_driver` platform driver
缁撴瀯閲屼娇鐢ㄧ殑鍚嶇О銆俰d 瀛楁锛堢 23 琛岋級璁句负 -1锛堢浉褰撲簬
`PLATFORM_DEVID_NONE`锛夛紝鍥犱负鎴戜滑涓嶉渶瑕佷负璁惧鎸囧畾 id锛歁USB 鎺у埗鍣?
椹卞姩宸茬粡鍦?musb-basics 涓涓哄垎閰嶈嚜鍔?id 浜嗐€傚湪 dev 瀛楁涓紝鎴戜滑
鍦ㄦ鍏虫敞 DMA 鐩稿叧淇℃伅銆俙dma_mask` 瀛楁锛堢 25 琛岋級瀹氫箟浜嗗皢瑕佷娇鐢ㄧ殑
DMA 鎺╃爜瀹藉害锛岃€?`coherent_dma_mask`锛堢 26 琛岋級鐢ㄩ€旂浉鍚岋紝浣嗛拡瀵?
`alloc_coherent` DMA 鏄犲皠锛氬湪杩欎袱绉嶆儏鍐典笅鎴戜滑閮戒娇鐢ㄤ竴涓?32 浣嶇殑鎺╃爜銆?
鐒跺悗 resource 瀛楁锛堢 29 琛岋級鍙槸涓€涓寚鍚戜箣鍓嶅畾涔夌殑璧勬簮缁撴瀯鐨勬寚閽堬紝
鑰?`num_resources` 瀛楁锛堢 28 琛岋級璁板綍浜嗚祫婧愮粨鏋勪腑瀹氫箟鐨勬暟缁勬暟閲?
锛堟湰渚嬩腑鏈変袱涓祫婧愭暟缁勮瀹氫箟锛夈€?

鍦?`arch/` 灞傞潰瀵?UDC platform data 鐨勭畝瑕佹瑙堝埌姝ょ粨鏉燂紝璁╂垜浠洖鍒?
`drivers/usb/musb/jz4740.c` 涓?MUSB glue layer 鐗瑰畾鐨?platform data锛?

   .. code-block:: c
    :emphasize-lines: 3,5,7-9,11

    static struct musb_hdrc_config jz4740_musb_config = {
	/** Silicon does not implement USB OTG. **/
	.multipoint = 0,
	/** Max EPs scanned, driver will decide which EP can be used. **/
	.num_eps    = 4,
	/** RAMbits needed to configure EPs from table **/
	.ram_bits   = 9,
	.fifo_cfg = jz4740_musb_fifo_cfg,
	.fifo_cfg_size = ARRAY_SIZE(jz4740_musb_fifo_cfg),
    };

    static struct musb_hdrc_platform_data jz4740_musb_platform_data = {
	.mode   = MUSB_PERIPHERAL,
	.config = &jz4740_musb_config,
    };

棣栧厛锛実lue layer 閰嶇疆鎺у埗鍣ㄩ┍鍔ㄦ搷浣滀腑涓庢帶鍒跺櫒纭欢鐗瑰畾鐩稿叧鐨勪竴浜?
鏂归潰銆傝繖鏄€氳繃 `jz4740_musb_config` `musb_hdrc_config` 缁撴瀯瀹屾垚鐨勩€?

瀹氫箟鎺у埗鍣ㄧ‖浠剁殑 OTG 鑳藉姏鏃讹紝multipoint 鎴愬憳锛堢 3 琛岋級璁句负 0
锛堢浉褰撲簬 false锛夛紝鍥犱负 JZ4740 UDC 涓嶅吋瀹?OTG銆傛帴鐫€ `num_eps`锛堢 5 琛岋級
瀹氫箟浜嗘帶鍒跺櫒纭欢鐨?USB 绔偣鏁伴噺锛屽寘鎷鐐?0锛氳繖閲屾垜浠湁 3 涓鐐瑰姞
绔偣 0銆傛帴涓嬫潵鏄?`ram_bits`锛堢 7 琛岋級锛屽畠鏄?MUSB 鎺у埗鍣ㄧ‖浠剁殑 RAM
鍦板潃鎬荤嚎瀹藉害銆傚綋鎺у埗鍣ㄩ┍鍔ㄦ棤娉曢€氳繃璇诲彇鐩稿叧鎺у埗鍣ㄧ‖浠跺瘎瀛樺櫒鏉ヨ嚜鍔?
閰嶇疆绔偣鏃讹紝灏遍渶瑕佽繖涓俊鎭€傝繖涓棶棰樺皢鍦ㄦ垜浠湪 musb-dev-quirks 涓?
璁ㄨ璁惧 quirks 鏃舵彁鍙娿€傛渶鍚庝袱涓瓧娈碉紙绗?8 琛屽拰绗?9 琛岋級涔熷叧涔庤澶?
quirks锛歚fifo_cfg` 鎸囧悜 USB 绔偣閰嶇疆琛紝`fifo_cfg_size` 璁板綍璇ラ厤缃〃
涓殑鏉＄洰鏁伴噺銆傛洿澶氬唴瀹瑰皢鍦?musb-dev-quirks 涓粙缁嶃€?

闅忓悗璇ラ厤缃宓屽叆鍒?`jz4740_musb_platform_data` `musb_hdrc_platform_data`
缁撴瀯锛堢 11 琛岋級涓細config 鏄寚鍚戦厤缃粨鏋勬湰韬殑鎸囬拡锛岃€?mode 鍛婅瘔
鎺у埗鍣ㄩ┍鍔ㄨ鎺у埗鍣ㄧ‖浠舵槸浠呭彲鐢ㄤ綔 `MUSB_HOST`銆佷粎鍙敤浣?
`MUSB_PERIPHERAL`锛岃繕鏄彲鐢ㄤ綔鍙屾ā寮忕殑 `MUSB_OTG`銆?

璇疯浣忥紝`jz4740_musb_platform_data` 闅忓悗琚敤鏉ヤ紶閫?platform data 淇℃伅锛?
姝ｅ鎴戜滑鍦?musb-basics 鐨?probe 鍑芥暟涓墍鐪嬪埌鐨勯偅鏍枫€?

## 璁惧 Quirks

鍦ㄥ畬鍠勭壒瀹氫簬浣犺澶囩殑 platform data 鏃讹紝浣犲彲鑳借繕闇€瑕佸湪 glue layer 涓?
缂栧啓涓€浜涗唬鐮侊紝浠ヨ閬挎煇浜涜澶囩壒瀹氱殑闄愬埗銆傝繖浜?quirks 鍙兘鏄敱鏌愪簺
纭欢缂洪櫡寮曡捣鐨勶紝鎴栬€呬粎浠呮槸 USB On-the-Go 瑙勮寖瀹炵幇涓嶅畬鏁寸殑鍚庢灉銆?

JZ4740 UDC 灏辫〃鐜板嚭杩欐牱鐨?quirks锛屽叾涓竴浜涙垜浠皢鍦ㄦ璁ㄨ锛屼互澧炶繘
浜嗚В锛屽敖绠¤繖浜涘彲鑳藉湪浣犳鍦ㄤ娇鐢ㄧ殑鎺у埗鍣ㄧ‖浠朵腑骞朵笉瀛樺湪銆?

璁╂垜浠厛鍥炲埌 init 鍑芥暟锛?

   .. code-block:: c
    :emphasize-lines: 12

    static int jz4740_musb_init(struct musb *musb)
    {
	musb->xceiv = usb_get_phy(USB_PHY_TYPE_USB2);
	if (!musb->xceiv) {
	    pr_err("HS UDC: no transceiver configured\n");
	    return -ENODEV;
	}

	/* Silicon does not implement ConfigData register.
  - Set dyn_fifo to avoid reading EP config from hardware.
	 */
	musb->dyn_fifo = true;

	musb->isr = jz4740_musb_interrupt;

	return 0;
    }

绗?12 琛岀殑鎸囦护甯姪 MUSB 鎺у埗鍣ㄩ┍鍔ㄨ閬夸簡杩欐牱涓€涓簨瀹烇細鎺у埗鍣ㄧ‖浠?
缂哄皯鐢ㄤ簬 USB 绔偣閰嶇疆鐨勫瘎瀛樺櫒銆?

濡傛灉娌℃湁杩欎簺瀵勫瓨鍣紝鎺у埗鍣ㄩ┍鍔ㄥ氨鏃犳硶浠庣‖浠惰鍙栫鐐归厤缃紝鍥犳鎴戜滑
浣跨敤绗?12 琛岀殑鎸囦护缁曡繃浠庣鐗囪鍙栭厤缃紝杞€屼緷璧栦竴涓厤缃〃锛?

```
    static const struct musb_fifo_cfg jz4740_musb_fifo_cfg[] = {
	{ .hw_ep_num = 1, .style = FIFO_TX, .maxpacket = 512, },
	{ .hw_ep_num = 1, .style = FIFO_RX, .maxpacket = 512, },
	{ .hw_ep_num = 2, .style = FIFO_TX, .maxpacket = 64, },
    };
```

鏌ョ湅涓婇潰鐨勯厤缃〃锛屾垜浠湅鍒版瘡涓鐐圭敱涓変釜瀛楁鎻忚堪锛歚hw_ep_num` 鏄?
绔偣鍙凤紝style 鏄叾鏂瑰悜锛堣涔堟槸 `FIFO_TX`锛岃〃绀虹敱鎺у埗鍣ㄩ┍鍔ㄥ悜鎺у埗鍣?
纭欢鍙戦€佹暟鎹寘锛涜涔堟槸 `FIFO_RX`锛岃〃绀轰粠纭欢鎺ユ敹鏁版嵁鍖咃級锛岃€?
maxpacket 瀹氫箟浜嗚绔偣涓婂彲浼犺緭鐨勬瘡涓暟鎹寘鐨勬渶澶у昂瀵搞€備粠琛ㄤ腑璇诲彇
鍙煡锛岀鐐?1 鍙敤浜庝竴娆℃€у彂閫佸拰鎺ユ敹 512 瀛楄妭鐨?USB 鏁版嵁鍖咃紙杩欏疄闄呬笂
鏄竴涓壒閲?in/out 绔偣锛夛紝绔偣 2 鍙敤浜庝竴娆℃€у彂閫?64 瀛楄妭鐨勬暟鎹寘
锛堣繖瀹為檯涓婃槸涓€涓腑鏂鐐癸級銆?

娉ㄦ剰锛岃繖閲屾病鏈夊叧浜庣鐐?0 鐨勪俊鎭細绔偣 0 鍦ㄦ瘡涓鐗囪璁′腑閮芥槸榛樿
瀹炵幇鐨勶紝骞舵寜鐓?USB 瑙勮寖鍏锋湁棰勫畾涔夌殑閰嶇疆銆傛洿澶氱鐐归厤缃〃鐨勭ず渚嬭
鍙傝 `musb_core.c`銆?

鐜板湪璁╂垜浠洖鍒颁腑鏂鐞嗗嚱鏁帮細

   .. code-block:: c
    :emphasize-lines: 18-19

    static irqreturn_t jz4740_musb_interrupt(int irq, void *__hci)
    {
	unsigned long   flags;
	irqreturn_t     retval = IRQ_NONE;
	struct musb     *musb = __hci;

	spin_lock_irqsave(&musb->lock, flags);

	musb->int_usb = musb_readb(musb->mregs, MUSB_INTRUSB);
	musb->int_tx = musb_readw(musb->mregs, MUSB_INTRTX);
	musb->int_rx = musb_readw(musb->mregs, MUSB_INTRRX);

	/*
  - The controller is gadget only, the state of the host mode IRQ bits is
  - undefined. Mask them to make sure that the musb driver core will
  - never see them set
	 */
	musb->int_usb &= MUSB_INTR_SUSPEND | MUSB_INTR_RESUME |
	    MUSB_INTR_RESET | MUSB_INTR_SOF;

	if (musb->int_usb || musb->int_tx || musb->int_rx)
	    retval = musb_interrupt(musb);

	spin_unlock_irqrestore(&musb->lock, flags);

	return retval;
    }

涓婇潰绗?18 琛岀殑鎸囦护鏄帶鍒跺櫒椹卞姩瑙勯伩杩欐牱涓€涓簨瀹炵殑涓€绉嶆柟寮忥細鐢ㄤ簬 USB
涓绘満妯″紡鎿嶄綔鐨勬煇浜涗腑鏂綅鍦?`MUSB_INTRUSB` 瀵勫瓨鍣ㄤ腑缂哄け锛屽洜姝ゅ浜?
鏈畾涔夌殑纭欢鐘舵€侊紝鍥犱负姝?MUSB 鎺у埗鍣ㄧ‖浠朵粎鐢ㄤ簬澶栬妯″紡銆傚洜姝わ紝glue
layer 閫氳繃瀵逛粠 `MUSB_INTRUSB` 璇诲彇鐨勫€间笌瀵勫瓨鍣ㄤ腑瀹為檯瀹炵幇鐨勪綅杩涜
閫昏緫 AND 鎿嶄綔锛屽皢杩欎簺缂哄け鐨勪綅灞忚斀鎺夛紝浠ラ伩鍏嶄骇鐢熷瘎鐢熶腑鏂€?

杩欎簺鍙槸 JZ4740 USB 璁惧鎺у埗鍣ㄤ腑鍙戠幇鐨勫皯鏁板嚑涓?quirks銆傚叾浠栦竴浜?
鍒欑洿鎺ュ湪 MUSB core 涓緱鍒板鐞嗭紝鍥犱负閭ｄ簺淇瓒冲閫氱敤锛岃兘澶熶负鏈€缁?
鍏朵粬鎺у埗鍣ㄧ‖浠舵洿濂藉湴澶勭悊闂銆?

## 缁撹

缂栧啓 Linux MUSB glue layer 搴斿綋鏄竴椤规洿鏄撲笂鎵嬬殑浠诲姟鏄紝鍥犱负鏈枃妗?
璇曞浘灞曠ず杩欓」缁冧範鐨勬潵榫欏幓鑴夈€?

JZ4740 USB 璁惧鎺у埗鍣ㄧ浉褰撶畝鍗曪紝鎴戝笇鏈涘畠鐨?glue layer 鑳戒綔涓轰竴涓?
濂戒緥瀛愪緵濂藉鑰呭弬鑰冦€傜粨鍚堝綋鍓嶇殑 MUSB glue layer 涓€璧蜂娇鐢紝鏈枃妗?
搴斿綋鑳芥彁渚涜冻澶熺殑鍏ラ棬鎸囧锛涗竾涓€浜嬫儏澶辨帶锛宭inux-usb 閭欢鍒楄〃褰掓。
鏄彟涓€涓彲渚涙煡闃呯殑鏈夌敤璧勬簮銆?

## 鑷磋阿

闈炲父鎰熻阿 Lars-Peter Clausen 鍜?Maarten ter Huurne锛屼粬浠湪鎾板啓
JZ4740 glue layer 鏈熼棿鍥炵瓟浜嗘垜鐨勯棶棰橈紝骞跺府鍔╂垜灏嗕唬鐮佹暣鐞嗗緱浜曚簳鏈夋潯銆?

鎴戣繕瑕佹劅璋㈡暣涓?Qi-Hardware 绀惧尯鎵€缁欎簣鐨勬剦蹇寚瀵间笌鏀寔銆?

## 璧勬簮

USB Home Page: https://www.usb.org

linux-usb Mailing List Archives: https://lore.kernel.org/linux-usb

USB On-the-Go Basics:
https://www.maximintegrated.com/app-notes/index.mvp/id/1822

Writing USB Device Drivers <writing-usb-driver>

Texas Instruments USB Configuration Wiki Page:
https://web.archive.org/web/20201215135015/http://processors.wiki.ti.com/index.php/Usbgeneralpage
