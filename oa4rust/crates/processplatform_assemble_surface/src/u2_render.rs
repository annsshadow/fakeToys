// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 渲染/转换族真实现（承 doc->word / html->pdf / preview pdf·image 的 501 桩）。
//!
//! 契约逐条对齐 o2server `x_processplatform_assemble_surface/attachment`：
//! - `docToWord`：o2 本地分支（`local()`）把 content（HTML 片段）包进
//!   OLE2/CFB 容器（POIFSFileSystem.createDocument("WordDocument")），Word 可
//!   直接打开显示。这里用 `cfb` crate 忠实复刻该行为。
//! - `htmlToPdf`：o2 用 iText html2pdf 本地排版。这里用 `genpdf` 做**简化排版**
//!   真实现：提取块级结构（h1-h6/p/div/li/tr/br）渲染为 A4 段落，产出真实 PDF、
//!   内容完整，但非浏览器级排版保真——不谎称保真，超出简化排版能力的样式会被丢弃。
//! - `previewPdf`/`previewImage`：o2 该路径把附件 POST 给 **O2 云转换服务**
//!   （`DocumentTools.toPdf/toImage` → collect 服务器），离线环境 o2 自身也不可用。
//!
//!   这里按附件类型本地分级：PDF 原样透传 / 图片重编码或嵌入 PDF；无法本地转换
//!   的类型（office 文档等）保持 501——与「不造假」纪律一致。
//!
//! 产出统一落 GeneralFile 表（content=base64，flag 与 id 同值），返回 `{id}`。

use ab_glyph::{Font as _, ScaleFont as _};
use base64::Engine as _;
use std::io::{Cursor, Write as _};

use axum::Json;
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;


/// GeneralFile 落盘并返回 `{id}`（flag 与 id 同值）。
pub async fn general_file_store(
    pool: &Pool,
    person: &str,
    name: &str,
    bytes: &[u8],
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    let content_b64 = base64::engine::general_purpose::STANDARD.encode(bytes);
    let size = bytes.len() as i64;
    client
        .execute(
            "INSERT INTO x_general_assemble_general_file (id, name, flag, content, size, creator, create_time) \
             VALUES ($1, $2, $3, $4, $5, $6, NOW())",
            &[&id, &name, &id, &content_b64, &size, &person],
        )
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, "general file insert failed");
            AppError::Internal
        })?;
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("id".to_string(), Value::String(id))]),
    ))))
}

/// HTML 片段包进 OLE2/CFB 容器（复刻 o2 `ActionDocToWord.local()`：
/// POIFSFileSystem.createDocument(is, "WordDocument")）。Word 打开显示 HTML 内容。
pub fn html_to_word_binary(content: &str) -> Result<Vec<u8>, AppError> {
    let html = format!("<html><head></head><body>{content}</body></html>");
    let cursor = Cursor::new(Vec::new());
    let mut cfb = cfb::OpenOptions::new().create_with(cursor).map_err(|e| {
        tracing::warn!(error = %e, "cfb container create failed");
        AppError::Internal
    })?;
    {
        let mut stream = cfb
            .create_stream("WordDocument")
            .map_err(|e| {
                tracing::warn!(error = %e, "cfb stream create failed");
                AppError::Internal
            })?;
        stream
            .write_all(html.as_bytes())
            .map_err(|_| AppError::Internal)?;
    }
    Ok(cfb.into_inner().into_inner())
}

