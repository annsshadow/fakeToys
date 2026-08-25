
## 高层 CI API


   本文档已过时
本文档根Linux DVB API 描述高层 CI API
通过高层 CI 方法，任何具有几乎任意随机架构的新卡都可以用这种风格实现，switch 语句中的定义可以轻松适配
任何卡，从而无需任何额外ioctl
缺点在于驱动/硬件必须管理其余部分。对于应用程序员来说，这就像Linux DVB API 中定义的 CI ioctl
发接收数组一样简单。为了容纳此特性，API 没有做任何改动

#### 为何需要另一CI 接口

这是最常问的问题之一。嗯，这是个好问题。严格来说，这不是一个新接口
CI 接口DVB API ca.h 中定义为
	typedef struct ca_slot_info {
		int num;               /** 妲戒綅鍙?**/

		int type;              /** 此槽位支持的 CA 接口 **/
	#define CA_CI            1     /** CI 高层接口 **/
	#define CA_CI_LINK       2     /** CI 閾捐矾灞傛帴鍙?**/
	#define CA_CI_PHYS       4     /** CI 鐗╃悊灞傛帴鍙?**/
	#define CA_DESCR         8     /** 内置解扰**/
	#define CA_SC          128     /** 简单智能卡接口 **/

		unsigned int flags;
	#define CA_CI_MODULE_PRESENT 1 /** 模块（或卡）已插**/
	#define CA_CI_MODULE_READY   2
	} ca_slot_info_t;

CI 接口遵循 CI 高层接口，而大多数应用程序并未实现它。因此重新审视了这一领域
CI 接口相当不同，因为它试图容纳所有落入其他类别的、基CI 的其他设备
这意味着CI 接口仅在应用层处EN50221 风格标签，会话管理不由应用程序负责。驱硬件将负责所有这些
此接口纯粹是一个交APDU EN50221 接口。这意味着在应用程序到驱动的通信中不存在会话管理、链路层传输层。就这么简单。驱硬件必须负责这些
通过此高CI 接口，可以使用常ioctl 来定义接口
所有这ioctl 对高CI 接口同样有效

#define CA_RESET          _IO('o', 128)
#define CA_GET_CAP        _IOR('o', 129, ca_caps_t)
#define CA_GET_SLOT_INFO  _IOR('o', 130, ca_slot_info_t)
#define CA_GET_DESCR_INFO _IOR('o', 131, ca_descr_info_t)
#define CA_GET_MSG        _IOR('o', 132, ca_msg_t)
#define CA_SEND_MSG       _IOW('o', 133, ca_msg_t)
#define CA_SET_DESCR      _IOW('o', 134, ca_descr_t)


查询设备时，设备产生如下信息

# 	CA_GET_SLOT_INFO

	Command = [info]
	APP: Number=[^1^]
	APP: Type=[^1^]
	APP: flags=[^1^]
	APP: CI 高层接口
	APP: CA/CI 模块已插
# 	CA_GET_CAP

	Command = [caps]
	APP: Slots=[^1^]
	APP: Type=[^1^]
	APP: 解扰器密[^16^]
	APP: Type=[^1^]

# 	CA_SEND_MSG

	Descriptors(Program Level)=[ 09 06 06 04 05 50 ff f1]
	Found CA descriptor @ program level

	(20) ES type=[^2^] ES pid=[^201^]  ES length =[0 (0x0)]
	(25) ES type=[^4^] ES pid=[^301^]  ES length =[0 (0x0)]
	ca_message length is 25 (0x19) bytes
	EN50221 CA MSG=[ 9f 80 32 19 03 01 2d d1 f0 08 01 09 06 06 04 05 50 ff f1 02 e0 c9 00 00 04 e1 2d 00 00]


并非 API 中的所ioctl 都在驱动中实现，那些无法通过 API 实现的硬件其他特性则使用 CA_GET_MSG CA_SEND_MSG ioctl 来实现。使用一EN50221 风格的包装器来交换数据，以与其他硬件保持兼容

	/** 来自/发往 CI-CAM 的消**/
	typedef struct ca_msg {
		unsigned int index;
		unsigned int type;
		unsigned int length;
		unsigned char msg[^256^];
	} ca_msg_t;


数据的流向可以描述如下：


	App (User)
	-----
	parse
	  |
	  |
	  v
# 	en50221 APDU（打包）

   |	  |				| High Level CI driver
   |	  |				|
   |	  v				|
   |	en50221 APDU（解包）	|
   |	  |				|
   |	  |				|
   |	  v				|
   |	完整性检		|
   |	  |				|
   |	  |				|
   |	  v				|
#    |	do（依赖硬件）		|

	  |    Hardware
	  |
	  v

高层 CI 接口使用 EN50221 DVB 标准，遵循标准确保了面向未来