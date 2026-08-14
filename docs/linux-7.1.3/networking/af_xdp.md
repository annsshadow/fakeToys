
## AF_XDP


## 姒傝堪


AF_XDP 鏄竴涓负楂樻€ц兘鏁版嵁鍖呭鐞嗚€屼紭鍖栫殑鍦板潃鏃忋€?
鏈枃妗ｅ亣瀹氳鑰呭凡缁忕啛鎮?BPF 鍜?XDP銆傚鏋滀笉鐔熸倝锛孋ilium 椤圭洰鍦?http://cilium.readthedocs.io/en/latest/bpf/ 鎻愪緵浜嗕竴浠戒紭绉€鐨勫弬鑰冩寚鍗椼€?
閫氳繃 XDP 绋嬪簭涓殑 XDP_REDIRECT 鍔ㄤ綔锛岀▼搴忓彲浠ヤ娇鐢?bpf_redirect_map() 鍑芥暟灏嗗叆鍙ｅ抚锛坕ngress frame锛夐噸瀹氬悜鍒板叾浠栧惎鐢ㄤ簡 XDP 鐨?netdev锛屻€侫F_XDP 濂楁帴瀛椾娇寰?XDP 绋嬪簭鑳藉灏嗗抚閲嶅畾鍚戝埌鐢ㄦ埛绌洪棿搴旂敤绋嬪簭涓殑涓€鍧楀唴瀛樼紦鍐插尯銆?
涓€涓?AF_XDP 濂楁帴瀛楋紙XSK锛変娇鐢ㄦ櫘閫氱殑 socket() 绯荤粺璋冪敤鍒涘缓銆傛瘡涓?XSK 鍏宠仈涓や釜
鐜紙ring锛夛細RX 鐜拰 TX 鐜€傚鎺ュ瓧鍙互鍦?RX 鐜笂鎺ユ敹鏁版嵁鍖咃紝涔熷彲浠ュ湪 TX 鐜笂鍙戦€?鏁版嵁鍖呫€傝繖涓や釜鐜垎鍒€氳繃 setsockopt XDP_RX_RING 鍜?XDP_TX_RING 娉ㄥ唽骞惰瀹氬ぇ灏忋€?姣忎釜濂楁帴瀛楀繀椤昏嚦灏戞嫢鏈夊叾涓竴涓幆銆俁X 鎴?TX 鎻忚堪绗︾幆鎸囧悜绉颁负 UMEM 鐨勫唴瀛樺尯鍩熶腑鐨?涓€涓暟鎹紦鍐插尯銆俁X 鍜?TX 鍙互鍏变韩鍚屼竴涓?UMEM锛屼粠鑰屾暟鎹寘鏃犻渶鍦?RX 鍜?TX 涔嬮棿澶嶅埗銆?姝ゅ锛屽鏋滄煇涓暟鎹寘鍥犱负鍙兘闇€瑕侀噸浼犺€岃淇濈暀涓€娈垫椂闂达紝鎸囧悜璇ユ暟鎹寘鐨勬弿杩扮鍙互
琚敼涓烘寚鍚戝彟涓€涓暟鎹寘锛屽苟绔嬪嵆琚噸鐢ㄣ€傝繖涔熼伩鍏嶄簡鏁版嵁澶嶅埗銆?
UMEM 鐢辫嫢骞插ぇ灏忕浉绛夌殑鍧楋紙chunk锛夌粍鎴愩€傛煇涓幆涓殑鎻忚堪绗﹂€氳繃寮曠敤鍏?addr 鏉ュ紩鐢ㄤ竴涓?甯э紙frame锛夈€俛ddr 鍙槸鏁翠釜 UMEM 鍖哄煙鍐呯殑涓€涓亸绉婚噺銆傜敤鎴风┖闂翠娇鐢ㄥ畠璁や负鏈€鍚堥€傜殑鏂瑰紡
锛坢alloc銆乵map銆佸ぇ椤电瓑锛変负杩欎釜 UMEM 鍒嗛厤鍐呭瓨銆傜劧鍚庤繖鍧楀唴瀛樺尯鍩熼€氳繃鏂扮殑 setsockopt
XDP_UMEM_REG 娉ㄥ唽鍒板唴鏍搞€俇MEM 杩樻湁涓や釜鐜細FILL 鐜拰 COMPLETION 鐜€侳ILL 鐜敱
搴旂敤绋嬪簭浣跨敤锛屽悜涓嬩紶閫?addr 渚涘唴鏍稿～鍏?RX 鏁版嵁鍖呮暟鎹€傛瘡涓暟鎹寘琚帴鏀跺悗锛屽杩欎簺甯х殑
寮曠敤灏变細鍑虹幇鍦?RX 鐜腑銆傚彟涓€鏂归潰锛孋OMPLETION 鐜寘鍚唴鏍稿凡缁忓畬鏁村彂閫併€佺幇鍦ㄥ彲浠ヨ
鐢ㄦ埛绌洪棿鍐嶆鐢ㄤ簬 TX 鎴?RX 鐨勫抚 addr銆傚洜姝わ紝鍑虹幇鍦?COMPLETION 鐜腑鐨勫抚 addr 鏄箣鍓?浣跨敤 TX 鐜彂閫佺殑閭ｄ簺 addr銆傛€讳箣锛孯X 鍜?FILL 鐜敤浜?RX 璺緞锛岃€?TX 鍜?COMPLETION
鐜敤浜?TX 璺緞銆?
濂楁帴瀛楁渶缁堥€氳繃 bind() 璋冪敤缁戝畾鍒版煇涓澶囦互鍙婅璁惧涓婄殑涓€涓壒瀹氶槦鍒?id锛岃€屼笖鍙湁鍦?bind 瀹屾垚涔嬪悗锛屾祦閲忔墠浼氬紑濮嬫祦鍔ㄣ€?
濡傛灉闇€瑕侊紝UMEM 鍙互鍦ㄨ繘绋嬩箣闂村叡浜€傚鏋滄煇涓繘绋嬫兂杩欐牱鍋氾紝瀹冨彧闇€璺宠繃 UMEM 鍙婂叾瀵瑰簲
涓や釜鐜殑娉ㄥ唽锛屽湪 bind 璋冪敤涓缃?XDP_SHARED_UMEM 鏍囧織锛屽苟鎻愪氦瀹冩兂瑕佸叡浜?UMEM 鐨?閭ｄ釜杩涚▼鐨?XSK 浠ュ強瀹冭嚜宸辨柊鍒涘缓鐨?XSK 濂楁帴瀛椼€傜劧鍚庢柊杩涚▼浼氬湪瀹冭嚜宸辩殑 RX 鐜腑鏀跺埌鎸囧悜
杩欎釜鍏变韩 UMEM 鐨勫抚 addr 寮曠敤銆傝娉ㄦ剰锛岀敱浜庣幆缁撴瀯鍑轰簬鎬ц兘鍘熷洜鏄崟娑堣垂鑰?/ 鍗曠敓浜ц€?锛坰ingle-consumer / single-producer锛夌殑锛屾柊杩涚▼蹇呴』鍒涘缓瀹冭嚜宸辩殑甯︽湁 RX 鍜?TX 鐜殑
濂楁帴瀛楋紝鍥犱负瀹冩棤娉曚笌璇ヨ繘绋嬪叡浜繖閮ㄥ垎銆傝繖涔熸槸涓轰粈涔堟瘡涓?UMEM 鍙湁涓€缁?FILL 鍜?COMPLETION 鐜殑鍘熷洜銆傚鐞?UMEM 鏄崟涓繘绋嬬殑璐ｄ换銆?
閭ｄ箞鏁版嵁鍖呮槸濡備綍浠?XDP 绋嬪簭鍒嗗彂鍒板悇涓?XSK 鐨勫憿锛熸湁涓€涓О涓?XSKMAP锛堝畬鏁村悕绉?BPF_MAP_TYPE_XSKMAP锛夌殑 BPF map銆傜敤鎴风┖闂村簲鐢ㄧ▼搴忓彲浠ュ湪杩欎釜 map 涓换鎰忎綅缃斁缃竴涓?XSK銆傜劧鍚?XDP 绋嬪簭鍙互灏嗘暟鎹寘閲嶅畾鍚戝埌璇?map 涓殑鐗瑰畾绱㈠紩锛屾鏃?XDP 浼氭牎楠岃 map 涓殑
XSK 纭疄缁戝畾鍒颁簡閭ｄ釜璁惧鍜岀幆缂栧彿銆傚鏋滄病鏈夛紝鏁版嵁鍖呬細琚涪寮冦€傚鏋滆绱㈠紩澶?map 涓虹┖锛?鏁版嵁鍖呬篃浼氳涓㈠純銆傝繖涔熸剰鍛崇潃锛屽綋鍓嶅繀椤诲姞杞戒竴涓?XDP 绋嬪簭锛堝苟涓?XSKMAP 涓嚦灏戞湁涓€涓?XSK锛夋墠鑳介€氳繃 XSK 灏嗕换浣曟祦閲忛€佸埌鐢ㄦ埛绌洪棿銆?
AF_XDP 鍙互鍦ㄤ袱绉嶄笉鍚岀殑妯″紡涓嬭繍琛岋細XDP_SKB 鍜?XDP_DRV銆傚鏋滈┍鍔ㄤ笉鏀寔 XDP锛屾垨鑰呭湪
鍔犺浇 XDP 绋嬪簭鏃舵樉寮忛€夋嫨浜?XDP_SKB锛屽垯閲囩敤 XDP_SKB 妯″紡锛岃妯″紡浣跨敤 SKB 閰嶅悎閫氱敤鐨?XDP 鏀寔锛屽苟灏嗘暟鎹鍒跺埌鐢ㄦ埛绌洪棿銆傝繖鏄竴绉嶉€傜敤浜庝换浣曠綉缁滆澶囩殑鍥為€€妯″紡銆傚彟涓€鏂归潰锛?濡傛灉椹卞姩鏀寔 XDP锛孉F_XDP 浠ｇ爜灏嗕娇鐢ㄥ畠鏉ユ彁渚涙洿濂界殑鎬ц兘锛屼絾鏁版嵁浠嶇劧浼氳澶嶅埗鍒扮敤鎴风┖闂淬€?
## 姒傚康


瑕佷娇鐢?AF_XDP 濂楁帴瀛楋紝闇€瑕佸缓绔嬭嫢骞茬浉鍏宠仈鐨勫璞°€傝繖浜涘璞″強鍏堕€夐」灏嗗湪浠ヤ笅鍚勮妭涓鏄庛€?
鍏充簬 AF_XDP 宸ヤ綔鍘熺悊鐨勬瑙堬紝浣犱篃鍙互鍙傞槄 2018 骞?Linux Plumbers 浼氳涓婂叧浜庤涓婚鐨?鏂囩珷锛歨ttp://vger.kernel.org/lpc_net2018_talks/lpc18_paper_af_xdp_perf-v2.pdf銆傝鍕?鍙傝€?2017 骞村叧浜庘€淎F_PACKET v4鈥濓紙AF_XDP 鐨勯娆″皾璇曪級鐨勬枃绔犮€傝嚜閭ｄ互鍚庡嚑涔庢墍鏈夊唴瀹归兘
鏀瑰彉浜嗐€侸onathan Corbet 涔熷湪 LWN 涓婂啓浜嗕竴绡囦紭绉€鐨勬枃绔犫€淎ccelerating networking with
AF_XDP鈥濄€傚彲鍦?https://lwn.net/Articles/750845/ 鎵惧埌銆?
### UMEM


UMEM 鏄竴娈佃櫄鎷熻繛缁唴瀛樺尯鍩燂紝琚垝鍒嗕负澶у皬鐩哥瓑鐨勫抚锛坒rame锛夈€備竴涓?UMEM 鍏宠仈鍒颁竴涓?netdev 浠ュ強璇?netdev 鐨勪竴涓壒瀹氶槦鍒?id銆傚畠鏄€氳繃浣跨敤 XDP_UMEM_REG setsockopt 绯荤粺
璋冪敤鏉ュ垱寤哄拰閰嶇疆锛堝潡澶у皬銆乭eadroom銆佽捣濮嬪湴鍧€鍜屽ぇ灏忥級鐨勩€俇MEM 閫氳繃 bind() 绯荤粺璋冪敤
缁戝畾鍒?netdev 鍜岄槦鍒?id銆?
涓€涓?AF_XDP 濂楁帴瀛楅摼鎺ュ埌鍗曚釜 UMEM锛屼絾涓€涓?UMEM 鍙互鏈夊涓?AF_XDP 濂楁帴瀛椼€傝鍏变韩閫氳繃
鏌愪釜濂楁帴瀛?A 鍒涘缓鐨?UMEM锛屼笅涓€涓鎺ュ瓧 B 鍙互閫氳繃鍦?struct sockaddr_xdp 鎴愬憳
sxdp_flags 涓缃?XDP_SHARED_UMEM 鏍囧織锛屽苟灏?A 鐨勬枃浠舵弿杩扮浼犵粰 struct sockaddr_xdp
鎴愬憳 sxdp_shared_umem_fd 鏉ュ疄鐜般€?
UMEM 鏈変袱涓崟鐢熶骇鑰?/ 鍗曟秷璐硅€咃紙single-producer / single-consumer锛夌幆锛岀敤浜庡湪鍐呮牳鍜?鐢ㄦ埛绌洪棿搴旂敤绋嬪簭涔嬮棿杞Щ UMEM 甯х殑鎵€鏈夋潈銆?
### 鐜?

鍏辨湁鍥涚涓嶅悓鐨勭幆锛欶ILL銆丆OMPLETION銆丷X 鍜?TX銆傛墍鏈夌幆閮芥槸鍗曠敓浜ц€?/ 鍗曟秷璐硅€呯殑锛屽洜姝?褰撳涓繘绋?/ 绾跨▼璇诲啓瀹冧滑鏃讹紝鐢ㄦ埛绌洪棿搴旂敤绋嬪簭闇€瑕佹樉寮忕殑鍚屾銆?
UMEM 浣跨敤涓や釜鐜細FILL 鍜?COMPLETION銆備笌 UMEM 鍏宠仈鐨勬瘡涓鎺ュ瓧蹇呴』鎷ユ湁 RX 闃熷垪銆?TX 闃熷垪鎴栦袱鑰呯殕鏈夈€備緥濡傦紝鍋囪鏈変竴涓寘鍚洓涓鎺ュ瓧锛堥兘杩涜 TX 鍜?RX锛夌殑閰嶇疆銆傞偅涔堝皢浼?鏈変竴涓?FILL 鐜€佷竴涓?COMPLETION 鐜€佸洓涓?TX 鐜拰鍥涗釜 RX 鐜€?
杩欎簺鐜槸鍩轰簬 head锛堢敓浜ц€咃級/ tail锛堟秷璐硅€咃級鐨勭幆銆傜敓浜ц€呭湪 struct xdp_ring 鐨?producer 鎴愬憳鎵€鎸囧悜鐨勭储寮曞鍐欏叆鏁版嵁鐜紝骞堕€掑鐢熶骇鑰呯储寮曘€傛秷璐硅€呭湪 struct xdp_ring 鐨?consumer 鎴愬憳鎵€鎸囧悜鐨勭储寮曞璇诲彇鏁版嵁鐜紝骞堕€掑娑堣垂鑰呯储寮曘€?
杩欎簺鐜€氳繃 _RING setsockopt 绯荤粺璋冪敤杩涜閰嶇疆鍜屽垱寤猴紝骞朵娇鐢ㄩ€傚綋鐨?mmap() 鍋忕Щ閲忔槧灏勫埌
鐢ㄦ埛绌洪棿锛圶DP_PGOFF_RX_RING銆乆DP_PGOFF_TX_RING銆乆DP_UMEM_PGOFF_FILL_RING 鍜?XDP_UMEM_PGOFF_COMPLETION_RING锛夈€?
鐜殑澶у皬蹇呴』鏄?2 鐨勫箓銆?
#### UMEM FILL 鐜?

FILL 鐜敤浜庡皢 UMEM 甯х殑鎵€鏈夋潈浠庣敤鎴风┖闂磋浆绉诲埌鍐呮牳绌洪棿銆俇MEM addr 鍦ㄨ鐜腑浼犻€掋€備緥濡傦紝
濡傛灉 UMEM 鏄?64k锛屾瘡涓潡鏄?4k锛岄偅涔?UMEM 鏈?16 涓潡锛屽彲浠ヤ紶閫?0 鍒?64k 涔嬮棿鐨?addr銆?
浼犻€掔粰鍐呮牳鐨勫抚鐢ㄤ簬鍏ュ彛璺緞锛圧X 鐜級銆?
鐢ㄦ埛搴旂敤绋嬪簭鍚戣繖涓幆鐢熶骇锛坧roduce锛塙MEM addr銆傝娉ㄦ剰锛屽鏋滀互瀵归綈鍧楁ā寮忚繍琛屽簲鐢ㄧ▼搴忥紝
鍐呮牳浼氬睆钄戒紶鍏ョ殑 addr銆備緥濡傦紝瀵逛簬 2k 鐨勫潡澶у皬锛宎ddr 鐨?log2(2048) 涓渶浣庢湁鏁堜綅锛圠SB锛?浼氳灞忚斀锛岃繖鎰忓懗鐫€ 2048銆?050 鍜?3000 閮芥寚鍚戝悓涓€涓潡銆傚鏋滅敤鎴峰簲鐢ㄧ▼搴忎互闈炲榻愬潡妯″紡
杩愯锛屽垯浼犲叆鐨?addr 浼氫繚鎸佸師鏍枫€?

#### UMEM COMPLETION 鐜?

