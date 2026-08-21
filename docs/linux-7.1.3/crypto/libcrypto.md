
## 加密库（Crypto library


`lib/crypto/` 提供比传crypto API 更快速、更便捷地访问加密算法的途径

加密算法的支持被放在专用的函数中加密敏捷（Crypto agility）在需要时交由调用方处理

crypto 库函数力求简单直接，并遵循约定俗成的规范。主要的文档（相当详尽）以内doc 形式提供；本页仅补充一些高层级的背景说明

需要说明的是，crypto 库并非全新事物。`lib/` 目录下自 2005 年起就包含加密函数。更准确地说，它只是把那些在实践中被证明有效的方式做了扩展。在很大程度上，它只是与内核中其它地方已有的做法保持一致


## 适用范围与目标读者（Scope / intended audience

crypto 库文档主要面向需要在内核代码中使用特定加密算法（算法集）的内核开发者。例我只需要计SHA-256 哈希"。次要受众是从事加密算法实现本身的开发者

如果你在寻找更通用的密码学信息（例如不同加密算法之间的差异、如何选择合适的算法），请参考外部资料，它们对这类信息覆盖得更为全面。如果你在为新内核特性选择算法时需要帮助，而该特性尚未预定义相应算法，请通过 `linux-crypto@vger.kernel.org` 寻求建议


## 代码组织（Code organization

- `lib/crypto/*.c`：加密算法实现

- `lib/crypto/$(SRCARCH)/`：加密算法的架构相关代码。之所以放在这里而非 `arch/` 的某个位置，部分是为了让通用的、架构优化的代码能够轻松地构建为单一可加载模块（当算法以 'm' 形式配置 kconfig 时）

- `lib/crypto/tests/`：加密算法的 KUnit 测试

- `include/crypto/`：加密头文件，供 crypto 库与传统 crypto API 使用

通常，一个内核模块对应一种算法。有时相关的算法会被归入同一个模块。这里刻意没有采用通用框架，尽管多个算法会共用一些工具函数

算法模块由一个三态（tristate）kconfig 符号 `CRYPTO_LIB_$(ALGORITHM)` 控制。通常库函数会被静态链接进内核，隐藏的符号不会出现kconfig 菜单中。相反，相关算法需要的其它 kconfig 符号会被自动选中

许多算法有多种实现：通用实现与架构优化实现。模块的初始化函数（built-in 情况下为 initcall）会根据可用CPU 特性自动启用最佳实现

需要说明的是，crypto 库不使用 `crypto/`、`arch/$(SRCARCH)/crypto/`、`drivers/crypto/` 这些目录——它们用于传crypto API。在可能的情况下，传crypto API 中的算法会通过调用库来实现


## 优势（Advantages

相比传统 crypto API，库的优势在于：

- 库函数往往更易使用。例如，计算哈希值只需一次函数调用。大多数库函数总是成功并返`void`，从而无需编写错误处理代码。大多数函数接受标准的虚拟地址，而非 scatterlist（后者既难用又低效）

- 库函数通常更快，尤其是对短输入。它们直接调用加密算法，避免了低效的间接调用、内存分配、字符串解析、算法注册表查找等不必要API 开销。架构优化代码默认启用

- 库函数使用标准的链接期依赖，而非易出错的、按名称动态加载的方式。无需通过强制内建相关模块、添加软依赖等手段来变通

- 库专注于在绝大多数系统上表现最佳的方式：基CPU 的加密算法实现，并利用可用的 CPU 加速（AES 指令）

- 库使用标准的 KUnit 测试，而非自定义的临时测试

- 库的加密算法实现往往具有更高的可靠性（assurance），因为其设计更简单，更多代码会被定期测试

- 库支持那些不适合传统 crypto API 僵硬框架的特性，例如交错哈希（interleaved hashing）与 XOF


## 使用（Usage

- 内核使用者应尽可能使用库（而非传统 crypto API）。许多子系统已经完成了迁移，通常能显著简化代码并改善性能

- 允许内核特性让用户通过传统 crypto API 的名称提供任意字符串来选择任意算法的特性，一般应继续使用传统 crypto API 以保持向后兼容

- 说明：新的内核特性不应支持每一种算法，而应审慎地选定所支持的算法（集）。历史表明，做出审慎、周到的选择能极大简化代码维护，降低出错概率（如使用已过时、不安全或不合适的算法），并使特性更易使用


## 测试（Testing

crypto 库使用标准的 KUnit 测试。与内核的许KUnit 测试一样，可在配置测试时通过以下命令运行

`tools/testing/kunit/kunit.py run --alltests`

还提供了 `.kunitconfig` 文件，用于仅运行 crypto 库的测试。例如，以下命令以用户Linux（User-Mode Linux）运行：

`tools/testing/kunit/kunit.py run --kunitconfig=lib/crypto/`

许多加密算法有架构优化实现。测试时需要构建适用于相应硬件、并运行测试的内核（例如通过 QEMU）。以下是一QEMU 示例

`tools/testing/kunit/kunit.py run --kunitconfig=lib/crypto/ --arch=arm64 --make_options LLVM=1`

根据被测代码的不同，可能需要向 QEMU 传递相应标志，以模拟代码所能触达的正确硬件类型

由于加密代码的正确性至关重要，新的架构优化代码在被接受前须QEMU 测试

说明：crypto 库包FIPS 140 自检。该自检很轻量，专门为满FIPS 140 要求而设计，其存在仅仅是为了满足该要求。常规测试则由内核开发者与集成者使用更为全面的 KUnit 测试套件来完成


## API 文档

- [libcrypto-blockcipher](libcrypto-blockcipher)
- [libcrypto-hash](libcrypto-hash)
- [libcrypto-signature](libcrypto-signature)
- [libcrypto-utils](libcrypto-utils)
- [sha3](sha3)
