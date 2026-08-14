
## 瑙嗛璁惧鐨勫唴閮ㄨ〃绀?

`/dev` 鐩綍涓殑瀹為檯璁惧鑺傜偣鏄娇鐢?`video_device` 缁撴瀯浣擄紙`v4l2-dev.h`锛?鍒涘缓鐨勩€傝缁撴瀯浣撴棦鍙互鍔ㄦ€佸垎閰嶏紝涔熷彲浠ュ唴宓屽埌鏇村ぇ鐨勭粨鏋勪綋涓€?
瑕佸姩鎬佸垎閰嶅畠锛屼娇鐢?`video_device_alloc`锛?

	struct video_device *vdev = video_device_alloc();

	if (vdev == NULL)
		return -ENOMEM;

	vdev->release = video_device_release;

濡傛灉浣犳妸瀹冨唴宓屽埌鏇村ぇ鐨勭粨鏋勪綋涓紝閭ｄ箞蹇呴』灏?`release()` 鍥炶皟璁剧疆涓轰綘
鑷繁鐨勫嚱鏁帮細


	struct video_device *vdev = &my_vdev->vdev;

	vdev->release = my_vdev_release;

蹇呴』璁剧疆 `release()` 鍥炶皟锛屽畠浼氬湪瑙嗛璁惧鐨勬渶鍚庝竴涓娇鐢ㄨ€呴€€鍑烘椂琚皟鐢ㄣ€?
榛樿鐨?`video_device_release` 鍥炶皟鐩墠鍙槸璋冪敤 `kfree` 鏉ラ噴鏀炬墍鍒嗛厤鐨?鍐呭瓨銆?
杩樻湁涓€涓?`video_device_release_empty` 鍑芥暟锛屽畠浠€涔堜篃涓嶅仛锛堜负绌猴級锛屽綋缁撴瀯浣?琚唴宓屻€佷笖閲婃斁鏃舵棤浜嬪彲鍋氭椂搴斿綋浣跨敤瀹冦€?
浣犺繕搴斿綋璁剧疆 `video_device` 鐨勪互涓嬪瓧娈碉細

- `video_device`->v4l2_dev锛氬繀椤昏缃负鐖惰澶?`v4l2_device`銆?
- `video_device`->name锛氳缃负鏈夋弿杩版€т笖鍞竴鐨勫€笺€?
- `video_device`->vfl_dir锛氬浜庨噰闆嗚澶囷紙capture锛夎缃负
  `VFL_DIR_RX`锛坄VFL_DIR_RX` 鐨勫€间负 0锛屾墍浠ヨ繖閫氬父宸茬粡鏄粯璁ゅ€硷級锛?  瀵逛簬杈撳嚭璁惧璁剧疆涓?`VFL_DIR_TX`锛屽浜?mem2mem锛堢紪瑙ｇ爜锛夎澶囪缃负
  `VFL_DIR_M2M`銆?
- `video_device`->fops锛氳缃负 `v4l2_file_operations` 缁撴瀯浣撱€?
- `video_device`->ioctl_ops锛氬鏋滀綘浣跨敤 `v4l2_ioctl_ops`
  鏉ョ畝鍖?ioctl 鐨勭淮鎶わ紙寮虹儓寤鸿浣跨敤锛屽苟涓斿皢鏉ュ彲鑳藉彉涓哄己鍒惰姹傦紒锛夛紝
  鍒欏皢鍏惰缃负浣犵殑 `v4l2_ioctl_ops` 缁撴瀯浣撱€俙video_device`->vfl_type 鍜?  `video_device`->vfl_dir 瀛楁鐢ㄤ簬绂佺敤涓庣被鍨?鏂瑰悜缁勫悎涓嶅尮閰嶇殑鎿嶄綔銆?  渚嬪锛岄潪 VBI 鑺傜偣浼氱鐢?VBI 鎿嶄綔锛岄噰闆嗚澶囦細绂佺敤杈撳嚭鎿嶄綔銆傝繖鏍峰氨鏈夊彲鑳?  浠呬负 vbi 鍜?video 鑺傜偣鎻愪緵鍚屼竴涓?`v4l2_ioctl_ops` 缁撴瀯浣撱€?
- `video_device`->lock锛氬鏋滀綘鎯冲湪椹卞姩涓畬鎴愭墍鏈夌殑鍔犻攣锛屽垯淇濈暀涓?  `NULL`銆傚惁鍒欎綘瑕佺粰瀹冧竴涓寚鍚?`mutex_lock` 缁撴瀯浣撶殑鎸囬拡锛屽湪
  `video_device`->unlocked_ioctl 鏂囦欢鎿嶄綔琚皟鐢ㄤ箣鍓嶏紝鏍稿績灞備細鑾峰彇璇ラ攣锛?  骞跺湪璋冪敤涔嬪悗閲婃斁瀹冦€傛洿澶氱粏鑺傝鍙傞槄涓嬩竴鑺傘€?
- `video_device`->queue锛氫竴涓寚鍚戜笌鏈澶囪妭鐐瑰叧鑱旂殑 struct vb2_queue
  鐨勬寚閽堛€傚鏋?queue 涓嶄负 `NULL`锛屼笖 queue->lock 涓嶄负 `NULL`锛岄偅涔堝浜?  鎺掗槦绫?ioctl锛坄VIDIOC_REQBUFS`銆乣CREATE_BUFS`銆乣QBUF`銆乣DQBUF`銆?  `QUERYBUF`銆乣PREPARE_BUF`銆乣STREAMON` 鍜?`STREAMOFF`锛夛紝浼氫娇鐢?  queue->lock 鑰岄潪涓婇潰鐨勯攣銆傝繖鏍?vb2 <vb2_framework> 鎺掗槦妗嗘灦灏辨棤闇€
  绛夊緟鍏朵粬 ioctl銆傝 queue 鎸囬拡涔熻 vb2 <vb2_framework> 杈呭姪鍑芥暟鐢ㄦ潵
  妫€鏌ユ帓闃熺殑褰掑睘锛堝嵆璋冪敤瀹冪殑鏂囦欢鍙ユ焺鏄惁琚厑璁告墽琛岃鎿嶄綔锛夈€?
- `video_device`->prio锛氳窡韪紭鍏堢骇銆傜敤浜庡疄鐜?`VIDIOC_G_PRIORITY`
  鍜?`VIDIOC_S_PRIORITY`銆傚鏋滀繚鐣欎负 `NULL`锛屽垯浼氫娇鐢?`v4l2_device`
  涓殑 struct v4l2_prio_state銆傚鏋滀綘鎯宠姣忎釜锛堢粍锛夎澶囪妭鐐规嫢鏈夌嫭绔嬬殑
  浼樺厛绾х姸鎬侊紝閭ｄ箞鍙互灏嗗叾鎸囧悜浣犺嚜宸辩殑 struct `v4l2_prio_state`銆?
- `video_device`->dev_parent锛氫粎褰?v4l2_device 浠?`NULL` 浣滀负鐖?  `device` 缁撴瀯浣撴敞鍐屾椂鎵嶈缃畠銆傝繖绉嶆儏鍐靛彧鍑虹幇鍦ㄤ竴涓‖浠惰澶囨嫢鏈夊涓?  鍏变韩鍚屼竴涓?`v4l2_device` 鏍稿績鐨?PCI 璁惧鏃躲€?
  cx88 椹卞姩灏辨槸涓€涓緥瀛愶細涓€涓牳蹇?`v4l2_device` 缁撴瀯浣擄紝浣嗚涓€涓師濮嬭棰?  PCI 璁惧锛坈x8800锛夊拰涓€涓?MPEG PCI 璁惧锛坈x8802锛夊叡鍚屼娇鐢ㄣ€傜敱浜?  `v4l2_device` 涓嶈兘鍚屾椂鍏宠仈涓や釜 PCI 璁惧锛屽畠鍦ㄥ缓绔嬫椂鏈缃埗璁惧銆?  浣嗗湪鍒濆鍖?struct video_device 鏃朵綘**纭疄**鐭ラ亾璇ヤ娇鐢ㄥ摢涓埗 PCI 璁惧锛?  鍥犳浣犲皢 `dev_device` 璁剧疆涓烘纭殑 PCI 璁惧銆?
濡傛灉浣犱娇鐢?`v4l2_ioctl_ops`锛岄偅涔堝簲褰撳湪浣犵殑 `v4l2_file_operations`
缁撴瀯浣撲腑鎶?`video_device`->unlocked_ioctl 璁剧疆涓?`video_ioctl2`銆?
鍦ㄦ煇浜涙儏鍐典笅锛屼綘鎯冲憡鐭ユ牳蹇冿細浣犲湪 `v4l2_ioctl_ops` 涓寚瀹氱殑鏌愪釜鍑芥暟搴斿綋琚?蹇界暐銆備綘鍙互鍦ㄨ皟鐢?`video_register_device` 涔嬪墠閫氳繃璋冪敤浠ヤ笅鍑芥暟鏉ユ爣璁版绫?ioctl锛?
	`v4l2_disable_ioctl <v4l2_disable_ioctl>`
	(`vdev <video_device>`, cmd).

濡傛灉浣犲笇鏈涘熀浜庡閮ㄥ洜绱狅紙渚嬪鎵€浣跨敤鐨勫崱锛夊叧闂?`v4l2_ioctl_ops` 涓殑鏌愪簺
鐗规€э紝鑰屽張涓嶆兂鏂板缓涓€涓粨鏋勪綋锛岄€氬父灏遍渶瑕佽繖鏍峰仛銆?
`v4l2_file_operations` 缁撴瀯浣撴槸 file_operations 鐨勪竴涓瓙闆嗐€備富瑕佸尯鍒湪浜?鐪佺暐浜?inode 鍙傛暟锛屽洜涓哄畠浠庢湭琚娇鐢ㄣ€?
濡傛灉闇€瑕佷笌 media framework 闆嗘垚锛屼綘蹇呴』閫氳繃璋冪敤 `media_entity_pads_init`
鏉ュ垵濮嬪寲鍐呭祵鍦?`video_device` 缁撴瀯浣撲腑鐨?`media_entity` 缁撴瀯浣?锛坋ntity 瀛楁锛夛細


	struct media_pad *pad = &my_vdev->pad;
	int err;

	err = media_entity_pads_init(&vdev->entity, 1, pad);

pads 鏁扮粍蹇呴』浜嬪厛鍒濆鍖栧畬姣曘€傛棤闇€鎵嬪姩璁剧疆 struct media_entity 鐨?type 鍜?name 瀛楁銆?
褰撹棰戣澶囪鎵撳紑/鍏抽棴鏃讹紝瀵硅 entity 鐨勫紩鐢ㄤ細琚嚜鍔ㄨ幏鍙?閲婃斁銆?
### ioctls 涓庡姞閿?

V4L 鏍稿績鎻愪緵鍙€夌殑鍔犻攣鏈嶅姟銆備富瑕佺殑鏈嶅姟鏄?struct video_device 涓殑 lock
瀛楁锛屽畠鏄竴涓寚鍚戜簰鏂ヤ綋鐨勬寚閽堛€傚鏋滀綘璁剧疆浜嗚鎸囬拡锛岄偅涔?unlocked_ioctl
灏嗕娇鐢ㄥ畠鏉ヤ覆琛屽寲鎵€鏈?ioctl銆?
濡傛灉浣犱娇鐢ㄧ殑鏄?videobuf2 妗嗘灦 <vb2_framework>锛岄偅涔堣繕鍙互璁剧疆绗簩涓攣锛?`video_device`->queue->lock銆傚鏋滆缃簡瀹冿紝閭ｄ箞瀵逛簬鎵€鏈夋帓闃熺被 ioctl
锛堝畬鏁村垪琛ㄨ涓婁竴鑺傦級锛屽皢浣跨敤璇ラ攣鑰岄潪 `video_device`->lock 鏉ヤ覆琛屽寲銆?
瀵规帓闃熺被 ioctl 浣跨敤涓嶅悓閿佺殑濂藉鍦ㄤ簬锛屽浜庢煇浜涢┍鍔紙灏ゅ叾鏄?USB 椹卞姩锛夛紝
鏌愪簺鍛戒护锛堜緥濡傝缃帶鍒堕」锛夊彲鑳借€楁椂杈冮暱锛屽洜姝や綘甯屾湜瀵圭紦鍐插尯鎺掗槦绫?ioctl
浣跨敤鐙珛鐨勯攣銆傝繖鏍蜂綘鐨?`VIDIOC_DQBUF` 灏变笉浼氬洜涓洪┍鍔ㄦ蹇欎簬鏇存敼锛堜緥濡傦級
鎽勫儚澶存洕鍏夊弬鏁拌€屽仠婊炪€?
褰撶劧锛屼綘涔熷彲浠ュ缁堝皢閭ｄ袱涓攣鎸囬拡閮戒繚鐣欎负 `NULL`锛岃嚜琛屽畬鎴愭墍鏈夌殑鍔犻攣銆?
鍦ㄤ娇鐢?videobuf2 <vb2_framework> 鐨勬儏鍐典笅锛屼綘蹇呴』灏?`queue->lock`
鎸囬拡璁剧疆涓轰綘鐢ㄤ簬涓茶鍖栨帓闃熺被 ioctl 鐨勯攣銆傝繖鑳界‘淇濆湪 `VIDIOC_DQBUF`
绛夊緟缂撳啿鍖哄埌杈炬椂璇ラ攣琚噴鏀撅紝骞跺湪涔嬪悗閲嶆柊鑾峰彇銆?
鐑彃鎷旀柇寮€鐨勫疄鐜颁篃搴斿綋鍦ㄨ皟鐢?v4l2_device_disconnect 涔嬪墠鑾峰彇
`video_device` 涓婄殑閿併€傚鏋滀綘杩樹娇鐢ㄤ簡 `video_device`->queue->lock锛岄偅涔?蹇呴』鍏堥攣瀹?`video_device`->queue->lock锛屽啀閿佸畾 `video_device`->lock銆?杩欐牱浣犲彲浠ョ‘淇濊皟鐢?`v4l2_device_disconnect` 鏃舵病鏈?ioctl 姝ｅ湪杩愯銆?
### 瑙嗛璁惧娉ㄥ唽


鎺ヤ笅鏉ワ紝浣犱娇鐢?`video_register_device` 娉ㄥ唽瑙嗛璁惧銆傝繖浼氫负浣犲垱寤哄瓧绗?璁惧銆?

	err = video_register_device(vdev, VFL_TYPE_VIDEO, -1);
	if (err) {
		video_device_release(vdev); /** or kfree(my_vdev); **/
		return err;
	}

濡傛灉 `v4l2_device` 鐖惰澶囨嫢鏈夐潪 `NULL` 鐨?mdev 瀛楁锛岄偅涔堣瑙嗛璁惧鐨?entity 浼氳嚜鍔ㄦ敞鍐屽埌 media 璁惧銆?
娉ㄥ唽鍝釜璁惧鍙栧喅浜?type 鍙傛暟銆傜幇鏈夌殑绫诲瀷濡備笅锛?
========================== ====================	 ==============================
`vfl_devnode_type` 璁惧鍚?	     鐢ㄩ€?========================== ====================	 ==============================
`VFL_TYPE_VIDEO`         `/dev/videoX`       鐢ㄤ簬瑙嗛杈撳叆/杈撳嚭璁惧
`VFL_TYPE_VBI`           `/dev/vbiX`         鐢ㄤ簬鍨傜洿娑堥殣鏁版嵁锛堝嵆瀛楀箷銆?					     鍥炬枃鐢佃锛?`VFL_TYPE_RADIO`         `/dev/radioX`       鐢ㄤ簬鏀堕煶鏈鸿皟璋愬櫒
`VFL_TYPE_SUBDEV`        `/dev/v4l-subdevX`  鐢ㄤ簬 V4L2 瀛愯澶?`VFL_TYPE_SDR`           `/dev/swradioX`     鐢ㄤ簬杞欢瀹氫箟鏃犵嚎鐢碉紙SDR锛?					     璋冭皭鍣?`VFL_TYPE_TOUCH`         `/dev/v4l-touchX`   鐢ㄤ簬瑙︽懜浼犳劅鍣?========================== ====================	 ==============================

