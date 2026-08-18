
## 鏂囦欢绯荤粺鎸傝浇 API锛團ilesystem Mount API锛?

 (1) 姒傝堪銆?
 (2) 鏂囦欢绯荤粺涓婁笅鏂囷紙filesystem context锛夈€?
 (3) 鏂囦欢绯荤粺涓婁笅鏂囨搷浣溿€?
 (4) 鏂囦欢绯荤粺涓婁笅鏂囧畨鍏ㄣ€?
 (5) VFS 鏂囦欢绯荤粺涓婁笅鏂?API銆?
 (6) 瓒呯骇鍧楋紙superblock锛夊垱寤鸿緟鍔╁嚱鏁般€?
 (7) 鍙傛暟鎻忚堪銆?
 (8) 鍙傛暟杈呭姪鍑芥暟銆?

## 姒傝堪

鐜板湪锛屾柊鎸傝浇鐨勫垱寤鸿鍦ㄤ竴涓姝ラ鐨勮繃绋嬩腑瀹屾垚锛?
 (1) 鍒涘缓涓€涓枃浠剁郴缁熶笂涓嬫枃銆?
 (2) 瑙ｆ瀽鍙傛暟骞跺皢瀹冧滑闄勫姞鍒颁笂涓嬫枃銆傚弬鏁伴鏈熶粠鐢ㄦ埛绌洪棿閫愪釜浼犲叆锛屼笉杩囦篃鍙互澶勭悊浼犵粺鐨勪簩杩涘埗鍙傛暟銆?
 (3) 楠岃瘉骞堕澶勭悊涓婁笅鏂囥€?
 (4) 鑾峰彇鎴栧垱寤轰竴涓秴绾у潡浠ュ強鍙寕杞界殑鏍广€?
 (5) 鎵ц鎸傝浇銆?
 (6) 杩斿洖涓€鏉￠檮鍔犲埌涓婁笅鏂囩殑閿欒娑堟伅銆?
 (7) 閿€姣佷笂涓嬫枃銆?
```

	int (*init_fs_context)(struct fs_context *fc);
	const struct fs_parameter_description *parameters;

```
绗竴涓湪璁剧疆鏂囦欢绯荤粺涓婁笅鏂囩殑鏂囦欢绯荤粺鐩稿叧閮ㄥ垎鏃惰璋冪敤锛屽寘鎷澶栫殑绌洪棿锛涚浜屼釜鎸囧悜鍙傛暟鎻忚堪锛岀敤浜庡湪娉ㄥ唽鏃堕獙璇侊紝浠ュ強渚涙湭鏉ョ殑绯荤粺璋冪敤鏌ヨ銆?
娉ㄦ剰锛屽畨鍏ㄥ垵濮嬪寲鏄湪璋冪敤鏂囦欢绯荤粺**涔嬪悗**瀹屾垚鐨勶紝浠ヤ究鍙互棣栧厛璋冩暣鍛藉悕绌洪棿銆?

## 鏂囦欢绯荤粺涓婁笅鏂?
瓒呯骇鍧楃殑鍒涘缓涓庨噸鏂伴厤缃敱涓€涓枃浠剁郴缁?```

	struct fs_context {
		const struct fs_context_operations *ops;
		struct file_system_type *fs_type;
		void			*fs_private;
		struct dentry		*root;
		struct user_namespace	*user_ns;
		struct net		*net_ns;
		const struct cred	*cred;
		char			*source;
		char			*subtype;
		void			*security;
		void			*s_fs_info;
		unsigned int		sb_flags;
		unsigned int		sb_flags_mask;
		unsigned int		s_iflags;
		enum fs_context_purpose	purpose:8;
		...
	};

```
fs_context 鐨勫瓧娈靛涓嬶細

