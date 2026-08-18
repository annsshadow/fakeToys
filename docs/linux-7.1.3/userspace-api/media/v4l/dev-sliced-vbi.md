


######## 鍒囩墖 VBI 鏁版嵁鎺ュ彛


VBI 鏄?Vertical Blanking Interval锛堝満娑堥殣闂撮殧锛夌殑缂╁啓锛屽嵆妯℃嫙瑙嗛淇″彿鍚勮搴忓垪涔嬮棿鐨勪竴涓棿闅欍€?鍦?VBI 鏈熼棿涓嶄紶杈撳浘鍍忎俊鎭紝杩欎负闃存瀬灏勭嚎绠＄數瑙嗙殑鐢靛瓙鏉熻繑鍥炲睆骞曢《閮ㄧ暀鍑轰簡涓€浜涙椂闂淬€?
鍒囩墖 VBI 璁惧浣跨敤纭欢瑙ｈ皟鍦?VBI 涓紶杈撶殑鏁版嵁銆俈4L2 椹卞姩**涓嶅簲**閫氳繃杞欢鏉ュ畬鎴愭宸ヤ綔锛屽彟璇峰弬瑙?鍘熷 VBI 鎺ュ彛 <raw-vbi>銆傛暟鎹互鍥哄畾澶у皬鐨勭煭鏁版嵁鍖呭舰寮忎紶閫掞紝姣忎釜鏁版嵁鍖呰鐩栦竴琛屾壂鎻忕嚎銆?姣忚棰戝抚鐨勬暟鎹寘鏁伴噺鏄彲鍙樼殑銆?
鍒囩墖 VBI 鎹曡幏鍜岃緭鍑鸿澶囬€氳繃鍜屽師濮?VBI 璁惧鐩稿悓鐨勫瓧绗︾壒娈婃枃浠惰繘琛岃闂€傚綋椹卞姩鍚屾椂鏀寔杩欎袱绉嶆帴鍙ｆ椂锛?`/dev/vbi` 璁惧鐨勯粯璁ゅ姛鑳芥槸**鍘熷** VBI 鎹曡幏鎴栬緭鍑猴紝鍒囩墖 VBI 鍔熻兘浠呭湪璋冪敤濡備笅瀹氫箟鐨?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鍚庢墠鍙敤銆傚悓鏍凤紝`/dev/video` 璁惧涔熷彲鑳芥敮鎸佸垏鐗?VBI API锛?浣嗘澶勭殑榛樿鍔熻兘鏄棰戞崟鑾锋垨杈撳嚭銆傚鏋滈┍鍔ㄦ敮鎸侊紝蹇呴』浣跨敤涓嶅悓鐨勬枃浠舵弿杩扮鏉ュ悓鏃朵紶閫掑師濮嬪拰鍒囩墖 VBI 鏁版嵁銆?
## 鏌ヨ鑳藉姏


鏀寔鍒囩墖 VBI 鎹曡幏鎴栬緭鍑?API 鐨勮澶囧垎鍒缃?`v4l2_capability` 缁撴瀯浣?`capabilities` 瀛楁涓殑 `V4L2_CAP_SLICED_VBI_CAPTURE` 鎴?`V4L2_CAP_SLICED_VBI_OUTPUT` 鏍囧織锛岃缁撴瀯浣撶敱
VIDIOC_QUERYCAP ioctl 杩斿洖銆傚繀椤昏嚦灏戞敮鎸佷竴绉?read/write 鎴栨祦寮?I/O 鏂规硶 <io>銆傚垏鐗?VBI 璁惧鍙兘甯︽湁璋冭皭鍣ㄦ垨璋冨埗鍣ㄣ€?
## 杈呭姪鍔熻兘


鍒囩墖 VBI 璁惧搴斿綋鏀寔瑙嗛杈撳叆鎴栬緭鍑?<video> 浠ュ強璋冭皭鍣ㄦ垨璋冨埗鍣?<tuner> ioctl
锛堝鏋滃畠浠叿澶囪繖浜涜兘鍔涳級锛屽苟涓斿彲鑳芥敮鎸佹帶鍒?ioctl銆傝棰戞爣鍑?<standard> ioctl 鎻愪緵浜?缂栫▼鍒囩墖 VBI 璁惧鎵€闇€鐨勫叧閿俊鎭紝鍥犳蹇呴』鏀寔銆?

## 鍒囩墖 VBI 鏍煎紡鍗忓晢


