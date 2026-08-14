## InfiniBand 与远程 DMA（RDMA）接口

本页是 InfiniBand 与远程直接内存访问（RDMA）子系统内核接口文档的入口，汇总了核心接口、RDMA Verbs 传输库以及上层协议（如 iSER）等相关参考。面向开发 RDMA 驱动与内核中间件、需要了解该子系统导出函数与内部接口的读者。



## 简介与概述


待定（TBD）

## InfiniBand 核心接口


    :internal:

    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

## RDMA Verbs 传输库


    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

    :export:

## 上层协议


### 用于 RDMA 的 iSCSI 扩展（iSER）


   :internal:

   :functions: iscsi_iser_pdu_alloc iser_initialize_task_headers
               iscsi_iser_task_init iscsi_iser_mtask_xmit iscsi_iser_task_xmit
               iscsi_iser_cleanup_task iscsi_iser_check_protection
               iscsi_iser_conn_create iscsi_iser_conn_bind
               iscsi_iser_conn_start iscsi_iser_conn_stop
               iscsi_iser_session_destroy iscsi_iser_session_create
               iscsi_iser_set_param iscsi_iser_ep_connect iscsi_iser_ep_poll
               iscsi_iser_ep_disconnect

   :internal:

   :internal:

### InfiniBand SCSI RDMA 协议目标支持


   :internal:

   :internal:

### 用于 RDMA 的 iSCSI 扩展（iSER）目标支持


   :internal:
