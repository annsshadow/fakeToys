## 内核对杂项二进制格式的支持（binfmt_misc）


该内核特性允许你几乎（限制见下文）只需在 shell 中输入程序名即可调用任意程序。这包括例如编译后的 Java(TM)、Python 或 Emacs 程序。

为此，你必须告诉 binfmt_misc 哪个解释器应配合哪个二进制文件被调用。Binfmt_misc 通过将文件开头的若干字节与你提供的魔数字节序列（屏蔽掉指定的位）进行匹配来识别二进制类型。Binfmt_misc 还可以识别文件扩展名，例如 `.com` 或 `.exe`。

```

	mount binfmt_misc -t binfmt_misc /proc/sys/fs/binfmt_misc

```
要实际注册一个新的二进制类型，你必须构造一个形如 `:name:type:offset:magic:mask:interpreter:flags` 的字符串（其中的 `:` 可以根据需要选择），并将其 echo 到 `/proc/sys/fs/binfmt_misc/register`。

以下为各字段的含义：

- `name`
   是一个标识符字符串。将在 `/proc/sys/fs/binfmt_misc` 下以该名称创建一个新的 /proc 文件；出于显而易见的原因，不能包含斜杠 `/`。
- `type`
   是识别类型。魔数识别用 `M`，扩展名识别用 `E`。
- `offset`
   是文件中 magic/mask 的偏移量，以字节计。如果省略则默认为 0（即你写成 `:name:type::magic...`）。在使用文件名扩展名匹配时被忽略。
- `magic`
   是 binfmt_misc 要匹配的字节序列。魔数字符串可以包含十六进制编码的字符，如 `\x0a` 或 `\xA4`。注意你必须转义任何 NUL 字节；解析在遇到第一个 NUL 时停止。在 shell 环境中，你可能必须写成 `\\x0a` 以防止 shell 吃掉你的 `\`。如果选择了文件名扩展名匹配，则此处为要识别的扩展名（不含 `.`，不允许使用 `\x0a` 特殊形式）。扩展名匹配区分大小写，且不允许斜杠 `/`！
- `mask`
   是一个（可选，默认为全 0xff）掩码。你可以像 magic 一样提供一个与 magic 等长的字符串来屏蔽某些匹配位。该掩码会与文件的字节序列进行与运算。注意你必须转义任何 NUL 字节；解析在遇到第一个 NUL 时停止。在使用文件名扩展名匹配时被忽略。
- `interpreter`
   是要以二进制文件作为第一个参数来调用的程序（请指定完整路径）。
- `flags`
   是一个可选字段，控制解释器调用的若干方面。它是一个大写字母字符串，每个字母控制一个方面。支持以下标志：

      `P` - 保留 argv[^0^]
            传统行为是 binfmt_misc 会用二进制的完整路径覆盖原始的 argv[^0^]。包含此标志时，binfmt_misc 会为此向参数向量添加一个参数，从而保留原始的 `argv[^0^]`。例如，如果你的 interp 设为 `/bin/foo` 且你运行 `blah`（位于 `/usr/local/bin`），则内核将以 `argv[]` 设为 `["/bin/foo", "/usr/local/bin/blah", "blah"]` 来执行 `/bin/foo`。解释器必须意识到这一点，才能以 `argv[]` 设为 `["blah"]` 来执行 `/usr/local/bin/blah`。
      `O` - 打开二进制（open-binary）
	    传统行为是向解释器传递二进制的完整路径作为参数。包含此标志时，binfmt_misc 会打开该文件用于读取，并将其描述符（而非完整路径）作为参数传递，从而允许解释器执行不可读的二进制文件。应谨慎使用此特性——必须信任解释器不会泄露不可读二进制文件的内容。
      `C` - 凭证（credentials）
            当前，binfmt_misc 的行为是根据解释器来计算新进程的凭证和安全令牌。包含此标志时，这些属性将根据二进制文件计算。它也隐含了 `O` 标志。应谨慎使用此特性，因为当以 binfmt_misc 运行由 root 拥有的 setuid 二进制文件时，解释器将以 root 权限运行。
      `F` - 固定二进制（fix binary）
            通常 binfmt_misc 的行为是在调用 misc 格式文件时才惰性生成（spawn）二进制文件。然而，在挂载命名空间和 changeroots 面前这种方式表现不佳，因此 `F` 模式会在仿真安装完成后立即打开二进制文件，并使用打开的映像来生成模拟器，这意味着一旦安装就始终可用，与环境如何变化无关。


有一些限制：

 - 整个注册字符串不得超过 1920 个字符
 - magic 必须位于文件的前 128 字节内，即 offset+size(magic) 必须小于 128
 - 解释器字符串不得超过 127 个字符

要使用 binfmt_misc，你必须先挂载它。你可以使用 `mount -t binfmt_misc none /proc/sys/fs/binfmt_misc` 命令挂载它，或者向你的 `/etc/fstab` 添加一行 `none  /proc/sys/fs/binfmt_misc binfmt_misc defaults 0 0`，使其在启动时自动挂载。

你可能希望在启动期间在你的某个 `/etc/rc` 脚本中添加二进制格式。请阅读你的 init 程序的手册以了解正确的做法。

注意添加条目的顺序！后添加的条目会先被匹配！


以下是一些示例（假设你在 `/proc/sys/fs/binfmt_misc` 目录下）：

```

    echo ':i386:M::\x7fELF\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x03:\xff\xff\xff\xff\xff\xfe\xfe\xff\xff\xff\xff\xff\xff\xff\xff\xff\xfb\xff\xff:/bin/em86:' > register
    echo ':i486:M::\x7fELF\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x06:\xff\xff\xff\xff\xff\xfe\xfe\xff\xff\xff\xff\xff\xff\xff\xff\xff\xfb\xff\xff:/bin/em86:' > register

```
```

    echo ':DEXE:M::\x0eDEX::/usr/bin/dosexec:' > register

```
```

    echo ':DOSWin:M::MZ::/usr/local/bin/wine:' > register

```
有关 Java 支持，请参见 Documentation/admin-guide/java.rst


你可以通过向 `/proc/sys/fs/binfmt_misc/status` 或 `/proc/.../the_name` echo 0（禁用）或 1（启用）来启用/禁用 binfmt_misc 或某个二进制类型。查看该文件的内容会告诉你 `binfmt_misc/the_entry` 的当前状态。

你可以通过向 `/proc/.../the_name` 或 `/proc/sys/fs/binfmt_misc/status` echo -1 来删除一个条目或所有条目。


### 提示


如果你想向解释器传递特殊参数，可以为它编写一个包装脚本。
示例请参见 [Documentation/admin-guide/java.rst <./java>](Documentation/admin-guide/java.rst <./java>)。

你的解释器不应在 PATH 中查找文件名；内核会向它传递要使用的完整文件名（或文件描述符）。使用 `$PATH` 可能导致意外行为，并可能带来安全隐患。


Richard Günther <rguenth@tat.physik.uni-tuebingen.de>
