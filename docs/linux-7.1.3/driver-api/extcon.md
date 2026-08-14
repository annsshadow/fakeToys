## Extcon 璁惧瀛愮郴缁?

## 姒傝堪


Extcon锛圗xternal Connector锛屽閮ㄨ繛鎺ュ櫒锛夊瓙绯荤粺鎻愪緵浜嗕竴涓粺涓€鐨勬鏋讹紝鐢ㄤ簬绠＄悊
Linux 绯荤粺涓殑澶栭儴杩炴帴鍣ㄣ€傚畠鍏佽椹卞姩鎶ュ憡澶栭儴杩炴帴鍣ㄧ殑鐘舵€侊紝骞朵负鐢ㄦ埛绌洪棿鎻愪緵
鏍囧噯鍖栫殑鎺ュ彛鏉ユ煡璇㈠拰鐩戞帶杩欎簺鐘舵€併€?
Extcon 鍦ㄥ叿鏈夊绉嶈繛鎺ラ€夐」鐨勭幇浠ｈ澶囷紙渚嬪鏅鸿兘鎵嬫満銆佸钩鏉跨數鑴戝拰绗旇鏈數鑴戯級涓?灏ゅ叾鏈夌敤銆傚畠鏈夊姪浜庣鐞嗗悇绉嶇被鍨嬬殑杩炴帴鍣紝鍖呮嫭锛?
1. USB 杩炴帴鍣紙渚嬪 USB-C銆乵icro-USB锛?2. 鍏呯數绔彛锛堜緥濡傚揩鍏呫€佹棤绾垮厖鐢碉級
3. 闊抽鎻掑瓟锛堜緥濡?3.5mm 鑰虫満鎻掑瓟锛?4. 瑙嗛杈撳嚭锛堜緥濡?HDMI銆丏isplayPort锛?5. 鎵╁睍鍧?
鐪熷疄涓栫晫鐨勪緥瀛愶細

1. 鏅鸿兘鎵嬫満 USB-C 绔彛锛?   鏅鸿兘鎵嬫満涓婂崟涓?USB-C 绔彛鍙互鎻愪緵澶氱鍔熻兘銆侲xtcon 鍙互绠＄悊璇ョ鍙ｇ殑涓嶅悓鐘舵€侊紝
   渚嬪锛?   - USB 鏁版嵁杩炴帴
   - 鍏呯數锛堝悇绉嶇被鍨嬶紝濡傚揩鍏呫€乁SB Power Delivery锛?   - 闊抽杈撳嚭锛圲SB-C 鑰虫満锛?   - 瑙嗛杈撳嚭锛圲SB-C 杞?HDMI 閫傞厤鍣級

2. 绗旇鏈數鑴戞墿灞曞潪锛?   褰撶瑪璁版湰鐢佃剳杩炴帴鍒版墿灞曞潪鏃讹紝浼氬悓鏃惰繘琛屽涓繛鎺ャ€侲xtcon 鍙互澶勭悊浠ヤ笅鐘舵€佺殑
   鍙樺寲锛?   - 鐢靛姏浼犺緭
   - 澶栭儴鏄剧ず鍣?   - USB 闆嗙嚎鍣ㄨ繛鎺?   - 浠ュお缃戣繛鎺?
3. 鏃犵嚎鍏呯數鏉匡細
   Extcon 鍙互绠＄悊鏃犵嚎鍏呯數杩炴帴鐨勭姸鎬侊紝浣跨郴缁熻兘澶熷湪璁惧鏀句笂鎴栫Щ寮€鍏呯數鏉挎椂鍋氬嚭
   鎭板綋鐨勫搷搴斻€?
4. 鏅鸿兘鐢佃 HDMI 绔彛锛?   鍦ㄦ櫤鑳界數瑙嗕腑锛孍xtcon 鍙互绠＄悊澶氫釜 HDMI 绔彛锛屾娴嬭澶囦綍鏃惰繛鎺ユ垨鏂紑锛屽苟鍙兘
   璇嗗埆璁惧鐨勭被鍨嬶紙渚嬪娓告垙鏈恒€佹満椤剁洅銆佽摑鍏夋挱鏀惧櫒锛夈€?
