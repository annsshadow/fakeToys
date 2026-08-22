## VGA 软光
by Pavel Machek <pavel@atrey.karlin.mff.cuni.cz>
and Martin Mares <mj@atrey.karlin.mff.cuni.cz>

Linux 现在具备一定操纵光标外观的能力。通常，你可以设置硬件光标的大小。现在你可以玩几个新花样：可以让光标看起来像一个不闪烁的红色方块，让它反显所在字符的背景，或者高亮该字符，并仍可选择原来的硬件光标是否保持可见。也许还有其他我从未想到的玩法
光标外观`<ESC>[1;2;3c` 转义序列控制，其1 是如下所述的参数。若省略其中任何一个，它们将默认为零
第一个参```

		0=default
		1=invisible
		2=underline,
		...
		8=full block
		+ 16 if you want the software cursor to be applied
		+ 32 if you want to always change the background color
		+ 64 if you dislike having the background the same as the
		     foreground.

	Highlights are ignored for the last two flags.

```
第二个参	选择你想要更改的字符属性位
	（只需用本参数的值对其进行异或即可）。在标准
	VGA 上，高四位指定背景色，低四位指定
	前景色。在两组中，低三位设置颜色（与控制台使用的普	颜色码相同），最高位开启高亮（有时是闪烁——这取决于你	VGA 的配置）
第三个参	由你想要设置的字符属性位组成
	位的设置发生在位翻转之前，因此你可以简单地通过将某一位置	设置掩码与翻转掩码二者之中来清除该位
### 示例


```

	echo -e '\033[?2c'

```
```

	echo -e '\033[?6c'

```
```

	echo -e '\033[?17;0;64c'

```
