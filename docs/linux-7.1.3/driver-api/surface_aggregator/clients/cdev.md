


## 用户空间 EC 接口（cdev）


`surface_aggregator_cdev` 模块为 SSAM 控制器提供一个 misc 设备（混杂设备），
以便从用户空间到 SAM EC 建立（或多或少）直接的连接。它旨在用于开发和调试，
因此不应以任何其他方式使用或依赖它。注意该模块不会自动加载，必须手动加载。

所提供的接口可通过 `/dev/surface/aggregator` 设备文件访问。该接口的全部功能都
通过 IOCTL 提供。这些 IOCTL 及其各自的输入/输出参数结构体定义在
`include/uapi/linux/surface_aggregator/cdev.h` 中。

用于访问该接口的小型 python 库与脚本可在此处找到：
https://github.com/linux-surface/surface-aggregator-module/tree/master/scripts/ssam。



## 接收事件


可以通过从设备文件读取来接收事件。它们由 |ssam_cdev_event| 数据类型表示。

然而，在事件可以被读取之前，必须经由 `SSAM_CDEV_NOTIF_REGISTER` IOCTL 注册
所需的通知器（notifier）。通知器本质上是回调，在 EC 发送事件时被调用。在此
接口中，它们与特定的目标类别和设备文件实例相关联。它们将该类别的任何事件
转发到相应实例的缓冲区，随后即可从中读取。

通知器本身并不会在 EC 上启用事件。因此，可能还需要通过 `SSAM_CDEV_EVENT_ENABLE`
IOCTL 来启用事件。虽然通知器是每客户端（即每设备文件实例）工作的，但事件的
启用是全局的，针对 EC 及其所有客户端（无论用户空间还是非用户空间）。
`SSAM_CDEV_EVENT_ENABLE` 和 `SSAM_CDEV_EVENT_DISABLE` IOCTL 会对事件进行引用
计数，因此只要存在请求过它的客户端，事件就保持启用。

注意，一旦客户端实例关闭，已启用的事件并不会被自动禁用。因此任何客户端进程
（或进程组）都应使其事件启用调用与相应的事件禁用调用相平衡。然而，在不同的
客户端实例上启用和禁用事件是完全合法的。例如，可以在客户端实例 `A` 上建立
通知器并读取事件，在实例 `B` 上启用这些事件（注意由于事件是全局启用/禁用的，
这些事件也会被 A 收到），在不再需要事件后，通过实例 `C` 禁用先前已启用的事件。


## 控制器 IOCTL


提供以下 IOCTL：

   :widths: 1 1 1 1 4
   :header-rows: 1

   - - 类型
     - 编号
     - 方向
     - 名称
     - 描述

   - - `0xA5`
     - `1`
     - `WR`
     - `REQUEST`
     - 执行同步 SAM 请求。

   - - `0xA5`
     - `2`
     - `W`
     - `NOTIF_REGISTER`
     - 注册事件通知器。

   - - `0xA5`
     - `3`
     - `W`
     - `NOTIF_UNREGISTER`
     - 注销事件通知器。

   - - `0xA5`
     - `4`
     - `W`
     - `EVENT_ENABLE`
     - 启用事件源。

   - - `0xA5`
     - `5`
     - `W`
     - `EVENT_DISABLE`
     - 禁用事件源。


### ``SSAM_CDEV_REQUEST``


定义为 `_IOWR(0xA5, 1, struct ssam_cdev_request)`。

执行一个同步 SAM 请求。请求规范以 |ssam_cdev_request| 类型的参数传入，然后由
IOCTL 写入/修改，以返回请求的状态和结果。

请求负载数据必须单独分配，并通过 `payload.data` 和 `payload.length` 成员传入。
如果需要响应，响应缓冲必须由调用者分配，并通过 `response.data` 成员传入。
`response.length` 成员必须设置为该缓冲的容量，若不需要响应则设为零。请求完成时，
调用会将响应写入响应缓冲（若其容量允许），并用响应的实际字节大小覆盖 length 字段。

此外，如果请求有响应，必须通过请求标志来指示，就像内核内请求那样。请求标志可
通过 `flags` 成员设置，其值对应于 |ssam_cdev_request_flags| 中的值。

最后，请求本身的状态在 `status` 成员中返回（负值 errno 表示失败）。注意，IOCTL
的失败指示与请求的失败指示是分开的：如果在请求建立过程中（`-EFAULT`）或提供的
参数及其任一字段无效（`-EINVAL`）时发生任何失败，IOCTL 会返回负的 status 码。
此时可能会设置请求参数的 status 值，以更详细地说明出错原因（例如 `-ENOMEM` 表示
内存不足），但该值也可能为零。如果请求已在 IOCTL 内部成功建立、提交并完成
（即交还给用户空间），IOCTL 会以零 status 码返回，但若请求在提交后实际执行失败，
请求的 `status` 成员仍可能为负值。

参数结构体的完整定义如下。

### ``SSAM_CDEV_NOTIF_REGISTER``


定义为 `_IOW(0xA5, 2, struct ssam_cdev_notifier_desc)`。

为给定通知器描述中指定的事件目标类别，以指定优先级注册一个通知器。注册通知器
是接收事件的必要条件，但它本身并不会启用事件。为某个特定目标类别注册通知器后，
该类别的所有事件都将被转发到用户空间客户端，并随后可从设备文件实例读取。注意，
可能还需要启用事件，例如通过 `SSAM_CDEV_EVENT_ENABLE` IOCTL，EC 才会发送它们。

每个目标类别和客户端实例只能注册一个通知器。如果通知器已经注册，该 IOCTL 将
以 `-EEXIST` 失败。

当设备文件实例关闭时，通知器会自动被移除。

### ``SSAM_CDEV_NOTIF_UNREGISTER``


定义为 `_IOW(0xA5, 3, struct ssam_cdev_notifier_desc)`。

注销与指定目标类别相关联的通知器。该 IOCTL 会忽略 priority 字段。如果此客户端
实例和给定类别尚未注册任何通知器，该 IOCTL 将失败并返回 `-ENOENT`。

### ``SSAM_CDEV_EVENT_ENABLE``


定义为 `_IOW(0xA5, 4, struct ssam_cdev_event_desc)`。

启用与给定事件描述符相关联的事件。

注意，该调用本身不会注册通知器，它只会在控制器上启用事件。如果你想通过读取
设备文件来接收事件，你需要在该实例上注册相应的通知器。

当设备文件关闭时，事件不会被自动禁用。这必须通过调用 `SSAM_CDEV_EVENT_DISABLE`
IOCTL 手动完成。

### ``SSAM_CDEV_EVENT_DISABLE``


定义为 `_IOW(0xA5, 5, struct ssam_cdev_event_desc)`。

禁用与给定事件描述符相关联的事件。

注意，这不会注销任何通知器。在该调用之后，事件仍可能被接收并转发到用户空间。
停止接收事件的唯一安全方式是注销所有先前已注册的通知器。


## 结构体与枚举