/// 极简 HTML 实体解码（正文提取所需的最小集）。
fn decode_entities(text: &str) -> String {
    text.replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// 块级 HTML → (样式标记, 纯文本) 行序列。
/// 只认 o2 表单/文档常见的块级标签；其余标签剥壳保留文字（诚实简化）。
pub fn html_blocks(html: &str) -> Vec<(u8, String)> {
    // 样式标记：0=正文 1..=6=h1..h6
    let bytes = html.as_bytes();
    let mut blocks = Vec::new();
    let mut current = String::new();
    let mut current_style: u8 = 0;
    let mut heading_depth: u8 = 0;
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'<' {
            // 找到 '>'，取出标签文本（小写比较；字节扫描不落 current）
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'>' {
                j += 1;
            }
            let tag = html[i + 1..j.min(html.len())].trim().to_lowercase();
            let is_close = tag.starts_with('/');
            let name: String = tag
                .trim_start_matches('/')
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric())
                .collect();
            match name.as_str() {
                "script" | "style" => {
                    // 跳过整块内容直到对应闭合标签
                    let close = format!("</{}", name);
                    let lower_rest = html[j..].to_lowercase();
                    if let Some(pos) = lower_rest.find(&close) {
                        i = j + pos + close.len();
                        // 吃到 '>'
                        while i < bytes.len() && bytes[i] != b'>' {
                            i += 1;
                        }
                    }
                    i += 1;
                    continue;
                }
                "br" => {
                    current.push('\n');
                    i = j + 1;
                    continue;
                }
                "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                    // 标题文本已挂在 current：flush 必须发生在样式重置之前
                    let text = current.trim().to_string();
                    if !text.is_empty() {
                        blocks.push((current_style, decode_entities(&text)));
                    }
                    current.clear();
                    if is_close {
                        heading_depth = 0;
                        current_style = 0;
                    } else {
                        let depth: u8 = name[1..].parse().unwrap_or(0);
                        heading_depth = depth;
                        current_style = depth;
                    }
                    i = j + 1;
                    continue;
                }
                "p" | "div" | "li" | "tr" | "table" => {
                    let text = current.trim().to_string();
                    if !text.is_empty() {
                        blocks.push((current_style, decode_entities(&text)));
                    }
                    current.clear();
                    if !is_close {
                        current_style = heading_depth;
                    }
                    i = j + 1;
                    continue;
                }
                _ => {
                    i = j + 1;
                    continue;
                }
            }
        }
        // UTF-8 安全：复制完整字符
        let ch_len = utf8_char_len(bytes[i]);
        current.push_str(&html[i..i + ch_len]);
        i += ch_len;
    }
    let text = decode_entities(current.trim());
    if !text.is_empty() {
        blocks.push((current_style, text));
    }
    blocks
}

fn utf8_char_len(b: u8) -> usize {
    match b {
        0x00..=0x7F => 1,
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1,
    }
}

/// 探测可用中文字体（pdf 排版嵌入）。找不到返回 None → 调用方 501（诚实：
/// 无字体时产出的 PDF 会丢字，不做静默丢字输出）。
pub fn probe_cjk_font() -> Option<Vec<u8>> {
    for path in [
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/System/Library/Fonts/PingFang.ttc",
    ] {
        if let Ok(bytes) = std::fs::read(path) {
            return Some(bytes);
        }
    }
    None
}

/// 简化排版 HTML → PDF 字节。无可用 CJK 字体时返回 None（调用方 501）。
pub fn html_to_pdf_bytes(html: &str) -> Result<Option<Vec<u8>>, AppError> {
    let Some(font_bytes) = probe_cjk_font() else {
        return Ok(None);
    };
    // bold/italic/合成体统一复用 regular 字形（简化排版无字重变化）
    let mk_font = || {
        genpdf::fonts::FontData::new(font_bytes.clone(), None).map_err(|_| AppError::Internal)
    };
    let family = genpdf::fonts::FontFamily {
        regular: mk_font()?,
        bold: mk_font()?,
        italic: mk_font()?,
        bold_italic: mk_font()?,
    };
    let mut doc = genpdf::Document::new(family);
    doc.set_title("oa4rust simplified html render");
    for (style, text) in html_blocks(html) {
        let font_size = match style {
            1 => 24,
            2 => 20,
            3 => 17,
            4 => 15,
            5 => 13,
            6 => 12,
            _ => 11,
        };
        let para = genpdf::elements::Paragraph::new(genpdf::style::StyledString::new(
            text,
            genpdf::style::Style::new().bold().with_font_size(font_size),
        ));
        doc.push(para);
        doc.push(genpdf::elements::Paragraph::new(genpdf::style::StyledString::new(
            " ",
            genpdf::style::Style::new().with_font_size(6),
        )));
    }
    let mut out = Vec::new();
    doc.render(&mut out)
        .map_err(|e| {
            tracing::warn!(error = %e, "pdf render failed");
            AppError::Internal
        })?;
    Ok(Some(out))
}

/// 附件类型分级：按魔数判定（扩展名仅供参考）。
pub enum AttKind {
    Pdf,
    Image(image::ImageFormat),
    Other,
}

pub fn classify_attachment(name: &str, bytes: &[u8]) -> AttKind {
    if bytes.starts_with(b"%PDF") {
        return AttKind::Pdf;
    }
    if let Ok(format) = image::guess_format(bytes) {
        return AttKind::Image(format);
    }
    let lower = name.to_lowercase();
    if lower.ends_with(".pdf") {
        return AttKind::Pdf;
    }
    AttKind::Other
}

/// 图片重编码为 PNG（预览用统一格式）。
pub fn image_to_png(bytes: &[u8]) -> Result<Option<Vec<u8>>, AppError> {
    let img = image::load_from_memory(bytes)
        .map_err(|e| {
            tracing::warn!(error = %e, "image decode failed");
            AppError::BadRequest("attachment is not a decodable image".to_string())
        })?;
    let mut out = Vec::new();
    img.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png)
        .map_err(|_| AppError::Internal)?;
    Ok(Some(out))
}

/// 简化排版 HTML → PNG：块级结构 + ab_glyph 文本光栅化（白底黑字，标题字号大）。
/// 与 html_to_pdf_bytes 同构的「简化排版真实现」——内容完整、非浏览器级保真。
/// 无可用 CJK 字体时返回 None（调用方 501，不产出丢字图）。
pub fn html_to_image_bytes(html: &str) -> Result<Option<Vec<u8>>, AppError> {
    let Some(font_bytes) = probe_cjk_font() else {
        return Ok(None);
    };
    // ttc 集合取第 0 面；ttf 直取
    let font = match ab_glyph::FontRef::try_from_slice(&font_bytes) {
        Ok(f) => f,
        Err(_) => ab_glyph::FontRef::try_from_slice_and_index(&font_bytes, 0)
            .map_err(|e| {
                tracing::warn!(error = %e, "font parse failed");
                AppError::Internal
            })?,
    };
    const WIDTH: u32 = 1240; // A4@150dpi
    let blocks = html_blocks(html);
    if blocks.is_empty() {
        return Ok(None);
    }
    // 先排版算行高，再开画布
    struct Line { size: f32, text: String }
    let mut lines: Vec<Line> = Vec::new();
    for (style, text) in &blocks {
        let size = match style {
            1 => 48.0,
            2 => 40.0,
            3 => 34.0,
            4 => 30.0,
            5 => 26.0,
            6 => 24.0,
            _ => 22.0,
        };
        // 按宽度硬折行（CJK 逐字；ASCII 词保持完整切分粒度足够简化排版）
        let scaled = font.as_scaled(size);
        let mut line = String::new();
        let mut width_px = 0.0f32;
        for ch in text.chars() {
            let advance = scaled.h_advance(scaled.glyph_id(ch));
            if width_px + advance > (WIDTH - 80) as f32 && !line.is_empty() {
                lines.push(Line { size, text: std::mem::take(&mut line) });
                width_px = 0.0;
            }
            line.push(ch);
            width_px += advance;
        }
        if !line.is_empty() {
            lines.push(Line { size, text: line });
        }
    }
    let line_height_total: f32 = lines.iter().map(|l| l.size * 1.5).sum();
    let height = (line_height_total as u32 + 80).max(200);
    let mut canvas = image::RgbaImage::from_pixel(WIDTH, height, image::Rgba([255, 255, 255, 255]));
    let mut y = 40.0f32;
    for line in &lines {
        let scaled = font.as_scaled(line.size);
        let mut x = 40.0f32;
        for ch in line.text.chars() {
            let mut glyph = scaled.scaled_glyph(ch);
            glyph.position = ab_glyph::point(x, y + line.size); // baseline
            if let Some(outlined) = scaled.outline_glyph(glyph) {
                outlined.draw(|px, py, coverage| {
                    let (px, py) = (px as i64, py as i64);
                    if px < 0 || py < 0 || px >= WIDTH as i64 || py >= height as i64 {
                        return;
                    }
                    let a = (coverage * 255.0) as u8;
                    if a == 0 {
                        return;
                    }
                    // 黑字 alpha 合成
                    let pixel = canvas.get_pixel_mut(px as u32, py as u32);
                    let inv = 255 - a;
                    let blend = |c: u8| ((c as u16 * inv as u16) / 255) as u8;
                    *pixel = image::Rgba([blend(255), blend(255), blend(255), 255]);
                });
            }
            x += scaled.h_advance(scaled.glyph_id(ch));
        }
        y += line.size * 1.5;
    }
    let mut out = Vec::new();
    image::DynamicImage::ImageRgba8(canvas)
        .write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png)
        .map_err(|_| AppError::Internal)?;
    Ok(Some(out))
}

