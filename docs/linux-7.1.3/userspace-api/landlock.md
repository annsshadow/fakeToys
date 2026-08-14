
## Landlock锛氭棤鐗规潈璁块棶鎺у埗


:Author: Micka毛l Sala眉n
:Date: March 2026

Landlock 鐨勭洰鏍囨槸鑳藉闄愬埗涓€缁勮繘绋嬬殑鐜鏉冨埄锛堜緥濡傚叏灞€鏂囦欢绯荤粺鎴栫綉缁滆闂級銆傚洜涓?Landlock 鏄竴涓彲鍫嗗彔鐨?LSM锛屽畠浣垮緱鍒涘缓瀹夊叏娌欑鎴愪负鍙兘锛屼綔涓洪櫎鐜版湁绯荤粺绾ц闂帶鍒朵箣澶栫殑鏂扮殑瀹夊叏灞傘€傝繖绫绘矙绠辨湁鏈涘府鍔╃紦瑙ｇ敤鎴风┖闂村簲鐢ㄤ腑缂洪櫡鎴栨剰澶?鎭舵剰琛屼负鐨勫畨鍏ㄥ奖鍝嶃€侺andlock 璧嬩簣浠讳綍杩涚▼锛堝寘鎷棤鐗规潈杩涚▼锛夊畨鍏ㄥ湴闄愬埗鑷韩鐨勮兘鍔涖€?
鎴戜滑鍙互閫氳繃鍦ㄥ唴鏍告棩蹇椾腑瀵绘壘 "landlock: Up and running"锛堜互 root 韬唤锛夋潵蹇€熺‘璁よ繍琛屼腑鐨勭郴缁熸槸鍚﹀惎鐢ㄤ簡 Landlock锛?`dmesg | grep landlock || journalctl -kb -g landlock` 銆?寮€鍙戣€呬篃鍙互鍊熷姪鐩稿叧鐨勭郴缁熻皟鐢?<landlock_abi_versions> 杞绘澗妫€鏌?Landlock 鏀寔鎯呭喌銆?濡傛灉褰撳墠涓嶆敮鎸?Landlock锛屾垜浠渶瑕侀€傚綋鍦伴厤缃唴鏍?<kernel_support>銆?
## Landlock 瑙勫垯


Landlock 瑙勫垯鎻忚堪杩涚▼鎵撶畻鍦ㄥ璞′笂鎵ц鐨勪竴涓姩浣溿€備竴缁勮鍒欒鑱氬悎杩涗竴涓鍒欓泦锛坮uleset锛夛紝瀹冮殢鍚庡彲浠ラ檺鍒跺疄鏂藉畠鐨勭嚎绋嬶紝浠ュ強瀹冩湭鏉ョ殑瀛愯繘绋嬨€?
鐜版湁鐨勪袱绫昏鍒欎负锛?
Filesystem rules
    瀵逛簬杩欎簺瑙勫垯锛屽璞℃槸鏂囦欢灞傜骇锛岀浉鍏崇殑鏂囦欢绯荤粺鍔ㄤ綔鐢?    `filesystem access rights` 瀹氫箟銆?
Network rules (since ABI v4)
    瀵逛簬杩欎簺瑙勫垯锛屽璞℃槸 TCP 绔彛锛岀浉鍏冲姩浣滅敱 `network access rights` 瀹氫箟銆?
### 瀹氫箟涓庡疄鏂藉畨鍏ㄧ瓥鐣?

鎴戜滑棣栧厛闇€瑕佸畾涔夊皢瀹圭撼鎴戜滑瑙勫垯鐨勮鍒欓泦銆?
鍦ㄦ绀轰緥涓紝瑙勫垯闆嗗皢鍖呭惈鍙厑璁告枃浠剁郴缁熻鍔ㄤ綔骞跺缓绔嬬壒瀹?TCP 杩炴帴鐨勮鍒欍€傛枃浠剁郴缁熷啓鍔ㄤ綔涓庡叾浠?TCP 鍔ㄤ綔灏嗚鎷掔粷銆?
瑙勫垯闆嗛殢鍚庨渶瑕佸鐞嗚繖涓ょ被鍔ㄤ綔銆傝繖鏄悜鍚庝笌鍚戝墠鍏煎鎬ф墍蹇呴渶鐨勶紙鍗冲唴鏍镐笌鐢ㄦ埛绌洪棿鍙兘浜掍笉璁よ瘑瀵规柟鏀寔鐨勫彈闄愰」锛夛紝鍥犳闇€瑕佸榛樿鎷掔粷鐨勮闂潈闄愬姞浠ユ槑纭€?

    struct landlock_ruleset_attr ruleset_attr = {
        .handled_access_fs =
            LANDLOCK_ACCESS_FS_EXECUTE |
            LANDLOCK_ACCESS_FS_WRITE_FILE |
            LANDLOCK_ACCESS_FS_READ_FILE |
            LANDLOCK_ACCESS_FS_READ_DIR |
            LANDLOCK_ACCESS_FS_REMOVE_DIR |
            LANDLOCK_ACCESS_FS_REMOVE_FILE |
            LANDLOCK_ACCESS_FS_MAKE_CHAR |
            LANDLOCK_ACCESS_FS_MAKE_DIR |
            LANDLOCK_ACCESS_FS_MAKE_REG |
            LANDLOCK_ACCESS_FS_MAKE_SOCK |
            LANDLOCK_ACCESS_FS_MAKE_FIFO |
            LANDLOCK_ACCESS_FS_MAKE_BLOCK |
            LANDLOCK_ACCESS_FS_MAKE_SYM |
            LANDLOCK_ACCESS_FS_REFER |
            LANDLOCK_ACCESS_FS_TRUNCATE |
            LANDLOCK_ACCESS_FS_IOCTL_DEV |
            LANDLOCK_ACCESS_FS_RESOLVE_UNIX,
        .handled_access_net =
            LANDLOCK_ACCESS_NET_BIND_TCP |
            LANDLOCK_ACCESS_NET_CONNECT_TCP,
        .scoped =
            LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
            LANDLOCK_SCOPE_SIGNAL,
    };

