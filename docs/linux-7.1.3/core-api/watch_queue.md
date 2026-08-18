## 閫氱敤閫氱煡鏈哄埗


閫氱敤閫氱煡鏈哄埗鏋勫缓鍦ㄦ爣鍑嗙閬擄紙pipe锛夐┍鍔ㄤ箣涓婏紝瀹冨疄闄呬笂灏嗘潵鑷唴鏍哥殑閫氱煡娑堟伅鎷兼帴鍒扮閬撲腑
```

  * Key/keyring notifications


```
閫氱煡缂撳啿鍖哄彲閫氳繃浠ヤ笅鏂瑰紡鍚敤锛?
	"General setup"/"General notification queue"
	(CONFIG_WATCH_QUEUE)

鏈枃妗ｅ寘鍚互涓嬬珷鑺傦細



## 姒傝堪


璇ユ満鍒惰〃鐜颁负涓€涓互鐗规畩妯″紡鎵撳紑鐨勭閬撱€傜閬撳唴閮ㄧ殑鐜舰缂撳啿鍖虹敤浜庝繚瀛樼敱鍐呮牳鐢熸垚鐨勬秷鎭€傝繖浜涙秷鎭殢鍚庣敱 read() 璇诲嚭銆傛绫荤閬撲笂绂佺敤浜?splice 鍙婄被浼兼搷浣滐紝鍥犱负瀹冧滑鍦ㄦ煇浜涙儏鍐典笅鍙兘甯屾湜鎾ら攢瀵圭幆褰㈢紦鍐插尯鐨勬坊鍔犫€斺€旇€岃繖鏈€缁堝彲鑳戒細涓庨€氱煡娑堟伅浜ら敊鍦ㄤ竴璧枫€?
绠￠亾鐨勬墍鏈夎€呭繀椤诲憡璇夊唴鏍稿畠甯屾湜閫氳繃璇ョ閬撶洃瑙嗗摢浜涙潵婧愩€傚彧鏈夊凡杩炴帴鍒版煇涓閬撶殑鏉ユ簮鎵嶄細鍚戝叾涓彃鍏ユ秷鎭€傝娉ㄦ剰锛屼竴涓潵婧愬彲浠ョ粦瀹氬埌澶氫釜绠￠亾锛屽苟鍚屾椂鍚戞墍鏈夎繖浜涚閬撴彃鍏ユ秷鎭€?
涔熷彲浠ュ湪绠￠亾涓婃斁缃繃婊ゅ櫒锛屼互渚垮湪涓嶆劅鍏磋叮鏃跺拷鐣ユ煇浜涙潵婧愮被鍨嬪拰瀛愪簨浠躲€?
濡傛灉鐜舰缂撳啿鍖轰腑娌℃湁鍙敤鐨勬Ы浣嶏紝鎴栬€呮病鏈夊彲鐢ㄧ殑棰勫垎閰嶆秷鎭紦鍐插尯锛屽垯娑堟伅灏嗚涓㈠純銆傚湪杩欎袱绉嶆儏鍐典笅锛宺ead() 浼氬湪缂撳啿鍖轰腑褰撳墠鏈€鍚庝竴鏉℃秷鎭璇诲彇涔嬪悗锛屽悜杈撳嚭缂撳啿鍖烘彃鍏ヤ竴鏉?WATCH_META_LOSS_NOTIFICATION 娑堟伅銆?
璇锋敞鎰忥紝鍦ㄤ骇鐢熼€氱煡鏃讹紝鍐呮牳涓嶄細绛夊緟娑堣垂鑰呮潵鏀堕泦瀹冿紝鑰屾槸鐩存帴缁х画銆傝繖鎰忓懗鐫€閫氱煡鍙互鍦ㄦ寔鏈夎嚜鏃嬮攣鐨勬儏鍐典笅鐢熸垚锛屽悓鏃朵篃淇濇姢鍐呮牳涓嶄細琚敤鎴风┖闂寸殑鏁呴殰鏃犻檺鏈熷湴鍗′綇銆?

## 娑堟伅缁撴瀯


```

	struct watch_notification {
		__u32	type:24;
		__u32	subtype:8;
		__u32	info;
	};

```
銆宼ype銆嶈〃绀洪€氱煡璁板綍鐨勬潵婧愶紝"subtype" 琛ㄧず璇ユ潵婧愮殑璁板綍绫诲瀷锛堝弬瑙佷笅闈㈢殑鈥滅洃瑙嗘潵婧愨€濅竴鑺傦級銆倀ype 涔熷彲鑳芥槸 "WATCH_TYPE_META"銆傝繖鏄竴绉嶇敱鐩戣闃熷垪鑷韩鍦ㄥ唴閮ㄧ敓鎴愮殑鐗规畩璁板綍绫诲瀷銆傚畠鏈変袱涓瓙绫诲瀷锛?
  - WATCH_META_REMOVAL_NOTIFICATION
  - WATCH_META_LOSS_NOTIFICATION

鍓嶈€呰〃绀哄畨瑁呬簡鐩戣鐨勫璞¤绉婚櫎鎴栭攢姣侊紝鍚庤€呰〃绀烘煇浜涙秷鎭凡涓㈠け銆?
銆宨nfo銆嶈〃绀哄椤瑰唴瀹癸紝鍖呮嫭锛?
  - 娑堟伅鐨勯暱搴︼紙浠ュ瓧鑺備负鍗曚綅锛屽惈澶撮儴锛夛紙鐢?WATCH_INFO_LENGTH 鎺╃爜锛屽苟鍙崇Щ WATCH_INFO_LENGTH__SHIFT锛夈€傝繖琛ㄧず璁板綍鐨勫ぇ灏忥紝浠嬩簬 8 鍒?127 瀛楄妭涔嬮棿銆?
  - 鐩戣 ID锛堢敤 WATCH_INFO_ID 鎺╃爜锛屽苟鍙崇Щ WATCH_INFO_ID__SHIFT锛夈€傝繖琛ㄧず鐩戣鐨勮皟鐢ㄨ€?ID锛屼粙浜?0 鍒?255 涔嬮棿銆傚涓洃瑙嗗彲浠ュ叡浜竴涓槦鍒楋紝杩欐彁渚涗簡涓€绉嶅尯鍒嗗畠浠殑鏂规硶銆?
  - 绫诲瀷鐗瑰畾鐨勫瓧娈碉紙WATCH_INFO_TYPE_INFO锛夈€傚畠鐢遍€氱煡鐢熶骇鑰呰缃紝鐢ㄤ簬琛ㄧず鐗瑰畾浜庤绫诲瀷鍜屽瓙绫诲瀷鐨勬煇浜涘惈涔夈€?
info 涓櫎闀垮害涔嬪鐨勬墍鏈夊唴瀹归兘鍙敤浜庤繃婊ゃ€?
澶撮儴涔嬪悗鍙互璺熼殢琛ュ厖淇℃伅銆傚叾鏍煎紡鐢辩被鍨嬪拰瀛愮被鍨嬭嚜琛屽畾涔夈€?

## 鐩戣鍒楄〃锛堥€氱煡鏉ユ簮锛堿PI


銆寃atch list銆嶏紙鐩戣鍒楄〃锛夋槸璁㈤槄浜嗘煇涓€氱煡鏉ユ簮鐨勭洃瑙嗚€呭垪琛ㄣ€備竴涓垪琛ㄥ彲浠ラ檮鍔犲埌鏌愪釜瀵硅薄涓婏紙渚嬪瀵嗛挜鎴栬秴绾у潡锛夛紝涔熷彲浠ユ槸鍏ㄥ眬鐨勶紙渚嬪鐢ㄤ簬璁惧浜嬩欢锛夈€備粠鐢ㄦ埛绌洪棿鐨勮搴︽潵鐪嬶紝闈炲叏灞€鐨勭洃瑙嗗垪琛ㄩ€氬父閫氳繃鍏舵墍灞炲璞＄殑寮曠敤鏉ユ寚浠ｏ紙渚嬪浣跨敤 KEYCTL_NOTIFY 骞剁粰瀹氫竴涓瘑閽ュ簭鍒楀彿鏉ョ洃瑙嗛偅涓壒瀹氱殑瀵嗛挜锛夈€?
瑕佺鐞嗙洃瑙嗗垪琛紝鎻愪緵浜嗕互涓嬪嚱鏁帮細


```

	void init_watch_list(struct watch_list *wlist,
			     void (*release_watch)(struct watch *wlist));

    鍒濆鍖栦竴涓洃瑙嗗垪琛ㄣ€傚鏋?``release_watch`` 涓嶄负 NULL锛屽垯瀹冭〃绀哄湪
    watch_list 瀵硅薄琚攢姣佹椂搴旇皟鐢ㄧ殑涓€涓嚱鏁帮紝鐢ㄤ簬閲婃斁鐩戣鍒楄〃瀵硅鐩戣瀵硅薄
    鎸佹湁鐨勪换浣曞紩鐢ㄣ€?
  * ``void remove_watch_list(struct watch_list *wlist);``

    绉婚櫎璁㈤槄鍒版煇涓?watch_list 鐨勬墍鏈夌洃瑙嗗苟閲婃斁瀹冧滑锛岀劧鍚庨攢姣?    watch_list 瀵硅薄鏈韩銆?

```
## 鐩戣闃熷垪锛堥€氱煡杈撳嚭锛堿PI


銆寃atch queue銆嶏紙鐩戣闃熷垪锛夋槸搴旂敤绋嬪簭鍒嗛厤鐨勪竴娈电紦鍐插尯锛岄€氱煡璁板綍灏嗚鍐欏叆鍏朵腑銆傚叾杩愪綔瀹屽叏闅愯棌鍦ㄧ閬撹澶囬┍鍔ㄥ唴閮紝浣嗚璁剧疆鐩戣锛屽繀椤昏幏鍙栧瀹冪殑寮曠敤銆傚彲浠ラ€氳繃浠ヤ笅鏂瑰紡绠＄悊锛?
  - `struct watch_queue *get_watch_queue(int fd);`

    鐢变簬鐩戣闃熷垪鏄€氳繃瀹炵幇璇ョ紦鍐插尯鐨勭閬撶殑 fd 鍚戝唴鏍告爣璇嗙殑锛岀敤鎴风┖闂村繀椤婚€氳繃绯荤粺璋冪敤浼犻€掕 fd銆傝繖鍙敤浜庝粠绯荤粺璋冪敤涓煡鎵惧埌鐩戣闃熷垪鐨勪笉閫忔槑鎸囬拡銆?
  - `void put_watch_queue(struct watch_queue *wqueue);`

    杩欏皢涓㈠純浠?`get_watch_queue()` 鑾峰緱鐨勫紩鐢ㄣ€?

## 鐩戣璁㈤槄 API


銆寃atch銆嶏紙鐩戣锛夋槸鐩戣鍒楄〃涓婄殑涓€涓闃咃紝瀹冩寚鏄庝簡搴斿皢閫氱煡璁板綍鍐欏叆鍏朵腑鐨勭洃瑙嗛槦鍒楋紙涔熷氨鏄紦鍐插尯锛夈€傜洃瑙嗛槦鍒楀璞′篃鍙互鎼哄甫璇ュ璞＄殑杩囨护瑙勫垯锛岃繖浜涜鍒欑敱
```

	struct watch {
		union {
			u32		info_id;	/* ID to be OR'd in to info field */
			...
		};
		void			*private;	/* Private data for the watched object */
		u64			id;		/* Internal identifier */
		...
	};

```
`info_id` 鍊煎簲鏄竴涓粠鐢ㄦ埛绌洪棿鑾峰緱鐨?8 浣嶆暟锛屽苟宸︾Щ WATCH_INFO_ID__SHIFT銆傚綋閫氱煡琚啓鍏ュ叧鑱旂殑鐩戣闃熷垪缂撳啿鍖烘椂锛屽畠浼氶€氳繃 OR 杩愮畻骞跺叆 **struct watch_notification** 鐨?info 瀛楁涓殑 WATCH_INFO_ID銆?
`private` 瀛楁鏄笌 watch_list 鍏宠仈鐨勯┍鍔ㄦ暟鎹紝鐢?**``watch_list** : release_watch()`` 鏂规硶娓呯悊銆?
`id` 瀛楁鏄潵婧愮殑 ID銆備互涓嶅悓 ID 鍙戝竷鐨勯€氱煡浼氳蹇界暐銆?
鎻愪緵浠ヤ笅鍑芥暟鏉ョ鐞嗙洃瑙嗭細

  - `void init_watch(struct watch **watch, struct watch_queue **wqueue);`

    鍒濆鍖栦竴涓洃瑙嗗璞★紝灏嗗叾鎸囬拡璁剧疆涓虹洃瑙嗛槦鍒楋紝骞朵娇鐢ㄩ€傚綋鐨勫睆闅滀互閬垮厤 lockdep 璀﹀憡銆?
  - `int add_watch_to_object(struct watch **watch, struct watch_list **wlist);`

    灏嗙洃瑙嗚闃呭埌鐩戣鍒楄〃锛堥€氱煡鏉ユ簮锛夈€傚湪璋冪敤姝ゅ嚱鏁颁箣鍓嶏紝watch 缁撴瀯浣撲腑椹卞姩鍙缃殑瀛楁蹇呴』宸茬粡璁剧疆濂姐€?
```

	int remove_watch_from_object(struct watch_list *wlist,
				     struct watch_queue *wqueue,
				     u64 id, false);

    浠庣洃瑙嗗垪琛ㄤ腑绉婚櫎涓€涓洃瑙嗭紝鍏朵腑璇ョ洃瑙嗗繀椤诲尮閰嶆寚瀹氱殑鐩戣闃熷垪
    锛坄`wqueue``锛夊拰瀵硅薄鏍囪瘑绗︼紙``id``锛夈€備細鍚戠洃瑙嗛槦鍒楀彂閫佷竴涓€氱煡
    锛坄`WATCH_META_REMOVAL_NOTIFICATION``锛夛紝鎸囩ず璇ョ洃瑙嗗凡琚Щ闄ゃ€?
  * ``int remove_watch_from_object(struct watch_list *wlist, NULL, 0, true);``

    绉婚櫎鐩戣鍒楄〃涓殑鎵€鏈夌洃瑙嗐€傞璁¤繖灏嗗湪閿€姣佷箣鍓嶈璋冪敤锛屽苟涓斿埌姝や负姝?    璇ョ洃瑙嗗垪琛ㄥ鏂扮殑鐩戣搴斿綋宸蹭笉鍙闂€備細鍚戞瘡涓凡璁㈤槄鐩戣鐨勭洃瑙嗛槦鍒?    鍙戦€佷竴涓€氱煡锛坄`WATCH_META_REMOVAL_NOTIFICATION``锛夛紝鎸囩ず璇ョ洃瑙?    宸茶绉婚櫎銆?

```
## 閫氱煡鍙戝竷 API


瑕佸皢閫氱煡鍙戝竷鍒扮洃瑙嗗垪琛紝浠ヤ究璁㈤槄鐨勭洃瑙嗗彲浠ョ湅鍒板畠锛?```

	void post_watch_notification(struct watch_list *wlist,
				     struct watch_notification *n,
				     const struct cred *cred,
				     u64 id);

```
閫氱煡搴旈鍏堟牸寮忓寲锛屽苟浼犲叆鎸囧悜澶撮儴锛坄n`锛夌殑鎸囬拡銆傞€氱煡鍙兘澶т簬姝ゅぇ灏忥紝浠ョ紦鍐插尯妲戒綅涓哄崟浣嶇殑灏哄璁板綍鍦?`n->info & WATCH_INFO_LENGTH` 涓€?
`cred` 缁撴瀯浣撹〃绀烘潵婧愶紙涓讳綋锛夌殑鍑瘉锛屽畠琚紶閫掔粰 LSM锛堝 SELinux锛夛紝浠ユ牴鎹闃熷垪锛堝璞★級鐨勫嚟璇佸厑璁告垨鎶戝埗鍦ㄥ悇闃熷垪涓褰曡閫氱煡銆?
`id` 鏄潵婧愬璞＄殑 ID锛堜緥濡傚瘑閽ヤ笂鐨勫簭鍒楀彿锛夈€傚彧鏈夎缃簡鐩稿悓 ID 鐨勭洃瑙嗘墠鑳界湅鍒版閫氱煡銆?

## 鐩戣鏉ユ簮


