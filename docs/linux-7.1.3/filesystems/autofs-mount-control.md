
## autofs 鍐呮牳妯″潡鐨勬潅椤硅澶囨帶鍒舵搷浣?

## 闂


autofs 鍦ㄦ椿鍔ㄩ噸鍚紙鍗冲湪瀛樺湪绻佸繖鎸傝浇鏃堕噸鍚?autofs锛夋柟闈㈠瓨鍦ㄤ竴涓棶棰樸€?
鍦ㄦ甯告搷浣滀腑锛宎utofs 浣跨敤鍦ㄥ彈绠＄洰褰曚笂鎵撳紑鐨勬枃浠舵弿杩扮锛屼互渚胯兘澶熷彂鍑烘帶鍒舵搷浣溿€?浣跨敤鏂囦欢鎻忚堪绗﹁ ioctl 鎿嶄綔鑳藉璁块棶瀛樺偍鍦ㄨ秴绾у潡涓殑 autofs 鐗瑰畾淇℃伅銆傝繖浜涙搷浣?鍖呮嫭灏?autofs 鎸傝浇璁剧疆涓?catatonic锛堝兊姝伙級銆佽缃繃鏈熻秴鏃朵互鍙婅姹傝繃鏈熸鏌ャ€傛濡?涓嬮潰鎵€瑙ｉ噴鐨勶紝鏌愪簺绫诲瀷鐨?autofs 瑙﹀彂鎸傝浇鏈€缁堝彲鑳戒細瑕嗙洊 autofs 鎸傝浇鏈韩锛岃繖灏变娇
寰楀鏋滄垜浠繕娌℃湁鎸佹湁涓€涓凡鎵撳紑鐨勬枃浠舵弿杩扮锛屽氨鏃犳硶浣跨敤 open(2) 鏉ヨ幏鍙栫敤浜庤繖浜?鎿嶄綔鐨勬枃浠舵弿杩扮銆?
鐩墠 autofs 鍦ㄩ噸鍚椂浣跨敤 "umount -l"锛堟儼鎬у嵏杞斤級鏉ユ竻鐞嗘椿鍔ㄦ寕杞姐€傝櫧鐒舵儼鎬у嵏杞藉湪
澶у鏁版儏鍐典笅鏈夋晥锛屼絾浠讳綍闇€瑕佸洖婧寕杞芥爲鏉ユ瀯閫犺矾寰勭殑鎿嶄綔锛屼緥濡?getcwd(2) 浠ュ強 proc
鏂囦欢绯荤粺 /proc/<pid>/cwd锛屽皢涓嶅啀宸ヤ綔锛屽洜涓烘瀯閫犺矾寰勬墍渚濇嵁鐨勬寕杞界偣宸茬粡浠庢寕杞芥爲涓?鑴辩銆?
autofs 鐪熸鐨勯棶棰樺湪浜庡畠鏃犳硶閲嶆柊杩炴帴鍒板凡鏈夌殑鎸傝浇銆備汉浠珛鍒讳細鎯冲埌锛屽彧瑕佸姞涓婇噸鏂?鎸傝浇 autofs 鏂囦欢绯荤粺鐨勮兘鍔涘氨鑳借В鍐筹紝浣嗛仐鎲剧殑鏄繖琛屼笉閫氥€傝繖鏄洜涓?autofs 鐩存帴鎸傝浇
浠ュ強宓屽鎸傝浇鏍?"鎸夐渶鎸傝浇涓庤繃鏈? 鐨勫疄鐜帮紝鏄皢鏂囦欢绯荤粺鐩存帴鎸傝浇鍦ㄦ寕杞借Е鍙戝櫒鐩綍鐨?dentry 涔嬩笂銆?
渚嬪锛岃嚜鍔ㄥ寲鎸傝浇鏄犲皠锛坅utomount map锛夋湁涓ょ绫诲瀷锛氱洿鎺ワ紙direct锛屽湪鍐呮牳妯″潡婧愮爜涓?浣犱細鐪嬪埌绗笁绉嶇О涓?offset 鐨勭被鍨嬶紝瀹冨彧鏄竴绉嶄吉瑁呯殑鐩存帴鎸傝浇锛夊拰闂存帴锛坕ndirect锛夈€?
```

    /-      /etc/auto.direct
    /test   /etc/auto.indirect

```
```

    /etc/auto.direct:

    /automount/dparse/g6  budgie:/autofs/export1
    /automount/dparse/g1  shark:/autofs/export1
    and so on.

```
```

    g1    shark:/autofs/export1
    g6    budgie:/autofs/export1
    and so on.

```
瀵逛簬涓婇潰鐨勯棿鎺ユ槧灏勶紝鍦?/test 涓婃寕杞戒簡涓€涓?autofs 鏂囦欢绯荤粺锛屽苟鐢?inode 鏌ユ壘鎿嶄綔涓?姣忎釜瀛愮洰褰曢敭瑙﹀彂鎸傝浇銆備緥濡傦紝鎴戜滑鐪嬪埌鍦?/test/g1 涓婃寕杞戒簡 shark:/autofs/export1銆?
鐩存帴鎸傝浇鐨勫鐞嗘柟寮忔槸鍦ㄦ瘡涓畬鏁磋矾寰勶紙濡?/automount/dparse/g1锛変笂寤虹珛涓€涓?autofs
鎸傝浇锛屽苟灏嗗叾鐢ㄤ綔鎸傝浇瑙﹀彂鍣ㄣ€傚洜姝ゅ綋鎴戜滑娌跨潃璺緞璧颁笅鍘绘椂锛屼細灏?shark:/autofs/export1
鎸傝浇鍒?"杩欎釜鎸傝浇鐐逛箣涓?銆傜敱浜庤繖浜涙案杩滈兘鏄洰褰曪紝鎴戜滑鍙互浣跨敤 follow_link inode
鎿嶄綔鏉ヨЕ鍙戞寕杞姐€?
浣嗘槸锛岀洿鎺ユ槧灏勫拰闂存帴鏄犲皠涓殑姣忎釜鏉＄洰閮藉彲浠ユ湁鍋忕Щ锛坥ffset锛夛紝浠庤€屽彉鎴愬鎸傝浇鏄犲皠
鏉＄洰銆?
```

    g1  \
    /        shark:/autofs/export5/testing/test \
    /s1      shark:/autofs/export/testing/test/s1 \
    /s2      shark:/autofs/export5/testing/test/s2 \
    /s1/ss1  shark:/autofs/export1 \
    /s2/ss2  shark:/autofs/export2

```
```

    /automount/dparse/g1 \
	/       shark:/autofs/export5/testing/test \
	/s1     shark:/autofs/export/testing/test/s1 \
	/s2     shark:/autofs/export5/testing/test/s2 \
	/s1/ss1 shark:/autofs/export2 \
	/s2/ss2 shark:/autofs/export2

```
autofs 绗?4 鐗堢殑涓€涓棶棰樻槸锛屽綋鎸傝浇涓€涓甫鏈夊ぇ閲忓亸绉伙紙鍙兘杩樺祵濂楋級鐨勬潯鐩椂锛屾垜浠?闇€瑕佹妸鎵€鏈夎繖浜涘亸绉讳綔涓轰竴涓暣浣撳崟鍏冩潵鎸傝浇鍜屽嵏杞姐€傝繖鏈韩涓嶆槸闂锛岄櫎浜嗗浜庨偅浜涘湪
鏄犲皠鏉＄洰涓湁澶ч噺鍋忕Щ鐨勭敤鎴枫€傝繖涓満鍒剁敤浜庝紬鎵€鍛ㄧ煡鐨?"hosts" 鏄犲皠锛屾垜浠凡缁忚杩?涓€浜涙儏鍐碉紙鍦?2.4 鍐呮牳涓級锛屽叾涓彲鐢ㄧ殑鎸傝浇鏁伴噺鎴栧彲鐢ㄧ殑鐗规潈绔彛鏁伴噺琚€楀敖銆?
鍦ㄧ 5 鐗堜腑锛屾垜浠彧鍦ㄦ部鍋忕Щ鏍戝悜涓嬭蛋鏃舵墠鎸傝浇瀹冧滑锛岃繃鏈熸椂鍚岀悊锛屼粠鑰岃В鍐充簡涓婅堪闂銆?瀹炵幇涓婅繕鏈変竴浜涙洿璇︾粏鐨勭粏鑺傦紝浣嗗氨瑙ｉ噴闂鑰岃█骞朵笉闇€瑕併€備竴涓噸瑕佺殑缁嗚妭鏄紝杩欎簺
鍋忕Щ浣跨敤涓庝笂杩扮洿鎺ユ寕杞界浉鍚岀殑鏈哄埗瀹炵幇锛屽洜姝ゆ寕杞界偣浼氳涓€涓寕杞芥墍瑕嗙洊銆?
褰撳墠鐨?autofs 瀹炵幇浣跨敤鍦ㄦ寕杞界偣涓婃墦寮€鐨?ioctl 鏂囦欢鎻忚堪绗﹁繘琛屾帶鍒舵搷浣溿€傝鎻忚堪绗?鎸佹湁鐨勫紩鐢ㄤ細鍦ㄥ垽鏂竴涓寕杞芥槸鍚﹀湪浣跨敤鏃惰璁″叆锛屼篃鐢ㄤ簬璁块棶淇濆瓨鍦ㄦ寕杞借秴绾у潡涓殑
autofs 鏂囦欢绯荤粺淇℃伅銆傚洜姝や粛闇€淇濈暀鏂囦欢鍙ユ焺鐨勪娇鐢ㄣ€?