鍥犱负鎴戜滑鍙兘鏃犳硶鐭ラ亾搴旂敤灏嗗湪鍝釜鍐呮牳鐗堟湰涓婃墽琛岋紝閬靛惊灏藉姏鑰屼负锛坆est-effort锛夌殑瀹夊叏绛栫暐鏇村畨鍏ㄣ€傜‘瀹烇紝鎴戜滑搴斿綋灏藉彲鑳藉鍦颁繚鎶ょ敤鎴凤紝鏃犺浠栦滑浣跨敤浠€涔堝唴鏍搞€?
涓轰簡涓庤緝鏃х殑 Linux 鐗堟湰鍏煎锛屾垜浠娴嬪彲鐢ㄧ殑 Landlock ABI 鐗堟湰锛屽苟浠呬娇鐢ㄥ彲鐢ㄧ殑璁块棶鏉冮檺瀛愰泦锛?

    int abi;

    abi = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_VERSION);
    if (abi < 0) {
        /** Degrades gracefully if Landlock is not handled. **/
        perror("The running kernel does not enable to use Landlock");
        return 0;
    }
    switch (abi) {
    case 1:
        /** Removes LANDLOCK_ACCESS_FS_REFER for ABI < 2 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_REFER;
        __attribute__((fallthrough));
    case 2:
        /** Removes LANDLOCK_ACCESS_FS_TRUNCATE for ABI < 3 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_TRUNCATE;
        __attribute__((fallthrough));
    case 3:
        /** Removes network support for ABI < 4 **/
        ruleset_attr.handled_access_net &=
            ~(LANDLOCK_ACCESS_NET_BIND_TCP |
              LANDLOCK_ACCESS_NET_CONNECT_TCP);
        __attribute__((fallthrough));
    case 4:
        /** Removes LANDLOCK_ACCESS_FS_IOCTL_DEV for ABI < 5 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_IOCTL_DEV;
        __attribute__((fallthrough));
    case 5:
        /** Removes LANDLOCK_SCOPE_** for ABI < 6 */
        ruleset_attr.scoped &= ~(LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
                                 LANDLOCK_SCOPE_SIGNAL);
        __attribute__((fallthrough));
    case 6 ... 8:
        /** Removes LANDLOCK_ACCESS_FS_RESOLVE_UNIX for ABI < 9 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_RESOLVE_UNIX;
    }

杩欏氨鍚敤浜嗗皢鍖呭惈鎴戜滑瑙勫垯鐨勩€佸寘瀹瑰紡瑙勫垯闆嗙殑鍒涘缓銆?

    int ruleset_fd;

    ruleset_fd = landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
    if (ruleset_fd < 0) {
        perror("Failed to create a ruleset");
        return 1;
    }

鎴戜滑鐜板湪鍙互鍊熷姪杩斿洖鐨勬寚浠ｆ瑙勫垯闆嗙殑鏂囦欢鎻忚堪绗︼紝鍚戣瑙勫垯闆嗘坊鍔犱竴鏉℃柊瑙勫垯銆傝繖鏉¤鍒欏皢鍏佽璇诲彇涓庢墽琛屾枃浠跺眰绾?`/usr`銆傝嫢娌℃湁鍙︿竴鏉¤鍒欙紝鍐欏姩浣滈殢鍚庡皢琚鍒欓泦鎷掔粷銆備负浜嗘妸 `/usr` 鍔犲叆瑙勫垯闆嗭紝鎴戜滑鐢?`O_PATH` 鏍囧織鎵撳紑瀹冿紝骞剁敤姝ゆ枃浠舵弿杩扮濉厖 &struct landlock_path_beneath_attr銆?

    int err;
    struct landlock_path_beneath_attr path_beneath = {
        .allowed_access =
            LANDLOCK_ACCESS_FS_EXECUTE |
            LANDLOCK_ACCESS_FS_READ_FILE |
            LANDLOCK_ACCESS_FS_READ_DIR,
    };

    path_beneath.parent_fd = open("/usr", O_PATH | O_CLOEXEC);
    if (path_beneath.parent_fd < 0) {
        perror("Failed to open file");
        close(ruleset_fd);
        return 1;
    }
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
                            &path_beneath, 0);
    close(path_beneath.parent_fd);
    if (err) {
        perror("Failed to update ruleset");
        close(ruleset_fd);
        return 1;
    }

涔熷彲鑳介渶瑕佹牴鎹?Landlock ABI 鐗堟湰绛涢€夎闂潈闄愶紝閬靛惊涓庡墠杩拌鍒欓泦鍒涘缓鐩稿悓鐨勯€昏緫鏉ュ垱寤鸿鍒欍€傚湪鏈緥涓笉闇€瑕侊紝鍥犱负鎵€鏈夎姹傜殑 `allowed_access` 鏉冮檺鍦?ABI 1 涓凡鍙敤銆?
瀵逛簬缃戠粶璁块棶鎺у埗锛屾垜浠彲浠ユ坊鍔犱竴缁勫厑璁稿皢鏌愪釜绔彛鍙风敤浜庣壒瀹氬姩浣滐紙HTTPS 杩炴帴锛夌殑瑙勫垯銆?

    struct landlock_net_port_attr net_port = {
        .allowed_access = LANDLOCK_ACCESS_NET_CONNECT_TCP,
        .port = 443,
    };

    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
                            &net_port, 0);

褰撳悜 `landlock_restrict_self()` 浼犲叆闈炵┖鐨?`flags` 鍙傛暟鏃讹紝瀵?restrict 鏍囧織涔熼渶瑕佺被浼肩殑鍚戝悗鍏煎鎬ф鏌ワ紙鍙敤鏍囧織璇峰弬瑙?sys_landlock_restrict_self() 鏂囨。锛夛細


    __u32 restrict_flags =
        LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON |
        LANDLOCK_RESTRICT_SELF_TSYNC;
    switch (abi) {
    case 1 ... 6:
        /** Removes logging flags for ABI < 7 **/
        restrict_flags &= ~(LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF |
                            LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON |
                            LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF);
        __attribute__((fallthrough));
    case 7:
        /*
         - Removes multithreaded enforcement flag for ABI < 8
         *
         - WARNING: Without this flag, calling landlock_restrict_self(2) is
         - only equivalent if the calling process is single-threaded. Below
         - ABI v8 (and as of ABI v8, when not using this flag), a Landlock
         - policy would only be enforced for the calling thread and its
         - children (and not for all threads, including parents and siblings).
         */
        restrict_flags &= ~LANDLOCK_RESTRICT_SELF_TSYNC;
    }

涓嬩竴姝ユ槸闄愬埗褰撳墠绾跨▼鑾峰彇鏇村鐗规潈锛堜緥濡傞€氳繃 SUID 浜岃繘鍒讹級銆傛垜浠幇鍦ㄦ湁浜嗕竴涓鍒欓泦锛氱涓€鏉¤鍒欏厑璁稿 `/usr` 鐨勮涓庢墽琛岃闂紝鍚屾椂鎷掔粷鏂囦欢绯荤粺鎵€鏈夊叾浠栬澶勭悊鐨勮闂紱绗簩鏉¤鍒欏厑璁?HTTPS 杩炴帴銆?

    if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
        perror("Failed to restrict privileges");
        close(ruleset_fd);
        return 1;
    }

褰撳墠绾跨▼鐜板湪宸插噯澶囧ソ鐢ㄨ鍒欓泦鑷垜娌欑鍖栥€?

    if (landlock_restrict_self(ruleset_fd, restrict_flags)) {
        perror("Failed to enforce ruleset");
        close(ruleset_fd);
        return 1;
    }
    close(ruleset_fd);

濡傛灉 `landlock_restrict_self` 绯荤粺璋冪敤鎴愬姛锛屽綋鍓嶇嚎绋嬬幇鍦ㄥ凡琚檺鍒讹紝骞朵笖姝ょ瓥鐣ヤ篃灏嗚瀹炴柦鍒板畠闅忓悗鍒涘缓鐨勬墍鏈夊瓙杩涚▼涓娿€備竴鏃︾嚎绋嬭 Landlock 鍖栵紝灏辨病鏈夊姙娉曠Щ闄ゅ畠鐨勫畨鍏ㄧ瓥鐣ワ紱鍙厑璁告坊鍔犳洿澶氶檺鍒躲€傝繖浜涚嚎绋嬬幇鍦ㄥ浜庝竴涓柊鐨?Landlock 鍩熶腑锛岃鍩熸槸瀹冧滑鐖跺煙锛堣嫢鏈夛級涓庢柊瑙勫垯闆嗙殑鍚堝苟銆?
瀹屾暣鍙伐浣滅殑浠ｇ爜鍙湪 `samples/landlock/sandboxer.c`_ 涓壘鍒般€?
### 鑹ソ瀹炶返


寤鸿灏藉彲鑳藉皢璁块棶鏉冮檺璁剧疆鍒版枃浠跺眰绾х殑鍙惰妭鐐广€備緥濡傦紝涓庢妸 `~/` 璁句负鍙灞傜骇銆佹妸 `~/tmp/` 璁句负璇诲啓灞傜骇鐩告瘮锛屾洿濂界殑鍋氭硶鏄妸 `~/doc/` 璁句负鍙灞傜骇銆佹妸 `~/tmp/` 璁句负璇诲啓灞傜骇銆傞伒寰繖涓€鑹ソ瀹炶返浼氬甫鏉ヤ笉渚濊禆浜庡叾浣嶇疆锛堝嵆鐖剁洰褰曪級鐨勮嚜瓒冲眰绾с€傝繖鍦ㄦ垜浠鍏佽閾炬帴鎴栭噸鍛藉悕鏃跺挨鍏剁浉鍏炽€傜‘瀹烇紝姣忎釜鐩綍鎷ユ湁涓€鑷寸殑璁块棶鏉冮檺锛屼娇寰楀彲浠ュ湪涓嶄緷璧栫洰鏍囩洰褰曡闂潈闄愶紙鏈搷浣滄墍闇€鐨勬潈闄愰櫎澶栵紝鍙傝 `LANDLOCK_ACCESS_FS_REFER` 鏂囨。锛夌殑鎯呭喌涓嬫敼鍙樿繖浜涚洰褰曠殑浣嶇疆銆?
鎷ユ湁鑷冻灞傜骇涔熸湁鍔╀簬鎶婃墍闇€鐨勮闂潈闄愭敹绱у埌鏈€灏忕殑鏁版嵁闆嗗悎銆傝繖涔熸湁鍔╀簬閬垮厤鈥?sinkhole 鐩綍鈥濓紙鍗虫暟鎹彲浠ヨ閾炬帴鍒板叾涓€佸嵈鏃犳硶浠庝腑閾炬帴鍑烘潵鐨勭洰褰曪級銆傜劧鑰岋紝杩欏彇鍐充簬鏁版嵁缁勭粐锛岃€屾暟鎹粍缁囧彲鑳戒笉鍙楀紑鍙戣€呮帶鍒躲€傚湪杩欑鎯呭喌涓嬶紝鎺堜簣 `~/tmp/` 璇诲啓璁块棶锛堣€岄潪浠呭啓璁块棶锛夛紝浼氭綔鍦ㄥ湴鍏佽鎶?`~/tmp/` 绉诲姩鍒颁竴涓笉鍙鐩綍锛屽悓鏃朵粛淇濈暀鍒楀嚭 `~/tmp/` 鍐呭鐨勮兘鍔涖€?
### 鏂囦欢璺緞璁块棶鏉冮檺鐨勫眰绾?

姣忓綋涓€涓嚎绋嬪鑷韩瀹炴柦涓€涓鍒欓泦鏃讹紝瀹冨氨鐢ㄦ柊鐨勪竴灞傜瓥鐣ユ洿鏂板畠鐨?Landlock 鍩熴€傝繖涓€琛ュ厖绛栫暐浼氫笌浠讳綍鍙兘宸茬粡鍦ㄩ檺鍒舵绾跨▼鐨勫叾浠栬鍒欓泦鍫嗗彔鍦ㄤ竴璧枫€備竴涓娌欑鍖栫殑绾跨▼闅忓悗鍙互鐢ㄤ竴涓柊瀹炴柦鐨勮鍒欓泦瀹夊叏鍦颁负鑷繁娣诲姞鏇村绾︽潫銆?
鑻ユ煇绛栫暐灞傚湪璺緞涓婇伃閬囩殑鍏惰鍒欎腑鑷冲皯鏈変竴鏉℃巿浜堣璁块棶锛屽垯璇ョ瓥鐣ュ眰鎺堜簣瀵规枃浠惰矾寰勭殑璁块棶銆備竴涓娌欑鍖栫殑绾跨▼鍙湁鍦ㄥ畠鐨勬墍鏈夊凡瀹炴柦绛栫暐灞備互鍙婃墍鏈夊叾浠栫郴缁熻闂帶鍒讹紙渚嬪鏂囦欢绯荤粺 DAC銆佸叾瀹?LSM 绛栫暐绛夛級閮芥巿浜堣璁块棶鏃讹紝鎵嶈兘璁块棶鏌愪釜鏂囦欢璺緞銆?
### 缁戝畾鎸傝浇涓?OverlayFS


Landlock 鑳藉闄愬埗瀵规枃浠跺眰绾х殑璁块棶锛岃繖鎰忓懗鐫€杩欎簺璁块棶鏉冮檺鍙互闅忕粦瀹氭寕杞戒紶鎾紙鍙傝 Documentation/filesystems/sharedsubtree.rst锛夛紝浣嗕笉鑳介殢 Documentation/filesystems/overlayfs.rst 浼犳挱銆?
缁戝畾鎸傝浇灏嗘簮鏂囦欢灞傜骇闀滃儚鍒扮洰鏍囥€傜洰鏍囧眰绾ч殢鍚庣敱瀹屽叏鐩稿悓鐨勬枃浠剁粍鎴愶紝Landlock 瑙勫垯鍙互缁戝畾鍒板叾涓婏紝鏃犺鏄€氳繃婧愯矾寰勮繕鏄洰鏍囪矾寰勩€傝繖浜涜鍒欏湪璺緞涓婇伃閬囨椂闄愬埗璁块棶锛岃繖鎰忓懗鐫€瀹冧滑鍙互鍚屾椂闄愬埗瀵瑰涓枃浠跺眰绾х殑璁块棶锛屾棤璁鸿繖浜涘眰绾ф槸鍚︾粦瀹氭寕杞界殑缁撴灉銆?
涓€涓?OverlayFS 鎸傝浇鐐圭敱涓婂眰涓庝笅灞傜粍鎴愩€傝繖浜涘眰鍦ㄤ竴涓悎骞剁洰褰曚腑琚粍鍚堬紝璇ュ悎骞剁洰褰曞湪鎸傝浇鐐瑰鍙樺緱鍙敤銆傝繖涓悎骞跺眰绾у彲鑳藉寘鍚潵鑷笂灞備笌涓嬪眰鐨勬枃浠讹紝浣嗗湪鍚堝苟灞傜骇涓婃墽琛岀殑淇敼鍙弽鏄犲埌涓婂眰銆備粠 Landlock 绛栫暐鐨勮搴︾湅锛屾墍鏈?OverlayFS 灞備笌鍚堝苟灞傜骇閮芥槸鐙珛鐨勶紝鍚勮嚜鍖呭惈鑷繁鐨勪竴缁勬枃浠朵笌鐩綍锛岃繖涓庣粦瀹氭寕杞戒笉鍚屻€傞檺鍒舵煇涓?OverlayFS 灞傜殑绛栫暐涓嶄細闄愬埗鐢辨浜х敓鐨勫悎骞跺眰绾э紝鍙嶄箣浜︾劧銆傚洜姝?Landlock 鐢ㄦ埛搴斿綋鍙€冭檻浠栦滑鎯冲厑璁歌闂殑鏂囦欢灞傜骇锛岃€屼笉蹇呯搴曞眰鏂囦欢绯荤粺銆?
### 缁ф壙


