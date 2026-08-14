## 内核驱动 i2c-nvidia-gpu


数据手册：未公开。

作者：
	Ajay Gupta <ajayg@nvidia.com>

### 描述


i2c-nvidia-gpu 是针对 NVIDIA Turing 及更晚 GPU 中所包含 I2C 控制器的驱动，用于与 GPU 上的 Type-C 控制器通信。

```

  01:00.3 Serial bus controller [0c80]: NVIDIA Corporation Device 1ad9 (rev a1)

```
则该驱动应支持你的 GPU 的 I2C 控制器。
