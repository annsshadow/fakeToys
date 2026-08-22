## dm-flakey


target linear target 相同，只是它会周期性地表现出不可靠的行为。它已被证明在模故障设备用于测试时很有用
从表被加载的时刻起，设备可用 <up interval> 秒，然后表现出不可靠行为 <down interval> 秒，
然后这个循环重复
另外，也考虑将其dm-delay target 结合使用，后者可以延迟读写和/或将其发送到不同底层设备
### 表参

```

  <dev path> <offset> <up interval> <down interval> \
    [<num_features> [<feature arguments>]]

```
必选参数：

    <dev path>        底层块设备的完整路径名，或“major:minor设备号    <offset>        设备内的起始扇区    <up interval>        设备可用的秒数    <down interval>        设备返回错误的秒数
可选特性参数：

  如果不存在任何特性参数，在不可靠期间，所I/O 都返回错误
  error_reads	所有读 I/O 都以报错失败	I/O 被正确处理
  drop_writes	所有写 I/O 被静默忽略	I/O 被正确处理
  error_writes	所有写 I/O 都以报错失败	I/O 被正确处理
  corrupt_bio_byte <Nth_byte> <direction> <value> <flags>	<down interval> 期间，将每个匹配 bio 的数据的<Nth_byte> 字节替换<value>
    <Nth_byte>	要替换的字节偏移	计数1 开始，以替换第一个字节    <direction>	'r' 表示损坏读，'w' 表示损坏写	'w' drop_writes 不兼容    <value>	要写入的值（0-255）    <flags>	仅当 bio->bi_opf 设置了所有选定的标志时才执行替换
  random_read_corrupt <probability>
	<down interval> 期间，将bio 中的随机字节替换为随机值。probability 是一个介	0 1000000000 之间的整数，表示 0% 100% 的损坏概率
  random_write_corrupt <probability>
	<down interval> 期间，将bio 中的随机字节替换为随机值。probability 是一个介	0 1000000000 之间的整数，表示 0% 100% 的损坏概率
示例
```

  corrupt_bio_byte 32 r 1 0

```
```

  corrupt_bio_byte 224 w 0 32

```
