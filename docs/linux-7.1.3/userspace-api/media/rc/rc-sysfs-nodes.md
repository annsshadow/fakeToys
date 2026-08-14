
######## 遥控器的 sysfs 节点


如 Documentation/ABI/testing/sysfs-class-rc 中所定义，以下是控制遥控器的 sysfs 节点：



## /sys/class/rc/


`/sys/class/rc/` 类子目录属于遥控器（Remote Controller）核心，并提供一个用于配置红外
遥控器接收器的 sysfs 接口。



## /sys/class/rc/rcN/


为每一个遥控器接收器设备创建一个 `/sys/class/rc/rcN` 目录，其中 N 是接收器的编号。



## /sys/class/rc/rcN/protocols


```

	rc5 [rc6] nec jvc [sony]

```
已启用的协议显示在 [] 方括号中。

写入 “+proto” 会将该协议添加到已启用协议列表中。

写入 “-proto” 会从已启用协议列表中移除该协议。

写入 “proto” 将仅启用 “proto”。

写入 “none” 将禁用所有协议。

如果使用了无效的协议组合或未知的协议名，写入将以 `EINVAL` 失败。



## /sys/class/rc/rcN/filter


设置扫描码（scancode）过滤器的期望值。

与 `/sys/class/rc/rcN/filter_mask` 结合使用，以设置 filter mask 中所设位的期望值。如果
硬件支持，则不匹配该过滤器的扫描码将被忽略。否则写入将以错误失败。

如果当前协议被更改，该值可能被重置为 0。



## /sys/class/rc/rcN/filter_mask


设置用于比较的扫描码过滤器掩码位。与 `/sys/class/rc/rcN/filter` 结合使用，以设置扫描码中
应与期望值进行比较的位。值为 0 会禁用该过滤器，以允许处理所有有效的扫描码。

如果硬件支持，则不匹配该过滤器的扫描码将被忽略。否则写入将以错误失败。

如果当前协议被更改，该值可能被重置为 0。



## /sys/class/rc/rcN/wakeup_protocols


读取该文件会返回可用于唤醒事件的可用协议列表：
```

	rc-5 nec nec-x rc-6-0 rc-6-6a-24 [rc-6-6a-32] rc-6-mce

```
注意，此处列出了协议变体，因此 `nec`、`sony`、`rc-5`、`rc-6` 各自不同的位长度编码会被列出
（如果可用）。

注意，所有协议变体都会被列出。

已启用的唤醒协议显示在 [] 方括号中。

一次只能选择一个协议。

写入 “proto” 将使用 “proto” 作为唤醒事件。

写入 “none” 将禁用唤醒。

如果使用了无效的协议组合或未知的协议名，或硬件不支持唤醒，写入将以 `EINVAL` 失败。



## /sys/class/rc/rcN/wakeup_filter


设置扫描码唤醒过滤器的期望值。与 `/sys/class/rc/rcN/wakeup_filter_mask` 结合使用，以设置
wakeup filter mask 中所设位的期望值，用于触发系统唤醒事件。

如果硬件支持且 wakeup_filter_mask 不为 0，则匹配该过滤器的扫描码将唤醒系统，例如从挂起到
RAM（suspend to RAM）或断电（power off）中唤醒。否则写入将以错误失败。

如果唤醒协议被更改，该值可能被重置为 0。



## /sys/class/rc/rcN/wakeup_filter_mask


设置用于比较的扫描码唤醒过滤器掩码位。与 `/sys/class/rc/rcN/wakeup_filter` 结合使用，以
设置扫描码中应与期望值进行比较的位，用于触发系统唤醒事件。

如果硬件支持且 wakeup_filter_mask 不为 0，则匹配该过滤器的扫描码将唤醒系统，例如从挂起到
RAM 或断电中唤醒。否则写入将以错误失败。

如果唤醒协议被更改，该值可能被重置为 0。
