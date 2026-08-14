## 事件直方图


本文档由 Tom Zanussi 编写


## 1. 简介


  直方图触发器是一类特殊的事件触发器，可用于将跟踪事件数据聚合为直方图。
  有关跟踪事件与事件触发器的更多信息，请参阅 Documentation/trace/events.rst。


## 2. 直方图触发器命令


  直方图触发器命令是一种事件触发器命令，它将事件命中聚合到一个哈希表中，
  该哈希表以一个（或多个）跟踪事件格式字段（或栈回溯）作为键，并以从
  一个（或多个）跟踪事件格式字段和/或事件计数（hitcount）派生出的一组
  累计总和作为值。
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

  enable_hist 与 disable_hist 触发器可用于让某个事件有条件地启动和停止另一个
  事件已附着的 hist 触发器。可以将任意数量的 enable_hist 与 disable_hist 触发器
  附加到给定事件上，从而让该事件启动并停止大量其他事件的聚合。
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
### 2.1. “特殊”事件字段


  有一系列“特殊事件字段”可用作 hist 触发器中的键或值。它们看起来和行为都像是
  真正的事件字段，但实际上并不是事件字段定义或格式文件的一部分。不过，它们对
  任何事件都可用，并且可以出现在真正事件字段能出现的任何地方。它们包括：

    ====================== ==== =======================================
    common_timestamp       u64  与事件关联的（来自环形缓冲区的）时间戳，
                                单位为纳秒。可通过 .usecs 修饰，使时间戳
		        被解释为微秒。
    common_cpu             int  事件发生所在的 CPU。
    ====================== ==== =======================================

### 2.2. 扩展错误信息


  在调用 hist 触发器命令时遇到某些错误条件，可通过 tracing/error_log 文件
  获得扩展错误信息。详情请参阅 Documentation/trace/ftrace.rst 中的“错误条件”
  一节。

### 2.3. ‘hist’ 触发器示例


  第一组示例使用 kmalloc 事件创建聚合。可用于 hist 触发器的字段列示于
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
### 2.4. 事件间直方图触发器


事件间直方图触发器是一类 hist 触发器，它把一个（或多个）其他事件的值组合起来，
并利用这些数据创建直方图。事件间直方图的 data 反过来又可以作为进一步组合直方图的
来源，从而形成一条相关的直方图链，这对某些应用很重要。

可以用这种方式使用的事件间量中，最重要的例子是延迟（latency），它其实就是两个
事件之间时间戳的差值。尽管延迟是最重要的事件间量，但请注意，由于该支持在整个
跟踪事件子系统中是完全通用的，因此任何事件字段都可以用于事件间量。

一个将来自其他直方图的数据组合成有用链条的直方图例子，是“wakeupswitch latency”
直方图，它把“wakeup latency”直方图与“switch latency”直方图组合在一起。

通常，一个 hist 触发器规格说明包含一个（可能为复合的）键，以及一或多个数值，
这些数值是与该键关联、持续更新的总和。在这种情况下，直方图规格说明由单个键与
值的规格组成，它们引用与单一事件类型关联的跟踪事件字段。

事件间直方图触发器扩展允许引用来自多个事件的字段，并将其组合为一个多事件直方图
规格说明。为了支持这一总体目标，向 hist 触发器支持中新增了若干使能特性：

  - 为了计算事件间量，需要把一个事件中的值保存下来，然后再由另一个事件引用。
    这就要求引入对直方图“变量（variables）”的支持。

  - 事件间量的计算及其组合，要求对变量应用简单表达式（加和减）提供最基本的支持。

  - 由事件间量构成的直方图，在逻辑上并不是任一事件的直方图（因此让任一事件的
    'hist' 文件来承载直方图输出并不合理）。为了体现该直方图与一组事件的组合相关联
    这一概念，新增了支持以允许创建“合成（synthetic）”事件，即从其他事件派生出来的
    事件。这些合成事件与任何其他事件一样是完备的事件，可以按此方式使用，例如用来
    创建前面提到的“组合”直方图。

  - 一组“动作（actions）”可以与直方图条目相关联——这些动作既可以用来生成前面
    提到的合成事件，也可以用于其他目的，例如当命中某个“最大（max）”延迟时保存
    上下文。

  - 跟踪事件本身并不带有与之关联的“时间戳”，但在底层的 ftrace 环形缓冲区中，
    与每个事件一起保存了一个隐式的时间戳。该时间戳现在以一个名为 'common_timestamp'
    的合成字段的形式暴露出来，可以像其他任何事件字段一样在直方图中使用；它并不是
    跟踪格式中的真实字段，而是一个合成出来的值，尽管如此仍可以像真实字段一样使用。
    默认情况下其单位为纳秒；在 common_timestamp 字段后附加 '.usecs' 可将单位改为微秒。

关于事件间时间戳的注意事项：如果在直方图中使用了 common_timestamp，跟踪缓冲区会
自动切换为使用绝对时间戳和“global”跟踪时钟，以避免与其他在跨 CPU 时不连续的时钟
之间出现虚假的时间戳差异。也可以通过改用其他跟踪时钟来覆盖这一行为，即使用
"clock=XXX" hist 触发器属性，其中 XXX 是 tracing/trace_clock 伪文件中列出的
任一时钟。

这些特性在后续各节中有更详细的说明。

### 2.5. 直方图变量


变量就是简单的命名位置，用于在匹配的事件之间保存和检索值。所谓“匹配（matching）”
事件，是指拥有匹配键的事件——如果为某个对应于该键的直方图条目保存了一个变量，
那么任何拥有匹配键的后续事件都可以访问该变量。

变量的值通常对任何后续事件都可用，直到它被某个后续事件设为其他值为止。该规则
唯一的例外是：任何在表达式中使用的变量本质上都是“读一次（read-once）”的——一旦
它被后续事件中的某个表达式使用，就会被重置为“未设置（unset）”状态，这意味着除非
再次设置，否则不能再次使用。这不仅确保事件不会在计算中使用未初始化的变量，也确保
该变量只被使用一次，而不会用于任何不相关的后续匹配。

保存变量的基本语法是：简单地把一个不对应任何关键字的唯一变量名，连同 '=' 号作为
前缀，加在任意事件字段上。

键或值都可以用这种方式保存和检索。这会为带有该键的直方图条目创建一个名为 'ts0'
的变量
```

  # echo 'hist:keys=next_pid:vals=$ts0:ts0=common_timestamp ... >> \
	event/trigger

```
ts0 变量可以被任何拥有与 'next_pid' 相同 pid 的后续事件访问。

变量引用是通过在变量名前加 '$' 符号构成的。因此，例如上面的 ts0 变量在表达式中
会被引用为 '$ts0'。

由于使用了 'vals='，上面这个 common_timestamp 变量的值也会像普通直方图值一样被
求和（尽管对于时间戳而言这没什么意义）。
```

  # echo 'hist:timer_pid=common_pid:key=timer_pid ...' >> event/trigger

```
如果一个变量不是键变量，也没有以 'vals=' 为前缀，那么相关联的事件字段会被
保存到变量中，但不会被求和
```

  # echo 'hist:keys=next_pid:ts1=common_timestamp ...' >> event/trigger

```
可以同时赋值多个变量。下面这行会把 ts0 和 b 都创建为变量，二者
```

  # echo 'hist:keys=pid:vals=$ts0,$b:ts0=common_timestamp,b=field1 ...' >> \
	event/trigger

```
注意，变量赋值既可以出现在使用之前，也可以出现在使用之后。下面这条命令与
```

  # echo 'hist:keys=pid:ts0=common_timestamp,b=field1:vals=$ts0,$b ...' >> \
	event/trigger

```
任意数量的、未绑定到 'vals=' 前缀的变量，也可以通过用冒号分隔来赋值。
下面同样是
```

  # echo 'hist:keys=pid:ts0=common_timestamp:b=field1 ...' >> event/trigger

```
按照上述方式设置的变量，可以在另一个事件的表达式中引用和使用。
```

  # echo 'hist:keys=pid,prio:ts0=common_timestamp ...' >> event1/trigger
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp-$ts0 ...' >> event2/trigger

```
在上面第一行中，事件的时间戳被保存到变量 ts0 中。在下一行中，从第二个事件
的时间戳减去 ts0，得到延迟值，并将其赋给另一个变量 'wakeup_lat'。下面的
直方图触发器进而利用 wakeup_lat 变量来计算组合延迟
```

  # echo 'hist:key=pid:wakeupswitch_lat=$wakeup_lat+$switchtime_lat ...' >> event3/trigger

```
表达式支持使用加法、减法、乘法和除法运算符（+-*/）。

注意，如果在解析时无法检测到除零（即除数不是常量），则结果为 -1。
```

  # echo 'hist:keys=next_pid:timestamp_secs=common_timestamp/1000000 ...' >> event/trigger

```

```

  # echo 'hist:keys=next_pid:us_per_sec=1000000 ...' >> event/trigger
  # echo 'hist:keys=next_pid:timestamp_secs=common_timestamp/$us_per_sec ...' >> event/trigger

```
变量甚至可以保存栈回溯，这在合成事件中很有用。

### 2.6. 合成事件


合成事件是由 hist 触发器变量或与一个（或多个）其他事件关联的字段所生成的、
用户自定义的事件。它们的目的是提供一种机制，以与现有且已为人所熟悉的普通事件
用法一致的方式，展示跨越多个事件的数据。

要定义一个合成事件，用户需编写一个简单的规格说明，其中包含新事件的名称以及
一个或多个变量及其类型（可以是任何合法的字段类型），用分号分隔，写入
tracing/synthetic_events 文件。

可用类型请参阅 synth_field_size()。

如果字段名包含 [n]，则该字段被视为静态数组。

如果字段名包含 []（无下标），则该字段被视为动态数组，它只会占用事件中与
容纳该数组所需同样大小的空间。

字符串字段可以使用静态记法指定：

  char name[^32^];

或者使用动态记法：

  char name[];

两者的尺寸上限均为 256。

例如，下面创建一个名为 'wakeup_latency' 的新事件，包含 3 个字段：lat、pid
和 prio。这些字段中的每一个都只是
```

  # echo 'wakeup_latency \
          u64 lat; \
          pid_t pid; \
	  int prio' >> \
	  /sys/kernel/tracing/synthetic_events

```
读取 tracing/synthetic_events 文件会列出当前所有已定义的
```

  # cat /sys/kernel/tracing/synthetic_events
    wakeup_latency u64 lat; pid_t pid; int prio

```
已有的合成事件定义可以通过在前面加上
```

  # echo '!wakeup_latency u64 lat pid_t pid int prio' >> \
    /sys/kernel/tracing/synthetic_events

```
此时，事件子系统中还没有真正实例化出一个 'wakeup_latency' 事件——要做到
这一点，需要实例化一个“hist 触发器动作”并将其绑定到其他事件上定义的真实
字段与变量（关于如何使用 hist 触发器的 'onmatch' 动作来完成，请参见下面的
2.7 节）。一旦完成，就会创建出 'wakeup_latency' 合成事件实例。

新事件创建在 tracing/events/synthetic/ 目录下
```

  # ls /sys/kernel/tracing/events/synthetic/wakeup_latency
        enable  filter  format  hist  id  trigger

```

```

  # echo 'hist:keys=pid,prio,lat.log2:sort=lat' >> \
        /sys/kernel/tracing/events/synthetic/wakeup_latency/trigger

```
上面以 2 的幂次分组展示了延迟 "lat"。

与任何其他事件一样，一旦为该事件启用了直方图，
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
延迟值也可以按给定大小进行线性分组，
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
要保存栈回溯，可创建一个带有 "unsigned long[]" 类型字段（甚至只需 "long[]"）
的合成事件。例如，要查看某个任务在被
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
带有栈回溯字段的合成事件，可将其用作键，
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
### 2.7. 直方图触发器的“处理器”与“动作”


直方图触发器的“动作”是一个函数，每当有直方图条目被添加或更新时（大多数情况下
是有条件地）执行。

当直方图条目被添加或更新时，由直方图触发器的“处理器”来决定相应的动作是否真正
被调用。

直方图触发器的处理器与动作以如下一般形式成对组合：

  <handler>.<action>

要为给定事件指定一个 handler.action 对，只需在 hist 触发器规格说明中、用冒号
将该 handler.action 对括起来即可。

理论上，任何处理器都可以与任何动作组合，但在实践中，并非每一种 handler.action
组合当前都受支持；如果某个 handler.action 组合不受支持，hist 触发器将以 -EINVAL
失败。

如果未显式指定，默认的“handler.action”与以往一样，仅仅是更新与某条目关联的一组
值。不过，某些应用可能希望此时执行额外的动作，例如生成另一个事件，或者比较并
保存最大值。

受支持的处理器与动作列于下方，并在后续段落中结合一些常见且有用的 handler.action
组合的描述进行更详细的说明。

可用的处理器有：

  - onmatch(matching.event)    - 在任意添加或更新时调用动作
  - onmax(var)                 - 当 var 超过当前最大值时调用动作
  - onchange(var)              - 当 var 发生变化时调用动作

可用的动作有：

  - trace(<synthetic_event_name>,param list)   - 生成合成事件
  - save(field,...)                            - 保存当前事件字段
  - snapshot()                                 - 对跟踪缓冲区做快照

