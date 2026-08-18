


######## 瑙嗛杈撳嚭鍙犲姞鎺ュ彛锛圴ideo Output Overlay Interface锛?

**涔熺О涓哄睆涓婃樉绀猴紙On-Screen Display锛孫SD锛?*

鏌愪簺瑙嗛杈撳嚭璁惧鍙互灏嗗抚缂撳啿锛坒ramebuffer锛夊浘鍍忓彔鍔犲埌
浼犲嚭鐨勮棰戜俊鍙蜂笂銆傚簲鐢ㄧ▼搴忓彲浠ヤ娇鐢ㄦ鎺ュ彛璁剧疆杩欐牱鐨勫彔鍔狅紝
璇ユ帴鍙ｅ€熺敤浜嗚棰戝彔鍔?<overlay> 鎺ュ彛鐨勭粨鏋勪笌 ioctl銆?
OSD 鍔熻兘閫氳繃璁块棶瑙嗛杈撳嚭 <capture> 鍔熻兘鐨勫悓涓€涓瓧绗﹁澶?鐗规畩鏂囦欢鏉ヨ闂€?
   杩欐牱鐨?`/dev/video` 璁惧鐨勯粯璁ゅ姛鑳芥槸瑙嗛鎹曡幏鎴?   杈撳嚭銆侽SD 鍔熻兘鍙湁鍦ㄤ娇鐢?VIDIOC_S_FMT <VIDIOC_G_FMT>
   ioctl 璋冪敤涔嬪悗鎵嶅彲鐢ㄣ€?
## 鏌ヨ鑳藉姏


鏀寔**瑙嗛杈撳嚭鍙犲姞**鎺ュ彛鐨勮澶囦細鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?struct `v4l2_capability` 鐨?`capabilities` 瀛楁涓缃?`V4L2_CAP_VIDEO_OUTPUT_OVERLAY` 鏍囧織銆?
## 甯х紦鍐?

