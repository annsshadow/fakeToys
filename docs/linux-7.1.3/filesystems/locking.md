## 鍔犻攣锛圠ocking锛?


涓嬫枃鎻忚堪浜嗕笌 VFS 鐩稿叧鏂规硶鐨勫姞閿佽鍒欍€傚畠锛堟嵁淇★級鏄渶鏂扮殑銆?*璇?*锛屽鏋滀綘鏇存敼浜嗕换浣曞師鍨嬫垨鍔犻攣鍗忚鈥斺€旇鏇存柊鏈枃浠躲€傚苟鏇存柊浠ｇ爜鏍戜腑鐨勭浉鍏冲疄渚嬶紝涓嶈鐣欑粰鏂囦欢绯荤粺/璁惧绛夌殑缁存姢鑰呭幓鍋氥€傝嚦灏戯紝鎶婂彲鐤戞儏鍐电殑鍒楄〃鏀惧埌鏈枃浠舵湯灏俱€備笉瑕佹妸瀹冨彉鎴愭棩蹇椻€斺€旀爲澶栦唬鐮佺殑缁存姢鑰呭簲褰撹兘澶熶娇鐢?diff(1)銆?

姝ゅ鐩墠缂哄け鐨勫唴瀹癸細socket 鎿嶄綔銆侫lexey锛?

## dentry_operations


```
	int (*d_revalidate)(struct inode *, const struct qstr *,
			    struct dentry *, unsigned int);
	int (*d_weak_revalidate)(struct dentry *, unsigned int);
	int (*d_hash)(const struct dentry *, struct qstr *);
	int (*d_compare)(const struct dentry *,
			unsigned int, const char *, const struct qstr *);
	int (*d_delete)(struct dentry *);
	int (*d_init)(struct dentry *);
	void (*d_release)(struct dentry *);
	void (*d_iput)(struct dentry *, struct inode *);
	char *(*d_dname)((struct dentry *dentry, char *buffer, int buflen);
	struct vfsmount *(*d_automount)(struct path *path);
	int (*d_manage)(const struct path *, bool);
	struct dentry *(*d_real)(struct dentry *, enum d_real_type type);
	bool (*d_unalias_trylock)(const struct dentry *);
	void (*d_unalias_unlock)(const struct dentry *);

```
鍔犻攣瑙勫垯锛?

================== ===========	========	==============	========
ops		   rename_lock	->d_lock	may block	rcu-walk
================== ===========	========	==============	========
d_revalidate:	   no		no		yes (ref-walk)	maybe
d_weak_revalidate: no		no		yes	 	no
d_hash		   no		no		no		maybe
d_compare:	   yes		no		no		maybe
d_delete:	   no		yes		no		no
d_init:		   no		no		yes		no
d_release:	   no		no		yes		no
d_prune:           no		yes		no		no
d_iput:		   no		no		yes		no
d_dname:	   no		no		no		no
d_automount:	   no		no		yes		no
d_manage:	   no		no		yes (ref-walk)	maybe
d_real		   no		no		yes 		no
d_unalias_trylock  yes		no		no 		no
d_unalias_unlock   yes		no		no 		no
================== ===========	========	==============	========

## inode_operations


```
	int (*create) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t, bool);
	struct dentry * (*lookup) (struct inode *,struct dentry *, unsigned int);
	int (*link) (struct dentry *,struct inode *,struct dentry *);
	int (*unlink) (struct inode *,struct dentry *);
	int (*symlink) (struct mnt_idmap *, struct inode *,struct dentry *,const char *);
	struct dentry *(*mkdir) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t);
	int (*rmdir) (struct inode *,struct dentry *);
	int (*mknod) (struct mnt_idmap *, struct inode *,struct dentry *,umode_t,dev_t);
	int (*rename) (struct mnt_idmap *, struct inode *, struct dentry *,
			struct inode *, struct dentry *, unsigned int);
	int (*readlink) (struct dentry *, char __user *,int);
	const char *(*get_link) (struct dentry *, struct inode *, struct delayed_call *);
	void (*truncate) (struct inode *);
	int (*permission) (struct mnt_idmap *, struct inode *, int, unsigned int);
	struct posix_acl * (*get_inode_acl)(struct inode *, int, bool);
	int (*setattr) (struct mnt_idmap *, struct dentry *, struct iattr *);
	int (*getattr) (struct mnt_idmap *, const struct path *, struct kstat *, u32, unsigned int);
	ssize_t (*listxattr) (struct dentry *, char *, size_t);
	int (*fiemap)(struct inode *, struct fiemap_extent_info *, u64 start, u64 len);
	void (*update_time)(struct inode *inode, enum fs_update_time type,
			    int flags);
	void (*sync_lazytime)(struct inode *inode);
	int (*atomic_open)(struct inode *, struct dentry *,
				struct file *, unsigned open_flag,
				umode_t create_mode);
	int (*tmpfile) (struct mnt_idmap *, struct inode *,
			struct file *, umode_t);
	int (*fileattr_set)(struct mnt_idmap *idmap,
			    struct dentry *dentry, struct file_kattr *fa);
	int (*fileattr_get)(struct dentry *dentry, struct file_kattr *fa);
	struct posix_acl * (*get_acl)(struct mnt_idmap *, struct dentry *, int);
	struct offset_ctx *(*get_offset_ctx)(struct inode *inode);

```
鍔犻攣瑙勫垯锛?
	鍏ㄩ儴閮藉彲鑳介樆濉?

==============	==================================================
ops		i_rwsem(inode)
==============	==================================================
lookup:		shared
create:		exclusive
link:		exclusive (both)
mknod:		exclusive
symlink:	exclusive
mkdir:		exclusive
unlink:		exclusive (both)
rmdir:		exclusive (both)(see below)
rename:		exclusive (both parents, some children)	(see below)
readlink:	no
get_link:	no
setattr:	exclusive
permission:	no (may not block if called in rcu-walk mode)
get_inode_acl:	no
get_acl:	no
getattr:	no
listxattr:	no
fiemap:		no
update_time:	no
sync_lazytime:	no
atomic_open:	shared (exclusive if O_CREAT is set in open flags)
tmpfile:	no
fileattr_get:	no or exclusive
fileattr_set:	exclusive
get_offset_ctx  no
==============	==================================================


	姝ゅ锛?>rmdir()銆?>unlink() 鍜?->rename() 瀵硅鎿嶄綔瀵硅薄锛坴ictim锛夋寔鏈?
	->i_rwsem 鐨勭嫭鍗犻攣銆?
	璺ㄧ洰褰曠殑 ->rename() 鎸佹湁锛堟瘡涓秴绾у潡锛?>s_vfs_rename_sem銆?
	->unlink() 鍜?->rename() 瀵规墍鏈夋秹鍙婄殑闈炵洰褰曢」鎸佹湁 ->i_rwsem 鐙崰閿併€?
	->rename() 瀵逛换浣曟敼鍙樼埗鐩綍鐨勫瓙鐩綍鎸佹湁 ->i_rwsem 鐙崰閿併€?

鏈夊叧鐩綍鎿嶄綔鍔犻攣鏂规鐨勬洿璇︾粏璁ㄨ锛岃鍙傞槄
Documentation/filesystems/directory-locking.rst銆?

## xattr_handler operations


```
	bool (*list)(struct dentry *dentry);
	int (*get)(const struct xattr_handler *handler, struct dentry *dentry,
		   struct inode *inode, const char *name, void *buffer,
		   size_t size);
	int (*set)(const struct xattr_handler *handler,
                   struct mnt_idmap *idmap,
                   struct dentry *dentry, struct inode *inode, const char *name,
                   const void *buffer, size_t size, int flags);

```
鍔犻攣瑙勫垯锛?
	鍏ㄩ儴閮藉彲鑳介樆濉?

=====		==============
ops		i_rwsem(inode)
=====		==============
list:		no
get:		no
set:		exclusive
=====		==============

## super_operations


```
	struct inode *(*alloc_inode)(struct super_block *sb);
	void (*free_inode)(struct inode *);
	void (*destroy_inode)(struct inode *);
	void (*dirty_inode) (struct inode *, int flags);
	int (*write_inode) (struct inode *, struct writeback_control *wbc);
	int (*drop_inode) (struct inode *);
	void (*evict_inode) (struct inode *);
	void (*put_super) (struct super_block *);
	int (*sync_fs)(struct super_block *sb, int wait);
	int (*freeze_fs) (struct super_block *);
	int (*unfreeze_fs) (struct super_block *);
	int (*statfs) (struct dentry *, struct kstatfs *);
	void (*umount_begin) (struct super_block *);
	int (*show_options)(struct seq_file *, struct dentry *);
	ssize_t (*quota_read)(struct super_block *, int, char *, size_t, loff_t);
	ssize_t (*quota_write)(struct super_block *, int, const char *, size_t, loff_t);

```
鍔犻攣瑙勫垯锛?
	鍏ㄩ儴閮藉彲鑳介樆濉?[骞堕潪濡傛锛岃涓嬫枃]

======================	============	========================
ops			s_umount	note
======================	============	========================
alloc_inode:
free_inode:				called from RCU callback
destroy_inode:
dirty_inode:
write_inode:
drop_inode:				!!!inode->i_lock!!!
evict_inode:
put_super:		write
sync_fs:		read
freeze_fs:		write
unfreeze_fs:		write
statfs:			maybe(read)	(see below)
umount_begin:		no
show_options:		no		(namespace_sem)
quota_read:		no		(see below)
quota_write:		no		(see below)
======================	============	========================

->statfs() 鍦ㄧ敱 ustat(2)锛堝師鐢熸垨鍏煎锛夎皟鐢ㄦ椂鎸佹湁 s_umount锛堝叡浜級锛屼絾杩欐槸绯熺硶 API 鐨勬剰澶栦骇鐗╋紱s_umount 鐢ㄤ簬鍦ㄦ垜浠彧鏈夌敤鎴锋€佺粰鍑虹殑 dev_t 鏉ユ爣璇嗚秴绾у潡鏃跺皢鍏跺浐瀹氫綇銆傚叾浠栦竴鍒囷紙statfs()銆乫statfs() 绛夛級鍦ㄨ皟鐢?->statfs() 鏃跺苟涓嶆寔鏈夊畠鈥斺€旇秴绾у潡閫氳繃瑙ｆ瀽浼犵粰绯荤粺璋冪敤鐨勮矾寰勫悕鏉ュ浐瀹氥€?

->quota_read() 鍜?->quota_write() 杩欎袱涓嚱鏁伴兘淇濊瘉鏄敱閰嶉浠ｇ爜锛堥€氳繃 dqio_sem锛夋搷浣滈厤棰濇枃浠剁殑鍞竴鍑芥暟锛堥櫎闈炵鐞嗗憳鐪熺殑鎯虫悶鐮镐粈涔堬紝鍦ㄩ厤棰濆紑鍚椂鍐欏叆閰嶉鏂囦欢锛夈€傛湁鍏冲姞閿佺殑鍏朵粬缁嗚妭锛屽彟璇峰弬闃?dquot_operations 涓€鑺傘€?

## file_system_type


```
	void (*kill_sb) (struct super_block *);

```
鍔犻攣瑙勫垯锛?

=======		=========
ops		may block
=======		=========
kill_sb		yes
=======		=========

->kill_sb() 鎸佹湁涓€涓啓閿佸畾鐨勮秴绾у潡锛屽湪鍏朵笂瀹屾垚鎵€鏈夊叧闂伐浣滐紝瑙ｉ攣骞堕噴鏀惧紩鐢ㄣ€?

## address_space_operations

```
	int (*read_folio)(struct file *, struct folio *);
	int (*writepages)(struct address_space *, struct writeback_control *);
	bool (*dirty_folio)(struct address_space *, struct folio *folio);
	void (*readahead)(struct readahead_control *);
	int (*write_begin)(const struct kiocb *, struct address_space *mapping,
				loff_t pos, unsigned len,
				struct folio **foliop, void **fsdata);
	int (*write_end)(const struct kiocb *, struct address_space *mapping,
				loff_t pos, unsigned len, unsigned copied,
				struct folio *folio, void *fsdata);
	sector_t (*bmap)(struct address_space *, sector_t);
	void (*invalidate_folio) (struct folio *, size_t start, size_t len);
	bool (*release_folio)(struct folio *, gfp_t);
	void (*free_folio)(struct folio *);
	int (*direct_IO)(struct kiocb *, struct iov_iter *iter);
	int (*migrate_folio)(struct address_space *, struct folio *dst,
			struct folio *src, enum migrate_mode);
	int (*launder_folio)(struct folio *);
	bool (*is_partially_uptodate)(struct folio *, size_t from, size_t count);
	int (*error_remove_folio)(struct address_space *, struct folio *);
	int (*swap_activate)(struct swap_info_struct *sis, struct file *f, sector_t *span)
	int (*swap_deactivate)(struct file *);
	int (*swap_rw)(struct kiocb *iocb, struct iov_iter *iter);

```
鍔犻攣瑙勫垯锛?
	闄?dirty_folio 鍜?free_folio 澶栵紝鍏ㄩ儴閮藉彲鑳介樆濉?

======================	======================== =========	===============
ops			folio locked		 i_rwsem	invalidate_lock
======================	======================== =========	===============
read_folio:		yes, unlocks				shared
writepages:
dirty_folio:		maybe
readahead:		yes, unlocks				shared
write_begin:		locks the folio		 exclusive
write_end:		yes, unlocks		 exclusive
bmap:
invalidate_folio:	yes					exclusive
release_folio:		yes
free_folio:		yes
direct_IO:
migrate_folio:		yes (both)
launder_folio:		yes
is_partially_uptodate:	yes
error_remove_folio:	yes
swap_activate:		no
swap_deactivate:	no
swap_rw:		yes, unlocks
======================	======================== =========	===============

->write_begin()銆?>write_end() 鍜?->read_folio() 鍙兘浠庤姹傚鐞嗙▼搴忥紙/dev/loop锛夎皟鐢ㄣ€?

->read_folio() 浼氳В閿佽 folio锛屾棤璁烘槸鍚屾鍦拌繕鏄€氳繃 I/O 瀹屾垚銆?

->readahead() 鍍?->read_folio() 涓€鏍凤紝瀵瑰皾璇曡繘琛?I/O 鐨?folio 杩涜瑙ｉ攣銆?

->writepages() 鐢ㄤ簬鍛ㄦ湡鎬у洖鍐欎互鍙婄敱绯荤粺璋冪敤鍙戣捣鐨勫悓姝ユ搷浣溿€俛ddress_space 搴斿綋閽堝鑷冲皯 `**nr_to_write` 涓〉鍚姩 I/O銆傛瘡鍐欏叆涓€涓〉锛屽繀椤婚€掑噺 `**nr_to_write`銆俛ddress_space 鐨勫疄鐜板啓鍏ョ殑椤靛彲鑳芥瘮 `*nr_to_write` 瑕佹眰鐨勫锛堟垨灏戯級锛屼絾搴斿敖閲忔帴杩戙€傚鏋?nr_to_write 涓?NULL锛屽垯蹇呴』鍐欏叆鎵€鏈夎剰椤点€?