以下是常用的 handler.action 组合：

  - onmatch(matching.event).trace(<synthetic_event_name>,param list)

    'onmatch(matching.event).trace(<synthetic_event_name>,param
    list)' 直方图触发器动作在事件匹配且直方图条目将被添加或更新时被调用。它会
    以 'param list' 中给出的值生成指定的合成事件。其结果是生成一个合成事件，
    该事件由调用事件发生时刻那些变量中所包含的值构成。例如，如果合成事件名是
    'wakeup_latency'，则使用 onmatch(event).trace(wakeup_latency,arg1,arg2)
    来生成 wakeup_latency 事件。

    此外还有另一种等价形式可用于生成合成事件。在这种形式中，合成事件名被当作
    函数名来使用。例如，再次使用 'wakeup_latency' 合成事件名，则 wakeup_latency
    事件会通过像函数调用一样的方式来生成，事件字段值作为实参传入：
    onmatch(event).wakeup_latency(arg1,arg2)。这种形式的语法为：

      onmatch(matching.event).<synthetic_event_name>(param list)

    无论哪种情况，'param list' 都由一个或多个参数组成，这些参数既可以是定义于
    'matching.event' 或目标事件上的变量，也可以是字段。param list 中指定的变量
    或字段既可以是完全限定名，也可以是非限定名。如果变量以非限定名指定，则它在
    两个事件之间必须唯一。作为 param 使用的字段名，如果引用的是目标事件，可以
    是非限定名；但如果引用的是匹配事件，则必须是完全限定名。完全限定名的格式为
    'system.event_name.$var_name' 或 'system.event_name.field'。

    'matching.event' 规格说明其实就是与 onmatch() 功能中目标事件相匹配的那个事件
    的完全限定事件名，格式为 'system.event_name'。会比较两个事件的直方图键以确定
    事件是否匹配。如果使用了多个直方图键，则它们必须按指定顺序全部匹配。

    最后，'param list' 中变量/字段的数量与类型，必须与所生成的合成事件中字段的
    数量与类型相匹配。

    作为示例，下面定义了一个简单的合成事件，并在调用该合成事件时，使用定义于
    sched_wakeup_new 事件上的一个变量作为参数。这里我们先定义该合成
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
### 2.8. 用户空间创建触发器


写入 /sys/kernel/tracing/trace_marker 会写入 ftrace 环形缓冲区。通过写入位于
/sys/kernel/tracing/events/ftrace/print/ 的触发器文件，它也可以像事件一样起作用。

修改 cyclictest，使其在休眠前写入 trace_marker 文件
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
上面创建了一个名为 "latency" 的合成事件，以及两个针对 trace_marker 的直方图：
当向 trace_marker 文件写入 "start" 时触发其中一个，写入 "end" 时触发另一个。
如果 pid 匹配，则它会以计算得到的延迟作为参数来调用 "latency" 合成事件。最后，
向 latency 合成事件添加一个直方图，以记录计算得到的延迟以及 pid。
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
注意，-b 1000 只是为了启用 --tracemark 而使用。
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
注意，写操作发生在休眠前后，因此理想情况下它们都应为 250 微秒。如果你好奇为什么
会有若干次小于 250 微秒，那是因为 cyclictest 的工作方式：如果某次迭代来迟了，
下一次就会把定时器设置为在小于 250 微秒后唤醒。也就是说，如果某次迭代晚了
50 微秒，则下一次唤醒会在 200 微秒后。

但这在用户空间中也很容易实现。为了让它更有意思，我们可以把发生在
```

 # cd /sys/kernel/tracing
 # echo 'latency u64 lat' > synthetic_events
 # echo 'hist:keys=pid:ts0=common_timestamp.usecs' > events/sched/sched_waking/trigger
 # echo 'hist:keys=common_pid:lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).latency($lat) if buf == "end"' > events/ftrace/print/trigger
 # echo 'hist:keys=lat,common_pid:sort=lat' > events/synthetic/latency/trigger

```
这次的不同之处在于，不再使用 trace_marker 来启动延迟计时，而是使用 sched_waking
事件，将 trace_marker 写入时的 common_pid 与 sched_waking 正在唤醒的 pid 进行匹配。
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
这并不能告诉我们 cyclictest 可能晚醒了多少，但它确实向我们展示了一个不错的直方图，
反映从 cyclictest 被唤醒到它进入用户空间所经历的时间。
