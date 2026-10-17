-- 00015: 结算重算审计台账（R1 第一步：可观测 + 可回退）
--
-- ## 为什么要有这张表
--
-- R1 要让服务端自己重跑一遍战斗，用重算值替代客户端上报值。
-- 一旦这么做，有一个**之前不存在的**失败模式：
--
--   **Go 引擎与 TS 引擎出现漂移 → 诚实玩家的上报被判不符 → 奖励被拒发。**
--
-- 这个失败模式有三个特别糟的性质：
--
--   1. **批量**：漂移是系统性的，一处算错 → 某关/某构筑的所有玩家全被拒。
--   2. **静默**：结算返回 422，客户端只会显示「上报不可信」，
--      玩家与运营都看不出「其实是我们算错了」。
--   3. **不可回退**：没有台账就不知道该回退哪一段、影响面多大。
--
-- 前两条合起来意味着：**引擎漂移的后果在被发现之前是不可见的**，
-- 而发现的方式是玩家投诉。那不是监控，那是事故通报。
--
-- 所以这张表不是「顺手记一笔」，它是 R1 能不能安全上线的**前置条件**。
--
-- ## 它记录什么
--
-- 上报值与重算值**并列存放**，以及「具体哪些字段不符」。
--
-- ⚠️ 为什么不只记一个 `mismatched BOOLEAN`：
-- 「分数不符」这个信号无法区分下面两种完全不同的情况 ——
--
--   (a) 玩家在作弊；
--   (b) Go 引擎的某一步算错了。
--
-- 而处置方式完全相反：(a) 要封号，(b) 要回滚引擎。
-- 区分办法是**按关卡/按字段聚合** —— 若不符集中在某一关或某一个字段，
-- 那几乎必然是 (b)；若均匀散布在单个玩家身上，那是 (a)。
-- 所以 `mismatched_fields` 必须是可聚合的维度，而不是一个布尔。
--
-- ## 为什么重算值允许 NULL
--
-- 引擎尚未接入时，`shadow` 模式记的就是「还没算」。
-- 把两列设成 NOT NULL 会逼迫引擎一上线就得写满，
-- 于是这张表在「引擎未接入」阶段根本没法用 ——
-- 而**恰恰是那段时间最需要它**（先观察，再收紧）。
--
-- ## 与 replay_verifications 的区别
--
-- 那张表是**自愿验真**（玩家或运营拿重算的 hash 发起比对），
-- 它证明的是「改数据不改凭证」。
-- 本表是**服务端强制重算**的记录，`verifier` 永远是服务端，
-- 且它记的不只是 hash，还有直接换算成奖励的那几个数。
--
-- ## 回滚
--
-- Down 只删表。本表**只增不改**玩家数据，删掉不影响任何在途逻辑；
-- 唯一的代价是历史重算记录消失 —— 而那正是回滚时要看的证据，
-- 所以注释里写明「有损」。

-- +goose Up

CREATE TABLE settle_recomputes (
    id                     BIGSERIAL PRIMARY KEY,

    -- 关联对象。注意**不建外键到 battle_records**：
    -- battle_records.id 是 BIGSERIAL，而这张表的价值恰恰在于
    -- 「那一局被判定不符」时还能查到当时的原始上报值。
    -- 加外键会让 battle_records 被清理时连带删掉证据。
    battle_id              BIGINT   NOT NULL,
    user_id                BIGINT   NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    level_id               INTEGER  NOT NULL,

    -- 上报值（客户端）
    reported_score         BIGINT   NOT NULL,
    reported_kills         INTEGER  NOT NULL,
    reported_duration_ms   INTEGER  NOT NULL,
    reported_hash          TEXT     NOT NULL DEFAULT '',

    -- 重算值（服务端引擎）。引擎未接入时为 NULL。
    recomputed_hash        TEXT,
    recomputed_score       BIGINT,
    recomputed_kills       INTEGER,
    recomputed_duration_ms INTEGER,

    -- 不符的具体字段。可聚合维度，见文件头「为什么不只记一个布尔」。
    -- 约定取值：'score' / 'kills' / 'leaked' / 'duration' / 'hash'
    mismatched_fields      TEXT[]   NOT NULL DEFAULT '{}',

    -- 当时的处置模式。shadow=只记录不拒发；enforce=拒发奖励。
    mode                   TEXT     NOT NULL,

    -- 是否真的拒发了奖励。
    --
    -- ⚠️ 与 mode 分开存的原因：熔断器可以在 enforce 模式下把**单次**处置
    -- 降级成 shadow（见 recompute_gate.go）。若只存 mode，
    -- 「这一局到底有没有被拒」就从这一个布尔里读不出来，
    -- 而那恰恰是运营问的第一个问题。
    rejected               BOOLEAN  NOT NULL DEFAULT FALSE,

    -- 引擎版本。用于「哪一次发布开始大量不符」的定位。
    engine_version         TEXT     NOT NULL DEFAULT '',

    created_at             TIMESTAMPTZ NOT NULL DEFAULT now(),

    -- mode 只能是这两个值。CHECK 而不是靠应用层保证 ——
    -- 因为这张表会被多处写入，而「一条 mode='enforce' 的 shadow 记录」
    -- 会让不符率统计悄悄失真。
    CONSTRAINT chk_settle_recomputes_mode
        CHECK (mode IN ('shadow', 'enforce'))
);

-- 按玩家查：运营接到「这个号是不是在作弊」的举报时
CREATE INDEX idx_settle_recomputes_user
    ON settle_recomputes (user_id, created_at DESC);

-- 按关卡聚合：区分「引擎某一关算错」与「某玩家全关都在造假」的主力索引
CREATE INDEX idx_settle_recomputes_level
    ON settle_recomputes (level_id, created_at DESC)
    WHERE mismatched_fields <> '{}';

-- 时间序：熔断器与看板都要按时间窗口统计
CREATE INDEX idx_settle_recomputes_time
    ON settle_recomputes (created_at DESC);

-- ⚠️ 刻意**不建**「只索引不符行」的部分索引：
-- 熔断器要算的是「不符数 / 总数」而不是「不符数」，
-- 只有不符行的话分母就没了 —— 而分母恰恰是判断
-- 「这是一次系统性漂移还是零星作弊」的关键。

-- +goose Down
--
-- ⚠️ 有损：历史重算记录会消失，而那正是回滚时唯一的证据。
-- 只在「宁可丢证据也要立刻回滚」时才执行。

DROP TABLE IF EXISTS settle_recomputes;