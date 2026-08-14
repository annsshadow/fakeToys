## ETMv4 sysfs Linux 椹卞姩缂栫▼鍙傝€冦€?


    :Author:   Mike Leach <mike.leach@linaro.org>
    :Date:     October 11th, 2019

浣滀负鐜版湁 ETMv4 椹卞姩鏂囨。鐨勮ˉ鍏呫€?

### Sysfs 鏂囦欢涓庣洰褰?


Root: `/sys/bus/coresight/devices/etm<N>`


浠ヤ笅娈佃惤璇存槑浜?sysfs 鏂囦欢涓庡畠浠墍褰卞搷鐨?ETMv4 瀵勫瓨鍣ㄤ箣闂寸殑鍏宠仈銆傛敞鎰忓瘎瀛樺櫒鍚嶇О浠ョ渷鐣?鈥楾RC鈥?鍓嶇紑鐨勫舰寮忕粰鍑恒€?

----

:File:            `mode` (rw)
:Trace Registers: {CONFIGR + others}
:Notes:
    浣嶉€夋嫨璺熻釜鐗规€с€傚弬瑙佷笅鏂囩殑 鈥榤ode鈥?灏忚妭銆傝缃叾涓殑浣嶅皢瀵艰嚧瀵硅窡韪厤缃瘎瀛樺櫒
    鍙婂叾浠栧瘎瀛樺櫒杩涜绛変环鐨勭紪绋嬶紝浠ュ惎鐢ㄦ墍璇锋眰鐨勭壒鎬с€?

:Syntax & eg:
    `echo bitfield > mode`

    bitfield 鏈€澶?32 浣嶏紝鐢ㄤ簬璁剧疆璺熻釜鐗规€с€?

:Example:
    `$> echo 0x012 > mode`

----

:File:            `reset` (wo)
:Trace Registers: All
:Notes:
    灏嗘墍鏈夌紪绋嬪浣嶄负涓嶄骇鐢熶换浣曡窡韪?/ 鏈紪绋嬩换浣曢€昏緫銆?

:Syntax:
    `echo 1 > reset`

----

:File:            `enable_source` (wo)
:Trace Registers: PRGCTLR, All hardware regs.
:Notes:
    - > 0 : 浣跨敤椹卞姩涓繚瀛樼殑褰撳墠鍊煎纭欢杩涜缂栫▼骞跺惎鐢ㄨ窡韪€?

    - = 0 : 绂佺敤璺熻釜纭欢銆?

:Syntax:
    `echo 1 > enable_source`

----

:File:            `cpu` (ro)
:Trace Registers: None.
:Notes:
    姝?ETM 鎵€杩炴帴鐨?CPU ID銆?

:Example:
    `$> cat cpu`

    `$> 0`

----

:File:            `ts_source` (ro)
:Trace Registers: None.
:Notes:
    褰撳疄鐜颁簡 FEAT_TRF 鏃讹紝涓鸿窡韪細璇濇墍鐢?TRFCR_ELx.TS 鐨勫€笺€傚惁鍒?-1
    琛ㄧず鏈煡鐨勬椂闂存簮銆傛鏌?trcidr0.tssize 浠ユ煡鐪嬫槸鍚﹀瓨鍦ㄥ叏灞€鏃堕棿鎴炽€?

:Example:
    `$> cat ts_source`

    `$> 1`

----

:File:            `addr_idx` (rw)
:Trace Registers: None.
:Notes:
    鐢ㄤ簬绱㈠紩鍦板潃姣旇緝鍣ㄥ拰鑼冨洿鐗规€х殑铏氭嫙瀵勫瓨鍣ㄣ€備负鑼冨洿涓殑涓€瀵规瘮杈冨櫒
    璁剧疆绗竴涓殑绱㈠紩銆?

:Syntax:
    `echo idx > addr_idx`

    鍏朵腑 idx < nr_addr_cmp x 2

----

:File:            `addr_range` (rw)
:Trace Registers: ACVR[idx, idx+1], VIIECTLR
:Notes:
    鐢?addr_idx 閫夋嫨鐨勬煇涓寖鍥村搴旂殑鍦板潃瀵广€傛牴鎹彲閫夊弬鏁拌繘琛屽寘鍚?/ 鎺掗櫎锛?
    鑻ョ渷鐣ュ垯浣跨敤褰撳墠 鈥榤ode鈥?璁剧疆銆傚湪鎺у埗瀵勫瓨鍣ㄤ腑閫夋嫨姣旇緝鍣ㄨ寖鍥淬€?
    绱㈠紩涓哄鏁版椂鎶ラ敊銆?

