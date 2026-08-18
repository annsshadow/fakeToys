
######## ioctl FE_READ_STATUS


## Name


FE_READ_STATUS - 杩斿洖鍓嶇锛坒ront-end锛夌殑鐘舵€佷俊鎭€傝璋冪敤浠呴渶瑕佸璁惧鍏锋湁鍙璁块棶鏉冮檺銆?
## Synopsis


`int ioctl(int fd, FE_READ_STATUS, unsigned int *status)`

## Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`status`
    鎸囧悜涓€涓綅鎺╃爜鏁存暟鐨勬寚閽堬紝鐢?enum `fe_status` 涓畾涔夌殑鍊煎～鍏呫€?
## Description


鎵€鏈夋暟瀛楃數瑙嗭紙Digital TV锛夊墠绔澶囬兘鏀寔 `FE_READ_STATUS` ioctl銆傚畠鐢ㄤ簬鍦ㄨ皟璋愶紙tune锛変箣鍚庢鏌ュ墠绔殑閿佸畾锛坙ocking锛夌姸鎬併€傝 ioctl 鎺ユ敹涓€涓寚鍚戞暣鏁扮殑鎸囬拡锛岀姸鎬佷俊鎭皢琚啓鍏ュ叾涓€?

   status 鐨勫疄闄呭ぇ灏忎负 sizeof(enum fe_status)锛屽叾鍊奸殢浣撶郴缁撴瀯鑰屼笉鍚屻€傝繖涓€鐐归渶瑕佸湪灏嗘潵淇銆?
## int fe_status


fe_status 鍙傛暟鐢ㄤ簬鎸囩ず鍓嶇纭欢鐨勫綋鍓嶇姸鎬佸拰/鎴栫姸鎬佸彉鍖栥€傚畠鏄娇鐢?enum `fe_status` 鐨勫€兼寜浣嶆帺鐮侊紙bitmask锛夌粍鍚堣€屾垚鐨勩€?
## Return Value


鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?