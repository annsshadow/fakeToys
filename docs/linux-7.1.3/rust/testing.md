
## 测试


本文档包含如何测试内核中 Rust 代码的有用信息。

测试共有三种：

- KUnit 测试。
- `#[test]` 测试。
- Kselftest（内核自测试）。

### KUnit 测试


这些测试来自 Rust 文档中的示例，它们会被转换成 KUnit 测试。

######## 用法


这些测试可以通过 KUnit 运行。例如通过 `kunit_tool`（`kunit.py`）
```
	./tools/testing/kunit/kunit.py run --make_options LLVM=1 --arch x86_64 --kconfig_add CONFIG_RUST=y

```
另外，KUnit 也可以在启动时将它们作为内核内建模块运行。关于通用的 KUnit 文档，
请参阅 Documentation/dev-tools/kunit/index.rst；关于内核内建与命令行测试的
细节，请参阅 Documentation/dev-tools/kunit/architecture.rst。

```
	CONFIG_KUNIT
	   Kernel hacking -> Kernel Testing and Coverage -> KUnit - Enable support for unit tests
	CONFIG_RUST_KERNEL_DOCTESTS
	   Kernel hacking -> Rust hacking -> Doctests for the `kernel` crate

```
在内核配置系统中。

######## KUnit 测试即文档测试


这些文档测试通常是任意条目（例如函数、结构体、模块……）的用法示例。

它们非常方便，因为只需写在文档旁边即可。例如：


	/// Sums two numbers.
	///
	/// ```
	/// assert_eq!(mymod::f(10, 20), 30);
	/// ```
	pub fn f(a: i32, b: i32) -> i32 {
	    a + b
	}

在用户空间中，这些测试由 `rustdoc` 收集并运行。直接使用该工具已经很有用，因为
它可以验证示例能够编译（从而强制它们与所文档化的代码保持同步），同时也可以
运行那些不依赖内核内 API 的示例。

然而，对于内核，这些测试会被转换成 KUnit 测试套件。这意味着文档测试会被编译为
Rust 内核对象，从而能够针对已构建的内核运行。

这种 KUnit 集成的一个好处是，Rust 文档测试可以复用已有的
```
	KTAP version 1
	1..1
	    KTAP version 1
	    # Subtest: rust_doctests_kernel
	    1..59
	    # rust_doctest_kernel_build_assert_rs_0.location: rust/kernel/build_assert.rs:13
	    ok 1 rust_doctest_kernel_build_assert_rs_0
	    # rust_doctest_kernel_build_assert_rs_1.location: rust/kernel/build_assert.rs:56
	    ok 2 rust_doctest_kernel_build_assert_rs_1
	    # rust_doctest_kernel_init_rs_0.location: rust/kernel/init.rs:122
	    ok 3 rust_doctest_kernel_init_rs_0
	    ...
	    # rust_doctest_kernel_types_rs_2.location: rust/kernel/types.rs:150
	    ok 59 rust_doctest_kernel_types_rs_2
	# rust_doctests_kernel: pass:59 fail:0 skip:0 total:59
	# Totals: pass:59 fail:0 skip:0 total:59
	ok 1 rust_doctests_kernel

```
使用 `? <https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator>`_
运算符的测试也照常支持，例如：


	/// ```
	/// # use kernel::{spawn_work_item, workqueue};
	/// spawn_work_item!(workqueue::system(), || pr_info!("x\n"))?;
	/// # Ok::<(), Error>(())
	/// ```

这些测试也会在 `CLIPPY=1` 下使用 Clippy 进行编译，就像普通代码一样，因此也能
受益于额外的 lint 检查。

