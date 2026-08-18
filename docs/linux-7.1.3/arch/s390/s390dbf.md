## S390 璋冭瘯鐗规€э紙S390 Debug Feature锛?

files:
      - arch/s390/kernel/debug.c
      - arch/s390/include/asm/debug.h

### 鎻忚堪锛圖escription锛夛細


鏈壒鎬х殑鐩爣鏄彁渚涗竴涓唴鏍歌皟璇曟棩蹇?API锛屽叾涓殑鏃ュ織璁板綍鍙互楂樻晥鍦板瓨鍌ㄥ湪鍐呭瓨涓紝姣忎釜缁勪欢锛堜緥濡傝澶囬┍鍔級閮藉彲浠ユ嫢鏈夊悇鑷嫭绔嬬殑璋冭瘯鏃ュ織銆?杩欐牱鍋氱殑涓€涓洰鐨勬槸鍦ㄧ敓浜х郴缁熷穿婧冨悗妫€鏌ヨ皟璇曟棩蹇楋紝浠ュ垎鏋愬穿婧冪殑鍘熷洜銆?
濡傛灉绯荤粺浠嶅湪杩愯锛屼絾鍙湁鏌愪釜浣跨敤浜?dbf 鐨勫瓙缁勪欢澶辫触锛屽垯鍙互閫氳繃 Linux 鐨?debugfs 鏂囦欢绯荤粺鍦ㄨ繍琛屼腑鐨勭郴缁熶笂鏌ョ湅璋冭瘯鏃ュ織銆?
璇ヨ皟璇曠壒鎬у浜庡唴鏍稿拰椹卞姩鐨勫紑鍙戜篃鍙兘闈炲父鏈夌敤銆?
### 璁捐锛圖esign锛夛細


鍐呮牳缁勪欢锛堜緥濡傝澶囬┍鍔級鍙互閫氳繃鍑芥暟璋冪敤 `debug_register()` 鍦ㄨ皟璇曠壒鎬т腑娉ㄥ唽鑷繁銆?璇ュ嚱鏁颁负璋冪敤鑰呭垵濮嬪寲涓€涓皟璇曟棩蹇椼€傛瘡涓皟璇曟棩蹇楀瓨鍦ㄨ嫢骞蹭釜璋冭瘯鍖哄煙锛坉ebug area锛夛紝鍏朵腑鍚屼竴鏃跺埢鎭板ソ鏈変竴涓浜庢椿鍔ㄧ姸鎬併€傛瘡涓皟璇曞尯鍩熺敱鍐呭瓨涓繛缁殑椤电粍鎴愩€傚湪璋冭瘯鍖哄煙涓瓨鍌ㄧ潃璋冭瘯鏉＄洰锛坙og records锛夛紝瀹冧滑鐢?event 璋冪敤鍜?exception 璋冪敤鍐欏叆銆?
event 璋冪敤灏嗘寚瀹氱殑璋冭瘯鏉＄洰鍐欏叆娲诲姩璋冭瘯鍖哄煙锛屽苟鏇存柊璇ユ椿鍔ㄥ尯鍩熺殑鏃ュ織鎸囬拡銆傚鏋滃埌杈炬椿鍔ㄨ皟璇曞尯鍩熺殑鏈熬锛屽垯杩涜鍥炵粫锛坵rap around锛岀幆褰㈢紦鍐插尯锛夛紝涓嬩竴涓皟璇曟潯鐩皢琚啓鍏ユ椿鍔ㄨ皟璇曞尯鍩熺殑寮€濮嬪銆?
exception 璋冪敤灏嗘寚瀹氱殑璋冭瘯鏉＄洰鍐欏叆鏃ュ織锛屽苟鍒囨崲鍒颁笅涓€涓皟璇曞尯鍩熴€傝繖鏍峰仛鏄负浜嗙‘淇濇弿杩板紓甯告潵婧愮殑閭ｄ簺璁板綍鍦ㄥ綋鍓嶇殑鍖哄煙鍙戠敓鍥炵粫鏃朵笉浼氳瑕嗙洊銆?
璋冭瘯鍖哄煙鏈韩涔熶互鐜舰缂撳啿鍖虹殑椤哄簭鎺掑垪銆傚綋鍦ㄦ渶鍚庝竴涓皟璇曞尯鍩熶腑鎶涘嚭寮傚父鏃讹紝鍚庣画鐨勮皟璇曟潯鐩細鍐嶆鍐欏叆鏈€寮€濮嬬殑閭ｄ釜鍖哄煙銆?
event 璋冪敤鍜?exception 璋冪敤鏈夊洓绉嶇増鏈細涓€绉嶇敤浜庤褰曞師濮嬫暟鎹紝涓€绉嶇敤浜庢枃鏈紝涓€绉嶇敤浜庢暟瀛楋紙unsigned int 鍜?long锛夛紝杩樻湁涓€绉嶇敤浜庣被 sprintf 鐨勬牸寮忓寲瀛楃涓层€?
姣忎釜璋冭瘯鏉＄洰鍖呭惈浠ヤ笅鏁版嵁锛?
- 鏃堕棿鎴筹紙Timestamp锛?- 璋冪敤浠诲姟鐨?Cpu 缂栧彿
- 璋冭瘯鏉＄洰鐨勭骇鍒紙0...6锛?- 杩斿洖鍦板潃锛圧eturn Address锛屾寚鍚戣皟鐢ㄨ€咃級
- 鏍囧織锛屾寚绀鸿鏉＄洰鏄惁涓哄紓甯?
璋冭瘯鏃ュ織鍙互鍦ㄨ繍琛屼腑鐨勭郴缁熶笂閫氳繃 debugfs 鏂囦欢绯荤粺涓殑鏉＄洰鏉ユ鏌ャ€傚湪椤跺眰鐩綍 "`s390dbf`" 涓嬶紝涓烘瘡涓凡娉ㄥ唽鐨勭粍浠堕兘鏈変竴涓互鍏跺搴旂粍浠跺懡鍚嶇殑鐩綍銆俤ebugfs 閫氬父搴旀寕杞藉埌 `/sys/kernel/debug`锛屽洜姝よ皟璇曠壒鎬у彲浠ュ湪 `/sys/kernel/debug/s390dbf` 涓嬭璁块棶銆?
鐩綍鐨勫唴瀹规槸涓€浜涙枃浠讹紝瀹冧滑琛ㄧず瀵硅皟璇曟棩蹇楃殑涓嶅悓瑙嗗浘锛坴iew锛夈€傛瘡涓粍浠跺彲浠ラ€氳繃浣跨敤鍑芥暟 `debug_register_view()` 娉ㄥ唽鏉ュ喅瀹氫娇鐢ㄥ摢浜涜鍥俱€傛彁渚涗簡鐢ㄤ簬 hex/ascii 鍜?sprintf 鏁版嵁鐨勯瀹氫箟瑙嗗浘銆?涔熷彲浠ュ畾涔夊叾浠栬鍥俱€傚彧闇€璇诲彇瀵瑰簲鐨?debugfs 鏂囦欢鍗冲彲妫€鏌ユ煇涓鍥剧殑鍐呭銆?
鎵€鏈夎皟璇曟棩蹇楅兘鏈変竴涓綋鍓嶈皟璇曠骇鍒紙鑼冨洿浠?0 鍒?6锛夈€?榛樿绾у埆涓?3銆侲vent 鍜?Exception 鍑芥暟鏈変竴涓?:c`level` 鍙傛暟銆傚彧鏈夌骇鍒綆浜庢垨绛変簬褰撳墠绾у埆鐨勮皟璇曟潯鐩墠浼氳鍐欏叆鏃ュ織銆傝繖鎰忓懗鐫€锛屽湪鍐欏叆浜嬩欢鏃讹紝楂樹紭鍏堢骇鐨勬棩蹇楁潯鐩簲褰撳叿鏈夎緝浣庣殑绾у埆鍊硷紝鑰屼綆浼樺厛绾х殑鏉＄洰搴斿綋鍏锋湁杈冮珮鐨勭骇鍒€笺€?鍙互閫氳繃 debugfs 鏂囦欢绯荤粺锛屽悜涓烘瘡涓皟璇曟棩蹇楁彁渚涚殑 `level` debugfs 鏂囦欢鍐欏叆涓€涓暟瀛楀瓧绗︿覆 "x" 鏉ユ敼鍙樺綋鍓嶈皟璇曠骇鍒€傞€氳繃鍦?`level` debugfs 鏂囦欢涓婂啓鍏?"-" 鍙互瀹屽叏鍏抽棴璋冭瘯銆?
```
	> echo "-" > /sys/kernel/debug/s390dbf/dasd/level

```
涔熷彲浠ヤ负姣忎竴涓皟璇曟棩蹇楀叏灞€鍦板仠鐢ㄨ皟璇曠壒鎬с€傛偍鍙互閫氳繃 `/proc/sys/s390dbf` 涓殑 2 涓?sysctl 鍙傛暟鏉ユ敼鍙樺叾琛屼负锛?
鐩墠鏈?2 绉嶅彲鑳界殑瑙﹀彂鍣ㄤ細鍏ㄥ眬鍋滄璋冭瘯鐗规€с€傜涓€绉嶅彲鑳芥€ф槸浣跨敤 `debug_active` sysctl銆傚鏋滆缃负 1锛岃皟璇曠壒鎬ф鍦ㄨ繍琛岋紱濡傛灉 `debug_active` 璁剧疆涓?0锛岃皟璇曠壒鎬ц鍏抽棴銆?
绗簩绉嶅仠姝㈣皟璇曠壒鎬х殑瑙﹀彂鍣ㄦ槸鍐呮牳 oops銆傝繖鏍峰彲浠ラ槻姝㈣皟璇曠壒鎬ц鐩?oops 涔嬪墠鍙戠敓鐨勮皟璇曚俊鎭€傚湪 oops 涔嬪悗锛屾偍鍙互閫氳繃鍚?`/proc/sys/s390dbf/debug_active` 鍐欏叆 1 鏉ラ噸鏂版縺娲昏皟璇曠壒鎬с€備笉杩囷紝涓嶅缓璁湪鐢熶骇鐜涓娇鐢ㄥ彂鐢熻繃 oops 鐨勫唴鏍搞€?
濡傛灉鎮ㄦ兂绂佹鍋滅敤璋冭瘯鐗规€э紝鍙互浣跨敤 `debug_stoppable` sysctl銆傚鏋滃皢 `debug_stoppable` 璁剧疆涓?0锛岃皟璇曠壒鎬у皢鏃犳硶琚仠姝€傚鏋滆皟璇曠壒鎬у凡缁忓仠姝紝瀹冨皢淇濇寔鍋滅敤鐘舵€併€?
### 鍐呮牳鎺ュ彛锛圞ernel Interfaces锛夛細



### 棰勫畾涔夎鍥撅紙Predefined views锛夛細



  extern struct debug_view debug_hex_ascii_view;

  extern struct debug_view debug_sprintf_view;

### 绀轰緥锛圗xamples锛?


  /*
   - hex_ascii-view Example
   */

  #include <linux/init.h>
  #include <asm/debug.h>

  static debug_info_t *debug_info;

  static int init(void)
  {
      /** register 4 debug areas with one page each and 4 byte data field **/

      debug_info = debug_register("test", 1, 4, 4 );
      debug_register_view(debug_info, &debug_hex_ascii_view);

      debug_text_event(debug_info, 4 , "one ");
      debug_int_exception(debug_info, 4, 4711);
      debug_event(debug_info, 3, &debug_info, 4);

      return 0;
  }

  static void cleanup(void)
  {
      debug_unregister(debug_info);
  }

  module_init(init);
  module_exit(cleanup);


  /*
   - sprintf-view Example
   */

  #include <linux/init.h>
  #include <asm/debug.h>

  static debug_info_t *debug_info;

  static int init(void)
  {
      /** register 4 debug areas with one page each and data field for **/
      /** format string pointer + 2 varargs (= 3 ** sizeof(long))       */

      debug_info = debug_register("test", 1, 4, sizeof(long) * 3);
      debug_register_view(debug_info, &debug_sprintf_view);

      debug_sprintf_event(debug_info, 2 , "first event in %s:%i\n",__FILE__,__LINE__);
      debug_sprintf_exception(debug_info, 1, "pointer to debug info: %p\n",&debug_info);

      return 0;
  }

  static void cleanup(void)
  {
      debug_unregister(debug_info);
  }

  module_init(init);
  module_exit(cleanup);