```

       const struct fs_context_operations *ops

     杩欎簺鏄彲浠ュ湪鏂囦欢绯荤粺涓婁笅鏂囦笂鎵ц鐨勬搷浣滐紙瑙佷笅鏂囷級銆傝繖蹇呴』鐢?->init_fs_context() file_system_type 鎿嶄綔璁剧疆銆?
   * ::

       struct file_system_type *fs_type

     鎸囧悜姝ｅ湪鏋勫缓鎴栭噸鏂伴厤缃殑鏂囦欢绯荤粺鐨?file_system_type 鐨勬寚閽堛€傝繖浼氫繚鐣欏绫诲瀷鎵€鏈夎€呯殑涓€涓紩鐢ㄣ€?
   * ::

       void *fs_private

     鎸囧悜鏂囦欢绯荤粺绉佹湁鏁版嵁鐨勬寚閽堛€傛枃浠剁郴缁熼渶瑕佸皢鍏惰В鏋愬嚭鐨勪换浣曢€夐」瀛樺偍鍦ㄨ繖閲屻€?
   * ::

       struct dentry *root

     鎸囧悜鍙寕杞芥爲鐨勬牴锛堜互鍙婇棿鎺ユ寚鍚戝叾瓒呯骇鍧楋級鐨勬寚閽堛€傝繖鐢?->get_tree() 鎿嶄綔濉厖銆傚鏋滆缃簡瀹冿紝涔熷繀椤绘寔鏈夊 root->d_sb 鐨勪竴涓椿鍔ㄥ紩鐢ㄣ€?
   * ::

       struct user_namespace *user_ns
       struct net *net_ns

     杩欐槸璋冪敤杩涚▼鎵€浣跨敤鐨勫懡鍚嶇┖闂寸殑涓€涓瓙闆嗐€傚畠浠繚鐣欏姣忎釜鍛藉悕绌洪棿鐨勫紩鐢ㄣ€傝闃呯殑鍛藉悕绌洪棿鍙兘琚枃浠剁郴缁熸浛鎹紝浠ュ弽鏄犲叾浠栨潵婧愶紝渚嬪鑷姩鎸傝浇锛坅utomount锛夋椂鐖舵寕杞界殑瓒呯骇鍧椼€?
   * ::

       const struct cred *cred

     鎸傝浇鑰呯殑鍑瘉銆傝繖淇濈暀瀵瑰嚟璇佺殑涓€涓紩鐢ㄣ€?
   * ::

       char *source

     杩欐寚瀹氫簡鏉ユ簮銆傚畠鍙互鏄竴涓潡璁惧锛堜緥濡?/dev/sda1锛夛紝鎴栦竴浜涙洿鐗规畩鐨勪笢瑗匡紝渚嬪 NFS 鎵€鏈熸湜鐨?"host:/path"銆?
   * ::

       char *subtype

     杩欐槸涓€涓娣诲姞鍒?/proc/mounts 涓樉绀虹殑绫诲瀷鐨勫瓧绗︿覆锛岀敤浜庨檺瀹氬畠锛堢敱 FUSE 浣跨敤锛夈€傚鏋滄枃浠剁郴缁熼渶瑕侊紝鍙互璁剧疆瀹冦€?
   * ::

       void *security

     LSM 鐢ㄦ潵鎸傛帴鍏惰秴绾у潡瀹夊叏鏁版嵁鐨勫湴鏂广€傜浉鍏崇殑瀹夊叏鎿嶄綔鍦ㄤ笅闈㈡弿杩般€?
   * ::

       void *s_fs_info

     涓烘柊鐨勮秴绾у潡寤鸿鐨?s_fs_info锛岀敱 sget_fc() 璁剧疆鍦ㄨ秴绾у潡涓€傝繖鍙敤浜庡尯鍒嗚秴绾у潡銆?
   * ::

       unsigned int sb_flags
       unsigned int sb_flags_mask

     瑕佸湪 super_block::s_flags 涓缃?娓呴櫎鍝簺 SB_* 鏍囧織浣嶃€?
   * ::

       unsigned int s_iflags

     杩欎簺灏嗗湪鍒涘缓瓒呯骇鍧楁椂涓?s->s_iflags 鍋氭寜浣嶆垨銆?
   * ::

       enum fs_context_purpose

     杩欒〃绀轰笂涓嬫枃鐨勭敤閫斻€傚彲鐢ㄧ殑鍊兼湁锛?
	==========================	======================================
	FS_CONTEXT_FOR_MOUNT,		New superblock for explicit mount
	FS_CONTEXT_FOR_SUBMOUNT		New automatic submount of extant mount
	FS_CONTEXT_FOR_RECONFIGURE	Change an existing mount
	==========================	======================================

```
鎸傝浇涓婁笅鏂囬€氳繃璋冪敤 vfs_new_fs_context() 鎴?vfs_dup_fs_context() 鍒涘缓锛屽苟閫氳繃 put_fs_context() 閿€姣併€傛敞鎰忚缁撴瀯娌℃湁琚紩鐢ㄨ鏁般€?
VFS銆佸畨鍏ㄥ拰鏂囦欢绯荤粺鐨勬寕杞介€夐」鐢?vfs_parse_mount_option() 閫愪釜璁剧疆銆傜敱鏃х殑 mount(2) 绯荤粺璋冪敤浣滀负涓€椤垫暟鎹彁渚涚殑閫夐」鍙互鐢?generic_parse_monolithic() 瑙ｆ瀽銆?
鍦ㄦ寕杞芥椂锛屾枃浠剁郴缁熻鍏佽浠庝换浣曟寚閽堜腑鍙栬蛋鏁版嵁骞跺皢鍏堕檮鍔犲埌瓒呯骇鍧楋紙鎴栧叾浠栦粈涔堬級锛屽墠鎻愭槸瀹冩竻闄や簡鎸傝浇涓婁笅鏂囦腑鐨勬寚閽堛€?
鏂囦欢绯荤粺涔熻鍏佽鍒嗛厤璧勬簮骞剁敤鎸傝浇涓婁笅鏂囧浐瀹氬畠浠€備緥濡傦紝NFS 鍙兘浼氬浐瀹氱浉搴旂殑鍗忚鐗堟湰妯″潡銆?

## 鏂囦欢绯荤粺涓婁笅鏂囨搷浣?
```

	struct fs_context_operations {
		void (*free)(struct fs_context *fc);
		int (*dup)(struct fs_context *fc, struct fs_context *src_fc);
		int (*parse_param)(struct fs_context *fc,
				   struct fs_parameter *param);
		int (*parse_monolithic)(struct fs_context *fc, void *data);
		int (*get_tree)(struct fs_context *fc);
		int (*reconfigure)(struct fs_context *fc);
	};

```
杩欎簺鎿嶄綔鍦ㄦ寕杞借繃绋嬬殑鍚勪釜闃舵琚皟鐢ㄦ潵绠＄悊鏂囦欢绯荤粺涓婁笅鏂囥€傚畠浠涓嬶細

```

	void (*free)(struct fs_context *fc);

     褰撲笂涓嬫枃琚攢姣佹椂璋冪敤锛岀敤浜庢竻鐞嗘枃浠剁郴缁熶笂涓嬫枃鐨勬枃浠剁郴缁熺浉鍏抽儴鍒嗐€傚畠搴斿綋鎰忚瘑鍒颁笂涓嬫枃鐨勬煇浜涢儴鍒嗗彲鑳藉凡琚Щ闄ゅ苟琚涓?NULL锛堢敱 ->get_tree() 瀹屾垚锛夈€?
   * ::

	int (*dup)(struct fs_context *fc, struct fs_context *src_fc);

     褰撴枃浠剁郴缁熶笂涓嬫枃琚鍒舵椂璋冪敤锛屼互澶嶅埗鏂囦欢绯荤粺绉佹湁鏁版嵁銆傚彲浠ヨ繑鍥炰竴涓敊璇潵鎸囩ず澶嶅埗澶辫触銆?
     .. Warning::

         娉ㄦ剰锛屽嵆浣胯繖澶辫触浜嗭紝put_fs_context() 涔熶細绱ф帴鍏跺悗琚皟鐢紝鍥犳 ->dup() *蹇呴』* 璁╂枃浠剁郴缁熺鏈夋暟鎹 ->free() 鏄畨鍏ㄧ殑銆?
   * ::

	int (*parse_param)(struct fs_context *fc,
			   struct fs_parameter *param);

     褰撳悜鏂囦欢绯荤粺涓婁笅鏂囨坊鍔犲弬鏁版椂璋冪敤銆俻aram 鎸囧悜閿悕锛屽彲鑳借繕鏈変竴涓€煎璞°€俈FS 鐩稿叧鐨勯€夐」灏嗗凡琚墧闄わ紝骞朵笖 fc->sb_flags 宸插湪涓婁笅鏂囦腑鏇存柊銆傚畨鍏ㄩ€夐」涔熷皢宸茶鍓旈櫎锛屽苟涓?fc->security 宸叉洿鏂般€?
     鍙傛暟鍙互鐢?fs_parse() 鍜?fs_lookup_param() 鏉ヨВ鏋愩€傛敞鎰忔潵婧愶紙source锛夋槸浣滀负鍚嶄负 "source" 鐨勫弬鏁板憟鐜扮殑銆?
     濡傛灉鎴愬姛锛屽簲杩斿洖 0锛屽惁鍒欒繑鍥炰竴涓礋鐨勯敊璇爜銆?
   * ::

	int (*parse_monolithic)(struct fs_context *fc, void *data);

     褰撹皟鐢?mount(2) 绯荤粺璋冪敤浠ヤ竴娆℃€т紶鍏ユ暣涓暟鎹〉鏃惰皟鐢ㄣ€傚鏋滈鏈熻繖鍙槸涓€涓敱閫楀彿鍒嗛殧鐨?"key[=val]" 鏉＄洰鍒楄〃锛岄偅涔堝彲浠ュ皢鍏惰涓?NULL銆?
     杩斿洖鍊间笌 ->parse_param() 鐩稿悓銆?
     濡傛灉鏂囦欢绯荤粺锛堜緥濡?NFS锛夐渶瑕佸厛妫€鏌ユ暟鎹紝鐒跺悗鍙戠幇瀹冩槸鏍囧噯鐨勯敭-鍊煎垪琛紝閭ｄ箞瀹冨彲浠ヨ浆浜ょ粰 generic_parse_monolithic()銆?
   * ::

	int (*get_tree)(struct fs_context *fc);

     璋冪敤浠ヨ幏鍙栨垨鍒涘缓鍙寕杞界殑鏍逛笌瓒呯骇鍧楋紝浣跨敤瀛樺偍鍦ㄦ枃浠剁郴缁熶笂涓嬫枃涓殑淇℃伅锛堥噸鏂伴厤缃€氳繃涓€涓笉鍚岀殑鍚戦噺杩涜锛夈€傚畠鍙互灏嗗叾鎯宠鐨勪换浣曡祫婧愪粠鏂囦欢绯荤粺涓婁笅鏂囧垎绂伙紝骞惰浆绉诲埌瀹冨垱寤虹殑瓒呯骇鍧椾笂銆?
     鎴愬姛鏃跺畠搴斿皢 fc->root 璁剧疆涓哄彲鎸傝浇鐨勬牴骞惰繑鍥?0銆傚湪鍑洪敊鐨勬儏鍐典笅锛屽畠搴旇繑鍥炰竴涓礋鐨勯敊璇爜銆?
     鍦ㄧ敤鎴风┖闂撮┍鍔ㄧ殑涓婁笅鏂囦笂锛岃闃舵浼氳璁剧疆涓哄彧鍏佽鍦ㄤ换浣曠壒瀹氫笂涓嬫枃涓婅皟鐢ㄤ竴娆°€?
   * ::

	int (*reconfigure)(struct fs_context *fc);

     璋冪敤浠ヤ娇鐢ㄦ枃浠剁郴缁熶笂涓嬫枃涓瓨鍌ㄧ殑淇℃伅鏉ュ疄鏂借秴绾у潡鐨勯噸鏂伴厤缃€傚畠鍙互灏嗗叾鎯宠鐨勪换浣曡祫婧愪粠鏂囦欢绯荤粺涓婁笅鏂囧垎绂伙紝骞惰浆绉诲埌瓒呯骇鍧椼€傝秴绾у潡鍙互浠?fc->root->d_sb 鎵惧埌銆?
     鎴愬姛鏃跺畠搴旇繑鍥?0銆傚湪鍑洪敊鐨勬儏鍐典笅锛屽畠搴旇繑鍥炰竴涓礋鐨勯敊璇爜銆?

```
## 鏂囦欢绯荤粺涓婁笅鏂囧畨鍏?
鏂囦欢绯荤粺涓婁笅鏂囧寘鍚竴涓畨鍏ㄦ寚閽堬紝LSM 鍙互鐢ㄥ畠鏉ヤ负瑕佹寕杞界殑瓒呯骇鍧楁瀯寤哄畨鍏ㄤ笂涓嬫枃銆傛柊鐨勬寕杞戒唬鐮佷负姝ょ洰鐨勪娇鐢ㄤ簡鑻ュ共鎿嶄綔锛?
```

	int security_fs_context_alloc(struct fs_context *fc,
				      struct dentry *reference);

     璋冪敤浠ュ垵濮嬪寲 fc->security锛堝畠琚璁句负 NULL锛夊苟鍒嗛厤鎵€闇€璧勬簮銆傛垚鍔熷簲杩斿洖 0锛屽け璐ヨ繑鍥炶礋鐨勯敊璇爜銆?
     reference 鍦ㄤ笂涓嬫枃鏄负瓒呯骇鍧楅噸鏂伴厤缃紙FS_CONTEXT_FOR_RECONFIGURE锛夎€屽垱寤烘椂涓洪潪 NULL锛屾鏃跺畠鎸囧悜瑕侀噸鏂伴厤缃殑瓒呯骇鍧楃殑鏍?dentry銆傚湪瀛愭寕杞斤紙FS_CONTEXT_FOR_SUBMOUNT锛夌殑鎯呭喌涓嬪畠涔熶负闈?NULL锛屾鏃跺畠鎸囧悜鑷姩鎸傝浇鐐广€?
   * ::

	int security_fs_context_dup(struct fs_context *fc,
				    struct fs_context *src_fc);

     璋冪敤浠ュ垵濮嬪寲 fc->security锛堝畠琚璁句负 NULL锛夊苟鍒嗛厤鎵€闇€璧勬簮銆傚師濮嬬殑鏂囦欢绯荤粺涓婁笅鏂囩敱 src_fc 鎸囧悜锛屽彲鐢ㄦ潵鍙傝€冦€傛垚鍔熷簲杩斿洖 0锛屽け璐ヨ繑鍥炶礋鐨勯敊璇爜銆?
   * ::

	void security_fs_context_free(struct fs_context *fc);

     璋冪敤浠ユ竻鐞嗛檮鍔犲埌 fc->security 鐨勪换浣曞唴瀹广€傛敞鎰忓叾鍐呭鍙兘宸茶杞Щ鍒拌秴绾у潡锛屽苟涓旀寚閽堝湪 get_tree 鏈熼棿琚竻绌恒€?
   * ::

	int security_fs_context_parse_param(struct fs_context *fc,
					    struct fs_parameter *param);

     涓烘瘡涓寕杞藉弬鏁帮紙鍖呮嫭鏉ユ簮锛夎皟鐢ㄣ€傚弬鏁颁笌 ->parse_param() 鏂规硶鐩稿悓銆傚簲杩斿洖 0 琛ㄧず璇ュ弬鏁板簲琚紶閫掔粰鏂囦欢绯荤粺锛岃繑鍥?1 琛ㄧず璇ュ弬鏁板簲琚涪寮冿紝鎴栬繑鍥炰竴涓敊璇互琛ㄧず璇ュ弬鏁板簲琚嫆缁濄€?
     param 鎸囧悜鐨勫€煎彲鑳借淇敼锛堝鏋滄槸瀛楃涓诧級鎴栬绐冨彇锛堝墠鎻愭槸鍊兼寚閽堣璁句负 NULL锛夈€傚鏋滆绐冨彇锛屽繀椤昏繑鍥?1 浠ラ槻姝㈠畠琚紶閫掔粰鏂囦欢绯荤粺銆?
   * ::

	int security_fs_context_validate(struct fs_context *fc);

     鍦ㄦ墍鏈夐€夐」閮借瑙ｆ瀽涔嬪悗璋冪敤锛屼互鏁翠綋楠岃瘉杩欎竴闆嗗悎锛屽苟杩涜浠讳綍蹇呰鐨勫垎閰嶏紝浣垮緱 security_sb_get_tree() 鍜?security_sb_reconfigure() 涓嶅お鍙兘澶辫触銆傚簲杩斿洖 0 鎴栬礋鐨勯敊璇爜銆?
     鍦ㄩ噸鏂伴厤缃殑鎯呭喌涓嬶紝鐩爣瓒呯骇鍧楀彲浠ラ€氳繃 fc->root 璁块棶銆?
   * ::

	int security_sb_get_tree(struct fs_context *fc);

     鍦ㄦ寕杞借繃绋嬩腑璋冪敤锛屼互楠岃瘉鎸囧畾鐨勮秴绾у潡鏄惁琚厑璁告寕杞斤紝骞跺皢瀹夊叏鏁版嵁杞Щ鍒伴偅閲屻€傚簲杩斿洖 0 鎴栬礋鐨勯敊璇爜銆?
   * ::

	void security_sb_reconfigure(struct fs_context *fc);

     璋冪敤浠ュ皢浠讳綍閲嶆柊閰嶇疆搴旂敤鍒?LSM 鐨勪笂涓嬫枃銆傚畠缁濅笉鑳藉け璐ャ€傞敊璇鏌ュ拰璧勬簮鍒嗛厤蹇呴』鐢卞弬鏁拌В鏋愬拰楠岃瘉閽╁瓙鎻愬墠瀹屾垚銆?
   * ::

	int security_sb_mountpoint(struct fs_context *fc,
			           struct path *mountpoint,
				   unsigned int mnt_flags);

     鍦ㄦ寕杞借繃绋嬩腑璋冪敤锛屼互楠岃瘉闄勫姞鍒颁笂涓嬫枃鐨勬牴 dentry 鏄惁鍏佽琚檮鍔犲埌鎸囧畾鐨勬寕杞界偣銆傛垚鍔熷簲杩斿洖 0锛屽け璐ヨ繑鍥炶礋鐨勯敊璇爜銆?

```
## VFS 鏂囦欢绯荤粺涓婁笅鏂?API

鏈夊洓涓搷浣滅敤浜庡垱寤烘枃浠剁郴缁熶笂涓嬫枃锛屼竴涓敤浜庨攢姣佷笂涓嬫枃锛?
```

       struct fs_context *fs_context_for_mount(struct file_system_type *fs_type,
					       unsigned int sb_flags);

     鍒嗛厤涓€涓枃浠剁郴缁熶笂涓嬫枃锛岀敤浜庤缃竴涓柊鐨勬寕杞斤紝鏃犺鏄娇鐢ㄦ柊鐨勮秴绾у潡杩樻槸鍏变韩宸叉湁鐨勮秴绾у潡銆傝繖浼氳缃秴绾у潡鏍囧織锛屽垵濮嬪寲瀹夊叏锛屽苟璋冪敤 fs_type->init_fs_context() 鏉ュ垵濮嬪寲鏂囦欢绯荤粺绉佹湁鏁版嵁銆?
     fs_type 鎸囧畾绠＄悊璇ヤ笂涓嬫枃鐨勬枃浠剁郴缁熺被鍨嬶紝sb_flags 棰勮鍏朵腑瀛樺偍鐨勮秴绾у潡鏍囧織銆?
   * ::

       struct fs_context *fs_context_for_reconfigure(
		struct dentry *dentry,
		unsigned int sb_flags,
		unsigned int sb_flags_mask);

     鍒嗛厤涓€涓枃浠剁郴缁熶笂涓嬫枃锛岀敤浜庨噸鏂伴厤缃竴涓凡鏈夌殑瓒呯骇鍧椼€俤entry 鎻愪緵瀵硅閰嶇疆鐨勮秴绾у潡鐨勫紩鐢ㄣ€俿b_flags 鍜?sb_flags_mask 鎸囨槑鍝簺瓒呯骇鍧楁爣蹇楅渶瑕佹敼鍙樹互鍙婃敼鎴愪粈涔堛€?
   * ::

       struct fs_context *fs_context_for_submount(
		struct file_system_type *fs_type,
		struct dentry *reference);

     鍒嗛厤涓€涓枃浠剁郴缁熶笂涓嬫枃锛岀敤浜庝负鑷姩鎸傝浇鐐规垨鍏朵粬娲剧敓鐨勮秴绾у潡鍒涘缓涓€涓柊鐨勬寕杞姐€俧s_type 鎸囧畾绠＄悊璇ヤ笂涓嬫枃鐨勬枃浠剁郴缁熺被鍨嬶紝reference dentry 鎻愪緵鍙傛暟銆傚懡鍚嶇┖闂翠篃浠?reference dentry 鐨勮秴绾у潡浼犳挱銆?
     娉ㄦ剰锛屼笉瑕佹眰 reference dentry 涓?fs_type 灞炰簬鐩稿悓鐨勬枃浠剁郴缁熺被鍨嬨€?
   * ::

        struct fs_context *vfs_dup_fs_context(struct fs_context *src_fc);

     澶嶅埗涓€涓枃浠剁郴缁熶笂涓嬫枃锛屽鍒跺叾涓褰曠殑浠讳綍閫夐」锛屽苟澶嶅埗鎴栭澶栧紩鐢ㄥ叾涓寔鏈夌殑浠讳綍璧勬簮銆傝繖鍙敤浜庢枃浠剁郴缁熷繀椤诲湪鎸傝浇鍐呭啀杩涜鎸傝浇鐨勬儏鍐碉紝渚嬪 NFS4 閫氳繃鍐呴儴鎸傝浇鐩爣鏈嶅姟鍣ㄧ殑鏍癸紝鐒跺悗鍋氫竴娆＄鏈夌殑璺緞閬嶅巻锛坧athwalk锛夊埌杈剧洰鏍囩洰褰曘€?
     鏂颁笂涓嬫枃涓殑 purpose 浠庢棫鐨勭户鎵胯€屾潵銆?
   * ::

       void put_fs_context(struct fs_context *fc);

     閿€姣佷竴涓枃浠剁郴缁熶笂涓嬫枃锛岄噴鏀惧畠鎸佹湁鐨勪换浣曡祫婧愩€傝繖浼氳皟鐢?->free() 鎿嶄綔銆傝繖棰勬湡鐢变换浣曞垱寤轰簡鏂囦欢绯荤粺涓婁笅鏂囩殑浜鸿皟鐢ㄣ€?
     .. Warning::

        鏂囦欢绯荤粺涓婁笅鏂囨病鏈夎寮曠敤璁℃暟锛屽洜姝よ繖浼氬鑷存棤鏉′欢鐨勯攢姣併€?
```
鍦ㄦ墍鏈変笂杩版搷浣滀腑锛岄櫎浜?put 鎿嶄綔涔嬪锛岃繑鍥炵殑鏄竴涓寕杞戒笂涓嬫枃鎸囬拡鎴栦竴涓礋鐨勯敊璇爜銆?
瀵逛簬鍏朵綑鐨勬搷浣滐紝濡傛灉鍙戠敓閿欒锛屽皢杩斿洖涓€涓礋鐨勯敊璇爜銆?
```

        int vfs_parse_fs_param(struct fs_context *fc,
			       struct fs_parameter *param);

     鍚戞枃浠剁郴缁熶笂涓嬫枃鎻愪緵鍗曚釜鎸傝浇鍙傛暟銆傝繖鍖呮嫭鏉ユ簮/璁惧鐨勬寚瀹氾紝瀹冧綔涓?"source" 鍙傛暟鎸囧畾锛堝鏋滄枃浠剁郴缁熸敮鎸侊紝鍙互澶氭鎸囧畾锛夈€?
     param 鎸囧畾鍙傛暟閿悕鍜屽€笺€傝鍙傛暟浼氬厛琚鏌ワ紝鐪嬪畠鏄惁瀵瑰簲涓€涓爣鍑嗙殑鎸傝浇鏍囧織锛堣繖绉嶆儏鍐典笅鐢ㄤ簬璁剧疆涓€涓?SB_xxx 鏍囧織骞惰娑堣垂锛夋垨涓€涓畨鍏ㄩ€夐」锛堣繖绉嶆儏鍐典笅鐢?LSM 娑堣垂锛夛紝鐒跺悗鎵嶈浼犻€掔粰鏂囦欢绯荤粺銆?
     鍙傛暟鍊兼槸甯︾被鍨嬬殑锛屽彲浠ユ槸浠ヤ笅涔嬩竴锛?
	====================		=============================
	fs_value_is_flag		Parameter not given a value
	fs_value_is_string		Value is a string
	fs_value_is_blob		Value is a binary blob
	fs_value_is_filename		Value is a filename* + dirfd
	fs_value_is_file		Value is an open file (file*)
	====================		=============================

     濡傛灉鏈変竴涓€硷紝璇ュ€煎瓨鍌ㄥ湪 struct 鐨勪竴涓仈鍚堜綋涓殑 param->{string,blob,name,file} 涔嬩竴閲屻€傛敞鎰忚鍑芥暟鍙兘浼氱獌鍙栧苟娓呯┖璇ユ寚閽堬紝浣嗛殢鍚庤璐熻矗澶勭疆璇ュ璞°€?
   * ::

       int vfs_parse_fs_qstr(struct fs_context *fc, const char *key,
			       const struct qstr *value);

     vfs_parse_fs_param() 鐨勪竴涓寘瑁咃紝浼氬鍒朵紶缁欏畠鐨?value 瀛楃涓层€?
   * ::

       int vfs_parse_fs_string(struct fs_context *fc, const char *key,
			       const char *value);

     vfs_parse_fs_param() 鐨勪竴涓寘瑁咃紝浼氬鍒朵紶缁欏畠鐨?value 瀛楃涓层€?
   * ::

       int generic_parse_monolithic(struct fs_context *fc, void *data);

     瑙ｆ瀽 sys_mount() 鐨勬暟鎹〉锛屽亣璁惧叾褰㈠紡涓虹敱閫楀彿鍒嗛殧鐨勭敱 key[=val] 閫夐」缁勬垚鐨勬枃鏈垪琛ㄣ€傚垪琛ㄤ腑鐨勬瘡涓€椤归兘琚紶缁?vfs_mount_option()銆傚綋 ->parse_monolithic() 鏂规硶涓?NULL 鏃惰繖鏄粯璁よ涓恒€?
   * ::

       int vfs_get_tree(struct fs_context *fc);

     鑾峰彇鎴栧垱寤哄彲鎸傝浇鐨勬牴涓庤秴绾у潡锛屼娇鐢ㄦ枃浠剁郴缁熶笂涓嬫枃涓殑鍙傛暟鏉ラ€夋嫨/閰嶇疆瓒呯骇鍧椼€傝繖浼氳皟鐢?->get_tree() 鏂规硶銆?
   * ::

       struct vfsmount *vfs_create_mount(struct fs_context *fc);

     鏍规嵁缁欏畾鐨勬枃浠剁郴缁熶笂涓嬫枃涓殑鍙傛暟鍒涘缓涓€涓寕杞姐€傛敞鎰忚繖涓嶄細灏嗘寕杞介檮鍔犲埌浠讳綍涓滆タ涓娿€?

```
## 瓒呯骇鍧楀垱寤鸿緟鍔╁嚱鏁?
VFS 鎻愪緵浜嗚嫢骞茶緟鍔╁嚱鏁颁緵鏂囦欢绯荤粺鍦ㄥ垱寤烘垨鏌ユ壘瓒呯骇鍧楁椂浣跨敤銆?
```

       struct super_block *
       sget_fc(struct fs_context *fc,
	       int (*test)(struct super_block *sb, struct fs_context *fc),
	       int (*set)(struct super_block *sb, struct fs_context *fc));

     杩欐槸鏍稿績渚嬬▼銆傚鏋?test 涓洪潪 NULL锛屽畠浼氫娇鐢?test 鍑芥暟鍦?fs_context 涓悳绱㈠尮閰嶆潯浠剁殑宸叉湁瓒呯骇鍧椼€傚鏋滄病鎵惧埌鍖归厤椤癸紝灏卞垱寤轰竴涓柊鐨勮秴绾у潡锛屽苟璋冪敤 set 鍑芥暟鏉ヨ缃畠銆?
     鍦ㄨ皟鐢?set 鍑芥暟涔嬪墠锛宖c->s_fs_info 灏嗚杞Щ鍒?sb->s_fs_info鈥斺€斿苟涓斿鏋?set 杩斿洖鎴愬姛锛堝嵆 0锛夛紝fc->s_fs_info 灏嗚娓呯┖銆?
```
浠ヤ笅杈呭姪鍑芥暟閮藉寘瑁呬簡 sget_fc()锛?
	(1) vfs_get_single_super

	    绯荤粺涓彧鑳藉瓨鍦ㄨ繖鏍蜂竴涓秴绾у潡銆備换浣曡繘涓€姝ヨ幏鍙栨柊瓒呯骇鍧楃殑灏濊瘯閮戒細寰楀埌杩欎竴涓紙骞朵笖浠讳綍鍙傛暟宸紓閮戒細琚拷鐣ワ級銆?
	(2) vfs_get_keyed_super

	    鍙兘瀛樺湪澶氫釜姝ょ被鍨嬬殑瓒呯骇鍧楋紝瀹冧滑浠ュ悇鑷殑 s_fs_info 鎸囬拡浣滀负閿紙渚嬪杩欏彲鑳芥寚鍚戜竴涓懡鍚嶇┖闂达級銆?
	(3) vfs_get_independent_super

	    鍙兘瀛樺湪澶氫釜鐙珛鐨勬绫昏秴绾у潡銆傝鍑芥暟浠庝笉鍖归厤宸叉湁鐨勪竴涓紝鎬绘槸鍒涘缓涓€涓柊鐨勩€?

## 鍙傛暟鎻忚堪

鍙傛暟浣跨敤 linux/fs_parser.h 涓畾涔夌殑缁撴瀯鏉ユ弿杩般€?```

	struct fs_parameter_description {
		const struct fs_parameter_spec *specs;
		const struct fs_parameter_enum *enums;
	};

```
```

	enum {
		Opt_autocell,
		Opt_bar,
		Opt_dyn,
		Opt_foo,
		Opt_source,
	};

	static const struct fs_parameter_description afs_fs_parameters = {
		.specs		= afs_param_specs,
		.enums		= afs_param_enums,
	};

```
鍏舵垚鍛樺涓嬶細

```

       const struct fs_parameter_specification *specs;

     鍙傛暟瑙勬牸琛紝浠ヤ竴涓┖鏉＄洰缁堟锛屽叾涓殑鏉＄洰绫诲瀷涓?:

	struct fs_parameter_spec {
		const char		*name;
		u8			opt;
		enum fs_parameter_type	type:8;
		unsigned short		flags;
	};

     'name' 瀛楁鏄竴涓涓庡弬鏁伴敭绮剧‘鍖归厤鐨勫瓧绗︿覆锛堜笉鏀寔閫氶厤绗︺€佹ā寮忥紝涔熶笉鍖哄垎澶у皬鍐欙級锛?opt' 鏄?fs_parser() 鍑芥暟鍦ㄦ垚鍔熷尮閰嶇殑鎯呭喌涓嬭繑鍥炵殑鍊笺€?
     'type' 瀛楁鎸囨槑鏈熸湜鐨勫€肩被鍨嬶紝蹇呴』鏄互涓嬩箣涓€锛?
	=======================	=======================	=====================
	TYPE NAME		EXPECTED VALUE		RESULT IN
	=======================	=======================	=====================
	fs_param_is_flag	No value		n/a
	fs_param_is_bool	Boolean value		result->boolean
	fs_param_is_u32		32-bit unsigned int	result->uint_32
	fs_param_is_u32_octal	32-bit octal int	result->uint_32
	fs_param_is_u32_hex	32-bit hex int		result->uint_32
	fs_param_is_s32		32-bit signed int	result->int_32
	fs_param_is_u64		64-bit unsigned int	result->uint_64
	fs_param_is_enum	Enum value name 	result->uint_32
	fs_param_is_string	Arbitrary string	param->string
	fs_param_is_blockdev	Blockdev path		* Needs lookup
	fs_param_is_fd		File descriptor		result->int_32
	fs_param_is_uid		User ID (u32)           result->uid
	fs_param_is_gid		Group ID (u32)          result->gid
	=======================	=======================	=====================

     娉ㄦ剰锛屽鏋滃€肩殑绫诲瀷鏄?fs_param_is_bool锛宖s_parse() 浼氬皾璇曞皢浠讳綍瀛楃涓插€间笌 "0"銆?1"銆?no"銆?yes"銆?false"銆?true" 鍖归厤銆?
     姣忎釜鍙傛暟杩樺彲浠ョ敤 'flags' 闄愬畾锛?
	=======================	================================================
	fs_param_v_optional	The value is optional
	fs_param_neg_with_no	result->negated set if key is prefixed with "no"
	fs_param_neg_with_empty	result->negated set if value is ""
	fs_param_deprecated	The parameter is deprecated.
	=======================	================================================

     瀹冧滑鐢辫澶氫究鍒╁畯鍖呰锛?
	=======================	===============================================
	MACRO			SPECIFIES
	=======================	===============================================
	fsparam_flag()		fs_param_is_flag
	fsparam_flag_no()	fs_param_is_flag, fs_param_neg_with_no
	fsparam_bool()		fs_param_is_bool
	fsparam_u32()		fs_param_is_u32
	fsparam_u32oct()	fs_param_is_u32_octal
	fsparam_s32()		fs_param_is_s32
	fsparam_u64()		fs_param_is_u64
	fsparam_enum()		fs_param_is_enum
	fsparam_string()	fs_param_is_string
	fsparam_bdev()		fs_param_is_blockdev
	fsparam_fd()		fs_param_is_fd
	fsparam_uid()		fs_param_is_uid
	fsparam_gid()		fs_param_is_gid
	=======================	===============================================

     浠ヤ笂鍏ㄩ儴鍙栦袱涓弬鏁帮細name 瀛楃涓插拰閫夐」缂栧彿鈥斺€斾緥濡?:

	static const struct fs_parameter_spec afs_param_specs[] = {
		fsparam_flag	("autocell",	Opt_autocell),
		fsparam_flag	("dyn",		Opt_dyn),
		fsparam_string	("source",	Opt_source),
		fsparam_flag_no	("foo",		Opt_foo),
		{}
	};

     杩樻彁渚涗簡涓€涓澶栫殑瀹?__fsparam()锛屽畠鍙栭澶栫殑涓€瀵瑰弬鏁版潵涓轰笉鍖归厤涓婅堪浠讳綍瀹忕殑鎯呭喌鎸囧畾绫诲瀷鍜屾爣蹇椼€?
 (2) ::

       const struct fs_parameter_enum *enums;

     鏋氫妇鍊煎悕鍒版暣鏁扮殑鏄犲皠琛紝浠ヤ竴涓┖鏉＄洰缁堟銆傚叾绫诲瀷涓?:

	struct fs_parameter_enum {
		u8		opt;
		char		name[14];
		u8		value;
	};

     璇ユ暟缁勬槸涓€涓互 { 鍙傛暟 ID, name } 涓洪敭鐨勬湭鎺掑簭鍏冪礌鍒楄〃锛屾寚绀鸿鏄犲皠鍒扮殑 value锛屼緥濡?:

	static const struct fs_parameter_enum afs_param_enums[] = {
		{ Opt_bar,   "x",      1},
		{ Opt_bar,   "y",      23},
		{ Opt_bar,   "z",      42},
	};

     濡傛灉閬囧埌 fs_param_is_enum 绫诲瀷鐨勫弬鏁帮紝fs_parse() 浼氬皾璇曞湪鏋氫妇琛ㄤ腑鏌ユ壘璇ュ€硷紝缁撴灉灏嗗瓨鍌ㄥ湪瑙ｆ瀽缁撴灉涓€?
```
瑙ｆ瀽鍣ㄥ簲鐢?file_system_type 缁撴瀯涓殑 parser 鎸囬拡鎸囧悜锛屽洜涓鸿繖灏嗘彁渚涙敞鍐屾椂鐨勯獙璇侊紙濡傛灉 CONFIG_VALIDATE_FS_PARSER=y锛夛紝骞跺皢鍏佽閫氳繃 fsinfo() 绯荤粺璋冪敤浠庣敤鎴风┖闂存煡璇㈣鎻忚堪銆?

## 鍙傛暟杈呭姪鍑芥暟

鎻愪緵浜嗚嫢骞茶緟鍔╁嚱鏁版潵甯姪鏂囦欢绯荤粺鎴?LSM 澶勭悊瀹冩墍鑾峰緱鐨勫弬鏁般€?
```

       int lookup_constant(const struct constant_table tbl[],
			   const char *name, int not_found);

     鍦ㄢ€滃悕瀛?-> 鏁存暟鈥濇槧灏勮〃涓寜鍚嶅瓧鏌ユ壘涓€涓父閲忋€傝琛ㄦ槸涓€涓厓绱犵被鍨嬩负濡備笅鐨勭粨鏋勭殑鏁扮粍::

	struct constant_table {
		const char	*name;
		int		value;
	};

     濡傛灉鎵惧埌鍖归厤锛岃繑鍥炲搴旂殑鍊笺€傚鏋滄病鎵惧埌鍖归厤锛屽垯鏀逛负杩斿洖 not_found 鍊笺€?
   * ::

       bool fs_validate_description(const char *name,
                                    const struct fs_parameter_description *desc);

     杩欏鍙傛暟鎻忚堪鎵ц涓€浜涢獙璇佹鏌ャ€傚鏋滄弿杩拌壇濂藉垯杩斿洖 true锛屽惁鍒欒繑鍥?false銆傚鏋滈獙璇佸け璐ワ紝瀹冧細灏嗛敊璇褰曞埌鍐呮牳鏃ュ織缂撳啿鍖恒€?
   * ::

        int fs_parse(struct fs_context *fc,
		     const struct fs_parameter_description *desc,
		     struct fs_parameter *param,
		     struct fs_parse_result *result);

     杩欐槸鍙傛暟鐨勪富瑙ｉ噴鍣ㄣ€傚畠浣跨敤鍙傛暟鎻忚堪閫氳繃閿悕鏌ユ壘鍙傛暟锛屽苟灏嗗叾杞崲涓轰竴涓€夐」缂栧彿锛堝畠杩斿洖璇ョ紪鍙凤級銆?
     濡傛灉鎴愬姛锛屽苟涓斿鏋滃弬鏁扮被鍨嬫寚绀虹粨鏋滄槸甯冨皵銆佹暣鏁般€佹灇涓俱€乽id 鎴?gid 绫诲瀷锛岃鍊间細琚鍑芥暟杞崲锛岀粨鏋滃瓨鍌ㄥ湪 result->{boolean,int_32,uint_32,uint_64,uid,gid} 涓€?
     濡傛灉鏈€鍒濇病鏈夊尮閰嶏紝浣嗛敭甯︽湁 "no" 鍓嶇紑涓旀病鏈夊€硷紝鍒欎細灏濊瘯鐢ㄥ幓鎺夊墠缂€鐨勯敭鍘绘煡鎵俱€傚鏋滆繖鍖归厤鍒颁竴涓被鍨嬪甫鏈?fs_param_neg_with_no 鏍囧織鐨勫弬鏁帮紝鍒欎細褰㈡垚鍖归厤锛屽苟涓?result->negated 浼氳璁句负 true銆?
     濡傛灉鍙傛暟涓嶅尮閰嶏紝灏嗚繑鍥?-ENOPARAM锛涘鏋滃弬鏁板尮閰嶄絾鍊兼湁璇紝灏嗚繑鍥?-EINVAL锛涘惁鍒欎細杩斿洖璇ュ弬鏁扮殑閫夐」缂栧彿銆?
   * ::

       int fs_lookup_param(struct fs_context *fc,
			   struct fs_parameter *value,
			   bool want_bdev,
			   unsigned int flags,
			   struct path *_path);

     杩欐帴鍙椾竴涓惡甯﹀瓧绗︿覆鎴栨枃浠跺悕绫诲瀷鐨勫弬鏁帮紝骞跺皾璇曞鍏跺仛璺緞鏌ユ壘銆傚鏋滃弬鏁版湡鏈涗竴涓潡璁惧锛屽垯浼氭鏌ヨ inode 鏄惁纭疄浠ｈ〃涓€涓潡璁惧銆?
     鎴愬姛鏃惰繑鍥?0锛屽苟涓?``*_path`` 浼氳璁剧疆锛涘惁鍒欒繑鍥炰竴涓礋鐨勯敊璇爜銆?
```
