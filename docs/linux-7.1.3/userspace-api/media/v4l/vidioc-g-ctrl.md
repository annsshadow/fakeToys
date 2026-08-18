


######## ioctl VIDIOC_G_CTRL, VIDIOC_S_CTRL


## 鍚嶇О锛圢ame锛?

VIDIOC_G_CTRL - VIDIOC_S_CTRL - 鑾峰彇鎴栬缃煇涓帶浠剁殑鍊?
## 姒傝锛圫ynopsis锛?

`int ioctl(int fd, VIDIOC_G_CTRL, struct v4l2_control *argp)`


`int ioctl(int fd, VIDIOC_S_CTRL, struct v4l2_control *argp)`

## 鍙傛暟锛圓rguments锛?

`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_control` 鐨勬寚閽堛€?
## 鎻忚堪锛圖escription锛?

涓轰簡鑾峰彇鏌愪釜鎺т欢鐨勫綋鍓嶅€硷紝搴旂敤绋嬪簭鍒濆鍖栦竴涓?struct `v4l2_control` 鐨?`id` 瀛楁锛?骞剁敤鎸囧悜璇ョ粨鏋勭殑鎸囬拡璋冪敤 VIDIOC_G_CTRL <VIDIOC_G_CTRL> ioctl銆備负浜嗘洿鏀规煇涓帶浠?鐨勫€硷紝搴旂敤绋嬪簭鍒濆鍖?struct `v4l2_control` 鐨?`id` 涓?`value` 瀛楁锛屽苟璋冪敤
VIDIOC_S_CTRL <VIDIOC_G_CTRL> ioctl銆?
褰?`id` 鏃犳晥鏃讹紝椹卞姩杩斿洖 `EINVAL` 閿欒鐮併€傚綋 `value` 瓒呭嚭鑼冨洿鏃讹紝椹卞姩鍙互閫夋嫨閲囩敤
鏈€鎺ヨ繎鐨勬湁鏁堝€硷紝鎴栬繑鍥?`ERANGE` 閿欒鐮侊紝浠ョ湅璧锋潵鏇村悎閫傝€呬负鍑嗐€傜劧鑰岋紝VIDIOC_S_CTRL
<VIDIOC_G_CTRL> 鏄竴涓彧鍐?ioctl锛屽畠涓嶄細杩斿洖瀹為檯鐨勬柊鍊笺€傚鏋?`value` 瀵逛簬璇ユ帶浠?涓嶅悎閫傦紙渚嬪锛屽畠寮曠敤浜嗚彍鍗曟帶浠朵竴涓笉鍙楁敮鎸佺殑鑿滃崟绱㈠紩锛夛紝閭ｄ箞涔熶細杩斿洖 EINVAL 閿欒鐮併€?
杩欎簺 ioctl 浠呴€傜敤浜庣敤鎴锋帶浠躲€傚浜庡叾瀹冩帶浠剁被锛屽繀椤讳娇鐢?VIDIOC_G_EXT_CTRLS
<VIDIOC_G_EXT_CTRLS>銆乂IDIOC_S_EXT_CTRLS <VIDIOC_G_EXT_CTRLS> 鎴?VIDIOC_TRY_EXT_CTRLS <VIDIOC_G_EXT_CTRLS>銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `id`
      - 鏍囪瘑鎺т欢锛岀敱搴旂敤绋嬪簭璁剧疆銆?    - - __s32
      - `value`
      - 鏂板€兼垨褰撳墠鍊笺€?
## 杩斿洖鍊硷紙Return Value锛?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟閫傚綋鍦拌缃?`errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
EINVAL
    缁撴瀯 `v4l2_control` 鐨?`id` 鏃犳晥锛屾垨 `value` 瀵逛簬缁欏畾鎺т欢涓嶅悎閫傦紙鍗筹紝鏍规嵁
    VIDIOC_QUERYMENU <VIDIOC_QUERYCTRL> 閫夋嫨浜嗛┍鍔ㄤ笉鏀寔鐨勮彍鍗曢」锛夈€?
ERANGE
    缁撴瀯 `v4l2_control` 鐨?`value` 瓒呭嚭鑼冨洿銆?
EBUSY
    璇ユ帶浠舵殏鏃朵笉鍙洿鏀癸紝鍙兘鏄洜涓哄彟涓€涓簲鐢ㄧ▼搴忔帴绠′簡姝ゆ帶浠舵墍灞炵殑璁惧鍔熻兘銆?
EACCES
    璇曞浘璁剧疆鍙鎺т欢锛屾垨鑾峰彇鍙啓鎺т欢銆?
    鎴栬€咃紝濡傛灉瀛樺湪璇曞浘璁剧疆涓€涓潪娲诲姩鎺т欢鐨勬搷浣滐紝鑰岄┍鍔ㄦ棤娉曞湪鎺т欢鍐嶆婵€娲讳箣鍓?    缂撳瓨鏂板€笺€?