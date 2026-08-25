## HD-Audio 编解码器特定型号


## ALC880

3stack
    背板 3 个插孔与一个耳机输出
3stack-digout
    背板 3 个插孔、一个耳机输出与一SPDIF 输出
5stack
    背板 5 个插孔，前面2 个插5stack-digout
    背板 5 个插孔，前面2 个插孔，一SPDIF 输出
6stack
    背板 6 个插孔，前面2 个插6stack-digout
    6 个插孔，带一SPDIF 输出
6stack-automute
    6 个插孔，带耳机插孔检
## ALC260

gpio1
    启用 GPIO1
coef
    通过 COEF 表启EAPD
fujitsu
    FSC S7020 的修fujitsu-jwse
    带插孔模式与耳机麦克风支持的 FSC S7020 修正

## ALC262

inv-dmic
    内置麦克风反相的变通方fsc-h270
    Fujitsu-Siemens Celsius H270 的修fsc-s7110
    Fujitsu-Siemens Lifebook S7110 的修hp-z200
    HP Z200 的修tyan
    Tyan Thunder n6650W 的修lenovo-3000
    Lenovo 3000 的修benq
    Benq ED8 的修benq-t31
    Benq T31 的修bayleybay
    Intel BayleyBay 的修
## ALC267/268

inv-dmic
    内置麦克风反相的变通方hp-eapd
    NID 0x15 上禁用耳机 EAPD
spdif
    NID 0x1e 上启SPDIF 输出

## ALC22x/23x/25x/269/27x/28x/29x（以及厂商特定的 ALC3xxx 型号
laptop-amic
    带模拟麦克风输入的笔记本
laptop-dmic
    带数字麦克风输入的笔记本
alc269-dmic
    启用 ALC269(VA) 数字麦克风变通方alc271-dmic
    启用 ALC271X 数字麦克风变通方inv-dmic
    内置麦克风反相的变通方headset-mic
    表示组合式头戴（耳机+麦克风）插孔
headset-mode
    ALC269 及同类提供更全面的头戴支headset-mode-no-hp-mic
    不含耳机麦克风的头戴模式支持
lenovo-dock
    为部分联想机器启用扩展坞 I/O
hp-gpio-led
    HP 笔记本上GPIO LED 支持
hp-dock-gpio-mic1-led
    带麦克风 LED 支持HP 扩展dell-headset-multi
    头戴插孔，也可用作麦克风输入
dell-headset-dock
    头戴插孔（不含麦克风输入），同时带扩展坞 I/O
dell-headset3
    头戴插孔（不含麦克风输入），同时带扩展坞 I/O，变3
dell-headset4
    头戴插孔（不含麦克风输入），同时带扩展坞 I/O，变4
alc283-dac-wcaps
    ALC283 Chromebook 修复
alc283-sense-combo
    ALC283 上的组合插孔检tpt440-dock
    用于联想 Thinkpad 扩展坞支持的引脚配置
tpt440
    联想 Thinkpad T440s 配置
tpt460
    联想 Thinkpad T460/560 配置
tpt470-dock
    鑱旀兂 Thinkpad T470 鎵╁睍鍧為厤缃?dual-codecs
    带双编解码器的联想笔记本
alc700-ref
    ALC700 编解码器Intel 参考板
vaio
    索尼 VAIO 笔记本引脚修dell-m101z
    Dell M101z COEF 设置
asus-g73jw
    ASUS G73JW 低音炮引脚修lenovo-eapd
    联想笔记本的反相 EAPD 设置
sony-hweq
    索尼笔记本的硬件 EQ COEF 设置
pcm44k
    固定 PCM 44kHz 约束（用于有缺陷的设备）
lifebook
    Fujitsu Lifebook 鎵╁睍鍧炲紩鑴氫慨澶?lifebook-extmic
    Fujitsu Lifebook 澶存埓楹﹀厠椋庝慨澶?lifebook-hp-pin
    Fujitsu Lifebook 耳机引脚修复
lifebook-u7x7
    Lifebook U7x7 修复
alc269vb-amic
    ALC269VB 妯℃嫙楹﹀厠椋庡紩鑴氫慨澶?alc269vb-dmic
    ALC269VB 鏁板瓧楹﹀厠椋庡紩鑴氫慨澶?hp-mute-led-mic1
    HP 上通过 Mic1 引脚的静LED