姣忎竴涓敱 `clone(2)` 浜х敓鐨勬柊绾跨▼閮戒粠鐖剁嚎绋嬬户鎵?Landlock 鍩熼檺鍒躲€傝繖绫讳技浜?seccomp 缁ф壙锛堝弬瑙?Documentation/userspace-api/seccomp_filter.rst锛夋垨浠讳綍澶勭悊浠诲姟 `credentials(7)` 鐨勫叾瀹?LSM銆備緥濡傦紝涓€涓繘绋嬬殑鏌愪釜绾跨▼鍙互瀵瑰畠鑷韩搴旂敤 Landlock 瑙勫垯锛屼絾杩欎簺瑙勫垯涓嶄細鑷姩搴旂敤鍒板叾瀹冨厔寮熺嚎绋嬶紙涓嶅悓浜?POSIX 绾跨▼鍑瘉鍙樻洿锛屽弬瑙?`nptl(7)`锛夈€?
褰撲竴涓嚎绋嬭嚜鎴戞矙绠卞寲鏃讹紝鎴戜滑淇濊瘉鐩稿叧瀹夊叏绛栫暐浼氭寔缁疄鏂藉湪璇ョ嚎绋嬬殑鎵€鏈夊悗浠ｄ笂銆傝繖浣垮緱鍙互鎸夊簲鐢ㄥ垱寤虹嫭绔嬩笖妯″潡鍖栫殑瀹夊叏绛栫暐锛屽畠浠細鏍规嵁鍏惰繍琛屾椂鐖剁瓥鐣ヨ嚜鍔ㄧ浉浜掔粍鍚堛€?
### Ptrace 闄愬埗


涓€涓娌欑鍖栫殑杩涚▼鎷ユ湁鐨勭壒鏉冨皯浜庢湭琚矙绠卞寲鐨勮繘绋嬶紝鍥犳鍦ㄦ搷浣滃彟涓€涓繘绋嬫椂蹇呴』鍙楀埌棰濆闄愬埗銆備负浜嗗厑璁稿湪鐩爣杩涚▼涓婁娇鐢?`ptrace(2)` 鍙婄浉鍏崇郴缁熻皟鐢紝涓€涓娌欑鍖栫殑杩涚▼搴斿綋鎷ユ湁鐩爣杩涚▼璁块棶鏉冮檺鐨勮秴闆嗭紝杩欐剰鍛崇潃琚窡韪€咃紙tracee锛夊繀椤诲浜庤窡韪€咃紙tracer锛夌殑瀛愬煙涓€?
### IPC 浣滅敤鍩?

绫讳技浜庨殣鍚殑 `Ptrace restrictions`_锛屾垜浠彲鑳芥兂瑕佽繘涓€姝ラ檺鍒舵矙绠变箣闂寸殑浜や簰銆傚洜姝わ紝鍦ㄥ垱寤鸿鍒欓泦鏃讹紝姣忎釜 Landlock 鍩熷彲浠ラ檺鍒舵煇浜涙搷浣滅殑浣滅敤鍩燂紝浣垮緱杩欎簺鎿嶄綔鍙兘瑙﹀強鍚屼竴 Landlock 鍩熷唴鎴栧祵濂?Landlock 鍩燂紙鈥渟cope鈥濓級鍐呯殑杩涚▼銆?
鍙彈浣滅敤鍩熼檺鍒剁殑鎿嶄綔鏈夛細

`LANDLOCK_SCOPE_SIGNAL`
    杩欓檺鍒朵簡鍚戣繍琛屼簬鍚屼竴鎴栧祵濂?Landlock 鍩熷唴鐨勭洰鏍囪繘绋嬪彂閫佷俊鍙枫€?
`LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET`
    杩欓檺鍒朵簡鎴戜滑鍙互 `connect(2)` 鐨勬娊璞?`unix(7)` 濂楁帴瀛楅泦鍚堬紝浠呴檺鐢卞悓涓€鎴栧祵濂?Landlock 鍩熷唴鐨勮繘绋嬪垱寤虹殑濂楁帴瀛楀湴鍧€銆?
    瀵规湭杩炴帴鏁版嵁鎶ュ鎺ュ瓧鎵ц `sendto(2)` 浼氳褰撲綔杩涜浜嗕竴娆￠殣鍚殑 `connect(2)`锛屽鏋滆繙绔苟闈炴簮鑷悓涓€鎴栧祵濂?Landlock 鍩燂紝鍒欎細琚樆濉炪€?
    瀵逛箣鍓嶅凡杩炴帴鐨勫鎺ュ瓧鎵ц `sendto(2)` 涓嶅彈闄愬埗銆傝繖瀵规暟鎹姤涓庢祦濂楁帴瀛楅兘閫傜敤銆?
IPC 浣滅敤鍩熶笉鏀寔閫氳繃 `landlock_add_rule(2)` 璁剧疆渚嬪銆傚鏋滀竴涓搷浣滃湪鏌愪釜鍩熷唴鍙椾綔鐢ㄥ煙闄愬埗锛屽垯娌℃湁浠讳綍瑙勫垯鍙互琚坊鍔犳潵鍏佽璁块棶浣滅敤鍩熶箣澶栫殑璧勬簮鎴栬繘绋嬨€?
### 鎴柇鏂囦欢


