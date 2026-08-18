
### V4L2 鏂囦欢鍙ユ焺


struct v4l2_fh 鎻愪緵浜嗕竴绉嶇畝渚跨殑鏂瑰紡鏉ヤ繚瀛?V4L2 妗嗘灦鎵€浣跨敤鐨勩€佷笌鏂囦欢鍙ユ焺鐩稿叧鐨勭壒瀹氭暟鎹€?鍦ㄦ墍鏈夐┍鍔ㄤ腑閮藉繀椤讳娇鐢ㄥ畠銆?
struct v4l2_fh 鍦ㄩ┍鍔ㄧ殑 `open()` 鏂囦欢鎿嶄綔澶勭悊鍑芥暟涓垎閰嶃€傚畠閫氬父鍐呭祵浜庝竴涓洿澶х殑銆?椹卞姩鐗瑰畾鐨勭粨鏋勪腑銆俙v4l2_fh` 蹇呴』閫氳繃璋冪敤 `v4l2_fh_init` 杩涜鍒濆鍖栵紝
骞堕€氳繃 `v4l2_fh_add` 娣诲姞鍒?video 璁惧銆傝繖閫氳繃灏?`file->private_data`
璁句负鎸囧悜 `v4l2_fh` 鐨勬寚閽堬紝浠庤€屽皢 `v4l2_fh` 涓?`file` 鍏宠仈璧锋潵銆?
绫讳技鍦帮紝struct v4l2_fh 鍦ㄩ┍鍔ㄧ殑 `release()` 鏂囦欢鎿嶄綔澶勭悊鍑芥暟涓噴鏀俱€傚湪閲婃斁涔嬪墠锛?蹇呴』鍏堢敤 `v4l2_fh_del` 浠?video 璁惧绉婚櫎锛屽苟鐢?`v4l2_fh_exit` 娓呯悊銆?
椹卞姩涓嶅緱鐩存帴璁块棶 `file->private_data`銆傚畠浠彲浠ラ€氳繃璋冪敤 `file_to_v4l2_fh`
鑾峰彇涓?`file` 鍏宠仈鐨?`v4l2_fh`銆傞┍鍔ㄥ彲浠ヤ娇鐢?container_of 瀹忔彁鍙栧畠浠嚜宸辩殑鏂囦欢鍙ユ焺缁撴瀯銆?
绀轰緥锛?

	struct my_fh {
		int blah;
		struct v4l2_fh fh;
	};

	...

	int my_open(struct file *file)
	{
		struct my_fh *my_fh;
		struct video_device *vfd;
		int ret;

		...

		my_fh = kzalloc(sizeof(*my_fh), GFP_KERNEL);

		...

		v4l2_fh_init(&my_fh->fh, vfd);

		...

		v4l2_fh_add(&my_fh->fh, file);
		return 0;
	}

	int my_release(struct file *file)
	{
		struct v4l2_fh *fh = file_to_v4l2_fh(file);
		struct my_fh *my_fh = container_of(fh, struct my_fh, fh);

		...
		v4l2_fh_del(&my_fh->fh, file);
		v4l2_fh_exit(&my_fh->fh);
		kfree(my_fh);
		return 0;
	}

涓嬮潰绠€瑕佹弿杩版墍浣跨敤鐨?`v4l2_fh` 鍑芥暟锛?
`v4l2_fh_init <v4l2_fh_init>`
锛坄fh <v4l2_fh>`, `vdev <video_device>`锛?
- 鍒濆鍖栨枃浠跺彞鏌勩€傝繖**蹇呴』**鍦ㄩ┍鍔ㄧ殑 `v4l2_file_operations`->open() 澶勭悊鍑芥暟涓墽琛屻€?
`v4l2_fh_add <v4l2_fh_add>`
锛坄fh <v4l2_fh>`, struct file \*filp锛?
- 灏嗕竴涓?`v4l2_fh` 娣诲姞鍒?`video_device` 鐨勬枃浠跺彞鏌勫垪琛ㄣ€?  蹇呴』鍦ㄦ枃浠跺彞鏌勫畬鍏ㄥ垵濮嬪寲鍚庤皟鐢ㄣ€?
`v4l2_fh_del <v4l2_fh_del>`
锛坄fh <v4l2_fh>`, struct file \*filp锛?
- 瑙ｉ櫎鏂囦欢鍙ユ焺涓?`video_device` 鐨勫叧鑱斻€傜幇鍦ㄥ彲浠ヨ皟鐢ㄦ枃浠跺彞鏌勭殑閫€鍑哄嚱鏁般€?
`v4l2_fh_exit <v4l2_fh_exit>`
锛坄fh <v4l2_fh>`锛?
- 鍙嶅垵濮嬪寲鏂囦欢鍙ユ焺銆傚弽鍒濆鍖栦箣鍚庯紝`v4l2_fh` 鐨勫唴瀛樺彲琚噴鏀俱€?
`file_to_v4l2_fh <file_to_v4l2_fh>`
锛坰truct file \*filp锛?
- 鑾峰彇涓?`file` 鍏宠仈鐨?`v4l2_fh` 瀹炰緥銆?
濡傛灉 struct v4l2_fh 鏈鍐呭祵锛屽垯鍙互浣跨敤浠ヤ笅杈呭姪鍑芥暟锛?
`v4l2_fh_open <v4l2_fh_open>`
锛坰truct file \*filp锛?
- 璇ュ嚱鏁板垎閰嶄竴涓?struct v4l2_fh锛屽垵濮嬪寲瀹冿紝骞跺皢鍏舵坊鍔犲埌涓庤鏂囦欢缁撴瀯鍏宠仈鐨?  struct video_device銆?
`v4l2_fh_release <v4l2_fh_release>`
锛坰truct file \*filp锛?
- 璇ュ嚱鏁板皢鍏朵粠涓庢枃浠剁粨鏋勫叧鑱旂殑 struct video_device 涓垹闄わ紝鍙嶅垵濮嬪寲 `v4l2_fh`
  骞堕噴鏀惧畠銆?
杩欎袱涓嚱鏁板彲浠ユ彃鍏ュ埌 v4l2_file_operation 鐨?`open()` 涓?`release()` 鎿嶄綔涓€?
鑻ュ共椹卞姩闇€瑕佸湪绗竴涓枃浠跺彞鏌勮鎵撳紑浠ュ強鏈€鍚庝竴涓枃浠跺彞鏌勮鍏抽棴鏃舵墽琛屾煇浜涙搷浣溿€備负姝ゆ坊鍔犱簡
涓や釜杈呭姪鍑芥暟锛岀敤浜庢鏌?`v4l2_fh` 缁撴瀯鏄惁鏄叧鑱旇澶囪妭鐐瑰敮涓€鎵撳紑鐨勬枃浠跺彞鏌勶細

`v4l2_fh_is_singular <v4l2_fh_is_singular>`
锛坄fh <v4l2_fh>`锛?
- 濡傛灉鏂囦欢鍙ユ焺鏄敮涓€鐨勬墦寮€鏂囦欢鍙ユ焺鍒欒繑鍥?1锛屽惁鍒欒繑鍥?0銆?
`v4l2_fh_is_singular_file <v4l2_fh_is_singular_file>`
锛坰truct file \*filp锛?
- 鍚屼笂锛屼絾瀹冧互 filp->private_data 璋冪敤 v4l2_fh_is_singular銆?
##### V4L2 fh 鍑芥暟涓庢暟鎹粨鏋?