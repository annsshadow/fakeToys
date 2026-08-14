## HD-Audio DP-MST 支持


为了支持 DP MST 音频，HD Audio hdmi 编解码器驱动引入了虚拟引脚（virtual pin）和动态 pcm 分配。

虚拟引脚是 per_pin 的扩展。DP MST 与传统的根本区别在于 DP MST 引入了设备条目（device entry）。每个引脚可以包含多个设备条目。每个设备条目的行为就像一个引脚。

由于每个引脚可能包含多个设备条目，而每个编解码器可能包含多个引脚，如果我们对每个 per_pin 使用一个 pcm，就会产生大量 PCM。新的方案是创建少量 PCM，并动态地将 pcm 绑定到 per_pin。驱动使用 spec->dyn_pcm_assign 标志来指示是否使用新方案。

## PCM

待补充

## 引脚初始化


每个引脚可能有多个设备条目（虚拟引脚）。在 Intel 平台上，设备条目数量是动态变化的。如果连接了 DP MST hub，则处于 DP MST 模式，设备条目数量为 3。否则，设备条目数量为 1。

为了简化实现，无论是否处于 DP MST 模式，所有设备条目都会在启动时初始化。

## 连接列表


DP MST 复用了连接列表代码。代码可以复用是因为同一引脚上的设备条目具有相同的连接列表。

这意味着 DP MST 无需设备条目设置即可获取设备条目的连接列表。

## 插孔（Jack）


假设：
 - MST 必须是 dyn_pcm_assign，且它是 acomp（针对 Intel 场景）；
 - NON-MST 可能是也可能不是 dyn_pcm_assign，它可以是 acomp 或 !acomp；

因此存在以下场景：
 a. MST（&& dyn_pcm_assign && acomp）
 b. NON-MST && dyn_pcm_assign && acomp
 c. NON-MST && !dyn_pcm_assign && !acomp

下面的讨论将忽略 MST 和 NON-MST 的区别，因为它对插孔处理影响不大。

驱动在 hdmi_spec 中使用 struct hdmi_pcm pcm[] 数组，snd_jack 是 hdmi_pcm 的一个成员。每个引脚有一个 struct hdmi_pcm * pcm 指针。

对于 !dyn_pcm_assign，per_pin->pcm 会在初始化时静态地分配到 spec->pcm[n]。

对于 dyn_pcm_assign，per_pin->pcm 会在显示器热插拔时分配到 spec->pcm[n]。


### 构建插孔


- dyn_pcm_assign

  不使用 hda_jack，而是直接使用 spec->pcm_rec[pcm_idx].jack 中的 snd_jack。

- !dyn_pcm_assign

  使用 hda_jack，并静态地将 spec->pcm_rec[pcm_idx].jack = jack->jack。


### 开启非请求事件


如果不是 acomp，则开启非请求事件（unsolicited event）。


### 显示器热插拔事件处理


- acomp

  pin_eld_notify() -> check_presence_and_report() -> hdmi_present_sense() ->
  sync_eld_via_acomp()。

  无论是 dyn_pcm_assign 还是 !dyn_pcm_assign，都直接在 spec->pcm_rec[pcm_idx].jack 上调用 snd_jack_report()

- !acomp

  hdmi_unsol_event() -> hdmi_intrinsic_event() -> check_presence_and_report() ->
  hdmi_present_sense() -> hdmi_prepsent_sense_via_verbs()

  对于 dyn_pcm_assign，直接在 spec->pcm_rec[pcm_idx].jack 上调用 snd_jack_report()。
  使用 hda_jack 机制来处理插孔事件。


## 其他待后续补充
