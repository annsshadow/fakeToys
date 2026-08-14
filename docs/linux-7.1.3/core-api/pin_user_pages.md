
## pin_user_pages() 鍙婄浉鍏宠皟鐢?
## 姒傝堪

```

 pin_user_pages()
 pin_user_pages_fast()
 pin_user_pages_remote()

```
## FOLL_PIN 鍩烘湰鎻忚堪

FOLL_PIN 鍜?FOLL_LONGTERM 鏄彲浠ヤ紶閫掔粰 get_user_pages*()锛堚€済up鈥濓級鍑芥暟鏃忕殑鏍囧織銆侳OLL_PIN 涓?FOLL_LONGTERM 鏈夌潃鏄捐憲鐨勪氦浜掑拰鐩镐簰渚濊禆鍏崇郴锛屽洜姝よ繖閲屼竴骞朵粙缁嶃€?
FOLL_PIN 鏄?gup 鍐呴儴鐨勶紝鎰忓懗鐫€瀹冧笉搴斿嚭鐜板湪 gup 璋冪敤鐐广€傝繖浣垮緱鐩稿叧鐨勫寘瑁呭嚱鏁帮紙pin_user_pages*() 鍙婂叾浠栵級鑳藉璁剧疆杩欎簺鏍囧織鐨勬纭粍鍚堬紝骞舵鏌ラ棶棰樸€?
鍙︿竴鏂归潰锛孎OLL_LONGTERM **鍙互**鍦?gup 璋冪敤鐐硅缃€傝繖鏍峰仛鏄负浜嗛伩鍏嶅垱寤哄ぇ閲忓寘瑁呭嚱鏁版潵瑕嗙洊 get**()銆乸in**()銆丗OLL_LONGTERM 绛夌殑鎵€鏈夌粍鍚堛€傛澶栵紝pin_user_pages**() API 涓?get_user_pages**() API 鏄庢樉涓嶅悓锛屽洜姝よ繖鏄竴涓嚜鐒剁殑鍒掑垎绾匡紝涔熸槸杩涜鍗曠嫭鍖呰璋冪敤鐨勫ソ鍒囧叆鐐广€傛崲鍙ヨ瘽璇达紝瀵?DMA-pinned 椤典娇鐢?pin_user_pages*()锛屽鍏朵粬鎯呭喌浣跨敤 get_user_pages*()銆傛湰鏂囨。鍚庨潰鎻忚堪浜嗕簲绉嶆儏鍐碉紝浠ヨ繘涓€姝ラ槓鏄庤繖涓€姒傚康銆?
瀵逛簬缁欏畾鐨?gup 璋冪敤锛孎OLL_PIN 涓?FOLL_GET 鏄簰鏂ョ殑銆備笉杩囷紝澶氫釜绾跨▼鍜岃皟鐢ㄧ偣鍙互閫氳繃 FOLL_PIN 鍜?FOLL_GET 鑷敱鍦?pin 鐩稿悓鐨?struct page銆傞渶瑕侀€夋嫨鍏朵腑涔嬩竴鐨勬槸璋冪敤鐐癸紝鑰屼笉鏄?struct page銆?
FOLL_PIN 鐨勫疄鐜颁笌 FOLL_GET 鍑犱箮鐩稿悓锛屽彧鏄?FOLL_PIN 浣跨敤浜嗕笉鍚岀殑寮曠敤璁℃暟鎶€鏈€?
FOLL_PIN 鏄?FOLL_LONGTERM 鐨勫厛鍐虫潯浠躲€傛崲鍙ヨ瘽璇达紝FOLL_LONGTERM 鏄?FOLL_PIN 鐨勪竴绉嶆洿鍙楅檺鐨勭壒瀹氭儏鍐点€?
## 姣忎釜鍖呰鍑芥暟璁剧疆浜嗗摢浜涙爣蹇?
瀵逛簬杩欎簺 pin_user_pages*() 鍑芥暟锛孎OLL_PIN 涓庤皟鐢ㄨ€呮彁渚涚殑浠讳綍 gup 鏍囧織杩涜 OR 杩愮畻銆傝皟鐢ㄨ€呴渶瑕佷紶鍏ヤ竴涓潪绌虹殑 struct pages* 鏁扮粍锛岀劧鍚庡嚱鏁伴€氳繃涓烘瘡涓〉澧炲姞涓€涓壒娈婂€硷細GUP_PIN_COUNTING_BIAS 鏉?pin 椤点€?
瀵逛簬澶?folio锛坙arge folios锛夛紝涓嶄娇鐢?GUP_PIN_COUNTING_BIAS 鏂规銆傜浉鍙嶏紝浣跨敤 struct folio 涓彲鐢ㄧ殑棰濆绌洪棿鏉ョ洿鎺ュ瓨鍌?pincount銆?
杩欑閽堝澶?folio 鐨勬柟娉曢伩鍏嶄簡涓嬮潰璁ㄨ鐨勮鏁颁笂闄愰棶棰樸€傞偅浜涢檺鍒朵細琚法椤碉紙huge pages锛変弗閲嶅姞鍓э紝鍥犱负姣忎釜灏鹃〉閮戒細鍚戝ご椤垫坊鍔犱竴涓紩鐢ㄨ鏁般€備簨瀹炰笂锛屾祴璇曡〃鏄庯紝鍦ㄦ病鏈夊崟鐙?pincount 瀛楁鐨勬儏鍐典笅锛屽湪鏌愪簺宸ㄩ〉鍘嬪姏娴嬭瘯涓瀵熷埌浜嗗紩鐢ㄨ鏁版孩鍑恒€?
杩欎篃鎰忓懗鐫€宸ㄩ〉鍜屽ぇ folio 涓嶄細閬彈
```

 Function
 --------
 pin_user_pages          FOLL_PIN 鎬绘槸鐢辨鍑芥暟鍦ㄥ唴閮ㄨ缃€? pin_user_pages_fast     FOLL_PIN 鎬绘槸鐢辨鍑芥暟鍦ㄥ唴閮ㄨ缃€? pin_user_pages_remote   FOLL_PIN 鎬绘槸鐢辨鍑芥暟鍦ㄥ唴閮ㄨ缃€?
```
瀵逛簬杩欎簺 get_user_pages*() 鍑芥暟锛屽彲鑳芥牴鏈笉浼氭寚瀹?FOLL_GET銆傝涓烘瘮涓婇潰绋嶅鏉備竴浜涖€傚鏋?**娌℃湁**鎸囧畾 FOLL_GET锛屼絾璋冪敤鑰呬紶鍏ヤ簡闈炵┖鐨?struct pages* 鏁扮粍锛岄偅涔堝嚱鏁颁細涓轰綘璁剧疆 FOLL_GET锛屽苟缁х画閫氳繃澧炲姞寮曠敤璁℃暟鏉?pin 椤?```

 Function
 --------
 get_user_pages           FOLL_GET 鏈夋椂鐢辨鍑芥暟鍦ㄥ唴閮ㄨ缃€? get_user_pages_fast      FOLL_GET 鏈夋椂鐢辨鍑芥暟鍦ㄥ唴閮ㄨ缃€? get_user_pages_remote    FOLL_GET 鏈夋椂鐢辨鍑芥暟鍦ㄥ唴閮ㄨ缃€?
```
## 璺熻釜 dma-pinned 椤?
璺熻釜 dma-pinned 椤电殑涓€浜涘叧閿璁＄害鏉熶笌瑙ｅ喅鏂规锛?
- 闇€瑕佹瘡涓?struct page 涓€涓疄闄呯殑寮曠敤璁℃暟銆傝繖鏄洜涓哄涓繘绋嬪彲鑳戒細 pin 鍜?unpin 涓€涓〉銆?
- 鍋囬槼鎬э紙鎶ュ憡涓€涓〉琚?dma-pinned锛岃€屽疄闄呬笂骞堕潪濡傛锛夋槸鍙互鎺ュ彈鐨勶紝浣嗗亣闃存€т笉琛屻€?
- 涓烘涓嶈兘澧炲ぇ struct page 鐨勫ぇ灏忥紝鑰屼笖鎵€鏈夊瓧娈甸兘宸茶浣跨敤銆?
- 閴翠簬浠ヤ笂锛屾垜浠彲浠ラ€氳繃浣跨敤 page->_refcount 瀛楁涓€滄煇绉嶆剰涔変笂鐨勨€濋珮浣嶆潵閲嶈浇璇ュ瓧娈碉紝浠ヤ綔涓?dma-pinned 璁℃暟銆傗€滄煇绉嶆剰涔変笂鐨勨€濇剰鍛崇潃锛屾垜浠笉灏?page->_refcount 鍒掑垎涓轰綅瀛楁锛岃€屾槸绠€鍗曞湴灏嗕竴涓腑绛夊ぇ灏忕殑鍊硷紙GUP_PIN_COUNTING_BIAS锛屾渶鍒濋€変负 1024锛?0 浣嶏級鍔犲埌 page->_refcount 涓娿€傝繖鎻愪緵浜嗘ā绯婄殑琛屼负锛氬鏋滀竴涓〉琚皟鐢ㄤ簡 1024 娆?get_page()锛岄偅涔堝畠灏嗚〃鐜颁负鍏锋湁鍗曚釜 dma-pinned 璁℃暟銆傚啀娆¤鏄庯紝杩欐槸鍙帴鍙楃殑銆?
杩欎篃甯︽潵浜嗛檺鍒讹細鍙湁 31-10==21 浣嶅彲鐢ㄤ簬涓€涓瘡娆￠€掑 10 浣嶇殑璁℃暟鍣ㄣ€?
- 鐢变簬璇ラ檺鍒讹紝浣跨敤 FOLL_PIN 鏃跺闆堕〉锛坺ero pages锛夊仛浜嗙壒娈婂鐞嗐€傛垜浠彧鏄亣瑁?pin 浜嗕竴涓浂椤碘€斺€旀牴鏈笉鏀瑰彉鍏跺紩鐢ㄨ鏁版垨 pincount锛堝畠鏄案涔呯殑锛屽洜姝ゆ病鏈夊繀瑕侊級銆倁npinning 鍑芥暟瀵归浂椤典篃涓嶅仛浠讳綍鎿嶄綔銆傝繖瀵硅皟鐢ㄨ€呮槸閫忔槑鐨勩€?
- 璋冪敤鑰呭繀椤绘樉寮忚姹傗€滈〉鐨?dma-pinned 璺熻釜鈥濄€傛崲鍙ヨ瘽璇达紝浠呬粎璋冪敤 get_user_pages() 鏄笉澶熺殑锛涘繀椤讳娇鐢ㄤ竴缁勬柊鍑芥暟锛屽嵆 pin_user_page() 鍙婂叾鐩稿叧鍑芥暟銆?
## FOLL_PIN銆丗OLL_GET銆丗OLL_LONGTERM锛氫綍鏃朵娇鐢ㄥ摢涓爣蹇?
鎰熻阿 Jan Kara銆乂lastimil Babka 浠ュ強鍏朵粬鍑犱綅 -mm 浜哄憳鎻忚堪浜嗚繖浜涚被鍒細

