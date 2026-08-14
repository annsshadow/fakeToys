
## DebugFS


Copyright |copy| 2009 Jonathan Corbet <corbet@lwn.net>

Debugfs 浣滀负涓€绉嶇畝鍗曠殑鏂瑰紡瀛樺湪锛岃鍐呮牳寮€鍙戣€呰兘澶熷悜鐢ㄦ埛绌洪棿鎻愪緵淇℃伅銆備笌浠呯敤浜庢彁渚?杩涚▼鐩稿叧淇℃伅鐨?/proc锛屾垨鑰呮湁鐫€涓ユ牸"姣忎釜鏂囦欢涓€涓€?瑙勫垯鐨?sysfs 涓嶅悓锛宒ebugfs 鏍规湰
娌℃湁浠讳綍瑙勫垯銆傚紑鍙戣€呭彲浠ュ湪鍏朵腑鏀惧叆浠栦滑鎯宠鐨勪换浣曚俊鎭€俤ebugfs 鏂囦欢绯荤粺涔熶笉鎵撶畻浣滀负
鍚戠敤鎴风┖闂存彁渚涚殑绋冲畾 ABI锛涚悊璁轰笂锛屽鍏跺鍑虹殑鏂囦欢娌℃湁浠讳綍绋冲畾鎬х害鏉熴€傜劧鑰岋紝鐜板疄骞堕潪
鎬绘槸濡傛绠€鍗?[^1^]_锛涘嵆浣挎槸 debugfs 鎺ュ彛锛屾渶濂戒篃浠?闇€瑕佹案涔呯淮鎶?鐨勭悊蹇垫潵璁捐銆?
```

    mount -t debugfs none /sys/kernel/debug

```
锛堟垨涓€鏉＄瓑浠风殑 /etc/fstab 琛岋級銆?debugfs 鏍圭洰褰曢粯璁ゅ彧鏈?root 鐢ㄦ埛鍙闂€傝鏀瑰彉鏁存５鏍戠殑璁块棶鏉冮檺锛屽彲浠ヤ娇鐢?"uid"銆?"gid" 鍜?"mode" 鎸傝浇閫夐」銆?
娉ㄦ剰锛宒ebugfs API 浠呬互 GPL 鏂瑰紡瀵煎嚭缁欐ā鍧椼€?
浣跨敤 debugfs 鐨勪唬鐮佸簲鍖呭惈 <linux/debugfs.h>銆傜劧鍚庯紝绗竴浠朵簨灏嗘槸鑷冲皯鍒涘缓涓€涓洰褰曟潵
瀹圭撼涓€缁?```

    struct dentry *debugfs_create_dir(const char *name, struct dentry *parent);

```
璇ヨ皟鐢ㄥ鏋滄垚鍔燂紝灏嗗湪鎸囧畾鐨勭埗鐩綍涔嬩笅鍒涘缓涓€涓悕涓?name 鐨勭洰褰曘€傚鏋?parent 涓?NULL锛?鐩綍灏嗚鍒涘缓鍦?debugfs 鏍圭洰褰曚笅銆傛垚鍔熸椂锛岃繑鍥炲€兼槸涓€涓?struct dentry 鎸囬拡锛屽彲鐢ㄤ簬
鍦ㄨ鐩綍涓垱寤烘枃浠讹紙浠ュ強鏈€鍚庢竻鐞嗗畠锛夈€傝繑鍥?ERR_PTR(-ERROR) 琛ㄧず鍑虹幇浜嗛棶棰樸€傚鏋滆繑鍥?ERR_PTR(-ENODEV)锛屽垯琛ㄦ槑鍐呮牳鏄湪鏈惎鐢?debugfs 鏀寔鐨勬儏鍐典笅鏋勫缓鐨勶紝涓嬮潰鎻忚堪鐨勫嚱鏁?閮戒笉浼氬伐浣溿€?
```

    struct dentry *debugfs_create_file(const char *name, umode_t mode,
				       struct dentry *parent, void *data,
				       const struct file_operations *fops);

```
杩欓噷锛宯ame 鏄鍒涘缓鐨勬枃浠剁殑鍚嶇О锛宮ode 鎻忚堪鏂囦欢搴斿叿鏈夌殑璁块棶鏉冮檺锛宲arent 鎸囨槑鎸佹湁璇?鏂囦欢鐨勭洰褰曪紝data 灏嗚瀛樺偍鍦ㄧ粨鏋?inode 缁撴瀯鐨?i_private 瀛楁涓紝鑰?fops 鏄竴缁勫疄鐜?鏂囦欢琛屼负鐨勬枃浠舵搷浣溿€傝嚦灏戝簲鎻愪緵 read() 鍜?鎴?write() 鎿嶄綔锛涘叾浠栨搷浣滃彲鎸夐渶瑕佸姞鍏ャ€傚悓鏍凤紝
杩斿洖鍊兼槸鎵€鍒涘缓鏂囦欢鐨?dentry 鎸囬拡锛屽嚭閿欐椂涓?ERR_PTR(-ERROR)锛屾垨鑰呰嫢缂哄皯 debugfs 鏀寔
鍒欎负 ERR_PTR(-ENODEV)銆?
瑕佸垱寤轰竴涓叿鏈夊垵濮嬪ぇ灏忕殑鏂囦欢锛屽彲浠ヤ娇鐢ㄤ互涓嬪嚱鏁?```

    void debugfs_create_file_size(const char *name, umode_t mode,
				  struct dentry *parent, void *data,
				  const struct file_operations *fops,
				  loff_t file_size);

```
file_size 鏄枃浠剁殑鍒濆澶у皬銆傚叾浣欏弬鏁颁笌鍑芥暟 debugfs_create_file 鐩稿悓銆?
鍦ㄨ澶氭儏鍐典笅锛屽垱寤轰竴缁勬枃浠舵搷浣滃疄闄呬笂骞舵棤蹇呰锛沝ebugfs 浠ｇ爜涓虹畝鍗曞満鏅彁渚涗簡鑻ュ共
杈呭姪鍑芥暟銆傚寘鍚崟涓暣鏁板€肩殑鏂囦欢鍙互鐢?```

    void debugfs_create_u8(const char *name, umode_t mode,
			   struct dentry *parent, u8 *value);
    void debugfs_create_u16(const char *name, umode_t mode,
			    struct dentry *parent, u16 *value);
    void debugfs_create_u32(const char *name, umode_t mode,
			    struct dentry *parent, u32 *value);
    void debugfs_create_u64(const char *name, umode_t mode,
			    struct dentry *parent, u64 *value);

```
杩欎簺鏂囦欢鏀寔璇诲啓缁欏畾鐨勫€硷紱濡傛灉鏌愪釜鐗瑰畾鏂囦欢涓嶅簲琚啓鍏ワ紝鍙渶鐩稿簲鍦拌缃?mode 浣嶅嵆鍙€?杩欎簺鏂囦欢涓殑鍊间互鍗佽繘鍒惰〃绀猴紱濡傛灉鍗佸叚杩涘埗鏇村悎閫傦紝鍒?```

    void debugfs_create_x8(const char *name, umode_t mode,
			   struct dentry *parent, u8 *value);
    void debugfs_create_x16(const char *name, umode_t mode,
			    struct dentry *parent, u16 *value);
    void debugfs_create_x32(const char *name, umode_t mode,
			    struct dentry *parent, u32 *value);
    void debugfs_create_x64(const char *name, umode_t mode,
			    struct dentry *parent, u64 *value);

```
鍙寮€鍙戣€呯煡閬撹瀵煎嚭鐨勬暟鍊煎ぇ灏忥紝杩欎簺鍑芥暟灏卞緢鏈夌敤銆備笉杩囷紝鏌愪簺绫诲瀷鍦ㄤ笉鍚岀殑浣撶郴缁撴瀯涓?鍙兘鍏锋湁涓嶅悓鐨勪綅瀹斤紝杩欎娇寰楁儏鍐电◢寰鏉備簡涓€浜涖€傝繕鏈?```

    void debugfs_create_size_t(const char *name, umode_t mode,
			       struct dentry *parent, size_t *value);

```
姝ｅ鎵€鏂欙紝璇ュ嚱鏁颁細鍒涘缓涓€涓?debugfs 鏂囦欢鏉ヨ〃绀轰竴涓?size_t 绫诲瀷鐨勫彉閲忋€?
绫讳技鍦帮紝瀵逛簬 unsigned long 绫诲瀷鐨勫彉閲忎篃鏈夎緟鍔╁嚱鏁帮紝浠ュ崄杩涘埗琛ㄧず
```

    struct dentry *debugfs_create_ulong(const char *name, umode_t mode,
					struct dentry *parent,
					unsigned long *value);
    void debugfs_create_xul(const char *name, umode_t mode,
			    struct dentry *parent, unsigned long *value);

```
```

    void debugfs_create_bool(const char *name, umode_t mode,
                             struct dentry *parent, bool *value);

```
瀵圭粨鏋滄枃浠剁殑涓€娆¤鍙栧皢浜х敓 Y锛堝浜庨潪闆跺€硷級鎴?N锛屽悗璺熶竴涓崲琛岀銆傚鏋滃鍏跺啓鍏ワ紝瀹冨皢
鎺ュ彈澶у啓鎴栧皬鍐欑殑鍊硷紝鎴栬€?1 鎴?0銆備换浣曞叾浠栬緭鍏ラ兘浼氳闈欓粯蹇界暐銆?
```

    void debugfs_create_atomic_t(const char *name, umode_t mode,
				 struct dentry *parent, atomic_t *value)

```
瀵硅鏂囦欢鐨勮鍙栧皢鑾峰緱 atomic_t 鍊硷紝瀵硅鏂囦欢鐨勫啓鍏ュ皢璁剧疆 atomic_t 鍊笺€?
鍙︿竴涓€夐」鏄鍑轰竴涓换鎰忎簩杩涘埗鏁版嵁鍧楋紝浣跨敤
```

    struct debugfs_blob_wrapper {
	void *data;
	unsigned long size;
    };

    struct dentry *debugfs_create_blob(const char *name, umode_t mode,
				       struct dentry *parent,
				       struct debugfs_blob_wrapper *blob);

```
瀵硅鏂囦欢鐨勮鍙栧皢杩斿洖 debugfs_blob_wrapper 缁撴瀯鎵€鎸囧悜鐨勬暟鎹€備竴浜涢┍鍔ㄤ娇鐢?"blob" 浣滀负
杩斿洖澶氳锛堥潤鎬侊級鏍煎紡鍖栨枃鏈緭鍑虹殑绠€鍗曟柟寮忋€傝鍑芥暟鍙敤浜庡鍑轰簩杩涘埗淇℃伅锛屼絾涓荤嚎涓技涔?娌℃湁杩欐牱鍋氱殑浠ｇ爜銆傛敞鎰忥紝鎵€鏈夌敤 debugfs_create_blob() 鍒涘缓鐨勬枃浠堕兘鏄彧璇荤殑銆?
濡傛灉浣犳兂杞偍涓€鍧楀瘎瀛樺櫒锛堣繖鍦ㄥ紑鍙戣繃绋嬩腑缁忓父鍙戠敓锛屽敖绠″緢灏戞湁杩欐牱鐨勪唬鐮佽繘鍏ヤ富绾匡級锛?debugfs 鎻愪緵涓や釜鍑芥暟锛氫竴涓敤浜庡垱寤轰粎鍚瘎瀛樺櫒鐨勬枃浠讹紝鍙︿竴涓敤浜庡湪鍙︿竴涓『搴忔枃浠剁殑
涓棿鎻掑叆涓€涓瘎瀛樺櫒鍧?```

    struct debugfs_reg32 {
	char *name;
	unsigned long offset;
    };

    struct debugfs_regset32 {
	const struct debugfs_reg32 *regs;
	int nregs;
	void __iomem *base;
	struct device *dev;     /* Optional device for Runtime PM */
    };

    debugfs_create_regset32(const char *name, umode_t mode,
			    struct dentry *parent,
			    struct debugfs_regset32 *regset);

    void debugfs_print_regs32(struct seq_file *s, const struct debugfs_reg32 *regs,
			 int nregs, void __iomem *base, char *prefix);

```
"base" 鍙傛暟鍙互涓?0锛屼絾浣犲彲鑳芥兂鐢?__stringify 鏉ユ瀯寤?reg32 鏁扮粍锛屽苟涓旇澶氬瘎瀛樺櫒鍚?锛堝畯锛夊疄闄呬笂鏄浉瀵逛簬瀵勫瓨鍣ㄥ潡鍩哄潃鐨勫瓧鑺傚亸绉汇€?
```

    struct debugfs_u32_array {
	u32 *array;
	u32 n_elements;
    };

    void debugfs_create_u32_array(const char *name, umode_t mode,
			struct dentry *parent,
			struct debugfs_u32_array *array);

```
"array" 鍙傛暟灏佽浜嗘寚鍚戞暟缁勬暟鎹殑鎸囬拡鍙婂叾鍏冪礌涓暟銆傛敞鎰忥細涓€鏃︽暟缁勮鍒涘缓锛屽叾澶у皬灏?鏃犳硶鏇存敼銆?
```

   void debugfs_create_devm_seqfile(struct device *dev,
				const char *name,
				struct dentry *parent,
				int (*read_fn)(struct seq_file *s,
					void *data));

```
"dev" 鍙傛暟鏄笌姝?debugfs 鏂囦欢鐩稿叧鐨勮澶囷紝"read_fn" 鏄竴涓嚱鏁版寚閽堬紝灏嗚璋冪敤浠ユ墦鍗?seq_file 鐨勫唴瀹广€?
```

    struct dentry *debugfs_change_name(struct dentry *dentry,
					  const char *fmt, ...);

    struct dentry *debugfs_create_symlink(const char *name,
                                          struct dentry *parent,
				      	  const char *target);

```
瀵?debugfs_change_name() 鐨勮皟鐢ㄤ細涓轰竴涓凡瀛樺湪鐨?debugfs 鏂囦欢璧嬩簣涓€涓柊鍚嶇О锛屼笖濮嬬粓
鍦ㄥ悓涓€鐩綍涓€俷ew_name 鍦ㄨ皟鐢ㄥ墠蹇呴』涓嶅瓨鍦紱鎴愬姛鏃惰繑鍥炲€间负 0锛屽け璐ユ椂杩斿洖 -E...銆傜鍙?閾炬帴鍙互鐢?debugfs_create_symlink() 鍒涘缓銆?
鎵€鏈?debugfs 鐢ㄦ埛閮藉繀椤昏€冭檻涓€涓鐐癸細鍦?debugfs 涓垱寤虹殑浠讳綍鐩綍閮戒笉浼氳鑷姩娓呯悊銆?濡傛灉涓€涓ā鍧楀湪鍗歌浇鏃舵病鏈夋樉寮忕Щ闄?debugfs 鏉＄洰锛岀粨鏋滃皢鏄ぇ閲忛檲鏃ф寚閽堬紝浠ュ強鏃犵┓鏃犲敖鐨勩€?鏋佸叾涓嶅弸濂界殑琛屼负銆傚洜姝わ紝鎵€鏈?debugfs 鐢ㄦ埛鈥斺€旇嚦灏戞槸閭ｄ簺鍙互琚瀯寤轰负妯″潡鐨勨€斺€斿繀椤诲噯澶囧ソ
绉婚櫎瀹冧滑鍦ㄩ偅閲屽垱寤虹殑鎵€鏈夋枃浠跺拰鐩綍銆備竴涓枃浠?```

    void debugfs_remove(struct dentry *dentry);

```
dentry 鍊煎彲浠ヤ负 NULL 鎴栭敊璇€硷紝姝ゆ椂涓嶄細绉婚櫎浠讳綍鍐呭銆傛敞鎰忥紝璇ュ嚱鏁颁細閫掑綊绉婚櫎鍏朵笅鏂圭殑
鎵€鏈夋枃浠跺拰鐩綍銆備互鍓嶏紝debugfs_remove_recursive() 鐢ㄤ簬鎵ц璇ヤ换鍔★紝浣嗙幇鍦ㄨ鍑芥暟鍙槸
debugfs_remove() 鐨勪竴涓埆鍚嶃€俤ebugfs_remove_recursive() 搴旇瑙嗕负宸插簾寮冦€?