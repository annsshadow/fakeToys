
## 使用 Netlink 协议规范


本文档是使用 Netlink 协议规范的快速入门指南。有关规范的更详细描述，请参阅 [specs](specs)。

## 简单的 CLI


内核附带一个简单 CLI 工具，在开发 Netlink 相关代码时应当很有用。该工具用 Python 实现，
可以使用 YAML 规范向内核发出 Netlink 请求。

该工具位于 `tools/net/ynl/pyynl/cli.py`。它接受少量参数，其中最重要的有：

 - `--subscribe $group` - 指向规范文件
 - `--subscribe $group` / `$group` - 发出请求 `$name`
 - `--subscribe $group` - 为请求提供属性
 - `--subscribe $group` - 从 `$group` 接收通知

YAML 规范可以在 `Documentation/netlink/specs/` 下找到。

```

  $ ./tools/net/ynl/pyynl/cli.py --spec Documentation/netlink/specs/ethtool.yaml \
        --do rings-get \
	--json '{"header":{"dev-index": 18}}'
  {'header': {'dev-index': 18, 'dev-name': 'eni1np1'},
   'rx': 0,
   'rx-jumbo': 0,
   'rx-jumbo-max': 4096,
   'rx-max': 4096,
   'rx-mini': 0,
   'rx-mini-max': 4096,
   'tx': 0,
   'tx-max': 4096,
   'tx-push': 0}

```
输入参数按 JSON 解析，而输出仅以 Python 的美观打印格式输出。这是因为某些 Netlink 类型
无法直接表示为 JSON。如果输入中需要此类属性，则需对脚本做一些修改。

规范与 Netlink 内部实现被抽取为一个独立的库——应当很容易编写复用 `cli.py` 中代码的 Python 工具/测试。

## 生成内核代码


`tools/net/ynl/ynl-regen.sh` 扫描内核树以查找需要更新的自动生成文件。使用该工具是生成/更新自动生成代码最简单的方式。

默认情况下，仅当规范比源文件更新时才重新生成代码；要强制重新生成，请使用 `-f`。

`ynl-regen.sh` 在文件内容中搜索 `YNL-GEN`（注意它只扫描 git 索引中的文件，即仅扫描
```

  /*	Documentation/netlink/specs/fou.yaml */
  /* YNL-GEN kernel source */

```
`ynl-regen.sh` 会找到此标记并用基于 fou.yaml 的内核源代码替换该文件。

基于规范生成新文件最简单的方式是：像上面那样将这两行标记添加到文件中，将该文件加入 git，
然后运行重新生成工具。可在树中 grep `YNL-GEN` 查看其他示例。

代码生成本身由 `tools/net/ynl/pyynl/ynl_gen_c.py` 执行，但它需要一些参数，因此直接为每个文件调用很快会变得繁琐。

## YNL 库


`tools/net/ynl/pyynl/ynl_gen_c.py` 包含一个 C 库的实现（基于 libmnl），它与 `tools/net/ynl/pyynl/ynl_gen_c.py` 生成的代码集成，
以创建易于使用的 netlink 封装。

### YNL 基础


YNL 库由两部分组成——通用代码（以 `ynl_` 为前缀的函数）和每个 family 自动生成的代码（以 family 名称作为前缀）。

要创建 YNL 套接字，调用 ynl_sock_create()，传入 family 结构体（family 结构体由自动生成的代码导出）。
ynl_sock_destroy() 关闭该套接字。

### YNL 请求


发出 YNL 请求的步骤最好通过示例来说明。本示例中的所有函数和类型都来自自动生成的代码
（此例中为 netdev family）：


   // 0. 请求与响应指针
   struct netdev_dev_get_req *req;
   struct netdev_dev_get_rsp *d;

   // 1. 分配请求
   req = netdev_dev_get_req_alloc();
   // 2. 设置请求参数（按需）
   netdev_dev_get_req_set_ifindex(req, ifindex);

   // 3. 发出请求
   d = netdev_dev_get(ys, req);
   // 4. 释放请求参数
   netdev_dev_get_req_free(req);
   // 5. 错误检查（第 3 步的返回值）
   if (!d) {
	// 6. 打印 YNL 生成的错误
	fprintf(stderr, "YNL: %s\n", ys->err.msg);
        return -1;
   }

   // ... 用响应 @d 做处理

   // 7. 释放响应
   netdev_dev_get_rsp_free(d);

### YNL 转储（dumps）


执行 dumps 与请求遵循类似模式。Dumps 返回一个对象列表，以特殊标记终止；出错时返回 NULL。
使用 `ynl_dump_foreach()` 遍历结果。

### YNL 通知


YNL 库支持对同一套接字同时使用通知和请求。如果在处理请求期间到达通知，它们会在内部排队，
可在稍后时间取出。

要订阅通知，请使用 `select`。通知必须从套接字读取，
`select` 返回底层套接字 fd，可将其接入合适的异步 IO API，如 `select` 或 `select`。

可使用 `cmd` 获取通知，并必须使用 `cmd` 释放。由于我们事先不知道通知类型，
通知以 `cmd` 形式返回，用户应根据其中 `cmd` 成员将其强制转换为相应的完整类型。