### CASE 1: 鐩存帴 IO锛圖IO锛?
瀛樺湪 GUP 寮曠敤锛岃繖浜涢〉浣滀负 DIO 缂撳啿鍖恒€傝繖浜涚紦鍐插尯闇€瑕佺殑鏃堕棿鐩稿杈冪煭锛堝洜姝ゅ畠浠笉鏄€滈暱鏈熺殑鈥濓級銆備笌 folio_mkclean() 鎴?munmap() 娌℃湁鐗规畩鐨勫悓姝?```

    FOLL_PIN

```
鈥︹€︿絾涓庡叾鐩存帴璁剧疆 FOLL_PIN锛岃皟鐢ㄧ偣搴斿綋浣跨敤璁剧疆浜?FOLL_PIN 鐨?pin_user_pages*() 渚嬬▼涔嬩竴銆?
### CASE 2: RDMA

瀛樺湪 GUP 寮曠敤锛岃繖浜涢〉浣滀负 DMA 缂撳啿鍖恒€傝繖浜涚紦鍐插尯闇€瑕佸緢闀挎椂闂达紙鈥滈暱鏈熲€濓級銆傛病鏈夋彁渚涗笌 folio_mkclean() 鎴?munmap() 鐨勭壒娈婂悓姝ャ€傚洜姝ゆ爣蹇?```

    FOLL_PIN | FOLL_LONGTERM

```
娉ㄦ剰锛氭煇浜涢〉锛屼緥濡?DAX 椤碉紝鏃犳硶琚暱鏈?pin銆傝繖鏄洜涓?DAX 椤垫病鏈夊崟鐙殑椤电紦瀛橈紝鍥犳鈥減inning鈥濇剰鍛崇潃閿佸畾鏂囦欢绯荤粺鍧楋紝鑰岋紙鐩墠锛夎繕涓嶄互鏀寔杩欑鏂瑰紡銆?
### CASE 3: MMU notifier 娉ㄥ唽锛屽甫鎴栦笉甯︾己椤电‖浠?
璁惧椹卞姩鍙互閫氳繃 get_user_pages*() pin 椤碉紝骞朵负璇ュ唴瀛樿寖鍥存敞鍐?mmu notifier 鍥炶皟銆傜劧鍚庯紝鍦ㄦ敹鍒?notifier 鐨勨€渋nvalidate range鈥濆洖璋冩椂锛屽仠姝㈣澶囦娇鐢ㄨ鑼冨洿锛屽苟 unpin 杩欎簺椤点€傚彲鑳借繕鏈夊叾浠栧彲琛岀殑鏂规锛屼緥濡傛樉寮忓湴閽堝寰呭鐞嗙殑 IO 杩涜鍚屾锛屼互杈惧埌澶ц嚧鐩稿悓鐨勬晥鏋溿€?
鎴栬€咃紝濡傛灉纭欢鏀寔鍙噸鏀剧己椤碉紙replayable page faults锛夛紝閭ｄ箞璁惧椹卞姩鍙互瀹屽叏閬垮厤 pinning锛堣繖鏄悊鎯崇殑锛夛紝濡備笅鎵€绀猴細鍍忎笂闈竴鏍锋敞鍐?mmu notifier 鍥炶皟锛屼絾涓嶆槸鍦ㄥ洖璋冧腑鍋滄璁惧骞?unpin锛岃€屽彧鏄皢璇ヨ寖鍥翠粠璁惧鐨勯〉琛ㄤ腑绉婚櫎銆?
鏃犺鍝鏂瑰紡锛屽彧瑕侀┍鍔ㄥ湪 mmu notifier 鍥炶皟鏃?unpin 杩欎簺椤碉紝灏变笌鏂囦欢绯荤粺鍜?mm锛坒olio_mkclean()銆乵unmap() 绛夛級鏈変簡閫傚綋鐨勫悓姝ャ€傚洜姝わ紝涓嶉渶瑕佽缃换浣曚竴涓爣蹇椼€?
### CASE 4: 浠呬负 struct page 鎿嶄綔鑰?pinning

濡傛灉鍙奖鍝?struct page 鏁版嵁锛堜笌椤垫墍杩借釜鐨勫疄闄呭唴瀛樺唴瀹圭浉瀵癸級锛岄偅涔堟櫘閫氱殑 GUP 璋冪敤灏辫冻澶熶簡锛屼笉闇€瑕佽缃换浣曚竴涓爣蹇椼€?
### CASE 5: 涓轰簡鍐欏叆椤靛唴鏁版嵁鑰?pinning

鍗充娇涓嶆秹鍙?DMA 鎴栫洿鎺?IO锛屼粎浠呮槸鈥減in銆佸啓鍏ラ〉鏁版嵁銆乽npin鈥濊繖鏍风畝鍗曠殑鎯呭喌涔熶細閫犳垚闂銆侰ASE 5 鍙互琚涓?CASE 1 鍔犱笂 CASE 2 鍐嶅姞涓婁换浣曡皟鐢ㄨ妯″紡鐨勬儏鍐电殑瓒呴泦銆傛崲鍙ヨ瘽璇达紝濡傛灉浠ｇ爜鏃笉鏄?CASE 1 涔熶笉鏄?CASE 2锛屽畠浠嶇劧鍙兘闇€瑕?FOLL_PIN锛屽浜庡涓嬭繖鏍风殑妯″紡锛?
姝ｇ‘锛堜娇鐢?FOLL_PIN 璋冪敤锛夛細
    pin_user_pages()
    鍐欏叆杩欎簺椤靛唴鐨勬暟鎹?    unpin_user_pages()

閿欒锛堜娇鐢?FOLL_GET 璋冪敤锛夛細
    get_user_pages()
    鍐欏叆杩欎簺椤靛唴鐨勬暟鎹?    put_page()

## folio_maybe_dma_pinned()锛歱inning 鐨勫叏閮ㄦ剰涔?
灏?folio 鏍囪涓衡€淒MA-pinned鈥濇垨鈥済up-pinned鈥濈殑鍏ㄩ儴鎰忎箟鍦ㄤ簬鑳藉鏌ヨ鈥滆繖涓?folio 鏄惁琚?DMA-pinned锛熲€濊繖浣垮緱璇稿 folio_mkclean()锛堜互鍙婁竴鑸殑鏂囦欢绯荤粺鍥炲啓浠ｇ爜锛変箣绫荤殑浠ｇ爜鑳藉鍦ㄧ敱浜庢绫?pin 鑰屾棤娉曡В闄ゆ槧灏勬煇涓?folio 鏃讹紝瀵硅鍋氫粈涔堝仛鍑烘槑鏅虹殑鍐冲畾銆?
鍦ㄨ繖浜涙儏鍐典笅璇ュ仛浠€涔堬紝鏄暱杈炬暟骞寸殑璁ㄨ涓庝簤璁虹殑涓婚锛堝弬瑙佹湰鏂囨。鏈熬鐨勫弬鑰冩枃鐚級銆傝繖鏄竴涓?TODO 椤癸細寰呰闂鐞嗘竻鍚庤ˉ鍏ㄧ粏鑺傘€傚悓鏃讹紝鍙互鑲畾鍦拌
```

        static inline bool folio_maybe_dma_pinned(struct folio *folio)

```
鈥︹€︽槸瑙ｅ喅闀挎湡瀛樺湪鐨?gup+DMA 闂鐨勫厛鍐虫潯浠躲€?
## 鎬濊€?FOLL_GET銆丗OLL_PIN 鍜?FOLL_LONGTERM 鐨勫彟涓€绉嶆柟寮?
鎬濊€冭繖浜涙爣蹇楃殑鍙︿竴绉嶆柟寮忔槸浣滀负涓€绯诲垪閫愭鍔犲己鐨勯檺鍒讹細FOLL_GET 鐢ㄤ簬 struct page 鎿嶄綔锛屼笉褰卞搷 struct page 鎵€寮曠敤鐨勬暟鎹€侳OLL_PIN 鏄?FOLL_GET 鐨?*鏇夸唬鍝?*锛岀敤浜庡鍏舵暟鎹?*灏嗚**璁块棶鐨勯〉杩涜鐭湡 pin銆傚洜姝わ紝FOLL_PIN 鏄竴绉嶁€滄洿涓ユ牸鈥濈殑 pin 褰㈠紡銆傛渶鍚庯紝FOLL_LONGTERM 鏄竴涓檺鍒舵洿寮虹殑銆佷互 FOLL_PIN 涓哄厛鍐虫潯浠剁殑鎯呭喌锛氳繖鐢ㄤ簬灏嗚闀挎湡 pin 涓斿叾鏁版嵁灏嗚璁块棶鐨勯〉銆?
## 鍗曞厓娴嬭瘯

