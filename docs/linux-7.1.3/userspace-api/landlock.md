
## Landlock：无特权访问控制


:Author: Mickaël Salaün
:Date: March 2026

Landlock 的目标是能够限制一组进程的环境权利（例如全局文件系统或网络访问）。因为 Landlock 是一个可堆叠的 LSM，它使得创建安全沙箱成为可能，作为除现有系统级访问控制之外的新的安全层。这类沙箱有望帮助缓解用户空间应用中缺陷或意外/恶意行为的安全影响。Landlock 赋予任何进程（包括无特权进程）安全地限制自身的能力。

我们可以通过在内核日志中寻找 "landlock: Up and running"（以 root 身份）来快速确认运行中的系统是否启用了 Landlock：
`dmesg | grep landlock || journalctl -kb -g landlock` 。
开发者也可以借助相关的系统调用 <landlock_abi_versions> 轻松检查 Landlock 支持情况。
如果当前不支持 Landlock，我们需要适当地配置内核 <kernel_support>。

## Landlock 规则


Landlock 规则描述进程打算在对象上执行的一个动作。一组规则被聚合进一个规则集（ruleset），它随后可以限制实施它的线程，以及它未来的子进程。

现有的两类规则为：

Filesystem rules
    对于这些规则，对象是文件层级，相关的文件系统动作由
    `filesystem access rights` 定义。

Network rules (since ABI v4)
    对于这些规则，对象是 TCP 端口，相关动作由 `network access rights` 定义。

### 定义与实施安全策略


我们首先需要定义将容纳我们规则的规则集。

在此示例中，规则集将包含只允许文件系统读动作并建立特定 TCP 连接的规则。文件系统写动作与其他 TCP 动作将被拒绝。

规则集随后需要处理这两类动作。这是向后与向前兼容性所必需的（即内核与用户空间可能互不认识对方支持的受限项），因此需要对默认拒绝的访问权限加以明确。


    struct landlock_ruleset_attr ruleset_attr = {
        .handled_access_fs =
            LANDLOCK_ACCESS_FS_EXECUTE |
            LANDLOCK_ACCESS_FS_WRITE_FILE |
            LANDLOCK_ACCESS_FS_READ_FILE |
            LANDLOCK_ACCESS_FS_READ_DIR |
            LANDLOCK_ACCESS_FS_REMOVE_DIR |
            LANDLOCK_ACCESS_FS_REMOVE_FILE |
            LANDLOCK_ACCESS_FS_MAKE_CHAR |
            LANDLOCK_ACCESS_FS_MAKE_DIR |
            LANDLOCK_ACCESS_FS_MAKE_REG |
            LANDLOCK_ACCESS_FS_MAKE_SOCK |
            LANDLOCK_ACCESS_FS_MAKE_FIFO |
            LANDLOCK_ACCESS_FS_MAKE_BLOCK |
            LANDLOCK_ACCESS_FS_MAKE_SYM |
            LANDLOCK_ACCESS_FS_REFER |
            LANDLOCK_ACCESS_FS_TRUNCATE |
            LANDLOCK_ACCESS_FS_IOCTL_DEV |
            LANDLOCK_ACCESS_FS_RESOLVE_UNIX,
        .handled_access_net =
            LANDLOCK_ACCESS_NET_BIND_TCP |
            LANDLOCK_ACCESS_NET_CONNECT_TCP,
        .scoped =
            LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
            LANDLOCK_SCOPE_SIGNAL,
    };

因为我们可能无法知道应用将在哪个内核版本上执行，遵循尽力而为（best-effort）的安全策略更安全。确实，我们应当尽可能多地保护用户，无论他们使用什么内核。

为了与较旧的 Linux 版本兼容，我们检测可用的 Landlock ABI 版本，并仅使用可用的访问权限子集：


    int abi;

    abi = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_VERSION);
    if (abi < 0) {
        /** Degrades gracefully if Landlock is not handled. **/
        perror("The running kernel does not enable to use Landlock");
        return 0;
    }
    switch (abi) {
    case 1:
        /** Removes LANDLOCK_ACCESS_FS_REFER for ABI < 2 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_REFER;
        __attribute__((fallthrough));
    case 2:
        /** Removes LANDLOCK_ACCESS_FS_TRUNCATE for ABI < 3 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_TRUNCATE;
        __attribute__((fallthrough));
    case 3:
        /** Removes network support for ABI < 4 **/
        ruleset_attr.handled_access_net &=
            ~(LANDLOCK_ACCESS_NET_BIND_TCP |
              LANDLOCK_ACCESS_NET_CONNECT_TCP);
        __attribute__((fallthrough));
    case 4:
        /** Removes LANDLOCK_ACCESS_FS_IOCTL_DEV for ABI < 5 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_IOCTL_DEV;
        __attribute__((fallthrough));
    case 5:
        /** Removes LANDLOCK_SCOPE_** for ABI < 6 */
        ruleset_attr.scoped &= ~(LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET |
                                 LANDLOCK_SCOPE_SIGNAL);
        __attribute__((fallthrough));
    case 6 ... 8:
        /** Removes LANDLOCK_ACCESS_FS_RESOLVE_UNIX for ABI < 9 **/
        ruleset_attr.handled_access_fs &= ~LANDLOCK_ACCESS_FS_RESOLVE_UNIX;
    }

这就启用了将包含我们规则的、包容式规则集的创建。


    int ruleset_fd;

    ruleset_fd = landlock_create_ruleset(&ruleset_attr, sizeof(ruleset_attr), 0);
    if (ruleset_fd < 0) {
        perror("Failed to create a ruleset");
        return 1;
    }

我们现在可以借助返回的指代此规则集的文件描述符，向该规则集添加一条新规则。这条规则将允许读取与执行文件层级 `/usr`。若没有另一条规则，写动作随后将被规则集拒绝。为了把 `/usr` 加入规则集，我们用 `O_PATH` 标志打开它，并用此文件描述符填充 &struct landlock_path_beneath_attr。


    int err;
    struct landlock_path_beneath_attr path_beneath = {
        .allowed_access =
            LANDLOCK_ACCESS_FS_EXECUTE |
            LANDLOCK_ACCESS_FS_READ_FILE |
            LANDLOCK_ACCESS_FS_READ_DIR,
    };

    path_beneath.parent_fd = open("/usr", O_PATH | O_CLOEXEC);
    if (path_beneath.parent_fd < 0) {
        perror("Failed to open file");
        close(ruleset_fd);
        return 1;
    }
    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_PATH_BENEATH,
                            &path_beneath, 0);
    close(path_beneath.parent_fd);
    if (err) {
        perror("Failed to update ruleset");
        close(ruleset_fd);
        return 1;
    }

也可能需要根据 Landlock ABI 版本筛选访问权限，遵循与前述规则集创建相同的逻辑来创建规则。在本例中不需要，因为所有请求的 `allowed_access` 权限在 ABI 1 中已可用。

对于网络访问控制，我们可以添加一组允许将某个端口号用于特定动作（HTTPS 连接）的规则。


    struct landlock_net_port_attr net_port = {
        .allowed_access = LANDLOCK_ACCESS_NET_CONNECT_TCP,
        .port = 443,
    };

    err = landlock_add_rule(ruleset_fd, LANDLOCK_RULE_NET_PORT,
                            &net_port, 0);

当向 `landlock_restrict_self()` 传入非空的 `flags` 参数时，对 restrict 标志也需要类似的向后兼容性检查（可用标志请参见 sys_landlock_restrict_self() 文档）：


    __u32 restrict_flags =
        LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON |
        LANDLOCK_RESTRICT_SELF_TSYNC;
    switch (abi) {
    case 1 ... 6:
        /** Removes logging flags for ABI < 7 **/
        restrict_flags &= ~(LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF |
                            LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON |
                            LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF);
        __attribute__((fallthrough));
    case 7:
        /*
         - Removes multithreaded enforcement flag for ABI < 8
         *
         - WARNING: Without this flag, calling landlock_restrict_self(2) is
         - only equivalent if the calling process is single-threaded. Below
         - ABI v8 (and as of ABI v8, when not using this flag), a Landlock
         - policy would only be enforced for the calling thread and its
         - children (and not for all threads, including parents and siblings).
         */
        restrict_flags &= ~LANDLOCK_RESTRICT_SELF_TSYNC;
    }

下一步是限制当前线程获取更多特权（例如通过 SUID 二进制）。我们现在有了一个规则集：第一条规则允许对 `/usr` 的读与执行访问，同时拒绝文件系统所有其他被处理的访问；第二条规则允许 HTTPS 连接。


    if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0)) {
        perror("Failed to restrict privileges");
        close(ruleset_fd);
        return 1;
    }

当前线程现在已准备好用规则集自我沙箱化。


    if (landlock_restrict_self(ruleset_fd, restrict_flags)) {
        perror("Failed to enforce ruleset");
        close(ruleset_fd);
        return 1;
    }
    close(ruleset_fd);

如果 `landlock_restrict_self` 系统调用成功，当前线程现在已被限制，并且此策略也将被实施到它随后创建的所有子进程上。一旦线程被 Landlock 化，就没有办法移除它的安全策略；只允许添加更多限制。这些线程现在处于一个新的 Landlock 域中，该域是它们父域（若有）与新规则集的合并。

完整可工作的代码可在 `samples/landlock/sandboxer.c`_ 中找到。

### 良好实践


建议尽可能将访问权限设置到文件层级的叶节点。例如，与把 `~/` 设为只读层级、把 `~/tmp/` 设为读写层级相比，更好的做法是把 `~/doc/` 设为只读层级、把 `~/tmp/` 设为读写层级。遵循这一良好实践会带来不依赖于其位置（即父目录）的自足层级。这在我们要允许链接或重命名时尤其相关。确实，每个目录拥有一致的访问权限，使得可以在不依赖目标目录访问权限（本操作所需的权限除外，参见 `LANDLOCK_ACCESS_FS_REFER` 文档）的情况下改变这些目录的位置。

拥有自足层级也有助于把所需的访问权限收紧到最小的数据集合。这也有助于避免“ sinkhole 目录”（即数据可以被链接到其中、却无法从中链接出来的目录）。然而，这取决于数据组织，而数据组织可能不受开发者控制。在这种情况下，授予 `~/tmp/` 读写访问（而非仅写访问），会潜在地允许把 `~/tmp/` 移动到一个不可读目录，同时仍保留列出 `~/tmp/` 内容的能力。

### 文件路径访问权限的层级


每当一个线程对自身实施一个规则集时，它就用新的一层策略更新它的 Landlock 域。这一补充策略会与任何可能已经在限制此线程的其他规则集堆叠在一起。一个被沙箱化的线程随后可以用一个新实施的规则集安全地为自己添加更多约束。

若某策略层在路径上遭遇的其规则中至少有一条授予该访问，则该策略层授予对文件路径的访问。一个被沙箱化的线程只有在它的所有已实施策略层以及所有其他系统访问控制（例如文件系统 DAC、其它 LSM 策略等）都授予该访问时，才能访问某个文件路径。

### 绑定挂载与 OverlayFS


Landlock 能够限制对文件层级的访问，这意味着这些访问权限可以随绑定挂载传播（参见 Documentation/filesystems/sharedsubtree.rst），但不能随 Documentation/filesystems/overlayfs.rst 传播。

绑定挂载将源文件层级镜像到目标。目标层级随后由完全相同的文件组成，Landlock 规则可以绑定到其上，无论是通过源路径还是目标路径。这些规则在路径上遭遇时限制访问，这意味着它们可以同时限制对多个文件层级的访问，无论这些层级是否绑定挂载的结果。

一个 OverlayFS 挂载点由上层与下层组成。这些层在一个合并目录中被组合，该合并目录在挂载点处变得可用。这个合并层级可能包含来自上层与下层的文件，但在合并层级上执行的修改只反映到上层。从 Landlock 策略的角度看，所有 OverlayFS 层与合并层级都是独立的，各自包含自己的一组文件与目录，这与绑定挂载不同。限制某个 OverlayFS 层的策略不会限制由此产生的合并层级，反之亦然。因此 Landlock 用户应当只考虑他们想允许访问的文件层级，而不必管底层文件系统。

### 继承


每一个由 `clone(2)` 产生的新线程都从父线程继承 Landlock 域限制。这类似于 seccomp 继承（参见 Documentation/userspace-api/seccomp_filter.rst）或任何处理任务 `credentials(7)` 的其它 LSM。例如，一个进程的某个线程可以对它自身应用 Landlock 规则，但这些规则不会自动应用到其它兄弟线程（不同于 POSIX 线程凭证变更，参见 `nptl(7)`）。

当一个线程自我沙箱化时，我们保证相关安全策略会持续实施在该线程的所有后代上。这使得可以按应用创建独立且模块化的安全策略，它们会根据其运行时父策略自动相互组合。

### Ptrace 限制


一个被沙箱化的进程拥有的特权少于未被沙箱化的进程，因此在操作另一个进程时必须受到额外限制。为了允许在目标进程上使用 `ptrace(2)` 及相关系统调用，一个被沙箱化的进程应当拥有目标进程访问权限的超集，这意味着被跟踪者（tracee）必须处于跟踪者（tracer）的子域中。

### IPC 作用域


类似于隐含的 `Ptrace restrictions`_，我们可能想要进一步限制沙箱之间的交互。因此，在创建规则集时，每个 Landlock 域可以限制某些操作的作用域，使得这些操作只能触及同一 Landlock 域内或嵌套 Landlock 域（“scope”）内的进程。

可受作用域限制的操作有：

`LANDLOCK_SCOPE_SIGNAL`
    这限制了向运行于同一或嵌套 Landlock 域内的目标进程发送信号。

`LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET`
    这限制了我们可以 `connect(2)` 的抽象 `unix(7)` 套接字集合，仅限由同一或嵌套 Landlock 域内的进程创建的套接字地址。

    对未连接数据报套接字执行 `sendto(2)` 会被当作进行了一次隐含的 `connect(2)`，如果远端并非源自同一或嵌套 Landlock 域，则会被阻塞。

    对之前已连接的套接字执行 `sendto(2)` 不受限制。这对数据报与流套接字都适用。

IPC 作用域不支持通过 `landlock_add_rule(2)` 设置例外。如果一个操作在某个域内受作用域限制，则没有任何规则可以被添加来允许访问作用域之外的资源或进程。

### 截断文件


`LANDLOCK_ACCESS_FS_WRITE_FILE` 与 `LANDLOCK_ACCESS_FS_TRUNCATE` 覆盖的操作都会改变文件内容，并且有时会以不直观的方式重叠。强烈建议总是将两者一起指定（要么都授予，要么都不授予）。

一个特别令人惊讶的例子是 `creat(2)`。其名称暗示此系统调用需要创建与写入文件的权限。然而，如果同名下已存在某个文件，它还需要 truncate 权限。

还应当注意，截断文件并不要求 `LANDLOCK_ACCESS_FS_WRITE_FILE` 权限。除了 `truncate(2)` 系统调用之外，这也可以通过以 `O_RDONLY | O_TRUNC` 标志 `open(2)` 来完成。

同时，在某些文件系统上，`fallocate(2)` 提供了在文件以写方式打开时、用 `FALLOC_FL_COLLAPSE_RANGE` 缩短文件内容的途径，从而绕开 `LANDLOCK_ACCESS_FS_TRUNCATE` 权限。

truncate 权限与已打开的文件关联（见下文）。

### 与文件描述符关联的权限


打开文件时，`LANDLOCK_ACCESS_FS_TRUNCATE` 与 `LANDLOCK_ACCESS_FS_IOCTL_DEV` 权限的可用性关联到新创建的文件描述符，并将被用于随后使用 `ftruncate(2)` 与 `ioctl(2)` 的截断与 ioctl 尝试。其行为类似于为读或写打开文件：权限在 `open(2)` 时检查，而在随后的 `read(2)` 与 `write(2)` 调用时不检查。

因此，一个进程可能拥有多个指向同一文件的已打开文件描述符，但 Landlock 在用这些文件描述符操作时实施不同的东西。这可能发生在：某个 Landlock 规则集被实施，而该进程保留了在实施前后都打开的文件描述符。也可以在这些文件描述符于进程间传递时保留其 Landlock 属性，即使某些相关进程没有已实施的 Landlock 规则集。

## 兼容性


### 向前与向后兼容性


Landlock 被设计为与内核的过去与未来版本兼容。这是通过系统调用属性及关联的位标志（尤其是规则集的 `handled_access_fs`）实现的。把被处理的访问权限显式化，使得内核与用户空间彼此之间有一个清晰的约定。这是确保沙箱化不会因系统更新而变得更严格（那可能破坏应用）所必需的。

