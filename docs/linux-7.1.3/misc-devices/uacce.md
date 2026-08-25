
## Uacce (Unified/用户-space-访问-intended Accelerator Framework)


### 简


Uacce (Unified/用户-space-访问-intended Accelerator Framework) targets
提供 Shared 虚拟 Addressing (SVA) accelerators 进程.
accelerator 访问 数据 结构main 处理
differs 数据 sharing 处理io 设备, share
数据 content rather 地址.
unified 地址, 硬件 用户空间 进程
share same 虚拟地址 communication.
Uacce takes 硬件 accelerator heterogeneous processor,
IOMMU share same CPU 椤，琛?result same translation
va pa.

```

         __________________________       __________________________
        |                          |     |                          |
        |  User application (CPU)  |     |   Hardware Accelerator   |
        |__________________________|     |__________________________|

                     |                                 |
                     | va                              | va
                     V                                 V
                 __________                        __________
                |          |                      |          |
                |   MMU    |                      |  IOMMU   |
                |__________|                      |__________|
                     |                                 |
                     |                                 |
                     V pa                              V pa
                 _______________________________________
                |                                       |
                |              Memory                   |
                |_______________________________________|



```
### 架构


Uacce 内核 模块, taking charge iommu 地址 sharing.
用户 驱动 libraries called WarpDrive.

uacce 设备, built around IOMMU SVA API, 访问 multiple
地址 spaces, including one PASID.

虚拟 concept, 队列, 使用 communication. 提供
FIFO-like 接口. maintains unified 地址 space
application involved 硬件.

```

                             ___________________                  ________________
                            |                   |   user API     |                |
                            | WarpDrive library | ------------>  |  user driver   |
                            |___________________|                |________________|
                                     |                                    |
                                     |                                    |
                                     | queue fd                           |
                                     |                                    |
                                     |                                    |
                                     v                                    |
     ___________________         _________                                |
    |                   |       |         |                               | mmap memory
    | Other framework   |       |  uacce  |                               | r/w interface
    | crypto/nic/others |       |_________|                               |
    |___________________|                                                 |
             |                       |                                    |
             | register              | register                           |
             |                       |                                    |
             |                       |                                    |
             |                _________________       __________          |
             |               |                 |     |          |         |
              -------------  |  Device Driver  |     |  IOMMU   |         |
                             |_________________|     |__________|         |
                                     |                                    |
                                     |                                    V
                                     |                            ___________________
                                     |                           |                   |
                                     --------------------------  |  Device(Hardware) |
                                                                 |___________________|


```
### work


Uacce uses mmap IOMMU play trick.

Uacce 创建 chrdev every 设备 registered . New 队列
created 用户 application 打开 chrdev. 文件 描述使用
用户 处理 队列.
accelerator 设备 present itself Uacce object, exports
chrdev 用户空间. 用户 application communicates
硬件 ioctl ( 控制 path) share 内存 ( 数据 path).

控制 path 硬件 文件 操作, 数据 path
mmap space 队列 fd.

队列 文件 地址 space:

```

   /**
   * enum uacce_qfrt: qfrt type
   * @UACCE_QFRT_MMIO: device mmio region
   * @UACCE_QFRT_DUS: device user share region
   */
  enum uacce_qfrt {
          UACCE_QFRT_MMIO = 0,
          UACCE_QFRT_DUS = 1,
  };

```
regions 可differ 设备 类型 类型.
region mmapped once, otherwise -EEXIST 返回.

设备 mmio region mapped 硬件 mmio space. generally
使用 doorbell notification 硬件. fast enough
数据 channel.

设备 用户 share region 使用 share 数据 缓冲用户 进程
设备.


### Uacce 瀵勫瓨鍣?API


瀵勫瓨鍣?API defined uacce.h.

```

  struct uacce_interface {
    char name[UACCE_MAX_NAME_SIZE];
    unsigned int flags;
    const struct uacce_ops *ops;
  };

```
According IOMMU capability, uacce_interface 标志 :

```

  /**
   * UACCE Device flags:
   * UACCE_DEV_SVA: Shared Virtual Addresses
   *              Support PASID
   *              Support device page faults (PCI PRI or SMMU Stall)
   */
  #define UACCE_DEV_SVA               BIT(0)

  struct uacce_device *uacce_alloc(struct device *parent,
                                   struct uacce_interface *interface);
  int uacce_register(struct uacce_device *uacce);
  void uacce_remove(struct uacce_device *uacce);

```
uacce_register results :

. uacce 模块 compiled, ERR_PTR(-ENODEV)

b. Succeed desired 标志

c. Succeed negotiated 标志, 示例

uacce_interface.标志 = UACCE_DEV_SVA uacce->标志 = ~UACCE_DEV_SVA

用户 驱动 need 检返回 well negotiated uacce->标志.


### 用户 驱动


队列 文件 mmap space need 用户 驱动 wrap communication
协议. Uacce 提供 attributes sysfs 用户 驱动
match right accelerator accordingly.
More details 文档/ABI/testing/sysfs-驱动-uacce.
