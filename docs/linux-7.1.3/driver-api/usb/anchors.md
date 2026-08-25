#### USB Anchors


## 什么是 anchor

USB 驱动需要支持一些回调，这些回调要求驱动停止对某个接口的所IO。为此，驱动必须记录它已提交URB，以确认它们全部完成，或对其调用 usb_kill_urb。anchor 是一种数据结构，负责记录 URB 并提供处理多URB 的方法
## 分配与初始化


没有用于分配 anchor API。它只是被声明为 struct usb_anchor。必须调`init_usb_anchor` 来初始化该数据结构
## 释放


一anchor 不再关联任何 URB，就可以使用普通的内存管理操作将其释放
## URB anchor 的关联与解除关联


通过URB 显式调用 `usb_anchor_urb` 来建URB anchor 的关联。该关联会一直保持，直到 URB 通过（成功）完成而结束。因此解除关联是自动的。还提供了一个函数用于强制结束（kill）与某个 anchor 关联的所URB此外，也可以通过 `usb_unanchor_urb` 进行解除关联
## 对多URB 的操

### :c:func:`usb_kill_anchored_urbs`


该函数杀死与某个 anchor 关联的所URB。URB 会按照其提交的时间逆序被调用。这样可以保证数据不会被重排序
### :c:func:`usb_scuttle_anchored_urbs`


某个 anchor 的所URB 会被一次性全部解除锚定
### :c:func:`usb_wait_anchor_empty_timeout`


该函数等待与某个 anchor 关联的所URB 完成或超时，以先发生者为准。其返回值会告诉你是否达到了超时
### :c:func:`usb_anchor_empty`


如果没有 URB anchor 关联，则返回 true。加锁由调用者负责
### :c:func:`usb_get_from_anchor`


返回某个 anchor 中最旧的锚定 URB。该 URB 会被解除锚定并带引用返回。由于你可以在一anchor 中混合指向多个目标的 URB，因此无法保证返回的是按时间顺序最先提交的 URB