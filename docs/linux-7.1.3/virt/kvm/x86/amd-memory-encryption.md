
## 安全加密虚拟化（SEV）


## 概述


安全加密虚拟化（SEV）是 AMD 处理器上提供的一项特性。

SEV 是 AMD-V 架构的扩展，支持在虚拟机监控器（hypervisor）控制下运行虚拟机（VM）。启用后，虚拟机的存储器内容将使用专属于该虚拟机的密钥进行透明加密。

虚拟机监控器可以通过 CPUID 指令确定是否支持 SEV。CPUID 功能 0x8000001f 报告相关信息

```

	0x8000001f[eax]:
			Bit[1] 	indicates support for SEV
	    ...
		  [ecx]:
			Bits[31:0]  Number of encrypted guests supported simultaneously

```
如果存在 SEV 支持，则 MSR 0xc001_0010（MSR_AMD64_SYSCFG）和 MSR 0xc001_0015

```

	0xc001_0010:
		Bit[23]	   1 = memory encryption can be enabled
			   0 = memory encryption can not be enabled

	0xc001_0015:
		Bit[0]	   1 = memory encryption can be enabled
			   0 = memory encryption can not be enabled

```
当 SEV 支持可用时，可以通过如下方式在特定的虚拟机中启用它

```

	VMCB[0x90]:
		Bit[1]	    1 = SEV is enabled
			    0 = SEV is disabled

```
SEV 硬件使用 ASID 将内存加密密钥与虚拟机关联。因此，启用 SEV 的客户机的 ASID 必须介于 1 与 CPUID 0x8000001f[ecx] 字段定义的最大值之间。

## KVM_MEMORY_ENCRYPT_OP ioctl


访问 SEV 的主要 ioctl 是 KVM_MEMORY_ENCRYPT_OP，它作用于 VM 文件描述符。如果 KVM_MEMORY_ENCRYPT_OP 的参数为 NULL，则当 SEV 启用时该 ioctl 返回 0，禁用时返回 `ENOTTY`（在某些较旧的 Linux 版本上，即使参数为 NULL，该 ioctl 也会尝试正常运行，因此当 SEV 启用时很可能返回 `EFAULT` 而非零）。如果非 NULL，则参数指向

```

       struct kvm_sev_cmd {
               __u32 id;
               __u64 data;
               __u32 error;
               __u32 sev_fd;
       };


```
`id` 字段包含子命令，`data` 字段指向另一个包含该命令特定参数的结构体。`sev_fd` 应指向在 `/dev/sev` 设备上打开的文件描述符（如果需要的话，见各命令说明）。

输出时，`error` 在成功时为零，否则为错误码。错误码定义于 `<linux/psp-dev.h>`。

KVM 实现了以下命令，以支持 SEV 客户机的常见生命周期事件，例如启动、运行、快照、迁移和销毁。

### 1. KVM_SEV_INIT2


KVM_SEV_INIT2 命令由虚拟机监控器用于初始化 SEV 平台上下文。在典型的工作流中，此命令应是发出的第一个命令。

要被接受此命令，必须已将 KVM_X86_SEV_VM 或 KVM_X86_SEV_ES_VM 传给 KVM_CREATE_VM ioctl。使用这些机器类型创建的虚拟机，在调用 KVM_SEV_INIT2 之前无法运行。

参数：struct kvm_sev_init（输入）

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_init {
                __u64 vmsa_features;  /* initial value of features field in VMSA */
                __u32 flags;          /* must be 0 */
                __u16 ghcb_version;   /* maximum guest GHCB version allowed */
                __u16 pad1;
                __u32 pad2[8];
        };

```
如果虚拟机监控器不支持 `flags` 或 `vmsa_features` 中设置的任何位，则为错误。对于 SEV 虚拟机，`vmsa_features` 必须为零，因为它们没有 VMSA。

对于 SEV 虚拟机，`ghcb_version` 必须为零，因为它们不发出 GHCB 请求。如果其他任何客户机类型的 `ghcb_version` 为零，则允许的最大客户机 GHCB 协议将默认使用版本 2。

此命令取代了已废弃的 KVM_SEV_INIT 和 KVM_SEV_ES_INIT 命令。这些命令没有任何参数（``data`` 字段未使用），并且仅适用于 KVM_X86_DEFAULT_VM 机器类型（0）。

它们的行为如同：

- KVM_SEV_INIT 的 VM 类型为 KVM_X86_SEV_VM，KVM_SEV_ES_INIT 为 KVM_X86_SEV_ES_VM

- `struct kvm_sev_init` 的 `flags` 和 `vmsa_features` 字段被设为零，且 KVM_SEV_INIT 的 `ghcb_version` 设为 0，KVM_SEV_ES_INIT 设为 1。

如果 `KVM_X86_SEV_VMSA_FEATURES` 属性不存在，则虚拟机监控器仅支持 KVM_SEV_INIT 和 KVM_SEV_ES_INIT。在此情况下，请注意 KVM_SEV_ES_INIT 可能会根据 `kvm-amd.ko` 的 `debug_swap` 参数的值设置 debug swap VMSA 特性（位 5）。

### 2. KVM_SEV_LAUNCH_START


KVM_SEV_LAUNCH_START 命令用于创建内存加密上下文。要创建加密上下文，用户必须提供客户机策略、所有者的公钥 Diffie-Hellman（PDH）密钥和会话信息。

参数：struct kvm_sev_launch_start（输入/输出）

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_launch_start {
                __u32 handle;           /* if zero then firmware creates a new handle */
                __u32 policy;           /* guest's policy */

                __u64 dh_uaddr;         /* userspace address pointing to the guest owner's PDH key */
                __u32 dh_len;

                __u64 session_addr;     /* userspace address which points to the guest session information */
                __u32 session_len;
        };

```
成功时，'handle' 字段包含一个新句柄；出错时为负数。

KVM_SEV_LAUNCH_START 要求 `sev_fd` 字段有效。

更多细节，请参见 SEV 规范第 6.2 节。

### 3. KVM_SEV_LAUNCH_UPDATE_DATA


KVM_SEV_LAUNCH_UPDATE_DATA 用于加密一个内存区域。它还会计算内存内容的度量值（measurement）。该度量是内存内容的签名，可以发送给客户机所有者，作为内存已被固件正确加密的证明（attestation）。

参数（输入）：struct kvm_sev_launch_update_data

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_launch_update {
                __u64 uaddr;    /* userspace address to be encrypted (must be 16-byte aligned) */
                __u32 len;      /* length of the data to be encrypted (must be 16-byte aligned) */
        };

```
更多细节，请参见 SEV 规范第 6.3 节。

### 4. KVM_SEV_LAUNCH_MEASURE


KVM_SEV_LAUNCH_MEASURE 命令用于获取由 KVM_SEV_LAUNCH_UPDATE_DATA 命令加密的数据的度量值。客户机所有者可能会等到能够验证度量值后，才向客户机提供机密信息。由于客户机所有者在启动时知道客户机的初始内容，因此可以通过将度量值与其期望的值进行比较来验证。

如果输入时 len 为零，则会将度量值 blob 的长度写入 len，uaddr 不被使用。

参数（输入）：struct kvm_sev_launch_measure

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_launch_measure {
                __u64 uaddr;    /* where to copy the measurement */
                __u32 len;      /* length of measurement blob */
        };

```
关于度量值验证流程的更多细节，请参见 SEV 规范第 6.4 节。

### 5. KVM_SEV_LAUNCH_FINISH


启动流程完成后，可以发出 KVM_SEV_LAUNCH_FINISH 命令，使客户机准备好执行。

返回值：成功时 0，出错时 -负数

### 6. KVM_SEV_GUEST_STATUS


KVM_SEV_GUEST_STATUS 命令用于获取已启用 SEV 的客户机的状态信息。

参数（输出）：struct kvm_sev_guest_status

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_guest_status {
                __u32 handle;   /* guest handle */
                __u32 policy;   /* guest policy */
                __u8 state;     /* guest state (see enum below) */
        };

```
SEV 客户机状态：

```

        enum {
        SEV_STATE_INVALID = 0;
        SEV_STATE_LAUNCHING,    /* guest is currently being launched */
        SEV_STATE_SECRET,       /* guest is being launched and ready to accept the ciphertext data */
        SEV_STATE_RUNNING,      /* guest is fully launched and running */
        SEV_STATE_RECEIVING,    /* guest is being migrated in from another SEV machine */
        SEV_STATE_SENDING       /* guest is getting migrated out to another SEV machine */
        };

```
### 7. KVM_SEV_DBG_DECRYPT


虚拟机监控器可以使用 KVM_SEV_DEBUG_DECRYPT 命令请求固件解密给定内存区域的数据。

参数（输入）：struct kvm_sev_dbg

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_dbg {
                __u64 src_uaddr;        /* userspace address of data to decrypt */
                __u64 dst_uaddr;        /* userspace address of destination */
                __u32 len;              /* length of memory region to decrypt */
        };

```
如果客户机策略不允许调试，该命令会返回错误。

### 8. KVM_SEV_DBG_ENCRYPT


虚拟机监控器可以使用 KVM_SEV_DEBUG_ENCRYPT 命令请求固件加密给定内存区域的数据。

参数（输入）：struct kvm_sev_dbg

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_dbg {
                __u64 src_uaddr;        /* userspace address of data to encrypt */
                __u64 dst_uaddr;        /* userspace address of destination */
                __u32 len;              /* length of memory region to encrypt */
        };

```
如果客户机策略不允许调试，该命令会返回错误。

### 9. KVM_SEV_LAUNCH_SECRET


虚拟机监控器可以使用 KVM_SEV_LAUNCH_SECRET 命令在度量值已被客户机所有者验证后注入机密数据。

参数（输入）：struct kvm_sev_launch_secret

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_launch_secret {
                __u64 hdr_uaddr;        /* userspace address containing the packet header */
                __u32 hdr_len;

                __u64 guest_uaddr;      /* the guest memory region where the secret should be injected */
                __u32 guest_len;

                __u64 trans_uaddr;      /* the hypervisor memory region which contains the secret */
                __u32 trans_len;
        };

```
### 10. KVM_SEV_GET_ATTESTATION_REPORT


虚拟机监控器可以使用 KVM_SEV_GET_ATTESTATION_REPORT 命令查询证明（attestation）报告，该报告包含通过 KVM_SEV_LAUNCH 命令传入的客户机内存和 VMSA 的 SHA-256 摘要，并用 PEK 签名。该命令返回的摘要应与客户机所有者通过 KVM_SEV_LAUNCH_MEASURE 使用的摘要相匹配。

如果输入时 len 为零，则会将度量值 blob 的长度写入 len，uaddr 不被使用。

参数（输入）：struct kvm_sev_attestation

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_attestation_report {
                __u8 mnonce[16];        /* A random mnonce that will be placed in the report */

                __u64 uaddr;            /* userspace address where the report should be copied */
                __u32 len;
        };

```
### 11. KVM_SEV_SEND_START


虚拟机监控器可以使用 KVM_SEV_SEND_START 命令创建外出的客户机加密上下文。

如果输入时 session_len 为零，则会将客户机会话信息的长度写入 session_len，其他所有字段不被使用。

参数（输入）：struct kvm_sev_send_start

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_send_start {
                __u32 policy;                 /* guest policy */

                __u64 pdh_cert_uaddr;         /* platform Diffie-Hellman certificate */
                __u32 pdh_cert_len;

                __u64 plat_certs_uaddr;        /* platform certificate chain */
                __u32 plat_certs_len;

                __u64 amd_certs_uaddr;        /* AMD certificate */
                __u32 amd_certs_len;

                __u64 session_uaddr;          /* Guest session information */
                __u32 session_len;
        };

```
### 12. KVM_SEV_SEND_UPDATE_DATA


虚拟机监控器可以使用 KVM_SEV_SEND_UPDATE_DATA 命令，使用 KVM_SEV_SEND_START 创建的加密上下文来加密外出的客户机内存区域。

如果输入时 hdr_len 或 trans_len 为零，则会将包头和传输区域的长度分别写入 hdr_len 和 trans_len，其他所有字段不被使用。

参数（输入）：struct kvm_sev_send_update_data

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_launch_send_update_data {
                __u64 hdr_uaddr;        /* userspace address containing the packet header */
                __u32 hdr_len;

                __u64 guest_uaddr;      /* the source memory region to be encrypted */
                __u32 guest_len;

                __u64 trans_uaddr;      /* the destination memory region  */
                __u32 trans_len;
        };

```
### 13. KVM_SEV_SEND_FINISH


迁移流程完成后，虚拟机监控器可以发出 KVM_SEV_SEND_FINISH 命令来删除加密上下文。

返回值：成功时 0，出错时 -负数

### 14. KVM_SEV_SEND_CANCEL


在完成 SEND_START 之后、SEND_FINISH 之前，源 VMM 可以发出 SEND_CANCEL 命令来停止迁移。这是必要的，以便被取消的迁移稍后可以使用新的目标重新启动。

返回值：成功时 0，出错时 -负数

### 15. KVM_SEV_RECEIVE_START


KVM_SEV_RECEIVE_START 命令用于为进入的 SEV 客户机创建内存加密上下文。要创建加密上下文，用户必须提供客户机策略、平台公钥 Diffie-Hellman（PDH）密钥和会话信息。

参数：struct kvm_sev_receive_start（输入/输出）

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_receive_start {
                __u32 handle;           /* if zero then firmware creates a new handle */
                __u32 policy;           /* guest's policy */

                __u64 pdh_uaddr;        /* userspace address pointing to the PDH key */
                __u32 pdh_len;

                __u64 session_uaddr;    /* userspace address which points to the guest session information */
                __u32 session_len;
        };

```
成功时，'handle' 字段包含一个新句柄；出错时为负数。

