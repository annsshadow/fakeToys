## ALSA Jack 控件


## 我们为什么需要 Jack kcontrol


ALSA 使用 kcontrol 向用户空间导出音频控件（开关、音量、多路复用器等）。这意味着像 pulseaudio 这样的用户空间应用程序可以在没有插入耳机时关闭耳机并打开扬声器。

旧的 ALSA jack 代码仅为每个注册的 jack 创建输入设备。这些 jack 输入设备无法被以非 root 身份运行的用户空间设备读取。

新的 jack 代码为每个 jack 创建嵌入式 jack kcontrol，任何进程都可以读取。

这可以结合 UCM，使用户空间能够根据 jack 插入或拔出事件更智能地路由音频。

## Jack Kcontrol 内部机制


每个 jack 都会有一个 kcontrol 列表，以便我们可以在 jack 创建阶段创建一个 kcontrol 并将其附加到 jack 上。我们也可以在任何需要的时候向已有的 jack 添加 kcontrol。

当 Jack 被释放时，这些 kcontrol 会被自动释放。

## 如何使用 jack kcontrol


为了保持兼容性，snd_jack_new() 被修改，添加了两个参数：

initial_kctl
  如果为 true，则创建一个 kcontrol 并将其添加到 jack 列表。
phantom_jack
  不为 phantom jack 创建输入设备。

HDA jack 可以将 phantom_jack 设为 true 以创建一个 phantom jack，并将 initial_kctl 设为 true 以使用正确的 id 创建一个初始 kcontrol。

ASoC jack 应将 initial_kctl 设为 false。引脚名称将被赋为 jack kcontrol 名称。
