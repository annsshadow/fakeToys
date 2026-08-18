
## Linux USB 瑙嗛绫伙紙UVC锛夐┍鍔?

鏈枃浠惰褰?UVC 椹卞姩涓竴浜涢┍鍔ㄧ壒鏈夌殑鍐呭锛屼緥濡傞┍鍔ㄤ笓鐢ㄧ殑 ioctl 浠ュ強瀹炵幇璇存槑銆?
闂鍜屾剰瑙佸彲浠ュ彂閫佸埌 Linux UVC 寮€鍙戦偖浠跺垪琛?linux-media@vger.kernel.org銆?

### 鎵╁睍鍗曞厓锛圶U锛夋敮鎸?

#### 绠€浠?

UVC 瑙勮寖鍏佽閫氳繃鎵╁睍鍗曞厓锛圶U锛夊疄鐜板巶鍟嗚嚜瀹氫箟鐨勬墿灞曘€侺inux UVC 椹卞姩閫氳繃涓ょ鐙珛鐨勬満鍒舵潵鏀寔鎵╁睍鍗曞厓鎺у埗锛圶U 鎺у埗锛夛細

  - 閫氳繃灏?XU 鎺у埗鏄犲皠鍒?V4L2 鎺у埗
  - 閫氳繃椹卞姩涓撶敤鐨?ioctl 鎺ュ彛

绗竴绉嶆満鍒跺厑璁搁€氱敤鐨?V4L2 搴旂敤绋嬪簭鍦ㄤ娇鐢ㄦ煇浜?XU 鎺у埗鏃讹紝灏嗗叾鏄犲皠鍒?V4L2 鎺у埗涓婏紝杩欎簺鎺у埗闅忓悗浼氬湪甯歌鐨勬帶鍒舵灇涓捐繃绋嬩腑鍑虹幇銆?
绗簩绉嶆満鍒堕渶瑕佸簲鐢ㄧ▼搴忓叿澶?uvcvideo 鐩稿叧鐨勪笓闂ㄧ煡璇嗘墠鑳借闂?XU 鎺у埗锛屼絾瀹冨皢鏁翠釜 UVC XU 姒傚康鏆撮湶缁欑敤鎴风┖闂达紝浠ヨ幏寰楁渶澶х殑鐏垫椿鎬с€?
杩欎袱绉嶆満鍒朵簰涓鸿ˉ鍏咃紝涓嬫枃灏嗗垎鍒缁嗕粙缁嶃€?

#### 鎺у埗鏄犲皠


UVC 椹卞姩涓虹敤鎴风┖闂村簲鐢ㄧ▼搴忔彁渚涗簡涓€绉嶅湪杩愯鏃跺畾涔夋墍璋撯€滄帶鍒舵槧灏勨€濈殑 API銆傝繖浜涙槧灏勫厑璁稿皢鍗曚釜 XU 鎺у埗鎴栧叾瀛楄妭鑼冨洿鏄犲皠鍒版柊鐨?V4L2 鎺у埗銆傝繖绫绘帶鍒剁殑琛ㄧ幇鍜岃涓轰笌鏅€?V4L2 鎺у埗锛堝嵆浜害銆佸姣斿害绛夋爣鍑嗘帶鍒讹級瀹屽叏涓€鑷淬€備笉杩囷紝瀵硅繖绫?V4L2 鎺у埗鐨勮鎴栧啓浼氳Е鍙戝鐩稿簲 XU 鎺у埗鐨勮鎴栧啓銆?
鐢ㄤ簬鍒涘缓杩欎簺鎺у埗鏄犲皠鐨?ioctl 鍚嶄负 UVCIOC_CTRL_MAP銆傛棭鏈熼┍鍔ㄧ増鏈紙0.2.0 涔嬪墠锛夐渶瑕佷簨鍏堜娇鐢ㄥ彟涓€涓?ioctl锛圲VCIOC_CTRL_ADD锛夊皢 XU 鎺у埗淇℃伅浼犻€掔粰 UVC 椹卞姩銆傝繖宸蹭笉鍐嶅繀瑕侊紝鍥犱负杈冩柊鐨?uvcvideo 鐗堟湰浼氱洿鎺ヤ粠璁惧鏌ヨ璇ヤ俊鎭€?
鍏充簬 UVCIOC_CTRL_MAP ioctl 鐨勮缁嗕俊鎭紝璇峰弬闃呬笅鏂団€淚OCTL 鍙傝€冣€濅竴鑺傘€?

3. 椹卞姩涓撶敤鐨?XU 鎺у埗鎺ュ彛

瀵逛簬闇€瑕佺洿鎺ヨ闂?XU 鎺у埗鐨勫簲鐢ㄧ▼搴忥紙渚嬪鍑轰簬娴嬭瘯銆佸浐浠朵笂浼犳垨璁块棶浜岃繘鍒舵帶鍒剁殑鐩殑锛夛紝鎻愪緵浜嗙浜岀璁块棶 XU 鎺у埗鐨勬満鍒讹紝鍏跺舰寮忎负椹卞姩涓撶敤鐨?ioctl锛屽嵆 UVCIOC_CTRL_QUERY銆?
瀵硅 ioctl 鐨勮皟鐢ㄥ厑璁稿簲鐢ㄧ▼搴忓悜 UVC 椹卞姩鍙戦€佹煡璇紝杩欎簺鏌ヨ浼氱洿鎺ユ槧灏勫埌搴曞眰鐨?UVC 鎺у埗璇锋眰銆?
涓轰簡鍙戣捣杩欐牱鐨勮姹傦紝闇€瑕佸厛鐭ラ亾璇ユ帶鍒剁殑鎵╁睍鍗曞厓 ID锛圲VC unit ID锛夊拰鎺у埗閫夋嫨瀛愶紙control selector锛夈€傝繖浜涗俊鎭涔堥渶瑕佸湪搴旂敤绋嬪簭涓‖缂栫爜锛岃涔堥渶瑕侀€氳繃鍏朵粬鏂瑰紡鏌ヨ锛屼緥濡傝В鏋?UVC 鎻忚堪绗︼紝鎴栬€呭湪鍙敤鐨勬儏鍐典笅浣跨敤濯掍綋鎺у埗鍣?API 鏉ユ灇涓捐澶囩殑瀹炰綋锛坋ntity锛夈€?
闄ら潪宸茬粡鐭ラ亾鎺у埗鐨勫ぇ灏忥紝鍚﹀垯鏈夊繀瑕佸厛鍙戣捣涓€涓?UVC_GET_LEN 璇锋眰锛屼互渚垮垎閰嶈冻澶熷ぇ鐨勭紦鍐插尯骞跺皢缂撳啿鍖哄ぇ灏忚缃负姝ｇ‘鐨勫€笺€傜被浼煎湴锛岃纭 UVC_GET_CUR 鎴?UVC_SET_CUR 鏄惁瀵规煇涓粰瀹氭帶鍒舵槸鏈夋晥鐨勮姹傦紝搴斿綋鍏堝彂璧蜂竴涓?UVC_GET_INFO 璇锋眰銆傜粨鏋滃瓧鑺傜殑绗?0 浣嶏紙鏀寔 GET锛夊拰绗?1 浣嶏紙鏀寔 SET锛夋寚绀哄摢浜涜姹傛槸鏈夋晥鐨勩€?
闅忕潃 UVCIOC_CTRL_QUERY ioctl 鐨勫姞鍏ワ紝UVCIOC_CTRL_GET 鍜?UVCIOC_CTRL_SET 杩欎袱涓?ioctl 宸茬粡杩囨椂锛屽洜涓哄畠浠殑鍔熻兘鍙槸鍓嶈€呭姛鑳界殑瀛愰泦銆傜洰鍓嶅畠浠粛琚敮鎸侊紝浣嗘垜浠紦鍔卞簲鐢ㄧ▼搴忓紑鍙戣€呮敼鐢?UVCIOC_CTRL_QUERY銆?
鍏充簬 UVCIOC_CTRL_QUERY ioctl 鐨勮缁嗕俊鎭紝璇峰弬闃呬笅鏂団€淚OCTL 鍙傝€冣€濅竴鑺傘€?

#### 瀹夊叏鎬?

璇?API 鐩墠涓嶆彁渚涚粏绮掑害鐨勮闂帶鍒舵満鍒躲€俇VCIOC_CTRL_ADD 鍜?UVCIOC_CTRL_MAP 杩欎袱涓?ioctl 闇€瑕佽秴绾х敤鎴锋潈闄愩€?
娆㈣繋鎻愬嚭鏀硅繘寤鸿銆?

#### 璋冭瘯


涓轰簡璋冭瘯涓?XU 鎺у埗鎴栦竴鑸帶鍒剁浉鍏崇殑闂锛屽缓璁湪妯″潡鍙傛暟 'trace' 涓惎鐢?UVC_TRACE_CONTROL 浣嶃€傝繖浼氫娇棰濆鐨勮緭鍑鸿鍐欏叆绯荤粺鏃ュ織銆?

#### IOCTL 鍙傝€?

##### UVCIOC_CTRL_MAP 鈥斺€?灏?UVC 鎺у埗鏄犲皠鍒?V4L2 鎺у埗


鍙傛暟锛歴truct uvc_xu_control_mapping

**鎻忚堪**锛?
	璇?ioctl 鍦?UVC 鎺у埗鎴栧叾涓€閮ㄥ垎涓庢煇涓?V4L2 鎺у埗涔嬮棿鍒涘缓鏄犲皠銆備竴鏃﹀畾涔夊ソ鏄犲皠锛岀敤鎴风┖闂村簲鐢ㄧ▼搴忓氨鍙互閫氳繃 V4L2 鎺у埗 API 璁块棶鍘傚晢鑷畾涔夌殑 UVC 鎺у埗銆?
	瑕佸垱寤烘槧灏勶紝搴旂敤绋嬪簭闇€瑕佺敤涓€涓凡缁忕敱 UVCIOC_CTRL_ADD 瀹氫箟鐨勭幇鏈?UVC 鎺у埗鐨勪俊鎭紝浠ュ強涓€涓柊鐨?V4L2 鎺у埗锛屾潵濉厖 uvc_xu_control_mapping 缁撴瀯浣撱€?
	涓€涓?UVC 鎺у埗鍙互鏄犲皠鍒板涓?V4L2 鎺у埗銆備緥濡傦紝涓€涓?UVC 骞崇Щ/鍊炬枩锛坧an/tilt锛夋帶鍒跺彲浠ヨ鏄犲皠鍒扮嫭绔嬬殑骞崇Щ鍜屽€炬枩 V4L2 鎺у埗銆俇VC 鎺у埗浣跨敤 'size' 鍜?'offset' 瀛楁琚垝鍒嗕负浜掍笉閲嶅彔鐨勫瓧娈碉紝鐒跺悗鍒嗗埆鏄犲皠鍒?V4L2 鎺у埗銆?
	瀵逛簬鏈夌鍙锋暣鏁扮殑 V4L2 鎺у埗锛宒ata_type 瀛楁搴旇涓?UVC_CTRL_DATA_TYPE_SIGNED銆傚叾浠栧彇鍊肩洰鍓嶈蹇界暐銆?
**杩斿洖鍊?*锛?
	鎴愬姛鏃惰繑鍥?0銆傚嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?errno銆?
	ENOMEM
		娌℃湁瓒冲鐨勫唴瀛樻潵鎵ц璇ユ搷浣溿€?	EPERM
		鏉冮檺涓嶈冻锛堥渶瑕佽秴绾х敤鎴锋潈闄愶級銆?	EINVAL
		涓嶅瓨鍦ㄨ繖鏍风殑 UVC 鎺у埗銆?	EOVERFLOW
		璇锋眰鐨?offset 鍜?size 浼氫娇 UVC 鎺у埗婧㈠嚭銆?	EEXIST
		鏄犲皠宸插瓨鍦ㄣ€?
**鏁版嵁绫诲瀷**锛?

 - struct uvc_xu_control_mapping

	__u32	id		V4L2 鎺у埗鏍囪瘑绗?	__u8	name[^32^]	V4L2 鎺у埗鍚嶇О
	__u8	entity[^16^]	UVC 鎵╁睍鍗曞厓 GUID
	__u8	selector	UVC 鎺у埗閫夋嫨瀛?	__u8	size		V4L2 鎺у埗澶у皬锛堜互浣嶄负鍗曚綅锛?	__u8	offset		V4L2 鎺у埗鍋忕Щ锛堜互浣嶄负鍗曚綅锛?	enum v4l2_ctrl_type
		v4l2_type	V4L2 鎺у埗绫诲瀷
	enum uvc_control_data_type
		data_type	UVC 鎺у埗鏁版嵁绫诲瀷
	struct uvc_menu_info
		*menu_info	鑿滃崟椤规暟缁勶紙浠呯敤浜庤彍鍗曞瀷鎺у埗锛?	__u32	menu_count	鑿滃崟椤规暟閲忥紙浠呯敤浜庤彍鍗曞瀷鎺у埗锛?
 - struct uvc_menu_info

	__u32	value		璁惧浣跨敤鐨勮彍鍗曢」鍊?	__u8	name[^32^]	鑿滃崟椤瑰悕绉?

 - enum uvc_control_data_type

	UVC_CTRL_DATA_TYPE_RAW		鍘熷鎺у埗锛堝瓧鑺傛暟缁勶級
	UVC_CTRL_DATA_TYPE_SIGNED	鏈夌鍙锋暣鏁?	UVC_CTRL_DATA_TYPE_UNSIGNED	鏃犵鍙锋暣鏁?	UVC_CTRL_DATA_TYPE_BOOLEAN	甯冨皵鍊?	UVC_CTRL_DATA_TYPE_ENUM		鏋氫妇
	UVC_CTRL_DATA_TYPE_BITMASK	浣嶆帺鐮?	UVC_CTRL_DATA_TYPE_RECT		鐭╁舰鍖哄煙


##### UVCIOC_CTRL_QUERY 鈥斺€?鏌ヨ涓€涓?UVC XU 鎺у埗


鍙傛暟锛歴truct uvc_xu_control_query

**鎻忚堪**锛?
	璇?ioctl 鏌ヨ涓€涓敱鍏舵墿灞曞崟鍏?ID 鍜屾帶鍒堕€夋嫨瀛愭爣璇嗙殑 UVC XU 鎺у埗銆?
	鏈夊绉嶄笉鍚岀殑鏌ヨ鍙敤锛屽畠浠笌 UVC 瑙勮寖涓弿杩扮殑搴曞眰鎺у埗璇锋眰绱у瘑瀵瑰簲銆傝繖浜涜姹傚寘鎷細

	UVC_GET_CUR
		鑾峰彇鎺у埗鐨勫綋鍓嶅€笺€?	UVC_GET_MIN
		鑾峰彇鎺у埗鐨勬渶灏忓€笺€?	UVC_GET_MAX
		鑾峰彇鎺у埗鐨勬渶澶у€笺€?	UVC_GET_DEF
		鑾峰彇鎺у埗鐨勯粯璁ゅ€笺€?	UVC_GET_RES
		鏌ヨ鎺у埗鐨勫垎杈ㄧ巼锛屽嵆鍏佽鐨勬帶鍒跺€肩殑姝ラ暱澶у皬銆?	UVC_GET_LEN
		鏌ヨ鎺у埗鐨勫ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?	UVC_GET_INFO
		鏌ヨ鎺у埗淇℃伅浣嶅浘锛屾寚绀烘槸鍚︽敮鎸?get/set 璇锋眰銆?	UVC_SET_CUR
		鏇存柊鎺у埗鐨勫€笺€?
	搴旂敤绋嬪簭蹇呴』灏?'size' 瀛楁璁剧疆涓鸿鎺у埗鐨勬纭暱搴︺€備緥澶栨儏鍐垫槸 UVC_GET_LEN 鍜?UVC_GET_INFO 鏌ヨ锛屽畠浠殑 size 蹇呴』鍒嗗埆璁句负 2 鍜?1銆?data' 瀛楁蹇呴』鎸囧悜涓€涓湁鏁堢殑銆佸彲鍐欑殑缂撳啿鍖猴紝涓旇冻澶熷ぇ浠ュ绾虫寚瀹氭暟閲忕殑鏁版嵁瀛楄妭銆?
	鏁版嵁鐩存帴浠庤澶囧鍒讹紝涓嶇粡杩囦换浣曢┍鍔ㄤ晶鐨勫鐞嗐€傚簲鐢ㄧ▼搴忚礋璐ｅ鏁版嵁缂撳啿鍖鸿繘琛屾牸寮忓寲锛屽寘鎷皬绔?澶х杞崲銆傝繖涓€鐐瑰浜?UVC_GET_LEN 璇锋眰鐨勭粨鏋滃挨鍏堕噸瑕侊紝璇ョ粨鏋滃缁堢敱璁惧浠ュ皬绔?16 浣嶆暣鏁扮殑褰㈠紡杩斿洖銆?
**杩斿洖鍊?*锛?
	鎴愬姛鏃惰繑鍥?0銆傚嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?errno銆?
	ENOENT
		璁惧涓嶆敮鎸佺粰瀹氱殑鎺у埗锛屾垨鑰呮壘涓嶅埌鎸囧畾鐨勬墿灞曞崟鍏冦€?	ENOBUFS
		鎸囧畾鐨勭紦鍐插尯澶у皬涓嶆纭紙杩囧ぇ鎴栬繃灏忥級銆?	EINVAL
		浼犲叆浜嗘棤鏁堢殑璇锋眰鐮併€?	EBADRQC
		缁欏畾鐨勬帶鍒朵笉鏀寔璇ヨ姹傘€?	EFAULT
		data 鎸囬拡寮曠敤浜嗕笉鍙闂殑鍐呭瓨鍖哄煙銆?
**鏁版嵁绫诲瀷**锛?

 - struct uvc_xu_control_query

	__u8	unit		鎵╁睍鍗曞厓 ID
	__u8	selector	鎺у埗閫夋嫨瀛?	__u8	query		瑕佸彂閫佺粰璁惧鐨勮姹傜爜
	__u16	size		鎺у埗鏁版嵁澶у皬锛堜互瀛楄妭涓哄崟浣嶏級
	__u8	*data		鎺у埗鍊?

### 椹卞姩涓撶敤鐨?V4L2 鎺у埗


uvcvideo 椹卞姩瀹炵幇浜嗕互涓?UVC 涓撶敤鐨勬帶鍒讹細

