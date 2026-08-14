

## 绠€浠?

Linux 鐨勬憞鏉嗛┍鍔ㄤ负澶氱鎽囨潌鍙婄被浼艰澶囨彁渚涙敮鎸併€傚畠鍩轰簬涓€涓洿澶х殑椤圭洰锛岃椤圭洰鏃ㄥ湪鏀寔 Linux 涓殑鎵€鏈夎緭鍏ヨ澶囥€?
璇ラ」鐩殑閭欢鍒楄〃涓猴細

	linux-input@vger.kernel.org

鍚?majordomo@vger.kernel.org 鍙戦€?"subscribe linux-input" 鍗冲彲璁㈤槄銆?
## 浣跨敤


瀵逛簬鍩烘湰浣跨敤锛屼綘鍙渶鍦ㄥ唴鏍搁厤缃腑閫夋嫨姝ｇ‘鐨勯€夐」鍗冲彲銆?
### 宸ュ叿


鍑轰簬娴嬭瘯鍙婂叾浠栫洰鐨勶紙渚嬪涓茶璁惧锛夛紝鏈変竴缁勫伐鍏凤紝濡?`jstest`銆乣jscal` 鍜?`evtest`锛岄€氬父琚墦鍖呬负 `joystick`銆乣input-utils`銆乣evtest` 绛夈€?
濡傛灉浣犵殑鎽囨潌杩炴帴鍒颁覆鍙ｏ紝鍒欓渶瑕?`inputattach` 宸ュ叿銆?
### 璁惧鑺傜偣


涓轰簡璁╁簲鐢ㄧ▼搴忚兘澶熶娇鐢ㄦ憞鏉嗭紝搴斿湪 /dev 涓垱寤鸿澶囪妭鐐广€傞€氬父鐢辩郴缁熻嚜鍔ㄥ畬鎴愶紝浣?```

    cd /dev
    rm js*
    mkdir input
    mknod input/js0 c 13 0
    mknod input/js1 c 13 1
    mknod input/js2 c 13 2
    mknod input/js3 c 13 3
    ln -s input/js0 js0
    ln -s input/js1 js1
    ln -s input/js2 js2
    ln -s input/js3 js3

```
```

    mknod input/event0 c 13 64
    mknod input/event1 c 13 65
    mknod input/event2 c 13 66
    mknod input/event3 c 13 67

```
### 鎵€闇€妯″潡


涓轰娇鎵€鏈夋憞鏉嗛┍鍔ㄦ甯稿伐浣滐紝浣犻渶瑕佺敤鎴锋€佹帴鍙?```

	modprobe joydev

```
```

	modprobe ns558

```
鑰屽浜庝覆鍙ｆ憞鏉嗭紝浣犻渶瑕佷覆琛岃緭鍏ョ嚎璺?```

	modprobe serport
	inputattach -xxx /dev/tts/X &

```
闄ゆ涔嬪锛屼綘杩橀渶瑕佹憞鏉嗛┍鍔ㄦā鍧楁湰韬紝閫氬父鏄?```

	modprobe analog

```
涓轰簡瀹炵幇妯″潡鑷姩鍔犺浇锛岀被浼间笅闈㈢殑閰嶇疆鍙兘鏈夋晥鈥斺€旇鏍规嵁瀹為檯鎯呭喌璋冩暣
```

	alias tty-ldisc-2 serport
	alias char-major-13 input
	above input joydev ns558 analog
	options analog map=gamepad,none,2btn

```
### 楠岃瘉鏄惁宸ヤ綔


涓轰簡娴嬭瘯鎽囨潌椹卞姩鍔熻兘锛屽彲浠ヤ娇鐢?jstest
```

	jstest /dev/input/js0

```
瀹冨簲鏄剧ず涓€琛屾憞鏉嗘暟鍊硷紝褰撲綘绉诲姩鎽囨潌骞舵寜涓嬫寜閽椂杩欎簺鏁板€间細鏇存柊銆傚綋鎽囨潌澶勪簬涓績浣嶇疆鏃讹紝鎵€鏈夎酱閮藉簲涓洪浂銆傚畠浠笉搴旇嚜琛岃烦鍙樺埌鍏朵粬鎺ヨ繎鐨勫€硷紝骞朵笖鍦ㄦ憞鏉嗙殑浠讳綍鍏朵粬浣嶇疆閮藉簲淇濇寔绋冲畾銆傚畠浠簲鍏锋湁浠?-32767 鍒?32767 鐨勫畬鏁磋寖鍥淬€傚鏋滄弧瓒虫墍鏈夎繖浜涙潯浠讹紝閭ｅ氨涓€鍒囨甯革紝浣犲彲浠ョ帺娓告垙浜嗐€?)

濡傛灉涓嶆槸锛屽垯鍙兘瀛樺湪闂銆傚皾璇曟牎鍑嗘憞鏉嗭紝濡傛灉浠嶇劧涓嶅伐浣滐紝璇烽槄璇绘湰鏂囦欢鐨勯┍鍔ㄤ竴鑺傘€佹帓闅滀竴鑺備互鍙?FAQ銆?
### 鏍″噯


瀵逛簬澶у鏁版憞鏉嗭紝浣犱笉闇€瑕佷换浣曟墜鍔ㄦ牎鍑嗭紝鍥犱负鎽囨潌搴旂敱椹卞姩鑷姩锛坅utomagically锛夎嚜鍔ㄦ牎鍑嗐€傜劧鑰岋紝瀵逛簬鏌愪簺妯℃嫙鎽囨潌锛屽畠浠涔堜笉浣跨敤绾挎€х數闃伙紝瑕佷箞褰撲綘
```

	jscal -c /dev/input/js0

```
鍖呭惈鍦?joystick 鍖呬腑锛岀敤浜庤缃瘮椹卞姩鑷韩閫夋嫨鏇村ソ鐨勬牎姝ｇ郴鏁般€?
鏍″噯鎽囨潌鍚庯紝浣犲彲浠ョ敤 jstest 鍛戒护楠岃瘉鏄惁鍠滄鏂扮殑鏍″噯锛屽鏋滃枩娆紝浣犲彲浠ヤ繚瀛?```

	jscal -p /dev/input/js0 > /etc/joystick.cal

```
```

	source /etc/joystick.cal

```
杩欐牱锛屽湪涓嬩竴娆￠噸鍚悗浣犵殑鎽囨潌灏嗕繚鎸佸凡鏍″噯鐘舵€併€備綘涔熷彲浠ユ妸 `jscal -p` 杩欎竴琛屽姞鍏ヤ綘鐨勫叧鏈鸿剼鏈€?
## 纭欢鐗瑰畾椹卞姩淇℃伅


鏈妭鎻忚堪鍚勪釜鐙珛鐨勭‖浠剁壒瀹氶┍鍔ㄣ€?
### 妯℃嫙鎽囨潌


analog.c 椹卞姩浣跨敤娓告垙鍙ｇ殑妯℃嫙鏍囧噯杈撳叆锛屽洜姝ゆ敮鎸佹墍鏈夋爣鍑嗘憞鏉嗕笌娓告垙鎵嬫焺銆傚畠涓烘浣跨敤浜嗛潪甯稿厛杩涚殑渚嬬▼锛岃揪鍒颁簡鍏朵粬浠讳綍绯荤粺閮芥棤娉曚紒鍙婄殑鏁版嵁绮惧害銆?
瀹冭繕鏀寔璇稿涓?CH Flightstick Pro銆乀hrustMaster FCS 鎴?6 閿強 8 閿父鎴忔墜鏌勫吋瀹圭殑棰濆甯介敭鍜屾寜閽瓑鎵╁睍銆係aitek Cyborg 'digital' 鎽囨潌涔熷彈姝ら┍鍔ㄦ敮鎸侊紝鍥犱负瀹冧滑鏈川涓婃槸鍔犲己鐗?CHF 鎽囨潌銆?
浣嗗敮涓€鍙互鑷姩妫€娴嬬殑绫诲瀷鏄細

- 2 杞淬€? 閿憞鏉?- 3 杞淬€? 閿憞鏉?- 4 杞淬€? 閿憞鏉?- Saitek Cyborg 'digital' 鎽囨潌

瀵逛簬鍏朵粬鎽囨潌绫诲瀷锛堟洿澶?鏇村皯杞淬€佸附閿拰鎸夐挳锛夌殑鏀寔锛屼綘闇€瑕佸湪灏?analog 鎻掑叆鍐呮牳鏃讹紝鍦ㄥ唴鏍稿懡浠よ鎴栨ā鍧楀懡浠よ涓婃寚瀹氱被鍨嬨€?```

	analog.map=<type1>,<type2>,<type3>,....

```
'ttype'锛堢被鍨嬶級鏄笅琛ㄤ腑鎽囨潌鐨勭被鍨嬶紝瀹氫箟绯荤粺涓父鎴忓彛涓婂瓨鍦ㄧ殑鎽囨潌锛屼粠 gameport0 寮€濮嬶紝绗簩涓?'type' 鏉＄洰瀹氫箟 gameport1 涓婄殑鎽囨潌锛屼緷姝ょ被鎺ㄣ€?
	========= =====================================================
	Type      Meaning
	========= =====================================================
	none      璇ョ鍙ｄ笂鏃犳ā鎷熸憞鏉?	auto      鑷姩妫€娴嬫憞鏉?	2btn      2 閿?n 杞存憞鏉?	y-joy     涓€鏉?Y 绾夸笂涓や釜 2 閿?2 杞存憞鏉?	y-pad     涓€鏉?Y 绾夸笂涓や釜 2 閿?2 杞存父鎴忔墜鏌?	fcs       Thrustmaster FCS 鍏煎鎽囨潌
	chf       甯?CH Flightstick 鍏煎甯介敭鐨勬憞鏉?	fullchf   CH Flightstick 鍏煎锛屽甫涓や釜甯介敭鍜?6 涓寜閽?	gamepad   4/6 閿?n 杞存父鎴忔墜鏌?	gamepad8  8 閿?2 杞存父鎴忔墜鏌?	========= =====================================================

濡傛灉浣犵殑鎽囨潌涓嶅睘浜庝笂杩颁换浣曠被鍒紝浣犲彲浠ュ皢绫诲瀷鎸囧畾涓轰竴涓暟瀛楋紝鏂规硶鏄粍鍚堜笅琛ㄤ腑鐨勪綅銆傞櫎闈炰綘纭疄娓呮鑷繁鍦ㄥ仛浠€涔堬紝鍚﹀垯涓嶅缓璁繖鏍峰仛銆傝繖骞朵笉鍗遍櫓锛屼絾涔熶笉绠€鍗曘€?
	==== =========================
	Bit  Meaning
	==== =========================
	 0   Axis X1
	 1   Axis Y1
	 2   Axis X2
	 3   Axis Y2
	 4   Button A
	 5   Button B
	 6   Button C
	 7   Button D
	 8   CHF Buttons X and Y
	 9   CHF Hat 1
	10   CHF Hat 2
	11   FCS Hat
	12   Pad Button X
	13   Pad Button Y
	14   Pad Button U
	15   Pad Button V
	16   Saitek F1-F4 Buttons
	17   Saitek Digital Mode
	19   GamePad
	20   Joy2 Axis X1
	21   Joy2 Axis Y1
	22   Joy2 Axis X2
	23   Joy2 Axis Y2
	24   Joy2 Button A
	25   Joy2 Button B
	26   Joy2 Button C
	27   Joy2 Button D
	31   Joy2 GamePad
	==== =========================

### Microsoft SideWinder 鎽囨潌


sidewinder.c 妯″潡鏀寔 Microsoft 'Digital Overdrive' 鍗忚銆傛墍鏈夊綋鍓嶆敮鎸佺殑鎽囨潌锛?
- Microsoft SideWinder 3D Pro
- Microsoft SideWinder Force Feedback Pro
- Microsoft SideWinder Force Feedback Wheel
- Microsoft SideWinder FreeStyle Pro
- Microsoft SideWinder GamePad锛堟渶澶氬洓涓紝閾惧紡杩炴帴锛?- Microsoft SideWinder Precision Pro
- Microsoft SideWinder Precision Pro USB

鍧囧彲鑷姩妫€娴嬶紝鍥犳涓嶉渶瑕佹ā鍧楀弬鏁般€?
3D Pro 鏈変竴涓渶瑕佹敞鎰忎箣澶勩€傚畠浼氭姤鍛?9 涓寜閽紝灏界鎽囨潌鍙湁 8 涓€傜 9 涓寜閽槸鎽囨潌鍚庝晶鐨勬ā寮忓紑鍏炽€備笉杩囷紝绉诲姩瀹冧細浣挎憞鏉嗗浣嶏紝骞朵娇鍏跺湪绾︿笁鍒嗕箣涓€绉掑唴鏃犲搷搴斻€傛澶栵紝鎽囨潌杩樹細閲嶆柊灞呬腑锛屽皢杩欐鏃堕棿鍐呯殑浣嶇疆浣滀负鏂扮殑涓績浣嶇疆銆傛兂鐢ㄥ氨鐢紝浣嗗厛鎯虫竻妤氥€?
SideWinder Standard 涓嶆槸鏁板瓧鎽囨潌锛屽洜姝ょ敱涓婃枃鎻忚堪鐨勬ā鎷熼┍鍔ㄦ敮鎸併€?
### Logitech ADI 璁惧


adi.c 妯″潡鏀寔 Logitech ADI 鍗忚銆傚畠搴旀敮鎸佷换浣曚娇鐢ㄨ鍗忚鐨?Logitech 璁惧銆傝繖鍖呮嫭浣嗕笉闄愪簬锛?
- Logitech CyberMan 2
- Logitech ThunderPad Digital
- Logitech WingMan Extreme Digital
- Logitech WingMan Formula
- Logitech WingMan Interceptor
- Logitech WingMan GamePad
- Logitech WingMan GamePad USB
- Logitech WingMan GamePad Extreme
- Logitech WingMan Extreme Digital 3D

ADI 璁惧鏄嚜鍔ㄦ娴嬬殑锛岃椹卞姩鍦ㄤ娇鐢?Y 绾挎垨閾惧紡杩炴帴鐨勬儏鍐典笅锛屾敮鎸佸崟涓父鎴忓彛涓婃渶澶氫袱涓紙浠绘剰缁勫悎锛夎澶囥€?
Logitech WingMan Joystick銆丩ogitech WingMan Attack銆丩ogitech WingMan Extreme 浠ュ強 Logitech WingMan ThunderPad 涓嶆槸鏁板瓧鎽囨潌锛岀敱涓婃枃鎻忚堪鐨勬ā鎷熼┍鍔ㄥ鐞嗐€侺ogitech WingMan Warrior 鍜?Logitech Magellan 鐢变笅鏂囨弿杩扮殑涓茶椹卞姩鏀寔銆侺ogitech WingMan Force 鍜?Logitech WingMan Formula Force 鐢变笅鏂囨弿杩扮殑 I-Force 椹卞姩鏀寔銆侺ogitech CyberMan 灏氫笉鏀寔銆?
### Gravis GrIP


grip.c 妯″潡鏀寔 Gravis GrIP 鍗忚銆傚畠鐩墠鏀寔锛?
- Gravis GamePad Pro
- Gravis BlackHawk Digital
- Gravis Xterminator
- Gravis Xterminator DualControl

鎵€鏈夎繖浜涜澶囬兘鏄嚜鍔ㄦ娴嬬殑锛屼綘鐢氳嚦鍙互鍦ㄥ崟涓父鎴忓彛涓婁互浠绘剰缁勫悎浣跨敤鏈€澶氫袱涓繖鏍风殑鎵嬫焺锛屾棤璁烘槸閾惧紡杩炴帴杩樻槸浣跨敤 Y 绾裤€?
GrIP MultiPort 灏氫笉鏀寔銆侴ravis Stinger 鏄覆琛岃澶囷紝鐢?stinger 椹卞姩鏀寔銆傚叾浠?Gravis 鎽囨潌鐢辨ā鎷熼┍鍔ㄦ敮鎸併€?
### FPGaming A3D 涓?MadCatz A3D


鐢?FPGaming 鍒涘缓鐨?Assassin 3D 鍗忚锛屾棦琚?FPGaming 鑷繁浣跨敤锛屼篃琚巿鏉冪粰 MadCatz銆侫3D 璁惧鐢?a3d.c 妯″潡鏀寔銆傚畠鐩墠鏀寔锛?
- FPGaming Assassin 3D
- MadCatz Panther
- MadCatz Panther XL

鎵€鏈夎繖浜涜澶囬兘鏄嚜鍔ㄦ娴嬬殑銆傜敱浜?Assassin 3D 涓?Panther 鍏佽杩炴帴妯℃嫙鎽囨潌锛屼綘杩橀渶瑕佸姞杞芥ā鎷熼┍鍔ㄦ潵澶勭悊鎵€杩炴帴鐨勬憞鏉嗐€?
杞ㄨ抗鐞冨簲浣滀负鏅€氶紶鏍囬厤鍚?USB mousedev 妯″潡宸ヤ綔銆傛湁鍏冲浣曡缃?USB 榧犳爣锛岃鍙傝 USB 鏂囨。銆?
### ThrustMaster DirectConnect (BSP)


tmdc.c 妯″潡鏀寔 TM DirectConnect (BSP) 鍗忚銆傝繖鍖呮嫭浣嗕笉闄愪簬锛?
- ThrustMaster Millennium 3D Interceptor
- ThrustMaster 3D Rage Pad
- ThrustMaster Fusion Digital Game Pad

鏈洿鎺ユ敮鎸佷絾鏈夋湜宸ヤ綔鐨勮澶囷細

- ThrustMaster FragMaster
- ThrustMaster Attack Throttle

濡傛灉浣犳嫢鏈夊叾涓箣涓€锛岃鑱旂郴鎴戙€?
TMDC 璁惧鏄嚜鍔ㄦ娴嬬殑锛屽洜姝や笉闇€瑕佺粰妯″潡浼犲弬鏁般€備娇鐢?Y 绾匡紝鏈€澶氬彲灏嗕袱涓?TMDC 璁惧杩炴帴鍒板崟涓父鎴忓彛銆?
### Creative Labs Blaster


cobra.c 妯″潡鏀寔 Blaster 鍗忚銆傚畠浠呮敮鎸侊細

- Creative Blaster GamePad Cobra

浣跨敤 Y 绾匡紝鏈€澶氬彲鍦ㄥ崟涓父鎴忓彛涓婁娇鐢ㄤ袱涓繖鏍风殑璁惧銆?
### Genius Digital 鎽囨潌


gf2k.c 妯″潡鏀寔 Genius 鏁板瓧閫氫俊鎽囨潌銆傝繖鍖呮嫭锛?
- Genius Flight2000 F-23 鎽囨潌
- Genius Flight2000 F-31 鎽囨潌
- Genius G-09D 娓告垙鎵嬫焺

鍏朵粬 Genius 鏁板瓧鎽囨潌灏氫笉鏀寔锛屼絾鐩稿綋瀹规槗娣诲姞鏀寔銆?
### InterAct Digital 鎽囨潌


interact.c 妯″潡鏀寔 InterAct 鏁板瓧閫氫俊鎽囨潌銆傝繖鍖呮嫭锛?
- InterAct HammerHead/FX 娓告垙鎵嬫焺
- InterAct ProPad8 娓告垙鎵嬫焺

鍏朵粬 InterAct 鏁板瓧鎽囨潌灏氫笉鏀寔锛屼絾鐩稿綋瀹规槗娣诲姞鏀寔銆?
### PDPI Lightning 4 娓告垙鍗?

lightning.c 妯″潡鏀寔 PDPI Lightning 4 娓告垙鍗°€傛ā鍧楀姞杞藉悗锛屽彲鐢ㄦā鎷熼┍鍔ㄦ潵澶勭悊鎽囨潌銆傛暟瀛楅€氫俊鎽囨潌鍙兘鍦ㄧ鍙?0 涓婂伐浣滐紝鑰屼娇鐢?Y 绾匡紝浣犲彲浠ュ皢鏈€澶?8 涓ā鎷熸憞鏉嗚繛鎺ュ埌鍗曚釜 L4 鍗′笂锛涘鏋滀綘鐨勭郴缁熶腑鏈変袱寮犲崱锛屽垯鏄?16 涓€?
### Trident 4DWave / Aureal Vortex


甯︽湁 Trident 4DWave DX/NX 鎴?Aureal Vortex/Vortex2 鑺墖缁勭殑澹板崱鎻愪緵"澧炲己娓告垙鍙ｏ紙Enhanced Game Port锛?妯″紡锛岀敱澹板崱璐熻矗杞鎽囨潌銆俻cigame.c 妯″潡鏀寔姝ゆā寮忋€傚姞杞藉悗锛屾ā鎷熼┍鍔ㄥ嵆鍙娇鐢ㄨ繖浜涙父鎴忓彛鐨勫寮虹壒鎬с€?
### Crystal SoundFusion


甯︽湁 Crystal SoundFusion 鑺墖缁勭殑澹板崱鎻愪緵"澧炲己娓告垙鍙ｏ紙Enhanced Game Port锛?锛屼笌涓婃枃鐨?4DWave 鎴?Vortex 闈炲父鐩镐技銆傝繖涓€鐐癸紝浠ュ強 SoundFusion 绔彛鐨勬櫘閫氭ā寮忥紝閮界敱 cs461x.c 妯″潡鏀寔銆?
### SoundBlaster Live!


Live! 鏈変竴涓壒娈婄殑 PCI 娓告垙鍙ｏ紝灏界瀹冧笉鍍?4DWave 鍙婂叾鍚岀被閭ｆ牱鎻愪緵浠讳綍"澧炲己"鍔熻兘锛屼絾姣斿叾 ISA 鍚岀被瑕佸揩寰楀銆傚畠涔熼渶瑕佺壒娈婃敮鎸侊紝鍥犳浣跨敤 emu10k1-gp.c 妯″潡锛岃€屼笉鏄櫘閫氱殑 ns558.c銆?
### SoundBlaster 64 涓?128 - ES1370 涓?ES1371銆丒SS Solo1 涓?S3 SonicVibes


杩欎簺 PCI 澹板崱鏈夌壒瀹氱殑娓告垙鍙ｃ€傚畠浠敱澹板崱椹卞姩鑷韩澶勭悊銆傝纭繚涓轰綘鐨勭浉搴斿０鍗″湪鎽囨潌鑿滃崟涓€夋嫨娓告垙鍙ｆ敮鎸侊紝骞跺湪澹伴煶鑿滃崟涓€夋嫨澹板崱鏀寔銆?
### Amiga


杩炴帴鍒?Amiga 鐨?Amiga 鎽囨潌鐢?amijoy.c 椹卞姩鏀寔銆傜敱浜庡畠浠棤娉曡鑷姩妫€娴嬶紝璇ラ┍鍔ㄦ湁涓€涓懡浠よ锛?
	amijoy.map=<a>,<b>

a 涓?b 瀹氫箟杩炴帴鍒?Amiga 鐨?JOY0DAT 涓?JOY1DAT 绔彛鐨勬憞鏉嗐€?
	====== ===========================
	Value  Joystick type
	====== ===========================
	  0    None
	  1    1-button digital joystick
	====== ===========================

鐩墠涓嶆敮鎸佹洿澶氭憞鏉嗙被鍨嬶紝浣嗗鏋滄垜鎵嬭竟鑳芥嬁鍒颁竴鍙?Amiga锛屾湭鏉ヨ繖搴旇浼氭敼鍙樸€?
### 娓告垙涓绘満涓?8 浣嶆墜鏌勫強鎽囨潌


杩欎簺鎵嬫焺鍜屾憞鏉嗗苟闈炰负 PC 浠ュ強杩愯 Linux 鐨勫叾浠栬绠楁満璁捐锛岄€氬父闇€瑕侀€氳繃骞跺彛杩炴帴鐨勭壒娈婅繛鎺ュ櫒銆?
鏇村璧勮璇峰弬瑙?joystick-parport銆?
### SpaceTec/LabTec 璁惧


SpaceTec 涓茶璁惧浣跨敤 SpaceWare 鍗忚閫氫俊銆俿paceorb.c 鍜?spaceball.c 椹卞姩鏀寔璇ュ崗璁€俿paceorb.c 褰撳墠鏀寔鐨勮澶囷細

- SpaceTec SpaceBall Avenger
- SpaceTec SpaceOrb 360

spaceball.c 褰撳墠鏀寔鐨勮澶囷細

- SpaceTec SpaceBall 4000 FLX

闄や簡鍦ㄥ唴鏍镐腑鎷ユ湁 spaceorb/spaceball 鍜?serport 妯″潡澶栵紝浣犺繕闇€瑕佸皢涓€涓覆鍙ｈ繛鎺ュ埌瀹冦€備负姝わ紝杩愯
```

	inputattach --spaceorb /dev/tts/x &

```
```

	inputattach --spaceball /dev/tts/x &

```
鍏朵腑 /dev/tts/x 鏄澶囨墍杩炴帴鐨勪覆鍙ｃ€傚畬鎴愭鎿嶄綔鍚庯紝璁惧灏嗚鎶ュ憡骞跺紑濮嬪伐浣溿€?
SpaceOrb 鏈変竴涓渶瑕佹敞鎰忎箣澶勩€傜 6 涓寜閽紝鍗崇悆浣撳簳閮ㄧ殑閭ｄ釜锛屽敖绠¤鎶ュ憡涓烘櫘閫氭寜閽紝浣嗕細瀵艰嚧 spaceorb 鍐呴儴閲嶆柊灞呬腑锛屽皢闆剁偣绉诲姩鍒版寜涓嬫寜閽椂鐞冩墍鍦ㄧ殑浣嶇疆銆傚洜姝わ紝鍦ㄥ皢瀹冪粦瀹氬埌鍏朵粬鍔熻兘涔嬪墠璇峰厛鎯虫竻妤氥€?
SpaceTec SpaceBall 2003 FLX 涓?3003 FLX 灏氫笉鏀寔銆?
### Logitech SWIFT 璁惧


warrior.c 妯″潡鏀寔 SWIFT 涓茶鍗忚銆傚畠鐩墠浠呮敮鎸侊細

- Logitech WingMan Warrior

浣嗘湭鏉ワ紝Logitech CyberMan锛堝師濮嬬増鏈紝鑰岄潪 CM2锛変篃鍙兘寰楀埌鏀寔銆傝浣跨敤妯″潡锛屼綘闇€瑕佸湪涔嬪悗杩愯 inputattach
```

	inputattach --warrior /dev/tts/x &

```
/dev/tts/x 鏄綘鐨?Warrior 鎵€杩炴帴鐨勪覆鍙ｃ€?
### Magellan / Space Mouse


鐢?LogiCad3d锛堝墠韬?Space Systems锛変负璁稿鍏朵粬鍏徃锛圠ogitech銆丠P 绛夛級鍒堕€犵殑 Magellan锛堟垨 Space Mouse锛夛紝鐢?joy-magellan 妯″潡鏀寔銆傚畠鐩墠浠呮敮鎸侊細

- Magellan 3D
- Space Mouse

鍨嬪彿锛?Plus' 鐗堟湰鐨勯澶栨寜閽皻涓嶆敮鎸併€?```

	inputattach --magellan /dev/tts/x &

```
鍛戒护銆備箣鍚?Magellan 灏嗚妫€娴嬨€佸垵濮嬪寲銆佸彂鍑鸿渹楦ｏ紝骞朵笖 /dev/input/jsX 璁惧搴斿彉寰楀彲鐢ㄣ€?
### I-Force 璁惧


鎵€鏈?I-Force 璁惧閮界敱 iforce 妯″潡鏀寔銆傝繖鍖呮嫭锛?
- AVB Mag Turbo Force
- AVB Top Shot Pegasus
- AVB Top Shot Force Feedback Racing Wheel
- Boeder Force Feedback Wheel
- Logitech WingMan Force
- Logitech WingMan Force Wheel
- Guillemot Race Leader Force Feedback
- Guillemot Force Feedback Racing Wheel
- Thrustmaster Motor Sport GT

```

	inputattach --iforce /dev/tts/x &