Extcon 妗嗘灦閫氳繃鎻愪緵鏍囧噯鍖栫殑鏂瑰紡鏉ユ姤鍛婁笌鏌ヨ杩炴帴鍣ㄧ姸鎬併€佸鐞嗕簰鏂ヨ繛鎺ヤ互鍙婄鐞?杩炴帴鍣ㄥ睘鎬э紝绠€鍖栦簡杩欎簺澶嶆潅鍦烘櫙鐨勯┍鍔ㄥ紑鍙戙€傝繖浣垮緱鐜颁唬璁惧涓澶栭儴杩炴帴鐨勫鐞嗘洿鍔?绋冲仴鍜岀伒娲汇€?
## 鍏抽敭缁勪欢


### extcon_dev


```

    struct extcon_dev {
        const char *name;
        const unsigned int *supported_cable;
        const u32 *mutually_exclusive;

        /* Internal data */
        struct device dev;
        unsigned int id;
        struct raw_notifier_head nh_all;
        struct raw_notifier_head *nh;
        struct list_head entry;
        int max_supported;
        spinlock_t lock;
        u32 state;

        /* Sysfs related */
        struct device_type extcon_dev_type;
        struct extcon_cable *cables;
        struct attribute_group attr_g_muex;
        struct attribute **attrs_muex;
        struct device_attribute *d_attrs_muex;
    };

```
鍏抽敭瀛楁锛?
- `name`锛欵xtcon 璁惧鐨勫悕绉?- `supported_cable`锛氭敮鎸佺殑绾跨紗绫诲瀷鏁扮粍
- `mutually_exclusive`锛氬畾涔変簰鏂ョ嚎缂嗙被鍨嬬殑鏁扮粍
  璇ュ瓧娈靛浜庡己鍒跺疄鏂界‖浠剁害鏉熻嚦鍏抽噸瑕併€傚畠鏄竴涓?32 浣嶆棤绗﹀彿鏁存暟鏁扮粍锛屽叾涓瘡涓?  鍏冪礌浠ｈ〃涓€缁勪簰鏂ョ殑绾跨紗绫诲瀷銆傝鏁扮粍搴斾互 0 缁撳熬銆?
  渚嬪锛?
```

      static const u32 mutually_exclusive[] = {
          BIT(0) | BIT(1),  /* Cable 0 and 1 are mutually exclusive */
          BIT(2) | BIT(3) | BIT(4),  /* Cables 2, 3, and 4 are mutually exclusive */
          0  /* Terminator */
      };

  鍦ㄦ绀轰緥涓紝绾跨紗 0 鍜?1 涓嶈兘鍚屾椂杩炴帴锛岀嚎缂?2銆? 鍜?4 涔熸槸浜掓枼鐨勩€傝繖瀵逛簬璇稿
  鍗曚釜绔彛鏃㈠彲浠ユ槸 USB 涔熷彲浠ユ槸 HDMI銆佷絾涓嶈兘鍚屾椂鏄袱鑰呰繖鏍风殑鍦烘櫙寰堟湁鐢ㄣ€?
  Extcon 鏍稿績鍒╃敤杩欎簺淇℃伅鏉ラ槻姝㈡棤鏁堢殑绾跨紗鐘舵€佺粍鍚堬紝纭繚鎶ュ憡鐨勭姸鎬佸缁堜笌纭欢
  鑳藉姏涓€鑷淬€?
```
- `state`锛氳澶囩殑褰撳墠鐘舵€侊紙宸茶繛鎺ョ嚎缂嗙殑浣嶅浘锛?

### extcon_cable


