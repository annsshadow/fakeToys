## 使用 GPIO 的子系统驱动


注意，标准内核驱动已存在于常见的 GPIO 任务，并且会为该工作提供正确的内核态与用户态
API/ABI，而且这些驱动可以借助设备树或 ACPI 等硬件描述，非常容易地与其他内核子系统
互联：

- leds-gpio：drivers/leds/leds-gpio.c 将处理连接到 GPIO 线的 LED，为你提供 LED 的
  sysfs 接口

- ledtrig-gpio：drivers/leds/trigger/ledtrig-gpio.c 将提供一个 LED 触发器，
  即一个 LED 会根据 GPIO 线变为高电平或低电平而亮/灭
  （而该 LED 进而又可能如上所述使用 leds-gpio）。

- gpio-keys：drivers/input/keyboard/gpio_keys.c 用于当你的 GPIO 线能
  在按键按下时产生中断的情况。也支持去抖。

- gpio-keys-polled：drivers/input/keyboard/gpio_keys_polled.c 用于当你的
  GPIO 线无法产生中断、从而需要由定时器周期性轮询的情况。

- gpio_mouse：drivers/input/mouse/gpio_mouse.c 用于通过仅使用 GPIO 而无需鼠标端口
  来提供一个最多三键的鼠标。你可以剪断鼠标线缆并将导线连到 GPIO 线，或将鼠标连接器
  焊接到这些线上以获得更持久的此类方案。

- gpio-beeper：drivers/input/misc/gpio-beeper.c 用于通过连接到 GPIO 线的外部扬声器
  发出蜂鸣声。（如果蜂鸣由开/关控制，而要产生真正的 PWM 波形，请见下文 pwm-gpio。）

- pwm-gpio：drivers/pwm/pwm-gpio.c 用于以高分辨率定时器翻转 GPIO，在 GPIO 线上产生
  PWM 波形，正如 Linux 高分辨率定时器所能做到的那样。

- extcon-gpio：drivers/extcon/extcon-gpio.c 用于当你需要读取外部连接器状态（例如音频
  驱动器的耳机线或 HDMI 连接器）时。它会提供比 GPIO 更好的用户态 sysfs 接口。

- restart-gpio：drivers/power/reset/gpio-restart.c 用于通过拉低一条 GPIO 线来重启/
  重新引导系统，并将注册一个重启处理器，以便用户态可以发出正确的系统调用来重启系统。

- poweroff-gpio：drivers/power/reset/gpio-poweroff.c 用于通过拉低一条 GPIO 线来关闭
  系统电源，并将注册一个 pm_power_off() 回调，以便用户态可以发出正确的系统调用来关闭
  系统电源。

- gpio-gate-clock：drivers/clk/clk-gpio.c 用于控制一个使用 GPIO 的受控时钟
  （开/关），并与时钟子系统集成。

- i2c-gpio：drivers/i2c/busses/i2c-gpio.c 用于通过翻转（bitbang）两条 GPIO 线来驱动
  一个 I2C 总线（两条线，SDA 与 SCL 线）。它对系统而言将如同任何其它 I2C 总线一样出现，
  并使得可以像连接任何其它 I2C 总线驱动那样，连接总线上 I2C 设备的驱动。

- spi_gpio：drivers/spi/spi-gpio.c 用于通过 GPIO 翻转（bitbang）来驱动一个 SPI 总线
  （可变数量的线，至少 SCK，以及可选的 MISO、MOSI 与片选线）。它对系统而言将如同任何
  其它 SPI 总线一样出现，并使得可以像连接任何其它 SPI 总线驱动那样，连接总线上 SPI 设备
  的驱动。例如，任何 MMC/SD 卡随后都可以通过来自 MMC/SD 卡子系统的 mmc_spi 主机连接到
  此 SPI。

- w1-gpio：drivers/w1/masters/w1-gpio.c 用于通过一条 GPIO 线驱动单总线（one-wire），
  与 W1 子系统集成，并像处理任何其它 W1 设备那样处理总线上的设备。

- gpio-fan：drivers/hwmon/gpio-fan.c 用于控制连接到一条 GPIO 线（以及可选地一条 GPIO
  告警线）的风扇来为系统散热，提供所有正确的内核态与 sysfs 接口，使你的系统不会过热。

- gpio-regulator：drivers/regulator/gpio-regulator.c 用于通过拉低一条 GPIO 线来控制
  提供某一电压的稳压器（regulator），与稳压器子系统集成，并为你提供所有正确的接口。

- gpio-wdt：drivers/watchdog/gpio_wdt.c 用于提供一个看门狗定时器，它将周期性地通过
  从 1 到 0 再到 1 地翻转连接到 GPIO 线的硬件来“ping”它。如果该硬件没有周期性地收到
  它的“ping”，它就会重置系统。

- gpio-nand：drivers/mtd/nand/raw/gpio.c 用于将 NAND 闪存芯片连接到一组简单的 GPIO 线：
  RDY、NCE、ALE、CLE、NWP。它与 NAND 闪存 MTD 子系统交互，并提供与其它任何 NAND 驱动
  硬件一样的芯片访问与分区解析。

- ps2-gpio：drivers/input/serio/ps2-gpio.c 用于通过翻转两条 GPIO 线来驱动 PS/2（IBM）
  serio 总线、数据与时钟线。它对系统而言将如同任何其它 serio 总线一样出现，并使得可以
  连接例如键盘以及其它基于 PS/2 协议的设备的驱动。

- cec-gpio：drivers/media/platform/cec-gpio/ 用于仅使用 GPIO 来与 CEC（消费电子控制）
  总线交互。它用于与 HDMI 总线上的设备通信。

- gpio-charger：drivers/power/supply/gpio-charger.c 用于当你需要做电池充电，而所有
  可以用来检查交流充电器存在与否、或诸如使用 GPIO 线指示充电状态等更复杂任务的依据
  只有 GPIO 线时，该驱动提供这些功能，并且还提供了一种清晰定义的方式，用于从设备树等
  硬件描述传递充电参数。

- gpio-mux：drivers/mux/gpio.c 用于通过 n 条 GPIO 线控制一个多路复用器，从而你可以通过
  激活不同的 GPIO 线来多路复用进 2^n 个不同的设备。GPIO 通常位于 SoC 上，而设备是
  某些 SoC 外部的实体，例如 PCB 上可以有选择地启用的不同组件。

除了这些之外，还有一些特殊 GPIO 驱动位于 MMC/SD 等子系统中，用于读取卡检测与写保护
GPIO 线；以及位于 TTY 串行子系统中，用于通过使用两条 GPIO 线来模拟 MCTRL（调制解调器
控制）信号 CTS/RTS。MTD NOR 闪存也有用于额外 GPIO 线的附加件，尽管地址总线通常直接
连到闪存。

请使用这些驱动，而不要从用户态直接操作 GPIO；它们比你的用户态代码能更好地与内核
框架集成。不用说，仅仅使用适当的内核驱动，就能通过提供现成的组件，特别是简化并加速
你的嵌入式开发。
