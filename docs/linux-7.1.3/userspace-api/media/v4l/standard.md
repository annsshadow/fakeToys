######## 瑙嗛鏍囧噯

瑙嗛璁惧閫氬父鏀寔涓€涓垨澶氫釜涓嶅悓鐨勮棰戞爣鍑嗘垨鏍囧噯鐨勫彉浣撱€傛瘡涓棰戣緭鍏ュ拰杈撳嚭鍙兘鏀寔鍙︿竴缁勬爣鍑嗐€傝繖缁勬爣鍑嗙敱 VIDIOC_ENUMINPUT 鍜?VIDIOC_ENUMOUTPUT ioctl 鍒嗗埆杩斿洖鐨?struct `v4l2_input` 鍜?struct `v4l2_output` 鐨?`std` 瀛楁鎶ュ憡銆?
V4L2 涓哄綋鍓嶅叏鐞冧娇鐢ㄧ殑姣忕妯℃嫙瑙嗛鏍囧噯瀹氫箟浜嗕竴浣嶏紝骞朵负椹卞姩瀹氫箟鐨勬爣鍑嗛鐣欎簡浣嶏紝渚嬪鐢ㄤ簬鍦?PAL 鐢佃涓婅鐪?NTSC 褰曞儚甯︼紙鍙嶄箣浜︾劧锛夌殑娣峰悎鏍囧噯銆傚簲鐢ㄧ▼搴忓彲浠ヤ娇鐢ㄩ瀹氫箟鐨勪綅鏉ラ€夋嫨鐗瑰畾鏍囧噯锛屽敖绠℃洿鎺ㄨ崘鍚戠敤鎴峰睍绀轰竴涓彈鏀寔鏍囧噯鑿滃崟銆傝鏋氫妇骞舵煡璇㈠彈鏀寔鏍囧噯鐨勫睘鎬э紝搴旂敤绋嬪簭浣跨敤 VIDIOC_ENUMSTD ioctl銆?
璁稿宸插畾涔夌殑鏍囧噯瀹為檯涓婂彧鏄皯鏁板嚑涓富瑕佹爣鍑嗙殑鍙樹綋銆傜‖浠跺疄闄呬笂鍙兘涓嶅尯鍒嗗畠浠紝鎴栬€呭湪鍐呴儴鍖哄垎骞惰嚜鍔ㄥ垏鎹€傚洜姝ゆ灇涓惧嚭鐨勬爣鍑嗕篃鍖呭惈涓€缁勬垨澶氱粍鏍囧噯浣嶃€?
鍋囪涓€涓亣鎯崇殑璋冭皭鍣ㄨ兘澶熻В璋?B/PAL銆丟/PAL 鍜?I/PAL 淇″彿銆傜涓€涓灇涓惧嚭鐨勬爣鍑嗘槸涓€缁?B 鍜?G/PAL锛屾牴鎹湪 UHF 鎴?VHF 娉㈡閫夋嫨鐨勫皠棰戣嚜鍔ㄥ垏鎹€傛灇涓剧粰鍑?"PAL-B/G" 鎴?"PAL-I" 閫夐」銆傜被浼煎湴锛屼竴涓鍚堣緭鍏ュ彲鑳戒細鍚堝苟鏍囧噯锛屾灇涓惧嚭 "PAL-B/G/H/I"銆?NTSC-M" 鍜?"SECAM-D/K"銆俒#f1]_

瑕佹煡璇㈠拰閫夋嫨褰撳墠瑙嗛杈撳叆鎴栬緭鍑烘墍浣跨敤鐨勬爣鍑嗭紝搴旂敤绋嬪簭鍒嗗埆璋冪敤 VIDIOC_G_STD <VIDIOC_G_STD> 鍜?VIDIOC_S_STD <VIDIOC_G_STD> ioctl銆傝**鎺ユ敹**鍒扮殑鏍囧噯鍙互閫氳繃 VIDIOC_QUERYSTD ioctl 鎰熺煡銆?
   鎵€鏈夎繖浜?ioctl 鐨勫弬鏁伴兘鏄竴涓寚鍚?v4l2_std_id <v4l2-std-id> 绫诲瀷锛堜竴涓爣鍑嗛泦鍚堬級鐨勬寚閽堬紝**鑰屼笉鏄?*鏍囧噯鏋氫妇涓殑绱㈠紩銆傚綋璁惧鍏锋湁涓€涓垨澶氫釜瑙嗛杈撳叆鎴栬緭鍑烘椂锛岄┍鍔ㄥ繀椤诲疄鐜版墍鏈夎棰戞爣鍑?ioctl銆?
瀵逛簬璇稿 USB 鎽勫儚澶磋繖绫昏棰戞爣鍑嗘蹇垫剰涔変笉澶х殑璁惧锛屾湁鐗规畩鐨勮鍒欍€傛洿涓€鑸湴璇达紝瀵逛簬浠讳綍鎹曡幏鎴栬緭鍑鸿澶囷紝濡傛灉瀹冿細

- 鏃犳硶浠ヨ棰戞爣鍑嗙殑鏍囩О閫熺巼鎹曡幏鍦烘垨甯э紝鎴栬€?
- 鏍规湰涓嶆敮鎸佽棰戞爣鍑嗘牸寮忋€?
鍦ㄦ锛岄┍鍔ㄥ簲灏?struct `v4l2_input` 鍜?struct `v4l2_output` 鐨?`std` 瀛楁璁句负闆讹紝骞朵笖 VIDIOC_G_STD <VIDIOC_G_STD>銆乂IDIOC_S_STD <VIDIOC_G_STD>銆乂IDIOC_QUERYSTD 鍜?VIDIOC_ENUMSTD ioctl 搴旇繑鍥?`ENOTTY` 閿欒鐮佹垨 `EINVAL` 閿欒鐮併€?
搴旂敤绋嬪簭鍙互鍒╃敤 input-capabilities 鍜?output-capabilities 鏍囧織鏉ョ‘瀹氳棰戞爣鍑?ioctl 鏄惁鍙笌缁欏畾鐨勮緭鍏ユ垨杈撳嚭涓€璧蜂娇鐢ㄣ€?
## 绀轰緥锛氬叧浜庡綋鍓嶈棰戞爣鍑嗙殑淇℃伅

    v4l2_std_id std_id;
    struct v4l2_standard standard;

    if (-1 == ioctl(fd, VIDIOC_G_STD, &std_id)) {
	/* Note when VIDIOC_ENUMSTD always returns ENOTTY this
	   is no video device or it falls under the USB exception,
	   and VIDIOC_G_STD returning ENOTTY is no error. */

	perror("VIDIOC_G_STD");
	exit(EXIT_FAILURE);
    }

    memset(&standard, 0, sizeof(standard));
    standard.index = 0;

    while (0 == ioctl(fd, VIDIOC_ENUMSTD, &standard)) {
	if (standard.id & std_id) {
	       printf("Current video standard: %s\\n", standard.name);
	       exit(EXIT_SUCCESS);
	}

	standard.index++;
    }

    /* EINVAL indicates the end of the enumeration, which cannot be
       empty unless this device falls under the USB exception. */

    if (errno == EINVAL || standard.index == 0) {
	perror("VIDIOC_ENUMSTD");
	exit(EXIT_FAILURE);
    }

## 绀轰緥锛氬垪鍑哄綋鍓嶈緭鍏ユ敮鎸佺殑瑙嗛鏍囧噯

    struct v4l2_input input;
    struct v4l2_standard standard;

    memset(&input, 0, sizeof(input));

    if (-1 == ioctl(fd, VIDIOC_G_INPUT, &input.index)) {
	perror("VIDIOC_G_INPUT");
	exit(EXIT_FAILURE);
    }

    if (-1 == ioctl(fd, VIDIOC_ENUMINPUT, &input)) {
	perror("VIDIOC_ENUM_INPUT");
	exit(EXIT_FAILURE);
    }

    printf("Current input %s supports:\\n", input.name);

    memset(&standard, 0, sizeof(standard));
    standard.index = 0;

    while (0 == ioctl(fd, VIDIOC_ENUMSTD, &standard)) {
	if (standard.id & input.std)
	    printf("%s\\n", standard.name);

	standard.index++;
    }

    /* EINVAL indicates the end of the enumeration, which cannot be
       empty unless this device falls under the USB exception. */

    if (errno != EINVAL || standard.index == 0) {
	perror("VIDIOC_ENUMSTD");
	exit(EXIT_FAILURE);
    }

## 绀轰緥锛氶€夋嫨涓€涓柊鐨勮棰戞爣鍑?
    struct v4l2_input input;
    v4l2_std_id std_id;

    memset(&input, 0, sizeof(input));

    if (-1 == ioctl(fd, VIDIOC_G_INPUT, &input.index)) {
	perror("VIDIOC_G_INPUT");
	exit(EXIT_FAILURE);
    }

    if (-1 == ioctl(fd, VIDIOC_ENUMINPUT, &input)) {
	perror("VIDIOC_ENUM_INPUT");
	exit(EXIT_FAILURE);
    }

    if (0 == (input.std & V4L2_STD_PAL_BG)) {
	fprintf(stderr, "Oops. B/G PAL is not supported.\\n");
	exit(EXIT_FAILURE);
    }

    /* Note this is also supposed to work when only B
       or G/PAL is supported. */

    std_id = V4L2_STD_PAL_BG;

    if (-1 == ioctl(fd, VIDIOC_S_STD, &std_id)) {
	perror("VIDIOC_S_STD");
	exit(EXIT_FAILURE);
    }

   涓€浜涚敤鎴峰凡缁忚 PAL銆丯TSC 鍜?SECAM 杩欎簺鎶€鏈湳璇悶绯婃秱浜嗐€傚綋杞欢鎴栫‖浠跺彲浠ヨ嚜鍔ㄥ畬鎴愭椂锛屾病鏈夊繀瑕佽姹備粬浠幓鍖哄垎 B銆丟銆丏 鎴?K銆?