
## Intel(R) 管理引擎（ME）客户端总线 API


## 动机


MEI 字符设备对于专用应用程序从用户空间向 Intel ME 中的众多固件设备发送和
接收数据很有用。然而，对于 ME 的某些功能而言，复用现有的软件栈并通过现有的
内核子系统来暴露它们更有意义。

为了无缝接入内核设备驱动模型，我们在 MEI 驱动之上添加了一个内核虚拟总线
抽象。这使得可以为各种 MEI 特性实现 Linux 内核驱动，作为各自子系统中独立的
实体。甚至可以通过向现有代码添加一层 MEI CL 总线层，来潜在地复用已有的
设备驱动。


## MEI CL 总线 API


为某个 MEI 客户端实现驱动与任何其它基于总线的设备驱动非常相似。驱动通过
`include/linux/mei_cl_bus.c` 中定义的 `struct mei_cl_driver` 结构将自己
注册为 MEI CL 总线驱动。


        struct mei_cl_driver {
                struct device_driver driver;
                const char *name;

                const struct mei_cl_device_id *id_table;

                int (**probe)(struct mei_cl_device **dev, const struct mei_cl_id *id);
                int (**remove)(struct mei_cl_device **dev);
        };



`include/linux/mod_devicetable.h` 中定义的 `struct mei_cl_device_id` 结构允许
驱动将自己绑定到一个设备名。


        struct mei_cl_device_id {
                char name[MEI_CL_NAME_SIZE];
                uuid_le uuid;
                __u8    version;
                kernel_ulong_t driver_info;
        };

要真正在 ME 客户端总线上注册一个驱动，必须调用 `mei_cl_add_driver` API。这
通常在模块初始化时调用。

一旦驱动注册并绑定到设备，驱动通常会尝试在该总线上做一些 I/O，而这应当通过
`mei_cl_send` 和 `mei_cl_recv` 函数完成。更详细的信息见 API 一节。

为了让驱动收到有关待处理流量或事件的通知，驱动应当分别通过
`mei_cl_devev_register_rx_cb` 和 `mei_cldev_register_notify_cb` 函数注册回调。


### API:

    :export: drivers/misc/mei/bus.c



## 示例


作为一个理论示例，假设 ME 带有一个 "contact" NFC IP。该设备的驱动初始化和
退出例程如下所示：


        #define CONTACT_DRIVER_NAME "contact"

        static struct mei_cl_device_id contact_mei_cl_tbl[] = {
                { CONTACT_DRIVER_NAME, },

                /** required last entry **/
                { }
        };
        MODULE_DEVICE_TABLE(mei_cl, contact_mei_cl_tbl);

        static struct mei_cl_driver contact_driver = {
                .id_table = contact_mei_tbl,
                .name = CONTACT_DRIVER_NAME,

                .probe = contact_probe,
                .remove = contact_remove,
        };

        static int contact_init(void)
        {
                int r;

                r = mei_cl_driver_register(&contact_driver);
                if (r) {
                        pr_err(CONTACT_DRIVER_NAME ": driver registration failed\n");
                        return r;
                }

                return 0;
        }

        static void __exit contact_exit(void)
        {
                mei_cl_driver_unregister(&contact_driver);
        }

        module_init(contact_init);
        module_exit(contact_exit);

而该驱动简化后的 probe 例程如下所示：


        int contact_probe(struct mei_cl_device **dev, struct mei_cl_device_id **id)
        {
                [...]
                mei_cldev_enable(dev);

                mei_cldev_register_rx_cb(dev, contact_rx_cb);

                return 0;
        }

在 probe 例程中，驱动首先使能 MEI 设备，然后注册一个 rx 处理程序，这尽可能
接近于注册一个线程化 IRQ 处理程序。该处理程序的实现通常会调用
`mei_cldev_recv`，然后处理接收到的数据。


        #define MAX_PAYLOAD 128
        #define HDR_SIZE 4
        static void conntact_rx_cb(struct mei_cl_device *cldev)
        {
                struct contact *c = mei_cldev_get_drvdata(cldev);
                unsigned char payload[MAX_PAYLOAD];
                ssize_t payload_sz;

                payload_sz = mei_cldev_recv(cldev, payload,  MAX_PAYLOAD)
                if (reply_size < HDR_SIZE) {
                        return;
                }

                c->process_rx(payload);

        }

## MEI 客户端总线驱动


- [hdcp](hdcp)
- [nfc](nfc)
