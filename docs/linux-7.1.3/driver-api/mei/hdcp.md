## HDCP锛。


ME FW 作为一个安全引擎，提供Intel 图形设备HDCP2.2 接收端（sink）之间建HDCP2.2 协议协商的能力

ME FW 根据 HDCP 2.2 规范准备 HDCP2.2 协商参数，并对其签名和加密。Intel 图形将生成的二进制数据块发送给 HDCP2.2 接收端

类似地，HDCP2.2 接收端的响应被传送回 ME FW 进行解密与验证

一HDCP2.2 协商的所有步骤完成，ME FW 将在收到请求时将该端口配置为已认证状态，并向 Intel 图形硬件提供 HDCP 加密密钥


### mei_hdcp 驱动


    :doc: MEI_HDCP 客户端驱

### mei_hdcp API


    :functions:
