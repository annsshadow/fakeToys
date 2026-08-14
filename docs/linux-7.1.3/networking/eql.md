
## EQL 椹卞姩锛氫覆琛?IP 璐熻浇鍧囪　 HOWTO


  Simon "Guru Aleph-Null" Janes, simon@ncm.com

  v1.1锛?995 骞?2 鏈?27 鏃?
  鏈墜鍐屼粙缁?EQL 璁惧椹卞姩銆侲QL 鏄竴涓蒋浠惰澶囷紝鍙浣犲 IP 涓茶閾捐矾
  锛圫LIP 鎴栨湭鍘嬬缉鐨?PPP锛夎繘琛岃礋杞藉潎琛′互鎻愬崌甯﹀銆傚畠涓嶄細闄嶄綆浣犵殑寤惰繜
  锛堝嵆 ping 鏃堕棿锛夛紝闄ら潪浣犵殑閾捐矾涓婃湰鏉ュ氨鏈夊ぇ閲忔祦閲忥紝閭ｆ牱瀹冧細鏈夋墍甯姪銆?  璇ラ┍鍔ㄥ凡鍦?1.1.75 鍐呮牳涓婃祴璇曪紝骞跺凡鐭ュ彲骞插噣鍦版墦杩?1.1.86銆傚 1.1.92
  涔熷仛杩囦竴浜涙祴璇曪紝浣跨敤鐨勬槸 v1.1 琛ヤ竵锛岃琛ヤ竵浠呬负浜嗗湪鏈€鏂扮殑鍐呮牳婧愮爜鏍戜腑
  骞插噣鍦版墦涓婅€屽垱寤恒€傦紙鏄殑锛屽伐浣滄甯搞€傦級

## 1. 绠€浠?

  鍝釜鏇寸碂锛?6K 涓撶嚎鐨勬槀璐佃垂鐢紝杩樻槸涓ゆ潯鐢佃瘽绾匡紵寰堝彲鑳芥槸鍓嶈€呫€傚鏋滀綘
  娓存湜鏇村甯﹀锛屽苟涓?ISP 姣旇緝鐏垫椿锛岀幇鍦ㄥ彲浠ュ皢澶氫釜璋冨埗瑙ｈ皟鍣ㄧ粦瀹氬湪涓€璧凤紝
  浣滀负涓€鏉＄偣瀵圭偣閾捐矾宸ヤ綔浠ユ彁鍗囧甫瀹姐€傝€屼笖涓ょ閮戒笉闇€瑕佺壒娈婄殑榛戠洅瀛愩€?

  eql 椹卞姩浠呭湪 Livingston PortMaster-2e 缁堢鏈嶅姟鍣ㄤ笂娴嬭瘯杩囥€傛垜涓嶇煡閬撳叾浠?  缁堢鏈嶅姟鍣ㄦ槸鍚︽敮鎸佽礋杞藉潎琛★紝浣嗘垜纭煡 PortMaster 鏀寔锛岃€屼笖鍋氬緱鍑犱箮鍜?  eql 椹卞姩涓€鏍峰ソ锛堚€斺€?閬楁喚鐨勬槸锛屽湪鎴戠洰鍓嶇殑娴嬭瘯涓紝Livingston PortMaster 2e
  鐨勮礋杞藉潎琛℃瘮娴嬭瘯鏈轰娇鐢?28.8 Kbps 涓?14.4 Kbps 杩炴帴鏃惰鎱㈢害 1 鍒?2 KB/s銆?  涓嶈繃鎴戜笉纭畾杩欏埌搴曟槸 PortMaster 鐨勯棶棰橈紝杩樻槸 Linux 鐨?TCP 椹卞姩鐨勯棶棰樸€?  涓嶈繃鏈変汉鍛婅瘔鎴?Linux 鐨?TCP 瀹炵幇鐩稿綋蹇€傗€斺€旓級


  鎴戝悜鍚勪綅 ISP 寤鸿锛屽璐熻浇鍧囪　瀹㈡埛鎸夌浜岀嚎鐨?75%銆佺涓夌嚎鐨?50% 绛夋敹璐?  澶ф姣旇緝鍏钩鈥︹€?

  鍢匡紝澶у閮藉彲浠ュ仛鍋氭ⅵ鍢涒€︹€?

