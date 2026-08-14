

######## 瑙嗛杈撳叆涓庤緭鍑?

瑙嗛杈撳叆鍜岃緭鍑烘槸璁惧鐨勭墿鐞嗚繛鎺ュ櫒銆傝繖浜涘彲浠ユ槸渚嬪锛歊F 杩炴帴鍣紙澶╃嚎/鏈夌嚎鐢佃锛夈€丆VBS锛堝張绉板鍚堣棰戯級銆丼-Video 鍜?RGB 杩炴帴鍣ㄣ€傜浉鏈轰紶鎰熷櫒涔熻瑙嗕负涓€绉嶈棰戣緭鍏ャ€傝棰戝拰 VBI 鎹曡幏璁惧鍏锋湁杈撳叆銆傝棰戝拰 VBI 杈撳嚭璁惧鍏锋湁杈撳嚭锛屽悇鑷嚦灏戞湁涓€涓€傛棤绾跨數璁惧娌℃湁瑙嗛杈撳叆鎴栬緭鍑恒€?
瑕佷簡瑙ｅ彲鐢ㄨ緭鍏ュ拰杈撳嚭鐨勬暟閲忎笌灞炴€э紝搴旂敤绋嬪簭鍙互鍒嗗埆浣跨敤
VIDIOC_ENUMINPUT 鍜?VIDIOC_ENUMOUTPUT ioctl 鏉ユ灇涓惧畠浠€俈IDIOC_ENUMINPUT
ioctl 杩斿洖鐨?`v4l2_input` 缁撴瀯浣撹繕鍖呭惈閫傜敤浜庢煡璇㈠綋鍓嶈棰戣緭鍏ユ椂鐨勪俊鍙风姸鎬佷俊鎭€?
VIDIOC_G_INPUT <VIDIOC_G_INPUT> 鍜?VIDIOC_G_OUTPUT <VIDIOC_G_OUTPUT> ioctl 杩斿洖褰撳墠瑙嗛杈撳叆鎴栬緭鍑虹殑绱㈠紩銆傝閫夋嫨涓嶅悓鐨勮緭鍏ユ垨杈撳嚭锛屽簲鐢ㄧ▼搴忚皟鐢?VIDIOC_S_INPUT <VIDIOC_G_INPUT> 鍜?VIDIOC_S_OUTPUT <VIDIOC_G_OUTPUT> ioctl銆傚綋璁惧鍏锋湁涓€涓垨澶氫釜杈撳叆鏃讹紝椹卞姩蹇呴』瀹炵幇鎵€鏈夎緭鍏?ioctl锛涘綋璁惧鍏锋湁涓€涓垨澶氫釜杈撳嚭鏃讹紝蹇呴』瀹炵幇鎵€鏈夎緭鍑?ioctl銆?
## 绀轰緥锛氬叧浜庡綋鍓嶈棰戣緭鍏ョ殑淇℃伅



    struct v4l2_input input;
    int index;

    if (-1 == ioctl(fd, VIDIOC_G_INPUT, &index)) {
	perror("VIDIOC_G_INPUT");
	exit(EXIT_FAILURE);
    }

    memset(&input, 0, sizeof(input));
    input.index = index;

    if (-1 == ioctl(fd, VIDIOC_ENUMINPUT, &input)) {
	perror("VIDIOC_ENUMINPUT");
	exit(EXIT_FAILURE);
    }

    printf("Current input: %s\n", input.name);


## 绀轰緥锛氬垏鎹㈠埌绗竴涓棰戣緭鍏?


    int index;

    index = 0;

    if (-1 == ioctl(fd, VIDIOC_S_INPUT, &index)) {
	perror("VIDIOC_S_INPUT");
	exit(EXIT_FAILURE);
    }
