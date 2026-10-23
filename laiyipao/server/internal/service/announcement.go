package service

import (
	"context"
	"fmt"
	"time"
)

// AnnouncementView 是**玩家侧**的公告。
//
// # 为什么它不与 AdminAnnouncementView 复用
//
// 后台那一份要带 `published` 与 `created_at`（运营要看草稿、要看创建时间）；
// 玩家侧只可能拿到 `published = true` 的行，回一个恒为 true 的布尔
// 是把后台的形状泄漏到玩家端 —— 客户端会开始写 `if (a.published)`，
// 而那条件永远成立，属于本仓库反复要消灭的「看起来在判断，其实没判断」。
//
// `body` 原样下发（不做 Markdown 渲染）：小程序侧没有 Markdown 组件，
// 富文本要靠 rich-text，那是另一个渲染面。先按纯文本，换行保留。
type AnnouncementView struct {
	ID    int    `json:"id"`
	Title string `json:"title"`
	Body  string `json:"body"`
	// PublishedAt 是发布时间。正常非空；`NULLS LAST` 排序兜住历史脏数据。
	PublishedAt *time.Time `json:"published_at"`
}

// 玩家侧公告的条数上限。
//
// 取 5 与 20 两个数的理由不同：
//   - 缺省 5：首页卡片最多铺 3 条（再多会把「开始关卡」挤出首屏），
//     留 2 条余量给客户端自行取舍，服务端不替它决定展示几条。
//   - 上限 20：与后台列表的 500 不同 —— 玩家侧是**公网端点且无鉴权**，
//     一条 SQL 返回多少字节由请求方决定，必须夹死。
const (
	defaultAnnouncementLimit = 5
	maxAnnouncementLimit     = 20
)

// ListAnnouncements 返回**已发布**的公告，按发布时间倒序。
//
// 判据（`announcement_test.go`）：
//   - 未发布的绝不出现（`WHERE published`）；
//   - `limit` 越界**夹到边界**而不是回落到缺省值 —— 这是两件事：
//     `?limit=100` 期望「尽可能多」，给它 5 条是**比请求更少、也比上限更少**，
//     且不报错。第一版就是 `limit <= 0 || limit > max { limit = default }`，
//     于是 100 和 0 得到同一个结果，判据也区分不出来；
//   - 排序按 `published_at DESC NULLS LAST, id DESC`：
//     同一秒发布的两条靠 id 稳定排序，否则分页/重取会抖动。
func (s *Service) ListAnnouncements(ctx context.Context, limit int) ([]AnnouncementView, error) {
	switch {
	case limit <= 0:
		// 没传 / 传了非正数 → 缺省条数
		limit = defaultAnnouncementLimit
	case limit > maxAnnouncementLimit:
		// 超出上限 → **夹到上限**，不是退回缺省
		limit = maxAnnouncementLimit
	}
	rows, err := s.pool.Query(ctx, `
		SELECT id, title, body, published_at
		FROM announcements
		WHERE published
		ORDER BY published_at DESC NULLS LAST, id DESC
		LIMIT $1`, limit)
	if err != nil {
		return nil, fmt.Errorf("list announcements: %w", err)
	}
	defer rows.Close()

	// 非 nil 空切片：`[]` 而不是 `null`。
	// 客户端对 `null` 调 `.length` 会抛，而「没有公告」是完全正常的状态。
	out := make([]AnnouncementView, 0, limit)
	for rows.Next() {
		var a AnnouncementView
		if err := rows.Scan(&a.ID, &a.Title, &a.Body, &a.PublishedAt); err != nil {
			return nil, fmt.Errorf("scan announcement: %w", err)
		}
		out = append(out, a)
	}
	return out, rows.Err()
}
