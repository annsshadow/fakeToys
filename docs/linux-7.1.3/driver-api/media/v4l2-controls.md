
## V4L2 鎺т欢

### 绠€浠?
V4L2 鎺т欢锛坈ontrol锛堿PI 鐪嬭捣鏉ヨ冻澶熺畝鍗曪紝浣嗚鍦ㄩ┍鍔ㄤ腑姝ｇ‘瀹炵幇鍗村緢蹇彉寰楅潪甯稿洶闅俱€?涓嶈繃澶勭悊鎺т欢鎵€闇€鐨勪唬鐮佸ぇ閮ㄥ垎鍏跺疄骞朵笉鐗瑰畾浜庢煇涓┍鍔紝鍙互绉诲埌 V4L 鏍稿績妗嗘灦涓€?
姣曠珶锛岄┍鍔ㄥ紑鍙戣€呭敮涓€鎰熷叴瓒ｇ殑閮ㄥ垎鏄細

1) 濡備綍娣诲姞涓€涓帶浠讹紵
2) 濡備綍璁剧疆鎺т欢鐨勫€硷紵锛堝嵆 s_ctrl锛?
鍋跺皵杩樹細鐢ㄥ埌锛?
3) 濡備綍鑾峰彇鎺т欢鐨勫€硷紵锛堝嵆 g_volatile_ctrl锛?4) 濡備綍鏍￠獙鐢ㄦ埛鎻愯鐨勬帶浠跺€硷紵锛堝嵆 try_ctrl锛?
鍏朵綑涓€鍒囬兘鍙互鍦ㄩ泦涓瀹屾垚銆?
鎺у埗妗嗘灦锛坈ontrol framework锛夌殑鍒涘缓锛屾槸涓轰簡鎶?V4L2 瑙勮寖涓叧浜庢帶浠剁殑鎵€鏈夎鍒欏湪
涓€涓泦涓殑鍦版柟瀹炵幇锛屽苟涓斿敖鍙兘璁╅┍鍔ㄥ紑鍙戣€呯殑宸ヤ綔鍙樺緱杞绘澗銆?
娉ㄦ剰锛屾帶鍒舵鏋朵緷璧?V4L2 椹卞姩鐨?`v4l2_device` 缁撴瀯浣擄紝浠ュ強瀛愯澶囷紙sub-device锛?椹卞姩鐨?v4l2_subdev 缁撴瀯浣撱€?
### 妗嗘灦涓殑瀵硅薄

鏈変袱涓富瑕佸璞★細

`v4l2_ctrl` 瀵硅薄鎻忚堪鎺т欢鐨勫睘鎬э紝骞惰窡韪帶浠剁殑鍊硷紙鍖呮嫭褰撳墠鍊煎拰鎻愯鐨勬柊鍊硷級銆?
`v4l2_ctrl_handler` 鏄窡韪帶浠剁殑瀵硅薄銆傚畠缁存姢涓€涓畠鎵€鎷ユ湁鐨?v4l2_ctrl 瀵硅薄鍒楄〃锛?浠ュ強鍙︿竴涓寚鍚戞帶浠剁殑寮曠敤鍒楄〃锛岃繖浜涙帶浠跺彲鑳界敱鍏跺畠澶勭悊鍣紙handler锛夋嫢鏈夈€?
### V4L2 鍜屽瓙璁惧椹卞姩鐨勫熀鏈敤娉?
1) 鍑嗗椹卞姩锛?
	#include <media/v4l2-ctrls.h>

1.1) 灏嗗鐞嗗櫒锛坔andler锛夋坊鍔犲埌椹卞姩鐨勯《灞傜粨鏋勪綋锛?
瀵逛簬 V4L2 椹卞姩锛?
	struct foo_dev {
		...
		struct v4l2_device v4l2_dev;
		...
		struct v4l2_ctrl_handler ctrl_handler;
		...
	};

瀵逛簬瀛愯澶囬┍鍔細

	struct foo_dev {
		...
		struct v4l2_subdev sd;
		...
		struct v4l2_ctrl_handler ctrl_handler;
		...
	};

1.2) 鍒濆鍖栧鐞嗗櫒锛坔andler锛夛細

	v4l2_ctrl_handler_init(&foo->ctrl_handler, nr_of_controls);

绗簩涓弬鏁版槸涓€涓彁绀猴紝鍛婅瘔璇ュ嚱鏁拌澶勭悊鍣ㄩ鏈熻澶勭悊澶氬皯涓帶浠躲€傚畠灏嗗熀浜庤淇℃伅
鍒嗛厤涓€涓搱甯岃〃銆傝繖浠呬粎鏄竴涓彁绀恒€?
1.3) 灏嗘帶鍒跺鐞嗗櫒锛坈ontrol handler锛夋寕鎺ュ埌椹卞姩锛?
瀵逛簬 V4L2 椹卞姩锛?
	foo->v4l2_dev.ctrl_handler = &foo->ctrl_handler;

瀵逛簬瀛愯澶囬┍鍔細

	foo->sd.ctrl_handler = &foo->ctrl_handler;

1.4) 鍦ㄦ渶鍚庢竻鐞嗗鐞嗗櫒锛坔andler锛夛細

	v4l2_ctrl_handler_free(&foo->ctrl_handler);

`v4l2_ctrl_handler_free` 涓嶄細瑙︾澶勭悊鍣ㄧ殑 `error` 瀛楁銆?
2) 娣诲姞鎺т欢锛?
閫氳繃璋冪敤 `v4l2_ctrl_new_std` 娣诲姞闈炶彍鍗曪紙non-menu锛夋帶浠讹細

	struct v4l2_ctrl **v4l2_ctrl_new_std(struct v4l2_ctrl_handler **hdl,
			const struct v4l2_ctrl_ops *ops,
			u32 id, s32 min, s32 max, u32 step, s32 def);

鑿滃崟锛坢enu锛夊拰鏁存暟鑿滃崟锛坕nteger menu锛夋帶浠堕€氳繃璋冪敤 `v4l2_ctrl_new_std_menu`
娣诲姞锛?
	struct v4l2_ctrl **v4l2_ctrl_new_std_menu(struct v4l2_ctrl_handler **hdl,
			const struct v4l2_ctrl_ops *ops,
			u32 id, s32 max, s32 skip_mask, s32 def);

甯︽湁椹卞姩鐗瑰畾鑿滃崟鐨勮彍鍗曟帶浠堕€氳繃璋冪敤 `v4l2_ctrl_new_std_menu_items` 娣诲姞锛?
       struct v4l2_ctrl *v4l2_ctrl_new_std_menu_items(
                       struct v4l2_ctrl_handler *hdl,
                       const struct v4l2_ctrl_ops *ops, u32 id, s32 max,
                       s32 skip_mask, s32 def, const char ** const **qmenu);

鏍囧噯澶嶅悎锛坈ompound锛夋帶浠跺彲浠ラ€氳繃璋冪敤 `v4l2_ctrl_new_std_compound` 娣诲姞锛?
       struct v4l2_ctrl **v4l2_ctrl_new_std_compound(struct v4l2_ctrl_handler **hdl,
                       const struct v4l2_ctrl_ops *ops, u32 id,
                       const union v4l2_ctrl_ptr p_def);

甯︽湁椹卞姩鐗瑰畾鑿滃崟鐨勬暣鏁拌彍鍗曟帶浠跺彲浠ラ€氳繃璋冪敤 `v4l2_ctrl_new_int_menu` 娣诲姞锛?
	struct v4l2_ctrl **v4l2_ctrl_new_int_menu(struct v4l2_ctrl_handler **hdl,
			const struct v4l2_ctrl_ops *ops,
			u32 id, s32 max, s32 def, const s64 *qmenu_int);

杩欎簺鍑芥暟閫氬父鍦?`v4l2_ctrl_handler_init` 涔嬪悗绔嬪嵆璋冪敤锛?
	static const s64 exp_bias_qmenu[] = {
	       -2, -1, 0, 1, 2
	};
	static const char * const test_pattern[] = {
		"Disabled",
		"Vertical Bars",
		"Solid Black",
		"Solid White",
	};

	v4l2_ctrl_handler_init(&foo->ctrl_handler, nr_of_controls);
	v4l2_ctrl_new_std(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_BRIGHTNESS, 0, 255, 1, 128);
	v4l2_ctrl_new_std(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_CONTRAST, 0, 255, 1, 128);
	v4l2_ctrl_new_std_menu(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_POWER_LINE_FREQUENCY,
			V4L2_CID_POWER_LINE_FREQUENCY_60HZ, 0,
			V4L2_CID_POWER_LINE_FREQUENCY_DISABLED);
	v4l2_ctrl_new_int_menu(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_EXPOSURE_BIAS,
			ARRAY_SIZE(exp_bias_qmenu) - 1,
			ARRAY_SIZE(exp_bias_qmenu) / 2 - 1,
			exp_bias_qmenu);
	v4l2_ctrl_new_std_menu_items(&foo->ctrl_handler, &foo_ctrl_ops,
			V4L2_CID_TEST_PATTERN, ARRAY_SIZE(test_pattern) - 1, 0,
			0, test_pattern);
	...
	if (foo->ctrl_handler.error)
		return v4l2_ctrl_handler_free(&foo->ctrl_handler);

`v4l2_ctrl_new_std` 鍑芥暟杩斿洖鎸囧悜鏂版帶浠剁殑 v4l2_ctrl 鎸囬拡锛屼絾濡傛灉浣犱笉闇€瑕佸湪鎺т欢
鎿嶄綔锛坈ontrol ops锛変箣澶栬闂鎸囬拡锛屽垯鏃犻渶淇濆瓨瀹冦€?
`v4l2_ctrl_new_std` 鍑芥暟浼氬熀浜庢帶浠?ID 濉厖澶ч儴鍒嗗瓧娈碉紝闄や簡鏈€灏忓€笺€佹渶澶у€笺€佹闀?鍜岄粯璁ゅ€笺€傝繖浜涢€氳繃鏈€鍚庡洓涓弬鏁颁紶鍏ャ€傝繖浜涘€兼槸椹卞姩鐗瑰畾鐨勶紝鑰岀被鍨嬨€佸悕绉般€佹爣蹇楃瓑
鎺т欢灞炴€ч兘鏄叏灞€鐨勩€傛帶浠剁殑褰撳墠鍊间細琚涓洪粯璁ゅ€笺€?
`v4l2_ctrl_new_std_menu` 鍑芥暟闈炲父鐩镐技锛屼絾瀹冪敤浜庤彍鍗曟帶浠躲€傛病鏈?min 鍙傛暟锛屽洜涓?瀵逛簬鑿滃崟鎺т欢瀹冨缁堜负 0锛屽彇鑰屼唬涔嬬殑鏄?step 涔嬪鏈変竴涓?skip_mask 鍙傛暟锛氬鏋滀綅 X
涓?1锛屽垯鑿滃崟椤?X 琚烦杩囥€?
`v4l2_ctrl_new_int_menu` 鍑芥暟鍒涘缓涓€涓甫鏈夐┍鍔ㄧ壒瀹氳彍鍗曢」鐨勬柊鏍囧噯鏁存暟鑿滃崟鎺т欢銆?瀹冧笌 v4l2_ctrl_new_std_menu 鐨勪笉鍚屼箣澶勫湪浜庡畠娌℃湁 mask 鍙傛暟锛屽苟涓斾互鏈€鍚庝竴涓弬鏁?鎺ュ彈涓€涓湁绗﹀彿 64 浣嶆暣鏁版暟缁勶紝鏋勬垚绮剧‘鐨勮彍鍗曢」鍒楄〃銆?
`v4l2_ctrl_new_std_menu_items` 鍑芥暟涓?v4l2_ctrl_new_std_menu 闈炲父鐩镐技锛屼絾澶氫簡涓€涓?鍙傛暟 qmenu锛屽畠鏄竴涓師鏈爣鍑嗚彍鍗曟帶浠剁殑椹卞姩鐗瑰畾鑿滃崟銆傝繖绫绘帶浠剁殑涓€涓ソ渚嬪瓙鏄?鍏锋湁鐢熸垚娴嬭瘯鍥炬鑳藉姏鐨勬崟鑾?鏄剧ず/浼犳劅鍣ㄨ澶囩殑娴嬭瘯鍥炬鎺т欢銆傝繖浜涙祴璇曞浘妗堟槸纭欢
鐗瑰畾鐨勶紝鍥犳鑿滃崟鐨勫唴瀹逛細鍥犺澶囪€屽紓銆?
娉ㄦ剰锛屽鏋滄煇澶勫け璐ワ紝鍑芥暟灏嗚繑鍥?NULL 鎴栭敊璇紝骞跺皢 ctrl_handler->error 璁剧疆涓洪敊璇爜銆?濡傛灉 ctrl_handler->error 宸茬粡璁剧疆锛屽垯瀹冨彧浼氳繑鍥炶€屼笉鍋氫换浣曚簨鎯呫€傚浜庢棤娉曞垎閰嶅唴閮?鏁版嵁缁撴瀯鐨?v4l2_ctrl_handler_init 涔熸槸濡傛銆?
杩欎娇寰楀垵濮嬪寲澶勭悊鍣紙handler锛夊苟鐩存帴娣诲姞鎵€鏈夋帶浠躲€佸彧鍦ㄦ渶鍚庢鏌ラ敊璇爜鍙樺緱寰堝鏄撱€?鐪佸幓浜嗗ぇ閲忛噸澶嶇殑閿欒妫€鏌ャ€?
寤鸿鎸夋帶浠?ID 鍗囧簭娣诲姞鎺т欢锛氳繖鏍蜂細蹇竴鐐广€?
3) 鍙€夊湴寮哄埗鍒濆鎺т欢璁剧疆锛?
	v4l2_ctrl_handler_setup(&foo->ctrl_handler);

