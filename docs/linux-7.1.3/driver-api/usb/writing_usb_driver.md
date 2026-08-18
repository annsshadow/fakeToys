
## 缂栧啓 USB 璁惧椹卞姩


:Author: Greg Kroah-Hartman

## 绠€浠?

Linux USB 瀛愮郴缁熷凡缁忎粠 2.2.7 鍐呮牳涓粎鏀寔涓ょ涓嶅悓绫诲瀷鐨勮澶囷紙榧犳爣鍜岄敭鐩橈級锛屽彂灞曞埌 2.4 鍐呮牳涓敮鎸?20 澶氱涓嶅悓绫诲瀷鐨勮澶囥€侺inux 鐩墠鏀寔鍑犱箮鎵€鏈夌殑 USB 绫昏澶囷紙閿洏銆侀紶鏍囥€佽皟鍒惰В璋冨櫒銆佹墦鍗版満鍜岄煶绠辩瓑鏍囧噯绫诲瀷璁惧锛夛紝浠ュ強鏁伴噺涓嶆柇澧為暱鐨勫巶鍟嗙壒瀹氳澶囷紙濡?USB 杞覆鍙ｈ浆鎹㈠櫒銆佹暟鐮佺浉鏈恒€佷互澶綉璁惧鍜?MP3 鎾斁鍣級銆傛湁鍏冲綋鍓嶆敮鎸佺殑鍚勭被 USB 璁惧鐨勫畬鏁村垪琛紝璇峰弬闃呰祫婧愶紙Resources锛夈€?
鍓╀綑閭ｄ簺鍦?Linux 涓婃病鏈夋敮鎸佺殑 USB 璁惧锛屽嚑涔庨兘鏄巶鍟嗙壒瀹氱殑璁惧銆傛瘡涓巶鍟嗛兘鍐冲畾瀹炵幇鑷畾涔夌殑鍗忚鏉ヤ笌瀹冧滑鐨勮澶囬€氫俊锛屽洜姝ら€氬父闇€瑕佸垱寤轰竴涓嚜瀹氫箟椹卞姩銆備竴浜涘巶鍟嗗鍏?USB 鍗忚鎸佸紑鏀炬€佸害锛屽苟鍗忓姪鍒涘缓 Linux 椹卞姩锛岃€屽彟涓€浜涘巶鍟嗗垯涓嶅叕甯冨畠浠紝寮€鍙戣€呰杩繘琛岄€嗗悜宸ョ▼銆傛湁鍏充竴浜涙柟渚跨殑閫嗗悜宸ョ▼宸ュ叿鐨勯摼鎺ワ紝璇峰弬闃呰祫婧愶紙Resources锛夈€?
鐢变簬姣忎竴绉嶄笉鍚岀殑鍗忚閮戒細瀵艰嚧鍒涘缓涓€涓柊椹卞姩锛屾垜缂栧啓浜嗕竴涓€氱敤鐨?USB 椹卞姩妗嗘灦锛坰keleton锛夛紝瀹冧豢鐓у唴鏍告簮鐮佹爲涓殑 pci-skeleton.c 鏂囦欢锛岃澶?PCI 缃戠粶椹卞姩閮藉熀浜庤鏂囦欢銆傝繖涓?USB 妗嗘灦鍙互鍦ㄥ唴鏍告簮鐮佹爲鐨?drivers/usb/usb-skeleton.c 涓壘鍒般€傚湪鏈枃涓紝鎴戝皢閫愭璁茶В璇ユ鏋堕┍鍔ㄧ殑鍩烘湰缁撴瀯锛岃В閲婂叾涓殑鍚勪釜閮ㄥ垎锛屼互鍙婇渶瑕佸仛浠€涔堟潵閽堝浣犵殑鐗瑰畾璁惧杩涜瀹氬埗銆?

## Linux USB 鍩虹


