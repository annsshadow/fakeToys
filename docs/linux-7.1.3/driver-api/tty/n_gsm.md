## GSM 0710 tty 澶氳矾澶嶇敤鍣ㄤ娇鐢ㄨ鏄?

璇ョ嚎璺绋嬪疄鐜颁簡 GSM 07.10 澶氳矾澶嶇敤鍗忚锛岃瑙佷互涓?3GPP 鏂囨。锛?
	https://www.3gpp.org/ftp/Specs/archive/07_series/07.10/0710-720.zip

鏈枃妗ｇ粰鍑轰簡涓€浜涘叧浜庡浣曞皢璇ラ┍鍔ㄧ敤浜庤繛鎺ュ埌鐗╃悊涓插彛鐨?GPRS 鍜?3G 璋冨埗瑙ｈ皟鍣ㄧ殑鎻愮ず銆?
## 濡備綍浣跨敤


### 閰嶇疆鍙戣捣鏂?

#. 閫氳繃鍏朵覆鍙ｅ皢璋冨埗瑙ｈ皟鍣ㄥ垵濮嬪寲涓?0710 澶氳矾澶嶇敤锛坢ux锛夋ā寮忥紙閫氬父浣跨敤 `AT+CMUX=` 鍛戒护锛夈€傛牴鎹墍鐢ㄨ皟鍒惰В璋冨櫒鐨勪笉鍚岋紝鍙互鍚戣鍛戒护浼犲叆鎴栧鎴栧皯鍙傛暟銆?
#. 浣跨敤 `TIOCSETD` ioctl 灏嗕覆琛岀嚎璺垏鎹负浣跨敤 n_gsm 绾胯矾瑙勭▼銆?
#. 濡傛湁闇€瑕侊紝浣跨敤 `GSMIOC_GETCONF_EXT`/`GSMIOC_SETCONF_EXT` ioctl 閰嶇疆澶氳矾澶嶇敤鍣ㄣ€?
#. 浣跨敤 `GSMIOC_GETCONF`/`GSMIOC_SETCONF` ioctl 閰嶇疆澶氳矾澶嶇敤鍣ㄣ€?
#. 瀵逛簬闈為粯璁ら厤缃紝浣跨敤 `GSMIOC_GETCONF_DLCI`/`GSMIOC_SETCONF_DLCI` ioctl 閰嶇疆 DLC銆?
#. 鑾峰彇鎵€鐢ㄤ覆鍙ｇ殑鍩哄噯 gsmtty 缂栧彿銆?
   鍒濆鍖栫▼搴忕殑涓昏閮ㄥ垎
```

      #include <stdio.h>
      #include <stdint.h>
      #include <linux/gsmmux.h>
      #include <linux/tty.h>

      #define DEFAULT_SPEED	B115200
      #define SERIAL_PORT	/dev/ttyS0

      int ldisc = N_GSM0710;
      struct gsm_config c;
      struct gsm_config_ext ce;
      struct gsm_dlci_config dc;
      struct termios configuration;
      uint32_t first;

      /* open the serial port connected to the modem */
      fd = open(SERIAL_PORT, O_RDWR | O_NOCTTY | O_NDELAY);

      /* configure the serial port : speed, flow control ... */

      /* send the AT commands to switch the modem to CMUX mode
         and check that it's successful (should return OK) */
      write(fd, "AT+CMUX=0\r", 10);

      /* experience showed that some modems need some time before
         being able to answer to the first MUX packet so a delay
         may be needed here in some case */
      sleep(3);

      /* use n_gsm line discipline */
      ioctl(fd, TIOCSETD, &ldisc);

      /* get n_gsm extended configuration */
      ioctl(fd, GSMIOC_GETCONF_EXT, &ce);
      /* use keep-alive once every 5s for modem connection supervision */
      ce.keep_alive = 500;
      /* set the new extended configuration */
      ioctl(fd, GSMIOC_SETCONF_EXT, &ce);
      /* get n_gsm configuration */
      ioctl(fd, GSMIOC_GETCONF, &c);
      /* we are initiator and need encoding 0 (basic) */
      c.initiator = 1;
      c.encapsulation = 0;
      /* our modem defaults to a maximum size of 127 bytes */
      c.mru = 127;
      c.mtu = 127;
      /* set the new configuration */
      ioctl(fd, GSMIOC_SETCONF, &c);
      /* get DLC 1 configuration */
      dc.channel = 1;
      ioctl(fd, GSMIOC_GETCONF_DLCI, &dc);
      /* the first user channel gets a higher priority */
      dc.priority = 1;
      /* set the new DLC 1 specific configuration */
      ioctl(fd, GSMIOC_SETCONF_DLCI, &dc);
      /* get first gsmtty device node */
      ioctl(fd, GSMIOC_GETFIRST, &first);
      printf("first muxed line: /dev/gsmtty%i\n", first);

      /* and wait for ever to keep the line discipline enabled */
      daemon(0,0);
      pause();

```
#. 灏嗚繖浜涜澶囧綋浣滄櫘閫氫覆鍙ｄ娇鐢ㄣ€?
   渚嬪锛屽彲浠ワ細

   - 浣跨敤 **gnokii** 鍦?`ttygsm1` 涓婂彂閫?/ 鎺ユ敹鐭俊
   - 浣跨敤 **ppp** 鍦?`ttygsm2` 涓婂缓绔嬫暟鎹摼璺?