COMPLETION 鐜敤浜庢妸 UMEM 甯х殑鎵€鏈夋潈浠庡唴鏍哥┖闂磋浆绉诲埌鐢ㄦ埛绌洪棿銆備笌 FILL 鐜竴鏍凤紝浣跨敤鐨勬槸
UMEM 绱㈠紩銆?
浠庡唴鏍镐紶閫掔粰鐢ㄦ埛绌洪棿鐨勫抚鏄凡缁忚鍙戦€侊紙TX 鐜級涓斿彲浠ヨ鐢ㄦ埛绌洪棿鍐嶆浣跨敤鐨勫抚銆?
鐢ㄦ埛搴旂敤绋嬪簭浠庤繖涓幆娑堣垂锛坈onsume锛塙MEM addr銆?

#### RX 鐜?

RX 鐜槸濂楁帴瀛楃殑鎺ユ敹绔€傜幆涓殑姣忎竴椤归兘鏄竴涓?struct xdp_desc 鎻忚堪绗︺€傝鎻忚堪绗﹀寘鍚?UMEM 鍋忕Щ锛坅ddr锛変互鍙婃暟鎹暱搴︼紙len锛夈€?
濡傛灉娌℃湁閫氳繃 FILL 鐜悜鍐呮牳浼犻€掍换浣曞抚锛孯X 鐜笂灏变笉浼氾紙涔熸棤娉曪級鍑虹幇浠讳綍鎻忚堪绗︺€?
鐢ㄦ埛搴旂敤绋嬪簭浠庤繖涓幆娑堣垂 struct xdp_desc 鎻忚堪绗︺€?
#### TX 鐜?

TX 鐜敤浜庡彂閫佸抚銆俿truct xdp_desc 鎻忚堪绗﹁濉厖锛堢储寮曘€侀暱搴﹀拰鍋忕Щ锛夊悗浼犲叆璇ョ幆銆?
瑕佸惎鍔ㄤ紶杈擄紝闇€瑕佷竴娆?sendmsg() 绯荤粺璋冪敤銆傝繖涓€鐐规湭鏉ュ彲鑳戒細鏀惧銆?
鐢ㄦ埛搴旂敤绋嬪簭鍚戣繖涓幆鐢熶骇 struct xdp_desc 鎻忚堪绗︺€?
## Libbpf


Libbpf 鏄竴涓敤浜?eBPF 鍜?XDP 鐨勮緟鍔╁簱锛屼娇杩欎簺鎶€鏈殑浣跨敤绠€鍗曞緢澶氥€傚畠杩樺寘鍚?tools/testing/selftests/bpf/xsk.h 涓殑鐗瑰畾杈呭姪鍑芥暟锛屼究浜庝娇鐢?AF_XDP銆傚畠鍖呭惈涓ょ
鍑芥暟锛氫竴绉嶅彲鐢ㄤ簬绠€鍖?AF_XDP 濂楁帴瀛楃殑寤虹珛锛屽彟涓€绉嶅彲鐢ㄤ簬鏁版嵁闈紙data plane锛変互瀹夊叏蹇€?鍦拌闂繖浜涚幆銆?
鎴戜滑寤鸿浣犱娇鐢ㄨ繖涓簱锛岄櫎闈炰綘宸茬粡鎴愪负楂樼骇鐢ㄦ埛銆傚畠浼氫娇浣犵殑绋嬪簭绠€鍗曞緢澶氥€?
## XSKMAP / BPF_MAP_TYPE_XSKMAP


鍦?XDP 渚ф湁涓€涓?BPF map 绫诲瀷 BPF_MAP_TYPE_XSKMAP锛圶SKMAP锛夛紝瀹冧笌 bpf_redirect_map()
閰嶅悎浣跨敤锛屽皢鍏ュ彛甯т紶閫掔粰涓€涓鎺ュ瓧銆?
鐢ㄦ埛搴旂敤绋嬪簭閫氳繃 bpf() 绯荤粺璋冪敤灏嗗鎺ュ瓧鎻掑叆璇?map銆?
璇锋敞鎰忥紝濡傛灉 XDP 绋嬪簭璇曞浘閲嶅畾鍚戝埌涓€涓笌闃熷垪閰嶇疆鍜?netdev 涓嶅尮閰嶇殑濂楁帴瀛楋紝璇ュ抚浼氳
涓㈠純銆備緥濡傦紝涓€涓?AF_XDP 濂楁帴瀛楃粦瀹氬埌 netdev eth0 鍜岄槦鍒?17銆傚彧鏈変负 eth0 鍜岄槦鍒?17
鎵ц鐨?XDP 绋嬪簭鎵嶈兘鎴愬姛灏嗘暟鎹紶缁欒濂楁帴瀛椼€傝鍙傝€冪ず渚嬪簲鐢ㄧ▼搴忥紙samples/bpf/锛変腑鐨?渚嬪瓙銆?
## 閰嶇疆鏍囧織涓庡鎺ュ瓧閫夐」


浠ヤ笅鏄彲鐢ㄤ簬鎺у埗鍜岀洃瑙?AF_XDP 濂楁帴瀛楄涓虹殑鍚勭閰嶇疆鏍囧織銆?
### XDP_COPY 鍜?XDP_ZEROCOPY 缁戝畾鏍囧織


褰撲綘缁戝畾鍒颁竴涓鎺ュ瓧鏃讹紝鍐呮牳浼氶鍏堝皾璇曚娇鐢ㄩ浂鎷疯礉锛坺ero-copy锛夈€傚鏋滀笉鏀寔闆舵嫹璐濓紝瀹冧細
鍥為€€鍒颁娇鐢ㄦ嫹璐濓紙copy锛夋ā寮忥紝鍗虫妸鎵€鏈夋暟鎹寘鎷疯礉鍒扮敤鎴风┖闂淬€備絾濡傛灉浣犳兂寮哄埗浣跨敤鏌愮妯″紡锛?鍙互浣跨敤浠ヤ笅鏍囧織銆傚鏋滀綘鍦?bind 璋冪敤涓紶鍏?XDP_COPY 鏍囧織锛屽唴鏍镐細寮哄埗璇ュ鎺ュ瓧杩涘叆鎷疯礉
妯″紡銆傚鏋滃畠鏃犳硶浣跨敤鎷疯礉妯″紡锛宐ind 璋冪敤灏嗕互閿欒澶辫触銆傜浉鍙嶏紝XDP_ZEROCOPY 鏍囧織浼氬己鍒?濂楁帴瀛楄繘鍏ラ浂鎷疯礉妯″紡锛屽惁鍒欏け璐ャ€?
### XDP_SHARED_UMEM 缁戝畾鏍囧織


璇ユ爣蹇椾娇浣犺兘澶熷皢澶氫釜濂楁帴瀛楃粦瀹氬埌鍚屼竴涓?UMEM銆傚畠閫傜敤浜庣浉鍚岀殑闃熷垪 id 涔嬮棿銆佷笉鍚岄槦鍒?id
涔嬮棿浠ュ強涓嶅悓 netdev / 璁惧涔嬮棿銆傚湪姝ゆā寮忎笅锛屾瘡涓鎺ュ瓧鐓у父鎷ユ湁鑷繁鐨?RX 鍜?TX 鐜紝浣?浣犱細鎷ユ湁涓€缁勬垨澶氱粍 FILL 鍜?COMPLETION 鐜銆備綘蹇呴』涓轰綘缁戝畾鍒扮殑姣忎釜鍞竴鐨?netdev 鍜?闃熷垪 id 鍏冪粍鍒涘缓杩欐牱涓€缁勩€?
鍏堜粠鎴戜滑甯屾湜鍦ㄧ粦瀹氬埌鐩稿悓 netdev 鍜岄槦鍒?id 鐨勫鎺ュ瓧涔嬮棿鍏变韩 UMEM 鐨勬儏鍐佃璧枫€俇MEM锛堢粦瀹?鍒扮涓€涓垱寤虹殑濂楁帴瀛楋級灏嗗彧鏈変竴涓?FILL 鐜拰涓€涓?COMPLETION 鐜紝鍥犱负鎴戜滑宸茬粡缁戝畾鐨勫敮涓€
netdev銆乹ueue_id 鍏冪粍鍙湁涓€涓€傝浣跨敤姝ゆā寮忥紝鍒涘缓绗竴涓鎺ュ瓧骞朵互甯歌鏂瑰紡缁戝畾瀹冦€?鍒涘缓绗簩涓鎺ュ瓧骞跺垱寤?RX 鍜?TX 鐜紙鎴栬嚦灏戝叾涓箣涓€锛夛紝浣嗕笉瑕佸垱寤?FILL 鎴?COMPLETION
鐜紝鍥犱负灏嗕娇鐢ㄧ涓€涓鎺ュ瓧鐨勯偅浜涖€傚湪 bind 璋冪敤涓紝璁剧疆 XDP_SHARED_UMEM 閫夐」锛屽苟鍦?sxdp_shared_umem_fd 瀛楁涓彁渚涘垵濮嬪鎺ュ瓧鐨?fd銆備綘鍙互浠ヨ繖绉嶆柟寮忛檮鍔犱换鎰忔暟閲忕殑棰濆
濂楁帴瀛椼€?
閭ｄ箞鏁版嵁鍖呬細鍒拌揪鍝釜濂楁帴瀛楀憿锛熻繖鐢?XDP 绋嬪簭鍐冲畾銆傛妸鎵€鏈夊鎺ュ瓧鏀惧叆 XSK_MAP锛屽苟鎸囨槑浣犳兂
鎶婃瘡涓暟鎹寘鍙戦€佸埌鏁扮粍涓殑鍝釜绱㈠紩銆備笅闈㈠睍绀轰簡涓€涓畝鍗曠殑杞锛坮ound-robin锛夊垎鍙戞暟鎹寘
绀轰緥锛?

   #include <linux/bpf.h>
   #include "bpf_helpers.h"

   #define MAX_SOCKS 16

   struct {
       __uint(type, BPF_MAP_TYPE_XSKMAP);
       __uint(max_entries, MAX_SOCKS);
       __uint(key_size, sizeof(int));
       __uint(value_size, sizeof(int));
   } xsks_map SEC(".maps");

   static unsigned int rr;

   SEC("xdp_sock") int xdp_sock_prog(struct xdp_md *ctx)
   {
       rr = (rr + 1) & (MAX_SOCKS - 1);

       return bpf_redirect_map(&xsks_map, rr, XDP_DROP);
   }

璇锋敞鎰忥紝鐢变簬鍙湁涓€缁?FILL 鍜?COMPLETION 鐜紝鑰屼笖瀹冧滑鏄崟鐢熶骇鑰呫€佸崟娑堣垂鑰呯幆锛屼綘闇€瑕?纭繚澶氫釜杩涚▼鎴栫嚎绋嬩笉浼氬苟鍙戜娇鐢ㄨ繖浜涚幆銆俵ibbpf 浠ｇ爜鐩墠娌℃湁浠讳綍鍚屾鍘熻鏉ヤ繚鎶ゅ涓敤鎴枫€?
濡傛灉浣犲垱寤哄涓粦瀹氬埌鍚屼竴涓?UMEM 鐨勫鎺ュ瓧锛宭ibbpf 浼氫娇鐢ㄦ妯″紡銆備絾璇锋敞鎰忥紝浣犻渶瑕佸湪
xsk_socket__create 璋冪敤涓彁渚?XSK_LIBBPF_FLAGS__INHIBIT_PROG_LOAD libbpf_flag锛屽苟鍔犺浇
浣犺嚜宸辩殑 XDP 绋嬪簭锛屽洜涓?libbpf 涓病鏈夊唴缃殑銆佸彲涓轰綘璺敱娴侀噺鐨勭▼搴忋€?
绗簩绉嶆儏鍐垫槸浣犲湪缁戝畾鍒颁笉鍚岄槦鍒?id 鍜?/ 鎴栦笉鍚?netdev 鐨勫鎺ュ瓧涔嬮棿鍏变韩 UMEM銆傚湪杩欑
鎯呭喌涓嬶紝浣犲繀椤讳负姣忎釜鍞竴鐨?netdev銆乹ueue_id 瀵瑰垱寤轰竴涓?FILL 鐜拰涓€涓?COMPLETION 鐜€?鍋囪浣犳兂鍒涘缓涓や釜缁戝畾鍒板悓涓€ netdev 涓婁笉鍚岄槦鍒?id 鐨勫鎺ュ瓧銆傚垱寤虹涓€涓鎺ュ瓧骞朵互甯歌鏂瑰紡
缁戝畾瀹冦€傚垱寤虹浜屼釜濂楁帴瀛楀苟鍒涘缓 RX 鍜?TX 鐜紙鎴栬嚦灏戝叾涓箣涓€锛夛紝鐒跺悗涓鸿繖涓鎺ュ瓧鍒涘缓涓€涓?FILL 鍜?COMPLETION 鐜€傜劧鍚庡湪 bind 璋冪敤涓紝璁剧疆 XDP_SHARED_UMEM 閫夐」锛屽苟鍦?sxdp_shared_umem_fd 瀛楁涓彁渚涘垵濮嬪鎺ュ瓧鐨?fd锛堝洜涓轰綘鍦ㄨ濂楁帴瀛椾笂娉ㄥ唽浜?UMEM锛夈€傝繖涓や釜
濂楁帴瀛楃幇鍦ㄥ皢鍏变韩鍚屼竴涓?UMEM銆?
涓嶉渶瑕佸儚鍓嶉潰濂楁帴瀛楃粦瀹氬埌鐩稿悓闃熷垪 id 鍜岃澶囩殑鎯呭舰閭ｆ牱鎻愪緵 XDP 绋嬪簭銆傜浉鍙嶏紝浣跨敤 NIC 鐨?鏁版嵁鍖呭鍚戯紙packet steering锛夎兘鍔涘皢鏁版嵁鍖呭鍚戞纭殑闃熷垪銆傚湪鍓嶉潰鐨勪緥瀛愪腑锛屽鎺ュ瓧涔嬮棿鍙?鍏变韩涓€涓槦鍒楋紝鎵€浠?NIC 鏃犳硶杩涜杩欑瀵煎悜銆傚畠鍙兘鍦ㄩ槦鍒椾箣闂村仛瀵煎悜銆?
鍦?libbpf 涓紝浣犻渶瑕佷娇鐢?xsk_socket__create_shared() API锛屽洜涓哄畠鎺ュ彈涓€涓?FILL 鐜拰
涓€涓?COMPLETION 鐜殑寮曠敤锛岃繖涓や釜鐜細涓轰綘鍒涘缓骞剁粦瀹氬埌鍏变韩 UMEM銆備綘鍙互瀵瑰垱寤虹殑鎵€鏈夊鎺ュ瓧
閮戒娇鐢ㄨ繖涓嚱鏁帮紝涔熷彲浠ュ彧瀵圭浜屼釜鍙婁箣鍚庣殑濂楁帴瀛椾娇鐢ㄥ畠锛岃€屽绗竴涓鎺ュ瓧浣跨敤
xsk_socket__create()銆備袱绉嶆柟娉曞緱鍒扮浉鍚岀殑缁撴灉銆?
璇锋敞鎰忥紝UMEM 鍙互鍦ㄧ浉鍚岄槦鍒?id 鍜岃澶囩殑濂楁帴瀛椾箣闂村叡浜紝涔熷彲浠ュ悓鏃跺湪鐩稿悓璁惧鐨勪笉鍚岄槦鍒?涔嬮棿浠ュ強涓嶅悓璁惧涔嬮棿鍏变韩銆?
### XDP_USE_NEED_WAKEUP 缁戝畾鏍囧織


璇ラ€夐」鏂板浜嗗涓€涓悕涓?need_wakeup 鐨勬柊鏍囧織鐨勬敮鎸侊紝瀹冨瓨鍦ㄤ簬 FILL 鐜拰 TX 鐜腑锛堢敤鎴风┖闂?浣滀负鐢熶骇鑰呯殑閭ｄ簺鐜級銆傚綋鍦?bind 璋冪敤涓缃閫夐」鏃讹紝濡傛灉鍐呮牳闇€瑕佽绯荤粺璋冪敤鏄惧紡鍞ら啋鎵嶈兘
缁х画澶勭悊鏁版嵁鍖咃紝need_wakeup 鏍囧織浼氳缃綅銆傚鏋滆鏍囧織涓洪浂锛屽垯涓嶉渶瑕佺郴缁熻皟鐢ㄣ€?
濡傛灉 FILL 鐜笂璁剧疆浜嗚鏍囧織锛屽簲鐢ㄧ▼搴忛渶瑕佽皟鐢?poll() 鎵嶈兘缁х画鍦?RX 鐜笂鎺ユ敹鏁版嵁鍖呫€備緥濡傦紝
褰撳唴鏍告娴嬪埌 FILL 鐜笂宸叉病鏈夌紦鍐插尯銆丯IC 鐨?RX HW 鐜笂涔熸病鏈夊墿浣欑紦鍐插尯鏃讹紝灏变細鍙戠敓杩欑
鎯呭喌銆傛鏃朵腑鏂鍏抽棴锛屽洜涓?NIC 鏃犳硶鎺ユ敹浠讳綍鏁版嵁鍖咃紙鍥犱负娌℃湁缂撳啿鍖哄彲鏀惧叆锛夛紝浜庢槸璁剧疆
need_wakeup 鏍囧織锛屼互渚跨敤鎴风┖闂村彲浠ユ妸缂撳啿鍖烘斁鍒?FILL 鐜笂锛岀劧鍚庤皟鐢?poll()锛岃鍐呮牳椹卞姩鎶?杩欎簺缂撳啿鍖烘斁鍒?HW 鐜笂骞跺紑濮嬫帴鏀舵暟鎹寘銆?
濡傛灉 TX 鐜笂璁剧疆浜嗚鏍囧織锛屾剰鍛崇潃搴旂敤绋嬪簭闇€瑕佹樉寮忛€氱煡鍐呮牳鍙戦€佹斁鍒?TX 鐜笂鐨勪换浣曟暟鎹寘銆?杩欏彲浠ラ€氳繃 poll() 璋冪敤锛堝鍚?RX 璺緞閭ｆ牱锛夋垨璋冪敤 sendto() 鏉ュ畬鎴愩€?
TX 璺緞涓娇鐢?libbpf 杈呭姪鍑芥暟鐨勪竴涓ず渚嬪涓嬶細


   if (xsk_ring_prod__needs_wakeup(&my_tx_ring))
       sendto(xsk_socket__fd(xsk_handle), NULL, 0, MSG_DONTWAIT, NULL, 0);

