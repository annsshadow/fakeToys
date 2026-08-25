## 对象生命周期调试基础设施


:Author: Thomas Gleixner

## 简

debugobjects 是一个通用的基础设施，用于跟踪内核对象的生命周期，并验证针对这些对象的操作
debugobjects 可用于检查以下错误模式：

- 激活未初始化的对象

- 初始化活跃（active）对
- 使用已释已销毁的对象

debugobjects 不会改变真实对象的数据结构，因此可以以最小的运行时开销编译进内核，并通过内核命令行选项按需启用
## 如何使用 debugobjects


一个内核子系统需要提供一个描述对象类型的数据结构，并在适当的位置加入对调试代码的调用。描述对象类型的数据结构至少需要对象类型的名称。可以提供（并且应当提供）可选函数，用于修复检测到的问题，这样内核就能继续工作，并且可以从一个运行中的系统获取调试信息，而不必通过串行控制台和显示器上的栈跟踪记录来进行硬核调试
debugobjects 提供的调试调用有
- debug_object_init

- debug_object_init_on_stack

- debug_object_activate

- debug_object_deactivate

- debug_object_destroy

- debug_object_free

- debug_object_assert_init

这些函数中的每一个都接受真实对象的地址，以及一个指向对象类型特定的调试描述结构的指针
检测到的每个错误都会被记录到统计信息中，并且有数量限制的错误会printk 输出，包括完整的栈跟踪
统计信息可通过 /sys/kernel/debug/debug_objects/stats 获取。它们提供了关于警告数量、成功修复数量的信息，以及关于内部跟踪对象的使用情况和内部跟踪对象池状态的信息
## 调试函数


   :functions: debug_object_init

每当真实对象的初始化函数被调用时，都会调用此函数
当真实对象已经被 debugobjects 跟踪时，会检查该对象是否可以被初始化。对于活跃和已销毁的对象，不允许进行初始化。当 debugobjects 检测到错误时，如果调用者提供了对象类型描述结构fixup_init 函数，它就会调用该函数。fixup 函数可以在真实对象初始化之前纠正问题。例如，它可以停用一个活跃对象，以防止对子系统造成损害
当真实对象尚未被 debugobjects 跟踪时，debugobjects 会为真实对象分配一个跟踪器对象，并将跟踪器对象的状态设置为 ODEBUG_STATE_INIT。它会验证该对象不在调用者的栈上。如果它在调用者的栈上，则会输出有限数量的警告（包括完整的栈跟踪）。调用代码必须使debug_object_init_on_stack()，并在离开分配该对象的函数之前将其移除。见下一节
   :functions: debug_object_init_on_stack

每当位于栈上的真实对象的初始化函数被调用时，都会调用此函数
当真实对象已经被 debugobjects 跟踪时，会检查该对象是否可以被初始化。对于活跃和已销毁的对象，不允许进行初始化。当 debugobjects 检测到错误时，如果调用者提供了对象类型描述结构fixup_init 函数，它就会调用该函数。fixup 函数可以在真实对象初始化之前纠正问题。例如，它可以停用一个活跃对象，以防止对子系统造成损害
当真实对象尚未被 debugobjects 跟踪时，debugobjects 会为真实对象分配一个跟踪器对象，并将跟踪器对象的状态设置为 ODEBUG_STATE_INIT。它会验证该对象位于调用者的栈上
位于栈上的对象必须在分配该对象的函数返回之前，通过调用 debug_object_free() 将其从跟踪器中移除。否则我们会继续跟踪过时的对象
   :functions: debug_object_activate

每当真实对象的激活函数被调用时，都会调用此函数
当真实对象已经被 debugobjects 跟踪时，会检查该对象是否可以被激活。对于活跃和已销毁的对象，不允许进行激活。当 debugobjects 检测到错误时，如果调用者提供了对象类型描述结构fixup_activate 函数，它就会调用该函数。fixup 函数可以在真实对象激活之前纠正问题。例如，它可以停用一个活跃对象，以防止对子系统造成损害
当真实对象尚未被 debugobjects 跟踪时，如果可用，则会调fixup_activate 函数。这对于允许合法地激活静态分配并初始化的对象是必需的。fixup 函数检查对象是否有效，并调debug_objects_init() 函数来初始化对该对象的跟踪
当激活合法时，相关联的跟踪器对象的状态被设置ODEBUG_STATE_ACTIVE

   :functions: debug_object_deactivate

每当真实对象的停用函数被调用时，都会调用此函数
当真实对象被 debugobjects 跟踪时，会检查该对象是否可以被停用。对于未被跟踪或已销毁的对象，不允许进行停用
当停用合法时，相关联的跟踪器对象的状态被设置ODEBUG_STATE_INACTIVE
   :functions: debug_object_destroy