濡傛灉浣犺缂栧啓涓€涓?Linux USB 椹卞姩锛岃鍏堢啛鎮?USB 鍗忚瑙勮寖銆傚畠锛岃繛鍚岃澶氬叾瀹冩湁鐢ㄧ殑鏂囨。锛屽彲浠ュ湪 USB 涓婚〉锛堝弬瑙佽祫婧愶級鎵惧埌銆備竴绡囦粙缁?Linux USB 瀛愮郴缁熺殑浼樼鏂囩珷鍙互鍦?USB 宸ヤ綔璁惧鍒楄〃锛堝弬瑙佽祫婧愶級鎵惧埌銆傚畠瑙ｉ噴浜?Linux USB 瀛愮郴缁熸槸濡備綍缁勭粐鐨勶紝骞跺悜璇昏€呬粙缁嶄簡 USB urb锛圲SB Request Block锛孶SB 璇锋眰鍧楋級鐨勬蹇碉紝杩欏 USB 椹卞姩鑷冲叧閲嶈銆?
Linux USB 椹卞姩闇€瑕佸仛鐨勭涓€浠朵簨鏄悜 Linux USB 瀛愮郴缁熸敞鍐岃嚜宸憋紝鎻愪緵涓€浜涘叧浜庤椹卞姩鏀寔鍝簺璁惧銆佷互鍙婂湪绯荤粺鎻掑叆鎴栫Щ闄よ椹卞姩鎵€鏀寔鐨勮澶囨椂璋冪敤鍝簺鍑芥暟鐨勪俊鎭€傛墍鏈夎繖浜涗俊鎭兘閫氳繃 `usb_driver` 缁撴瀯浼犻€掔粰 USB 瀛愮郴缁?```

    static struct usb_driver skel_driver = {
	    .name        = "skeleton",
	    .probe       = skel_probe,
	    .disconnect  = skel_disconnect,
	    .suspend     = skel_suspend,
	    .resume      = skel_resume,
	    .pre_reset   = skel_pre_reset,
	    .post_reset  = skel_post_reset,
	    .id_table    = skel_table,
	    .supports_autosuspend = 1,
    };


```
鍙橀噺鍚嶏紙name锛夋槸涓€涓弿杩拌椹卞姩瀛楃涓层€傚畠鐢ㄤ簬鎵撳嵃鍒扮郴缁熸棩蹇椾腑鐨勪俊鎭€ф秷鎭€俻robe 鍜?disconnect 鍑芥暟鎸囬拡鍦ㄨ `id_table` 鍙橀噺鎵€鎻愪緵鐨勪俊鎭尮閰嶅埌鐨勮澶囪鐪嬪埌鎴栬绉婚櫎鏃惰璋冪敤銆?
fops 鍜?minor 鍙橀噺鏄彲閫夌殑銆傚ぇ澶氭暟 USB 椹卞姩浼氭寕鎺ュ埌鍙︿竴涓唴鏍稿瓙绯荤粺锛屼緥濡?SCSI銆佺綉缁滄垨 TTY 瀛愮郴缁熴€傝繖绫婚┍鍔ㄥ悜鍙︿竴涓唴鏍稿瓙绯荤粺娉ㄥ唽鑷繁锛屼换浣曠敤鎴风┖闂寸殑鐨勪氦浜掗兘閫氳繃璇ユ帴鍙ｆ彁渚涖€備絾瀵逛簬娌℃湁鍖归厤鍐呮牳瀛愮郴缁熺殑椹卞姩锛屼緥濡?MP3 鎾斁鍣ㄦ垨鎵弿浠紝灏遍渶瑕佷竴绉嶄笌鐢ㄦ埛绌洪棿浜や簰鐨勬柟娉曘€俇SB 瀛愮郴缁熸彁渚涗簡涓€绉嶆敞鍐屾璁惧鍙凤紙minor device number锛夊拰涓€缁?`file_operations` 鍑芥暟鎸囬拡鐨勬柟寮忔潵瀹炵幇杩欑鐢ㄦ埛绌洪棿浜や簰銆傛鏋堕┍鍔ㄩ渶瑕佽繖绫绘帴鍙ｏ紝鍥犳瀹冩彁渚涗簡涓€涓璁惧璧峰鍙蜂互鍙婃寚鍚戝叾 `file_operations` 鍑芥暟鐨勬寚閽堛€?
鐒跺悗璇?USB 椹卞姩閫氳繃璋冪敤 usb_register() 娉ㄥ唽鍒?USB 瀛愮郴缁燂紝
```

    static int __init usb_skel_init(void)
    {
	    int result;

	    /* register this driver with the USB subsystem */
	    result = usb_register(&skel_driver);
	    if (result < 0) {
		    pr_err("usb_register failed for the %s driver. Error number %d\n",
		           skel_driver.name, result);
		    return -1;
	    }

	    return 0;
    }
    module_init(usb_skel_init);


```
褰撹椹卞姩浠庣郴缁熶腑鍗歌浇鏃讹紝瀹冮渶瑕佸悜 USB 瀛愮郴缁熸敞閿€鑷繁銆傝繖閫氳繃 usb_deregister() 瀹屾垚
```

    static void __exit usb_skel_exit(void)
    {
	    /* deregister this driver with the USB subsystem */
	    usb_deregister(&skel_driver);
    }
    module_exit(usb_skel_exit);


```
涓轰簡鍚敤 linux-hotplug 绯荤粺鍦ㄨ澶囨彃鍏ユ椂鑷姩鍔犺浇璇ラ┍鍔紝浣犻渶瑕佸垱寤轰竴涓?`MODULE_DEVICE_TABLE`銆備互涓嬩唬鐮佸憡璇?hotplug 鑴氭湰璇ユā鍧楁敮鎸?```

    /* table of devices that work with this driver */
    static struct usb_device_id skel_table [] = {
	    { USB_DEVICE(USB_SKEL_VENDOR_ID, USB_SKEL_PRODUCT_ID) },
	    { }                      /* Terminating entry */
    };
    MODULE_DEVICE_TABLE (usb, skel_table);


```
杩樻湁鍏跺畠瀹忓彲鐢ㄤ簬鎻忚堪鏀寔涓€鏁翠釜 USB 椹卞姩绫荤殑 struct `usb_device_id`銆傛湁鍏虫浜嬬殑鏇村淇℃伅锛岃鍙傞槄 usb.h <usb_header>銆?