杩欏皢鏃犳潯浠跺湴瀵规墍鏈夋帶浠惰皟鐢?s_ctrl銆傚疄闄呬笂杩欎細鎶婄‖浠跺垵濮嬪寲涓洪粯璁ゆ帶浠跺€笺€傚缓璁綘
杩欐牱鍋氾紝鍥犱负杩欒兘纭繚鍐呴儴鏁版嵁缁撴瀯鍜岀‖浠朵繚鎸佷竴鑷淬€?
4) 鏈€鍚庯細瀹炵幇 `v4l2_ctrl_ops`

	static const struct v4l2_ctrl_ops foo_ctrl_ops = {
		.s_ctrl = foo_s_ctrl,
	};

閫氬父浣犲彧闇€瑕?s_ctrl锛?
	static int foo_s_ctrl(struct v4l2_ctrl *ctrl)
	{
		struct foo *state = container_of(ctrl->handler, struct foo, ctrl_handler);

		switch (ctrl->id) {
		case V4L2_CID_BRIGHTNESS:
			write_reg(0x123, ctrl->val);
			break;
		case V4L2_CID_CONTRAST:
			write_reg(0x456, ctrl->val);
			break;
		}
		return 0;
	}

鎺у埗鎿嶄綔锛坈ontrol ops锛変互 v4l2_ctrl 鎸囬拡浣滀负鍙傛暟琚皟鐢ㄣ€傛柊鐨勬帶浠跺€煎凡缁忚鏍￠獙杩囷紝
鎵€浠ヤ綘鍙渶瀹為檯鍘绘洿鏂扮‖浠跺瘎瀛樺櫒鍗冲彲銆?
浣犲畬鎴愪簡锛佽繖瀵逛簬鎴戜滑鐨勫ぇ澶氭暟椹卞姩鏉ヨ宸茬粡瓒冲銆傛棤闇€瀵规帶浠跺€煎仛浠讳綍鏍￠獙锛屼篃鏃犻渶
瀹炵幇 QUERYCTRL銆丵UERY_EXT_CTRL 鍜?QUERYMENU銆傝€?G/S_CTRL 浠ュ強 G/TRY/S_EXT_CTRLS
浼氳鑷姩鏀寔銆?
   鍏朵綑灏忚妭娑夊強鏇撮珮绾х殑鎺т欢涓婚鍜屽満鏅€傚疄闄呬笂锛屽涓婃墍杩扮殑鍩烘湰鐢ㄦ硶瀵瑰ぇ澶氭暟椹卞姩
   鏉ヨ宸茬粡瓒冲銆?
### 缁ф壙瀛愯澶囨帶浠?
褰撻€氳繃璋冪敤 v4l2_device_register_subdev() 灏嗕竴涓瓙璁惧娉ㄥ唽鍒?V4L2 椹卞姩锛屽苟涓?v4l2_subdev 鍜?v4l2_device 鐨?ctrl_handler 瀛楁閮藉凡璁剧疆鏃讹紝璇ュ瓙璁惧鐨勬帶浠跺皢
鑷姩鍦?V4L2 椹卞姩涓篃鍙敤銆傚鏋滃瓙璁惧椹卞姩鍖呭惈鐨勬帶浠跺湪 V4L2 椹卞姩涓凡缁忓瓨鍦紝鍒?閭ｄ簺鎺т欢浼氳璺宠繃锛堝洜姝?V4L2 椹卞姩濮嬬粓鍙互瑕嗙洊瀛愯澶囨帶浠讹級銆?
杩欓噷鍙戠敓鐨勬槸锛寁4l2_device_register_subdev() 璋冪敤 v4l2_ctrl_add_handler()锛屽皢
瀛愯澶囩殑鎺т欢娣诲姞鍒?v4l2_device 鐨勬帶浠朵腑銆?
### 璁块棶鎺т欢鍊?
鎺у埗妗嗘灦鍐呴儴浣跨敤浠ヤ笅鑱斿悎浣擄紙union锛夋潵璁块棶鎺т欢鍊硷細

	union v4l2_ctrl_ptr {
		s32 *p_s32;
		s64 *p_s64;
		char *p_char;
		void *p;
	};

v4l2_ctrl 缁撴瀯浣撳寘鍚互涓嬪彲鐢ㄤ簬璁块棶褰撳墠鍊煎拰鏂板€肩殑瀛楁锛?
	s32 val;
	struct {
		s32 val;
	} cur;


	union v4l2_ctrl_ptr p_new;
	union v4l2_ctrl_ptr p_cur;

濡傛灉鎺т欢鏄畝鍗曠殑 s32 绫诲瀷锛屽垯锛?
	&ctrl->val == ctrl->p_new.p_s32
	&ctrl->cur.val == ctrl->p_cur.p_s32

瀵逛簬鎵€鏈夊叾瀹冪被鍨嬶紝浣跨敤 ctrl->p_cur.p<something>銆傚熀鏈笂 val 鍜?cur.val 瀛楁鍙互
瑙嗕负鍒悕锛屽洜涓哄畠浠浣跨敤寰楀姝ら绻併€?
鍦ㄦ帶鍒舵搷浣滐紙control ops锛夊唴閮ㄤ綘鍙互鑷敱浣跨敤杩欎簺瀛楁銆倂al 鍜?cur.val 涓嶈█鑷槑銆?p_char 鎸囬拡鎸囧悜闀垮害涓?ctrl->maximum + 1 鐨勫瓧绗︾紦鍐插尯锛屽苟涓旀€绘槸浠?0 缁撳熬銆?
闄ら潪鎺т欢琚爣璁颁负 volatile锛堟槗鍙橈級锛屽惁鍒?p_cur 瀛楁鎸囧悜褰撳墠缂撳瓨鐨勬帶浠跺€笺€傚綋浣犲垱寤?涓€涓柊鎺т欢鏃讹紝璇ュ€间細琚涓轰笌榛樿鍊肩浉鍚屻€傝皟鐢?v4l2_ctrl_handler_setup() 涔嬪悗锛岃
鍊间細琚紶閫掔粰纭欢銆傞€氬父璋冪敤姝ゅ嚱鏁版槸涓ソ涓绘剰銆?
姣忓綋璁剧疆浜嗕竴涓柊鍊硷紝璇ユ柊鍊间細琚嚜鍔ㄧ紦瀛樸€傝繖鎰忓懗鐫€澶у鏁伴┍鍔ㄤ笉闇€瑕佸疄鐜?g_volatile_ctrl()
鎿嶄綔锛坥p锛夈€備緥澶栨儏鍐垫槸杩斿洖鏄撳彉瀵勫瓨鍣紙渚嬪鎸佺画鍙樺寲鐨勪俊鍙峰己搴﹁鏁帮級鐨勬帶浠躲€傚湪杩欑
鎯呭喌涓嬶紝浣犻渶瑕佸儚涓嬮潰杩欐牱瀹炵幇 g_volatile_ctrl锛?
	static int foo_g_volatile_ctrl(struct v4l2_ctrl *ctrl)
	{
		switch (ctrl->id) {
		case V4L2_CID_BRIGHTNESS:
			ctrl->val = read_reg(0x123);
			break;
		}
	}

