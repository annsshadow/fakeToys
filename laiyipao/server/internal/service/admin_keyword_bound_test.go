package service

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// 后台搜索词（`?keyword=`）的长度上限（第 154 轮）。
//
// # 缺陷
//
// `AdminListLevels` / `AdminListUsers` 的 keyword 此前**零长度校验**，
// 直接拼成 `%<kw>%` 交给 LIKE。而 `nickname` / `levels.name` 都是 TEXT，
// 上界只由 fasthttp 的 header 上限（4KB）间接决定。
//
// 触发它不需要恶意：运营从工单或聊天记录里**粘贴一整段话**进去，
// 服务端老实拿整段去全表 LIKE，必然空集 —— 运营据此得出
// 「查无此人」，而真实原因是「整段被当成一个整体去匹配」。
// 这正是本项目反复修的「静默降级而不是失败」。
//
// # 为什么这一组用例必须同时覆盖两个函数
//
// 转义与上界是**两件事**，而它们现在的处境相同：转义在两个文件里
// 各写了一遍（两份 Replacer 拷贝，见 trimAdminKeyword 的注释）。
// 只在 AdminListUsers 上加断言，AdminListLevels 照样可以退化 ——
// 那正是「守卫覆盖了一个结构、漏了同族的另一个」。
func TestAdminKeywordLengthIsCapped(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 上限本身必须能过（否则就是「把正常输入也拒了」的另一种错）。
	atLimit := strings.Repeat("字", MaxAdminKeywordLen)
	if n := len([]rune(atLimit)); n != MaxAdminKeywordLen {
		t.Fatalf("前置条件：构造出的搜索词应是 %d 字，实际 %d", MaxAdminKeywordLen, n)
	}
	overLimit := atLimit + "字"

	// 判据是「返回 ErrBadInput」而不是「返回空结果」：
	// 静默回空集就是上面说的那种「看起来查过了」。
	t.Run("AdminListUsers 超长 → ErrBadInput", func(t *testing.T) {
		if _, _, err := ts.AdminListUsers(ctx, overLimit, 50, 0); !errors.Is(err, ErrBadInput) {
			t.Errorf("超长搜索词应被 %v 拒绝，实际 %v（静默回空集 = 运营以为查无此人）", ErrBadInput, err)
		}
	})
	t.Run("AdminListLevels 超长 → ErrBadInput", func(t *testing.T) {
		if _, _, err := ts.AdminListLevels(ctx, 0, overLimit); !errors.Is(err, ErrBadInput) {
			t.Errorf("超长搜索词应被 %v 拒绝，实际 %v", ErrBadInput, err)
		}
	})
	t.Run("上限恰好不拒（边界是闭的）", func(t *testing.T) {
		if _, _, err := ts.AdminListUsers(ctx, atLimit, 50, 0); err != nil {
			t.Errorf("恰好 %d 字符应放行，实际 %v", MaxAdminKeywordLen, err)
		}
		if _, _, err := ts.AdminListLevels(ctx, 0, atLimit); err != nil {
			t.Errorf("恰好 %d 字符应放行，实际 %v", MaxAdminKeywordLen, err)
		}
	})
}

// 判据必须落在**归一化之后**：先 trim 再数长度。
//
// ⚠️ 这一条不是补边界，而是在钉住一个顺序：把「超长判定」写在
// `TrimSpace` **之前**，上面那组用例照样全绿 —— 而运营从聊天记录里
// 复制昵称时几乎必然带上尾随空格/换行，那些字符会把一个合法昵称
// 顶过上限，于是他搜索自己刚复制的昵称，收到「搜索词太长」。
func TestAdminKeywordLengthIgnoresSurroundingWhitespace(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 前后各塞 200 个空白：trim 之后是 5 个字符，必须放行。
	padded := strings.Repeat(" ", 100) + "张小明" + strings.Repeat("\n", 100)
	if _, _, err := ts.AdminListUsers(ctx, padded, 50, 0); err != nil {
		t.Errorf("前后空白不该计入长度（先 trim 再数）：%v", err)
	}
	if _, _, err := ts.AdminListLevels(ctx, 0, padded); err != nil {
		t.Errorf("前后空白不该计入长度（先 trim 再数）：%v", err)
	}
}

// 长度判据按**字符**而不是字节。
//
// 一个 64 个汉字的搜索词是 192 字节。按字节判会把上限实际压到 21 个
// 汉字 —— 那是「按字符写文档、按字节执行」的典型漂移，而且它只对
// 中文用户生效（英文用户看到的上限是 64）。
func TestAdminKeywordLengthCountsRunesNotBytes(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	kw := strings.Repeat("字", MaxAdminKeywordLen) // 192 字节
	if len(kw) <= MaxAdminKeywordLen {
		t.Fatalf("前置条件：%d 个汉字应超过 %d 字节，实际 %d", MaxAdminKeywordLen, MaxAdminKeywordLen, len(kw))
	}
	if _, _, err := ts.AdminListUsers(ctx, kw, 50, 0); err != nil {
		t.Errorf("%d 个汉字（%d 字节）在字符口径下恰好达上限，应放行：%v",
			MaxAdminKeywordLen, len(kw), err)
	}
}
