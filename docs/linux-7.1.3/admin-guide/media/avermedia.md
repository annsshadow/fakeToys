## AverMedia DVB-T（BT878）发布说

2006 骞?2 鏈?14 鏃。

目前已支持多AverMedia 设备。更完整、更及时的内容请查阅

https://linuxtv.org/wiki/index.php/AVerMedia

### AverMedia DVB-T

AverMedia DVB-T 是一款低成本PCI DVB 接收卡，提供三个输入接口

- RF 调谐器输入（RF Tuner Input
- 复合视频输入（Composite Video Input，RCA 接口
- S-Video 输入（Mini-DIN 接口

RF 调谐器输入连接至卡上的调谐器模块。该调谐器在代码中通常被称为“前端（Frontend）”。AverMedia DVB-T 所用前端为 Microtune 7202D。后linux-dvb 邮件列表确认，Microtune 7202D sp887x 驱动支持，该驱动可在 dvb-hw CVS 模块中找到

DVB-T 卡基BT878 芯片，BT878 是一种非常常见的多媒体桥接芯片，也常出现在模拟电视卡上。卡上不MPEG2 解码器，因此 MPEG2 解码需由软件完成；与之相对的是那些由芯片组完成 MPEG2 硬件解码的卡

### 让卡片工作起来（Getting the card going

现阶段已能确AverMedia DVBT 其余设备节点的功能。然而，涉及调谐、接收并提供 MPEG2 数据流的功能，目前仅在使用可用版本的驱动时方可实现。卡片上可用的其他功能（例如卡上提供的额外模拟输入）尚待测试。一旦发现相关内容，我会随时更新本文档

为卡片上电后，请按以下顺序加载内核模块：

- modprobe bttv（通常会自动加载）
- modprobe dvb-bt8xx（可dvb-bt8xx 放入 /etc/modules 或对应的模块目录

模块插入后，内核会激活相应的 DVB 设备节点，随后即可使scan、tzap、dvbstream 等工具访问该卡

前端模块 sp887x.o 需要外部固件。请使用命令 `get_dvb_firmware sp887x` 下载固件，并将其复制/usr/lib/hotplug/firmware /lib/firmware/（具体路径取决于固件 hotplug 的配置）

### 已知限制（Known Limitations

目前可以确认前端能够完成调谐，且 /dev/dvb/adapter{x}/frontend0 会向 /dev/dvb/adapter{x}/dvr0 提供 MPEG2 数据流。卡片的其余功能我尚未测试，有空时会更新本文档

限制主要来自 i2c 层返回的错误信息不一致。尽管这会在 dmesg 系统日志中产生错误，但似乎并不影响前端正常发挥作用

### 后续更新（Further updates

dvbstream VideoLAN Client（Windows 版）配合 DVB 使用效果很好，事实上这也是我目前观看 DVB-T 的主要方式。此外，VLC 也能顺利解码 HDTV 信号，尽PC 偶尔会丢几帧——我猜测这源于处理能力不足（解码Windows 下由软件完成）

非常感谢 Nigel Pearson 在驱动近期修订后更新了本文档
