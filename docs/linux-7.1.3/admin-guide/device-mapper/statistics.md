## DM 统计


Device Mapper 支持在 DM 设备用户定义的区域上收集 I/O 统计。如果没有定义任何区域，
就不会收集任何统计，因此不会带来任何性能影响。目前仅支持基于 bio 的 DM 设备。

每个用户定义的区域指定一个起始扇区、长度和步长。将为指定范围内每个步长大小的
区域分别收集统计。

区域内每个步长大小区域的 I/O 统计计数器格式与 `/sys/block/*/stat` 或
`/proc/diskstats` 相同（参见 Documentation/admin-guide/iostats.rst）。但还提供了
两个额外的计数器（12 和 13）：读取和写入所花费的总时间。当使用了 histogram 参数时，
会报告第 14 个参数，它表示延迟的直方图。所有这些计数器都可以通过向相应的 DM 设备
发送 @stats_print 消息（经由 dmsetup）来访问。

报告的时间以毫秒为单位，粒度取决于内核时钟滴答。当使用 precise_timestamps 选项时，
报告的时间以纳秒为单位。

每个区域都有一个对应的唯一标识符，我们称之为 region_id，它在区域创建时分配。
在查询该区域的统计、删除该区域等时，必须提供 region_id。唯一的 region_id 使得多个
用户空间程序能够请求并处理同一个 DM 设备的统计，而不会互相踩到对方的数据。

DM 统计的创建将通过 kmalloc 分配内存，或回退到使用 vmalloc 空间。DM 统计最多可能
分配系统总内存的 1/4。管理员可以通过读取以下内容查看使用了多少内存：

	/sys/module/dm_mod/parameters/stats_current_allocated_bytes

## 消息


    @stats_create <range> <step> [<number_of_optional_arguments> <optional_arguments>...] [<program_id> [<aux_data>]]
	创建一个新区域并返回 region_id。

	<range>
	  "-"
		整个设备
	  "<start_sector>+<length>"
		一段长度为 <length> 的 512 字节扇区，
		从 <start_sector> 开始。

	<step>
	  "<area_size>"
		该范围被细分为多个区域，每个区域包含
		<area_size> 个扇区。
	  "/<number_of_areas>"
		该范围被细分为指定数量的
		区域。

	<number_of_optional_arguments>
	  可选参数的数量

	<optional_arguments>
	  支持以下可选参数：

	  precise_timestamps
		使用具有纳秒分辨率的精确计时器，
		而非 "jiffies" 变量。使用此参数时，
		结果时间以纳秒而非毫秒为单位。精确时间戳
		比基于 jiffies 的时间戳获取起来稍慢一些。
	  histogram:n1,n2,n3,n4,...
		收集延迟的直方图。数字
		n1、n2 等是代表直方图边界的时间。如果未使用
		precise_timestamps，时间以毫秒为单位，否则以
		纳秒为单位。对于每个范围，内核将报告在该范围内
		完成的请求数量。例如，如果我们使用
		"histogram:10,20,30"，内核将报告四个数字
		a:b:c:d。a 是耗时 0-10 毫秒完成的请求数，b 是
		耗时 10-20 毫秒完成的请求数，c 是耗时 20-30 毫秒
		完成的请求数，d 是耗时超过 30 毫秒完成的请求数。

	<program_id>
	  一个可选参数。一个唯一标识该范围用户空间拥有者的名称。
	  这将范围分组在一起，以便用户空间程序能够识别它们
	  创建的范围，并忽略由他人创建的范围。内核在
	  @stats_list 消息的输出中将该字符串返回，但不将其用于
	  任何其它用途。如果我们省略可选参数的数量，program id 不能
	  是一个数字，否则它会被解释为可选参数的数量。

	<aux_data>
	  一个可选参数。一个提供辅助数据的词，对创建该范围的
	  客户端程序有用。内核在 @stats_list 消息的输出中将该字符串
	  返回，但不使用该值做任何事情。

    @stats_delete <region_id>
	删除具有指定 id 的区域。

	<region_id>
	  从 @stats_create 返回的 region_id

    @stats_clear <region_id>
	清除除进行中 I/O 计数器之外的所有计数器。

	<region_id>
	  从 @stats_create 返回的 region_id

    @stats_list [<program_id>]
	列出所有用 @stats_create 注册的区域。

	<program_id>
	  一个可选参数。
	  如果指定了该参数，只返回匹配的区域。
	  如果未指定，则返回所有区域。

	输出格式：
	  <region_id>: <start_sector>+<length> <step> <program_id> <aux_data>
	        precise_timestamps histogram:n1,n2,n3,...

	字符串 "precise_timestamps" 和 "histogram" 仅在创建区域时
	指定了它们的情况下才会被打印。

    @stats_print <region_id> [<starting_line> <number_of_lines>]
	打印一个区域中每个步长大小区域的计数器。

	<region_id>
	  从 @stats_create 返回的 region_id

	<starting_line>
	  输出中起始行的索引。
	  如果省略，则返回所有行。

	<number_of_lines>
	  输出中要包含的行数。
	  如果省略，则返回所有行。

	区域中每个步长大小区域的输出格式：

	  <start_sector>+<length>
		counters

	  前 11 个计数器与 `/sys/block/*/stat 或 /proc/diskstats`
	  含义相同。

	  详情请参阅 Documentation/admin-guide/iostats.rst。

   1. 已完成的读取次数
   2. 已合并的读取次数
   3. 读取的扇区数
   4. 读取所花费的毫秒数
   5. 已完成的写入次数
   6. 已合并的写入次数
   7. 写入的扇区数
   8. 写入所花费的毫秒数
   9. 当前进行中的 I/O 数量
   10. 执行 I/O 所花费的毫秒数
   11. 执行 I/O 所花费的加权毫秒数

	  额外的计数器：

   12. 读取所花费的总时间（毫秒）
   13. 写入所花费的总时间（毫秒）

    @stats_print_clear <region_id> [<starting_line> <number_of_lines>]
	原子地打印然后清除除进行中 I/O 计数器之外的所有计数器。当消费
	统计的客户端不想丢失任何统计（那些在打印和清除之间被更新的）
	时很有用。

	<region_id>
	  从 @stats_create 返回的 region_id

	<starting_line>
	  输出中起始行的索引。
	  如果省略，则打印并清除所有行。

	<number_of_lines>
	  要处理的行数。
	  如果省略，则打印并清除所有行。

    @stats_set_aux <region_id> <aux_data>
	为指定区域存储辅助数据 aux_data。

	<region_id>
	  从 @stats_create 返回的 region_id

	<aux_data>
	  标识对创建该范围的客户端程序有用的数据的字符串。内核在
	  @stats_list 消息的输出中将该字符串返回，但不将其用于任何
	  用途。

## 示例


将 DM 设备 'vol' 细分为 100 块，并开始收集
```

  dmsetup message vol 0 @stats_create - /100

```
将辅助数据字符串设为 "foo bar baz"（每个
```

  dmsetup message vol 0 @stats_set_aux 0 foo\\ bar\\ baz

```
```

  dmsetup message vol 0 @stats_list

```
```

  dmsetup message vol 0 @stats_print 0

```
```

  dmsetup message vol 0 @stats_delete 0

```
