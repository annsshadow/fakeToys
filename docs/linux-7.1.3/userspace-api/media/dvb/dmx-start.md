


## DMX_START


### 鍚嶇О


DMX_START

### 姒傝



`int ioctl(int fd, DMX_START)`

### 鍙傛暟


`fd`
    鐢?`open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
### 鎻忚堪


姝?ioctl 璋冪敤鐢ㄤ簬鍚姩閫氳繃 ioctl 璋冪敤 DMX_SET_FILTER 鎴?DMX_SET_PES_FILTER 瀹氫箟鐨勫疄闄呰繃婊ゆ搷浣溿€?
### 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?

    :header-rows:  0
    :stub-columns: 0

    - .. row 1

       - `EINVAL`

       - 鏃犳晥鍙傛暟锛屽嵆鏈€氳繃 DMX_SET_FILTER 鎴?DMX_SET_PES_FILTER ioctl
	  鎻愪緵浠讳綍杩囨护鍙傛暟銆?
    - .. row 2

       - `EBUSY`

       - 姝ら敊璇爜琛ㄧず瀛樺湪鍐茬獊璇锋眰銆傛湁娲诲姩鐨勮繃婊ゅ櫒姝ｅ湪浠庡彟涓€涓緭鍏ユ簮
	 杩囨护鏁版嵁銆傚湪鍚姩姝よ繃婊ゅ櫒涔嬪墠锛岃纭繚杩欎簺杩囨护鍣ㄥ凡鍋滄銆?
閫氱敤閿欒鐮佺殑鎻忚堪瑙侀€氱敤閿欒鐮?<gen-errors> 绔犺妭銆?