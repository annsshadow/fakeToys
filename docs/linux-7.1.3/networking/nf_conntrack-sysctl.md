
## Netfilter 杩炴帴璺熻釜 Sysfs 鍙橀噺


## /proc/sys/net/netfilter/nf_conntrack_* 鍙橀噺锛?

nf_conntrack_acct - BOOLEAN
 - 0 - 绂佺敤锛堥粯璁わ級
 - 闈?0 - 鍚敤

	鍚敤杩炴帴璺熻釜娴佽璐︺€備細涓烘瘡涓祦娣诲姞 64 浣嶅瓧鑺備笌鍖呰鏁板櫒銆?
nf_conntrack_buckets - INTEGER
	鍝堝笇琛ㄧ殑澶у皬銆傚鏋滃湪妯″潡鍔犺浇鏃舵湭浣滀负鍙傛暟鎸囧畾锛岄粯璁ゅぇ灏忛€氳繃灏嗘€诲唴瀛橀櫎浠?16384
	鏉ョ‘瀹氭《鐨勬暟閲忋€傚搱甯岃〃姘歌繙涓嶄細灏戜簬 1024 涓《锛屼篃姘歌繙涓嶄細澶氫簬 262144 涓《銆?	璇?sysctl 浠呭湪鍒濆缃戠粶鍛藉悕绌洪棿涓彲鍐欍€?
nf_conntrack_checksum - BOOLEAN
 - 0 - 绂佺敤
 - 闈?0 - 鍚敤锛堥粯璁わ級

	鏍￠獙鍏ュ寘鏍￠獙鍜屻€傛牎楠屽拰閿欒鐨勫寘澶勪簬 INVALID 鐘舵€併€傚鏋滃惎鐢ㄦ閫夐」锛屾绫诲寘灏嗕笉琚?	鑰冭檻鐢ㄤ簬杩炴帴璺熻釜銆?
nf_conntrack_count - INTEGER锛堝彧璇伙級
	褰撳墠宸插垎閰嶇殑娴佹潯鐩暟閲忋€?
nf_conntrack_events - BOOLEAN
 - 0 - 绂佺敤
 - 1 - 鍚敤
 - 2 - 鑷姩锛堥粯璁わ級

	濡傛灉鍚敤姝ら€夐」锛岃繛鎺ヨ窡韪唬鐮佸皢閫氳繃 ctnetlink 鍚戠敤鎴风┖闂存彁渚涜繛鎺ヨ窡韪簨浠躲€傞粯璁?	鎯呭喌涓嬶紝濡傛灉鏈夌敤鎴风┖闂寸▼搴忔鍦ㄧ洃鍚?ctnetlink 浜嬩欢锛屽垯鍒嗛厤璇ユ墿灞曘€?
nf_conntrack_expect_max - INTEGER
	鏈熸湜锛坋xpectation锛夎〃鐨勬渶澶уぇ灏忋€傞粯璁ゅ€间负 nf_conntrack_buckets / 256銆傛渶灏忓€间负 1銆?
nf_conntrack_frag6_high_thresh - INTEGER
	default 262144

	鐢ㄤ簬閲嶇粍 IPv6 鍒嗙墖鐨勬渶澶у唴瀛樸€傚綋涓轰笂杩扮洰鐨勫垎閰嶄簡 nf_conntrack_frag6_high_thresh
	瀛楄妭鐨勫唴瀛樻椂锛屽垎鐗囧鐞嗙▼搴忓皢涓㈠純鍖咃紝鐩村埌杈惧埌 nf_conntrack_frag6_low_thresh銆?
nf_conntrack_frag6_low_thresh - INTEGER
	default 196608

	鍙傝 nf_conntrack_frag6_low_thresh

nf_conntrack_frag6_timeout - INTEGER锛堢锛?	default 60

	鍦ㄥ唴瀛樹腑淇濈暀 IPv6 鍒嗙墖鐨勬椂闀裤€?
