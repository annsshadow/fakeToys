## Linux 鍜?骞惰 绔彛 IDE 璁惧


PARIDE v1.03   (c) 1997-8  Grant Guenther <grant@torque.net>
PATA_PARPORT   (c) 2023 Ondrej Zary

## 1. Introduction


Owing 鍒?the simplicity 鍜?near universality 鐨?the 骞惰 绔彛 鎺ュ彛
鍒?personal computers, 璁稿 澶栭儴 璁惧 渚嬪 portable hard-disk,
CD-ROM, LS-120 鍜?tape drives 浣跨敤 the 骞惰 绔彛 鍒?connect 鍒?瀹冧滑鐨?
host computer.  鍚屾椂 涓€浜?璁惧 (notably scanners) 浣跨敤 ad-hoc 鏂规硶
鍒?pass 鍛戒护 鍜?鏁版嵁 through the 骞惰 绔彛 鎺ュ彛, 澶у鏁?
澶栭儴 璁惧 鏄?actually identical 鍒?涓€涓?鍐呴儴 鍨嬪彿, 浣?涓?
涓€涓?parallel-port adapter 鑺墖 added 鍦?  涓€浜?鐨?the original 骞惰 绔彛
adapters 鏇炬槸 little 澶氫簬 mechanisms 鐢ㄤ簬 multiplexing 涓€涓?SCSI 鎬荤嚎.
(The Iomega PPA-3 adapter 浣跨敤 鍦?the ZIP drives 鏄?涓€涓?绀轰緥 鐨?姝?
approach).  澶у鏁?鐢垫祦 designs, 鐒惰€? take 涓€涓?涓嶅悓 approach.
The adapter 鑺墖 reproduces 涓€涓?small ISA 鎴?IDE 鎬荤嚎 鍦?the 澶栭儴 璁惧
鍜?the communication 鍗忚 鎻愪緵 鎿嶄綔 鐢ㄤ簬 reading 鍜?writing
璁惧 瀵勫瓨鍣? 浠ュ強 鏁版嵁 鍧?transfer 鍑芥暟.  鏈夋椂,
the 璁惧 姝ｅ湪 addressed 閫氳繃 the 骞惰 cable 鏄?涓€涓?鏍囧噯 SCSI
鎺у埗鍣?绫讳技 涓€涓?NCR 5380.  The "ditto" family 鐨?澶栭儴 tape
drives 浣跨敤 the ISA replicator 鍒?鎺ュ彛 涓€涓?floppy disk 鎺у埗鍣?
鍏?鏄?鐒跺悗 connected 鍒?涓€涓?floppy-tape mechanism.  The vast majority
鐨?澶栭儴 骞惰 绔彛 璁惧, 鐒惰€? 鏄?鐜板湪 鍩轰簬 鏍囧噯
IDE 绫诲瀷 璁惧, 鍏?闇€瑕?鏃?intermediate 鎺у埗鍣?  鑻?one
鏇炬槸 鍒?鎵撳紑 up 涓€涓?骞惰 绔彛 CD-ROM drive, 渚嬪, one 灏嗕細
find 涓€涓?鏍囧噯 ATAPI CD-ROM drive, 涓€涓?鐢垫簮 supply, 鍜?涓€涓?鍗曚釜 adapter
璇?interconnected 涓€涓?鏍囧噯 PC 骞惰 绔彛 cable 鍜?涓€涓?鏍囧噯
IDE cable.  瀹冩槸 閫氬父 鍙兘 鍒?exchange the CD-ROM 璁惧 涓?
浠讳綍 鍏朵粬 璁惧 浣跨敤 the IDE 鎺ュ彛.

The document describes the 鏀寔 鍦?Linux 鐢ㄤ簬 骞惰 绔彛 IDE
璁惧.  瀹?鎵ц 涓?cover 骞惰 绔彛 SCSI 璁惧, "ditto" tape
drives 鎴?scanners.  璁稿 涓嶅悓 璁惧 鏄?鍙楁敮鎸?鐢?the
骞惰 绔彛 IDE 瀛愮郴缁? including:

 - MicroSolutions backpack CD-ROM
 - MicroSolutions backpack PD/CD
 - MicroSolutions backpack hard-drives
 - MicroSolutions backpack 8000t tape drive
 - SyQuest EZ-135, EZ-230 & SparQ drives
 - Avatar Shark
 - Imation Superdisk LS-120
 - Maxell Superdisk LS-120
 - FreeCom 鐢垫簮 CD
 - Hewlett-Packard 5GB 鍜?8GB tape drives
 - Hewlett-Packard 7100 鍜?7200 CD-RW drives

浠ュ強 澶у鏁?鐨?the clone 鍜?no-name products 鍦?the market.

鍒?鏀寔 姝ょ被 涓€涓?wide range 鐨?璁惧, pata_parport 鏄?actually structured
鍦?two parts. 瀛樺湪 涓€涓?base pata_parport 妯″潡 鍏?鎻愪緵 涓€涓?鎺ュ彛
鍒?鍐呮牳 libata 瀛愮郴缁? registry 鍜?涓€浜?閫氱敤 鏂规硶 鐢ㄤ簬 accessing
the 骞惰 ports.

The second component 鏄?涓€涓?set 鐨?low-level 鍗忚 椹卞姩 鐢ㄤ簬 姣忎釜 鐨?the
骞惰 绔彛 IDE adapter chips.  Thanks 鍒?the interest 鍜?encouragement 鐨?
Linux users 鏉ヨ嚜 璁稿 parts 鐨?the world, 鏀寔 鏄?鍙敤 鐢ㄤ簬 almost 鍏ㄩ儴
known adapter 鍗忚:

	====    ====================================== ====
        aten    ATEN EH-100                            (HK)
        bpck    Microsolutions backpack                (US)
        comm    DataStor (old-type) "commuter" adapter (TW)
        dstr    DataStor EP-2000                       (TW)
        epat    Shuttle EPAT                           (UK)
        epia    Shuttle EPIA                           (UK)
	fit2    FIT TD-2000			       (US)
	fit3    FIT TD-3000			       (US)
	friq    Freecom IQ cable                       (DE)
        frpw    Freecom 鐢垫簮                          (DE)
        kbic    KingByte KBIC-951涓€涓?鍜?KBIC-971涓€涓?      (TW)
	ktti    KT Technology PHd adapter              (SG)
        鍦?0    OnSpec 90c20                           (US)
        鍦?6    OnSpec 90c26                           (US)
	====    ====================================== ====


## 2. 浣跨敤 pata_parport 瀛愮郴缁?


鍚屾椂 configuring the Linux 鍐呮牳, 鎮?鍙?choose 浠讳竴涓?鍒?build
the pata_parport 椹卞姩 杩涘叆 鎮ㄧ殑 鍐呮牳, 鎴?鍒?build them 浣滀负 妯″潡.

鍦?浠讳竴涓?case, 鎮?灏?闇€瑕?鍒?select "骞惰 绔彛 IDE 璁惧 鏀寔"
鍜?鑷冲皯 one 鐨?the 骞惰 绔彛 communication 鍗忚.
鑻?鎮?鎵ц 涓?know 浠€涔?kind 鐨?骞惰 绔彛 adapter 鏄?浣跨敤 鍦?鎮ㄧ殑 drive,
鎮?鍙互 begin 鐢?checking the 鏂囦欢 names 鍜?浠讳綍 text 鏂囦欢 鍦?鎮ㄧ殑 DOS
installation floppy.  Alternatively, 鎮ㄥ彲浠?look 鍦?the markings 鍦?
the adapter 鑺墖 itself.  璇?s 閫氬父 sufficient 鍒?identify the
correct 璁惧.

鎮ㄥ彲浠?actually select 鍏ㄩ儴 the 鍗忚 妯″潡, 鍜?鍏佽 the pata_parport
瀛愮郴缁?鍒?try them 鍏ㄩ儴 鐢ㄤ簬 鎮?

鐢ㄤ簬 the "brand-name" products listed 涓婃枃, 姝ゅ 鏄?the 鍗忚
鍜?high-level 椹卞姩 璇?鎮?灏嗕細 浣跨敤:

	================	============	========
	Manufacturer		鍨嬪彿		鍗忚
	================	============	========
	MicroSolutions		CD-ROM		bpck
	MicroSolutions		PD drive	bpck
	MicroSolutions		hard-drive	bpck
	MicroSolutions          8000t tape      bpck
	SyQuest			EZ, SparQ	epat
	Imation			Superdisk	epat
	Maxell                  Superdisk       friq
	Avatar			Shark		epat
	FreeCom			CD-ROM		frpw
	Hewlett-Packard		5GB Tape	epat
	Hewlett-Packard		7200e (CD)	epat
	Hewlett-Packard		7200e (CD-R)	epat
	================	============	========

鍏ㄩ儴 parports 鍜?鍏ㄩ儴 鍗忚 椹卞姩 鏄?probed automatically 闄ら潪 probe=0
鍙傛暟 鏄?浣跨敤. 鍥犳 just "modprobe epat" 鏄?enough 鐢ㄤ簬 涓€涓?Imation SuperDisk
drive 鍒?work.

