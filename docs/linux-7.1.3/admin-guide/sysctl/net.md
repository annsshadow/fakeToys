## /proc/sys/net/ 鏂囨。


鐗堟潈


Copyright (c) 1999

 - Terrehon Bowden <terrehon@pacbell.net>
 - Bodo Bauer <bb@ricochet.net>

Copyright (c) 2000

 - Jorge Nerin <comandante@zaralinux.com>

Copyright (c) 2009

 - Shen Feng <shen@cn.fujitsu.com>

鏈夊叧涓€鑸俊鎭笌娉曞緥澹版槑锛岃鍙傞槄 index.rst銆?
------------------------------------------------------------------------------

鏈枃浠跺寘鍚?/proc/sys/net 涓?sysctl 鏂囦欢鐨勬枃妗ｃ€?
鍐呮牳缃戠粶閮ㄥ垎鐨勬帴鍙ｄ綅浜?/proc/sys/net銆備笅琛ㄦ樉绀轰簡鎵€鏈夊彲鑳界殑瀛愮洰褰曘€傛牴鎹唴鏍搁厤缃殑涓嶅悓锛屼綘涔熻鍙兘鐪嬪埌鍏朵腑涓€閮ㄥ垎銆?

琛細/proc/sys/net 涓殑瀛愮洰褰?
 ========= =================== = ========== ===================
 Directory Content               Directory  Content
 ========= =================== = ========== ===================
 802       E802 protocol         mptcp      Multipath TCP
 appletalk Appletalk protocol    netfilter  Network Filter
 ax25      AX25                  netrom     NET/ROM
 bridge    Bridging              rose       X.25 PLP layer
 core      General parameter     tipc       TIPC
 ethernet  Ethernet protocol     unix       Unix domain sockets
 ipv4      IP version 4          vsock      VSOCK sockets
 ipv6      IP version 6          x25        X.25 protocol
 ========= =================== = ========== ===================

## 1. /proc/sys/net/core - 缃戠粶鏍稿績閫夐」


### bpf_jit_enable


璇ュ姛鑳藉惎鐢?BPF 鍗虫椂锛圝ust in Time锛孞IT锛夌紪璇戝櫒銆侭PF 鏄竴绉嶇伒娲讳笖楂樻晥鐨勫熀纭€璁炬柦锛屽厑璁稿湪鍚勪釜閽╁瓙锛坔ook锛夌偣鎵ц瀛楄妭鐮併€傚畠琚敤浜庤嫢骞?Linux 鍐呮牳瀛愮郴缁燂紝渚嬪缃戠粶锛堝 XDP銆乼c锛夈€佽拷韪紙濡?kprobes銆乽probes銆乼racepoints锛夊拰瀹夊叏锛堝 seccomp锛夈€侺LVM 鏈変竴涓?BPF 鍚庣锛屽彲浠ュ皢鍙楅檺鐨?C 缂栬瘧涓轰竴绯诲垪 BPF 鎸囦护銆傞€氳繃 bpf(2) 鍔犺浇绋嬪簭骞剁粡鍐呮牳涓殑楠岃瘉鍣紙verifier锛夋鏌ュ悗锛孞IT 浼氬皢杩欎簺 BPF proglet 缈昏瘧涓烘湰鏈?CPU 鎸囦护銆侸IT 鏈変袱绉嶇被鍨嬶紝杈冩柊鐨?eBPF JIT 褰撳墠鍦ㄤ互涓嬫灦鏋勪笂鍙楁敮鎸侊細

  - x86_64
  - x86_32
  - arm64
  - arm32
  - ppc64
  - ppc32
  - sparc64
  - mips64
  - s390x
  - riscv64
  - riscv32
  - loongarch64
  - arc

杈冩棫鐨?cBPF JIT 鍦ㄤ互涓嬫灦鏋勪笂鍙楁敮鎸侊細

  - mips
  - sparc

eBPF JIT 鏄?cBPF JIT 鐨勮秴闆嗭紝鎰忓懗鐫€鍐呮牳浼氬皢 cBPF 鎸囦护杩佺Щ涓?eBPF 鎸囦护锛岀劧鍚庨€忔槑鍦板鍏跺仛 JIT 缂栬瘧銆傝緝鏃х殑 cBPF JIT 鍙兘缈昏瘧 tcpdump 杩囨护鍣ㄣ€乻eccomp 瑙勫垯绛夛紝鑰屼笉鑳界炕璇戜笂鏂囨彁鍒扮殑閫氳繃 bpf(2) 鍔犺浇鐨?eBPF 绋嬪簭銆?
鍙栧€硷細

 - 0 - 绂佺敤 JIT锛堥粯璁ゅ€硷級
 - 1 - 鍚敤 JIT
 - 2 - 鍚敤 JIT锛屽苟瑕佹眰缂栬瘧鍣ㄥ湪鍐呮牳鏃ュ織涓彂鍑鸿窡韪俊鎭紙trace锛夈€?
### bpf_jit_harden


璇ュ姛鑳藉 BPF JIT 缂栬瘧鍣ㄥ惎鐢ㄥ姞鍥猴紙hardening锛夈€傚彈鏀寔鐨勬槸 eBPF JIT 鍚庣銆傚惎鐢ㄥ姞鍥轰細浠ユ€ц兘涓轰唬浠凤紝浣嗗彲浠ョ紦瑙?JIT spraying 鏀诲嚮銆?
鍙栧€硷細

 - 0 - 绂佺敤 JIT 鍔犲浐锛堥粯璁ゅ€硷級
 - 1 - 浠呭鏃犵壒鏉冪敤鎴峰惎鐢?JIT 鍔犲浐
 - 2 - 瀵规墍鏈夌敤鎴峰惎鐢?JIT 鍔犲浐

