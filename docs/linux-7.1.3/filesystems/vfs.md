
## Linux 铏氭嫙鏂囦欢绯荤粺姒傝堪


鍘熷浣滆€咃細Richard Gooch <rgooch@atnf.csiro.au>

- 鐗堟潈鎵€鏈?(C) 1999 Richard Gooch
- 鐗堟潈鎵€鏈?(C) 2005 Pekka Enberg


## 寮曡█

铏氭嫙鏂囦欢绯荤粺锛圴irtual File System锛屼篃绉颁负铏氭嫙鏂囦欢绯荤粺浜ゆ崲鏈猴紝Virtual
Filesystem Switch锛夋槸鍐呮牳涓彁渚涙枃浠剁郴缁熸帴鍙ｇ粰鐢ㄦ埛绌洪棿绋嬪簭鐨勮蒋浠跺眰銆傚畠杩樺湪
鍐呮牳鍐呴儴鎻愪緵浜嗕竴绉嶆娊璞★紝浣垮緱涓嶅悓鐨勬枃浠剁郴缁熷疄鐜拌兘澶熷叡瀛樸€?
VFS 绯荤粺璋冪敤 open(2)銆乻tat(2)銆乺ead(2)銆亀rite(2)銆乧hmod(2) 绛夋槸浠庤繘绋嬩笂涓嬫枃涓?璋冪敤鐨勩€傛枃浠剁郴缁熷姞閿佸湪鏂囨。 Documentation/filesystems/locking.rst 涓弿杩般€?

### 鐩綍椤圭紦瀛橈紙dcache锛?
VFS 瀹炵幇浜?open(2)銆乻tat(2)銆乧hmod(2) 鍙婄被浼肩殑绯荤粺璋冪敤銆備紶閫掔粰瀹冧滑鐨勮矾寰勫悕
鍙傛暟琚?VFS 鐢ㄦ潵鍦ㄧ洰褰曢」缂撳瓨锛堜篃绉颁负 dentry 缂撳瓨鎴?dcache锛変腑杩涜鏌ユ壘銆傝繖鎻愪緵
浜嗕竴绉嶉潪甯稿揩閫熺殑鏌ユ壘鏈哄埗锛岀敤浜庡皢璺緞鍚嶏紙鏂囦欢鍚嶏級杞崲涓虹壒瀹氱殑 dentry銆侱entry
瀛樺湪浜?RAM 涓紝浠庝笉琚繚瀛樺埌纾佺洏锛氬畠浠粎涓烘€ц兘鑰屽瓨鍦ㄣ€?
dentry 缂撳瓨鏃ㄥ湪浣滀负浣犳暣涓枃浠剁┖闂寸殑瑙嗗浘銆傜敱浜庡ぇ澶氭暟璁＄畻鏈烘棤娉曞悓鏃跺皢鍏ㄩ儴
dentry 鏀惧叆 RAM锛岀紦瀛樹腑鏌愪簺閮ㄥ垎鏄己澶辩殑銆備负浜嗗皢浣犵殑璺緞鍚嶈В鏋愪负涓€涓?dentry锛?VFS 鍙兘涓嶅緱涓嶆部閫斿垱寤?dentry锛岀劧鍚庡姞杞?inode銆傝繖鏄€氳繃鏌ユ壘 inode 鏉ュ畬鎴愮殑銆?

### Inode 瀵硅薄

鍗曚釜 dentry 閫氬父鏈変竴涓寚鍚?inode 鐨勬寚閽堛€侷node 鏄枃浠剁郴缁熷璞★紝渚嬪鏅€氭枃浠躲€?鐩綍銆丗IFO 鍙婂叾浠栦竴浜涘璞°€傚畠浠涔堜綅浜庣鐩樹笂锛堝浜庡潡璁惧鏂囦欢绯荤粺锛夛紝瑕佷箞浣嶄簬
鍐呭瓨涓紙瀵逛簬浼枃浠剁郴缁燂級銆備綅浜庣鐩樹笂鐨?inode 鍦ㄩ渶瑕佹椂琚鍒跺埌鍐呭瓨涓紝瀵?inode
鐨勪慨鏀逛細琚啓鍥炵鐩樸€傚崟涓?inode 鍙互琚涓?dentry 鎸囧悜锛堜緥濡傜‖閾炬帴灏辨槸杩欐牱鍋氱殑锛夈€?
瑕佹煡鎵句竴涓?inode锛岄渶瑕?VFS 璋冪敤鐖剁洰褰?inode 鐨?lookup() 鏂规硶銆傝鏂规硶鐢?inode
鎵€鍦ㄧ殑鐗瑰畾鏂囦欢绯荤粺瀹炵幇瀹夎銆備竴鏃?VFS 鎷垮埌浜嗘墍闇€鐨?dentry锛堣繘鑰屾嬁鍒?inode锛夛紝
鎴戜滑灏卞彲浠ュ仛閭ｄ簺鏃犺亰鐨勪簨鎯呬簡锛屾瘮濡傜敤 open(2) 鎵撳紑鏂囦欢锛屾垨鐢?stat(2) 鍋风湅 inode
鏁版嵁銆俿tat(2) 鎿嶄綔鐩稿綋绠€鍗曪細涓€鏃?VFS 鎷垮埌 dentry锛屽畠灏卞伔鐪?inode 鏁版嵁骞跺皢鍏朵腑
涓€閮ㄥ垎浼犲洖鐢ㄦ埛绌洪棿銆?

### File 瀵硅薄

鎵撳紑涓€涓枃浠堕渶瑕佸彟涓€涓搷浣滐細鍒嗛厤涓€涓?file 缁撴瀯锛堣繖鏄枃浠舵弿杩扮鍦ㄥ唴鏍镐晶鐨?瀹炵幇锛夈€傛柊鍒嗛厤鐨?file 缁撴瀯鐢ㄦ寚鍚?dentry 鐨勪竴缁勬枃浠舵搷浣滄垚鍛樺嚱鏁颁互鍙婁竴涓寚鍚?dentry 鐨勬寚閽堝垵濮嬪寲銆傝繖浜涘彇鑷?inode 鏁版嵁銆傜劧鍚庤皟鐢?open() 鏂囦欢鏂规硶锛屼互渚跨壒瀹氱殑
鏂囦欢绯荤粺瀹炵幇鑳藉瀹屾垚瀹冪殑宸ヤ綔銆備綘鍙互鐪嬪埌杩欐槸 VFS 鎵ц鐨勫張涓€涓垏鎹€傝 file
缁撴瀯琚斁鍏ヨ繘绋嬬殑 file descriptor 琛ㄤ腑銆?
璇诲彇銆佸啓鍏ュ拰鍏抽棴鏂囦欢锛堜互鍙婂叾浠栧悇绉?VFS 鎿嶄綔锛夋槸閫氳繃浣跨敤鐢ㄦ埛绌洪棿鏂囦欢鎻忚堪绗?鏉ヨ幏鍙栫浉搴旂殑 file 缁撴瀯锛岀劧鍚庤皟鐢ㄦ墍闇€鐨?file 缁撴瀯鏂规硶浠ュ畬鎴愭墍闇€宸ヤ綔鏉ュ畬鎴愮殑銆?鍙鏂囦欢鏄墦寮€鐨勶紝瀹冨氨淇濇寔 dentry 鍦ㄤ娇鐢ㄤ腑锛岃€岃繖鍙堟剰鍛崇潃 VFS inode 浠嶅湪浣跨敤涓€?

## 娉ㄥ唽涓庢寕杞戒竴涓枃浠剁郴缁?
瑕佹敞鍐屽拰娉ㄩ攢涓€涓枃浠剁郴缁燂紝璇蜂娇鐢ㄤ互涓?API 鍑芥暟锛?

	#include <linux/fs.h>

	extern int register_filesystem(struct file_system_type *);
	extern int unregister_filesystem(struct file_system_type *);

鎵€浼犲叆鐨?struct file_system_type 鎻忚堪浜嗕綘鐨勬枃浠剁郴缁熴€傚綋璇锋眰灏嗘煇涓枃浠剁郴缁熸寕杞?鍒颁綘鐨勫懡鍚嶇┖闂翠腑鐨勬煇涓洰褰曟椂锛孷FS 浼氳皟鐢ㄨ鐗瑰畾鏂囦欢绯荤粺鐨勭浉搴?get_tree() 鏂规硶銆?璇﹁ Documentation/filesystems/mount_api.rst銆?
浣犲彲浠ュ湪 /proc/filesystems 鏂囦欢涓湅鍒版敞鍐屽埌鍐呮牳鐨勬墍鏈夋枃浠剁郴缁熴€?

### struct file_system_type

杩欐弿杩颁簡鏂囦欢绯荤粺銆傚畾涔変簡浠ヤ笅鎴愬憳锛?

	struct file_system_type {
		const char *name;
		int fs_flags;
		int (**init_fs_context)(struct fs_context **);
		const struct fs_parameter_spec *parameters;
		void (**kill_sb) (struct super_block **);
		struct module *owner;
		struct file_system_type * next;
		struct hlist_head fs_supers;

		struct lock_class_key s_lock_key;
		struct lock_class_key s_umount_key;
		struct lock_class_key s_vfs_rename_key;
		struct lock_class_key s_writers_key[SB_FREEZE_LEVELS];

		struct lock_class_key i_lock_key;
		struct lock_class_key i_mutex_key;
		struct lock_class_key invalidate_lock_key;
		struct lock_class_key i_mutex_dir_key;
	};

`name`
	鏂囦欢绯荤粺绫诲瀷鐨勫悕绉帮紝渚嬪 "ext2"銆?iso9660"銆?msdos" 绛?
`fs_flags`
	鍚勭鏍囧織锛堝 FS_REQUIRES_DEV銆丗S_NO_DCACHE 绛夛級

`init_fs_context`
	鐢ㄦ枃浠剁郴缁熺壒瀹氱殑鏁版嵁鍒濆鍖?'struct fs_context' 鐨?->ops 涓?->fs_private 瀛楁銆?
`parameters`
	鎸囧悜鏂囦欢绯荤粺鍙傛暟鎻忚堪绗︽暟缁?'struct fs_parameter_spec' 鐨勬寚閽堛€?	鏇村淇℃伅瑙?Documentation/filesystems/mount_api.rst銆?
`kill_sb`
	褰撹鏂囦欢绯荤粺鐨勪竴涓疄渚嬪簲褰撳叧闂椂璋冪敤鐨勬柟娉?
`owner`
	渚?VFS 鍐呴儴浣跨敤锛氬湪澶у鏁版儏鍐典笅浣犲簲灏嗗叾鍒濆鍖栦负 THIS_MODULE銆?
`next`
	渚?VFS 鍐呴儴浣跨敤锛氫綘搴斿皢鍏跺垵濮嬪寲涓?NULL

`fs_supers`
	渚?VFS 鍐呴儴浣跨敤锛氭枃浠剁郴缁熷疄渚嬶紙瓒呯骇鍧楋級鐨?hlist

  s_lock_key銆乻_umount_key銆乻_vfs_rename_key銆乻_writers_key銆?  i_lock_key銆乮_mutex_key銆乮nvalidate_lock_key銆乮_mutex_dir_key锛歭ockdep 涓撶敤

## 瓒呯骇鍧楋紙Superblock锛夊璞?

涓€涓秴绾у潡瀵硅薄浠ｈ〃涓€涓凡鎸傝浇鐨勬枃浠剁郴缁熴€?

### struct super_operations

杩欐弿杩颁簡 VFS 濡備綍鎿嶄綔浣犵殑鏂囦欢绯荤粺鐨勮秴绾у潡銆傚畾涔変簡浠ヤ笅鎴愬憳锛?

	struct super_operations {
		struct inode **(**alloc_inode)(struct super_block *sb);
		void (**destroy_inode)(struct inode **);
		void (**free_inode)(struct inode **);

		void (**dirty_inode) (struct inode **, int flags);
		int (**write_inode) (struct inode **, struct writeback_control *wbc);
		int (**drop_inode) (struct inode **);
		void (**evict_inode) (struct inode **);
		void (**put_super) (struct super_block **);
		int (**sync_fs)(struct super_block **sb, int wait);
		int (**freeze_super) (struct super_block **sb,
					enum freeze_holder who);
		int (**freeze_fs) (struct super_block **);
		int (**thaw_super) (struct super_block **sb,
					enum freeze_wholder who);
		int (**unfreeze_fs) (struct super_block **);
		int (**statfs) (struct dentry **, struct kstatfs *);
		void (**umount_begin) (struct super_block **);

		int (**show_options)(struct seq_file **, struct dentry *);
		int (**show_devname)(struct seq_file **, struct dentry *);
		int (**show_path)(struct seq_file **, struct dentry *);
		int (**show_stats)(struct seq_file **, struct dentry *);

		ssize_t (**quota_read)(struct super_block **, int, char *, size_t, loff_t);
		ssize_t (**quota_write)(struct super_block **, int, const char *, size_t, loff_t);
		struct dquot **(**get_dquots)(struct inode **);

		long (**nr_cached_objects)(struct super_block **,
					struct shrink_control *);
		long (**free_cached_objects)(struct super_block **,
					struct shrink_control *);
	};

闄ら潪鍙︽湁璇存槑锛屾墍鏈夋柟娉曢兘鍦ㄤ笉鎸佹湁浠讳綍閿佺殑鎯呭喌涓嬭皟鐢ㄣ€傝繖鎰忓懗鐫€澶у鏁版柟娉曞彲浠?瀹夊叏鍦伴樆濉炪€傛墍鏈夋柟娉曢兘鍙粠杩涚▼涓婁笅鏂囪皟鐢紙鍗充笉鏄粠涓柇澶勭悊绋嬪簭鎴栧簳鍗婇儴璋冪敤锛夈€?
`alloc_inode`
	璇ユ柟娉曠敱 alloc_inode() 璋冪敤锛屼负 struct inode 鍒嗛厤鍐呭瓨骞跺垵濮嬪寲瀹冦€傚鏋滄湭
	瀹氫箟姝ゅ嚱鏁帮紝鍒欏垎閰嶄竴涓畝鍗曠殑 'struct inode'銆傞€氬父 alloc_inode 浼氳鐢ㄦ潵
	鍒嗛厤涓€涓洿澶х殑銆佸叾涓唴宓屼簡 'struct inode' 鐨勭粨鏋勩€?
`destroy_inode`
	璇ユ柟娉曠敱 destroy_inode() 璋冪敤锛屼互閲婃斁涓?struct inode 鍒嗛厤鐨?resource銆?	浠呭綋瀹氫箟浜?->alloc_inode锛屽苟涓斿彧鏄挙閿€ ->alloc_inode 鎵€鍋氱殑涓€鍒囨椂鎵嶉渶瑕佸畠銆?
`free_inode`
	璇ユ柟娉曚粠 RCU 鍥炶皟涓皟鐢ㄣ€傚鏋滀綘鍦?->destroy_inode 涓娇鐢?call_rcu() 鏉?	閲婃斁 'struct inode' 鍐呭瓨锛岄偅涔堟渶濂藉湪璇ユ柟娉曚腑閲婃斁鍐呭瓨銆?
`dirty_inode`
	褰?inode 琚爣璁颁负鑴忔椂鐢?VFS 璋冪敤銆傝繖鐗规寚 inode 鑷韩琚爣璁颁负鑴忥紝鑰岄潪鍏?	鏁版嵁銆傚鏋滄洿鏂伴渶瑕佺敱 fdatasync() 鎸佷箙鍖栵紝鍒欎細鍦?flags 鍙傛暟涓缃?	I_DIRTY_DATASYNC銆傚鏋滃惎鐢ㄤ簡 lazytime锛屼笖 struct inode 鑷笂娆?->dirty_inode
	璋冪敤浠ユ潵鏇存柊浜嗘椂闂达紝鍒欎細鍦?flags 涓缃?I_DIRTY_TIME銆?
`write_inode`
	褰?VFS 闇€瑕佸皢涓€涓?inode 鍐欏叆纾佺洏鏃惰皟鐢ㄣ€傜浜屼釜鍙傛暟鎸囩ず鍐欏叆鏄惁搴斾负鍚屾鐨勶紝
	骞堕潪鎵€鏈夋枃浠剁郴缁熼兘浼氭鏌ヨ鏍囧織銆?
`drop_inode`
	褰撳 inode 鐨勬渶鍚庝竴娆¤闂鏀惧純鏃惰皟鐢紝姝ゆ椂鎸佹湁 inode->i_lock 鑷棆閿併€?
	璇ユ柟娉曞簲涓?NULL锛堟櫘閫?UNIX 鏂囦欢绯荤粺璇箟锛夛紝鎴栦负 "inode_just_drop"锛堝浜?	涓嶅笇鏈涚紦瀛?inode 鐨勬枃浠剁郴缁熲€斺€斿鑷存棤璁?i_nlink 鍊间负浣曪紝"delete_inode" 鎬绘槸
	琚皟鐢級銆?
	"inode_just_drop()" 琛屼负涓庡湪 put_inode() 鎯呭喌涓嬩娇鐢?"force_delete" 鐨勬棫鍋氭硶
	绛夋晥锛屼絾娌℃湁 "force_delete()" 鏂规硶鎵€瀛樺湪鐨勭珵鎬併€?
`evict_inode`
	褰?VFS 鎯宠椹遍€愶紙evict锛変竴涓?inode 鏃惰皟鐢ㄣ€傝皟鐢ㄨ€?*涓嶄細**椹遍€?pagecache 鎴?	inode 鍏宠仈鐨勫厓鏁版嵁缂撳啿鍖猴紱璇ユ柟娉曞繀椤讳娇鐢?truncate_inode_pages_final() 鏉?	娓呴櫎瀹冧滑銆傝皟鐢ㄨ€呯‘淇濆湪 ->evict_inode() 琚皟鐢ㄦ湡闂达紙鎴栦箣鍚庯級涓嶄細鏈夐拡瀵硅
	inode 鐨勫紓姝ュ洖鍐欒繍琛屻€傚彲閫夈€?
`put_super`
	褰?VFS 甯屾湜閲婃斁瓒呯骇鍧楋紙鍗冲嵏杞斤級鏃惰皟鐢ㄣ€傝皟鐢ㄦ椂鎸佹湁瓒呯骇鍧楅攣銆?
`sync_fs`
	褰?VFS 姝ｅ湪鍐欏嚭涓庝竴涓秴绾у潡鍏宠仈鐨勬墍鏈夎剰鏁版嵁鏃惰皟鐢ㄣ€傜浜屼釜鍙傛暟鎸囩ず璇ユ柟娉曟槸
	鍚﹀簲绛夊緟鍐欏嚭瀹屾垚銆傚彲閫夈€?
`freeze_super`
	濡傛灉鎻愪緵锛屽垯浠ｆ浛 ->freeze_fs 鍥炶皟璋冪敤銆備富瑕佸尯鍒湪浜?->freeze_super 鍦?	涓嶈幏鍙?down_write(&sb->s_umount) 鐨勬儏鍐典笅璋冪敤銆傚鏋滄枃浠剁郴缁熷疄鐜颁簡瀹冨苟涓?	涔熷笇鏈涜皟鐢?->freeze_fs锛屽垯瀹冨繀椤绘樉寮忓湴浠庢鍥炶皟涓皟鐢?->freeze_fs銆傚彲閫夈€?
`freeze_fs`
	褰?VFS 閿佸畾涓€涓枃浠剁郴缁熷苟寮哄埗鍏惰繘鍏ヤ竴鑷寸姸鎬佹椂璋冪敤銆傝鏂规硶褰撳墠琚€昏緫鍗风鐞嗗櫒
	锛圠VM锛夊拰 ioctl(FIFREEZE) 浣跨敤銆傚彲閫夈€?
`thaw_super`
	褰?VFS 鍦?->freeze_super 涔嬪悗瑙ｉ攣涓€涓枃浠剁郴缁熷苟浣垮叾鍐嶆鍙啓鏃惰皟鐢ㄣ€傚彲閫夈€?
`unfreeze_fs`
	褰?VFS 鍦?->freeze_fs 涔嬪悗瑙ｉ攣涓€涓枃浠剁郴缁熷苟浣垮叾鍐嶆鍙啓鏃惰皟鐢ㄣ€傚彲閫夈€?
`statfs`
	褰?VFS 闇€瑕佽幏鍙栨枃浠剁郴缁熺粺璁′俊鎭椂璋冪敤銆?
`umount_begin`
	褰?VFS 姝ｅ湪鍗歌浇涓€涓枃浠剁郴缁熸椂璋冪敤銆?
`show_options`
	鐢?VFS 璋冪敤锛岀敤浜庢樉绀?/proc/<pid>/mounts 涓?/proc/<pid>/mountinfo 鐨勬寕杞?	閫夐」銆傦紙瑙?鎸傝浇閫夐」"涓€鑺傦級

`show_devname`
	鍙€夈€傜敱 VFS 璋冪敤锛岀敤浜庢樉绀?/proc/<pid>/{mounts,mountinfo,mountstats} 鐨?	璁惧鍚嶃€傚鏋滄湭鎻愪緵锛屽垯灏嗕娇鐢?'(struct mount).mnt_devname'銆?
`show_path`
	鍙€夈€傜敱 VFS 璋冪敤锛堥拡瀵?/proc/<pid>/mountinfo锛夛紝鐢ㄤ簬鏄剧ず鐩稿浜庢枃浠剁郴缁熸牴鐨?	鎸傝浇鏍?dentry 璺緞銆?
`show_stats`
	鍙€夈€傜敱 VFS 璋冪敤锛堥拡瀵?/proc/<pid>/mountstats锛夛紝鐢ㄤ簬鏄剧ず鏂囦欢绯荤粺鐗瑰畾鐨勬寕杞?	缁熻淇℃伅銆?
`quota_read`
	鐢?VFS 璋冪敤锛屼互浠庢枃浠剁郴缁熼厤棰濇枃浠惰鍙栥€?
`quota_write`
	鐢?VFS 璋冪敤锛屼互鍚戞枃浠剁郴缁熼厤棰濇枃浠跺啓鍏ャ€?
`get_dquots`
	鐢?quota 璋冪敤锛屼互鑾峰彇鏌愪釜鐗瑰畾 inode 鐨?'struct dquot' 鏁扮粍銆傚彲閫夈€?
`nr_cached_objects`
	鐢辨枃浠剁郴缁熺殑 sb 缂撳瓨鏀剁缉鍑芥暟璋冪敤锛屼互杩斿洖瀹冩墍鍖呭惈鐨勩€佸彲閲婃斁鐨勭紦瀛樺璞℃暟閲忋€?	鍙€夈€?
`free_cache_objects`
	鐢辨枃浠剁郴缁熺殑 sb 缂撳瓨鏀剁缉鍑芥暟璋冪敤锛屼互鎵弿鎸囧畾鏁伴噺鐨勫璞＄殑灏濊瘯閲婃斁瀹冧滑銆?	鍙€夛紝浣嗕换浣曞疄鐜版鏂规硶鐨勬枃浠剁郴缁熶篃闇€瑕佸疄鐜?->nr_cached_objects 鎵嶈兘琚纭?	璋冪敤銆?
	鎴戜滑瀵规枃浠剁郴缁熷彲鑳介亣鍒扮殑浠讳綍閿欒閮芥棤鑳戒负鍔涳紝鍥犳杩斿洖绫诲瀷涓?void銆傚鏋?VM
	璇曞浘鍦?GFP_NOFS 鏉′欢涓嬪洖鏀讹紝鍒欐案杩滀笉浼氳皟鐢ㄥ畠锛屽洜姝よ鏂规硶鑷韩鏃犻渶澶勭悊閭ｇ
	鎯呭喌銆?
	瀹炵幇蹇呴』鍦ㄤ换浣曟墍鍋氱殑鎵弿寰幆涓寘鍚湁鏉′欢鐨勯噸璋冨害锛坮eschedule锛夎皟鐢ㄣ€傝繖浣垮緱
	VFS 鑳藉纭畾鍚堥€傜殑鎵弿鎵瑰ぇ灏忥紝鑰屾棤闇€鎷呭績瀹炵幇浼氬洜涓哄ぇ鐨勬壂鎻忔壒澶у皬鑰屽鑷?	鍋滈】锛坔oldoff锛夐棶棰樸€?
璁剧疆 inode 鐨勪汉璐熻矗濉啓 "i_op" 瀛楁銆傝繖鏄竴涓寚鍚?"struct inode_operations" 鐨?鎸囬拡锛屽悗鑰呮弿杩颁簡鍙湪鍗曚釜 inode 涓婃墽琛岀殑鏂规硶銆?

### struct xattr_handler


鍦ㄦ敮鎸佹墿灞曞睘鎬э紙xattr锛夌殑鏂囦欢绯荤粺涓婏紝s_xattr 瓒呯骇鍧楀瓧娈垫寚鍚戜竴涓互 NULL 缁撳熬鐨?xattr 澶勭悊鍣ㄦ暟缁勩€傛墿灞曞睘鎬ф槸 鍚嶇О:鍊?瀵广€?
`name`
	鎸囩ず璇ュ鐞嗗櫒鍖归厤鍏锋湁鎸囧畾鍚嶇О锛堝 "system.posix_acl_access"锛夌殑灞炴€э紱prefix
	瀛楁蹇呴』涓?NULL銆?
`prefix`
	鎸囩ず璇ュ鐞嗗櫒鍖归厤鍏锋湁鎸囧畾鍚嶇О鍓嶇紑锛堝 "user."锛夌殑鎵€鏈夊睘鎬э紱name 瀛楁蹇呴』涓?	NULL銆?
`list`
	纭畾鏄惁搴斿綋涓烘煇涓壒瀹?dentry 鍒楀嚭鍖归厤姝?xattr 澶勭悊鍣ㄧ殑灞炴€с€傝鏌愪簺
	listxattr 瀹炵幇锛堝 generic_listxattr锛変娇鐢ㄣ€?
`get`
	鐢?VFS 璋冪敤锛屼互鑾峰彇鏌愪釜鐗瑰畾鎵╁睍灞炴€х殑鍊笺€傝鏂规硶鐢?getxattr(2) 绯荤粺璋冪敤璋冪敤銆?
`set`
	鐢?VFS 璋冪敤锛屼互璁剧疆鏌愪釜鐗瑰畾鎵╁睍灞炴€х殑鍊笺€傚綋鏂板€间负 NULL 鏃讹紝璋冪敤浠ョЩ闄ゆ煇涓?	鐗瑰畾鎵╁睍灞炴€с€傝鏂规硶鐢?setxattr(2) 涓?removexattr(2) 绯荤粺璋冪敤璋冪敤銆?
褰撴枃浠剁郴缁熺殑 xattr 澶勭悊鍣ㄩ兘涓嶅尮閰嶆寚瀹氱殑灞炴€у悕锛屾垨鑰呮枃浠剁郴缁熶笉鏀寔鎵╁睍灞炴€ф椂锛?鍚勭 `*xattr(2)` 绯荤粺璋冪敤杩斿洖 -EOPNOTSUPP銆?

## Inode 瀵硅薄


涓€涓?inode 瀵硅薄浠ｈ〃鏂囦欢绯荤粺涓殑涓€涓璞°€?

### struct inode_operations

杩欐弿杩颁簡 VFS 濡備綍鎿嶄綔浣犵殑鏂囦欢绯荤粺涓殑 inode銆傝嚜鍐呮牳 2.6.22 璧凤紝瀹氫箟浜嗕互涓嬫垚鍛橈細


	struct inode_operations {
		int (**create) (struct mnt_idmap **, struct inode **,struct dentry **, umode_t, bool);
		struct dentry ** (**lookup) (struct inode **,struct dentry **, unsigned int);
		int (**link) (struct dentry **,struct inode **,struct dentry **);
		int (**unlink) (struct inode **,struct dentry *);
		int (**symlink) (struct mnt_idmap **, struct inode **,struct dentry **,const char *);
		struct dentry **(**mkdir) (struct mnt_idmap **, struct inode **,struct dentry *,umode_t);
		int (**rmdir) (struct inode **,struct dentry *);
		int (**mknod) (struct mnt_idmap **, struct inode **,struct dentry **,umode_t,dev_t);
		int (**rename) (struct mnt_idmap **, struct inode **, struct dentry **,
			       struct inode **, struct dentry **, unsigned int);
		int (**readlink) (struct dentry **, char __user *,int);
		const char **(**get_link) (struct dentry **, struct inode **,
					 struct delayed_call *);
		int (**permission) (struct mnt_idmap **, struct inode *, int);
		struct posix_acl ** (**get_inode_acl)(struct inode *, int, bool);
		int (**setattr) (struct mnt_idmap **, struct dentry **, struct iattr **);
		int (**getattr) (struct mnt_idmap **, const struct path **, struct kstat **, u32, unsigned int);
		ssize_t (**listxattr) (struct dentry **, char *, size_t);
		void (**update_time)(struct inode **inode, enum fs_update_time type,
				    int flags);
		void (**sync_lazytime)(struct inode **inode);
		int (**atomic_open)(struct inode **, struct dentry **, struct file **,
				   unsigned open_flag, umode_t create_mode);
		int (**tmpfile) (struct mnt_idmap **, struct inode **, struct file **, umode_t);
		struct posix_acl ** (**get_acl)(struct mnt_idmap **, struct dentry **, int);
	        int (**set_acl)(struct mnt_idmap **, struct dentry **, struct posix_acl **, int);
		int (**fileattr_set)(struct mnt_idmap **idmap,
				    struct dentry **dentry, struct file_kattr **fa);
		int (**fileattr_get)(struct dentry **dentry, struct file_kattr *fa);
	        struct offset_ctx **(**get_offset_ctx)(struct inode *inode);
	};

鍚屾牱锛岄櫎闈炲彟鏈夎鏄庯紝鎵€鏈夋柟娉曢兘鍦ㄤ笉鎸佹湁浠讳綍閿佺殑鎯呭喌涓嬭皟鐢ㄣ€?
`create`
	鐢?open(2) 涓?creat(2) 绯荤粺璋冪敤璋冪敤銆備粎褰撲綘鎯宠鏀寔鏅€氭枃浠舵椂鎵嶉渶瑕併€備綘寰楀埌鐨?	dentry 涓嶅簲鏈?inode锛堝嵆瀹冨簲鏄竴涓礋 dentry锛夈€傝繖閲屼綘澶ф浼氱敤 d_instantiate()
	杩炲悓 dentry 涓庢柊寤虹殑 inode 涓€璧疯皟鐢ㄣ€?
`lookup`
	褰?VFS 闇€瑕佸湪鐖剁洰褰曚腑鏌ユ壘涓€涓?inode 鏃惰皟鐢ㄣ€傝鏌ユ壘鐨勫悕绉板湪 dentry 涓€傝
	鏂规硶蹇呴』璋冪敤 d_add() 灏嗘壘鍒扮殑 inode 鎻掑叆 dentry銆俰node 缁撴瀯涓殑 "i_count"
	瀛楁搴斿綋閫掑銆傚鏋滄寚瀹氱殑 inode 涓嶅瓨鍦紝鍒欏簲鍚?dentry 涓彃鍏ヤ竴涓?NULL inode
	锛堣繖绉颁负璐?dentry锛夈€備粠璇ヤ緥绋嬭繑鍥為敊璇爜蹇呴』鍙湪鍙戠敓鐪熷疄閿欒鏃舵墠杩涜锛屽惁鍒?	浣跨敤 create(2)銆乵knod(2)銆乵kdir(2) 绛夌郴缁熻皟鐢ㄥ垱寤?inode 灏嗕細澶辫触銆傚鏋滀綘甯屾湜
	閲嶈浇 dentry 鏂规硶锛岄偅涔堜綘搴旇鍒濆鍖?dentry 涓殑 "d_dop" 瀛楁锛涜繖鏄竴涓寚鍚?	struct "dentry_operations" 鐨勬寚閽堛€傝皟鐢ㄨ鏂规硶鏃舵寔鏈夌洰褰?inode 淇″彿閲忋€?
`link`
	鐢?link(2) 绯荤粺璋冪敤璋冪敤銆備粎褰撲綘鎯宠鏀寔纭摼鎺ユ椂鎵嶉渶瑕併€備綘澶ф闇€瑕佸儚鍦?	create() 鏂规硶涓偅鏍疯皟鐢?d_instantiate()銆?
`unlink`
	鐢?unlink(2) 绯荤粺璋冪敤璋冪敤銆備粎褰撲綘鎯宠鏀寔鍒犻櫎 inode 鏃舵墠闇€瑕併€?
`symlink`
	鐢?symlink(2) 绯荤粺璋冪敤璋冪敤銆備粎褰撲綘鎯宠鏀寔绗﹀彿閾炬帴鏃舵墠闇€瑕併€備綘澶ф闇€瑕佸儚鍦?	create() 鏂规硶涓偅鏍疯皟鐢?d_instantiate()銆?
`mkdir`
	鐢?mkdir(2) 绯荤粺璋冪敤璋冪敤銆備粎褰撲綘鎯宠鏀寔鍒涘缓瀛愮洰褰曟椂鎵嶉渶瑕併€備綘澶ф闇€瑕佸儚鍦?	create() 鏂规硶涓偅鏍疯皟鐢?d_instantiate_new()銆?
	濡傛灉鏈娇鐢?d_instantiate_new()锛屼笖鎻愪緵浜?fh_to_dentry() 瀵煎嚭鎿嶄綔锛屾垨鑰呭瓨鍌ㄥ彲鑳?	閫氳繃鍙︿竴鏉¤矾寰勶紙渚嬪閫氳繃缃戠粶鏂囦欢绯荤粺锛夎璁块棶锛屽垯鍙兘闇€瑕佹洿鍔犲皬蹇冦€傞噸瑕佺殑
	鏄紝濡傛灉 inode 宸蹭笉鍐嶆槸 I_NEW 涓斿瓨鍦ㄨ inode 鍙兘宸茬粡琚檮鍔犲埌鏌愪釜 dentry 鐨勪换浣?	鍙兘锛屽垯涓嶅簲浣跨敤 d_instantate()銆傝繖鏄洜涓?VFS 涓竴鏉＄‖鎬ц鍒欙細涓€涓洰褰曞彧鑳芥湁
	涓€涓?dentry銆?
	渚嬪锛屽鏋滀竴涓?NFS 鏂囦欢绯荤粺琚寕杞戒袱娆★紝鏂扮殑鐩綍鍙兘鍦ㄥ師濮嬫寕杞界偣涔嬪墠灏卞湪鍙︿竴涓?	鎸傝浇鐐逛笂鍙锛屽苟涓斾竴瀵?name_to_handle_at()銆乷pen_by_handle_at() 璋冪敤鍙兘鍦ㄧ涓€涓?	mkdir 杩斿洖涔嬪墠锛岀敤涓€涓?IS_ROOT() dentry 瀹炰緥鍖栬鐩綍 inode銆?
	濡傛灉瀛樺湪浠讳綍杩欑鍙兘鎬э紝鍒欐柊鐨?inode 搴斿綋琚?d_drop() 鎺夛紝骞剁敤 d_splice_alias()
	闄勫姞銆傝繑鍥炵殑 dentry锛堝鏋滄湁锛夊簲鐢?->mkdir() 杩斿洖銆?
`rmdir`
	鐢?rmdir(2) 绯荤粺璋冪敤璋冪敤銆備粎褰撲綘鎯宠鏀寔鍒犻櫎瀛愮洰褰曟椂鎵嶉渶瑕併€?
`mknod`
	鐢?mknod(2) 绯荤粺璋冪敤璋冪敤锛屼互鍒涘缓璁惧锛堝瓧绗︺€佸潡锛塱node 鎴栧懡鍚嶇閬擄紙FIFO锛夋垨
	濂楁帴瀛椼€備粎褰撲綘鎯宠鏀寔鍒涘缓杩欎簺绫诲瀷鐨?inode 鏃舵墠闇€瑕併€備綘澶ф闇€瑕佸儚鍦?create()
	鏂规硶涓偅鏍疯皟鐢?d_instantiate()銆?
`rename`
	鐢?rename(2) 绯荤粺璋冪敤璋冪敤锛屼互灏嗚瀵硅薄閲嶅懡鍚嶄负鐢辩浜屼釜 inode 涓?dentry 缁欏嚭鐨?	鐖剁洰褰曞拰鍚嶇О銆?
	鏂囦欢绯荤粺蹇呴』涓轰换浣曚笉鍙楁敮鎸佹垨鏈煡鐨?flags 杩斿洖 -EINVAL銆傚綋鍓嶅疄鐜颁簡浠ヤ笅鏍囧織锛?	(1) RENAME_NOREPLACE锛氳鏍囧織琛ㄧず锛屽鏋?rename 鐨勭洰鏍囧瓨鍦紝鍒?rename 搴斿綋
	浠?-EEXIST 澶辫触锛岃€岄潪鏇挎崲鐩爣銆俈FS 宸茬粡妫€鏌ヤ簡瀛樺湪鎬э紝鍥犳瀵逛簬鏈湴鏂囦欢绯荤粺锛?	RENAME_NOREPLACE 鐨勫疄鐜扮瓑鍚屼簬鏅€氱殑 rename銆?	(2) RENAME_EXCHANGE锛氫氦鎹㈡簮涓庣洰鏍囥€備袱鑰呴兘蹇呴』瀛樺湪锛涜繖鐢?VFS 妫€鏌ャ€備笌鏅€?	rename 涓嶅悓锛屾簮涓庣洰鏍囧彲浠ユ槸涓嶅悓鐨勭被鍨嬨€?
`get_link`
	鐢?VFS 璋冪敤锛屼互璺熼殢涓€涓鍙烽摼鎺ュ埌瀹冩墍鎸囧悜鐨?inode銆備粎褰撲綘鎯宠鏀寔绗﹀彿閾炬帴鏃?	鎵嶉渶瑕併€傝鏂规硶杩斿洖瑕侀亶鍘嗙殑绗﹀彿閾炬帴浣擄紙骞跺彲鑳界敤 nd_jump_link() 閲嶇疆褰撳墠浣嶇疆锛夈€?	濡傛灉绗﹀彿閾炬帴浣撳湪 inode 娑堝け涔嬪墠閮戒笉浼氭秷澶憋紝鍒欐棤闇€鍋氬叾浠栦簨鎯咃紱濡傛灉瀹冮渶瑕佷互
	鍏朵粬鏂瑰紡琚浐瀹氾紙pinned锛夛紝鍒欓€氳繃璁?get_link(..., ..., done) 璋冪敤
	set_delayed_call(done, destructor, argument) 鏉ュ畨鎺掑叾閲婃斁銆傚湪閭ｇ鎯呭喌涓嬶紝涓€鏃?	VFS 澶勭悊瀹屼綘杩斿洖鐨勯摼鎺ヤ綋锛屽氨浼氳皟鐢?destructor(argument)銆傚彲鑳藉湪 RCU 妯″紡涓?	璋冪敤锛涜繖鐢?NULL dentry 鍙傛暟鎸囩ず銆傚鏋滄棤娉曞湪涓嶇寮€ RCU 妯″紡鐨勬儏鍐典笅澶勭悊璇锋眰锛?	鍒欒瀹冭繑鍥?ERR_PTR(-ECHILD)銆?
	濡傛灉鏂囦欢绯荤粺灏嗙鍙烽摼鎺ョ洰鏍囧瓨鍌ㄥ湪 ->i_link 涓紝VFS 鍙兘鐩存帴浣跨敤瀹冭€屾棤闇€璋冪敤
	->get_link()锛涚劧鑰岋紝->get_link() 浠嶅繀椤绘彁渚涖€?>i_link 鍦?RCU 瀹介檺鏈熶箣鍚庢墠鑳?	琚噴鏀俱€傚湪 iget() 涔嬪悗鐨勬椂闂村啓鍏?->i_link 闇€瑕佷竴涓?'release' 鍐呭瓨灞忛殰銆?
`readlink`
	鐜板湪瀹冨彧鏄?readlink(2) 鍦ㄦ煇浜涙儏鍐典笅浣跨敤鐨勪竴涓鐩栵細褰?->get_link 浣跨敤
	nd_jump_link() 鎴栧璞″疄闄呬笂涓嶆槸绗﹀彿閾炬帴鏃躲€傞€氬父鏂囦欢绯荤粺搴斿綋鍙疄鐜?->get_link
	鐢ㄤ簬绗﹀彿閾炬帴锛岃€?readlink(2) 灏嗚嚜鍔ㄤ娇鐢ㄥ畠銆?
`permission`
	鐢?VFS 璋冪敤锛屼互妫€鏌ョ被 POSIX 鏂囦欢绯荤粺鐨勮闂潈闄愩€?
	鍙兘鍦?rcu-walk 妯″紡涓嬭皟鐢紙mask & MAY_NOT_BLOCK锛夈€傚鏋滃湪 rcu-walk 妯″紡涓嬶紝
	鏂囦欢绯荤粺蹇呴』鍦ㄤ笉闃诲鎴栦笉鍐欏叆 inode 鐨勬儏鍐典笅妫€鏌ユ潈闄愩€?
	濡傛灉閬囧埌 rcu-walk 鏃犳硶澶勭悊鐨勬儏鍐碉紝杩斿洖 -ECHILD锛屽畠灏嗗湪 ref-walk 妯″紡涓嬪啀娆¤璋冪敤銆?
`setattr`
	鐢?VFS 璋冪敤锛屼互璁剧疆鏂囦欢鐨勫睘鎬с€傝鏂规硶鐢?chmod(2) 鍙婄浉鍏崇殑绯荤粺璋冪敤璋冪敤銆?
`getattr`
	鐢?VFS 璋冪敤锛屼互鑾峰彇鏂囦欢鐨勫睘鎬с€傝鏂规硶鐢?stat(2) 鍙婄浉鍏崇殑绯荤粺璋冪敤璋冪敤銆?
`listxattr`
	鐢?VFS 璋冪敤锛屼互鍒楀嚭缁欏畾鏂囦欢鐨勬墍鏈夋墿灞曞睘鎬с€傝鏂规硶鐢?listxattr(2) 绯荤粺璋冪敤璋冪敤銆?
`update_time`
	鐢?VFS 璋冪敤锛屼互鏇存柊 inode 鐨勭壒瀹氭椂闂存垨 i_version銆傚鏋滄湭瀹氫箟姝ゅ嚱鏁帮紝VFS 灏?	鑷鏇存柊 inode 骞惰皟鐢?mark_inode_dirty_sync銆?
`sync_lazytime`锛?	鐢卞洖鍐欙紙writeback锛変唬鐮佽皟鐢紝浠ュ皢鎯版€ф椂闂存埑鏇存柊涓轰細琚悓姝ヨ繘纾佺洏 inode 鐨?	甯歌鏃堕棿鎴虫洿鏂般€?
`atomic_open`
	鍦?open 鐨勬渶鍚庝竴涓垎閲忎笂璋冪敤銆備娇鐢ㄨ鍙€夋柟娉曪紝鏂囦欢绯荤粺鍙互鍦ㄤ竴娆″師瀛愭搷浣滀腑
	鏌ユ壘銆佸彲鑳藉垱寤哄苟鎵撳紑鏂囦欢銆傚鏋滃畠鎯虫妸瀹為檯鐨勬墦寮€鐣欑粰璋冪敤鑰咃紙渚嬪锛屽鏋滄枃浠?	缁撴灉鏄鍙烽摼鎺ャ€佽澶囷紝鎴栧彧鏄枃浠剁郴缁熶笉浼氳繘琛屽師瀛愭墦寮€鐨勪笢瑗匡級锛屽畠鍙互閫氳繃
	杩斿洖 finish_no_open(file, dentry) 鏉ュ彂鍑烘淇″彿銆傝鏂规硶浠呭湪鏈€鍚庝竴涓垎閲忔槸璐熺殑
	鎴栭渶瑕佹煡鎵炬椂鎵嶈璋冪敤銆傜紦瀛樼殑姝?dentry 浠嶇敱 f_op->open() 澶勭悊銆傚鏋滄枃浠惰鍒涘缓锛?	鍒欏簲鍦?file->f_mode 涓缃?FMODE_CREATED 鏍囧織銆傚湪 O_EXCL 鐨勬儏鍐典笅锛岃鏂规硶蹇呴』
	浠呭綋鏂囦欢涓嶅瓨鍦ㄦ椂鎵嶆垚鍔燂紝鍥犳 FMODE_CREATED 鍦ㄦ垚鍔熸椂搴斿綋鎬绘槸琚缃€?
`tmpfile`
	鍦?O_TMPFILE open() 鐨勬湯灏捐皟鐢ㄣ€傚彲閫夛紝绛変环浜庡湪缁欏畾鐩綍涓師瀛愬湴鍒涘缓銆佹墦寮€骞?	瑙ｉ櫎閾炬帴涓€涓枃浠躲€傛垚鍔熸椂闇€瑕佽繑鍥炴椂鏂囦欢宸茬粡鎵撳紑锛涜繖鍙互閫氳繃鍦ㄦ湯灏剧洿鎺ヨ皟鐢?	finish_open_simple() 鏉ュ畬鎴愩€?
`fileattr_get`
	鍦?ioctl(FS_IOC_GETFLAGS) 涓?ioctl(FS_IOC_FSGETXATTR) 涓婅皟鐢紝浠ユ绱㈡潅椤规枃浠?	鏍囧織涓庡睘鎬с€傚湪鐩稿叧鐨?SET 鎿嶄綔涔嬪墠涔熶細琚皟鐢紝浠ユ鏌ュ皢瑕佹敼鍙樼殑鍐呭锛堟鏃?	鎸佹湁 i_rwsem 鎺掍粬閿侊級銆傚鏋滄湭璁剧疆锛屽垯鍥為€€鍒?f_op->ioctl()銆?
`fileattr_set`
	鍦?ioctl(FS_IOC_SETFLAGS) 涓?ioctl(FS_IOC_FSSETXATTR) 涓婅皟鐢紝浠ユ洿鏀规潅椤规枃浠?	鏍囧織涓庡睘鎬с€傝皟鐢ㄨ€呮寔鏈?i_rwsem 鎺掍粬閿併€傚鏋滄湭璁剧疆锛屽垯鍥為€€鍒?f_op->ioctl()銆?
`get_offset_ctx`
	琚皟鐢ㄤ互鑾峰彇鐩綍 inode 鐨?offset 涓婁笅鏂囥€傛枃浠剁郴缁熷繀椤诲畾涔夋鎿嶄綔鎵嶈兘浣跨敤
	simple_offset_dir_operations銆?
## 鍦板潃绌洪棿锛圓ddress Space锛夊璞?

鍦板潃绌洪棿瀵硅薄鐢ㄤ簬瀵归〉缂撳瓨锛坧age cache锛変腑鐨勯〉杩涜鍒嗙粍涓庣鐞嗐€傚畠鍙敤浜庤窡韪竴涓?鏂囦欢锛堟垨鍏朵粬浠讳綍涓滆タ锛変腑鐨勯〉锛屽苟璺熻釜鏂囦欢鍚勬鍒拌繘绋嬪湴鍧€绌洪棿鐨勬槧灏勩€?
鍦板潃绌洪棿鍙互鎻愪緵鑻ュ共涓嶅悓浣嗙浉鍏崇殑鏈嶅姟銆傝繖浜涘寘鎷紶杈惧唴瀛樺帇鍔涖€佹寜鍦板潃杩涜椤垫煡鎵撅紝
浠ュ強璺熻釜琚爣璁颁负鑴忥紙Dirty锛夋垨鍥炲啓锛圵riteback锛夌殑椤点€?
绗竴涓彲浠ョ嫭绔嬩簬鍏朵粬鏈嶅姟浣跨敤銆俈M 鍙互灏濊瘯閲婃斁骞插噣椤典互閲嶇敤瀹冧滑銆備负姝わ紝瀹冨彲浠?鍦ㄥ甫鏈?private 鏍囧織鐨勫共鍑€ folio 涓婅皟鐢?->release_folio銆傛病鏈?PagePrivate 涓?娌℃湁澶栭儴寮曠敤鐨勫共鍑€椤靛皢鍦ㄤ笉閫氱煡鍦板潃绌洪棿鐨勬儏鍐典笅琚噴鏀俱€?
涓轰簡瀹炵幇璇ュ姛鑳斤紝椤甸渶瑕佽鏀惧湪涓€涓?LRU 涓婏紙閫氳繃 lru_cache_add锛夛紝骞朵笖姣忓綋椤佃
浣跨敤鏃堕兘闇€瑕佽皟鐢?mark_page_active銆?
椤甸€氬父鎸?->index 淇濆瓨鍦ㄤ竴涓熀鏁版爲锛坮adix tree锛夌储寮曚腑銆傝鏍戠淮鎶ゆ瘡涓〉鐨?PG_Dirty
涓?PG_Writeback 鐘舵€佷俊鎭紝浠ヤ究鍙互蹇€熸壘鍒板甫鏈夎繖涓や釜鏍囧織涓换鎰忎竴涓殑椤点€?
Dirty 鏍囩涓昏琚?mpage_writepages鈥斺€旈粯璁ょ殑 ->writepages 鏂规硶浣跨敤銆傚畠浣跨敤璇ユ爣绛?鏉ユ煡鎵捐鍥炲啓鐨勮剰椤点€傚鏋滄湭浣跨敤 mpage_writepages锛堝嵆鍦板潃绌洪棿鎻愪緵浜嗚嚜宸辩殑
->writepages锛夛紝鍒?PAGECACHE_TAG_DIRTY 鏍囩鍑犱箮鏈浣跨敤銆倃rite_inode_now 涓?sync_inode 纭疄浣跨敤瀹冿紙閫氳繃 __sync_single_inode锛夋潵妫€鏌?->writepages 鏄惁鎴愬姛
鍐欏嚭浜嗘暣涓湴鍧€绌洪棿銆?
Writeback 鏍囩琚?filemap**wait** 涓?sync_page* 鍑芥暟閫氳繃 filemap_fdatawait_range
浣跨敤锛屼互绛夊緟鎵€鏈夊洖鍐欏畬鎴愩€?
鍦板潃绌洪棿澶勭悊鍣ㄥ彲浠ュ皢棰濆淇℃伅闄勫姞鍒伴〉涓婏紝閫氬父浣跨敤 'struct page' 涓殑 'private'
瀛楁銆傚鏋滈檮鍔犱簡姝ょ被淇℃伅锛屽垯搴斿綋璁剧疆 PG_Private 鏍囧織銆傝繖灏嗗鑷村悇绉?VM 渚嬬▼瀵?鍦板潃绌洪棿澶勭悊鍣ㄨ繘琛岄澶栬皟鐢紝浠ュ鐞嗚鏁版嵁銆?
鍦板潃绌洪棿鍏呭綋瀛樺偍涓庡簲鐢ㄧ▼搴忎箣闂寸殑涓粙銆傛暟鎹互鏁撮〉涓哄崟浣嶈鍏ュ湴鍧€绌洪棿锛屽苟閫氳繃
澶嶅埗璇ラ〉鎴栧唴瀛樻槧灏勮椤垫彁渚涚粰搴旂敤绋嬪簭銆傛暟鎹敱搴旂敤绋嬪簭鍐欏叆鍦板潃绌洪棿锛岀劧鍚庨€氬父浠?鏁撮〉鍐欏洖瀛樺偍锛屼笉杩囧湴鍧€绌洪棿瀵瑰啓鍏ュぇ灏忔湁鏇寸簿缁嗙殑鎺у埗銆?
璇诲彇杩囩▼鏈川涓婂彧闇€瑕?'read_folio'銆傚啓鍏ヨ繃绋嬫洿澶嶆潅锛屽畠浣跨敤 write_begin/write_end
鎴?dirty_folio 灏嗘暟鎹啓鍏ュ湴鍧€绌洪棿锛屽苟浣跨敤 writepages 灏嗘暟鎹洖鍐欒嚦瀛樺偍銆?
浠庡湴鍧€绌洪棿绉婚櫎椤甸渶瑕佹帓浠栧湴鎸佹湁 inode 鐨?i_rwsem锛岃€屽悜鍦板潃绌洪棿娣诲姞椤甸渶瑕佹帓浠栧湴
鎸佹湁 inode 鐨?i_mapping->invalidate_lock銆?
褰撴暟鎹鍐欏叆椤垫椂锛屽簲褰撹缃?PG_Dirty 鏍囧織銆傚畠閫氬父涓€鐩翠繚鎸佽缃紝鐩村埌 writepages
瑕佹眰灏嗗叾鍐欏嚭銆傝繖搴斿綋娓呴櫎 PG_Dirty 骞惰缃?PG_Writeback銆傚湪 PG_Dirty 琚竻闄ゅ悗鐨?浠讳綍鏃跺埢閮藉彲浠ュ疄闄呭啓鍑恒€備竴鏃︾‘瀹氬畨鍏紝PG_Writeback 琚竻闄ゃ€?
鍥炲啓鍒╃敤浜?writeback_control 缁撴瀯鏉ユ寚瀵兼搷浣溿€傝繖涓?writepages 鎿嶄綔鎻愪緵浜嗕竴浜涘叧浜?鍥炲啓璇锋眰鐨勬€ц川涓庡師鍥犮€佷互鍙婃墽琛屾椂绾︽潫鏉′欢鐨勪俊鎭€傚畠涔熻鐢ㄦ潵灏嗙粨鏋滀俊鎭繑鍥炵粰璋冪敤鑰呫€?
### 鍥炲啓鏈熼棿鐨勯敊璇鐞?
澶у鏁拌繘琛岀紦鍐?I/O 鐨勫簲鐢ㄧ▼搴忎細瀹氭湡璋冪敤鏂囦欢鍚屾璋冪敤锛坒sync銆乫datasync銆乵sync 鎴?sync_file_range锛夛紝浠ョ‘淇濆啓鍏ョ殑鏁版嵁宸茬粡鍒拌揪鍚庡瀛樺偍銆傚綋鍥炲啓鏈熼棿鍙戠敓閿欒鏃讹紝瀹冧滑
鏈熸湜鍦ㄥ彂鍑烘枃浠跺悓姝ヨ姹傛椂鎶ュ憡璇ラ敊璇€傚湪涓€涓姹備笂鎶ュ憡閿欒涔嬪悗锛屽悓涓€鏂囦欢鎻忚堪绗︿笂鐨?鍚庣画璇锋眰搴斿綋杩斿洖 0锛岄櫎闈炶嚜涓婃鏂囦欢鍚屾浠ユ潵鍙戠敓浜嗚繘涓€姝ョ殑鍥炲啓閿欒銆?
鐞嗘兂鎯呭喌涓嬶紝鍐呮牳鍙細鍚戦偅浜涚‘瀹炶繘琛屼簡鍐欏叆銆佷絾闅忓悗鍥炲啓澶辫触鐨勬枃浠舵弿杩扮鎶ュ憡閿欒銆?鐒惰€岋紝閫氱敤鐨勯〉缂撳瓨鍩虹璁炬柦骞朵笉璺熻釜寮勮剰浜嗘瘡涓崟鐙〉鐨勬枃浠舵弿杩扮锛屽洜姝ゆ棤娉曠‘瀹?鍝簺鏂囦欢鎻忚堪绗﹀簲褰撴敹鍒伴敊璇€?
鐩稿弽锛屽唴鏍镐腑閫氱敤鐨勫洖鍐欓敊璇窡韪熀纭€璁炬柦婊¤冻浜庡皢閿欒鎶ュ憡缁欏湪鍙戠敓閿欒鏃舵墦寮€鐨勬墍鏈?鏂囦欢鎻忚堪绗︿笂鐨?fsync銆傚湪澶氫釜鍐欏叆鑰呯殑鎯呭喌涓嬶紝瀹冧滑閮戒細鍦ㄥ悗缁殑 fsync 涓婃敹鍒颁竴涓敊璇紝
鍗充娇閫氳繃璇ョ壒瀹氭枃浠舵弿杩扮鎵€鍋氱殑鎵€鏈夊啓鍏ラ兘鎴愬姛浜嗭紙鐢氳嚦鍗充娇璇ユ枃浠舵弿杩扮涓婃牴鏈病鏈?浠讳綍鍐欏叆锛夈€?
甯屾湜浣跨敤姝ゅ熀纭€璁炬柦鐨勬枃浠剁郴缁熷簲璇ュ湪閿欒鍙戠敓鏃惰皟鐢?mapping_set_error锛屽皢閿欒璁板綍
鍦ㄥ湴鍧€绌洪棿涓€傜劧鍚庯紝鍦ㄩ€氳繃瀹冧滑鐨?file->fsync 鎿嶄綔浠庨〉缂撳瓨鍥炲啓鏁版嵁涔嬪悗锛屽畠浠簲褰?璋冪敤 file_check_and_advance_wb_err锛屼互纭繚 struct file 鐨勯敊璇父鏍囧凡缁忔帹杩涘埌鍚庡
璁惧鍙戝嚭鐨勯敊璇祦涓殑姝ｇ‘浣嶇疆銆?

### struct address_space_operations

杩欐弿杩颁簡 VFS 濡備綍鎿嶄綔浣犵殑鏂囦欢绯荤粺涓枃浠跺埌椤电紦瀛樼殑鏄犲皠銆傚畾涔変簡浠ヤ笅鎴愬憳锛?

	struct address_space_operations {
		int (**read_folio)(struct file **, struct folio *);
		int (**writepages)(struct address_space **, struct writeback_control *);
		bool (**dirty_folio)(struct address_space **, struct folio *);
		void (**readahead)(struct readahead_control **);
		int (**write_begin)(const struct kiocb **, struct address_space *mapping,
				   loff_t pos, unsigned len,
				   struct page **pagep, void **fsdata);
		int (**write_end)(const struct kiocb **, struct address_space *mapping,
				 loff_t pos, unsigned len, unsigned copied,
				 struct folio **folio, void **fsdata);
		sector_t (**bmap)(struct address_space **, sector_t);
		void (**invalidate_folio) (struct folio **, size_t start, size_t len);
		bool (**release_folio)(struct folio **, gfp_t);
		void (**free_folio)(struct folio **);
		ssize_t (**direct_IO)(struct kiocb **, struct iov_iter *iter);
		int (**migrate_folio)(struct mapping **, struct folio *dst,
				struct folio *src, enum migrate_mode);
		int (**launder_folio) (struct folio **);

		bool (**is_partially_uptodate) (struct folio **, size_t from,
					       size_t count);
		void (**is_dirty_writeback)(struct folio **, bool **, bool **);
		int (**error_remove_folio)(struct mapping **mapping, struct folio *);
		int (**swap_activate)(struct swap_info_struct **sis, struct file **f, sector_t **span)
		int (**swap_deactivate)(struct file **);
		int (**swap_rw)(struct kiocb **iocb, struct iov_iter *iter);
	};