```

	# echo "port protocol mode unit delay" >/sys/bus/pata_parport/new_device

```
浣曞:

	======== ================================================
	绔彛	 parport name (鎴?"auto" 鐢ㄤ簬 鍏ㄩ儴 parports)
	鍗忚 鍗忚 name (鎴?"auto" 鐢ㄤ簬 鍏ㄩ儴 鍗忚)
	妯″紡	 妯″紡 鏁板瓧 (protocol-specific) 鎴?-1 鐢ㄤ簬 probe
	unit	 unit 鏁板瓧 (鐢ㄤ簬 backpack 浠? 鍙傝 涓嬫枃)
	delay	 I/O delay (鍙傝 troubleshooting section 涓嬫枃)
	======== ================================================

鑻?鎮?happen 鍒?涓?浣跨敤 涓€涓?MicroSolutions backpack 璁惧, 鎮?灏?
涔?闇€瑕?鍒?know the unit ID 鏁板瓧 鐢ㄤ簬 姣忎釜 drive.  杩欐槸 閫氬父
the 鏈€鍚?two digits 鐨?the drive's 涓茶 鏁板瓧 (浣?璇诲彇 MicroSolutions'
documentation 鍏充簬 姝?.

鑻?鎮?omit the 鍙傛暟 鏉ヨ嚜 the end, defaults 灏?涓?浣跨敤, e.g.:

```

	# echo auto >/sys/bus/pata_parport/new_device

```
```

	# echo "parport0 epat 4" >/sys/bus/pata_parport/new_device

```
```

	# echo "parport0 auto" >/sys/bus/pata_parport/new_device

```
```

	# echo "auto epat" >/sys/bus/pata_parport/new_device

```
```

	# echo pata_parport.0 >/sys/bus/pata_parport/delete_device


```
## 3. Troubleshooting


### 3.1  浣跨敤 EPP 妯″紡 鑻?鎮ㄥ彲浠?


The 澶у鏁?閫氱敤 problems 璇?people report 涓?the pata_parport 椹卞姩
concern the 骞惰 绔彛 CMOS 璁剧疆.  鍦?姝?time, none 鐨?the
鍗忚 妯″潡 鏀寔 ECP 妯″紡, 鎴?浠讳綍 ECP combination modes.
鑻?鎮?鏄?able 鍒?鎵ц 鍥犳, 璇?set 鎮ㄧ殑 骞惰 绔彛 杩涘叆 EPP 妯″紡
浣跨敤 鎮ㄧ殑 CMOS setup procedure.

### 3.2  Check the 绔彛 delay


涓€浜?骞惰 ports cannot reliably transfer 鏁版嵁 鍦?full speed.  鍒?
鍋忕Щ the 閿欒, the 鍗忚 妯″潡 introduce 涓€涓?"绔彛
delay" 涔嬮棿 姣忎釜 access 鍒?the i/o ports.  姣忎釜 鍗忚 sets
涓€涓?榛樿 鍊?鐢ㄤ簬 姝?delay.  鍦?澶у鏁?cases, the 鐢ㄦ埛 鍙?override
the 榛樿 鍜?set 瀹?鍒?0 - resulting 鍦?somewhat higher transfer
rates.  鍦?涓€浜?rare cases (especially 涓?older 486 绯荤粺) the
榛樿 delays 鏄?涓?long enough.  鑻?鎮?experience corrupt 鏁版嵁
transfers, 鎴?unexpected failures, 鎮?鍙?wish 鍒?increase the
绔彛 delay.

### 3.3  涓€浜?drives 闇€瑕?涓€涓?鎵撳嵃鏈?reset


閭ｉ噷 appear 鍒?涓?涓€涓?鏁板瓧 鐨?"noname" 澶栭儴 drives 鍦?the market
璇?鎵ц 涓?濮嬬粓 鐢垫簮 up correctly.  鎴戜滑 鍏锋湁 noticed 姝?涓?涓€浜?
drives 鍩轰簬 OnSpec 鍜?older Freecom adapters.  鍦?杩欎簺 rare cases,
the adapter 鍙?閫氬父 涓?reinitialised 鐢?issuing 涓€涓?"鎵撳嵃鏈?reset" 鍦?
the 骞惰 绔彛.  浣滀负 the reset 鎿嶄綔 鏄?potentially disruptive 鍦?
澶氫釜 璁惧 environments, the pata_parport 椹卞姩 灏?涓?鎵ц 瀹?
```

	insmod lp reset=1
	rmmod lp

```
鑻?鎮?鍏锋湁 one 鐨?杩欎簺 marginal cases, 鎮?搴斿綋 probably build
鎮ㄧ殑 pata_parport 椹卞姩 浣滀负 妯″潡, 鍜?arrange 鍒?鎵ц the 鎵撳嵃鏈?reset
涔嬪墠 loading the pata_parport 椹卞姩.
