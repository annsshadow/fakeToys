## Configfs 鈥斺€?鐢辩敤鎴风┖闂撮┍鍔ㄧ殑鍏ф牳瀵硅薄閰嶇疆


Joel Becker <joel.becker@oracle.com>

鏇存柊锛?005 骞?3 鏈?31 鏃?

Copyright (c) 2005 Oracle Corporation,
	Joel Becker <joel.becker@oracle.com>


## 浠€涔堟槸 configfs锛?


configfs 鏄竴涓熀浜庡唴瀛樼殑鏂囦欢绯荤粺锛屾彁渚涗笌 sysfs 鍔熻兘鐩稿弽鐨勬湇鍔°€俿ysfs
鏄熀浜庢枃浠剁郴缁熺殑鍐呮牳瀵硅薄瑙嗗浘锛岃€?configfs 鏄熀浜庢枃浠剁郴缁熺殑鍐呮牳瀵硅薄锛堝嵆
config_items锛夌鐞嗗櫒銆?

浣跨敤 sysfs 鏃讹紝瀵硅薄鍦ㄥ唴鏍镐腑鍒涘缓锛堜緥濡傦紝鍦ㄥ彂鐜拌澶囨椂锛夛紝骞跺悜 sysfs 娉ㄥ唽銆?
闅忓悗瀹冪殑灞炴€т究浼氬嚭鐜板湪 sysfs 涓紝鍏佽鐢ㄦ埛绌洪棿閫氳繃 readdir(3)/read(2) 璇诲彇
杩欎簺灞炴€с€傚畠鍙兘鍏佽閫氳繃 write(2) 淇敼鏌愪簺灞炴€с€傝鐐瑰湪浜庯紝瀵硅薄鍦ㄥ唴鏍镐腑
鍒涘缓鍜岄攢姣侊紝鍐呮牳鎺у埗鐫€ sysfs 琛ㄧず鐨勭敓瀛樺懆鏈燂紝鑰?sysfs 浠呬粎鏄繖涓€鍒囩殑涓€鎵?
绐楀彛銆?

涓€涓?configfs 鐨?config_item 閫氳繃鏄惧紡鐨勭敤鎴风┖闂存搷浣?mkdir(2) 鍒涘缓锛屽苟閫氳繃
rmdir(2) 閿€姣併€傚睘鎬у湪 mkdir(2) 鏃跺嵆鍑虹幇锛屽苟鍙€氳繃 read(2) 鍜?write(2) 璇诲彇
鎴栦慨鏀广€備笌 sysfs 涓€鏍凤紝readdir(3) 鏌ヨ椤瑰拰/鎴栧睘鎬х殑鍒楄〃銆俿ymlink(2) 鍙敤浜?
灏嗛」鍒嗙粍鍦ㄤ竴璧枫€備笌 sysfs 涓嶅悓鐨勬槸锛岃〃绀虹殑鐢熷瓨鍛ㄦ湡瀹屽叏鐢辩敤鎴风┖闂撮┍鍔ㄣ€傛敮鎾?
杩欎簺椤圭殑鍏ф牳妯″潡蹇呴』瀵规浣滃嚭鍝嶅簲銆?

sysfs 鍜?configfs 鍙互骞朵笖搴旇鍏卞瓨浜庡悓涓€绯荤粺涓€備簩鑰呬簰涓鸿ˉ鍏咃紝骞堕潪鏇夸唬鍏崇郴銆?

## 浣跨敤 configfs


configfs 鍙互缂栬瘧涓烘ā鍧楁垨缂栧叆鍐呮牳銆備綘鍙互閫氳繃浠ヤ笅鏂瑰紡璁块棶瀹冿細

```
	mount -t configfs none /config
```

闄ら潪鍚屾椂鍔犺浇浜嗗鎴风妯″潡锛屽惁鍒?configfs 鏍戝皢鏄┖鐨勩€傝繖浜涙ā鍧椾綔涓哄瓙绯荤粺鍚?
configfs 娉ㄥ唽浜嗗畠浠殑椤圭被鍨嬨€備竴鏃﹀鎴风瀛愮郴缁熻鍔犺浇锛屽畠灏变細浣滀负 /config
涓嬬殑涓€涓紙鎴栧涓級瀛愮洰褰曞嚭鐜般€備笌 sysfs 涓€鏍凤紝鏃犺鏄惁鎸傝浇鍒?/config锛?
configfs 鏍戝缁堝瓨鍦ㄣ€?

涓€椤归€氳繃 mkdir(2) 鍒涘缓銆傝椤圭殑灞炴€т篃浼氬湪姝ゆ椂鍑虹幇銆俽eaddir(3) 鍙互纭畾鏈夊摢浜?
灞炴€э紝read(2) 鍙互鏌ヨ瀹冧滑鐨勯粯璁ゅ€硷紝write(2) 鍙互瀛樺偍鏂板€笺€備笉瑕佸湪涓€涓睘鎬?
鏂囦欢涓贩鍏ュ涓睘鎬с€?

