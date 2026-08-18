## 鍏充簬 kobjects銆乲sets 鍜?ktypes 閭ｄ簺浣犱粠涓嶆兂浜嗚В鐨勪簨


:Author: Greg Kroah-Hartman <gregkh@linuxfoundation.org>
:Last updated: December 19, 2007

鍩轰簬 Jon Corbet 涓?lwn.net 鎵€鍐欍€佸彂琛ㄤ簬 2003 骞?10 鏈?1 鏃ャ€佷綅浜?https://lwn.net/Articles/51437/ 鐨勫師鍒涙枃绔犮€?
鐞嗚В椹卞姩妯″瀷 鈥斺€?浠ュ強鏋勫缓浜庡叾涓婄殑 kobject 鎶借薄 鈥斺€?鐨勫洶闅句箣涓€鍦ㄤ簬娌℃湁鏄捐€屾槗瑙佺殑璧风偣銆傚鐞?kobject 闇€瑕佺悊瑙ｅ嚑绉嶄笉鍚岀殑绫诲瀷锛岃€屽畠浠郊姝ょ浉浜掑紩鐢ㄣ€備负浜嗚浜嬫儏鏇寸畝鍗曪紝鎴戜滑灏嗛噰鐢ㄥ娆￠€掕繘鐨勬柟寮忥紝浠庢ā绯婄殑鏈寮€濮嬶紝骞堕€愭琛ュ厖缁嗚妭銆備负姝わ紝涓嬮潰鍏堢粰鍑烘垜浠皢鐢ㄥ埌鐨勪竴浜涙湳璇殑蹇€熷畾涔夈€?
 - kobject 鏄?struct kobject 绫诲瀷鐨勫璞°€俴object 鍏锋湁涓€涓悕绉板拰寮曠敤璁℃暟銆俴object 杩樻湁涓€涓埗鎸囬拡锛堝厑璁稿皢瀵硅薄鎺掑垪鎴愬眰绾х粨鏋勶級銆佷竴涓壒瀹氱被鍨嬶紝浠ュ強閫氬父鍦?sysfs 铏氭嫙鏂囦欢绯荤粺涓殑涓€绉嶈〃绀恒€?
   kobject 鏈韩涓€鑸苟涓嶅紩浜哄叧娉紱鐩稿弽锛屽畠浠€氬父宓屽叆鍦ㄥ寘鍚唬鐮佺湡姝ｆ劅鍏磋叮鍐呭鐨勫叾浠栫粨鏋勪腑銆?
   浠讳綍缁撴瀯浣撻兘**缁濅笉搴?*宓屽叆澶氫簬涓€涓?kobject銆傚鏋滄湁锛岃瀵硅薄鐨勫紩鐢ㄨ鏁板繀瀹氫細娣蜂贡涓斾笉姝ｇ‘锛屼綘鐨勪唬鐮佸氨浼氭湁 bug銆傛墍浠ヤ笉瑕佽繖鏍峰仛銆?
 - ktype 鏄祵鍏ヤ簡 kobject 鐨勫璞＄殑绫诲瀷銆傛瘡涓祵鍏ヤ簡 kobject 鐨勭粨鏋勪綋閮介渶瑕佷竴涓搴旂殑 ktype銆俴type 鎺у埗鍦?kobject 琚垱寤哄拰閿€姣佹椂鍙戠敓鐨勪簨鎯呫€?
 - kset 鏄竴缁?kobject銆傝繖浜?kobject 鍙互鏄浉鍚岀殑 ktype锛屼篃鍙互灞炰簬涓嶅悓鐨?ktype銆俴set 鏄?kobject 闆嗗悎鐨勫熀鏈鍣ㄧ被鍨嬨€俴set 鍖呭惈瀹冧滑鑷繁鐨?kobject锛屼絾浣犲彲浠ユ斁蹇冨湴蹇界暐杩欎竴瀹炵幇缁嗚妭锛屽洜涓?kset 鏍稿績浠ｇ爜浼氳嚜鍔ㄥ鐞嗚繖涓?kobject銆?
   褰撲綘鐪嬪埌涓€涓～婊″叾浠栫洰褰曠殑 sysfs 鐩綍鏃讹紝閫氬父鍏朵腑姣忎釜鐩綍閮藉搴斾簬鍚屼竴涓?kset 涓殑涓€涓?kobject銆?
鎴戜滑灏嗕簡瑙ｅ浣曞垱寤哄拰鎿嶄綔鎵€鏈夎繖浜涚被鍨嬨€傛垜浠皢閲囩敤鑷簳鍚戜笂鐨勬柟娉曪紝鍥犳鍏堝洖鍒?kobject銆?

## 宓屽叆 kobject


鍐呮牳浠ｇ爜寰堝皯浼氬垱寤轰竴涓嫭绔嬬殑 kobject锛屽敮涓€鐨勯噸澶т緥澶栧皢鍦ㄤ笅鏂囪鏄庛€傜浉鍙嶏紝kobject 琚敤鏉ユ帶鍒跺鏇村ぇ鐨勩€佺壒瀹氶鍩熷璞＄殑璁块棶銆備负姝わ紝kobject 浼氬祵鍏ュ湪鍏朵粬缁撴瀯涓€傚鏋滀綘涔犳儻浜庣敤闈㈠悜瀵硅薄鐨勬湳璇€濊€冮棶棰橈紝kobject 鍙瑙嗕负涓€涓《灞傜殑鎶借薄绫伙紝鍏朵粬绫讳粠瀹冩淳鐢熴€俴object 瀹炵幇浜嗕竴缁勬湰韬敤澶勪笉澶с€佷絾鍦ㄥ叾浠栧璞′腑寰堝ソ鐢ㄧ殑鑳藉姏銆侰 璇█涓嶅厑璁哥洿鎺ヨ〃杈剧户鎵匡紝鍥犳蹇呴』浣跨敤鍏朵粬鎶€鏈€斺€斾緥濡傜粨鏋勪綋宓屽叆銆?
锛堥『渚胯涓€鍙ワ紝瀵逛簬鐔熸倝鍐呮牳閾捐〃瀹炵幇鐨勪汉锛岃繖绫讳技浜?"list_head" 缁撴瀯浣撴湰韬緢灏戝崟鐙湁鐢紝浣嗘€绘槸浼氬祵鍏ュ湪鎰熷叴瓒ｇ殑澶у璞′腑銆傦級

鍥犳锛屼緥濡?`drivers/uio/uio.c` 涓殑 UIO 浠ｇ爜鏈変竴涓粨鏋勪綋
```

    struct uio_map {
            struct kobject kobj;
            struct uio_mem *mem;
    };

```
濡傛灉浣犳湁涓€涓?struct uio_map 缁撴瀯浣擄紝鎵惧埌鍏跺祵鍏ョ殑 kobject 鍙渶浣跨敤 kobj 鎴愬憳銆備笉杩囷紝澶勭悊 kobject 鐨勪唬鐮侀€氬父浼氶亣鍒扮浉鍙嶇殑闂锛氱粰瀹氫竴涓?struct kobject 鎸囬拡锛屾寚鍚戝寘鍚畠鐨勭粨鏋勪綋鐨勬寚閽堟槸浠€涔堬紵浣犲繀椤婚伩鍏嶉偅浜涘彇宸ф墜娈碉紙渚嬪鍋囪 kobject 浣嶄簬缁撴瀯浣撶殑寮€澶达級
```

    container_of(ptr, type, member)

```
鍏朵腑锛?
  - `ptr` 鏄寚鍚戝祵鍏ョ殑 kobject 鐨勬寚閽堬紝
  - `type` 鏄寘鍚缁撴瀯浣撶殑绫诲瀷锛屽苟涓?  - `member` 鏄?`pointer` 鎵€鎸囧悜鐨勭粨鏋勪綋瀛楁鐨勫悕绉般€?
container_of() 鐨勮繑鍥炲€兼槸鎸囧悜鐩稿簲瀹瑰櫒绫诲瀷鐨勬寚閽堛€傚洜姝わ紝渚嬪锛屼竴涓寚鍚戝祵鍏?*浜?* struct uio_map 涓殑 struct kobject 鐨勬寚閽?`kp` 鍙互琚浆鎹负鎸囧悜璇?```

    struct uio_map *u_map = container_of(kp, struct uio_map, kobj);

```
涓轰簡鏂逛究锛岀▼搴忓憳甯稿父瀹氫箟涓€涓畝鍗曠殑瀹忥紝鐢ㄤ簬灏?kobject 鎸囬拡**鍙嶅悜杞崲**鍥炲寘鍚畠鐨勭被鍨嬨€傚湪涓嬮潰杩欑鎯呭喌涓鏄姝?```

    struct uio_map {
            struct kobject kobj;
            struct uio_mem *mem;
    };

    #define to_map(map) container_of(map, struct uio_map, kobj)

```
鍏朵腑瀹忓弬鏁?"map" 鏄寚鍚?struct uio_map 涓?struct kobject 鐨勬寚閽堬紝浣嶄簬
```

    struct uio_map *map = to_map(kobj);


```
## kobject 鐨勫垵濮嬪寲


鍒涘缓 kobject 鐨勪唬鐮佸綋鐒跺繀椤诲垵濮嬪寲璇ュ璞°€備竴浜?```

    void kobject_init(struct kobject *kobj, const struct kobj_type *ktype);

```
瑕佷娇 kobject 琚纭垱寤猴紝ktype 鏄繀闇€鐨勶紝鍥犱负姣忎釜 kobject 閮藉繀椤绘湁涓€涓叧鑱旂殑 kobj_type銆傝皟鐢?kobject_init() 涔嬪悗锛岃
```

    int kobject_add(struct kobject *kobj, struct kobject *parent,
                    const char *fmt, ...);

```
杩欎細姝ｇ‘璁剧疆 kobject 鐨勭埗瀵硅薄鍙婂叾鍚嶇О銆傚鏋?kobject 瑕佸叧鑱斿埌鐗瑰畾鐨?kset锛屽垯蹇呴』鍦ㄨ皟鐢?kobject_add() 涔嬪墠璧嬪€?kobj->kset銆傚鏋?kset 涓?kobject 鍏宠仈锛岄偅涔堝湪璋冪敤 kobject_add() 鏃跺彲灏?kobject 鐨勭埗瀵硅薄璁句负 NULL锛屾鏃?kobject 鐨勭埗瀵硅薄灏嗘槸 kset 鑷韩銆?
鐢变簬 kobject 鐨勫悕绉版槸鍦ㄥ畠琚姞鍏ュ唴鏍告椂璁剧疆鐨勶紝鍥犳缁濅笉搴旂洿鎺ユ搷绾?kobject 鐨勫悕绉般€傚鏋滀綘蹇呴』鏇存敼
```

    int kobject_rename(struct kobject *kobj, const char *new_name);

```
   kobject_rename() 涓嶆墽琛屼换浣曞姞閿侊紝涔熸病鏈夊叧浜庡摢浜涘悕绉版湁鏁堢殑纭垏姒傚康锛屽洜姝よ皟鐢ㄨ€呭繀椤昏嚜宸辨彁渚涘仴鍏ㄦ€ф鏌ュ拰涓茶鍖栥€?
鏈変竴涓悕涓?kobject_set_name() 鐨勫嚱鏁帮紝浣嗛偅灞炰簬鍘嗗彶閬楃暀鍨冨溇锛屾鍦ㄨ绉婚櫎銆傚鏋滀綘鐨勪唬鐮侀渶瑕佽皟鐢ㄨ繖涓嚱鏁帮紝閭ｆ槸涓嶆纭殑锛岄渶瑕佷慨澶嶃€?
瑕佹纭闂?kobject 鐨勫悕绉帮紝浣跨敤鍑芥暟
```

    const char *kobject_name(const struct kobject * kobj);

```
鏈変竴涓緟鍔╁嚱鏁板彲鍚屾椂鍒濆鍖?kobject 骞跺皢鍏跺姞鍏?```

    int kobject_init_and_add(struct kobject *kobj, const struct kobj_type *ktype,
                             struct kobject *parent, const char *fmt, ...);

```
   鍏跺弬鏁颁笌鍓嶉潰鎻忚堪鐨?kobject_init() 鍜?kobject_add() 鍑芥暟鐩稿悓銆?

## Uevents锛堢敤鎴锋€佷簨浠讹級


鍦?kobject 鍚?kobject 鏍稿績娉ㄥ唽涔嬪悗锛屼綘闇€瑕佸悜澶栫晫瀹ｅ憡瀹冨凡琚垱寤恒€傝繖鍙互閫氳繃
```

    int kobject_uevent(struct kobject *kobj, enum kobject_action action);

```
   褰?kobject 棣栨鍔犲叆鍐呮牳鏃朵娇鐢?**KOBJ_ADD** 鍔ㄤ綔銆傝繖搴斿綋浠呭湪 kobject 鐨勪换浣曞睘鎬ф垨瀛愬璞￠兘宸叉纭垵濮嬪寲涔嬪悗杩涜锛屽洜涓鸿璋冪敤鍙戠敓鏃剁敤鎴风┖闂翠細绔嬪嵆寮€濮嬫煡鎵惧畠浠€?
   褰?kobject 浠庡唴鏍哥Щ闄ゆ椂锛堝浣曟搷浣滅殑缁嗚妭瑙佷笅鏂囷級锛?*KOBJ_REMOVE** 鐨?uevent 浼氱敱 kobject 鏍稿績鑷姩鍒涘缓锛屽洜姝よ皟鐢ㄨ€呮棤闇€鎿嶅績鎵嬪姩鍘诲仛銆?

## 寮曠敤璁℃暟


kobject 鐨勫叧閿姛鑳戒箣涓€鏄厖褰撳叾鎵€宓屽叆瀵硅薄鐨勫紩鐢ㄨ鏁板櫒銆傚彧瑕佸璇ュ璞＄殑寮曠敤瀛樺湪锛岃瀵硅薄锛堝強鍏舵敮鎾戜唬鐮侊級灏卞繀椤荤户缁瓨鍦ㄣ€?```

    struct kobject *kobject_get(struct kobject *kobj);
    void kobject_put(struct kobject *kobj);

```
   鎴愬姛璋冪敤 kobject_get() 浼氶€掑 kobject 鐨勫紩鐢ㄨ鏁帮紝骞惰繑鍥炴寚鍚戣 kobject 鐨勬寚閽堛€?
   褰撻噴鏀句竴涓紩鐢ㄦ椂锛岃皟鐢?kobject_put() 浼氶€掑噺寮曠敤璁℃暟锛屽苟鍙兘閲婃斁璇ュ璞°€傛敞鎰?kobject_init() 灏嗗紩鐢ㄨ鏁拌涓?1锛屽洜姝よ缃?kobject 鐨勪唬鐮佹渶缁堥渶瑕佽皟鐢ㄤ竴娆?kobject_put() 鏉ラ噴鏀捐寮曠敤銆?
鐢变簬 kobject 鏄姩鎬佺殑锛屽畠浠粷涓嶈兘闈欐€佸０鏄庢垨鍦ㄦ爤涓婂０鏄庯紝鑰屽繀椤诲缁堝姩鎬佸垎閰嶃€傛湭鏉ョ殑鍐呮牳鐗堟湰灏嗗寘鍚闈欐€佸垱寤虹殑 kobject 鐨勮繍琛屾椂妫€鏌ワ紝骞跺悜寮€鍙戣€呰鍛婅繖绉嶄笉褰撶敤娉曘€?
濡傛灉浣犵殑鍏ㄩ儴闇€姹傚彧鏄负浣犵殑缁撴瀯浣撴彁渚涘紩鐢ㄨ鏁板櫒锛岃鏀圭敤 struct kref锛涗娇鐢?kobject 鍒欒繃浜庨噸閲忕骇銆傛湁鍏冲浣曚娇鐢?struct kref 鐨勬洿澶氫俊鎭紝璇峰弬闃?Linux 鍐呮牳婧愪唬鐮佹爲涓殑 Documentation/core-api/kref.rst 鏂囦欢銆?

## 鍒涘缓鈥滅畝鍗曗€濈殑 kobject


