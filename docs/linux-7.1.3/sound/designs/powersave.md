## 关于省电模式的说

AC97 HD-audio 驱动具有自动省电模式该特性分别通过 Kconfig 选项 `CONFIG_SND_AC97_POWER_SAVE`
`CONFIG_SND_HDA_POWER_SAVE` 启用
通过自动省电，当不需要进行任何操作时，驱动会适当地关闭编解码器（codec）电源当没有应用程序使用设备且/或没有设置模拟回环（analog loopback）时，会完全或部分地禁用电源这将节省一定的功耗，因此对笔记本电脑（甚至台式机）都有好处
自动断电的超时时间可以通过 snd-ac97-codec snd-hda-intel 模块`power_save`
模块选项指定。以秒为单位指定超时值 表示禁用自动省电超时的默认值由 `CONFIG_SND_AC97_POWER_SAVE_DEFAULT` `CONFIG_SND_HDA_POWER_SAVE_DEFAULT` Kconfig 选项给出。将此值设1
（最小值）并不推荐，因为许多应用程序会频繁重新打开设备0 对于正常操作来说是一个不错的选择
`power_save` 选项被导出为可写。这意味着你可以随时通过 sysfs 调整该值。例如，要开10 秒超时的自动省电模式，向
`/sys/module/snd_ac97_codec/parameters/power_save` 写入（通常作为 root）：
```

	# echo 10 > /sys/module/snd_ac97_codec/parameters/power_save


```
注意，在改变电源状态时你可能会听到咔哒爆音（click noise/pop）。此外，从掉电状唤醒到活动状态通常也需要一定时间。这些往往很难修复，所以除非你有修复补丁，否则不要
提交额外bug 报告 ;-)

对于 HD-audio 接口，还有另一个模块选项 power_save_controller。它启用/禁用
控制器侧的省电模式。开启它可能进一步减少一些功耗，但可能导致更长的唤醒时间咔哒声。如果你经常遇到这种情况，请尝试将其关闭