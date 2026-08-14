


######## ioctl VIDIOC_G_EDID, VIDIOC_S_EDID, VIDIOC_SUBDEV_G_EDID, VIDIOC_SUBDEV_S_EDID


## 鍚嶇О


VIDIOC_G_EDID - VIDIOC_S_EDID - VIDIOC_SUBDEV_G_EDID - VIDIOC_SUBDEV_S_EDID - 鑾峰彇鎴栬缃棰戞帴鏀跺櫒/鍙戦€佸櫒鐨?EDID

## 姒傝


`int ioctl(int fd, VIDIOC_G_EDID, struct v4l2_edid *argp)`


`int ioctl(int fd, VIDIOC_S_EDID, struct v4l2_edid *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_G_EDID, struct v4l2_edid *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_S_EDID, struct v4l2_edid *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
   鎸囧悜 struct `v4l2_edid` 鐨勬寚閽堛€?
## 鎻忚堪


杩欎簺 ioctl 鍙敤浜庤幏鍙栨垨璁剧疆涓庢帴鏀跺櫒鐨勮緭鍏ユ垨鍙戦€佸櫒璁惧鐨勮緭鍑虹浉鍏宠仈鐨?EDID銆傚畠浠彲浠ヤ笌瀛愯澶囪妭鐐?锛?dev/v4l-subdevX锛夋垨瑙嗛鑺傜偣锛?dev/videoX锛変竴璧蜂娇鐢ㄣ€?
涓庤棰戣妭鐐逛竴璧蜂娇鐢ㄦ椂锛宍pad` 瀛楁琛ㄧず杈撳叆锛堝浜庤棰戦噰闆嗚澶囷級鎴栬緭鍑猴紙瀵逛簬瑙嗛杈撳嚭璁惧锛夌储寮曪紝鍒嗗埆鐢?VIDIOC_ENUMINPUT 鍜?VIDIOC_ENUMOUTPUT 杩斿洖銆備笌瀛愯澶囪妭鐐逛竴璧蜂娇鐢ㄦ椂锛宍pad` 瀛楁琛ㄧず瀛愯澶囩殑杈撳叆鎴?杈撳嚭 pad銆傚鏋滃浜庣粰瀹氱殑 `pad` 鍊兼病鏈?EDID 鏀寔锛屽垯灏嗚繑鍥?`EINVAL` 閿欒鐮併€?
瑕佽幏鍙?EDID 鏁版嵁锛屽簲鐢ㄧ▼搴忓繀椤诲～鍐?`pad`銆乣start_block`銆乣blocks` 鍜?`edid` 瀛楁锛屽皢 `reserved`
鏁扮粍娓呴浂锛屽苟璋冪敤 VIDIOC_G_EDID <VIDIOC_G_EDID>銆備粠 `start_block` 鍧楀紑濮嬨€佸ぇ灏忎负 `blocks` 鐨勫綋鍓?EDID 灏嗚鏀惧叆 `edid` 鎸囧悜鐨勫唴瀛樹腑銆俙edid` 鎸囬拡蹇呴』鎸囧悜鑷冲皯 `blocks` * 128 瀛楄妭澶у皬鐨勫唴瀛橈紙涓€涓潡鐨?澶у皬涓?128 瀛楄妭锛夈€?
濡傛灉鍧楁暟灏戜簬鎸囧畾鐨勬暟閲忥紝鍒欓┍鍔ㄤ細灏?`blocks` 璁剧疆涓哄疄闄呯殑鍧楁暟銆傚鏋滄牴鏈病鏈変换浣?EDID 鍧楀彲鐢紝鍒欒缃?閿欒鐮?`ENODATA`銆?
濡傛灉鍧楀繀椤讳粠 sink 鑾峰彇锛屽垯姝よ皟鐢ㄥ皢闃诲锛岀洿鍒板畠浠璇诲彇銆?
濡傛灉鍦ㄨ皟鐢?VIDIOC_G_EDID <VIDIOC_G_EDID> 鏃?`start_block` 鍜?`blocks` 閮借缃负 0锛屽垯椹卞姩浼氬皢
`blocks` 璁剧疆涓哄彲鐢ㄧ殑 EDID 鍧楁€绘暟锛屽苟杩斿洖 0 鑰屼笉澶嶅埗浠讳綍鏁版嵁銆傝繖鏄彂鐜版湁澶氬皯涓?EDID 鍧楃殑绠€鍗曟柟娉曘€?

   濡傛灉娌℃湁浠讳綍 EDID 鍧楀彲鐢紝鍒欓┍鍔ㄤ細灏?`blocks` 璁剧疆涓?0 骞惰繑鍥?0銆?
瑕佽缃帴鏀跺櫒鐨?EDID 鍧楋紝搴旂敤绋嬪簭蹇呴』濉啓 `pad`銆乣blocks` 鍜?`edid` 瀛楁锛屽皢 `start_block` 璁剧疆涓?0锛?骞跺皢 `reserved` 鏁扮粍娓呴浂銆備笉鍙兘鍙缃?EDID 鐨勪竴閮ㄥ垎锛屽畠鎬绘槸鍏ㄦ湁鎴栧叏鏃犮€傝缃?EDID 鏁版嵁浠呭鎺ユ敹鍣ㄦ湁鏁堬紝
鍥犱负瀵瑰彂閫佸櫒鏉ヨ娌℃湁鎰忎箟銆?
椹卞姩鍋囧畾浼犲叆鐨勬槸瀹屾暣鐨?EDID銆傚鏋?EDID 鍧楀浜庣‖浠惰兘澶勭悊鐨勬暟閲忥紝鍒欎笉浼氬啓鍏?EDID锛岃€屾槸璁剧疆閿欒鐮?`E2BIG`锛屽苟涓?`blocks` 琚缃负纭欢鏀寔鐨勬渶澶у€笺€傚鏋?`start_block` 涓?0 浠ュ鐨勪换浣曞€硷紝鍒欒缃敊璇爜
`EINVAL`銆?
瑕佺鐢?EDID锛屼綘灏?`blocks` 璁剧疆涓?0銆傛牴鎹‖浠剁殑涓嶅悓锛岃繖浼氬皢鐑彃鎷斿紩鑴氭媺浣庡拰/鎴栦互鏌愮鏂瑰紡闃绘婧愯鍙?EDID
鏁版嵁銆傛棤璁哄浣曪紝鏈€缁堢粨鏋滄槸鐩稿悓鐨勶細EDID 涓嶅啀鍙敤銆?

    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u32
      - `pad`
      - 瑕佽幏鍙?璁剧疆 EDID 鍧楃殑 pad銆備笌瑙嗛璁惧鑺傜偣涓€璧蜂娇鐢ㄦ椂锛宲ad 琛ㄧず杈撳叆鎴栬緭鍑虹储寮曪紝鍒嗗埆鐢?	VIDIOC_ENUMINPUT 鍜?VIDIOC_ENUMOUTPUT 杩斿洖銆?    - - __u32
      - `start_block`
      - 浠庢鍧楀紑濮嬭鍙?EDID銆傝缃?EDID 鏃跺繀椤讳负 0銆?    - - __u32
      - `blocks`
      - 瑕佽幏鍙栨垨璁剧疆鐨勫潡鏁般€傚繀椤诲皬浜庢垨绛変簬 256锛堟爣鍑嗗畾涔夌殑鏈€澶у潡鏁帮級銆傚綋浣犺缃?EDID 涓?`blocks` 涓?0
	鏃讹紝鍒?EDID 琚鐢ㄦ垨鎿﹂櫎銆?    - - __u32
      - `reserved`\ [^5^]
      - 涓烘湭鏉ユ墿灞曚繚鐣欍€傚簲鐢ㄧ▼搴忓拰椹卞姩蹇呴』灏嗘暟缁勮缃负闆躲€?    - - __u8 *
      - `edid`
      - 鎸囧悜鍖呭惈 EDID 鐨勫唴瀛樸€傛渶灏忓ぇ灏忎负 `blocks` * 128銆?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?
`ENODATA`
    EDID 鏁版嵁涓嶅彲鐢ㄣ€?
`E2BIG`
    浣犳彁渚涚殑 EDID 鏁版嵁瓒呰繃浜嗙‖浠惰兘澶勭悊鐨勬暟閲忋€?