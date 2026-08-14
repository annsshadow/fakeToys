
## 用于 Linux 的电子邮件客户端信息（Email clients info for Linux）

### Git

如今大多数开发者使用 `git send-email` 而非常规的电子邮件客户端。它的手册页写得相当好。在接收端，
维护者使用 `git am` 来应用补丁。

如果你刚接触 `git`，那么先把你的第一个补丁发送给你自己。把它保存为包含全部头部的原始文本。运行
`git am raw_email.txt`，然后用 `git log` 查看变更日志。当这能正常工作后，再把补丁发送到相应的邮件
列表。

### 一般偏好（General Preferences）

Linux 内核的补丁通过电子邮件提交，最好是作为邮件正文的行内文本。一些维护者接受附件，但附件应当具有
`text/plain` 内容类型。然而，附件通常不招人待见，因为它会让补丁评审过程中引用补丁片段变得更困难。

还强烈建议你在邮件正文中使用纯文本，无论是对补丁还是其它邮件都是如此。https://useplaintext.email 可能
有助于你了解如何配置你偏好的邮件客户端，以及在你还没有偏好时列出推荐的邮件客户端。

用于 Linux 内核补丁的邮件客户端应当原封不动地发送补丁文本。例如，它们不应修改或删除制表符或空格，
即便在行首或行尾也不行。

不要发送带有 `format=flowed` 的补丁。这可能导致意外且不想要的换行。

不要让你的邮件客户端为你做自动单词换行。这也可能损坏你的补丁。

邮件客户端不应修改文本的字符集编码。通过邮件发送的补丁应当只使用 ASCII 或 UTF-8 编码。如果你把邮件
客户端配置为以 UTF-8 编码发送邮件，就可以避免一些潜在的字符集问题。

邮件客户端应当生成并维护 "References:" 或 "In-Reply-To:" 头部，以免邮件线程被打断。

复制粘贴（或剪切粘贴）通常对补丁不起作用，因为制表符会被转换为空格。使用 xclipboard、xclip 和/或
xcutsel 也许可以，但最好自己测试一下，或者直接避免复制粘贴。

不要在包含补丁的邮件中使用 PGP/GPG 签名。这会破坏许多读取和应用补丁的脚本。（这应该是可以修复的。）

在把补丁发送到 Linux 邮件列表之前，先给自己发一份补丁、保存收到的邮件，并用 'patch' 成功应用它，是个
好主意。

### 一些邮件客户端（MUA）提示（Some email client (MUA) hints）

以下是一些针对编辑和发送 Linux 内核补丁的特定 MUA 配置提示。这些并非完整的软件包配置总结。

图例（Legend）：

- TUI = 基于文本的用户界面（text-based user interface）
- GUI = 图形用户界面（graphical user interface）

######## Alpine (TUI)

配置选项：

在 `Sending Preferences` 部分：

- `Do Not Send Flowed Text` 必须 `enabled`（启用）
- `Strip Whitespace Before Sending` 必须 `disabled`（禁用）

在撰写邮件时，光标应放在补丁应出现的位置，然后按 `CTRL-R` 让你指定要插入到邮件中的补丁文件。

######## Claws Mail (GUI)

可以工作。有些人成功地把这个用于补丁。

要插入补丁，使用 `Message-->Insert File`（`CTRL-I`）或外部编辑器。

如果插入的补丁需要在 Claws 的撰写窗口中编辑，`Configuration-->Preferences-->Compose-->Wrapping`
中的 "Auto wrapping" 应当禁用。

######## Evolution (GUI)

有些人成功地把这个用于补丁。

撰写邮件时选择：Preformat
  从 `Format-->Paragraph Style-->Preformatted`（`CTRL-7`）或工具栏中选择

然后使用：
`Insert-->Text File...`（`ALT-N x`）来插入补丁。

你也可以 `diff -Nru old.c new.c | xclip`，选择 `Preformat`，然后用中键粘贴。

######## Kmail (GUI)

有些人成功地把 Kmail 用于补丁。

默认不撰写 HTML 的设置是合适的；不要启用它。

