
## dm-stripe


Device-Mapper 的“striped”目标用于跨一个或多个底层设备创建条带化（RAID-0）设备。数据以“块（chunk）”为单位写入，连续的块在底层设备间轮转。这可通过并行利用多个物理设备来潜在地提升 I/O 吞吐量
参数num devs> <chunk size> [<dev path> <offset>]+
    <num devs>:
	底层设备数量    <chunk size>:
	每个数据块的大小。必须至少与系统PAGE_SIZE 一样大    <dev path>:
	底层块设备的完整路径名，或“major:minor”设备号    <offset>:
	设备内的起始扇区
可以指定一个或多个底层设备。条带化设备的大小必须是块大小乘以底层设备数量的整数倍

## 示例脚本


```

  #!/usr/bin/perl -w
  # Create a striped device across any number of underlying devices. The device
  # will be called "stripe_dev" and have a chunk-size of 128k.

  my $chunk_size = 128 * 2;
  my $dev_name = "stripe_dev";
  my $num_devs = @ARGV;
  my @devs = @ARGV;
  my ($min_dev_size, $stripe_dev_size, $i);

  if (!$num_devs) {
          die("Specify at least one device\n");
  }

  $min_dev_size = `blockdev --getsz $devs[0]`;
  for ($i = 1; $i < $num_devs; $i++) {
          my $this_size = `blockdev --getsz $devs[$i]`;
          $min_dev_size = ($min_dev_size < $this_size) ?
                          $min_dev_size : $this_size;
  }

  $stripe_dev_size = $min_dev_size * $num_devs;
  $stripe_dev_size -= $stripe_dev_size % ($chunk_size * $num_devs);

  $table = "0 $stripe_dev_size striped $num_devs $chunk_size";
  for ($i = 0; $i < $num_devs; $i++) {
          $table .= " $devs[$i] 0";
  }

  `echo $table | dmsetup create $dev_name`;

```
