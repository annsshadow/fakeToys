
## ACPI _OSI _REV 方法


ACPI BIOS 可以使用"操作系统接口"（Operating System Interfaces）方法（_OSI）来查明操作系统支持哪些功能。例如，如果 BIOS AML 代码包含 _OSI("XYZ")，内核的 AML 解释器就可以求值该方法，查看它是否支持 'XYZ'，并BIOS 回答 YES NO
ACPI _REV 方法返回"OSPM 所支持ACPI 规范版本"（Revision of the ACPI specification that OSPM supports）
本文档说明了 BIOS Linux 应当如何使用这些方法，以及为何如此使用。同时它也解释了这些方法为何被普遍误用
## 如何使用 _OSI


Linux 运行于两类机器之上——一类是 OEM 测试过与 Linux 兼容的机器，另一类从未用 Linux 测试过，只是安装Linux 来取代原有的操作系统（Windows OSX）
数量更大的一组是仅测试过运行 Windows 的系统。不仅如此，其中许多只测试过运行某一个特定版本的 Windows。因此，即便 BIOS 可能使用 _OSI 来查询正在运行的是哪个版本的 Windows，实际上 BIOS 中只有一条路径被真正测试过。经验表明，BIOS 中未经测试的路径会让 Linux 暴露在一整类 BIOS bug 之中。出于这个原因，Linux _OSI 默认值必须继续声称与所有版本的 Windows 兼容
Linux 实际上并不与 Windows 兼容，而且Linux 把最新版本的 Windows 加入_OSI 字符串列表时，Linux 社区也曾因为回归问题而受到损害。因此，未来在合入上游之前，额外的字符串可能会被更彻底地审查。但它们很可能最终都会被加入
如果一OEM 想用同一BIOS 镜像同时支持 Linux Windows，应该怎么做？通常他们需要为 Linux 做不同的处理，以应对 Linux Windows 的差异
在这种情况下，OEM 应当创建Linux 内核执行的自定义 ASL，并修改 Linux 内核驱动来执行该自定ASL。实现这一点最简单的方式是引入一个由 Linux 内核调用的设备专用方法（_DSM）
过去内核曾经支持类似这样的写法：_OSI("Linux-OEM-my_interface_name")，其中若这是一OEM 专用钩子，则需'OEM'，'my_interface_name' 描述该钩子，它可能是某个怪癖（quirk）、bug bug 修复
然而，人们发现它被其他 BIOS 厂商滥用，用来在完全不相干的系统中修改完全无关的代码。这促使社区对其所有用途进行了评估。评估发现，原先使用这些字符串的理由都已不再需要。因此，内核默认不再对任何自定义Linux-* 字符串作出响应
这很简单。继续读下去，看看如何把它用错
## _OSI 之前，曾_OS


ACPI 1.0 "_OS" 规定求值为一个用于标识操作系统的字符串的对象"（object that evaluates to a string that identifies the operating system）
ACPI BIOS 的流程会包含_OS 的求值，内核中的 AML 解释器会向它返回一个标识该操作系统的字符串
Windows 98, SE: "Microsoft Windows"
Windows ME: "Microsoft WindowsME:Millennium Edition"
Windows NT: "Microsoft Windows NT"

其设计理念是：在一个需要运行多个操作系统的平台上，BIOS 可以使用 _OS 来启用某个操作系统可能支持的 devices，或者启用使平台与既有操作系统兼容所需的怪癖bug 变通方案
_OS 存在根本性问题。首先，BIOS 需要知道所有可能运行在它之上的操作系统版本的名称，还要知道这些操作系统的所有怪癖。显然，BIOS 向操作系统询*具体**的事情（例如"你支持某个特定接口吗"）更有意义，于是 ACPI 3.0 中诞生了 _OSI 来取_OS
_OS 已被废弃，不过即便到今天，许BIOS 仍在查找 _OS "Microsoft Windows NT"，尽管任何人用这些旧操作系统覆盖机器自带系统似乎不太现实
Linux 回答 "Microsoft Windows NT" 以迎合那BIOS 习惯用法。这*唯一**可行的策略，因为现代 Windows 正是这样做的，否则可能把 BIOS 引向未经测试的路径
## _OSI 诞生，随即被误用


