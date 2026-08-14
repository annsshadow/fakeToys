
## Uacce (Unified/鐢ㄦ埛-space-璁块棶-intended Accelerator Framework)


### 绠€浠?


Uacce (Unified/鐢ㄦ埛-space-璁块棶-intended Accelerator Framework) targets
鎻愪緵 Shared 铏氭嫙 Addressing (SVA) accelerators 杩涚▼.
accelerator 璁块棶 鏁版嵁 缁撴瀯浣?main 澶勭悊鍣?
differs 鏁版嵁 sharing 澶勭悊鍣?io 璁惧, share
鏁版嵁 content rather 鍦板潃.
unified 鍦板潃, 纭欢 鐢ㄦ埛绌洪棿 杩涚▼
share same 铏氭嫙鍦板潃 communication.
Uacce takes 纭欢 accelerator heterogeneous processor,
IOMMU share same CPU 椤?琛?result same translation
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
### 鏋舵瀯


Uacce 鍐呮牳 妯″潡, taking charge iommu 鍦板潃 sharing.
鐢ㄦ埛 椹卞姩 libraries called WarpDrive.

uacce 璁惧, built around IOMMU SVA API, 璁块棶 multiple
鍦板潃 spaces, including one PASID.

铏氭嫙 concept, 闃熷垪, 浣跨敤 communication. 鎻愪緵
FIFO-like 鎺ュ彛. maintains unified 鍦板潃 space
application involved 纭欢.

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

Uacce 鍒涘缓 chrdev every 璁惧 registered . New 闃熷垪
created 鐢ㄦ埛 application 鎵撳紑 chrdev. 鏂囦欢 鎻忚堪绗?浣跨敤
鐢ㄦ埛 澶勭悊 闃熷垪.
accelerator 璁惧 present itself Uacce object, exports
chrdev 鐢ㄦ埛绌洪棿. 鐢ㄦ埛 application communicates
纭欢 ioctl ( 鎺у埗 path) share 鍐呭瓨 ( 鏁版嵁 path).

鎺у埗 path 纭欢 鏂囦欢 鎿嶄綔, 鏁版嵁 path
mmap space 闃熷垪 fd.

闃熷垪 鏂囦欢 鍦板潃 space:

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
regions 鍙€?differ 璁惧 绫诲瀷 绫诲瀷.
region mmapped once, otherwise -EEXIST 杩斿洖.

璁惧 mmio region mapped 纭欢 mmio space. generally
浣跨敤 doorbell notification 纭欢. fast enough
鏁版嵁 channel.

璁惧 鐢ㄦ埛 share region 浣跨敤 share 鏁版嵁 缂撳啿鍖?鐢ㄦ埛 杩涚▼
璁惧.


### Uacce 瀵勫瓨鍣?API


瀵勫瓨鍣?API defined uacce.h.

```

  struct uacce_interface {
    char name[UACCE_MAX_NAME_SIZE];
    unsigned int flags;
    const struct uacce_ops *ops;
  };

```
According IOMMU capability, uacce_interface 鏍囧織 :

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

. uacce 妯″潡 compiled, ERR_PTR(-ENODEV)

b. Succeed desired 鏍囧織

c. Succeed negotiated 鏍囧織, 绀轰緥

uacce_interface.鏍囧織 = UACCE_DEV_SVA uacce->鏍囧織 = ~UACCE_DEV_SVA

鐢ㄦ埛 椹卞姩 need 妫€鏌?杩斿洖 鍊?well negotiated uacce->鏍囧織.


### 鐢ㄦ埛 椹卞姩


闃熷垪 鏂囦欢 mmap space need 鐢ㄦ埛 椹卞姩 wrap communication
鍗忚. Uacce 鎻愪緵 attributes sysfs 鐢ㄦ埛 椹卞姩
match right accelerator accordingly.
More details 鏂囨。/ABI/testing/sysfs-椹卞姩-uacce.
