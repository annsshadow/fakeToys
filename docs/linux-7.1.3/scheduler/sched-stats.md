## 璋冨害鍣ㄧ粺璁★紙Scheduler Statistics锛?


schedstats 鐨勭 17 鐗堢Щ闄や簡 'lb_imbalance' 瀛楁锛屽洜涓哄畠宸?
涓嶅啀鏈夋剰涔夛紝杞€屾坊鍔犱簡鏇村叿鐩稿叧鎬х殑瀛楁锛屽嵆
'lb_imbalance_load'銆?lb_imbalance_util'銆?lb_imbalance_task' 鍜?
'lb_imbalance_misfit'銆俤omain 瀛楁浠庢鐗堟湰璧锋墦鍗?
鐩稿簲璋冨害鍩熺殑鍚嶇О銆?

schedstats 鐨勭 16 鐗堟洿鏀逛簡 'enum cpu_idle_type' 鍐呴儴
瀹氫箟椤哄簭锛屼粠鑰屾敼鍙樹簡 show_schedstat() 涓?
[CPU_MAX_IDLE_TYPES] 鍒楃殑椤哄簭銆傜壒鍒槸 CPU_IDLE
涓?__CPU_NOT_IDLE 鐨勪綅缃簰鎹簡銆傛暟缁勫ぇ灏忎笉鍙樸€?

schedstats 鐨勭 15 鐗堝垹闄や簡閮ㄥ垎 sched_yield 鐨勮鏁板櫒锛?
yld_exp_empty銆亂ld_act_empty 鍜?yld_both_empty銆傞櫎姝や箣澶栵紝
瀹冧笌绗?14 鐗堝畬鍏ㄧ浉鍚屻€傝缁嗕俊鎭

	https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/scheduler/sched-stats.txt?id=1e1dbb259c79b

schedstats 鐨勭 14 鐗堝寘鍚 sched_domains 鐨勬敮鎸侊紝璇ョ壒鎬у湪
2.6.20 杩涘叆涓荤嚎鍐呮牳锛屽敖绠″畠涓庣 12 鐗堢殑缁熻鐩稿悓
锛堢 12 鐗堝瓨鍦ㄤ簬 2.6.13-2.6.19 鍐呮牳涓紝绗?13 鐗堜粠鏈彂甯冿級銆?
 鏌愪簺璁℃暟鍣ㄦ洿閫傚悎鎸夎繍琛岄槦鍒楋紙runqueue锛夌粺璁★紝鍙︿竴浜涘垯
 鎸夊煙缁熻銆傝娉ㄦ剰锛屽煙锛堝強鍏剁浉鍏充俊鎭級浠呭湪
 浣跨敤 CONFIG_SMP 鐨勬満鍣ㄤ笂鎵嶇浉鍏充笖鍙敤銆?

鍦?schedstat 绗?14 鐗堜腑锛屽垪鍑虹殑姣忎釜 cpu 鑷冲皯鏈変竴绾?
鍩熺粺璁★紝骞朵笖寰堝彲鑳戒笉姝竴绾?
鍩熴€傚湪姝ゅ疄鐜颁腑鍩熸病鏈夌壒瀹氬悕绉帮紝浣?
 缂栧彿鏈€楂樼殑鍩熼€氬父璐熻矗鍗忚皟鏁存満涓婃墍鏈?
 cpu 鐨勫潎琛★紝鑰?domain0 鏄渶鑱氱劍鐨勫煙锛?
 鏈夋椂浠呭湪鎴愬鐨?cpu 涔嬮棿杩涜鍧囪　銆傜洰鍓?
 娌℃湁鏋舵瀯闇€瑕佽秴杩囦笁绾у煙銆傚煙缁熻涓殑
 绗竴涓瓧娈垫槸涓€涓綅鍥撅紝鎸囩ず鍝簺 cpu 鍙楄鍩熷奖鍝?
 銆傝缁嗕俊鎭

	https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/sched-stats.txt?id=b762f3ffb797c

schedstat 鏂囨。浠庣 10 鐗堣捣缁存姢锛岀 11 鍜?12 鐗堟湭鏇存柊銆?
绗?10 鐗堢殑璇︾粏淇℃伅瑙?

	https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/sched-stats.txt?id=1da177e4c3f4

杩欎簺瀛楁閮芥槸璁℃暟鍣紝鍙細閫掑銆備娇鐢ㄨ繖浜涘瓧娈电殑
绋嬪簭闇€瑕佸厛杩涜涓€娆″熀绾胯娴嬶紝鐒跺悗璁＄畻
姣忔鍚庣画瑙傛祴鏃惰鏁板櫒鐨勫彉鍖栥€傛湁涓€涓?perl 鑴氭湰
鍙互瀵瑰叾涓澶氬瓧娈垫墽琛屾鎿嶄綔锛岃

    http://eaglet.pdxhosts.com/rick/linux/schedstat/

璇锋敞鎰忥紝浠讳綍姝ょ被鑴氭湰閮藉繀鐒朵笌鐗堟湰鐩稿叧锛屽洜涓烘洿鏀圭増鏈殑
涓昏鍘熷洜灏辨槸杈撳嚭鏍煎紡鐨勫彉鍔ㄣ€傚浜庡笇鏈?
缂栧啓鑷繁鑴氭湰鐨勪汉锛屽瓧娈靛湪姝ゆ弿杩般€?

### CPU 缁熻

cpu<N> 1 2 3 4 5 6 7 8 9

绗竴涓瓧娈垫槸 sched_yield() 缁熻锛?

     1) 璋冪敤 sched_yield() 鐨勬鏁?

鎺ヤ笅鏉ヤ笁涓槸 schedule() 缁熻锛?

     2) 璇ュ瓧娈垫槸 O(1) 璋冨害鍣ㄤ腑閬楃暀鐨勬暟缁勮繃鏈熻鏁帮紝鍑轰簬 ABI 鍏煎鎬т簣浠ヤ繚鐣欙紝浣嗗叾鍊煎缁堜负闆躲€?
     3) 璋冪敤 schedule() 鐨勬鏁?
     4) schedule() 浣垮鐞嗗櫒杩涘叆绌洪棽鐘舵€佺殑娆℃暟

鎺ヤ笅鏉ヤ袱涓槸 try_to_wake_up() 缁熻锛?

     5) 璋冪敤 try_to_wake_up() 鐨勬鏁?
     6) 涓哄敜閱掓湰鍦?cpu 鑰岃皟鐢?try_to_wake_up() 鐨勬鏁?

鎺ヤ笅鏉ヤ笁涓槸鎻忚堪璋冨害寤惰繜鐨勭粺璁★細

     7) 鏈鐞嗗櫒涓婂悇浠诲姟杩愯鎵€鑰楄垂鐨勬€绘椂闂达紙绾崇锛?
     8) 鏈鐞嗗櫒涓婂悇浠诲姟绛夊緟杩愯鎵€鑰楄垂鐨勬€绘椂闂达紙绾崇锛?
     9) 鍦ㄦ湰 cpu 涓婅繍琛岀殑鏃剁墖锛坱imeslice锛夋暟閲?


### 鍩熺粺璁?

瀵规瘡涓墍鎻忚堪鐨?cpu锛屾瘡涓煙閮戒細鐢熸垚涓€琛岃繖鏍风殑杈撳嚭銆傦紙璇锋敞鎰忥紝濡傛灉
 鏈畾涔?CONFIG_SMP锛屽垯**涓嶄細**浣跨敤浠讳綍鍩燂紝杩欎簺琛?
 涓嶄細鍑虹幇鍦ㄨ緭鍑轰腑銆傦級

domain<N> <name> <cpumask> 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32 33 34 35 36 37 38 39 40 41 42 43 44 45

<name> 瀛楁鎵撳嵃璋冨害鍩熺殑鍚嶇О锛屼粎鍦?schedstat 鐗堟湰 >= 17 鏃舵敮鎸併€?
鍦ㄤ箣鍓嶇殑鐗堟湰涓紝<cpumask> 鏄涓€涓?
瀛楁銆?

<cpumask> 瀛楁鏄竴涓綅鎺╃爜锛屾寚绀鸿鍩熷湪鍝簺 cpu 涓婅繍琛?
銆?

