
## Chelsio S3 iSCSI 驱动（Linux 版）


## 简

基于 Chelsio T3 ASIC 的适配器（S310、S320、S302、S304、Mezz 卡等产品系列）支iSCSI 加速以iSCSI 直接数据放置（DDP），由硬件处理开销较大的逐字节操作，例如
CRC 计算与校验，以及直接向最终主机内存目标地址发起 DMA
 - iSCSI PDU 摘要生成与校
	  发送时，Chelsio S3 硬件计算并将头部与数据摘要插入到 PDU 中	  接收时，Chelsio S3 硬件计算并校PDU 的头部与数据摘要
 - 直接数据放置（DDP
	  S3 硬件可以根据 Data-In PDU 中的发起者任务标签（ITT）或 Data-Out
	  PDU 中的目标任务标签（TTT），iSCSI Data-In Data-Out PDU 	  有效载荷直接放置到预先提交的、最终目的地的主机内存缓冲区中
 - PDU 发送与恢复

	  发送时，S3 硬件从主机驱动接收完整的 PDU（头+ 数据），计算并插	  摘要，在必要时将 PDU 分解为多TCP 段，并将所TCP 段发送到网络上	  如需重传，它会处TCP 重传
	  接收时，S3 硬件通过重组 TCP 段来恢复 iSCSI PDU，分离头部与数据	  计算并校验摘要，然后将头部转发给主机。有效载荷数据如有可能将直接
	  放置到预先提交的主机 DDP 缓冲区中，否则有效载荷数据也会发送给主机
cxgb3i 驱动open-iscsi 发起者对接，并在适用处通过 Chelsio 硬件提供 iSCSI 加速
## 使用 cxgb3i 驱动


要使 open-iscsi 发起者获得加速，需要执行以下步骤：

1. 加载 cxgb3i 驱动modprobe cxgb3i"

   cxgb3i 模块会向 open-iscsi 注册一个新的传输类 "cxgb3i"
```

	Device Drivers
		SCSI device support --->
			[*] SCSI low-level drivers  --->
				<M>   Chelsio S3xx iSCSI support

```
2. /etc/iscsi/ifaces/ 下为新的传输"cxgb3i" 创建一个接口文件
```

	iface.transport_name = cxgb3i
	iface.net_ifacename = <ethX>
	iface.ipaddress = <iscsi ip address>

   * 若指定了 iface.ipaddress，则 <iscsi ip address> 必须ethX IP 地址相同     或位于同一子网内。请确保IP 地址在网络中唯一
```
3. 编辑 /etc/iscsi/iscsid.conf
   默认设置 MaxRecvDataSegmentLength31072）过大；
```

	node.conn[0].iscsi.MaxRecvDataSegmentLength = 8192

   * MaxRecvDataSegmentLength 过大，普通会话的登录会失败。系统会dmesg      记录格式如下的错误消息：
     "cxgb3i: ERR! MaxRecvSegmentLength <X> too big. Need to be <= <Y>."

```
4. 要使 open-iscsi 流量经由 cxgb3i 的加速路径，大多iscsiadm 命令都需要指   "-I <iface file name>" 选项iface file name> 为第 2 步中创建的传输接口文件
