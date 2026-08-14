### 清除 WARN_ONCE


WARN_ONCE / WARN_ON_ONCE / printk_once 只输出一次消息。

echo 1 > /sys/kernel/debug/clear_warn_once

清除状态并允许警告再次打印一次。这在测试套件运行后用于复现问题时可能很有用。
