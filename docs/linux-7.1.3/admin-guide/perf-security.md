
## Perf 浜嬩欢涓庡伐鍏峰畨鍏?

### 姒傝堪


Linux 鐨勬€ц兘璁℃暟鍣紙perf_events锛塠^1^]_ 銆?[^2^]_ , [^3^]_ 鐨勪娇鐢ㄥ彲鑳藉甫鏉ョ浉褰撳ぇ鐨?椋庨櫓锛屽鑷磋鐩戞帶杩涚▼璁块棶鐨勬晱鎰熸暟鎹硠闇层€傛棤璁烘槸鍦ㄧ洿鎺ヤ娇鐢?perf_events 绯荤粺璋冪敤
API [^2^]_ 鐨勫満鏅腑锛岃繕鏄€氳繃 Perf 宸ュ叿鐢ㄦ埛鎬佸疄鐢ㄧ▼搴忥紙Perf锛塠^3^]_ , [^4^]_ 鐢熸垚
鐨勬暟鎹枃浠朵腑锛岄兘鍙兘鍙戠敓鏁版嵁娉勯湶銆傝椋庨櫓鍙栧喅浜?perf_events 鎬ц兘鐩戞帶鍗曞厓锛圥MU锛?[^2^]_ 涓?Perf 涓烘€ц兘鍒嗘瀽鎵€閲囬泦鍜屾毚闇茬殑鏁版嵁鐨勬€ц川銆傛墍閲囬泦鐨勭郴缁熶笌鎬ц兘鏁版嵁鍙?鍒嗕负浠ヤ笅鍑犵被锛?
1. 绯荤粺纭欢涓庤蒋浠堕厤缃暟鎹紝渚嬪锛欳PU 鍨嬪彿鍙婂叾缂撳瓨閰嶇疆銆佸彲鐢ㄥ唴瀛樺ぇ灏忓強鍏?   鎷撴墤銆佹墍鐢ㄧ殑鍐呮牳涓?Perf 鐗堟湰銆佹€ц兘鐩戞帶璁剧疆锛堝惈瀹為獙鏃堕棿銆佷簨浠堕厤缃€丳erf
   鍛戒护琛屽弬鏁扮瓑锛夈€?
2. 鐢ㄦ埛鎬佷笌鍐呮牳妯″潡璺緞鍙婂叾鍔犺浇鍦板潃涓庡ぇ灏忋€佽繘绋嬩笌绾跨▼鍚嶅強鍏?PID 鍜?TID銆?   鎵€鎹曡幏纭欢涓庤蒋浠朵簨浠剁殑鏃堕棿鎴炽€?
3. 鍐呮牳杞欢璁℃暟鍣ㄧ殑鍐呭锛堜緥濡備笂涓嬫枃鍒囨崲銆佺己椤点€丆PU 杩佺Щ锛夈€佹灦鏋勭‖浠舵€ц兘
   璁℃暟鍣紙PMC锛塠^8^]_ 浠ュ強鏈哄櫒鐗瑰畾瀵勫瓨鍣紙MSR锛塠^9^]_ 鈥斺€?瀹冧滑涓虹郴缁熶腑鍚勭被
   琚洃鎺ч儴鍒嗭紙渚嬪鍐呭瓨鎺у埗鍣紙IMC锛夈€佷簰杩烇紙QPI/UPI锛夋垨澶栬锛圥CIe锛塽ncore
   璁℃暟鍣級鎻愪緵鎵ц搴﹂噺锛岃€屼笉鐩存帴褰掑睘浜庝换浣曟墽琛屼笂涓嬫枃鐘舵€併€?
4. 鏋舵瀯鎵ц涓婁笅鏂囧瘎瀛樺櫒鐨勫唴瀹癸紙渚嬪 x86_64 涓婄殑 RIP銆丷SP銆丷BP锛夈€佽繘绋嬬殑鐢ㄦ埛
   鎬佷笌鍐呮牳鎬佸唴瀛樺湴鍧€鍙婃暟鎹紝浠ュ強鎹曡幏姝ょ被鍒暟鎹殑鍚勭被鏋舵瀯 MSR 鐨勫唴瀹广€?
