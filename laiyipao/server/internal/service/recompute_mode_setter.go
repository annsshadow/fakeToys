package service

import (
	"fmt"
	"strings"
)

// 运行时切档（2026-10-15 随整局重算接线一起加）。
//
// # 为什么需要它
//
// `config.SettleRecomputeMode` 的注释写着「切档不需要发版：改环境变量重启即可」。
// 但那条路径在**测试**里用不了：`New()` 在构造时就调了 `initRecomputeGate`，
// 把档位固化进了 gate；此后改 `Cfg` 不会有任何效果 ——
// 而这个「改了却不生效」正是 `initRecomputeGate` 里那段 fail loud
// 注释最担心的那种静默失败的反面版本：
// 运营以为切了档，实际上没切。
//
// 所以这里给出显式的 setter：改档位与重建 gate 是**同一个动作**，
// 调用方不可能只做一半。

// SetRecomputeMode 运行时切换重算处置档位。
//
// 非法值**不静默降级**：返回错误，由调用方决定怎么办。
// 与 `initRecomputeGate` 的差别是——后者在启动路径上，
// 只能打印警告后降级（服务要能起来）；这里是显式调用，
// 调用方有机会处理。
//
// 并发安全：它替换整个 gate 指针。当前调用方都是启动期或测试期，
// 没有并发切档的需求；若将来要支持热切换，得改成 `atomic.Pointer`
// 并给 gate 内部加读写锁 —— 那时再说，不提前做。
func (s *Service) SetRecomputeMode(mode string) error {
	if !ValidRecomputeMode(mode) {
		return fmt.Errorf("非法的重算档位 %q（合法值：%s）",
			mode, strings.Join(RecomputeModes, "/"))
	}
	s.Cfg.SettleRecomputeMode = mode
	s.initRecomputeGate()
	return nil
}

// RecomputeMode 读当前生效的档位。
//
// 读的是 **gate 里的**，不是 `Cfg` 里的 ——
// 两者在「有人只改了 Cfg 没重建 gate」时会不一致，
// 而这里要回答的是「现在到底哪个档在生效」。
func (s *Service) RecomputeMode() string {
	if s.gate == nil {
		return RecomputeOff
	}
	return s.gate.Mode()
}
