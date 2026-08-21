## 测试设备驱动中的挂起与恢复支

	(C) 2007 Rafael J. Wysocki <rjw@sisk.pl>, GPL

## 1. 准备测试系统


遗憾的是，要有效测试驱动中对系统级挂起（suspend）与恢复（resume）转换的支持，必须在一个加载了该驱动、功能完整的系统上实际进行挂起与恢复。此外，应当多次进行，最好连续多次，并且针对休眠（hibernation，即 suspend to disk STD）与挂起到内存（suspend to RAM，STR）分别进行，因为这两种情况涉及略有不同的操作以及与机BIOS 的不同交互
当然，为此目的，测试系统必须已知在不加载被测驱动的情况下能够正常挂起与恢复。因此，如果可能，你应当在开始测试新驱动之前，先解决测试系统中所有与挂起/恢复相关的问题。有关挂恢复功能调试的更多信息，请参Documentation/power/basic-pm-debugging.rst
## 2. 测试驱动


一旦你在没有新驱动的情况下解决了测试系统的挂起/恢复相关问题，就可以开始测试它
a) 将驱动编译为模块，加载它并尝试休眠的测试模式（参见：Documentation/power/basic-pm-debugging.rst）
b) 加载驱动并尝试在“reboot”、“shutdown”与“platform”模式下休眠（参见：Documentation/power/basic-pm-debugging.rst）
c) 将驱动直接编译进内核并尝试休眠的测试模式
d) 尝试在驱动直接编译进内核的情况下，于“reboot”、“shutdown”与“platform”模式下休眠
e) 尝试挂起的测试模式（参见：Documentation/power/basic-pm-debugging.rst）。[STR 测试而言，驱动是否编译为模块并无影响。]

f) 尝试在加载驱动的情况下使s2ram 工具挂起到内存（参见：Documentation/power/basic-pm-debugging.rst）
上述每个测试都应重复多次，并STD 测试应当STR 测试混合进行。如果任何一个测试失败，则该驱动不能被视为挂恢复安全的（suspend/resume-safe）