#. 鍦ㄥ叧闂墿鐞嗙鍙ｄ箣鍓嶏紝鍏堝叧闂墍鏈夎櫄鎷熺鍙ｃ€?
   娉ㄦ剰锛屽叧闂墿鐞嗙鍙ｅ悗璋冨埗瑙ｈ皟鍣ㄤ粛澶勪簬澶氳矾澶嶇敤妯″紡銆傝繖鍙兘浼氬鑷寸◢鍚庢棤娉曟垚鍔熼噸鏂版墦寮€璇ョ鍙ｃ€備负閬垮厤杩欑鎯呭喌锛屽彲浠ュ湪鍒濆鍖栧璺鐢ㄦā寮忎箣鍓嶏紝鍦ㄧ‖浠跺厑璁哥殑鎯呭喌涓嬪浣嶈皟鍒惰В璋冨櫒锛屾垨鑰呮墜鍔ㄥ彂閫佹柇寮€杩炴帴鍛戒护甯?```

      0xf9, 0x03, 0xef, 0x03, 0xc3, 0x16, 0xf9

```
### 閰嶇疆璇锋眰鏂?

#. 閫氳繃鍏朵覆鍙ｆ帴鏀?`AT+CMUX=` 鍛戒护锛屽垵濮嬪寲澶氳矾澶嶇敤妯″紡閰嶇疆銆?
#. 浣跨敤 `TIOCSETD` ioctl 灏嗕覆琛岀嚎璺垏鎹负浣跨敤 **n_gsm** 绾胯矾瑙勭▼銆?
#. 濡傛湁闇€瑕侊紝浣跨敤 `GSMIOC_GETCONF_EXT`/`GSMIOC_SETCONF_EXT`
   ioctl 閰嶇疆澶氳矾澶嶇敤鍣ㄣ€?
#. 浣跨敤 `GSMIOC_GETCONF`/`GSMIOC_SETCONF` ioctl 閰嶇疆澶氳矾澶嶇敤鍣ㄣ€?
#. 瀵逛簬闈為粯璁ら厤缃紝浣跨敤 `GSMIOC_GETCONF_DLCI`/`GSMIOC_SETCONF_DLCI` ioctl 閰嶇疆 DLC銆?
```

        #include <stdio.h>
        #include <stdint.h>
        #include <linux/gsmmux.h>
        #include <linux/tty.h>
        #define DEFAULT_SPEED	B115200
        #define SERIAL_PORT	/dev/ttyS0

	int ldisc = N_GSM0710;
	struct gsm_config c;
	struct gsm_config_ext ce;
	struct gsm_dlci_config dc;
	struct termios configuration;
	uint32_t first;

	/* open the serial port */
	fd = open(SERIAL_PORT, O_RDWR | O_NOCTTY | O_NDELAY);

	/* configure the serial port : speed, flow control ... */

	/* get serial data and check "AT+CMUX=command" parameter ... */

	/* use n_gsm line discipline */
	ioctl(fd, TIOCSETD, &ldisc);

	/* get n_gsm extended configuration */
	ioctl(fd, GSMIOC_GETCONF_EXT, &ce);
	/* use keep-alive once every 5s for peer connection supervision */
	ce.keep_alive = 500;
	/* set the new extended configuration */
	ioctl(fd, GSMIOC_SETCONF_EXT, &ce);
	/* get n_gsm configuration */
	ioctl(fd, GSMIOC_GETCONF, &c);
	/* we are requester and need encoding 0 (basic) */
	c.initiator = 0;
	c.encapsulation = 0;
	/* our modem defaults to a maximum size of 127 bytes */
	c.mru = 127;
	c.mtu = 127;
	/* set the new configuration */
	ioctl(fd, GSMIOC_SETCONF, &c);
	/* get DLC 1 configuration */
	dc.channel = 1;
	ioctl(fd, GSMIOC_GETCONF_DLCI, &dc);
	/* the first user channel gets a higher priority */
	dc.priority = 1;
	/* set the new DLC 1 specific configuration */
	ioctl(fd, GSMIOC_SETCONF_DLCI, &dc);
	/* get first gsmtty device node */
	ioctl(fd, GSMIOC_GETFIRST, &first);
	printf("first muxed line: /dev/gsmtty%i\n", first);

	/* and wait for ever to keep the line discipline enabled */
	daemon(0,0);
	pause();

```
11-03-08 - Eric B茅nard - <eric@eukrea.com>
