
## 一般信息


本文档包含在从事内核的 Rust 支持相关工作时有用的一些信息。


### ``no_std``


内核中的 Rust 支持只能链接 `core <https://doc.rust-lang.org/core/>`_，
而不能链接 `std <https://doc.rust-lang.org/std/>`_。供内核使用的 crate 必须
使用 `#![no_std]` 属性来选择这种行为。


### 代码文档


内核的 Rust 代码使用其内置的文档生成器 `rustdoc` 来编写文档。

生成的 HTML 文档包含集成搜索、链接项（例如类型、函数、常量）、源代码等。
可以在以下地址阅读：

	https://rust.docs.kernel.org

对于 linux-next，请见：

	https://rust.docs.kernel.org/next/

每个主要发布版本也有对应的标签，例如：

	https://rust.docs.kernel.org/6.10/

这些文档也可以很容易地在本地生成和阅读。这相当快（与编译代码本身同量级），
且不需要特殊的工具或环境。这样做还有一个额外的好处，即它们会针对所使用
的特定内核配置量身定制。要生成它们，请使用 `rustdoc`
```

	make LLVM=1 rustdoc

```
```

	xdg-open Documentation/output/rust/rustdoc/kernel/index.html

```
要了解如何编写文档，请见 coding-guidelines.rst。


### 额外的 lint


虽然 `rustc` 是一个很有帮助的编译器，但还可以通过 Rust 的 lint 工具
`clippy` 获得一些额外的 lint 和分析。要启用它，请将 `CLIPPY=1` 传给
```

	make LLVM=1 CLIPPY=1

```
请注意，Clippy 可能会改变代码生成，因此不应在构建生产内核时启用它。


### 抽象与绑定


抽象是用 Rust 代码包装来自 C 侧的内核功能。

为了使用来自 C 侧的函数和类型，需要创建绑定（bindings）。绑定是那些来自
C 侧的函数和类型在 Rust 中的声明。

例如，可以在 Rust 中编写一个 `Mutex` 抽象，它包装来自 C 侧的
`struct mutex` 并通过绑定调用其函数。

并非所有内核内部 API 和概念都有可用的抽象，但我们有意随着时间推移扩大
覆盖范围。“叶”（Leaf）模块（例如驱动）不应直接使用 C 绑定。相反，子系统
应视需要提供尽可能安全的抽象。


	                                                rust/bindings/
	                                               (rust/helpers/)

	                                                   include/ -----+ <-+
	                                                                 |   |
	  drivers/              rust/kernel/              +----------+ <-+   |
	    fs/                                           | bindgen  |       |
	   .../            +-------------------+          +----------+ --+   |
	                   |    Abstractions   |                         |   |
	+---------+        | +------+ +------+ |          +----------+   |   |
	| my_foo  | -----> | | foo  | | bar  | | -------> | Bindings | <-+   |
	| driver  |  Safe  | | sub- | | sub- | |  Unsafe  |          |       |
	+---------+        | |system| |system| |          | bindings | <-----+
	     |             | +------+ +------+ |          |  crate   |       |
	     |             |   kernel crate    |          +----------+       |
	     |             +-------------------+                             |
	     |                                                               |
	     +------------------# FORBIDDEN #--------------------------------+

主要思想是，将所有与内核 C API 的直接交互封装到经过仔细审阅和文档化的抽象中。
这样一来，只要满足以下条件，这些抽象的使用者就不会引入未定义行为（UB）：

#. 抽象是正确的（“健全的”，sound）。
#. 任何 `unsafe` 块都遵守调用块内操作所必需的安全约定。类似地，任何
   `unsafe impl`\ s 都遵守实现该 trait 所必需的安全约定。

#### 绑定


通过将来自 `include/` 的 C 头文件包含进 `rust/bindings/bindings_helper.h`，
`bindgen` 工具会自动为所包含的子系统集成生成绑定。构建完成后，请查看
`rust/bindings/` 目录中的 `*_generated.rs` 输出文件。

对于 `bindgen` 不会自动生成的 C 头文件部分（例如 C 的 `inline` 函数或
非平凡的宏），可以在 `rust/helpers/` 中添加一个小的包装函数来使其同样对
Rust 侧可用。

#### 抽象


抽象是绑定与内核内使用者之间的一层。它们位于 `rust/kernel/`，其作用是将
对绑定的不安全访问封装到它们暴露给使用者的、尽可能安全的 API 中。抽象的
使用者包括用 Rust 编写的驱动或文件系统等。

除了安全性方面，抽象还应当是“符合人体工学的”（ergonomic），也就是说它们
将 C 接口转变为“惯用的”（idiomatic）Rust 代码。基本的例子包括将 C 的资源
获取与释放转变为 Rust 的构造函数与析构函数，或将 C 的整数错误码转变为
Rust 的 `Result`\ s。


### 条件编译


Rust 代码可以基于内核配置进行条件编译：


	#[cfg(CONFIG_X)]       // 已启用               (`y` 或 `m`)
	#[cfg(CONFIG_X="y")]   // 作为内建启用 (`y`)
	#[cfg(CONFIG_X="m")]   // 作为模块启用   (`m`)
	#[cfg(not(CONFIG_X))]  // 已禁用

对于 Rust 的 `cfg` 不支持的其他谓词（例如带有数值比较的表达式），可以
定义一个新的 Kconfig 符号：


	config RUSTC_HAS_SPAN_FILE
		def_bool RUSTC_VERSION >= 108800
