
## RISC-V Linux 用户 ABI


### /proc/cpuinfo 中的 ISA 字符串顺

ISA 字符串中扩展名的标准顺序定义于《RISC-V 指令集手I 非特ISA（RISC-V Instruction Set Manual Volume I Unprivileged ISA）第 27 （文档版20191213）
该规范在排序问题上使用了诸如 should（应当）这类含糊的措辞，因此就我们的目而言适用以下规则
#. 单字母扩展在前，按标准顺序排列。标准顺序为 "IMAFDQLCBKJTPVH"
#. 所有多字母扩展之间用下划线与其他扩展分隔
#. 额外的标准扩展（'Z' 开头）排在单字母扩展之后、任何更高特权级扩展之前
#. 对于额外的标准扩展，'Z' 之后的第一个字母按惯例表示关系最密切的字母序扩展类别   如果命名了多'Z' 扩展，则先按上述标准顺序以类别排序，再在同一类别内按字母序排列
#. 标准监管者级（supervisor-level）扩展（'S' 开头）排在标准非特权扩展之后   如果列出了多个监管者级扩展，则按字母序排列
#. 标准机器级（machine-level）扩展（'Zxm' 开头）排在任何更低特权级的标准扩展之后   如果列出了多个机器级扩展，则按字母序排列
#. 非标准扩展（'X' 开头）排在所有标准扩展之后。如果列出了多个非标准扩展，   按字母序排列
```
   rv64imadc_zifoo_zigoo_zafoo_sbar_scar_zxmbaz_xqux_xrux

```
### /proc/cpuinfo 中的 "isa" "hart isa" 

/proc/cpuinfo 中的 "isa" 行描述了内核识别、且所hart 都实现的 RISC-V ISA
扩展的最小公分母。相反，"hart isa" 行描述的是内核在所描述的具hart 上识别到扩展集合，即使这些扩展可能并非系统中所hart 都具备
在两行中，某个扩展的存在都只保证硬件具备所描述的能力。在该扩展的能力能被用户程序完全使用之前，可能还需要额外的内核支持或策略变更。类似地，对S 模式扩展在这些行中出现并不保证内核正在利用该扩展，也不保证该特性对由此内核管理的客户机
虚拟机可见
反过来，这些行中缺失某个扩展并不一定意味着硬件不支持该特性。正在运行的内核可能
无法识别该扩展，或者可能有意将其从列表中移除
### 非对齐访

用户态支持非对齐的标量（scalar）访问，但性能可能较差。非对齐的向量（vector）访只有在支Zicclsm 扩展时才被支持
### 指针掩码


用户态的指针掩码支持（Supm 扩展）通过 `PR_SET_TAGGED_ADDR_CTRL` `PR_GET_TAGGED_ADDR_CTRL` 这两`prctl()` 操作提供。指针掩码默认关闭。要启用它，
用户态必须用 `PR_PMLEN` 字段设置为应用所需的掩标记位数来调`PR_SET_TAGGED_ADDR_CTRL`。`PR_PMLEN` 被解释为一个下界；如果内核无法满足该请求，
`PR_SET_TAGGED_ADDR_CTRL` 操作将失败。实际的标记位数`PR_GET_TAGGED_ADDR_CTRL`
操作通过 `PR_PMLEN` 返回
此外，当指针掩码被启用时（`PR_PMLEN` 大于 0），支持带标记的地址 ABI，其接口行为AArch64 文档中所记载的一致（Documentation/arch/arm64/tagged-address-abi.rst）