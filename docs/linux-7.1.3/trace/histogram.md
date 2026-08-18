## 浜嬩欢鐩存柟鍥?


鏈枃妗ｇ敱 Tom Zanussi 缂栧啓


## 1. 绠€浠?


  鐩存柟鍥捐Е鍙戝櫒鏄竴绫荤壒娈婄殑浜嬩欢瑙﹀彂鍣紝鍙敤浜庡皢璺熻釜浜嬩欢鏁版嵁鑱氬悎涓虹洿鏂瑰浘銆?
  鏈夊叧璺熻釜浜嬩欢涓庝簨浠惰Е鍙戝櫒鐨勬洿澶氫俊鎭紝璇峰弬闃?Documentation/trace/events.rst銆?


## 2. 鐩存柟鍥捐Е鍙戝櫒鍛戒护


  鐩存柟鍥捐Е鍙戝櫒鍛戒护鏄竴绉嶄簨浠惰Е鍙戝櫒鍛戒护锛屽畠灏嗕簨浠跺懡涓仛鍚堝埌涓€涓搱甯岃〃涓紝
  璇ュ搱甯岃〃浠ヤ竴涓紙鎴栧涓級璺熻釜浜嬩欢鏍煎紡瀛楁锛堟垨鏍堝洖婧級浣滀负閿紝骞朵互浠?
  涓€涓紙鎴栧涓級璺熻釜浜嬩欢鏍煎紡瀛楁鍜?鎴栦簨浠惰鏁帮紙hitcount锛夋淳鐢熷嚭鐨勪竴缁?
  绱鎬诲拰浣滀负鍊笺€?
```

        hist:keys=<field1[,field2,...]>[:values=<field1[,field2,...]>]
          [:sort=<field1[,field2,...]>][:size=#entries][:pause][:continue]
          [:clear][:name=histname1][:nohitcount][:<handler>.<action>] [if <filter>]

  When a matching event is hit, an entry is added to a hash table
  using the key(s) and value(s) named.  Keys and values correspond to
  fields in the event's format description.  Values must correspond to
  numeric fields - on an event hit, the value(s) will be added to a
  sum kept for that field.  The special string 'hitcount' can be used
  in place of an explicit value field - this is simply a count of
  event hits.  If 'values' isn't specified, an implicit 'hitcount'
  value will be automatically created and used as the only value.
  Keys can be any field, or the special string 'common_stacktrace', which
  will use the event's kernel stacktrace as the key.  The keywords
  'keys' or 'key' can be used to specify keys, and the keywords
  'values', 'vals', or 'val' can be used to specify values.  Compound
  keys consisting of up to three fields can be specified by the 'keys'
  keyword.  Hashing a compound key produces a unique entry in the
  table for each unique combination of component keys, and can be
  useful for providing more fine-grained summaries of event data.
  Additionally, sort keys consisting of up to two fields can be
  specified by the 'sort' keyword.  If more than one field is
  specified, the result will be a 'sort within a sort': the first key
  is taken to be the primary sort key and the second the secondary
  key.  If a hist trigger is given a name using the 'name' parameter,
  its histogram data will be shared with other triggers of the same
  name, and trigger hits will update this common data.  Only triggers
  with 'compatible' fields can be combined in this way; triggers are
  'compatible' if the fields named in the trigger share the same
  number and type of fields and those fields also have the same names.
  Note that any two events always share the compatible 'hitcount' and
  'common_stacktrace' fields and can therefore be combined using those
  fields, however pointless that may be.

  'hist' triggers add a 'hist' file to each event's subdirectory.
  Reading the 'hist' file for the event will dump the hash table in
  its entirety to stdout.  If there are multiple hist triggers
  attached to an event, there will be a table for each trigger in the
  output.  The table displayed for a named trigger will be the same as
  any other instance having the same name. Each printed hash table
  entry is a simple list of the keys and values comprising the entry;
  keys are printed first and are delineated by curly braces, and are
  followed by the set of value fields for the entry.  By default,
  numeric fields are displayed as base-10 integers.  This can be
  modified by appending any of the following modifiers to the field
  name:

	=============  =================================================
        .hex           display a number as a hex value
	.sym           display an address as a symbol
	.sym-offset    display an address as a symbol and offset
	.syscall       display a syscall id as a system call name
	.execname      display a common_pid as a program name
	.log2          display log2 value rather than raw number
	.buckets=size  display grouping of values rather than raw number
	.usecs         display a common_timestamp in microseconds
        .percent       display a number of percentage value
        .graph         display a bar-graph of a value
	.stacktrace    display as a stacktrace (must be a long[] type)
	=============  =================================================

  Note that in general the semantics of a given field aren't
  interpreted when applying a modifier to it, but there are some
  restrictions to be aware of in this regard:

    - only the 'hex' modifier can be used for values (because values
      are essentially sums, and the other modifiers don't make sense
      in that context).
    - the 'execname' modifier can only be used on a 'common_pid'.  The
      reason for this is that the execname is simply the 'comm' value
      saved for the 'current' process when an event was triggered,
      which is the same as the common_pid value saved by the event
      tracing code.  Trying to apply that comm value to other pid
      values wouldn't be correct, and typically events that care save
      pid-specific comm fields in the event itself.

  A typical usage scenario would be the following to enable a hist
  trigger, read its current contents, and then turn it off::

    # echo 'hist:keys=skbaddr.hex:vals=len' > \
      /sys/kernel/tracing/events/net/netif_rx/trigger

    # cat /sys/kernel/tracing/events/net/netif_rx/hist

    # echo '!hist:keys=skbaddr.hex:vals=len' > \
      /sys/kernel/tracing/events/net/netif_rx/trigger

  The trigger file itself can be read to show the details of the
  currently attached hist trigger.  This information is also displayed
  at the top of the 'hist' file when read.

  By default, the size of the hash table is 2048 entries.  The 'size'
  parameter can be used to specify more or fewer than that.  The units
  are in terms of hashtable entries - if a run uses more entries than
  specified, the results will show the number of 'drops', the number
  of hits that were ignored.  The size should be a power of 2 between
  128 and 131072 (any non- power-of-2 number specified will be rounded
  up).

  The 'sort' parameter can be used to specify a value field to sort
  on.  The default if unspecified is 'hitcount' and the default sort
  order is 'ascending'.  To sort in the opposite direction, append
  .descending' to the sort key.

  The 'pause' parameter can be used to pause an existing hist trigger
  or to start a hist trigger but not log any events until told to do
  so.  'continue' or 'cont' can be used to start or restart a paused
  hist trigger.

  The 'clear' parameter will clear the contents of a running hist
  trigger and leave its current paused/active state.

  Note that the 'pause', 'cont', and 'clear' parameters should be
  applied using 'append' shell operator ('>>') if applied to an
  existing trigger, rather than via the '>' operator, which will cause
  the trigger to be removed through truncation.

  The 'nohitcount' (or NOHC) parameter will suppress display of
  raw hitcount in the histogram. This option requires at least one
  value field which is not a 'raw hitcount'. For example,
  'hist:...:vals=hitcount:nohitcount' is rejected, but
  'hist:...:vals=hitcount.percent:nohitcount' is OK.

```
- enable_hist/disable_hist

  enable_hist 涓?disable_hist 瑙﹀彂鍣ㄥ彲鐢ㄤ簬璁╂煇涓簨浠舵湁鏉′欢鍦板惎鍔ㄥ拰鍋滄鍙︿竴涓?
  浜嬩欢宸查檮鐫€鐨?hist 瑙﹀彂鍣ㄣ€傚彲浠ュ皢浠绘剰鏁伴噺鐨?enable_hist 涓?disable_hist 瑙﹀彂鍣?
  闄勫姞鍒扮粰瀹氫簨浠朵笂锛屼粠鑰岃璇ヤ簨浠跺惎鍔ㄥ苟鍋滄澶ч噺鍏朵粬浜嬩欢鐨勮仛鍚堛€?
```

      enable_hist:<system>:<event>[:count]
      disable_hist:<system>:<event>[:count]

  Instead of enabling or disabling the tracing of the target event
  into the trace buffer as the enable/disable_event triggers do, the
  enable/disable_hist triggers enable or disable the aggregation of
  the target event into a hash table.

  A typical usage scenario for the enable_hist/disable_hist triggers
  would be to first set up a paused hist trigger on some event,
  followed by an enable_hist/disable_hist pair that turns the hist
  aggregation on and off when conditions of interest are hit::

   # echo 'hist:keys=skbaddr.hex:vals=len:pause' > \
      /sys/kernel/tracing/events/net/netif_receive_skb/trigger

    # echo 'enable_hist:net:netif_receive_skb if filename==/usr/bin/wget' > \
      /sys/kernel/tracing/events/sched/sched_process_exec/trigger

    # echo 'disable_hist:net:netif_receive_skb if comm==wget' > \
      /sys/kernel/tracing/events/sched/sched_process_exit/trigger

  The above sets up an initially paused hist trigger which is unpaused
  and starts aggregating events when a given program is executed, and
  which stops aggregating when the process exits and the hist trigger
  is paused again.

  The examples below provide a more concrete illustration of the
  concepts and typical usage patterns discussed above.

```
### 2.1. 鈥滅壒娈娾€濅簨浠跺瓧娈?


  鏈変竴绯诲垪鈥滅壒娈婁簨浠跺瓧娈碘€濆彲鐢ㄤ綔 hist 瑙﹀彂鍣ㄤ腑鐨勯敭鎴栧€笺€傚畠浠湅璧锋潵鍜岃涓洪兘鍍忔槸
  鐪熸鐨勪簨浠跺瓧娈碉紝浣嗗疄闄呬笂骞朵笉鏄簨浠跺瓧娈靛畾涔夋垨鏍煎紡鏂囦欢鐨勪竴閮ㄥ垎銆備笉杩囷紝瀹冧滑瀵?
  浠讳綍浜嬩欢閮藉彲鐢紝骞朵笖鍙互鍑虹幇鍦ㄧ湡姝ｄ簨浠跺瓧娈佃兘鍑虹幇鐨勪换浣曞湴鏂广€傚畠浠寘鎷細

    ====================== ==== =======================================
    common_timestamp       u64  涓庝簨浠跺叧鑱旂殑锛堟潵鑷幆褰㈢紦鍐插尯鐨勶級鏃堕棿鎴筹紝
                                鍗曚綅涓虹撼绉掋€傚彲閫氳繃 .usecs 淇グ锛屼娇鏃堕棿鎴?
		        琚В閲婁负寰銆?
    common_cpu             int  浜嬩欢鍙戠敓鎵€鍦ㄧ殑 CPU銆?
    ====================== ==== =======================================

### 2.2. 鎵╁睍閿欒淇℃伅


  鍦ㄨ皟鐢?hist 瑙﹀彂鍣ㄥ懡浠ゆ椂閬囧埌鏌愪簺閿欒鏉′欢锛屽彲閫氳繃 tracing/error_log 鏂囦欢
  鑾峰緱鎵╁睍閿欒淇℃伅銆傝鎯呰鍙傞槄 Documentation/trace/ftrace.rst 涓殑鈥滈敊璇潯浠垛€?
  涓€鑺傘€?

### 2.3. 鈥榟ist鈥?瑙﹀彂鍣ㄧず渚?


  绗竴缁勭ず渚嬩娇鐢?kmalloc 浜嬩欢鍒涘缓鑱氬悎銆傚彲鐢ㄤ簬 hist 瑙﹀彂鍣ㄧ殑瀛楁鍒楃ず浜?
```

    # cat /sys/kernel/tracing/events/kmem/kmalloc/format
    name: kmalloc
    ID: 374
    format:
	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
	field:unsigned char common_preempt_count;		offset:3;	size:1;	signed:0;
	field:int common_pid;					offset:4;	size:4;	signed:1;

	field:unsigned long call_site;				offset:8;	size:8;	signed:0;
	field:const void * ptr;					offset:16;	size:8;	signed:0;
	field:size_t bytes_req;					offset:24;	size:8;	signed:0;
	field:size_t bytes_alloc;				offset:32;	size:8;	signed:0;
	field:gfp_t gfp_flags;					offset:40;	size:4;	signed:0;

  We'll start by creating a hist trigger that generates a simple table
  that lists the total number of bytes requested for each function in
  the kernel that made one or more calls to kmalloc::

    # echo 'hist:key=call_site:val=bytes_req.buckets=32' > \
            /sys/kernel/tracing/events/kmem/kmalloc/trigger

  This tells the tracing system to create a 'hist' trigger using the
  call_site field of the kmalloc event as the key for the table, which
  just means that each unique call_site address will have an entry
  created for it in the table.  The 'val=bytes_req' parameter tells
  the hist trigger that for each unique entry (call_site) in the
  table, it should keep a running total of the number of bytes
  requested by that call_site.

  We'll let it run for a while and then dump the contents of the 'hist'
  file in the kmalloc event's subdirectory (for readability, a number
  of entries have been omitted)::

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=call_site:vals=bytes_req:sort=hitcount:size=2048 [active]

    { call_site: 18446744072106379007 } hitcount:          1  bytes_req:        176
    { call_site: 18446744071579557049 } hitcount:          1  bytes_req:       1024
    { call_site: 18446744071580608289 } hitcount:          1  bytes_req:      16384
    { call_site: 18446744071581827654 } hitcount:          1  bytes_req:         24
    { call_site: 18446744071580700980 } hitcount:          1  bytes_req:          8
    { call_site: 18446744071579359876 } hitcount:          1  bytes_req:        152
    { call_site: 18446744071580795365 } hitcount:          3  bytes_req:        144
    { call_site: 18446744071581303129 } hitcount:          3  bytes_req:        144
    { call_site: 18446744071580713234 } hitcount:          4  bytes_req:       2560
    { call_site: 18446744071580933750 } hitcount:          4  bytes_req:        736
    .
    .
    .
    { call_site: 18446744072106047046 } hitcount:         69  bytes_req:       5576
    { call_site: 18446744071582116407 } hitcount:         73  bytes_req:       2336
    { call_site: 18446744072106054684 } hitcount:        136  bytes_req:     140504
    { call_site: 18446744072106224230 } hitcount:        136  bytes_req:      19584
    { call_site: 18446744072106078074 } hitcount:        153  bytes_req:       2448
    { call_site: 18446744072106062406 } hitcount:        153  bytes_req:      36720
    { call_site: 18446744071582507929 } hitcount:        153  bytes_req:      37088
    { call_site: 18446744072102520590 } hitcount:        273  bytes_req:      10920
    { call_site: 18446744071582143559 } hitcount:        358  bytes_req:        716
    { call_site: 18446744072106465852 } hitcount:        417  bytes_req:      56712
    { call_site: 18446744072102523378 } hitcount:        485  bytes_req:      27160
    { call_site: 18446744072099568646 } hitcount:       1676  bytes_req:      33520

    Totals:
        Hits: 4610
        Entries: 45
        Dropped: 0

  The output displays a line for each entry, beginning with the key
  specified in the trigger, followed by the value(s) also specified in
  the trigger.  At the beginning of the output is a line that displays
  the trigger info, which can also be displayed by reading the
  'trigger' file::

    # cat /sys/kernel/tracing/events/kmem/kmalloc/trigger
    hist:keys=call_site:vals=bytes_req:sort=hitcount:size=2048 [active]

  At the end of the output are a few lines that display the overall
  totals for the run.  The 'Hits' field shows the total number of
  times the event trigger was hit, the 'Entries' field shows the total
  number of used entries in the hash table, and the 'Dropped' field
  shows the number of hits that were dropped because the number of
  used entries for the run exceeded the maximum number of entries
  allowed for the table (normally 0, but if not a hint that you may
  want to increase the size of the table using the 'size' parameter).

  Notice in the above output that there's an extra field, 'hitcount',
  which wasn't specified in the trigger.  Also notice that in the
  trigger info output, there's a parameter, 'sort=hitcount', which
  wasn't specified in the trigger either.  The reason for that is that
  every trigger implicitly keeps a count of the total number of hits
  attributed to a given entry, called the 'hitcount'.  That hitcount
  information is explicitly displayed in the output, and in the
  absence of a user-specified sort parameter, is used as the default
  sort field.

  The value 'hitcount' can be used in place of an explicit value in
  the 'values' parameter if you don't really need to have any
  particular field summed and are mainly interested in hit
  frequencies.

  To turn the hist trigger off, simply call up the trigger in the
  command history and re-execute it with a '!' prepended::

    # echo '!hist:key=call_site:val=bytes_req' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

  Finally, notice that the call_site as displayed in the output above
  isn't really very useful.  It's an address, but normally addresses
  are displayed in hex.  To have a numeric field displayed as a hex
  value, simply append '.hex' to the field name in the trigger::

    # echo 'hist:key=call_site.hex:val=bytes_req' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=call_site.hex:vals=bytes_req:sort=hitcount:size=2048 [active]

    { call_site: ffffffffa026b291 } hitcount:          1  bytes_req:        433
    { call_site: ffffffffa07186ff } hitcount:          1  bytes_req:        176
    { call_site: ffffffff811ae721 } hitcount:          1  bytes_req:      16384
    { call_site: ffffffff811c5134 } hitcount:          1  bytes_req:          8
    { call_site: ffffffffa04a9ebb } hitcount:          1  bytes_req:        511
    { call_site: ffffffff8122e0a6 } hitcount:          1  bytes_req:         12
    { call_site: ffffffff8107da84 } hitcount:          1  bytes_req:        152
    { call_site: ffffffff812d8246 } hitcount:          1  bytes_req:         24
    { call_site: ffffffff811dc1e5 } hitcount:          3  bytes_req:        144
    { call_site: ffffffffa02515e8 } hitcount:          3  bytes_req:        648
    { call_site: ffffffff81258159 } hitcount:          3  bytes_req:        144
    { call_site: ffffffff811c80f4 } hitcount:          4  bytes_req:        544
    .
    .
    .
    { call_site: ffffffffa06c7646 } hitcount:        106  bytes_req:       8024
    { call_site: ffffffffa06cb246 } hitcount:        132  bytes_req:      31680
    { call_site: ffffffffa06cef7a } hitcount:        132  bytes_req:       2112
    { call_site: ffffffff8137e399 } hitcount:        132  bytes_req:      23232
    { call_site: ffffffffa06c941c } hitcount:        185  bytes_req:     171360
    { call_site: ffffffffa06f2a66 } hitcount:        185  bytes_req:      26640
    { call_site: ffffffffa036a70e } hitcount:        265  bytes_req:      10600
    { call_site: ffffffff81325447 } hitcount:        292  bytes_req:        584
    { call_site: ffffffffa072da3c } hitcount:        446  bytes_req:      60656
    { call_site: ffffffffa036b1f2 } hitcount:        526  bytes_req:      29456
    { call_site: ffffffffa0099c06 } hitcount:       1780  bytes_req:      35600

    Totals:
        Hits: 4775
        Entries: 46
        Dropped: 0

  Even that's only marginally more useful - while hex values do look
  more like addresses, what users are typically more interested in
  when looking at text addresses are the corresponding symbols
  instead.  To have an address displayed as symbolic value instead,
  simply append '.sym' or '.sym-offset' to the field name in the
  trigger::

    # echo 'hist:key=call_site.sym:val=bytes_req' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=call_site.sym:vals=bytes_req:sort=hitcount:size=2048 [active]

    { call_site: [ffffffff810adcb9] syslog_print_all                              } hitcount:          1  bytes_req:       1024
    { call_site: [ffffffff8154bc62] usb_control_msg                               } hitcount:          1  bytes_req:          8
    { call_site: [ffffffffa00bf6fe] hidraw_send_report [hid]                      } hitcount:          1  bytes_req:          7
    { call_site: [ffffffff8154acbe] usb_alloc_urb                                 } hitcount:          1  bytes_req:        192
    { call_site: [ffffffffa00bf1ca] hidraw_report_event [hid]                     } hitcount:          1  bytes_req:          7
    { call_site: [ffffffff811e3a25] __seq_open_private                            } hitcount:          1  bytes_req:         40
    { call_site: [ffffffff8109524a] alloc_fair_sched_group                        } hitcount:          2  bytes_req:        128
    { call_site: [ffffffff811febd5] fsnotify_alloc_group                          } hitcount:          2  bytes_req:        528
    { call_site: [ffffffff81440f58] __tty_buffer_request_room                     } hitcount:          2  bytes_req:       2624
    { call_site: [ffffffff81200ba6] inotify_new_group                             } hitcount:          2  bytes_req:         96
    { call_site: [ffffffffa05e19af] ieee80211_start_tx_ba_session [mac80211]      } hitcount:          2  bytes_req:        464
    { call_site: [ffffffff81672406] tcp_get_metrics                               } hitcount:          2  bytes_req:        304
    { call_site: [ffffffff81097ec2] alloc_rt_sched_group                          } hitcount:          2  bytes_req:        128
    { call_site: [ffffffff81089b05] sched_create_group                            } hitcount:          2  bytes_req:       1424
    .
    .
    .
    { call_site: [ffffffffa04a580c] intel_crtc_page_flip [i915]                   } hitcount:       1185  bytes_req:     123240
    { call_site: [ffffffffa0287592] drm_mode_page_flip_ioctl [drm]                } hitcount:       1185  bytes_req:     104280
    { call_site: [ffffffffa04c4a3c] intel_plane_duplicate_state [i915]            } hitcount:       1402  bytes_req:     190672
    { call_site: [ffffffff812891ca] ext4_find_extent                              } hitcount:       1518  bytes_req:     146208
    { call_site: [ffffffffa029070e] drm_vma_node_allow [drm]                      } hitcount:       1746  bytes_req:      69840
    { call_site: [ffffffffa045e7c4] i915_gem_do_execbuffer.isra.23 [i915]         } hitcount:       2021  bytes_req:     792312
    { call_site: [ffffffffa02911f2] drm_modeset_lock_crtc [drm]                   } hitcount:       2592  bytes_req:     145152
    { call_site: [ffffffffa0489a66] intel_ring_begin [i915]                       } hitcount:       2629  bytes_req:     378576
    { call_site: [ffffffffa046041c] i915_gem_execbuffer2 [i915]                   } hitcount:       2629  bytes_req:    3783248
    { call_site: [ffffffff81325607] apparmor_file_alloc_security                  } hitcount:       5192  bytes_req:      10384
    { call_site: [ffffffffa00b7c06] hid_report_raw_event [hid]                    } hitcount:       5529  bytes_req:     110584
    { call_site: [ffffffff8131ebf7] aa_alloc_task_context                         } hitcount:      21943  bytes_req:     702176
    { call_site: [ffffffff8125847d] ext4_htree_store_dirent                       } hitcount:      55759  bytes_req:    5074265

    Totals:
        Hits: 109928
        Entries: 71
        Dropped: 0

  Because the default sort key above is 'hitcount', the above shows a
  the list of call_sites by increasing hitcount, so that at the bottom
  we see the functions that made the most kmalloc calls during the
  run.  If instead we wanted to see the top kmalloc callers in
  terms of the number of bytes requested rather than the number of
  calls, and we wanted the top caller to appear at the top, we can use
  the 'sort' parameter, along with the 'descending' modifier::

    # echo 'hist:key=call_site.sym:val=bytes_req:sort=bytes_req.descending' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=call_site.sym:vals=bytes_req:sort=bytes_req.descending:size=2048 [active]

    { call_site: [ffffffffa046041c] i915_gem_execbuffer2 [i915]                   } hitcount:       2186  bytes_req:    3397464
    { call_site: [ffffffffa045e7c4] i915_gem_do_execbuffer.isra.23 [i915]         } hitcount:       1790  bytes_req:     712176
    { call_site: [ffffffff8125847d] ext4_htree_store_dirent                       } hitcount:       8132  bytes_req:     513135
    { call_site: [ffffffff811e2a1b] seq_buf_alloc                                 } hitcount:        106  bytes_req:     440128
    { call_site: [ffffffffa0489a66] intel_ring_begin [i915]                       } hitcount:       2186  bytes_req:     314784
    { call_site: [ffffffff812891ca] ext4_find_extent                              } hitcount:       2174  bytes_req:     208992
    { call_site: [ffffffff811ae8e1] __kmalloc                                     } hitcount:          8  bytes_req:     131072
    { call_site: [ffffffffa04c4a3c] intel_plane_duplicate_state [i915]            } hitcount:        859  bytes_req:     116824
    { call_site: [ffffffffa02911f2] drm_modeset_lock_crtc [drm]                   } hitcount:       1834  bytes_req:     102704
    { call_site: [ffffffffa04a580c] intel_crtc_page_flip [i915]                   } hitcount:        972  bytes_req:     101088
    { call_site: [ffffffffa0287592] drm_mode_page_flip_ioctl [drm]                } hitcount:        972  bytes_req:      85536
    { call_site: [ffffffffa00b7c06] hid_report_raw_event [hid]                    } hitcount:       3333  bytes_req:      66664
    { call_site: [ffffffff8137e559] sg_kmalloc                                    } hitcount:        209  bytes_req:      61632
    .
    .
    .
    { call_site: [ffffffff81095225] alloc_fair_sched_group                        } hitcount:          2  bytes_req:        128
    { call_site: [ffffffff81097ec2] alloc_rt_sched_group                          } hitcount:          2  bytes_req:        128
    { call_site: [ffffffff812d8406] copy_semundo                                  } hitcount:          2  bytes_req:         48
    { call_site: [ffffffff81200ba6] inotify_new_group                             } hitcount:          1  bytes_req:         48
    { call_site: [ffffffffa027121a] drm_getmagic [drm]                            } hitcount:          1  bytes_req:         48
    { call_site: [ffffffff811e3a25] __seq_open_private                            } hitcount:          1  bytes_req:         40
    { call_site: [ffffffff811c52f4] bprm_change_interp                            } hitcount:          2  bytes_req:         16
    { call_site: [ffffffff8154bc62] usb_control_msg                               } hitcount:          1  bytes_req:          8
    { call_site: [ffffffffa00bf1ca] hidraw_report_event [hid]                     } hitcount:          1  bytes_req:          7
    { call_site: [ffffffffa00bf6fe] hidraw_send_report [hid]                      } hitcount:          1  bytes_req:          7

    Totals:
        Hits: 32133
        Entries: 81
        Dropped: 0

  To display the offset and size information in addition to the symbol
  name, just use 'sym-offset' instead::

    # echo 'hist:key=call_site.sym-offset:val=bytes_req:sort=bytes_req.descending' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=call_site.sym-offset:vals=bytes_req:sort=bytes_req.descending:size=2048 [active]

    { call_site: [ffffffffa046041c] i915_gem_execbuffer2+0x6c/0x2c0 [i915]                  } hitcount:       4569  bytes_req:    3163720
    { call_site: [ffffffffa0489a66] intel_ring_begin+0xc6/0x1f0 [i915]                      } hitcount:       4569  bytes_req:     657936
    { call_site: [ffffffffa045e7c4] i915_gem_do_execbuffer.isra.23+0x694/0x1020 [i915]      } hitcount:       1519  bytes_req:     472936
    { call_site: [ffffffffa045e646] i915_gem_do_execbuffer.isra.23+0x516/0x1020 [i915]      } hitcount:       3050  bytes_req:     211832
    { call_site: [ffffffff811e2a1b] seq_buf_alloc+0x1b/0x50                                 } hitcount:         34  bytes_req:     148384
    { call_site: [ffffffffa04a580c] intel_crtc_page_flip+0xbc/0x870 [i915]                  } hitcount:       1385  bytes_req:     144040
    { call_site: [ffffffff811ae8e1] __kmalloc+0x191/0x1b0                                   } hitcount:          8  bytes_req:     131072
    { call_site: [ffffffffa0287592] drm_mode_page_flip_ioctl+0x282/0x360 [drm]              } hitcount:       1385  bytes_req:     121880
    { call_site: [ffffffffa02911f2] drm_modeset_lock_crtc+0x32/0x100 [drm]                  } hitcount:       1848  bytes_req:     103488
    { call_site: [ffffffffa04c4a3c] intel_plane_duplicate_state+0x2c/0xa0 [i915]            } hitcount:        461  bytes_req:      62696
    { call_site: [ffffffffa029070e] drm_vma_node_allow+0x2e/0xd0 [drm]                      } hitcount:       1541  bytes_req:      61640
    { call_site: [ffffffff815f8d7b] sk_prot_alloc+0xcb/0x1b0                                } hitcount:         57  bytes_req:      57456
    .
    .
    .
    { call_site: [ffffffff8109524a] alloc_fair_sched_group+0x5a/0x1a0                       } hitcount:          2  bytes_req:        128
    { call_site: [ffffffffa027b921] drm_vm_open_locked+0x31/0xa0 [drm]                      } hitcount:          3  bytes_req:         96
    { call_site: [ffffffff8122e266] proc_self_follow_link+0x76/0xb0                         } hitcount:          8  bytes_req:         96
    { call_site: [ffffffff81213e80] load_elf_binary+0x240/0x1650                            } hitcount:          3  bytes_req:         84
    { call_site: [ffffffff8154bc62] usb_control_msg+0x42/0x110                              } hitcount:          1  bytes_req:          8
    { call_site: [ffffffffa00bf6fe] hidraw_send_report+0x7e/0x1a0 [hid]                     } hitcount:          1  bytes_req:          7
    { call_site: [ffffffffa00bf1ca] hidraw_report_event+0x8a/0x120 [hid]                    } hitcount:          1  bytes_req:          7

    Totals:
        Hits: 26098
        Entries: 64
        Dropped: 0

  We can also add multiple fields to the 'values' parameter.  For
  example, we might want to see the total number of bytes allocated
  alongside bytes requested, and display the result sorted by bytes
  allocated in a descending order::

    # echo 'hist:keys=call_site.sym:values=bytes_req,bytes_alloc:sort=bytes_alloc.descending' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=call_site.sym:vals=bytes_req,bytes_alloc:sort=bytes_alloc.descending:size=2048 [active]

    { call_site: [ffffffffa046041c] i915_gem_execbuffer2 [i915]                   } hitcount:       7403  bytes_req:    4084360  bytes_alloc:    5958016
    { call_site: [ffffffff811e2a1b] seq_buf_alloc                                 } hitcount:        541  bytes_req:    2213968  bytes_alloc:    2228224
    { call_site: [ffffffffa0489a66] intel_ring_begin [i915]                       } hitcount:       7404  bytes_req:    1066176  bytes_alloc:    1421568
    { call_site: [ffffffffa045e7c4] i915_gem_do_execbuffer.isra.23 [i915]         } hitcount:       1565  bytes_req:     557368  bytes_alloc:    1037760
    { call_site: [ffffffff8125847d] ext4_htree_store_dirent                       } hitcount:       9557  bytes_req:     595778  bytes_alloc:     695744
    { call_site: [ffffffffa045e646] i915_gem_do_execbuffer.isra.23 [i915]         } hitcount:       5839  bytes_req:     430680  bytes_alloc:     470400
    { call_site: [ffffffffa04c4a3c] intel_plane_duplicate_state [i915]            } hitcount:       2388  bytes_req:     324768  bytes_alloc:     458496
    { call_site: [ffffffffa02911f2] drm_modeset_lock_crtc [drm]                   } hitcount:       3911  bytes_req:     219016  bytes_alloc:     250304
    { call_site: [ffffffff815f8d7b] sk_prot_alloc                                 } hitcount:        235  bytes_req:     236880  bytes_alloc:     240640
    { call_site: [ffffffff8137e559] sg_kmalloc                                    } hitcount:        557  bytes_req:     169024  bytes_alloc:     221760
    { call_site: [ffffffffa00b7c06] hid_report_raw_event [hid]                    } hitcount:       9378  bytes_req:     187548  bytes_alloc:     206312
    { call_site: [ffffffffa04a580c] intel_crtc_page_flip [i915]                   } hitcount:       1519  bytes_req:     157976  bytes_alloc:     194432
    .
    .
    .
    { call_site: [ffffffff8109bd3b] sched_autogroup_create_attach                 } hitcount:          2  bytes_req:        144  bytes_alloc:        192
    { call_site: [ffffffff81097ee8] alloc_rt_sched_group                          } hitcount:          2  bytes_req:        128  bytes_alloc:        128
    { call_site: [ffffffff8109524a] alloc_fair_sched_group                        } hitcount:          2  bytes_req:        128  bytes_alloc:        128
    { call_site: [ffffffff81095225] alloc_fair_sched_group                        } hitcount:          2  bytes_req:        128  bytes_alloc:        128
    { call_site: [ffffffff81097ec2] alloc_rt_sched_group                          } hitcount:          2  bytes_req:        128  bytes_alloc:        128
    { call_site: [ffffffff81213e80] load_elf_binary                               } hitcount:          3  bytes_req:         84  bytes_alloc:         96
    { call_site: [ffffffff81079a2e] kthread_create_on_node                        } hitcount:          1  bytes_req:         56  bytes_alloc:         64
    { call_site: [ffffffffa00bf6fe] hidraw_send_report [hid]                      } hitcount:          1  bytes_req:          7  bytes_alloc:          8
    { call_site: [ffffffff8154bc62] usb_control_msg                               } hitcount:          1  bytes_req:          8  bytes_alloc:          8
    { call_site: [ffffffffa00bf1ca] hidraw_report_event [hid]                     } hitcount:          1  bytes_req:          7  bytes_alloc:          8

    Totals:
        Hits: 66598
        Entries: 65
        Dropped: 0

  Finally, to finish off our kmalloc example, instead of simply having
  the hist trigger display symbolic call_sites, we can have the hist
  trigger additionally display the complete set of kernel stack traces
  that led to each call_site.  To do that, we simply use the special
  value 'common_stacktrace' for the key parameter::

    # echo 'hist:keys=common_stacktrace:values=bytes_req,bytes_alloc:sort=bytes_alloc' > \
           /sys/kernel/tracing/events/kmem/kmalloc/trigger

  The above trigger will use the kernel stack trace in effect when an
  event is triggered as the key for the hash table.  This allows the
  enumeration of every kernel callpath that led up to a particular
  event, along with a running total of any of the event fields for
  that event.  Here we tally bytes requested and bytes allocated for
  every callpath in the system that led up to a kmalloc (in this case
  every callpath to a kmalloc for a kernel compile)::

    # cat /sys/kernel/tracing/events/kmem/kmalloc/hist
    # trigger info: hist:keys=common_stacktrace:vals=bytes_req,bytes_alloc:sort=bytes_alloc:size=2048 [active]

    { common_stacktrace:
         __kmalloc_track_caller+0x10b/0x1a0
         kmemdup+0x20/0x50
         hidraw_report_event+0x8a/0x120 [hid]
         hid_report_raw_event+0x3ea/0x440 [hid]
         hid_input_report+0x112/0x190 [hid]
         hid_irq_in+0xc2/0x260 [usbhid]
         __usb_hcd_giveback_urb+0x72/0x120
         usb_giveback_urb_bh+0x9e/0xe0
         tasklet_hi_action+0xf8/0x100
         __do_softirq+0x114/0x2c0
         irq_exit+0xa5/0xb0
         do_IRQ+0x5a/0xf0
         ret_from_intr+0x0/0x30
         cpuidle_enter+0x17/0x20
         cpu_startup_entry+0x315/0x3e0
         rest_init+0x7c/0x80
    } hitcount:          3  bytes_req:         21  bytes_alloc:         24
    { common_stacktrace:
         __kmalloc_track_caller+0x10b/0x1a0
         kmemdup+0x20/0x50
         hidraw_report_event+0x8a/0x120 [hid]
         hid_report_raw_event+0x3ea/0x440 [hid]
         hid_input_report+0x112/0x190 [hid]
         hid_irq_in+0xc2/0x260 [usbhid]
         __usb_hcd_giveback_urb+0x72/0x120
         usb_giveback_urb_bh+0x9e/0xe0
         tasklet_hi_action+0xf8/0x100
         __do_softirq+0x114/0x2c0
         irq_exit+0xa5/0xb0
         do_IRQ+0x5a/0xf0
         ret_from_intr+0x0/0x30
    } hitcount:          3  bytes_req:         21  bytes_alloc:         24
    { common_stacktrace:
         kmem_cache_alloc_trace+0xeb/0x150
         aa_alloc_task_context+0x27/0x40
         apparmor_cred_prepare+0x1f/0x50
         security_prepare_creds+0x16/0x20
         prepare_creds+0xdf/0x1a0
         SyS_capset+0xb5/0x200
         system_call_fastpath+0x12/0x6a
    } hitcount:          1  bytes_req:         32  bytes_alloc:         32
    .
    .
    .
    { common_stacktrace:
         __kmalloc+0x11b/0x1b0
         i915_gem_execbuffer2+0x6c/0x2c0 [i915]
         drm_ioctl+0x349/0x670 [drm]
         do_vfs_ioctl+0x2f0/0x4f0
         SyS_ioctl+0x81/0xa0
         system_call_fastpath+0x12/0x6a
    } hitcount:      17726  bytes_req:   13944120  bytes_alloc:   19593808
    { common_stacktrace:
         __kmalloc+0x11b/0x1b0
         load_elf_phdrs+0x76/0xa0
         load_elf_binary+0x102/0x1650
         search_binary_handler+0x97/0x1d0
         do_execveat_common.isra.34+0x551/0x6e0
         SyS_execve+0x3a/0x50
         return_from_execve+0x0/0x23
    } hitcount:      33348  bytes_req:   17152128  bytes_alloc:   20226048
    { common_stacktrace:
         kmem_cache_alloc_trace+0xeb/0x150
         apparmor_file_alloc_security+0x27/0x40
         security_file_alloc+0x16/0x20
         get_empty_filp+0x93/0x1c0
         path_openat+0x31/0x5f0
         do_filp_open+0x3a/0x90
         do_sys_open+0x128/0x220
         SyS_open+0x1e/0x20
         system_call_fastpath+0x12/0x6a
    } hitcount:    4766422  bytes_req:    9532844  bytes_alloc:   38131376
    { common_stacktrace:
         __kmalloc+0x11b/0x1b0
         seq_buf_alloc+0x1b/0x50
         seq_read+0x2cc/0x370
         proc_reg_read+0x3d/0x80
         __vfs_read+0x28/0xe0
         vfs_read+0x86/0x140
         SyS_read+0x46/0xb0
         system_call_fastpath+0x12/0x6a
    } hitcount:      19133  bytes_req:   78368768  bytes_alloc:   78368768

    Totals:
        Hits: 6085872
        Entries: 253
        Dropped: 0

  If you key a hist trigger on common_pid, in order for example to
  gather and display sorted totals for each process, you can use the
  special .execname modifier to display the executable names for the
  processes in the table rather than raw pids.  The example below
  keeps a per-process sum of total bytes read::

    # echo 'hist:key=common_pid.execname:val=count:sort=count.descending' > \
           /sys/kernel/tracing/events/syscalls/sys_enter_read/trigger

    # cat /sys/kernel/tracing/events/syscalls/sys_enter_read/hist
    # trigger info: hist:keys=common_pid.execname:vals=count:sort=count.descending:size=2048 [active]

    { common_pid: gnome-terminal  [      3196] } hitcount:        280  count:    1093512
    { common_pid: Xorg            [      1309] } hitcount:        525  count:     256640
    { common_pid: compiz          [      2889] } hitcount:         59  count:     254400
    { common_pid: bash            [      8710] } hitcount:          3  count:      66369
    { common_pid: dbus-daemon-lau [      8703] } hitcount:         49  count:      47739
    { common_pid: irqbalance      [      1252] } hitcount:         27  count:      27648
    { common_pid: 01ifupdown      [      8705] } hitcount:          3  count:      17216
    { common_pid: dbus-daemon     [       772] } hitcount:         10  count:      12396
    { common_pid: Socket Thread   [      8342] } hitcount:         11  count:      11264
    { common_pid: nm-dhcp-client. [      8701] } hitcount:          6  count:       7424
    { common_pid: gmain           [      1315] } hitcount:         18  count:       6336
    .
    .
    .
    { common_pid: postgres        [      1892] } hitcount:          2  count:         32
    { common_pid: postgres        [      1891] } hitcount:          2  count:         32
    { common_pid: gmain           [      8704] } hitcount:          2  count:         32
    { common_pid: upstart-dbus-br [      2740] } hitcount:         21  count:         21
    { common_pid: nm-dispatcher.a [      8696] } hitcount:          1  count:         16
    { common_pid: indicator-datet [      2904] } hitcount:          1  count:         16
    { common_pid: gdbus           [      2998] } hitcount:          1  count:         16
    { common_pid: rtkit-daemon    [      2052] } hitcount:          1  count:          8
    { common_pid: init            [         1] } hitcount:          2  count:          2

    Totals:
        Hits: 2116
        Entries: 51
        Dropped: 0

  Similarly, if you key a hist trigger on syscall id, for example to
  gather and display a list of systemwide syscall hits, you can use
  the special .syscall modifier to display the syscall names rather
  than raw ids.  The example below keeps a running total of syscall
  counts for the system during the run::

    # echo 'hist:key=id.syscall:val=hitcount' > \
           /sys/kernel/tracing/events/raw_syscalls/sys_enter/trigger

    # cat /sys/kernel/tracing/events/raw_syscalls/sys_enter/hist
    # trigger info: hist:keys=id.syscall:vals=hitcount:sort=hitcount:size=2048 [active]

    { id: sys_fsync                     [ 74] } hitcount:          1
    { id: sys_newuname                  [ 63] } hitcount:          1
    { id: sys_prctl                     [157] } hitcount:          1
    { id: sys_statfs                    [137] } hitcount:          1
    { id: sys_symlink                   [ 88] } hitcount:          1
    { id: sys_sendmmsg                  [307] } hitcount:          1
    { id: sys_semctl                    [ 66] } hitcount:          1
    { id: sys_readlink                  [ 89] } hitcount:          3
    { id: sys_bind                      [ 49] } hitcount:          3
    { id: sys_getsockname               [ 51] } hitcount:          3
    { id: sys_unlink                    [ 87] } hitcount:          3
    { id: sys_rename                    [ 82] } hitcount:          4
    { id: unknown_syscall               [ 58] } hitcount:          4
    { id: sys_connect                   [ 42] } hitcount:          4
    { id: sys_getpid                    [ 39] } hitcount:          4
    .
    .
    .
    { id: sys_rt_sigprocmask            [ 14] } hitcount:        952
    { id: sys_futex                     [202] } hitcount:       1534
    { id: sys_write                     [  1] } hitcount:       2689
    { id: sys_setitimer                 [ 38] } hitcount:       2797
    { id: sys_read                      [  0] } hitcount:       3202
    { id: sys_select                    [ 23] } hitcount:       3773
    { id: sys_writev                    [ 20] } hitcount:       4531
    { id: sys_poll                      [  7] } hitcount:       8314
    { id: sys_recvmsg                   [ 47] } hitcount:      13738
    { id: sys_ioctl                     [ 16] } hitcount:      21843

    Totals:
        Hits: 67612
        Entries: 72
        Dropped: 0

  The syscall counts above provide a rough overall picture of system
  call activity on the system; we can see for example that the most
  popular system call on this system was the 'sys_ioctl' system call.

  We can use 'compound' keys to refine that number and provide some
  further insight as to which processes exactly contribute to the
  overall ioctl count.

  The command below keeps a hitcount for every unique combination of
  system call id and pid - the end result is essentially a table
  that keeps a per-pid sum of system call hits.  The results are
  sorted using the system call id as the primary key, and the
  hitcount sum as the secondary key::

    # echo 'hist:key=id.syscall,common_pid.execname:val=hitcount:sort=id,hitcount' > \
           /sys/kernel/tracing/events/raw_syscalls/sys_enter/trigger

    # cat /sys/kernel/tracing/events/raw_syscalls/sys_enter/hist
    # trigger info: hist:keys=id.syscall,common_pid.execname:vals=hitcount:sort=id.syscall,hitcount:size=2048 [active]

    { id: sys_read                      [  0], common_pid: rtkit-daemon    [      1877] } hitcount:          1
    { id: sys_read                      [  0], common_pid: gdbus           [      2976] } hitcount:          1
    { id: sys_read                      [  0], common_pid: console-kit-dae [      3400] } hitcount:          1
    { id: sys_read                      [  0], common_pid: postgres        [      1865] } hitcount:          1
    { id: sys_read                      [  0], common_pid: deja-dup-monito [      3543] } hitcount:          2
    { id: sys_read                      [  0], common_pid: NetworkManager  [       890] } hitcount:          2
    { id: sys_read                      [  0], common_pid: evolution-calen [      3048] } hitcount:          2
    { id: sys_read                      [  0], common_pid: postgres        [      1864] } hitcount:          2
    { id: sys_read                      [  0], common_pid: nm-applet       [      3022] } hitcount:          2
    { id: sys_read                      [  0], common_pid: whoopsie        [      1212] } hitcount:          2
    .
    .
    .
    { id: sys_ioctl                     [ 16], common_pid: bash            [      8479] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: bash            [      3472] } hitcount:         12
    { id: sys_ioctl                     [ 16], common_pid: gnome-terminal  [      3199] } hitcount:         16
    { id: sys_ioctl                     [ 16], common_pid: Xorg            [      1267] } hitcount:       1808
    { id: sys_ioctl                     [ 16], common_pid: compiz          [      2994] } hitcount:       5580
    .
    .
    .
    { id: sys_waitid                    [247], common_pid: upstart-dbus-br [      2690] } hitcount:          3
    { id: sys_waitid                    [247], common_pid: upstart-dbus-br [      2688] } hitcount:         16
    { id: sys_inotify_add_watch         [254], common_pid: gmain           [       975] } hitcount:          2
    { id: sys_inotify_add_watch         [254], common_pid: gmain           [      3204] } hitcount:          4
    { id: sys_inotify_add_watch         [254], common_pid: gmain           [      2888] } hitcount:          4
    { id: sys_inotify_add_watch         [254], common_pid: gmain           [      3003] } hitcount:          4
    { id: sys_inotify_add_watch         [254], common_pid: gmain           [      2873] } hitcount:          4
    { id: sys_inotify_add_watch         [254], common_pid: gmain           [      3196] } hitcount:          6
    { id: sys_openat                    [257], common_pid: java            [      2623] } hitcount:          2
    { id: sys_eventfd2                  [290], common_pid: ibus-ui-gtk3    [      2760] } hitcount:          4
    { id: sys_eventfd2                  [290], common_pid: compiz          [      2994] } hitcount:          6

    Totals:
        Hits: 31536
        Entries: 323
        Dropped: 0

  The above list does give us a breakdown of the ioctl syscall by
  pid, but it also gives us quite a bit more than that, which we
  don't really care about at the moment.  Since we know the syscall
  id for sys_ioctl (16, displayed next to the sys_ioctl name), we
  can use that to filter out all the other syscalls::

    # echo 'hist:key=id.syscall,common_pid.execname:val=hitcount:sort=id,hitcount if id == 16' > \
           /sys/kernel/tracing/events/raw_syscalls/sys_enter/trigger

    # cat /sys/kernel/tracing/events/raw_syscalls/sys_enter/hist
    # trigger info: hist:keys=id.syscall,common_pid.execname:vals=hitcount:sort=id.syscall,hitcount:size=2048 if id == 16 [active]

    { id: sys_ioctl                     [ 16], common_pid: gmain           [      2769] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: evolution-addre [      8571] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: gmain           [      3003] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: gmain           [      2781] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: gmain           [      2829] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: bash            [      8726] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: bash            [      8508] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: gmain           [      2970] } hitcount:          1
    { id: sys_ioctl                     [ 16], common_pid: gmain           [      2768] } hitcount:          1
    .
    .
    .
    { id: sys_ioctl                     [ 16], common_pid: pool            [      8559] } hitcount:         45
    { id: sys_ioctl                     [ 16], common_pid: pool            [      8555] } hitcount:         48
    { id: sys_ioctl                     [ 16], common_pid: pool            [      8551] } hitcount:         48
    { id: sys_ioctl                     [ 16], common_pid: avahi-daemon    [       896] } hitcount:         66
    { id: sys_ioctl                     [ 16], common_pid: Xorg            [      1267] } hitcount:      26674
    { id: sys_ioctl                     [ 16], common_pid: compiz          [      2994] } hitcount:      73443

    Totals:
        Hits: 101162
        Entries: 103
        Dropped: 0

  The above output shows that 'compiz' and 'Xorg' are far and away
  the heaviest ioctl callers (which might lead to questions about
  whether they really need to be making all those calls and to
  possible avenues for further investigation.)

  The compound key examples used a key and a sum value (hitcount) to
  sort the output, but we can just as easily use two keys instead.
  Here's an example where we use a compound key composed of the
  common_pid and size event fields.  Sorting with pid as the primary
  key and 'size' as the secondary key allows us to display an
  ordered summary of the recvfrom sizes, with counts, received by
  each process::

    # echo 'hist:key=common_pid.execname,size:val=hitcount:sort=common_pid,size' > \
           /sys/kernel/tracing/events/syscalls/sys_enter_recvfrom/trigger

    # cat /sys/kernel/tracing/events/syscalls/sys_enter_recvfrom/hist
    # trigger info: hist:keys=common_pid.execname,size:vals=hitcount:sort=common_pid.execname,size:size=2048 [active]

    { common_pid: smbd            [       784], size:          4 } hitcount:          1
    { common_pid: dnsmasq         [      1412], size:       4096 } hitcount:        672
    { common_pid: postgres        [      1796], size:       1000 } hitcount:          6
    { common_pid: postgres        [      1867], size:       1000 } hitcount:         10
    { common_pid: bamfdaemon      [      2787], size:         28 } hitcount:          2
    { common_pid: bamfdaemon      [      2787], size:      14360 } hitcount:          1
    { common_pid: compiz          [      2994], size:          8 } hitcount:          1
    { common_pid: compiz          [      2994], size:         20 } hitcount:         11
    { common_pid: gnome-terminal  [      3199], size:          4 } hitcount:          2
    { common_pid: firefox         [      8817], size:          4 } hitcount:          1
    { common_pid: firefox         [      8817], size:          8 } hitcount:          5
    { common_pid: firefox         [      8817], size:        588 } hitcount:          2
    { common_pid: firefox         [      8817], size:        628 } hitcount:          1
    { common_pid: firefox         [      8817], size:       6944 } hitcount:          1
    { common_pid: firefox         [      8817], size:     408880 } hitcount:          2
    { common_pid: firefox         [      8822], size:          8 } hitcount:          2
    { common_pid: firefox         [      8822], size:        160 } hitcount:          2
    { common_pid: firefox         [      8822], size:        320 } hitcount:          2
    { common_pid: firefox         [      8822], size:        352 } hitcount:          1
    .
    .
    .
    { common_pid: pool            [      8923], size:       1960 } hitcount:         10
    { common_pid: pool            [      8923], size:       2048 } hitcount:         10
    { common_pid: pool            [      8924], size:       1960 } hitcount:         10
    { common_pid: pool            [      8924], size:       2048 } hitcount:         10
    { common_pid: pool            [      8928], size:       1964 } hitcount:          4
    { common_pid: pool            [      8928], size:       1965 } hitcount:          2
    { common_pid: pool            [      8928], size:       2048 } hitcount:          6
    { common_pid: pool            [      8929], size:       1982 } hitcount:          1
    { common_pid: pool            [      8929], size:       2048 } hitcount:          1

    Totals:
        Hits: 2016
        Entries: 224
        Dropped: 0

  The above example also illustrates the fact that although a compound
  key is treated as a single entity for hashing purposes, the sub-keys
  it's composed of can be accessed independently.

  The next example uses a string field as the hash key and
  demonstrates how you can manually pause and continue a hist trigger.
  In this example, we'll aggregate fork counts and don't expect a
  large number of entries in the hash table, so we'll drop it to a
  much smaller number, say 256::

    # echo 'hist:key=child_comm:val=hitcount:size=256' > \
           /sys/kernel/tracing/events/sched/sched_process_fork/trigger

    # cat /sys/kernel/tracing/events/sched/sched_process_fork/hist
    # trigger info: hist:keys=child_comm:vals=hitcount:sort=hitcount:size=256 [active]

    { child_comm: dconf worker                        } hitcount:          1
    { child_comm: ibus-daemon                         } hitcount:          1
    { child_comm: whoopsie                            } hitcount:          1
    { child_comm: smbd                                } hitcount:          1
    { child_comm: gdbus                               } hitcount:          1
    { child_comm: kthreadd                            } hitcount:          1
    { child_comm: dconf worker                        } hitcount:          1
    { child_comm: evolution-alarm                     } hitcount:          2
    { child_comm: Socket Thread                       } hitcount:          2
    { child_comm: postgres                            } hitcount:          2
    { child_comm: bash                                } hitcount:          3
    { child_comm: compiz                              } hitcount:          3
    { child_comm: evolution-sourc                     } hitcount:          4
    { child_comm: dhclient                            } hitcount:          4
    { child_comm: pool                                } hitcount:          5
    { child_comm: nm-dispatcher.a                     } hitcount:          8
    { child_comm: firefox                             } hitcount:          8
    { child_comm: dbus-daemon                         } hitcount:          8
    { child_comm: glib-pacrunner                      } hitcount:         10
    { child_comm: evolution                           } hitcount:         23

    Totals:
        Hits: 89
        Entries: 20
        Dropped: 0

  If we want to pause the hist trigger, we can simply append :pause to
  the command that started the trigger.  Notice that the trigger info
  displays as [paused]::

    # echo 'hist:key=child_comm:val=hitcount:size=256:pause' >> \
           /sys/kernel/tracing/events/sched/sched_process_fork/trigger

    # cat /sys/kernel/tracing/events/sched/sched_process_fork/hist
    # trigger info: hist:keys=child_comm:vals=hitcount:sort=hitcount:size=256 [paused]

    { child_comm: dconf worker                        } hitcount:          1
    { child_comm: kthreadd                            } hitcount:          1
    { child_comm: dconf worker                        } hitcount:          1
    { child_comm: gdbus                               } hitcount:          1
    { child_comm: ibus-daemon                         } hitcount:          1
    { child_comm: Socket Thread                       } hitcount:          2
    { child_comm: evolution-alarm                     } hitcount:          2
    { child_comm: smbd                                } hitcount:          2
    { child_comm: bash                                } hitcount:          3
    { child_comm: whoopsie                            } hitcount:          3
    { child_comm: compiz                              } hitcount:          3
    { child_comm: evolution-sourc                     } hitcount:          4
    { child_comm: pool                                } hitcount:          5
    { child_comm: postgres                            } hitcount:          6
    { child_comm: firefox                             } hitcount:          8
    { child_comm: dhclient                            } hitcount:         10
    { child_comm: emacs                               } hitcount:         12
    { child_comm: dbus-daemon                         } hitcount:         20
    { child_comm: nm-dispatcher.a                     } hitcount:         20
    { child_comm: evolution                           } hitcount:         35
    { child_comm: glib-pacrunner                      } hitcount:         59

    Totals:
        Hits: 199
        Entries: 21
        Dropped: 0

  To manually continue having the trigger aggregate events, append
  :cont instead.  Notice that the trigger info displays as [active]
  again, and the data has changed::

    # echo 'hist:key=child_comm:val=hitcount:size=256:cont' >> \
           /sys/kernel/tracing/events/sched/sched_process_fork/trigger

    # cat /sys/kernel/tracing/events/sched/sched_process_fork/hist
    # trigger info: hist:keys=child_comm:vals=hitcount:sort=hitcount:size=256 [active]

    { child_comm: dconf worker                        } hitcount:          1
    { child_comm: dconf worker                        } hitcount:          1
    { child_comm: kthreadd                            } hitcount:          1
    { child_comm: gdbus                               } hitcount:          1
    { child_comm: ibus-daemon                         } hitcount:          1
    { child_comm: Socket Thread                       } hitcount:          2
    { child_comm: evolution-alarm                     } hitcount:          2
    { child_comm: smbd                                } hitcount:          2
    { child_comm: whoopsie                            } hitcount:          3
    { child_comm: compiz                              } hitcount:          3
    { child_comm: evolution-sourc                     } hitcount:          4
    { child_comm: bash                                } hitcount:          5
    { child_comm: pool                                } hitcount:          5
    { child_comm: postgres                            } hitcount:          6
    { child_comm: firefox                             } hitcount:          8
    { child_comm: dhclient                            } hitcount:         11
    { child_comm: emacs                               } hitcount:         12
    { child_comm: dbus-daemon                         } hitcount:         22
    { child_comm: nm-dispatcher.a                     } hitcount:         22
    { child_comm: evolution                           } hitcount:         35
    { child_comm: glib-pacrunner                      } hitcount:         59

    Totals:
        Hits: 206
        Entries: 21
        Dropped: 0

  The previous example showed how to start and stop a hist trigger by
  appending 'pause' and 'continue' to the hist trigger command.  A
  hist trigger can also be started in a paused state by initially
  starting the trigger with ':pause' appended.  This allows you to
  start the trigger only when you're ready to start collecting data
  and not before.  For example, you could start the trigger in a
  paused state, then unpause it and do something you want to measure,
  then pause the trigger again when done.

  Of course, doing this manually can be difficult and error-prone, but
  it is possible to automatically start and stop a hist trigger based
  on some condition, via the enable_hist and disable_hist triggers.

  For example, suppose we wanted to take a look at the relative
  weights in terms of skb length for each callpath that leads to a
  netif_receive_skb event when downloading a decent-sized file using
  wget.

  First we set up an initially paused stacktrace trigger on the
  netif_receive_skb event::

    # echo 'hist:key=common_stacktrace:vals=len:pause' > \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger

  Next, we set up an 'enable_hist' trigger on the sched_process_exec
  event, with an 'if filename==/usr/bin/wget' filter.  The effect of
  this new trigger is that it will 'unpause' the hist trigger we just
  set up on netif_receive_skb if and only if it sees a
  sched_process_exec event with a filename of '/usr/bin/wget'.  When
  that happens, all netif_receive_skb events are aggregated into a
  hash table keyed on stacktrace::

    # echo 'enable_hist:net:netif_receive_skb if filename==/usr/bin/wget' > \
           /sys/kernel/tracing/events/sched/sched_process_exec/trigger

  The aggregation continues until the netif_receive_skb is paused
  again, which is what the following disable_hist event does by
  creating a similar setup on the sched_process_exit event, using the
  filter 'comm==wget'::

    # echo 'disable_hist:net:netif_receive_skb if comm==wget' > \
           /sys/kernel/tracing/events/sched/sched_process_exit/trigger

  Whenever a process exits and the comm field of the disable_hist
  trigger filter matches 'comm==wget', the netif_receive_skb hist
  trigger is disabled.

  The overall effect is that netif_receive_skb events are aggregated
  into the hash table for only the duration of the wget.  Executing a
  wget command and then listing the 'hist' file will display the
  output generated by the wget command::

    $ wget https://www.kernel.org/pub/linux/kernel/v3.x/patch-3.19.xz

    # cat /sys/kernel/tracing/events/net/netif_receive_skb/hist
    # trigger info: hist:keys=common_stacktrace:vals=len:sort=hitcount:size=2048 [paused]

    { common_stacktrace:
         __netif_receive_skb_core+0x46d/0x990
         __netif_receive_skb+0x18/0x60
         netif_receive_skb_internal+0x23/0x90
         napi_gro_receive+0xc8/0x100
         ieee80211_deliver_skb+0xd6/0x270 [mac80211]
         ieee80211_rx_handlers+0xccf/0x22f0 [mac80211]
         ieee80211_prepare_and_rx_handle+0x4e7/0xc40 [mac80211]
         ieee80211_rx+0x31d/0x900 [mac80211]
         iwlagn_rx_reply_rx+0x3db/0x6f0 [iwldvm]
         iwl_rx_dispatch+0x8e/0xf0 [iwldvm]
         iwl_pcie_irq_handler+0xe3c/0x12f0 [iwlwifi]
         irq_thread_fn+0x20/0x50
         irq_thread+0x11f/0x150
         kthread+0xd2/0xf0
         ret_from_fork+0x42/0x70
    } hitcount:         85  len:      28884
    { common_stacktrace:
         __netif_receive_skb_core+0x46d/0x990
         __netif_receive_skb+0x18/0x60
         netif_receive_skb_internal+0x23/0x90
         napi_gro_complete+0xa4/0xe0
         dev_gro_receive+0x23a/0x360
         napi_gro_receive+0x30/0x100
         ieee80211_deliver_skb+0xd6/0x270 [mac80211]
         ieee80211_rx_handlers+0xccf/0x22f0 [mac80211]
         ieee80211_prepare_and_rx_handle+0x4e7/0xc40 [mac80211]
         ieee80211_rx+0x31d/0x900 [mac80211]
         iwlagn_rx_reply_rx+0x3db/0x6f0 [iwldvm]
         iwl_rx_dispatch+0x8e/0xf0 [iwldvm]
         iwl_pcie_irq_handler+0xe3c/0x12f0 [iwlwifi]
         irq_thread_fn+0x20/0x50
         irq_thread+0x11f/0x150
         kthread+0xd2/0xf0
    } hitcount:         98  len:     664329
    { common_stacktrace:
         __netif_receive_skb_core+0x46d/0x990
         __netif_receive_skb+0x18/0x60
         process_backlog+0xa8/0x150
         net_rx_action+0x15d/0x340
         __do_softirq+0x114/0x2c0
         do_softirq_own_stack+0x1c/0x30
         do_softirq+0x65/0x70
         __local_bh_enable_ip+0xb5/0xc0
         ip_finish_output+0x1f4/0x840
         ip_output+0x6b/0xc0
         ip_local_out_sk+0x31/0x40
         ip_send_skb+0x1a/0x50
         udp_send_skb+0x173/0x2a0
         udp_sendmsg+0x2bf/0x9f0
         inet_sendmsg+0x64/0xa0
         sock_sendmsg+0x3d/0x50
    } hitcount:        115  len:      13030
    { common_stacktrace:
         __netif_receive_skb_core+0x46d/0x990
         __netif_receive_skb+0x18/0x60
         netif_receive_skb_internal+0x23/0x90
         napi_gro_complete+0xa4/0xe0
         napi_gro_flush+0x6d/0x90
         iwl_pcie_irq_handler+0x92a/0x12f0 [iwlwifi]
         irq_thread_fn+0x20/0x50
         irq_thread+0x11f/0x150
         kthread+0xd2/0xf0
         ret_from_fork+0x42/0x70
    } hitcount:        934  len:    5512212

    Totals:
        Hits: 1232
        Entries: 4
        Dropped: 0

  The above shows all the netif_receive_skb callpaths and their total
  lengths for the duration of the wget command.

  The 'clear' hist trigger param can be used to clear the hash table.
  Suppose we wanted to try another run of the previous example but
  this time also wanted to see the complete list of events that went
  into the histogram.  In order to avoid having to set everything up
  again, we can just clear the histogram first::

    # echo 'hist:key=common_stacktrace:vals=len:clear' >> \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger

  Just to verify that it is in fact cleared, here's what we now see in
  the hist file::

    # cat /sys/kernel/tracing/events/net/netif_receive_skb/hist
    # trigger info: hist:keys=common_stacktrace:vals=len:sort=hitcount:size=2048 [paused]

    Totals:
        Hits: 0
        Entries: 0
        Dropped: 0

  Since we want to see the detailed list of every netif_receive_skb
  event occurring during the new run, which are in fact the same
  events being aggregated into the hash table, we add some additional
  'enable_event' events to the triggering sched_process_exec and
  sched_process_exit events as such::

    # echo 'enable_event:net:netif_receive_skb if filename==/usr/bin/wget' > \
           /sys/kernel/tracing/events/sched/sched_process_exec/trigger

    # echo 'disable_event:net:netif_receive_skb if comm==wget' > \
           /sys/kernel/tracing/events/sched/sched_process_exit/trigger

  If you read the trigger files for the sched_process_exec and
  sched_process_exit triggers, you should see two triggers for each:
  one enabling/disabling the hist aggregation and the other
  enabling/disabling the logging of events::

    # cat /sys/kernel/tracing/events/sched/sched_process_exec/trigger
    enable_event:net:netif_receive_skb:unlimited if filename==/usr/bin/wget
    enable_hist:net:netif_receive_skb:unlimited if filename==/usr/bin/wget

    # cat /sys/kernel/tracing/events/sched/sched_process_exit/trigger
    enable_event:net:netif_receive_skb:unlimited if comm==wget
    disable_hist:net:netif_receive_skb:unlimited if comm==wget

  In other words, whenever either of the sched_process_exec or
  sched_process_exit events is hit and matches 'wget', it enables or
  disables both the histogram and the event log, and what you end up
  with is a hash table and set of events just covering the specified
  duration.  Run the wget command again::

    $ wget https://www.kernel.org/pub/linux/kernel/v3.x/patch-3.19.xz

  Displaying the 'hist' file should show something similar to what you
  saw in the last run, but this time you should also see the
  individual events in the trace file::

    # cat /sys/kernel/tracing/trace

    # tracer: nop
    #
    # entries-in-buffer/entries-written: 183/1426   #P:4
    #
    #                              _-----=> irqs-off
    #                             / _----=> need-resched
    #                            | / _---=> hardirq/softirq
    #                            || / _--=> preempt-depth
    #                            ||| /     delay
    #           TASK-PID   CPU#  ||||    TIMESTAMP  FUNCTION
    #              | |       |   ||||       |         |
                wget-15108 [000] ..s1 31769.606929: netif_receive_skb: dev=lo skbaddr=ffff88009c353100 len=60
                wget-15108 [000] ..s1 31769.606999: netif_receive_skb: dev=lo skbaddr=ffff88009c353200 len=60
             dnsmasq-1382  [000] ..s1 31769.677652: netif_receive_skb: dev=lo skbaddr=ffff88009c352b00 len=130
             dnsmasq-1382  [000] ..s1 31769.685917: netif_receive_skb: dev=lo skbaddr=ffff88009c352200 len=138
    ##### CPU 2 buffer started ####
      irq/29-iwlwifi-559   [002] ..s. 31772.031529: netif_receive_skb: dev=wlan0 skbaddr=ffff88009d433d00 len=2948
      irq/29-iwlwifi-559   [002] ..s. 31772.031572: netif_receive_skb: dev=wlan0 skbaddr=ffff88009d432200 len=1500
      irq/29-iwlwifi-559   [002] ..s. 31772.032196: netif_receive_skb: dev=wlan0 skbaddr=ffff88009d433100 len=2948
      irq/29-iwlwifi-559   [002] ..s. 31772.032761: netif_receive_skb: dev=wlan0 skbaddr=ffff88009d433000 len=2948
      irq/29-iwlwifi-559   [002] ..s. 31772.033220: netif_receive_skb: dev=wlan0 skbaddr=ffff88009d432e00 len=1500
    .
    .
    .

  The following example demonstrates how multiple hist triggers can be
  attached to a given event.  This capability can be useful for
  creating a set of different summaries derived from the same set of
  events, or for comparing the effects of different filters, among
  other things::

    # echo 'hist:keys=skbaddr.hex:vals=len if len < 0' >> \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger
    # echo 'hist:keys=skbaddr.hex:vals=len if len > 4096' >> \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger
    # echo 'hist:keys=skbaddr.hex:vals=len if len == 256' >> \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger
    # echo 'hist:keys=skbaddr.hex:vals=len' >> \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger
    # echo 'hist:keys=len:vals=common_preempt_count' >> \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger

  The above set of commands create four triggers differing only in
  their filters, along with a completely different though fairly
  nonsensical trigger.  Note that in order to append multiple hist
  triggers to the same file, you should use the '>>' operator to
  append them ('>' will also add the new hist trigger, but will remove
  any existing hist triggers beforehand).

  Displaying the contents of the 'hist' file for the event shows the
  contents of all five histograms::

    # cat /sys/kernel/tracing/events/net/netif_receive_skb/hist

    # event histogram
    #
    # trigger info: hist:keys=len:vals=hitcount,common_preempt_count:sort=hitcount:size=2048 [active]
    #

    { len:        176 } hitcount:          1  common_preempt_count:          0
    { len:        223 } hitcount:          1  common_preempt_count:          0
    { len:       4854 } hitcount:          1  common_preempt_count:          0
    { len:        395 } hitcount:          1  common_preempt_count:          0
    { len:        177 } hitcount:          1  common_preempt_count:          0
    { len:        446 } hitcount:          1  common_preempt_count:          0
    { len:       1601 } hitcount:          1  common_preempt_count:          0
    .
    .
    .
    { len:       1280 } hitcount:         66  common_preempt_count:          0
    { len:        116 } hitcount:         81  common_preempt_count:         40
    { len:        708 } hitcount:        112  common_preempt_count:          0
    { len:         46 } hitcount:        221  common_preempt_count:          0
    { len:       1264 } hitcount:        458  common_preempt_count:          0

    Totals:
        Hits: 1428
        Entries: 147
        Dropped: 0


    # event histogram
    #
    # trigger info: hist:keys=skbaddr.hex:vals=hitcount,len:sort=hitcount:size=2048 [active]
    #

    { skbaddr: ffff8800baee5e00 } hitcount:          1  len:        130
    { skbaddr: ffff88005f3d5600 } hitcount:          1  len:       1280
    { skbaddr: ffff88005f3d4900 } hitcount:          1  len:       1280
    { skbaddr: ffff88009fed6300 } hitcount:          1  len:        115
    { skbaddr: ffff88009fe0ad00 } hitcount:          1  len:        115
    { skbaddr: ffff88008cdb1900 } hitcount:          1  len:         46
    { skbaddr: ffff880064b5ef00 } hitcount:          1  len:        118
    { skbaddr: ffff880044e3c700 } hitcount:          1  len:         60
    { skbaddr: ffff880100065900 } hitcount:          1  len:         46
    { skbaddr: ffff8800d46bd500 } hitcount:          1  len:        116
    { skbaddr: ffff88005f3d5f00 } hitcount:          1  len:       1280
    { skbaddr: ffff880100064700 } hitcount:          1  len:        365
    { skbaddr: ffff8800badb6f00 } hitcount:          1  len:         60
    .
    .
    .
    { skbaddr: ffff88009fe0be00 } hitcount:         27  len:      24677
    { skbaddr: ffff88009fe0a400 } hitcount:         27  len:      23052
    { skbaddr: ffff88009fe0b700 } hitcount:         31  len:      25589
    { skbaddr: ffff88009fe0b600 } hitcount:         32  len:      27326
    { skbaddr: ffff88006a462800 } hitcount:         68  len:      71678
    { skbaddr: ffff88006a463700 } hitcount:         70  len:      72678
    { skbaddr: ffff88006a462b00 } hitcount:         71  len:      77589
    { skbaddr: ffff88006a463600 } hitcount:         73  len:      71307
    { skbaddr: ffff88006a462200 } hitcount:         81  len:      81032

    Totals:
        Hits: 1451
        Entries: 318
        Dropped: 0


    # event histogram
    #
    # trigger info: hist:keys=skbaddr.hex:vals=hitcount,len:sort=hitcount:size=2048 if len == 256 [active]
    #


    Totals:
        Hits: 0
        Entries: 0
        Dropped: 0


    # event histogram
    #
    # trigger info: hist:keys=skbaddr.hex:vals=hitcount,len:sort=hitcount:size=2048 if len > 4096 [active]
    #

    { skbaddr: ffff88009fd2c300 } hitcount:          1  len:       7212
    { skbaddr: ffff8800d2bcce00 } hitcount:          1  len:       7212
    { skbaddr: ffff8800d2bcd700 } hitcount:          1  len:       7212
    { skbaddr: ffff8800d2bcda00 } hitcount:          1  len:      21492
    { skbaddr: ffff8800ae2e2d00 } hitcount:          1  len:       7212
    { skbaddr: ffff8800d2bcdb00 } hitcount:          1  len:       7212
    { skbaddr: ffff88006a4df500 } hitcount:          1  len:       4854
    { skbaddr: ffff88008ce47b00 } hitcount:          1  len:      18636
    { skbaddr: ffff8800ae2e2200 } hitcount:          1  len:      12924
    { skbaddr: ffff88005f3e1000 } hitcount:          1  len:       4356
    { skbaddr: ffff8800d2bcdc00 } hitcount:          2  len:      24420
    { skbaddr: ffff8800d2bcc200 } hitcount:          2  len:      12996

    Totals:
        Hits: 14
        Entries: 12
        Dropped: 0


    # event histogram
    #
    # trigger info: hist:keys=skbaddr.hex:vals=hitcount,len:sort=hitcount:size=2048 if len < 0 [active]
    #


    Totals:
        Hits: 0
        Entries: 0
        Dropped: 0

  Named triggers can be used to have triggers share a common set of
  histogram data.  This capability is mostly useful for combining the
  output of events generated by tracepoints contained inside inline
  functions, but names can be used in a hist trigger on any event.
  For example, these two triggers when hit will update the same 'len'
  field in the shared 'foo' histogram data::

    # echo 'hist:name=foo:keys=skbaddr.hex:vals=len' > \
           /sys/kernel/tracing/events/net/netif_receive_skb/trigger
    # echo 'hist:name=foo:keys=skbaddr.hex:vals=len' > \
           /sys/kernel/tracing/events/net/netif_rx/trigger

  You can see that they're updating common histogram data by reading
  each event's hist files at the same time::

    # cat /sys/kernel/tracing/events/net/netif_receive_skb/hist;
      cat /sys/kernel/tracing/events/net/netif_rx/hist

    # event histogram
    #
    # trigger info: hist:name=foo:keys=skbaddr.hex:vals=hitcount,len:sort=hitcount:size=2048 [active]
    #

    { skbaddr: ffff88000ad53500 } hitcount:          1  len:         46
    { skbaddr: ffff8800af5a1500 } hitcount:          1  len:         76
    { skbaddr: ffff8800d62a1900 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bccb00 } hitcount:          1  len:        468
    { skbaddr: ffff8800d3c69900 } hitcount:          1  len:         46
    { skbaddr: ffff88009ff09100 } hitcount:          1  len:         52
    { skbaddr: ffff88010f13ab00 } hitcount:          1  len:        168
    { skbaddr: ffff88006a54f400 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bcc500 } hitcount:          1  len:        260
    { skbaddr: ffff880064505000 } hitcount:          1  len:         46
    { skbaddr: ffff8800baf24e00 } hitcount:          1  len:         32
    { skbaddr: ffff88009fe0ad00 } hitcount:          1  len:         46
    { skbaddr: ffff8800d3edff00 } hitcount:          1  len:         44
    { skbaddr: ffff88009fe0b400 } hitcount:          1  len:        168
    { skbaddr: ffff8800a1c55a00 } hitcount:          1  len:         40
    { skbaddr: ffff8800d2bcd100 } hitcount:          1  len:         40
    { skbaddr: ffff880064505f00 } hitcount:          1  len:        174
    { skbaddr: ffff8800a8bff200 } hitcount:          1  len:        160
    { skbaddr: ffff880044e3cc00 } hitcount:          1  len:         76
    { skbaddr: ffff8800a8bfe700 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bcdc00 } hitcount:          1  len:         32
    { skbaddr: ffff8800a1f64800 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bcde00 } hitcount:          1  len:        988
    { skbaddr: ffff88006a5dea00 } hitcount:          1  len:         46
    { skbaddr: ffff88002e37a200 } hitcount:          1  len:         44
    { skbaddr: ffff8800a1f32c00 } hitcount:          2  len:        676
    { skbaddr: ffff88000ad52600 } hitcount:          2  len:        107
    { skbaddr: ffff8800a1f91e00 } hitcount:          2  len:         92
    { skbaddr: ffff8800af5a0200 } hitcount:          2  len:        142
    { skbaddr: ffff8800d2bcc600 } hitcount:          2  len:        220
    { skbaddr: ffff8800ba36f500 } hitcount:          2  len:         92
    { skbaddr: ffff8800d021f800 } hitcount:          2  len:         92
    { skbaddr: ffff8800a1f33600 } hitcount:          2  len:        675
    { skbaddr: ffff8800a8bfff00 } hitcount:          3  len:        138
    { skbaddr: ffff8800d62a1300 } hitcount:          3  len:        138
    { skbaddr: ffff88002e37a100 } hitcount:          4  len:        184
    { skbaddr: ffff880064504400 } hitcount:          4  len:        184
    { skbaddr: ffff8800a8bfec00 } hitcount:          4  len:        184
    { skbaddr: ffff88000ad53700 } hitcount:          5  len:        230
    { skbaddr: ffff8800d2bcdb00 } hitcount:          5  len:        196
    { skbaddr: ffff8800a1f90000 } hitcount:          6  len:        276
    { skbaddr: ffff88006a54f900 } hitcount:          6  len:        276

    Totals:
        Hits: 81
        Entries: 42
        Dropped: 0
    # event histogram
    #
    # trigger info: hist:name=foo:keys=skbaddr.hex:vals=hitcount,len:sort=hitcount:size=2048 [active]
    #

    { skbaddr: ffff88000ad53500 } hitcount:          1  len:         46
    { skbaddr: ffff8800af5a1500 } hitcount:          1  len:         76
    { skbaddr: ffff8800d62a1900 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bccb00 } hitcount:          1  len:        468
    { skbaddr: ffff8800d3c69900 } hitcount:          1  len:         46
    { skbaddr: ffff88009ff09100 } hitcount:          1  len:         52
    { skbaddr: ffff88010f13ab00 } hitcount:          1  len:        168
    { skbaddr: ffff88006a54f400 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bcc500 } hitcount:          1  len:        260
    { skbaddr: ffff880064505000 } hitcount:          1  len:         46
    { skbaddr: ffff8800baf24e00 } hitcount:          1  len:         32
    { skbaddr: ffff88009fe0ad00 } hitcount:          1  len:         46
    { skbaddr: ffff8800d3edff00 } hitcount:          1  len:         44
    { skbaddr: ffff88009fe0b400 } hitcount:          1  len:        168
    { skbaddr: ffff8800a1c55a00 } hitcount:          1  len:         40
    { skbaddr: ffff8800d2bcd100 } hitcount:          1  len:         40
    { skbaddr: ffff880064505f00 } hitcount:          1  len:        174
    { skbaddr: ffff8800a8bff200 } hitcount:          1  len:        160
    { skbaddr: ffff880044e3cc00 } hitcount:          1  len:         76
    { skbaddr: ffff8800a8bfe700 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bcdc00 } hitcount:          1  len:         32
    { skbaddr: ffff8800a1f64800 } hitcount:          1  len:         46
    { skbaddr: ffff8800d2bcde00 } hitcount:          1  len:        988
    { skbaddr: ffff88006a5dea00 } hitcount:          1  len:         46
    { skbaddr: ffff88002e37a200 } hitcount:          1  len:         44
    { skbaddr: ffff8800a1f32c00 } hitcount:          2  len:        676
    { skbaddr: ffff88000ad52600 } hitcount:          2  len:        107
    { skbaddr: ffff8800a1f91e00 } hitcount:          2  len:         92
    { skbaddr: ffff8800af5a0200 } hitcount:          2  len:        142
    { skbaddr: ffff8800d2bcc600 } hitcount:          2  len:        220
    { skbaddr: ffff8800ba36f500 } hitcount:          2  len:         92
    { skbaddr: ffff8800d021f800 } hitcount:          2  len:         92
    { skbaddr: ffff8800a1f33600 } hitcount:          2  len:        675
    { skbaddr: ffff8800a8bfff00 } hitcount:          3  len:        138
    { skbaddr: ffff8800d62a1300 } hitcount:          3  len:        138
    { skbaddr: ffff88002e37a100 } hitcount:          4  len:        184
    { skbaddr: ffff880064504400 } hitcount:          4  len:        184
    { skbaddr: ffff8800a8bfec00 } hitcount:          4  len:        184
    { skbaddr: ffff88000ad53700 } hitcount:          5  len:        230
    { skbaddr: ffff8800d2bcdb00 } hitcount:          5  len:        196
    { skbaddr: ffff8800a1f90000 } hitcount:          6  len:        276
    { skbaddr: ffff88006a54f900 } hitcount:          6  len:        276

    Totals:
        Hits: 81
        Entries: 42
        Dropped: 0

  And here's an example that shows how to combine histogram data from
  any two events even if they don't share any 'compatible' fields
  other than 'hitcount' and 'common_stacktrace'.  These commands create a
  couple of triggers named 'bar' using those fields::

    # echo 'hist:name=bar:key=common_stacktrace:val=hitcount' > \
           /sys/kernel/tracing/events/sched/sched_process_fork/trigger
    # echo 'hist:name=bar:key=common_stacktrace:val=hitcount' > \
          /sys/kernel/tracing/events/net/netif_rx/trigger

  And displaying the output of either shows some interesting if
  somewhat confusing output::

    # cat /sys/kernel/tracing/events/sched/sched_process_fork/hist
    # cat /sys/kernel/tracing/events/net/netif_rx/hist

    # event histogram
    #
    # trigger info: hist:name=bar:keys=common_stacktrace:vals=hitcount:sort=hitcount:size=2048 [active]
    #

    { common_stacktrace:
             kernel_clone+0x18e/0x330
             kernel_thread+0x29/0x30
             kthreadd+0x154/0x1b0
             ret_from_fork+0x3f/0x70
    } hitcount:          1
    { common_stacktrace:
             netif_rx_internal+0xb2/0xd0
             netif_rx_ni+0x20/0x70
             dev_loopback_xmit+0xaa/0xd0
             ip_mc_output+0x126/0x240
             ip_local_out_sk+0x31/0x40
             igmp_send_report+0x1e9/0x230
             igmp_timer_expire+0xe9/0x120
             call_timer_fn+0x39/0xf0
             run_timer_softirq+0x1e1/0x290
             __do_softirq+0xfd/0x290
             irq_exit+0x98/0xb0
             smp_apic_timer_interrupt+0x4a/0x60
             apic_timer_interrupt+0x6d/0x80
             cpuidle_enter+0x17/0x20
             call_cpuidle+0x3b/0x60
             cpu_startup_entry+0x22d/0x310
    } hitcount:          1
    { common_stacktrace:
             netif_rx_internal+0xb2/0xd0
             netif_rx_ni+0x20/0x70
             dev_loopback_xmit+0xaa/0xd0
             ip_mc_output+0x17f/0x240
             ip_local_out_sk+0x31/0x40
             ip_send_skb+0x1a/0x50
             udp_send_skb+0x13e/0x270
             udp_sendmsg+0x2bf/0x980
             inet_sendmsg+0x67/0xa0
             sock_sendmsg+0x38/0x50
             SYSC_sendto+0xef/0x170
             SyS_sendto+0xe/0x10
             entry_SYSCALL_64_fastpath+0x12/0x6a
    } hitcount:          2
    { common_stacktrace:
             netif_rx_internal+0xb2/0xd0
             netif_rx+0x1c/0x60
             loopback_xmit+0x6c/0xb0
             dev_hard_start_xmit+0x219/0x3a0
             __dev_queue_xmit+0x415/0x4f0
             dev_queue_xmit_sk+0x13/0x20
             ip_finish_output2+0x237/0x340
             ip_finish_output+0x113/0x1d0
             ip_output+0x66/0xc0
             ip_local_out_sk+0x31/0x40
             ip_send_skb+0x1a/0x50
             udp_send_skb+0x16d/0x270
             udp_sendmsg+0x2bf/0x980
             inet_sendmsg+0x67/0xa0
             sock_sendmsg+0x38/0x50
             ___sys_sendmsg+0x14e/0x270
    } hitcount:         76
    { common_stacktrace:
             netif_rx_internal+0xb2/0xd0
             netif_rx+0x1c/0x60
             loopback_xmit+0x6c/0xb0
             dev_hard_start_xmit+0x219/0x3a0
             __dev_queue_xmit+0x415/0x4f0
             dev_queue_xmit_sk+0x13/0x20
             ip_finish_output2+0x237/0x340
             ip_finish_output+0x113/0x1d0
             ip_output+0x66/0xc0
             ip_local_out_sk+0x31/0x40
             ip_send_skb+0x1a/0x50
             udp_send_skb+0x16d/0x270
             udp_sendmsg+0x2bf/0x980
             inet_sendmsg+0x67/0xa0
             sock_sendmsg+0x38/0x50
             ___sys_sendmsg+0x269/0x270
    } hitcount:         77
    { common_stacktrace:
             netif_rx_internal+0xb2/0xd0
             netif_rx+0x1c/0x60
             loopback_xmit+0x6c/0xb0
             dev_hard_start_xmit+0x219/0x3a0
             __dev_queue_xmit+0x415/0x4f0
             dev_queue_xmit_sk+0x13/0x20
             ip_finish_output2+0x237/0x340
             ip_finish_output+0x113/0x1d0
             ip_output+0x66/0xc0
             ip_local_out_sk+0x31/0x40
             ip_send_skb+0x1a/0x50
             udp_send_skb+0x16d/0x270
             udp_sendmsg+0x2bf/0x980
             inet_sendmsg+0x67/0xa0
             sock_sendmsg+0x38/0x50
             SYSC_sendto+0xef/0x170
    } hitcount:         88
    { common_stacktrace:
             kernel_clone+0x18e/0x330
             SyS_clone+0x19/0x20
             entry_SYSCALL_64_fastpath+0x12/0x6a
    } hitcount:        244

    Totals:
        Hits: 489
        Entries: 7
        Dropped: 0

```
### 2.4. 浜嬩欢闂寸洿鏂瑰浘瑙﹀彂鍣?


浜嬩欢闂寸洿鏂瑰浘瑙﹀彂鍣ㄦ槸涓€绫?hist 瑙﹀彂鍣紝瀹冩妸涓€涓紙鎴栧涓級鍏朵粬浜嬩欢鐨勫€肩粍鍚堣捣鏉ワ紝
骞跺埄鐢ㄨ繖浜涙暟鎹垱寤虹洿鏂瑰浘銆備簨浠堕棿鐩存柟鍥剧殑 data 鍙嶈繃鏉ュ張鍙互浣滀负杩涗竴姝ョ粍鍚堢洿鏂瑰浘鐨?
鏉ユ簮锛屼粠鑰屽舰鎴愪竴鏉＄浉鍏崇殑鐩存柟鍥鹃摼锛岃繖瀵规煇浜涘簲鐢ㄥ緢閲嶈銆?

鍙互鐢ㄨ繖绉嶆柟寮忎娇鐢ㄧ殑浜嬩欢闂撮噺涓紝鏈€閲嶈鐨勪緥瀛愭槸寤惰繜锛坙atency锛夛紝瀹冨叾瀹炲氨鏄袱涓?
浜嬩欢涔嬮棿鏃堕棿鎴崇殑宸€笺€傚敖绠″欢杩熸槸鏈€閲嶈鐨勪簨浠堕棿閲忥紝浣嗚娉ㄦ剰锛岀敱浜庤鏀寔鍦ㄦ暣涓?
璺熻釜浜嬩欢瀛愮郴缁熶腑鏄畬鍏ㄩ€氱敤鐨勶紝鍥犳浠讳綍浜嬩欢瀛楁閮藉彲浠ョ敤浜庝簨浠堕棿閲忋€?

涓€涓皢鏉ヨ嚜鍏朵粬鐩存柟鍥剧殑鏁版嵁缁勫悎鎴愭湁鐢ㄩ摼鏉＄殑鐩存柟鍥句緥瀛愶紝鏄€渨akeupswitch latency鈥?
鐩存柟鍥撅紝瀹冩妸鈥渨akeup latency鈥濈洿鏂瑰浘涓庘€渟witch latency鈥濈洿鏂瑰浘缁勫悎鍦ㄤ竴璧枫€?

閫氬父锛屼竴涓?hist 瑙﹀彂鍣ㄨ鏍艰鏄庡寘鍚竴涓紙鍙兘涓哄鍚堢殑锛夐敭锛屼互鍙婁竴鎴栧涓暟鍊硷紝
杩欎簺鏁板€兼槸涓庤閿叧鑱斻€佹寔缁洿鏂扮殑鎬诲拰銆傚湪杩欑鎯呭喌涓嬶紝鐩存柟鍥捐鏍艰鏄庣敱鍗曚釜閿笌
鍊肩殑瑙勬牸缁勬垚锛屽畠浠紩鐢ㄤ笌鍗曚竴浜嬩欢绫诲瀷鍏宠仈鐨勮窡韪簨浠跺瓧娈点€?

浜嬩欢闂寸洿鏂瑰浘瑙﹀彂鍣ㄦ墿灞曞厑璁稿紩鐢ㄦ潵鑷涓簨浠剁殑瀛楁锛屽苟灏嗗叾缁勫悎涓轰竴涓浜嬩欢鐩存柟鍥?
瑙勬牸璇存槑銆備负浜嗘敮鎸佽繖涓€鎬讳綋鐩爣锛屽悜 hist 瑙﹀彂鍣ㄦ敮鎸佷腑鏂板浜嗚嫢骞蹭娇鑳界壒鎬э細

  - 涓轰簡璁＄畻浜嬩欢闂撮噺锛岄渶瑕佹妸涓€涓簨浠朵腑鐨勫€间繚瀛樹笅鏉ワ紝鐒跺悗鍐嶇敱鍙︿竴涓簨浠跺紩鐢ㄣ€?
    杩欏氨瑕佹眰寮曞叆瀵圭洿鏂瑰浘鈥滃彉閲忥紙variables锛夆€濈殑鏀寔銆?

  - 浜嬩欢闂撮噺鐨勮绠楀強鍏剁粍鍚堬紝瑕佹眰瀵瑰彉閲忓簲鐢ㄧ畝鍗曡〃杈惧紡锛堝姞鍜屽噺锛夋彁渚涙渶鍩烘湰鐨勬敮鎸併€?

  - 鐢变簨浠堕棿閲忔瀯鎴愮殑鐩存柟鍥撅紝鍦ㄩ€昏緫涓婂苟涓嶆槸浠讳竴浜嬩欢鐨勭洿鏂瑰浘锛堝洜姝よ浠讳竴浜嬩欢鐨?
    'hist' 鏂囦欢鏉ユ壙杞界洿鏂瑰浘杈撳嚭骞朵笉鍚堢悊锛夈€備负浜嗕綋鐜拌鐩存柟鍥句笌涓€缁勪簨浠剁殑缁勫悎鐩稿叧鑱?
    杩欎竴姒傚康锛屾柊澧炰簡鏀寔浠ュ厑璁稿垱寤衡€滃悎鎴愶紙synthetic锛夆€濅簨浠讹紝鍗充粠鍏朵粬浜嬩欢娲剧敓鍑烘潵鐨?
    浜嬩欢銆傝繖浜涘悎鎴愪簨浠朵笌浠讳綍鍏朵粬浜嬩欢涓€鏍锋槸瀹屽鐨勪簨浠讹紝鍙互鎸夋鏂瑰紡浣跨敤锛屼緥濡傜敤鏉?
    鍒涘缓鍓嶉潰鎻愬埌鐨勨€滅粍鍚堚€濈洿鏂瑰浘銆?

  - 涓€缁勨€滃姩浣滐紙actions锛夆€濆彲浠ヤ笌鐩存柟鍥炬潯鐩浉鍏宠仈鈥斺€旇繖浜涘姩浣滄棦鍙互鐢ㄦ潵鐢熸垚鍓嶉潰
    鎻愬埌鐨勫悎鎴愪簨浠讹紝涔熷彲浠ョ敤浜庡叾浠栫洰鐨勶紝渚嬪褰撳懡涓煇涓€滄渶澶э紙max锛夆€濆欢杩熸椂淇濆瓨
    涓婁笅鏂囥€?

  - 璺熻釜浜嬩欢鏈韩骞朵笉甯︽湁涓庝箣鍏宠仈鐨勨€滄椂闂存埑鈥濓紝浣嗗湪搴曞眰鐨?ftrace 鐜舰缂撳啿鍖轰腑锛?
    涓庢瘡涓簨浠朵竴璧蜂繚瀛樹簡涓€涓殣寮忕殑鏃堕棿鎴炽€傝鏃堕棿鎴崇幇鍦ㄤ互涓€涓悕涓?'common_timestamp'
    鐨勫悎鎴愬瓧娈电殑褰㈠紡鏆撮湶鍑烘潵锛屽彲浠ュ儚鍏朵粬浠讳綍浜嬩欢瀛楁涓€鏍峰湪鐩存柟鍥句腑浣跨敤锛涘畠骞朵笉鏄?
    璺熻釜鏍煎紡涓殑鐪熷疄瀛楁锛岃€屾槸涓€涓悎鎴愬嚭鏉ョ殑鍊硷紝灏界濡傛浠嶅彲浠ュ儚鐪熷疄瀛楁涓€鏍蜂娇鐢ㄣ€?
    榛樿鎯呭喌涓嬪叾鍗曚綅涓虹撼绉掞紱鍦?common_timestamp 瀛楁鍚庨檮鍔?'.usecs' 鍙皢鍗曚綅鏀逛负寰銆?

鍏充簬浜嬩欢闂存椂闂存埑鐨勬敞鎰忎簨椤癸細濡傛灉鍦ㄧ洿鏂瑰浘涓娇鐢ㄤ簡 common_timestamp锛岃窡韪紦鍐插尯浼?
鑷姩鍒囨崲涓轰娇鐢ㄧ粷瀵规椂闂存埑鍜屸€済lobal鈥濊窡韪椂閽燂紝浠ラ伩鍏嶄笌鍏朵粬鍦ㄨ法 CPU 鏃朵笉杩炵画鐨勬椂閽?
涔嬮棿鍑虹幇铏氬亣鐨勬椂闂存埑宸紓銆備篃鍙互閫氳繃鏀圭敤鍏朵粬璺熻釜鏃堕挓鏉ヨ鐩栬繖涓€琛屼负锛屽嵆浣跨敤
"clock=XXX" hist 瑙﹀彂鍣ㄥ睘鎬э紝鍏朵腑 XXX 鏄?tracing/trace_clock 浼枃浠朵腑鍒楀嚭鐨?
浠讳竴鏃堕挓銆?

杩欎簺鐗规€у湪鍚庣画鍚勮妭涓湁鏇磋缁嗙殑璇存槑銆?

### 2.5. 鐩存柟鍥惧彉閲?


鍙橀噺灏辨槸绠€鍗曠殑鍛藉悕浣嶇疆锛岀敤浜庡湪鍖归厤鐨勪簨浠朵箣闂翠繚瀛樺拰妫€绱㈠€笺€傛墍璋撯€滃尮閰嶏紙matching锛夆€?
浜嬩欢锛屾槸鎸囨嫢鏈夊尮閰嶉敭鐨勪簨浠垛€斺€斿鏋滀负鏌愪釜瀵瑰簲浜庤閿殑鐩存柟鍥炬潯鐩繚瀛樹簡涓€涓彉閲忥紝
閭ｄ箞浠讳綍鎷ユ湁鍖归厤閿殑鍚庣画浜嬩欢閮藉彲浠ヨ闂鍙橀噺銆?

鍙橀噺鐨勫€奸€氬父瀵逛换浣曞悗缁簨浠堕兘鍙敤锛岀洿鍒板畠琚煇涓悗缁簨浠惰涓哄叾浠栧€间负姝€傝瑙勫垯
鍞竴鐨勪緥澶栨槸锛氫换浣曞湪琛ㄨ揪寮忎腑浣跨敤鐨勫彉閲忔湰璐ㄤ笂閮芥槸鈥滆涓€娆★紙read-once锛夆€濈殑鈥斺€斾竴鏃?
瀹冭鍚庣画浜嬩欢涓殑鏌愪釜琛ㄨ揪寮忎娇鐢紝灏变細琚噸缃负鈥滄湭璁剧疆锛坲nset锛夆€濈姸鎬侊紝杩欐剰鍛崇潃闄ら潪
鍐嶆璁剧疆锛屽惁鍒欎笉鑳藉啀娆′娇鐢ㄣ€傝繖涓嶄粎纭繚浜嬩欢涓嶄細鍦ㄨ绠椾腑浣跨敤鏈垵濮嬪寲鐨勫彉閲忥紝涔熺‘淇?
璇ュ彉閲忓彧琚娇鐢ㄤ竴娆★紝鑰屼笉浼氱敤浜庝换浣曚笉鐩稿叧鐨勫悗缁尮閰嶃€?

淇濆瓨鍙橀噺鐨勫熀鏈娉曟槸锛氱畝鍗曞湴鎶婁竴涓笉瀵瑰簲浠讳綍鍏抽敭瀛楃殑鍞竴鍙橀噺鍚嶏紝杩炲悓 '=' 鍙蜂綔涓?
鍓嶇紑锛屽姞鍦ㄤ换鎰忎簨浠跺瓧娈典笂銆?

閿垨鍊奸兘鍙互鐢ㄨ繖绉嶆柟寮忎繚瀛樺拰妫€绱€傝繖浼氫负甯︽湁璇ラ敭鐨勭洿鏂瑰浘鏉＄洰鍒涘缓涓€涓悕涓?'ts0'
鐨勫彉閲?
```

  # echo 'hist:keys=next_pid:vals=$ts0:ts0=common_timestamp ... >> \
	event/trigger

```
ts0 鍙橀噺鍙互琚换浣曟嫢鏈変笌 'next_pid' 鐩稿悓 pid 鐨勫悗缁簨浠惰闂€?

鍙橀噺寮曠敤鏄€氳繃鍦ㄥ彉閲忓悕鍓嶅姞 '$' 绗﹀彿鏋勬垚鐨勩€傚洜姝わ紝渚嬪涓婇潰鐨?ts0 鍙橀噺鍦ㄨ〃杈惧紡涓?
浼氳寮曠敤涓?'$ts0'銆?

鐢变簬浣跨敤浜?'vals='锛屼笂闈㈣繖涓?common_timestamp 鍙橀噺鐨勫€间篃浼氬儚鏅€氱洿鏂瑰浘鍊间竴鏍疯
姹傚拰锛堝敖绠″浜庢椂闂存埑鑰岃█杩欐病浠€涔堟剰涔夛級銆?
```

  # echo 'hist:timer_pid=common_pid:key=timer_pid ...' >> event/trigger

```
濡傛灉涓€涓彉閲忎笉鏄敭鍙橀噺锛屼篃娌℃湁浠?'vals=' 涓哄墠缂€锛岄偅涔堢浉鍏宠仈鐨勪簨浠跺瓧娈典細琚?
淇濆瓨鍒板彉閲忎腑锛屼絾涓嶄細琚眰鍜?
```

  # echo 'hist:keys=next_pid:ts1=common_timestamp ...' >> event/trigger

```
鍙互鍚屾椂璧嬪€煎涓彉閲忋€備笅闈㈣繖琛屼細鎶?ts0 鍜?b 閮藉垱寤轰负鍙橀噺锛屼簩鑰?
```

  # echo 'hist:keys=pid:vals=$ts0,$b:ts0=common_timestamp,b=field1 ...' >> \
	event/trigger

```
娉ㄦ剰锛屽彉閲忚祴鍊兼棦鍙互鍑虹幇鍦ㄤ娇鐢ㄤ箣鍓嶏紝涔熷彲浠ュ嚭鐜板湪浣跨敤涔嬪悗銆備笅闈㈣繖鏉″懡浠や笌
```

  # echo 'hist:keys=pid:ts0=common_timestamp,b=field1:vals=$ts0,$b ...' >> \
	event/trigger

```
浠绘剰鏁伴噺鐨勩€佹湭缁戝畾鍒?'vals=' 鍓嶇紑鐨勫彉閲忥紝涔熷彲浠ラ€氳繃鐢ㄥ啋鍙峰垎闅旀潵璧嬪€笺€?
涓嬮潰鍚屾牱鏄?
```

  # echo 'hist:keys=pid:ts0=common_timestamp:b=field1 ...' >> event/trigger

```
鎸夌収涓婅堪鏂瑰紡璁剧疆鐨勫彉閲忥紝鍙互鍦ㄥ彟涓€涓簨浠剁殑琛ㄨ揪寮忎腑寮曠敤鍜屼娇鐢ㄣ€?
```

  # echo 'hist:keys=pid,prio:ts0=common_timestamp ...' >> event1/trigger
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp-$ts0 ...' >> event2/trigger

```
鍦ㄤ笂闈㈢涓€琛屼腑锛屼簨浠剁殑鏃堕棿鎴宠淇濆瓨鍒板彉閲?ts0 涓€傚湪涓嬩竴琛屼腑锛屼粠绗簩涓簨浠?
鐨勬椂闂存埑鍑忓幓 ts0锛屽緱鍒板欢杩熷€硷紝骞跺皢鍏惰祴缁欏彟涓€涓彉閲?'wakeup_lat'銆備笅闈㈢殑
鐩存柟鍥捐Е鍙戝櫒杩涜€屽埄鐢?wakeup_lat 鍙橀噺鏉ヨ绠楃粍鍚堝欢杩?
```

  # echo 'hist:key=pid:wakeupswitch_lat=$wakeup_lat+$switchtime_lat ...' >> event3/trigger

```
琛ㄨ揪寮忔敮鎸佷娇鐢ㄥ姞娉曘€佸噺娉曘€佷箻娉曞拰闄ゆ硶杩愮畻绗︼紙+-*/锛夈€?

娉ㄦ剰锛屽鏋滃湪瑙ｆ瀽鏃舵棤娉曟娴嬪埌闄ら浂锛堝嵆闄ゆ暟涓嶆槸甯搁噺锛夛紝鍒欑粨鏋滀负 -1銆?
```

  # echo 'hist:keys=next_pid:timestamp_secs=common_timestamp/1000000 ...' >> event/trigger

```

```

  # echo 'hist:keys=next_pid:us_per_sec=1000000 ...' >> event/trigger
  # echo 'hist:keys=next_pid:timestamp_secs=common_timestamp/$us_per_sec ...' >> event/trigger

```
鍙橀噺鐢氳嚦鍙互淇濆瓨鏍堝洖婧紝杩欏湪鍚堟垚浜嬩欢涓緢鏈夌敤銆?

### 2.6. 鍚堟垚浜嬩欢


鍚堟垚浜嬩欢鏄敱 hist 瑙﹀彂鍣ㄥ彉閲忔垨涓庝竴涓紙鎴栧涓級鍏朵粬浜嬩欢鍏宠仈鐨勫瓧娈垫墍鐢熸垚鐨勩€?
鐢ㄦ埛鑷畾涔夌殑浜嬩欢銆傚畠浠殑鐩殑鏄彁渚涗竴绉嶆満鍒讹紝浠ヤ笌鐜版湁涓斿凡涓轰汉鎵€鐔熸倝鐨勬櫘閫氫簨浠?
鐢ㄦ硶涓€鑷寸殑鏂瑰紡锛屽睍绀鸿法瓒婂涓簨浠剁殑鏁版嵁銆?

瑕佸畾涔変竴涓悎鎴愪簨浠讹紝鐢ㄦ埛闇€缂栧啓涓€涓畝鍗曠殑瑙勬牸璇存槑锛屽叾涓寘鍚柊浜嬩欢鐨勫悕绉颁互鍙?
涓€涓垨澶氫釜鍙橀噺鍙婂叾绫诲瀷锛堝彲浠ユ槸浠讳綍鍚堟硶鐨勫瓧娈电被鍨嬶級锛岀敤鍒嗗彿鍒嗛殧锛屽啓鍏?
tracing/synthetic_events 鏂囦欢銆?

鍙敤绫诲瀷璇峰弬闃?synth_field_size()銆?

濡傛灉瀛楁鍚嶅寘鍚?[n]锛屽垯璇ュ瓧娈佃瑙嗕负闈欐€佹暟缁勩€?

濡傛灉瀛楁鍚嶅寘鍚?[]锛堟棤涓嬫爣锛夛紝鍒欒瀛楁琚涓哄姩鎬佹暟缁勶紝瀹冨彧浼氬崰鐢ㄤ簨浠朵腑涓?
瀹圭撼璇ユ暟缁勬墍闇€鍚屾牱澶у皬鐨勭┖闂淬€?

瀛楃涓插瓧娈靛彲浠ヤ娇鐢ㄩ潤鎬佽娉曟寚瀹氾細

  char name[^32^];

鎴栬€呬娇鐢ㄥ姩鎬佽娉曪細

  char name[];

涓よ€呯殑灏哄涓婇檺鍧囦负 256銆?

渚嬪锛屼笅闈㈠垱寤轰竴涓悕涓?'wakeup_latency' 鐨勬柊浜嬩欢锛屽寘鍚?3 涓瓧娈碉細lat銆乸id
鍜?prio銆傝繖浜涘瓧娈典腑鐨勬瘡涓€涓兘鍙槸
```

  # echo 'wakeup_latency \
          u64 lat; \
          pid_t pid; \
	  int prio' >> \
	  /sys/kernel/tracing/synthetic_events

```
璇诲彇 tracing/synthetic_events 鏂囦欢浼氬垪鍑哄綋鍓嶆墍鏈夊凡瀹氫箟鐨?
```

  # cat /sys/kernel/tracing/synthetic_events
    wakeup_latency u64 lat; pid_t pid; int prio

```
宸叉湁鐨勫悎鎴愪簨浠跺畾涔夊彲浠ラ€氳繃鍦ㄥ墠闈㈠姞涓?
```

  # echo '!wakeup_latency u64 lat pid_t pid int prio' >> \
    /sys/kernel/tracing/synthetic_events

```
姝ゆ椂锛屼簨浠跺瓙绯荤粺涓繕娌℃湁鐪熸瀹炰緥鍖栧嚭涓€涓?'wakeup_latency' 浜嬩欢鈥斺€旇鍋氬埌
杩欎竴鐐癸紝闇€瑕佸疄渚嬪寲涓€涓€渉ist 瑙﹀彂鍣ㄥ姩浣溾€濆苟灏嗗叾缁戝畾鍒板叾浠栦簨浠朵笂瀹氫箟鐨勭湡瀹?
瀛楁涓庡彉閲忥紙鍏充簬濡備綍浣跨敤 hist 瑙﹀彂鍣ㄧ殑 'onmatch' 鍔ㄤ綔鏉ュ畬鎴愶紝璇峰弬瑙佷笅闈㈢殑
2.7 鑺傦級銆備竴鏃﹀畬鎴愶紝灏变細鍒涘缓鍑?'wakeup_latency' 鍚堟垚浜嬩欢瀹炰緥銆?

鏂颁簨浠跺垱寤哄湪 tracing/events/synthetic/ 鐩綍涓?
```

  # ls /sys/kernel/tracing/events/synthetic/wakeup_latency
        enable  filter  format  hist  id  trigger

```

```

  # echo 'hist:keys=pid,prio,lat.log2:sort=lat' >> \
        /sys/kernel/tracing/events/synthetic/wakeup_latency/trigger

```
涓婇潰浠?2 鐨勫箓娆″垎缁勫睍绀轰簡寤惰繜 "lat"銆?

