######## ioctl VIDIOC_QUERY_DV_TIMINGS


## 鍚嶇О


VIDIOC_QUERY_DV_TIMINGS - VIDIOC_SUBDEV_QUERY_DV_TIMINGS - 妫€娴嬪綋鍓嶈緭鍏ユ敹鍒扮殑 DV 棰勮

## 鎽樿


`int ioctl(int fd, VIDIOC_QUERY_DV_TIMINGS, struct v4l2_dv_timings *argp)`


`int ioctl(int fd, VIDIOC_SUBDEV_QUERY_DV_TIMINGS, struct v4l2_dv_timings *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`argp`
    鎸囧悜 struct `v4l2_dv_timings` 鐨勬寚閽堛€?
## 鎻忚堪


纭欢鍙兘鑳藉鑷姩妫€娴嬪綋鍓嶇殑 DV 鏃跺簭锛岀被浼间簬妫€娴嬭棰戞爣鍑嗐€備负姝わ紝搴旂敤绋嬪簭浠ユ寚鍚?涓€涓?struct `v4l2_dv_timings` 鐨勬寚閽堣皟鐢?VIDIOC_QUERY_DV_TIMINGS銆備竴鏃︾‖浠舵娴嬪埌
鏃跺簭锛屽畠灏嗗～鍏呰鏃跺簭缁撴瀯銆?

   椹卞姩**涓嶅緱**鍦ㄦ娴嬪埌鏂版椂搴忔椂鑷姩鍒囨崲鏃跺簭銆傜浉鍙嶏紝椹卞姩搴斿彂閫?   `V4L2_EVENT_SOURCE_CHANGE` 浜嬩欢锛堝鏋滃畠浠敮鎸侊級锛屽苟鏈熸湜鐢ㄦ埛绌洪棿閫氳繃璋冪敤
   VIDIOC_QUERY_DV_TIMINGS 鏉ラ噰鍙栬鍔ㄣ€傚師鍥犳槸鏂版椂搴忛€氬父涔熸剰鍛崇潃涓嶅悓鐨勭紦鍐插尯澶у皬锛?   鑰屼綘鏃犳硶鍦ㄨ繍琛屾椂鏇存敼缂撳啿鍖哄ぇ灏忋€備竴鑸€岃█锛屾帴鏀跺埌 Source Change 浜嬩欢鐨勫簲鐢ㄧ▼搴?   蹇呴』璋冪敤 VIDIOC_QUERY_DV_TIMINGS锛屽鏋滄娴嬪埌鐨勬椂搴忔湁鏁堬紝鍒欏繀椤诲仠姝㈡祦浼犺緭銆佽缃?   鏂版椂搴忋€佸垎閰嶆柊缂撳啿鍖哄苟閲嶆柊鍚姩娴佷紶杈撱€?
濡傛灉鍥犱负鏃犱俊鍙疯€屾棤娉曟娴嬫椂搴忥紝鍒欒繑鍥?ENOLINK銆傚鏋滄娴嬪埌淇″彿锛屼絾瀹冧笉绋冲畾涓旀帴鏀跺櫒
鏃犳硶閿佸畾鍒拌淇″彿锛屽垯杩斿洖 `ENOLCK`銆傚鏋滄帴鏀跺櫒鑳藉閿佸畾鍒颁俊鍙凤紝浣嗘牸寮忎笉鍙楁敮鎸侊紙渚嬪
鍥犱负鍍忕礌鏃堕挓瓒呭嚭纭欢鑳藉姏鑼冨洿锛夛紝鍒欓┍鍔ㄥ～鍏呭畠鎵€鑳芥壘鍒扮殑浠绘剰鏃跺簭骞惰繑鍥?`ERANGE`銆傚湪
璇ユ儏鍐典笅锛屽簲鐢ㄧ▼搴忓彲浠ヨ皟鐢?VIDIOC_DV_TIMINGS_CAP锛屽皢鎵惧埌鐨勬椂搴忎笌纭欢鑳藉姏杩涜姣旇緝锛?浠ヤ究鍚戠敤鎴锋彁渚涙洿鍏峰弽棣堛€?
## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪
Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?
ENODATA
    璇ヨ緭鍏ユ垨杈撳嚭涓嶆敮鎸佹暟瀛楄棰戞椂搴忋€?
ENOLINK
    鍥犱负鏈壘鍒颁俊鍙凤紝鏃犳硶妫€娴嬪埌浠讳綍鏃跺簭銆?
ENOLCK
    淇″彿涓嶇ǔ瀹氾紝纭欢鏃犳硶閿佸畾鍒板畠銆?
ERANGE
    鎵惧埌浜嗘椂搴忥紝浣嗗畠浠秴鍑轰簡纭欢鑳藉姏鑼冨洿銆?