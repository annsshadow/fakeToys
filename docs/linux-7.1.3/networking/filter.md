

## Linux 濂楁帴瀛楄繃婊ゅ嵆浼厠鍒╂暟鎹寘杩囨护鍣紙BPF锛?

### 娉ㄦ剰


鏈枃浠舵浘缁忚褰曚簡 eBPF 鐨勬牸寮忎笌鏈哄埗锛屽嵆浣胯繖浜涘唴瀹逛笌濂楁帴瀛楄繃婊ゆ棤鍏炽€傚叧浜?eBPF 鐨勬洿澶氱粏鑺傝鍙傞槄 ../bpf/index.rst銆?
### 绠€浠?

Linux 濂楁帴瀛楄繃婊わ紙LSF锛夋淳鐢熻嚜 Berkeley Packet Filter銆傚敖绠?BSD 涓?Linux
鍐呮牳杩囨护涔嬮棿瀛樺湪涓€浜涙槑鏄惧樊寮傦紝浣嗗綋鎴戜滑鍦?Linux 璇涓皥鍙?BPF 鎴?LSF 鏃讹紝
鎸囩殑鏄?Linux 鍐呮牳涓畬鍏ㄧ浉鍚岀殑杩囨护鏈哄埗銆?
BPF 鍏佽鐢ㄦ埛绌洪棿绋嬪簭灏嗚繃婊ゅ櫒闄勫姞鍒颁换浣曞鎺ュ瓧涓婏紝骞跺厑璁告垨绂佹鏌愪簺绫诲瀷鐨?鏁版嵁閫氳繃璇ュ鎺ュ瓧銆侺SF 閬靛惊涓?BSD 鐨?BPF 瀹屽叏鐩稿悓鐨勮繃婊ゅ櫒浠ｇ爜缁撴瀯锛屽洜姝?鍙傝€?BSD 鐨?bpf.4 鎵嬪唽椤靛鍒涘缓杩囨护鍣ㄩ潪甯告湁甯姪銆?
鍦?Linux 涓婏紝BPF 姣斿湪 BSD 涓婄畝鍗曞緱澶氥€備綘涓嶅繀鎷呭績璁惧涔嬬被鐨勪簨鎯呫€備綘鍙渶
鍒涘缓浣犵殑杩囨护鍣ㄤ唬鐮侊紝閫氳繃 SO_ATTACH_FILTER 閫夐」灏嗗叾鍙戦€佸埌鍐呮牳锛屽鏋滀綘鐨?杩囨护鍣ㄤ唬鐮侀€氳繃浜嗗唴鏍哥殑妫€鏌ワ紝浣犲氨鍙互绔嬪嵆寮€濮嬪湪璇ュ鎺ュ瓧涓婅繃婊ゆ暟鎹€?
浣犱篃鍙互閫氳繃 SO_DETACH_FILTER 閫夐」浠庡鎺ュ瓧涓婂垎绂昏繃婊ゅ櫒銆傝繖鍙兘涓嶅お甯哥敤锛?鍥犱负褰撲綘鍏抽棴甯︽湁杩囨护鍣ㄧ殑濂楁帴瀛楁椂锛岃繃婊ゅ櫒浼氳鑷姩绉婚櫎銆傚彟涓€绉嶄笉澶父瑙佺殑
鎯呭喌鍙兘鏄紝鍦ㄥ凡缁忚繍琛岀潃鍙︿竴涓繃婊ゅ櫒鐨勫悓涓€濂楁帴瀛椾笂娣诲姞涓嶅悓鐨勮繃婊ゅ櫒锛氬唴鏍?璐熻矗绉婚櫎鏃х殑骞舵斁缃綘鐨勬柊杩囨护鍣紝鍓嶆彁鏄綘鐨勮繃婊ゅ櫒閫氳繃浜嗘鏌ワ紝鍚﹀垯濡傛灉澶辫触锛?鏃х殑杩囨护鍣ㄥ皢淇濈暀鍦ㄨ濂楁帴瀛椾笂銆?
SO_LOCK_FILTER 閫夐」鍏佽閿佸畾闄勫姞鍒板鎺ュ瓧鐨勮繃婊ゅ櫒銆備竴鏃﹁缃紝杩囨护鍣ㄥ氨鏃犳硶
琚Щ闄ゆ垨鏇存敼銆傝繖鍏佽涓€涓繘绋嬪缓绔嬪鎺ュ瓧銆侀檮鍔犺繃婊ゅ櫒銆侀攣瀹氬畠锛岀劧鍚庢斁寮冪壒鏉冿紝
骞剁‘淇¤杩囨护鍣ㄤ細涓€鐩翠繚鐣欏埌濂楁帴瀛楀叧闂€?
姝ゆ瀯閫犳渶澶х殑浣跨敤鑰呭彲鑳藉氨鏄?libpcap銆傚彂鍑哄儚 `tcpdump -i em1 port 22` 杩欐牱
鐨勯珮绾ц繃婊ゅ櫒鍛戒护锛屼細缁忚繃 libpcap 鍐呴儴缂栬瘧鍣紝鐢熸垚涓€涓渶缁堝彲浠ラ€氳繃
SO_ATTACH_FILTER 鍔犺浇鍒板唴鏍哥殑缁撴瀯銆俙tcpdump -i em1 port 22 -ddd` 浼氭樉绀?姝ｈ鏀惧叆璇ョ粨鏋勭殑鍐呭銆?
灏界鎴戜滑杩欓噷鍙皥璁哄鎺ュ瓧锛屼絾 Linux 涓殑 BPF 杩樿鐢ㄤ簬鏇村鍦版柟銆俷etfilter 鏈?xt_bpf锛屽唴鏍?qdisc 灞傛湁 cls_bpf锛岃繕鏈?SECCOMP-BPF锛圫ECure COMPuting
[^1^]_锛夛紝浠ュ強璁稿鍏朵粬鍦版柟锛屼緥濡?team 椹卞姩銆丳TP 浠ｇ爜绛夐兘鍦ㄤ娇鐢?BPF銆?

Original BPF paper:

Steven McCanne and Van Jacobson. 1993. The BSD packet filter: a new
architecture for user-level packet capture. In Proceedings of the
USENIX Winter 1993 Conference Proceedings on USENIX Winter 1993
Conference Proceedings (USENIX'93). USENIX Association, Berkeley,
CA, USA, 2-2. [http://www.tcpdump.org/papers/bpf-usenix93.pdf]

### 缁撴瀯


鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍖呭惈 <linux/filter.h>锛屽叾涓惈鏈?```

	struct sock_filter {	/* Filter block */
		__u16	code;   /* Actual filter code */
		__u8	jt;	/* Jump true */
		__u8	jf;	/* Jump false */
		__u32	k;      /* Generic multiuse field */
	};

```
杩欐牱鐨勭粨鏋勪綋琚粍瑁呮垚涓€涓敱 4 鍏冪粍缁勬垚鐨勬暟缁勶紝鍖呭惈 code銆乯t銆乯f 鍜?k 鍊笺€?jt 鍜?jf 鏄烦杞亸绉伙紝k 鏄竴涓€氱敤鐨?```

	struct sock_fprog {			/* Required for SO_ATTACH_FILTER. */
		unsigned short		   len;	/* Number of filter blocks */
		struct sock_filter __user *filter;
	};

```
瀵逛簬濂楁帴瀛楄繃婊わ紝鎸囧悜璇ョ粨鏋勪綋鐨勬寚閽堬紙濡傚悗缁ず渚嬫墍绀猴級閫氳繃 setsockopt(2)
浼犻€掔粰鍐呮牳銆?
### 绀轰緥


```

    #include <sys/socket.h>
    #include <sys/types.h>
    #include <arpa/inet.h>
    #include <linux/if_ether.h>
    /* ... */

    /* From the example above: tcpdump -i em1 port 22 -dd */
    struct sock_filter code[] = {
	    { 0x28,  0,  0, 0x0000000c },
	    { 0x15,  0,  8, 0x000086dd },
	    { 0x30,  0,  0, 0x00000014 },
	    { 0x15,  2,  0, 0x00000084 },
	    { 0x15,  1,  0, 0x00000006 },
	    { 0x15,  0, 17, 0x00000011 },
	    { 0x28,  0,  0, 0x00000036 },
	    { 0x15, 14,  0, 0x00000016 },
	    { 0x28,  0,  0, 0x00000038 },
	    { 0x15, 12, 13, 0x00000016 },
	    { 0x15,  0, 12, 0x00000800 },
	    { 0x30,  0,  0, 0x00000017 },
	    { 0x15,  2,  0, 0x00000084 },
	    { 0x15,  1,  0, 0x00000006 },
	    { 0x15,  0,  8, 0x00000011 },
	    { 0x28,  0,  0, 0x00000014 },
	    { 0x45,  6,  0, 0x00001fff },
	    { 0xb1,  0,  0, 0x0000000e },
	    { 0x48,  0,  0, 0x0000000e },
	    { 0x15,  2,  0, 0x00000016 },
	    { 0x48,  0,  0, 0x00000010 },
	    { 0x15,  0,  1, 0x00000016 },
	    { 0x06,  0,  0, 0x0000ffff },
	    { 0x06,  0,  0, 0x00000000 },
    };

    struct sock_fprog bpf = {
	    .len = ARRAY_SIZE(code),
	    .filter = code,
    };

    sock = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
    if (sock < 0)
	    /* ... bail out ... */

    ret = setsockopt(sock, SOL_SOCKET, SO_ATTACH_FILTER, &bpf, sizeof(bpf));
    if (ret < 0)
	    /* ... bail out ... */

    /* ... */
    close(sock);

```
涓婇潰鐨勭ず渚嬩唬鐮佷负 PF_PACKET 濂楁帴瀛楅檮鍔犱簡涓€涓鎺ュ瓧杩囨护鍣紝浠ヨ鎵€鏈夌鍙ｄ负 22
鐨?IPv4/IPv6 鏁版嵁鍖呴€氳繃銆傚叾浣欑殑鏁版嵁鍖呭皢琚濂楁帴瀛椾涪寮冦€?
瀵?SO_DETACH_FILTER 鐨?setsockopt(2) 璋冪敤涓嶉渶瑕佷换浣曞弬鏁帮紝鑰岀敤浜庨槻姝㈣繃婊ゅ櫒
琚垎绂荤殑 SO_LOCK_FILTER 鎺ュ彈涓€涓彇鍊间负 0 鎴?1 鐨勬暣鏁板€笺€?
璇锋敞鎰忥紝濂楁帴瀛楄繃婊ゅ櫒骞朵笉浠呴檺浜?PF_PACKET 濂楁帴瀛楋紝涔熷彲浠ョ敤浜庡叾浠栧鎺ュ瓧鏃忋€?
绯荤粺璋冪敤鎬荤粨锛?
 - setsockopt(sockfd, SOL_SOCKET, SO_ATTACH_FILTER, &val, sizeof(val));
 - setsockopt(sockfd, SOL_SOCKET, SO_DETACH_FILTER, &val, sizeof(val));
 - setsockopt(sockfd, SOL_SOCKET, SO_LOCK_FILTER,   &val, sizeof(val));

閫氬父锛屽湪鍖呭鎺ュ瓧涓婅繘琛屽鎺ュ瓧杩囨护鐨勫ぇ閮ㄥ垎鐢ㄤ緥閮戒細琚?libpcap 浠ラ珮绾ц娉曡鐩栵紝
鍥犳浣滀负搴旂敤绋嬪簭寮€鍙戣€咃紝浣犲簲璇ュ潥鎸佷娇鐢ㄥ畠銆俵ibpcap 鍦ㄦ墍鏈夎繖浜涗箣涓婂皝瑁呬簡鑷繁鐨?灞傘€?
闄ら潪 i) 涓嶄娇鐢?涓嶉摼鎺?libpcap 涓嶅彲琛岋紝ii) 鎵€闇€鐨?BPF 杩囨护鍣ㄤ娇鐢ㄤ簡 libpcap
缂栬瘧鍣ㄤ笉鏀寔鐨?Linux 鎵╁睍锛宨ii) 杩囨护鍣ㄥ彲鑳芥洿澶嶆潅涓旀棤娉曠敤 libpcap 缂栬瘧鍣ㄥ共鍑€
鍦板疄鐜帮紝鎴?iv) 鐗瑰畾鐨勮繃婊ゅ櫒浠ｇ爜闇€瑕佷互涓嶅悓浜?libpcap 鍐呴儴缂栬瘧鍣ㄧ殑鏂瑰紡杩涜
浼樺寲锛涢偅涔堝湪杩欐牱鐨勬儏褰笅锛屾墜鍔ㄢ€滄墜鍐欌€濊繖鏍风殑杩囨护鍣ㄥ彲浠ヤ綔涓轰竴绉嶆浛浠ｆ柟妗堛€備緥濡傦紝
xt_bpf 鍜?cls_bpf 鐢ㄦ埛鍙兘鏈変細浜х敓鏇村鏉傝繃婊ゅ櫒浠ｇ爜鐨勯渶姹傦紝鎴栬€呬骇鐢熸棤娉曠敤
libpcap 琛ㄨ揪鐨勪唬鐮侊紙渚嬪涓嶅悓浠ｇ爜璺緞鏈変笉鍚岃繑鍥炵爜锛夈€傛澶栵紝BPF JIT 瀹炵幇鑰?鍙兘甯屾湜鎵嬪姩缂栧啓娴嬭瘯鐢ㄤ緥锛屽洜姝や篃闇€瑕佸 BPF 浠ｇ爜鐨勫簳灞傝闂€?
### BPF 寮曟搸涓庢寚浠ら泦


鍦?tools/bpf/ 涓嬫湁涓€涓悕涓?bpf_asm 鐨勫皬杈呭姪宸ュ叿锛屽彲鐢ㄤ簬涓轰笂涓€鑺傛彁鍒扮殑绀轰緥
鍦烘櫙缂栧啓搴曞眰杩囨护鍣ㄣ€傝繖閲屾彁鍒扮殑绫绘眹缂栬娉曞凡鍦?bpf_asm 涓疄鐜帮紝骞跺皢鐢ㄤ簬杩涗竴姝?鐨勮В閲婏紙鑰屼笉鏄洿鎺ュ鐞嗗彲璇绘€ц緝宸殑鎿嶄綔鐮侊紝鍘熺悊鏄浉鍚岀殑锛夈€傝璇硶绱у瘑妯′豢
Steven McCanne 鍜?Van Jacobson 鐨?BPF 璁烘枃銆?
BPF 鏋舵瀯鐢变互涓嬪熀鏈厓绱犵粍鎴愶細

  =======          ====================================================
  Element          Description
  =======          ====================================================
  A                32 bit wide accumulator
  X                32 bit wide X register
  M[]              16 x 32 bit wide misc registers aka "scratch memory
		   store", addressable from 0 to 15
  =======          ====================================================

涓€涓敱 bpf_asm 缈昏瘧鎴愨€渙pcodes鈥濈殑绋嬪簭鏄竴涓暟缁勶紝鍏?```

  op:16, jt:8, jf:8, k:32

