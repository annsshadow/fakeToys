
## VFIO Mediated 璁惧


:Copyright: |copy| 2016, NVIDIA CORPORATION. 鍏ㄩ儴 rights reserved.
:Author: Neo Jia <cjia@nvidia.com>
:Author: Kirti Wankhede <kwankhede@nvidia.com>



## 铏氭嫙 鍑芥暟 I/O (VFIO) Mediated 璁惧[1]


The 鏁板瓧 鐨?浣跨敤 cases 鐢ㄤ簬 virtualizing DMA 璁惧 璇?鎵ц 涓?鍏锋湁 built-in
SR_IOV capability 鏄?increasing. Previously, 鍒?virtualize 姝ょ被 璁惧,
developers 鏇炬湁 鍒?鍒涘缓 瀹冧滑鐨?own 绠＄悊 interfaces 鍜?APIs, 鍜?鐒跺悗
integrate them 涓?鐢ㄦ埛绌洪棿 杞欢. 鍒?simplify integration 涓?鐢ㄦ埛绌洪棿
杞欢, 鎴戜滑 鍏锋湁 identified 閫氱敤 requirements 鍜?涓€涓?unified 绠＄悊
鎺ュ彛 鐢ㄤ簬 姝ょ被 璁惧.

The VFIO 椹卞姩 framework 鎻愪緵 unified APIs 鐢ㄤ簬 direct 璁惧 access. 瀹冩槸
涓€涓?IOMMU/device-agnostic framework 鐢ㄤ簬 exposing direct 璁惧 access 鍒?鐢ㄦ埛
space 鍦?涓€涓?secure, IOMMU-protected environment. 姝?framework 鏄?浣跨敤 鐢ㄤ簬
澶氫釜 璁惧, 渚嬪 GPUs, 缃戠粶 adapters, 鍜?compute accelerators. 涓?
direct 璁惧 access, 铏氭嫙 machines 鎴?鐢ㄦ埛绌洪棿 applications 鍏锋湁 direct
access 鍒?the 鐗╃悊 璁惧. 姝?framework 鏄?reused 鐢ㄤ簬 mediated 璁惧.

The mediated 鏍稿績 椹卞姩 鎻愪緵 涓€涓?閫氱敤 鎺ュ彛 鐢ㄤ簬 mediated 璁惧
绠＄悊 璇?鍙?涓?浣跨敤 鐢?椹卞姩 鐨?涓嶅悓 璁惧. 姝?妯″潡
鎻愪緵 涓€涓?generic 鎺ュ彛 鍒?perform 杩欎簺 鎿嶄綔:

- 鍒涘缓 鍜?destroy 涓€涓?mediated 璁惧
- Add 涓€涓?mediated 璁惧 鍒?鍜?remove 瀹?鏉ヨ嚜 涓€涓?mediated 鎬荤嚎 椹卞姩
- Add 涓€涓?mediated 璁惧 鍒?鍜?remove 瀹?鏉ヨ嚜 涓€涓?IOMMU group

The mediated 鏍稿績 椹卞姩 涔?鎻愪緵 涓€涓?鎺ュ彛 鍒?娉ㄥ唽 涓€涓?鎬荤嚎 椹卞姩.
渚嬪, the mediated VFIO mdev 椹卞姩 鏄?designed 鐢ㄤ簬 mediated 璁惧 鍜?
supports VFIO APIs. The mediated 鎬荤嚎 椹卞姩 adds 涓€涓?mediated 璁惧 鍒?鍜?
removes 瀹?鏉ヨ嚜 涓€涓?VFIO group.

The 浠ヤ笅 high-level 鍧?diagram 鏄剧ず the 涓昏 components 鍜?interfaces
鍦?the VFIO mediated 椹卞姩 framework. The diagram 鏄剧ず NVIDIA, Intel, 鍜?IBM
```

     +---------------+
     |               |
     | +-----------+ |  mdev_register_driver() +--------------+
     | |           | +<------------------------+              |
     | |  mdev     | |                         |              |
     | |  bus      | +------------------------>+ vfio_mdev.ko |<-> VFIO user
     | |  driver   | |     probe()/remove()    |              |    APIs
     | |           | |                         +--------------+
     | +-----------+ |
     |               |
     |  MDEV CORE    |
     |   MODULE      |
     |   mdev.ko     |
     | +-----------+ |  mdev_register_parent() +--------------+
     | |           | +<------------------------+              |
     | |           | |                         | ccw_device.ko|<-> physical
     | |           | +------------------------>+              |    device
     | |           | |        callbacks        +--------------+
     | | Physical  | |
     | |  device   | |  mdev_register_parent() +--------------+
     | | interface | |<------------------------+              |
     | |           | |                         |  i915.ko     |<-> physical
     | |           | +------------------------>+              |    device
     | |           | |        callbacks        +--------------+
     | +-----------+ |
     +---------------+


```
## Registration Interfaces


The mediated 鏍稿績 椹卞姩 鎻愪緵 the 浠ヤ笅 types 鐨?registration
interfaces:

- Registration 鎺ュ彛 鐢ㄤ簬 涓€涓?mediated 鎬荤嚎 椹卞姩
- 鐗╃悊 璁惧 椹卞姩 鎺ュ彛

### Registration 鎺ュ彛 鐢ㄤ簬 涓€涓?Mediated 鎬荤嚎 椹卞姩


The registration 鎺ュ彛 鐢ㄤ簬 涓€涓?mediated 璁惧 椹卞姩 鎻愪緵 the 浠ヤ笅
```

     /*
      * struct mdev_driver [2] - Mediated device's driver
      * @probe: called when new device created
      * @remove: called when device removed
      * @driver: device driver structure
      */
     struct mdev_driver {
	     int  (*probe)  (struct mdev_device *dev);
	     void (*remove) (struct mdev_device *dev);
	     unsigned int (*get_available)(struct mdev_type *mtype);
	     ssize_t (*show_description)(struct mdev_type *mtype, char *buf);
	     struct device_driver    driver;
     };

```
涓€涓?mediated 鎬荤嚎 椹卞姩 鐢ㄤ簬 mdev 搴斿綋 浣跨敤 姝?缁撴瀯浣?鍦?the 鍑芥暟 calls
鍒?娉ㄥ唽 鍜?娉ㄩ攢 itself 涓?the 鏍稿績 椹卞姩:

```

    int mdev_register_driver(struct mdev_driver *drv);

```
```

    void mdev_unregister_driver(struct mdev_driver *drv);

```
The mediated 鎬荤嚎 椹卞姩's probe 鍑芥暟 搴斿綋 鍒涘缓 涓€涓?vfio_璁惧 鍦ㄢ€︿箣涓?
the mdev_璁惧 鍜?connect 瀹?鍒?涓€涓?appropriate implementation 鐨?
vfio_璁惧_ops.

褰?涓€涓?椹卞姩 wants 鍒?add the GUID creation sysfs 鍒?涓€涓?existing 璁惧 瀹?鍏锋湁
```

    int mdev_register_parent(struct mdev_parent *parent, struct device *dev,
			struct mdev_driver *mdev_driver);

```
姝?灏?鎻愪緵 the 'mdev_鍙楁敮鎸乢types/XX/鍒涘缓' 鏂囦欢 鍏?鍙?鐒跺悗 涓?
浣跨敤 鍒?trigger the creation 鐨?涓€涓?mdev_璁惧. The 宸插垱寤?mdev_璁惧 灏?涓?
attached 鍒?the specified 椹卞姩.

```

    void mdev_unregister_parent(struct mdev_parent *parent);

```
鍏?灏?unbind 鍜?destroy 鍏ㄩ儴 the 宸插垱寤?mdevs 鍜?remove the sysfs 鏂囦欢.

## Mediated 璁惧 绠＄悊 鎺ュ彛 Through sysfs


The 绠＄悊 鎺ュ彛 through sysfs enables 鐢ㄦ埛绌洪棿 杞欢, 渚嬪
libvirt, 鍒?query 鍜?configure mediated 璁惧 鍦?涓€涓?hardware-agnostic fashion.
姝?绠＄悊 鎺ュ彛 鎻愪緵 flexibility 鍒?the underlying 鐗╃悊
璁惧's 椹卞姩 鍒?鏀寔 鐗规€?渚嬪:

- Mediated 璁惧 hot plug
- 澶氫釜 mediated 璁惧 鍦?涓€涓?鍗曚釜 铏氭嫙 machine
- 澶氫釜 mediated 璁惧 鏉ヨ嚜 涓嶅悓 鐗╃悊 璁惧

### Links 鍦?the mdev_鎬荤嚎 绫?Directory

The /sys/绫?mdev_鎬荤嚎/ directory 鍖呭惈 links 鍒?璁惧 璇?鏄?registered
涓?the mdev 鏍稿績 椹卞姩.

### Directories 鍜?鏂囦欢 鍦ㄢ€︿笅 the sysfs 鐢ㄤ簬 姣忎釜 鐗╃悊 璁惧


```

  |- [parent physical device]
  |--- Vendor-specific-attributes [optional]
  |--- [mdev_supported_types]
  |     |--- [<type-id>]
  |     |   |--- create
  |     |   |--- name
  |     |   |--- available_instances
  |     |   |--- device_api
  |     |   |--- description
  |     |   |--- [devices]
  |     |--- [<type-id>]
  |     |   |--- create
  |     |   |--- name
  |     |   |--- available_instances
  |     |   |--- device_api
  |     |   |--- description
  |     |   |--- [devices]
  |     |--- [<type-id>]
  |          |--- create
  |          |--- name
  |          |--- available_instances
  |          |--- device_api
  |          |--- description
  |          |--- [devices]

```
- [mdev_鍙楁敮鎸乢types]

  The 鍒楀嚭 鐨?currently 鍙楁敮鎸?mediated 璁惧 types 鍜?瀹冧滑鐨?details.

  [<type-id>], 璁惧_api, 鍜?鍙敤_instances 鏄?mandatory attributes
  璇?搴斿綋 涓?provided 鐢?鍘傚晢 椹卞姩.