`read_folio`
	鐢遍〉缂撳瓨璋冪敤锛屼互浠庡悗澶囧瓨鍌ㄨ鍙栦竴涓?folio銆?file' 鍙傛暟涓虹綉缁滄枃浠剁郴缁熸彁渚涜璇?	淇℃伅锛屽潡璁惧鏂囦欢绯荤粺閫氬父涓嶄娇鐢ㄥ畠銆傚鏋滆皟鐢ㄨ€呮病鏈夋墦寮€鐨勬枃浠讹紙渚嬪锛屽鏋滃唴鏍?	姝ｅ湪涓鸿嚜宸辨墽琛岃鍙栵紝鑰岄潪浠ｈ〃甯︽湁鎵撳紑鏂囦欢鐨勭敤鎴风┖闂磋繘绋嬶級锛屽畠鍙兘涓?NULL銆?
	濡傛灉鏄犲皠涓嶆敮鎸佸ぇ folio锛屽垯 folio 灏嗗寘鍚崟涓〉銆傝皟鐢?read_folio 鏃?folio 浼氳
	閿佸畾銆傚鏋滆鍙栨垚鍔熷畬鎴愶紝鍒?folio 搴旇鏍囪涓?uptodate銆傛棤璁烘垚鍔熶笌鍚︼紝鏂囦欢绯荤粺閮?	搴斿湪璇诲彇瀹屾垚鍚庤В閿?folio銆傛枃浠剁郴缁熸棤闇€淇敼 folio 涓婄殑寮曠敤璁℃暟锛涢〉缂撳瓨鎸佹湁寮曠敤
	璁℃暟锛屽苟涓斿湪 folio 瑙ｉ攣涔嬪墠涓嶄細閲婃斁瀹冦€?
	鏂囦欢绯荤粺鍙互鍚屾鍦板疄鐜?->read_folio()銆傚湪姝ｅ父鎿嶄綔涓紝folio 鏄€氳繃 ->readahead()
	鏂规硶璇诲彇鐨勩€傚彧鏈夊湪璇ユ柟娉曞け璐ワ紝鎴栬皟鐢ㄨ€呴渶瑕佺瓑寰呰鍙栧畬鎴愭椂锛岄〉缂撳瓨鎵嶄細璋冪敤
	->read_folio()銆傛枃浠剁郴缁熶笉搴斿皾璇曞湪 ->read_folio() 鎿嶄綔涓墽琛岃嚜宸辩殑棰勮銆?
	濡傛灉鏂囦欢绯荤粺姝ゆ椂鏃犳硶鎵ц璇诲彇锛屽畠鍙互瑙ｉ攣 folio锛屾墽琛屽畠闇€瑕佺‘淇濊鍙栧皢鏉ヤ細鎴愬姛
	鎵€闇€鐨勪换浣曞姩浣滐紝骞惰繑鍥?AOP_TRUNCATED_PAGE銆傚湪杩欑鎯呭喌涓嬶紝璋冪敤鑰呭簲褰撴煡鎵?folio銆?	閿佸畾瀹冿紝骞跺啀娆¤皟鐢?->read_folio銆?
	璋冪敤鑰呭彲浠ョ洿鎺ヨ皟鐢?->read_folio() 鏂规硶锛屼絾浣跨敤 read_mapping_folio() 灏嗚礋璐ｅ姞閿併€?	绛夊緟璇诲彇瀹屾垚锛屽苟澶勭悊 AOP_TRUNCATED_PAGE 绛夋儏鍐点€?
`writepages`
	鐢?VM 璋冪敤锛屼互鍐欏嚭涓庡湴鍧€绌洪棿瀵硅薄鍏宠仈鐨勯〉銆傚鏋?wbc->sync_mode 鏄?WB_SYNC_ALL锛?	鍒?writeback_control 灏嗘寚瀹氫竴涓繀椤昏鍐欏嚭鐨勯〉鑼冨洿銆傚鏋滃畠鏄?WB_SYNC_NONE锛屽垯
	缁欏嚭 nr_to_write锛屽苟搴斿綋灏藉彲鑳藉鍦板啓鍑洪偅涔堝鐨勯〉銆傚鏋滄病鏈夌粰鍑?->writepages锛?	鍒欐敼鐢?mpage_writepages銆傚畠灏嗕粠鍦板潃绌洪棿涓€夋嫨琚爣璁颁负 DIRTY 鐨勯〉骞跺皢瀹冧滑鍥炲啓銆?
`dirty_folio`
	鐢?VM 璋冪敤锛屼互灏嗕竴涓?folio 鏍囪涓鸿剰銆傚鏋滃湴鍧€绌洪棿灏嗙鏈夋暟鎹檮鍔犲埌 folio锛屽苟涓?	璇ユ暟鎹渶瑕佸湪 folio 鍙樿剰鏃舵洿鏂帮紝鍒欑壒鍒渶瑕佸畠銆備緥濡傦紝褰撲竴涓唴瀛樻槧灏勭殑椤佃淇敼鏃?	灏变細璋冪敤瀹冦€傚鏋滃畾涔変簡瀹冿紝瀹冨簲褰撹缃?folio 鐨勮剰鏍囧織锛屼互鍙?i_pages 涓殑
	PAGECACHE_TAG_DIRTY 鎼滅储鏍囪銆?
`readahead`
	鐢?VM 璋冪敤锛屼互璇诲彇涓庡湴鍧€绌洪棿瀵硅薄鍏宠仈鐨勯〉銆傝繖浜涢〉鍦ㄩ〉缂撳瓨涓槸杩炵画鐨勶紝骞朵笖鏄?	琚攣瀹氱殑銆傚疄鐜板簲褰撳湪瀵规瘡涓〉鍚姩 I/O 涔嬪悗閫掑噺椤靛紩鐢ㄨ鏁般€傞€氬父璇ラ〉浼氱敱 I/O
	瀹屾垚澶勭悊绋嬪簭瑙ｉ攣銆傝繖缁勯〉琚垎鎴愪竴浜涘悓姝ラ〉锛屽悗璺熶竴浜涘紓姝ラ〉锛宺ac->ra->async_size
	缁欏嚭寮傛椤电殑鏁伴噺銆傛枃浠剁郴缁熷簲褰撳皾璇曡鍙栨墍鏈夊悓姝ラ〉锛屼絾涓€鏃﹀埌杈惧紓姝ラ〉灏卞彲浠ュ喅瀹?	鍋滄銆傚鏋滃畠纭疄鍐冲畾鍋滄灏濊瘯 I/O锛屽畠鍙互绠€鍗曞湴杩斿洖銆傝皟鐢ㄨ€呭皢浠庡湴鍧€绌洪棿涓Щ闄?	鍓╀綑鐨勯〉銆佽В閿佸畠浠苟閫掑噺椤靛紩鐢ㄨ鏁般€傚鏋?I/O 鎴愬姛瀹屾垚锛屽垯璁剧疆 PageUptodate銆?
`write_begin`
	鐢遍€氱敤鐨勭紦鍐插啓鍏ヤ唬鐮佽皟鐢紝浠ヨ鏂囦欢绯荤粺鍑嗗鍦ㄦ枃浠朵腑缁欏畾鍋忕Щ澶勫啓鍏?len 瀛楄妭銆?	鍦板潃绌洪棿搴斿綋閫氳繃蹇呰鍦板垎閰嶇┖闂翠互鍙婅繘琛屼换浣曞叾浠栧唴閮ㄨ璐︼紝鏉ユ鏌ュ啓鍏ユ槸鍚﹁兘澶?	瀹屾垚銆傚鏋滃啓鍏ュ皢鏇存柊瀛樺偍涓婁换浣曞熀鏈潡锛坆asic-block锛夌殑閮ㄥ垎锛岄偅涔堥偅浜涘潡搴斿綋琚?	棰勮锛堝鏋滃皻鏈鍙栵級锛屼互渚挎洿鏂板悗鐨勫潡鑳藉琚纭啓鍑恒€?
	鏂囦欢绯荤粺蹇呴』涓烘寚瀹氬亸绉诲杩斿洖閿佸畾鐨勯〉缂撳瓨 folio锛屾斁鍦?`*foliop` 涓紝渚涜皟鐢ㄨ€?	鍐欏叆銆?
	瀹冨繀椤昏兘澶熷鐞嗙煭鍐欏叆锛堜紶閫掔粰 write_begin 鐨勯暱搴﹀ぇ浜庤澶嶅埗鍒?folio 涓殑瀛楄妭鏁?	鐨勬儏鍐碉級銆?
	鍙互鍦?fsdata 涓繑鍥炰竴涓?void *锛屽畠闅忓悗琚紶鍏?write_end銆?
	鎴愬姛鏃惰繑鍥?0锛涘け璐ユ椂杩斿洖 < 0锛堝嵆閿欒鐮侊級锛屾鏃朵笉璋冪敤 write_end銆?
`write_end`
	鍦ㄦ垚鍔熺殑 write_begin 涓庢暟鎹鍒朵箣鍚庯紝蹇呴』璋冪敤 write_end銆俵en 鏄紶鍏?write_begin
	鐨勫師濮?len锛宑opied 鏄兘澶熷鍒剁殑瀛楄妭鏁般€?
	鏂囦欢绯荤粺蹇呴』璐熻矗瑙ｉ攣 folio銆侀€掑噺鍏跺紩鐢ㄨ鏁板苟鏇存柊 i_size銆?
	澶辫触鏃惰繑鍥?< 0锛屽惁鍒欒繑鍥炶兘澶熷鍒跺埌椤电紦瀛樹腑鐨勫瓧鑺傛暟锛?= 'copied'锛夈€?
`bmap`
	鐢?VFS 璋冪敤锛屼互灏嗗璞″唴鐨勯€昏緫鍧楀亸绉绘槧灏勪负鐗╃悊鍧楀彿銆傝鏂规硶琚?FIBMAP ioctl 浠ュ強
	鐢ㄤ簬澶勭悊浜ゆ崲鏂囦欢锛坰wap-file锛変娇鐢ㄣ€備负浜嗚兘澶熶氦鎹㈠埌鏂囦欢锛岃鏂囦欢蹇呴』鍏锋湁鍒板潡璁惧
	鐨勭ǔ瀹氭槧灏勩€備氦鎹㈢郴缁熶笉缁忚繃鏂囦欢绯荤粺锛岃€屾槸浣跨敤 bmap 鎵惧嚭鏂囦欢涓潡鐨勪綅缃苟鐩存帴浣跨敤
	閭ｄ簺鍦板潃銆?
`invalidate_folio`
	濡傛灉 folio 甯︽湁绉佹湁鏁版嵁锛岄偅涔堝綋 folio 鐨勯儴鍒嗘垨鍏ㄩ儴瑕佷粠鍦板潃绌洪棿绉婚櫎鏃讹紝灏嗚皟鐢?	invalidate_folio銆傝繖閫氬父瀵瑰簲浜庢埅鏂€佹墦娲烇紙punch hole锛夋垨鍦板潃绌洪棿鐨勫畬鍏ㄥけ鏁堬紙鍦?	鍚庝竴绉嶆儏鍐典笅 'offset' 鎬绘槸涓?0锛?length' 涓?folio_size()锛夈€備换浣曚笌 folio 鍏宠仈鐨?	绉佹湁鏁版嵁閮藉簲褰撹鏇存柊浠ュ弽鏄犳鎴柇銆傚鏋?offset 涓?0 涓?length 涓?folio_size()锛?	鍒欏簲褰撻噴鏀剧鏈夋暟鎹紝鍥犱负 folio 蹇呴』鑳藉琚畬鍏ㄤ涪寮冦€傝繖鍙互閫氳繃璋冪敤 ->release_folio
	鍑芥暟鏉ュ畬鎴愶紝浣嗗湪杩欑鎯呭喌涓嬮噴鏀惧繀椤绘垚鍔熴€?
`release_folio`
	release_folio 鍦ㄥ甫鏈夌鏈夋暟鎹殑 folio 涓婅皟鐢紝浠ュ憡鐭ユ枃浠剁郴缁熻 folio 鍗冲皢琚噴鏀俱€?	->release_folio 搴斿綋浠庤 folio 涓Щ闄や换浣曠鏈夋暟鎹苟娓呴櫎 private 鏍囧織銆傚鏋?	release_folio() 澶辫触锛屽畠搴斿綋杩斿洖 false銆俽elease_folio() 鐢ㄤ簬涓や釜涓嶅悓浣嗙浉鍏崇殑
	鎯呭喌銆傜涓€涓槸褰?VM 鎯宠閲婃斁涓€涓病鏈夋椿鍔ㄧ敤鎴风殑骞插噣 folio 鏃躲€傚鏋?->release_folio
	鎴愬姛锛岃 folio 灏嗕粠鍦板潃绌洪棿绉婚櫎骞惰閲婃斁銆?
	绗簩绉嶆儏鍐垫槸褰撹姹備娇鍦板潃绌洪棿涓殑閮ㄥ垎鎴栧叏閮?folio 澶辨晥鏃躲€傝繖鍙兘閫氳繃
	fadvise(POSIX_FADV_DONTNEED) 绯荤粺璋冪敤鍙戠敓锛屾垨鑰呯敱鏂囦欢绯荤粺鏄惧紡璇锋眰锛堝鍚?nfs 涓?	9p 鎵€鍋氱殑锛屽綋瀹冧滑璁や负缂撳瓨鍙兘涓庡瓨鍌ㄤ笉涓€鑷存椂锛夐€氳繃璋冪敤 invalidate_inode_pages2()銆?	濡傛灉鏂囦欢绯荤粺杩涜浜嗚繖鏍风殑璋冪敤锛屽苟涓旈渶瑕佺‘淇濇墍鏈夌殑 folio 閮借澶辨晥锛岄偅涔堝畠鐨?	release_folio 灏嗛渶瑕佺‘淇濊繖涓€鐐广€傚鏋滃畠灏氫笉鑳介噴鏀剧鏈夋暟鎹紝瀹冩垨璁稿彲浠ユ竻闄?uptodate
	鏍囧織銆?
