yealink usb-p1k 电话驱动文档



状态



p1k 是一款相对廉价的 USB 1.1 电话，具备：


  - 键盘		完全支持，yealink.ko / input event API
  - LCD		完全支持，yealink.ko / sysfs API
  - LED		完全支持，yealink.ko / sysfs API
  - 拨号音		完全支持，yealink.ko / sysfs API
  - 铃声		完全支持，yealink.ko / sysfs API
  - 音频播放		完全支持，snd_usb_audio.ko / alsa API
  - 音频录制		完全支持，snd_usb_audio.ko / alsa API


有关厂商文档，请参阅 http://www.yealink.com



键盘特性



内核中当前的映射由 map_p1k_to_key 提供
```

   Physical USB-P1K button layout	input events


              up			     up
        IN           OUT		left,	right
             down			    down

      pickup   C    hangup		enter, backspace, escape
        1      2      3			1, 2, 3
        4      5      6			4, 5, 6,
        7      8      9			7, 8, 9,
        *      0      #			*, 0, #,

```

“up”和“down”键在按钮上以箭头表示。“pickup”和“hangup”键在按钮上以绿色和红色电话表示。



LCD 特性


```

    |[]   [][]   [][]   [][]   in   |[][]
    |[] M [][] D [][] : [][]   out  |[][]
                              store

    NEW REP         SU MO TU WE TH FR SA

    [] [] [] [] [] [] [] [] [] [] [] []
    [] [] [] [] [] [] [] [] [] [] [] []


  Line 1  Format (see below)	: 18.e8.M8.88...188
	  Icon names		:   M  D  :  IN OUT STORE
  Line 2  Format		: .........
	  Icon name		: NEW REP SU MO TU WE TH FR SA
  Line 3  Format		: 888888888888


```

格式描述：
  从用户空间的角度看，世界被划分为“digits”（数字）和“icons”（图标）。
  一个数字可以有字符集，一个图标只能处于“开”或“关”状态。

```

    '8' :  Generic 7 segment digit with individual addressable segments

    Reduced capability 7 segment digit, when segments are hard wired together.
    '1' : 2 segments digit only able to produce a 1.
    'e' : Most significant day of the month digit,
          able to produce at least 1 2 3.
    'M' : Most significant minute digit,
          able to produce at least 0 1 2 3 4 5.

    Icons or pictograms:
    '.' : For example like AM, PM, SU, a 'dot' .. or other single segment
	  elements.


```

驱动使用方法


```

  /sys/.../
           line1	Read/Write, lcd line1
           line2	Read/Write, lcd line2
           line3	Read/Write, lcd line3

	   get_icons    Read, returns a set of available icons.
	   hide_icon    Write, hide the element by writing the icon name.
	   show_icon    Write, display the element by writing the icon name.

	   map_seg7	Read/Write, the 7 segments char set, common for all
			yealink phones. (see map_to_7segment.h)

	   ringtone	Write, upload binary representation of a ringtone,
			see yealink.c. status EXPERIMENTAL due to potential
			races between async. and sync usb calls.


```

lineX



读取 /sys/../lineX 将返回带有当前值的格式字符串。

```

    cat ./line3
    888888888888
    Linux Rocks!

```

写入 /sys/../lineX 将设置对应的 LCD 行。


 - 多余的字符会被忽略。
 - 如果写入的字符少于允许数量，剩余的数字保持不变。
 - 制表符 '\t' 和 '\n' 字符不会覆盖原有内容。
 - 向图标写入空格将始终隐藏其内容。

```

    date +"%m.%e.%k:%M"  | sed 's/^0/ /' > ./line1

  Will update the LCD with the current date & time.


```

get_icons


```

  cat ./get_icons
  on M
  on D
  on :
     IN
     OUT
     STORE
     NEW
     REP
     SU
     MO
     TU
     WE
     TH
     FR
     SA
     LED
     DIALTONE
     RINGTONE


```

显示/隐藏图标



写入这些文件将更新图标的状态。
一次只能更新一个图标。


如果某个图标也位于 ./lineX 上，则其对应的值会使用该图标的首字母进行更新。

```

    echo -n "STORE" > ./show_icon

    cat ./line1
    18.e8.M8.88...188
		  S

  Example - sound the ringtone for 10 seconds::

    echo -n RINGTONE > /sys/..../show_icon
    sleep 10
    echo -n RINGTONE > /sys/..../hide_icon


```

声音特性



声音由 ALSA 驱动 snd_usb_audio 提供支持


设备实际的极限是一个 16 位通道，采样率与播放率均为 8000 Hz。

```

    arecord -v -d 10 -r 8000 -f S16_LE -t wav  foobar.wav

  Example - playback test::

    aplay foobar.wav


```

故障排查



:Q: 模块 yealink 编译并安装没有任何问题，但电话未被初始化，且对任何操作都没有反应。
:A: 如果你在 dmesg 中看到类似如下内容：
    hiddev0: USB HID v1.00 Device [Yealink Network Technology Ltd. VOIP USB Phone
    这意味着 hid 驱动抢先占用了该设备。请尝试在任何其他 usb hid 驱动之前加载 yealink 模块。请参阅你所使用发行版提供的关于模块配置的说明。


:Q: 电话现在可以工作了（显示版本并接受按键输入），但我找不到 sysfs 文件。
:A: sysfs 文件位于特定的 usb 端点上。在大多数发行版中，你可以执行："find /sys/ -name get_icons" 来获取线索。



致谢与感谢



  - Olivier Vandorpe，因启动 usbb2k-api 项目并完成了大量逆向工程工作。
  - Martin Diehl，因指出如何处理 USB 内存分配。
  - Dmitry Torokhov，因大量的代码审查与建议。