configfs 鏈変袱绉嶇被鍨嬬殑灞炴€э細

- 鏅€氬睘鎬э紝涓?sysfs 灞炴€х被浼硷紝鏄皬鐨?ASCII 鏂囨湰鏂囦欢锛屾渶澶уぇ灏忎负鍗曢〉
  锛圥AGE_SIZE锛屽湪 i386 涓婁负 4096锛夈€傛渶濂芥瘡涓枃浠跺彧浣跨敤涓€涓€硷紝骞朵笖閫傜敤涓?
  sysfs 鐩稿悓鐨勬敞鎰忎簨椤广€俢onfigfs 鏈熸湜 write(2) 涓€娆℃€у瓨鍌ㄦ暣涓紦鍐插尯銆傚湪鍚?
  鏅€?configfs 灞炴€у啓鍏ユ椂锛岀敤鎴风┖闂磋繘绋嬪簲鍏堣鍙栨暣涓枃浠讹紝淇敼鍏跺笇鏈涙洿鏀圭殑
  閮ㄥ垎锛岀劧鍚庡皢鏁翠釜缂撳啿鍖哄啓鍥炪€?

- 浜岃繘鍒跺睘鎬э紝涓?sysfs 浜岃繘鍒跺睘鎬ф湁浜涚被浼硷紝浣嗚涔変笂鏈変竴浜涚粏寰彉鍖栥€侾AGE_SIZE
  鐨勯檺鍒朵笉閫傜敤锛屼絾鏁翠釜浜岃繘鍒堕」蹇呴』鑳芥斁鍏ュ崟涓唴鏍?vmalloc 缂撳啿鍖轰腑銆傛潵鑷敤鎴?
  绌洪棿鐨?write(2) 璋冪敤浼氳缂撳啿锛屽睘鎬х殑 write_bin_attribute 鏂规硶灏嗗湪鏈€鍚庝竴娆?
  鍏抽棴鏃惰璋冪敤锛屽洜姝ょ敤鎴风┖闂村繀椤绘鏌?close(2) 鐨勮繑鍥炵爜锛屼互纭鎿嶄綔宸叉垚鍔?
  瀹屾垚銆備负浜嗛伩鍏嶆伓鎰忕敤鎴蜂娇鍐呮牳 OOM锛屾瘡涓簩杩涘埗灞炴€ч兘鏈変竴涓渶澶х紦鍐插尯鍊笺€?

褰撴煇椤归渶瑕佽閿€姣佹椂锛屼娇鐢?rmdir(2) 灏嗗叾绉婚櫎銆傚鏋滄湁浠讳綍鍏朵粬椤归€氳繃 symlink(2)
閾炬帴鍒板畠锛屽垯璇ラ」涓嶈兘琚攢姣併€傞摼鎺ュ彲浠ラ€氳繃 unlink(2) 绉婚櫎銆?

## 閰嶇疆 FakeNBD锛氫竴涓ず渚?


璁炬兂鏈変竴涓綉缁滃潡璁惧锛圢BD锛夐┍鍔紝鍏佽浣犺闂繙绋嬪潡璁惧銆傜О涔嬩负 FakeNBD銆?
FakeNBD 浣跨敤 configfs 杩涜閰嶇疆銆傛樉鐒讹紝浼氭湁涓€涓緢濂界殑绋嬪簭渚涚郴缁熺鐞嗗憳鐢ㄦ潵
閰嶇疆 FakeNBD锛屼絾璇ョ▼搴忔€诲緱浠ユ煇绉嶆柟寮忓皢閰嶇疆鍛婄煡椹卞姩銆傝繖灏辨槸 configfs 鐨勭敤姝?
涔嬪湴銆?

褰?FakeNBD 椹卞姩琚姞杞芥椂锛屽畠浼氬悜 configfs 娉ㄥ唽鑷繁銆?

```
	# ls /config
	fakenbd
```

涓€涓?fakenbd 杩炴帴鍙互閫氳繃 mkdir(2) 鍒涘缓銆傚悕绉版槸浠绘剰鐨勶紝浣嗗伐鍏峰彲鑳戒細鍒╃敤
璇ュ悕绉般€備篃璁革細

```
	# mkdir /config/fakenbd/disk1
	# ls /config/fakenbd/disk1
	target device rw
```

target 灞炴€у寘鍚?FakeNBD 灏嗚杩炴帴鐨勬湇鍔″櫒 IP 鍦板潃銆俤evice 灞炴€ф槸鏈嶅姟鍣ㄤ笂鐨?
璁惧銆傚彲浠ラ瑙侊紝rw 灞炴€у喅瀹氳杩炴帴鏄惁

```
	# echo 10.0.0.1 > /config/fakenbd/disk1/target
	# echo /dev/sda1 > /config/fakenbd/disk1/device
	# echo 1 > /config/fakenbd/disk1/rw
```

灏辫繖鏍枫€備粎姝よ€屽凡銆傜幇鍦ㄨ澶囧凡缁忛厤缃ソ浜嗭紝鑰屼笖杩樻槸閫氳繃 shell 瀹屾垚鐨勩€?

## 鐢?configfs 缂栫爜


configfs 涓殑姣忎釜瀵硅薄閮芥槸涓€涓?config_item銆備竴涓?config_item 鍙嶆槧浜嗗瓙绯荤粺
涓殑涓€涓璞°€傚畠鍏锋湁涓庡璞′笂鍊肩浉瀵瑰簲鐨勫睘鎬с€俢onfigfs 澶勭悊璇ュ璞″強鍏跺睘鎬х殑
鏂囦欢绯荤粺琛ㄧず锛屼娇寰楀瓙绯荤粺鍙渶鍏虫敞鍩烘湰鐨?show/store 浜や簰銆?

椤瑰湪 config_group 鍐呴儴鍒涘缓鍜岄攢姣併€備竴涓粍鏄竴缁勫叡浜浉鍚屽睘鎬у拰鎿嶄綔鐨勯」鐨勯泦鍚堛€?
椤归€氳繃 mkdir(2) 鍒涘缓銆侀€氳繃 rmdir(2) 绉婚櫎锛屼絾杩欑敱 configfs 澶勭悊銆傝缁勬湁涓€缁?
鎵ц杩欎簺鎿嶄綔鐨勬柟娉曘€?

瀛愮郴缁熸槸瀹㈡埛绔ā鍧楃殑椤跺眰銆傚湪鍒濆鍖栨湡闂达紝瀹㈡埛绔ā鍧楀悜 configfs 娉ㄥ唽瀛愮郴缁燂紝
璇ュ瓙绯荤粺浣滀负 configfs 鏂囦欢绯荤粺椤跺眰鐨勭洰褰曞嚭鐜般€傚瓙绯荤粺鍚屾椂涔熸槸涓€涓?config_group锛?
骞朵笖鍙互瀹屾垚 config_group 鑳藉仛鐨勬墍鏈変簨鎯呫€?

## struct config_item


```
	struct config_item {
		char                    *ci_name;
		char                    ci_namebuf[UOBJ_NAME_LEN];
		struct kref             ci_kref;
		struct list_head        ci_entry;
		struct config_item      *ci_parent;
		struct config_group     *ci_group;
		struct config_item_type *ci_type;
		struct dentry           *ci_dentry;
	};

	void config_item_init(struct config_item *);
	void config_item_init_type_name(struct config_item *,
					const char *name,
					struct config_item_type *type);
	struct config_item *config_item_get(struct config_item *);
	void config_item_put(struct config_item *);
```

閫氬父锛宻truct config_item 宓屽叆鍦ㄤ竴涓鍣ㄧ粨鏋勪腑锛岃缁撴瀯瀹為檯浠ｈ〃浜嗗瓙绯荤粺姝ｅ湪
鍋氱殑浜嬫儏銆傝缁撴瀯鐨?config_item 閮ㄥ垎灏辨槸瀵硅薄涓?configfs 浜や簰鐨勬柟寮忋€?

鏃犺鏄潤鎬佸畾涔夊湪婧愭枃浠朵腑锛岃繕鏄敱鐖?config_group 鍒涘缓锛屼竴涓?config_item 閮藉繀椤?
璋冪敤鍏朵腑涓€涓?_init() 鍑芥暟銆傝繖浼氬垵濮嬪寲寮曠敤璁℃暟骞惰缃浉搴旂殑瀛楁銆?

鎵€鏈変娇鐢?config_item 鐨勫湴鏂归兘搴旇閫氳繃 config_item_get() 鎸佹湁瀹冪殑涓€涓紩鐢紝
骞跺湪鐢ㄥ畬鍚庨€氳繃 config_item_put() 閲婃斁寮曠敤銆?

鍗曞嚟鑷韩锛屼竴涓?config_item 闄や簡鍑虹幇鍦?configfs 涓箣澶栧仛涓嶄簡澶浜嬫儏銆傞€氬父瀛愮郴缁?
甯屾湜璇ラ」鏄剧ず鍜?鎴栧瓨鍌ㄥ睘鎬х瓑銆備负姝わ紝瀹冮渶瑕佷竴涓被鍨嬨€?

## struct config_item_type


```
	struct configfs_item_operations {
		void (*release)(struct config_item *);
		int (*allow_link)(struct config_item *src,
				  struct config_item *target);
		void (*drop_link)(struct config_item *src,
				 struct config_item *target);
	};

	struct config_item_type {
		struct module                           *ct_owner;
		struct configfs_item_operations         *ct_item_ops;
		struct configfs_group_operations        *ct_group_ops;
		struct configfs_attribute               **ct_attrs;
		struct configfs_bin_attribute		**ct_bin_attrs;
	};
```

