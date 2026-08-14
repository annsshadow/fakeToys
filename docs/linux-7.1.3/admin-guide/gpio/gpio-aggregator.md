
## GPIO 聚合器

GPIO 聚合器提供了一种将 GPIO 聚合，并以一个新的 gpio_chip 暴露出来的机制。它支持
以下使用场景。

### 使用 Sysfs 聚合 GPIO

GPIO 控制器通过 /dev/gpiochip* 字符设备导出到用户空间。对这些设备的访问控制由
标准 UNIX 文件系统权限提供，采用"全有或全无"的方式：要么某个 GPIO 控制器对某个
用户可访问，要么不可访问。

GPIO 聚合器通过将一组一个或多个 GPIO 聚合为一个新的 gpio_chip，来为这组 GPIO
提供访问控制，该 gpio_chip 可以使用标准 UNIX 文件拥有者和权限分配给某个组或用户。
此外，这也简化并加固了将 GPIO 导出到虚拟机的过程，因为虚拟机只需获取整个 GPIO
控制器，而不再需要关心要获取哪些 GPIO、不获取哪些 GPIO，从而减小了攻击面。

聚合的 GPIO 控制器通过写入 sysfs 中的只写属性文件来实例化和销毁。

    /sys/bus/platform/drivers/gpio-aggregator/

	"new_device" ...
		用户空间可以通过向 "new_device" 文件写入一个描述要聚合的
		GPIO 的字符串，来请求内核实例化一个聚合的 GPIO 控制器，
		格式如下：

		.. code-block:: none

		    [<gpioA>] [<gpiochipB> <offsets>] ...

		其中：

		    "<gpioA>" ...
			    是一个 GPIO 线条名称，

		    "<gpiochipB>" ...
			    是一个 GPIO 芯片标签，并且

		    "<offsets>" ...
			    是一个以逗号分隔的 GPIO 偏移和/或由短横线表示的
			    GPIO 偏移范围列表。

		Example: 通过聚合 "e6052000.gpio" 的 GPIO 线条 19 以及
		"e6050000.gpio" 的 GPIO 线条 20-21，实例化一个新的 GPIO
		聚合器，成为一个新的 gpio_chip：

		.. code-block:: sh

		    $ echo 'e6052000.gpio 19 e6050000.gpio 20-21' > new_device

	"delete_device" ...
		用户空间可以通过将其设备名称写入 "delete_device" 文件，
		来请求内核在使用后销毁一个聚合的 GPIO 控制器。

		Example: 销毁先前创建的聚合 GPIO 控制器，假定其名为
		"gpio-aggregator.0"：

		.. code-block:: sh

		    $ echo gpio-aggregator.0 > delete_device


### 使用 Configfs 聚合 GPIO

**Group:** `/config/gpio-aggregator`

    这是 gpio-aggregator configfs 树的根目录。

**Group:** `/config/gpio-aggregator/<example-name>`

    该目录表示一个 GPIO 聚合器设备。你可以为 `<example-name>` 指定任意名称
    （例如 `agg0`），但以 `_sysfs` 前缀开头的名称除外，这些名称保留给通过
    Sysfs 创建的设备的自动生成的 configfs 条目使用。

**Attribute:** `/config/gpio-aggregator/<example-name>/live`

    `live` 属性允许在设备完全配置好后触发其实际创建。可接受的值为：

    - `1`, `yes`, `true` : 启用虚拟设备
    - `0`, `no`, `false` : 禁用虚拟设备

**Attribute:** `/config/gpio-aggregator/<example-name>/dev_name`

    只读的 `dev_name` 属性暴露该设备的名称，它将出现在平台上（例如
    `gpio-aggregator.0`）。这对于识别新创建的聚合器所对应的字符设备很有用。
    如果它是 `gpio-aggregator.0`，那么
    `/sys/devices/platform/gpio-aggregator.0/gpiochipX` 路径会告诉你该 GPIO
    设备的 id 是 `X`。

当你想要实例化 `Y+1`（Y >= 0）条线时，必须为每条想要实例化的虚拟线创建子目录，
名称必须严格为 `line0`、`line1`、...、`lineY`。在通过将 `live` 设为 1 来激活
设备之前，配置好所有线。

**Group:** `/config/gpio-aggregator/<example-name>/<lineY>/`

    该目录表示要包含进聚合器的一条 GPIO 线。

**Attribute:** `/config/gpio-aggregator/<example-name>/<lineY>/key`

**Attribute:** `/config/gpio-aggregator/<example-name>/<lineY>/offset`

    创建 `<lineY>` 目录后的默认值为：

    - `key` : <empty>
    - `offset` : -1

    `key` 必须始终显式配置，而 `offset` 则视情况而定。每个 `<lineY>` 有两种
    配置模式：

    (a). 按 GPIO 线条名称查找：

         - 将 `key` 设置为线条名称。
         - 确保 `offset` 保持为 -1（默认值）。

    (b). 按 GPIO 芯片名称以及该芯片内的线条偏移查找：

         - 将 `key` 设置为芯片名称。
         - 将 `offset` 设置为线条偏移（0 <= `offset` < 65535）。

**Attribute:** `/config/gpio-aggregator/<example-name>/<lineY>/name`

    `name` 属性为 lineY 设置一个自定义名称。如果留空，该线将保持无名。

配置完成后，必须将 `'live'` 属性设为 1 才能实例化聚合器设备。可以将其设回 0 来
销毁该虚拟设备。模块会同步等待新的聚合器设备被成功探测，如果未发生，写入 `'live'`
将导致错误。这与使用 sysfs `new_device` 接口创建时的情况不同。

   对于通过 Sysfs 创建的聚合器，configfs 条目会自动生成，并表现为
   `/config/gpio-aggregator/_sysfs.<N>/`。你无法使用 mkdir(2)/rmdir(2) 添加或
   删除线条目录。要修改线条，必须使用 "delete_device" 接口拆除现有设备并从头
   重新配置。但是，当 `live` 由手工设为 0 时（即它不等待延后的探测），你仍然
   可以用 `live` 属性切换聚合器，并调整每条线的 `key`、`offset` 和 `name`
   属性。

#### 示例配置命令



    # 为聚合器设备创建目录
    $ mkdir /sys/kernel/config/gpio-aggregator/agg0

    # 配置每条线
    $ mkdir /sys/kernel/config/gpio-aggregator/agg0/line0
    $ echo gpiochip0 > /sys/kernel/config/gpio-aggregator/agg0/line0/key
    $ echo 6         > /sys/kernel/config/gpio-aggregator/agg0/line0/offset
    $ echo test0     > /sys/kernel/config/gpio-aggregator/agg0/line0/name
    $ mkdir /sys/kernel/config/gpio-aggregator/agg0/line1
    $ echo gpiochip0 > /sys/kernel/config/gpio-aggregator/agg0/line1/key
    $ echo 7         > /sys/kernel/config/gpio-aggregator/agg0/line1/offset
    $ echo test1     > /sys/kernel/config/gpio-aggregator/agg0/line1/name

    # 激活聚合器设备
    $ echo 1         > /sys/kernel/config/gpio-aggregator/agg0/live


### 通用 GPIO 驱动

GPIO 聚合器也可以用作 DT 中描述的、由简单 GPIO 操作的设备的通用驱动，而无需
专用的内核内驱动。这在工业控制中很有用，并且与例如 spidev 并无不同，后者允许
用户从用户空间与 SPI 设备通信。

将一个设备绑定到 GPIO 聚合器，可以通过修改 gpio-aggregator 驱动，或者通过写入
Sysfs 中的 "driver_override" 文件来完成。

Example: 如果 "door" 是一个在 DT 中描述的、由 GPIO 操作的设备，使用其自己的
```

	door {
		compatible = "myvendor,mydoor";

		gpios = <&gpio2 19 GPIO_ACTIVE_HIGH>,
			<&gpio2 20 GPIO_ACTIVE_LOW>;
		gpio-line-names = "open", "lock";
	};

```
它可以通过以下两种方式绑定到 GPIO 聚合器：

1. 将其 compatible 值添加到 `gpio_aggregator_dt_ids[]`，
2. 使用 "driver_override" 手动绑定：


    $ echo gpio-aggregator > /sys/bus/platform/devices/door/driver_override
    $ echo door > /sys/bus/platform/drivers/gpio-aggregator/bind

之后，会创建一个新的 gpiochip "door"：


    $ gpioinfo door
    gpiochip12 - 2 lines:
	    line   0:       "open"       unused   input  active-high
	    line   1:       "lock"       unused   input  active-high