hp-mute-led-mic2
    HP 上通过 Mic2 引脚的静LED
hp-mute-led-mic3
    HP 上通过 Mic3 引脚的静LED
hp-gpio-mic1
    HP 上的 GPIO + Mic1 引脚 LED
hp-line1-mic1
    HP 上通过 Line1 + Mic1 引脚的静LED
noshutup
    跳过 shutup 回调
sony-nomic
    索尼笔记本头戴麦克风修复
aspire-headset-mic
    Acer Aspire 头戴引脚修复
asus-x101
    ASUS X101 修复
acer-ao7xx
    Acer AO7xx 修复
acer-aspire-e1
    Acer Aspire E1 修复
acer-ac700
    Acer AC700 修复
limit-mic-boost
    限制联想机器上内置麦克风的增asus-zenbook
    ASUS Zenbook 修复
asus-zenbook-ux31a
    ASUS Zenbook UX31A 修复
ordissimo
    Ordissimo EVE2（或 Malata PC-B1303）修asus-tx300
    ASUS TX300 修复
alc283-int-mic
    联想机器ALC283 COEF 设置
mono-speakers
    Dell Inspiron 的低音炮与头戴修alc290-subwoofer
    Dell Vostro 的低音炮修复
thinkpad
    与联想机器上thinkpad_acpi 驱动绑定
dmic-thinkpad
    thinkpad_acpi 绑定 + 数字麦克风支alc255-acer
    Acer 机器上的 ALC255 修复
alc255-asus
    ASUS 机器上的 ALC255 修复
alc255-dell1
    Dell 机器上的 ALC255 修复
alc255-dell2
    Dell 机器上的 ALC255 修复，变2
alc293-dell1
    Dell 机器上的 ALC293 修复
alc283-headset
    ALC283 上的头戴引脚修复
aspire-v5
    Acer Aspire V5 修复
hp-gpio4
    HP GPIO Mic1 引脚静音 LED 修复
hp-gpio-led
    HP 上的 GPIO 静音 LED
hp-gpio2-hotkey
    HP 上带热键处理GPIO 静音 LED
hp-dock-pins
    HP 上的 GPIO 静音 LED 与扩展坞支持
hp-dock-gpio-mic
    HP 上的 GPIO、麦克风静音 LED 与扩展坞支持
hp-9480m
    HP 9480m 修复
alc288-dell1
    Dell 机器上的 ALC288 修复
alc288-dell-xps13
    Dell XPS13 上的 ALC288 修复
dell-e7x
    Dell E7x 修复
alc293-dell
    Dell 机器上的 ALC293 修复
alc298-dell1
    Dell 机器上的 ALC298 修复
alc298-dell-aio
    Dell AIO 机器上的 ALC298 修复
alc275-dell-xps
    Dell XPS 机型上的 ALC275 修复
lenovo-spk-noise
    联想机器上扬声器噪声的变通方lenovo-hotkey
    联想机器上通过 Mic2 引脚的热键支dell-spk-noise
    Dell 机器上扬声器噪声的变通方alc255-dell1
    Dell 机器上的 ALC255 修复
alc295-disable-dac3
    ALC295 上禁DAC3 路由
alc280-hp-headset
    HP Elitebook 修复
alc221-hp-mic
    HP 机器上的前置麦克风引脚修alc298-spk-volume
    ALC298 上扬声器引脚路由的变通方dell-inspiron-7559
    Dell Inspiron 7559 修复
ativ-book
    三星 Ativ book 8 修复
alc221-hp-mic
    HP 机器上的 ALC221 头戴修复
alc256-asus-mic
    ASUS 机器上的 ALC256 修复
alc256-asus-aio
    ASUS AIO 机器上的 ALC256 修复
alc233-eapd
    ASUS 机器上的 ALC233 修复
alc294-lenovo-mic
    联想 AIO 机器上的 ALC294 麦克风引脚修alc225-wyse
    Dell Wyse 修复
alc274-dell-aio
    Dell AIO 机器上的 ALC274 修复
alc255-dummy-lineout
    Dell Precision 3930 修复
alc255-dell-headset
    Dell Precision 3630 修复
alc295-hp-x360
    HP Spectre X360 修复
alc-sense-combo
    Chrome 平台的头戴按键支huawei-mbx-stereo
    为华MBX 立体声扬声器启用初始化动词；
    可能有风险，自行承担尝试风险
alc298-samsung-headphone
    ALC298 的三星笔记本
alc256-samsung-headphone
    ALC256 的三星笔记本

## ALC66x/67x/892

aspire
    Aspire 笔记本低音炮引脚修复
ideapad
    Ideapad 笔记本低音炮引脚修复
mario
    Chromebook mario 型号修复
hp-rp5800
    HP RP5800 耳机引脚修复
asus-mode1
    ASUS
asus-mode2
    ASUS
asus-mode3
    ASUS
asus-mode4
    ASUS
asus-mode5
    ASUS
asus-mode6
    ASUS
asus-mode7
    ASUS
asus-mode8
    ASUS
zotac-z68
    Zotac Z68 前置耳机修复
inv-dmic
    内置麦克风反相的变通方alc662-headset-multi
    Dell 头戴插孔，也可用作麦克风输入（ALC662dell-headset-multi
    头戴插孔，也可用作麦克风输入
alc662-headset
    ALC662 上的头戴模式支持
alc668-headset
    ALC668 上的头戴模式支持
bass16
    引脚 0x16 上的低音扬声器修bass1a
    引脚 0x1a 上的低音扬声器修automute
    ALC668 的自动静音修dell-xps13
    Dell XPS13 修复
asus-nx50
    ASUS Nx50 修复
asus-nx51
    ASUS Nx51 修复
asus-g751
    ASUS G751 修复
alc891-headset
    ALC891 上的头戴模式支持
alc891-headset-multi
    Dell 头戴插孔，也可用作麦克风输入（ALC891acer-veriton
    Acer Veriton 扬声器引脚修asrock-mobo
    修复无效0x15 / 0x16 引脚
usi-headset
    USI 机器上的头戴支持
dual-codecs
    带双编解码器的联想笔记本
alc285-hp-amp-init
    需要扬声器放大器初始化HP 笔记本（ALC285
## ALC680

N/A

## ALC88x/898/1150/1220

abit-aw9d
    Abit AW9D-MAX 引脚修复
lenovo-y530
    Lenovo Y530 引脚修复
acer-aspire-7736
    Acer Aspire 7736 修复
asus-w90v
    ASUS W90V 引脚修复
cd
    启用音频 CD 引脚 NID 0x1c
no-front-hp
    禁用前置耳机引脚 NID 0x1b
vaio-tt
    VAIO TT 引脚修复
eee1601
    ASUS Eee 1601 COEF 设置
alc882-eapd
    更改 ALC882 上的 EAPD COEF 模式
alc883-eapd
    更改 ALC883 上的 EAPD COEF 模式
gpio1
    启用 GPIO1
gpio2
    启用 GPIO2
gpio3
    启用 GPIO3
alc889-coef
    设置 ALC889 COEF
asus-w2jc
    ASUS W2JC 修复
acer-aspire-4930g
    Acer Aspire 4930G/5930G/6530G/6930G/7730G
acer-aspire-8930g
    Acer Aspire 8330G/6935G
acer-aspire
    其他 Acer Aspire
macpro-gpio
    Mac Pro GPIO 设置
dac-route
    Acer Aspire DAC 路由的变通方mbp-vref
    Macbook Pro Vref 设置
imac91-vref
    iMac 9,1 Vref 设置
mba11-vref
    MacBook Air 1,1 Vref 设置
mba21-vref
    MacBook Air 2,1 Vref 设置
mp11-vref
    Mac Pro 1,1 Vref 设置
mp41-vref
    Mac Pro 4,1 Vref 设置
inv-dmic
    内置麦克风反相的变通方no-primary-hp
    VAIO Z/VGC-LN51JGB 变通方案（针对固定的扬声器 DACasus-bass
    ASUS ET2700 的低音扬声器设置
dual-codecs
    用于游戏主板ALC1220 双编解码clevo-p950
    Clevo P950 修复

## ALC861/660

N/A

## ALC861VD/660VD

N/A

## CMI9880

minimal
    背板 3 个插min_fp
    背板 3 个插孔，前面2 个插full
    背板 6 个插孔，前面2 个插full_dig
    背板 6 个插孔，前面2 个插孔，SPDIF 输入/输出
allout
    背板 5 个插孔，前面2 个插孔，SPDIF 输出