鏈€鍚庝竴涓弬鏁拌浣犲彲浠ュ鎵€浣跨敤鐨勮澶囪妭鐐圭紪鍙凤紙鍗?`videoX` 涓殑 X锛夋柦鍔犱竴瀹?绋嬪害鐨勬帶鍒躲€傞€氬父浣犱細浼犲叆 -1锛岃 v4l2 妗嗘灦鎸戦€夌涓€涓┖闂茬紪鍙枫€備絾鏈夋椂鐢ㄦ埛
甯屾湜閫夋嫨鐗瑰畾鐨勮妭鐐圭紪鍙枫€傞┍鍔ㄩ€氬父鍏佽鐢ㄦ埛鍦ㄩ┍鍔ㄦā鍧楅€夐」涓寚瀹氱壒瀹氱殑璁惧鑺傜偣
缂栧彿銆傝缂栧彿闅忓悗琚紶缁欐鍑芥暟锛寁ideo_register_device 浼氬皾璇曢€夋嫨璇ヨ澶囪妭鐐?缂栧彿銆傚鏋滆缂栧彿宸茶鍗犵敤锛屽垯浼氶€夋嫨涓嬩竴涓┖闂茬殑璁惧鑺傜偣缂栧彿锛屽苟鍚戝唴鏍告棩蹇?鍙戦€佷竴鏉¤鍛娿€?
鍙︿竴涓娇鐢ㄥ満鏅槸锛氬鏋滈┍鍔ㄥ垱寤轰簡寰堝璁惧銆傛鏃舵妸涓嶅悓鐨勮棰戣澶囨斁鍦ㄤ笉鍚岀殑
鍖洪棿涓彲鑳戒細寰堟湁鐢ㄣ€備緥濡傦紝瑙嗛閲囬泦璁惧浠?0 寮€濮嬶紝瑙嗛杈撳嚭璁惧浠?16 寮€濮嬨€?鍥犳浣犲彲浠ヤ娇鐢ㄦ渶鍚庝竴涓弬鏁版潵鎸囧畾鏈€灏忕殑璁惧鑺傜偣缂栧彿锛寁4l2 妗嗘灦浼氬皾璇曟寫閫?绛変簬鎴栧ぇ浜庝綘鎵€浼犲叆鍊肩殑绗竴涓┖闂茬紪鍙枫€傚鏋滃け璐ワ紝鍒欏彧浼氭寫閫夌涓€涓┖闂茬紪鍙枫€?
鏃㈢劧鍦ㄨ繖绉嶆儏鍐典笅浣犲苟涓嶅叧蹇冩棤娉曢€夋嫨鎸囧畾璁惧鑺傜偣缂栧彿鐨勮鍛婏紝浣犲彲浠ユ敼涓鸿皟鐢?`video_register_device_no_warn` 鍑芥暟銆?
姣忓綋鍒涘缓璁惧鑺傜偣鏃讹紝涔熶細涓轰綘鍒涘缓涓€浜涘睘鎬с€傚鏋滀綘鏌ョ湅
`/sys/class/video4linux`锛屽氨鑳界湅鍒拌繖浜涜澶囥€傝繘鍏ヤ緥濡?`video0`锛屼綘浼氱湅鍒?'name'銆?dev_debug' 鍜?'index' 灞炴€с€?name' 灞炴€у氨鏄?video_device 缁撴瀯浣撶殑
'name' 瀛楁銆?dev_debug' 灞炴€у彲鐢ㄤ簬鍚敤鏍稿績璋冭瘯銆傛洿璇︾粏鐨勪俊鎭鍙傞槄涓嬩竴鑺傘€?
'index' 灞炴€ф槸璁惧鑺傜偣鐨勭储寮曪細姣忚皟鐢ㄤ竴娆?`video_register_device()`锛岀储寮曞氨
鍔?1銆備綘娉ㄥ唽鐨勭涓€涓棰戣澶囪妭鐐规€绘槸浠庣储寮?0 寮€濮嬨€?
鐢ㄦ埛鍙互璁剧疆鍒╃敤 index 灞炴€х殑 udev 瑙勫垯锛屼互鐢熸垚鑺卞摠鐨勮澶囧悕锛堜緥濡傜敤浜?MPEG
瑙嗛閲囬泦璁惧鑺傜偣鐨?'`mpegX`'锛夈€?
璁惧鎴愬姛娉ㄥ唽鍚庯紝浣犲彲浠ヤ娇鐢ㄤ互涓嬪瓧娈碉細

