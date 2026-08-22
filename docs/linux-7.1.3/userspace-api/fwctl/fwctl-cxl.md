
## fwctl cxl driver


:Author: Dave Jiang

## Overview


CXL 规范定义了一组可以发送到 CXL 设备或交换机邮箱的命令。它也为发送到邮箱的厂商特定命令留出了空间。fwctl 提供了一条路径，允许用户空间发送一组被允许的信箱命令到设备，这些命令由内核驱动进行调节
将使用以3 条命令来支持 CXL 特性：
CXL spec r3.1 8.2.9.6.1 Get Supported Features (Opcode 0500h)
CXL spec r3.1 8.2.9.6.2 Get Feature (Opcode 0501h)
CXL spec r3.1 8.2.9.6.3 Set Feature (Opcode 0502h)

"Get Supported Features" 的返回数据可能会被内核驱动过滤，以丢弃任何被内核禁止或正被内核独占使用的特性。驱动会"Get Supported Features Supported Feature Entry" "Set Feature Size" 设为 0，以表示该特性不可被修改Get Supported Features" 命令"Get Features" 属于 FWCTL_RPC_CONFIGURATION fwctl 策略范畴
对于 "Set Feature" 命令，访问策略目前根据设备报告的 Set Feature 影响（effects）分为两类。如Set Feature 会导致设备立即发生变更，fwctl 访问策略必须FWCTL_RPC_DEBUG_WRITE_FULL。该级别的影响掩码（set effects mask）为 "immediate config change"immediate data change"immediate policy change" "immediate log change"。如果影响是 "config change with cold reset" "config change with conventional reset"，则 fwctl 访问策略必须FWCTL_RPC_DEBUG_WRITE 或更高
## fwctl cxl User API


### 1. Driver info query


应用程序的第一步是发出 ioctl(FWCTL_CMD_INFO)。成功调用该 ioctl 意味着 Features 能力可用，并返回一个全0 32 位负载。需要用 `FWCTL_DEVICE_TYPE_CXL` 填充 `fwctl_info.out_device_type` 来填`struct fwctl_info`。返回的数据应为 `struct fwctl_info_cxl`，其中包含一个应全为 0 的保32 位字段
### 2. Send hardware commands


下一步是从用户空间通过 ioctl(FWCTL_RPC) 向驱动发'Get Supported Features' 命令。由 `fwctl_rpc.in` 指向一`struct fwctl_rpc_cxl`。`struct fwctl_rpc_cxl.in_payload` 指向CXL 规范定义的硬件输入结构。`fwctl_rpc.out` 指向包含 `struct fwctl_rpc_cxl_out` 的缓冲区，后者将硬件输出数据内联`fwctl_rpc_cxl_out.payload`。该命令会被调用两次。第一次用于获取所支持特性的数量。第二次用于获取具体的特性详情作为输出数据
在获得具体的特性详情后，就可以适当地编写并发Get/Set Feature 命令。对"Set Feature" 命令，所检索到的特性信息包含一effects 字段，详细说明将要触发的 "Set Feature" 命令的结果。这会告知用户系统是否被配置为允许该 "Set Feature" 命令
#### Code example of a Get Feature



        static int cxl_fwctl_rpc_get_test_feature(int fd, struct test_feature *feat_ctx,
                                                  const uint32_t expected_data)
        {
                struct cxl_mbox_get_feat_in *feat_in;
                struct fwctl_rpc_cxl_out *out;
                struct fwctl_rpc rpc = {0};
                struct fwctl_rpc_cxl *in;
                size_t out_size, in_size;
                uint32_t val;
                void *data;
                int rc;

                in_size = sizeof(**in) + sizeof(**feat_in);
                rc = posix_memalign((void **)&in, 16, in_size);
                if (rc)
                        return -ENOMEM;
                memset(in, 0, in_size);
                feat_in = &in->get_feat_in;

                uuid_copy(feat_in->uuid, feat_ctx->uuid);
                feat_in->count = feat_ctx->get_size;

                out_size = sizeof(*out) + feat_ctx->get_size;
                rc = posix_memalign((void **)&out, 16, out_size);
                if (rc)
                        goto free_in;
                memset(out, 0, out_size);

                in->opcode = CXL_MBOX_OPCODE_GET_FEATURE;
                in->op_size = sizeof(*feat_in);

                rpc.size = sizeof(rpc);
                rpc.scope = FWCTL_RPC_CONFIGURATION;
                rpc.in_len = in_size;
                rpc.out_len = out_size;
                rpc.in = (uint64_t)(uint64_t *)in;
                rpc.out = (uint64_t)(uint64_t *)out;

                rc = send_command(fd, &rpc, out);
                if (rc)
                        goto free_all;

                data = out->payload;
                val = le32toh(**(__le32 **)data);
                if (memcmp(&val, &expected_data, sizeof(val)) != 0) {
                        rc = -ENXIO;
                        goto free_all;
                }

        free_all:
                free(out);
        free_in:
                free(in);
                return rc;
        }

有关如何运用此路径的详细用户代码示例，请参阅 CXL CLI 测试目录
<https://github.com/pmem/ndctl/tree/main/test/fwctl.c>銆。

## fwctl cxl Kernel API


   :export:
