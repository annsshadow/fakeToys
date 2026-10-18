package service

// 结算重算的**处置闸门**（R1 第一步）。
//
// # 这个文件解决的问题
//
// R1 要让服务端自己重跑一遍战斗并据此发奖。那会带来一个**之前不存在**的
// 失败模式：Go 引擎与 TS 引擎漂移 → 诚实玩家的上报被判不符 → 奖励被拒发。
//
// 这个失败模式最糟的地方不是「会误伤」，而是**误伤在被发现之前不可见**：
// 结算返回 422，客户端显示「上报不可信」，玩家觉得自己被作弊了，
// 运营看不出「其实是我们算错了」。发现方式是玩家投诉。
//
// 所以闸门提供三件事：
//
//  1. **三档模式** —— off / shadow / enforce。可以不发版就切档。
//  2. **影子模式** —— 只记录不拒发。这是唯一能安全验证「引擎重算准不准」的档位：
//     在它上面观察若干天，不符率平稳之后才谈 enforce。
//  3. **熔断** —— enforce 模式下若不符率**突增**，自动降级回 shadow。
//
// # 但闸门管着**两类**重算，它们的规则相反
//
// 这是本文件最容易做错、也真的做错过的地方。
//
//	整局引擎（尚未移植）
//	    不符**有歧义** —— 可能是作弊，也可能是我方引擎算错了。
//	    熔断在这里是正当的：漂移是系统性的，批量拒发是灾难。
//
//	replay_skills 的 S 段（已在生产运行）
//	    不符**无歧义** —— 服务端与客户端读同一份 DB 数据，
//	    中间没有第二个实现，所以不符只可能是有人手改了上报值。
//	    熔断在这里**有害**：作弊脚本一开刷、不符率冲高，
//	    防护就被降级 —— 恰好在攻击发生时关掉刹车。
//
// 所以熔断与降级状态都按 scope 分开（见 RecomputeScope 与 RecomputeGate.demoted）。
// 共用一个总开关会同时产生两个故障：引擎漂移时误伤诚实玩家，
// 以及攻击峰值时自动解除防护。
//
// # 为什么缺省档位是 enforce 而不是 off
//
// 挡不住的一条推理是：「引擎会漂移，所以新装部署先别 enforce」。
// 它把上面两类混成了一类 —— 而第二类**早就在生产上 enforce 着**。
// 按那个推理做，等于让新装部署出厂即关闭一条运行中的反作弊校验。
//
// 「先 shadow 观察」是**接入引擎时**要主动做的动作，不是缺省状态。
// 详见 config.SettleRecomputeMode 的注释。
//
// # 为什么熔断必须自动，而不是「监控 + 人工降级」
//
// 漂移是**发布之后立刻**发生的。人盯看板的时间尺度是小时，
// 而一个漂移可以在几分钟内拒发上千个真实玩家的奖励。
//
// 更糟的是：人工降级需要「有人已经注意到了」。而这件事的表现是
// 玩家投诉打进客服 —— 那时伤害已经造成了。
//
// 自动熔断把「发现」这一环从人身上移走：
// **它在伤害发生的那一刻就停止伤害**，同时把证据留在台账里供事后分析。
//
// # 为什么熔断不自动「升回」enforce
//
// 降级是「出事了先停下」，升级是「没事了可以继续」。
// 前者的代价是误判（白白降级，观察一段时间再手动开回来即可）；
// 后者的代价是**在故障未修好的情况下自动恢复伤害**。
//
// 所以升回只允许人工：配置改回来 + 重启。这是刻意的。

import (
	"context"
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
	"time"
)

// 重算处置模式。
const (
	// RecomputeOff 不重算、不记录。新装部署的默认档。
	RecomputeOff = "off"
	// RecomputeShadow 重算并记录，但**不拒发奖励**。
	RecomputeShadow = "shadow"
	// RecomputeEnforce 重算并记录，不符则拒发奖励。
	RecomputeEnforce = "enforce"
)

// RecomputeModes 是全部合法取值 —— 校验与文档共用同一份来源。
//
// ⚠️ 刻意做成 slice 而不是 map：它的**顺序**要能直接当作
// 「严格程度从低到高」来读，而 map 的遍历顺序不保证。
// 将来若要加一档，插在正确的位置上即可。
var RecomputeModes = []string{RecomputeOff, RecomputeShadow, RecomputeEnforce}

// ValidRecomputeMode 报告 mode 是否合法。
func ValidRecomputeMode(mode string) bool {
	for _, m := range RecomputeModes {
		if m == mode {
			return true
		}
	}
	return false
}

// StrictnessOf 返回某档的严格程度（越大越严格）。
//
// 用途：熔断降级后要记「本来应该是哪一档」，否则台账里
// 留下的 mode 会被读成「这就是配置值」，从而看不出发生过降级。
func StrictnessOf(mode string) int {
	switch mode {
	case RecomputeOff:
		return 0
	case RecomputeShadow:
		return 1
	case RecomputeEnforce:
		return 2
	}
	return -1 // 非法
}

// RecomputeDecision 是闸门对「这一局怎么处理」的裁决。
type RecomputeDecision struct {
	// Compute 为真时，调用方**必须**执行重算并调用 RecordRecompute。
	Compute bool
	// Enforce 为真时，不符即拒发奖励；否则只记录。
	Enforce bool
	// Mode 是**实际生效**的档位（可能已被熔断降级）。
	//
	// ⚠️ 必须是「实际生效」而不是「配置的」—— 记配置值的话，
	// 台账里会留下一批 mode=enforce 却没拒发的行，
	// 而那正是运营查「为什么这批没被拒」时最需要看到的差异。
	Mode string
	// Demoted 为真表示本次是被熔断降级的。
	Demoted bool
	// Reason 是人可读的说明，进台账与日志。
	Reason string
}

// RecomputeEngineVersionSkillsOnly 是「只重算 S 段」时的引擎版本标记。
//
// 为什么需要「版本」字段：漂移是**发布之后**才发生的，
// 而台账里那一批行的价值恰恰在于「它们是哪一次发布产生的」。
// 没有版本号，运营看到「最近一批不符」时无法把它对应到某个提交。
const RecomputeEngineVersionSkillsOnly = "skills-only/v1"

// RecomputeEngineVersionNone 是「尚未重算」时的版本标记。
const RecomputeEngineVersionNone = "none"

// Gate 返回重算闸门。
func (s *Service) Gate() *RecomputeGate { return s.gate }

// recomputeVerdict 返回「当前实际生效的档位」与「是否 enforce」。
//
// 它是闸门在结算路径上的唯一入口 —— 结算代码**不直接碰**闸门，只问这一句。
// 这样以后引擎接进来时，闸门逻辑的改动不会波及结算代码。
//
// scope 决定本裁决是否允许被熔断降级，见 RecomputeScope 的注释。
func (s *Service) recomputeVerdict(ctx context.Context, scope RecomputeScope) (mode string, enforce bool) {
	if s.gate == nil {
		// 闸门恒不为 nil（New 里必建）。这里仍兜一层：
		// 结算路径上 nil 指针 panic 的代价，远高于「退到最保守档」。
		return RecomputeOff, false
	}
	d := s.gate.Decide(ctx, timeNow(), scope)
	return d.Mode, d.Enforce
}

// initRecomputeGate 构造闸门并把它挂到 Service 上。
//
// # 为什么不在 New 里直接写
//
// 闸门的不符率回调要查库（s.pool），而 s 在构造过程中还不存在。
// 所以先建 svc、再回填 gate —— Go 里打破构造期循环依赖的常规手法。
func (s *Service) initRecomputeGate() {
	s.gate = NewRecomputeGate(s.Cfg.SettleRecomputeMode, s.recomputeMismatchRate)

	if !ValidRecomputeMode(s.Cfg.SettleRecomputeMode) {
		// fail loud（AGENTS #11）：非法值**静默**退到 off 会让
		// 「我明明配了 enforce」变成「其实一直没生效」——
		// 而那正是最不该静默的一种失败（运营会以为防护已开）。
		fmt.Fprintf(os.Stderr,
			"[recompute] SETTLE_RECOMPUTE_MODE=%q 不是合法值（合法：%s），"+
				"已退到 %q —— **重算与 replay_skills 校验都不会生效**。\n",
			s.Cfg.SettleRecomputeMode, strings.Join(RecomputeModes, "/"), RecomputeOff)
		return
	}
	switch s.Cfg.SettleRecomputeMode {
	case RecomputeOff:
		fmt.Fprintln(os.Stderr,
			"[recompute] 模式 off：不做服务端重算，replay_skills 不符也**不拒发**（只记台账）。\n"+
				"  ⚠️ 这关掉了 replay_skills 的反作弊校验。仅在排查「校验误伤」时使用。")
	case RecomputeShadow:
		fmt.Fprintln(os.Stderr,
			"[recompute] 模式 shadow：重算并记台账，**不拒发**。\n"+
				"  ⚠️ 这同样关掉了 replay_skills 的反作弊校验（它不符即作弊证据，不熔断）。\n"+
				"  接整局引擎前可在此档观察引擎不符率；只看 S 段时保持 enforce。")
	case RecomputeEnforce:
		fmt.Fprintln(os.Stderr,
			"[recompute] 模式 enforce：重算并记台账，不符即拒发。\n"+
				"  S 段（replay_skills）不符一律拒发，**不参与熔断** —— 它不符即作弊证据。\n"+
				"  引擎侧不符率超阈值会自动降级回 shadow（并打印原因）；不会自动升档。")
	}
}

// RecomputeGate 是重算闸门。
//
// 并发安全：结算路径是多并发的，而熔断状态是全局的。
// 用 mutex 而不是原子变量 —— 降级是**罕见**事件，
// 且降级时要顺带冻结一个时间窗，重原子操作反而更复杂。
type RecomputeGate struct {
	mu sync.Mutex

	mode string

	// demoted **按 scope 分开**记录，而不是一个全局字段。
	//
	// ⚠️ 这里踩过一次坑，值得写下来：降级状态做成全局的，
	// 于是引擎侧的熔断会**连带**把 S 段也降掉 ——
	// 一个「引擎算错了」的故障，顺手关掉了反作弊校验。
	//
	// 那正是 Scope 注释里警告的「安全网装在刹车上」，
	// 只是从另一条路径实现出来的。
	// 分开记之后，「降级」只影响它自己那个 scope。
	demoted [numRecomputeScopes]demotion

	// rateFn 由 service 层注入（查台账）。放成函数是为了让
	// gate 本身不依赖数据库 —— 它的逻辑（档位 + 熔断判定）
	// 可以纯内存单测。
	rateFn func(ctx context.Context, since time.Time) (mismatch, total int, err error)
}

// demotion 是一个 scope 的降级状态。
type demotion struct {
	// to 非空表示当前被熔断降级过（值是降级后的档）。
	to string
	// at 是降级发生时刻。
	at time.Time
	// reason 供 /readyz 与日志引用。
	reason string
}

// NewRecomputeGate 构造闸门。
//
// ⚠️ 非法档位退到 **off**（而不是 enforce）：
// 配错时「什么都不做」比「意外开始拒发真实玩家」安全。
// 但必须**大声**告诉运维配错了 —— 见 initRecomputeGate。
//
// rateFn 可为 nil —— 那时引擎侧**不做熔断**（无法观测就无法安全降级）。
// 这种情况下必须如实说明，因为「不熔断的引擎 enforce」是最危险的组合。
// S 段不受影响（它本来就不熔断）。
func NewRecomputeGate(mode string, rateFn func(context.Context, time.Time) (int, int, error)) *RecomputeGate {
	if !ValidRecomputeMode(mode) {
		mode = RecomputeOff
	}
	return &RecomputeGate{mode: mode, rateFn: rateFn}
}

// Mode 返回**配置的**档位（未经熔断）。
func (g *RecomputeGate) Mode() string {
	g.mu.Lock()
	defer g.mu.Unlock()
	return g.mode
}

// Demoted 返回指定 scope 当前是否处于熔断降级状态。
func (g *RecomputeGate) Demoted(scope RecomputeScope) (bool, string, time.Time) {
	g.mu.Lock()
	defer g.mu.Unlock()
	d := g.demoted[scope]
	if d.to == "" {
		return false, "", time.Time{}
	}
	return true, d.reason, d.at
}

// RecomputeScope 标明「这次裁决是为哪一类重算做的」。
//
// # 为什么必须区分两类，而不能只有一个总开关
//
// 两类重算的**不符含义完全不同**，而熔断的正当性完全取决于这一点：
//
//	引擎（skills 之外的整局重算）
//	    不符 = **有歧义**。可能是玩家作弊，也可能是 Go 引擎算错了。
//	    而漂移是**系统性**的 —— 某一段公式错了 → 所有玩家的上报都不符。
//	    这时批量拒发是在制造大规模伤害，所以需要熔断兜底。
//
//	S 段（replay_skills 重算）
//	    不符 = **无歧义的作弊证据**。服务端从 DB 重算，客户端从 DB 上报，
//	    两边读的是同一份数据，中间没有第二个实现。
//	    所以「不符」只可能是有人手改了上报值。
//
// 对第二类装熔断会造成一个非常糟的后果：
// 作弊脚本开始刷 → 不符率瞬间冲高 → 熔断触发 → 校验被降级 →
// **防护恰好在攻击发生时失效**，而且所有诚实玩家的请求也一起被放过。
// 那等于把安全网装在了刹车上。
//
// 所以熔断只对引擎生效，对 S 段永不生效。
type RecomputeScope int

const (
	// ScopeEngine 是整局引擎重算。不符有歧义，允许熔断降级。
	ScopeEngine RecomputeScope = iota
	// ScopeSkills 是 replay_skills（S 段）重算。
	// 不符即作弊证据，**永不**被熔断降级。
	ScopeSkills

	// numRecomputeScopes 是 scope 数量。降级状态按它开数组。
	//
	// ⚠️ 加新 scope 时**必须**同步改这里，否则数组越界 panic ——
	//   这是刻意的：宁可启动即崩，也不要静默覆盖别人的降级状态。
	numRecomputeScopes = iota
)

// Decide 裁决这一局怎么处理。
//
// 熔断判定**每次调用都做**（而不是只在启动时做一次）：
// 漂移是发布之后才出现的，启动时统计到的样本量为零。
func (g *RecomputeGate) Decide(ctx context.Context, now time.Time, scope RecomputeScope) RecomputeDecision {
	g.mu.Lock()
	defer g.mu.Unlock()

	if g.mode == RecomputeOff {
		return RecomputeDecision{Reason: "重算未启用（SETTLE_RECOMPUTE_MODE=off）"}
	}

	// 只在 enforce 档、且**仅对引擎**考虑熔断：
	//   - shadow 本来就不拒发，降它没有意义；
	//   - 把 shadow 降成 off 会让台账停止记录 —— 那恰恰是
	//     「正在出问题时最需要证据」的场景；
	//   - S 段的不符是作弊证据，降级它等于在攻击发生时关掉防护（见 Scope 注释）。
	if g.mode == RecomputeEnforce && scope == ScopeEngine && g.demoted[scope].to == "" {
		if trip, reason := g.checkBreaker(ctx, now); trip {
			g.demoted[scope] = demotion{to: RecomputeShadow, at: now, reason: reason}
			fmt.Fprintf(os.Stderr,
				"[recompute] 熔断降级 enforce → shadow（%s）。\n"+
					"  这通常意味着 Go 引擎与 TS 引擎漂移，诚实玩家的上报正被判不符。\n"+
					"  奖励不再被拒发，但重算仍在记录 —— 证据在 settle_recomputes 表。\n"+
					"  修复后需**人工**把 SETTLE_RECOMPUTE_MODE 改回 enforce（不自动升档）。\n"+
					"  ⚠️ 本次降级**只作用于引擎**；replay_skills 的反作弊校验不受影响。\n",
				reason)
		}
	}

	// 读**本 scope 自己的**降级状态 —— 不是全局的。
	// 读全局的话，引擎的熔断会连带把 S 段降级（见 RecomputeGate 的 demoted 字段）。
	eff := g.mode
	demoted := false
	if d := g.demoted[scope]; d.to != "" && StrictnessOf(d.to) < StrictnessOf(g.mode) {
		eff = d.to
		demoted = true
	}

	return RecomputeDecision{
		Compute: eff != RecomputeOff,
		Enforce: eff == RecomputeEnforce,
		Mode:    eff,
		Demoted: demoted,
		Reason:  g.reasonLocked(eff, demoted, scope),
	}
}

// checkBreaker 判断是否该熔断。
//
// # 阈值为什么是「样本量 + 不符率」两条，而不是只看不符率
//
// 只看不符率的话：**低流量时段**任意一次误判就是 100% 不符率 → 立刻熔断。
// 那会造成一种很坏的形态：每天凌晨必然降级一次，白天手动开回来，
// 然后夜里又降 —— 反复横跳，而根因可能只是那一局恰好撞上某条边界。
//
// 所以两条都要：样本量不够时**不下结论**（返回 false 并说明），
// 因为「还没看懂」和「看懂了没问题」必须区分开。
func (g *RecomputeGate) checkBreaker(ctx context.Context, now time.Time) (bool, string) {
	if g.rateFn == nil {
		return false, ""
	}
	// 窗口取 10 分钟：要短到「伤害还没扩大」，长到「不是被一局噪声触发」。
	const window = 10 * time.Minute
	mismatch, total, err := g.rateFn(ctx, now.Add(-window))
	if err != nil {
		// ⚠️ 查不到统计时**不熔断**：熔断的依据是「观测到大量不符」，
		// 而不是「查不到数据」。后者可能是数据库抖动 ——
		// 那时降级会让一次短暂故障变成一次长达数小时的功能停摆。
		return false, ""
	}
	return evaluateBreaker(total, mismatch, MinBreakerSamples, MaxBreakerRate)
}

// MinBreakerSamples 是熔断所需的最小样本量。
//
// 200 是拍的，但依据说得清：真实线上每分钟几十到几百次结算，
// 10 分钟窗口里 200 条对应**每天几分钟**的量级 ——
// 低于这个量，任何比率的统计噪声都大于信号。
const MinBreakerSamples = 200

// MaxBreakerRate 是允许的不符率上限（千分比）。
//
// 20‰ = 2%。定这个数的理由：
//
//   - 诚实玩家的不符率**理论上是 0** —— 重算与真实上报逐位相同才不叫不符。
//     所以任何非零值都值得看，而 2% 足以把「一次系统性漂移」
//     与「零星作弊」分开（作弊通常是某个号的 100%，不是全服的 2%）。
//   - 不能定 0：样本里总会混进网络重试、客户端时钟漂移这类边缘情况。
//   - 不能定 5%：5% 意味着 20 分之一的真实玩家被拒发，那已经是事故了。
const MaxBreakerRate = 20

// evaluateBreaker 是熔断判定的**纯函数**部分（便于单测）。
//
// 返回 (是否熔断, 说明)。说明非空时便于日志与台账引用。
func evaluateBreaker(total, mismatch, minSamples, maxRate int) (bool, string) {
	if total < minSamples {
		return false, ""
	}
	if mismatch*1000 <= total*maxRate {
		return false, ""
	}
	rate := mismatch * 100 / total // 整数百分比，仅用于文案
	return true, fmt.Sprintf(
		"近 %d 次重算中 %d 次不符（%d%%，上限 %d‰），超过阈值 %d 条样本",
		total, mismatch, rate, maxRate, minSamples)
}

// reasonLocked 返回人可读的处置说明。调用方须已持锁。
func (g *RecomputeGate) reasonLocked(eff string, demoted bool, scope RecomputeScope) string {
	base := "重算模式 " + eff
	if demoted {
		return base + "（已被熔断从 " + g.mode + " 降级：" + g.demoted[scope].reason + "）"
	}
	return base
}

// --- 重算台账 ---

// RecomputeRecord 是一次重算的完整记录。
//
// ⚠️ 「重算值」一律用**指针**：nil = 引擎未接入 / 未产出。
// 用值类型（int64）的话「没算」与「算出来是 0」无法区分 ——
// 而「算出来是 0」在合法战斗里不可能出现，那正是最强的可疑信号。
type RecomputeRecord struct {
	BattleID int64
	UserID   int64
	LevelID  int

	ReportedScore      int64
	ReportedKills      int
	ReportedDurationMs int
	ReportedHash       string

	RecomputedHash       string
	RecomputedScore      int64
	RecomputedKills      int
	RecomputedDurationMs int
	HasRecomputed        bool

	MismatchedFields []string

	Mode          string
	Rejected      bool
	EngineVersion string
}

// MismatchFields 返回不符字段列表（never nil，调用方可直接写库）。
func (r *RecomputeRecord) MismatchFields() []string {
	if r.MismatchedFields == nil {
		return []string{}
	}
	return r.MismatchedFields
}

// recomputeRow 是台账表的落库结构。
type recomputeRow struct {
	BattleID             int64
	UserID               int64
	LevelID              int
	ReportedScore        int64
	ReportedKills        int
	ReportedDurationMs   int
	ReportedHash         string
	RecomputedHash       *string
	RecomputedScore      *int64
	RecomputedKills      *int
	RecomputedDurationMs *int
	MismatchedFields     []string
	Mode                 string
	Rejected             bool
	EngineVersion        string
	CreatedAt            time.Time
}

// RecordRecompute 把一次重算结果写进台账。
//
// # 为什么写入**永不返回错误给调用方**
//
// 结算主流程不该因为「台账写不进去」而失败 ——
// 那会把一个**观测手段**的故障升级成**业务**故障。
//
// 但「永不报错」不能等于「静默吞掉」：写失败必须打日志，
// 否则「台账空着」与「没人重算过」在观测上无法区分 ——
// 那正是这张表存在的理由被推翻。
func (s *Service) RecordRecompute(ctx context.Context, r RecomputeRecord) {
	row := recomputeRow{
		BattleID:           r.BattleID,
		UserID:             r.UserID,
		LevelID:            r.LevelID,
		ReportedScore:      r.ReportedScore,
		ReportedKills:      r.ReportedKills,
		ReportedDurationMs: r.ReportedDurationMs,
		ReportedHash:       r.ReportedHash,
		MismatchedFields:   r.MismatchFields(),
		Mode:               r.Mode,
		Rejected:           r.Rejected,
		EngineVersion:      r.EngineVersion,
	}
	if r.HasRecomputed {
		h, sc, k, d := r.RecomputedHash, r.RecomputedScore,
			r.RecomputedKills, r.RecomputedDurationMs
		row.RecomputedHash, row.RecomputedScore = &h, &sc
		row.RecomputedKills, row.RecomputedDurationMs = &k, &d
	}
	if !ValidRecomputeMode(row.Mode) {
		row.Mode = RecomputeShadow // 非法值退到最保守档，绝不退到 enforce
	}

	if _, err := s.pool.Exec(ctx, `
		INSERT INTO settle_recomputes
		  (battle_id, user_id, level_id,
		   reported_score, reported_kills, reported_duration_ms, reported_hash,
		   recomputed_hash, recomputed_score, recomputed_kills, recomputed_duration_ms,
		   mismatched_fields, mode, rejected, engine_version)
		VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)`,
		row.BattleID, row.UserID, row.LevelID,
		row.ReportedScore, row.ReportedKills, row.ReportedDurationMs, row.ReportedHash,
		row.RecomputedHash, row.RecomputedScore, row.RecomputedKills, row.RecomputedDurationMs,
		row.MismatchedFields, row.Mode, row.Rejected, row.EngineVersion,
	); err != nil {
		fmt.Fprintf(os.Stderr,
			"[recompute] 台账写入失败 battle=%d user=%d mode=%s：%v\n"+
				"  结算本身仍按正常流程完成，但这一局**没有**留下重算证据。\n"+
				"  若持续出现，请检查 settle_recomputes 表与数据库连接。\n",
			r.BattleID, r.UserID, r.Mode, err)
	}
}

