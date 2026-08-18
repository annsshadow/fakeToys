## Linux 涓殑绾㈤粦鏍戯紙rbtree锛?

:Date: January 18, 2007
:Author: Rob Landley <rob@landley.net>

### 浠€涔堟槸绾㈤粦鏍戯紝瀹冧滑鏈変粈涔堢敤閫旓紵


绾㈤粦鏍戞槸涓€绉嶈嚜骞宠　鐨勪簩鍙夋悳绱㈡爲锛岀敤浜庡瓨鍌ㄥ彲鎺掑簭鐨勯敭/鍊兼暟鎹銆傚畠涓庡熀鏁版爲锛堢敤浜庨珮鏁堝瓨鍌ㄧ█鐤忔暟缁勶紝鍥犺€屼娇鐢ㄩ暱鏁村瀷绱㈠紩鏉ユ彃鍏?璁块棶/鍒犻櫎鑺傜偣锛夊拰鍝堝笇琛紙涓嶄繚鎸佹湁搴忎互渚挎寜椤哄簭閬嶅巻锛屼笖蹇呴』閽堝鐗瑰畾澶у皬鍜屽搱甯屽嚱鏁拌皟浼橈紝鑰?rbtree 鍦ㄥ瓨鍌ㄤ换鎰忛敭鏃跺彲浼橀泤鍦版墿灞曪級涓嶅悓銆?
绾㈤粦鏍戜笌 AVL 鏍戠被浼硷紝浣嗗湪鎻掑叆鍜屽垹闄ゆ椂鎻愪緵鏇村揩閫熴€佸疄鏃舵湁鐣岀殑 worst case 鎬ц兘锛堝垎鍒渶澶氫袱娆″拰涓夋鏃嬭浆鏉ュ钩琛℃爲锛夛紝鏌ユ壘鏃堕棿绋嶆參锛堜絾浠嶆槸 O(log n)锛夈€?
寮曠敤 Linux Weekly News 鐨勮娉曪細

    鍐呮牳涓娇鐢ㄤ簡鑻ュ共绾㈤粦鏍戙€俤eadline 鍜?CFQ I/O 璋冨害鍣ㄥ埄鐢?rbtree 鏉ヨ窡韪姹傦紱鏁版嵁鍖?CD/DVD 椹卞姩涔熻繖鏍峰仛銆傞珮绮惧害瀹氭椂鍣ㄤ唬鐮佷娇鐢?rbtree 鏉ョ粍缁囧緟澶勭悊鐨勫畾鏃跺櫒璇锋眰銆俥xt3 鏂囦欢绯荤粺鐢ㄧ孩榛戞爲璺熻釜鐩綍椤广€傝櫄鎷熷唴瀛樺尯鍩燂紙VMA锛夌敤绾㈤粦鏍戣窡韪紝epoll 鏂囦欢鎻忚堪绗︺€佸姞瀵嗗瘑閽ヤ互鍙娾€滃垎灞備护鐗屾《锛坔ierarchical token bucket锛夆€濊皟搴﹀櫒涓殑缃戠粶鏁版嵁鍖呬篃鏄姝ゃ€?
鏈枃妗ｄ粙缁?Linux rbtree 瀹炵幇鐨勪娇鐢ㄦ柟娉曘€傛湁鍏崇孩榛戞爲鏈川涓庡疄鐜扮殑鏇村淇℃伅锛岃鍙傞槄锛?
  Linux Weekly News 涓婂叧浜庣孩榛戞爲鐨勬枃绔?    https://lwn.net/Articles/184495/

  缁村熀鐧剧涓婂叧浜庣孩榛戞爲鐨勬潯鐩?    https://en.wikipedia.org/wiki/Red-black_tree

### 绾㈤粦鏍戠殑 Linux 瀹炵幇


Linux 鐨?rbtree 瀹炵幇浣嶄簬 "lib/rbtree.c" 鏂囦欢涓€備娇鐢ㄥ畠闇€瑕?"#include <linux/rbtree.h>"銆?
Linux 鐨?rbtree 瀹炵幇閽堝閫熷害杩涜浜嗕紭鍖栵紝鍥犳姣旀洿浼犵粺鐨勬爲瀹炵幇灏戜簡涓€灞傞棿鎺ワ紙骞跺叿鏈夋洿濂界殑缂撳瓨灞€閮ㄦ€э級銆傚畠涓嶄娇鐢ㄦ寚鍚戠嫭绔嬬殑 rb_node 涓庢暟鎹粨鏋勪綋鐨勬寚閽堬紝鑰屾槸灏嗘瘡涓?struct rb_node 瀹炰緥宓屽叆鍒板畠鎵€缁勭粐鐨勬暟鎹粨鏋勪綋涓€傝€屼笖锛屽畠涓嶄娇鐢ㄦ瘮杈冨洖璋冨嚱鏁版寚閽堬紝鑰屾槸瑕佹眰鐢ㄦ埛鑷缂栧啓璋冪敤鎵€鎻愪緵 rbtree 鍑芥暟鐨勬爲鏌ユ壘涓庢彃鍏ュ嚱鏁般€傚姞閿佷篃鐢?rbtree 浠ｇ爜鐨勪娇鐢ㄨ€呰礋璐ｃ€?
### 鍒涘缓涓€涓柊鐨?rbtree


```

  struct mytype {
  	struct rb_node node;
  	char *keystring;
  };

```
褰撳鐞嗘寚鍚戝唴宓?struct rb_node 鐨勬寚閽堟椂锛屽彲浠ヤ娇鐢ㄦ爣鍑嗙殑 container_of() 瀹忚闂叾鎵€鍦ㄧ殑鏁版嵁缁撴瀯浣撱€傛澶栵紝涔熷彲浠ラ€氳繃 rb_entry(node, type, member) 鐩存帴璁块棶鍚勪釜鎴愬憳銆?
姣忎釜 rbtree 鐨勬牴閮ㄩ兘鏄竴涓?rb_root 缁撴瀯浣擄紝鍙€氳繃浠ヤ笅鏂瑰紡鍒濆鍖栦负绌猴細

  struct rb_root mytree = RB_ROOT;

### 鍦?rbtree 涓煡鎵惧€?

涓轰綘鐨勬爲缂栧啓鏌ユ壘鍑芥暟鐩稿綋鐩存帴锛氫粠鏍瑰紑濮嬶紝姣旇緝姣忎釜鍊硷紝骞舵寜闇€娌垮乏鍒嗘敮鎴栧彸鍒嗘敮鍚戜笅銆?
```

  struct mytype *my_search(struct rb_root *root, char *string)
  {
  	struct rb_node *node = root->rb_node;

  	while (node) {
  		struct mytype *data = container_of(node, struct mytype, node);
		int result;

		result = strcmp(string, data->keystring);

		if (result < 0)
  			node = node->rb_left;
		else if (result > 0)
  			node = node->rb_right;
		else
  			return data;
	}
	return NULL;
  }

```
### 鍚?rbtree 涓彃鍏ユ暟鎹?

鍚戞爲涓彃鍏ユ暟鎹紝棣栧厛闇€鏌ユ壘鏂拌妭鐐圭殑鎻掑叆浣嶇疆锛岀劧鍚庢彃鍏ヨ鑺傜偣骞跺鏍戦噸鏂板钩琛★紙鈥滈噸鏂扮潃鑹测€濓級銆?
鎻掑叆鏃剁殑鏌ユ壘涓庡墠杩版煡鎵剧殑涓嶅悓涔嬪鍦ㄤ簬锛氳鎵惧埌鐢ㄤ簬瀚佹帴鏂拌妭鐐圭殑鎸囬拡鎵€鍦ㄤ綅缃€傛柊鑺傜偣杩橀渶瑕佷竴涓寚鍚戝叾鐖惰妭鐐圭殑閾炬帴锛屼互渚胯繘琛岄噸鏂板钩琛°€?
```

  int my_insert(struct rb_root *root, struct mytype *data)
  {
  	struct rb_node **new = &(root->rb_node), *parent = NULL;

  	/* Figure out where to put new node */
  	while (*new) {
  		struct mytype *this = container_of(*new, struct mytype, node);
  		int result = strcmp(data->keystring, this->keystring);

		parent = *new;
  		if (result < 0)
  			new = &((*new)->rb_left);
  		else if (result > 0)
  			new = &((*new)->rb_right);
  		else
  			return FALSE;
  	}

  	/* Add new node and rebalance tree. */
  	rb_link_node(&data->node, parent, new);
  	rb_insert_color(&data->node, root);

	return TRUE;
  }

```
### 浠?rbtree 涓垹闄ゆ垨鏇挎崲宸叉湁鏁版嵁


```

  void rb_erase(struct rb_node *victim, struct rb_root *tree);

```
```

  struct mytype *data = mysearch(&mytree, "walrus");

  if (data) {
  	rb_erase(&data->node, &mytree);
  	myfree(data);
  }

```
```

  void rb_replace_node(struct rb_node *old, struct rb_node *new,
  			struct rb_root *tree);

```
浠ヨ繖绉嶆柟寮忔浛鎹㈣妭鐐逛笉浼氬鏍戦噸鏂版帓搴忥細濡傛灉鏂拌妭鐐圭殑閿笌鏃ц妭鐐逛笉鍚岋紝rbtree 寰堝彲鑳借鐮村潖銆?
### 閬嶅巻 rbtree 涓瓨鍌ㄧ殑鍏冪礌锛堟寜鎺掑簭椤哄簭锛?

鎻愪緵浜嗗洓涓嚱鏁帮紝鐢ㄤ簬鎸夋帓搴忛『搴忛亶鍘?rbtree 鐨勫唴瀹广€傚畠浠€傜敤浜庝换鎰忔爲锛岄€氬父涓嶉渶瑕?```

  struct rb_node *rb_first(struct rb_root *tree);
  struct rb_node *rb_last(struct rb_root *tree);
  struct rb_node *rb_next(struct rb_node *node);
  struct rb_node *rb_prev(struct rb_node *node);

```
瑕佸紑濮嬮亶鍘嗭紝浣跨敤鎸囧悜鏍戞牴鐨勬寚閽堣皟鐢?rb_first() 鎴?rb_last()锛屽畠浠細杩斿洖鎸囧悜鏍戜腑绗竴涓垨鏈€鍚庝竴涓厓绱犳墍鍖呭惈鑺傜偣缁撴瀯浣撶殑鎸囬拡銆傝缁х画閬嶅巻锛屽彲鍦ㄥ綋鍓嶈妭鐐逛笂璋冪敤 rb_next() 鎴?rb_prev() 鑾峰彇涓嬩竴涓垨涓婁竴涓妭鐐广€傚綋娌℃湁鏇村鑺傜偣鏃讹紝灏嗚繑鍥?NULL銆?
杩欎簺杩唬鍑芥暟杩斿洖鎸囧悜鍐呭祵 struct rb_node 鐨勬寚閽堬紝鍙€熷姪 container_of() 瀹忚闂叾鎵€鍦ㄧ殑鏁版嵁缁撴瀯浣擄紝涔熷彲閫氳繃 rb_entry(node, type, member) 鐩存帴璁块棶鍚勪釜鎴愬憳銆?
```

  struct rb_node *node;
  for (node = rb_first(&mytree); node; node = rb_next(node))
	printk("key=%s\n", rb_entry(node, struct mytype, node)->keystring);

```
### 甯︾紦瀛樼殑 rbtree


璁＄畻鏈€宸︼紙鏈€灏忥級鑺傜偣鏄簩鍙夋悳绱㈡爲涓浉褰撳父瑙佺殑浠诲姟锛屼緥濡傜敤浜庨亶鍘嗭紝鎴栫敤浜庝緷璧栫壒瀹氶『搴忕殑鑷韩閫昏緫銆備负姝わ紝鐢ㄦ埛鍙互浣跨敤 'struct rb_root_cached' 灏?O(logN) 鐨?rb_first() 璋冪敤浼樺寲涓轰竴娆＄畝鍗曠殑鎸囬拡鑾峰彇锛屼粠鑰岄伩鍏嶅彲鑳戒唬浠烽珮鏄傜殑鏍戦亶鍘嗐€傝繖鏍峰仛甯︽潵鐨勭淮鎶よ繍琛屾椂寮€閿€鍙拷鐣ヤ笉璁★紝浣嗕細鍗犵敤鏇村ぇ鐨勫唴瀛樸€?
涓?rb_root 缁撴瀯浣撶被浼硷紝甯︾紦瀛樼殑 rbtree 閫氳繃浠ヤ笅鏂瑰紡鍒濆鍖栦负绌猴細
```

  struct rb_root_cached mytree = RB_ROOT_CACHED;

```
甯︾紦瀛樼殑 rbtree 鍙槸涓€涓櫘閫氱殑 rb_root锛岄澶栧甫鏈変竴涓敤浜庣紦瀛樻渶宸﹁妭鐐圭殑鎸囬拡銆傝繖浣垮緱 rb_root_cached 鍙互鍑虹幇鍦?rb_root 鑳藉嚭鐜扮殑浠讳綍鍦版柟锛屼粠鑰屾棦鏀寔澧炲己鍨嬫爲锛屼篃鍙渶灏戦噺棰濆
```

  struct rb_node *rb_first_cached(struct rb_root_cached *tree);
  void rb_insert_color_cached(struct rb_node *, struct rb_root_cached *, bool);
  void rb_erase_cached(struct rb_node *node, struct rb_root_cached *);

```
鎻掑叆鍜屽垹闄よ皟鐢ㄩ兘鏈夊悇鑷搴旂殑澧炲己鍨?```

  void rb_insert_augmented_cached(struct rb_node *node, struct rb_root_cached *,
				  bool, struct rb_augment_callbacks *);
  void rb_erase_augmented_cached(struct rb_node *, struct rb_root_cached *,
				 struct rb_augment_callbacks *);


```
### 瀵瑰寮哄瀷 rbtree 鐨勬敮鎸?

澧炲己鍨?rbtree 鏄湪姣忎釜鑺傜偣涓瓨鍌ㄢ€滀竴浜涒€濋澶栨暟鎹殑 rbtree锛屽叾涓妭鐐?N 鐨勯澶栨暟鎹繀椤绘槸 N 涓烘牴鐨勫瓙鏍戜腑鎵€鏈夎妭鐐瑰唴瀹圭殑鍑芥暟銆傝繖浜涙暟鎹彲鐢ㄤ簬涓?rbtree 澧炲己涓€浜涙柊鍔熻兘銆傚寮哄瀷 rbtree 鏄瀯寤哄湪鍩虹 rbtree 鍩虹璁炬柦涔嬩笂鐨勫彲閫夌壒鎬с€傛兂瑕佷娇鐢ㄨ鐗规€х殑 rbtree 浣跨敤鑰咃紝蹇呴』鍦ㄦ彃鍏ュ拰鍒犻櫎鑺傜偣鏃讹紝閰嶅悎鐢ㄦ埛鎻愪緵澧炲己鍥炶皟鏉ヨ皟鐢ㄥ寮哄嚱鏁般€?
瀹炵幇澧炲己鍨?rbtree 鎿嶄綔鐨?C 鏂囦欢蹇呴』鍖呭惈 <linux/rbtree_augmented.h> 鑰屼笉鏄?<linux/rbtree.h>銆傝娉ㄦ剰锛宭inux/rbtree_augmented.h 鏆撮湶浜嗕竴浜涗綘涓嶅簲渚濊禆鐨?rbtree 瀹炵幇缁嗚妭锛涜鍙娇鐢ㄥ叾涓凡鏂囨。鍖栫殑 API锛屽苟涓斾篃涓嶈鍦ㄥご鏂囦欢涓寘鍚?<linux/rbtree_augmented.h>锛屼互灏介噺闄嶄綆浣犵殑浣跨敤鑰呮剰澶栦緷璧栬繖浜涘疄鐜扮粏鑺傜殑鍙兘鎬с€?
鎻掑叆鏃讹紝浣跨敤鑰呭繀椤绘洿鏂伴€氬悜琚彃鍏ヨ妭鐐圭殑璺緞涓婄殑澧炲己淇℃伅锛岀劧鍚庡儚寰€甯镐竴鏍疯皟鐢?rb_link_node()锛屽苟浣跨敤 rb_augment_inserted() 浠ｆ浛閫氬父鐨?rb_insert_color() 璋冪敤銆傚鏋?rb_augment_inserted() 閲嶆柊骞宠　浜?rbtree锛屽畠浼氬洖璋冪敤鎴锋彁渚涚殑鍑芥暟鏉ユ洿鏂板彈褰卞搷瀛愭爲涓婄殑澧炲己淇℃伅銆?
鍒犻櫎鑺傜偣鏃讹紝浣跨敤鑰呭繀椤昏皟鐢?rb_erase_augmented() 鑰屼笉鏄?rb_erase()銆俽b_erase_augmented() 浼氬洖璋冪敤鎴锋彁渚涚殑鍑芥暟锛屼互鏇存柊鍙楀奖鍝嶅瓙鏍戜笂鐨勫寮轰俊鎭€?
鍦ㄨ繖涓ょ鎯呭喌涓嬶紝鍥炶皟閮介€氳繃 struct rb_augment_callbacks 鎻愪緵銆傚繀椤诲畾涔?3 涓洖璋冿細

- 涓€涓紶鎾紙propagation锛夊洖璋冿紝鐢ㄤ簬鏇存柊缁欏畾鑺傜偣鍙婂叾绁栧厛鐨勫寮哄€硷紝鐩村埌缁欏畾鐨勫仠姝㈢偣锛堟垨 NULL 琛ㄧず涓€璺洿鏂板埌鏍癸級銆?
- 涓€涓鍒讹紙copy锛夊洖璋冿紝鐢ㄤ簬灏嗙粰瀹氬瓙鏍戠殑澧炲己鍊煎鍒跺埌鏂版寚瀹氱殑瀛愭爲鏍广€?
- 涓€涓爲鏃嬭浆锛坱ree rotation锛夊洖璋冿紝鐢ㄤ簬灏嗙粰瀹氬瓙鏍戠殑澧炲己鍊煎鍒跺埌鏂版寚瀹氱殑瀛愭爲鏍癸紝骞堕噸鏂拌绠楀師瀛愭爲鏍圭殑澧炲己淇℃伅銆?
rb_erase_augmented() 鐨勭紪璇戜唬鐮佸彲鑳戒細鍐呰仈浼犳挱鍜屽鍒跺洖璋冿紝浠庤€屼骇鐢熶竴涓緝澶х殑鍑芥暟锛屽洜姝ゆ瘡涓寮哄瀷 rbtree 浣跨敤鑰呭簲褰撳彧鏈変竴涓?rb_erase_augmented() 璋冪敤鐐癸紝浠ラ檺鍒剁紪璇戝悗鐨勪唬鐮佸ぇ灏忋€?

##### 浣跨敤绀轰緥


鍖洪棿鏍戞槸澧炲己鍨?rb 鏍戠殑涓€涓緥瀛愩€傚弬鑰冣€斺€擟ormen銆丩eiserson銆丷ivest 鍜?Stein 鎵€钁楃殑銆婄畻娉曞璁恒€嬨€傛湁鍏冲尯闂存爲鐨勬洿澶氱粏鑺傦細

缁忓吀鐨?rbtree 鍙湁涓€涓敭锛屾棤娉曠洿鎺ョ敤浜庡瓨鍌?[lo:hi] 杩欐牱鐨勫尯闂磋寖鍥达紝涔熸棤娉曞揩閫熸煡鎵炬槸鍚︿笌鏂扮殑 lo:hi 鍙戠敓閲嶅彔锛屾垨鍒ゆ柇鏄惁瀛樺湪涓庢柊 lo:hi 瀹屽叏鍖归厤鐨勯」銆?
涓嶈繃锛宺btree 鍙互琚寮猴紝浠ョ粨鏋勫寲鐨勬柟寮忓瓨鍌ㄦ绫诲尯闂磋寖鍥达紝浠庤€岃兘澶熷疄鐜伴珮鏁堢殑鏌ユ壘涓庣簿纭尮閰嶃€?
瀛樺偍鍦ㄦ瘡涓妭鐐逛腑鐨勮繖绉嶁€滈澶栦俊鎭€濓紝鏄叾鎵€鏈夊悗浠ｈ妭鐐逛腑鐨勬渶澶?hi锛坢ax_hi锛夊€笺€傚彧闇€鏌ョ湅鑺傜偣鍙婂叾鐩存帴瀛愯妭鐐癸紝鍗冲彲鍦ㄦ瘡涓妭鐐逛笂缁存姢璇ヤ俊鎭€傚畠灏嗚鐢ㄤ簬 O(log n) 鐨勬煡鎵句腑锛屼互鎵惧埌鏈€浣庡尮閰嶏紙鎵€鏈夊尮閰嶉」涓渶浣庣殑璧峰鍦板潃锛?```

  struct interval_tree_node *
  interval_tree_first_match(struct rb_root *root,
			    unsigned long start, unsigned long last)
  {
	struct interval_tree_node *node;

	if (!root->rb_node)
		return NULL;
	node = rb_entry(root->rb_node, struct interval_tree_node, rb);

	while (true) {
		if (node->rb.rb_left) {
			struct interval_tree_node *left =
				rb_entry(node->rb.rb_left,
					 struct interval_tree_node, rb);
			if (left->__subtree_last >= start) {
				/*
				 * Some nodes in left subtree satisfy Cond2.
				 * Iterate to find the leftmost such node N.
				 * If it also satisfies Cond1, that's the match
				 * we are looking for. Otherwise, there is no
				 * matching interval as nodes to the right of N
				 * can't satisfy Cond1 either.
				 */
				node = left;
				continue;
			}
		}
		if (node->start <= last) {		/* Cond1 */
			if (node->last >= start)	/* Cond2 */
				return node;	/* node is leftmost match */
			if (node->rb.rb_right) {
				node = rb_entry(node->rb.rb_right,
					struct interval_tree_node, rb);
				if (node->__subtree_last >= start)
					continue;
			}
		}
		return NULL;	/* No match */
	}
  }

```
```

  static inline unsigned long
  compute_subtree_last(struct interval_tree_node *node)
  {
	unsigned long max = node->last, subtree_last;
	if (node->rb.rb_left) {
		subtree_last = rb_entry(node->rb.rb_left,
			struct interval_tree_node, rb)->__subtree_last;
		if (max < subtree_last)
			max = subtree_last;
	}
	if (node->rb.rb_right) {
		subtree_last = rb_entry(node->rb.rb_right,
			struct interval_tree_node, rb)->__subtree_last;
		if (max < subtree_last)
			max = subtree_last;
	}
	return max;
  }

  static void augment_propagate(struct rb_node *rb, struct rb_node *stop)
  {
	while (rb != stop) {
		struct interval_tree_node *node =
			rb_entry(rb, struct interval_tree_node, rb);
		unsigned long subtree_last = compute_subtree_last(node);
		if (node->__subtree_last == subtree_last)
			break;
		node->__subtree_last = subtree_last;
		rb = rb_parent(&node->rb);
	}
  }

  static void augment_copy(struct rb_node *rb_old, struct rb_node *rb_new)
  {
	struct interval_tree_node *old =
		rb_entry(rb_old, struct interval_tree_node, rb);
	struct interval_tree_node *new =
		rb_entry(rb_new, struct interval_tree_node, rb);

	new->__subtree_last = old->__subtree_last;
  }

  static void augment_rotate(struct rb_node *rb_old, struct rb_node *rb_new)
  {
	struct interval_tree_node *old =
		rb_entry(rb_old, struct interval_tree_node, rb);
	struct interval_tree_node *new =
		rb_entry(rb_new, struct interval_tree_node, rb);

	new->__subtree_last = old->__subtree_last;
	old->__subtree_last = compute_subtree_last(old);
  }

  static const struct rb_augment_callbacks augment_callbacks = {
	augment_propagate, augment_copy, augment_rotate
  };

  void interval_tree_insert(struct interval_tree_node *node,
			    struct rb_root *root)
  {
	struct rb_node **link = &root->rb_node, *rb_parent = NULL;
	unsigned long start = node->start, last = node->last;
	struct interval_tree_node *parent;

	while (*link) {
		rb_parent = *link;
		parent = rb_entry(rb_parent, struct interval_tree_node, rb);
		if (parent->__subtree_last < last)
			parent->__subtree_last = last;
		if (start < parent->start)
			link = &parent->rb.rb_left;
		else
			link = &parent->rb.rb_right;
	}

	node->__subtree_last = last;
	rb_link_node(&node->rb, rb_parent, link);
	rb_insert_augmented(&node->rb, root, &augment_callbacks);
  }

  void interval_tree_remove(struct interval_tree_node *node,
			    struct rb_root *root)
  {
	rb_erase_augmented(&node->rb, root, &augment_callbacks);
  }

```
