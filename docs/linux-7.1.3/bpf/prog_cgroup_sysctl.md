
## BPF_PROG_TYPE_CGROUP_SYSCTL


鏈枃妗ｆ弿杩?`BPF_PROG_TYPE_CGROUP_SYSCTL` 绋嬪簭绫诲瀷锛屽畠涓?sysctl 鎻愪緵 cgroup-bpf
閽╁瓙銆?
璇ラ挬瀛愬繀椤婚檮鍔犲埌涓€涓?cgroup锛屽苟涓旀瘡褰撹 cgroup 鍐呯殑杩涚▼灏濊瘯浠庢垨鍚?proc 涓殑
sysctl 鏃嬮挳璇诲彇鎴栧啓鍏ユ椂閮戒細琚皟鐢ㄣ€?
######## 1. 闄勫姞绫诲瀷


蹇呴』浣跨敤 `BPF_CGROUP_SYSCTL` 闄勫姞绫诲瀷鏉ュ皢 `BPF_PROG_TYPE_CGROUP_SYSCTL` 绋嬪簭
闄勫姞鍒?cgroup銆?
######## 2. 涓婁笅鏂?

`BPF_PROG_TYPE_CGROUP_SYSCTL` 鎻愪緵瀵逛互涓嬩笂涓嬫枃鐨勮闂紝鏉ヨ嚜
```

    struct bpf_sysctl {
        __u32 write;
        __u32 file_pos;
    };

```
- `write` 鎸囩ず sysctl 鍊兼琚鍙栵紙`0`锛夎繕鏄啓鍏ワ紙`1`锛夈€傝瀛楁鏄彧璇荤殑銆?
- `file_pos` 鎸囩ず姝ｅ湪琚闂€佽鍙栨垨鍐欏叆鐨?sysctl 鐨勬枃浠朵綅缃€傝瀛楁鏄彲璇诲啓鐨勩€?  鍐欏叆璇ュ瓧娈典細璁剧疆 sysctl proc 鏂囦欢鐨勮捣濮嬩綅缃紝闅忓悗鐨?`read(2)` 灏嗕粠璇ヤ綅缃鍙栵紝
  鎴?`write(2)` 灏嗗啓鍏ヨ浣嶇疆銆備緥濡傦紝鍗充娇鍦ㄧ敱鐢ㄦ埛绌洪棿鍦?`file_pos > 0` 鏃惰皟鐢?  `write(2)` 鐨勬儏鍐典笅锛屼篃鍙互鍚戣瀛楁鍐欏叆闆讹紝浠庤€岄€氳繃 `bpf_sysctl_set_new_value()`
  瑕嗙洊鏁翠釜 sysctl 鍊笺€傚悜璇ュ瓧娈靛啓鍏ラ潪闆跺€煎彲鐢ㄤ簬浠庢寚瀹氱殑 `file_pos` 寮€濮嬭闂儴鍒?  sysctl 鍊笺€傚苟闈炴墍鏈?sysctl 閮芥敮鎸佷互 ``file_pos != 0`` 璁块棶锛屼緥濡傚鏁板€煎瀷
  sysctl 鏉＄洰鐨勫啓鍏ュ繀椤诲缁堝湪鏂囦欢浣嶇疆 `0`銆傚彟璇峰弬瑙?`kernel.sysctl_writes_strict`
  sysctl銆?
鍏充簬濡備綍璁块棶涓婁笅鏂囧瓧娈电殑鏇村缁嗚妭锛岃鍙傝 `linux/bpf.h`_銆?
######## 3. 杩斿洖鐮?

`BPF_PROG_TYPE_CGROUP_SYSCTL` 绋嬪簭蹇呴』杩斿洖浠ヤ笅杩斿洖鐮佷箣涓€锛?
- `0` 琛ㄧず鈥滄嫆缁濊闂?sysctl鈥濓紱
- `1` 琛ㄧず鈥滅户缁闂€濄€?
濡傛灉绋嬪簭杩斿洖 `0`锛岀敤鎴风┖闂村皢浠?`read(2)` 鎴?`write(2)` 寰楀埌 `-1`锛屽苟涓?`errno`
灏嗚璁句负 `EPERM`銆?
######## 4. 杈呭姪鍑芥暟


鐢变簬 sysctl 鏃嬮挳鐢卞悕绉板拰鍊艰〃绀猴紝sysctl 涓撶敤鐨?BPF 杈呭姪鍑芥暟渚ч噸浜庢彁渚涘杩欎簺
灞炴€х殑璁块棶锛?
- `bpf_sysctl_get_name()` 鐢ㄤ簬鑾峰彇 sysctl 鍚嶇О锛堟濡傚畠鍦?`/proc/sys` 涓彲瑙佺殑
  閭ｆ牱锛夛紝鏀惧叆鐢?BPF 绋嬪簭鎻愪緵鐨勭紦鍐插尯涓紱

- `bpf_sysctl_get_current_value()` 鐢ㄤ簬鑾峰彇 sysctl 褰撳墠鎸佹湁鐨勫瓧绗︿覆鍊硷紝鏀惧叆鐢?  BPF 绋嬪簭鎻愪緵鐨勭紦鍐插尯涓€傝杈呭姪鍑芥暟鍦ㄥ sysctl 鐨?`read(2)` 鍜?`write(2)` 涓?  閮藉彲鐢紱

- `bpf_sysctl_get_new_value()` 鐢ㄤ簬鍦ㄥ疄闄呭啓鍏ュ彂鐢熶箣鍓嶏紝鑾峰彇褰撳墠姝ｈ鍐欏叆 sysctl
  鐨勬柊瀛楃涓插€笺€傝杈呭姪鍑芥暟鍙兘鐢ㄤ簬 `ctx->write == 1`锛?
- `bpf_sysctl_set_new_value()` 鐢ㄤ簬鍦ㄥ疄闄呭啓鍏ュ彂鐢熶箣鍓嶏紝瑕嗙洊褰撳墠姝ｈ鍐欏叆 sysctl
  鐨勬柊瀛楃涓插€笺€俿ysctl 鍊煎皢浠庡綋鍓嶇殑 `ctx->file_pos` 寮€濮嬭瑕嗙洊銆傚鏋滆瑕嗙洊
  鏁翠釜鍊硷紝BPF 绋嬪簭鍙互鍦ㄨ皟鐢ㄨ杈呭姪鍑芥暟涔嬪墠灏?`file_pos` 璁句负闆躲€傝杈呭姪鍑芥暟鍙兘
  鐢ㄤ簬 `ctx->write == 1`銆傜敱璇ヨ緟鍔╁嚱鏁拌缃殑鏂板瓧绗︿覆鍊间細琚唴鏍镐互涓庣敤鎴风┖闂翠紶鍏?  鐨勭瓑鏁堝瓧绗︿覆鐩稿悓鐨勬柟寮忓寰呭拰鏍￠獙銆?
BPF 绋嬪簭鐪嬪緟 sysctl 鍊肩殑鏂瑰紡涓庣敤鎴风┖闂村湪 proc 鏂囦欢绯荤粺涓浉鍚岋紝鍗充綔涓轰竴涓瓧绗︿覆銆?鐢变簬璁稿 sysctl 鍊艰〃绀烘暣鏁版垨鏁存暟鍚戦噺锛屼互涓嬭緟鍔╁嚱鏁板彲鐢ㄤ簬浠庡瓧绗︿覆涓幏鍙栨暟鍊硷細

- `bpf_strtol()` 鐢ㄤ簬灏嗗瓧绗︿覆鐨勫垵濮嬮儴鍒嗚浆鎹负 long 鏁存暟锛岀被浼间簬鐢ㄦ埛绌洪棿鐨?  `strtol(3)`_锛?- `bpf_strtoul()` 鐢ㄤ簬灏嗗瓧绗︿覆鐨勫垵濮嬮儴鍒嗚浆鎹负 unsigned long 鏁存暟锛岀被浼间簬鐢ㄦ埛
  绌洪棿鐨?`strtoul(3)`_锛?
鍏充簬姝ゅ鎻忚堪杈呭姪鍑芥暟鐨勬洿澶氱粏鑺傦紝璇峰弬瑙?`linux/bpf.h`_銆?
######## 5. 绀轰緥


璇峰弬瑙?`test_sysctl_prog.c`_锛屼互鑾峰彇涓€涓敤 C 缂栧啓鐨?BPF 绋嬪簭绀轰緥锛岃绋嬪簭璁块棶
sysctl 鍚嶇О鍜屽€硷紝瑙ｆ瀽瀛楃涓插€间互鑾峰彇鏁存暟鍚戦噺锛屽苟鎹鍋氬嚭鍏佽鎴栨嫆缁濊闂?sysctl
鐨勫喅瀹氥€?
######## 6. 娉ㄦ剰浜嬮」


`BPF_PROG_TYPE_CGROUP_SYSCTL` 鏃ㄥ湪鐢ㄤ簬**鍙俊鐨?* root 鐜锛屼緥濡傜敤浜庣洃鎺?sysctl
鐨勪娇鐢紝鎴栨崟鑾蜂互 root 韬唤鍦ㄧ嫭绔?cgroup 涓繍琛岀殑搴旂敤绋嬪簭璇曞浘璁剧疆鐨勪笉鍚堢悊鍊笺€?
鐢变簬鍦?`sys_read` / `sys_write` 鏃惰皟鐢ㄤ簡 `task_dfl_cgroup(current)`锛屽畠鍙兘杩斿洖
涓?`sys_open` 鏃朵笉鍚岀殑缁撴灉锛屽嵆鍦?proc 鏂囦欢绯荤粺涓墦寮€ sysctl 鏂囦欢鐨勮繘绋嬪彲鑳戒笉鍚屼簬
姝ｅ皾璇曚粠/鍚戝畠璇诲彇鎴栧啓鍏ョ殑杩涚▼锛屽苟涓旇繖鏍蜂袱涓繘绋嬪彲鑳借繍琛屽湪涓嶅悓鐨?cgroup 涓紝杩?鎰忓懗鐫€ `BPF_PROG_TYPE_CGROUP_SYSCTL` 涓嶅簲琚敤浣滈檺鍒?sysctl 浣跨敤鐨勫畨鍏ㄦ満鍒躲€?
涓庝换浣?cgroup-bpf 绋嬪簭涓€鏍凤紝濡傛灉浠?root 韬唤鍦?cgroup 涓繍琛岀殑搴旂敤绋嬪簭涓嶅簲琚厑璁?鍒嗙/鏇挎崲鐢辩鐞嗗憳闄勫姞鐨?BPF 绋嬪簭锛屽垯搴斿綋棰濆灏忓績銆?
   ../../tools/testing/selftests/bpf/progs/test_sysctl_prog.c