瑕佷簡瑙ｇ‖浠舵敮鎸佸摢浜涙暟鎹湇鍔★紝搴旂敤绋嬪簭鍙互璋冪敤
VIDIOC_G_SLICED_VBI_CAP <VIDIOC_G_SLICED_VBI_CAP> ioctl銆?鎵€鏈夊疄鐜颁簡鍒囩墖 VBI 鎺ュ彛鐨勯┍鍔ㄩ兘蹇呴』鏀寔姝?ioctl銆傚綋纭欢姣忓抚鑳藉鎹曡幏鎴栬緭鍑虹殑 VBI 琛屾暟锛?鎴栧叾鑳藉鍦ㄧ粰瀹氳涓婅瘑鍒殑鏈嶅姟鏁伴噺鍙楀埌闄愬埗鏃讹紝缁撴灉鍙兘涓?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鐨勭粨鏋滀笉鍚屻€備緥濡傚湪 PAL 鐨勭 16 琛屼笂锛?纭欢鍙兘鑳藉鏌ユ壘 VPS 鎴栧浘鏂囩數瑙嗭紙Teletext锛変俊鍙凤紝浣嗕笉鑳藉悓鏃舵煡鎵句袱鑰呫€?
瑕佺‘瀹氬綋鍓嶉€夋嫨鐨勬湇鍔★紝搴旂敤绋嬪簭灏?`v4l2_format` 缁撴瀯浣撶殑 `type` 瀛楁璁剧疆涓?`V4L2_BUF_TYPE_SLICED_VBI_CAPTURE` 鎴?`V4L2_BUF_TYPE_SLICED_VBI_OUTPUT`锛岀劧鍚?VIDIOC_G_FMT <VIDIOC_G_FMT>
ioctl 浼氬～鍏?`fmt.sliced` 鎴愬憳锛屽嵆涓€涓?`v4l2_sliced_vbi_format` 缁撴瀯浣撱€?
搴旂敤绋嬪簭鍙互閫氳繃鍒濆鍖栨垨淇敼 `fmt.sliced` 鎴愬憳锛屽苟璋冪敤鎸囧悜
`v4l2_format` 缁撴瀯浣撶殑 VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鏉ヨ姹備笉鍚岀殑鍙傛暟銆?
鍒囩墖 VBI API 姣斿師濮?VBI API 鏇村鏉傦紝鍥犱负蹇呴』鍛婅瘔纭欢鍦ㄦ瘡涓€琛屾壂鎻忕嚎涓婃湡鏈涘摢绉?VBI 鏈嶅姟銆傚苟闈炴墍鏈?鏈嶅姟閮借兘琚‖浠跺湪鎵€鏈夎涓婃敮鎸侊紙瀵逛簬 VBI 杈撳嚭灏ゅ叾濡傛锛屽叾涓浘鏂囩數瑙嗛€氬父涓嶅彈鏀寔锛岃€屽叾浠栨湇鍔″彧鑳芥彃鍏ュ埌
鐗瑰畾鐨勬煇涓€琛岋級銆傜劧鑰屽湪璁稿鎯呭喌涓嬶紝鍙渶灏?`service_set` 瀛楁璁剧疆涓烘墍闇€鐨勬湇鍔★紝骞惰椹卞姩鏍规嵁
纭欢鑳藉姏鏉ュ～鍏?`service_lines` 鏁扮粍灏辫冻澶熶簡銆傚彧鏈夊湪闇€瑕佹洿绮剧‘鐨勬帶鍒舵椂锛岀▼搴忓憳鎵嶅簲鏄惧紡璁剧疆
`service_lines` 鏁扮粍銆?
VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 浼氭牴鎹‖浠惰兘鍔涗慨鏀瑰弬鏁般€傚綋椹卞姩鍦ㄦ鍒诲垎閰嶈祫婧愭椂锛屽鏋?鎵€闇€璧勬簮鏆傛椂涓嶅彲鐢紝瀹冨彲鑳借繑鍥?`EBUSY` 閿欒鐮併€傚叾浠栧彲鑳借繑鍥?`EBUSY` 鐨勮祫婧愬垎閰嶇偣鍖呮嫭
VIDIOC_STREAMON ioctl 浠ュ強绗竴娆?`read()`銆乣write()` 鍜?`select()` 璋冪敤銆?

### struct v4l2_sliced_vbi_format



    \begingroup
    \scriptsize
    \setlength{\tabcolsep}{2pt}



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 3 2 2 2

    - - __u16
      - `service_set`
      - `2`

	濡傛灉鍦ㄤ娇鐢?VIDIOC_S_FMT <VIDIOC_G_FMT> 鎴?	VIDIOC_TRY_FMT <VIDIOC_G_FMT> 浼犻€掓椂 `service_set` 闈為浂锛?	鍒?`service_lines` 鏁扮粍灏嗙敱椹卞姩鏍规嵁姝ゅ瓧娈典腑鎸囧畾鐨勬湇鍔¤繘琛屽～鍏呫€?	渚嬪锛屽鏋?`service_set` 琚垵濮嬪寲涓?`V4L2_SLICED_TELETEXT_B | V4L2_SLICED_WSS_625`锛?	cx25840 瑙嗛瑙ｇ爜鍣ㄧ殑椹卞姩浼氬皢涓や釜鍦?[#f1]_ 鐨勭 7-22 琛岃缃负
	`V4L2_SLICED_TELETEXT_B`锛屽苟灏嗙涓€涓満鐨勭 23 琛岃缃负
	`V4L2_SLICED_WSS_625`銆傚鏋?`service_set` 琚缃负闆讹紝鍒欏皢鏀圭敤
	`service_lines` 鐨勫€笺€?
	杩斿洖鏃讹紝椹卞姩灏嗘瀛楁璁剧疆涓鸿繑鍥炵殑 `service_lines` 鏁扮粍涓墍鏈夊厓绱犵殑骞堕泦銆?	濡傛灉纭欢鏃犳硶鍚屾椂澶勭悊鏇村鏈嶅姟锛屽畠鍙兘鍖呭惈姣旇姹傛洿灏戠殑鏈嶅姟锛屼篃璁稿彧鏈変竴涓€?	濡傛灉鎵€璇锋眰鐨勬湇鍔″潎涓嶅彈纭欢鏀寔锛屽畠鍙兘涓虹┖锛堥浂锛夈€?    - - __u16
      - `service_lines`\ [^2^][^24^]
      - `2`

	搴旂敤绋嬪簭鐢ㄩ┍鍔ㄥ簲褰撳湪鐩稿簲鎵弿琛屼笂鏌ユ壘鎴栨彃鍏ョ殑鏁版嵁鏈嶅姟闆嗗悎鏉ュ垵濮嬪寲姝ゆ暟缁勩€?	鍙楃‖浠惰兘鍔涢檺鍒讹紝椹卞姩浼氳繑鍥炴墍璇锋眰鐨勯泦鍚堛€佷竴涓瓙闆嗭紙鍙兘鍙槸涓€涓湇鍔★級鎴栦竴涓┖闆嗐€?	褰撶‖浠舵棤娉曞湪鍚屼竴琛屼笂澶勭悊澶氫釜鏈嶅姟鏃讹紝椹卞姩搴斿綋閫夋嫨鍏朵腑涓€涓€傛棤娉曞亣瀹氶┍鍔ㄤ細閫夋嫨鍝釜鏈嶅姟銆?
	鏁版嵁鏈嶅姟鍦?vbi-services2 涓畾涔夈€傛暟缁勭储寮曟槧灏勫埌 ITU-R 琛屽彿\ [#f2]_锛屽涓嬫墍绀猴細
#     * -

      - Element
      - 525 line systems
      - 625 line systems
#     * -

      - `service_lines`\ [^0^][^1^]
      - 1
      - 1
#     * -

      - `service_lines`\ [^0^][^23^]
      - 23
      - 23
#     * -

      - `service_lines`\ [^1^][^1^]
      - 264
      - 314
#     * -

      - `service_lines`\ [^1^][^23^]
      - 286
      - 336
#     * -

      - `2` 椹卞姩蹇呴』灏?`service_lines` [^0^][^0^] 鍜?	`service_lines`\ [^1^][^0^] 璁句负闆躲€?	`V4L2_VBI_ITU_525_F1_START`銆乣V4L2_VBI_ITU_525_F2_START`銆?	`V4L2_VBI_ITU_625_F1_START` 鍜?`V4L2_VBI_ITU_625_F2_START`
	鐨勫畾涔夊垎鍒粰鍑轰簡姣忕 525 鎴?625 琛屾牸寮忓悇涓満鐨勮捣濮嬭鍙凤紝浠ユ柟渚夸娇鐢ㄣ€備笉瑕佸繕璁?	ITU 琛屽彿浠?1 寮€濮嬶紝鑰屼笉鏄?0銆?    - - __u32
      - `io_size`
      - `2` 涓€娆?`read()` 鎴?`write()` 璋冪敤鎵€浼犻€掔殑鏈€澶у瓧鑺傛暟锛?	浠ュ強 VIDIOC_QBUF 鍜?	VIDIOC_DQBUF <VIDIOC_QBUF> ioctl 鐨勭紦鍐插尯澶у皬锛堜互瀛楄妭涓哄崟浣嶏級銆?	椹卞姩灏嗘瀛楁璁剧疆涓?`v4l2_sliced_vbi_data` 缁撴瀯浣撳ぇ灏忎箻浠ヨ繑鍥炵殑
	`service_lines` 鏁扮粍涓潪闆跺厓绱犵殑鏁伴噺锛堝嵆鍙兘鎼哄甫鏁版嵁鐨勮鏁帮級銆?    - - __u32
      - `reserved`\ [^2^]
      - `2` 姝ゆ暟缁勪负鏈潵鎵╁睍鑰屼繚鐣欍€?
	搴旂敤绋嬪簭鍜岄┍鍔ㄥ繀椤诲皢鍏惰缃负闆躲€?

    \endgroup