灞炰簬绗洓绫荤殑鏁版嵁鍙兘鍖呭惈鏁忔劅杩涚▼鏁版嵁銆傚鏋滄煇浜涚洃鎺фā寮忎笅鐨?PMU 鎹曡幏鎵ц涓婁笅鏂?瀵勫瓨鍣ㄧ殑鍊兼垨杩涚▼鍐呭瓨涓殑鏁版嵁锛岄偅涔堝姝ょ被鐩戞帶妯″紡鐨勮闂繀椤昏姝ｇ‘鎺掑簭鍜屽畨鍏?淇濇姢銆傚洜姝わ紝perf_events 鎬ц兘鐩戞帶涓庡彲瑙傛祴鎬ф搷浣滄槸瀹夊叏璁块棶鎺у埗绠＄悊鐨勫璞?[^5^]_ 銆?
### perf_events 璁块棶鎺у埗


涓轰簡鎵ц瀹夊叏妫€鏌ワ紝Linux 鐨勫疄鐜板皢杩涚▼鍒嗕负涓ょ被 [^6^]_ 锛歛锛夌壒鏉冭繘绋嬶紙鍏舵湁鏁堢敤鎴?ID 涓?0锛屽嵆瓒呯骇鐢ㄦ埛鎴?root锛夛紝浠ュ強 b锛夐潪鐗规潈杩涚▼锛堝叾鏈夋晥 UID 闈為浂锛夈€傜壒鏉冭繘绋?缁曡繃鎵€鏈夊唴鏍稿畨鍏ㄦ潈闄愭鏌ワ紝鍥犳 perf_events 鎬ц兘鐩戞帶瀵圭壒鏉冭繘绋嬪畬鍏ㄥ紑鏀撅紝涓嶅彈
璁块棶銆佽寖鍥翠笌璧勬簮闄愬埗銆?
闈炵壒鏉冭繘绋嬪垯瑕佽繘琛屽熀浜庤繘绋嬪嚟鎹?[^5^]_ 锛堥€氬父鏄細鏈夋晥 UID銆佹湁鏁?GID 浠ュ強闄勫姞
缁勫垪琛級鐨勫畬鏁村畨鍏ㄦ潈闄愭鏌ャ€?
Linux 灏嗕紶缁熶笂涓庤秴绾х敤鎴峰叧鑱旂殑鐗规潈鍒掑垎涓轰笉鍚岀殑鍗曞厓锛岀О涓?capabilities [^6^]_ 锛?瀹冧滑鍙互鍦ㄩ潪鐗规潈鐢ㄦ埛鐨勮繘绋嬪拰鏂囦欢涓婃寜绾跨▼鐙珛鍦板惎鐢ㄥ拰绂佺敤銆?
鍚敤浜?CAP_PERFMON capability 鐨勯潪鐗规潈杩涚▼锛屽湪 perf_events 鎬ц兘鐩戞帶涓庡彲瑙傛祴鎬?鎿嶄綔鏂归潰琚涓虹壒鏉冭繘绋嬶紝浠庤€岀粫杩囧唴鏍镐腑鐨?**鑼冨洿锛坰cope锛?* 鏉冮檺妫€鏌ャ€侰AP_PERFMON
鍦ㄥ唴鏍镐腑涓烘€ц兘鐩戞帶涓庡彲瑙傛祴鎬ф搷浣滃疄鐜颁簡鏈€灏忕壒鏉冨師鍒?[^13^]_ 锛圥OSIX 1003.1e:
2.2.2.39锛夛紝骞舵彁渚涗簡涓€绉嶅畨鍏ㄧ殑绯荤粺鎬ц兘鐩戞帶涓庡彲瑙傛祴鎬ф柟娉曘€?
鍑轰簬鍚戝悗鍏煎鐨勮€冭檻锛屽 perf_events 鐩戞帶涓庡彲瑙傛祴鎬ф搷浣滅殑璁块棶涔熷 CAP_SYS_ADMIN
鐗规潈杩涚▼寮€鏀撅紝浣嗙浉姣?CAP_PERFMON capability锛屼笉寤鸿灏?CAP_SYS_ADMIN 鐢ㄤ簬瀹夊叏
鐩戞帶涓庡彲瑙傛祴鎬у満鏅€傚鏋滄煇杩涚▼浣跨敤 perf_events 绯荤粺璋冪敤 API 鐨勭郴缁熷璁¤褰?[^14^]_
鍚屾椂鍖呭惈鑾峰彇 CAP_PERFMON 涓?CAP_SYS_ADMIN 涓ょ capability 鐨勬嫆缁濊褰曪紝鍒欏缓璁?鍗曠嫭涓鸿杩涚▼鎻愪緵 CAP_PERFMON capability锛屼綔涓鸿В鍐虫€ц兘鐩戞帶涓庡彲瑙傛祴鎬т娇鐢ㄧ浉鍏?鍙岄噸璁块棶鎷掔粷鏃ュ織鐨勯閫夊畨鍏ㄦ柟娉曘€?
鍦?Linux v5.9 涔嬪墠锛屼娇鐢?perf_events 绯荤粺璋冪敤鐨勯潪鐗规潈杩涚▼杩橀』鎺ュ彈
PTRACE_MODE_READ_REALCREDS ptrace 璁块棶妯″紡妫€鏌?[^7^]_ 锛屽叾缁撴灉鍐冲畾鏄惁鍏佽鐩戞帶銆?鍥犳锛屾彁渚涗簡 CAP_SYS_PTRACE capability 鐨勯潪鐗规潈杩涚▼瀹為檯涓婅兘澶熼€氳繃璇ユ鏌ャ€備粠
Linux v5.9 璧凤紝涓嶅啀闇€瑕?CAP_SYS_PTRACE capability锛屽彧瑕佷负杩涚▼鎻愪緵 CAP_PERFMON
灏辫冻浠ヨ繘琛屾€ц兘鐩戞帶涓庡彲瑙傛祴鎬ф搷浣溿€?
鎺堜簣闈炵壒鏉冭繘绋嬬殑鍏朵粬 capability 鍙互鏈夋晥鍚敤瀵瑰悗缁鐩戞帶杩涚▼鎴栫郴缁熸€ц兘鍒嗘瀽鎵€闇€
棰濆鏁版嵁鐨勯噰闆嗐€備緥濡傦紝CAP_SYSLOG capability 鍏佽浠?/proc/kallsyms 鏂囦欢璇诲彇
鍐呮牳鎬佸唴瀛樺湴鍧€銆?
### 鐗规潈 Perf 鐢ㄦ埛缁?