`free_folio`
	涓€鏃?folio 鍦ㄩ〉缂撳瓨涓笉鍐嶅彲瑙侊紝灏辫皟鐢?free_folio锛屼互鍏佽娓呯悊浠讳綍绉佹湁鏁版嵁銆傜敱浜?	瀹冨彲鑳界敱鍐呭瓨鍥炴敹鍣ㄨ皟鐢紝瀹冧笉搴斿亣璁惧師濮嬬殑鍦板潃绌洪棿鏄犲皠浠嶇劧瀛樺湪锛屽苟涓斾笉搴旈樆濉炪€?
`direct_IO`
	鐢遍€氱敤鐨勮/鍐欎緥绋嬭皟鐢紝浠ユ墽琛?direct_IO鈥斺€斿嵆缁曡繃椤电紦瀛樸€佺洿鎺ュ湪瀛樺偍涓庡簲鐢ㄧ▼搴?	鍦板潃绌洪棿涔嬮棿浼犺緭鏁版嵁鐨?I/O 璇锋眰銆?
`migrate_folio`
	杩欑敤浜庡帇缂╃墿鐞嗗唴瀛樼殑浣跨敤銆傚鏋?VM 鎯宠閲嶅畾浣嶄竴涓?folio锛堜篃璁镐粠涓€涓彂鍑哄嵆灏?	鏁呴殰淇″彿鐨勫瓨鍌ㄨ澶囷級锛屽畠浼氬悜璇ュ嚱鏁颁紶鍏ヤ竴涓柊鐨?folio 涓庝竴涓棫鐨?folio銆?	migrate_folio 搴斿綋杞Щ浠讳綍绉佹湁鏁版嵁锛屽苟鏇存柊瀹冨 folio 鐨勪换浣曞紩鐢ㄣ€?
`launder_folio`
	鍦ㄩ噴鏀?folio 涔嬪墠璋冪敤鈥斺€斿畠灏嗚剰 folio 鍥炲啓銆備负浜嗛槻姝?folio 鍐嶆鍙樿剰锛屽畠鍦ㄦ暣涓?	鎿嶄綔鏈熼棿淇濇寔閿佸畾銆?
`is_partially_uptodate`
	褰撻€氳繃椤电紦瀛樿鍙栨枃浠讹紝涓斿簳灞傚潡澶у皬灏忎簬 folio 澶у皬鏃剁敱 VM 璋冪敤銆傚鏋滄墍闇€鐨勫潡
	鏄渶鏂扮殑锛屽垯璇诲彇鏃犻渶 I/O 鍗冲彲瀹屾垚锛岃€屾棤闇€灏嗘暣涓〉鏇存柊鍒版渶鏂般€?
`is_dirty_writeback`
	褰?VM 灏濊瘯鍥炴敹涓€涓?folio 鏃惰皟鐢ㄣ€俈M 浣跨敤鑴忎笌鍥炲啓淇℃伅鏉ョ‘瀹氭槸鍚﹂渶瑕佸仠椤匡紙stall锛?	浠ョ粰 flusher 涓€涓畬鎴愭煇浜?I/O 鐨勬満浼氥€傞€氬父瀹冨彲浠ヤ娇鐢?folio_test_dirty 涓?	folio_test_writeback锛屼絾鏌愪簺鏂囦欢绯荤粺鏈夋洿澶嶆潅鐨勭姸鎬侊紙NFS 涓笉绋冲畾鐨?folio 浼氶樆姝?	鍥炴敹锛夛紝鎴栬€呯敱浜庡姞閿侀棶棰樿€屼笉璁剧疆閭ｄ簺鏍囧織銆傝鍥炶皟鍏佽鏂囦欢绯荤粺鍚?VM 鎸囩ず涓€涓?	folio 鏄惁搴斿綋涓轰簡鍋滈】鐨勭洰鐨勮€岃褰撲綔鑴忕殑鎴栧洖鍐欑殑銆?
`error_remove_folio`
	濡傛灉瀵硅鍦板潃绌洪棿鍏佽鎴柇锛岄€氬父璁句负 generic_error_remove_folio銆傜敤浜庡唴瀛樻晠闅?	锛坢emory failure锛夊鐞嗐€傝缃畠鎰忓懗鐫€浣犺澶勭悊椤靛湪浣犱箣涓嬫秷澶辩殑鎯呭喌锛岄櫎闈炰綘宸?	灏嗗畠浠攣瀹氭垨澧炲姞浜嗗紩鐢ㄨ鏁般€?
`swap_activate`
	琚皟鐢ㄤ互涓虹粰瀹氱殑鏂囦欢鍑嗗浜ゆ崲銆傚畠搴斿綋鎵ц浠讳綍蹇呰鐨勯獙璇佷笌鍑嗗锛屼互纭繚鍐欏叆鑳藉
	浠ユ渶灏忕殑鍐呭瓨鍒嗛厤瀹屾垚銆傚畠搴斿綋璋冪敤 add_swap_extent()锛屾垨杈呭姪鍑芥暟
	iomap_swapfile_activate()锛屽苟杩斿洖鎵€娣诲姞鐨勫尯娈碉紙extent锛夋暟閲忋€傚鏋?I/O 搴斿綋閫氳繃
	->swap_rw() 鎻愪氦锛屽畠搴斿綋璁剧疆 SWP_FS_OPS锛屽惁鍒?I/O 灏嗚鐩存帴鎻愪氦鍒板潡璁惧
	`sis->bdev`銆?
`swap_deactivate`
	鍦ㄥ swap_activate 鎴愬姛鐨勬枃浠舵墽琛?swapoff 鏈熼棿璋冪敤銆?
`swap_rw`
	褰撹缃簡 SWP_FS_OPS 鏃惰皟鐢紝浠ヨ鍙栨垨鍐欏叆浜ゆ崲椤点€?
## File 瀵硅薄


涓€涓?file 瀵硅薄浠ｈ〃涓€涓杩涚▼鎵撳紑鐨勬枃浠躲€傚湪 POSIX 鏈涓紝杩欎篃琚О涓?鎵撳紑鏂囦欢
鎻忚堪"锛坥pen file description锛夈€?

### struct file_operations

杩欐弿杩颁簡 VFS 濡備綍鎿嶄綔涓€涓墦寮€鐨勬枃浠躲€傝嚜鍐呮牳 4.18 璧凤紝瀹氫箟浜嗕互涓嬫垚鍛橈細


	struct file_operations {
		struct module *owner;
		fop_flags_t fop_flags;
		loff_t (**llseek) (struct file **, loff_t, int);
		ssize_t (**read) (struct file **, char __user **, size_t, loff_t **);
		ssize_t (**write) (struct file **, const char __user **, size_t, loff_t **);
		ssize_t (**read_iter) (struct kiocb **, struct iov_iter *);
		ssize_t (**write_iter) (struct kiocb **, struct iov_iter *);
		int (**iopoll)(struct kiocb **kiocb, struct io_comp_batch *,
				unsigned int flags);
		int (**iterate_shared) (struct file **, struct dir_context *);
		__poll_t (**poll) (struct file **, struct poll_table_struct *);
		long (**unlocked_ioctl) (struct file **, unsigned int, unsigned long);
		long (**compat_ioctl) (struct file **, unsigned int, unsigned long);
		int (**mmap) (struct file **, struct vm_area_struct *);
		int (**open) (struct inode **, struct file *);
		int (**flush) (struct file **, fl_owner_t id);
		int (**release) (struct inode **, struct file *);
		int (**fsync) (struct file **, loff_t, loff_t, int datasync);
		int (**fasync) (int, struct file **, int);
		int (**lock) (struct file **, int, struct file_lock *);
		unsigned long (**get_unmapped_area)(struct file **, unsigned long, unsigned long, unsigned long, unsigned long);
		int (*check_flags)(int);
		int (**flock) (struct file **, int, struct file_lock *);
		ssize_t (**splice_write)(struct pipe_inode_info **, struct file **, loff_t **, size_t, unsigned int);
		ssize_t (**splice_read)(struct file **, loff_t **, struct pipe_inode_info **, size_t, unsigned int);
		void (**splice_eof)(struct file **file);
		int (**setlease)(struct file **, int, struct file_lease **, void **);
		long (**fallocate)(struct file **file, int mode, loff_t offset,
				  loff_t len);
		void (**show_fdinfo)(struct seq_file **m, struct file *f);
	#ifndef CONFIG_MMU
		unsigned (**mmap_capabilities)(struct file **);
	#endif
		ssize_t (**copy_file_range)(struct file **, loff_t, struct file *,
				loff_t, size_t, unsigned int);
		loff_t (**remap_file_range)(struct file **file_in, loff_t pos_in,
					   struct file *file_out, loff_t pos_out,
					   loff_t len, unsigned int remap_flags);
		int (**fadvise)(struct file **, loff_t, loff_t, int);
		int (**uring_cmd)(struct io_uring_cmd **ioucmd, unsigned int issue_flags);
		int (**uring_cmd_iopoll)(struct io_uring_cmd **, struct io_comp_batch *,
					unsigned int poll_flags);
		int (**mmap_prepare)(struct vm_area_desc **);
	};

鍚屾牱锛岄櫎闈炲彟鏈夎鏄庯紝鎵€鏈夋柟娉曢兘鍦ㄤ笉鎸佹湁浠讳綍閿佺殑鎯呭喌涓嬭皟鐢ㄣ€?
`llseek`
	褰?VFS 闇€瑕佺Щ鍔ㄦ枃浠朵綅缃储寮曟椂璋冪敤

`read`
	鐢?read(2) 鍙婄浉鍏崇殑绯荤粺璋冪敤璋冪敤

`read_iter`
	鍙兘寮傛鐨勮鍙栵紝浠?iov_iter 涓虹洰鏍?
`write`
	鐢?write(2) 鍙婄浉鍏崇殑绯荤粺璋冪敤璋冪敤

`write_iter`
	鍙兘寮傛鐨勫啓鍏ワ紝浠?iov_iter 涓烘簮

`iopoll`
	褰?aio 鎯宠鍦?HIPRI iocb 涓婅疆璇㈠畬鎴愭椂璋冪敤

`iterate_shared`
	褰?VFS 闇€瑕佽鍙栫洰褰曞唴瀹规椂璋冪敤

`poll`
	褰撹繘绋嬫兂瑕佹鏌ヨ鏂囦欢涓婃槸鍚︽湁娲诲姩锛屽苟锛堝彲閫夊湴锛変竴鐩寸潯鐪犵洿鍒版湁娲诲姩鏃讹紝鐢?VFS
	璋冪敤銆傜敱 select(2) 涓?poll(2) 绯荤粺璋冪敤璋冪敤銆?
`unlocked_ioctl`
	鐢?ioctl(2) 绯荤粺璋冪敤璋冪敤銆?
`compat_ioctl`
	褰撳湪 64 浣嶅唴鏍镐笂浣跨敤 32 浣嶇郴缁熻皟鐢ㄦ椂锛岀敱 ioctl(2) 绯荤粺璋冪敤璋冪敤銆?
`mmap`
	鐢?mmap(2) 绯荤粺璋冪敤璋冪敤銆傚凡搴熷純锛屾帹鑽愪娇鐢?`mmap_prepare`銆?
`open`
	褰撳簲褰撴墦寮€涓€涓?inode 鏃剁敱 VFS 璋冪敤銆傚綋 VFS 鎵撳紑涓€涓枃浠舵椂锛屽畠鍒涘缓涓€涓柊鐨?	"struct file"銆傜劧鍚庡畠涓鸿繖涓柊鍒嗛厤鐨勬枃浠剁粨鏋勮皟鐢?open 鏂规硶銆備綘涔熻浼氳涓?open
	鏂规硶纭疄灞炰簬 "struct inode_operations"锛屼綘涔熻鏄鐨勩€傛垜鎯冲畠鏄互鐜板湪杩欑鏂瑰紡
	瀹屾垚鐨勶紝鍥犱负杩欒鏂囦欢绯荤粺瀹炵幇璧锋潵鏇寸畝鍗曘€傚鏋滀綘鎯宠鎸囧悜涓€涓澶囩粨鏋勶紝open()
	鏂规硶鏄垵濮嬪寲 file 缁撴瀯涓殑 "private_data" 鎴愬憳鐨勫ソ鍦版柟銆?
`flush`
	鐢?close(2) 绯荤粺璋冪敤璋冪敤锛屼互鍒锋柊涓€涓枃浠躲€?
`release`
	褰撳涓€涓墦寮€鏂囦欢鐨勬渶鍚庝竴娆″紩鐢ㄨ鍏抽棴鏃惰皟鐢ㄣ€?
`fsync`
	鐢?fsync(2) 绯荤粺璋冪敤璋冪敤銆傚彟瑙佷笂鏂囨爣棰樹负"鍥炲啓鏈熼棿鐨勯敊璇鐞?涓€鑺傘€?
`fasync`
	褰撲负鏂囦欢鍚敤寮傛锛堥潪闃诲锛夋ā寮忔椂锛岀敱 fcntl(2) 绯荤粺璋冪敤璋冪敤銆?
`lock`
	鐢?fcntl(2) 绯荤粺璋冪敤閽堝 F_GETLK銆丗_SETLK 涓?F_SETLKW 鍛戒护璋冪敤銆?
`get_unmapped_area`
	鐢?mmap(2) 绯荤粺璋冪敤璋冪敤銆?
`check_flags`
	鐢?fcntl(2) 绯荤粺璋冪敤閽堝 F_SETFL 鍛戒护璋冪敤銆?
`flock`
	鐢?flock(2) 绯荤粺璋冪敤璋冪敤銆?
`splice_write`
	鐢?VFS 璋冪敤锛屼互灏嗘暟鎹粠绠￠亾鎷兼帴锛坰plice锛夊埌鏂囦欢銆傝鏂规硶琚?splice(2) 绯荤粺璋冪敤浣跨敤銆?
`splice_read`
	鐢?VFS 璋冪敤锛屼互灏嗘暟鎹粠鏂囦欢鎷兼帴鍒扮閬撱€傝鏂规硶琚?splice(2) 绯荤粺璋冪敤浣跨敤銆?
`setlease`
	鐢?VFS 璋冪敤锛屼互璁剧疆鎴栭噴鏀炬枃浠堕攣绉熺害锛坙ease锛夈€傚笇鏈涗娇鐢ㄥ唴鏍稿唴閮ㄧ绾﹀疄鐜扮殑鏈湴
	鏂囦欢绯荤粺搴斿皢姝よ涓?generic_setlease()銆傚叾浠?setlease 瀹炵幇搴斿湪璁剧疆涔嬪悗璋冪敤
	generic_setlease() 鏉ヨ褰曟垨绉婚櫎 inode 涓殑绉熺害銆傚綋璁句负 NULL 鏃讹紝灏濊瘯璁剧疆鎴栫Щ闄?	绉熺害灏嗚繑鍥?-EINVAL銆?
`fallocate`
	鐢?VFS 璋冪敤锛屼互棰勫垎閰嶅潡鎴栨墦娲烇紙punch a hole锛夈€?
`copy_file_range`
	鐢?copy_file_range(2) 绯荤粺璋冪敤璋冪敤銆?
`remap_file_range`
	鐢?ioctl(2) 绯荤粺璋冪敤閽堝 FICLONERANGE銆丗ICLONE 涓?FIDEDUPERANGE 鍛戒护璋冪敤锛屼互閲嶆槧灏?	鏂囦欢鑼冨洿銆備竴涓疄鐜板簲褰撳皢婧愭枃浠?pos_in 澶勭殑 len 瀛楄妭閲嶆槧灏勫埌鐩爣鏂囦欢 pos_out 澶勩€?	瀹炵幇蹇呴』澶勭悊璋冪敤鑰呬紶鍏?len == 0 鐨勬儏鍐碉紱杩欐剰鍛崇潃"閲嶆槧灏勫埌婧愭枃浠剁殑鏈熬"銆傝繑鍥炲€?	搴旀槸琚噸鏄犲皠鐨勫瓧鑺傛暟锛屾垨鑰呭鏋滃湪浠讳綍瀛楄妭琚噸鏄犲皠涔嬪墠鍙戠敓閿欒锛屽垯鏄€氬父鐨勮礋閿欒鐮併€?	remap_flags 鍙傛暟鎺ュ彈 REMAP_FILE_* 鏍囧織銆傚鏋滆缃簡 REMAP_FILE_DEDUP锛屽垯瀹炵幇蹇呴』
	浠呭湪鎵€璇锋眰鐨勬枃浠惰寖鍥村唴瀹瑰畬鍏ㄧ浉鍚屾椂鎵嶉噸鏄犲皠銆傚鏋滆缃簡 REMAP_FILE_CAN_SHORTEN锛?	璋冪敤鑰呭彲浠ユ帴鍙楀疄鐜扮缉鐭姹傞暱搴︿互婊¤冻瀵归綈鎴?EOF 瑕佹眰锛堟垨浠讳綍鍏朵粬鍘熷洜锛夈€?
`fadvise`
	鍙兘鐢?fadvise64() 绯荤粺璋冪敤璋冪敤銆?
`mmap_prepare`
	鐢?mmap(2) 绯荤粺璋冪敤璋冪敤銆傚厑璁?VFS 寤虹珛鏂囦欢鏀寔鐨勶紙file-backed锛夊唴瀛樻槧灏勶紝鏈€鏄捐憲鍦?	鏄缓绔嬬浉鍏崇殑绉佹湁鐘舵€佷笌 VMA 鍥炶皟銆?
	濡傛灉杩橀渶瑕佽繘涓€姝ョ殑鎿嶄綔锛屼緥濡傞〉琛ㄧ殑棰勫～鍏咃紙pre-population锛夛紝杩欏彲浠ラ€氳繃
	vm_area_desc->action 瀛楁鍙婄浉鍏崇殑鍙傛暟鏉ユ寚瀹氥€?
娉ㄦ剰锛屾枃浠舵搷浣滄槸鐢?inode 鎵€鍦ㄧ殑鐗瑰畾鏂囦欢绯荤粺瀹炵幇鐨勩€傚綋鎵撳紑涓€涓澶囪妭鐐癸紙瀛楃鎴栧潡
鐗规畩鏂囦欢锛夋椂锛屽ぇ澶氭暟鏂囦欢绯荤粺浼氳皟鐢?VFS 涓殑鐗规畩鏀寔渚嬬▼锛岃繖浜涗緥绋嬪皢瀹氫綅鎵€闇€鐨勮澶?椹卞姩淇℃伅銆傝繖浜涙敮鎸佷緥绋嬪皢鏂囦欢绯荤粺鐨勬枃浠舵搷浣滄浛鎹负璁惧椹卞姩鐨勯偅浜涙搷浣滐紝鐒跺悗缁х画璋冪敤
璇ユ枃浠舵柊鐨?open() 鏂规硶銆傝繖灏辨槸鍦ㄦ枃浠剁郴缁熶腑鎵撳紑涓€涓澶囨枃浠舵渶缁堜細璋冪敤鍒拌澶囬┍鍔?open() 鏂规硶鐨勬柟寮忋€?

## 鐩綍椤圭紦瀛橈紙dcache锛?


### struct dentry_operations

杩欐弿杩颁簡鏂囦欢绯荤粺濡備綍閲嶈浇鏍囧噯鐨?dentry 鎿嶄綔銆侱entry 涓?dcache 鏄?VFS 涓庡悇涓枃浠剁郴缁?瀹炵幇鐨勫湴鐩樸€傝澶囬┍鍔ㄤ笌姝ゆ棤鍏炽€傝繖浜涙柟娉曞彲浠ヨ涓?NULL锛屽洜涓哄畠浠涔堟槸鍙€夌殑锛岃涔?VFS 浣跨敤榛樿鍊笺€傝嚜鍐呮牳 2.6.22 璧凤紝瀹氫箟浜嗕互涓嬫垚鍛橈細


	struct dentry_operations {
		int (**d_revalidate)(struct inode **, const struct qstr *,
				    struct dentry *, unsigned int);
		int (**d_weak_revalidate)(struct dentry **, unsigned int);
		int (**d_hash)(const struct dentry **, struct qstr *);
		int (**d_compare)(const struct dentry **,
				 unsigned int, const char **, const struct qstr **);
		int (**d_delete)(const struct dentry **);
		int (**d_init)(struct dentry **);
		void (**d_release)(struct dentry **);
		void (**d_iput)(struct dentry **, struct inode *);
		char **(**d_dname)(struct dentry **, char **, int);
		struct vfsmount **(**d_automount)(struct path *);
		int (**d_manage)(const struct path **, bool);
		struct dentry **(**d_real)(struct dentry *, enum d_real_type type);
		bool (**d_unalias_trylock)(const struct dentry **);
		void (**d_unalias_unlock)(const struct dentry **);
	};

`d_revalidate`
	褰?VFS 闇€瑕侀噸鏂伴獙璇侊紙revalidate锛変竴涓?dentry 鏃惰皟鐢ㄣ€傛瘡褰撳悕绉版煡鎵惧湪 dcache 涓?	鎵惧埌涓€涓?dentry 鏃跺氨浼氳皟鐢ㄥ畠銆傚ぇ澶氭暟鏈湴鏂囦欢绯荤粺灏嗗叾淇濈暀涓?NULL锛屽洜涓哄畠浠湪
	dcache 涓殑鎵€鏈?dentry 閮芥槸鏈夋晥鐨勩€傜綉缁滄枃浠剁郴缁熷垯涓嶅悓锛屽洜涓烘湇鍔″櫒涓婄殑浜嬫儏鍙互鍦?	瀹㈡埛绔湭蹇呯煡鎯呯殑鎯呭喌涓嬪彂鐢熷彉鍖栥€?
	濡傛灉 dentry 浠嶇劧鏈夋晥锛岃鍑芥暟搴旇繑鍥炰竴涓鍊硷紱濡傛灉鏃犳晥锛屽垯杩斿洖闆舵垨涓€涓礋鐨勯敊璇爜銆?
	d_revalidate 鍙兘鍦?rcu-walk 妯″紡涓嬭皟鐢紙flags & LOOKUP_RCU锛夈€傚鏋滃湪 rcu-walk
	妯″紡涓嬶紝鏂囦欢绯荤粺蹇呴』鍦ㄤ笉闃诲鎴栦笉鍐欏叆 dentry 鐨勬儏鍐典笅閲嶆柊楠岃瘉 dentry锛宒_parent 涓?	d_inode 涓嶅簲鍦ㄦ病鏈夊皬蹇冪殑鎯呭喌涓嬩娇鐢紙鍥犱负瀹冧滑鍙兘鏀瑰彉锛屽苟涓斿湪 d_inode 鐨勬儏鍐典笅锛?	鐢氳嚦鍙兘鍦ㄦ垜浠殑澶勭悊杩囩▼涓彉鎴?NULL锛夈€?
	濡傛灉閬囧埌 rcu-walk 鏃犳硶澶勭悊鐨勬儏鍐碉紝杩斿洖 -ECHILD锛屽畠灏嗗湪 ref-walk 妯″紡涓嬪啀娆¤璋冪敤銆?
`d_weak_revalidate`
	褰?VFS 闇€瑕侀噸鏂伴獙璇佷竴涓?璺宠繃鐨?锛坖umped锛塪entry 鏃惰皟鐢ㄣ€傝繖鍦ㄤ竴涓矾寰勯亶鍘嗙粨鏉熶簬
	涓€涓笉鏄€氳繃鍦ㄧ埗鐩綍涓煡鎵捐€岃幏寰楃殑 dentry 鏃惰皟鐢ㄣ€傝繖鍖呮嫭 "/"銆?." 涓?".."锛?	浠ュ強 procfs 椋庢牸鐨勭鍙烽摼鎺ヤ笌鎸傝浇鐐归亶鍘嗐€?
	鍦ㄨ繖绉嶆儏鍐典笅锛屾垜浠緝灏戝叧蹇?dentry 鏄惁浠嶇劧瀹屽叏姝ｇ‘锛岃€屾洿鍏冲績 inode 鏄惁浠嶇劧鏈夋晥銆?	涓?d_revalidate 涓€鏍凤紝澶у鏁版湰鍦版枃浠剁郴缁熶細灏嗗叾璁句负 NULL锛屽洜涓哄畠浠殑 dcache 鏉＄洰
	鎬绘槸鏈夋晥鐨勩€?
	璇ュ嚱鏁扮殑杩斿洖鐮佽涔変笌 d_revalidate 鐩稿悓銆?
	d_weak_revalidate 鍙湪绂诲紑 rcu-walk 妯″紡涔嬪悗璋冪敤銆?
`d_hash`
	褰?VFS 灏嗕竴涓?dentry 鍔犲叆鍝堝笇琛ㄦ椂璋冪敤銆備紶缁?d_hash 鐨勭涓€涓?dentry 鏄灏嗗悕绉?	鍝堝笇鍒扮殑鐖剁洰褰曘€?
	鍏充簬浠€涔堝彲浠ュ畨鍏ㄨВ寮曠敤绛夛紝涓?d_compare 鏈夌浉鍚岀殑鍔犻攣涓庡悓姝ヨ鍒欍€?
`d_compare`
	璋冪敤浠ュ皢 dentry 鍚嶇О涓庣粰瀹氬悕绉版瘮杈冦€傜涓€涓?dentry 鏄姣旇緝鐨?dentry 鐨勭埗鐩綍锛?	绗簩涓槸瀛?dentry銆俵en 涓?name 瀛楃涓叉槸瑕佹瘮杈冪殑 dentry 鐨勫睘鎬с€俼str 鏄涓庝箣
	姣旇緝鐨勫悕绉般€?
	蹇呴』鏄父閲忎笖骞傜瓑鐨勶紝骞跺簲灏藉彲鑳戒笉鍔犻攣锛屼笖涓嶅簲鍐欏叆 dentry銆備笉搴斿湪娌℃湁澶ч噺灏忓績鐨?	鎯呭喌涓嬭В寮曠敤 dentry 涔嬪鐨勬寚閽堬紙渚嬪锛屼笉搴斾娇鐢?d_parent銆乨_inode銆乨_name锛夈€?
	鐒惰€岋紝鎴戜滑鐨?vfsmount 鏄鍥哄畾鐨勶紝涓旀寔鏈?RCU锛屽洜姝?dentry 涓?inode 涓嶄細娑堝け锛屾垜浠?	鐨?sb 鎴栨枃浠剁郴缁熸ā鍧椾篃涓嶄細銆傚彲浠ヤ娇鐢?->d_sb銆?
	杩欐槸涓€涓鎵嬬殑璋冪敤绾﹀畾锛屽洜涓哄畠闇€瑕佸湪"rcu-walk"涓嬭皟鐢紝鍗虫病鏈変换浣曢攣鎴栧浜嬬墿鐨?	寮曠敤銆?
`d_delete`
	褰撳涓€涓?dentry 鐨勬渶鍚庝竴娆″紩鐢ㄨ鏀惧純銆佷笖 dcache 姝ｅ湪鍐冲畾鏄惁缂撳瓨瀹冩椂璋冪敤銆傝繑鍥?1
	琛ㄧず绔嬪嵆鍒犻櫎锛屾垨杩斿洖 0 琛ㄧず缂撳瓨璇?dentry銆傞粯璁ゆ槸 NULL锛屾剰鍛崇潃鎬绘槸缂撳瓨涓€涓彲杈剧殑
	dentry銆俤_delete 蹇呴』鏄父閲忎笖骞傜瓑鐨勩€?
`d_init`
	褰撲竴涓?dentry 琚垎閰嶆椂璋冪敤銆?
`d_release`
	褰撲竴涓?dentry 鐪熸琚噴鏀炬椂璋冪敤銆?
`d_iput`
	褰撲竴涓?dentry 澶卞幓鍏?inode 鏃讹紙灏卞湪瀹冭閲婃斁涔嬪墠锛夎皟鐢ㄣ€傚綋瀹冧负 NULL 鏃剁殑榛樿琛屼负鏄?	VFS 璋冪敤 iput()銆傚鏋滀綘瀹氫箟浜嗚鏂规硶锛屽垯蹇呴』鑷繁璋冪敤 iput()銆?