娉ㄦ剰浣犲湪 g_volatile_ctrl 涓篃浣跨敤浜嗏€滄柊鍊尖€濊仈鍚堜綋銆備竴鑸潵璇达紝闇€瑕佸疄鐜?g_volatile_ctrl
鐨勬帶浠舵槸鍙鎺т欢銆傚鏋滀笉鏄紝鍒欏綋鎺т欢鏀瑰彉鏃朵笉浼氱敓鎴?V4L2_EVENT_CTRL_CH_VALUE 浜嬩欢銆?
瑕佸皢涓€涓帶浠舵爣璁颁负 volatile锛屼綘蹇呴』璁剧疆 V4L2_CTRL_FLAG_VOLATILE锛?
	ctrl = v4l2_ctrl_new_std(&sd->ctrl_handler, ...);
	if (ctrl)
		ctrl->flags |= V4L2_CTRL_FLAG_VOLATILE;

瀵逛簬 try/s_ctrl锛屾柊鍊硷紙鍗崇敤鎴蜂紶鍏ョ殑鍊硷級浼氳濉叆锛屼綘鍙互鍦?try_ctrl 涓慨鏀瑰畠浠紝
鎴栧湪 s_ctrl 涓缃畠浠€?cur' 鑱斿悎浣撳寘鍚綋鍓嶅€硷紝浣犱篃鍙互锛堜絾涓嶈兘淇敼锛侊級浣跨敤瀹冦€?
濡傛灉 s_ctrl 杩斿洖 0锛圤K锛夛紝鍒欐帶鍒舵鏋朵細鎶婃柊鐨勬渶缁堝€煎鍒跺埌 'cur' 鑱斿悎浣撱€?
鍦?g_volatile/s/try_ctrl 鍐呴儴锛屼綘鍙互璁块棶鍚屼竴涓鐞嗗櫒锛坔andler锛夋嫢鏈夌殑鎵€鏈夋帶浠剁殑
鍊硷紝鍥犱负澶勭悊鍣紙handler锛夌殑閿侊紙lock锛夎鎸佹湁銆傚鏋滀綘闇€瑕佽闂叾瀹冨鐞嗗櫒锛坔andler锛?鎷ユ湁鐨勬帶浠跺€硷紝鍒欏繀椤婚潪甯稿皬蹇冿紝閬垮厤寮曞叆姝婚攣銆?
鍦ㄦ帶鍒舵搷浣滐紙control ops锛変箣澶栵紝浣犲繀椤婚€氳繃杈呭姪鍑芥暟鏉ュ畨鍏ㄥ湴鑾峰彇鎴栬缃┍鍔ㄤ腑鐨勫崟涓?鎺т欢鍊硷細

	s32 v4l2_ctrl_g_ctrl(struct v4l2_ctrl *ctrl);
	int v4l2_ctrl_s_ctrl(struct v4l2_ctrl *ctrl, s32 val);

杩欎簺鍑芥暟涓庢帶鍒舵鏋剁殑浜や簰鏂瑰紡涓?VIDIOC_G/S_CTRL ioctl 鐩稿悓銆備笉杩囷紝涓嶈鍦ㄦ帶鍒舵搷浣?g_volatile/s/try_ctrl 鍐呴儴浣跨敤瀹冧滑锛屽洜涓鸿繖浼氬鑷存閿侊紝鍥犱负杩欎簺杈呭姪鍑芥暟鍚屾牱浼氶攣瀹?澶勭悊鍣紙handler锛夈€?
浣犱篃鍙互鑷繁鑾峰彇澶勭悊鍣紙handler锛夐攣锛?
	mutex_lock(&state->ctrl_handler.lock);
	pr_info("String value is '%s'\n", ctrl1->p_cur.p_char);
	pr_info("Integer value is '%s'\n", ctrl2->cur.val);
	mutex_unlock(&state->ctrl_handler.lock);

### 鑿滃崟鎺т欢

v4l2_ctrl 缁撴瀯浣撳寘鍚繖涓仈鍚堜綋锛?
	union {
		u32 step;
		u32 menu_skip_mask;
	};

瀵逛簬鑿滃崟鎺т欢浣跨敤 menu_skip_mask銆傚畠鐨勪綔鐢ㄦ槸璁╀綘鍙互杞绘澗鎺掗櫎鏌愪簺鑿滃崟椤广€傝繖鍦?VIDIOC_QUERYMENU 鐨勫疄鐜颁腑浼氱敤鍒帮紝褰撴煇涓彍鍗曢」涓嶅瓨鍦ㄦ椂浣犲彲浠ヨ繑鍥?-EINVAL銆傛敞鎰忥紝
瀵逛簬鑿滃崟鎺т欢锛孷IDIOC_QUERYCTRL 濮嬬粓杩斿洖姝ラ暱鍊?1銆?
涓€涓緢濂界殑渚嬪瓙鏄?MPEG Audio Layer II Bitrate 鑿滃崟鎺т欢锛屽叾涓彍鍗曟槸鏍囧噯鍖栧彲鑳?姣旂壒鐜囩殑鍒楄〃銆備絾鍦ㄥ疄闄呬腑锛岀‖浠跺疄鐜板彧浼氭敮鎸佸叾涓殑涓€涓瓙闆嗐€傞€氳繃璁剧疆 skip 鎺╃爜
锛坢ask锛夛紝浣犲彲浠ュ憡璇夋鏋跺摢浜涜彍鍗曢」搴旇琚烦杩囥€傚皢鍏惰缃负 0 琛ㄧず鏀寔鎵€鏈夎彍鍗曢」銆?
浣犲彲浠ラ€氳繃 v4l2_ctrl_config 缁撴瀯浣擄紙閽堝鑷畾涔夋帶浠讹級鎴栬皟鐢?v4l2_ctrl_new_std_menu()
鏉ヨ缃鎺╃爜锛坢ask锛夈€?
### 鑷畾涔夋帶浠?
鍙互浣跨敤 v4l2_ctrl_new_custom() 鍒涘缓椹卞姩鐗瑰畾鐨勬帶浠讹細

	static const struct v4l2_ctrl_config ctrl_filter = {
		.ops = &ctrl_custom_ops,
		.id = V4L2_CID_MPEG_CX2341X_VIDEO_SPATIAL_FILTER,
		.name = "Spatial Filter",
		.type = V4L2_CTRL_TYPE_INTEGER,
		.flags = V4L2_CTRL_FLAG_SLIDER,
		.max = 15,
		.step = 1,
	};

	ctrl = v4l2_ctrl_new_custom(&foo->ctrl_handler, &ctrl_filter, NULL);

鏈€鍚庝竴涓弬鏁版槸 priv 鎸囬拡锛屽彲璁剧疆涓洪┍鍔ㄧ壒瀹氱殑绉佹湁鏁版嵁銆?
v4l2_ctrl_config 缁撴瀯浣撹繕鏈変竴涓瓧娈电敤浜庤缃?is_private 鏍囧織銆?
濡傛灉鏈缃?name 瀛楁锛屽垯妗嗘灦浼氬亣瀹氳繖鏄竴涓爣鍑嗘帶浠讹紝骞剁浉搴斿湴濉厖 name銆乼ype 鍜?flags 瀛楁銆?
### 娲诲姩锛坅ctive锛変笌鎶撳彇锛坓rabbed锛夋帶浠?
濡傛灉浣犻亣鍒版帶浠朵箣闂存洿澶嶆潅鐨勫叧绯伙紝閭ｄ箞浣犲彲鑳藉繀椤绘縺娲绘垨鍋滅敤鎺т欢銆備緥濡傦紝濡傛灉 Chroma
AGC 鎺т欢寮€鍚紝閭ｄ箞 Chroma Gain 鎺т欢灏辨槸闈炴椿鍔ㄧ殑銆備篃灏辨槸璇达紝浣犲彲浠ヨ缃畠锛屼絾鍙
鑷姩澧炵泭鎺у埗杩樺紑鐫€锛岀‖浠跺氨涓嶄細浣跨敤璇ュ€笺€傚吀鍨嬬殑鐢ㄦ埛鐣岄潰鍙互绂佺敤姝ょ被杈撳叆瀛楁銆?
浣犲彲浠ヤ娇鐢?v4l2_ctrl_activate() 璁剧疆鈥滄椿鍔ㄢ€濈姸鎬併€傞粯璁ゆ儏鍐典笅鎵€鏈夋帶浠堕兘鏄椿鍔ㄧ殑銆?娉ㄦ剰妗嗘灦涓嶄細妫€鏌ユ鏍囧織銆傚畠绾补鏄负 GUI 鍑嗗鐨勩€傝鍑芥暟閫氬父鍦?s_ctrl 鍐呴儴璋冪敤銆?
鍙︿竴涓爣蹇楁槸鈥滄姄鍙栤€濓紙grabbed锛夋爣蹇椼€備竴涓鎶撳彇鐨勬帶浠舵剰鍛崇潃浣犳棤娉曟洿鏀瑰畠锛屽洜涓哄畠姝?琚煇涓祫婧愪娇鐢ㄣ€傚吀鍨嬬殑渚嬪瓙鏄?MPEG 姣旂壒鐜囨帶浠讹紝鍦ㄦ崟鑾疯繘琛屾湡闂存棤娉曟洿鏀广€?
濡傛灉浣跨敤 v4l2_ctrl_grab() 灏嗕竴涓帶浠惰缃负鈥滄姄鍙栤€濓紝閭ｄ箞褰撹瘯鍥捐缃鎺т欢鏃舵鏋跺皢
杩斿洖 -EBUSY銆倂4l2_ctrl_grab() 鍑芥暟閫氬父鍦ㄩ┍鍔ㄥ惎鍔ㄦ垨鍋滄娴佷紶杈撴椂璋冪敤銆?
### 鎺т欢绨囷紙Control Clusters锛?
榛樿鎯呭喌涓嬫墍鏈夋帶浠跺郊姝ょ嫭绔嬨€備絾鍦ㄦ洿澶嶆潅鐨勫満鏅腑锛屼綘鍙兘寰楀埌涓€涓帶浠跺鍙︿竴涓殑
渚濊禆鍏崇郴銆傚湪杩欑鎯呭喌涓嬶紝浣犻渶瑕佸皢瀹冧滑鈥滆仛绫烩€濓紙cluster锛夛細

	struct foo {
		struct v4l2_ctrl_handler ctrl_handler;
	#define AUDIO_CL_VOLUME (0)
	#define AUDIO_CL_MUTE   (1)
		struct v4l2_ctrl *audio_cluster[^2^];
		...
	};

	state->audio_cluster[AUDIO_CL_VOLUME] =
		v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	state->audio_cluster[AUDIO_CL_MUTE] =
		v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	v4l2_ctrl_cluster(ARRAY_SIZE(state->audio_cluster), state->audio_cluster);

浠庝粖浠ュ悗锛屽彧瑕佸睘浜庡悓涓€涓皣鐨勪竴涓垨澶氫釜鎺т欢琚缃紙鎴栤€滆幏鍙栤€濓紝鎴栤€滃皾璇曗€濓級锛屽彧浼?璋冪敤绗竴涓帶浠讹紙鏈緥涓负鈥渧olume鈥濓級鐨勬帶鍒舵搷浣滐紙control ops锛夈€備綘瀹為檯涓婂垱寤轰簡涓€涓?鏂扮殑澶嶅悎鎺т欢銆傜被浼间簬 C 璇█涓€渟truct鈥濈殑宸ヤ綔鏂瑰紡銆?
鍥犳锛屽綋 s_ctrl 浠?V4L2_CID_AUDIO_VOLUME 浣滀负鍙傛暟琚皟鐢ㄦ椂锛屼綘搴旇璁剧疆灞炰簬
audio_cluster 鐨勫叏閮ㄤ袱涓帶浠讹細

	static int foo_s_ctrl(struct v4l2_ctrl *ctrl)
	{
		struct foo *state = container_of(ctrl->handler, struct foo, ctrl_handler);

		switch (ctrl->id) {
		case V4L2_CID_AUDIO_VOLUME: {
			struct v4l2_ctrl *mute = ctrl->cluster[AUDIO_CL_MUTE];

			write_reg(0x123, mute->val ? 0 : ctrl->val);
			break;
		}
		case V4L2_CID_CONTRAST:
			write_reg(0x456, ctrl->val);
			break;
		}
		return 0;
	}

鍦ㄤ笂闈㈢殑渚嬪瓙涓紝瀵逛簬 VOLUME 鎯呭喌锛屼互涓嬩笁鑰呯瓑浠凤細

	ctrl == ctrl->cluster[AUDIO_CL_VOLUME] == state->audio_cluster[AUDIO_CL_VOLUME]
	ctrl->cluster[AUDIO_CL_MUTE] == state->audio_cluster[AUDIO_CL_MUTE]

鍦ㄥ疄璺典腑锛屽儚杩欐牱浣跨敤绨囨暟缁勪細鍙樺緱闈炲父绻佺悙銆傚洜姝ゆ敼鐢ㄤ互涓嬬瓑浠风殑鏂规硶锛?
	struct {
		/** audio cluster **/
		struct v4l2_ctrl *volume;
		struct v4l2_ctrl *mute;
	};