鏈夋椂寮€鍙戣€呮兂瑕佺殑鍙槸鍦?sysfs 灞傜骇涓垱寤轰竴涓畝鍗曠洰褰曠殑鏂规硶锛岃€屼笉蹇呭幓澶勭悊 kset銆乻how 鍜?store 鍑芥暟浠ュ強鍏朵粬缁嗚妭杩欎竴鏁村澶嶆潅鎬с€傝繖鏄簲褰撳垱寤哄崟涓?kobject 鐨勫敮涓€渚嬪鎯呭喌銆傝鍒涘缓杩欐牱涓€涓?```

    struct kobject *kobject_create_and_add(const char *name, struct kobject *parent);

```
璇ュ嚱鏁板皢鍒涘缓涓€涓?kobject锛屽苟灏嗗叾缃簬 sysfs 涓寚瀹氱埗 kobject 涓嬫柟鐨勪綅缃€傝鍒涘缓绠€鍗曞睘鎬?```

    int sysfs_create_file(struct kobject *kobj, const struct attribute *attr);

```
```

    int sysfs_create_group(struct kobject *kobj, const struct attribute_group *grp);

```
   杩欓噷浣跨敤鐨勪袱绫诲睘鎬э紝閰嶅悎鐢?kobject_create_and_add() 鍒涘缓鐨?kobject锛岄兘鍙互鏄?kobj_attribute 绫诲瀷锛屽洜姝ゆ棤闇€鍒涘缓鐗规畩鐨勮嚜瀹氫箟灞炴€с€?
鏈夊叧绠€鍗?kobject 鍜屽睘鎬х殑瀹炵幇锛岃鍙傝绀轰緥妯″潡 `samples/kobject/kobject-example.c`銆?

## ktype 涓庨噴鏀炬柟娉?

璁ㄨ涓粛鐒剁己灏戠殑涓€涓噸瑕侀棶棰樻槸锛氬綋 kobject 鐨勫紩鐢ㄨ鏁伴檷涓洪浂鏃朵細鍙戠敓浠€涔堛€傚垱寤?kobject 鐨勪唬鐮侀€氬父涓嶇煡閬撹繖浣曟椂浼氬彂鐢燂紱濡傛灉鐭ラ亾锛屼竴寮€濮嬩篃灏辨病澶氬ぇ蹇呰浣跨敤 kobject 浜嗐€傚綋寮曞叆 sysfs 鍚庯紝鍗充究鏄彲棰勬祴鐨勫璞＄敓鍛藉懆鏈熶篃浼氬彉寰楁洿澶嶆潅锛屽洜涓哄唴鏍哥殑鍏朵粬閮ㄥ垎鍙互鑾峰彇绯荤粺涓换浣曞凡娉ㄥ唽 kobject 鐨勫紩鐢ㄣ€?
鏈€缁堢粨鏋滄槸锛屽彈 kobject 淇濇姢鐨勭粨鏋勪綋鍦ㄥ叾寮曠敤璁℃暟褰掗浂涔嬪墠涓嶈兘琚噴鏀俱€傚紩鐢ㄨ鏁颁笉鍙楀垱寤?kobject 鐨勪唬鐮佺殑鐩存帴鎺у埗銆傚洜姝わ紝姣忓綋鍏舵煇涓?kobject 鐨勬渶鍚庝竴涓紩鐢ㄦ秷澶辨椂锛岃浠ｇ爜蹇呴』浠ュ紓姝ユ柟寮忓緱鍒伴€氱煡銆?
涓€鏃︿綘閫氳繃 kobject_add() 娉ㄥ唽浜?kobject锛屽氨缁濅笉鑳界敤 kfree() 鐩存帴閲婃斁瀹冦€傚敮涓€瀹夊叏鐨勬柟寮忔槸浣跨敤 kobject_put()銆傝壇濂藉疄璺垫槸濮嬬粓鍦?kobject_init() 涔嬪悗浣跨敤 kobject_put()锛屼互閬垮厤閿欒鎮勬倓娣峰叆銆?
杩欑閫氱煡閫氳繃 kobject 鐨?release() 鏂规硶瀹屾垚銆傞€氬父
```

    void my_object_release(struct kobject *kobj)
    {
            struct my_object *mine = container_of(kobj, struct my_object, kobj);

            /* Perform any additional cleanup on this object, then... */
            kfree(mine);
    }

```
   鏈変竴涓鐐规€庝箞寮鸿皟閮戒笉涓鸿繃锛氭瘡涓?kobject 閮藉繀椤绘湁涓€涓?release() 鏂规硶锛屽苟涓?kobject 蹇呴』淇濇寔瀛樺湪锛堝浜庝竴鑷寸姸鎬侊級鐩村埌璇ユ柟娉曡璋冪敤銆傚鏋滀笉婊¤冻杩欎簺绾︽潫锛屼唬鐮佸氨鏄湁缂洪櫡鐨勩€傛敞鎰忥紝濡傛灉浣犲繕璁版彁渚?release() 鏂规硶锛屽唴鏍镐細璀﹀憡浣犮€備笉瑕佽瘯鍥鹃€氳繃鎻愪緵涓€涓€滅┖鈥濈殑 release 鍑芥暟鏉ユ秷闄よ繖涓鍛娿€?
   濡傛灉浣犵殑娓呯悊鍑芥暟鍙渶璋冪敤 kfree()锛岄偅涔堜綘蹇呴』鍒涘缓涓€涓寘瑁呭嚱鏁帮紝浣跨敤 container_of() 鍚戜笂杞瀷涓烘纭殑绫诲瀷锛堝涓婇潰鐨勪緥瀛愭墍绀猴級锛岀劧鍚庡鏁翠釜缁撴瀯浣撹皟鐢?kfree()銆?
   娉ㄦ剰锛宬object 鐨勫悕绉板湪 release 鍑芥暟涓彲鐢紝浣嗙粷涓嶅簲鍦ㄦ鍥炶皟涓洿鏀瑰畠銆傚惁鍒?kobject 鏍稿績涓細鍑虹幇鍐呭瓨娉勬紡锛岃繖浼氳浜轰笉蹇€?
