package domain

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strconv"
	"testing"
)

// TestProbeReplayDump 一次性探针：导出 Go 侧整局重放的 replay 事件流。
//
// 保留在仓库里的理由：`TestWholeBattleReplayKnownDivergences` 记录了
// 4 条未收敛的分歧，它们的定位手段就是「把两端的 replay 流导出后逐条比」。
// 探针删掉之后下一次分歧就只能靠肉眼。
func TestProbeReplayDump(t *testing.T) {
	if os.Getenv("PROBE_OUT") == "" {
		t.Skip("设 PROBE_OUT 与 PROBE_CASE 才运行（见 miniapp/scripts/probe-replay-dump.ts 的对端）")
	}
	raw, err := os.ReadFile(filepath.Join("..", "..", "testdata", "replay_vectors.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc replayVectorDoc
	if err := json.Unmarshal(raw, &doc); err != nil {
		t.Fatal(err)
	}
	idx, _ := strconv.Atoi(orEnvDefault("PROBE_CASE", "1"))
	out := os.Getenv("PROBE_OUT")
	c := doc.Cases[idx]
	enemies, skills, rules := buildReplayDeps(t)
	build := toBuildSnapshot(t, c.Build)
	eq := EquippedFromSnapshot(build, skills)
	seed, _ := strconv.ParseInt(c.Seed, 10, 64)
	att, _ := AttackerFromSnapshot(build)
	e := NewBattleEngine(BattleConfig{
		Level: replayLevel(t, c.LevelID), Enemies: enemies, Skills: skills,
		Equipped: eq, Attacker: att, Seed: seed,
		ActiveSlots: build.ActiveSlots, ScoreRules: rules,
	})
	e.SetReplayScript(c.CardPicks)
	e.Start()
	RunToEnd(e, MaxTicksPerReplay)
	lines := make([]string, 0, len(e.replay))
	for _, r := range e.replay {
		lines = append(lines, r.String())
	}
	if err := os.WriteFile(out, []byte(joinLines(lines)), 0o600); err != nil {
		t.Fatal(err)
	}
	t.Logf("case %d replay=%d -> %s", c.LevelID, len(e.replay), out)
}

func orEnvDefault(k, def string) string {
	if v := os.Getenv(k); v != "" {
		return v
	}
	return def
}

func joinLines(v []string) string {
	out := ""
	for i, s := range v {
		if i > 0 {
			out += "\n"
		}
		out += s
	}
	return out
}
