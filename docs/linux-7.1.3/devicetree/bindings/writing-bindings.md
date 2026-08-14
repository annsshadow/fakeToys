
## 设计和编写 Devicetree 绑定的应当做与不应当做


这是一份聚焦于绑定设计的常见评审反馈条目清单。每条规则都有例外，绑定也存在许多
灰色地带。

关于补丁相关的指南，请参见
Documentation/devicetree/bindings/submitting-patches.rst


## 总体设计


- 应当（DO）尝试使绑定完整，即使驱动不支持某些特性。例如，如果设备有中断，那么
  即使驱动仅处于轮询模式，也应包含 'interrupts' 属性。

- 不要（DON'T）在绑定中提及 Linux 或"device driver"（设备驱动）。绑定应当基于
  硬件所具有的东西，而不是基于某个 OS 和驱动当前支持的东西。

- 应当（DO）使用与设备类别相匹配的节点名。DT 规范中定义了许多少标准名称。如果
  没有，考虑添加它。

- 应当（DO）检查示例是否与文档相符，尤其是在做了评审修改之后。

- 不要（DON'T）仅仅为了实例化驱动而创建节点。多功能设备仅当子节点拥有自己的 DT
  资源时才需要子节点。单个节点可以是多个提供者（例如时钟和复位）。

- 不要（DON'T）将设备节点名视为稳定的 ABI，而应使用 phandle 或 compatibles 来
  查找兄弟设备。例外：给定设备的子节点可以被视为 ABI，前提是在绑定中显式说明。

- 不要（DON'T）单独使用 'syscon' 而不带特定的 compatible 字符串。一个 'syscon'
  硬件块应当拥有一个足够唯一的 compatible 字符串，以便（至少）推断出整个块的寄存器
  布局。

- 不要（DON'T）对非平凡设备使用 'simple-mfd' compatible，即子节点依赖父节点的某些
  资源的情况。类似地，'simple-bus' 不应被用于复杂的总线，甚至 'regs' 属性的存在即
  意味着该设备不是简单总线。


## 属性


- 应当（DO）使 'compatible' 属性具体明确。

   - 不要（DON'T）在 compatible 字符串中使用通配符或设备族名称。

   - 应当（DO）在设备与先前实现相同或为其超集时使用回退（fallback）compatibles。

   - 应当（DO）在有新特性或缺陷时添加新的 compatibles。

   - 应当（DO）对所有 SoC 设备使用 SoC 特定的 compatible，若合适则后跟一个回退。
    SoC 特定的 compatibles 也优先用于回退。

   - 不要（DON'T）使用总线后缀来编码设备所使用的接口类型。父总线节点已经暗示了
     该接口。如果设备不可能是其它任何东西，不要添加设备类型。

- 应当（DO）在设备特定的属性名上使用厂商前缀。考虑这些属性是否可能被同类设备
  共用。查阅其它现有绑定以寻找类似设备。

- 不要（DON'T）重新定义通用属性。只需引用其定义，并定义设备特定的约束。

- 不要（DON'T）为了避免某个特定 compatible 而添加属性。如果属性是由 compatible
  暗示（可从 compatible 推导）出来的，不要添加属性。

- 应当（DO）对带有科学单位（scientific units）的属性使用通用属性的单位后缀。推荐
  的后缀列于
  https://github.com/devicetree-org/dt-schema/blob/main/dtschema/schemas/property-units.yaml

- 应当（DO）以约束的方式定义属性。多少个条目？可能的值有哪些？顺序如何？所有这些
  约束同样代表了 ABI。

- 不要（DON'T）在缺乏为何必须做出更改及其影响的明确而详细理由的情况下，做出破坏
  ABI 的更改。ABI 影响超出了 Linux 内核，因为它也涵盖其它开源上游项目。


## 典型情况与注意事项


- Phandle 条目，如 clocks/dmas/interrupts/resets，应当总是显式排序。如果有多于一个
  phandle，则包含 {clock,dma,interrupt,reset}-names。使用时，这两个字段都需要相同
  的约束（例如条目列表）。

- 对于 {clock,dma,interrupt,reset}-names 中使用的名称，不要添加任何后缀，例如：
  对于中断使用 "tx" 而不是 "txirq"。

- 没有 schema 类型（例如没有标准后缀或未由 schema 定义）的属性需要类型，即使这
  是一个枚举。

- 如果 schema 包含其它 schema（例如 /schemas/i2c/i2c-controller.yaml），使用
  "unevaluatedProperties:false"。其它情况下，通常使用 "additionalProperties:false"。

- 对于更大设备（例如 SoC 块）的子块/组件，使用基于设备的 compatible（例如基于
  SoC 的 compatible），而不是该组件的自定义版本化。例如使用 "vendor,soc1234-i2c"
  而不是 "vendor,i2c-v2"。

- "syscon" 不是一个通用属性。使用厂商和类型，例如 "vendor,power-manager-syscon"。

- 不要添加实例索引（ID）属性或自定义的 OF aliases。如果设备有不同的编程模型，它们
  可能需要不同的 compatibles。如果此类设备以不同方式使用其它设备（例如它们以不同
  方式编程 phy），使用 cell/phandle 参数。

- 绑定文件应命名为类似 compatible 的形式：vendor,device.yaml。如果绑定中有多个
  compatibles，使用其中一个回退或一个更通用的名称，但仍要匹配 compatible 的风格。

## 板级/SoC 的 .dts 文件


- 应当（DO）将所有 MMIO 设备放在一个总线节点下，而不是顶层。

- 应当（DO）使用非空的 'ranges' 来限制子总线/设备的大小。64 位平台不需要所有设备
  都拥有 64 位地址和大小。