鎺ヤ笅鏉ョ殑 33 涓瓧娈垫槸 sched_balance_rq() 鐨勫悇绉嶇粺璁★紝鎸?
绌洪棽绫诲瀷锛坆usy銆乮dle 鍜?newly idle锛夊垎缁勶細

    1) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃惰皟鐢?sched_balance_rq() 鐨勬鏁?
    2) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃?sched_balance_rq() 妫€鏌ュ悗鍙戠幇璐熻浇鏃犻渶鍧囪　鐨勬鏁?
    3) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃?sched_balance_rq() 灏濊瘯杩佺Щ涓€涓垨澶氫釜浠诲姟浣嗗け璐ョ殑娆℃暟
    4) cpu 绻佸繖鏃讹紝姝ゅ煙鍐呰礋杞界殑鎬讳笉鍧囪　閲?
    5) cpu 绻佸繖鏃讹紝姝ゅ煙鍐呭埄鐢ㄧ巼鐨勬€讳笉鍧囪　閲?
    6) cpu 绻佸繖鏃讹紝姝ゅ煙鍐呬换鍔℃€绘暟鐨勬€讳笉鍧囪　閲?
    7) cpu 绻佸繖鏃讹紝姝ゅ煙鍐呯敱涓嶅尮閰嶏紙misfit锛変换鍔″鑷寸殑鎬讳笉鍧囪　閲?
    8) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃惰皟鐢?detach_task() 鐨勬鏁?
    9) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃跺嵆浣跨洰鏍囦换鍔?cache-hot 浠嶈皟鐢?detach_task() 鐨勬鏁?
    10) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃?sched_balance_rq() 琚皟鐢ㄤ絾鏈壘鍒版洿绻佸繖闃熷垪鐨勬鏁?
    11) 鍦ㄦ鍩熶腑锛宑pu 绻佸繖鏃跺彂鐜版洿绻佸繖鐨勯槦鍒椾絾鏈彂鐜版洿绻佸繖鐨勫垎缁勶紙group锛夌殑娆℃暟

    12) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃惰皟鐢?sched_balance_rq() 鐨勬鏁?
    13) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃?sched_balance_rq() 妫€鏌ュ悗鍙戠幇璐熻浇鏃犻渶鍧囪　鐨勬鏁?
    14) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃?sched_balance_rq() 灏濊瘯杩佺Щ涓€涓垨澶氫釜浠诲姟浣嗗け璐ョ殑娆℃暟
    15) cpu 绌洪棽鏃讹紝姝ゅ煙鍐呰礋杞界殑鎬讳笉鍧囪　閲?
    16) cpu 绌洪棽鏃讹紝姝ゅ煙鍐呭埄鐢ㄧ巼鐨勬€讳笉鍧囪　閲?
    17) cpu 绌洪棽鏃讹紝姝ゅ煙鍐呬换鍔℃€绘暟鐨勬€讳笉鍧囪　閲?
    18) cpu 绌洪棽鏃讹紝姝ゅ煙鍐呯敱涓嶅尮閰嶏紙misfit锛変换鍔″鑷寸殑鎬讳笉鍧囪　閲?
    19) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃惰皟鐢?detach_task() 鐨勬鏁?
    20) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃跺嵆浣跨洰鏍囦换鍔?cache-hot 浠嶈皟鐢?detach_task() 鐨勬鏁?
    21) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃?sched_balance_rq() 琚皟鐢ㄤ絾鏈壘鍒版洿绻佸繖闃熷垪鐨勬鏁?
    22) 鍦ㄦ鍩熶腑锛宑pu 绌洪棽鏃跺彂鐜版洿绻佸繖鐨勯槦鍒椾絾鏈彂鐜版洿绻佸繖鐨勫垎缁勶紙group锛夌殑娆℃暟

    23) 鍦ㄦ鍩熶腑锛宑pu 鍗冲皢杩涘叆绌洪棽鏃惰皟鐢?sched_balance_rq() 鐨勬鏁?
    24) 鍦ㄦ鍩熶腑锛宑pu 鍗冲皢杩涘叆绌洪棽鏃?sched_balance_rq() 妫€鏌ュ悗鍙戠幇璐熻浇鏃犻渶鍧囪　鐨勬鏁?
    25) 鍦ㄦ鍩熶腑锛宑pu 鍗冲皢杩涘叆绌洪棽鏃?sched_balance_rq() 灏濊瘯杩佺Щ涓€涓垨澶氫釜浠诲姟浣嗗け璐ョ殑娆℃暟
    26) cpu 鍗冲皢杩涘叆绌洪棽鏃讹紝姝ゅ煙鍐呰礋杞界殑鎬讳笉鍧囪　閲?
    27) cpu 鍗冲皢杩涘叆绌洪棽鏃讹紝姝ゅ煙鍐呭埄鐢ㄧ巼鐨勬€讳笉鍧囪　閲?
    28) cpu 鍗冲皢杩涘叆绌洪棽鏃讹紝姝ゅ煙鍐呬换鍔℃€绘暟鐨勬€讳笉鍧囪　閲?
    29) cpu 鍗冲皢杩涘叆绌洪棽鏃讹紝姝ゅ煙鍐呯敱涓嶅尮閰嶏紙misfit锛変换鍔″鑷寸殑鎬讳笉鍧囪　閲?
    30) 鍦ㄦ鍩熶腑锛屾柊杩涘叆绌洪棽锛坣ewly idle锛夌姸鎬佹椂璋冪敤 detach_task() 鐨勬鏁?
    31) 鍦ㄦ鍩熶腑锛宑pu 鍗冲皢杩涘叆绌洪棽鏃跺嵆浣跨洰鏍囦换鍔?cache-hot 浠嶈皟鐢?detach_task() 鐨勬鏁?
    32) 鍦ㄦ鍩熶腑锛宑pu 鍗冲皢杩涘叆绌洪棽鏃?sched_balance_rq() 琚皟鐢ㄤ絾鏈壘鍒版洿绻佸繖闃熷垪鐨勬鏁?
    33) 鍦ㄦ鍩熶腑锛宑pu 鍗冲皢杩涘叆绌洪棽鏃跺彂鐜版洿绻佸繖鐨勯槦鍒椾絾鏈彂鐜版洿绻佸繖鐨勫垎缁勶紙group锛夌殑娆℃暟

   鎺ヤ笅鏉ョ殑涓夐」涓?active_load_balance() 鐨勭粺璁★細

    34) 璋冪敤 active_load_balance() 鐨勬鏁?
    35) active_load_balance() 灏濊瘯杩佺Щ浠诲姟浣嗗け璐ョ殑娆℃暟
    36) active_load_balance() 鎴愬姛杩佺Щ浠诲姟鐨勬鏁?

   鎺ヤ笅鏉ョ殑涓夐」涓?sched_balance_exec() 鐨勭粺璁★細

    37) sbe_cnt 鏈浣跨敤
    38) sbe_balanced 鏈浣跨敤
    39) sbe_pushed 鏈浣跨敤

   鎺ヤ笅鏉ョ殑涓夐」涓?sched_balance_fork() 鐨勭粺璁★細

    40) sbf_cnt 鏈浣跨敤
    41) sbf_balanced 鏈浣跨敤
    42) sbf_pushed 鏈浣跨敤

   鎺ヤ笅鏉ョ殑涓夐」涓?try_to_wake_up() 鐨勭粺璁★細

    43) 鍦ㄦ鍩熶腑锛宼ry_to_wake_up() 鍞ら啋浜嗕笂娆¤繍琛屼簬鏈煙涓彟涓€ cpu 鐨勪换鍔＄殑娆℃暟
    44) 鍦ㄦ鍩熶腑锛宼ry_to_wake_up() 灏嗕换鍔¤縼绉诲埌鍞ら啋 cpu 鐨勬鏁帮紙鍥犲叾鑷韩 cpu 涓?cache-cold锛?
    45) 鍦ㄦ鍩熶腑锛宼ry_to_wake_up() 鍚姩琚姩鍧囪　锛坧assive balancing锛夌殑娆℃暟

