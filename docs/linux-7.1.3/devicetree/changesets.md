## Devicetree 变更集


Deticetree 变更集（changeset）是一种允许对活动设备树应用修改的方法，
其保证要么完整应用全部修改，要么完全不应用。若在应用变更集的过程中
发生错误，设备树将回滚到先前的状态。已应用的变更集也可以被移除。

当应用一个变更集时，所有修改会在发出 OF_RECONFIG 通知之前一次性应用到
设备树上。这样，接收方在收到通知时看到的是设备树完整且一致的状态。

一个变更集的执行顺序如下：

1. of_changeset_init() —— 初始化一个变更集

2. 多次调用 DT 设备树修改函数，包括 of_changeset_attach_node()、
   of_changeset_detach_node()、of_changeset_add_property()、
   of_changeset_remove_property、of_changeset_update_property() 来
   准备一组修改。此阶段不会对活动设备树做任何修改。所有修改操作都
   记录在 of_changeset 的 'entries' 列表中。

3. of_changeset_apply() —— 将修改应用到设备树。要么整个变更集被应用，
   要么在出现错误时设备树恢复到先前的状态。核心通过加锁来保证正确的
   串行化。如需，可使用不加锁的版本 __of_changeset_apply。

如果已成功应用的变更集需要被移除，可使用 of_changeset_revert() 完成。