writepages 搴斿綋_鍙猒鍐欏叆褰撳墠瀛樺湪浜?mapping->i_pages 涓殑椤点€?

->dirty_folio() 鍦ㄧ洰鏍?folio 琚爣璁颁负闇€瑕佸洖鍐欐椂锛岀敱鍐呮牳涓殑澶氬浣嶇疆璋冪敤銆傝 folio 涓嶈兘琚埅鏂紝鍥犱负瑕佷箞璋冪敤鑰呮寔鏈?folio 閿侊紝瑕佷箞璋冪敤鑰呭湪鎸佹湁椤佃〃閿佺殑鎯呭喌涓嬫壘鍒颁簡璇?folio锛岃€岄〉琛ㄩ攣浼氶樆姝㈡埅鏂€?

->bmap() 鐩墠鐢辨煇浜涙枃浠剁郴缁熸彁渚涚殑閬楃暀 ioctl()锛團IBMAP锛変互鍙婁氦鎹㈠櫒锛坰wapper锛変娇鐢ㄣ€傚悗鑰呮渶缁堜細娑堝け銆傝淇濇寔鐜扮姸锛屼笉瑕佹粙鐢熸柊鐨勮皟鐢ㄨ€呫€?

->invalidate_folio() 鍦ㄦ枃浠剁郴缁熷繀椤诲皾璇曞湪椤佃鎴柇鏃朵涪寮冭椤电殑閮ㄥ垎鎴栧叏閮ㄧ紦鍐插尯鏃惰皟鐢ㄣ€傛垚鍔熸椂杩斿洖闆躲€傛枃浠剁郴缁熷繀椤诲湪鎴柇/鎵撴礊璺緞涓娇椤电紦瀛樺け鏁堬紙骞跺洜姝よ皟鐢?->invalidate_folio锛変箣鍓嶏紝鐙崰鑾峰彇 invalidate_lock锛屼互闃绘椤电紦瀛樺け鏁堜笌椤电紦瀛樺～鍏呭嚱鏁帮紙缂洪〉銆佽鈥︹€︼級涔嬮棿鐨勭珵浜夈€?

->release_folio() 鍦?MM 鎯宠瀵?folio 鍋氬嚭浼氫娇鏂囦欢绯荤粺鐨勭鏈夋暟鎹け鏁堢殑淇敼鏃惰皟鐢ㄣ€備緥濡傦紝瀹冨彲鑳藉嵆灏嗕粠 address_space 涓Щ闄ゆ垨琚媶鍒嗐€傝 folio 澶勪簬閿佸畾鐘舵€佷笖涓嶅湪鍥炲啓涓€傚畠鍙兘鏄剰鐨勩€俫fp 鍙傛暟閫氬父涓嶇敤浜庡垎閰嶏紝鑰屾槸鐢ㄦ潵鎸囩ず鏂囦欢绯荤粺鍙互鍋氫粈涔堟潵灏濊瘯閲婃斁绉佹湁鏁版嵁銆傛枃浠剁郴缁熷彲浠ヨ繑鍥?false 浠ヨ〃绀鸿 folio 鐨勭鏈夋暟鎹棤娉曢噴鏀俱€傚鏋滆繑鍥?true锛屽畠搴旇宸茬粡灏嗙鏈夋暟鎹粠璇?folio 涓Щ闄ゃ€傚鏋滄枃浠剁郴缁熸病鏈夋彁渚?->release_folio 鏂规硶锛岄〉缂撳瓨灏嗗亣瀹氱鏈夋暟鎹槸 buffer_heads 骞惰皟鐢?try_to_free_buffers()銆?

->free_folio() 鍦ㄥ唴鏍稿皢璇?folio 浠庨〉缂撳瓨涓涪寮冩椂璋冪敤銆?

->launder_folio() 鍙兘鍦ㄩ噴鏀句竴涓?folio 涔嬪墠锛屽鏋滃畠浠嶈鍙戠幇鏄剰鐨勶紝琚皟鐢ㄣ€傚鏋?folio 琚垚鍔熸竻鐞嗗垯杩斿洖闆讹紝鍚﹀垯杩斿洖閿欒鍊笺€傛敞鎰忥紝涓轰簡闃叉 folio 琚噸鏂版槧灏勫洖鏉ュ苟閲嶆柊鍙樿剰锛屽畠闇€瑕佸湪鏁翠釜鎿嶄綔鏈熼棿淇濇寔閿佸畾銆?

->swap_activate() 灏嗚璋冪敤鏉ヤ负缁欏畾鐨勬枃浠跺噯澶囦氦鎹€傚畠搴斿綋鎵ц浠讳綍蹇呰鐨勯獙璇佸拰鍑嗗宸ヤ綔锛屼互纭繚鍐欏叆鍙互鍦ㄦ渶灏忓唴瀛樺垎閰嶇殑鎯呭喌涓嬭繘琛屻€傚畠搴斿綋璋冪敤 add_swap_extent()锛屾垨杈呭姪鍑芥暟 iomap_swapfile_activate()锛屽苟杩斿洖鎵€娣诲姞鍖烘鐨勬暟閲忋€傚鏋?IO 搴斿綋閫氳繃 ->swap_rw() 鎻愪氦锛屽畠搴斿綋璁剧疆 SWP_FS_OPS锛屽惁鍒?IO 灏嗚鐩存帴鎻愪氦鍒板潡璁惧 `sis->bdev`銆?

