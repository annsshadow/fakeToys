
## Linux-USB 涓绘満绔?API


## Linux 涓婄殑 USB 绠€浠?

閫氱敤涓茶鎬荤嚎锛圲SB锛夌敤浜庡皢涓绘満锛堜緥濡?PC 鎴栧伐浣滅珯锛夎繛鎺ュ埌鑻ュ共澶栧洿璁惧銆俇SB
浣跨敤鏍戝舰缁撴瀯锛屼互涓绘満涓烘牴锛堢郴缁熺殑涓昏澶囷級锛屼互闆嗙嚎鍣紙hub锛変负鍐呴儴鑺傜偣锛屼互
澶栧洿璁惧涓哄彾瀛愶紙浠庤澶囷級銆傜幇浠?PC 鏀寔鑻ュ共杩欐牱鐨?USB 璁惧鏍戯紝閫氬父鏈夊嚑涓?USB 3.0锛? GBit/s锛夋垨 USB 3.1锛?0 GBit/s锛夋€荤嚎锛屼互鍙婁竴浜涢仐鐣欑殑 USB 2.0
锛?80 MBit/s锛夋€荤嚎浠ラ槻涓囦竴銆?
杩欑涓?浠庨潪瀵圭О鎬ф槸鍑轰簬鑻ュ共鍘熷洜鑰岃璁＄殑锛屽叾涓竴涓槸鏄撶敤鎬с€傚湪鐗╃悊涓婁笉鍙兘
寮勯敊涓婃父鍜屼笅娓革紝鎴栬€咃紙鍦?type C 鎻掑ご鐨勬儏鍐典笅锛夎繖鏃犲叧绱ц锛堟垨鑰呭畠浠唴缃簬
澶栧洿璁惧涓級銆傛澶栵紝涓绘満杞欢涓嶉渶瑕佸鐞嗗垎甯冨紡鑷姩閰嶇疆锛屽洜涓洪鍏堟寚瀹氱殑涓昏妭鐐?绠＄悊鐫€杩欎竴鍒囥€?
鍐呮牳寮€鍙戣€呭湪 2.2 鍐呮牳绯诲垪鏃╂湡灏变负 Linux 娣诲姞浜?USB 鏀寔锛屽苟鑷涓嶆柇寮€鍙戝畠銆?闄や簡瀵规瘡涓€浠ｆ柊 USB 鐨勬敮鎸佸锛屽悇绉嶄富鏈烘帶鍒跺櫒涔熻幏寰椾簡鏀寔锛屾柊澧炰簡鐢ㄤ簬澶栧洿
璁惧鐨勯┍鍔紝骞跺紩鍏ヤ簡鐢ㄤ簬寤惰繜搴﹂噺鐨勯珮绾х壒鎬у拰鏀硅繘鐨勭數婧愮鐞嗐€?
Linux 鏃㈠彲浠ュ湪 USB 璁惧鍐呴儴杩愯锛屼篃鍙互鍦ㄦ帶鍒惰繖浜涜澶囩殑涓绘満涓婅繍琛屻€備絾鏄湪
閭ｄ簺澶栧洿璁惧鍐呴儴杩愯鐨?USB 璁惧椹卞姩鍋氱殑浜嬫儏涓庡湪涓绘満鍐呴儴杩愯鐨勯偅浜涗笉鍚岋紝鍥犳
瀹冧滑琚祴浜堜簡涓€涓笉鍚岀殑鍚嶇О锛?*gadget 椹卞姩**锛堝皬宸ュ叿椹卞姩锛夈€傛湰鏂囨。涓嶆兜鐩?gadget 椹卞姩銆?
## USB 涓绘満绔?API 妯″瀷


鐢ㄤ簬 USB 璁惧鐨勪富鏈虹椹卞姩涓?"usbcore" API 閫氫俊銆傛湁涓や釜銆備竴涓槸闈㈠悜
**閫氱敤** 椹卞姩锛堥€氳繃椹卞姩妗嗘灦鏆撮湶锛夌殑锛屽彟涓€涓槸闈㈠悜浣滀负 *鏍稿績涓€閮ㄥ垎* 鐨?椹卞姩鐨勩€傛绫绘牳蹇冮┍鍔ㄥ寘鎷?**hub** 椹卞姩锛堢鐞?USB 璁惧鐨勬爲锛夊拰鍑犵涓嶅悓鐨?*涓绘満鎺у埗鍣ㄩ┍鍔?锛屽畠浠帶鍒跺悇鑷殑鎬荤嚎銆?
USB 椹卞姩鎵€鐪嬪埌鐨勮澶囨ā鍨嬬浉瀵瑰鏉傘€?
- USB 鏀寔鍥涚鏁版嵁浼犺緭锛堟帶鍒躲€佹壒閲忋€佷腑鏂拰绛夋椂锛夈€傚叾涓袱绉嶏紙鎺у埗鍜屾壒閲忥級
  鍦ㄥ甫瀹藉彲鐢ㄦ椂浣跨敤甯﹀锛岃€屽彟澶栦袱绉嶏紙涓柇鍜岀瓑鏃讹級琚皟搴︿互鎻愪緵鏈変繚璇佺殑甯﹀銆?
- 璁惧鎻忚堪妯″瀷鍖呮嫭姣忎釜璁惧涓€涓垨澶氫釜鈥滈厤缃€濓紙configuration锛夛紝涓€娆″彧鏈?  鍏朵腑涓€涓浜庢椿鍔ㄧ姸鎬併€傝澶囧簲璇ヨ兘澶熶互浣庝簬鍏舵渶楂橀€熷害鐨勯€熷害杩愯锛屽苟鍙互
  鎻愪緵涓€涓?BOS 鎻忚堪绗︽潵鏄剧ず瀹冧滑浠嶅畬鍏ㄥ彲杩愯鐨勬渶浣庨€熷害銆?
- 浠?USB 3.0 璧凤紝閰嶇疆鍏锋湁涓€涓垨澶氫釜鈥滃姛鑳解€濓紙function锛夛紝瀹冧滑鎻愪緵閫氱敤鍔熻兘
  骞朵负浜嗙數婧愮鐞嗙殑鐩殑琚粍鍚堝湪涓€璧枫€?
- 閰嶇疆鎴栧姛鑳藉叿鏈変竴涓垨澶氫釜鈥滄帴鍙ｂ€濓紙interface锛夛紝姣忎釜鎺ュ彛鍙兘鍏锋湁鈥滃鐢?  璁剧疆鈥濓紙alternate setting锛夈€傛帴鍙ｅ彲鑳界敱 USB 鈥滅被鈥濓紙Class锛夎鑼冩爣鍑嗗寲锛?  涔熷彲鑳芥槸鐗瑰畾浜庢煇涓緵搴斿晢鎴栬澶囩殑銆?
  USB 璁惧椹卞姩瀹為檯涓婄粦瀹氬埌鎺ュ彛锛岃€岄潪璁惧銆傚彲浠ユ妸瀹冧滑鐪嬩綔鈥滄帴鍙ｉ┍鍔ㄢ€濓紝
  灏界浣犲彲鑳界湅涓嶅埌璁稿杩欑鍖哄垎寰堥噸瑕佺殑璁惧銆?澶у鏁?USB 璁惧寰堢畝鍗曪紝鍙湁
  涓€涓姛鑳姐€佷竴涓厤缃€佷竴涓帴鍙ｅ拰涓€涓鐢ㄨ缃€?

- 鎺ュ彛鍏锋湁涓€涓垨澶氫釜鈥滅鐐光€濓紙endpoint锛夛紝姣忎釜绔偣鏀寔涓€绉嶇被鍨嬪拰鏂瑰悜鐨勬暟鎹?  浼犺緭锛屼緥濡傗€滄壒閲忚緭鍑衡€濓紙bulk out锛夋垨鈥滀腑鏂緭鍏モ€濓紙interrupt in锛夈€傛暣涓?  閰嶇疆鍦ㄦ瘡涓柟鍚戜笂鏈€澶氬彲鏈夊崄鍏釜绔偣锛屽湪鍏ㄩ儴鎺ュ彛涔嬮棿鎸夐渶鍒嗛厤銆?
- USB 涓婄殑鏁版嵁浼犺緭鏄垎缁勭殑锛坧acketized锛夛紱姣忎釜绔偣鏈変竴涓渶澶у寘澶у皬銆傞┍鍔?  閫氬父蹇呴』鎰忚瘑鍒颁竴浜涚害瀹氾紝渚嬪浣跨敤鈥滅煭鈥濓紙鍖呮嫭闆堕暱搴︼級鍖呮潵鏍囧織鎵归噺浼犺緭鐨?  缁撴潫銆?
- Linux USB API 鏀寔鎺у埗娑堟伅鍜屾壒閲忔秷鎭殑鍚屾璋冪敤銆傚畠涔熸敮鎸佹墍鏈夌被鍨嬫暟鎹紶杈?  鐨勫紓姝ヨ皟鐢紝浣跨敤绉颁负 鈥淯RB鈥濓紙USB Request Block锛孶SB 璇锋眰鍧楋級鐨勮姹傜粨鏋勩€?
鐩稿簲鍦帮紝鏆撮湶缁欒澶囬┍鍔ㄧ殑 USB 鏍稿績 API 娑电洊浜嗙浉褰撳ぇ鐨勮寖鍥淬€備綘鍙兘闇€瑕佹煡闃?USB
3.0 瑙勮寖锛堝彲浠?www.usb.org 鍏嶈垂鍦ㄧ嚎鑾峰彇锛変互鍙婄被鎴栬澶囪鑼冦€?
鍞竴鐪熸鎺ヨЕ纭欢锛堣鍙?鍐欏叆瀵勫瓨鍣ㄣ€佸鐞?IRQ 绛夛級鐨勪富鏈虹椹卞姩鏄?HCD锛堜富鏈?鎺у埗鍣ㄩ┍鍔級銆傜悊璁轰笂锛屾墍鏈?HCD 閮介€氳繃鐩稿悓鐨?API 鎻愪緵鐩稿悓鐨勫姛鑳姐€傚湪瀹炶返涓紝
杩欐鍙樺緱瓒婃潵瓒婄湡瀹烇紝浣嗕粛瀛樺湪宸紓锛屽挨鍏舵槸鍦ㄨ緝涓嶅父瑙佹帶鍒跺櫒涓婄殑鏁呴殰澶勭悊鏂归潰銆?涓嶅悓鐨勬帶鍒跺櫒涓嶄竴瀹氭姤鍛婃晠闅滅殑鐩稿悓鏂归潰锛屽苟涓斾粠鏁呴殰锛堝寘鎷蒋浠跺紩璧风殑鏁呴殰锛屼緥濡?鍙栨秷涓€涓?URB 鐨勯摼鎺ワ級涓仮澶嶅皻鏈畬鍏ㄤ竴鑷淬€傝澶囬┍鍔ㄤ綔鑰呭簲璇ョ壒鎰忓姣忎釜涓嶅悓鐨?涓绘満鎺у埗鍣ㄩ┍鍔ㄨ繘琛屾柇寮€娴嬭瘯锛堝湪璁惧娲昏穬鏃讹級锛屼互纭繚椹卞姩鑷韩娌℃湁 bug锛屽苟纭繚
瀹冧滑涓嶄緷璧栦簬鏌愪簺 HCD 鐗瑰畾鐨勮涓恒€?

## USB 鏍囧噯绫诲瀷


鍦?`include/uapi/linux/usb/ch9.h` 涓紝浣犲彲浠ユ壘鍒?USB 瑙勮寖绗?9 绔犱腑瀹氫箟鐨?USB 鏁版嵁绫诲瀷銆傝繖浜涙暟鎹被鍨嬪湪 USB 鍚勫浠ュ強鍖呮嫭杩欎釜涓绘満绔?API銆乬adget API銆乽sb
瀛楃璁惧鍜?debugfs 鎺ュ彛鍦ㄥ唴鐨?API 涓娇鐢ㄣ€傝鏂囦欢鏈韩琚?`include/linux/usb/ch9.h`
鍖呭惈锛屽悗鑰呰繕鍖呭惈涓€浜涚敤浜庡鐞嗚繖浜涙暟鎹被鍨嬬殑宸ュ叿渚嬬▼鐨勫０鏄庯紱鍏跺疄鐜颁綅浜?`drivers/usb/common/common.c`銆?
   :export:

姝ゅ锛屼竴浜涘鍒涘缓璋冭瘯杈撳嚭鏈夌敤鐨勫嚱鏁板畾涔夊湪 `drivers/usb/common/debug.c` 涓€?

## 涓绘満绔暟鎹被鍨嬩笌瀹?

涓绘満绔?API 鍚戦┍鍔ㄦ毚闇蹭簡鑻ュ共灞傦紝鍏朵腑涓€浜涙瘮鍏朵粬鏇村繀瑕併€傝繖浜涙敮鎸佷富鏈虹椹卞姩鍜?璁惧鐨勭敓鍛藉懆鏈熸ā鍨嬶紝骞舵敮鎸侀€氳繃 usbcore 灏嗙紦鍐插尯浼犻€掔粰涓鸿澶囬┍鍔ㄦ墽琛?I/O 鐨?鏌愪釜 HCD銆?
   :internal:

## USB 鏍稿績 API


USB API 涓湁涓ょ鍩烘湰鐨?I/O 妯″瀷銆傛渶鍩虹鐨勯偅涓槸寮傛鐨勶細椹卞姩浠?URB 鐨勫舰寮?鎻愪氦璇锋眰锛孶RB 鐨勫畬鎴愬洖璋冨鐞嗕笅涓€姝ャ€傛墍鏈?USB 浼犺緭绫诲瀷閮芥敮鎸佽妯″瀷锛屽敖绠?鎺у埗 URB锛堟€绘槸鏈?setup 鍜?status 闃舵锛屼絾鍙兘娌℃湁鏁版嵁闃舵锛夊拰绛夋椂 URB锛堝厑璁?澶ф暟鎹寘骞跺寘鍚瘡鍖呮晠闅滄姤鍛婏級鏈夌壒娈婃儏鍐点€傛瀯寤轰簬鍏朵笂鐨勬槸鍚屾 API 鏀寔锛屽嵆
椹卞姩璋冪敤涓€涓緥绋嬶紝璇ヤ緥绋嬪垎閰嶄竴涓垨澶氫釜 URB銆佹彁浜ゅ畠浠紝骞剁瓑寰呭畠浠畬鎴愩€傛湁
鐢ㄤ簬鍗曠紦鍐插尯鎺у埗鍜屾壒閲忎紶杈擄紙鍦ㄦ煇浜涢┍鍔ㄦ柇寮€鍦烘櫙涓敤璧锋潵杈冨埆鎵級鐨勫悓姝ュ寘瑁咃紝
浠ュ強鐢ㄤ簬鍩轰簬 scatterlist 鐨勬祦 I/O锛堟壒閲忔垨涓柇锛夌殑鍚屾鍖呰銆?
USB 椹卞姩闇€瑕佹彁渚涘彲鐢ㄤ簬 DMA 鐨勭紦鍐插尯锛屽敖绠″畠浠笉涓€瀹氶渶瑕佽嚜宸辨彁渚?DMA 鏄犲皠銆?鏈変竴浜涘湪鍒嗛厤 DMA 缂撳啿鍖烘椂浣跨敤鐨?API锛屽彲浠ラ伩鍏嶅湪鏌愪簺绯荤粺涓婁娇鐢ㄥ弽寮圭紦鍐插尯
锛坆ounce buffer锛夈€傚湪鏌愪簺鎯呭喌涓嬶紝椹卞姩鍙兘鑳藉渚濊禆 64 浣?DMA 鏉ユ秷闄ゅ彟涓€绉嶅弽寮?缂撳啿鍖恒€?
   :export:

   :export:

   :export:

   :export:

   :export:

   :export:

   :export:

## 涓绘満鎺у埗鍣?API


杩欎簺 API 浠呬緵涓绘満鎺у埗鍣ㄩ┍鍔ㄤ娇鐢紝鍏朵腑澶у鏁板疄鐜颁簡鏍囧噯鐨勫瘎瀛樺櫒鎺ュ彛锛屼緥濡?XHCI銆丒HCI銆丱HCI 鎴?UHCI銆俇HCI 鏄渶鏃╃殑鎺ュ彛涔嬩竴锛岀敱 Intel 璁捐骞惰 VIA 浣跨敤锛?瀹冨湪纭欢鏂归潰鍋氬緱涓嶅銆侽HCI 璁捐寰楁洿鏅氾紝璁╃‖浠跺仛鏇村宸ヤ綔锛堟洿澶х殑浼犺緭銆佽窡韪?鍗忚鐘舵€佺瓑锛夈€侲HCI 鏄即闅?USB 2.0 璁捐鐨勶紱瀹冪殑璁捐鏃㈡湁绫讳技 OHCI 鐨勭壒鎬?锛堢‖浠跺仛鏇村宸ヤ綔锛変篃鏈夌被浼?UHCI 鐨勭壒鎬э紙ISO 鏀寔鐨勬煇浜涢儴鍒嗐€乀D 鍒楄〃澶勭悊锛夈€?XHCI 鏄即闅?USB 3.0 璁捐鐨勩€傚畠缁х画灏嗗姛鑳芥敮鎸佽浆绉诲埌纭欢涓€?
闄や簡鈥滀笁宸ㄥご鈥濅箣澶栬繕鏈夊叾浠栦富鏈烘帶鍒跺櫒锛屽敖绠″ぇ澶氭暟鍩轰簬 PCI 鐨勬帶鍒跺櫒锛堜互鍙婂皯鏁?闈?PCI 鐨勶級浣跨敤杩欎簺鎺ュ彛涔嬩竴銆傚苟闈炴墍鏈変富鏈烘帶鍒跺櫒閮戒娇鐢?DMA锛涙湁浜涗娇鐢?PIO锛岃繕
鏈変竴涓ā鎷熷櫒鍜屼竴涓櫄鎷熶富鏈烘帶鍒跺櫒鐢ㄤ簬閫氳繃缃戣矾浼犺緭 USB銆?
鎵€鏈夎繖浜涙帶鍒跺櫒鐨勯┍鍔ㄩ兘鍙互浣跨敤鐩稿悓鐨勫熀纭€ API銆傜敱浜庡巻鍙插師鍥狅紝瀹冧滑鍒嗕负涓ゅ眰锛?:c:type:`struct usb_bus <usb_bus>` 鏄竴涓浉褰撹杽鐨勪竴灞傦紝鍦?2.2 鍐呮牳涓彉寰?鍙敤锛岃€?`struct usb_hcd <usb_hcd>` 鏄竴涓壒鎬ф洿涓板瘜鐨勫眰锛屽畠璁?HCD 鍏变韩
閫氱敤浠ｇ爜锛屼粠鑰岀缉灏忛┍鍔ㄥぇ灏忓苟鏄捐憲鍑忓皯 hcd 鐗瑰畾鐨勮涓恒€?
   :export:

   :export:

   :internal:

## USB 瀛楃璁惧鑺傜偣


鏈珷浠嬬粛 Linux 瀛楃璁惧鑺傜偣銆備綘鍙兘鍊惧悜浜庨伩鍏嶄负浣犵殑 USB 椹卞姩缂栧啓鏂扮殑鍐呮牳浠ｇ爜銆?鐢ㄦ埛妯″紡璁惧椹卞姩閫氬父琚墦鍖呬负搴旂敤绋嬪簭鎴栧簱锛屽苟鍙兘閫氳繃鍖呰瀹冪殑缂栫▼搴撴潵浣跨敤
瀛楃璁惧銆傛绫诲簱鍖呮嫭锛?
 - `libusb <http://libusb.sourceforge.net>`__锛堢敤浜?C/C++锛夛紝浠ュ強
 - `jUSB <http://jUSB.sourceforge.net>`__锛堢敤浜?Java锛夈€?
