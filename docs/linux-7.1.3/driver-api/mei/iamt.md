
## Intel(R) 主动管理技术（Intel AMT）


Intel ME 接口一个突出的用途是与运行在 Intel ME 上的固件中实现的 Intel(R) 主动管理技术（Intel AMT）进行通信。

Intel AMT 提供了远程带外（OOB）管理主机的能力，即使运行在主机处理器上的操作系统已经崩溃或处于睡眠状态。

Intel AMT 的一些使用示例如下：
   - 监控硬件状态与平台组件
   - 远程断电/上电（对绿色计算或夜间 IT 维护很有用）
   - 操作系统更新
   - 存储有用的平台信息，例如软件资产
   - 内置硬件 KVM
   - 基于远程管理控制台所设策略，对以太网与 IP 协议流进行选择性网络隔离
   - 来自远程管理控制台的 IDE 设备重定向

Intel AMT（OOB）通信基于 SOAP（自 6.0 版本起已弃用）over HTTP/S，或基于 WS-Management 协议 over HTTP/S，这些请求来自远程管理控制台应用程序。

关于 Intel AMT 的更多信息：
https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm


### Intel AMT 应用程序


    1) Intel 本地管理服务（Intel LMS）

       在平台上本地运行的应用程序与 Intel AMT 2.0 及更高版本通信的方式，同网络应用程序通过 SOAP over HTTP（自 6.0 版本起已弃用）或 WS-Management over SOAP over HTTP 通信的方式一致。这意味着某些 Intel AMT 特性可以从本地应用程序访问，使用与通过网络与 Intel AMT 通信的远程应用程序相同的网络接口。

       当本地应用程序发送一条发往本地 Intel AMT 主机名的消息时，监听发往该主机名流量的 Intel LMS 会拦截该消息并将其路由到 Intel MEI。
       更多信息：
       https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm
       在 "About Intel AMT" => "Local Access" 下

       下载 Intel LMS：
       https://github.com/intel/lms

       Intel LMS 使用 Intel MEI 驱动，通过定义的 GUID 打开到 Intel LMS 固件特性的连接，然后使用该特性进行通信，通信采用一种称为 Intel AMT 端口转发协议（Intel APF 协议）的协议。该协议用于从单一应用程序维护与 Intel AMT 的多个会话。

       协议规范参见 Intel AMT 软件开发工具包（SDK）
       https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm
       在 "SDK Resources" => "Intel(R) vPro(TM) Gateway (MPS)"
       => "Information for Intel(R) vPro(TM) Gateway Developers"
       => "Description of the Intel AMT Port Forwarding (APF) Protocol" 下

    2) 使用本地代理进行 Intel AMT 远程配置

       本地代理使 IT 人员能够开箱即用地配置 Intel AMT，而无需安装额外的数据来启用设置。远程配置过程可能涉及一个运行在主机上的、由 ISV 开发的远程配置代理。
       更多信息：
       https://software.intel.com/sites/manageability/AMT_Implementation_and_Reference_Guide/default.htm
       在 "Setup and Configuration of Intel AMT" =>
       "SDK Tools Supporting Setup and Configuration" =>
       "Using the Local Agent Sample" 下

### Intel AMT 操作系统健康看门狗


Intel AMT 看门狗是一个操作系统健康（挂起/崩溃）看门狗。
每当操作系统挂起或崩溃时，Intel AMT 会向该事件的任何订阅者发送一个事件。这一机制意味着，即使主机发生硬性故障，IT 也能知道平台何时崩溃。

Intel AMT 看门狗由两部分组成：
    1) 固件特性 —— 接收心跳，并在心跳停止时发送事件。
    2) Intel MEI iAMT 看门狗驱动 —— 连接到看门狗特性，配置看门狗并发送心跳。

Intel iAMT 看门狗 MEI 驱动使用内核看门狗 API 来配置 Intel AMT 看门狗并向其发送心跳。看门狗的默认超时时间为 120 秒。

如果固件中未启用 Intel AMT，则看门狗客户端不会在 me 客户端总线上枚举，看门狗设备也不会被暴露。

---
linux-mei@linux.intel.com