->swap_deactivate() 灏嗗湪 ->swap_activate() 杩斿洖鎴愬姛涔嬪悗锛屽湪 sys_swapoff() 璺緞涓璋冪敤銆?

->swap_rw 灏嗗湪璁剧疆浜?SWP_FS_OPS 鏃讹紝涓轰氦鎹?IO 琚皟鐢ㄣ€?

## file_lock_operations


```
	void (*fl_copy_lock)(struct file_lock *, struct file_lock *);
	void (*fl_release_private)(struct file_lock *);


```
鍔犻攣瑙勫垯锛?

===================	=============	=========
ops			inode->i_lock	may block
===================	=============	=========
fl_copy_lock:		yes		no
fl_release_private:	maybe		maybe[^1^]_
===================	=============	=========

   ->fl_release_private 瀵逛簬 flock 鎴?POSIX 閿侊紝鐩墠鍏佽闃诲銆備絾瀵逛簬绉熺害锛坙ease锛夛紝浠嶇劧鍙互鍦ㄦ寔鏈?i_lock 鏃堕噴鏀撅紝鍥犳绉熺害涓婅皟鐢ㄧ殑 fl_release_private 涓嶅簲闃诲銆?

## lock_manager_operations


```
	void (*lm_notify)(struct file_lock *);  /* unblock callback */
	int (*lm_grant)(struct file_lock *, struct file_lock *, int);
	void (*lm_break)(struct file_lock *); /* break_lease callback */
	int (*lm_change)(struct file_lock **, int);
	bool (*lm_breaker_owns_lease)(struct file_lock *);
        bool (*lm_lock_expirable)(struct file_lock *);
        void (*lm_expire_lock)(void);
        bool (*lm_breaker_timedout)(struct file_lease *);

```
鍔犻攣瑙勫垯锛?

======================	=============	=================	=========
ops			   flc_lock  	blocked_lock_lock	may block
======================	=============	=================	=========
lm_notify:		no      	yes			no
lm_grant:		no		no			no
lm_break:		yes		no			no
lm_change		yes		no			no
lm_breaker_owns_lease:	yes     	no			no
lm_lock_expirable	yes		no			no
lm_expire_lock		no		no			yes
lm_open_conflict	yes		no			no
lm_breaker_timedout     yes             no                      no
======================	=============	=================	=========

## buffer_head


```
	void (*b_end_io)(struct buffer_head *bh, int uptodate);

```
鍔犻攣瑙勫垯锛?

浠庝腑鏂腑璋冪敤銆傛崲鍙ヨ瘽璇达紝杩欓噷闇€瑕佹瀬搴﹀皬蹇冦€俠h 鏄攣瀹氱殑锛屼絾閭ｆ槸杩欓噷浠呮湁鐨勪繚璇併€傜洰鍓嶅彧鏈?RAID1銆乭ighmem銆乫s/buffer.c 鍜?fs/ntfs/aops.c 鎻愪緵杩欎簺銆傚潡璁惧鍦?IO 瀹屾垚鏃惰皟鐢ㄦ鏂规硶銆?

## block_device_operations

```
	int (*open) (struct block_device *, fmode_t);
	int (*release) (struct gendisk *, fmode_t);
	int (*ioctl) (struct block_device *, fmode_t, unsigned, unsigned long);
	int (*compat_ioctl) (struct block_device *, fmode_t, unsigned, unsigned long);
	int (*direct_access) (struct block_device *, sector_t, void **,
				unsigned long *);
	void (*unlock_native_capacity) (struct gendisk *);
	int (*getgeo)(struct gendisk *, struct hd_geometry *);
	void (*swap_slot_free_notify) (struct block_device *, unsigned long);

```
鍔犻攣瑙勫垯锛?

======================= ===================
ops			open_mutex
======================= ===================
open:			yes
release:		yes
ioctl:			no
compat_ioctl:		no
direct_access:		no
unlock_native_capacity:	no
getgeo:			no
swap_slot_free_notify:	no	(see below)
======================= ===================

swap_slot_free_notify 鍦ㄦ寔鏈?swap_lock 骞朵笖鏈夋椂鎸佹湁椤甸攣鐨勬儏鍐典笅琚皟鐢ㄣ€?


## file_operations


```
	loff_t (*llseek) (struct file *, loff_t, int);
	ssize_t (*read) (struct file *, char __user *, size_t, loff_t *);
	ssize_t (*write) (struct file *, const char __user *, size_t, loff_t *);
	ssize_t (*read_iter) (struct kiocb *, struct iov_iter *);
	ssize_t (*write_iter) (struct kiocb *, struct iov_iter *);
	int (*iopoll) (struct kiocb *kiocb, bool spin);
	int (*iterate_shared) (struct file *, struct dir_context *);
	__poll_t (*poll) (struct file *, struct poll_table_struct *);
	long (*unlocked_ioctl) (struct file *, unsigned int, unsigned long);
	long (*compat_ioctl) (struct file *, unsigned int, unsigned long);
	int (*mmap) (struct file *, struct vm_area_struct *);
	int (*open) (struct inode *, struct file *);
	int (*flush) (struct file *);
	int (*release) (struct inode *, struct file *);
	int (*fsync) (struct file *, loff_t start, loff_t end, int datasync);
	int (*fasync) (int, struct file *, int);
	int (*lock) (struct file *, int, struct file_lock *);
	unsigned long (*get_unmapped_area)(struct file *, unsigned long,
			unsigned long, unsigned long, unsigned long);
	int (*check_flags)(int);
	int (*flock) (struct file *, int, struct file_lock *);
	ssize_t (*splice_write)(struct pipe_inode_info *, struct file *, loff_t *,
			size_t, unsigned int);
	ssize_t (*splice_read)(struct file *, loff_t *, struct pipe_inode_info *,
			size_t, unsigned int);
	int (*setlease)(struct file *, long, struct file_lock **, void **);
	long (*fallocate)(struct file *, int, loff_t, loff_t);
	void (*show_fdinfo)(struct seq_file *m, struct file *f);
	unsigned (*mmap_capabilities)(struct file *);
	ssize_t (*copy_file_range)(struct file *, loff_t, struct file *,
			loff_t, size_t, unsigned int);
	loff_t (*remap_file_range)(struct file *file_in, loff_t pos_in,
			struct file *file_out, loff_t pos_out,
			loff_t len, unsigned int remap_flags);
	int (*fadvise)(struct file *, loff_t, loff_t, int);

```
鍔犻攣瑙勫垯锛?
	鍏ㄩ儴閮藉彲鑳介樆濉炪€?

->llseek() 鐨勫姞閿佸凡浠?llseek 绉诲埌浜嗗悇涓?llseek 瀹炵幇涓€傚鏋滀綘鐨勬枃浠剁郴缁熸病鏈変娇鐢?generic_file_llseek锛屽垯闇€瑕佸湪浣犵殑 ->llseek() 涓幏鍙栧苟閲婃斁閫傚綋鐨勯攣銆傚浜庤澶氭枃浠剁郴缁熸潵璇达紝鑾峰彇 inode 浜掓枼浣撴垨骞茶剢鏀圭敤 i_size_read() 鍙兘鏄畨鍏ㄧ殑銆傛敞鎰忥細杩欏苟涓嶈兘淇濇姢 file->f_pos 鍏嶅彈骞跺彂淇敼锛屽洜涓鸿繖鏄敤鎴锋€侀渶瑕佽嚜琛屽鐞嗙殑浜嬫儏銆?

->iterate_shared() 鍦ㄦ寔鏈?i_rwsem锛堣锛変互鍙?file 鐨?f_pos_lock锛堢嫭鍗狅級鐨勬儏鍐典笅琚皟鐢ㄣ€?

->fasync() 璐熻矗缁存姢 filp->f_flags 涓殑 FASYNC 浣嶃€傚ぇ澶氭暟瀹炰緥璋冪敤 fasync_helper()锛岀敱瀹冨畬鎴愯缁存姢锛屾墍浠ヨ繖閫氬父涓嶆槸闇€瑕佹媴蹇冪殑浜嬨€傚ぇ浜?0 鐨勮繑鍥炲€间細鍦?VFS 灞傝鏄犲皠涓洪浂銆?

->readdir() 鍜岀洰褰曚笂鐨?->ioctl() 蹇呴』琚慨鏀广€傜悊鎯虫儏鍐典笅锛屾垜浠細鎶?->readdir() 绉诲埌 inode_operations锛屽苟涓虹洰褰?->ioctl() 浣跨敤涓€涓崟鐙殑鏂规硶锛屾垨鑰呭共鑴嗗畬鍏ㄥ幓鎺夊悗鑰呫€傞棶棰樹箣涓€鏄紝瀵逛簬浠讳綍绫讳技浜庤仈鍚堟寕杞斤紙union-mount锛夌殑鎯呭喌锛屾垜浠苟涓嶄細涓烘墍鏈夌粍浠堕兘鎸佹湁涓€涓?struct file銆傝€屼笖褰撳墠鎺ュ彛涔嬫墍浠ヤ竴鍥㈢碂杩樻湁鍏朵粬鍘熷洜鈥︹€?

->read 瀵圭洰褰曠殑璇诲彇寰堝彲鑳藉繀椤诲幓鎺夆€斺€旀垜浠簲褰撶洿鎺ュ湪 sys_read() 鍙婂叾鍚岀被涓己鍒惰繑鍥?-EISDIR銆?

->setlease 鎿嶄綔搴斿綋鍦ㄥ悇涓枃浠剁郴缁熶腑璁剧疆绉熺害涔嬪墠鎴栦箣鍚庤皟鐢?generic_setlease()锛屼互璁板綍鎿嶄綔鐨勭粨鏋溿€?

->fallocate 瀹炵幇蹇呴』闈炲父灏忓績锛屽湪鎵撴礊鎴栨墽琛屽叾浠栦娇椤电紦瀛樺唴瀹瑰け鏁堢殑鎿嶄綔鏃讹紝淇濇寔椤电紦瀛樼殑涓€鑷存€с€傞€氬父鏂囦欢绯荤粺闇€瑕佽皟鐢?truncate_inode_pages_range() 鏉ヤ娇椤电紦瀛樼殑鐩稿叧鑼冨洿澶辨晥銆傜劧鑰屾枃浠剁郴缁熼€氬父杩橀渶瑕佹洿鏂板叾鍐呴儴鐨勶紙浠ュ強纾佺洏涓婄殑锛夋枃浠跺亸绉?-> 纾佺洏鍧楁槧灏勮鍥俱€傚湪杩欎釜鏇存柊瀹屾垚涔嬪墠锛屾枃浠剁郴缁熼渶瑕侀樆姝㈤〉閿欒浠ュ強浠庣鐩橀噸鏂板姞杞界幇宸茶繃鏃剁殑椤电紦瀛樺唴瀹圭殑璇绘搷浣溿€傜敱浜?VFS 鍦ㄤ粠纾佺洏鍔犺浇椤垫椂锛坒ilemap_fault()銆乫ilemap_read()銆乺eadahead 璺緞锛変互鍏变韩妯″紡鑾峰彇 mapping->invalidate_lock锛宖allocate 瀹炵幇蹇呴』鑾峰彇 invalidate_lock 鏉ラ樆姝㈤噸鏂板姞杞姐€?

->copy_file_range 鍜?->remap_file_range 瀹炵幇闇€瑕佸湪鎿嶄綔杩愯鏈熼棿锛岄拡瀵规枃浠舵暟鎹殑淇敼杩涜涓茶鍖栥€傝闃绘閫氳繃 write(2) 鍙婄被浼兼搷浣滆繘琛岀殑淇敼锛屽彲浠ヤ娇鐢?inode->i_rwsem銆傝闃绘閫氳繃鍐呭瓨鏄犲皠鍦ㄦ搷浣滄湡闂翠慨鏀规枃浠跺唴瀹癸紝鏂囦欢绯荤粺蹇呴』鑾峰彇 mapping->invalidate_lock 鏉ヤ笌 ->page_mkwrite 鍗忚皟銆?

## dquot_operations


```
	int (*write_dquot) (struct dquot *);
	int (*acquire_dquot) (struct dquot *);
	int (*release_dquot) (struct dquot *);
	int (*mark_dirty) (struct dquot *);
	int (*write_info) (struct super_block *, int);

```
杩欎簺鎿嶄綔鏃ㄥ湪鎴愪负鎴栧鎴栧皯鍖呰鎬х殑鍑芥暟锛岀‘淇濇纭殑鍔犻攣锛堢浉瀵逛簬鏂囦欢绯荤粺锛夊苟璋冪敤閫氱敤鐨勯厤棰濇搷浣溿€?

鏂囦欢绯荤粺鍙互浠庨€氱敤閰嶉鍑芥暟涓湡寰呬粈涔堬細

==============	============	=========================
ops		FS recursion	Held locks when called
==============	============	=========================
write_dquot:	yes		dqonoff_sem or dqptr_sem
acquire_dquot:	yes		dqonoff_sem or dqptr_sem
release_dquot:	yes		dqonoff_sem or dqptr_sem
mark_dirty:	no		-
write_info:	yes		dqonoff_sem
==============	============	=========================

FS recursion 鎸囦粠瓒呯骇鍧楁搷浣滀腑璋冪敤 ->quota_read() 鍜?->quota_write()銆?

鏈夊叧閰嶉鍔犻攣鐨勬洿澶氱粏鑺傚彲浠ュ湪 fs/dquot.c 涓壘鍒般€?

## vm_operations_struct


```
	void (*open)(struct vm_area_struct *);
	void (*close)(struct vm_area_struct *);
	vm_fault_t (*fault)(struct vm_fault *);
	vm_fault_t (*huge_fault)(struct vm_fault *, unsigned int order);
	vm_fault_t (*map_pages)(struct vm_fault *, pgoff_t start, pgoff_t end);
	vm_fault_t (*page_mkwrite)(struct vm_area_struct *, struct vm_fault *);
	vm_fault_t (*pfn_mkwrite)(struct vm_area_struct *, struct vm_fault *);
	int (*access)(struct vm_area_struct *, unsigned long, void*, int, int);

```
鍔犻攣瑙勫垯锛?

=============	==========	===========================
ops		mmap_lock	PageLocked(page)
=============	==========	===========================
open:		write
close:		read/write
fault:		read		can return with page locked
huge_fault:	maybe-read
map_pages:	maybe-read
page_mkwrite:	read		can return with page locked
pfn_mkwrite:	read
access:		read
=============	==========	===========================