`LANDLOCK_ACCESS_FS_WRITE_FILE` 涓?`LANDLOCK_ACCESS_FS_TRUNCATE` 瑕嗙洊鐨勬搷浣滈兘浼氭敼鍙樻枃浠跺唴瀹癸紝骞朵笖鏈夋椂浼氫互涓嶇洿瑙傜殑鏂瑰紡閲嶅彔銆傚己鐑堝缓璁€绘槸灏嗕袱鑰呬竴璧锋寚瀹氾紙瑕佷箞閮芥巿浜堬紝瑕佷箞閮戒笉鎺堜簣锛夈€?
涓€涓壒鍒护浜烘儕璁剁殑渚嬪瓙鏄?`creat(2)`銆傚叾鍚嶇О鏆楃ず姝ょ郴缁熻皟鐢ㄩ渶瑕佸垱寤轰笌鍐欏叆鏂囦欢鐨勬潈闄愩€傜劧鑰岋紝濡傛灉鍚屽悕涓嬪凡瀛樺湪鏌愪釜鏂囦欢锛屽畠杩橀渶瑕?truncate 鏉冮檺銆?
杩樺簲褰撴敞鎰忥紝鎴柇鏂囦欢骞朵笉瑕佹眰 `LANDLOCK_ACCESS_FS_WRITE_FILE` 鏉冮檺銆傞櫎浜?`truncate(2)` 绯荤粺璋冪敤涔嬪锛岃繖涔熷彲浠ラ€氳繃浠?`O_RDONLY | O_TRUNC` 鏍囧織 `open(2)` 鏉ュ畬鎴愩€?
鍚屾椂锛屽湪鏌愪簺鏂囦欢绯荤粺涓婏紝`fallocate(2)` 鎻愪緵浜嗗湪鏂囦欢浠ュ啓鏂瑰紡鎵撳紑鏃躲€佺敤 `FALLOC_FL_COLLAPSE_RANGE` 缂╃煭鏂囦欢鍐呭鐨勯€斿緞锛屼粠鑰岀粫寮€ `LANDLOCK_ACCESS_FS_TRUNCATE` 鏉冮檺銆?
truncate 鏉冮檺涓庡凡鎵撳紑鐨勬枃浠跺叧鑱旓紙瑙佷笅鏂囷級銆?
### 涓庢枃浠舵弿杩扮鍏宠仈鐨勬潈闄?

鎵撳紑鏂囦欢鏃讹紝`LANDLOCK_ACCESS_FS_TRUNCATE` 涓?`LANDLOCK_ACCESS_FS_IOCTL_DEV` 鏉冮檺鐨勫彲鐢ㄦ€у叧鑱斿埌鏂板垱寤虹殑鏂囦欢鎻忚堪绗︼紝骞跺皢琚敤浜庨殢鍚庝娇鐢?`ftruncate(2)` 涓?`ioctl(2)` 鐨勬埅鏂笌 ioctl 灏濊瘯銆傚叾琛屼负绫讳技浜庝负璇绘垨鍐欐墦寮€鏂囦欢锛氭潈闄愬湪 `open(2)` 鏃舵鏌ワ紝鑰屽湪闅忓悗鐨?`read(2)` 涓?`write(2)` 璋冪敤鏃朵笉妫€鏌ャ€?
鍥犳锛屼竴涓繘绋嬪彲鑳芥嫢鏈夊涓寚鍚戝悓涓€鏂囦欢鐨勫凡鎵撳紑鏂囦欢鎻忚堪绗︼紝浣?Landlock 鍦ㄧ敤杩欎簺鏂囦欢鎻忚堪绗︽搷浣滄椂瀹炴柦涓嶅悓鐨勪笢瑗裤€傝繖鍙兘鍙戠敓鍦細鏌愪釜 Landlock 瑙勫垯闆嗚瀹炴柦锛岃€岃杩涚▼淇濈暀浜嗗湪瀹炴柦鍓嶅悗閮芥墦寮€鐨勬枃浠舵弿杩扮銆備篃鍙互鍦ㄨ繖浜涙枃浠舵弿杩扮浜庤繘绋嬮棿浼犻€掓椂淇濈暀鍏?Landlock 灞炴€э紝鍗充娇鏌愪簺鐩稿叧杩涚▼娌℃湁宸插疄鏂界殑 Landlock 瑙勫垯闆嗐€?
## 鍏煎鎬?

### 鍚戝墠涓庡悜鍚庡吋瀹规€?

Landlock 琚璁′负涓庡唴鏍哥殑杩囧幓涓庢湭鏉ョ増鏈吋瀹广€傝繖鏄€氳繃绯荤粺璋冪敤灞炴€у強鍏宠仈鐨勪綅鏍囧織锛堝挨鍏舵槸瑙勫垯闆嗙殑 `handled_access_fs`锛夊疄鐜扮殑銆傛妸琚鐞嗙殑璁块棶鏉冮檺鏄惧紡鍖栵紝浣垮緱鍐呮牳涓庣敤鎴风┖闂村郊姝や箣闂存湁涓€涓竻鏅扮殑绾﹀畾銆傝繖鏄‘淇濇矙绠卞寲涓嶄細鍥犵郴缁熸洿鏂拌€屽彉寰楁洿涓ユ牸锛堥偅鍙兘鐮村潖搴旂敤锛夋墍蹇呴渶鐨勩€?
寮€鍙戣€呭彲浠ヨ闃?`Landlock mailing list <https://subspace.kernel.org/lists.linux.dev.html>`_ 鏉ユ湁鎰忓湴鐢ㄦ渶鏂板彲鐢ㄧ壒鎬ф洿鏂板苟娴嬭瘯浠栦滑鐨勫簲鐢ㄣ€備负浜嗙敤鎴风殑鍒╃泭锛屽苟涓斿洜涓轰粬浠彲鑳戒娇鐢ㄤ笉鍚岀殑鍐呮牳鐗堟湰锛屽己鐑堝缓璁伒寰敖鍔涜€屼负鐨勫畨鍏ㄧ瓥鐣ワ細鍦ㄨ繍琛屾椂妫€鏌?Landlock ABI 鐗堟湰锛屽苟鍙疄鏂藉彈鏀寔鐨勭壒鎬с€?

### Landlock ABI 鐗堟湰


Landlock ABI 鐗堟湰鍙互鐢?sys_landlock_create_ruleset() 绯荤粺璋冪敤璇诲彇锛?

    int abi;

    abi = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_VERSION);
    if (abi < 0) {
        switch (errno) {
        case ENOSYS:
            printf("Landlock is not supported by the current kernel.\n");
            break;
        case EOPNOTSUPP:
            printf("Landlock is currently disabled.\n");
            break;
        }
        return 0;
    }
    if (abi >= 2) {
        printf("Landlock supports LANDLOCK_ACCESS_FS_REFER.\n");
    }

闄ら潪鍏舵枃妗ｄ腑鏄庣‘娉ㄦ槑锛屾墍鏈?Landlock 鍐呮牳鎺ュ彛閮借绗竴涓?ABI 鐗堟湰鏀寔銆?
### Landlock 鍕樿


闄?ABI 鐗堟湰澶栵紝Landlock 杩樻彁渚涗竴绉嶅嫎璇紙errata锛夋満鍒讹紝鐢ㄤ簬璺熻釜鍙兘褰卞搷鍚戝悗鍏煎鎬ф垨闇€瑕佺敤鎴风┖闂寸煡鏅撶殑闂淇銆傚嫎璇綅鎺╃爜鍙互鐢ㄤ互涓嬫柟寮忔煡璇細


    int errata;

    errata = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_ERRATA);
    if (errata < 0) {
        /** Landlock not available or disabled **/
        return 0;
    }

杩斿洖鐨勫€兼槸涓€涓綅鎺╃爜锛屽叾涓瘡涓綅浠ｈ〃涓€涓壒瀹氱殑 erratum銆傚鏋滅 N 浣嶈缃綅锛坄errata & (1 << (N - 1))`锛夛紝鍒?erratum N 宸插湪杩愯涓殑鍐呮牳涓慨澶嶃€?

   **澶у鏁板簲鐢ㄤ笉搴旀鏌ュ嫎璇€?* 鍦?99.9% 鐨勬儏鍐典笅锛屾鏌ュ嫎璇槸涓嶅繀瑕佺殑锛屼細澧炲姞浠ｇ爜澶嶆潅搴︼紝骞朵笖鑻ヨ璇敤杩樺彲鑳介檷浣庝繚鎶ゃ€備緥濡傦紝鍦ㄦ煇涓?erratum 鏈淇鏃剁鐢ㄦ矙绠憋紝鍙兘浣跨郴缁熸瘮浣跨敤 Landlock 鐨勫敖鍔涜€屼负淇濇姢鏇翠笉瀹夊叏銆傚鏈夌枒闂紝蹇界暐鍕樿銆?
    :doc: erratum_1

    :doc: erratum_2

    :doc: erratum_3

#### 濡備綍妫€鏌ュ嫎璇?

濡傛灉浣犵‘瀹氫綘鐨勫簲鐢ㄩ渶瑕佹鏌ョ壒瀹氬嫎璇紝浣跨敤濡備笅妯″紡锛?

    int errata = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_ERRATA);
    if (errata >= 0) {
        /** Check for specific erratum (1-indexed) **/
        if (errata & (1 << (erratum_number - 1))) {
            /** Erratum N is fixed in this kernel **/
        } else {
            /** Erratum N is NOT fixed - consider implications for your use case **/
        }
    }

**閲嶈锛?* 鍙湁褰撲綘鐨勫簲鐢ㄧ壒鍒緷璧栦簬鍥犺淇鑰屾敼鍙樼殑琛屼负鏃讹紝鎵嶆鏌ュ嫎璇€傝繖浜涗慨澶嶉€氬父浼氳 Landlock 闄愬埗鏇村皯鎴栨洿姝ｇ‘锛岃€屼笉鏄洿涓ユ牸銆?
## 鍐呮牳鎺ュ彛


### 璁块棶鏉冮檺


    :identifiers: fs_access net_access scope

### 鍒涘缓鏂扮殑瑙勫垯闆?

    :identifiers: sys_landlock_create_ruleset

    :identifiers: landlock_ruleset_attr

### 鎵╁睍瑙勫垯闆?

    :identifiers: sys_landlock_add_rule

    :identifiers: landlock_rule_type landlock_path_beneath_attr
                  landlock_net_port_attr

### 瀹炴柦瑙勫垯闆?

    :identifiers: sys_landlock_restrict_self

## 褰撳墠闄愬埗


### 鏂囦欢绯荤粺鎷撴墤淇敼


琚枃浠剁郴缁熼檺鍒舵矙绠卞寲鐨勭嚎绋嬩笉鑳戒慨鏀规枃浠剁郴缁熸嫇鎵戯紝鏃犺鏄€氳繃 `mount(2)` 杩樻槸 `pivot_root(2)`銆傜劧鑰岋紝`chroot(2)` 璋冪敤涓嶄細琚嫆缁濄€?
### 鐗规畩鏂囦欢绯荤粺


鏍规嵁瑙勫垯闆嗚澶勭悊鐨勮闂紝Landlock 鍙互闄愬埗瀵瑰父瑙勬枃浠朵笌鐩綍鐨勮闂€傜劧鑰岋紝骞堕潪鏉ヨ嚜鐢ㄦ埛鍙鏂囦欢绯荤粺锛堜緥濡?pipe銆乻ocket锛夈€佷絾浠嶅彲閫氳繃 `/proc/<pid>/fd/*` 璁块棶鐨勬枃浠讹紝鐩墠鏃犳硶琚樉寮忛檺鍒躲€傜被浼煎湴锛屾煇浜涚壒娈婂唴鏍告枃浠剁郴缁燂紙濡?nsfs锛屽彲閫氳繃 `/proc/<pid>/ns/*` 璁块棶锛夌洰鍓嶄篃鏃犳硶琚樉寮忛檺鍒躲€備笉杩囷紝鍊熷姪 `ptrace restrictions`_锛屽姝ょ被鏁忔劅 `/proc` 鏂囦欢鐨勮闂細鏍规嵁鍩熷眰绾ц嚜鍔ㄥ彈鍒伴檺鍒躲€傛湭鏉ョ殑 Landlock 婕旇繘浠嶅彲鑳介€氳繃涓撻棬鐨勮鍒欓泦鏍囧織鍚敤瀵规绫昏矾寰勭殑鏄惧紡闄愬埗銆?
### 瑙勫垯闆嗗眰绾?

鍫嗗彔瑙勫垯闆嗙殑灞傜骇闄愬埗涓?16 灞傘€傝繖瀵逛簬涓€涓笇鏈涘湪鍏剁户鎵跨殑 16 涓鍒欓泦涔嬪鍐嶅疄鏂戒竴涓柊瑙勫垯闆嗙殑浠诲姟鑰岃█鍙兘鎴愰棶棰樸€備竴鏃﹁揪鍒版闄愬埗锛宻ys_landlock_restrict_self() 杩斿洖 E2BIG銆傚洜姝ゅ己鐑堝缓璁湪鏌愪釜绾跨▼鐨勭敓鍛藉懆鏈熶腑涓€娆℃€т粩缁嗗湴鏋勫缓瑙勫垯闆嗭紝鐗瑰埆鏄浜庨偅浜涘彲鑳藉惎鍔ㄥ叾瀹冧篃鍙兘鎯宠嚜鎴戞矙绠卞寲鐨勫簲鐢ㄧ殑搴旂敤锛堜緥濡?shells銆佸鍣ㄧ鐞嗗櫒绛夛級銆?
### 鍐呭瓨浣跨敤


涓哄垱寤鸿鍒欓泦鑰屽垎閰嶇殑鍐呮牳鍐呭瓨浼氳璁拌处锛屽苟鍙€氳繃 Documentation/admin-guide/cgroup-v1/memory.rst 鍔犱互闄愬埗銆?
### IOCTL 鏀寔


`LANDLOCK_ACCESS_FS_IOCTL_DEV` 鏉冮檺闄愬埗 `ioctl(2)` 鐨勪娇鐢紝浣嗗畠鍙€傜敤浜?*鏂版墦寮€鐨?*璁惧鏂囦欢銆傝繖鍏蜂綋鎰忓懗鐫€棰勫厛瀛樺湪鐨勬枃浠舵弿杩扮锛堝 stdin銆乻tdout 涓?stderr锛変笉鍙楀奖鍝嶃€?
鐢ㄦ埛搴斿綋鎰忚瘑鍒帮紝TTY 璁惧浼犵粺涓婂厑璁搁€氳繃 `TIOCSTI` 涓?`TIOCLINUX` IOCTL 鍛戒护鎺у埗鍚屼竴 TTY 涓婄殑鍏跺畠杩涚▼銆傝繖涓よ€呴兘闇€瑕佺幇浠?Linux 绯荤粺涓婄殑 `CAP_SYS_ADMIN`锛屼絾 `TIOCSTI` 鐨勮涓烘槸鍙厤缃殑銆?
鍥犳鍦ㄨ緝鏃х殑绯荤粺涓婏紝寤鸿鍏抽棴缁ф壙鐨?TTY 鏂囦欢鎻忚堪绗︼紝鎴栧敖鍙兘浠?`/proc/self/fd/*` 閲嶆柊鎵撳紑瀹冧滑鑰屼笉甯?`LANDLOCK_ACCESS_FS_IOCTL_DEV` 鏉冮檺銆?
Landlock 鐨?IOCTL 鏀寔鐩墠鏄矖绮掑害鐨勶紝浣嗘湭鏉ュ彲鑳藉彉寰楁洿缁嗙矑搴︺€傚湪閭ｄ箣鍓嶏紝寤鸿鐢ㄦ埛閫氳繃鏂囦欢灞傜骇鏉ュ缓绔嬩粬浠墍闇€鐨勪繚璇侊紝鍙湪鐪熸闇€瑕佺殑鍦版柟鍏佽 `LANDLOCK_ACCESS_FS_IOCTL_DEV` 鏉冮檺銆?
## 浠ュ線鐨勯檺鍒?

### 鏂囦欢閲嶅懡鍚嶄笌閾炬帴锛圓BI < 2锛?

鍥犱负 Landlock 闈㈠悜鏃犵壒鏉冭闂帶鍒讹紝瀹冮渶瑕佹伆褰撳湴澶勭悊瑙勫垯鐨勭粍鎴愩€傝繖涓€鎬ц川涔熸剰鍛崇潃瑙勫垯鐨勫祵濂椼€傛伆褰撳湴澶勭悊澶氫釜瑙勫垯闆嗗眰绾э紙姣忎釜閮借兘闄愬埗瀵规枃浠剁殑璁块棶锛夛紝涔熸剰鍛崇潃瑙勫垯闆嗛檺鍒朵粠鐖剁骇鍒板叾灞傜骇鐨勭户鎵裤€傚洜涓烘枃浠堕€氳繃鍏跺眰绾ц璇嗗埆涓庨檺鍒讹紝灏嗕竴涓枃浠朵粠涓€涓洰褰曠Щ鍔ㄦ垨閾炬帴鍒板彟涓€涓洰褰曟剰鍛崇潃灞傜骇绾︽潫鐨勪紶鎾紝鎴栨牴鎹繖浜涘彲鑳戒涪澶辩殑绾︽潫鏉ラ檺鍒惰繖浜涘姩浣溿€備负浜嗛槻姝㈤€氳繃閲嶅懡鍚嶆垨閾炬帴杩涜鏉冮檺鎻愬崌锛屽苟涓斾负浜嗙畝鍗曡捣瑙侊紝Landlock 姝ゅ墠灏嗛摼鎺ヤ笌閲嶅懡鍚嶉檺鍒跺湪鍚屼竴鐩綍鍐呫€備粠 Landlock ABI 鐗堟湰 2 寮€濮嬶紝鐜板湪鍙互鍊熷姪鏂扮殑 `LANDLOCK_ACCESS_FS_REFER` 璁块棶鏉冮檺瀹夊叏鍦版帶鍒堕噸鍛藉悕涓庨摼鎺ャ€?
### 鏂囦欢鎴柇锛圓BI < 3锛?

鍦ㄧ涓変釜 Landlock ABI 涔嬪墠鏃犳硶鎷掔粷鏂囦欢鎴柇锛屽洜姝ゅ湪浣跨敤鍙敮鎸佺涓€鎴栫浜?ABI 鐨勫唴鏍告椂锛屾埅鏂€绘槸琚厑璁搞€?
浠?Landlock ABI 鐗堟湰 3 寮€濮嬶紝鐜板湪鍙互鍊熷姪鏂扮殑 `LANDLOCK_ACCESS_FS_TRUNCATE` 璁块棶鏉冮檺瀹夊叏鍦版帶鍒舵埅鏂€?
### TCP 缁戝畾涓庤繛鎺ワ紙ABI < 4锛?

浠?Landlock ABI 鐗堟湰 4 寮€濮嬶紝鐜板湪鍙互鍊熷姪鏂扮殑 `LANDLOCK_ACCESS_NET_BIND_TCP` 涓?`LANDLOCK_ACCESS_NET_CONNECT_TCP` 璁块棶鏉冮檺锛屽皢 TCP 缁戝畾涓庤繛鎺ュ姩浣滈檺鍒跺埌浠呬竴缁勫厑璁哥殑绔彛銆?
### 璁惧 IOCTL锛圓BI < 5锛?

鍦ㄧ浜斾釜 Landlock ABI 涔嬪墠鏃犳硶鎷掔粷 IOCTL 鎿嶄綔锛屽洜姝ゅ湪浣跨敤鍙敮鎸佹洿鏃?ABI 鐨勫唴鏍告椂锛宍ioctl(2)` 鎬绘槸琚厑璁搞€?
浠?Landlock ABI 鐗堟湰 5 寮€濮嬶紝鍙互鍊熷姪鏂扮殑 `LANDLOCK_ACCESS_FS_IOCTL_DEV` 鏉冮檺锛岄檺鍒跺瀛楃璁惧涓庡潡璁惧浣跨敤 `ioctl(2)`銆?
### 鎶借薄 UNIX 濂楁帴瀛楋紙ABI < 6锛?

浠?Landlock ABI 鐗堟湰 6 寮€濮嬶紝鍙互閫氳繃灏?`LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET` 璁剧疆鍒?`scoped` 瑙勫垯闆嗗睘鎬э紝鏉ラ檺鍒跺鎶借薄 `unix(7)` 濂楁帴瀛楃殑杩炴帴銆?
### 淇″彿锛圓BI < 6锛?

浠?Landlock ABI 鐗堟湰 6 寮€濮嬶紝鍙互閫氳繃灏?`LANDLOCK_SCOPE_SIGNAL` 璁剧疆鍒?`scoped` 瑙勫垯闆嗗睘鎬э紝鏉ラ檺鍒?`signal(7)` 鐨勫彂閫併€?
### 鏃ュ織锛圓BI < 7锛?

浠?Landlock ABI 鐗堟湰 7 寮€濮嬶紝鍙互閫氳繃浼犲叆 sys_landlock_restrict_self() 鐨?`LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF`銆乣LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON` 涓?`LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF` 鏍囧織锛屾帶鍒?Landlock 瀹¤浜嬩欢鐨勬棩蹇楄褰曘€傚叧浜庡璁＄殑鏇村缁嗚妭璇峰弬瑙?Documentation/admin-guide/LSM/landlock.rst銆?
### 绾跨▼鍚屾锛圓BI < 8锛?

浠?Landlock ABI 鐗堟湰 8 寮€濮嬶紝鐜板湪鍙互鍊熷姪浼犲叆 sys_landlock_restrict_self() 鐨?`LANDLOCK_RESTRICT_SELF_TSYNC` 鏍囧織锛岃法璋冪敤杩涚▼鐨勬墍鏈夌嚎绋嬪疄鏂?Landlock 瑙勫垯闆嗐€?
### 璺緞鍚?UNIX 濂楁帴瀛楋紙ABI < 9锛?

浠?Landlock ABI 鐗堟湰 9 寮€濮嬶紝鍙互鍊熷姪鏂扮殑 `LANDLOCK_ACCESS_FS_RESOLVE_UNIX` 鏉冮檺锛岄檺鍒跺璺緞鍚?UNIX 鍩熷鎺ュ瓧锛坄unix(7)`锛夌殑杩炴帴銆?

## 鍐呮牳鏀寔


### 鏋勫缓鏃堕厤缃?

Landlock 棣栧厛鍦?Linux 5.13 涓紩鍏ワ紝浣嗗繀椤诲湪鏋勫缓鏃剁敤 `CONFIG_SECURITY_LANDLOCK=y` 閰嶇疆銆侺andlock 涔熷繀椤诲儚鍏跺畠瀹夊叏妯″潡涓€鏍峰湪鍚姩鏃跺惎鐢ㄣ€傞粯璁ゅ惎鐢ㄧ殑瀹夊叏妯″潡鍒楄〃鐢?`CONFIG_LSM` 璁剧疆銆傚洜姝ゅ唴鏍搁厤缃簲褰撳寘鍚?`CONFIG_LSM=landlock,[...]`锛屽叾涓?`[...]` 鏄繍琛岀郴缁熷叾瀹冨彲鑳芥湁鐢ㄧ殑瀹夊叏妯″潡鍒楄〃锛堝弬瑙?`CONFIG_LSM` 鐨勫府鍔╋級銆?
### 鍚姩鏃堕厤缃?

濡傛灉杩愯涓殑鍐呮牳鍦?`CONFIG_LSM` 涓病鏈?`landlock`锛屾垜浠彲浠ラ€氳繃鍦ㄥ紩瀵煎姞杞界▼搴忛厤缃腑灏?`lsm=landlock,[...]` 娣诲姞鍒?Documentation/admin-guide/kernel-parameters.rst 鏉ュ惎鐢?Landlock銆?
渚嬪锛屽鏋滃綋鍓嶇殑 built-in 閰嶇疆鏄細

```
    $ zgrep -h "^CONFIG_LSM=" "/boot/config-$(uname -r)" /proc/config.gz 2>/dev/null
    CONFIG_LSM="lockdown,yama,integrity,apparmor"
```

鈥︹€﹀苟涓斿鏋滃懡浠よ涔熶笉鍖呭惈 `landlock`锛?
```
    $ sed -n 's/.**\(\<lsm=\S\+\).**/\1/p' /proc/cmdline
    lsm=lockdown,yama,integrity,apparmor
