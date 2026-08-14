## 键盘通知器


可以使用 register_keyboard_notifier 在键盘事件发生时获得回调
（详见 kbd_keycode() 函数）。传入的结构体为 keyboard_notifier_param
（参见 <linux/keyboard.h>）：

- 'vc' 始终提供该键盘事件所适用的虚拟控制台（VC）；
- 'down' 对于按键事件为 1，对于松开事件为 0；
- 'shift' 为当前修饰键状态，掩码位索引为 KG_*；
- 'ledstate' 为当前 LED 状态；
- 'value' 取决于事件类型。

- KBD_KEYCODE 事件总是在其他事件之前发送，value 为键码。
- KBD_UNBOUND_KEYCODE 事件在键码未绑定到某个 keysym 时发送。
  value 为键码。
- KBD_UNICODE 事件在 键码 -> keysym 转换产生一个
  unicode 字符时发送。value 为该 unicode 值。
- KBD_KEYSYM 事件在 键码 -> keysym 转换产生一个
  非 unicode 字符时发送。value 为该 keysym。
- KBD_POST_KEYSYM 事件在处理完非 unicode keysym 之后发送。
  这允许例如检查最终得到的 LED。

对于除最后一种外的每种事件，回调可以返回 NOTIFY_STOP 以“吃掉”该事件：
通知循环被停止，键盘事件被丢弃。

```

    kbd_keycode(keycode) {
	...
	params.value = keycode;
	if (notifier_call_chain(KBD_KEYCODE,&params) == NOTIFY_STOP)
	    || !bound) {
		notifier_call_chain(KBD_UNBOUND_KEYCODE,&params);
		return;
	}

	if (unicode) {
		param.value = unicode;
		if (notifier_call_chain(KBD_UNICODE,&params) == NOTIFY_STOP)
			return;
		emit unicode;
		return;
	}

	params.value = keysym;
	if (notifier_call_chain(KBD_KEYSYM,&params) == NOTIFY_STOP)
		return;
	apply keysym;
	notifier_call_chain(KBD_POST_KEYSYM,&params);
    }

```
