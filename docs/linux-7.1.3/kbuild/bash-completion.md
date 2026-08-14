
## Kbuild 的 Bash 补全


内核构建系统是用 Makefile 编写的，而 `make` 命令的 Bash 补全可通过
`bash-completion`_ 项目获得。

然而，内核构建的 Makefile 非常复杂。`make` 命令的通用补全规则除 `make`
命令自身的选项外，无法为内核构建系统提供有意义的建议。

为了增强对各种变量与目标的补全，内核源码在其 `scripts/bash-completion/make`
中包含了一个自己的补全脚本。

该脚本在内核树内工作时提供额外的补全。在内核树之外，它默认回退到 `make`
命令的通用补全规则。

## 先决条件


该脚本依赖于 `bash-completion`_ 项目提供的辅助函数。请确保它已安装在你的
系统上。在大多数发行版中，你可以通过标准包管理器安装 `bash-completion`
软件包。

## 如何使用


```

  $ source scripts/bash-completion/make

```
或者，你可以将它复制到 Bash 补全脚本的搜索路径中。
```

  $ mkdir -p ~/.local/share/bash-completion/completions
  $ cp scripts/bash-completion/make ~/.local/share/bash-completion/completions/

```
## 细节


在以下情况下会启用针对 Kbuild 的额外补全：

 - 你处于内核源码的根目录。
 - 你处于由 O= 选项创建的顶层构建目录
   （通过指向内核源码的 `source` 符号链接检查）。
 - -C make 选项指定了内核源码或构建目录。
 - -f make 选项指定了内核源码或构建目录中的某个文件。

如果以上都不满足，则回退到通用补全规则。

补全支持：

  - 常用目标，例如 `all`、`menuconfig`、`dtbs` 等。
  - Make（或环境）变量，例如 `ARCH`、`LLVM` 等。
  - 单目标构建（`foo/bar/baz.o`）
  - 配置文件（`**_defconfig` 与 `**.config`）

一些变量提供智能行为。例如，`CROSS_COMPILE=` 后跟一个 TAB 会显示已安装的
工具链。所显示的 defconfig 文件列表取决于 `ARCH=` 变量的值。
