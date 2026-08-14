## /proc/sys/xen/


版权 (c) 2026, Shubham Chakraborty <chakrabortyshubham66@gmail.com>

有关一般信息和法律声明，请参阅
Documentation/admin-guide/sysctl/index.rst。

------------------------------------------------------------------------------

这些文件是否出现在 `/proc/sys/xen/` 中，取决于内核配置：

## balloon/hotplug_unpopulated


此标志控制是否将未填充的内存范围自动热插拔为系统 RAM。

- `0`：未填充的范围不被热插拔（默认）。
- `1`：未填充的范围被自动热插拔。

启用后，Xen balloon 驱动程序会将 Xen 内存映射中标记为未填充的内存区域作为
可用 RAM 添加到系统中。这允许在 Xen 客户域中动态扩展内存。

此选项仅在以内核配置了 `CONFIG_XEN_BALLOON_MEMORY_HOTPLUG` 时才可用。
