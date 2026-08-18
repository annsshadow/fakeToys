## 濡備綍姝ｇ‘浣跨敤 printk 鏍煎紡璇存槑绗?

:Author: Randy Dunlap <rdunlap@infradead.org>
:Author: Andrew Murray <amurray@mpc-data.co.uk>


## 鏁存暟绫诲瀷


```
	If variable is of Type,		use printk format specifier:
	------------------------------------------------------------
		signed char		%d or %hhx
		unsigned char		%u or %x
		char			%u or %x
		short int		%d or %hx
		unsigned short int	%u or %x
		int			%d or %x
		unsigned int		%u or %x
		long			%ld or %lx
		unsigned long		%lu or %lx
		long long		%lld or %llx
		unsigned long long	%llu or %llx
		size_t			%zu or %zx
		ssize_t			%zd or %zx
		s8			%d or %hhx
		u8			%u or %x
		s16			%d or %hx
		u16			%u or %x
		s32			%d or %x
		u32			%u or %x
		s64			%lld or %llx
		u64			%llu or %llx
```

濡傛灉 <type> 鐨勫ぇ灏忎緷璧栦簬浣撶郴缁撴瀯锛堜緥濡?cycles_t銆乼cflag_t锛夛紝鎴栬€呬緷璧栦簬鏌愪釜閰嶇疆閫夐」鏉ュ喅瀹氬ぇ灏忥紙渚嬪 blk_status_t锛夛紝鍒欏簲浣跨敤鍏跺彲鑳界殑鏈€澶х被鍨嬪搴旂殑鏍煎紡璇存槑绗︼紝骞舵樉寮忚浆鎹负璇ョ被鍨嬨€?
```
	printk("test: latency: %llu cycles\n", (unsigned long long)time);
```

鎻愰啋锛歚sizeof()` 杩斿洖鐨勭被鍨嬫槸 size_t銆?
鍐呮牳鐨?printf 涓嶆敮鎸?%n銆傛诞鐐规牸寮忥紙%e銆?f銆?g銆?a锛夊嚭浜庢樉鑰屾槗瑙佺殑鍘熷洜涔熶笉琚瘑鍒€備娇鐢ㄤ换浣曚笉鍙楁敮鎸佺殑璇存槑绗︽垨闀垮害闄愬畾绗﹂兘浼氬鑷翠竴涓?WARN 骞舵彁鏃╀粠 vsnprintf() 杩斿洖銆?
## 鎸囬拡绫诲瀷

鍘熷鐨勬寚閽堝€煎彲浠ョ敤 %p 鎵撳嵃锛屽畠浼氬湪鎵撳嵃鍓嶅鍦板潃杩涜鍝堝笇銆傚唴鏍歌繕鏀寔鐢ㄤ簬鎵撳嵃涓嶅悓绫诲瀷鎸囬拡鐨勬墿灞曡鏄庣銆?
鏌愪簺鎵╁睍璇存槑绗︿細鎵撳嵃缁欏畾鍦板潃涓婄殑鏁版嵁锛岃€屼笉鏄墦鍗板湴鍧€鏈韩銆傚湪杩欑鎯呭喌涓嬶紝浼氬嚭鐜颁互涓嬮敊璇秷鎭細

```
	(null)	 data on plain NULL address
	(efault) data on invalid address
	(einval) invalid data on a valid address
```

### 鏅€氭寚閽?

```
	%p	abcdef12 or 00000000abcdef12
```

涓嶅甫鎵╁睍璇存槑绗︼紙鍗宠８鐨?%p锛夋墦鍗扮殑鎸囬拡浼氳鍝堝笇锛屼互闃叉娉勯湶鍐呮牳鍐呭瓨甯冨眬淇℃伅銆傝繖杩樻湁涓€涓澶栫殑濂藉锛屽氨鏄彁渚涗簡涓€涓敮涓€鏍囪瘑绗︺€傚湪 64 浣嶆満鍣ㄤ笂锛岄珮 32 浣嶈娓呴浂銆傚唴鏍镐細鎵撳嵃 `(ptrval)`锛岀洿鍒板畠鏀堕泦鍒拌冻澶熺殑鐔典负姝€?
灏藉彲鑳戒娇鐢ㄤ笓闂ㄧ殑淇グ绗︼紙濡?%pS 鎴?%pB锛岃涓嬫枃锛夛紝浠ラ伩鍏嶉渶瑕佹彁渚涗簨鍚庢墠鑳借В璇荤殑鏈搱甯屽湴鍧€銆傚鏋滃仛涓嶅埌锛屼笖鎵撳嵃鍦板潃鐨勭洰鐨勬槸涓鸿皟璇曟彁渚涙洿澶氫俊鎭殑锛岄偅涔堝湪璋冭瘯鏈熼棿浣跨敤 %p 骞跺湪鍐呮牳鍚姩鏃跺姞涓?`no_hash_pointers` 鍙傛暟锛屽畠浼氭墦鍗版墍鏈夋湭缁忎慨鏀圭殑 %p 鍦板潃銆傚鏋滀綘**纭疄**濮嬬粓鎯宠鏈慨鏀圭殑鍦板潃锛岃鍙傞槄涓嬮潰鐨?%px銆?
濡傛灉锛堜笖浠呭綋锛変綘鎵撳嵃鍦板潃鏄綔涓鸿櫄鎷熸枃浠讹紙渚嬪 procfs 鎴?sysfs 涓紝浣跨敤 seq_printf() 鑰岄潪 printk() 璇诲嚭锛夌殑鍐呭銆佷緵鐢ㄦ埛绌洪棿杩涚▼璇诲彇锛岃浣跨敤涓嬫枃鎻忚堪鐨?%pK 淇グ绗︼紝鑰屼笉鏄?%p 鎴?%px銆?
### 閿欒鎸囬拡


```
	%pe	-ENOSPC
```

鐢ㄤ簬灏嗛敊璇寚閽堬紙鍗?IS_ERR() 涓虹湡鐨勬寚閽堬級浣滀负绗﹀彿鍖栫殑閿欒鍚嶆墦鍗般€傛病鏈夊凡鐭ョ鍙峰悕鐨勯敊璇€间互鍗佽繘鍒舵墦鍗帮紝鑰屼紶缁?%pe 鐨勯潪 ERR_PTR 浼氳褰撲綔鏅€氱殑 %p 澶勭悊銆?
### 绗﹀彿/鍑芥暟鎸囬拡


```
	%pS	versatile_init+0x0/0x110
	%ps	versatile_init
	%pSR	versatile_init+0x9/0x110
		(with __builtin_extract_return_addr() translation)
	%pB	prev_fn_of_versatile_init+0x88/0x88
```

`S` 鍜?`s` 璇存槑绗︾敤浜庝互绗﹀彿鏍煎紡鎵撳嵃鎸囬拡銆傚畠浠骇鐢熷甫锛圫锛夋垨涓嶅甫锛坰锛夊亸绉婚噺鐨勭鍙峰悕銆傚鏋滅鐢ㄤ簡 KALLSYMS锛屽垯鏀逛负鎵撳嵃绗﹀彿鍦板潃銆?
`B` 璇存槑绗︿骇鐢熷甫鍋忕Щ閲忕殑绗﹀彿鍚嶏紝搴斿湪鎵撳嵃鏍堝洖婧椂浣跨敤銆傝璇存槑绗︿細鑰冭檻鍦ㄤ娇鐢ㄥ熬璋冪敤骞朵互 GCC 鐨?noreturn 灞炴€ф爣璁版椂鍙兘鍙戠敓鐨勭紪璇戝櫒浼樺寲褰卞搷銆?
濡傛灉鎸囬拡浣嶄簬鏌愪釜妯″潡鍐呴儴锛屽垯妯″潡鍚嶄互鍙婂彲閫夌殑 build ID 浼氱揣鎺ュ湪绗﹀彿鍚嶄箣鍚庢墦鍗帮紝骞跺湪璇存槑绗︽湯灏鹃檮鍔犱竴涓?`b`銆?
```
	%pS	versatile_init+0x0/0x110 [module_name]
	%pSb	versatile_init+0x0/0x110 [module_name ed5019fdf5e53be37cb1ba7899292d7e143b259e]
	%pSRb	versatile_init+0x9/0x110 [module_name ed5019fdf5e53be37cb1ba7899292d7e143b259e]
		(with __builtin_extract_return_addr() translation)
	%pBb	prev_fn_of_versatile_init+0x88/0x88 [module_name ed5019fdf5e53be37cb1ba7899292d7e143b259e]
```

### 鏉ヨ嚜 BPF / tracing 鐨勬帰娴嬫寚閽?

```
	%pks	kernel string
	%pus	user string
```

`k` 鍜?`u` 璇存槑绗︾敤浜庢墦鍗板厛鍓嶆帰娴嬪埌鐨勩€佹潵鑷唴鏍稿唴瀛橈紙k锛夋垨鐢ㄦ埛鍐呭瓨锛坲锛夌殑鍐呭瓨銆傚悗缁殑 `s` 璇存槑绗︿細鎵撳嵃涓€涓瓧绗︿覆銆傚湪甯歌 vsnprintf() 涓洿鎺ヤ娇鐢ㄦ椂锛岋紙k锛夊拰锛坲锛夋敞瑙ｄ細琚拷鐣ワ紱浣嗗湪 BPF 鐨?bpf_trace_printk() 涓娇鐢ㄦ椂锛屼緥濡傦紝瀹冧細鍦ㄤ笉瑙﹀彂缂洪〉鐨勬儏鍐典笅璇诲彇鍏舵墍鎸囧悜鐨勫唴瀛樸€?
### 鍐呮牳鎸囬拡


```
	%pK	01234567 or 0123456789abcdef
