
## GPIO 字符设备用户空间 API


这是字符设备 API 的最新版本（v2），如 `include/uapi/linux/gpio.h.` 所定义。

首次添加于 5.10。

   不要滥用用户空间 API 来控制已有合适内核驱动的硬件。可能已经有适合你用例的驱动，而现有的内核驱动必定比从用户空间位操作（bitbashing）提供更优的方案。

   请阅读 Documentation/driver-api/gpio/drivers-on-gpio.rst 以避免在用户空间中重新发明内核轮子。

   同样，对于多功能线路，可能有其他子系统，如 Documentation/spi/index.rst、Documentation/i2c/index.rst、Documentation/driver-api/pwm.rst、Documentation/w1/index.rst 等，为你的硬件提供合适的驱动和 API。

使用字符设备 API 的基本示例可在 `tools/gpio/*` 中找到。

该 API 围绕两个主要对象构建：gpio-v2-chip 和 gpio-v2-line-request。


## Chip


Chip 代表单个 GPIO 芯片，并通过形如 `/dev/gpiochipX` 的设备文件暴露给用户空间。

每个芯片支持一定数量的 GPIO 线路，`chip.lines<gpiochip_info>`。芯片上的线路通过范围从 0 到 `chip.lines - 1` 的 `offset` 标识，即 `[0,chip.lines)`。

线路通过 gpio-v2-get-line-ioctl.rst 从芯片请求，所得的行请求用于访问 GPIO 芯片的线路或监视线路的边沿事件。

在本文档中，在 GPIO 设备文件上调用 `open()` 返回的文件描述符称为 `chip_fd`。

### 操作


可对 chip 执行以下操作：

- [获取线路](gpio-v2-get-line-ioctl)
- [获取芯片信息](gpio-get-chipinfo-ioctl)
- [获取线路信息](gpio-v2-get-lineinfo-ioctl)
- [监视线路信息](gpio-v2-get-lineinfo-watch-ioctl)
- [取消监视线路信息](gpio-get-lineinfo-unwatch-ioctl)
- [读取线路信息变更事件](gpio-v2-lineinfo-changed-read)


## 行请求（Line Request）


行请求由 gpio-v2-get-line-ioctl.rst 创建，并提供对一组被请求线路的访问。行请求通过 gpio-v2-get-line-ioctl.rst 在 `request.fd<gpio_v2_line_request>` 中返回的匿名文件描述符暴露给用户空间。

在本文档中，行请求文件描述符称为 `req_fd`。

### 操作


可对行请求执行以下操作：

- [获取线路值](gpio-v2-line-get-values-ioctl)
- [设置线路值](gpio-v2-line-set-values-ioctl)
- [读取线路边沿事件](gpio-v2-line-event-read)
- [重新配置线路](gpio-v2-line-set-config-ioctl)

## 类型


本节包含 API v2 所引用的结构体和枚举，定义于 `include/uapi/linux/gpio.h`。

   :identifiers:
    gpio_v2_line_attr_id
    gpio_v2_line_attribute
    gpio_v2_line_changed_type
    gpio_v2_line_config
    gpio_v2_line_config_attribute
    gpio_v2_line_event
    gpio_v2_line_event_id
    gpio_v2_line_flag
    gpio_v2_line_info
    gpio_v2_line_info_changed
    gpio_v2_line_request
    gpio_v2_line_values
    gpiochip_info

- [错误码](error-codes)
