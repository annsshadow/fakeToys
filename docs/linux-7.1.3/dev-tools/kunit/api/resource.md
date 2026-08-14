## 资源 API


本文件记录 KUnit 资源 API。

大多数用户无需直接使用此 API；高级用户可用其在每次测试的基础上存储状态、注册
自定义清理动作等。

   :internal:

### 受管设备


用于使用 KUnit 托管的 struct device 与 struct device_driver 的函数。使用这些需包含
`kunit/device.h`。

   :internal:
