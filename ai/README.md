# AI 训练数据增强工具

用于生成房产客服问答场景的 AI 模型训练数据，基于百度 ERNIE API 和 NLP 增强技术，将少量种子数据扩充为多轮问答对。

## 数据格式

训练数据为 JSON 数组，每条记录包含以下字段：

```json
{
  "input": "",
  "instruction": "用户提问内容",
  "output": "客服回答内容"
}
```

## 文件说明

| 文件 | 说明 |
|------|------|
| `train_data.json` | 原始种子数据集，约 200+ 条问答对 |
| `train_data_final02.json` | 多线程增强后的主数据集，约 2.8 万+ 条 |
| `train_data_final01.json` | 早期中间版本，约 5,622 条 |
| `enhanceTXT.py` | 核心数据增强脚本 |
| `bak/` | 备份目录，包含中间格式和断点续传记录 |

## 环境要求

- Python 3.x
- 依赖包：`nlpaug`、`nlpcda`、`requests`

## 环境变量

运行前需设置百度云鉴权信息：

```bash
export BAIDU_API_KEY=<your_api_key>
export BAIDU_SECRET_KEY=<your_secret_key>
```

Windows：

```cmd
set BAIDU_API_KEY=<your_api_key>
set BAIDU_SECRET_KEY=<your_secret_key>
```

## 使用方法

### 单线程模式

```bash
python enhanceTXT.py
```

### 多线程模式

脚本内建 40 线程并发处理，通过 `Queue` 分配任务，每个线程输出独立的结果文件：

- `train_data_processed_Thread-{n} (worker).json` — 线程输出
- `train_data_cached_Thread-{n} (worker).json` — 缓存
- `train_data_failed_Thread-{n} (worker).json` — 失败记录

合并多线程结果：

```bash
python enhanceTXT.py
# 会自动合并 train_data_processed_Thread-{n} 到 train_data_final02.json
```

## 增强策略

脚本通过百度 ERNIE Chat API 将每条 `instruction` 扩充为 5 种不同的提问方式，同时保持原意不变。支持断点续传，中断后可从中断位置继续处理。

## 注意事项

- 百度 API 有调用频率限制和费用，请合理控制并发数
- 日志输出到 `app.log`，仅 ERROR 级别日志打印到控制台
- 原始种子数据包含业务敏感信息，请勿对外泄露
