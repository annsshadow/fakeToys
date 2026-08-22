
DPAA2 DPIO（数据通路 I/O）概



:版权所有：|copy| 2016-2018 NXP


本文档概述了 Freescale DPAA2 DPIO 驱动


简



DPAA2 DPIO（数据通路 I/O）是一个硬件对象，提供将帧入队和出队到网络接口及其他加速器的接口。DPIO 还为网络接口提供硬件缓冲池管理


本文档概述了 Linux DPIO 驱动、其子组件及API


有关 DPAA2 的总体概述以及 Linux DPAA2 驱动总体架构，请参阅
Documentation/networking/device_drivers/ethernet/freescale/dpaa2/overview.rst銆。


驱动概述



DPIO 驱动绑定到在 fsl-mc 总线上发现的 DPIO 对象，并提供以下服务


  A. 允许其他驱动（例如以太网驱动）为其各自的对象入队和出队帧
  B. 允许驱动注册数据可用通知回调，当队列或通道上有数据可用时触
  C. 允许驱动管理硬件缓冲


Linux DPIO 驱动3 个主要组件构成—
   DPIO 对象驱动——管DPIO 对象fsl-mc 驱动


   DPIO 服务——向其他 Linux 驱动提供服务 API

```

          fsl-mc          other
           bus           drivers
            |               |
        +---+----+   +------+-----+
        |DPIO obj|   |DPIO service|
        | driver |---|  (DPIO)    |
        +--------+   +------+-----+
                            |
                     +------+-----+
                     |    QBman   |
                     | portal i/f |
                     +------------+
                            |
                         hardware


```

下图展示DPIO 驱动各组件如何与其他部分配合
```

                                                   +------------+
                                                   | OS Network |
                                                   |   Stack    |
                 +------------+                    +------------+
                 | Allocator  |. . . . . . .       |  Ethernet  |
                 |(DPMCP,DPBP)|                    |   (DPNI)   |
                 +-.----------+                    +---+---+----+
                  .          .                         ^   |
                 .            .           <data avail, |   |<enqueue,
                .              .           tx confirm> |   | dequeue>
    +-------------+             .                      |   |
    | DPRC driver |              .    +--------+ +------------+
    |   (DPRC)    |               . . |DPIO obj| |DPIO service|
    +----------+--+                   | driver |-|  (DPIO)    |
               |                      +--------+ +------+-----+
               |<dev add/remove>                 +------|-----+
               |                                 |   QBman    |
          +----+--------------+                  | portal i/f |
          |   MC-bus driver   |                  +------------+
          |                   |                     |
          | /soc/fsl-mc       |                     |
          +-------------------+                     |
                                                    |
 =========================================|=========|========================
                                        +-+--DPIO---|-----------+
                                        |           |           |
                                        |        QBman Portal   |
                                        +-----------------------+

 ============================================================================


```

DPIO 对象驱动（dpio-driver.c



   dpio-driver 组件fsl-mc 总线注册，以处理类型"dpio" 的对象。probe() 的实现处DPIO 的基本初始化，包括映DPIO 区域（QBman SW portal）以及初始化中断并注irq 处理函数。dpio-driver 将探测到DPIO 注册dpio-service


DPIO 服务（dpio-service.c, dpaa2-io.h



   dpio service 组件DPAA2 驱动（例如以太网驱动）提供入队、通知和缓冲管理方面的服务。系统通常会为每个 CPU 分配 1 DPIO 对象，以便入队操作能够在所CPU 上同时发生


   通知处理
      dpaa2_io_service_register()


      dpaa2_io_service_deregister()


      dpaa2_io_service_rearm()


   入队
      dpaa2_io_service_pull_fq()


      dpaa2_io_service_pull_channel()


      dpaa2_io_service_enqueue_fq()


      dpaa2_io_service_enqueue_qd()


      dpaa2_io_store_create()


      dpaa2_io_store_destroy()


      dpaa2_io_store_next()


   缓冲池管
      dpaa2_io_service_release()


      dpaa2_io_service_acquire()


QBman portal 接口（qbman-portal.c



   qbman-portal 组件提供用于执行底层硬件位操作的 API，例如：


      - 初始Qman 软件 portal
      - 鏋勫缓骞跺彂閫?portal 鍛戒护
      - portal 中断配置与处


   这些 qbman-portal API 不向其他驱动公开，仅dpio-service 使用


其他（dpaa2-fd.h, dpaa2-global.h



   帧描述符以及分散/聚集（scatter-gather）的定义，以及用于操作它们的 API，定义在 dpaa2-fd.h 中


   出队结果结构体及其解API 定义dpaa2-global.h 中