## 瑙ｅ喅鏂规


涓轰簡鑳藉鍦ㄩ噸鍚?autofs 鏃惰宸叉湁鐨勭洿鎺ャ€侀棿鎺ュ拰鍋忕Щ鎸傝浇淇濇寔鍘熶綅锛屾垜浠渶瑕佽兘澶熻幏鍙?杩欎簺鍙兘琚鐩栫殑 autofs 鎸傝浇鐐圭殑鏂囦欢鍙ユ焺銆備笌鍏跺彧瀹炵幇涓€涓绔嬬殑鎿嶄綔锛屾垜浠喅瀹?閲嶆柊瀹炵幇鐜版湁鐨?ioctl 鎺ュ彛锛屽苟娣诲姞鏂扮殑鎿嶄綔鏉ユ彁渚涜繖涓姛鑳姐€?
姝ゅ锛屼负浜嗚兘澶熼噸寤哄甫鏈夌箒蹇欐寕杞界殑鎸傝浇鏍戯紝瑙﹀彂鎸傝浇鐨勬渶鍚庝竴鍚嶇敤鎴风殑 uid 鍜?gid
闇€瑕佸彲鐢紝鍥犱负瀹冧滑鍙互鐢ㄤ綔 autofs 鏄犲皠涓殑瀹忔浛鎹㈠彉閲忋€傚畠浠湪鎸傝浇璇锋眰鏃惰璁板綍锛?骞舵柊澧炰簡涓€涓搷浣滄潵妫€绱㈠畠浠€?
鐢变簬鎴戜滑閲嶆柊瀹炵幇鎺у埗鎺ュ彛锛岀幇鏈夋帴鍙ｄ笂鐨勫彟澶栧嚑涓棶棰樹篃寰楀埌浜嗚В鍐炽€傞鍏堬紝褰撲竴涓?鎸傝浇鎴栬繃鏈熸搷浣滃畬鎴愭椂锛屼細閫氳繃 "send ready锛堝彂閫佸氨缁級" 鎴?"send fail锛堝彂閫佸け璐ワ級"
鎿嶄綔鍚戝唴鏍歌繑鍥炰竴涓姸鎬併€俰octl 鎺ュ彛鐨?"send fail" 鎿嶄綔鍙兘鍙戦€?ENOENT锛屽洜姝ら噸鏂?瀹炵幇鍏佽鐢ㄦ埛绌洪棿鍙戦€佸疄闄呯殑鐘舵€併€傚浣跨敤瓒呭ぇ鏄犲皠鐨勭敤鎴锋潵璇达紝鍙︿竴涓紑閿€寰堝ぇ鐨勬搷浣?鏄垽鏂竴涓寕杞芥槸鍚﹀瓨鍦ㄣ€傝繖閫氬父娑夊強鎵弿 /proc/mounts锛岀敱浜庨渶瑕佺浉褰撻绻佸湴鎵ц锛?鍦ㄦ寕杞借〃涓湁澶ч噺鏉＄洰鏃朵細寮曞叆鏄捐憲鐨勯澶栧紑閿€銆傝繕鏂板浜嗕竴涓敤浜庢煡鎵炬寕杞界偣 dentry
锛堟棤璁烘槸鍚﹁瑕嗙洊锛夋寕杞界姸鎬佺殑鎿嶄綔銆?
褰撳墠鐨勫唴鏍稿紑鍙戠瓥鐣ュ缓璁伩鍏嶄娇鐢?ioctl 鏈哄埗锛岃浆鑰岄噰鐢?Netlink 涔嬬被鐨勭郴缁熴€傛垜浠?灏濊瘯鐢ㄨ绯荤粺鏉ュ疄鐜颁互璇勪及鍏堕€傜敤鎬э紝缁撴灉鍙戠幇瀹冨湪鏈満鏅笅骞朵笉鍚堥€傘€傝繖閲屼娇鐢ㄧ殑鏄?Generic Netlink 绯荤粺锛屽洜涓哄師濮嬬殑 Netlink 浼氭樉钁楀鍔犲鏉傚害銆傛鏃犵枒闂紝Generic
Netlink 绯荤粺瀵逛簬甯歌鎯呭喌涓嬬殑 ioctl 鍑芥暟鏄竴涓紭闆呯殑瑙ｅ喅鏂规锛屼絾瀹冨彲鑳藉苟涓嶆槸涓€涓?瀹屾暣鐨勬浛浠ｅ搧锛屽ぇ姒傛槸鍥犱负瀹冪殑涓昏鐩殑鏄綔涓轰竴涓秷鎭€荤嚎瀹炵幇锛岃€岄潪涓撻棬浣滀负 ioctl
鐨勬浛浠ｅ搧銆傝櫧鐒跺彲浠ョ粫杩囪繖涓€鐐癸紝浣嗘湁涓€涓【铏戝鑷翠簡涓嶄娇鐢ㄥ畠鐨勫喅瀹氥€傝繖灏辨槸瀹堟姢杩涚▼
涓殑 autofs 杩囨湡宸茬粡鍙樺緱杩囦簬澶嶆潅锛屽師鍥犲湪浜庢灇涓捐繃鏈熷€欓€夎€咃紝鍑犱箮鍒棤鍘熷洜锛屽彧鏄负浜?"璁℃暟" 闇€瑕佽皟鐢ㄨ繃鏈?ioctl 鐨勬鏁般€傝繖娑夊強鎵弿鎸傝浇琛紝瀵逛簬浣跨敤澶ф槧灏勭殑鐢ㄦ埛宸茶璇佹槑
鏄緢澶х殑寮€閿€銆傛敼杩涘畠鐨勬渶浣虫柟寮忔槸灏濊瘯鍥炲埌寰堜箙浠ュ墠杩囨湡鐨勬柟寮忋€備篃灏辨槸璇达紝褰撲负涓€涓?鎸傝浇锛堟枃浠跺彞鏌勶級鍙戝嚭杩囨湡璇锋眰鏃讹紝鎴戜滑搴旇鎸佺画鍥炶皟瀹堟姢杩涚▼锛岀洿鍒板啀涔熸棤娉曞嵏杞戒换浣?鎸傝浇锛岀劧鍚庢墠鍚戝畧鎶よ繘绋嬭繑鍥為€傚綋鐨勭姸鎬併€傜洰鍓嶆垜浠竴娆″彧杩囨湡涓€涓寕杞姐€侴eneric Netlink
鐨勫疄鐜颁細鍥犱负娑堟伅鎬荤嚎鏋舵瀯鐨勮姹傦紝鎺掗櫎鏈潵寮€鍙戜腑鐨勮繖绉嶅彲鑳芥€с€?

