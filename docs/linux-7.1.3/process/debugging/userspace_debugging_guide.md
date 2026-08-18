
## 鐢ㄦ埛绌洪棿璋冭瘯寤鸿


鏈枃妗ｇ畝瑕佹杩颁簡浠庣敤鎴风┖闂磋皟璇?Linux 鍐呮牳鐨勫父鐢ㄥ伐鍏枫€?闈㈠悜椹卞姩寮€鍙戣€呯殑璋冭瘯寤鸿璇疯 :doc:`姝ゅ
</process/debugging/driver_development_debugging_guide>`銆?鍏充簬涓€鑸€ц皟璇曞缓璁紝瑙?:doc:`閫氱敤寤鸿鏂囨。
</process/debugging/index>`銆?
    :depth: 3

浠ヤ笅鍚勮妭鍚戜綘灞曠ず鍙敤鐨勫伐鍏枫€?
### Dynamic debug锛堝姩鎬佽皟璇曪級


閫氳繃鍚敤/绂佺敤鏃ュ織娑堟伅鏉ヨ繃婊ゆ渶缁堣繘鍏ュ唴鏍告棩蹇楃殑鍐呭鐨勬満鍒躲€?
鍓嶇疆鏉′欢锛歚CONFIG_DYNAMIC_DEBUG`

鍔ㄦ€佽皟璇曞彧鑳介拡瀵逛互涓嬬洰鏍囷細

- pr_debug()
- dev_dbg()
- print_hex_dump_debug()
- print_hex_dump_bytes()

鍥犳锛屽氨鐩墠鑰岃█锛屾宸ュ叿鐨勫彲鐢ㄦ€х浉褰撴湁闄愶紝鍥犱负鍚戜唬鐮佸簱娣诲姞璋冭瘯鎵撳嵃骞舵病鏈?缁熶竴鐨勮鍒欙紝瀵艰嚧杩欎簺鎵撳嵃鐨勫疄鐜版柟寮忎簲鑺卞叓闂ㄣ€?
鍙﹀璇锋敞鎰忥紝澶у鏁拌皟璇曡鍙ラ兘瀹炵幇涓?dprintk() 鐨勬煇绉嶅彉浣擄紝蹇呴』閫氳繃鐩稿簲妯″潡
涓殑鍙傛暟鏉ユ縺娲伙紝鍔ㄦ€佽皟璇曟棤娉曟浛浣犲畬鎴愯繖涓€姝ャ€?
```

  $ alias ddcmd='echo $* > /proc/dynamic_debug/control'
  $ ddcmd '-p; file v4l2-h264.c +p'
  $ grep =p /proc/dynamic_debug/control
   drivers/media/v4l2-core/v4l2-h264.c:372 [v4l2_h264]print_ref_list_b =p
   "ref_pic_list_b%u (cur_poc %u%c) %s"
   drivers/media/v4l2-core/v4l2-h264.c:333 [v4l2_h264]print_ref_list_p =p
   "ref_pic_list_p (cur_poc %u%c) %s\n"

```
**浣曟椂搴旇浼樺厛浣跨敤瀹冭€屼笉鏄?Ftrace锛?*

- 褰撲唬鐮佷腑鍖呭惈鏈夋晥鐨勬墦鍗拌鍙ヤ箣涓€锛堣涓婃枃锛夋椂锛屾垨鑰呭綋浣犲湪寮€鍙戣繃绋嬩腑娣诲姞浜?  澶氫釜 pr_debug() 璇彞鏃?- 褰撴椂搴忎笉鎴愰棶棰樻椂锛屽嵆浠ｇ爜涓殑澶氫釜 pr_debug() 璇彞涓嶄細寮曡捣寤惰繜鏃?- 褰撲綘鏇村叧蹇冩帴鏀剁壒瀹氱殑鏃ュ織娑堟伅锛岃€屼笉鏄拷韪嚱鏁拌璋冪敤鐨勬ā寮忔椂

瀹屾暣鏂囨。瑙?[/admin-guide/dynamic-debug-howto](/admin-guide/dynamic-debug-howto)

### Ftrace


鍓嶇疆鏉′欢锛歚CONFIG_DYNAMIC_FTRACE`

姝ゅ伐鍏蜂娇鐢?tracefs 鏂囦欢绯荤粺鏉ュ瓨鏀炬帶鍒舵枃浠跺拰杈撳嚭鏂囦欢銆傝鏂囦欢绯荤粺浼氳鎸傝浇涓?涓€涓?`tracing` 鐩綍锛屽彲浠ュ湪 `/sys/kernel/` 鎴?`/sys/debug/kernel/` 涓壘鍒般€?
涓€浜涙渶閲嶈鐨勮皟璇曟搷浣滀负锛?
- 浣犲彲浠ラ€氳繃灏嗗嚱鏁板悕娣诲姞鍒?`set_ftrace_filter` 鏂囦欢锛堝畠鎺ュ彈
  `available_filter_functions` 鏂囦欢涓嚭鐜扮殑浠讳綍鍑芥暟鍚嶏級鏉ユ墽琛屽嚱鏁拌窡韪紱鎴栬€?  浣犱篃鍙互灏嗙壒瀹氬嚱鏁扮殑鍚嶇О娣诲姞鍒?`set_ftrace_notrace` 鏂囦欢鏉ョ鐢ㄥ畠浠紙鏇村
  淇℃伅瑙侊細trace/ftrace:dynamic ftrace锛夈€?- 涓轰簡鎵惧嚭璋冪敤鐨勬潵婧愶紝浣犲彲浠ユ縺娲?`options/func_stack_trace` 涓嬬殑
  `func_stack_trace` 閫夐」銆?- 閫氳繃鎶婃湡鏈涚殑鍑芥暟娣诲姞鍒?`set_graph_function` 鏂囦欢涓紙闇€瑕侀厤缃?  `FUNCTION_GRAPH_RETVAL`锛夛紝鍙互璺熻釜鍑芥暟璋冪敤鐨勫瓙鍑芥暟骞舵樉绀鸿繑鍥炲€硷紱鏇村淇℃伅瑙?  trace/ftrace:dynamic ftrace with the function graph tracer銆?
瀹屾暣鐨?Ftrace 鏂囨。瑙?[/trace/ftrace](/trace/ftrace)

鎴栬€咃紝浣犱篃鍙互閫氳繃 :ref:`浣跨敤浜嬩欢璺熻釜
<trace/events:2. using event tracing>` 鏉ヨ窡韪壒瀹氫簨浠讹紝鍏跺畾涔夋柟寮忚姝ゅ锛?:ref:`鍒涘缓涓€涓嚜瀹氫箟鐨?Ftrace 璺熻釜鐐?<process/debugging/driver_development_debugging_guide:ftrace>`銆?
瀹屾暣鐨?Ftrace 浜嬩欢璺熻釜鏂囨。瑙?[/trace/events](/trace/events)


#### Reading the ftrace log锛堣鍙?ftrace 鏃ュ織锛?

`trace` 鏂囦欢鍙互鍍忎换浣曞叾浠栨枃浠朵竴鏍疯鍙栵紙`cat`銆乣tail`銆乣head`銆乣vim` 绛夛級锛?鏂囦欢鐨勫ぇ灏忓彈 `buffer_size_kb` 闄愬埗锛坄echo 1000 > buffer_size_kb`锛夈€?trace/ftrace:trace_pipe 鐨勮涓轰笌 `trace` 鏂囦欢绫讳技锛屼絾姣忓綋浣犱粠璇ユ枃浠惰鍙栨椂锛?鍐呭浼氳娑堣垂鎺夈€?
#### Kernelshark


涓€涓?GUI 鐣岄潰锛岀敤浜庡皢 `trace-cmd
<https://git.kernel.org/pub/scm/utils/trace-cmd/trace-cmd.git/>`__ 搴旂敤绋嬪簭鐨?杈撳嚭鍙鍖栦负鍥惧舰鍜屽垪琛ㄨ鍥俱€?
瀹屾暣鏂囨。瑙?`<https://kernelshark.org/Documentation.html>`__

### Perf 鍙婃浛浠ｅ伐鍏?

涓婇潰鎻愬埌鐨勫伐鍏锋彁渚涗簡妫€鏌ュ唴鏍镐唬鐮併€佺粨鏋溿€佸彉閲忓€肩瓑鐨勬柟娉曘€傛湁鏃朵綘棣栧厛寰楀紕娓呮
浠庡摢閲屽叆鎵嬪幓鐪嬶紝瀵逛簬杩欎簺鎯呭喌锛屼竴濂楁€ц兘璺熻釜宸ュ叿鍙互甯姪浣犳瀹氶棶棰樸€?
#### 涓轰粈涔堝簲璇ュ仛鎬ц兘鍒嗘瀽锛?

