-- 00013_achievement_epoch.sql 成就改为终身累计（第 93 轮）

-- +goose Up

-- # 缺陷：成就可以每天重复领取
--
-- `user_tasks` 按 `(user_id, task_id, task_date)` 分行，
-- 而 `periodStart(now, "achievement")` 原先落到今天零点 ——
-- 于是「成就」也成了每天一行：
--
--	今天：progress = 20，claimed_at = NULL  → 领取，写 claimed_at
--	明天：**新的一行**，claimed_at = NULL → 再领一次
--
-- 而 `ClaimTask` 同样只读 `periodStart(now, scope)`（今天那一行）。
--
-- 修复：`periodStart(now, "achievement")` 返回固定哨兵日期 1970-01-01，
-- 于是一个用户对每条成就永远只有一行，`GREATEST` 累计成终身进度，
-- `claimed_at` 一旦写入就永久生效。
--
-- # 为什么需要这条迁移
--
-- 改了 `periodStart` 之后，已存在的成就行仍留在各自的历史日期上，
-- 而哨兵日期那一行是**新插入的、claimed_at 为 NULL** ——
-- 于是所有已经领过成就的账号会**再领一次**。
--
-- 所以必须把历史行合并进哨兵行：
--   progress  取 MAX（终身进度）
--   claimed_at 取最早的（已经领过就永久领过）
--
-- ⚠️ 用 MIN(claimed_at) 而不是 NOW()：
-- 「曾经领过」是既成事实，迁移不该改写它。

INSERT INTO user_tasks (user_id, task_id, task_date, progress, claimed_at)
SELECT ut.user_id,
       ut.task_id,
       DATE '1970-01-01',
       MAX(ut.progress),
       MIN(ut.claimed_at)
  FROM user_tasks ut
  JOIN tasks t ON t.id = ut.task_id
 WHERE t.scope = 'achievement'
   AND ut.task_date <> DATE '1970-01-01'
 GROUP BY ut.user_id, ut.task_id
ON CONFLICT (user_id, task_id, task_date) DO UPDATE
   SET progress  = GREATEST(user_tasks.progress, EXCLUDED.progress),
       claimed_at = LEAST(user_tasks.claimed_at, EXCLUDED.claimed_at);

-- 历史行已合并进哨兵行，删掉以免 `bumpTasks` 的 GREATEST
-- 把「今天的低关号」与「历史高关号」反复比较（虽仍正确，但没必要留）。
DELETE FROM user_tasks ut
 USING tasks t
 WHERE t.id = ut.task_id
   AND t.scope = 'achievement'
   AND ut.task_date <> DATE '1970-01-01';

-- +goose Down

-- 回滚成「按天分行」：把哨兵行拆成一行，日期取今天。
-- ⚠️ 不可逆：历史周期的行已经无法还原，
-- 而 progress 会被压成「终身最大值」当成今天的进度。
INSERT INTO user_tasks (user_id, task_id, task_date, progress, claimed_at)
SELECT user_id, task_id, CURRENT_DATE, progress, claimed_at
  FROM user_tasks
 WHERE task_date = DATE '1970-01-01'
ON CONFLICT (user_id, task_id, task_date) DO UPDATE
   SET progress  = GREATEST(user_tasks.progress, EXCLUDED.progress),
       claimed_at = LEAST(user_tasks.claimed_at, EXCLUDED.claimed_at);

DELETE FROM user_tasks WHERE task_date = DATE '1970-01-01';