auto
    自动配置读取 BIOS（默认）

## AD1882 / AD1882A

3stack
    3 栈模3stack-automute
    带前置耳机自动静音3 栈（默认6stack
    6 栈模
## AD1884A / AD1883 / AD1984A / AD1984B

desktop	3 栈台式机（默认）
laptop	带耳机插孔检测的笔记mobile	带耳机插孔检测的移动设备
thinkpad	联想 Thinkpad X300
touchsmart	HP Touchsmart

## AD1884

N/A

## AD1981

basic		3 插孔（默认）
hp		HP nx6320
thinkpad	联想 Thinkpad T60/X60/Z60
toshiba	东芝 U205

## AD1983

N/A

## AD1984

basic	默认配置
thinkpad	联想 Thinkpad T61/X61
dell_desktop	Dell T3400

## AD1986A

3stack
    3 栈，共享环绕laptop
    2 声道（FSC V2060、三M50laptop-imic
    带内置麦克风2 声道
eapd
    持续开EAPD

## AD1988/AD1988B/AD1989A/AD1989B

6stack
    6 插孔
6stack-dig
    同上，带 SPDIF
3stack
    3 插孔
3stack-dig
    同上，带 SPDIF
laptop
    带耳机插孔自动静音3 插孔
laptop-dig
    同上，带 SPDIF
auto
    自动配置读取 BIOS（默认）

## Conexant 5045

cap-mix-amp
    修复混音器组件的输入电平上限
toshiba-p105
    东芝 P105 修正
hp-530
    HP 530 修正

## Conexant 5047

cap-mix-amp
    修复混音器组件的输入电平上限

## Conexant 5051

lenovo-x200
    联想 X200 修正

## Conexant 5066

stereo-dmic
    反相立体声数字麦克风的变通方gpio1
    启用 GPIO1 引脚
headphone-mic-pin
    在无检测的情况下启用耳机麦克NID 0x18
tp410
    Thinkpad T400 及同类修thinkpad
    Thinkpad 静音/麦克LED 修正
lemote-a1004
    Lemote A1004 修正
lemote-a1205
    Lemote A1205 修正
olpc-xo
    OLPC XO 修正
mute-led-eapd
    通过 EAPD 控制静音 LED
hp-dock
    HP 鎵╁睍鍧炴敮鎸?mute-led-gpio
    通过 GPIO 控制静音 LED
hp-mic-fix
    HP 机器上头戴麦克风引脚的修
## STAC9200

ref
    参考板
oqo
    OQO Model 2
dell-d21
    Dell（未知）
dell-d22
    Dell（未知）
dell-d23
    Dell（未知）
dell-m21
    Dell Inspiron 630m、Dell Inspiron 640m
dell-m22
    Dell Latitude D620、Dell Latitude D820
dell-m23
    Dell XPS M1710、Dell Precision M90
dell-m24
    Dell Latitude 120L
dell-m25
    Dell Inspiron E1505n
dell-m26
    Dell Inspiron 1501
dell-m27
    Dell Inspiron E1705/9400
gateway-m4
    EAPD 控制Gateway 笔记gateway-m4-2
    EAPD 控制Gateway 笔记panasonic
    松下 CF-74
auto
    BIOS 设置（默认）

## STAC9205/9254

ref
    参考板
dell-m42
    Dell（未知）
dell-m43
    Dell Precision
dell-m44
    Dell Inspiron
eapd
    保持 EAPD 开启（Gateway T1616auto
    BIOS 设置（默认）

## STAC9220/9221

ref
    参考板
3stack
    D945 3 鏍?5stack
    D945 5 鏍?+ SPDIF
intel-mac-v1
    Intel Mac 类型 1
intel-mac-v2
    Intel Mac 类型 2
intel-mac-v3
    Intel Mac 类型 3
intel-mac-v4
    Intel Mac 类型 4
intel-mac-v5
    Intel Mac 类型 5
intel-mac-auto
    Intel Mac（根据子系统 id 检测类型）
macmini
    Intel Mac Mini（等价于类型 3macbook
    Intel Mac Book（等价于类型 5macbook-pro-v1
    Intel Mac Book Pro 第一代（等价于类3macbook-pro
    Intel Mac Book Pro 第二代（等价于类3imac-intel
    Intel iMac（等价于类型 2imac-intel-20
    Intel iMac（较新版本）（等价于类型 3ecs202
    ECS/PC chips
