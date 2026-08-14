## Compute Express Link 子系统成熟度地图

Linux CXL 子系统跟踪动态的 `CXL specification
<https://computeexpresslink.org/cxl-specification-landing-page>`_，该规范持续通过新功能、能力更新和修复来响应新的用例。在任意给定时刻，该子系统的某些方面比其他方面更成熟。虽然周期的拉取请求总结了每个合并窗口
`work being incorporated each merge window
<https://lore.kernel.org/linux-cxl/?q=s%3APULL+s%3ACXL+tc%3Atorvalds+NOT+s%3ARe>`_，
但它们并不总是能传达相对于起点和未来最终目标的进展情况。

下面是该子系统主要职责的粗略拆分，以及相应的成熟度评分。期望本文档的变更历史能够提供一个子系统随时间成熟的概览总结。

成熟度评分如下：

- [^3^] 成熟（Mature）：该领域的工作已完成，短期内没有变更。

  注意，根据新的测试结果或最终用户报告，该评分可能会从一个内核版本退化到下一个内核版本。

- [^2^] 稳定中（Stabilizing）：主要功能可运行，常见情况已成熟，但已知的边缘情况仍在处理中。

- [^1^] 初始（Initial）：已退出概念验证阶段的能力，但随着真实世界测试的进行，可能仍有显著的差距需要弥补、修复需要应用。

- [^0^] 已知缺口（Known gap）：该功能处于中期到长期才能实现的范围。

  如果规范中有一个功能在本文档中连 '0' 评分都没有，那么 linux-cxl@vger.kernel.org 社区中很可能没有人开始关注它。

- X：不在内核支持范围内，或不需要内核支持

## 功能与能力

### 枚举 / 配置

子系统所有基本的枚举和对象模型都已就位，但仍有若干边缘情况有待解决。

- [^2^] CXL 窗口枚举

  - [^2^] 扩展线性内存侧缓存 <extended-linear>
  - [^0^] 低内存空洞（Low Memory-hole）
  - [X] 异构交错（Hetero-interleave）

- [^2^] 交换机枚举

  - [^0^] CXL 寄存器枚举的链路建立依赖

- [^2^] HDM 解码器配置

  - [^0^] 解码器目标与粒度约束

- [^2^] 性能枚举

  - [^3^] 端点 CDAT
  - [^3^] 交换机 CDAT
  - [^1^] CDAT 到核心内存管理（Core-mm）的集成

    - [^1^] x86
    - [^0^] Arm64
    - [^0^] 所有其他架构

  - [^0^] 共享链路

- [^2^] 热插拔
  （见 CXL 窗口枚举）

  - [^0^] 处理 Soft Reserved 冲突

- [^0^] RCH 链路状态 <rch-link-status>
- [^0^] Fabrics / G-FAM（第 7 章）
- [^0^] 全局访问端点（Global Access Endpoint）

### RAS

在许多方面，CXL 可以被视为对通常由定制 EDAC 驱动处理之事的标准化。这里的开放式开发主要由上述的枚举边缘情况引起。

- [^3^] 组件事件（OS）
- [^2^] 组件事件（FFM）
- [^1^] 端点协议错误（OS）
- [^1^] 端点协议错误（FFM）
- [^0^] 交换机协议错误（OS）
- [^1^] 交换机协议错误（FFM）
- [^2^] DPA->HPA 地址转换

    - [^1^] XOR 交错转换
      （见 CXL 窗口枚举）

- [^1^] 内存故障（Memory Failure）协调
- [^0^] 清洗（Scrub）控制
- [^2^] ACPI 错误注入 EINJ

  - [^0^] EINJ v2
  - [X] 合规性 DOE

- [^2^] 原生错误注入
- [^3^] RCH 错误处理
- [^1^] VH 错误处理
- [^0^] PPR
- [^0^]  sparing（备用）
- [^0^] 设备内置自测

### 邮箱命令

- [^3^] 固件更新
- [^3^] 健康 / 告警
- [^1^] 后台命令 <background-commands>
- [^3^] 消毒（Sanitization）
- [^3^] 安全命令
- [^3^] RAW 命令调试透传
- [^0^] 仅 CEL 校验透传
- [^0^] 交换机 CCI
- [^3^] 时间戳
- [^1^] PMEM 标签
- [^3^] PMEM GPF / 脏关闭（Dirty Shutdown）
- [^0^] 扫描介质（Scan Media）

### PMU

- [^1^] Type 3 PMU
- [^0^] 交换机 USP/DSP、根端口

### 安全

- [X] CXL 可信执行环境安全协议（TSP）
- [X] CXL IDE（被 TSP 取代）

### 内存池化

- [^1^] LD 的热插拔（通过 PCI 热插拔）
- [^0^] 动态容量设备（DCD）支持

### 多主机共享

- [^0^] 硬件一致性共享内存
- [^0^] 软件管理一致性共享内存

### 多主机内存

- [^0^] 动态容量设备支持
- [^0^] 共享

### 加速器

- [^0^] 加速器内存枚举 HDM-D（CXL 1.1/2.0 Type-2）
- [^0^] 加速器内存枚举 HDM-DB（CXL 3.0 Type-2）
- [^0^] CXL.cache 68b（CXL 2.0）
- [^0^] CXL.cache 256b 缓存 ID（CXL 3.0）

### 用户流程支持

- [^2^] 按区域偏移注入并清除中毒（poison）

## 细节

- **扩展线性内存侧缓存**：一个 HMAT 提案，用于枚举一个内存侧缓存的存在，该缓存容量扩展了 SRAT 地址范围容量。`See the ECN
  <https://lore.kernel.org/linux-cxl/6650e4f835a0e_195e294a8@dwillia2-mobl3.amr.corp.intel.com.notmuch/>`_
  了解更多细节：

- **RCH 链路状态**：RCH（受限 CXL 主机，Restricted CXL Host）拓扑最终会把一些标准寄存器（如 PCIe 链路状态 / 能力）隐藏在 CXL RCRB（Root Complex Register Block，根复合体寄存器块）中。

- **后台命令**：CXL 后台命令机制比较尴尬，因为单个槽位可能被各种命令无限期地垄断。需要一个 `cancel on conflict
  <http://lore.kernel.org/r/66035c2e8ba17_770232948b@dwillia2-xfh.jf.intel.com.notmuch>`_
  设施，以确保内核能够保证优先级命令的向前推进。
