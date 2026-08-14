
## Synopsys DesignWare PCIe 流量生成器（亦称 xData）驱动


支持的芯片：
Synopsys DesignWare PCIe 原型方案

数据手册：
不公开提供

作者：
Gustavo Pimentel <gustavo.pimentel@synopsys.com>

### 描述


该驱动应作为主机侧（Root Complex）驱动以及包含此 IP 的 Synopsys DesignWare
原型使用。

dw-xdata-pcie 驱动可用于启用/禁用任一方向（互斥）的 PCIe 流量生成器，并允许
进行 PCIe 链路性能分析。

与该驱动的交互通过模块参数完成，并可在运行时更改。驱动将请求的命令状态
信息输出到 `/var/log/kern.log` 或 dmesg。

### 示例


#### 写 TLPs 流量生成 - Root Complex 到 Endpoint 方向


```

 # echo 1 > /sys/class/misc/dw-xdata-pcie.0/write


```
```

 # cat /sys/class/misc/dw-xdata-pcie.0/write
 204


```
```

 # echo 0 > /sys/class/misc/dw-xdata-pcie.0/write


```
#### 读 TLPs 流量生成 - Endpoint 到 Root Complex 方向


```

 # echo 1 > /sys/class/misc/dw-xdata-pcie.0/read


```
```

 # cat /sys/class/misc/dw-xdata-pcie.0/read
 199


```
```

 # echo 0 > /sys/class/misc/dw-xdata-pcie.0/read


```
