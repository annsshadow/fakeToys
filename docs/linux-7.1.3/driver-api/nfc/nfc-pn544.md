## NXP Semiconductors PN544 近场通信芯片的内核驱动程



### 一般的


PN544是一款非接触式集成传输模
沟通。驱动程序位drivers/nfc/ 下并被编译为
模块名为“pn544”

主机接口：I2C、SPI HSU，该驱动程序目前仅支I2C

### 协议


在正常（HCI）模式和固件更新模式下读取和
写函数的行为有点不同，因为消息格
或者协议不同

在正(HCI) 模式下，使用的协议源ETSI
人机交互规范。使用特定协议更新固件，
这与人机交互不同

HCI 消息8 位标头和消息正文组成。这
标头包含消息长度HCI 消息的最大大小为
33. HCI 模式下，测试发送的消息是否正确
校验和。固件更新消息的长度为秒 (MSB)
和消息的第三个（LSB）字节。最FW 消息长度
1024 字节

有关 ETSI HCI 规范，请参阅
http://www.etsi.org/WebSite/Technologies/ProtocolSpecification.aspx
