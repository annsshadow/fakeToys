
## TDX Guest API 文档


## 1. 概述


TDX guest 驱动通过 /dev/tdx-guest 这个 misc 设备暴露 IOCTL 接口，以允许用户空间获取某些 TDX guest 特有的细节。

## 2. API 说明


本节针对每个受支持的 IOCTL，提供以下信息以及通用说明。

:Input parameters: 传给 IOCTL 的参数及相关细节。
:Output: 关于输出数据和返回值的细节（含非常见错误值的说明）。

### 2.1 TDX_CMD_GET_REPORT0


:Input parameters: struct tdx_report_req
:Output: 成功执行后，TDREPORT 数据被复制到 tdx_report_req.tdreport 并返回 0。对于无效操作数返回 -EINVAL，TDCALL 失败时返回 -EIO，其他常见失败时返回标准错误号。

TDX_CMD_GET_REPORT0 IOCTL 可被证明（attestation）软件用来通过 TDCALL[TDG.MR.REPORT] 从 TDX module 获取 TDREPORT0（即 TDREPORT subtype 0）。

该 IOCTL CMD 的末尾添加了一个 subtype 索引，用以唯一标识特定 subtype 的 TDREPORT 请求。尽管 subtype 选项在 TDX Module v1.0 规范中标题为“TDG.MR.REPORT”的小节里被提及，但目前并未使用，且要求该值为 0。为了使 IOCTL 实现保持简单，subtype 选项没有被纳入输入 ABI。不过未来若 TDX Module 支持多个 subtype，将会创建一个新的 IOCTL CMD 来处理。为了保持 IOCTL 命名一致，subtype 索引作为 IOCTL CMD 的一部分被加入。

### 参考


TDX 参考资料汇总于此：

https://www.intel.com/content/www/us/en/developer/articles/technical/intel-trust-domain-extensions.html

该驱动基于 TDX module 规范 v1.0 与 TDX GHCI 规范 v1.0。
