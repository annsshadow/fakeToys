
## 濡備綍璁?bt8xx 绯诲垪鍗″伐浣?

Authors:
	 Richard Walker,
	 Jamie Honan,
	 Michael Hunold,
	 Manu Abraham,
	 Uwe Bugla,
	 Michael Krufky

### 涓€鑸俊鎭?

杩欑被鍗′互 bt878a 浣滀负 PCI 鎺ュ彛锛屽苟涓旈渶瑕?bttv 椹卞姩鏉ヨ闂?bt8xx 鑺墖缁勭殑 i2c 鎬荤嚎鍜?gpio 寮曡剼銆?
鍏充簬 Linux 鍐呮牳鏀寔鐨勫熀浜?Conexant Bt8xx PCI 妗ョ殑鍗＄殑瀹屾暣鍒楄〃锛岃鍙傝
`Documentation/admin-guide/media/bttv-cardlist.rst`銆?
涓轰簡鑳藉缂栬瘧鍐呮牳锛屽簲閰嶇疆涓€浜涢€夐」
```

    ./scripts/config -e PCI
    ./scripts/config -e INPUT
    ./scripts/config -m I2C
    ./scripts/config -m MEDIA_SUPPORT
    ./scripts/config -e MEDIA_PCI_SUPPORT
    ./scripts/config -e MEDIA_ANALOG_TV_SUPPORT
    ./scripts/config -e MEDIA_DIGITAL_TV_SUPPORT
    ./scripts/config -e MEDIA_RADIO_SUPPORT
    ./scripts/config -e RC_CORE
    ./scripts/config -m VIDEO_BT848
    ./scripts/config -m DVB_BT8XX

```
濡傛灉浣犲笇鏈涜嚜鍔ㄦ敮鎸?Bt8xx 鐨勬墍鏈夊彲鑳藉彉浣?```

    ./scripts/config -e MEDIA_SUBDRV_AUTOSELECT

```
   璇疯皑鎱庝娇鐢ㄤ互涓嬮€夐」锛屽洜涓哄彇娑堥€夋嫨瀹為檯蹇呴渶鐨勯┍鍔ㄥ彲鑳藉鑷村洜缂哄皯椹卞姩鏀寔鑰屾棤娉曡皟璋愮殑 DVB 璁惧銆?
濡傛灉浣犵殑鐩爣鍙槸鏀寔鏌愪釜鐗瑰畾鐨勬澘鍗★紝浣犲彲浠ョ鐢?MEDIA_SUBDRV_AUTOSELECT 骞舵墜鍔ㄩ€夋嫨浣犵殑鏉垮崱鎵€闇€鐨?鍓嶇椹卞姩銆傝繖鏍凤紝浣犲彲浠ヨ妭鐪佷竴浜?RAM銆?
浣犲彲浠ラ€氳繃璋冪敤 make xconfig/qconfig/menuconfig锛屽苟鏌ョ湅杩欎簺鑿滃崟閫夐」鏉ュ畬鎴愶紙浠呭湪
`Autoselect ancillary drivers` 琚鐢ㄦ椂鍚敤锛夛細

#) `Device drivers` => `Multimedia support` => `Customize TV tuners`
#) `Device drivers` => `Multimedia support` => `Customize DVB frontends`

鐒跺悗锛屽湪涓婅堪姣忎釜鑿滃崟涓紝璇烽€夋嫨浣犳澘鍗＄壒瀹氱殑鍓嶇涓庤皟璋愬櫒妯″潡銆?

### 鍔犺浇妯″潡


甯歌鎯呭喌锛氬鏋?bttv 椹卞姩妫€娴嬪埌涓€涓熀浜?bt8xx 鐨?DVB 鍗★紝鎵€鏈夊墠绔笌鍚庣妯″潡閮戒細鑷姩鍔犺浇銆?
渚嬪鎯呭喌鏈夛細

