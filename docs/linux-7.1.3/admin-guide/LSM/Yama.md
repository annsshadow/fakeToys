## Yama


Yama 鏄竴涓?Linux 瀹夊叏妯″潡锛圠inux Security Module锛夛紝瀹冩敹闆嗘牳蹇冨唴鏍告湰韬湭澶勭悊鐨勩€佺郴缁熻寖鍥寸殑
DAC 瀹夊叏淇濇姢銆傚畠鍙互鍦ㄦ瀯寤烘椂閫氳繃 `CONFIG_SECURITY_YAMA` 閫夋嫨锛屽苟鍙湪杩愯鏃堕€氳繃
`/proc/sys/kernel/yama` 涓殑 sysctl 杩涜鎺у埗锛?
## ptrace_scope


闅忕潃 Linux 鏃ョ泭娴佽锛屽畠灏嗘垚涓烘伓鎰忚蒋浠舵洿澶х殑鐩爣銆侺inux 杩涚▼鎺ュ彛涓€涓壒鍒护浜烘媴蹇х殑寮辩偣鏄紝
鍗曚釜鐢ㄦ埛鍙互妫€鏌ュ叾浠讳綍杩涚▼鐨勫唴瀛樺拰杩愯鐘舵€併€備緥濡傦紝濡傛灉鏌愪釜搴旂敤绋嬪簭锛堝 Pidgin锛夎鏀荤牬锛屾敾鍑?鑰呭氨鍙互闄勫姞鍒板叾浠栨鍦ㄨ繍琛岀殑杩涚▼锛堝 Firefox銆丼SH 浼氳瘽銆丟PG agent 绛夛級锛屼互鎻愬彇棰濆鐨勫嚟鎹紝
骞跺湪涓嶅€熷姪鐢ㄦ埛鍗忓姪鐨勭綉缁滈挀楸肩殑鎯呭喌涓嬬户缁墿澶ф敾鍑昏寖鍥淬€?
杩欏苟闈炰竴涓悊璁洪棶棰樸€俙SSH session hijacking
<https://www.blackhat.com/presentations/bh-usa-05/bh-us-05-boileau.pdf>`_
鍜?`arbitrary code injection
<https://c-skills.blogspot.com/2007/05/injectso.html>`_
鏀诲嚮宸茬粡瀛樺湪锛屽苟涓斿鏋?ptrace 琚厑璁稿儚浠ュ墠涓€鏍疯繍琛岋紝瀹冧滑浠嶇劧鍙兘鍙戠敓銆傜敱浜?ptrace 寰堝皯琚潪
寮€鍙戣€呭拰闈炵鐞嗗憳浣跨敤锛屽簲鍏佽绯荤粺鏋勫缓鑰呴€夋嫨绂佺敤姝よ皟璇曠郴缁熴€?
浣滀负瑙ｅ喅鏂规锛屼竴浜涘簲鐢ㄧ▼搴忎娇鐢?`prctl(PR_SET_DUMPABLE, ...)` 涓撻棬绂佹姝ょ被 ptrace 闄勫姞
锛堜緥濡?ssh-agent锛夛紝浣嗚澶氬簲鐢ㄧ▼搴忔病鏈夎繖鏍峰仛銆備竴涓洿閫氱敤鐨勮В鍐虫柟妗堟槸鍙厑璁镐粠鐖惰繘绋嬬洿鎺ュ瀛?杩涚▼杩涜 ptrace锛堝嵆鐩存帴鐨?鈥済db EXE鈥?鍜?鈥渟trace EXE鈥?浠嶇劧鏈夋晥锛夛紝鎴栬€呴渶瑕?`CAP_SYS_PTRACE`
锛堝嵆 鈥済db --pid=PID鈥?鍜?鈥渟trace -p PID鈥?浣滀负 root 浠嶇劧鏈夋晥锛夈€?
鍦ㄦā寮?1 涓嬶紝瀹氫箟浜嗚皟璇曡繘绋嬩笌鍏朵笅绾э紙inferior锛変箣闂村簲鐢ㄧ壒瀹氬叧绯荤殑杞欢锛堝穿婧冨鐞嗙▼搴忕瓑锛夊彲浠?浣跨敤 `prctl(PR_SET_PTRACER, pid, ...)`銆備竴涓笅绾у彲浠ュ０鏄庡厑璁稿摢浜涘叾浠栬繘绋嬶紙鍙婂叾鍚庝唬锛夊鍏惰皟鐢?`PTRACE_ATTACH`銆傛瘡涓笅绾т竴娆″彧鑳藉瓨鍦ㄤ竴涓繖鏍风殑宸插０鏄庤皟璇曡繘绋嬨€備緥濡傦紝KDE銆丆hromium 鍜?Firefox
鐨勫穿婧冨鐞嗙▼搴忥紝浠ュ強 Wine锛堢敤浜庡彧鍏佽 Wine 杩涚▼涔嬮棿鐩镐簰 ptrace锛変娇鐢ㄤ簡瀹冦€傚鏋滀竴涓繘绋嬪笇鏈?瀹屽叏绂佺敤杩欎簺 ptrace 闄愬埗锛屽畠鍙互璋冪敤 `prctl(PR_SET_PTRACER, PR_SET_PTRACER_ANY, ...)`锛屼互渚?浠讳綍鍏朵粬鏈簲琚厑璁哥殑杩涚▼锛堝嵆浣挎槸澶栭儴 pid 鍛藉悕绌洪棿涓殑杩涚▼锛夐兘鍙互闄勫姞銆?
sysctl 璁剧疆锛堝彧鏈?`CAP_SYS_PTRACE` 鎵嶈兘鍐欏叆锛変负锛?
0 - 缁忓吀 ptrace 鏉冮檺锛?    涓€涓繘绋嬪彲浠?`PTRACE_ATTACH` 鍒颁换浣曞湪鐩稿悓 uid 涓嬭繍琛岀殑鍏朵粬杩涚▼锛屽彧瑕佸畠鏄彲杞偍鐨勶紙鍗?    娌℃湁鍒囨崲杩?uid銆佹病鏈変互鐗规潈鍚姩锛屾垨娌℃湁宸茬粡璋冪敤杩?`prctl(PR_SET_DUMPABLE...)`锛夈€傜被浼煎湴锛?    `PTRACE_TRACEME` 涓嶅彉銆?
1 - 鍙楅檺 ptrace锛?    涓€涓繘绋嬪繀椤讳笌鍏舵兂瑕佽皟鐢?`PTRACE_ATTACH` 鐨勪笅绾ф湁棰勫畾涔夌殑鍏崇郴銆傞粯璁ゆ儏鍐典笅锛岃繖绉嶅叧绯绘槸
    浠呭綋涓婅堪缁忓吀鏉′欢涔熸弧瓒虫椂鐨勫悗浠ｅ叧绯汇€傝鏇存敼鍏崇郴锛屼笅绾у彲浠ヨ皟鐢?    `prctl(PR_SET_PTRACER, debugger, ...)` 鏉ュ０鏄庝竴涓鍏佽鐨勮皟璇曞櫒 PID 瀵硅涓嬬骇璋冪敤
    `PTRACE_ATTACH`銆備娇鐢?`PTRACE_TRACEME` 涓嶅彉銆?
2 - 浠呯鐞嗗憳闄勫姞锛?    鍙湁甯︽湁 `CAP_SYS_PTRACE` 鐨勮繘绋嬪彲浠ヤ娇鐢?ptrace锛屾棤璁烘槸閫氳繃 `PTRACE_ATTACH` 杩樻槸閫氳繃
    瀛愯繘绋嬭皟鐢?`PTRACE_TRACEME`銆?
3 - 绂佹闄勫姞锛?    娌℃湁浠讳綍杩涚▼鍙互浣跨敤 `PTRACE_ATTACH` 鎴栭€氳繃 `PTRACE_TRACEME` 浣跨敤 ptrace銆備竴鏃﹁缃紝姝?    sysctl 鍊兼棤娉曟洿鏀广€?
鏈€鍒濈殑浠呭瓙杩涚▼閫昏緫鍩轰簬 grsecurity 涓殑闄愬埗銆?