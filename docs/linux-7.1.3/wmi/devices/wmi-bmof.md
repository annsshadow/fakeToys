## WMI 内嵌二进制 MOF 驱动


## 简介


许多机器内嵌了用于描述其 ACPI WMI 接口细节的 WMI 二进制 MOF（Managed Object Format）元数据。这些数据可通过 `bmfdec <https://github.com/pali/bmfdec>`_ 之类的工具解码，以获得可读性良好的 WMI 接口描述，这对开发新的 WMI 驱动很有用。

二进制 MOF 数据可从相关 WMI 设备的 `bmof` sysfs 属性中获取。请注意，给定系统上可能存在多个包含二进制 MOF 数据的 WMI 设备。

## WMI 接口


二进制 MOF WMI 设备由 WMI GUID `05901221-D566-11D1-B2F0-00A0C9062910` 标识。二进制 MOF 可通过执行 WMI 数据块查询获取。结果随后以可变大小的 ACPI 缓冲区形式返回。