```

鐢ㄤ簬鎵撳嵃搴斿闈炵壒鏉冪敤鎴烽殣钘忕殑鍐呮牳鎸囬拡銆?pK 鐨勮涓哄彇鍐充簬 kptr_restrict sysctl鈥斺€旀洿澶氱粏鑺傝鍙傞槄 Documentation/admin-guide/sysctl/kernel.rst銆?
姝や慨楗扮**浠?*鐢ㄤ簬浜х敓鐢辩敤鎴风┖闂翠粠 procfs 鎴?sysfs 璇诲彇鐨勬枃浠跺唴瀹癸紝鑰岄潪鐢ㄤ簬 dmesg銆傛湁鍏冲浣曞湪 printk() 涓鐞嗗搱甯屾寚閽堢殑璁ㄨ锛岃鍙傞槄涓婇潰鍏充簬 %p 鐨勭珷鑺傘€?
### 鏈慨鏀圭殑鍦板潃


```
	%px	01234567 or 0123456789abcdef
```

褰撲綘**纭疄**鎯宠鎵撳嵃鍦板潃鏃朵娇鐢ㄣ€傚湪鎵撳嵃涔嬪墠锛岃鑰冭檻鏄惁姝ｅ湪娉勯湶鏈夊叧鍐呮牳鍐呭瓨甯冨眬鐨勬晱鎰熶俊鎭€?px 鍦ㄥ姛鑳戒笂绛変环浜?%lx锛堟垨 %lu锛夈€備箣鎵€浠ヤ紭鍏堥€夋嫨 %px锛屾槸鍥犱负瀹冩洿渚夸簬鐢?grep 鎼滅储銆傚鏋滃皢鏉ユ垜浠渶瑕佷慨鏀瑰唴鏍稿鐞嗘寚閽堟墦鍗扮殑鏂瑰紡锛屾垜浠皢鏇村鏄撴壘鍒拌皟鐢ㄧ偣銆?
鍦ㄤ娇鐢?%px 涔嬪墠锛岃鑰冭檻鏄惁浠呬娇鐢?%p 骞跺湪璋冭瘯浼氳瘽鏈熼棿鍚敤 `no_hash_pointers` 鍐呮牳鍙傛暟锛堣涓婃枃 %p 鐨勬弿杩帮級灏辫冻澶熶簡銆備娇鐢?%px 鐨勪竴涓悎鐞嗗満鏅槸鍦?panic 涔嬪墠绔嬪嵆鎵撳嵃淇℃伅锛岀敱浜?panic 浼氶樆姝换浣曟晱鎰熶俊鎭鍒╃敤锛岃€屼笖浣跨敤 %px 灏辨棤闇€鐢?no_hash_pointers 鏉ュ鐜拌 panic銆?
### 鎸囬拡宸€?

```
	%td	2560
	%tx	a00
```

鐢ㄤ簬鎵撳嵃鎸囬拡宸€硷紝瀵?ptrdiff_t 浣跨敤 %t 淇グ绗︺€?
```
	printk("test: difference between pointers: %td\n", ptr2 - ptr1);
```

### 缁撴瀯浣撹祫婧愶紙struct resources锛?

```
	%pr	[mem 0x60000000-0x6fffffff flags 0x2200] or
		[mem 0x60000000 flags 0x2200] or
		[mem 0x0000000060000000-0x000000006fffffff flags 0x2200]
		[mem 0x0000000060000000 flags 0x2200]
	%pR	[mem 0x60000000-0x6fffffff pref] or
		[mem 0x60000000 pref] or
		[mem 0x0000000060000000-0x000000006fffffff pref]
		[mem 0x0000000060000000 pref]
```

鐢ㄤ簬鎵撳嵃缁撴瀯浣撹祫婧愩€俙R` 鍜?`r` 璇存槑绗︿細浜х敓甯︼紙R锛夋垨涓嶅甫锛坮锛夊凡瑙ｇ爜 flags 鎴愬憳鐨勬墦鍗拌祫婧愩€傚鏋?start 绛変簬 end锛屽垯鍙墦鍗?start 鍊笺€?
閫氳繃寮曠敤浼犻€掋€?
### 鐗╃悊鍦板潃绫诲瀷 phys_addr_t


```
	%pa[p]	0x01234567 or 0x0123456789abcdef
```

鐢ㄤ簬鎵撳嵃 phys_addr_t 绫诲瀷锛堝強鍏惰鐢熺被鍨嬶紝濡?resource_size_t锛夛紝瀹冨彲浠ラ殢鏋勫缓閫夐」鍙樺寲锛岃€屼笌 CPU 鏁版嵁閫氳矾鐨勫搴︽棤鍏炽€?
閫氳繃寮曠敤浼犻€掋€?
### 缁撴瀯浣撹寖鍥达紙struct range锛?

