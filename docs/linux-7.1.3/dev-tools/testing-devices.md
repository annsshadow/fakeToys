## 使用 kselftest 进行设备测试


目前有若干不同的 kselftest 可用于通用地测试设备，它们在内核覆盖上有部分重叠且要求不同。本文档旨在对每个测试给出一个概览

注意：本文档中的路径是相对于 kselftest 文件夹（`tools/testing/selftests`）的

面向设备kselftest

- Devicetree（`dt`

  - **覆盖范围**：Devicetree 中所描述设备的探测状
  - **要求**：无

- 错误日志（`devices/error_logs`

  - **覆盖范围**：来自任意设备的错误（或更严重）日志消息是否存在
  - **要求**：无

- 可发现总线（`devices/probe`

  - **覆盖范围**：参考文件中描述USB PCI 设备的存在及其探测状
  - **要求**：在 YAML 参考文件中手动描述应被测试的设备（示例
    `devices/probe/boards/google,spherion.yaml`锛。

- 存在性（`devices/exist`

  - **覆盖范围**：所有设备的存在
  - **要求**：在已知良好的内核上生成参考文件（详见
    `devices/exist/README.rst`锛。

因此，建议在所有的（基DT 的）平台上都启用错误日志Devicetree 测试，因为它们没有任何要求。然后，为了大幅提升覆盖率，为每个平台生成参考文件并启用 exist 测试。可发现总线测试可用于验证特USB PCI 设备的探测状态，但在大多数情况下可能并不值得
