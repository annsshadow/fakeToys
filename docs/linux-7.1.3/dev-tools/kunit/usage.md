
## 编写测试


### 测试用例


KUnit 中的基本单元是测试用例（test case）。测试用例是一个签名为 `void (**)(struct kunit **test)` 的函数。它会调用被测函数，然后为应当发生的事情设置**期望（expectation）**。例如：


	void example_test_success(struct kunit *test)
	{
	}

	void example_test_failure(struct kunit *test)
	{
		KUNIT_FAIL(test, "This test never passes.");
	}

在上面的示例中，`example_test_success` 因为什么都不做而总是通过；由于没有设置任何期望，因此所有期望都通过。另一方面，`example_test_failure` 因为调用了 `KUNIT_FAIL` 而总是失败，`KUNIT_FAIL` 是一个特殊的期望，它会记录一条消息并导致测试用例失败。

#### 期望

**期望（expectation）**指定我们期望某段代码在测试中做某件事。期望像函数一样被调用。测试用例通过为被测代码的行为设置期望来构成。当一个或多个期望失败时，测试用例失败，并记录有关失败的信息。例如：


	void add_test_basic(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, add(1, 1));
	}

在上面的示例中，`add_test_basic` 对名为 `add` 的函数行为做了若干断言。第一个参数始终是 `struct kunit *` 类型，包含有关当前测试上下文的信息。在本例中，第二个参数是期望值。最后一个值是实际值。如果 `add` 通过了所有这些期望，测试用例 `add_test_basic` 将通过；如果这些期望中有任何一个失败，测试用例将失败。

当任何期望被违反时，测试用例就会**失败**；但是，测试会继续运行，并尝试其他期望，直到测试用例结束或以其他方式被终止。这与后面讨论的**断言（assertion）**不同。

要了解更多 KUnit 期望，请参阅 Documentation/dev-tools/kunit/api/test.rst。

   单个测试用例应当简短、易于理解，并专注于单一行为。

例如，如果我们想严格测试上面的 `add` 函数，可以创建额外的测试用例来测试 `add` 函数应当具备的每一个属性，如下所示：


	void add_test_basic(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 1, add(1, 0));
		KUNIT_EXPECT_EQ(test, 2, add(1, 1));
	}

	void add_test_negative(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, 0, add(-1, 1));
	}

	void add_test_max(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, INT_MAX, add(0, INT_MAX));
		KUNIT_EXPECT_EQ(test, -1, add(INT_MAX, INT_MIN));
	}

	void add_test_overflow(struct kunit *test)
	{
		KUNIT_EXPECT_EQ(test, INT_MIN, add(INT_MAX, 1));
	}

#### 断言


断言类似于期望，只是当条件不满足时，断言会立即终止测试用例。例如：


	static void test_sort(struct kunit *test)
	{
		int *a, i, r = 1;
		a = kunit_kmalloc_array(test, TEST_LEN, sizeof(*a), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, a);
		for (i = 0; i < TEST_LEN; i++) {
			r = (r * 725861) % 6599;
			a[i] = r;
		}
		sort(a, TEST_LEN, sizeof(*a), cmpint, NULL);
		for (i = 0; i < TEST_LEN-1; i++)
			KUNIT_EXPECT_LE(test, a[i], a[i + 1]);
	}

在此示例中，我们需要能够分配一个数组来测试 `sort()` 函数。因此我们使用 `KUNIT_ASSERT_NOT_ERR_OR_NULL()` 在出现分配错误时中止测试。

   在其他测试框架中，`ASSERT` 宏通常通过调用 `return` 实现，因此它们只能从测试函数
   中生效。在 KUnit 中，我们会在失败时停止当前的 kthread，因此可以从任何地方调用它们。

   警告：上述规则有一个例外。你不应在测试套件的 exit() 函数或资源的释放函数中使用
   断言。这些函数在测试关闭时运行，此处的断言会阻止后续清理代码运行，可能导致内存泄漏。

### 自定义错误消息


每个 `KUNIT_EXPECT` 和 `KUNIT_ASSERT` 宏都有一个 `_MSG` 变体。它们接受一个格式字符串和参数，为自动生成的错误消息提供额外的上下文。


	char some_str[^41^];
	generate_sha1_hex_string(some_str);

	/** Before. Not easy to tell why the test failed. **/
	KUNIT_EXPECT_EQ(test, strlen(some_str), 40);

	/** After. Now we see the offending string. **/
	KUNIT_EXPECT_EQ_MSG(test, strlen(some_str), 40, "some_str='%s'", some_str);

或者，可以通过使用 `KUNIT_FAIL()` 完全控制错误消息，例如：


	/** Before **/
	KUNIT_EXPECT_EQ(test, some_setup_function(), 0);

	/** After: full control over the failure message. **/
	if (some_setup_function())
		KUNIT_FAIL(test, "Failed to setup thing for testing");


#### 测试套件


我们需要许多测试用例来覆盖该单元的所有行为。拥有许多相似的测试是很常见的。为了减少这些紧密相关测试中的重复，大多数单元测试框架（包括 KUnit）都提供了**测试套件（test suite）**的概念。测试套件是一组测试用例的集合，带有可选的 setup 和 teardown 函数，分别在整套套件和/或每个测试用例之前/之后运行。

   测试用例只有在与某个测试套件关联时才会运行。

例如：


	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE(example_test_foo),
		KUNIT_CASE(example_test_bar),
		KUNIT_CASE(example_test_baz),
		{}
	};

	static struct kunit_suite example_test_suite = {
		.name = "example",
		.init = example_test_init,
		.exit = example_test_exit,
		.suite_init = example_suite_init,
		.suite_exit = example_suite_exit,
		.test_cases = example_test_cases,
	};
	kunit_test_suite(example_test_suite);

在上面的示例中，测试套件 `example_test_suite` 会先运行 `example_suite_init`，然后运行测试用例 `example_test_foo`、`example_test_bar` 和 `example_test_baz`。每个测试用例在运行前会立即调用 `example_test_init`，运行后会立即调用 `example_test_exit`。最后，在所有其他内容之后调用 `example_suite_exit`。`kunit_test_suite(example_test_suite)` 将该测试套件注册到 KUnit 测试框架。

   `exit` 和 `suite_exit` 函数即使 `init` 或 `suite_init` 失败也会运行。请确保它们能够
   处理由 `init` 或 `suite_init` 遇到错误或提前退出所导致的任何不一致状态。

`kunit_test_suite(...)` 是一个宏，它告诉链接器将指定的测试套件放入一个特殊的链接器段（linker section），以便 KUnit 在 `late_init` 之后或测试模块加载时（如果测试被构建为模块）运行它。

更多信息，请参阅 Documentation/dev-tools/kunit/api/test.rst。


### 为其他架构编写测试


编写能在 UML 上运行的测试，优于仅能在特定架构下运行的测试。编写能在 QEMU 或其他易于获取（且免费）的软件环境下运行的测试，优于针对特定硬件的测试。

尽管如此，仍有充分的理由编写架构或硬件特定的测试。例如，我们可能想测试真正属于 `arch/some-arch/*` 的代码。即便如此，也尽量编写不依赖于物理硬件的测试。我们的一些测试用例可能不需要硬件，只有少数测试真正需要硬件来测试。当硬件不可用时，与其禁用测试，我们可以跳过它们。

既然我们已经确切确定了哪些部分是硬件特定的，编写和运行这些测试的实际过程与编写普通 KUnit 测试相同。

   我们可能需要重置硬件状态。如果这不可能，我们可能只能在每次调用中运行一个测试用例。

   （依赖硬件的 KUnit 测试。）

## 常见模式


### 隔离行为


单元测试将待测代码的范围限制到单一单元。它控制在被测单元调用某个函数时运行哪些代码。当一个函数作为 API 的一部分暴露出来，使得该函数的定义可以在不影响代码库其余部分的情况下更改时，就属于这种情况。在内核中，这来自两种构造：类（class，即包含实现者提供的函数指针的结构体）和架构特定函数（其定义在编译时选定）。

#### 类


类并不是 C 编程语言内置的构造；然而，它是一个容易推导出的概念。因此，在大多数情况下，每个不使用标准化面向对象库（如 GNOME 的 GObject）的项目都有自己略微不同的面向对象编程方式；Linux 内核也不例外。

内核面向对象编程的核心概念是类（class）。在内核中，**类**是包含函数指针的结构体。这在**实现者（implementer）**和**使用者（user）**之间创建了一个契约，因为它强制它们使用相同的函数签名，而无需直接调用该函数。要成为一个类，函数指针必须指定一个指向该类的指针（称为**类句柄（class handle）**）作为参数之一。因此，成员函数（也称为**方法（method）**）可以访问成员变量（也称为**字段（field）**），使得同一个实现可以有多个**实例（instance）**。

类可以通过**子类（child class）**嵌入**父类（parent class）**来被**重写（override）**。然后，当调用子类的**方法**时，子类实现知道传递给它的指针是包含在子类中的父类。因此，子类可以计算出指向自身的指针，因为指向父类的指针与指向子类的指针之间总是存在固定的偏移量。这个偏移量就是父结构体在子结构体中包含的偏移量。例如：


	struct shape {
		int (**area)(struct shape **this);
	};

	struct rectangle {
		struct shape parent;
		int length;
		int width;
	};

	int rectangle_area(struct shape *this)
	{
		struct rectangle *self = container_of(this, struct rectangle, parent);

		return self->length * self->width;
	};

	void rectangle_new(struct rectangle *self, int length, int width)
	{
		self->parent.area = rectangle_area;
		self->length = length;
		self->width = width;
	}

在此示例中，从指向父类的指针计算指向子类的指针由 `container_of` 完成。

#### 伪类


为了对调用类中某个方法的代码进行单元测试，该方法的行为必须是可控的，否则测试就不再是单元测试，而变成了集成测试。

伪类（fake class）实现了一段与生产实例中运行的代码不同，但从调用者的角度看行为相同的代码。这样做是为了替换难以处理或速度较慢的依赖。例如，实现一个将"内容"存储在内部缓冲区中的伪 EEPROM。假设我们有一个表示 EEPROM 的类：


	struct eeprom {
		ssize_t (**read)(struct eeprom **this, size_t offset, char *buffer, size_t count);
		ssize_t (**write)(struct eeprom **this, size_t offset, const char *buffer, size_t count);
	};

我们想测试对 EEPROM 写入进行缓冲的代码：


	struct eeprom_buffer {
		ssize_t (**write)(struct eeprom_buffer **this, const char *buffer, size_t count);
		int flush(struct eeprom_buffer *this);
		size_t flush_count; /** Flushes when buffer exceeds flush_count. **/
	};

	struct eeprom_buffer **new_eeprom_buffer(struct eeprom **eeprom);
	void destroy_eeprom_buffer(struct eeprom *eeprom);

我们可以通过**伪化（faking out）**底层 EEPROM 来测试这段代码：


	struct fake_eeprom {
		struct eeprom parent;
		char contents[FAKE_EEPROM_CONTENTS_SIZE];
	};

	ssize_t fake_eeprom_read(struct eeprom **parent, size_t offset, char **buffer, size_t count)
	{
		struct fake_eeprom *this = container_of(parent, struct fake_eeprom, parent);

		count = min(count, FAKE_EEPROM_CONTENTS_SIZE - offset);
		memcpy(buffer, this->contents + offset, count);

		return count;
	}

	ssize_t fake_eeprom_write(struct eeprom **parent, size_t offset, const char **buffer, size_t count)
	{
		struct fake_eeprom *this = container_of(parent, struct fake_eeprom, parent);

		count = min(count, FAKE_EEPROM_CONTENTS_SIZE - offset);
		memcpy(this->contents + offset, buffer, count);

		return count;
	}

	void fake_eeprom_init(struct fake_eeprom *this)
	{
		this->parent.read = fake_eeprom_read;
		this->parent.write = fake_eeprom_write;
		memset(this->contents, 0, FAKE_EEPROM_CONTENTS_SIZE);
	}

我们现在可以用它来测试 `struct eeprom_buffer`：


	struct eeprom_buffer_test {
		struct fake_eeprom *fake_eeprom;
		struct eeprom_buffer *eeprom_buffer;
	};

	static void eeprom_buffer_test_does_not_write_until_flush(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;
		struct eeprom_buffer *eeprom_buffer = ctx->eeprom_buffer;
		struct fake_eeprom *fake_eeprom = ctx->fake_eeprom;
		char buffer[] = {0xff};

		eeprom_buffer->flush_count = SIZE_MAX;

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0);

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0);

		eeprom_buffer->flush(eeprom_buffer);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0xff);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0xff);
	}

	static void eeprom_buffer_test_flushes_after_flush_count_met(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;
		struct eeprom_buffer *eeprom_buffer = ctx->eeprom_buffer;
		struct fake_eeprom *fake_eeprom = ctx->fake_eeprom;
		char buffer[] = {0xff};

		eeprom_buffer->flush_count = 2;

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0);

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0xff);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0xff);
	}

	static void eeprom_buffer_test_flushes_increments_of_flush_count(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;
		struct eeprom_buffer *eeprom_buffer = ctx->eeprom_buffer;
		struct fake_eeprom *fake_eeprom = ctx->fake_eeprom;
		char buffer[] = {0xff, 0xff};

		eeprom_buffer->flush_count = 2;

		eeprom_buffer->write(eeprom_buffer, buffer, 1);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0);

		eeprom_buffer->write(eeprom_buffer, buffer, 2);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^0^], 0xff);
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^1^], 0xff);
		/** Should have only flushed the first two bytes. **/
		KUNIT_EXPECT_EQ(test, fake_eeprom->contents[^2^], 0);
	}

	static int eeprom_buffer_test_init(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx;

		ctx = kunit_kzalloc(test, sizeof(*ctx), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx);

		ctx->fake_eeprom = kunit_kzalloc(test, sizeof(*ctx->fake_eeprom), GFP_KERNEL);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx->fake_eeprom);
		fake_eeprom_init(ctx->fake_eeprom);

		ctx->eeprom_buffer = new_eeprom_buffer(&ctx->fake_eeprom->parent);
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx->eeprom_buffer);

		test->priv = ctx;

		return 0;
	}

	static void eeprom_buffer_test_exit(struct kunit *test)
	{
		struct eeprom_buffer_test *ctx = test->priv;

		destroy_eeprom_buffer(ctx->eeprom_buffer);
	}

### 针对多个输入进行测试


仅测试少数几个输入不足以确保代码正常工作，例如：测试哈希函数。

我们可以编写一个辅助宏或函数。该函数针对每个输入被调用。例如，要测试 `sha1sum(1)`，我们可以编写：


	#define TEST_SHA1(in, want) \
		sha1sum(in, out); \
		KUNIT_EXPECT_STREQ_MSG(test, out, want, "sha1sum(%s)", in);

	char out[^40^];
	TEST_SHA1("hello world",  "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed");
	TEST_SHA1("hello world!", "430ce34d020724ed75a196dfc2ad67c77772d169");

注意使用 `KUNIT_EXPECT_STREQ` 的 `_MSG` 版本来打印更详细的错误，并使辅助宏中的断言更清晰。

当同一个期望被多次调用（在循环或辅助函数中）时，`_MSG` 变体很有用，此时行号不足以识别是哪个失败，如下所示。

在复杂情况下，相比于辅助宏的变体，我们推荐使用**表驱动测试（table-driven test）**，例如：


	int i;
	char out[^40^];

	struct sha1_test_case {
		const char *str;
		const char *sha1;
	};

	struct sha1_test_case cases[] = {
		{
			.str = "hello world",
			.sha1 = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed",
		},
		{
			.str = "hello world!",
			.sha1 = "430ce34d020724ed75a196dfc2ad67c77772d169",
		},
	};
	for (i = 0; i < ARRAY_SIZE(cases); ++i) {
		sha1sum(cases[i].str, out);
		KUNIT_EXPECT_STREQ_MSG(test, out, cases[i].sha1,
		                      "sha1sum(%s)", cases[i].str);
	}

这涉及更多的样板代码，但它可以：

- 当有多个输入/输出时（得益于字段名）更具可读性。

  - 例如，参见 `fs/ext4/inode-test.c`。

- 如果测试用例在多个测试之间共享，可减少重复。

  - 例如：如果我们想测试 `sha256sum`，可以添加 `sha256` 字段并重用 `cases`。

- 可转换为"参数化测试"。

#### 参数化测试


为了让一个测试用例针对多个输入运行，KUnit 提供了参数化测试框架。该特性将前面讨论的表驱动测试概念形式化并进行了扩展。

如果在注册测试用例时提供了参数生成器函数，则 KUnit 测试被确定为参数化的。测试用户可以编写自己的生成器函数，也可以使用 KUnit 提供的生成器函数。生成器函数存储在 `kunit_case->generate_params` 中，可以使用下面小节中描述的宏进行设置。

为了建立术语，"参数化测试"是指一个运行多次（每个"参数"或"参数运行"运行一次）的测试。每个参数运行都有自己独立的 `struct kunit`（"参数运行上下文"），并且可以访问共享的父级 `struct kunit`（"参数化测试上下文"）。

##### 向测试传递参数

有三种方式向测试提供参数：

数组参数宏：

   KUnit 为常见的表驱动测试模式提供了特殊支持。通过对上一小节的 `cases` 数组应用 `KUNIT_ARRAY_PARAM` 或 `KUNIT_ARRAY_PARAM_DESC`，我们可以创建一个参数化测试，如下所示：


	// This is copy-pasted from above.
	struct sha1_test_case {
		const char *str;
		const char *sha1;
	};
	static const struct sha1_test_case cases[] = {
		{
			.str = "hello world",
			.sha1 = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed",
		},
		{
			.str = "hello world!",
			.sha1 = "430ce34d020724ed75a196dfc2ad67c77772d169",
		},
	};

	// Creates `sha1_gen_params()` to iterate over `cases` while using
	// the struct member `str` for the case description.
	KUNIT_ARRAY_PARAM_DESC(sha1, cases, str);

	// Looks no different from a normal test.
	static void sha1_test(struct kunit *test)
	{
		// This function can just contain the body of the for-loop.
		// The former `cases[i]` is accessible under test->param_value.
		char out[^40^];
		struct sha1_test_case **test_param = (struct sha1_test_case **)(test->param_value);

		sha1sum(test_param->str, out);
		KUNIT_EXPECT_STREQ_MSG(test, out, test_param->sha1,
				      "sha1sum(%s)", test_param->str);
	}

	// Instead of KUNIT_CASE, we use KUNIT_CASE_PARAM and pass in the
	// function declared by KUNIT_ARRAY_PARAM or KUNIT_ARRAY_PARAM_DESC.
	static struct kunit_case sha1_test_cases[] = {
		KUNIT_CASE_PARAM(sha1_test, sha1_gen_params),
		{}
	};

自定义参数生成器函数：

   生成器函数负责逐个生成参数，其签名如下：
   `const void** (**)(struct kunit **test, const void **prev, char *desc)`。
   你可以将生成器函数传递给 `KUNIT_CASE_PARAM` 或 `KUNIT_CASE_PARAM_WITH_INIT` 宏。

   该函数接收先前生成的参数作为 `prev` 参数（第一次调用时为 `NULL`），还可以访问作为
   `test` 参数传递的参数化测试上下文。KUnit 反复调用该函数，直到它返回 `NULL`，这表示
   参数化测试结束。

   下面是它如何工作的示例：


	#define MAX_TEST_BUFFER_SIZE 8

	// Example generator function. It produces a sequence of buffer sizes that
	// are powers of two, starting at 1 (e.g., 1, 2, 4, 8).
	static const void **buffer_size_gen_params(struct kunit **test, const void **prev, char **desc)
	{
		long prev_buffer_size = (long)prev;
		long next_buffer_size = 1; // Start with an initial size of 1.

		// Stop generating parameters if the limit is reached or exceeded.
		if (prev_buffer_size >= MAX_TEST_BUFFER_SIZE)
			return NULL;

		// For subsequent calls, calculate the next size by doubling the previous one.
		if (prev)
			next_buffer_size = prev_buffer_size << 1;

		return (void *)next_buffer_size;
	}

	// Simple test to validate that kunit_kzalloc provides zeroed memory.
	static void buffer_zero_test(struct kunit *test)
	{
		long buffer_size = (long)test->param_value;
		// Use kunit_kzalloc to allocate a zero-initialized buffer. This makes the
		// memory "parameter run managed," meaning it's automatically cleaned up at
		// the end of each parameter run.
		int **buf = kunit_kzalloc(test, buffer_size ** sizeof(int), GFP_KERNEL);

		// Ensure the allocation was successful.
		KUNIT_ASSERT_NOT_NULL(test, buf);

		// Loop through the buffer and confirm every element is zero.
		for (int i = 0; i < buffer_size; i++)
			KUNIT_EXPECT_EQ(test, buf[i], 0);
	}

	static struct kunit_case buffer_test_cases[] = {
		KUNIT_CASE_PARAM(buffer_zero_test, buffer_size_gen_params),
		{}
	};

在 init 函数中运行时注册参数数组：

   对于可能需要初始化参数化测试的场景，你可以直接将参数数组注册到参数化测试上下文中。

   为此，你必须将参数化测试上下文、数组本身、数组大小以及一个 `get_description()` 函数
   传递给 `kunit_register_params_array()` 宏。该宏填充参数化测试上下文中的
   `struct kunit_params`，有效地存储一个参数数组对象。`get_description()` 函数将用于
   填充参数描述，其签名如下：`void (**)(struct kunit **test, const void **param, char **desc)`。
   注意它也可以访问参数化测试上下文。

```
         When using this way to register a parameter array, you will need to
         manually pass ``kunit_array_gen_params()`` as the generator function to
         ``KUNIT_CASE_PARAM_WITH_INIT``. ``kunit_array_gen_params()`` is a KUnit
         helper that will use the registered array to generate the parameters.

	 If needed, instead of passing the KUnit helper, you can also pass your
	 own custom generator function that utilizes the parameter array. To
	 access the parameter array from within the parameter generator
	 function use ``test->params_array.params``.

   The ``kunit_register_params_array()`` macro should be called within a
   ``param_init()`` function that initializes the parameterized test and has
   the following signature ``int (*)(struct kunit *test)``. For a detailed
   explanation of this mechanism please refer to the "Adding Shared Resources"
   section that is after this one. This method supports registering both
   dynamically built and static parameter arrays.

   The code snippet below shows the ``example_param_init_dynamic_arr`` test that
   utilizes ``make_fibonacci_params()`` to create a dynamic array, which is then
   registered using ``kunit_register_params_array()``. To see the full code
   please refer to lib/kunit/kunit-example-test.c.

```

	/*
 - Example of a parameterized test param_init() function that registers a dynamic
 - array of parameters.
	*/
	static int example_param_init_dynamic_arr(struct kunit *test)
	{
		size_t seq_size;
		int *fibonacci_params;

		kunit_info(test, "initializing parameterized test\n");

		seq_size = 6;
		fibonacci_params = make_fibonacci_params(test, seq_size);
		if (!fibonacci_params)
			return -ENOMEM;
		/*
  - Passes the dynamic parameter array information to the parameterized test
  - context struct kunit. The array and its metadata will be stored in
  - test->parent->params_array. The array itself will be located in
  - params_data.params.
		*/
		kunit_register_params_array(test, fibonacci_params, seq_size,
					example_param_dynamic_arr_get_desc);
		return 0;
	}

	static struct kunit_case example_test_cases[] = {
		/*
   - Note how we pass kunit_array_gen_params() to use the array we
   - registered in example_param_init_dynamic_arr() to generate
   - parameters.
		 */
		KUNIT_CASE_PARAM_WITH_INIT(example_params_test_with_init_dynamic_arr,
					   kunit_array_gen_params,
					   example_param_init_dynamic_arr,
					   example_param_exit_dynamic_arr),
		{}
	};

##### 添加共享资源

本框架中的所有参数运行都持有一个对参数化测试上下文的引用，可通过父级 `struct kunit` 指针访问。参数化测试上下文本身并不用于执行任何测试逻辑；相反，它作为共享资源的容器。

可以通过使用 `KUNIT_CASE_PARAM_WITH_INIT` 来添加在参数化测试的各个参数运行之间共享的资源，你需要向它传递自定义的 `param_init()` 和 `param_exit()` 函数。这些函数分别在该参数化测试之前和之后各运行一次。

`param_init()` 函数签名为 `int (**)(struct kunit **test)`，可用于向参数化测试上下文的 `resources` 或 `priv` 字段添加资源、注册参数数组，以及任何其他初始化逻辑。