涓庝换浣曞叾浠栦簨浠朵竴鏍凤紝涓€鏃︿负璇ヤ簨浠跺惎鐢ㄤ簡鐩存柟鍥撅紝
```

  # cat /sys/kernel/tracing/events/synthetic/wakeup_latency/hist

  # event histogram
  #
  # trigger info: hist:keys=pid,prio,lat.log2:vals=hitcount:sort=lat.log2:size=2048 [active]
  #

  { pid:       2035, prio:          9, lat: ~ 2^2  } hitcount:         43
  { pid:       2034, prio:          9, lat: ~ 2^2  } hitcount:         60
  { pid:       2029, prio:          9, lat: ~ 2^2  } hitcount:        965
  { pid:       2034, prio:        120, lat: ~ 2^2  } hitcount:          9
  { pid:       2033, prio:        120, lat: ~ 2^2  } hitcount:          5
  { pid:       2030, prio:          9, lat: ~ 2^2  } hitcount:        335
  { pid:       2030, prio:        120, lat: ~ 2^2  } hitcount:         10
  { pid:       2032, prio:        120, lat: ~ 2^2  } hitcount:          1
  { pid:       2035, prio:        120, lat: ~ 2^2  } hitcount:          2
  { pid:       2031, prio:          9, lat: ~ 2^2  } hitcount:        176
  { pid:       2028, prio:        120, lat: ~ 2^2  } hitcount:         15
  { pid:       2033, prio:          9, lat: ~ 2^2  } hitcount:         91
  { pid:       2032, prio:          9, lat: ~ 2^2  } hitcount:        125
  { pid:       2029, prio:        120, lat: ~ 2^2  } hitcount:          4
  { pid:       2031, prio:        120, lat: ~ 2^2  } hitcount:          3
  { pid:       2029, prio:        120, lat: ~ 2^3  } hitcount:          2
  { pid:       2035, prio:          9, lat: ~ 2^3  } hitcount:         41
  { pid:       2030, prio:        120, lat: ~ 2^3  } hitcount:          1
  { pid:       2032, prio:          9, lat: ~ 2^3  } hitcount:         32
  { pid:       2031, prio:          9, lat: ~ 2^3  } hitcount:         44
  { pid:       2034, prio:          9, lat: ~ 2^3  } hitcount:         40
  { pid:       2030, prio:          9, lat: ~ 2^3  } hitcount:         29
  { pid:       2033, prio:          9, lat: ~ 2^3  } hitcount:         31
  { pid:       2029, prio:          9, lat: ~ 2^3  } hitcount:         31
  { pid:       2028, prio:        120, lat: ~ 2^3  } hitcount:         18
  { pid:       2031, prio:        120, lat: ~ 2^3  } hitcount:          2
  { pid:       2028, prio:        120, lat: ~ 2^4  } hitcount:          1
  { pid:       2029, prio:          9, lat: ~ 2^4  } hitcount:          4
  { pid:       2031, prio:        120, lat: ~ 2^7  } hitcount:          1
  { pid:       2032, prio:        120, lat: ~ 2^7  } hitcount:          1

  Totals:
      Hits: 2122
      Entries: 30
      Dropped: 0


```
寤惰繜鍊间篃鍙互鎸夌粰瀹氬ぇ灏忚繘琛岀嚎鎬у垎缁勶紝
```

  # echo 'hist:keys=pid,prio,lat.buckets=10:sort=lat' >> \
        /sys/kernel/tracing/events/synthetic/wakeup_latency/trigger

  # event histogram
  #
  # trigger info: hist:keys=pid,prio,lat.buckets=10:vals=hitcount:sort=lat.buckets=10:size=2048 [active]
  #

  { pid:       2067, prio:          9, lat: ~ 0-9 } hitcount:        220
  { pid:       2068, prio:          9, lat: ~ 0-9 } hitcount:        157
  { pid:       2070, prio:          9, lat: ~ 0-9 } hitcount:        100
  { pid:       2067, prio:        120, lat: ~ 0-9 } hitcount:          6
  { pid:       2065, prio:        120, lat: ~ 0-9 } hitcount:          2
  { pid:       2066, prio:        120, lat: ~ 0-9 } hitcount:          2
  { pid:       2069, prio:          9, lat: ~ 0-9 } hitcount:        122
  { pid:       2069, prio:        120, lat: ~ 0-9 } hitcount:          8
  { pid:       2070, prio:        120, lat: ~ 0-9 } hitcount:          1
  { pid:       2068, prio:        120, lat: ~ 0-9 } hitcount:          7
  { pid:       2066, prio:          9, lat: ~ 0-9 } hitcount:        365
  { pid:       2064, prio:        120, lat: ~ 0-9 } hitcount:         35
  { pid:       2065, prio:          9, lat: ~ 0-9 } hitcount:        998
  { pid:       2071, prio:          9, lat: ~ 0-9 } hitcount:         85
  { pid:       2065, prio:          9, lat: ~ 10-19 } hitcount:          2
  { pid:       2064, prio:        120, lat: ~ 10-19 } hitcount:          2

  Totals:
      Hits: 2112
      Entries: 16
      Dropped: 0

```
瑕佷繚瀛樻爤鍥炴函锛屽彲鍒涘缓涓€涓甫鏈?"unsigned long[]" 绫诲瀷瀛楁锛堢敋鑷冲彧闇€ "long[]"锛?
鐨勫悎鎴愪簨浠躲€備緥濡傦紝瑕佹煡鐪嬫煇涓换鍔″湪琚?
```

  # cd /sys/kernel/tracing
  # echo 's:block_lat pid_t pid; u64 delta; unsigned long[] stack;' > dynamic_events
  # echo 'hist:keys=next_pid:ts=common_timestamp.usecs,st=common_stacktrace  if prev_state == 2' >> events/sched/sched_switch/trigger
  # echo 'hist:keys=prev_pid:delta=common_timestamp.usecs-$ts,s=$st:onmax($delta).trace(block_lat,prev_pid,$delta,$s)' >> events/sched/sched_switch/trigger
  # echo 1 > events/synthetic/block_lat/enable
  # cat trace

  # tracer: nop
  #
  # entries-in-buffer/entries-written: 2/2   #P:8
  #
  #                                _-----=> irqs-off/BH-disabled
  #                               / _----=> need-resched
  #                              | / _---=> hardirq/softirq
  #                              || / _--=> preempt-depth
  #                              ||| / _-=> migrate-disable
  #                              |||| /     delay
  #           TASK-PID     CPU#  |||||  TIMESTAMP  FUNCTION
  #              | |         |   |||||     |         |
            <idle>-0       [005] d..4.   521.164922: block_lat: pid=0 delta=8322 stack=STACK:
  => __schedule+0x448/0x7b0
  => schedule+0x5a/0xb0
  => io_schedule+0x42/0x70
  => bit_wait_io+0xd/0x60
  => __wait_on_bit+0x4b/0x140
  => out_of_line_wait_on_bit+0x91/0xb0
  => jbd2_journal_commit_transaction+0x1679/0x1a70
  => kjournald2+0xa9/0x280
  => kthread+0xe9/0x110
  => ret_from_fork+0x2c/0x50

             <...>-2       [004] d..4.   525.184257: block_lat: pid=2 delta=76 stack=STACK:
  => __schedule+0x448/0x7b0
  => schedule+0x5a/0xb0
  => schedule_timeout+0x11a/0x150
  => wait_for_completion_killable+0x144/0x1f0
  => __kthread_create_on_node+0xe7/0x1e0
  => kthread_create_on_node+0x51/0x70
  => create_worker+0xcc/0x1a0
  => worker_thread+0x2ad/0x380
  => kthread+0xe9/0x110
  => ret_from_fork+0x2c/0x50

```
甯︽湁鏍堝洖婧瓧娈电殑鍚堟垚浜嬩欢锛屽彲灏嗗叾鐢ㄤ綔閿紝
```

  # echo 'hist:keys=delta.buckets=100,stack.stacktrace:sort=delta' > events/synthetic/block_lat/trigger
  # cat events/synthetic/block_lat/hist

  # event histogram
  #
  # trigger info: hist:keys=delta.buckets=100,stack.stacktrace:vals=hitcount:sort=delta.buckets=100:size=2048 [active]
  #
  { delta: ~ 0-99, stack.stacktrace         __schedule+0xa19/0x1520
         schedule+0x6b/0x110
         io_schedule+0x46/0x80
         bit_wait_io+0x11/0x80
         __wait_on_bit+0x4e/0x120
         out_of_line_wait_on_bit+0x8d/0xb0
         __wait_on_buffer+0x33/0x40
         jbd2_journal_commit_transaction+0x155a/0x19b0
         kjournald2+0xab/0x270
         kthread+0xfa/0x130
         ret_from_fork+0x29/0x50
  } hitcount:          1
  { delta: ~ 0-99, stack.stacktrace         __schedule+0xa19/0x1520
         schedule+0x6b/0x110
         io_schedule+0x46/0x80
         rq_qos_wait+0xd0/0x170
         wbt_wait+0x9e/0xf0
         __rq_qos_throttle+0x25/0x40
         blk_mq_submit_bio+0x2c3/0x5b0
         __submit_bio+0xff/0x190
         submit_bio_noacct_nocheck+0x25b/0x2b0
         submit_bio_noacct+0x20b/0x600
         submit_bio+0x28/0x90
         ext4_bio_write_page+0x1e0/0x8c0
         mpage_submit_page+0x60/0x80
         mpage_process_page_bufs+0x16c/0x180
         mpage_prepare_extent_to_map+0x23f/0x530
  } hitcount:          1
  { delta: ~ 0-99, stack.stacktrace         __schedule+0xa19/0x1520
         schedule+0x6b/0x110
         schedule_hrtimeout_range_clock+0x97/0x110
         schedule_hrtimeout_range+0x13/0x20
         usleep_range_state+0x65/0x90
         __intel_wait_for_register+0x1c1/0x230 [i915]
         intel_psr_wait_for_idle_locked+0x171/0x2a0 [i915]
         intel_pipe_update_start+0x169/0x360 [i915]
         intel_update_crtc+0x112/0x490 [i915]
         skl_commit_modeset_enables+0x199/0x600 [i915]
         intel_atomic_commit_tail+0x7c4/0x1080 [i915]
         intel_atomic_commit_work+0x12/0x20 [i915]
         process_one_work+0x21c/0x3f0
         worker_thread+0x50/0x3e0
         kthread+0xfa/0x130
  } hitcount:          3
  { delta: ~ 0-99, stack.stacktrace         __schedule+0xa19/0x1520
         schedule+0x6b/0x110
         schedule_timeout+0x11e/0x160
         __wait_for_common+0x8f/0x190
         wait_for_completion+0x24/0x30
         __flush_work.isra.0+0x1cc/0x360
         flush_work+0xe/0x20
         drm_mode_rmfb+0x18b/0x1d0 [drm]
         drm_mode_rmfb_ioctl+0x10/0x20 [drm]
         drm_ioctl_kernel+0xb8/0x150 [drm]
         drm_ioctl+0x243/0x560 [drm]
         __x64_sys_ioctl+0x92/0xd0
         do_syscall_64+0x59/0x90
         entry_SYSCALL_64_after_hwframe+0x72/0xdc
  } hitcount:          1
  { delta: ~ 0-99, stack.stacktrace         __schedule+0xa19/0x1520
         schedule+0x6b/0x110
         schedule_timeout+0x87/0x160
         __wait_for_common+0x8f/0x190
         wait_for_completion_timeout+0x1d/0x30
         drm_atomic_helper_wait_for_flip_done+0x57/0x90 [drm_kms_helper]
         intel_atomic_commit_tail+0x8ce/0x1080 [i915]
         intel_atomic_commit_work+0x12/0x20 [i915]
         process_one_work+0x21c/0x3f0
         worker_thread+0x50/0x3e0
         kthread+0xfa/0x130
         ret_from_fork+0x29/0x50
  } hitcount:          1
  { delta: ~ 100-199, stack.stacktrace         __schedule+0xa19/0x1520
         schedule+0x6b/0x110
         schedule_hrtimeout_range_clock+0x97/0x110
         schedule_hrtimeout_range+0x13/0x20
         usleep_range_state+0x65/0x90
         pci_set_low_power_state+0x17f/0x1f0
         pci_set_power_state+0x49/0x250
         pci_finish_runtime_suspend+0x4a/0x90
         pci_pm_runtime_suspend+0xcb/0x1b0
         __rpm_callback+0x48/0x120
         rpm_callback+0x67/0x70
         rpm_suspend+0x167/0x780
         rpm_idle+0x25a/0x380
         pm_runtime_work+0x93/0xc0
         process_one_work+0x21c/0x3f0
  } hitcount:          1

  Totals:
    Hits: 10
    Entries: 7
    Dropped: 0

```
### 2.7. 鐩存柟鍥捐Е鍙戝櫒鐨勨€滃鐞嗗櫒鈥濅笌鈥滃姩浣溾€?


鐩存柟鍥捐Е鍙戝櫒鐨勨€滃姩浣溾€濇槸涓€涓嚱鏁帮紝姣忓綋鏈夌洿鏂瑰浘鏉＄洰琚坊鍔犳垨鏇存柊鏃讹紙澶у鏁版儏鍐典笅
鏄湁鏉′欢鍦帮級鎵ц銆?

褰撶洿鏂瑰浘鏉＄洰琚坊鍔犳垨鏇存柊鏃讹紝鐢辩洿鏂瑰浘瑙﹀彂鍣ㄧ殑鈥滃鐞嗗櫒鈥濇潵鍐冲畾鐩稿簲鐨勫姩浣滄槸鍚︾湡姝?
琚皟鐢ㄣ€?

鐩存柟鍥捐Е鍙戝櫒鐨勫鐞嗗櫒涓庡姩浣滀互濡備笅涓€鑸舰寮忔垚瀵圭粍鍚堬細

  <handler>.<action>

瑕佷负缁欏畾浜嬩欢鎸囧畾涓€涓?handler.action 瀵癸紝鍙渶鍦?hist 瑙﹀彂鍣ㄨ鏍艰鏄庝腑銆佺敤鍐掑彿
灏嗚 handler.action 瀵规嫭璧锋潵鍗冲彲銆?

鐞嗚涓婏紝浠讳綍澶勭悊鍣ㄩ兘鍙互涓庝换浣曞姩浣滅粍鍚堬紝浣嗗湪瀹炶返涓紝骞堕潪姣忎竴绉?handler.action
缁勫悎褰撳墠閮藉彈鏀寔锛涘鏋滄煇涓?handler.action 缁勫悎涓嶅彈鏀寔锛宧ist 瑙﹀彂鍣ㄥ皢浠?-EINVAL
澶辫触銆?

濡傛灉鏈樉寮忔寚瀹氾紝榛樿鐨勨€渉andler.action鈥濅笌浠ュ線涓€鏍凤紝浠呬粎鏄洿鏂颁笌鏌愭潯鐩叧鑱旂殑涓€缁?
鍊笺€備笉杩囷紝鏌愪簺搴旂敤鍙兘甯屾湜姝ゆ椂鎵ц棰濆鐨勫姩浣滐紝渚嬪鐢熸垚鍙︿竴涓簨浠讹紝鎴栬€呮瘮杈冨苟
淇濆瓨鏈€澶у€笺€?

鍙楁敮鎸佺殑澶勭悊鍣ㄤ笌鍔ㄤ綔鍒椾簬涓嬫柟锛屽苟鍦ㄥ悗缁钀戒腑缁撳悎涓€浜涘父瑙佷笖鏈夌敤鐨?handler.action
缁勫悎鐨勬弿杩拌繘琛屾洿璇︾粏鐨勮鏄庛€?

鍙敤鐨勫鐞嗗櫒鏈夛細

  - onmatch(matching.event)    - 鍦ㄤ换鎰忔坊鍔犳垨鏇存柊鏃惰皟鐢ㄥ姩浣?
  - onmax(var)                 - 褰?var 瓒呰繃褰撳墠鏈€澶у€兼椂璋冪敤鍔ㄤ綔
  - onchange(var)              - 褰?var 鍙戠敓鍙樺寲鏃惰皟鐢ㄥ姩浣?

鍙敤鐨勫姩浣滄湁锛?

  - trace(<synthetic_event_name>,param list)   - 鐢熸垚鍚堟垚浜嬩欢
  - save(field,...)                            - 淇濆瓨褰撳墠浜嬩欢瀛楁
  - snapshot()                                 - 瀵硅窡韪紦鍐插尯鍋氬揩鐓?

浠ヤ笅鏄父鐢ㄧ殑 handler.action 缁勫悎锛?

  - onmatch(matching.event).trace(<synthetic_event_name>,param list)

    'onmatch(matching.event).trace(<synthetic_event_name>,param
    list)' 鐩存柟鍥捐Е鍙戝櫒鍔ㄤ綔鍦ㄤ簨浠跺尮閰嶄笖鐩存柟鍥炬潯鐩皢琚坊鍔犳垨鏇存柊鏃惰璋冪敤銆傚畠浼?
    浠?'param list' 涓粰鍑虹殑鍊肩敓鎴愭寚瀹氱殑鍚堟垚浜嬩欢銆傚叾缁撴灉鏄敓鎴愪竴涓悎鎴愪簨浠讹紝
    璇ヤ簨浠剁敱璋冪敤浜嬩欢鍙戠敓鏃跺埢閭ｄ簺鍙橀噺涓墍鍖呭惈鐨勫€兼瀯鎴愩€備緥濡傦紝濡傛灉鍚堟垚浜嬩欢鍚嶆槸
    'wakeup_latency'锛屽垯浣跨敤 onmatch(event).trace(wakeup_latency,arg1,arg2)
    鏉ョ敓鎴?wakeup_latency 浜嬩欢銆?

    姝ゅ杩樻湁鍙︿竴绉嶇瓑浠峰舰寮忓彲鐢ㄤ簬鐢熸垚鍚堟垚浜嬩欢銆傚湪杩欑褰㈠紡涓紝鍚堟垚浜嬩欢鍚嶈褰撲綔
    鍑芥暟鍚嶆潵浣跨敤銆備緥濡傦紝鍐嶆浣跨敤 'wakeup_latency' 鍚堟垚浜嬩欢鍚嶏紝鍒?wakeup_latency
    浜嬩欢浼氶€氳繃鍍忓嚱鏁拌皟鐢ㄤ竴鏍风殑鏂瑰紡鏉ョ敓鎴愶紝浜嬩欢瀛楁鍊间綔涓哄疄鍙備紶鍏ワ細
    onmatch(event).wakeup_latency(arg1,arg2)銆傝繖绉嶅舰寮忕殑璇硶涓猴細

      onmatch(matching.event).<synthetic_event_name>(param list)

    鏃犺鍝鎯呭喌锛?param list' 閮界敱涓€涓垨澶氫釜鍙傛暟缁勬垚锛岃繖浜涘弬鏁版棦鍙互鏄畾涔変簬
    'matching.event' 鎴栫洰鏍囦簨浠朵笂鐨勫彉閲忥紝涔熷彲浠ユ槸瀛楁銆俻aram list 涓寚瀹氱殑鍙橀噺
    鎴栧瓧娈垫棦鍙互鏄畬鍏ㄩ檺瀹氬悕锛屼篃鍙互鏄潪闄愬畾鍚嶃€傚鏋滃彉閲忎互闈為檺瀹氬悕鎸囧畾锛屽垯瀹冨湪
    涓や釜浜嬩欢涔嬮棿蹇呴』鍞竴銆備綔涓?param 浣跨敤鐨勫瓧娈靛悕锛屽鏋滃紩鐢ㄧ殑鏄洰鏍囦簨浠讹紝鍙互
    鏄潪闄愬畾鍚嶏紱浣嗗鏋滃紩鐢ㄧ殑鏄尮閰嶄簨浠讹紝鍒欏繀椤绘槸瀹屽叏闄愬畾鍚嶃€傚畬鍏ㄩ檺瀹氬悕鐨勬牸寮忎负
    'system.event_name.$var_name' 鎴?'system.event_name.field'銆?

    'matching.event' 瑙勬牸璇存槑鍏跺疄灏辨槸涓?onmatch() 鍔熻兘涓洰鏍囦簨浠剁浉鍖归厤鐨勯偅涓簨浠?
    鐨勫畬鍏ㄩ檺瀹氫簨浠跺悕锛屾牸寮忎负 'system.event_name'銆備細姣旇緝涓や釜浜嬩欢鐨勭洿鏂瑰浘閿互纭畾
    浜嬩欢鏄惁鍖归厤銆傚鏋滀娇鐢ㄤ簡澶氫釜鐩存柟鍥鹃敭锛屽垯瀹冧滑蹇呴』鎸夋寚瀹氶『搴忓叏閮ㄥ尮閰嶃€?

    鏈€鍚庯紝'param list' 涓彉閲?瀛楁鐨勬暟閲忎笌绫诲瀷锛屽繀椤讳笌鎵€鐢熸垚鐨勫悎鎴愪簨浠朵腑瀛楁鐨?
    鏁伴噺涓庣被鍨嬬浉鍖归厤銆?

    浣滀负绀轰緥锛屼笅闈㈠畾涔変簡涓€涓畝鍗曠殑鍚堟垚浜嬩欢锛屽苟鍦ㄨ皟鐢ㄨ鍚堟垚浜嬩欢鏃讹紝浣跨敤瀹氫箟浜?
    sched_wakeup_new 浜嬩欢涓婄殑涓€涓彉閲忎綔涓哄弬鏁般€傝繖閲屾垜浠厛瀹氫箟璇ュ悎鎴?
```

      # echo 'wakeup_new_test pid_t pid' >> \
             /sys/kernel/tracing/synthetic_events

      # cat /sys/kernel/tracing/synthetic_events
            wakeup_new_test pid_t pid

    The following hist trigger both defines the missing testpid
    variable and specifies an onmatch() action that generates a
    wakeup_new_test synthetic event whenever a sched_wakeup_new event
    occurs, which because of the 'if comm == "cyclictest"' filter only
    happens when the executable is cyclictest::

      # echo 'hist:keys=$testpid:testpid=pid:onmatch(sched.sched_wakeup_new).\
              wakeup_new_test($testpid) if comm=="cyclictest"' >> \
              /sys/kernel/tracing/events/sched/sched_wakeup_new/trigger

    Or, equivalently, using the 'trace' keyword syntax::

      # echo 'hist:keys=$testpid:testpid=pid:onmatch(sched.sched_wakeup_new).\
              trace(wakeup_new_test,$testpid) if comm=="cyclictest"' >> \
              /sys/kernel/tracing/events/sched/sched_wakeup_new/trigger

    Creating and displaying a histogram based on those events is now
    just a matter of using the fields and new synthetic event in the
    tracing/events/synthetic directory, as usual::

      # echo 'hist:keys=pid:sort=pid' >> \
             /sys/kernel/tracing/events/synthetic/wakeup_new_test/trigger

    Running 'cyclictest' should cause wakeup_new events to generate
    wakeup_new_test synthetic events which should result in histogram
    output in the wakeup_new_test event's hist file::

      # cat /sys/kernel/tracing/events/synthetic/wakeup_new_test/hist

    A more typical usage would be to use two events to calculate a
    latency.  The following example uses a set of hist triggers to
    produce a 'wakeup_latency' histogram.

    First, we define a 'wakeup_latency' synthetic event::

      # echo 'wakeup_latency u64 lat; pid_t pid; int prio' >> \
              /sys/kernel/tracing/synthetic_events

    Next, we specify that whenever we see a sched_waking event for a
    cyclictest thread, save the timestamp in a 'ts0' variable::

      # echo 'hist:keys=$saved_pid:saved_pid=pid:ts0=common_timestamp.usecs \
              if comm=="cyclictest"' >> \
	      /sys/kernel/tracing/events/sched/sched_waking/trigger

    Then, when the corresponding thread is actually scheduled onto the
    CPU by a sched_switch event (saved_pid matches next_pid), calculate
    the latency and use that along with another variable and an event field
    to generate a wakeup_latency synthetic event::

      # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:\
              onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,\
	              $saved_pid,next_prio) if next_comm=="cyclictest"' >> \
	      /sys/kernel/tracing/events/sched/sched_switch/trigger

    We also need to create a histogram on the wakeup_latency synthetic
    event in order to aggregate the generated synthetic event data::

      # echo 'hist:keys=pid,prio,lat:sort=pid,lat' >> \
              /sys/kernel/tracing/events/synthetic/wakeup_latency/trigger

    Finally, once we've run cyclictest to actually generate some
    events, we can see the output by looking at the wakeup_latency
    synthetic event's hist file::

      # cat /sys/kernel/tracing/events/synthetic/wakeup_latency/hist

  - onmax(var).save(field,..	.)

    The 'onmax(var).save(field,...)' hist trigger action is invoked
    whenever the value of 'var' associated with a histogram entry
    exceeds the current maximum contained in that variable.

    The end result is that the trace event fields specified as the
    onmax.save() params will be saved if 'var' exceeds the current
    maximum for that hist trigger entry.  This allows context from the
    event that exhibited the new maximum to be saved for later
    reference.  When the histogram is displayed, additional fields
    displaying the saved values will be printed.

    As an example the below defines a couple of hist triggers, one for
    sched_waking and another for sched_switch, keyed on pid.  Whenever
    a sched_waking occurs, the timestamp is saved in the entry
    corresponding to the current pid, and when the scheduler switches
    back to that pid, the timestamp difference is calculated.  If the
    resulting latency, stored in wakeup_lat, exceeds the current
    maximum latency, the values specified in the save() fields are
    recorded::

      # echo 'hist:keys=pid:ts0=common_timestamp.usecs \
              if comm=="cyclictest"' >> \
              /sys/kernel/tracing/events/sched/sched_waking/trigger

      # echo 'hist:keys=next_pid:\
              wakeup_lat=common_timestamp.usecs-$ts0:\
              onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm) \
              if next_comm=="cyclictest"' >> \
              /sys/kernel/tracing/events/sched/sched_switch/trigger

    When the histogram is displayed, the max value and the saved
    values corresponding to the max are displayed following the rest
    of the fields::

      # cat /sys/kernel/tracing/events/sched/sched_switch/hist
        { next_pid:       2255 } hitcount:        239
          common_timestamp-ts0:          0
          max:         27
	  next_comm: cyclictest
          prev_pid:          0  prev_prio:        120  prev_comm: swapper/1

        { next_pid:       2256 } hitcount:       2355
          common_timestamp-ts0: 0
          max:         49  next_comm: cyclictest
          prev_pid:          0  prev_prio:        120  prev_comm: swapper/0

        Totals:
            Hits: 12970
            Entries: 2
            Dropped: 0

  - onmax(var).snapshot()

    The 'onmax(var).snapshot()' hist trigger action is invoked
    whenever the value of 'var' associated with a histogram entry
    exceeds the current maximum contained in that variable.

    The end result is that a global snapshot of the trace buffer will
    be saved in the tracing/snapshot file if 'var' exceeds the current
    maximum for any hist trigger entry.

    Note that in this case the maximum is a global maximum for the
    current trace instance, which is the maximum across all buckets of
    the histogram.  The key of the specific trace event that caused
    the global maximum and the global maximum itself are displayed,
    along with a message stating that a snapshot has been taken and
    where to find it.  The user can use the key information displayed
    to locate the corresponding bucket in the histogram for even more
    detail.

    As an example the below defines a couple of hist triggers, one for
    sched_waking and another for sched_switch, keyed on pid.  Whenever
    a sched_waking event occurs, the timestamp is saved in the entry
    corresponding to the current pid, and when the scheduler switches
    back to that pid, the timestamp difference is calculated.  If the
    resulting latency, stored in wakeup_lat, exceeds the current
    maximum latency, a snapshot is taken.  As part of the setup, all
    the scheduler events are also enabled, which are the events that
    will show up in the snapshot when it is taken at some point::

      # echo 1 > /sys/kernel/tracing/events/sched/enable

      # echo 'hist:keys=pid:ts0=common_timestamp.usecs \
              if comm=="cyclictest"' >> \
              /sys/kernel/tracing/events/sched/sched_waking/trigger

      # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
              onmax($wakeup_lat).save(next_prio,next_comm,prev_pid,prev_prio, \
	      prev_comm):onmax($wakeup_lat).snapshot() \
	      if next_comm=="cyclictest"' >> \
	      /sys/kernel/tracing/events/sched/sched_switch/trigger

    When the histogram is displayed, for each bucket the max value
    and the saved values corresponding to the max are displayed
    following the rest of the fields.

    If a snapshot was taken, there is also a message indicating that,
    along with the value and event that triggered the global maximum::

      # cat /sys/kernel/tracing/events/sched/sched_switch/hist
        { next_pid:       2101 } hitcount:        200
	  max:         52  next_prio:        120  next_comm: cyclictest \
          prev_pid:          0  prev_prio:        120  prev_comm: swapper/6

        { next_pid:       2103 } hitcount:       1326
	  max:        572  next_prio:         19  next_comm: cyclictest \
          prev_pid:          0  prev_prio:        120  prev_comm: swapper/1

        { next_pid:       2102 } hitcount:       1982 \
	  max:         74  next_prio:         19  next_comm: cyclictest \
          prev_pid:          0  prev_prio:        120  prev_comm: swapper/5

      Snapshot taken (see tracing/snapshot).  Details:
	  triggering value { onmax($wakeup_lat) }:        572	\
	  triggered by event with key: { next_pid:       2103 }

      Totals:
          Hits: 3508
          Entries: 3
          Dropped: 0

    In the above case, the event that triggered the global maximum has
    the key with next_pid == 2103.  If you look at the bucket that has
    2103 as the key, you'll find the additional values save()'d along
    with the local maximum for that bucket, which should be the same
    as the global maximum (since that was the same value that
    triggered the global snapshot).

    And finally, looking at the snapshot data should show at or near
    the end the event that triggered the snapshot (in this case you
    can verify the timestamps between the sched_waking and
    sched_switch events, which should match the time displayed in the
    global maximum)::

     # cat /sys/kernel/tracing/snapshot

         <...>-2103  [005] d..3   309.873125: sched_switch: prev_comm=cyclictest prev_pid=2103 prev_prio=19 prev_state=D ==> next_comm=swapper/5 next_pid=0 next_prio=120
         <idle>-0     [005] d.h3   309.873611: sched_waking: comm=cyclictest pid=2102 prio=19 target_cpu=005
         <idle>-0     [005] dNh4   309.873613: sched_wakeup: comm=cyclictest pid=2102 prio=19 target_cpu=005
         <idle>-0     [005] d..3   309.873616: sched_switch: prev_comm=swapper/5 prev_pid=0 prev_prio=120 prev_state=S ==> next_comm=cyclictest next_pid=2102 next_prio=19
         <...>-2102  [005] d..3   309.873625: sched_switch: prev_comm=cyclictest prev_pid=2102 prev_prio=19 prev_state=D ==> next_comm=swapper/5 next_pid=0 next_prio=120
         <idle>-0     [005] d.h3   309.874624: sched_waking: comm=cyclictest pid=2102 prio=19 target_cpu=005
         <idle>-0     [005] dNh4   309.874626: sched_wakeup: comm=cyclictest pid=2102 prio=19 target_cpu=005
         <idle>-0     [005] dNh3   309.874628: sched_waking: comm=cyclictest pid=2103 prio=19 target_cpu=005
         <idle>-0     [005] dNh4   309.874630: sched_wakeup: comm=cyclictest pid=2103 prio=19 target_cpu=005
         <idle>-0     [005] d..3   309.874633: sched_switch: prev_comm=swapper/5 prev_pid=0 prev_prio=120 prev_state=S ==> next_comm=cyclictest next_pid=2102 next_prio=19
         <idle>-0     [004] d.h3   309.874757: sched_waking: comm=gnome-terminal- pid=1699 prio=120 target_cpu=004
         <idle>-0     [004] dNh4   309.874762: sched_wakeup: comm=gnome-terminal- pid=1699 prio=120 target_cpu=004
         <idle>-0     [004] d..3   309.874766: sched_switch: prev_comm=swapper/4 prev_pid=0 prev_prio=120 prev_state=S ==> next_comm=gnome-terminal- next_pid=1699 next_prio=120
     gnome-terminal--1699  [004] d.h2   309.874941: sched_stat_runtime: comm=gnome-terminal- pid=1699 runtime=180706 [ns] vruntime=1126870572 [ns]
         <idle>-0     [003] d.s4   309.874956: sched_waking: comm=rcu_sched pid=9 prio=120 target_cpu=007
         <idle>-0     [003] d.s5   309.874960: sched_wake_idle_without_ipi: cpu=7
         <idle>-0     [003] d.s5   309.874961: sched_wakeup: comm=rcu_sched pid=9 prio=120 target_cpu=007
         <idle>-0     [007] d..3   309.874963: sched_switch: prev_comm=swapper/7 prev_pid=0 prev_prio=120 prev_state=S ==> next_comm=rcu_sched next_pid=9 next_prio=120
      rcu_sched-9     [007] d..3   309.874973: sched_stat_runtime: comm=rcu_sched pid=9 runtime=13646 [ns] vruntime=22531430286 [ns]
      rcu_sched-9     [007] d..3   309.874978: sched_switch: prev_comm=rcu_sched prev_pid=9 prev_prio=120 prev_state=R+ ==> next_comm=swapper/7 next_pid=0 next_prio=120
          <...>-2102  [005] d..4   309.874994: sched_migrate_task: comm=cyclictest pid=2103 prio=19 orig_cpu=5 dest_cpu=1
          <...>-2102  [005] d..4   309.875185: sched_wake_idle_without_ipi: cpu=1
         <idle>-0     [001] d..3   309.875200: sched_switch: prev_comm=swapper/1 prev_pid=0 prev_prio=120 prev_state=S ==> next_comm=cyclictest next_pid=2103 next_prio=19

  - onchange(var).save(field,..	.)

    The 'onchange(var).save(field,...)' hist trigger action is invoked
    whenever the value of 'var' associated with a histogram entry
    changes.

    The end result is that the trace event fields specified as the
    onchange.save() params will be saved if 'var' changes for that
    hist trigger entry.  This allows context from the event that
    changed the value to be saved for later reference.  When the
    histogram is displayed, additional fields displaying the saved
    values will be printed.

  - onchange(var).snapshot()

    The 'onchange(var).snapshot()' hist trigger action is invoked
    whenever the value of 'var' associated with a histogram entry
    changes.

    The end result is that a global snapshot of the trace buffer will
    be saved in the tracing/snapshot file if 'var' changes for any
    hist trigger entry.

    Note that in this case the changed value is a global variable
    associated with current trace instance.  The key of the specific
    trace event that caused the value to change and the global value
    itself are displayed, along with a message stating that a snapshot
    has been taken and where to find it.  The user can use the key
    information displayed to locate the corresponding bucket in the
    histogram for even more detail.

    As an example the below defines a hist trigger on the tcp_probe
    event, keyed on dport.  Whenever a tcp_probe event occurs, the
    cwnd field is checked against the current value stored in the
    $cwnd variable.  If the value has changed, a snapshot is taken.
    As part of the setup, all the scheduler and tcp events are also
    enabled, which are the events that will show up in the snapshot
    when it is taken at some point::

      # echo 1 > /sys/kernel/tracing/events/sched/enable
      # echo 1 > /sys/kernel/tracing/events/tcp/enable

      # echo 'hist:keys=dport:cwnd=snd_cwnd: \
              onchange($cwnd).save(snd_wnd,srtt,rcv_wnd): \
	      onchange($cwnd).snapshot()' >> \
	      /sys/kernel/tracing/events/tcp/tcp_probe/trigger

    When the histogram is displayed, for each bucket the tracked value
    and the saved values corresponding to that value are displayed
    following the rest of the fields.

    If a snapshot was taken, there is also a message indicating that,
    along with the value and event that triggered the snapshot::

      # cat /sys/kernel/tracing/events/tcp/tcp_probe/hist

      { dport:       1521 } hitcount:          8
	changed:         10  snd_wnd:      35456  srtt:     154262  rcv_wnd:      42112

      { dport:         80 } hitcount:         23
	changed:         10  snd_wnd:      28960  srtt:      19604  rcv_wnd:      29312

      { dport:       9001 } hitcount:        172
	changed:         10  snd_wnd:      48384  srtt:     260444  rcv_wnd:      55168

      { dport:        443 } hitcount:        211
	changed:         10  snd_wnd:      26960  srtt:      17379  rcv_wnd:      28800

      Snapshot taken (see tracing/snapshot).  Details:

          triggering value { onchange($cwnd) }:         10
          triggered by event with key: { dport:         80 }

      Totals:
          Hits: 414
          Entries: 4
          Dropped: 0

    In the above case, the event that triggered the snapshot has the
    key with dport == 80.  If you look at the bucket that has 80 as
    the key, you'll find the additional values save()'d along with the
    changed value for that bucket, which should be the same as the
    global changed value (since that was the same value that triggered
    the global snapshot).

    And finally, looking at the snapshot data should show at or near
    the end the event that triggered the snapshot::

      # cat /sys/kernel/tracing/snapshot

         gnome-shell-1261  [006] dN.3    49.823113: sched_stat_runtime: comm=gnome-shell pid=1261 runtime=49347 [ns] vruntime=1835730389 [ns]
       kworker/u16:4-773   [003] d..3    49.823114: sched_switch: prev_comm=kworker/u16:4 prev_pid=773 prev_prio=120 prev_state=R+ ==> next_comm=kworker/3:2 next_pid=135 next_prio=120
         gnome-shell-1261  [006] d..3    49.823114: sched_switch: prev_comm=gnome-shell prev_pid=1261 prev_prio=120 prev_state=R+ ==> next_comm=kworker/6:2 next_pid=387 next_prio=120
         kworker/3:2-135   [003] d..3    49.823118: sched_stat_runtime: comm=kworker/3:2 pid=135 runtime=5339 [ns] vruntime=17815800388 [ns]
         kworker/6:2-387   [006] d..3    49.823120: sched_stat_runtime: comm=kworker/6:2 pid=387 runtime=9594 [ns] vruntime=14589605367 [ns]
         kworker/6:2-387   [006] d..3    49.823122: sched_switch: prev_comm=kworker/6:2 prev_pid=387 prev_prio=120 prev_state=R+ ==> next_comm=gnome-shell next_pid=1261 next_prio=120
         kworker/3:2-135   [003] d..3    49.823123: sched_switch: prev_comm=kworker/3:2 prev_pid=135 prev_prio=120 prev_state=T ==> next_comm=swapper/3 next_pid=0 next_prio=120
              <idle>-0     [004] ..s7    49.823798: tcp_probe: src=10.0.0.10:54326 dest=23.215.104.193:80 mark=0x0 length=32 snd_nxt=0xe3ae2ff5 snd_una=0xe3ae2ecd snd_cwnd=10 ssthresh=2147483647 snd_wnd=28960 srtt=19604 rcv_wnd=29312

```
### 2.8. 鐢ㄦ埛绌洪棿鍒涘缓瑙﹀彂鍣?


鍐欏叆 /sys/kernel/tracing/trace_marker 浼氬啓鍏?ftrace 鐜舰缂撳啿鍖恒€傞€氳繃鍐欏叆浣嶄簬
/sys/kernel/tracing/events/ftrace/print/ 鐨勮Е鍙戝櫒鏂囦欢锛屽畠涔熷彲浠ュ儚浜嬩欢涓€鏍疯捣浣滅敤銆?

淇敼 cyclictest锛屼娇鍏跺湪浼戠湢鍓嶅啓鍏?trace_marker 鏂囦欢
```

  static void traceputs(char *str)
  {
	/* tracemark_fd is the trace_marker file descriptor */
	if (tracemark_fd < 0)
		return;
	/* write the tracemark message */
	write(tracemark_fd, str, strlen(str));
  }

```

```

	traceputs("start");
	clock_nanosleep(...);
	traceputs("end");

```

```

 # cd /sys/kernel/tracing
 # echo 'latency u64 lat' > synthetic_events
 # echo 'hist:keys=common_pid:ts0=common_timestamp.usecs if buf == "start"' > events/ftrace/print/trigger
 # echo 'hist:keys=common_pid:lat=common_timestamp.usecs-$ts0:onmatch(ftrace.print).latency($lat) if buf == "end"' >> events/ftrace/print/trigger
 # echo 'hist:keys=lat,common_pid:sort=lat' > events/synthetic/latency/trigger

```
涓婇潰鍒涘缓浜嗕竴涓悕涓?"latency" 鐨勫悎鎴愪簨浠讹紝浠ュ強涓や釜閽堝 trace_marker 鐨勭洿鏂瑰浘锛?
褰撳悜 trace_marker 鏂囦欢鍐欏叆 "start" 鏃惰Е鍙戝叾涓竴涓紝鍐欏叆 "end" 鏃惰Е鍙戝彟涓€涓€?
濡傛灉 pid 鍖归厤锛屽垯瀹冧細浠ヨ绠楀緱鍒扮殑寤惰繜浣滀负鍙傛暟鏉ヨ皟鐢?"latency" 鍚堟垚浜嬩欢銆傛渶鍚庯紝
鍚?latency 鍚堟垚浜嬩欢娣诲姞涓€涓洿鏂瑰浘锛屼互璁板綍璁＄畻寰楀埌鐨勫欢杩熶互鍙?pid銆?
```

 # ./cyclictest -p80 -d0 -i250 -n -a -t --tracemark -b 1000

 -p80  : run threads at priority 80
 -d0   : have all threads run at the same interval
 -i250 : start the interval at 250 microseconds (all threads will do this)
 -n    : sleep with nanosleep
 -a    : affine all threads to a separate CPU
 -t    : one thread per available CPU
 --tracemark : enable trace mark writing
 -b 1000 : stop if any latency is greater than 1000 microseconds

```
娉ㄦ剰锛?b 1000 鍙槸涓轰簡鍚敤 --tracemark 鑰屼娇鐢ㄣ€?
```

 # cat events/synthetic/latency/hist
 # event histogram
 #
 # trigger info: hist:keys=lat,common_pid:vals=hitcount:sort=lat:size=2048 [active]
 #

 { lat:        107, common_pid:       2039 } hitcount:          1
 { lat:        122, common_pid:       2041 } hitcount:          1
 { lat:        166, common_pid:       2039 } hitcount:          1
 { lat:        174, common_pid:       2039 } hitcount:          1
 { lat:        194, common_pid:       2041 } hitcount:          1
 { lat:        196, common_pid:       2036 } hitcount:          1
 { lat:        197, common_pid:       2038 } hitcount:          1
 { lat:        198, common_pid:       2039 } hitcount:          1
 { lat:        199, common_pid:       2039 } hitcount:          1
 { lat:        200, common_pid:       2041 } hitcount:          1
 { lat:        201, common_pid:       2039 } hitcount:          2
 { lat:        202, common_pid:       2038 } hitcount:          1
 { lat:        202, common_pid:       2043 } hitcount:          1
 { lat:        203, common_pid:       2039 } hitcount:          1
 { lat:        203, common_pid:       2036 } hitcount:          1
 { lat:        203, common_pid:       2041 } hitcount:          1
 { lat:        206, common_pid:       2038 } hitcount:          2
 { lat:        207, common_pid:       2039 } hitcount:          1
 { lat:        207, common_pid:       2036 } hitcount:          1
 { lat:        208, common_pid:       2040 } hitcount:          1
 { lat:        209, common_pid:       2043 } hitcount:          1
 { lat:        210, common_pid:       2039 } hitcount:          1
 { lat:        211, common_pid:       2039 } hitcount:          4
 { lat:        212, common_pid:       2043 } hitcount:          1
 { lat:        212, common_pid:       2039 } hitcount:          2
 { lat:        213, common_pid:       2039 } hitcount:          1
 { lat:        214, common_pid:       2038 } hitcount:          1
 { lat:        214, common_pid:       2039 } hitcount:          2
 { lat:        214, common_pid:       2042 } hitcount:          1
 { lat:        215, common_pid:       2039 } hitcount:          1
 { lat:        217, common_pid:       2036 } hitcount:          1
 { lat:        217, common_pid:       2040 } hitcount:          1
 { lat:        217, common_pid:       2039 } hitcount:          1
 { lat:        218, common_pid:       2039 } hitcount:          6
 { lat:        219, common_pid:       2039 } hitcount:          9
 { lat:        220, common_pid:       2039 } hitcount:         11
 { lat:        221, common_pid:       2039 } hitcount:          5
 { lat:        221, common_pid:       2042 } hitcount:          1
 { lat:        222, common_pid:       2039 } hitcount:          7
 { lat:        223, common_pid:       2036 } hitcount:          1
 { lat:        223, common_pid:       2039 } hitcount:          3
 { lat:        224, common_pid:       2039 } hitcount:          4
 { lat:        224, common_pid:       2037 } hitcount:          1
 { lat:        224, common_pid:       2036 } hitcount:          2
 { lat:        225, common_pid:       2039 } hitcount:          5
 { lat:        225, common_pid:       2042 } hitcount:          1
 { lat:        226, common_pid:       2039 } hitcount:          7
 { lat:        226, common_pid:       2036 } hitcount:          4
 { lat:        227, common_pid:       2039 } hitcount:          6
 { lat:        227, common_pid:       2036 } hitcount:         12
 { lat:        227, common_pid:       2043 } hitcount:          1
 { lat:        228, common_pid:       2039 } hitcount:          7
 { lat:        228, common_pid:       2036 } hitcount:         14
 { lat:        229, common_pid:       2039 } hitcount:          9
 { lat:        229, common_pid:       2036 } hitcount:          8
 { lat:        229, common_pid:       2038 } hitcount:          1
 { lat:        230, common_pid:       2039 } hitcount:         11
 { lat:        230, common_pid:       2036 } hitcount:          6
 { lat:        230, common_pid:       2043 } hitcount:          1
 { lat:        230, common_pid:       2042 } hitcount:          2
 { lat:        231, common_pid:       2041 } hitcount:          1
 { lat:        231, common_pid:       2036 } hitcount:          6
 { lat:        231, common_pid:       2043 } hitcount:          1
 { lat:        231, common_pid:       2039 } hitcount:          8
 { lat:        232, common_pid:       2037 } hitcount:          1
 { lat:        232, common_pid:       2039 } hitcount:          6
 { lat:        232, common_pid:       2040 } hitcount:          2
 { lat:        232, common_pid:       2036 } hitcount:          5
 { lat:        232, common_pid:       2043 } hitcount:          1
 { lat:        233, common_pid:       2036 } hitcount:          5
 { lat:        233, common_pid:       2039 } hitcount:         11
 { lat:        234, common_pid:       2039 } hitcount:          4
 { lat:        234, common_pid:       2038 } hitcount:          2
 { lat:        234, common_pid:       2043 } hitcount:          2
 { lat:        234, common_pid:       2036 } hitcount:         11
 { lat:        234, common_pid:       2040 } hitcount:          1
 { lat:        235, common_pid:       2037 } hitcount:          2
 { lat:        235, common_pid:       2036 } hitcount:          8
 { lat:        235, common_pid:       2043 } hitcount:          2
 { lat:        235, common_pid:       2039 } hitcount:          5
 { lat:        235, common_pid:       2042 } hitcount:          2
 { lat:        235, common_pid:       2040 } hitcount:          4
 { lat:        235, common_pid:       2041 } hitcount:          1
 { lat:        236, common_pid:       2036 } hitcount:          7
 { lat:        236, common_pid:       2037 } hitcount:          1
 { lat:        236, common_pid:       2041 } hitcount:          5
 { lat:        236, common_pid:       2039 } hitcount:          3
 { lat:        236, common_pid:       2043 } hitcount:          9
 { lat:        236, common_pid:       2040 } hitcount:          7
 { lat:        237, common_pid:       2037 } hitcount:          1
 { lat:        237, common_pid:       2040 } hitcount:          1
 { lat:        237, common_pid:       2036 } hitcount:          9
 { lat:        237, common_pid:       2039 } hitcount:          3
 { lat:        237, common_pid:       2043 } hitcount:          8
 { lat:        237, common_pid:       2042 } hitcount:          2
 { lat:        237, common_pid:       2041 } hitcount:          2
 { lat:        238, common_pid:       2043 } hitcount:         10
 { lat:        238, common_pid:       2040 } hitcount:          1
 { lat:        238, common_pid:       2037 } hitcount:          9
 { lat:        238, common_pid:       2038 } hitcount:          1
 { lat:        238, common_pid:       2039 } hitcount:          1
 { lat:        238, common_pid:       2042 } hitcount:          3
 { lat:        238, common_pid:       2036 } hitcount:          7
 { lat:        239, common_pid:       2041 } hitcount:          1
 { lat:        239, common_pid:       2043 } hitcount:         11
 { lat:        239, common_pid:       2037 } hitcount:         11
 { lat:        239, common_pid:       2038 } hitcount:          6
 { lat:        239, common_pid:       2036 } hitcount:          7
 { lat:        239, common_pid:       2040 } hitcount:          1
 { lat:        239, common_pid:       2042 } hitcount:          9
 { lat:        240, common_pid:       2037 } hitcount:         29
 { lat:        240, common_pid:       2043 } hitcount:         15
 { lat:        240, common_pid:       2040 } hitcount:         44
 { lat:        240, common_pid:       2039 } hitcount:          1
 { lat:        240, common_pid:       2041 } hitcount:          2
 { lat:        240, common_pid:       2038 } hitcount:          1
 { lat:        240, common_pid:       2036 } hitcount:         10
 { lat:        240, common_pid:       2042 } hitcount:         13
 { lat:        241, common_pid:       2036 } hitcount:         21
 { lat:        241, common_pid:       2041 } hitcount:         36
 { lat:        241, common_pid:       2037 } hitcount:         34
 { lat:        241, common_pid:       2042 } hitcount:         14
 { lat:        241, common_pid:       2040 } hitcount:         94
 { lat:        241, common_pid:       2039 } hitcount:         12
 { lat:        241, common_pid:       2038 } hitcount:          2
 { lat:        241, common_pid:       2043 } hitcount:         28
 { lat:        242, common_pid:       2040 } hitcount:        109
 { lat:        242, common_pid:       2041 } hitcount:        506
 { lat:        242, common_pid:       2039 } hitcount:        155
 { lat:        242, common_pid:       2042 } hitcount:         21
 { lat:        242, common_pid:       2037 } hitcount:         52
 { lat:        242, common_pid:       2043 } hitcount:         21
 { lat:        242, common_pid:       2036 } hitcount:         16
 { lat:        242, common_pid:       2038 } hitcount:        156
 { lat:        243, common_pid:       2037 } hitcount:         46
 { lat:        243, common_pid:       2039 } hitcount:         40
 { lat:        243, common_pid:       2042 } hitcount:        119
 { lat:        243, common_pid:       2041 } hitcount:        611
 { lat:        243, common_pid:       2036 } hitcount:         69
 { lat:        243, common_pid:       2038 } hitcount:        784
 { lat:        243, common_pid:       2040 } hitcount:        323
 { lat:        243, common_pid:       2043 } hitcount:         14
 { lat:        244, common_pid:       2043 } hitcount:         35
 { lat:        244, common_pid:       2042 } hitcount:        305
 { lat:        244, common_pid:       2039 } hitcount:          8
 { lat:        244, common_pid:       2040 } hitcount:       4515
 { lat:        244, common_pid:       2038 } hitcount:        371
 { lat:        244, common_pid:       2037 } hitcount:         31
 { lat:        244, common_pid:       2036 } hitcount:        114
 { lat:        244, common_pid:       2041 } hitcount:       3396
 { lat:        245, common_pid:       2036 } hitcount:        700
 { lat:        245, common_pid:       2041 } hitcount:       2772
 { lat:        245, common_pid:       2037 } hitcount:        268
 { lat:        245, common_pid:       2039 } hitcount:        472
 { lat:        245, common_pid:       2038 } hitcount:       2758
 { lat:        245, common_pid:       2042 } hitcount:       3833
 { lat:        245, common_pid:       2040 } hitcount:       3105
 { lat:        245, common_pid:       2043 } hitcount:        645
 { lat:        246, common_pid:       2038 } hitcount:       3451
 { lat:        246, common_pid:       2041 } hitcount:        142
 { lat:        246, common_pid:       2037 } hitcount:       5101
 { lat:        246, common_pid:       2040 } hitcount:         68
 { lat:        246, common_pid:       2043 } hitcount:       5099
 { lat:        246, common_pid:       2039 } hitcount:       5608
 { lat:        246, common_pid:       2042 } hitcount:       3723
 { lat:        246, common_pid:       2036 } hitcount:       4738
 { lat:        247, common_pid:       2042 } hitcount:        312
 { lat:        247, common_pid:       2043 } hitcount:       2385
 { lat:        247, common_pid:       2041 } hitcount:        452
 { lat:        247, common_pid:       2038 } hitcount:        792
 { lat:        247, common_pid:       2040 } hitcount:         78
 { lat:        247, common_pid:       2036 } hitcount:       2375
 { lat:        247, common_pid:       2039 } hitcount:       1834
 { lat:        247, common_pid:       2037 } hitcount:       2655
 { lat:        248, common_pid:       2037 } hitcount:         36
 { lat:        248, common_pid:       2042 } hitcount:         11
 { lat:        248, common_pid:       2038 } hitcount:        122
 { lat:        248, common_pid:       2036 } hitcount:        135
 { lat:        248, common_pid:       2039 } hitcount:         26
 { lat:        248, common_pid:       2041 } hitcount:        503
 { lat:        248, common_pid:       2043 } hitcount:         66
 { lat:        248, common_pid:       2040 } hitcount:         46
 { lat:        249, common_pid:       2037 } hitcount:         29
 { lat:        249, common_pid:       2038 } hitcount:          1
 { lat:        249, common_pid:       2043 } hitcount:         29
 { lat:        249, common_pid:       2039 } hitcount:          8
 { lat:        249, common_pid:       2042 } hitcount:         56
 { lat:        249, common_pid:       2040 } hitcount:         27
 { lat:        249, common_pid:       2041 } hitcount:         11
 { lat:        249, common_pid:       2036 } hitcount:         27
 { lat:        250, common_pid:       2038 } hitcount:          1
 { lat:        250, common_pid:       2036 } hitcount:         30
 { lat:        250, common_pid:       2040 } hitcount:         19
 { lat:        250, common_pid:       2043 } hitcount:         22
 { lat:        250, common_pid:       2042 } hitcount:         20
 { lat:        250, common_pid:       2041 } hitcount:          1
 { lat:        250, common_pid:       2039 } hitcount:          6
 { lat:        250, common_pid:       2037 } hitcount:         48
 { lat:        251, common_pid:       2037 } hitcount:         43
 { lat:        251, common_pid:       2039 } hitcount:          1
 { lat:        251, common_pid:       2036 } hitcount:         12
 { lat:        251, common_pid:       2042 } hitcount:          2
 { lat:        251, common_pid:       2041 } hitcount:          1
 { lat:        251, common_pid:       2043 } hitcount:         15
 { lat:        251, common_pid:       2040 } hitcount:          3
 { lat:        252, common_pid:       2040 } hitcount:          1
 { lat:        252, common_pid:       2036 } hitcount:         12
 { lat:        252, common_pid:       2037 } hitcount:         21
 { lat:        252, common_pid:       2043 } hitcount:         14
 { lat:        253, common_pid:       2037 } hitcount:         21
 { lat:        253, common_pid:       2039 } hitcount:          2
 { lat:        253, common_pid:       2036 } hitcount:          9
 { lat:        253, common_pid:       2043 } hitcount:          6
 { lat:        253, common_pid:       2040 } hitcount:          1
 { lat:        254, common_pid:       2036 } hitcount:          8
 { lat:        254, common_pid:       2043 } hitcount:          3
 { lat:        254, common_pid:       2041 } hitcount:          1
 { lat:        254, common_pid:       2042 } hitcount:          1
 { lat:        254, common_pid:       2039 } hitcount:          1
 { lat:        254, common_pid:       2037 } hitcount:         12
 { lat:        255, common_pid:       2043 } hitcount:          1
 { lat:        255, common_pid:       2037 } hitcount:          2
 { lat:        255, common_pid:       2036 } hitcount:          2
 { lat:        255, common_pid:       2039 } hitcount:          8
 { lat:        256, common_pid:       2043 } hitcount:          1
 { lat:        256, common_pid:       2036 } hitcount:          4
 { lat:        256, common_pid:       2039 } hitcount:          6
 { lat:        257, common_pid:       2039 } hitcount:          5
 { lat:        257, common_pid:       2036 } hitcount:          4
 { lat:        258, common_pid:       2039 } hitcount:          5
 { lat:        258, common_pid:       2036 } hitcount:          2
 { lat:        259, common_pid:       2036 } hitcount:          7
 { lat:        259, common_pid:       2039 } hitcount:          7
 { lat:        260, common_pid:       2036 } hitcount:          8
 { lat:        260, common_pid:       2039 } hitcount:          6
 { lat:        261, common_pid:       2036 } hitcount:          5
 { lat:        261, common_pid:       2039 } hitcount:          7
 { lat:        262, common_pid:       2039 } hitcount:          5
 { lat:        262, common_pid:       2036 } hitcount:          5
 { lat:        263, common_pid:       2039 } hitcount:          7
 { lat:        263, common_pid:       2036 } hitcount:          7
 { lat:        264, common_pid:       2039 } hitcount:          9
 { lat:        264, common_pid:       2036 } hitcount:          9
 { lat:        265, common_pid:       2036 } hitcount:          5
 { lat:        265, common_pid:       2039 } hitcount:          1
 { lat:        266, common_pid:       2036 } hitcount:          1
 { lat:        266, common_pid:       2039 } hitcount:          3
 { lat:        267, common_pid:       2036 } hitcount:          1
 { lat:        267, common_pid:       2039 } hitcount:          3
 { lat:        268, common_pid:       2036 } hitcount:          1
 { lat:        268, common_pid:       2039 } hitcount:          6
 { lat:        269, common_pid:       2036 } hitcount:          1
 { lat:        269, common_pid:       2043 } hitcount:          1
 { lat:        269, common_pid:       2039 } hitcount:          2
 { lat:        270, common_pid:       2040 } hitcount:          1
 { lat:        270, common_pid:       2039 } hitcount:          6
 { lat:        271, common_pid:       2041 } hitcount:          1
 { lat:        271, common_pid:       2039 } hitcount:          5
 { lat:        272, common_pid:       2039 } hitcount:         10
 { lat:        273, common_pid:       2039 } hitcount:          8
 { lat:        274, common_pid:       2039 } hitcount:          2
 { lat:        275, common_pid:       2039 } hitcount:          1
 { lat:        276, common_pid:       2039 } hitcount:          2
 { lat:        276, common_pid:       2037 } hitcount:          1
 { lat:        276, common_pid:       2038 } hitcount:          1
 { lat:        277, common_pid:       2039 } hitcount:          1
 { lat:        277, common_pid:       2042 } hitcount:          1
 { lat:        278, common_pid:       2039 } hitcount:          1
 { lat:        279, common_pid:       2039 } hitcount:          4
 { lat:        279, common_pid:       2043 } hitcount:          1
 { lat:        280, common_pid:       2039 } hitcount:          3
 { lat:        283, common_pid:       2036 } hitcount:          2
 { lat:        284, common_pid:       2039 } hitcount:          1
 { lat:        284, common_pid:       2043 } hitcount:          1
 { lat:        288, common_pid:       2039 } hitcount:          1
 { lat:        289, common_pid:       2039 } hitcount:          1
 { lat:        300, common_pid:       2039 } hitcount:          1
 { lat:        384, common_pid:       2039 } hitcount:          1

 Totals:
     Hits: 67625
     Entries: 278
     Dropped: 0

```
娉ㄦ剰锛屽啓鎿嶄綔鍙戠敓鍦ㄤ紤鐪犲墠鍚庯紝鍥犳鐞嗘兂鎯呭喌涓嬪畠浠兘搴斾负 250 寰銆傚鏋滀綘濂藉涓轰粈涔?
浼氭湁鑻ュ共娆″皬浜?250 寰锛岄偅鏄洜涓?cyclictest 鐨勫伐浣滄柟寮忥細濡傛灉鏌愭杩唬鏉ヨ繜浜嗭紝
涓嬩竴娆″氨浼氭妸瀹氭椂鍣ㄨ缃负鍦ㄥ皬浜?250 寰鍚庡敜閱掋€備篃灏辨槸璇达紝濡傛灉鏌愭杩唬鏅氫簡
50 寰锛屽垯涓嬩竴娆″敜閱掍細鍦?200 寰鍚庛€?

浣嗚繖鍦ㄧ敤鎴风┖闂翠腑涔熷緢瀹规槗瀹炵幇銆備负浜嗚瀹冩洿鏈夋剰鎬濓紝鎴戜滑鍙互鎶婂彂鐢熷湪
```

 # cd /sys/kernel/tracing
 # echo 'latency u64 lat' > synthetic_events
 # echo 'hist:keys=pid:ts0=common_timestamp.usecs' > events/sched/sched_waking/trigger
 # echo 'hist:keys=common_pid:lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).latency($lat) if buf == "end"' > events/ftrace/print/trigger
 # echo 'hist:keys=lat,common_pid:sort=lat' > events/synthetic/latency/trigger

```
杩欐鐨勪笉鍚屼箣澶勫湪浜庯紝涓嶅啀浣跨敤 trace_marker 鏉ュ惎鍔ㄥ欢杩熻鏃讹紝鑰屾槸浣跨敤 sched_waking
浜嬩欢锛屽皢 trace_marker 鍐欏叆鏃剁殑 common_pid 涓?sched_waking 姝ｅ湪鍞ら啋鐨?pid 杩涜鍖归厤銆?
```

 # cat events/synthetic/latency/hist
 # event histogram
 #
 # trigger info: hist:keys=lat,common_pid:vals=hitcount:sort=lat:size=2048 [active]
 #

 { lat:          7, common_pid:       2302 } hitcount:        640
 { lat:          7, common_pid:       2299 } hitcount:         42
 { lat:          7, common_pid:       2303 } hitcount:         18
 { lat:          7, common_pid:       2305 } hitcount:        166
 { lat:          7, common_pid:       2306 } hitcount:          1
 { lat:          7, common_pid:       2301 } hitcount:         91
 { lat:          7, common_pid:       2300 } hitcount:         17
 { lat:          8, common_pid:       2303 } hitcount:       8296
 { lat:          8, common_pid:       2304 } hitcount:       6864
 { lat:          8, common_pid:       2305 } hitcount:       9464
 { lat:          8, common_pid:       2301 } hitcount:       9213
 { lat:          8, common_pid:       2306 } hitcount:       6246
 { lat:          8, common_pid:       2302 } hitcount:       8797
 { lat:          8, common_pid:       2299 } hitcount:       8771
 { lat:          8, common_pid:       2300 } hitcount:       8119
 { lat:          9, common_pid:       2305 } hitcount:       1519
 { lat:          9, common_pid:       2299 } hitcount:       2346
 { lat:          9, common_pid:       2303 } hitcount:       2841
 { lat:          9, common_pid:       2301 } hitcount:       1846
 { lat:          9, common_pid:       2304 } hitcount:       3861
 { lat:          9, common_pid:       2302 } hitcount:       1210
 { lat:          9, common_pid:       2300 } hitcount:       2762
 { lat:          9, common_pid:       2306 } hitcount:       4247
 { lat:         10, common_pid:       2299 } hitcount:         16
 { lat:         10, common_pid:       2306 } hitcount:        333
 { lat:         10, common_pid:       2303 } hitcount:         16
 { lat:         10, common_pid:       2304 } hitcount:        168
 { lat:         10, common_pid:       2302 } hitcount:        240
 { lat:         10, common_pid:       2301 } hitcount:         28
 { lat:         10, common_pid:       2300 } hitcount:         95
 { lat:         10, common_pid:       2305 } hitcount:         18
 { lat:         11, common_pid:       2303 } hitcount:          5
 { lat:         11, common_pid:       2305 } hitcount:          8
 { lat:         11, common_pid:       2306 } hitcount:        221
 { lat:         11, common_pid:       2302 } hitcount:         76
 { lat:         11, common_pid:       2304 } hitcount:         26
 { lat:         11, common_pid:       2300 } hitcount:        125
 { lat:         11, common_pid:       2299 } hitcount:          2
 { lat:         12, common_pid:       2305 } hitcount:          3
 { lat:         12, common_pid:       2300 } hitcount:          6
 { lat:         12, common_pid:       2306 } hitcount:         90
 { lat:         12, common_pid:       2302 } hitcount:          4
 { lat:         12, common_pid:       2303 } hitcount:          1
 { lat:         12, common_pid:       2304 } hitcount:        122
 { lat:         13, common_pid:       2300 } hitcount:         12
 { lat:         13, common_pid:       2301 } hitcount:          1
 { lat:         13, common_pid:       2306 } hitcount:         32
 { lat:         13, common_pid:       2302 } hitcount:          5
 { lat:         13, common_pid:       2305 } hitcount:          1
 { lat:         13, common_pid:       2303 } hitcount:          1
 { lat:         13, common_pid:       2304 } hitcount:         61
 { lat:         14, common_pid:       2303 } hitcount:          4
 { lat:         14, common_pid:       2306 } hitcount:          5
 { lat:         14, common_pid:       2305 } hitcount:          4
 { lat:         14, common_pid:       2304 } hitcount:         62
 { lat:         14, common_pid:       2302 } hitcount:         19
 { lat:         14, common_pid:       2300 } hitcount:         33
 { lat:         14, common_pid:       2299 } hitcount:          1
 { lat:         14, common_pid:       2301 } hitcount:          4
 { lat:         15, common_pid:       2305 } hitcount:          1
 { lat:         15, common_pid:       2302 } hitcount:         25
 { lat:         15, common_pid:       2300 } hitcount:         11
 { lat:         15, common_pid:       2299 } hitcount:          5
 { lat:         15, common_pid:       2301 } hitcount:          1
 { lat:         15, common_pid:       2304 } hitcount:          8
 { lat:         15, common_pid:       2303 } hitcount:          1
 { lat:         15, common_pid:       2306 } hitcount:          6
 { lat:         16, common_pid:       2302 } hitcount:         31
 { lat:         16, common_pid:       2306 } hitcount:          3
 { lat:         16, common_pid:       2300 } hitcount:          5
 { lat:         17, common_pid:       2302 } hitcount:          6
 { lat:         17, common_pid:       2303 } hitcount:          1
 { lat:         18, common_pid:       2304 } hitcount:          1
 { lat:         18, common_pid:       2302 } hitcount:          8
 { lat:         18, common_pid:       2299 } hitcount:          1
 { lat:         18, common_pid:       2301 } hitcount:          1
 { lat:         19, common_pid:       2303 } hitcount:          4
 { lat:         19, common_pid:       2304 } hitcount:          5
 { lat:         19, common_pid:       2302 } hitcount:          4
 { lat:         19, common_pid:       2299 } hitcount:          3
 { lat:         19, common_pid:       2306 } hitcount:          1
 { lat:         19, common_pid:       2300 } hitcount:          4
 { lat:         19, common_pid:       2305 } hitcount:          5
 { lat:         20, common_pid:       2299 } hitcount:          2
 { lat:         20, common_pid:       2302 } hitcount:          3
 { lat:         20, common_pid:       2305 } hitcount:          1
 { lat:         20, common_pid:       2300 } hitcount:          2
 { lat:         20, common_pid:       2301 } hitcount:          2
 { lat:         20, common_pid:       2303 } hitcount:          3
 { lat:         21, common_pid:       2305 } hitcount:          1
 { lat:         21, common_pid:       2299 } hitcount:          5
 { lat:         21, common_pid:       2303 } hitcount:          4
 { lat:         21, common_pid:       2302 } hitcount:          7
 { lat:         21, common_pid:       2300 } hitcount:          1
 { lat:         21, common_pid:       2301 } hitcount:          5
 { lat:         21, common_pid:       2304 } hitcount:          2
 { lat:         22, common_pid:       2302 } hitcount:          5
 { lat:         22, common_pid:       2303 } hitcount:          1
 { lat:         22, common_pid:       2306 } hitcount:          3
 { lat:         22, common_pid:       2301 } hitcount:          2
 { lat:         22, common_pid:       2300 } hitcount:          1
 { lat:         22, common_pid:       2299 } hitcount:          1
 { lat:         22, common_pid:       2305 } hitcount:          1
 { lat:         22, common_pid:       2304 } hitcount:          1
 { lat:         23, common_pid:       2299 } hitcount:          1
 { lat:         23, common_pid:       2306 } hitcount:          2
 { lat:         23, common_pid:       2302 } hitcount:          6
 { lat:         24, common_pid:       2302 } hitcount:          3
 { lat:         24, common_pid:       2300 } hitcount:          1
 { lat:         24, common_pid:       2306 } hitcount:          2
 { lat:         24, common_pid:       2305 } hitcount:          1
 { lat:         24, common_pid:       2299 } hitcount:          1
 { lat:         25, common_pid:       2300 } hitcount:          1
 { lat:         25, common_pid:       2302 } hitcount:          4
 { lat:         26, common_pid:       2302 } hitcount:          2
 { lat:         27, common_pid:       2305 } hitcount:          1
 { lat:         27, common_pid:       2300 } hitcount:          1
 { lat:         27, common_pid:       2302 } hitcount:          3
 { lat:         28, common_pid:       2306 } hitcount:          1
 { lat:         28, common_pid:       2302 } hitcount:          4
 { lat:         29, common_pid:       2302 } hitcount:          1
 { lat:         29, common_pid:       2300 } hitcount:          2
 { lat:         29, common_pid:       2306 } hitcount:          1
 { lat:         29, common_pid:       2304 } hitcount:          1
 { lat:         30, common_pid:       2302 } hitcount:          4
 { lat:         31, common_pid:       2302 } hitcount:          6
 { lat:         32, common_pid:       2302 } hitcount:          1
 { lat:         33, common_pid:       2299 } hitcount:          1
 { lat:         33, common_pid:       2302 } hitcount:          3
 { lat:         34, common_pid:       2302 } hitcount:          2
 { lat:         35, common_pid:       2302 } hitcount:          1
 { lat:         35, common_pid:       2304 } hitcount:          1
 { lat:         36, common_pid:       2302 } hitcount:          4
 { lat:         37, common_pid:       2302 } hitcount:          6
 { lat:         38, common_pid:       2302 } hitcount:          2
 { lat:         39, common_pid:       2302 } hitcount:          2
 { lat:         39, common_pid:       2304 } hitcount:          1
 { lat:         40, common_pid:       2304 } hitcount:          2
 { lat:         40, common_pid:       2302 } hitcount:          5
 { lat:         41, common_pid:       2304 } hitcount:          1
 { lat:         41, common_pid:       2302 } hitcount:          8
 { lat:         42, common_pid:       2302 } hitcount:          6
 { lat:         42, common_pid:       2304 } hitcount:          1
 { lat:         43, common_pid:       2302 } hitcount:          3
 { lat:         43, common_pid:       2304 } hitcount:          4
 { lat:         44, common_pid:       2302 } hitcount:          6
 { lat:         45, common_pid:       2302 } hitcount:          5
 { lat:         46, common_pid:       2302 } hitcount:          5
 { lat:         47, common_pid:       2302 } hitcount:          7
 { lat:         48, common_pid:       2301 } hitcount:          1
 { lat:         48, common_pid:       2302 } hitcount:          9
 { lat:         49, common_pid:       2302 } hitcount:          3
 { lat:         50, common_pid:       2302 } hitcount:          1
 { lat:         50, common_pid:       2301 } hitcount:          1
 { lat:         51, common_pid:       2302 } hitcount:          2
 { lat:         51, common_pid:       2301 } hitcount:          1
 { lat:         61, common_pid:       2302 } hitcount:          1
 { lat:        110, common_pid:       2302 } hitcount:          1

 Totals:
     Hits: 89565
     Entries: 158
     Dropped: 0

```
杩欏苟涓嶈兘鍛婅瘔鎴戜滑 cyclictest 鍙兘鏅氶啋浜嗗灏戯紝浣嗗畠纭疄鍚戞垜浠睍绀轰簡涓€涓笉閿欑殑鐩存柟鍥撅紝
鍙嶆槧浠?cyclictest 琚敜閱掑埌瀹冭繘鍏ョ敤鎴风┖闂存墍缁忓巻鐨勬椂闂淬€?