鍏充簬瀹冪殑涓€浜涙棫淇℃伅鍙互鍦?USB 鎸囧崡鐨?鈥淯SB Device Filesystem鈥?涓€鑺備腑鐪嬪埌銆俇SB
鎸囧崡鐨勬渶鏂板壇鏈彲鍦?http://www.linux-usb.org/ 鎵惧埌銆?

  - 瀹冧滑杩囧幓鏄€氳繃 **usbfs** 瀹炵幇鐨勶紝浣嗚繖涓嶅睘浜?sysfs 璋冭瘯鎺ュ彛鐨勪竴閮ㄥ垎銆?
   - 杩欎釜鐗瑰畾鐨勬枃妗ｆ槸涓嶅畬鏁寸殑锛屽挨鍏舵槸鍦ㄥ紓姝ユā寮忔柟闈€傝嚜鍐呮牳 2.5.66 璧凤紝浠ｇ爜
     鍜岃繖浠斤紙鏂扮殑锛夋枃妗ｉ渶瑕佷氦鍙夊闃呫€?
### "devtmpfs" 涓湁鍝簺鏂囦欢锛?

浼犵粺涓婃寕杞藉湪 `/dev/bus/usb/`锛寀sbfs 鐨勭壒鎬у寘鎷細

- `/dev/bus/usb/BBB/DDD` 鈥︹€?鏆撮湶姣忎釜璁惧鐨勯厤缃弿杩扮銆佸苟鏀寔涓€绯诲垪鐢ㄤ簬鍙戝嚭
   璁惧璇锋眰锛堝寘鎷璁惧鐨?I/O锛夌殑 ioctl 鐨勯瓟娉曟枃浠躲€傦紙绾补渚涚▼搴忚闂€傦級

姣忎釜鎬荤嚎琚祴浜堜竴涓紪鍙凤紙`BBB`锛夛紝鍩轰簬瀹冭鏋氫妇鐨勬椂闂达紱鍦ㄦ瘡涓€荤嚎鍐咃紝姣忎釜璁惧
琚祴浜堜竴涓被浼肩殑缂栧彿锛坄DDD`锛夈€傞偅浜?`BBB/DDD` 璺緞涓嶆槸鈥滅ǔ瀹氣€濈殑鏍囪瘑绗︼紱
鍗充娇浣犳€绘槸鎶婅澶囨彃鍦ㄥ悓涓€涓泦绾垮櫒绔彛涓婏紝涔熼鏈熷畠浠細鍙樺寲銆?鐢氳嚦涓嶈鎯虫妸瀹冧滑
淇濆瓨鍦ㄥ簲鐢ㄧ▼搴忕殑閰嶇疆鏂囦欢涓€? 鏈夌ǔ瀹氱殑鏍囪瘑绗﹀彲渚涙兂瑕佷娇鐢ㄥ畠浠殑鐢ㄦ埛妯″紡
搴旂敤绋嬪簭浣跨敤銆侶ID 鍜岀綉缁滆澶囨毚闇茶繖浜涚ǔ瀹?ID锛屽洜姝や緥濡備綘鍙互纭畾浣犲憡璇変簡姝ｇ‘鐨?UPS 鍘诲叧闂畠鐨勭浜屼釜鏈嶅姟鍣ㄣ€傝娉ㄦ剰锛屽畠锛堣繕锛夋病鏈夋毚闇查偅浜?ID銆?
### /dev/bus/usb/BBB/DDD


浠ヤ互涓嬪熀鏈柟寮忎箣涓€浣跨敤杩欎簺鏂囦欢锛?
- **鍙互瀵瑰畠浠繘琛岃鍙栵紝** 棣栧厛浜х敓璁惧鎻忚堪绗︼紙18 瀛楄妭锛夛紝鐒跺悗鏄綋鍓嶉厤缃殑
  鎻忚堪绗︺€傛湁鍏宠繖浜涗簩杩涘埗鏁版嵁鏍煎紡鐨勮缁嗕俊鎭紝璇峰弬闃?USB 2.0 瑙勮寖銆備綘灏嗛渶瑕佹妸
  澶ч儴鍒嗗瀛楄妭鍊间粠 little endian 鏍煎紡杞崲鍒颁綘鐨勫師鐢熶富鏈哄瓧鑺傚簭锛屽敖绠¤澶囨弿杩扮
  涓殑灏戞暟瀛楁锛堜袱涓?BCD 缂栫爜瀛楁锛屼互鍙婁緵搴斿晢鍜屼骇鍝?ID锛夊凡缁忎负浣犲仛浜嗗瓧鑺備氦鎹€?  娉ㄦ剰閰嶇疆鎻忚堪绗﹀寘鍚帴鍙ｃ€佸鐢ㄨ缃€佺鐐圭殑鎻忚堪绗︼紝浠ュ強鍙兘棰濆鐨勭被鎻忚堪绗︺€?
- **鎵ц USB 鎿嶄綔** 浣跨敤 **ioctl()** 璇锋眰鏉ュ彂鍑虹鐐?I/O 璇锋眰锛堝悓姝ユ垨寮傛锛夋垨
  绠＄悊璁惧銆傝繖浜涜姹傞渶瑕?`CAP_SYS_RAWIO` 鑳藉姏锛屼互鍙婃枃浠剁郴缁熻闂潈闄愩€備竴娆″彧鑳?  鍦ㄨ繖浜涜澶囨枃浠朵箣涓€涓婂彂鍑轰竴涓?ioctl 璇锋眰銆傝繖鎰忓懗鐫€濡傛灉浣犳浠庝竴涓嚎绋嬪悓姝ヨ鍙?  涓€涓鐐癸紝鍦ㄨ鍙栧畬鎴愪箣鍓嶄綘灏嗘棤娉曚粠鍙︿竴涓嚎绋嬪啓鍏ヤ笉鍚岀殑绔偣銆傝繖瀵?**鍗婂弻宸?*
  锛坔alf duplex锛夊崗璁湁鏁堬紝浣嗗湪鍏朵粬鎯呭喌涓嬩綘浼氫娇鐢ㄥ紓姝?I/O 璇锋眰銆?
姣忎釜杩炴帴鐨?USB 璁惧鏈変竴涓枃浠躲€俙BBB` 琛ㄧず鎬荤嚎缂栧彿銆俙DDD` 琛ㄧず璇ユ€荤嚎涓婄殑
璁惧鍦板潃銆傝繖涓や釜鏁板瓧閮芥槸椤哄簭鍒嗛厤鐨勶紝骞朵笖鍙互琚噸鐢紝鍥犳浣犱笉鑳戒緷璧栧畠浠潵
绋冲畾鍦拌闂澶囥€備緥濡傦紝璁惧鍦ㄤ粛杩炴帴鏃堕噸鏂版灇涓撅紙涔熻鏈変汉纰板姩浜嗗畠浠殑鐢垫簮銆侀泦绾垮櫒
鎴?USB 鐢电紗锛夋槸鐩稿甯歌鐨勶紝鍥犳涓€涓澶囧湪浣犻娆¤繛鎺ユ椂鍙兘鏄?`002/027`锛岃€?绋嶅悗鍙樻垚 `002/048`銆?
杩欎簺鏂囦欢鍙互浣滀负浜岃繘鍒舵暟鎹鍙栥€備簩杩涘埗鏁版嵁棣栧厛鐢辫澶囨弿杩扮缁勬垚锛岀劧鍚庢槸璇?璁惧姣忎釜閰嶇疆鐨勬弿杩扮銆傝澶囨弿杩扮涓殑澶氬瓧鑺傚瓧娈电敱鍐呮牳杞崲涓哄涓诲瓧鑺傚簭銆?閰嶇疆鎻忚堪绗︽槸鎬荤嚎瀛楄妭搴忥紙bus endian锛夋牸寮忥紒閰嶇疆鎻忚堪绗﹀郊姝ょ浉璺?wTotalLength
瀛楄妭銆傚鏋滀竴涓澶囪繑鍥炵殑鐨勯厤缃弿杩扮鏁版嵁灏戜簬 wTotalLength 鎸囩ず鐨勶紝鏂囦欢涓?缂哄け瀛楄妭澶勫皢鍑虹幇涓€涓┖娲炪€傛淇℃伅涔熶互鏂囨湰褰㈠紡鏄剧ず鍦?`/sys/kernel/debug/usb/devices`
鏂囦欢涓紝绋嶅悗鎻忚堪銆?
杩欎簺鏂囦欢涔熷彲浠ョ敤浜庝负 USB 璁惧缂栧啓鐢ㄦ埛绾ч┍鍔ㄣ€備綘浼氫互璇?鍐欐柟寮忔墦寮€
`/dev/bus/usb/BBB/DDD` 鏂囦欢锛岃鍙栧畠鐨勬弿杩扮浠ョ‘璁ゅ畠鏄綘鏈熸湜鐨勮澶囷紝鐒跺悗
浣跨敤涓€涓?ioctl 璋冪敤缁戝畾鍒颁竴涓紙鎴栧彲鑳藉嚑涓級鎺ュ彛銆備綘浼氬悜璁惧鍙戝嚭鏇村 ioctl
浠ヤ娇鐢ㄦ帶鍒躲€佹壒閲忔垨鍏朵粬绉嶇被鐨?USB 浼犺緭涓庝箣閫氫俊銆傝繖浜?IOCTL 鍒楀湪
`<linux/usbdevice_fs.h>` 鏂囦欢涓紝鍦ㄦ挵鍐欐湰鏂囨椂锛屾簮浠ｇ爜锛坄linux/drivers/usb/core/devio.c`锛?鏄浣曢€氳繃杩欎簺鏂囦欢璁块棶璁惧鐨勪富瑕佸弬鑰冦€?
娉ㄦ剰锛岀敱浜庨粯璁ゆ儏鍐典笅杩欎簺 `BBB/DDD` 鏂囦欢鍙兘鐢?root 鍐欏叆锛屽彧鏈?root 鍙互缂栧啓
姝ょ被鐢ㄦ埛妯″紡椹卞姩銆備綘鍙互閫氳繃浣跨敤 `chmod` 鏈夐€夋嫨鍦版巿浜堝叾浠栫敤鎴疯/鍐欐潈闄愩€?姝ゅ锛屽儚 `devmode=0666` 杩欐牱鐨?usbfs 鎸傝浇閫夐」鍙兘鏈夊府鍔┿€?

### 鐢ㄦ埛妯″紡椹卞姩鐨勭敓鍛藉懆鏈?

杩欐牱涓€涓┍鍔ㄩ鍏堥渶瑕佷负瀹冪煡閬撳浣曞鐞嗙殑璁惧鎵惧埌涓€涓澶囨枃浠躲€備篃璁稿畠鏄洜涓?`/sbin/hotplug` 浜嬩欢澶勭悊浠ｇ悊閫夋嫨浜嗚椹卞姩鏉ュ鐞嗘柊璁惧鑰岃鍛婄煡鐨勩€傛垨鑰呭畠鍙兘鏄?涓€涓壂鎻忔墍鏈?`/dev/bus/usb` 璁惧鏂囦欢骞跺拷鐣ュぇ澶氭暟璁惧鐨勫簲鐢ㄧ▼搴忋€傛棤璁哄摢绉?鎯呭喌锛屽畠閮藉簲璇?`read()` 璁惧鏂囦欢涓殑鎵€鏈夋弿杩扮锛屽苟灏嗗畠浠笌瀹冪煡閬撳浣曞鐞嗙殑
杩涜鏍稿銆傚畠鍙兘鍙槸鎷掔粷闄ょ壒瀹氫緵搴斿晢鍜屼骇鍝?ID 涔嬪鐨勬墍鏈変笢瑗匡紝鎴栬€呴渶瑕佹洿澶嶆潅鐨?绛栫暐銆?
缁濅笉瑕佸亣璁剧郴缁熶笂涓€娆″彧浼氭湁涓€涓繖鏍风殑璁惧锛佸鏋滀綘鐨勪唬鐮佷笉鑳藉悓鏃跺鐞嗗涓澶囷紝
鑷冲皯瑕佸湪鏈夊浜庝竴涓椂妫€娴嬪嚭鏉ワ紝骞惰浣犵殑鐢ㄦ埛閫夋嫨浣跨敤鍝釜璁惧銆?
涓€鏃︿綘鐨勭敤鎴锋ā寮忛┍鍔ㄧ煡閬撹浣跨敤鍝釜璁惧锛屽畠灏变細浠ヤ袱绉嶉鏍间箣涓€涓庝箣浜や簰銆傜畝鍗曠殑
椋庢牸鏄彧鍙戝嚭鎺у埗璇锋眰锛涙湁浜涜澶囦笉闇€瑕佹瘮杩欐洿澶嶆潅鐨勪氦浜掋€傦紙涓€涓緥瀛愬彲鑳芥槸杞欢
浣跨敤渚涘簲鍟嗙壒瀹氱殑鎺у埗璇锋眰杩涜涓€浜涘垵濮嬪寲鎴栭厤缃换鍔★紝鍏朵綑閮ㄥ垎浣跨敤鍐呮牳椹卞姩銆傦級

鏇村彲鑳界殑鏄紝浣犻渶瑕佷竴涓洿澶嶆潅鐨勯鏍奸┍鍔細涓€涓娇鐢ㄩ潪鎺у埗绔偣銆佽鍙栨垨鍐欏叆鏁版嵁
骞跺０鏄庣嫭鍗犱娇鐢ㄤ竴涓帴鍙ｇ殑椹卞姩銆?*鎵归噺** 浼犺緭鏈€瀹规槗浣跨敤锛屼絾鍙湁瀹冧滑鐨勫厔寮?**涓柇** 浼犺緭鑳戒笌浣庨€熻澶囦竴璧峰伐浣溿€備腑鏂拰 **绛夋椂** 浼犺緭閮芥彁渚涙湇鍚勪繚璇侊紝鍥犱负
瀹冧滑鐨勫甫瀹芥槸棰勭暀鐨勩€傝繖绉嶁€滃懆鏈熸€р€濅紶杈撻€氳繃 usbfs 浣跨敤璧锋潵寰堝埆鎵紝闄ら潪浣犱娇鐢?寮傛璋冪敤銆傜劧鑰岋紝涓柇浼犺緭涔熷彲浠ヤ互鍚屾鐨勨€滀竴娆℃€р€濋鏍间娇鐢ㄣ€?
浣犵殑鐢ㄦ埛妯″紡椹卞姩姘歌繙涓嶉渶瑕佹搷蹇冨湪璁惧鏂紑鏃舵竻鐞嗚姹傜姸鎬侊紝灏界瀹冨簲璇ュ湪涓€寮€濮?鐪嬪埌 ENODEV 閿欒鏃跺敖蹇叧闂畠鎵撳紑鐨勬枃浠舵弿杩扮銆?
### ioctl() 璇锋眰


瑕佷娇鐢ㄨ繖浜?ioctl锛屼綘闇€瑕佸湪浣犵殑浠ｇ爜涓寘鍚互涓嬪ご鏂囦欢锛?
```

    #include <linux/usb.h>
    #include <linux/usbdevice_fs.h>
    #include <asm/byteorder.h>

```
鏍囧噯鐨?USB 璁惧妯″瀷璇锋眰锛屾潵鑷?USB 2.0 瑙勮寖鐨?鈥淐hapter 9鈥濓紝浼氫粠 `<linux/usb/ch9.h>`
澶存枃浠惰嚜鍔ㄥ寘鍚€?
闄ら潪鍙︽湁璇存槑锛岃繖閲屾弿杩扮殑 ioctl 璇锋眰浼氭洿鏂板畠浠墍搴旂敤浜庣殑 usbfs 鏂囦欢鐨勪慨鏀规椂闂?锛堥櫎闈炲畠浠け璐ワ級銆傝繑鍥為浂琛ㄧず鎴愬姛锛涘惁鍒欙紝杩斿洖涓€涓爣鍑?USB 閿欒鐮侊紙杩欎簺鍦?usb-error-codes 涓湁鏂囨。锛夈€?
杩欎簺鏂囦欢涓殑姣忎竴涓兘澶氳矾澶嶇敤浜嗗埌鑻ュ共 I/O 娴佺殑璁块棶锛屾瘡涓鐐逛竴涓€傛瘡涓澶囨湁
涓€涓帶鍒剁鐐癸紙绔偣闆讹級锛屽畠鏀寔鏈夐檺鐨?RPC 椋庢牸璁块棶銆傝澶囩敱 hub_wq锛堝湪鍐呮牳涓級
璁剧疆褰卞搷鍔熻€楀拰鍩烘湰鍔熻兘绛変簨鐗╃殑璁惧绾?**閰嶇疆** 鏉ラ厤缃€傜鐐规槸 USB **鎺ュ彛** 鐨?涓€閮ㄥ垎锛屾帴鍙ｅ彲鑳藉叿鏈夊奖鍝嶅摢浜涚鐐瑰彲鐢ㄧ瓑浜嬬墿鐨?**澶囩敤璁剧疆**锛坅ltsetting锛夈€傝澶?璁惧鍙湁涓€涓厤缃拰涓€涓帴鍙ｏ紝鍥犳瀹冧滑鐨勯┍鍔ㄤ細蹇界暐閰嶇疆鍜屽鐢ㄨ缃€?
#### 绠＄悊/鐘舵€佽姹?

璁稿 usbfs 璇锋眰骞朵笉鐩存帴澶勭悊璁惧 I/O銆傚畠浠富瑕佷笌璁惧绠＄悊鍜岀姸鎬佺浉鍏炽€傝繖浜涢兘鏄?鍚屾璇锋眰銆?
USBDEVFS_CLAIMINTERFACE
    杩欑敤浜庡己鍒?usbfs 澹版槑涓€涓壒瀹氭帴鍙ｏ紝璇ユ帴鍙ｄ箣鍓嶆湭琚?usbfs 鎴栦换浣曞叾浠栧唴鏍?    椹卞姩澹版槑銆俰octl 鍙傛暟鏄竴涓繚瀛樻帴鍙ｇ紪鍙凤紙鏉ヨ嚜鎻忚堪绗︾殑 bInterfaceNumber锛?    鐨勬暣鏁般€?
    娉ㄦ剰锛屽鏋滀綘鐨勯┍鍔ㄥ湪灏濊瘯浣跨敤鏌愪釜鎺ュ彛鐨勪竴涓鐐逛箣鍓嶆病鏈夊０鏄庡畠锛屽苟涓旀病鏈夊叾浠?    椹卞姩缁戝畾鍒板畠锛岄偅涔堣鎺ュ彛浼氳 usbfs 鑷姩澹版槑銆?
    姝ゅ０鏄庝細琚?RELEASEINTERFACE ioctl 閲婃斁锛屾垨鍦ㄥ叧闂枃浠舵弿杩扮鏃堕噴鏀俱€傛枃浠?    淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
USBDEVFS_CONNECTINFO
    璇存槑璁惧鏄惁涓轰綆閫熴€俰octl 鍙傛暟鎸囧悜涓€涓?
```

	struct usbdevfs_connectinfo {
		unsigned int   devnum;
		unsigned char  slow;
	};

    鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
    *浣犳棤娉曞垎杈ㄢ€滈潪鎱㈤€熲€濊澶囨槸浠ラ珮閫燂紙480 MBit/sec锛夎繕鏄叏閫燂紙12 MBit/sec锛?    杩炴帴鐨勩€? 浣犲簲璇ュ凡缁忕煡閬?devnum 鍊硷紝瀹冨氨鏄澶囨枃浠跺悕鐨?DDD 鍊笺€?
```
USBDEVFS_GET_SPEED
    杩斿洖璁惧鐨勯€熷害銆傝閫熷害浣滀负涓€涓暟鍊艰繑鍥烇紝渚濇嵁 enum usb_device_speed銆?
    鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
USBDEVFS_GETDRIVER
    杩斿洖缁戝畾鍒扮粰瀹氭帴鍙ｏ紙涓€涓瓧绗︿覆锛夌殑鍐呮牳椹卞姩鐨勫悕绉般€傚弬鏁版槸涓€涓寚鍚戞缁撴瀯鐨?    鎸囬拡锛岃缁撴瀯涓?
```

	struct usbdevfs_getdriver {
		unsigned int  interface;
		char          driver[USBDEVFS_MAXDRIVERNAME + 1];
	};

    鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
```
USBDEVFS_IOCTL
    灏嗘潵鑷敤鎴风┖闂寸殑璇锋眰鍚戜笅浼犻€掑埌涓€涓凡缁戝畾鐨勫唴鏍搁┍鍔?
```

	struct usbdevfs_ioctl {
		int     ifno;
		int     ioctl_code;
		void    *data;
	};

	/* user mode call looks like this.
	 * 'request' becomes the driver->ioctl() 'code' parameter.
	 * the size of 'param' is encoded in 'request', and that data
	 * is copied to or from the driver->ioctl() 'buf' parameter.
	 */
	static int
	usbdev_ioctl (int fd, int ifno, unsigned request, void *param)
	{
		struct usbdevfs_ioctl   wrapper;

		wrapper.ifno = ifno;
		wrapper.ioctl_code = request;
		wrapper.data = param;

		return ioctl (fd, USBDEVFS_IOCTL, &wrapper);
	}

    鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
    姝よ姹傝鍐呮牳椹卞姩閫氳繃鏂囦欢绯荤粺鎿嶄綔涓庣敤鎴锋ā寮忎唬鐮佸璇濓紝鍗充娇瀹冧滑娌℃湁鍒涘缓瀛楃鎴?    鍧楃壒娈婅澶囥€傚畠涔熻鐢ㄦ潵鍋氳濡傝闂澶囧簲浣跨敤鍝釜璁惧鐗规畩鏂囦欢涔嬬被鐨勪簨鎯呫€備袱涓?    棰勫畾涔夌殑 ioctl 鐢ㄤ簬鏂紑鍜岄噸鏂拌繛鎺ュ唴鏍搁┍鍔紝浠ヤ究鐢ㄦ埛妯″紡浠ｇ爜鍙互瀹屽叏绠＄悊璁惧
    鐨勭粦瀹氬拰閰嶇疆銆?
```
USBDEVFS_RELEASEINTERFACE
    杩欑敤浜庡湪鍏抽棴鏂囦欢鎻忚堪绗︿箣鍓嶏紝閲婃斁 usbfs 瀵规帴鍙ｆ墍鍋氱殑澹版槑锛堟棤璁烘槸闅愬紡鐨勮繕鏄?    鐢变簬 USBDEVFS_CLAIMINTERFACE 璋冪敤锛夈€俰octl 鍙傛暟鏄竴涓繚瀛樻帴鍙ｇ紪鍙凤紙鏉ヨ嚜
    鎻忚堪绗︾殑 bInterfaceNumber锛夌殑鏁存暟锛涙枃浠朵慨鏀规椂闂翠笉浼氬洜涓烘璇锋眰鑰屾洿鏂般€?
```

	*涓嶈繘琛屾鏌ヤ互纭繚鍙戣捣澹版槑鐨勪换鍔″氨鏄噴鏀惧畠鐨勯偅涓换鍔°€傝繖鎰忓懗鐫€鐢ㄦ埛妯″紡
	椹卞姩鍙兘浼氬共鎵板叾浠栭┍鍔ㄣ€?

```
USBDEVFS_RESETEP
    灏嗙鐐癸紙鎵归噺鎴栦腑鏂級鐨勬暟鎹垏鎹㈠€奸噸缃负 DATA0銆俰octl 鍙傛暟鏄竴涓暣鏁扮殑绔偣
    缂栧彿锛? 鍒?15锛屽绔偣鎻忚堪绗︿腑鎵€鏍囪瘑锛夛紝濡傛灉璁惧鐨勭鐐瑰悜涓绘満鍙戦€佹暟鎹紝鍒?    鍔犱笂 USB_DIR_IN銆?
```

	*閬垮厤浣跨敤姝よ姹傘€傚畠鍙兘搴旇琚Щ闄ゃ€? 浣跨敤瀹冮€氬父鎰忓懗鐫€璁惧鍜岄┍鍔ㄥ皢澶卞幓
	鍒囨崲鍚屾銆傚鏋滀綘鐪熺殑澶卞幓浜嗗悓姝ワ紝浣犲彲鑳介渶瑕佷娇鐢ㄥ儚 CLEAR_HALT 鎴?	SET_INTERFACE 杩欐牱鐨勮姹備笌璁惧瀹屽叏鎻℃墜銆?
```
USBDEVFS_DROP_PRIVILEGES
    杩欑敤浜庢斁寮冨湪 usbfs 鏂囦欢鎻忚堪绗︿笂鎵ц鏌愪簺琚涓烘槸鐗规潈鎿嶄綔鐨勮兘鍔涖€傝繖鍖呮嫭澹版槑
    浠绘剰鎺ュ彛銆侀噸缃竴涓綋鍓嶆湁鍏朵粬鐢ㄦ埛澹版槑浜嗘帴鍙ｇ殑璁惧锛屼互鍙婂彂鍑?USBDEVFS_IOCTL
    璋冪敤銆俰octl 鍙傛暟鏄竴涓?32 浣嶆帺鐮侊紝琛ㄧず鐢ㄦ埛琚厑璁稿湪姝ゆ枃浠舵弿杩扮涓婂０鏄庣殑鎺ュ彛銆?    浣犲彲浠ュ娆″彂鍑烘 ioctl 浠ユ敹绐勮鎺╃爜銆?
#### 鍚屾 I/O 鏀寔


鍚屾璇锋眰娑夊強鍐呮牳闃诲锛岀洿鍒扮敤鎴锋ā寮忚姹傚畬鎴愶紝瑕佷箞鎴愬姛瀹屾垚锛岃涔堟姤鍛婇敊璇€傚湪
澶у鏁版儏鍐典笅杩欐槸浣跨敤 usbfs 鐨勬渶绠€鍗曟柟寮忥紝灏界濡備笂鎵€杩帮紝瀹冪‘瀹為樆姝簡鍚屾椂鍚戝涓?绔偣鎵ц I/O銆?
USBDEVFS_BULK
    鍚戣澶囧彂鍑轰竴涓壒閲忚鎴栧啓璇锋眰銆俰octl

```

	struct usbdevfs_bulktransfer {
		unsigned int  ep;
		unsigned int  len;
		unsigned int  timeout; /* in milliseconds */
		void          *data;
	};

    ``ep`` 鍊兼爣璇嗕竴涓壒閲忕鐐圭紪鍙凤紙1 鍒?15锛屽绔偣鎻忚堪绗︿腑鎵€鏍囪瘑锛夛紝褰撳紩鐢ㄤ竴涓?    浠庤澶囧悜涓绘満鍙戦€佹暟鎹殑绔偣鏃讹紝鐢?USB_DIR_IN 鎺╃爜銆傛暟鎹紦鍐插尯鐨勯暱搴︾敱 ``len``
    鏍囪瘑锛涜繎鏈熺殑鍐呮牳鏀寔楂樿揪绾?128K 瀛楄妭鐨勮姹傘€?FIXME 璇存槑濡備綍杩斿洖璇诲彇闀垮害锛?    浠ュ強濡備綍澶勭悊鐭銆?銆?
```
USBDEVFS_CLEAR_HALT
    娓呴櫎绔偣鏆傚仠锛坔alt/stall锛夊苟閲嶇疆绔偣鍒囨崲銆傝繖浠呭鏍囬噺鎴栦腑鏂鐐规湁鎰忎箟銆俰octl
    鍙傛暟鏄竴涓暣鏁扮殑绔偣缂栧彿锛? 鍒?15锛屽绔偣鎻忚堪绗︿腑鎵€鏍囪瘑锛夛紝褰撳紩鐢ㄤ竴涓粠璁惧
    鍚戜富鏈哄彂閫佹暟鎹殑绔偣鏃讹紝鐢?USB_DIR_IN 鎺╃爜銆?
    鍦ㄥ凡缁忓仠姝紙stall锛夈€佸悜鏁版嵁浼犺緭璇锋眰杩斿洖 `-EPIPE` 鐘舵€佺殑鎵归噺鎴栦腑鏂鐐逛笂浣跨敤
    瀹冦€備笉瑕佺洿鎺ュ彂鍑烘帶鍒惰姹傦紝鍥犱负閭ｄ細浣夸富鏈虹殑鏁版嵁鍒囨崲璁板綍澶辨晥銆?
USBDEVFS_CONTROL
    鍚戣澶囧彂鍑轰竴涓帶鍒惰姹傘€俰octl 鍙傛暟鎸囧悜

```

	struct usbdevfs_ctrltransfer {
		__u8   bRequestType;
		__u8   bRequest;
		__u16  wValue;
		__u16  wIndex;
		__u16  wLength;
		__u32  timeout;  /* in milliseconds */
		void   *data;
	};

    姝ょ粨鏋勭殑鍓嶅叓涓瓧鑺傛槸瑕佸彂閫佺粰璁惧鐨?SETUP 鍖呯殑鍐呭锛涜瑙?USB 2.0 瑙勮寖銆?    bRequestType 鍊兼槸閫氳繃缁勫悎涓€涓?``USB_TYPE_*`` 鍊笺€佷竴涓?``USB_DIR_*`` 鍊煎拰涓€涓?    ``USB_RECIP_*`` 鍊硷紙鏉ヨ嚜 ``linux/usb.h``锛夋瀯鎴愮殑銆傚鏋?wLength 闈為浂锛屽畠鎻忚堪
    鏁版嵁缂撳啿鍖洪暱搴︼紝璇ョ紦鍐插尯鎴栬€呰鍐欏叆璁惧锛圲SB_DIR_OUT锛夛紝鎴栬€呬粠璁惧璇诲彇
    锛圲SB_DIR_IN锛夈€?
    鍦ㄦ挵鍐欐湰鏂囨椂锛屼綘涓嶈兘鍦ㄨ澶囧拰涓绘満涔嬮棿浼犺緭瓒呰繃 4 KB 鐨勬暟鎹紱usbfs 鏈変竴涓檺鍒讹紝
    涓€浜涗富鏈烘帶鍒跺櫒椹卞姩涔熸湁涓€涓檺鍒躲€傦紙杩欓€氬父涓嶆槸闂銆傦級*鍙﹀*锛屾病鏈夊姙娉曡浠?    璁惧鑾峰緱鐭鍥炴槸涓嶅彲浠ョ殑銆?
```
USBDEVFS_RESET
    鎵ц涓€涓?USB 绾х殑璁惧澶嶄綅銆俰octl 鍙傛暟琚拷鐣ャ€傚浣嶄箣鍚庯紝杩欎細閲嶆柊缁戝畾鎵€鏈?    璁惧鎺ュ彛銆傛枃浠朵慨鏀规椂闂翠笉浼氬洜涓烘璇锋眰鑰屾洿鏂般€?

	**閬垮厤浣跨敤姝よ皟鐢?* 鐩村埌鏌愪簺 usbcore bug 琚慨澶嶏紝鍥犱负瀹冩病鏈夊畬鍏ㄥ悓姝ヨ澶囥€?	鎺ュ彛鍜岄┍鍔紙涓嶄粎浠呮槸 usbfs锛夌姸鎬併€?
USBDEVFS_SETINTERFACE
    璁剧疆鎺ュ彛鐨勫鐢ㄨ缃€俰octl 鍙傛暟

```

	struct usbdevfs_setinterface {
		unsigned int  interface;
		unsigned int  altsetting;
	};

    鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
    閭ｄ簺缁撴瀯鎴愬憳鏉ヨ嚜搴旂敤浜庡綋鍓嶉厤缃殑鏌愪釜鎺ュ彛鎻忚堪绗︺€傛帴鍙ｇ紪鍙锋槸 bInterfaceNumber
    鍊硷紝澶囩敤璁剧疆缂栧彿鏄?bAlternateSetting 鍊笺€傦紙杩欎細閲嶇疆鎺ュ彛涓殑姣忎釜绔偣銆傦級

```
USBDEVFS_SETCONFIGURATION
    涓鸿澶囧彂鍑?`usb_set_configuration()` 璋冪敤銆傚弬鏁版槸涓€涓繚瀛橀厤缃紪鍙凤紙鏉ヨ嚜
    鎻忚堪绗︾殑 bConfigurationValue锛夌殑鏁存暟銆傛枃浠朵慨鏀规椂闂翠笉浼氬洜涓烘璇锋眰鑰屾洿鏂般€?

	**閬垮厤浣跨敤姝よ皟鐢?* 鐩村埌鏌愪簺 usbcore bug 琚慨澶嶏紝鍥犱负瀹冩病鏈夊畬鍏ㄥ悓姝ヨ澶囥€?	鎺ュ彛鍜岄┍鍔紙涓嶄粎浠呮槸 usbfs锛夌姸鎬併€?
#### 寮傛 I/O 鏀寔


濡備笂鎵€杩帮紝鍦ㄦ煇浜涙儏鍐典笅锛屼粠鐢ㄦ埛妯″紡浠ｇ爜鍙戣捣骞跺彂鎿嶄綔鍙兘寰堥噸瑕併€傝繖瀵瑰懆鏈熸€т紶杈?锛堜腑鏂拰绛夋椂锛夊挨鍏堕噸瑕侊紝浣嗗畠涔熷彲浠ョ敤浜庡叾浠栫绫荤殑 USB 璇锋眰銆傚湪杩欑鎯呭喌涓嬶紝杩欓噷
鎻忚堪鐨勫紓姝ヨ姹傛槸蹇呬笉鍙皯鐨勩€備笉鏄彁浜や竴涓姹傚苟璁╁唴鏍搁樆濉炵洿鍒板畠瀹屾垚锛岃€屾槸灏嗛樆濉?鍒嗙寮€鏉ャ€?
杩欎簺璇锋眰琚墦鍖呭埌涓€涓被浼间簬鍐呮牳璁惧椹卞姩浣跨敤鐨?URB 鐨勭粨鏋勪腑銆傦紙杩欓噷娌℃湁 POSIX
寮傛 I/O 鏀寔锛屾姳姝夈€傦級瀹冩爣璇嗙鐐圭被鍨嬶紙`USBDEVFS_URB_TYPE_*`锛夈€佺鐐癸紙缂栧彿锛?閰屾儏鐢?USB_DIR_IN 鎺╃爜锛夈€佺紦鍐插尯鍜岄暱搴︼紝浠ュ強涓€涓敤浜庡敮涓€鏍囪瘑姣忎釜璇锋眰鐨勭敤鎴?鈥滀笂涓嬫枃鈥濆€笺€傦紙瀹冮€氬父鏄寚鍚戞瘡璇锋眰鏁版嵁鐨勬寚閽堛€傦級鏍囧織鍙互淇敼璇锋眰锛堟病鏈夊唴鏍搁┍鍔?鏀寔鐨勯偅涔堝锛夈€?
姣忎釜璇锋眰鍙互鎸囧畾涓€涓疄鏃朵俊鍙风紪鍙凤紙鍦?SIGRTMIN 鍜?SIGRTMAX 涔嬮棿锛屽惈杈圭晫锛夛紝浠?璇锋眰鍦ㄨ姹傚畬鎴愭椂鍙戦€佷竴涓俊鍙枫€?
褰?usbfs 杩斿洖杩欎簺 urb 鏃讹紝鐘舵€佸€艰鏇存柊锛屽苟涓旂紦鍐插尯鍙兘宸茶淇敼銆傞櫎浜嗙瓑鏃朵紶杈?涔嬪锛宎ctual_length 琚洿鏂颁互璇存槑浼犺緭浜嗗灏戝瓧鑺傦紱濡傛灉璁剧疆浜?USBDEVFS_URB_DISABLE_SPD 鏍囧織锛堚€滅煭鍖呬笉鍙互鈥濓級锛屽鏋?
```

    struct usbdevfs_iso_packet_desc {
	    unsigned int                     length;
	    unsigned int                     actual_length;
	    unsigned int                     status;
    };

    struct usbdevfs_urb {
	    unsigned char                    type;
	    unsigned char                    endpoint;
	    int                              status;
	    unsigned int                     flags;
	    void                             *buffer;
	    int                              buffer_length;
	    int                              actual_length;
	    int                              start_frame;
	    int                              number_of_packets;
	    int                              error_count;
	    unsigned int                     signr;
	    void                             *usercontext;
	    struct usbdevfs_iso_packet_desc  iso_frame_desc[];
    };

```
瀵逛簬杩欎簺寮傛璇锋眰锛屾枃浠朵慨鏀规椂闂村弽鏄犺姹傝鍙戣捣鐨勬椂闂淬€傝繖涓庡畠浠湪鍚屾璇锋眰涓殑
浣跨敤褰㈡垚瀵规瘮锛屽湪鍚屾璇锋眰涓畠鍙嶆槧璇锋眰瀹屾垚鐨勬椂闂淬€?
USBDEVFS_DISCARDURB
    **TBS** 鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
USBDEVFS_DISCSIGNAL
    **TBS** 鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
USBDEVFS_REAPURB
    **TBS** 鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
USBDEVFS_REAPURBNDELAY
    **TBS** 鏂囦欢淇敼鏃堕棿涓嶄細鍥犱负姝よ姹傝€屾洿鏂般€?
USBDEVFS_SUBMITURB
    **TBS**

## USB 璁惧


USB 璁惧鐜板湪閫氳繃 debugfs 瀵煎嚭锛?
- `/sys/kernel/debug/usb/devices` 鈥︹€?涓€涓枃鏈枃浠讹紝鏄剧ず鍐呮牳宸茬煡鐨勬瘡涓?USB 璁惧
   鍙婂叾閰嶇疆鎻忚堪绗︺€備綘涔熷彲浠?poll() 瀹冩潵浜嗚В鏂拌澶囥€?
### /sys/kernel/debug/usb/devices


姝ゆ枃浠跺鐢ㄦ埛妯″紡涓殑鐘舵€佹煡鐪嬪伐鍏峰緢鏂逛究锛岃繖浜涘伐鍏峰彲浠ユ壂鎻忔枃鏈牸寮忓苟蹇界暐澶ч儴鍒?鍐呭銆傛洿璇︾粏鐨勮澶囩姸鎬侊紙鍖呮嫭绫诲拰渚涘簲鍟嗙姸鎬侊級鍙粠璁惧鐗瑰畾鐨勬枃浠朵腑鑾峰彇銆傚叧浜庢
鏂囦欢褰撳墠鏍煎紡鐨勪俊鎭紝璇疯涓嬫枃銆?
姝ゆ枃浠朵笌 poll() 绯荤粺璋冪敤缁撳悎锛屼篃鍙互鐢ㄤ簬

```

    int fd;
    struct pollfd pfd;

    fd = open("/sys/kernel/debug/usb/devices", O_RDONLY);
    pfd = { fd, POLLIN, 0 };
    for (;;) {
	/* The first time through, this call will return immediately. */
	poll(&pfd, 1, -1);

	/* To see what's changed, compare the file's previous and current
	   contents or scan the filesystem.  (Scanning is more precise.) */
    }

```
娉ㄦ剰锛岃繖绉嶈涓烘棬鍦ㄧ敤浜庝俊鎭拰璋冭瘯鐩殑銆備緥濡傦紝浣跨敤鍍?udev 鎴?HAL 杩欐牱鐨勭▼搴忔潵
鍒濆鍖栬澶囨垨鍚姩鐢ㄦ埛妯″紡杈呭姪绋嬪簭浼氭洿鍚堥€傘€?
鍦ㄦ鏂囦欢涓紝姣忎釜璁惧鐨勮緭鍑烘湁澶氳 ASCII 杈撳嚭銆?
鎴戠壒鎰忔妸瀹冨仛鎴?ASCII 鑰岄潪浜岃繘鍒讹紝浠ヤ究鏈変汉鏃犻渶浣跨敤杈呭姪绋嬪簭灏辫兘浠庝腑鑾峰彇涓€浜?鏈夌敤鏁版嵁銆備絾鏄紝鍊熷姪杈呭姪绋嬪簭锛屾瘡涓?`T:` 琛岋紙鎷撴墤淇℃伅锛歀ev銆丳rnt銆丳ort銆丆nt锛?鐨勫墠 4 鍒椾腑鐨勬暟瀛楀彲鐢ㄤ簬鏋勫缓 USB 鎷撴墤鍥俱€?
```

	T = Topology (etc.)
	B = Bandwidth (applies only to USB host controllers, which are
	virtualized as root hubs)
	D = Device descriptor info.
	P = Product ID info. (from Device descriptor, but they won't fit
	together on one line)
	S = String descriptors.
	C = Configuration descriptor info. (* = active configuration)
	I = Interface descriptor info.
	E = Endpoint descriptor info.

```
#### /sys/kernel/debug/usb/devices 杈撳嚭鏍煎紡


```

  d = decimal number (may have leading spaces or 0's)
  x = hexadecimal number (may have leading spaces or 0's)
  s = string



```
##### 鎷撴墤淇℃伅


```

	T:  Bus=dd Lev=dd Prnt=dd Port=dd Cnt=dd Dev#=ddd Spd=dddd MxCh=dd
	|   |      |      |       |       |      |        |        |__MaxChildren
	|   |      |      |       |       |      |        |__Device Speed in Mbps
	|   |      |      |       |       |      |__DeviceNumber
	|   |      |      |       |       |__Count of devices at this level
	|   |      |      |       |__Connector/Port on Parent for this device
	|   |      |      |__Parent DeviceNumber
	|   |      |__Level in topology for this bus
	|   |__Bus number
	|__Topology info tag

```
閫熷害鍙兘鏄細

	======= ======================================================
	1.5	Mbit/s for low speed USB
	12	Mbit/s for full speed USB
	480	Mbit/s for high speed USB (added for USB 2.0)
	5000	Mbit/s for SuperSpeed USB (added for USB 3.0)
	======= ======================================================

鐢变簬杩峰け鍦ㄦ椂闂寸殑杩烽浘涓師鍥狅紝绔彛鍙锋€绘槸姣斿疄闄呭皬 1銆備緥濡傦紝鎻掑叆绔彛 4 鐨勮澶?灏嗘樉绀轰负 `Port=03`銆?
##### 甯﹀淇℃伅


```

	B:  Alloc=ddd/ddd us (xx%), #Int=ddd, #Iso=ddd
	|   |                       |         |__Number of isochronous requests
	|   |                       |__Number of interrupt requests
	|   |__Total Bandwidth allocated to this bus
	|__Bandwidth info tag

```
甯﹀鍒嗛厤鏄涓€甯э紙姣锛変腑鏈夊灏戣浣跨敤鐨勪竴涓繎浼笺€傚畠鍙弽鏄犲懆鏈熸€т紶杈擄紝杩欐槸
鍞竴棰勭暀甯﹀鐨勪紶杈撱€傛帶鍒跺拰鎵归噺浼犺緭浣跨敤鎵€鏈夊叾浠栧甫瀹斤紝鍖呮嫭鏈鐢ㄤ簬浼犺緭锛堜緥濡?鐢ㄤ簬鐭寘锛夌殑棰勭暀甯﹀銆?
璇ョ櫨鍒嗘瘮鏄偅浜涗紶杈撹皟搴︿簡澶氬皯鈥滈鐣欌€濆甫瀹姐€傚浜庝綆閫熸垨鍏ㄩ€熸€荤嚎锛堢矖鐣ュ湴璇存槸
鈥淯SB 1.1鈥濓級锛岄鐣欎簡 90% 鐨勬€荤嚎甯﹀銆傚浜庨珮閫熸€荤嚎锛堢矖鐣ュ湴璇存槸鈥淯SB 2.0鈥濓級
棰勭暀浜?80%銆?

##### 璁惧鎻忚堪绗︿俊鎭笌浜у搧 ID 淇℃伅


```

	D:  Ver=x.xx Cls=xx(s) Sub=xx Prot=xx MxPS=dd #Cfgs=dd
	P:  Vendor=xxxx ProdID=xxxx Rev=xx.xx

```
```

	D:  Ver=x.xx Cls=xx(sssss) Sub=xx Prot=xx MxPS=dd #Cfgs=dd
	|   |        |             |      |       |       |__NumberConfigurations
	|   |        |             |      |       |__MaxPacketSize of Default Endpoint
	|   |        |             |      |__DeviceProtocol
	|   |        |             |__DeviceSubClass
	|   |        |__DeviceClass
	|   |__Device USB version
	|__Device info tag #1

```
```

	P:  Vendor=xxxx ProdID=xxxx Rev=xx.xx
	|   |           |           |__Product revision number
	|   |           |__Product ID code
	|   |__Vendor ID code
	|__Device info tag #2


```
##### 瀛楃涓叉弿杩扮淇℃伅


```

	S:  Manufacturer=ssss
	|   |__Manufacturer of this device as read from the device.
	|      For USB host controller drivers (virtual root hubs) this may
	|      be omitted, or (for newer drivers) will identify the kernel
	|      version and the driver which provides this hub emulation.
	|__String info tag

	S:  Product=ssss
	|   |__Product description of this device as read from the device.
	|      For older USB host controller drivers (virtual root hubs) this
	|      indicates the driver; for newer ones, it's a product (and vendor)
	|      description that often comes from the kernel's PCI ID database.
	|__String info tag

	S:  SerialNumber=ssss
	|   |__Serial Number of this device as read from the device.
	|      For USB host controller drivers (virtual root hubs) this is
	|      some unique ID, normally a bus ID (address or slot name) that
	|      can't be shared with any other device.
	|__String info tag



```
##### 閰嶇疆鎻忚堪绗︿俊鎭?

```

	C:* #Ifs=dd Cfg#=dd Atr=xx MPwr=dddmA
	| | |       |       |      |__MaxPower in mA
	| | |       |       |__Attributes
	| | |       |__ConfiguratioNumber
	| | |__NumberOfInterfaces
	| |__ "*" indicates the active configuration (others are " ")
	|__Config info tag

```
USB 璁惧鍙兘鏈夊涓厤缃紝姣忎釜鐨勮涓哄ぇ涓嶇浉鍚屻€備緥濡傦紝涓€涓€荤嚎渚涚數鐨勯厤缃彲鑳芥瘮
鑷緵鐢电殑閰嶇疆鑳藉姏寮卞緱澶氥€備竴娆″彧鑳芥湁涓€涓澶囬厤缃浜庢椿鍔ㄧ姸鎬侊紱澶у鏁拌澶囧彧鏈?涓€涓厤缃€?
姣忎釜閰嶇疆鐢变竴涓垨澶氫釜鎺ュ彛缁勬垚銆傛瘡涓帴鍙ｆ湇鍔′簬涓€涓笉鍚岀殑鈥滃姛鑳解€濓紝閫氬父缁戝畾鍒?涓嶅悓鐨?USB 璁惧椹卞姩銆備竴涓父瑙佺殑渚嬪瓙鏄竴涓甫鏈夌敤浜庢挱鏀剧殑闊抽鎺ュ彛鍜岀敤浜庤蒋浠堕煶閲?鎺у埗鐨?HID 鎺ュ彛鐨?USB 鎵０鍣ㄣ€?
##### 鎺ュ彛鎻忚堪绗︿俊鎭紙姣忎釜閰嶇疆鍙湁澶氫釜锛?

```

	I:* If#=dd Alt=dd #EPs=dd Cls=xx(sssss) Sub=xx Prot=xx Driver=ssss
	| | |      |      |       |             |      |       |__Driver name
	| | |      |      |       |             |      |          or "(none)"
	| | |      |      |       |             |      |__InterfaceProtocol
	| | |      |      |       |             |__InterfaceSubClass
	| | |      |      |       |__InterfaceClass
	| | |      |      |__NumberOfEndpoints
	| | |      |__AlternateSettingNumber
	| | |__InterfaceNumber
	| |__ "*" indicates the active altsetting (others are " ")
	|__Interface info tag

```
涓€涓粰瀹氱殑鎺ュ彛鍙兘鏈変竴涓垨澶氫釜鈥滃鐢ㄢ€濊缃€備緥濡傦紝榛樿璁剧疆鍙兘涓嶄娇鐢ㄨ秴杩囧皯閲忕殑
鍛ㄦ湡鎬у甫瀹姐€傝浣跨敤鎬荤嚎甯﹀鐨勬樉钁楅儴鍒嗭紝椹卞姩蹇呴』閫夋嫨涓€涓潪榛樿鐨勫鐢ㄨ缃€?
涓€涓帴鍙ｄ竴娆″彧鑳芥湁涓€涓缃浜庢椿鍔ㄧ姸鎬侊紝骞朵笖涓€娆″彧鑳芥湁涓€涓┍鍔ㄧ粦瀹氬埌涓€涓?鎺ュ彛銆傚ぇ澶氭暟璁惧姣忎釜鎺ュ彛鍙湁涓€涓鐢ㄨ缃€?

