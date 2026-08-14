
## 编码规范


本文档描述了如何在内核中编写 Rust 代码。


### 风格与格式


代码应当使用 `rustfmt` 进行格式化。这样，偶尔为内核做贡献的人就不需要学习并记住又一份风格指南。更重要的是，审阅者和维护者不再需要花费时间指出风格问题，因此合并一个改动可能需要的补丁往返次数也会更少。

  `rustfmt`。因此这些仍然需要被留意。

使用 `rustfmt` 的默认设置。这意味着遵循惯用的 Rust 风格。例如，使用 4 个空格而不是制表符进行缩进。

方便的做法是让编辑器/IDE 在输入时、保存时或提交时自动格式化。不过，如果由于某种原因在某个时刻需要重新格式化整个内核的 Rust 源码，可以使用以下命令：

```
	make LLVM=1 rustfmt
```
也可以检查是否所有内容都已格式化（打印出差异）：

```
	make LLVM=1 rustfmtcheck
```
与内核其余部分使用 `clang-format` 类似，`rustfmt` 作用于单个文件，并且不需要内核配置。有时它甚至可以在代码有语法错误时工作。

#### 导入


默认情况下，`rustfmt` 会以在合并和变基时容易引发冲突的方式格式化导入，因为在某些情况下它会把多个项压缩到同一行。例如：

	// Do not use this style.
	use crate::{
	    example1,
	    example2::{example3, example4, example5},
	    example6, example7,
	    example8::example9,
	};

相反，内核使用如下所示的纵向布局：

	use crate::{
	    example1,
	    example2::{
	        example3,
	        example4,
	        example5, //
	    },
	    example6,
	    example7,
	    example8::example9, //
	};

也就是说，每个项独占一行，并且只要列表中不止一个项，就使用花括号。

末尾的空注释用于保留这种格式。不仅如此，`rustfmt` 在添加了该空注释后实际上会将导入纵向重新格式化。也就是说，可以通过对如下输入运行 `rustfmt`，轻松地将原始示例重新格式化为期望的风格：

	// Do not use this style.
	use crate::{
	    example1,
	    example2::{example3, example4, example5, //
	    },
	    example6, example7,
	    example8::example9, //
	};

末尾的空注释对嵌套导入（如上所示）以及单项导入都有效——这对于最小化补丁系列内部的差异很有用：

	use crate::{
	    example1, //
	};

末尾的空注释在花括号内的任意行都有效，但最好将其保留在最后一项中，因为这让人联想到其他格式化工具中的末尾逗号。有时，由于列表中内容的变动，在补丁系列中避免多次移动该注释可能更简单。

可能会有一些需要例外的情况，即这些都不是硬性规则。也有一些代码尚未迁移到这种风格，但请不要引入其他风格的代码。

最终的目标是让 `rustfmt` 在稳定版本中自动支持这种格式（或类似的格式），而无需末尾的空注释。因此，在某个时候，目标是移除这些注释。


### 注释


“普通”注释（即 `//`，而不是以 `///` 或 `//!` 开头的代码文档）使用 Markdown 编写，方式与文档注释相同，即使它们不会被渲染。这提高了一致性、简化了规则，并允许更容易地在两种注释之间移动内容。例如：

	// `object` is ready to be handled now.
	f(object);

此外，就像文档一样，注释在句子开头首字母大写，并以句号结尾（即使只有一个句子）。这包括 `// SAFETY:`、`// TODO:` 以及其他“带标签”的注释，例如：

	// FIXME: The error should be handled properly.

注释不应当用于文档目的：注释用于实现细节，而不是给用户看的。这种区分即使源文件的读者同时是某个 API 的实现者和用户时也是有用的。事实上，有时同时使用注释和文档会很有用。例如，对于一个 `TODO` 列表，或者对文档本身进行注释。对于后一种情况，注释可以插入在中间；也就是说，更靠近要被注释的那行文档。对于任何其他情况，注释写在文档之后，例如：

	/// Returns a new [`Foo`].
	///
	/// # Examples
	///
	// TODO: Find a better example.
	/// ```
	/// let foo = f(42);
	/// ```
	// FIXME: Use fallible approach.
	pub fn f(x: i32) -> Foo {
	    // ...
	}

这适用于公共和私有项。这提高了与公共项的一致性，使得可见性的变更涉及更少的改动，并且将允许我们 potentially 也为私有项生成文档。换句话说，如果为私有项编写了文档，那么仍然应当使用 `///`。例如：

	/// My private function.
	// TODO: ...
	fn f() {}

一种特殊的注释是 `// SAFETY:` 注释。它们必须出现在每个 `unsafe` 块之前，并解释为什么该块内的代码是正确的/安全的，即为什么它在任何情况下都不会触发未定义行为，例如：

	// SAFETY: `p` is valid by the safety requirements.
	unsafe { *p = 0; }

`// SAFETY:` 注释不应与代码文档中的 `# Safety` 小节混淆。`# Safety` 小节规定了调用者（对于函数）或实现者（对于 trait）需要遵守的契约。`// SAFETY:` 注释则说明为什么某次调用（对于函数）或实现（对于 trait）确实遵守了 `# Safety` 小节或语言参考中所陈述的前置条件。


### 代码文档


Rust 内核代码的文档方式不同于 C 内核代码（即通过 kernel-doc）。相反，使用为 Rust 代码编写文档的常规系统：`rustdoc` 工具，它使用 Markdown（一种轻量级标记语言）。

要学习 Markdown，外面有很多可用的指南。例如，位于：

	https://commonmark.org/help/

一个文档完备的 Rust 函数可能长这样：

	/// Returns the contained [`Some`] value, consuming the `self` value,
	/// without checking that the value is not [`None`].
	///
	/// # Safety
	///
	/// Calling this method on [`None`] is **[undefined behavior]**.
	///
	/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
	///
	/// # Examples
	///
	/// ```
	/// let x = Some("air");
	/// assert_eq!(unsafe { x.unwrap_unchecked() }, "air");
	/// ```
	pub unsafe fn unwrap_unchecked(self) -> T {
	    match self {
	        Some(val) => val,

	        // SAFETY: The safety contract must be upheld by the caller.
	        None => unsafe { hint::unreachable_unchecked() },
	    }
	}

这个例子展示了一些 `rustdoc` 特性以及内核中遵循的一些约定：

- 第一段必须是简要描述所文档化项功能的单个句子。进一步的解释必须放在额外的段落中。

- 不安全的函数必须在 `# Safety` 小节中记录其安全性前置条件。

- 虽然这里没有展示，但如果一个函数可能会 panic，则必须满足该条件的情况必须在一个 `# Panics` 小节中描述。

  请注意，panic 应当非常罕见，并且只有在有充分理由时才使用。在几乎所有情况下，都应当使用可失败的方式，通常返回一个 `Result`。

- 如果提供使用示例有助于读者，则必须写在一个名为 `# Examples` 的小节中。

- Rust 项（函数、类型、常量……）必须适当地链接（`rustdoc` 会自动创建链接）。

- 任何 `unsafe` 块之前都必须有一个 `// SAFETY:` 注释，描述为什么其中的代码是安全的。

  虽然有时理由看起来微不足道，因此似乎不需要，但编写这些注释不仅仅是一种记录已考虑因素的好方法，更重要的是，它提供了一种途径来表明不存在**额外**的隐式约束。

要了解更多关于如何为 Rust 编写文档以及额外特性的内容，请参阅 `rustdoc` 书籍：

	https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html

此外，内核支持通过在链接目标前加上 `srctree/` 来创建相对于源码树的链接。例如：

	//! C header: [`include/linux/printk.h`](srctree/include/linux/printk.h)

或者：

	/// [`struct mutex`]: srctree/include/linux/mutex.h


### C FFI 类型


Rust 内核代码使用类型别名（如 `c_int`）来引用 C 类型（如 `int`），这些别名可从 `kernel` prelude 中直接获得。请**不要使用来自 ``core``
: ffi`` 的别名——它们可能不能映射到正确的类型。

这些别名通常应当直接通过其标识符引用，即作为一个单段路径。例如：

	fn f(p: *const c_char) -> c_int {
	    // ...
	}


### 命名


Rust 内核代码遵循常规的 Rust 命名约定：

	https://rust-lang.github.io/api-guidelines/naming.html

当将现有的 C 概念（例如宏、函数、对象……）包装到 Rust 抽象中时，应当使用尽可能接近 C 侧的名称，以避免混淆，并提高在 C 和 Rust 两侧之间来回切换时的可读性。例如，来自 C 的 `pr_info` 等宏在 Rust 侧也使用相同的名称。

话虽如此，大小写应当调整为遵循 Rust 的命名约定，并且由模块和类型引入的命名空间不应在项名中重复。例如，当包装如下常量时：

	#define GPIO_LINE_DIRECTION_IN	0
	#define GPIO_LINE_DIRECTION_OUT	1