/// 图片嵌入单页 PDF（A4，按宽适配）。
pub fn image_to_pdf_bytes(bytes: &[u8]) -> Result<Option<Vec<u8>>, AppError> {
    let Some(font_bytes) = probe_cjk_font() else {
        return Ok(None);
    };
    let img = image::load_from_memory(bytes)
        .map_err(|_| AppError::BadRequest("attachment is not a decodable image".to_string()))?;
    // genpdf Image 不支持 alpha；预缩放到 170mm@300dpi 内，避免巨图溢出 A4
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    let max_px = ((170.0 / 25.4) * 300.0) as u32;
    let scaled = if w > max_px {
        let ratio = max_px as f64 / w as f64;
        image::DynamicImage::ImageRgb8(rgb).resize_exact(
            max_px,
            ((h as f64 * ratio) as u32).max(1),
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        image::DynamicImage::ImageRgb8(rgb)
    };
    // bold/italic/合成体统一复用 regular 字形（简化排版无字重变化）
    let mk_font = || {
        genpdf::fonts::FontData::new(font_bytes.clone(), None).map_err(|_| AppError::Internal)
    };
    let family = genpdf::fonts::FontFamily {
        regular: mk_font()?,
        bold: mk_font()?,
        italic: mk_font()?,
        bold_italic: mk_font()?,
    };
    let mut doc = genpdf::Document::new(family);
    let image = genpdf::elements::Image::from_dynamic_image(scaled).map_err(|e| {
        tracing::warn!(error = %e, "image embed failed");
        AppError::BadRequest("attachment is not an embeddable image".to_string())
    })?;
    doc.push(image);
    let mut out = Vec::new();
    doc.render(&mut out).map_err(|_| AppError::Internal)?;
    Ok(Some(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn cfb_container_round_trips() {
        let bytes = html_to_word_binary("<p>hello 你好</p>").unwrap();
        assert!(bytes.len() > 512, "CFB sector-aligned container");
        assert_eq!(&bytes[..8], b"\xD0\xCF\x11\xE0\xA1\xB1\x1A\xE1", "OLE2 magic");
        // 可读回
        let mut compound = cfb::OpenOptions::new().open_with(Cursor::new(&bytes)).unwrap();
        let mut back = Vec::new();
        use std::io::Read;
        compound
            .open_stream("WordDocument")
            .unwrap()
            .read_to_end(&mut back)
            .unwrap();
        assert_eq!(String::from_utf8(back).unwrap(), "<html><head></head><body><p>hello 你好</p></body></html>");
    }

    #[test]
    fn html_blocks_extracts_headings_and_paragraphs() {
        let blocks = html_blocks(
            "<h1>标题</h1><p>第一段 &amp; 实体</p><div>第二段</div><script>evil()</script>",
        );
        assert_eq!(blocks[0], (1, "标题".to_string()));
        assert_eq!(blocks[1], (0, "第一段 & 实体".to_string()));
        assert_eq!(blocks[2], (0, "第二段".to_string()));
        assert_eq!(blocks.len(), 3, "script 内容被剔除");
    }

    #[test]
    fn classify_by_magic_not_extension() {
        let mut fake = b"%.PNG not really".to_vec();
        fake.extend_from_slice(&[0u8; 32]);
        assert!(matches!(classify_attachment("a.png", &fake), AttKind::Other));
        let pdf = b"%PDF-1.7 fake".to_vec();
        assert!(matches!(classify_attachment("a.txt", &pdf), AttKind::Pdf));
    }

    #[tokio::test]
    async fn html_to_image_renders_real_png() {
        match html_to_image_bytes("<h1>标题</h1><p>正文内容 body</p>") {
            Ok(Some(bytes)) => {
                assert_eq!(&bytes[1..4], b"PNG", "真实 PNG 签名");
                let decoded = image::load_from_memory(&bytes).unwrap();
                assert!(decoded.dimensions().0 >= 1240, "A4@150dpi 画布宽");
            }
            // 无字体环境（CI）诚实降级
            Ok(None) => {}
            Err(e) => panic!("unexpected error: {e}"),
        }
    }

    #[tokio::test]
    async fn html_to_pdf_renders_with_system_font() {
        match html_to_pdf_bytes("<h1>合同标题</h1><p>正文内容</p>") {
            Ok(Some(bytes)) => assert!(bytes.starts_with(b"%PDF")),
            // 无中文字体环境（CI）诚实降级：不出丢字 PDF
            Ok(None) => {}
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}