config_item_type 鏈€鍩烘湰鐨勫姛鑳芥槸瀹氫箟鍙互鍦?config_item 涓婃墽琛屽摢浜涙搷浣溿€傛墍鏈?
鍔ㄦ€佸垎閰嶇殑椤圭殑閮介渶瑕佹彁渚?ct_item_ops->release() 鏂规硶銆傚綋 config_item 鐨勫紩鐢ㄨ鏁?
杈惧埌闆舵椂浼氳皟鐢ㄨ鏂规硶銆?

## struct configfs_attribute


```
	struct configfs_attribute {
		char                    *ca_name;
		struct module           *ca_owner;
		umode_t                  ca_mode;
		ssize_t (*show)(struct config_item *, char *);
		ssize_t (*store)(struct config_item *, const char *, size_t);
	};
```

褰撲竴涓?config_item 甯屾湜鏌愪釜灞炴€т綔涓烘枃浠跺嚭鐜板湪鍏?configfs 鐩綍涓椂锛屽畠蹇呴』
瀹氫箟涓€涓弿杩板畠鐨?configfs_attribute銆傜劧鍚庡畠灏嗗睘鎬ф坊鍔犲埌浠?NULL 缁撳熬鐨勬暟缁?
config_item_type->ct_attrs 涓€傚綋璇ラ」鍑虹幇鍦?configfs 涓椂锛屽睘鎬ф枃浠跺皢浠?
configfs_attribute->ca_name 浣滀负鏂囦欢鍚嶅嚭鐜般€俢onfigfs_attribute->ca_mode 鎸囧畾浜?
鏂囦欢鏉冮檺銆?

濡傛灉涓€涓睘鎬ф槸鍙鐨勫苟涓旀彁渚涗簡 ->show 鏂规硶锛岄偅涔堟瘡褰撶敤鎴风┖闂村璇ュ睘鎬ц姹?
read(2) 鏃堕兘浼氳皟鐢ㄨ鏂规硶銆傚鏋滀竴涓睘鎬ф槸鍙啓鐨勫苟涓旀彁渚涗簡 ->store 鏂规硶锛岄偅涔?
姣忓綋鐢ㄦ埛绌洪棿瀵硅灞炴€ц姹?write(2) 鏃堕兘浼氳皟鐢ㄨ鏂规硶銆?

## struct configfs_bin_attribute


```
	struct configfs_bin_attribute {
		struct configfs_attribute	cb_attr;
		void				*cb_private;
		size_t				cb_max_size;
	};
```

褰撻渶瑕佷娇鐢ㄤ簩杩涘埗 blob 浣滀负椤瑰湪鍏?configfs 鐩綍涓枃浠剁殑鍐呭鏃讹紝灏变細鐢ㄥ埌浜岃繘鍒?
灞炴€с€備负姝わ紝灏嗕簩杩涘埗灞炴€ф坊鍔犲埌浠?NULL 缁撳熬鐨勬暟缁?config_item_type->ct_bin_attrs
涓紝褰撹椤瑰嚭鐜板湪 configfs 涓椂锛屽睘鎬ф枃浠跺皢浠?configfs_bin_attribute->cb_attr.ca_name
浣滀负鏂囦欢鍚嶅嚭鐜般€俢onfigfs_bin_attribute->cb_attr.ca_mode 鎸囧畾浜嗘枃浠舵潈闄愩€俢b_private
鎴愬憳渚涢┍鍔ㄤ娇鐢紝鑰?cb_max_size 鎴愬憳鎸囧畾浜嗚浣跨敤鐨?vmalloc 缂撳啿鍖虹殑鏈€澶уぇ灏忋€?

濡傛灉浜岃繘鍒跺睘鎬ф槸鍙鐨勶紝骞朵笖 config_item 鎻愪緵浜?ct_item_ops->read_bin_attribute()
鏂规硶锛岄偅涔堟瘡褰撶敤鎴风┖闂村璇ュ睘鎬ц姹?read(2) 鏃堕兘浼氳皟鐢ㄨ鏂规硶銆倃rite(2) 鐨勬儏鍐?
鐩稿弽銆傝/鍐欐槸缂撳啿鐨勶紝鍥犳鍙細鍙戠敓鍗曟璇?鍐欙紱灞炴€ф棤闇€鍏冲績杩欎竴鐐广€?

## struct config_group


涓€涓?config_item 涓嶈兘瀛ょ珛瀛樺湪銆傚垱寤哄畠鐨勫敮涓€鏂瑰紡鏄湪 config_group 涓婃墽琛?
mkdir(2)銆傝繖浼氳Е鍙戝垱寤轰竴涓細

```
	struct config_group {
		struct config_item		cg_item;
		struct list_head		cg_children;
		struct configfs_subsystem 	*cg_subsys;
		struct list_head		default_groups;
		struct list_head		group_entry;
	};

	void config_group_init(struct config_group *group);
	void config_group_init_type_name(struct config_group *group,
					 const char *name,
					 struct config_item_type *type);
```

