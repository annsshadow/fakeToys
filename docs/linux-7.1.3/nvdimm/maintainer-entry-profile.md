## LIBNVDIMM 维护者条目档

### 概述

libnvdimm 子系统管理跨多种架构的持久内存。邮件列表由 patchwork 在此跟踪https://patchwork.kernel.org/project/linux-nvdimm/list/
…该实例被配置为在补丁被接受和上游合并时向提交者反馈。补丁会被合并到
'libnvdimm-fixes' 'libnvdimm-for-next' 分支。这些分支可在此获取https://git.kernel.org/pub/scm/linux/kernel/git/nvdimm/nvdimm.git/

一般而言，补丁可以针对最新的 -rc 提交；不过，如果传入的代码变更依赖于其他
待定变更，则该补丁应基于 libnvdimm-for-next 分支。然而，由于持久内存处于存储
与内存的交汇处，在某些情况下，补丁更适合通过 Filesystem Memory Management
树合并。如有疑问，请抄nvdimm 列表，维护者会帮助路由
提交的内容将被暴露给 kbuild robot 进行编译回归测试。在提交前从该基础设施获得
成功通知会有所帮助，但并非必须
### 提交检查清单补
该子系统通过 ndctl 工具提供单元测试https://github.com/pmem/ndctl
这些测试需要在补丁进入上游之前通过，但未必需要在首次发布之前通过。如果需帮助搭建测试环境，请联系邮件列表
#### ACPI 设备特定方法（_DSM
在考虑启用_DSM 族的的补丁之前，必须ACPI 规范工作组的 NVDIMM 子团为其分配一个格式接口代码。一般而言，该子系统的立场是反NVDIMM 命令集的
扩散，因此请务必认真考虑实现对现有命令集的支持。参drivers/acpi/nfit/nfit.h
以了解受支持命令集的集合
### 关键周期日期

新的提交可以随时发送，但如果希望进入下一个合并窗口，则应-rc4 之前发送，最好在 -rc6 之前libnvdimm-for-next 分支中稳定下来。当然，如果一个补丁集
需要超2 周的审查rc4 已经太晚，有些补丁可能需要多个开发周期来审查
### 审查节奏

一般而言，在催促寻求反馈之前请等待最长一周。建议使用私信提醒。或者，也可请其他拥libnvdimm 变更 Reviewed-by 标签的开发者查看并提供他们的意见