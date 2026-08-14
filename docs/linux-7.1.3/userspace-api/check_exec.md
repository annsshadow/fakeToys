
## 可执行性检查


`AT_EXECVE_CHECK` `execveat(2)` 标志，以及 `SECBIT_EXEC_RESTRICT_FILE` 和
`SECBIT_EXEC_DENY_INTERACTIVE` 安全位，用于脚本解释器和动态链接器，以强制执行
由内核处理的、一致的执行安全策略。参见 `samples/check-exec/inc.c`_ 示例。

解释器是否应检查这些安全位，取决于运行恶意脚本相对于执行环境的安全风险，以及
内核能否检查脚本是否可信。例如，在服务器上运行的 Python 脚本可以使用任意系统调用
并访问任意文件。此类解释器应当被改进以使用这些安全位，并让用户定义其安全策略。
然而，在 Web 浏览器中运行的 JavaScript 引擎应当已经被沙箱化，因此不应能够危害
用户的环境。

为定制化执行环境（例如加固的 Linux 发行版或密封容器镜像）构建的脚本解释器或
动态链接器，可以在不检查相关安全位的情况下使用 `AT_EXECVE_CHECK`，前提是
向后兼容性由其它机制处理（例如原子更新确保允许执行所有合法的库）。因此建议
脚本解释器和动态链接器默认在运行时检查安全位，同时也提供让自定义构建表现得
如同 `SECBIT_EXEC_RESTRICT_FILE` 或 `SECBIT_EXEC_DENY_INTERACTIVE` 始终被
设为 1（即始终强制执行限制）的能力。

## AT_EXECVE_CHECK


将 `AT_EXECVE_CHECK` 标志传给 `execveat(2)` 仅对常规文件执行检查，如果允许
执行该文件则返回 0，忽略文件格式及其相关的解释器依赖（例如 ELF 库、脚本的
shebang）。

程序应始终执行此检查，以对不由内核直接执行、而是传递给用户空间解释器的文件应用
内核级检查。从解释器的角度看，所有包含可执行代码的文件都应被检查。但是，此检查的
结果应仅根据 `SECBIT_EXEC_RESTRICT_FILE` 或 `SECBIT_EXEC_DENY_INTERACTIVE` 来
强制执行。

此标志的主要目的是改善执行环境的安全性和一致性，确保直接文件执行（例如
`./script.sh`）和间接文件执行（例如 `sh script.sh`）得到相同的结果。例如，
这可用于根据调用方的环境检查文件是否可信。

在安全环境中，库和任何可执行依赖也应被检查。例如，动态链接应确保允许执行所有
库，以避免简单的绕过（例如使用 `LD_PRELOAD`）。要使这种安全执行环境有意义，
只有受信任的代码才能被执行，这还需要完整性保证。

为避免导致检查时间与使用时间（TOCTOU）问题的竞态条件，`AT_EXECVE_CHECK` 应与
`AT_EMPTY_PATH` 配合使用，以针对文件描述符而非路径进行检查。

## SECBIT_EXEC_RESTRICT_FILE 与 SECBIT_EXEC_DENY_INTERACTIVE


当设置了 `SECBIT_EXEC_RESTRICT_FILE` 时，进程应仅在针对相关文件描述符并使用
`AT_EXECVE_CHECK` 标志的 `execveat(2)` 调用成功时才解释或执行文件。

此安全位可由用户会话管理器、服务管理器、容器运行时、沙箱工具等设置……除测试
环境外，还应设置相关的 `SECBIT_EXEC_RESTRICT_FILE_LOCKED` 位。

程序应仅根据安全位强制执行一致的限制，而不依赖任何其它用户控制的配置。实际上，
这些安全位的用例是仅信任由系统配置（通过内核）审查过的可执行代码，因此我们应
小心，不要让不受信任的用户控制此配置。

不过，只要不是用于禁用安全位检查的方式，脚本解释器仍可使用用户配置（如环境变量）。
例如，`PATH` 和 `LD_PRELOAD` 变量可由脚本的调用方设置。更改这些变量可能导致
非预期代码执行，但只会来自经审查的可执行程序，这是可以接受的。为了使这有意义，
系统应提供一致的安全策略，以避免任意代码执行，例如通过强制执行写异或执行策略。

当设置了 `SECBIT_EXEC_DENY_INTERACTIVE` 时，进程绝不应解释交互式用户命令
（例如脚本）。但是，如果此类命令通过文件描述符（例如 stdin）传入，且针对相关
文件描述符并使用 `AT_EXECVE_CHECK` 标志的 `execveat(2)` 调用成功，则应解释
其内容。

例如，以脚本片段作为参数的脚本解释器在设置了 `SECBIT_EXEC_DENY_INTERACTIVE`
时应始终拒绝此类执行。

此安全位可由用户会话管理器、服务管理器、容器运行时、沙箱工具等设置……除测试
环境外，还应设置相关的 `SECBIT_EXEC_DENY_INTERACTIVE_LOCKED` 位。

以下是脚本解释器根据任意执行安全位组合的预期行为：

1. `SECBIT_EXEC_RESTRICT_FILE=0` 且 `SECBIT_EXEC_DENY_INTERACTIVE=0`

   始终解释脚本，并允许任意用户命令（默认）。

   没有威胁，每个人和所有东西都被信任，但我们可以通过始终执行但被脚本解释器忽略的
   `execveat(2)` 加 `AT_EXECVE_CHECK` 调用来防范潜在问题。实际上，此检查对于
   让系统管理员能够验证请求（例如通过审计）并为迁移到安全模式做准备仍然很重要。

2. `SECBIT_EXEC_RESTRICT_FILE=1` 且 `SECBIT_EXEC_DENY_INTERACTIVE=0`

   如果脚本不可执行则拒绝解释，但允许任意用户命令。

   威胁是受信任（且未被欺骗）用户运行的（潜在）恶意脚本。这可以防止非预期的脚本
   执行（例如 ``sh /tmp/*.sh``）。这对于（半受限的）用户会话有意义。

3. `SECBIT_EXEC_RESTRICT_FILE=0` 且 `SECBIT_EXEC_DENY_INTERACTIVE=1`

   始终解释脚本，但拒绝任意用户命令。

   此用例可能对安全服务（即没有交互式用户会话）有用，其中脚本的完整性已验证
   （例如使用 IMA/EVM 或 dm-verity/IPE），但访问权限可能尚未就绪。实际上，任意
   交互式命令会难检查得多。

4. `SECBIT_EXEC_RESTRICT_FILE=1` 且 `SECBIT_EXEC_DENY_INTERACTIVE=1`

   如果脚本不可执行则拒绝解释，并且也拒绝任何任意用户命令。

   威胁是未受信任用户（但受信任代码）运行的恶意脚本。这对于只能执行受信任脚本的
   系统服务有意义。

   https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/samples/check-exec/inc.c