涔熷氨鏄锛屼粎褰撹鏍囧織琚疆浣嶆椂鎵嶄娇鐢ㄧ郴缁熻皟鐢ㄣ€?
鎴戜滑寤鸿浣犲缁堝惎鐢ㄦ妯″紡锛屽洜涓哄畠閫氬父甯︽潵鏇村ソ鐨勬€ц兘锛岀壒鍒槸鍦ㄥ簲鐢ㄧ▼搴忓拰椹卞姩杩愯鍦ㄥ悓涓€涓?鏍镐笂鏃讹紱鍗充究搴旂敤绋嬪簭鍜屽唴鏍搁┍鍔ㄤ娇鐢ㄤ笉鍚岀殑鏍革紝瀹冧篃浼氬噺灏?TX 璺緞鎵€闇€鐨勭郴缁熻皟鐢ㄦ暟閲忋€?
### XDP_{RX|TX|UMEM_FILL|UMEM_COMPLETION}_RING setsockopts


杩欎簺 setsockopt 鍒嗗埆璁剧疆 RX銆乀X銆丗ILL 鍜?COMPLETION 鐜簲褰撴嫢鏈夌殑鎻忚堪绗︽暟閲忋€傚繀椤昏缃?RX 鍜?TX 鐜腑鑷冲皯涓€涓殑澶у皬銆傚鏋滀袱鑰呴兘璁剧疆锛屼綘鐨勫簲鐢ㄧ▼搴忓皢鏃㈣兘鎺ユ敹涔熻兘鍙戦€佹祦閲忥紱浣嗗鏋滀綘
鍙兂鍋氬叾涓箣涓€锛屽彲浠ュ彧寤虹珛鍏朵腑涓€涓潵鑺傜渷璧勬簮銆侳ILL 鐜拰 COMPLETION 鐜兘鏄繀闇€鐨勶紝鍥犱负浣?闇€瑕佷竴涓粦瀹氬埌濂楁帴瀛楃殑 UMEM銆備絾濡傛灉浣跨敤浜?XDP_SHARED_UMEM 鏍囧織锛岀涓€涓箣鍚庣殑浠讳綍濂楁帴瀛?閮芥病鏈?UMEM锛岃繖绉嶆儏鍐典笅灏变笉搴旇鍒涘缓浠讳綍 FILL 鎴?COMPLETION 鐜紝鍥犱负灏嗕娇鐢ㄦ潵鑷叡浜?UMEM
鐨勯偅浜涖€傝娉ㄦ剰锛岃繖浜涚幆鏄崟鐢熶骇鑰呭崟娑堣垂鑰呯殑锛屾墍浠ヤ笉瑕佸皾璇曞悓鏃朵粠澶氫釜杩涚▼璁块棶瀹冧滑銆傚弬瑙?XDP_SHARED_UMEM 涓€鑺傘€?
鍦?libbpf 涓紝浣犲彲浠ュ垎鍒皢 NULL 浼犵粰 xsk_socket__create 鍑芥暟鐨?rx 鍜?tx 鍙傛暟锛屾潵鍒涘缓
鍙帴鏀讹紙Rx-only锛夊拰鍙彂閫侊紙Tx-only锛夌殑濂楁帴瀛椼€?
濡傛灉浣犲垱寤轰簡鍙彂閫佺殑濂楁帴瀛楋紝鎴戜滑寤鸿浣犱笉瑕佸湪 fill 鐜笂鏀句换浣曟暟鎹寘銆傚鏋滆繖鏍峰仛锛岄┍鍔ㄥ彲鑳?浼氳涓轰綘灏嗘帴鏀舵煇浜涗笢瑗匡紙鑰屽疄闄呬笂浣犱笉浼氾級锛岃繖浼氬鎬ц兘浜х敓璐熼潰褰卞搷銆?
### XDP_UMEM_REG setsockopt


璇?setsockopt 灏嗕竴涓?UMEM 娉ㄥ唽鍒板鎺ュ瓧銆傝繖鏄寘鍚墍鏈夊彲瀹圭撼鏁版嵁鍖呯殑缂撳啿鍖虹殑鍖哄煙銆傝璋冪敤
鎺ュ彈涓€涓寚鍚戣鍖哄煙璧峰浣嶇疆鐨勬寚閽堜互鍙婂畠鐨勫ぇ灏忋€傛澶栵紝瀹冭繕鏈変竴涓悕涓?chunk_size 鐨勫弬鏁帮紝
琛ㄧず UMEM 琚垝鍒嗘垚鐨勫潡澶у皬銆傜洰鍓嶅畠鍙兘鏄?2K 鎴?4K銆傚鏋滀綘鏈変竴涓?128K 鐨?UMEM 鍖哄煙鍜?2K
鐨勫潡澶у皬锛岃繖鎰忓懗鐫€浣犵殑 UMEM 鍖哄煙鏈€澶氬彲瀹圭撼 128K / 2K = 64 涓暟鎹寘锛屼笖浣犵殑鏈€澶ф暟鎹寘
澶у皬鍙互鏄?2K銆?
杩樻湁涓€涓€夐」鍙互璁剧疆 UMEM 涓瘡涓紦鍐插尯鐨?headroom銆傚鏋滀綘灏嗗叾璁句负 N 瀛楄妭锛屾剰鍛崇潃鏁版嵁鍖呭皢
浠庣紦鍐插尯绗?N 瀛楄妭澶勫紑濮嬶紝鐣欎笅鍓?N 瀛楄妭渚涘簲鐢ㄧ▼搴忎娇鐢ㄣ€傛渶鍚庝竴涓€夐」鏄?flags 瀛楁锛屼絾灏?閽堝姣忎釜 UMEM 鏍囧織鍦ㄥ崟鐙殑绔犺妭涓鏄庛€?
### SO_BINDTODEVICE setsockopt


杩欐槸涓€涓€氱敤鐨?SOL_SOCKET 閫夐」锛屽彲鐢ㄤ簬灏?AF_XDP 濂楁帴瀛楃粦瀹氬埌鐗瑰畾鐨勭綉缁滄帴鍙ｃ€傚綋濂楁帴瀛楃敱
鐗规潈杩涚▼鍒涘缓骞朵紶閫掔粰闈炵壒鏉冭繘绋嬫椂锛屽畠寰堟湁鐢ㄣ€備竴鏃﹁缃簡璇ラ€夐」锛屽唴鏍稿皢鎷掔粷灏嗚濂楁帴瀛楃粦瀹氬埌
涓嶅悓鎺ュ彛鐨勫皾璇曘€傛洿鏂拌鍊奸渶瑕?CAP_NET_RAW銆?
### XDP_MAX_TX_SKB_BUDGET setsockopt


璇?setsockopt 璁剧疆鍦ㄤ竴娆?send 绯荤粺璋冪敤涓彲浠ュ鐞嗗苟浼犵粰椹卞姩鐨勬弿杩扮鏈€澶ф暟閲忋€傚畠搴旂敤浜?鎷疯礉妯″紡锛岀敤浜庤搴旂敤绋嬪簭璋冧紭姣忎釜濂楁帴瀛楃殑鏈€澶ц凯浠ｆ鏁帮紝浠ヨ幏寰楁洿濂界殑鍚炲悙閲忓苟闄嶄綆 send 绯荤粺
璋冪敤鐨勯鐜囥€傚厑璁哥殑鑼冨洿鏄?[32, xs->tx->nentries]銆?
### XDP_STATISTICS getsockopt


鑾峰彇濂楁帴瀛楃殑涓㈠純缁熻淇℃伅锛屽彲鐢ㄤ簬璋冭瘯鐩殑銆傛敮鎸佺殑缁熻淇℃伅濡備笅鎵€绀猴細


   struct xdp_statistics {
       __u64 rx_dropped; /** Dropped for reasons other than invalid desc **/
       __u64 rx_invalid_descs; /** Dropped due to invalid descriptor **/
       __u64 tx_invalid_descs; /** Dropped due to invalid descriptor **/
   };

### XDP_OPTIONS getsockopt