## 2. 鍐呮牳閰嶇疆


  杩欓噷鎴戞弿杩拌鍐呮牳鏀寔 eql 椹卞姩骞舵甯稿伐浣滅殑鎬讳綋姝ラ锛屼粠鎵撹ˉ涓併€佺紪璇戝埌瀹夎銆?

### 2.1. 缁欏唴鏍告墦琛ヤ竵


  濡傛灉浣犳病鏈夋垨鏃犳硶鑾峰緱宸茬粡鍚堝叆 eql 椹卞姩鐨勫唴鏍革紝鍙粠
  ftp://slaughter.ncm.com/pub/Linux/LOAD_BALANCING/eql-1.1.tar.gz 鑾峰彇椹卞姩鍓湰銆?  灏嗗綊妗ｈВ鍖呭埌涓€涓槑鏄剧殑浣嶇疆锛屼緥濡?/usr/local/src/銆傚畠浼?```
       -rw-r--r-- guru/ncm	198 Jan 19 18:53 1995 eql-1.1/NO-WARRANTY
       -rw-r--r-- guru/ncm	30620 Feb 27 21:40 1995 eql-1.1/eql-1.1.patch
       -rwxr-xr-x guru/ncm	16111 Jan 12 22:29 1995 eql-1.1/eql_enslave
       -rw-r--r-- guru/ncm	2195 Jan 10 21:48 1995 eql-1.1/eql_enslave.c

  Unpack a recent kernel (something after 1.1.92) someplace convenient
  like say /usr/src/linux-1.1.92.eql. Use symbolic links to point
  /usr/src/linux to this development directory.


  Apply the patch by running the commands::

       cd /usr/src
       patch </usr/local/src/eql-1.1/eql-1.1.patch


```
### 2.2. 缂栬瘧鍐呮牳


  鎵撳畬琛ヤ竵鍚庯紝杩愯 make config 骞朵负浣犵殑纭欢閰嶇疆鍐呮牳銆?

  閰嶇疆瀹屾垚鍚庯紝鎸変綘鐨勪範鎯繘琛?make 鍜屽畨瑁呫€?

## 3. 缃戠粶閰嶇疆


  鍒扮洰鍓嶄负姝紝鎴戝彧灏?eql 璁惧涓?Matt Dillon 鐨?DSLIP SLIP 杩炴帴绠＄悊鍣ㄤ竴璧?  浣跨敤杩囷紙鈥斺€?"閭ｄ釜涓轰簡蹇€熷啓鍑鸿繖涔堝浠ｇ爜鑰屽崠鎺変簡鐏甸瓊鐨勪汉銆? 鈥斺€旓級銆?  濡備綍涓哄叾浠栤€滆繛鎺モ€濈鐞嗗櫒閰嶇疆瀹冿紝鐢变綘鑷繁鍐冲畾銆傛垜瑙佽繃鐨勫鏁板叾浠栬繛鎺ョ鐞嗗櫒
  鍦ㄥ鐞嗗浜庝竴涓繛鎺ユ椂鍋氬緱骞朵笉濂姐€?

### 3.1. /etc/rc.d/rc.inet1


  鍦?rc.inet1 涓紝鐢?ifconfig 灏?eql 璁惧閰嶇疆涓轰綘鏈哄櫒閫氬父浣跨敤鐨?IP 鍦板潃锛?  浠ュ強浣犲亸濂界殑 SLIP 绾胯矾 MTU銆傛湁浜轰細璇?MTU 瀵逛袱涓皟鍒惰В璋冨櫒搴斿ぇ鑷翠负閫氬父
  澶у皬鐨勪竴鍗婏紝涓変釜涓轰笁鍒嗕箣涓€锛屽洓涓负鍥涘垎涔嬩竴锛屼互姝ょ被鎺ㄢ€︹€︿絾闄嶅埌 296 浠ヤ笅
  鍙兘灏辫繃搴︿簡銆備笅闈㈡槸涓€涓?ifconfig 绀轰緥
```
       ifconfig eql 198.67.33.239 mtu 1006

  Once the eql device is up and running, add a static default route to
  it in the routing table using the cool new route syntax that makes
  life so much easier::

       route add default eql