```
	%pra    [range 0x0000000060000000-0x000000006fffffff] or
		[range 0x0000000060000000]
```

鐢ㄤ簬鎵撳嵃缁撴瀯浣撹寖鍥淬€俿truct range 淇濆瓨浠绘剰鑼冨洿鐨?u64 鍊笺€傚鏋?start 绛変簬 end锛屽垯鍙墦鍗?start 鍊笺€?
閫氳繃寮曠敤浼犻€掋€?
### DMA 鍦板潃绫诲瀷 dma_addr_t


```
	%pad	0x01234567 or 0x0123456789abcdef
```

鐢ㄤ簬鎵撳嵃 dma_addr_t 绫诲瀷锛屽畠鍙互闅忔瀯寤洪€夐」鍙樺寲锛岃€屼笌 CPU 鏁版嵁閫氳矾鐨勫搴︽棤鍏炽€?
閫氳繃寮曠敤浼犻€掋€?
### 浣滀负杞箟瀛楃涓茬殑鍘熷缂撳啿鍖?

```
	%*pE[achnops]
```

```
		1b 62 20 5c 43 07 22 90 0d 5d
```

涓嬮潰鍑犱釜渚嬪瓙灞曠ず浜嗚浆鎹㈡槸濡備綍杩涜鐨勶紙涓嶅寘鍚鍥寸殑锛?
```
		%*pE		"\eb \C\a"\220\r]"
		%*pEhp		"\x1bb \C\x07"\x90\x0d]"
		%*pEa		"\e\142\040\\\103\a\042\220\r\135"
```

杞崲瑙勫垯鏍规嵁鍙€夌殑鏍囧織缁勫悎鏉ュ簲鐢紙璇︽儏璇峰弬闃?`string_escape_mem` 鍐呮牳鏂囨。锛夛細

 - a - ESCAPE_ANY
 - c - ESCAPE_SPECIAL
 - h - ESCAPE_HEX
 - n - ESCAPE_NULL
 - o - ESCAPE_OCTAL
 - p - ESCAPE_NP
 - s - ESCAPE_SPACE

榛樿浣跨敤 ESCAPE_ANY_NP銆?
ESCAPE_ANY_NP 瀵硅澶氭儏鍐甸兘鏄悎鐞嗙殑閫夋嫨锛屽挨鍏舵槸鍦ㄦ墦鍗?SSID 鏃躲€?
濡傛灉鐪佺暐浜嗗瓧娈靛搴︼紝鍒欏彧浼氳浆涔?1 涓瓧鑺傘€?
### 浣滀负鍗佸叚杩涘埗瀛楃涓茬殑鍘熷缂撳啿鍖?

```
	%*ph	00 01 02  ...  3f
	%*phC	00:01:02: ... :3f
	%*phD	00-01-02- ... -3f
	%*phN	000102 ... 3f
```

鐢ㄤ簬浠ュ皬鍐欏崄鍏繘鍒跺瓧绗︿覆褰㈠紡鎵撳嵃灏忓瀷缂撳啿鍖猴紙鏈€闀?64 瀛楄妭锛夛紝骞跺甫鏈夋煇绉嶅垎闅旂銆傚浜庢洿澶х殑缂撳啿鍖猴紝鑰冭檻浣跨敤 `print_hex_dump`銆?
### MAC/FDDI 鍦板潃


```
	%pM	00:01:02:03:04:05
	%pMR	05:04:03:02:01:00
	%pMF	00-01-02-03-04-05
	%pm	000102030405
	%pmR	050403020100
```

鐢ㄤ簬浠ュ崄鍏繘鍒惰〃绀烘硶鎵撳嵃 6 瀛楄妭鐨?MAC/FDDI 鍦板潃銆俙M` 鍜?`m` 璇存槑绗︿細浜х敓甯︼紙M锛夋垨涓嶅甫锛坢锛夊瓧鑺傚垎闅旂鐨勬墦鍗板湴鍧€銆傞粯璁ょ殑瀛楄妭鍒嗛殧绗︽槸鍐掑彿锛?锛夈€?
瀵逛簬 FDDI 鍦板潃锛屽彲鍦?`M` 璇存槑绗︿箣鍚庝娇鐢?`F` 璇存槑绗︼紝浠ヤ娇鐢ㄧ煭妯嚎锛?锛夊垎闅旂浠ｆ浛榛樿鍒嗛殧绗︺€?
瀵逛簬钃濈墮鍦板潃锛屽簲鍦?`M` 璇存槑绗︿箣鍚庝娇鐢?`R` 璇存槑绗︼紝浠ヤ娇鐢ㄥ弽杞殑瀛楄妭搴忥紝渚夸簬浠?little endian 椤哄簭鎺掑垪鐨勮摑鐗欏湴鍧€杩涜鐩磋瑙ｈ銆?
閫氳繃寮曠敤浼犻€掋€?
### IPv4 鍦板潃


```
	%pI4	1.2.3.4
	%pi4	001.002.003.004
	%p[Ii]4[hnbl]
```

鐢ㄤ簬鎵撳嵃浠ョ偣鍒嗛殧鐨勫崄杩涘埗 IPv4 鍦板潃銆俙I4` 鍜?`i4` 璇存槑绗︿細浜х敓甯︼紙i4锛夋垨涓嶅甫锛圛4锛夊墠瀵奸浂鐨勬墦鍗板湴鍧€銆?
闄勫姞鐨?`h`銆乣n`銆乣b` 鍜?`l` 璇存槑绗﹀垎鍒敤浜庢寚瀹氫富鏈哄簭銆佺綉缁滃簭銆佸ぇ绔垨灏忕搴忓湴鍧€銆傝嫢鏈彁渚涜鏄庣锛屽垯浣跨敤榛樿鐨勭綉缁滃簭/澶х搴忋€?
閫氳繃寮曠敤浼犻€掋€?
### IPv6 鍦板潃


```
	%pI6	0001:0002:0003:0004:0005:0006:0007:0008
	%pi6	00010002000300040005000600070008
	%pI6c	1:2:3:4:5:6:7:8
```

鐢ㄤ簬鎵撳嵃 IPv6 缃戠粶搴忕殑 16 浣嶅崄鍏繘鍒跺湴鍧€銆俙I6` 鍜?`i6` 璇存槑绗︿細浜х敓甯︼紙I6锛夋垨涓嶅甫锛坕6锛夊啋鍙峰垎闅旂鐨勬墦鍗板湴鍧€銆傚缁堜娇鐢ㄥ墠瀵奸浂銆?
闄勫姞鐨?`c` 璇存槑绗﹀彲涓?`I` 璇存槑绗︿竴璧蜂娇鐢紝浠ユ墦鍗扮敱 https://tools.ietf.org/html/rfc5952 鎻忚堪鐨勫帇缂?IPv6 鍦板潃銆?
閫氳繃寮曠敤浼犻€掋€?
### IPv4/IPv6 鍦板潃锛堥€氱敤锛屽甫绔彛銆乫lowinfo銆佷綔鐢ㄥ煙锛?

```
	%pIS	1.2.3.4		or 0001:0002:0003:0004:0005:0006:0007:0008
	%piS	001.002.003.004	or 00010002000300040005000600070008
	%pISc	1.2.3.4		or 1:2:3:4:5:6:7:8
	%pISpc	1.2.3.4:12345	or [1:2:3:4:5:6:7:8]:12345
	%p[Ii]S[pfschnbl]
```

鐢ㄤ簬鎵撳嵃 IP 鍦板潃锛岃€屾棤闇€鍖哄垎瀹冩槸 AF_INET 杩樻槸 AF_INET6 绫诲瀷銆傚彲浠ュ皢鎸囧悜鏈夋晥 struct sockaddr 鐨勬寚閽堬紙閫氳繃 `IS` 鎴?`iS` 鎸囧畾锛変紶缁欐鏍煎紡璇存槑绗︺€?
闄勫姞鐨?`p`銆乣f` 鍜?`s` 璇存槑绗﹀垎鍒敤浜庢寚瀹氱鍙ｏ紙IPv4銆両Pv6锛夈€乫lowinfo锛圛Pv6锛夊拰浣滅敤鍩燂紙IPv6锛夈€傜鍙ｅ甫鏈?`:` 鍓嶇紑锛宖lowinfo 甯︽湁 `/` 鍓嶇紑锛屼綔鐢ㄥ煙甯︽湁 `%` 鍓嶇紑锛屽悇鑷悗璺熷疄闄呭€笺€?
瀵逛簬 IPv6 鍦板潃锛屽鏋滅粰鍑轰簡闄勫姞璇存槑绗?`c`锛屽垯浣跨敤 https://tools.ietf.org/html/rfc5952 鎻忚堪鐨勫帇缂?IPv6 鍦板潃銆傚湪甯︽湁闄勫姞璇存槑绗?`p`銆乣f` 鎴?`s` 鐨勬儏鍐典笅锛孖Pv6 鍦板潃浼氳 `[`銆乣]` 鍖呭洿锛屾濡?https://tools.ietf.org/html/draft-ietf-6man-text-addr-representation-07 鎵€寤鸿鐨勩€?
瀵逛簬 IPv4 鍦板潃锛屼篃鍙互鍚屾牱浣跨敤闄勫姞鐨?`h`銆乣n`銆乣b` 鍜?`l` 璇存槑绗︼紝鑰屽湪 IPv6 鍦板潃鎯呭喌涓嬪畠浠細琚拷鐣ャ€?
閫氳繃寮曠敤浼犻€掋€?
```
	%pISfc		1.2.3.4		or [1:2:3:4:5:6:7:8]/123456789
	%pISsc		1.2.3.4		or [1:2:3:4:5:6:7:8]%1234567890
	%pISpfc		1.2.3.4:12345	or [1:2:3:4:5:6:7:8]:12345/123456789
```

