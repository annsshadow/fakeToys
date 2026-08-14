## USB serial


## 绠€浠?


  USB 涓插彛锛坰erial锛夐┍鍔ㄧ洰鍓嶆敮鎸佸绉嶄笉鍚岀殑 USB 杞覆鍙ｏ紙USB to serial锛夎浆鎹㈠櫒
  浜у搧锛屼互鍙婁竴浜涗粠鐢ㄦ埛绌洪棿锛坲serspace锛変娇鐢ㄤ覆鍙ｆ帴鍙ｄ笌璁惧杩涜閫氫俊鐨勮澶囥€?

  鍏充簬涓嶅悓璁惧鐨勫叿浣撲俊鎭紝璇峰弬瑙佷笅鏂囧悇涓骇鍝佺殑绔犺妭銆?


## 閰嶇疆


  褰撳墠璇ラ┍鍔ㄤ竴娆℃渶澶氬彲澶勭悊 256 涓笉鍚岀殑涓插彛鎺ュ彛銆?

    椹卞姩浣跨敤鐨勪富璁惧鍙凤紙major number锛変负 188锛屽洜姝よ浣跨敤姝ら┍鍔紝鍙墽琛岋細
```
	mknod /dev/ttyUSB0 c 188 0
	mknod /dev/ttyUSB1 c 188 1
	mknod /dev/ttyUSB2 c 188 2
	mknod /dev/ttyUSB3 c 188 3
		.
		.
		.
	mknod /dev/ttyUSB254 c 188 254
	mknod /dev/ttyUSB255 c 188 255
```

  褰撹澶囪杩炴帴骞惰椹卞姩璇嗗埆鍚庯紝椹卞姩浼氬悜绯荤粺鏃ュ織鎵撳嵃璇ヨ澶囨墍缁戝畾鍒扮殑鑺傜偣锛坣ode锛夈€?


## 鏀寔鐨勫叿浣撹澶?


### ConnectTech WhiteHEAT 鍥涚鍙ｈ浆鎹㈠櫒


  ConnectTech 闈炲父绉瀬鍦版彁渚涘叾璁惧鐨勭浉鍏充俊鎭紝鍖呮嫭鎻愪緵浜嗕竴鍙扮敤浜庢祴璇曠殑璁惧銆?

  璇ラ┍鍔ㄧ敱 Connect Tech Inc. 瀹樻柟鏀寔銆?
  http://www.connecttech.com

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Connect Tech 鐨勬敮鎸侀儴闂細support@connecttech.com


### HandSpring Visor銆丳alm USB 涓?Cli茅 USB 椹卞姩


  璇ラ┍鍔ㄩ€傜敤浜庢墍鏈?HandSpring USB銆丳alm USB 鍜?Sony Cli茅 USB 璁惧銆?

  鍙湁褰撹澶囧皾璇曡繛鎺ュ埌涓绘満锛坔ost锛夋椂锛屽畠鎵嶄細浣滀负涓€涓湁鏁堢殑 USB 璁惧鍑虹幇鍦?
  涓绘満闈㈠墠銆傚彂鐢熸鎯呭喌鏃讹紝璁惧浼氳姝ｇ‘鏋氫妇锛坋numerated锛夛紝鍒嗛厤涓€涓鍙ｏ紝鐒跺悗
  _搴斿綋_ 鍙互杩涜閫氫俊浜嗐€傚綋璁惧琚Щ闄わ紝鎴栬澶囩鍙栨秷浜嗚繛鎺ユ椂锛岄┍鍔ㄤ細濡ュ杽娓呯悊銆?

  娉ㄦ剰锛?
    杩欐剰鍛崇潃涓轰簡涓庤澶囬€氫俊锛屽繀椤诲湪灏濊瘯璁╀换浣曠▼搴忎笌璁惧閫氫俊涔嬪墠锛屽厛鎸変笅鍚屾
    锛坰ync锛夋寜閽€傝繖涓?pilot-xfer 鍙婂叾浠栬蒋浠跺寘鐨勫綋鍓嶆枃妗ｇ浉鎮栵紝浣嗙敱浜庤澶囦腑鐨?
    纭欢闄愬埗锛岃繖鏄敮涓€鍙鐨勬柟寮忋€?

  褰撹澶囪繛鎺ュ悗锛屽皾璇曞湪绗簩涓鍙ｄ笂涓庡叾閫氫俊锛堝鏋滀綘绯荤粺涓病鏈夊叾瀹?usb-serial
  璁惧锛岄€氬父涓?/dev/ttyUSB1锛夈€傜郴缁熸棩蹇椾細鍛婅瘔浣?HotSync 浼犺緭搴斾娇鐢ㄥ摢涓鍙ｃ€?
  鈥淕eneric鈥?绔彛鍙敤浜庡叾瀹冭澶囬€氫俊锛屼緥濡備竴鏉?PPP 閾捐矾銆?

  瀵逛簬涓€浜?Sony Cli茅 璁惧锛屽繀椤讳娇鐢?/dev/ttyUSB0 鏉ヤ笌璁惧閫氫俊銆傛墍鏈?OS 鐗堟湰 3.5
  鐨勮澶囷紝浠ュ強澶у鏁板凡鍗囩骇鍒拌緝鏂?OS 鐗堟湰鐨勮澶囬兘濡傛銆傚叧浜庡摢涓槸姝ｇ‘鐨勭鍙ｏ紝
  璇锋煡鐪嬪唴鏍哥郴缁熸棩蹇椾腑鐨勪俊鎭€?

  濡傛灉鍦ㄦ寜涓嬪悓姝ユ寜閽悗锛岀郴缁熸棩蹇椾腑娌℃湁浠讳綍鏄剧ず锛屽皾璇曢噸缃澶囷紝鍏堢儹閲嶇疆锛坔ot
  reset锛夛紝蹇呰鏃跺啀鍐烽噸缃紙cold reset锛夈€傛湁浜涜澶囧湪涓?USB 绔彛姝ｅ父閫氫俊涔嬪墠
  闇€瑕佽繖鏍峰仛銆?

  鏈紪璇戣繘鍐呮牳鐨勮澶囧彲浠ラ€氳繃妯″潡鍙傛暟鎸囧畾銆備緥濡傦細
  modprobe visor vendor=0x54c product=0x66

  鍏充簬椹卞姩鐨勮繖涓€閮ㄥ垎锛屾湁涓€涓綉椤靛拰閭欢鍒楄〃锛?
  http://sourceforge.net/projects/usbvisor/

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Greg Kroah-Hartman锛歡reg@kroah.com