浠?XDP 濂楁帴瀛楄幏鍙栭€夐」銆傜洰鍓嶅敮涓€鏀寔鐨勬槸 XDP_OPTIONS_ZEROCOPY锛屽畠鍛婅瘔浣犳槸鍚﹀紑鍚簡闆舵嫹璐濄€?
## 澶氱紦鍐插尯鏀寔


鍊熷姪澶氱紦鍐插尯鏀寔锛屼娇鐢?AF_XDP 濂楁帴瀛楃殑绋嬪簭鍙互鍦ㄦ嫹璐濇ā寮忓拰闆舵嫹璐濇ā寮忎笅鎺ユ敹鍜屽彂閫佺敱澶氫釜
缂撳啿鍖虹粍鎴愮殑鏁版嵁鍖呫€備緥濡傦紝涓€涓暟鎹寘鍙互鐢变袱涓抚 / 缂撳啿鍖虹粍鎴愶紝涓€涓寘鍚ご閮ㄣ€佸彟涓€涓寘鍚?鏁版嵁锛涙垨鑰呬竴涓?9K 鐨勪互澶綉宸ㄥ瀷甯э紙jumbo frame锛夊彲浠ラ€氳繃灏嗕笁涓?4K 甯ч摼鎺ヨ捣鏉ユ瀯閫犮€?
涓€浜涘畾涔夛細

- 涓€涓暟鎹寘鐢变竴涓垨澶氫釜甯х粍鎴?
- 鏌愪釜 AF_XDP 鐜腑鐨勬弿杩扮鎬绘槸寮曠敤鍗曚釜甯с€傚綋鏁版嵁鍖呯敱鍗曚釜甯х粍鎴愭椂锛岃鎻忚堪绗﹀紩鐢ㄦ暣涓?  鏁版嵁鍖呫€?
瑕佷负 AF_XDP 濂楁帴瀛楀惎鐢ㄥ缂撳啿鍖烘敮鎸侊紝璇蜂娇鐢ㄦ柊鐨勭粦瀹氭爣蹇?XDP_USE_SG銆傚鏋滀笉鎻愪緵瀹冿紝鎵€鏈?澶氱紦鍐插尯鏁版嵁鍖呴兘浼氬儚浠ュ墠涓€鏍疯涓㈠純銆傝娉ㄦ剰锛屽姞杞界殑 XDP 绋嬪簭涔熼渶瑕佸浜庡缂撳啿鍖烘ā寮忋€傝繖
鍙互閫氳繃鎶?"xdp.frags" 鐢ㄤ綔鎵€鐢?XDP 绋嬪簭鐨勬锛坰ection锛夊悕鏉ュ疄鐜般€?
涓轰簡琛ㄧず涓€涓敱澶氫釜甯х粍鎴愮殑鏁版嵁鍖咃紝鍦?Rx 鍜?Tx 鎻忚堪绗︾殑 options 瀛楁涓紩鍏ヤ簡涓€涓悕涓?XDP_PKT_CONTD 鐨勬柊鏍囧織銆傚鏋滃畠涓虹湡锛?锛夛紝琛ㄧず鏁版嵁鍖呭欢缁埌涓嬩竴涓弿杩扮锛涘鏋滀负鍋囷紙0锛夛紝
琛ㄧず杩欐槸鏁版嵁鍖呯殑鏈€鍚庝竴涓弿杩扮銆備负浠€涔堥噰鐢ㄨ澶?NIC 涓寘缁撴潫锛坋op锛夋爣蹇楃殑鍙嶅悜閫昏緫锛熶粎浠?鏄负浜嗕笌澶氱紦鍐插尯搴旂敤绋嬪簭淇濇寔鍏煎鈥斺€旈偅浜涘簲鐢ㄧ▼搴忓湪 Rx 涓婃妸璇ヤ綅璁句负鍋囷紝骞跺湪 Tx 涓婃妸 options
瀛楁璁句负闆讹紝鍥犱负鍏朵粬浠讳綍鍊奸兘浼氳瑙嗕负鏃犳晥鎻忚堪绗︺€?
浠ヤ笅鏄皢鐢卞涓抚缁勬垚鐨勬暟鎹寘鐢熶骇锛坧roduce锛夊埌 AF_XDP Tx 鐜椂鐨勮涔夛細

- 褰撳彂鐜颁竴涓棤鏁堟弿杩扮鏃讹紝璇ユ暟鎹寘鐨勬墍鏈夊叾浠栨弿杩扮 / 甯ч兘浼氳鏍囪涓烘棤鏁堜笖涓嶅畬鎴愩€備笅涓€涓?  鎻忚堪绗︿細琚綋浣滀竴涓柊鏁版嵁鍖呯殑寮€濮嬶紝鍗充究杩欏苟闈炴湰鎰忥紙鍥犱负鎴戜滑鏃犳硶鐚滄祴鏈剰锛夈€傚拰浠ュ墠涓€鏍凤紝
  濡傛灉浣犵殑绋嬪簭姝ｅ湪鐢熶骇鏃犳晥鎻忚堪绗︼紝璇存槑浣犳湁涓€涓繀椤讳慨澶嶇殑 bug銆?
- 闆堕暱搴︽弿杩扮琚涓烘棤鏁堟弿杩扮銆?
- 瀵逛簬鎷疯礉妯″紡锛屼竴涓暟鎹寘鏀寔鐨勫抚鏈€澶ф暟閲忕瓑浜?CONFIG_MAX_SKB_FRAGS + 1銆傚鏋滆秴鍑猴紝
  鍒扮洰鍓嶄负姝㈢疮绉殑鎵€鏈夋弿杩扮閮戒細琚涪寮冨苟瑙嗕负鏃犳晥銆傝缂栧啓涓€涓彲鍦ㄤ换浣曠郴缁熶笂杩愯銆佷笉鍙楄
  閰嶇疆璁剧疆褰卞搷鐨勫簲鐢ㄧ▼搴忥紝璇峰皢 frags 鏁伴噺闄愬埗涓?18锛屽洜涓鸿閰嶇疆鐨勬渶灏忓€兼槸 17銆?
- 瀵逛簬闆舵嫹璐濇ā寮忥紝涓婇檺鍙栧喅浜?NIC 纭欢鏀寔鐨勭▼搴︺€傚湪鎴戜滑妫€鏌ヨ繃鐨?NIC 涓婇€氬父鑷冲皯鏈変簲涓€?  鎴戜滑鍒绘剰閫夋嫨涓嶄负闆舵嫹璐濇ā寮忓己鍒朵竴涓浐瀹氱殑涓婇檺锛堜緥濡?CONFIG_MAX_SKB_FRAGS + 1锛夛紝鍥犱负閭?  浼氬鑷村湪搴曞眰杩涜鎷疯礉浠ラ€傚簲璇?NIC 鏀寔鐨勪笂闄愩€傝繖鏈夎繚闆舵嫹璐濇ā寮忕殑鐩殑銆傚浣曟帰娴嬭涓婇檺灏嗗湪
  鈥滄帰娴嬪缂撳啿鍖烘敮鎸佲€濅竴鑺備腑璇存槑銆?
鍦ㄦ嫹璐濇ā寮忕殑 Rx 璺緞涓婏紝xsk 鏍稿績浼氬湪闇€瑕佹椂鎶?XDP 鏁版嵁澶嶅埗鍒板涓弿杩扮锛屽苟鎸夊墠杩拌缃?XDP_PKT_CONTD 鏍囧織銆傞浂鎷疯礉妯″紡宸ヤ綔鏂瑰紡鐩稿悓锛屽彧鏄暟鎹笉琚鍒躲€傚綋搴旂敤绋嬪簭鎷垮埌涓€涓?XDP_PKT_CONTD 鏍囧織璁句负 1 鐨勬弿杩扮鏃讹紝鎰忓懗鐫€璇ユ暟鎹寘鐢卞涓紦鍐插尯缁勬垚锛屽苟寤剁画鍒颁笅涓€涓?鎻忚堪绗︿腑鐨勪笅涓€涓紦鍐插尯銆傚綋鏀跺埌涓€涓?XDP_PKT_CONTD == 0 鐨勬弿杩扮鏃讹紝鎰忓懗鐫€杩欐槸璇ユ暟鎹寘鐨?鏈€鍚庝竴涓紦鍐插尯銆侫F_XDP 淇濊瘉鍙妸瀹屾暣鐨勬暟鎹寘锛堟暟鎹寘涓殑鎵€鏈夊抚锛夊彂閫佺粰搴旂敤绋嬪簭銆傚鏋?AF_XDP 鐨?Rx 鐜腑娌℃湁瓒冲绌洪棿锛岃鏁版嵁鍖呯殑鎵€鏈夊抚閮藉皢琚涪寮冦€?
濡傛灉搴旂敤绋嬪簭璇诲彇涓€鎵规弿杩扮锛堜緥濡備娇鐢?libxdp 鎺ュ彛锛夛紝涓嶈兘淇濊瘉杩欐壒鎻忚堪绗︿細浠ヤ竴涓畬鏁寸殑鏁版嵁鍖?缁撴潫銆傚畠鍙兘鍦ㄤ竴涓暟鎹寘鐨勪腑闂寸粨鏉燂紝璇ユ暟鎹寘鐨勫叾浣欑紦鍐插尯浼氬湪涓嬩竴鎵圭殑寮€澶村埌杈撅紝鍥犱负 libxdp
鎺ュ彛涓嶄細璇诲彇鏁翠釜鐜紙闄ら潪浣犳湁鏋佸ぇ鐨勬壒澶у皬鎴栨瀬灏忕殑鐜ぇ灏忥級銆?
閽堝 Rx 鍜?Tx 澶氱紦鍐插尯鏀寔鐨勭ず渚嬬▼搴忓彲鍦ㄦ湰鏂囨。鍚庨潰鎵惧埌銆?
### 鐢ㄦ硶