## autofs 鏉傞」璁惧鎸傝浇鎺у埗鎺ュ彛


鎺у埗鎺ュ彛鏄墦寮€涓€涓澶囪妭鐐癸紝閫氬父鏄?/dev/autofs銆?
鎵€鏈?ioctl 閮戒娇鐢ㄤ竴涓€氱敤缁撴瀯鏉ヤ紶閫掓墍闇€鍙傛暟
```

    struct autofs_dev_ioctl {
	    __u32 ver_major;
	    __u32 ver_minor;
	    __u32 size;             /* total size of data passed in
				    * including this struct */
	    __s32 ioctlfd;          /* automount command fd */

	    /* Command parameters */
	    union {
		    struct args_protover		protover;
		    struct args_protosubver		protosubver;
		    struct args_openmount		openmount;
		    struct args_ready		ready;
		    struct args_fail		fail;
		    struct args_setpipefd		setpipefd;
		    struct args_timeout		timeout;
		    struct args_requester		requester;
		    struct args_expire		expire;
		    struct args_askumount		askumount;
		    struct args_ismountpoint	ismountpoint;
	    };

	    char path[];
    };

```
ioctlfd 瀛楁鏄竴涓?autofs 鎸傝浇鐐圭殑鎸傝浇鐐规枃浠舵弿杩扮銆傚畠鐢?open 璋冪敤杩斿洖锛屽苟琚櫎
"鍒ゆ柇缁欏畾璺緞鏄惁涓烘寕杞界偣" 涔嬪鐨勬墍鏈夎皟鐢ㄦ墍浣跨敤锛屽湪璇ヨ皟鐢ㄤ腑鍙互鍙€夊湴浣跨敤瀹冩潵妫€鏌?瀵瑰簲浜庣粰瀹氭寕杞界偣鏂囦欢鎻忚堪绗︾殑鐗瑰畾鎸傝浇锛屼互鍙婂綋璇锋眰 autofs 鏂囦欢绯荤粺涓煇涓洰褰曞唴鏈€鍚?涓€娆℃垚鍔熸寕杞界殑 uid 鍜?gid 鏃躲€?
涓婇潰鎻忚堪鐨勮仈鍚堬紙union锛夌敤浜庝紶杈捐皟鐢ㄧ殑鍙傛暟鍜岀粨鏋溿€?
path 瀛楁鐢ㄤ簬鍦ㄩ渶瑕佺殑鍦版柟浼犻€掍竴涓矾寰勶紝size 瀛楁鐢ㄤ簬鍦ㄧ炕璇戜粠鐢ㄦ埛绌洪棿鍙戦€佺殑缁撴瀯鏃?璁″叆澧為暱鍚庣殑缁撴瀯闀垮害銆?
杩欎釜缁撴瀯鍙互鍦ㄨ缃壒瀹氬瓧娈典箣鍓嶏紝閫氳繃浣跨敤 void 鍑芥暟璋冪敤
init_autofs_dev_ioctl(`struct autofs_dev_ioctl *`) 鏉ュ垵濮嬪寲銆?
鎵€鏈?ioctl 閮戒細灏嗚繖涓粨鏋勪粠鐢ㄦ埛绌洪棿澶嶅埗鍒板唴鏍哥┖闂达紝濡傛灉 size 鍙傛暟灏忎簬缁撴瀯鏈韩
鐨勫ぇ灏忓垯杩斿洖 -EINVAL锛屽鏋滃唴鏍稿唴瀛樺垎閰嶅け璐ュ垯杩斿洖 -ENOMEM锛屽鏋滃鍒舵湰韬け璐ュ垯杩斿洖
-EFAULT銆傚叾浠栨鏌ュ寘鎷皢鐢ㄦ埛绌洪棿缂栬瘧杩涚殑鐗堟湰涓庢ā鍧楃増鏈繘琛岀増鏈牎楠岋紝涓嶅尮閰嶄細瀵艰嚧
杩斿洖 -EINVAL銆傚鏋?size 瀛楁澶т簬缁撴瀯澶у皬锛屽垯鍋囧畾瀛樺湪涓€涓矾寰勶紝骞舵鏌ュ畠鏄惁浠?"/"
寮€澶村苟浠?NULL 缁撳熬锛屽惁鍒欒繑鍥?-EINVAL銆傚湪杩欎簺妫€鏌ヤ箣鍚庯紝瀵逛簬鎵€鏈?ioctl 鍛戒护锛岄櫎浜?AUTOFS_DEV_IOCTL_VERSION_CMD銆丄UTOFS_DEV_IOCTL_OPENMOUNT_CMD 鍜?AUTOFS_DEV_IOCTL_CLOSEMOUNT_CMD 涔嬪锛岄兘浼氭牎楠?ioctlfd锛屽鏋滃畠涓嶆槸涓€涓湁鏁堢殑
鎻忚堪绗︽垨涓嶅搴斾竴涓?autofs 鎸傝浇鐐癸紝鍒欒繑鍥?-EBADF銆?ENOTTY 鎴?-EINVAL锛堜笉鏄竴涓?autofs 鎻忚堪绗︼級銆?

