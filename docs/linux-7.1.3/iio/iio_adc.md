
## IIO 对 ADC 的抽象（IIO Abstractions for ADCs）

## 1. 概述（Overview）

IIO 子系统支持许多模数转换器（Analog to Digital Converters，简称 ADC）。某些 ADC 具有一些
特性和特征，需要由 IIO 设备驱动以特定方式来支持。本文档描述了常见的 ADC 特性，并解释了 IIO
子系统是如何支持它们的。

## 1. ADC 通道类型（ADC Channel Types）

ADC 可以具有不同类型的输入，每种输入以略微不同的方式测量模拟电压。ADC 在通常给定电压参考、
输入类型和输入极性的一个范围内，对模拟输入电压进行数字化。确定一个 ADC 通道所允许的输入范围
是必要的，以便得出把测量值转换为现实单位（电压测量为毫伏、电流测量为毫安等）所需的缩放因子
和偏移量。

复杂的设计可能具有非线性特征，或集成组件（如放大器和参考缓冲区），在推导 ADC 所允许的输入
范围时也需要考虑它们。为清晰起见，下面各小节假设输入范围仅取决于所提供的电压参考、输入类型
和输入极性。

ADC 输入有三种通用类型（单端、差分、伪差分）和两种可能的极性（单极性、双极性）。输入类型
（单端、差分、伪差分）是一个通道特征，与极性（单极性、双极性）方面完全独立。一篇关于 ADC
输入类型的综合性文章（本文很大程度上基于它）可以在以下地址找到：
https://www.analog.com/en/resources/technical-articles/sar-adc-input-types.html。

### 1.1 单端通道（Single-ended channels）

单端通道把模拟输入电压相对于地（ground）进行数字化，可以是单极性也可以是双极性。

##### 1.1.1 单端单极性通道（Single-ended Unipolar Channels）

```

  ---------- VREF -------------
      ´ `           ´ `                  _____________
    /     \       /     \               /             |
   /       \     /       \         --- <  IN    ADC   |
            \   /         \   /         \             |
             `-´           `-´           \       VREF |
  -------- GND (0V) -----------           +-----------+
                                                  ^
                                                  |
                                             External VREF

```
**单端单极性**通道的输入电压允许从 GND 摆动到 VREF（其中 VREF 是电位高于系统地（system ground）
的电压参考）。最大输入电压也称为 VFS（Voltage input Full-Scale，满量程输入电压），由 VREF
决定。电压参考可以由外部电源提供，也可以从芯片电源推导而来。

单端单极性通道可以在设备树（device tree）中如下描述：
```

    adc@0 {
        ...
        #address-cells = <1>;
        #size-cells = <0>;

        channel@0 {
            reg = <0>;
        };
    };

```
总是允许在设备树中纳入 ADC 通道节点。不过，如果设备具有一组一致的输入（例如全部为单端），
那么声明这些通道节点是可选的。

对于支持单端与差分混合通道的设备，有一个注意事项：当 `reg` 是一个与输入引脚编号不匹配的任意
数字时，单端通道节点还需要提供一个 `single-channel` 属性。

请参见 `Documentation/devicetree/bindings/iio/adc/adc.yaml` 以获取 ADC 专有设备树属性的完整
文档。

##### 1.1.2 单端双极性通道（Single-ended Bipolar Channels）

```

  ---------- +VREF ------------
      ´ `           ´ `                  _____________________
    /     \       /     \               /                     |
   /       \     /       \         --- <  IN          ADC     |
            \   /         \   /         \                     |
             `-´           `-´           \       +VREF  -VREF |
  ---------- -VREF ------------           +-------------------+
                                                  ^       ^
                                                  |       |
                             External +VREF ------+  External -VREF

```
对于**单端双极性**通道，模拟电压输入可以从 -VREF 到 +VREF（其中 -VREF 是电位较低的电压参考，
而 +VREF 是电位较高的参考）。某些 ADC 芯片从 +VREF 推导出较低的参考，另一些则从单独的输入获取。
通常 +VREF 和 -VREF 是对称的，但也不一定必须如此。当 -VREF 低于系统地（system ground）时，
这些输入也称为单端真双极性（single-ended true bipolar）。此外，尽管从电气角度看双极性与真双极性
有明显区别，IIO 并不对它们做显式区分。
```

    adc@0 {
        ...
        #address-cells = <1>;
        #size-cells = <0>;

        channel@0 {
            reg = <0>;
            bipolar;
        };
    };

```
### 1.2 差分通道（Differential channels）

差分电压测量把正输入（IN+）相对于负输入（IN-）在 -VREF 到 +VREF 的范围内进行数字化。换句话说，
差分通道测量 IN+ 与 IN- 之间的电势差，通常用 IN+ - IN- 公式表示。

##### 1.2.1 差分双极性通道（Differential Bipolar Channels）

