
## dm-stripe


Device-Mapper 鐨勨€渟triped鈥濈洰鏍囩敤浜庤法涓€涓垨澶氫釜搴曞眰璁惧鍒涘缓鏉″甫鍖栵紙鍗?RAID-0锛夎澶囥€傛暟鎹互鈥滃潡锛坈hunk锛夆€濅负鍗曚綅鍐欏叆锛岃繛缁殑鍧楀湪搴曞眰璁惧闂磋疆杞€傝繖鍙€氳繃骞惰鍒╃敤澶氫釜鐗╃悊璁惧鏉ユ綔鍦ㄥ湴鎻愬崌 I/O 鍚炲悙閲忋€?
鍙傛暟锛?num devs> <chunk size> [<dev path> <offset>]+
    <num devs>:
	搴曞眰璁惧鏁伴噺銆?    <chunk size>:
	姣忎釜鏁版嵁鍧楃殑澶у皬銆傚繀椤昏嚦灏戜笌绯荤粺鐨?PAGE_SIZE 涓€鏍峰ぇ銆?    <dev path>:
	搴曞眰鍧楄澶囩殑瀹屾暣璺緞鍚嶏紝鎴栤€渕ajor:minor鈥濊澶囧彿銆?    <offset>:
	璁惧鍐呯殑璧峰鎵囧尯銆?
鍙互鎸囧畾涓€涓垨澶氫釜搴曞眰璁惧銆傛潯甯﹀寲璁惧鐨勫ぇ灏忓繀椤绘槸鍧楀ぇ灏忎箻浠ュ簳灞傝澶囨暟閲忕殑鏁存暟鍊嶃€?

## 绀轰緥鑴氭湰


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
