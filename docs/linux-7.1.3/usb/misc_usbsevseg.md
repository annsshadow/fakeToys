## USB 7 段数码管显示


Delcom Engineering 制

### 设备信息

USB VENDOR_ID	0x0fc5
USB PRODUCT_ID	0x1227
6 字符8 字符显示器都具有 PRODUCT_ID，并且根Delcom Engineering 的说法，无法从设备获取可查询的信息来区分它们

### 设备模式

默认情况下，驱动假定显示器只6 个字符 个字符的模式为：

	MSB 0x06; LSB 0x3f

对于 8 字符显示器：

	MSB 0x08; LSB 0xff

设备可以接受“文本”，可以raw、hex ascii 文本模式
raw 手动控制每个段，
hex 期望每个字符的值在 0-15 之间
ascii 期望每个字符的值在 '0'-'9' 'A'-'F' 之间
默认ascii

### 设备操作

1. 打开设备
	echo 1 > /sys/bus/usb/.../powered
2. 设置设备的模式：
	echo $mode_msb > /sys/bus/usb/.../mode_msb
	echo $mode_lsb > /sys/bus/usb/.../mode_lsb
3. 设置文本模式
	echo $textmode > /sys/bus/usb/.../textmode
4. 设置文本（例如）
	echo "123ABC" > /sys/bus/usb/.../text (ascii)
	echo "A1B2" > /sys/bus/usb/.../text (ascii)
	echo -ne "\x01\x02\x03" > /sys/bus/usb/.../text (hex)
5. 设置小数点
	设备6 8 个小数点
	要设置第 n 个小数点，计10 ** n
	并将echo /sys/bus/usb/.../decimals
	要设置多个小数点，将各个幂相加
	例如，要设置0 个和3 个小数点
	echo 1001 > /sys/bus/usb/.../decimals