```

  -------- +VREF ------         +-------------------+
    ´ `       ´ `              /                    |
  /     \   /     \   /   --- <  IN+                |
         `-´       `-´         |                    |
  -------- -VREF ------        |                    |
                               |            ADC     |
  -------- +VREF ------        |                    |
        ´ `       ´ `          |                    |
  \   /     \   /     \   --- <  IN-                |
   `-´       `-´               \       +VREF  -VREF |
  -------- -VREF ------         +-------------------+
                                         ^       ^
                                         |       +---- External -VREF
                                  External +VREF

```
**差分双极性**输入的模拟信号同样允许在 -VREF 到 +VREF 之间摆动。名称中的"双极性"部分意味着
差值（IN+ - IN-）的结果可以为正或为负。如果 -VREF 低于系统 GND，这些也称为差分真双极性输入。
```

    adc@0 {
        ...
        #address-cells = <1>;
        #size-cells = <0>;

        channel@0 {
            reg = <0>;
            bipolar;
            diff-channels = <0 1>;
        };
    };

```
在 ADC 驱动中，会为通道把 `struct iio_chan_spec` 中的 `differential = 1` 置位。尽管有三类通用
输入类型，`differential` 仅用于区分差分与非差分（单端或伪差分）输入类型。更多信息参见
`include/linux/iio/iio.h`。

##### 1.2.2 差分单极性通道（Differential Unipolar Channels）

对于**差分单极性**通道，正输入的模拟电压也必须高于负输入的电压。因此，差分单极性通道实际允许的
输入范围是 IN- 到 +VREF。由于 IN+ 随被测模拟信号摆动，且输入配置必须保证 IN+ 不会低于 IN-（也
不会让 IN- 升高到 IN+ 之上），大多数差分单极性通道配置都把 IN- 固定在一个已知的、不落在被测信号
预期电压范围内的电压上。这就导致一种等效于伪差分通道的配置。因此，差分单极性配置通常可以作为
伪差分单极性通道来支持。

### 1.3 伪差分通道（Pseudo-differential Channels）

还有第三类 ADC 输入类型，称为伪差分，或称单端到差分配置。伪差分通道与差分通道类似，也是测量
IN+ 相对于 IN-。但与双极性差分通道不同，负输入被限制在很窄的电压范围（视为恒定电压），而只有
IN+ 允许摆动。伪差分通道可以由一对差分输入构成：把负输入限制在一个已知电压，同时只允许正输入
摆动。有时，提供给 IN- 的输入称为共模电压（common-mode voltage）。此外，某些器件有一个 COM 引脚，
允许单端输入以共模电压为参考，从而成为伪差分通道。通常，共模输入电压可以在设备树中描述为一个
电压调节器（例如 `com-supply`），因为它本质上是一个恒定电压源。

##### 1.3.1 伪差分单极性通道（Pseudo-differential Unipolar Channels）

```

  -------- +VREF ------          +-------------------+
    ´ `       ´ `               /                    |
  /     \   /     \   /    --- <  IN+                |
         `-´       `-´          |                    |
  --------- IN- -------         |            ADC     |
                                |                    |
  Common-mode voltage -->  --- <  IN-                |
                                \       +VREF  -VREF |
                                 +-------------------+
                                         ^       ^
                                         |       +---- External -VREF
                                  External +VREF

```
**伪差分单极性**输入具有差分单极性通道会有的那些限制，即正输入 IN+ 的模拟电压必须保持在 IN- 到
+VREF 之间。提供给 IN- 的固定电压通常称为共模电压，并且它必须位于 -VREF 到 +VREF 之间，正如对
任何差分通道负输入的信号所预期的那样。

从 IN+ 测得的电压是相对于 IN- 的，但与差分通道不同，伪差分配置旨在测量单端输入信号。为了让应用
程序能够计算出 IN+ 相对于系统地（system ground）的电压，IIO 通道可以提供一个 `_offset` sysfs
属性，在把原始数据转换为电压单位时加到 ADC 输出上。在许多配置中，共模输入电压处于 GND 电平，
因此由于它恒为零，`_offset` 属性被省略。
```

    adc@0 {
        ...
        #address-cells = <1>;
        #size-cells = <0>;

        channel@0 {
            reg = <0>;
            single-channel = <0>;
            common-mode-channel = <1>;
        };
    };

```
不要在伪差分通道的 `iio_chan_spec` 结构体中设置 `differential`。

##### 1.3.2 伪差分双极性通道（Pseudo-differential Bipolar Channels）

```

  -------- +VREF ------          +-------------------+
    ´ `       ´ `               /                    |
  /     \   /     \   /    --- <  IN+                |
         `-´       `-´          |                    |
  -------- -VREF ------         |            ADC     |
                                |                    |
  Common-mode voltage -->  --- <  IN-                |
                                \       +VREF  -VREF |
                                 +-------------------+
                                          ^       ^
                                          |       +---- External -VREF
                                   External +VREF

```
**伪差分双极性**输入不受 IN- 电平的限制，但会受到 -VREF 或 GND 的限制（取决于输入端下限，取决于
具体 ADC）。与它们的单极性对应通道类似，伪差分双极性通道应当声明一个 `_offset` 属性，以便把原始
ADC 数据转换为电压单位。对于把 IN- 连接到 GND 的配置，`_offset` 通常被省略。
```

    adc@0 {
        ...
        #address-cells = <1>;
        #size-cells = <0>;

        channel@0 {
            reg = <0>;
            bipolar;
            single-channel = <0>;
            common-mode-channel = <1>;
        };
    };

```
