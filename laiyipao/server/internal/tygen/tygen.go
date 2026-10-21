// Package tygen generates TypeScript interfaces from Go response types.
//
// # 为什么要有它
//
// 后台此前 15 个端点的响应是手写的 `map[string]any` —— 形状由裸 SQL 与 map
// 字面量共同决定，**前后端两侧都没有事实源**。于是：
//
//  1. 前端用 `Record<string, unknown>` 承接，字段名拼错**不报错**；
//  2. 前端测试的 fixture 按前端形状写，于是「列恒空」测出来是绿的。
//
// 本包把 Go 的响应 struct 变成 TypeScript interface，让两侧的形状**由同一份
// 定义派生**。关键性质：**改一个 json tag，两端同时红**。
//
// # 为什么不手写生成器而用反射
//
// 手写一个 Go→TS 的解析器要处理标签、嵌入、泛型、递归…反射把这些全免了：
// Go 的 `encoding/json` 用的就是同一套规则（json tag 优先、`-` 跳过、
// 匿名嵌入 struct 平铺），所以反射的结果**与 json.Marshal 的行为一致** ——
// 这正是我们要的：TS 类型必须描述**线上真实的 JSON**。
//
// # 边界
//
//   - `map[string]any` → `{ [key: string]: unknown }`（不猜 value 类型）；
//   - `any` / `interface{}` → `unknown`；
//   - `[]byte` → `string`（Go 的 json 把 []byte 编码成 base64 字符串）；
//   - `time.Time` → `string`（RFC3339）；
//   - 数字一律 `number`（int64 超过 2^53 的问题由**服务端改用字符串下发**
//     解决，见 seed 字段的注释 —— 生成器不替业务做这个决定）。
package tygen

import (
	"fmt"
	"reflect"
	"sort"
	"strings"
	"time"
)

// Type 是一个已注册的类型：TS 里的名字 + Go 里的类型。
type Type struct {
	Name string
	Go   any // a value of the type, e.g. service.AdminLevelView{}
}

// Registry 是要生成的类型集合。
type Registry []Type

// tsTypeOf 把一个 Go 类型映射成 TS 类型表达式。
//
// `named` 给出「该 Go 类型在注册表里的 TS 名」；找不到时返回 ""，
// 表示「这是一个匿名 struct / 未注册的具名类型」→ 由调用方决定是否内联展开。
func tsTypeOf(t reflect.Type, named map[reflect.Type]string) string {
	if t == nil {
		return "unknown"
	}
	// time.Time 是特例：它不是 struct 形状，编码成 RFC3339 字符串。
	if t == reflect.TypeOf(time.Time{}) {
		return "string"
	}
	switch t.Kind() {
	case reflect.Pointer:
		return tsTypeOf(t.Elem(), named) + " | null"
	case reflect.String:
		return "string"
	case reflect.Bool:
		return "boolean"
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64,
		reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64,
		reflect.Float32, reflect.Float64:
		return "number"
	case reflect.Slice:
		// []byte 走 base64 字符串
		if t.Elem().Kind() == reflect.Uint8 {
			return "string"
		}
		return tsTypeOf(t.Elem(), named) + "[]"
	case reflect.Array:
		return tsTypeOf(t.Elem(), named) + "[]"
	case reflect.Map:
		if t.Key().Kind() != reflect.String {
			return "unknown"
		}
		return "{ [key: string]: " + tsTypeOf(t.Elem(), named) + " }"
	case reflect.Interface:
		return "unknown"
	case reflect.Struct:
		if name, ok := named[t]; ok {
			return name
		}
		return "" // anonymous: caller inlines it
	}
	return "unknown"
}

// fieldInfo 是一个要输出的字段。
type fieldInfo struct {
	tsName   string
	tsType   string
	optional bool
	comment  []string
}

