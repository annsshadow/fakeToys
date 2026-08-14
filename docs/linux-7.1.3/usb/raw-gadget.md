## USB Raw Gadget（原始小工具）


USB Raw Gadget 是一个小工具（gadget）驱动，它让用户可以空间对 gadget 的通信过程进行底层控制。

与任何其他 gadget 驱动一样，Raw Gadget 通过 USB gadget API 实现 USB 设备。与大多数 gadget 驱动不同，Raw Gadget 自身不实现任何具体的 USB 功能，而是需要用户空间来完成。

Raw Gadget 目前严格来说是一个调试特性，不应在生产环境中使用。请改用 GadgetFS。

通过 CONFIG_USB_RAW_GADGET 启用。

#### 与 GadgetFS 的对比


Raw Gadget 与 GadgetFS 类似，但为用户空间提供了对 USB gadget 层更直接的访问。关键区别在于：

1. Raw Gadget 将每个 USB 请求传递给用户空间以获取响应，而 GadgetFS 根据所提供的描述符在内部响应某些 USB 请求。注意，UDC 驱动可能会自行响应某些请求，且永远不会将它们转发到 gadget 层。

2. Raw Gadget 允许提供任意数据作为对 USB 请求的响应，而 GadgetFS 对所提供的 USB 描述符执行健全性检查。这使得 Raw Gadget 适合通过提供畸形数据作为对 USB 请求的响应来进行模糊测试（fuzzing）。

3. Raw Gadget 提供了一种选择要绑定的 UDC 设备/驱动的方法，而 GadgetFS 当前绑定到第一个可用的 UDC。这允许多个 Raw Gadget 实例绑定到不同的 UDC。

4. Raw Gadget 显式暴露有关端点地址和能力的信息。这使得用户可以编写与 UDC 无关的 gadget。

5. Raw Gadget 具有基于 ioctl 的接口，而不是基于文件系统的接口。

#### 用户空间接口


用户可以通过打开 `/dev/raw-gadget` 并发起 ioctl 调用来与 Raw Gadget 交互；详见 include/uapi/linux/usb/raw_gadget.h 中的注释。多个 Raw Gadget 实例（绑定到不同的 UDC）可以同时被使用。

Raw Gadget 的典型使用场景：

1. 通过打开 `/dev/raw-gadget` 创建一个 Raw Gadget 实例。
2. 通过 `USB_RAW_IOCTL_INIT` 初始化该实例。
3. 通过 `USB_RAW_IOCTL_RUN` 启动该实例。
4. 在循环中发起 `USB_RAW_IOCTL_EVENT_FETCH` 以接收来自 Raw Gadget 的事件，并根据需要实现何种 USB gadget 来作出反应。

请注意，某些 UDC 驱动为端点分配了固定地址，因此描述符中不能使用任意端点地址。尽管如此，Raw Gadget 提供了一种与 UDC 无关的编写 USB gadget 的方法。一旦通过 `USB_RAW_IOCTL_EVENT_FETCH` 收到 `USB_RAW_EVENT_CONNECT`，就可以使用 `USB_RAW_IOCTL_EPS_INFO` 来查明 UDC 驱动所拥有的端点信息。在此基础上，用户空间必须为 gadget 选择 UDC 端点，并在端点描述符中相应地分配地址。

Raw Gadget 的使用示例和测试套件：

https://github.com/xairy/raw-gadget

#### 内部细节


每个 Raw Gadget 端点的读/写 ioctl 都会提交一个 USB 请求并等待其完成。这样做是故意的，以便通过单个系统调用完整处理单个 USB 请求来辅助覆盖率引导的模糊测试。这一特性必须在实现中保留。

#### 潜在的未来改进


- 支持 `O_NONBLOCK` I/O。这将是另一种操作模式，Raw Gadget 不会等待每个 USB 请求完成。

- 支持 USB 3 特性（在启用端点时接受 SS 端点伴随描述符；允许为批量传输提供 `stream_id`）。

- 支持等时（ISO）传输特性（为已完成的请求暴露 `frame_number`）。
