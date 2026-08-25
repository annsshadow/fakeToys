
## 检查需要的翻译更新


该脚本用于跟踪不同语言环境下文档的翻译状态，即文档是否与英文版本保持同步

### 工作原理


它使`git log` 命令来追踪翻译提交（按作者日期排序）中最新英文提交，以及 HEAD 中最新英文提交。如果出现任何差异，该文件即被视为已过期，随后会收集并报告需要更新的提交

宸插疄鐜板姛鑳。

- 检查某个语言环境下的所有文
- 检查单个文件或一组文
- 提供更改输出格式的选项
- 跟踪尚无翻译的文件的翻译状

### 用法


```
   tools/docs/checktransupdate.py --help
```
具体的用法详情请参考参数解析器的输出

示例

- `tools/docs/checktransupdate.py -l zh_CN`
   这将打印 zh_CN 语言环境下所有需要更新的文件
- `tools/docs/checktransupdate.py Documentation/translations/zh_CN/dev-tools/testing-overview.rst`
   这将只打印指定文件的状态

其输出大致如下：

```
    Documentation/dev-tools/kfence.rst
    No translation in the locale of zh_CN

    Documentation/translations/zh_CN/dev-tools/testing-overview.rst
    commit 42fb9cfd5b18 ("Documentation: dev-tools: Add link to RV docs")
    1 commits needs resolving in total

```
待实现功

- 文件可以是一个文件夹而不仅仅是一个文
