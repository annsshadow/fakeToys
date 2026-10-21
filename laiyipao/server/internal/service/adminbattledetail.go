package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"time"

	"github.com/jackc/pgx/v5"
)

// AdminBattleDetailView 是后台「战报详情」抽屉的响应。
//
// # 为什么它与 AdminBattleView 是两个类型
//
// 列表要的是**排序与筛选用的列**（分数、星级、验真状态…），
// 详情要的是**这一局到底发生了什么**（发射/命中、元素分布、反应分布、地形、
// 以及给运营重放用的**种子**）。
//
// 此前 `GET /admin/battles/:id` 返回的是 `ReplayInfo`（复现信息），
// 而前端抽屉按 `AdminBattle` 读 —— 于是 **14/16 个字段恒空**，
// 运营看到的是一片空白。两种形状都没有错，只是**用错了地方**。
//
// # 种子从哪来
//
// `battle_records` 自己不存 seed（那是开局凭证 `battle_tokens` 的东西），
// 但运营的验真流程**必须**有种子才能重放 → 这里 LEFT JOIN 出 `seed`。
// 没有凭证的战报（历史数据/凭证被清理）seed 为空字符串，抽屉显示「—」。
type AdminBattleDetailView struct {
	ID     int64 `json:"id"`
	UserID int64 `json:"user_id"`
	// Nickname 是玩家昵称（列表行也有，这里带上是为了抽屉能显示是谁）。
	Nickname string `json:"nickname"`
	LevelID  int    `json:"level_id"`
	Result   string `json:"result"`
	Stars    int    `json:"stars"`
	Score    int64  `json:"score"`
	Kills    int    `json:"kills"`
	Leaked   int    `json:"leaked"`
	// HpLeft 是结算后剩余的防线血量。
	HpLeft      int `json:"hp_left"`
	WaveReached int `json:"wave_reached"`
	DurationMs  int `json:"duration_ms"`
	// Shots / Hits 是发射与命中数 —— 命中率是「面板堆砌 vs 精准输出」的直接读数。
	Shots int `json:"shots"`
	Hits  int `json:"hits"`
	// Reactions 是反应触发总次数（I-1）。
	Reactions int `json:"reactions"`
	// HeatMax 是本局热量峰值（I-2）。
	HeatMax int `json:"heat_max"`
	// ElementsUsed / ReactionsUsed 是「用了什么」的分布（I-1 / I-4）。
	ElementsUsed  map[string]int64 `json:"elements_used"`
	ReactionsUsed map[string]int64 `json:"reactions_used"`
	// TerrainUsed 是本局触发的地形（I-4）。
	TerrainUsed []string `json:"terrain_used"`
	// ReplayHash 是结算时记录的哈希（I-6 的锚点）。
	ReplayHash string `json:"replay_hash"`
	// Seed 是重放所需的种子（字符串下发：int64 可能超过 2^53）。
	Seed string `json:"seed"`
	// PowerAfter 是结算后的战力。
	PowerAfter int64     `json:"power_after"`
	CreatedAt  time.Time `json:"created_at"`
}

// AdminBattleDetailResp 是 `GET /admin/battles/:id` 的响应。
type AdminBattleDetailResp struct {
	Battle AdminBattleDetailView `json:"battle"`
}

// AdminBattleDetail 读单场战报的详情。
//
// 运营侧**不加**归属过滤 —— 查任意玩家的战报本来就是它的职责。
func (s *Service) AdminBattleDetail(ctx context.Context, id int64) (AdminBattleDetailView, error) {
	var (
		b             AdminBattleDetailView
		elementsRaw   []byte
		reactionsRaw  []byte
		terrainRaw    []byte
		seed          *int64
		createdAt     time.Time
		elementsUsed  map[string]int64
		reactionsUsed map[string]int64
		terrainUsed   []string
	)
	err := s.pool.QueryRow(ctx, `
		SELECT br.id, br.user_id, u.nickname, br.level_id, br.result, br.stars,
		       br.score, br.kills, br.leaked, br.hp_left, br.wave_reached,
		       br.duration_ms, br.shots, br.hits, br.reactions, br.heat_max,
		       br.elements_used, br.reactions_used, br.terrain_used,
		       br.replay_hash, bt.seed, br.power_after, br.created_at
		FROM battle_records br
		JOIN users u ON u.id = br.user_id
		LEFT JOIN battle_tokens bt ON bt.id = br.battle_token_id
		WHERE br.id = $1`, id).Scan(
		&b.ID, &b.UserID, &b.Nickname, &b.LevelID, &b.Result, &b.Stars,
		&b.Score, &b.Kills, &b.Leaked, &b.HpLeft, &b.WaveReached,
		&b.DurationMs, &b.Shots, &b.Hits, &b.Reactions, &b.HeatMax,
		&elementsRaw, &reactionsRaw, &terrainRaw,
		&b.ReplayHash, &seed, &b.PowerAfter, &createdAt)
	if err != nil {
		// 2026-10-21：**区分「不存在」与「查不了」**。
		//
		// 此前这个端点走 `AdminGetReplay`，而后者把**任何**错误都报成
		// `ErrNotFound` —— 那是**玩家侧**的防预言机措施（不能让人二分出
		// 「某个 id 是否存在」）。
		//
		// 但运营端本来就能看全部战报列表，**没有预言机要防**；
		// 于是库一挂，运营看到的是「战报不存在」—— 一个**假的**答案。
		// 故障时该说的是「服务异常」，不是「这条记录没了」。
		if errors.Is(err, pgx.ErrNoRows) {
			return AdminBattleDetailView{}, fmt.Errorf("%w: battle %d", ErrNotFound, id)
		}
		return AdminBattleDetailView{}, fmt.Errorf("admin battle detail %d: %w", id, err)
	}
	// 第 149 轮同款：这三列是 jsonb，坏掉时**必须响**。
	// 静默给空会让运营以为「这一局没用任何元素 / 没触发任何反应 / 没有地形」——
	// 而真相是「数据坏了」。
	if err := json.Unmarshal(elementsRaw, &elementsUsed); err != nil {
		return AdminBattleDetailView{}, fmt.Errorf("admin battle detail %d: elements_used 非法 jsonb: %w", id, err)
	}
	if err := json.Unmarshal(reactionsRaw, &reactionsUsed); err != nil {
		return AdminBattleDetailView{}, fmt.Errorf("admin battle detail %d: reactions_used 非法 jsonb: %w", id, err)
	}
	if err := json.Unmarshal(terrainRaw, &terrainUsed); err != nil {
		return AdminBattleDetailView{}, fmt.Errorf("admin battle detail %d: terrain_used 非法 jsonb: %w", id, err)
	}
	b.ElementsUsed = elementsUsed
	b.ReactionsUsed = reactionsUsed
	b.TerrainUsed = terrainUsed
	b.CreatedAt = createdAt
	if seed != nil {
		// 种子以**字符串**下发：它是 64 位 LCG 值（如 -7046029255919282421），
		// 超过 2^53 后 JS Number 直接失真。玩家侧同族约定见 economy.go:734。
		b.Seed = strconv.FormatInt(*seed, 10)
	}
	return b, nil
}
