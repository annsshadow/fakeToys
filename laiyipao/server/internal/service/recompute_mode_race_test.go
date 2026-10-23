// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

package service

import (
	"fmt"
	"sync"
	"testing"
)

// TestSetRecomputeModeConcurrentWithStateReads 钉住「运行期切档 × 状态读」并发无竞争。
//
// # 背景（2026-10-24 修数据竞争）
//
// `SetRecomputeMode` 曾经直接写 `s.Cfg.SettleRecomputeMode`——而 `Cfg` 是
// `New` 时的**值拷贝**；`RecomputeModeState`（GET /admin/recomputes/mode）
// 并发读同一个字段。运营在后台页面切档（PUT）的同时另一人刷新状态（GET），
// 即构成数据竞争。`go test -race` 能抓，但此前没有用例覆盖这条路径。
//
// 修法：档位的**运行期唯一真相是 gate**（`atomic.Pointer` 整体替换），
// 切档只换 gate、不写 Cfg；状态读单次 Load，快照内自洽。
//
// # 判据
//  1. `-race` 下并发切档 + 并发读：无竞争报告；
//  2. 每次读到的快照自洽（权威档位 = S 段 = 配置档，enforcing 与档位一致）；
//  3. 终态显式切档后，状态全家桶一致；
//  4. 运行期切档**不写 Cfg**（竞争的根源已消除）。
func TestSetRecomputeModeConcurrentWithStateReads(t *testing.T) {
	s := &Service{}
	s.Cfg.SettleRecomputeMode = RecomputeOff
	s.initRecomputeGate() // 与 New 同路径：按 Cfg 建初始 gate

	modes := []string{RecomputeOff, RecomputeShadow, RecomputeEnforce}
	valid := func(m string) bool {
		for _, x := range modes {
			if m == x {
				return true
			}
		}
		return false
	}

	var wg sync.WaitGroup
	// 写侧：反复切档（模拟运营连点 / 多个运营同时切）。
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			for j := 0; j < 50; j++ {
				m := modes[(i+j)%len(modes)]
				if err := s.SetRecomputeMode(m); err != nil {
					panic(fmt.Sprintf("合法档位 %q 不应报错: %v", m, err))
				}
			}
		}(i)
	}
	// 读侧：并发读状态与档位（模拟后台页面轮询 / 多人同时看）。
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < 200; j++ {
				st := s.RecomputeModeState()
				// 并发期间只断言**逐字段**有效 + 快照内自洽
				// （单次 Load 保证 configure/skills/engine 同源一个 gate）。
				for _, m := range []string{st.Mode, st.ConfiguredMode, st.EngineMode, st.SkillsMode} {
					if !valid(m) {
						panic(fmt.Sprintf("状态字段出现非法档位 %q: %+v", m, st))
					}
				}
				if st.Mode != st.SkillsMode || st.SkillsMode != st.ConfiguredMode {
					panic(fmt.Sprintf("快照不自洽（权威档位/S 段/配置档应同源）: %+v", st))
				}
				if st.Enforcing != (st.Mode == RecomputeEnforce) {
					panic(fmt.Sprintf("enforcing 与档位不符: %+v", st))
				}
				_ = s.RecomputeMode()
			}
		}()
	}
	wg.Wait()

	// 终态：显式切一档，随后读必须完全一致。
	if err := s.SetRecomputeMode(RecomputeEnforce); err != nil {
		t.Fatalf("终态切档应成功: %v", err)
	}
	st := s.RecomputeModeState()
	if st.ConfiguredMode != RecomputeEnforce || st.Mode != RecomputeEnforce ||
		st.EngineMode != RecomputeEnforce || st.SkillsMode != RecomputeEnforce || !st.Enforcing {
		t.Errorf("终态应为 enforce 全家桶，实际 %+v", st)
	}
	// 运行期切档不得写 Cfg——那是数据竞争的根源。
	if s.Cfg.SettleRecomputeMode != RecomputeOff {
		t.Errorf("运行期切档不应写 Cfg（应保持启动配置 %q），实际 %q",
			RecomputeOff, s.Cfg.SettleRecomputeMode)
	}
}
