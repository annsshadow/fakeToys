
## Configfs GPIO 模拟器


configfs GPIO 模拟器（gpio-sim）提供了一种创建用于测试的模拟 GPIO 芯片的方法。这些芯片
暴露的线路既可以使用标准 GPIO 字符设备接口访问，也可以使用 sysfs 属性进行操作。

### 创建模拟芯片


gpio-sim 模块注册了一个名为 `'gpio-sim'` 的 configfs 子系统。关于 configfs 文件系统的
细节，请参阅 configfs 文档。

用户可以创建 configfs 组和条目的层级结构，并修改所暴露属性的值。一旦芯片被实例化，这个
层级结构将被转换为相应的设备属性。总体结构如下：

**组：** `/config/gpio-sim`

这是 gpio-sim configfs 树的顶层目录。

**组：** `/config/gpio-sim/gpio-device`

**属性：** `/config/gpio-sim/gpio-device/dev_name`

**属性：** `/config/gpio-sim/gpio-device/live`

这是一个表示 GPIO 平台设备的目录。`'dev_name'` 属性是只读的，允许用户空间读取平台设备
名（例如 `'gpio-sim.0'`）。`'live'` 属性用于在设备完全配置好后触发其实际创建。可接受的值
为：`'1'` 启用模拟设备，`'0'` 禁用并拆除它。

**组：** `/config/gpio-sim/gpio-device/gpio-bankX`

**属性：** `/config/gpio-sim/gpio-device/gpio-bankX/chip_name`

**属性：** `/config/gpio-sim/gpio-device/gpio-bankX/num_lines`

该组表示顶层平台设备下的一个 GPIO bank。`'chip_name'` 属性是只读的，允许用户空间读取
该 bank 设备的设备名。`'num_lines'` 属性用于指定该 bank 暴露的线路数量。

**组：** `/config/gpio-sim/gpio-device/gpio-bankX/lineY`

**属性：** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/name`

**属性：** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/valid`

该组表示偏移为 Y 的单条线路。`valid` 属性指示该线路是否可用作 GPIO。`name` 属性用于设置
由 'gpio-line-names' 属性所表示的线路名。

**条目：** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/hog`

**属性：** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/hog/name`

**属性：** `/config/gpio-sim/gpio-device/gpio-bankX/lineY/hog/direction`

该条目让 gpio-sim 模块独占（hog）关联的线路。`'name'` 属性指定要使用的内核内消费者名。
`'direction'` 属性指定独占方向，且必须为以下之一：`'input'`、`'output-high'` 和
`'output-low'`。

在每个 bank 目录内部，有一组可用于配置新芯片的属性。此外，用户可以在芯片目录内 `mkdir()`
子目录，用于传递特定线路的额外配置。这些子目录的名称必须采用 `'line<offset>'` 的形式
（例如 `'line0'`、`'line20'` 等），因为该名称会被模块用来把配置分配给给定偏移处的特定
线路。

配置完成后，必须将 `'live'` 属性设为 1 以实例化芯片。也可以将其设回 0 来销毁模拟芯片。
模块会同步等待新的模拟设备被成功探测，如果未发生，写入 `'live'` 将导致错误。

模拟 GPIO 芯片也可以在设备树中定义。compatible 字符串必须为：`"gpio-simulator"`。支持的
属性有：

  `"gpio-sim,label"` - 芯片标签

其它标准 GPIO 属性（如 `"gpio-line-names"`、`"ngpios"` 或 `"gpio-hog"`）同样受支持。
详情请参阅 GPIO 文档。

一个定义 GPIO 模拟器的设备树代码示例：


    gpio-sim {
        compatible = "gpio-simulator";

        bank0 {
            gpio-controller;
            #gpio-cells = <2>;
            ngpios = <16>;
            gpio-sim,label = "dt-bank0";
            gpio-line-names = "", "sim-foo", "", "sim-bar";
        };

        bank1 {
            gpio-controller;
            #gpio-cells = <2>;
            ngpios = <8>;
            gpio-sim,label = "dt-bank1";

            line3 {
                gpio-hog;
                gpios = <3 0>;
                output-high;
                line-name = "sim-hog-from-dt";
            };
        };
    };

### 操作模拟线路


每个模拟 GPIO 芯片在其设备目录下为每条暴露的线路创建一个独立的 sysfs 组
（例如 `/sys/devices/platform/gpio-sim.X/gpiochipY/`）。每个组的名称为 `'sim_gpioX'` 形式，
其中 X 是线路的偏移。每个组内部有两个属性：

    `pull` - 允许读取和设置每条线路的当前模拟上拉/下拉设置，写入时其值必须为以下之一：
               `'pull-up'`、`'pull-down'`

    `value` - 允许读取线路的当前值，如果该线路正被用户空间驱动，则该值可能与上拉/下拉设置不同
