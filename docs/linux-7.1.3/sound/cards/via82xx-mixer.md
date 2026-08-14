## VIA82xx 混音器


在许多 VIA82xx 主板上，`Input Source Select` 混音控制不起作用。在此类主板上将其设为
`Input2` 会导致录音挂起，或通过 OSS 模拟以 EIO（输入/输出错误）失败。对于此类网卡，
该控制应保持在 `Input1`。