config_group 缁撴瀯鍖呭惈涓€涓?config_item銆傛纭厤缃椤规剰鍛崇潃璇ョ粍鏈韩鍙互鍍忎竴涓?
椤逛竴鏍峰伐浣溿€備笉杩囷紝瀹冭兘鍋氱殑鏇村锛氬畠鍙互鍒涘缓瀛愰」鎴栧瓙缁勩€傝繖鏄€氳繃鍦ㄨ缁勭殑
group 鎿嶄綔涓寚瀹氱殑鏂规硶鏉ュ畬鎴愮殑锛?

```
	struct configfs_group_operations {
		struct config_item *(*make_item)(struct config_group *group,
						 const char *name);
		struct config_group *(*make_group)(struct config_group *group,
						   const char *name);
		void (*disconnect_notify)(struct config_group *group,
					  struct config_item *item);
		void (*drop_item)(struct config_group *group,
				  struct config_item *item);
	};
```

涓€涓粍閫氳繃鎻愪緵 ct_group_ops->make_item() 鏂规硶鏉ュ垱寤哄瓙椤广€傚鏋滄彁渚涗簡璇ユ柟娉曪紝
瀹冧細鍦ㄨ缁勭洰褰曚腑鐨?mkdir(2) 鏃惰璋冪敤銆傚瓙绯荤粺鍒嗛厤涓€涓柊鐨?config_item锛堟垨鏇村彲鑳?
鏄叾瀹瑰櫒缁撴瀯锛夛紝鍒濆鍖栧畠锛屽苟灏嗗叾杩斿洖缁?configfs銆俢onfigfs 闅忓悗浼氬～鍏呮枃浠剁郴缁熸爲
浠ュ弽鏄犺繖涓柊椤广€?

濡傛灉瀛愮郴缁熷笇鏈涘瓙椤规湰韬槸涓€涓粍锛屽垯瀛愮郴缁熸彁渚?ct_group_ops->make_group()銆傚叾浠?
涓€鍒囬兘琛ㄧ幇鐩稿悓锛屽湪缁勪笂浣跨敤缁勭殑 _init() 鍑芥暟銆?

鏈€鍚庯紝褰撶敤鎴风┖闂村璇ラ」鎴栫粍璋冪敤 rmdir(2) 鏃讹紝浼氳皟鐢?ct_group_ops->drop_item()銆?
鐢变簬 config_group 涔熸槸涓€涓?config_item锛屽洜姝や笉闇€瑕佸崟鐙殑 drop_group() 鏂规硶銆傚瓙绯荤粺
蹇呴』瀵归」鍒嗛厤鏃跺垵濮嬪寲鐨勫紩鐢ㄦ墽琛?config_item_put()銆傚鏋滃瓙绯荤粺鏃犱簨鍙仛锛屽畠鍙互鐪佺暐
ct_group_ops->drop_item() 鏂规硶锛宑onfigfs 灏嗕唬琛ㄥ瓙绯荤粺瀵硅椤硅皟鐢?config_item_put()銆?

閲嶈锛?
   drop_item() 鏄?void 绫诲瀷锛屽洜姝ゆ棤娉曞け璐ャ€傚綋璋冪敤 rmdir(2) 鏃讹紝configfs 灏嗘妸璇ラ」
   浠庢枃浠剁郴缁熸爲涓Щ闄わ紙鍓嶆彁鏄畠娌℃湁闇€瑕佷繚鎸佸繖纰岀殑瀛愰」锛夈€傚瓙绯荤粺璐熻矗瀵规浣滃嚭
   鍝嶅簲銆傚鏋滃瓙绯荤粺鍦ㄥ叾浠栫嚎绋嬩腑鎸佹湁瀵硅椤圭殑寮曠敤锛屽唴瀛樻槸瀹夊叏鐨勩€傝椤瑰疄闄呬粠瀛愮郴缁熺殑
   浣跨敤涓秷澶卞彲鑳介渶瑕佷竴浜涙椂闂淬€備絾瀹冨凡缁忎粠 configfs 涓秷澶变簡銆?

