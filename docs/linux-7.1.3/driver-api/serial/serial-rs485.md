## RS485 涓茶閫氫俊


## 1. 绠€浠?


   EIA-485锛屽張绉?TIA/EIA-485 鎴?RS-485锛屾槸涓€涓畾涔夌敤浜庡钩琛″紡鏁板瓧澶氱偣绯荤粺鐨?
   椹卞姩鍣ㄥ拰鎺ユ敹鍣ㄧ數姘旂壒鎬х殑鏍囧噯銆?
   璇ユ爣鍑嗗箍娉涚敤浜庡伐涓氳嚜鍔ㄥ寲棰嗗煙鐨勯€氫俊锛屽洜涓哄畠鍙互鏈夋晥鍦扮敤浜庨暱璺濈浼犺緭锛?
   骞朵笖鍦ㄧ數姘斿櫔澹扮幆澧冧腑涔熻兘宸ヤ綔銆?

## 2. 纭欢鐩稿叧鑰冭檻


   鏌愪簺 CPU/UART锛堜緥濡?Atmel AT91 鎴?16C950 UART锛夊唴缃簡鍗婂弻宸ユā寮忥紝鑳藉閫氳繃
   鍒囨崲 RTS 鎴?DTR 淇″彿鑷姩鎺у埗绾胯矾鏂瑰悜銆傝繖鍙敤浜庢帶鍒跺閮ㄧ殑鍗婂弻宸ョ‖浠讹紝濡?RS485
   鏀跺彂鍣紝鎴栦换浣曡繛鎺ュ埌 RS232 鐨勫崐鍙屽伐璁惧锛屽鏌愪簺璋冨埗瑙ｈ皟鍣ㄣ€?

   瀵逛簬杩欎簺寰帶鍒跺櫒锛孡inux 椹卞姩搴斿綋鑳藉鍚屾椂宸ヤ綔浜庝袱绉嶆ā寮忥紝骞朵笖搴斿綋鍦ㄧ敤鎴峰眰鎻愪緵
   閫傚綋鐨?ioctl锛堣鍚庢枃锛夛紝浠ュ厑璁镐粠涓€绉嶆ā寮忓垏鎹㈠埌鍙︿竴绉嶆ā寮忥紝鍙嶄箣浜︾劧銆?

## 3. 鍐呮牳涓凡鏈夌殑鏁版嵁缁撴瀯


   Linux 鍐呮牳鎻愪緵浜?struct serial_rs485 鏉ュ鐞?RS485 閫氫俊銆傝鏁版嵁缁撴瀯鐢ㄤ簬鍦?
   骞冲彴鏁版嵁鍜?ioctl 涓缃拰閰嶇疆 RS485 鍙傛暟銆?

   璁惧鏍戜篃鍙互鎻愪緵 RS485 鍚姩鍙傛暟锛圼#DT-bindings]_锛夈€傚綋椹卞姩璋冪敤
   uart_get_rs485_mode() 鏃讹紝涓茶鏍稿績浼氭牴鎹澶囨爲缁欏嚭鐨勫€煎～鍏?struct serial_rs485銆?

   浠讳綍鑳藉鍚屾椂宸ヤ綔浜?RS232 鍜?RS485 鐨勮澶囩殑椹卞姩閮藉簲瀹炵幇 `struct uart_port` 鍥炶皟锛屽苟鍦?
   `struct uart_port` 涓彁渚?`rs485_supported`銆備覆琛屾牳蹇冭皟鐢?`rs485_supported` 鏉ュ搷搴?
   TIOCSRS485 ioctl锛堣涓嬫枃锛夊畬鎴愯澶囩浉鍏崇殑閮ㄥ垎銆俙struct uart_port` 鍥炶皟鎺ユ敹涓€涓寚鍚戠粡杩?
   鍑€鍖栫殑 struct serial_rs485 鐨勬寚閽堛€傜敤鎴风┖闂存彁渚涚殑 struct serial_rs485 鍦ㄨ皟鐢?
   `struct uart_port` 涔嬪墠浼氬厛缁?`rs485_supported` 鍑€鍖栵紝璇ュ洖璋冩寚绀洪┍鍔ㄩ拡瀵?`struct uart_port`
   鏀寔鍝簺 RS485 鐗规€с€俆IOCGRS485 ioctl 鍙敤浜庤鍥炰笌褰撳墠閰嶇疆鍖归厤鐨?struct serial_rs485銆?

   :identifiers: serial_rs485 uart_get_rs485_mode

## 4. 鐢ㄦ埛灞傜殑浣跨敤


   鍦ㄧ敤鎴峰眰锛屽彲浠ヤ娇鐢ㄥ墠杩扮殑鎺ュ彛鑾峰彇/璁剧疆 RS485 閰嶇疆
```

	#include <linux/serial.h>

	/* Include definition for RS485 ioctls: TIOCGRS485 and TIOCSRS485 */
	#include <sys/ioctl.h>

	/* Open your specific device (e.g., /dev/mydevice): */
	int fd = open ("/dev/mydevice", O_RDWR);
	if (fd < 0) {
		/* Error handling. See errno. */
	}

	struct serial_rs485 rs485conf;

	/* Enable RS485 mode: */
	rs485conf.flags |= SER_RS485_ENABLED;

	/* Set logical level for RTS pin equal to 1 when sending: */
	rs485conf.flags |= SER_RS485_RTS_ON_SEND;
	/* or, set logical level for RTS pin equal to 0 when sending: */
	rs485conf.flags &= ~(SER_RS485_RTS_ON_SEND);

	/* Set logical level for RTS pin equal to 1 after sending: */
	rs485conf.flags |= SER_RS485_RTS_AFTER_SEND;
	/* or, set logical level for RTS pin equal to 0 after sending: */
	rs485conf.flags &= ~(SER_RS485_RTS_AFTER_SEND);

	/* Set rts delay before send, if needed: */
	rs485conf.delay_rts_before_send = ...;

	/* Set rts delay after send, if needed: */
	rs485conf.delay_rts_after_send = ...;

	/* Set this flag if you want to receive data even while sending data */
	rs485conf.flags |= SER_RS485_RX_DURING_TX;

	if (ioctl (fd, TIOCSRS485, &rs485conf) < 0) {
		/* Error handling. See errno. */
	}

	/* Use read() and write() syscalls here... */

	/* Close the device when finished: */
	if (close (fd) < 0) {
		/* Error handling. See errno. */
	}

```
## 5. 澶氱偣瀵诲潃


   Linux 鍐呮牳涓哄鐐?RS-485 涓茶閫氫俊绾胯矾鎻愪緵浜嗗鍧€妯″紡銆傝瀵诲潃妯″紡閫氳繃鍦?
   struct serial_rs485 涓缃?`SER_RS485_ADDRB` 鏍囧織鏉ュ惎鐢ㄣ€俿truct serial_rs485 鍙︽湁涓や釜
   闄勫姞鏍囧織鍜屽瓧娈碉紝鐢ㄤ簬鍚敤鎺ユ敹鍦板潃鍜岀洰鐨勫湴鍧€銆?

   鍦板潃妯″紡鏍囧織锛?
 - `SER_RS485_ADDR_DEST`锛氬惎鐢ㄥ鍧€妯″紡锛堝悓鏃惰缃?termios 涓殑 ADDRB锛夈€?
 - `SER_RS485_ADDR_DEST`锛氬惎鐢ㄦ帴鏀讹紙杩囨护锛夊湴鍧€銆?
 - `SER_RS485_ADDR_DEST`锛氳缃洰鐨勫湴鍧€銆?

   鍦板潃瀛楁锛堢敱鐩稿簲鐨?`addr_dest` 鏍囧織鍚敤锛夛細
 - `addr_dest`锛氭帴鏀跺湴鍧€銆?
 - `addr_dest`锛氱洰鐨勫湴鍧€銆?

   涓€鏃﹁缃簡鎺ユ敹鍦板潃锛岄€氫俊灏卞彧鑳戒笌鐗瑰畾璁惧杩涜锛屽叾浠栧绛夋柟浼氳杩囨护鎺夈€傛槸鍚﹀己鍒?
   鎵ц杩囨护鐢辨帴鏀舵柟鍐冲畾銆傝嫢鏈缃?`SER_RS485_ADDR_RECV`锛屾帴鏀跺湴鍧€灏嗚娓呴櫎銆?

   娉ㄦ剰锛氬苟闈炴墍鏈夋敮鎸?RS485 鐨勮澶囬兘鏀寔澶氱偣瀵诲潃銆?

## 6. 鍙傝€冭祫鏂?


