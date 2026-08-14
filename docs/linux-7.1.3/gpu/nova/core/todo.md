
## 浠诲姟娓呭崟


浠诲姟鍙兘鍖呭惈浠ヤ笅瀛楁锛?
- `Complexity`锛堝鏉傚害锛夛細鎻忚堪鎵€闇€鐨勫 Rust 鍜?鎴栫浉搴斿唴鏍?API 鎴栧瓙绯荤粺鐨勭啛鎮夌▼搴︺€傚叡鏈夊洓绉嶅鏉傚害锛歚Beginner`锛堝垵绾э級銆乣Intermediate`锛堜腑绾э級銆乣Advanced`锛堥珮绾э級鍜?`Expert`锛堜笓瀹讹級銆?- `Reference`锛堝紩鐢級锛氬鍏朵粬浠诲姟鐨勫紩鐢ㄣ€?- `Link`锛堥摼鎺ワ級锛氬閮ㄨ祫婧愮殑閾炬帴銆?- `Contact`锛堣仈绯讳汉锛夛細鍙互灏变换鍔¤繘涓€姝ヤ俊鎭仈绯荤殑浜哄憳銆?
浠诲姟鍚嶇О鍚庨潰鍙兘甯︽湁 `[ABCD]` 浠ｇ爜銆傝浠ｇ爜鍙敤浜庡湪浠ｇ爜涓?grep 涓庤浠诲姟鐩稿叧鐨?`TODO` 鏉＄洰銆?
## 鍚敤宸ヤ綔锛圧ust锛?

杩欎簺浠诲姟骞堕潪鐩存帴鍏宠仈 nova-core锛岃€屾槸鎵€闇€ API 鏂归潰鐨勫墠鎻愭潯浠躲€?
### FromPrimitive API [FPRI]


鏈夋椂闇€瑕佸皢涓€涓暟瀛楄浆鎹负鏌愪釜鏋氫妇鎴栫粨鏋勭殑鍊笺€?
nova-core 鐨勪竴涓ソ渚嬪瓙鏄?`Chipset` 鏋氫妇绫诲瀷锛屽畠瀹氫箟浜嗗€?`AD102`銆傚湪鎺㈡祴 GPU 鏃讹紝鍙互浠庢煇涓瘎瀛樺櫒璇诲嚭鍊?`0x192`锛岃〃鏄庤姱鐗囩粍涓?AD102銆傚洜姝わ紝鏋氫妇鍊?`AD102` 搴斿綋浠庢暟瀛?`0x192` 鎺ㄥ鑰屾潵銆傜洰鍓嶏紝nova-core 涓烘浣跨敤浜嗚嚜瀹氫箟鐨?*瀹炵幇锛坄Chipset` : from_u32锛?*銆?
鐩告瘮涔嬩笅锛屾洿鐞嗘兂鐨勬槸鎷ユ湁绫讳技 num crate 涓殑 `FromPrimitive` trait [^1^] 杩欐牱鐨勪笢瑗裤€?
鎷ユ湁杩欑娉涘寲杩樻湁鍔╀簬瀹炵幇涓€涓€氱敤瀹忥紝鑷姩鐢熸垚鍊间笌鏁板瓧涔嬮棿鐨勭浉搴旀槧灏勩€?
杩囧幓鏇惧 FromPrimitive 鏀寔鍋氳繃宸ヤ綔锛屼絾姝ゅ悗渚挎病鏈夊啀璺熻繘 [^1^]銆?
涔熻€冭檻杩?ToPrimitive [^2^]銆?
| Complexity: Beginner
| Link: https://docs.rs/num/latest/num/trait.FromPrimitive.html
| Link: https://lore.kernel.org/all/cover.1750689857.git.y.j3ms.n@gmail.com/ [^1^]
| Link: https://rust-for-linux.zulipchat.com/#narrow/channel/288089-General/topic/Implement.20.60FromPrimitive.60.20trait.20.2B.20derive.20macro.20for.20nova-core/with/541971854 [^2^]

### Numerical operations [NUMM]


Nova 浣跨敤浜嗕笉灞炰簬鏍囧噯搴擄紙鎴栨病鏈変负鍐呮牳鍋氫紭鍖栧疄鐜帮級鐨勬暣鏁拌繍绠椼€傝繖浜涘寘鎷細

- "鏌ユ壘鏈€鍚庣疆浣嶆瘮鐗?锛團ind Last Set Bit锛屽唴鏍?C 閮ㄥ垎鐨?`fls` 鍑芥暟锛夎繍绠椼€?
涓€涓?`num` 鏍稿績鍐呮牳妯″潡姝ｅ湪璁捐涓紝鐢ㄤ簬鎻愪緵杩欎簺杩愮畻銆?
| Complexity: Intermediate
| Contact: Alexandre Courbot

### Page abstraction for foreign pages