鍏朵腑鈥滅壒鏉冪敤鎴封€濆湪姝や笂涓嬫枃涓寚鍦ㄥ叾鏍圭敤鎴峰懡鍚嶇┖闂达紙root user name space锛変腑鎷ユ湁 CAP_BPF 鎴?CAP_SYS_ADMIN 鐨勮繘绋嬨€?
### bpf_jit_kallsyms


褰?BPF JIT 缂栬瘧鍣ㄥ惎鐢ㄥ悗锛岀紪璇戝嚭鐨勬槧鍍忓鍐呮牳鑰岃█鏄湭鐭ュ湴鍧€锛屾剰鍛崇潃瀹冧滑鏃笉浼氬嚭鐜板湪璺熻釜淇℃伅涓紝涔熶笉浼氬嚭鐜板湪 /proc/kallsyms 涓€傛鍔熻兘瀵煎嚭杩欎簺鍦板潃锛屽彲鐢ㄤ簬璋冭瘯/杩借釜銆傚鏋滃惎鐢ㄤ簡 bpf_jit_harden锛屾鍔熻兘灏嗚绂佺敤銆?
鍙栧€硷細

 - 0 - 绂佺敤 JIT kallsyms 瀵煎嚭锛堥粯璁ゅ€硷級
 - 1 - 浠呭鐗规潈鐢ㄦ埛瀵煎嚭 JIT kallsyms

### bpf_jit_limit


璇ュ姛鑳藉 BPF JIT 缂栬瘧鍣ㄧ殑鍐呭瓨鍒嗛厤寮哄埗鎵ц涓€涓叏灞€涓婇檺锛屼互渚垮湪鍏惰瓒呰繃鍚庢嫆缁濇棤鐗规潈鐨?JIT 璇锋眰銆俠pf_jit_limit 鍖呭惈璇ュ叏灞€涓婇檺鐨勫€硷紙浠ュ瓧鑺備负鍗曚綅锛夈€?
### dev_weight


鍐呮牳鍦ㄥ崟涓?NAPI 涓柇涓兘澶熷鐞嗙殑鍖呯殑鏈€澶ф暟閲忥紝瀹冩槸涓€涓瘡 CPU 鍙橀噺銆傚浜庢敮鎸?LRO 鎴?GRO_HW 鐨勯┍鍔紝涓€涓‖浠惰仛鍚堢殑鍖呭湪姝や笂涓嬫枃涓璁′负涓€涓寘銆?
榛樿鍊硷細64

### dev_weight_rx_bias


RPS锛堝 RFS銆乤RFS锛夊鐞嗕細涓庨┍鍔ㄦ敞鍐岀殑 NAPI poll 鍑芥暟绔炰簤姣忎釜杞腑鏂懆鏈熺殑 netdev_budget銆傛鍙傛暟褰卞搷鍦?RX 杞腑鏂懆鏈熶腑锛屾墍閰嶇疆鐨?netdev_budget 閲屾湁澶氬皯姣斾緥琚敤浜庡熀浜?RPS 鐨勫寘澶勭悊銆傚畠杩涗竴姝ユ棬鍦ㄤ娇褰撳墠鐨?dev_weight 鑳藉閫傞厤缃戠粶鏍?RX/TX 渚т笉瀵圭О CPU 闇€姹傜殑鎯呭喌銆傦紙鍙傝 dev_weight_tx_bias锛夊畠鍦ㄦ瘡 CPU 鍩虹涓婄敓鏁堛€傚叾纭畾鍩轰簬 dev_weight锛屽苟鎸変箻娉曡绠楋紙dev_weight * dev_weight_rx_bias锛夈€?
榛樿鍊硷細1

### dev_weight_tx_bias


缂╂斁涓€涓?TX 杞腑鏂懆鏈熷唴鑳藉澶勭悊鐨勫寘鐨勬渶澶ф暟閲忋€傚湪姣?CPU 鍩虹涓婄敓鏁堛€傚厑璁告牴鎹笉瀵圭О鐨勭綉缁滄爤澶勭悊闇€姹傜缉鏀惧綋鍓嶇殑 dev_weight銆傛敞鎰忛伩鍏嶄娇 TX 杞腑鏂鐞嗘垚涓?CPU 娑堣€楀ぇ鎴枫€?
璁＄畻鍩轰簬 dev_weight锛坉ev_weight * dev_weight_tx_bias锛夈€?
榛樿鍊硷細1

### default_qdisc


