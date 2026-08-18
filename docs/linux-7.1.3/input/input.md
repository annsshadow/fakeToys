
## 绠€浠?

:Copyright: |copy| 1999-2001 Vojtech Pavlik <vojtech@ucw.cz> - Sponsored by SuSE

## 鏋舵瀯


Input 瀛愮郴缁熸槸璁捐涓烘敮鎸?Linux 涓嬫墍鏈夎緭鍏ヨ澶囩殑涓€缁勯┍鍔ㄧ殑闆嗗悎銆傚ぇ澶氭暟椹卞姩浣嶄簬 drivers/input锛屼笉杩囦篃鏈変笉灏戜綅浜?drivers/hid 鍜?drivers/platform銆?
杈撳叆瀛愮郴缁熺殑鏍稿績鏄綅浜庢渶搴曞眰鐨?input 妯″潡锛屽畠蹇呴』鍦ㄤ换浣曞叾浠栬緭鍏ユā鍧椾箣鍓嶅姞杞解€斺€斿畠浣滀负涓ょ粍妯″潡涔嬮棿鐨勯€氫俊鏂瑰紡锛?
### 璁惧椹卞姩


杩欎簺妯″潡涓庣‖浠跺璇濓紙渚嬪缁忕敱 USB锛夛紝骞跺悜 input 妯″潡鎻愪緵浜嬩欢锛堟寜閿€侀紶鏍囩Щ鍔級銆?
### 浜嬩欢澶勭悊绋嬪簭


杩欎簺妯″潡浠庤緭鍏ユ牳蹇冭幏鍙栦簨浠讹紝骞堕€氳繃鍚勭鎺ュ彛灏嗗叾浼犻€掑埌鎵€闇€涔嬪鈥斺€旀寜閿€佸線鍐呮牳锛岄紶鏍囩Щ鍔ㄧ粡鐢辨ā鎷熺殑 PS/2 鎺ュ彛閫佸線 GPM 鍜?X锛岀瓑绛夈€?
## 绠€鍗曠敤娉?

瀵逛簬鏈€甯歌鐨勯厤缃紝鍗充竴涓?USB 榧犳爣鍜屼竴涓?USB 閿洏锛屼綘闇€瑕佸姞杞戒互涓嬫ā鍧楋紙鎴栧皢鍏剁紪璇戣繘鍐呮牳
```

	input
	mousedev
	usbcore
	uhci_hcd or ohci_hcd or ehci_hcd
	usbhid
	hid_generic

```
姝ゅ悗锛孶SB 閿洏灏嗙珛鍗冲伐浣滐紝USB 榧犳爣
```

	crw-r--r--   1 root     root      13,  63 Mar 28 22:45 mice

```
璇ヨ澶囬€氬父鐢辩郴缁熻嚜鍔ㄥ垱寤恒€傚懡浠?```

	cd /dev
	mkdir input
	mknod input/mice c 13 63

```
涔嬪悗锛屼綘蹇呴』鎶?GPM锛堟枃鏈ā寮忛紶鏍囧壀鍒囩矘璐村伐鍏凤級鎸囧悜
```

	gpm -t ps2 -m /dev/input/mice

```
```

	Section "Pointer"
	    Protocol    "ImPS/2"
	    Device      "/dev/input/mice"
	    ZAxisMapping 4 5
	EndSection

```
褰撲綘瀹屾垚涓婅堪鎵€鏈夋搷浣滃悗锛屽氨鍙互浣跨敤浣犵殑 USB 榧犳爣鍜岄敭鐩樹簡銆?
## 璇︾粏鎻忚堪


### 浜嬩欢澶勭悊绋嬪簭


浜嬩欢澶勭悊绋嬪簭鏍规嵁闇€瑕佹妸鏉ヨ嚜璁惧鐨勪簨浠跺垎鍙戠粰鐢ㄦ埛绌洪棿鍜屽唴鏍稿唴娑堣垂鑰呫€?
#### evdev


`evdev` 鏄€氱敤鐨勮緭鍏ヤ簨浠舵帴鍙ｃ€傚畠鎶婂唴鏍镐腑浜х敓鐨勪簨浠惰繛鍚屾椂闂存埑鐩存帴浼犻€掔粰绋嬪簭銆備簨浠剁爜鍦ㄦ墍鏈夋灦鏋勪笂閮界浉鍚岋紝涓斾笌纭欢鏃犲叧銆?
杩欐槸鐢ㄦ埛绌洪棿娑堣垂鐢ㄦ埛杈撳叆鐨勯閫夋帴鍙ｏ紝榧撳姳鎵€鏈夊鎴风浣跨敤瀹冦€?
鏈夊叧 API 鐨勮鏄庯紝璇峰弬瑙?event-interface銆?
```

	crw-r--r--   1 root     root      13,  64 Apr  1 10:49 event0
	crw-r--r--   1 root     root      13,  65 Apr  1 10:50 event1
	crw-r--r--   1 root     root      13,  66 Apr  1 10:50 event2
	crw-r--r--   1 root     root      13,  67 Apr  1 10:50 event3
	...

```
鏈変袱缁勬璁惧鍙疯寖鍥达細64 鍒?95 鏄潤鎬侀仐鐣欒寖鍥淬€傚鏋滅郴缁熶腑杈撳叆璁惧瓒呰繃 32 涓紝鍒欎細浠ヤ粠 256 寮€濮嬬殑娆¤澶囧彿鍒涘缓棰濆鐨?evdev 鑺傜偣銆?
#### keyboard


`keyboard` 鏄唴鏍稿唴鐨勮緭鍏ュ鐞嗙▼搴忥紝鏄?VT 浠ｇ爜鐨勪竴閮ㄥ垎銆傚畠娑堣垂閿洏鎸夐敭骞跺鐞?VT 鎺у埗鍙扮殑鐢ㄦ埛杈撳叆銆?
#### mousedev


`mousedev` 鏄竴涓浣跨敤榧犳爣杈撳叆鐨勯仐鐣欑▼搴忓緱浠ュ伐浣滅殑鍏煎灞傘€傚畠浠庨紶鏍囨垨鏁板瓧鍖栦华/鎵嬪啓鏉胯幏鍙栦簨浠讹紝骞跺悜鐢ㄦ埛绌洪棿鎻愪緵涓€涓?PS/2 椋庢牸鐨勶紙绫讳技 /dev/psaux锛夐紶鏍囪澶囥€?
```

	crw-r--r--   1 root     root      13,  32 Mar 28 22:45 mouse0
	crw-r--r--   1 root     root      13,  33 Mar 29 00:41 mouse1
	crw-r--r--   1 root     root      13,  34 Mar 29 00:41 mouse2
	crw-r--r--   1 root     root      13,  35 Apr  1 10:50 mouse3
	...
	...
	crw-r--r--   1 root     root      13,  62 Apr  1 10:50 mouse30
	crw-r--r--   1 root     root      13,  63 Apr  1 10:50 mice

```
姣忎釜 `mouse` 璁惧閮藉垎閰嶇粰鍗曚釜榧犳爣鎴栨暟瀛楀寲浠紝鏈€鍚庝竴涓?`mice` 闄ゅ銆傝繖涓崟涓€瀛楃璁惧琚墍鏈夐紶鏍囧拰鏁板瓧鍖栦华鍏变韩锛屽嵆浣挎病鏈夎繛鎺ヤ换浣曡澶囷紝璇ヨ澶囦篃瀛樺湪銆傝繖瀵逛簬 USB 榧犳爣鐨勭儹鎻掓嫈寰堟湁鐢紝浣垮緱涓嶅鐞嗙儹鎻掓嫈鐨勬棫绋嬪簭鍗充娇鍦ㄦ病鏈夐紶鏍囨椂涔熻兘鎵撳紑璇ヨ澶囥€?
鍐呮牳閰嶇疆涓殑 CONFIG_INPUT_MOUSEDEV_SCREEN_[XY] 鏄綘鍦?XFree86 涓睆骞曠殑澶у皬锛堝儚绱狅級銆傚鏋滀綘鎯冲湪 X 涓娇鐢ㄦ暟瀛楀寲浠紝灏遍渶瑕佸畠锛屽洜涓哄叾绉诲姩鏄€氳繃涓€涓櫄鎷?PS/2 榧犳爣鍙戦€佺粰 X 鐨勶紝鍥犳闇€瑕佺浉搴旂缉鏀俱€傚鏋滃彧浣跨敤榧犳爣锛屽垯涓嶄細鐢ㄥ埌杩欎簺鍊笺€?
Mousedev 浼氭牴鎹鍙栨暟鎹殑绋嬪簭鐨勯渶姹傦紝鐢熸垚 PS/2銆両mPS/2锛圡icrosoft IntelliMouse锛夋垨 ExplorerPS/2锛圛ntelliMouse Explorer锛夊崗璁€備綘鍙互鎶?GPM 鍜?X 璁句负鍏朵腑浠绘剰涓€绉嶃€傚鏋滄兂浣跨敤 USB 榧犳爣鐨勬粴杞紝闇€瑕?ImPS/2锛涘鏋滄兂浣跨敤棰濆鐨勶紙鏈€澶?5 涓級鎸夐挳锛岄渶瑕?ExplorerPS/2銆?
#### joydev


`joydev` 瀹炵幇浜?v0.x 鍜?v1.x 鐨?Linux 娓告垙鏉?API銆傝鎯呰鍙傝 joystick-api銆?
```

	crw-r--r--   1 root     root      13,   0 Apr  1 10:50 js0
	crw-r--r--   1 root     root      13,   1 Apr  1 10:50 js1
	crw-r--r--   1 root     root      13,   2 Apr  1 10:50 js2
	crw-r--r--   1 root     root      13,   3 Apr  1 10:50 js3
	...

```
浠ユ绫绘帹鐩村埌閬楃暀鑼冨洿鍐呯殑 js31锛屽鏋滃瓨鍦ㄦ洿澶氭父鎴忔潌璁惧锛岃繕浼氭湁娆¤澶囧彿澶т簬 256 鐨勯澶栬妭鐐广€?
### 璁惧椹卞姩


璁惧椹卞姩鏄骇鐢熶簨浠剁殑妯″潡銆?
#### hid-generic


`hid-generic` 鏄暣涓浠朵腑鏈€澶с€佹渶澶嶆潅鐨勯┍鍔ㄤ箣涓€銆傚畠澶勭悊鎵€鏈?HID 璁惧锛岀敱浜庤澶囩绫绘瀬鍏剁箒澶氾紝涓?USB HID 瑙勮寖骞朵笉绠€鍗曪紝瀹冮渶瑕佸姝ゅ簽澶с€?
鐩墠锛屽畠澶勭悊 USB 榧犳爣銆佹父鎴忔潌銆佹父鎴忔墜鏌勩€佹柟鍚戠洏銆侀敭鐩樸€佽建杩圭悆鍜屾暟瀛楀寲浠€?
鐒惰€岋紝USB 涔熸妸 HID 鐢ㄤ簬鏄剧ず鍣ㄦ帶鍒躲€佹壃澹板櫒鎺у埗銆乁PS銆丩CD 浠ュ強璁稿鍏朵粬鐢ㄩ€斻€?
鏄剧ず鍣ㄥ拰鎵０鍣ㄦ帶鍒跺簲褰撳緢瀹规槗鍔犲叆 hid/input 鎺ュ彛锛屼絾瀵逛簬 UPS 鍜?LCD 鏉ヨ鎰忎箟涓嶅ぇ銆備负姝わ紝璁捐浜?hiddev 鎺ュ彛銆傛洿澶氫俊鎭鍙傝 Documentation/hid/hiddev.rst銆?
usbhid 妯″潡鐨勪娇鐢ㄩ潪甯哥畝鍗曪紝瀹冧笉甯︿换浣曞弬鏁帮紝鑷姩妫€娴嬩竴鍒囷紝褰撴彃鍏?HID 璁惧鏃讹紝浼氭伆褰撳湴妫€娴嬪埌瀹冦€?
涓嶈繃锛岀敱浜庤澶囧樊寮傛瀬澶э紝浣犲彲鑳戒細纰板埌涓€涓伐浣滀笉澶ソ鐨勮澶囥€傚湪杩欑鎯呭喌涓嬶紝璇峰湪 hid-core.c 寮€澶?#define DEBUG 骞舵妸 syslog 璺熻釜淇℃伅鍙戠粰鎴戙€?
#### usbmouse


瀵逛簬宓屽叆寮忕郴缁熴€佸甫鏈夋崯鍧?HID 鎻忚堪绗︾殑榧犳爣锛屼互鍙婁换浣曚笉閫傚悎浣跨敤搴炲ぇ鐨?usbhid 鐨勫満鍚堬紝鏈?usbmouse 椹卞姩銆傚畠鍙鐞?USB 榧犳爣锛屼娇鐢ㄦ洿绠€鍗曠殑 HIDBP 鍗忚銆傝繖涔熸剰鍛崇潃榧犳爣蹇呴』鏀寔杩欎釜鏇寸畝鍗曠殑鍗忚锛屼絾骞堕潪鎵€鏈夐紶鏍囬兘鏀寔銆傚鏋滄病鏈変粈涔堝己鐑堢悊鐢变娇鐢ㄨ繖涓ā鍧楋紝璇锋敼鐢?usbhid銆?
#### usbkbd


