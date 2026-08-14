## page owner: Tracking about who allocated each page


## Introduction


page owner 鐢ㄤ簬璺熻釜姣忎釜椤甸潰鏄敱璋佸垎閰嶇殑銆傚畠鍙敤浜庤皟璇曞唴瀛樻硠婕忥紝鎴栨壘鍑哄唴瀛樺崰鐢ㄥぇ鎴枫€?
褰撳彂鐢熷垎閰嶆椂锛屽叧浜庡垎閰嶇殑淇℃伅锛堝璋冪敤鏍堝拰椤甸潰鐨?order锛変細琚瓨鍌ㄥ埌姣忎釜椤甸潰鐨勭壒瀹氬瓨鍌ㄤ腑銆?
褰撴垜浠渶瑕佷簡瑙ｆ墍鏈夐〉闈㈢殑鐘舵€佹椂锛屽彲浠ヨ幏鍙栧苟鍒嗘瀽杩欎簺淇℃伅銆?

灏界鎴戜滑宸茬粡鏈夌敤浜庤窡韪〉闈㈠垎閰?閲婃斁鐨?tracepoint锛屼絾鐢ㄥ畠鏉ュ垎鏋愯皝鍒嗛厤浜嗘瘡涓〉闈㈢浉褰?
澶嶆潅銆傛垜浠渶瑕佹墿澶?trace 缂撳啿鍖猴紝浠ラ槻姝㈠湪鐢ㄦ埛绌洪棿绋嬪簭鍚姩鍓嶅彂鐢熼噸鍙犮€傝€屼笖锛屽惎鍔ㄧ殑绋嬪簭浼?
鎸佺画杞偍 trace 缂撳啿鍖轰互渚涘悗缁垎鏋愶紝杩欐瘮浠呬粎灏嗗叾淇濈暀鍦ㄥ唴瀛樹腑鏇村彲鑳芥敼鍙樼郴缁熻涓猴紝鍥犳涓?
鍒╀簬璋冭瘯銆?

page owner 涔熷彲鐢ㄤ簬鍚勭鍏朵粬鐩殑銆備緥濡傦紝閫氳繃姣忎釜椤甸潰鐨?gfp 鏍囧織淇℃伅鍙互鑾峰緱鍑嗙‘鐨勭鐗?
缁熻銆傚鏋滃惎鐢ㄤ簡 page owner锛岃繖宸茬粡瀹炵幇骞舵縺娲汇€傛杩庡叾浠栫敤娉曘€?

瀹冭繕鍙敤浜庢樉绀烘墍鏈夎皟鐢ㄦ爤鍙婂叾褰撳墠鍒嗛厤鐨勫熀椤垫暟閲忥紝杩欒鎴戜滑鏃犻渶绛涙煡鎵€鏈夐〉闈㈠苟鍖归厤鍒嗛厤鍜?
閲婃斁鎿嶄綔锛屽氨鑳藉揩閫熶簡瑙ｅ唴瀛樼殑鍘诲悜銆備篃鍙互鍙樉绀烘墍鏈夎皟鐢ㄦ爤锛堜笉鍚爤鍥炴函锛夌殑鏁板瓧鏍囪瘑绗﹀強鍏?
鍒嗛厤鐨勫熀椤垫暟閲忥紙璇诲彇鍜岃В鏋愭洿蹇紝渚嬪鐢ㄤ簬鐩戞帶锛夛紝涔嬪悗鍙互涓庤皟鐢ㄦ爤鍖归厤锛坰how_handles 鍜?
show_stacks_handles锛夈€?

page owner 榛樿鏄鐢ㄧ殑銆傚洜姝わ紝濡傛灉浣犳兂浣跨敤瀹冿紝闇€瑕佸湪鍚姩鍛戒护琛屼腑娣诲姞鈥減age_owner=on鈥濄€?
濡傛灉鍐呮牳鏋勫缓浜?page owner锛屼絾鐢变簬鏈惎鐢ㄥ惎鍔ㄩ€夐」鑰屽湪杩愯鏃惰绂佺敤锛岃繍琛屾椂寮€閿€鏄井涔庡叾寰殑銆?
濡傛灉鍦ㄨ繍琛屾椂绂佺敤锛屽垯涓嶉渶瑕佸唴瀛樻潵瀛樺偍鎵€鏈夎€呬俊鎭紝鍥犳娌℃湁杩愯鏃跺唴瀛樺紑閿€銆傝€屼笖锛宲age owner
鍙悜椤甸潰鍒嗛厤鍣ㄧ儹璺緞涓彃鍏ヤ簡涓や釜涓嶅お鍙兘鎵ц鐨勫垎鏀紝濡傛灉鏈惎鐢紝鍒欏垎閰嶅氨鍍忔病鏈?page owner
鐨勫唴鏍镐竴鏍疯繘琛屻€傝繖涓や釜涓嶅お鍙兘鎵ц鐨勫垎鏀笉搴斿奖鍝嶅垎閰嶆€ц兘锛岀壒鍒槸鍦ㄥ彲鐢ㄩ潤鎬侀敭璺宠浆鏍囩
琛ヤ竵锛坰tatic keys jump label patching锛夊姛鑳界殑鎯呭喌涓嬨€備互涓嬫槸璇ヨ鏂藉鑷寸殑鍐呮牳浠ｇ爜澶у皬鍙樺寲銆?

铏界劧鍚敤 page owner 浼氫娇鍐呮牳澶у皬澧炲姞鍑犲崈瀛楄妭锛屼絾鍏朵腑澶ч儴鍒嗕唬鐮佷綅浜庨〉闈㈠垎閰嶅櫒鍙婂叾鐑矾寰?
涔嬪銆傚湪闇€瑕佹椂鐢ㄥ惎鐢?page owner 鏋勫缓鍐呮牳骞跺湪闇€瑕佹椂鎵撳紑瀹冿紝鏄皟璇曞唴鏍稿唴瀛橀棶棰樼殑缁濅匠閫夋嫨銆?

鏈変竴涓敱瀹炵幇缁嗚妭寮曡捣鐨勬敞鎰忎簨椤广€俻age owner 灏嗕俊鎭瓨鍌ㄥ埌 struct page extension 鐨勫唴瀛樹腑銆?
鍦ㄧ█鐤忓唴瀛橈紙sparse memory锛夌郴缁熶腑锛岃繖鍧楀唴瀛樼殑鍒濆鍖栨椂闂存櫄浜庨〉闈㈠垎閰嶅櫒鍚姩鐨勬椂闂达紝鍥犳鍦?
鍒濆鍖栦箣鍓嶏紝璁稿椤甸潰鍙兘宸茬粡琚垎閰嶏紝瀹冧滑灏嗘病鏈夋墍鏈夎€呬俊鎭€備负浜嗕慨澶嶈繖涓€鐐癸紝杩欎簺鏃╂湡鍒嗛厤鐨?
椤甸潰鍦ㄥ垵濮嬪寲闃舵琚鏌ュ苟鏍囪涓哄凡鍒嗛厤銆傝櫧鐒惰繖骞朵笉鎰忓懗鐫€瀹冧滑鏈夋纭殑鎵€鏈夎€呬俊鎭紝浣嗚嚦灏戞垜浠?
鍙互鏇村噯纭湴鍒ゆ柇椤甸潰鏄惁琚垎閰嶃€傚湪涓€涓?2GB 鍐呭瓨鐨?x86-64 铏氭嫙鏈轰笂锛屾崟鑾峰苟鏍囪浜?13343 涓?
鏃╂湡鍒嗛厤鐨勯〉闈紝灏界瀹冧滑澶у鏄粠 struct page extension 鐗规€у垎閰嶇殑銆傛棤璁哄浣曪紝涔嬪悗娌℃湁椤甸潰
澶勪簬鏈窡韪姸鎬併€?

