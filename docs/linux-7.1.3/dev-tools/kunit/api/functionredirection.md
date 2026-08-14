
## 函数重定向 API


## 概述


编写单元测试时，能够将待测代码与内核其它部分隔离开来是很重要的。这保证了测试的可靠性
（不会受外部因素影响）、减少对特定硬件或配置选项的依赖（使测试更容易运行），并保护
系统其余部分的稳定性（降低测试特定状态干扰系统其余部分的可能性）。

对于某些代码（通常是通用数据结构、辅助函数以及其它"纯函数"）来说这很容易，但对于
其它代码（如设备驱动、文件系统、核心子系统）来说，代码与内核其它部分高度耦合。

这种耦合往往以某种方式源于全局状态：无论是设备的全局列表、文件系统，还是某些硬件状态。
测试需要小心地管理、隔离和恢复状态，或者也可以通过用"假"（fake）或"模拟"（mock）
变体替换对该状态的访问和修改来完全避开它。

通过重构对此类状态的访问（例如引入一层间接层，该层可以使用或模拟一组独立的测试状态）
也能做到。然而，这样的重构本身有代价（在能写测试之前进行重大重构并不理想）。

一种更简单的拦截和替换某些函数调用的方式是使用基于静态桩（static stub）的函数重定向。


## 静态桩


静态桩是一种将一个函数（"真实"函数）的调用重定向到另一个函数（"替换"函数）的方法。

它的工作原理是向"真实"函数中添加一个宏，该宏检查是否有测试正在运行，以及是否存在可用的
替换函数。如果存在，就会调用该函数以代替原函数。

使用静态桩相当直接：

1. 将 KUNIT_STATIC_STUB_REDIRECT() 宏添加到"真实"函数的开头。

   这应该是函数中紧随任何变量声明之后的第一条语句。KUNIT_STATIC_STUB_REDIRECT() 接受
   函数名，后跟传递给真实函数的所有参数。

   例如：

   .. code-block:: c

   void send_data_to_hardware(const char *str)
   {
   	KUNIT_STATIC_STUB_REDIRECT(send_data_to_hardware, str);
   	/** 真实实现 **/
   }

2. 编写一个或多个替换函数。

   这些函数应当具有与真实函数相同的函数签名。如果它们需要访问或修改测试特定状态，可以
   使用 kunit_get_current_test() 获取一个 struct kunit 指针。然后可以将其传给期望/断言
   宏，或用于查找 KUnit 资源。

   例如：

   .. code-block:: c

   void fake_send_data_to_hardware(const char *str)
   {
   	struct kunit *test = kunit_get_current_test();
   	KUNIT_EXPECT_STREQ(test, str, "Hello World!");
   }

3. 从你的测试中激活静态桩。

   在测试内部，可以使用 kunit_activate_static_stub() 启用重定向，它接受一个 struct kunit
   指针、真实函数和替换函数。你可以用不同的替换函数多次调用它，以交换该函数的实现。

   在我们的例子中，这将是

   .. code-block:: c

   kunit_activate_static_stub(test,
   			   send_data_to_hardware,
   			   fake_send_data_to_hardware);

4. 调用（可能是间接地）真实函数。

   一旦激活了重定向，任何对真实函数的调用都会改为调用替换函数。此类调用可能深埋在另一个
   函数的实现中，但必须来自测试的 kthread。

   例如：

   .. code-block:: c

   send_data_to_hardware("Hello World!"); /** 成功 **/
   send_data_to_hardware("Something else"); /** 测试失败。 **/

5. （可选）禁用该桩。

   当不再需要它时，使用 kunit_deactivate_static_stub() 禁用重定向（从而恢复"真实"函数的
   原始行为）。否则，它会在测试退出时自动禁用。

   例如：

   .. code-block:: c

   kunit_deactivate_static_stub(test, send_data_to_hardware);

也可以利用这些替换函数来测试某个函数是否被调用过，例如：

   void send_data_to_hardware(const char *str)
   {
   	KUNIT_STATIC_STUB_REDIRECT(send_data_to_hardware, str);
   	/** 真实实现 **/
   }

   /** 在测试文件中 **/
   int times_called = 0;
   void fake_send_data_to_hardware(const char *str)
   {
   	times_called++;
   }
   ...
   /** 在测试用例中，在测试期间重定向调用 **/
   kunit_activate_static_stub(test, send_data_to_hardware, fake_send_data_to_hardware);

   send_data_to_hardware("hello");
   KUNIT_EXPECT_EQ(test, times_called, 1);

   /** 如果需要，也可以提前停用该桩 **/
   kunit_deactivate_static_stub(test, send_data_to_hardware);


   send_data_to_hardware("hello again");
   KUNIT_EXPECT_EQ(test, times_called, 1);


## API 参考

   :internal:

	send_data_to_hardware("hello");
	KUNIT_EXPECT_EQ(test, times_called, 1);

	/** Can also deactivate the stub early, if wanted **/
	kunit_deactivate_static_stub(test, send_data_to_hardware);

	send_data_to_hardware("hello again");
	KUNIT_EXPECT_EQ(test, times_called, 1);



## API Reference


   :internal:
