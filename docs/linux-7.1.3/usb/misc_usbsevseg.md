## USB 7 娈垫暟鐮佺鏄剧ず鍣?


鐢?Delcom Engineering 鍒堕€?

### 璁惧淇℃伅

USB VENDOR_ID	0x0fc5
USB PRODUCT_ID	0x1227
6 瀛楃鍜?8 瀛楃鏄剧ず鍣ㄩ兘鍏锋湁 PRODUCT_ID锛屽苟涓旀牴鎹?Delcom Engineering 鐨勮娉曪紝鏃犳硶浠庤澶囪幏鍙栧彲鏌ヨ鐨勪俊鎭潵鍖哄垎瀹冧滑銆?

### 璁惧妯″紡

榛樿鎯呭喌涓嬶紝椹卞姩鍋囧畾鏄剧ず鍣ㄥ彧鏈?6 涓瓧绗︺€? 涓瓧绗︾殑妯″紡涓猴細

	MSB 0x06; LSB 0x3f

瀵逛簬 8 瀛楃鏄剧ず鍣細

	MSB 0x08; LSB 0xff

璁惧鍙互鎺ュ彈鈥滄枃鏈€濓紝鍙互鏄?raw銆乭ex 鎴?ascii 鏂囨湰妯″紡銆?
raw 鎵嬪姩鎺у埗姣忎釜娈碉紝
hex 鏈熸湜姣忎釜瀛楃鐨勫€煎湪 0-15 涔嬮棿锛?
ascii 鏈熸湜姣忎釜瀛楃鐨勫€煎湪 '0'-'9' 鍜?'A'-'F' 涔嬮棿銆?
榛樿鏄?ascii銆?

### 璁惧鎿嶄綔

1. 鎵撳紑璁惧锛?
	echo 1 > /sys/bus/usb/.../powered
2. 璁剧疆璁惧鐨勬ā寮忥細
	echo $mode_msb > /sys/bus/usb/.../mode_msb
	echo $mode_lsb > /sys/bus/usb/.../mode_lsb
3. 璁剧疆鏂囨湰妯″紡锛?
	echo $textmode > /sys/bus/usb/.../textmode
4. 璁剧疆鏂囨湰锛堜緥濡傦級锛?
	echo "123ABC" > /sys/bus/usb/.../text (ascii)
	echo "A1B2" > /sys/bus/usb/.../text (ascii)
	echo -ne "\x01\x02\x03" > /sys/bus/usb/.../text (hex)
5. 璁剧疆灏忔暟鐐广€?
	璁惧鏈?6 鎴?8 涓皬鏁扮偣銆?
	瑕佽缃 n 涓皬鏁扮偣锛岃绠?10 ** n
	骞跺皢鍏?echo 鍒?/sys/bus/usb/.../decimals
	瑕佽缃涓皬鏁扮偣锛屽皢鍚勪釜骞傜浉鍔犮€?
	渚嬪锛岃璁剧疆绗?0 涓拰绗?3 涓皬鏁扮偣锛?
	echo 1001 > /sys/bus/usb/.../decimals