## Usage


```

	cd tools/mm
	make page_owner_sort

```
2) 鍚敤 page owner锛氬湪鍚姩鍛戒护琛屼腑娣诲姞鈥減age_owner=on鈥濄€?

3) 鍋氫綘鎯宠璋冭瘯鐨勫伐浣溿€?

```

	cat /sys/kernel/debug/page_owner_stacks/show_stacks > stacks.txt
	cat stacks.txt
	 post_alloc_hook+0x177/0x1a0
	 get_page_from_freelist+0xd01/0xd80
	 __alloc_pages+0x39e/0x7e0
	 allocate_slab+0xbc/0x3f0
	 ___slab_alloc+0x528/0x8a0
	 kmem_cache_alloc+0x224/0x3b0
	 sk_prot_alloc+0x58/0x1a0
	 sk_alloc+0x32/0x4f0
	 inet_create+0x427/0xb50
	 __sock_create+0x2e4/0x650
	 inet_ctl_sock_create+0x30/0x180
	 igmp_net_init+0xc1/0x130
	 ops_init+0x167/0x410
	 setup_net+0x304/0xa60
	 copy_net_ns+0x29b/0x4a0
	 create_new_namespaces+0x4a1/0x820
	nr_base_pages: 16
	...
	...
	echo 7000 > /sys/kernel/debug/page_owner_stacks/count_threshold
	cat /sys/kernel/debug/page_owner_stacks/show_stacks> stacks_7000.txt
	cat stacks_7000.txt
	 post_alloc_hook+0x177/0x1a0
	 get_page_from_freelist+0xd01/0xd80
	 __alloc_pages+0x39e/0x7e0
	 alloc_pages_mpol+0x22e/0x490
	 folio_alloc+0xd5/0x110
	 filemap_alloc_folio+0x78/0x230
	 page_cache_ra_order+0x287/0x6f0
	 filemap_get_pages+0x517/0x1160
	 filemap_read+0x304/0x9f0
	 xfs_file_buffered_read+0xe6/0x1d0 [xfs]
	 xfs_file_read_iter+0x1f0/0x380 [xfs]
	 __kernel_read+0x3b9/0x730
	 kernel_read_file+0x309/0x4d0
	 __do_sys_finit_module+0x381/0x730
	 do_syscall_64+0x8d/0x150
	 entry_SYSCALL_64_after_hwframe+0x62/0x6a
	nr_base_pages: 20824
	...

	cat /sys/kernel/debug/page_owner_stacks/show_handles > handles_7000.txt
	cat handles_7000.txt
	handle: 42
	nr_base_pages: 20824
	...

	cat /sys/kernel/debug/page_owner_stacks/show_stacks_handles > stacks_handles.txt
	cat stacks_handles.txt
	 post_alloc_hook+0x177/0x1a0
	 get_page_from_freelist+0xd01/0xd80
	 __alloc_pages+0x39e/0x7e0
	 alloc_pages_mpol+0x22e/0x490
	 folio_alloc+0xd5/0x110
	 filemap_alloc_folio+0x78/0x230
	 page_cache_ra_order+0x287/0x6f0
	 filemap_get_pages+0x517/0x1160
	 filemap_read+0x304/0x9f0
	 xfs_file_buffered_read+0xe6/0x1d0 [xfs]
	 xfs_file_read_iter+0x1f0/0x380 [xfs]
	 __kernel_read+0x3b9/0x730
	 kernel_read_file+0x309/0x4d0
	 __do_sys_finit_module+0x381/0x730
	 do_syscall_64+0x8d/0x150
	 entry_SYSCALL_64_after_hwframe+0x62/0x6a
	handle: 42
	...

	cat /sys/kernel/debug/page_owner > page_owner_full.txt
	./page_owner_sort page_owner_full.txt sorted_page_owner.txt

   The general output of ``page_owner_full.txt`` is as follows::

	Page allocated via order XXX, ...
	PFN XXX ...
	// Detailed stack

	Page allocated via order XXX, ...
	PFN XXX ...
	// Detailed stack
    By default, it will do full pfn dump, to start with a given pfn,
    page_owner supports fseek.

    FILE *fp = fopen("/sys/kernel/debug/page_owner", "r");
    fseek(fp, pfn_start, SEEK_SET);

   The ``page_owner_sort`` tool ignores ``PFN`` rows, puts the remaining rows
   in buf, uses regexp to extract the page order value, counts the times
   and pages of buf, and finally sorts them according to the parameter(s).

   See the result about who allocated each page
   in the ``sorted_page_owner.txt``. General output::

	XXX times, XXX pages:
	Page allocated via order XXX, ...
	// Detailed stack

   By default, ``page_owner_sort`` is sorted according to the times of buf.
   If you want to sort by the page nums of buf, use the ``-m`` parameter.
   The detailed parameters are:

   fundamental function::

	Sort:
		-a		Sort by memory allocation time.
		-m		Sort by total memory.
		-p		Sort by pid.
		-P		Sort by tgid.
		-n		Sort by task command name.
		-r		Sort by memory release time.
		-s		Sort by stack trace.
		-t		Sort by times (default).
		--sort <order>	Specify sorting order.  Sorting syntax is [+|-]key[,[+|-]key[,...]].
				Choose a key from the **STANDARD FORMAT SPECIFIERS** section. The "+" is
				optional since default direction is increasing numerical or lexicographic
				order. Mixed use of abbreviated and complete-form of keys is allowed.

		Examples:
				./page_owner_sort <input> <output> --sort=n,+pid,-tgid
				./page_owner_sort <input> <output> --sort=at

   additional function::

	Cull:
		--cull <rules>
				Specify culling rules.Culling syntax is key[,key[,...]].Choose a
				multi-letter key from the **STANDARD FORMAT SPECIFIERS** section.

		<rules> is a single argument in the form of a comma-separated list,
		which offers a way to specify individual culling rules.  The recognized
		keywords are described in the **STANDARD FORMAT SPECIFIERS** section below.
		<rules> can be specified by the sequence of keys k1,k2, ..., as described in
		the STANDARD SORT KEYS section below. Mixed use of abbreviated and
		complete-form of keys is allowed.

		Examples:
				./page_owner_sort <input> <output> --cull=stacktrace
				./page_owner_sort <input> <output> --cull=st,pid,name
				./page_owner_sort <input> <output> --cull=n,f

	Filter:
		-f		Filter out the information of blocks whose memory has been released.

	Select:
		--pid <pidlist>		Select by pid. This selects the blocks whose process ID
					numbers appear in <pidlist>.
		--tgid <tgidlist>	Select by tgid. This selects the blocks whose thread
					group ID numbers appear in <tgidlist>.
		--name <cmdlist>	Select by task command name. This selects the blocks whose
					task command name appear in <cmdlist>.

		<pidlist>, <tgidlist>, <cmdlist> are single arguments in the form of a comma-separated list,
		which offers a way to specify individual selecting rules.


		Examples:
				./page_owner_sort <input> <output> --pid=1
				./page_owner_sort <input> <output> --tgid=1,2,3
				./page_owner_sort <input> <output> --name name1,name2

```
## STANDARD FORMAT SPECIFIERS

```

  For --sort option:

	KEY		LONG		DESCRIPTION
	p		pid		process ID
	tg		tgid		thread group ID
	n		name		task command name
	st		stacktrace	stack trace of the page allocation
	T		txt		full text of block
	ft		free_ts		timestamp of the page when it was released
	at		alloc_ts	timestamp of the page when it was allocated
	ator		allocator	memory allocator for pages

  For --cull option:

	KEY		LONG		DESCRIPTION
	p		pid		process ID
	tg		tgid		thread group ID
	n		name		task command name
	f		free		whether the page has been released or not
	st		stacktrace	stack trace of the page allocation
	ator		allocator	memory allocator for pages

```