### Debugfs 鎺ュ彛锛圖ebugfs Interface锛?

鍙互閫氳繃璇诲彇瀵瑰簲鐨?debugfs 鏂囦欢鏉ユ鏌ヨ皟璇曟棩蹇楃殑瑙嗗浘锛?
```
  > ls /sys/kernel/debug/s390dbf/dasd
  flush  hex_ascii  level pages
  > cat /sys/kernel/debug/s390dbf/dasd/hex_ascii | sort -k2,2 -s
  00 00974733272:680099 2 - 02 0006ad7e  07 ea 4a 90 | ....
  00 00974733272:682210 2 - 02 0006ade6  46 52 45 45 | FREE
  00 00974733272:682213 2 - 02 0006adf6  07 ea 4a 90 | ....
  00 00974733272:682281 1 * 02 0006ab08  41 4c 4c 43 | EXCP
  01 00974733272:682284 2 - 02 0006ab16  45 43 4b 44 | ECKD
  01 00974733272:682287 2 - 02 0006ab28  00 00 00 04 | ....
  01 00974733272:682289 2 - 02 0006ab3e  00 00 00 20 | ...
  01 00974733272:682297 2 - 02 0006ad7e  07 ea 4a 90 | ....
  01 00974733272:684384 2 - 00 0006ade6  46 52 45 45 | FREE
  01 00974733272:684388 2 - 00 0006adf6  07 ea 4a 90 | ....

```
鍏充簬涓婅堪杈撳嚭鐨勮В閲婏紝璇峰弬瑙佸叧浜庨瀹氫箟瑙嗗浘鐨勫皬鑺傦紒

### 鏀瑰彉璋冭瘯绾у埆锛圕hanging the debug level锛?


```
  > cat /sys/kernel/debug/s390dbf/dasd/level
  3
  > echo "5" > /sys/kernel/debug/s390dbf/dasd/level
  > cat /sys/kernel/debug/s390dbf/dasd/level
  5

```
### 鍒锋柊璋冭瘯鍖哄煙锛團lushing debug areas锛?

鍙互閫氳繃鍚?debugfs 鏂囦欢 "flush" 鍐欏叆鎵€闇€鍖哄煙鐨勭紪鍙凤紙0...n锛夋潵鍒锋柊璇ヨ皟璇曞尯鍩熴€備娇鐢?"-" 鏃讹紝鎵€鏈夎皟璇曞尯鍩熼兘浼氳鍒锋柊銆?
绀轰緥锛?
```
     > echo "0" > /sys/kernel/debug/s390dbf/dasd/flush

```
```
     > echo "-" > /sys/kernel/debug/s390dbf/dasd/flush

```
### 鏀瑰彉璋冭瘯鍖哄煙鐨勫ぇ灏忥紙Changing the size of debug areas锛?

瑕佽皟鏁磋皟璇曞尯鍩熺殑澶у皬锛岃灏嗘墍闇€鐨勯〉璁℃暟鍐欏叆 "pages" 鏂囦欢銆?濡傛灉鐜版湁鏁版嵁鑳芥斁寰椾笅鍒欎細琚繚鐣欙紱鍚﹀垯锛屾渶鏃х殑鏉＄洰浼氳涓㈠純銆?
绀轰緥锛?
```
  > echo "4" > /sys/kernel/debug/s390dbf/dasd/pages

```
### 鍋滄璋冭瘯鐗规€э紙Stopping the debug feature锛?