## 璁惧鎿嶄綔


褰撲竴涓笌浣犵殑椹卞姩鍚?USB 鏍稿績娉ㄥ唽鐨?ID 妯″紡鐩稿尮閰嶇殑璁惧琚彃鍏?USB 鎬荤嚎鏃讹紝浼氳皟鐢?probe 鍑芥暟銆備紶閫掔粰瀹冪殑鏄?`usb_device` 缁撴瀯銆佹帴鍙ｅ彿浠ュ強
```

    static int skel_probe(struct usb_interface *interface,
	const struct usb_device_id *id)


```
椹卞姩鐜板湪闇€瑕侀獙璇佽璁惧纭疄鏄畠鍙互鎺ュ彈鐨勩€傚鏋滄槸锛屽畠杩斿洖 0銆傚鏋滀笉鏄紝鎴栬€呭湪鍒濆鍖栨湡闂村彂鐢熶换浣曢敊璇紝鍒?probe 鍑芥暟杩斿洖涓€涓敊璇爜锛堝 `-ENOMEM` 鎴?`-ENODEV`锛夈€?
鍦ㄦ鏋堕┍鍔ㄤ腑锛屾垜浠‘瀹氬摢浜涚鐐硅鏍囪涓烘壒閲忚緭鍏ワ紙bulk-in锛夊拰鎵归噺杈撳嚭锛坆ulk-out锛夈€傛垜浠垱寤虹紦鍐插尯鏉ヤ繚瀛樺皢浠庤澶囧彂閫佸拰鎺ユ敹鐨勬暟鎹紝骞跺垵濮嬪寲涓€涓敤浜庡悜璁惧鍐欏叆鏁版嵁鐨?USB urb銆?
鐩稿弽锛屽綋璁惧浠?USB 鎬荤嚎绉婚櫎鏃讹紝浼氳皟鐢?disconnect 鍑芥暟锛屽苟浼犲叆璁惧鎸囬拡銆傞┍鍔ㄩ渶瑕佹竻鐞嗘鏃跺凡鍒嗛厤鐨勪换浣曠鏈夋暟鎹紝骞跺叧闂?USB 绯荤粺涓换浣曞緟澶勭悊鐨?urb銆?
鐜板湪璁惧宸叉彃鍏ョ郴缁熶笖椹卞姩宸茬粦瀹氬埌璇ヨ澶囷紝浠庣敤鎴风▼搴忚瘯鍥句笌璇ヨ澶囬€氫俊鏃讹紝浼犻€掔粰 USB 瀛愮郴缁熺殑 `file_operations` 缁撴瀯涓殑浠讳綍鍑芥暟閮藉皢琚皟鐢ㄣ€傜涓€涓璋冪敤鐨勫嚱鏁板皢鏄?open锛屽洜涓虹▼搴忚瘯鍥炬墦寮€璇ヨ澶囪繘琛?I/O銆傛垜浠€掑绉佹湁浣跨敤璁℃暟锛屽苟灏嗘寚鍚戞垜浠唴閮ㄧ粨鏋勭殑鎸囬拡淇濆瓨鍒?file 缁撴瀯涓€傝繖鏍峰仛鏄负浜嗗皢鏉ュ鏂囦欢鎿嶄綔鐨勮皟鐢ㄨ兘澶熻椹卞姩纭畾鐢ㄦ埛姝ｅ湪瀵诲潃鐨勬槸鍝釜璁惧銆傛墍鏈?```

    /* increment our usage count for the device */
    kref_get(&dev->kref);

    /* save our object in the file's private structure */
    file->private_data = dev;


```
鍦?open 鍑芥暟琚皟鐢ㄤ箣鍚庯紝浼氳皟鐢?read 鍜?write 鍑芥暟鏉ユ帴鏀跺拰鍙戦€佹暟鎹粰璁惧銆傚湪 `skel_write` 鍑芥暟涓紝鎴戜滑鎺ユ敹鍒扮敤鎴锋兂瑕佸彂閫佺粰璁惧鐨勬暟鎹寚閽堜互鍙婃暟鎹ぇ灏忋€傝鍑芥暟鏍规嵁瀹冨凡鍒涘缓鐨勫啓 urb 鐨勫ぇ灏忥紙璇ュぇ灏忓彇鍐充簬璁惧鎵€鎷ユ湁鐨勬壒閲忚緭鍑虹鐐圭殑澶у皬锛夋潵纭畾瀹冭兘鍚戣澶囧彂閫佸灏戞暟鎹€傜劧鍚庡皢鏁版嵁浠庣敤鎴风┖闂存嫹璐濆埌鍐呮牳绌洪棿锛屽皢 urb 鎸囧悜璇ユ暟鎹紝骞跺皢 urb 鎻愪氦缁?USB
```

    /* we can only write as much as 1 urb will hold */
    size_t writesize = min_t(size_t, count, MAX_TRANSFER);

    /* copy the data from user space into our urb */
    copy_from_user(buf, user_buffer, writesize);

    /* set up our urb */
    usb_fill_bulk_urb(urb,
		      dev->udev,
		      usb_sndbulkpipe(dev->udev, dev->bulk_out_endpointAddr),
		      buf,
		      writesize,
		      skel_write_bulk_callback,
		      dev);

    /* send the data out the bulk port */
    retval = usb_submit_urb(urb, GFP_KERNEL);
    if (retval) {
	    dev_err(&dev->interface->dev,
                "%s - failed submitting write urb, error %d\n",
                __func__, retval);
    }


```
褰撳啓 urb 浣跨敤 `usb_fill_bulk_urb` 鍑芥暟濉ソ閫傚綋鐨勪俊鎭悗锛屾垜浠皢 urb 鐨勫畬鎴愬洖璋冨嚱鏁版寚鍚戞垜浠嚜宸辩殑 `skel_write_bulk_callback` 鍑芥暟銆傚綋 urb 琚?USB 瀛愮郴缁熷畬鎴愭椂锛屼細璋冪敤璇ュ嚱鏁般€傚洖璋冨嚱鏁板湪涓柇涓婁笅鏂囦腑琚皟鐢紝鍥犳蹇呴』灏忓績涓嶈鍦ㄥ叾涓仛杩囧鐨勫鐞嗐€傛垜浠殑 `skel_write_bulk_callback` 瀹炵幇鍙槸鎶ュ憡 urb 鏄惁鎴愬姛瀹屾垚锛岀劧鍚庤繑鍥炪€?
璇诲嚱鏁扮殑宸ヤ綔鏂瑰紡涓庡啓鍑芥暟鐣ユ湁涓嶅悓锛氭垜浠笉浣跨敤 urb 灏嗘暟鎹粠璁惧浼犺緭鍒伴┍鍔ㄣ€傜浉鍙嶏紝鎴戜滑璋冪敤 `usb_bulk_msg` 鍑芥暟锛屽畠鍙敤浜庡悜璁惧鍙戦€佹垨鎺ユ敹鏁版嵁锛岃€屾棤闇€鍒涘缓 urb 骞跺鐞?urb 瀹屾垚鍥炶皟銆傛垜浠皟鐢?`usb_bulk_msg` 鍑芥暟锛岀粰瀹冧竴涓敤浜庢斁缃粠璁惧鎺ユ敹鍒扮殑浠讳綍鏁版嵁鐨勭紦鍐插尯锛屼互鍙婁竴涓秴鏃跺€笺€傚鏋滆秴鏃舵湡闄愬埌鏈熻€屾病鏈変粠璁惧鎺ユ敹鍒颁换浣曟暟鎹紝璇ュ嚱鏁板皢澶辫触骞惰繑鍥?```

    /* do an immediate bulk read to get data from the device */
    retval = usb_bulk_msg (skel->dev,
			   usb_rcvbulkpipe (skel->dev,
			   skel->bulk_in_endpointAddr),
			   skel->bulk_in_buffer,
			   skel->bulk_in_size,
			   &count, 5000);
    /* if the read was successful, copy the data to user space */
    if (!retval) {
	    if (copy_to_user (buffer, skel->bulk_in_buffer, count))
		    retval = -EFAULT;
	    else
		    retval = count;
    }


```
`usb_bulk_msg` 鍑芥暟瀵逛簬瀵硅澶囪繘琛屽崟娆¤鎴栧啓闈炲父鏈夌敤锛涗絾鏄紝濡傛灉浣犻渶瑕佹寔缁湴璇绘垨鍐欒澶囷紝寤鸿寤虹珛鑷繁鐨?urb 骞跺皢鍏舵彁浜ょ粰 USB 瀛愮郴缁熴€?
褰撶敤鎴风▼搴忛噴鏀惧畠鐢ㄤ簬涓庤璁惧閫氫俊鐨勬枃浠跺彞鏌勬椂锛屼細璋冪敤椹卞姩涓殑 release 鍑芥暟銆傚湪璇ュ嚱鏁颁腑锛屾垜浠€掑噺绉佹湁浣跨敤璁℃暟锛屽苟绛夊緟鍙兘鐨?```

    /* decrement our usage count for the device */
    --skel->open_count;


```
USB 椹卞姩蹇呴』鑳藉骞虫粦澶勭悊鐨勪竴涓緝鍥伴毦鐨勯棶棰樻槸锛歎SB 璁惧鍙兘鍦ㄤ换浣曟椂鍒讳粠绯荤粺涓绉婚櫎锛屽嵆浣夸竴涓▼搴忓綋鍓嶆鍦ㄤ笌瀹冮€氫俊銆傚畠闇€瑕佽兘澶熷叧闂换浣曞綋鍓嶇殑璇诲啓锛屽苟閫氱煡鐢ㄦ埛绌洪棿绋嬪簭璇ヨ澶囧凡涓嶅啀瀛樺湪銆備互涓嬩唬鐮侊紙鍑芥暟 `skel_delete`锛夋槸涓€涓浣曞鐞?```

    static inline void skel_delete (struct usb_skel *dev)
    {
	kfree (dev->bulk_in_buffer);
	if (dev->bulk_out_buffer != NULL)
	    usb_free_coherent (dev->udev, dev->bulk_out_size,
		dev->bulk_out_buffer,
		dev->write_urb->transfer_dma);
	usb_free_urb (dev->write_urb);
	kfree (dev);
    }


```
濡傛灉涓€涓▼搴忓綋鍓嶆寔鏈夎璁惧鐨勬墦寮€鍙ユ焺锛屾垜浠浣?`device_present` 鏍囧織銆傚浜庢瘡涓€涓湡鏈涜澶囧瓨鍦ㄧ殑璇汇€佸啓銆乺elease 浠ュ強鍏跺畠鍑芥暟锛岄┍鍔ㄩ鍏堟鏌ヨ鏍囧織浠ユ煡鐪嬭澶囨槸鍚︿粛鐒跺瓨鍦ㄣ€傚鏋滀笉瀛樺湪锛屽畠鎶ュ憡璁惧宸叉秷澶憋紝骞跺悜鐢ㄦ埛绌洪棿绋嬪簭杩斿洖 `-ENODEV` 閿欒銆傚綋鏈€缁堣皟鐢?release 鍑芥暟鏃讹紝瀹冨垽鏂槸鍚︽病鏈夎澶囷紝濡傛灉鏄紝鍒欐墽琛?`skel_disconnect` 鍑芥暟鍦ㄦ病鏈夋墦寮€鐨勬枃浠舵椂閫氬父浼氬仛鐨勬竻鐞嗗伐浣滐紙瑙佹竻鍗?5锛夈€?