// structFields 解析一个 struct 的字段（按 Go 的 json 规则）。
func structFields(t reflect.Type, named map[reflect.Type]string) ([]fieldInfo, error) {
	if t.Kind() == reflect.Pointer {
		t = t.Elem()
	}
	var out []fieldInfo
	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		if f.PkgPath != "" && !f.Anonymous {
			continue // unexported
		}
		tag := f.Tag.Get("json")
		if tag == "-" {
			continue
		}
		name, opts, _ := strings.Cut(tag, ",")
		optional := strings.Contains(opts, "omitempty")

		// 匿名嵌入 struct（无 json tag）→ 平铺它的字段。
		if f.Anonymous && name == "" {
			ft := f.Type
			for ft.Kind() == reflect.Pointer {
				ft = ft.Elem()
			}
			if ft.Kind() == reflect.Struct && ft != reflect.TypeOf(time.Time{}) {
				sub, err := structFields(ft, named)
				if err != nil {
					return nil, err
				}
				out = append(out, sub...)
				continue
			}
		}
		if name == "" {
			name = f.Name
		}
		ft := f.Type
		ts := tsTypeOf(ft, named)
		if ts == "" {
			// 未注册的匿名 struct → 内联展开
			inline, err := inlineObject(ft, named)
			if err != nil {
				return nil, err
			}
			ts = inline
		}
		out = append(out, fieldInfo{
			tsName:   name,
			tsType:   ts,
			optional: optional,
			comment:  commentLines(f),
		})
	}
	return out, nil
}

// inlineObject 把一个匿名 struct 内联成 TS 对象字面量类型。
func inlineObject(t reflect.Type, named map[reflect.Type]string) (string, error) {
	if t.Kind() == reflect.Pointer {
		t = t.Elem()
	}
	fields, err := structFields(t, named)
	if err != nil {
		return "", err
	}
	parts := make([]string, 0, len(fields))
	for _, f := range fields {
		parts = append(parts, f.tsName+": "+f.tsType)
	}
	return "{ " + strings.Join(parts, "; ") + " }", nil
}

// commentLines 提取字段的文档注释（Go doc 风格），作为 TS 注释输出。
func commentLines(f reflect.StructField) []string {
	doc := f.Tag.Get("doc")
	if doc == "" {
		return nil
	}
	return strings.Split(doc, "\n")
}

// Generate 产出 TS 文本。
func (r Registry) Generate(header string) (string, error) {
	named := map[reflect.Type]string{}
	for _, e := range r {
		t := reflect.TypeOf(e.Go)
		for t.Kind() == reflect.Pointer {
			t = t.Elem()
		}
		if _, dup := named[t]; dup {
			return "", fmt.Errorf("注册表里 %s 重复了", e.Name)
		}
		named[t] = e.Name
	}

	var b strings.Builder
	b.WriteString(header)
	b.WriteString("\n")

	// 按名字排序 → 生成物稳定，diff 只反映真实变化。
	entries := append(Registry(nil), r...)
	sort.Slice(entries, func(i, j int) bool { return entries[i].Name < entries[j].Name })

	for _, e := range entries {
		t := reflect.TypeOf(e.Go)
		for t.Kind() == reflect.Pointer {
			t = t.Elem()
		}
		if t.Kind() != reflect.Struct {
			return "", fmt.Errorf("%s 不是 struct（是 %s）", e.Name, t.Kind())
		}
		fields, err := structFields(t, named)
		if err != nil {
			return "", fmt.Errorf("%s: %w", e.Name, err)
		}
		fmt.Fprintf(&b, "export interface %s {\n", e.Name)
		for _, f := range fields {
			for _, c := range f.comment {
				b.WriteString("  // " + c + "\n")
			}
			opt := ""
			if f.optional {
				opt = "?"
			}
			fmt.Fprintf(&b, "  %s%s: %s\n", f.tsName, opt, f.tsType)
		}
		b.WriteString("}\n\n")
	}
	return b.String(), nil
}