调用此函数以标记一个对象已被销毁。这对于防止使用内存中仍然可用但已无效的对象很有用：无论是静态分配的对象，还是稍后才会被释放的对象
当真实对象被 debugobjects 跟踪时，会检查该对象是否可以被销毁。对于活跃和已销毁的对象，不允许进行销毁。当 debugobjects 检测到错误时，如果调用者提供了对象类型描述结构fixup_destroy 函数，它就会调用该函数。fixup 函数可以在真实对象销毁之前纠正问题。例如，它可以停用一个活跃对象，以防止对子系统造成损害
当销毁合法时，相关联的跟踪器对象的状态被设置ODEBUG_STATE_DESTROYED
   :functions: debug_object_free

在对象被释放之前会调用此函数
当真实对象被 debugobjects 跟踪时，会检查该对象是否可以被释放。对于活跃对象，不允许进行释放。当 debugobjects 检测到错误时，如果调用者提供了对象类型描述结构fixup_free 函数，它就会调用该函数。fixup 函数可以在真实对象释放之前纠正问题。例如，它可以停用一个活跃对象，以防止对子系统造成损害
注意，debug_object_free 会将对象从跟踪器中移除。之后对该对象的使用会被其他调试检查检测到

   :functions: debug_object_assert_init

调用此函数以断言一个对象已经被初始化
当真实对象未debugobjects 跟踪时，它会以硬编码的对象状ODEBUG_NOT_AVAILABLE 调用调用者提供的对象类型描述结构中的 fixup_assert_init。fixup 函数可以通过调用 debug_object_init 和其他特定的初始化函数来纠正问题
当真实对象已经被 debugobjects 跟踪时，则忽略它
## 修复（fixup）函

### 调试对象类型描述结构


   :internal:

### fixup_init


当在 debug_object_init 中检测到问题时，会从调试代码中调用此函数。该函数接受对象的地址以及当前记录在跟踪器中的状态
当对象状态为以下情况时，debug_object_init 调用
- ODEBUG_STATE_ACTIVE

当修复成功时，函数返true，否则返false。返回值被用于更新统计信息
注意，在损害被修复之后，该函数需要再次调debug_object_init() 函数，以保持状态一致
### fixup_activate


当在 debug_object_activate 中检测到问题时，会从调试代码中调用此函数
当对象状态为以下情况时，debug_object_activate 调用
- ODEBUG_STATE_NOTAVAILABLE

- ODEBUG_STATE_ACTIVE

当修复成功时，函数返true，否则返false。返回值被用于更新统计信息
注意，在损害被修复之后，该函数需要再次调debug_object_activate() 函数，以保持状态一致
静态初始化对象的激活是一种特殊情况。当 debug_object_activate() 没有该对象地址对应的被跟踪对象时，就会以对象状ODEBUG_STATE_NOTAVAILABLE 调用 fixup_activate()。fixup 函数需要检查这是否是静态初始化对象的合法情况。如果是，它会调debug_object_init() debug_object_activate() 让跟踪器知晓该对象并将其标记为活跃。在这种情况下，函数应当返回 false，因为这不是一次真正的修复
### fixup_destroy


当在 debug_object_destroy 中检测到问题时，会从调试代码中调用此函数
当对象状态为以下情况时，debug_object_destroy 调用
- ODEBUG_STATE_ACTIVE

当修复成功时，函数返true，否则返false。返回值被用于更新统计信息
### fixup_free


当在 debug_object_free 中检测到问题时，会从调试代码中调用此函数。此外，debug_check_no_obj_freed() 健全性检查发现活跃对象时，它也可以从 kfree/vfree 中的调试检查被调用
当对象状态为以下情况时，debug_object_free() debug_check_no_obj_freed() 调用
- ODEBUG_STATE_ACTIVE

当修复成功时，函数返true，否则返false。返回值被用于更新统计信息
### fixup_assert_init


当在 debug_object_assert_init 中检测到问题时，会从调试代码中调用此函数
当在调试桶中找不到该对象时，debug_object_assert_init() 以硬编码状ODEBUG_STATE_NOTAVAILABLE 调用
当修复成功时，函数返true，否则返false。返回值被用于更新统计信息
注意，此函数应当确保在返回之前调用了 debug_object_init()
静态初始化对象的处理是一种特殊情况。fixup 函数应当检查这是否是静态初始化对象的合法情况。在这种情况下只应调debug_object_init() 让跟踪器知晓该对象。然后函数应当返false，因为这不是一次真正的修复
## 已知缺陷与假

无（但愿如此）