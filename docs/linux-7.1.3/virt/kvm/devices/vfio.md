
## VFIO 虚拟设备


支持的设备类型：

  - KVM_DEV_TYPE_VFIO

每个 VM 只能创建一个 VFIO 实例。所创建的设备跟踪 VM 正在使用的 VFIO 文件（group 或 device），以及那些对 VM 的正确性和加速至关重要的 group/device 特性。随着 group/device 被 VM 启用或禁用，应当就它们的存在更新 KVM。当向 KVM 注册时，KVM 会持有一个对 VFIO 文件的引用。

组：
  KVM_DEV_VFIO_FILE
	alias: KVM_DEV_VFIO_GROUP

KVM_DEV_VFIO_FILE 属性：
  KVM_DEV_VFIO_FILE_ADD：向 VFIO-KVM 设备跟踪中添加一个 VFIO 文件（group/device）

	kvm_device_attr.addr 指向 VFIO 文件的 int32_t 文件描述符。

  KVM_DEV_VFIO_FILE_DEL：从 VFIO-KVM 设备跟踪中移除一个 VFIO 文件（group/device）

	kvm_device_attr.addr 指向 VFIO 文件的 int32_t 文件描述符。

KVM_DEV_VFIO_GROUP（仅限于处理 VFIO group fd 的传统 kvm 设备组）：
  KVM_DEV_VFIO_GROUP_ADD：与 KVM_DEV_VFIO_FILE_ADD 相同，但仅针对 group fd

  KVM_DEV_VFIO_GROUP_DEL：与 KVM_DEV_VFIO_FILE_DEL 相同，但仅针对 group fd

  KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE：附加一个客户机可见的 TCE 表，
	由 sPAPR KVM 分配。
```

		struct kvm_vfio_spapr_tce {
			__s32	groupfd;
			__s32	tablefd;
		};

	其中：

	- @groupfd 是 VFIO group 的文件描述符；
	- @tablefd 是通过 KVM_CREATE_SPAPR_TCE 分配的 TCE 表的文件描述符。

```
上面的 FILE/GROUP_ADD 操作应当在通过 VFIO_GROUP_GET_DEVICE_FD 访问设备文件描述符之前调用，以支持那些需要在其 .open_device() 回调中设置 kvm 指针的驱动。对于通过字符设备 open 获得设备文件描述符（并通过 VFIO_DEVICE_BIND_IOMMUFD 获得设备访问）的情况也同样如此。对于此类文件描述符，应在 VFIO_DEVICE_BIND_IOMMUFD 之前调用 FILE_ADD，以支持前面提到的那些驱动。