鏈夎叮鐨勬槸锛宺elease() 鏂规硶骞朵笉瀛樺偍鍦?kobject 鑷韩涓紝鑰屾槸涓?ktype 鍏宠仈銆傚洜姝よ鎴戜滑寮曞叆 struct
```

    struct kobj_type {
            void (*release)(struct kobject *kobj);
            const struct sysfs_ops *sysfs_ops;
            const struct attribute_group **default_groups;
            const struct kobj_ns_type_operations *(*child_ns_type)(struct kobject *kobj);
            const void *(*namespace)(struct kobject *kobj);
            void (*get_ownership)(struct kobject *kobj, kuid_t *uid, kgid_t *gid);
    };

```
   璇ョ粨鏋勪綋鐢ㄤ簬鎻忚堪涓€绉嶇壒瀹氱被鍨嬬殑 kobject锛堟垨鑰呮洿鍑嗙‘鍦拌锛屾槸鍖呭惈瀹冪殑瀵硅薄锛夈€傛瘡涓?kobject 閮介渶瑕佹湁涓€涓叧鑱旂殑 kobj_type 缁撴瀯浣擄紱鍦ㄨ皟鐢?kobject_init() 鎴?kobject_init_and_add() 鏃跺繀椤绘寚瀹氭寚鍚戣缁撴瀯浣撶殑鎸囬拡銆?
struct kobj_type 涓殑 release 瀛楁褰撶劧鏄寚鍚戞绫?kobject 鐨?release() 鏂规硶鐨勬寚閽堛€傚彟澶栦袱涓瓧娈碉紙sysfs_ops 鍜?default_groups锛夋帶鍒舵绫诲璞″湪 sysfs 涓殑琛ㄧず鏂瑰紡锛涜繖瓒呭嚭浜嗘湰鏂囨。鐨勮寖鍥淬€?
default_groups 鎸囬拡鏄竴涓粯璁ゅ睘鎬у垪琛紝浼氫负娉ㄥ唽鍒拌 ktype 鐨勪换浣?kobject 鑷姩鍒涘缓銆?

## kset


kset 浠呬粎鏄竴缁勫笇鏈涘郊姝ゅ叧鑱旂殑 kobject 鐨勯泦鍚堛€傚苟涓嶈姹傚畠浠睘浜庣浉鍚岀殑 ktype锛屼絾濡傛灉涓嶆槸锛屽垯瑕侀潪甯稿皬蹇冦€?
kset 鎻愪緵浠ヤ笅鍔熻兘锛?
 - 瀹冨厖褰撲竴涓绾充竴缁勫璞＄殑琚嬪瓙銆傚唴鏍稿彲浠ョ敤 kset 鏉ヨ窡韪€滄墍鏈夊潡璁惧鈥濇垨鈥滄墍鏈?PCI 璁惧椹卞姩鈥濄€?
 - kset 涔熸槸 sysfs 涓殑涓€涓瓙鐩綍锛屽叧鑱旂殑 kobject 鍙互鍑虹幇鍏朵腑銆傛瘡涓?kset 鍖呭惈涓€涓?kobject锛岃 kobject 鍙璁剧疆涓哄叾浠?kobject 鐨勭埗瀵硅薄锛泂ysfs 灞傜骇涓殑椤剁骇鐩綍灏辨槸浠ヨ繖绉嶆柟寮忔瀯寤虹殑銆?
 - kset 鍙互鏀寔 kobject 鐨勨€滅儹鎻掓嫈鈥濓紝骞跺奖鍝?uevent 浜嬩欢濡備綍涓婃姤缁欑敤鎴风┖闂淬€?
鐢ㄩ潰鍚戝璞＄殑鏈璇达紝鈥渒set鈥?鏄《灞傜殑瀹瑰櫒绫伙紱kset 鍖呭惈瀹冧滑鑷繁鐨?kobject锛屼絾閭ｄ釜 kobject 鐢?kset 浠ｇ爜绠＄悊锛屼笉搴旇浠讳綍鍏朵粬鐢ㄦ埛鎿嶇旱銆?
kset 灏嗗叾瀛愬璞′繚瀛樺湪涓€涓爣鍑嗙殑 kernel 閾捐〃涓€俴object 閫氳繃瀹冧滑鐨?kset 瀛楁鎸囧洖鍏舵墍灞炵殑 kset銆傚湪鍑犱箮鎵€鏈夋儏鍐典笅锛屽睘浜庢煇涓?kset 鐨?kobject 鍦ㄥ叾鐖跺璞′腑鎷ユ湁璇?kset锛堜弗鏍煎湴璇达紝鏄叾宓屽叆鐨?kobject锛夈€?
鐢变簬 kset 鍐呴儴鍖呭惈涓€涓?kobject锛屽畠搴斿缁堣鍔ㄦ€佸垱寤猴紝鑰岀粷涓嶈兘闈欐€佸０鏄庢垨鍦ㄦ爤涓婂０鏄庛€傝鍒涘缓涓€涓柊鐨?```

  struct kset *kset_create_and_add(const char *name,
                                   const struct kset_uevent_ops *uevent_ops,
                                   struct kobject *parent_kobj);

```
```

  void kset_unregister(struct kset *k);

```
   鏉ラ攢姣佸畠銆傝繖浼氫粠 sysfs 涓Щ闄?kset 骞堕€掑噺鍏跺紩鐢ㄨ鏁般€傚綋寮曠敤璁℃暟褰掗浂鏃讹紝kset 灏嗚閲婃斁銆傜敱浜庡彲鑳戒粛瀛樺湪瀵?kset 鐨勫叾浠栧紩鐢紝閲婃斁鍙兘鍙戠敓鍦?kset_unregister() 杩斿洖涔嬪悗銆?
浣跨敤 kset 鐨勭ず渚嬪彲鍙傝鍐呮牳鏍戜腑鐨?`samples/kobject/kset-example.c` 鏂囦欢銆?
濡傛灉 kset 甯屾湜鎺у埗 kobject 鐨?uevent 鎿嶄綔
```

  struct kset_uevent_ops {
          int (* const filter)(struct kobject *kobj);
          const char *(* const name)(struct kobject *kobj);
          int (* const uevent)(struct kobject *kobj, struct kobj_uevent_env *env);
  };


```
   filter 鍑芥暟鍏佽 kset 闃绘涓烘煇涓壒瀹?kobject 鍚戠敤鎴风┖闂村彂鍑?uevent銆傚鏋滃嚱鏁拌繑鍥?0锛屽垯涓嶄細鍙戝嚭 uevent銆?
   name 鍑芥暟浼氳璋冪敤锛屼互瑕嗙洊 uevent 鍙戦€佺粰鐢ㄦ埛绌洪棿鐨?kset 鐨勯粯璁ゅ悕绉般€傞粯璁ゆ儏鍐典笅鍚嶇О涓?kset 鏈韩鐩稿悓锛屼絾鑻ユ彁渚涗簡姝ゅ嚱鏁帮紝瀹冨彲浠ヨ鐩栬鍚嶇О銆?
   uevent 鍑芥暟浼氬湪 uevent 鍗冲皢鍙戦€佺粰鐢ㄦ埛绌洪棿鏃惰璋冪敤锛屼互渚垮悜 uevent 娣诲姞鏇村鐜鍙橀噺銆?
   鏈変汉鍙兘浼氶棶锛屾棦鐒舵病鏈夌粰鍑烘墽琛岃鍔熻兘鐨勫嚱鏁帮紝kobject 绌剁珶鏄浣曡鍔犲叆 kset 鐨勩€傜瓟妗堟槸杩欎釜浠诲姟鐢?kobject_add() 澶勭悊銆傚綋 kobject 琚紶缁?kobject_add() 鏃讹紝瀹冪殑 kset 鎴愬憳搴旀寚鍚戣 kobject 灏嗗綊灞炵殑 kset銆俴object_add() 浼氬鐞嗗叾浣欎簨瀹溿€?
   濡傛灉灞炰簬鏌愪釜 kset 鐨?kobject 娌℃湁璁剧疆鐖?kobject锛屽畠灏嗚鍔犲叆璇?kset 鐨勭洰褰曘€傚苟闈?kset 鐨勬墍鏈夋垚鍛橀兘涓€瀹氫綅浜?kset 鐩綍涓€傚鏋滃湪 kobject 琚姞鍏ヤ箣鍓嶆樉寮忔寚瀹氫簡鐖?kobject锛岄偅涔堣 kobject 浼氳娉ㄥ唽鍒?kset锛屼絾浼氭坊鍔犲湪鐖?kobject 涔嬩笅銆?

## kobject 鐨勭Щ闄?

鍦?kobject 鎴愬姛鍚?kobject 鏍稿績娉ㄥ唽涔嬪悗锛屽綋浠ｇ爜浣跨敤瀹屾瘯鏃跺繀椤绘竻鐞嗗畠銆傝鍋氬埌杩欎竴鐐癸紝璋冪敤 kobject_put()銆傝繖鏍凤紝kobject 鏍稿績浼氳嚜鍔ㄦ竻鐞嗚 kobject 鍒嗛厤鐨勬墍鏈夊唴瀛樸€傚鏋滃凡涓鸿瀵硅薄鍙戦€佷簡 `KOBJ_ADD` uevent锛屽垯浼氬彂閫佺浉搴旂殑 `KOBJ_REMOVE` uevent锛屽苟涓斾换浣曞叾浠?sysfs 鍐呭姟宸ヤ綔涔熶細涓鸿皟鐢ㄨ€呭Ε鍠勫鐞嗐€?
濡傛灉浣犻渶瑕佸 kobject 杩涜涓ら樁娈靛垹闄わ紙渚嬪鍦ㄤ綘涓嶈兘鐫＄湢鐨勬椂鍊欓渶瑕侀攢姣佽瀵硅薄锛夛紝閭ｄ箞璋冪敤 kobject_del()锛屽畠浼氬皢 kobject 浠?sysfs 娉ㄩ攢銆傝繖浼氳 kobject 鈥滀笉鍙鈥濓紝浣嗗畠骞舵湭琚竻鐞嗭紝瀵硅薄鐨勫紩鐢ㄨ鏁颁粛鐒剁浉鍚屻€傜◢鍚庤皟鐢?kobject_put() 鏉ュ畬鎴愪笌璇?kobject 鍏宠仈鐨勫唴瀛樻竻鐞嗐€?
濡傛灉瀛樺湪寰幆寮曠敤锛宬object_del() 鍙敤浜庝涪寮冨鐖跺璞＄殑寮曠敤銆傚湪鏌愪簺鎯呭喌涓嬬埗瀵硅薄寮曠敤瀛愬璞℃槸鍚堟硶鐨勩€傚惊鐜紩鐢╛蹇呴』_閫氳繃鏄惧紡璋冪敤 kobject_del() 鏉ユ墦鐮达紝浠ヤ究璋冪敤 release 鍑芥暟锛屼娇鍓嶅惊鐜腑鐨勫璞″郊姝ら噴鏀俱€?

## 鍙弬鑰冪殑绀轰緥浠ｇ爜


鏈夊叧姝ｇ‘浣跨敤 kset 鍜?kobject 鐨勬洿瀹屾暣绀轰緥锛岃鍙傞槄绀轰緥绋嬪簭 `samples/kobject/{kobject-example.c,kset-example.c}`锛屽鏋滀綘閫夋嫨浜?`CONFIG_SAMPLE_KOBJECT`锛屽畠浠皢琚瀯寤轰负鍙姞杞芥ā鍧椼€?