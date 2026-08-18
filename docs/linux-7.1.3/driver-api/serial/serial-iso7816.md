## ISO7816 涓茶閫氫俊


## 1. 绠€浠?

  ISO/IEC7816 鏄竴绯诲垪瑙勫畾闆嗘垚鐢佃矾鍗★紙ICC锛屼篃绉颁负鏅鸿兘鍗★級鐨勬爣鍑嗐€?
## 2. 涓庣‖浠剁浉鍏崇殑鑰冭檻


  鏌愪簺 CPU/UART锛堜緥濡?Microchip AT91锛夊寘鍚竴涓唴缃ā寮忥紝鑳藉澶勭悊涓庢櫤鑳藉崱鐨勯€氫俊銆?
  瀵逛簬杩欎簺寰帶鍒跺櫒锛孡inux 椹卞姩搴斿綋琚仛鎴愯兘澶熷湪涓ょ妯″紡涓嬪伐浣滐紝骞朵笖搴斿綋鍦ㄧ敤鎴峰眰
  鎻愪緵閫傚綋鐨?ioctl锛堣鍚庢枃锛夛紝浠ュ厑璁镐粠涓€绉嶆ā寮忓垏鎹㈠埌鍙︿竴绉嶆ā寮忥紝鍙嶄箣浜︾劧銆?
## 3. 鍐呮牳涓凡鏈夌殑鏁版嵁缁撴瀯


  Linux 鍐呮牳鎻愪緵浜?serial_iso7816 缁撴瀯浣擄紙瑙?[^1^]锛夋潵澶勭悊 ISO7816 閫氫俊銆傝鏁版嵁
  缁撴瀯鐢ㄤ簬鍦?ioctl 涓缃拰閰嶇疆 ISO7816 鍙傛暟銆?
  浠讳綍鑳藉鍚屾椂浣滀负 RS232 涓?ISO7816 宸ヤ綔鐨勮澶囩殑椹卞姩锛岄兘搴斿綋鍦?uart_port 缁撴瀯浣撲腑
  瀹炵幇 iso7816_config 鍥炶皟銆俿erial_core 璋冪敤 iso7816_config 鏉ュ畬鎴愯澶囩浉鍏崇殑閮ㄥ垎锛?  浠ュ搷搴?TIOCGISO7816 涓?TIOCSISO7816 ioctl锛堣涓嬫枃锛夈€俰so7816_config 鍥炶皟鎺ユ敹涓€涓?  鎸囧悜 struct serial_iso7816 鐨勬寚閽堛€?
## 4. 鍦ㄧ敤鎴峰眰鐨勪娇鐢?

  鍦ㄧ敤鎴峰眰锛屽彲浠ヤ娇鐢ㄥ墠闈㈢殑鏂瑰紡鑾峰彇/璁剧疆 ISO7816 閰嶇疆

```

	#include <linux/serial.h>

	/* 鍖呭惈 ISO7816 ioctl 鐨勫畾涔夛細TIOCSISO7816 涓?TIOCGISO7816 */
	#include <sys/ioctl.h>

	/* 鎵撳紑浣犵殑鐗瑰畾璁惧锛堜緥濡?/dev/mydevice锛夛細 */
	int fd = open ("/dev/mydevice", O_RDWR);
	if (fd < 0) {
		/* 閿欒澶勭悊銆傚弬瑙?errno銆?*/
	}

	struct serial_iso7816 iso7816conf;

	/* 淇濈暀瀛楁蹇呴』娓呴浂 */
	memset(&iso7816conf, 0, sizeof(iso7816conf));

	/* 鍚敤 ISO7816 妯″紡锛?*/
	iso7816conf.flags |= SER_ISO7816_ENABLED;

	/* 閫夋嫨鍗忚锛?*/
	/* T=0 */
	iso7816conf.flags |= SER_ISO7816_T(0);
	/* 鎴?T=1 */
	iso7816conf.flags |= SER_ISO7816_T(1);

	/* 璁剧疆淇濇姢鏃堕棿锛坓uard time锛夛細 */
	iso7816conf.tg = 2;

	/* 璁剧疆鏃堕挓棰戠巼 */
	iso7816conf.clk = 3571200;

	/* 璁剧疆浼犺緭鍥犲瓙锛?*/
	iso7816conf.sc_fi = 372;
	iso7816conf.sc_di = 1;

	if (ioctl(fd_usart, TIOCSISO7816, &iso7816conf) < 0) {
		/* 閿欒澶勭悊銆傚弬瑙?errno銆?*/
	}

	/* 鍦ㄦ浣跨敤 read() 涓?write() 绯荤粺璋冪敤... */

	/* 瀹屾垚鍚庡叧闂澶囷細 */
	if (close (fd) < 0) {
		/* 閿欒澶勭悊銆傚弬瑙?errno銆?*/
	}

```
## 5. 鍙傝€冭祫鏂?

 [^1^]    include/uapi/linux/serial.h