- `video_device`->vfl_type锛氫紶缁?`video_register_device` 鐨勮澶囩被鍨嬨€?- `video_device`->minor锛氭墍鍒嗛厤鐨勮澶囨璁惧鍙枫€?- `video_device`->num锛氳澶囪妭鐐圭紪鍙凤紙鍗?`videoX` 涓殑 X锛夈€?- `video_device`->index锛氳澶囩储寮曞彿銆?
濡傛灉娉ㄥ唽澶辫触锛岄偅涔堜綘闇€瑕佽皟鐢?`video_device_release` 鏉ラ噴鏀炬墍鍒嗛厤鐨?`video_device` 缁撴瀯浣擄紝鎴栬€呭鏋滆 `video_device` 鏄唴宓岀殑锛屽垯閲婃斁浣犺嚜宸辩殑
缁撴瀯浣撱€傚鏋滄敞鍐屽け璐ワ紝`vdev->release()` 鍥炶皟姘歌繙涓嶄細琚皟鐢紝浣犱篃涓嶅簲灏濊瘯
鍦ㄦ敞鍐屽け璐ョ殑鎯呭喌涓嬫敞閿€璇ヨ澶囥€?
### 瑙嗛璁惧璋冭瘯


涓烘瘡涓棰戙€乿bi銆乺adio 鎴?swradio 璁惧鍦?`/sys/class/video4linux/<devX>/`
涓嬪垱寤虹殑 'dev_debug' 灞炴€э紝鍙敤浜庡惎鐢ㄦ枃浠舵搷浣滅殑鏃ュ織銆?
瀹冩槸涓€涓綅鎺╃爜锛屽彲浠ヨ缃互涓嬩綅锛?
===== ================================================================
鎺╃爜  鎻忚堪
===== ================================================================
0x01  璁板綍 ioctl 鍚嶇О涓庨敊璇爜銆俈IDIOC_(D)QBUF ioctl 浠呭湪 0x08 浣嶄篃琚?      璁剧疆鏃舵墠浼氳璁板綍銆?0x02  璁板綍 ioctl 鍚嶇О鍙傛暟涓庨敊璇爜銆俈IDIOC_(D)QBUF ioctl 浠呭湪 0x08 浣?      涔熻璁剧疆鏃舵墠浼氳璁板綍銆?0x04  璁板綍鏂囦欢鎿嶄綔 open銆乺elease銆乺ead銆亀rite銆乵map 鍜?      get_unmapped_area銆俽ead 鍜?write 鎿嶄綔浠呭湪 0x08 浣嶈璁剧疆鏃?      鎵嶄細琚褰曘€?0x08  璁板綍 read 鍜?write 鏂囦欢鎿嶄綔锛屼互鍙?VIDIOC_QBUF 鍜?      VIDIOC_DQBUF ioctl銆?0x10  璁板綍 poll 鏂囦欢鎿嶄綔銆?0x20  璁板綍鎺у埗鎿嶄綔涓殑閿欒涓庢秷鎭€?===== ================================================================

### 瑙嗛璁惧娓呯悊


