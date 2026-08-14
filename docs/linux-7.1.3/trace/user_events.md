## user_events锛氬熀浜庣敤鎴风殑浜嬩欢璺熻釜


:Author: Beau Belgrave

### 姒傝堪

鍩轰簬鐢ㄦ埛鐨勮窡韪簨浠跺厑璁哥敤鎴疯繘绋嬪垱寤轰簨浠跺苟璺熻釜鏁版嵁锛岃繖浜涙暟鎹彲浠ラ€氳繃鐜版湁宸ュ叿锛堜緥濡?ftrace 鍜?perf锛夋煡鐪嬨€?
瑕佸惎鐢ㄦ鐗规€э紝璇峰湪鏋勫缓鍐呮牳鏃惰缃?CONFIG_USER_EVENTS=y銆?

绋嬪簭鍙互閫氳繃 /sys/kernel/tracing/user_events_status 鏌ョ湅浜嬩欢鐨勭姸鎬侊紝骞跺彲浠ラ€氳繃
/sys/kernel/tracing/user_events_data 娉ㄥ唽鍜屽啓鍑烘暟鎹€?

绋嬪簭涔熷彲浠ヤ娇鐢?/sys/kernel/tracing/dynamic_events锛岄€氳繃 u: 鍓嶇紑鏉ユ敞鍐屽拰鍒犻櫎鍩轰簬鐢ㄦ埛鐨勪簨浠躲€?
鍙戦€佺粰 dynamic_events 鐨勫懡浠ゆ牸寮忎笌甯︽湁 u: 鍓嶇紑鐨?ioctl 鐩稿悓銆傜敱浜庝簨浠朵細鎸佷箙瀛樺湪锛岃繖闇€瑕?CAP_PERFMON 鏉冮檺锛?
鍚﹀垯浼氳繑鍥?-EPERM銆?

閫氬父锛岀▼搴忎細娉ㄥ唽涓€缁勫畠浠笇鏈涙毚闇茬粰鑳藉璇诲彇 trace_events 鐨勫伐鍏凤紙濡?ftrace 鍜?perf锛夌殑浜嬩欢銆傛敞鍐岃繃绋?
浼氬憡璇夊唴鏍革細濡傛灉浠讳綍宸ュ叿鍚敤浜嗚浜嬩欢骞朵笖搴斿綋鍐欏嚭鏁版嵁锛屽簲褰撳弽鏄犲埌鍝釜鍦板潃鍜屽摢涓綅銆傛敞鍐屼細杩斿洖涓€涓?
鍐欑储寮曪紙write index锛夛紝鐢ㄤ簬鎻忚堪鍦?/sys/kernel/tracing/user_events_data 鏂囦欢涓婅皟鐢?write() 鎴?writev() 鏃?
鐨勬暟鎹€?

鏈枃妗ｄ腑寮曠敤鐨勭粨鏋勪綋鍖呭惈鍦ㄦ簮鐮佹爲涓殑 /include/uapi/linux/user_events.h 鏂囦欢閲屻€?

**娉ㄦ剰锛?* *user_events_status 鍜?user_events_data 閮戒綅浜?tracefs 鏂囦欢绯荤粺涓嬶紝鍙兘鎸傝浇鍦?
涓庝笂杩颁笉鍚岀殑璺緞涓娿€?

### 娉ㄥ唽

鍦ㄧ敤鎴疯繘绋嬪唴閮ㄨ繘琛屾敞鍐岋紝鏄€氳繃瀵?/sys/kernel/tracing/user_events_data 鏂囦欢鍙戝嚭 ioctl() 鏉ュ畬鎴愮殑銆?
瑕佸彂鍑虹殑鍛戒护鏄?DIAG_IOCSREG銆?

```

  struct user_reg {
        /* Input: Size of the user_reg structure being used */
        __u32 size;

        /* Input: Bit in enable address to use */
        __u8 enable_bit;

        /* Input: Enable size in bytes at address */
        __u8 enable_size;

        /* Input: Flags to use, if any */
        __u16 flags;

        /* Input: Address to update when enabled */
        __u64 enable_addr;

        /* Input: Pointer to string with event name, description and flags */
        __u64 name_args;

        /* Output: Index of the event to use when writing data */
        __u32 write_index;
  } __attribute__((__packed__));

```
struct user_reg 瑕佹眰姝ｇ‘璁剧疆涓婅堪鎵€鏈夎緭鍏ュ瓧娈点€?

- size锛氬繀椤昏缃负 sizeof(struct user_reg)銆?

- enable_bit锛氱敤浜庡湪 enable_addr 鎸囧畾鍦板潃澶勫弽鏄犱簨浠剁姸鎬佺殑浣嶃€?

- enable_size锛歟nable_addr 鎵€鎸囧畾鍊肩殑澶у皬銆?
  蹇呴』鏄?4锛?2 浣嶏級鎴?8锛?4 浣嶏級銆?4 浣嶅€煎彧鍏佽鍦?64 浣嶅唴鏍镐笂浣跨敤锛岃€?32 浣嶅€煎彲浠ュ湪鎵€鏈夊唴鏍镐笂浣跨敤銆?

- flags锛氳浣跨敤鐨勬爣蹇楋紙濡傛灉鏈夌殑璇濓級銆?
  璋冪敤鑰呭簲褰撳厛灏濊瘯甯︽爣蹇楄皟鐢紝骞跺湪涓嶅甫鏍囧織鐨勬儏鍐典笅閲嶈瘯锛屼互纭繚瀵硅緝浣庣増鏈唴鏍哥殑鍏煎鎬с€傚鏋滄煇涓爣蹇椾笉鍙楁敮鎸侊紝浼氳繑鍥?-EINVAL銆?

- enable_addr锛氱敤浜庡弽鏄犱簨浠剁姸鎬佺殑鍊肩殑鍦板潃銆傝鍦板潃蹇呴』鍦ㄧ敤鎴风▼搴忓唴鑷劧瀵归綈涓斿彲鍐欍€?

- name_args锛氱敤浜庢弿杩颁簨浠剁殑鍚嶇О鍜屽弬鏁帮紝璇﹁鍛戒护鏍煎紡銆?

褰撳墠鏀寔浠ヤ笅鏍囧織銆?

- USER_EVENT_REG_PERSIST锛氬綋鏈€鍚庝竴涓紩鐢ㄥ叧闂椂锛屼簨浠朵笉浼氳鍒犻櫎銆傚鏋滄煇涓簨浠跺嵆浣垮湪璇ヨ繘绋嬪叧闂垨娉ㄩ攢璇ヤ簨浠跺悗浠嶅簲瀛樺湪锛岃皟鐢ㄨ€呭彲浠ヤ娇鐢ㄦ鏍囧織銆傞渶瑕?CAP_PERFMON 鏉冮檺锛屽惁鍒欎細杩斿洖 -EPERM銆?

- USER_EVENT_REG_MULTI_FORMAT锛氫簨浠跺彲浠ュ寘鍚绉嶆牸寮忋€傝繖鍏佽绋嬪簭鍦ㄥ叾浜嬩欢鏍煎紡鍙戠敓鍙樺寲涓斿笇鏈涗娇鐢ㄧ浉鍚屽悕绉版椂锛岄伩鍏嶈嚜韬闃诲銆備娇鐢ㄦ鏍囧織鏃讹紝tracepoint 鍚嶇О灏嗛噰鐢?"name.unique_id" 鐨勬柊鏍煎紡锛岃€岄潪鏃х殑 "name" 鏍煎紡銆傚皢涓烘瘡缁勫敮涓€鐨勶紙鍚嶇О锛屾牸寮忥級瀵瑰垱寤轰竴涓?tracepoint銆傝繖鎰忓懗鐫€濡傛灉澶氫釜杩涚▼浣跨敤鐩稿悓鐨勫悕绉板拰鏍煎紡锛屽畠浠皢浣跨敤鍚屼竴涓?tracepoint銆傚鏋滆繕鏈夊彟涓€涓繘绋嬩娇鐢ㄧ浉鍚岀殑鍚嶇О锛屼絾鏍煎紡涓庡叾浠栬繘绋嬩笉鍚岋紝瀹冨皢浣跨敤涓€涓甫鏈夋柊鍞竴 id 鐨勪笉鍚?tracepoint銆傚綍鍒剁▼搴忛渶瑕佹壂鎻?tracefs锛屾壘鍑哄畠浠劅鍏磋叮鐨勪簨浠跺悕绉扮殑鍚勭涓嶅悓鏍煎紡銆傝 tracepoint 鐨勭郴缁熷悕绉颁篃灏嗕娇鐢?"user_events_multi" 鑰岄潪 "user_events"銆傝繖鍙互闃叉鍗曟牸寮忎簨浠跺悕绉颁笌 tracefs 涓换浣曞鏍煎紡浜嬩欢鍚嶇О鍐茬獊銆倁nique_id 浠ュ崄鍏繘鍒跺瓧绗︿覆褰㈠紡杈撳嚭銆傚綍鍒剁▼搴忓簲纭繚 tracepoint 鍚嶇О浠ュ畠浠敞鍐岀殑浜嬩欢鍚嶇О寮€澶达紝骞朵笖鍚庣紑浠?. 寮€澶翠笖鍙寘鍚崄鍏繘鍒跺瓧绗︺€備緥濡傦紝瑕佹煡鎵句簨浠?"test" 鐨勬墍鏈夌増鏈紝鍙互浣跨敤姝ｅ垯琛ㄨ揪寮?"^test\.[0-9a-fA-F]+$"銆?

娉ㄥ唽鎴愬姛鍚庡皢璁剧疆浠ヤ笅鍐呭銆?

- write_index锛氱敤浜庢鏂囦欢鎻忚堪绗︾殑绱㈠紩锛屼唬琛ㄥ啓鍑烘暟鎹椂瀵瑰簲鐨勮繖涓簨浠躲€傝绱㈠紩瀵逛簬鐢ㄤ簬娉ㄥ唽鐨勮繖涓枃浠舵弿杩扮瀹炰緥鏄敮涓€鐨勩€傝瑙佲€滃啓鍏ユ暟鎹€濅竴鑺傘€?

鍩轰簬鐢ㄦ埛鐨勪簨浠朵細鍍?"user_events" 瀛愮郴缁熶笅鐨勪换浣曞叾浠栦簨浠朵竴鏍峰嚭鐜板湪 tracefs 涓€傝繖鎰忓懗鐫€甯屾湜鎸傛帴杩欎簺浜嬩欢鐨勫伐鍏烽渶瑕佷娇鐢?/sys/kernel/tracing/events/user_events/[name]/enable锛屾垨鍦ㄦ寕鎺?褰曞埗鏃朵娇鐢?perf record -e user_events:[name]銆?

**娉ㄦ剰锛?* 浜嬩欢瀛愮郴缁熷悕绉伴粯璁ゆ槸 "user_events"銆傝皟鐢ㄨ€呬笉搴斿亣璁惧畠灏嗘案杩滄槸 "user_events"銆傝繍缁存柟淇濈暀灏嗘潵涓烘敮鎸佷簨浠堕殧绂昏€屾寜杩涚▼鏇存敼瀛愮郴缁熷悕绉扮殑鏉冨埄銆傛澶栵紝濡傛灉浣跨敤 USER_EVENT_REG_MULTI_FORMAT 鏍囧織锛宼racepoint 鍚嶇О灏嗚闄勫姞涓€涓敮涓€ id锛屼笖绯荤粺鍚嶇О灏嗗涓婃墍杩板彉涓?"user_events_multi"銆?

##### 鍛戒护鏍煎紡

```

  name[:FLAG1[,FLAG2...]] [Field1[;Field2...]]

```
##### 鏀寔鐨勬爣蹇?

鏆傛棤

##### 瀛楁鏍煎紡

```

  type name [size]

```
鏀寔鍩烘湰绫诲瀷锛坃_data_loc銆乽32銆乽64銆乮nt銆乧har銆乧har[^20^] 绛夛級銆?
榧撳姳鐢ㄦ埛绋嬪簭浣跨敤鏄庣‘鎸囧畾澶у皬鐨勭被鍨嬶紝渚嬪 u32銆?

**娉ㄦ剰锛?* **涓嶆敮鎸?long 绫诲瀷锛屽洜涓哄叾澶у皬鍦ㄧ敤鎴风┖闂村拰鍐呮牳涔嬮棿鍙兘涓嶅悓銆?*

澶у皬浠呭浠?struct 鍓嶇紑寮€澶寸殑绫诲瀷鏈夋晥銆傝繖鍏佽鐢ㄦ埛鍦ㄩ渶瑕佹椂鍚戝伐鍏锋弿杩拌嚜瀹氫箟鐨?struct銆?

```

  struct mytype {
    char data[20];
  };

```
```

  struct mytype myname 20

```
### 鍒犻櫎

鍦ㄧ敤鎴疯繘绋嬪唴閮ㄥ垹闄や竴涓簨浠讹紝鏄€氳繃瀵?/sys/kernel/tracing/user_events_data 鏂囦欢鍙戝嚭 ioctl() 鏉ュ畬鎴愮殑銆?
瑕佸彂鍑虹殑鍛戒护鏄?DIAG_IOCSDEL銆?

姝ゅ懡浠ゅ彧闇€瑕佷竴涓瓧绗︿覆锛屾寜鍚嶇О鎸囧畾瑕佸垹闄ょ殑浜嬩欢銆傚彧鏈夊綋璇ヤ簨浠朵笉鍐嶆湁浠讳綍寮曠敤锛堝湪鐢ㄦ埛绌洪棿鍜屽唴鏍哥┖闂村潎濡傛锛夋椂锛屽垹闄ゆ墠浼氭垚鍔熴€?
鍥犳锛岀敤鎴风▼搴忓簲褰撲娇鐢ㄤ竴涓崟鐙殑鏂囦欢鏉ヨ姹傚垹闄わ紝鑰屼笉鏄敤浜庢敞鍐岀殑閭ｄ釜鏂囦欢銆?

**娉ㄦ剰锛?* 榛樿鎯呭喌涓嬶紝褰撲簨浠朵笉鍐嶆湁浠讳綍寮曠敤鏃朵細鑷姩鍒犻櫎銆傚鏋滅▼搴忎笉甯屾湜鑷姩鍒犻櫎锛屽繀椤诲湪娉ㄥ唽浜嬩欢鏃朵娇鐢?
USER_EVENT_REG_PERSIST 鏍囧織銆備竴鏃︿娇鐢ㄤ簡璇ユ爣蹇楋紝浜嬩欢灏嗕竴鐩村瓨鍦紝鐩村埌璋冪敤 DIAG_IOCSDEL銆傛敞鍐屽拰鍒犻櫎涓€涓寔涔呭寲浜嬩欢
閮介渶瑕?CAP_PERFMON 鏉冮檺锛屽惁鍒欎細杩斿洖 -EPERM銆傚綋鍚屼竴涓簨浠跺悕绉板瓨鍦ㄥ绉嶆牸寮忔椂锛屾墍鏈夊悓鍚嶇殑浜嬩欢閮藉皢琚皾璇曞垹闄ゃ€?
濡傛灉鍙兂鍒犻櫎鏌愪釜鐗瑰畾鐗堟湰锛屽垯搴斾娇鐢?/sys/kernel/tracing/dynamic_events 鏂囦欢鏉ュ垹闄よ鐗瑰畾鏍煎紡鐨勪簨浠躲€?

### 娉ㄩ攢

濡傛灉鍦ㄦ敞鍐屾煇涓簨浠朵箣鍚庝笉鍐嶅笇鏈涘畠琚洿鏂帮紝鍒欏彲浠ラ€氳繃瀵?/sys/kernel/tracing/user_events_data 鏂囦欢鍙戝嚭 ioctl() 鏉ョ鐢ㄥ畠銆?
瑕佸彂鍑虹殑鍛戒护鏄?DIAG_IOCSUNREG銆傝繖涓庡垹闄や笉鍚岋紝鍒犻櫎浼氱湡姝ｅ皢浜嬩欢浠庣郴缁熶腑绉婚櫎銆傛敞閿€鍙槸鍛婅瘔鍐呮牳浣犵殑杩涚▼
涓嶅啀鍏冲績璇ヤ簨浠剁殑鏇存柊銆?

```

  struct user_unreg {
        /* Input: Size of the user_unreg structure being used */
        __u32 size;

        /* Input: Bit to unregister */
        __u8 disable_bit;

        /* Input: Reserved, set to 0 */
        __u8 __reserved;

        /* Input: Reserved, set to 0 */
        __u16 __reserved2;

        /* Input: Address to unregister */
        __u64 disable_addr;
  } __attribute__((__packed__));

```
struct user_unreg 瑕佹眰姝ｇ‘璁剧疆涓婅堪鎵€鏈夎緭鍏ュ瓧娈点€?

- size锛氬繀椤昏缃负 sizeof(struct user_unreg)銆?

- disable_bit锛氬繀椤昏缃负瑕佺鐢ㄧ殑浣嶏紙鍗充箣鍓嶉€氳繃 enable_bit 娉ㄥ唽鐨勫悓涓€涓綅锛夈€?

- disable_addr锛氬繀椤昏缃负瑕佺鐢ㄧ殑鍦板潃锛堝嵆涔嬪墠閫氳繃 enable_addr 娉ㄥ唽鐨勫悓涓€涓湴鍧€锛夈€?

**娉ㄦ剰锛?* 浜嬩欢鍦ㄨ皟鐢?execve() 鏃朵細鑷姩娉ㄩ攢銆傚湪 fork() 鏈熼棿锛屽凡娉ㄥ唽鐨勪簨浠朵細琚繚鐣欙紝濡傛灉甯屾湜娉ㄩ攢锛屽繀椤诲湪姣忎釜杩涚▼涓墜鍔ㄦ敞閿€銆?

### 鐘舵€?

褰撳伐鍏锋寕鎺?褰曞埗鍩轰簬鐢ㄦ埛鐨勪簨浠舵椂锛屼簨浠剁殑鐘舵€佷細瀹炴椂鏇存柊銆傝繖浣跨敤鎴风▼搴忓彧鍦ㄦ湁涓滆タ鐪熸鎸傛帴鍒拌浜嬩欢鏃讹紝鎵嶆壙鎷?write() 鎴?
writev() 璋冪敤鐨勫紑閿€銆?

闅忕潃宸ュ叿鎸傛帴鎴栬劚绂昏浜嬩欢锛屽唴鏍镐細鏇存柊涓鸿浜嬩欢娉ㄥ唽鐨勬寚瀹氫綅銆傜敤鎴风▼搴忓彧闇€妫€鏌ヨ浣嶆槸鍚﹁缃綅锛屽氨鑳界煡閬撴槸鍚︽湁涓滆タ鎸傛帴銆?

绠＄悊鍛樺彲浠ヨ交鏉炬煡鐪嬫墍鏈夊凡娉ㄥ唽浜嬩欢鐨勭姸鎬侊紝鏂规硶鏄鍙?
```

  Name [# Comments]
  ...

  Active: ActiveCount
  Busy: BusyCount

```
```

  test

  Active: 1
  Busy: 0

```
```

  test # Used by ftrace

  Active: 1
  Busy: 1

```
### 鍐欏叆鏁版嵁

娉ㄥ唽浜嬩欢鍚庯紝鐢ㄤ簬娉ㄥ唽鐨勫悓涓€涓?fd 鍙互鐢ㄦ潵涓鸿浜嬩欢鍐欏叆涓€鏉¤褰曘€傝繑鍥炵殑 write_index 蹇呴』浣嶄簬鏁版嵁鐨勬渶鍓嶉潰锛?
鍏朵綑鏁版嵁鍒欒瑙嗕负璇ヤ簨浠剁殑璐熻浇锛坧ayload锛夈€?

渚嬪锛屽鏋滆繑鍥炵殑 write_index 鏄?1锛岃€屾垜鎯冲啓鍑轰竴涓?int 绫诲瀷鐨勮礋杞斤紝閭ｄ箞鏁版嵁鐨勫ぇ灏忓繀椤讳负 8 瀛楄妭锛? 涓?int锛夛紝
鍏朵腑鍓?4 涓瓧鑺傜瓑浜?1锛屽悗 4 涓瓧鑺傜瓑浜庢垜鎯宠浣滀负璐熻浇鐨勫€笺€?

```

  int index;
  int payload;

```
鐢ㄦ埛绋嬪簭鍙兘鎷ユ湁浼楁墍鍛ㄧ煡鐨勭粨鏋勪綋锛屽笇鏈涘皢鍏朵綔涓鸿礋杞藉彂鍑恒€傚湪杩欑鎯呭喌涓嬪彲浠ヤ娇鐢?writev()锛屽叾涓涓€涓悜閲忔槸绱㈠紩锛?
鍚庣画鐨勫悜閲忔槸瀹為檯鐨勪簨浠惰礋杞姐€?

```

  struct payload {
        int src;
        int dst;
        int flags;
  } __attribute__((__packed__));

```
```

  struct iovec io[2];
  struct payload e;

  io[0].iov_base = &write_index;
  io[0].iov_len = sizeof(write_index);
  io[1].iov_base = &e;
  io[1].iov_len = sizeof(e);

  writev(fd, (const struct iovec*)io, 2);

```
**娉ㄦ剰锛?* **write_index 涓嶄細琚彂鍑哄埌姝ｅ湪褰曞埗鐨?trace 涓€?*

### 绀轰緥浠ｇ爜

绀轰緥浠ｇ爜瑙?samples/user_events銆?