## 鍚屾锛圛sochronous锛夋暟鎹?

杩欎釜 usb-skeleton 椹卞姩娌℃湁浠讳綍鍙戦€佹垨鎺ユ敹涓柇鏁版嵁鎴栧悓姝ユ暟鎹殑渚嬪瓙銆備腑鏂暟鎹殑鍙戦€佸嚑涔庝笌鎵归噺鏁版嵁瀹屽叏鐩稿悓锛屽彧鏈変竴浜涘井灏忕殑渚嬪銆傚悓姝ユ暟鎹殑宸ヤ綔鏂瑰紡涓嶅悓锛屾湁杩炵画鐨勬暟鎹祦琚彂閫佹垨鎺ユ敹銆傞煶棰戝拰瑙嗛鐩告満椹卞姩鏄鐞嗗悓姝ユ暟鎹殑椹卞姩鐨勫ソ渚嬪瓙锛屽鏋滀綘涔熼渶瑕佸仛杩欎欢浜嬶紝瀹冧滑浼氬緢鏈夌敤銆?

## 缁撹


濡?usb-skeleton 椹卞姩鎵€绀猴紝缂栧啓 Linux USB 璁惧椹卞姩骞朵笉鏄竴椤瑰洶闅剧殑浠诲姟銆傝椹卞姩锛岀粨鍚堝綋鍓嶇殑鍏跺畠 USB 椹卞姩锛屽簲褰撴彁渚涜冻澶熺殑渚嬪瓙锛屽府鍔╁垵瀛﹁€呬綔鑰呭湪鏈€鐭殑鏃堕棿鍐呭垱寤轰竴涓彲宸ヤ綔鐨勯┍鍔ㄣ€俵inux-usb-devel 閭欢鍒楄〃鐨勫綊妗ｄ篃鍖呭惈澶ч噺鏈夌敤鐨勪俊鎭€?

## 璧勬簮锛圧esources锛?

The Linux USB Project:
http://www.linux-usb.org/

Linux Hotplug Project:
http://linux-hotplug.sourceforge.net/

linux-usb Mailing List Archives:
https://lore.kernel.org/linux-usb/

Programming Guide for Linux USB Device Drivers:
https://lmu.web.psi.ch/docu/manuals/software_manuals/linux_sl/usb_linux_programming_guide.pdf

USB Home Page: https://www.usb.org