`param_exit()` 函数签名为 `void (**)(struct kunit **test)`，可用于释放任何非参数化测试管理的资源（即参数化测试结束时不会自动清理的资源），以及任何其他退出逻辑。

`param_init()` 和 `param_exit()` 都会在背后传入参数化测试上下文。然而，测试用例函数接收的是参数运行上下文。因此，要从测试用例函数中管理和访问共享资源，你必须使用 `test->parent`。

例如，查找由 Resource API 分配的共享资源需要将 `test->parent` 传递给 `kunit_find_resource()`。这一原则也适用于测试用例函数中可能使用的所有其他 API，包括 `kunit_kzalloc()`、`kunit_kmalloc_array()` 等（参见 Documentation/dev-tools/kunit/api/test.rst 和 Documentation/dev-tools/kunit/api/resource.rst）。

   `suite->init()` 函数在每个参数运行之前执行，它接收的是参数运行上下文。因此，在
   `suite->init()` 中设置的任何资源都会在每个参数运行之后被清理。

下面的代码展示了如何添加共享资源。注意此代码使用了 Resource API，你可以在以下位置阅读更多内容：Documentation/dev-tools/kunit/api/resource.rst。要查看此代码的完整版本，请参考 lib/kunit/kunit-example-test.c。


	static int example_resource_init(struct kunit_resource **res, void **context)
	{
		... /** Code that allocates memory and stores context in res->data. **/
	}

	/** This function deallocates memory for the kunit_resource->data field. **/
	static void example_resource_free(struct kunit_resource *res)
	{
		kfree(res->data);
	}

	/** This match function locates a test resource based on defined criteria. **/
	static bool example_resource_alloc_match(struct kunit **test, struct kunit_resource **res,
						 void *match_data)
	{
		return res->data && res->free == example_resource_free;
	}

	/** Function to initialize the parameterized test. **/
	static int example_param_init(struct kunit *test)
	{
		int ctx = 3; /** Data to be stored. **/
		void *data = kunit_alloc_resource(test, example_resource_init,
						  example_resource_free,
						  GFP_KERNEL, &ctx);
		if (!data)
			return -ENOMEM;
		kunit_register_params_array(test, example_params_array,
					    ARRAY_SIZE(example_params_array));
		return 0;
	}

	/** Example test that uses shared resources in test->resources. **/
	static void example_params_test_with_init(struct kunit *test)
	{
		int threshold;
		const struct example_param *param = test->param_value;
		/**  Here we pass test->parent to access the parameterized test context. **/
		struct kunit_resource *res = kunit_find_resource(test->parent,
								 example_resource_alloc_match,
								 NULL);

		threshold = **((int **)res->data);
		KUNIT_ASSERT_LE(test, param->value, threshold);
		kunit_put_resource(res);
	}

	static struct kunit_case example_test_cases[] = {
		KUNIT_CASE_PARAM_WITH_INIT(example_params_test_with_init, kunit_array_gen_params,
					   example_param_init, NULL),
		{}
	};

作为使用 KUnit Resource API 共享资源的替代方案，你可以将它们放入 `test->parent->priv`。这是一种更轻量级的资源存储方法，最适合不需要复杂资源管理的场景。

如前所述，`param_init()` 和 `param_exit()` 获取的是参数化测试上下文。因此，你可以在 `param_init/exit` 中直接使用 `test->priv` 来管理共享资源。但是，从测试用例函数内部，你必须向上导航到父级 `struct kunit`，即参数化测试上下文。因此，你需要使用 `test->parent->priv` 来访问这些相同的资源。

放置在 `test->parent->priv` 中的资源需要在内存中分配，以便在各个参数运行之间持续存在。如果使用 KUnit 内存分配 API（在下面的"分配内存"小节中有更多说明）分配内存，你就不必担心释放问题。这些 API 会使内存成为"参数化测试托管"的，确保在参数化测试结束后自动清理。

下面的代码演示了在共享资源中使用 `priv` 字段的示例：


	static const struct example_param {
		int value;
	} example_params_array[] = {
		{ .value = 3, },
		{ .value = 2, },
		{ .value = 1, },
		{ .value = 0, },
	};

	/** Initialize the parameterized test context. **/
	static int example_param_init_priv(struct kunit *test)
	{
		int ctx = 3; /** Data to be stored. **/
		int arr_size = ARRAY_SIZE(example_params_array);

		/*
   - Allocate memory using kunit_kzalloc(). Since the `param_init`
   - function receives the parameterized test context, this memory
   - allocation will be scoped to the lifetime of the parameterized test.
		 */
		test->priv = kunit_kzalloc(test, sizeof(int), GFP_KERNEL);

		/** Assign the context value to test->priv.**/
		**((int **)test->priv) = ctx;

		/** Register the parameter array. **/
		kunit_register_params_array(test, example_params_array, arr_size, NULL);
		return 0;
	}

	static void example_params_test_with_init_priv(struct kunit *test)
	{
		int threshold;
		const struct example_param *param = test->param_value;

		/** By design, test->parent will not be NULL. **/
		KUNIT_ASSERT_NOT_NULL(test, test->parent);

		/** Here we use test->parent->priv to access the shared resource. **/
		threshold = **(int **)test->parent->priv;

		KUNIT_ASSERT_LE(test, param->value, threshold);
	}

	static struct kunit_case example_tests[] = {
		KUNIT_CASE_PARAM_WITH_INIT(example_params_test_with_init_priv,
					   kunit_array_gen_params,
					   example_param_init_priv, NULL),
		{}
	};

### 分配内存


在你会使用 `kzalloc` 的地方，可以改用 `kunit_kzalloc`，因为 KUnit 会确保内存在测试完成后被释放。

这很有用，因为它让我们可以使用 `KUNIT_ASSERT_EQ` 宏在测试中提前退出，而无需担心记得调用 `kfree`。例如：


	void example_test_allocation(struct kunit *test)
	{
		char *buffer = kunit_kzalloc(test, 16, GFP_KERNEL);
		/** Ensure allocation succeeded. **/
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, buffer);

		KUNIT_ASSERT_STREQ(test, buffer, "");
	}

### 注册清理动作


如果你需要执行一些超出简单使用 `kunit_kzalloc` 的清理工作，可以注册一个自定义的"延迟动作（deferred action）"，它是在测试退出时运行的一个清理函数（无论是干净退出，还是因为断言失败退出）。

动作（action）是没有返回值、只有一个 `void*` 上下文参数的简单函数，它们扮演的角色与 Python 和 Go 测试中的"cleanup"函数、支持该特性的语言中的"defer"语句，以及（在某些情况下）RAII 语言中的析构函数相同。

这些对于从全局列表中注销某些内容、关闭文件或其他资源，或释放资源非常有用。

例如：


	static void cleanup_device(void *ctx)
	{
		struct device **dev = (struct device **)ctx;

		device_unregister(dev);
	}

	void example_device_test(struct kunit *test)
	{
		struct my_device dev;

		device_register(&dev);

		kunit_add_action(test, &cleanup_device, &dev);
	}

注意，对于像 device_unregister 这样只接受单个指针大小参数的函数，可以使用 `KUNIT_DEFINE_ACTION_WRAPPER()` 宏自动生成一个包装器，例如：


	KUNIT_DEFINE_ACTION_WRAPPER(device_unregister, device_unregister_wrapper, struct device *);
	kunit_add_action(test, &device_unregister_wrapper, &dev);

你应该优先这样做，而不是手动转换为 `kunit_action_t` 类型，因为转换函数指针会破坏控制流完整性（CFI）。

`kunit_add_action` 可能会失败，例如系统内存不足时。你可以改用 `kunit_add_action_or_reset`，它会在无法延迟执行时立即运行该动作。

如果你需要更多地控制清理函数的调用时机，可以使用 `kunit_release_action` 提前触发它，或使用 `kunit_remove_action` 完全取消它。


### 测试静态函数


如果你想测试静态函数，而又不想将这些函数暴露到测试之外，一种选择是有条件地导出符号。当启用 KUnit 时，该符号被暴露，否则保持静态。要使用此方法，请遵循以下模板。


	/** In the file containing functions to test "my_file.c" **/

	#include <kunit/visibility.h>
	#include <my_file.h>
	...
	VISIBLE_IF_KUNIT int do_interesting_thing()
	{
	...
	}
	EXPORT_SYMBOL_IF_KUNIT(do_interesting_thing);

	/** In the header file "my_file.h" **/

	#if IS_ENABLED(CONFIG_KUNIT)
		int do_interesting_thing(void);
	#endif

	/** In the KUnit test file "my_file_test.c" **/

	#include <kunit/visibility.h>
	#include <my_file.h>
	...
	MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
	...
	// Use do_interesting_thing() in tests

要查看完整示例，请参阅这个 `patch <https://lore.kernel.org/all/20221207014024.340230-3-rmoar@google.com/>`_，其中测试被修改为使用上述宏有条件地暴露静态函数以供测试。

作为上述方法的**替代方案**，你可以有条件地 `#include` 测试文件到 .c 文件的末尾。这不推荐，但在需要时有效。例如：


	/** In "my_file.c" **/

	static int do_interesting_thing();

	#ifdef CONFIG_MY_KUNIT_TEST
	#include "my_kunit_test.c"
	#endif

### 注入仅用于测试的代码


与上面所示类似，我们可以添加特定于测试的逻辑。例如：


	/** In my_file.h **/

	#ifdef CONFIG_MY_KUNIT_TEST
	/** Defined in my_kunit_test.c **/
	void test_only_hook(void);
	#else
	void test_only_hook(void) { }
	#endif

这种仅用于测试的代码可以通过访问当前的 `kunit_test` 变得更有用，如下一节所示：**访问当前测试**。

### 访问当前测试


