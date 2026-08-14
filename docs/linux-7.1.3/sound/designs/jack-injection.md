## ALSA Jack 软件注入


## Jack 注入简介


这里的 jack 注入是指用户可以通过 debugfs 接口向音频 jack 注入插入（plugin）或拔出（plugout）
事件，这有助于验证 ALSA 用户空间的改动。例如，我们修改了 pulseaudio 中的音频配置文件切换代码，
我们想验证该改动是否如预期工作、是否引入了回归。这种情况下，我们可以向一个或某些音频 jack
注入插入或拔出事件，而无需实际接触机器并将物理设备插拔到音频 jack 上。

在这个设计中，一个音频 jack 并不等同于一个物理音频 jack。有时一个物理音频 jack 包含多个功能，
ALSA 驱动会为一个 `snd_jack` 创建多个 `jack_kctl`，这里 `snd_jack` 代表一个物理音频 jack，
而 `jack_kctl` 代表一个功能，例如一个物理 jack 有两个功能：headphone 和 mic_in，ALSA ASoC
驱动会为此 jack 构建 2 个 `jack_kctl`。jack 注入是基于 `jack_kctl` 而非 `snd_jack` 实现的。

要向音频 jack 注入事件，我们需要先通过 `sw_inject_enable` 启用 jack 注入，一旦启用，该 jack
将不再因硬件事件而改变状态，我们可以通过 `jackin_inject` 注入插入或拔出事件，并通过 `status`
检查 jack 状态，测试完成后我们也需要通过 `sw_inject_enable` 禁用 jack 注入，一旦禁用，jack
状态将根据最后一次报告的硬件事件恢复，并将随未来的硬件事件而改变。

## Jack 注入接口的布局


如果用户在内核中启用了 SND_JACK_INJECTION_DEBUG，音频 jack 注入接口将如下创建：
```

   $debugfs_mount_dir/sound
   |-- card0
   |-- |-- HDMI_DP_pcm_10_Jack
   |-- |-- |-- jackin_inject
   |-- |-- |-- kctl_id
   |-- |-- |-- mask_bits
   |-- |-- |-- status
   |-- |-- |-- sw_inject_enable
   |-- |-- |-- type
   ...
   |-- |-- HDMI_DP_pcm_9_Jack
   |--     |-- jackin_inject
   |--     |-- kctl_id
   |--     |-- mask_bits
   |--     |-- status
   |--     |-- sw_inject_enable
   |--     |-- type
   |-- card1
       |-- HDMI_DP_pcm_5_Jack
       |-- |-- jackin_inject
       |-- |-- kctl_id
       |-- |-- mask_bits
       |-- |-- status
       |-- |-- sw_inject_enable
       |-- |-- type
       ...
       |-- Headphone_Jack
       |-- |-- jackin_inject
       |-- |-- kctl_id
       |-- |-- mask_bits
       |-- |-- status
       |-- |-- sw_inject_enable
       |-- |-- type
       |-- Headset_Mic_Jack
           |-- jackin_inject
           |-- kctl_id
           |-- mask_bits
           |-- status
           |-- sw_inject_enable
           |-- type

```
## 各节点的解释


kctl_id
  read-only，获取 jack_kctl->kctl 的 id
```

     sound/card1/Headphone_Jack# cat kctl_id
     Headphone Jack

```
mask_bits
  read-only，获取 jack_kctl 支持的 events mask_bits
```

     sound/card1/Headphone_Jack# cat mask_bits
     0x0001 HEADPHONE(0x0001)

```
status
  read-only，获取 jack_kctl 的当前状态

- 耳机未插入：

```

     sound/card1/Headphone_Jack# cat status
     Unplugged

```
- 耳机已插入：

```

     sound/card1/Headphone_Jack# cat status
     Plugged

```
type
  read-only，从 type 获取 snd_jack 支持的 events（物理音频 jack 上所有支持的 events）
```

     sound/card1/Headphone_Jack# cat type
     0x7803 HEADPHONE(0x0001) MICROPHONE(0x0002) BTN_3(0x0800) BTN_2(0x1000) BTN_1(0x2000) BTN_0(0x4000)

```
sw_inject_enable
  read-write，启用或禁用注入

- 注入已禁用：

```

     sound/card1/Headphone_Jack# cat sw_inject_enable
     Jack: Headphone Jack		Inject Enabled: 0

```
- 注入已启用：

```

     sound/card1/Headphone_Jack# cat sw_inject_enable
     Jack: Headphone Jack		Inject Enabled: 1

```
- 启用 jack 注入：

```

     sound/card1/Headphone_Jack# echo 1 > sw_inject_enable

```
- 禁用 jack 注入：

```

     sound/card1/Headphone_Jack# echo 0 > sw_inject_enable

```
jackin_inject
  write-only，注入插入或拔出

- 注入插入：

```

     sound/card1/Headphone_Jack# echo 1 > jackin_inject

```
- 注入拔出：

```

     sound/card1/Headphone_Jack# echo 0 > jackin_inject

```