### UUID/GUID 鍦板潃


```
	%pUb	00010203-0405-0607-0809-0a0b0c0d0e0f
	%pUB	00010203-0405-0607-0809-0A0B0C0D0E0F
	%pUl	03020100-0504-0706-0809-0a0b0c0e0e0f
	%pUL	03020100-0504-0706-0809-0A0B0C0E0E0F
```

鐢ㄤ簬鎵撳嵃 16 瀛楄妭鐨?UUID/GUID 鍦板潃銆傞檮鍔犵殑 `l`銆乣L`銆乣b` 鍜?`B` 璇存槑绗︾敤浜庝互灏忕搴忥紙l 灏忓啓鎴?L 澶у啓鍗佸叚杩涘埗锛夋垨澶х搴忥紙b 灏忓啓鎴?B 澶у啓鍗佸叚杩涘埗锛夋寚瀹氬湴鍧€銆?
鏈娇鐢ㄩ檮鍔犺鏄庣鏃讹紝灏嗘墦鍗伴粯璁ょ殑澶х搴忓皬鍐欏崄鍏繘鍒惰〃绀恒€?
閫氳繃寮曠敤浼犻€掋€?
### dentry 鍚嶇О


```
	%pd{,2,3,4}
	%pD{,2,3,4}
```

鐢ㄤ簬鎵撳嵃 dentry 鍚嶇О锛涘鏋滄垜浠笌 `d_move` 鍙戠敓绔炰簤锛屽悕绉板彲鑳芥槸鏂版棫鍚嶇О鐨勬贩鍚堬紝浣嗕笉浼氬彂鐢?oops銆?pd dentry 鏄垜浠繃鍘讳娇鐢ㄧ殑 %s dentry->d_name.name 鐨勪竴涓洿瀹夊叏鐨勭瓑浠峰舰寮忥紝%pd<n> 鎵撳嵃鏈€鍚?`n` 涓垎閲忋€?pD 瀵?struct file 鍋氬悓鏍风殑浜嬫儏銆?
閫氳繃寮曠敤浼犻€掋€?
### block_device 鍚嶇О


```
	%pg	sda, sda1 or loop0p1
```

鐢ㄤ簬鎵撳嵃 block_device 鎸囬拡鐨勫悕绉般€?
### struct va_format


```
	%pV
```

鐢ㄤ簬鎵撳嵃 struct va_format 缁撴瀯浣撱€傚畠浠寘鍚竴涓牸寮忓瓧绗︿覆锛?
```
	struct va_format {
		const char *fmt;
		va_list *va;
	};
```

瀹炵幇浜嗕竴绉?閫掑綊 vsnprintf"銆?
涓嶈鍦ㄦ病鏈夋煇绉嶆満鍒舵潵楠岃瘉鏍煎紡瀛楃涓插拰 va_list 鍙傛暟姝ｇ‘鎬х殑鎯呭喌涓嬩娇鐢ㄦ鍔熻兘銆?
閫氳繃寮曠敤浼犻€掋€?
### 璁惧鏍戣妭鐐?

```
	%pOF[fnpPcCF]
```

鐢ㄤ簬鎵撳嵃璁惧鏍戣妭鐐圭粨鏋勪綋銆傞粯璁よ涓虹瓑浠蜂簬 %pOFf銆?
 - f - 璁惧鑺傜偣 full_name
 - n - 璁惧鑺傜偣 name
 - p - 璁惧鑺傜偣 phandle
 - P - 璁惧鑺傜偣璺緞瑙勬牸锛坣ame + @unit锛? - F - 璁惧鑺傜偣 flags
 - c - 涓昏 compatible 瀛楃涓? - C - 瀹屾暣 compatible 瀛楃涓?
浣跨敤澶氫釜鍙傛暟鏃讹紝鍒嗛殧绗︿负 ':'銆?
```
	%pOF	/foo/bar@0			- 鑺傜偣鍏ㄥ悕
	%pOFf	/foo/bar@0			- 鍚屼笂
	%pOFfp	/foo/bar@0:10			- 鑺傜偣鍏ㄥ悕 + phandle
	%pOFfcF	/foo/bar@0:foo,device:--P-	- 鑺傜偣鍏ㄥ悕 +
	                                          major compatible 瀛楃涓?+
						  鑺傜偣 flags
							D - 鍔ㄦ€侊紙dynamic锛?							d - 宸插垎绂伙紙detached锛?							P - 宸插～鍏咃紙Populated锛?							B - 宸插～鍏呮€荤嚎锛圥opulated bus锛?```

閫氳繃寮曠敤浼犻€掋€?
### Fwnode 鍙ユ焺


```
	%pfw[fP]
```

鐢ㄤ簬鎵撳嵃鏈夊叧 fwnode_handle 鐨勪俊鎭€傞粯璁ゆ槸鎵撳嵃瀹屾暣鐨勮妭鐐瑰悕锛屽寘鎷矾寰勩€傝繖浜涗慨楗扮鍦ㄥ姛鑳戒笂绛変环浜庝笂闈㈢殑 %pOF銆?
 - f - 鑺傜偣鐨勫叏鍚嶏紝鍖呮嫭璺緞
 - P - 鑺傜偣鐨勫悕绉帮紝鍖呮嫭鍦板潃锛堝鏋滄湁锛?
```
	%pfwf	\_SB.PCI0.CIO2.port@1.endpoint@0	- 瀹屾暣鑺傜偣鍚?	%pfwP	endpoint@0				- 鑺傜偣鍚?```

```
	%pfwf	/ocp@68000000/i2c@48072000/camera@10/port/endpoint - 鍏ㄥ悕
	%pfwP	endpoint				- 鑺傜偣鍚?```

### 鏃堕棿涓庢棩鏈?

```
	%pt[RT]			YYYY-mm-ddTHH:MM:SS
	%pt[RT]s		YYYY-mm-dd HH:MM:SS
	%pt[RT]d		YYYY-mm-dd
	%pt[RT]t		HH:MM:SS
	%ptSp			<seconds>.<nanoseconds>
	%pt[RST][dt][r][s]
```

```
	R  struct rtc_time 鐨勫唴瀹?	S  struct timespec64 鐨勫唴瀹?	T  time64_t 绫诲瀷
```

浠ヤ汉绫诲彲璇荤殑鏍煎紡銆?
榛樿鎯呭喌涓嬪勾浠戒細鍔?1900锛屾湀浠戒細鍔?1銆備娇鐢?%pt[RT]r锛堝師濮嬶級鏉ユ姂鍒舵琛屼负銆?
%pt[RT]s锛堢┖鏍硷級浼氱敤 ' '锛堢┖鏍硷級浠ｆ浛鏃ユ湡涓庢椂闂翠箣闂寸殑 ISO 8601 鍒嗛殧绗?'T'锛堝ぇ鍐?T锛夈€傚綋鏃ユ湡鎴栨椂闂磋鐪佺暐鏃跺畠涓嶈捣浣滅敤銆?
%ptSp 绛変环浜?struct timespec64 鍐呭鐨?%lld.%09ld銆傚綋缁欏嚭鍏朵粬璇存槑绗︽椂锛屽畠灏卞彉鎴?%ptT[dt][r][s].%09ld 鐨勭浉搴旂瓑浠峰舰寮忋€傛崲鍙ヨ瘽璇达紝绉掍互浜虹被鍙鐨勬牸寮忔墦鍗帮紝鍚庤窡涓€涓偣浠ュ強绾崇銆?
閫氳繃寮曠敤浼犻€掋€?
### struct clk


```
	%pC	pll1
```

鐢ㄤ簬鎵撳嵃 struct clk 缁撴瀯浣撱€?pC 鎵撳嵃鏃堕挓锛堥€氱敤鏃堕挓妗嗘灦锛孋ommon Clock Framework锛夌殑鍚嶇О锛屾垨涓€涓敮涓€鐨?32 浣?ID锛堥仐鐣欐椂閽熸鏋讹級銆?
閫氳繃寮曠敤浼犻€掋€?
### 浣嶅浘鍙婂叾琛嶇敓绫诲瀷锛堝 cpumask 鍜?nodemask锛?

```
	%*pb	0779
	%*pbl	0,3-6,8-10
```

鐢ㄤ簬鎵撳嵃浣嶅浘鍙婂叾琛嶇敓绫诲瀷锛堝 cpumask 鍜?nodemask锛夛紝%*pb 浠ュ瓧娈靛搴︿负浣嶆暟杈撳嚭浣嶅浘锛?*pbl 浠ュ瓧娈靛搴︿负浣嶆暟灏嗕綅鍥句綔涓鸿寖鍥村垪琛ㄨ緭鍑恒€?
瀛楁瀹藉害鎸夊€间紶閫掞紝浣嶅浘閫氳繃寮曠敤浼犻€掋€傝緟鍔╁畯 cpumask_pr_args() 鍜?nodemask_pr_args() 鍙敤浜庣畝鍖?cpumask 鍜?nodemask 鐨勬墦鍗般€?
### 鏍囧織浣嶅煙锛堝椤垫爣蹇楀拰 gfp_flags锛?

```
	%pGp	0x17ffffc0002036(referenced|uptodate|lru|active|private|node=0|zone=2|lastcpupid=0x1fffff)
	%pGg	GFP_USER|GFP_DMA32|GFP_NOWARN
	%pGv	read|exec|mayread|maywrite|mayexec|denywrite
```

鐢ㄤ簬灏嗘爣蹇椾綅鍩熶綔涓轰竴缁勬瀯閫犺鍊肩殑绗﹀彿甯搁噺鎵撳嵃銆傛爣蹇楃殑绫诲瀷鐢辩涓変釜瀛楃缁欏嚭銆傜洰鍓嶆敮鎸佺殑鏈夛細

        - p - [p]age 鏍囧織锛屾湡鏈涚被鍨嬩负 (`unsigned long *`) 鐨勫€?        - v - [v]ma_flags锛屾湡鏈涚被鍨嬩负 (`unsigned long *`) 鐨勫€?        - g - [g]fp_flags锛屾湡鏈涚被鍨嬩负 (`gfp_t *`) 鐨勫€?
鏍囧織鍚嶅拰鎵撳嵃椤哄簭鍙栧喅浜庡叿浣撶被鍨嬨€?
娉ㄦ剰锛屾鏍煎紡涓嶅簲鐩存帴鍦?tracepoint 鐨?`TP_printk()` 閮ㄥ垎涓娇鐢ㄣ€傝€屽簲鏀圭敤 <trace/events/mmflags.h> 涓殑 show_*_flags() 鍑芥暟銆?
閫氳繃寮曠敤浼犻€掋€?
### 缃戠粶璁惧鐗规€?

```
	%pNF	0x000000000000c000
```

鐢ㄤ簬鎵撳嵃 netdev_features_t銆?
閫氳繃寮曠敤浼犻€掋€?
### V4L2 鍜?DRM 鐨?FourCC 鐮侊紙鍍忕礌鏍煎紡锛?

```
	%p4cc
```

鎵撳嵃 V4L2 鎴?DRM 浣跨敤鐨?FourCC 鐮侊紝鍖呮嫭鏍煎紡瀛楄妭搴忓強鍏跺崄鍏繘鍒舵暟鍊笺€?
閫氳繃寮曠敤浼犻€掋€?
```
	%p4cc	BG12 little-endian (0x32314742)
	%p4cc	Y10  little-endian (0x20303159)
	%p4cc	NV12 big-endian (0xb231564e)
```

### 閫氱敤 FourCC 鐮?

```
	%p4c[h[R]lb]	gP00 (0x67503030)
```

鎵撳嵃閫氱敤鐨?FourCC 鐮侊紝鍚屾椂浠?ASCII 瀛楃鍙婂叾鍗佸叚杩涘埗鏁板€煎舰寮忚緭鍑恒€?
閫氱敤 FourCC 鐮佹€绘槸浠ュぇ绔牸寮忔墦鍗帮紝鍗虫渶楂樻湁鏁堝瓧鑺傚湪鍓嶃€傝繖涓?V4L/DRM 鐨?FourCC 鐩稿弽銆?
闄勫姞鐨?`h`銆乣hR`銆乣l` 鍜?`b` 璇存槑绗﹀畾涔変簡鐢ㄤ簬鍔犺浇鎵€瀛樺偍瀛楄妭鐨勫瓧鑺傚簭銆傛暟鎹彲鑳借瑙ｉ噴涓轰富鏈哄簭銆佸弽涓绘満瀛楄妭搴忋€佸皬绔簭鎴栧ぇ绔簭銆?
閫氳繃寮曠敤浼犻€掋€?
```
	%p4ch	gP00 (0x67503030)
	%p4chR	00Pg (0x30305067)
	%p4cl	gP00 (0x67503030)
	%p4cb	00Pg (0x30305067)
```

```
	%p4ch	gP00 (0x67503030)
	%p4chR	00Pg (0x30305067)
	%p4cl	00Pg (0x30305067)
	%p4cb	gP00 (0x67503030)
```

### Rust


```
	%pA
```

**浠呮墦绠楃敤浜庝粠 Rust 浠ｇ爜鏍煎紡鍖?* ``core``
: fmt::Arguments``銆?璇?*涓嶈**浠?C 浠ｇ爜涓娇鐢ㄥ畠銆?
## 鑷磋阿


濡傛灉浣犳坊鍔犱簡鍏朵粬 %p 鎵╁睍锛岃鍦ㄥ彲琛岀殑鎯呭喌涓嬬敤涓€涓垨澶氫釜娴嬭瘯鐢ㄤ緥鎵╁睍 <lib/tests/printf_kunit.c>銆?
鎰熻阿浣犵殑閰嶅悎涓庡叧娉ㄣ€?