```

鈥︹€︽垜浠簲褰撻厤缃紩瀵煎姞杞界▼搴忥紝璁剧疆涓€涓墿灞?`lsm` 鐨勫懡浠よ锛?```

  lsm=landlock,lockdown,yama,integrity,apparmor

```
閲嶅惎涔嬪悗锛屾垜浠彲浠ラ€氳繃鏌ョ湅鍐呮牳鏃ュ織鏉ョ‘璁?Landlock 宸插惎鍔ㄥ苟杩愯锛?
```
    # dmesg | grep landlock || journalctl -kb -g landlock
    [    0.000000] Command line: [...] lsm=landlock,lockdown,yama,integrity,apparmor
    [    0.000000] Kernel command line: [...] lsm=landlock,lockdown,yama,integrity,apparmor
    [    0.000000] LSM: initializing lsm=lockdown,capability,landlock,yama,integrity,apparmor
    [    0.000000] landlock: Up and running.
```

鍐呮牳鍙兘鍦ㄦ瀯寤烘椂琚厤缃负鎬绘槸鍔犺浇 `lockdown` 涓?`capability` LSM銆傚湪杩欑鎯呭喌涓嬶紝鍗充究瀹冧滑娌℃湁鍦ㄥ紩瀵煎姞杞界▼搴忎腑閰嶇疆锛岃繖浜?LSM 涔熶細鍑虹幇鍦?`LSM: initializing` 鏃ュ織琛屽紑澶淬€?
### 缃戠粶鏀寔


涓轰簡鑳藉鏄惧紡鍏佽 TCP 鎿嶄綔锛堜緥濡傜敤 `LANDLOCK_ACCESS_NET_BIND_TCP` 娣诲姞缃戠粶瑙勫垯锛夛紝鍐呮牳蹇呴』鏀寔 TCP锛坄CONFIG_INET=y`锛夈€傚惁鍒欙紝sys_landlock_add_rule() 浼氳繑鍥炰竴涓?`EAFNOSUPPORT` 閿欒锛屽彲浠ュ畨鍏ㄥ湴蹇界暐瀹冿紝鍥犱负杩欑被 TCP 鎿嶄綔鏈潵灏变笉鍙兘銆?
## 闂瓟


