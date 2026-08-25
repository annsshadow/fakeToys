
## 提交 Devicetree（DT）绑定补

## I. 面向补丁提交

  0) 来自 `Documentation/process/submitting-patches.rst` 的常规补丁提交规则同样适用
  1) 补丁`Documentation/` `include/dt-bindings/` 部分
```

       "dt-bindings: <binding dir>: ..."

     少数子系统，ASoC、media、regulators、SCSI、SPI UFS，基于子系统名称期望前缀顺序相反::

       "<binding dir>: dt-bindings: ..."

     主题80 个字符十分宝贵。建议不要使"Documentation"doc" "YAML"，因为这些都是隐含的。所有绑定都是文档，且所有新绑定都应采用 Devicetree schema 格式。也应避免重"binding"，因此对于一个新设备，通常类似下面这样即可::

       "dt-bindings: iio: adc: Add ROHM BD79100G"

     将其他格式转换为 DT schema::

       "dt-bindings: iio: adc: adi,ad7476: Convert to DT schema"

  2) DT 绑定文件采用 DT schema 格式书写，使json-schema 词汇YAML 文件格式。DT 绑定文件必须通过运行以下命令的校:

       make dt_binding_check

     关于 schema 与工具配置的更多细节，请参见 `Documentation/devicetree/bindings/writing-schema.rst`
  3) DT 绑定文件应采用双重许可。首选许可标签为 (GPL-2.0-only OR BSD-2-Clause)
  4) 将整个补丁系列提交到 devicetree 邮件列表

       devicetree@vger.kernel.org

     并抄送（Cc）DT 维护者。使`scripts/get_maintainer.pl` 识别所DT 维护者
  5) 补丁`Documentation/` 部分应位于实现该绑定的代码之前，随补丁系列一并提交
  6) 在芯片或板级 DTS 文件中使用的任何 compatible 字符串，必须先前已在对应DT 绑定文件 `Documentation/devicetree/bindings` 中记录。即Linux 设备驱动尚未匹配compatible 字符串，此规则同样适用。[ 若未遵循此步骤，checkpatch 将会发出警告，自提交 bff5da4335256513497cc8c79f9a9d1665e09864checkpatch: add DT compatible string documentation checks"）起生效]

  7) DTS 总体上被视为与驱动无关的硬件描述，因此任DTS 补丁，无论使用已有还是新的绑定，都应置于补丁集末尾，以表明驱动对 DTS 没有依赖。DTS 无论如何都会通过独立的树或分支合入，因此不同的顺序会表明该系列不可二分（non-bisectable）
     如果某个驱动子系统维护者倾向于合入整个集合而非其中相关部分，请DTS 补丁拆分为独立的补丁集，并在变更日志或封面信中引用邮件列表上的绑定提交
  8) 如果某个已记录的 compatible 字符串尚未被驱动匹配，文档还应包含该驱动所匹配compatible 字符串
  9) 绑定正被 Linux 内核之外的多个项目积极使用，在修改已有绑定时可能需要额外的谨慎与考量
```
## II. 闈㈠悜鍐呮牳缁存姢鑰。

  1) 如果你对审查某个绑定感到不确定，请回复该绑定并请devicetree 维护者给予指导。这将有助于他们确定优先审查哪些、哪些可以放行
  2) 对于驱动（非子系统）绑定：如果你对该绑定感到满意，且几周后仍未收devicetree 维护者的 Acked-by，请直接将其合入
     对于子系统绑定（影响多个设备的任何内容），必须让一devicetree 维护者对其进行审查
  3) 对于经过多棵树的补丁系列，绑定补丁应与使用该绑定的驱动放在一起
  4) DTS 文件绝不应通过驱动子系统树合入，而应始终通过平台 SoC 树在专用分支上合入（另见 `Documentation/process/maintainer-soc.rst`）
## III. 注意事项


  0) 关于 devicetree ABI 的细节，请参`Documentation/devicetree/bindings/ABI.rst`
  1) 本文档旨在作为对 2013 年内核峰会所确定流程的总体熟悉指引。如有疑问，devicetree 维护者当前的意见优先于本文档。在这种情况下，欢迎提交更新本文档的补丁