### PocketPC PDA 椹卞姩


  璇ラ┍鍔ㄥ彲鐢ㄤ簬閫氳繃 USB 绾跨紗/搴曞骇锛坈radle锛夎繛鎺ュ埌杩愯 Windows CE 3.0 鎴?
  PocketPC 2002 鐨?Compaq iPAQ銆丠P Jornada銆丆asio EM500 鍙婂叾瀹?PDA銆?
  澶у鏁拌 ActiveSync 鏀寔鐨勮澶囧紑绠卞嵆鍙娇鐢ㄣ€傚浜庡叾瀹冭澶囷紝璇蜂娇鐢ㄦā鍧楀弬鏁版寚瀹?
  浜у搧锛坧roduct锛夊拰鍘傚晢锛坴endor锛塱d銆備緥濡傦細
  modprobe ipaq vendor=0x3f0 product=0x1125

  椹卞姩鎻愪緵涓€涓覆鍙ｆ帴鍙ｏ紙閫氬父鍦?/dev/ttyUSB0 涓婏級锛屽彲鍦ㄦ涔嬩笂杩愯 ppp 骞跺缓绔嬩笌
  PDA 鐨?TCP/IP 閾捐矾銆傚畬鎴愬悗锛屼綘鍙互浼犺緭鏂囦欢銆佸浠姐€佷笅杞介偖浠剁瓑銆備娇鐢?USB 鏈€鏄捐憲鐨?
  浼樺娍鏄€熷害鈥斺€旀垜鍙互浠?73 鍒?113 kbytes/sec 鐨勯€熷害涓庢垜鐨?iPAQ 杩涜涓嬭浇/涓婁紶銆?

  璇ラ┍鍔ㄥ彧鏄埄鐢?USB 杩炴帴鎵€闇€鐨勪竴缁勭粍浠朵箣涓€銆傝璁块棶 http://synce.sourceforge.net锛?
  鍏朵腑鍖呭惈鎵€闇€鐨勮蒋浠跺寘浠ュ強绠€鍗曠殑鍒嗘鎿嶄綔鎸囧崡锛坔owto锛夈€?

  杩炴帴鍚庯紝浣犲彲浠ヤ娇鐢?Win CE 绔殑绋嬪簭濡?ftpView銆丳ocket Outlook锛屼互鍙?Linux 绔殑
  xcerdisp銆乻ynce 宸ュ叿銆?

  瑕佷娇鐢?Pocket IE锛岃鎸夌収 http://www.tekguru.co.uk/EM500/usbtonet.htm 缁欏嚭鐨?
  璇存槑锛屽湪 Win98 涓婂疄鐜板悓鏍风殑鏁堟灉銆傜渷鐣ヤ唬鐞嗘湇鍔″櫒閮ㄥ垎锛涗笌 Win98 涓嶅悓锛孡inux 瀹屽叏
  鑳藉杞彂鏁版嵁鍖呫€傝嚦灏戝 iPAQ 杩橀渶瑕佷竴澶勪慨鏀光€斺€旈€氳繃
  Start/Settings/Connections 鑿滃崟绂佺敤鑷姩鍚屾锛坅utosync锛夛紝鍙栨秷鍕鹃€?
  鈥淎utomatically synchronize ...鈥?妗嗐€傝繘鍏?Start/Programs/Connections锛岃繛鎺ョ嚎缂嗗苟
  閫夋嫨 鈥渦sbdial鈥濓紙鎴栦綘涓烘柊 USB 杩炴帴璧风殑鍚嶅瓧锛夈€備綘鏈€缁堝簲褰撲細鐪嬪埌涓€涓?
  鈥淐onnected to usbdial鈥?绐楀彛锛岀姸鎬佹樉绀轰负宸茶繛鎺ャ€傜幇鍦ㄥ惎鍔?PIE 骞跺紑濮嬫祻瑙堛€?

  濡傛灉鐢变簬鏌愮鍘熷洜鏃犳硶宸ヤ綔锛岃浠ユā鍧楀弬鏁?鈥渄ebug鈥?璁句负 1 鍔犺浇 usbserial 鍜?ipaq
  妯″潡锛屽苟妫€鏌ョ郴缁熸棩蹇椼€備綘涔熷彲浠ュ皾璇曞湪寤虹珛杩炴帴鍓嶅浣犵殑 PDA 杩涜杞噸缃紙soft-reset锛夈€?

  鏍规嵁浣犵殑 PDA锛屽彲鑳借繕鏈夊叾瀹冨姛鑳藉彲鐢ㄣ€傛嵁 Wes Cilldhaire
  <billybobjoehenrybob@hotmail.com> 鎵€杩帮紝鍦?Toshiba E570 涓婏紝鈥︹€﹀鏋滀綘鍚姩杩涘叆
  寮曞鍔犺浇绋嬪簭锛坆ootloader锛夛紙鍦ㄦ寜涓嬮噸缃寜閽椂鎸変綇鐢垫簮閿紝骞舵寔缁寜浣忕數婧愰敭鐩村埌
  鏄剧ず寮曞鍔犺浇绋嬪簭鐢婚潰锛夛紝鐒跺悗灏嗗叾鏀惧叆宸插姞杞?ipaq 椹卞姩鐨勫簳搴т腑锛屽湪 /dev/ttyUSB0
  涓婃墦寮€涓€涓粓绔紝瀹冧細缁欎綘涓€涓?鈥淯SB Reflash鈥?缁堢锛屽彲鐢ㄤ簬鍒峰啓 ROM锛屼互鍙?microP
  浠ｇ爜鈥︹€﹁繖鏍峰氨涓嶉渶瑕?Toshiba 浠峰€?350 缇庡厓鐨勭敤浜庡埛鍐欑殑涓插彛绾跨紗浜嗭紒锛?D
  娉ㄦ剰锛氳繖灏氭湭缁忚繃娴嬭瘯銆備娇鐢ㄩ闄╄嚜璐熴€?

  鍏充簬椹卞姩鐨勪换浣曢棶棰樻垨鏁呴殰锛岃鑱旂郴 Ganesh Varadarajan锛?ganesh@veritas.com>


### Keyspan PDA 涓插彛閫傞厤鍣?


  鍗曠鍙?DB-9 涓插彛閫傞厤鍣紝浣滀负 iMac 鐨?PDA 閫傞厤鍣ㄦ帹骞匡紙涓昏鍦?Macintosh 鐩綍涓?
  閿€鍞紝鏄竴涓崐閫忔槑鐧?缁跨浉闂寸殑杞崲鍣紙dongle锛夛級銆傜浉褰撶畝鍗曠殑璁惧銆傚浐浠朵负鑷埗
  锛坔omebrew锛夈€傝椹卞姩涔熼€傜敤浜?Xircom/Entrega 鍗曠鍙ｄ覆鍙ｉ€傞厤鍣ㄣ€?

  褰撳墠鐘舵€侊細

   鍙互姝ｅ父宸ヤ綔鐨勯儴鍒嗭細
     - 鍩烘湰鐨勮緭鍏?杈撳嚭锛堜互 'cu' 娴嬭瘯锛?
     - 涓插彛绾胯矾璺熶笉涓婃椂鐨勯樆濉炲啓鍏ワ紙blocking write锛?
     - 鏀瑰彉娉㈢壒鐜囷紙鏈€楂?115200锛?
     - 鑾峰彇/璁剧疆璋冨埗瑙ｈ皟鍣ㄦ帶鍒跺紩鑴氾紙TIOCM{GET,SET,BIS,BIC}锛?
     - 鍙戦€?break锛堝敖绠℃寔缁椂闂寸湅璧锋潵鍙枒锛?

   涓嶈兘姝ｅ父宸ヤ綔鐨勯儴鍒嗭細
     - 璁惧瀛楃涓诧紙鍐呮牳璁板綍鐨勶級甯︽湁灏鹃殢鐨勪簩杩涘埗鍨冨溇
     - 璁惧 ID 涓嶆纭紝鍙兘涓庡叾浠?Keyspan 浜у搧鍐茬獊
     - 鏀瑰彉娉㈢壒鐜囧簲褰撳啿鍒?tx/rx 浠ラ伩鍏嶅嚭鐜版畫缂虹殑鍗婁釜瀛楃

   todo 鍒楄〃涓殑澶ч」锛?
     - 鏍￠獙浣嶏紙parity锛夛紝姣忓瓧绗?7 鎴?8 浣嶏紝1 鎴?2 涓仠姝綅
     - 纭欢娴佹帶锛圚W flow control锛?
     - 骞堕潪鎵€鏈夋爣鍑?USB 鎻忚堪绗﹂兘琚鐞嗭細
       Get_Status銆丼et_Feature銆丱_NONBLOCK銆乻elect()

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Brian Warner锛歸arner@lothar.com


### Keyspan USA 绯诲垪涓插彛閫傞厤鍣?


  鍗曘€佸弻鍜屽洓绔彛閫傞厤鍣ㄢ€斺€旈┍鍔ㄤ娇鐢?Keyspan 鎻愪緵鐨勫浐浠讹紝骞跺湪鍏舵敮鎸佷笅寮€鍙戙€?

  褰撳墠鐘舵€侊細

    USA-18X銆乁SA-28X銆乁SA-19銆乁SA-19W 鍜?USA-49W 鍧囧彈鏀寔锛屽苟宸插湪涓嶅悓娉㈢壒鐜囦笅
    浠?8-N-1 瀛楃璁剧疆杩涜浜嗙浉褰撳厖鍒嗙殑娴嬭瘯銆傚叾瀹冨瓧绗﹂暱搴﹀拰鏍￠獙璁剧疆鐩墠灏氭湭娴嬭瘯銆?

    USA-28 灏氭湭鏀寔锛屼絾鏀寔瀹冨簲褰撶浉褰撶洿鎺ャ€傚鏋滀綘闇€瑕佹鍔熻兘锛岃鑱旂郴缁存姢鑰呫€?

  鏇村淇℃伅瑙侊細

        http://www.carnationsoftware.com/carnation/Keyspan.html

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Hugh Blemings锛歨ugh@misc.nu


### FTDI 鍗曠鍙ｄ覆鍙ｉ┍鍔?


  杩欐槸涓€涓崟绔彛 DB-25 涓插彛閫傞厤鍣ㄣ€?

  鏀寔鐨勮澶囧寘鎷細

                - TripNav TN-200 USB GPS
                - Navis Engineering Bureau CH-4711 USB GPS

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Bill Ryder銆?


### ZyXEL omni.net lcd plus ISDN TA


  杩欐槸涓€涓?ISDN TA銆傝灏嗘垚鍔熶笌闂閮芥姤鍛婄粰 azummo@towertech.it


### Cypress M8 CY4601 绯诲垪涓插彛椹卞姩


  璇ラ┍鍔ㄥぇ閮ㄥ垎鐢?Neil 鈥渒oyama鈥?Whelchel 寮€鍙戙€傝嚜姝ゅ墠鐨勫舰寮忎互鏉ュ凡寰楀埌鏀硅繘锛屼互鏀寔
  鍔ㄦ€佷覆鍙ｇ嚎璺缃苟鏀瑰杽浜嗙嚎璺鐞嗐€傝椹卞姩澶ч儴鍒嗙ǔ瀹氾紝骞跺凡鍦?smp 鏈哄櫒锛堝弻 P2锛?
  涓婃祴璇曡繃銆?

    CY4601 绯诲垪鏀寔鐨勮姱鐗囩粍锛?

		CY7C63723, CY7C63742, CY7C63743, CY7C64013

    鏀寔鐨勮澶囷細

  - DeLorme 鐨?USB Earthmate GPS锛圫iRF Star II lp 鏋舵瀯锛?
  - Cypress HID->COM RS232 閫傞厤鍣?

		Note:
			Cypress Semiconductor 澹扮О涓庤
			hid->com 璁惧娌℃湁浠讳綍鍏宠仈銆?

     澶у鏁颁娇鐢?CY4601 绯诲垪鑺墖缁勭殑璁惧搴斿綋閮藉彲涓庤椹卞姩閰嶅悎宸ヤ綔锛屽彧瑕佸畠浠?
     閬靛惊 CY4601 usbserial 瑙勮寖銆?

    鎶€鏈鏄庯細

        Earthmate 榛樿浠?4800 8N1 鍚姩鈥︹€﹂┍鍔ㄥ湪鍚姩鏃朵細鍒濆鍖栦负璇ヨ缃€?
        usbserial 鏍稿績鎻愪緵鍏朵綑鐨?termios 璁剧疆锛屼互鍙婁竴浜涜嚜瀹氫箟鐨?termios锛?
        浠ヤ娇杈撳嚭鏍煎紡姝ｇ‘涓斿彲瑙ｆ瀽銆?

```
		$PSRF100,<protocol>,<baud>,<databits>,<stopbits>,<parity>*CHECKSUM
		$PSRF100,0,9600,8,1,0*0C

		It should then be sufficient to change the port termios to match this
		to begin communicating.

	As far as I can tell it supports pretty much every sirf command as
	documented online available with firmware 2.31, with some unknown
	message ids.

	The hid->com adapter can run at a maximum baud of 115200bps.  Please note
	that the device has trouble or is incapable of raising line voltage properly.
	It will be fine with null modem links, as long as you do not try to link two
	together without hacking the adapter to set the line high.

	The driver is smp safe.  Performance with the driver is rather low when using
	it for transferring files.  This is being worked on, but I would be willing to
	accept patches.  An urb queue or packet buffer would likely fit the bill here.

	If you have any questions, problems, patches, feature requests, etc. you can
	contact me here via email:

					dignome@gmail.com

		(your problems/patches can alternately be submitted to usb-devel)


```

### Digi AccelePort 椹卞姩


  璇ラ┍鍔ㄦ敮鎸?Digi AccelePort USB 2 鍜?4 璁惧锛屽嵆 2 绔彛锛堝鍔犱竴涓苟鍙ｏ級鍜?4 绔彛鐨?
  USB 涓插彛杞崲鍣ㄣ€傝椹卞姩鐩墠灏氫笉鏀寔 Digi AccelePort USB 8銆?

  璇ラ┍鍔ㄥ湪 SMP 涓嬮厤鍚?usb-uhci 椹卞姩宸ヤ綔銆傚畠鍦?SMP 涓嬮厤鍚?uhci 椹卞姩涓嶅伐浣溿€?

  璇ラ┍鍔ㄥぇ浣撲笂鍙伐浣滐紝灏界鎴戜滑杩樻湁鍑犱釜 ioctl 寰呭疄鐜帮紝浠ュ強鏈€鍚庣殑娴嬭瘯涓庤皟璇曡鍋氥€?
  USB 2 涓婄殑骞跺彛浣滀负涓插彛杞苟鍙ｈ浆鎹㈠櫒琚敮鎸侊紱鎹㈠彞璇濊锛屽湪 Linux 涓婂畠琛ㄧ幇涓哄彟涓€涓?
  USB 涓插彛锛屽敖绠＄墿鐞嗕笂瀹冪‘瀹炴槸涓€涓苟鍙ｃ€侱igi AccelePort USB 8 灏氫笉鍙楁敮鎸併€?

  鍏充簬姝ら┍鍔ㄧ殑闂鎴栨晠闅滐紝璇疯仈绯?Peter Berger锛坧berger@brimson.com锛夋垨 Al Borchers
  锛坅lborchers@steinerpoint.com锛夈€?


### Belkin USB 涓插彛閫傞厤鍣?F5U103


  鏉ヨ嚜 Belkin 鐨勫崟绔彛 DB-9/PS-2 涓插彛閫傞厤鍣紝鍥轰欢鐢?eTEK Labs 鎻愪緵銆侾eracom 鍗曠鍙?
  涓插彛閫傞厤鍣ㄤ互鍙?GoHubs 閫傞厤鍣ㄤ篃鍙笌璇ラ┍鍔ㄩ厤鍚堝伐浣溿€?

  褰撳墠鐘舵€侊細

    浠ヤ笅椤圭洰宸叉祴璇曞苟鍙伐浣滐細

      - 娉㈢壒鐜?     300-230400
      - 鏁版嵁浣?     5-8
      - 鍋滄浣?     1-2
      - 鏍￠獙浣?     N,E,O,M,S
      - 鎻℃墜        None銆丼oftware锛圶ON/XOFF锛夈€丠ardware锛圕TSRTS銆丆TSDTR锛塠^1^]_
      - Break        璁剧疆涓庢竻闄?
      - 绾胯矾鎺у埗    杈撳叆/杈撳嚭鏌ヨ涓庢帶鍒?[^2^]_

  .. [^1^]
         纭欢杈撳叆娴佹帶浠呭湪鍥轰欢鐗堟湰楂樹簬 2.06 鏃跺惎鐢ㄣ€傝闃呰鎻忚堪 Belkin 鍥轰欢鍕樿
         锛坋rrata锛夌殑婧愪唬鐮佹敞閲娿€傜‖浠惰緭鍑烘祦鎺у湪鎵€鏈夊浐浠剁増鏈腑鍧囧彲宸ヤ綔銆?

  .. [^2^]
         瀵硅緭鍏ワ紙CTS銆丏SR銆丆D銆丷I锛夌殑鏌ヨ鏄剧ず鏈€鍚庝竴娆℃姤鍛婄殑鐘舵€併€傚杈撳嚭
         锛圖TR銆丷TS锛夌殑鏌ヨ鏄剧ず鏈€鍚庝竴娆¤姹傜殑鐘舵€侊紝鍙兘涓嶅弽鏄犵敱鑷姩纭欢娴佹帶
         璁剧疆鐨勫綋鍓嶇姸鎬併€?

  TO DO 鍒楄〃锛?
    - 娣诲姞鐪熸鐨勮皟鍒惰В璋冨櫒鎺у埗绾挎煡璇㈣兘鍔涖€傚綋鍓嶈窡韪腑鏂紙interrupt锛夋姤鍛婄殑鐘舵€?
      鍜岃姹傜殑鐘舵€併€?
    - 娣诲姞鍚戝簲鐢ㄧ▼搴忔姤鍛?UART 閿欒鎯呭喌鐨勯敊璇姤鍛娿€?
    - 娣诲姞瀵?flush ioctl 鐨勬敮鎸併€?
    - 娣诲姞鎵€鏈夊叾瀹冪己澶辩殑鍐呭 :)

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?William Greathouse锛歸greathouse@smva.com


### Empeg empeg-car Mark I/II 椹卞姩


  杩欐槸涓€涓疄楠屾€ч┍鍔紝鐢ㄤ簬涓?Empeg empeg-car mp3 鎾斁鍣ㄧ殑瀹㈡埛绔悓姝ュ伐鍏锋彁渚涜繛鎺?
  鏀寔銆?

  鎻愮ず锛?
    - 涓嶈蹇樿涓?ttyUSB{0,1,2,...} 鍒涘缓璁惧鑺傜偣
    - modprobe empeg锛坢odprobe 鏄綘鐨勫ソ甯墜锛?
    - emptool --usb /dev/ttyUSB0锛堟垨浣犱负璁惧鑺傜偣璧风殑浠讳綍鍚嶅瓧锛?

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Gary Brubaker锛歺avyer@ix.netcom.com


### MCT USB 鍗曠鍙ｄ覆鍙ｉ€傞厤鍣?U232


  璇ラ┍鍔ㄩ€傜敤浜庢潵鑷?Magic Control Technology Corp. 鐨?MCT USB-RS232 杞崲鍣?
  锛?5 閽堬紝鍨嬪彿 U232-P25锛夛紙涔熸湁 9 閽堝瀷鍙?U232-P9锛夈€傚叧浜庤璁惧鐨勬洿澶氫俊鎭彲鍦?
  鍒堕€犲晢缃戠珯鎵惧埌锛歨ttp://www.mct.com.tw銆?

  璇ラ┍鍔ㄥぇ浣撳彲宸ヤ綔锛屼絾浠嶉渶瑕佹洿澶氭祴璇曘€傚畠娲剧敓鑷?Belkin USB 涓插彛閫傞厤鍣?F5U103 椹卞姩锛?
  鍏?TODO 鍒楄〃瀵瑰畠涔熸湁鏁堛€?

  璇ラ┍鍔ㄤ篃宸插彂鐜板彲鐢ㄤ簬鍏锋湁鐩稿悓 Vendor ID 浣嗕笉鍚?Product ID 鐨勫叾瀹冧骇鍝併€係itecom 鐨?
  U232-P25 涓插彛杞崲鍣ㄤ娇鐢?Product ID 0x230 鍜?Vendor ID 0x711锛屽苟鍙笌璇ラ┍鍔ㄩ厤鍚?
  宸ヤ綔銆傛澶栵紝D-Link 鐨?DU-H3SP USB BAY 涔熷彲涓庤椹卞姩閰嶅悎宸ヤ綔銆?

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Wolfgang Grandegger锛歸olfgang@ces.ch


### Inside Out Networks Edgeport 椹卞姩


  璇ラ┍鍔ㄦ敮鎸?Inside Out Networks 鍒堕€犵殑鎵€鏈夎澶囷紝鍏蜂綋涓轰互涓嬪瀷鍙凤細

       - Edgeport/4
       - Rapidport/4
       - Edgeport/4t
       - Edgeport/2
       - Edgeport/4i
       - Edgeport/2i
       - Edgeport/421
       - Edgeport/21
       - Edgeport/8
       - Edgeport/8 Dual
       - Edgeport/2D8
       - Edgeport/4D8
       - Edgeport/8i
       - Edgeport/2 DIN
       - Edgeport/4 DIN
       - Edgeport/16 Dual

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Greg Kroah-Hartman锛歡reg@kroah.com


### REINER SCT cyberJack pinpad/e-com USB 鑺墖鍗¤鍗″櫒


  闈㈠悜 ISO 7816 鍏煎鐨勬帴瑙﹀紡鑺墖鍗★紙contactbased chipcard锛夌殑鎺ュ彛锛屼緥濡?GSM SIM銆?

  褰撳墠鐘舵€侊細

    杩欐槸璇?USB 璇诲崱鍣ㄩ┍鍔ㄧ殑 kernel 閮ㄥ垎銆備篃鎻愪緵浜嗕竴涓敤浜?CT-API 椹卞姩鐨勭敤鎴烽儴鍒?
    锛坲ser part锛夈€備笅杞界珯鐐瑰緟瀹氾紙TBA锛夈€傜洰鍓嶏紝浣犲彲浠ュ悜缁存姢鑰咃紙linux-usb@sii.li锛?
    绱㈠彇銆?

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?linux-usb@sii.li


### Prolific PL2303 椹卞姩


  璇ラ┍鍔ㄦ敮鎸佷换浣曞唴鍚?Prolific 鐨?PL2303 鑺墖鐨勮澶囥€傝繖鍖呮嫭澶ч噺鍗曠鍙?USB 杞覆鍙?
  杞崲鍣ㄣ€佽秴杩?70% 鐨?USB GPS 璁惧锛?010 骞达級锛屼互鍙婁竴浜?USB UPS銆傛潵鑷?Aten
  锛圲C-232锛夊拰 IO-Data 鐨勮澶囷紝浠ュ強 DCU-11 鎵嬫満绾跨紗锛岄兘鍙笌璇ラ┍鍔ㄩ厤鍚堝伐浣溿€?

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Greg Kroah-Hartman锛歡reg@kroah.com


### KL5KUSB105 鑺墖缁?/ PalmConnect USB 鍗曠鍙ｉ€傞厤鍣?


褰撳墠鐘舵€侊細

  璇ラ┍鍔ㄦ槸閫氳繃瑙傚療 Palm 鍦?Windows 涓嬬殑椹卞姩鎵€鍋氱殑 usb 鎬荤嚎浜嬪姟锛坆us transaction锛?
  鎷煎噾鑰屾垚鐨勶紝鍥犳澶ч噺鍔熻兘浠嶇劧缂哄け銆傚€煎緱娉ㄦ剰鐨勬槸锛屼覆鍙?ioctl 鏈夋椂鏄吉閫犵殑鎴栧皻鏈?
  瀹炵幇銆備笉杩囧鏌ヨ DSR 鍜?CTS 绾胯矾鐘舵€佺殑鏀寔宸插疄鐜帮紙灏界瀹炵幇寰楀苟涓嶄紭闆咃級锛屽洜姝?
  浣犲父鐢ㄧ殑 autopilot(1) 鍜?pilot-manager -daemon 璋冪敤鍙互宸ヤ綔銆傛敮鎸佹渶楂?115200 鐨?
  娉㈢壒鐜囷紝浣嗕笉鏀寔鎻℃墜锛堣蒋浠舵垨纭欢锛夛紝杩欏氨鏄负浠€涔堝湪瑙ｅ喅姝ら棶棰樹箣鍓嶏紝瀵逛簬澶т紶杈撻噺
  鏄庢櫤鐨勫仛娉曟槸闄嶄綆鎵€鐢ㄩ€熺巼銆?

  鍏充簬璇ラ┍鍔ㄧ殑鏈€鏂颁俊鎭紝璇疯 http://www.uuhaus.de/linux/palmconnect.html銆?

### Winchiphead CH341 椹卞姩


  璇ラ┍鍔ㄩ€傜敤浜?Winchiphead CH341 USB-RS232 杞崲鍣ㄣ€傝鑺墖杩樺疄鐜颁簡 IEEE 1284 骞跺彛銆?
  I2C 鍜?SPI锛屼絾椹卞姩骞朵笉鏀寔銆傚崗璁槸浠?Windows 椹卞姩鐨勮涓哄垎鏋愬緱鍑虹殑锛岀洰鍓嶆病鏈?
  鍙敤鐨勬暟鎹墜鍐岋紙datasheet锛夈€?

  鍒堕€犲晢缃戠珯锛歨ttp://www.winchiphead.com/銆?

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?frank@kingswood-consulting.co.uk銆?

