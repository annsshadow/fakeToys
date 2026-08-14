
## arm64 上的 vCPU 特性选择


KVM/arm64 提供两种机制，允许用户空间配置呈现给客户机的 CPU 特性。

## KVM_ARM_VCPU_INIT


`KVM_ARM_VCPU_INIT` ioctl 接受一个特性标志位图
**（``struct kvm_vcpu_init**
: features``）。通过该接口启用的特性是
**opt-in**（选择性加入）的，并可能改变/扩展 UAPI。有关该 ioctl 所控制的特性的完整
文档，请参见 KVM_ARM_VCPU_INIT。

除此之外，KVM 支持的所有 CPU 特性都由架构化的 ID 寄存器描述。

## ID 寄存器


Arm 架构规定了一系列 **ID 寄存器**，用于描述 CPU 实现所支持的架构特性集合。KVM 将客户机的 ID 寄存器初始化为系统所支持的最大 CPU 特性集合。ID 寄存器的值在 KVM 中可以是 VM 作用域（VM-scoped）的，这意味着这些值可以在一个 VM 的所有 vCPU 之间共享。

KVM 允许用户空间通过 `KVM_SET_ONE_REG` ioctl 向 ID 寄存器写入值，从而 **opt-out**（选择退出）某些由 ID 寄存器描述的 CPU 特性。ID 寄存器在 VM 启动之前是可变的，即用户空间已对该 VM 中至少一个 vCPU 调用了 `KVM_RUN`。用户空间可以使用 `KVM_ARM_GET_REG_WRITABLE_MASKS` 来发现 ID 寄存器中哪些字段是可变的。更多细节请参见 ioctl 文档 <KVM_ARM_GET_REG_WRITABLE_MASKS>。

用户空间被允许根据架构在 DDI0487J.a D19.1.3 "ID 寄存器中字段的 ID 方案原则（Principles of the ID scheme for fields in ID register）"中规定的规则来 **限制** 或 **屏蔽** CPU 特性。KVM 不允许超出系统能力的 ID 寄存器值。

   强烈建议用户空间在访问 vCPU 其余 CPU 寄存器状态之前修改 ID 寄存器的值。KVM 可能会利用 ID 寄存器的值来控制特性模拟。将 ID 寄存器修改与其他系统寄存器访问交错进行可能导致不可预测的行为。