// recomputeMismatchRate 统计 `since` 之后的不符率。
//
// 形状是 (mismatch, total) 而不是「一个比率」——
// 分母为 0 时「比率」只能返回 0，那会让「样本不足」与「零不符」不可区分，
// 而熔断判定必须能区分这两者（见 evaluateBreaker 的注释）。
func (s *Service) recomputeMismatchRate(ctx context.Context, since time.Time) (int, int, error) {
	var mismatch, total int
	err := s.pool.QueryRow(ctx, `
		SELECT COUNT(*) FILTER (WHERE mismatched_fields <> '{}'), COUNT(*)
		FROM settle_recomputes WHERE created_at >= $1`, since).
		Scan(&mismatch, &total)
	if err != nil {
		return 0, 0, fmt.Errorf("recompute mismatch rate: %w", err)
	}
	return mismatch, total, nil
}

// --- 后台读侧 ---

// RecomputeView 是后台台账的一行。
type RecomputeView struct {
	ID            int64          `json:"id"`
	BattleID      int64          `json:"battle_id"`
	UserID        int64          `json:"user_id"`
	Nickname      string         `json:"nickname"`
	LevelID       int            `json:"level_id"`
	Reported      RecomputeSide  `json:"reported"`
	Recomputed    *RecomputeSide `json:"recomputed"`
	Mismatched    []string       `json:"mismatched_fields"`
	Mode          string         `json:"mode"`
	Rejected      bool           `json:"rejected"`
	EngineVersion string         `json:"engine_version"`
	CreatedAt     time.Time      `json:"created_at"`
}

// RecomputeSide 是上报值/重算值共用的形状。
type RecomputeSide struct {
	Score      int64  `json:"score"`
	Kills      int    `json:"kills"`
	DurationMs int    `json:"duration_ms"`
	Hash       string `json:"hash"`
}

// RecomputeFilter 是后台台账的筛选条件。
type RecomputeFilter struct {
	// OnlyMismatch 为真时只返回有不符字段的行。
	OnlyMismatch bool
	LevelID      int
	UserID       int64
	Limit        int
	Offset       int
}

