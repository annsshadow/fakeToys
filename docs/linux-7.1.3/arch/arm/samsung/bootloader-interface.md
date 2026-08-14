## Exynos 鏉夸笂鍐呮牳涓庡紩瀵煎姞杞界▼搴忎箣闂寸殑鎺ュ彛

鏈枃妗ｆ弿杩?Linux 鍐呮牳涓庡熀浜庝笁鏄?Exynos 鐨勬澘鍗′笂寮曞鍔犺浇绋嬪簭锛圲-Boot/SBOOT 绛夛級涔嬮棿鐨勬帴鍙ｇ害瀹氾紝鍒楀嚭闈炲畨鍏?瀹夊叏妯″紡涓嬪悇 SYSRAM 涓?PMU 鍋忕Щ鐨勭敤閫旓紝渚涘钩鍙扮Щ妞嶄笌鍥轰欢寮€鍙戣€呭弬鑰冦€?


浣滆€咃細Krzysztof Kozlowski

鏃ユ湡锛?015 骞?6 鏈?6 鏃?
鏈枃妗ｈ瘯鍥炬弿杩扮洰鍓?Linux 鍐呮牳涓庡熀浜庝笁鏄?Exynos 鐨勬澘鍗′笂寮曞鍔犺浇绋嬪簭涔嬮棿鎵€浣跨敤鐨勬帴鍙ｃ€?杩欏苟涓嶆槸瀵规帴鍙ｇ殑瀹氫箟锛岃€屽彧鏄鐜版湁鐘舵€佺殑鎻忚堪锛屼粎渚涘弬鑰冦€?
鍦ㄦ湰鏂囨。涓紝鈥滃紩瀵煎姞杞界▼搴忊€濇寚浠ヤ笅浠绘剰涓€绉嶏細U-boot銆佷笓鏈?SBOOT锛屾垨浠讳綍鍏朵粬鍦ㄦ墽琛屽唴鏍镐箣鍓?鍒濆鍖栨澘鍗＄殑銆佺敤浜?ARMv7 涓?ARMv8 鐨勫浐浠躲€?

1. 闈炲畨鍏ㄦā寮忥紙Non-Secure mode锛?
鍦板潃锛?     sysram_ns_base_addr

============= ============================================ ==================
鍋忕Щ          鍊?                                         鐢ㄩ€?============= ============================================ ==================
0x08          exynos_cpu_resume_ns, mcpm_entry_point       绯荤粺鎸傝捣锛圫ystem suspend锛?0x0c          0x00000bad (Magic cookie)                    绯荤粺鎸傝捣
0x1c          exynos4_secondary_startup                    杈呭姪 CPU 鍚姩
0x1c + 4*cpu  exynos4_secondary_startup (Exynos4412)       杈呭姪 CPU 鍚姩
0x20          0xfcba0d10 (Magic cookie)                    AFTR
0x24          exynos_cpu_resume_ns                         AFTR
0x28 + 4*cpu  0x8 (Magic cookie, Exynos3250)               AFTR
0x28          0x0 鎴栨仮澶嶆湡闂存渶鍚庣殑鍊?(Exynos542x)           绯荤粺鎸傝捣
============= ============================================ ==================


2. 瀹夊叏妯″紡锛圫ecure mode锛?
鍦板潃锛?     sysram_base_addr

============= ============================================ ==================
鍋忕Щ          鍊?                                         鐢ㄩ€?============= ============================================ ==================
0x00          exynos4_secondary_startup                    杈呭姪 CPU 鍚姩
0x04          exynos4_secondary_startup (Exynos542x)       杈呭姪 CPU 鍚姩
4*cpu         exynos4_secondary_startup (Exynos4412)       杈呭姪 CPU 鍚姩
0x20          exynos_cpu_resume (Exynos4210 r1.0)          AFTR
0x24          0xfcba0d10 (Magic cookie, Exynos4210 r1.0)   AFTR
============= ============================================ ==================

鍦板潃锛?     pmu_base_addr

============= ============================================ ==================
鍋忕Щ          鍊?                                         鐢ㄩ€?============= ============================================ ==================
0x0800        exynos_cpu_resume                            AFTR, suspend
0x0800        mcpm_entry_point (Exynos542x with MCPM)      AFTR, suspend
0x0804        0xfcba0d10 (Magic cookie)                    AFTR
0x0804        0x00000bad (Magic cookie)                    绯荤粺鎸傝捣
0x0814        exynos4_secondary_startup (Exynos4210 r1.1)  杈呭姪 CPU 鍚姩
0x0818        0xfcba0d10 (Magic cookie, Exynos4210 r1.1)   AFTR
0x081C        exynos_cpu_resume (Exynos4210 r1.1)          AFTR
============= ============================================ ==================

3. 鍏朵粬锛堟棤璁哄畨鍏?闈炲畨鍏ㄦā寮忥級

鍦板潃锛?     pmu_base_addr

============= =============================== ===============================
鍋忕Щ          鍊?                         鐢ㄩ€?============= =============================== ===============================
0x0908        Non-zero                        杈呭姪 CPU 鍚姩鎸囩ず
                                              鐢ㄤ簬 Exynos3250 涓?Exynos542x
============= =============================== ===============================

4. 鏈琛紙Glossary锛?
AFTR - ARM Off Top Running锛屼竴绉嶄綆鍔熻€楁ā寮忥紝Cortex 鏍稿績鍙婅澶氬叾浠栨ā鍧楄鐢垫簮闂ㄦ帶锛坧ower gated锛夛紝
       浣?TOP 妯″潡闄ゅ
MCPM - Multi-Cluster Power Management锛堝绨囩數婧愮鐞嗭級
