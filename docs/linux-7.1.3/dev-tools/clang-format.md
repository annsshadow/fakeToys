
## clang-format


`clang-format` 是一个根据一组规则和启发式方法格式化 C/C++/... 代码的工具。和大多数工具一样，它并不完美，也无法覆盖每一种情况，但它已经足够好用、能帮上忙

`clang-format` 可用于多种用途：

  - 快速将一块代码重新格式化为内核风格。在移动代码、对排序时特别有用。参clangformatreformat_

  - 在你维护的文件、你审查的补丁、diff 等中发现风格错误、笔误以及可能的改进。参clangformatreview_

  - 帮助你遵循代码风格规则，对于刚接触内核开发、或者同时在多个采用不同代码风格的项目中工作的人特别有用

它的配置文件是内核树根目录下`.clang-format`。其中包含的规则试图近似最常见的内核代码风格。它们也尽可能遵Documentation/process/coding-style.rst <codingstyle>。由于并非所有内核都遵循相同的风格，你可能希望针对某个特定的子系统或文件夹调整默认值。为此，你可以在子文件夹中编写另一`.clang-format` 文件来覆盖默认值

该工具本身早已被包含在流行的 Linux 发行版仓库中。请在你的仓库中搜索 `clang-format`。否则，你可以下载预编译LLVM/clang 二进制文件，或者从以下地址构建源代码：

    https://releases.llvm.org/download.html

有关该工具的更多信息，请参阅

    https://clang.llvm.org/docs/ClangFormat.html

    https://clang.llvm.org/docs/ClangFormatStyleOptions.html



### 审查文件和补丁的代码风格


通过以行内（inline）模式运行该工具，你可以审查整个子系统、文件夹或单个文件的代码风格错误、笔误或改进之处

```

    # Make sure your working directory is clean!
    clang-format -i kernel/*.[ch]

```
然后查看 git diff

统计这种 diff 的行数也有助于改调整配置文件中的风格选项；以及测试新`clang-format` 特版本

`clang-format` 也支持读取统一 diff，因此你可以轻松地审查补丁和 git diff。请参阅文档

    https://clang.llvm.org/docs/ClangFormat.html#script-for-patch-reformatting

```

    int formatted_code;
    // clang-format off
        void    unformatted_code  ;
    // clang-format on
    void formatted_code_again;

```
虽然使用它来让某个文件始终与 `clang-format` 保持同步可能很诱人，特别是当你在编写新文件或是维护者时，但请注意，其他人可能运行着不同版本`clang-format`，或者根本没有该工具。因此，你可能应该避免在内核源码中使用它；至少在我们确认 `clang-format` 是否变得普及之前



### 重新格式化代码块


通过使用与文本编辑器的集成，你可以用一次按键重新格式化任意代码块（选区）。这在移动代码、处理深度缩进的复杂代码、多行宏（以及对齐它们的反斜杠）等情况下特别有用

请记住，在工具未能完美处理的那些情况下，你始终可以在事后调整这些修改。但作为初步近似，它会非常有用

许多流行的文本编辑器都有集成支持。其中一些（vim、emacs、BBEdit Visual Studio）内置了支持。有关说明，请阅读：

    https://clang.llvm.org/docs/ClangFormat.html

对于 Atom、Eclipse、Sublime Text、Visual Studio Code、XCode 以及其它编辑器和 IDE，你应该能够找到即用型插件

对于这种用例，考虑使用一个辅助的 `.clang-format`，以便你可以调整一些选项。参clangformatextra_



### 缺失的支


`clang-format` 缺少对内核代码中一些常见东西的支持。它们很容易记住，所以如果你经常使用该工具，很快就会学会避开/忽略它们

尤其，你会注意到一些非常常见的情况

```

        #define TRACING_MAP_BITS_DEFAULT       11
        #define TRACING_MAP_BITS_MAX           17
        #define TRACING_MAP_BITS_MIN           7

    vs.::

        #define TRACING_MAP_BITS_DEFAULT 11
        #define TRACING_MAP_BITS_MAX 17
        #define TRACING_MAP_BITS_MIN 7

  - Aligned designated initializers, e.g.::

        static const struct file_operations uprobe_events_ops = {
                .owner          = THIS_MODULE,
                .open           = probes_open,
                .read           = seq_read,
                .llseek         = seq_lseek,
                .release        = seq_release,
                .write          = probes_write,
        };

    vs.::

        static const struct file_operations uprobe_events_ops = {
                .owner = THIS_MODULE,
                .open = probes_open,
                .read = seq_read,
                .llseek = seq_lseek,
                .release = seq_release,
                .write = probes_write,
        };


```

### 额外的特选项


为了在输出与当前代码之间的差异最小化，配置文件中默认没有启用某些特风格选项。换句话说，为了让差异尽可能小，从而使全文件风格的审查以及 diff 和补丁的审查尽可能容易

在其它情况下（例如特定的子系文件文件），内核风格可能有所不同，启用其中一些选项可能会更好地近似那里的风格

例如

  - 对齐赋值（`AlignConsecutiveAssignments`）

  - 对齐声明（`AlignConsecutiveDeclarations`）

  - 重新排版注释中的文本（`ReflowComments`）

  - 排序 `#includes`（`SortIncludes`）

它们通常对代码块重新格式化更有用，而非全文件。你可能想创建另一`.clang-format` 文件，并从你的编辑器/IDE 中改用它
