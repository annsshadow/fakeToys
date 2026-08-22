
## Linux 安全模块（LSM

:Author: Casey Schaufler
:Date: 2023 骞?7 鏈。
Linux 安全模块（LSM）提供了一种机制，用于Linux 安全策略实现额外访问控制
各个安全模块可以支持以下任意属性：

`LSM_ATTR_CURRENT` 是进程当前、活跃的安全上下文proc 文件系统通过 `/proc/self/attr/current` 提供该值SELinux、Smack AppArmor 安全模块支持此属性Smack 还通过 `/proc/self/attr/smack/current` 提供该值AppArmor 还通过 `/proc/self/attr/apparmor/current` 提供该值
`LSM_ATTR_EXEC` 是当前映像被执行时进程的安全上下文proc 文件系统通过 `/proc/self/attr/exec` 提供该值SELinux AppArmor 安全模块支持此属性AppArmor 还通过 `/proc/self/attr/apparmor/exec` 提供该值
`LSM_ATTR_FSCREATE` 是进程在创建文件系统对象时使用的安全上下文proc 文件系统通过 `/proc/self/attr/fscreate` 提供该值SELinux 安全模块支持此属性
`LSM_ATTR_KEYCREATE` 是进程在创建密钥对象时使用的安全上下文proc 文件系统通过 `/proc/self/attr/keycreate` 提供该值SELinux 安全模块支持此属性
`LSM_ATTR_PREV` 是设置当前安全上下文时进程的安全上下文proc 文件系统通过 `/proc/self/attr/prev` 提供该值SELinux AppArmor 安全模块支持此属性AppArmor 还通过 `/proc/self/attr/apparmor/prev` 提供该值
`LSM_ATTR_SOCKCREATE` 是进程在创建套接字对象时使用的安全上下文proc 文件系统通过 `/proc/self/attr/sockcreate` 提供该值SELinux 安全模块支持此属性
## 内核接口


### 设置当前进程的安全属

    :identifiers: sys_lsm_set_self_attr

### 获取当前进程的指定安全属

    :identifiers: sys_lsm_get_self_attr

    :identifiers: sys_lsm_list_modules

## 附加文档


- Documentation/security/lsm.rst
- Documentation/security/lsm-development.rst
