## 纭畾鎬ц嚜鍔ㄦ満鎻掓々


dot2k 鍒涘缓鐨?RV 鐩戣鍣ㄦ枃浠讹紝鍚嶄负 "$MODEL_NAME.c"锛屽寘鍚竴涓笓闂ㄧ敤浜庢彃妗╋紙instrumentation锛?
鐨勮妭銆?

```

  /*
   * This is the instrumentation part of the monitor.
   *
   * This is the section where manual work is required. Here the kernel events
   * are translated into model's event.
   *
   */
  static void handle_preempt_disable(void *data, /* XXX: fill header */)
  {
	da_handle_event_wip(preempt_disable_wip);
  }

  static void handle_preempt_enable(void *data, /* XXX: fill header */)
  {
	da_handle_event_wip(preempt_enable_wip);
  }

  static void handle_sched_waking(void *data, /* XXX: fill header */)
  {
	da_handle_event_wip(sched_waking_wip);
  }

  static int enable_wip(void)
  {
	int retval;

	retval = da_monitor_init_wip();
	if (retval)
		return retval;

	rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_disable);
	rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_enable);
	rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_sched_waking);

	return 0;
  }

```
璇ヨ妭椤堕儴鐨勬敞閲婅В閲婁簡鎬讳綋鎬濊矾锛氭彃妗╄妭鎶?*鍐呮牳浜嬩欢**缈昏瘧鎴?妯″瀷鐨勪簨浠?銆?

### 璺熻釜鍥炶皟鍑芥暟


鍓嶄笁涓嚱鏁版槸鏉ヨ嚜 wip 妯″瀷鐨勪笁涓簨浠跺悇鑷殑鍥炶皟*澶勭悊鍑芥暟*鐨勮捣鐐广€傚紑鍙戣€呬笉涓€瀹氶渶瑕佷娇鐢ㄥ畠浠細
瀹冧滑鍙槸璧风偣銆?

```

 void handle_preempt_disable(void *data, /* XXX: fill header */)
 {
        da_handle_event_wip(preempt_disable_wip);
 }

```
鏉ヨ嚜妯″瀷鐨?preempt_disable 浜嬩欢鐩存帴杩炴帴鍒?preemptirq:preempt_disable銆俻reemptirq:preempt_disable
浜嬩欢
```

  TP_PROTO(unsigned long ip, unsigned long parent_ip)

```
```

  void handle_preempt_disable(void *data, unsigned long ip, unsigned long parent_ip)

```
鍦ㄨ繖绉嶆儏鍐典笅锛屽唴鏍镐簨浠朵笌鑷姩鏈轰簨浠朵竴涓€瀵瑰簲锛岀‘瀹烇紝璇ュ嚱鏁颁笉闇€瑕佸仛鍏跺畠淇敼銆?

涓嬩竴涓鐞嗗嚱鏁?handle_preempt_enable() 鍏锋湁涓?handle_preempt_disable() 鐩稿悓鐨勫弬鏁板垪琛ㄣ€?
鍖哄埆鍦ㄤ簬 preempt_enable 浜嬩欢灏嗙敤浜庢妸绯荤粺鍚屾鍒版ā鍨嬨€?

鏈€鍒濓紝**妯″瀷**琚疆浜庡垵濮嬬姸鎬併€傜劧鑰岋紝**绯荤粺**鍙兘鍦ㄤ篃鍙兘涓嶅湪鍒濆鐘舵€併€傜洃瑙嗗櫒鍦ㄧ煡閬撶郴缁熷凡
鍒拌揪鍒濆鐘舵€佷箣鍓嶄笉鑳藉紑濮嬪鐞嗕簨浠躲€傚惁鍒欙紝鐩戣鍣ㄥ拰绯荤粺鍙兘澶辨銆?