鐢ㄤ簬缃戠粶璁惧鐨勯粯璁ゆ帓闃熻鍒欙紙queuing discipline锛夈€傝繖鍏佽鐢ㄥ彟涓€绉嶈鍒欒鐩栭粯璁ょ殑 pfifo_fast銆傜敱浜庨粯璁ゆ帓闃熻鍒欐槸鍦ㄤ笉闄勫姞棰濆鍙傛暟鐨勬儏鍐典笅鍒涘缓鐨勶紝鍥犳鏈€閫傚悎鐢ㄤ簬閭ｄ簺鏃犻渶閰嶇疆鍗冲彲鑹ソ宸ヤ綔鐨勬帓闃熻鍒欙紝渚嬪闅忔満鍏钩闃熷垪锛坰fq锛夈€丆oDel锛坈odel锛夋垨鍏钩闃熷垪 CoDel锛坒q_codel锛夈€備笉瑕佷娇鐢ㄥ儚鍒嗗眰浠ょ墝妗讹紙Hierarchical Token Bucket锛夋垨璧ゅ瓧杞锛圖eficit Round Robin锛夎繖鏍烽渶瑕佽缃被鍒拰甯﹀鐨勬帓闃熻鍒欍€傛敞鎰忥紝鐗╃悊澶氶槦鍒楁帴鍙ｄ粛鐒朵娇鐢?mq 浣滀负鏍?qdisc锛岃€?mq 鍙堜娇鐢ㄦ榛樿鍊间綔涓哄叾鍙跺瓙銆傝櫄鎷熻澶囷紙渚嬪 lo 鎴?veth锛変細蹇界暐姝よ缃紝杞€岄粯璁や娇鐢?noqueue銆?
榛樿鍊硷細pfifo_fast

### busy_read


鐢ㄤ簬 socket 璇诲彇鐨勪綆寤惰繜蹇欒疆璇紙busy poll锛夎秴鏃躲€傦紙闇€瑕?CONFIG_NET_RX_BUSY_POLL锛夊湪璁惧闃熷垪涓婂繖寰幆绛夊緟鏁版嵁鍖呯殑澶ц嚧鏃堕棿锛屼互寰涓哄崟浣嶃€傚畠璁剧疆 SO_BUSY_POLL socket 閫夐」鐨勯粯璁ゅ€笺€傚彲浠ラ€氳繃璁剧疆 socket 閫夐」 SO_BUSY_POLL 鏉ヨ缃垨瑕嗙洊姣忎釜 socket 鐨勫€硷紝杩欎篃鏄帹鑽愮殑鍚敤鏂瑰紡銆傚鏋滀綘闇€瑕侀€氳繃 sysctl 鍏ㄥ眬鍚敤璇ュ姛鑳斤紝寤鸿浣跨敤鍊?50銆?
浼氬鍔犲姛鑰椼€?
榛樿鍊硷細0锛堝叧闂級

### busy_poll


鐢ㄤ簬 poll 鍜?select 鐨勪綆寤惰繜蹇欒疆璇㈣秴鏃躲€傦紙闇€瑕?CONFIG_NET_RX_BUSY_POLL锛夊繖寰幆绛夊緟浜嬩欢鐨勫ぇ鑷存椂闂达紝浠ュ井绉掍负鍗曚綅銆傛帹鑽愬€煎彇鍐充簬浣犺疆璇㈢殑 socket 鏁伴噺銆傚浜庤嫢骞?socket 鐢?50锛屽浜庢暟鐧句釜鐢?100銆傚啀澶氱殑璇濅綘鍙兘鎯充娇鐢?epoll銆傛敞鎰忓彧鏈夎缃簡 SO_BUSY_POLL 鐨?socket 鎵嶄細琚繖杞锛屽洜姝や綘瑕佷箞鍦ㄨ繖浜?socket 涓婃湁閫夋嫨鍦拌缃?SO_BUSY_POLL锛岃涔堝叏灞€璁剧疆 sysctl.net.busy_read銆?
浼氬鍔犲姛鑰椼€?
榛樿鍊硷細0锛堝叧闂級

### mem_pcpu_rsv


姣?CPU 棰勭暀鐨勮浆鍙戝垎閰嶏紙forward alloc锛夌紦瀛樺ぇ灏忥紝浠ラ〉涓哄崟浣嶃€傞粯璁ゆ瘡 CPU 1MB銆?
### bypass_prot_mem


璺宠繃灏?socket 缂撳啿鍖鸿鍏ョ敱 net.ipv4.tcp_mem銆乶et.ipv4.udp_mem 绛夋帶鍒剁殑鍏ㄥ眬姣忓崗璁唴瀛樼粺璁°€?
榛樿鍊硷細0锛堝叧闂級

### rmem_default


socket 鎺ユ敹缂撳啿鍖虹殑榛樿璁剧疆锛堜互瀛楄妭涓哄崟浣嶏級銆?
### rmem_max


鎺ユ敹 socket 缂撳啿鍖虹殑鏈€澶уぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?
榛樿鍊硷細4194304

### rps_default_mask


鍦ㄦ柊鍒涘缓鐨勭綉缁滆澶囦笂浣跨敤鐨勯粯璁?RPS CPU 鎺╃爜銆傜┖鎺╃爜琛ㄧず榛樿绂佺敤 RPS銆?
### tstamp_allow_data


鍏佽杩涚▼鎺ユ敹涓庡師濮嬪寘鍐呭涓€璧峰洖鐜紙loop锛夌殑鍙戦€佹椂闂存埑銆傚鏋滅鐢紝鏉ヨ嚜鏃犵壒鏉冭繘绋嬬殑鍙戦€佹椂闂存埑璇锋眰浼氳涓㈠純锛岄櫎闈炶缃簡 socket 閫夐」 SOF_TIMESTAMPING_OPT_TSONLY銆?
榛樿鍊硷細1锛堝紑鍚級


### wmem_default


socket 鍙戦€佺紦鍐插尯鐨勯粯璁よ缃紙浠ュ瓧鑺備负鍗曚綅锛夈€?
### wmem_max


鍙戦€?socket 缂撳啿鍖虹殑鏈€澶уぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?
榛樿鍊硷細4194304

### message_burst 涓?message_cost


