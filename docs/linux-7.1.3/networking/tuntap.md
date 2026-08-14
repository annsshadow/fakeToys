
## Universal TUN/TAP device driver锛堥€氱敤 TUN/TAP 璁惧椹卞姩锛?

Copyright |copy| 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>

  Linux銆丼olaris 椹卞姩绋嬪簭
  Copyright |copy| 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>

  FreeBSD TAP 椹卞姩绋嬪簭
  Copyright |copy| 1999-2000 Maksim Yevmenkin <m_evmenkin@yahoo.com>

  鏈枃妗ｄ簬 2002 骞寸敱 Florian Thiel <florian.thiel@gmx.net> 淇

## 1. 璇存槑锛圖escription锛?

  TUN/TAP 涓虹敤鎴风┖闂寸▼搴忔彁渚涙暟鎹寘鐨勬帴鏀朵笌鍙戦€佽兘鍔涖€?  瀹冨彲浠ヨ鐪嬩綔涓€涓畝鍗曠殑鐐瑰鐐癸紙Point-to-Point锛夎澶囨垨浠ュお缃戯紙Ethernet锛夎澶囷紝
  涓嶅悓涔嬪鍦ㄤ簬锛氬畠涓嶆槸浠庣墿鐞嗕粙璐ㄦ帴鏀舵暟鎹寘锛岃€屾槸浠庣敤鎴风┖闂寸▼搴忔帴鏀讹紱
  鍙戦€佹暟鎹寘鏃朵篃涓嶆槸缁忕敱鐗╃悊浠嬭川锛岃€屾槸鍐欏叆鐢ㄦ埛绌洪棿绋嬪簭銆?
  瑕佷娇鐢ㄦ椹卞姩锛岀▼搴忓繀椤绘墦寮€ /dev/net/tun 骞跺彂鍑虹浉搴旂殑 ioctl() 璋冪敤锛?  鍚戝唴鏍告敞鍐屼竴涓綉缁滆澶囥€傛牴鎹墍閫夋嫨鐨勯€夐」锛岃缃戠粶璁惧浼氬憟鐜颁负 tunXX 鎴?tapXX銆?  褰撶▼搴忓叧闂鏂囦欢鎻忚堪绗︽椂锛岃缃戠粶璁惧鍙婂叾鎵€鏈夌浉鍏宠矾鐢遍兘浼氭秷澶便€?
  鏍规嵁鎵€閫夎澶囩殑绫诲瀷锛岀敤鎴风┖闂寸▼搴忓繀椤昏鍙?鍐欏叆 IP 鏁版嵁鍖咃紙tun 璁惧锛?  鎴栦互澶綉甯э紙tap 璁惧锛夈€傚叿浣撲娇鐢ㄥ摢涓€涓紝鍙栧喅浜?ioctl() 璋冪敤鏃舵墍浼犲叆鐨勬爣蹇椼€?
  http://vtun.sourceforge.net/tun 涓婄殑杞欢鍖呭寘鍚簡涓や釜鍏充簬濡備綍浣跨敤 tun 鍜?tap
  璁惧鐨勭畝鍗曠ず渚嬨€傝繖涓や釜绋嬪簭閮藉儚鏄粙浜庝袱涓綉缁滄帴鍙ｄ箣闂寸殑妗ャ€?  br_select.c - 鍩轰簬 select 绯荤粺璋冪敤鐨勬ˉ銆?  br_sigio.c  - 鍩轰簬寮傛 IO 涓?SIGIO 淇″彿鐨勬ˉ銆?  涓嶈繃锛屾渶濂界殑绀轰緥杩樻槸 VTun http://vtun.sourceforge.net :))

## 2. 閰嶇疆锛圕onfiguration锛?

```

     mkdir /dev/net锛堝鏋滃皻涓嶅瓨鍦級
     mknod /dev/net/tun c 10 200

  璁剧疆鏉冮檺::

     e.g. chmod 0666 /dev/net/tun

  鍏佽闈?root 鐢ㄦ埛璁块棶璇ヨ澶囧苟鏃犲嵄瀹筹紝鍥犱负鍒涘缓缃戠粶璁惧鎴栬€呰繛鎺ュ埌
  涓嶅睘浜庤鐢ㄦ埛鐨勭綉缁滆澶囬兘闇€瑕?CAP_NET_ADMIN 鑳藉姏銆傚鏋滀綘甯屾湜鍒涘缓
  鎸佷箙鍖栬澶囧苟灏嗗叾鎵€鏈夋潈浜ょ粰闈炵壒鏉冪敤鎴凤紝閭ｄ箞灏遍渶瑕佽杩欎簺鐢ㄦ埛鑳藉
  浣跨敤 /dev/net/tun 璁惧銆?
  椹卞姩妯″潡鑷姩鍔犺浇

     璇风‘淇濅綘鐨勫唴鏍镐腑鍚敤浜嗏€淜ernel module loader鈥濃€斺€旀ā鍧楄嚜鍔ㄥ姞杞芥敮鎸併€?     鍐呮牳搴斿綋鍦ㄩ娆¤闂椂鑷姩鍔犺浇瀹冦€?
  鎵嬪姩鍔犺浇

     鎵嬪姩鎻掑叆妯″潡::

	modprobe tun

  濡傛灉浣犻噰鐢ㄥ悗涓€绉嶆柟寮忥紝閭ｄ箞姣忔闇€瑕佹椂灏卞緱鎵嬪姩鍔犺浇妯″潡锛涘鏋滈噰鐢ㄥ墠涓€绉嶆柟寮忥紝
  閭ｄ箞鍦ㄦ墦寮€ /dev/net/tun 鏃朵細鑷姩鍔犺浇銆?
```
## 3. 绋嬪簭鎺ュ彛锛圥rogram interface锛?

### 3.1 缃戠粶璁惧鍒嗛厤锛圢etwork device allocation锛?

`char *dev` 搴斾负甯︽牸寮忓瓧绗︿覆鐨勮澶囧悕锛堜緥濡?"tun%d"锛夛紝涓嶈繃锛堟嵁鎴戞墍鐭ワ級瀹冧篃鍙互鏄换鎰忓悎娉曠殑缃戠粶璁惧鍚嶃€?娉ㄦ剰锛岃瀛楃鎸囬拡浼氳鐪熷疄鐨勮澶囧悕瑕嗙洊銆?```

  #include <linux/if.h>
  #include <linux/if_tun.h>

  int tun_alloc(char *dev)
  {
      struct ifreq ifr;
      int fd, err;

      if( (fd = open("/dev/net/tun", O_RDWR)) < 0 )
	 return tun_alloc_old(dev);

      memset(&ifr, 0, sizeof(ifr));

      /* Flags: IFF_TUN   - TUN device (no Ethernet headers)
       *        IFF_TAP   - TAP device
       *
       *        IFF_NO_PI - Do not provide packet information
       */
      ifr.ifr_flags = IFF_TUN;
      if( *dev )
	 strscpy_pad(ifr.ifr_name, dev, IFNAMSIZ);

      if( (err = ioctl(fd, TUNSETIFF, (void *) &ifr)) < 0 ){
	 close(fd);
	 return err;
      }
      strcpy(dev, ifr.ifr_name);
      return fd;
  }

```
### 3.2 甯ф牸寮忥紙Frame format锛?

```

     Flags [2 bytes]
     Proto [2 bytes]
     Raw protocol(IP, IPv6, etc) frame.

```
### 3.3 澶氶槦鍒?tuntap 鎺ュ彛锛圡ultiqueue tuntap interface锛?

