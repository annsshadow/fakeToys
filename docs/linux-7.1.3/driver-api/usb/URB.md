#### USB Request Block锛圲RB锛?


:Revised: 2000-Dec-05
:Again:   2002-Jul-06
:Again:   2005-Sep-19
:Again:   2017-Mar-29



    USB 瀛愮郴缁熺幇鍦ㄥ湪 usb-hostside-api 绔犺妭鏈変竴涓浉褰撳畬鏁寸殑閮ㄥ垎锛岀敱褰撳墠
    婧愪唬鐮佺敓鎴愩€傝繖浠界壒瀹氱殑鏂囨。骞朵笉瀹屾暣锛屼篃鍙兘娌℃湁鏇存柊鍒版渶鏂扮増鏈紱
    闄ゅ揩閫熸瑙堜箣澶栵紝璇峰嬁渚濊禆瀹冦€?

## 鍩烘湰姒傚康锛屾垨鑰呰鈥滀粈涔堟槸 URB锛熲€?


鏂伴┍鍔ㄧ殑鍩烘湰鎬濇兂鏄秷鎭紶閫掞紝娑堟伅鏈韩琚О涓?USB 璇锋眰鍧楋紙USB Request
Block锛夛紝绠€绉?URB銆?

- 涓€涓?URB 鍖呭惈鎵ц浠讳綍 USB 浜嬪姟骞跺皢鏁版嵁涓庣姸鎬佸洖浼犳墍闇€鐨勫叏閮ㄧ浉鍏充俊鎭€?

- URB 鐨勬墽琛屾湰璐ㄤ笂鏄竴涓紓姝ユ搷浣滐紝鍗?`usb_submit_urb` 璋冪敤鍦ㄦ垚鍔熷湴灏?
  鎵€璇锋眰鐨勬搷浣滃姞鍏ラ槦鍒楀悗浼氱珛鍗宠繑鍥炪€?

- 涓€涓?URB 鐨勪紶杈撳彲浠ラ殢鏃堕€氳繃 `usb_unlink_urb` 鍙栨秷銆?

- 姣忎釜 URB 閮芥湁涓€涓畬鎴愬鐞嗗嚱鏁帮紝鍦ㄥ姩浣滄垚鍔熷畬鎴愭垨鍙栨秷鍚庤璋冪敤銆俇RB
  杩樺寘鍚竴涓敤浜庡悜瀹屾垚澶勭悊鍑芥暟浼犻€掍俊鎭殑涓婁笅鏂囨寚閽堛€?

- 璁惧鐨勬瘡涓鐐逛粠閫昏緫涓婇兘鏀寔涓€涓姹傞槦鍒椼€備綘鍙互濉弧璇ラ槦鍒楋紝杩欐牱褰?
  浣犵殑椹卞姩澶勭悊鍙︿竴涓姹傜殑瀹屾垚鏃讹紝USB 纭欢浠嶈兘鍚戞煇涓鐐逛紶杈撴暟鎹€傝繖鑳?
  鏈€澶у寲 USB 甯﹀鐨勫埄鐢紝骞跺湪浣跨敤鍛ㄦ湡鎬т紶杈撴ā寮忔椂鏀寔涓庤澶囦箣闂存棤缂濈殑
  鏁版嵁娴佷紶杈撱€?


## URB 缁撴瀯


```

  struct urb
  {
  // (IN) device and pipe specify the endpoint queue
	struct usb_device *dev;         // pointer to associated USB device
	unsigned int pipe;              // endpoint information

	unsigned int transfer_flags;    // URB_ISO_ASAP, URB_SHORT_NOT_OK, etc.

  // (IN) all urbs need completion routines
	void *context;                  // context for completion routine
	usb_complete_t complete;        // pointer to completion routine

  // (OUT) status after each completion
	int status;                     // returned status

  // (IN) buffer used for data transfers
	void *transfer_buffer;          // associated data buffer
	u32 transfer_buffer_length;     // data buffer length
	int number_of_packets;          // size of iso_frame_desc

  // (OUT) sometimes only part of CTRL/BULK/INTR transfer_buffer is used
	u32 actual_length;              // actual data buffer length

  // (IN) setup stage for CTRL (pass a struct usb_ctrlrequest)
	unsigned char *setup_packet;    // setup packet (control only)

  // Only for PERIODIC transfers (ISO, INTERRUPT)
    // (IN/OUT) start_frame is set unless URB_ISO_ASAP isn't set
	int start_frame;                // start frame
	int interval;                   // polling interval

    // ISO only: packets are only "best effort"; each can have errors
	int error_count;                // number of errors
	struct usb_iso_packet_descriptor iso_frame_desc[0];
  };

```
浣犵殑椹卞姩蹇呴』浣跨敤瀹冩墍澹版槑鎺ュ彛涓浉搴旂鐐规弿杩扮鐨勫€兼潵鍒涘缓鈥減ipe鈥濆€笺€?


## 濡備綍鑾峰彇涓€涓?URB锛?


```

	struct urb *usb_alloc_urb(int isoframes, int mem_flags)

```
杩斿洖鍊兼槸鎵€鍒嗛厤 URB 鐨勬寚閽堬紝鑻ュ垎閰嶅け璐ュ垯涓?0銆傚弬鏁?isoframes 鎸囧畾浜嗕綘
鎯宠璋冨害鐨勭瓑鏃朵紶杈撳抚鐨勬暟閲忋€傚浜?CTRL/BULK/INT锛屼娇鐢?0銆俶em_flags 鍙傛暟
淇濆瓨鏍囧噯鐨勫瓨鍌ㄥ垎閰嶆爣蹇楋紝浣夸綘鑳芥帶鍒讹紙闄ゅ叾浠栧锛夊簳灞備唬鐮佹槸鍚﹀彲鑳介樆濉炪€?

```

	void usb_free_urb(struct urb *urb)

```
浣犲彲浠ラ噴鏀句竴涓凡缁忔彁浜ゃ€佷絾杩樻湭鍦?completion 鍥炶皟涓繑鍥炵粰浣犵殑 urb銆傚畠浼氬湪
涓嶅啀浣跨敤鏃惰鑷姩閲婃斁銆?


## 闇€瑕佸～鍏呭摢浜涘唴瀹癸紵


鏍规嵁浜嬪姟绫诲瀷鐨勪笉鍚岋紝`linux/usb.h` 涓畾涔変簡涓€浜涘唴鑱斿嚱鏁版潵绠€鍖栧垵濮嬪寲锛?
渚嬪 `usb_fill_control_urb`銆乣usb_fill_bulk_urb` 鍜?
`usb_fill_int_urb`銆備竴鑸€岃█锛屽畠浠渶瑕?usb 璁惧鎸囬拡銆乸ipe锛坲sb.h 涓?
鐨勫父鐢ㄦ牸寮忥級銆佷紶杈撶紦鍐插尯銆佹湡鏈涚殑浼犺緭闀垮害銆佸畬鎴愬鐞嗗嚱鏁板強鍏朵笂涓嬫枃銆傚彲浠?
鏌ョ湅涓€浜涘凡鏈夌殑椹卞姩鏉ヤ簡瑙ｅ畠浠殑鐢ㄦ硶銆?

鏍囧織锛?

- 瀵逛簬 ISO锛屾湁涓ょ鍚姩琛屼负锛氭寚瀹氱殑 start_frame 鎴?ASAP銆?
- 瀵逛簬 ASAP锛屽湪 transfer_flags 涓缃?`URB_ISO_ASAP`銆?

濡傛灉涓嶅厑璁哥煭鍖咃紝鍒欏湪 transfer_flags 涓缃?`URB_SHORT_NOT_OK`銆?


## 濡備綍鎻愪氦涓€涓?URB锛?


```

	int usb_submit_urb(struct urb *urb, int mem_flags)

```
`mem_flags` 鍙傛暟锛堝 `GFP_ATOMIC`锛夋帶鍒跺瓨鍌ㄥ垎閰嶏紝渚嬪褰撳唴瀛樼揣寮犳椂
搴曞眰鏄惁鍙兘闃诲銆?

瀹冧細绔嬪嵆杩斿洖锛岃繑鍥炵姸鎬佷负 0锛堣姹傚凡鍏ラ槦锛夋垨鏌愪釜閿欒鐮侊紝閫氬父鐢变互涓嬪師鍥?
寮曡捣锛?

- 鍐呭瓨涓嶈冻锛坄-ENOMEM`锛?
- 璁惧宸叉嫈鍑猴紙`-ENODEV`锛?
- 绔偣鍋滄粸锛坄-EPIPE`锛?
- 鎺掗槦鐨?ISO 浼犺緭杩囧锛坄-EAGAIN`锛?
- 璇锋眰鐨?ISO 甯ц繃澶氾紙`-EFBIG`锛?
- 鏃犳晥鐨?INT 闂撮殧锛坄-EINVAL`锛?
- INT 鐨勬暟鎹寘瓒呰繃涓€涓紙`-EINVAL`锛?

鎻愪氦鍚庯紝`urb->status` 涓?`-EINPROGRESS`锛涗絾鏄紝闄や簡鍦ㄤ綘鐨勫畬鎴愬洖璋冧腑锛?
浣犵粷涓嶅簲鏌ョ湅璇ュ€笺€?

瀵逛簬绛夋椂绔偣锛屼綘鐨勫畬鎴愬鐞嗗嚱鏁板簲浣跨敤澶氱紦鍐诧紝浠?`URB_ISO_ASAP` 鏍囧織鍚?
鍚屼竴绔偣锛堥噸鏂帮級鎻愪氦 URB锛屼互鑾峰緱鏃犵紳鐨?ISO 娴佷紶杈撱€?


## 濡備綍鍙栨秷涓€涓鍦ㄨ繍琛岀殑 URB锛?


鏈変袱绉嶆柟娉曞彲浠ュ彇娑堜綘宸茬粡鎻愪氦浣嗚繕鏈繑鍥炵粰浣犵殑椹卞姩鐨?URB銆傚浜庡紓姝ュ彇娑堬紝
璋冪敤
```

	int usb_unlink_urb(struct urb *urb)

```
瀹冧細鎶?urb 浠庡唴閮ㄥ垪琛ㄤ腑绉婚櫎锛屽苟閲婃斁鎵€鏈夊凡鍒嗛厤鐨勭‖浠舵弿杩扮銆傜姸鎬佷細琚?
淇敼涓哄弽鏄?unlink銆傛敞鎰?`usb_unlink_urb` 杩斿洖鏃?URB 閫氬父灏氭湭瀹屾垚锛涗綘
蹇呴』缁х画绛夊緟瀹屾垚澶勭悊鍑芥暟琚皟鐢ㄣ€?

```

	void usb_kill_urb(struct urb *urb)

```
瀹冨畬鎴?`usb_unlink_urb` 鎵€鍋氱殑鎵€鏈変簨鎯咃紝姝ゅ杩樹細绛夊緟 URB 宸茶繑鍥炰笖瀹屾垚
澶勭悊鍑芥暟宸叉墽琛屽畬姣曘€傚畠杩樺皢 URB 鏍囪涓烘殏鏃朵笉鍙敤锛岃繖鏍峰鏋滃畬鎴愬鐞嗗嚱鏁?
鎴栧叾浠栦换浣曚唬鐮佸皾璇曢噸鏂版彁浜ゅ畠锛屼細寰楀埌涓€涓?`-EPERM` 閿欒銆傚洜姝や綘鍙互纭俊锛?
褰?`usb_kill_urb` 杩斿洖鏃讹紝璇?URB 宸插畬鍏ㄧ┖闂层€?

鏈変竴涓敓鍛藉懆鏈熼棶棰橀渶瑕佽€冭檻銆備竴涓?URB 鍙兘鍦ㄤ换浣曟椂鍊欏畬鎴愶紝鑰屽畬鎴愬鐞?
鍑芥暟鍙兘浼氶噴鏀捐 URB銆傚鏋滆繖绉嶆儏鍐靛彂鐢熷湪 `usb_unlink_urb` 鎴?
`usb_kill_urb` 杩愯鏃讹紝灏嗗鑷村唴瀛樿闂繚瑙勩€傞┍鍔ㄦ湁璐ｄ换閬垮厤杩欑鎯呭喌锛?
閫氬父鎰忓懗鐫€闇€瑕佹煇绉嶉攣鏉ラ槻姝?URB 浠嶅湪浣跨敤鏃惰閲婃斁銆?

鍙︿竴鏂归潰锛岀敱浜?usb_unlink_urb 鍙兘鏈€缁堜細璋冪敤瀹屾垚澶勭悊鍑芥暟锛岃澶勭悊鍑芥暟
涓嶈兘鑾峰彇鍦ㄨ皟鐢?usb_unlink_urb 鏃舵墍鎸佹湁鐨勪换浣曢攣銆傝В鍐虫闂鐨勯€氱敤鏂规硶鏄?
鍦ㄦ寔鏈夐攣鏃跺鍔?URB 鐨勫紩鐢ㄨ鏁帮紝鐒跺悗閲婃斁閿佸苟璋冪敤 usb_unlink_urb 鎴?
usb_kill_urb锛屾渶鍚庡啀鍑忓皯 URB 鐨勫紩鐢ㄨ鏁般€備綘澧炲姞
```

	struct urb *usb_get_urb(struct urb *urb)

```
锛堝拷鐣ヨ繑鍥炲€硷紱瀹冧笌鍙傛暟鐩稿悓锛夊苟閫氳繃璋冪敤 `usb_free_urb` 鍑忓皯寮曠敤璁℃暟銆?
褰撶劧锛屽鏋滀笉瀛樺湪瀹屾垚澶勭悊鍑芥暟閲婃斁 URB 鐨勫嵄闄╋紝涓婅堪杩欎簺閮芥棤闇€杩涜銆?


## 鍏充簬瀹屾垚澶勭悊鍑芥暟锛?


```

	typedef void (*usb_complete_t)(struct urb *)

```
涔熷氨鏄锛屽畠鑾峰緱寮曞彂瀹屾垚璋冪敤鐨?URB銆傚湪瀹屾垚澶勭悊鍑芥暟涓紝浣犲簲璇ユ煡鐪?
`urb->status` 浠ユ娴嬩换浣?USB 閿欒銆傜敱浜?context 鍙傛暟鍖呭惈鍦?URB 涓紝浣?
鍙互鍚戝畬鎴愬鐞嗗嚱鏁颁紶閫掍俊鎭€?

娉ㄦ剰锛屽嵆浣挎姤鍛婁簡閿欒锛堟垨 unlink锛夛紝鏁版嵁涔熷彲鑳藉凡缁忚浼犺緭銆傝繖鏄洜涓?USB
浼犺緭鏄垎鍖呯殑锛涗紶杈撲綘鐨?1KByte 缂撳啿鍖哄彲鑳介渶瑕佸崄鍏釜鍖咃紝鑰屽湪瀹屾垚琚皟鐢?
涔嬪墠锛屽叾涓崄涓彲鑳藉凡缁忔垚鍔熶紶杈撱€?


   NEVER SLEEP IN A COMPLETION HANDLER.

   杩欎簺鍑芥暟缁忓父鍦ㄥ師瀛愪笂涓嬫枃琚皟鐢ㄣ€?

鍦ㄥ綋鍓嶅唴鏍镐腑锛屽畬鎴愬鐞嗗嚱鏁拌繍琛屾椂鏈湴涓柇鏄叧闂殑锛屼絾鏈潵杩欎竴鐐逛細鏀瑰彉锛?
鍥犳涓嶈鍋囪鏈湴 IRQ 鍦ㄥ畬鎴愬鐞嗗嚱鏁板唴閮ㄦ€绘槸琚鐢ㄣ€?

## 濡備綍杩涜绛夋椂锛圛SO锛変紶杈擄紵


闄や簡鎵归噺浼犺緭涓瓨鍦ㄧ殑瀛楁澶栵紝瀵逛簬 ISO锛屼綘杩橀渶瑕佽缃?`urb->interval`
浠ユ寚鏄庤繘琛屼紶杈撶殑棰戠巼锛涢€氬父姣忓抚涓€娆★紙瀵逛簬楂橀€熻澶囧垯鏄瘡寰抚涓€娆★級銆傚疄闄?
浣跨敤鐨勯棿闅斿皢鏄皬浜庣瓑浜庝綘鎵€鎸囧畾鍊肩殑涓€涓?2 鐨勫箓銆備綘鍙互浣跨敤
`usb_fill_int_urb` 瀹忔潵濉厖澶у鏁?ISO 浼犺緭瀛楁銆?

瀵逛簬 ISO 浼犺緭锛屼綘杩橀渶瑕佷负鎯宠璋冨害鐨勬瘡涓寘濉厖涓€涓?
`usb_iso_packet_descriptor` 缁撴瀯锛岃缁撴瀯鐢?`usb_alloc_urb` 鍒嗛厤鍦?
URB 鐨勬湯灏俱€?

`usb_submit_urb` 璋冪敤浼氭妸 `urb->interval` 淇敼涓哄皬浜庣瓑浜庢墍璇锋眰闂撮殧鍊?
鐨勫疄闄呭疄鐜伴棿闅斿€笺€傚鏋滀娇鐢ㄤ簡 `URB_ISO_ASAP` 璋冨害锛宍urb->start_frame`
涔熶細琚洿鏂般€?

瀵逛簬姣忎竴椤癸紝浣犲繀椤绘寚瀹氭甯х殑鏁版嵁鍋忕Щ锛堝熀鍧€涓?transfer_buffer锛夛紝浠ュ強浣?
鎯宠鍐欏叆/鏈熸湜璇诲彇鐨勯暱搴︺€傚畬鎴愬悗锛宎ctual_length 鍖呭惈瀹為檯浼犺緭鐨勯暱搴︼紝
status 鍖呭惈姝ゅ抚 ISO 浼犺緭鐨勭粨鏋滅姸鎬併€傚厑璁镐负涓嶅悓甯ф寚瀹氫笉鍚岀殑闀垮害锛堜緥濡傜敤浜?
闊抽鍚屾/鑷€傚簲浼犺緭閫熺巼锛夈€備綘涔熷彲浠ヤ娇鐢ㄩ暱搴?0 鏉ョ渷鐣ヤ竴涓垨澶氫釜甯?
锛坰triping锛夈€?

瀵逛簬璋冨害锛屼綘鍙互閫夋嫨鑷繁鐨勮捣濮嬪抚鎴?`URB_ISO_ASAP`銆傚鍓嶆墍杩帮紝濡傛灉浣犲缁?
鑷冲皯淇濇寔涓€涓?URB 鍦ㄩ槦鍒椾腑锛屽苟涓斾綘鐨勫畬鎴愬鐞嗗嚱鏁颁笉鏂紙閲嶆柊锛夋彁浜や竴涓洿鏅?
鐨?URB锛屼綘灏嗚幏寰楀钩婊戠殑 ISO 娴佷紶杈擄紙鍦?usb 甯﹀鍏佽鐨勬儏鍐典笅锛夈€?

濡傛灉浣犳寚瀹氳嚜宸辩殑璧峰甯э紝璇风‘淇濆畠姣斿綋鍓嶅抚鎻愬墠鑻ュ共甯с€傚鏋滀綘瑕佸皢 ISO 鏁版嵁
涓庢煇涓叾浠栦簨浠舵祦鍚屾锛屽彲鑳戒細闇€瑕佽繖绉嶆ā鍨嬨€?


## 濡備綍鍚姩涓柇锛圛NT锛変紶杈擄紵


涓柇浼犺緭涓庣瓑鏃朵紶杈撶被浼硷紝鏄懆鏈熸€х殑锛屽彂鐢熷湪 2 鐨勫箓锛?銆?銆? 绛夛級涓崟浣?
鐨勯棿闅斾笂銆傚崟浣嶅浜庡叏閫熷拰浣庨€熻澶囨槸甯э紝瀵逛簬楂橀€熻澶囨槸寰抚銆備綘鍙互浣跨敤
`usb_fill_int_urb` 瀹忔潵濉厖 INT 浼犺緭瀛楁銆?

`usb_submit_urb` 璋冪敤浼氭妸 `urb->interval` 淇敼涓哄皬浜庣瓑浜庢墍璇锋眰闂撮殧鍊?
鐨勫疄闄呭疄鐜伴棿闅斿€笺€?

鍦?Linux 2.6 涓紝涓庢棭鏈熺増鏈笉鍚岋紝涓柇 URB 鍦ㄥ畬鎴愭椂涓嶄細鑷姩閲嶅惎銆傚畠浠湪
瀹屾垚澶勭悊鍑芥暟琚皟鐢ㄦ椂缁撴潫锛屽氨鍍忓叾浠?URB 涓€鏍枫€傚鏋滀綘甯屾湜涓柇 URB 閲嶆柊鍚姩锛?
浣犵殑瀹屾垚澶勭悊鍑芥暟蹇呴』閲嶆柊鎻愪氦瀹冦€?