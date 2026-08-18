


######## ioctl VIDIOC_G_PRIORITY, VIDIOC_S_PRIORITY


## 鍚嶇О


VIDIOC_G_PRIORITY - VIDIOC_S_PRIORITY - 鏌ヨ鎴栬姹備笌鏂囦欢鎻忚堪绗﹀叧鑱旂殑璁块棶浼樺厛绾?
## 姒傝



`int ioctl(int fd, VIDIOC_G_PRIORITY, enum v4l2_priority *argp)`


`int ioctl(int fd, VIDIOC_S_PRIORITY, const enum v4l2_priority *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 enum `v4l2_priority` 绫诲瀷鐨勬寚閽堛€?
## 鎻忚堪


瑕佹煡璇㈠綋鍓嶇殑璁块棶浼樺厛绾э紝搴旂敤绋嬪簭璋冪敤 VIDIOC_G_PRIORITY <VIDIOC_G_PRIORITY> ioctl锛屽苟浼犲叆涓€涓寚鍚?enum v4l2_priority 鍙橀噺鐨勬寚閽堬紝椹卞姩灏嗘妸褰撳墠浼樺厛绾у瓨鍏ュ叾涓€?
瑕佽姹傛煇涓闂紭鍏堢骇锛屽簲鐢ㄧ▼搴忓皢鏈熸湜鐨勪紭鍏堢骇瀛樺叆涓€涓?enum v4l2_priority 鍙橀噺锛屽苟璋冪敤 VIDIOC_S_PRIORITY <VIDIOC_G_PRIORITY> ioctl锛屼紶鍏ユ寚鍚戣鍙橀噺鐨勬寚閽堛€?


    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - `V4L2_PRIORITY_UNSET`
      - 0
      -
    - - `V4L2_PRIORITY_BACKGROUND`
      - 1
      - 鏈€浣庝紭鍏堢骇锛岄€氬父鏄悗鍙拌繍琛岀殑搴旂敤绋嬪簭锛屼緥濡傜洃瑙?VBI 浼犺緭銆傚鏋滃涓簲鐢ㄧ▼搴忔兂浠?	姝や紭鍏堢骇浠庤澶囪鍙栵紝鍒欓渶瑕佷竴涓繍琛屽湪鐢ㄦ埛绌洪棿鐨勪唬鐞嗗簲鐢ㄧ▼搴忋€?    - - `V4L2_PRIORITY_INTERACTIVE`
      - 2
      -
    - - `V4L2_PRIORITY_DEFAULT`
      - 2
      - 涓瓑浼樺厛绾э紝閫氬父鏄敤鎴峰惎鍔ㄥ苟浜や簰鎺у埗鐨勫簲鐢ㄧ▼搴忋€備緥濡傜數瑙嗘煡鐪嬪櫒銆佸浘鏂囩數瑙嗭紙Teletext锛?	娴忚鍣紝鎴栦粎鐢ㄤ簬鏀瑰彉棰戦亾鎴栬棰戞帶鍒剁殑鈥滈潰鏉库€濆簲鐢ㄧ▼搴忋€傞櫎闈炴煇搴旂敤绋嬪簭璇锋眰浜嗗叾浠栦紭鍏堢骇锛?	鍚﹀垯杩欐槸榛樿浼樺厛绾с€?    - - `V4L2_PRIORITY_RECORD`
      - 3
      - 鏈€楂樹紭鍏堢骇銆傚彧鏈変竴涓枃浠舵弿杩扮鍙互鍏锋湁姝や紭鍏堢骇锛屽畠浼氶樆姝换浣曞叾浠?fd 鏀瑰彉璁惧灞炴€с€?	閫氬父鏄儚瑙嗛褰曞埗杩欐牱涓嶈兘琚腑鏂殑搴旂敤绋嬪簭銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    璇锋眰鐨勪紭鍏堢骇鍊兼棤鏁堛€?
EBUSY
    鍙︿竴涓簲鐢ㄧ▼搴忓凡缁忚姹備簡鏇撮珮鐨勪紭鍏堢骇銆?