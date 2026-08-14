
## 权威的 SEV 客体 API 文档


## 1. 总体描述


SEV API 是一组 ioctl，由客体或虚拟机监控器用来获取或设置 SEV 虚拟机的某个方面。这些
ioctl 属于以下类别：

 - 虚拟机监控器 ioctl：这些查询并设置影响整个 SEV 固件的全局属性。这些 ioctl 由平台
   配置工具使用。

 - 客体 ioctl：这些查询并设置 SEV 虚拟机的属性。

## 2. API 描述


本节描述用于从 SEV 固件查询 SEV 客体报告的 ioctl。对于每个 ioctl，除了描述外还提供以下
信息：

  Technology（技术）：
      由哪个 SEV 技术提供此 ioctl。SEV、SEV-ES、SEV-SNP 或全部。

  Type（类型）：
      虚拟机监控器或客体。该 ioctl 可以在客体或虚拟机监控器内部使用。

  Parameters（参数）：
      该 ioctl 接受哪些参数。

  Returns（返回）：
      返回值。一般的错误号（-ENOMEM、-EINVAL）不展开说明，但有特定含义的错误会说明。

客体 ioctl 应当在一个 /dev/sev-guest 设备的文件描述符上发出。该 ioctl 接受
struct snp_user_guest_request。输入和输出结构分别通过 req_data 和 resp_data 字段指定。
如果 ioctl 由于固件错误而执行失败，则 fw_error 代码会被设置，否则 fw_error 会被设为 -1。

固件会检查消息序列计数器比客体的消息序列计数器大 1。如果客体驱动未能递增消息计数器
（例如计数器溢出），则返回 -EIO。
```

        struct snp_guest_request_ioctl {
                /* 消息版本号 */
                __u32 msg_version;

                /* 请求和响应结构的地址 */
                __u64 req_data;
                __u64 resp_data;

                /* bits[63:32]: VMM 错误码, bits[31:0] 固件错误码 (见 psp-sev.h) */
                union {
                        __u64 exitinfo2;
                        struct {
                                __u32 fw_error;
                                __u32 vmm_error;
                        };
                };
        };

```
主机 ioctl 被发出到一个 /dev/sev 设备的文件描述符。该 ioctl 接受如下所述的命令
ID/输入结构。
```

        struct sev_issue_cmd {
                /* 命令 ID */
                __u32 cmd;

                /* 命令请求结构 */
                __u64 data;

                /* 失败时的固件错误码 (见 psp-sev.h) */
                __u32 error;
        };


```
### 2.1 SNP_GET_REPORT

:Technology: sev-snp
:Type: guest ioctl
:Parameters (in): struct snp_report_req
:Returns (out): struct snp_report_resp on success, -negative on error

SNP_GET_REPORT ioctl 可用于从 SEV-SNP 固件查询证明（attestation）报告。该 ioctl 使用
SEV-SNP 固件提供的 SNP_GUEST_REQUEST（MSG_REPORT_REQ）命令来查询证明报告。

成功时，snp_report_resp.data 将包含报告。报告包含的格式在 SEV-SNP 规范中描述。更多
细节请参阅 SEV-SNP 规范。

### 2.2 SNP_GET_DERIVED_KEY

:Technology: sev-snp
:Type: guest ioctl
:Parameters (in): struct snp_derived_key_req
:Returns (out): struct snp_derived_key_resp on success, -negative on error

SNP_GET_DERIVED_KEY ioctl 可用于获取从一个根密钥派生的密钥。派生的密钥可以被客体用于
任何目的，例如密封密钥（sealing keys）或与外部实体通信。

该 ioctl 使用 SEV-SNP 固件提供的 SNP_GUEST_REQUEST（MSG_KEY_REQ）命令来派生密钥。关于
密钥派生请求中传入的各个字段的更多细节，请参阅 SEV-SNP 规范。

成功时，snp_derived_key_resp.data 包含派生的密钥值。更多细节请参阅 SEV-SNP 规范。

### 2.3 SNP_GET_EXT_REPORT

:Technology: sev-snp
:Type: guest ioctl
:Parameters (in/out): struct snp_ext_report_req
:Returns (out): struct snp_report_resp on success, -negative on error

SNP_GET_EXT_REPORT ioctl 与 SNP_GET_REPORT 类似。区别在于随报告一起返回的额外证书
数据。返回的证书数据由虚拟机监控器通过 SNP_SET_EXT_CONFIG 提供。

该 ioctl 使用 SEV-SNP 固件提供的 SNP_GUEST_REQUEST（MSG_REPORT_REQ）命令来获取证明报告。

成功时，snp_ext_report_resp.data 将包含证明报告，snp_ext_report_req.certs_address 将
包含证书 blob。如果 blob 的长度小于预期，则 snp_ext_report_req.certs_len 会被更新为
预期值。

关于如何解析证书 blob 的更多细节，请参阅 GHCB 规范。

### 2.4 SNP_PLATFORM_STATUS

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Parameters (out): struct sev_user_data_snp_status
:Returns (out): 0 on success, -negative on error

SNP_PLATFORM_STATUS 命令用于查询 SNP 平台状态。状态包括 API 主、次版本号等。更多细节
请参阅 SEV-SNP 规范。

### 2.5 SNP_COMMIT

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Returns (out): 0 on success, -negative on error

SNP_COMMIT 用于使用 SEV-SNP 固件的 SNP_COMMIT 命令提交当前已安装的固件。这防止回滚到
之前已提交的固件版本。这也会将报告的 TCB 更新为与当前已安装固件相匹配。

### 2.6 SNP_SET_CONFIG

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Parameters (in): struct sev_user_data_snp_config
:Returns (out): 0 on success, -negative on error

SNP_SET_CONFIG 用于设置系统范围的配置，例如证明报告中报告的 TCB 版本。该命令类似于
SEV-SNP 规范中定义的 SNP_CONFIG 命令。受此命令影响的固件参数的当前值可以通过
SNP_PLATFORM_STATUS 查询。

### 2.7 SNP_VLEK_LOAD

:Technology: sev-snp
:Type: hypervisor ioctl cmd
:Parameters (in): struct sev_user_data_snp_vlek_load
:Returns (out): 0 on success, -negative on error

在请求证明报告时，客体能够指定它是希望 SNP 固件使用由芯片唯一机密派生的版本化芯片
签注密钥（VCEK）来签署报告，还是使用从 AMD 密钥派生服务（KDS）获取、并由分配给已注册
云服务提供商的种子派生的版本化加载签注密钥（VLEK）。

对于 VLEK 密钥，SNP_VLEK_LOAD SNP 命令用于在从 KDS 获取它们之后将其加载到系统中，并且
与 SEV-SNP 规范中指定的 SNP_VLEK_LOAD 固件命令密切相关。

## 3. SEV-SNP CPUID 强制执行


SEV-SNP 客体可以访问一个特殊页，其中包含一张由 PSP 在 SNP_LAUNCH_UPDATE 固件命令过程中
验证过的 CPUID 值表。它针对 CPUID 值的有效性提供以下保证：

 - 它的地址通过引导加载程序/固件（经由 CC blob）获得，那些二进制文件将作为 SEV-SNP
   证明报告的一部分被度量。
 - 它的初始状态会被加密/pvalidated，因此在运行期间试图修改它会导致写入垃圾数据，或者
   如果虚拟机监控器试图替换后台页，会因验证状态变化而产生 #VC 异常。
 - 虚拟机监控器通过使用普通页或非 CPUID 加密页来绕过 PSP 检查的尝试，会改变 SEV-SNP
   证明报告提供的度量。
 - CPUID 页的内容**不**被度量，但作为客体初始化的一部分试图修改 CPUID 页的预期内容，
   会被 PSP 在 SNP_LAUNCH_UPDATE 期间对该页执行的 PSP CPUID 强制执行策略检查所拦截，并在
   之后（如果客体所有者实现了自己对 CPUID 值的检查）变得明显。

需要注意的是，最后这条保证只有在内核在引导的所有阶段都注意使用 SEV-SNP CPUID 时才有用。
否则，客体所有者证明无法提供内核在引导过程中某个时刻没有被喂入错误值的保证。

## 4. SEV 客体驱动通信密钥


SEV 客体与 AMD 安全处理器（ASP，即 PSP）中的 SEV 固件之间的通信受 VM 平台通信密钥
（VMPCK）保护。默认情况下，sev-guest 驱动使用客体运行所在的 VM 特权级（VMPL）关联的
VMPCK。如果该密钥被 sev-guest 驱动擦除（关于 VMPCK 可能被擦除的原因，请参见驱动），可以
通过重新加载 sev-guest 驱动并使用 vmpck_id 模块参数指定所需密钥来使用不同的密钥。

### 参考


SEV-SNP 和 GHCB 规范：developer.amd.com/sev

该驱动基于 SEV-SNP 固件规范 0.9 和 GHCB 规范版本 2.0。
