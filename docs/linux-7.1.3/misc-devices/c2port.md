
## C2 绔彛鏀寔


(C) Copyright 2007 Rodolfo Giometti <giometti@enneenne.com>

鏈▼搴忔槸鑷敱杞欢锛涗綘鍙互鍦ㄨ嚜鐢辫蒋浠跺熀閲戜細鍙戝竷鐨?GNU 閫氱敤鍏叡璁稿彲璇佹潯娆句笅閲嶆柊
鍒嗗彂鍜?鎴栦慨鏀瑰畠锛涙棤璁烘槸璁稿彲璇佺殑绗?2 鐗堬紝杩樻槸锛堢敱浣犻€夋嫨锛変换浣曟洿鏅氱殑鐗堟湰銆?
鏈▼搴忕殑鍒嗗彂甯屾湜瀹冩湁鐢紝浣嗘病鏈変换浣曟媴淇濓紱鐢氳嚦涓嶉殣鍚换浣曢€傞攢鎬ф垨鐗瑰畾鐢ㄩ€旈€傜敤鎬?鐨勬媴淇濄€傛洿澶氱粏鑺傝鍙傞槄 GNU 閫氱敤鍏叡璁稿彲璇併€?
### 姒傝堪


鏈┍鍔ㄥ疄鐜颁簡 Linux 瀵?Silicon Labs锛圫ilabs锛塁2 鎺ュ彛鐨勬敮鎸侊紝璇ユ帴鍙ｇ敤浜庡井鎺у埗鍣ㄧ殑
鍦ㄧ郴缁熺紪绋嬶紙in-system programming锛夈€?
閫氳繃浣跨敤鏈┍鍔紝浣犲彲浠ュ湪娌℃湁 EC2 鎴?EC3 璋冭瘯閫傞厤鍣ㄧ殑鎯呭喌涓嬪鍦ㄧ郴缁熼棯瀛樿繘琛岄噸鏂?缂栫▼銆傝鏂规鍦ㄩ偅浜涘井鎺у埗鍣ㄩ€氳繃鐗规畩 GPIO 寮曡剼杩炴帴鐨勭郴缁熶腑涔熷緢鏈夌敤銆?
### 鍙傝€冭祫鏂?

C2 鎺ュ彛鐨勪富瑕佸弬鑰冭祫鏂欎綅浜?(https://www.silabs.com) Silicon Laboratories 缃戠珯锛?鍙傝锛?
- AN127: FLASH Programming via the C2 Interface锛屽湴鍧€锛?  https://www.silabs.com/Support Documents/TechnicalDocs/an127.pdf

- C2 Specification锛屽湴鍧€锛?  https://www.silabs.com/pages/DownloadDoc.aspx?FILEURL=Support%20Documents/TechnicalDocs/an127.pdf&src=SearchResults

瀹冨疄鐜颁簡涓€涓弻绾夸覆琛岄€氫俊鍗忚锛坆it banging锛夛紝鏃ㄥ湪涓轰綆寮曡剼鏁扮殑 Silicon Labs 璁惧
瀹炵幇鍦ㄧ郴缁熺紪绋嬨€佽皟璇曞拰杈圭晫鎵弿娴嬭瘯銆傜洰鍓嶈繖娈典唬鐮佷粎鏀寔闂瓨缂栫▼锛屼絾鎵╁睍寰堝鏄?娣诲姞銆?
### 浣跨敤椹卞姩


涓€鏃﹂┍鍔ㄨ鍔犺浇锛屼綘灏卞彲浠ヤ娇鐢?sysfs 鏀寔鏉ヨ幏鍙?C2port 鐨勶細

```

  # ls /sys/class/c2port/c2port0/
  access            flash_block_size  flash_erase       rev_id
  dev_id            flash_blocks_num  flash_size        subsystem/
  flash_access      flash_data        reset             uevent

```
鏈€鍒?C2port 璁块棶鏄绂佺敤鐨勶紝鍥犱负浣犵殑纭欢鍙兘灏嗚繖浜涚嚎璺笌鍏朵粬璁惧澶嶇敤锛屽洜姝よ
鑾峰彇璁块棶鏉冮檺闇€鎵ц锛?
```

  # echo 1 > /sys/class/c2port/c2port0/access

```
姝ゅ悗浣犲簲璇ヨ鍙栬澶?ID 涓庣増鏈?ID锛?
```

  # cat /sys/class/c2port/c2port0/dev_id
  8
  # cat /sys/class/c2port/c2port0/rev_id
  1

```
鐒惰€屽嚭浜庡畨鍏ㄥ師鍥狅紝鍦ㄧ郴缁熼棯瀛樿闂粯璁ゆ槸涓嶏細

```

  # echo 1 > /sys/class/c2port/c2port0/flash_access

```
```

  # cat /sys/class/c2port/c2port0/flash_data > image

```
```

  # echo 1 > /sys/class/c2port/c2port0/flash_erase

```
```

  # cat image > /sys/class/c2port/c2port0/flash_data

```
```

  # echo 1 > /sys/class/c2port/c2port0/reset

```