涓?usbmouse 绫讳技锛岃繖涓ā鍧楃敤绠€鍖栫殑 HIDBP 鍗忚涓庨敭鐩橀€氫俊銆傚畠鏇村皬锛屼絾涓嶆敮鎸佷换浣曢澶栫殑鐗规畩鎸夐敭銆傚鏋滄病鏈夌壒娈婄悊鐢变娇鐢ㄥ畠锛岃鏀圭敤 usbhid銆?
#### psmouse


杩欐槸閫傜敤浜庢墍鏈変娇鐢?PS/2 鍗忚鐨勬寚鐐硅澶囩殑椹卞姩锛屽寘鎷?Synaptics 鍜?ALPS 瑙︽帶鏉裤€両ntellimouse Explorer 璁惧銆丩ogitech PS/2 榧犳爣绛夌瓑銆?
#### atkbd


杩欐槸鐢ㄤ簬 PS/2锛圓T锛夐敭鐩樼殑椹卞姩銆?
#### iforce


鐢ㄤ簬 I-Force 娓告垙鏉嗗拰鏂瑰悜鐩樼殑椹卞姩锛屽彲閫氳繃 USB 鍜?RS232銆傚畠鐜板湪鍖呭惈鍔涘弽棣堬紙Force Feedback锛夋敮鎸侊紝灏界 Immersion Corp. 灏嗗崗璁涓哄晢涓氭満瀵嗭紝涓€涓瓧涔熶笉鎰块€忛湶銆?
## 楠岃瘉鏄惁宸ヤ綔


鍦ㄩ敭鐩樹笂鏁插嚑涓敭锛屽氨瓒充互妫€鏌ラ敭鐩樻槸鍚﹀伐浣滃苟宸叉纭繛鎺ュ埌鍐呮牳閿洏椹卞姩銆?
鎵ц `cat /dev/input/mouse0`锛坈, 13, 32锛夊彲浠ラ獙璇侀紶鏍囦篃琚ā鎷熷嚭鏉ヤ簡锛涚Щ鍔ㄩ紶鏍囨椂搴斿綋鍑虹幇瀛楃銆?
浣犲彲浠ョ敤 `jstest` 宸ュ叿娴嬭瘯娓告垙鏉嗘ā鎷燂紝璇ュ伐鍏峰湪 joystick 杞欢鍖呬腑鍙敤锛堝弬瑙?joystick-doc锛夈€?
浣犲彲浠ョ敤 `evtest` 宸ュ叿娴嬭瘯浜嬩欢璁惧銆?
## 浜嬩欢鎺ュ彛


浣犲彲浠ヤ娇鐢ㄩ樆濉炲拰闈為樆濉炶鍙栵紝涔熷彲浠ュ湪 /dev/input/eventX 璁惧涓婁娇鐢?select()锛屼綘鎬绘槸浼氬緱鍒颁竴涓暣鏁颁釜鐨勮緭鍏?```

    struct input_event {
	    struct timeval time;
	    unsigned short type;
	    unsigned short code;
	    int value;
    };

```
`time` 鏄椂闂存埑锛屽畠杩斿洖浜嬩欢鍙戠敓鐨勬椂闂淬€俙type` 渚嬪鐩稿绉诲姩涓?EV_REL銆佹寜閿寜涓嬫垨鏉惧紑涓?EV_KEY銆傛洿澶氱被鍨嬪畾涔変簬 include/uapi/linux/input-event-codes.h銆?
`code` 鏄簨浠剁爜锛屼緥濡?REL_X 鎴?KEY_BACKSPACE锛屽畬鏁村垪琛ㄥ悓鏍峰湪 include/uapi/linux/input-event-codes.h 涓€?
`value` 鏄簨浠舵惡甯︾殑鍊笺€傚浜?EV_REL 鏄浉瀵瑰彉鍖栭噺锛屽浜?EV_ABS锛堟父鎴忔潌鈥︹€︼級鏄粷瀵规柊鍊硷紝瀵逛簬 EV_KEY 鍒欐槸锛氭澗寮€涓?0锛屾寜涓嬩负 1锛岃嚜鍔ㄩ噸澶嶄负 2銆?
鏈夊叧鍚勭浜嬩欢鐮佺殑鏇村淇℃伅锛岃鍙傝 input-event-codes銆?