閽堝骞堕潪鐢?Rust 椤垫娊璞″垱寤恒€佷笖娌℃湁鐩存帴鎵€鏈夋潈鐨勯〉鐨?Rust 鎶借薄銆?
Abdiel Janulgue [^1^] 鍜?Lina [^2^] 姝ｅ湪杩涜绉瀬鐨勫伐浣滐紙active ongoing work锛夈€?
| Complexity: Advanced
| Link: https://lore.kernel.org/linux-mm/20241119112408.779243-1-abdiel.janulgue@gmail.com/ [^1^]
| Link: https://lore.kernel.org/rust-for-linux/20250202-rust-page-v1-0-e3170d7fe55e@asahilina.net/ [^2^]

### PCI MISC APIs


閫氳繃 SR-IOV銆乧apability銆丮SI API 鎶借薄鏉ユ墿灞曠幇鏈夌殑 PCI 璁惧/椹卞姩鎶借薄銆?
SR-IOV [^1^] 姝ｅ湪杩涜涓€?
| Complexity: Beginner
| Link: https://lore.kernel.org/all/20251119-rust-pci-sriov-v1-0-883a94599a97@redhat.com/ [^1^]

## GPU锛堟杩帮級


### Initial Devinit support


瀹炵幇 BIOS 璁惧鍒濆鍖栵紝鍗冲唴瀛樺ぇ灏忕‘瀹氥€佺瓑寰呫€丳LL 閰嶇疆銆?
| Contact: Dave Airlie
| Complexity: Beginner

### MMU / PT management


璁捐 MMU / 椤佃〃绠＄悊鐨勬灦鏋勩€?
鎴戜滑闇€瑕佽€冭檻鍒帮紝nova-drm 闇€瑕佺浉褰撶粏绮掑害鐨勬帶鍒讹紝灏ゅ叾鏄湪閿佹柟闈紝浠ヤ究鑳藉瀹炵幇寮傛 Vulkan 闃熷垪銆?
铏界劧閫氬父鍏变韩鐩稿簲浠ｇ爜鏄悊鎯崇殑锛屼絾闇€瑕佽瘎浼板叡浜浉搴斾唬鐮佹槸鍚︼紙浠ュ強鍦ㄤ綍绉嶇▼搴︿笂锛夊悎閫傘€?
| Complexity: Expert

### VRAM memory allocator


鐮旂┒ VRAM 鍐呭瓨鍒嗛厤鍣ㄧ殑鍚勭閫夐」銆?
涓€浜涘彲鑳界殑閫夐」锛?  - RB 鏍戯紙鍖洪棿鏍戯級/ drm_mm 鐨?Rust 鎶借薄
  - maple_tree
  - 鍘熺敓 Rust 闆嗗悎

浣跨敤 drm_buddy [^1^] 鐨勫伐浣滄鍦ㄨ繘琛屼腑銆?
| Complexity: Advanced
| Link: https://lore.kernel.org/all/20251219203805.1246586-4-joelagnelf@nvidia.com/ [^1^]

### Instance Memory


瀹炵幇瀵圭敤浜庡瓨鍌ㄩ〉琛ㄧ殑 instmem锛坆ar2锛夌殑鏀寔銆?
| Complexity: Intermediate
| Contact: Dave Airlie

## GPU System Processor (GSP)


### Export GSP log buffers


Timur Tabi [^1^] 杩戞湡鐨勮ˉ涓佸鍔犱簡閫氳繃 debugfs 鏆撮湶 GSP-RM 鏃ュ織缂撳啿鍖虹殑鏀寔锛堝嵆浣垮湪椹卞姩鎺㈡祴澶辫触鍚庝篃鑳芥毚闇诧級銆?
杩欏 nova-core 涔熸槸涓€涓湁瓒ｇ殑鐗规€э紝灏ゅ叾鏄湪鏃╂湡闃舵銆?
| Link: https://lore.kernel.org/nouveau/20241030202952.694055-2-ttabi@nvidia.com/ [^1^]
| Reference: Debugfs abstractions
| Complexity: Intermediate

### GSP firmware abstraction


GSP-RM 鍥轰欢 API 涓嶇ǔ瀹氾紝鍦ㄦ暟鎹粨鏋勫拰璇箟鏂归潰鍙兘鍦ㄤ笉鍚岀増鏈箣闂村彂鐢熶笉鍏煎鐨勫彉鍖栥€?
杩欎釜闂鏄?nova-core 浣跨敤 Rust 鐨勪竴澶у姩鏈轰箣涓€锛屽洜涓轰簨瀹炶瘉鏄?Rust 鐨勮繃绋嬪畯锛坧rocedural macro锛夌壒鎬ф彁渚涗簡涓€绉嶇浉褰撲紭闆呯殑鏂瑰紡鏉ヨВ鍐宠繖涓€闂锛?
1. 浠?C 澶存枃浠朵负姣忎釜鐗堟湰鍦ㄧ嫭绔嬬殑鍛藉悕绌洪棿涓敓鎴?Rust 缁撴瀯
2. 鏋勫缓瀹炵幇鍥轰欢鎺ュ彛鐨勬娊璞＄粨鏋勶紙浣嶄簬閫氱敤鍛藉悕绌洪棿鍐咃級锛涚敤鐗堟湰鏍囪瘑绗︽爣娉ㄥ疄鐜扮殑宸紓
3. 浣跨敤杩囩▼瀹忎粠璇ユ娊璞＄敓鎴愬疄闄呯殑姣忎釜鐗堟湰鐨勫疄鐜?4. 鍦ㄨ繍琛屾椂瀹炰緥鍖栨纭殑鐗堟湰绫诲瀷锛堝彲浠ョ‘淇″畠浠兘鏈夌浉鍚岀殑鎺ュ彛锛屽洜涓哄畠鐢卞叕鍏?trait 瀹氫箟锛?
鍦?nova-core PoC 椹卞姩鐨勭幆澧冧腑宸叉湁璇ユā寮忕殑 PoC锛堟蹇甸獙璇侊級瀹炵幇銆?
璇ヤ换鍔℃棬鍦ㄥ畬鍠勮鐗规€э紝骞剁悊鎯虫儏鍐典笅灏嗗叾娉涘寲锛屼互渚垮叾浠栭┍鍔ㄤ篃鑳戒娇鐢ㄣ€?
| Complexity: Expert