### Moschip MCS7720銆丮CS7715 椹卞姩


  杩欎簺鑺墖鍑虹幇鍦ㄥ瀹跺埗閫犲晢锛堝 Syba 鍜?Cables Unlimited锛夐攢鍞殑璁惧涓€傚彲鑳借繕鏈?
  鍏跺畠鍘傚晢銆?720 鎻愪緵涓や釜涓插彛锛?715 鎻愪緵涓€涓覆鍙ｅ拰涓€涓爣鍑?PC 骞跺彛銆?715 骞跺彛鐨?
  鏀寔鐢变竴涓崟鐙殑閫夐」鍚敤锛岄櫎闈炲厛鍦ㄨ澶囬┍鍔紙Device Drivers锛夐厤缃彍鍗曠殑椤跺眰鍚敤
  骞跺彛鏀寔锛屽惁鍒欒閫夐」涓嶄細鍑虹幇銆傜洰鍓嶅苟鍙ｄ粎鏀寔鍏煎妯″紡锛坈ompatibility mode锛?
  锛堟棤 ECP/EPP锛夈€?

  TODO锛?
    - 涓哄苟鍙ｅ疄鐜?ECP/EPP 妯″紡銆?
    - 楂樹簬 115200 鐨勬尝鐗圭巼鐩墠鏈夐棶棰樸€?
    - 鍩轰簬 Moschip MCS7703 鐨勫崟涓插彛璁惧锛屽彧闇€鍦?usb_device_id 琛ㄤ腑绠€鍗曟坊鍔犱竴椤癸紝
      灏卞彲鑳戒笌姝ら┍鍔ㄩ厤鍚堝伐浣溿€傛垜娌℃湁杩欐牱鐨勮澶囷紝鍥犳鏃犳硶纭畾銆?

### 閫氱敤锛圙eneric锛変覆鍙ｉ┍鍔?


  濡傛灉浣犵殑璁惧涓嶆槸涓婇潰鍒楀嚭鐨勮澶囷紝涔熶笉涓庝笂杩板瀷鍙峰吋瀹癸紝浣犲彲浠ュ皾璇?鈥済eneric鈥?
  鎺ュ彛銆傝鎺ュ彛涓嶆彁渚涗换浣曞彂閫佺粰璁惧鐨勬帶鍒舵秷鎭紙control message锛夛紝涔熶笉鏀寔浠讳綍
  褰㈠紡鐨勮澶囨祦鎺с€備綘鐨勮澶囧彧闇€鑷冲皯鍏锋湁涓€涓壒閲忚緭鍏ワ紙bulk in锛夌鐐癸紝鎴栦竴涓壒閲忚緭鍑?
  锛坆ulk out锛夌鐐广€?

```
	echo <vid> <pid> >/sys/bus/usb-serial/drivers/generic/new_id

  鍏朵腑 <vid> 鍜?<pid> 鏇挎崲涓轰綘璁惧鐨勫巶鍟?id 鍜屼骇鍝?id 鐨勫崄鍏繘鍒惰〃绀恒€?
  濡傛灉椹卞姩缂栬瘧涓烘ā鍧楋紝浣犱篃鍙互鍦ㄥ姞杞芥ā鍧楁椂鎻愪緵涓€涓?id::

	insmod usbserial vendor=0x#### product=0x####

  璇ラ┍鍔ㄥ凡鎴愬姛鐢ㄤ簬杩炴帴鍒?NetChip USB 寮€鍙戞澘锛屾彁渚涗簡涓€绉嶆棤闇€缂栧啓鑷畾涔夐┍鍔ㄥ嵆鍙?
  寮€鍙?USB 鍥轰欢鐨勬柟寮忋€?

  鍏充簬姝ら┍鍔ㄧ殑浠讳綍闂鎴栨晠闅滐紝璇疯仈绯?Greg Kroah-Hartman锛歡reg@kroah.com


```

## 鑱旂郴鏂瑰紡


  濡傛灉浠讳綍浜哄湪浣跨敤涓婅堪鎸囧畾浜у搧涓殑椹卞姩鏃堕亣鍒伴棶棰橈紝璇疯仈绯讳笂闈㈠垪鍑虹殑鐗瑰畾椹卞姩鐨?
  浣滆€咃紝鎴栧姞鍏?Linux-USB 閭欢鍒楄〃锛堝叧浜庡姞鍏ラ偖浠跺垪琛ㄧ殑淇℃伅锛屼互鍙婂叾鍙悳绱㈠綊妗ｇ殑
  閾炬帴锛岃 http://www.linux-usb.org/ 锛?


Greg Kroah-Hartman
greg@kroah.com