褰撹皟鐢?drop_item() 鏃讹紝椤圭殑閾炬帴鍏崇郴宸茬粡琚媶闄ゃ€傚畠涓嶅啀鎸佹湁鍏剁埗椤圭殑寮曠敤锛屼篃
涓嶅湪椤圭殑灞傛缁撴瀯涓崰鏈変竴甯箣鍦般€傚鏋滃鎴风闇€瑕佸湪鎷嗛櫎鍙戠敓涔嬪墠鍋氫竴浜涙竻鐞嗗伐浣滐紝
瀛愮郴缁熷彲浠ュ疄鐜?ct_group_ops->disconnect_notify() 鏂规硶銆傝鏂规硶鍦?configfs 宸插皢椤?
浠庢枃浠剁郴缁熻鍥句腑绉婚櫎涔嬪悗銆佷絾鍦ㄨ椤逛粠鍏剁埗缁勪腑绉婚櫎涔嬪墠琚皟鐢ㄣ€備笌 drop_item() 涓€鏍凤紝
disconnect_notify() 鏄?void 绫诲瀷涓斾笉鑳藉け璐ャ€傚鎴风瀛愮郴缁熶笉搴斿湪姝ゅ閲婃斁浠讳綍寮曠敤锛?
鍥犱负瀹冧滑浠嶇劧蹇呴』鍦?drop_item() 涓墽琛屻€?

鍙 config_group 浠嶇劧鎷ユ湁瀛愰」锛屽氨涓嶈兘琚Щ闄ゃ€傝繖鏄湪 configfs 鐨?rmdir(2) 浠ｇ爜涓?
瀹炵幇鐨勩€?>drop_item() 涓嶄細琚皟鐢紝鍥犱负椤瑰皻鏈涓㈠純銆俽mdir(2) 浼氬け璐ワ紝鍥犱负鐩綍
闈炵┖銆?

## struct configfs_subsystem


涓€涓瓙绯荤粺蹇呴』娉ㄥ唽鑷繁锛岄€氬父鍦?module_init 鏃躲€傝繖鏄€氳繃锛?

```
	struct configfs_subsystem {
		struct config_group	su_group;
		struct mutex		su_mutex;
	};

	int configfs_register_subsystem(struct configfs_subsystem *subsys);
	void configfs_unregister_subsystem(struct configfs_subsystem *subsys);
```

涓€涓瓙绯荤粺鐢变竴涓《灞?config_group 鍜屼竴涓?mutex 缁勬垚銆傝缁勬槸鍒涘缓瀛?config_item
鐨勫湴鏂广€傚浜庝竴涓瓙绯荤粺锛岃繖涓粍閫氬父鏄潤鎬佸畾涔夌殑銆傚湪璋冪敤
configfs_register_subsystem() 涔嬪墠锛屽瓙绯荤粺蹇呴』閫氳繃閫氬父鐨勭粍 _init() 鍑芥暟鍒濆鍖栬
缁勶紝骞朵笖杩樺繀椤诲垵濮嬪寲 mutex銆?

褰撴敞鍐岃皟鐢ㄨ繑鍥炴椂锛屽瓙绯荤粺灏卞浜庢椿鍔ㄧ姸鎬侊紝骞朵笖鍙互閫氳繃 configfs 鐪嬪埌銆傛鏃讹紝鍙互
璋冪敤 mkdir(2)锛屽瓙绯荤粺蹇呴』涓烘鍋氬ソ鍑嗗銆?

## 涓€涓ず渚?


杩欎簺鍩烘湰姒傚康鐨勬渶浣崇ず渚嬫槸 samples/configfs/configfs_sample.c 涓殑 simple_children
瀛愮郴缁?缁勫拰 simple_child 椤广€傚畠灞曠ず浜嗕竴涓樉绀哄拰瀛樺偍灞炴€х殑骞冲嚒瀵硅薄锛屼互鍙婁竴涓?
鍒涘缓鍜岄攢姣佽繖浜涘瓙椤圭殑绠€鍗曠粍銆?

## 灞傛瀵艰埅涓庡瓙绯荤粺 Mutex


configfs 杩樻彁渚涗簡涓€涓澶栫殑濂藉銆俢onfig_group 鍜?config_item 鐢变簬鍑虹幇鍦ㄦ枃浠剁郴缁熶腑
鑰屾帓鍒楁垚灞傛缁撴瀯銆傚瓙绯荤粺缁濅笉瑙︾鏂囦欢绯荤粺鐨勯儴鍒嗭紝浣嗗瓙绯荤粺鍙兘瀵硅繖涓眰娆℃劅鍏磋叮銆?
鍥犳锛岃灞傛閫氳繃 config_group->cg_children 鍜?config_item->ci_parent 缁撴瀯鎴愬憳琚?
闀滃儚鍑烘潵銆?

瀛愮郴缁熷彲浠ラ亶鍘?cg_children 鍒楄〃鍜?ci_parent 鎸囬拡鏉ユ煡鐪嬬敱瀛愮郴缁熷垱寤虹殑鏍戙€傝繖鍙兘涓?
configfs 瀵硅灞傛鐨勭鐞嗕骇鐢熺珵浜夛紝鍥犳 configfs 浣跨敤瀛愮郴缁?mutex 鏉ヤ繚鎶や慨鏀广€傛瘡褰?
瀛愮郴缁熸兂瑕侀亶鍘嗚灞傛鏃讹紝瀹冨繀椤诲湪瀛愮郴缁?mutex 鐨勪繚鎶や笅杩涜銆?

鍦?newly allocated item 灏氭湭閾炬帴杩涜灞傛鏃讹紝瀛愮郴缁熷皢琚樆姝㈣幏鍙?mutex銆傜被浼煎湴锛?
鍦?dropping item 灏氭湭琚В闄ら摼鎺ユ椂锛屽畠涔熸棤娉曡幏鍙?mutex銆傝繖鎰忓懗鐫€鍙椤瑰湪 configfs
涓紝鍏?ci_parent 鎸囬拡灏辨案杩滀笉浼氭槸 NULL锛屽苟涓旈」浠呭湪鍚屼竴鏃堕棿娈靛唴瀛樺湪浜庡叾鐖堕」鐨?
cg_children 鍒楄〃涓€傝繖浣垮緱瀛愮郴缁熷湪鎸佹湁 mutex 鏃跺彲浠ヤ俊浠?ci_parent 鍜?cg_children銆?

## 閫氳繃 symlink(2) 杩涜椤硅仛鍚?


configfs 閫氳繃 group->item 鐨勭埗/瀛愬叧绯绘彁渚涗竴涓畝鍗曠殑缁勩€傜劧鑰岋紝鏇村ぇ鐨勭幆澧冨父甯搁渶瑕?
鍦ㄧ埗/瀛愯繛鎺ヤ箣澶栬繘琛岃仛鍚堛€傝繖鏄€氳繃 symlink(2) 瀹炵幇鐨勩€?

涓€涓?config_item 鍙互鎻愪緵 ct_item_ops->allow_link() 鍜?ct_item_ops->drop_link() 鏂规硶銆?
濡傛灉 ->allow_link() 鏂规硶瀛樺湪锛屽氨鍙互浠ヨ config_item 浣滀负閾炬帴婧愭潵璋冪敤 symlink(2)銆?
杩欎簺閾炬帴鍙厑璁稿湪 configfs 鐨?config_item 涔嬮棿寤虹珛銆備换浣曞湪 configfs 鏂囦欢绯荤粺涔嬪鐨?
symlink(2) 灏濊瘯閮藉皢琚嫆缁濄€?

褰撹皟鐢?symlink(2) 鏃讹紝婧?config_item 鐨?->allow_link() 鏂规硶浼氳璋冪敤锛屼紶鍏ュ畠鑷繁鍜?
鐩爣椤广€傚鏋滄簮椤瑰厑璁搁摼鎺ュ埌鐩爣椤癸紝鍒欒繑鍥?0銆傚鏋滄簮椤瑰彧甯屾湜閾炬帴鍒版煇绉嶇壒瀹氱被鍨嬬殑
瀵硅薄锛堜緥濡傚畠鑷繁瀛愮郴缁熷唴鐨勫璞★級锛屽畠鍙兘甯屾湜鎷掔粷涓€涓摼鎺ャ€?

褰撳绗﹀彿閾炬帴璋冪敤 unlink(2) 鏃讹紝婧愰」浼氶€氳繃 ->drop_link() 鏂规硶寰楀埌閫氱煡銆備笌 ->drop_item()
鏂规硶涓€鏍凤紝杩欐槸涓€涓?void 鍑芥暟锛屼笉鑳借繑鍥炲け璐ャ€傚瓙绯荤粺璐熻矗鍝嶅簲杩欎竴鍙樺寲銆?

涓€涓?config_item 鍦ㄩ摼鎺ュ埌浠讳綍鍏朵粬椤规椂涓嶈兘琚Щ闄わ紝鍦ㄦ湁鍏朵粬椤归摼鎺ュ埌瀹冩椂涔熶笉鑳借绉婚櫎銆?
configfs 涓笉鍏佽鎮┖绗﹀彿閾炬帴銆?

## 鑷姩鍒涘缓鐨勫瓙缁?


涓€涓柊鐨?config_group 鍙兘甯屾湜鎷ユ湁涓ょ绫诲瀷鐨勫瓙 config_item銆傝櫧鐒惰繖鍙互閫氳繃
->make_item() 涓殑榄旀硶鍚嶇О鏉ョ紪鐮侊紝浣嗘湁涓€绉嶆柟娉曡鐢ㄦ埛绌洪棿鐪嬪埌杩欑宸紓浼氭洿鍔犳槑纭€?

configfs 涓嶆槸璁╀竴涓粍涓殑鏌愪簺椤硅〃鐜板緱涓庡叾浠栭」涓嶅悓锛岃€屾槸鎻愪緵浜嗕竴绉嶆柟娉曪紝鍗冲湪鐖剁粍
鍒涘缓鏃惰嚜鍔ㄥ湪鍏跺唴閮ㄥ垱寤轰竴涓垨澶氫釜瀛愮粍銆傚洜姝わ紝mkdir("parent") 浼氱敓鎴?"parent"銆?
"parent/subgroup1"锛屼竴鐩村埌 "parent/subgroupN"銆傜被鍨嬩负 1 鐨勯」鐜板湪鍙互鍦?
"parent/subgroup1" 涓垱寤猴紝绫诲瀷涓?N 鐨勯」鍙互鍦?"parent/subgroupN" 涓垱寤恒€?

