
## 任务清单


任务可能包含以下字段
- `Complexity`（复杂度）：描述所需的对 Rust 或相应内API 或子系统的熟悉程度。共有四种复杂度：`Beginner`（初级）、`Intermediate`（中级）、`Advanced`（高级）`Expert`（专家）- `Reference`（引用）：对其他任务的引用- `Link`（链接）：外部资源的链接- `Contact`（联系人）：可以就任务进一步信息联系的人员
任务名称后面可能带有 `[ABCD]` 代码。该代码可用于在代码grep 与该任务相关`TODO` 条目
## 启用工作（Rust

这些任务并非直接关联 nova-core，而是所需 API 方面的前提条件
### FromPrimitive API [FPRI]


有时需要将一个数字转换为某个枚举或结构的值
nova-core 的一个好例子`Chipset` 枚举类型，它定义了`AD102`。在探测 GPU 时，可以从某个寄存器读出`0x192`，表明芯片组AD102。因此，枚举`AD102` 应当从数`0x192` 推导而来。目前，nova-core 为此使用了自定义*实现（`Chipset` : from_u32*
相比之下，更理想的是拥有类似 num crate 中的 `FromPrimitive` trait [^1^] 这样的东西
拥有这种泛化还有助于实现一个通用宏，自动生成值与数字之间的相应映射
过去曾对 FromPrimitive 支持做过工作，但此后便没有再跟进 [^1^]
也考虑ToPrimitive [^2^]
| Complexity: Beginner
| Link: https://docs.rs/num/latest/num/trait.FromPrimitive.html
| Link: https://lore.kernel.org/all/cover.1750689857.git.y.j3ms.n@gmail.com/ [^1^]
| Link: https://rust-for-linux.zulipchat.com/#narrow/channel/288089-General/topic/Implement.20.60FromPrimitive.60.20trait.20.2B.20derive.20macro.20for.20nova-core/with/541971854 [^2^]

### Numerical operations [NUMM]


Nova 使用了不属于标准库（或没有为内核做优化实现）的整数运算。这些包括：

- "查找最后置位比（Find Last Set Bit，内C 部分`fls` 函数）运算
一`num` 核心内核模块正在设计中，用于提供这些运算
| Complexity: Intermediate
| Contact: Alexandre Courbot

### Page abstraction for foreign pages


针对并非Rust 页抽象创建、且没有直接所有权的页Rust 抽象
Abdiel Janulgue [^1^] Lina [^2^] 正在进行积极的工作（active ongoing work）
| Complexity: Advanced
| Link: https://lore.kernel.org/linux-mm/20241119112408.779243-1-abdiel.janulgue@gmail.com/ [^1^]
| Link: https://lore.kernel.org/rust-for-linux/20250202-rust-page-v1-0-e3170d7fe55e@asahilina.net/ [^2^]

### PCI MISC APIs


通过 SR-IOV、capability、MSI API 抽象来扩展现有的 PCI 设备/驱动抽象
SR-IOV [^1^] 正在进行中
| Complexity: Beginner
| Link: https://lore.kernel.org/all/20251119-rust-pci-sriov-v1-0-883a94599a97@redhat.com/ [^1^]

## GPU（概述）


### Initial Devinit support


实现 BIOS 设备初始化，即内存大小确定、等待、PLL 配置
| Contact: Dave Airlie
| Complexity: Beginner

### MMU / PT management


设计 MMU / 页表管理的架构
我们需要考虑到，nova-drm 需要相当细粒度的控制，尤其是在锁方面，以便能够实现异步 Vulkan 队列
虽然通常共享相应代码是理想的，但需要评估共享相应代码是否（以及在何种程度上）合适
| Complexity: Expert

### VRAM memory allocator


研究 VRAM 内存分配器的各种选项
一些可能的选项  - RB 树（区间树）/ drm_mm Rust 抽象
  - maple_tree
  - 原生 Rust 集合

