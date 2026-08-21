## 系统状态变

一些用户非常不愿意重启系统。这就带来了提供更多livepatch（热补丁）并在它们之维护一定兼容性的需求
使用累积式（cumulative）livepatch 来维护更livepatch 要容易得多。每个新livepatch 会完全替换任何更旧的 livepatch。它可以保留、添加、甚至移除修复。由原子替换（atomic replace）特性，用任意一个版本的 livepatch 替换另一个版本通常
都是安全的
问题可能出在影子变量（shadow variables）和回调（callbacks）上。它们可能改变系的行为或状态，以至于回退并使用更旧的 livepatch 或原始内核代码不再安全。此外，
任何新的 livepatch 必须能够检测到已安装的 livepatch 已经做了哪些变更
这正livepatch 系统状态跟踪发挥作用的地方。它允许
  - 存储用于操作和恢复系统状态所需的数
  - 使用 change id version 定义 livepatch 之间的兼容

## 1. Livepatch 系统状API


系统状态可能被多个 livepatch 回调或新使用的代码修改。同时必须能够找到已由已安装
livepatch 完成的变更
每个被修改的状态由 struct klp_state 描述，参include/linux/livepatch.h
每个 livepatch 定义了一struct klp_states 数组。它们列出了livepatch 修改所有状态
livepatch 作者必须为每个 struct klp_state 定义以下两个字段
  - **id**

    - 用于标识受影响的系统状态的非零数字
  - **version**

    - 描述由给livepatch 支持的系统状态变更变体的数字
可以通过两个函数操作系统状态：

  - klp_get_state()

    - 获取与给livepatch state id 关联struct klp_state
  - klp_get_prev_state()

    - 获取与给feature id 以及已安livepatch 关联struct klp_state
## 2. Livepatch 兼容

系统状态版本用于防止加载不兼容livepatch。该检查在 livepatch 被启用时进行规则如下
  - 任何全新的系统状态修改都允许
  - 对于已被修改的系统状态，允许相同或更高版本的修改
  - 累积livepatch 必须处理来自已安livepatch 的所有系统状态修改
  - 非累积式 livepatch 允许触碰已被修改的系统状态
## 3. 支持的场

livepatch 有其生命周期，系统状态变更也是如此。每个兼容的 livepatch 都必须支以下场景
  - livepatch 被启用、且该状态尚未被正被替换livepatch 修改时，修改系统状态
  - 当变更已由正被替换的 livepatch 完成时，接管或更新系统状态修改
  - livepatch 被禁用时，恢复原始状态
  - 当转换（transition）被回退时，恢复先前的状态。它可能是原始系统状态，也可    是正被替换的 livepatch 所做的状态修改
  - 当发生错误且 livepatch 无法启用时，移除任何已做出的修改
## 4. 预期用法


系统状态通常livepatch 回调修改。每个回调的预期角色如下
**pre_patch()**

  - 在必要时分配 **state->data**。分配可能失败，**pre_patch()** 是唯一能够
    阻止 livepatch 加载的回调。当数据已由先前安装livepatch 提供时，不需    分配
  - 执行新代码在转换完成之前就需要做的任何其它准备工作。例如，初始    **state->data**
    系统状态本身通常**post_patch()** 中修改，那时整个系统能够处理它
  - 在出错时清理自身的烂摊子。这可以通过自定义代码完成，或显式调    **post_unpatch()**
**post_patch()**

  - 当它们兼容时，从先前livepatch 复制 **state->data**
  - 执行实际的系统状态修改。最终让新代码可以使用它
  - 确保 **state->data** 拥有所有必要的信息
  - 当不再需要时，从被替换的 livepatch 释放 **state->data**
**pre_unpatch()**

  - 阻止livepatch 添加、依赖系统状态变更的代码的运行
  - 回退系统状态修改
**post_unpatch()**

  - 通过检**klp_get_prev_state()** 来区分转换回退livepatch 禁用
  - 在转换回退的情况下，恢复先前的系统状态。这可能意味着什么都不做
  - 移除任何不再需要的设置或数据

   **pre_unpatch()** 通常执行**post_patch()** 对称的操作。不同之处在于它   livepatch 被禁用时调用。因此它无需关心任何先前安装livepatch
   **post_unpatch()** 通常执行**pre_patch()** 对称的操作。它也可能在转换回退
   期间被调用。因此它必须处理先前安装livepatch 的状态