dell-d81
    Dell（未知）
dell-d82
    Dell（未知）
dell-m81
    Dell（未知）
dell-m82
    Dell XPS M1210
auto
    BIOS 设置（默认）

## STAC9202/9250/9251

ref
    参考板，基础配置
m1
    部分 Gateway MX 系列笔记本（NX560XLm1-2
    部分 Gateway MX 系列笔记本（MX6453m2
    部分 Gateway MX 系列笔记本（M255m2-2
    部分 Gateway MX 系列笔记m3
    部分 Gateway MX 系列笔记m5
    部分 Gateway MX 系列笔记本（MP6954m6
    部分 Gateway NX 系列笔记auto
    BIOS 设置（默认）

## STAC9227/9228/9229/927x

ref
    参考板
ref-no-jd
    无耳机/麦克风插孔检测的参考板
3stack
    D965 3 鏍?5stack
    D965 5 鏍?+ SPDIF
5stack-no-fp
    无前面板D965 5 dell-3stack
    Dell Dimension E520
dell-bios
    配合 Dell BIOS 设置的修dell-bios-amic
    配合含模拟麦克风Dell BIOS 设置的修volknob
    配合音量旋钮组件 0x24 的修auto
    BIOS 设置（默认）

## STAC92HD71B*

ref
    参考板
dell-m4-1
    Dell 鍙板紡鏈?dell-m4-2
    Dell 鍙板紡鏈?dell-m4-3
    Dell 鍙板紡鏈?hp-m4
    HP mini 1000
hp-dv5
    HP dv 系列
hp-hdx
    HP HDX 系列
hp-dv4-1222nr
    HP dv4-1222nr（带 LED 支持auto
    BIOS 设置（默认）

## STAC92HD73*

ref
    参考板
no-jd
    BIOS 设置但不含插孔检intel
    Intel D**45** 主板
dell-m6-amic
    带模拟麦克风Dell 台式笔记dell-m6-dmic
    带数字麦克风Dell 台式笔记dell-m6
    带两种类型麦克风Dell 台式笔记dell-eq
    Dell 台式笔记alienware
    Alienware M17x
asus-mobo
    5.1/SPDIF 输出ASUS 主板引脚配置
auto
    BIOS 设置（默认）

## STAC92HD83*

ref
    参考板
mic-ref
    带端口电源管理的参考板
dell-s14
    Dell 笔记dell-vostro-3500
    Dell Vostro 3500 笔记hp-dv7-4000
    HP dv-7 4000
hp_cNB11_intquad
    4 个扬声器HP CNB 型号
hp-zephyr
    HP Zephyr
hp-led
    BIOS 损坏导致静音 LED 异常HP
hp-inv-led
    BIOS 损坏导致反相静音 LED 异常HP
hp-mic-led
    带麦克风静音 LED HP
headset-jack
    4 针头戴插孔的 Dell Latitude
hp-envy-bass
    HP Envy 低音扬声器引脚修复（NID 0x0fhp-envy-ts-bass
    HP Envy TS 低音扬声器引脚修复（NID 0x10hp-bnb13-eq
    HP 笔记本的硬件均衡器设hp-envy-ts-bass
    HP Envy TS 低音支持
auto
    BIOS 设置（默认）

## STAC92HD95

hp-led
    HP 笔记本的 LED 支持
hp-bass
    HP Spectre 13 的低HPF 设置

## STAC9872

vaio
    SPDIF VAIO 笔记auto
    BIOS 设置（默认）

## Cirrus Logic CS4206/4207

mbp53
    MacBook Pro 5,3
mbp55
    MacBook Pro 5,5
imac27
    iMac 27 英寸
imac27_122
    iMac 12,2
apple
    通用 Apple 修正
mbp101
    MacBookPro 10,1
mbp81
    MacBookPro 8,1
mba42
    MacBookAir 4,2
auto
    BIOS 设置（默认）

## Cirrus Logic CS4208

mba6
    MacBook Air 6,1 鍜?6,2
gpio0
    启用 GPIO 0 放大mbp11
    MacBookPro 11,2
macmini
    MacMini 7,1
auto
    BIOS 设置（默认）

## VIA VT17xx/VT18xx/VT20xx

auto
    BIOS 设置（默认）
