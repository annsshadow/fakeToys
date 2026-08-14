## dm-linear


Device-Mapper 鐨?"linear" 鐩爣灏?Device-Mapper 璁惧鐨勪竴涓嚎鎬у尯闂存槧灏勫埌
鍙︿竴涓澶囩殑绾挎€у尯闂翠笂銆傝繖鏄€昏緫鍗风鐞嗗櫒鐨勫熀鏈瀯寤哄潡銆?

鍙傛暟锛?dev path> <offset>
    <dev path>锛?
	搴曞眰鍧楄澶囩殑瀹屾暣璺緞鍚嶏紝鎴?
        "major:minor" 璁惧鍙枫€?
    <offset>锛?
	璁惧鍐呯殑璧峰鎵囧尯銆?


## 绀轰緥鑴氭湰


```
  #!/bin/sh
  # Create an identity mapping for a device
  echo "0 `blockdev --getsz $1` linear $1 0" | dmsetup create identity

```
```
  #!/bin/sh
  # Join 2 devices together
  size1=`blockdev --getsz $1`
  size2=`blockdev --getsz $2`
  echo "0 $size1 linear $1 0
  $size1 $size2 linear $2 0" | dmsetup create joined

```
```
  #!/usr/bin/perl -w
  # Split a device into 4M chunks and then join them together in reverse order.

  my $name = "reverse";
  my $extent_size = 4 * 1024 * 2;
  my $dev = $ARGV[0];
  my $table = "";
  my $count = 0;

  if (!defined($dev)) {
          die("Please specify a device.\n");
  }

  my $dev_size = `blockdev --getsz $dev`;
  my $extents = int($dev_size / $extent_size) -
                (($dev_size % $extent_size) ? 1 : 0);

  while ($extents > 0) {
          my $this_start = $count * $extent_size;
          $extents--;
          $count++;
          my $this_offset = $extents * $extent_size;

          $table .= "$this_start $extent_size linear $dev $this_offset\n";
  }

  `echo \"$table\" | dmsetup create $name`;

```