### /proc/<pid>/schedstat

schedstats 杩樻柊澧炰簡涓€涓?/proc/<pid>/schedstat 鏂囦欢锛屼互鍖呭惈
杩涚▼绾у埆鐨勭浉鍚屼俊鎭€傝鏂囦欢涓?
鏈変笁涓瓧娈碉紝瀵瑰簲浜庤杩涚▼锛?

     1) 鍦?cpu 涓婅姳璐圭殑鏃堕棿锛堢撼绉掞級
     2) 鍦ㄨ繍琛岄槦鍒椾笂绛夊緟鐨勬椂闂达紙绾崇锛?
     3) 鍦ㄦ湰 cpu 涓婅繍琛岀殑鏃剁墖锛坱imeslice锛夋暟閲?

鍙互寰堝鏄撳湴缂栧啓涓€涓▼搴忔潵鍒╃敤杩欎簺棰濆瀛楁锛屼互
鎶ュ憡鐗瑰畾杩涚▼鎴栦竴缁勮繘绋嬪湪
璋冨害鍣ㄧ瓥鐣ヤ笅鐨勮繍琛屾儏鍐点€傛绫荤▼搴忕殑涓€涓畝鍗曠増鏈
銆?


    http://eaglet.pdxhosts.com/rick/linux/schedstat/v12/latency.c