使用 drm_buddy [^1^] 的工作正在进行中
| Complexity: Advanced
| Link: https://lore.kernel.org/all/20251219203805.1246586-4-joelagnelf@nvidia.com/ [^1^]

### Instance Memory


实现对用于存储页表的 instmem（bar2）的支持
| Complexity: Intermediate
| Contact: Dave Airlie

## GPU System Processor (GSP)


### Export GSP log buffers


Timur Tabi [^1^] 近期的补丁增加了通过 debugfs 暴露 GSP-RM 日志缓冲区的支持（即使在驱动探测失败后也能暴露）
这对 nova-core 也是一个有趣的特性，尤其是在早期阶段
| Link: https://lore.kernel.org/nouveau/20241030202952.694055-2-ttabi@nvidia.com/ [^1^]
| Reference: Debugfs abstractions
| Complexity: Intermediate

### GSP firmware abstraction


GSP-RM 固件 API 不稳定，在数据结构和语义方面可能在不同版本之间发生不兼容的变化
这个问题nova-core 使用 Rust 的一大动机之一，因为事实证Rust 的过程宏（procedural macro）特性提供了一种相当优雅的方式来解决这一问题
1. C 头文件为每个版本在独立的命名空间中生Rust 结构
2. 构建实现固件接口的抽象结构（位于通用命名空间内）；用版本标识符标注实现的差异
3. 使用过程宏从该抽象生成实际的每个版本的实4. 在运行时实例化正确的版本类型（可以确信它们都有相同的接口，因为它由公trait 定义
nova-core PoC 驱动的环境中已有该模式的 PoC（概念验证）实现
该任务旨在完善该特性，并理想情况下将其泛化，以便其他驱动也能使用
| Complexity: Expert

### GSP message queue


实现底层GSP 消息队列（command、status），用于内核驱动GSP 之间的通信
| Complexity: Advanced
| Contact: Dave Airlie

### Bootstrap GSP


调用引导固件来启GSP 处理器；执行初始控制消息
| Complexity: Intermediate
| Contact: Dave Airlie

### Client / Device APIs


实现用于 client / device 分配GSP 消息接口，以及相应的 client device 分配 API
| Complexity: Intermediate
| Contact: Dave Airlie

### Bar PDE handling


同步内核驱动GSP 之间针对 BAR 的页表处理
| Complexity: Beginner
| Contact: Dave Airlie

### FIFO engine


实现FIFO 引擎的支持，即相应的 GSP 消息接口，并提供用于 chid 分配和通道处理API
| Complexity: Advanced
| Contact: Dave Airlie

### GR engine


实现对图形引擎的支持，即相应GSP 消息接口，并提供用于（golden）上下文创建和提升（promotion）的 API
| Complexity: Advanced
| Contact: Dave Airlie

### CE engine


实现对拷贝引擎的支持，即相应GSP 消息接口
| Complexity: Intermediate
| Contact: Dave Airlie

### VFN IRQ controller


VFN 中断控制器的支持
| Complexity: Intermediate
| Contact: Dave Airlie

## 外部 API


### nova-core base API


设计用于连接二级驱动（即 vGPU 管理器和 nova-drm）的 API 公共部分
| Complexity: Advanced

### vGPU manager API


设计 base API 未覆盖、但 vGPU 管理器所需API 部分
| Complexity: Advanced

### nova-core C API


vGPU 管理器驱动所需API 实现 C 包装器
| Complexity: Intermediate

## 测试


### CI pipeline


研究持续集成测试的选项
这可以从最简单的运行 KUnit 测试，到运行（图形）CTS，再到启动（多个）客户机 VM 来测VFIO 用例
也值得考虑引入一个直接位uAPI 之上的新测试套件，以进行更有针对性的测试和调试。可能存在与 Mesa 项目协作/共享代码的选项
| Complexity: Advanced
