
## VFIO AP 锁概述（VFIO AP Locks Overview
本文档描述了vfio_ap 设备驱动安全运行相关的锁。通篇将使用以下变量来表示此处所描述结构的实例：


  struct ap_matrix_dev *matrix_dev;
  struct ap_matrix_mdev *matrix_mdev;
  struct kvm *kvm;

### Matrix Devices 锁（drivers/s390/crypto/vfio_ap_private.h


  struct ap_matrix_dev {
  	...
  	struct list_head mdev_list;
  	struct mutex mdevs_lock;
  	...
  }

Matrix Devices 锁（matrix_dev->mdevs_lock）实现为包含于单struct ap_matrix_dev 对象内的全局互斥量（mutex）。该锁控制对 matrix_dev->mdev_list 中每matrix_mdev 内所包含的所有字段的访问。在读取、写入或使用某个 matrix_mdev 实例（表vfio_ap 设备驱动的某mediated 设备）中某字段的数据时，必须持有该锁
### KVM 锁（include/linux/kvm_host.h


  struct kvm {
  	...
  	struct mutex lock;
  	...
  }

KVM 锁（kvm->lock）控制对 KVM 客户机（guest）状态数据的访问。当将一个或多个 AP 适配器domain 或控制域（control domain）热插拔（plug/unplug）进或移出客户机时，vfio_ap 设备驱动必须持有该锁
KVM 指针存储matrix_mdev 实例中（matrix_mdev->kvm = kvm），该实例包含已附加KVM 客户机的 mediated 设备的状态
### Guests 锁（drivers/s390/crypto/vfio_ap_private.h


  struct ap_matrix_dev {
  	...
  	struct list_head mdev_list;
  	struct mutex guests_lock;
  	...
  }

Guests 锁（matrix_dev->guests_lock）控制对 matrix_mdev 实例（matrix_dev->mdev_list）的访问，这些实例表示持有已附加KVM 客户机的 mediated 设备状态的 mediated 设备。必须持有该锁：

1. vfio_ap 设备驱动使用 KVM 指针来热插拔/移出直通（passed through）到 KVM 客户机的 AP 设备时，控制KVM 指针（matrix_mdev->kvm）的访问
2. matrix_dev->mdev_list 添加或从中移matrix_mdev 实例。这在遍历该列表以查找用于热插拔/移出直通到 KVM 客户机的 AP 设备ap_matrix_mdev 实例时，对确保正确的加锁顺序是必要的
   例如，当vfio_ap 设备驱动移除一个队列设备时，如果该适配器（adapter）被直通到 KVM 客户机，则必须将其移出。为了确定该适配器是否被直通，必须找到该队列所分配matrix_mdev 对象。随后可KVM 指针（matrix_mdev->kvm）来判断mediated 设备是否被直通（matrix_mdev->kvm != NULL），若是，则移出该适配器
如果 KVM 指针未被用于热插移出直通到 KVM 客户机的设备，则不必获取 Guests 锁；但在这种情况下，由于 KVM 指针是在 Matrix Devices 锁的保护下被设置和清除的，必须持Matrix Devices 锁（matrix_dev->mdevs_lock）才能访KVM 指针。一个恰当的例子是处PQAP(AQIC) 指令子函数拦截（interception）的函数。该处理函数只需要访KVM 指针来设置或清除 IRQ 资源，因此只需持有 matrix_dev->mdevs_lock
### PQAP Hook 锁（arch/s390/include/asm/kvm_host.h


  typedef int (**crypto_hook)(struct kvm_vcpu **vcpu);

  struct kvm_s390_crypto {
  	...
  	struct rw_semaphore pqap_hook_rwsem;
  	crypto_hook *pqap_hook;
  	...
  };

PQAP Hook 锁是一个读写信号量（r/w semaphore），控制对处理函数指`(*kvm->arch.crypto.pqap_hook)` 的访问，该指针在 PQAP(AQIC) 指令子函数被宿主机拦截时会被调用。当设置 pqap_hook 值时必须以写模式持有该锁，在调用 pqap_hook 函数时以读模式持有