## IOAM6 Sysfs 变量



## /proc/sys/net/conf/<iface>/ioam6_* 变量


ioam6_enabled - BOOL
        在该接口入口处接受（= 启用）或忽略 禁用）IPv6 IOAM 选项

        - 0 - 禁用（默认）
        - 1 - 启用

ioam6_id - SHORT INTEGER
        定义此接口的 IOAM id

        默认值为 ~0

ioam6_id_wide - INTEGER
        定义此接口的IOAM id

        默认值为 ~0