- 娌℃湁 EEPROM銆佸叡浜竴涓€氱敤 PCI 瀛愮郴缁?ID 鐨勬棫鐢佃鍗★紱
- 甯︽湁鎴栦笉甯︽湁 CA 鎻掓Ы銆佷笖涓嶅寘鍚?Eeprom 鐨勬棫 TwinHan DST 鍗℃垨鍏跺厠闅嗐€?
鍦ㄤ互涓嬫儏鍐典笅锛屽彲鑳介渶瑕侀€氳繃浼犻€?modprobe 鍙傛暟鏉ヨ鐩?bttv 鍜?dvb-bt8xx 椹卞姩鐨?PCI 绫诲瀷妫€娴嬨€?
#### 杩愯 TwinHan 鍙婂叾鍏嬮殕鍗?

濡?`Documentation/admin-guide/media/bttv-cardlist.rst` 鎵€绀猴紝TwinHan 鍙婂叾鍏嬮殕鍗′娇鐢?`card=113`
modprobe 鍙傛暟銆傚洜姝わ紝涓轰簡姝ｇ‘
```

	$ modprobe bttv card=113
	$ modprobe dst

```
```

	verbose=0:		绂佺敤娑堟伅
		1:		浠呮樉绀洪敊璇秷鎭?		2:		鏄剧ず閫氱煡
		3:		鏄剧ず鍏朵粬鏈夌敤鐨勬秷鎭?		4:		璋冭瘯璁剧疆
	dst_addons=0:		鍗′粎涓哄厤璐瑰崼鏄燂紙FTA锛夊崱
		0x20:		鍗″甫鏈夌敤浜庡姞鎵伴閬撶殑鏉′欢鎺ユ敹鎻掓Ы
	dst_algo=0:		锛堥粯璁わ級杞欢璋冭皭绠楁硶
	         1:		纭欢璋冭皭绠楁硶


```
鑷姩妫€娴嬬殑鍊肩敱鍗＄殑鈥滃搷搴斿瓧绗︿覆鈥濆喅瀹氥€?
鍦ㄤ綘鐨勬棩蹇椾腑鍙锛屼緥濡傦細dst_get_device_id: Recognize [DSTMCI]銆?
瀵逛簬缂洪櫡鎶ュ憡锛岃鍙戦€佷竴浠芥縺娲讳簡 verbose=4 鐨勫畬鏁存棩蹇椼€傚彟璇峰弬瑙?`Documentation/admin-guide/media/ci.rst`銆?
#### 杩愯澶氬紶鍗?

鍏充簬鍗?ID 鐨勫畬鏁村垪琛紝璇峰弬瑙?`Documentation/admin-guide/media/bttv-cardlist.rst`銆備竴浜涚ず渚嬶細

	===========================	===
	Brand name			ID
	===========================	===
	Pinnacle PCTV Sat		 94
	Nebula Electronics Digi TV	104
	pcHDTV HD-2000 TV		112
	Twinhan DST and clones		113
	Avermedia AverTV DVB-T 77:	123
	Avermedia AverTV DVB-T 761	124
	DViCO FusionHDTV DVB-T Lite	128
	DViCO FusionHDTV 5 Lite		135
	===========================	===

   褰撲綘鏈夊寮犲崱鏃讹紝鍗?ID 鐨勯『搴忓簲涓庣郴缁熸娴嬪埌瀹冧滑鐨勯『搴忎竴鑷淬€傝娉ㄦ剰锛岀Щ闄?鎻掑叆鍏朵粬 PCI 鍗″彲鑳戒細
   鏀瑰彉妫€娴嬮『搴忋€?
```

	$ modprobe bttv card=113 card=135

```
濡傛灉杩樻湁杩涗竴姝ョ殑闂锛岃璁㈤槄骞跺悜閭欢鍒楄〃鍙戦€侀棶棰橈細linux-media@vger.kernel.org銆?
#### 鎺㈡祴 PCI 瀛愮郴缁?ID 鎹熷潖鐨勫崱


鏈変竴浜?TwinHan 鍗＄敱浜庢煇绉嶅師鍥犲叾 EEPROM 宸叉崯鍧忋€傝繖浜涘崱娌℃湁姝ｇ‘鐨?PCI 瀛愮郴缁?ID銆?```

	$ echo 109e 0878 $subvendor $subdevice > \
		/sys/bus/pci/drivers/bt878/new_id

```
```

	109e: PCI_VENDOR_ID_BROOKTREE
	0878: PCI_DEVICE_ID_BROOKTREE_878

```