为了让开发者能够轻松看到是哪一行文档测试代码导致了失败，会向日志打印一行
KTAP 诊断信息。其中包含原始测试的位置（文件和行号），即（而非转换后代码中的
位置）
```
	# rust_doctest_kernel_types_rs_2.location: rust/kernel/types.rs:150

```
Rust 测试似乎使用 Rust 标准库（`core`）中常用的 `assert!` 和 `assert_eq!`
宏来进行断言。我们提供了一个自定义版本，将调用转发到 KUnit。重要的是，这些
宏不需要传入上下文（context），这与 KUnit 测试所用的宏（即 `struct kunit *`）
不同。这使得它们更易使用，并且文档的读者无需关心使用的是哪个测试框架。此外，
这可能让我们在未来更轻松地测试第三方代码。

当前的一个限制是 KUnit 不支持在其他任务中进行断言。因此，我们目前如果断言
确实失败，就只是向内核日志打印一个错误。另外，文档测试不会针对非公开函数运行。

由于这些测试就是示例，即它们是文档的一部分，因此通常应当像“真实代码”那样编写。
因此，例如，与其使用 `unwrap()` 或 `expect()`，不如使用 `?` 运算符。更多背景
请参见：

	https://rust.docs.kernel.org/kernel/error/type.Result.html#error-codes-in-c-and-rust

### ``#[test]`` 测试


此外，还有 `#[test]` 测试。与文档测试类似，它们也与你在用户空间所期望的颇为
相似，并且它们也被映射到 KUnit。

这些测试由 `kunit_tests` 过程宏引入，该宏以测试套件的名称作为参数。

例如，假设我们要测试文档测试小节中的函数 `f`。我们可以在与函数所在的同一个
文件中编写：


	#[kunit_tests(rust_kernel_mymod)]
	mod tests {
	    use super::*;

	    #[test]
	    fn test_f() {
	        assert_eq!(f(10, 20), 30);
	    }
	}

```
	    KTAP version 1
	    # Subtest: rust_kernel_mymod
	    # speed: normal
	    1..1
	    # test_f.speed: normal
	    ok 1 test_f
	ok 1 rust_kernel_mymod

```
与文档测试一样，`assert!` 和 `assert_eq!` 宏被映射回 KUnit，且不会 panic。
类似地，`? <https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator>`_
运算符也受支持，即测试函数可以返回空（即单元类型 `()`）或 `Result`（即任意
`Result<T, E>`）。例如：


	#[kunit_tests(rust_kernel_mymod)]
	mod tests {
	    use super::*;

	    #[test]
	    fn test_g() -> Result {
	        let x = g()?;
	        assert_eq!(x, 30);
	        Ok(())
	    }
	}

```
	    KTAP version 1
	    # Subtest: rust_kernel_mymod
	    # speed: normal
	    1..1
	    # test_g: ASSERTION FAILED at rust/kernel/lib.rs:335
	    Expected is_test_result_ok(test_g()) to be true, but is false
	    # test_g.speed: normal
	    not ok 1 test_g
	not ok 1 rust_kernel_mymod

```
如果一个 `#[test]` 测试能作为用户的示例而有价值，那么请改用文档测试。即使是
API 的边界情况，例如错误或边界情形，也值得在示例中展示。

### ``rusttest`` 主机测试


这些是用户空间测试，可以在主机（即运行编译的环境）上构建并运行
```
	make LLVM=1 rusttest

```
这需要内核的 `.config`。

目前，它们主要用于测试 `macros` crate 的示例。

### Kselftest（内核自测试）


Kselftest 也可在 `tools/testing/selftests/rust` 目录中找到。

测试所需的内核配置选项列在 `tools/testing/selftests/rust/config` 文件中，
可借助以下命令包含进来
```
	./scripts/kconfig/merge_config.sh .config tools/testing/selftests/rust/config

```
Kselftest 在内核源码树内构建，旨在运行于安装了相同内核的系统上。

一旦安装并启动了与源码树匹配的内核，执行
```
	make TARGETS="rust" kselftest

```
关于通用的 Kselftest 文档，请参阅 Documentation/dev-tools/kselftest.rst。