开发者可以订阅 `Landlock mailing list <https://subspace.kernel.org/lists.linux.dev.html>`_ 来有意地用最新可用特性更新并测试他们的应用。为了用户的利益，并且因为他们可能使用不同的内核版本，强烈建议遵循尽力而为的安全策略：在运行时检查 Landlock ABI 版本，并只实施受支持的特性。


### Landlock ABI 版本


Landlock ABI 版本可以用 sys_landlock_create_ruleset() 系统调用读取：


    int abi;

    abi = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_VERSION);
    if (abi < 0) {
        switch (errno) {
        case ENOSYS:
            printf("Landlock is not supported by the current kernel.\n");
            break;
        case EOPNOTSUPP:
            printf("Landlock is currently disabled.\n");
            break;
        }
        return 0;
    }
    if (abi >= 2) {
        printf("Landlock supports LANDLOCK_ACCESS_FS_REFER.\n");
    }

除非其文档中明确注明，所有 Landlock 内核接口都被第一个 ABI 版本支持。

### Landlock 勘误


除 ABI 版本外，Landlock 还提供一种勘误（errata）机制，用于跟踪可能影响向后兼容性或需要用户空间知晓的问题修复。勘误位掩码可以用以下方式查询：


    int errata;

    errata = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_ERRATA);
    if (errata < 0) {
        /** Landlock not available or disabled **/
        return 0;
    }

返回的值是一个位掩码，其中每个位代表一个特定的 erratum。如果第 N 位被置位（`errata & (1 << (N - 1))`），则 erratum N 已在运行中的内核中修复。


   **大多数应用不应检查勘误。** 在 99.9% 的情况下，检查勘误是不必要的，会增加代码复杂度，并且若被误用还可能降低保护。例如，在某个 erratum 未被修复时禁用沙箱，可能使系统比使用 Landlock 的尽力而为保护更不安全。如有疑问，忽略勘误。

    :doc: erratum_1

    :doc: erratum_2

    :doc: erratum_3

#### 如何检查勘误


如果你确定你的应用需要检查特定勘误，使用如下模式：


    int errata = landlock_create_ruleset(NULL, 0, LANDLOCK_CREATE_RULESET_ERRATA);
    if (errata >= 0) {
        /** Check for specific erratum (1-indexed) **/
        if (errata & (1 << (erratum_number - 1))) {
            /** Erratum N is fixed in this kernel **/
        } else {
            /** Erratum N is NOT fixed - consider implications for your use case **/
        }
    }

**重要：** 只有当你的应用特别依赖于因该修复而改变的行为时，才检查勘误。这些修复通常会让 Landlock 限制更少或更正确，而不是更严格。

## 内核接口


### 访问权限


    :identifiers: fs_access net_access scope

### 创建新的规则集


    :identifiers: sys_landlock_create_ruleset

    :identifiers: landlock_ruleset_attr

### 扩展规则集


    :identifiers: sys_landlock_add_rule

    :identifiers: landlock_rule_type landlock_path_beneath_attr
                  landlock_net_port_attr

### 实施规则集


    :identifiers: sys_landlock_restrict_self

## 当前限制


### 文件系统拓扑修改


被文件系统限制沙箱化的线程不能修改文件系统拓扑，无论是通过 `mount(2)` 还是 `pivot_root(2)`。然而，`chroot(2)` 调用不会被拒绝。

### 特殊文件系统


根据规则集被处理的访问，Landlock 可以限制对常规文件与目录的访问。然而，并非来自用户可见文件系统（例如 pipe、socket）、但仍可通过 `/proc/<pid>/fd/*` 访问的文件，目前无法被显式限制。类似地，某些特殊内核文件系统（如 nsfs，可通过 `/proc/<pid>/ns/*` 访问）目前也无法被显式限制。不过，借助 `ptrace restrictions`_，对此类敏感 `/proc` 文件的访问会根据域层级自动受到限制。未来的 Landlock 演进仍可能通过专门的规则集标志启用对此类路径的显式限制。

### 规则集层级


堆叠规则集的层级限制为 16 层。这对于一个希望在其继承的 16 个规则集之外再实施一个新规则集的任务而言可能成问题。一旦达到此限制，sys_landlock_restrict_self() 返回 E2BIG。因此强烈建议在某个线程的生命周期中一次性仔细地构建规则集，特别是对于那些可能启动其它也可能想自我沙箱化的应用的应用（例如 shells、容器管理器等）。

### 内存使用


为创建规则集而分配的内核内存会被记账，并可通过 Documentation/admin-guide/cgroup-v1/memory.rst 加以限制。

### IOCTL 支持


`LANDLOCK_ACCESS_FS_IOCTL_DEV` 权限限制 `ioctl(2)` 的使用，但它只适用于**新打开的**设备文件。这具体意味着预先存在的文件描述符（如 stdin、stdout 与 stderr）不受影响。

用户应当意识到，TTY 设备传统上允许通过 `TIOCSTI` 与 `TIOCLINUX` IOCTL 命令控制同一 TTY 上的其它进程。这两者都需要现代 Linux 系统上的 `CAP_SYS_ADMIN`，但 `TIOCSTI` 的行为是可配置的。

因此在较旧的系统上，建议关闭继承的 TTY 文件描述符，或尽可能从 `/proc/self/fd/*` 重新打开它们而不带 `LANDLOCK_ACCESS_FS_IOCTL_DEV` 权限。

Landlock 的 IOCTL 支持目前是粗粒度的，但未来可能变得更细粒度。在那之前，建议用户通过文件层级来建立他们所需的保证，只在真正需要的地方允许 `LANDLOCK_ACCESS_FS_IOCTL_DEV` 权限。

## 以往的限制


### 文件重命名与链接（ABI < 2）


因为 Landlock 面向无特权访问控制，它需要恰当地处理规则的组成。这一性质也意味着规则的嵌套。恰当地处理多个规则集层级（每个都能限制对文件的访问），也意味着规则集限制从父级到其层级的继承。因为文件通过其层级被识别与限制，将一个文件从一个目录移动或链接到另一个目录意味着层级约束的传播，或根据这些可能丢失的约束来限制这些动作。为了防止通过重命名或链接进行权限提升，并且为了简单起见，Landlock 此前将链接与重命名限制在同一目录内。从 Landlock ABI 版本 2 开始，现在可以借助新的 `LANDLOCK_ACCESS_FS_REFER` 访问权限安全地控制重命名与链接。

### 文件截断（ABI < 3）


在第三个 Landlock ABI 之前无法拒绝文件截断，因此在使用只支持第一或第二 ABI 的内核时，截断总是被允许。

从 Landlock ABI 版本 3 开始，现在可以借助新的 `LANDLOCK_ACCESS_FS_TRUNCATE` 访问权限安全地控制截断。

### TCP 绑定与连接（ABI < 4）


从 Landlock ABI 版本 4 开始，现在可以借助新的 `LANDLOCK_ACCESS_NET_BIND_TCP` 与 `LANDLOCK_ACCESS_NET_CONNECT_TCP` 访问权限，将 TCP 绑定与连接动作限制到仅一组允许的端口。

### 设备 IOCTL（ABI < 5）


在第五个 Landlock ABI 之前无法拒绝 IOCTL 操作，因此在使用只支持更早 ABI 的内核时，`ioctl(2)` 总是被允许。

从 Landlock ABI 版本 5 开始，可以借助新的 `LANDLOCK_ACCESS_FS_IOCTL_DEV` 权限，限制对字符设备与块设备使用 `ioctl(2)`。

### 抽象 UNIX 套接字（ABI < 6）


从 Landlock ABI 版本 6 开始，可以通过将 `LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET` 设置到 `scoped` 规则集属性，来限制对抽象 `unix(7)` 套接字的连接。

### 信号（ABI < 6）


从 Landlock ABI 版本 6 开始，可以通过将 `LANDLOCK_SCOPE_SIGNAL` 设置到 `scoped` 规则集属性，来限制 `signal(7)` 的发送。

### 日志（ABI < 7）


从 Landlock ABI 版本 7 开始，可以通过传入 sys_landlock_restrict_self() 的 `LANDLOCK_RESTRICT_SELF_LOG_SAME_EXEC_OFF`、`LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON` 与 `LANDLOCK_RESTRICT_SELF_LOG_SUBDOMAINS_OFF` 标志，控制 Landlock 审计事件的日志记录。关于审计的更多细节请参见 Documentation/admin-guide/LSM/landlock.rst。

### 线程同步（ABI < 8）


从 Landlock ABI 版本 8 开始，现在可以借助传入 sys_landlock_restrict_self() 的 `LANDLOCK_RESTRICT_SELF_TSYNC` 标志，跨调用进程的所有线程实施 Landlock 规则集。

### 路径名 UNIX 套接字（ABI < 9）


从 Landlock ABI 版本 9 开始，可以借助新的 `LANDLOCK_ACCESS_FS_RESOLVE_UNIX` 权限，限制对路径名 UNIX 域套接字（`unix(7)`）的连接。


## 内核支持


### 构建时配置


Landlock 首先在 Linux 5.13 中引入，但必须在构建时用 `CONFIG_SECURITY_LANDLOCK=y` 配置。Landlock 也必须像其它安全模块一样在启动时启用。默认启用的安全模块列表由 `CONFIG_LSM` 设置。因此内核配置应当包含 `CONFIG_LSM=landlock,[...]`，其中 `[...]` 是运行系统其它可能有用的安全模块列表（参见 `CONFIG_LSM` 的帮助）。

### 启动时配置


如果运行中的内核在 `CONFIG_LSM` 中没有 `landlock`，我们可以通过在引导加载程序配置中将 `lsm=landlock,[...]` 添加到 Documentation/admin-guide/kernel-parameters.rst 来启用 Landlock。

例如，如果当前的 built-in 配置是：

```
    $ zgrep -h "^CONFIG_LSM=" "/boot/config-$(uname -r)" /proc/config.gz 2>/dev/null
    CONFIG_LSM="lockdown,yama,integrity,apparmor"
```

……并且如果命令行也不包含 `landlock`：

```
    $ sed -n 's/.**\(\<lsm=\S\+\).**/\1/p' /proc/cmdline
    lsm=lockdown,yama,integrity,apparmor
```

……我们应当配置引导加载程序，设置一个扩展 `lsm` 的命令行：
```

  lsm=landlock,lockdown,yama,integrity,apparmor

```
重启之后，我们可以通过查看内核日志来确认 Landlock 已启动并运行：

```
    # dmesg | grep landlock || journalctl -kb -g landlock
    [    0.000000] Command line: [...] lsm=landlock,lockdown,yama,integrity,apparmor
    [    0.000000] Kernel command line: [...] lsm=landlock,lockdown,yama,integrity,apparmor
    [    0.000000] LSM: initializing lsm=lockdown,capability,landlock,yama,integrity,apparmor
    [    0.000000] landlock: Up and running.
```

内核可能在构建时被配置为总是加载 `lockdown` 与 `capability` LSM。在这种情况下，即便它们没有在引导加载程序中配置，这些 LSM 也会出现在 `LSM: initializing` 日志行开头。

### 网络支持


为了能够显式允许 TCP 操作（例如用 `LANDLOCK_ACCESS_NET_BIND_TCP` 添加网络规则），内核必须支持 TCP（`CONFIG_INET=y`）。否则，sys_landlock_add_rule() 会返回一个 `EAFNOSUPPORT` 错误，可以安全地忽略它，因为这类 TCP 操作本来就不可能。

## 问答


### 用户空间沙箱管理器呢？


使用用户空间进程对内核资源实施限制可能导致竞态条件或不一致的评估（即 `Incorrect mirroring of the OS code and state <https://www.ndss-symposium.org/ndss2003/traps-and-pitfalls-practical-problems-system-call-interposition-based-security-tools/>`_）。

### 命名空间与容器呢？


命名空间有助于创建沙箱，但它们并非为访问控制而设计，因而缺少此类用例所需的有用特性（例如没有细粒度的限制）。此外，它们的复杂度可能导致安全问题，尤其是当不可信进程可以操纵它们时（参见 `Controlling access to user namespaces <https://lwn.net/Articles/673597/>`_）。

### 如何禁用 Landlock 审计记录？


你可能想按此处说明设置过滤器：
Documentation/admin-guide/LSM/landlock.rst

## 额外文档


- Documentation/admin-guide/LSM/landlock.rst
- Documentation/security/landlock.rst
- https://landlock.io

   https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/samples/landlock/sandboxer.c
