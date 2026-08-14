


######## 璋冭皭鍣ㄤ笌璋冨埗鍣?


## 璋冭皭鍣?

瑙嗛杈撳叆璁惧鍙互鎷ユ湁涓€涓垨澶氫釜瑙ｈ皟 RF 淇″彿鐨勮皟璋愬櫒銆傛瘡涓皟璋愬櫒鍏宠仈涓€涓垨澶氫釜
瑙嗛杈撳叆锛屽叿浣撳彇鍐充簬璋冭皭鍣ㄤ笂 RF 杩炴帴鍣ㄧ殑鏁伴噺銆傜敱 VIDIOC_ENUMINPUT ioctl 杩斿洖鐨?鐩稿簲缁撴瀯浣?`v4l2_input` 鐨?`type` 瀛楁琚涓?`V4L2_INPUT_TYPE_TUNER`锛屽叾
`tuner` 瀛楁鍖呭惈璇ヨ皟璋愬櫒鐨勭储寮曞彿銆?
灏勯杈撳叆璁惧鎭板ソ鏈変竴涓储寮曚负 0 鐨勮皟璋愬櫒锛屾病鏈夎棰戣緭鍏ャ€?
搴旂敤绋嬪簭浣跨敤 VIDIOC_G_TUNER <VIDIOC_G_TUNER> 鍜?VIDIOC_S_TUNER <VIDIOC_G_TUNER> ioctl 鍒嗗埆鏌ヨ鍜屾洿鏀硅皟璋愬櫒灞炴€с€俈IDIOC_G_TUNER <VIDIOC_G_TUNER>
杩斿洖鐨?`v4l2_tuner` 缁撴瀯浣撹繕鍖呭惈褰撳墠瑙嗛鎴栧皠棰戣緭鍏ユ墍瀵瑰簲鐨勮皟璋愬櫒琚煡璇㈡椂
閫傜敤鐨勪俊鍙风姸鎬佷俊鎭€?

   VIDIOC_S_TUNER <VIDIOC_G_TUNER> 鍦ㄦ湁澶氫釜璋冭皭鍣ㄦ椂骞朵笉浼氬垏鎹㈠綋鍓嶈皟璋愬櫒銆傝皟璋愬櫒
   瀹屽叏鐢卞綋鍓嶈棰戣緭鍏ュ喅瀹氥€傚綋璁惧鎷ユ湁涓€涓垨澶氫釜璋冭皭鍣ㄦ椂锛岄┍鍔ㄥ繀椤诲悓鏃舵敮鎸佽繖涓や釜
   ioctl锛屽苟鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?`v4l2_capability` 缁撴瀯浣撲腑璁剧疆
   `V4L2_CAP_TUNER` 鏍囧織銆?

## 璋冨埗鍣?

瑙嗛杈撳嚭璁惧鍙互鎷ユ湁涓€涓垨澶氫釜璋冨埗鍣紝鐢ㄤ簬灏嗚棰戜俊鍙疯皟鍒跺悗杈愬皠鍑哄幓锛屾垨杩炴帴鍒?鐢佃鏈烘垨褰曞儚鏈虹殑澶╃嚎杈撳叆绔€傛瘡涓皟鍒跺櫒鍏宠仈涓€涓垨澶氫釜瑙嗛杈撳嚭锛屽叿浣撳彇鍐充簬璋冨埗鍣?涓?RF 杩炴帴鍣ㄧ殑鏁伴噺銆傜敱 VIDIOC_ENUMOUTPUT ioctl 杩斿洖鐨勭浉搴?`v4l2_output` 缁撴瀯浣撶殑 `type` 瀛楁琚涓?`V4L2_OUTPUT_TYPE_MODULATOR`锛屽叾
`modulator` 瀛楁鍖呭惈璇ヨ皟鍒跺櫒鐨勭储寮曞彿銆?
灏勯杈撳嚭璁惧鎭板ソ鏈変竴涓储寮曚负 0 鐨勮皟鍒跺櫒锛屾病鏈夎棰戣緭鍑恒€?
瑙嗛鎴栧皠棰戣澶囦笉鑳藉悓鏃舵敮鎸佽皟璋愬櫒鍜岃皟鍒跺櫒銆傛绫荤‖浠跺繀椤讳娇鐢ㄤ袱涓嫭绔嬬殑璁惧鑺傜偣锛?涓€涓敮鎸佽皟璋愬櫒鍔熻兘锛屼竴涓敮鎸佽皟鍒跺櫒鍔熻兘銆傚師鍥犲湪浜?VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY>
ioctl 鐨勯檺鍒讹細鏃犳硶鎸囨槑棰戠巼鏄粰璋冭皭鍣ㄨ繕鏄皟鍒跺櫒浣跨敤銆?
搴旂敤绋嬪簭浣跨敤 VIDIOC_G_MODULATOR <VIDIOC_G_MODULATOR> 鍜?VIDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> ioctl 鏌ヨ鍜屾洿鏀硅皟鍒跺櫒灞炴€с€傛敞鎰忥紝褰撳瓨鍦ㄥ涓?璋冨埗鍣ㄦ椂锛孷IDIOC_S_MODULATOR <VIDIOC_G_MODULATOR> 骞朵笉浼氬垏鎹㈠綋鍓嶈皟鍒跺櫒銆傝皟鍒跺櫒瀹屽叏鐢卞綋鍓嶈棰戣緭鍑哄喅瀹氥€傚綋璁惧鎷ユ湁
涓€涓垨澶氫釜璋冨埗鍣ㄦ椂锛岄┍鍔ㄥ繀椤诲悓鏃舵敮鎸佽繖涓や釜 ioctl锛屽苟鍦?VIDIOC_QUERYCAP ioctl 杩斿洖鐨?`v4l2_capability` 缁撴瀯浣撲腑璁剧疆
`V4L2_CAP_MODULATOR` 鏍囧織銆?

## 灏勯


搴旂敤绋嬪簭浣跨敤 VIDIOC_G_FREQUENCY <VIDIOC_G_FREQUENCY> 鍜?VIDIOC_S_FREQUENCY <VIDIOC_G_FREQUENCY> ioctl 鏉ヨ幏鍙栧拰璁剧疆璋冭皭鍣ㄦ垨璋冨埗鍣ㄧ殑灏勯棰戠巼锛?杩欎袱涓?ioctl 閮芥帴鍙椾竴涓寚鍚?`v4l2_frequency` 缁撴瀯浣撶殑鎸囬拡銆傝繖浜?ioctl 鍚屾牱閫傜敤浜?鐢佃鍜屽皠棰戣澶囥€傚綋鏀寔璋冭皭鍣ㄦ垨璋冨埗鍣?ioctl锛屾垨璁惧涓哄皠棰戣澶囨椂锛岄┍鍔ㄥ繀椤诲悓鏃?鏀寔杩欎袱涓?ioctl銆?
