## ALSA Jack 控件


## 我们为什么需Jack kcontrol


ALSA 使用 kcontrol 向用户空间导出音频控件（开关、音量、多路复用器等）。这意味着pulseaudio 这样的用户空间应用程序可以在没有插入耳机时关闭耳机并打开扬声器

旧的 ALSA jack 代码仅为每个注册jack 创建输入设备。这jack 输入设备无法被以root 身份运行的用户空间设备读取

新的 jack 代码为每jack 创建嵌入jack kcontrol，任何进程都可以读取

这可以结UCM，使用户空间能够根据 jack 插入或拔出事件更智能地路由音频

## Jack Kcontrol 内部机制


每个 jack 都会有一kcontrol 列表，以便我们可以在 jack 创建阶段创建一kcontrol 并将其附加到 jack 上。我们也可以在任何需要的时候向已有jack 添加 kcontrol

Jack 被释放时，这kcontrol 会被自动释放

## 如何使用 jack kcontrol


为了保持兼容性，snd_jack_new() 被修改，添加了两个参数：

initial_kctl
  如果true，则创建一kcontrol 并将其添加到 jack 列表
phantom_jack
  不为 phantom jack 创建输入设备

HDA jack 可以phantom_jack 设为 true 以创建一phantom jack，并initial_kctl 设为 true 以使用正确的 id 创建一个初kcontrol

ASoC jack 应将 initial_kctl 设为 false。引脚名称将被赋jack kcontrol 名称