### 鍒囩墖 VBI 鏈嶅姟



    \footnotesize


    :header-rows:  1
    :stub-columns: 0
    :widths:       2 1 1 2 2

    - - Symbol
      - Value
      - Reference
      - Lines, usually
      - Payload
    - - `V4L2_SLICED_TELETEXT_B` (Teletext System B)
      - 0x0001
      - ets300706,

	itu653
      - PAL/SECAM 绗?7-22 琛岋紝320-335锛堢浜屼釜鍦?7-22锛?      - 45 瀛楄妭鍥炬枃鐢佃鏁版嵁鍖呬腑鐨勬渶鍚?42 涓瓧鑺傦紝鍗充笉鍚椂閽熷鍏ュ拰鎴愬抚鐮侊紝
	鏈€浣庢湁鏁堜綅鍏堜紶杈撱€?    - - `V4L2_SLICED_VPS`
      - 0x0400
      - ets300231
      - PAL 绗?16 琛?      - 鏍规嵁 ETS 300 231 鍥?9锛屼粠绗?3 瀛楄妭鍒扮 15 瀛楄妭锛屾渶浣庢湁鏁堜綅鍏堜紶杈撱€?    - - `V4L2_SLICED_CAPTION_525`
      - 0x1000
      - cea608
      - NTSC 绗?21 琛岋紝284锛堢浜屼釜鍦?21锛?      - 鎸変紶杈撻『搴忕殑涓や釜瀛楄妭锛屽寘鍚鍋舵牎楠屼綅锛屾渶浣庢湁鏁堜綅鍏堜紶杈撱€?    - - `V4L2_SLICED_WSS_625`
      - 0x4000
      - itu1119,

	en300294
      - PAL/SECAM 绗?23 琛?      - 璇峰弬瑙佷笅闈㈢殑 v4l2-sliced-wss-625-payload銆?    - - `V4L2_SLICED_VBI_525`
      - 0x1000
      - `2` 閫傜敤浜?525 琛岀郴缁熺殑鏈嶅姟闆嗗悎銆?    - - `V4L2_SLICED_VBI_625`
      - 0x4401
      - `2` 閫傜敤浜?625 琛岀郴缁熺殑鏈嶅姟闆嗗悎銆?

    \normalsize

