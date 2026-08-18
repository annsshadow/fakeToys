## Changes since 2.5.0:

## 鑷?2.5.0 璧风殑鍙樻洿锛?


---


**recommended**

**寤鸿**

New helpers: sb_bread(), sb_getblk(), sb_find_get_block(), set_bh(),
sb_set_blocksize() and sb_min_blocksize().

鏂板杈呭姪鍑芥暟锛歴b_bread()銆乻b_getblk()銆乻b_find_get_block()銆乻et_bh()銆?
sb_set_blocksize() 鍜?sb_min_blocksize()銆?

Use them.

璇蜂娇鐢ㄥ畠浠€?

(sb_find_get_block() replaces 2.4's get_hash_table())

锛坰b_find_get_block() 鍙栦唬 2.4 鐨?get_hash_table()锛?

---


**recommended**

**寤鸿**

New methods: ->alloc_inode() and ->destroy_inode().

鏂板鏂规硶锛?>alloc_inode() 鍜?->destroy_inode()銆?

Remove inode->u.foo_inode_i

绉婚櫎 inode->u.foo_inode_i

```

	struct foo_inode_info {
		/* fs-private stuff */
		struct inode vfs_inode;
	};
	static inline struct foo_inode_info *FOO_I(struct inode *inode)
	{
		return list_entry(inode, struct foo_inode_info, vfs_inode);
	}

```

Use FOO_I(inode) instead of &inode->u.foo_inode_i;

浣跨敤 FOO_I(inode) 鍙栦唬 &inode->u.foo_inode_i锛?

Add foo_alloc_inode() and foo_destroy_inode() - the former should allocate
foo_inode_info and return the address of ->vfs_inode, the latter should free
FOO_I(inode) (see in-tree filesystems for examples).

鏂板 foo_alloc_inode() 鍜?foo_destroy_inode()鈥斺€斿墠鑰呭簲鍒嗛厤
foo_inode_info 骞惰繑鍥?->vfs_inode 鐨勫湴鍧€锛屽悗鑰呭簲閲婃斁
FOO_I(inode)锛堝弬瑙佹爲鍐呮枃浠剁郴缁熺殑绀轰緥锛夈€?

Make them ->alloc_inode and ->destroy_inode in your super_operations.

鍦ㄤ綘鐨?super_operations 涓皢瀹冧滑璁句负 ->alloc_inode 鍜?->destroy_inode銆?

Keep in mind that now you need explicit initialization of private data
typically between calling iget_locked() and unlocking the inode.

璇锋敞鎰忥紝鐜板湪浣犻渶瑕佸湪璋冪敤 iget_locked() 涓庤В閿?inode 涔嬮棿鏄惧紡鍦?
鍒濆鍖栫鏈夋暟鎹€?

At some point that will become mandatory.

杩欎竴鐐瑰湪鏌愪釜鏃跺€欏皢鍙樹负寮哄埗瑕佹眰銆?

**mandatory**

**寮哄埗**

The foo_inode_info should always be allocated through alloc_inode_sb() rather
than kmem_cache_alloc() or kmalloc() related to set up the inode reclaim context
correctly.

foo_inode_info 搴斿缁堥€氳繃 alloc_inode_sb() 鑰岄潪 kmem_cache_alloc() 鎴?
kmalloc() 鍒嗛厤锛屼互姝ｇ‘寤虹珛 inode 鍥炴敹涓婁笅鏂囥€?

---


**mandatory**

**寮哄埗**

Change of file_system_type method (->read_super to ->get_sb)

file_system_type 鏂规硶鐨勫彉鏇达紙->read_super 鏀逛负 ->get_sb锛?

->read_super() is no more.  Ditto for DECLARE_FSTYPE and DECLARE_FSTYPE_DEV.

->read_super() 宸蹭笉澶嶅瓨鍦ㄣ€侱ECLARE_FSTYPE 鍜?DECLARE_FSTYPE_DEV 鍚岀悊銆?

Turn your foo_read_super() into a function that would return 0 in case of
success and negative number in case of error (-EINVAL unless you have more

灏嗕綘鐨?foo_read_super() 鏀规垚涓€涓嚱鏁帮紝鎴愬姛鏃惰繑鍥?0锛屽嚭閿欐椂杩斿洖璐熸暟
锛堥櫎闈炰綘鏈夋洿鍏蜂綋鐨勯敊璇爜锛屽惁鍒欎负 -EINVAL锛夛細

```

  int foo_get_sb(struct file_system_type *fs_type,
	int flags, const char *dev_name, void *data, struct vfsmount *mnt)
  {
	return get_sb_bdev(fs_type, flags, dev_name, data, foo_fill_super,
			   mnt);
  }

```

(or similar with s/bdev/nodev/ or s/bdev/single/, depending on the kind of
filesystem).

锛堟垨鑰呰鏂囦欢绯荤粺绫诲瀷锛岀敤 s/bdev/nodev/ 鎴?s/bdev/single/ 鐨勭被浼煎啓娉曪級銆?

Replace DECLARE_FSTYPE... with explicit initializer and have ->get_sb set as
foo_get_sb.

灏?DECLARE_FSTYPE... 鏇挎崲涓烘樉寮忓垵濮嬪寲鍣紝骞跺皢 ->get_sb 璁句负
foo_get_sb銆?

---


**mandatory**

**寮哄埗**

Locking change: ->s_vfs_rename_sem is taken only by cross-directory renames.
Most likely there is no need to change anything, but if you relied on
global exclusion between renames for some internal purpose - you need to
change your internal locking.  Otherwise exclusion warranties remain the
same (i.e. parents and victim are locked, etc.).

鍔犻攣鍙樻洿锛?>s_vfs_rename_sem 浠呯敱璺ㄧ洰褰曢噸鍛藉悕鑾峰彇銆傚緢鍙兘浣犱笉闇€瑕?
鍋氫换浣曟敼鍔紝浣嗗鏋滀綘涓轰簡鏌愪簺鍐呴儴鐩殑渚濊禆閲嶅懡鍚嶄箣闂寸殑鍏ㄥ眬浜掓枼鈥斺€斾綘
闇€瑕佷慨鏀逛綘鐨勫唴閮ㄥ姞閿併€傚惁鍒欐帓瀹冧繚璇佷繚鎸佷笉鍙橈紙鍗崇埗鐩綍鍜屽彈瀹宠€呰
閿佸畾绛夛級銆?

---


**informational**

**璇存槑**

Now we have the exclusion between ->lookup() and directory removal (by
->rmdir() and ->rename()).  If you used to need that exclusion and do
it by internal locking (most of filesystems couldn't care less) - you
can relax your locking.

鐜板湪鎴戜滑鏈変簡 ->lookup() 涓庣洰褰曞垹闄わ紙閫氳繃 ->rmdir() 鍜?->rename()锛?
涔嬮棿鐨勪簰鏂ャ€傚鏋滀綘鏇剧粡闇€瑕佽浜掓枼骞堕€氳繃鍐呴儴鍔犻攣鏉ュ疄鐜帮紙澶у鏁版枃浠剁郴缁?
鏍规湰涓嶅叧蹇冿級鈥斺€斾綘鍙互鏀惧浣犵殑鍔犻攣銆?

---


**mandatory**

**寮哄埗**

->lookup(), ->truncate(), ->create(), ->unlink(), ->mknod(), ->mkdir(),
->rmdir(), ->link(), ->lseek(), ->symlink(), ->rename()
and ->readdir() are called without BKL now.  Grab it on entry, drop upon return
- that will guarantee the same locking you used to have.  If your method or its
parts do not need BKL - better yet, now you can shift lock_kernel() and
unlock_kernel() so that they would protect exactly what needs to be
protected.

->lookup()銆?>truncate()銆?>create()銆?>unlink()銆?>mknod()銆?>mkdir()銆?
->rmdir()銆?>link()銆?>lseek()銆?>symlink()銆?>rename()
鍜?->readdir() 鐜板湪涓嶅啀鎸佹湁 BKL 鏃惰璋冪敤銆傚湪鍏ュ彛澶勮幏鍙栧畠锛屽湪杩斿洖鏃堕噴鏀?
鈥斺€旇繖灏嗕繚璇佷笌浣犱互寰€鎷ユ湁鐨勭浉鍚屽姞閿併€傚鏋滀綘鐨勬柟娉曟垨鍏堕儴鍒嗕笉闇€瑕?BKL鈥斺€?
閭ｅ氨鏇村ソ浜嗭紝鐜板湪浣犲彲浠ョЩ鍔?lock_kernel() 鍜?unlock_kernel()锛屼娇瀹冧滑
鎭板ソ淇濇姢闇€瑕佷繚鎶ょ殑鍐呭銆?

---


**mandatory**

**寮哄埗**

BKL is also moved from around sb operations. BKL should have been shifted into
individual fs sb_op functions.  If you don't need it, remove it.

BKL 涔熷凡浠?sb 鎿嶄綔鍛ㄥ洿绉昏蛋銆侭KL 鏈簲琚Щ鍏ュ悇涓枃浠剁郴缁熻嚜宸辩殑 sb_op
鍑芥暟銆傚鏋滀綘涓嶉渶瑕佸畠锛屽氨绉婚櫎瀹冦€?

---


**informational**

**璇存槑**

check for ->link() target not being a directory is done by callers.  Feel
free to drop it...

瀵?->link() 鐩爣涓嶆槸鐩綍鐨勬鏌ュ凡鐢辫皟鐢ㄦ柟瀹屾垚銆傚彲浠ユ斁蹇冨湴鍘绘帀瀹冣€︹€?

---


**informational**

**璇存槑**

->link() callers hold ->i_mutex on the object we are linking to.  Some of your
problems might be over...

->link() 鐨勮皟鐢ㄦ柟瀵规垜浠墍閾炬帴鍒扮殑瀵硅薄鎸佹湁 ->i_mutex銆備綘鐨勪竴浜涢棶棰樺彲鑳?
灏辨瑙ｅ喅浜嗏€︹€?

---


**mandatory**

**寮哄埗**

new file_system_type method - kill_sb(superblock).  If you are converting

鏂扮殑 file_system_type 鏂规硶鈥斺€攌ill_sb(superblock)銆傚鏋滀綘姝ｅ湪杞崲

```

	FS_REQUIRES_DEV		-	kill_block_super
	FS_LITTER		-	kill_litter_super
	neither			-	kill_anon_super

```

FS_LITTER is gone - just remove it from fs_flags.

FS_LITTER 宸蹭笉瀛樺湪鈥斺€斿彧闇€浠?fs_flags 涓Щ闄ゅ畠銆?

---


**mandatory**

**寮哄埗**

FS_SINGLE is gone (actually, that had happened back when ->get_sb()
went in - and hadn't been documented ;-/).  Just remove it from fs_flags
(and see ->get_sb() entry for other actions).

FS_SINGLE 宸蹭笉瀛樺湪锛堝疄闄呬笂锛屽畠鍦?->get_sb() 寮曞叆鏃跺氨宸茬粡娑堝け鈥斺€斿彧鏄?
娌℃湁琚枃妗ｈ褰?;-/锛夈€傚彧闇€浠?fs_flags 涓Щ闄ゅ畠锛堝苟鍙傝 ->get_sb() 鏉＄洰
浜嗚В鍏朵粬鎿嶄綔锛夈€?

---


**mandatory**

**寮哄埗**

->setattr() is called without BKL now.  Caller _always_ holds ->i_mutex, so
watch for ->i_mutex-grabbing code that might be used by your ->setattr().
Callers of notify_change() need ->i_mutex now.

->setattr() 鐜板湪涓嶅啀鎸佹湁 BKL 鏃惰璋冪敤銆傝皟鐢ㄦ柟_濮嬬粓_鎸佹湁 ->i_mutex锛屽洜姝?
瑕佹敞鎰忎綘鍙兘琚?->setattr() 浣跨敤鐨勮幏鍙?->i_mutex 鐨勪唬鐮併€俷otify_change()
鐨勮皟鐢ㄦ柟鐜板湪闇€瑕?->i_mutex銆?

---


**recommended**

**寤鸿**

New super_block field `struct export_operations *s_export_op` for
explicit support for exporting, e.g. via NFS.  The structure is fully
documented at its declaration in include/linux/fs.h, and in
Documentation/filesystems/nfs/exporting.rst.

鏂扮殑 super_block 瀛楁 `struct export_operations *s_export_op` 鐢ㄤ簬
鏄惧紡鏀寔瀵煎嚭锛屼緥濡傞€氳繃 NFS銆傝缁撴瀯鍦ㄥ叾 include/linux/fs.h 涓殑澹版槑澶勶紝
浠ュ強 Documentation/filesystems/nfs/exporting.rst 涓湁瀹屾暣鏂囨。銆?

Briefly it allows for the definition of decode_fh and encode_fh operations
to encode and decode filehandles, and allows the filesystem to use
a standard helper function for decode_fh, and provide file-system specific
support for this helper, particularly get_parent.

绠€鑰岃█涔嬶紝瀹冨厑璁稿畾涔?decode_fh 鍜?encode_fh 鎿嶄綔鏉ョ紪鐮佸拰瑙ｇ爜鏂囦欢鍙ユ焺
锛坒ilehandle锛夛紝骞跺厑璁告枃浠剁郴缁熶负 decode_fh 浣跨敤涓€涓爣鍑嗚緟鍔╁嚱鏁帮紝浠ュ強
涓鸿杈呭姪鍑芥暟鎻愪緵鏂囦欢绯荤粺鐗瑰畾鐨勬敮鎸侊紝灏ゅ叾鏄?get_parent銆?

It is planned that this will be required for exporting once the code
settles down a bit.

璁″垝鏄紝寰呬唬鐮佺◢寰ǔ瀹氬悗锛岃繖灏嗘垚涓哄鍑烘墍蹇呴』鐨勩€?

**mandatory**

**寮哄埗**

s_export_op is now required for exporting a filesystem.
isofs, ext2, ext3, fat
can be used as examples of very different filesystems.

瀵煎嚭鏂囦欢绯荤粺鐜板湪瑕佹眰鎻愪緵 s_export_op銆?
isofs銆乪xt2銆乪xt3銆乫at
鍙綔涓哄樊寮傚緢澶х殑鏂囦欢绯荤粺鐨勭ず渚嬨€?

---


**mandatory**

**寮哄埗**

iget4() and the read_inode2 callback have been superseded by iget5_locked()

iget4() 鍜?read_inode2 鍥炶皟宸茶 iget5_locked() 鍙栦唬

```

    struct inode *iget5_locked(struct super_block *sb, unsigned long ino,
				int (*test)(struct inode *, void *),
				int (*set)(struct inode *, void *),
				void *data);

```

'test' is an additional function that can be used when the inode
number is not sufficient to identify the actual file object. 'set'
should be a non-blocking function that initializes those parts of a
newly created inode to allow the test function to succeed. 'data' is
passed as an opaque value to both test and set functions.

'test' 鏄竴涓檮鍔犲嚱鏁帮紝褰?inode 鍙蜂笉瓒充互鏍囪瘑瀹為檯鏂囦欢瀵硅薄鏃跺彲浣跨敤瀹冦€?
'set' 搴旀槸涓€涓潪闃诲鍑芥暟锛岃礋璐ｅ垵濮嬪寲鏂板垱寤?inode 鐨勯偅浜涢儴鍒嗭紝浠ヤ娇
test 鍑芥暟鑳藉鎴愬姛銆?data' 浣滀负涓€涓笉閫忔槑鍊间紶閫掔粰 test 鍜?set 涓や釜鍑芥暟銆?

When the inode has been created by iget5_locked(), it will be returned with the
I_NEW flag set and will still be locked.  The filesystem then needs to finalize
the initialization. Once the inode is initialized it must be unlocked by
calling unlock_new_inode().

褰?inode 鐢?iget5_locked() 鍒涘缓鏃讹紝瀹冧細琚繑鍥炰笖甯︽湁 I_NEW 鏍囧織锛屽苟浠嶈
閿佸畾銆傜劧鍚庢枃浠剁郴缁熼渶瑕佸畬鎴愬垵濮嬪寲銆備竴鏃?inode 鍒濆鍖栧畬鎴愶紝蹇呴』閫氳繃璋冪敤
unlock_new_inode() 瑙ｉ攣銆?

The filesystem is responsible for setting (and possibly testing) i_ino
when appropriate. There is also a simpler iget_locked function that
just takes the superblock and inode number as arguments and does the
test and set for you.

鏂囦欢绯荤粺璐熻矗鍦ㄩ€傚綋鐨勬椂鍊欒缃紙骞跺彲鑳芥祴璇曪級i_ino銆傝繕鏈変竴涓洿绠€鍗曠殑
iget_locked 鍑芥暟锛屽畠鍙帴鍙?super_block 鍜?inode 鍙蜂綔涓哄弬鏁帮紝骞朵负浣?
瀹屾垚 test 鍜?set銆?

```

	inode = iget_locked(sb, ino);
	if (inode_state_read_once(inode) & I_NEW) {
		err = read_inode_from_disk(inode);
		if (err < 0) {
			iget_failed(inode);
			return err;
		}
		unlock_new_inode(inode);
	}

```

Note that if the process of setting up a new inode fails, then iget_failed()
should be called on the inode to render it dead, and an appropriate error
should be passed back to the caller.

娉ㄦ剰锛屽鏋滃缓绔嬫柊 inode 鐨勮繃绋嬪け璐ワ紝鍒欏簲鍦ㄨ inode 涓婅皟鐢?iget_failed()
浣垮叾澶辨晥锛屽苟鍚戣皟鐢ㄦ柟杩斿洖閫傚綋鐨勯敊璇€?

---


**recommended**

**寤鸿**

->getattr() finally getting used.  See instances in nfs, minix, etc.

->getattr() 缁堜簬琚敤涓婁簡銆傚弬瑙?nfs銆乵inix 绛夌殑瀹炰緥銆?

---


**mandatory**

**寮哄埗**

->revalidate() is gone.  If your filesystem had it - provide ->getattr()
and let it call whatever you had as ->revlidate() + (for symlinks that
had ->revalidate()) add calls in ->follow_link()/->readlink().

->revalidate() 宸蹭笉瀛樺湪銆傚鏋滀綘鐨勬枃浠剁郴缁熸浘鏈夊畠鈥斺€旇鎻愪緵 ->getattr()锛?
骞惰瀹冭皟鐢ㄤ綘鍘熸潵鐨?->revlidate()锛屼笖锛堝浜庢浘鎷ユ湁 ->revalidate() 鐨?
绗﹀彿閾炬帴锛夊湪 ->follow_link()/->readlink() 涓坊鍔犺皟鐢ㄣ€?

---


**mandatory**

**寮哄埗**

->d_parent changes are not protected by BKL anymore.  Read access is safe
if at least one of the following is true:

->d_parent 鐨勫彉鏇翠笉鍐嶅彈 BKL 淇濇姢銆傚鏋滀互涓嬭嚦灏戜竴椤规垚绔嬶紝鍒欒鍙栬闂槸
瀹夊叏鐨勶細

 - filesystem has no cross-directory rename()
 - we know that parent had been locked (e.g. we are looking at
	  ->d_parent of ->lookup() argument).
 - we are called from ->rename().
 - the child's ->d_lock is held

 - 鏂囦欢绯荤粺娌℃湁璺ㄧ洰褰?rename()
 - 鎴戜滑鐭ラ亾鐖剁洰褰曞凡琚攣瀹氾紙渚嬪锛屾垜浠鍦ㄦ煡鐪?->lookup() 鍙傛暟鐨?
	  ->d_parent锛夈€?
 - 鎴戜滑姝ｄ粠 ->rename() 涓璋冪敤銆?
 - 瀛愰」鐨?->d_lock 琚寔鏈?

Audit your code and add locking if needed.  Notice that any place that is
not protected by the conditions above is risky even in the old tree - you
had been relying on BKL and that's prone to screwups.  Old tree had quite
a few holes of that kind - unprotected access to ->d_parent leading to
anything from oops to silent memory corruption.

瀹℃煡浣犵殑浠ｇ爜锛屽苟鍦ㄩ渶瑕佹椂娣诲姞鍔犻攣銆傝娉ㄦ剰锛屽嵆渚垮湪鏃т唬鐮佹爲涓紝浠讳綍涓?
鍙椾笂杩版潯浠朵繚鎶ょ殑鍦版柟涔熸槸鏈夐闄╃殑鈥斺€斾綘鏇句緷璧?BKL锛岃€岄偅寰堝鏄撳嚭閿欍€傛棫浠ｇ爜鏍?
鏈夌浉褰撳姝ょ被婕忔礊鈥斺€斿 ->d_parent 鐨勬棤淇濇姢璁块棶浼氬鑷翠粠 oops 鍒伴潤榛樺唴瀛?
鎹熷潖绛夊悇绉嶉棶棰樸€?

---


**mandatory**

**寮哄埗**

FS_NOMOUNT is gone.  If you use it - just set SB_NOUSER in flags
(see rootfs for one kind of solution and bdev/socket/pipe for another).

FS_NOMOUNT 宸蹭笉瀛樺湪銆傚鏋滀綘浣跨敤瀹冣€斺€斿彧闇€鍦?flags 涓缃?SB_NOUSER
锛堝弬瑙?rootfs 浣滀负涓€绫昏В鍐虫柟妗堬紝浠ュ強 bdev/socket/pipe 浣滀负鍙︿竴绫伙級銆?

---


**recommended**

**寤鸿**

Use bdev_read_only(bdev) instead of is_read_only(kdev).  The latter
is still alive, but only because of the mess in drivers/s390/block/dasd.c.
As soon as it gets fixed is_read_only() will die.

浣跨敤 bdev_read_only(bdev) 鍙栦唬 is_read_only(kdev)銆傚悗鑰呬粛鐒跺瓨鍦紝浣嗕粎浠?
鏄洜涓?drivers/s390/block/dasd.c 涓殑娣蜂贡銆備竴鏃﹀畠琚慨澶嶏紝is_read_only()
灏变細娑堝け銆?

---


**mandatory**

**寮哄埗**

->permission() is called without BKL now. Grab it on entry, drop upon
return - that will guarantee the same locking you used to have.  If
your method or its parts do not need BKL - better yet, now you can
shift lock_kernel() and unlock_kernel() so that they would protect
exactly what needs to be protected.

->permission() 鐜板湪涓嶅啀鎸佹湁 BKL 鏃惰璋冪敤銆傚湪鍏ュ彛澶勮幏鍙栧畠锛屽湪杩斿洖鏃堕噴鏀?
鈥斺€旇繖灏嗕繚璇佷笌浣犱互寰€鎷ユ湁鐨勭浉鍚屽姞閿併€傚鏋滀綘鐨勬柟娉曟垨鍏堕儴鍒嗕笉闇€瑕?BKL鈥斺€?
閭ｅ氨鏇村ソ浜嗭紝鐜板湪浣犲彲浠ョЩ鍔?lock_kernel() 鍜?unlock_kernel()锛屼娇瀹冧滑
鎭板ソ淇濇姢闇€瑕佷繚鎶ょ殑鍐呭銆?

---


**mandatory**

**寮哄埗**

->statfs() is now called without BKL held.  BKL should have been
shifted into individual fs sb_op functions where it's not clear that
it's safe to remove it.  If you don't need it, remove it.

->statfs() 鐜板湪涓嶅啀鎸佹湁 BKL 鏃惰璋冪敤銆侭KL 鏈簲琚Щ鍏ュ悇涓枃浠剁郴缁熻嚜宸辩殑
sb_op 鍑芥暟锛屽湪閭ｄ簺灏氫笉娓呮鑳藉惁瀹夊叏绉婚櫎瀹冪殑鍦版柟銆傚鏋滀綘涓嶉渶瑕佸畠锛屽氨绉婚櫎瀹冦€?

---


**mandatory**

**寮哄埗**

is_read_only() is gone; use bdev_read_only() instead.

is_read_only() 宸蹭笉瀛樺湪锛涜鏀圭敤 bdev_read_only()銆?

---


**mandatory**

**寮哄埗**

destroy_buffers() is gone; use invalidate_bdev().

destroy_buffers() 宸蹭笉瀛樺湪锛涜鏀圭敤 invalidate_bdev()銆?

---


**mandatory**

**寮哄埗**

fsync_dev() is gone; use fsync_bdev().  NOTE: lvm breakage is
deliberate; as soon as struct block_device * is propagated in a reasonable
way by that code fixing will become trivial; until then nothing can be
done.

fsync_dev() 宸蹭笉瀛樺湪锛涜鏀圭敤 fsync_bdev()銆傛敞鎰忥細lvm 鐨勭牬鍧忔槸
鏈夋剰涓轰箣锛涘彧瑕?struct block_device * 琚浠ｇ爜浠ュ悎鐞嗘柟寮忎紶鎾紝淇灏?
鍙樺緱杞昏€屾槗涓撅紱鍦ㄦ涔嬪墠鏃犺兘涓哄姏銆?

**mandatory**

**寮哄埗**

block truncation on error exit from ->write_begin, and ->direct_IO
moved from generic methods (block_write_begin, cont_write_begin,
nobh_write_begin, blockdev_direct_IO*) to callers.  Take a look at
ext2_write_failed and callers for an example.

鍦?->write_begin 鍑洪敊閫€鍑烘椂鐨勫潡鎴柇锛屼互鍙?->direct_IO 宸蹭粠閫氱敤鏂规硶
锛坆lock_write_begin銆乧ont_write_begin銆乶obh_write_begin銆?
blockdev_direct_IO*锛夌Щ鑷宠皟鐢ㄦ柟銆傚弬瑙?ext2_write_failed 鍙婂叾璋冪敤鏂逛綔涓?
绀轰緥銆?

**mandatory**

**寮哄埗**

->truncate is gone.  The whole truncate sequence needs to be
implemented in ->setattr, which is now mandatory for filesystems
implementing on-disk size changes.  Start with a copy of the old inode_setattr
and vmtruncate, and the reorder the vmtruncate + foofs_vmtruncate sequence to
be in order of zeroing blocks using block_truncate_page or similar helpers,
size update and on finally on-disk truncation which should not fail.
setattr_prepare (which used to be inode_change_ok) now includes the size checks
for ATTR_SIZE and must be called in the beginning of ->setattr unconditionally.

->truncate 宸蹭笉瀛樺湪銆傛暣涓埅鏂簭鍒楅渶瑕佸湪 ->setattr 涓疄鐜帮紝瀵逛簬瀹炵幇浜?
纾佺洏涓婂ぇ灏忓彉鏇寸殑鏂囦欢绯荤粺锛岃鏂规硶鐜板湪鏄己鍒剁殑銆備粠鏃х増 inode_setattr
鍜?vmtruncate 鐨勫壇鏈紑濮嬶紝骞跺皢 vmtruncate + foofs_vmtruncate 搴忓垪閲嶆柊鎺掑簭涓?
浣跨敤 block_truncate_page 鎴栫被浼艰緟鍔╁嚱鏁版竻闆跺潡銆佹洿鏂板ぇ灏忥紝鏈€鍚庤繘琛屼笉搴斿け璐ョ殑
纾佺洏鎴柇銆俿etattr_prepare锛堝師鍚?inode_change_ok锛夌幇鍦ㄥ寘鍚 ATTR_SIZE 鐨?
澶у皬妫€鏌ワ紝涓斿繀椤诲湪 ->setattr 鐨勫紑澶存棤鏉′欢璋冪敤銆?

**mandatory**

**寮哄埗**

->clear_inode() and ->delete_inode() are gone; ->evict_inode() should
be used instead.  It gets called whenever the inode is evicted, whether it has
remaining links or not.  Caller does **not** evict the pagecache or inode-associated
metadata buffers; the method has to use truncate_inode_pages_final() to get rid
of those. Caller makes sure async writeback cannot be running for the inode while
(or after) ->evict_inode() is called.

->clear_inode() 鍜?->delete_inode() 宸蹭笉瀛樺湪锛涘簲鏀圭敤 ->evict_inode()銆?
鍙 inode 琚€愬嚭锛堟棤璁哄畠鏄惁杩樻湁鍓╀綑閾炬帴锛夛紝瀹冨氨浼氳璋冪敤銆傝皟鐢ㄦ柟**涓嶄細**
閫愬嚭 pagecache 鎴?inode 鍏宠仈鐨勫厓鏁版嵁缂撳啿鍖猴紱璇ユ柟娉曞繀椤讳娇鐢?
truncate_inode_pages_final() 鏉ユ竻闄ゅ畠浠€傝皟鐢ㄦ柟纭繚鍦?->evict_inode()
琚皟鐢ㄦ椂锛堟垨涔嬪悗锛変笉浼氬璇?inode 杩愯寮傛鍥炲啓銆?

->drop_inode() returns int now; it's called on final iput() with
inode->i_lock held and it returns true if filesystems wants the inode to be
dropped.  As before, inode_generic_drop() is still the default and it's been
updated appropriately.  inode_just_drop() is also alive and it consists
simply of return 1.  Note that all actual eviction work is done by caller after
->drop_inode() returns.

->drop_inode() 鐜板湪杩斿洖 int锛涘畠鍦ㄦ渶缁?iput() 鏃惰璋冪敤锛屾寔鏈?
inode->i_lock锛屽鏋滄枃浠剁郴缁熷笇鏈涗涪寮冭 inode 鍒欒繑鍥?true銆傚拰浠ュ墠涓€鏍凤紝
inode_generic_drop() 浠嶆槸榛樿瀹炵幇锛屽苟涓斿凡琚€傚綋鏇存柊銆俰node_just_drop()
涔熶粛鐒舵湁鏁堬紝瀹冨彧鏄畝鍗曞湴 return 1銆傛敞鎰忥紝鎵€鏈夊疄闄呯殑閫愬嚭宸ヤ綔閮界敱璋冪敤鏂瑰湪
->drop_inode() 杩斿洖鍚庡畬鎴愩€?

As before, clear_inode() must be called exactly once on each call of
->evict_inode() (as it used to be for each call of ->delete_inode()).  Unlike
before, if you are using inode-associated metadata buffers (i.e.
mark_buffer_dirty_inode()), it's your responsibility to call
invalidate_inode_buffers() before clear_inode().

鍜屼互鍓嶄竴鏍凤紝clear_inode() 蹇呴』鍦ㄦ瘡娆?->evict_inode() 璋冪敤鏃舵伆濂借皟鐢ㄤ竴娆?
锛堟濡備互鍓嶅姣忔 ->delete_inode() 璋冪敤閭ｆ牱锛夈€備笌浠ュ墠涓嶅悓鐨勬槸锛屽鏋滀綘浣跨敤
inode 鍏宠仈鐨勫厓鏁版嵁缂撳啿鍖猴紙鍗?mark_buffer_dirty_inode()锛夛紝鍦?clear_inode()
涔嬪墠璋冪敤 invalidate_inode_buffers() 鏄綘鐨勮矗浠汇€?

NOTE: checking i_nlink in the beginning of ->write_inode() and bailing out
if it's zero is not **and** **never** **had** **been** enough.  Final unlink() and iput()
may happen while the inode is in the middle of ->write_inode(); e.g. if you blindly
free the on-disk inode, you may end up doing that while ->write_inode() is writing
to it.

娉ㄦ剰锛氬湪 ->write_inode() 寮€澶存鏌?i_nlink锛岃嫢涓洪浂灏遍€€鍑虹殑鍋氭硶**鐜板湪涓嶆槸**
涓?*浠庢潵閮戒笉鏇?*瓒冲銆傛渶缁堢殑 unlink() 鍜?iput() 鍙兘鍦ㄨ inode 姝ｅ浜?
->write_inode() 杩囩▼涓椂鍙戠敓锛涗緥濡傦紝濡傛灉浣犵洸鐩噴鏀剧鐩樹笂鐨?inode锛屼綘鍙兘浼?
鍦?->write_inode() 姝ｅ湪鍚戝畠鍐欏叆鏃惰繖涔堝仛銆?

---


**mandatory**

**寮哄埗**

.d_delete() now only advises the dcache as to whether or not to cache
unreferenced dentries, and is now only called when the dentry refcount goes to
0. Even on 0 refcount transition, it must be able to tolerate being called 0,
1, or more times (eg. constant, idempotent).

.d_delete() 鐜板湪鍙槸鍚?dcache 寤鸿鏄惁缂撳瓨鏈紩鐢ㄧ殑 dentries锛屽苟涓旂幇鍦?
浠呭湪 dentry 寮曠敤璁℃暟闄嶄负 0 鏃惰璋冪敤銆傚嵆浣垮湪寮曠敤璁℃暟闄嶄负 0 鐨勮浆鍙樻椂锛屽畠涔?
蹇呴』鑳藉瀹瑰繊琚皟鐢?0 娆°€? 娆℃垨澶氭锛堜緥濡傦紝甯搁噺銆佸箓绛夛級銆?

---


**mandatory**

**寮哄埗**

.d_compare() calling convention and locking rules are significantly
changed. Read updated documentation in Documentation/filesystems/vfs.rst (and
look at examples of other filesystems) for guidance.

.d_compare() 鐨勮皟鐢ㄧ害瀹氬拰鍔犻攣瑙勫垯宸插彂鐢熼噸澶у彉鍖栥€傝闃呰
Documentation/filesystems/vfs.rst 涓洿鏂板悗鐨勬枃妗ｏ紙骞跺弬鑰冨叾浠栨枃浠剁郴缁熺殑
绀轰緥锛変互鑾峰彇鎸囧銆?

---


**mandatory**

**寮哄埗**

.d_hash() calling convention and locking rules are significantly
changed. Read updated documentation in Documentation/filesystems/vfs.rst (and
look at examples of other filesystems) for guidance.

.d_hash() 鐨勮皟鐢ㄧ害瀹氬拰鍔犻攣瑙勫垯宸插彂鐢熼噸澶у彉鍖栥€傝闃呰
Documentation/filesystems/vfs.rst 涓洿鏂板悗鐨勬枃妗ｏ紙骞跺弬鑰冨叾浠栨枃浠剁郴缁熺殑
绀轰緥锛変互鑾峰彇鎸囧銆?

---


**mandatory**

**寮哄埗**

dcache_lock is gone, replaced by fine grained locks. See fs/dcache.c
for details of what locks to replace dcache_lock with in order to protect
particular things. Most of the time, a filesystem only needs ->d_lock, which
protects **all** the dcache state of a given dentry.

dcache_lock 宸蹭笉瀛樺湪锛岃缁嗙矑搴﹂攣鍙栦唬銆傚叧浜庡簲鍒嗗埆鐢ㄥ摢浜涢攣鏉ュ彇浠?dcache_lock
浠ヤ繚鎶ょ壒瀹氬唴瀹癸紝璇﹁ fs/dcache.c銆傚ぇ澶氭暟鎯呭喌涓嬶紝鏂囦欢绯荤粺鍙渶瑕?->d_lock锛?
瀹冧繚鎶ょ粰瀹?dentry 鐨?*鎵€鏈?* dcache 鐘舵€併€?

---


**mandatory**

**寮哄埗**

Filesystems must RCU-free their inodes, if they can have been accessed
via rcu-walk path walk (basically, if the file can have had a path name in the
vfs namespace).

濡傛灉鏂囦欢绯荤粺鍙兘閫氳繃 rcu-walk 璺緞閬嶅巻琚闂紙鍩烘湰涓婏紝鍗虫枃浠跺彲鑳藉湪 vfs
鍛藉悕绌洪棿涓嫢鏈夎繃璺緞鍚嶏級锛屽垯蹇呴』浠?RCU 鏂瑰紡閲婃斁鍏?inode銆?

Even though i_dentry and i_rcu share storage in a union, we will
initialize the former in inode_init_always(), so just leave it alone in
the callback.  It used to be necessary to clean it there, but not anymore
(starting at 3.2).

灏界 i_dentry 鍜?i_rcu 鍦ㄤ竴涓仈鍚堜腑鍏变韩瀛樺偍锛屾垜浠皢鍦?inode_init_always()
涓垵濮嬪寲鍓嶈€咃紝鍥犳鍦ㄥ洖璋冧腑鏀剧潃涓嶇鍗冲彲銆傝繃鍘婚渶瑕佸湪閭ｉ噷娓呯悊瀹冿紝浣嗙幇鍦?
涓嶅啀闇€瑕佷簡锛堣嚜 3.2 璧凤級銆?

---


**recommended**

**寤鸿**

vfs now tries to do path walking in "rcu-walk mode", which avoids
atomic operations and scalability hazards on dentries and inodes (see
Documentation/filesystems/path-lookup.txt). d_hash and d_compare changes
(above) are examples of the changes required to support this. For more complex
filesystem callbacks, the vfs drops out of rcu-walk mode before the fs call, so
no changes are required to the filesystem. However, this is costly and loses
the benefits of rcu-walk mode. We will begin to add filesystem callbacks that
are rcu-walk aware, shown below. Filesystems should take advantage of this
where possible.

vfs 鐜板湪灏濊瘯浠?rcu-walk 妯″紡"杩涜璺緞閬嶅巻锛岃繖閬垮厤浜?dentries 鍜?inodes 涓婄殑
鍘熷瓙鎿嶄綔鍙婂彲浼哥缉鎬ч殣鎮ｏ紙瑙?Documentation/filesystems/path-lookup.txt锛夈€備笂鏂?
鐨?d_hash 鍜?d_compare 鍙樻洿灏辨槸涓烘敮鎸佽繖涓€鐐规墍闇€鏀瑰姩鐨勭ず渚嬨€傚浜庢洿澶嶆潅鐨勬枃浠剁郴缁?
鍥炶皟锛寁fs 浼氬湪璋冪敤鏂囦欢绯荤粺涔嬪墠閫€鍑?rcu-walk 妯″紡锛屽洜姝ゆ枃浠剁郴缁熸棤闇€鏀瑰姩銆傜劧鑰岋紝
杩欎唬浠烽珮鏄傚苟浼氫抚澶?rcu-walk 妯″紡鐨勪紭鍔裤€傛垜浠皢寮€濮嬫坊鍔犳劅鐭?rcu-walk 鐨勬枃浠剁郴缁?
鍥炶皟锛屽涓嬫墍绀恒€傛枃浠剁郴缁熷簲鍦ㄥ彲鑳芥椂鍔犱互鍒╃敤銆?

---


**mandatory**

**寮哄埗**

d_revalidate is a callback that is made on every path element (if
the filesystem provides it), which requires dropping out of rcu-walk mode. This
may now be called in rcu-walk mode (nd->flags & LOOKUP_RCU). -ECHILD should be
returned if the filesystem cannot handle rcu-walk. See
Documentation/filesystems/vfs.rst for more details.

d_revalidate 鏄竴涓姣忎釜璺緞鍏冪礌杩涜鐨勫洖璋冿紙濡傛灉鏂囦欢绯荤粺鎻愪緵瀹冿級锛屽畠瑕佹眰
閫€鍑?rcu-walk 妯″紡銆傜幇鍦ㄥ畠鍙兘鍦?rcu-walk 妯″紡锛坣d->flags & LOOKUP_RCU锛変笅琚?
璋冪敤銆傚鏋滄枃浠剁郴缁熸棤娉曞鐞?rcu-walk锛屽簲杩斿洖 -ECHILD銆傝瑙?
Documentation/filesystems/vfs.rst銆?

permission is an inode permission check that is called on many or all
directory inodes on the way down a path walk (to check for exec permission). It
must now be rcu-walk aware (mask & MAY_NOT_BLOCK).  See
Documentation/filesystems/vfs.rst for more details.

permission 鏄竴涓?inode 鏉冮檺妫€鏌ワ紝鍦ㄨ矾寰勯亶鍘嗗悜涓嬭繃绋嬩腑瀵硅澶氭垨鍏ㄩ儴鐩綍 inode
璋冪敤锛堢敤浜庢鏌ユ墽琛屾潈闄愶級銆傚畠鐜板湪蹇呴』鎰熺煡 rcu-walk锛坢ask & MAY_NOT_BLOCK锛夈€?
璇﹁ Documentation/filesystems/vfs.rst銆?

---


**mandatory**

**寮哄埗**

In ->fallocate() you must check the mode option passed in.  If your
filesystem does not support hole punching (deallocating space in the middle of a
file) you must return -EOPNOTSUPP if FALLOC_FL_PUNCH_HOLE is set in mode.
Currently you can only have FALLOC_FL_PUNCH_HOLE with FALLOC_FL_KEEP_SIZE set,
so the i_size should not change when hole punching, even when puching the end of
a file off.

鍦?->fallocate() 涓紝浣犲繀椤绘鏌ヤ紶鍏ョ殑 mode 閫夐」銆傚鏋滀綘鐨勬枃浠剁郴缁熶笉鏀寔绌垮瓟
锛堝湪鏂囦欢涓棿閲婃斁绌洪棿锛夛紝鍒欏綋 mode 涓缃簡 FALLOC_FL_PUNCH_HOLE 鏃跺繀椤昏繑鍥?
-EOPNOTSUPP銆傜洰鍓嶄綘鍙兘鍦ㄥ悓鏃惰缃簡 FALLOC_FL_KEEP_SIZE 鏃朵娇鐢?
FALLOC_FL_PUNCH_HOLE锛屽洜姝ょ┛瀛旀椂 i_size 涓嶅簲鏀瑰彉锛屽嵆浣跨┛瀛旂殑鏄枃浠跺熬閮ㄣ€?

---


**mandatory**

**寮哄埗**

->get_sb() and ->mount() are gone. Switch to using the new mount API. See
Documentation/filesystems/mount_api.rst for more details.

->get_sb() 鍜?->mount() 宸蹭笉瀛樺湪銆傝鍒囨崲鍒颁娇鐢ㄦ柊鐨勬寕杞?API銆傝瑙?
Documentation/filesystems/mount_api.rst銆?

---


**mandatory**

**寮哄埗**

->permission() and generic_permission()have lost flags
argument; instead of passing IPERM_FLAG_RCU we add MAY_NOT_BLOCK into mask.

->permission() 鍜?generic_permission() 宸蹭笉鍐嶆湁 flags 鍙傛暟锛涙垜浠笉鍐嶄紶鍏?
IPERM_FLAG_RCU锛岃€屾槸灏?MAY_NOT_BLOCK 鍔犲叆 mask銆?

generic_permission() has also lost the check_acl argument; ACL checking
has been taken to VFS and filesystems need to provide a non-NULL
->i_op->get_inode_acl to read an ACL from disk.

generic_permission() 涔熶笉鍐嶅叿鏈?check_acl 鍙傛暟锛汚CL 妫€鏌ュ凡琚撼鍏?VFS锛?
鏂囦欢绯荤粺闇€瑕佹彁渚涗竴涓潪 NULL 鐨?->i_op->get_inode_acl 鏉ヤ粠纾佺洏璇诲彇 ACL銆?

---


**mandatory**

**寮哄埗**

If you implement your own ->llseek() you must handle SEEK_HOLE and
SEEK_DATA.  You can handle this by returning -EINVAL, but it would be nicer to
support it in some way.  The generic handler assumes that the entire file is
data and there is a virtual hole at the end of the file.  So if the provided
offset is less than i_size and SEEK_DATA is specified, return the same offset.
If the above is true for the offset and you are given SEEK_HOLE, return the end
of the file.  If the offset is i_size or greater return -ENXIO in either case.

濡傛灉浣犲疄鐜颁簡鑷繁鐨?->llseek()锛屼綘蹇呴』澶勭悊 SEEK_HOLE 鍜?SEEK_DATA銆備綘鍙互閫氳繃
杩斿洖 -EINVAL 鏉ュ鐞嗭紝浣嗘渶濂戒互鏌愮鏂瑰紡鏀寔瀹冦€傞€氱敤澶勭悊鍑芥暟鍋囧畾鏁翠釜鏂囦欢閮芥槸
鏁版嵁锛屽苟涓旀枃浠舵湯灏炬湁涓€涓櫄鎷熺殑绌烘礊銆傚洜姝わ紝濡傛灉鎻愪緵鐨勫亸绉婚噺灏忎簬 i_size 涓?
鎸囧畾浜?SEEK_DATA锛屽垯杩斿洖鐩稿悓鐨勫亸绉婚噺銆傚鏋滀笂杩板鍋忕Щ閲忔垚绔嬩笖浣犳敹鍒?SEEK_HOLE锛?
鍒欒繑鍥炴枃浠舵湯灏俱€傚鏋滃亸绉婚噺涓?i_size 鎴栨洿澶э紝鍒欎袱绉嶆儏鍐典笅閮借繑鍥?-ENXIO銆?

**mandatory**

**寮哄埗**

If you have your own ->fsync() you must make sure to call
filemap_write_and_wait_range() so that all dirty pages are synced out properly.
You must also keep in mind that ->fsync() is not called with i_mutex held
anymore, so if you require i_mutex locking you must make sure to take it and
release it yourself.

濡傛灉浣犳湁鑷繁瀹炵幇鐨?->fsync()锛屼綘蹇呴』纭繚璋冪敤 filemap_write_and_wait_range()锛?
浠ヤ究鎵€鏈夎剰椤佃姝ｇ‘鍚屾鍐欏嚭銆備綘杩樺繀椤昏浣忥紝->fsync() 涓嶅啀鍦ㄦ寔鏈?i_mutex 鏃?
琚皟鐢紝鍥犳濡傛灉浣犻渶瑕?i_mutex 鍔犻攣锛屼綘蹇呴』纭繚鑷繁鑾峰彇骞跺湪涔嬪悗閲婃斁瀹冦€?

---


**mandatory**

**寮哄埗**

d_alloc_root() is gone, along with a lot of bugs caused by code
misusing it.  Replacement: d_make_root(inode).  On success d_make_root(inode)
allocates and returns a new dentry instantiated with the passed in inode.
On failure NULL is returned and the passed in inode is dropped so the reference
to inode is consumed in all cases and failure handling need not do any cleanup
for the inode.  If d_make_root(inode) is passed a NULL inode it returns NULL

d_alloc_root() 宸蹭笉瀛樺湪锛岃繛鍚岀敱婊ョ敤瀹冪殑浠ｇ爜寮曡捣鐨勮澶?bug 涓€璧枫€傛浛浠ｈ€咃細
d_make_root(inode)銆傛垚鍔熸椂 d_make_root(inode) 鍒嗛厤骞惰繑鍥炰竴涓敤浼犲叆 inode 瀹炰緥鍖栫殑
鏂?dentry銆傚け璐ユ椂杩斿洖 NULL锛屽苟涓斾紶鍏ョ殑 inode 琚涪寮冿紝鍥犳鍦ㄦ墍鏈夋儏鍐典笅瀵?inode 鐨?
寮曠敤閮借娑堣垂锛岄敊璇鐞嗘棤闇€瀵?inode 鍋氫换浣曟竻鐞嗐€傚鏋?d_make_root(inode) 琚紶鍏ヤ竴涓?
NULL inode锛屽畠杩斿洖 NULL锛?

```

	inode = foofs_new_inode(....);
	s->s_root = d_make_root(inode);
	if (!s->s_root)
		/* Nothing needed for the inode cleanup */
		return -ENOMEM;
	...

```

---


**mandatory**

**寮哄埗**

The witch is dead!  Well, 2/3 of it, anyway.  ->d_revalidate() and
->lookup() do **not** take struct nameidata anymore; just the flags.

濂冲帆姝讳簡锛佸棷锛岃嚦灏戞浜?2/3銆?>d_revalidate() 鍜?->lookup() 涓嶅啀鎺ュ彈
struct nameidata锛涘彧鎺ュ彈 flags銆?

---


**mandatory**

**寮哄埗**

->create() doesn't take `struct nameidata *`; unlike the previous
two, it gets "is it an O_EXCL or equivalent?" boolean argument.  Note that
local filesystems can ignore this argument - they are guaranteed that the
object doesn't exist.  It's remote/distributed ones that might care...

->create() 涓嶅啀鎺ュ彈 `struct nameidata *`锛涗笌鍓嶄袱涓笉鍚岋紝瀹冭幏寰椾竴涓?瀹冩槸
O_EXCL 鎴栫瓑浠风殑鍚楋紵"甯冨皵鍙傛暟銆傛敞鎰忔湰鍦版枃浠剁郴缁熷彲浠ュ拷鐣ヨ繖涓弬鏁扳€斺€斿畠浠淇濊瘉
瀵硅薄涓嶅瓨鍦ㄣ€傜湡姝ｅ彲鑳藉湪鎰忕殑鏄繙绋?鍒嗗竷寮忔枃浠剁郴缁熲€︹€?

---


**mandatory**

**寮哄埗**

FS_REVAL_DOT is gone; if you used to have it, add ->d_weak_revalidate()
in your dentry operations instead.

FS_REVAL_DOT 宸蹭笉瀛樺湪锛涘鏋滀綘鏇剧粡浣跨敤瀹冿紝璇锋敼鍦ㄤ綘鐨?dentry 鎿嶄綔涓坊鍔?
->d_weak_revalidate()銆?

---


**mandatory**

**寮哄埗**

vfs_readdir() is gone; switch to iterate_dir() instead

vfs_readdir() 宸蹭笉瀛樺湪锛涜鏀圭敤 iterate_dir()

---


**mandatory**

**寮哄埗**

->readdir() is gone now; switch to ->iterate_shared()

->readdir() 鐜板湪宸蹭笉瀛樺湪锛涜鏀圭敤 ->iterate_shared()

**mandatory**

**寮哄埗**

vfs_follow_link has been removed.  Filesystems must use nd_set_link
from ->follow_link for normal symlinks, or nd_jump_link for magic
/proc/<pid> style links.

vfs_follow_link 宸茶绉婚櫎銆傛枃浠剁郴缁熷繀椤诲鏅€氱鍙烽摼鎺ヤ娇鐢ㄦ潵鑷?->follow_link 鐨?
nd_set_link锛屾垨瀵圭壒娈婄殑 /proc/<pid> 椋庢牸閾炬帴浣跨敤 nd_jump_link銆?

---


**mandatory**

**寮哄埗**

iget5_locked()/ilookup5()/ilookup5_nowait() test() callback used to be
called with both ->i_lock and inode_hash_lock held; the former is **not**
taken anymore, so verify that your callbacks do not rely on it (none
of the in-tree instances did).  inode_hash_lock is still held,
of course, so they are still serialized wrt removal from inode hash,
as well as wrt set() callback of iget5_locked().

iget5_locked()/ilookup5()/ilookup5_nowait() 鐨?test() 鍥炶皟杩囧幓鍦ㄥ悓鏃舵寔鏈?
->i_lock 鍜?inode_hash_lock 鏃惰璋冪敤锛涘墠鑰?*涓嶅啀**琚寔鏈夛紝鍥犳璇风‘璁や綘鐨勫洖璋?
涓嶄緷璧栦簬瀹冿紙鏍戝唴鎵€鏈夊疄渚嬮兘娌℃湁渚濊禆锛夈€傚綋鐒讹紝inode_hash_lock 浠嶈鎸佹湁锛屽洜姝ゅ畠浠?
鐩稿浜庝粠 inode 鍝堝笇涓Щ闄ゃ€佷互鍙婄浉瀵逛簬 iget5_locked() 鐨?set() 鍥炶皟浠嶇劧鏄覆琛岀殑銆?

---


**mandatory**

**寮哄埗**

d_materialise_unique() is gone; d_splice_alias() does everything you
need now.  Remember that they have opposite orders of arguments ;-/

d_materialise_unique() 宸蹭笉瀛樺湪锛沝_splice_alias() 鐜板湪瀹屾垚浜嗕綘闇€瑕佺殑涓€鍒囥€?
璁颁綇瀹冧滑鐨勫弬鏁伴『搴忔槸鐩稿弽鐨?;-/

---


**mandatory**

**寮哄埗**

f_dentry is gone; use f_path.dentry, or, better yet, see if you can avoid
it entirely.

f_dentry 宸蹭笉瀛樺湪锛涜浣跨敤 f_path.dentry锛屾垨鑰呮渶濂界湅鐪嬩綘鏄惁鑳藉畬鍏ㄩ伩鍏嶄娇鐢ㄥ畠銆?

---


**mandatory**

**寮哄埗**

never call ->read() and ->write() directly; use __vfs_{read,write} or
wrappers; instead of checking for ->write or ->read being NULL, look for
FMODE_CAN_{WRITE,READ} in file->f_mode.

鍒囧嬁鐩存帴璋冪敤 ->read() 鍜?->write()锛涜浣跨敤 __vfs_{read,write} 鎴栧寘瑁呭嚱鏁帮紱
涓嶈妫€鏌?->write 鎴?->read 鏄惁涓?NULL锛岃€屾槸鏌ョ湅 file->f_mode 涓殑
FMODE_CAN_{WRITE,READ}銆?

---


**mandatory**

**寮哄埗**

do _not_ use new_sync_{read,write} for ->read/->write; leave it NULL
instead.

璇峰嬁瀵?->read/->write 浣跨敤 new_sync_{read,write}锛涜€屾槸灏嗗叾淇濈暀涓?NULL銆?

---


**mandatory**

	->aio_read/->aio_write are gone.  Use ->read_iter/->write_iter.

	->aio_read/->aio_write 宸蹭笉瀛樺湪銆傝浣跨敤 ->read_iter/->write_iter銆?

---


**recommended**

**寤鸿**

for embedded ("fast") symlinks just set inode->i_link to wherever the
symlink body is and use simple_follow_link() as ->follow_link().

瀵逛簬鍐呭祵锛?fast"锛夌鍙烽摼鎺ワ紝鍙渶灏?inode->i_link 璁句负绗﹀彿閾炬帴姝ｆ枃鎵€鍦ㄥ锛屽苟
浣跨敤 simple_follow_link() 浣滀负 ->follow_link()銆?

---


**mandatory**

**寮哄埗**

calling conventions for ->follow_link() have changed.  Instead of returning
cookie and using nd_set_link() to store the body to traverse, we return
the body to traverse and store the cookie using explicit void ** argument.
nameidata isn't passed at all - nd_jump_link() doesn't need it and
nd_[gs]et_link() is gone.

->follow_link() 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉銆傛垜浠笉鍐嶈繑鍥?cookie 骞朵娇鐢?nd_set_link() 瀛樺偍
瑕侀亶鍘嗙殑姝ｆ枃锛岃€屾槸杩斿洖瑕侀亶鍘嗙殑姝ｆ枃锛屽苟浣跨敤鏄惧紡鐨?void ** 鍙傛暟瀛樺偍 cookie銆?
nameidata 鏍规湰涓嶅啀浼犲叆鈥斺€攏d_jump_link() 涓嶅啀闇€瑕佸畠锛宯d_[gs]et_link() 涔熷凡娑堝け銆?

---


**mandatory**

**寮哄埗**

calling conventions for ->put_link() have changed.  It gets inode instead of
dentry,  it does not get nameidata at all and it gets called only when cookie
is non-NULL.  Note that link body isn't available anymore, so if you need it,
store it as cookie.

->put_link() 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉銆傚畠鑾峰緱 inode 鑰岄潪 dentry锛屽畬鍏ㄤ笉鍐嶈幏寰?nameidata锛?
骞朵笖浠呭湪 cookie 闈?NULL 鏃惰璋冪敤銆傛敞鎰忛摼鎺ユ鏂囦笉鍐嶅彲鐢紝鍥犳濡傛灉浣犻渶瑕佸畠锛岃灏嗗叾
浣滀负 cookie 瀛樺偍銆?

---


**mandatory**

**寮哄埗**

any symlink that might use page_follow_link_light/page_put_link() must
have inode_nohighmem(inode) called before anything might start playing with
its pagecache.  No highmem pages should end up in the pagecache of such
symlinks.  That includes any preseeding that might be done during symlink
creation.  page_symlink() will honour the mapping gfp flags, so once
you've done inode_nohighmem() it's safe to use, but if you allocate and
insert the page manually, make sure to use the right gfp flags.

浠讳綍鍙兘浣跨敤 page_follow_link_light/page_put_link() 鐨勭鍙烽摼鎺ュ繀椤诲湪浠讳綍鍙兘
寮€濮嬫搷浣滃叾 pagecache 涔嬪墠璋冪敤 inode_nohighmem(inode)銆傛绫荤鍙烽摼鎺ョ殑 pagecache 涓?
涓嶅簲鍑虹幇楂樼鍐呭瓨锛坔ighmem锛夐〉銆傝繖鍖呮嫭鍦ㄧ鍙烽摼鎺ュ垱寤鸿繃绋嬩腑鍙兘鍋氱殑浠讳綍棰勭疆銆?
page_symlink() 浼氶伒寰槧灏勭殑 gfp 鏍囧織锛屽洜姝や竴鏃︿綘璋冪敤浜?inode_nohighmem() 灏卞彲浠?
瀹夊叏浣跨敤瀹冿紝浣嗗鏋滀綘鏄墜鍔ㄥ垎閰嶅苟鎻掑叆椤甸潰锛岃纭繚浣跨敤姝ｇ‘鐨?gfp 鏍囧織銆?

---


**mandatory**

**寮哄埗**

->follow_link() is replaced with ->get_link(); same API, except that

->follow_link() 琚?->get_link() 鍙栦唬锛汚PI 鐩稿悓锛岄櫎浜嗭細

 - ->get_link() gets inode as a separate argument
 - ->get_link() may be called in RCU mode - in that case NULL
	  dentry is passed

 - ->get_link() 棰濆鑾峰緱 inode 浣滀负鍙傛暟
 - ->get_link() 鍙兘鍦?RCU 妯″紡涓嬭璋冪敤鈥斺€旀鏃朵紶鍏?
	  鐨?dentry 涓?NULL

---


**mandatory**

**寮哄埗**

->get_link() gets struct delayed_call `*done` now, and should do
set_delayed_call() where it used to set `*cookie`.

->get_link() 鐜板湪鑾峰緱 struct delayed_call `*done`锛屽苟涓斿簲鍦ㄨ繃鍘昏缃?`*cookie`
鐨勫湴鏂规敼鍋?set_delayed_call()銆?

->put_link() is gone - just give the destructor to set_delayed_call()
in ->get_link().

->put_link() 宸蹭笉瀛樺湪鈥斺€斿彧闇€鍦?->get_link() 涓皢鏋愭瀯鍑芥暟浜ょ粰 set_delayed_call()銆?

---


**mandatory**

**寮哄埗**

->getxattr() and xattr_handler.get() get dentry and inode passed separately.
dentry might be yet to be attached to inode, so do _not_ use its ->d_inode
in the instances.  Rationale: !@#!@# security_d_instantiate() needs to be
called before we attach dentry to inode.

->getxattr() 鍜?xattr_handler.get() 鐨?dentry 鍜?inode 琚垎寮€浼犲叆銆俤entry 鍙兘
灏氭湭闄勫姞鍒?inode锛屽洜姝ゅ疄渚嬩腑**涓嶈**浣跨敤鍏?->d_inode銆傜悊鐢憋細!@#!@# 
security_d_instantiate() 闇€瑕佸湪鎴戜滑灏?dentry 闄勫姞鍒?inode 涔嬪墠琚皟鐢ㄣ€?

---


**mandatory**

**寮哄埗**

symlinks are no longer the only inodes that do **not** have i_bdev/i_cdev/
i_pipe/i_link union zeroed out at inode eviction.  As the result, you can't
assume that non-NULL value in ->i_nlink at ->destroy_inode() implies that
it's a symlink.  Checking ->i_mode is really needed now.  In-tree we had
to fix shmem_destroy_callback() that used to take that kind of shortcut;
watch out, since that shortcut is no longer valid.

绗﹀彿閾炬帴涓嶅啀鏄敮涓€鍦?inode 閫愬嚭鏃?*涓?*灏?i_bdev/i_cdev/i_pipe/i_link 鑱斿悎娓呴浂鐨?
inode銆傚洜姝わ紝浣犱笉鑳藉啀鍋囧畾 ->destroy_inode() 鏃?->i_nlink 涓殑闈?NULL 鍊兼剰鍛崇潃
瀹冩槸绗﹀彿閾炬帴銆傜幇鍦ㄧ‘瀹炴湁蹇呰妫€鏌?->i_mode銆傚湪鏍戝唴鎴戜滑涓嶅緱涓嶄慨澶嶆浘缁忚蛋杩欑
鎹峰緞鐨?shmem_destroy_callback()锛涜褰撳績锛屽洜涓鸿鎹峰緞宸蹭笉鍐嶆湁鏁堛€?

---


**mandatory**

**寮哄埗**

->i_mutex is replaced with ->i_rwsem now.  inode_lock() et.al. work as
they used to - they just take it exclusive.  However, ->lookup() may be
called with parent locked shared.  Its instances must not

->i_mutex 鐜板湪琚?->i_rwsem 鍙栦唬銆俰node_lock() 绛夌殑宸ヤ綔鏂瑰紡濡傚悓浠ュ線鈥斺€斿畠浠彧鏄?
浠ョ嫭鍗犳柟寮忚幏鍙栧畠銆備絾鏄紝->lookup() 鍙兘鍦ㄧ埗鐩綍琚叡浜攣瀹氭椂琚皟鐢ㄣ€傚叾瀹炵幇蹇呴』
涓嶏細

 - use d_instantiate) and d_rehash() separately - use d_add() or
	  d_splice_alias() instead.
 - use d_rehash() alone - call d_add(new_dentry, NULL) instead.
 - in the unlikely case when (read-only) access to filesystem
	  data structures needs exclusion for some reason, arrange it
	  yourself.  None of the in-tree filesystems needed that.
 - rely on ->d_parent and ->d_name not changing after dentry has
	  been fed to d_add() or d_splice_alias().  Again, none of the
	  in-tree instances relied upon that.

 - 鍗曠嫭浣跨敤 d_instantiate() 鍜?d_rehash()鈥斺€旇鏀圭敤 d_add() 鎴?
	  d_splice_alias()銆?
 - 鍗曠嫭浣跨敤 d_rehash()鈥斺€旇鏀硅皟鐢?d_add(new_dentry, NULL)銆?
 - 鍦ㄦ瀬灏戞暟鎯呭喌涓嬶紝濡傛灉锛堝彧璇伙級璁块棶鏂囦欢绯荤粺鏁版嵁缁撴瀯鍑轰簬鏌愮鍘熷洜闇€瑕佷簰鏂ワ紝
	  璇疯嚜琛屽畨鎺掋€傛爲鍐呮病鏈変换浣曟枃浠剁郴缁熼渶瑕侀偅鏍峰仛銆?
 - 渚濊禆 ->d_parent 鍜?->d_name 鍦?dentry 琚氦缁?d_add() 鎴?
	  d_splice_alias() 鍚庝笉鍐嶆敼鍙樸€傚悓鏍凤紝鏍戝唴娌℃湁浠讳綍瀹炰緥渚濊禆杩欎竴鐐广€?

We are guaranteed that lookups of the same name in the same directory
will not happen in parallel ("same" in the sense of your ->d_compare()).
Lookups on different names in the same directory can and do happen in
parallel now.

鎴戜滑淇濊瘉鍚屼竴鐩綍涓浉鍚屽悕绉扮殑鏌ユ壘涓嶄細骞惰鍙戠敓锛?鐩稿悓"鎸囦綘鐨?->d_compare()
鎰忎箟涓婄殑鐩稿悓锛夈€傚悓涓€鐩綍涓笉鍚屽悕绉扮殑鏌ユ壘鐜板湪鍙互骞朵笖纭疄浼氬苟琛屽彂鐢熴€?

---


**mandatory**

**寮哄埗**

->iterate_shared() is added.
Exclusion on struct file level is still provided (as well as that
between it and lseek on the same struct file), but if your directory
has been opened several times, you can get these called in parallel.
Exclusion between that method and all directory-modifying ones is
still provided, of course.

鏂板浜?->iterate_shared()銆傚湪 struct file 绾у埆涓婄殑浜掓枼浠嶇劧鎻愪緵锛堜互鍙婂畠涓庡悓涓€
struct file 涓婄殑 lseek 涔嬮棿鐨勪簰鏂ワ級锛屼絾濡傛灉浣犵殑鐩綍琚墦寮€浜嗗娆★紝浣犲彲鑳戒細
骞惰鍦版敹鍒拌繖浜涜皟鐢ㄣ€傚綋鐒讹紝璇ユ柟娉曚笌鎵€鏈夌洰褰曚慨鏀规柟娉曚箣闂寸殑浜掓枼浠嶇劧鎻愪緵銆?

If you have any per-inode or per-dentry in-core data structures modified
by ->iterate_shared(), you might need something to serialize the access
to them.  If you do dcache pre-seeding, you'll need to switch to
d_alloc_parallel() for that; look for in-tree examples.

濡傛灉浣犳湁浠讳綍浼氳 ->iterate_shared() 淇敼鐨勬瘡 inode 鎴栨瘡 dentry 鍐呭瓨鏁版嵁缁撴瀯锛?
浣犲彲鑳介渶瑕佹煇绉嶆満鍒舵潵涓茶鍖栧瀹冧滑鐨勮闂€傚鏋滀綘杩涜浜?dcache 棰勭疆锛屼綘灏嗛渶瑕?
涓烘鍒囨崲鍒?d_alloc_parallel()锛涜鏌ユ壘鏍戝唴绀轰緥銆?

---


**mandatory**

**寮哄埗**

->atomic_open() calls without O_CREAT may happen in parallel.

涓嶅甫 O_CREAT 鐨?->atomic_open() 璋冪敤鍙兘浼氬苟琛屽彂鐢熴€?

---


**mandatory**

**寮哄埗**

->setxattr() and xattr_handler.set() get dentry and inode passed separately.
The xattr_handler.set() gets passed the user namespace of the mount the inode
is seen from so filesystems can idmap the i_uid and i_gid accordingly.
dentry might be yet to be attached to inode, so do _not_ use its ->d_inode
in the instances.  Rationale: !@#!@# security_d_instantiate() needs to be
called before we attach dentry to inode and !@#!@##!@$!$#!@#$!@$!@$ smack
->d_instantiate() uses not just ->getxattr() but ->setxattr() as well.

->setxattr() 鍜?xattr_handler.set() 鐨?dentry 鍜?inode 琚垎寮€浼犲叆銆?
xattr_handler.set() 浼氭敹鍒拌 inode 鎵€灞炴寕杞界殑鐢ㄦ埛鍛藉悕绌洪棿锛屼互渚挎枃浠剁郴缁?
鐩稿簲鍦板 i_uid 鍜?i_gid 杩涜 id 鏄犲皠銆俤entry 鍙兘灏氭湭闄勫姞鍒?inode锛屽洜姝?
瀹炰緥涓?*涓嶈**浣跨敤鍏?->d_inode銆傜悊鐢憋細!@#!@# security_d_instantiate() 闇€瑕佸湪
鎴戜滑灏?dentry 闄勫姞鍒?inode 涔嬪墠琚皟鐢紝骞朵笖 !@#!@##!@$!$#!@#$!@$!@$ smack 鐨?
->d_instantiate() 涓嶄粎浣跨敤 ->getxattr()锛屼篃浣跨敤 ->setxattr()銆?

---


**mandatory**

**寮哄埗**

->d_compare() doesn't get parent as a separate argument anymore.  If you
used it for finding the struct super_block involved, dentry->d_sb will
work just as well; if it's something more complicated, use dentry->d_parent.
Just be careful not to assume that fetching it more than once will yield
the same value - in RCU mode it could change under you.

->d_compare() 涓嶅啀灏嗙埗鐩綍浣滀负鍗曠嫭鍙傛暟銆傚鏋滀綘鏇剧敤瀹冩潵瀵绘壘鐩稿叧鐨?struct
super_block锛宒entry->d_sb 鍚屾牱閫傜敤锛涘鏋滄槸鏇村鏉傜殑鎯呭喌锛岃浣跨敤 dentry->d_parent銆?
鍙槸瑕佸皬蹇冿紝涓嶈鍋囧畾澶氭鑾峰彇瀹冧細寰楀埌鐩稿悓鐨勫€尖€斺€斿湪 RCU 妯″紡涓嬪畠鍙兘鍦ㄤ綘涓嶇煡鎯呮椂
鏀瑰彉銆?

---


**mandatory**

**寮哄埗**

->rename() has an added flags argument.  Any flags not handled by the
filesystem should result in EINVAL being returned.

->rename() 鏂板浜嗕竴涓?flags 鍙傛暟銆傛枃浠剁郴缁熸湭澶勭悊鐨勪换浣曟爣蹇楅兘搴斿鑷磋繑鍥?EINVAL銆?

---


**recommended**

**寤鸿**

->readlink is optional for symlinks.  Don't set, unless filesystem needs
to fake something for readlink(2).

->readlink 瀵圭鍙烽摼鎺ユ槸鍙€夌殑銆傞櫎闈炴枃浠剁郴缁熼渶瑕佷负 readlink(2) 浼€犳煇浜涘唴瀹癸紝
鍚﹀垯涓嶈璁剧疆瀹冦€?

---


**mandatory**

**寮哄埗**

->getattr() is now passed a struct path rather than a vfsmount and
dentry separately, and it now has request_mask and query_flags arguments
to specify the fields and sync type requested by statx.  Filesystems not
supporting any statx-specific features may ignore the new arguments.

->getattr() 鐜板湪浼犲叆涓€涓?struct path锛岃€屼笉鏄崟鐙殑 vfsmount 鍜?dentry锛屽苟涓?
瀹冪幇鍦ㄦ湁 request_mask 鍜?query_flags 鍙傛暟鏉ユ寚瀹?statx 鎵€璇锋眰鐨勫瓧娈靛拰鍚屾绫诲瀷銆?
涓嶆敮鎸佷换浣?statx 鐗瑰畾鐗规€х殑鏂囦欢绯荤粺鍙互蹇界暐杩欎簺鏂板弬鏁般€?

---


**mandatory**

**寮哄埗**

->atomic_open() calling conventions have changed.  Gone is `int *opened`,
along with FILE_OPENED/FILE_CREATED.  In place of those we have
FMODE_OPENED/FMODE_CREATED, set in file->f_mode.  Additionally, return
value for 'called finish_no_open(), open it yourself' case has become
0, not 1.  Since finish_no_open() itself is returning 0 now, that part
does not need any changes in ->atomic_open() instances.

->atomic_open() 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉銆俙int *opened` 杩炲悓 FILE_OPENED/FILE_CREATED
涓€璧锋秷澶变簡銆傚彇鑰屼唬涔嬬殑鏄缃湪 file->f_mode 涓殑 FMODE_OPENED/FMODE_CREATED銆?
姝ゅ锛?璋冪敤浜?finish_no_open()锛岀敱浣犺嚜宸辨墦寮€"杩欑鎯呭喌鐨勮繑鍥炲€煎彉鎴愪簡 0 鑰岄潪 1銆?
鐢变簬 finish_no_open() 鏈韩鐜板湪杩斿洖 0锛岄偅閮ㄥ垎鍦?->atomic_open() 瀹炰緥涓笉闇€瑕佷换浣?
鏀瑰姩銆?

---


**mandatory**

**寮哄埗**

alloc_file() has become static now; two wrappers are to be used instead.
alloc_file_pseudo(inode, vfsmount, name, flags, ops) is for the cases
when dentry needs to be created; that's the majority of old alloc_file()
users.  Calling conventions: on success a reference to new struct file
is returned and callers reference to inode is subsumed by that.  On
failure, ERR_PTR() is returned and no caller's references are affected,
so the caller needs to drop the inode reference it held.
alloc_file_clone(file, flags, ops) does not affect any caller's references.
On success you get a new struct file sharing the mount/dentry with the
original, on failure - ERR_PTR().

alloc_file() 鐜板湪宸叉垚涓洪潤鎬佸嚱鏁帮紱搴旀敼鐢ㄤ袱涓寘瑁呭嚱鏁般€?
alloc_file_pseudo(inode, vfsmount, name, flags, ops) 鐢ㄤ簬闇€瑕佸垱寤?dentry 鐨勬儏鍐碉紱
杩欐槸澶у鏁版棫 alloc_file() 鐢ㄦ埛鐨勬儏褰€傝皟鐢ㄧ害瀹氾細鎴愬姛鏃惰繑鍥炰竴涓鏂?struct file
鐨勫紩鐢紝璋冪敤鏂瑰 inode 鐨勫紩鐢ㄨ鍏跺惛鏀躲€傚け璐ユ椂杩斿洖 ERR_PTR()锛屼笖涓嶅奖鍝嶈皟鐢ㄦ柟鐨勪换浣?
寮曠敤锛屽洜姝よ皟鐢ㄦ柟闇€瑕侀噴鏀惧畠鎸佹湁鐨?inode 寮曠敤銆?
alloc_file_clone(file, flags, ops) 涓嶅奖鍝嶈皟鐢ㄦ柟鐨勪换浣曞紩鐢ㄣ€傛垚鍔熸椂浣犺幏寰椾竴涓笌鍘熷
鍏变韩鎸傝浇/dentry 鐨勬柊 struct file锛屽け璐ユ椂鈥斺€旇繑鍥?ERR_PTR()銆?

---


**mandatory**

**寮哄埗**

->clone_file_range() and ->dedupe_file_range have been replaced with
->remap_file_range().  See Documentation/filesystems/vfs.rst for more
information.

->clone_file_range() 鍜?->dedupe_file_range 宸茶 ->remap_file_range() 鍙栦唬銆?
璇﹁ Documentation/filesystems/vfs.rst銆?

---


**recommended**

**寤鸿**

```

	if (IS_ERR(inode))
		return ERR_CAST(inode);
	return d_splice_alias(inode, dentry);

```

don't need to bother with the check - d_splice_alias() will do the
right thing when given ERR_PTR(...) as inode.  Moreover, passing NULL
inode to d_splice_alias() will also do the right thing (equivalent of
d_add(dentry, NULL); return NULL;), so that kind of special cases
also doesn't need a separate treatment.

鏃犻渶璐瑰績鍋氳妫€鏌モ€斺€斿綋浼犲叆鐨?inode 涓?ERR_PTR(...) 鏃讹紝d_splice_alias() 浼氬仛姝ｇ‘鐨?
浜嬨€傛澶栵紝鍚?d_splice_alias() 浼犲叆 NULL inode 涔熶細鍋氭纭殑浜嬶紙绛変环浜?
d_add(dentry, NULL); return NULL;锛夛紝鍥犳姝ょ被鐗规畩鎯呭喌涔熶笉闇€鍗曠嫭澶勭悊銆?

---


**strongly recommended**

**寮虹儓寤鸿**

take the RCU-delayed parts of ->destroy_inode() into a new method -
->free_inode().  If ->destroy_inode() becomes empty - all the better,
just get rid of it.  Synchronous work (e.g. the stuff that can't
be done from an RCU callback, or any WARN_ON() where we want the
stack trace) **might** be movable to ->evict_inode(); however,
that goes only for the things that are not needed to balance something
done by ->alloc_inode().  IOW, if it's cleaning up the stuff that
might have accumulated over the life of in-core inode, ->evict_inode()
might be a fit.

灏?->destroy_inode() 涓?RCU 寤惰繜鐨勯儴鍒嗘彁鍙栧埌涓€涓柊鏂规硶 ->free_inode() 涓€傚鏋?
->destroy_inode() 鍙樼┖鈥斺€旈偅灏辨洿濂戒簡锛岀洿鎺ュ幓鎺夊畠銆傚悓姝ュ伐浣滐紙渚嬪鏃犳硶浠?RCU 鍥炶皟涓?
瀹屾垚鐨勪簨鎯咃紝鎴栦换浣曟垜浠笇鏈涜幏寰楁爤鍥炴函鐨?WARN_ON()锛?*鍙兘**鍙互绉诲埌 ->evict_inode()锛?
涓嶈繃锛岃繖鍙€傜敤浜庨偅浜涗笉闇€瑕佺敤鏉ュ钩琛?->alloc_inode() 鎵€瀹屾垚涔嬩簨鐨勫唴瀹广€傛崲瑷€涔嬶紝濡傛灉
瀹冩槸鍦ㄦ竻鐞嗗彲鑳藉湪鍐呭瓨 inode 鐢熷懡鍛ㄦ湡鍐呯疮绉殑涓滆タ锛?>evict_inode() 鍙兘鍚堥€傘€?

Rules for inode destruction:

inode 閿€姣佽鍒欙細

 - if ->destroy_inode() is non-NULL, it gets called
 - if ->free_inode() is non-NULL, it gets scheduled by call_rcu()
 - combination of NULL ->destroy_inode and NULL ->free_inode is
	  treated as NULL/free_inode_nonrcu, to preserve the compatibility.

 - 濡傛灉 ->destroy_inode() 闈?NULL锛屽垯浼氳璋冪敤
 - 濡傛灉 ->free_inode() 闈?NULL锛屽垯浼氳 call_rcu() 璋冨害
 - NULL ->destroy_inode 涓?NULL ->free_inode 鐨勭粍鍚堣瑙嗕负
	  NULL/free_inode_nonrcu锛屼互淇濇寔鍏煎鎬с€?

Note that the callback (be it via ->free_inode() or explicit call_rcu()
in ->destroy_inode()) is **NOT** ordered wrt superblock destruction;
as the matter of fact, the superblock and all associated structures
might be already gone.  The filesystem driver is guaranteed to be still
there, but that's it.  Freeing memory in the callback is fine; doing
more than that is possible, but requires a lot of care and is best
avoided.

娉ㄦ剰锛岃鍥炶皟锛堟棤璁烘槸閫氳繃 ->free_inode() 杩樻槸 ->destroy_inode() 涓殑鏄惧紡
call_rcu()锛変笌瓒呯骇鍧楅攢姣?*娌℃湁**椤哄簭淇濊瘉锛涗簨瀹炰笂锛岃秴绾у潡鍙婃墍鏈夌浉鍏崇粨鏋?
鍙兘鏃╁凡娑堝け銆傛枃浠剁郴缁熼┍鍔ㄤ繚璇佷粛鐒跺瓨鍦紝浣嗕粎姝よ€屽凡銆傚湪鍥炶皟涓噴鏀惧唴瀛樻病闂锛?
鍋氭洿澶氫簨鎯呮槸鍙兘鐨勶紝浣嗛渶瑕佹瀬搴﹀皬蹇冿紝鏈€濂介伩鍏嶃€?

---


**mandatory**

**寮哄埗**

DCACHE_RCUACCESS is gone; having an RCU delay on dentry freeing is the
default.  DCACHE_NORCU opts out, and only d_alloc_pseudo() has any
business doing so.

DCACHE_RCUACCESS 宸蹭笉瀛樺湪锛涘湪 dentry 閲婃斁鏃跺甫鏈?RCU 寤惰繜鏄粯璁よ涓恒€侱CACHE_NORCU
閫夋嫨閫€鍑猴紝骞朵笖鍙湁 d_alloc_pseudo() 鎵嶅簲褰撻偅鏍峰仛銆?

---


**mandatory**

**寮哄埗**

d_alloc_pseudo() is internal-only; uses outside of alloc_file_pseudo() are
very suspect (and won't work in modules).  Such uses are very likely to
be misspelled d_alloc_anon().

d_alloc_pseudo() 浠呴檺鍐呴儴浣跨敤锛涘湪 alloc_file_pseudo() 涔嬪鐨勪娇鐢ㄩ潪甯稿彲鐤戯紙涓斿湪
妯″潡涓棤娉曞伐浣滐級銆傛绫讳娇鐢ㄥ緢鍙兘鏄妸 d_alloc_anon() 鎷奸敊浜嗐€?

---


**mandatory**

**寮哄埗**

[should've been added in 2016] stale comment in finish_open() notwithstanding,
failure exits in ->atomic_open() instances should **NOT** fput() the file,
no matter what.  Everything is handled by the caller.

[鏈簲鍦?2016 骞村姞鍏 灏界 finish_open() 涓湁杩囨椂鐨勬敞閲婏紝->atomic_open() 瀹炰緥涓?
鐨勫け璐ラ€€鍑?*涓嶅簲** fput() 璇ユ枃浠讹紝鏃犺濡備綍閮戒笉搴斻€備竴鍒囩敱璋冪敤鏂瑰鐞嗐€?

---


**mandatory**

**寮哄埗**

clone_private_mount() returns a longterm mount now, so the proper destructor of
its result is kern_unmount() or kern_unmount_array().

clone_private_mount() 鐜板湪杩斿洖涓€涓暱鏈熸寕杞斤紙longterm mount锛夛紝鍥犳鍏剁粨鏋滅殑閫傚綋
鏋愭瀯鍑芥暟鏄?kern_unmount() 鎴?kern_unmount_array()銆?

---


**mandatory**

**寮哄埗**

zero-length bvec segments are disallowed, they must be filtered out before
passed on to an iterator.

闆堕暱搴︾殑 bvec 娈垫槸涓嶅厑璁哥殑锛屽湪浼犻€掔粰杩唬鍣ㄤ箣鍓嶅繀椤诲皢鍏惰繃婊ゆ帀銆?

---


**mandatory**

**寮哄埗**

For bvec based itererators bio_iov_iter_get_pages() now doesn't copy bvecs but
uses the one provided. Anyone issuing kiocb-I/O should ensure that the bvec and
page references stay until I/O has completed, i.e. until ->ki_complete() has
been called or returned with non -EIOCBQUEUED code.

瀵逛簬鍩轰簬 bvec 鐨勮凯浠ｅ櫒锛宐io_iov_iter_get_pages() 鐜板湪涓嶅啀澶嶅埗 bvec锛岃€屾槸浣跨敤鎻愪緵鐨?
閭ｄ竴涓€備换浣曞彂璧?kiocb-I/O 鐨勪汉閮藉簲纭繚 bvec 鍜岄〉寮曠敤涓€鐩翠繚鎸佸埌 I/O 瀹屾垚锛屽嵆鐩村埌
->ki_complete() 琚皟鐢ㄦ垨浠ラ潪 -EIOCBQUEUED 浠ｇ爜杩斿洖銆?

---


**mandatory**

**寮哄埗**

mnt_want_write_file() can now only be paired with mnt_drop_write_file(),
whereas previously it could be paired with mnt_drop_write() as well.

mnt_want_write_file() 鐜板湪鍙兘涓?mnt_drop_write_file() 閰嶅锛岃€屾鍓嶅畠涔熷彲浠ヤ笌
mnt_drop_write() 閰嶅銆?

---


**mandatory**

**寮哄埗**

iov_iter_copy_from_user_atomic() is gone; use copy_page_from_iter_atomic().
The difference is copy_page_from_iter_atomic() advances the iterator and
you don't need iov_iter_advance() after it.  However, if you decide to use
only a part of obtained data, you should do iov_iter_revert().

iov_iter_copy_from_user_atomic() 宸蹭笉瀛樺湪锛涜浣跨敤 copy_page_from_iter_atomic()銆?
鍖哄埆鍦ㄤ簬 copy_page_from_iter_atomic() 浼氭帹杩涜凯浠ｅ櫒锛屼箣鍚庝綘涓嶉渶瑕?iov_iter_advance()銆?
浣嗘槸锛屽鏋滀綘鍐冲畾鍙娇鐢ㄦ墍鑾锋暟鎹殑涓€閮ㄥ垎锛屼綘搴旇鍋?iov_iter_revert()銆?

---


**mandatory**

**寮哄埗**

Calling conventions for file_open_root() changed; now it takes struct path *
instead of passing mount and dentry separately.  For callers that used to
pass <mnt, mnt->mnt_root> pair (i.e. the root of given mount), a new helper
is provided - file_open_root_mnt().  In-tree users adjusted.

file_open_root() 鐨勮皟鐢ㄧ害瀹氭敼鍙樹簡锛涚幇鍦ㄥ畠鎺ュ彈 struct path * 鑰岄潪鍒嗗埆浼犲叆鎸傝浇鍜?
dentry銆傚浜庤繃鍘讳紶鍏?<mnt, mnt->mnt_root> 瀵癸紙鍗崇粰瀹氭寕杞界殑鏍癸級鐨勮皟鐢ㄦ柟锛屾彁渚涗簡涓€涓?
鏂拌緟鍔╁嚱鏁扳€斺€攆ile_open_root_mnt()銆傛爲鍐呯敤鎴峰凡鐩稿簲璋冩暣銆?

---


**mandatory**

**寮哄埗**

no_llseek is gone; don't set .llseek to that - just leave it NULL instead.
Checks for "does that file have llseek(2), or should it fail with ESPIPE"
should be done by looking at FMODE_LSEEK in file->f_mode.

no_llseek 宸蹭笉瀛樺湪锛涗笉瑕佸皢 .llseek 璁句负瀹冣€斺€斿彧闇€灏嗗叾淇濈暀涓?NULL銆傚"璇ユ枃浠舵槸鍚︽湁
llseek(2)锛岃繕鏄簲褰撲互 ESPIPE 澶辫触"鐨勬鏌ュ簲閫氳繃鏌ョ湅 file->f_mode 涓殑 FMODE_LSEEK
鏉ュ畬鎴愩€?

---


**mandatory**

**寮哄埗**

filldir_t (readdir callbacks) calling conventions have changed.  Instead of
returning 0 or -E... it returns bool now.  false means "no more" (as -E... used
to) and true - "keep going" (as 0 in old calling conventions).  Rationale:
callers never looked at specific -E... values anyway. -> iterate_shared()
instances require no changes at all, all filldir_t ones in the tree
converted.

filldir_t锛坮eaddir 鍥炶皟锛夌殑璋冪敤绾﹀畾宸叉敼鍙樸€傚畠鐜板湪杩斿洖 bool锛岃€岄潪 0 鎴?-E...銆?
false 琛ㄧず"娌℃湁浜?锛堝鍚岃繃鍘荤殑 -E...锛夛紝true 琛ㄧず"缁х画"锛堝鍚屾棫璋冪敤绾﹀畾涓殑 0锛夈€?
鐞嗙敱锛氳皟鐢ㄦ柟鍙嶆浠庢湭鏌ョ湅鍏蜂綋鐨?-E... 鍊笺€?>iterate_shared() 瀹炰緥瀹屽叏鏃犻渶鏀瑰姩锛屾爲鍐?
鎵€鏈?filldir_t 閮藉凡琚浆鎹€?

---


**mandatory**

**寮哄埗**

Calling conventions for ->tmpfile() have changed.  It now takes a struct
file pointer instead of struct dentry pointer.  d_tmpfile() is similarly
changed to simplify callers.  The passed file is in a non-open state and on
success must be opened before returning (e.g. by calling
finish_open_simple()).

->tmpfile() 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉銆傚畠鐜板湪鎺ュ彈涓€涓?struct file 鎸囬拡鑰岄潪 struct dentry
鎸囬拡銆俤_tmpfile() 涔熷仛浜嗙被浼兼敼鍔ㄤ互绠€鍖栬皟鐢ㄦ柟銆備紶鍏ョ殑鏂囦欢澶勪簬鏈墦寮€鐘舵€侊紝鎴愬姛鏃?
蹇呴』鍦ㄨ繑鍥炲墠鎵撳紑锛堜緥濡傞€氳繃璋冪敤 finish_open_simple()锛夈€?

---


**mandatory**

**寮哄埗**

Calling convention for ->huge_fault has changed.  It now takes a page
order instead of an enum page_entry_size, and it may be called without the
mmap_lock held.  All in-tree users have been audited and do not seem to
depend on the mmap_lock being held, but out of tree users should verify
for themselves.  If they do need it, they can return VM_FAULT_RETRY to
be called with the mmap_lock held.

->huge_fault 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉銆傚畠鐜板湪鎺ュ彈涓€涓〉闃讹紙page order锛夎€岄潪 enum
page_entry_size锛屽苟涓斿彲鑳藉湪鏈寔鏈?mmap_lock 鏃惰璋冪敤銆傛爲鍐呮墍鏈夌敤鎴烽兘宸茶瀹℃煡锛屼技涔?
涓嶄緷璧栨寔鏈?mmap_lock锛屼絾鏍戝鐢ㄦ埛搴旇嚜琛屾牳瀹炪€傚鏋滃畠浠‘瀹為渶瑕侊紝鍙互杩斿洖
VM_FAULT_RETRY 浠ヤ究鍦ㄦ寔鏈?mmap_lock 鏃惰璋冪敤銆?

---


**mandatory**

**寮哄埗**

The order of opening block devices and matching or creating superblocks has
changed.

鎵撳紑鍧楄澶囦笌鍖归厤鎴栧垱寤鸿秴绾у潡鐨勯『搴忓凡鏀瑰彉銆?

The old logic opened block devices first and then tried to find a
suitable superblock to reuse based on the block device pointer.

鏃ч€昏緫鍏堟墦寮€鍧楄澶囷紝鐒跺悗灏濊瘯鏍规嵁鍧楄澶囨寚閽堝鎵句竴涓彲澶嶇敤鐨勫悎閫傝秴绾у潡銆?

The new logic tries to find a suitable superblock first based on the device
number, and opening the block device afterwards.

鏂伴€昏緫鍏堝皾璇曟牴鎹澶囧彿瀵绘壘鍚堥€傜殑瓒呯骇鍧楋紝涔嬪悗鍐嶆墦寮€鍧楄澶囥€?

Since opening block devices cannot happen under s_umount because of lock
ordering requirements s_umount is now dropped while opening block devices and
reacquired before calling fill_super().

鐢变簬鍔犻攣椤哄簭瑕佹眰锛屾墦寮€鍧楄澶囦笉鑳藉湪 s_umount 涓嬭繘琛岋紝鍥犳鐜板湪鍦ㄦ墦寮€鍧楄澶囨椂浼氶噴鏀?
s_umount锛屽苟鍦ㄨ皟鐢?fill_super() 涔嬪墠閲嶆柊鑾峰彇銆?

In the old logic concurrent mounters would find the superblock on the list of
superblocks for the filesystem type. Since the first opener of the block device
would hold s_umount they would wait until the superblock became either born or
was discarded due to initialization failure.

鍦ㄦ棫閫昏緫涓紝骞跺彂鎸傝浇鑰呬細鍦ㄦ枃浠剁郴缁熺被鍨嬬殑瓒呯骇鍧楀垪琛ㄤ腑鎵惧埌璇ヨ秴绾у潡銆傜敱浜庡潡璁惧鐨?
绗竴涓墦寮€鑰呬細鎸佹湁 s_umount锛屽畠浠細绛夊緟鐩村埌璇ヨ秴绾у潡瑕佷箞"鍑虹敓"锛岃涔堝洜鍒濆鍖栧け璐?
鑰岃涓㈠純銆?

Since the new logic drops s_umount concurrent mounters could grab s_umount and
would spin. Instead they are now made to wait using an explicit wait-wake
mechanism without having to hold s_umount.

鐢变簬鏂伴€昏緫閲婃斁浜?s_umount锛屽苟鍙戞寕杞借€呭彲鑳借幏鍙?s_umount 骞惰嚜鏃嬨€傜浉鍙嶏紝鐜板湪瀹冧滑浣跨敤
涓€涓樉寮忕殑绛夊緟-鍞ら啋鏈哄埗鏉ョ瓑寰咃紝鑰屾棤闇€鎸佹湁 s_umount銆?

---


**mandatory**

**寮哄埗**

The holder of a block device is now the superblock.

鍧楄澶囩殑鎸佹湁鑰呯幇鍦ㄦ槸瓒呯骇鍧椼€?

The holder of a block device used to be the file_system_type which wasn't
particularly useful. It wasn't possible to go from block device to owning
superblock without matching on the device pointer stored in the superblock.
This mechanism would only work for a single device so the block layer couldn't
find the owning superblock of any additional devices.

鍧楄澶囩殑鎸佹湁鑰呰繃鍘绘槸 file_system_type锛岃繖骞朵笉鐗瑰埆鏈夌敤銆傚鏋滀笉鍖归厤瓒呯骇鍧椾腑瀛樺偍鐨?
璁惧鎸囬拡锛屽氨鏃犳硶浠庡潡璁惧鍥炴函鍒版墍灞炶秴绾у潡銆傝鏈哄埗鍙兘鐢ㄤ簬鍗曚釜璁惧锛屽洜姝ゅ潡灞傛棤娉?
鎵惧埌浠讳綍棰濆璁惧鐨勬墍灞炶秴绾у潡銆?

In the old mechanism reusing or creating a superblock for a racing mount(2) and
umount(2) relied on the file_system_type as the holder. This was severely
underdocumented however:

鍦ㄦ棫鏈哄埗涓紝涓虹珵鎬佷腑鐨?mount(2) 鍜?umount(2) 澶嶇敤鎴栧垱寤鸿秴绾у潡渚濊禆浜庝綔涓烘寔鏈夎€呯殑
file_system_type銆傜劧鑰岃繖涓€鐐规枃妗ｈ褰曚弗閲嶄笉瓒筹細

(1) Any concurrent mounter that managed to grab an active reference on an
    existing superblock was made to wait until the superblock either became
    ready or until the superblock was removed from the list of superblocks of
    the filesystem type. If the superblock is ready the caller would simple
    reuse it.

(1) 浠讳綍鎴愬姛鑾峰彇鐜版湁瓒呯骇鍧楁椿鍔ㄥ紩鐢ㄧ殑骞跺彂鎸傝浇鑰呬細琚姹傜瓑寰咃紝鐩村埌璇ヨ秴绾у潡瑕佷箞
    鍑嗗灏辩华锛岃涔堜粠鏂囦欢绯荤粺绫诲瀷鐨勮秴绾у潡鍒楄〃涓绉婚櫎銆傚鏋滆秴绾у潡宸插氨缁紝璋冪敤鏂?
    鍙渶澶嶇敤瀹冦€?

(2) If the mounter came after deactivate_locked_super() but before
    the superblock had been removed from the list of superblocks of the
    filesystem type the mounter would wait until the superblock was shutdown,
    reuse the block device and allocate a new superblock.

(2) 濡傛灉鎸傝浇鑰呭湪 deactivate_locked_super() 涔嬪悗銆佷絾瓒呯骇鍧椾粠鏂囦欢绯荤粺绫诲瀷鐨勮秴绾у潡
    鍒楄〃涓绉婚櫎涔嬪墠鍒版潵锛屾寕杞借€呬細绛夊緟鐩村埌璇ヨ秴绾у潡鍏抽棴锛屽鐢ㄥ潡璁惧骞跺垎閰嶄竴涓柊鐨?
    瓒呯骇鍧椼€?

(3) If the mounter came after deactivate_locked_super() and after
    the superblock had been removed from the list of superblocks of the
    filesystem type the mounter would reuse the block device and allocate a new
    superblock (the bd_holder point may still be set to the filesystem type).

(3) 濡傛灉鎸傝浇鑰呭湪 deactivate_locked_super() 涔嬪悗銆佷笖瓒呯骇鍧楀凡浠庢枃浠剁郴缁熺被鍨嬬殑瓒呯骇鍧?
    鍒楄〃涓绉婚櫎涔嬪悗鍒版潵锛屾寕杞借€呬細澶嶇敤鍧楄澶囧苟鍒嗛厤涓€涓柊鐨勮秴绾у潡锛坆d_holder 鎸囬拡
    鍙兘浠嶈璁句负鏂囦欢绯荤粺绫诲瀷锛夈€?

Because the holder of the block device was the file_system_type any concurrent
mounter could open the block devices of any superblock of the same
file_system_type without risking seeing EBUSY because the block device was
still in use by another superblock.

鐢变簬鍧楄澶囩殑鎸佹湁鑰呮槸 file_system_type锛屼换浣曞苟鍙戞寕杞借€呴兘鍙互鎵撳紑鍚屼竴 file_system_type
鐨勪换浣曡秴绾у潡鐨勫潡璁惧锛岃€屼笉蹇呮媴蹇冪湅鍒?EBUSY锛屽洜涓鸿鍧楄澶囦粛琚彟涓€涓秴绾у潡浣跨敤銆?

Making the superblock the owner of the block device changes this as the holder
is now a unique superblock and thus block devices associated with it cannot be
reused by concurrent mounters. So a concurrent mounter in (2) could suddenly
see EBUSY when trying to open a block device whose holder was a different
superblock.

璁╄秴绾у潡鎴愪负鍧楄澶囩殑鎷ユ湁鑰呮敼鍙樹簡杩欎竴鐐癸紝鍥犱负鎸佹湁鑰呯幇鍦ㄦ槸涓€涓敮涓€鐨勮秴绾у潡锛屽洜姝?
涓庝箣鍏宠仈鐨勫潡璁惧涓嶈兘琚苟鍙戞寕杞借€呭鐢ㄣ€傛墍浠ワ紝(2) 涓殑骞跺彂鎸傝浇鑰呭湪灏濊瘯鎵撳紑涓€涓寔鏈夎€呬负
涓嶅悓瓒呯骇鍧楃殑鍧楄澶囨椂锛屽彲鑳界獊鐒剁湅鍒?EBUSY銆?

The new logic thus waits until the superblock and the devices are shutdown in
->kill_sb(). Removal of the superblock from the list of superblocks of the
filesystem type is now moved to a later point when the devices are closed:

鍥犳鏂伴€昏緫浼氱瓑寰呯洿鍒拌秴绾у潡鍜岃澶囧湪 ->kill_sb() 涓鍏抽棴銆傝秴绾у潡浠庢枃浠剁郴缁熺被鍨嬬殑
瓒呯骇鍧楀垪琛ㄤ腑绉婚櫎锛岀幇鍦ㄨ鎺ㄨ繜鍒拌澶囧叧闂椂锛?

(1) Any concurrent mounter managing to grab an active reference on an existing
    superblock is made to wait until the superblock is either ready or until
    the superblock and all devices are shutdown in ->kill_sb(). If the
    superblock is ready the caller will simply reuse it.

(1) 浠讳綍鎴愬姛鑾峰彇鐜版湁瓒呯骇鍧楁椿鍔ㄥ紩鐢ㄧ殑骞跺彂鎸傝浇鑰呬細琚姹傜瓑寰咃紝鐩村埌璇ヨ秴绾у潡瑕佷箞灏辩华锛?
    瑕佷箞瓒呯骇鍧楀拰鎵€鏈夎澶囧湪 ->kill_sb() 涓鍏抽棴銆傚鏋滆秴绾у潡宸插氨缁紝璋冪敤鏂瑰彧闇€澶嶇敤瀹冦€?

(2) If the mounter comes after deactivate_locked_super() but before
    the superblock has been removed from the list of superblocks of the
    filesystem type the mounter is made to wait until the superblock and the
    devices are shut down in ->kill_sb() and the superblock is removed from the
    list of superblocks of the filesystem type. The mounter will allocate a new
    superblock and grab ownership of the block device (the bd_holder pointer of
    the block device will be set to the newly allocated superblock).

(2) 濡傛灉鎸傝浇鑰呭湪 deactivate_locked_super() 涔嬪悗銆佷絾瓒呯骇鍧椾粠鏂囦欢绯荤粺绫诲瀷鐨勮秴绾у潡
    鍒楄〃涓绉婚櫎涔嬪墠鍒版潵锛屾寕杞借€呬細琚姹傜瓑寰咃紝鐩村埌瓒呯骇鍧楀拰璁惧鍦?->kill_sb() 涓叧闂紝
    骞朵笖璇ヨ秴绾у潡浠庢枃浠剁郴缁熺被鍨嬬殑瓒呯骇鍧楀垪琛ㄤ腑琚Щ闄ゃ€傛寕杞借€呭皢鍒嗛厤涓€涓柊鐨勮秴绾у潡骞惰幏鍙?
    鍧楄澶囩殑鎵€鏈夋潈锛堝潡璁惧鐨?bd_holder 鎸囬拡灏嗚璁句负鏂板垎閰嶇殑瓒呯骇鍧楋級銆?

(3) This case is now collapsed into (2) as the superblock is left on the list
    of superblocks of the filesystem type until all devices are shutdown in
    ->kill_sb(). In other words, if the superblock isn't on the list of
    superblock of the filesystem type anymore then it has given up ownership of
    all associated block devices (the bd_holder pointer is NULL).

(3) 杩欑鎯呭喌鐜板湪琚苟鍏?(2)锛屽洜涓鸿秴绾у潡浼氫竴鐩寸暀鍦ㄦ枃浠剁郴缁熺被鍨嬬殑瓒呯骇鍧楀垪琛ㄤ腑锛岀洿鍒版墍鏈?
    璁惧鍦?->kill_sb() 涓叧闂€傛崲瑷€涔嬶紝濡傛灉瓒呯骇鍧楀凡涓嶅湪鏂囦欢绯荤粺绫诲瀷鐨勮秴绾у潡鍒楄〃涓紝
    閭ｄ箞瀹冨凡缁忔斁寮冧簡鎵€鏈夊叧鑱斿潡璁惧鐨勬墍鏈夋潈锛坆d_holder 鎸囬拡涓?NULL锛夈€?

As this is a VFS level change it has no practical consequences for filesystems
other than that all of them must use one of the provided kill_litter_super(),
kill_anon_super(), or kill_block_super() helpers.

鐢变簬杩欐槸涓€涓?VFS 绾у埆鐨勫彉鏇达紝瀹冨鏂囦欢绯荤粺娌℃湁瀹為檯褰卞搷锛屽彧鏄墍鏈夋枃浠剁郴缁熼兘蹇呴』浣跨敤
鎵€鎻愪緵鐨?kill_litter_super()銆乲ill_anon_super() 鎴?kill_block_super() 杈呭姪鍑芥暟涔嬩竴銆?

---


**mandatory**

**寮哄埗**

Lock ordering has been changed so that s_umount ranks above open_mutex again.
All places where s_umount was taken under open_mutex have been fixed up.

鍔犻攣椤哄簭宸叉敼鍙橈紝浣垮緱 s_umount 鍐嶆鎺掑湪 open_mutex 涔嬩笂銆傛墍鏈夊湪 open_mutex 涓嬭幏鍙?
s_umount 鐨勫湴鏂归兘宸茶淇銆?

---


**mandatory**

**寮哄埗**

export_operations ->encode_fh() no longer has a default implementation to
encode FILEID_INO32_GEN* file handles.
Filesystems that used the default implementation may use the generic helper
generic_encode_ino32_fh() explicitly.

export_operations ->encode_fh() 涓嶅啀鏈夌敤浜庣紪鐮?FILEID_INO32_GEN* 鏂囦欢鍙ユ焺鐨勯粯璁?
瀹炵幇銆傛浘浣跨敤榛樿瀹炵幇鐨勬枃浠剁郴缁熷彲浠ユ樉寮忎娇鐢ㄩ€氱敤杈呭姪鍑芥暟 generic_encode_ino32_fh()銆?

---


**mandatory**

**寮哄埗**

If ->rename() update of .. on cross-directory move needs an exclusion with
directory modifications, do **not** lock the subdirectory in question in your
->rename() - it's done by the caller now [that item should've been added in
28eceeda130f "fs: Lock moved directories"].

濡傛灉 ->rename() 鍦ㄨ法鐩綍绉诲姩鏃跺 .. 鐨勬洿鏂伴渶瑕佺洰褰曚慨鏀圭殑浜掓枼锛岃**涓嶈**鍦ㄤ綘鐨?
->rename() 涓攣瀹氱浉鍏崇殑瀛愮洰褰曗€斺€旂幇鍦ㄥ畠鐢辫皟鐢ㄦ柟瀹屾垚 [璇ラ」鏈簲鍦?28eceeda130f
"fs: Lock moved directories" 涓姞鍏銆?

---


**mandatory**

**寮哄埗**

On same-directory ->rename() the (tautological) update of .. is not protected
by any locks; just don't do it if the old parent is the same as the new one.
We really can't lock two subdirectories in same-directory rename - not without
deadlocks.

鍦ㄥ悓鐩綍 ->rename() 涓紝瀵?.. 鐨勶紙鍚屼箟鍙嶅寮忕殑锛夋洿鏂颁笉鍙椾换浣曢攣淇濇姢锛涘鏋滄棫鐖剁洰褰?
涓庢柊鐖剁洰褰曠浉鍚岋紝灏卞共鑴嗕笉瑕佸仛瀹冦€傛垜浠‘瀹炴棤娉曞湪鍚岀洰褰曢噸鍛藉悕涓攣瀹氫袱涓瓙鐩綍鈥斺€斿惁鍒?
浼氭閿併€?

---


**mandatory**

**寮哄埗**

lock_rename() and lock_rename_child() may fail in cross-directory case, if
their arguments do not have a common ancestor.  In that case ERR_PTR(-EXDEV)
is returned, with no locks taken.  In-tree users updated; out-of-tree ones
would need to do so.

lock_rename() 鍜?lock_rename_child() 鍦ㄨ法鐩綍鎯呭喌涓嬪彲鑳藉け璐ワ紝濡傛灉瀹冧滑鐨勫弬鏁版病鏈?
鍏卞悓绁栧厛銆傛鏃惰繑鍥?ERR_PTR(-EXDEV)锛屼笖涓嶈幏鍙栦换浣曢攣銆傛爲鍐呯敤鎴峰凡鏇存柊锛涙爲澶栫敤鎴烽渶瑕?
鑷鏇存柊銆?

---


**mandatory**

**寮哄埗**

The list of children anchored in parent dentry got turned into hlist now.
Field names got changed (->d_children/->d_sib instead of ->d_subdirs/->d_child
for anchor/entries resp.), so any affected places will be immediately caught
by compiler.

閿氬畾鍦ㄧ埗 dentry 涓殑瀛愰」鍒楄〃鐜板湪鍙樻垚浜?hlist銆傚瓧娈靛悕涔熸敼浜嗭紙閿氱偣/瀛愰」鍒嗗埆鐢?
->d_children/->d_sib 鍙栦唬 ->d_subdirs/->d_child锛夛紝鍥犳浠讳綍鍙楀奖鍝嶇殑鍦版柟浼氳缂栬瘧鍣?
绔嬪嵆鎹曡幏銆?

---


**mandatory**

**寮哄埗**

->d_delete() instances are now called for dentries with ->d_lock held
and refcount equal to 0.  They are not permitted to drop/regain ->d_lock.
None of in-tree instances did anything of that sort.  Make sure yours do not...

->d_delete() 瀹炰緥鐜板湪鍦ㄦ寔鏈?->d_lock 涓斿紩鐢ㄨ鏁颁负 0 鐨?dentries 涓婅璋冪敤銆傚畠浠?
涓嶅厑璁搁噴鏀?閲嶆柊鑾峰彇 ->d_lock銆傛爲鍐呭疄渚嬮兘娌℃湁鍋氳繖绫讳簨鎯呫€傝纭繚浣犵殑瀹炰緥涔熶笉瑕佲€︹€?

---


**mandatory**

**寮哄埗**

->d_prune() instances are now called without ->d_lock held on the parent.
->d_lock on dentry itself is still held; if you need per-parent exclusions (none
of the in-tree instances did), use your own spinlock.

->d_prune() 瀹炰緥鐜板湪鍦ㄧ埗鐩綍鏈寔鏈?->d_lock 鏃惰璋冪敤銆俤entry 鑷韩鐨?->d_lock 浠嶈
鎸佹湁锛涘鏋滀綘闇€瑕佹瘡鐖剁洰褰曠殑浜掓枼锛堟爲鍐呭疄渚嬮兘涓嶉渶瑕侊級锛岃浣跨敤浣犺嚜宸辩殑鑷棆閿併€?

->d_iput() and ->d_release() are called with victim dentry still in the
list of parent's children.  It is still unhashed, marked killed, etc., just not
removed from parent's ->d_children yet.

->d_iput() 鍜?->d_release() 鍦ㄥ彈瀹宠€?dentry 浠嶄綅浜庣埗鐩綍鐨勫瓙椤瑰垪琛ㄤ腑琚皟鐢ㄣ€傚畠
浠嶆湭琚暎鍒椼€佽鏍囪涓哄凡鏉€姝荤瓑锛屽彧鏄皻鏈粠鐖剁洰褰曠殑 ->d_children 涓Щ闄ゃ€?

Anyone iterating through the list of children needs to be aware of the
half-killed dentries that might be seen there; taking ->d_lock on those will
see them negative, unhashed and with negative refcount, which means that most
of the in-kernel users would've done the right thing anyway without any adjustment.

浠讳綍閬嶅巻瀛愰」鍒楄〃鐨勪汉閮介渶瑕佹剰璇嗗埌閭ｉ噷鍙兘鐪嬪埌鍗婃潃姝伙紙half-killed锛夌殑 dentries锛涘
瀹冧滑鑾峰彇 ->d_lock 浼氱湅鍒板畠浠槸璐熺殑銆佹湭鏁ｅ垪鐨勶紝涓斿紩鐢ㄨ鏁颁负璐燂紝杩欐剰鍛崇潃澶у鏁板唴鏍稿唴
鐢ㄦ埛鏃犺濡備綍閮戒細鍋氭纭殑浜嬶紝鏃犻渶浠讳綍璋冩暣銆?

---


**recommended**

**寤鸿**

Block device freezing and thawing have been moved to holder operations.

鍧楄澶囩殑鍐荤粨锛坒reezing锛変笌瑙ｅ喕锛坱hawing锛夊凡绉昏嚦鎸佹湁鑰呮搷浣溿€?

Before this change, get_active_super() would only be able to find the
superblock of the main block device, i.e., the one stored in sb->s_bdev. Block
device freezing now works for any block device owned by a given superblock, not
just the main block device. The get_active_super() helper and bd_fsfreeze_sb
pointer are gone.

鍦ㄦ鍙樻洿涔嬪墠锛実et_active_super() 鍙兘鎵惧埌涓诲潡璁惧锛堝嵆瀛樺偍鍦?sb->s_bdev 涓殑閭ｄ釜锛?
鐨勮秴绾у潡銆傚潡璁惧鍐荤粨鐜板湪閫傜敤浜庣粰瀹氳秴绾у潡鎷ユ湁鐨勪换浣曞潡璁惧锛岃€屼笉浠呬粎鏄富鍧楄澶囥€?
get_active_super() 杈呭姪鍑芥暟鍜?bd_fsfreeze_sb 鎸囬拡宸蹭笉瀛樺湪銆?

---


**mandatory**

**寮哄埗**

set_blocksize() takes opened struct file instead of struct block_device now
and it **must** be opened exclusive.

set_blocksize() 鐜板湪鎺ュ彈宸叉墦寮€鐨?struct file 鑰岄潪 struct block_device锛屽苟涓斿畠
**蹇呴』**浠ョ嫭鍗犳柟寮忔墦寮€銆?

---


**mandatory**

**寮哄埗**

->d_revalidate() gets two extra arguments - inode of parent directory and
name our dentry is expected to have.  Both are stable (dir is pinned in
non-RCU case and will stay around during the call in RCU case, and name
is guaranteed to stay unchanging).  Your instance doesn't have to use
either, but it often helps to avoid a lot of painful boilerplate.
Note that while name->name is stable and NUL-terminated, it may (and
often will) have name->name[name->len] equal to '/' rather than '\0' -
in normal case it points into the pathname being looked up.
NOTE: if you need something like full path from the root of filesystem,
you are still on your own - this assists with simple cases, but it's not
magic.

->d_revalidate() 鑾峰緱涓や釜棰濆鐨勫弬鏁扳€斺€旂埗鐩綍鐨?inode 鍜屾垜浠殑 dentry 棰勬湡鎷ユ湁鐨?
鍚嶅瓧銆備袱鑰呴兘鏄ǔ瀹氱殑锛堝湪闈?RCU 鎯呭喌涓?dir 琚浐瀹氾紝鍦?RCU 鎯呭喌涓嬭皟鐢ㄦ湡闂翠篃浼氫繚鐣欙紝
涓斿悕瀛椾繚璇佷繚鎸佷笉鍙橈級銆備綘鐨勫疄渚嬩笉涓€瀹氳浣跨敤瀹冧滑锛屼絾浣跨敤瀹冧滑閫氬父鏈夊姪浜庨伩鍏嶅ぇ閲忕棝鑻︾殑
鏍锋澘浠ｇ爜銆傛敞鎰忥紝铏界劧 name->name 鏄ǔ瀹氫笖浠?NUL 缁撳熬鐨勶紝浣嗗畠鍙兘锛堜笖缁忓父锛変娇寰?
name->name[name->len] 绛変簬 '/' 鑰屼笉鏄?'\0'鈥斺€旀甯告儏鍐典笅瀹冩寚鍚戞鍦ㄦ煡鎵剧殑璺緞鍚嶃€?
娉ㄦ剰锛氬鏋滀綘闇€瑕佺被浼间粠鏂囦欢绯荤粺鏍瑰紑濮嬬殑瀹屾暣璺緞锛屼粛闇€鑷繁澶勭悊鈥斺€旇繖鍙兘鍗忓姪绠€鍗曟儏鍐碉紝
骞堕潪榄旀硶銆?

---


**recommended**

**寤鸿**

kern_path_locked() and user_path_locked() no longer return a negative
dentry so this doesn't need to be checked.  If the name cannot be found,
ERR_PTR(-ENOENT) is returned.

kern_path_locked() 鍜?user_path_locked() 涓嶅啀杩斿洖璐?dentry锛屽洜姝ゆ棤闇€妫€鏌ヨ繖涓€鐐广€?
濡傛灉鎵句笉鍒拌鍚嶅瓧锛屽垯杩斿洖 ERR_PTR(-ENOENT)銆?

---


**recommended**

**寤鸿**

lookup_one_qstr_excl() is changed to return errors in more cases, so
these conditions don't require explicit checks:

lookup_one_qstr_excl() 琚敼涓哄湪鏇村鎯呭喌涓嬭繑鍥為敊璇紝鍥犳杩欎簺鏉′欢鏃犻渶鏄惧紡妫€鏌ワ細

 - if LOOKUP_CREATE is NOT given, then the dentry won't be negative,
   ERR_PTR(-ENOENT) is returned instead
 - if LOOKUP_EXCL IS given, then the dentry won't be positive,
   ERR_PTR(-EEXIST) is rreturned instread

 - 濡傛灉鏈粰瀹?LOOKUP_CREATE锛屽垯 dentry 涓嶄細鏄礋鐨勶紝鏀逛负杩斿洖
   ERR_PTR(-ENOENT)
 - 濡傛灉缁欏畾浜?LOOKUP_EXCL锛屽垯 dentry 涓嶄細鏄鐨勶紝鏀逛负杩斿洖
   ERR_PTR(-EEXIST)

LOOKUP_EXCL now means "target must not exist".  It can be combined with
LOOK_CREATE or LOOKUP_RENAME_TARGET.

LOOKUP_EXCL 鐜板湪鎰忎负"鐩爣蹇呴』涓嶅瓨鍦?銆傚畠鍙互涓?LOOK_CREATE 鎴?
LOOKUP_RENAME_TARGET 缁勫悎銆?

---


**mandatory**

invalidate_inodes() is gone use evict_inodes() instead.

invalidate_inodes() 宸蹭笉瀛樺湪锛涜鏀圭敤 evict_inodes()銆?

---


**mandatory**

**寮哄埗**

->mkdir() now returns a dentry.  If the created inode is found to
already be in cache and have a dentry (often IS_ROOT()), it will need to
be spliced into the given name in place of the given dentry. That dentry
now needs to be returned.  If the original dentry is used, NULL should
be returned.  Any error should be returned with ERR_PTR().

->mkdir() 鐜板湪杩斿洖涓€涓?dentry銆傚鏋滃垱寤虹殑 inode 琚彂鐜板凡鍦ㄧ紦瀛樹腑涓旀嫢鏈変竴涓?dentry
锛堥€氬父鏄?IS_ROOT()锛夛紝鍒欓渶瑕佸皢鍏舵嫾鎺ヨ繘缁欏畾鍚嶅瓧浠ュ彇浠ｇ粰瀹氱殑 dentry銆傜幇鍦ㄩ渶瑕佽繑鍥為偅涓?
dentry銆傚鏋滀娇鐢ㄤ簡鍘熷 dentry锛屽垯搴旇繑鍥?NULL銆備换浣曢敊璇兘搴旈€氳繃 ERR_PTR() 杩斿洖銆?

In general, filesystems which use d_instantiate_new() to install the new
inode can safely return NULL.  Filesystems which may not have an I_NEW inode
should use d_drop();d_splice_alias() and return the result of the latter.

涓€鑸潵璇达紝浣跨敤 d_instantiate_new() 瀹夎鏂?inode 鐨勬枃浠剁郴缁熷彲浠ュ畨鍏ㄨ繑鍥?NULL銆傚彲鑳?
娌℃湁 I_NEW inode 鐨勬枃浠剁郴缁熷簲浣跨敤 d_drop();d_splice_alias() 骞惰繑鍥炲悗鑰呯殑缁撴灉銆?

If a positive dentry cannot be returned for some reason, in-kernel
clients such as cachefiles, nfsd, smb/server may not perform ideally but
will fail-safe.

濡傛灉鍑轰簬鏌愮鍘熷洜鏃犳硶杩斿洖姝ｇ殑 dentry锛岃濡?cachefiles銆乶fsd銆乻mb/server 绛夊唴鏍稿唴
瀹㈡埛绔彲鑳芥棤娉曡揪鍒扮悊鎯宠〃鐜帮紝浣嗕細瀹夊叏澶辫触銆?

---


** mandatory**

**寮哄埗**

lookup_one(), lookup_one_unlocked(), lookup_one_positive_unlocked() now
take a qstr instead of a name and len.  These, not the "one_len"
versions, should be used whenever accessing a filesystem from outside
that filesysmtem, through a mount point - which will have a mnt_idmap.

lookup_one()銆乴ookup_one_unlocked()銆乴ookup_one_positive_unlocked() 鐜板湪鎺ュ彈涓€涓?
qstr 鑰岄潪 name 鍜?len銆傛瘡褰撻€氳繃鎸傝浇鐐癸紙瀹冨皢鏈変竴涓?mnt_idmap锛変粠鏂囦欢绯荤粺澶栭儴璁块棶璇?
鏂囦欢绯荤粺鏃讹紝閮藉簲浣跨敤杩欎簺鑰岄潪"one_len"鐗堟湰銆?

---


** mandatory**

**寮哄埗**

Functions try_lookup_one_len(), lookup_one_len(),
lookup_one_len_unlocked() and lookup_positive_unlocked() have been
renamed to try_lookup_noperm(), lookup_noperm(),
lookup_noperm_unlocked(), lookup_noperm_positive_unlocked().  They now
take a qstr instead of separate name and length.  QSTR() can be used
when strlen() is needed for the length.

鍑芥暟 try_lookup_one_len()銆乴ookup_one_len()銆乴ookup_one_len_unlocked() 鍜?
lookup_positive_unlocked() 宸查噸鍛藉悕涓?try_lookup_noperm()銆乴ookup_noperm()銆?
lookup_noperm_unlocked()銆乴ookup_noperm_positive_unlocked()銆傚畠浠幇鍦ㄦ帴鍙?qstr 鑰岄潪
鍒嗗紑鐨?name 鍜?length銆傚綋闇€瑕佷互 strlen() 浣滀负闀垮害鏃讹紝鍙互浣跨敤 QSTR()銆?

These function no longer do any permission checking - they previously
checked that the caller has 'X' permission on the parent.  They must
ONLY be used internally by a filesystem on itself when it knows that
permissions are irrelevant or in a context where permission checks have
already been performed such as after vfs_path_parent_lookup()

杩欎簺鍑芥暟涓嶅啀鍋氫换浣曟潈闄愭鏌モ€斺€斿畠浠繃鍘讳細妫€鏌ヨ皟鐢ㄦ柟鍦ㄧ埗鐩綍涓婃嫢鏈?'X' 鏉冮檺銆傚畠浠?
鍙兘鐢辨枃浠剁郴缁熷湪瀹冭嚜韬笂鍐呴儴浣跨敤锛屽綋瀹冪煡閬撴潈闄愭棤鍏崇揣瑕侊紝鎴栧凡鍦ㄦ潈闄愭鏌ュ凡瀹屾垚鐨勪笂涓嬫枃
涓紙渚嬪 vfs_path_parent_lookup() 涔嬪悗锛夈€?

---


** mandatory**

**寮哄埗**

d_hash_and_lookup() is no longer exported or available outside the VFS.
Use try_lookup_noperm() instead.  This adds name validation and takes
arguments in the opposite order but is otherwise identical.

d_hash_and_lookup() 涓嶅啀琚鍑猴紝涔熸棤娉曞湪 VFS 涔嬪浣跨敤銆傝鏀圭敤
try_lookup_noperm()銆傚畠浼氶澶栬繘琛屽悕瀛楁牎楠岋紝骞朵互鐩稿弽椤哄簭鎺ュ彈鍙傛暟锛岄櫎姝や箣澶栧畬鍏ㄧ浉鍚屻€?

Using try_lookup_noperm() will require linux/namei.h to be included.

浣跨敤 try_lookup_noperm() 闇€瑕佸寘鍚?linux/namei.h銆?

---


**mandatory**

**寮哄埗**

Calling conventions for ->d_automount() have changed; we should **not** grab
an extra reference to new mount - it should be returned with refcount 1.

->d_automount() 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉锛涙垜浠?*涓嶅簲**瀵规柊鎸傝浇鑾峰彇棰濆寮曠敤鈥斺€斿畠搴斾互寮曠敤璁℃暟
1 杩斿洖銆?

---

collect_mounts()/drop_collected_mounts()/iterate_mounts() are gone now.
Replacement is collect_paths()/drop_collected_path(), with no special
iterator needed.  Instead of a cloned mount tree, the new interface returns
an array of struct path, one for each mount collect_mounts() would've
created.  These struct path point to locations in the caller's namespace
that would be roots of the cloned mounts.

collect_mounts()/drop_collected_mounts()/iterate_mounts() 鐜板湪宸蹭笉瀛樺湪銆傚彇浠ｈ€?
鏄?collect_paths()/drop_collected_path()锛屾棤闇€鐗规畩杩唬鍣ㄣ€傛柊鎺ュ彛涓嶅啀杩斿洖鍏嬮殕鐨勬寕杞?
鏍戯紝鑰屾槸杩斿洖涓€涓?struct path 鏁扮粍锛宑ollect_mounts() 鍘熸湰浼氫负姣忎釜鎸傝浇鍒涘缓涓€涓€傝繖浜?
struct path 鎸囧悜璋冪敤鏂瑰懡鍚嶇┖闂翠腑鐨勪綅缃紝鍗抽偅浜涘厠闅嗘寕杞界殑鏍广€?

---


**mandatory**

**寮哄埗**

If your filesystem sets the default dentry_operations, use set_default_d_op()
rather than manually setting sb->s_d_op.

濡傛灉浣犵殑鏂囦欢绯荤粺璁剧疆浜嗛粯璁?dentry_operations锛岃浣跨敤 set_default_d_op() 鑰岄潪鎵嬪姩
璁剧疆 sb->s_d_op銆?

---


**mandatory**

**寮哄埗**

d_set_d_op() is no longer exported (or public, for that matter); _if_
your filesystem really needed that, make use of d_splice_alias_ops()
to have them set.  Better yet, think hard whether you need different
->d_op for different dentries - if not, just use set_default_d_op()
at mount time and be done with that.  Currently procfs is the only
thing that really needs ->d_op varying between dentries.

d_set_d_op() 涓嶅啀琚鍑猴紙浜嬪疄涓婁篃涓嶅啀鏄叕寮€鐨勶級锛沖濡傛灉_浣犵殑鏂囦欢绯荤粺纭疄鏇剧粡闇€瑕佸畠锛?
璇峰埄鐢?d_splice_alias_ops() 鏉ヨ缃畠浠€傛洿濂界殑鍋氭硶鏄紝璁ょ湡鑰冭檻浣犳槸鍚︾湡鐨勯渶瑕佷负涓嶅悓鐨?
dentries 浣跨敤涓嶅悓鐨?->d_op鈥斺€斿鏋滀笉闇€瑕侊紝鍙渶鍦ㄦ寕杞芥椂浣跨敤 set_default_d_op() 骞朵簡浜嬨€?
鐩墠 procfs 鏄敮涓€鐪熸闇€瑕?->d_op 鍦?dentries 涔嬮棿鍙樺寲鐨勪笢瑗裤€?

---


**highly recommended**

**楂樺害寤鸿**

The file operations mmap() callback is deprecated in favour of
mmap_prepare(). This passes a pointer to a vm_area_desc to the callback
rather than a VMA, as the VMA at this stage is not yet valid.

鏂囦欢鎿嶄綔 mmap() 鍥炶皟宸茶寮冪敤锛屾帹鑽愪娇鐢?mmap_prepare()銆傚畠浼氬悜鍥炶皟浼犻€掍竴涓寚鍚?
vm_area_desc 鐨勬寚閽堬紝鑰岄潪 VMA锛屽洜涓哄湪姝ら樁娈?VMA 灏氭湭鏈夋晥銆?

The vm_area_desc provides the minimum required information for a filesystem
to initialise state upon memory mapping of a file-backed region, and output
parameters for the file system to set this state.

vm_area_desc 鎻愪緵浜嗘枃浠剁郴缁熷湪鏂囦欢鍚庡锛坒ile-backed锛夊尯鍩熷唴瀛樻槧灏勬椂鍒濆鍖栫姸鎬佹墍闇€鐨?
鏈€灏戜俊鎭紝浠ュ強渚涙枃浠剁郴缁熻缃鐘舵€佺殑杈撳嚭鍙傛暟銆?

In nearly all cases, this is all that is required for a filesystem. However, if
a filesystem needs to perform an operation such a pre-population of page tables,
then that action can be specified in the vm_area_desc->action field, which can
be configured using the mmap_action_*() helpers.

鍦ㄥ嚑涔庢墍鏈夋儏鍐典笅锛岃繖瀵规枃浠剁郴缁熻€岃█宸茶冻澶熴€備絾鏄紝濡傛灉鏂囦欢绯荤粺闇€瑕佹墽琛岃濡傞濉厖椤佃〃
涔嬬被鐨勬搷浣滐紝鍒欏彲浠ュ湪 vm_area_desc->action 瀛楁涓寚瀹氳鍔ㄤ綔锛屽畠鍙互浣跨敤 mmap_action_*()
杈呭姪鍑芥暟杩涜閰嶇疆銆?

---


**mandatory**

**寮哄埗**

Several functions are renamed:

鑻ュ共鍑芥暟琚噸鍛藉悕锛?

- kern_path_locked -> start_removing_path
- kern_path_create -> start_creating_path
- user_path_create -> start_creating_user_path
- user_path_locked_at -> start_removing_user_path_at
- done_path_create -> end_creating_path

- kern_path_locked -> start_removing_path
- kern_path_create -> start_creating_path
- user_path_create -> start_creating_user_path
- user_path_locked_at -> start_removing_user_path_at
- done_path_create -> end_creating_path

---


**mandatory**

**寮哄埗**

Calling conventions for vfs_parse_fs_string() have changed; it does **not**
take length anymore (value ? strlen(value) : 0 is used).  If you want
a different length, use

vfs_parse_fs_string() 鐨勮皟鐢ㄧ害瀹氬凡鏀瑰彉锛涘畠**涓嶅啀**鎺ュ彈闀垮害鍙傛暟锛堜娇鐢?
value ? strlen(value) : 0锛夈€傚鏋滀綘鎯宠涓嶅悓鐨勯暱搴︼紝璇蜂娇鐢?

	vfs_parse_fs_qstr(fc, key, &QSTR_LEN(value, len))

instead.

鏉ヤ唬鏇裤€?

---


**mandatory**

**寮哄埗**

vfs_mkdir() now returns a dentry - the one returned by ->mkdir().  If
that dentry is different from the dentry passed in, including if it is
an IS_ERR() dentry pointer, the original dentry is dput().

vfs_mkdir() 鐜板湪杩斿洖涓€涓?dentry鈥斺€斿嵆 ->mkdir() 鎵€杩斿洖鐨勯偅涓€傚鏋滆 dentry 涓庝紶鍏ョ殑
dentry 涓嶅悓锛堝寘鎷畠鏄?IS_ERR() dentry 鎸囬拡鐨勬儏鍐碉級锛屽師濮?dentry 浼氳 dput()銆?

When vfs_mkdir() returns an error, and so both dputs() the original
dentry and doesn't provide a replacement, it also unlocks the parent.
Consequently the return value from vfs_mkdir() can be passed to
end_creating() and the parent will be unlocked precisely when necessary.

褰?vfs_mkdir() 杩斿洖閿欒锛屼粠鑰屾棦 dput() 鍘熷 dentry 鍙堜笉鎻愪緵鏇夸唬鏃讹紝瀹冭繕浼氳В閿佺埗鐩綍銆?
鍥犳 vfs_mkdir() 鐨勮繑鍥炲€煎彲浠ヤ紶缁?end_creating()锛岃€岀埗鐩綍浼氬湪鎭板ソ蹇呰鏃惰瑙ｉ攣銆?

---


**mandatory**

**寮哄埗**

kill_litter_super() is gone; convert to DCACHE_PERSISTENT use (as all
in-tree filesystems have done).

kill_litter_super() 宸蹭笉瀛樺湪锛涜鏀圭敤 DCACHE_PERSISTENT锛堟濡傛墍鏈夋爲鍐呮枃浠剁郴缁熸墍鍋氱殑
閭ｆ牱锛夈€?

---


**mandatory**

**寮哄埗**

The ->setlease() file_operation must now be explicitly set in order to provide
support for leases. When set to NULL, the kernel will now return -EINVAL to
attempts to set a lease. Filesystems that wish to use the kernel-internal lease
implementation should set it to generic_setlease().

->setlease() 鏂囦欢鎿嶄綔鐜板湪蹇呴』琚樉寮忚缃紝浠ユ彁渚涘绉熺害锛坙ease锛夌殑鏀寔銆傚綋瀹冭璁句负
NULL 鏃讹紝鍐呮牳鐜板湪浼氬璁剧疆绉熺害鐨勫皾璇曡繑鍥?-EINVAL銆傚笇鏈涗娇鐢ㄥ唴鏍稿唴閮ㄧ绾﹀疄鐜扮殑鏂囦欢绯荤粺
搴斿皢鍏惰涓?generic_setlease()銆?

---


**mandatory**

**寮哄埗**

fs/namei.c primitives that consume filesystem references (do_renameat2(),
do_linkat(), do_symlinkat(), do_mkdirat(), do_mknodat(), do_unlinkat()
and do_rmdir()) are gone; they are replaced with non-consuming analogues
(filename_renameat2(), etc.)
Callers are adjusted - responsibility for dropping the filenames belongs
to them now.

fs/namei.c 涓秷璐规枃浠剁郴缁熷紩鐢ㄧ殑鍘熻锛坉o_renameat2()銆乨o_linkat()銆乨o_symlinkat()銆?
do_mkdirat()銆乨o_mknodat()銆乨o_unlinkat() 鍜?do_rmdir()锛夊凡涓嶅瓨鍦紱瀹冧滑琚潪娑堣垂鎬х殑
瀵瑰簲鐗╁彇浠ｏ紙filename_renameat2() 绛夛級銆傝皟鐢ㄦ柟宸茬浉搴旇皟鏁粹€斺€旂幇鍦ㄩ噴鏀炬枃浠跺悕鐨勮矗浠诲睘浜?
瀹冧滑銆?

---


**mandatory**

**寮哄埗**

readlink_copy() now requires link length as the 4th argument. Said length needs
to match what strlen() would return if it was ran on the string.

readlink_copy() 鐜板湪闇€瑕侀摼鎺ラ暱搴︿綔涓虹 4 涓弬鏁般€傝闀垮害闇€瑕佸尮閰嶅鏋滃璇ュ瓧绗︿覆杩愯
strlen() 浼氳繑鍥炵殑鍊笺€?

However, if the string is freely accessible for the duration of inode's
lifetime, consider using inode_set_cached_link() instead.

浣嗘槸锛屽鏋滆瀛楃涓插湪 inode 鐢熷懡鍛ㄦ湡鍐呭彲鑷敱璁块棶锛岃鑰冭檻鏀圭敤 inode_set_cached_link()銆?

---


**mandatory**

**寮哄埗**

lookup_one_qstr_excl() is no longer exported - use start_creating() or
similar.

lookup_one_qstr_excl() 涓嶅啀琚鍑衡€斺€旇浣跨敤 start_creating() 鎴栫被浼煎嚱鏁般€?

---


** mandatory**

**寮哄埗**

lock_rename(), lock_rename_child(), unlock_rename() are no
longer available.  Use start_renaming() or similar.

lock_rename()銆乴ock_rename_child()銆乽nlock_rename() 涓嶅啀鍙敤銆傝浣跨敤
start_renaming() 鎴栫被浼煎嚱鏁般€?

---


**recommended**

**寤鸿**

If you really need to iterate through dentries for given inode, use
for_each_alias(dentry, inode) instead of hlist_for_each_entry; better
yet, see if any of the exported primitives could be used instead of
the entire loop.  You still need to hold ->i_lock of the inode over
either form of manual loop.

濡傛灉浣犵‘瀹為渶瑕侀亶鍘嗙粰瀹?inode 鐨?dentries锛岃浣跨敤 for_each_alias(dentry, inode) 鑰岄潪
hlist_for_each_entry锛涙洿濂界殑鍋氭硶鏄紝鐪嬬湅鏄惁鑳界敤浠讳綍瀵煎嚭鐨勫師璇潵鍙栦唬鏁翠釜寰幆銆傛棤璁?
鍝褰㈠紡鐨勬墜鍔ㄥ惊鐜紝浣犱粛鐒堕渶瑕佸湪寰幆鏈熼棿鎸佹湁璇?inode 鐨?->i_lock銆?