Rust 中的等价形式可能如下（忽略文档）：

	pub mod gpio {
	    pub enum LineDirection {
	        In = bindings::GPIO_LINE_DIRECTION_IN as _,
	        Out = bindings::GPIO_LINE_DIRECTION_OUT as _,
	    }
	}

也就是说，`GPIO_LINE_DIRECTION_IN` 的等价引用应当是
**``gpio``
: LineDirection::In``。特别地，它不应被命名为
**``gpio``
: gpio_line_direction::GPIO_LINE_DIRECTION_IN``。


### Lint 检查


在 Rust 中，可以在局部 `allow` 特定的警告（诊断信息、lint），使编译器忽略给定函数、模块、块等范围内某个警告的实例。

它类似于 C 中的 `#pragma GCC diagnostic push` + `ignored` + `pop` [#]_：

	#pragma GCC diagnostic push
	#pragma GCC diagnostic ignored "-Wunused-function"
	static void f(void) {}
	#pragma GCC diagnostic pop

       attributes（C23 的 `[[maybe_unused]]`）可能会被使用；不过，该例子旨在反映之后讨论的 Rust 中相应的 lint。

但简洁得多：

	#[allow(dead_code)]
	fn f() {}

凭借这一特性，可以舒适地默认启用更多诊断（即 `W=` 级别之外）。特别是那些可能有一些误报，但除此之外保持启用以捕获潜在错误相当有用的诊断。

除此之外，Rust 提供了 `expect` 属性，将其更进一步。它使得如果警告未被产生，编译器会发出警告。例如，以下内容将确保当 `f()` 在某处被调用时，我们将不得不移除该属性：

	#[expect(dead_code)]
	fn f() {}

```
	warning: this lint expectation is unfulfilled
	 --> x.rs:3:10
	  |
	3 | #[expect(dead_code)]
	  |          ^^^^^^^^^
	  |
	  = note: `#[warn(unfulfilled_lint_expectations)]` on by default
```

这意味着 `expect`\ s 在不再需要时不会被遗忘，这可能发生在多种情况下，例如：

- 在开发过程中添加的临时属性。

- 编译器、Clippy 或自定义工具中 lint 的改进，可能会移除一个误报。

- 当该 lint 不再需要，因为预期它会在某个时刻被移除，例如上面的 `dead_code` 例子。

它还提高了剩余 `allow`\ s 的可见性，并降低了误用一个的可能性。

因此，除非下列情况，否则优先使用 `expect` 而非 `allow`：

- 条件编译在某些情况而非其他情况下触发警告。

  如果触发警告（或不触发警告）的情况相对于总数只有少数，那么可以考虑使用条件 `expect`（即 `cfg_attr(..., expect(...))`）。否则，直接使用 `allow` 可能更简单。

- 在宏内部，当不同的调用可能生成在某些情况而非其他情况下触发警告的扩展代码时。

- 当代码可能因某些架构而非其他架构触发警告时，例如向 C FFI 类型进行的 `as` 转换。

作为一个更完整的例子，考虑这个程序：

	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

这里，如果未设置 `CONFIG_X`，函数 `g()` 就是死代码。我们可以在这里使用 `expect` 吗？

	#[expect(dead_code)]
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

如果在设置了 `CONFIG_X` 的情况下，这会发出一个 lint，因为在该配置中它不是死代码。因此，在这样的情况中，我们不能原样使用 `expect`。

一个简单的办法是使用 `allow`：

	#[allow(dead_code)]
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

另一种选择是使用条件 `expect`：

	#[cfg_attr(not(CONFIG_X), expect(dead_code))]
	fn g() {}

	fn main() {
	    #[cfg(CONFIG_X)]
	    g();
	}

这将确保如果有人在某处引入了对 `g()` 的另一个调用（例如无条件地），那么就能被发现它不再是死代码。不过，`cfg_attr` 比简单的 `allow` 更复杂。

因此，当涉及超过一两个配置，或者该 lint 可能因非局部变更（如 `dead_code`）而被触发时，使用条件 `expect`\ s 可能并不值得。

有关 Rust 中诊断信息的更多信息，请参阅：

	https://doc.rust-lang.org/stable/reference/attributes/diagnostics.html

### 错误处理


有关 Linux 专用 Rust 错误处理的一些背景和指南，请参阅：

	https://rust.docs.kernel.org/kernel/error/type.Result.html#error-codes-in-c-and-rust