鏌ョ湅鑷姩鏈哄畾涔夛紝鍙互鐪嬪埌绯荤粺鍜屾ā鍨嬮鏈熷湪 preempt_enable 鎵ц鍚庤繑鍥炲埌鍒濆鐘舵€併€傚洜姝わ紝瀹?
鍙互鍦ㄧ洃瑙嗚妭鐨勫垵濮嬪寲鏃剁敤浜庢妸绯荤粺鍜屾ā鍨嬪悓姝ャ€?

寮€濮嬮€氳繃涓€涓壒娈婄殑 handle 鍑芥暟鍛婄煡锛?
```

  da_handle_start_event_wip(preempt_enable_wip);

```
```

  void handle_preempt_enable(void *data, unsigned long ip, unsigned long parent_ip)
  {
        da_handle_start_event_wip(preempt_enable_wip);
  }

```
```

  void handle_sched_waking(void *data, struct task_struct *task)
  {
        da_handle_event_wip(sched_waking_wip);
  }

```
鑰岃В閲婂垯鐣欑粰璇昏€呬綔涓虹粌涔犮€?

### enable 鍜?disable 鍑芥暟


```

  enable_$(MONITOR_NAME)()
  disable_$(MONITOR_NAME)()

```
杩欎簺鍑芥暟鍒嗗埆鍦ㄧ洃瑙嗗櫒琚惎鐢ㄥ拰绂佺敤鏃惰皟鐢ㄣ€?

瀹冧滑搴斿綋鐢ㄤ簬鎶婃彃妗?*闄勫姞锛坅ttach锛?*鍜?*鍒嗙锛坉etach锛?*鍒拌繍琛屼腑鐨勭郴缁熴€傚紑鍙戣€呭繀椤诲湪鐩稿簲鐨?
鍑芥暟涓坊鍔犲皢鍏剁洃瑙嗗櫒**闄勫姞**鍜?*鍒嗙**鍒扮郴缁熸墍闇€鐨勪竴鍒囥€?

```

 enable_wip()
 disable_wip()

```
浣嗕笉闇€瑕佸仛淇敼锛屽洜涓猴細榛樿鎯呭喌涓嬶紝杩欎簺鍑芥暟**闄勫姞**鍜?*鍒嗙** tracepoints_to_attach锛岃繖瀵逛簬
姝ゆ儏鍐靛凡缁忚冻澶熴€?

### 鎻掓々杈呭姪鍑芥暟


涓轰簡瀹屾垚鎻掓々锛屽湪鐩戣鍚敤闃舵锛岄渶瑕佹妸**澶勭悊鍑芥暟**闄勫姞鍒颁竴涓唴鏍镐簨浠躲€?

RV 鎺ュ彛涔熺畝鍖栦簡杩欎竴姝ャ€備緥濡傦紝瀹?"rv_attach_trace_probe()" 鐢ㄤ簬鎶?wip 妯″瀷浜嬩欢杩炴帴鍒?
鐩稿簲鐨勫唴鏍镐簨浠躲€俤ot2k 浼氳嚜鍔ㄤ负姣忎釜妯″瀷浜嬩欢鍦ㄥ惎鐢ㄩ樁娈垫坊鍔?"rv_attach_trace_probe()" 鍑芥暟
璋冪敤锛屼綔涓哄缓璁€?

```

  static int enable_wip(void)
  {
        int retval;

        retval = da_monitor_init_wip();
        if (retval)
                return retval;

        rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_enable);
        rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_sched_waking);
        rv_attach_trace_probe("wip", /* XXX: tracepoint */, handle_preempt_disable);

        return 0;
  }

```
鐒跺悗杩欎簺鎺㈤拡闇€瑕佸湪绂佺敤闃舵琚垎绂汇€?

[^1^] wip 妯″瀷鍦ㄤ互涓嬫枃妗ｄ腑缁欏嚭锛?

  Documentation/trace/rv/deterministic_automata.rst

wip 鐩戣鍣ㄥ湪浠ヤ笅鏂囨。涓粰鍑猴細

  Documentation/trace/rv/monitor_synthesis.rst
