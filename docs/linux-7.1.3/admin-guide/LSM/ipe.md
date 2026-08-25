
## 完整性策略强制执行（Integrity Policy Enforcement，IPE


   This is the documentation for admins, system builders, or individuals
   attempting to use IPE. If you're looking for more developer-focused
   documentation about IPE please see [IPE 的开发文</security/ipe>](the design docs </security/ipe>).

### 概述


完整性策略强制执行（Integrity Policy Enforcement，IPE）是一Linux 安全模块
（LSM），采用与传统访问控制互补的方式进行访问控制。与依赖标签和路径做决策传统访问控制机制不同，IPE 聚焦于系统组件所固有的、不可变的安全属性。这些属是系统组件的基本属性或特征，无法被更改，从而为安全决策提供了一致且可靠基础
具体而言，在 IPE 的语境中，系统组件主要指文件或这些文件所在的设备。不过，只是一个起点。系统组件的概念是灵活的，可以随着系统演进而扩展以纳入新的元素不可变属性包括文件的来源，它在时间上保持恒定且不可更改。例如，可以编写 IPE
策略来信任源initramfs 的文件。由initramfs 通常由引导加载程序验证，文件被视为可信；“文件来initramfs”在 IPE 的考量下成为一个不可变属性
不可变属性的概念还延伸到文件来源上所启用的安全特性，例如 dm-verity fs-verity，它们提供了一层完整性与信任保障。例如，IPE 允许定义信任来自
dm-verity 保护设备的文件的策略。dm-verity 通过提供其内容可验证且不可变状态来确保整个设备的完整性。类似地，fs-verity 提供文件系统级别的完整性检查，
IPE 能够强制执行信任fs-verity 保护的文件的策略。这两个特性一旦建立就
无法关闭，因此它们被视为不可变属性。这些示例展示了 IPE 如何利用不可变属（例如文件的来源及其完整性保护机制）来做出访问控制决策
具体而言，对IPE 策略，它具备通过将安全属性与策略中定义的参考值进行比对来
强制实施严格访问控制的能力。这种评估可以基于安全属性的存在（例如，验证某个
文件是否源自 initramfs），或者评估某个不可变安全属性的内部状态。后者包括检dm-verity 保护设备roothash、确dm-verity 是否拥有有效的签名、评fs-verity 保护文件digest，或者确fs-verity 是否拥有有效的内建签名。这细致的策略强制执行方法实现了高度安全且可定制、并针对特定安全需求与信任模型
量身打造的系统防御机制
要启IPE，请确保 `CONFIG_SECURITY_IPE`（位`Security -> Integrity Policy Enforcement (IPE)`）配置选项已启用
### 使用场景


