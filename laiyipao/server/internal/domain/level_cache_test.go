package domain

// 关卡缓存测试与基准（2026-10-30 实质优化第 14 轮）。
//
// 判据：
//   - CachedLevel 与 GenerateLevel 对全部 100 关逐关 DeepEqual（确定性不变）；
//   - 非法 id 返回零值而不是 panic；
//   - 基准对比「每次生成」与「缓存读取」的耗时差异。

import (
	"reflect"
	"testing"
)

func TestCachedLevelMatchesGenerateLevel(t *testing.T) {
	for id := 1; id <= TotalLevels; id++ {
		cached := CachedLevel(id)
		fresh := GenerateLevel(id)
		if !reflect.DeepEqual(cached, fresh) {
			t.Errorf("第 %d 关：CachedLevel 与 GenerateLevel 结果不一致", id)
		}
	}
}

func TestCachedLevelInvalidID(t *testing.T) {
	for _, id := range []int{0, -1, TotalLevels + 1} {
		lv := CachedLevel(id)
		if lv.ID != 0 {
			t.Errorf("CachedLevel(%d) 应返回零值，实际 ID=%d", id, lv.ID)
		}
	}
}

// BenchmarkGenerateLevel 是优化前的路径：每次请求重新生成整关。
func BenchmarkGenerateLevel(b *testing.B) {
	for i := 0; i < b.N; i++ {
		_ = GenerateLevel(i%TotalLevels + 1)
	}
}

// BenchmarkCachedLevel 是优化后的路径：进程内只生成一次，之后 O(1) 读取。
func BenchmarkCachedLevel(b *testing.B) {
	// 预热：首次调用会生成全部关卡，不计入基准。
	_ = CachedLevel(1)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		_ = CachedLevel(i%TotalLevels + 1)
	}
}