:Depends: `mode, addr_idx`
:Syntax:
   `echo addr1 addr2 [exclude] > addr_range`

   鍏朵腑 addr1 涓?addr2 鐣屽畾璇ヨ寖鍥达紝涓?addr1 < addr2銆?

   Optional exclude value:-

   - 0 for include
   - 1 for exclude.
:Example:
   `$> echo 0x0000 0x2000 0 > addr_range`

----

:File:            `addr_single` (rw)
:Trace Registers: ACVR[idx]
:Notes:
    鏍规嵁 addr_idx 璁剧疆涓€涓嫭绔嬬殑鍦板潃姣旇緝鍣ㄣ€傚綋璇ュ湴鍧€姣旇緝鍣ㄧ敤浣滀簨浠?
    鐢熸垚閫昏緫绛夌殑涓€閮ㄥ垎鏃朵娇鐢ㄣ€?

:Depends: `addr_idx`
:Syntax:
   `echo addr1 > addr_single`

----

:File:           `addr_start` (rw)
:Trace Registers: ACVR[idx], VISSCTLR
:Notes:
    鏍规嵁 addr_idx 璁剧疆璺熻釜璧峰鍦板潃姣旇緝鍣ㄣ€傚湪鎺у埗瀵勫瓨鍣ㄤ腑閫夋嫨姣旇緝鍣ㄣ€?

:Depends: `addr_idx`
:Syntax:
    `echo addr1 > addr_start`

----

:File:            `addr_stop` (rw)
:Trace Registers: ACVR[idx], VISSCTLR
:Notes:
    鏍规嵁 addr_idx 璁剧疆璺熻釜鍋滄鍦板潃姣旇緝鍣ㄣ€傚湪鎺у埗瀵勫瓨鍣ㄤ腑閫夋嫨姣旇緝鍣ㄣ€?

:Depends: `addr_idx`
:Syntax:
    `echo addr1 > addr_stop`

----

:File:            `addr_context` (rw)
:Trace Registers: ACATR[idx,{6:4}]
:Notes:
    灏嗕笂涓嬫枃 ID 姣旇緝鍣ㄩ摼鎺ュ埌鍦板潃姣旇緝鍣?addr_idx

:Depends: `addr_idx`
:Syntax:
    `echo ctxt_idx > addr_context`

    鍏朵腑 ctxt_idx 涓烘墍閾炬帴鐨勪笂涓嬫枃 id / vmid 姣旇緝鍣ㄧ殑绱㈠紩銆?

----

:File:            `addr_ctxtype` (rw)
:Trace Registers: ACATR[idx,{3:2}]
:Notes:
    杈撳叆鍊煎瓧绗︿覆銆備负鎵€閾炬帴鐨勪笂涓嬫枃 ID 姣旇緝鍣ㄨ缃被鍨?

:Depends: `addr_idx`
:Syntax:
    `echo type > addr_ctxtype`

    绫诲瀷涓?{all, vmid, ctxid, none} 涔嬩竴
:Example:
    `$> echo ctxid > addr_ctxtype`

----

:File:            `addr_exlevel_s_ns` (rw)
:Trace Registers: ACATR[idx,{14:8}]
:Notes:
    涓烘墍閫夊湴鍧€姣旇緝鍣ㄨ缃?ELx 瀹夊叏涓庨潪瀹夊叏鍖归厤浣?

:Depends: `addr_idx`
:Syntax:
    `echo val > addr_exlevel_s_ns`

    val 涓虹敤浜庢帓闄ょ殑寮傚父绾у埆瀵瑰簲鐨?7 浣嶅€笺€傝緭鍏ュ€煎湪瀵勫瓨鍣ㄤ腑琚Щ浣嶅埌姝ｇ‘鐨勪綅銆?
:Example:
    `$> echo 0x4F > addr_exlevel_s_ns`

----

:File:            `addr_instdatatype` (rw)
:Trace Registers: ACATR[idx,{1:0}]
:Notes:
    璁剧疆鐢ㄤ簬鍖归厤鐨勫湴鍧€姣旇緝鍣ㄧ被鍨嬨€傞┍鍔ㄤ粎鏀寔璁剧疆涓烘寚浠ゅ湴鍧€绫诲瀷銆?

:Depends: `addr_idx`

----

:File:            `addr_cmp_view` (ro)
:Trace Registers: ACVR[idx, idx+1], ACATR[idx], VIIECTLR
:Notes:
    璇诲彇褰撳墠閫変腑鐨勫湴鍧€姣旇緝鍣ㄣ€傚鏋滃睘浜庢煇涓湴鍧€鑼冨洿锛屽垯鏄剧ず涓や釜鍦板潃銆?

:Depends: `addr_idx`
:Syntax:
    `cat addr_cmp_view`
:Example:
    `$> cat addr_cmp_view`

   `addr_cmp[^0^] range 0x0 0xffffffffffffffff include ctrl(0x4b00)`

----

:File:            `nr_addr_cmp` (ro)
:Trace Registers: From IDR4
:Notes:
    鍦板潃姣旇緝鍣ㄥ鐨勬暟閲?

----

:File:            `sshot_idx` (rw)
:Trace Registers: None
:Notes:
    閫夋嫨鍗曟瑙﹀彂锛坰ingle shot锛夊瘎瀛樺櫒缁勩€?

----

:File:            `sshot_ctrl` (rw)
:Trace Registers: SSCCR[idx]
:Notes:
    璁块棶鍗曟瑙﹀彂姣旇緝鍣ㄦ帶鍒跺瘎瀛樺櫒銆?

:Depends: `sshot_idx`
:Syntax:
    `echo val > sshot_ctrl`

    灏?val 鍐欏叆鎵€閫夋帶鍒跺瘎瀛樺櫒銆?

----

:File:            `sshot_status` (ro)
:Trace Registers: SSCSR[idx]
:Notes:
    璇诲彇鍗曟瑙﹀彂姣旇緝鍣ㄧ姸鎬佸瘎瀛樺櫒

:Depends: `sshot_idx`
:Syntax:
    `cat sshot_status`

    璇诲彇鐘舵€併€?
:Example:
    `$> cat sshot_status`

    `0x1`

----

:File:            `sshot_pe_ctrl` (rw)
:Trace Registers: SSPCICR[idx]
:Notes:
    璁块棶鍗曟瑙﹀彂 PE 姣旇緝鍣ㄨ緭鍏ユ帶鍒跺瘎瀛樺櫒銆?

:Depends: `sshot_idx`
:Syntax:
    `echo val > sshot_pe_ctrl`

    灏?val 鍐欏叆鎵€閫夋帶鍒跺瘎瀛樺櫒銆?

----

:File:            `ns_exlevel_vinst` (rw)
:Trace Registers: VICTLR{23:20}
:Notes:
    瀵瑰畨鍏ㄥ紓甯哥骇鍒繃婊ゅ櫒杩涜缂栫▼銆傝缃?/ 娓呴櫎 NS
    寮傚父杩囨护鍣ㄤ綅銆傝缃?鈥?鈥?灏嗘帓闄よ寮傚父绾у埆鐨勮窡韪€?

:Syntax:
    `echo bitfield > ns_exlevel_viinst`

    鍏朵腑 bitfield 鍖呭惈鐢ㄤ簬璁剧疆 / 娓呴櫎 EL0 鍒?EL2 鐨勪綅
:Example:
    `%> echo 0x4 > ns_exlevel_viinst`

    Excludes EL2 NS trace.

----

:File:            `vinst_pe_cmp_start_stop` (rw)
:Trace Registers: VIPCSSCTLR
:Notes:
    璁块棶 PE 鍚仠姣旇緝鍣ㄨ緭鍏ユ帶鍒跺瘎瀛樺櫒

----

:File:            `bb_ctrl` (rw)
:Trace Registers: BBCTLR
:Notes:
    瀹氫箟鍒嗘敮骞挎挱锛圔ranch Broadcast锛夋墍浣滅敤鐨勮寖鍥淬€?
    榛樿鍊?(0x0) 涓哄叏閮ㄥ湴鍧€銆?

:Depends: BB enabled.

----

:File:            `cyc_threshold` (rw)
:Trace Registers: CCCTLR
:Notes:
    璁剧疆灏嗗彂鍑虹殑鍛ㄦ湡璁℃暟闃堝€笺€傝嫢灏濊瘯璁剧疆涓轰綆浜?IDR3 涓畾涔夌殑鏈€灏忓€煎垯鎶ラ敊锛?
    骞舵寜鏈夋晥浣嶅搴﹁繘琛屾帺鐮併€?

:Depends: CC enabled.

----

:File:            `syncfreq` (rw)
:Trace Registers: SYNCPR
:Notes:
    璁剧疆璺熻釜鍚屾鍛ㄦ湡銆傚€间负 2 鐨勫箓锛屽彲涓?0锛堝叧闂級鎴?8-20銆傞┍鍔ㄩ粯璁ゅ€间负 12锛堟瘡 4096 瀛楄妭锛夈€?

----

:File:            `cntr_idx` (rw)
:Trace Registers: none
:Notes:
    閫夋嫨瑕佽闂殑璁℃暟鍣?

:Syntax:
    `echo idx > cntr_idx`

    鍏朵腑 idx < nr_cntr

----

:File:            `cntr_ctrl` (rw)
:Trace Registers: CNTCTLR[idx]
:Notes:
    璁剧疆璁℃暟鍣ㄦ帶鍒跺€笺€?

:Depends: `cntr_idx`
:Syntax:
    `echo val > cntr_ctrl`

    鍏朵腑 val 渚濇嵁 ETMv4 瑙勮寖銆?

----

:File:            `cntrldvr` (rw)
:Trace Registers: CNTRLDVR[idx]
:Notes:
    璁剧疆璁℃暟鍣ㄩ噸瑁呰浇鍊笺€?

:Depends: `cntr_idx`
:Syntax:
    `echo val > cntrldvr`

    鍏朵腑 val 渚濇嵁 ETMv4 瑙勮寖銆?

----

:File:            `nr_cntr` (ro)
:Trace Registers: From IDR5

:Notes:
    宸插疄鐜扮殑璁℃暟鍣ㄦ暟閲忋€?

----

:File:            `ctxid_idx` (rw)
:Trace Registers: None
:Notes:
    閫夋嫨瑕佽闂殑涓婁笅鏂?ID 姣旇緝鍣?

:Syntax:
    `echo idx > ctxid_idx`

    鍏朵腑 idx < numcidc

----

:File:            `ctxid_pid` (rw)
:Trace Registers: CIDCVR[idx]
:Notes:
   璁剧疆涓婁笅鏂?ID 姣旇緝鍣ㄥ€?

:Depends: `ctxid_idx`

----

:File: `ctxid_masks` (rw)
:Trace Registers: CIDCCTLR0, CIDCCTLR1, CIDCVR<0-7>
:Notes:
    鐢ㄤ簬璁剧疆 1-8 涓笂涓嬫枃 ID 姣旇緝鍣ㄥ瓧鑺傛帺鐮佺殑鍊煎銆備細鍦?CID
    鍊煎瘎瀛樺櫒涓嚜鍔ㄥ皢鎺╃爜瀛楄妭娓呴浂銆?

:Syntax:
    `echo m3m2m1m0 [m7m6m5m4] > ctxid_masks`

    32 浣嶅€肩敱鎺╃爜瀛楄妭缁勬垚锛屽叾涓?mN 琛ㄧず涓婁笅鏂?ID 姣旇緝鍣?N 鐨?
    瀛楄妭鎺╃爜鍊笺€?

    鍦ㄤ笂涓嬫枃 ID 姣旇緝鍣ㄥ皯浜?4 涓殑绯荤粺涓婁笉闇€瑕佺浜屼釜鍊?

----

:File:            `numcidc` (ro)
:Trace Registers: From IDR4
:Notes:
    涓婁笅鏂?ID 姣旇緝鍣ㄧ殑鏁伴噺

----

:File:            `vmid_idx` (rw)
:Trace Registers: None
:Notes:
    閫夋嫨瑕佽闂殑 VM ID 姣旇緝鍣ㄣ€?

:Syntax:
    `echo idx > vmid_idx`

    鍏朵腑 idx < numvmidc

----

:File:            `vmid_val` (rw)
:Trace Registers: VMIDCVR[idx]
:Notes:
    璁剧疆 VM ID 姣旇緝鍣ㄥ€?

:Depends: `vmid_idx`

----

:File:            `vmid_masks` (rw)
:Trace Registers: VMIDCCTLR0, VMIDCCTLR1, VMIDCVR<0-7>
:Notes:
    鐢ㄤ簬璁剧疆 1-8 涓?VM ID 姣旇緝鍣ㄥ瓧鑺傛帺鐮佺殑鍊煎銆備細鍦?VMID 鍊煎瘎瀛樺櫒涓?
    鑷姩灏嗘帺鐮佸瓧鑺傛竻闆躲€?

:Syntax:
    `echo m3m2m1m0 [m7m6m5m4] > vmid_masks`

    鍏朵腑 mN 琛ㄧず VMID 姣旇緝鍣?N 鐨勫瓧鑺傛帺鐮佸€笺€傚湪 VMID 姣旇緝鍣ㄥ皯浜?4 涓殑绯荤粺涓婁笉闇€瑕佺浜屼釜鍊笺€?

----

:File:            `numvmidc` (ro)
:Trace Registers: From IDR4
:Notes:
    VMID 姣旇緝鍣ㄧ殑鏁伴噺

----

:File:            `res_idx` (rw)
:Trace Registers: None.
:Notes:
    閫夋嫨瑕佽闂殑璧勬簮閫夋嫨鍣ㄦ帶鍒躲€傚繀椤讳负 2 鎴栨洿楂橈紝鍥犱负閫夋嫨鍣?0 鍜?1 鏄‖杩炵嚎鐨勩€?

:Syntax:
    `echo idx > res_idx`

    鍏朵腑 2 <= idx < nr_resource x 2

----

:File:            `res_ctrl` (rw)
:Trace Registers: RSCTLR[idx]
:Notes:
    璁剧疆璧勬簮閫夋嫨鍣ㄦ帶鍒跺€笺€傚彇鍊奸伒寰?ETMv4 瑙勮寖銆?

:Depends: `res_idx`
:Syntax:
    `echo val > res_cntr`

    鍏朵腑 val 渚濇嵁 ETMv4 瑙勮寖銆?

----

:File:            `nr_resource` (ro)
:Trace Registers: From IDR4
:Notes:
    璧勬簮閫夋嫨鍣ㄥ鐨勬暟閲?

----

:File:            `event` (rw)
:Trace Registers: EVENTCTRL0R
:Notes:
    璁剧疆鏈€澶?4 涓凡瀹炵幇鐨勪簨浠跺瓧娈点€?

:Syntax:
    `echo ev3ev2ev1ev0 > event`

    鍏朵腑 evN 涓轰竴涓?8 浣嶄簨浠跺瓧娈点€傛渶澶?4 涓簨浠跺瓧娈电粍鎴?32 浣嶈緭鍏ュ€笺€傛湁鏁堝瓧娈电殑鏁伴噺鍙栧喅浜庡叿浣撳疄鐜帮紝鐢?IDR0 瀹氫箟銆?

----

:File: `event_instren` (rw)
:Trace Registers: EVENTCTRL1R
:Notes:
    閫夋嫨灏嗕簨浠跺寘鎻掑叆璺熻釜娴佺殑浜嬩欢銆?

:Depends: EVENTCTRL0R
:Syntax:
    `echo bitfield > event_instren`

    鍏朵腑 bitfield 鏍规嵁浜嬩欢瀛楁鐨勬暟閲忔渶澶氫负 4 浣嶃€?

----

:File:            `event_ts` (rw)
:Trace Registers: TSCTLR
:Notes:
    璁剧疆灏嗙敓鎴愭椂闂存埑璇锋眰鐨勪簨浠躲€?

:Depends: `TS activated`
:Syntax:
    `echo evfield > event_ts`

    鍏朵腑 evfield 涓轰竴涓?8 浣嶄簨浠堕€夋嫨鍣ㄣ€?

----

:File:            `seq_idx` (rw)
:Trace Registers: None
:Notes:
    搴忓垪鍣ㄤ簨浠跺瘎瀛樺櫒閫夋嫨 - 0 鍒?2

----

:File:            `seq_state` (rw)
:Trace Registers: SEQSTR
:Notes:
    搴忓垪鍣ㄥ綋鍓嶇姸鎬?- 0 鍒?3銆?

----

:File:            `seq_event` (rw)
:Trace Registers: SEQEVR[idx]
:Notes:
    鐘舵€佽浆绉讳簨浠跺瘎瀛樺櫒

:Depends: `seq_idx`
:Syntax:
    `echo evBevF > seq_event`

    鍏朵腑 evBevF 鏄竴涓敱涓や綅浜嬩欢閫夋嫨鍣ㄧ粍鎴愮殑 16 浣嶅€硷細

    - evB : 鍚戝悗锛坆ack锛?
    - evF : 鍚戝墠锛坒orwards锛?

----

:File:            `seq_reset_event` (rw)
:Trace Registers: SEQRSTEVR
:Notes:
    搴忓垪鍣ㄥ浣嶄簨浠?

:Syntax:
    `echo evfield > seq_reset_event`

    鍏朵腑 evfield 涓轰竴涓?8 浣嶄簨浠堕€夋嫨鍣ㄣ€?

----

:File:            `nrseqstate` (ro)
:Trace Registers: From IDR5
:Notes:
    搴忓垪鍣ㄧ姸鎬佹暟閲忥紙0 鎴?4锛?

----

:File:            `nr_pe_cmp` (ro)
:Trace Registers: From IDR4
:Notes:
    PE 姣旇緝鍣ㄨ緭鍏ョ殑鏁伴噺

----

:File:            `nr_ext_inp` (ro)
:Trace Registers: From IDR5
:Notes:
    澶栭儴杈撳叆鐨勬暟閲?

----

:File:            `nr_ss_cmp` (ro)
:Trace Registers: From IDR4
:Notes:
    鍗曟瑙﹀彂鎺у埗瀵勫瓨鍣ㄧ殑鏁伴噺

----

**娉ㄦ剰锛?* 鍦ㄥ浠绘剰鍦板潃姣旇緝鍣ㄨ繘琛岀紪绋嬫椂锛岄┍鍔ㄤ細涓鸿姣旇緝鍣ㄦ墦涓婁娇鐢ㄧ被鍨嬬殑鏍囪 鈥斺€?鍗?RANGE銆丼INGLE銆丼TART銆丼TOP銆備竴鏃﹁缃簡璇ユ爣璁帮紝鍒欏彧鑳戒娇鐢ㄥ鍏惰繘琛岀紪绋嬬殑鍚屼竴涓?sysfs 鏂囦欢 / 绫诲瀷鏉ヤ慨鏀瑰叾鍊笺€?

```

  % echo 0 > addr_idx		; select address comparator 0
  % echo 0x1000 0x5000 0 > addr_range ; set address range on comparators 0, 1.
  % echo 0x2000 > addr_start    ; error as comparator 0 is a range comparator
  % echo 2 > addr_idx		; select address comparator 2
  % echo 0x2000 > addr_start	; this is OK as comparator 2 is unused.
  % echo 0x3000 > addr_stop	; error as comparator 2 set as start address.
  % echo 2 > addr_idx		; select address comparator 3
  % echo 0x3000 > addr_stop	; this is OK

```
瑕佹竻闄ゆ墍鏈夋瘮杈冨櫒锛堜互鍙婃墍鏈夊叾浠栫‖浠讹級涓婄殑缂栫▼锛屼娇鐢?
```

  % echo 1 > reset



```

### 鈥榤ode鈥?sysfs 鍙傛暟銆?


杩欐槸涓€涓綅瀛楁閫夋嫨鍙傛暟锛岀敤浜庤缃?ETM 鐨勬€讳綋璺熻釜妯″紡銆備笅琛ㄤ娇鐢ㄩ┍鍔ㄦ簮鏂囦欢涓殑瀹忓畾涔夋潵鎻忚堪鍚勪釜浣嶏紝骞剁粰鍑哄叾鎵€浠ｈ〃鐗规€х殑璇存槑銆傝澶氱壒鎬ф槸鍙€夌殑锛屽洜姝や緷璧栦簬纭欢鐨勫疄鐜般€?

浣嶅垎閰嶅涓嬶細-

----

**bit (0):**
    ETM_MODE_EXCLUDE

**description:**
    杩欐槸璁剧疆鍦板潃鑼冨洿鏃跺寘鍚?/ 鎺掗櫎鍑芥暟鐨勯粯璁ゅ€笺€傜疆 1 琛ㄧず鎺掗櫎鑼冨洿銆傝缃?mode
    鍙傛暟鏃讹紝璇ュ€间細搴旂敤鍒板綋鍓嶇储寮曠殑鍦板潃鑼冨洿銆?


**bit (4):**
    ETM_MODE_BB

**description:**
    鑻ョ‖浠舵敮鎸?[IDR0] 鍒欒缃互鍚敤鍒嗘敮骞挎挱銆傝鍔熻兘鐨勪富瑕佺敤閫旀槸鍦ㄤ唬鐮佷簬杩愯鏃惰鍔ㄦ€佹墦琛ヤ竵銆佷粎浣跨敤鏉′欢鍒嗘敮鍙兘鏃犳硶閲嶅缓瀹屾暣绋嬪簭娴佺▼鐨勬儏鍐典笅銆?

    鐩墠 Perf 涓嶆敮鎸佸悜瑙ｇ爜鍣ㄦ彁渚涗慨鏀瑰悗鐨勪簩杩涘埗鏂囦欢锛屽洜姝よ鍔熻兘浠呯敤浜庤皟璇曠洰鐨勬垨閰嶅悎绗笁鏂瑰伐鍏蜂娇鐢ㄣ€?

    閫夋嫨姝ら€夐」灏嗗鑷寸敓鎴愮殑璺熻釜閲忔樉钁楀鍔犫€斺€斿彲鑳藉瓨鍦ㄦ孩鍑洪闄╋紝鎴栬鐩栫殑鎸囦护鏇村皯銆傛敞鎰忥紝姝ら€夐」杩樹細瑕嗙洊 ETM_MODE_RETURNSTACK <coresight-return-stack> 鐨勪换浣曡缃紝鍥犳鍦ㄥ垎鏀箍鎾寖鍥翠笌杩斿洖鏍堣寖鍥撮噸鍙犵殑鎯呭喌涓嬶紝璇ヨ寖鍥村唴灏嗕笉鍙敤杩斿洖鏍堛€?


**bit (5):**
    ETMv4_MODE_CYCACC

**description:**
    鑻ユ敮鎸?[IDR0] 鍒欒缃互鍚敤鍛ㄦ湡绮剧‘璺熻釜銆?


**bit (6):**
    ETMv4_MODE_CTXID

**description:**
    鑻ョ‖浠舵敮鎸?[IDR2] 鍒欒缃互鍚敤涓婁笅鏂?ID 璺熻釜銆?


**bit (7):**
    ETM_MODE_VMID

**description:**
    鑻ユ敮鎸?[IDR2] 鍒欒缃互鍚敤铏氭嫙鏈?ID 璺熻釜銆?


**bit (11):**
    ETMv4_MODE_TIMESTAMP

**description:**
    鑻ユ敮鎸?[IDR0] 鍒欒缃互鍚敤鏃堕棿鎴崇敓鎴愩€?


**bit (12):**
    ETM_MODE_RETURNSTACK
**description:**
    鑻ユ敮鎸?[IDR0] 鍒欒缃互鍚敤璺熻釜杩斿洖鏍堛€?


**bit (13-14):**
    ETM_MODE_QELEM(val)

**description:**
    鈥榲al鈥?鍐冲畾鎵€鍚敤鐨?Q 鍏冪礌鏀寔绾у埆锛堣嫢鐢?ETM [IDR0] 瀹炵幇锛夈€?


**bit (19):**
    ETM_MODE_ATB_TRIGGER

**description:**
    鑻ユ敮鎸?[IDR5] 鍒欒缃互鍦ㄤ簨浠舵帶鍒跺瘎瀛樺櫒 [EVENTCTLR1] 涓惎鐢?ATBTRIGGER 浣嶃€?


**bit (20):**
    ETM_MODE_LPOVERRIDE

**description:**
    鑻ユ敮鎸?[IDR5] 鍒欒缃互鍦ㄤ簨浠舵帶鍒跺瘎瀛樺櫒 [EVENTCTLR1] 涓惎鐢?LPOVERRIDE 浣嶃€?


