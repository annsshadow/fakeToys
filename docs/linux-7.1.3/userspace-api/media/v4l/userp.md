


######## Streaming I/O (User Pointers)


褰?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct `v4l2_capability` 鐨?`capabilities` 瀛楁涓殑 `V4L2_CAP_STREAMING` 鏍囧織琚缃椂锛岃緭鍏ュ拰杈撳嚭璁惧鏀寔杩欑 I/O 鏂规硶銆傝€岀壒瀹氱敤鎴锋寚閽堟柟娉曪紙涓嶄粎浠呮槸鍐呭瓨鏄犲皠锛夋槸鍚﹀彈鏀寔锛屽繀椤婚€氳繃璋冪敤 VIDIOC_REQBUFS ioctl 骞跺皢鍐呭瓨绫诲瀷璁剧疆涓?`V4L2_MEMORY_USERPTR` 鏉ョ‘瀹氥€?
杩欑 I/O 鏂规硶缁撳悎浜?read/write 鍜屽唴瀛樻槧灏勬柟娉曠殑浼樼偣銆傜紦鍐插尯锛坧lane锛夌敱搴旂敤绋嬪簭鑷韩鍒嗛厤锛屽苟涓斿彲浠ラ┗鐣欏湪渚嬪铏氭嫙鍐呭瓨鎴栧叡浜唴瀛樹腑銆傚彧浜ゆ崲鎸囧悜鏁版嵁鐨勬寚閽堬紝杩欎簺鎸囬拡鍜屽厓淇℃伅鍦?struct `v4l2_buffer`锛堟垨澶?plane API 鎯呭喌涓嬬殑 struct `v4l2_plane`锛変腑浼犻€掋€傚繀椤婚€氳繃璋冪敤 VIDIOC_REQBUFS 骞朵紶鍏ユ墍闇€鐨勭紦鍐插尯绫诲瀷锛屽皢椹卞姩鍒囨崲鍒扮敤鎴锋寚閽?I/O 妯″紡銆?浜嬪厛涓嶅垎閰嶄换浣曠紦鍐插尯锛坧lane锛夛紝鍥犳瀹冧滑涓嶈绱㈠紩锛屼篃涓嶈兘鍍忔槧灏勭紦鍐插尯閭ｆ牱閫氳繃 VIDIOC_QUERYBUF <VIDIOC_QUERYBUF> ioctl 鏌ヨ銆?
## Example: Initiating streaming I/O with user pointers



    struct v4l2_requestbuffers reqbuf;

    memset (&reqbuf, 0, sizeof (reqbuf));
    reqbuf.type = V4L2_BUF_TYPE_VIDEO_CAPTURE;
    reqbuf.memory = V4L2_MEMORY_USERPTR;

    if (ioctl (fd, VIDIOC_REQBUFS, &reqbuf) == -1) {
	if (errno == EINVAL)
	    printf ("Video capturing or user pointer streaming is not supported\\n");
	else
	    perror ("VIDIOC_REQBUFS");

	exit (EXIT_FAILURE);
    }

缂撳啿鍖猴紙plane锛夌殑鍦板潃鍜屽ぇ灏忓湪杩愯鏃堕€氳繃 VIDIOC_QBUF <VIDIOC_QBUF> ioctl 浼犻€掋€傚敖绠＄紦鍐插尯閫氬父琚惊鐜娇鐢紝浣嗗簲鐢ㄧ▼搴忓彲浠ュ湪姣忔 VIDIOC_QBUF <VIDIOC_QBUF> 璋冪敤鏃朵紶鍏ヤ笉鍚岀殑鍦板潃鍜屽ぇ灏忋€傚鏋滅‖浠舵湁闇€瑕侊紝椹卞姩浼氬湪鐗╃悊鍐呭瓨涓氦鎹㈠唴瀛橀〉锛屼互鍒涘缓涓€鍧楄繛缁殑鍐呭瓨鍖哄煙銆傝繖瀵瑰簲鐢ㄧ▼搴忔槸閫忔槑鐨勶紝鍙戠敓鍦ㄥ唴鏍哥殑铏氭嫙鍐呭瓨瀛愮郴缁熶腑銆傚綋缂撳啿鍖洪〉琚崲鍑哄埌纾佺洏鍚庯紝瀹冧滑浼氳鍙栧洖锛屽苟鏈€缁堣閿佸畾鍦ㄧ墿鐞嗗唴瀛樹腑浠ヤ緵 DMA 浣跨敤銆俒#f1]_

濉厖鎴栨樉绀哄畬姣曠殑缂撳啿鍖洪€氳繃 VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鍑洪槦銆傞┍鍔ㄥ彲浠ュ湪 DMA 瀹屾垚涓庢 ioctl 涔嬮棿鐨勪换浣曟椂鍊欒В閿佸唴瀛橀〉銆傚綋璋冪敤 VIDIOC_STREAMOFF <VIDIOC_STREAMON>銆乂IDIOC_REQBUFS锛屾垨璁惧琚叧闂椂锛屽唴瀛樹篃浼氳瑙ｉ攣銆傚簲鐢ㄧ▼搴忓繀椤绘敞鎰忥紝涓嶈鍦ㄧ紦鍐插尯鍑洪槦涔嬪墠灏卞皢鍏堕噴鏀俱€傞鍏堬紝缂撳啿鍖轰細琚攣瀹氭洿闀挎椂闂达紝娴垂鐗╃悊鍐呭瓨銆傚叾娆★紝褰撳唴瀛樿繑鍥炲埌搴旂敤绋嬪簭鐨勭┖闂插垪琛ㄥ苟琚殢鍚庣敤浜庡叾浠栫敤閫旀椂锛岄┍鍔ㄤ笉浼氭敹鍒伴€氱煡锛屽彲鑳戒細瀹屾垚鎵€璇锋眰鐨?DMA 骞惰鐩栨湁浠峰€肩殑鏁版嵁銆?
瀵逛簬閲囬泦锛坈apturing锛夊簲鐢紝閫氬父鐨勫仛娉曟槸鍏ラ槦鑻ュ共绌虹紦鍐插尯锛屽紑濮嬮噰闆嗗苟杩涘叆璇诲彇寰幆銆傚湪杩欓噷锛屽簲鐢ㄧ▼搴忕瓑寰呯洿鍒版湁宸插～鍏呯殑缂撳啿鍖哄彲浠ュ嚭闃燂紝骞跺湪鏁版嵁涓嶅啀闇€瑕佹椂閲嶆柊鍏ラ槦璇ョ紦鍐插尯銆傝緭鍑猴紙output锛夊簲鐢ㄥ垯濉厖骞跺叆闃熺紦鍐插尯锛屽綋鍫嗗彔浜嗚冻澶熷鐨勭紦鍐插尯鍚庡紑濮嬭緭鍑恒€傚湪鍐欏叆寰幆涓紝褰撳簲鐢ㄧ▼搴忕敤灏界┖闂茬紦鍐插尯鏃讹紝瀹冨繀椤荤瓑寰呯洿鍒版湁绌虹紦鍐插尯鍙互鍑洪槦骞惰閲嶇敤銆傚瓨鍦ㄤ袱绉嶆柟娉曟潵鎸傝捣搴旂敤绋嬪簭鐨勬墽琛岋紝鐩村埌涓€涓垨澶氫釜缂撳啿鍖哄彲浠ュ嚭闃熴€傞粯璁ゆ儏鍐典笅锛屽綋澶栧彂闃熷垪涓病鏈夌紦鍐插尯鏃?:ref:`VIDIOC_DQBUF <VIDIOC_QBUF>` 浼氶樆濉炪€傚綋 `open()` 鍑芥暟琚紶鍏ヤ簡 `O_NONBLOCK` 鏍囧織鏃讹紝褰撴病鏈夊彲鐢ㄧ紦鍐插尯鏃讹紝VIDIOC_DQBUF <VIDIOC_QBUF> 浼氱珛鍗宠繑鍥?`EAGAIN` 閿欒鐮併€?ref:`select() <func-select>` 鎴?`poll()` 鍑芥暟濮嬬粓鍙敤銆?
瑕佸惎鍔ㄥ拰鍋滄閲囬泦鎴栬緭鍑哄簲鐢紝璋冪敤 VIDIOC_STREAMON <VIDIOC_STREAMON> 鍜?VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctl銆?
   VIDIOC_STREAMOFF <VIDIOC_STREAMON> 浼氫綔涓哄壇浣滅敤浠庝袱涓槦鍒椾腑绉婚櫎鎵€鏈夌紦鍐插尯骞惰В閿佹墍鏈夌紦鍐插尯銆傜敱浜庡湪澶氫换鍔＄郴缁熶笂涓嶅瓨鍦?鐜板湪"鎵ц鏌愪簨鐨勮涔夛紝濡傛灉搴旂敤绋嬪簭闇€瑕佷笌鍏朵粬浜嬩欢鍚屾锛屽畠搴斿綋妫€鏌ユ墍閲囬泦鎴栬緭鍑虹紦鍐插尯鐨?struct `v4l2_buffer` `timestamp`銆?
瀹炵幇鐢ㄦ埛鎸囬拡 I/O 鐨勯┍鍔ㄥ繀椤绘敮鎸?VIDIOC_REQBUFS <VIDIOC_REQBUFS>銆乂IDIOC_QBUF <VIDIOC_QBUF>銆乂IDIOC_DQBUF <VIDIOC_QBUF>銆乂IDIOC_STREAMON <VIDIOC_STREAMON> 鍜?VIDIOC_STREAMOFF <VIDIOC_STREAMON> ioctl锛屼互鍙?`select()` 鍜?`poll()` 鍑芥暟銆俒#f2]_

   鎴戜滑鏈熸湜棰戠箒浣跨敤鐨勭紦鍐插尯閫氬父涓嶄細琚崲鍑恒€傛棤璁哄浣曪紝浜ゆ崲銆侀攣瀹氭垨鐢熸垚鍒嗘暎-鑱氶泦锛坰catter-gather锛夊垪琛ㄧ殑杩囩▼鍙兘寰堣€楁椂銆傝繖绉嶅欢杩熷彲浠ラ€氳繃杈撳叆缂撳啿鍖洪槦鍒楃殑娣卞害鏉ユ帺鐩栵紝鎴栬杩樺彲浠ラ€氳繃缁存姢缂撳瓨锛堝亣璁炬煇涓紦鍐插尯寰堝揩浼氬啀娆″叆闃燂級鏉ユ帺鐩栥€傚彟涓€鏂归潰锛屼负浜嗕紭鍖栧唴瀛樹娇鐢紝椹卞姩鍙互闄愬埗棰勫厛閿佸畾鐨勭紦鍐插尯鏁伴噺锛屽苟浼樺厛鍥炴敹鏈€杩戜娇鐢ㄧ殑缂撳啿鍖恒€傚綋鐒讹紝杈撳叆闃熷垪涓┖闂茬紦鍐插尯鐨勯〉涓嶉渶瑕佷繚瀛樺埌纾佺洏銆傝緭鍑虹紦鍐插尯蹇呴』鍦ㄨ緭鍏ュ拰杈撳嚭闃熷垪涓兘琚繚瀛橈紝鍥犱负搴旂敤绋嬪簭鍙兘涓庡叾浠栬繘绋嬪叡浜畠浠€?
   鍦ㄩ┍鍔ㄥ眰闈紝`select()` 鍜?`poll()` 鏄浉鍚岀殑锛岃€屼笖 `select()` 澶噸瑕佷簡锛屼笉鑳芥槸鍙€夐」銆傚叾浣欑殑搴斿綋鏄捐€屾槗瑙併€?