IPE 在固定功能设备（fixed-function devices）上表现最佳：即那些用途被明确定义
且不应被更改的设备（例如数据中心中的网络防火墙设备、IoT 设备等），其中所软件和配置都由系统所有者构建与提供
IPE 距离用于通用计算还很遥远：Linux 社区整体上倾向于遵循去中心化的信任模型
（即众所周知的信任网，web of trust），IPE 目前还不支持它。相反，IPE 支持
PKI（公钥基础设施，public key infrastructure），它通常指定一组提供某种绝对信的可信实体
此外，虽然如今大多数软件包都经过签名，但软件包内部的文件（例如可执行文件往往未经签名。这使得在不对包管理器及其背后的生态系统做重大改动的情况下，很在期望包管理器可用的系统中利IPE
digest_cache LSM [#digest_cache_lsm]_ 是一个与 IPE 结合使用时，可用于启用并支持
通用计算使用场景的系统
### 已知限制


IPE 无法验证匿名可执行内存的完整性，例如gcc 闭包libffi3.4.2）创建的
trampoline，或 JIT 生成的代码。遗憾的是，由于这是动态生成的代码，IPE 无法确保
这些代码的完整性以形成信任基础
当解释型语言编写的程序通过将程序文件传递给解释器来调用时，IPE 无法验证这些
程序的完整性。这是因为解释器执行这些文件的方式：脚本本身并未通过 IPE 的某钩子作为可执行代码被评估，而仅仅是被读取的文本文件（与编译后的可执行文件相对）然而，随着 `AT_EXECVE_CHECK` 标志的引入（[AT_EXECVE_CHECK </userspace-api/check_exec>](AT_EXECVE_CHECK </userspace-api/check_exec>)），
解释器可以利用它向内核发出信号，表明某个脚本文件将被执行，并请求内核对其执行
LSM 安全检查
IPE EXECUTE 操作强制在编译后的可执行文件与解释型脚本之间存在差异：对于编后的可执行文件，当加载可执行内容时，强制由内核在 `execve()`、`execveat()``mmap()` `mprotect()` 系统调用期间自动触发。对于解释型脚本，强制需要解释器
使用`AT_EXECVE_CHECK` 标志`execveat()` 进行显式集成。与 IPE 在执行过程中
拦截exec 系统调用不同，该机制需要解释器主动采取行动，而现有的解释器除非添加了
该信号调用，否则不会被自动支持
### 威胁模型


IPE 专门瞄准内核初次启动后，对用户空间可执行代码（包括通过 `modprobe` `insmod` 从用户空间加载的内核模块）进行篡改的风险
举例来说，考虑这样一种场景：一个可能恶意、不受信任的二进制文件连同所有必要的
依赖（包括加载器libc）一起被下载。IPE 在此语境下的主要作用是阻止此类二进制
文件及其依赖的执行
IPE 通过允许它们运行之前验证所有可执行代码的完整性与真实性来实现这一点。它进行彻底的检查，以确保代码的完整性完好，并且它们符合所定义策略中授权的参考（digest、签名等）。如果一个二进制文件未能通过此验证过程——无论是由于其完整已被破坏，还是不满足授权标准——IPE 都将拒绝其执行。此外，IPE 会生成审计日志，
可用于检测和分析因策略违规导致的失败
篡改威胁场景包括由一系列参与者对可执行代码进行的修改或替换，包括
- 能够物理接触硬件的参与- 能够本地网络访问系统的参与- 能够访问部署系统的参与- 受外部控制的被攻破内部系- 系统的恶意最终用- 被攻破的系统最终用- 对系统的远程（外部）攻破

IPE 不缓解来自恶意但已授权开发者（拥有签名证书访问权限）的威胁，也不缓解他所使用的被攻破的开发工具（即面向返回编程攻击，return-oriented programming
attacks）。此外，IPE 在用户空间与内核空间之间划出了严格的安全边界。因此，内核
级漏洞利用被视为超出 IPE 的范围，缓解工作留给其它机制
### 策略


IPE 策略是一种纯文本 [#devdoc]_ 策略，由跨多行的多条语句组成。在策略顶部有一必需行，指明策略名称和策略版本，用于
```

   policy_name=Ex_Policy policy_version=0.0.0

```
策略名称是一个唯一键，以人类可读的名称标识此策略。它用于securityfs 下创节点，并唯一标识策略以部署新策略或更新现有策略
策略版本指示策略的当前版本（而非策略语法版本）。它用于防止将策略回滚到可能
不安全的旧版本
IPE 策略的下一部分是规则（rules）。规则由 key=value 对（称为属性，properties构成。IPE 规则需要两个属性：`action`，它决定 IPE 在匹配到该规则时做什么；以及
`op`，它决定应在何时评估该规则。顺序是有意义的，规则必须以 `op` 开头，并以
```

   op=EXECUTE action=ALLOW

```
结尾。此示例将允许任何执行。额外的属性用于评估被评估文件的不可变安全属性这些属性旨在描述内核中能够提供某种完整性验证的系统，使IPE 能够基于属性的来确定资源的信任度
规则自上而下评估。因此，任何撤销规则或拒绝规则都应放在文件靠前的位置，以确保
这些规则在带`action=ALLOW` 的规则之前被评估
IPE 策略支持注释。字'#' 将作为注释，忽略 '#' 右侧直到换行符之前的所有字符
IPE 评估的默认行为也可以在策略中通过 `DEFAULT` 语句来表达。这可以在全局级别完成```

   # Global
   DEFAULT action=ALLOW

   # Operation Specific
   DEFAULT op=EXECUTE action=ALLOW

```
必须IPE 中所有已知操作设置默认值。如果你想保持较旧策略与可能引入新操作的较新
内核兼容，请设置一个全局默认`ALLOW`，然后按操作逐个覆盖默认值（如上所示）
对于可配置的基于策略LSM，在启动时强制执行可配置策略、围绕读取和解析策略存在
若干问题
1. 内核**不应**从用户空间读取文件，因此直接读取策略文件是被禁止的2. 内核命令行有字符数限制，一个内核模块不应为其自身配置保留整个字符限制3. 内核生态系统中有各种各样的引导加载程序，因此交付一个内存块将是代价高昂、难   维护的
因此，IPE 通过一个称为“启动策略”（boot policy）的概念解决了这个问题。启动策是编译进内核的最小策略。该策略旨在将系统带入用户空间已就绪、可以接收命令的状态，
此时可以通过 securityfs 部署更复杂的策略。启动策略可以通过 `SECURITY_IPE_BOOT_POLICY`
配置选项指定，它接受一个指向要应用IPE 策略纯文本版本的路径。该策略将被编译内核。如果未指定，IPE 将被禁用，直到通过 securityfs 部署并激活某个策略
#### 部署策略


策略可以通过 securityfs 从用户空间部署。这些策略通过 PKCS#7 消息格式进行签名以强制实现某种程度的策略授权（禁止攻击者获得不受约束的 root 权限并部署一“allow all”策略）。这些策略必须由链接`SYSTEM_TRUSTED_KEYRING` 的证书签名，
或者——如果分别启用了 `CONFIG_IPE_POLICY_SIG_SECONDARY_KEYRING` `CONFIG_IPE_POLICY_SIG_PLATFORM_KEYRING`——由次级或平台密钥环签名```

   openssl smime -sign \
      -in "$MY_POLICY" \
      -signer "$MY_CERTIFICATE" \
      -inkey "$MY_PRIVATE_KEY" \
      -noattr \
      -nodetach \
      -nosmimecap \
      -outform der \
      -out "$MY_POLICY.p7b"

```
部署策略是通过 securityfs `new_policy` 节点完成的。要部署策略，只需将文cat ```

   cat "$MY_POLICY.p7b" > /sys/kernel/security/ipe/new_policy

```
成功后，这将`/sys/kernel/security/ipe/policies/` 下创建一个子目录。该子目将是所部署策略`policy_name` 字段，因此对于上面的示例，目录将`/sys/kernel/security/ipe/policies/Ex_Policy`。该目录中将有七个文件：`pkcs7``policy`、`name`、`version`、`active`、`update` `delete`
`pkcs7` 文件是只读的。读取它会返回提供给内核的、代表该策略的原PKCS#7 数据如果读取的策略是启动策略，由于它未经签名，这将返`ENOENT`
`policy` 文件是只读的。读取它会返回策略的 PKCS#7 内部内容，即纯文本策略
`active` 文件用于将某个策略设置为当前活动策略。该文件是可读写的（rw），接受`"1"` 以将该策略设为活动。由于同一时刻只能有一个策略处于活动状态，所有其它策都将被标记为不活动。被标记为活动的策略必须具有大于或等于当前运行版本的策略版本
`update` 文件用于更新已经存在于内核中的策略。该文件是只写的，接受一PKCS#7 签名策略。将始终对此策略执行两项检查：第一，`policy_names` 必须与更新版本和现有版本
匹配。第二，更新后的策略必须具有大于当前运行版本的策略版本。这是为了防止回滚攻击
`delete` 文件用于移除不再需要的策略。该文件是只写的，接受`1` 以删除该策略删除时，代表该策略的 securityfs 节点将被移除。不过，删除当前活动策略是不允许的，
会返回操作不被允许的错误
类似地，`update` `new_policy` 写入都可能导致坏消息（策略语法错误）或文已存在错误。后者发生在尝试部署一个带`policy_name` 的策略，而内核已经有一具有相同 `policy_name` 的已部署策略时
部署策略**不会**导致 IPE 开始强制执行该策略。IPE 只会强制执行被标记为活动的策略请注意，同一时刻只能有一个策略处于活动状态
部署成功后，可以通过写入文件
`/sys/kernel/security/ipe/policies/$policy_name/active` 来激活该策略```

   echo 1 > "/sys/kernel/security/ipe/policies/Ex_Policy/active"

```
从以上时刻起，`Ex_Policy` 现在就成为系统上被强制执行的策略
IPE 也提供删除策略的方式。这可以通过 `delete` securityfs 节点完成`/sys/kernel/security/ipe/policies/$policy_name/delete````

   echo 1 > "/sys/kernel/security/ipe/policies/$policy_name/delete"

```
删除策略只有一个要求：被删除的策略必须处于不活动状态

   If a traditional MAC system is enabled (SELinux, apparmor, smack), all
   writes to ipe's securityfs nodes require `CAP_MAC_ADMIN`.

#### 模式


IPE 支持两种运行模式：宽容模式（permissive，类似于 SELinux permissive 模式和强制模式（enforced）。在宽容模式下，所有事件都会被检查，策略违规会被记录，但
策略实际上并未被强制执行。这让用户能够在强制执行之前测试策略
默认模式是强制（enforce），可以通过内核命令行参`ipe.enforce=(0|1)`，或
securityfs 节点 `/sys/kernel/security/ipe/enforce` 来更改

   If a traditional MAC system is enabled (SELinux, apparmor, smack, etcetera),
   all writes to ipe's securityfs nodes require `CAP_MAC_ADMIN`.

#### 审计事件


##### 1420 AUDIT_IPE_ACCESS

```

   type=1420 audit(1653364370.067:61): ipe_op=EXECUTE ipe_hook=MMAP enforcing=1 pid=2241 comm="ld-linux.so" path="/deny/lib/libc.so.6" dev="sda2" ino=14549020 rule="DEFAULT action=DENY"
   type=1300 audit(1653364370.067:61): SYSCALL arch=c000003e syscall=9 success=no exit=-13 a0=7f1105a28000 a1=195000 a2=5 a3=812 items=0 ppid=2219 pid=2241 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=2 comm="ld-linux.so" exe="/tmp/ipe-test/lib/ld-linux.so" subj=unconfined key=(null)
   type=1327 audit(1653364370.067:61): 707974686F6E3300746573742F6D61696E2E7079002D6E00

   type=1420 audit(1653364735.161:64): ipe_op=EXECUTE ipe_hook=MMAP enforcing=1 pid=2472 comm="mmap_test" path=? dev=? ino=? rule="DEFAULT action=DENY"
   type=1300 audit(1653364735.161:64): SYSCALL arch=c000003e syscall=9 success=no exit=-13 a0=0 a1=1000 a2=4 a3=21 items=0 ppid=2219 pid=2472 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=2 comm="mmap_test" exe="/root/overlake_test/upstream_test/vol_fsverity/bin/mmap_test" subj=unconfined key=(null)
   type=1327 audit(1653364735.161:64): 707974686F6E3300746573742F6D61696E2E7079002D6E00

```
此事件表IPE 做出了一个访问控制决策；IPE 特定的记录（1420）总是与一`AUDITSYSCALL` 记录一起发出
可以通过 `AUDITSYSCALL` 记录`success` 属性和退出码来判IPE 处于宽容模式还是
强制模式

字段描述
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| Field     | Value Type | Optional? | Description of Value                                                            |
+===========+============+===========+=================================================================================+
| ipe_op    | string     | No        | The IPE operation name associated with the log                                  |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| ipe_hook  | string     | No        | The name of the LSM hook that triggered the IPE event                           |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| enforcing | integer    | No        | The current IPE enforcing state 1 is in enforcing mode, 0 is in permissive mode |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| pid       | integer    | No        | The pid of the process that triggered the IPE event.                            |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| comm      | string     | No        | The command line program name of the process that triggered the IPE event       |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| path      | string     | Yes       | The absolute path to the evaluated file                                         |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| ino       | integer    | Yes       | The inode number of the evaluated file                                          |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| dev       | string     | Yes       | The device name of the evaluated file, e.g. vda                                 |
+-----------+------------+-----------+---------------------------------------------------------------------------------+
| rule      | string     | No        | The matched policy rule                                                         |
+-----------+------------+-----------+---------------------------------------------------------------------------------+

##### 1421 AUDIT_IPE_CONFIG_CHANGE


```

   type=1421 audit(1653425583.136:54): old_active_pol_name="Allow_All" old_active_pol_version=0.0.0 old_policy_digest=sha256:E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855 new_active_pol_name="boot_verified" new_active_pol_version=0.0.0 new_policy_digest=sha256:820EEA5B40CA42B51F68962354BA083122A20BB846F26765076DD8EED7B8F4DB auid=4294967295 ses=4294967295 lsm=ipe res=1
   type=1300 audit(1653425583.136:54): SYSCALL arch=c000003e syscall=1 success=yes exit=2 a0=3 a1=5596fcae1fb0 a2=2 a3=2 items=0 ppid=184 pid=229 auid=4294967295 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=4294967295 comm="python3" exe="/usr/bin/python3.10" key=(null)
   type=1327 audit(1653425583.136:54): PROCTITLE proctitle=707974686F6E3300746573742F6D61696E2E7079002D66002E2

```
此事件表IPE 将活动策略从某一个切换到了另一个，并附带两个策略的版本与哈digest。注IPE 同一时刻只能有一个活动策略，所有访问决策评估都基于当前活动策略部署新策略的正常流程是先将待部署策略加载进内核，然后再将活动策略切换到它
此记录总是`write` 系统调用`AUDITSYSCALL` 记录一起发出
字段描述
+------------------------+------------+-----------+---------------------------------------------------+
| Field                  | Value Type | Optional? | Description of Value                              |
+========================+============+===========+===================================================+
| old_active_pol_name    | string     | Yes       | The name of previous active policy                |
+------------------------+------------+-----------+---------------------------------------------------+
| old_active_pol_version | string     | Yes       | The version of previous active policy             |
+------------------------+------------+-----------+---------------------------------------------------+
| old_policy_digest      | string     | Yes       | The hash of previous active policy                |
+------------------------+------------+-----------+---------------------------------------------------+
| new_active_pol_name    | string     | No        | The name of current active policy                 |
+------------------------+------------+-----------+---------------------------------------------------+
| new_active_pol_version | string     | No        | The version of current active policy              |
+------------------------+------------+-----------+---------------------------------------------------+
| new_policy_digest      | string     | No        | The hash of current active policy                 |
+------------------------+------------+-----------+---------------------------------------------------+
| auid                   | integer    | No        | The login user ID                                 |
+------------------------+------------+-----------+---------------------------------------------------+
| ses                    | integer    | No        | The login session ID                              |
+------------------------+------------+-----------+---------------------------------------------------+
| lsm                    | string     | No        | The lsm name associated with the event            |
+------------------------+------------+-----------+---------------------------------------------------+
| res                    | integer    | No        | The result of the audited operation(success/fail) |
+------------------------+------------+-----------+---------------------------------------------------+

##### 1422 AUDIT_IPE_POLICY_LOAD


```

   type=1422 audit(1653425529.927:53): policy_name="boot_verified" policy_version=0.0.0 policy_digest=sha256:820EEA5B40CA42B51F68962354BA083122A20BB846F26765076DD8EED7B8F4DB auid=4294967295 ses=4294967295 lsm=ipe res=1 errno=0
   type=1300 audit(1653425529.927:53): arch=c000003e syscall=1 success=yes exit=2567 a0=3 a1=5596fcae1fb0 a2=a07 a3=2 items=0 ppid=184 pid=229 auid=4294967295 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=0 fsgid=0 tty=pts0 ses=4294967295 comm="python3" exe="/usr/bin/python3.10" key=(null)
   type=1327 audit(1653425529.927:53): PROCTITLE proctitle=707974686F6E3300746573742F6D61696E2E7079002D66002E2E

```
此记录表示一个新策略已连同策略名称、策略版本和策略哈希被加载进内核
此记录总是`write` 系统调用`AUDITSYSCALL` 记录一起发出
字段描述
+----------------+------------+-----------+-------------------------------------------------------------+
| Field          | Value Type | Optional? | Description of Value                                        |
+================+============+===========+=============================================================+
| policy_name    | string     | Yes       | The policy_name                                             |
+----------------+------------+-----------+-------------------------------------------------------------+
| policy_version | string     | Yes       | The policy_version                                          |
+----------------+------------+-----------+-------------------------------------------------------------+
| policy_digest  | string     | Yes       | The policy hash                                             |
+----------------+------------+-----------+-------------------------------------------------------------+
| auid           | integer    | No        | The login user ID                                           |
+----------------+------------+-----------+-------------------------------------------------------------+
| ses            | integer    | No        | The login session ID                                        |
+----------------+------------+-----------+-------------------------------------------------------------+
| lsm            | string     | No        | The lsm name associated with the event                      |
+----------------+------------+-----------+-------------------------------------------------------------+
| res            | integer    | No        | The result of the audited operation(success/fail)           |
+----------------+------------+-----------+-------------------------------------------------------------+
| errno          | integer    | No        | Error code from policy loading operations (see table below) |
+----------------+------------+-----------+-------------------------------------------------------------+

策略错误码（errno）：

以下表格列出了在加载或更新策略时可能出现errno 字段中的错误码：

+----------------+--------------------------------------------------------+
| Error Code     | Description                                            |
+================+========================================================+
| 0              | Success                                                |
+----------------+--------------------------------------------------------+
| -EPERM         | Insufficient permission                                |
+----------------+--------------------------------------------------------+
| -EEXIST        | Same name policy already deployed                      |
+----------------+--------------------------------------------------------+
| -EBADMSG       | Policy is invalid                                      |
+----------------+--------------------------------------------------------+
| -ENOMEM        | Out of memory (OOM)                                    |
+----------------+--------------------------------------------------------+
| -ERANGE        | Policy version number overflow                         |
+----------------+--------------------------------------------------------+
| -EINVAL        | Policy version parsing error                           |
+----------------+--------------------------------------------------------+
| -ENOKEY        | Key used to sign the IPE policy not found in keyring   |
+----------------+--------------------------------------------------------+
| -EKEYREJECTED  | Policy signature verification failed                   |
+----------------+--------------------------------------------------------+
| -ESTALE        | Attempting to update an IPE policy with older version  |
+----------------+--------------------------------------------------------+
| -ENOENT        | Policy was deleted while updating                      |
+----------------+--------------------------------------------------------+

##### 1404 AUDIT_MAC_STATUS


```

   type=1404 audit(1653425689.008:55): enforcing=0 old_enforcing=1 auid=4294967295 ses=4294967295 enabled=1 old-enabled=1 lsm=ipe res=1
   type=1300 audit(1653425689.008:55): arch=c000003e syscall=1 success=yes exit=2 a0=1 a1=55c1065e5c60 a2=2 a3=0 items=0 ppid=405 pid=441 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=)
   type=1327 audit(1653425689.008:55): proctitle="-bash"

   type=1404 audit(1653425689.008:55): enforcing=1 old_enforcing=0 auid=4294967295 ses=4294967295 enabled=1 old-enabled=1 lsm=ipe res=1
   type=1300 audit(1653425689.008:55): arch=c000003e syscall=1 success=yes exit=2 a0=1 a1=55c1065e5c60 a2=2 a3=0 items=0 ppid=405 pid=441 auid=0 uid=0 gid=0 euid=0 suid=0 fsuid=0 egid=0 sgid=)
   type=1327 audit(1653425689.008:55): proctitle="-bash"

```
此记录总是`write` 系统调用`AUDITSYSCALL` 记录一起发出
字段描述
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| Field         | Value Type | Optional? | Description of Value                                                                            |
+===============+============+===========+=================================================================================================+
| enforcing     | integer    | No        | The enforcing state IPE is being switched to, 1 is in enforcing mode, 0 is in permissive mode   |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| old_enforcing | integer    | No        | The enforcing state IPE is being switched from, 1 is in enforcing mode, 0 is in permissive mode |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| auid          | integer    | No        | The login user ID                                                                               |
+---------------+------------+-----------+---------------------------------------------------------------------------------------------------+
| ses           | integer    | No        | The login session ID                                                                            |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| enabled       | integer    | No        | The new TTY audit enabled setting                                                               |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| old-enabled   | integer    | No        | The old TTY audit enabled setting                                                               |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| lsm           | string     | No        | The lsm name associated with the event                                                          |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+
| res           | integer    | No        | The result of the audited operation(success/fail)                                               |
+---------------+------------+-----------+-------------------------------------------------------------------------------------------------+


##### 成功审计


IPE 支持成功审计（success auditing）。启用后，所有通过 IPE 策略且未被阻止的事件
都会发出一条审计事件。默认情况下此功能被禁用，可以通过内核命令`ipe.success_audit=(0|1)` `/sys/kernel/security/ipe/success_audit`
securityfs 文件启用
*非常**嘈杂，因IPE 会检查系统上的每一个用户空间二进制文件，但对调试策很有用

   If a traditional MAC system is enabled (SELinux, apparmor, smack, etcetera),
   all writes to ipe's securityfs nodes require `CAP_MAC_ADMIN`.

### 属

如上所述，IPE 属性是IPE 策略中表达的 `key=value` 对。有两个属性内建（built-in于策略解析器：`op` `action`。其它属性用于限制被评估文件的不可变安全属性目前这些属性是`boot_verified`'、`'dmverity_signature'`、`'dmverity_roothash'``'fsverity_signature'`、`'fsverity_digest'`。IPE 支持的所有属性的描述如下
#### op


指示规则所适用的操作。必须作为每个规则的第一个标记出现。IPE 支持以下操作
   `EXECUTE`

      与任何试图被执行、或作为可执行文件加载的文件相关
   `FIRMWARE`锛?
      与通过 firmware_class 接口加载的固件相关。这同时涵盖预分配的缓冲区和
      固件文件本身
   `KMODULE`锛?
      与通过 `modprobe` `insmod` 加载内核模块相关
   `KEXEC_IMAGE`锛?
      与通过 `kexec` 加载内核映像相关
   `KEXEC_INITRAMFS`

      与通过 `kexec --initrd` 加载 initrd 映像相关
   `POLICY`锛?
      通过内核空间发起的读取来控制策略加载
      此类的一个例子是通过将策略文件路径写`$securityfs/ima/policy` 来加      IMA 策略
   `X509_CERT`锛?
      閫氳繃 Kconfig `CONFIG_IMA_X509_PATH` 鍜?`CONFIG_EVM_X509_PATH` 鎺у埗
      加载 IMA 证书
#### action


   决定当规则匹配时 IPE 应该做什么。必须作为每个规则的最后一个子句出现。可   是以下之一
   `ALLOW`锛?
      如果规则匹配，显式允许继续访问该资源，不再执行更多规则
   `DENY`锛?
      如果规则匹配，显式禁止继续访问该资源，不再执行更多规则
#### boot_verified


   此属性可用于授权来自 initramfs 的文件```

         boot_verified=(TRUE|FALSE)


   .. WARNING::

      This property will trust files from initramfs(rootfs). It should
      only be used during early booting stage. Before mounting the real
      rootfs on top of the initramfs, initramfs script will recursively
      remove all files and directories on the initramfs. This is typically
      implemented by using switch_root(8) [#switch_root]_. Therefore the
      initramfs will be empty and not accessible after the real
      rootfs takes over. It is advised to switch to a different policy
      that doesn't rely on the property after this point.
      This ensures that the trust policies remain relevant and effective
      throughout the system's operation.

```
#### dmverity_roothash


   此属性可用于授权或撤销特定dm-verity 卷，通过它们root hash 进行标识   它依赖于 DM_VERITY 模块。此属性由 `IPE_PROP_DM_VERITY` 配置选项控制，当
   `SECURITY_IPE` `DM_VERITY` 都启用时会自动被选中```

      dmverity_roothash=DigestName:HexadecimalString

   The supported DigestNames for dmverity_roothash are [#dmveritydigests]_

      + blake2b-512
      + blake2s-256
      + sha256
      + sha384
      + sha512
      + sha3-224
      + sha3-256
      + sha3-384
      + sha3-512
      + sm3
      + rmd160

```
#### dmverity_signature


   此属性可用于授权所有拥有由 dm-verity 配置指定的密钥环（要么是系统可信密钥环，
   要么是次级密钥环）验证过的签root hash dm-verity 卷。它依赖   `DM_VERITY_VERIFY_ROOTHASH_SIG` 配置选项，并`IPE_PROP_DM_VERITY_SIGNATURE`
   配置选项控制，当 `SECURITY_IPE`、`DM_VERITY`    `DM_VERITY_VERIFY_ROOTHASH_SIG` 都启用时会自动被选中```

      dmverity_signature=(TRUE|FALSE)

```
#### fsverity_digest


   此属性可用于授权特定的、启用了 fs-verity 的文件，通过它们fs-verity digest
   进行标识。它依赖`FS_VERITY` 配置选项，并`IPE_PROP_FS_VERITY` 配置选项
   控制，当 `SECURITY_IPE` `FS_VERITY` 都启用时会自动被选中```

      fsverity_digest=DigestName:HexadecimalString

   The supported DigestNames for fsverity_digest are [#fsveritydigest]_

      + sha256
      + sha512

```
#### fsverity_signature


   此属性用于授权所有由 fs-verity 内建签名机制验证过的、启用了 fs-verity 的文件   签名验证依赖于存储在 ".fs-verity" 密钥环中的密钥。它依赖   `FS_VERITY_BUILTIN_SIGNATURES` 配置选项，并`IPE_PROP_FS_VERITY` 配置选项
   控制，当 `SECURITY_IPE`、`FS_VERITY` `FS_VERITY_BUILTIN_SIGNATURES`    启用时会自动被选中```

      fsverity_signature=(TRUE|FALSE)

```
### 策略示例


#### 允许全部


```

   policy_name=Allow_All policy_version=0.0.0
   DEFAULT action=ALLOW

```
#### 仅允initramfs


```

   policy_name=Allow_Initramfs policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE boot_verified=TRUE action=ALLOW

```
#### 允许任何已签名且已验证的 dm-verity 卷以initramfs


```

   policy_name=Allow_Signed_DMV_And_Initramfs policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE boot_verified=TRUE action=ALLOW
   op=EXECUTE dmverity_signature=TRUE action=ALLOW

```
#### 禁止从特定的 dm-verity 卷执

```

   policy_name=Deny_DMV_By_Roothash policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE dmverity_roothash=sha256:cd2c5bae7c6c579edaae4353049d58eb5f2e8be0244bf05345bc8e5ed257baff action=DENY

   op=EXECUTE boot_verified=TRUE action=ALLOW
   op=EXECUTE dmverity_signature=TRUE action=ALLOW

```
#### 仅允许特定的 dm-verity 

```

   policy_name=Allow_DMV_By_Roothash policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE dmverity_roothash=sha256:401fcec5944823ae12f62726e8184407a5fa9599783f030dec146938 action=ALLOW

```
#### 允许任何带有有效内建签名fs-verity 文件


```

   policy_name=Allow_Signed_And_Validated_FSVerity policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE fsverity_signature=TRUE action=ALLOW

```
#### 允许执行特定fs-verity 文件


```

   policy_name=ALLOW_FSV_By_Digest policy_version=0.0.0
   DEFAULT action=DENY

   op=EXECUTE fsverity_digest=sha256:fd88f2b8824e197f850bf4c5109bea5cf0ee38104f710843bb72da796ba5af9e action=ALLOW

```
### 附加信息


- `Github Repository <https://github.com/microsoft/ipe>`_
- [IPE 的开发与设计文档 </security/ipe>](Developer and design docs for IPE </security/ipe>)

### 常见问题（FAQ

Q:
   与其它提供某种基于信任的访问控制LSM 相比，区别在哪里
A:

   一般而言，还有另外两LSM 能提供类似功能：IMA Loadpin
   IMA IPE 在功能上非常相似。两者之间的显著区别在于策略。[#devdoc]_

   Loadpin IPE 的差异相当大，因Loadpin 只覆IPE 的内核读取操作，IPE
   能够在内核读取之上控制执行。信任模型也不同；Loadpin 将其信任根植于初始超级块
   （super-block），IPE 的信任源自内核自身（通过 `SYSTEM_TRUSTED_KEYS`）
-----------


             this topic.

                      the Linux crypto API; IPE does not impose any
                      restrictions on the digest algorithm itself;
                      thus, this list may be out of date.

                     kernel's fsverity support; IPE does not impose any
                     restrictions on the digest algorithm itself;
                     thus, this list may be out of date.
