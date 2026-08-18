


######## ioctl VIDIOC_STREAMON, VIDIOC_STREAMOFF


## 鍚嶇О


VIDIOC_STREAMON - VIDIOC_STREAMOFF - 寮€濮嬫垨鍋滄娴?I/O

## 姒傝


`int ioctl(int fd, VIDIOC_STREAMON, const int *argp)`


`int ioctl(int fd, VIDIOC_STREAMOFF, const int *argp)`

## 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?

`argp`
    鎸囧悜涓€涓暣鏁扮殑鎸囬拡銆?

## 鎻忚堪


`VIDIOC_STREAMON` 涓?`VIDIOC_STREAMOFF` ioctl 鍦ㄦ祦锛坰treaming锛夛紙鍐呭瓨鏄犲皠 <mmap>銆佺敤鎴锋寚閽?<userp> 鎴?DMABUF <dmabuf>锛塈/O 鏈熼棿鍚姩涓庡仠姝㈡崟鑾锋垨杈撳嚭杩囩▼銆?

鍦ㄨ皟鐢?`VIDIOC_STREAMON` 涔嬪墠锛屾崟鑾风‖浠惰绂佺敤锛屼笉浼氬～鍏呬换浣曡緭鍏ョ紦鍐插尯锛堝鏋滀紶鍏ラ槦鍒椾腑鏈変换浣曠┖缂撳啿鍖猴級銆傚湪璋冪敤 `VIDIOC_STREAMON` 涔嬪墠锛岃緭鍑虹‖浠惰绂佺敤锛屼笉浼氫骇鐢熶换浣曡棰戜俊鍙枫€?

鍐呭瓨鍒板唴瀛樿澶囩洿鍒颁负鎹曡幏涓庤緭鍑轰袱绉嶆祦绫诲瀷閮借皟鐢ㄤ簡 `VIDIOC_STREAMON` 鍚庢墠浼氬惎鍔ㄣ€?

濡傛灉 `VIDIOC_STREAMON` 澶辫触锛屽垯浠讳綍宸叉帓闃熺殑缂撳啿鍖哄皢淇濇寔鎺掗槦鐘舵€併€?

`VIDIOC_STREAMOFF` ioctl 闄や簡涓鎴栧畬鎴愪换浣曡繘琛屼腑鐨?DMA 澶栵紝杩樹細瑙ｉ攣浠讳綍閿佸畾鍦ㄧ墿鐞嗗唴瀛樹腑鐨勭敤鎴锋寚閽堢紦鍐插尯锛屽苟灏嗘墍鏈夌紦鍐插尯浠庝紶鍏ヤ笌浼犲嚭闃熷垪涓Щ闄ゃ€傝繖鎰忓懗鐫€鎵€鏈夊凡鎹曡幏浣嗗皻鏈嚭闃熺殑甯ч兘灏嗕涪澶憋紝鍚屾牱鎵€鏈夊凡鍏ラ槦鐢ㄤ簬杈撳嚭浣嗗皻鏈紶杈撶殑甯т篃浼氫涪澶便€侷/O 杩斿洖鍒颁笌璋冪敤 VIDIOC_REQBUFS 涔嬪悗鐩稿悓鐨勭姸鎬侊紝骞跺彲鐩稿簲鍦伴噸鏂板惎鍔ㄣ€?

濡傛灉缂撳啿鍖哄凡閫氳繃 VIDIOC_QBUF 鎺掗槦锛屼笖鍦ㄤ粠鏈皟鐢ㄨ繃 `VIDIOC_STREAMON` 鐨勬儏鍐典笅璋冪敤浜?`VIDIOC_STREAMOFF`锛岄偅涔堣繖浜涘凡鎺掗槦鐨勭紦鍐插尯涔熷皢浠庝紶鍏ラ槦鍒椾腑绉婚櫎锛屽苟鍏ㄩ儴杩斿洖鍒颁笌璋冪敤 VIDIOC_REQBUFS 涔嬪悗鐩稿悓鐨勭姸鎬侊紝鍙浉搴斿湴閲嶆柊鍚姩銆?

涓や釜 ioctl 閮芥帴鍙椾竴涓寚鍚戞暣鏁扮殑鎸囬拡锛屽嵆鏈熸湜鐨勭紦鍐插尯鎴栨祦绫诲瀷銆傝繖涓庣粨鏋勪綋 `v4l2_requestbuffers` 鐨?`type` 鐩稿悓銆?

濡傛灉鍦ㄦ祦宸插湪杩涜涓椂璋冪敤 `VIDIOC_STREAMON`锛屾垨鍦ㄦ祦宸插仠姝㈡椂璋冪敤 `VIDIOC_STREAMOFF`锛屽垯杩斿洖 0銆傚湪 `VIDIOC_STREAMON` 鐨勬儏鍐典笅浠€涔堜篃涓嶄細鍙戠敓锛屼絾 `VIDIOC_STREAMOFF` 浼氬涓婃墍杩板皢宸叉帓闃熺殑缂撳啿鍖鸿繑鍥炲埌瀹冧滑鐨勮捣濮嬬姸鎬併€?


   搴旂敤绋嬪簭鍙兘鍦?`VIDIOC_STREAMON` 鎴?`VIDIOC_STREAMOFF` 璋冪敤涔嬪墠鎴栦箣鍚庣殑鏈煡鏃堕棿娈靛唴琚姠鍗狅紝娌℃湁鈥滅幇鍦ㄢ€濆紑濮嬫垨鍋滄鐨勬蹇点€傚彲浠ヤ娇鐢ㄧ紦鍐插尯鏃堕棿鎴虫潵涓庡叾浠栦簨浠跺悓姝ャ€?

## 杩斿洖鍊?


鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1 骞剁浉搴斿湴璁剧疆 `errno` 鍙橀噺銆傞€氱敤閿欒鐮佸湪閫氱敤閿欒鐮?<gen-errors> 绔犺妭涓弿杩般€?

EINVAL
    涓嶆敮鎸佺紦鍐插尯 `type`锛屾垨灏氭湭鍒嗛厤锛堝唴瀛樻槧灏勶級鎴栧叆闃燂紙杈撳嚭锛変换浣曠紦鍐插尯銆?

EPIPE
    椹卞姩瀹炵幇浜?pad 绾ф牸寮忛厤缃?<pad-level-formats>锛屼笖娴佹按绾块厤缃棤鏁堛€?

ENOLINK
    椹卞姩瀹炵幇浜?Media Controller 鎺ュ彛锛屼笖娴佹按绾块摼璺厤缃棤鏁堛€?