->fault() 鍦ㄥ嵆灏嗗涓€涓厛鍓嶄笉瀛樺湪鐨?pte 浜х敓缂洪〉鏃惰皟鐢ㄣ€傛枃浠剁郴缁熷繀椤绘壘鍒板苟杩斿洖涓庝紶鍏?vm_fault 缁撴瀯涓殑 "pgoff" 鍏宠仈鐨勯〉銆傚鏋滈〉鏈夊彲鑳借鎴柇鍜?鎴栧け鏁堬紝鍒欐枃浠剁郴缁熷繀椤婚攣瀹?invalidate_lock锛岀劧鍚庣‘淇濊椤靛皻鏈鎴柇锛坕nvalidate_lock 浼氶樆姝㈠悗缁殑鎴柇锛夛紝鐒跺悗浠?VM_FAULT_LOCKED 杩斿洖锛屽苟涓旇椤靛浜庨攣瀹氱姸鎬併€俈M 灏嗚В閿佽椤点€?

->huge_fault() 鍦ㄤ笉瀛樺湪 PUD 鎴?PMD 椤规椂琚皟鐢ㄣ€傝繖缁欐枃浠剁郴缁熸彁渚涗簡瀹夎涓€涓?PUD 鎴?PMD 澶у皬椤电殑鏈轰細銆傛枃浠剁郴缁熶篃鍙互浣跨敤 ->fault 鏂规硶杩斿洖 PMD 澶у皬鐨勯〉锛屽洜姝ゅ疄鐜版鍑芥暟鍙兘涓嶆槸蹇呴渶鐨勩€傜壒鍒湴锛屾枃浠剁郴缁熶笉搴斾粠 ->huge_fault() 涓皟鐢?filemap_fault()銆傝皟鐢ㄦ鏂规硶鏃跺彲鑳戒笉鎸佹湁 mmap_lock銆?

->map_pages() 鍦?VM 瑕佹眰鏄犲皠鏄撲簬璁块棶鐨勯〉鏃惰璋冪敤銆傛枃浠剁郴缁熷簲褰撴壘鍒板苟鏄犲皠涓庝粠 "start_pgoff" 鍒?"end_pgoff" 鍋忕Щ閲忓叧鑱旂殑椤点€?>map_pages() 鍦ㄦ寔鏈?RCU 閿佺殑鎯呭喌涓嬭皟鐢紝涓斾笉鑳介樆濉炪€傚鏋滄棤娉曞湪涓嶉樆濉炵殑鎯呭喌涓嬪埌杈炬煇涓〉锛屾枃浠剁郴缁熷簲褰撹烦杩囧畠銆傛枃浠剁郴缁熷簲褰撲娇鐢?set_pte_range() 鏉ヨ缃〉琛ㄩ」銆備笌椤靛叧鑱旂殑椤圭殑鎸囬拡閫氳繃 vm_fault 缁撴瀯涓殑 "pte" 瀛楁浼犲叆銆傚叾浠栧亸绉婚噺鐨勯」鎸囬拡搴斿綋鐩稿 "pte" 璁＄畻銆?

->page_mkwrite() 鍦ㄥ厛鍓嶅彧璇荤殑 pte 鍗冲皢鍙樹负鍙啓鏃惰璋冪敤銆傛枃浠剁郴缁熷悓鏍峰繀椤荤‘淇濅笉瀛樺湪 truncate/invalidate 绔炰簤锛屾垨涓庤濡?->remap_file_range 鎴?->copy_file_range 绛夋搷浣滀箣闂寸殑绔炰簤锛岀劧鍚庝互椤甸攣瀹氱姸鎬佽繑鍥炪€傞€氬父 mapping->invalidate_lock 閫傜敤浜庨€傚綋鐨勪覆琛屽寲銆傚鏋滆椤靛凡琚埅鏂紝鏂囦欢绯荤粺涓嶅簲鍍?->fault() 澶勭悊绋嬪簭閭ｆ牱鏌ユ壘鏂伴〉锛岃€屽彧闇€浠?VM_FAULT_NOPAGE 杩斿洖锛岃繖灏嗗鑷?VM 閲嶈瘯璇ョ己椤点€?

->pfn_mkwrite() 涓?page_mkwrite 鐩稿悓锛屼絾褰?pte 鏄?VM_PFNMAP 鎴?VM_MIXEDMAP 涓斾负鏃犻〉椤规椂銆傛湡鏈涜繑鍥?VM_FAULT_NOPAGE锛屾垨 VM_FAULT_ERROR 绫诲瀷涔嬩竴銆傛璋冪敤涔嬪悗鐨勯粯璁よ涓烘槸浣?pte 鍙樹负璇诲啓锛岄櫎闈?pfn_mkwrite 杩斿洖閿欒銆?

->access() 鍦?get_user_pages() 鍦?access_process_vm() 涓け璐ユ椂璋冪敤锛岄€氬父鐢ㄤ簬閫氳繃 /proc/pid/mem 鎴?ptrace 璋冭瘯涓€涓繘绋嬨€傛鍑芥暟浠呭 VM_IO | VM_PFNMAP 鐨?VMA 鏄繀闇€鐨勩€?

--------------------------------------------------------------------------------

			鍙枒 stuff

锛堝鏋滀綘寮勫潖浜嗕粈涔堬紝鎴栬€呮敞鎰忓埌瀹冨凡鎹熷潖鍗存病鏈夎嚜宸变慨澶嶁€斺€旇嚦灏戞妸瀹冩斁鍦ㄨ繖閲岋級