##### 绔偣鎻忚堪绗︿俊鎭紙姣忎釜鎺ュ彛鍙湁澶氫釜锛?

```

	E:  Ad=xx(s) Atr=xx(ssss) MxPS=dddd Ivl=dddss
	|   |        |            |         |__Interval (max) between transfers
	|   |        |            |__EndpointMaxPacketSize
	|   |        |__Attributes(EndpointType)
	|   |__EndpointAddress(I=In,O=Out)
	|__Endpoint info tag

```
瀵逛簬鎵€鏈夊懆鏈熸€э紙涓柇鎴栫瓑鏃讹級绔偣锛岄棿闅旈兘鏄潪闆剁殑銆傚浜庨珮閫熺鐐癸紝浼犺緭闂撮殧鍙兘
浠ュ井绉掕€岄潪姣鏉ュ害閲忋€?
瀵逛簬楂橀€熷懆鏈熸€х鐐癸紝`EndpointMaxPacketSize` 鍙嶆槧姣忓井甯х殑鏁版嵁浼犺緭澶у皬銆傚浜?鈥滈珮甯﹀鈥濈鐐癸紝閭ｅ彲浠ュ弽鏄犳瘡涓鐐逛袱涓垨涓変釜鍖咃紙姣?125 寰鏈€澶?3KB锛夈€?
浣跨敤 Linux-USB 鍗忚鏍堬紝鍛ㄦ湡鎬у甫瀹介鐣欎娇鐢?URB 鎻愪緵鐨勪紶杈撻棿闅斿拰澶у皬锛屽畠浠彲鑳?灏忎簬绔偣鎻忚堪绗︿腑鎵惧埌鐨勯偅浜涖€?
#### 浣跨敤绀轰緥


濡傛灉鐢ㄦ埛鎴栬剼鏈彧瀵规爣閲忎俊鎭劅鍏磋叮锛屼緥濡傦紝浣跨敤绫讳技
`grep ^T: /sys/kernel/debug/usb/devices` 鐨勫懡浠ゅ彧鑾峰彇鎷撴墤琛屻€傚儚
`grep -i ^[tdp]: /sys/kernel/debug/usb/devices` 杩欐牱鐨勫懡浠ゅ彲鐢ㄤ簬鍙垪鍑轰互鏂规嫭鍙蜂腑
瀛楃寮€澶寸殑琛岋紝鍏朵腑鏈夋晥瀛楃涓?TDPCIE銆傚€熷姪绋嶅己涓€鐐圭殑鑴氭湰锛屽畠鍙互鏄剧ず浠讳綍閫夊畾鐨?琛岋紙渚嬪锛屽彧鏈?T銆丏 鍜?P 琛岋級骞舵洿鏀瑰畠浠殑杈撳嚭鏍煎紡銆傦紙`procusb` Perl 鑴氭湰鏄?杩欎釜鎯虫硶鐨勫紑绔€傚畠灏嗗彧鍒楀嚭浠?TBDPSCIE 涓€夊畾鐨勮锛屾垨鏉ヨ嚜 `/sys/kernel/debug/usb/devices`
鐨勨€滄墍鏈夆€濊銆傦級

鎷撴墤琛屽彲鐢ㄤ簬鐢熸垚绯荤粺鏍归泦绾垮櫒涓?USB 璁惧鐨勫浘褰?鍥剧ず銆傦紙鍏充簬濡備綍鎵ц姝ゆ搷浣滐紝璇?鍙傝涓嬫枃鏇村鍐呭銆傦級

鎺ュ彛琛屽彲鐢ㄤ簬纭畾姣忎釜璁惧姝ｅ湪浣跨敤浠€涔堥┍鍔紝浠ュ強瀹冩縺娲讳簡鍝釜澶囩敤璁剧疆銆?
閰嶇疆琛屽彲鐢ㄤ簬鍒楀嚭绯荤粺 USB 璁惧姝ｅ湪浣跨敤鐨勬渶澶у姛鐜囷紙浠ユ瀹変负鍗曚綅锛夈€備緥濡傦紝
`grep ^C: /sys/kernel/debug/usb/devices`銆?

杩欐槸涓€涓緥瀛愶紝鏉ヨ嚜涓€涓叿鏈?UHCI 鏍归泦绾垮櫒銆佽繛鎺ュ埌鏍归泦绾垮櫒鐨勫閮ㄩ泦绾垮櫒锛屼互鍙?杩炴帴鍒板閮ㄩ泦绾垮櫒鐨勯紶鏍囧拰涓茶杞崲鍣ㄧ殑绯荤粺銆?
```

	T:  Bus=00 Lev=00 Prnt=00 Port=00 Cnt=00 Dev#=  1 Spd=12   MxCh= 2
	B:  Alloc= 28/900 us ( 3%), #Int=  2, #Iso=  0
	D:  Ver= 1.00 Cls=09(hub  ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=0000 ProdID=0000 Rev= 0.00
	S:  Product=USB UHCI Root Hub
	S:  SerialNumber=dce0
	C:* #Ifs= 1 Cfg#= 1 Atr=40 MxPwr=  0mA
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
	E:  Ad=81(I) Atr=03(Int.) MxPS=   8 Ivl=255ms

	T:  Bus=00 Lev=01 Prnt=01 Port=00 Cnt=01 Dev#=  2 Spd=12   MxCh= 4
	D:  Ver= 1.00 Cls=09(hub  ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=0451 ProdID=1446 Rev= 1.00
	C:* #Ifs= 1 Cfg#= 1 Atr=e0 MxPwr=100mA
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
	E:  Ad=81(I) Atr=03(Int.) MxPS=   1 Ivl=255ms

	T:  Bus=00 Lev=02 Prnt=02 Port=00 Cnt=01 Dev#=  3 Spd=1.5  MxCh= 0
	D:  Ver= 1.00 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=04b4 ProdID=0001 Rev= 0.00
	C:* #Ifs= 1 Cfg#= 1 Atr=80 MxPwr=100mA
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=03(HID  ) Sub=01 Prot=02 Driver=mouse
	E:  Ad=81(I) Atr=03(Int.) MxPS=   3 Ivl= 10ms

	T:  Bus=00 Lev=02 Prnt=02 Port=02 Cnt=02 Dev#=  4 Spd=12   MxCh= 0
	D:  Ver= 1.00 Cls=00(>ifc ) Sub=00 Prot=00 MxPS= 8 #Cfgs=  1
	P:  Vendor=0565 ProdID=0001 Rev= 1.08
	S:  Manufacturer=Peracom Networks, Inc.
	S:  Product=Peracom USB to Serial Converter
	C:* #Ifs= 1 Cfg#= 1 Atr=a0 MxPwr=100mA
	I:  If#= 0 Alt= 0 #EPs= 3 Cls=00(>ifc ) Sub=00 Prot=00 Driver=serial
	E:  Ad=81(I) Atr=02(Bulk) MxPS=  64 Ivl= 16ms
	E:  Ad=01(O) Atr=02(Bulk) MxPS=  16 Ivl= 16ms
	E:  Ad=82(I) Atr=03(Int.) MxPS=   8 Ivl=  8ms


```
鍙粠姝や腑閫夋嫨 `T:` 鍜?`I:` 琛岋紙渚嬪锛屼娇鐢?`procusb ti`锛夛紝鎴戜滑鏈?
```

	T:  Bus=00 Lev=00 Prnt=00 Port=00 Cnt=00 Dev#=  1 Spd=12   MxCh= 2
	T:  Bus=00 Lev=01 Prnt=01 Port=00 Cnt=01 Dev#=  2 Spd=12   MxCh= 4
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=09(hub  ) Sub=00 Prot=00 Driver=hub
	T:  Bus=00 Lev=02 Prnt=02 Port=00 Cnt=01 Dev#=  3 Spd=1.5  MxCh= 0
	I:  If#= 0 Alt= 0 #EPs= 1 Cls=03(HID  ) Sub=01 Prot=02 Driver=mouse
	T:  Bus=00 Lev=02 Prnt=02 Port=02 Cnt=02 Dev#=  4 Spd=12   MxCh= 0
	I:  If#= 0 Alt= 0 #EPs= 3 Cls=00(>ifc ) Sub=00 Prot=00 Driver=serial


```
```

                      +------------------+
                      |  PC/root_hub (12)|   Dev# = 1
                      +------------------+   (nn) is Mbps.
    Level 0           |  CN.0   |  CN.1  |   [CN = connector/port #]
                      +------------------+
                          /
                         /
            +-----------------------+
  Level 1   | Dev#2: 4-port hub (12)|
            +-----------------------+
            |CN.0 |CN.1 |CN.2 |CN.3 |
            +-----------------------+
                \           \____________________
                 \_____                          \
                       \                          \
               +--------------------+      +--------------------+
  Level 2      | Dev# 3: mouse (1.5)|      | Dev# 4: serial (12)|
               +--------------------+      +--------------------+



```
鎴栬€咃紝浠ユ洿鍍忔爲鐨勭粨鏋勶紙涓嶅甫绔彛 [杩炴帴鍣╙ 鐨?

```

	PC:  Dev# 1, root hub, 2 ports, 12 Mbps
	|_ CN.0:  Dev# 2, hub, 4 ports, 12 Mbps
	     |_ CN.0:  Dev #3, mouse, 1.5 Mbps
	     |_ CN.1:
	     |_ CN.2:  Dev #4, serial, 12 Mbps
	     |_ CN.3:
	|_ CN.1:

```
