## SMARTPQI - Microchip Smart Storage SCSI 驱动


本文件描述了 Microchip (http://www.microchip.com) PQI 控制器的 smartpqi SCSI 驱动。smartpqi 驱动
是 Microchip Corp. 的新一代 SCSI 驱动，也是首个实现 PQI 队列模型的 SCSI 驱动。

smartpqi 驱动将取代 Adaptec Series 9 控制器的 aacraid 驱动。使用 Adaptec Series 9 控制器且运行较旧
内核（4.9 之前）的客户必须配置 smartpqi 驱动，否则其卷将不会被添加到操作系统。

要获得 Microchip smartpqi 控制器的支持，请在配置内核时启用 smartpqi 驱动。

有关 PQI 队列接口的更多信息，请参见：

- http://www.t10.org/drafts.htm
- http://www.t10.org/members/w_pqi2.htm

## 支持的设备

<Controller names to be added as they become publicly available.>

## /sys 中 smartpqi 专用的条目


### smartpqi 主机属性

  - /sys/class/scsi_host/host*/rescan
  - /sys/class/scsi_host/host*/driver_version

  host rescan 属性是一个只写属性。向该属性写入将触发驱动扫描新增、更改或移除的
  设备，并通知 SCSI 中间层所检测到的任何变化。

  version 属性是只读的，将返回驱动版本与控制器固件版本。
```

              driver: 0.9.13-370
              firmware: 0.01-522

```
### smartpqi sas 设备属性

  HBA 设备会被添加到 SAS 传输层。这些属性由 SAS 传输层自动添加。

  /sys/class/sas_device/end_device-X:X/sas_address
  /sys/class/sas_device/end_device-X:X/enclosure_identifier
  /sys/class/sas_device/end_device-X:X/scsi_target_id

## smartpqi 专用的 ioctls


  为了与为 cciss 协议编写的应用程序保持兼容。

  CCISS_DEREGDISK, CCISS_REGNEWDISK, CCISS_REGNEWD
	上述三个 ioctl 都执行完全相同的操作，即让驱动重新扫描新设备。这与写入
	smartpqi 专用的主机 “rescan” 属性作用完全相同。

  CCISS_GETPCIINFO
	返回 PCI 域、总线、设备和功能以及 “board ID”（PCI 子系统 ID）。

  CCISS_GETDRIVVER
```

	  (DRIVER_MAJOR << 28) | (DRIVER_MINOR << 24) | (DRIVER_RELEASE << 16) | DRIVER_REVISION;

  CCISS_PASSTHRU
	允许将 “BMIC” 和 “CISS” 命令透传到 Smart Storage Array。
	这些命令被 SSA Array Configuration Utility、SNMP 存储代理等广泛使用。

```