杩欎簺鑷姩鍒涘缓鐨勫瓙缁勶紙鎴栫О榛樿缁勶級骞朵笉鎺掗櫎鐖剁粍鐨勫叾浠栧瓙椤广€傚鏋?ct_group_ops->make_group()
瀛樺湪锛屽彲浠ョ洿鎺ュ湪鐖剁粍涓婂垱寤哄叾浠栧瓙缁勩€?

涓€涓?configfs 瀛愮郴缁熼€氳繃浣跨敤 configfs_add_default_group() 鍑芥暟灏嗛粯璁ょ粍娣诲姞鍒扮埗
config_group 缁撴瀯鏉ユ寚瀹氬畠浠€傛瘡涓娣诲姞鐨勭粍涓庣埗缁勫湪鍚屼竴鏃堕棿琚～鍏呰繘 configfs 鏍戜腑銆?
绫讳技鍦帮紝瀹冧滑涓庣埗缁勫湪鍚屼竴鏃堕棿琚Щ闄ゃ€備笉鎻愪緵棰濆鐨勯€氱煡銆傚綋 ->drop_item() 鏂规硶璋冪敤
閫氱煡瀛愮郴缁熺埗缁勫嵆灏嗘秷澶辨椂锛岃繖涔熸剰鍛崇潃涓庤鐖剁粍鍏宠仈鐨勬瘡涓€涓粯璁ゅ瓙缁勩€?

鍥犳锛岄粯璁ょ粍涓嶈兘閫氳繃 rmdir(2) 鐩存帴绉婚櫎銆傚湪鐖剁粍涓婃墽琛?rmdir(2) 妫€鏌ュ瓙椤规椂锛屽畠浠?
涔熶笉琚€冭檻銆?

## 渚濊禆瀛愮郴缁?


鏈夋椂鍏朵粬椹卞姩渚濊禆浜庣壒瀹氱殑 configfs 椤广€備緥濡傦紝ocfs2 鎸傝浇渚濊禆浜庝竴涓績璺冲尯鍩熼」銆傚鏋滆
鍖哄煙椤归€氳繃 rmdir(2) 琚Щ闄わ紝ocfs2 鎸傝浇灏卞繀椤?BUG 鎴栬繘鍏ュ彧璇汇€傝繖骞朵笉鐞嗘兂銆?

configfs 鎻愪緵浜嗕袱涓澶栫殑 API 璋冪敤锛歝onfigfs_depend_item() 鍜?configfs_undepend_item()銆?
涓€涓鎴风椹卞姩鍙互鍦ㄤ竴涓凡瀛樺湪鐨勯」涓婅皟鐢?configfs_depend_item() 鏉ュ憡璇?configfs 瀹?
渚濊禆浜庤椤广€俢onfigfs 闅忓悗浼氶拡瀵硅椤圭殑 rmdir(2) 杩斿洖 -EBUSY銆傚綋璇ラ」涓嶅啀琚緷璧栨椂锛?
瀹㈡埛绔┍鍔ㄥ鍏惰皟鐢?configfs_undepend_item()銆?

杩欎簺 API 涓嶈兘鍦?configfs 鐨勪换浣曞洖璋冧箣涓嬭皟鐢紝鍥犱负瀹冧滑浼氬彂鐢熷啿绐併€傚畠浠彲鑳介樆濉炲拰鍒嗛厤銆?
瀹㈡埛绔┍鍔ㄥぇ姒備笉搴旇鍑嚜宸辩殑鎰忔効璋冪敤瀹冧滑锛岃€屽簲璇ユ彁渚涗竴涓緵澶栭儴瀛愮郴缁熻皟鐢ㄧ殑 API銆?

杩欐槸濡備綍宸ヤ綔鐨勶紵璁炬兂 ocfs2 鎸傝浇杩囩▼銆傚綋瀹冩寕杞芥椂锛屽畠璇锋眰涓€涓績璺冲尯鍩熼」銆傝繖鏄€氳繃璋冪敤
蹇冭烦浠ｇ爜瀹屾垚鐨勩€傚湪蹇冭烦浠ｇ爜鍐呴儴锛屼細鏌ユ壘璇ュ尯鍩熼」銆傚湪杩欓噷锛屽績璺充唬鐮佽皟鐢?
configfs_depend_item()銆傚鏋滄垚鍔燂紝閭ｄ箞蹇冭烦灏辩煡閬撹鍖哄煙鍙互瀹夊叏鍦颁氦缁?ocfs2銆傚鏋?
澶辫触锛岃鏄庡畠鍙嶆姝ｅ湪琚媶闄わ紝蹇冭烦鍙互浼橀泤鍦颁笂浼犱竴涓敊璇€?