浠讳綍鐗瑰畾鐨勭紦鍐插尯閮藉彲浠ョ敱澶氫釜鏉ユ簮鎻愪緵鏁版嵁銆傛潵婧愬寘鎷細

  - WATCH_TYPE_KEY_NOTIFY

    姝ょ被閫氱煡琛ㄧず瀵嗛挜鍜屽瘑閽ョ幆鐨勫彉鏇达紝鍖呮嫭瀵嗛挜鐜唴瀹圭殑鍙樻洿鎴栧瘑閽ュ睘鎬х殑鍙樻洿銆?
    鏇村淇℃伅璇峰弬闃?Documentation/security/keys/core.rst銆?

## 浜嬩欢杩囨护


涓€鏃﹀垱寤轰簡鐩戣闃熷垪锛屽氨鍙互搴旂敤涓€缁勮繃婊ゅ櫒鏉ラ檺鍒?```

	struct watch_notification_filter filter = {
		...
	};
	ioctl(fd, IOC_WATCH_QUEUE_SET_FILTER, &filter)

```
```

	struct watch_notification_filter {
		__u32	nr_filters;
		__u32	__reserved;
		struct watch_notification_type_filter filters[];
	};

```
鍏朵腑 "nr_filters" 鏄?filters[] 涓繃婊ゅ櫒鐨勬暟閲忥紝"__reserved"
```

	struct watch_notification_type_filter {
		__u32	type;
		__u32	info_filter;
		__u32	info_mask;
		__u32	subtype_filter[8];
	};

```
鍏朵腑锛?
  - `type` 鏄杩囨护鐨勪簨浠剁被鍨嬶紝搴斾负绫讳技
    "WATCH_TYPE_KEY_NOTIFY" 鐨勫€?
  - `info_filter` 鍜?`info_mask` 鐢ㄤ綔瀵?info 瀛楁鐨勮繃婊ゅ櫒锛屽叾
```

	(watch.info & info_mask) == info_filter

    渚嬪锛岃繖鍙敤浜庡拷鐣ラ偅浜涗笉鍦ㄦ寕杞芥爲涓鐩戣鐐逛笂鐨勪簨浠躲€?
  * ``subtype_filter`` 鏄竴涓綅鎺╃爜锛屾寚绀烘劅鍏磋叮鐨勫瓙绫诲瀷銆俿ubtype_filter[0]
    鐨勭 0 浣嶅搴斿瓙绫诲瀷 0锛岀 1 浣嶅搴斿瓙绫诲瀷 1锛屼緷姝ょ被鎺ㄣ€?
```
濡傛灉 ioctl() 鐨勫弬鏁颁负 NULL锛屽垯杩囨护鍣ㄥ皢琚Щ闄わ紝鏉ヨ嚜琚洃瑙嗘潵婧愮殑鎵€鏈変簨浠堕兘灏嗛€氳繃銆?

## 鐢ㄦ埛绌洪棿浠ｇ爜绀轰緥


```

	pipe2(fds, O_TMPFILE);
	ioctl(fds[1], IOC_WATCH_QUEUE_SET_SIZE, 256);

```
```

	keyctl(KEYCTL_WATCH_KEY, KEY_SPEC_SESSION_KEYRING, fds[1], 0x01);

```
```

	static void consumer(int rfd, struct watch_queue_buffer *buf)
	{
		unsigned char buffer[128];
		ssize_t buf_len;

		while (buf_len = read(rfd, buffer, sizeof(buffer)),
		       buf_len > 0
		       ) {
			void *p = buffer;
			void *end = buffer + buf_len;
			while (p < end) {
				union {
					struct watch_notification n;
					unsigned char buf1[128];
				} n;
				size_t largest, len;

				largest = end - p;
				if (largest > 128)
					largest = 128;
				memcpy(&n, p, largest);

				len = (n->info & WATCH_INFO_LENGTH) >>
					WATCH_INFO_LENGTH__SHIFT;
				if (len == 0 || len > largest)
					return;

				switch (n.n.type) {
				case WATCH_TYPE_META:
					got_meta(&n.n);
				case WATCH_TYPE_KEY_NOTIFY:
					saw_key_change(&n.n);
					break;
				}

				p += len;
			}
		}
	}

```