撰写邮件时，在选项下，取消勾选 "word wrap"。唯一的缺点是你输入的任何文本都不会自动换行，因此你必须在
补丁之前手动对文本进行换行。最简单的绕过办法是：先以启用 word wrap 的方式撰写邮件，然后将其保存为草稿。
一旦你再次从草稿中把它打开，它现在就已被硬性换行，你可以在不丢失现有换行的情况下取消勾选 "word wrap"。

在邮件底部，在插入补丁之前放入常用的补丁分隔符：三个连字符（`---`）。

然后从 `Message` 菜单项中选择 `insert file` 并选择你的补丁。作为额外的好处，你可以自定义消息创建工具栏
菜单，并把 `insert file` 图标放在那里。

把撰写窗口拉得足够宽，以使没有任何行被换行。截至 KMail 1.13.5（KDE 4.5.4），如果行在撰写窗口中发生换行，
KMail 会在发送邮件时应用单词换行。仅在选项菜单中禁用 word wrap 还不够。因此，如果你的补丁有很长的行，
你必须在发送邮件之前把撰写窗口拉得非常宽。参见：https://bugs.kde.org/show_bug.cgi?id=174034

你可以放心地对附件进行 GPG 签名，但补丁更偏好行内文本，因此不要对它们进行 GPG 签名。对以行内文本插入的
补丁进行签名会让它们难以从其 7-bit 编码中提取出来。

如果你绝对必须以附件而非行内文本的方式发送补丁，请右键单击附件并选择 `properties`，然后高亮
`Suggest automatic display`，以使附件被行内化从而更易查看。

当保存以行内文本发送的补丁时，从消息列表窗格中选择包含补丁的邮件，右键单击并选择 `save as`。如果它被正确
撰写，你可以把整封未修改的邮件用作补丁。邮件以仅用户可读写的方式保存，因此如果你把它们复制到别处，必须
用 chmod 让它们对组和其他人可读。

######## Lotus Notes (GUI)

离它远点。

######## IBM Verse (Web GUI)

参见 Lotus Notes。

######## Mutt (TUI)

大量 Linux 开发者使用 `mutt`，因此它必定工作得相当好。

Mutt 本身不带编辑器，因此无论你使用什么编辑器，都应当以不产生自动换行的方式来使用。大多数编辑器都有
一个 `insert file` 选项，可以原封不动地插入文件内容。
```

  set editor="vi"

```
```

  :set paste

```
```

  :r filename

```
如果你想把补丁行内包含进来。(a)ttach 在没有 `set paste` 的情况下也能正常工作。

你也可以用 `git format-patch` 生成补丁，然后使用 Mutt
```

    $ mutt -H 0001-some-bug-fix.patch

```
配置选项：

它应当使用默认设置就能工作。
```

  set send_charset="us-ascii:utf-8"

```
Mutt 高度可定制。以下是一个最小配置，可供起步：
```

  # .muttrc
  # ================  IMAP  ====================
  set imap_user = 'yourusername@gmail.com'
  set imap_pass = 'yourpassword'
  set spoolfile = imaps://imap.gmail.com/INBOX
  set folder = imaps://imap.gmail.com/
  set record="imaps://imap.gmail.com/[Gmail]/Sent Mail"
  set postponed="imaps://imap.gmail.com/[Gmail]/Drafts"
  set mbox="imaps://imap.gmail.com/[Gmail]/All Mail"

  # ================  SMTP  ====================
  set smtp_url = "smtp://username@smtp.gmail.com:587/"
  set smtp_pass = $imap_pass
  set ssl_force_tls = yes # Require encrypted connection

  # ================  Composition  ====================
  set editor = `echo \$EDITOR`
  set edit_headers = yes  # See the headers when editing
  set charset = UTF-8     # value of $LANG; also fallback for send_charset
  # Sender, email address, and sign-off line must match
  unset use_domain        # because joe@localhost is just embarrassing
  set realname = "YOUR NAME"
  set from = "username@gmail.com"
  set use_from = yes

```
Mutt 文档中有更多信息：

    https://gitlab.com/muttmua/mutt/-/wikis/UseCases/Gmail

    http://www.mutt.org/doc/manual/

######## Pine (TUI)

Pine 在过去有一些空白截断问题，但现在这些应该都已被修复。

如果可以，使用 alpine（pine 的后继者）。

配置选项：

- 较新版本需要 `quell-flowed-text`
- 需要 `no-strip-whitespace-before-send` 选项

######## Sylpheed (GUI)

- 对行内文本（或使用附件）工作良好。
- 允许使用外部编辑器。
- 在大文件夹上较慢。
- 不会在非 SSL 连接上做 TLS SMTP 认证。
- 在撰写窗口中有一个有帮助的标尺栏。
- 把地址添加到地址簿时不能正确识别显示名。

######## Thunderbird (GUI)

Thunderbird 是一个喜欢破坏文本的 Outlook 克隆，但有办法强迫它规矩行事。

在完成这些修改（包括安装扩展）之后，你需要重启 Thunderbird。

- 允许使用外部编辑器：

  使用 Thunderbird 和补丁时，最简单的事是使用打开你喜欢的外部编辑器的扩展。

  以下是一些能够做到这一点的示例扩展。

  - "External Editor Revived"

    https://github.com/Frederick888/external-editor-revived

    https://addons.thunderbird.net/en-GB/thunderbird/addon/external-editor-revived/

    它要求安装一个 "native messaging host"。请阅读可以在此处找到的 wiki：
    https://github.com/Frederick888/external-editor-revived/wiki

  - "External Editor"

    https://github.com/exteditor/exteditor

    为此，下载并安装该扩展，然后打开 `compose` 窗口，使用 `View-->Toolbars-->Customize...`
    为它添加一个按钮，然后当你想使用外部编辑器时只需点击新按钮。

    请注意，"External Editor" 要求你的编辑器不能 fork，换句话说，编辑器必须在关闭之前不返回。你可能
    需要传递额外的标志或更改你编辑器的设置。最值得注意的是，如果你在使用 gvim，那么必须把 `/usr/bin/gvim --nofork`
    （如果该二进制位于 `/usr/bin`）传入 external editor 设置的文本编辑器字段。如果你在使用其它编辑器，请阅读
    它的手册以了解如何做到这一点。

要把内部编辑器整治得像样一些，请这样做：

- 编辑你的 Thunderbird 配置设置，使其不使用 `format=flowed`！转到主窗口，找到你的主下拉菜单按钮。
  `Main Menu-->Preferences-->General-->Config Editor...` 以打开 Thunderbird 的注册表编辑器。

  - 把 `mailnews.send_plaintext_flowed` 设为 `false`

  - 把 `mailnews.wraplength` 从 `72` 改为 `0` **或**安装 "Toggle Line Wrap" 扩展

    https://github.com/jan-kiszka/togglelinewrap

    https://addons.thunderbird.net/thunderbird/addon/toggle-line-wrap

    以在运行时控制该注册表。

- 不要写 HTML 邮件！转到主窗口 `Main Menu-->Account Settings-->youracc@server.something-->Composition & Addressing`！
  在那里你可以禁用 "Compose messages in HTML format" 选项。

- 只以纯文本打开邮件！转到主窗口 `Main Menu-->View-->Message Body As-->Plain Text`！

######## TkRat (GUI)

可以工作。使用 "Insert file..." 或外部编辑器。

######## Gmail (Web GUI)

不能用于发送补丁。

Gmail 网页客户端会自动把制表符转换为空格。

同时它会以 CRLF 风格的换行每 78 个字符折行，尽管 tab2space 问题可以通过外部编辑器解决。

另一个问题是 Gmail 会对任何含有非 ASCII 字符的邮件进行 base64 编码。这包括诸如欧洲人名之类的内容。

######## HacKerMaiL (TUI)

HacKerMaiL（hkml）是一个基于 public-inbox 的简单邮件管理工具，不需要订阅邮件列表。它由 DAMON 维护者
开发和维护，旨在支持 DAMON 和通用内核子系统的简单开发工作流。详情请参阅 README
（https://github.com/sjp38/hackermail/blob/master/README.md）。
