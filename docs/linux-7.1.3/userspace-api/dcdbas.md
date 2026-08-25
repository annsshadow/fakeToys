## Dell 系统管理基础驱动


## 概述


Dell 系统管理基础驱动提供了一sysfs 接口，供 Dell OpenManage 等系统管理软件在特定Dell 系统上执行系统管理中断（SMI）与主机控制动作（在 OS 关机后进行系统电源循环或断电）
Dell OpenManage 在以Dell PowerEdge 系统上需要此驱动0030040000SC00SC1500SC55000SC600SC50655MC00 750。其Dell 软件（如开源的 libsmbios
项目）预期会利用此驱动，其中可能包括在其Dell 系统上使用此驱动
Dell libsmbios 项目致力于尽可能多地提供BIOS 信息的访问。关libsmbios 项目的更信息，请参见 http://linux.dell.com/libsmbios/main/

## 系统管理中断


在某Dell 系统上，系统管理软件必须通过系统管理中断（SMI）访问某些管理信息。SMI 数据
缓冲区必须位32 位地址空间中，SMI 需要该缓冲区的物理地址。驱动维SMI 所需内存，并为应用程序提供生SMI 的方式驱动为系统管理软件创建以sysfs 条目```

	/sys/devices/platform/dcdbas/smi_data
	/sys/devices/platform/dcdbas/smi_data_buf_phys_addr
	/sys/devices/platform/dcdbas/smi_data_buf_size
	/sys/devices/platform/dcdbas/smi_request

```
系统管理软件必须执行以下步骤以使用该驱动执行一SMI
1) 锁定 smi_data2) 将系统管理命令写smi_data3) smi_request 写入 "1" 以生成调用接SMI，或写入 "2" 以生成原SMI4) smi_data 读取系统管理命令的响应5) 解锁 smi_data

## 主机控制动作


Dell OpenManage 支持一种主机控制特性，允许管理员在 OS 完成关机后对系统执行电源循环断电。在某些 Dell 系统上，该主机控制特性要求驱动在 OS 完成关机后执行一SMI
驱动为系统管理软件创建以sysfs 条目，以安排驱动在系统完成关机后执行电源循环或断主机控制动作
/sys/devices/platform/dcdbas/host_control_action
/sys/devices/platform/dcdbas/host_control_smi_type
/sys/devices/platform/dcdbas/host_control_on_shutdown

Dell OpenManage 使用此驱动执行电源循环或断电主机控制动作的步骤如下：

1) 将待执行的主机控制动作写host_control_action2) 将驱动需要执行的 SMI 类型写入 host_control_smi_type3) host_control_on_shutdown 写入 "1" 以启用主机控制动作4) 发起 OS 关机   （当驱动收到 OS 已完成关机的通知时，会执行主机控SMI。）


## 主机控制 SMI 类型


下表显示了为执行电源循环或断电主机控制动作需要写host_control_smi_type 的值：

=================== =====================
PowerEdge 系统      Host Control SMI 类型
=================== =====================
      300             HC_SMITYPE_TYPE1
     1300             HC_SMITYPE_TYPE1
     1400             HC_SMITYPE_TYPE2
      500SC           HC_SMITYPE_TYPE2
     1500SC           HC_SMITYPE_TYPE2
     1550             HC_SMITYPE_TYPE2
      600SC           HC_SMITYPE_TYPE2
     1600SC           HC_SMITYPE_TYPE2
      650             HC_SMITYPE_TYPE2
     1655MC           HC_SMITYPE_TYPE2
      700             HC_SMITYPE_TYPE3
      750             HC_SMITYPE_TYPE3
=================== =====================
