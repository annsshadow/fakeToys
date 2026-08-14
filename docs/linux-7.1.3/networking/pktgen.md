
## Linux 鏁版嵁鍖呯敓鎴愬櫒锛坧acket generator锛変娇鐢ㄦ寚鍗?
鍚敤 CONFIG_NET_PKTGEN 浠ョ紪璇戝苟鏋勫缓 pktgen锛屽彲浠ュ唴缃埌鍐呮牳鎴栦綔涓烘ā鍧椼€傛帹鑽愪娇鐢ㄦā鍧楋紱濡傛灉闇€瑕佸垯 modprobe pktgen銆備竴鏃﹁繍琛岋紝pktgen 浼氫负姣忎釜 CPU 鍒涘缓涓€涓嚎绋嬶紝骞跺皢浜插拰鎬х粦瀹氬埌璇?CPU銆傜洃鎺у拰鎺у埗閫氳繃 /proc 瀹屾垚銆傛渶绠€鍗曠殑鏂规硶鏄€夋嫨涓€涓悎閫傜殑绀轰緥鑴氭湰骞堕厤缃畠銆?
```
    ps aux | grep pkt
    root       129  0.3  0.0     0    0 ?        SW    2003 523:20 [kpktgend_0]
    root       130  0.3  0.0     0    0 ?        SW    2003 509:50 [kpktgend_1]

```
```
	/proc/net/pktgen/pgctrl
	/proc/net/pktgen/kpktgend_X
	/proc/net/pktgen/ethX

```
## 涓烘渶澶ф€ц兘璋冧紭 NIC

榛樿鐨?NIC 璁剧疆锛堝彲鑳斤級骞舵湭閽堝 pktgen 杩欑浜轰负杩囪浇绫诲瀷鐨勫熀鍑嗘祴璇曡繘琛岃皟浼橈紝鍥犱负杩欎細鎹熷姝ｅ父浣跨敤鍦烘櫙銆?
```
 # ethtool -G ethX tx 1024

```
鏇村ぇ鐨?TX 鐜彲浠ユ彁楂?pktgen 鐨勬€ц兘锛屼絾鍦ㄤ竴鑸儏鍐典笅瀹冧細鏈夊锛?锛夊洜涓?TX 鐜紦鍐插尯鍙兘鍙樺緱姣?CPU 鐨?L1/L2 缂撳瓨鏇村ぇ锛?锛夊洜涓哄畠鍏佽鍦?NIC 纭欢灞傛湁鏇村鐨勬帓闃燂紙杩欏缂撳啿鍖鸿啫鑳€ bufferbloat 涓嶅埄锛夈€?
浜轰滑涓嶅簲鍖嗗繖寰楀嚭 HW TX 鐜腑鐨勬暟鎹寘/鎻忚堪绗︿細閫犳垚寤惰繜鐨勭粨璁恒€傞┍鍔ㄩ€氬父鍑轰簬鍚勭鎬ц兘鍘熷洜鑰屽欢杩熸竻鐞嗙幆缂撳啿鍖猴紝鍋滄粸鍦?TX 鐜腑鐨勬暟鎹寘鍙兘鍙槸鍦ㄧ瓑寰呮竻鐞嗐€?
杩欎釜娓呯悊闂鐗瑰埆閫傜敤浜庨┍鍔?ixgbe锛圛ntel 82599 鑺墖锛夈€傝椹卞姩锛坕xgbe锛夊皢 TX+RX 鐜竻鐞嗗悎骞讹紝鑰屾竻鐞嗛棿闅斿彈 ethtool --coalesce 璁剧疆涓殑鍙傛暟 "rx-usecs" 褰卞搷銆?
```
 # ethtool -C ethX rx-usecs 30

```
## 鍐呮牳绾跨▼

Pktgen 涓烘瘡涓?CPU 鍒涘缓涓€涓嚎绋嬶紝骞跺皢浜插拰鎬х粦瀹氬埌璇?CPU銆傝繖鍙互閫氳繃 proc 鏂囦欢 /proc/net/pktgen/kpktgend_X 鎺у埗銆?
```
 Running:
 Stopped: eth4@0
 Result: OK: add_device=eth4@0

```
鏈€閲嶈鐨勬槸鍒嗛厤缁欑嚎绋嬬殑璁惧銆?
涓や釜鍩烘湰鐨勭嚎绋嬪懡浠ゆ槸锛?
 - add_device DEVICE@NAME -- 娣诲姞涓€涓崟涓€璁惧
 - rem_device_all         -- 绉婚櫎鎵€鏈夊叧鑱旂殑璁惧

