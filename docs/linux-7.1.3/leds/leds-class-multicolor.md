
## Linux 下的多色 LED 处理


## 描述

多色 LED 类将单色 LED 归为一组，并允许控制最终合成颜色的两个方面：色相（hue）和亮度（lightness）。前者通过 multi_intensity 数组文件控制，后者通过 brightness 文件控制
## 多色类控
多色类以数组索引的方式将颜色归为一组并提供相应文件。这些文件是 led_class 框架创建LED 父节点下的子项。led_class 框架的文档见本文档目录中led-class.rst
每个彩色 LED 都会`multi_*` 文件下建立索引。颜色的顺序是任意的。可以读`multi_index` 文件以确定颜色名称对应的索引值
`multi_index` 文件是一个数组，包含在每`multi_*` 数组文件中定义的颜色字符串列表
`multi_intensity` 是一个可读写的数组，用于设置各个颜色强度。必须按顺序写入该数组的所有元素，颜色 LED 强度才会更新
## 目录布局示例


    root:/sys/class/leds/multicolor:status# ls -lR
    -rw-r--r--    1 root     root          4096 Oct 19 16:16 brightness
    -r--r--r--    1 root     root          4096 Oct 19 16:16 max_brightness
    -r--r--r--    1 root     root          4096 Oct 19 16:16 multi_index
    -rw-r--r--    1 root     root          4096 Oct 19 16:16 multi_intensity

..

## 多色类亮度控
每个 LED 的亮度级别根据“颜LED 强度设置 ÷ 全局 max_brightness 设置 × 请求的亮度”计算
`led_brightness = brightness * multi_intensity/max_brightness`

示例用户首先multi_intensity 文件写入各个 LED 的亮度级别，这些级别是实现某个多LED 组特定颜色输出所必需的
    # cat /sys/class/leds/multicolor:status/multi_index
    green blue red

    # echo 43 226 138 > /sys/class/leds/multicolor:status/multi_intensity

    red -
    	intensity = 138
    	max_brightness = 255
    green -
    	intensity = 43
    	max_brightness = 255
    blue -
    	intensity = 226
    	max_brightness = 255

..

用户可以通过写入全局 'brightness' 控制项来控制该多LED 组的亮度。假max_brightness 255，用户可能希望将该颜色组调暗一半。用户应向全局 brightness 文件写入128，随后写入每LED 的值会基于该值进行调整
    # cat /sys/class/leds/multicolor:status/max_brightness
    255
    # echo 128 > /sys/class/leds/multicolor:status/brightness

..

    adjusted_red_value = 128 * 138/255 = 69
    adjusted_green_value = 128 * 43/255 = 21
    adjusted_blue_value = 128 * 226/255 = 113

..

读取全局 brightness 文件将返回该颜色 LED 组的当前亮度值
    # cat /sys/class/leds/multicolor:status/brightness
    128

..