### GSP message queue


瀹炵幇搴曞眰鐨?GSP 娑堟伅闃熷垪锛坈ommand銆乻tatus锛夛紝鐢ㄤ簬鍐呮牳椹卞姩涓?GSP 涔嬮棿鐨勯€氫俊銆?
| Complexity: Advanced
| Contact: Dave Airlie

### Bootstrap GSP


璋冪敤寮曞鍥轰欢鏉ュ惎鍔?GSP 澶勭悊鍣紱鎵ц鍒濆鎺у埗娑堟伅銆?
| Complexity: Intermediate
| Contact: Dave Airlie

### Client / Device APIs


瀹炵幇鐢ㄤ簬 client / device 鍒嗛厤鐨?GSP 娑堟伅鎺ュ彛锛屼互鍙婄浉搴旂殑 client 鍜?device 鍒嗛厤 API銆?
| Complexity: Intermediate
| Contact: Dave Airlie

### Bar PDE handling


鍚屾鍐呮牳椹卞姩涓?GSP 涔嬮棿閽堝 BAR 鐨勯〉琛ㄥ鐞嗐€?
| Complexity: Beginner
| Contact: Dave Airlie

### FIFO engine


瀹炵幇瀵?FIFO 寮曟搸鐨勬敮鎸侊紝鍗崇浉搴旂殑 GSP 娑堟伅鎺ュ彛锛屽苟鎻愪緵鐢ㄤ簬 chid 鍒嗛厤鍜岄€氶亾澶勭悊鐨?API銆?
| Complexity: Advanced
| Contact: Dave Airlie

### GR engine


瀹炵幇瀵瑰浘褰㈠紩鎿庣殑鏀寔锛屽嵆鐩稿簲鐨?GSP 娑堟伅鎺ュ彛锛屽苟鎻愪緵鐢ㄤ簬锛坓olden锛変笂涓嬫枃鍒涘缓鍜屾彁鍗囷紙promotion锛夌殑 API銆?
| Complexity: Advanced
| Contact: Dave Airlie

### CE engine


瀹炵幇瀵规嫹璐濆紩鎿庣殑鏀寔锛屽嵆鐩稿簲鐨?GSP 娑堟伅鎺ュ彛銆?
| Complexity: Intermediate
| Contact: Dave Airlie

### VFN IRQ controller


瀵?VFN 涓柇鎺у埗鍣ㄧ殑鏀寔銆?
| Complexity: Intermediate
| Contact: Dave Airlie

## 澶栭儴 API


### nova-core base API


璁捐鐢ㄤ簬杩炴帴浜岀骇椹卞姩锛堝嵆 vGPU 绠＄悊鍣ㄥ拰 nova-drm锛夌殑 API 鍏叡閮ㄥ垎銆?
| Complexity: Advanced

### vGPU manager API


璁捐 base API 鏈鐩栥€佷絾 vGPU 绠＄悊鍣ㄦ墍闇€鐨?API 閮ㄥ垎銆?
| Complexity: Advanced

### nova-core C API


涓?vGPU 绠＄悊鍣ㄩ┍鍔ㄦ墍闇€鐨?API 瀹炵幇 C 鍖呰鍣ㄣ€?
| Complexity: Intermediate

## 娴嬭瘯


### CI pipeline


鐮旂┒鎸佺画闆嗘垚娴嬭瘯鐨勯€夐」銆?
杩欏彲浠ヤ粠鏈€绠€鍗曠殑杩愯 KUnit 娴嬭瘯锛屽埌杩愯锛堝浘褰級CTS锛屽啀鍒板惎鍔紙澶氫釜锛夊鎴锋満 VM 鏉ユ祴璇?VFIO 鐢ㄤ緥銆?
涔熷€煎緱鑰冭檻寮曞叆涓€涓洿鎺ヤ綅浜?uAPI 涔嬩笂鐨勬柊娴嬭瘯濂椾欢锛屼互杩涜鏇存湁閽堝鎬х殑娴嬭瘯鍜岃皟璇曘€傚彲鑳藉瓨鍦ㄤ笌 Mesa 椤圭洰鍗忎綔/鍏变韩浠ｇ爜鐨勯€夐」銆?
| Complexity: Advanced
