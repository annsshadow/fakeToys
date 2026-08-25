## 内核驱动 scx200_acb


作者：Christer Weinigel <wingel@nano-system.com>

该驱动取代较旧且从未合入的名i2c-nscacb 的驱动
### 模块参数


- base：最4 int
  SCx200 SC1100 设备ACCESS.bus 控制器的基地址

  默认情况下，驱动使用两个基地址 0x820 0x840。如果只想要一个基地址  请将第二个指定为 0 以覆盖此默认值
### 描述


启用 Geode SCx200 SC1100 处理器以CS5535 CS5536 Geode 配套设备上的
ACCESS.bus 控制器
### 设备特定说明


已知 SC1100 WRAP 板使用基地址 0x810 0x820。如scx200_acb 驱动内置
到内核中，请添加以下内容
```

  scx200_acb.base=0x810,0x820

```
如果 scx200_acb 驱动作为模块构建，请向以下文件添加该```

  options scx200_acb base=0x810,0x820

```
