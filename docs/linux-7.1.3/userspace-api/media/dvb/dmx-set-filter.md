## DMX_SET_FILTER


### Name


DMX_SET_FILTER

### Synopsis


`int ioctl(int fd, DMX_SET_FILTER, struct dmx_sct_filter_params *params)`

### Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`params`
    鎸囧悜鍖呭惈杩囨护鍙傛暟鐨勭粨鏋勪綋鐨勬寚閽堛€?
### Description


璇?ioctl 璋冪敤鏍规嵁鎵€鎻愪緵鐨勮繃婊ゅ櫒鍜屾帺鐮佸弬鏁拌缃竴涓繃婊ゅ櫒銆傚彲浠ュ畾涔変竴涓秴鏃舵椂闂达紝琛ㄧず绛夊緟鏌愪釜娈碉紙section锛夎鍔犺浇鐨勭鏁般€傚€间负 0 琛ㄧず涓嶅簲鐢ㄨ秴鏃躲€傛渶鍚庤繕鏈変竴涓爣蹇楀瓧娈碉紝鍙敤浜庢寚鏄庢煇涓鏄惁搴旇繘琛?CRC 鏍￠獙銆佽杩囨护鍣ㄦ槸鍚﹀簲涓衡€滀竴娆℃€э紙one-shot锛夆€濊繃婊ゅ櫒锛堝嵆鏄惁鍦ㄦ帴鏀跺埌绗竴涓鍚庡仠姝㈣繃婊ゆ搷浣滐級锛屼互鍙婅繃婊ゆ搷浣滄槸鍚﹀簲绔嬪嵆寮€濮嬶紙鏃犻渶绛夊緟 DMX_START ioctl 璋冪敤锛夈€傚鏋滀箣鍓嶅凡缁忚缃簡涓€涓繃婊ゅ櫒锛屽垯璇ヨ繃婊ゅ櫒灏嗚鍙栨秷锛屾帴鏀剁紦鍐插尯涔熶細琚竻绌恒€?
### Return Value


鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?