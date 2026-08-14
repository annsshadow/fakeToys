## dm-service-time


dm-service-time 鏄竴涓敤浜?device-mapper 鐩爣鐨勮矾寰勯€夋嫨鍣紙path selector锛夋ā鍧楋紝瀹冧负杈撳叆鐨?I/O 閫夋嫨浼拌鏈嶅姟鏃堕棿鏈€鐭殑璺緞銆?
姣忔潯璺緞鐨勬湇鍔℃椂闂撮€氳繃灏嗚矾寰勪笂鍦ㄩ€旓紙in-flight锛塈/O 鐨勬€诲ぇ灏忛櫎浠ヨ璺緞鐨勬€ц兘鍊兼潵浼拌銆傛€ц兘鍊兼槸璺緞缁勫唴鎵€鏈夎矾寰勪箣闂寸殑鐩稿鍚炲悙鍊硷紝鍙互浣滀负琛ㄥ弬鏁版寚瀹氥€?
璺緞閫夋嫨鍣ㄧ殑鍚嶇О涓?'service-time'銆?
姣忔潯璺緞鐨勮〃鍙傛暟锛?
    [<repeat_count> [<relative_throughput>]]
	<repeat_count>:
			鍦ㄥ垏鎹㈠埌涓嬩竴鏉¤矾寰勪箣鍓嶏紝浣跨敤鎵€閫夎矾寰勫垎鍙戠殑 I/O 鏁伴噺銆?			濡傛灉鏈粰鍑猴紝鍒欎娇鐢ㄥ唴閮ㄩ粯璁ゅ€笺€傝鏌ョ湅榛樿鍊硷紝璇峰弬瑙佸凡婵€娲荤殑琛ㄣ€?	<relative_throughput>:
			璇ヨ矾寰勫湪璺緞缁勫唴鎵€鏈夎矾寰勪箣闂寸殑鐩稿鍚炲悙鍊笺€?			鏈夋晥鑼冨洿涓?0-100銆?			濡傛灉鏈粰鍑猴紝鍒欎娇鐢ㄦ渶灏忓€?'1'銆?			濡傛灉缁欏畾 '0'锛屽垯鍦ㄦ湁鍏朵粬鍏锋湁姝ｅ€艰矾寰勫彲鐢ㄦ椂锛岃璺緞涓嶄細琚€変腑銆?
姣忔潯璺緞鐨勭姸鎬侊細

    <status> <fail-count> <in-flight-size> <relative_throughput>
	<status>:
		鑻ヨ矾寰勬椿鍔ㄥ垯涓?'A'锛岃嫢璺緞澶辫触鍒欎负 'F'銆?	<fail-count>:
		璺緞澶辫触鐨勬鏁般€?	<in-flight-size>:
		璇ヨ矾寰勪笂鍦ㄩ€?I/O 鐨勫ぇ灏忋€?	<relative_throughput>:
		璇ヨ矾寰勫湪璺緞缁勫唴鎵€鏈夎矾寰勪箣闂寸殑鐩稿鍚炲悙鍊笺€?

## 绠楁硶


dm-service-time 鍦?I/O 鍒嗗彂鏃跺皢鍏跺ぇ灏忓姞鍒?'in-flight-size'锛屽畬鎴愭椂鍒嗘鍑忓幓銆?
鍩烘湰涓婏紝dm-service-time 閫夋嫨鍏锋湁鏈€灏忔湇鍔℃椂闂寸殑璺緞
```

	('in-flight-size' + 'size-of-incoming-io') / 'relative_throughput'

```
鐒惰€岋紝涓轰簡灏藉彲鑳藉噺灏戣绠楋紝浣跨敤浜嗕互涓嬩竴浜涗紭鍖栥€?
 1. 濡傛灉鍚勮矾寰勫叿鏈夌浉鍚岀殑 'relative_throughput'锛岃烦杩囬櫎娉曪紝浠呮瘮杈?'in-flight-size'銆?
 2. 濡傛灉鍚勮矾寰勫叿鏈夌浉鍚岀殑 'in-flight-size'锛岃烦杩囬櫎娉曪紝浠呮瘮杈?'relative_throughput'銆?
 3. 濡傛灉鏌愪簺璺緞鍏锋湁闈為浂 'relative_throughput' 鑰屽叾浠栬矾寰勪负闆讹紝鍒欏拷鐣ラ偅浜?'relative_throughput' 涓洪浂鐨勮矾寰勩€?
濡傛灉鏃犳硶搴旂敤杩欎簺浼樺寲锛屽垯璁＄畻鏈嶅姟鏃堕棿骞舵瘮杈冩湇鍔℃椂闂淬€?濡傛灉璁＄畻鍑虹殑鏈嶅姟鏃堕棿鐩哥瓑锛屽叿鏈夋渶澶?'relative_throughput' 鐨勮矾寰勫彲鑳芥洿濂姐€傚洜姝ゆ帴涓嬫潵姣旇緝 'relative_throughput'銆?

## 绀轰緥

鍦?2 鏉¤矾寰勶紙sda 鍜?sdb锛夎浣跨敤銆乺epeat_count == 128銆佷笖 sda 骞冲潎鍚炲悙閲忎负 1GB/s銆乻db 涓?4GB/s 鐨勬儏鍐典笅锛?```

  # echo "0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 1 8:16 128 4" \
    dmsetup create test
  #
  # dmsetup table
  test: 0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 1 8:16 128 4
  #
  # dmsetup status
  test: 0 10 multipath 2 0 0 0 1 1 E 0 2 2 8:0 A 0 0 1 8:16 A 0 0 4


```
```

  # echo "0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 2 8:16 128 8" \
    dmsetup create test
  #
  # dmsetup table
  test: 0 10 multipath 0 0 1 1 service-time 0 2 2 8:0 128 2 8:16 128 8
  #
  # dmsetup status
  test: 0 10 multipath 2 0 0 0 1 1 E 0 2 2 8:0 A 0 0 2 8:16 A 0 0 8

```
