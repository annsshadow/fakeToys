
## 涓哄獟浣撹澶囨瀯寤烘敮鎸?

绗竴姝ユ槸涓嬭浇鍐呮牳婧愪唬鐮侊紝鍙互閫氳繃鍙戣鐗堢壒瀹氱殑婧愭枃浠讹紝涔熷彲浠ラ€氳繃鍐呮牳鐨勪富 git 鏍慭 [^1^]_銆?
浣嗚娉ㄦ剰锛屽鏋滐細

- 浣犳槸涓媷鏁㈣€咃紝鎯冲皾璇曟柊涓滆タ锛?- 濡傛灉浣犳兂鎶ュ憡涓€涓?bug锛?- 濡傛灉浣犳鍦ㄥ紑鍙戞柊琛ヤ竵

浣犲簲璇ヤ娇鐢ㄤ富濯掍綋寮€鍙戞爲鐨?`master` 鍒嗘敮锛?
    https://git.linuxtv.org/media.git/

鍦ㄨ繖绉嶆儏鍐典笅锛屼綘鍙互鍦?`LinuxTv wiki 椤甸潰 <https://linuxtv.org/wiki>`_ 鎵惧埌涓€浜涙湁鐢ㄧ殑淇℃伅锛?
    https://linuxtv.org/wiki/index.php/How_to_Obtain,_Build_and_Install_V4L-DVB_Device_Drivers


       https://git.kernel.org/pub/scm/li  nux/kernel/git/torvalds/linux.git/

## 閰嶇疆 Linux 鍐呮牳


```
    $ make menuconfig

```
鐒跺悗锛岄€夋嫨鎵€鏈夋湡鏈涚殑閫夐」骞堕€€鍑猴紝淇濆瓨閰嶇疆銆?
淇敼鍚庣殑閰嶇疆灏嗕綅浜?`.config` 鏂囦欢涓€傚畠澶ф浼氭槸
```

    ...
    # CONFIG_RC_CORE is not set
    # CONFIG_CEC_CORE is not set
    CONFIG_MEDIA_SUPPORT=m
    CONFIG_MEDIA_SUPPORT_FILTER=y
    ...

```
```
    Device Drivers --->
	<M> Remote Controller support  --->
	[ ] HDMI CEC RC integration
	[ ] Enable CEC error injection support
	[*] HDMI CEC drivers  --->
	<*> Multimedia support  --->

```
`Remote Controller support` 閫夐」鍚敤瀵归仴鎺у櫒鐨勬牳蹇冩敮鎸乗 [^2^]_銆?
`HDMI CEC RC integration` 閫夐」鍚敤 HDMI CEC 涓?Linux 鐨勯泦鎴愶紝鍏佽鍍忔帴鏀剁敱鐩存帴杩炴帴鍒版満鍣ㄧ殑閬ユ帶鍣ㄤ骇鐢熺殑鏁版嵁涓€鏍凤紝閫氳繃 HDMI CEC 鎺ユ敹鏁版嵁銆?
`HDMI CEC drivers` 閫夐」鍏佽閫夋嫨閫氳繃 HDMI 鎺ュ彛鎺ユ敹鍜?鎴栧彂閫?CEC 鐮佺殑骞宠嚭鍜?USB 椹卞姩\ [^3^]_銆?
鏈€鍚庝竴涓€夐」锛坄Multimedia support`锛夊惎鐢ㄥ鎽勫儚澶淬€侀煶瑙嗛閲囬泦鍗″拰鐢佃鐨勬敮鎸併€?
濯掍綋瀛愮郴缁熸敮鎸佹棦鍙互涓庝富鍐呮牳涓€璧锋瀯寤猴紝涔熷彲浠ヤ綔涓烘ā鍧楁瀯寤恒€傚湪澶у鏁扮敤渚嬩腑锛屾洿鍊惧悜浜庡皢鍏舵瀯寤轰负妯″潡銆?

   涓庡叾浣跨敤鑿滃崟锛屽唴鏍歌繕鎻愪緵浜嗕竴涓剼鏈紝鍏佽鐩存帴鍚敤閰嶇疆閫夐」銆傝鍚敤濯掍綋鏀寔
```

	$ scripts/config -m RC_CORE
	$ scripts/config -m MEDIA_SUPPORT

