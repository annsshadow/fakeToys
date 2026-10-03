# 🧠 llm-lab · 从零动手学大模型训练

一个**可运行、可验证、带完整中文讲义**的大模型训练教学项目。目标不是堆术语，而是让你亲手把一个 GPT **从零造出来 → 训练起来 → 再优化**，每个机制都看得见、改得动。

> 学习项目：模型只有几万~几十万参数，为的是"跑得动、看得清"。但它的每个组件（自动微分、注意力、训练循环、LoRA、DPO）和真正的大模型是同构的。

## 两条代码轨

| 轨道 | 目录 | 依赖 | 定位 |
| --- | --- | --- | --- |
| **原理轨 minigrad** | [`minigrad/`](minigrad) | 仅 `numpy` | 纯手写张量级自动微分 + GPT，**任何环境都能跑**。看懂它=看懂反向传播/注意力/训练循环的全部机制。 |
| **实战轨 torchgpt** | [`torchgpt/`](torchgpt) | `torch`（可选） | PyTorch 复刻 nanoGPT，演示工业界真用的东西：AMP 混合精度、梯度累积/裁剪、LoRA、DPO、困惑度评估。未装 torch 时相关测试自动跳过。 |

两轨结构刻意一致（Pre-LN、因果注意力、4x MLP），先在原理轨搞懂"为什么"，再去实战轨看"怎么工程化"。

## 快速开始

```bash
cd llm-lab
pip install -r requirements.txt      # 只需 numpy + pytest

python -m minigrad.train             # 训练字符级 GPT：loss ~4.1 → ~0.3，并生成文本
pytest tests/ -q                     # 19 passed, 1 skipped（含逐算子数值梯度校验）
```

想上 PyTorch 实战轨：

```bash
pip install -r requirements-torch.txt   # 版本按 https://pytorch.org 选
pytest tests/ -q                         # 原本 skip 的 7 个 torchgpt 用例会激活
```

## 目录结构

```
llm-lab/
├── minigrad/            # 原理轨（纯 NumPy）
│   ├── engine.py        #   张量级 autograd 引擎（反向传播地基）
│   ├── nn.py            #   Linear/Embedding/LayerNorm/注意力/Block/GPT
│   ├── optim.py         #   SGD / AdamW
│   ├── tokenizer.py     #   字符级 + BPE 分词器
│   ├── data.py          #   批次采样
│   ├── train.py         #   完整训练循环（可直接 python -m 运行）
│   └── sample.py        #   自回归文本生成
├── torchgpt/            # 实战轨（PyTorch）
│   ├── model.py         #   nanoGPT 风格 GPT（FlashAttention 快路径）
│   ├── train.py         #   AMP + 梯度累积 + 裁剪 + 余弦 LR + checkpoint
│   ├── lora.py          #   LoRA 低秩微调
│   ├── dpo.py           #   DPO 偏好对齐
│   ├── eval.py          #   困惑度评估
│   ├── sample.py / data.py / config.py
├── docs/                # 中文课程讲义 00~10（见下）
├── data/tiny_corpus.txt # 内置小语料
└── tests/               # 梯度校验 + 单元 + 端到端冒烟
```

## 课程讲义

从 [`docs/00-学习路线.md`](docs/00-学习路线.md) 开始，按顺序读讲义 + 读对应代码 + 做练习：

| # | 讲义 | 关键词 |
| --- | --- | --- |
| 01 | [分词与词表](docs/01-分词与词表.md) | 字符级 / BPE / 中文分词 |
| 02 | [嵌入与位置编码](docs/02-嵌入与位置编码.md) | token 嵌入 / 位置编码 / RoPE |
| 03 | [自注意力机制](docs/03-自注意力机制.md) | QKV / 缩放点积 / 因果 mask / 多头 |
| 04 | [Transformer 结构](docs/04-Transformer结构.md) | 残差 / Pre-LN / MLP / 参数量 |
| 05 | [预训练与训练循环](docs/05-预训练与训练循环.md) | autograd / 交叉熵 / AdamW / 过拟合 |
| 06 | [规模化训练](docs/06-规模化训练.md) | 混合精度 / 梯度累积 / DDP / FSDP / ZeRO |
| 07 | [微调 SFT·LoRA·QLoRA](docs/07-微调-SFT-LoRA-QLoRA.md) | 指令微调 / 低秩适配 / 4bit |
| 08 | [对齐 RLHF·PPO·DPO](docs/08-对齐-RLHF-PPO-DPO.md) | 偏好对齐 / 奖励模型 / DPO |
| 09 | [评估](docs/09-评估.md) | 困惑度 / 基准 / 采样策略 |
| 10 | [推理优化](docs/10-推理优化.md) | KV cache / 量化 / 投机解码 |

## 设计原则

- **可验证**：`tests/test_engine.py` 用中心差分对每个算子做数值梯度校验——引擎正确性有据可查，不是"看起来对"。
- **原理优先**：所有网络层都建立在自己手写的 autograd 之上，复杂函数（LayerNorm、注意力）的反向传播是"免费"的，这正是自动微分的威力。
- **两轨印证**：手写版讲清机制，PyTorch 版讲清工程，差异处（如权重共享、FlashAttention）在讲义里专门点出。

## 许可证

AGPL-3.0-or-later，随 fakeToys 仓库整体协议。