杩欎簺鍙傛暟鐢ㄤ簬闄愬埗浠庣綉缁滀唬鐮佸啓鍏ュ唴鏍告棩蹇楃殑璀﹀憡娑堟伅銆傚畠浠己鍒跺疄鏂戒竴涓€熺巼闄愬埗锛屼娇寰楁嫆缁濇湇鍔★紙denial-of-service锛夋敾鍑绘棤娉曞緱閫炪€傝緝澶х殑 message_cost 鍥犲瓙浼氬鑷村啓鍏ョ殑娑堟伅鏇村皯銆俶essage_burst 鎺у埗娑堟伅浣曟椂琚涪寮冦€傞粯璁よ缃皢璀﹀憡娑堟伅闄愬埗涓烘瘡浜旂涓€鏉°€?
### warnings


姝?sysctl 鐜板凡涓嶅啀浣跨敤銆?
瀹冩浘鐢ㄤ簬鎺у埗鏉ヨ嚜缃戠粶鏍堢殑鎺у埗鍙版秷鎭紝杩欎簺娑堟伅鍥犵綉缁滈棶棰橈紙濡傞噸澶嶅湴鍧€鎴栭敊璇牎楠屽拰锛夎€屼骇鐢熴€?
杩欎簺娑堟伅鐜板湪浠?KERN_DEBUG 绾у埆鍙戝嚭锛岄€氬父鍙互閫氳繃 dynamic_debug 璁炬柦鍚敤鍜屾帶鍒躲€?
### netdev_budget


鍦ㄤ竴涓疆璇㈠懆鏈燂紙NAPI poll锛変腑浠庢墍鏈夋帴鍙ｅ彇璧扮殑鍖呯殑鏈€澶ф暟閲忋€傚湪涓€涓疆璇㈠懆鏈熶腑锛屾敞鍐屽埌杞鐨勬帴鍙ｄ互杞锛坮ound-robin锛夋柟寮忚鎺㈡祴銆傛澶栵紝鍗充究 netdev_budget 灏氭湭鑰楀敖锛屼竴涓疆璇㈠懆鏈熶篃涓嶅緱瓒呰繃 netdev_budget_usecs 寰銆?
### netdev_budget_usecs


涓€涓?NAPI 杞鍛ㄦ湡涓殑鏈€澶у井绉掓暟銆傚綋杞鍛ㄦ湡涓?netdev_budget_usecs 宸叉祦閫濓紝鎴栧凡澶勭悊鐨勫寘鏁伴噺杈惧埌 netdev_budget 鏃讹紝杞灏嗛€€鍑恒€?
### netdev_max_backlog


褰撴帴鍙ｆ帴鏀舵暟鎹寘鐨勯€熷害蹇簬鍐呮牳澶勭悊瀹冧滑鐨勯€熷害鏃讹紝鍦ㄨ緭鍏ヤ晶鎺掗槦鐨勫寘鐨勬渶澶ф暟閲忋€?
### qdisc_max_burst


鍦ㄥ埌杈?qdisc 涔嬪墠鍙互涓存椂瀛樺偍鐨勫寘鐨勬渶澶ф暟閲忋€?
榛樿鍊硷細1000

### netdev_rss_key


鍚敤浜?RSS锛圧eceive Side Scaling锛夌殑椹卞姩浣跨敤涓€涓殢鏈虹敓鎴愮殑涓绘満瀵嗛挜锛坔ost key锛夈€?
鏌愪簺鐢ㄦ埛绌洪棿鍙兘闇€瑕佸湪椹卞姩灏氭湭鎻愪緵 ethtool -x 鏀寔鐨勬儏鍐典笅鑾峰彇鍏跺唴瀹广€?
```

  myhost:~# cat /proc/sys/net/core/netdev_rss_key
  84:50:f4:00:a8:15:d1:a7:e9:7f:1d:60:35:c7:47:25:42:97:74:ca:56:bb:b6:a1:d8: ... (256 bytes total)

```
濡傛灉浠庢潵娌℃湁椹卞姩璋冪敤杩?netdev_rss_key_fill() 鍑芥暟锛屾枃浠跺寘鍚叏閮ㄤ负 nul 鐨勫瓧鑺傘€?
娉ㄦ剰锛?  /proc/sys/net/core/netdev_rss_key 鍖呭惈 256 瀛楄妭鐨勫瘑閽ワ紝
  浣嗚澶氶┍鍔ㄥ彧浣跨敤鍏朵腑鐨?40 鎴?52 瀛楄妭銆?
```

  myhost:~# ethtool -x eth0
  RX flow hash indirection table for eth0 with 8 RX ring(s):
      0:    0     1     2     3     4     5     6     7
  RSS hash key:
  84:50:f4:00:a8:15:d1:a7:e9:7f:1d:60:35:c7:47:25:42:97:74:ca:56:bb:b6:a1:d8:43:e3:c9:0c:fd:17:55:c2:3a:4d:69:ed:f1:42:89

```
### netdev_tstamp_prequeue


濡傛灉璁句负 0锛孯X 鍖呮椂闂存埑鍙互鍦?RPS 澶勭悊涔嬪悗銆佺敱鐩爣 CPU 澶勭悊鍖呮椂閲囨牱銆傝繖鍙兘鍦ㄦ椂闂存埑涓婂紩鍏ヤ竴浜涘欢杩燂紝浣嗗厑璁稿皢璐熻浇鍒嗗竷鍒板涓?CPU 涓娿€?
濡傛灉璁句负 1锛堥粯璁ゅ€硷級锛屾椂闂存埑浼氬湪鎺掗槦涔嬪墠灏藉揩琚噰鏍枫€?
### netdev_unregister_timeout_secs