// AdminListRecomputes 分页读取重算台账。
//
// ⚠️ 排序固定为 `id DESC`：这张表的价值全在「最近的」——
// 漂移是刚刚发生的，按时间倒序才能一眼看到最新一批。
func (s *Service) AdminListRecomputes(ctx context.Context, f RecomputeFilter) ([]RecomputeView, int64, error) {
	if f.Limit <= 0 || f.Limit > 200 {
		f.Limit = 50
	}
	if f.Offset < 0 {
		f.Offset = 0
	}
	conds := []string{"1=1"}
	args := []any{}
	add := func(clause string, v any) {
		args = append(args, v)
		conds = append(conds, fmt.Sprintf(clause, len(args)))
	}
	if f.OnlyMismatch {
		conds = append(conds, "r.mismatched_fields <> '{}'")
	}
	if f.LevelID > 0 {
		add("r.level_id = $%d", f.LevelID)
	}
	if f.UserID > 0 {
		add("r.user_id = $%d", f.UserID)
	}
	where := strings.Join(conds, " AND ")

	var total int64
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM settle_recomputes r WHERE `+where, args...).
		Scan(&total); err != nil {
		return nil, 0, fmt.Errorf("count recomputes: %w", err)
	}

	args = append(args, f.Limit, f.Offset)
	limitPH := "$" + strconv.Itoa(len(args)-1)
	offsetPH := "$" + strconv.Itoa(len(args))
	rows, err := s.pool.Query(ctx, `
		SELECT r.id, r.battle_id, r.user_id, COALESCE(u.nickname,''), r.level_id,
		       r.reported_score, r.reported_kills, r.reported_duration_ms, r.reported_hash,
		       r.recomputed_score, r.recomputed_kills, r.recomputed_duration_ms, r.recomputed_hash,
		       r.mismatched_fields, r.mode, r.rejected, r.engine_version, r.created_at
		FROM settle_recomputes r
		LEFT JOIN users u ON u.id = r.user_id
		WHERE `+where+`
		ORDER BY r.id DESC LIMIT `+limitPH+` OFFSET `+offsetPH, args...)
	if err != nil {
		return nil, 0, fmt.Errorf("list recomputes: %w", err)
	}
	defer rows.Close()

	out := []RecomputeView{}
	for rows.Next() {
		var v RecomputeView
		var rs RecomputeSide
		var recScore *int64
		var recKills *int
		var recDur *int
		var recHash *string
		if err := rows.Scan(&v.ID, &v.BattleID, &v.UserID, &v.Nickname, &v.LevelID,
			&rs.Score, &rs.Kills, &rs.DurationMs, &rs.Hash,
			&recScore, &recKills, &recDur, &recHash,
			&v.Mismatched, &v.Mode, &v.Rejected, &v.EngineVersion, &v.CreatedAt); err != nil {
			return nil, 0, fmt.Errorf("scan recompute: %w", err)
		}
		v.Reported = rs
		if recScore != nil || recKills != nil || recDur != nil || recHash != nil {
			side := RecomputeSide{}
			if recScore != nil {
				side.Score = *recScore
			}
			if recKills != nil {
				side.Kills = *recKills
			}
			if recDur != nil {
				side.DurationMs = *recDur
			}
			if recHash != nil {
				side.Hash = *recHash
			}
			v.Recomputed = &side
		}
		out = append(out, v)
	}
	return out, total, rows.Err()
}

// RecomputeLevelStat 是按关卡聚合的统计。
//
// 为什么按**关卡**聚合是主要的判读方式：
// 系统性漂移几乎总是集中在少数关卡/构筑上（某段公式算错），
// 而零星作弊是均匀散布在单个玩家身上的。
// 两者在同一张表里，聚合维度决定了你能不能一眼分出来。
type RecomputeLevelStat struct {
	LevelID           int `json:"level_id"`
	Total             int `json:"total"`
	Mismatched        int `json:"mismatched"`
	RatePermille      int `json:"rate_permille"`
	MaxUserMismatches int `json:"max_user_mismatches"`
}

// AdminRecomputeStatsByLevel 按关卡聚合，返回不符率最高的前 N 关。
//
// # 内层聚合为什么**不能**把 mismatched_fields 放进 GROUP BY
//
// 第一版写的是 `GROUP BY level_id, mismatched_fields, user_id`，
// 那样 `per_user` 量到的是「该玩家在这一关、这一种不符字段组合下的行数」，
// 而不是「该玩家在这一关的不符行数」。
//
// 具体坏在哪：某玩家在第 3 关有 50 局相符 + 3 局不符，
// 分组后得到 per_user=50（相符那组）与 per_user=3（不符那组），
// `MAX(per_user)` 取到 **50** —— 于是「单玩家不符次数」显示成 50。
//
// 而这一列正是区分漂移与作弊的关键：漂移是**每个**玩家都中，
// 作弊是**某一个**玩家反复中。把它算错，两者就长得一样。
//
// 正确形状：先按 (level, user) 归并，在同一个组内同时数出
// 总行数与不符行数，外层再对关卡求和 / 取最大。
func (s *Service) AdminRecomputeStatsByLevel(ctx context.Context, since time.Time, limit int) ([]RecomputeLevelStat, error) {
	if limit <= 0 || limit > 200 {
		limit = 20
	}
	rows, err := s.pool.Query(ctx, `
		SELECT level_id,
		       SUM(per_user_total)::bigint AS total,
		       SUM(per_user_mism)::bigint  AS mismatched,
		       COALESCE(MAX(per_user_mism), 0) AS max_user
		FROM (
		    SELECT level_id, user_id,
		           COUNT(*) AS per_user_total,
		           COUNT(*) FILTER (WHERE mismatched_fields <> '{}') AS per_user_mism
		    FROM settle_recomputes
		    WHERE created_at >= $1
		    GROUP BY level_id, user_id
		) t
		GROUP BY level_id
		ORDER BY (SUM(per_user_mism))::float
		         / GREATEST(SUM(per_user_total), 1) DESC,
		         level_id
		LIMIT $2`, since, limit)
	if err != nil {
		return nil, fmt.Errorf("recompute stats: %w", err)
	}
	defer rows.Close()

	out := []RecomputeLevelStat{}
	for rows.Next() {
		var v RecomputeLevelStat
		var total, mismatched int64
		if err := rows.Scan(&v.LevelID, &total, &mismatched, &v.MaxUserMismatches); err != nil {
			return nil, err
		}
		v.Total = int(total)
		v.Mismatched = int(mismatched)
		if v.Total > 0 {
			v.RatePermille = v.Mismatched * 1000 / v.Total
		}
		out = append(out, v)
	}
	return out, rows.Err()
}

// RecomputeTotals 是窗口内的总账。
type RecomputeTotals struct {
	Total        int `json:"total"`
	Mismatched   int `json:"mismatched"`
	Rejected     int `json:"rejected"`
	RatePermille int `json:"rate_permille"`
}

// RecomputeSummary 是给后台看的「一句话现状」。
//
// # 为什么**必须**带闸门状态，而不只是统计数字
//
// 统计数字回答不了运营真正要问的那个问题：
// **「现在还在拦吗？」**
//
// 熔断触发后，系统状态变成「仍在重算、仍在记录，但**不再拒发**」。
// 这时台账里的不符行数可能完全正常（甚至因为不拒发而上升），
// 而单看数字的人会以为「防护开着」。
//
// `demoted` 就是为了让「我们正在放过本该拒掉的东西」无法被忽略。
type RecomputeSummary struct {
	// Mode 是**实际生效**的档位（可能已被熔断降级）。
	Mode string `json:"mode"`
	// ConfiguredMode 是配置值，用于区分「本来配的就是 shadow」与「被熔断了」。
	ConfiguredMode string `json:"configured_mode"`
	// Demoted 为真表示 enforce 已被熔断降级 —— 此时**奖励在放行**。
	Demoted bool `json:"demoted"`
	// DemotedReason 为空表示未降级。
	DemotedReason string `json:"demoted_reason"`
	// Since 是统计窗口起点。
	Since string `json:"since"`
	// BreakerRatePermille 是熔断阈值（‰），让界面上能标出「离熔断还有多远」。
	BreakerRatePermille int `json:"breaker_rate_permille"`
	// BreakerMinSamples 是熔断所需的最小样本量。
	BreakerMinSamples int `json:"breaker_min_samples"`

	Totals RecomputeTotals `json:"totals"`
	// Levels 是按关卡分档的台账分布（**只含不符**，口径同 Totals）。
	Levels []RecomputeLevelStat `json:"levels"`

	Engine EngineTotals `json:"engine"`
	// Engine 是**引擎侧**的累计读数（进程内计数器，从进程启动算起），
	// 与上面 Totals（台账 SQL，只含不符，受 since 窗口限制）口径不同。
	// 单看 Totals 会把「校验根本没跑」误判成「没问题」—— 见 EngineTotals 的注释。
}

// AdminRecomputeSummary 汇总窗口内的总账 + 按关卡分布 + 闸门当前状态。
func (s *Service) AdminRecomputeSummary(ctx context.Context, since time.Time, levelLimit int) (RecomputeSummary, error) {
	sum := RecomputeSummary{
		Since:               since.UTC().Format(time.RFC3339),
		BreakerRatePermille: MaxBreakerRate,
		BreakerMinSamples:   MinBreakerSamples,
		Levels:              []RecomputeLevelStat{},
		ConfiguredMode:      RecomputeOff,
		Mode:                RecomputeOff,
		// 引擎侧累计计数。放在最前面填：它是**唯一一个**
		// 「校验到底有没有在跑」的读数，其余字段都来自台账，
		// 而台账在「从不不符」时是空的。
		Engine: s.engineTotals(),
	}
	if s.gate != nil {
		sum.ConfiguredMode = s.gate.Mode()
		// 这里读的是**引擎**侧状态：S 段永不降级（见 RecomputeScope），
		// 所以「被降级了」这件事本身就直接说明是引擎在放行。
		demoted, reason, _ := s.gate.Demoted(ScopeEngine)
		sum.Demoted = demoted
		sum.DemotedReason = reason
		// 实际生效档位 = 配置档位，除非被熔断降到更松的那档。
		eff, _ := s.recomputeVerdict(ctx, ScopeEngine)
		sum.Mode = eff
	}

	var rejected int
	err := s.pool.QueryRow(ctx, `
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE mismatched_fields <> '{}'),
		       COUNT(*) FILTER (WHERE rejected)
		FROM settle_recomputes WHERE created_at >= $1`, since).
		Scan(&sum.Totals.Total, &sum.Totals.Mismatched, &rejected)
	if err != nil {
		return sum, fmt.Errorf("recompute totals: %w", err)
	}
	sum.Totals.Rejected = rejected
	if sum.Totals.Total > 0 {
		sum.Totals.RatePermille = sum.Totals.Mismatched * 1000 / sum.Totals.Total
	}

	levels, err := s.AdminRecomputeStatsByLevel(ctx, since, levelLimit)
	if err != nil {
		return sum, err
	}
	sum.Levels = levels
	return sum, nil
}
