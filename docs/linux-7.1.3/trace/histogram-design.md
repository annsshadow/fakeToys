## 直方图设计说



:作 Tom Zanussi <zanussi@kernel.org>

本文档试图描ftrace 直方图是如何工作的，以及各个组成部分如何映射到用于在 trace_events_hist.c tracing_map.c 中实现它们的数据结构

   以下所ftrace 直方图命令示例都假定当前工作目录

```
	# cd /sys/kernel/tracing

   Also, the histogram output displayed for those commands will be generally be truncated - only enough to make the point is displayed.

```

## 'hist_debug' 跟踪事件文件



如果内核编译时设置了 CONFIG_HIST_TRIGGERS_DEBUG，则会在每个事件的子目录中出现一个名'hist_debug' 的事件文件。该文件可随时读取，并将显示本文档所述的部分直方图触发器内部状态。具体的示例和输出将在下面的测试用例中描述

## 基础直方



首先是最基础的直方图。下面几乎是你用直方图能做的最简单的事情——在单个事件上用单个键创建一个直方图

```
  # echo 'hist:keys=pid' >> events/sched/sched_waking/trigger

  # cat events/sched/sched_waking/hist

  { pid:      18249 } hitcount:          1
  { pid:      13399 } hitcount:          1
  { pid:      17973 } hitcount:          1
  { pid:      12572 } hitcount:          1
  ...
  { pid:         10 } hitcount:        921
  { pid:      18255 } hitcount:       1444
  { pid:      25526 } hitcount:       2055
  { pid:       5257 } hitcount:       2055
  { pid:      27367 } hitcount:       2055
  { pid:       1728 } hitcount:       2161

  Totals:
    Hits: 21305
    Entries: 183
    Dropped: 0

```

这段代码sched_waking 事件上创建了一个以 pid 为键、以单个hitcount 为值的直方图。hitcount 即使没有被显式指定，也始终存在于每个直方图之中

hitcount 值是一个每个桶（bucket）对应的值，会在该键每次命中时自动递增，在本例中该键就pid

因此在这个直方图中，每个 pid 都有一个独立的桶，每个桶中包含一个对应的值，用于统计pid 调用 sched_waking 的次数

每个直方图都由一hist_data 结构体（struct hist_trigger_data）表示

为了跟踪直方图中的每个键和值字段，hist_data 维护了一个名fields[] 的这类字段数组。fields[] 数组是一个包含每个直方图键和值（还包括变量，稍后讨论）所对应struct hist_field 表示的数组。所以对于上面的直方图，我们有一个键和一个值；在本例中，这一个值是 hitcount 值，所有直方图都拥有它，无论它们是否定义了该值，而上面的直方图并没有定义它

每个 struct hist_field 都包含一个指向事trace_event_file ftrace_event_field 的指针，以及与之相关的各类信息，如大小、偏移、类型，还有一hist field 函数，用于从 ftrace 事件缓冲区中取出该字段的数据（大多数情况下如此——有hist_field 比如 hitcount 并不直接映射到跟踪缓冲区中的事件字段，在这些情况下，其函数实现从别处取得值）。flags 字段指示该字段属于哪种类型——键、值、变量、变量引用等，默认是值

除了 fields[] 数组之外，另一个重要的 hist_data 数据结构是为该直方图创建tracing_map 实例，它保存.map 成员中。tracing_map 实现了用于实现直方图的免锁哈希表（关于实tracing_map 的底层数据结构，请参kernel/trace/tracing_map.h 中的大量讨论）。就本讨论而言，tracing_map 包含若干个桶，每个桶对应一个由给定直方图键哈希得到tracing_map_elt 对象

下面是一张图，其第一部分描述了上述直方图hist_data 以及相关的键和值字段。正如你所看到的，fields 数组中有两个字段，一个是 hitcount val 字段，另一个是 pid 键的 key 字段

下面是该 tracing_map 在某个运行时刻可能呈现的快照图。它试图展示 hist_data 字段tracing_map 之间的关系：

```
  +------------------+
  | hist_data        |
  +------------------+     +----------------+
    | .fields[]      |---->| val = hitcount |----------------------------+
    +----------------+     +----------------+                            |
    | .map           |       | .size        |                            |
    +----------------+       +--------------+                            |
                             | .offset      |                            |
                             +--------------+                            |
                             | .fn()        |                            |
                             +--------------+                            |
                                   .                                     |
                                   .                                     |
                                   .                                     |
                           +----------------+ <--- n_vals                |
                           | key = pid      |----------------------------|--+
                           +----------------+                            |  |
                             | .size        |                            |  |
                             +--------------+                            |  |
                             | .offset      |                            |  |
                             +--------------+                            |  |
                             | .fn()        |                            |  |
                           +----------------+ <--- n_fields              |  |
                           | unused         |                            |  |
                           +----------------+                            |  |
                             |              |                            |  |
                             +--------------+                            |  |
                             |              |                            |  |
                             +--------------+                            |  |
                             |              |                            |  |
                             +--------------+                            |  |
                                            n_keys = n_fields - n_vals   |  |

```

hist_data n_vals n_fields 划定fields[] 数组的范围，并把键和值从代码其余部分中分离出来

下面是一个运行时刻的 tracing_map 部分表示图，展示了从 fields[] 数组的各个部分指tracing_map 对应部分的指针

tracing_map 由一tracing_map_entry 数组和一组预分配tracing_map_elt（下图简写为 map_entry map_elt）组成。hist_data.map 数组中的 map_entry 总数 = map->max_elts（实际上map->map_size，但其中只有 max_elts 个被使用。这map_insert() 算法所需的一个属性）

如果一map_entry 未被使用，即还没有键哈希到它，则它的 .key 值为 0，其 .val 指针NULL。一旦某map_entry 被占用，.key 值就包含该键的哈希值，.val 成员指向一map_elt，其中包含完整的键以map_elt.fields[] 数组中每个键或值对应的一个条目。map_elt.fields[] 数组中有一个条目对应于直方图中的每hist_field，而每个直方图值所对应的、持续聚合的求和值就保存在这里

该图试图展示 hist_data.fields[] map_elt.fields[] 之间的关系，图中用连线绘制了这种关联

```
  +-----------+		                                                 |  |
  | hist_data |		                                                 |  |
  +-----------+		                                                 |  |
    | .fields |		                                                 |  |
    +---------+     +-----------+		                         |  |
    | .map    |---->| map_entry |		                         |  |
    +---------+     +-----------+		                         |  |
                      | .key    |---> 0		                         |  |
                      +---------+		                         |  |
                      | .val    |---> NULL		                 |  |
                    +-----------+                                        |  |
                    | map_entry |                                        |  |
                    +-----------+                                        |  |
                      | .key    |---> pid = 999                          |  |
                      +---------+    +-----------+                       |  |
                      | .val    |--->| map_elt   |                       |  |
                      +---------+    +-----------+                       |  |
                           .           | .key    |---> full key *        |  |
                           .           +---------+    +---------------+  |  |
			   .           | .fields |--->| .sum (val)    |<-+  |
                    +-----------+      +---------+    | 2345          |  |  |
                    | map_entry |                     +---------------+  |  |
                    +-----------+                     | .offset (key) |<----+
                      | .key    |---> 0               | 0             |  |  |
                      +---------+                     +---------------+  |  |
                      | .val    |---> NULL                    .          |  |
                    +-----------+                             .          |  |
                    | map_entry |                             .          |  |
                    +-----------+                     +---------------+  |  |
                      | .key    |                     | .sum (val) or |  |  |
                      +---------+    +---------+      | .offset (key) |  |  |
                      | .val    |--->| map_elt |      +---------------+  |  |
                    +-----------+    +---------+      | .sum (val) or |  |  |
                    | map_entry |                     | .offset (key) |  |  |
                    +-----------+                     +---------------+  |  |
                      | .key    |---> pid = 4444                         |  |
                      +---------+    +-----------+                       |  |
                      | .val    |    | map_elt   |                       |  |
                      +---------+    +-----------+                       |  |
                                       | .key    |---> full key *        |  |
                                       +---------+    +---------------+  |  |
			               | .fields |--->| .sum (val)    |<-+  |
                                       +---------+    | 65523         |     |
                                                      +---------------+     |
                                                      | .offset (key) |<----+
                                                      | 0             |
                                                      +---------------+
                                                              .
                                                              .
                                                              .
                                                      +---------------+
                                                      | .sum (val) or |
                                                      | .offset (key) |
                                                      +---------------+
                                                      | .sum (val) or |
                                                      | .offset (key) |
                                                      +---------------+

```

```
  hist_data = struct hist_trigger_data
  hist_data.fields = struct hist_field
  fn = hist_field_fn_t
  map_entry = struct tracing_map_entry
  map_elt = struct tracing_map_elt
  map_elt.fields = struct tracing_map_field

```

每当发生一个新事件并且它关联了一hist 触发器时，就会调event_hist_trigger()。event_hist_trigger() 首先处理键：对于键中的每个子键（在上面的例子中，只有一个对应于 pid 的子键），会hist_data.fields[] 中取出表示该子键hist_field，并利用与该字段关联hist field 函数，以及字段的大小和偏移，从当前跟踪记录中取出该子键的数据

注意，hist field 函数曾经hist_field 结构中的一个函数指针。由于针Spectre 的缓解措施，它被改成fn_num，并且使hist_fn_call() 来调用对应于 hist_field 结构fn_num hist field 函数

一旦取回完整的键，就用它到 tracing_map 中查找该键。如果没有与该键关联tracing_map_elt，就会申请一个空的并插入到映射中供新键使用。无论哪种情况，都会返回与该键关联的 tracing_map_elt

一旦获得了 tracing_map_elt，就会调hist_trigger_elt_update()。顾名思义，它更新该元素，这基本上意味着更新该元素的字段。直方图中的每个键和值都关联着一tracing_map_field，它们各自对应于创建直方图时所创建的键和hist_field。hist_trigger_elt_update() 遍历每个hist_field，并像处理键那样，利hist_field 的函数、大小和偏移从当前跟踪记录中取出该字段的值。一旦取到该值，它就简单地把这个值加到该字段持续更新tracing_map_field.sum 成员上。有hist_field 函数，比hitcount，实际上并不从跟踪记录中取任何东西（hitcount 函数只是把计数器 sum 1），但思路是一样的

一旦所有值都被更新，hist_trigger_elt_update() 就完成并返回。注意，键中的每个子键也有对应的 tracing_map_field，但 hist_trigger_elt_update() 并不会查看或更新它们——它们只用于排序，而这可以稍后进行

### 基础直方图测



这是一个值得尝试的好例子。它产生 3 个值字段和 2 个键

```
  # echo 'hist:keys=common_pid,call_site.sym:values=bytes_req,bytes_alloc,hitcount' >> events/kmem/kmalloc/trigger

```

要查看调试数据，可以 cat 一kmem/kmalloc 'hist_debug' 文件。它会显示该直方图对应的触发器信息，以及与该直方图关联的 hist_data 的地址，这在后面的例子中会很有用。随后它会显示与该直方图关联的hist_field 数量，以及其中有多少对应于键、多少对应于值

接着它会显示每个字段的详细信息，包括该字段的 flags，以及每个字段在 hist_data fields[] 数组中的位置，这些信息对于验证内部状态是否正确非常有用，并且同样会在后面变得

```
  # cat events/kmem/kmalloc/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=common_pid,call_site.sym:vals=hitcount,bytes_req,bytes_alloc:sort=hitcount:size=2048 [active]
  #

  hist_data: 000000005e48c9a5

  n_vals: 3
  n_keys: 2
  n_fields: 5

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        VAL: normal u64 value
      ftrace_event_field name: bytes_req
      type: size_t
      size: 8
      is_signed: 0

    hist_data->fields[2]:
      flags:
        VAL: normal u64 value
      ftrace_event_field name: bytes_alloc
      type: size_t
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[3]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: common_pid
      type: int
      size: 8
      is_signed: 1

    hist_data->fields[4]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: call_site
      type: unsigned long
      size: 8
      is_signed: 0

```

```
  # echo '!hist:keys=common_pid,call_site.sym:values=bytes_req,bytes_alloc,hitcount' >> events/kmem/kmalloc/trigger

```

## 变量



变量允许一个直方图触发器保存的数据被另一个直方图触发器获取。例如，sched_waking 事件上的触发器可以捕获某个特pid 的时间戳，稍后切换到pid sched_switch 事件可以获取该时间戳并用它来计算时间差：

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >>
          events/sched/sched_waking/trigger

  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >>
          events/sched/sched_switch/trigger

```

就直方图数据结构而言，变量被实现为另一种类型的 hist_field，对于给定的 hist 触发器，它们被添加到所val 字段之后hist_data.fields[] 数组中。为了把它们与已有的键和值字段区分开，给它们赋予了一种新的标志类HIST_FIELD_FL_VAR（简写为 FL_VAR），并且它们还利用了 struct hist_field 中一个新.var.idx 字段成员，该成员将变量映射到一个专门新增的、用于存储和获取变量值的 map_elt.vars[] 数组的某个索引。下面的图展示了这些新元素，并新增了一个对应于上面 sched_waking 触发器中 ts0 变量的新变量条目 ts0

### sched_waking 鐩存柟鍥。



  +------------------+
  | hist_data        |<-------------------------------------------------------+
  +------------------+   +-------------------+                                |
    | .fields[]      |-->| val = hitcount    |                                |
    +----------------+   +-------------------+                                |
    | .map           |     | .size           |                                |
    +----------------+     +-----------------+                                |
                           | .offset         |                                |
                           +-----------------+                                |
                           | .fn()           |                                |
                           +-----------------+                                |
                           | .flags          |                                |
                           +-----------------+                                |
                           | .var.idx        |                                |
                         +-------------------+                                |
                         | var = ts0         |                                |
                         +-------------------+                                |
                           | .size           |                                |
                           +-----------------+                                |
                           | .offset         |                                |
                           +-----------------+                                |
                           | .fn()           |                                |
                           +-----------------+                                |
                           | .flags & FL_VAR |                                |
                           +-----------------+                                |
                           | .var.idx        |----------------------------+-+ |
                           +-----------------+                            | | |
			            .                                     | | |
				    .                                     | | |
                                    .                                     | | |
                         +-------------------+ <--- n_vals                | | |
                         | key = pid         |                            | | |
                         +-------------------+                            | | |
                           | .size           |                            | | |
                           +-----------------+                            | | |
                           | .offset         |                            | | |
                           +-----------------+                            | | |
                           | .fn()           |                            | | |
                           +-----------------+                            | | |
                           | .flags & FL_KEY |                            | | |
                           +-----------------+                            | | |
                           | .var.idx        |                            | | |
                         +-------------------+ <--- n_fields              | | |
                         | unused            |                            | | |
                         +-------------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                           |                 |                            | | |
                           +-----------------+                            | | |
                                             n_keys = n_fields - n_vals   | | |
                                                                          | | |

这与基础情形非常相似。在上图中，我们可以看到 struct hist_field 结构新增了一.flags 成员，并hist_data.fields 中新增了一个表ts0 变量的条目。对于一个普通的 val hist_fieldflags 只是 0（修饰符标志除外），但如果该值被定义为变量，.flags 会包含一FL_VAR 位

如你所见，ts0 条目.var.idx 成员包含了指向保存变量值的 tracing_map_elt .vars[] 数组的索引。每当设置或读取该变量的值时都会用到这个 idx。分配给给定变量map_elt.vars 索引，由 create_tracing_map_fields() 在调tracing_map_add_var() 之后赋值并保存.var.idx 中

下面是一个运行时刻的直方图表示图，它填充了映射，并与上面hist_data hist_field 数据结构相对应

该图试图展示 hist_data.fields[] map_elt.fields[] 以及 map_elt.vars[] 之间的关系，图中在两个图之间绘制了连线。对于每map_elt，你可以看到 .fields[] 成员指向某个键或值的 .sum .offset，.vars[] 成员指向某个变量的值。两图之间的箭头展示了这tracing_map 成员与相应字段定义之间的关联

```
  +-----------+		                                                  | | |
  | hist_data |		                                                  | | |
  +-----------+		                                                  | | |
    | .fields |		                                                  | | |
    +---------+     +-----------+		                          | | |
    | .map    |---->| map_entry |		                          | | |
    +---------+     +-----------+		                          | | |
                      | .key    |---> 0		                          | | |
                      +---------+		                          | | |
                      | .val    |---> NULL		                  | | |
                    +-----------+                                         | | |
                    | map_entry |                                         | | |
                    +-----------+                                         | | |
                      | .key    |---> pid = 999                           | | |
                      +---------+    +-----------+                        | | |
                      | .val    |--->| map_elt   |                        | | |
                      +---------+    +-----------+                        | | |
                           .           | .key    |---> full key *         | | |
                           .           +---------+    +---------------+   | | |
			   .           | .fields |--->| .sum (val)    |   | | |
                           .           +---------+    | 2345          |   | | |
                           .        +--| .vars   |    +---------------+   | | |
                           .        |  +---------+    | .offset (key) |   | | |
                           .        |                 | 0             |   | | |
                           .        |                 +---------------+   | | |
                           .        |                         .           | | |
                           .        |                         .           | | |
                           .        |                         .           | | |
                           .        |                 +---------------+   | | |
                           .        |                 | .sum (val) or |   | | |
                           .        |                 | .offset (key) |   | | |
                           .        |                 +---------------+   | | |
                           .        |                 | .sum (val) or |   | | |
                           .        |                 | .offset (key) |   | | |
                           .        |                 +---------------+   | | |
                           .        |                                     | | |
                           .        +---------------->+---------------+   | | |
			   .                          | ts0           |<--+ | |
                           .                          | 113345679876  |   | | |
                           .                          +---------------+   | | |
                           .                          | unused        |   | | |
                           .                          |               |   | | |
                           .                          +---------------+   | | |
                           .                                  .           | | |
                           .                                  .           | | |
                           .                                  .           | | |
                           .                          +---------------+   | | |
                           .                          | unused        |   | | |
                           .                          |               |   | | |
                           .                          +---------------+   | | |
                           .                          | unused        |   | | |
                           .                          |               |   | | |
                           .                          +---------------+   | | |
                           .                                              | | |
                    +-----------+                                         | | |
                    | map_entry |                                         | | |
                    +-----------+                                         | | |
                      | .key    |---> pid = 4444                          | | |
                      +---------+    +-----------+                        | | |
                      | .val    |--->| map_elt   |                        | | |
                      +---------+    +-----------+                        | | |
                           .           | .key    |---> full key *         | | |
                           .           +---------+    +---------------+   | | |
			   .           | .fields |--->| .sum (val)    |   | | |
                                       +---------+    | 2345          |   | | |
                                    +--| .vars   |    +---------------+   | | |
                                    |  +---------+    | .offset (key) |   | | |
                                    |                 | 0             |   | | |
                                    |                 +---------------+   | | |
                                    |                         .           | | |
                                    |                         .           | | |
                                    |                         .           | | |
                                    |                 +---------------+   | | |
                                    |                 | .sum (val) or |   | | |
                                    |                 | .offset (key) |   | | |
                                    |                 +---------------+   | | |
                                    |                 | .sum (val) or |   | | |
                                    |                 | .offset (key) |   | | |
                                    |                 +---------------+   | | |
                                    |                                     | | |
                                    |                 +---------------+   | | |
			            +---------------->| ts0           |<--+ | |
                                                      | 213499240729  |     | |
                                                      +---------------+     | |
                                                      | unused        |     | |
                                                      |               |     | |
                                                      +---------------+     | |
                                                              .             | |
                                                              .             | |
                                                              .             | |
                                                      +---------------+     | |
                                                      | unused        |     | |
                                                      |               |     | |
                                                      +---------------+     | |
                                                      | unused        |     | |
                                                      |               |     | |
                                                      +---------------+     | |

```

对于每个已使用的映射条目，都有一map_elt 指向一个包含与该直方图条目关联的变量当前值的 .vars 数组。所以在上面，与 pid 999 关联的时间戳113345679876，而在 pid 4444 的同一.var.idx 中的时间戳变量是 213499240729

### sched_switch 鐩存柟鍥。



上面 sched_waking 直方图所配对sched_switch 直方图如下所示。sched_switch 直方图最重要的方面在于它引用了上sched_waking 直方图中的一个变量

这个直方图图与到目前为止展示的其他图非常相似，但它增加了变量引用。你可以看到普通的 hitcount 和键字段，外加一个用sched_waking ts0 变量相同方式实现的新wakeup_lat 变量，但除此之外还有一个带有新 FL_VAR_REF（HIST_FIELD_FL_VAR_REF 的简写）标志的条目

与新的变量引用字段相关联的还有几个新hist_field 成员：var.hist_data var_ref_idx。对于一个变量引用，var.hist_data var.idx 配合使用，二者共同唯一标识某个特定直方图上的一个特定变量。var_ref_idx 只是用于缓存每个变量值的 var_ref_vals[] 数组的索引，每当某个 hist 触发器被更新时都会缓存。这些结果值随后被其他代码（例如使var_ref_idx 值来赋参trace action 代码）最终访问

下面的图描述sched_switch 的情形：

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >>
          events/sched/sched_switch/trigger

                                                                            | |
  +------------------+                                                      | |
  | hist_data        |                                                      | |
  +------------------+   +-----------------------+                          | |
    | .fields[]      |-->| val = hitcount        |                          | |
    +----------------+   +-----------------------+                          | |
    | .map           |     | .size               |                          | |
    +----------------+     +---------------------+                          | |
 +--| .var_refs[]    |     | .offset             |                          | |
 |  +----------------+     +---------------------+                          | |
 |                         | .fn()               |                          | |
 |   var_ref_vals[]        +---------------------+                          | |
 |  +-------------+        | .flags              |                          | |
 |  | $ts0        |<---+   +---------------------+                          | |
 |  +-------------+    |   | .var.idx            |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .var.hist_data      |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .var_ref_idx        |                          | |
 |  |             |    | +-----------------------+                          | |
 |  +-------------+    | | var = wakeup_lat      |                          | |
 |         .           | +-----------------------+                          | |
 |         .           |   | .size               |                          | |
 |         .           |   +---------------------+                          | |
 |  +-------------+    |   | .offset             |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .fn()               |                          | |
 |  |             |    |   +---------------------+                          | |
 |  +-------------+    |   | .flags & FL_VAR     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var.idx            |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var.hist_data      |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var_ref_idx        |                          | |
 |                     |   +---------------------+                          | |
 |                     |             .                                      | |
 |                     |             .                                      | |
 |                     |             .                                      | |
 |                     | +-----------------------+ <--- n_vals              | |
 |                     | | key = pid             |                          | |
 |                     | +-----------------------+                          | |
 |                     |   | .size               |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .offset             |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .fn()               |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .flags              |                          | |
 |                     |   +---------------------+                          | |
 |                     |   | .var.idx            |                          | |
 |                     | +-----------------------+ <--- n_fields            | |
 |                     | | unused                |                          | |
 |                     | +-----------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |   |                     |                          | |
 |                     |   +---------------------+                          | |
 |                     |                         n_keys = n_fields - n_vals | |
 |                     |                                                    | |
 |                     |						    | |
 |                     | +-----------------------+                          | |
 +---------------------->| var_ref = $ts0        |                          | |
                       | +-----------------------+                          | |
                       |   | .size               |                          | |
                       |   +---------------------+                          | |
                       |   | .offset             |                          | |
                       |   +---------------------+                          | |
                       |   | .fn()               |                          | |
                       |   +---------------------+                          | |
                       |   | .flags & FL_VAR_REF |                          | |
                       |   +---------------------+                          | |
                       |   | .var.idx            |--------------------------+ |
                       |   +---------------------+                            |
                       |   | .var.hist_data      |----------------------------+
                       |   +---------------------+
                       +---| .var_ref_idx        |
                           +---------------------+

```

```
  hist_data = struct hist_trigger_data
  hist_data.fields = struct hist_field
  fn = hist_field_fn_t
  FL_KEY = HIST_FIELD_FL_KEY
  FL_VAR = HIST_FIELD_FL_VAR
  FL_VAR_REF = HIST_FIELD_FL_VAR_REF

```

当一个直方图触发器使用了一个变量时，就会创建一个带HIST_FIELD_FL_VAR_REF 标志的新 hist_field。对于一VAR_REF 字段，其 var.idx var.hist_data 取与所引用变量相同的值，同时也包括所引用变量size、type is_signed 值。VAR_REF 字段.name 被设置为它所引用变量的名字。如果变量引用是使用显式system.event.$var_ref 记号创建的，那么hist_field system event_name 变量也会被设置

因此，为了处sched_switch 直方图的一个事件，因为我们引用了另一个直方图上的一个变量，所以需要先解析所有的变量引用。这是通过event_hist_trigger() 发起resolve_var_refs() 调用完成的。它的作用是取出表示 sched_switch 直方图的 hist_data 中的 var_refs[] 数组。对于其中的每一个，都会利用所引用变量var.hist_data 以及当前键，到那个直方图中查找对应的 tracing_map_elt。一旦找到，就用所引用变量var.idx，通过 tracing_map_read_var(elt, var.idx) 查找该变量的值，从而得到该元素对应的变量值，在上面这个例子中就是 ts0。注意，表示变量及其引用的两hist_field 拥有相同var.idx，所以这个过程是直接的

### 变量与变量引用测



这个例子sched_waking 事件上创建一个变ts0，并sched_switch 触发器中加以使用。sched_switch 触发器还

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >> events/sched/sched_switch/trigger

```

观察 sched_waking 'hist_debug' 输出，除了普通的键和hist_field 之外，在 val fields 节中我们可以看到一个带HIST_FIELD_FL_VAR 标志的字段，这表明该字段表示一个变量。注意，除了包含var.name 字段中的变量名之外，它还包含 var.idx，即指向保存该变量实际位置的 tracing_map_elt.vars[] 数组的索引。还要注意，输出显示变量位于

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 000000009536f554

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

继续sched_switch 触发器的 hist_debug 输出，除了那个未被使用的 wakeup_lat 变量之外，我们还看到一个新的、显示变量引用的节。变量引用之所以显示在一个独立的节中，是因为除了在逻辑上与变量和值相分离之外，它们实际上位于一个独立的 hist_data 数组 var_refs[] 中

在这个例子中，sched_switch 触发器引用了 sched_waking 触发器上的一个变$ts0。观察其细节，我们可以看到所引用变量var.hist_data 值与前面显示sched_waking 触发器相匹配，var.idx 值与前面显示的该变量var.idx 值相匹配。同时显示的还有该变量引用的 var_ref_idx 值，变量的值就是缓存在这里，供

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000f4ee8006

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 000000009536f554
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

## 动作与处理器（Actions and Handlers



在前面例子的基础上，我们现在要对那个 wakeup_lat 变量做点事情，即把它和另一个字段作为一个合成事件发送出去

下面onmatch() 动作基本意思是：每当我们有一sched_switch 事件，如果存在一个匹配的 sched_waking 事件（在本例中，sched_waking 直方图中存在一pid 与本 sched_switch 事件next_pid 字段相匹配），我们就取出 wakeup_latency() trace 动作中指定的变量，并用它们向跟踪流中生成一个新wakeup_latency 事件

注意，像 wakeup_latency()（它也可以等价地写成 trace(wakeup_latency,$wakeup_lat,next_pid)）这样的 trace 处理器，其实现要求传trace 处理器的参数必须是变量。在本例中，$wakeup_lat 显然是一个变量，next_pid 不是，因为它只是 sched_switch 跟踪事件中一个字段的名字。由于几乎每trace() save() 动作都会这样做，所以实现了一个特殊捷径，允许在这些情况下直接使用字段名。其工作方式是：在底层会为所指名的字段创建一个临时变量，这个变量才是实际传给 trace 处理器的东西。在代码和文档中，这类变量被称为“字段变量（field variable）”

其他跟踪事件直方图上的字段也可以被使用。在那种情况下，我们必须生成一个新的直方图以及一个命名不太恰当的 'synthetic_field'（这里的 synthetic 与合成事件毫无关系），并把那个特殊的直方图字段当作变量来使用

下面的图以上下文的方式，借助使用 onmatch() 处理器和 trace() 动作sched_switch 直方图，展示了上述新增的元素

```
  # echo 'wakeup_latency u64 lat; pid_t pid' >> synthetic_events

```

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >>
          events/sched/sched_waking/trigger

```

最后，我们sched_switch 事件上创建一hist 触发器，用来生成 wakeup_latency() trace 事件。在本例中，我们next_pid 传入 wakeup_latency 合成事件的调用，该：

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
          onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid)' >>
	  /sys/kernel/tracing/events/sched/sched_switch/trigger

```

sched_switch 事件的图与前面的例子类似，但它展示了 hist_data 新增field_vars[] 数组，并展示field_vars 与为实现字段变量而创建的变量及引用之间的关联。具体细节将在下面讨论：

```
    +------------------+
    | hist_data        |
    +------------------+   +-----------------------+
      | .fields[]      |-->| val = hitcount        |
      +----------------+   +-----------------------+
      | .map           |     | .size               |
      +----------------+     +---------------------+
  +---| .field_vars[]  |     | .offset             |
  |   +----------------+     +---------------------+
  |+--| .var_refs[]    |     | .offset             |
  ||  +----------------+     +---------------------+
  ||                         | .fn()               |
  ||   var_ref_vals[]        +---------------------+
  ||  +-------------+        | .flags              |
  ||  | $ts0        |<---+   +---------------------+
  ||  +-------------+    |   | .var.idx            |
  ||  | $next_pid   |<-+ |   +---------------------+
  ||  +-------------+  | |   | .var.hist_data      |
  ||+>| $wakeup_lat |  | |   +---------------------+
  ||| +-------------+  | |   | .var_ref_idx        |
  ||| |             |  | | +-----------------------+
  ||| +-------------+  | | | var = wakeup_lat      |
  |||        .         | | +-----------------------+
  |||        .         | |   | .size               |
  |||        .         | |   +---------------------+
  ||| +-------------+  | |   | .offset             |
  ||| |             |  | |   +---------------------+
  ||| +-------------+  | |   | .fn()               |
  ||| |             |  | |   +---------------------+
  ||| +-------------+  | |   | .flags & FL_VAR     |
  |||                  | |   +---------------------+
  |||                  | |   | .var.idx            |
  |||                  | |   +---------------------+
  |||                  | |   | .var.hist_data      |
  |||                  | |   +---------------------+
  |||                  | |   | .var_ref_idx        |
  |||                  | |   +---------------------+
  |||                  | |              .
  |||                  | |              .
  |||                  | |              .
  |||                  | |              .
  ||| +--------------+ | |              .
  +-->| field_var    | | |              .
   || +--------------+ | |              .
   ||   | var        | | |              .
   ||   +------------+ | |              .
   ||   | val        | | |              .
   || +--------------+ | |              .
   || | field_var    | | |              .
   || +--------------+ | |              .
   ||   | var        | | |              .
   ||   +------------+ | |              .
   ||   | val        | | |              .
   ||   +------------+ | |              .
   ||         .        | |              .
   ||         .        | |              .
   ||         .        | | +-----------------------+ <--- n_vals
   || +--------------+ | | | key = pid             |
   || | field_var    | | | +-----------------------+
   || +--------------+ | |   | .size               |
   ||   | var        |--+|   +---------------------+
   ||   +------------+ |||   | .offset             |
   ||   | val        |-+||   +---------------------+
   ||   +------------+ |||   | .fn()               |
   ||                  |||   +---------------------+
   ||                  |||   | .flags              |
   ||                  |||   +---------------------+
   ||                  |||   | .var.idx            |
   ||                  |||   +---------------------+ <--- n_fields
   ||                  |||
   ||                  |||                           n_keys = n_fields - n_vals
   ||                  ||| +-----------------------+
   ||                  |+->| var = next_pid        |
   ||                  | | +-----------------------+
   ||                  | |   | .size               |
   ||                  | |   +---------------------+
   ||                  | |   | .offset             |
   ||                  | |   +---------------------+
   ||                  | |   | .flags & FL_VAR     |
   ||                  | |   +---------------------+
   ||                  | |   | .var.idx            |
   ||                  | |   +---------------------+
   ||                  | |   | .var.hist_data      |
   ||                  | |   +-----------------------+
   ||                  +-->| val for next_pid      |
   ||                  | | +-----------------------+
   ||                  | |   | .size               |
   ||                  | |   +---------------------+
   ||                  | |   | .offset             |
   ||                  | |   +---------------------+
   ||                  | |   | .fn()               |
   ||                  | |   +---------------------+
   ||                  | |   | .flags              |
   ||                  | |   +---------------------+
   ||                  | |   |                     |
   ||                  | |   +---------------------+
   ||                  | |
   ||                  | |
   ||                  | | +-----------------------+
   +|------------------|-|>| var_ref = $ts0        |
    |                  | | +-----------------------+
    |                  | |   | .size               |
    |                  | |   +---------------------+
    |                  | |   | .offset             |
    |                  | |   +---------------------+
    |                  | |   | .fn()               |
    |                  | |   +---------------------+
    |                  | |   | .flags & FL_VAR_REF |
    |                  | |   +---------------------+
    |                  | +---| .var_ref_idx        |
    |                  |   +-----------------------+
    |                  |   | var_ref = $next_pid   |
    |                  |   +-----------------------+
    |                  |     | .size               |
    |                  |     +---------------------+
    |                  |     | .offset             |
    |                  |     +---------------------+
    |                  |     | .fn()               |
    |                  |     +---------------------+
    |                  |     | .flags & FL_VAR_REF |
    |                  |     +---------------------+
    |                  +-----| .var_ref_idx        |
    |                      +-----------------------+
    |                      | var_ref = $wakeup_lat |
    |                      +-----------------------+
    |                        | .size               |
    |                        +---------------------+
    |                        | .offset             |
    |                        +---------------------+
    |                        | .fn()               |
    |                        +---------------------+
    |                        | .flags & FL_VAR_REF |
    |                        +---------------------+
    +------------------------| .var_ref_idx        |
                             +---------------------+

```

如你所见，对于一个字段变量，会创建两hist_field：一个表示变量（在本例中next_pid），另一个用于像普val 字段那样从跟踪流中真正取得该字段的值。它们是独立于普通变量创建过程而创建的，并保存hist_data->field_vars[] 数组中。关于它们如何被使用，请见下文。此外，还会创建一个引hist_field，它是引用字段变量（trace() 动作中的 $next_pid 变量）所必需的

注意wakeup_lat 也是一个变量引用，引用表达common_timestamp-$ts0 的值，因此也需要创建一个表示该引用hist field 条目

当调hist_trigger_elt_update() 来获取普通的键和值字段时，它还会调用 update_field_vars()，后者会遍历为该直方图创建的每个 field_var（可hist_data->field_vars 获得），调用 val->fn() 从当前跟踪记录中获取数据，然后使用该变量var.idx 把变量设置到相应 tracing_map_elt elt->vars[var.idx] 处

一旦所有变量都已更新，就可以从 event_hist_trigger() 调用 resolve_var_refs()，此时不仅我们的 $ts0 $next_pid 引用可以被解析，$wakeup_lat 引用也可以。至此，trace() 动作只需访问var_ref_vals[] 数组中汇集的值，并生成该 trace 事件

对于save() 动作关联的字段变量，发生的是同样的过程

```
  hist_data = struct hist_trigger_data
  hist_data.fields = struct hist_field
  field_var = struct field_var
  fn = hist_field_fn_t
  FL_KEY = HIST_FIELD_FL_KEY
  FL_VAR = HIST_FIELD_FL_VAR
  FL_VAR_REF = HIST_FIELD_FL_VAR_REF

```

### trace() 动作的字段变量测



这个例子在前一个测试例子的基础上，最终用上了 wakeup_lat 变量，此外还创建了一对字段变量，然后通过 onmatch() 处理器把它们全部传给 wakeup_latency() trace 动作

```
  # echo 'wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

最后，像前面的测试例子一样，我们利用来自 sched_waking 触发器的 $ts0 引用，把唤醒延迟计算并赋wakeup_lat 变量，然后最终把它和 sched_switch 事件的一对字next_pid next_comm 一起，用来生成一wakeup_latency trace 事件。next_pid next_comm 事件字段

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,next_comm)' >> /sys/kernel/tracing/events/sched/sched_switch/trigger

```

sched_waking hist_debug 输出显示的数据与

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000d60ff61f

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

sched_switch hist_debug 输出显示了与前面测试例子相同的键和值字段——注wakeup_lat 仍在 val fields 节中，但新的字段变量并不在那里——尽管字段变量也是变量，它们被单独保存在 hist_data field_vars[] 数组中。虽然字段变量和普通变量位于不同的地方，但你可以看到这些变量在 tracing_map_elt.vars[] 中的实际位置确实像预期的那样具有递增的索引：wakeup_lat 占用var.idx = 0 的槽位，next_pid next_comm 的字段变量的值分别是 var.idx = 1 var.idx = 2。还要注意，这些值与变量引用字段节中对应那些变量的引用所显示的值相同。由于存在两个触发器，因此也就有两个 hist_data 地址，在进行匹配时也需要把这些地址考虑进来——你可以看到第一个变量引用的是前一hist 触发器（参见与该触发器关联的 hist_data 地址）上0 var.idx，而第二个变量引用的是 sched_switch hist 触发器上0 var.idx，其余所有变量引用也是如此

最后，动作跟踪变量节只显示了系统：

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,next_comm) [active]
  #

  hist_data: 0000000008f551b7

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000d60ff61f
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000008f551b7
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

    hist_data->var_refs[2]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 0000000008f551b7
      var_ref_idx (into hist_data->var_refs[]): 2
      type: pid_t
      size: 4
      is_signed: 0

    hist_data->var_refs[3]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2
      var.hist_data: 0000000008f551b7
      var_ref_idx (into hist_data->var_refs[]): 3
      type: char[16]
      size: 256
      is_signed: 0

  field variables:

    hist_data->field_vars[0]:

      field_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1

      field_vars[0].val:
      ftrace_event_field name: next_pid
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->field_vars[1]:

      field_vars[1].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2

      field_vars[1].val:
      ftrace_event_field name: next_comm
      type: char[16]
      size: 256
      is_signed: 0

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].match_data.event_system: sched
    hist_data->actions[0].match_data.event: sched_waking

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,next_comm)' >> /sys/kernel/tracing/events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo '!wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```

### action_data trace() 动作



如上所述，trace() 动作生成一个合成事件时，合成事件的所有参数要么已经是变量，要么被转换成了变量（通过字段变量），最终所有这些变量值都通过引用收集var_ref_vals[] 数组中

不过，var_ref_vals[] 数组中的值并不一定按照合成事件参数的相同顺序排列。为了解决这个问题，struct action_data 包含了另一个数var_ref_idx[]，用于将 trace 动作的参数映射到 var_ref_vals[] 的值。下面是一个：

```
  +------------------+     wakeup_latency()
  | action_data      |       event params               var_ref_vals[]
  +------------------+    +-----------------+        +-----------------+
    | .var_ref_idx[] |--->| $wakeup_lat idx |---+    |                 |
    +----------------+    +-----------------+   |    +-----------------+
    | .synth_event   |    | $next_pid idx   |---|-+  | $wakeup_lat val |
    +----------------+    +-----------------+   | |  +-----------------+
                                   .            | +->| $next_pid val   |
                                   .            |    +-----------------+
                                   .            |           .
                          +-----------------+   |           .
			  |                 |   |           .
			  +-----------------+   |    +-----------------+
                                                +--->| $wakeup_lat val |
                                                     +-----------------+

```

基本上，这在合成事件探测（probe）中最终是这样被使用的

```
  for each field i in .synth_event
    val_idx = .var_ref_idx[i]
    val = var_ref_vals[val_idx]

```

### action_data onXXX() 处理



除了 onmatch() 之外hist 触发onXXX() 动作，比onmax() onchange()，也会利用并在内部创建隐藏的变量。这些信息保存在 action_data.track_data 结构体中，并且也会像下面例子中描述的那样，显示在 hist_debug 输出中

通常，onmax() onchange() 处理器会与：

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
          onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm)' >>
          /sys/kernel/tracing/events/sched/sched_switch/trigger

```

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0: \
          onmax($wakeup_lat).snapshot()' >>
          /sys/kernel/tracing/events/sched/sched_switch/trigger

```

### save() 动作的字段变量测



在这个例子中，我们不生成合成事件，而是使用 save() 动作，在 onmax() 处理器检测到命中一个新的最大延迟时，保存字段值。和前面的例子一样，被保存的值也是字段值，但在这种情况下，它们保存在一个名save_vars[] 的独hist_data 数组中

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

不过在本例中，我们设sched_switch 触发器，以便每当命中一个新的最大延迟时，就保存一sched_switch 字段值。对onmax() 处理器和 save() 动作，都会创建变量，

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm)' >> events/sched/sched_switch/trigger

```

sched_waking hist_debug 输出显示的数据与

```
  # cat events/sched/sched_waking/hist_debug

  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000e6290f48

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

sched_switch 触发器的输出显示了与之前相同val key 值，但也显示了几个新的节

首先，动作跟踪变量节现在显示actions[].track_data 信息，描述了用于跟踪（在本例中）运行最大值的特殊跟踪变量和引用。actions[].track_data.var_ref 成员包含对被跟踪变量的引用，在本例中$wakeup_lat 变量。为了执onmax() 处理器函数，还需要一个变量，通过每当命中新最大值时就被更新来跟踪当前最大值。在本例中，我们可以看到一个自动生成的名为 '__max' 的变量已经被创建，并可见actions[].track_data.track_var 变量中

最后，在新的“save action variables”节中，我们可以看到 save() 函数4 个参数导致创建了 4 个字段变量，用于在命中最大值时保存所指名字段的值。这些变量保存在脱离hist_data 的一个独save_vars[] 数组中，因此显示在另一个：

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm) [active]
  #

  hist_data: 0000000057bcd28d

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000e6290f48
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000057bcd28d
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].track_data.var_ref:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000057bcd28d
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

    hist_data->actions[0].track_data.track_var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: __max
      var.idx (into tracing_map_elt.vars[]): 1
      type: u64
      size: 8
      is_signed: 0

  save action variables (save() params):

    hist_data->save_vars[0]:

      save_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2

      save_vars[0].val:
      ftrace_event_field name: next_comm
      type: char[16]
      size: 256
      is_signed: 0

    hist_data->save_vars[1]:

      save_vars[1].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: prev_pid
      var.idx (into tracing_map_elt.vars[]): 3

      save_vars[1].val:
      ftrace_event_field name: prev_pid
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->save_vars[2]:

      save_vars[2].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: prev_prio
      var.idx (into tracing_map_elt.vars[]): 4

      save_vars[2].val:
      ftrace_event_field name: prev_prio
      type: int
      size: 4
      is_signed: 1

    hist_data->save_vars[3]:

      save_vars[3].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: prev_comm
      var.idx (into tracing_map_elt.vars[]): 5

      save_vars[3].val:
      ftrace_event_field name: prev_comm
      type: char[16]
      size: 256
      is_signed: 0

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmax($wakeup_lat).save(next_comm,prev_pid,prev_prio,prev_comm)' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

## 几个特殊情况



尽管上面涵盖了直方图内部机制的基础，但还有几个特殊情况值得讨论，因为它们往往会带来更多的困惑。它们分别是其他直方图上的字段变量，以及别名（alias），两者都将在下面通过示例测试、使hist_debug 文件加以说明

### 其他直方图上的字段变量测



这个例子与前面的例子类似，但在本例中，sched_switch 触发器引用了另一个事件（sched_waking 事件）上的一hist 触发器字段。为了实现这一点，会为那个其他事件创建一个字段变量，但由于现有的直方图无法被使用（因为现有直方图是不可变的），所以会创建并使用一个带有匹配变量的新直方图，我们将在下面显示的 hist_debug 输出中看到这一点

首先，我们创wakeup_latency 合成事件。注意：

```
  # echo 'wakeup_latency u64 lat; pid_t pid; int prio' >> synthetic_events

```

```
  # echo 'hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

这里我们sched_switch 上设置一hist 触发器，使用命名 sched_waking 事件onmatch 处理器来发送一wakeup_latency 事件。注意，传给 wakeup_latency() 的第三个参数prio，它是一个需要为其创建字段变量的字段名。然而，sched_switch 事件上并没有任何 prio 字段，所以似乎不可能为它创建字段变量。与之匹配的 sched_waking 事件确实有一prio 字段，因此应该可以利用它来达到这个目的。问题在于，目前还不可能在现有直方图上定义一个新的变量，因此无法向现有的 sched_waking 直方图添加新prio 字段变量。不过，可以为同一个事件创建一个额外的、‘匹配’的 sched_waking 直方图（即它使用相同的键和过滤器），并在其上定义新的 prio 字段变量

```
  # echo 'hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,prio)' >> events/sched/sched_switch/trigger

```

下面sched_waking hist 触发器的 hist_debug 信息输出。注意输出中显示了两个直方图：第一个是我们在前面例子中见过的普sched_waking 直方图，第二个是我们为了提供 prio 字段变量而创建的那个特殊直方图

观察下面的第二个直方图，我们看到一个名synthetic_prio 的变量。这就是prio 字段创建的字段变量：

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000349570e4

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1


  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:synthetic_prio=prio:sort=hitcount:size=2048 [active]
  #

  hist_data: 000000006920cf38

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      ftrace_event_field name: prio
      var.name: synthetic_prio
      var.idx (into tracing_map_elt.vars[]): 0
      type: int
      size: 4
      is_signed: 1

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

观察下面sched_switch 直方图，我们可以看到sched_waking synthetic_prio 变量的引用，而观察其关联hist_data 地址，我们看到它的确与那个新直方图相关联。还要注意，其他引用分别指向一个普通变wakeup_lat，以及一个普通字段变next_pid，：

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,prio) [active]
  #

  hist_data: 00000000a73b67df

  n_vals: 2
  n_keys: 1
  n_fields: 3

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000349570e4
      var_ref_idx (into hist_data->var_refs[]): 0
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000a73b67df
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 0
      is_signed: 0

    hist_data->var_refs[2]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 00000000a73b67df
      var_ref_idx (into hist_data->var_refs[]): 2
      type: pid_t
      size: 4
      is_signed: 0

    hist_data->var_refs[3]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: synthetic_prio
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 000000006920cf38
      var_ref_idx (into hist_data->var_refs[]): 3
      type: int
      size: 4
      is_signed: 1

  field variables:

    hist_data->field_vars[0]:

      field_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_pid
      var.idx (into tracing_map_elt.vars[]): 1

      field_vars[0].val:
      ftrace_event_field name: next_pid
      type: pid_t
      size: 4
      is_signed: 1

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].match_data.event_system: sched
    hist_data->actions[0].match_data.event: sched_waking

```

```
  # echo '!hist:keys=next_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,next_pid,prio)' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo '!wakeup_latency u64 lat; pid_t pid; int prio' >> synthetic_events

```

### 别名测试



这个例子与前面的例子非常相似，但演示了别名（alias）标志

```
  # echo 'wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```

接下来，我们创建一个类似于前面例子sched_waking 触发器，

```
  # echo 'hist:keys=pid:waking_pid=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

```

对于 sched_switch 触发器，我们不直接在 wakeup_latency 合成事件的调用中使用 $waking_pid，而是创建一个名$woken_pid $waking_pid 别名，并在合成事件中使用它：

```
  # echo 'hist:keys=next_pid:woken_pid=$waking_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,$woken_pid,next_comm)' >> events/sched/sched_switch/trigger

```

观察 sched_waking hist_debug 输出，除了：

```
  # cat events/sched/sched_waking/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=pid:vals=hitcount:waking_pid=pid,ts0=common_timestamp.usecs:sort=hitcount:size=2048:clock=global [active]
  #

  hist_data: 00000000a250528c

  n_vals: 3
  n_keys: 1
  n_fields: 4

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
      ftrace_event_field name: pid
      var.name: waking_pid
      var.idx (into tracing_map_elt.vars[]): 0
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: ts0
      var.idx (into tracing_map_elt.vars[]): 1
      type: u64
      size: 8
      is_signed: 0

  key fields:

    hist_data->fields[3]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: pid
      type: pid_t
      size: 8
      is_signed: 1

```

sched_switch hist_debug 输出显示，一个名woken_pid 的变量已经被创建，并且还设置HIST_FIELD_FL_ALIAS 标志。它也设置了 HIST_FIELD_FL_VAR 标志，这就是它出现在 val field 节中的原因

尽管有这个实现细节，一个别名变量实际上更像一个变量引用；事实上，它可以被视为“引用的引用”。该实现会从被引用的变量引用复制 var_ref->fn()，在本例中是 waking_pid fn()，即 hist_field_var_ref()，并把该函数作为别名fn()。hist_field_var_ref() 这个 fn() 需要它所用变量引用的 var_ref_idx，因waking_pid var_ref_idx 也被复制到了别名中。最终结果是：当取回别名的值时，它最终所做的与原始引用会做的完全相同，即var_ref_vals[] 数组中取回相同的值。你可以在输出中验证这一点：注意别名var_ref_idx（在本例中是 woken_pid）与变量引用字段节中那个引用 waking_pid var_ref_idx 相同

此外，一旦它取得该值，由于它本身也是一个变量，它就会把该值保存进自己var.idx。所woken_pid 别名var.idx 0，当它的 fn() 被调用来更新自身时，它会用来var_ref_idx 0 的值填充该槽位。你还会注意到，在变量引用节中有一woken_pid var_ref。那是对 woken_pid 别名变量的引用，你可以看到它从与 woken_pid 别名相同var.idx（即 0）取回值，进而再把它自己保存在其 var_ref_idx 槽位 3 中，而这个位置上的值最终就是被赋给

```
  # cat events/sched/sched_switch/hist_debug

  # event histogram
  #
  # trigger info: hist:keys=next_pid:vals=hitcount:woken_pid=$waking_pid,wakeup_lat=common_timestamp.usecs-$ts0:sort=hitcount:size=2048:clock=global:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,$woken_pid,next_comm) [active]
  #

  hist_data: 0000000055d65ed0

  n_vals: 3
  n_keys: 1
  n_fields: 4

  val fields:

    hist_data->fields[0]:
      flags:
        VAL: HIST_FIELD_FL_HITCOUNT
      type: u64
      size: 8
      is_signed: 0

    hist_data->fields[1]:
      flags:
        HIST_FIELD_FL_VAR
        HIST_FIELD_FL_ALIAS
      var.name: woken_pid
      var.idx (into tracing_map_elt.vars[]): 0
      var_ref_idx (into hist_data->var_refs[]): 0
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->fields[2]:
      flags:
        HIST_FIELD_FL_VAR
      var.name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 1
      type: u64
      size: 0
      is_signed: 0

  key fields:

    hist_data->fields[3]:
      flags:
        HIST_FIELD_FL_KEY
      ftrace_event_field name: next_pid
      type: pid_t
      size: 8
      is_signed: 1

  variable reference fields:

    hist_data->var_refs[0]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: waking_pid
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 00000000a250528c
      var_ref_idx (into hist_data->var_refs[]): 0
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->var_refs[1]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: ts0
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 00000000a250528c
      var_ref_idx (into hist_data->var_refs[]): 1
      type: u64
      size: 8
      is_signed: 0

    hist_data->var_refs[2]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: wakeup_lat
      var.idx (into tracing_map_elt.vars[]): 1
      var.hist_data: 0000000055d65ed0
      var_ref_idx (into hist_data->var_refs[]): 2
      type: u64
      size: 0
      is_signed: 0

    hist_data->var_refs[3]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: woken_pid
      var.idx (into tracing_map_elt.vars[]): 0
      var.hist_data: 0000000055d65ed0
      var_ref_idx (into hist_data->var_refs[]): 3
      type: pid_t
      size: 4
      is_signed: 1

    hist_data->var_refs[4]:
      flags:
        HIST_FIELD_FL_VAR_REF
      name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2
      var.hist_data: 0000000055d65ed0
      var_ref_idx (into hist_data->var_refs[]): 4
      type: char[16]
      size: 256
      is_signed: 0

  field variables:

    hist_data->field_vars[0]:

      field_vars[0].var:
      flags:
        HIST_FIELD_FL_VAR
      var.name: next_comm
      var.idx (into tracing_map_elt.vars[]): 2

      field_vars[0].val:
      ftrace_event_field name: next_comm
      type: char[16]
      size: 256
      is_signed: 0

  action tracking variables (for onmax()/onchange()/onmatch()):

    hist_data->actions[0].match_data.event_system: sched
    hist_data->actions[0].match_data.event: sched_waking

```

```
  # echo '!hist:keys=next_pid:woken_pid=$waking_pid:wakeup_lat=common_timestamp.usecs-$ts0:onmatch(sched.sched_waking).wakeup_latency($wakeup_lat,$woken_pid,next_comm)' >> events/sched/sched_switch/trigger

  # echo '!hist:keys=pid:ts0=common_timestamp.usecs' >> events/sched/sched_waking/trigger

  # echo '!wakeup_latency u64 lat; pid_t pid; char comm[16]' >> synthetic_events

```