通过 _OSI*BIOS** 提供描述某个接口的字符串，并询问操作系统YES/NO，你是否与该接口兼容

例如，如果操作系统知道如何处ACPI 3.0 规范中新增的热相关扩展，_OSI("3.0 Thermal Model") 就会返回 TRUE。不知道这些扩展的旧操作系统会回FALSE，而新操作系统则可能返TRUE
对于特定于某个操作系统的接口，ACPI 规范规定 BIOS 与操作系统应约定一个形"Windows-interface_name" 的字符串
但两件事出了差错。首先，Windows 生态并没有按设计使_OSI，而是把它直接当作 _OS 的替代品——用来标识操作系统版本，而非操作系统所支持的接口。事实上，从一开始，ACPI 3.0 规范本身就在示例代码中使_OSI("Windows 2001") 把这种误用法固化了下来
这种误用法被沿用至今
Linux 别无选择，只能同样对 _OSI("Windows 2001") 及其后继版本返回 TRUE。否则几乎必然会导致一个仅在该 _OSI 返回 TRUE 时被测试过的 BIOS 出现故障
这一策略是有问题的，因为 Linux 从未与最新版本的 Windows 完全兼容，有时需要一年以上的时间才能消除不兼容问题
为了不落人后，Linux 社区通过_OSI("Linux") 返回 TRUE 把事情搞得更糟。这样做Windows _OSI 的误用还要糟糕，因为 "Linux" 甚至不包含任何版本信息。_OSI("Linux") 导致一BIOS 出现故障，因BIOS 编写者在未经测试BIOS 流程中使用了它。但也有一OEM 在已测试的流程中使用 _OSI("Linux") 来支持真正的 Linux 特性009 年，Linux 移除_OSI("Linux")，并新增了一个命令行参数，以供仍需要它的遗留系统恢复该行为。此外，对于所有调用它BIOS 都会打印 BIOS_BUG 警告
任何 BIOS 都不应使_OSI("Linux")
由此形成了一套让 Linux 最大化与那些在 Windows 机器上测试过ACPI BIOS 兼容性的策略。这里确实存在高估兼容性的真实风险；但另一种选择往往是灾难性的失败——因BIOS 走了**任何**操作系统都从未验证过的路径
## 不要使用 _REV


自从 _OSI("Linux") 被移除后，一BIOS 编写者改_REV 来在同一BIOS 中区Linux Windows 的差异
_REV ACPI 1.0 中定义，用于返回操作系统及其 AML 解释器所支持ACPI 版本
现代 Windows 返回 _REV = 2。Linux 曾经使用 ACPI_CA_SUPPORT_LEVEL，它会随着所支持规范版本的提升而递增
不幸的是，_REV 也被误用了。例如，某些 BIOS 会检_REV = 3 并为 Linux 做些处理，但Linux 返回 _REV = 4 时，那种支持就失效了
为应对这一问题，从 2015 年中开始，Linux 始终返回 _REV = 2。ACPI 规范也将更新，以反映 _REV 已被弃用并始终返2
## Apple Mac 涓?_OSI("Darwin")


Apple Mac 平台上，ACPI BIOS 调用 _OSI("Darwin") 来判断机器是否运Apple OSX
Linux _OSI("**Windows**") 策略类似，Linux 默认_OSI("Darwin") 回答 YES，以启用对硬件的完全访问以及 OSX 所能看到的、经过验证的 BIOS 路径。正如在Windows 测试的平台上一样，这一策略也存在风险
Linux-3.18 开始，内核_OSI("Darwin") 回答 YES，目的是启用 Mac Thunderbolt 支持。此外，如果内核注意_OSI("Darwin") 被调用，它会额外禁用所_OSI("**Windows**")，以免编写拙劣的 Mac BIOS 走入未经测试的路径组合
Linux-3.18 对默认值的改动Mac 笔记本上引发了功耗回归，而且 3.18 的实现不允许通过命令"acpi_osi=!Darwin" 修改默认值。Linux-4.7 修复了可以使acpi_osi=!Darwin 作为变通方案的问题，我们希望在 Linux-4.11 中看Mac Thunderbolt 电源管理支持