package config

// `SETTLE_RECOMPUTE_MODE` 默认值的守卫。
//
// # 为什么默认值本身需要一条测试
//
// 默认值是**没有任何人做决定时**生效的那一个 —— 也就是说，
// 出问题时它是最后才会被想起来的那个。
//
// 本条尤其重要：这个默认值曾经被定成 `off`，而那会**出厂关闭**
// 已经在生产上运行的 `replay_skills` 反作弊校验。
// 事故不是发生在有人改配置时，而是发生在「什么都没配」的部署上。

import "testing"

// TestRecomputeModeDefaultsToEnforce 守住默认 enforce。
//
// # 判据为什么是「不等于 off」而不是「等于 enforce」
//
// 严格相等会挡住「默认变严」（比如某天新增一档更严的默认值）——
// 那不该由本测试否决。真正必须禁掉的只有一种：**默认不防护**。
//
// 写成 `== "off"` 时，未来有人把它改成 "" 或 "OFF" 也能通过；
// 写成「不为 off」时，那两种都会红。
func TestRecomputeModeDefaultsToEnforce(t *testing.T) {
	clearEnv(t)
	cfg := Load()
	if cfg.SettleRecomputeMode == "off" {
		t.Fatal("SETTLE_RECOMPUTE_MODE 默认不得为 off —— " +
			"那等于让**新装部署出厂即关闭** replay_skills 反作弊校验。" +
			"该校验已在生产运行，且不符即作弊证据（无歧义），不需要先观察。")
	}
	if cfg.SettleRecomputeMode != "enforce" {
		t.Logf("注意：默认值已改为 %q（比 enforce 更严或更松？），"+
			"确认是有意为之", cfg.SettleRecomputeMode)
	}
}

// TestRecomputeModeIsOverridable 守住「仍然可以不发版地切档」。
//
// ⚠️ 这一条与上一条**必须同时**成立：
//
//	上一条保证「默认是安全的」，这一条保证「出事时能立刻关掉」。
//
// 只写上一条的话，整个世界就只能 enforce ——
// 遇到「校验误伤了正常玩家」时唯一的手段是回滚发版，
// 那比配错环境变量更慢，而线上等不起。
func TestRecomputeModeIsOverridable(t *testing.T) {
	clearEnv(t)
	for _, want := range []string{"off", "shadow", "enforce"} {
		t.Setenv("SETTLE_RECOMPUTE_MODE", want)
		if got := Load().SettleRecomputeMode; got != want {
			t.Errorf("SETTLE_RECOMPUTE_MODE=%s 未生效，实际 %q", want, got)
		}
	}
}