## ioctl 鍛戒护


浣跨敤璇ユ帴鍙ｇ殑瀹炵幇绀轰緥鍙互鍦?autofs 5.0.4 鍙婃洿楂樼増鏈腑鐪嬪埌锛屼綅浜庝粠 kernel.org 鐨?/pub/linux/daemons/autofs/v5 鐩綍鍙笅杞界殑鍙戣鐗?tar 鍖呬腑鐨勬枃浠?lib/dev-ioctl-lib.c銆?
璇ユ帴鍙ｅ疄鐜扮殑璁惧鑺傜偣 ioctl 鎿嶄綔濡備笅锛?

### AUTOFS_DEV_IOCTL_VERSION

鑾峰彇 autofs 璁惧 ioctl 鍐呮牳妯″潡瀹炵幇鐨勪富鐗堟湰鍙峰拰娆＄増鏈彿銆傚畠闇€瑕佷竴涓凡鍒濆鍖栫殑
struct autofs_dev_ioctl 浣滀负杈撳叆鍙傛暟锛屽苟灏嗙増鏈俊鎭缃埌浼犲叆鐨勭粨鏋勪腑銆傛垚鍔熸椂
杩斿洖 0锛屽鏋滄娴嬪埌鐗堟湰涓嶅尮閰嶅垯杩斿洖閿欒 -EINVAL銆?

### AUTOFS_DEV_IOCTL_PROTOVER_CMD 涓?AUTOFS_DEV_IOCTL_PROTOSUBVER_CMD

鑾峰彇宸插姞杞芥ā鍧楁墍鐞嗚В鐨?autofs 鍗忚鐗堟湰鐨勪富鐗堟湰鍙峰拰娆＄増鏈彿銆傝璋冪敤闇€瑕佷竴涓凡
鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰竴涓湁鏁堢殑 autofs 鎸傝浇鐐?鎻忚堪绗︼紝骞跺皢璇锋眰鐨勭増鏈彿璁剧疆鍒?struct args_protover 鐨?version 瀛楁鎴?struct args_protosubver 鐨?sub_version 瀛楁涓€傝繖浜涘懡浠ゆ垚鍔熸椂杩斿洖 0锛屽鏋滄牎楠?澶辫触鍒欒繑鍥炴煇涓礋鐨勯敊璇爜銆?

### AUTOFS_DEV_IOCTL_OPENMOUNT 涓?AUTOFS_DEV_IOCTL_CLOSEMOUNT

鑾峰彇骞堕噴鏀句竴涓?autofs 鍙楃鎸傝浇鐐硅矾寰勭殑鏂囦欢鎻忚堪绗︺€俹pen 璋冪敤闇€瑕佷竴涓凡鍒濆鍖栫殑
struct autofs_dev_ioctl锛屽叾 path 瀛楁宸茶缃€乻ize 瀛楁宸查€傚綋璋冩暣锛屼笖
struct args_openmount 鐨?devid 瀛楁璁剧疆涓?autofs 鎸傝浇鐨勮澶囧彿銆傝澶囧彿鍙互浠?/proc/mounts 涓樉绀虹殑鎸傝浇閫夐」涓幏寰椼€俢lose 璋冪敤闇€瑕佷竴涓凡鍒濆鍖栫殑
struct autofs_dev_ioct锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open 璋冪敤鑾峰緱鐨勬弿杩扮銆傛枃浠?鎻忚堪绗︾殑閲婃斁涔熷彲浠ョ敤 close(2) 瀹屾垚锛屽洜姝や换浣曟墦寮€鐨勬弿杩扮涔熶細鍦ㄨ繘绋嬮€€鍑烘椂琚叧闂€?close 璋冪敤琚寘鍚湪宸插疄鐜扮殑鎿嶄綔涓紝寰堝ぇ绋嬪害涓婃槸涓轰簡瀹屾暣鎬э紝骞朵负涓€鑷寸殑鐢ㄦ埛绌洪棿
瀹炵幇鎻愪緵鏀寔銆?

### AUTOFS_DEV_IOCTL_READY_CMD 涓?AUTOFS_DEV_IOCTL_FAIL_CMD

浠庣敤鎴风┖闂村悜鍐呮牳杩斿洖鎸傝浇鍜岃繃鏈熺粨鏋滅姸鎬併€傝繖涓や釜璋冪敤閮介渶瑕佷竴涓凡鍒濆鍖栫殑
struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open 璋冪敤鑾峰緱鐨勬弿杩扮锛屼笖
struct args_ready 鎴?struct args_fail 鐨?token 瀛楁璁剧疆涓虹瓑寰呴槦鍒椾护鐗屽彿锛岃浠ょ墝
鍙风敱鐢ㄦ埛绌洪棿鍦ㄥ墠杩版寕杞芥垨杩囨湡璇锋眰涓敹鍒般€俿truct args_fail 鐨?status 瀛楁琚缃负
鎿嶄綔鐨?errno銆傛垚鍔熸椂璁剧疆涓?0銆?

### AUTOFS_DEV_IOCTL_SETPIPEFD_CMD

璁剧疆鐢ㄤ簬鍐呮牳涓庡畧鎶よ繘绋嬮€氫俊鐨勭閬撴枃浠舵弿杩扮銆傞€氬父杩欏湪鎸傝浇鏃堕€氳繃閫夐」璁剧疆锛屼絾鍦?閲嶆柊杩炴帴宸叉湁鎸傝浇鏃讹紝鎴戜滑闇€瑕佷娇鐢ㄥ畠鏉ュ憡鐭?autofs 鎸傝浇鏂扮殑鍐呮牳绠￠亾鎻忚堪绗︺€備负浜嗕繚鎶?鎸傝浇涓嶈閿欒鍦拌缃閬撴弿杩扮锛屾垜浠繕瑕佹眰 autofs 鎸傝浇澶勪簬 catatonic 鐘舵€侊紙瑙佷笅涓€涓?璋冪敤锛夈€?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open
璋冪敤鑾峰緱鐨勬弿杩扮锛屼笖 struct args_setpipefd 鐨?pipefd 瀛楁璁剧疆涓鸿绠￠亾鐨勬弿杩扮銆?鎴愬姛鏃惰璋冪敤杩樺皢鐢ㄤ簬鏍囪瘑鎺у埗杩涚▼锛堜緥濡傛嫢鏈夌殑 automount(8) 瀹堟姢杩涚▼锛夌殑杩涚▼缁?id
璁剧疆涓鸿皟鐢ㄨ€呯殑杩涚▼缁勩€?

### AUTOFS_DEV_IOCTL_CATATONIC_CMD

浣?autofs 鎸傝浇鐐硅繘鍏?catatonic 鐘舵€併€俛utofs 鎸傝浇灏嗕笉鍐嶅彂鍑烘寕杞借姹傦紝鍐呮牳閫氫俊绠￠亾
鎻忚堪绗﹁閲婃斁锛岄槦鍒椾腑浠讳綍鍓╀綑鐨勭瓑寰呬篃琚噴鏀俱€?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open
璋冪敤鑾峰緱鐨勬弿杩扮銆?

### AUTOFS_DEV_IOCTL_TIMEOUT_CMD

璁剧疆 autofs 鎸傝浇鐐瑰唴鎸傝浇鐨勮繃鏈熻秴鏃躲€?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open
璋冪敤鑾峰緱鐨勬弿杩扮銆?

### AUTOFS_DEV_IOCTL_REQUESTER_CMD

杩斿洖鏈€鍚庢垚鍔熷湪缁欏畾璺緞 dentry 涓婅Е鍙戞寕杞界殑杩涚▼鐨?uid 鍜?gid銆?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 path 瀛楁璁剧疆涓虹浉鍏虫寕杞界偣锛?涓?size 瀛楁宸查€傚綋璋冩暣銆傝繑鍥炴椂锛宻truct args_requester 鐨?uid 瀛楁鍖呭惈 uid锛実id
瀛楁鍖呭惈 gid銆?
鍦ㄩ噸寤哄甫鏈夋椿鍔ㄦ寕杞界殑 autofs 鎸傝浇鏍戞椂锛屾垜浠渶瑕侀噸鏂拌繛鎺ュ埌閭ｄ簺鍙兘浣跨敤浜嗗師濮嬭繘绋?uid 鍜?gid锛堟垨瀹冧滑鐨勫瓧绗︿覆鍙樹綋锛夎繘琛屾槧灏勬潯鐩唴鎸傝浇鏌ユ壘鐨勬寕杞姐€傝繖涓皟鐢ㄦ彁渚涗簡鑾峰彇
璇?uid 鍜?gid 鐨勮兘鍔涳紝浠ヤ究鐢ㄦ埛绌洪棿鍦ㄦ寕杞芥槧灏勬煡鎵炬椂浣跨敤瀹冧滑銆?

### AUTOFS_DEV_IOCTL_EXPIRE_CMD

鍚戝唴鏍稿彂鍑轰竴涓拡瀵?autofs 鎸傝浇鐨勮繃鏈熻姹傘€傞€氬父杩欎釜 ioctl 浼氳鍙嶅璋冪敤锛岀洿鍒版壘涓嶅埌
鏇村杩囨湡鍊欓€夎€呫€?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open
璋冪敤鑾峰緱鐨勬弿杩扮銆傛澶栵紝鍙互閫氳繃鎶?struct args_expire 鐨?how 瀛楁璁剧疆涓?AUTOFS_EXP_IMMEDIATE 鎴?AUTOFS_EXP_FORCED锛屽垎鍒姹傜嫭绔嬩簬鎸傝浇瓒呮椂鐨勭珛鍗宠繃鏈熷拰
鐙珛浜庢寕杞芥槸鍚︾箒蹇欑殑寮哄埗杩囨湡銆傚鏋滄壘涓嶅埌杩囨湡鍊欓€夎€咃紝璇?ioctl 杩斿洖 -1 涓?errno
琚缃负 EAGAIN銆?
杩欎釜璋冪敤浣垮唴鏍告ā鍧楁鏌ュ搴旂粰瀹?ioctlfd 鐨勬寕杞戒腑鍙繃鏈熺殑鎸傝浇锛屽悜瀹堟姢杩涚▼鍙戝嚭杩囨湡
璇锋眰骞剁瓑寰呭叾瀹屾垚銆?

### AUTOFS_DEV_IOCTL_ASKUMOUNT_CMD

妫€鏌ヤ竴涓?autofs 鎸傝浇鐐规槸鍚﹀湪浣跨敤涓€?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl锛屽叾 ioctlfd 瀛楁璁剧疆涓轰粠 open
璋冪敤鑾峰緱鐨勬弿杩扮锛屽畠灏嗙粨鏋滆繑鍥炲埌 struct args_askumount 鐨?may_umount 瀛楁锛? 琛ㄧず
绻佸繖锛? 琛ㄧず鍚﹀垯銆?

### AUTOFS_DEV_IOCTL_ISMOUNTPOINT_CMD

妫€鏌ョ粰瀹氳矾寰勬槸鍚︿负鎸傝浇鐐广€?
璇ヨ皟鐢ㄩ渶瑕佷竴涓凡鍒濆鍖栫殑 struct autofs_dev_ioctl銆傛湁涓ょ鍙兘鐨勫彉浣撱€備袱鑰呴兘浣跨敤
path 瀛楁璁剧疆涓鸿妫€鏌ョ殑鎸傝浇鐐硅矾寰勶紝涓?size 瀛楁宸查€傚綋璋冩暣銆備竴绉嶄娇鐢?ioctlfd 瀛楁
鏉ユ爣璇嗚妫€鏌ョ殑鍏蜂綋鎸傝浇鐐癸紝鍙︿竴绉嶅彉浣撲娇鐢?path 骞跺彲閫夊湴鐢?struct args_ismountpoint
鐨?in.type 瀛楁璁剧疆涓烘煇涓?autofs 鎸傝浇绫诲瀷銆傚鏋滆璺緞鏄寕杞界偣锛岃皟鐢ㄨ繑鍥?1锛屽苟灏?out.devid 瀛楁璁剧疆涓鸿鎸傝浇鐨勮澶囧彿銆乷ut.magic 瀛楁璁剧疆涓虹浉鍏崇殑瓒呯骇鍧楅瓟鏁帮紙濡備笅
鎵€杩帮級锛屽惁鍒欒繑鍥?0銆傚湪涓ょ鎯呭喌涓嬶紝璁惧鍙凤紙鐢?new_encode_dev() 杩斿洖锛夐兘浼氳杩斿洖鍒?out.devid 瀛楁銆?
濡傛灉鎻愪緵浜嗕竴涓枃浠舵弿杩扮锛屾垜浠槸鍦ㄦ煡鎵句竴涓壒瀹氱殑鎸傝浇锛屼笉涓€瀹氫綅浜庢寕杞芥爤鐨勯《绔€?鍦ㄨ繖绉嶆儏鍐典笅锛屽鏋滆鎻忚堪绗﹀搴旂殑璺緞鏈韩鏄竴涓寕杞界偣锛屾垨鍖呭惈涓€涓寕杞斤紙渚嬪娌℃湁
鏍规寕杞界殑澶氭寕杞斤級锛屽垯琚涓烘寕杞界偣銆傚湪杩欑鎯呭喌涓嬶紝濡傛灉鎻忚堪绗﹀搴斾竴涓寕杞界偣锛屾垜浠?杩斿洖 1锛屽苟鍦ㄥ瓨鍦ㄨ鐩栨寕杞芥椂杩斿洖璇ヨ鐩栨寕杞界殑瓒呯骇鍧楅瓟鏁帮紝鍚﹀垯杩斿洖 0銆?
濡傛灉鎻愪緵浜嗕竴涓矾寰勶紙涓?ioctlfd 瀛楁璁剧疆涓?-1锛夛紝鍒欐煡鎵捐璺緞骞舵鏌ュ畠鏄惁鏄竴涓?鎸傝浇鐨勬牴銆傚鏋滆繕缁欏畾浜嗕竴涓被鍨嬶紝鎴戜滑鏄湪鏌ユ壘涓€涓壒瀹氱殑 autofs 鎸傝浇锛屽鏋滄壘涓嶅埌
鍖归厤鍒欒繑鍥炲け璐ャ€傚鏋滃畾浣嶅埌鐨勮矾寰勬槸涓€涓寕杞界殑鏍癸紝鍒欒繑鍥?1 浠ュ強璇ユ寕杞界殑瓒呯骇鍧楅瓟鏁帮紝
鍚﹀垯杩斿洖 0銆?