绀轰緥锛?
```
     > cat /proc/sys/s390dbf/debug_stoppable

```
```
     > echo 0 > /proc/sys/s390dbf/debug_active

```
### crash 鎺ュ彛锛坈rash Interface锛?

鑷?v5.1.0 璧凤紝`crash` 宸ュ叿鍐呯疆浜嗕竴涓懡浠?`s390dbf`锛岀敤浜庢樉绀烘墍鏈夎皟璇曟棩蹇楁垨灏嗗畠浠鍑哄埌鏂囦欢绯荤粺銆?鍊熷姪璇ュ伐鍏凤紝鍙互鍦ㄨ繍琛屼腑鐨勭郴缁熶笂浠ュ強绯荤粺宕╂簝鍚庣殑鍐呭瓨杞偍涓鏌ヨ皟璇曟棩蹇椼€?
### 璋冩煡鍘熷鍐呭瓨锛圛nvestigating raw memory锛?

鍦ㄨ繍琛屼腑鐨勭郴缁熶笂浠ュ強绯荤粺宕╂簝鍚庯紝璋冩煡璋冭瘯鏃ュ織鐨勬渶鍚庝竴绉嶅彲鑳芥€ф槸鏌ョ湅 VM 鎴栨湇鍔″厓绱狅紙Service Element锛変笅鐨勫師濮嬪唴瀛樸€?鍙互閫氳繃 System map 涓殑 `debug_area_first` 绗﹀彿鎵惧埌璋冭瘯鏃ュ織鐨勯敋鐐广€傜劧鍚庡繀椤婚『鐫€ debug.h 涓畾涔夌殑鏁版嵁缁撴瀯鐨勬纭寚閽堬紝鍦ㄥ唴瀛樹腑鎵惧埌璋冭瘯鍖哄煙銆?閫氬父锛屼娇鐢ㄨ皟璇曠壒鎬х殑妯″潡涔熶細鏈変竴涓寚鍚戣皟璇曟棩蹇楃殑鍏ㄥ眬鍙橀噺銆傞『鐫€璇ユ寚閽堜篃鍙互鎵惧埌鍐呭瓨涓殑璋冭瘯鏃ュ織銆?
瀵逛簬杩欑鏂规硶锛屽缓璁湪 `debug_register()` 涓娇鐢ㄩ暱搴︿负 '16 * x + 4' 瀛楄妭锛坸 = 0..n锛夌殑鏁版嵁瀛楁锛屼互渚挎煡鐪嬫牸寮忚壇濂界殑璋冭瘯鏉＄洰銆?

### 棰勫畾涔夎鍥撅紙Predefined Views锛?


鏈変袱绉嶉瀹氫箟瑙嗗浘锛歨ex_ascii 鍜?sprintf銆?hex_ascii 瑙嗗浘浠ュ崄鍏繘鍒跺拰 ascii 褰㈠紡鏄剧ず鏁版嵁瀛楁锛堜緥濡?`45 43 4b 44 | ECKD`锛夈€?
sprintf 瑙嗗浘浠ヤ笌 sprintf 鍑芥暟鐩稿悓鐨勬柟寮忔牸寮忓寲璋冭瘯鏉＄洰銆俿printf 鐨?event/exception 鍑芥暟鍚戣皟璇曟潯鐩啓鍏ヤ竴涓寚鍚戞牸寮忓瓧绗︿覆鐨勬寚閽堬紙澶у皬 = sizeof(long)锛夛紝骞朵负姣忎釜鍙彉鍙傛暟鍐欏叆涓€涓?long 鍊笺€傚洜姝わ紝渚嬪瀵逛簬涓€涓甫涓€涓牸寮忓瓧绗︿覆鍔犱袱涓彲鍙樺弬鏁扮殑璋冭瘯鏉＄洰锛岄渶瑕佸湪 debug_register() 鍑芥暟涓垎閰嶄竴涓?(3 * sizeof(long)) 瀛楄妭鐨勬暟鎹尯鍩熴€?
閲嶈锛圛MPORTANT锛夛細
  鍦?sprintf event 鍑芥暟涓娇鐢?"%s" 鏄嵄闄╃殑銆傚彧鏈夊湪浼犲叆瀛楃涓茬殑鍐呭瓨鍙璋冭瘯鐗规€у瓨鍦ㄥ氨涓€鐩村彲鐢ㄧ殑鎯呭喌涓嬶紝鎮ㄦ墠鑳藉湪 sprintf event 鍑芥暟涓娇鐢?"%s"銆傚叾鑳屽悗鐨勫師鍥犳槸锛屽嚭浜庢€ц兘鑰冭檻锛岃皟璇曠壒鎬т腑鍙瓨鍌ㄤ簡鎸囧悜璇ュ瓧绗︿覆鐨勬寚閽堛€傚鏋滄偍璁板綍浜嗕竴涓箣鍚庤閲婃斁鐨勫瓧绗︿覆锛屽湪妫€鏌ヨ皟璇曠壒鎬ф椂浼氬緱鍒颁竴涓?OOPS锛屽洜涓洪偅鏃惰皟璇曠壒鎬т細璁块棶宸茬粡琚噴鏀剧殑鍐呭瓨銆?
娉ㄦ剰锛圢OTE锛夛細
  濡傛灉浣跨敤 sprintf 瑙嗗浘锛岃涓嶈浣跨敤闄?sprintf-event 鍜?-exception 鍑芥暟涔嬪鐨勫叾浠?event/exception 鍑芥暟銆?
hex_ascii 鍜?sprintf 瑙嗗浘鐨勬牸寮忓涓嬶細

- 鍖哄煙缂栧彿锛圢umber of area锛?- 鏃堕棿鎴筹紙鏍煎紡涓鸿嚜 1970 骞?1 鏈?1 鏃?00:00:00 鍗忚皟涓栫晫鏃讹紙UTC锛夎捣鐨勭鍜屽井绉掞級
- 璋冭瘯鏉＄洰鐨勭骇鍒紙level锛?- 寮傚父鏍囧織锛圗xception flag锛? = 寮傚父锛?- 璋冪敤浠诲姟鐨?Cpu 缂栧彿
- 杩斿洖鍦板潃锛圧eturn Address锛屾寚鍚戣皟鐢ㄨ€咃級
- 鏁版嵁瀛楁锛坉ata field锛?
hex_ascii 瑙嗗浘鐨勫吀鍨嬭濡備笅鎵€绀猴紙绗竴琛?```
  area  time           level exception cpu caller    data (hex + ascii)
  --------------------------------------------------------------------------
  00    00964419409:440690 1 -         00  88023fe


```
### 瀹氫箟瑙嗗浘锛圖efining views锛?


瑙嗗浘閫氳繃 'debug_view' 缁撴瀯鏉ユ寚瀹氥€傚叾涓畾涔変簡涓€浜涚敤浜庤鍐?debugfs 鏂囦欢鐨勫洖璋冨嚱鏁帮細


  struct debug_view {
	char name[DEBUG_MAX_PROCF_LEN];
	debug_prolog_proc_t* prolog_proc;
	debug_header_proc_t* header_proc;
	debug_format_proc_t* format_proc;
	debug_input_proc_t*  input_proc;
	void*                private_data;
  };

鍏朵腑锛?

  typedef int (debug_header_proc_t) (debug_info_t* id,
				     struct debug_view* view,
				     int area,
				     debug_entry_t* entry,
				     char* out_buf);

  typedef int (debug_format_proc_t) (debug_info_t* id,
				     struct debug_view** view, char** out_buf,
				     const char* in_buf);
  typedef int (debug_prolog_proc_t) (debug_info_t* id,
				     struct debug_view* view,
				     char* out_buf);
  typedef int (debug_input_proc_t) (debug_info_t* id,
				    struct debug_view* view,
				    struct file** file, const char** user_buf,
				    size_t in_buf_size, loff_t* offset);


"private_data" 鎴愬憳鍙敤浣滄寚鍚戣鍥剧壒瀹氭暟鎹殑鎸囬拡銆?璋冭瘯鐗规€ф湰韬苟涓嶄細浣跨敤瀹冦€?
```
  "prolog_proc output"

  "header_proc output 1"  "format_proc output 1"
  "header_proc output 2"  "format_proc output 2"
  "header_proc output 3"  "format_proc output 3"
  ...

```
褰撲粠 debugfs 璇诲彇涓€涓鍥炬椂锛岃皟璇曠壒鎬ц皟鐢ㄤ竴娆?'prolog_proc' 鏉ュ啓鍏?prolog銆?鐒跺悗涓烘瘡涓凡瀛樺湪鐨勮皟璇曟潯鐩皟鐢?'header_proc' 鍜?'format_proc'銆?
input_proc 鍙敤浜庡湪鍐欏叆璇ヨ鍥炬椂瀹炵幇鏌愪簺鍔熻兘锛堜緥濡傚儚 `echo "0" > /sys/kernel/debug/s390dbf/dasd/level` 閭ｆ牱锛夈€?
瀵逛簬 header_proc锛屽彲浠ヤ娇鐢ㄥ湪 debug.h 涓畾涔夌殑榛樿鍑芥暟 `debug_dflt_header_fn()`锛屽畠浼氫骇鐢熶笌棰勫畾涔夎鍥剧浉鍚岀殑澶撮儴杈撳嚭銆?```
  00 00964419409:440761 2 - 00 88023ec

```
瑕佷簡瑙ｅ浣曚娇鐢ㄨ繖浜涘洖璋冨嚱鏁帮紝璇锋煡鐪嬮粯璁よ鍥剧殑瀹炵幇锛?
绀轰緥锛?

  #include <asm/debug.h>

  #define UNKNOWNSTR "data: %08x"

  const char* messages[] =
  {"This error...........\n",
   "That error...........\n",
   "Problem..............\n",
   "Something went wrong.\n",
   "Everything ok........\n",
   NULL
  };

  static int debug_test_format_fn(
     debug_info_t **id, struct debug_view **view,
     char **out_buf, const char **in_buf
  )
  {
    int i, rc = 0;

    if (id->buf_size >= 4) {
       int msg_nr = **((int**)in_buf);
       if (msg_nr < sizeof(messages) / sizeof(char*) - 1)
	  rc += sprintf(out_buf, "%s", messages[msg_nr]);
       else
	  rc += sprintf(out_buf, UNKNOWNSTR, msg_nr);
    }
    return rc;
  }

  struct debug_view debug_test_view = {
    "myview",                 /** name of view **/
    NULL,                     /** no prolog **/
    &debug_dflt_header_fn,    /** default header for each entry **/
    &debug_test_format_fn,    /** our own format function **/
    NULL,                     /** no input function **/
    NULL                      /** no private data **/
  };

## 娴嬭瘯锛坱est锛夛細



  debug_info_t *debug_info;
  int i;
  ...
  debug_info = debug_register("test", 0, 4, 4);
  debug_register_view(debug_info, &debug_test_view);
  for (i = 0; i < 10; i ++)
    debug_int_event(debug_info, 1, i);

```
  > cat /sys/kernel/debug/s390dbf/test/myview
  00 00964419734:611402 1 - 00 88042ca   This error...........
  00 00964419734:611405 1 - 00 88042ca   That error...........
  00 00964419734:611408 1 - 00 88042ca   Problem..............
  00 00964419734:611411 1 - 00 88042ca   Something went wrong.
  00 00964419734:611414 1 - 00 88042ca   Everything ok........
  00 00964419734:611417 1 - 00 88042ca   data: 00000005
  00 00964419734:611419 1 - 00 88042ca   data: 00000006
  00 00964419734:611422 1 - 00 88042ca   data: 00000007
  00 00964419734:611425 1 - 00 88042ca   data: 00000008
  00 00964419734:611428 1 - 00 88042ca   data: 00000009

```