杩欎釜鍖垮悕缁撴瀯浣撶敤浜庢竻鏅板湴鈥滆仛绫烩€濊繖涓や釜鎺т欢鎸囬拡锛屼絾瀹冩病鏈夊叾瀹冪敤閫斻€傛晥鏋滀笌鍒涘缓
涓€涓甫涓や釜鎺т欢鎸囬拡鐨勬暟缁勭浉鍚屻€傛墍浠ヤ綘鍙互鐩存帴杩欐牱鍋氾細

	state->volume = v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	state->mute = v4l2_ctrl_new_std(&state->ctrl_handler, ...);
	v4l2_ctrl_cluster(2, &state->volume);

鍦?foo_s_ctrl 涓綘鍙互鐩存帴浣跨敤杩欎簺鎸囬拡锛歴tate->mute->val銆?
娉ㄦ剰锛岀皣涓殑鎺т欢鍙兘涓?NULL銆備緥濡傦紝濡傛灉鐢变簬鏌愮鍘熷洜 mute 浠庢湭琚坊鍔狅紙鍥犱负纭欢
涓嶆敮鎸佽鐗瑰畾鐗规€э級锛岄偅涔?mute 灏嗘槸 NULL銆傛墍浠ュ湪杩欑鎯呭喌涓嬫垜浠湁涓€涓寘鍚?2 涓帶浠?鐨勭皣锛屽叾涓彧鏈?1 涓疄闄呰瀹炰緥鍖栥€傚敮涓€鐨勯檺鍒舵槸绨囩殑绗竴涓帶浠跺繀椤诲缁堝瓨鍦紝鍥犱负
瀹冩槸绨囩殑鈥滀富鈥濓紙master锛夋帶浠躲€備富鎺т欢鏄瘑鍒绨囩殑鎺т欢锛屽苟鎻愪緵鐢ㄤ簬璇ョ皣鐨?v4l2_ctrl_ops 缁撴瀯浣撶殑鎸囬拡銆?
鏄剧劧锛岀皣鏁扮粍涓殑鎵€鏈夋帶浠跺繀椤昏鍒濆鍖栦负鏈夋晥鐨勬帶浠舵垨 NULL銆?
鍦ㄦ瀬灏戞暟鎯呭喌涓嬶紝浣犲彲鑳芥兂鐭ラ亾绨囦腑鐨勫摢浜涙帶浠跺疄闄呬笂鏄鐢ㄦ埛鏄惧紡璁剧疆鐨勩€備负姝や綘鍙互
妫€鏌ユ瘡涓帶浠剁殑鈥渋s_new鈥濇爣蹇椼€備緥濡傦紝鍦?volume/mute 绨囩殑鎯呭喌涓嬶紝濡傛灉鍙负鐢ㄦ埛璋冪敤浜?VIDIOC_S_CTRL 璁剧疆 mute锛岄偅涔?mute 鎺т欢鐨勨€渋s_new鈥濇爣蹇椾細琚缃€傚鏋滅敤鎴蜂负 mute 鍜?volume 鎺т欢閮借皟鐢ㄤ簡 VIDIOC_S_EXT_CTRLS锛岄偅涔堜袱涓帶浠剁殑鈥渋s_new鈥濇爣蹇楅兘灏嗘槸 1銆?
鈥渋s_new鈥濇爣蹇楀湪浠?v4l2_ctrl_handler_setup() 璋冪敤鏃跺缁堜负 1銆?
### 浣跨敤鑷姩绨囷紙Auto Clusters锛夊鐞?autogain/gain 绫诲瀷鎺т欢

涓€绉嶅父瑙佺殑鎺т欢绨囩被鍨嬪鐞嗙殑鏄€渁uto-foo/foo鈥濈被鍨嬬殑鎺т欢銆傚吀鍨嬬殑渚嬪瓙鏄?autogain/gain銆乤utoexposure/exposure銆乤utowhitebalance/red balance/blue balance銆?鍦ㄦ墍鏈夋儏鍐典笅锛屼綘閮芥湁涓€涓帶浠跺喅瀹氬彟涓€涓帶浠舵槸鐢辩‖浠惰嚜鍔ㄥ鐞嗭紝杩樻槸鐢辩敤鎴锋墜鍔ㄦ帶鍒躲€?
濡傛灉绨囧浜庤嚜鍔ㄦā寮忥紝閭ｄ箞鎵嬪姩鎺т欢搴旇琚爣璁颁负闈炴椿鍔紙inactive锛夊拰鏄撳彉锛坴olatile锛夈€?褰撹鍙栨槗鍙樻帶浠舵椂锛実_volatile_ctrl 鎿嶄綔搴旇杩斿洖鐢辩‖浠惰嚜鍔ㄦā寮忚嚜鍔ㄨ缃殑鍊笺€?
濡傛灉绨囪鍒囨崲鍒版墜鍔ㄦā寮忥紝閭ｄ箞鎵嬪姩鎺т欢搴旇閲嶆柊鍙樹负娲诲姩锛坅ctive锛夛紝骞朵笖娓呴櫎 volatile
鏍囧織锛堝洜姝ゅ湪鎵嬪姩妯″紡涓嬩笉鍐嶈皟鐢?g_volatile_ctrl锛夈€傛澶栵紝灏卞湪鍒囨崲鍒版墜鍔ㄦā寮忎箣鍓嶏紝
鐢辫嚜鍔ㄦā寮忕‘瀹氱殑褰撳墠鍊间細琚鍒朵负鏂扮殑鎵嬪姩鍊笺€?
鏈€鍚庯紝搴旇涓鸿嚜鍔ㄦ帶浠惰缃?V4L2_CTRL_FLAG_UPDATE锛屽洜涓烘洿鏀硅鎺т欢浼氬奖鍝嶆墜鍔ㄦ帶浠剁殑
鎺у埗鏍囧織銆?
涓轰簡绠€鍖栬繖涓€鐐癸紝寮曞叆浜嗕竴涓?v4l2_ctrl_cluster 鐨勭壒娈婂彉浣擄細

	void v4l2_ctrl_auto_cluster(unsigned ncontrols, struct v4l2_ctrl **controls,
				    u8 manual_val, bool set_volatile);

鍓嶄袱涓弬鏁颁笌 v4l2_ctrl_cluster 鐩稿悓銆傜涓変釜鍙傛暟鍛婅瘔妗嗘灦鍝釜鍊间細灏嗙皣鍒囨崲鍒版墜鍔ㄦā寮忋€?鏈€鍚庝竴涓弬鏁板彲閫夊湴锛坥ptionally锛変负闈炶嚜鍔ㄦ帶浠惰缃?V4L2_CTRL_FLAG_VOLATILE銆傚鏋滀负
false锛屽垯鎵嬪姩鎺т欢姘歌繙涓嶄細鏄槗鍙樼殑銆傚鏋滅‖浠朵笉鍏佽浣犺鍥炵敱鑷姩妯″紡纭畾鐨勫€硷紙渚嬪
濡傛灉 autogain 寮€鍚紝纭欢涓嶅厑璁镐綘鑾峰彇褰撳墠澧炵泭鍊硷級锛屼綘閫氬父浼氫娇鐢?false銆?
绨囩殑绗竴涓帶浠惰鍋囧畾涓衡€渁uto鈥濇帶浠躲€?
浣跨敤姝ゅ嚱鏁板彲纭繚浣犳棤闇€澶勭悊鎵€鏈夊鏉傜殑鏍囧織鍜屾槗鍙橈紙volatile锛夊鐞嗐€?
### VIDIOC_LOG_STATUS 鏀寔

杩欎釜 ioctl 鍏佽浣犲皢椹卞姩鐨勫綋鍓嶇姸鎬佽浆鍌ㄥ埌鍐呮牳鏃ュ織銆倂4l2_ctrl_handler_log_status
(ctrl_handler, prefix) 鍙敤浜庡皢缁欏畾澶勭悊鍣紙handler锛夋墍鎷ユ湁鐨勬帶浠跺€艰浆鍌ㄥ埌鏃ュ織銆?浣犱篃鍙互鎻愪緵涓€涓墠缂€锛坧refix锛夈€傚鏋滃墠缂€娌℃湁浠ョ┖鏍肩粨灏撅紝鍒欎細涓轰綘娣诲姞鈥? 鈥濄€?
### 涓嶅悓瑙嗛鑺傜偣浣跨敤涓嶅悓鐨勫鐞嗗櫒

閫氬父 V4L2 椹卞姩鍙湁涓€涓鎵€鏈夎棰戣妭鐐瑰叏灞€鐨勬帶鍒跺鐞嗗櫒锛坔andler锛夈€備絾浣犱篃鍙互涓?涓嶅悓鐨勮棰戣妭鐐规寚瀹氫笉鍚岀殑鎺у埗澶勭悊鍣ㄣ€備綘鍙互閫氳繃鎵嬪姩璁剧疆 struct video_device 鐨?ctrl_handler 瀛楁鏉ュ仛鍒拌繖涓€鐐广€?
濡傛灉娌℃湁娑夊強瀛愯澶囷紙subdev锛夛紝杩欐病鏈夐棶棰橈紱浣嗗鏋滄湁锛岄偅涔堜綘闇€瑕侀樆姝㈠瓙璁惧鎺т欢
鑷姩鍚堝苟鍒板叏灞€鎺у埗澶勭悊鍣ㄣ€備綘鍙渶灏?struct v4l2_device 涓殑 ctrl_handler 瀛楁
璁剧疆涓?NULL 鍗冲彲銆傜幇鍦?v4l2_device_register_subdev() 灏嗕笉鍐嶅悎骞跺瓙璁惧鎺т欢銆?
鍦ㄦ瘡涓瓙璁惧琚坊鍔犱箣鍚庯紝浣犲皢蹇呴』鎵嬪姩璋冪敤 v4l2_ctrl_add_handler锛屽皢瀛愯澶囩殑鎺у埗
澶勭悊鍣紙sd->ctrl_handler锛夋坊鍔犲埌鎵€闇€鐨勫鐞嗗櫒銆傝繖涓帶鍒跺鐞嗗櫒鍙兘鐗瑰畾浜庢煇涓?video_device锛屾垨鏌愪釜 video_device 鐨勫瓙闆嗐€備緥濡傦細radio 璁惧鑺傜偣鍙湁闊抽鎺т欢锛岃€?video 鍜?vbi 璁惧鑺傜偣鍏变韩鍚屼竴涓敤浜庨煶棰戝拰瑙嗛鎺т欢鐨勬帶鍒跺鐞嗗櫒銆?
濡傛灉浣犲笇鏈涜涓€涓鐞嗗櫒锛堜緥濡傜敤浜?radio 璁惧鑺傜偣锛夋嫢鏈夊彟涓€涓鐞嗗櫒锛堜緥濡傜敤浜?video 璁惧鑺傜偣锛夌殑瀛愰泦锛岄偅涔堜綘搴旇棣栧厛娣诲姞鎺т欢鍒扮涓€涓鐞嗗櫒锛屾坊鍔犲叾瀹冩帶浠跺埌
绗簩涓鐞嗗櫒锛屾渶鍚庡皢绗竴涓鐞嗗櫒娣诲姞鍒扮浜屼釜澶勭悊鍣ㄣ€備緥濡傦細

	v4l2_ctrl_new_std(&radio_ctrl_handler, &radio_ops, V4L2_CID_AUDIO_VOLUME, ...);
	v4l2_ctrl_new_std(&radio_ctrl_handler, &radio_ops, V4L2_CID_AUDIO_MUTE, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &video_ops, V4L2_CID_BRIGHTNESS, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &video_ops, V4L2_CID_CONTRAST, ...);
	v4l2_ctrl_add_handler(&video_ctrl_handler, &radio_ctrl_handler, NULL);

v4l2_ctrl_add_handler() 鐨勬渶鍚庝竴涓弬鏁版槸涓€涓繃婊ゅ嚱鏁帮紝鍏佽浣犺繃婊ゅ摢浜涙帶浠朵細琚坊鍔犮€?濡傛灉浣犳兂娣诲姞鎵€鏈夋帶浠讹紝鍒欏皢鍏惰涓?NULL銆?
鎴栬€呬綘鍙互灏嗙壒瀹氭帶浠舵坊鍔犲埌涓€涓鐞嗗櫒锛?
	volume = v4l2_ctrl_new_std(&video_ctrl_handler, &ops, V4L2_CID_AUDIO_VOLUME, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &ops, V4L2_CID_BRIGHTNESS, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &ops, V4L2_CID_CONTRAST, ...);

浣犱笉搴旇鍋氱殑鏄负涓や釜澶勭悊鍣ㄥ垱寤轰袱涓浉鍚岀殑鎺т欢銆備緥濡傦細

	v4l2_ctrl_new_std(&radio_ctrl_handler, &radio_ops, V4L2_CID_AUDIO_MUTE, ...);
	v4l2_ctrl_new_std(&video_ctrl_handler, &video_ops, V4L2_CID_AUDIO_MUTE, ...);

杩欏緢绯熺硶锛屽洜涓洪潤闊?radio 涓嶄細鏀瑰彉 video 闈欓煶鎺т欢銆傝鍒欐槸锛氬浜庢瘡涓綘鍙互鎷ㄥ姩鐨?纭欢鈥滄棆閽€濓紝搴旇鏈変竴涓帶浠躲€?
### 鏌ユ壘鎺т欢

閫氬父浣犲凡缁忚嚜宸卞垱寤轰簡鎺т欢锛屽苟涓斿彲浠ユ妸 struct v4l2_ctrl 鎸囬拡淇濆瓨鍒拌嚜宸辩殑缁撴瀯浣撲腑銆?
浣嗘湁鏃朵綘闇€瑕佷粠涓€涓綘涓嶆嫢鏈夌殑鍙︿竴涓鐞嗗櫒锛坔andler锛変腑鏌ユ壘鎺т欢銆備緥濡傦紝濡傛灉浣犲繀椤?浠庝竴涓瓙璁惧锛坰ubdev锛変腑鏌ユ壘 volume 鎺т欢銆?
浣犲彲浠ラ€氳繃璋冪敤 v4l2_ctrl_find 鏉ュ仛鍒拌繖涓€鐐癸細

	struct v4l2_ctrl *volume;

	volume = v4l2_ctrl_find(sd->ctrl_handler, V4L2_CID_AUDIO_VOLUME);

鐢变簬 v4l2_ctrl_find 浼氶攣瀹氬鐞嗗櫒锛坔andler锛夛紝鎵€浠ヤ綘蹇呴』灏忓績鍦ㄥ摢閲屼娇鐢ㄥ畠銆備緥濡傦紝
杩欏苟涓嶆槸涓€涓ソ涓绘剰锛?
	struct v4l2_ctrl_handler ctrl_handler;

	v4l2_ctrl_new_std(&ctrl_handler, &video_ops, V4L2_CID_BRIGHTNESS, ...);
	v4l2_ctrl_new_std(&ctrl_handler, &video_ops, V4L2_CID_CONTRAST, ...);

鈥︹€﹁€屽湪 video_ops.s_ctrl 涓細

	case V4L2_CID_BRIGHTNESS:
		contrast = v4l2_find_ctrl(&ctrl_handler, V4L2_CID_CONTRAST);
		...

褰撴鏋惰皟鐢?s_ctrl 鏃讹紝ctrl_handler.lock 宸茬粡琚幏鍙栵紝鍥犳璇曞浘浠庡悓涓€涓鐞嗗櫒鏌ユ壘
鍙︿竴涓帶浠朵細瀵艰嚧姝婚攣銆?
寤鸿涓嶈鍦ㄦ帶鍒舵搷浣滐紙control ops锛夊唴閮ㄤ娇鐢ㄦ鍑芥暟銆?
### 闃绘鎺т欢缁ф壙

褰撲娇鐢ㄤ竴涓帶鍒跺鐞嗗櫒锛坔andler锛夐€氳繃 v4l2_ctrl_add_handler 娣诲姞鍒板彟涓€涓椂锛岄粯璁?鎯呭喌涓嬪叾涓竴涓殑鎵€鏈夋帶浠堕兘浼氳鍚堝苟鍒板彟涓€涓€備絾涓€涓瓙璁惧鍙兘鎷ユ湁瀵规煇涓珮绾у祵鍏ュ紡
绯荤粺鏈夋剰涔夈€佷絾鍦ㄦ秷璐圭骇纭欢涓娇鐢ㄦ椂姣棤鎰忎箟鐨勫簳灞傛帶浠躲€傚湪杩欑鎯呭喌涓嬶紝浣犲笇鏈涘皢杩欎簺
搴曞眰鎺т欢淇濈暀鍦ㄥ瓙璁惧鏈湴銆備綘鍙互閫氳繃灏嗘帶浠剁殑鈥渋s_private鈥濇爣蹇楄涓?1 鏉ュ仛鍒拌繖涓€鐐癸細

	static const struct v4l2_ctrl_config ctrl_private = {
		.ops = &ctrl_custom_ops,
		.id = V4L2_CID_...,
		.name = "Some Private Control",
		.type = V4L2_CTRL_TYPE_INTEGER,
		.max = 15,
		.step = 1,
		.is_private = 1,
	};

	ctrl = v4l2_ctrl_new_custom(&foo->ctrl_handler, &ctrl_private, NULL);

鐜板湪璋冪敤 v4l2_ctrl_add_handler 鏃朵細璺宠繃杩欎簺鎺т欢銆?
### V4L2_CTRL_TYPE_CTRL_CLASS 鎺т欢

GUI 鍙互浣跨敤姝ょ被鎺т欢鏉ヨ幏鍙栨帶浠剁被锛坈ontrol class锛夌殑鍚嶇О銆傚姛鑳藉畬澶囩殑 GUI 鍙互鍒涘缓
涓€涓甫澶氫釜閫夐」鍗＄殑瀵硅瘽妗嗭紝姣忎釜閫夐」鍗″寘鍚睘浜庢煇涓壒瀹氭帶浠剁被鐨勬帶浠躲€傛瘡涓€夐」鍗＄殑
鍚嶇О鍙互閫氳繃鏌ヨ涓€涓?ID 涓?<control class | 1> 鐨勭壒娈婃帶浠舵潵鎵惧埌銆?
椹卞姩鏃犻渶鍏冲績杩欎竴鐐广€傛瘡褰撴坊鍔犲睘浜庝竴涓柊鐨勬帶浠剁被鐨勭涓€涓帶浠舵椂锛屾鏋朵細鑷姩娣诲姞姝ょ被
鎺т欢銆?
### 娣诲姞閫氱煡鍥炶皟锛圢otify Callbacks锛?
鏈夋椂骞冲彴鎴栨ˉ鎺ワ紙bridge锛夐┍鍔ㄩ渶瑕佸湪瀛愯澶囬┍鍔ㄧ殑鏌愪釜鎺т欢鏀瑰彉鏃舵敹鍒伴€氱煡銆備綘鍙互閫氳繃
璋冪敤姝ゅ嚱鏁拌缃?notify 鍥炶皟锛?
	void v4l2_ctrl_notify(struct v4l2_ctrl *ctrl,
		void (**notify)(struct v4l2_ctrl **ctrl, void **priv), void **priv);

姣忓綋缁欏畾鐨勬帶浠跺€兼敼鍙樻椂锛宯otify 鍥炶皟浼氫互鎸囧悜璇ユ帶浠剁殑鎸囬拡浠ュ強浼犵粰 v4l2_ctrl_notify
鐨?priv 鎸囬拡琚皟鐢ㄣ€傛敞鎰忥紝璋冪敤 notify 鍑芥暟鏃舵帶鍒跺鐞嗗櫒锛坔andler锛夌殑閿侊紙lock锛夎鎸佹湁銆?
姣忎釜鎺у埗澶勭悊鍣紙handler锛夊彧鑳芥湁涓€涓?notify 鍑芥暟銆備换浣曡缃彟涓€涓?notify 鍑芥暟鐨勫皾璇?閮戒細瀵艰嚧 WARN_ON銆?
### v4l2_ctrl 鍑芥暟涓庢暟鎹粨鏋?