```
鍛戒护銆備箣鍚?I-Force 璁惧灏嗚妫€娴嬶紝骞朵笖 /dev/input/jsX 璁惧搴斿彉寰楀彲鐢ㄣ€?
濡傛灉浣犻€氳繃 USB 绔彛浣跨敤璁惧锛屽垯涓嶉渶瑕?inputattach 鍛戒护銆?
I-Force 椹卞姩鐜板湪鏀寔閫氳繃 event 鎺ュ彛杩涜鍔涘弽棣堛€?
璇锋敞鎰忥紝Logitech WingMan 3D 璁惧_涓峗鍙楁妯″潡鏀寔锛岃€屾槸鐢?hid 鏀寔銆傝繖浜涜澶囦笉鏀寔鍔涘弽棣堛€侺ogitech 娓告垙鎵嬫焺涔熸槸 hid 璁惧銆?
### Gravis Stinger 娓告垙鎵嬫焺


涓洪厤鍚堢瑪璁版湰鐢佃剳浣跨敤鑰岃璁＄殑 Gravis Stinger 涓插彛娓告垙鎵嬫焺锛岀敱 stinger.c 妯″潡鏀寔銆傝浣跨敤瀹冿紝杩炴帴
```

	inputattach --stinger /dev/tty/x &

```
鍏朵腑 x 鏄覆鍙ｇ紪鍙枫€?
## 鎺掗殰


浣犻亣鍒颁竴浜涢棶棰樻湁鐩稿綋楂樼殑姒傜巼銆傝娴嬭瘯椹卞姩鏄惁宸ヤ綔锛屽鏈夌枒闂紝鍙娇鐢?jstest 宸ュ叿鐨勬煇浜涙ā寮忋€傛渶鏈夌敤鐨勬ā寮忔槸 "normal"鈥斺€旈拡瀵?1.x
```

	jstest --normal /dev/input/js0
	jstest --old    /dev/input/js0

