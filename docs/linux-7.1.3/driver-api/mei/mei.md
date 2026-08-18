
## 绠€浠?


Intel Management Engine锛圛ntel ME锛岃嫳鐗瑰皵绠＄悊寮曟搸锛夋槸椹荤暀鍦ㄦ煇浜?Intel 鑺墖缁勫唴閮ㄧ殑涓€涓殧绂讳笖鍙椾繚鎶ょ殑璁＄畻璧勬簮锛堝崗澶勭悊鍣級銆侷ntel ME 涓鸿绠楁満/IT 绠＄悊涓庡畨鍏ㄧ壒鎬ф彁渚涙敮鎸併€傚疄闄呯殑鍔熻兘闆嗗悎鍙栧喅浜?Intel 鑺墖缁勭殑 SKU銆?

Intel Management Engine Interface锛圛ntel MEI锛屾鍓嶇О涓?HECI锛夋槸涓绘満涓?Intel ME 涔嬮棿鐨勬帴鍙ｃ€傝鎺ュ彛浣滀负 PCI 璁惧鏆撮湶缁欎富鏈猴紝瀹為檯涓婂彲鑳戒細鏆撮湶鍑哄涓?PCI 璁惧銆侷ntel MEI 椹卞姩璐熻矗涓绘満搴旂敤绋嬪簭涓?Intel ME 鐗规€т箣闂寸殑閫氫俊閫氶亾銆?

姣忎釜 Intel ME 鐗规€э紙鎴?Intel ME 瀹㈡埛绔級閮界敱涓€涓敮涓€鐨?GUID 瀵诲潃锛屽苟涓旀瘡涓鎴风閮芥湁鑷繁鐨勫崗璁€傝鍗忚鏄熀浜庢秷鎭殑锛屽甫鏈変竴涓ご閮ㄥ拰璐熻浇锛岃礋杞芥渶澶у瓧鑺傛暟鐢卞鎴风鍦ㄨ繛鎺ユ椂閫氬憡銆?

## Intel MEI 椹卞姩


璇ラ┍鍔ㄦ毚闇蹭竴涓瓧绗﹁澶囷紝鍏惰澶囪妭鐐逛负 /dev/meiX銆?

搴旂敤绋嬪簭鍦?/dev/meiX 澶勪簬鎵撳紑鐘舵€佹椂涓庢煇涓?Intel ME 鐗规€т繚鎸侀€氫俊銆備笌鐗瑰畾鐗规€х殑缁戝畾閫氳繃璋冪敤 `MEI_CONNECT_CLIENT_IOCTL` 瀹屾垚锛岃璋冪敤浼犲叆鏈熸湜鐨?GUID銆傚彲浠ュ悓鏃舵墦寮€鐨勬煇涓?Intel ME 鐗规€х殑瀹炰緥鏁伴噺鍙栧喅浜庤 Intel ME 鐗规€э紝浣嗗ぇ澶氭暟鐗规€у彧鍏佽鍗曚釜瀹炰緥銆?

璇ラ┍鍔ㄥ鍥轰欢鐗规€т笌涓绘満搴旂敤绋嬪簭涔嬮棿浼犻€掔殑鏁版嵁鏄€忔槑鐨勩€?

鐢变簬鏌愪簺 Intel ME 鐗规€у彲浠ユ敼鍙樼郴缁熼厤缃紝榛樿鎯呭喌涓嬭椹卞姩鍙厑璁哥壒鏉冪敤鎴疯闂畠銆?

浼氳瘽閫氳繃璋冪敤 :c`close(fd)` 缁堟銆?

涓€涓笌 Intel AMTHI 瀹㈡埛绔€氫俊鐨勫簲鐢ㄧ▼搴忕殑浠ｇ爜鐗囨锛?

涓轰簡鏀寔铏氭嫙鍖栨垨娌欑鍖栵紝鍙椾俊浠荤殑鐩戠绋嬪簭鍙互浣跨敤 `MEI_CONNECT_CLIENT_IOCTL_VTAG` 鏉ヤ笌鏌愪釜 Intel ME 鐗规€у垱寤鸿櫄鎷熼€氶亾銆傚苟闈炴墍鏈夌壒鎬ч兘鏀寔铏氭嫙閫氶亾锛岃繖鏍风殑瀹㈡埛绔細鍥炵瓟 EOPNOTSUPP銆?


	struct mei_connect_client_data data;
	fd = open(MEI_DEVICE);

	data.d.in_client_uuid = AMTHI_GUID;

	ioctl(fd, IOCTL_MEI_CONNECT_CLIENT, &data);

	printf("Ver=%d, MaxLen=%ld\n",
	       data.d.in_client_uuid.protocol_version,
	       data.d.in_client_uuid.max_msg_length);

	[...]

	write(fd, amthi_req_data, amthi_req_data_len);

	[...]

	read(fd, &amthi_res_data, amthi_res_data_len);

	[...]
	close(fd);


鐢ㄦ埛绌洪棿 API

## IOCTL锛?


Intel MEI 椹卞姩鏀寔浠ヤ笅 IOCTL 鍛戒护锛?

### IOCTL_MEI_CONNECT_CLIENT


