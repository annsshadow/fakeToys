## 旧微码（Old Microcode）


内核维护一张已发布微码的表。在启动时微码早于该表的系统会显示“Vulnerable（易受攻击）”。这意味着该系统对某些已知的 CPU 问题存在漏洞。可能是安全问题，也可能是功能问题，内核并不知道也不关心。

你应该更新 CPU 微码以缓解任何暴露风险。这通常通过正常的发行版更新来更新 /lib/firmware/intel-ucode/ 下的文件来完成。Intel 也在 github 仓库中分发这些文件：

	https://github.com/intel/Intel-Linux-Processor-Microcode-Data-Files.git

与所有其他硬件漏洞一样，暴露情况在启动时确定。运行时微码更新不会改变此漏洞的状态。
