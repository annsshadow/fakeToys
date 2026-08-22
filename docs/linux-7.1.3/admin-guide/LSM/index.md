## Linux 安全模块（LSM）用

Linux 安全模块（LSM）框架提供了一种机制，使各种安全检查可以被新的内核扩展挂钩（hook）。“模块（module）”这一名称有点用词不当，因为这些扩展实际上并不是可加载的内核模块。相反，它们是在构建时通过 CONFIG_DEFAULT_SECURITY 可选，并且在给定内核中编译了多LSM 的情况下，可以通过 `"security=..."` 内核命令行参数在启动时覆盖
LSM 接口的主要使用者是强制访问控制（MAC）扩展，它们提供全面的安全策略。例子包SELinux、Smack、Tomoyo AppArmor。除了较大的 MAC 扩展外，还可以使LSM 构建其他扩展，在 Linux 核心功能本身不提供这些调整时，对系统操作进行特定修改
Linux capabilities 模块将始终被包含。其后可以跟任意数量的“minor（次要）”模块，以及至多一个“major（主要）”模块。关capabilities 的更多细节，请参Linux man-pages 项目中的 `capabilities(7)`
可以通过读取 `/sys/kernel/security/lsm` 找到活动安全模块的列表。这是一个逗号分隔的列表，并且始终会包capability 模块。该列表反映了进行检查的顺序。capability 模块始终排在第一位，其后跟随任意“minor”模块（Yama），然后是那个“major”模块（SELinux，如果配置了一个的话）
与“major”安全模块关联的进程属性应使用 `/proc/.../attr` 中的特殊文件访问与维护。一个安全模块可以在那里维护一个以该模块命名的、特定于模块的子目录。`/proc/.../attr/smack` Smack 安全模块提供，并包含其所有特殊文件。直接在 `/proc/.../attr` 中的文件仍作为为提供子目录的模块保留的遗留接口
- [apparmor](apparmor)
- [LoadPin](LoadPin)
- [SELinux](SELinux)
- [Smack](Smack)
- [tomoyo](tomoyo)
- [Yama](Yama)
- [SafeSetID](SafeSetID)
- [ipe](ipe)
- [landlock](landlock)