鍦ㄤ互涓嬪師鍥犱箣涓€绛夋儏鍐典笅锛屾€ц兘鍒嗘瀽鏄竴涓緢濂界殑绗竴姝ワ細

- 浣犳棤娉曠晫瀹氶棶棰?- 浣犱笉鐭ラ亾瀹冨彂鐢熷湪鍝噷
- 杩愯涓殑绯荤粺涓嶅簲琚墦鏂紝鎴栬€呭畠鏄竴涓繙绋嬬郴缁燂紝浣犳棤娉曞湪鍏朵腑瀹夎鏂扮殑
  妯″潡/鍐呮牳

#### 濡備綍鐢?linux 宸ュ叿鍋氫竴涓畝鍗曠殑鍒嗘瀽锛?

鍦ㄦ€ц兘鍒嗘瀽鐨勫紑澶达紝浣犲彲浠ヤ粠甯哥敤宸ュ叿寮€濮嬶紝渚嬪锛?
- `top` / `htop` / `atop`锛?鑾峰彇绯荤粺璐熻浇姒傝锛屾煡鐪嬬壒瀹氳繘绋嬩笂鐨勫皷宄?锛?- `mpstat -P ALL`锛?*鏌ョ湅 CPU 涔嬮棿鐨勮礋杞藉垎甯?*锛?- `iostat -x`锛?*瑙傚療杈撳叆杈撳嚭璁惧鐨勫埄鐢ㄧ巼鍜屾€ц兘**锛?- `vmstat`锛?*绯荤粺鍐呭瓨浣跨敤姒傝**锛?- `pidstat`锛?*绫讳技浜?* `vmstat` *浣嗘寜杩涚▼锛屼互渚胯仛鐒﹀埌鐩爣*锛?- `strace -tp $PID`锛?涓€鏃︿綘鐭ラ亾浜嗚繘绋嬶紝灏卞彲浠ュ紕娓呮瀹冨浣曚笌鍐呮牳閫氫俊*锛?
杩欎簺搴旇鏈夊姪浜庡厖鍒嗙缉灏忚鏌ョ湅鐨勮寖鍥淬€?
#### Diving deeper with perf锛堢敤 perf 娣卞叆鎸栨帢锛?

**perf** 宸ュ叿鎻愪緵浜嗕竴绯诲垪鎸囨爣鍜屼簨浠讹紝浠ヨ繘涓€姝ヨ仛鐒﹂棶棰樸€?
鍓嶇疆鏉′欢锛氬湪浣犵殑绯荤粺涓婃瀯寤烘垨瀹夎 perf

```

  # perf stat -d find /usr -name 'gcc*' | wc -l

   Performance counter stats for 'find /usr -name gcc*':

     1277.81 msec    task-clock             #    0.997 CPUs utilized
     9               context-switches       #    7.043 /sec
     1               cpu-migrations         #    0.783 /sec
     704             page-faults            #  550.943 /sec
     766548897       cycles                 #    0.600 GHz                         (97.15%)
     798285467       instructions           #    1.04  insn per cycle              (97.15%)
     57582731        branches               #   45.064 M/sec                       (2.85%)
     3842573         branch-misses          #    6.67% of all branches             (97.15%)
     281616097       L1-dcache-loads        #  220.390 M/sec                       (97.15%)
     4220975         L1-dcache-load-misses  #    1.50% of all L1-dcache accesses   (97.15%)
     <not supported> LLC-loads
     <not supported> LLC-load-misses

   1.281746009 seconds time elapsed

   0.508796000 seconds user
   0.773209000 seconds sys


  52

```
浜嬩欢鍜屾寚鏍囩殑鍙敤鎬у彇鍐充簬浣犺繍琛岀殑绯荤粺銆?
瀹屾暣鏂囨。瑙?`<https://perf.wiki.kernel.org/index.php/Main_Page>`__

#### Perfetto


涓€濂楃敤浜庢祴閲忓拰鍒嗘瀽搴旂敤绋嬪簭涓庣郴缁熻〃鐜板浣曠殑宸ュ叿銆備綘鍙互鍊熷姪瀹冩潵锛?
- 璇嗗埆鐡堕
- 浼樺寲浠ｇ爜
- 璁╄蒋浠惰繍琛屽緱鏇村揩銆佹洿楂樻晥銆?
**perfetto 涓?perf 鏈変粈涔堝尯鍒紵**