娉ㄩ攢缃戠粶璁惧瓒呮椂鏃堕棿锛堜互绉掍负鍗曚綅锛夈€傛閫夐」鎺у埗鍦ㄨ澶囨敞閿€鏈熼棿绛夊緟缃戠粶璁惧寮曠敤璁℃暟闄嶄负 0 鏃讹紝鍙戝嚭璀﹀憡鎵€鐢ㄧ殑瓒呮椂锛堢锛夈€傝緝灏忕殑鍊煎湪浜屽垎鏌ラ敊锛坆isection锛夋椂鍙兘鏈夊姪浜庢洿蹇湴妫€娴嬪埌娉勬紡鐨勫紩鐢ㄣ€傝緝澶х殑鍊煎彲鑳芥湁鍔╀簬鍦ㄧ紦鎱?楂樿礋杞界郴缁熶笂閬垮厤璇姤璀﹀憡銆傞粯璁ゅ€间负 10锛屾渶灏忓€间负 1锛屾渶澶у€间负 3600銆?
### skb_defer_max


鐢卞垎閰嶅畠浠殑 CPU 閲婃斁鐨勩€佹瘡 CPU 鐨?skb 鍒楄〃鐨勬渶澶уぇ灏忥紙浠?skb 璁★級銆?
榛樿鍊硷細128

### optmem_max


姣忎釜 socket 鍏佽鐨勮緟鍔╃紦鍐诧紙ancillary buffer锛夋渶澶уぇ灏忋€傝緟鍔╂暟鎹槸涓€绯诲垪甯﹂檮鍔犳暟鎹殑 struct cmsghdr 缁撴瀯銆俆CP 鍙戦€侀浂鎷疯礉锛坱x zerocopy锛変篃浣跨敤 optmem_max 浣滀负鍏跺唴閮ㄧ粨鏋勭殑涓婇檺銆?
榛樿鍊硷細128 KB

### fb_tunnels_only_for_init_net


鎺у埗鏄惁鑷姩鍒涘缓鍥為€€闅ч亾锛堝 tunl0銆乬re0銆乬retap0銆乪rspan0銆乻it0銆乮p6tnl0銆乮p6gre0锛夈€傛湁 3 绉嶅彲鑳斤細

(a) 鍊?= 0锛涘湪鍚勪釜缃戠粶鍛藉悕绌洪棿涓姞杞芥ā鍧楁椂鍒涘缓鐩稿簲鐨勫洖閫€闅ч亾锛堝悜鍚庡吋瀹硅涓猴級銆?(b) 鍊?= 1锛沎kcmd 鍊硷細initns] 鐩稿簲鐨勫洖閫€闅ч亾浠呭湪 init 缃戠粶鍛藉悕绌洪棿涓垱寤猴紝鍏朵粬鎵€鏈夌綉缁滃懡鍚嶇┖闂撮兘涓嶄細鎷ユ湁瀹冧滑銆?(c) 鍊?= 2锛沎kcmd 鍊硷細none] 鍦ㄤ换鎰忕綉缁滃懡鍚嶇┖闂翠腑鍔犺浇妯″潡鏃堕兘涓嶄細鍒涘缓鍥為€€闅ч亾銆傚鏋滆繖浜涙ā鍧楁槸鍐呭缓鐨勶紝鍚姩鍚庡皢鍊艰涓衡€?鈥濇病鏈夋剰涔夛紝鍥犳鏈変竴涓唴鏍稿懡浠よ閫夐」鍙互鏇存敼姝ら粯璁ゅ€笺€傛洿澶氱粏鑺傝鍙傞槄 Documentation/admin-guide/kernel-parameters.txt銆?
涓嶅垱寤哄洖閫€闅ч亾锛岃鐢ㄦ埛绌洪棿鑳藉鍙垱寤烘墍闇€鍐呭锛屽苟閬垮厤鍒涘缓鍐椾綑鐨勮澶囥€?
榛樿鍊硷細0锛堝嚭浜庡吋瀹规€у師鍥狅級

### devconf_inherit_init_net


鎺у埗涓€涓柊鐨勭綉缁滃懡鍚嶇┖闂存槸鍚﹀簲缁ф壙 /proc/sys/net/{ipv4,ipv6}/conf/{all,default}/ 涓嬬殑鎵€鏈夊綋鍓嶈缃€傞粯璁ゆ儏鍐典笅锛屾垜浠繚鎸佸綋鍓嶈涓猴細瀵逛簬 IPv4锛屾垜浠粠 init_net 缁ф壙鎵€鏈夊綋鍓嶈缃紱瀵逛簬 IPv6锛屾垜浠皢鎵€鏈夎缃噸缃负榛樿鍊笺€?
濡傛灉璁句负 1锛孖Pv4 鍜?IPv6 璁剧疆閮借寮哄埗浠?init_net 涓殑褰撳墠璁剧疆缁ф壙銆傚鏋滆涓?2锛孖Pv4 鍜?IPv6 璁剧疆閮借寮哄埗閲嶇疆涓哄悇鑷殑榛樿鍊笺€傚鏋滆涓?3锛孖Pv4 鍜?IPv6 璁剧疆閮借寮哄埗浠庡垱寤烘鏂?netns 鐨勯偅涓?netns 涓殑褰撳墠璁剧疆缁ф壙銆?
榛樿鍊硷細0锛堝嚭浜庡吋瀹规€у師鍥狅級

### txrehash


鎺у埗褰?SO_TXREHASH 閫夐」琚涓?SOCK_TXREHASH_DEFAULT锛堝嵆鏈 setsockopt 瑕嗙洊锛夋椂锛宻ocket 涓婄殑榛樿鍝堝笇閲嶇畻锛坔ash rethink锛夎涓恒€?
濡傛灉璁句负 1锛堥粯璁ゅ€硷級锛屼細鍦ㄧ洃鍚?socket 涓婃墽琛屽搱甯岄噸绠椼€傚鏋滆涓?0锛屽垯涓嶆墽琛屽搱甯岄噸绠椼€?
### txq_reselection_ms


鎺у埗涓€涓箒蹇欑殑宸茶繛鎺ユ祦鍙互澶氶绻佸湴锛堜互姣涓哄崟浣嶏級閫夋嫨鍙︿竴涓?tx 闃熷垪銆?
褰撶敤鎴风嚎绋嬪凡杩佺Щ涓?XPS 浼氶€夋嫨涓嶅悓闃熷垪鏃讹紝閲嶆柊閫夋嫨鏄彲鍙栫殑銆傚嵆浣挎病鏈?XPS锛屽鏋滄祦鍝堝笇鍙戠敓浜嗗彉鍖栵紝涔熷彲鑳藉彂鐢熷悓鏍风殑鎯呭喌銆?
浣嗗垏鎹?txq 鍙兘寮曞叆涔卞簭锛坮eorder锛夛紝灏ゅ叾鏄湪鏃ч槦鍒楀浜庨珮鍘嬪姏涓嬫椂銆傜幇浠?TCP 鏍堣嫢涔卞簭鍙戠敓寰椾笉棰戠箒锛岃兘澶熷緢濂藉湴搴斿銆?
瑕佺鐢ㄦ鍔熻兘锛岃灏嗗€艰涓?0銆?
榛樿鍊硷細1000

### gro_normal_batch


GRO 杈撳嚭鏃舵壒閲忓悎骞剁殑鏈€澶ф鏁般€傚綋涓€涓寘閫€鍑?GRO 鏃讹紙鏃犺鏄綔涓哄悎骞跺悗鐨勮秴绾у抚锛坰uperframe锛夛紝杩樻槸浣滀负 GRO 鍐冲畾涓嶅悎骞剁殑鍘熷鍖咃級锛屽畠浼氳鏀惧叆涓€涓瘡 NAPI 鐨勫垪琛ㄤ腑銆傚綋娈电殑鏁伴噺杈惧埌 gro_normal_batch 涓婇檺鏃讹紝璇ュ垪琛ㄤ細琚紶閫掔粰缃戠粶鏍堛€?
### high_order_alloc_disable


榛樿鎯呭喌涓嬶紝椤电鐗囷紙page frag锛夊垎閰嶅櫒灏濊瘯浣跨敤楂橀樁椤碉紙鍦?x86 涓婁负 order-3锛夈€傝櫧鐒堕粯璁よ涓哄湪澶у鏁版儏鍐典笅鏁堟灉鑹ソ锛屼絾鏌愪簺鐢ㄦ埛鍙兘閬囧埌椤靛垎閰?閲婃斁涓殑浜夌敤銆傚湪杈冩棫鐨勫唴鏍革紙< 5.14锛変笂褰撻珮闃堕〉鏈瓨鍌ㄥ湪姣?CPU 鍒楄〃涓婃椂锛岃繖涓€鐐瑰挨涓烘槑鏄俱€傝繖鍏佽閫夋嫨鏀圭敤 order-0 鍒嗛厤锛屼絾鐜板湪涓昏鍏锋湁鍘嗗彶鎰忎箟銆?
榛樿鍊硷細0

### 2. /proc/sys/net/unix - Unix 鍩熷鎺ュ瓧鍙傛暟


姝ょ洰褰曚腑鍙湁涓€涓枃浠躲€倁nix_dgram_qlen 闄愬埗 Unix 鍩?socket 缂撳啿鍖轰腑鎺掗槦鐨?datagram 鐨勬渶澶ф暟閲忋€傞櫎闈炴寚瀹氫簡 PF_UNIX 鏍囧織锛屽惁鍒欏畠涓嶄細鐢熸晥銆?

### 3. /proc/sys/net/ipv4 - IPV4 璁剧疆


璇峰弬闃咃細Documentation/networking/ip-sysctl.rst 涓?Documentation/admin-guide/sysctl/net.rst锛屼簡瑙ｈ繖浜涙潯鐩殑璇存槑銆?

### 4. Appletalk


/proc/sys/net/appletalk 鐩綍鍦?Appletalk 鍔犺浇鏃朵繚瀛樺叾閰嶇疆鏁版嵁銆傚彲閰嶇疆鐨勫弬鏁版湁锛?
### aarp-expiry-time


鍦ㄥ皢涓€涓?ARP 鏉＄洰杩囨湡涔嬪墠鎴戜滑淇濈暀瀹冪殑鏃堕棿銆傜敤浜庢窐姹帮紙age out锛夋棫鐨勪富鏈恒€?
### aarp-resolve-time


鎴戜滑灏嗗皾璇曡В鏋愪竴涓?Appletalk 鍦板潃鎵€鑺辫垂鐨勬椂闂淬€?
### aarp-retransmit-limit


鍦ㄦ斁寮冧箣鍓嶆垜浠皢閲嶄紶涓€娆℃煡璇㈢殑娆℃暟銆?
### aarp-tick-time


鎺у埗妫€鏌ヨ繃鏈燂紙expire锛夌殑閫熺巼銆?
鐩綍 /proc/net/appletalk 淇濆瓨鏈哄櫒涓婃椿璺?Appletalk socket 鐨勫垪琛ㄣ€?
杩欎簺瀛楁鎸囩ず DDP 绫诲瀷銆佹湰鍦板湴鍧€锛坣etwork:node 鏍煎紡锛夈€佽繙绔湴鍧€銆佸彂閫佹寕璧烽槦鍒楃殑澶у皬銆佹帴鏀堕槦鍒楃殑澶у皬锛堢瓑寰呭簲鐢ㄧ▼搴忚鍙栫殑瀛楄妭鏁帮級銆佺姸鎬佷互鍙婃嫢鏈夎 socket 鐨?uid銆?
/proc/net/atalk_iface 鍒楀嚭鎵€鏈変负 appletalk 閰嶇疆鐨勬帴鍙ｃ€傚畠鏄剧ず鎺ュ彛鍚嶇О銆佸叾 Appletalk 鍦板潃銆佽鍦板潃涓婄殑缃戠粶鑼冨洿锛堟垨 phase 1 缃戠粶涓殑缃戠粶鍙凤級锛屼互鍙婃帴鍙ｇ殑鐘舵€併€?
/proc/net/atalk_route 鍒楀嚭姣忎釜宸茬煡鐨勭綉缁滆矾鐢便€傚畠鍒楀嚭璺敱鎵€鎸囧悜鐨勭洰鏍囷紙缃戠粶锛夈€佽矾鐢卞櫒锛堝彲鑳界洿鎺ョ浉杩烇級銆佽矾鐢辨爣蹇楋紝浠ュ強璇ヨ矾鐢辨墍浣跨敤鐨勮澶囥€?
### 5. TIPC


### tipc_rmem


TIPC 鍗忚鐜板湪鏈変竴涓拡瀵规帴鏀跺唴瀛樼殑鍙皟鍙傛暟锛岀被浼间簬 tcp_rmem鈥斺€斿嵆涓€涓寘鍚?3 涓?INTEGER 鐨勫悜閲忥細(min, default, max)

```

    # cat /proc/sys/net/tipc/tipc_rmem
    4252725 34021800        68043600
    #

```
max 鍊艰璁句负 CONN_OVERLOAD_LIMIT锛岃€?default 鍜?min 鍊兼槸璇ュ悓涓€鍊肩殑缂╂斁锛堢Щ浣嶏級鐗堟湰銆傛敞鎰?min 鍊肩洰鍓嶅湪鏈夋剰涔夌殑灞傞潰骞舵湭琚娇鐢紝浣嗕繚鐣欒繖涓笁鍏冪粍鏄负浜嗕笌 tcp_rmem 绛変繚鎸佷竴鑷淬€?
### named_timeout


TIPC 鍚嶇О琛ㄦ洿鏂板湪闆嗙兢涓槸寮傛鍒嗗彂鐨勶紝娌℃湁浠讳綍褰㈠紡鐨勪簨鍔″鐞嗐€傝繖鎰忓懗鐫€鍙兘鍑虹幇涓嶅悓鐨勭珵鎬佸満鏅€傚叾涓竴绉嶆儏鍐垫槸锛屼竴涓妭鐐瑰彂鍑虹殑鍚嶇О鎾ら攢锛坣ame withdrawal锛夎鍙︿竴涓妭鐐规帴鏀舵椂锛屽彲鑳芥櫄浜庡凡缁忎粠涓€涓涓変釜鑺傜偣鎺ュ彈鐨勩€佷笌涔嬪墠閲嶅彔鐨勫悕绉板彂甯冿紙name publication锛夛紝灏界杩欎簺鍐茬獊鐨勬洿鏂版渶鍒濆彲鑳芥槸鎸夋纭殑椤哄簭鍙戝嚭鐨勩€傚鏋?named_timeout 闈為浂锛屽け璐ユ嫇鎵戞洿鏂颁細琚斁鍏ヤ竴涓欢杩熼槦鍒楋紝鐩村埌鍙︿竴涓竻闄よ閿欒鐨勪簨浠跺埌杈撅紝鎴栬€呯洿鍒拌秴鏃跺埌鏈熴€傚€间互姣涓哄崟浣嶃€?
### 6. /proc/sys/net/vsock - VSOCK 濂楁帴瀛?

VSOCK 濂楁帴瀛楋紙AF_VSOCK锛夋彁渚涜櫄鎷熸満涓庡叾瀹夸富鏈轰箣闂寸殑閫氫俊銆俈SOCK 濂楁帴瀛楀湪缃戠粶鍛藉悕绌洪棿涓殑琛屼负鐢辫鍛藉悕绌洪棿鐨勬ā寮忥紙`global` 鎴?`local`锛夊喅瀹氾紝璇ユā寮忔帶鍒?CID锛圕ontext ID锛夊浣曞垎閰嶏紝浠ュ強 socket 濡備綍璺ㄥ懡鍚嶇┖闂翠氦浜掋€?
### ns_mode


鍙銆傛姤鍛婂綋鍓嶅懡鍚嶇┖闂寸殑妯″紡锛屽湪鍛藉悕绌洪棿鍒涘缓鏃惰瀹氾紝姝ゅ悗涓嶅彲鍙樸€?
鍙栧€硷細

 - `global` - 璇ュ懡鍚嶇┖闂村叡浜郴缁熻寖鍥寸殑 CID 鍒嗛厤锛屽叾 socket 鍙互鍒拌揪浠绘剰鍏ㄥ眬鍛藉悕绌洪棿涓殑浠绘剰 VM 鎴?socket銆傛鍛藉悕绌洪棿涓殑 socket 鏃犳硶鍒拌揪 local 鍛藉悕绌洪棿涓殑 socket銆? - `local` - 璇ュ懡鍚嶇┖闂存嫢鏈夌鏈夌殑 CID 鍒嗛厤锛屽叾 socket 鍙兘杩炴帴鍒板悓涓€鍛藉悕绌洪棿鍐呯殑 VM 鎴?socket銆?
init_net 鐨勬ā寮忓缁堜负 `global`銆?
### child_ns_mode


鎺у埗鏂板垱寤虹殑瀛愬懡鍚嶇┖闂村皢缁ф壙浣曠妯″紡銆傚湪鍛藉悕绌洪棿鍒涘缓鏃讹紝`ns_mode` 浠庣埗鍛藉悕绌洪棿鐨?`child_ns_mode` 缁ф壙銆傚垵濮嬪€间笌璇ュ懡鍚嶇┖闂磋嚜韬殑 `ns_mode` 鐩稿尮閰嶃€?
鍙栧€硷細

 - `global` - 瀛愬懡鍚嶇┖闂村皢鍏变韩绯荤粺鑼冨洿鐨?CID 鍒嗛厤锛屽叾 socket 灏嗚兘澶熷埌杈句换鎰忓叏灞€鍛藉悕绌洪棿涓殑浠绘剰 VM 鎴?socket銆? - `local` - 瀛愬懡鍚嶇┖闂村皢鎷ユ湁绉佹湁鐨?CID 鍒嗛厤锛屽叾 socket 灏嗗彧鑳藉湪鍏惰嚜韬懡鍚嶇┖闂村唴杩炴帴銆?
瀵?`child_ns_mode` 鐨勭涓€娆″啓鍏ヤ細閿佸畾鍏跺€笺€傚悗缁啓鍏ョ浉鍚岀殑鍊间細鎴愬姛锛屼絾鍐欏叆涓嶅悓鐨勫€间細杩斿洖 `-EBUSY`銆?
鏇存敼 `child_ns_mode` 鍙奖鍝嶆洿鏀逛箣鍚庡垱寤虹殑鍛藉悕绌洪棿锛涘畠涓嶄細淇敼褰撳墠鍛藉悕绌洪棿鎴栦换浣曞凡鏈夌殑瀛愬懡鍚嶇┖闂淬€?
`ns_mode` 璁句负 `local` 鐨勫懡鍚嶇┖闂存棤娉曞皢 `child_ns_mode` 鏀逛负 `global`锛堣繑鍥?`-EPERM`锛夈€?
### g2h_fallback


鎺у埗鍒颁笉琚涓绘満鍒板鎴锋満锛圚2G锛変紶杈撴墍鎷ユ湁鐨?CID 鐨勮繛鎺ワ紝鏄惁鑷姩鍥為€€锛坒all back锛夊埌瀹㈡埛鏈哄埌瀹夸富鏈猴紙G2H锛変紶杈撱€?
鍚敤鏃讹紝濡傛灉涓€娆?connect 鐨勭洰鏍囨槸涓€涓?H2G 浼犺緭锛堜緥濡?vhost-vsock锛変笉鏈嶅姟鐨?CID锛屾垨鑰呮牴鏈病鏈夊姞杞戒换浣?H2G 浼犺緭锛屽垯璇ヨ繛鎺ヤ細閫氳繃 G2H 浼犺緭锛堜緥濡?virtio-vsock锛夎矾鐢便€傝繖浣垮緱鍚屾椂杩愯宓屽 VM锛堥€氳繃 vhost-vsock锛変互鍙婂彲閫氳繃绠＄悊绋嬪簭锛堜緥濡?Nitro Enclaves锛夊埌杈剧殑鍏勫紵 VM 鐨勪富鏈猴紝鑳藉浣跨敤鍗曚竴 CID 绌洪棿瀵诲潃浜岃€咃紝鑰屾棤闇€搴旂敤绋嬪簭璁剧疆 `VMADDR_FLAG_TO_HOST`銆?
褰撳彂鐢熷洖閫€鏃讹紝浼氬湪杩滅鍦板潃涓婅嚜鍔ㄨ缃?`VMADDR_FLAG_TO_HOST`锛屼互渚跨敤鎴风┖闂村彲浠ラ€氳繃 `getpeername()` 纭畾璺緞銆?
娉ㄦ剰锛氬惎鐢ㄦ sysctl 鍚庯紝璇曞浘涓?H2G 浼犺緭鏈疄鐜扮殑瀹㈡埛鏈?CID 閫氫俊鐨勭敤鎴风┖闂翠細浜х敓瀹夸富鏈?vsock 娴侀噺銆備緷璧栦粎 H2G 闅旂鐨勭幆澧冨簲灏嗗叾璁句负 0銆?
鍙栧€硷細

 - 0 - 鍒?CID <= 2 鎴栧甫鏈?VMADDR_FLAG_TO_HOST 鐨勮繛鎺ヤ娇鐢?G2H锛涙墍鏈夊叾浠栬繛鎺ヤ娇鐢?H2G锛堝鏋?H2G 鏈姞杞斤紝鍒欏洜 ENODEV 澶辫触锛夈€? - 1 - 鍒?H2G 涓嶆嫢鏈夌殑 CID 鐨勮繛鎺ュ洖閫€鍒?G2H銆傦紙榛樿鍊硷級
