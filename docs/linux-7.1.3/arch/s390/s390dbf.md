## S390 调试特性（S390 Debug Feature

files:
      - arch/s390/kernel/debug.c
      - arch/s390/include/asm/debug.h

### 描述（Description）：


本特性的目标是提供一个内核调试日API，其中的日志记录可以高效地存储在内存中，每个组件（例如设备驱动）都可以拥有各自独立的调试日志这样做的一个目的是在生产系统崩溃后检查调试日志，以分析崩溃的原因
如果系统仍在运行，但只有某个使用dbf 的子组件失败，则可以通过 Linux debugfs 文件系统在运行中的系统上查看调试日志
该调试特性对于内核和驱动的开发也可能非常有用
### 设计（Design）：


内核组件（例如设备驱动）可以通过函数调用 `debug_register()` 在调试特性中注册自己该函数为调用者初始化一个调试日志。每个调试日志存在若干个调试区域（debug area），其中同一时刻恰好有一个处于活动状态。每个调试区域由内存中连续的页组成。在调试区域中存储着调试条目（log records），它们event 调用exception 调用写入
event 调用将指定的调试条目写入活动调试区域，并更新该活动区域的日志指针。如果到达活动调试区域的末尾，则进行回绕（wrap around，环形缓冲区），下一个调试条目将被写入活动调试区域的开始处
exception 调用将指定的调试条目写入日志，并切换到下一个调试区域。这样做是为了确保描述异常来源的那些记录在当前的区域发生回绕时不会被覆盖
调试区域本身也以环形缓冲区的顺序排列。当在最后一个调试区域中抛出异常时，后续的调试条目会再次写入最开始的那个区域
event 调用exception 调用有四种版本：一种用于记录原始数据，一种用于文本，一种用于数字（unsigned int long），还有一种用于类 sprintf 的格式化字符串
每个调试条目包含以下数据
- 时间戳（Timestamp- 调用任务Cpu 编号
- 调试条目的级别（0...6- 返回地址（Return Address，指向调用者）
- 标志，指示该条目是否为异
调试日志可以在运行中的系统上通过 debugfs 文件系统中的条目来检查。在顶层目录 "`s390dbf`" 下，为每个已注册的组件都有一个以其对应组件命名的目录。debugfs 通常应挂载到 `/sys/kernel/debug`，因此调试特性可以在 `/sys/kernel/debug/s390dbf` 下被访问
目录的内容是一些文件，它们表示对调试日志的不同视图（view）。每个组件可以通过使用函数 `debug_register_view()` 注册来决定使用哪些视图。提供了用于 hex/ascii sprintf 数据的预定义视图也可以定义其他视图。只需读取对应debugfs 文件即可检查某个视图的内容
所有调试日志都有一个当前调试级别（范围0 6）默认级别3。Event Exception 函数有一:c`level` 参数。只有级别低于或等于当前级别的调试条目才会被写入日志。这意味着，在写入事件时，高优先级的日志条目应当具有较低的级别值，而低优先级的条目应当具有较高的级别值可以通过 debugfs 文件系统，向为每个调试日志提供的 `level` debugfs 文件写入一个数字字符串 "x" 来改变当前调试级别。通过`level` debugfs 文件上写"-" 可以完全关闭调试
```
	> echo "-" > /sys/kernel/debug/s390dbf/dasd/level

```
也可以为每一个调试日志全局地停用调试特性。您可以通过 `/proc/sys/s390dbf` 中的 2 sysctl 参数来改变其行为
目前2 种可能的触发器会全局停止调试特性。第一种可能性是使用 `debug_active` sysctl。如果设置为 1，调试特性正在运行；如果 `debug_active` 设置0，调试特性被关闭
第二种停止调试特性的触发器是内核 oops。这样可以防止调试特性覆oops 之前发生的调试信息。在 oops 之后，您可以通过`/proc/sys/s390dbf/debug_active` 写入 1 来重新激活调试特性。不过，不建议在生产环境中使用发生过 oops 的内核
如果您想禁止停用调试特性，可以使用 `debug_stoppable` sysctl。如果将 `debug_stoppable` 设置0，调试特性将无法被停止。如果调试特性已经停止，它将保持停用状态
### 内核接口（Kernel Interfaces）：



### 预定义视图（Predefined views）：



  extern struct debug_view debug_hex_ascii_view;

  extern struct debug_view debug_sprintf_view;

### 示例（Examples


  /*
   - hex_ascii-view Example
   */

  #include <linux/init.h>
  #include <asm/debug.h>

  static debug_info_t *debug_info;

  static int init(void)
  {
      /** register 4 debug areas with one page each and 4 byte data field **/

      debug_info = debug_register("test", 1, 4, 4 );
      debug_register_view(debug_info, &debug_hex_ascii_view);

      debug_text_event(debug_info, 4 , "one ");
      debug_int_exception(debug_info, 4, 4711);
      debug_event(debug_info, 3, &debug_info, 4);

      return 0;
  }

  static void cleanup(void)
  {
      debug_unregister(debug_info);
  }

  module_init(init);
  module_exit(cleanup);


  /*
   - sprintf-view Example
   */

  #include <linux/init.h>
  #include <asm/debug.h>

  static debug_info_t *debug_info;

  static int init(void)
  {
      /** register 4 debug areas with one page each and data field for **/
      /** format string pointer + 2 varargs (= 3 ** sizeof(long))       */

      debug_info = debug_register("test", 1, 4, sizeof(long) * 3);
      debug_register_view(debug_info, &debug_sprintf_view);

      debug_sprintf_event(debug_info, 2 , "first event in %s:%i\n",__FILE__,__LINE__);
      debug_sprintf_exception(debug_info, 1, "pointer to debug info: %p\n",&debug_info);

      return 0;
  }

  static void cleanup(void)
  {
      debug_unregister(debug_info);
  }

  module_init(init);
  module_exit(cleanup);

### Debugfs 接口（Debugfs Interface

可以通过读取对应debugfs 文件来检查调试日志的视图
```
  > ls /sys/kernel/debug/s390dbf/dasd
  flush  hex_ascii  level pages
  > cat /sys/kernel/debug/s390dbf/dasd/hex_ascii | sort -k2,2 -s
  00 00974733272:680099 2 - 02 0006ad7e  07 ea 4a 90 | ....
  00 00974733272:682210 2 - 02 0006ade6  46 52 45 45 | FREE
  00 00974733272:682213 2 - 02 0006adf6  07 ea 4a 90 | ....
  00 00974733272:682281 1 * 02 0006ab08  41 4c 4c 43 | EXCP
  01 00974733272:682284 2 - 02 0006ab16  45 43 4b 44 | ECKD
  01 00974733272:682287 2 - 02 0006ab28  00 00 00 04 | ....
  01 00974733272:682289 2 - 02 0006ab3e  00 00 00 20 | ...
  01 00974733272:682297 2 - 02 0006ad7e  07 ea 4a 90 | ....
  01 00974733272:684384 2 - 00 0006ade6  46 52 45 45 | FREE
  01 00974733272:684388 2 - 00 0006adf6  07 ea 4a 90 | ....

```
关于上述输出的解释，请参见关于预定义视图的小节！

### 改变调试级别（Changing the debug level


```
  > cat /sys/kernel/debug/s390dbf/dasd/level
  3
  > echo "5" > /sys/kernel/debug/s390dbf/dasd/level
  > cat /sys/kernel/debug/s390dbf/dasd/level
  5

```
### 刷新调试区域（Flushing debug areas

可以通过debugfs 文件 "flush" 写入所需区域的编号（0...n）来刷新该调试区域。使"-" 时，所有调试区域都会被刷新
示例
```
     > echo "0" > /sys/kernel/debug/s390dbf/dasd/flush

```
```
     > echo "-" > /sys/kernel/debug/s390dbf/dasd/flush

```
### 改变调试区域的大小（Changing the size of debug areas

要调整调试区域的大小，请将所需的页计数写入 "pages" 文件如果现有数据能放得下则会被保留；否则，最旧的条目会被丢弃
示例
```
  > echo "4" > /sys/kernel/debug/s390dbf/dasd/pages

```
### 停止调试特性（Stopping the debug feature

示例
```
     > cat /proc/sys/s390dbf/debug_stoppable

```
```
     > echo 0 > /proc/sys/s390dbf/debug_active

```
### crash 接口（crash Interface

v5.1.0 起，`crash` 工具内置了一个命`s390dbf`，用于显示所有调试日志或将它们导出到文件系统借助该工具，可以在运行中的系统上以及系统崩溃后的内存转储中检查调试日志
### 调查原始内存（Investigating raw memory

在运行中的系统上以及系统崩溃后，调查调试日志的最后一种可能性是查看 VM 或服务元素（Service Element）下的原始内存可以通过 System map 中的 `debug_area_first` 符号找到调试日志的锚点。然后必须顺着 debug.h 中定义的数据结构的正确指针，在内存中找到调试区域通常，使用调试特性的模块也会有一个指向调试日志的全局变量。顺着该指针也可以找到内存中的调试日志
对于这种方法，建议在 `debug_register()` 中使用长度为 '16 * x + 4' 字节（x = 0..n）的数据字段，以便查看格式良好的调试条目

### 预定义视图（Predefined Views


有两种预定义视图：hex_ascii sprintfhex_ascii 视图以十六进制和 ascii 形式显示数据字段（例`45 43 4b 44 | ECKD`）
sprintf 视图以与 sprintf 函数相同的方式格式化调试条目。sprintf event/exception 函数向调试条目写入一个指向格式字符串的指针（大小 = sizeof(long)），并为每个可变参数写入一long 值。因此，例如对于一个带一个格式字符串加两个可变参数的调试条目，需要在 debug_register() 函数中分配一(3 * sizeof(long)) 字节的数据区域
重要（IMPORTANT）：
  sprintf event 函数中使"%s" 是危险的。只有在传入字符串的内存只要调试特性存在就一直可用的情况下，您才能在 sprintf event 函数中使"%s"。其背后的原因是，出于性能考虑，调试特性中只存储了指向该字符串的指针。如果您记录了一个之后被释放的字符串，在检查调试特性时会得到一OOPS，因为那时调试特性会访问已经被释放的内存
注意（NOTE）：
  如果使用 sprintf 视图，请不要使用sprintf-event -exception 函数之外的其event/exception 函数
hex_ascii sprintf 视图的格式如下：

- 区域编号（Number of area- 时间戳（格式为自 1970 1 1 00:00:00 协调世界时（UTC）起的秒和微秒）
- 调试条目的级别（level- 异常标志（Exception flag = 异常- 调用任务Cpu 编号
- 返回地址（Return Address，指向调用者）
- 数据字段（data field
hex_ascii 视图的典型行如下所示（第一```
  area  time           level exception cpu caller    data (hex + ascii)
  --------------------------------------------------------------------------
  00    00964419409:440690 1 -         00  88023fe


```
### 定义视图（Defining views


视图通过 'debug_view' 结构来指定。其中定义了一些用于读debugfs 文件的回调函数：


  struct debug_view {
	char name[DEBUG_MAX_PROCF_LEN];
	debug_prolog_proc_t* prolog_proc;
	debug_header_proc_t* header_proc;
	debug_format_proc_t* format_proc;
	debug_input_proc_t*  input_proc;
	void*                private_data;
  };

其中

  typedef int (debug_header_proc_t) (debug_info_t* id,
				     struct debug_view* view,
				     int area,
				     debug_entry_t* entry,
				     char* out_buf);

  typedef int (debug_format_proc_t) (debug_info_t* id,
				     struct debug_view** view, char** out_buf,
				     const char* in_buf);
  typedef int (debug_prolog_proc_t) (debug_info_t* id,
				     struct debug_view* view,
				     char* out_buf);
  typedef int (debug_input_proc_t) (debug_info_t* id,
				    struct debug_view* view,
				    struct file** file, const char** user_buf,
				    size_t in_buf_size, loff_t* offset);


"private_data" 成员可用作指向视图特定数据的指针调试特性本身并不会使用它
```
  "prolog_proc output"

  "header_proc output 1"  "format_proc output 1"
  "header_proc output 2"  "format_proc output 2"
  "header_proc output 3"  "format_proc output 3"
  ...

```
当从 debugfs 读取一个视图时，调试特性调用一'prolog_proc' 来写prolog然后为每个已存在的调试条目调'header_proc' 'format_proc'
input_proc 可用于在写入该视图时实现某些功能（例如像 `echo "0" > /sys/kernel/debug/s390dbf/dasd/level` 那样）
对于 header_proc，可以使用在 debug.h 中定义的默认函数 `debug_dflt_header_fn()`，它会产生与预定义视图相同的头部输出```
  00 00964419409:440761 2 - 00 88023ec

```
要了解如何使用这些回调函数，请查看默认视图的实现
示例

  #include <asm/debug.h>

  #define UNKNOWNSTR "data: %08x"

  const char* messages[] =
  {"This error...........\n",
   "That error...........\n",
   "Problem..............\n",
   "Something went wrong.\n",
   "Everything ok........\n",
   NULL
  };

  static int debug_test_format_fn(
     debug_info_t **id, struct debug_view **view,
     char **out_buf, const char **in_buf
  )
  {
    int i, rc = 0;

    if (id->buf_size >= 4) {
       int msg_nr = **((int**)in_buf);
       if (msg_nr < sizeof(messages) / sizeof(char*) - 1)
	  rc += sprintf(out_buf, "%s", messages[msg_nr]);
       else
	  rc += sprintf(out_buf, UNKNOWNSTR, msg_nr);
    }
    return rc;
  }

  struct debug_view debug_test_view = {
    "myview",                 /** name of view **/
    NULL,                     /** no prolog **/
    &debug_dflt_header_fn,    /** default header for each entry **/
    &debug_test_format_fn,    /** our own format function **/
    NULL,                     /** no input function **/
    NULL                      /** no private data **/
  };

## 测试（test）：



  debug_info_t *debug_info;
  int i;
  ...
  debug_info = debug_register("test", 0, 4, 4);
  debug_register_view(debug_info, &debug_test_view);
  for (i = 0; i < 10; i ++)
    debug_int_event(debug_info, 1, i);

```
  > cat /sys/kernel/debug/s390dbf/test/myview
  00 00964419734:611402 1 - 00 88042ca   This error...........
  00 00964419734:611405 1 - 00 88042ca   That error...........
  00 00964419734:611408 1 - 00 88042ca   Problem..............
  00 00964419734:611411 1 - 00 88042ca   Something went wrong.
  00 00964419734:611414 1 - 00 88042ca   Everything ok........
  00 00964419734:611417 1 - 00 88042ca   data: 00000005
  00 00964419734:611419 1 - 00 88042ca   data: 00000006
  00 00964419734:611422 1 - 00 88042ca   data: 00000007
  00 00964419734:611425 1 - 00 88042ca   data: 00000008
  00 00964419734:611428 1 - 00 88042ca   data: 00000009

```