浠?3.8 鐗堟湰寮€濮嬶紝Linux 鏀寔澶氶槦鍒?tuntap锛屽畠鍙互浣跨敤澶氫釜鏂囦欢鎻忚堪绗?锛堥槦鍒楋級鏉ュ苟琛屽湴鍙戦€佹垨鎺ユ敹鏁版嵁鍖呫€傝澶囧垎閰嶆柟寮忎笌姝ゅ墠鐩稿悓锛涘鏋滅敤鎴峰笇鏈?鍒涘缓澶氫釜闃熷垪锛屽垯蹇呴』浣跨敤鐩稿悓鐨勮澶囧悕澶氭璋冪敤甯︽湁 IFF_MULTI_QUEUE 鏍囧織鐨?TUNSETIFF銆?
`char *dev` 搴斾负璁惧鍚嶏紝queues 鏄鍒涘缓鐨勯槦鍒楁暟閲忥紝fds 鐢ㄤ簬瀛樺偍骞跺悜璋冪敤鑰?杩斿洖鎵€鍒涘缓鐨勬枃浠舵弿杩扮锛堥槦鍒楋級銆傛瘡涓枃浠舵弿杩扮閮戒綔涓虹敤鎴风┖闂村彲璁块棶鐨?涓€涓槦鍒楃殑鎺ュ彛銆?
```

  #include <linux/if.h>
  #include <linux/if_tun.h>

  int tun_alloc_mq(char *dev, int queues, int *fds)
  {
      struct ifreq ifr;
      int fd, err, i;

      if (!dev)
	  return -1;

      memset(&ifr, 0, sizeof(ifr));
      /* Flags: IFF_TUN   - TUN device (no Ethernet headers)
       *        IFF_TAP   - TAP device
       *
       *        IFF_NO_PI - Do not provide packet information
       *        IFF_MULTI_QUEUE - Create a queue of multiqueue device
       */
      ifr.ifr_flags = IFF_TAP | IFF_NO_PI | IFF_MULTI_QUEUE;
      strcpy(ifr.ifr_name, dev);

      for (i = 0; i < queues; i++) {
	  if ((fd = open("/dev/net/tun", O_RDWR)) < 0)
	     goto err;
	  err = ioctl(fd, TUNSETIFF, (void *)&ifr);
	  if (err) {
	     close(fd);
	     goto err;
	  }
	  fds[i] = fd;
      }

      return 0;
  err:
      for (--i; i >= 0; i--)
	  close(fds[i]);
      return err;
  }

```
寮曞叆浜嗕竴涓柊鐨?ioctl(TUNSETQUEUE) 鐢ㄤ簬鍚敤鎴栫鐢ㄦ煇涓槦鍒椼€傚綋浠?IFF_DETACH_QUEUE 鏍囧織璋冪敤瀹冩椂锛岃闃熷垪琚鐢紱褰撲互 IFF_ATTACH_QUEUE 鏍囧織
璋冪敤鏃讹紝璇ラ槦鍒楄鍚敤銆傞€氳繃 TUNSETIFF 鍒涘缓鍚庯紝璇ラ槦鍒楅粯璁ゅ浜庡惎鐢ㄧ姸鎬併€?
fd 涓烘垜浠兂瑕佸惎鐢ㄦ垨绂佺敤鐨勬枃浠舵弿杩扮锛堥槦鍒楋級锛屽綋
```

  #include <linux/if.h>
  #include <linux/if_tun.h>

  int tun_set_queue(int fd, int enable)
  {
      struct ifreq ifr;

      memset(&ifr, 0, sizeof(ifr));

      if (enable)
	 ifr.ifr_flags = IFF_ATTACH_QUEUE;
      else
	 ifr.ifr_flags = IFF_DETACH_QUEUE;

      return ioctl(fd, TUNSETQUEUE, (void *)&ifr);
  }

```
## Universal TUN/TAP device driver 甯歌闂锛團requently Asked Question锛?

1. TUN/TAP 椹卞姩鏀寔鍝簺骞冲彴锛?
鐩墠璇ラ┍鍔ㄥ凡閽堝 3 绉?Unix 绯荤粺缂栧啓锛?
  - Linux kernels 2.2.x, 2.4.x
  - FreeBSD 3.x, 4.x, 5.x
  - Solaris 2.6, 7.0, 8.0

2. TUN/TAP 椹卞姩鐨勭敤閫旀槸浠€涔堬紵