椹卞姩鍦ㄥ簲鐢ㄧ▼搴忓皾璇曞湪娌℃湁浜嬪厛杩涜鏍煎紡鍗忓晢鐨勬儏鍐典笅璇诲彇鎴栧啓鍏ユ暟鎹€佸湪鍒囨崲瑙嗛鏍囧噯涔嬪悗锛堣繖鍙兘浣垮崗鍟嗙殑
VBI 鍙傛暟澶辨晥锛変互鍙婂湪鍒囨崲瑙嗛杈撳叆涔嬪悗锛堣繖鍙兘浣滀负鍓綔鐢ㄦ敼鍙樿棰戞爣鍑嗭級鏃讹紝鍙兘杩斿洖 `EINVAL` 閿欒鐮併€?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鍦ㄥ簲鐢ㄧ▼搴忓皾璇曞湪 I/O 杩涜鏈熼棿锛堝湪
VIDIOC_STREAMON 鍜?VIDIOC_STREAMOFF <VIDIOC_STREAMON> 璋冪敤涔嬮棿锛屼互鍙婄涓€娆?`read()` 鎴?`write()` 璋冪敤涔嬪悗锛夋洿鏀规牸寮忔椂锛屽彲鑳借繑鍥?`EBUSY` 閿欒鐮併€?

#### V4L2_SLICED_WSS_625 璐熻浇


`V4L2_SLICED_WSS_625` 鐨勮礋杞戒负锛?
           +-----+------------------+-----------------------+
	   |Byte |        0         |           1           |
           +-----+--------+---------+-----------+-----------+
	   |     | msb    | lsb     | msb       | lsb       |
           |     +-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+
	   | Bit |7|6|5|4 | 3|2|1|0 | x|x|13|12 | 11|10|9|8 |
           +-----+-+-+-+--+--+-+-+--+--+-+--+---+---+--+-+--+

## 璇诲彇鍜屽啓鍏ュ垏鐗?VBI 鏁版嵁


涓€娆?`read()` 鎴?`write()` 璋冪敤蹇呴』浼犻€掑睘浜庝竴涓棰戝抚鐨勬墍鏈夋暟鎹€傚嵆涓€涓?`v4l2_sliced_vbi_data` 缁撴瀯浣撴暟缁勶紝鍖呭惈涓€涓垨澶氫釜鍏冪礌锛屼笖鎬诲ぇ灏忎笉瓒呰繃 `io_size` 瀛楄妭銆?鍚屾牱锛屽湪娴佸紡 I/O 妯″紡涓嬶紝涓€涓?`io_size` 瀛楄妭鐨勭紦鍐插尯蹇呴』鍖呭惈涓€甯ц棰戠殑鏁版嵁銆?鏈娇鐢ㄧ殑 `v4l2_sliced_vbi_data` 鍏冪礌鐨?`id` 蹇呴』涓洪浂銆?

### struct v4l2_sliced_vbi_data



    :header-rows:  0
    :stub-columns: 0
    :widths:       3 1 4

    - - __u32
      - `id`
      - 鏉ヨ嚜 vbi-services 鐨勪竴涓爣蹇楋紝鏍囪瘑姝ゆ暟鎹寘涓暟鎹殑绫诲瀷銆傚繀椤诲彧璁剧疆涓€涓綅銆?	褰撴崟鑾锋暟鎹寘鐨?`id` 涓洪浂鏃讹紝璇ユ暟鎹寘涓虹┖锛屽叾浠栧瓧娈电殑鍐呭鏈畾涔夈€傚簲鐢ㄧ▼搴忓簲褰撳拷鐣?	绌烘暟鎹寘銆傚綋鐢ㄤ簬杈撳嚭鐨勬暟鎹寘鐨?`id` 涓洪浂鏃讹紝`data` 瀛楁鐨勫唴瀹规湭瀹氫箟锛岄┍鍔ㄥ繀椤?	涓嶅啀鍦ㄨ姹傜殑 `field` 鍜?`line` 涓婃彃鍏ユ暟鎹€?    - - __u32
      - `field`
      - 姝ゆ暟鎹鎹曡幏鑷垨灏嗚琚彃鍏ュ埌鐨勮棰戝満缂栧彿銆俙0` 琛ㄧず绗竴涓満锛宍1` 琛ㄧず绗簩涓満銆?    - - __u32
      - `line`
      - 姝ゆ暟鎹鎹曡幏鑷垨灏嗚琚彃鍏ュ埌鐨勫満锛堢浉瀵逛簬甯ц€岃█锛夎鍙枫€傛湁鏁堝€艰鍙傝 vbi-525 鍜?	vbi-625銆傚鏋滅‖浠舵棤娉曞彲闈犺瘑鍒壂鎻忚锛屽垏鐗?VBI 鎹曡幏璁惧鍙互灏嗘墍鏈夋暟鎹寘鐨勮鍙疯缃负
	`0`銆傚満缂栧彿蹇呴』濮嬬粓鏈夋晥銆?    - - __u32
      - `reserved`
      - 姝ゅ瓧娈典负鏈潵鎵╁睍鑰屼繚鐣欍€傚簲鐢ㄧ▼搴忓拰椹卞姩蹇呴』灏嗗叾璁剧疆涓洪浂銆?    - - __u8
      - `data`\ [^48^]
      - 鏁版嵁鍖呰礋杞姐€傛瘡绉嶆暟鎹被鍨嬩紶閫掔殑鍐呭鍜屽瓧鑺傛暟瑙?vbi-services銆傛鏁扮粍鏈熬濉厖瀛楄妭鐨?	鍐呭鏈畾涔夛紝椹卞姩鍜屽簲鐢ㄧ▼搴忓簲褰撳拷鐣ュ畠浠€?
鏁版嵁鍖呭缁堟寜琛屽彿鍗囧簭浼犻€掞紝娌℃湁閲嶅鐨勮鍙枫€傚綋搴旂敤绋嬪簭杩濆弽姝よ鍒欐椂锛宍write()` 鍑芥暟鍜?VIDIOC_QBUF ioctl 蹇呴』杩斿洖 `EINVAL` 閿欒鐮併€傚綋搴旂敤绋嬪簭浼犻€掍簡涓嶆纭殑鍦烘垨琛屽彿锛屾垨鑰呬紶閫掍簡
鏈笌 VIDIOC_G_FMT <VIDIOC_G_FMT> 鎴?VIDIOC_S_FMT <VIDIOC_G_FMT> ioctl 鍗忓晢杩囩殑 `field`銆乣line` 鍜?`id` 鐨勭粍鍚堟椂锛?瀹冧滑涔熷繀椤昏繑鍥?EINVAL 閿欒鐮併€傚綋琛屽彿鏈煡鏃讹紝椹卞姩蹇呴』鎸変紶杈撻『搴忎紶閫掓暟鎹寘銆傞┍鍔ㄥ彲浠ュ湪鏁版嵁鍖呮暟缁勭殑
浠绘剰浣嶇疆鎻掑叆 `id` 璁句负闆剁殑绌烘暟鎹寘銆?
涓轰簡纭繚鍚屾骞跺尯鍒簬涓㈠抚锛屽綋鎹曡幏鐨勫抚涓嶅寘鍚换浣曟墍璇锋眰鐨勬暟鎹湇鍔℃椂锛岄┍鍔ㄥ繀椤讳紶閫掍竴涓垨澶氫釜绌烘暟鎹寘銆?褰撳簲鐢ㄧ▼搴忔湭鑳藉強鏃朵紶閫?VBI 鏁版嵁浠ヨ繘琛岃緭鍑烘椂锛岄┍鍔ㄥ繀椤诲啀娆¤緭鍑烘渶鍚庝竴涓?VPS 鍜?WSS 鏁版嵁鍖咃紝骞剁鐢?闅愯棌瀛楀箷锛圕losed Caption锛夊拰鍥炬枃鐢佃鏁版嵁鐨勮緭鍑猴紝鎴栬€呰緭鍑鸿闅愯棌瀛楀箷鍜屽浘鏂囩數瑙嗚В鐮佸櫒蹇界暐鐨勬暟鎹€?
鍒囩墖 VBI 璁惧鍙兘鏀寔 read/write <rw> 鍜?鎴栨祦寮忥紙鍐呭瓨鏄犲皠 <mmap> 鍜?鎴?鐢ㄦ埛鎸囬拡 <userp>锛塈/O銆傚悗鑰呮彁渚涗簡鍒╃敤缂撳啿鍖烘椂闂存埑鏉ュ悓姝ヨ棰戝拰 VBI 鏁版嵁鐨勫彲鑳芥€с€?
## MPEG 娴佷腑鐨勫垏鐗?VBI 鏁版嵁


濡傛灉璁惧鑳藉浜х敓 MPEG 杈撳嚭娴侊紝瀹冨彲鑳借兘澶熸彁渚?鍗忓晢杩囩殑鍒囩墖 VBI 鏈嶅姟 <sliced-vbi-format-negotiation>锛屼綔涓哄祵鍏ュ湪 MPEG 娴佷腑鐨勬暟鎹€?鐢ㄦ埛鎴栧簲鐢ㄧ▼搴忎娇鐢?V4L2_CID_MPEG_STREAM_VBI_FMT <v4l2-mpeg-stream-vbi-fmt>
鎺у埗椤规潵鎺у埗杩欑鍒囩墖 VBI 鏁版嵁鐨勬彃鍏ャ€?
濡傛灉椹卞姩涓嶆彁渚?V4L2_CID_MPEG_STREAM_VBI_FMT <v4l2-mpeg-stream-vbi-fmt>
鎺у埗椤癸紝鎴栬€呭彧鍏佽灏嗚鎺у埗椤硅缃负
V4L2_MPEG_STREAM_VBI_FMT_NONE <v4l2-mpeg-stream-vbi-fmt>锛?鍒欒澶囨棤娉曞皢鍒囩墖 VBI 鏁版嵁宓屽叆鍒?MPEG 娴佷腑銆?
V4L2_CID_MPEG_STREAM_VBI_FMT <v4l2-mpeg-stream-vbi-fmt>
鎺у埗椤逛笉浼氶殣寮忓湴璁╄澶囬┍鍔ㄦ崟鑾锋垨鍋滄鎹曡幏鍒囩墖 VBI 鏁版嵁銆傝鎺у埗椤逛粎鎸囩ず鍦?MPEG 娴佷腑宓屽叆鍒囩墖 VBI 鏁版嵁
锛堝鏋滃簲鐢ㄧ▼搴忓凡鍗忓晢鎹曡幏鏌愮鍒囩墖 VBI 鏈嶅姟锛夈€?
涔熷彲鑳藉嚭鐜拌澶囧彧鑳藉皢鍒囩墖 VBI 鏁版嵁宓屽叆鏌愪簺绫诲瀷鐨?MPEG 娴佷腑鐨勬儏鍐碉細渚嬪鍦?MPEG-2 PS 涓彲浠ワ紝浣嗗湪
MPEG-2 TS 涓笉琛屻€傚湪杩欑鎯呭喌涓嬶紝濡傛灉璇锋眰浜嗗垏鐗?VBI 鏁版嵁鎻掑叆锛屽垏鐗?VBI 鏁版嵁灏嗚宓屽叆鍒板彈鏀寔鐨?MPEG
娴佺被鍨嬩腑锛屽苟鍦ㄨ澶囦笉鏀寔鍒囩墖 VBI 鏁版嵁鎻掑叆鐨?MPEG 娴佺被鍨嬩腑琚潤榛樼渷鐣ャ€?
浠ヤ笅灏忚妭瑙勫畾浜嗗祵鍏ョ殑鍒囩墖 VBI 鏁版嵁鐨勬牸寮忋€?
### MPEG 娴佸祵鍏ョ殑鍒囩墖 VBI 鏁版嵁鏍煎紡锛歂ONE


V4L2_MPEG_STREAM_VBI_FMT_NONE <v4l2-mpeg-stream-vbi-fmt>
宓屽叆鍒囩墖 VBI 鏍煎紡搴旇椹卞姩瑙ｉ噴涓哄仠姝㈠湪 MPEG 娴佷腑宓屽叆鍒囩墖 VBI 鏁版嵁鐨勬帶鍒堕」銆傝缃鏍煎紡鏃讹紝璁惧鎴栭┍鍔?閮戒笉搴斿湪 MPEG 娴佷腑鎻掑叆鈥滅┖鐨勨€濆祵鍏ュ垏鐗?VBI 鏁版嵁鍖呫€傛鏍煎紡鏈瀹氫换浣?MPEG 娴佹暟鎹粨鏋勩€?
### MPEG 娴佸祵鍏ョ殑鍒囩墖 VBI 鏁版嵁鏍煎紡锛欼VTV


褰撳彈鏀寔鏃讹紝V4L2_MPEG_STREAM_VBI_FMT_IVTV <v4l2-mpeg-stream-vbi-fmt>
宓屽叆鍒囩墖 VBI 鏍煎紡鎸囩ず椹卞姩鍦?MPEG 娴佷腑锛屼簬灏佽鍦?MPEG-2 **Program Pack**锛堢▼搴忓寘锛変腑鐨?MPEG-2 *Private Stream 1 PES**锛堢鏈夋祦 1 PES锛夋暟鎹寘鍐咃紝姣忓抚宓屽叆鏈€澶?36 琛屽垏鐗?VBI 鏁版嵁銆?
**鍘嗗彶鑳屾櫙**锛氭鏍煎紡瑙勮寖婧愯嚜 `ivtv` 椹卞姩浣跨敤鐨勪竴绉嶈嚜瀹氫箟鐨勩€佸祵鍏ュ紡鐨勫垏鐗?VBI 鏁版嵁鏍煎紡銆?璇ユ牸寮忓凡鍦ㄥ唴鏍告簮鐮佹枃浠?`Documentation/userspace-api/media/drivers/cx2341x-uapi.rst` 涓闈炴寮忓湴瑙勫畾銆傛鏍煎紡鐨?璐熻浇鏈€澶уぇ灏忎互鍙婂叾瀹冩柟闈紝鐢?CX23415 MPEG 瑙ｇ爜鍣ㄥ湪鎻愬彇銆佽В鐮佸拰鏄剧ず宓屽叆鍦?MPEG 娴佷腑鐨勫垏鐗?VBI 鏁版嵁
鏂归潰鐨勮兘鍔涘拰闄愬埗鎵€鍐冲畾銆?
姝ゆ牸寮忕殑浣跨敤**骞堕潪** `ivtv` 椹卞姩鎵€**鐙崰**锛屼篃**骞堕潪** CX2341x 璁惧鎵€鐙崰锛屽洜涓哄皢鍒囩墖 VBI 鏁版嵁鍖?鎻掑叆鍒?MPEG 娴佷腑鏄敱椹卞姩杞欢瀹炵幇鐨勩€傝嚦灏?`cx18` 椹卞姩涔熶互杩欑鏍煎紡鎻愪緵浜嗗悜 MPEG-2 PS 涓彃鍏ュ垏鐗?VBI 鏁版嵁銆?
浠ヤ笅瀹氫箟瑙勫畾浜嗗綋璁剧疆浜?V4L2_MPEG_STREAM_VBI_FMT_IVTV <v4l2-mpeg-stream-vbi-fmt>
鏃讹紝鍖呭惈鍒囩墖 VBI 鏁版嵁鐨?MPEG-2 *Private Stream 1 PES* 鏁版嵁鍖呯殑璐熻浇銆?锛堟澶勪笉璇﹁堪 MPEG-2 **Private Stream 1 PES** 鏁版嵁鍖呭ご鍜屽皝瑁呯殑 MPEG-2 **Program Pack** 鍖呭ご銆?鏈夊叧杩欎簺鏁版嵁鍖呭ご鐨勮缁嗕俊鎭紝璇峰弬闃?MPEG-2 瑙勮寖銆傦級

鍖呭惈鍒囩墖 VBI 鏁版嵁鐨?MPEG-2 **Private Stream 1 PES** 鏁版嵁鍖呯殑璐熻浇鐢?`v4l2_mpeg_vbi_fmt_ivtv` 缁撴瀯浣撹瀹氥€傝礋杞介暱搴︽槸鍙彉鐨勶紝鍙栧喅浜庤棰戝抚涓瓨鍦ㄧ殑鍒囩墖 VBI 鏁版嵁鐨勫疄闄呰鏁般€?璐熻浇鏈熬鍙互鐢ㄦ湭鎸囧畾鐨勫～鍏呭瓧鑺傝繘琛屽～鍏咃紝浠ヤ娇璐熻浇鏈熬瀵归綈鍒?4 瀛楄妭杈圭晫銆傝礋杞界粷涓嶅簲瓒呰繃 1552 瀛楄妭
锛? 涓満锛屾瘡涓満 18 琛岋紝姣忚 43 瀛楄妭鏁版嵁锛屽鍔犱竴涓?4 瀛楄妭鐨勯瓟鏁帮級銆?

### struct v4l2_mpeg_vbi_fmt_ivtv



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `magic`\ [^4^]
      - 鏉ヨ嚜 v4l2-mpeg-vbi-fmt-ivtv-magic 鐨勪竴涓€滈瓟鏁扳€濆父閲忥紝鐢ㄤ簬琛ㄦ槑杩欐槸涓€涓湁鏁堢殑
	鍒囩墖 VBI 鏁版嵁璐熻浇锛屽苟鎸囩ず鍖垮悕鑱斿悎鐨勫摢涓垚鍛?`itv0` 鎴?`ITV0` 鐢ㄤ簬璐熻浇鏁版嵁銆?    - - union {
      - (anonymous)
    - - struct `v4l2_mpeg_vbi_itv0`
      - `itv0`
      - 鍒囩墖 VBI 鏁版嵁璐熻浇鐨勪富瑕佸舰寮忥紝鍖呭惈 1 鍒?35 琛屽垏鐗?VBI 鏁版嵁銆傝繖绉嶅舰寮忕殑璐熻浇涓彁渚?	浜嗚鎺╃爜锛屾寚绀烘彁渚涗簡鍝簺 VBI 琛屻€?    - - struct v4l2_mpeg_vbi_ITV0 <v4l2-mpeg-vbi-itv0-1>
      - `ITV0`
      - 褰撳瓨鍦?36 琛屽垏鐗?VBI 鏁版嵁鏃朵娇鐢ㄧ殑鍒囩墖 VBI 鏁版嵁璐熻浇鐨勫彟涓€绉嶅舰寮忋€傝繖绉嶅舰寮忕殑璐熻浇涓笉鎻愪緵
	琛屾帺鐮侊紱鎵€鏈夋湁鏁堢殑琛屾帺鐮佷綅閮借闅愬紡璁剧疆銆?    - - }
      -


### struct v4l2_mpeg_vbi_fmt_ivtv magic 瀛楁鐨勯瓟鏁板父閲?


    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4

    - - Defined Symbol
      - Value
      - Description
    - - `V4L2_MPEG_VBI_IVTV_MAGIC0`
      - "itv0"
      - 琛ㄦ槑 `v4l2_mpeg_vbi_fmt_ivtv` 缁撴瀯浣撲腑鑱斿悎鐨?`itv0` 鎴愬憳
	鏈夋晥銆?    - - `V4L2_MPEG_VBI_IVTV_MAGIC1`
      - "ITV0"
      - 琛ㄦ槑 `v4l2_mpeg_vbi_fmt_ivtv` 缁撴瀯浣撲腑鑱斿悎鐨?`ITV0` 鎴愬憳
	鏈夋晥锛屽苟涓斿瓨鍦?36 琛屽垏鐗?VBI 鏁版嵁銆?


### structs v4l2_mpeg_vbi_itv0 鍜?v4l2_mpeg_vbi_ITV0



   \footnotesize


    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __le32
      - `linemask`\ [^2^]
      - 鎸囩ず瀛樺湪鐨?VBI 鏈嶅姟琛岀殑浣嶆帺鐮併€傝繖浜?`linemask` 鍊煎湪 MPEG 娴佷腑浠ュ皬绔瓧鑺傚簭瀛樺偍銆?	涓嬮潰缁欏嚭浜嗕竴浜?`linemask` 浣嶄綅缃強鍏跺搴旂殑 VBI 琛屽彿鍜岃棰戝満銆俠\ `0` 琛ㄧず
	`linemask` 鍊肩殑鏈€浣庢湁鏁堜綅锛?

```

	    linemask[0] b0:     line  6  first field
	    linemask[0] b17:    line 23  first field
	    linemask[0] b18:    line  6  second field
	    linemask[0] b31:    line 19  second field
	    linemask[1] b0:     line 20  second field
	    linemask[1] b3:     line 23  second field
	    linemask[1] b4-b31: unused and set to 0
    * - struct
	:c:type:`v4l2_mpeg_vbi_itv0_line`
      - ``line``\ [35]
      - 杩欐槸涓€涓彲鍙橀暱搴︽暟缁勶紝淇濆瓨 1 鍒?35 琛屽垏鐗?VBI 鏁版嵁銆傚瓨鍦ㄧ殑鍒囩墖 VBI 鏁版嵁琛屽搴斾簬
	``linemask`` 鏁扮粍涓缃殑浣嶏紝浠?``linemask``\ [0] 鐨?b\ :sub:`0` 寮€濮嬶紝涓€鐩村埌
	``linemask``\ [0] 鐨?b\ :sub:`31`锛屽啀浠?``linemask``\ [1] 鐨?b\ :sub:`0` 寮€濮嬶紝
	涓€鐩村埌 ``linemask``\ [1] 鐨?b\ :sub:`3`銆俙`line``\ [0] 瀵瑰簲浜庡湪 ``linemask`` 鏁扮粍
	涓壘鍒扮殑绗竴涓璁剧疆鐨勪綅锛宍`line``\ [1] 瀵瑰簲浜庢壘鍒扮殑绗簩涓璁剧疆鐨勪綅锛屼緷姝ょ被鎺ㄣ€傚鏋?	娌℃湁璁剧疆 ``linemask`` 鏁扮粍鐨勪綅锛屽垯 ``line``\ [0] 鍙兘鍖呭惈涓€琛屽簲鐢ㄧ▼搴忓簲蹇界暐鐨?	鏈寚瀹氭暟鎹€?
```

   \normalsize


### struct v4l2_mpeg_vbi_ITV0



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - struct
	`v4l2_mpeg_vbi_itv0_line`
      - `line`\ [^36^]
      - 涓€涓浐瀹氶暱搴︿负 36 琛岀殑鍒囩墖 VBI 鏁版嵁鏁扮粍銆俙line`\ [^0^] 鍒?`line`\ [^17^] 瀵瑰簲浜?	绗竴涓満鐨勭 6 鍒?23 琛屻€俙line`\ [^18^] 鍒?`line`\ [^35^] 瀵瑰簲浜庣浜屼釜鍦虹殑绗?6 鍒?23 琛屻€?


### struct v4l2_mpeg_vbi_itv0_line



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2

    - - __u8
      - `id`
      - 鏉ヨ嚜 ITV0-Line-Identifier-Constants 鐨勪竴涓鏍囪瘑绗﹀€硷紝鎸囩ず姝よ涓婂瓨鍌ㄧ殑鍒囩墖
	VBI 鏁版嵁鐨勭被鍨嬨€?    - - __u8
      - `data`\ [^42^]
      - 璇ヨ鐨勫垏鐗?VBI 鏁版嵁銆?


### struct v4l2_mpeg_vbi_itv0_line id 瀛楁鐨勮鏍囪瘑绗?

    :header-rows:  1
    :stub-columns: 0
    :widths:       3 1 4

    - - Defined Symbol
      - Value
      - Description
    - - `V4L2_MPEG_VBI_IVTV_TELETEXT_B`
      - 1
      - 鏈夊叧琛岃礋杞界殑鎻忚堪锛岃鍙傞槄鍒囩墖 VBI 鏈嶅姟 <vbi-services2>銆?    - - `V4L2_MPEG_VBI_IVTV_CAPTION_525`
      - 4
      - 鏈夊叧琛岃礋杞界殑鎻忚堪锛岃鍙傞槄鍒囩墖 VBI 鏈嶅姟 <vbi-services2>銆?    - - `V4L2_MPEG_VBI_IVTV_WSS_625`
      - 5
      - 鏈夊叧琛岃礋杞界殑鎻忚堪锛岃鍙傞槄鍒囩墖 VBI 鏈嶅姟 <vbi-services2>銆?    - - `V4L2_MPEG_VBI_IVTV_VPS`
      - 7
      - 鏈夊叧琛岃礋杞界殑鎻忚堪锛岃鍙傞槄鍒囩墖 VBI 鏈嶅姟 <vbi-services2>銆?

   鏍规嵁 ETS 300 706 <ets300706>锛岀涓€涓満鐨勭 6-22 琛屽拰绗簩涓満鐨勭 5-22 琛屽彲鑳芥惡甯﹀浘鏂囩數瑙嗘暟鎹€?
   鍙﹁鍙傞槄 vbi-525 鍜?vbi-625銆?