```
       鎯宠浣跨敤鏌愪簺鍙兘渚濊禆閬ユ帶鍣ㄦ牳蹇冩敮鎸佺殑鐢佃鍗￠┍鍔ㄦ椂銆?
       浣跨敤濯掍綋 HDMI CEC 鏀寔鏃躲€?
       杩欎簺鐗瑰畾浜?GPU 鐨勯┍鍔ㄩ€氳繃 `Device Drivers` 涓嬬殑 `Graphics support`
       鑿滃崟閫夋嫨銆?
       褰撴煇涓?GPU 椹卞姩鏀寔 HDMI CEC 鏃讹紝瀹冧細鑷姩鍦ㄥ獟浣撳瓙绯荤粺鍚敤 CEC 鏍稿績鏀寔銆?
### 濯掍綋渚濊禆


搴斿綋娉ㄦ剰锛屼粠涓€涓共鍑€鐨勯厤缃紑濮嬪惎鐢ㄤ笂杩伴€夐」閫氬父杩樹笉澶熴€傚獟浣撳瓙绯荤粺渚濊禆浜庤嫢骞插叾浠?Linux 鏍稿績鏀寔鎵嶈兘宸ヤ綔銆?
渚嬪锛屽ぇ澶氭暟濯掍綋璁惧浣跨敤涓茶閫氫俊鎬荤嚎鏉ヤ笌鏌愪簺澶栬閫氫俊銆傝繖绉嶆€荤嚎绉颁负 I虏C
锛圛nter-Integrated Circuit锛岄泦鎴愮數璺棿鎬荤嚎锛夈€備负浜嗚兘澶熸瀯寤哄姝ょ被纭欢鐨勬敮鎸侊紝搴斿綋鍚敤 I虏C 鎬荤嚎鏀寔锛屽彲浠ラ€氳繃
```

    ./scripts/config -m I2C

```
鍙︿竴涓緥瀛愶細閬ユ帶鍣ㄦ牳蹇冮渶瑕佹敮鎸?```

    ./scripts/config -m INPUT

```
鏍规嵁浣犳兂鍚敤鐨勫叿浣撻┍鍔紝鍙兘杩橀渶瑕佸叾浠栨牳蹇冨姛鑳斤紙濡?PCI 鍜?鎴?USB 鏀寔锛夈€?
### 鍚敤閬ユ帶鍣ㄦ敮鎸?

閬ユ帶鍣ㄨ彍鍗曞厑璁搁€夋嫨鐗瑰畾璁惧鐨勯┍鍔ㄣ€?```

         --- Remote Controller support
         <M>   Compile Remote Controller keymap modules
         [*]   LIRC user interface
         [*]     Support for eBPF programs attached to lirc devices
         [*]   Remote controller decoders  --->
         [*]   Remote Controller devices  --->

```
`Compile Remote Controller keymap modules` 閫夐」涓鸿嫢骞蹭釜娴佽鐨勯仴鎺у櫒鍒涘缓閿槧灏勩€?
`LIRC user interface` 閫夐」閫氳繃鍚敤涓€涓厑璁哥敤鎴风┖闂存帴鏀舵潵鑷仴鎺у櫒鐨勫師濮嬫暟鎹殑 API锛屽湪浣跨敤 `lirc` 绋嬪簭鏃跺鍔犲寮哄姛鑳姐€?
`Support for eBPF programs attached to lirc devices` 閫夐」鍏佽浣跨敤鐗规畩绋嬪簭锛堢О涓?eBPF锛夛紝璁╁簲鐢ㄧ▼搴忚兘澶熷悜 Linux 鍐呮牳娣诲姞棰濆鐨勯仴鎺у櫒瑙ｇ爜鍔熻兘銆?
`Remote controller decoders` 閫夐」鍏佽閫夋嫨灏嗚 Linux 鍐呮牳璇嗗埆鐨勫崗璁€傞櫎闈炰綘鎯崇鐢ㄦ煇涓壒瀹氱殑瑙ｇ爜鍣紝鍚﹀垯寤鸿淇濇寔鎵€鏈夊瓙閫夐」鍚敤銆?
`Remote Controller devices` 鍏佽浣犻€夋嫨鏀寔浣犵殑璁惧鎵€闇€鐨勯┍鍔ㄣ€?
鍚屾牱鐨勯厤缃篃鍙互閫氳繃 `script/config` 鑴氭湰璁剧疆銆備緥濡傦紝涓轰簡鏀寔 ITE 閬ユ帶鍣?```

	$ scripts/config -e INPUT
	$ scripts/config -e ACPI
	$ scripts/config -e MODULES
	$ scripts/config -m RC_CORE
	$ scripts/config -e RC_DEVICES
	$ scripts/config -e RC_DECODERS
	$ scripts/config -m IR_RC5_DECODER
	$ scripts/config -m IR_ITE_CIR

```
### 鍚敤 HDMI CEC 鏀寔


褰撴煇涓┍鍔ㄩ渶瑕?HDMI CEC 鏀寔鏃讹紝瀹冧細鑷姩璁剧疆銆傚洜姝わ紝浣犲彧闇€瑕佸惎鐢ㄥ闇€瑕佸畠鐨勬樉鍗＄殑鏀寔锛屾垨鑰呴€氳繃鏌愪釜鐜版湁鐨?HDMI 椹卞姩鏉ュ惎鐢ㄣ€?
鐗瑰畾浜?HDMI 鐨勯┍鍔ㄤ綅浜?`HDMI CEC drivers`
```

	--- HDMI CEC drivers
	< >   ChromeOS EC CEC driver
	< >   Amlogic Meson AO CEC driver
	< >   Amlogic Meson G12A AO CEC driver
	< >   Generic GPIO-based CEC driver
	< >   Samsung S5P CEC driver
	< >   STMicroelectronics STiH4xx HDMI CEC driver
	< >   STMicroelectronics STM32 HDMI CEC driver
	< >   Tegra HDMI CEC driver
	< >   SECO Boards HDMI CEC driver
	[ ]     SECO Boards IR RC5 support
	< >   Pulse Eight HDMI CEC
	< >   RainShadow Tech HDMI CEC

```
       HDMI 璁惧渚濊禆浜庣郴缁熺殑鏋舵瀯锛屽苟涓斿湪鏂板唴鏍镐笂鍙兘鏈夋墍涓嶅悓銆?
### 鍚敤濯掍綋鏀寔


濯掍綋鑿滃崟姣旈仴鎺у櫒鑿滃崟鏈夋洿澶氱殑閫夐」銆?```

	--- Media support
	[ ] Filter media drivers
	[*] Autoselect ancillary drivers
	    Media device types --->
	    Media core support --->
	    Video4Linux options --->
	    Media controller options --->
	    Digital TV options --->
	    HDMI CEC options --->
	    Media drivers --->
	    Media ancillary drivers --->

```
闄ら潪浣犵‘鍒囩煡閬撹嚜宸卞湪鍋氫粈涔堬紝鎴栬€呬綘鎯充负 SoC 骞冲彴鏋勫缓涓€涓┍鍔紝鍚﹀垯寮虹儓寤鸿淇濇寔 `Autoselect ancillary drivers` 閫夐」寮€鍚紝鍥犱负瀹冧細鑷姩閫夋嫨鎵€闇€鐨?I虏C 杈呭姪椹卞姩銆?
鐜板湪鏈夊涓嬫墍杩扮殑涓ょ鏂瑰紡鏉ラ€夋嫨濯掍綋璁惧椹卞姩銆?
##### ``Filter media drivers`` 鑿滃崟


姝よ彍鍗曟棬鍦ㄧ畝鍖?PC 鍜岀瑪璁版湰鐢佃剳纭欢鐨勮缃€傚畠鐨勫伐浣滄柟寮忔槸璁╃敤鎴锋寚瀹氶渶瑕佸摢绫诲獟浣撻┍鍔紝
```

	[ ] Cameras and video grabbers
	[ ] Analog TV
	[ ] Digital TV
	[ ] AM/FM radio receivers/transmitters
	[ ] Software defined radio
	[ ] Platform-specific devices
	[ ] Test drivers