瑕佷娇鐢?AF_XDP 濂楁帴瀛楋紝闇€瑕佷袱涓儴鍒嗭細鐢ㄦ埛绌洪棿搴旂敤绋嬪簭鍜?XDP 绋嬪簭銆傚叧浜庡畬鏁寸殑寤虹珛鍜屼娇鐢?绀轰緥锛岃鍙傝€?xdp-project锛?https://github.com/xdp-project/bpf-examples/tree/main/AF_XDP-example銆?
XDP 浠ｇ爜绀轰緥浠ｇ爜濡備笅锛?

   SEC("xdp_sock") int xdp_sock_prog(struct xdp_md *ctx)
   {
       int index = ctx->rx_queue_index;

       // A set entry here means that the corresponding queue_id
       // has an active AF_XDP socket bound to it.
       if (bpf_map_lookup_elem(&xsks_map, &index))
           return bpf_redirect_map(&xsks_map, index, 0);

       return XDP_PASS;
   }

涓€涓畝鍗曚絾鎬ц兘骞朵笉楂樼殑鐜嚭闃燂紙dequeue锛夊拰鍏ラ槦锛坋nqueue锛夊彲鑳藉涓嬫墍绀猴細


    // struct xdp_rxtx_ring {
    //     __u32 *producer;
    //     __u32 *consumer;
    //     struct xdp_desc *desc;
    // };

    // struct xdp_umem_ring {
    //     __u32 *producer;
    //     __u32 *consumer;
    //     __u64 *desc;
    // };

    // typedef struct xdp_rxtx_ring RING;
    // typedef struct xdp_umem_ring RING;

    // typedef struct xdp_desc RING_TYPE;
    // typedef __u64 RING_TYPE;

    int dequeue_one(RING **ring, RING_TYPE **item)
    {
        __u32 entries = **ring->producer - **ring->consumer;

        if (entries == 0)
            return -1;

        // read-barrier!

        **item = ring->desc[**ring->consumer & (RING_SIZE - 1)];
        (*ring->consumer)++;
        return 0;
    }

    int enqueue_one(RING **ring, const RING_TYPE **item)
    {
        u32 free_entries = RING_SIZE - (**ring->producer - **ring->consumer);

        if (free_entries == 0)
            return -1;

        ring->desc[**ring->producer & (RING_SIZE - 1)] = **item;

        // write-barrier!

        (*ring->producer)++;
        return 0;
    }

浣嗚浣跨敤 libbpf 鐨勫嚱鏁帮紝鍥犱负瀹冧滑缁忚繃浼樺寲涓斿紑绠卞嵆鐢ㄣ€傞偅浼氳浣犵殑鐢熸椿鏇磋交鏉俱€?
### 澶氱紦鍐插尯 Rx 鐢ㄦ硶


涓嬮潰鏄竴涓畝鍗曠殑 Rx 璺緞浼唬鐮佺ず渚嬶紙涓虹畝娲佽捣瑙佷娇鐢ㄤ簡 libxdp 鎺ュ彛锛夈€備负淇濇寔绠€鐭紝鐪佺暐浜?閿欒璺緞锛?

    void rx_packets(struct xsk_socket_info *xsk)
    {
        static bool new_packet = true;
        u32 idx_rx = 0, idx_fq = 0;
        static char *pkt;

        int rcvd = xsk_ring_cons__peek(&xsk->rx, opt_batch_size, &idx_rx);

        xsk_ring_prod__reserve(&xsk->umem->fq, rcvd, &idx_fq);

        for (int i = 0; i < rcvd; i++) {
            struct xdp_desc *desc = xsk_ring_cons__rx_desc(&xsk->rx, idx_rx++);
            char *frag = xsk_umem__get_data(xsk->umem->buffer, desc->addr);
            bool eop = !(desc->options & XDP_PKT_CONTD);

            if (new_packet)
                pkt = frag;
            else
                add_frag_to_pkt(pkt, frag);

            if (eop)
                process_pkt(pkt);

            new_packet = eop;

            *xsk_ring_prod__fill_addr(&xsk->umem->fq, idx_fq++) = desc->addr;
        }

        xsk_ring_prod__submit(&xsk->umem->fq, rcvd);
        xsk_ring_cons__release(&xsk->rx, rcvd);
    }

### 澶氱紦鍐插尯 Tx 鐢ㄦ硶


涓嬮潰鏄竴涓?Tx 璺緞浼唬鐮佺ず渚嬶紙涓虹畝娲佽捣瑙佷娇鐢ㄤ簡 libxdp 鎺ュ彛锛夛紝蹇界暐 umem 澶у皬鏈夐檺杩欎竴鐐癸紝
浠ュ強鎴戜滑鏈€缁堜細鑰楀敖寰呭彂閫佹暟鎹寘杩欎竴鐐广€傚悓鏃跺亣璁?pkts.addr 鎸囧悜 umem 涓殑涓€涓湁鏁堜綅缃細


    void tx_packets(struct xsk_socket_info **xsk, struct pkt **pkts,
                    int batch_size)
    {
        u32 idx, i, pkt_nb = 0;

        xsk_ring_prod__reserve(&xsk->tx, batch_size, &idx);

        for (i = 0; i < batch_size;) {
            u64 addr = pkts[pkt_nb].addr;
            u32 len = pkts[pkt_nb].size;

            do {
                struct xdp_desc *tx_desc;

                tx_desc = xsk_ring_prod__tx_desc(&xsk->tx, idx + i++);
                tx_desc->addr = addr;

                if (len > xsk_frame_size) {
                    tx_desc->len = xsk_frame_size;
                    tx_desc->options = XDP_PKT_CONTD;
                } else {
                    tx_desc->len = len;
                    tx_desc->options = 0;
                    pkt_nb++;
                }
                len -= tx_desc->len;
                addr += xsk_frame_size;

                if (i == batch_size) {
                /* Remember len, addr, pkt_nb for next iteration.
                 - Skipped for simplicity.
                 */
                    break;
                }
            } while (len);
        }

        xsk_ring_prod__submit(&xsk->tx, i);
    }

### 鎺㈡祴澶氱紦鍐插尯鏀寔


瑕佸彂鐜版煇涓┍鍔ㄦ槸鍚﹀湪 SKB 鎴?DRV 妯″紡涓嬫敮鎸佸缂撳啿鍖?AF_XDP锛屽彲浣跨敤 linux/netdev.h 涓?netlink 鐨?XDP_FEATURES 鐗规€ф煡璇?NETDEV_XDP_ACT_RX_SG 鏀寔銆傝繖涓庢煡璇?XDP 澶氱紦鍐插尯鏀寔
浣跨敤鐨勬槸鍚屼竴涓爣蹇椼€傚鏋滄煇涓┍鍔ㄤ腑鐨?XDP 鏀寔澶氱紦鍐插尯锛岄偅涔?AF_XDP 鍦?SKB 鍜?DRV 妯″紡涓?涔熷皢鏀寔瀹冦€?
瑕佸彂鐜版煇涓┍鍔ㄦ槸鍚﹀湪闆舵嫹璐濇ā寮忎笅鏀寔澶氱紦鍐插尯 AF_XDP锛屽彲浣跨敤 XDP_FEATURES 骞跺厛妫€鏌?NETDEV_XDP_ACT_XSK_ZEROCOPY 鏍囧織銆傚鏋滃畠琚疆浣嶏紝鎰忓懗鐫€鑷冲皯鏀寔闆舵嫹璐濓紝浣犲簲璇ュ幓妫€鏌?linux/netdev.h 涓殑 netlink 灞炴€?NETDEV_A_DEV_XDP_ZC_MAX_SEGS銆傚皢杩斿洖涓€涓棤绗﹀彿鏁存暟鍊硷紝
琛ㄧず姝よ澶囧湪闆舵嫹璐濇ā寮忎笅鏀寔鐨勬渶澶?frags 鏁伴噺銆備互涓嬫槸鍙兘鐨勮繑鍥炲€硷細