`V4L2_CID_UVC_REGION_OF_INTEREST_RECT (struct)`
	璇ユ帶鍒跺喅瀹氭劅鍏磋叮鍖哄煙锛圧OI锛夈€俁OI 鏄竴涓敱缁撴瀯浣?`v4l2_rect` 琛ㄧず鐨勭煩褰㈠尯鍩熴€傝鐭╁舰閲囩敤鍏ㄥ眬浼犳劅鍣ㄥ潗鏍囷紝浠ュ儚绱犱负鍗曚綅銆傚畠鐙珛浜庤鍦猴紙field of view锛夛紝涓嶅彈浠讳綍瑁佸壀鎴栫缉鏀剧殑褰卞搷銆?
	浣跨敤 `V4L2_CTRL_WHICH_MIN_VAL` 鍜?`V4L2_CTRL_WHICH_MAX_VAL` 鏉ユ煡璇㈢煩褰㈠ぇ灏忕殑鑼冨洿銆?
	璁剧疆涓€涓?ROI 鍙互璁╃浉鏈洪拡瀵硅鍖哄煙浼樺寲閲囬泦銆俙V4L2_CID_REGION_OF_INTEREST_AUTO` 鎺у埗鐨勫€煎喅瀹氫簡鍏蜂綋鐨勮涓恒€?
	璇ユ帶鍒剁殑浣跨敤绀轰緥鍙弬瑙侊細
	`Chrome OS USB camera HAL銆?	<https://chromium.googlesource.com/chromiumos/platform2/+/refs/heads/release-R121-15699.B/camera/hal/usb/>`


`V4L2_CID_UVC_REGION_OF_INTEREST_AUTO (bitmask)`
	璇ユ帶鍒跺喅瀹氬摢浜涳紙濡傛灉鏈夌殑璇濓級鏉胯浇鍔熻兘搴斿綋璺熻釜褰撳墠 `V4L2_CID_UVD__REGION_OF_INTEREST_RECT` 鍊兼墍鎸囧畾鐨勬劅鍏磋叮鍖哄煙銆?
	鏈€澶у€兼槸涓€涓寚绀烘墍鏈夊彈鏀寔鑷姩鎺у埗鐨勬帺鐮併€?
    :header-rows:  0
    :stub-columns: 0

    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_EXPOSURE`
      - 璁剧疆璇ヤ綅浼氫娇鑷姩鏇濆厜璺熻釜鎰熷叴瓒ｅ尯鍩燂紝鑰屼笉鏄暣骞呭浘鍍忋€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_IRIS`
      - 璁剧疆璇ヤ綅浼氫娇鑷姩鍏夊湀锛坕ris锛夎窡韪劅鍏磋叮鍖哄煙锛岃€屼笉鏄暣骞呭浘鍍忋€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_WHITE_BALANCE`
      - 璁剧疆璇ヤ綅浼氫娇鑷姩鐧藉钩琛¤窡韪劅鍏磋叮鍖哄煙锛岃€屼笉鏄暣骞呭浘鍍忋€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_FOCUS`
      - 璁剧疆璇ヤ綅浼氫娇鑷姩瀵圭劍璋冩暣璺熻釜鎰熷叴瓒ｅ尯鍩燂紝鑰屼笉鏄暣骞呭浘鍍忋€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_FACE_DETECT`
      - 璁剧疆璇ヤ綅浼氫娇鑷姩浜鸿劯妫€娴嬭窡韪劅鍏磋叮鍖哄煙锛岃€屼笉鏄暣骞呭浘鍍忋€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_DETECT_AND_TRACK`
      - 璁剧疆璇ヤ綅浼氬惎鐢ㄨ嚜鍔ㄤ汉鑴告娴嬩笌璺熻釜銆傞┍鍔ㄥ彲鑳戒細鏇存柊 `V4L2_CID_REGION_OF_INTEREST_RECT` 鐨勫綋鍓嶅€笺€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_IMAGE_STABILIZATION`
      - 璁剧疆璇ヤ綅浼氬惎鐢ㄨ嚜鍔ㄥ浘鍍忕ǔ瀹氥€傞┍鍔ㄥ彲鑳戒細鏇存柊 `V4L2_CID_REGION_OF_INTEREST_RECT` 鐨勫綋鍓嶅€笺€?    - - `V4L2_UVC_REGION_OF_INTEREST_AUTO_HIGHER_QUALITY`
      - 璁剧疆璇ヤ綅浼氬湪鍙兘鐨勬儏鍐典笅浠ユ洿楂樿川閲忚嚜鍔ㄩ噰闆嗘寚瀹氬尯鍩熴€?