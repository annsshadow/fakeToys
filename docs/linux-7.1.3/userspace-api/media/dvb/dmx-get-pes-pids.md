## DMX_GET_PES_PIDS


### Name


DMX_GET_PES_PIDS

### Synopsis


`int ioctl(fd, DMX_GET_PES_PIDS, __u16 pids[^5^])`

### Arguments


`fd`
    `open()` 杩斿洖鐨勬枃浠舵弿杩扮銆?
`pids`
    鐢ㄤ簬瀛樺偍 5 涓妭鐩?ID锛圥rogram ID锛夌殑鏁扮粍銆?
### Description


璇?ioctl 鐢ㄤ簬鏌ヨ DVB 璁惧锛屼互杩斿洖缁欏畾鏈嶅姟涓煶棰戙€佽棰戙€佸浘鏂囩數瑙嗭紙teletext锛夈€佸瓧骞曞拰 PCR 鑺傜洰鎵€浣跨敤鐨勭涓€涓?PID銆傚畠浠寜濡備笅鏂瑰紡瀛樺偍锛?
=======================	========	=======================================
PID  element		position	content
=======================	========	=======================================
pids[DMX_PES_AUDIO]	0		first audio PID
pids[DMX_PES_VIDEO]	1		first video PID
pids[DMX_PES_TELETEXT]	2		first teletext PID
pids[DMX_PES_SUBTITLE]	3		first subtitle PID
pids[DMX_PES_PCR]	4		first Program Clock Reference PID
=======================	========	=======================================


	绛変簬 0xffff 鐨勫€艰〃绀鸿 PID 鏈鍐呮牳锛圞ernel锛夊～鍏呫€?
### Return Value


鎴愬姛鏃惰繑鍥?0銆?
鍑洪敊鏃惰繑鍥?-1锛屽苟鐩稿簲鍦拌缃?`errno` 鍙橀噺銆?
閫氱敤閿欒鐮佸湪 Generic Error Codes <gen-errors> 绔犺妭涓弿杩般€?