```
鍏冪礌 op 鏄竴涓?16 浣嶅鐨勬搷浣滅爜锛屽叾涓紪鐮佷簡鐗瑰畾鐨勬寚浠ゃ€俲t 鍜?jf 鏄袱涓?8 浣?瀹界殑璺宠浆鐩爣锛屼竴涓敤浜庘€滄潯浠朵负鐪熸椂璺宠浆鈥濓紝鍙︿竴涓敤浜庘€滄潯浠朵负鍋囨椂璺宠浆鈥濄€傛渶鍚庯紝
鍏冪礌 k 鍖呭惈涓€涓潅椤瑰弬鏁帮紝鍙互鏍规嵁 op 涓粰瀹氱殑鎸囦护浠ヤ笉鍚屾柟寮忚В閲娿€?
鎸囦护闆嗙敱鍔犺浇銆佸瓨鍌ㄣ€佸垎鏀€乤lu銆佹潅椤瑰拰杩斿洖鎸囦护缁勬垚锛岃繖浜涗篃鍦?bpf_asm 璇硶涓?琛ㄧず銆備笅琛ㄥ垪鍑轰簡鎵€鏈夊彲鐢ㄧ殑 bpf_asm 鎸囦护锛屼互鍙婂畠浠湪 linux/filter.h 涓畾涔夌殑
鍩虹鎿嶄綔鐮佺殑鍚箟锛?
  ===========      ===================  =====================
  Instruction      Addressing mode      Description
  ===========      ===================  =====================
  ld               1, 2, 3, 4, 12       Load word into A
  ldi              4                    Load word into A
  ldh              1, 2                 Load half-word into A
  ldb              1, 2                 Load byte into A
  ldx              3, 4, 5, 12          Load word into X
  ldxi             4                    Load word into X
  ldxb             5                    Load byte into X

  st               3                    Store A into M[]
  stx              3                    Store X into M[]

  jmp              6                    Jump to label
  ja               6                    Jump to label
  jeq              7, 8, 9, 10          Jump on A == <x>
  jneq             9, 10                Jump on A != <x>
  jne              9, 10                Jump on A != <x>
  jlt              9, 10                Jump on A <  <x>
  jle              9, 10                Jump on A <= <x>
  jgt              7, 8, 9, 10          Jump on A >  <x>
  jge              7, 8, 9, 10          Jump on A >= <x>
  jset             7, 8, 9, 10          Jump on A &  <x>

  add              0, 4                 A + <x>
  sub              0, 4                 A - <x>
  mul              0, 4                 A * <x>
  div              0, 4                 A / <x>
  mod              0, 4                 A % <x>
  neg                                   !A
  and              0, 4                 A & <x>
  or               0, 4                 A | <x>
  xor              0, 4                 A ^ <x>
  lsh              0, 4                 A << <x>
  rsh              0, 4                 A >> <x>

  tax                                   Copy A into X
  txa                                   Copy X into A

  ret              4, 11                Return
  ===========      ===================  =====================

涓嬩竴琛ㄦ樉绀轰簡绗?2 鍒椾腑鐨勫鍧€鏍煎紡锛?
  ===============  ===================  ===============================================
  Addressing mode  Syntax               Description
  ===============  ===================  ===============================================
   0               x/%x                 Register X
   1               [k]                  BHW at byte offset k in the packet
   2               [x + k]              BHW at the offset X + k in the packet
   3               M[k]                 Word at offset k in M[]
   4               #k                   Literal value stored in k
   5               4**([k]&0xf)          Lower nibble ** 4 at byte offset k in the packet
   6               L                    Jump label L
   7               #k,Lt,Lf             Jump to Lt if true, otherwise jump to Lf
   8               x/%x,Lt,Lf           Jump to Lt if true, otherwise jump to Lf
   9               #k,Lt                Jump to Lt if predicate is true
  10               x/%x,Lt              Jump to Lt if predicate is true
  11               a/%a                 Accumulator A
  12               extension            BPF extension
  ===============  ===================  ===============================================

Linux 鍐呮牳杩樻湁鍑犱釜 BPF 鎵╁睍锛屽畠浠€氳繃灏?k 鍙傛暟鈥滈噸杞解€濅负璐熷亸绉诲姞涓婄壒瀹氱殑鎵╁睍
鍋忕Щ锛屼笌鍔犺浇鎸囦护杩欎竴绫讳竴璧蜂娇鐢ㄣ€傛绫?BPF 鎵╁睍鐨勭粨鏋滆鍔犺浇鍒?A 涓€?
鍙兘鐨?BPF 鎵╁睍濡備笅琛ㄦ墍绀猴細

  ===================================   =================================================
  Extension                             Description
  ===================================   =================================================
  len                                   skb->len
  proto                                 skb->protocol
  type                                  skb->pkt_type
  poff                                  Payload start offset
  ifidx                                 skb->dev->ifindex
  nla                                   Netlink attribute of type X with offset A
  nlan                                  Nested Netlink attribute of type X with offset A
  mark                                  skb->mark
  queue                                 skb->queue_mapping
  hatype                                skb->dev->type
  rxhash                                skb->hash
  cpu                                   raw_smp_processor_id()
  vlan_tci                              skb_vlan_tag_get(skb)
  vlan_avail                            skb_vlan_tag_present(skb)
  vlan_tpid                             skb->vlan_proto
  rand                                  get_random_u32()
  ===================================   =================================================

杩欎簺鎵╁睍涔熷彲浠ュ姞涓?'#' 鍓嶇紑銆?搴曞眰 BPF 绀轰緥锛?
```

  ldh [12]
  jne #0x806, drop
  ret #-1
  drop: ret #0

```
```

  ldh [12]
  jne #0x800, drop
  ldb [23]
  jneq #6, drop
  ret #-1
  drop: ret #0

```
```

  ldh [12]
  jne #0x800, drop
  ldb [23]
  jneq #1, drop
  # get a random uint32 number
  ld rand
  mod #4
  jneq #1, drop
  ret #-1
  drop: ret #0

```
```

  ld [4]                  /* offsetof(struct seccomp_data, arch) */
  jne #0xc000003e, bad    /* AUDIT_ARCH_X86_64 */
  ld [0]                  /* offsetof(struct seccomp_data, nr) */
  jeq #15, good           /* __NR_rt_sigreturn */
  jeq #231, good          /* __NR_exit_group */
  jeq #60, good           /* __NR_exit */
  jeq #0, good            /* __NR_read */
  jeq #1, good            /* __NR_write */
  jeq #5, good            /* __NR_fstat */
  jeq #9, good            /* __NR_mmap */
  jeq #14, good           /* __NR_rt_sigprocmask */
  jeq #13, good           /* __NR_rt_sigaction */
  jeq #35, good           /* __NR_nanosleep */
  bad: ret #0             /* SECCOMP_RET_KILL_THREAD */
  good: ret #0x7fff0000   /* SECCOMP_RET_ALLOW */

```
搴曞眰 BPF 鎵╁睍绀轰緥锛?
```

  ld ifidx
  jneq #13, drop
  ret #-1
  drop: ret #0

```
```

  ld vlan_tci
  jneq #10, drop
  ret #-1
  drop: ret #0

```
涓婇潰鐨勭ず渚嬩唬鐮佸彲浠ユ斁鍏ヤ竴涓枃浠讹紙杩欓噷绉颁负鈥渇oo鈥濓級锛岀劧鍚庝紶閫掔粰 bpf_asm 宸ュ叿
浠ョ敓鎴愭搷浣滅爜锛屽叾杈撳嚭鏄?xt_bpf 鍜?cls_bpf 鑳藉鐞嗚В骞跺彲鐩存帴鍔犺浇鐨勩€備娇鐢ㄤ笂闈?绀轰緥鐨?```

    $ ./bpf_asm foo
    4,40 0 0 12,21 0 1 2054,6 0 0 4294967295,6 0 0 0,

```
```

    $ ./bpf_asm -c foo
    { 0x28,  0,  0, 0x0000000c },
    { 0x15,  0,  1, 0x00000806 },
    { 0x06,  0,  0, 0xffffffff },
    { 0x06,  0,  0, 0000000000 },

```
鐗瑰埆鏄紝鐢变簬涓?xt_bpf 鎴?cls_bpf 涓€璧蜂娇鐢ㄥ彲鑳藉鑷存洿澶嶆潅鐨?BPF 杩囨护鍣紝涓€
寮€濮嬪彲鑳藉苟涓嶆槑鏄撅紝鍥犳鍦ㄩ檮鍔犲埌鐪熷疄绯荤粺涔嬪墠娴嬭瘯杩囨护鍣ㄦ槸寰堝ソ鐨勫仛娉曘€備负姝わ紝
鍐呮牳婧愪唬鐮佺洰褰曚笅鐨?tools/bpf/ 涓湁涓€涓悕涓?bpf_dbg 鐨勫皬宸ュ叿銆傝璋冭瘯鍣ㄥ厑璁?閽堝缁欏畾鐨?pcap 鏂囦欢娴嬭瘯 BPF 杩囨护鍣紝瀵?pcap 鏁版嵁鍖呬笂鐨?BPF 浠ｇ爜杩涜鍗曟
鎵ц锛屽苟杩涜 BPF 鏈哄櫒瀵勫瓨鍣ㄨ浆鍌ㄣ€?
```

    # ./bpf_dbg

```
濡傛灉杈撳叆鍜岃緭鍑轰笉绛変簬 stdin/stdout锛宐pf_dbg 浼氬皢鏇夸唬鐨?stdin 婧愪綔涓虹涓€涓弬鏁帮紝
灏嗘浛浠ｇ殑 stdout 鎺ユ敹鍣ㄤ綔涓虹浜屼釜鍙傛暟锛屼緥濡?`./bpf_dbg test_in.txt test_out.txt`銆?
闄ゆ涔嬪锛屽彲浠ラ€氳繃鏂囦欢 "~/.bpf_dbg_init" 璁剧疆鐗瑰畾鐨?libreadline 閰嶇疆锛屽懡浠?鍘嗗彶瀛樺偍鍦ㄦ枃浠?"~/.bpf_dbg_history" 涓€?
bpf_dbg 涓殑浜や簰閫氳繃涓€涓悓鏍锋敮鎸佽嚜鍔ㄨˉ鍏ㄧ殑 shell 杩涜锛堝悗缁互 '>' 寮€澶寸殑
绀轰緥鍛戒护琛ㄧず bpf_dbg shell锛夈€傞€氬父鐨勫伐浣滄祦绋嬫槸鈥︹€?
- load bpf 6,40 0 0 12,21 0 3 2048,48 0 0 23,21 0 1 1,6 0 0 65535,6 0 0 0
  浠?bpf_asm 鐨勬爣鍑嗚緭鍑哄姞杞?BPF 杩囨护鍣紝鎴栫粡鐢变緥濡?`tcpdump -iem1 -ddd port 22 | tr '\n' ','` 杞崲鑰屾潵銆傝娉ㄦ剰锛屼负浜?JIT 璋冭瘯锛堜笅涓€鑺傦級锛屾鍛戒护浼氬垱寤轰竴涓复鏃跺鎺ュ瓧骞跺皢 BPF 浠ｇ爜鍔犺浇鍒板唴鏍镐腑銆傚洜姝わ紝瀹冨 JIT 寮€鍙戣€呬篃寰堟湁鐢ㄣ€?
- load pcap foo.pcap

  鍔犺浇鏍囧噯鐨?tcpdump pcap 鏂囦欢銆?
- run [<n>]

bpf passes:1 fails:9
  閬嶅巻 pcap 涓殑鎵€鏈夋暟鎹寘锛岀粺璁¤繃婊ゅ櫒灏嗕骇鐢熷灏戞閫氳繃锛坧ass锛夊拰澶辫触锛坒ail锛夈€?  鍙互缁欏畾瑕侀亶鍘嗙殑鏁版嵁鍖呮暟閲忎笂闄愩€?
```

	l0:	ldh [12]
	l1:	jeq #0x800, l2, l5
	l2:	ldb [23]
	l3:	jeq #0x1, l4, l5
	l4:	ret #0xffff
	l5:	ret #0

  Prints out BPF code disassembly.

```
```

	/* { op, jt, jf, k }, */
	{ 0x28,  0,  0, 0x0000000c },
	{ 0x15,  0,  3, 0x00000800 },
	{ 0x30,  0,  0, 0x00000017 },
	{ 0x15,  0,  1, 0x00000001 },
	{ 0x06,  0,  0, 0x0000ffff },
	{ 0x06,  0,  0, 0000000000 },

  Prints out C-style BPF code dump.

```
```

	breakpoint at: l0:	ldh [12]

```
```

	breakpoint at: l1:	jeq #0x800, l2, l5

  ...

  Sets breakpoints at particular BPF instructions. Issuing a `run` command
  will walk through the pcap file continuing from the current packet and
  break when a breakpoint is being hit (another `run` will continue from
  the currently active breakpoint executing next instructions):

  * run::

	-- register dump --
	pc:       [0]                       <-- program counter
	code:     [40] jt[0] jf[0] k[12]    <-- plain BPF code of current instruction
	curr:     l0:	ldh [12]              <-- disassembly of current instruction
	A:        [00000000][0]             <-- content of A (hex, decimal)
	X:        [00000000][0]             <-- content of X (hex, decimal)
	M[0,15]:  [00000000][0]             <-- folded content of M (hex, decimal)
	-- packet dump --                   <-- Current packet from pcap (hex)
	len: 42
	    0: 00 19 cb 55 55 a4 00 14 a4 43 78 69 08 06 00 01
	16: 08 00 06 04 00 01 00 14 a4 43 78 69 0a 3b 01 26
	32: 00 00 00 00 00 00 0a 3b 01 01
	(breakpoint)
	>

  * breakpoint::

	breakpoints: 0 1

    Prints currently set breakpoints.

```
- step [-<n>, +<n>]

  浠庡綋鍓?pc 鍋忕Щ閲忓紑濮嬪 BPF 绋嬪簭杩涜鍗曟鎵ц銆傚洜姝わ紝姣忔璋冪敤 step 鏃讹紝閮戒細
  杈撳嚭涓婇潰鐨勫瘎瀛樺櫒杞偍銆傝繖鍙互鍦ㄦ椂闂翠笂鍚戝墠鍜屽悜鍚庣Щ鍔紝鍗曠函鐨?`step` 浼氬湪涓?  涓€鏉?BPF 鎸囦护澶勪腑鏂紝鍗?+1銆傦紙杩欓噷涓嶉渶瑕佸彂鍑?`run`銆傦級

- select <n>

  浠?pcap 鏂囦欢涓€夋嫨涓€涓粰瀹氱殑鏁版嵁鍖呬互缁х画銆傚洜姝わ紝鍦ㄤ笅涓€娆?`run` 鎴?`step`
  鏃讹紝BPF 绋嬪簭灏嗛拡瀵圭敤鎴烽鍏堥€夋嫨鐨勬暟鎹寘杩涜姹傚€笺€傜紪鍙蜂笌 Wireshark 涓€鏍蜂粠
  绱㈠紩 1 寮€濮嬨€?
- quit

  閫€鍑?bpf_dbg銆?
### JIT 缂栬瘧鍣?

Linux 鍐呮牳鍐呯疆浜嗕竴涓敤浜?x86_64銆丼PARC銆丳owerPC銆丄RM銆丄RM64銆丮IPS銆丷ISC-V銆?s390 鍜?ARC 鐨?BPF JIT 缂栬瘧鍣紝鍙€氳繃 CONFIG_BPF_JIT 鍚敤銆傚鏋滆缃簡
```

  echo 1 > /proc/sys/net/core/bpf_jit_enable

```
瀵逛簬 JIT 寮€鍙戣€咃紝杩涜瀹¤绛夌敤閫旓紝姣忔缂栬瘧杩愯閮藉彲浠ヨ緭鍑虹敓鎴愮殑
```

  echo 2 > /proc/sys/net/core/bpf_jit_enable

```
```

    [ 3389.935842] flen=6 proglen=70 pass=3 image=ffffffffa0069c8f
    [ 3389.935847] JIT code: 00000000: 55 48 89 e5 48 83 ec 60 48 89 5d f8 44 8b 4f 68
    [ 3389.935849] JIT code: 00000010: 44 2b 4f 6c 4c 8b 87 d8 00 00 00 be 0c 00 00 00
    [ 3389.935850] JIT code: 00000020: e8 1d 94 ff e0 3d 00 08 00 00 75 16 be 17 00 00
    [ 3389.935851] JIT code: 00000030: 00 e8 28 94 ff e0 83 f8 01 75 07 b8 ff ff 00 00
    [ 3389.935852] JIT code: 00000040: eb 02 31 c0 c9 c3