```
```

	evtest /dev/input/event0

```
鍝︼紝杩樿闃呰 FAQ锛?)

## FAQ


:Q: 杩愯 'jstest /dev/input/js0' 鍑虹幇 "File not found" 閿欒銆傚師鍥犳槸浠€涔堬紵
:A: 璁惧鏂囦欢涓嶅瓨鍦ㄣ€傚垱寤哄畠浠紙瑙佺 2.2 鑺傦級銆?
:Q: 鑳藉惁灏嗘垜鏃х殑 Atari/Commodore/Amiga/娓告垙涓绘満鎽囨潌鎴栨墜鏌勶紙浣跨敤 9 閽?D 鍨?Cannon 杩炴帴鍣級杩炴帴鍒版垜 PC 鐨勪覆鍙ｏ紵
:A: 鍙互锛屼絾浼氱儳姣佷綘鐨勪覆鍙ｆ垨鎵嬫焺銆傚綋鐒讹紝瀹冧笉浼氬伐浣溿€?
:Q: 鎴戠殑鎽囨潌鍦?Quake / Quake 2 涓笉璧蜂綔鐢ㄣ€傚師鍥犳槸浠€涔堬紵
:A: Quake / Quake 2 涓嶆敮鎸佹憞鏉嗐€備娇鐢?joy2key 涓哄畠浠ā鎷熸寜閿€?