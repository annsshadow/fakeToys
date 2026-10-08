package httpapi

import (
	"context"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"testing"
)

// `replay_skills` 的结算侧集成测试（第 56 轮）。
//
// ## 为什么必须在这里测，而不能只靠单元测试
//
// `scripts/e2e.ps1` **自己构造 settle payload**，而且**不带** `replay_skills`
// —— 发的是空串，服务端按设计放行（「没上报」与「上报了但不对」要分开）。
//
// 所以那条 e2e 对本校验**什么都没证明**。它只证明「没上报不会被锁死」。
//
// 这里补上真正的那一半：**上报了，就必须对得上**。

func TestE2EReplaySkillsMustMatchBuild(t *testing.T) {
	e := newE2E(t)
	uid, tokStr := e.newPlayer(t, "replayskills")

	// 这条测试要买 6 次 token（每个用例一次），每局结算都扣体力。
	// 体力不够时会在中途以「energy 不足」失败 —— 那个报错完全指不到真正的原因，
	// 所以这里先给足。
	e.exec(t, `UPDATE user_wallets SET energy = 200, energy_updated_at = now() WHERE user_id = $1`, uid)

	// 算出服务端期望值。
	//
	// 这里**不复用** SettleBattle 内部的那个函数：它是未导出的，跨包调不到；
	// 而这里直接查库 + 调**导出的** `domain.BuildReplaySkillsSegment`，
	// 测的正是被测逻辑本身，而不是「拿实现去证明实现」。
	// ⚠️ 先把技能升到 10 级（第 56 轮补）。
	//
	// 第一版这里是「新玩家的构筑」—— 全是 1 级，`coef = 1000`，
	// 于是**等级烘焙整段逻辑等于没被测**：把它删掉测试照样全绿。
	// 变异验证实测就是这个结果（去掉 coef 不红）。
	//
	// 同理槽位：默认只有 5 个（0~4），此时「字符串排序」与「按槽位排序」
	// 结果完全一致，排序那段也测不到。这两处靠 domain 单元测试覆盖，
	// 那边能构造 10+ 槽位与未知 id。
	e.exec(t, `UPDATE user_skills SET level = 10 WHERE user_id = $1`, uid)

	want := e.expectedReplaySkills(t, uid)
	if want == "" {
		t.Fatal("新玩家的构筑应有技能；若为空说明 SeedSkills 或 user_skill_slots 没种上")
	}
	t.Logf("服务端重算的 S 段：%s", want)

	// ---- 1) 上报正确值 → 必须通过 ----
	// 这一条现在验的是「原始构筑（未改底伤前）的正确值」，
	// 而上面 case 0 验的是「运营改过底伤后的正确值」—— 两个都要过。
	status, body, _ := settle(t, e, tokStr, want)
	if status != 200 {
		t.Fatalf("**正确的** replay_skills 被拒了：%d %v\n"+
			"这说明服务端重算与客户端算法不一致 —— 会把所有合法玩家锁死", status, body)
	}
	// ---- 0) 运营改技能底伤后，重算必须跟着变 ----
	//
	// 这一条是**从一次真实翻车里长出来的**，不是预防性测试。
	//
	// 第一版 `replaySkillsSegment` 读的是 `domain.SeedSkills`（Go 常量），
	// 而客户端的技能内容是服务端**从 `skills` 表下发**的。
	// 两者不一致时：客户端上报 DB 的底伤、我比对常量的底伤
	// → 所有合法玩家的每一局都被判成作弊。
	//
	// 开发库当时就是漂的（基础技能 1 的底伤被管理端测试改成了 33，常量仍是 100），
	// 全量跑测试才暴露出来。所以这里直接把这个场景钉住：
	// 改了 DB 的底伤，重算值必须跟着变。
	e.exec(t, `UPDATE skills SET base_damage = base_damage + 7
		WHERE id = (SELECT MIN(skill_id) FROM user_skill_slots WHERE user_id = $1)`, uid)
	wantAfterEdit := e.expectedReplaySkills(t, uid)
	if wantAfterEdit == want {
		t.Fatal("改了 skills.base_damage 之后重算值没变 —— " +
			"重算一定没读 skills 表；那意味着它读的是 Go 常量，运营一改技能就会全量误封")
	}
	status, body, _ = settle(t, e, tokStr, wantAfterEdit)
	if status != 200 {
		t.Fatalf("按改动后的 DB 底伤重算，却被拒了：%d %v", status, body)
	}

	// ---- 1b) 某技能没有 user_skills 行时，等级按 1 算 ----
	//
	// 变异验证发现 LEFT JOIN 改成 INNER JOIN 测试不会红 ——
	// 因为夹具里每条装备技能都恰好有 `user_skills` 行，两种写法结果一样。
	// 这条把「缺行时按 1 级算、且该槽位仍然出现」钉住，
	// 否则将来有人改成 INNER JOIN 就等于悄悄让缺行的技能从校验里消失。
	e.exec(t, `DELETE FROM user_skills
		WHERE user_id = $1
		  AND skill_id = (SELECT MAX(skill_id) FROM user_skill_slots WHERE user_id = $1)`, uid)
	wantNoLevel := e.expectedReplaySkills(t, uid)
	if wantNoLevel == wantAfterEdit {
		t.Fatal("删掉 user_skills 行后重算值没变 —— 等级默认值这条路径没被测到")
	}
	status, body, _ = settle(t, e, tokStr, wantNoLevel)
	if status != 200 {
		t.Fatalf("缺 user_skills 行（等级按 1 算）的 S 段被拒了：%d %v", status, body)
	}

	// ---- 2) 伪造底伤（等价于假报技能等级）→ 必须被拒 ----
	forged := forgeBaseDamage(wantAfterEdit)
	if forged == want {
		t.Fatalf("伪造函数没能改出不同的值（输入 %q）", want)
	}
	status, _, _ = settle(t, e, tokStr, forged)
	if status == 200 {
		t.Errorf("伪造底伤的 replay_skills 被放行了（200）—— 这是本测试最想抓的那一种")
	}

	// ---- 3) 换一个不存在的技能 id → 必须被拒 ----
	status, _, _ = settle(t, e, tokStr, "0:999999:100000:1:20")
	if status == 200 {
		t.Errorf("伪造技能 id 的 replay_skills 被放行了（200）")
	}

	// ---- 4) **等长**篡改底伤 → 必须被拒 ----
	//
	// 这一条专门用来区分「逐字比对」与「只比长度」。
	// 变异验证时我试过把服务端改成只比长度，测试没红 —— 因为
	// 「×10」会让长度变化，长度检查也就跟着拒了。
	// 但换成**等长**的篡改（100 → 900），长度检查就完全失效。
	// 真实作弊者改属性时不会特意保持长度，所以这条必须有。
	swapped, ok2 := swapBaseDamageSameLength(wantAfterEdit)
	if ok2 {
		status, _, _ = settle(t, e, tokStr, swapped)
		if status == 200 {
			t.Errorf("等长篡改底伤的 replay_skills 被放行了（200）—— 说明只比了长度")
		}
	} else {
		t.Log("S 段里没有多位数底伤，跳过等长篡改用例")
	}
}

// settle 走一次完整的买 token + 结算，返回状态码与响应体。
func settle(t *testing.T, e *e2e, tokStr string, skills string) (int, map[string]any, int64) {
	t.Helper()
	status, body := e.post(t, "/api/v1/battle/token", tokStr, map[string]any{"level_id": 1})
	if status != 200 {
		t.Fatalf("购买响应失败 %d %v", status, body)
	}
	id := num(body, "token_id")
	payload := settleBody(id)
	payload["replay_skills"] = skills
	st, resp := e.post(t, "/api/v1/battle/settle", tokStr, payload)
	return st, resp, id
}

// forgeBaseDamage 把 S 段里**最后一条**的底伤改成原值的 10 倍。
//
// 只改一条、只改底伤，是最贴近真实作弊形态的最小变异：
// 「我的技能等级更高」这类伪造不会改技能 id 或槽位。
func forgeBaseDamage(seg string) string {
	if seg == "" {
		return "0:1:999999:1:20"
	}
	// 形如 "0:1:100:1:20,1:2:140:2:30"
	segs := strings.Split(seg, ",")
	last := segs[len(segs)-1]
	parts := strings.Split(last, ":")
	if len(parts) != 5 {
		return seg + ",9:9:999999:9:9"
	}
	base, err := strconv.Atoi(parts[2])
	if err != nil {
		base = 100
	}
	parts[2] = strconv.Itoa(base * 10)
	segs[len(segs)-1] = strings.Join(parts, ":")
	return strings.Join(segs, ",")
}

// expectedReplaySkills 按真实数据库内容算出期望的 S 段。
//
// ## ⚠️ 这里刻意**不**调用 `domain.BuildReplaySkillsSegment`
//
// 我第一版就是这么写的，结果变异验证 0/3 —— 改 `BuildReplaySkillsSegment`
// 的排序、等级烘焙、未知技能处理时，**期望值和被测值一起变**，测试照样全绿。
//
// 这就是「拿实现去证明实现」：两边同源时，任何一侧的错误都会被对方镜像掉，
// 于是测试看起来在守，其实什么都没守。
//
// 所以这里只依赖两样东西：
//  1. skills / user_skills / user_skill_slots 三张表的**原始行**（不走 Go 内容表）
//  2. 字面格式与等级公式（写在下面，看得见就不容易错）
//
// 生产侧若改了公式或排序，这里会红 —— 那正是它该做的事。
func (e *e2e) expectedReplaySkills(t *testing.T, uid int64) string {
	t.Helper()
	rows, err := e.pool.Query(context.Background(),
		`SELECT sl.slot, s.id, s.base_damage, s.apply_stacks, s.heat_cost,
		        COALESCE(us.level, 1)
		   FROM user_skill_slots sl
		   JOIN skills s ON s.id = sl.skill_id
		   LEFT JOIN user_skills us
		          ON us.skill_id = sl.skill_id AND us.user_id = sl.user_id
		  WHERE sl.user_id = $1`,
		uid,
	)
	if err != nil {
		t.Fatalf("查构筑失败：%v", err)
	}
	defer rows.Close()

	segs := []string{}
	for rows.Next() {
		var slot, id, lvl int
		var base, stacks, heat int64
		if err := rows.Scan(&slot, &id, &base, &stacks, &heat, &lvl); err != nil {
			t.Fatalf("扫构筑行失败：%v", err)
		}
		// 等级加成：coef = 1000 + (level-1)*50，底伤 = base * coef / 1000
		// 公式里的 50 按约定写死在这里；
		// `TestSkillRulesMatchServerContract` 守着它与 `SkillLevelCoefPermille` 一致。
		coef := 1000 + (lvl-1)*50
		segs = append(segs, fmt.Sprintf("%d:%d:%d:%d:%d",
			slot, id, base*int64(coef)/1000, stacks, heat))
	}
	// ⚠️ 排的是**格式化后的字符串**，不是按槽位 —— 与客户端 Array.sort 对齐。
	sort.Strings(segs)
	return strings.Join(segs, ",")
}

// swapBaseDamageSameLength 把最后一条的底伤改成**同长度**但不同的值（100 → 200）。
//
// 返回 (新串, 是否成功)。底伤不是多位数时无法等长替换，返回 false。
func swapBaseDamageSameLength(seg string) (string, bool) {
	if seg == "" {
		return "", false
	}
	segs := strings.Split(seg, ",")
	last := segs[len(segs)-1]
	parts := strings.Split(last, ":")
	if len(parts) != 5 {
		return "", false
	}
	base, err := strconv.Atoi(parts[2])
	if err != nil || base < 100 {
		return "", false
	}
	// 等长但不同：把最高位换成另一个非零数字
	s := parts[2]
	repl := s[0]
	if repl == '9' {
		repl = '8'
	} else {
		repl++
	}
	parts[2] = string(repl) + s[1:]
	if parts[2] == s {
		return "", false
	}
	segs[len(segs)-1] = strings.Join(parts, ":")
	return strings.Join(segs, ","), true
}