1锛氳璁惧涓嶆敮鎸侀浂鎷疯礉澶氱紦鍐插尯锛屽洜涓烘渶澶氭敮鎸佷竴涓?fragment 鎰忓懗鐫€鏃犳硶杩涜澶氱紦鍐插尯銆?
>=2锛氳璁惧鍦ㄩ浂鎷疯礉妯″紡涓嬫敮鎸佸缂撳啿鍖恒€傝繑鍥炵殑鏁板瓧琛ㄧず鏀寔鐨勬渶澶?frags 鏁伴噺銆?
鍏充簬濡備綍閫氳繃杩欎簺鎺ュ彛锛堥€氳繃 libbpf锛変娇鐢ㄧ殑绀轰緥锛岃鍙傝€?tools/testing/selftests/bpf/
xskxceiver.c銆?
### 闆舵嫹璐濋┍鍔ㄧ殑澶氱紦鍐插尯鏀寔


闆舵嫹璐濋┍鍔ㄩ€氬父浣跨敤鎵瑰鐞?API 杩涜 Rx 鍜?Tx 澶勭悊銆傝娉ㄦ剰锛孴x 鎵瑰鐞?API 淇濊瘉瀹冧細鎻愪緵涓€鎵?浠ュ畬鏁存暟鎹寘缁撳熬鐨?Tx 鎻忚堪绗︺€傝繖鏄负浜嗕究浜庝负闆舵嫹璐濋┍鍔ㄦ墿灞曞缂撳啿鍖烘敮鎸併€?
## 绀轰緥搴旂敤绋嬪簭


鏈変竴涓悕涓?xdpsock 鐨勫熀鍑嗘祴璇?/ 娴嬭瘯搴旂敤绋嬪簭锛屽彲鍦?https://github.com/xdp-project/bpf-examples/tree/main/AF_XDP-example 鎵惧埌锛屽畠婕旂ず浜嗗浣?灏?AF_XDP 濂楁帴瀛椾笌绉佹湁 UMEM 涓€璧蜂娇鐢ㄣ€傚亣璁句綘鎯宠鏉ヨ嚜绔彛 4242 鐨?UDP 娴侀噺鏈€缁堣繘鍏ラ槦鍒?16锛屾垜浠皢鍦ㄨ闃熷垪涓婂惎鐢?AF_XDP銆傝繖閲屾垜浠娇鐢?ethtool锛?```

      ethtool -N p3p2 rx-flow-hash udp4 fn
      ethtool -N p3p2 flow-type udp4 src-port 4242 dst-port 4242 \
          action 16

```
鍦?XDP_DRV 妯″紡涓嬭繍琛?rxdrop 鍩哄噯娴嬭瘯鍙互杩欐牱鍋氾細
```

      samples/bpf/xdpsock -i p3p2 -q 16 -r -N

```
瀵逛簬 XDP_SKB 妯″紡锛屼娇鐢ㄥ紑鍏?"-S" 浠ｆ浛 "-N"锛屾墍鏈夐€夐」閮藉彲浠ュ儚寰€甯镐竴鏍风敤 "-h" 鏄剧ず銆?
杩欎釜绀轰緥搴旂敤绋嬪簭浣跨敤 libbpf 鏉ョ畝鍖?AF_XDP 鐨勫缓绔嬪拰浣跨敤銆傚鏋滀綘鎯充簡瑙?AF_XDP 鐨勫師濮?uapi
鏄浣曠湡姝ｇ敤浜庡疄鐜版洿楂樼骇鍔熻兘鐨勶紝璇锋煡鐪?tools/testing/selftests/bpf/xsk.[ch] 涓殑
libbpf 浠ｇ爜銆?
## 甯歌闂


闂細鎴戝湪濂楁帴瀛椾笂鐪嬩笉鍒颁换浣曟祦閲忋€傛垜鍋氶敊浜嗕粈涔堬紵

绛旓細褰撶墿鐞?NIC 鐨?netdev 琚垵濮嬪寲鏃讹紝Linux 閫氬父姣忎釜鏍稿垎閰嶄竴瀵?RX 鍜?TX 闃熷垪銆傚洜姝ゅ湪涓€涓?8 鏍哥郴缁熶笂锛屼細鍒嗛厤闃熷垪 id 0 鍒?7锛屾瘡涓牳涓€涓€傚湪 AF_XDP 鐨?bind 璋冪敤鎴?xsk_socket__create
libbpf 鍑芥暟璋冪敤涓紝浣犳寚瀹氫竴涓缁戝畾鐨勭壒瀹氶槦鍒?id锛岃€屼綘鍦ㄨ濂楁帴瀛椾笂鍙兘鏀跺埌娴佸悜璇ラ槦鍒楃殑
娴侀噺銆傛墍浠ュ湪涓婇潰鐨勪緥瀛愪腑锛屽鏋滀綘缁戝畾鍒伴槦鍒?0锛屼綘灏嗙湅涓嶅埌浠讳綍琚垎鍙戝埌闃熷垪 1 鍒?7 鐨勬祦閲忋€?濡傛灉杩愭皵濂斤紝浣犱細鐪嬪埌杩欎簺娴侀噺锛屼絾閫氬父瀹冧滑浼氳惤鍒版煇涓綘娌℃湁缁戝畾鐨勯槦鍒椾笂銆?
鏈夎嫢骞茬鏂规硶鍙互瑙ｅ喅鎶婁綘鎯宠鐨勬祦閲忛€佸埌鎵€缁戝畾闃熷垪 id 鐨勯棶棰樸€傚鏋滀綘鎯崇湅鍒版墍鏈夋祦閲忥紝浣犲彲浠?寮哄埗 netdev 鍙嫢鏈?1 涓槦鍒楋紝闃熷垪
```

     sudo ethtool -L <interface> combined 1

    If you want to only see part of the traffic, you can program the
    NIC through ethtool to filter out your traffic to a single queue id
    that you can bind your XDP socket to. Here is one example in which
    UDP traffic to and from port 4242 are sent to queue 2::

      sudo ethtool -N <interface> rx-flow-hash udp4 fn
      sudo ethtool -N <interface> flow-type udp4 src-port 4242 dst-port \
      4242 action 2

    A number of other ways are possible all up to the capabilities of
    the NIC you have.

```

闂細鎴戝彲浠ヤ娇鐢?XSKMAP 鍦ㄦ嫹璐濇ā寮忎笅瀹炵幇涓嶅悓 umem 涔嬮棿鐨勫垏鎹㈠悧锛?
绛旓細绠€鐭殑鍥炵瓟鏄笉琛岋紝鐩墠涓嶆敮鎸併€俋SKMAP 鍙兘鐢ㄤ簬鎶婅繘鍏ラ槦鍒?id X 鐨勬祦閲忓垏鎹㈠埌缁戝畾鍒板悓涓€
闃熷垪 id X 鐨勫鎺ュ瓧銆俋SKMAP 鍙互鍖呭惈缁戝畾鍒颁笉鍚岄槦鍒?id锛堜緥濡?X 鍜?Y锛夌殑濂楁帴瀛楋紝浣嗗彧鏈夋潵鑷?闃熷垪 id Y 鐨勬祦閲忔墠鑳借瀵煎悜缁戝畾鍒板悓涓€闃熷垪 id Y 鐨勫鎺ュ瓧銆傚湪闆舵嫹璐濇ā寮忎笅锛屼綘搴旇鍦ㄤ綘鐨?NIC
涓娇鐢?switch 鎴栧叾浠栧垎鍙戞満鍒讹紝鎶婃祦閲忓鍚戞纭殑闃熷垪 id 鍜屽鎺ュ瓧銆?
闂細鎴戠殑鏁版嵁鍖呮湁鏃朵細鎹熷潖銆傚摢閲屽嚭閿欎簡锛?
绛旓細蹇呴』灏忓績涓嶈鎶?UMEM 涓殑鍚屼竴涓紦鍐插尯鍚屾椂鍠傜粰澶氫釜鐜€備緥濡傦紝濡傛灉浣犳妸鍚屼竴涓紦鍐插尯鍚屾椂
鍠傜粰 FILL 鐜拰 TX 鐜紝NIC 鍙兘浼氬湪缂撳啿鍖哄彂閫佹暟鎹殑鍚屾椂鎺ユ敹鏁版嵁鍒拌缂撳啿鍖恒€傝繖浼氬鑷存煇浜?鏁版嵁鍖呮崯鍧忋€傛妸鍚屼竴涓紦鍐插尯鍠傜粰灞炰簬涓嶅悓闃熷垪 id 鎴栫敱 XDP_SHARED_UMEM 鏍囧織缁戝畾鐨勪笉鍚?netdev
鐨?FILL 鐜篃浼氭湁鍚屾牱鐨勯棶棰樸€?
## 鑷磋阿


- Bj枚rn T枚pel (AF_XDP core)
- Magnus Karlsson (AF_XDP core)
- Alexander Duyck
- Alexei Starovoitov
- Daniel Borkmann
- Jesper Dangaard Brouer
- John Fastabend
- Jonathan Corbet (LWN coverage)
- Michael S. Tsirkin
- Qi Z Zhang
- Willem de Bruijn