更多细节，请参见 SEV 规范第 6.12 节。

### 16. KVM_SEV_RECEIVE_UPDATE_DATA


虚拟机监控器可以使用 KVM_SEV_RECEIVE_UPDATE_DATA 命令，将进入的缓冲区复制到在 KVM_SEV_RECEIVE_START 期间创建了加密上下文的客户机内存区域。

参数（输入）：struct kvm_sev_receive_update_data

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_launch_receive_update_data {
                __u64 hdr_uaddr;        /* userspace address containing the packet header */
                __u32 hdr_len;

                __u64 guest_uaddr;      /* the destination guest memory region */
                __u32 guest_len;

                __u64 trans_uaddr;      /* the incoming buffer memory region  */
                __u32 trans_len;
        };

```
### 17. KVM_SEV_RECEIVE_FINISH


迁移流程完成后，虚拟机监控器可以发出 KVM_SEV_RECEIVE_FINISH 命令使客户机准备好执行。

返回值：成功时 0，出错时 -负数

### 18. KVM_SEV_SNP_LAUNCH_START


KVM_SNP_LAUNCH_START 命令用于为 SEV-SNP 客户机创建内存加密上下文。必须在发出 KVM_SEV_SNP_LAUNCH_UPDATE 或 KVM_SEV_SNP_LAUNCH_FINISH 之前调用它；

参数（输入）：struct kvm_sev_snp_launch_start

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_snp_launch_start {
                __u64 policy;           /* Guest policy to use. */
                __u8 gosvw[16];         /* Guest OS visible workarounds. */
                __u16 flags;            /* Must be zero. */
                __u8 pad0[6];
                __u64 pad1[4];
        };

```
关于 `struct kvm_sev_snp_launch_start` 中输入参数的更多细节，请参见 SEV-SNP 规范 [snp-fw-abi]_ 中的 SNP_LAUNCH_START。

### 19. KVM_SEV_SNP_LAUNCH_UPDATE


KVM_SEV_SNP_LAUNCH_UPDATE 命令用于将用户空间提供的数据加载到客户机 GPA 范围中，将内容度量到由 KVM_SEV_SNP_LAUNCH_START 创建的 SNP 客户机上下文中，然后对该 GPA 范围进行加密/验证，使其在启动后即可使用与该客户机上下文关联的加密密钥直接读取；此后，它可以在解锁任何机密之前，对其上下文关联的度量值进行证明（attest）。

此命令初始化的 GPA 范围必须事先设置 KVM_MEMORY_ATTRIBUTE_PRIVATE 属性。关于这方面的更多细节，请参见 KVM_SET_MEMORY_ATTRIBUTES 的文档。

成功时，不能保证此命令已处理所请求的整个范围。相反，`struct kvm_sev_snp_launch_update` 的 `gfn_start`、`uaddr` 和 `len` 字段会被更新为对应于尚未处理的剩余范围。调用者应继续调用此命令，直到这些字段表明整个范围已处理完毕，例如 `len` 为 0，`gfn_start` 等于范围中最后一个 GFN 加 1，且 `uaddr` 为用户空间提供的源缓冲区地址的最后一个字节加 1。在 `type` 为 KVM_SEV_SNP_PAGE_TYPE_ZERO 的情况下，`uaddr` 将被完全忽略。

参数（输入）：struct kvm_sev_snp_launch_update

返回值：成功时 0，出错时 < 0，需要调用者重试时 -EAGAIN

```

        struct kvm_sev_snp_launch_update {
                __u64 gfn_start;        /* Guest page number to load/encrypt data into. */
                __u64 uaddr;            /* 4k-aligned address of data to be loaded/encrypted. */
                __u64 len;              /* 4k-aligned length in bytes to copy into guest memory.*/
                __u8 type;              /* The type of the guest pages being initialized. */
                __u8 pad0;
                __u16 flags;            /* Must be zero. */
                __u32 pad1;
                __u64 pad2[4];

        };

```

```

        KVM_SEV_SNP_PAGE_TYPE_NORMAL
        KVM_SEV_SNP_PAGE_TYPE_ZERO
        KVM_SEV_SNP_PAGE_TYPE_UNMEASURED
        KVM_SEV_SNP_PAGE_TYPE_SECRETS
        KVM_SEV_SNP_PAGE_TYPE_CPUID

```
关于每种页面类型如何被使用/度量，请参见 SEV-SNP 规范 [snp-fw-abi]_。

### 20. KVM_SEV_SNP_LAUNCH_FINISH


SNP 客户机启动流程完成后，可以发出 KVM_SEV_SNP_LAUNCH_FINISH 命令使客户机准备好执行。

参数（输入）：struct kvm_sev_snp_launch_finish

返回值：成功时 0，出错时 -负数

```

        struct kvm_sev_snp_launch_finish {
                __u64 id_block_uaddr;
                __u64 id_auth_uaddr;
                __u8 id_block_en;
                __u8 auth_key_en;
                __u8 vcek_disabled;
                __u8 host_data[32];
                __u8 pad0[3];
                __u16 flags;                    /* Must be zero */
                __u64 pad1[4];
        };


```
关于 `struct kvm_sev_snp_launch_finish` 中输入参数的更多细节，请参见 SEV-SNP 规范 [snp-fw-abi]_ 中的 SNP_LAUNCH_FINISH。

### 21. KVM_SEV_SNP_ENABLE_REQ_CERTS


KVM_SEV_SNP_ENABLE_REQ_CERTS 命令会将 KVM 配置为在处理客户机证明报告时，以 `KVM_EXIT_SNP_REQ_CERTS` 退出类型退出到用户空间，从而允许用户空间提供与固件用于签署该证明报告的背书密钥（endorsement key）相对应的证书。

返回值：成功时 0，出错时 -负数

注意：固件使用的背书密钥可能会因为更新 SEV-SNP 固件或加载新的背书密钥等管理活动而改变，因此需要小心确保返回的证书数据与发送证明请求时固件实际使用的背书密钥保持同步。建议的方案是使用文件锁（例如通过 fcntl() 的 F_OFD_SETLK），方式如下：

  - 在作为处理 `KVM_EXIT_SNP_REQ_CERTS` 退出类型的一部分而获取/提供证书数据之前，VMM 应在读取证书 blob 文件并将其返回给 KVM 之前，获取该文件上的共享/读锁或独占/写锁，并继续持有该锁，直到证明请求实际发送到固件。为方便起见，VMM 可以在提供证书数据之后、恢复 vCPU 之前，设置 kvm_run 的 `immediate_exit` 标志。这将确保 vCPU 在从固件取回证明请求后会以 `-EINTR` 再次退出到用户空间，此时 VMM 可以安全地释放文件锁。

  - 对 SNP 固件 TCB 值或背书密钥执行更新（例如通过 `/dev/sev` 接口如 `SNP_COMMIT`、`SNP_SET_CONFIG` 或 `SNP_VLEK_LOAD`，更多细节请参见 Documentation/virt/coco/sev-guest.rst）且需要更新证书 blob 的工具/库，同样应在任何对背书密钥或证书 blob 内容的更新期间对证书 blob 持有独占锁，以确保使用上述方案的 VMM 不会返回与证明请求实际发出时固件使用的背书密钥不同步的证书 blob 数据。

推荐此方案，以便工具可以使用相当通用/自然的方法通过文件锁来同步固件/证书更新，从而更容易在工具/VMM/供应商之间保持互操作性。

## 设备属性 API


SEV 实现的属性可以通过 `/dev/kvm` 设备节点上的 `KVM_HAS_DEVICE_ATTR` 和 `KVM_GET_DEVICE_ATTR` ioctl，使用组 `KVM_X86_GRP_SEV` 来获取。

当前实现了以下属性：

- `KVM_X86_SEV_VMSA_FEATURES`：返回 `KVM_SEV_INIT2` 的 `vmsa_features` 中被接受的所有位的集合。

- `KVM_X86_SEV_SNP_REQ_CERTS`：如果内核支持 `KVM_EXIT_SNP_REQ_CERTS` 退出，则返回 1；该退出允许为每个 SNP 证明请求从用户空间获取背书密钥证书。

## 固件管理


SEV 客户机密钥管理由一个称为 AMD 安全处理器（AMD-SP）的独立处理器处理。运行在 AMD-SP 内部的固件提供了一个安全的密钥管理接口，用于执行常见的虚拟机监控器活动，例如加密引导代码、快照、迁移和调试客户机。更多信息请参见 SEV 密钥管理规范 [api-spec]_

AMD-SP 固件可以通过其自身的非易失性存储初始化，或者操作系统可以使用 `ccp` 模块的 `init_ex_path` 参数来管理固件的 NV 存储。如果 `init_ex_path` 指定的文件不存在或无效，操作系统将用 PSP 非易失性存储创建或覆盖该文件。

## 参考


更多信息请参见 [white-paper]_、[api-spec]_、[amd-apm]_、[kvm-forum]_ 和 [snp-fw-abi]_。