- perf 鏄綔涓?Linux 鍐呮牳涓€閮ㄥ垎銆佸苟涓撻棬閽堝 Linux 鍐呮牳鐨勫伐鍏凤紝鍏锋湁 CLI 鐢ㄦ埛
  鐣岄潰銆?- perfetto 鏄法骞冲彴鐨勬€ц兘鍒嗘瀽鎶€鏈爤锛屽皢鍔熻兘鎵╁睍鍒扮敤鎴风┖闂达紝骞舵彁渚?WEB
  鐢ㄦ埛鐣岄潰銆?
瀹屾暣鏂囨。瑙?`<https://perfetto.dev/docs/>`__

### Kernel panic analysis tools锛堝唴鏍稿穿婧冨垎鏋愬伐鍏凤級


  瑕佹崟鑾峰穿婧冭浆鍌ㄨ浣跨敤 `Kdump` 鍜?`Kexec`銆備笅闈綘鍙互鎵惧埌涓€浜涘垎鏋愭暟鎹殑寤鸿銆?
  瀹屾暣鏂囨。瑙?[/admin-guide/kdump/kdump](/admin-guide/kdump/kdump)

  涓轰簡鎵惧嚭浠ｇ爜涓搴旂殑琛岋紝浣犲彲浠ヤ娇鐢?`faddr2line
  <https://elixir.bootlin.com/linux/v6.11.6/source/scripts/faddr2line>`__锛涙敞鎰?  瑕佷娇瀹冨伐浣滐紝浣犻渶瑕佸惎鐢?`CONFIG_DEBUG_INFO`銆?
  浣跨敤 `faddr2line` 鐨勬浛浠ｆ柟妗堟槸浣跨敤 `objdump`锛堜互鍙婇拡瀵逛笉鍚屽钩鍙扮殑琛嶇敓宸ュ叿锛?  濡?`aarch64-linux-gnu-objdump`锛夈€備互杩欎竴琛屼负渚嬶細

  `[  +0.000240]  rkvdec_device_run+0x50/0x138 [rockchip_vdec]`銆?
```

    aarch64-linux-gnu-objdump -dS drivers/staging/media/rkvdec/rockchip-vdec.ko | grep rkvdec_device_run\>: -A 40
    0000000000000ac8 <rkvdec_device_run>:
     ac8:	d503201f 	nop
     acc:	d503201f 	nop
    {
     ad0:	d503233f 	paciasp
     ad4:	a9bd7bfd 	stp	x29, x30, [sp, #-48]!
     ad8:	910003fd 	mov	x29, sp
     adc:	a90153f3 	stp	x19, x20, [sp, #16]
     ae0:	a9025bf5 	stp	x21, x22, [sp, #32]
        const struct rkvdec_coded_fmt_desc *desc = ctx->coded_fmt_desc;
     ae4:	f9411814 	ldr	x20, [x0, #560]
        struct rkvdec_dev *rkvdec = ctx->dev;
     ae8:	f9418015 	ldr	x21, [x0, #768]
        if (WARN_ON(!desc))
     aec:	b4000654 	cbz	x20, bb4 <rkvdec_device_run+0xec>
        ret = pm_runtime_resume_and_get(rkvdec->dev);
     af0:	f943d2b6 	ldr	x22, [x21, #1952]
        ret = __pm_runtime_resume(dev, RPM_GET_PUT);
     af4:	aa0003f3 	mov	x19, x0
     af8:	52800081 	mov	w1, #0x4                   	// #4
     afc:	aa1603e0 	mov	x0, x22
     b00:	94000000 	bl	0 <__pm_runtime_resume>
        if (ret < 0) {
     b04:	37f80340 	tbnz	w0, #31, b6c <rkvdec_device_run+0xa4>
        dev_warn(rkvdec->dev, "Not good\n");
     b08:	f943d2a0 	ldr	x0, [x21, #1952]
     b0c:	90000001 	adrp	x1, 0 <rkvdec_try_ctrl-0x8>
     b10:	91000021 	add	x1, x1, #0x0
     b14:	94000000 	bl	0 <_dev_warn>
        *bad = 1;
     b18:	d2800001 	mov	x1, #0x0                   	// #0
     ...
```

**Copyright** 漏2024 : Collabora