在某些情况下，我们需要从测试文件之外调用仅用于测试的代码。这在提供函数的"模拟（mock）"实现，或从错误处理程序中使任何当前测试失败时很有帮助。我们可以通过 `task_struct` 中的 `kunit_test` 字段来实现，该字段可以使用 `kunit/test-bug.h` 中的 `kunit_get_current_test()` 函数访问。

`kunit_get_current_test()` 即使在未启用 KUnit 时调用也是安全的。如果未启用 KUnit，或者当前任务中没有运行测试，它将返回 `NULL`。它会被编译为无操作或静态键检查，因此在没有测试运行时对性能影响可以忽略不计。

下面的示例用它来实现 `foo` 函数的一个"模拟"实现：


	#include <kunit/test-bug.h> /** for kunit_get_current_test **/

	struct test_data {
		int foo_result;
		int want_foo_called_with;
	};

	static int fake_foo(int arg)
	{
		struct kunit *test = kunit_get_current_test();
		struct test_data *test_data = test->priv;

		KUNIT_EXPECT_EQ(test, test_data->want_foo_called_with, arg);
		return test_data->foo_result;
	}

	static void example_simple_test(struct kunit *test)
	{
		/* Assume priv (private, a member used to pass test data from
   - the init function) is allocated in the suite's .init */
		struct test_data *test_data = test->priv;

		test_data->foo_result = 42;
		test_data->want_foo_called_with = 1;

		/* In a real test, we'd probably pass a pointer to fake_foo somewhere
   - like an ops struct, etc. instead of calling it directly. */
		KUNIT_EXPECT_EQ(test, fake_foo(1), 42);
	}

在此示例中，我们使用 `struct kunit` 的 `priv` 成员作为从 init 函数向测试传递数据的一种方式。通常 `priv` 是一个可用于任何用户数据的指针。这比使用静态变量更受欢迎，因为它避免了并发问题。

如果我们想要更灵活一些，可以使用一个具名的 `kunit_resource`。每个测试可以有多个资源，它们具有字符串名称，提供了与 `priv` 成员相同的灵活性，而且例如还允许辅助函数创建资源而不会相互冲突。还可以为每个资源定义清理函数，从而轻松避免资源泄漏。更多信息，请参阅 Documentation/dev-tools/kunit/api/resource.rst。

### 使当前测试失败


如果我们想使当前测试失败，可以使用 `kunit_fail_current_test(fmt, args...)`，它定义在 `<kunit/test-bug.h>` 中，不需要引入 `<kunit/test.h>`。例如，我们有一个选项可以在某些数据结构上启用一些额外的调试检查，如下所示：


	#include <kunit/test-bug.h>

	#ifdef CONFIG_EXTRA_DEBUG_CHECKS
	static void validate_my_data(struct data *data)
	{
		if (is_valid(data))
			return;

		kunit_fail_current_test("data %p is invalid", data);

		/** Normal, non-KUnit, error reporting code here. **/
	}
	#else
	static void my_debug_function(void) { }
	#endif

`kunit_fail_current_test()` 即使在未启用 KUnit 时调用也是安全的。如果未启用 KUnit，或者当前任务中没有运行测试，它将什么都不做。它会被编译为无操作或静态键检查，因此在没有测试运行时对性能影响可以忽略不计。

### 管理模拟设备与驱动


在测试驱动或与驱动交互的代码时，许多函数将需要一个 `struct device` 或 `struct device_driver`。在许多情况下，测试某个给定函数并不需要设置一个真实的设备，因此可以使用一个模拟设备来代替。

KUnit 提供了用于创建和管理这些模拟设备的辅助函数，它们在内部是 `struct kunit_device` 类型，并挂载到一个特殊的 `kunit_bus` 上。这些设备支持托管的设备资源（devres），如 Documentation/driver-api/driver-model/devres.rst 所述。

要创建一个由 KUnit 托管的 `struct device_driver`，使用 `kunit_driver_create()`，它将在 `kunit_bus` 上创建一个具有给定名称的驱动。该驱动会在相应测试结束时自动销毁，但也可以使用 `driver_unregister()` 手动销毁。

要创建一个模拟设备，使用 `kunit_device_register()`，它将创建并注册一个设备，使用由 `kunit_driver_create()` 创建的新 KUnit 托管驱动。要提供特定的、非 KUnit 托管的驱动，请改用 `kunit_device_register_with_driver()`。与托管驱动一样，KUnit 托管的模拟设备会在测试结束时自动清理，但也可以使用 `kunit_device_unregister()` 提前手动清理。

在 `root_device_register()` 不适用的场景下，应优先使用 KUnit 设备；在设备并非 platform 设备的情况下，应优先使用 KUnit 设备而非 `platform_device_register()`。

例如：


	#include <kunit/device.h>

	static void test_my_device(struct kunit *test)
	{
		struct device *fake_device;
		const char *dev_managed_string;

		// Create a fake device.
		fake_device = kunit_device_register(test, "my_device");
		KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fake_device)

		// Pass it to functions which need a device.
		dev_managed_string = devm_kstrdup(fake_device, "Hello, World!");

		// Everything is cleaned up automatically when the test ends.
	}
