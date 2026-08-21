## NVDIMM 瀹夊叏鎬。

### 1. 引言


随着 Intel Device Specific Methods（DSM，设备特定方法）v1.8 规范 [^1^] 的引入，
安全相关DSM 被加入。该规范新增了以下安DSMget security state"（获安全状态）set passphrase"（设置口令）disable passphrase"（禁用口令）"unlock unit"（解锁单元）freeze lock"（冻结锁）secure erase"（安全擦除）
以及 "overwrite"（覆写）。为了支持这些安全操作，struct dimm 中新增了一security_ops 数据结构，并暴露了通用 API 以支持与厂商无关的操作用法
### 2. Sysfs 接口


nvdimm sysfs 目录中提供了 "security" 这一 sysfs 属性。例如：
/sys/devices/LNXSYSTM:00/LNXSYBUS:00/ACPI0012:00/ndbus0/nmem0/security

该属性的 "show" 属性会显示DIMM 的安全状态。可用的状态有：disabled（已禁用）unlocked（已解锁）、locked（已锁定）、frozen（已冻结）和 overwrite（覆写中）如果不支持安全特性，sysfs 属性将不可见
对该属性执行写操作时，"store" 属性会接受若干命令以支持部分安全功能：
update <old_keyid> <new_keyid> - 启用或更新口令disable <keyid> - 禁用已启用的安全特性并移除密钥freeze - 冻结安全状态的变更erase <keyid> - 删除现有用户加密密钥overwrite <keyid> - 擦除整个 nvdimmmaster_update <keyid> <new_keyid> - 启用或更新主口令master_erase <keyid> - 删除现有用户加密密钥
### 3. 密钥管理


密钥通过 DIMM id 与负载相关联。例如：
# cat /sys/devices/LNXSYSTM:00/LNXSYBUS:00/ACPI0012:00/ndbus0/nmem0/nfit/id
8089-a2-1740-00000133
DIMM id 会与密钥负载（口令）一起提供给内核
安全密钥每个 DIMM 一把密的方式管理。密口令"预期32 字节长。这类似ATA 安全规范 [^2^]。在 nvdimm 解锁期间，密钥最初通过 request_key() 内核 API
调用获取。用户有责任确保所有密钥都已置于内核用户密钥环（user keyring）中以便
解锁
格式enc32 nvdimm 加密密钥（encrypted-key）的描述格式为：
nvdimm:<bus-provider-specific-unique-id>

创建 enc32 格式encrypted-keys 请参见文`Documentation/security/keys/trusted-encrypted.rst`。使用主可信密钥（master
trusted key）配TPM 来封装（sealing）encrypted-keys 是推荐做法
### 4. 解锁


当内核枚DIMM 时，内核会尝试从内核用户密钥环中检索密钥。这是解锁一个已锁定
DIMM 的唯一时机。一旦解锁，DIMM 将保持解锁状态直到重启。通常某个实体（例shell 脚本）会initramfs 阶段将所有相关的 encrypted-keys 注入内核用户密钥环这为解锁功能提供了访问所有相关密钥（其中包含对应 nvdimm 的口令）的途径。同建议libnvdimm modprobe 加载之前注入密钥
### 5. 更新


进行更新时，预期现有的密钥会从内核用户密钥环中移除，并以不同的（旧）密钥重新
注入。旧密钥的描述是什么无关紧要，因为更新操作我们只关keyid。同时预期新密钥
以本文档前面描述的格式注入其描述。写sysfs 属性的更新命令格式为：
update <old keyid> <new keyid>

如果由于启用安全特性而不存在keyid，则应传0
### 6. 冻结（Freeze

freeze 操作不需要任何密钥。安全配置可由具root 权限的用户冻结
### 7. 禁用（Disable

安全禁用的命令格式为disable <keyid>

一个绑定到nvdimm、带有当前口令负载的密钥应当存在于内核用户密钥环中
### 8. 安全擦除（Secure Erase

执行安全擦除的命令格式为erase <keyid>

一个绑定到nvdimm、带有当前口令负载的密钥应当存在于内核用户密钥环中
### 9. 覆写（Overwrite

执行覆写的命令格式为overwrite <keyid>

如果未启用安全特性，覆写可以在没有密钥的情况下进行。可传入密钥序列0 来表无密钥
可以轮询 sysfs 属"security" 以等待覆写完成。根nvdimm 大小不同，覆写可持续数十分钟或更久
一个绑定到nvdimm、带有当前用户口令的 encrypted-key 应当被注入，并通过 sysfs
传入keyid
### 10. 主更新（Master Update

执行主更新的命令格式为：
update <old keyid> <new keyid>

主更新的运行机制update 相同，只是传入内核的是主口令密钥。主口令密钥只是
另一encrypted-key
该命令仅在安全特性被禁用时可用
### 11. 主擦除（Master Erase

执行主擦除的命令格式为：
master_erase <current keyid>

该命令的运行机制erase 相同，只是传入内核的是主口令密钥。主口令密钥只是另一encrypted-key
该命令仅在主安全特性已启用时可用，这由扩展安全状态指示
[^1^]: https://pmem.io/documents/NVDIMM_DSM_Interface-V1.8.pdf

[^2^]: http://www.t13.org/documents/UploadedDocuments/docs2006/e05179r4-ACS-SecurityClarifications.pdf