### 鐢ㄦ埛绌洪棿娌欑绠＄悊鍣ㄥ憿锛?

浣跨敤鐢ㄦ埛绌洪棿杩涚▼瀵瑰唴鏍歌祫婧愬疄鏂介檺鍒跺彲鑳藉鑷寸珵鎬佹潯浠舵垨涓嶄竴鑷寸殑璇勪及锛堝嵆 `Incorrect mirroring of the OS code and state <https://www.ndss-symposium.org/ndss2003/traps-and-pitfalls-practical-problems-system-call-interposition-based-security-tools/>`_锛夈€?
### 鍛藉悕绌洪棿涓庡鍣ㄥ憿锛?

鍛藉悕绌洪棿鏈夊姪浜庡垱寤烘矙绠憋紝浣嗗畠浠苟闈炰负璁块棶鎺у埗鑰岃璁★紝鍥犺€岀己灏戞绫荤敤渚嬫墍闇€鐨勬湁鐢ㄧ壒鎬э紙渚嬪娌℃湁缁嗙矑搴︾殑闄愬埗锛夈€傛澶栵紝瀹冧滑鐨勫鏉傚害鍙兘瀵艰嚧瀹夊叏闂锛屽挨鍏舵槸褰撲笉鍙俊杩涚▼鍙互鎿嶇旱瀹冧滑鏃讹紙鍙傝 `Controlling access to user namespaces <https://lwn.net/Articles/673597/>`_锛夈€?
### 濡備綍绂佺敤 Landlock 瀹¤璁板綍锛?

浣犲彲鑳芥兂鎸夋澶勮鏄庤缃繃婊ゅ櫒锛?Documentation/admin-guide/LSM/landlock.rst

## 棰濆鏂囨。


- Documentation/admin-guide/LSM/landlock.rst
- Documentation/security/landlock.rst
- https://landlock.io

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/samples/landlock/sandboxer.c
