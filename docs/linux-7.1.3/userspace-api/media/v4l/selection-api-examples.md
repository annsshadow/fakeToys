
######## 绀轰緥

鏈〉鎻愪緵 V4L2 閫夋嫨锛坰election锛夋帴鍙ｇ殑浣跨敤绀轰緥锛岄€氳繃 C 浠ｇ爜鐗囨婕旂ず瑁佸壀锛坈rop锛変笌鍚堟垚锛坈ompose锛夊弬鏁扮殑鏌ヨ銆侀噸缃笌缂╂斁绛夋搷浣滐紝渚涘簲鐢ㄥ紑鍙戣€呭湪鐢ㄦ埛绌洪棿瀹炵幇瑙嗛鎹曡幏鎴栬緭鍑烘椂鍙傝€冦€?


锛堝亣瀹氫负瑙嗛鎹曡幏璁惧锛涘鍏朵粬璁惧璇锋洿鏀?`V4L2_BUF_TYPE_VIDEO_CAPTURE`锛?濡傞渶閰嶇疆鍚堟垚鍖哄煙锛岃灏嗙洰鏍囨敼涓?`V4L2_SEL_TGT_COMPOSE_*` 绯诲垪锛?
## 绀轰緥锛氶噸缃鍓弬鏁?


	struct v4l2_selection sel = {
	    .type = V4L2_BUF_TYPE_VIDEO_CAPTURE,
	    .target = V4L2_SEL_TGT_CROP_DEFAULT,
	};
	ret = ioctl(fd, VIDIOC_G_SELECTION, &sel);
	if (ret)
	    exit(-1);
	sel.target = V4L2_SEL_TGT_CROP;
	ret = ioctl(fd, VIDIOC_S_SELECTION, &sel);
	if (ret)
	    exit(-1);

鍦ㄦ樉绀哄櫒涓ぎ璁剧疆涓€涓緭鍑哄悎鎴愬尯鍩燂紝鍏跺ぇ灏?*鑷冲**涓洪檺鍒跺€肩殑涓€鍗娿€?
## 绀轰緥锛氱畝鍗曠缉灏?


	struct v4l2_selection sel = {
	    .type = V4L2_BUF_TYPE_VIDEO_OUTPUT,
	    .target = V4L2_SEL_TGT_COMPOSE_BOUNDS,
	};
	struct v4l2_rect r;

	ret = ioctl(fd, VIDIOC_G_SELECTION, &sel);
	if (ret)
	    exit(-1);
	/** 璁剧疆鏇村皬鐨勫悎鎴愮煩褰?**/
	r.width = sel.r.width / 2;
	r.height = sel.r.height / 2;
	r.left = sel.r.width / 4;
	r.top = sel.r.height / 4;
	sel.r = r;
	sel.target = V4L2_SEL_TGT_COMPOSE;
	sel.flags = V4L2_SEL_FLAG_LE;
	ret = ioctl(fd, VIDIOC_S_SELECTION, &sel);
	if (ret)
	    exit(-1);

鍋囧畾涓鸿棰戣緭鍑鸿澶囷紱瀵瑰叾浠栬澶囪鏇存敼 `V4L2_BUF_TYPE_VIDEO_OUTPUT`

## 绀轰緥锛氭煡璇㈢缉鏀惧洜瀛?


	struct v4l2_selection compose = {
	    .type = V4L2_BUF_TYPE_VIDEO_OUTPUT,
	    .target = V4L2_SEL_TGT_COMPOSE,
	};
	struct v4l2_selection crop = {
	    .type = V4L2_BUF_TYPE_VIDEO_OUTPUT,
	    .target = V4L2_SEL_TGT_CROP,
	};
	double hscale, vscale;

	ret = ioctl(fd, VIDIOC_G_SELECTION, &compose);
	if (ret)
	    exit(-1);
	ret = ioctl(fd, VIDIOC_G_SELECTION, &crop);
	if (ret)
	    exit(-1);

	/** 璁＄畻缂╂斁鍥犲瓙 **/
	hscale = (double)compose.r.width / crop.r.width;
	vscale = (double)compose.r.height / crop.r.height;