褰撳悜绾跨▼娣诲姞璁惧鏃讹紝浼氬垱寤轰竴涓浉搴旂殑 proc 鏂囦欢锛岀敤浜庨厤缃璁惧銆傚洜姝わ紝璁惧鍚嶇О闇€瑕佸敮涓€銆?
涓轰簡鏀寔灏嗗悓涓€璁惧娣诲姞鍒板涓嚎绋嬶紙杩欏澶氶槦鍒?NIC 寰堟湁鐢級锛岃澶囧懡鍚嶆柟妗堢敤 "@" 杩涜浜嗘墿灞曪細device@something

"@" 涔嬪悗鐨勯儴鍒嗗彲浠ユ槸浠绘剰鍐呭锛屼絾閫氬父涔犳儻浣跨敤绾跨▼鍙枫€?
## 鏌ョ湅璁惧

Params 閮ㄥ垎淇濆瓨閰嶇疆淇℃伅銆侰urrent 閮ㄥ垎淇濆瓨杩愯缁熻淇℃伅銆俁esult 鍦ㄤ竴娆¤繍琛屽悗鎴栧湪涔嬪悗鎵撳嵃銆?
```
    /proc/net/pktgen/eth4@0

    Params: count 100000  min_pkt_size: 60  max_pkt_size: 60
	frags: 0  delay: 0  clone_skb: 64  ifname: eth4@0
	flows: 0 flowlen: 0
	queue_map_min: 0  queue_map_max: 0
	dst_min: 192.168.81.2  dst_max:
	src_min:   src_max:
	src_mac: 90:e2:ba:0a:56:b4 dst_mac: 00:1b:21:3c:9d:f8
	udp_src_min: 9  udp_src_max: 109  udp_dst_min: 9  udp_dst_max: 9
	src_mac_count: 0  dst_mac_count: 0
	Flags: UDPSRC_RND  NO_TIMESTAMP  QUEUE_MAP_CPU
    Current:
	pkts-sofar: 100000  errors: 0
	started: 623913381008us  stopped: 623913396439us idle: 25us
	seq_num: 100001  cur_dst_mac_offset: 0  cur_src_mac_offset: 0
	cur_saddr: 192.168.8.3  cur_daddr: 192.168.81.2
	cur_udp_dst: 9  cur_udp_src: 42
	cur_queue_map: 0
	flows: 0
    Result: OK: 15430(c15405+d25) usec, 100000 (60byte,0frags)
    6480562pps 3110Mb/sec (3110669760bps) errors: 0

```
## 閰嶇疆璁惧

