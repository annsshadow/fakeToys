## 错误解码


### x86


AMD 系统上的错误解码应使用 rasdaemon 工具完成：
https://github.com/mchehab/rasdaemon/

当该守护进程运行时，它会自动记录并解码错误。否则，仍可通过提供以下参数来
解码此类错误：

```

        $ rasdaemon -p --status <STATUS> --ipid <IPID> --smca

```
此外，用户也可以传入特定的 family 和 model 来解码错误：

```

        $ rasdaemon -p --status <STATUS> --ipid <IPID> --smca --family <CPU Family> --model <CPU Model> --bank <BANK_NUM>

```
