## 固件搜索路径


在您的根文件系统上查找固件时使用以下搜索路径。

- fw_path_para - 模块参数 - 默认为空，因此被忽略
- /lib/firmware/updates/UTS_RELEASE/
- /lib/firmware/updates/
- /lib/firmware/UTS_RELEASE/
- /lib/firmware/

模块参数 ''path'' 可以传递给 firmware_class 模块，以激活第一个可选的
自定义 fw_path_para。自定义路径的长度最多为 256 个字符。传入的内核
参数应为：

- 'firmware_class.path=$CUSTOMIZED_PATH'

还有一种替代方法可以在启动后运行时自定义路径，您可以使用以下文件：

- /sys/module/firmware_class/parameters/path

您可以将自定义路径 echo 写入其中，所请求的固件将首先在该路径下搜索。
请注意，换行符会被考虑在内，并且可能不会产生预期的效果。例如，您可能
希望使用：

echo -n /path/to/script > /sys/module/firmware_class/parameters/path

以确保使用您的脚本。