```
褰?CONFIG_BPF_JIT_ALWAYS_ON 鍚敤鏃讹紝bpf_jit_enable 琚案涔呰涓?1锛岃缃叾浠?浠讳綍鍊奸兘浼氳繑鍥炲け璐ャ€傚嵆浣垮皢 bpf_jit_enable 璁句负 2 涔熸槸濡傛锛屽洜涓哄皢鏈€缁?JIT
鏄犲儚杞偍鍒板唴鏍告棩蹇楁槸涓嶆帹鑽愮殑锛屼竴鑸缓璁敼鐢ㄩ€氳繃 bpftool锛堜綅浜?tools/bpf/bpftool/
涓嬶級杩涜鑷渷銆?
鍦ㄥ唴鏍告簮浠ｇ爜鏍戠殑 tools/bpf/ 涓嬶紝bpf_jit_disasm 鐢ㄤ簬
```

	# ./bpf_jit_disasm
	70 bytes emitted from JIT compiler (pass:3, flen:6)
	ffffffffa0069c8f + <x>:
	0:	push   %rbp
	1:	mov    %rsp,%rbp
	4:	sub    $0x60,%rsp
	8:	mov    %rbx,-0x8(%rbp)
	c:	mov    0x68(%rdi),%r9d
	10:	sub    0x6c(%rdi),%r9d
	14:	mov    0xd8(%rdi),%r8
	1b:	mov    $0xc,%esi
	20:	callq  0xffffffffe0ff9442
	25:	cmp    $0x800,%eax
	2a:	jne    0x0000000000000042
	2c:	mov    $0x17,%esi
	31:	callq  0xffffffffe0ff945e
	36:	cmp    $0x1,%eax
	39:	jne    0x0000000000000042
	3b:	mov    $0xffff,%eax
	40:	jmp    0x0000000000000044
	42:	xor    %eax,%eax
	44:	leaveq
	45:	retq

	Issuing option `-o` will "annotate" opcodes to resulting assembler
	.instructions, which can be very useful for JIT developers:

	# ./bpf_jit_disasm -o
	70 bytes emitted from JIT compiler (pass:3, flen:6)
	ffffffffa0069c8f + <x>:
	0:	push   %rbp
		55
	1:	mov    %rsp,%rbp
		48 89 e5
	4:	sub    $0x60,%rsp
		48 83 ec 60
	8:	mov    %rbx,-0x8(%rbp)
		48 89 5d f8
	c:	mov    0x68(%rdi),%r9d
		44 8b 4f 68
	10:	sub    0x6c(%rdi),%r9d
		44 2b 4f 6c
	14:	mov    0xd8(%rdi),%r8
		4c 8b 87 d8 00 00 00
	1b:	mov    $0xc,%esi
		be 0c 00 00 00
	20:	callq  0xffffffffe0ff9442
		e8 1d 94 ff e0
	25:	cmp    $0x800,%eax
		3d 00 08 00 00
	2a:	jne    0x0000000000000042
		75 16
	2c:	mov    $0x17,%esi
		be 17 00 00 00
	31:	callq  0xffffffffe0ff945e
		e8 28 94 ff e0
	36:	cmp    $0x1,%eax
		83 f8 01
	39:	jne    0x0000000000000042
		75 07
	3b:	mov    $0xffff,%eax
		b8 ff ff 00 00
	40:	jmp    0x0000000000000044
		eb 02
	42:	xor    %eax,%eax
		31 c0
	44:	leaveq
		c9
	45:	retq
		c3

```
瀵逛簬 BPF JIT 寮€鍙戣€咃紝bpf_jit_disasm銆乥pf_asm 鍜?bpf_dbg 鎻愪緵浜嗕竴涓湁鐢ㄧ殑宸ュ叿
閾撅紝鐢ㄤ簬寮€鍙戝拰娴嬭瘯鍐呮牳鐨?JIT 缂栬瘧鍣ㄣ€?
### BPF 鍐呮牳鍐呴儴鏈哄埗


鍦ㄥ唴鏍歌В閲婂櫒鍐呴儴锛屼娇鐢ㄧ殑鏄竴绉嶄笉鍚岀殑鎸囦护闆嗘牸寮忥紝鍏跺簳灞傚師鐞嗕笌鍓嶉潰娈佃惤鎻忚堪
鐨?BPF 鐩镐技銆傜劧鑰岋紝璇ユ寚浠ら泦鏍煎紡鏇磋创杩戝簳灞傛灦鏋勮繘琛屽缓妯★紝浠ユā浠垮師鐢熸寚浠ら泦锛?浠庤€屽彲浠ヨ幏寰楁洿濂界殑鎬ц兘锛堣瑙佸悗鏂囷級銆傝繖涓柊鐨?ISA 琚О涓?eBPF銆傝鎯呰鍙傞槄
../bpf/index.rst銆傦紙娉ㄦ剰锛氭簮鑷?[e]xtended BPF 鐨?eBPF 涓?BPF 鎵╁睍骞朵笉鐩稿悓锛?eBPF 鏄竴绉?ISA锛岃€?BPF 鎵╁睍鍙互杩芥函鍒扮粡鍏?BPF 瀵?BPF_LD | BPF_{B,H,W} |
BPF_ABS 鎸囦护鐨勨€滈噸杞解€濄€傦級

鏂版寚浠ら泦鏈€鍒濊璁℃椂鐨勫彲鑳界洰鏍囨槸鐢ㄢ€滃彈闄?C锛坮estricted C锛夆€濈紪鍐欑▼搴忥紝骞堕€氳繃
鍙€夌殑 GCC/LLVM 鍚庣缂栬瘧涓?eBPF锛屼粠鑰岃兘澶熶互鏈€灏忕殑鎬ц兘寮€閿€鍒嗕袱姝ュ嵆鏃舵槧灏?鍒扮幇浠?64 浣?CPU锛屽嵆 C -> eBPF -> 鍘熺敓浠ｇ爜銆?
鐩墠锛屾柊鏍煎紡琚敤浜庤繍琛岀敤鎴?BPF 绋嬪簭锛屽叾涓寘鎷?seccomp BPF銆佺粡鍏稿鎺ュ瓧
杩囨护鍣ㄣ€乧ls_bpf 娴侀噺鍒嗙被鍣ㄣ€乼eam 椹卞姩鐢ㄤ簬鍏惰礋杞藉潎琛℃ā寮忕殑鍒嗙被鍣ㄣ€乶etfilter
鐨?xt_bpf 鎵╁睍銆丳TP 瑙ｆ瀽鍣?鍒嗙被鍣ㄧ瓑绛夈€傚畠浠叏閮ㄧ敱鍐呮牳鍐呴儴杞崲涓烘柊鐨勬寚浠ら泦
琛ㄧず锛屽苟鍦?eBPF 瑙ｉ噴鍣ㄤ腑杩愯銆傚浜庡唴鏍稿唴澶勭悊绋嬪簭锛岃繖涓€鍒囬€氳繃 bpf_prog_create()
寤虹珛杩囨护鍣ㄣ€侀€氳繃 bpf_prog_destroy() 閿€姣佽繃婊ゅ櫒鏉ラ€忔槑鍦板伐浣溿€傚嚱鏁?bpf_prog_run(filter, ctx) 閫忔槑鍦拌皟鐢?eBPF 瑙ｉ噴鍣ㄦ垨 JIT 缂栬瘧鍚庣殑浠ｇ爜鏉ヨ繍琛?杩囨护鍣ㄣ€?filter' 鏄潵鑷?bpf_prog_create() 鐨勬寚鍚?struct bpf_prog 鐨勬寚閽堬紝
'ctx' 鏄粰瀹氱殑涓婁笅鏂囷紙渚嬪 skb 鎸囬拡锛夈€傚湪鍚庡彴杞崲涓烘柊甯冨眬涔嬪墠锛?bpf_check_classic() 鐨勬墍鏈夌害鏉熷拰闄愬埗閮介€傜敤锛?
鐩墠锛岀粡鍏?BPF 鏍煎紡鐢ㄤ簬澶у鏁?32 浣嶆灦鏋勪笂鐨?JIT 缂栬瘧锛岃€?x86-64銆乤arch64銆?s390x銆乸owerpc64銆乻parc64銆乤rm32銆乺iscv64銆乺iscv32銆乴oongarch64銆乤rc 鍒?浠?eBPF 鎸囦护闆嗚繘琛?JIT 缂栬瘧銆?
### 娴嬭瘯


闄や簡 BPF 宸ュ叿閾句箣澶栵紝鍐呮牳杩橀檮甯︿竴涓祴璇曟ā鍧楋紝鍏朵腑鍖呭惈閽堝缁忓吀鍜?eBPF 鐨?鍚勭娴嬭瘯鐢ㄤ緥锛屽彲浠ュ BPF 瑙ｉ噴鍣ㄥ拰 JIT 缂栬瘧鍣ㄦ墽琛屻€傚畠浣嶄簬 lib/test_bpf.c 涓紝
骞朵笖
```

  CONFIG_TEST_BPF=m

```
鍦ㄦā鍧楁瀯寤哄苟瀹夎鍚庯紝鍙互閫氳繃 insmod 鎴?modprobe 閽堝 'test_bpf' 妯″潡鎵ц
娴嬭瘯濂椾欢銆傛祴璇曠敤渚嬬殑缁撴灉锛堝寘鎷互绾崇涓哄崟浣嶇殑鏃堕棿锛夊彲浠ュ湪鍐呮牳鏃ュ織锛坉mesg锛?涓壘鍒般€?
### 鏉傞」


姝ゅ锛宼rinity锛圠inux 绯荤粺璋冪敤妯＄硦娴嬭瘯鍣級涔熷唴缃簡瀵?BPF 鍜?SECCOMP-BPF
鍐呮牳妯＄硦娴嬭瘯鐨勬敮鎸併€?
### 浣滆€?

鎾板啓鏈枃妗ｆ槸甯屾湜瀹冭兘鏈夋墍鍔╃泭锛屽苟涓烘綔鍦ㄧ殑 BPF 榛戝鎴栧畨鍏ㄥ璁′汉鍛樻彁渚涘搴曞眰
鏋舵瀯鏇村ソ鐨勬瑙堛€?
- Jay Schulist <jschlst@samba.org>
- Daniel Borkmann <daniel@iogearbox.net>
- Alexei Starovoitov <ast@kernel.org>