```
鍥犳锛屽鏋滀綘鍙兂娣诲姞瀵规憚鍍忓ご鎴栬棰戦噰闆嗗崱鐨勬敮鎸侊紝鍙€夋嫨绗竴涓€夐」鍗冲彲銆傚厑璁稿閫夈€?
涓€鏃﹂€夋嫨浜嗘鑿滃崟涓婄殑閫夐」锛屾瀯寤虹郴缁熷氨浼氳嚜鍔ㄩ€夋嫨鎵€闇€鐨勬牳蹇冮┍鍔紝浠ユ敮鎸佹墍閫夊姛鑳姐€?

   澶у鏁扮數瑙嗗崱鏄贩鍚堢殑锛氬畠浠悓鏃舵敮鎸佹ā鎷熺數瑙嗗拰鏁板瓧鐢佃銆?
   濡傛灉浣犳湁涓€寮犳贩鍚堝崱锛屽彲鑳介渶瑕佸湪鑿滃崟涓悓鏃跺惎鐢?`Analog TV`
   鍜?`Digital TV`銆?
浣跨敤姝ら€夐」鏃讹紝濯掍綋鏀寔鏍稿績鍔熻兘鐨勯粯璁ゅ€奸€氬父瓒充互鎻愪緵椹卞姩鐨勫熀鏈姛鑳姐€備笉杩囷紝浣犲彲浠ヤ娇鐢ㄤ互涓嬪悇椤硅缃笅鐨勯厤缃墜鍔ㄥ惎鐢ㄤ竴浜涙墍闇€鐨勯澶栵紙鍙€夛級鍔熻兘
```

	    Media core support --->
	    Video4Linux options --->
	    Media controller options --->
	    Digital TV options --->
	    HDMI CEC options --->

```
涓€鏃﹂€夋嫨浜嗘墍闇€鐨勮繃婊ゅ櫒锛岀鍚堣繃婊ゆ潯浠剁殑椹卞姩灏嗗湪 `Media support->Media drivers` 瀛愯彍鍗曚腑鍙敤銆?
##### ``Media Core Support`` 鑿滃崟锛堜笉杩囨护锛?

濡傛灉绂佺敤 `Filter media drivers` 鑿滃崟锛屾墍鏈変緷璧栧凡婊¤冻銆佸彲鐢ㄤ簬浣犵殑绯荤粺鐨勯┍鍔ㄩ兘搴旇鏄剧ず鍦?`Media drivers` 鑿滃崟涓€?
浣嗚娉ㄦ剰锛屼綘搴斿綋棣栧厛纭繚 `Media Core Support` 鑿滃崟鍏峰浣犵殑椹卞姩鎵€闇€鐨勬墍鏈夋牳蹇冨姛鑳斤紝鍚﹀垯鐩稿簲鐨勮澶囬┍鍔ㄤ笉浼氭樉绀恒€?
### 绀轰緥


涓轰簡瀵?[this table <cx231xx-cardlist>](this table <cx231xx-cardlist>) 涓垪鍑虹殑鏌愪竴鍧楁澘鍗″惎鐢ㄦā鍧楀寲鏀寔锛屽苟閰嶅悎妯″潡鍖栫殑濯掍綋鏍稿績妯″潡锛?```

    CONFIG_MODULES=y
    CONFIG_USB=y
    CONFIG_I2C=y
    CONFIG_INPUT=y
    CONFIG_RC_CORE=m
    CONFIG_MEDIA_SUPPORT=m
    CONFIG_MEDIA_SUPPORT_FILTER=y
    CONFIG_MEDIA_ANALOG_TV_SUPPORT=y
    CONFIG_MEDIA_DIGITAL_TV_SUPPORT=y
    CONFIG_MEDIA_USB_SUPPORT=y
    CONFIG_VIDEO_CX231XX=y
    CONFIG_VIDEO_CX231XX_DVB=y

```
## 鏋勫缓骞跺畨瑁呮柊鍐呮牳


涓€鏃?`.config` 鏂囦欢鍏峰浜嗕竴鍒囨墍闇€锛屾瀯寤烘墍闇€鐨勪竴鍒囧氨鏄?```

    $ make

```
```

    $ sudo make modules_install
    $ sudo make install

```
## 浠呮瀯寤烘柊鐨勫獟浣撻┍鍔ㄥ拰鏍稿績


浠庡紑鍙戞爲杩愯涓€涓柊鐨勫紑鍙戝唴鏍搁€氬父鏄湁椋庨櫓鐨勶紝鍥犱负瀹冨彲鑳藉寘鍚彲鑳芥湁 bug 鐨勫疄楠屾€ф敼鍔ㄣ€傚洜姝わ紝鏈変竴浜涙柟娉曞彲浠ヤ娇鐢ㄦ浛浠ｆ爲鏉ヤ粎鏋勫缓鏂扮殑椹卞姩銆?
鏈変竴涓?`Linux Kernel backports 椤圭洰
<https://backports.wiki.kernel.org/index.php/Main_Page>`_锛屽叾涓寘鍚棬鍦ㄩ拡瀵圭ǔ瀹氬唴鏍哥紪璇戠殑杈冩柊椹卞姩銆?
璐熻矗缁存姢濯掍綋瀛愮郴缁熺殑 LinuxTV 寮€鍙戣€呬篃缁存姢浜嗕竴涓?backport 鏍戯紝鍏朵腑鍙寘鍚瘡澶╀粠鏈€鏂板唴鏍告洿鏂扮殑濯掍綋椹卞姩銆傝鏍戜綅浜庯細

https://git.linuxtv.org/media_build.git/

搴斿綋娉ㄦ剰锛岃櫧鐒跺皢 `media_build` 鏍戠敤浜庢祴璇曠洰鐨勭浉瀵瑰畨鍏紝浣嗗苟涓嶈兘淇濊瘉瀹冭兘鍦ㄩ殢鏈虹殑鍐呮牳涓婂伐浣滐紙鐢氳嚦鏋勫缓鎴愬姛锛夈€傝鏍戦伒寰€滃敖鍔涜€屼负鈥濈殑鍘熷垯缁存姢锛屽湪鎴戜滑鏃堕棿鍏佽鏃朵慨澶嶅叾涓殑闂銆?
濡傛灉浣犲彂鐜板畠鏈変换浣曢棶棰橈紝娆㈣繋鍚?Linux 濯掍綋瀛愮郴缁熺殑閭欢鍒楄〃鎻愪氦琛ヤ竵锛歮edia@vger.kernel.org銆傚鏋滀綘涓?media-build 鎻愪氦鏂拌ˉ涓侊紝璇峰湪閭欢涓婚涓坊鍔?`[PATCH media-build]`銆?
```

    $ ./build

```

    1) 濡傛灉 `media-build` 鏍戣鏇存柊锛屼綘鍙兘闇€瑕佽繍琛屼袱娆★紱
    2) 濡傛灉浣犺繃鍘绘浘涓轰笌浣犲綋鍓嶄娇鐢ㄧ殑涓嶅悓鍐呮牳鐗堟湰鏋勫缓杩囧畠锛屼綘鍙兘闇€瑕佹墽琛屼竴娆?`make distclean`锛?    3) 榛樿鎯呭喌涓嬶紝瀹冧細浣跨敤浣犳鍦ㄨ繍琛岀殑鍐呮牳涓负濯掍綋瀹氫箟鐨勭浉鍚岄厤缃€夐」銆?
涓轰簡閫夋嫨涓嶅悓鐨勯┍鍔ㄦ垨涓嶅悓鐨勯厤缃€夐」锛?```

    $ make menuconfig

```
```

    $ make && sudo make install

```
杩欏皢瑕嗙洊浣犵殑鍐呮牳涔嬪墠姝ｅ湪浣跨敤鐨勫獟浣撻┍鍔ㄣ€?