```

    struct extcon_cable {
        struct extcon_dev *edev;
        int cable_index;
        struct attribute_group attr_g;
        struct device_attribute attr_name;
        struct device_attribute attr_state;
        struct attribute *attrs[3];
        union extcon_property_value usb_propval[EXTCON_PROP_USB_CNT];
        union extcon_property_value chg_propval[EXTCON_PROP_CHG_CNT];
        union extcon_property_value jack_propval[EXTCON_PROP_JACK_CNT];
        union extcon_property_value disp_propval[EXTCON_PROP_DISP_CNT];
        DECLARE_BITMAP(usb_bits, EXTCON_PROP_USB_CNT);
        DECLARE_BITMAP(chg_bits, EXTCON_PROP_CHG_CNT);
        DECLARE_BITMAP(jack_bits, EXTCON_PROP_JACK_CNT);
        DECLARE_BITMAP(disp_bits, EXTCON_PROP_DISP_CNT);
    };

```
## 鏍稿績鍑芥暟


   :identifiers: extcon_get_state

   :identifiers: extcon_set_state

   :identifiers: extcon_set_state_sync

   :identifiers: extcon_get_property


## Sysfs 鎺ュ彛


Extcon 璁惧鏆撮湶浠ヤ笅 sysfs 灞炴€э細

- `name`锛欵xtcon 璁惧鐨勫悕绉?- `state`锛氭墍鏈夊彈鏀寔绾跨紗鐨勫綋鍓嶇姸鎬?- `cable.N/name`锛氱 N 涓彈鏀寔绾跨紗鐨勫悕绉?- `cable.N/state`锛氱 N 涓彈鏀寔绾跨紗鐨勭姸鎬?
### 浣跨敤绀轰緥


    #include <linux/module.h>
    #include <linux/platform_device.h>
    #include <linux/extcon.h>

    struct my_extcon_data {
        struct extcon_dev *edev;
        struct device *dev;
    };

    static const unsigned int my_extcon_cable[] = {
        EXTCON_USB,
        EXTCON_USB_HOST,
        EXTCON_NONE,
    };

    static int my_extcon_probe(struct platform_device *pdev)
    {
        struct my_extcon_data *data;
        int ret;

        data = devm_kzalloc(&pdev->dev, sizeof(*data), GFP_KERNEL);
        if (!data)
            return -ENOMEM;

        data->dev = &pdev->dev;

        /** Initialize extcon device **/
        data->edev = devm_extcon_dev_allocate(data->dev, my_extcon_cable);
        if (IS_ERR(data->edev)) {
            dev_err(data->dev, "Failed to allocate extcon device\n");
            return PTR_ERR(data->edev);
        }

        /** Register extcon device **/
        ret = devm_extcon_dev_register(data->dev, data->edev);
        if (ret < 0) {
            dev_err(data->dev, "Failed to register extcon device\n");
            return ret;
        }

        platform_set_drvdata(pdev, data);

        /** Example: Set initial state **/
        extcon_set_state_sync(data->edev, EXTCON_USB, true);

        dev_info(data->dev, "My extcon driver probed successfully\n");
        return 0;
    }

    static int my_extcon_remove(struct platform_device *pdev)
    {
        struct my_extcon_data *data = platform_get_drvdata(pdev);

        /** Example: Clear state before removal **/
        extcon_set_state_sync(data->edev, EXTCON_USB, false);

        dev_info(data->dev, "My extcon driver removed\n");
        return 0;
    }

    static const struct of_device_id my_extcon_of_match[] = {
        { .compatible = "my,extcon-device", },
        { },
    };
    MODULE_DEVICE_TABLE(of, my_extcon_of_match);

    static struct platform_driver my_extcon_driver = {
        .driver = {
            .name = "my-extcon-driver",
            .of_match_table = my_extcon_of_match,
        },
        .probe = my_extcon_probe,
        .remove = my_extcon_remove,
    };

    module_platform_driver(my_extcon_driver);

### 姝ょず渚嬫紨绀轰簡锛?

- 瀹氫箟鍙楁敮鎸佺殑绾跨紗绫诲瀷锛堟湰渚嬩腑涓?USB 鍜?USB Host锛夈€?- 鍒嗛厤骞舵敞鍐屼竴涓?extcon 璁惧銆?- 涓虹嚎缂嗚缃垵濮嬬姸鎬侊紙鏈緥涓负 USB 宸茶繛鎺ワ級銆?- 鍦ㄩ┍鍔ㄨ绉婚櫎鏃舵竻闄ょ姸鎬併€?