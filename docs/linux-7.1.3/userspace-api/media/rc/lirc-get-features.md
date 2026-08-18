

######## ioctl LIRC_GET_FEATURES


## 鍚嶇О

LIRC_GET_FEATURES - 鑾峰彇搴曞眰纭欢璁惧鐨勭壒鎬?


## Synopsis

`int ioctl(int fd, LIRC_GET_FEATURES, __u32 *features)`


## Arguments

`fd`
鐢?open() 杩斿洖鐨勬枃浠舵弿杩扮銆?

`features`
LIRC 鐗规€х殑浣嶆帺鐮併€?


## 鎻忚堪

鑾峰彇搴曞眰纭欢璁惧鐨勭壒鎬с€傞┍鍔ㄤ細瀹ｅ憡瀹冩墍鏀寔鐨勬煇浜涚壒鎬э紝璋冪敤鏂瑰啀鎹鍙戣捣鐩稿簲鐨?ioctl銆?


## LIRC 鐗规€?


`LIRC_CAN_REC_RAW`

鏈娇鐢ㄣ€備繚鐣欎粎涓洪伩鍏嶇牬鍧?uAPI銆?


`LIRC_CAN_REC_PULSE`

鏈娇鐢ㄣ€備繚鐣欎粎涓洪伩鍏嶇牬鍧?uAPI銆傚湪鍙戦€佹椂浣跨敤 `LIRC_MODE_PULSE` <lirc-妯″紡-pulse>銆?


`LIRC_CAN_REC_MODE2`

鍘熷 IR 椹卞姩鎺ユ敹鏃朵娇鐢ㄣ€傛剰鍛崇潃浣跨敤 `LIRC_MODE_MODE2` <lirc-妯″紡-MODE2>銆傚悓鏃朵篃鎰忓懗鐫€鏀寔 `LIRC_MODE_SCANCODE` <lirc-妯″紡-SCANCODE>锛屽彧瑕佸唴鏍哥増鏈冻澶熸柊銆傚彲浣跨敤 `lirc_set_rec_mode` 鍒囨崲妯″紡銆?


`LIRC_CAN_REC_LIRCCODE`

鏈娇鐢ㄣ€備繚鐣欎粎涓洪伩鍏嶇牬鍧?uAPI銆?


`LIRC_CAN_REC_SCANCODE`

scancode 椹卞姩鎺ユ敹鏃朵娇鐢ㄣ€傛剰鍛崇潃浣跨敤 `LIRC_MODE_SCANCODE` <lirc-妯″紡-SCANCODE>銆?


`LIRC_CAN_SET_SEND_CARRIER`

椹卞姩鏀寔浣跨敤 ioctl `LIRC_SET_SEND_CARRIER` <LIRC_SET_SEND_CARRIER> 鏀瑰彉璋冨埗棰戠巼銆?


`LIRC_CAN_SET_SEND_DUTY_CYCLE`

椹卞姩鏀寔浣跨敤 ioctl `LIRC_SET_SEND_DUTY_CYCLE` <LIRC_SET_SEND_DUTY_CYCLE> 鏀瑰彉鍗犵┖姣斻€?


`LIRC_CAN_SET_TRANSMITTER_MASK`

椹卞姩鏀寔浣跨敤 ioctl `LIRC_SET_TRANSMITTER_MASK` <LIRC_SET_TRANSMITTER_MASK> 鏀瑰彉婵€娲荤殑鍙戦€佸櫒銆?


`LIRC_CAN_SET_REC_CARRIER`

椹卞姩鏀寔浣跨敤 ioctl `LIRC_SET_REC_CARRIER` <LIRC_SET_REC_CARRIER> 璁剧疆鎺ユ敹杞芥尝棰戠巼銆?


`LIRC_CAN_SET_REC_CARRIER_RANGE`

椹卞姩鏀寔 ioctl `LIRC_SET_REC_CARRIER_RANGE` <LIRC_SET_REC_CARRIER_RANGE>銆?


`LIRC_CAN_GET_REC_RESOLUTION`

椹卞姩鏀寔 ioctl `LIRC_GET_REC_RESOLUTION` <LIRC_GET_REC_RESOLUTION>銆?


`LIRC_CAN_SET_REC_TIMEOUT`

椹卞姩鏀寔 ioctl `LIRC_SET_REC_TIMEOUT` <LIRC_SET_REC_TIMEOUT>銆?


`LIRC_CAN_MEASURE_CARRIER`

椹卞姩鏀寔浣跨敤 ioctl `LIRC_SET_MEASURE_CARRIER_MODE` <LIRC_SET_MEASURE_CARRIER_MODE> 娴嬮噺璋冨埗棰戠巼銆?


`LIRC_CAN_USE_WIDEBAND_RECEIVER`

椹卞姩鏀寔浣跨敤 ioctl `LIRC_SET_WIDEBAND_RECEIVER` <LIRC_SET_WIDEBAND_RECEIVER> 杩涘叆瀛︿範妯″紡銆?


`LIRC_CAN_SEND_RAW`

鏈娇鐢ㄣ€備繚鐣欎粎涓洪伩鍏嶇牬鍧?uAPI銆?


`LIRC_CAN_SEND_PULSE`

椹卞姩鏀寔浣跨敤 `LIRC_MODE_PULSE` <lirc-妯″紡-pulse> 鍙戦€侊紙浜︾О IR blasting / IR 鍙戝皠锛夈€傛剰鍛崇潃鏀寔浣跨敤 `LIRC_MODE_SCANCODE` <lirc-妯″紡-SCANCODE> 鍙戦€侊紝鍙鍐呮牳鐗堟湰瓒冲鏂般€傚彲浣跨敤 `lirc_set_send_mode` 鍒囨崲妯″紡銆?


`LIRC_CAN_SEND_MODE2`

鏈娇鐢ㄣ€備繚鐣欎粎涓洪伩鍏嶇牬鍧?uAPI銆傚湪鎺ユ敹鏃朵娇鐢?`LIRC_MODE_MODE2` <lirc-妯″紡-mode2>銆?


`LIRC_CAN_SEND_LIRCCODE`

鏈娇鐢ㄣ€備繚鐣欎粎涓洪伩鍏嶇牬鍧?uAPI銆?


## 杩斿洖鍊?

鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖 -1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆傞€氱敤鐨勯敊璇爜鍦ㄣ€奊eneric 閿欒 Codes銆?gen-閿欒> 绔犺妭涓弿杩般€?