**bit (21):**
    ETM_MODE_ISTALL_EN

**description:**
    璁剧疆浠ュ湪鍋滈】鎺у埗瀵勫瓨鍣?[STALLCTLR] 涓惎鐢?ISTALL 浣嶃€?


**bit (23):**
    ETM_MODE_INSTPRIO

**description:**
    鑻ユ敮鎸?[IDR0] 鍒欒缃互鍦ㄥ仠椤挎帶鍒跺瘎瀛樺櫒 [STALLCTLR] 涓惎鐢?INSTPRIORITY 浣嶃€?


**bit (24):**
    ETM_MODE_NOOVERFLOW

**description:**
    鑻ユ敮鎸?[IDR3] 鍒欒缃互鍦ㄥ仠椤挎帶鍒跺瘎瀛樺櫒 [STALLCTLR] 涓惎鐢?NOOVERFLOW 浣嶃€?


**bit (25):**
    ETM_MODE_TRACE_RESET

**description:**
    鑻ユ敮鎸?[IDR3] 鍒欒缃互鍦ㄨ鍥炬寚浠ゆ帶鍒跺瘎瀛樺櫒 [VICTLR] 涓惎鐢?TRCRESET 浣嶃€?


**bit (26):**
    ETM_MODE_TRACE_ERR

**description:**
    璁剧疆浠ュ湪瑙嗗浘鎸囦护鎺у埗瀵勫瓨鍣?[VICTLR] 涓惎鐢?TRCCTRL 浣嶃€?


**bit (27):**
    ETM_MODE_VIEWINST_STARTSTOP

**description:**
    璁剧疆瑙嗗浘鎸囦护鎺у埗瀵勫瓨鍣?[VICTLR] 涓?ViewInst 鍚仠閫昏緫鐨勫垵濮嬬姸鎬佸€笺€?


**bit (30):**
    ETM_MODE_EXCL_KERN

**description:**
    璁剧疆榛樿璺熻釜閰嶇疆浠ユ帓闄ゅ唴鏍告ā寮忚窡韪紙鍙傝娉?a锛夈€?


**bit (31):**
    ETM_MODE_EXCL_USER

**description:**
    璁剧疆榛樿璺熻釜閰嶇疆浠ユ帓闄ょ敤鎴风┖闂磋窡韪紙鍙傝娉?a锛夈€?

----

**娉?a)** 鍚姩鏃讹紝ETM 琚紪绋嬩负浣跨敤鍦板潃鑼冨洿姣旇緝鍣?0 璺熻釜鏁翠釜鍦板潃绌洪棿銆傗€榤ode鈥?浣?30 / 31 浼氫慨鏀规璁剧疆锛屽湪鍦板潃鑼冨洿姣旇緝鍣ㄤ腑涓?NS 鐘舵€佽缃敤鎴风┖闂达紙EL0锛夋垨鍐呮牳绌洪棿锛圗L1锛夌殑 EL 鎺掗櫎浣嶃€傦紙榛樿璁剧疆鎺掗櫎鎵€鏈夊畨鍏?EL 鍜?NS EL2锛?

涓€鏃︿娇鐢ㄤ簡 reset 鍙傛暟锛屽拰/鎴栧疄鐜颁簡鑷畾涔夌紪绋嬧€斺€斾娇鐢ㄨ繖浜涗綅灏嗕互鐩稿悓鏂瑰紡璁剧疆鍦板潃姣旇緝鍣?0 鐨?EL 浣嶃€?

**娉?b)** 浣?2-3銆?-10銆?5-16銆?8銆?2 鎺у埗浠呬笌鏁版嵁璺熻釜鍗忓悓宸ヤ綔鐨勭壒鎬с€傜敱浜?ETMv4 鍦ㄦ灦鏋勪笂绂佹 A-profile 鏁版嵁璺熻釜锛屾澶勫皢鍏剁渷鐣ャ€傚彲鑳界殑鐢ㄩ€旀槸鍐呮牳浣滀负寮傛瀯绯荤粺鐨勪竴閮ㄥ垎鏀寔瀵?R 鎴?M profile 鍩虹璁炬柦杩涜鎺у埗鐨勬儏鍐点€?

浣?17銆?8-29 鏈娇鐢ㄣ€?