褰撳繀椤荤Щ闄よ棰戣澶囪妭鐐规椂锛堟棤璁烘槸椹卞姩鍗歌浇鏈熼棿锛岃繕鏄洜涓?USB 璁惧琚柇寮€锛夛紝
浣犲簲褰撲娇鐢ㄤ互涓嬫柟寮忔敞閿€瀹冧滑锛?
	`video_unregister_device`
	(`vdev <video_device>`);

杩欎細灏嗚澶囪妭鐐逛粠 sysfs 涓Щ闄わ紙瀵艰嚧 udev 灏嗗畠浠粠 `/dev` 涓Щ闄わ級銆?
`video_unregister_device` 杩斿洖鍚庯紝涓嶈兘鍐嶆墦寮€鏂扮殑璁惧銆備絾鏄紝瀵逛簬 USB 璁惧锛?鏌愪簺搴旂敤绋嬪簭鍙兘浠嶇劧鎵撳紑浜嗗叾涓竴涓澶囪妭鐐广€傚洜姝ゅ湪娉ㄩ攢涔嬪悗锛屾墍鏈夋枃浠舵搷浣?锛堝綋鐒讹紝release 闄ゅ锛変篃閮戒細杩斿洖閿欒銆?
褰撹棰戣澶囪妭鐐圭殑鏈€鍚庝竴涓娇鐢ㄨ€呴€€鍑烘椂锛屼細璋冪敤 `vdev->release()`
鍥炶皟锛屼綘鍙互鍦ㄩ偅閲岃繘琛屾渶缁堢殑娓呯悊銆?
濡傛灉宸插垵濮嬪寲锛屽埆蹇樹簡娓呯悊涓庤棰戣澶囧叧鑱旂殑 media entity锛?
	`media_entity_cleanup <media_entity_cleanup>`
	(&vdev->entity);

杩欏彲浠ヤ粠 release 鍥炶皟涓畬鎴愩€?

### 杈呭姪鍑芥暟


鏈変竴浜涙湁鐢ㄧ殑杈呭姪鍑芥暟锛?
- 鏂囦欢涓?`video_device` 绉佹湁鏁版嵁

浣犲彲浠ヤ娇鐢ㄤ互涓嬫柟寮忓湪 video_device 缁撴瀯浣撲腑璁剧疆/鑾峰彇椹卞姩绉佹湁鏁版嵁锛?
	`video_get_drvdata <video_get_drvdata>`
	(`vdev <video_device>`);

	`video_set_drvdata <video_set_drvdata>`
	(`vdev <video_device>`);

娉ㄦ剰锛屼綘鍙互鍦ㄨ皟鐢?`video_register_device` 涔嬪墠瀹夊叏鍦拌皟鐢?`video_set_drvdata`銆?
杩樻湁杩欎釜鍑芥暟锛?
	`video_devdata <video_devdata>`
	(struct file \*file);

杩斿洖灞炰簬璇?file 缁撴瀯浣撶殑 video_device銆?
`video_devdata` 鍑芥暟灏?`video_get_drvdata` 涓?`video_devdata` 缁撳悎璧锋潵锛?
	`video_drvdata <video_drvdata>`
	(struct file \*file);

浣犲彲浠ヤ娇鐢ㄤ互涓嬫柟寮忎粠 `video_device` 缁撴瀯浣撹浆鍒?v4l2_device 缁撴瀯浣擄細


	struct v4l2_device *v4l2_dev = vdev->v4l2_dev;

- 璁惧鑺傜偣鍚?
`video_device` 鑺傜偣鐨勫唴鏍稿悕鍙互浣跨敤浠ヤ笅鏂瑰紡鑾峰彇锛?
	`video_device_node_name <video_device_node_name>`
	(`vdev <video_device>`);

璇ュ悕绉拌 udev 绛夌敤鎴风┖闂村伐鍏风敤浣滄彁绀恒€傚簲褰撳敖鍙兘浣跨敤璇ュ嚱鏁帮紝鑰屼笉瑕佺洿鎺?璁块棶 video_device 鐨?**num** 涓?**video_device** 鐨?**minor** 瀛楁銆?
### video_device 鍑芥暟涓庢暟鎹粨鏋?