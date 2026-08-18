锘?
## /proc 鏂囦欢绯荤粺


=====================  =======================================  ================
/proc/sys              Terrehon Bowden <terrehon@pacbell.net>,  1999 骞?10 鏈?7 鏃?
                       Bodo Bauer <bb@ricochet.net>
2.4.x update	       Jorge Nerin <comandante@zaralinux.com>   2000 骞?11 鏈?14 鏃?
move /proc/sys	       Shen Feng <shen@cn.fujitsu.com>	       2009 骞?4 鏈?1 鏃?
fixes/update part 1.1  Stefani Seibold <stefani@seibold.net>    2009 骞?6 鏈?9 鏃?
=====================  =======================================  ================



  0     鍓嶈█
  0.1	绠€浠?鑷磋阿
  0.2	娉曞緥澹版槑

  1	鏀堕泦绯荤粺淇℃伅
  1.1	杩涚▼鐗瑰畾鐨勫瓙鐩綍
  1.2	鍐呮牳鏁版嵁
  1.3	/proc/ide 涓殑 IDE 璁惧
  1.4	/proc/net 涓殑缃戠粶淇℃伅
  1.5	SCSI 淇℃伅
  1.6	/proc/parport 涓殑骞惰绔彛淇℃伅
  1.7	/proc/tty 涓殑 TTY 淇℃伅
  1.8	/proc/stat 涓殑鏉傞」鍐呮牳缁熻
  1.9	Ext4 鏂囦欢绯荤粺鍙傛暟

  2	淇敼绯荤粺鍙傛暟

  3	姣忚繘绋嬪弬鏁?
  3.1	/proc/<pid>/oom_adj & /proc/<pid>/oom_score_adj - 璋冩暣 oom-killer
							鍒嗘暟
  3.2	/proc/<pid>/oom_score - 鏄剧ず褰撳墠 oom-killer 鍒嗘暟
  3.3	/proc/<pid>/io - 鏄剧ず IO 缁熻瀛楁
  3.4	/proc/<pid>/coredump_filter - 鏍稿績杞偍杩囨护璁剧疆
  3.5	/proc/<pid>/mountinfo - 鍏充簬鎸傝浇鐨勪俊鎭?
  3.6	/proc/<pid>/comm  & /proc/<pid>/task/<tid>/comm
  3.7   /proc/<pid>/task/<tid>/children - 鍏充簬浠诲姟瀛愯繘绋嬬殑淇℃伅
  3.8   /proc/<pid>/fdinfo/<fd> - 鍏充簬宸叉墦寮€鏂囦欢鐨勪俊鎭?
  3.9   /proc/<pid>/map_files - 鍏充簬鍐呭瓨鏄犲皠鏂囦欢鐨勪俊鎭?
  3.10  /proc/<pid>/timerslack_ns - 浠诲姟 timerslack 鍊?
  3.11	/proc/<pid>/patch_state - Livepatch 琛ヤ竵鎿嶄綔鐘舵€?
  3.12	/proc/<pid>/arch_status - 浠诲姟鏋舵瀯鐗瑰畾淇℃伅
  3.13  /proc/<pid>/fd - 鎸囧悜鎵撳紑鏂囦欢鐨勭鍙烽摼鎺ュ垪琛?
  3.14  /proc/<pid>/ksm_stat - 鍏充簬杩涚▼ ksm 鐘舵€佺殑淇℃伅

  4	閰嶇疆 procfs
  4.1	鎸傝浇閫夐」

  5	鏂囦欢绯荤粺琛屼负

## 鍓嶈█


### 0.1 绠€浠?鑷磋阿


鎴戜滑瑕佹劅璋?Alan Cox銆丷ik van Riel銆丄lexey Kuznetsov 浠ュ強璁稿鍏朵粬浜猴紝鎰熻阿浠栦滑甯姪缂栧啓鏈枃妗ｃ€傛垜浠繕瑕佺壒鍒劅璋?Andi Kleen 鎻愪緵鐨勬枃妗ｏ紝鎴戜滑鍦ㄥ垱寤烘湰鏂囨。鏃跺ぇ閲忎緷璧栧畠锛屼互鍙婁粬鎻愪緵鐨勯澶栦俊鎭€傛劅璋㈡墍鏈変负 Linux 鍐呮牳璐＄尞婧愪唬鐮佹垨鏂囨。锛屽苟甯姪鍒涢€犺繖娆惧嚭鑹茶蒋浠剁殑鍏朵粬浜衡€︹€?:)

鏈枃妗ｇ殑鏈€鏂扮増鏈彲鍦ㄧ嚎鑾峰彇锛?
https://www.kernel.org/doc/html/latest/filesystems/proc.html

### 0.2 娉曞緥澹版槑


鎴戜滑涓嶄繚璇佹湰鏂囨。鐨勬纭€э紝濡傛灉浣犲洜涓烘枃妗ｄ笉姝ｇ‘鑰屾悶涔变簡绯荤粺鏉ユ壘鎴戜滑鎶辨€紝鎴戜滑涓嶄細鎰熷埌璐熻矗鈥︹€?

## 绗?1 绔狅細鏀堕泦绯荤粺淇℃伅


### 鏈珷鍐呭


- 鐮旂┒浼枃浠剁郴缁?/proc 鐨勫睘鎬у強鍏舵彁渚涙鍦ㄨ繍琛岀殑 Linux 绯荤粺淇℃伅鐨勮兘鍔?
- 妫€鏌?/proc 鐨勭粨鏋?
- 鎻ず鏈夊叧鍐呮牳鍜岀郴缁熶腑杩愯杩涚▼鐨勫悇绉嶄俊鎭?

------------------------------------------------------------------------------

proc 鏂囦欢绯荤粺鍏呭綋鍐呮牳鍐呴儴鏁版嵁缁撴瀯鐨勪竴涓帴鍙ｃ€傚畠鍙互鐢ㄦ潵鑾峰彇绯荤粺淇℃伅锛屽苟鍦ㄨ繍琛屾椂锛坰ysctl锛夋洿鏀规煇浜涘唴鏍稿弬鏁般€?

棣栧厛锛屾垜浠潵鐪嬬湅 /proc 鐨勫彧璇婚儴鍒嗐€傚湪绗?2 绔犱腑锛屾垜浠皢灞曠ず濡備綍浣跨敤 /proc/sys 鏉ユ洿鏀硅缃€?

### 1.1 杩涚▼鐗瑰畾鐨勫瓙鐩綍


鐩綍 /proc锛堥櫎鍏朵粬鍐呭澶栵級鍖呭惈绯荤粺涓瘡涓繍琛岃繘绋嬬殑瀛愮洰褰曪紝璇ュ瓙鐩綍浠ヨ繘绋?ID锛圥ID锛夊懡鍚嶃€?

閾炬帴 'self' 鎸囧悜姝ｅ湪璇诲彇璇ユ枃浠剁郴缁熺殑杩涚▼銆傛瘡涓繘绋嬪瓙鐩綍閮藉叿鏈夎〃 1-1 涓垪鍑虹殑鏉＄洰銆?

杩涚▼鍙互鍦ㄦ病鏈変换浣曢澶栨潈闄愮殑鎯呭喌涓嬩粠 /proc/PID/* 璇诲彇鑷韩淇℃伅銆傚綋璇诲彇鍏朵粬杩涚▼鐨?/proc/PID/* 淇℃伅鏃讹紝璇诲彇杩涚▼闇€瑕佸叿鏈?PTRACE_MODE_READ 璁块棶鏉冮檺鐨?CAP_SYS_PTRACE 鑳藉姏锛屾垨鑰呭叿鏈?CAP_PERFMON 鑳藉姏銆傝繖閫傜敤浜庢墍鏈夊彧璇讳俊鎭紝濡?`maps`銆乣environ`銆乣pagemap` 绛夈€傚敮涓€鐨勪緥澶栨槸 `mem` 鏂囦欢锛岀敱浜庡叾璇诲啓鎬ц川锛屽畠闇€瑕佸叿鏈夋洿楂樻潈闄?PTRACE_MODE_ATTACH 鐨?CAP_SYS_PTRACE 鑳藉姏锛汣AP_PERFMON 鑳藉姏涓嶆巿浜堝鍏朵粬杩涚▼鐨?/proc/PID/mem 鐨勮闂潈闄愩€?

娉ㄦ剰锛屽 /proc/<pid> 鎴栧叾鍖呭惈鐨勪换浣曟枃浠舵垨瀛愮洰褰曠殑宸叉墦寮€鏂囦欢鎻忚堪绗︼紝骞朵笉鑳介槻姝?<pid> 鍦ㄨ杩涚▼閫€鍑烘椂琚叾浠栬繘绋嬪鐢ㄣ€傚宸叉墦寮€ /proc/<pid> 鏂囦欢鎻忚堪绗︺€佷笖瀵瑰簲浜庡凡姝讳骸杩涚▼鐨勬搷浣滐紝缁濅笉浼氫綔鐢ㄤ簬鍐呮牳鍙兘纰板阀涔熷垎閰嶄簡杩涚▼ ID <pid> 鐨勪换浣曟柊杩涚▼銆傜浉鍙嶏紝瀵硅繖浜?FD 鐨勬搷浣滈€氬父浼氫互 ESRCH 澶辫触銆?


 =============  ===============================================================
 鏂囦欢	鍐呭
 =============  ===============================================================
 clear_refs	娓呴櫎 smaps 杈撳嚭涓樉绀虹殑椤靛紩鐢ㄤ綅
 cmdline	鍛戒护琛屽弬鏁?
 cpu		褰撳墠鍜屼笂娆℃墽琛屾墍鍦ㄧ殑 cpu	(2.4)(smp)
 cwd		鎸囧悜褰撳墠宸ヤ綔鐩綍鐨勯摼鎺?
 environ	鐜鍙橀噺鐨勫€?
 exe		鎸囧悜璇ヨ繘绋嬪彲鎵ц鏂囦欢鐨勯摼鎺?
 fd		鍖呭惈鎵€鏈夋枃浠舵弿杩扮鐨勭洰褰?
 maps		鍒板彲鎵ц鏂囦欢鍜屽簱鏂囦欢鐨勫唴瀛樻槧灏?(2.4)
 mem		璇ヨ繘绋嬫寔鏈夌殑鍐呭瓨
 root		鎸囧悜璇ヨ繘绋嬫牴鐩綍鐨勯摼鎺?
 stat		杩涚▼鐘舵€?
 statm		杩涚▼鍐呭瓨鐘舵€佷俊鎭?
 status		浜虹被鍙褰㈠紡鐨勮繘绋嬬姸鎬?
 wchan		鍚敤 CONFIG_KALLSYMS=y 鏃跺瓨鍦細鏄剧ず浠诲姟琚樆濉炰簬鍏朵腑鐨勫唴鏍稿嚱鏁?
		绗﹀彿锛屾湭闃诲鍒欎负 "0"銆?
 pagemap	椤佃〃
 stack		鎶ュ憡瀹屾暣鐨勬爤鍥炴函锛岄€氳繃 CONFIG_STACKTRACE 鍚敤
 smaps		鍩轰簬 maps 鐨勬墿灞曪紝鏄剧ず姣忎釜鏄犲皠鐨勫唴瀛樻秷鑰楀強鍏跺叧鑱旂殑鏍囧織
 smaps_rollup	璇ヨ繘绋嬫墍鏈夋槧灏勭殑 smaps 绱缁熻銆傝繖鍙互浠?smaps 鎺ㄥ锛屼絾鏇村揩鏇存柟渚?
 numa_maps	鍩轰簬 maps 鐨勬墿灞曪紝鏄剧ず姣忎釜鏄犲皠鐨勫唴瀛樺眬閮ㄦ€с€佺粦瀹氱瓥鐣ヤ互鍙?
		鍐呭瓨浣跨敤閲忥紙浠ラ〉涓哄崟浣嶏級銆?
 =============  ===============================================================

渚嬪锛岃鑾峰彇杩涚▼鐨勭姸鎬佷俊鎭紝鍙渶锛?

```
  >cat /proc/self/status
  Name:   cat
  State:  R (running)
  Tgid:   5452
  Pid:    5452
  PPid:   743
  TracerPid:      0						(2.4)
  Uid:    501     501     501     501
  Gid:    100     100     100     100
  FDSize: 256
  Groups: 100 14 16
  Kthread:    0
  VmPeak:     5004 kB
  VmSize:     5004 kB
  VmLck:         0 kB
  VmHWM:       476 kB
  VmRSS:       476 kB
  RssAnon:             352 kB
  RssFile:             120 kB
  RssShmem:              4 kB
  VmData:      156 kB
  VmStk:        88 kB
  VmExe:        68 kB
  VmLib:      1412 kB
  VmPTE:        20 kb
  VmSwap:        0 kB
  HugetlbPages:          0 kB
  CoreDumping:    0
  THP_enabled:	  1
  Threads:        1
  SigQ:   0/28578
  SigPnd: 0000000000000000
  ShdPnd: 0000000000000000
  SigBlk: 0000000000000000
  SigIgn: 0000000000000000
  SigCgt: 0000000000000000
  CapInh: 00000000fffffeff
  CapPrm: 0000000000000000
  CapEff: 0000000000000000
  CapBnd: ffffffffffffffff
  CapAmb: 0000000000000000
  NoNewPrivs:     0
  Seccomp:        0
  Speculation_Store_Bypass:       thread vulnerable
  SpeculationIndirectBranch:      conditional enabled
  voluntary_ctxt_switches:        0
  nonvoluntary_ctxt_switches:     1
```

杩欐樉绀虹殑淇℃伅涓庝綘鐢?ps 鍛戒护鐪嬪埌鐨勪俊鎭嚑涔庣浉鍚屻€傚疄闄呬笂锛宲s 浣跨敤 proc 鏂囦欢绯荤粺鏉ヨ幏鍙栧叾淇℃伅銆備絾璇诲彇鏂囦欢 /proc/PID/status 鍙互璁╀綘鏇磋缁嗗湴鏌ョ湅璇ヨ繘绋嬨€傚畠鐨勫瓧娈靛湪琛?1-2 涓弿杩般€?

statm 鏂囦欢鍖呭惈鍏充簬杩涚▼鍐呭瓨浣跨敤鏇磋缁嗙殑淇℃伅銆傚畠鐨勪竷涓瓧娈靛湪琛?1-3 涓В閲娿€俿tat 鏂囦欢鍖呭惈鍏充簬杩涚▼鏈韩鐨勮缁嗕俊鎭€傚畠鐨勫瓧娈靛湪琛?1-4 涓В閲娿€?

锛堥拡瀵?SMP CONFIG 鐢ㄦ埛锛?

涓轰簡浣跨粺璁″彲鎵╁睍锛孯SS 鐩稿叧鐨勪俊鎭互寮傛鏂瑰紡澶勭悊锛屽叾鍊煎彲鑳戒笉鏄緢绮剧‘銆傝鏌ョ湅鏌愪釜鏃跺埢鐨勭簿纭揩鐓э紝浣犲彲浠ユ煡鐪?/proc/<pid>/smaps 鏂囦欢骞舵壂鎻忛〉琛ㄣ€傝繖寰堟參浣嗛潪甯哥簿纭€?


 ==========================  ===================================================
 瀛楁                       鍐呭
 ==========================  ===================================================
 Name                        鍙墽琛屾枃浠剁殑鏂囦欢鍚?
 Umask                       鏂囦欢妯″紡鍒涘缓鎺╃爜
 State                       鐘舵€侊紙R 涓鸿繍琛屼腑锛孲 涓虹潯鐪狅紝D 涓哄浜庝笉鍙腑鏂瓑寰呬腑鐨勭潯鐪狅紝
			     Z 涓哄兊灏歌繘绋嬶紝T 涓鸿璺熻釜鎴栧仠姝級
 Tgid                        绾跨▼缁?ID
 Ngid                        NUMA 缁?ID锛堟棤鍒欎负 0锛?
 Pid                         杩涚▼ id
 PPid                        鐖惰繘绋嬬殑杩涚▼ id
 TracerPid                   璺熻釜姝よ繘绋嬬殑杩涚▼ PID锛堝鏋滄病鏈夛紝鎴栬窡韪€呭湪褰撳墠 pid 鍛藉悕绌洪棿涔嬪鍒欎负 0锛?
 Uid                         鐪熷疄銆佹湁鏁堛€佷繚瀛橀泦鍜屾枃浠剁郴缁?UIDs
 Gid                         鐪熷疄銆佹湁鏁堛€佷繚瀛橀泦鍜屾枃浠剁郴缁?GIDs
 FDSize                      褰撳墠宸插垎閰嶇殑鏂囦欢鎻忚堪绗︽Ы鏁伴噺
 Groups                      琛ュ厖缁勫垪琛?
 NStgid                      鍚庝唬鍛藉悕绌洪棿绾跨▼缁?ID 灞傜骇
 NSpid                       鍚庝唬鍛藉悕绌洪棿杩涚▼ ID 灞傜骇
 NSpgid                      鍚庝唬鍛藉悕绌洪棿杩涚▼缁?ID 灞傜骇
 NSsid                       鍚庝唬鍛藉悕绌洪棿浼氳瘽 ID 灞傜骇
 Kthread                     鍐呮牳绾跨▼鏍囧織锛? 涓烘槸锛? 涓哄惁
 VmPeak                      宄板€艰櫄鎷熷唴瀛樺ぇ灏?
 VmSize                      绋嬪簭鎬诲ぇ灏?
 VmLck                       閿佸畾鍐呭瓨澶у皬
 VmPin                       鍥哄畾锛坧inned锛夊唴瀛樺ぇ灏?
 VmHWM                       宄板€煎父椹婚泦澶у皬锛?楂樻按浣嶆爣璁?锛?
 VmRSS                       鍐呭瓨閮ㄥ垎鐨勫ぇ灏忋€傚畠鍖呭惈浠ヤ笅涓変釜閮ㄥ垎
                             锛圴mRSS = RssAnon + RssFile + RssShmem锛?
 RssAnon                     甯搁┗鍖垮悕鍐呭瓨澶у皬
 RssFile                     甯搁┗鏂囦欢鏄犲皠澶у皬
 RssShmem                    甯搁┗ shmem 鍐呭瓨澶у皬锛堝寘鎷?SysV shm銆?
                             tmpfs 鏄犲皠鍜屽叡浜尶鍚嶆槧灏勶級
 VmData                      绉佹湁鏁版嵁娈靛ぇ灏?
 VmStk                       鏍堟澶у皬
 VmExe                       鏂囨湰娈靛ぇ灏?
 VmLib                       鍏变韩搴撲唬鐮佸ぇ灏?
 VmPTE                       椤佃〃椤瑰ぇ灏?
 VmSwap                      鍖垮悕绉佹湁鏁版嵁浣跨敤鐨勪氦鎹㈤噺
                             锛堜笉鍖呮嫭 shmem 浜ゆ崲浣跨敤閲忥級
 HugetlbPages                hugetlb 鍐呭瓨閮ㄥ垎澶у皬
 CoreDumping                 杩涚▼鍐呭瓨褰撳墠姝ｅ湪琚浆鍌?
                             锛堟潃姝昏繘绋嬪彲鑳藉鑷存牳蹇冩枃浠舵崯鍧忥級
 THP_enabled                 杩涚▼琚厑璁镐娇鐢?THP锛堝綋杩涚▼涓婅缃簡
                             PR_SET_THP_DISABLE 浠ュ畬鍏ㄧ鐢?THP 鏃惰繑鍥?0锛?
                             鑰屼笉浠呬粎鏄儴鍒嗙鐢級
 Threads                     绾跨▼鏁伴噺
 SigQ                        宸叉帓闃熶俊鍙锋暟/闃熷垪鏈€澶ф暟
 SigPnd                      绾跨▼鎸傝捣淇″彿浣嶅浘
 ShdPnd                      杩涚▼鍏变韩鎸傝捣淇″彿浣嶅浘
 SigBlk                      闃诲淇″彿浣嶅浘
 SigIgn                      蹇界暐淇″彿浣嶅浘
 SigCgt                      鎹曡幏淇″彿浣嶅浘
 CapInh                      鍙户鎵胯兘鍔涗綅鍥?
 CapPrm                      鍏佽鐨勮兘鍔涗綅鍥?
 CapEff                      鏈夋晥鑳藉姏浣嶅浘
 CapBnd                      鑳藉姏杈圭晫闆嗕綅鍥?
 CapAmb                      鐜鑳藉姏浣嶅浘
 NoNewPrivs                  no_new_privs锛岀被浼?prctl(PR_GET_NO_NEW_PRIV, ...)
 Seccomp                     seccomp 妯″紡锛岀被浼?prctl(PR_GET_SECCOMP, ...)
 Speculation_Store_Bypass    鎺ㄦ祴鎬у瓨鍌ㄧ粫杩囩紦瑙ｇ姸鎬?
 SpeculationIndirectBranch   闂存帴鍒嗘敮鎺ㄦ祴妯″紡
 Cpus_allowed                璇ヨ繘绋嬪彲鍦ㄥ叾涓婅繍琛岀殑 CPU 鎺╃爜
 Cpus_allowed_list           涓庡墠鑰呯浉鍚岋紝浣嗕负"鍒楄〃鏍煎紡"
 Mems_allowed               璇ヨ繘绋嬪厑璁哥殑鍐呭瓨鑺傜偣鎺╃爜
 Mems_allowed_list           涓庡墠鑰呯浉鍚岋紝浣嗕负"鍒楄〃鏍煎紡"
 voluntary_ctxt_switches     鑷効涓婁笅鏂囧垏鎹㈡鏁?
 nonvoluntary_ctxt_switches  闈炶嚜鎰夸笂涓嬫枃鍒囨崲娆℃暟
 ==========================  ===================================================



 ======== ===============================	==============================
 瀛楁    鍐呭
 ======== ===============================	==============================
 size     绋嬪簭鎬诲ぇ灏忥紙椤碉級			锛堝悓 status 涓殑 VmSize锛?
 resident 鍐呭瓨閮ㄥ垎澶у皬锛堥〉锛?		锛堝悓 status 涓殑 VmRSS锛?
 shared   鍏变韩椤垫暟閲?			锛堝嵆鐢辨枃浠舵敮鎸侊紝鍚?status 涓殑 RssFile+RssShmem锛?
 trs      涓?浠ｇ爜'鐨勯〉鏁伴噺			锛堜笉鍖呮嫭搴擄紱宸叉崯鍧忥紝鍖呭惈鏁版嵁娈碉級
 lrs      搴撻〉鏁伴噺				锛?.6 涓婂缁堜负 0锛?
 drs      鏁版嵁/鏍堢殑椤垫暟閲?		锛堝寘鎷簱锛涘凡鎹熷潖锛屽寘鍚簱鏂囨湰锛?
 dt       鑴忛〉鏁伴噺				锛?.6 涓婂缁堜负 0锛?
 ======== ===============================	==============================



  ============= ===============================================================
  瀛楁         鍐呭
  ============= ===============================================================
  pid           杩涚▼ id
  tcomm         鍙墽琛屾枃浠剁殑鏂囦欢鍚?
  state         鐘舵€侊紙R 涓鸿繍琛屼腑锛孲 涓虹潯鐪狅紝D 涓哄浜庝笉鍙腑鏂瓑寰呬腑鐨勭潯鐪狅紝
                 Z 涓哄兊灏歌繘绋嬶紝T 涓鸿璺熻釜鎴栧仠姝級
  ppid          鐖惰繘绋嬬殑杩涚▼ id
  pgrp          杩涚▼鐨?pgrp
  sid           浼氳瘽 id
  tty_nr        杩涚▼浣跨敤鐨?tty
  tty_pgrp      tty 鐨?pgrp
  flags         浠诲姟鏍囧織
  min_flt       娆¤缂洪〉娆℃暟
  cmin_flt      鍖呭惈瀛愯繘绋嬬殑娆¤缂洪〉娆℃暟
  maj_flt       涓昏缂洪〉娆℃暟
  cmaj_flt      鍖呭惈瀛愯繘绋嬬殑涓昏缂洪〉娆℃暟
  utime         鐢ㄦ埛妯″紡 jiffies
  stime         鍐呮牳妯″紡 jiffies
  cutime        鍖呭惈瀛愯繘绋嬬殑鐢ㄦ埛妯″紡 jiffies
  cstime        鍖呭惈瀛愯繘绋嬬殑鍐呮牳妯″紡 jiffies
  priority      浼樺厛绾х骇鍒?
  nice          nice 绾у埆
  num_threads   绾跨▼鏁伴噺
  it_real_value	(宸插簾寮冿紝濮嬬粓涓?0)
  start_time    杩涚▼鍦ㄧ郴缁熷惎鍔ㄥ悗鍚姩鐨勬椂闂?
  vsize         铏氭嫙鍐呭瓨澶у皬
  rss           甯搁┗闆嗗唴瀛樺ぇ灏?
  rsslim        褰撳墠 rss 鐨勫瓧鑺傛暟闄愬埗
  start_code    绋嬪簭鏂囨湰鍙繍琛岀殑鍦板潃涓婇檺
  end_code      绋嬪簭鏂囨湰鍙繍琛岀殑鍦板潃涓嬮檺
  start_stack   涓昏繘绋嬫爤璧峰鍦板潃
  esp           ESP 褰撳墠鍊?
  eip           EIP 褰撳墠鍊?
  pending       鎸傝捣淇″彿浣嶅浘
  blocked       闃诲淇″彿浣嶅浘
  sigign        蹇界暐淇″彿浣嶅浘
  sigcatch      鎹曡幏淇″彿浣嶅浘
  0		(鍗犱綅绗︼紝鏇句负 wchan 鍦板潃锛?
		鏀圭敤 /proc/PID/wchan)
  0             (鍗犱綅绗?
  0             (鍗犱綅绗?
  exit_signal   閫€鍑烘椂鍙戦€佺粰鐖剁嚎绋嬬殑淇″彿
  task_cpu      浠诲姟琚皟搴﹀埌鐨?CPU
  rt_priority   瀹炴椂浼樺厛绾?
  policy        璋冨害绛栫暐锛坢an sched_setscheduler锛?
  blkio_ticks   绛夊緟鍧楄澶?IO 鑺辫垂鐨勬椂闂?
  gtime         浠诲姟鍦?jiffies 涓殑瀹㈡埛锛坓uest锛夋椂闂?
  cgtime        浠诲姟瀛愯繘绋嬪湪 jiffies 涓殑瀹㈡埛鏃堕棿
  start_data    绋嬪簭鏁版嵁+bss 鏀剧疆鐨勫湴鍧€涓婇檺
  end_data      绋嬪簭鏁版嵁+bss 鏀剧疆鐨勫湴鍧€涓嬮檺
  start_brk     鍙€氳繃 brk() 鎵╁睍绋嬪簭鍫嗙殑鍦板潃涓婇檺
  arg_start     绋嬪簭鍛戒护琛屾斁缃殑鍦板潃涓婇檺
  arg_end       绋嬪簭鍛戒护琛屾斁缃殑鍦板潃涓嬮檺
  env_start     绋嬪簭鐜鏀剧疆鐨勫湴鍧€涓婇檺
  env_end       绋嬪簭鐜鏀剧疆鐨勫湴鍧€涓嬮檺
  exit_code     绾跨▼鐨?exit_code锛屽舰寮忎负 waitpid 绯荤粺璋冪敤鎵€鎶ュ憡鐨勫€?
  ============= ===============================================================

/proc/PID/maps 鏂囦欢鍖呭惈褰撳墠宸叉槧灏勭殑鍐呭瓨鍖哄煙鍙婂叾璁块棶鏉冮檺銆?

```
    address           perms offset  dev   inode      pathname

    08048000-08049000 r-xp 00000000 03:00 8312       /opt/test
    08049000-0804a000 rw-p 00001000 03:00 8312       /opt/test
    0804a000-0806b000 rw-p 00000000 00:00 0          [heap]
    a7cb1000-a7cb2000 ---p 00000000 00:00 0
    a7cb2000-a7eb2000 rw-p 00000000 00:00 0
    a7eb2000-a7eb3000 ---p 00000000 00:00 0
    a7eb3000-a7ed5000 rw-p 00000000 00:00 0
    a7ed5000-a8008000 r-xp 00000000 03:00 4222       /lib/libc.so.6
    a8008000-a800a000 r--p 00133000 03:00 4222       /lib/libc.so.6
    a800a000-a800b000 rw-p 00135000 03:00 4222       /lib/libc.so.6
    a800b000-a800e000 rw-p 00000000 00:00 0
    a800e000-a8022000 r-xp 00000000 03:00 14462      /lib/libpthread.so.0
    a8022000-a8023000 r--p 00013000 03:00 14462      /lib/libpthread.so.0
    a8023000-a8024000 rw-p 00014000 03:00 14462      /lib/libpthread.so.0
    a8024000-a8027000 rw-p 00000000 00:00 0
    a8027000-a8043000 r-xp 00000000 03:00 8317       /lib/ld-linux.so.2
    a8043000-a8044000 r--p 0001b000 03:00 8317       /lib/ld-linux.so.2
    a8044000-a8045000 rw-p 0001c000 03:00 8317       /lib/ld-linux.so.2
    aff35000-aff4a000 rw-p 00000000 00:00 0          [stack]
    ffffe000-fffff000 r-xp 00000000 00:00 0          [vdso]
```

鍏朵腑 "address" 鏄畠鎵€鍗犵敤鐨勮繘绋嬪湴鍧€绌洪棿锛?perms"锛?

```
 r = read
 w = write
 x = execute
 s = shared
 p = private (copy on write)
```

"offset" 鏄槧灏勫唴鐨勫亸绉伙紝"dev" 鏄澶囷紙major:minor锛夛紝"inode" 鏄璁惧涓婄殑 inode銆? 琛ㄧず娌℃湁涓庤鍐呭瓨鍖哄煙鍏宠仈鐨?inode锛孊SS锛堟湭鍒濆鍖栨暟鎹級灏辨槸杩欑鎯呭喌銆?pathname" 鏄剧ず璇ユ槧灏勫叧鑱旂殑鏂囦欢鍚嶃€傚鏋滆鏄犲皠鏈笌鏂囦欢鍏宠仈锛?

 ===================        ===========================================
 [heap]                     绋嬪簭鐨勫爢
 [stack]                    涓昏繘绋嬬殑鏍?
 [vdso]                     "铏氭嫙鍔ㄦ€佸叡浜璞?锛?
                            鍐呮牳绯荤粺璋冪敤澶勭悊绋嬪簭
 [anon:<name>]              鐢辩敤鎴风┖闂村懡鍚嶇殑涓€涓鏈夊尶鍚嶆槧灏?
 [anon_shmem:<name>]        鐢辩敤鎴风┖闂村懡鍚嶇殑涓€涓尶鍚嶅叡浜唴瀛樻槧灏?
 ===================        ===========================================

鎴栬€呭鏋滀负绌猴紝鍒欒鏄犲皠鏄尶鍚嶇殑銆?

浠?6.11 鍐呮牳寮€濮嬶紝/proc/PID/maps 鎻愪緵浜嗕竴涓浛浠ｇ殑鍩轰簬 ioctl() 鐨?API锛岃兘澶熺伒娲讳笖楂樻晥鍦版煡璇㈠拰杩囨护鍗曚釜 VMA銆傝繖涓帴鍙ｆ槸浜岃繘鍒剁殑锛屾棬鍦ㄧ敤浜庢洿楂樻晥銆佹洿鏂逛究鐨勭▼搴忓寲浣跨敤銆俙struct procmap_query`锛堝畾涔夊湪 linux/fs.h UAPI 澶存枃浠朵腑锛変綔涓?`PROCMAP_QUERY` ioctl() 鍛戒护鐨勮緭鍏?杈撳嚭鍙傛暟銆傛湁鍏虫煡璇㈣涔夈€佹敮鎸佺殑鏍囧織銆佽繑鍥炵殑鏁版嵁浠ュ強涓€鑸?API 浣跨敤淇℃伅鐨勮缁嗕俊鎭紝璇峰弬闃?linus/fs.h UAPI 澶存枃浠朵腑鐨勬敞閲娿€?

/proc/PID/smaps 鏄熀浜?maps 鐨勬墿灞曪紝鏄剧ず杩涚▼姣忎釜鏄犲皠鐨勫唴瀛樻秷鑰椼€傚浜庢瘡涓槧灏勶紙鍗宠櫄鎷燂細

```
    08048000-080bc000 r-xp 00000000 03:02 13130      /bin/bash

    Size:               1084 kB
    KernelPageSize:        4 kB
    MMUPageSize:           4 kB
    Rss:                 892 kB
    Pss:                 374 kB
    Pss_Dirty:             0 kB
    Shared_Clean:        892 kB
    Shared_Dirty:          0 kB
    Private_Clean:         0 kB
    Private_Dirty:         0 kB
    Referenced:          892 kB
    Anonymous:             0 kB
    KSM:                   0 kB
    LazyFree:              0 kB
    AnonHugePages:         0 kB
    FilePmdMapped:         0 kB
    ShmemPmdMapped:        0 kB
    Shared_Hugetlb:        0 kB
    Private_Hugetlb:       0 kB
    Swap:                  0 kB
    SwapPss:               0 kB
    Locked:                0 kB
    THPeligible:           0
    VmFlags: rd ex mr mw me dw
```

杩欎簺琛屼腑鐨勭涓€琛屾樉绀虹殑淇℃伅涓?/proc/PID/maps 涓樉绀虹殑鏄犲皠淇℃伅鐩稿悓銆傚悗闈㈢殑琛屾樉绀猴細鏄犲皠鐨勫ぇ灏忥紙size锛夛紱鍦ㄦ敮鎸佷竴涓?VMA 鏃跺垎閰嶇殑鏈€灏忓彲鑳介〉澶у皬锛圞ernelPageSize锛夛紝瀹冩槸鍙慨鏀?VMA 鐨勭矑搴︼紱MMU 鍦ㄦ敮鎸佷竴涓?VMA 鏃跺彲浣跨敤鐨勬渶灏忓彲鑳介〉澶у皬锛圡MUPageSize锛夛紱褰撳墠椹荤暀鍦?RAM 涓殑璇ユ槧灏勭殑鏁伴噺锛圧SS锛夛紱璇ヨ繘绋嬪湪姝ゆ槧灏勪腑鐨勬瘮渚嬩唤棰濓紙PSS锛夛紱浠ュ強璇ユ槧灏勪腑骞插噣鍜岃剰鐨勫叡浜笌绉佹湁椤电殑鏁伴噺銆?

"KernelPageSize" 濮嬬粓瀵瑰簲浜?"MMUPageSize"锛岄櫎闈炲湪 MMU 浣跨敤杈冨皬椤靛ぇ灏忕殑绯荤粺涓婃ā鎷熶簡鏇村ぇ鐨勫唴鏍搁〉澶у皬锛屾煇浜涘甫鏈?hugetlb 鐨?PPC64 閰嶇疆灏辨槸杩欐牱鐨勬儏鍐点€傛澶栵紝"KernelPageSize" 鍜?"MMUPageSize" 濮嬬粓瀵瑰簲浜庡湪 VMA 鏁翠釜鐢熷懡鍛ㄦ湡涓彲鑳介亣鍒扮殑鏈€灏忓彲鑳界矑搴︼紙鍥為€€锛夈€傝繖浜涘€间笉鍙楃敓鏁堜腑鐨勯€忔槑澶ч〉锛圱ransparent Huge Pages锛夛紝鎴栦换浣曞鏇村ぇ MMU 椤靛ぇ灏忕殑浣跨敤锛堟棤璁烘槸閫氳繃鏋舵瀯鎬уぇ椤垫槧灏勶紝杩樻槸 MMU 鎵ц鐨勮櫄鎷熻寖鍥寸殑鍏朵粬鏄惧紡/闅愬紡鍚堝苟锛夌殑褰卞搷銆?AnonHugePages"銆?ShmemPmdMapped" 鍜?"FilePmdMapped" 鎻愪緵浜嗗 PMD 绾у埆鏋舵瀯鎬уぇ椤垫槧灏勪娇鐢ㄦ儏鍐电殑娲炲療銆?

涓€涓繘绋嬬殑"姣斾緥闆嗗ぇ灏?锛圥SS锛夋槸瀹冨湪鍐呭瓨涓嫢鏈夌殑椤佃鏁帮紝鍏朵腑姣忎釜椤甸兘闄や互鍏变韩瀹冪殑杩涚▼鏁伴噺銆傚洜姝わ紝濡傛灉涓€涓繘绋嬫湁 1000 涓〉瀹屽叏褰掕嚜宸辨墍鏈夛紝骞朵笌鍙︿竴涓繘绋嬪叡浜?1000 涓〉锛屽畠鐨?PSS 灏嗕负 1500銆?Pss_Dirty" 鏄?PSS 涓敱鑴忛〉缁勬垚鐨勯儴鍒嗐€傦紙涓嶅寘鍚?"Pss_Clean"锛屼絾鍙互閫氳繃浠?"Pss" 涓噺鍘?"Pss_Dirty" 鏉ヨ绠椼€傦級

浼犵粺涓婏紝涓€涓〉濡傛灉鎭板ソ琚槧灏勪竴娆★紝鍒欒涓?绉佹湁"锛岃€屽綋琚槧灏勫娆℃椂锛堝嵆浣垮湪鍚屼竴涓繘绋嬩腑琚槧灏勫娆★級璁颁负"鍏变韩"銆傛敞鎰忚繖绉嶈璐︾嫭绔嬩簬 MAP_SHARED銆?

鍦ㄦ煇浜涘唴鏍搁厤缃腑锛屽睘浜庢洿澶у垎閰嶏紙渚嬪 THP锛変竴閮ㄥ垎鐨勯〉鐨勮涔夊彲鑳戒笉鍚岋細濡傛灉涓€涓緝澶у垎閰嶇殑鎵€鏈夐〉**纭畾**鏄犲皠鍦ㄥ悓涓€涓繘绋嬩腑锛屽嵆浣胯椤靛湪璇ヨ繘绋嬩腑琚槧灏勫娆★紝涔熻涓?绉佹湁"銆傚鏋滀竴涓緝澶у垎閰嶇殑浠绘剰椤?*鍙兘**鏄犲皠鍦ㄤ笉鍚岀殑杩涚▼涓紝鍒欒涓?鍏变韩"銆傚湪鏌愪簺鎯呭喌涓嬶紝涓€涓緝澶х殑鍒嗛厤鍙兘琚涓?鍙兘琚涓繘绋嬫槧灏?锛屽嵆浣垮疄闄呭凡涓嶅啀濡傛銆?

鏌愪簺鍐呮牳閰嶇疆涓嶈窡韪緝澶у垎閰嶄腑涓€閮ㄥ垎鐨勯〉琚槧灏勭殑绮剧‘娆℃暟銆傚湪杩欑鎯呭喌涓嬶紝璁＄畻 PSS 鏃讹紝鍙兘浼氫娇鐢ㄨ杈冨ぇ鍒嗛厤涓瘡椤电殑骞冲潎鏄犲皠鏁帮紝浣滀负璇ラ〉鏄犲皠鏁伴噺鐨勮繎浼煎€笺€傝繖绉嶆儏鍐典笅 PSS 璁＄畻灏嗕笉绮剧‘銆?

"Referenced" 琛ㄧず褰撳墠琚爣璁颁负寮曠敤鎴栧凡璁块棶鐨勫唴瀛橀噺銆?

"Anonymous" 鏄剧ず涓嶅睘浜庝换浣曟枃浠剁殑鍐呭瓨閲忋€傚嵆浣挎槸涓庢枃浠跺叧鑱旂殑鏄犲皠涔熷彲鑳藉寘鍚尶鍚嶉〉锛氬綋浣跨敤 MAP_PRIVATE 涓旀煇椤佃淇敼鏃讹紝璇ユ枃浠堕〉浼氳涓€涓鏈夌殑鍖垮悕鍓湰鏇挎崲銆?

"KSM" 鎶ュ憡鏈夊灏戦〉鏄?KSM 椤点€傛敞鎰?KSM 鏀剧疆鐨勯浂椤典笉鍖呭惈鍦ㄥ唴锛屽彧鍖呭惈瀹為檯鐨?KSM 椤点€?

"LazyFree" 鏄剧ず鐢?madvise(MADV_FREE) 鏍囪鐨勫唴瀛橀噺銆傚唴瀛樹笉浼氶殢 madvise() 绔嬪嵆閲婃斁銆傚湪鍐呭瓨鍘嬪姏涓嬶紝濡傛灉鍐呭瓨鏄共鍑€鐨勶紝瀹冧細琚噴鏀俱€傝娉ㄦ剰锛岀敱浜庡綋鍓嶅疄鐜颁腑浣跨敤鐨勪紭鍖栵紝鎵撳嵃鐨勫€煎彲鑳戒綆浜庣湡瀹炲€笺€傚鏋滀笉甯屾湜杩欐牱锛岃鎻愪氦 bug 鎶ュ憡銆?

"AnonHugePages"銆?ShmemPmdMapped" 鍜?"FilePmdMapped" 鏄剧ず浜嗗綋鍓嶇敱 PMD 绾у埆鐨勬灦鏋勬€уぇ椤垫槧灏勬墍鏀寔鐨勯€忔槑澶ч〉鐨勫唴瀛橀噺銆?AnonHugePages" 瀵瑰簲浜庝笉灞炰簬鏂囦欢鐨勫唴瀛橈紝"ShmemPmdMapped" 瀵瑰簲浜庡叡浜唴瀛橈紙shmem/tmpfs锛夛紝"FilePmdMapped" 瀵瑰簲浜庢枃浠舵敮鎸佺殑鍐呭瓨锛堜笉鍖呮嫭 shmem/tmpfs锛夈€?

瀵逛簬鏈 PMD 绾у埆鐨勬灦鏋勬€уぇ椤垫槧灏勬槧灏勭殑閫忔槑澶ч〉锛堟垨绫讳技姒傚康锛夛紝娌℃湁涓撻棬鐨勬潯鐩€?

"Shared_Hugetlb" 鍜?"Private_Hugetlb" 鏄剧ず浜嗙敱 hugetlbfs 椤垫敮鎸佺殑鍐呭瓨閲忥紝鐢变簬鍘嗗彶鍘熷洜锛岃繖閮ㄥ垎**涓?*璁″叆 "RSS" 鎴?"PSS" 瀛楁銆傚苟涓斿畠浠篃涓嶅寘鍚湪 {Shared,Private}_{Clean,Dirty} 瀛楁涓€?

"Swap" 鏄剧ず浜嗚浣跨敤浣嗕綅浜庝氦鎹㈢┖闂翠腑鐨勩€佸師鏈簲涓哄尶鍚嶇殑鍐呭瓨閲忋€?

瀵逛簬 shmem 鏄犲皠锛?Swap" 杩樺寘鎷簳灞?shmem 瀵硅薄涓凡鏄犲皠锛堜笖鏈鍐欐椂澶嶅埗鏇挎崲锛夊苟浣嶄簬浜ゆ崲绌洪棿涓殑閭ｉ儴鍒嗗ぇ灏忋€?SwapPss" 鏄剧ず璇ユ槧灏勭殑姣斾緥浜ゆ崲浠介銆備笌 "Swap" 涓嶅悓锛屽畠涓嶈鍏ュ簳灞?shmem 瀵硅薄鎹㈠嚭鐨勯〉銆?Locked" 鎸囩ず璇ユ槧灏勬槸鍚﹁閿佸畾鍦ㄥ唴瀛樹腑銆?

"THPeligible" 鎸囩ず璇ユ槧灏勬槸鍚︽湁璧勬牸鍒嗛厤浠讳綍褰撳墠宸插惎鐢ㄥぇ灏忕殑鑷劧瀵归綈 THP 椤点€備负鐪熷垯涓?1锛屽惁鍒欎负 0銆?

濡傛灉鍐呮牳鍜?CPU 閮芥敮鎸佷繚鎶ら敭锛坧keys锛夛紝"ProtectionKey" 鎸囩ず涓庤铏氭嫙鍐呭瓨鍖哄煙鍏宠仈鐨勫唴瀛樹繚鎶ら敭銆?

"VmFlags" 瀛楁鍊煎緱鍗曠嫭鎻忚堪銆傝鎴愬憳浠ュ弻瀛楁瘝缂栫爜鐨勬柟寮忚〃绀轰笌鐗瑰畾铏氭嫙鍐呭瓨鍖哄煙鍏宠仈鐨勫唴鏍告爣蹇椼€備唬鐮佸涓嬶細

    ==    =============================================================
    rd    鍙锛坮eadable锛?
    wr    鍙啓锛坵riteable锛?
    ex    鍙墽琛岋紙executable锛?
    sh    鍏变韩锛坰hared锛?
    mr    鍙鍙栵紙may read锛?
    mw    鍙啓鍏ワ紙may write锛?
    me    鍙墽琛岋紙may execute锛?
    ms    鍙叡浜紙may share锛?
    gd    鏍堟鍚戜笅澧為暱锛坰tack segment growns down锛?
    pf    绾?PFN 鑼冨洿锛坧ure PFN range锛?
    lo    椤佃閿佸畾鍦ㄥ唴瀛樹腑锛坧ages are locked in memory锛?
    io    鍐呭瓨鏄犲皠 I/O 鍖哄煙锛坢emory mapped I/O area锛?
    sr    鎻愪緵浜嗛『搴忚寤鸿锛坰equential read advise provided锛?
    rr    鎻愪緵浜嗛殢鏈鸿寤鸿锛坮andom read advise provided锛?
    dc    娲剧敓锛坒ork锛夋椂涓嶅鍒惰鍖哄煙锛坉o not copy area on fork锛?
    de    閲嶆槧灏勬椂涓嶆墿灞曡鍖哄煙锛坉o not expand area on remapping锛?
    ac    璇ュ尯鍩熷彲璁拌处锛坅rea is accountable锛?
    nr    鏈负璇ュ尯鍩熶繚鐣欎氦鎹㈢┖闂达紙swap space is not reserved for the area锛?
    ht    璇ュ尯鍩熶娇鐢ㄥぇ tlb 椤碉紙area uses huge tlb pages锛?
    sf    鍚屾椤甸敊璇紙synchronous page fault锛?
    ar    鏋舵瀯鐗瑰畾鏍囧織锛坅rchitecture specific flag锛?
    wf    娲剧敓鏃舵摝闄わ紙wipe on fork锛?
    dd    涓嶅寘鍚湪鏍稿績杞偍涓紙do not include area into core dump锛?
    sd    杞剰鏍囧織锛坰oft dirty flag锛?
    mm    娣峰悎鏄犲皠鍖哄煙锛坢ixed map area锛?
    hg    澶ч〉寤鸿鏍囧織锛坔uge page advise flag锛?
    nh    鏃犲ぇ椤靛缓璁爣蹇楋紙no huge page advise flag锛?
    mg    鍙悎骞跺缓璁爣蹇楋紙mergeable advise flag锛?
    bt    arm64 BTI 淇濇姢椤碉紙arm64 BTI guarded page锛?
    mt    鍚敤浜?arm64 MTE 鍒嗛厤鏍囩锛坅rm64 MTE allocation tags are enabled锛?
    um    userfaultfd 缂哄け璺熻釜锛坲serfaultfd missing tracking锛?
    uw    userfaultfd 鍐欎繚鎶よ窡韪紙userfaultfd wr-protect tracking锛?
    ui    userfaultfd 娆¤閿欒锛坲serfaultfd minor fault锛?
    ss    褰卞瓙/淇濇姢鎺у埗鏍堥〉锛坰hadow/guarded control stack page锛?
    sl    宸插皝瀛橈紙sealed锛?
    lf    鍑洪敊鏃堕攣瀹氶〉锛坙ock on fault pages锛?
    dp    濮嬬粓鍙儼鎬ч噴鏀剧殑鏄犲皠锛坅lways lazily freeable mapping锛?
    gu    鍙兘鍖呭惈淇濇姢鍖哄煙锛堣嫢鏈缃紝鍒欒偗瀹氫笉鍖呭惈锛?
    ==    =============================================================

娉ㄦ剰锛屼笉鑳戒繚璇佹瘡涓爣蹇楀拰鍏宠仈鍔╄绗﹀湪鎵€鏈夊悗缁唴鏍哥増鏈腑閮藉瓨鍦ㄣ€備簨鎯呬細鍙戠敓鍙樺寲锛屾爣蹇楀彲鑳戒細娑堝け锛屾垨鑰呯浉鍙嶁€斺€旀柊澧炪€傚畠浠惈涔夌殑瑙ｉ噴鍦ㄦ湭鏉ヤ篃鍙兘鏀瑰彉銆傚洜姝よ繖浜涙爣蹇楃殑姣忎釜浣跨敤鑰呴兘蹇呴』閽堝姣忎釜鐗瑰畾鐨勫唴鏍哥増鏈潵璺熻釜鍏剁‘鍒囪涔夈€?

鍙湁褰撳惎鐢ㄤ簡 CONFIG_MMU 鍐呮牳閰嶇疆閫夐」鏃讹紝姝ゆ枃浠舵墠瀛樺湪銆?

娉ㄦ剰锛氳鍙?/proc/PID/maps 鎴?/proc/PID/smaps 鏈川涓婃槸瀛樺湪绔炴€佺殑锛堝彧鏈夊湪鍗曟璇诲彇璋冪敤涓墠鑳借幏寰椾竴鑷寸殑杈撳嚭锛夈€?

杩欓€氬父鍦ㄨ繘琛岃繖浜涙枃浠剁殑閮ㄥ垎璇诲彇銆佸悓鏃跺唴瀛樻槧灏勬鍦ㄨ淇敼鏃惰〃鐜板嚭鏉ャ€傚敖绠″瓨鍦ㄧ珵鎬侊紝鎴戜滑浠嶆彁渚涗互涓嬩繚璇侊細

1) 鏄犲皠鐨勫湴鍧€姘歌繙涓嶄細鍚庨€€锛岃繖鎰忓懗鐫€浠绘剰涓や釜鍖哄煙姘歌繙涓嶄細閲嶅彔銆?
2) 濡傛灉鍦?smaps/maps 閬嶅巻鐨勬暣涓敓鍛藉懆鏈熷唴鏌愪釜缁欏畾 vaddr 涓婂缁堟湁鍐呭锛屽垯浼氭湁瀵瑰簲鐨勮緭鍑恒€?

/proc/PID/smaps_rollup 鏂囦欢鍖呭惈涓?/proc/PID/smaps 鐩稿悓鐨勫瓧娈碉紝浣嗗畠浠殑鍊兼槸璇ヨ繘绋嬫墍鏈夋槧灏勫搴斿€肩殑鎬诲拰銆傛澶栵紝瀹冭繕鍖呭惈浠ヤ笅瀛楁锛?

- Pss_Anon
- Pss_File
- Pss_Shmem

瀹冧滑琛ㄧず濡備笂涓?smaps 鎵€鎻忚堪鐨勫尶鍚嶃€佹枃浠跺拰 shmem 椤电殑姣斾緥浠介銆傝繖浜涘瓧娈靛湪 smaps 涓鐪佺暐锛屽洜涓烘瘡涓槧灏勯兘鏍囪瘑浜嗗畠鎵€鍖呭惈鐨勬墍鏈夐〉鐨勭被鍨嬶紙anon銆乫ile 鎴?shmem锛夈€傚洜姝?smaps_rollup 涓殑鎵€鏈変俊鎭兘鍙互浠?smaps 鎺ㄥ鍑烘潵锛屼絾浠ｄ环瑕侀珮寰楀銆?

/proc/PID/clear_refs 鐢ㄤ簬閲嶇疆涓庤繘绋嬪叧鑱旂殑鐗╃悊鍜岃櫄鎷熼〉涓婄殑 PG_Referenced 鍜?ACCESSED/YOUNG 浣嶏紝浠ュ強 pte 涓婄殑杞剰浣嶏紙璇﹁ Documentation/admin-guide/mm/soft-dirty.rst锛夈€?

```
    > echo 1 > /proc/PID/clear_refs
```

```
    > echo 2 > /proc/PID/clear_refs
```

```
    > echo 3 > /proc/PID/clear_refs
```

```
    > echo 4 > /proc/PID/clear_refs
```

瑕侀噸缃嘲鍊煎父椹婚泦澶у皬锛?楂樻按浣嶆爣璁?锛変负杩涚▼鐨勶細

```
    > echo 5 > /proc/PID/clear_refs
```

鍐欏叆 /proc/PID/clear_refs 鐨勪换浣曞叾浠栧€奸兘涓嶄細浜х敓鏁堟灉銆?

/proc/pid/pagemap 缁欏嚭 PFN锛屽彲鐢ㄤ簬閫氳繃 /proc/kpageflags 鏌ユ壘 pageflags锛屼互鍙婇€氳繃 /proc/kpagecount 鏌ユ壘涓€涓〉琚槧灏勭殑娆℃暟銆傝缁嗚В閲婅 Documentation/admin-guide/mm/pagemap.rst銆?

/proc/pid/numa_maps 鏄熀浜?maps 鐨勬墿灞曪紝鏄剧ず鍐呭瓨灞€閮ㄦ€у拰缁戝畾绛栫暐锛屼互鍙婃瘡涓槧灏勭殑鍐呭瓨浣跨敤閲忥紙浠ラ〉涓哄崟浣嶏級銆傝緭鍑洪伒寰€氱敤鏍煎紡锛屽叾涓槧灏勭粏鑺傜敱

```
    address   policy    mapping details

    00400000 default file=/usr/local/bin/app mapped=1 active=0 N3=1 kernelpagesize_kB=4
    00600000 default file=/usr/local/bin/app anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206000000 default file=/lib64/ld-2.12.so mapped=26 mapmax=6 N0=24 N3=2 kernelpagesize_kB=4
    320621f000 default file=/lib64/ld-2.12.so anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206220000 default file=/lib64/ld-2.12.so anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206221000 default anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206800000 default file=/lib64/libc-2.12.so mapped=59 mapmax=21 active=55 N0=41 N3=18 kernelpagesize_kB=4
    320698b000 default file=/lib64/libc-2.12.so
    3206b8a000 default file=/lib64/libc-2.12.so anon=2 dirty=2 N3=2 kernelpagesize_kB=4
    3206b8e000 default file=/lib64/libc-2.12.so anon=1 dirty=1 N3=1 kernelpagesize_kB=4
    3206b8f000 default anon=3 dirty=3 active=1 N3=3 kernelpagesize_kB=4
    7f4dc10a2000 default anon=3 dirty=3 N3=3 kernelpagesize_kB=4
    7f4dc10b4000 default anon=2 dirty=2 active=1 N3=2 kernelpagesize_kB=4
    7f4dc1200000 default file=/anon_hugepage\040(deleted) huge anon=1 dirty=1 N3=1 kernelpagesize_kB=2048
    7fff335f0000 default stack anon=3 dirty=3 N3=3 kernelpagesize_kB=4
    7fff3369d000 default mapped=1 mapmax=35 active=0 N3=1 kernelpagesize_kB=4
```

鍏朵腑锛?

"address" 鏄鏄犲皠鐨勮捣濮嬪湴鍧€锛?

"policy" 鎶ュ憡涓鸿鏄犲皠璁剧疆鐨?NUMA 鍐呭瓨绛栫暐锛堣 Documentation/admin-guide/mm/numa_memory_policy.rst锛夛紱

"mapping details" 姹囨€讳簡鏄犲皠鏁版嵁锛屽鏄犲皠绫诲瀷銆侀〉浣跨敤璁℃暟鍣ㄣ€佽妭鐐瑰眬閮ㄦ€ч〉璁℃暟鍣紙N0 == node0锛孨1 == node1锛屸€︹€︼級浠ュ強鏀寔璇ユ槧灏勭殑鍐呮牳椤靛ぇ灏忥紙浠?KB 涓哄崟浣嶏級銆?

娉ㄦ剰锛屾煇浜涘唴鏍搁厤缃笉璺熻釜杈冨ぇ鍒嗛厤锛堜緥濡?THP锛変腑涓€閮ㄥ垎鐨勯〉琚槧灏勭殑绮剧‘娆℃暟銆傚湪杩欎簺閰嶇疆涓紝"mapmax" 鍙兘瀵瑰簲浜庢绫昏緝澶у垎閰嶄腑姣忛〉鐨勫钩鍧囨槧灏勬暟銆?
### 1.2 鍐呮牳鏁版嵁


涓庤繘绋嬫潯鐩被浼硷紝鍐呮牳鏁版嵁鏂囦欢鎻愪緵鍏充簬杩愯涓殑鍐呮牳鐨勪俊鎭€傜敤浜庤幏鍙栬繖浜涗俊鎭殑鏂囦欢鍖呭惈鍦?/proc 涓紝骞跺垪浜庤〃 1-5銆傚苟闈炴墍鏈夎繖浜涙枃浠堕兘浼氬嚭鐜板湪浣犵殑绯荤粺涓€傝繖鍙栧喅浜庡唴鏍搁厤缃拰宸插姞杞界殑妯″潡锛屽摢浜涙枃浠跺瓨鍦紝鍝簺缂哄け銆?


 ============ ===============================================================
 鏂囦欢        鍐呭
 ============ ===============================================================
 allocinfo    鍐呭瓨鍒嗛厤鎬ц兘鍒嗘瀽淇℃伅
 apm          楂樼骇鐢垫簮绠＄悊锛圓dvanced power management锛変俊鎭?
 bootconfig   浠?boot config 鑾峰彇鐨勫唴鏍稿懡浠よ锛?
 	      浠ュ強锛屽鏋滄湁鏉ヨ嚜寮曞鍔犺浇绋嬪簭鐨?
 	      鍐呮牳鍙傛暟锛屽垯鏈変竴琛?"# Parameters from bootloader:"
 	      鍚庤窡鍖呭惈杩欎簺鍙傛暟鐨勮锛屽墠闈㈠姞 "# "銆?(5.5)
 buddyinfo    鍐呮牳鍐呭瓨鍒嗛厤鍣ㄤ俊鎭紙瑙佹鏂囷級			(2.5)
 bus          鍖呭惈鎬荤嚎鐗瑰畾淇℃伅鐨勭洰褰?
 cmdline      鍐呮牳鍛戒护琛岋紝鍖呮嫭鏉ヨ嚜寮曞鍔犺浇绋嬪簭鍜屽祵鍏ュ湪鍐呮牳鏄犲儚涓殑
 cpuinfo      鍏充簬 CPU 鐨勪俊鎭?
 devices      鍙敤璁惧锛堝潡璁惧鍜屽瓧绗﹁澶囷級
 dma          宸蹭娇鐢ㄧ殑 DMA 閫氶亾
 filesystems  鏀寔鐨勬枃浠剁郴缁?
 driver       鍦ㄦ鍒嗙粍鐨勪笉鍚岄┍鍔紝鐩墠涓?rtc			(2.4)
 execdomains  鎵ц鍩燂紙Execdomains锛夛紝涓庡畨鍏ㄧ浉鍏?		(2.4)
 fb 	      甯х紦鍐诧紙Frame Buffer锛夎澶?		(2.4)
 fs 	      鏂囦欢绯荤粺鍙傛暟锛岀洰鍓嶄负 nfs/exports		(2.4)
 ide          鍖呭惈鍏充簬 IDE 瀛愮郴缁熶俊鎭殑鐩綍
 interrupts   涓柇浣跨敤鎯呭喌
 iomem 	      鍐呭瓨鏄犲皠锛圡emory map锛?			(2.4)
 ioports      I/O 绔彛浣跨敤鎯呭喌
 irq 	      irq 鍒?cpu 浜插拰鎬х殑鎺╃爜				(2.4)(smp锛?
 isapnp       ISA PnP (Plug&Play) 淇℃伅				(2.4)
 kcore        鍐呮牳鏍稿績鏄犲儚锛堝彲浠ユ槸 ELF 鎴?A.OUT锛堝湪 2.4 涓凡搴熷純锛夛級
 kmsg         鍐呮牳娑堟伅
 ksyms        鍐呮牳绗﹀彿琛?
 loadavg      杩囧幓 1銆? 鍜?15 鍒嗛挓鐨勫钩鍧囪礋杞斤紱
                褰撳墠鍙繍琛岃繘绋嬫暟锛堣繍琛屾垨鍦ㄥ氨缁槦鍒椾腑锛夛紱
                绯荤粺涓繘绋嬫€绘暟锛?
                鏈€鍚庡垱寤虹殑 pid銆?
                闄?褰撳墠鍙繍琛岃繘绋嬫暟"鍜?绯荤粺涓繘绋嬫€绘暟"澶栵紝
                鎵€鏈夊瓧娈甸兘鐢ㄧ┖鏍煎垎闅旓紝杩欎袱鑰呬箣闂寸敤鏂滄潬锛?/'锛夊垎闅斻€傜ず渚嬶細
                0.61 0.61 0.55 3/828 22084
 locks        鍐呮牳閿?
 meminfo      鍐呭瓨淇℃伅
 misc         鏉傞」
 modules      宸插姞杞芥ā鍧楀垪琛?
 mounts       宸叉寕杞界殑鏂囦欢绯荤粺
 net          缃戠粶淇℃伅锛堣姝ｆ枃锛?
 pagetypeinfo 棰濆鐨勯〉鍒嗛厤鍣ㄤ俊鎭紙瑙佹鏂囷級			(2.5)
 partitions   绯荤粺宸茬煡鐨?partitions 琛?
 pci 	      PCI 鎬荤嚎鐨勫凡搴熷純淇℃伅锛堟柊鏂瑰紡 -> /proc/bus/pci/锛?
                鐢?lspci 瑙ｈ€?			(2.4)
 rtc          瀹炴椂鏃堕挓锛圧eal time clock锛?
 scsi         SCSI 淇℃伅锛堣姝ｆ枃锛?
 slabinfo     Slab 姹犱俊鎭?
 softirqs     softirq 浣跨敤鎯呭喌
 stat         鎬讳綋缁熻
 swaps        浜ゆ崲绌洪棿鍒╃敤鐜?
 sys          瑙佺 2 绔?
 sysvipc      SysVIPC 璧勬簮锛坢sg銆乻em銆乻hm锛夌殑淇℃伅			(2.4)
 tty 	      tty 椹卞姩鐨勪俊鎭?
 uptime       鑷惎鍔ㄤ互鏉ョ殑澧欎笂鏃堕挓鏃堕棿锛屼互鍙婃墍鏈?cpu 鐨勫悎骞剁┖闂叉椂闂?
 version      鍐呮牳鐗堟湰
 video 	      video 璧勬簮鐨?bttv 淇℃伅				(2.4)
 vmallocinfo  鏄剧ず vmalloced 鍖哄煙
 ============ ===============================================================

渚嬪锛屼綘鍙互妫€鏌ュ綋鍓嶆鍦ㄤ娇鐢ㄧ殑涓柇浠ュ強鍝簺

```
  > cat /proc/interrupts
             CPU0
    0:    8728810          XT-PIC  timer
    1:        895          XT-PIC  keyboard
    2:          0          XT-PIC  cascade
    3:     531695          XT-PIC  aha152x
    4:    2014133          XT-PIC  serial
    5:      44401          XT-PIC  pcnet_cs
    8:          2          XT-PIC  rtc
   11:          8          XT-PIC  i82365
   12:     182918          XT-PIC  PS/2 Mouse
   13:          1          XT-PIC  fpu
   14:    1232265          XT-PIC  ide0
   15:          7          XT-PIC  ide1
  NMI:          0
```

鍦?2.4.* 涓紝鍚戣鏂囦欢娣诲姞浜?couple 琛?LOC & ERR锛堣繖娆℃槸

```
  > cat /proc/interrupts

             CPU0       CPU1
    0:    1243498    1214548    IO-APIC-edge  timer
    1:       8949       8958    IO-APIC-edge  keyboard
    2:          0          0          XT-PIC  cascade
    5:      11286      10161    IO-APIC-edge  soundblaster
    8:          1          0    IO-APIC-edge  rtc
    9:      27422      27407    IO-APIC-edge  3c503
   12:     113645     113873    IO-APIC-edge  PS/2 Mouse
   13:          0          0          XT-PIC  fpu
   14:      22491      24012    IO-APIC-edge  ide0
   15:       2183       2415    IO-APIC-edge  ide1
   17:      30564      30414   IO-APIC-level  eth0
   18:        177        164   IO-APIC-level  bttv
  NMI:    2457961    2457959
  LOC:    2457882    2457881
  ERR:       2155
```

鍦ㄨ繖绉嶆儏鍐典笅 NMI 澧炲姞锛屽洜涓烘瘡娆″畾鏃跺櫒涓柇閮戒細鐢熸垚涓€涓?NMI锛堜笉鍙睆钄戒腑鏂級锛孨MI 鐪嬮棬鐙楃敤瀹冩潵妫€娴嬫閿併€?

LOC 鏄瘡涓?CPU 鍐呴儴 APIC 鐨勬湰鍦颁腑鏂鏁板櫒銆?

ERR 鍦?IO-APIC 鎬荤嚎锛堝湪 SMP 绯荤粺涓繛鎺?CPU 鐨勬€荤嚎锛夊嚭鐜伴敊璇椂澧炲姞銆傝繖鎰忓懗鐫€妫€娴嬪埌浜嗕竴涓敊璇紝IO-APIC 浼氳嚜鍔ㄩ噸璇曚紶杈擄紝鍥犳杩欏簲璇ヤ笉鏄ぇ闂锛屼絾浣犲簲璇ラ槄璇?SMP-FAQ銆?

鍦?2.6.2* 涓紝/proc/interrupts 鍐嶆琚墿灞曘€傝繖娆＄殑鐩爣鏄 /proc/interrupts 鏄剧ず绯荤粺涓娇鐢ㄧ殑姣忎釜 IRQ 鍚戦噺锛岃€屼笉浠呬粎鏄偅浜涜璁や负鏄?鏈€閲嶈"鐨勩€傛柊鐨勫悜閲忔湁锛?

THR
  褰撴満鍣ㄦ鏌ラ槇鍊艰鏁板櫒锛堥€氬父璁℃暟鍐呭瓨鎴栫紦瀛樼殑 ECC 绾犳閿欒锛夎秴杩囧彲閰嶇疆闃堝€兼椂寮曞彂鐨勪腑鏂€備粎鍦ㄦ煇浜涚郴缁熶笂鍙敤銆?

TRM
  褰?CPU 鐨勬俯搴﹂槇鍊艰瓒呰繃鏃跺彂鐢熺儹浜嬩欢涓柇銆傚綋娓╁害闄嶅洖姝ｅ父鏃朵篃鍙兘鐢熸垚姝や腑鏂€?

SPU
  浼腑鏂紙spurious interrupt锛夋槸鏌愪釜 IO 璁惧鍦ㄨ兘琚?APIC 瀹屽叏澶勭悊涔嬪墠琚紩鍙戝張鎷変綆鐨勪腑鏂€傚洜姝?APIC 鐪嬪埌浜嗕腑鏂紝浣嗕笉鐭ラ亾瀹冩潵鑷摢涓澶囥€傚浜庤繖绉嶆儏鍐碉紝APIC 灏嗙敓鎴?IRQ 鍚戦噺涓?0xff 鐨勪腑鏂€傝繖涔熷彲鑳界敱鑺墖缁?bug 寮曡捣銆?

RES銆丆AL銆乀LB
  閲嶆柊璋冨害銆佽皟鐢ㄥ拰 TLB 鍒锋柊涓柇鏄牴鎹搷浣滅郴缁熺殑闇€瑕佷粠涓€涓?CPU 鍙戦€佸埌鍙︿竴涓?CPU 鐨勩€傞€氬父锛屽畠浠殑缁熻淇℃伅琚唴鏍稿紑鍙戣€呭拰鎰熷叴瓒ｇ殑鐢ㄦ埛鐢ㄦ潵纭畾缁欏畾绫诲瀷涓柇鐨勫彂鐢熸儏鍐点€?

涓婅堪 IRQ 鍚戦噺浠呭湪鐩稿叧鏃舵樉绀恒€備緥濡傦紝闃堝€煎悜閲忓湪 x86_64 骞冲彴涓婁笉瀛樺湪銆傚綋绯荤粺涓哄崟澶勭悊鍣ㄦ椂锛屽叾浠栧悜閲忎細琚姂鍒躲€傛埅鑷虫湰鏂囨挵鍐欐椂锛屽彧鏈?i386 鍜?x86_64 骞冲彴鏀寔鏂扮殑 IRQ 鍚戦噺鏄剧ず銆?

鍊煎緱鍏虫敞鐨勪竴鐐规槸 2.4 涓紩鍏ヤ簡 /proc/irq 鐩綍銆傚畠鍙互鐢ㄦ潵璁剧疆 IRQ 鍒?CPU 鐨勪翰鍜屾€с€傝繖鎰忓懗鐫€浣犲彲浠ュ皢 IRQ"鎸傞挬"鍒颁粎涓€涓?CPU锛屾垨鎺掗櫎鏌愪釜 CPU 澶勭悊 IRQ銆俰rq 瀛愮洰褰曠殑鍐呭鏄瘡涓?IRQ 鐨勪竴涓瓙鐩綍锛屼互鍙?default_smp_affinity銆?

```
  > ls /proc/irq/
  0  10  12  14  16  18  2  4  6  8  default_smp_affinity
  1  11  13  15  17  19  3  5  7  9
  > ls /proc/irq/0/
  smp_affinity
```

smp_affinity 鏄竴涓綅鎺╃爜锛屽彲浠ュ湪鍏朵腑鎸囧畾鍝簺 CPU 鍙互澶勭悊

```
  > echo 1 > /proc/irq/10/smp_affinity
```

杩欐剰鍛崇潃鍙湁绗竴涓?CPU 浼氬鐞嗚 IRQ锛屼絾浣犱篃鍙互 echo 5锛岃繖鎰忓懗鐫€鍙湁绗竴涓拰绗笁涓?CPU 鍙互澶勭悊璇?IRQ銆?

```
  > cat /proc/irq/0/smp_affinity
  ffffffff
```

杩樻湁涓€涓浛浠ｆ帴鍙?smp_affinity_list锛屽厑璁告寚瀹?

```
  > cat /proc/irq/0/smp_affinity_list
  1024-1031
```

default_smp_affinity 鎺╃爜閫傜敤浜庢墍鏈夐潪娲诲姩 IRQ锛屽嵆灏氭湭琚垎閰?婵€娲汇€佸洜姝ょ己灏?/proc/irq/[0-9]* 鐩綍鐨?IRQ銆?

SMP 绯荤粺涓婄殑 node 鏂囦欢鏄剧ず浣跨敤 IRQ 鐨勮澶囨墍鎶ュ憡鐨勩€佸叾鑷韩鎵€闄勫姞鍒扮殑鑺傜偣銆傝纭欢灞€閮ㄦ€т俊鎭笉鍖呮嫭浠讳綍鍙兘鐨勯┍鍔ㄥ眬閮ㄦ€у亸濂界殑淇℃伅銆?

IRQ 鐨勮矾鐢辨柟寮忕敱 IO-APIC 澶勭悊锛屽苟涓斿湪鎵€鏈夎鍏佽澶勭悊瀹冪殑 CPU 涔嬮棿閲囩敤杞锛圧ound Robin锛夈€傚儚寰€甯镐竴鏍凤紝鍐呮牳鎷ユ湁姣斾綘鏇村鐨勪俊鎭紝骞朵笖鍋氬緱姣斾綘濂斤紝鍥犳榛樿鍊煎鍑犱箮鎵€鏈変汉鏉ヨ閮芥槸鏈€浣抽€夋嫨銆俒娉ㄦ剰杩欎粎閫傜敤浜庨偅浜涙敮鎸?Round Robin"涓柇鍒嗗竷鐨?IO-APIC銆俔

/proc 涓繕鏈変笁涓洿閲嶈鐨勫瓙鐩綍锛歯et銆乻csi 鍜?sys銆備竴鑸殑瑙勫垯鏄紝杩欎簺鐩綍鐨勫唴瀹癸紝鐢氳嚦瀹冧滑鐨勫瓨鍦紝閮藉彇鍐充簬浣犵殑鍐呮牳閰嶇疆銆傚鏋滄湭鍚敤 SCSI锛屽垯 scsi 鐩綍鍙兘涓嶅瓨鍦ㄣ€俷et 涔熸槸涓€鏍凤紝瀹冨彧鏈夊湪杩愯涓殑鍐呮牳瀛樺湪缃戠粶鏀寔鏃舵墠瀛樺湪銆?

slabinfo 鏂囦欢鎻愪緵 slab 绾у埆鐨勫唴瀛樹娇鐢ㄤ俊鎭€侺inux 鍦?2.2 鐗堟湰涓娇鐢?slab 姹犺繘琛岄〉绾у埆浠ヤ笂鐨勫唴瀛樼鐞嗐€傚父鐢ㄥ璞℃嫢鏈夎嚜宸辩殑 slab 姹狅紙濡傜綉缁滅紦鍐插尯銆佺洰褰曠紦瀛樼瓑锛夈€?

```
    > cat /proc/buddyinfo

    Node 0, zone      DMA      0      4      5      4      4      3 ...
    Node 0, zone   Normal      1      0      0      1    101      8 ...
    Node 0, zone  HighMem      2      0      0      1      1      0 ...
```

澶栭儴纰庣墖鍦ㄦ煇浜涘伐浣滆礋杞戒笅鏄釜闂锛宐uddyinfo 鏄府鍔╄瘖鏂繖浜涢棶棰樼殑涓€涓湁鐢ㄥ伐鍏枫€侭uddyinfo 浼氱粰浣犱竴涓嚎绱紝鍛婅瘔浣犺兘澶熷畨鍏ㄥ垎閰嶅澶х殑鍖哄煙锛屾垨鑰呬负浠€涔堜箣鍓嶇殑鍒嗛厤浼氬け璐ャ€?

姣忎竴鍒楄〃绀哄彲鐢ㄧ殑鏌愪釜闃讹紙order锛夌殑椤垫暟閲忋€傚湪杩欑鎯呭喌涓嬶紝ZONE_DMA 涓湁 0 涓?2^0*PAGE_SIZE 鐨勫潡锛孼ONE_DMA 涓湁 4 涓?2^1*PAGE_SIZE 鐨勫潡锛孼ONE_NORMAL 涓湁 101 涓?2^4*PAGE_SIZE 鐨勫潡锛岀瓑绛夆€︹€?

鍏充簬澶栭儴纰庣墖鐨勬洿澶氫俊鎭彲浠ュ湪浠ヤ笅鎵惧埌锛?

```
    > cat /proc/pagetypeinfo
    Page block order: 9
    Pages per block:  512

    Free pages count per migrate type at order       0      1      2      3      4      5      6      7      8      9     10
    Node    0, zone      DMA, type    Unmovable      0      0      0      1      1      1      1      1      1      1      0
    Node    0, zone      DMA, type  Reclaimable      0      0      0      0      0      0      0      0      0      0      0
    Node    0, zone      DMA, type      Movable      1      1      2      1      2      1      1      0      1      0      2
    Node    0, zone      DMA, type      Reserve      0      0      0      0      0      0      0      0      0      1      0
    Node    0, zone      DMA, type      Isolate      0      0      0      0      0      0      0      0      0      0      0
    Node    0, zone    DMA32, type    Unmovable    103     54     77      1      1      1     11      8      7      1      9
    Node    0, zone    DMA32, type  Reclaimable      0      0      2      1      0      0      0      0      1      0      0
    Node    0, zone    DMA32, type      Movable    169    152    113     91     77     54     39     13      6      1    452
    Node    0, zone    DMA32, type      Reserve      1      2      2      2      2      0      1      1      1      1      0
    Node    0, zone    DMA32, type      Isolate      0      0      0      0      0      0      0      0      0      0      0

    Number of blocks type     Unmovable  Reclaimable      Movable      Reserve      Isolate
    Node 0, zone      DMA            2            0            5            1            0
    Node 0, zone    DMA32           41            6          967            2            0
```

鍐呮牳涓殑纰庣墖閬垮厤閫氳繃灏嗕笉鍚岃縼绉荤被鍨嬬殑椤靛垎缁勫埌绉颁负椤靛潡锛坧age block锛夌殑鐩稿悓杩炵画鍐呭瓨鍖哄煙鏉ュ伐浣溿€傞〉鍧楅€氬父鏄粯璁ゅぇ椤靛ぇ灏忥紝渚嬪 X86-64 涓婁负 2MB銆傞€氳繃鏍规嵁椤电殑鍙Щ鍔ㄦ€у鍏惰繘琛屽垎缁勶紝鍐呮牳鍙互鍥炴敹椤靛潡鍐呯殑椤典互婊¤冻楂橀樁鍒嗛厤銆?

pagetypinfo 浠ュ叧浜庨〉鍧楀ぇ灏忕殑淇℃伅寮€澶淬€傜劧鍚庡畠缁欏嚭涓?buddyinfo 鐩稿悓绫诲瀷鐨勪俊鎭紝鍙槸鎸夎縼绉荤被鍨嬬粏鍒嗭紝骞朵互姣忕绫诲瀷鏈夊灏戜釜椤靛潡鐨勮缁嗕俊鎭粨鏉熴€?

濡傛灉 min_free_kbytes 宸茶姝ｇ‘璋冩暣锛堢敱鏉ヨ嚜 libhugetlbfs 鐨?hugeadm 鎻愬嚭寤鸿 https://github.com/libhugetlbfs/libhugetlbfs/锛夛紝鍒欏彲浠ヤ及璁″湪缁欏畾鏃跺埢鍙互鍒嗛厤鐨勫ぇ椤电殑鍙兘鏁伴噺銆傞櫎闈炲唴瀛樺凡琚?mlock() 閿佸畾锛屽惁鍒欐墍鏈?Movable"鍧楅兘搴旇鏄彲鍒嗛厤鐨勩€備竴浜?Reclaimable 鍧椾篃搴旇鏄彲鍒嗛厤鐨勶紝灏界涓烘鍙兘蹇呴』鍥炴敹澶ч噺鏂囦欢绯荤粺鍏冩暟鎹€?

#### allocinfo


鎻愪緵鍏充簬浠ｇ爜搴撲腑鎵€鏈変綅缃殑鍐呭瓨鍒嗛厤鐨勪俊鎭€備唬鐮佷腑鐨勬瘡涓垎閰嶇敱鍏舵簮鏂囦欢銆佽鍙枫€佹ā鍧楋紙濡傛灉鏉ヨ嚜鍙姞杞芥ā鍧楋級浠ュ強璋冪敤璇ュ垎閰嶇殑鍑芥暟鏍囪瘑銆備細鎶ュ憡姣忎釜浣嶇疆鍒嗛厤鐨勫瓧鑺傛暟鍜岃皟鐢ㄦ鏁般€傜涓€琛屾寚绀烘枃浠剁殑鐗堟湰锛岀浜岃鏄垪鍑烘枃浠朵腑鍚勫瓧娈电殑琛ㄥご銆?
濡傛灉鏂囦欢鐗堟湰涓?2.0 鎴栨洿楂橈紝鍒欐瘡琛屽彲鑳藉寘鍚澶栫殑 <key>:<value> 瀵癸紝琛ㄧず鍏充簬璋冪敤鐐圭殑棰濆淇℃伅銆備緥濡傦紝濡傛灉璁℃暟鍣ㄤ笉鍑嗙‘锛岃琛屼細琚檮鍔?"accurate:no" 瀵广€?

v2 涓敮鎸佺殑鏍囪锛?
accurate:no

              鐢变簬鏈兘鍒嗛厤鍐呭瓨鏉ヨ窡韪湪姝や綅缃繘琛岀殑閮ㄥ垎鍒嗛厤锛屾湰琛屼腑璁℃暟鍣ㄧ殑缁濆鍊间笉鍑嗙‘銆傝繖浜涜鏁板櫒鐨勫閲忔槸鍑嗙‘鐨勶紝鍥犳璁℃暟鍣ㄥ彲鐢ㄤ簬璺熻釜鍒嗛厤澶у皬鍜岃鏁板彉鍖栥€?

绀轰緥杈撳嚭銆?

```

    > tail -n +3 /proc/allocinfo | sort -rn
   127664128    31168 mm/page_ext.c:270 func:alloc_page_ext
    56373248     4737 mm/slub.c:2259 func:alloc_slab_page
    14880768     3633 mm/readahead.c:247 func:page_cache_ra_unbounded
    14417920     3520 mm/mm_init.c:2530 func:alloc_large_system_hash
    13377536      234 block/blk-mq.c:3421 func:blk_mq_alloc_rqs
    11718656     2861 mm/filemap.c:1919 func:__filemap_get_folio
     9192960     2800 kernel/fork.c:307 func:alloc_thread_stack_node
     4206592        4 net/netfilter/nf_conntrack_core.c:2567 func:nf_ct_alloc_hashtable
     4136960     1010 drivers/staging/ctagmod/ctagmod.c:20 [ctagmod] func:ctagmod_start
     3940352      962 mm/memory.c:4214 func:alloc_anon_folio
     2894464    22613 fs/kernfs/dir.c:615 func:__kernfs_new_node
     ...

```

#### meminfo


鎻愪緵鍏充簬鍐呭瓨鍒嗗竷鍜屽埄鐢ㄧ巼鐨勪俊鎭€傝繖鍥犳灦鏋勫拰缂栬瘧閫夐」鑰屽紓銆傝繖閲屾姤鍛婄殑涓€浜涜鏁板櫒鏈夐噸鍙犮€傜敱闈為噸鍙犺鏁板櫒鎶ュ憡鐨勫唴瀛樺彲鑳戒笉绛変簬鏁翠綋鍐呭瓨浣跨敤閲忥紝瀵逛簬鏌愪簺宸ヤ綔璐熻浇锛屽樊寮傚彲鑳藉緢澶с€傚湪璁稿鎯呭喌涓嬶紝鏈夊叾浠栨柟娉曞彲浠ラ€氳繃鐗瑰畾瀛愮郴缁熺殑鎺ュ彛鎵惧埌棰濆鐨勫唴瀛橈紝渚嬪鐢ㄤ簬 TCP 鍐呭瓨鍒嗛厤鐨?/proc/net/sockstat銆?

绀轰緥杈撳嚭銆備綘鍙兘娌℃湁鎵€鏈夎繖浜涘瓧娈点€?

```
    > cat /proc/meminfo

    MemTotal:       32858820 kB
    MemFree:        21001236 kB
    MemAvailable:   27214312 kB
    Buffers:          581092 kB
    Cached:          5587612 kB
    SwapCached:            0 kB
    Active:          3237152 kB
    Inactive:        7586256 kB
    Active(anon):      94064 kB
    Inactive(anon):  4570616 kB
    Active(file):    3143088 kB
    Inactive(file):  3015640 kB
    Unevictable:           0 kB
    Mlocked:               0 kB
    SwapTotal:             0 kB
    SwapFree:              0 kB
    Zswap:              1904 kB
    Zswapped:           7792 kB
    Dirty:                12 kB
    Writeback:             0 kB
    AnonPages:       4654780 kB
    Mapped:           266244 kB
    Shmem:              9976 kB
    KReclaimable:     517708 kB
    Slab:             660044 kB
    SReclaimable:     517708 kB
    SUnreclaim:       142336 kB
    KernelStack:       11168 kB
    PageTables:        20540 kB
    SecPageTables:         0 kB
    NFS_Unstable:          0 kB
    Bounce:                0 kB
    WritebackTmp:          0 kB
    CommitLimit:    16429408 kB
    Committed_AS:    7715148 kB
    VmallocTotal:   34359738367 kB
    VmallocUsed:       40444 kB
    VmallocChunk:          0 kB
    Percpu:            29312 kB
    EarlyMemtestBad:       0 kB
    HardwareCorrupted:     0 kB
    AnonHugePages:   4149248 kB
    ShmemHugePages:        0 kB
    ShmemPmdMapped:        0 kB
    FileHugePages:         0 kB
    FilePmdMapped:         0 kB
    CmaTotal:              0 kB
    CmaFree:               0 kB
    Unaccepted:            0 kB
    Balloon:               0 kB
    GPUActive:             0 kB
    GPUReclaim:            0 kB
    HugePages_Total:       0
    HugePages_Free:        0
    HugePages_Rsvd:        0
    HugePages_Surp:        0
    Hugepagesize:       2048 kB
    Hugetlb:               0 kB
    DirectMap4k:      401152 kB
    DirectMap2M:    10008576 kB
    DirectMap1G:    24117248 kB
```

MemTotal
              鎬诲彲鐢?RAM锛堝嵆鐗╃悊 RAM 鍑忓幓灏戦噺淇濈暀浣嶅拰鍐呮牳浜岃繘鍒朵唬鐮侊級
MemFree
              鎬荤┖闂?RAM銆傚湪 highmem 绯荤粺涓婏紝涓?LowFree+HighFree 涔嬪拰
MemAvailable
              鍦ㄤ笉浜ゆ崲鐨勬儏鍐典笅锛屽彲鐢ㄤ簬鍚姩鏂板簲鐢ㄧ▼搴忕殑鍐呭瓨閲忎及璁°€傛牴鎹?MemFree銆?
              SReclaimable銆佹枃浠?LRU 鍒楄〃鐨勫ぇ灏忥紝浠ュ強姣忎釜 zone 鐨勪綆
              姘翠綅绾胯绠椼€?
              璇ヤ及璁¤€冭檻浜嗙郴缁熼渶瑕佷竴浜涢〉缂撳瓨鎵嶈兘鑹ソ杩愯锛屽苟涓旂敱浜庢湁椤圭洰
              姝ｅ湪浣跨敤锛屽苟闈炴墍鏈夊彲鍥炴敹鐨?slab 閮藉彲鍥炴敹銆傝繖浜涘洜绱犵殑
              褰卞搷浼氬洜绯荤粺鑰屽紓銆?
Buffers
              鍘熷纾佺洏鍧楃殑鐩稿涓存椂瀛樺偍锛屼笉搴斿彉寰楅潪甯稿ぇ锛堢害 20MB 宸﹀彸锛?
Cached
              浠庣鐩樿鍙栫殑鏂囦欢锛堥〉缂撳瓨锛変互鍙?tmpfs 鍜?shmem 鐨勫唴瀛樼紦瀛樸€?
              涓嶅寘鎷?SwapCached銆?
SwapCached
              鏇剧粡琚崲鍑恒€佸張琚崲鍏ヤ絾浠嶅湪浜ゆ崲鏂囦欢涓殑鍐呭瓨锛堝鏋滃唴瀛樹笉瓒筹紝
              瀹冧笉闇€瑕佸啀娆¤鎹㈠嚭锛屽洜涓哄畠宸茬粡鍦ㄤ氦鎹㈡枃浠朵腑銆傝繖鑺傜渷浜?I/O锛?
Active
              鏈€杩戜娇鐢ㄨ繃鐨勫唴瀛橈紝閫氬父闄ら潪缁濆蹇呰鍚﹀垯涓嶈鍥炴敹銆?
Inactive
              鏈€杩戣緝灏戜娇鐢ㄧ殑鍐呭瓨銆傚畠鏇撮€傚悎琚洖鏀剁敤浜庡叾浠栫洰鐨?
Unevictable
              涓烘棤娉曞洖鏀剁殑鐢ㄦ埛绌洪棿鍒嗛厤鐨勫唴瀛橈紝渚嬪 mlocked 椤点€乺amfs 鍚庣椤点€?
              secret memfd 椤电瓑銆?
Mlocked
              鐢?mlock() 閿佸畾鐨勫唴瀛樸€?
HighTotal, HighFree
              Highmem 鏄墿鐞嗗唴瀛樹腑 ~860MB 浠ヤ笂鐨勬墍鏈夊唴瀛樸€?
              Highmem 鍖哄煙渚涚敤鎴风┖闂寸▼搴忔垨椤电紦瀛樹娇鐢ㄣ€傚唴鏍稿繀椤讳娇鐢ㄦ妧宸ф潵
              璁块棶姝ゅ唴瀛橈紝浣垮叾璁块棶姣?lowmem 鎱€?
LowTotal, LowFree
              Lowmem 鏄彲浠ョ敤浜?highmem 鍙娇鐢ㄧ殑鎵€鏈夌敤閫旂殑鍐呭瓨锛屼絾瀹冧篃鍙緵
              鍐呮牳鐢ㄤ簬鑷韩鐨勬暟鎹粨鏋勩€傞櫎璁稿鍏朵粬鐢ㄩ€斿锛孲lab 涓殑鎵€鏈夊唴瀹?
              閮藉湪杩欓噷鍒嗛厤銆傚綋浣犵敤灏?lowmem 鏃讹紝浼氬彂鐢熺碂绯曠殑浜嬫儏銆?
SwapTotal
              鍙敤浜ゆ崲绌洪棿鐨勬€婚噺
SwapFree
              宸蹭粠 RAM 涓€愬嚭銆佹殏鏃朵綅浜庣鐩樹笂鐨勫唴瀛?
Zswap
              zswap 鍚庣娑堣€楃殑鍐呭瓨锛堝帇缂╁悗鐨勫ぇ灏忥級
Zswapped
              瀛樺偍鍦?zswap 涓殑鍖垮悕鍐呭瓨閲忥紙鍘熷澶у皬锛?
Dirty
              姝ｅ湪绛夊緟鍐欏洖纾佺洏鐨勫唴瀛?
Writeback
              姝ｅ湪琚富鍔ㄥ啓鍥炵鐩樼殑鍐呭瓨
AnonPages
              鏄犲皠鍒扮敤鎴风┖闂撮〉琛ㄧ殑銆佹棤鏂囦欢鏀拺鐨勯〉銆傛敞鎰忔煇浜涘唴鏍搁厤缃彲鑳藉皢
              杈冨ぇ鍒嗛厤锛堜緥濡?THP锛夌殑鎵€鏈夐〉瑙嗕负"宸叉槧灏?锛屼竴鏃﹀崟涓〉琚槧灏勩€?
Mapped
              宸茶 mmapped 鐨勬枃浠讹紝渚嬪搴撱€傛敞鎰忔煇浜涘唴鏍搁厤缃彲鑳藉皢杈冨ぇ鍒嗛厤
              锛堜緥濡?THP锛夌殑鎵€鏈夐〉瑙嗕负"宸叉槧灏?锛屼竴鏃﹀崟涓〉琚槧灏勩€?
Shmem
              鍏变韩鍐呭瓨锛坰hmem锛夊拰 tmpfs 浣跨敤鐨勬€诲唴瀛?
KReclaimable
              鍐呮牳鍦ㄥ唴瀛樺帇鍔涗笅浼氬皾璇曞洖鏀剁殑鍐呮牳鍒嗛厤銆傚寘鎷?SReclaimable锛堣涓嬶級锛?
              浠ュ強鍏朵粬甯︽湁 shrinker 鐨勭洿鎺ュ垎閰嶃€?
Slab
              鍐呮牳鍐呮暟鎹粨鏋勭紦瀛?
SReclaimable
              Slab 涓彲鑳借鍥炴敹鐨勯儴鍒嗭紝渚嬪缂撳瓨
SUnreclaim
              Slab 涓湪鍐呭瓨鍘嬪姏涓嬫棤娉曞洖鏀剁殑閮ㄥ垎
KernelStack
              鎵€鏈変换鍔＄殑鏍告爤娑堣€楃殑鍐呭瓨
PageTables
              鐢ㄦ埛绌洪棿椤佃〃娑堣€楃殑鍐呭瓨
SecPageTables
              娆＄骇椤佃〃娑堣€楃殑鍐呭瓨锛岀洰鍓嶅寘鎷?x86 鍜?arm64 涓婄殑 KVM mmu 鍜?IOMMU 鍒嗛厤銆?
NFS_Unstable
              濮嬬粓涓洪浂銆備互鍓嶇敤浜庤鏁板凡鍐欏叆鏈嶅姟鍣ㄤ絾灏氭湭鎻愪氦鍒扮ǔ瀹氬瓨鍌ㄧ殑椤点€?
Bounce
              濮嬬粓涓洪浂銆備互鍓嶇敤浜庡潡璁惧"bounce buffers"鐨勫唴瀛樸€?
WritebackTmp
              濮嬬粓涓洪浂銆備互鍓嶇敤浜?FUSE 涓存椂鍐欏洖缂撳啿鍖虹殑鍐呭瓨銆?
CommitLimit
              鍩轰簬 overcommit 姣旂巼锛?vm.overcommit_ratio'锛夛紝杩欐槸绯荤粺涓婂綋鍓?
              鍙敤浜庡垎閰嶇殑鍐呭瓨鎬婚噺銆傚彧鏈夊湪鍚敤浜嗕弗鏍?overcommit 璁拌处鏃?
              锛?vm.overcommit_memory' 涓殑妯″紡 2锛夛紝鎵嶄細閬靛畧姝ら檺鍒躲€?

```
                CommitLimit = ([鎬?RAM 椤垫暟] - [鎬?huge TLB 椤垫暟]) *
                               overcommit_ratio / 100 + [鎬讳氦鎹㈤〉鏁癩

              渚嬪锛屽湪涓€涓叿鏈?1G 鐗╃悊 RAM 鍜?7G 浜ゆ崲绌洪棿銆佷笖 `vm.overcommit_ratio`
              涓?30 鐨勭郴缁熶笂锛屽皢寰楀埌 7.3G 鐨?CommitLimit銆?

              鏇村璇︾粏淇℃伅锛岃鍙傞槄 mm/overcommit-accounting 涓殑 overcommit 鏂囨。銆?
```
Committed_AS
              绯荤粺涓婂綋鍓嶅凡鍒嗛厤鐨勫唴瀛橀噺銆傚凡鎻愪氦鐨勫唴瀛樻槸鎵€鏈夎繘绋嬪凡鍒嗛厤鐨?
              鍐呭瓨涔嬪拰锛屽嵆浣垮畠浠皻鏈?浣跨敤"銆備竴涓?malloc() 浜?1G 鍐呭瓨浣?
              鍙Е鍙婂叾涓?300M 鐨勮繘绋嬶紝浼氭樉绀轰负浣跨敤浜?1G銆傝繖 1G 鏄凡琚?VM
              "鎻愪氦"鐨勫唴瀛橈紝鍙互鐢卞垎閰嶅簲鐢ㄧ▼搴忛殢鏃朵娇鐢ㄣ€傚湪绯荤粺涓婂惎鐢ㄤ簡涓ユ牸
              overcommit锛?vm.overcommit_memory' 涓殑妯″紡 2锛夋椂锛岃秴杩?
              CommitLimit锛堣瑙佷笂鏂囷級鐨勫垎閰嶅皢涓嶈鍏佽銆傚鏋滈渶瑕佷繚璇佽繘绋嬪湪
              鎴愬姛鍒嗛厤鍐呭瓨鍚庝笉浼氬洜缂哄皯鍐呭瓨鑰屽け璐ワ紝杩欏緢鏈夌敤銆?
VmallocTotal
              vmalloc 铏氭嫙鍦板潃绌洪棿鐨勬€诲ぇ灏?
VmallocUsed
              宸蹭娇鐢ㄧ殑 vmalloc 鍖哄煙澶у皬
VmallocChunk
              绌洪棽鐨?vmalloc 鍖哄煙涓渶澶х殑杩炵画鍧?
Percpu
              鍒嗛厤缁?percpu 鍒嗛厤鍣ㄧ敤浜庢敮鎾?percpu 鍒嗛厤鐨勫唴瀛樸€傛缁熻涓嶅寘鎷?
              鍏冩暟鎹殑寮€閿€銆?
EarlyMemtestBad
              浠?kB 涓哄崟浣嶇殑銆佽鏃╂湡 memtest 璇嗗埆涓烘崯鍧忕殑 RAM/鍐呭瓨閲忋€傚鏋滄湭杩愯
              memtest锛屽垯鏍规湰涓嶄細鏄剧ず姝ゅ瓧娈点€傚ぇ灏忔案杩滀笉浼氬悜涓嬭垗鍏ュ埌 0 kB銆?
              杩欐剰鍛崇潃濡傛灉鎶ュ憡涓?0 kB锛屼綘鍙互鏀惧績鍦板亣璁捐嚦灏戣繘琛屼簡涓€娆?memtest
              鎵弿锛屼笖娌℃湁浠讳綍涓€娆℃壂鎻忓彂鐜板崟涓崯鍧忕殑 RAM 瀛楄妭銆?
HardwareCorrupted
              鍐呮牳璇嗗埆涓哄凡鎹熷潖鐨?RAM/鍐呭瓨閲忥紙KB锛夈€?
AnonHugePages
              鏄犲皠鍒扮敤鎴风┖闂撮〉琛ㄧ殑鏃犳枃浠舵敮鎾戠殑澶ч〉
ShmemHugePages
              鐢卞叡浜唴瀛橈紙shmem锛夊拰 tmpfs 鐢ㄥぇ椤靛垎閰嶇殑鍐呭瓨
ShmemPmdMapped
              鐢ㄥぇ椤垫槧灏勫埌鐢ㄦ埛绌洪棿鐨勫叡浜唴瀛?
FileHugePages
              鏂囦欢绯荤粺鏁版嵁锛堥〉缂撳瓨锛夌敤澶ч〉鍒嗛厤鐨勫唴瀛?
FilePmdMapped
              鐢ㄥぇ椤垫槧灏勫埌鐢ㄦ埛绌洪棿鐨勯〉缂撳瓨
CmaTotal
              涓鸿繛缁唴瀛樺垎閰嶅櫒锛圕MA锛変繚鐣欑殑鍐呭瓨
CmaFree
              CMA 淇濈暀鍖轰腑鍓╀綑鐨勭┖闂插唴瀛?
Unaccepted
              灏氭湭琚?guest 鎺ュ彈鐨勫唴瀛?
Balloon
              鐢?VM Balloon 椹卞姩杩斿洖缁?Host 鐨勫唴瀛?
GPUActive
              鍒嗛厤缁欐椿鍔?GPU 瀵硅薄鐨勭郴缁熷唴瀛?
GPUReclaim
              瀛樺偍鍦?GPU 姹犱腑渚涘鐢ㄧ殑绯荤粺鍐呭瓨銆傛鍐呭瓨涓嶈鍏?GPUActive銆傚畠鏄?
              鍥犲叿鏈夐潪鏍囧噯椤佃〃灞炴€э紙濡?WC 鎴?UC锛夎€屼繚鐣欏湪澶嶇敤姹犱腑鐨?shrinker
              鍙洖鏀跺唴瀛樸€?
HugePages_Total, HugePages_Free, HugePages_Rsvd, HugePages_Surp, Hugepagesize, Hugetlb
              瑙?Documentation/admin-guide/mm/hugetlbpage.rst銆?
DirectMap4k, DirectMap2M, DirectMap1G
              鍐呮牳 RAM 鎭掔瓑鏄犲皠涓娇鐢ㄧ殑椤佃〃澶у皬缁嗗垎

#### vmallocinfo


鎻愪緵鍏充簬 vmalloced/vmaped 鍖哄煙鐨勪俊鎭€傛瘡涓尯鍩熶竴琛岋紝鍖呭惈璇ュ尯鍩熺殑铏氭嫙鍦板潃鑼冨洿銆佸瓧鑺傚ぇ灏忋€佸垱寤鸿€呯殑璋冪敤鑰呬俊鎭紝浠ュ強鍙栧喅浜庡尯鍩熺被鍨嬬殑鍙€変俊鎭細

 ==========  ===================================================
 pages=nr    椤垫暟閲?
 phys=addr   濡傛灉鎸囧畾浜嗙墿鐞嗗湴鍧€
 ioremap     I/O 鏄犲皠锛坕oremap() 鍙婂叾鐩稿叧鍑芥暟锛?
 vmalloc     vmalloc() 鍖哄煙
 vmap        vmap() 鏄犲皠鐨勯〉
 user        VM_USERMAP 鍖哄煙
 vpages     椤垫寚閽堢殑缂撳啿鍖鸿 vmalloced锛堝法澶у尯鍩燂級
 N<node>=nr  锛堜粎 NUMA 鍐呮牳涓婏級
             鍦ㄥ唴瀛樿妭鐐?<node> 涓婂垎閰嶇殑椤垫暟閲?
 ==========  ===================================================

```

    > cat /proc/vmallocinfo
    0xffffc20000000000-0xffffc20000201000 2101248 alloc_large_system_hash+0x204 ...
    /0x2c0 pages=512 vmalloc N0=128 N1=128 N2=128 N3=128
    0xffffc20000201000-0xffffc20000302000 1052672 alloc_large_system_hash+0x204 ...
    /0x2c0 pages=256 vmalloc N0=64 N1=64 N2=64 N3=64
    0xffffc20000302000-0xffffc20000304000    8192 acpi_tb_verify_table+0x21/0x4f...
    phys=7fee8000 ioremap
    0xffffc20000304000-0xffffc20000307000   12288 acpi_tb_verify_table+0x21/0x4f...
    phys=7fee7000 ioremap
    0xffffc2000031d000-0xffffc2000031f000    8192 init_vdso_vars+0x112/0x210
    0xffffc2000031f000-0xffffc2000032b000   49152 cramfs_uncompress_init+0x2e ...
    /0x80 pages=11 vmalloc N0=3 N1=3 N2=2 N3=3
    0xffffc2000033a000-0xffffc2000033d000   12288 sys_swapon+0x640/0xac0      ...
    pages=2 vmalloc N1=2
    0xffffc20000347000-0xffffc2000034c000   20480 xt_alloc_table_info+0xfe ...
    /0x130 [x_tables] pages=4 vmalloc N0=4
    0xffffffffa0000000-0xffffffffa000f000   61440 sys_init_module+0xc27/0x1d00 ...
    pages=14 vmalloc N2=14
    0xffffffffa000f000-0xffffffffa0014000   20480 sys_init_module+0xc27/0x1d00 ...
    pages=4 vmalloc N1=4
    0xffffffffa0014000-0xffffffffa0017000   12288 sys_init_module+0xc27/0x1d00 ...
    pages=2 vmalloc N1=2
    0xffffffffa0017000-0xffffffffa0022000   45056 sys_init_module+0xc27/0x1d00 ...
    pages=10 vmalloc N0=10

```

#### softirqs


鎻愪緵鑷惎鍔ㄤ互鏉ユ瘡涓?CPU 鏈嶅姟鐨?softirq 澶勭悊绋嬪簭璁℃暟銆?

```

    > cat /proc/softirqs
		  CPU0       CPU1       CPU2       CPU3
	HI:          0          0          0          0
    TIMER:       27166      27120      27097      27034
    NET_TX:          0          0          0         17
    NET_RX:         42          0          0         39
    BLOCK:           0          0        107       1121
    TASKLET:         0          0          0        290
    SCHED:       27035      26983      26971      26746
    HRTIMER:         0          0          0          0
	RCU:      1678       1769       2178       2250
```

### 1.3 /proc/net 涓殑缃戠粶淇℃伅


瀛愮洰褰?/proc/net 閬靛惊閫氬父鐨勬ā寮忋€傝〃 1-8 鏄剧ず浜嗗鏋滀綘閰嶇疆鍐呮牳鏀寔 IP 鐗堟湰 6 鎵€鑾峰緱鐨勯澶栧€笺€傝〃 1-9 鍒楀嚭浜嗚繖浜涙枃浠跺強鍏跺惈涔夈€?



 ========== =====================================================
 鏂囦欢      鍐呭
 ========== =====================================================
 udp6       UDP 濂楁帴瀛楋紙IPv6锛?
 tcp6       TCP 濂楁帴瀛楋紙IPv6锛?
 raw6       鍘熷璁惧缁熻锛圛Pv6锛?
 igmp6      鏈満宸插姞鍏ョ殑 IP 缁勬挱鍦板潃锛圛Pv6锛?
 if_inet6   IPv6 鎺ュ彛鍦板潃鍒楄〃
 ipv6_route 鍐呮牳 IPv6 璺敱琛?
 rt6_stats  鍏ㄥ眬 IPv6 璺敱琛ㄧ粺璁?
 sockstat6  濂楁帴瀛楃粺璁★紙IPv6锛?
 snmp6      Snmp 鏁版嵁锛圛Pv6锛?
 ========== =====================================================


 ============= ================================================================
 鏂囦欢         鍐呭
 ============= ================================================================
 arp           鍐呮牳 ARP 琛?
 dev           甯︾粺璁＄殑缃戠粶璁惧
 dev_mcast     璁惧姝ｅ湪鐩戝惉鐨勪簩灞傜粍鎾粍
                锛堟帴鍙ｇ储寮曘€佹爣绛俱€佸紩鐢ㄨ鏁般€佺粦瀹氬湴鍧€鏁帮級銆?
 dev_stat      缃戠粶璁惧鐘舵€?
 ip_fwchains   闃茬伀澧欓摼閾炬帴
 ip_fwnames    闃茬伀澧欓摼鍚嶇О
 ip_masq       鍖呭惈浼琛ㄧ殑鐩綍
 ip_masquerade 涓昏浼琛?
 netstat       缃戠粶缁熻
 raw           鍘熷璁惧缁熻
 route         鍐呮牳璺敱琛?
 rpc           鍖呭惈 rpc 淇℃伅鐨勭洰褰?
 rt_cache      璺敱缂撳瓨
 snmp          SNMP 鏁版嵁
 sockstat      濂楁帴瀛楃粺璁?
 softnet_stat  鍦ㄧ嚎 CPU 鐨勬瘡 CPU 鍏ョ珯鏁版嵁鍖呴槦鍒楃粺璁?
 tcp           TCP 濂楁帴瀛?
 udp           UDP 濂楁帴瀛?
 unix          UNIX 鍩熷鎺ュ瓧
 wireless      鏃犵嚎鎺ュ彛鏁版嵁锛圵avelan 绛夛級
 igmp          鏈満宸插姞鍏ョ殑 IP 缁勬挱鍦板潃
 psched        鍏ㄥ眬鏁版嵁鍖呰皟搴﹀櫒鍙傛暟銆?
 netlink       PF_NETLINK 濂楁帴瀛楀垪琛?
 ip_mr_vifs    缁勬挱铏氭嫙鎺ュ彛鍒楄〃
 ip_mr_cache   缁勬挱璺敱缂撳瓨鍒楄〃
 ============= ================================================================

浣犲彲浠ュ埄鐢ㄦ淇℃伅鏌ョ湅绯荤粺涓彲鐢ㄧ殑缃戠粶璁惧

```
  > cat /proc/net/dev
  Inter-|Receive                                                   |[...
   face |bytes    packets errs drop fifo frame compressed multicast|[...
      lo:  908188   5596     0    0    0     0          0         0 [...
    ppp0:15475140  20721   410    0    0   410          0         0 [...
    eth0:  614530   7085     0    0    0     0          0         1 [...

  ...] Transmit
  ...] bytes    packets errs drop fifo colls carrier compressed
  ...]  908188     5596    0    0    0     0       0          0
  ...] 1375103    17405    0    0    0     0       0          0
  ...] 1703981     5535    0    0    0     3       0          0
```

姝ゅ锛屾瘡涓?Channel Bond 鎺ュ彛閮芥湁鑷繁鐨勭洰褰曘€備緥濡傦紝bond0 璁惧灏嗘湁涓€涓悕涓?/proc/net/bond0/ 鐨勭洰褰曘€傚畠灏嗗寘鍚壒瀹氫簬璇?bond 鐨勪俊鎭紝渚嬪 bond 鐨勫綋鍓嶄粠璁惧銆佷粠璁惧鐨勯摼璺姸鎬侊紝浠ュ強浠庤澶囩殑閾捐矾澶辫触娆℃暟銆?

### 1.4 SCSI 淇℃伅


濡傛灉浣犵殑绯荤粺涓湁 SCSI 鎴?ATA 涓绘満閫傞厤鍣紝浣犱細鍦?/proc/scsi 涓壘鍒颁互璇ラ€傞厤鍣ㄩ┍鍔ㄥ懡鍚嶇殑瀛愮洰褰曘€?

```
  >cat /proc/scsi/scsi
  Attached devices:
  Host: scsi0 Channel: 00 Id: 00 Lun: 00
    Vendor: IBM      Model: DGHS09U          Rev: 03E0
    Type:   Direct-Access                    ANSI SCSI revision: 03
  Host: scsi0 Channel: 00 Id: 06 Lun: 00
    Vendor: PIONEER  Model: CD-ROM DR-U06S   Rev: 1.04
    Type:   CD-ROM                           ANSI SCSI revision: 02


```

浠ラ┍鍔ㄥ懡鍚嶇殑鐩綍閽堝绯荤粺涓壘鍒扮殑姣忎釜閫傞厤鍣ㄦ湁涓€涓枃浠躲€傝繖浜涙枃浠跺寘鍚叧浜庢帶鍒跺櫒鐨勪俊鎭紝鍖呮嫭鎵€浣跨敤鐨?IRQ 鍜?IO 鍦板潃鑼冨洿銆傛樉绀虹殑淇℃伅閲忓彇鍐充簬浣犱娇鐢ㄧ殑閫傞厤鍣ㄣ€傜ず渚嬫樉绀轰簡 Adaptec 鐨勮緭鍑?

```
  > cat /proc/scsi/aic7xxx/0

  Adaptec AIC7xxx driver version: 5.1.19/3.2.4
  Compile Options:
    TCQ Enabled By Default : Disabled
    AIC7XXX_PROC_STATS     : Disabled
    AIC7XXX_RESET_DELAY    : 5
  Adapter Configuration:
             SCSI Adapter: Adaptec AHA-294X Ultra SCSI host adapter
                             Ultra Wide Controller
      PCI MMAPed I/O Base: 0xeb001000
   Adapter SEEPROM Config: SEEPROM found and used.
        Adaptec SCSI BIOS: Enabled
                      IRQ: 10
                     SCBs: Active 0, Max Active 2,
                           Allocated 15, HW 16, Page 255
               Interrupts: 160328
        BIOS Control Word: 0x18b6
     Adapter Control Word: 0x005b
     Extended Translation: Enabled
  Disconnect Enable Flags: 0xffff
       Ultra Enable Flags: 0x0001
   Tag Queue Enable Flags: 0x0000
  Ordered Queue Tag Flags: 0x0000
  Default Tag Queue Depth: 8
      Tagged Queue By Device array for aic7xxx host instance 0:
        {255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255}
      Actual queue depth per device for aic7xxx host instance 0:
        {1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1}
  Statistics:
  (scsi0:0:0:0)
    Device using Wide/Sync transfers at 40.0 MByte/sec, offset 8
    Transinfo settings: current(12/8/1/0), goal(12/8/1/0), user(12/15/1/0)
    Total transfers 160151 (74577 reads and 85574 writes)
  (scsi0:0:6:0)
    Device using Narrow/Sync transfers at 5.0 MByte/sec, offset 15
    Transinfo settings: current(50/15/0/0), goal(50/15/0/0), user(50/15/0/0)
    Total transfers 0 (0 reads and 0 writes)


```

### 1.5 /proc/parport 涓殑骞惰绔彛淇℃伅


鐩綍 /proc/parport 鍖呭惈鍏充簬浣犵郴缁熷苟琛岀鍙ｇ殑淇℃伅銆傚畠涓烘瘡涓鍙ｆ湁涓€涓互绔彛鍙凤紙0,1,2,...锛夊懡鍚嶇殑瀛愮洰褰曘€?

杩欎簺鐩綍鍖呭惈琛?1-10 涓墍绀虹殑鍥涗釜鏂囦欢銆?



 ========= ====================================================================
 鏂囦欢      鍐呭
 ========= ====================================================================
 autoprobe 宸茶幏鍙栫殑浠绘剰 IEEE-1284 璁惧 ID 淇℃伅銆?
 devices   浣跨敤璇ョ鍙ｇ殑璁惧椹卞姩鍒楄〃銆傚綋鍓嶆鍦ㄤ娇鐢ㄨ绔彛鐨勮澶囧悕鏃佽竟浼氬嚭鐜颁竴涓?+锛堝畠鍙兘
           涓嶅嚭鐜板湪浠讳綍璁惧鍚嶆梺锛夈€?
 hardware  骞惰绔彛鐨勫熀鍦板潃銆両RQ 绾垮拰 DMA 閫氶亾銆?
 irq       parport 鐢ㄤ簬璇ョ鍙ｇ殑 IRQ銆傚畠鍦ㄤ竴涓崟鐙殑鏂囦欢涓紝鍏佽浣犻€氳繃鍐欏叆鏂板€?
           锛圛RQ 鍙锋垨 none锛夋潵鏇存敼瀹冦€?
 ========= ====================================================================

### 1.6 /proc/tty 涓殑 TTY 淇℃伅


鍏充簬鍙敤鍜屽疄闄呬娇鐢ㄧ殑 tty 鐨勪俊鎭彲浠ュ湪鐩綍 /proc/tty 涓壘鍒般€備綘浼氬湪杩欎釜鐩綍涓壘鍒伴┍鍔ㄥ拰绾胯矾瑙勭▼锛坙ine discipline锛夌殑鏉＄洰锛屽琛?1-11 鎵€绀恒€?



 ============= ==============================================
 鏂囦欢         鍐呭
 ============= ==============================================
 drivers      椹卞姩鍙婂叾浣跨敤鎯呭喌鐨勫垪琛?
 ldiscs       宸叉敞鍐岀殑绾胯矾瑙勭▼
 driver/serial 鍗曚釜 tty 绾胯矾鐨勪娇鐢ㄧ粺璁″拰鐘舵€?
 ============= ==============================================

瑕佹煡鐪嬪綋鍓嶆鍦ㄤ娇鐢ㄥ摢浜?tty锛屼綘鍙互鐩存帴鏌ョ湅鏂囦欢

```
  > cat /proc/tty/drivers
  pty_slave            /dev/pts      136   0-255 pty:slave
  pty_master           /dev/ptm      128   0-255 pty:master
  pty_slave            /dev/ttyp       3   0-255 pty:slave
  pty_master           /dev/pty        2   0-255 pty:master
  serial               /dev/cua        5   64-67 serial:callout
  serial               /dev/ttyS       4   64-67 serial
  /dev/tty0            /dev/tty0       4       0 system:vtmaster
  /dev/ptmx            /dev/ptmx       5       2 system
  /dev/console         /dev/console    5       1 system:console
  /dev/tty             /dev/tty        5       0 system:/dev/tty
  unknown              /dev/tty        4    1-63 console


```
### 1.7 /proc/stat 涓殑鏉傞」鍐呮牳缁熻


鍏充簬鍐呮牳娲诲姩鐨勫悇绉嶄俊鎭彲浠ュ湪 /proc/stat 鏂囦欢涓幏鍙栥€傝鏂囦欢涓姤鍛婄殑鎵€鏈夋暟瀛楅兘鏄仛鍚堝€?

```
  > cat /proc/stat
  cpu  237902850 368826709 106375398 1873517540 1135548 0 14507935 0 0 0
  cpu0 60045249 91891769 26331539 468411416 495718 0 5739640 0 0 0
  cpu1 59746288 91759249 26609887 468860630 312281 0 4384817 0 0 0
  cpu2 59489247 92985423 26904446 467808813 171668 0 2268998 0 0 0
  cpu3 58622065 92190267 26529524 468436680 155879 0 2114478 0 0 0
  intr 8688370575 8 3373 0 0 0 0 0 0 1 40791 0 0 353317 0 0 0 0 224789828 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 190974333 41958554 123983334 43 0 224593 0 0 0 <more 0's deleted>
  ctxt 22848221062
  btime 1605316999
  processes 746787147
  procs_running 2
  procs_blocked 0
  softirq 12121874454 100099120 3938138295 127375644 2795979 187870761 0 173808342 3072582055 52608 224184354
```

鏈€寮€濮嬬殑 "cpu" 琛岃仛鍚堜簡鎵€鏈夊叾浠?"cpuN" 琛屼腑鐨勬暟瀛椼€傝繖浜涙暟瀛楁爣璇嗕簡 CPU 鎵ц涓嶅悓绫诲瀷宸ヤ綔鎵€鑺辫垂鐨勬椂闂撮噺銆傛椂闂村崟浣嶆槸 USER_HZ锛堥€氬父鏄櫨鍒嗕箣涓€绉掞級銆傚悇鍒楃殑鍚箟浠庡乏鍒板彸濡備笅锛?

- user锛氬湪鐢ㄦ埛妯″紡涓嬫墽琛屾櫘閫氳繘绋?
- nice锛氬湪鐢ㄦ埛妯″紡涓嬫墽琛岃 nice 璋冩暣鐨勮繘绋?
- system锛氬湪鍐呮牳妯″紡涓嬫墽琛岀殑杩涚▼
- idle锛氱┖闂诧紙twiddling thumbs锛?
- iowait锛氫竴瑷€浠ヨ斀涔嬶紝iowait 琛ㄧず绛夊緟 I/O 瀹屾垚銆備絾鏈夊嚑涓棶棰橈細

  1. CPU 涓嶄細绛夊緟 I/O 瀹屾垚锛宨owait 鏄竴涓换鍔＄瓑寰?I/O 瀹屾垚鐨勬椂闂淬€傚綋 CPU 鍥犳湭瀹屾垚鐨勪换鍔?I/O 鑰岃繘鍏ョ┖闂茬姸鎬佹椂锛屽彟涓€涓换鍔″皢琚皟搴﹀埌璇?CPU 涓娿€?
  2. 鍦ㄥ鏍?CPU 涓紝绛夊緟 I/O 瀹屾垚鐨勪换鍔′笉鍦ㄤ换浣?CPU 涓婅繍琛岋紝鍥犳姣忎釜 CPU 鐨?iowait 闅句互璁＄畻銆?
  3. 鍦ㄦ煇浜涙儏鍐典笅锛?proc/stat 涓?iowait 瀛楁鐨勫€间細鍑忓皯銆?

  鍥犳锛屼粠 /proc/stat 璇诲彇 iowait 骞朵笉鍙潬銆?
- irq锛氭湇鍔′腑鏂?
- softirq锛氭湇鍔¤蒋涓柇
- steal锛氶潪鑷効绛夊緟
- guest锛氳繍琛屾櫘閫?guest
- guest_nice锛氳繍琛岃 nice 璋冩暣鐨?guest

"intr" 琛岀粰鍑轰簡鑷惎鍔ㄤ互鏉ユ湇鍔＄殑銆侀拡瀵规瘡涓彲鑳界郴缁熶腑鏂殑涓柇璁℃暟銆傜涓€鍒楁槸鎵€鏈夎鏈嶅姟涓柇鐨勬€绘暟锛屽寘鎷湭缂栧彿鐨勬灦鏋勭壒瀹氫腑鏂紱鍏跺悗鐨勬瘡涓€鍒楁槸璇ョ壒瀹氱紪鍙蜂腑鏂殑鎬绘暟銆傛湭缂栧彿鐨勪腑鏂笉浼氭樉绀猴紝鍙姹囨€诲埌鎬绘暟涓€?

"ctxt" 琛岀粰鍑轰簡璺ㄦ墍鏈?CPU 鐨勪笂涓嬫枃鍒囨崲鎬绘暟銆?

"btime" 琛岀粰鍑轰簡绯荤粺鍚姩鐨勬椂闂达紝浠ヨ嚜 Unix 绾厓浠ユ潵鐨勭鏁拌〃绀恒€?

"processes" 琛岀粰鍑轰簡宸插垱寤虹殑杩涚▼鍜岀嚎绋嬫暟锛屽寘鎷紙浣嗕笉闄愪簬锛夐€氳繃 fork() 鍜?clone() 绯荤粺璋冪敤鍒涘缓鐨勯偅浜涖€?

"procs_running" 琛岀粰鍑轰簡姝ｅ湪杩愯鎴栧噯澶囪繍琛岀殑绾跨▼鎬绘暟锛堝嵆鍙繍琛岀嚎绋嬫€绘暟锛夈€?

"procs_blocked" 琛岀粰鍑轰簡褰撳墠琚樆濉炪€佺瓑寰?I/O 瀹屾垚鐨勮繘绋嬫暟銆?

"softirq" 琛岀粰鍑轰簡鑷惎鍔ㄤ互鏉ユ湇鍔＄殑銆侀拡瀵规瘡涓彲鑳界郴缁?softirq 鐨?softirq 璁℃暟銆傜涓€鍒楁槸鎵€鏈夎鏈嶅姟 softirq 鐨勬€绘暟锛涘叾鍚庣殑姣忎竴鍒楁槸璇ョ壒瀹?softirq 鐨勬€绘暟銆?


### 1.8 Ext4 鏂囦欢绯荤粺鍙傛暟


鍏充簬宸叉寕杞?ext4 鏂囦欢绯荤粺鐨勪俊鎭彲浠ュ湪 /proc/fs/ext4 涓壘鍒般€傛瘡涓凡鎸傝浇鐨勬枃浠剁郴缁熶細鍦?/proc/fs/ext4 涓嬫湁涓€涓熀浜庡叾璁惧鍚嶇殑鐩綍锛堝嵆 /proc/fs/ext4/hdc 鎴?/proc/fs/ext4/sda9 鎴?/proc/fs/ext4/dm-0锛夈€傛瘡涓瘡璁惧鐩綍涓嬬殑鏂囦欢濡傝〃 1-12 鎵€绀恒€?



 ==============  ==========================================================
 鏂囦欢           鍐呭
 mb_groups       澶氬潡鍒嗛厤鍣ㄧ┖闂插潡 buddy 缂撳瓨鐨勮缁嗕俊鎭?
 ==============  ==========================================================

### 1.9 /proc/consoles


鏄剧ず宸叉敞鍐岀殑绯荤粺鎺у埗鍙扮嚎璺€?

瑕佹煡鐪嬪綋鍓嶇敤浜庣郴缁熸帶鍒跺彴鐨勫瓧绗﹁澶囩嚎璺細

```
  > cat /proc/consoles
  tty0                 -WU (ECp)       4:7
  ttyS0                -W- (Ep)        4:64
```

鍚勫垪濡備笅锛?

+--------------------+-------------------------------------------------------+
| device             | 璁惧鍚嶇О                                            |
+====================+=======================================================+
| operations         | * R = 鍙互杩涜璇绘搷浣?                                |
|                    | * W = 鍙互杩涜鍐欐搷浣?                                |
|                    | * U = 鍙互杩涜瑙ｉ櫎绌虹櫧锛坲nblank锛?                   |
+--------------------+-------------------------------------------------------+
| flags              | * E = 宸插惎鐢?                                         |
|                    | * C = 瀹冩槸棣栭€夋帶鍒跺彴                                  |
|                    | * B = 瀹冩槸涓诲紩瀵兼帶鍒跺彴                                |
|                    | * p = 瀹冪敤浜?printk 缂撳啿鍖?                           |
|                    | * b = 瀹冧笉鏄?TTY 鑰屾槸 Braille 璁惧                    |
|                    | * a = 鍦?cpu 绂荤嚎鏃跺畨鍏ㄤ娇鐢?                          |
+--------------------+-------------------------------------------------------+
| major:minor        | 璁惧鐨勪富璁惧鍙峰拰娆¤澶囧彿锛屼互鍐掑彿鍒嗛殧                  |
+--------------------+-------------------------------------------------------+

### 灏忕粨


/proc 鏂囦欢绯荤粺鎻愪緵鍏充簬杩愯绯荤粺鐨勪俊鎭€傚畠涓嶄粎鍏佽璁块棶杩涚▼鏁版嵁锛岃繕鍏佽浣犻€氳繃璇诲彇灞傜骇缁撴瀯涓殑鏂囦欢鏉ヨ姹傚唴鏍哥姸鎬併€?

/proc 鐨勭洰褰曠粨鏋勫弽鏄犱簡淇℃伅鐨勭被鍨嬶紝骞朵娇鏌ユ壘鐗瑰畾鏁版嵁鐨勪綅缃彉寰楀鏄擄紙鍗充究涓嶆槸鏄捐€屾槗瑙侊級銆?

## 绗?2 绔狅細淇敼绯荤粺鍙傛暟


### 鏈珷鍐呭


- 閫氳繃鍐欏叆 /proc/sys 涓殑鏂囦欢鏉ヤ慨鏀瑰唴鏍稿弬鏁?
- 鎺㈢储淇敼鐗瑰畾鍙傛暟鐨勬枃浠?
- 鍥為【 /proc/sys 鏂囦欢鏍?

------------------------------------------------------------------------------

/proc 涓潪甯告湁瓒ｇ殑涓€閮ㄥ垎鏄洰褰?/proc/sys銆傚畠涓嶄粎鏄俊鎭殑鏉ユ簮锛岃繕鍏佽浣犳洿鏀瑰唴鏍镐腑鐨勫弬鏁般€傚皾璇曟鎿嶄綔鏃惰闈炲父灏忓績銆備綘鍙互浼樺寲浣犵殑绯荤粺锛屼絾涔熷彲鑳戒娇瀹冨穿婧冦€傜粷涓嶈鍦ㄧ敓浜х郴缁熶笂鏇存敼鍐呮牳鍙傛暟銆傛惌寤轰竴鍙板紑鍙戞満鍣ㄥ苟杩涜娴嬭瘯锛屼互纭繚涓€鍒囨寜浣犳兂瑕佺殑鏂瑰紡宸ヤ綔銆備竴鏃﹀嚭閿欙紝浣犲彲鑳藉埆鏃犻€夋嫨锛屽彧鑳介噸鍚満鍣ㄣ€?

瑕佹洿鏀逛竴涓€硷紝鍙渶灏嗘柊鍊?echo 鍒版枃浠朵腑銆備綘闇€瑕佹槸 root 鎵嶈兘杩欐牱鍋氥€備綘鍙互鍒涘缓鑷繁鐨勫紩瀵艰剼鏈紝鍦ㄧ郴缁熸瘡娆″惎鍔ㄦ椂鎵ц姝ゆ搷浣溿€?

/proc/sys 涓殑鏂囦欢鍙敤浜庡井璋冨拰鐩戣 Linux 鍐呮牳杩愯涓殑鍚勭鍜屼竴鑸簨鍔°€傜敱浜庢煇浜涙枃浠跺彲鑳戒細涓嶇粡鎰忓湴鎵颁贡浣犵殑绯荤粺锛屽湪瀹為檯杩涜璋冩暣涔嬪墠锛屽缓璁悓鏃堕槄璇绘枃妗ｅ拰婧愪唬鐮併€傛棤璁哄浣曪紝鍐欏叆杩欎簺鏂囦欢涓殑浠讳綍鏂囦欢鏃堕兘瑕侀潪甯稿皬蹇冦€?proc 涓殑鏉＄洰鍦?2.1.* 鍜?2.2 鍐呮牳涔嬮棿鍙兘鐣ユ湁鍙樺寲锛屽洜姝ゅ鏈変换浣曠枒闂紝璇锋煡闃?linux/Documentation 鐩綍涓殑鍐呮牳鏂囨。銆傛湰绔犲ぇ閲忓熀浜?2.2 涔嬪墠鍐呮牳涓寘鍚殑鏂囨。锛屽苟鍦?Linux 鍐呮牳 2.2.1 鐗堟湰涓垚涓哄叾涓€閮ㄥ垎銆?

璇峰弬闃咃細Documentation/admin-guide/sysctl/ 鐩綍浠ヨ幏鍙栬繖浜涙潯鐩殑鎻忚堪銆?

### 灏忕粨


鍐呮牳琛屼负鐨勬煇浜涙柟闈㈠彲浠ュ湪杩愯鏃朵慨鏀癸紝鏃犻渶閲嶆柊缂栬瘧鍐呮牳锛岀敋鑷虫棤闇€閲嶅惎绯荤粺銆?proc/sys 鏍戜腑鐨勬枃浠朵笉浠呭彲浠ヨ鍙栵紝杩樺彲浠ヤ慨鏀广€備綘鍙互浣跨敤 echo 鍛戒护灏嗗€煎啓鍏ヨ繖浜涙枃浠讹紝浠庤€屾洿鏀瑰唴鏍哥殑榛樿璁剧疆銆?


## 绗?3 绔狅細姣忚繘绋嬪弬鏁?


### 3.1 /proc/<pid>/oom_adj & /proc/<pid>/oom_score_adj - 璋冩暣 oom-killer 鍒嗘暟


杩欎簺鏂囦欢鍙敤浜庤皟鏁寸敤浜庨€夋嫨鍐呭瓨涓嶈冻锛坥om锛夋潯浠朵笅鍝釜杩涚▼琚潃姝荤殑鍧忓害锛坆adness锛夊惎鍙戝紡銆?

鍧忓害鍚彂寮忎负姣忎釜鍊欓€変换鍔″垎閰嶄竴涓粠 0锛堜粠涓嶆潃姝伙級鍒?1000锛堟€绘槸鏉€姝伙級鐨勫€硷紝浠ョ‘瀹氬摢涓繘绋嬫槸鐩爣銆傝繖浜涘崟浣嶅ぇ鑷存槸鍩轰簬瀵瑰叾褰撳墠鍐呭瓨鍜屼氦鎹娇鐢ㄩ噺鐨勪及璁★紝璇ヨ繘绋嬪彲鑳戒粠涓垎閰嶇殑鍏佽鍐呭瓨鑼冨洿涓婄殑涓€涓瘮渚嬨€備緥濡傦紝濡傛灉涓€涓换鍔℃鍦ㄤ娇鐢ㄦ墍鏈夊厑璁哥殑鍐呭瓨锛屽畠鐨勫潖搴﹀垎鏁板皢鏄?1000銆傚鏋滃畠姝ｅ湪浣跨敤鍏跺厑璁稿唴瀛樼殑涓€鍗婏紝瀹冪殑鍒嗘暟灏嗘槸 500銆?

"鍏佽"鐨勫唴瀛橀噺鍙栧喅浜庤皟鐢?oom killer 鐨勪笂涓嬫枃銆傚鏋滄槸鍥犱负鍒嗛厤缁欏垎閰嶄换鍔＄殑 cpuset 鐨勫唴瀛樿€楀敖锛屽厑璁稿唴瀛樿〃绀鸿 cpuset 琚垎閰嶇殑涓€缁?mems銆傚鏋滄槸鍥犱负 mempolicy 鐨勮妭鐐硅€楀敖锛屽厑璁稿唴瀛樿〃绀鸿 mempolicy 鑺傜偣闆嗐€傚鏋滄槸鍥犱负杈惧埌浜嗗唴瀛橀檺鍒讹紙鎴栦氦鎹㈤檺鍒讹級锛屽厑璁稿唴瀛樺氨鏄厤缃殑闄愬埗銆傛渶鍚庯紝濡傛灉鏄洜涓烘暣涓郴缁熷唴瀛樹笉瓒筹紝鍏佽鍐呭瓨琛ㄧず鎵€鏈夊彲鍒嗛厤璧勬簮銆?

/proc/<pid>/oom_score_adj 鐨勫€煎湪鐢ㄤ簬纭畾瑕佹潃姝诲摢涓换鍔′箣鍓嶈鍔犲埌鍧忓害鍒嗘暟涓娿€傚彲鎺ュ彈鐨勫€艰寖鍥翠粠 -1000锛圤OM_SCORE_ADJ_MIN锛夊埌 +1000锛圤OM_SCORE_ADJ_MAX锛夈€傝繖鍏佽鐢ㄦ埛绌洪棿閫氳繃鎬绘槸鍋忓ソ鏌愪釜浠诲姟鎴栧畬鍏ㄧ鐢ㄥ畠鏉ユ瀬鍖?oom 鏉€姝荤殑鍋忓ソ銆傛渶浣庡彲鑳界殑鍊?-1000 鐩稿綋浜庡畬鍏ㄧ鐢ㄨ浠诲姟鐨?oom 鏉€姝伙紝鍥犱负瀹冩€绘槸鎶ュ憡鍧忓害鍒嗘暟涓?0銆?

鍥犳锛岀敤鎴风┖闂村畾涔夋瘡涓换鍔¤鑰冭檻鐨勫唴瀛橀噺闈炲父绠€鍗曘€備緥濡傦紝璁剧疆 /proc/<pid>/oom_score_adj 鍊间负 +500锛屽ぇ鑷寸浉褰撲簬鍏佽鍏变韩鍚屼竴绯荤粺銆乧puset銆乵empolicy 鎴栧唴瀛樻帶鍒跺櫒璧勬簮鐨勫叾浣欎换鍔″浣跨敤鑷冲皯 50% 鐨勫唴瀛樸€傚彟涓€鏂归潰锛屽€?-500 澶ц嚧鐩稿綋浜庝粠璇ヤ换鍔＄殑璁″垎涓墸闄ゅ叾鍏佽鍐呭瓨鐨?50%銆?

涓轰簡涓庝互鍓嶇殑鍐呮牳鍚戝悗鍏煎锛?proc/<pid>/oom_adj 涔熷彲鐢ㄤ簬璋冩暣鍧忓害鍒嗘暟銆傚叾鍙帴鍙楃殑鍊艰寖鍥翠粠 -16锛圤OM_ADJUST_MIN锛夊埌 +15锛圤OM_ADJUST_MAX锛夛紝浠ュ強鐗规畩鍊?-17锛圤OM_DISABLE锛変互瀹屽叏绂佺敤璇ヤ换鍔＄殑 oom 鏉€姝汇€傚叾鍊奸殢 /proc/<pid>/oom_score_adj 绾挎€х缉鏀俱€?

/proc/<pid>/oom_score_adj 鐨勫€间笉鑳介檷浣庡埌鏈€鍚庝竴涓敱 CAP_SYS_RESOURCE 杩涚▼璁剧疆鐨勫€间互涓嬨€傝闄嶄綆鍒版洿浣庣殑鍊奸渶瑕?CAP_SYS_RESOURCE銆?


### 3.2 /proc/<pid>/oom_score - 鏄剧ず褰撳墠 oom-killer 鍒嗘暟


璇ユ枃浠跺彲鐢ㄤ簬妫€鏌?oom-killer 瀵逛换浣曠粰瀹?<pid> 浣跨敤鐨勫綋鍓嶅垎鏁般€傚皢瀹冨拰 /proc/<pid>/oom_score_adj 涓€璧蜂娇鐢紝浠ヨ皟鏁村湪鍐呭瓨涓嶈冻鎯呭喌涓嬪簲鏉€姝诲摢涓繘绋嬨€?

璇锋敞鎰忥紝瀵煎嚭鐨勫€煎寘鍚?oom_score_adj锛屽洜姝ゅ畠瀹為檯涓婂湪 [0,2000] 鑼冨洿鍐呫€?


### 3.3  /proc/<pid>/io - 鏄剧ず IO 缁熻瀛楁


璇ユ枃浠跺寘鍚瘡涓繍琛岃繘绋嬬殑 IO 缁熻銆?

#### 绀轰緥


```
    test:/tmp # dd if=/dev/zero of=/tmp/test.dat &
    [1] 3828

    test:/tmp # cat /proc/3828/io
    rchar: 323934931
    wchar: 323929600
    syscr: 632687
    syscw: 632675
    read_bytes: 0
    write_bytes: 323932160
    cancelled_write_bytes: 0
```

#### 鎻忚堪


##### rchar


I/O 璁℃暟鍣細璇诲彇鐨勫瓧绗︽暟
璇ヤ换鍔″鑷翠粠瀛樺偍璇诲彇鐨勫瓧鑺傛暟銆傝繖绠€鍗曟槸璇ヨ繘绋嬩紶閫掔粰 read() 鍜?pread() 鐨勫瓧鑺傛暟涔嬪拰銆傚畠鍖呭惈鍍?tty IO 涔嬬被鐨勫唴瀹癸紝骞朵笖涓嶅彈鏄惁闇€瑕佸疄闄呯墿鐞嗙鐩?IO 鐨勫奖鍝嶏紙璇诲彇鍙兘鐢遍〉缂撳瓨婊¤冻锛夈€?


##### wchar


I/O 璁℃暟鍣細鍐欏叆鐨勫瓧绗︽暟
璇ヤ换鍔″鑷存垨灏嗚瀵艰嚧鍐欏叆纾佺洏鐨勫瓧鑺傛暟銆傝繖閲岄€傜敤鐨勬敞鎰忎簨椤逛笌 rchar 绫讳技銆?


##### syscr


I/O 璁℃暟鍣細璇荤郴缁熻皟鐢ㄦ暟
灏濊瘯缁熻璇?I/O 鎿嶄綔鐨勬暟閲忥紝鍗冲儚 read() 鍜?pread() 杩欐牱鐨勭郴缁熻皟鐢ㄣ€?


##### syscw


I/O 璁℃暟鍣細鍐欑郴缁熻皟鐢ㄦ暟
灏濊瘯缁熻鍐?I/O 鎿嶄綔鐨勬暟閲忥紝鍗冲儚 write() 鍜?pwrite() 杩欐牱鐨勭郴缁熻皟鐢ㄣ€?


##### read_bytes


I/O 璁℃暟鍣細璇诲彇鐨勫瓧鑺傛暟
灏濊瘯缁熻璇ヨ繘绋嬬湡姝ｅ鑷翠粠瀛樺偍灞傝幏鍙栧埌鐨勫瓧鑺傛暟銆傚湪 submit_bio() 绾у埆瀹屾垚锛屽洜姝ゅ浜庡潡璁惧鏀寔鐨勬枃浠剁郴缁熸槸鍑嗙‘鐨勩€?璇峰湪浠ュ悗琛ュ厖鍏充簬 NFS 鍜?CIFS 鐨勭姸鎬?


##### write_bytes


I/O 璁℃暟鍣細鍐欏叆鐨勫瓧鑺傛暟
灏濊瘯缁熻璇ヨ繘绋嬪鑷村彂閫佸埌瀛樺偍灞傜殑瀛楄妭鏁般€傝繖鍦ㄩ〉鍙樿剰鏃惰繘琛屻€?


##### cancelled_write_bytes


杩欓噷鏈€澶х殑涓嶅噯纭箣澶勬槸鎴柇锛坱runcate锛夈€傚鏋滀竴涓繘绋嬪悜涓€涓枃浠跺啓鍏?1MB锛岀劧鍚庡垹闄よ鏂囦欢锛屽畠瀹為檯涓婁笉浼氭墽琛屼换浣曞啓鍥炪€備絾瀹冧細琚涓哄鑷翠簡 1MB 鐨勫啓鍏ャ€?
鎹㈠彞璇濊锛氳杩涚▼閫氳繃鎴柇椤电紦瀛樿€屽鑷存湭鍙戠敓鐨勫瓧鑺傛暟銆備竴涓换鍔′篃鍙兘瀵艰嚧"璐?鐨?IO銆傚鏋滆浠诲姟鎴柇浜嗘煇浜涜剰椤电紦瀛橈紝鍙︿竴涓换鍔″凡琚鍏ワ紙鍦ㄥ叾 write_bytes 涓級鐨勬煇浜?IO 灏嗕笉浼氬彂鐢熴€傛垜浠琠鍙互_浠庢埅鏂换鍔＄殑 write_bytes 涓噺鍘昏鍊硷紝浣嗚繖鏍峰仛浼氬鑷翠俊鎭涪澶便€?



   鍦ㄥ叾褰撳墠鐨勫疄鐜扮姸鎬佷笅锛岃繖鍦?32 浣嶆満鍣ㄤ笂鏈変簺瀛樺湪绔炴€侊細濡傛灉杩涚▼ A 鍦ㄨ繘绋?B 鏇存柊鍏朵腑涓€涓?64 浣嶈鏁板櫒鏃惰鍙栬繘绋?B 鐨?/proc/pid/io锛岃繘绋?A 鍙兘浼氱湅鍒颁竴涓腑闂寸粨鏋溿€?


鍏充簬姝ょ殑鏇村淇℃伅鍙互鍦?Documentation/accounting 涓殑 taskstats 鏂囨。涓壘鍒般€?

### 3.4 /proc/<pid>/coredump_filter - 鏍稿績杞偍杩囨护璁剧疆

褰撲竴涓繘绋嬭杞偍鏃讹紝鍙鏍稿績鏂囦欢鐨勫ぇ灏忎笉鍙楅檺鍒讹紝鎵€鏈夊尶鍚嶅唴瀛橀兘浼氳鍐欏叆鏍稿績鏂囦欢銆備絾鏈夋椂鎴戜滑涓嶆兂杞偍鏌愪簺鍐呭瓨娈碉紝渚嬪宸ㄥぇ鐨勫叡浜唴瀛樻垨 DAX銆傜浉鍙嶏紝鏈夋椂鎴戜滑鎯冲皢鏂囦欢鏀寔鐨勫唴瀛樻淇濆瓨鍒版牳蹇冩枃浠朵腑锛岃€屼笉浠呬粎鏄悇涓枃浠躲€?

/proc/<pid>/coredump_filter 鍏佽浣犺嚜瀹氫箟褰?<pid> 杩涚▼琚浆鍌ㄦ椂灏嗚浆鍌ㄥ摢浜涘唴瀛樻銆俢oredump_filter 鏄竴涓唴瀛樼被鍨嬬殑浣嶆帺鐮併€傚鏋滀綅鎺╃爜鐨勬煇涓€浣嶈璁剧疆锛屽垯鐩稿簲鍐呭瓨绫诲瀷鐨勫唴瀛樻浼氳杞偍锛屽惁鍒欎笉浼氳浆鍌ㄣ€?

鏀寔浠ヤ笅 9 绉嶅唴瀛樼被鍨嬶細

  - 锛堜綅 0锛夊尶鍚嶇鏈夊唴瀛?
  - 锛堜綅 1锛夊尶鍚嶅叡浜唴瀛?
  - 锛堜綅 2锛夋枃浠舵敮鎸佺殑绉佹湁鍐呭瓨
  - 锛堜綅 3锛夋枃浠舵敮鎸佺殑鍏变韩鍐呭瓨
  - 锛堜綅 4锛夋枃浠舵敮鎸佺殑绉佹湁鍐呭瓨鍖哄煙涓殑 ELF 澶撮〉锛堜粎褰撲綅 2 琚竻闄ゆ椂鏈夋晥锛?
  - 锛堜綅 5锛塰ugetlb 绉佹湁鍐呭瓨
  - 锛堜綅 6锛塰ugetlb 鍏变韩鍐呭瓨
  - 锛堜綅 7锛塂AX 绉佹湁鍐呭瓨
  - 锛堜綅 8锛塂AX 鍏变韩鍐呭瓨

  娉ㄦ剰锛孧MIO 椤碉紙濡傚抚缂撳啿锛夋案杩滀笉浼氳杞偍锛岃€?vDSO 椤垫棤璁轰綅鎺╃爜鐘舵€佸浣曟€绘槸琚浆鍌ㄣ€?

  娉ㄦ剰浣?0-4 涓嶅奖鍝?hugetlb 鎴?DAX 鍐呭瓨銆俬ugetlb 鍐呭瓨浠呭彈浣?5-6 褰卞搷锛孌AX 浠呭彈浣?7-8 褰卞搷銆?

coredump_filter 鐨勯粯璁ゅ€兼槸 0x33锛涜繖鎰忓懗鐫€鎵€鏈夊尶鍚嶅唴瀛樻銆丒LF 澶撮〉鍜?hugetlb 绉佹湁鍐呭瓨閮戒細琚浆鍌ㄣ€?

濡傛灉浣犱笉鎯宠浆鍌ㄩ檮鍔犲埌 pid 1234 鐨勬墍鏈夊叡浜唴瀛樻锛?

```
  $ echo 0x31 > /proc/1234/coredump_filter
```

褰撳垱寤轰竴涓柊杩涚▼鏃讹紝璇ヨ繘绋嬩粠鍏剁埗杩涚▼缁ф壙浣嶆帺鐮佺姸鎬併€傚湪绋嬪簭杩愯涔嬪墠璁剧疆 coredump_filter 寰堟湁鐢ㄣ€?

```
  $ echo 0x7 > /proc/self/coredump_filter
  $ ./some_program
```

### 3.5	/proc/<pid>/mountinfo - 鍏充簬鎸傝浇鐨勪俊鎭?


```
    36 35 98:0 /mnt1 /mnt2 rw,noatime master:1 - ext3 /dev/root rw,errors=continue
    (1)(2)(3)   (4)   (5)      (6)     (n鈥) (m+1)(m+2) (m+3)         (m+4)

    (1)   mount ID:        鎸傝浇鐨勫敮涓€鏍囪瘑绗︼紙umount 鍚庡彲鑳借澶嶇敤锛?
    (2)   parent ID:       鐖舵寕杞界殑 ID锛堟垨鎸傝浇鏍戦《绔殑鑷韩 ID锛?
    (3)   major:minor:     鏂囦欢绯荤粺涓婃枃浠剁殑 st_dev 鍊?
    (4)   root:            鏂囦欢绯荤粺涓鎸傝浇鐨勬牴
    (5)   mount point:     鐩稿浜庤繘绋嬫牴鐩綍鐨勬寕杞界偣
    (6)   mount options:   姣忎釜鎸傝浇鐨勯€夐」
    (n鈥) optional fields: 闆朵釜鎴栧涓?"tag[:value]" 褰㈠紡鐨勫瓧娈?
    (m+1) separator:       鍙€夊瓧娈电粨鏉熺殑鏍囪
    (m+2) filesystem type: "type[.subtype]" 褰㈠紡鐨勬枃浠剁郴缁熷悕
    (m+3) mount source:    鏂囦欢绯荤粺鐗瑰畾淇℃伅鎴?"none"
    (m+4) super options:   姣忎釜瓒呯骇鍧楃殑閫夐」
```

瑙ｆ瀽鍣ㄥ簲蹇界暐鎵€鏈夋棤娉曡瘑鍒殑鍙€夊瓧娈点€傜洰鍓嶅彲鑳界殑鍙€夊瓧娈垫湁锛?

================  ==============================================================
shared:X          mount 鍦?peer group X 涓叡浜?
master:X          mount 鏄?peer group X 鐨勪粠灞烇紙slave锛?
propagate_from:X  璇?mount 鏄?slave 骞朵粠 peer group X 鎺ユ敹浼犳挱 [#]_
unbindable        mount 涓嶅彲缁戝畾锛坲nbindable锛?
================  ==============================================================

       X 鏄 mount 鐨勭洿鎺?master锛屾垨鑰呭鏋滃湪鍚屼竴鏍逛笅娌℃湁鍗犱富瀵肩殑 peer
       group锛屽垯鍙嚭鐜?"master:X" 瀛楁锛岃€屼笉鍑虹幇 "propagate_from:X" 瀛楁銆?

鍏充簬鎸傝浇浼犳挱鐨勬洿澶氫俊鎭紝璇峰弬闃咃細

  Documentation/filesystems/sharedsubtree.rst


### 3.6	/proc/<pid>/comm  & /proc/<pid>/task/<tid>/comm

杩欎簺鏂囦欢鎻愪緵浜嗕竴绉嶈闂换鍔?comm 鍊肩殑鏂规硶銆傚畠杩樺厑璁告煇涓换鍔¤缃畠鑷韩鎴栧叾鏌愪釜绾跨▼鍏勫紵鐨?comm 鍊笺€備笌 cmdline 鍊肩浉姣旓紝comm 鍊肩殑澶у皬鍙楀埌闄愬埗锛屽洜姝ゅ啓鍏ヨ秴杩囧唴鏍?TASK_COMM_LEN锛堝綋鍓嶄负 16 涓瓧绗︼紝鍖呭惈 NUL 缁堟绗︼級鐨勫唴瀹逛細瀵艰嚧 comm 鍊艰鎴柇銆?


### 3.7	/proc/<pid>/task/<tid>/children - 鍏充簬浠诲姟瀛愯繘绋嬬殑淇℃伅

璇ユ枃浠舵彁渚涗簡涓€绉嶅揩閫熻幏鍙栫敱 <pid>/<tid> 瀵规墍鎸囦换鍔＄殑绗竴灞傚瓙杩涚▼ pid 鐨勬柟娉曘€傚叾鏍煎紡涓轰互绌烘牸鍒嗛殧鐨?pid 娴併€?

娉ㄦ剰杩欓噷鐨?绗竴灞?鈥斺€斿鏋滀竴涓瓙杩涚▼杩樻湁瀹冭嚜宸辩殑瀛愯繘绋嬶紝鍒欎笉浼氬垪鍦ㄨ繖閲岋紱闇€瑕佽鍙?/proc/<children-pid>/task/<tid>/children 鏉ヨ幏鍙栧叾鍚庝唬銆?

鐢变簬璇ユ帴鍙ｆ棬鍦ㄥ揩閫熶笖寤変环锛屽畠涓嶄繚璇佹彁渚涚簿纭殑缁撴灉锛屾煇浜涘瓙杩涚▼鍙兘浼氳璺宠繃锛岀壒鍒槸濡傛灉瀹冧滑鍦ㄦ墦鍗板嚭 pid 涔嬪悗绔嬪嵆閫€鍑猴紝鍥犳鍦ㄩ渶瑕佺簿纭粨鏋滄椂锛岄渶瑕佸仠姝㈡垨鍐荤粨琚鏌ヨ繘绋嬨€?


### 3.8	/proc/<pid>/fdinfo/<fd> - 鍏充簬宸叉墦寮€鏂囦欢鐨勪俊鎭?

璇ユ枃浠舵彁渚涗笌宸叉墦寮€鏂囦欢鐩稿叧鐨勪俊鎭€傚父瑙勬枃浠惰嚦灏戞湁鍥涗釜瀛楁鈥斺€?pos'銆?flags'銆?mnt_id' 鍜?'ino'銆?pos' 浠ュ崄杩涘埗褰㈠紡琛ㄧず璇ュ凡鎵撳紑鏂囦欢鐨勫綋鍓嶅亸绉婚噺 [璇﹁ lseek(2)]锛?flags' 琛ㄧず鏂囦欢鍒涘缓鏃朵娇鐢ㄧ殑鍏繘鍒?O_xxx 鎺╃爜 [璇﹁ open(2)]锛?mnt_id' 琛ㄧず鍖呭惈璇ュ凡鎵撳紑鏂囦欢鐨勬枃浠剁郴缁熺殑鎸傝浇 ID [璇﹁ 3.5 /proc/<pid>/mountinfo]銆?ino' 琛ㄧず璇ユ枃浠剁殑 inode 鍙枫€?

```
	pos:	0
	flags:	0100002
	mnt_id:	19
	ino:	63107
```

```
    lock:       1: FLOCK  ADVISORY  WRITE 359 00:13:11691 0 EOF
```

鍍?eventfd銆乫snotify銆乻ignalfd銆乪poll 杩欐牱鐨勬枃浠讹紝鍦ㄥ父瑙勭殑 pos/flags 涔嬪杩樻彁渚涗笌鍏舵墍浠ｈ〃瀵硅薄鐩稿叧鐨勯檮鍔犱俊鎭€?

#### Eventfd 鏂囦欢


```
	pos:	0
	flags:	04002
	mnt_id:	9
	ino:	63107
	eventfd-count:	5a
```

鍏朵腑 'eventfd-count' 鏄竴涓鏁板櫒鐨勫崄鍏繘鍒跺€笺€?

#### Signalfd 鏂囦欢


```
	pos:	0
	flags:	04002
	mnt_id:	9
	ino:	63107
	sigmask:	0000000000000200
```

鍏朵腑 'sigmask' 鏄笌璇ユ枃浠跺叧鑱旂殑 signal mask 鐨勫崄鍏繘鍒跺€笺€?

#### Epoll 鏂囦欢


```
	pos:	0
	flags:	02
	mnt_id:	9
	ino:	63107
	tfd:        5 events:       1d data: ffffffffffffffff pos:0 ino:61af sdev:7
```

鍏朵腑 'tfd' 鏄崄杩涘埗褰㈠紡鐨勭洰鏍囨枃浠舵弿杩扮缂栧彿锛?events' 鏄鍦ㄨ鐩戣鐨勪簨浠舵帺鐮侊紝'data' 鏄笌鐩爣鍏宠仈鐨勬暟鎹?[璇﹁ epoll(7)]銆?

'pos' 鏄洰鏍囨枃浠跺綋鍓嶅亸绉婚噺鐨勫崄杩涘埗褰㈠紡 [瑙?lseek(2)]锛?ino' 鍜?'sdev' 鏄洰鏍囨枃浠舵墍鍦ㄤ綅缃殑 inode 鍜岃澶囧彿锛屽潎浠ュ崄鍏繘鍒舵牸寮忚〃绀恒€?

#### Fsnotify 鏂囦欢


```
	pos:	0
	flags:	02000000
	mnt_id:	9
	ino:	63107
	inotify wd:3 ino:9e7e sdev:800013 mask:800afce ignored_mask:0 fhandle-bytes:8 fhandle-type:1 f_handle:7e9e0000640d1b6d
```

鍏朵腑 'wd' 鏄崄杩涘埗褰㈠紡鐨勭洃瑙嗘弿杩扮锛屽嵆鐩爣鏂囦欢鎻忚堪绗︾紪鍙凤紝'ino' 鍜?'sdev' 鏄洰鏍囨枃浠舵墍鍦ㄧ殑 inode 鍜岃澶囧彿锛?mask' 鏄簨浠舵帺鐮侊紝鍧囦互鍗佸叚杩涘埗褰㈠紡 [璇﹁ inotify(7)]銆?

濡傛灉鍐呮牳鍦ㄦ瀯寤烘椂鍚敤浜?exportfs 鏀寔锛屽垯鍒扮洰鏍囨枃浠剁殑璺緞琚紪鐮佷负涓€涓枃浠跺彞鏌勩€傝鏂囦欢鍙ユ焺鐢变笁涓瓧娈?'fhandle-bytes'銆?fhandle-type' 鍜?'f_handle' 鎻愪緵锛屽潎涓哄崄鍏繘鍒舵牸寮忋€?

濡傛灉鍐呮牳鍦ㄦ病鏈?exportfs 鏀寔鐨勬儏鍐典笅鏋勫缓锛屽垯涓嶄細鎵撳嵃鍑烘枃浠跺彞鏌勩€?

濡傛灉灏氭湭闄勫姞浠讳綍 inotify 鏍囪锛屽垯 'inotify' 琛屼細琚渷鐣ャ€?

```
	pos:	0
	flags:	02
	mnt_id:	9
	ino:	63107
	fanotify flags:10 event-flags:0
	fanotify mnt_id:12 mflags:40 mask:38 ignored_mask:40000003
	fanotify ino:4f969 sdev:800013 mflags:0 mask:3b ignored_mask:40000000 fhandle-bytes:8 fhandle-type:1 f_handle:69f90400c275b5b4
```

鍏朵腑 fanotify 鐨?'flags' 鍜?'event-flags' 鏄?fanotify_init 璋冪敤涓娇鐢ㄧ殑鍊硷紝'mnt_id' 鏄寕杞界偣鏍囪瘑绗︼紝'mflags' 鏄笌鏍囪鍏宠仈鐨勩€佷笌浜嬩欢鎺╃爜鍒嗗紑璺熻釜鐨?flags 鍊笺€?ino' 鍜?'sdev' 鏄洰鏍?inode 鍜岃澶囧彿锛?mask' 鏄簨浠舵帺鐮侊紝'ignored_mask' 鏄琚拷鐣ョ殑浜嬩欢鎺╃爜銆傛墍鏈夊潎涓哄崄鍏繘鍒舵牸寮忋€傚紩鍏?'mflags'銆?mask' 鍜?'ignored_mask' 鎻愪緵浜嗗叧浜?fanotify_mark 璋冪敤涓娇鐢ㄧ殑 flags 鍜屾帺鐮佺殑淇℃伅 [璇﹁ fsnotify 鎵嬪唽椤礭銆?

铏界劧鍓嶄笁琛屾槸寮哄埗鐨勪笖濮嬬粓浼氭墦鍗帮紝浣嗗叾浣欓儴鍒嗘槸鍙€夌殑锛屽鏋滄病鏈夊垱寤轰换浣曟爣璁板垯鍙兘浼氳鐪佺暐銆?

#### Timerfd 鏂囦欢


```
	pos:	0
	flags:	02
	mnt_id:	9
	ino:	63107
	clockid: 0
	ticks: 0
	settime flags: 01
	it_value: (0, 49406829)
	it_interval: (1, 0)
```

鍏朵腑 'clockid' 鏄椂閽熺被鍨嬶紝'ticks' 鏄凡鍙戠敓鐨勫畾鏃跺櫒鍒版湡娆℃暟 [璇﹁ timerfd_create(2)]銆?settime flags' 鏄敤浜庤缃畾鏃跺櫒鐨勫叓杩涘埗褰㈠紡 flags [璇﹁ timerfd_settime(2)]銆?it_value' 鏄窛绂诲畾鏃跺櫒鍒版湡鐨勫墿浣欐椂闂淬€?it_interval' 鏄畾鏃跺櫒鐨勯棿闅斻€傛敞鎰忥紝瀹氭椂鍣ㄥ彲鑳戒娇鐢?TIMER_ABSTIME 閫夐」璁剧疆锛岃繖浼氭樉绀哄湪 'settime flags' 涓紝浣?'it_value' 浠嶇劧鏄剧ず瀹氭椂鍣ㄧ殑鍓╀綑鏃堕棿銆?

#### DMA Buffer 鏂囦欢


```
	pos:	0
	flags:	04002
	mnt_id:	9
	ino:	63107
	size:   32768
	count:  2
	exp_name:  system-heap
```

鍏朵腑 'size' 鏄?DMA buffer 鐨勫ぇ灏忥紙浠ュ瓧鑺備负鍗曚綅锛夈€?count' 鏄?DMA buffer 鏂囦欢鐨勬枃浠惰鏁般€?exp_name' 鏄?DMA buffer 瀵煎嚭鑰呯殑鍚嶇О銆?

#### VFIO Device 鏂囦欢


```
	pos:    0
	flags:  02000002
	mnt_id: 17
	ino:    5122
	vfio-device-syspath: /sys/devices/pci0000:e0/0000:e0:01.1/0000:e1:00.0/0000:e2:05.0/0000:e8:00.0
```

鍏朵腑 'vfio-device-syspath' 鏄笌 VFIO 璁惧鏂囦欢瀵瑰簲鐨?sysfs 璺緞銆?

### 3.9	/proc/<pid>/map_files - 鍏充簬鍐呭瓨鏄犲皠鏂囦欢鐨勪俊鎭?

璇ョ洰褰曞寘鍚〃绀哄唴瀛樻槧灏勬枃浠剁殑绗﹀彿閾炬帴銆?

```
     | lr-------- 1 root root 64 Jan 27 11:24 333c600000-333c620000 -> /usr/lib64/ld-2.18.so
     | lr-------- 1 root root 64 Jan 27 11:24 333c81f000-333c820000 -> /usr/lib64/ld-2.18.so
     | lr-------- 1 root root 64 Jan 27 11:24 333c820000-333c821000 -> /usr/lib64/ld-2.18.so
     | ...
     | lr-------- 1 root root 64 Jan 27 11:24 35d0421000-35d0422000 -> /usr/lib64/libselinux.so.1
     | lr-------- 1 root root 64 Jan 27 11:24 400000-41a000 -> /usr/bin/ls
```

閾炬帴鐨勫悕绉拌〃绀轰竴涓槧灏勭殑铏氭嫙鍐呭瓨杈圭晫锛屽嵆 **vm_area_struct**锛歷m_start-vm_area_struct::vm_end銆?

map_files 鐨勪富瑕佺敤閫旀槸浠ュ揩閫熺殑鏂瑰紡鑾峰彇涓€缁勫唴瀛樻槧灏勬枃浠讹紝鑰屾棤闇€瑙ｆ瀽 /proc/<pid>/maps 鎴?/proc/<pid>/smaps锛堣繖涓よ€呴兘鍖呭惈鏇村鐨勮褰曪級銆傚悓鏃讹紝鍙互浠庝袱涓繘绋嬬殑鏂囦欢鍒楄〃涓?open(2) 鏄犲皠锛屽苟姣旇緝瀹冧滑鐨?inode 鍙凤紝浠ョ‘瀹氬摢浜涘尶鍚嶅唴瀛樺尯鍩熷疄闄呬笂鏄叡浜殑銆?

### 3.10	/proc/<pid>/timerslack_ns - 浠诲姟 timerslack 鍊?

璇ユ枃浠舵彁渚涗换鍔＄殑 timerslack 鍊硷紙浠ョ撼绉掍负鍗曚綅锛夈€傝鍊兼寚瀹氫簡鏅€氬畾鏃跺櫒鍙互琚帹杩熺殑涓€娈垫椂闂达紝浠ヤ究灏嗗畾鏃跺櫒鍚堝苟锛岄伩鍏嶄笉蹇呰鐨勫敜閱掋€?

杩欏厑璁歌皟鏁翠换鍔＄殑浜や簰鎬т笌鍔熻€椾箣闂寸殑鏉冭　銆?

鍚戣鏂囦欢鍐欏叆 0 浼氬皢浠诲姟鐨?timerslack 璁句负榛樿鍊笺€?

鏈夋晥鍊艰寖鍥翠负 0 - ULLONG_MAX銆?

瑕佹洿鏀规煇浠诲姟鐨?timerslack_ns 鍊硷紝璁剧疆璇ュ€肩殑搴旂敤绋嬪簭蹇呴』瀵硅鎸囧畾浠诲姟鍏锋湁 PTRACE_MODE_ATTACH_FSCREDS 绾у埆鐨勬潈闄愩€?

### 3.11	/proc/<pid>/patch_state - Livepatch 琛ヤ竵鎿嶄綔鐘舵€?

褰撳惎鐢?CONFIG_LIVEPATCH 鏃讹紝璇ユ枃浠舵樉绀鸿浠诲姟鐨勮ˉ涓佺姸鎬佸€笺€?

鍊?'-1' 琛ㄧず娌℃湁琛ヤ竵澶勪簬杞崲锛坱ransition锛夌姸鎬併€?

鍊?'0' 琛ㄧず鏈変竴涓ˉ涓佸浜庤浆鎹㈢姸鎬佷笖璇ヤ换鍔℃湭琚墦琛ヤ竵銆傚鏋滆ˉ涓佹鍦ㄨ鍚敤锛屽垯璇ヤ换鍔″皻鏈鎵撹ˉ涓併€傚鏋滆ˉ涓佹鍦ㄨ绂佺敤锛屽垯璇ヤ换鍔″凡缁忚鍙栨秷琛ヤ竵銆?

鍊?'1' 琛ㄧず鏈変竴涓ˉ涓佸浜庤浆鎹㈢姸鎬佷笖璇ヤ换鍔″凡琚墦琛ヤ竵銆傚鏋滆ˉ涓佹鍦ㄨ鍚敤锛屽垯璇ヤ换鍔″凡缁忚鎵撹ˉ涓併€傚鏋滆ˉ涓佹鍦ㄨ绂佺敤锛屽垯璇ヤ换鍔″皻鏈鍙栨秷琛ヤ竵銆?

### 3.12 /proc/<pid>/arch_status - 浠诲姟鏋舵瀯鐗瑰畾鐘舵€?

褰撳惎鐢?CONFIG_PROC_PID_ARCH_STATUS 鏃讹紝璇ユ枃浠舵樉绀鸿浠诲姟鐨勬灦鏋勭壒瀹氱姸鎬併€?

#### 绀轰緥


```
 $ cat /proc/6753/arch_status
 AVX512_elapsed_ms:      8
```

#### 鎻忚堪


#### x86 鐗瑰畾鏉＄洰


##### AVX512_elapsed_ms


  濡傛灉鏈哄櫒鏀寔 AVX512锛岃鏉＄洰鏄剧ず鑷笂娆¤褰?AVX512 浣跨敤浠ユ潵缁忚繃鐨勬绉掓暟銆傝褰曟槸鍦ㄤ换鍔¤璋冨害鍑?CPU 鏃跺敖鍔涜繘琛岀殑銆傝繖鎰忓懗鐫€璇ュ€煎彇鍐充簬涓や釜鍥犵礌锛?

    1) 浠诲姟鍦?CPU 涓婃湭琚皟搴﹀嚭鎵€鑺辫垂鐨勬椂闂淬€傚湪 CPU 闅旂涓斿彧鏈変竴涓彲杩愯浠诲姟鐨勬儏鍐典笅锛岃繖鍙兘鑺辫垂鏁扮銆?

    2) 鑷换鍔′笂娆¤璋冨害鍑轰互鏉ョ粡杩囩殑鏃堕棿銆傛牴鎹璋冨害鍑虹殑鍘熷洜锛堟椂闂寸墖鑰楀敖銆乻yscall ...锛夛紝杩欏彲鑳芥槸浠绘剰闀跨殑鏃堕棿銆?

  鍥犳锛岃鍊间笉鑳借瑙嗕綔绮剧‘涓旀潈濞佺殑淇℃伅銆備娇鐢ㄦ淇℃伅鐨勫簲鐢ㄧ▼搴忓繀椤讳簡瑙ｇ郴缁熶笂鐨勬暣浣撳満鏅紝浠ョ‘瀹氭煇涓换鍔℃槸鍚︾湡鐨勬槸 AVX512 鐢ㄦ埛銆傜簿纭俊鎭彲浠ラ€氳繃鎬ц兘璁℃暟鍣ㄨ幏寰椼€?

  鐗规畩鍊?'-1' 琛ㄧず娌℃湁璁板綍鍒?AVX512 浣跨敤锛屽洜姝よ浠诲姟涓嶅お鍙兘鏄?AVX512 鐢ㄦ埛锛屼絾杩欎篃鍙栧喅浜庡伐浣滆礋杞藉拰璋冨害鍦烘櫙锛屼篃鍙兘鍑虹幇涓婅堪鍋囬槾鎬с€?

### 3.13 /proc/<pid>/fd - 鎸囧悜鎵撳紑鏂囦欢鐨勭鍙烽摼鎺ュ垪琛?

璇ョ洰褰曞寘鍚〃绀烘墦寮€鏂囦欢鐨勭鍙烽摼鎺ャ€?

```
  lr-x------ 1 root root 64 Sep 20 17:53 0 -> /dev/null
  l-wx------ 1 root root 64 Sep 20 17:53 1 -> /dev/null
  lrwx------ 1 root root 64 Sep 20 17:53 10 -> 'socket:[12539]'
  lrwx------ 1 root root 64 Sep 20 17:53 11 -> 'socket:[12540]'
  lrwx------ 1 root root 64 Sep 20 17:53 12 -> 'socket:[12542]'
```

杩涚▼鎵撳紑鏂囦欢鐨勬暟閲忓瓨鍌ㄥ湪 /proc/<pid>/fd 鐨?stat() 杈撳嚭鐨?'size' 鎴愬憳涓紝浠ヤ究蹇€熻闂€?


### 3.14 /proc/<pid>/ksm_stat - 鍏充簬杩涚▼ ksm 鐘舵€佺殑淇℃伅

褰撳惎鐢?CONFIG_KSM 鏃讹紝姣忎釜杩涚▼閮芥湁姝ゆ枃浠讹紝鏄剧ず ksm 鍚堝苟鐘舵€佺殑淇℃伅銆?

#### 绀轰緥


```
    / # cat /proc/self/ksm_stat
    ksm_rmap_items 0
    ksm_zero_pages 0
    ksm_merging_pages 0
    ksm_process_profit 0
    ksm_merge_any: no
    ksm_mergeable: no
```

#### 鎻忚堪


##### ksm_rmap_items


ksm_rmap_item 缁撴瀯鐨勪娇鐢ㄦ暟閲忋€俴sm_rmap_item 缁撴瀯瀛樺偍铏氭嫙鍦板潃鐨勫弽鍚戞槧灏勪俊鎭€侹SM 浼氫负璇ヨ繘绋嬫瘡涓 ksm 鎵弿鐨勯〉鐢熸垚涓€涓?ksm_rmap_item銆?

##### ksm_zero_pages


褰?/sys/kernel/mm/ksm/use_zero_pages 琚惎鐢ㄦ椂锛屽畠琛ㄧず鏈夊灏戜釜绌洪〉琚?KSM 涓庡唴鏍搁浂椤靛悎骞躲€?

##### ksm_merging_pages


瀹冭〃绀烘湁澶氬皯涓杩涚▼鐨勯〉鍙備笌浜?KSM 鍚堝苟锛堜笉鍖呮嫭 ksm_zero_pages锛夈€傚畠涓?/proc/<pid>/ksm_merging_pages 鎵€鏄剧ず鐨勫唴瀹圭浉鍚屻€?

##### ksm_process_profit


KSM 甯︽潵鐨勬敹鐩婏紙鑺傜渷鐨勫瓧鑺傛暟锛夈€侹SM 鍙互閫氳繃鍚堝苟鐩稿悓鐨勯〉鏉ヨ妭鐪佸唴瀛橈紝浣嗕篃鍙兘娑堣€楅澶栫殑鍐呭瓨锛屽洜涓哄畠闇€瑕佷负姣忎釜琚壂鎻忕殑椤电敓鎴愪竴涓?rmap_item 鏉ヤ繚瀛樺叾绠€瑕佺殑 rmap 淇℃伅銆傚叾涓竴浜涢〉鍙兘琚悎骞讹紝浣嗘湁浜涘湪澶氭妫€鏌ュ悗浠嶅彲鑳芥棤娉曞悎骞讹紝杩欎簺灏辨槸琚秷鑰楃殑鏃犳敹鐩婂唴瀛樸€?

##### ksm_merge_any


瀹冩寚瀹氳杩涚▼鐨?'mm 鏄惁宸茶 prctl() 鍔犲叆 KSM 鐨勫€欓€夊垪琛紝浠ュ強 KSM 鎵弿鏄惁鍦ㄨ繘绋嬬骇鍒瀹屽叏鍚敤銆?

##### ksm_mergeable


瀹冩寚瀹氳杩涚▼鐨?mms 涓槸鍚︽湁浠讳綍 VMA 褰撳墠閫傜敤浜?KSM銆?

鍏充簬 KSM 鐨勬洿澶氫俊鎭彲浠ュ湪 Documentation/admin-guide/mm/ksm.rst 涓壘鍒般€?


## 绗?4 绔狅細閰嶇疆 procfs


### 4.1	鎸傝浇閫夐」


鏀寔浠ヤ笅鎸傝浇閫夐」锛?

	=========	========================================================
	hidepid=	璁剧疆 /proc/<pid>/ 鐨勮闂ā寮忋€?
	gid=		璁剧疆琚巿鏉冧簡瑙ｈ繘绋嬩俊鎭殑缁勩€?
	subset=		鍙樉绀?procfs 鐨勬寚瀹氬瓙闆嗐€?
	pidns=		鎸囧畾璇?procfs 浣跨敤鐨勫懡鍚嶇┖闂淬€?
	=========	========================================================

hidepid=off 鎴?hidepid=0 琛ㄧず缁忓吀妯″紡鈥斺€旀瘡涓汉閮藉彲浠ヨ闂墍鏈?/proc/<pid>/ 鐩綍锛堥粯璁わ級銆?

hidepid=noaccess 鎴?hidepid=1 琛ㄧず鐢ㄦ埛鍙兘璁块棶鍏惰嚜韬殑 /proc/<pid>/ 鐩綍锛屼笉鑳借闂叾浠栦换浣曠洰褰曘€傚儚 cmdline銆乻ched*銆乻tatus 杩欐牱鐨勬晱鎰熸枃浠剁幇鍦ㄥ彈鍒颁繚鎶わ紝闃叉鍏朵粬鐢ㄦ埛璁块棶銆傝繖浣垮緱浠栦汉鏃犳硶寰楃煡鏄惁鏈夌敤鎴疯繍琛屼簡鐗瑰畾绋嬪簭锛堝墠鎻愭槸绋嬪簭娌℃湁閫氳繃鍏惰嚜韬涓烘毚闇茶嚜宸憋級銆備綔涓洪澶栫殑濂藉锛岀敱浜?/proc/<pid>/cmdline 瀵瑰叾浠栫敤鎴蜂笉鍙闂紝閭ｄ簺閫氳繃绋嬪簭鍙傛暟浼犻€掓晱鎰熶俊鎭殑缂栧啓涓嶈壇鐨勭▼搴忕幇鍦ㄤ篃鍙楀埌淇濇姢锛岄槻姝㈡湰鍦扮獌鍚€呫€?

hidepid=invisible 鎴?hidepid=2 琛ㄧず鍦?hidepid=1 鐨勫熀纭€涓婏紝鎵€鏈?/proc/<pid>/ 瀵瑰叾浠栫敤鎴峰畬鍏ㄤ笉鍙銆傝繖骞朵笉鎰忓懗鐫€闅愯棌浜嗘槸鍚﹀瓨鍦ㄥ叿鏈夌壒瀹?pid 鍊肩殑杩涚▼杩欎竴浜嬪疄锛堝畠鍙互閫氳繃鍏朵粬鏂瑰紡寰楃煡锛屼緥濡?"kill -0 $PID"锛夛紝浣嗗畠闅愯棌浜嗚繘绋嬬殑 uid 鍜?gid锛屽惁鍒欏彲浠ラ€氳繃 stat() /proc/<pid>/ 鏉ュ緱鐭ャ€傚畠鏋佸ぇ鍦板鍔犱簡鍏ヤ镜鑰呮敹闆嗘鍦ㄨ繍琛岃繘绋嬩俊鎭殑闅惧害锛屼緥濡傛煇涓畧鎶よ繘绋嬫槸鍚︿互鎻愭潈鏂瑰紡杩愯銆佸叾浠栫敤鎴锋槸鍚﹁繍琛屼簡鏌愪簺鏁忔劅绋嬪簭銆佸叾浠栫敤鎴锋槸鍚﹁繍琛屼簡浠讳綍绋嬪簭绛夌瓑銆?

hidepid=ptraceable 鎴?hidepid=4 琛ㄧず procfs 搴斿彧鍖呭惈璋冪敤鑰呭彲浠?ptrace 鐨?/proc/<pid>/ 鐩綍銆?

gid= 瀹氫箟涓€涓鎺堟潈浜嗚В杩涚▼淇℃伅鐨勭粍锛屽惁鍒欒淇℃伅浼氳 hidepid= 绂佹銆傚鏋滀綘浣跨敤鍍?identd 杩欐牱闇€瑕佷簡瑙ｈ繘绋嬩俊鎭殑瀹堟姢杩涚▼锛屽彧闇€灏?identd 鍔犲叆璇ョ粍銆?

subset=pid 闅愯棌 procfs 涓墍鏈変笌浠诲姟鏃犲叧鐨勬渶椤跺眰鏂囦欢鍜岀洰褰曘€?

pidns= 鎸囧畾涓€涓?pid 鍛藉悕绌洪棿锛堝彲浠ユ槸绫讳技 `/proc/$pid/ns/pid` 鐨勫瓧绗︿覆璺緞锛屼篃鍙互鏄娇鐢?`FSCONFIG_SET_FD` 鏃剁殑鏂囦欢鎻忚堪绗︼級锛宲rocfs 瀹炰緥鍦ㄨ浆鎹?pid 鏃跺皢浣跨敤璇ュ懡鍚嶇┖闂淬€傞粯璁ゆ儏鍐典笅锛宲rocfs 灏嗕娇鐢ㄨ皟鐢ㄨ繘绋嬬殑娲诲姩 pid 鍛藉悕绌洪棿銆傛敞鎰忥紝鐜版湁 procfs 瀹炰緥鐨?pid 鍛藉悕绌洪棿鏃犳硶琚慨鏀癸紙灏濊瘯杩欐牱鍋氫細寰楀埌 `-EBUSY` 閿欒锛夈€?

## 绗?5 绔狅細鏂囦欢绯荤粺琛屼负


鏈€鍒濓紝鍦?pid 鍛藉悕绌洪棿鍑虹幇涔嬪墠锛宲rocfs 鏄竴涓叏灞€鏂囦欢绯荤粺銆傝繖鎰忓懗鐫€绯荤粺涓彧鏈変竴涓?procfs 瀹炰緥銆?

褰撳姞鍏?pid 鍛藉悕绌洪棿鍚庯紝鍦ㄦ瘡涓?pid 鍛藉悕绌洪棿涓細鎸傝浇涓€涓嫭绔嬬殑 procfs 瀹炰緥銆傚洜姝わ紝procfs 鐨勬寕杞介€夐」鍦ㄦ墍鏈夋寕杞藉疄渚嬩箣闂存槸鍏ㄥ眬鐨勶細

```
	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=2 0 0

	# strace -e mount mount -o hidepid=1 -t proc proc /tmp/proc
	mount("proc", "/tmp/proc", "proc", 0, "hidepid=1") = 0
	+++ exited with 0 +++

	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=2 0 0
	proc /tmp/proc proc rw,relatime,hidepid=2 0 0
```

鍙湁鍦ㄩ噸鏂版寕杞?procfs 涔嬪悗锛屾寕杞介€夐」鎵嶄細鏀瑰彉锛?

```
	# mount -o remount,hidepid=1 -t proc proc /tmp/proc

	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=1 0 0
	proc /tmp/proc proc rw,relatime,hidepid=1 0 0
```

杩欑琛屼负涓庡叾浠栨枃浠剁郴缁熺殑琛屼负涓嶅悓銆?

鏂扮殑 procfs 琛屼负鏇寸被浼间簬鍏朵粬鏂囦欢绯荤粺銆傛瘡娆℃寕杞?procfs 閮戒細鍒涘缓涓€涓柊鐨?procfs 瀹炰緥銆傛寕杞介€夐」鍙奖鍝嶈嚜韬殑 procfs 瀹炰緥銆傝繖鎰忓懗鐫€鍙互鎷ユ湁澶氫釜 procfs 瀹炰緥锛?

```
	# mount -o hidepid=invisible -t proc proc /proc
	# mount -o hidepid=noaccess -t proc proc /tmp/proc
	# grep ^proc /proc/mounts
	proc /proc proc rw,relatime,hidepid=invisible 0 0
	proc /tmp/proc proc rw,relatime,hidepid=noaccess 0 0
```