- [<type-id>]

  The [<type-id>] name 鏄?宸插垱寤?鐢?adding the 璁惧 椹卞姩 瀛楃涓?浣滀负 涓€涓?prefix
  鍒?the 瀛楃涓?provided 鐢?the 鍘傚晢 椹卞姩. 姝?鏍煎紡 鐨?姝?name 鏄?浣滀负
```

	sprintf(buf, "%s-%s", dev_driver_string(parent->dev), group->name);

```
- 璁惧_api

  姝?attribute 鏄剧ず 鍏?璁惧 API 鏄?姝ｅ湪 宸插垱寤? 渚嬪,
  "vfio-PCI" 鐢ㄤ簬 涓€涓?PCI 璁惧.

- 鍙敤_instances

  姝?attribute 鏄剧ず the 鏁板瓧 鐨?璁惧 鐨?绫诲瀷 <type-id> 璇?鍙?涓?
  宸插垱寤?

- [璁惧]

  姝?directory 鍖呭惈 links 鍒?the 璁惧 鐨?绫诲瀷 <type-id> 璇?鍏锋湁 宸茬粡
  宸插垱寤?

- name

  姝?attribute 鏄剧ず 涓€涓?human readable name.

- description

  姝?attribute 鍙?鏄剧ず brief 鐗规€?description 鐨?the 绫诲瀷. 杩欐槸 涓€涓?
  鍙€?attribute.

### Directories 鍜?鏂囦欢 鍦ㄢ€︿笅 the sysfs 鐢ㄤ簬 姣忎釜 mdev 璁惧


```

  |- [parent phy device]
  |--- [$MDEV_UUID]
         |--- remove
         |--- mdev_type {link to its type}
         |--- vendor-specific-attributes [optional]

```
- remove (鍐欏叆 浠?

Writing '1' 鍒?the 'remove' 鏂囦欢 destroys the mdev 璁惧. The 鍘傚晢 椹卞姩 鍙?
fail the remove() 鍥炶皟鍑芥暟 鑻?璇?璁惧 鏄?active 鍜?the 鍘傚晢 椹卞姩
doesn't 鏀寔 hot unplug.

```

	# echo 1 > /sys/bus/mdev/devices/$mdev_UUID/remove

```
### Mediated 璁惧 Hot plug


Mediated 璁惧 鍙?涓?宸插垱寤?鍜?assigned 鍦?runtime. The procedure 鍒?hot
plug 涓€涓?mediated 璁惧 鏄?the 鐩稿悓 浣滀负 the procedure 鍒?hot plug 涓€涓?PCI 璁惧.

## Translation APIs 鐢ㄤ簬 Mediated 璁惧


The 浠ヤ笅 APIs 鏄?provided 鐢ㄤ簬 translating 鐢ㄦ埛 pfn 鍒?host pfn 鍦?涓€涓?VFIO
```

	int vfio_pin_pages(struct vfio_device *device, dma_addr_t iova,
				  int npage, int prot, struct page **pages);

	void vfio_unpin_pages(struct vfio_device *device, dma_addr_t iova,
				    int npage);

```
杩欎簺 鍑芥暟 call back 杩涘叆 the back-end IOMMU 妯″潡 鐢?浣跨敤 the pin_椤?
鍜?unpin_椤?callbacks 鐨?the 缁撴瀯浣?vfio_iommu_椹卞姩_ops[^4^]. Currently
杩欎簺 callbacks 鏄?鍙楁敮鎸?鍦?the 绫诲瀷1 IOMMU 妯″潡. 鍒?鍚敤 them 鐢ㄤ簬
鍏朵粬 IOMMU backend 妯″潡, 渚嬪 PPC64 sPAPR 妯″潡, 瀹冧滑 闇€瑕?鍒?鎻愪緵
杩欎簺 two 鍥炶皟鍑芥暟 鍑芥暟.

## References


1. 鍙傝 Documentation/driver-api/vfio.rst 鐢ㄤ簬 鏇村 information 鍦?VFIO.
2. 缁撴瀯浣?mdev_椹卞姩 鍦?鍖呭惈/linux/mdev.h
3. 缁撴瀯浣?mdev_parent_ops 鍦?鍖呭惈/linux/mdev.h
4. 缁撴瀯浣?vfio_iommu_椹卞姩_ops 鍦?鍖呭惈/linux/vfio.h
