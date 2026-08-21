## NTB 驱动

NTB（Non-Transparent Bridge，非透明桥）是一PCI-Express 桥接芯片，它将两台或多台计算各自独立的存储系统连接到同一 PCI-Express 结构（fabric）上。现有的 NTB 硬件支持一组通用
特性：门铃（doorbell）寄存器和内存转换窗口，以及诸如 scratchpad message 寄存器等非通用
特性。Scratchpad 寄存器是可读写的寄存器，可从设备的两侧访问，以便对端（peer）可以在一个固地址交换少量信息。Message 寄存器可用于同样的目的。此外，它们还带有特殊的状态位，以确保信息
不会被另一个对端重写。Doorbell 寄存器为对端提供了一种发送中断事件的方式。内存窗口允许对对端
内存进行转换后的读写访问
## NTB 核心驱动（ntb
NTB 核心驱动定义了一个封装了通用特性集API，并允许NTB 特性感兴趣的客户端（client）发由硬件驱动支持的 NTB 设备。这里“client”一词指的是使用 NTB API 的上层组件。这里“driver”或
“hardware driver”指的是针对特定厂商和型号的 NTB 硬件的驱动
## NTB 客户端驱
NTB 客户端驱动应NTB 核心驱动注册。注册之后，随着 NTB 硬件或硬件驱动被插入和移除，客户端的
probe remove 函数会被相应地调用。注册使Linux 设备框架，因此对于任何写PCI 驱动的人来说
都应该很熟悉
### NTB 典型客户端驱动实
NTB 的主要目的是在至少两个系统之间共享某块内存。因NTB 设备的特性，Scratchpad/Message
寄存器，主要用于执行正确的内存窗口初始化。NTB API 通常支持两类内存窗口接口：配置在本地 ntb
端口上的入站（inbound）转换，以及由对端在ntb 端口上配置的出站（outbound）转换。第一类是
```
 Inbound translation:

 Memory:              Local NTB Port:      Peer NTB Port:      Peer MMIO:
  ____________
 | dma-mapped |-ntb_mw_set_trans(addr)  |
 | memory     |        _v____________   |   ______________
 | (addr)     |<======| MW xlat addr |<====| MW base addr |<== memory-mapped IO
 |------------|       |--------------|  |  |--------------|

```
因此第一类内存窗口初始化的典型场景为) 分配一个内存区域；2) 将转换后的地址放入 NTB 配置3) 以某种方式通知对端设备已完成初始化) 对端设备映射相应的出站内存窗口，从而能够访问该共享
内存区域
第二类接口意味着共享窗口由对端配置：

```
 Outbound translation:

 Memory:        Local NTB Port:    Peer NTB Port:      Peer MMIO:
  ____________                      ______________
 | dma-mapped |                |   | MW base addr |<== memory-mapped IO
 | memory     |                |   |--------------|
 | (addr)     |<===================| MW xlat addr |<-ntb_peer_mw_set_trans(addr)
 |------------|                |   |--------------|

```
第二类接口初始化的典型场景为) 分配一个内存区域；2) 以某种方式将转换后的地址传递给对端设备3) 对端将转换后的地址放入 NTB 配置) 对端设备映射出站内存窗口，从而能够访问共享内存区域
如所见，所描述的场景可以组合成一个可移植的算法
 本地设备   1) 为共享窗口分配内   2) 用所分配区域的转换地址初始化内存窗      （如果本地内存窗口初始化不受支持，则可能失败   3) 将转换后的地址和内存窗口索引发送给对端设备

 对端设备   1) 用另一个设备分配的内存区域的检索到的地址初始化内存窗      （如果对端内存窗口初始化不受支持，则可能失败   2) 映射出站内存窗口

按照该场景，NTB 内存窗口 API 可按如下方式使用
 本地设备   1) ntb_mw_count(pidx) - 检索可为本地设备与指定索引端口的对端设备之      的内存窗口分配的内存范围数量   2) ntb_get_align(pidx, midx) - 检索限制共享内存区域对齐和大小的参数      然后就可以正确地分配内存了   3) 按照步骤 2 中检索到的限制分配物理连续的内存区域   4) ntb_mw_set_trans(pidx, midx) - 尝试为指定对端设备设置指定索引内存窗口的
      转换地址（如果本地转换地址设置不受支持，则可能失败   5) 使用例如 scratchpad message 寄存器，将转换后的基地址（通常与内存窗      编号一起）发送给对端设备
 对端设备   1) ntb_peer_mw_set_trans(pidx, midx) - 尝试为指定内存窗口设置从其他设备
      （与 pidx 相关）接收到的转换地址。如果检索到的地址例如超出了最大可能地址      未正确对齐，则可能失败   2) ntb_peer_mw_get_addr(widx) - 检索用于映射内存窗口的 MMIO 地址，从而可      访问共享内存
同样值得注意的是，方ntb_mw_count(pidx) 应当返回与对端端口索引为 -pidx ntb_peer_mw_count() 相同的值
### NTB 传输客户端（ntb\_transport）与 NTB 网络设备（ntb\_netdev
NTB 的主要客户端是传输（Transport）客户端，它NTB Netdev 配合使用。这些驱动协同工作，NTB 创建到对端的逻辑链路，从而交换网络数据包。传输客户端建立到对端的逻辑链路，并创建队列
对（queue pair）来交换消息和数据。NTB Netdev 随后使用传输队列对创建一个以太网设备。网络数在套接字缓冲区和传输队列对缓冲区之间拷贝。传输客户端除了 Netdev 之外也可用于其他用途，不过
目前还没有编写其他应用
### NTB 乒乓测试客户端（ntb\_pingpong
乒乓测试客户端用于演NTB 硬件的门铃和 scratchpad 寄存器，并作为一个简单的 NTB 客户端示例乒乓在启动时启用链路，等NTB 链路建立，然后着手读NTB 的门scratchpad 寄存器。对端之使用一个门铃位掩码互相中断，该掩码在每一轮中移位一位，以测试多个门铃位和中断向量的行为。乒驱动还会在每一轮写入对端门铃寄存器之前，读取第一个本scratchpad，并将该值加一后写入第一对端 scratchpad
模块参数
- unsafe - 某些硬件scratchpad 和门铃寄存器方面存在已知问题。默认情况下，乒乓不会尝	演练此类硬件。你可以通过设置 unsafe=1 来以自己的风险覆盖此行为- delay\_ms - 指定从收到门铃中断事件到为下一轮设置对端门铃寄存器之间的延迟- init\_db - 指定用于开始新一轮的门铃位。一旦所有门铃位都已移出范围，新一轮就开始- dyndbg - 建议在加载此模块时指dyndbg=+p，然后在控制台上观察调试输出
### NTB 工具测试客户端（ntb\_tool
工具测试客户端主要用于调NTB 硬件和驱动。该工具通过 debugfs 提供NTB 门铃的读取、设置和
清除，以及对 scratchpad 的读写
该工具目前没有任何模块参数
Debugfs 文件
- **debugfs**/ntb\_tool/**hw**/
	工具会为每个被探测到NTB 设备debugfs 中创建一个目录。该目录在下文中
	简写为 **hw**- **hw**/db
	该文件用于读取、设置和清除本地门铃。并非所有硬件都支持所有操作。要读取门铃	读取该文件。要设置门铃，写`s` 后跟要设置的位（例如：`echo 's 0x0101' > db`）	要清除门铃，写入 `c` 后跟要清除的位- **hw**/mask
	该文件用于读取、设置和清除本地门铃掩码。详**db**- **hw**/peer\_db
	该文件用于读取、设置和清除对端门铃。详**db**- **hw**/peer\_mask
	该文件用于读取、设置和清除对端门铃掩码。详**db**- **hw**/spad
	该文件用于读写本scratchpad。要读取所scratchpad 的值，读取该文件。要
	写入值，则写入一系列 scratchpad 编号与值的配对（例如：`echo '4 0x123 7 0xabc' > spad`
	# scratchpad `4` `7` 分别设为 `0x123` `0xabc`）- **hw**/peer\_spad
	该文件用于读写对scratchpad。详**spad**
### NTB MSI 测试客户端（ntb\_msi\_test
MSI 测试客户端用于测试和调试 MSI 库，该库允许NTB 内存窗口传MSI 中断。测试客户端通过
debugfs 文件系统与之交互
- **debugfs**/ntb\_msi\_test/**hw**/
	msi 测试会为每个被探测到NTB 设备debugfs 中创建一个目录。该目录在下文中
	简写为 **hw**- **hw**/port
	该文件描述本地端口编- **hw**/irq*\_occurrences
	每个中断都有一个对应的 occurrences 文件，读取时返回该中断被触发的次数- **hw**/peer*/port
	该文件描述每个对端的端口编号
- **hw**/peer*/count
	该文件描述可以在每个对端上触发的中断数量
- **hw**/peer*/trigger
	写入一个中断编号（小于 count 中指定的任何值）将在指定对端上触发该中断。该对端	中断对应occurrence 文件应当递增
## NTB 硬件驱动

NTB 硬件驱动应向 NTB 核心驱动注册设备。注册之后，客户端的 probe remove 函数会被调用
### NTB Intel 硬件驱动（ntb\_hw\_intel
Intel 硬件驱动支持 Xeon Atom CPU 上的 NTB
模块参数
- b2b\_mw\_idx
	如果要通过内存窗口访问对端 ntb，则使用此内存窗口来访问对端 ntb。零或正值从
	第一mw idx 开始，负值从最后一mw idx 开始。两侧必须在此设置相同的值！默认
	值为 `-1`- b2b\_mw\_share
	如果要通过内存窗口访问对端 ntb，且内存窗口足够大，仍然允许客户端使用内存窗口的
	后半部分用于对端地址转换- xeon\_b2b\_usd\_bar2\_addr64
	如果Xeon 硬件上使B2B 拓扑，则在链路上游侧 NTB 设备之间的总线上，对位	BAR2 的窗口使用此 64 位地址- xeon\_b2b\_usd\_bar4\_addr64 - 参见 **xeon\_b2b\_bar2\_addr64**- xeon\_b2b\_usd\_bar4\_addr32 - 参见 **xeon\_b2b\_bar2\_addr64**- xeon\_b2b\_usd\_bar5\_addr32 - 参见 **xeon\_b2b\_bar2\_addr64**- xeon\_b2b\_dsd\_bar2\_addr64 - 参见 **xeon\_b2b\_bar2\_addr64**- xeon\_b2b\_dsd\_bar4\_addr64 - 参见 **xeon\_b2b\_bar2\_addr64**- xeon\_b2b\_dsd\_bar4\_addr32 - 参见 **xeon\_b2b\_bar2\_addr64**- xeon\_b2b\_dsd\_bar5\_addr32 - 参见 **xeon\_b2b\_bar2\_addr64**