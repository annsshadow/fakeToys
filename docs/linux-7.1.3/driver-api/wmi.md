## WMI 驱动 API


WMI 驱动核心支持一种更现代的基于总线（bus）的接口来与 WMI 设备交互，以及一种较旧的基于 GUID 的接口。后者被视为已弃用，因此新的 WMI 驱动通常应避免使用它，因为它在某WMI 设备共享同一 GUID 时存在一些问题
现代的基于总线的接口则把每WMI 设备映射到一`struct wmi_device <wmi_device>`，因此它支持共享同一 GUID WMI 设备。驱动随后可以注册一`struct wmi_driver <wmi_driver>`，由驱动核心绑定到兼容的 WMI 设备

   :internal:

   :export:

   :export:
