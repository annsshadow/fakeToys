## 固件查找顺序


启用固件查找的方式有多种。以下是驱动发出 firmware API 调用后，固件将被查找的时间顺序。

- 首先检测 ''内建固件''（Built-in firmware），若固件存在则立即返回
- 接着查看 ''固件缓存''（Firmware cache），若找到则立即返回
- 然后执行 ''直接文件系统查找''（Direct filesystem lookup），若找到则立即返回
- 再执行 ''平台固件回退''（Platform firmware fallback），但仅在使用 firmware_request_platform() 时进行，若找到则立即返回
- 若仍未找到固件且回退机制已启用，则会创建 sysfs 接口。此后要么发出一个 kobject uevent，要么依赖自定义固件加载方式，直至达到超时时间。
