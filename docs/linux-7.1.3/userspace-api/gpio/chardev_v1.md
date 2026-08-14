
## GPIO 字符设备用户空间 API（v1）


   本 API 已被 chardev.rst（v2）取代。

   新开发应使用 v2 API，并鼓励已有开发尽快迁移，因为此 API 将在未来被移除。v2 API 是 v1 API
   在功能上的超集，因此任何 v1 调用都可以直接翻译为等价的 v2 调用。

   在迁移期间，此接口将继续得到维护，但新特性只会添加到新的 API 中。

首次加入于 4.8。

该 API 围绕三个主要对象构建：gpio-v1-chip、gpio-v1-line-handle 与 gpio-v1-line-event。

当本文档中使用“line event”时，它指的是可以监视一条线路上边沿事件的请求，而不是边沿事件本身。

## 芯片


Chip 代表一个单独的 GPIO 芯片，并通过形如 `/dev/gpiochipX` 的设备文件暴露给用户空间。

每个芯片支持若干条 GPIO 线，`chip.lines<gpiochip_info>`。芯片上的线由一个在从 0 到
`chip.lines - 1` 范围内的 `offset` 标识，即 `[0,chip.lines)`。

线通过 gpio-get-linehandle-ioctl.rst 从芯片请求，得到的线句柄用于访问 GPIO 芯片的线；或
通过 gpio-get-lineevent-ioctl.rst，得到的线事件用于监视一条 GPIO 线上的边沿事件。

在本文档中，在 GPIO 设备文件上调用 `open()` 返回的文件描述符被称为 `chip_fd`。

### 操作


可以对芯片执行以下操作：

- [获取线句柄](gpio-get-linehandle-ioctl)
- [获取线事件](gpio-get-lineevent-ioctl)
- [获取芯片信息](gpio-get-chipinfo-ioctl)
- [获取线信息](gpio-get-lineinfo-ioctl)
- [监视线信息](gpio-get-lineinfo-watch-ioctl)
- [取消监视线信息](gpio-get-lineinfo-unwatch-ioctl)
- [读取线信息变更事件](gpio-lineinfo-changed-read)

## 线句柄


线句柄由 gpio-get-linehandle-ioctl.rst 创建，提供对一组已请求线的访问。线句柄通过
gpio-get-linehandle-ioctl.rst 在 `request.fd<gpiohandle_request>` 中返回的匿名文件描述符
暴露给用户空间。

在本文档中，线句柄文件描述符被称为 `handle_fd`。

### 操作


可以对线句柄执行以下操作：

- [获取线值](gpio-handle-get-line-values-ioctl)
- [设置线值](gpio-handle-set-line-values-ioctl)
- [重新配置线](gpio-handle-set-config-ioctl)

## 线事件


线事件由 gpio-get-lineevent-ioctl.rst 创建，提供对一条已请求线的访问。线事件通过
gpio-get-lineevent-ioctl.rst 在 `request.fd<gpioevent_request>` 中返回的匿名文件描述符
暴露给用户空间。

在本文档中，线事件文件描述符被称为 `event_fd`。

### 操作


可以对线事件执行以下操作：

- [获取线值](gpio-handle-get-line-values-ioctl)
- [读取线边沿事件](gpio-lineevent-data-read)

## 类型


本节包含 ABI v1 所引用的结构体。

`struct gpiochip_info<gpiochip_info>` 在 ABI v1 与 v2 中通用。

   :identifiers:
    gpioevent_data
    gpioevent_request
    gpiohandle_config
    gpiohandle_data
    gpiohandle_request
    gpioline_info
    gpioline_info_changed

- [error-codes](error-codes)