capabilities 鏈哄埗銆佺壒鏉?capability-dumb 鏂囦欢 [^6^]_ 銆佹枃浠剁郴缁?ACL [^10^]_ 浠ュ強
sudo [^15^]_ 瀹炵敤绋嬪簭鍙敤鏉ュ垱寤轰笓鐢ㄧ殑鐗规潈 Perf 鐢ㄦ埛缁勶紝杩欎簺鐢ㄦ埛琚厑璁告棤闄愬埗鍦?鎵ц鎬ц兘鐩戞帶涓庡彲瑙傛祴鎬с€傚彲浠ラ噰鍙栦互涓嬫楠ゆ潵鍒涘缓杩欐牱鐨勭壒鏉?Perf 鐢ㄦ埛缁勩€?
1. 鍒涘缓鐗规潈 Perf 鐢ㄦ埛缁?perf_users锛屽皢 perf_users 缁勫垎閰嶇粰 Perf 宸ュ叿鍙墽琛屾枃浠讹紝
   骞堕檺鍒剁郴缁熶腑涓嶅湪 perf_users 缁勫唴鐨勫叾浠栫敤鎴疯闂鍙墽琛屾枃浠讹細

```
   # groupadd perf_users
   # ls -alhF
   -rwxr-xr-x  2 root root  11M Oct 19 15:12 perf
   # chgrp perf_users perf
   # ls -alhF
   -rwxr-xr-x  2 root perf_users  11M Oct 19 15:12 perf
   # chmod o-rwx perf
   # ls -alhF
   -rwxr-x---  2 root perf_users  11M Oct 19 15:12 perf
```

2. 涓?Perf 宸ュ叿鍙墽琛屾枃浠跺垎閰嶆墍闇€鐨?capabilities锛屽苟浣?perf_users 缁勬垚鍛樺叿澶?   鐩戞帶涓庡彲瑙傛祴鎬х壒鏉?[^6^]_ 锛?
```
   # setcap "cap_perfmon,cap_sys_ptrace,cap_syslog=ep" perf
   # setcap -v "cap_perfmon,cap_sys_ptrace,cap_syslog=ep" perf
   perf: OK
   # getcap perf
   perf = cap_sys_ptrace,cap_syslog,cap_perfmon+ep
```

濡傛灉瀹夎鐨?libcap [^16^]_ 灏氫笉鏀寔 "cap_perfmon"锛屽垯鏀圭敤 "38"锛屽嵆锛?
```
   # setcap "38,cap_ipc_lock,cap_sys_ptrace,cap_syslog=ep" perf
```

娉ㄦ剰锛屽浜?'perf top' 杩欑被宸ュ叿锛屼綘鍙兘闇€瑕佸湪缁勫悎涓姞鍏?'cap_ipc_lock'锛屾垨鑰?鏀圭敤 'perf top -m N' 浠ュ噺灏戝叾鐢ㄤ簬 perf 鐜舰缂撳啿鍖虹殑鍐呭瓨锛岃瑙佷笅鏂囩殑鈥滃唴瀛樺垎閰嶁€?涓€鑺傘€?
浣跨敤涓嶆敮鎸?CAP_PERFMON 鐨?libcap 浼氬鑷?cap_get_flag(caps, 38, CAP_EFFECTIVE,
&val) 澶辫触锛岃繘鑰屼娇榛樿浜嬩欢鍙樹负 'cycles:u'锛屽洜姝や綔涓哄彉閫氾紝璇锋樉寮忚姹?'cycles'
浜嬩欢锛屽嵆锛?
```
  # perf top -e cycles
```

浠ヤ究浠呭甫 CAP_PERFMON 鐨?perf 浜岃繘鍒朵篃鑳借幏寰楀唴鏍镐笌鐢ㄦ埛鏍锋湰銆?
杩欐牱涓€鏉ワ紝perf_users 缁勬垚鍛樹究鑳藉浣跨敤鎵€閰嶇疆 Perf 宸ュ叿鍙墽琛屾枃浠剁殑鍔熻兘杩涜鎬ц兘
鐩戞帶涓庡彲瑙傛祴鎬э紝璇ュ彲鎵ц鏂囦欢鍦ㄦ墽琛屾椂浼氶€氳繃 perf_events 瀛愮郴缁熺殑鑼冨洿妫€鏌ャ€?
濡傛灉鏃犳硶涓?Perf 宸ュ叿鍙墽琛屾枃浠跺垎閰嶆墍闇€鐨?capabilities锛堜緥濡傛枃浠剁郴缁熶互 nosuid
閫夐」鎸傝浇锛屾垨鏂囦欢绯荤粺涓嶆敮鎸佹墿灞曞睘鎬э級锛岄偅涔堝彲浠ュ垱寤?capabilities 鐗规潈鐜锛堣嚜鐒?灏辨槸 shell锛夈€傝 shell 涓哄唴閮ㄨ繘绋嬫彁渚?CAP_PERFMON 鍙婂叾浠栨墍闇€ capabilities锛屼粠鑰?鍦ㄨ鐜涓棤闄愬埗鍦拌繘琛屾€ц兘鐩戞帶涓庡彲瑙傛祴鎬ф搷浣溿€備粎 perf_users 缁勬垚鍛樺彲閫氳繃 sudo
瀹炵敤绋嬪簭杩涘叆璇ョ幆澧冦€備负鍒涘缓杩欐牱鐨勭幆澧冿細

1. 鍒涘缓浣跨敤 capsh 瀹炵敤绋嬪簭 [^16^]_ 鐨?shell 鑴氭湰锛屽皢 CAP_PERFMON 鍙婂叾浠栨墍闇€
   capabilities 鏀惧叆 shell 杩涚▼鐨勭幆澧?capability 闆嗭紙ambient capability set锛夛紝
   鍦ㄥ惎鐢?SECBIT_NO_SETUID_FIXUP銆丼ECBIT_NOROOT 涓?SECBIT_NO_CAP_AMBIENT_RAISE
   浣嶅悗閿佸畾杩涚▼瀹夊叏浣嶏紝鐒跺悗灏嗚繘绋嬭韩浠藉垏鎹负璇ヨ剼鏈殑 sudo 璋冪敤鑰咃紙鍏舵湰璐ㄤ笂搴斾负
   perf_users 缁勬垚鍛橈級锛?
```
   # ls -alh /usr/local/bin/perf.shell
   -rwxr-xr-x. 1 root root 83 Oct 13 23:57 /usr/local/bin/perf.shell
   # cat /usr/local/bin/perf.shell
   exec /usr/sbin/capsh --iab=^cap_perfmon --secbits=239 --user=$SUDO_USER -- -l
```

2. 鍦?/etc/sudoers 鏂囦欢涓负 perf_users 缁勬墿灞?sudo 绛栫暐锛?
```
   # grep perf_users /etc/sudoers
   %perf_users    ALL=/usr/local/bin/perf.shell
```

3. 妫€鏌?perf_users 缁勬垚鍛樻槸鍚﹁兘澶熻闂鐗规潈 shell锛屽苟鍦ㄥ唴閮ㄨ繘绋嬬殑鍏佽锛坧ermitted锛夈€?   鏈夋晥锛坋ffective锛変笌鐜锛坅mbient锛塩apability 闆嗕腑鍚敤浜?CAP_PERFMON 鍙婂叾浠?   鎵€闇€ capabilities锛?
```
  $ id
  uid=1003(capsh_test) gid=1004(capsh_test) groups=1004(capsh_test),1000(perf_users) context=unconfined_u:unconfined_r:unconfined_t:s0-s0:c0.c1023
  $ sudo perf.shell
  [sudo] password for capsh_test:
  $ grep Cap /proc/self/status
  CapInh:        0000004000000000
  CapPrm:        0000004000000000
  CapEff:        0000004000000000
  CapBnd:        000000ffffffffff
  CapAmb:        0000004000000000
  $ capsh --decode=0000004000000000
  0x0000004000000000=cap_perfmon
```

杩欐牱涓€鏉ワ紝perf_users 缁勬垚鍛樺氨鑳借闂鐗规潈鐜锛屽湪鍏朵腑浣跨敤鍙?CAP_PERFMON Linux
capability 绠℃帶鐨勬€ц兘鐩戞帶 API 鐨勫伐鍏枫€?
杩欑鐗瑰畾鐨勮闂帶鍒剁鐞嗕粎瀵逛互 CAP_SETPCAP銆丆AP_SETFCAP [^6^]_ capabilities 杩愯鐨?瓒呯骇鐢ㄦ埛鎴?root 杩涚▼鍙敤銆?
### 闈炵壒鏉冪敤鎴?

闈炵壒鏉冭繘绋嬬殑 perf_events **鑼冨洿锛坰cope锛?* 涓?**璁块棶锛坅ccess锛?* 鎺у埗鍙?perf_event_paranoid [^2^]_ 璁剧疆绠℃帶锛?
-1:
     瀵?perf_events 鎬ц兘鐩戞帶涓嶆柦鍔犱换浣?**鑼冨洿** 涓?**璁块棶** 闄愬埗銆傚湪涓哄瓨鍌?     鎬ц兘鏁版嵁鍒嗛厤鍐呭瓨缂撳啿鍖烘椂锛屽拷鐣ユ瘡鐢ㄦ埛姣?CPU 鐨?perf_event_mlock_kb [^2^]_
     閿佸畾闄愬埗銆傝繖鏄渶涓嶅畨鍏ㄧ殑妯″紡锛屽洜涓哄厑璁哥殑鐩戞帶 **鑼冨洿** 琚渶澶у寲锛屼笖瀵圭敤浜?     鎬ц兘鐩戞帶鐨?**璧勬簮** 涓嶆柦鍔犱换浣?perf_events 鐗瑰畾鐨勯檺鍒躲€?
>=0:
     **鑼冨洿** 鍖呭惈姣忚繘绋嬩笌绯荤粺鑼冨洿鐨勬€ц兘鐩戞帶锛屼絾鎺掗櫎鍘熷 tracepoint 涓?ftrace
     鍑芥暟 tracepoint 鐩戞帶銆傚湪鐢ㄦ埛鎬佹垨鍐呮牳鎬佹墽琛屾椂鍙戠敓鐨?CPU 涓庣郴缁熶簨浠堕兘鍙互
     琚洃鎺у拰鎹曡幏浠ヤ緵鍚庣画鍒嗘瀽銆備細鏂藉姞姣忕敤鎴锋瘡 CPU 鐨?perf_event_mlock_kb 閿佸畾
     闄愬埗锛屼絾鎷ユ湁 CAP_IPC_LOCK [^6^]_ capability 鐨勯潪鐗规潈杩涚▼浼氬拷鐣ヨ闄愬埗銆?
>=1:
     **鑼冨洿** 浠呭寘鍚瘡杩涚▼鎬ц兘鐩戞帶锛屾帓闄ょ郴缁熻寖鍥寸殑鎬ц兘鐩戞帶銆傚湪鐢ㄦ埛鎬佹垨鍐呮牳鎬?     鎵ц鏃跺彂鐢熺殑 CPU 涓庣郴缁熶簨浠堕兘鍙互琚洃鎺у拰鎹曡幏浠ヤ緵鍚庣画鍒嗘瀽銆備細鏂藉姞姣忕敤鎴?     姣?CPU 鐨?perf_event_mlock_kb 閿佸畾闄愬埗锛屼絾鎷ユ湁 CAP_IPC_LOCK capability 鐨?     闈炵壒鏉冭繘绋嬩細蹇界暐璇ラ檺鍒躲€?
>=2:
     **鑼冨洿** 浠呭寘鍚瘡杩涚▼鎬ц兘鐩戞帶銆傚彧鏈夋墽琛屼簬鐢ㄦ埛鎬佹椂鍙戠敓鐨?CPU 涓庣郴缁熶簨浠?     鍙互琚洃鎺у拰鎹曡幏浠ヤ緵鍚庣画鍒嗘瀽銆備細鏂藉姞姣忕敤鎴锋瘡 CPU 鐨?perf_event_mlock_kb
     閿佸畾闄愬埗锛屼絾鎷ユ湁 CAP_IPC_LOCK capability 鐨勯潪鐗规潈杩涚▼浼氬拷鐣ヨ闄愬埗銆?
### 璧勬簮鎺у埗


鎵撳紑鐨勬枃浠舵弿杩扮
+++++++++++++++++++++

perf_events 绯荤粺璋冪敤 API [^2^]_ 涓烘瘡涓厤缃殑 PMU 浜嬩欢鍒嗛厤鏂囦欢鎻忚堪绗︺€傛墦寮€鐨?鏂囦欢鎻忚堪绗︽槸涓€椤规寜杩涚▼鏍哥畻鐨勮祫婧愶紝鍙?RLIMIT_NOFILE [^11^]_ 闄愬埗锛坲limit -n锛?绠℃帶锛岃闄愬埗閫氬父婧愯嚜鐧诲綍 shell 杩涚▼銆傚綋鍦ㄥぇ鍨嬫湇鍔″櫒绯荤粺涓婁负闀夸簨浠跺垪琛ㄩ厤缃?Perf
閲囬泦鏃讹紝寰堝鏄撹Е鍙婃闄愬埗锛屼粠鑰岄樆姝㈡墍闇€鐨勭洃鎺ч厤缃€俁LIMIT_NOFILE 闄愬埗鍙互鎸夌敤鎴?淇敼 limits.conf 鏂囦欢 [^12^]_ 鐨勫唴瀹规潵鎻愰珮銆傞€氬父锛屼竴娆?Perf 閲囨牱浼氳瘽锛坧erf
record锛夋墍闇€鐨勬墦寮€ perf_event 鏂囦欢鎻忚堪绗︽暟閲忎笉灏戜簬琚洃鎺т簨浠舵暟涔樹互琚洃鎺?CPU 鏁般€?
鍐呭瓨鍒嗛厤
+++++++++++++++++

鐢ㄦ埛杩涚▼鍙敤浜庢崟鑾锋€ц兘鐩戞帶鏁版嵁鐨勫唴瀛橀噺鍙?perf_event_mlock_kb [^2^]_ 璁剧疆绠℃帶銆?杩欎竴 perf_events 鐗瑰畾鐨勮祫婧愯缃畾涔変簡鍏佽鐢ㄦ埛杩涚▼涓轰簡鎵ц鎬ц兘鐩戞帶鑰岃繘琛屾槧灏勭殑
鏁翠綋姣?CPU 鍐呭瓨涓婇檺銆傝璁剧疆鏈川涓婃墿灞曚簡 RLIMIT_MEMLOCK [^11^]_ 闄愬埗锛屼絾浠呴拡瀵?涓撲负鎹曡幏琚洃鎺ф€ц兘浜嬩欢鍙婄浉鍏虫暟鎹€屾槧灏勭殑鍐呭瓨鍖哄煙銆?
渚嬪锛屽鏋滀竴鍙版満鍣ㄦ湁鍏釜鏍稿績锛屼笖 perf_event_mlock_kb 闄愬埗璁句负 516 KiB锛岄偅涔堢敤鎴?杩涚▼鍙幏寰楄秴鍑?RLIMIT_MEMLOCK 闄愬埗锛坲limit -l锛夌殑 516 KiB * 8 = 4128 KiB 鍐呭瓨鐢ㄤ簬
perf_event mmap 缂撳啿鍖恒€傜壒鍒湴锛岃繖鎰忓懗鐫€濡傛灉鐢ㄦ埛鎯冲惎鍔ㄤ袱涓垨鏇村鎬ц兘鐩戞帶杩涚▼锛?灏遍渶瑕佹墜鍔ㄥ湪鐩戞帶杩涚▼涔嬮棿鍒嗛厤鍙敤鐨?4128 KiB锛屼緥濡備娇鐢?Perf record 妯″紡閫夐」
--mmap-pages銆傚惁鍒欙紝绗竴涓惎鍔ㄧ殑鎬ц兘鐩戞帶杩涚▼浼氬垎閰嶆帀鍏ㄩ儴鍙敤鐨?4128 KiB锛岃€屽叾浠?杩涚▼灏嗗洜鍐呭瓨涓嶈冻鑰屾棤娉曠户缁€?
RLIMIT_MEMLOCK 涓?perf_event_mlock_kb 璧勬簮绾︽潫瀵规嫢鏈?CAP_IPC_LOCK capability 鐨?杩涚▼琚拷鐣ャ€傚洜姝わ紝閫氳繃涓?Perf 鍙墽琛屾枃浠舵彁渚?CAP_IPC_LOCK capability锛屽彲浠ヤ负
perf_events/Perf 鐗规潈鐢ㄦ埛鎻愪緵瓒呭嚭杩欎簺绾︽潫鐨勫唴瀛橈紝鐢ㄤ簬 perf_events/Perf 鎬ц兘鐩戞帶
鐩殑銆?
### 鍙傝€冩枃鐚?

