锘?
## SCSI 涓棿灞?- 搴曞眰椹卞姩鎺ュ彛


## 绠€浠?

鏈枃妗ｆ杩颁簡 Linux SCSI 涓棿灞傦紙mid level锛変笌 SCSI 搴曞眰椹卞姩锛坙ower level driver锛変箣闂寸殑鎺ュ彛銆傚簳灞傞┍鍔紙LLD锛変篃琚О涓轰富鏈烘€荤嚎閫傞厤鍣紙HBA锛夐┍鍔ㄥ拰涓绘満椹卞姩锛圚D锛夈€傚湪姝よ澧冧笅锛?涓绘満锛坔ost锛?鏄绠楁満 IO 鎬荤嚎锛堜緥濡?PCI 鎴?ISA锛変笌 SCSI 浼犺緭灞備笂鍗曚釜 SCSI 鍙戣捣鑰呯鍙ｄ箣闂寸殑妗ャ€傚彂璧疯€咃紙"initiator"锛夌鍙ｏ紙SCSI 鏈锛屽弬瑙?SAM-3锛岀綉鍧€ http://www.t10.org锛夊悜"鐩爣锛坱arget锛?SCSI 绔彛锛堜緥濡傜鐩橈級鍙戦€?SCSI 鍛戒护銆傚湪涓€涓繍琛屼腑鐨勭郴缁熶腑鍙互瀛樺湪璁稿 LLD锛屼絾姣忕纭欢绫诲瀷鍙兘鏈変竴涓€傚ぇ澶氭暟 LLD 鍙互鎺у埗涓€涓垨澶氫釜 SCSI HBA銆傛煇浜?HBA 鍖呭惈澶氫釜涓绘満銆?

鍦ㄦ煇浜涙儏鍐典笅锛孲CSI 浼犺緭灞傛槸涓€鏉″湪 Linux 涓凡缁忔嫢鏈夎嚜韬瓙绯荤粺鐨勫閮ㄦ€荤嚎锛堜緥濡?USB 鍜?ieee1394锛夈€傚湪杩欑鎯呭喌涓嬶紝SCSI 瀛愮郴缁熺殑 LLD 鏄€氬線鍙︿竴涓┍鍔ㄥ瓙绯荤粺鐨勮蒋浠舵ˉ銆備緥瀛愭湁 usb-storage 椹卞姩锛堜綅浜?drivers/usb/storage 鐩綍锛変互鍙?ieee1394/sbp2 椹卞姩锛堜綅浜?drivers/ieee1394 鐩綍锛夈€?

渚嬪锛宎ic7xxx LLD 鎺у埗鍩轰簬璇ュ叕鍙?7xxx 绯诲垪鑺墖鐨?Adaptec SCSI 骞惰鎺ュ彛锛圫PI锛夋帶鍒跺櫒銆俛ic7xxx LLD 鍙互缂栬瘧杩涘唴鏍告垨浣滀负妯″潡鍔犺浇銆備竴涓?Linux 绯荤粺涓彧鑳芥湁涓€涓?aic7xxx LLD 鍦ㄨ繍琛岋紝浣嗗畠鍙兘鎺у埗璁稿 HBA銆傝繖浜?HBA 鍙兘浣嶄簬 PCI 瀛愬崱涓婏紝鎴栭泦鎴愬湪涓绘澘涓婏紙鎴栦袱鑰呭吋鏈夛級銆傛煇浜涘熀浜?aic7xxx 鐨?HBA 鏄弻鎺у埗鍣紝鍥犳浠ｈ〃涓や釜涓绘満銆傚儚澶у鏁扮幇浠?HBA 涓€鏍凤紝姣忎釜 aic7xxx 涓绘満閮芥湁鑷繁鐨?PCI 璁惧鍦板潃銆俒SCSI 涓绘満涓?PCI 璁惧涔嬮棿鐨勪竴涓€瀵瑰簲鍏崇郴寰堝父瑙侊紝浣嗗苟闈炲繀闇€锛堜緥濡?ISA 閫傞厤鍣級銆俔

SCSI 涓棿灞傚皢 LLD 涓?SCSI 涓婂眰椹卞姩鍜屽潡灞傜瓑鍏朵粬灞傞殧绂诲紑鏉ャ€傛湰鏂囨。鐨勬鐗堟湰澶ц嚧瀵瑰簲 Linux 鍐呮牳鐗堟湰 2.6.8銆?


## 鏂囨。

鍐呮牳婧愮爜鏍戜腑鍖呭惈涓€涓?SCSI 鏂囨。鐩綍锛岄€氬父鏄?Documentation/scsi銆傚ぇ澶氭暟鏂囨。閲囩敤 reStructuredText 鏍煎紡銆傛湰鏂囦欢鍚嶄负 scsi_mid_low_api.rst锛屽彲鍦ㄨ鐩綍涓壘鍒般€傛湰鏂囨。杈冩柊鐨勫壇鏈彲鍦?https://docs.kernel.org/scsi/scsi_mid_low_api.html 鎵惧埌銆傝澶?LLD 鍦?Documentation/scsi 涓湁鏂囨。锛堜緥濡?aic7xxx.rst锛夈€係CSI 涓棿灞傚湪 scsi.rst 涓湁绠€瑕佽鏄庯紝鍏朵腑鍖呭惈鎻忚堪 Linux 鍐呮牳 2.4 绯诲垪 SCSI 瀛愮郴缁熺殑鏂囨。鐨?URL銆傝鐩綍涓湁涓や唤涓婂眰椹卞姩鐨勬枃妗ｏ細st.rst锛圫CSI 纾佸甫椹卞姩锛夊拰 scsi-generic.rst锛堥拡瀵?sg 椹卞姩锛夈€?

鏌愪簺 LLD 鐨勬枃妗ｏ紙鎴?URL锛夊彲浠ュ湪 C 婧愮爜涓壘鍒帮紝鎴栬€呭湪涓?C 婧愮爜鐩稿悓鐨勭洰褰曚腑鎵惧埌銆備緥濡傦紝瑕佹壘鍒板叧浜?USB 澶у閲忓瓨鍌ㄩ┍鍔ㄧ殑 URL锛岃鏌ョ湅 /usr/src/linux/drivers/usb/storage 鐩綍銆?


## 椹卞姩缁撴瀯

浼犵粺涓婏紝SCSI 瀛愮郴缁熺殑 LLD 鍦?drivers/scsi 鐩綍涓嚦灏戞湁涓や唤鏂囦欢銆備緥濡傦紝鍚嶄负 "xyz" 鐨勯┍鍔ㄦ湁涓€涓ご鏂囦欢 "xyz.h" 鍜屼竴涓簮鏂囦欢 "xyz.c"銆俒瀹為檯涓婃病鏈夊厖鍒嗙殑鐞嗙敱涓嶈兘鎶婃墍鏈夊唴瀹规斁鍦ㄤ竴涓枃浠朵腑锛涘ご鏂囦欢鏄浣欑殑銆俔涓€浜涘凡绉绘鍒板涓搷浣滅郴缁熺殑椹卞姩鏈夎秴杩囦袱浠芥枃浠躲€備緥濡?aic7xxx 椹卞姩鏈変负閫氱敤浠ｇ爜鍜岀壒瀹氫簬鎿嶄綔绯荤粺鐨勪唬鐮侊紙渚嬪 FreeBSD 鍜?Linux锛夊垎鍒噯澶囩殑鐙珛鏂囦欢銆傝繖绫婚┍鍔ㄥ線寰€鍦?drivers/scsi 鐩綍涓嬫嫢鏈夎嚜宸辩殑瀛愮洰褰曘€?

鍚?Linux 娣诲姞涓€涓柊鐨?LLD 鏃讹紝浠ヤ笅鏂囦欢锛堜綅浜?drivers/scsi 鐩綍涓級闇€瑕佸姞浠ユ敞鎰忥細Makefile 鍜?Kconfig銆傛渶濂芥槸鐮旂┒鐜版湁 LLD 鏄浣曠粍缁囩殑銆?

闅忕潃 2.5 绯诲垪寮€鍙戝唴鏍告紨杩涗负 2.6 绯诲垪鐢熶骇鍐呮牳锛屾鎺ュ彛涔熷湪鍙戠敓鍙樺寲銆傚叾涓竴涓緥瀛愬氨鏄┍鍔ㄥ垵濮嬪寲浠ｇ爜锛岀幇鍦ㄦ湁涓ょ妯″瀷鍙敤銆傝緝鏃х殑妯″瀷绫讳技浜?Linux 2.4 绯诲垪涓殑鍋氭硶锛屽熀浜庡湪 HBA 椹卞姩鍔犺浇鏃舵娴嬪埌鐨勪富鏈恒€傝繖琚О涓?琚姩锛坧assive锛?鍒濆鍖栨ā鍨嬨€傝緝鏂扮殑妯″瀷鍏佽鍦?LLD 鐨勭敓鍛藉懆鏈熷唴鐑彃鎷旓紙浠ュ強鐑嫈锛塇BA锛岃绉颁负"鐑彃鎷旓紙hotplug锛?鍒濆鍖栨ā鍨嬨€傝緝鏂扮殑妯″瀷鏇村彈闈掔潗锛屽洜涓哄畠鏃㈣兘澶勭悊姘镐箙杩炴帴鐨勪紶缁?SCSI 璁惧锛屼篃鑳藉鐞嗙儹鎻掓嫈鐨勭幇浠?SCSI"璁惧锛堜緥濡傞€氳繃 USB 鎴?IEEE 1394 杩炴帴鐨勬暟鐮佺浉鏈猴級銆備袱绉嶅垵濮嬪寲妯″瀷灏嗗湪鍚庣画鍚勮妭涓璁恒€?

LLD 閫氳繃浠ヤ笅鍑犵鏂瑰紡涓?SCSI 瀛愮郴缁熶氦浜掞細

  a) 鐩存帴璋冪敤涓棿灞傛彁渚涚殑鍑芥暟
  b) 鍚戜腑闂村眰鎻愪緵鐨勬敞鍐屽嚱鏁颁紶鍏ヤ竴缁勫嚱鏁版寚閽堛€備腑闂村眰闅忓悗浼氬湪灏嗘潵鐨勬煇涓椂鍒昏皟鐢ㄨ繖浜涘嚱鏁般€侺LD 闇€瑕佹彁渚涜繖浜涘嚱鏁扮殑瀹炵幇銆?
  c) 鐩存帴璁块棶鐢变腑闂村眰缁存姢鐨勭煡鍚嶆暟鎹粨鏋勫疄渚?

a) 缁勪腑鐨勫嚱鏁板湪涓嬫枃鍚嶄负"涓棿灞傛彁渚涚殑鍑芥暟"鐨勫皬鑺備腑鍒楀嚭銆?

b) 缁勪腑鐨勫嚱鏁板湪涓嬫枃鍚嶄负"鎺ュ彛鍑芥暟"鐨勫皬鑺備腑鍒楀嚭銆傚畠浠殑鍑芥暟鎸囬拡琚斁缃埌 "struct scsi_host_template" 鐨勬垚鍛樹腑锛岃缁撴瀯鐨勪竴涓疄渚嬩細琚紶鍏?scsi_host_alloc()銆傚浜庨偅浜?LLD 涓嶅笇鏈涙彁渚涚殑鎺ュ彛鍑芥暟锛屽簲鍦?struct scsi_host_template 鐨勭浉搴旀垚鍛樹腑濉叆 NULL銆傚湪鏂囦欢浣滅敤鍩熷畾涔?struct scsi_host_template 瀹炰緥浼氬鑷存湭鏄惧紡鍒濆鍖栫殑鍑芥暟鎸囬拡鎴愬憳琚～鍏?NULL銆?

c) 缁勪腑鐨勭敤娉曞簲褰撹皑鎱庡鐞嗭紝灏ゅ叾鏄湪"鐑彃鎷?鐜涓€侺LD 搴斿綋浜嗚В涓庝腑闂村眰鍜屽叾浠栧眰鍏变韩鐨勫疄渚嬬殑鐢熷懡鍛ㄦ湡銆?

LLD 鍐呭畾涔夌殑鎵€鏈夊嚱鏁颁互鍙婃枃浠朵綔鐢ㄥ煙瀹氫箟鐨勬墍鏈夋暟鎹兘搴斾负 static銆備緥濡傦紝鍚嶄负 "xxx" 鐨?LLD 涓殑 sdev_init() 鍑芥暟鍙互瀹氫箟涓?
`static int xxx_sdev_init(struct scsi_device ** sdev) { /** code */ }`


## 鐑彃鎷斿垵濮嬪寲妯″瀷

鍦ㄦ妯″瀷涓紝LLD 鎺у埗鐫€ SCSI 涓绘満浣曟椂琚紩鍏ュ拰浠?SCSI 瀛愮郴缁熺Щ闄ゃ€備富鏈烘渶鏃╁彲浠ュ湪椹卞姩鍒濆鍖栨椂寮曞叆锛屾渶鏅氬彲浠ュ湪椹卞姩鍏抽棴鏃剁Щ闄ゃ€傞€氬父锛岄┍鍔ㄤ細鍝嶅簲涓€涓?sysfs probe() 鍥炶皟锛岃鍥炶皟琛ㄧず妫€娴嬪埌涓€涓?HBA銆傚湪纭鏂拌澶囨槸 LLD 鎯宠鎺у埗鐨勮澶囧悗锛孡LD 浼氬垵濮嬪寲璇?HBA锛岀劧鍚庡悜 SCSI 涓棿灞傛敞鍐屼竴涓柊涓绘満銆?

鍦?LLD 鍒濆鍖栨湡闂达紝椹卞姩搴斿綋鍚戝畠鎵€鏈熸湜鎵惧埌 HBA 鐨勭浉搴?IO 鎬荤嚎锛堜緥濡?PCI 鎬荤嚎锛夋敞鍐岃嚜韬€傝繖澶ф鍙互閫氳繃 sysfs 瀹屾垚銆備换浣曢┍鍔ㄥ弬鏁帮紙灏ゅ叾鏄偅浜涘湪椹卞姩鍔犺浇鍚庝粛鍙啓鐨勫弬鏁帮級涔熷彲浠ュ湪杩欎竴姝ラ€氳繃 sysfs 娉ㄥ唽銆係CSI 涓棿灞傛槸鍦?LLD 娉ㄥ唽鍏剁涓€涓?HBA 鏃舵墠棣栨寰楃煡璇?LLD 鐨勫瓨鍦ㄣ€?

鍦ㄧ◢鍚庣殑鏌愪釜鏃跺埢锛孡LD 寰楃煡涓€涓?HBA锛屾帴涓嬫潵鏄?LLD 涓庝腑闂村眰涔嬮棿鍏稿瀷鐨勮皟鐢ㄥ簭鍒椼€傛绀轰緥灞曠ず浜嗕腑闂村眰涓烘柊寮曞叆鐨?HBA 鎵弿鍑?3

```
	HBA PROBE: assume 2 SCSI devices found in scan
    LLD                   mid level                    LLD
    ===-------------------=========--------------------===------
    scsi_host_alloc()  -->
    scsi_add_host()  ---->
    scsi_scan_host()  -------+
			    |
			sdev_init()
			sdev_configure() -->  scsi_change_queue_depth()
			    |
			sdev_init()
			sdev_configure()
			    |
			sdev_init()   ***
			sdev_destroy() ***


    *** For scsi devices that the mid level tries to scan but do not
	respond, a sdev_init(), sdev_destroy() pair is called.

```

濡傛灉 LLD 鎯宠皟鏁撮粯璁ら槦鍒楄缃紝鍙互鍦ㄥ叾 sdev_configure() 渚嬬▼涓皟鐢?scsi_change_queue_depth()銆?

褰?HBA 琚Щ闄ゆ椂锛岃繖鍙兘鏄笌 LLD 妯″潡琚嵏杞斤紙渚嬪浣跨敤 "rmmod" 鍛戒护锛夌浉鍏崇殑鏈夊簭鍏抽棴鐨勪竴閮ㄥ垎锛屼篃鍙兘鏄搷搴?sysfs 鐨?remove() 鍥炶皟琚皟鐢ㄦ墍琛ㄧず鐨?鐑嫈"銆傛棤璁哄摢绉嶆儏鍐碉紝搴忓垪閮芥槸

```
	    HBA REMOVE: assume 2 SCSI devices attached
    LLD                      mid level                 LLD
    ===----------------------=========-----------------===------
    scsi_remove_host() ---------+
				|
			sdev_destroy()
			sdev_destroy()
    scsi_host_put()

```

LLD 璺熻釜 struct Scsi_Host 瀹炰緥锛堟寚閽堢敱 scsi_host_alloc() 杩斿洖锛夊彲鑳芥槸鏈夌敤鐨勩€傛绫诲疄渚嬬敱涓棿灞?鎷ユ湁"銆傚綋寮曠敤璁℃暟闄嶄负闆舵椂锛宻truct Scsi_Host 瀹炰緥浼氬湪 scsi_host_put() 涓閲婃斁銆?

鐑嫈涓€涓帶鍒剁潃鍦ㄥ鐞嗗凡鎸傝浇鏂囦欢绯荤粺涓婄殑 SCSI 鍛戒护鐨勭鐩樼殑 HBA锛屾槸涓€绉嶆湁瓒ｇ殑鎯呭舰銆備腑闂村眰姝ｅ湪寮曞叆寮曠敤璁℃暟閫昏緫鏉ュ簲瀵规墍娑夊強鐨勮澶氶棶棰樸€傝鍙傞槄涓嬫枃鍏充簬寮曠敤璁℃暟鐨勫皬鑺傘€?


鐑彃鎷旂殑姒傚康鍙互鎵╁睍鍒?SCSI 璁惧銆傚綋鍓嶏紝褰撴坊鍔犱竴涓?HBA 鏃讹紝scsi_scan_host() 鍑芥暟浼氳Е鍙戝杩炴帴鍒拌 HBA 鐨?SCSI 浼犺緭灞傜殑 SCSI 璁惧鎵弿銆傚湪杈冩柊鐨?SCSI 浼犺緭灞備笂锛孒BA 鍙兘鍦ㄦ壂鎻忓畬鎴恄涔嬪悗_鎵嶅緱鐭ヤ竴涓柊鐨?SCSI 璁惧銆?

```
		    SCSI DEVICE hotplug
    LLD                   mid level                    LLD
    ===-------------------=========--------------------===------
    scsi_add_device()  ------+
			    |
			sdev_init()
			sdev_configure()   [--> scsi_change_queue_depth()]

```

绫讳技鍦帮紝LLD 鍙兘浼氬緱鐭ヤ竴涓?SCSI 璁惧宸茶绉婚櫎锛堟嫈鍑猴級锛屾垨鑰呭埌瀹冪殑杩炴帴宸茶涓柇銆備竴浜涚幇鏈夌殑 SCSI 浼犺緭灞傦紙渚嬪 SPI锛夊彲鑳界洿鍒板悗缁?SCSI 鍛戒护澶辫触鎵嶄細寰楃煡 SCSI 璁惧宸茶绉婚櫎锛岃€岃鍛戒护澶辫触寰堝彲鑳戒細瀵艰嚧涓棿灞傚皢璇ヨ澶囩疆涓虹绾裤€傛娴嬪埌 SCSI 璁惧琚Щ闄ょ殑 LLD 鍙互涓诲姩灏嗗叾浠?

```
		    SCSI DEVICE hot unplug
    LLD                      mid level                 LLD
    ===----------------------=========-----------------===------
    scsi_remove_device() -------+
				|
			sdev_destroy()

```

LLD 璺熻釜 struct scsi_device 瀹炰緥锛堟寚閽堜綔涓?sdev_init() 鍜?sdev_configure() 鍥炶皟鐨勫弬鏁颁紶鍏ワ級鍙兘鏄湁鐢ㄧ殑銆傛绫诲疄渚嬬敱涓棿灞?鎷ユ湁"銆俿truct scsi_device 瀹炰緥浼氬湪 sdev_destroy() 涔嬪悗琚噴鏀俱€?


## 寮曠敤璁℃暟

Scsi_Host 缁撴瀯宸茬粡娣诲姞浜嗗紩鐢ㄨ鏁板熀纭€璁炬柦銆傝繖瀹為檯涓婂皢 struct Scsi_Host 瀹炰緥鐨勬墍鏈夋潈鍒嗘暎鍒颁娇鐢ㄥ畠浠殑鍚勪釜 SCSI 灞傘€傛鍓嶆绫诲疄渚嬪畬鍏ㄧ敱涓棿灞傛嫢鏈夈€侺LD 閫氬父涓嶉渶瑕佺洿鎺ユ搷浣滆繖浜涘紩鐢ㄨ鏁帮紝浣嗗湪鏌愪簺鎯呭喌涓嬪彲鑳介渶瑕併€?

涓?struct Scsi_Host 鐩稿叧鐨勩€佸€煎緱鍏虫敞鐨勫紩鐢ㄨ鏁板嚱鏁版湁 3 涓細

  - scsi_host_alloc()锛?
	杩斿洖涓€涓寚鍚戞柊 struct Scsi_Host 瀹炰緥鐨勬寚閽堬紝鍏跺紩鐢ㄨ鏁?^^ 琚涓?1

  - scsi_host_get()锛?
	灏嗙粰瀹氬疄渚嬬殑寮曠敤璁℃暟鍔?1

  - scsi_host_put()锛?
	灏嗙粰瀹氬疄渚嬬殑寮曠敤璁℃暟鍑?1銆傚鏋滃紩鐢ㄨ鏁拌揪鍒?0锛屽垯閲婃斁璇ュ疄渚?

scsi_device 缁撴瀯宸茬粡娣诲姞浜嗗紩鐢ㄨ鏁板熀纭€璁炬柦銆傝繖瀹為檯涓婂皢 struct scsi_device 瀹炰緥鐨勬墍鏈夋潈鍒嗘暎鍒颁娇鐢ㄥ畠浠殑鍚勪釜 SCSI 灞傘€傛鍓嶆绫诲疄渚嬪畬鍏ㄧ敱涓棿灞傛嫢鏈夈€傝鍙傞槄 include/scsi/scsi_device.h 鏈熬澹版槑鐨勮闂嚱鏁般€傚鏋?LLD 鎯充繚鐣欎竴涓寚鍚?scsi_device 瀹炰緥鐨勬寚閽堝壇鏈紝瀹冨簲褰撲娇鐢?scsi_device_get() 鏉ュ鍔犲叾寮曠敤璁℃暟銆傚綋涓嶅啀闇€瑕佽鎸囬拡鏃讹紝鍙互浣跨敤 scsi_device_put() 鏉ュ噺灏戝叾寮曠敤璁℃暟锛堝苟鍙兘灏嗗叾鍒犻櫎锛夈€?


   struct Scsi_Host 瀹為檯涓婃湁 2 涓紩鐢ㄨ鏁帮紝鐢辫繖浜涘嚱鏁板苟琛屾搷浣溿€?


## 绾﹀畾

棣栧厛锛孡inus Torvalds 鍏充簬 C 缂栫爜椋庢牸鐨勭湅娉曞彲浠ュ湪 Documentation/process/coding-style.rst 鏂囦欢涓壘鍒般€?

姝ゅ锛屽湪澶у鏁扮浉鍏?gcc 缂栬瘧鍣ㄦ敮鎸佺殑绋嬪害涓婇紦鍔变娇鐢?C99 澧炲己鐗规€с€傚洜姝わ紝鍦ㄩ€傚綋鐨勫湴鏂归紦鍔变娇鐢?C99 椋庢牸鐨勭粨鏋勫拰鏁扮粍鍒濆鍖栧櫒銆備絾涓嶈澶繃鍒嗭紝鍙橀暱鏁扮粍锛圴LA锛夊皻鏈緱鍒板Ε鍠勬敮鎸併€傚姝ょ殑涓€涓緥澶栨槸 `//` 椋庢牸鐨勬敞閲婏紱鍦?Linux 涓粛鐒舵洿鍋忓ソ `/**...**/` 椋庢牸鐨勬敞閲娿€?

缂栧啓鑹ソ銆佺粡杩囨祴璇曚笖鏈夋枃妗ｇ殑浠ｇ爜锛屾棤闇€涓虹鍚堜笂杩扮害瀹氳€岄噸鏂版牸寮忓寲銆備緥濡傦紝aic7xxx 椹卞姩鏄粠 FreeBSD 鍜?Adaptec 鑷繁鐨勫疄楠屽鏉ュ埌 Linux 鐨勩€傛鏃犵枒闂紝FreeBSD 鍜?Adaptec 鏈夊畠浠嚜宸辩殑缂栫爜绾﹀畾銆?


## 涓棿灞傛彁渚涚殑鍑芥暟

杩欎簺鍑芥暟鐢?SCSI 涓棿灞傛彁渚涳紝渚?LLD 浣跨敤銆傝繖浜涘嚱鏁扮殑鍚嶇О锛堝嵆鍏ュ彛鐐癸級琚鍑猴紝鍥犳浣滀负妯″潡鐨?LLD 鍙互璁块棶瀹冧滑銆傚唴鏍镐細瀹夋帓鍦ㄤ换浣?LLD 鍒濆鍖栦箣鍓嶅姞杞藉苟鍒濆鍖?SCSI 涓棿灞傘€備互涓嬪嚱鏁版寜瀛楁瘝椤哄簭鍒楀嚭锛屽畠浠殑鍚嶇О閮戒互 `scsi_` 寮€澶淬€?

鎽樿锛?

  - scsi_add_device - 鍒涘缓涓€涓柊鐨?scsi 璁惧锛坙u锛夊疄渚?
  - scsi_add_host - 鎵ц sysfs 娉ㄥ唽骞惰缃紶杈撶被
  - scsi_change_queue_depth - 鏇存敼 SCSI 璁惧涓婄殑闃熷垪娣卞害
  - scsi_bios_ptable - 杩斿洖鍧楄澶囧垎鍖鸿〃鐨勫壇鏈?
  - scsi_block_requests - 闃绘鍚戠粰瀹氫富鏈烘帓鍏ユ洿澶氬懡浠?
  - scsi_host_alloc - 杩斿洖涓€涓?refcount==1 鐨勬柊 scsi_host 瀹炰緥
  - scsi_host_get - 閫掑 Scsi_Host 瀹炰緥鐨勫紩鐢ㄨ鏁?
  - scsi_host_put - 閫掑噺 Scsi_Host 瀹炰緥鐨勫紩鐢ㄨ鏁帮紙鑻ヤ负 0 鍒欓噴鏀撅級
  - scsi_remove_device - 鍒嗙骞剁Щ闄や竴涓?SCSI 璁惧
  - scsi_remove_host - 鍒嗙骞剁Щ闄や富鏈烘嫢鏈夌殑鎵€鏈?SCSI 璁惧
  - scsi_report_bus_reset - 鎶ュ憡瑙傚療鍒扮殑 scsi _鎬荤嚎_ 澶嶄綅
  - scsi_scan_host - 鎵弿 SCSI 鎬荤嚎
  - scsi_track_queue_full - 璺熻釜杩炵画鐨?QUEUE_FULL 浜嬩欢
  - scsi_unblock_requests - 鍏佽鍚戠粰瀹氫富鏈烘帓鍏ユ洿澶氬懡浠?


```

    /**
    * scsi_add_device - creates new scsi device (lu) instance
    * @shost:   pointer to scsi host instance
    * @channel: channel number (rarely other than 0)
    * @id:      target id number
    * @lun:     logical unit number
    *
    *      Returns pointer to new struct scsi_device instance or
    *      ERR_PTR(-ENODEV) (or some other bent pointer) if something is
    *      wrong (e.g. no lu responds at given address)
    *
    *      Might block: yes
    *
    *      Notes: This call is usually performed internally during a scsi
    *      bus scan when an HBA is added (i.e. scsi_scan_host()). So it
    *      should only be called if the HBA becomes aware of a new scsi
    *      device (lu) after scsi_scan_host() has completed. If successful
    *      this call can lead to sdev_init() and sdev_configure() callbacks
    *      into the LLD.
    *
    *      Defined in: drivers/scsi/scsi_scan.c
    **/
    struct scsi_device * scsi_add_device(struct Scsi_Host *shost,
					unsigned int channel,
					unsigned int id, unsigned int lun)


    /**
    * scsi_add_host - perform sysfs registration and set up transport class
    * @shost:   pointer to scsi host instance
    * @dev:     pointer to struct device of type scsi class
    *
    *      Returns 0 on success, negative errno of failure (e.g. -ENOMEM)
    *
    *      Might block: no
    *
    *      Notes: Only required in "hotplug initialization model" after a
    *      successful call to scsi_host_alloc().  This function does not
    *	scan the bus; this can be done by calling scsi_scan_host() or
    *	in some other transport-specific way.  The LLD must set up
    *	the transport template before calling this function and may only
    *	access the transport class data after this function has been called.
    *
    *      Defined in: drivers/scsi/hosts.c
    **/
    int scsi_add_host(struct Scsi_Host *shost, struct device * dev)


    /**
    * scsi_change_queue_depth - allow LLD to change queue depth on a SCSI device
    * @sdev:       pointer to SCSI device to change queue depth on
    * @tags        Number of tags allowed if tagged queuing enabled,
    *              or number of commands the LLD can queue up
    *              in non-tagged mode (as per cmd_per_lun).
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Notes: Can be invoked any time on a SCSI device controlled by this
    *      LLD. [Specifically during and after sdev_configure() and prior to
    *      sdev_destroy().] Can safely be invoked from interrupt code.
    *
    *      Defined in: drivers/scsi/scsi.c [see source code for more notes]
    *
    **/
    int scsi_change_queue_depth(struct scsi_device *sdev, int tags)


    /**
    * scsi_bios_ptable - return copy of block device's partition table
    * @dev:        pointer to gendisk
    *
    *      Returns pointer to partition table, or NULL for failure
    *
    *      Might block: yes
    *
    *      Notes: Caller owns memory returned (free with kfree() )
    *
    *      Defined in: drivers/scsi/scsicam.c
    **/
    unsigned char *scsi_bios_ptable(struct gendisk *dev)


    /**
    * scsi_block_requests - prevent further commands being queued to given host
    *
    * @shost: pointer to host to block commands on
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Notes: There is no timer nor any other means by which the requests
    *      get unblocked other than the LLD calling scsi_unblock_requests().
    *
    *      Defined in: drivers/scsi/scsi_lib.c
    **/
    void scsi_block_requests(struct Scsi_Host * shost)


    /**
    * scsi_host_alloc - create a scsi host adapter instance and perform basic
    *                   initialization.
    * @sht:        pointer to scsi host template
    * @privsize:   extra bytes to allocate in hostdata array (which is the
    *              last member of the returned Scsi_Host instance)
    *
    *      Returns pointer to new Scsi_Host instance or NULL on failure
    *
    *      Might block: yes
    *
    *      Notes: When this call returns to the LLD, the SCSI bus scan on
    *      this host has _not_ yet been done.
    *      The hostdata array (by default zero length) is a per host scratch
    *      area for the LLD's exclusive use.
    *      Both associated refcounting objects have their refcount set to 1.
    *      Full registration (in sysfs) and a bus scan are performed later when
    *      scsi_add_host() and scsi_scan_host() are called.
    *
    *      Defined in: drivers/scsi/hosts.c .
    **/
    struct Scsi_Host * scsi_host_alloc(const struct scsi_host_template * sht,
				    int privsize)


    /**
    * scsi_host_get - increment Scsi_Host instance refcount
    * @shost:   pointer to struct Scsi_Host instance
    *
    *      Returns nothing
    *
    *      Might block: currently may block but may be changed to not block
    *
    *      Notes: Actually increments the counts in two sub-objects
    *
    *      Defined in: drivers/scsi/hosts.c
    **/
    void scsi_host_get(struct Scsi_Host *shost)


    /**
    * scsi_host_put - decrement Scsi_Host instance refcount, free if 0
    * @shost:   pointer to struct Scsi_Host instance
    *
    *      Returns nothing
    *
    *      Might block: currently may block but may be changed to not block
    *
    *      Notes: Actually decrements the counts in two sub-objects. If the
    *      latter refcount reaches 0, the Scsi_Host instance is freed.
    *      The LLD need not worry exactly when the Scsi_Host instance is
    *      freed, it just shouldn't access the instance after it has balanced
    *      out its refcount usage.
    *
    *      Defined in: drivers/scsi/hosts.c
    **/
    void scsi_host_put(struct Scsi_Host *shost)


    /**
    * scsi_remove_device - detach and remove a SCSI device
    * @sdev:      a pointer to a scsi device instance
    *
    *      Returns value: 0 on success, -EINVAL if device not attached
    *
    *      Might block: yes
    *
    *      Notes: If an LLD becomes aware that a scsi device (lu) has
    *      been removed but its host is still present then it can request
    *      the removal of that scsi device. If successful this call will
    *      lead to the sdev_destroy() callback being invoked. sdev is an
    *      invalid pointer after this call.
    *
    *      Defined in: drivers/scsi/scsi_sysfs.c .
    **/
    int scsi_remove_device(struct scsi_device *sdev)


    /**
    * scsi_remove_host - detach and remove all SCSI devices owned by host
    * @shost:      a pointer to a scsi host instance
    *
    *      Returns value: 0 on success, 1 on failure (e.g. LLD busy ?锛?
    *
    *      Might block: yes
    *
    *      Notes: Should only be invoked if the "hotplug initialization
    *      model" is being used. It should be called _prior_ to
    *      calling scsi_host_put().
    *
    *      Defined in: drivers/scsi/hosts.c .
    **/
    int scsi_remove_host(struct Scsi_Host *shost)


    /**
    * scsi_report_bus_reset - report scsi _bus_ reset observed
    * @shost: a pointer to a scsi host involved
    * @channel: channel (within) host on which scsi bus reset occurred
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Notes: This only needs to be called if the reset is one which
    *      originates from an unknown location.  Resets originated by the
    *      mid level itself don't need to call this, but there should be
    *      no harm.  The main purpose of this is to make sure that a
    *      CHECK_CONDITION is properly treated.
    *
    *      Defined in: drivers/scsi/scsi_error.c .
    **/
    void scsi_report_bus_reset(struct Scsi_Host * shost, int channel)


    /**
    * scsi_scan_host - scan SCSI bus
    * @shost: a pointer to a scsi host instance
    *
    *	Might block: yes
    *
    *	Notes: Should be called after scsi_add_host()
    *
    *	Defined in: drivers/scsi/scsi_scan.c
    **/
    void scsi_scan_host(struct Scsi_Host *shost)


    /**
    * scsi_track_queue_full - track successive QUEUE_FULL events on given
    *                      device to determine if and when there is a need
    *                      to adjust the queue depth on the device.
    * @sdev:  pointer to SCSI device instance
    * @depth: Current number of outstanding SCSI commands on this device,
    *         not counting the one returned as QUEUE_FULL.
    *
    *      Returns 0  - no change needed
    *              >0 - adjust queue depth to this new depth
    *              -1 - drop back to untagged operation using host->cmd_per_lun
    *                   as the untagged command depth
    *
    *      Might block: no
    *
    *      Notes: LLDs may call this at any time and we will do "The Right
    *              Thing"; interrupt context safe.
    *
    *      Defined in: drivers/scsi/scsi.c .
    **/
    int scsi_track_queue_full(struct scsi_device *sdev, int depth)


    /**
    * scsi_unblock_requests - allow further commands to be queued to given host
    *
    * @shost: pointer to host to unblock commands on
    *
    *      Returns nothing
    *
    *      Might block: no
    *
    *      Defined in: drivers/scsi/scsi_lib.c .
    **/
    void scsi_unblock_requests(struct Scsi_Host * shost)



```
## 鎺ュ彛鍑芥暟

鎺ュ彛鍑芥暟鐢?LLD 鎻愪緵锛堝畾涔夛級锛屽畠浠殑鍑芥暟鎸囬拡琚斁缃埌 struct scsi_host_template 鐨勪竴涓疄渚嬩腑锛岃瀹炰緥浼氳浼犲叆 scsi_host_alloc()銆傚叾涓竴浜涙槸蹇呴渶鐨勩€傛帴鍙ｅ嚱鏁板簲澹版槑涓?static銆傚叕璁ょ殑绾﹀畾鏄紝椹卞姩 "xyz" 浼氬０鏄庡畠鐨?sdev_configure()

```
    static int xyz_sdev_configure(struct scsi_device * sdev);

```

涓嬫枃鍒楀嚭鐨勬墍鏈夋帴鍙ｅ嚱鏁颁互姝ょ被鎺ㄣ€傛寚鍚戣鍑芥暟鐨勬寚閽堝簲琚斁鍏?"struct scsi_host_template" 瀹炰緥鐨?'sdev_configure' 鎴愬憳涓€傛寚鍚戞绫诲疄渚嬬殑鎸囬拡搴旇浼犲叆涓棿灞傜殑 scsi_host_alloc()銆?

鎺ュ彛鍑芥暟涔熷湪 include/scsi/scsi_host.h 鏂囦欢涓€佷綅浜?"struct scsi_host_template" 涓畠浠畾涔夌偣鐨勪笂鏂规湁鎻忚堪銆傚湪鏌愪簺鎯呭喌涓嬶紝scsi_host.h 涓粰鍑虹殑缁嗚妭姣斾笅鏂囨洿澶氥€?

鎺ュ彛鍑芥暟鎸夊瓧姣嶉『搴忓垪鍦ㄤ笅鏂广€?

鎽樿锛?

  - bios_param - 鑾峰彇纾佺洏鐨勭澶淬€佹墖鍖恒€佹煴闈俊鎭?
  - eh_timed_out - 閫氱煡涓绘満鏌愪釜鍛戒护鐨勫畾鏃跺櫒宸茶秴鏃?
  - eh_abort_handler - 涓缁欏畾鐨勫懡浠?
  - eh_bus_reset_handler - 鍙戣捣 SCSI 鎬荤嚎澶嶄綅
  - eh_device_reset_handler - 鍙戣捣 SCSI 璁惧澶嶄綅
  - eh_host_reset_handler - 澶嶄綅涓绘満锛堜富鏈烘€荤嚎閫傞厤鍣級
  - info - 鎻愪緵鍏充簬缁欏畾涓绘満鐨勪俊鎭?
  - ioctl - 椹卞姩鍙互鍝嶅簲 ioctl
  - proc_info - 鏀寔 /proc/scsi/{driver_name}/{host_no}
  - queuecommand - 灏?scsi 鍛戒护鍏ラ槦锛屽畬鎴愭椂璋冪敤 'done'
  - sdev_init - 鍦ㄥ悜鏂拌澶囧彂閫佷换浣曞懡浠や箣鍓?
  - sdev_configure - 璁惧杩炴帴鍚庨拡瀵圭粰瀹氳澶囩殑椹卞姩寰皟
  - sdev_destroy - 缁欏畾璁惧鍗冲皢鍏抽棴


```

    /**
    *      bios_param - fetch head, sector, cylinder info for a disk
    *      @sdev: pointer to scsi device context (defined in
    *             include/scsi/scsi_device.h)
    *      @disk: pointer to gendisk (defined in blkdev.h)
    *      @capacity:  device size (in 512 byte sectors)
    *      @params: three element array to place output:
    *              params[0] number of heads (max 255)
    *              params[1] number of sectors (max 63)
    *              params[2] number of cylinders
    *
    *      Return value is ignored
    *
    *      Locks: none
    *
    *      Calling context: process (sd)
    *
    *      Notes: an arbitrary geometry (based on READ CAPACITY) is used
    *      if this function is not provided. The params array is
    *      pre-initialized with made up values just in case this function
    *      doesn't output anything.
    *
    *      Optionally defined in: LLD
    **/
	int bios_param(struct scsi_device * sdev, struct gendisk *disk,
		    sector_t capacity, int params[3])


    /**
    *      eh_timed_out - The timer for the command has just fired
    *      @scp: identifies command timing out
    *
    *      Returns:
    *
    *      EH_HANDLED:             I fixed the error, please complete the command
    *      EH_RESET_TIMER:         I need more time, reset the timer and
    *                              begin counting again
    *      EH_NOT_HANDLED          Begin normal error recovery
    *
    *
    *      Locks: None held
    *
    *      Calling context: interrupt
    *
    *      Notes: This is to give the LLD an opportunity to do local recovery.
    *      This recovery is limited to determining if the outstanding command
    *      will ever complete.  You may not abort and restart the command from
    *      this callback.
    *
    *      Optionally defined in: LLD
    **/
	int eh_timed_out(struct scsi_cmnd * scp)


    /**
    *      eh_abort_handler - abort command associated with scp
    *      @scp: identifies command to be aborted
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: This is called only for a command that has timed out.
    *
    *      Optionally defined in: LLD
    **/
	int eh_abort_handler(struct scsi_cmnd * scp)


    /**
    *      eh_bus_reset_handler - issue SCSI bus reset
    *      @scp: SCSI bus that contains this device should be reset
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: Invoked from scsi_eh thread. No other commands will be
    *      queued on current host during eh.
    *
    *      Optionally defined in: LLD
    **/
	int eh_bus_reset_handler(struct scsi_cmnd * scp)


    /**
    *      eh_device_reset_handler - issue SCSI device reset
    *      @scp: identifies SCSI device to be reset
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: Invoked from scsi_eh thread. No other commands will be
    *      queued on current host during eh.
    *
    *      Optionally defined in: LLD
    **/
	int eh_device_reset_handler(struct scsi_cmnd * scp)


    /**
    *      eh_host_reset_handler - reset host (host bus adapter)
    *      @scp: SCSI host that contains this device should be reset
    *
    *      Returns SUCCESS if command aborted else FAILED
    *
    *      Locks: None held
    *
    *      Calling context: kernel thread
    *
    *      Notes: Invoked from scsi_eh thread. No other commands will be
    *      queued on current host during eh.
    *      With the default eh_strategy in place, if none of the _abort_,
    *      _device_reset_, _bus_reset_ or this eh handler function are
    *      defined (or they all return FAILED) then the device in question
    *      will be set offline whenever eh is invoked.
    *
    *      Optionally defined in: LLD
    **/
	int eh_host_reset_handler(struct scsi_cmnd * scp)


    /**
    *      info - supply information about given host: driver name plus data
    *             to distinguish given host
    *      @shp: host to supply information about
    *
    *      Return ASCII null terminated string. [This driver is assumed to
    *      manage the memory pointed to and maintain it, typically for the
    *      lifetime of this host.]
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Often supplies PCI or ISA information such as IO addresses
    *      and interrupt numbers. If not supplied struct Scsi_Host::name used
    *      instead. It is assumed the returned information fits on one line
    *      (i.e. does not included embedded newlines).
    *      The SCSI_IOCTL_PROBE_HOST ioctl yields the string returned by this
    *      function (or struct Scsi_Host::name if this function is not
    *      available).
    *
    *      Optionally defined in: LLD
    **/
	const char * info(struct Scsi_Host * shp)


    /**
    *      ioctl - driver can respond to ioctls
    *      @sdp: device that ioctl was issued for
    *      @cmd: ioctl number
    *      @arg: pointer to read or write data from. Since it points to
    *            user space, should use appropriate kernel functions
    *            (e.g. copy_from_user() ). In the Unix style this argument
    *            can also be viewed as an unsigned long.
    *
    *      Returns negative "errno" value when there is a problem. 0 or a
    *      positive value indicates success and is returned to the user space.
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: The SCSI subsystem uses a "trickle down" ioctl model.
    *      The user issues an ioctl() against an upper level driver
    *      (e.g. /dev/sdc) and if the upper level driver doesn't recognize
    *      the 'cmd' then it is passed to the SCSI mid level. If the SCSI
    *      mid level does not recognize it, then the LLD that controls
    *      the device receives the ioctl. According to recent Unix standards
    *      unsupported ioctl() 'cmd' numbers should return -ENOTTY.
    *
    *      Optionally defined in: LLD
    **/
	int ioctl(struct scsi_device *sdp, int cmd, void *arg)


    /**
    *      proc_info - supports /proc/scsi/{driver_name}/{host_no}
    *      @buffer: anchor point to output to (0==writeto1_read0) or fetch from
    *               (1==writeto1_read0).
    *      @start: where "interesting" data is written to. Ignored when
    *              1==writeto1_read0.
    *      @offset: offset within buffer 0==writeto1_read0 is actually
    *               interested in. Ignored when 1==writeto1_read0 .
    *      @length: maximum (or actual) extent of buffer
    *      @host_no: host number of interest (struct Scsi_Host::host_no)
    *      @writeto1_read0: 1 -> data coming from user space towards driver
    *                            (e.g. "echo some_string > /proc/scsi/xyz/2")
    *                       0 -> user what data from this driver
    *                            (e.g. "cat /proc/scsi/xyz/2")
    *
    *      Returns length when 1==writeto1_read0. Otherwise number of chars
    *      output to buffer past offset.
    *
    *      Locks: none held
    *
    *      Calling context: process
    *
    *      Notes: Driven from scsi_proc.c which interfaces to proc_fs. proc_fs
    *      support can now be configured out of the scsi subsystem.
    *
    *      Optionally defined in: LLD
    **/
	int proc_info(char * buffer, char ** start, off_t offset,
		    int length, int host_no, int writeto1_read0)


    /**
    *      queuecommand - queue scsi command, invoke scp->scsi_done on completion
    *      @shost: pointer to the scsi host object
    *      @scp: pointer to scsi command object
    *
    *      Returns 0 on success.
    *
    *      If there's a failure, return either:
    *
    *      SCSI_MLQUEUE_DEVICE_BUSY if the device queue is full, or
    *      SCSI_MLQUEUE_HOST_BUSY if the entire host queue is full
    *
    *      On both of these returns, the mid-layer will requeue the I/O
    *
    *      - if the return is SCSI_MLQUEUE_DEVICE_BUSY, only that particular
    *      device will be paused, and it will be unpaused when a command to
    *      the device returns (or after a brief delay if there are no more
    *      outstanding commands to it).  Commands to other devices continue
    *      to be processed normally.
    *
    *      - if the return is SCSI_MLQUEUE_HOST_BUSY, all I/O to the host
    *      is paused and will be unpaused when any command returns from
    *      the host (or after a brief delay if there are no outstanding
    *      commands to the host).
    *
    *      For compatibility with earlier versions of queuecommand, any
    *      other return value is treated the same as
    *      SCSI_MLQUEUE_HOST_BUSY.
    *
    *      Other types of errors that are detected immediately may be
    *      flagged by setting scp->result to an appropriate value,
    *      invoking the scp->scsi_done callback, and then returning 0
    *      from this function. If the command is not performed
    *      immediately (and the LLD is starting (or will start) the given
    *      command) then this function should place 0 in scp->result and
    *      return 0.
    *
    *      Command ownership.  If the driver returns zero, it owns the
    *      command and must take responsibility for ensuring the
    *      scp->scsi_done callback is executed.  Note: the driver may
    *      call scp->scsi_done before returning zero, but after it has
    *      called scp->scsi_done, it may not return any value other than
    *      zero.  If the driver makes a non-zero return, it must not
    *      execute the command's scsi_done callback at any time.
    *
    *      Locks: up to and including 2.6.36, struct Scsi_Host::host_lock
    *             held on entry (with "irqsave") and is expected to be
    *             held on return. From 2.6.37 onwards, queuecommand is
    *             called without any locks held.
    *
    *      Calling context: in interrupt (soft irq) or process context
    *
    *      Notes: This function should be relatively fast. Normally it
    *      will not wait for IO to complete. Hence the scp->scsi_done
    *      callback is invoked (often directly from an interrupt service
    *      routine) some time after this function has returned. In some
    *      cases (e.g. pseudo adapter drivers that manufacture the
    *      response to a SCSI INQUIRY) the scp->scsi_done callback may be
    *      invoked before this function returns.  If the scp->scsi_done
    *      callback is not invoked within a certain period the SCSI mid
    *      level will commence error processing.  If a status of CHECK
    *      CONDITION is placed in "result" when the scp->scsi_done
    *      callback is invoked, then the LLD driver should perform
    *      autosense and fill in the struct scsi_cmnd::sense_buffer
    *      array. The scsi_cmnd::sense_buffer array is zeroed prior to
    *      the mid level queuing a command to an LLD.
    *
    *      Defined in: LLD
    **/
	enum scsi_qc_status queuecommand(struct Scsi_Host *shost,
					 struct scsi_cmnd *scp)


    /**
    *      sdev_init -   prior to any commands being sent to a new device
    *                      (i.e. just prior to scan) this call is made
    *      @sdp: pointer to new device (about to be scanned)
    *
    *      Returns 0 if ok. Any other return is assumed to be an error and
    *      the device is ignored.
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Allows the driver to allocate any resources for a device
    *      prior to its initial scan. The corresponding scsi device may not
    *      exist but the mid level is just about to scan for it (i.e. send
    *      and INQUIRY command plus ...). If a device is found then
    *      sdev_configure() will be called while if a device is not found
    *      sdev_destroy() is called.
    *      For more details see the include/scsi/scsi_host.h file.
    *
    *      Optionally defined in: LLD
    **/
	int sdev_init(struct scsi_device *sdp)


    /**
    *      sdev_configure - driver fine tuning for given device just after it
    *                     has been first scanned (i.e. it responded to an
    *                     INQUIRY)
    *      @sdp: device that has just been attached
    *
    *      Returns 0 if ok. Any other return is assumed to be an error and
    *      the device is taken offline. [offline devices will _not_ have
    *      sdev_destroy() called on them so clean up resources.]
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Allows the driver to inspect the response to the initial
    *      INQUIRY done by the scanning code and take appropriate action.
    *      For more details see the include/scsi/scsi_host.h file.
    *
    *      Optionally defined in: LLD
    **/
	int sdev_configure(struct scsi_device *sdp)


    /**
    *      sdev_destroy - given device is about to be shut down. All
    *                      activity has ceased on this device.
    *      @sdp: device that is about to be shut down
    *
    *      Returns nothing
    *
    *      Locks: none
    *
    *      Calling context: process
    *
    *      Notes: Mid level structures for given device are still in place
    *      but are about to be torn down. Any per device resources allocated
    *      by this driver for given device should be freed now. No further
    *      commands will be sent for this sdp instance. [However the device
    *      could be re-attached in the future in which case a new instance
    *      of struct scsi_device would be supplied by future sdev_init()
    *      and sdev_configure() calls.]
    *
    *      Optionally defined in: LLD
    **/
	void sdev_destroy(struct scsi_device *sdp)



```
## 鏁版嵁缁撴瀯

### struct scsi_host_template