濡備笂鎵€杩帮紝TUN/TAP 椹卞姩鐨勪富瑕佺敤閫旀槸闅ч亾锛坱unneling锛夈€?瀹冭 VTun锛坔ttp://vtun.sourceforge.net锛夋墍浣跨敤銆?
鍙︿竴涓娇鐢?TUN/TAP 鐨勬湁瓒ｅ簲鐢ㄦ槸 pipsecd
锛坔ttp://perso.enst.fr/~beyssac/pipsec/锛夛紝杩欐槸涓€涓敤鎴风┖闂?IPSec
瀹炵幇锛屽彲浠ヤ娇鐢ㄥ畬鏁寸殑鍐呮牳璺敱锛堜笉鍚屼簬 FreeS/WAN锛夈€?
3. 铏氭嫙缃戠粶璁惧瀹為檯鏄浣曞伐浣滅殑锛?
铏氭嫙缃戠粶璁惧鍙互鐪嬩綔涓€涓畝鍗曠殑鐐瑰鐐规垨浠ュお缃戣澶囷紝瀹冧笌鏅€氳澶囩殑
涓嶅悓涔嬪鍦ㄤ簬锛氫笉鏄粠鐗╃悊浠嬭川鎺ユ敹鏁版嵁鍖咃紝鑰屾槸浠庣敤鎴风┖闂寸▼搴忔帴鏀讹紱
鍙戦€佹暟鎹寘鏃朵篃涓嶆槸缁忕敱鐗╃悊浠嬭川锛岃€屾槸鍙戦€佺粰鐢ㄦ埛绌洪棿绋嬪簭銆?
鍋囪浣犲湪 tap0 涓婇厤缃簡 IPv6锛岄偅涔堟瘡褰撳唴鏍稿悜 tap0 鍙戦€佷竴涓?IPv6 鏁版嵁鍖呮椂锛?瀹冨氨浼氳浼犻€掔粰搴旂敤绋嬪簭锛堜緥濡?VTun锛夈€傚簲鐢ㄧ▼搴忓璇ュ寘杩涜鍔犲瘑銆佸帇缂╋紝骞堕€氳繃
TCP 鎴?UDP 鍙戦€佸埌瀵圭銆傚绔殑搴旂敤绋嬪簭瀵规敹鍒扮殑鏁版嵁杩涜瑙ｅ帇缂╁拰瑙ｅ瘑锛岀劧鍚?灏嗘暟鎹寘鍐欏叆 TAP 璁惧锛屽唴鏍镐細鍍忓鐞嗘潵鑷湡瀹炵墿鐞嗚澶囩殑鏁版嵁鍖呬竴鏍峰鐞嗗畠銆?
4. TUN 椹卞姩鍜?TAP 椹卞姩鏈変粈涔堝尯鍒紵

TUN 澶勭悊 IP 甯с€俆AP 澶勭悊浠ュお缃戝抚銆?
杩欐剰鍛崇潃浣跨敤 tun 鏃跺繀椤昏鍙?鍐欏叆 IP 鏁版嵁鍖咃紝鑰屼娇鐢?tap 鏃跺垯璇诲彇/鍐欏叆
浠ュお缃戝抚銆?
5. BPF 涓?TUN/TAP 椹卞姩鏈変粈涔堝尯鍒紵

BPF 鏄竴绉嶉珮绾ф暟鎹寘杩囨护鍣ㄣ€傚畠鍙互闄勫姞鍒板凡鏈夌殑缃戠粶鎺ュ彛涓婏紝浣?骞朵笉鎻愪緵铏氭嫙缃戠粶鎺ュ彛銆俆UN/TAP 椹卞姩纭疄鎻愪緵铏氭嫙缃戠粶鎺ュ彛锛屽苟涓斿彲浠?灏?BPF 闄勫姞鍒拌鎺ュ彛涓娿€?
6. TAP 椹卞姩鏀寔鍐呮牳浠ュお缃戞ˉ鎺ュ悧锛?
鏀寔銆侺inux 鍜?FreeBSD 椹卞姩閮芥敮鎸佷互澶綉妗ユ帴銆?