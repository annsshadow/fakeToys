## fwctl bnxt 驱动


:Author: Pavan Chebbi

## 概述


BNXT 驱动通过 auxiliary_device 提供 fwctl 服务。bnxt_fwctl 驱动绑定到该设备，并fwctl
子系统注册自己
bnxt_fwctl 驱动对设备固件内部一无所知。它使用 bnxt 提供的上层协议（ULP）通道来向固件发硬件资源管理器（HWRM）命令
这些命令可以查询或更改由固件驱动的设备配置，以及读写对调试有用的寄存器
## bnxt_fwctl 用户 API


每个 RPC 请求fwctl_rpc 'in' 缓冲区中包含 HWRM 输入结构，'out' 将包含响应
一个典型的用户应用程序可以使用 ioctl() 发FWCTL_INFO 命令来发bnxt_fwctl RPC 能力如下所示：

        ioctl(fd, FWCTL_INFO, &fwctl_info_msg);

其中 fwctl_info_msg（类型为 struct fwctl_info）描述了 bnxt_info_msg（类型为 struct fwctl_info_bnxt）fwctl_info_msg 设置如下
        size = sizeof(struct fwctl_info);
        flags = 0;
        device_data_len = sizeof(bnxt_info_msg);
        out_device_data = (__aligned_u64)&bnxt_info_msg;

bnxt_info_msg uctx_caps 表示 include/uapi/fwctl/bnxt.h fwctl_bnxt_commands 所描述能力
FW RPC 本身，FWCTL_RPC 可使ioctl() 发送，如下所示：

        ioctl(fd, FWCTL_RPC, &fwctl_rpc_msg);

其中 fwctl_rpc_msg（类型为 struct fwctl_rpc）在'in' 缓冲区中携带 HWRM 命令。HWRM 输入
结构include/linux/bnxt/hsi.h 中描述。HWRM_VER_GET 的示例如下所示：

        struct hwrm_ver_get_output resp;
        struct fwctl_rpc fwctl_rpc_msg;
        struct hwrm_ver_get_input req;

        req.req_type = HWRM_VER_GET;
        req.hwrm_intf_maj = HWRM_VERSION_MAJOR;
        req.hwrm_intf_min = HWRM_VERSION_MINOR;
        req.hwrm_intf_upd = HWRM_VERSION_UPDATE;
        req.cmpl_ring = -1;
        req.target_id = -1;

        fwctl_rpc_msg.size = sizeof(struct fwctl_rpc);
        fwctl_rpc_msg.scope = FWCTL_RPC_DEBUG_READ_ONLY;
        fwctl_rpc_msg.in_len = sizeof(req);
        fwctl_rpc_msg.out_len = sizeof(resp);
        fwctl_rpc_msg.in = (__aligned_u64)&req;
        fwctl_rpc_msg.out = (__aligned_u64)&resp;

可以练习此接口的示例 python3 程序可在以下 git 仓库中找到：

https://github.com/Broadcom/fwctl-tools
