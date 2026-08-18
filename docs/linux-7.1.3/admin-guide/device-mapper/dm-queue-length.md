## dm-queue-length


dm-queue-length 鏄?device-mapper 鐩爣鐨勮矾寰勯€夋嫨鍣ㄦā鍧楋紝瀹冮€夋嫨鍏锋湁鏈€灏戝湪閫?I/O 鐨勮矾寰勩€傝矾寰勯€夋嫨鍣ㄥ悕绉颁负 'queue-length'銆?
姣忔潯璺緞鐨勮〃鍙傛暟锛歔<repeat_count>]

```

	<repeat_count>锛氬湪浣跨敤鎵€閫夎矾寰勫垎娲?I/O 鐨勬暟閲忥紝涔嬪悗鍒囨崲鍒颁笅涓€鏉¤矾寰勩€?			鑻ユ湭缁欏畾锛屼娇鐢ㄥ唴閮ㄩ粯璁ゅ€笺€傝鏌ョ湅榛樿鍊硷紝璇峰弬闃呭凡婵€娲荤殑琛ㄣ€?
```
姣忔潯璺緞鐨勭姸鎬侊細<status> <fail-count> <in-flight>

```

	<status>锛氳矾寰勫浜庢椿鍔ㄧ姸鎬佷负 'A'锛岃矾寰勫け璐ヤ负 'F'銆?	<fail-count>锛氳矾寰勫け璐ョ殑娆℃暟銆?	<in-flight>锛氳矾寰勪笂鍦ㄩ€?I/O 鐨勬暟閲忋€?

```
## 绠楁硶


dm-queue-length 鍦ㄥ垎娲?瀹屾垚 I/O 鏃跺垎鍒€掑/閫掑噺 'in-flight'銆?dm-queue-length 閫夋嫨鍏锋湁鏈€灏?'in-flight' 鐨勮矾寰勩€?

## 绀轰緥


鍦?2 鏉¤矾寰勶紙sda 涓?sdb锛変笖 repeat_count == 128 鐨勬儏鍐典笅浣跨敤銆?
```

  # echo "0 10 multipath 0 0 1 1 queue-length 0 2 1 8:0 128 8:16 128" \
    dmsetup create test
  #
  # dmsetup table
  test: 0 10 multipath 0 0 1 1 queue-length 0 2 1 8:0 128 8:16 128
  #
  # dmsetup status
  test: 0 10 multipath 2 0 0 0 1 1 E 0 2 1 8:0 A 0 0 8:16 A 0 0

```