杩炴帴鍒板浐浠剁壒鎬?瀹㈡埛绔€?


	Usage:

        struct mei_connect_client_data client_data;

        ioctl(fd, IOCTL_MEI_CONNECT_CLIENT, &client_data);

	Inputs:

        struct mei_connect_client_data - 鍖呭惈浠ヤ笅鍐呭
	Input field:

		in_client_uuid -	闇€瑕佽繛鎺ュ埌鐨?FW 鐗规€х殑 GUID銆?
         Outputs:
		out_client_properties - 瀹㈡埛绔睘鎬э細MTU 涓庡崗璁増鏈€?

         Error returns:

                ENOTTY  娌℃湁杩欐牱鐨勫鎴风锛堝嵆閿欒鐨?GUID锛夋垨杩炴帴涓嶈鍏佽銆?
		EINVAL	閿欒鐨?IOCTL 缂栧彿
		ENODEV	璁惧鎴栬繛鎺ユ湭鍒濆鍖栨垨灏氭湭灏辩华銆?
		ENOMEM	鏃犳硶涓哄鎴风鍐呴儴鏁版嵁鍒嗛厤鍐呭瓨銆?
		EFAULT	鑷村懡閿欒锛堜緥濡傛棤娉曡闂敤鎴疯緭鍏ユ暟鎹級
		EBUSY	杩炴帴宸茬粡鎵撳紑

:Note:
        max_msg_length锛圡TU锛夊湪瀹㈡埛绔睘鎬т腑鎻忚堪浜嗗彲浠ュ彂閫佹垨鎺ユ敹鐨勬渶澶ф暟鎹€傦紙渚嬪锛屽鏋?MTU=2K锛屽垯鍙互鍙戦€佹渶澶?2k 瀛楄妭鐨勮姹傦紝骞舵帴鏀舵渶澶?2k 瀛楄妭鐨勫搷搴旓級銆?

### IOCTL_MEI_CONNECT_CLIENT_VTAG锛?



        Usage:

        struct mei_connect_client_data_vtag client_data_vtag;

        ioctl(fd, IOCTL_MEI_CONNECT_CLIENT_VTAG, &client_data_vtag);

        Inputs:

        struct mei_connect_client_data_vtag - 鍖呭惈浠ヤ笅鍐呭
        Input field:

                in_client_uuid -  闇€瑕佽繛鎺ュ埌鐨?FW 鐗规€х殑 GUID銆?
                vtag - 铏氭嫙鏍囩 [1, 255]

         Outputs:
                out_client_properties - 瀹㈡埛绔睘鎬э細MTU 涓庡崗璁増鏈€?

         Error returns:

                ENOTTY 娌℃湁杩欐牱鐨勫鎴风锛堝嵆閿欒鐨?GUID锛夋垨杩炴帴涓嶈鍏佽銆?
                EINVAL 閿欒鐨?IOCTL 缂栧彿鎴?tag == 0
                ENODEV 璁惧鎴栬繛鎺ユ湭鍒濆鍖栨垨灏氭湭灏辩华銆?
                ENOMEM 鏃犳硶涓哄鎴风鍐呴儴鏁版嵁鍒嗛厤鍐呭瓨銆?
                EFAULT 鑷村懡閿欒锛堜緥濡傛棤娉曡闂敤鎴疯緭鍏ユ暟鎹級
                EBUSY  杩炴帴宸茬粡鎵撳紑
                EOPNOTSUPP 涓嶆敮鎸?Vtag

### IOCTL_MEI_NOTIFY_SET


鍚敤鎴栫鐢ㄤ簨浠堕€氱煡銆?



	Usage:

		uint32_t enable;

		ioctl(fd, IOCTL_MEI_NOTIFY_SET, &enable);


		uint32_t enable = 1;
		or
		uint32_t enable[disable] = 0;

	Error returns:


		EINVAL	閿欒鐨?IOCTL 缂栧彿
		ENODEV	璁惧鏈垵濮嬪寲鎴栧鎴风鏈繛鎺?
		ENOMEM	鏃犳硶涓哄鎴风鍐呴儴鏁版嵁鍒嗛厤鍐呭瓨銆?
		EFAULT	鑷村懡閿欒锛堜緥濡傛棤娉曡闂敤鎴疯緭鍏ユ暟鎹級
		EOPNOTSUPP 濡傛灉璁惧涓嶆敮鎸佽鐗规€?

:Note:
	瀹㈡埛绔繀椤诲凡杩炴帴鎵嶈兘鍚敤閫氱煡浜嬩欢


### IOCTL_MEI_NOTIFY_GET


妫€绱簨浠?



	Usage:
		uint32_t event;
		ioctl(fd, IOCTL_MEI_NOTIFY_GET, &event);

	Outputs:
		1 - 濡傛灉鏈変簨浠跺緟澶勭悊
		0 - 濡傛灉娌℃湁浜嬩欢寰呭鐞?

	Error returns:
		EINVAL	閿欒鐨?IOCTL 缂栧彿
		ENODEV	璁惧鏈垵濮嬪寲鎴栧鎴风鏈繛鎺?
		ENOMEM	鏃犳硶涓哄鎴风鍐呴儴鏁版嵁鍒嗛厤鍐呭瓨銆?
		EFAULT	鑷村懡閿欒锛堜緥濡傛棤娉曡闂敤鎴疯緭鍏ユ暟鎹級
		EOPNOTSUPP 濡傛灉璁惧涓嶆敮鎸佽鐗规€?

:Note:
	瀹㈡埛绔繀椤诲凡杩炴帴锛屽苟涓斿繀椤诲凡鍚敤浜嬩欢閫氱煡锛屾墠鑳芥帴鏀朵簨浠?



## 鏀寔鐨勮姱鐗囩粍


82X38/X48 Express 鍙婃洿鏂扮殑鍨嬪彿

linux-mei@linux.intel.com
