
## 快速开始（Quick Start）


本文档描述如何开始使用 Rust 进行内核开发。

有几种方式可以安装内核开发所需的 Rust 工具链。一种简单的方法是使用你的 Linux 发行版
提供的软件包（如果适用）——下面第一节解释了这种方式。这种方式的一个优点是，发行版通常
会使 Rust 所使用的 LLVM 与 Clang 相匹配。

另一种方式是使用 `kernel.org <https://kernel.org/pub/tools/llvm/rust/>`_ 上提供的
预构建稳定版 LLVM+Rust。这些是与“获取 LLVM”（Getting LLVM <getting_llvm>）中相同的
精简且快速的 LLVM 工具链，只是额外加入了 Linux 的 Rust 所支持的 Rust 版本。提供了两组：
“latest LLVM” 和 “matching LLVM”（更多信息请参阅该链接）。

此外，接下来的两个 “Requirements”（需求）小节分别解释了各个组件以及如何通过 `rustup`、
Rust 的独立安装程序，和/或自行构建来安装它们。

文档的其余部分解释了如何上手的其他方面。

### 发行版


######## Arch Linux


Arch Linux 提供较新的 Rust 版本，因此通常可以直接使用
```

	pacman -S rust rust-src rust-bindgen


```
######## Debian


Debian 13（Trixie），以及 Testing 和 Debian Unstable（Sid）提供较新的
```
	apt install rustc rust-src bindgen rustfmt rust-clippy


```
######## Fedora Linux


Fedora Linux 提供较新的 Rust 版本，因此通常可以直接使用
```
	dnf install rust rust-src bindgen-cli rustfmt clippy


```
######## Gentoo Linux


Gentoo Linux 提供较新的 Rust 版本，因此通常可以直接使用
```
	USE='rust-src rustfmt clippy' emerge dev-lang/rust dev-util/bindgen
```
`LIBCLANG_PATH` 可能需要设置。

######## Nix


Nix 提供较新的 Rust 版本，因此通常可以直接使用
```
	{ pkgs ? import <nixpkgs> {} }:
	pkgs.mkShell {
	  nativeBuildInputs = with pkgs; [ rustc rust-bindgen rustfmt clippy ];
	  RUST_LIB_SRC = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
	}


```
######## openSUSE


openSUSE Slowroll 和 openSUSE Tumbleweed 提供较新的 Rust 版本，因此
```
	zypper install rust rust-src rust-bindgen clang


```
######## Ubuntu


Ubuntu 25.10 和 26.04 LTS 提供较新的 Rust 版本，因此
```
	apt install rustc rust-src bindgen rustfmt rust-clippy
```
```
	RUST_LIB_SRC=/usr/src/rustc-$(rustc --version | cut -d' ' -f2)/library
```
为方便起见，`RUST_LIB_SRC` 可以导出到全局环境中。

#### 24.04 LTS 及更旧版本


虽然 Ubuntu 24.04 LTS 及更旧版本仍提供较新的 Rust 版本，但它们需要设置一些额外的
配置，使用
```
	apt install rustc-1.85 rust-1.85-src bindgen-0.71 rustfmt-1.85 \
		rust-1.85-clippy
	ln -s /usr/lib/rust-1.85/bin/rustfmt /usr/bin/rustfmt-1.85
	ln -s /usr/lib/rust-1.85/bin/clippy-driver /usr/bin/clippy-driver-1.85
```
这些软件包都没有将它们的工具设为默认，因此需要
```
	make LLVM=1 RUSTC=rustc-1.85 RUSTDOC=rustdoc-1.85 RUSTFMT=rustfmt-1.85 \
		CLIPPY_DRIVER=clippy-driver-1.85 BINDGEN=bindgen-0.71
```
或者，修改 `PATH` 变量，将 Rust 1.85 的二进制文件放在前面
```
	PATH=/usr/lib/rust-1.85/bin:$PATH
	update-alternatives --install /usr/bin/bindgen bindgen \
		/usr/bin/bindgen-0.71 100
	update-alternatives --set bindgen /usr/bin/bindgen-0.71
```
```
	RUST_LIB_SRC=/usr/src/rustc-$(rustc-1.85 --version | cut -d' ' -f2)/library
```
为方便起见，`RUST_LIB_SRC` 可以导出到全局环境中。

此外，`bindgen-0.71` 在较新的版本（24.04 LTS）中可用，但在较旧的版本（20.04 LTS 和
22.04 LTS）中可能不可用，因此 `bindgen` 可能需要手动构建（请参阅下文）。

### 需求：构建


本节解释如何获取构建所需的工具。

要轻松检查是否满足要求，可运行以下目标
```
	make LLVM=1 rustavailable
```
这会触发与 Kconfig 相同的逻辑，以判断是否需要启用 `RUST_IS_AVAILABLE`；如果不是，
它还会解释原因。

######## rustc


需要使用较新版本的 Rust 编译器。

如果使用 `rustup`，进入内核构建目录（或对 `set` 子命令使用 `--path=<build-dir>`
参数）并运行
```
	rustup override set stable
```
这会将你的工作目录配置为使用给定版本的 `rustc`，而不会影响你的默认工具链。

