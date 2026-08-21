## 可复现构建（Reproducible builds

通常期望使用同一组工具构建相同的源代码时能够复现，即输出
始终完全相同。这使得验证二进制发行版或嵌入式系统的构基础设施是否被破坏成为可能。这也可以更容易地验证源代码
或工具的更改是否对生成的二进制文件没有任何影响
`Reproducible Builds project`_ 有更多关于这一通用主题的信息本文档涵盖了构建内核可能不可复现的各种原因，以及如何避免它们
### 时间

内核在三个地方嵌入了时间戳：

- `uname()` 暴露并在 `/proc/version` 中包含的版本字符
- 嵌入initramfs 中的文件时间
- 如果通过 `CONFIG_IKHEADERS` 启用，内核或相应模块中嵌入的
  内核头文件的文件时间戳，通过 `/sys/kernel/kheaders.tar.xz` 暴露

默认情况下，时间戳是当前时间；在 `kheaders` 的情况下，是各个文件修改时间。这必须使用 `KBUILD_BUILD_TIMESTAMP`_ 变量来覆盖如果你是依据一git 提交来构建，可以使用它的提交日期
内核***使用 `__DATE__` `__TIME__` 宏，并在使用它们启用警告。如果你引入了确实使用这些宏的外部代码，你必通过设置 `SOURCE_DATE_EPOCH`_ 环境变量来覆盖它们所对应的时间戳
### 用户、主

内核`/proc/version` 中嵌入了构建用户与主机名这些必须使用 `KBUILD_BUILD_USER and KBUILD_BUILD_HOST`_ 变量来覆盖如果你是依据一git 提交来构建，可以使用它的提交者地址
### 绝对文件

当内核在树外（out-of-tree）构建时，调试信息可能包源文件与构建目录的绝对文件名。这些必须通过`KCFLAGS`_ `KAFLAGS`_ 变量中为每一个都包含 `-fdebug-prefix-map` 选项来覆盖，
以覆`.c` `.S` 文件
根据所使用的编译器，`__FILE__` 宏在树外构建时也可能
展开为绝对文件名。Kbuild 会自动使`-fmacro-prefix-map` 选项
来防止这一点（如果受支持）
Reproducible Builds 网站有更多关于这`prefix-map options`_ 的信息
某些 CONFIG 选项（如 `CONFIG_DEBUG_EFI`）会在目标文件中嵌入绝对路径此类选项应当被禁用
### 源代码包中的生成文件


`tools/` 子目录下某些程序的构建过程并不完全支持树外构建这可能导致后续使用例`make rpm-pkg` 构建源代码包包含了生成的文件。你应该在构建源代码包之前，通过运行
`make mrproper` `git clean -d -f -x` 来确保源代码树是干净的
### 模块签名


如果你启用了 `CONFIG_MODULE_SIG_ALL`，默认行为是为每次构生成一个不同的临时密钥，导致模块不可复现。然而，将签名密包含在你的源代码中大概会违背为模块签名的目的
对此的一种做法是将构建过程拆分，使不可复现的部分
可以当作源来处理
1. 生成一个持久的签名密钥。将该密钥的证书添加到内核源码中
2. `CONFIG_SYSTEM_TRUSTED_KEYS` 符号设置为包含签名密钥的证书   `CONFIG_MODULE_SIG_KEY` 设置为空字符串，并禁   `CONFIG_MODULE_SIG_ALL`。构建内核与模块
3. 为模块创建分离的签名，并将它们作为源发布
4. 执行第二次构建，附加模块签名。它可以重新构建模块   也可以使用第 2 步的输出
### 结构随机

如果你启用了 `CONFIG_RANDSTRUCT`，你将需要预先在
`scripts/basic/randstruct.seed` 中生成随机种子，以便每次构建
使用相同的值。细节请参见 `scripts/gen-randstruct-seed.sh`
### 调试信息冲突


这不是一个不可复现的问题，而是生成文件**过于**可复现的问题
一旦你为可复现构建设置了所有必要的变量，即使对于不同的内核
版本，vDSO 的调试信息也可能完全相同。这可能导致不同内核版本调试信息包之间出现文件冲突
为了避免这一点，你可以通过vDSO 中包含一个任意的“盐值”（salt字符串，使不同内核版本的 vDSO 彼此不同。这Kconfig 符号
`CONFIG_BUILD_SALT` 指定
### Git


git 中未提交的更改或不同的提ID 也可能导致不同的编译结果例如，在执行 `git reset HEAD^` 之后，即使代码相同，编译期间生成`include/config/kernel.release` 也会不同，最终将导致二进制差异细节请参`scripts/setlocalversion`