
## 数字电视条件接收接口（CI

   This documentation is outdated.

本文档描述高CI API 的用法，遵循 Linux DVB API。这不是对现有低CI API 的文档

   对于 Twinhan/Twinhan 克隆卡，dst_ca 模块负责 CI 的硬件处理。如果检测到一CI
   （Common Interface，即容纳 CAM（Conditional Access Module，条件接收模块）的接口）   该模块会自动加载
#### ca_zap


`ca_zap` 这样的用户空间应用程序是处理加密MPEG-TS 流所必需的
`ca_zap` 用户态应用程序负责将解扰（descrambling）相关信息发送给条件接收模块（CAM）
就目前而言，该应用程序需要以下条件才能正常工作
a) 使用 szap 调谐到一个有效频道
  eg: $ szap -c channels.conf -r "TMC" -x

b) 一个包含有PMT PID channels.conf

  eg: TMC:11996:h:0:27500:278:512:650:321

  这里278 是一个有效的 PMT PID。其余的值与 szap 使用的值相同
c) 运行 szap 之后，你必须运行 ca_zap，解扰器才能工作
  eg: $ ca_zap channels.conf "TMC"

d) 希望你能像使FTA 卡一样欣赏你订阅的喜爱频道

  目前 ca_zap dst_test 都仅用于演示目的，如有必要它们可以发展为完整的应用程序

#### 属于此类别的

目前属于此类别的卡有 Twinhan 及其克隆卡，这些卡以 VVMER、Tomato、Hercules、Orange 等名称销售
#### 受支持的 CI 模块


CI 模块的支持在很大程度上取决于卡上的固件。有些卡确实支持几乎全部可用CI 模块。要让这些卡支持额外CI 模块，目前没有什么太多可做的
目前该驱动已测试过的模块有：

(1) SCM 鐨?Irdeto 1 鍜?2
(2) SCM 鐨?Viaccess
(3) Dragoncam
