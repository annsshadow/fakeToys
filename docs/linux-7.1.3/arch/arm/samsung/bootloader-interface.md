## Exynos 板上内核与引导加载程序之间的接口

本文档描Linux 内核与基于三Exynos 的板卡上引导加载程序（U-Boot/SBOOT 等）之间的接口约定，列出非安安全模式下各 SYSRAM PMU 偏移的用途，供平台移植与固件开发者参考


作者：Krzysztof Kozlowski

日期015 6 6 
本文档试图描述目Linux 内核与基于三Exynos 的板卡上引导加载程序之间所使用的接口这并不是对接口的定义，而只是对现有状态的描述，仅供参考
在本文档中，“引导加载程序”指以下任意一种：U-boot、专SBOOT，或任何其他在执行内核之初始化板卡的、用ARMv7 ARMv8 的固件

1. 非安全模式（Non-Secure mode
地址     sysram_ns_base_addr

============= ============================================ ==================
偏移                                                   用============= ============================================ ==================
0x08          exynos_cpu_resume_ns, mcpm_entry_point       系统挂起（System suspend0x0c          0x00000bad (Magic cookie)                    系统挂起
0x1c          exynos4_secondary_startup                    辅助 CPU 启动
0x1c + 4*cpu  exynos4_secondary_startup (Exynos4412)       辅助 CPU 启动
0x20          0xfcba0d10 (Magic cookie)                    AFTR
0x24          exynos_cpu_resume_ns                         AFTR
0x28 + 4*cpu  0x8 (Magic cookie, Exynos3250)               AFTR
0x28          0x0 或恢复期间最后的(Exynos542x)           系统挂起
============= ============================================ ==================


2. 安全模式（Secure mode
地址     sysram_base_addr

============= ============================================ ==================
偏移                                                   用============= ============================================ ==================
0x00          exynos4_secondary_startup                    辅助 CPU 启动
0x04          exynos4_secondary_startup (Exynos542x)       辅助 CPU 启动
4*cpu         exynos4_secondary_startup (Exynos4412)       辅助 CPU 启动
0x20          exynos_cpu_resume (Exynos4210 r1.0)          AFTR
0x24          0xfcba0d10 (Magic cookie, Exynos4210 r1.0)   AFTR
============= ============================================ ==================

地址     pmu_base_addr

============= ============================================ ==================
偏移                                                   用============= ============================================ ==================
0x0800        exynos_cpu_resume                            AFTR, suspend
0x0800        mcpm_entry_point (Exynos542x with MCPM)      AFTR, suspend
0x0804        0xfcba0d10 (Magic cookie)                    AFTR
0x0804        0x00000bad (Magic cookie)                    系统挂起
0x0814        exynos4_secondary_startup (Exynos4210 r1.1)  辅助 CPU 启动
0x0818        0xfcba0d10 (Magic cookie, Exynos4210 r1.1)   AFTR
0x081C        exynos_cpu_resume (Exynos4210 r1.1)          AFTR
============= ============================================ ==================

3. 其他（无论安非安全模式）

地址     pmu_base_addr

============= =============================== ===============================
偏移                                   用============= =============================== ===============================
0x0908        Non-zero                        辅助 CPU 启动指示
                                              用于 Exynos3250 Exynos542x
============= =============================== ===============================

4. 术语表（Glossary
AFTR - ARM Off Top Running，一种低功耗模式，Cortex 核心及许多其他模块被电源门控（power gated），
       TOP 模块除外
MCPM - Multi-Cluster Power Management（多簇电源管理）
