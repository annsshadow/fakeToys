import os

F = r"D:/WORKSPACE/linux-7.1.3/docs/系统文档/scsi/arcmsr_spec.md"

T = {
"## ARECA FIRMWARE SPEC":
"## ARECA 固件规范（FIRMWARE SPEC）",
"## Usage of IOP331 adapter":
"## IOP331 适配器的使用",
"(All In/Out is in IOP331's view)":
"（所有输入/输出均从 IOP331 的视角出发）",
"### 1. Message 0":
"### 1. Message 0",
"- InitThread message and return code":
"- InitThread 消息与返回码",
"### 2. Doorbell is used for RS-232 emulation":
"### 2. Doorbell 用于 RS-232 仿真",
"data in ready":
"数据输入就绪",
"data out has been read":
"数据输出已被读取",
"data out ready":
"数据输出就绪",
"data in has been read":
"数据输入已被读取",
"### 3. Index Memory Usage":
"### 3. 索引内存使用",
"### 4. RS-232 emulation":
"### 4. RS-232 仿真",
"Currently 128 byte buffer is used:":
"当前使用 128 字节缓冲区：",
"### 5. PostQ":
"### 5. PostQ",
"All SCSI Command must be sent through postQ:":
"所有 SCSI 命令都必须通过 postQ 发送：",
"    (inbound queue port)":
"    （入站队列端口）",
"	Request frame must be 32 bytes aligned:":
"	请求帧必须 32 字节对齐：",
"		flag for post ccb":
"		用于 post ccb 的标志",
"		real address (bit27--bit31) of post arcmsr_cdb":
"		post arcmsr_cdb 的真实地址（bit27--bit31）",
"    (outbount queue port)":
"    （出站队列端口）",
"	Request reply:":
"	请求回复：",
"		    flag for reply":
"		回复标志",
"		    real address (bit27--bit31) of reply arcmsr_cdb":
"		reply arcmsr_cdb 的真实地址（bit27--bit31）",
"			    0   no error, ignore AdapStatus/DevStatus/SenseData":
"			    0   无错误，忽略 AdapStatus/DevStatus/SenseData",
"			    1   Error, error code in AdapStatus/DevStatus/SenseData":
"			    1   错误，错误码位于 AdapStatus/DevStatus/SenseData",
"### 6. BIOS request":
"### 6. BIOS 请求",
"All BIOS request is the same with request from PostQ":
"所有 BIOS 请求与来自 PostQ 的请求相同",
"Except:":
"例外：",
"Request frame is sent from configuration space:":
"请求帧从配置空间发送：",
"	offset: 0x18   writeonly to generate":
"	offset: 0x18   只写以生成",
"		       IRQ to IOP331":
"		       向 IOP331 的 IRQ",
"### 7. Definition of SGL entry (structure)":
"### 7. SGL 条目（结构体）的定义",
"### 8. Message1 Out - Diag Status Code (????)":
"### 8. Message1 输出 - 诊断状态码（????）",
"### 9. Message0 message code":
"### 9. Message0 消息码",
"0x01    Get Config":
"0x01    获取配置（Get Config）",
"	->offset 0xa00 :for outbound message code message_rwbuffer":
"	->offset 0xa00 :用于出站的消息码 message_rwbuffer",
"	(IOP331 send to driver)":
"	（IOP331 发送给驱动）",
"         - Added for checking of":
"         - 新增用于检查",
"				new firmware capability":
"			新的固件能力",
"0x02    Set Config":
"0x02    设置配置（Set Config）",
"	->offset 0xa00 :for inbound message code message_rwbuffer":
"	->offset 0xa00 :用于入站的消息码 message_rwbuffer",
"	(driver send to IOP331)":
"	（驱动发送给 IOP331）",
"	UPPER32 of Request Frame  (4)-->Driver Only":
"	请求帧的 UPPER32（4）-->仅驱动",
"0x03    Reset (Abort all queued Command)":
"0x03    重置（中止所有已排队的命令）",
"0x04    Stop Background Activity":
"0x04    停止后台活动",
"0x05    Flush Cache":
"0x05    刷新缓存",
"0x06    Start Background Activity":
"0x06    启动后台活动",
"	(re-start if background is halted)":
"	（如果后台已停止则重新启动）",
"0x07    Check If Host Command Pending":
"0x07    检查是否有主机命令挂起",
"	(Novell May Need This Function)":
"	（Novell 可能需要此功能）",
"0x08    Set controller time":
"0x08    设置控制器时间",
"	->offset 0xa00   for inbound message code message_rwbuffer":
"	->offset 0xa00   用于入站的消息码 message_rwbuffer",
"	(driver to IOP331)":
"	（驱动到 IOP331）",
"	byte 0   0xaa <-- signature":
"	byte 0   0xaa <-- 签名",
"	byte 1   0x55 <-- signature":
"	byte 1   0x55 <-- 签名",
"	byte 2   year (04)":
"	byte 2   年（04）",
"	byte 3   month (1..12)":
"	byte 3   月（1..12）",
"	byte 4   date (1..31)":
"	byte 4   日（1..31）",
"	byte 5   hour (0..23)":
"	byte 5   时（0..23）",
"	byte 6   minute (0..59)":
"	byte 6   分（0..59）",
"	byte 7   second (0..59)":
"	byte 7   秒（0..59）",
"## RS-232 Interface for Areca Raid Controller":
"## 用于 Areca RAID 控制器的 RS-232 接口",
"      The low level command interface is exclusive with VT100 terminal":
"       底层命令接口与 VT100 终端互斥",
"### 1. Sequence of command execution":
"### 1. 命令执行顺序",
"	(A) Header":
"	(A) 头（Header）",
"		3 bytes sequence (0x5E, 0x01, 0x61)":
"		3 字节序列（0x5E, 0x01, 0x61）",
"	(B) Command block":
"	(B) 命令块",
"		variable length of data including length,":
"		包含长度、",
"		command code, data and checksum byte":
"		命令码、数据和校验字节的可变长度数据",
"	(C) Return data":
"	(C) 返回数据",
"		variable length of data":
"		可变长度的数据",
"### 2. Command block":
"### 2. 命令块",
"	(A) 1st byte":
"	(A) 第 1 字节",
"		command block length (low byte)":
"		命令块长度（低字节）",
"	(B) 2nd byte":
"	(B) 第 2 字节",
"		command block length (high byte)":
"		命令块长度（高字节）",
"		.. Note:: command block length shouldn't > 2040 bytes,":
"		.. 注意:: 命令块长度不应超过 2040 字节，",
"			  length excludes these two bytes":
"			  长度不包含这两个字节",
"	(C) 3rd byte":
"	(C) 第 3 字节",
"		command code":
"		命令码",
"	(D) 4th and following bytes":
"	(D) 第 4 及后续字节",
"		variable length data bytes":
"		可变长度数据字节",
"	    depends on command code":
"	    取决于命令码",
"	(E) last byte":
"	(E) 最后 1 字节",
"	    checksum byte (sum of 1st byte until last data byte)":
"	    校验字节（从第 1 字节到最后一个数据字节的和）",
"### 3. Command code and associated data":
"### 3. 命令码及相关数据",
"The following are command code defined in raid controller Command":
"以下是 RAID 控制器中定义的命令码",
"code 0x10--0x1? are used for system level management,":
"命令码 0x10--0x1? 用于系统级管理，",
"no password checking is needed and should be implemented in separate":
"无需密码检查，并且应在独立的",
"well controlled utility and not for end user access.":
"受控工具中实现，不供最终用户访问。",
"Command code 0x20--0x?? always check the password,":
"命令码 0x20--0x?? 始终检查密码，",
"##### Command description":
"##### 命令描述",
"	Set the controller serial#":
"	设置控制器序列号",
"	Set vendor string for the controller":
"	设置控制器的厂商字符串",
"	Set the model name of the controller":
"	设置控制器的型号名称",
"	Identify device":
"	识别设备",
"	Verify password":
"	验证密码",
"	Logout GUI (force password checking on next command)":
"	注销 GUI（在下一个命令时强制进行密码检查）",
"	HTTP interface (reserved for Http proxy service)(0x16)":
"	HTTP 接口（保留用于 HTTP 代理服务）（0x16）",
"	Set the ethernet MAC address":
"	设置以太网 MAC 地址",
"	Set logo in HTTP":
"	在 HTTP 中设置徽标",
"	Poll If Event Log Changed":
"	轮询事件日志是否更改",
"	Read Event":
"	读取事件",
"	Get HW monitor data":
"	获取硬件监视器数据",
"	Quick create raid/volume set":
"	快速创建 RAID/卷集",
"	Get Raid Set Information":
"	获取 RAID 集信息",
"	Get Volume Set Information":
"	获取卷集信息",
"	Get Physical Drive Information":
"	获取物理驱动器信息",
"	Get System Information":
"	获取系统信息",
"	Clear System Event":
"	清除系统事件",
"	Mute current beeper":
"	静音当前蜂鸣器",
"	Disable beeper":
"	禁用蜂鸣器",
"	Change password":
"	更改密码",
"	Set host interface mode":
"	设置主机接口模式",
"	Set rebuild priority":
"	设置重建优先级",
"	Set maximum ATA mode to be used":
"	设置要使用的最大 ATA 模式",
"	Reset Controller":
"	重置控制器",
"	COM port setting":
"	COM 端口设置",
"	No operation":
"	无操作",
"	Set DHCP option and local IP address":
"	设置 DHCP 选项和本地 IP 地址",
"	Create pass through disk":
"	创建直通磁盘",
"	Modify pass through disk":
"	修改直通磁盘",
"	Delete pass through disk":
"	删除直通磁盘",
"	Identify Device":
"	识别设备",
"	Create Raid Set":
"	创建 RAID 集",
"	Delete Raid Set":
"	删除 RAID 集",
"	Expand Raid Set":
"	扩展 RAID 集",
"	Activate incomplete raid set":
"	激活不完整的 RAID 集",
"	Create hot spare disk":
"	创建热备盘",
"	Delete hot spare disk":
"	删除热备盘",
"	Create volume set":
"	创建卷集",
"	Modify volume Set":
"	修改卷集",
"	Delete volume set":
"	删除卷集",
"	Start volume consistency check":
"	启动卷一致性检查",
"	Stop volume consistency check":
"	停止卷一致性检查",
"    This function is removed, application like":
"    此功能已移除，应用程序若",
"    to implement quick create function":
"    要实现快速创建功能",
"    need to use GUI_CREATE_RAIDSET and GUI_CREATE_VOLUMESET function.":
"    需要使用 GUI_CREATE_RAIDSET 和 GUI_CREATE_VOLUMESET 功能。",
"### 4. Returned data":
"### 4. 返回的数据",
"	(A) Header":
"	(A) 头",
"    3 bytes sequence (0x5E, 0x01, 0x61)":
"    3 字节序列（0x5E, 0x01, 0x61）",
"(B) Length":
"(B) 长度",
"    2 bytes":
"    2 字节",
"    (low byte 1st, excludes length and checksum byte)":
"    （低字节在前，不包含长度和校验字节）",
"(C)":
"(C)",
"    status or data:":
"    状态或数据：",
"	2) If length > 1:":
"	2) 如果长度 > 1：",
"		data block returned from controller":
"		从控制器返回的数据块",
"		and the contents depends on the command code":
"		其内容取决于命令码",
"(E) Checksum":
"(E) 校验和",
"    checksum of length and status or data byte":
"    长度和状态或数据字节的校验和",
"     - Response with VT100 screen (discard it)":
"     - 以 VT100 屏幕响应（丢弃它）",
"			  .. Note:: page0 1st 2 byte must be":
"			  .. 注意:: page0 的前 2 字节必须为",
"				    actual length of the JPG file":
"				    JPG 文件的实际长度",
"			  .. Note:: no response data available":
"			  .. 注意:: 无可用响应数据",
}

src = open(F, encoding='utf-8').read()
out_lines = []
for line in src.split('\n'):
    key = line.rstrip()
    out_lines.append(T.get(key, line))
new = '\n'.join(out_lines)

n = new.count('```')
assert n % 2 == 0, "fence odd: %d" % n

tmp = F + '.tmp'
open(tmp, 'w', encoding='utf-8').write(new)
os.replace(tmp, F)
print("arcmsr done; fences:", n)