注意，该覆盖适用于当前工作目录（及其子目录）。

如果不使用 `rustup`，可从以下地址获取独立安装程序：

	https://forge.rust-lang.org/infra/other-installation-methods.html#standalone

######## Rust 标准库源码


需要 Rust 标准库源码，因为构建系统会对 `core` 进行交叉编译。

```
	rustup component add rust-src
```
组件是按工具链安装的，因此以后升级 Rust 编译器版本需要重新添加该组件。

否则，如果使用独立安装程序，Rust 源码树可以
```
	curl -L "https://static.rust-lang.org/dist/rust-src-$(rustc --version | cut -d' ' -f2).tar.gz" |
		tar -xzf - -C "$(rustc --print sysroot)/lib" \
		"rust-src-$(rustc --version | cut -d' ' -f2)/rust-src/lib/" \
		--strip-components=3
```
在这种情况下，以后升级 Rust 编译器版本需要手动更新源码树（方法是先删除
``$(rustc --print sysroot)/lib/rustlib/src/rust``，然后重新运行上述命令）。

######## libclang


`libclang`（LLVM 的一部分）被 `bindgen` 用来理解内核中的 C 代码，这意味着需要安装
LLVM；就像使用 `LLVM=1` 编译内核时一样。

Linux 发行版很可能提供合适的版本，因此最好先检查。

也有一些适用于若干系统和架构的二进制文件上传在：

	https://releases.llvm.org/download.html

否则，构建 LLVM 相当耗时，但过程并不复杂：

	https://llvm.org/docs/GettingStarted.html#getting-the-source-code-and-building-llvm

更多信息以及获取预构建版本和发行版软件包的其他方式，请参阅
Documentation/kbuild/llvm.rst。

######## bindgen


到内核 C 侧的绑定是在构建时使用 `bindgen` 工具生成的。

例如通过以下方式安装（注意，这会下载并构建该工具
```
	cargo install --locked bindgen-cli
```
`bindgen` 使用 `clang-sys` crate 来查找合适的 `libclang`（可能静态链接、动态链接或
在运行时加载）。默认情况下，上面的 `cargo` 命令会生成一个在运行时加载 `libclang` 的
`bindgen` 二进制文件。如果找不到（或者应该使用与找到的不同的 `libclang`），可以调整该
过程，例如使用 `LIBCLANG_PATH` 环境变量。详情请参阅 `clang-sys` 的文档：

	https://github.com/KyleMayes/clang-sys#linking

	https://github.com/KyleMayes/clang-sys#environment-variables

### 需求：开发


本节解释如何获取开发所需的工具。也就是说，仅在构建内核时并不需要这些工具。

######## rustfmt


`rustfmt` 工具用于自动格式化所有 Rust 内核代码，包括生成的 C 绑定（详情请参阅
coding-guidelines.rst）。

如果使用 `rustup`，其 `default` profile 已经安装了该工具，因此无需任何操作。如果使用
其他 profile，则
```
	rustup component add rustfmt
```
独立安装程序也随附 `rustfmt`。

######## clippy


`clippy` 是一个 Rust 的 linter。运行它可以提供 Rust 代码的额外警告。可以通过向 `make`
传入 `CLIPPY=1` 来运行（详情请参阅 general-information.rst）。

如果使用 `rustup`，其 `default` profile 已经安装了该工具，因此无需任何操作。如果使用
其他 profile，则
```
	rustup component add clippy
```
独立安装程序也随附 `clippy`。

######## rustdoc


`rustdoc` 是 Rust 的文档工具。它为 Rust 代码生成美观的 HTML 文档（详情请参阅
general-information.rst）。

`rustdoc` 也用于测试有文档的 Rust 代码中所提供的示例（称为 doctests 或文档测试）。
`rusttest` Make 目标使用了这一特性。

如果使用 `rustup`，所有 profile 都已经安装了该工具，因此无需任何操作。

独立安装程序也随附 `rustdoc`。

######## rust-analyzer


`rust-analyzer <https://rust-analyzer.github.io/>`_ 语言服务器可以与许多编辑器一起使用，
以实现语法高亮、补全、跳转到定义以及其他功能。

`rust-analyzer` 需要一个配置文件 `rust-project.json`，它
```
	make LLVM=1 rust-analyzer


```
### 配置


需要在 `General setup` 菜单中启用 `Rust support`（`CONFIG_RUST`）。只有当找到合适的
Rust 工具链（见上文）且满足其他要求时，该选项才会显示。反过来，这会使依赖于 Rust 的
其余选项可见。

```
	Kernel hacking
	    -> Sample kernel code
	        -> Rust samples
```
并启用一些示例模块，可以是内建或可作为模块加载。

### 构建


使用完整的 LLVM 工具链构建内核是受支持最好的配置
```
	make LLVM=1
```
使用 GCC 对某些配置也有效，但目前非常实验性。

### 深入探究（Hacking）


要深入了解，可以查看 `samples/rust/` 下的示例代码、 `rust/` 下的 Rust 支持代码，以及
`Kernel hacking` 下的 `Rust hacking` 菜单。
