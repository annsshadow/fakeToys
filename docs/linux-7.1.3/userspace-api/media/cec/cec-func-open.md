
######## cec open()


## 鍚嶇О


cec-open - 鎵撳紑涓€涓?cec 璁惧

## 姒傝


    #include <fcntl.h>


## 鍙傛暟


`device_name`
    瑕佹墦寮€鐨勮澶囥€?

`flags`
    鎵撳紑鏍囧織銆傝闂ā寮忓繀椤讳负 `O_RDWR`銆?

    褰撶粰瀹?`O_NONBLOCK` 鏍囧織鏃讹紝鍦ㄦ病鏈夋秷鎭垨浜嬩欢鍙敤鐨勬儏鍐典笅锛孋EC_RECEIVE <CEC_RECEIVE> 鍜?CEC_DQEVENT <CEC_DQEVENT> ioctl 灏嗚繑鍥?`EAGAIN` 閿欒鐮侊紝鑰?ioctl CEC_TRANSMIT <CEC_TRANSMIT>銆丆EC_ADAP_S_PHYS_ADDR <CEC_ADAP_S_PHYS_ADDR> 鍜?CEC_ADAP_S_LOG_ADDRS <CEC_ADAP_S_LOG_ADDRS> 閮借繑鍥?0銆?

    鍏跺畠鏍囧織娌℃湁鏁堟灉銆?

## 鎻忚堪


瑕佹墦寮€涓€涓?cec 璁惧锛屽簲鐢ㄧ▼搴忚皟鐢?`open()` 骞朵紶鍏ユ湡鏈涚殑璁惧鍚嶃€傝鍑芥暟娌℃湁鍓綔鐢紱璁惧閰嶇疆淇濇寔涓嶅彉銆?

褰撲互鍙妯″紡鎵撳紑璁惧鏃讹紝灏濊瘯淇敼鍏堕厤缃皢瀵艰嚧閿欒锛屽苟涓?`errno` 灏嗚璁句负 EBADF銆?

## 杩斿洖鍊?


`open()` 鎴愬姛鏃惰繑鍥炴柊鐨勬枃浠舵弿杩扮銆傚嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno`銆傚彲鑳界殑閿欒鐮佸寘鎷細

`EACCES`
    涓嶅厑璁稿鏂囦欢杩涜璇锋眰鐨勮闂€?

`EMFILE`
    璇ヨ繘绋嬪凡缁忔墦寮€浜嗘渶澶ф暟閲忕殑鏂囦欢銆?

`ENFILE`
    绯荤粺瀵规墦寮€鏂囦欢鎬绘暟鐨勯檺鍒跺凡缁忚揪鍒般€?

`ENOMEM`
    鍙敤鐨勫唴鏍稿唴瀛樹笉瓒炽€?

`ENODEV`
    鏈壘鍒拌澶囨垨宸茶绉婚櫎銆?