涓?*瑙嗛鍙犲姞**鎺ュ彛鐩稿弽锛屽抚缂撳啿閫氬父瀹炵幇鍦ㄧ數瑙嗗崱涓婅€岄潪
鏄惧崱涓娿€傚湪 Linux 涓婏紝瀹冧綔涓哄抚缂撳啿璁惧锛坄/dev/fbN`锛夊彲璁块棶銆?缁欏畾涓€涓?V4L2 璁惧锛屽簲鐢ㄧ▼搴忓彲浠ラ€氳繃璋冪敤
VIDIOC_G_FBUF <VIDIOC_G_FBUF> ioctl 鎵惧埌瀵瑰簲鐨勫抚缂撳啿璁惧銆?闄ゅ叾浠栦俊鎭锛屽畠杩斿洖甯х紦鍐插湪 struct `v4l2_framebuffer`
鐨?`base` 瀛楁涓殑鐗╃悊鍦板潃銆?甯х紦鍐茶澶?ioctl `FBIOGET_FSCREENINFO` 鍦?struct
`fb_fix_screeninfo` 鐨?`smem_start` 瀛楁涓繑鍥炵浉鍚岀殑鍦板潃銆?`FBIOGET_FSCREENINFO` ioctl 涓?struct `fb_fix_screeninfo`
瀹氫箟鍦?`linux/fb.h` 澶存枃浠朵腑銆?
甯х紦鍐茬殑瀹藉害涓庨珮搴﹀彇鍐充簬褰撳墠鐨勮棰戞爣鍑嗐€俈4L2 椹卞姩鍙兘浼氭嫆缁?鏇存敼瑙嗛鏍囧噯锛堟垨浠讳綍鍏朵粬鎰忓懗鐫€甯х紦鍐插ぇ灏忓彉鍖栫殑 ioctl锛夌殑灏濊瘯锛?杩斿洖 `EBUSY` 閿欒鐮侊紝鐩村埌鎵€鏈夊簲鐢ㄧ▼搴忛兘鍏抽棴浜嗗抚缂撳啿璁惧銆?
### 绀轰緥锛氫负 OSD 瀵绘壘甯х紦鍐茶澶?


    #include <linux/fb.h>

    struct v4l2_framebuffer fbuf;
    unsigned int i;
    int fb_fd;

    if (-1 == ioctl(fd, VIDIOC_G_FBUF, &fbuf)) {
	perror("VIDIOC_G_FBUF");
	exit(EXIT_FAILURE);
    }

    for (i = 0; i < 30; i++) {
	char dev_name[^16^];
	struct fb_fix_screeninfo si;

	snprintf(dev_name, sizeof(dev_name), "/dev/fb%u", i);

	fb_fd = open(dev_name, O_RDWR);
	if (-1 == fb_fd) {
	    switch (errno) {
	    case ENOENT: /** 鏃犳鏂囦欢 **/
	    case ENXIO:  /** 鏃犻┍鍔?**/
		continue;

	    default:
		perror("open");
		exit(EXIT_FAILURE);
	    }
	}

	if (0 == ioctl(fb_fd, FBIOGET_FSCREENINFO, &si)) {
	    if (si.smem_start == (unsigned long)fbuf.base)
		break;
	} else {
	    /** 鏄剧劧涓嶆槸涓€涓抚缂撳啿璁惧銆?**/
	}

	close(fb_fd);
	fb_fd = -1;
    }

    /* fb_fd 鏄棰戣緭鍑哄彔鍔犵殑甯х紦鍐茶澶囩殑鏂囦欢鎻忚堪绗︼紝
       濡傛灉鏈壘鍒拌澶囧垯涓?-1銆?*/


## 鍙犲姞绐楀彛涓庣缉鏀?

鍙犲姞鐢辨簮鐭╁舰涓庣洰鏍囩煩褰㈡帶鍒躲€傛簮鐭╁舰閫夋嫨瑕佸彔鍔犵殑甯х紦鍐插浘鍍忕殑
涓€涓瓙鍖哄煙锛岀洰鏍囩煩褰㈤€夋嫨鍥惧儚灏嗗嚭鐜扮殑浼犲嚭瑙嗛淇″彿涓殑涓€涓尯鍩熴€?椹卞姩鍙兘鏀寔涔熷彲鑳戒笉鏀寔缂╂斁锛屼互鍙婅繖浜涚煩褰㈢殑浠绘剰澶у皬鍜屼綅缃€?姝ゅ锛岄┍鍔ㄥ彲鑳芥敮鎸侊紙涔熷彲鑳戒笉鏀寔锛変负瑙嗛鍙犲姞 <overlay> 鎺ュ彛瀹氫箟鐨?浠讳綍锛堟垨娌℃湁锛夎鍓?娣峰悎鏂规硶銆?
struct `v4l2_window` 瀹氫箟婧愮煩褰㈢殑澶у皬銆佸畠鍦ㄥ抚缂撳啿涓殑浣嶇疆锛?浠ュ強鐢ㄤ簬鍙犲姞鐨勮鍓?娣峰悎鏂规硶銆傝鑾峰彇褰撳墠鍙傛暟锛屽簲鐢ㄧ▼搴忓皢
struct `v4l2_format` 鐨?`type` 瀛楁璁剧疆涓?`V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY` 骞惰皟鐢?VIDIOC_G_FMT <VIDIOC_G_FMT> ioctl銆傞┍鍔ㄥ～鍏呭悕涓?`win` 鐨?struct `v4l2_window` 瀛愮粨鏋勩€傛棤娉曞彇鍥炲厛鍓嶇紪绋嬬殑瑁佸壀鍒楄〃鎴栦綅鍥俱€?
瑕佺紪绋嬫簮鐭╁舰锛屽簲鐢ㄧ▼搴忓皢 struct `v4l2_format` 鐨?`type` 瀛楁
璁剧疆涓?`V4L2_BUF_TYPE_VIDEO_OUTPUT_OVERLAY`锛屽垵濮嬪寲 `win`
瀛愮粨鏋勫苟璋冪敤 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl銆?椹卞姩鏍规嵁纭欢闄愬埗璋冩暣鍙傛暟锛屽苟鍍?VIDIOC_G_FMT <VIDIOC_G_FMT> 閭ｆ牱
杩斿洖瀹為檯鍙傛暟銆備笌 VIDIOC_S_FMT <VIDIOC_G_FMT> 绫讳技锛?VIDIOC_TRY_FMT <VIDIOC_G_FMT> ioctl 鍙敤浜庡湪涓嶅疄闄呮敼鍙橀┍鍔ㄧ姸鎬佺殑
鎯呭喌涓嬩簡瑙ｉ┍鍔ㄨ兘鍔涖€備笌 VIDIOC_S_FMT <VIDIOC_G_FMT> 涓嶅悓锛岃繖涔熷彲浠ュ湪
鍙犲姞鍚敤涔嬪悗宸ヤ綔銆?
struct `v4l2_crop` 瀹氫箟鐩爣鐭╁舰鐨勫ぇ灏忎笌浣嶇疆銆傚彔鍔犵殑缂╂斁鍥犲瓙
鐢?struct `v4l2_window` 涓?struct `v4l2_crop` 涓粰瀹氱殑瀹藉害鍜岄珮搴?闅愬惈銆傝鍓?API 瀵?*瑙嗛杈撳嚭**涓?*瑙嗛杈撳嚭鍙犲姞**璁惧鐨勫簲鐢ㄦ柟寮忥紝
涓庡**瑙嗛鎹曡幏**涓?*瑙嗛鍙犲姞**璁惧鐩稿悓锛屽彧鏄弽杞簡
鏁版嵁娴佺殑鏂瑰悜銆傛洿澶氫俊鎭鍙傝 crop銆?
## 鍚敤鍙犲姞


娌℃湁鐢ㄤ簬鍚敤鎴栫鐢ㄥ彔鍔犵殑 V4L2 ioctl锛屼絾椹卞姩鐨勫抚缂撳啿鎺ュ彛
鍙兘鏀寔 `FBIOBLANK` ioctl銆?