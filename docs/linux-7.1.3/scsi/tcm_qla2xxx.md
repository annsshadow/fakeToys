## tcm_qla2xxx 驱动说明


### tcm_qla2xxx jam_host 属性

现在新增了一个名为 jam_host 的模块端点属性
```

	jam_host: boolean=0/1

```
该属性及配套代码仅在将 Kconfig 参数 TCM_QLA2XXX_DEBUG 设为 Y 时才被包含。

默认情况下该干扰（jammer）代码和功能是禁用的。

使用该属性可以控制对发往所选主机的 SCSI 命令的丢弃。

这对于测试错误处理、模拟缓慢排空（slow drain）以及其他 fabrics 问题可能有用。

将某个主机的 jam_host 属性设为布尔值 1，将丢弃发往该主机的命令。

重置回 0 以停止干扰。

```
  echo 1 > /sys/kernel/config/target/qla2xxx/21:00:00:24:ff:27:8f:ae/tpgt_1/attrib/jam_host

```
```
  echo 0 > /sys/kernel/config/target/qla2xxx/21:00:00:24:ff:27:8f:ae/tpgt_1/attrib/jam_host

```