姣忎釜 LLD 鏈変竴涓?"struct scsi_host_template" 瀹炰緥 [#]_銆傚畠閫氬父浣滀负椹卞姩澶存枃浠朵腑鐨勬枃浠朵綔鐢ㄥ煙 static 琚垵濮嬪寲銆傝繖鏍凤紝鏈樉寮忓垵濮嬪寲鐨勬垚鍛樹細琚涓?0 鎴?NULL銆傚€煎緱鍏虫敞鐨勬垚鍛橈細

    name
   - 椹卞姩鍚嶇О锛堝彲鍖呭惈绌烘牸锛岃闄愬埗鍦?80 涓瓧绗︿互鍐咃級

    proc_name
   - 鐢ㄤ簬 "/proc/scsi/<proc_name>/<host_no>" 鐨勫悕绉帮紝涔熺敱 sysfs 鍦ㄥ叾鏌愪釜 "drivers" 鐩綍涓娇鐢ㄣ€傚洜姝?"proc_name" 鍙兘鍖呭惈 Unix 鏂囦欢鍚嶅彲鎺ュ彈鐨勫瓧绗︺€?

   `(*queuecommand)()`
   - 涓棿灞傜敤鏉ュ悜 LLD 娉ㄥ叆 SCSI 鍛戒护鐨勪富瑕佸洖璋冦€?

    vendor_id
   - 涓€涓敮涓€鍊硷紝鐢ㄤ簬鏍囪瘑涓?Scsi_Host 鎻愪緵 LLD 鐨勫巶鍟嗐€傛渶甯哥敤浜庢牎楠屽巶鍟嗙壒瀹氱殑娑堟伅璇锋眰銆傚€肩敱涓€涓爣璇嗙绫诲瀷鍜屼竴涓巶鍟嗙壒瀹氬€肩粍鎴愩€傛湁鏁堟牸寮忚鏄庤 scsi_netlink.h銆?

璇ョ粨鏋勫湪 include/scsi/scsi_host.h 涓畾涔夊苟闄勬湁娉ㄩ噴

      濡傛灉瀹冩帶鍒跺嚑绫讳笉鍚岀殑纭欢锛堜緥濡備竴涓悓鏃跺鐞?ISA 鍜?PCI 鍗°€佸苟涓烘瘡绫荤‖浠跺崟鐙噯澶囦竴浠?struct scsi_host_template 瀹炰緥鐨?LLD锛夈€?

### struct Scsi_Host

LLD 鎺у埗鐨勬瘡涓富鏈猴紙HBA锛夋湁涓€涓?struct Scsi_Host 瀹炰緥銆俿truct Scsi_Host 缁撴瀯涓?"struct scsi_host_template" 鏈夎澶氬叡鍚屾垚鍛樸€傚綋鍒涘缓涓€涓柊鐨?struct Scsi_Host 瀹炰緥鏃讹紙鍦?hosts.c 鐨?scsi_host_alloc() 涓級锛岄偅浜涘叡鍚屾垚鍛樹細浠庨┍鍔ㄧ殑 struct scsi_host_template 瀹炰緥鍒濆鍖栬€屾潵銆傚€煎緱鍏虫敞鐨勬垚鍛橈細

    host_no
   - 绯荤粺鑼冨洿鍐呭敮涓€鐨勭紪鍙凤紝鐢ㄤ簬鏍囪瘑姝や富鏈恒€備粠 0 寮€濮嬫寜鍗囧簭鍒嗛厤銆?
    can_queue
   - 蹇呴』澶т簬 0锛涗笉瑕佸悜閫傞厤鍣ㄥ彂閫佽秴杩?can_queue 鏉″懡浠ゃ€?
    this_id
   - 涓绘満鐨?scsi id锛坰csi 鍙戣捣鑰咃級锛岃嫢鏈煡鍒欎负 -1
    sg_tablesize
   - 涓绘満鍏佽鐨勬渶澶у垎鏁?鑱氶泦锛坰catter gather锛夊厓绱犳暟閲忋€傚皢鍏惰涓?SG_ALL 鎴栨洿灏忎互閬垮厤閾惧紡 SG 鍒楄〃銆傚繀椤昏嚦灏戜负 1銆?
    max_sectors
   - 鍗曟潯 SCSI 鍛戒护鍏佽鐨勬渶澶ф墖鍖烘暟锛堥€氬父涓?512 瀛楄妭锛夈€傞粯璁ゅ€?0 浼氬鑷磋缃负 SCSI_DEFAULT_MAX_SECTORS锛堝湪 scsi_host.h 涓畾涔夛級锛屽綋鍓嶈涓?1024銆傚洜姝ゅ綋鏈畾涔?max_sectors 鏃讹紝纾佺洏鐨勬渶澶т紶杈撳ぇ灏忎负 512 KB銆傛敞鎰忔澶у皬鍙兘涓嶈冻浠ヨ繘琛岀鐩樺浐浠朵笂浼犮€?
    cmd_per_lun
   - 涓绘満鎺у埗鐨勮澶囦笂鍙互鎺掗槦鐨勬渶澶у懡浠ゆ暟銆備細琚?LLD 瀵?scsi_change_queue_depth() 鐨勮皟鐢ㄨ鐩栥€?
    hostt
   - 鎸囧悜鐢熸垚姝?struct Scsi_Host 瀹炰緥鐨勯┍鍔?struct scsi_host_template 鐨勬寚閽?
    hostt->proc_name
   - LLD 鐨勫悕绉般€傝繖鏄?sysfs 浣跨敤鐨勯┍鍔ㄥ悕绉般€?
    transportt
   - 鎸囧悜椹卞姩 struct scsi_transport_template 瀹炰緥鐨勬寚閽堬紙濡傛灉鏈夛級銆傚綋鍓嶆敮鎸?FC 鍜?SPI 浼犺緭灞傘€?
    hostdata[^0^]
   - 鍦?struct Scsi_Host 鏈熬涓?LLD 淇濈暀鐨勫尯鍩熴€傚ぇ灏忕敱浼犲叆 scsi_host_alloc() 鐨勭浜屼釜鍙傛暟锛堝悕涓?'privsize'锛夎缃€?

scsi_host 缁撴瀯鍦?include/scsi/scsi_host.h 涓畾涔?

### struct scsi_device

閫氬父锛屼富鏈轰笂姣忎釜 SCSI 閫昏緫鍗曞厓閮芥湁涓€涓缁撴瀯鐨勫疄渚嬨€傝繛鎺ュ埌涓绘満鐨?SCSI 璁惧鐢遍€氶亾鍙枫€佺洰鏍?id 鍜岄€昏緫鍗曞厓鍙凤紙lun锛夊敮涓€鏍囪瘑銆傝缁撴瀯鍦?include/scsi/scsi_device.h 涓畾涔夈€?

### struct scsi_cmnd

姝ょ粨鏋勭殑瀹炰緥灏?SCSI 鍛戒护浼犻€掔粰 LLD锛屽苟灏嗗搷搴旇繑鍥炵粰涓棿灞傘€係CSI 涓棿灞備細纭繚鎺掑叆 LLD 鐨?SCSI 鍛戒护涓嶈秴杩?**scsi_change_queue_depth()锛堟垨 struct Scsi_Host**锛歝md_per_lun锛夋墍鎸囩ず鐨勬暟閲忋€傛瘡涓?SCSI 璁惧鑷冲皯浼氭湁涓€涓?struct scsi_cmnd 瀹炰緥鍙敤銆傚€煎緱鍏虫敞鐨勬垚鍛橈細

    cmnd
   - 鍖呭惈 SCSI 鍛戒护鐨勬暟缁?
    cmd_len
   - SCSI 鍛戒护鐨勯暱搴︼紙瀛楄妭锛?
    sc_data_direction
   - 鏁版嵁闃舵鏁版嵁浼犺緭鐨勬柟鍚戙€傚弬瑙?include/linux/dma-mapping.h 涓殑 "enum dma_data_direction"
    result
   - 搴斿湪璋冪敤 'done' 涔嬪墠鐢?LLD 璁剧疆銆傚€?0 琛ㄧず鍛戒护鎴愬姛瀹屾垚锛堜笖鎵€鏈夋暟鎹紙濡傛灉鏈夛級宸蹭紶鍚戞垨浠?SCSI 鐩爣璁惧浼犲嚭锛夈€?result' 鏄竴涓?32 浣嶆棤绗﹀彿鏁存暟锛屽彲瑙嗕负涓や釜鐩稿叧鐨勫瓧鑺傘€係CSI 鐘舵€佸€煎湪鏈€浣庡瓧鑺傦紙LSB锛変腑銆傚弬瑙?include/scsi/scsi.h 涓殑 status_byte() 鍜?host_byte() 瀹忓強鐩稿叧甯搁噺銆?
    sense_buffer
   - 涓€涓暟缁勶紙鏈€澶уぇ灏忥細SCSI_SENSE_BUFFERSIZE 瀛楄妭锛夛紝褰?SCSI 鐘舵€侊紙'result' 鐨?LSB锛夎璁句负 CHECK_CONDITION (2) 鏃跺簲琚啓鍏ャ€傚綋璁剧疆浜?CHECK_CONDITION 鏃讹紝濡傛灉 sense_buffer[^0^] 鐨勯珮鍗婂瓧鑺傚€间负 7锛屽垯涓棿灞備細鍋囧畾 sense_buffer 鏁扮粍鍖呭惈鏈夋晥鐨?SCSI sense 缂撳啿锛涘惁鍒欎腑闂村眰浼氬彂鍑轰竴鏉?REQUEST_SENSE SCSI 鍛戒护鏉ュ彇鍥?sense 缂撳啿銆傚悗涓€绉嶇瓥鐣ュ湪瀛樺湪鍛戒护鎺掗槦鏃跺鏄撳嚭閿欙紝鍥犳 LLD 搴斿綋濮嬬粓"鑷姩鎰熺煡锛坅uto-sense锛?銆?
    device
   - 鎸囧悜姝ゅ懡浠ゆ墍鍏宠仈鐨?scsi_device 瀵硅薄鐨勬寚閽堛€?
    resid_len   锛堥€氳繃璋冪敤 scsi_set_resid() / scsi_get_resid() 璁块棶锛?
   - LLD 搴斿皢姝ゆ棤绗﹀彿鏁存暟璁句负璇锋眰鐨勪紶杈撻暱搴︼紙鍗?'request_bufflen'锛夊噺鍘诲疄闄呬紶杈撶殑瀛楄妭鏁般€?resid_len' 棰勮涓?0锛屽洜姝ゅ鏋?LLD 鏃犳硶妫€娴嬫瑺杞斤紙涓嶅簲鎶ュ憡杩囪浇锛夛紝鍙互蹇界暐瀹冦€侺LD 搴斿湪璋冪敤 'done' 涔嬪墠璁剧疆 'resid_len'銆傛渶鍊煎緱鍏虫敞鐨勬儏褰㈡槸浠?SCSI 鐩爣璁惧锛堜緥濡?READ锛変紶杈撳嚭鏉ョ殑銆佸彂鐢熸瑺杞界殑鏁版嵁浼犺緭銆?
    underflow
   - 濡傛灉瀹為檯浼犺緭鐨勫瓧鑺傛暟灏忎簬姝ゅ€硷紝LLD 搴斿皢 (DID_ERROR << 16) 鏀惧叆 'result'銆傚疄鐜版妫€鏌ョ殑 LLD 涓嶅锛岃€屽叾涓竴浜涘彧鏄悜鏃ュ織杈撳嚭涓€鏉￠敊璇秷鎭紝鑰屼笉鏄姤鍛?DID_ERROR銆侺LD 鏈€濂藉疄鐜?'resid_len'銆?

寤鸿 LLD 鍦ㄦ潵鑷?SCSI 鐩爣璁惧锛堜緥濡?READ锛夌殑鏁版嵁浼犺緭涓婅缃?'resid_len'銆傚綋杩欑被鏁版嵁浼犺緭鍏锋湁 MEDIUM ERROR 鍜?HARDWARE ERROR锛堜互鍙婂彲鑳界殑 RECOVERED ERROR锛夌殑 sense 閿椂锛岃缃?'resid_len' 灏や负閲嶈銆傚湪杩欎簺鎯呭喌涓嬶紝濡傛灉 LLD 涓嶇‘瀹氬凡鎺ユ敹鍒板灏戞暟鎹紝鏈€瀹夊叏鐨勫仛娉曟槸琛ㄦ槑娌℃湁鎺ユ敹鍒颁换浣曞瓧鑺傘€備緥濡傦細瑕佽〃鏄庢病鏈夋帴鏀跺埌鏈夋晥鏁版嵁

```
    scsi_set_resid(SCpnt, scsi_bufflen(SCpnt));

```

鍏朵腑 'SCpnt' 鏄寚鍚?scsi_cmnd 瀵硅薄鐨勬寚閽堛€傝琛ㄦ槑浠呮湁涓変釜 512

```
    scsi_set_resid(SCpnt, scsi_bufflen(SCpnt) - (3 * 512));

```

scsi_cmnd 缁撴瀯鍦?include/scsi/scsi_cmnd.h 涓畾涔?


## 閿?

姣忎釜 struct Scsi_Host 瀹炰緥閮芥湁涓€涓悕涓?struct
**Scsi_Host**
: default_lock 鐨勮嚜鏃嬮攣锛屽湪 scsi_host_alloc() [浣嶄簬
**hosts.c] 涓垵濮嬪寲銆傚湪鍚屼竴鍑芥暟涓紝struct Scsi_Host**
: host_lock 鎸囬拡
琚垵濮嬪寲涓烘寚鍚?default_lock銆傛鍚庯紝涓棿灞傛墽琛岀殑鍔犻攣涓庤В閿?
**鎿嶄綔浣跨敤 struct Scsi_Host**
: host_lock
鎸囬拡銆備互鍓嶉┍鍔ㄥ彲浠ヨ鐩?host_lock 鎸囬拡锛屼絾鐜板湪涓嶅啀鍏佽銆?


## 鑷姩鎰熺煡锛圓utosense锛?

鑷姩鎰熺煡锛圓utosense锛屾垨 auto-sense锛夊湪 SAM-2 鏂囨。涓瀹氫箟涓猴細褰撳彂鐢?CHECK CONDITION 鐘舵€佹椂锛?鍦?SCSI 鍛戒护瀹屾垚鏃惰嚜鍔ㄥ皢 sense 鏁版嵁杩斿洖缁欏簲鐢ㄧ▼搴忓鎴风"銆侺LD 搴斿綋鎵ц鑷姩鎰熺煡銆傝繖搴斿湪 LLD 妫€娴嬪埌 CHECK CONDITION 鐘舵€佹椂閫氳繃浠ヤ笅浠讳竴鏂瑰紡瀹屾垚锛?

    a) 鎸囩ず SCSI 鍗忚锛堜緥濡?SCSI 骞惰鎺ュ彛锛圫PI锛夛級瀵规绫诲搷搴旀墽琛屼竴涓澶栫殑鏁版嵁杈撳叆闃舵
    b) 鎴栬€咃紝鐢?LLD 鑷繁鍙戝嚭涓€鏉?REQUEST SENSE 鍛戒护

鏃犺鍝鏂瑰紡锛屽綋妫€娴嬪埌 CHECK CONDITION 鐘舵€佹椂锛屼腑闂村眰閫氳繃妫€鏌?struct
**scsi_cmnd**
: sense_buffer[^0^] 鏉ュ垽鏂?LLD 鏄惁宸叉墽琛岃嚜鍔ㄦ劅鐭ャ€傚鏋滆瀛楄妭鐨勯珮鍗婂瓧鑺備负 7锛堟垨 0xf锛夛紝鍒欏亣瀹氬凡鎵ц鑷姩鎰熺煡銆傚鏋滃畠鏄叾浠栧€硷紙骞朵笖璇ュ瓧鑺傚湪姣忔潯鍛戒护涔嬪墠琚垵濮嬪寲涓?0锛夛紝鍒欎腑闂村眰浼氬彂鍑轰竴鏉?REQUEST SENSE 鍛戒护銆?

鍦ㄥ瓨鍦ㄦ帓闃熷懡浠ょ殑鎯呭喌涓嬶紝缁存姢澶辫触鍛戒护鐨?sense 缂撳啿鏁版嵁鐩村埌鍚庣画 REQUEST SENSE 鐨?nexus"鍙兘浼氬け鍘诲悓姝ャ€傝繖灏辨槸涓轰粈涔?LLD 鏈€濂芥墽琛岃嚜鍔ㄦ劅鐭ャ€?


## 鐩稿浜?Linux 鍐呮牳 2.4 绯诲垪鐨勫彉鏇?

io_request_lock 宸茶鑻ュ共涓洿缁嗙矑搴︾殑閿佸彇浠ｃ€備笌 LLD **鐩稿叧鐨勬槸 struct Scsi_Host**: host_lock锛屾瘡涓?SCSI 涓绘満鍚勬湁涓€涓€?

鏃х殑閿欒澶勭悊鏈哄埗宸茶绉婚櫎銆傝繖鎰忓懗鐫€ LLD 鎺ュ彛鍑芥暟 abort() 鍜?reset() 宸茶绉婚櫎銆?*struct scsi_host_template**: use_new_eh_code 鏍囧織宸茶绉婚櫎銆?

鍦?2.4 绯诲垪涓紝SCSI 瀛愮郴缁熺殑閰嶇疆璇存槑涓庢墍鏈夊叾浠?Linux 瀛愮郴缁熺殑閰嶇疆璇存槑鑱氬悎鍦?Documentation/Configure.help 鏂囦欢涓€傚湪 2.6 绯诲垪涓紝SCSI 瀛愮郴缁熺幇鍦ㄦ嫢鏈夎嚜宸辩殑锛堝皬寰楀鐨勶級drivers/scsi/Kconfig 鏂囦欢锛屽叾涓悓鏃跺寘鍚厤缃拰甯姪淇℃伅銆?

struct SHT 宸查噸鍛藉悕涓?struct scsi_host_template銆?

澧炲姞浜?鐑彃鎷斿垵濮嬪寲妯″瀷"浠ュ強璁稿鐢ㄤ簬鏀寔瀹冪殑棰濆鍑芥暟銆?


## 鑷磋阿

浠ヤ笅浜哄＋瀵规湰鏂囨。鍋氬嚭浜嗚础鐚細

 - Mike Anderson <andmike at us dot ibm dot com>
 - James Bottomley <James dot Bottomley at hansenpartnership dot com>
 - Patrick Mansfield <patmans at us dot ibm dot com>
 - Christoph Hellwig <hch at infradead dot org>
 - Doug Ledford <dledford at redhat dot com>
 - Andries Brouwer <Andries dot Brouwer at cwi dot nl>
 - Randy Dunlap <rdunlap at xenotime dot net>
 - Alan Stern <stern at rowland dot harvard dot edu>


Douglas Gilbert
dgilbert at interlog dot com

21st September 2004
