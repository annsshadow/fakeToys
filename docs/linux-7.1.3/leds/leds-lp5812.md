
## 内核驱动 lp5812


- TI/National Semiconductor LP5812 LED 驱动
- Datasheet: https://www.ti.com/product/LP5812#tech-docs

Authors: Jared Zhou <jared-zhou@ti.com>

## 描述


LP5812 是一个 4x3 矩阵 LED 驱动，支持手动和自主动画控制。该驱动提供 sysfs 接口来控制和配置 LP5812 设备及其 LED 通道。

## Sysfs 接口


该驱动使用 Documentation/ABI/testing/sysfs-class-led-multicolor.rst 中定义的标准多色 LED class 接口。

每个 LP5812 LED 输出出现在 `/sys/class/leds/` 下，带有其分配的标签（例如 `LED_A`）。

暴露以下属性：
  - multi_intensity：每通道 RGB 强度控制
  - brightness：标准亮度控制（0-255）

## 自主控制模式


该驱动还支持通过设备树中定义的模式配置（例如 direct、tcmscan 或 mixscan 模式）进行自主控制。配置后，LP5812 可以在没有 CPU 干预的情况下生成过渡和颜色效果。

有关有效的模式字符串和配置示例，请参阅设备树绑定文档。

## 示例使用


```
    # 设置 RGB 强度（R=50, G=50, B=50）
    echo 50 50 50 > /sys/class/leds/LED_A/multi_intensity
    # 设置整体亮度为最大
    echo 255 > /sys/class/leds/LED_A/brightness

```