```

 tools/testing/selftests/mm/gup_test.c

```
鏈変互涓嬫柊鐨勮皟鐢ㄧ敤浜庢紨缁冩柊鐨?pin*() 鍖呰鍑芥暟锛?
- PIN_FAST_BENCHMARK (./gup_test -a)
- PIN_BASIC_TEST (./gup_test -b)

浣犲彲浠ョ洃鎺у凡鑾峰彇鍜屽凡閲婃斁鐨?dma-pinned 椤垫€绘暟
```

    /proc/vmstat/nr_foll_pin_acquired
    /proc/vmstat/nr_foll_pin_released

```
鍦ㄦ甯告儏鍐典笅锛岃繖涓や釜鍊肩浉绛夛紝闄ら潪瀛樺湪浠讳綍闀挎湡 [R]DMA pin锛屾垨澶勪簬 pin/unpin 杞崲鏈熼棿銆?
- nr_foll_pin_acquired锛氳嚜绯荤粺涓婄數浠ユ潵宸茶幏鍙栫殑 logical pins 鏁伴噺銆傚浜庡法椤碉紝澶撮〉琚?pin 涓€娆★紙閽堝宸ㄩ〉涓殑姣忎釜椤碘€斺€斿ご椤靛拰姣忎釜灏鹃〉锛夈€傝繖閬靛惊涓?get_user_pages() 鐢ㄤ簬宸ㄩ〉鐨勭浉鍚岃涓猴細褰?get_user_pages() 搴旂敤浜庡法椤垫椂锛屽ご椤甸拡瀵瑰法椤典腑鐨勬瘡涓熬椤垫垨澶撮〉琚紩鐢ㄨ鏁颁竴娆°€?
- nr_foll_pin_released锛氳嚜绯荤粺涓婄數浠ユ潵宸查噴鏀剧殑 logical pins 鏁伴噺銆傛敞鎰忥紝椤垫槸浠?PAGE_SIZE 绮掑害閲婃斁锛坲npin锛夌殑锛屽嵆浣挎渶鍒濈殑 pin 鏄簲鐢ㄤ簬宸ㄩ〉銆傜敱浜庝笂闈⑩€渘r_foll_pin_acquired鈥濅腑鎻忚堪鐨?pin 璁℃暟鐨勮涓猴紝
```

    pin_user_pages(huge_page);
    for (each page in huge_page)
        unpin_user_page(page);

```
```

    nr_foll_pin_released == nr_foll_pin_acquired

```
锛堚€︹€﹂櫎闈炵敱浜庡凡鏈夌殑闀挎湡 RDMA pin 鑰屽凡缁忓け鍘诲钩琛°€傦級

## 鍏朵粬璇婃柇

dump_page() 宸茶鐣ュ井澧炲己浠ュ鐞嗚繖浜涙柊鐨勮鏁板瓧娈碉紝骞舵洿濂藉湴鎶ュ憡澶?folio銆傚叿浣撴潵璇达紝瀵逛簬澶?folio锛屼細鎶ュ憡绮剧‘鐨?pincount銆?
## 鍙傝€冩枃鐚?
- `Some slow progress on get_user_pages() (Apr 2, 2019) <https://lwn.net/Articles/784574/>`_
- `DMA and get_user_pages() (LPC: Dec 12, 2018) <https://lwn.net/Articles/774411/>`_
- `The trouble with get_user_pages() (Apr 30, 2018) <https://lwn.net/Articles/753027/>`_
- `LWN kernel index: get_user_pages() <https://lwn.net/Kernel/Index/#Memory_management-get_user_pages>`_

John Hubbard锛?019 骞?10 鏈?