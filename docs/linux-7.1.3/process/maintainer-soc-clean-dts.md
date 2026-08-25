## 具有 DTS 合规性要求的 SoC 平台


### 概述


SoC 平台或子架构应遵Documentation/process/maintainer-soc.rst 中的所有规则。MAINTAINERS 中引用的本文档施加了以下附加要求
### 严格DTS DT Schema dtc 合规

SoC 平台 Devicetree 源文件（DTS 文件）的修改不应引入新的 `make dtbs_check W=1` 警告。新板级 DTS 中因所包含 DTSI 文件的问题而产生的警告，视为已有警告，而非新警告。对于拆分到不同树的补丁系列（DT 绑定经由驱动子系统树），linux-next 上的警告为准。平台维护者已部署自动化工具，应能指出任何新警告
如果引入新警告的提交以某种方式被接受，由此产生的问题应在合理时间内（例如一个发布周期内）修复，否则该提交应被回退