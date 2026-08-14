
KVM 补丁审查清单



1. 补丁必须遵循 Documentation/process/coding-style.rst 与
    Documentation/process/submitting-patches.rst。


2. 补丁应基于 kvm.git 的 master 或 next 分支。


3. 如果补丁引入或修改了新的用户空间 API：
    - 该 API 必须在 Documentation/virt/kvm/api.rst 中有文档说明
    - 该 API 必须可通过 KVM_CHECK_EXTENSION 被发现


4. 新增状态必须包含对保存/恢复的支持。


5. 新特性默认必须关闭（由用户空间显式请求开启）。
    性能改进可以且应当默认开启。


6. 新的 CPU 特性应通过 KVM_GET_SUPPORTED_CPUID2 暴露，
    或非 x86 架构上的等价接口


7. 该特性应当是可测试的（见下文）。


8. 在可行的情况下，改动应保持厂商中立。改动公共代码
    要好于将改动复制到各厂商代码中。


9. 类似地，优先改动与架构无关的代码，而非与架构相关的代码。


10. 用户/内核接口以及客户机/宿主机接口必须做到 64 位干净
    （所有变量和大小在 64 位上自然对齐；仅使用特定类型，
    即 u64 而非 ulong）。


11. 新的客户机可见特性要么必须在硬件手册中有文档说明，
    要么必须附带文档。


KVM 代码的测试



所有贡献给 KVM 的特性，以及许多情况下的缺陷修复，都应伴随某种
开源客户机与 VMM 中的测试和/或启用代码。KVM 被多套测试套件覆盖：


**Selftests**
  这些是底层测试，可对内核 API 进行细粒度测试。
  这包括 API 失败场景、在特定客户机指令之后调用 API，
  以及在单个测试中多次调用 `KVM_CREATE_VM`。它们随内核树一同提供，
  位于 `tools/testing/selftests/kvm`。


`kvm-unit-tests`
  一组小型客户机集合，从客户机角度测试 CPU 与模拟设备特性。
  它们运行于 QEMU 或 `kvmtool` 之下，并且通常并非 KVM 专用：
  它们可以通过 QEMU 支持的任何加速器运行，甚至可以在裸机上运行，
  从而能够跨不同虚拟化管理程序与处理器族比较行为。


Functional test suites
  存在多种功能测试集，例如 QEMU 的 `tests/functional` 测试套件
  与 `avocado-vt <https://avocado-vt.readthedocs.io/en/latest/>`__。它们通常涉及在虚拟机中运行完整的操作系统。


最佳的测试方式取决于该特性的复杂度和运作方式。以下是一些示例与准则：


New instructions（无新寄存器或 API）
  相应的 CPU 特性（如适用）应在 QEMU 中可用。如果这些指令需要
  KVM 中的模拟支持或其他代码，则值得为 `kvm-unit-tests` 或 selftests
  增加覆盖；如果指令涉及的 API 已有良好的 selftest 覆盖，则后者
  可能是更好的选择。


New hardware features（新寄存器，无新 API）
  这些应通过 `kvm-unit-tests` 进行测试；这或多或少意味着在 QEMU 和/或
  `kvmtool` 中提供支持。某些情况下可以改用 selftests，类似于上一种情况，
  或专门用于测试客户机状态保存/恢复中的边界情况。


Bug fixes 与性能改进
  这些通常不会引入新 API，但值得分享任何能验证你贡献的基准测试与测试，
  最好以回归测试的形式。测试与基准可以包含在 `kvm-unit-tests` 或 selftests 中，
  具体取决于你改动的细节。Selftests 对回归测试尤其有用，因为它们
  直接包含在内核树中。


Large scale internal changes（大规模内部改动）
  虽然很难给出单一策略，但应确保改动后的代码被 `kvm-unit-tests` 或 selftests
  覆盖。某些情况下受影响的代码会为任意客户机运行，功能测试已足够。
  请在 cover letter 中说明你的测试过程，这有助于发现现有测试套件的不足。


New APIs
  展示你的使用场景很重要。它可以简单到说明该特性已在裸机上使用，
  也可以是一个用户空间的概念验证实现。后者不必是开源的，尽管开源
  显然更便于测试。Selftests 应当测试 API 的边界情况，并且如果没有
  开源 VMM 使用该特性，还应覆盖基本的宿主机与客户机操作。


Bigger features（通常横跨宿主机与客户机）
  这些应得到 Linux 客户机的支持，仅对可在 Windows 客户机上测试的
  Hyper-V 特性有有限例外。强烈建议该特性能够配合开源宿主机 VMM
  （例如 QEMU 或 crosvm 中至少一个）以及客户机固件使用。Selftests
  应至少测试 API 错误情况。客户机操作可通过 selftests 或 `kvm-unit-tests`
  覆盖（这对半虚拟化和仅 Windows 的特性尤为重要）。强大的 selftest
  覆盖也可以替代在开源 VMM 中的实现，但通常不推荐这样做。


遵循上述关于在 selftests 与 `kvm-unit-tests` 中进行测试的建议，将使维护者
更易于审查并接受你的代码。事实上，甚至在你向上游贡献改动之前，
它也会让你为 KVM 开发更加轻松。


当然，KVM 维护者保留要求更多测试的权利，尽管他们也可能不时免除该要求。

