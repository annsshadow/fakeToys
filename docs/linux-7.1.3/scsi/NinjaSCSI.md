
## WorkBiT NinjaSCSI-3/32Bi Linux 椹卞姩


## 1. 璇存槑


杩欐槸 Workbit corp.锛坔ttp://www.workbit.co.jp/锛夌殑 NinjaSCSI-3 鐨?Linux 椹卞姩銆?
## 2. 鎴戠殑 Linux 鐜


:Linux 鍐呮牳: 2.4.7 / 2.2.19
:pcmcia-cs:    3.1.27
:gcc:          gcc-2.95.4
:PC 鍗?        I-O data PCSC-F (NinjaSCSI-3),
               I-O data CBSC-II 16 浣嶆ā寮?(NinjaSCSI-32Bi)
:SCSI 璁惧:    I-O data CDPS-PX24 (CD-ROM 椹卞姩鍣?,
               Media Intelligent MMO-640GT (鍏夌洏椹卞姩鍣?

## 3. 瀹夎


(a) 纭浣犵殑 PC 鍗℃槸鐪熸鐨?鈥淣injaSCSI-3鈥?鍗°€?
    濡傛灉浣犲凡缁忓畨瑁呬簡 pcmcia-cs锛宲cmcia 浼氬皢浣犵殑鍗℃姤鍛婁负 UNKNOWN 鍗★紝骞跺悜浣犵殑鎺у埗鍙版垨
    鏃ュ織鏂囦欢鍐欏叆 ["WBT", "NinjaSCSI-3", "R1.0"] 鎴栧叾浠栧瓧绗︿覆銆?
    浣犱篃鍙互浣跨敤 鈥渃ardctl鈥?绋嬪簭锛堣绋嬪簭浣嶄簬 pcmcia-cs 婧愮爜涓級鏉ヨ幏鍙栨洿澶氫俊鎭€?
```

	# cat /var/log/messages
	...
	Jan  2 03:45:06 lindberg cardmgr[78]: unsupported card in socket 1
	Jan  2 03:45:06 lindberg cardmgr[78]:   product info: "WBT", "NinjaSCSI-3", "R1.0"
	...
	# cardctl ident
	Socket 0:
	  no product info available
	Socket 1:
	  product info: "IO DATA", "CBSC16       ", "1"


```
(b) 鑾峰彇 Linux 鍐呮牳婧愮爜锛屽苟灏嗗叾瑙ｅ帇鍒?/usr/src銆傜敱浜?NinjaSCSI 椹卞姩闇€瑕?Linux 鍐呮牳
    婧愮爜涓殑涓€浜?SCSI 澶存枃浠讹紝鎴戝缓璁噸鏂版瀯寤轰綘鐨勫唴鏍革紱杩欏彲浠ユ秷闄や竴浜涚増鏈棶棰樸€?
```

	$ cd /usr/src
	$ tar -zxvf linux-x.x.x.tar.gz
	$ cd linux
	$ make config
	...

```
(c) 濡傛灉浣犲皢璇ラ┍鍔ㄤ笌鍐呮牳 2.2 閰嶅悎浣跨敤锛屽湪鏌愪釜鐩綍涓В鍘?pcmcia-cs 骞?make & install銆?    璇ラ┍鍔ㄩ渶瑕?pcmcia-cs 澶存枃浠躲€?
```

	$ cd /usr/src
	$ tar zxvf cs-pcmcia-cs-3.x.x.tar.gz
	...

```
```

	$ tar -zxvf nsp_cs-x.x.tar.gz
	$ cd nsp_cs-x.x
	$ emacs Makefile
	...
	$ make

```
(e) 灏?nsp_cs.ko 澶嶅埗鍒板悎閫傜殑浣嶇疆锛屼緥濡?/lib/modules/<鍐呮牳鐗堟湰>/pcmcia/ 銆?
(f) 灏嗚繖浜涜鍔犲叆 /etc/pcmcia/config 銆?
    濡傛灉浣犱娇鐢?pcmcia-cs-3.1.8 鎴栨洿楂樼増鏈紝鎴戜滑鍙互浣跨敤 鈥渘sp_cs.conf鈥?鏂囦欢銆?    鍥犳锛屼綘鏃犻渶缂栬緫鏂囦欢锛屽彧闇€澶嶅埗鍒?/etc/pcmcia/ 鍗冲彲銆?
```

	device "nsp_cs"
	  class "scsi" module "nsp_cs"

	card "WorkBit NinjaSCSI-3"
	  version "WBT", "NinjaSCSI-3", "R1.0"
	  bind "nsp_cs"

	card "WorkBit NinjaSCSI-32Bi (16bit)"
	  version "WORKBIT", "UltraNinja-16", "1"
	  bind "nsp_cs"

	# OEM
	card "WorkBit NinjaSCSI-32Bi (16bit) / IO-DATA"
	  version "IO DATA", "CBSC16       ", "1"
	  bind "nsp_cs"

	# OEM
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-1"
	  version "KME    ", "SCSI-CARD-001", "1"
	  bind "nsp_cs"
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-2"
	  version "KME    ", "SCSI-CARD-002", "1"
	  bind "nsp_cs"
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-3"
	  version "KME    ", "SCSI-CARD-003", "1"
	  bind "nsp_cs"
	card "WorkBit NinjaSCSI-32Bi (16bit) / KME-4"
	  version "KME    ", "SCSI-CARD-004", "1"
	  bind "nsp_cs"

```
```

	# /etc/rc.d/rc.pcmcia start        (BSD 椋庢牸)

    鎴?:

	# /etc/init.d/pcmcia start         (SYSV 椋庢牸)


```
## 4. 鍘嗗彶


鍙傝 README.nin_cs 銆?
## 5. 娉ㄦ剰浜嬮」


濡傛灉鍦ㄥ SCSI 璁惧鎵ц鏌愪簺鎿嶄綔锛屾垨鎸傝捣璁＄畻鏈烘椂寮瑰嚭鍗＄墖锛屼綘浼氶亣鍒颁竴浜?*涓ラ噸**閿欒锛?渚嬪纾佺洏宕╂簝銆?
褰撴垜姝ｇ‘浣跨敤璇ラ┍鍔ㄦ椂瀹冨伐浣滆壇濂姐€備絾鎴戜笉淇濊瘉浣犵殑鏁版嵁銆備娇鐢ㄨ椹卞姩鏃惰澶囦唤浣犵殑鏁版嵁銆?
## 6. 宸茬煡缂洪櫡


鍦?2.4 鍐呮牳涓紝浣犳棤娉曚娇鐢?640MB 鍏夌洏銆傝閿欒鏉ヨ嚜楂樺眰 SCSI 椹卞姩銆?
## 7. 娴嬭瘯


璇峰悜鎴戜滑鍙戦€佽杞欢鐨勪竴浜涙姤鍛婏紙缂洪櫡鎶ュ憡绛夛級銆傚彂閫佹姤鍛婃椂锛岃鍛婄煡鎴戜滑浠ヤ笅鎴栨洿澶氫俊鎭€?
 - 鍗″悕绉? - 鍐呮牳鐗堟湰
 - 浣犵殑 SCSI 璁惧鍚嶇О锛堢‖鐩樸€丆D-ROM 绛夆€︹€︼級

## 8. 鐗堟潈


 鍙傝 GPL銆?

2001/08/08 yokota@netlab.is.tsukuba.ac.jp <YOKOTA Hiroshi>