```
### 3.2. 鎵嬪姩绾冲叆锛坋nslave锛夎澶?

  鎵嬪姩绾冲叆璁惧闇€瑕佷袱涓疄鐢ㄧ▼搴忥細eql_enslave 鍜?eql_emancipate锛堚€斺€?eql_emancipate
  灏氭湭缂栧啓锛屽洜涓哄綋琚撼鍏ョ殑璁惧鈥滄浜♀€濇椂浼氳嚜鍔ㄩ€€鍑洪槦鍒椼€傛垜杩樻病鎵惧埌涓€涓ソ鐨勭悊鐢?  鍘诲啓瀹冣€︹€﹂櫎浜嗕负浜嗗畬鏁存€э紝浣嗛偅涓嶆槸涓ソ鐨勫姩鏈猴紝涓嶆槸鍚楋紵鈥斺€旓級


  绾冲叆璁惧鐨勮娉曟槸 "eql_enslave <master-name>
```
       eql_enslave eql sl0 28800
       eql_enslave eql ppp0 14400
       eql_enslave eql sl1 57600

  When you want to free a device from its life of slavery, you can
  either down the device with ifconfig (eql will automatically bury the
  dead slave and remove it from its queue) or use eql_emancipate to free
  it. (-- Or just ifconfig it down, and the eql driver will take it out
  for you.--)::

       eql_emancipate eql sl0
       eql_emancipate eql ppp0
       eql_emancipate eql sl1


```
### 3.3. eql 璁惧鐨?DSLIP 閰嶇疆


  鎬讳綋鎬濊矾鏄嚜鍔ㄥ缓绔嬪苟淇濇寔鎵€闇€鐨勫敖鍙兘澶氱殑 SLIP 杩炴帴銆?

##### 3.3.1.  /etc/slip/runslip.conf


```
	  name		sl-line-1
	  enabled
	  baud		38400
	  mtu		576
	  ducmd		-e /etc/slip/dialout/cua2-288.xp -t 9
	  command	 eql_enslave eql $interface 28800
	  address	 198.67.33.239
	  line		/dev/cua2

	  name		sl-line-2
	  enabled
	  baud		38400
	  mtu		576
	  ducmd		-e /etc/slip/dialout/cua3-288.xp -t 9
	  command	 eql_enslave eql $interface 28800
	  address	 198.67.33.239
	  line		/dev/cua3


```
### 3.4. 浣跨敤 PPP 鍜?eql 璁惧


  鎴戝皻鏈 PPP 璁惧鍋氫换浣曡礋杞藉潎琛℃祴璇曪紝涓昏鏄洜涓烘垜娌℃湁鍍?SLIP 鏈?DSLIP 閭ｆ牱
  鐨?PPP 杩炴帴绠＄悊鍣ㄣ€傛垜纭疄浠?LinuxNET:Billy 閭ｉ噷寰楀埌涓€涓叧浜?PPP 鎬ц兘鐨勫ソ寤鸿锛?  纭繚灏?asyncmap 璁剧疆涓烘煇涓€硷紝浠ュ厤鎺у埗瀛楃琚浆涔夈€?

  鎴戞浘鍦?95 骞?2 鏈?25-26 鏃ラ偅涓懆鏈皾璇曚负 eql 椹卞姩鎼竴濂楃敤浜庨噸鎷ㄤ涪澶?PPP 杩炴帴鐨?  PPP 鑴氭湰/绯荤粺锛堟鍚庤绉颁负鈥? 灏忔椂 PPP 鐥涙仺鑺傗€濓級銆備篃璁镐粖骞存櫄浜涙椂鍊欏惂銆?

## 4. 鍏充簬浠庤澶囪皟搴︾畻娉?

  浠庤澶囪皟搴﹀櫒寰堝彲鑳藉彲浠ヨ鍗佸嚑涓叾浠栨柟妗堟浛浠ｏ紝浠庤€屾洿蹇湴鎺ㄩ€佹祦閲忋€傚綋鍓嶉┍鍔?  閰嶇疆涓殑鍏紡缁忚繃璋冧紭锛屼互澶勭悊姣旂壒鐜団€滀紭鍏堢骇鈥濆樊寮傚法澶х殑浠庤澶囥€?

  鎴戝仛杩囩殑鎵€鏈夋祴璇曢兘浣跨敤涓や釜 28.8 V.FC 璋冨埗瑙ｈ皟鍣紝涓€涓互 28800 bps 鎴栨洿鎱㈣繛鎺ワ紝
  鍙︿竴涓缁堜互 14400 bps 杩炴帴銆?

  璋冨害鍣ㄧ殑涓€涓増鏈兘澶熷湪杩欎袱鏉?28800 涓?14400 杩炴帴涓婃帹閫?5.3 K/s 鐨勬祦閲忥紝浣嗗綋
  閾捐矾浼樺厛绾у樊璺濆緢澶э紙57600 vs. 14400锛夋椂锛屸€滆緝蹇€濈殑璋冨埗瑙ｈ皟鍣ㄦ帴鏀朵簡鍏ㄩ儴娴侀噺锛?  鑰屸€滆緝鎱⑩€濈殑璋冨埗瑙ｈ皟鍣ㄥ垯琚タ姝汇€?

## 5. 娴嬭瘯鑰呮姤鍛?

  鏈変簺浜虹敤鏇存柊鐨勫唴鏍革紙姣?1.1.75 鏂帮級璇曢獙杩?eql 璁惧銆傜敱浜庢棫寮忊€渟lave-balancing鈥?  椹卞姩閰嶇疆閫夐」琚Щ闄わ紝鎴戞鍚庡凡灏嗛┍鍔ㄦ洿鏂颁负鍙湪鏇存柊鐨勫唴鏍镐腑骞插噣鍦版墦琛ヤ竵銆?

  - LinuxNET 鐨?icee 鍦ㄦ病鏈変换浣?reject 鐨勬儏鍐典笅缁?1.1.86 鎵撲笂浜嗚ˉ涓侊紝骞惰兘澶?     鍚姩鍐呮牳骞剁撼鍏ュ嚑鏉?ISDN PPP 閾捐矾銆?
### 5.1. Randolph Bentson 鐨勬祴璇曟姤鍛?

```
    From bentson@grieg.seaslug.org Wed Feb  8 19:08:09 1995
    Date: Tue, 7 Feb 95 22:57 PST
    From: Randolph Bentson <bentson@grieg.seaslug.org>
    To: guru@ncm.com
    Subject: EQL driver tests


    I have been checking out your eql driver.  (Nice work, that!)
    Although you may already done this performance testing, here
    are some data I've discovered.

    Randolph Bentson
    bentson@grieg.seaslug.org

```
------------------------------------------------------------------


  EQL 鏄竴涓敱 Simon Janes 缂栧啓鐨勪吉璁惧椹卞姩锛屽彲鐢ㄤ簬灏嗗鏉?SLIP 杩炴帴鎹嗙粦涓?  鐪嬩技鍗曚竴鐨勮繛鎺ャ€傝繖璁╀汉鑳藉閫愭鏀瑰杽鎷ㄥ彿缃戠粶杩炴帴锛岃€屾棤闇€璐拱鏄傝吹鐨?DSU/CSU
  纭欢鍜屾湇鍔°€?
  鎴戝璇ヨ蒋浠跺仛浜嗕竴浜涙祴璇曪紝蹇冮噷鏈変袱涓洰鏍囷細涓€鏄‘璁ゅ畠纭疄濡傛弿杩扮殑閭ｆ牱宸ヤ綔锛?  浜屾槸浣滀负閿荤偧鎴戠殑璁惧椹卞姩鐨勪竴绉嶆柟娉曘€?
  浠ヤ笅鎬ц兘娴嬮噺鏁版嵁鏉ヨ嚜鍦ㄤ竴缁勪袱涓?Linux 绯荤粺锛?.1.84锛変箣闂磋繍琛岀殑 SLIP 杩炴帴锛?  涓€绔娇鐢?486DX2/66 閰?Cyclom-8Ys锛屽彟涓€绔娇鐢?486SLC/40 閰?Cyclom-16Y銆?  锛堜娇鐢ㄤ簡绔彛 0,1,2,3銆備箣鍚庣殑閰嶇疆浼氬皢绔彛閫夋嫨鍒嗘暎鍒版澘涓婄殑涓嶅悓 Cirrus 鑺墖涓娿€傦級
  涓€鏃﹀缓绔嬮摼璺紝鎴戝涓€娆?289284 瀛楄妭鏁版嵁鐨勪簩杩涘埗 ftp 浼犺緭璁℃椂銆傚鏋滄病鏈変换浣?  寮€閿€锛堝寘澶撮儴銆佸瓧绗﹂棿涓庡寘闂村欢杩熺瓑锛夛紝浼犺緭搴?```
      bits/sec	seconds
      345600	8.3
      234600	12.3
      172800	16.7
      153600	18.8
      76800	37.6
      57600	50.2
      38400	75.3
      28800	100.4
      19200	150.6
      9600	301.3

  A single line running at the lower speeds and with large packets
  comes to within 2% of this.  Performance is limited for the higher
  speeds (as predicted by the Cirrus databook) to an aggregate of
  about 160 kbits/sec.	The next round of testing will distribute
  the load across two or more Cirrus chips.

  The good news is that one gets nearly the full advantage of the
  second, third, and fourth line's bandwidth.  (The bad news is
  that the connection establishment seemed fragile for the higher
  speeds.  Once established, the connection seemed robust enough.)

  ======  ========	===  ========   ======= ======= ===
  #lines  speed		mtu  seconds	theory  actual  %of
	  kbit/sec	     duration	speed	speed	max
  ======  ========	===  ========   ======= ======= ===
  3	  115200	900	_	345600
  3	  115200	400	18.1	345600  159825  46
  2	  115200	900	_	230400
  2	  115200	600	18.1	230400  159825  69
  2	  115200	400	19.3	230400  149888  65
  4	  57600		900	_	234600
  4	  57600		600	_	234600
  4	  57600		400	_	234600
  3	  57600		600	20.9	172800  138413  80
  3	  57600		900	21.2	172800  136455  78
  3	  115200	600	21.7	345600  133311  38
  3	  57600		400	22.5	172800  128571  74
  4	  38400		900	25.2	153600  114795  74
  4	  38400		600	26.4	153600  109577  71
  4	  38400		400	27.3	153600  105965  68
  2	  57600		900	29.1	115200  99410.3 86
  1	  115200	900	30.7	115200  94229.3 81
  2	  57600		600	30.2	115200  95789.4 83
  3	  38400		900	30.3	115200  95473.3 82
  3	  38400		600	31.2	115200  92719.2 80
  1	  115200	600	31.3	115200  92423	80
  2	  57600		400	32.3	115200  89561.6 77
  1	  115200	400	32.8	115200  88196.3 76
  3	  38400		400	33.5	115200  86353.4 74
  2	  38400		900	43.7	76800	66197.7 86
  2	  38400		600	44	76800	65746.4 85
  2	  38400		400	47.2	76800	61289	79
  4	  19200		900	50.8	76800	56945.7 74
  4	  19200		400	53.2	76800	54376.7 70
  4	  19200		600	53.7	76800	53870.4 70
  1	  57600		900	54.6	57600	52982.4 91
  1	  57600		600	56.2	57600	51474	89
  3	  19200		900	60.5	57600	47815.5 83
  1	  57600		400	60.2	57600	48053.8 83
  3	  19200		600	62	57600	46658.7 81
  3	  19200		400	64.7	57600	44711.6 77
  1	  38400		900	79.4	38400	36433.8 94
  1	  38400		600	82.4	38400	35107.3 91
  2	  19200		900	84.4	38400	34275.4 89
  1	  38400		400	86.8	38400	33327.6 86
  2	  19200		600	87.6	38400	33023.3 85
  2	  19200		400	91.2	38400	31719.7 82
  4	  9600		900	94.7	38400	30547.4 79
  4	  9600		400	106	38400	27290.9 71
  4	  9600		600	110	38400	26298.5 68
  3	  9600		900	118	28800	24515.6 85
  3	  9600		600	120	28800	24107	83
  3	  9600		400	131	28800	22082.7 76
  1	  19200		900	155	19200	18663.5 97
  1	  19200		600	161	19200	17968	93
  1	  19200		400	170	19200	17016.7 88
  2	  9600		600	176	19200	16436.6 85
  2	  9600		900	180	19200	16071.3 83
  2	  9600		400	181	19200	15982.5 83
  1	  9600		900	305	9600	9484.72 98
  1	  9600		600	314	9600	9212.87 95
  1	  9600		400	332	9600	8713.37 90
  ======  ========	===  ========   ======= ======= ===

```
### 5.2. Anthony Healy 鐨勬姤鍛?

```
    Date: Mon, 13 Feb 1995 16:17:29 +1100 (EST)
    From: Antony Healey <ahealey@st.nepean.uws.edu.au>
    To: Simon Janes <guru@ncm.com>
    Subject: Re: Load Balancing

    Hi Simon,
	  I've installed your patch and it works great. I have trialed
	  it over twin SL/IP lines, just over null modems, but I was
	  able to data at over 48Kb/s [ISDN link -Simon]. I managed a
	  transfer of up to 7.5 Kbyte/s on one go, but averaged around
	  6.4 Kbyte/s, which I think is pretty cool.  :)

```