`d_dname`
	褰撻渶瑕佺敓鎴愪竴涓?dentry 鐨勮矾寰勫悕鏃惰皟鐢ㄣ€傚鏌愪簺浼枃浠剁郴缁燂紙sockfs銆乸ipefs 绛夛級鏈夌敤锛?	鐢ㄤ簬寤惰繜璺緞鍚嶇敓鎴愩€傦紙涓嶆槸鍦?dentry 鍒涘缓鏃跺仛锛岃€屾槸浠呭湪闇€瑕佽矾寰勬椂鎵嶅仛銆傦級鐪熷疄鏂囦欢
	绯荤粺澶ф涓嶆兂瑕佷娇鐢ㄥ畠锛屽洜涓哄畠浠殑 dentry 瀛樺湪浜庡叏灞€ dcache 鍝堝笇涓紝鍥犳瀹冧滑鐨勫搱甯?	搴斿綋鏄笉鍙樼殑銆傜敱浜庢病鏈夋寔閿侊紝d_dname() 涓嶅簲灏濊瘯淇敼 dentry 鏈韩锛岄櫎闈炰娇鐢ㄤ簡閫傚綋鐨?	SMP 瀹夊叏鎵嬫銆傛敞鎰忥細d_path() 鐨勯€昏緫鐩稿綋妫樻墜銆備緥濡傝繑鍥?"Hello" 鐨勬纭柟寮忔槸灏嗗畠
	鏀惧湪缂撳啿鍖虹殑鏈熬锛屽苟杩斿洖涓€涓寚鍚戠涓€涓瓧绗︾殑鎸囬拡銆傛彁渚涗簡 dynamic_dname() 杈呭姪
	鍑芥暟鏉ュ鐞嗚繖浠朵簨銆?
	绀轰緥锛?

	static char **pipefs_dname(struct dentry **dent, char *buffer, int buflen)
	{
		return dynamic_dname(dentry, buffer, buflen, "pipe:[%lu]",
				dentry->d_inode->i_ino);
	}

`d_automount`
	褰撹閬嶅巻涓€涓嚜鍔ㄦ寕杞斤紙automount锛塪entry 鏃惰皟鐢紙鍙€夛級銆傝繖搴斿綋鍒涘缓涓€涓柊鐨?VFS
	鎸傝浇璁板綍锛屽苟灏嗚璁板綍杩斿洖缁欒皟鐢ㄨ€呫€傝皟鐢ㄨ€呰鎻愪緵涓€涓?path 鍙傛暟锛岀粰鍑虹敤浜庢弿杩拌嚜鍔ㄦ寕杞?	鐩爣鐨勮嚜鍔ㄦ寕杞界洰褰曪紝浠ュ強鎻愪緵鍙户鎵挎寕杞藉弬鏁扮殑鐖?VFS 鎸傝浇璁板綍銆傚鏋滃叾浠栦汉鐜囧厛瀹屾垚浜?	鑷姩鎸傝浇锛屽垯搴旇繑鍥?NULL銆傚鏋?vfsmount 鍒涘缓澶辫触锛屽垯搴旇繑鍥炰竴涓敊璇爜銆傚鏋滆繑鍥?	-EISDIR锛屽垯璇ョ洰褰曞皢琚涓烘櫘閫氱洰褰曞苟杩旇繕缁?pathwalk 浠ョ户缁亶鍘嗐€?
	濡傛灉杩斿洖浜嗕竴涓?vfsmount锛岃皟鐢ㄨ€呭皢灏濊瘯灏嗗叾鎸傝浇鍒版寕杞界偣涓婏紝骞跺湪澶辫触鐨勬儏鍐典笅灏嗗叾浠?	杩囨湡鍒楄〃涓Щ闄ゃ€?
	璇ュ嚱鏁颁粎鍦?dentry 涓婅缃簡 DCACHE_NEED_AUTOMOUNT 鏃朵娇鐢ㄣ€傚鏋滄坊鍔犲埌 inode 鏃惰缃簡
	S_AUTOMOUNT锛屽垯杩欑敱 __d_instantiate() 璁剧疆銆?
`d_manage`
	璋冪敤浠ュ厑璁告枃浠剁郴缁熺鐞嗕粠涓€涓?dentry 鐨勮繃娓★紙鍙€夛級銆傝繖鍏佽 autofs 渚嬪鐣欎綇绛夊緟鍦?	"鎸傝浇鐐?鍚庨潰鎺㈢储鐨勫鎴风锛屽悓鏃惰瀹堟姢杩涚▼杩囧幓骞跺湪閭ｉ噷鏋勯€犲瓙鏍戙€傚簲褰撹繑鍥?0 浠ヨ璋冪敤
	杩涚▼缁х画銆傚彲浠ヨ繑鍥?-EISDIR 浠ュ憡璇?pathwalk 灏嗚鐩綍鐢ㄤ綔鏅€氱洰褰曪紝蹇界暐鎸傝浇鍦ㄥ叾涓婄殑
	浠讳綍涓滆タ锛屽苟涓斾笉妫€鏌ヨ嚜鍔ㄦ寕杞芥爣蹇椼€備换浣曞叾浠栭敊璇爜灏嗗畬鍏ㄤ腑姝?pathwalk銆?
	濡傛灉 'rcu_walk' 鍙傛暟涓虹湡锛屽垯璋冪敤鑰呮鍦?RCU-walk 妯″紡涓嬭繘琛岃矾寰勯亶鍘嗐€傚湪璇ユā寮忎笅
	涓嶅厑璁哥潯鐪狅紝骞朵笖鍙互閫氳繃杩斿洖 -ECHILD 鏉ヨ璋冪敤鑰呯寮€璇ユā寮忓苟鍐嶆璋冪敤銆備篃鍙互杩斿洖
	-EISDIR 浠ュ憡璇?pathwalk 蹇界暐 d_automount 鎴栦换浣曟寕杞姐€?
	璇ュ嚱鏁颁粎鍦ㄦ琚寮€鐨?dentry 涓婅缃簡 DCACHE_MANAGE_TRANSIT 鏃朵娇鐢ㄣ€?
`d_real`
	overlay/union 绫诲瀷鏂囦欢绯荤粺瀹炵幇姝ゆ柟娉曪紝浠ヨ繑鍥炶 overlay 闅愯棌鐨勬櫘閫氭枃浠剁殑涓€涓簳灞?	dentry銆?
	'type' 鍙傛暟鍙栧€?D_REAL_DATA 鎴?D_REAL_METADATA锛岀敤浜庤繑鍥炴寚鍚戞墭绠¤鏂囦欢鏁版嵁鎴?	鍏冩暟鎹殑 inode 鐨勭湡瀹炲簳灞?dentry銆?
	瀵逛簬闈炴櫘閫氭枃浠讹紝杩斿洖 'dentry' 鍙傛暟銆?
`d_unalias_trylock`
	濡傛灉瀛樺湪锛屽皢鐢?d_splice_alias() 鍦ㄧЩ鍔ㄤ竴涓鍏堝瓨鍦ㄧ殑宸查檮鍔犲埆鍚嶄箣鍓嶈皟鐢ㄣ€傝繑鍥?false
	浼氶樆姝?__d_move()锛屼娇 d_splice_alias() 浠?-ESTALE 澶辫触銆?
	鐞嗙敱锛氳缃?FS_RENAME_DOES_D_MOVE 灏嗛樆姝㈡潵鑷枃浠剁郴缁熸柟娉曞閮ㄧ殑 d_move() 涓?	d_exchange() 璋冪敤锛涚劧鑰岋紝瀹冧笉鑳戒繚璇佸凡闄勫姞鐨?dentry 涓嶄細琚?d_splice_alias() 鎵惧埌
	鐩綍 inode 鐨勯鍏堝瓨鍦ㄧ殑鍒悕鑰岄噸鍛藉悕鎴栫Щ鍔ㄣ€傞€氬父鎴戜滑涓嶄細鍦ㄦ剰锛涗笉杩囷紝鏈夋煇绉嶄笢瑗挎兂瑕?	鍦ㄩ樆濉炴搷浣滄湡闂寸ǔ瀹氭暣涓埌鏍圭殑璺緞鏃跺彲鑳介渶瑕佸畠銆傚弬瑙?9p 浣滀负涓€涓紙涓斿笇鏈涙槸鍞竴鐨勶級
	渚嬪瓙銆?
`d_unalias_unlock`
	搴斾笌 `d_unalias_trylock` 閰嶅锛涘悗鑰呭湪 __d_unalias() 涓殑 __d_move() 璋冪敤涔嬪悗璋冪敤銆?

姣忎釜 dentry 閮芥湁涓€涓寚鍚戝叾鐖?dentry 鐨勬寚閽堬紝浠ュ強涓€涓瓙 dentry 鐨勫搱甯屽垪琛ㄣ€傚瓙 dentry
鍩烘湰涓婂氨鍍忕洰褰曚腑鐨勬枃浠躲€?

### 鐩綍椤圭紦瀛?API

瀹氫箟浜嗚澶氬厑璁告枃浠剁郴缁熸搷浣?dentry 鐨勫嚱鏁帮細

`dget`
	涓哄凡瀛樺湪鐨?dentry 鎵撳紑涓€涓柊鍙ユ焺锛堣繖鍙槸閫掑浣跨敤璁℃暟锛夈€?
`dput`
	鍏抽棴涓€涓?dentry 鐨勫彞鏌勶紙閫掑噺浣跨敤璁℃暟锛夈€傚鏋滀娇鐢ㄨ鏁伴檷鍒?0锛屼笖 dentry 浠嶅湪瀹冪埗
	鐩綍鐨勫搱甯屼腑锛屽垯璋冪敤 "d_delete" 鏂规硶妫€鏌ュ畠鏄惁搴斿綋琚紦瀛樸€傚鏋滀笉搴旇缂撳瓨锛屾垨鑰?	濡傛灉 dentry 鏈鍝堝笇锛屽垯瀹冭鍒犻櫎銆傚惁鍒欙紝缂撳瓨鐨?dentry 琚斁鍏ヤ竴涓?LRU 鍒楄〃锛屼互渚垮湪
	鍐呭瓨涓嶈冻鏃惰鍥炴敹銆?
`d_drop`
	杩欏皢涓€涓?dentry 浠庡畠鐖剁洰褰曠殑鍝堝笇鍒楄〃涓彇娑堝搱甯屻€傞殢鍚庡 dput() 鐨勮皟鐢ㄥ皢鍦ㄦ dentry
	鐨勪娇鐢ㄨ鏁伴檷鍒?0 鏃堕噴鏀惧畠銆?
`d_delete`
	鍒犻櫎涓€涓?dentry銆傚鏋滄病鏈夊叾浠栧璇?dentry 鐨勬墦寮€寮曠敤锛屽垯璇?dentry 琚浆涓轰竴涓礋 dentry
	锛堣皟鐢?d_iput() 鏂规硶锛夈€傚鏋滄湁鍏朵粬寮曠敤锛屽垯鏀逛负璋冪敤 d_drop()銆?
`d_add`
	灏嗕竴涓?dentry 娣诲姞鍒板畠鐖剁洰褰曠殑鍝堝笇鍒楄〃锛岀劧鍚庤皟鐢?d_instantiate()銆?
`d_instantiate`
	灏嗕竴涓?dentry 娣诲姞鍒拌 inode 鐨勫埆鍚嶅搱甯屽垪琛紝骞舵洿鏂?"d_inode" 鎴愬憳銆俰node 缁撴瀯涓殑
	"i_count" 鎴愬憳搴斿綋琚缃?閫掑銆傚鏋?inode 鎸囬拡涓?NULL锛岃 dentry 琚О涓?璐?dentry"銆?	褰撲竴涓?inode 涓哄凡瀛樺湪鐨勮礋 dentry 鍒涘缓鏃讹紝閫氬父浼氳皟鐢ㄦ鍑芥暟銆?
`d_lookup`
	缁欏畾鍏剁埗鐩綍涓庤矾寰勫悕鍒嗛噺锛屾煡鎵句竴涓?dentry銆傚畠浠?dcache 鍝堝笇琛ㄤ腑鏌ユ壘鍏锋湁璇ュ悕绉扮殑
	瀛愰」銆傚鏋滄壘鍒帮紝鍒欓€掑寮曠敤璁℃暟骞惰繑鍥炶 dentry銆傝皟鐢ㄨ€呬娇鐢ㄥ畬姣曞悗蹇呴』鐢?dput() 閲婃斁
	璇?dentry銆?

## 鎸傝浇閫夐」



### 瑙ｆ瀽閫夐」

鍦ㄦ寕杞戒笌閲嶆柊鎸傝浇鏃讹紝鏂囦欢绯荤粺浼氭敹鍒颁竴涓瓧绗︿覆锛屽叾涓寘鍚互閫楀彿鍒嗛殧鐨勬寕杞介€夐」鍒楄〃銆?閫夐」鍙互鏄互涓嬩袱绉嶅舰寮忎箣涓€锛?
  option
  option=value

<linux/parser.h> 澶存枃浠跺畾涔変簡涓€涓湁鍔╀簬瑙ｆ瀽杩欎簺閫夐」鐨?API銆傚湪鐜版湁鐨勬枃浠剁郴缁熶腑锛屾湁
澶ч噺濡備綍浣跨敤瀹冪殑绀轰緥銆?

### 鏄剧ず閫夐」

濡傛灉涓€涓枃浠剁郴缁熸帴鍙楁寕杞介€夐」锛屽畠蹇呴』瀹氫箟 show_options() 浠ユ樉绀烘墍鏈夊綋鍓嶆椿鍔ㄧ殑閫夐」銆?瑙勫垯鏄細

  - 蹇呴』鏄剧ず閭ｄ簺闈為粯璁ゃ€佹垨鍏跺€间笌榛樿鍊间笉鍚岀殑閫夐」

  - 鍙互鏄剧ず閭ｄ簺榛樿鍚敤銆佹垨鍏锋湁榛樿鍊肩殑閫夐」

浠呭湪鎸傝浇杈呭姪绋嬪簭涓庡唴鏍镐箣闂村唴閮ㄤ娇鐢紙渚嬪鏂囦欢鎻忚堪绗︼級锛屾垨浠呭湪鎸傝浇鏈熼棿璧蜂綔鐢紙渚嬪
鎺у埗鏃ュ織锛坖ournal锛夊垱寤虹殑閭ｄ簺锛夌殑閫夐」鍏嶄簬涓婅堪瑙勫垯銆?
涓婅堪瑙勫垯鐨勬牴鏈師鍥犳槸纭繚鍙互鍩轰簬 /proc/mounts 涓壘鍒扮殑淇℃伅鍑嗙‘鍦板鍒朵竴娆℃寕杞斤紙渚嬪
鍗歌浇鍚庡啀娆℃寕杞斤級銆?

## 璧勬簮


锛堟敞鎰忥紝鍏朵腑涓€浜涜祫婧愭湭涓庢渶鏂板唴鏍哥増鏈繚鎸佸悓姝ャ€傦級

Creating Linux virtual filesystems. 2002
    <https://lwn.net/Articles/13325/>

The Linux Virtual File-system Layer by Neil Brown. 1999
    <http://www.cse.unsw.edu.au/~neilb/oss/linux-commentary/vfs.html>

A tour of the Linux VFS by Michael K. Johnson. 1996
    <https://www.tldp.org/LDP/khg/HyperNews/get/fs/vfstour.html>

A small trail through the Linux kernel by Andries Brouwer. 2001
    <https://www.win.tue.nl/~aeb/linux/vfs/trail.html>
