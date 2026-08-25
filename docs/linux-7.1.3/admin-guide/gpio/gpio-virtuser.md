
## 铏氭嫙 GPIO 娑堣垂鑰。

虚拟 GPIO 消费者（virtual GPIO consumer）模块允许用户实例化虚拟设备，这些设会请GPIO 并通过 debugfs 控制其行为。虚拟消费者设备可以通过设备树或 configfs
进行实例化
虚拟消费者使用面向驱动程序的 GPIO API，并允许通过用户空间驱动的自动化测试来覆它。GPIO 使用 `gpiod_get_array()` 请求，因此我们支持每个连ID 对应多个 GPIO
### 创建 GPIO 消费

gpio-consumer 模块注册了一个名`'gpio-virtuser'` configfs 子系统。有configfs 文件系统的详细信息，请参configfs 文档
用户可以创建 configfs 组和条目的层级结构，并修改所暴露属性的值。消费者一旦实例化该层级结构将被转换为适当的设备属性。其通用结构如下
**组：** `/config/gpio-virtuser`

这是 gpio-consumer configfs 树的顶层目录
**组：** `/config/gpio-consumer/example-name`

**属性：** `/config/gpio-consumer/example-name/live`

**属性：** `/config/gpio-consumer/example-name/dev_name`

这是一个代GPIO 消费者设备的目录
只读`dev_name` 属性暴露该设备在系统中的名称，即它会出现在平台总线上。这对于
`/sys/kernel/debug/gpio-virtuser/$dev_name` 下定位关联的 debugfs 目录很有用
`'live'` 属性用于在设备完全配置好后触发其实际创建。可接受的值为：`'1'` 用于启用
虚拟设备，`'0'` 用于禁用并拆除它
### 创建 GPIO 查找

用户可以在设备组下创建多configfs 组：

**组：** `/config/gpio-consumer/example-name/con_id`

`'con_id'` 目录代表单个 GPIO 查找，其值映射到 `gpiod_get()` 函数`'con_id'`
参数。例如：`con_id` == `'reset'` 映射`reset-gpios` 设备属性
用户可以为每个查找分配多GPIO。每GPIO `'con_id'` 组下的一个具有用户定名称的子目录
**属性：** `/config/gpio-consumer/example-name/con_id/0/key`

**属性：** `/config/gpio-consumer/example-name/con_id/0/offset`

**属性：** `/config/gpio-consumer/example-name/con_id/0/drive`

**属性：** `/config/gpio-consumer/example-name/con_id/0/pull`

**属性：** `/config/gpio-consumer/example-name/con_id/0/active_low`

**属性：** `/config/gpio-consumer/example-name/con_id/0/transitory`

这是一个描`con_id-gpios` 属性中单个 GPIO 的组
对于使用 configfs 创建的虚拟消费者，我们使用机器查找表，因此可以将该组视为文件系`'struct gpiod_lookup'` 中单个条目的字段之间的映射
`'key'` 属性代表该 GPIO 所属的芯片名称GPIO 线路名称。这取决`'offset'` 属的值：如果其>= 0，则 `'key'` 代表要查找的芯片标签，`'offset'` 代表该芯片中
线路的偏移量。如`'offset'` < 0，则 `'key'` 代表线路的名称
其余属性映射到 GPIO 查找结构`'flags'` 字段。前两个接受字符串值作为参数：

**`'drive'`* `'push-pull'`、`'open-drain'`、`'open-source'`
**`'pull'`* `'pull-up'`、`'pull-down'`、`'pull-disabled'`、`'as-is'`

`'active_low'` `'transitory'` 是布尔属性
### 激GPIO 消费

配置完成后，必须`'live'` 属性设置为 1 以实例化消费者。将其设0 可销毁虚拟设备模块将同步等待新模拟的设备被成功探测，如果未发生，写`'live'` 将导致错误
### 设备

虚拟 GPIO 消费者也可以在设备树中定义。其兼容字符串必须为：`"gpio-virtuser"`并至少有一个遵循标准化 GPIO 模式的属性
一个定义虚GPIO 消费者的设备树代码示例：


    gpio-virt-consumer {
        compatible = "gpio-virtuser";

        foo-gpios = <&gpio0 5 GPIO_ACTIVE_LOW>, <&gpio1 2 0>;
        bar-gpios = <&gpio0 6 0>;
    };

### 鎺у埗铏氭嫙 GPIO 娑堣垂鑰。

激活后，设备将导出 debugfs 属性，用于控制 GPIO 数组以及每个单独请求GPIO 线路我们来考虑以下设备属性：`foo-gpios = <&gpio0 0 0>, <&gpio0 4 0>;`
将创建以debugfs 属性组
**组：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo/`

这是包含整个 GPIO 数组属性的组
**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo/values`

**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo/values_atomic`

这两个属性都允许读取和设GPIO 值数组。用户必须以字符串形式传入数组所包含数量值，字符串由代表非激活和激GPIO 状态的零和一组成。在本例中：`echo 11 > values`
`values_atomic` 属性的工作方式`values` 相同，但内核将在中断上下文中执行 GPIO
驱动回调
**组：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/`

这是一个代表单GPIO 的组，`$index` 是其在数组中的偏移量
**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/consumer`

允许设置和读GPIO 线路的消费者标签
**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/debounce`

允许设置和读GPIO 线路的去抖周期
**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/direction`

**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/direction_atomic`

这两个属性允许设GPIO 线路的方向。它们接`"input"` `"output"` 作为值原子变体在中断上下文中执行驱动回调
**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/interrupts`

如果该线路以输入模式请求，向该属性写`1` 将使模块监听GPIO 上的边沿中断写入 `0` 将禁用监视。读取该属性返回当前已注册的中断数（两个边沿）
**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/value`

**属性：** `/sys/kernel/debug/gpio-virtuser/$dev_name/gpiod:foo:$index/value_atomic`

这两个属性都允许读取和设置各个已请求 GPIO 线路的值。它们接受以下值：`1` `0`