

######## 遥控器按键表

遗憾的是，多年来一直没有人努力为不同设备创建统一的红外（IR）键码。这导致同一个红外键名在不同的红外设备上被映射得完全不同。结果就是，相同的红外键名在不同的红外设备上被映射得完全不同。正因如此，V4L2 API 现在规定了一种用于在红外上映射媒体按键的标准
这一标准应当同时V4L/DVB 驱动和用户空间应用程序所采用
这些模块Linux 的输入（input）层中将遥控器注册为键盘。这意味着红外按键看起来会像普通的键盘按键（前提是启用CONFIG_INPUT_KEYBOARD）。借助事件设备（CONFIG_INPUT_EVDEV），应用程序可以通过 /dev/input/event 设备来访问遥控器



    :header-rows:  0
    :stub-columns: 0
    :widths:       1 1 2


    - .. row 1

       - 键码

       - 含义

       - 红外上的按键示例

    - .. row 2

       - **鏁板瓧閿?*

    - .. row 3

       - `KEY_NUMERIC_0`

       - 键盘数字 0

       - 0

    - .. row 4

       - `KEY_NUMERIC_1`

       - 键盘数字 1

       - 1

    - .. row 5

       - `KEY_NUMERIC_2`

       - 键盘数字 2

       - 2

    - .. row 6

       - `KEY_NUMERIC_3`

       - 键盘数字 3

       - 3

    - .. row 7

       - `KEY_NUMERIC_4`

       - 键盘数字 4

       - 4

    - .. row 8

       - `KEY_NUMERIC_5`

       - 键盘数字 5

       - 5

    - .. row 9

       - `KEY_NUMERIC_6`

       - 键盘数字 6

       - 6

    - .. row 10

       - `KEY_NUMERIC_7`

       - 键盘数字 7

       - 7

    - .. row 11

       - `KEY_NUMERIC_8`

       - 键盘数字 8

       - 8

    - .. row 12

       - `KEY_NUMERIC_9`

       - 键盘数字 9

       - 9

    - .. row 13

       - **影片播放控制**

    - .. row 14

       - `KEY_FORWARD`

       - 立即向前快进

       - >> / FORWARD

    - .. row 15

       - `KEY_BACK`

       - 立即回退

       - <<< / BACK

    - .. row 16

       - `KEY_FASTFORWARD`

       - 以更快速度播放影片

       - >>> / FORWARD

    - .. row 17

       - `KEY_REWIND`

       - 倒放影片

       - REWIND / BACKWARD

    - .. row 18

       - `KEY_NEXT`

       - 选择下一章节/子章间隔

       - NEXT / SKIP

    - .. row 19

       - `KEY_PREVIOUS`

       - 选择上一章节/子章间隔

       - << / PREV / PREVIOUS

    - .. row 20

       - `KEY_AGAIN`

       - 重复视频或某段视
       - REPEAT / LOOP / RECALL

    - .. row 21

       - `KEY_PAUSE`

       - 暂停数据
       - PAUSE / FREEZE

    - .. row 22

       - `KEY_PLAY`

       - 以正常时移方式播放影
       - NORMAL TIMESHIFT / LIVE / >

    - .. row 23

       - `KEY_PLAYPAUSE`

       - 在播放与暂停之间切换

       - PLAY / PAUSE

    - .. row 24

       - `KEY_STOP`

       - 停止数据
       - STOP

    - .. row 25

       - `KEY_RECORD`

       - 开停止录制数据
       - CAPTURE / REC / RECORD/PAUSE

    - .. row 26

       - `KEY_CAMERA`

       - 拍摄一张图
       - CAMERA ICON / CAPTURE / SNAPSHOT

    - .. row 27

       - `KEY_SHUFFLE`

       - 启用随机播放模式

       - SHUFFLE

    - .. row 28

       - `KEY_TIME`

       - 激活时移模
       - TIME SHIFT

    - .. row 29

       - `KEY_TITLE`

       - 允许切换章节

       - CHAPTER

    - .. row 30

       - `KEY_SUBTITLE`

       - 允许切换字幕

       - SUBTITLE

    - .. row 31

       - **图像控制**

    - .. row 32

       - `KEY_BRIGHTNESSDOWN`

       - 降低亮度

       - BRIGHTNESS DECREASE

    - .. row 33

       - `KEY_BRIGHTNESSUP`

       - 提高亮度

       - BRIGHTNESS INCREASE

    - .. row 34

       - `KEY_ANGLE`

       - 切换摄像机角度（针对存储了多个角度的视频
       - ANGLE / SWAP

    - .. row 35

       - `KEY_EPG`

       - 打开电子节目指南（EPG
       - EPG / GUIDE

    - .. row 36

       - `KEY_TEXT`

       - 激切换隐藏字幕模式

       - CLOSED CAPTION/TELETEXT / DVD TEXT / TELETEXT / TTX

    - .. row 37

       - **音频控制**

    - .. row 38

       - `KEY_AUDIO`

       - 更改音频
       - AUDIO SOURCE / AUDIO / MUSIC

    - .. row 39

       - `KEY_MUTE`

       - 静音/取消静音

       - MUTE / DEMUTE / UNMUTE

    - .. row 40

       - `KEY_VOLUMEDOWN`

       - 降低音量

       - VOLUME- / VOLUME DOWN

    - .. row 41

       - `KEY_VOLUMEUP`

       - 提高音量

       - VOLUME+ / VOLUME UP

    - .. row 42

       - `KEY_MODE`

       - 更改声音模式

       - MONO/STEREO

    - .. row 43

       - `KEY_LANGUAGE`

       - 选择语言

       - 1ST / 2ND LANGUAGE / DVD LANG / MTS/SAP / MTS SEL

    - .. row 44

       - **频道控制**

    - .. row 45

       - `KEY_CHANNEL`

       - 跳到下一个收藏频
       - ALT / CHANNEL / CH SURFING / SURF / FAV

    - .. row 46

       - `KEY_CHANNELDOWN`

       - 按序递减频道

       - CHANNEL - / CHANNEL DOWN / DOWN

    - .. row 47

       - `KEY_CHANNELUP`

       - 按序递增频道

       - CHANNEL + / CHANNEL UP / UP

    - .. row 48

       - `KEY_DIGITS`

       - 用多于一位数字输入频
       - PLUS / 100/ 1xx / xxx / -/-- / Single Double Triple Digit

    - .. row 49

       - `KEY_SEARCH`

       - 开始频道自动扫
       - SCAN / AUTOSCAN

    - .. row 50

       - **褰╄壊閿?*

    - .. row 51

       - `KEY_BLUE`

       - 红外蓝色
       - BLUE

    - .. row 52

       - `KEY_GREEN`

       - 红外绿色
       - GREEN

    - .. row 53

       - `KEY_RED`

       - 红外红色
       - RED

    - .. row 54

       - `KEY_YELLOW`

       - 红外黄色
       - YELLOW

    - .. row 55

       - **媒体选择**

    - .. row 56

       - `KEY_CD`

       - 将输入源切换到光盘（Compact Disc
       - CD

    - .. row 57

       - `KEY_DVD`

       - 将输入切换到 DVD

       - DVD / DVD MENU

    - .. row 58

       - `KEY_EJECTCLOSECD`

       - 打开/关闭 CD/DVD 播放
       - -> ) / CLOSE / OPEN

    - .. row 59

       - `KEY_MEDIA`

       - 打开/关闭媒体应用程序

       - PC/TV / TURN ON/OFF APP

    - .. row 60

       - `KEY_PC`

       - 从电视切换到电脑

       - PC

    - .. row 61

       - `KEY_RADIO`

       - 进入 AM/FM 收音机模
       - RADIO / TV/FM / TV/RADIO / FM / FM/RADIO

    - .. row 62

       - `KEY_TV`

       - 选择电视模式

       - TV / LIVE TV

    - .. row 63

       - `KEY_TV2`

       - 选择有线电视模式

       - AIR/CBL

    - .. row 64

       - `KEY_VCR`

       - 选择录像机（VCR）模
       - VCR MODE / DTR

    - .. row 65

       - `KEY_VIDEO`

       - 在输入模式之间切
       - SOURCE / SELECT / DISPLAY / SWITCH INPUTS / VIDEO

    - .. row 66

       - **电源控制**

    - .. row 67

       - `KEY_POWER`

       - 打开/关闭计算
       - SYSTEM POWER / COMPUTER POWER

    - .. row 68

       - `KEY_POWER2`

       - 打开/关闭应用程序

       - TV ON/OFF / POWER

    - .. row 69

       - `KEY_SLEEP`

       - 激活睡眠定时器

       - SLEEP / SLEEP TIMER

    - .. row 70

       - `KEY_SUSPEND`

       - 将计算机置于挂起模式

       - STANDBY / SUSPEND

    - .. row 71

       - **窗口控制**

    - .. row 72

       - `KEY_CLEAR`

       - 停止数据流并返回默认的输入视音频

       - CLEAR / RESET / BOSS KEY

    - .. row 73

       - `KEY_CYCLEWINDOWS`

       - 最小化窗口并移动到下一
       - ALT-TAB / MINIMIZE / DESKTOP

    - .. row 74

       - `KEY_FAVORITES`

       - 打开收藏流窗
       - TV WALL / Favorites

    - .. row 75

       - `KEY_MENU`

       - 调用应用程序菜单

       - 2ND CONTROLS (USA: MENU) / DVD/MENU / SHOW/HIDE CTRL

    - .. row 76

       - `KEY_NEW`

       - 打开/关闭画中
       - PIP

    - .. row 77

       - `KEY_OK`

       - 向应用程序发送确认码

       - OK / ENTER / RETURN

    - .. row 78

       - `KEY_ASPECT_RATIO`

       - 閫夋嫨灞忓箷瀹介珮姣。
       - 4:3 16:9 SELECT

    - .. row 79

       - `KEY_FULL_SCREEN`

       - 将设备置于缩全屏模式

       - ZOOM / FULL SCREEN / ZOOM+ / HIDE PANEL / SWITCH

    - .. row 80

       - **瀵艰埅閿?*

    - .. row 81

       - `KEY_ESC`

       - 取消当前操作

       - CANCEL / BACK

    - .. row 82

       - `KEY_HELP`

       - 打开帮助窗口

       - HELP

    - .. row 83

       - `KEY_HOMEPAGE`

       - 导航到主
       - HOME

    - .. row 84

       - `KEY_INFO`

       - 打开屏幕显示（OSD
       - DISPLAY INFORMATION / OSD

    - .. row 85

       - `KEY_WWW`

       - 打开默认浏览
       - WEB

    - .. row 86

       - `KEY_UP`

       - 上方向键

       - UP

    - .. row 87

       - `KEY_DOWN`

       - 下方向键

       - DOWN

    - .. row 88

       - `KEY_LEFT`

       - 左方向键

       - LEFT

    - .. row 89

       - `KEY_RIGHT`

       - 右方向键

       - RIGHT

    - .. row 90

       - **鏉傞」閿?*

    - .. row 91

       - `KEY_DOT`

       - 返回一个点

       - .

    - .. row 92

       - `KEY_FN`

       - 选择一个功
       - FUNCTION


需要注意的是，在有些较廉价的红外遥控器上，有时会缺少一些基本的按键。因此，建议


    :header-rows:  0
    :stub-columns: 0


    - .. row 1

       - 在较简单的红外遥控器上，若没有独立的频道键，需要将 UP 映射`KEY_CHANNELUP`

    - .. row 2

       - 在较简单的红外遥控器上，若没有独立的频道键，需要将 DOWN 映射`KEY_CHANNELDOWN`

    - .. row 3

       - 在较简单的红外遥控器上，若没有独立的音量键，需要将 LEFT 映射`KEY_VOLUMEDOWN`

    - .. row 4

       - 在较简单的红外遥控器上，若没有独立的音量键，需要将 RIGHT 映射`KEY_VOLUMEUP`