nf_conntrack_generic_timeout - INTEGER锛堢锛?	default 600

	閫氱敤瓒呮椂鐨勯粯璁ゅ€笺€傝繖鎸囩殑鏄 4 灞傛湭鐭?涓嶆敮鎸佺殑鍗忚銆?
nf_conntrack_icmp_timeout - INTEGER锛堢锛?	default 30

	ICMP 瓒呮椂鐨勯粯璁ゅ€笺€?
nf_conntrack_icmpv6_timeout - INTEGER锛堢锛?	default 30

	ICMP6 瓒呮椂鐨勯粯璁ゅ€笺€?
nf_conntrack_log_invalid - INTEGER
 - 0   - 绂佺敤锛堥粯璁わ級
 - 1   - 璁板綍 ICMP 鍖? - 6   - 璁板綍 TCP 鍖? - 17  - 璁板綍 UDP 鍖? - 41  - 璁板綍 ICMPv6 鍖? - 136 - 璁板綍 UDPLITE 鍖? - 255 - 璁板綍浠绘剰鍗忚鐨勫寘

	璁板綍鐢卞€兼寚瀹氱殑绫诲瀷鐨勬棤鏁堝寘銆?
nf_conntrack_max - INTEGER
        鍏佽鐨勮繛璺熻釜鏉＄洰鐨勬渶澶ф暟閲忋€傞粯璁ゆ儏鍐典笅璇ュ€艰涓?nf_conntrack_buckets銆傛敞鎰忥紝
        杩炴帴璺熻釜鏉＄洰浼氳鍔犲叆琛ㄤ腑涓ゆ鈥斺€斾竴娆＄敤浜庡師濮嬫柟鍚戯紝涓€娆＄敤浜庡洖澶嶆柟鍚戯紙鍗冲湴鍧€
        鍙嶈浆锛夈€傝繖鎰忓懗鐫€榛樿璁剧疆涓嬶紝琛ㄦ弧鏃剁殑骞冲潎鍝堝笇閾鹃暱搴︿负 2锛岃€屼笉鏄?1銆?
nf_conntrack_tcp_be_liberal - BOOLEAN
 - 0 - 绂佺敤锛堥粯璁わ級
 - 闈?0 - 鍚敤

	涓ヤ簬寰嬪繁锛屽浠ュ緟浜猴紙鍦ㄤ綘瑕佸仛鐨勪簨涓婁繚瀹堬紝鍦ㄤ粠浠栦汉澶勬帴鍙楃殑涓滆タ涓婂鏉撅級銆傚鏋滈潪闆讹紝
	鎴戜滑鍙皢绐楀彛澶栫殑 RST 娈垫爣璁颁负 INVALID銆?
nf_conntrack_tcp_ignore_invalid_rst - BOOLEAN
 - 0 - 绂佺敤锛堥粯璁わ級
 - 1 - 鍚敤

	濡傛灉涓?1锛屾垜浠笉灏嗙獥鍙ｅ鐨?RST 娈垫爣璁颁负 INVALID銆?
nf_conntrack_tcp_loose - BOOLEAN
 - 0 - 绂佺敤
 - 闈?0 - 鍚敤锛堥粯璁わ級

	濡傛灉璁句负 0锛屾垜浠皢绂佺敤鎷惧彇锛坧ick up锛夊凡寤虹珛鐨勮繛鎺ャ€?
nf_conntrack_tcp_max_retrans - INTEGER
	default 3

	鍦ㄦ湭鏀跺埌鏉ヨ嚜鐩殑鍦扮殑锛堝彲鎺ュ彈鐨勶級ACK 鐨勬儏鍐典笅鍙互閲嶄紶鐨勬渶澶у寘鏁般€傚鏋滆揪鍒版鏁帮紝
	灏嗗惎鍔ㄤ竴涓洿鐭殑瀹氭椂鍣ㄣ€?
nf_conntrack_tcp_timeout_close - INTEGER锛堢锛?	default 10

nf_conntrack_tcp_timeout_close_wait - INTEGER锛堢锛?	default 60

nf_conntrack_tcp_timeout_established - INTEGER锛堢锛?	default 432000锛? 澶╋級

nf_conntrack_tcp_timeout_fin_wait - INTEGER锛堢锛?	default 120

nf_conntrack_tcp_timeout_last_ack - INTEGER锛堢锛?	default 30

nf_conntrack_tcp_timeout_max_retrans - INTEGER锛堢锛?	default 300

nf_conntrack_tcp_timeout_syn_recv - INTEGER锛堢锛?	default 60

nf_conntrack_tcp_timeout_syn_sent - INTEGER锛堢锛?	default 120

nf_conntrack_tcp_timeout_time_wait - INTEGER锛堢锛?	default 120

nf_conntrack_tcp_timeout_unacknowledged - INTEGER锛堢锛?	default 300

nf_conntrack_timestamp - BOOLEAN
 - 0 - 绂佺敤锛堥粯璁わ級
 - 闈?0 - 鍚敤

	鍚敤杩炴帴璺熻釜娴佹椂闂存埑銆?
nf_conntrack_sctp_timeout_closed - INTEGER锛堢锛?	default 10

nf_conntrack_sctp_timeout_cookie_wait - INTEGER锛堢锛?	default 3

nf_conntrack_sctp_timeout_cookie_echoed - INTEGER锛堢锛?	default 3

nf_conntrack_sctp_timeout_established - INTEGER锛堢锛?	default 210

	榛樿鍊艰涓?(hb_interval * path_max_retrans + rto_max)

nf_conntrack_sctp_timeout_shutdown_sent - INTEGER锛堢锛?	default 3

nf_conntrack_sctp_timeout_shutdown_recd - INTEGER锛堢锛?	default 3

nf_conntrack_sctp_timeout_shutdown_ack_sent - INTEGER锛堢锛?	default 3

nf_conntrack_sctp_timeout_heartbeat_sent - INTEGER锛堢锛?	default 30

	璇ヨ秴鏃剁敤浜庡湪杈呭姪璺緞涓婂缓绔嬭繛鎺ヨ窡韪潯鐩€傞粯璁ゅ€艰涓?hb_interval銆?
nf_conntrack_udp_timeout - INTEGER锛堢锛?	default 30

nf_conntrack_udp_timeout_stream - INTEGER锛堢锛?	default 120

	鍦ㄦ娴嬪埌 UDP 娴佺殑鎯呭喌涓嬪皢浣跨敤姝ゆ墿灞曡秴鏃躲€?
nf_conntrack_gre_timeout - INTEGER锛堢锛?	default 30

nf_conntrack_gre_timeout_stream - INTEGER锛堢锛?	default 180

	鍦ㄦ娴嬪埌 GRE 娴佺殑鎯呭喌涓嬪皢浣跨敤姝ゆ墿灞曡秴鏃躲€?
nf_hooks_lwtunnel - BOOLEAN
 - 0 - 绂佺敤锛堥粯璁わ級
 - 闈?0 - 鍚敤

	濡傛灉鍚敤姝ら€夐」锛岃交閲忕骇闅ч亾锛坙ightweight tunnel锛塶etfilter 閽╁瓙琚惎鐢ㄣ€備竴鏃﹀惎鐢紝
	姝ら€夐」鏃犳硶琚鐢ㄣ€?
nf_flowtable_tcp_timeout - INTEGER锛堢锛?        default 30

        鎺у埗 TCP 杩炴帴鐨勫嵏杞借秴鏃躲€俆CP 杩炴帴鍙互浠?nf conntrack 鍗歌浇鍒?nf flow table銆?        涓€鏃﹁€佸寲锛岃繛鎺ュ皢杩斿洖鍒?nf conntrack銆?
nf_flowtable_udp_timeout - INTEGER锛堢锛?        default 30

        鎺у埗 UDP 杩炴帴鐨勫嵏杞借秴鏃躲€俇DP 杩炴帴鍙互浠?nf conntrack 鍗歌浇鍒?nf flow table銆?        涓€鏃﹁€佸寲锛岃繛鎺ュ皢杩斿洖鍒?nf conntrack銆?