杩欐槸閫氳繃 /proc 鎺ュ彛瀹屾垚鐨勶紝骞朵笖鏈€瀹规槗閫氳繃绀轰緥鑴氭湰涓畾涔夌殑 pgset 鏉ュ畬鎴愩€備綘闇€瑕佹寚瀹?PGDEV 鐜鍙橀噺鏉ヤ娇鐢ㄧず渚嬭剼鏈腑鐨勫嚱鏁般€?
```
    export PGDEV=/proc/net/pktgen/eth4@0
    source samples/pktgen/functions.sh

```
```
 pg_ctrl start           starts injection.
 pg_ctrl stop            aborts injection. Also, ^C aborts generator.

 pgset "clone_skb 1"     sets the number of copies of the same packet
 pgset "clone_skb 0"     use single SKB for all transmits
 pgset "burst 8"         uses xmit_more API to queue 8 copies of the same
			 packet and update HW tx queue tail pointer once.
			 "burst 1" is the default
 pgset "pkt_size 9014"   sets packet size to 9014
 pgset "frags 5"         packet will consist of 5 fragments
 pgset "count 200000"    sets number of packets to send, set to zero
			 for continuous sends until explicitly stopped.

 pgset "delay 5000"      adds delay to hard_start_xmit(). nanoseconds

 pgset "dst 10.0.0.1"    sets IP destination address
			 (BEWARE! This generator is very aggressive!)

 pgset "dst_min 10.0.0.1"            Same as dst
 pgset "dst_max 10.0.0.254"          Set the maximum destination IP.
 pgset "src_min 10.0.0.1"            Set the minimum (or only) source IP.
 pgset "src_max 10.0.0.254"          Set the maximum source IP.
 pgset "dst6 fec0::1"     IPV6 destination address
 pgset "src6 fec0::2"     IPV6 source address
 pgset "dstmac 00:00:00:00:00:00"    sets MAC destination address
 pgset "srcmac 00:00:00:00:00:00"    sets MAC source address

 pgset "queue_map_min 0" Sets the min value of tx queue interval
 pgset "queue_map_max 7" Sets the max value of tx queue interval, for multiqueue devices
			 To select queue 1 of a given device,
			 use queue_map_min=1 and queue_map_max=1

 pgset "src_mac_count 1" Sets the number of MACs we'll range through.
			 The 'minimum' MAC is what you set with srcmac.

 pgset "dst_mac_count 1" Sets the number of MACs we'll range through.
			 The 'minimum' MAC is what you set with dstmac.

 pgset "flag [name]"     Set a flag to determine behaviour.  Current flags
			 are: IPSRC_RND # IP source is random (between min/max)
			      IPDST_RND # IP destination is random
			      UDPSRC_RND, UDPDST_RND,
			      MACSRC_RND, MACDST_RND
			      TXSIZE_RND, IPV6,
			      MPLS_RND, VID_RND, SVID_RND
			      FLOW_SEQ,
			      QUEUE_MAP_RND # queue map random
			      QUEUE_MAP_CPU # queue map mirrors smp_processor_id()
			      UDPCSUM,
			      IPSEC # IPsec encapsulation (needs CONFIG_XFRM)
			      NODE_ALLOC # node specific memory allocation
			      NO_TIMESTAMP # disable timestamping
			      SHARED # enable shared SKB
 pgset 'flag ![name]'    Clear a flag to determine behaviour.
			 Note that you might need to use single quote in
			 interactive mode, so that your shell wouldn't expand
			 the specified flag as a history command.

 pgset "spi [SPI_VALUE]" Set specific SA used to transform packet.

 pgset "udp_src_min 9"   set UDP source port min, If < udp_src_max, then
			 cycle through the port range.

 pgset "udp_src_max 9"   set UDP source port max.
 pgset "udp_dst_min 9"   set UDP destination port min, If < udp_dst_max, then
			 cycle through the port range.
 pgset "udp_dst_max 9"   set UDP destination port max.

 pgset "mpls 0001000a,0002000a,0000000a" set MPLS labels (in this example
					 outer label=16,middle label=32,
					 inner label=0 (IPv4 NULL)) Note that
					 there must be no spaces between the
					 arguments. Leading zeros are required.
					 Do not set the bottom of stack bit,
					 that's done automatically. If you do
					 set the bottom of stack bit, that
					 indicates that you want to randomly
					 generate that address and the flag
					 MPLS_RND will be turned on. You
					 can have any mix of random and fixed
					 labels in the label stack.

 pgset "mpls 0"		  turn off mpls (or any invalid argument works too!)

 pgset "vlan_id 77"       set VLAN ID 0-4095
 pgset "vlan_p 3"         set priority bit 0-7 (default 0)
 pgset "vlan_cfi 0"       set canonical format identifier 0-1 (default 0)

 pgset "svlan_id 22"      set SVLAN ID 0-4095
 pgset "svlan_p 3"        set priority bit 0-7 (default 0)
 pgset "svlan_cfi 0"      set canonical format identifier 0-1 (default 0)

 pgset "vlan_id 9999"     > 4095 remove vlan and svlan tags
 pgset "svlan 9999"       > 4095 remove svlan tag


 pgset "tos XX"           set former IPv4 TOS field (e.g. "tos 28" for AF11 no ECN, default 00)
 pgset "traffic_class XX" set former IPv6 TRAFFIC CLASS (e.g. "traffic_class B8" for EF no ECN, default 00)

 pgset "rate 300M"        set rate to 300 Mb/s
 pgset "ratep 1000000"    set rate to 1Mpps

 pgset "xmit_mode netif_receive"  RX inject into stack netif_receive_skb()
				  Works with "burst" but not with "clone_skb".
				  Default xmit_mode is "start_xmit".

```
## 绀轰緥鑴氭湰

samples/pktgen 鐩綍涓寘鍚竴缁?pktgen 鐨勬暀绋嬭剼鏈拰杈呭姪宸ュ叿銆傝緟鍔╂枃浠?parameters.sh 鏀寔鍦ㄥ悇绀轰緥鑴氭湰涔嬮棿杩涜绠€鍗曚笖涓€鑷寸殑鍙傛暟瑙ｆ瀽銆?
```
 ./pktgen_sample01_simple.sh -i eth4 -m 00:1B:21:3C:9D:F8 -d 192.168.8.2

```
```
  ./pktgen_sample01_simple.sh [-vx] -i ethX

  -i : ($DEV)       output interface/device (required)
  -s : ($PKT_SIZE)  packet size
  -d : ($DEST_IP)   destination IP. CIDR (e.g. 198.18.0.0/15) is also allowed
  -m : ($DST_MAC)   destination MAC-addr
  -p : ($DST_PORT)  destination PORT range (e.g. 433-444) is also allowed
  -t : ($THREADS)   threads to start
  -f : ($F_THREAD)  index of first thread (zero indexed CPU number)
  -c : ($SKB_CLONE) SKB clones send before alloc new SKB
  -n : ($COUNT)     num messages to send per thread, 0 means indefinitely
  -b : ($BURST)     HW level bursting of SKBs
  -v : ($VERBOSE)   verbose
  -x : ($DEBUG)     debug
  -6 : ($IP6)       IPv6
  -w : ($DELAY)     Tx Delay value (ns)
  -a : ($APPEND)    Script will not reset generator's state, but will append its config

```
鎵€鍒楀嚭鐨勫叏灞€鍙橀噺涔熷湪鍏朵腑銆備緥濡傦紝蹇呴渶鐨勬帴鍙?璁惧鍙傛暟 "-i" 璁剧疆浜嗗彉閲?$DEV銆傚鍒?pktgen_sampleXX 鑴氭湰骞朵慨鏀瑰畠浠互閫傚簲浣犺嚜宸辩殑闇€瑕併€?

## 涓柇浜插拰鎬?
娉ㄦ剰锛屽綋鍚戠壒瀹?CPU 娣诲姞璁惧鏃讹紝鍚屾椂鍒嗛厤 /proc/irq/XX/smp_affinity 浠ュ皢 TX 涓柇缁戝畾鍒板悓涓€ CPU 鏄釜濂戒富鎰忋€傝繖鍑忓皯浜嗛噴鏀?skb 鏃剁殑缂撳瓨鎶栧姩锛坈ache bouncing锛夈€?
姝ゅ浣跨敤璁惧鏍囧織 QUEUE_MAP_CPU锛屽畠灏?SKB 鐨?TX 闃熷垪鏄犲皠鍒拌繍琛岀嚎绋嬬殑 CPU锛堢洿鎺ユ潵鑷?smp_processor_id()锛夈€?
## 鍚敤 IPsec

榛樿鐨?IPsec 杞崲浣跨敤 ESP 灏佽鍔犱紶杈撴ā寮?
```
    pgset "flag IPSEC"
    pgset "flows 1"

```
涓轰簡閬垮厤鐮村潖鐜版湁鐨勭敤浜?AH 绫诲瀷鍜岄毀閬撴ā寮忕殑娴嬭瘯搴婅剼鏈紝浣犲彲浠ヤ娇鐢?"pgset spi SPI_VALUE" 鏉ユ寚瀹氳閲囩敤鐨勮浆鎹㈡ā寮忋€?
## 绂佺敤鍏变韩 SKB

榛樿鎯呭喌涓嬶紝pktgen 鍙戦€佺殑 SKB 鏄叡浜殑锛堢敤鎴疯鏁?> 1锛夈€?
```
	pg_set "flag !SHARED"

```
鐒惰€岋紝濡傛灉閰嶇疆浜?"clone_skb" 鎴?"burst" 鍙傛暟锛宻kb 浠嶉渶瑕佽 pktgen 鎸佹湁浠ヤ究杩涗竴姝ヨ闂€傚洜姝よ skb 蹇呴』鏄叡浜殑銆?
## 褰撳墠鍛戒护涓庨厤缃€夐」

```
    start
    stop
    reset

```
```
    add_device
    rem_device_all

```
```
    count
    clone_skb
    burst
    debug

    frags
    delay

    src_mac_count
    dst_mac_count

    pkt_size
    min_pkt_size
    max_pkt_size

    queue_map_min
    queue_map_max
    skb_priority

    tos           (ipv4)
    traffic_class (ipv6)

    mpls

    udp_src_min
    udp_src_max

    udp_dst_min
    udp_dst_max

    node

    flag
    IPSRC_RND
    IPDST_RND
    UDPSRC_RND
    UDPDST_RND
    MACSRC_RND
    MACDST_RND
    TXSIZE_RND
    IPV6
    MPLS_RND
    VID_RND
    SVID_RND
    FLOW_SEQ
    QUEUE_MAP_RND
    QUEUE_MAP_CPU
    UDPCSUM
    IPSEC
    NODE_ALLOC
    NO_TIMESTAMP
    SHARED

    spi (ipsec)

    dst_min
    dst_max

    src_min
    src_max

    dst_mac
    src_mac

    clear_counters

    src6
    dst6
    dst6_max
    dst6_min

    flows
    flowlen

    rate
    ratep

    xmit_mode <start_xmit|netif_receive>

    vlan_cfi
    vlan_id
    vlan_p

    svlan_cfi
    svlan_id
    svlan_p

```
鍙傝€冩枃鐚細

- ftp://robur.slu.se/pub/Linux/net-development/pktgen-testing/
- ftp://robur.slu.se/pub/Linux/net-development/pktgen-testing/examples/

Linux-Kongress in Erlangen 2004 鐨勮鏂囥€?- ftp://robur.slu.se/pub/Linux/net-development/pktgen-testing/pktgen_paper.pdf

鎰熻阿锛?
Grant Grundler 鍦?IA-64 鍜?parisc 涓婄殑娴嬭瘯锛孒arald Welte銆丩ennert Buytenhek銆丼tephen Hemminger銆丄ndi Kleen銆丏ave Miller 浠ュ強璁稿鍏朵粬浜恒€?

绁?Linux 缃戠粶寮€鍙戦『鍒┿€?