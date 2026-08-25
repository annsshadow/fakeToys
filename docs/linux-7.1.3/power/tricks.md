## swsusp/S3 技


Pavel Machek <pavel@ucw.cz>

如果你想哄骗 swsusp/S3 让它工作，你可以尝试

- 使用最小配置，关闭你并不真正需要的 USB、AGP 等驱

- 关闭 APIC 与抢占（preempt

- 使用 ext2。至少它fsck 是可用的。[如果似乎出了问题，在有机会时强制进行 fsck]

- 关闭模块

- 使用 VGA 文本控制台，关闭 X。[如果你确实想X，可以稍后尝vesafb]

- 尽量运行尽可能少的进程，最好进入单用户模式

- 由于视频问题，swsusp 应该S3 更容易使其工作。先尝试那个

当你让它工作后，试着找出究竟是什么破坏了挂起（suspend），并最好修复它
