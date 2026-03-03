use std::path::{Path, PathBuf};

use eframe::egui;
use egui::text::{LayoutJob, TextFormat};
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, vec2};
use rfd::FileDialog;

use crate::io::{read_text_with_fallback, write_text_utf8};
use crate::parser::parse_markdown;
use crate::{Block, ListKind, Span};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReaderTheme {
    GitHub,
    Typora,
    Compact,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum WorkspaceMode {
    Read,
    Edit,
    Split,
}

impl WorkspaceMode {
    fn all() -> [Self; 3] {
        [Self::Read, Self::Edit, Self::Split]
    }

    fn label(self) -> &'static str {
        match self {
            Self::Read => "阅读",
            Self::Edit => "编辑",
            Self::Split => "分栏",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum PreviewMode {
    Desktop,
    Mobile,
}

impl PreviewMode {
    fn all() -> [Self; 2] {
        [Self::Desktop, Self::Mobile]
    }

    fn label(self) -> &'static str {
        match self {
            Self::Desktop => "桌面预览",
            Self::Mobile => "移动预览",
        }
    }

    fn max_width(self, desktop_width: f32) -> f32 {
        match self {
            Self::Desktop => desktop_width,
            Self::Mobile => 390.0,
        }
    }
}

impl ReaderTheme {
    pub fn all() -> [Self; 3] {
        [Self::GitHub, Self::Typora, Self::Compact]
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::GitHub => "GitHub 风",
            Self::Typora => "Typora 风",
            Self::Compact => "紧凑风",
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ThemeMetrics {
    pub content_max_width: f32,
    pub body_size: f32,
    pub body_line_height: f32,
    pub code_size: f32,
    pub code_line_height: f32,
    pub paragraph_gap: f32,
    pub list_gap: f32,
    pub block_gap: f32,
    pub heading_sizes: [f32; 6],
    pub heading_line_heights: [f32; 6],
    pub heading_gaps: [f32; 6],
    pub table_cell_line_height: f32,
}

pub fn metrics_for_theme(theme: ReaderTheme) -> ThemeMetrics {
    match theme {
        ReaderTheme::GitHub => ThemeMetrics {
            content_max_width: 900.0,
            body_size: 16.0,
            body_line_height: 27.0,
            code_size: 14.0,
            code_line_height: 22.0,
            paragraph_gap: 14.0,
            list_gap: 11.0,
            block_gap: 12.0,
            heading_sizes: [34.0, 30.0, 25.0, 21.0, 18.5, 17.0],
            heading_line_heights: [44.0, 40.0, 34.0, 30.0, 28.0, 26.0],
            heading_gaps: [14.0, 12.0, 10.0, 9.0, 8.0, 8.0],
            table_cell_line_height: 24.0,
        },
        ReaderTheme::Typora => ThemeMetrics {
            content_max_width: 860.0,
            body_size: 17.0,
            body_line_height: 30.0,
            code_size: 14.5,
            code_line_height: 23.0,
            paragraph_gap: 16.0,
            list_gap: 12.0,
            block_gap: 14.0,
            heading_sizes: [36.0, 31.0, 26.0, 22.0, 19.0, 17.5],
            heading_line_heights: [48.0, 42.0, 36.0, 31.0, 28.0, 26.0],
            heading_gaps: [16.0, 13.0, 11.0, 10.0, 9.0, 8.0],
            table_cell_line_height: 26.0,
        },
        ReaderTheme::Compact => ThemeMetrics {
            content_max_width: 980.0,
            body_size: 15.0,
            body_line_height: 24.0,
            code_size: 13.0,
            code_line_height: 20.0,
            paragraph_gap: 9.0,
            list_gap: 8.0,
            block_gap: 9.0,
            heading_sizes: [30.0, 27.0, 23.0, 19.0, 17.0, 16.0],
            heading_line_heights: [38.0, 34.0, 30.0, 26.0, 24.0, 23.0],
            heading_gaps: [10.0, 9.0, 8.0, 7.0, 6.0, 6.0],
            table_cell_line_height: 22.0,
        },
    }
}

pub struct MarkdownReaderApp {
    current_file: Option<PathBuf>,
    markdown_text: String,
    saved_snapshot: String,
    blocks: Vec<Block>,
    dirty: bool,
    mode: WorkspaceMode,
    preview_mode: PreviewMode,
    edit_on_left: bool,
    status_message: Option<String>,
    last_error: Option<String>,
    style_initialized: bool,
    theme: ReaderTheme,
    metrics: ThemeMetrics,
}

impl Default for MarkdownReaderApp {
    fn default() -> Self {
        let theme = ReaderTheme::GitHub;
        Self {
            current_file: None,
            markdown_text: String::new(),
            saved_snapshot: String::new(),
            blocks: Vec::new(),
            dirty: false,
            mode: WorkspaceMode::Read,
            preview_mode: PreviewMode::Desktop,
            edit_on_left: true,
            status_message: None,
            last_error: None,
            style_initialized: false,
            theme,
            metrics: metrics_for_theme(theme),
        }
    }
}

impl MarkdownReaderApp {
    pub fn with_initial_file(path: Option<PathBuf>) -> Self {
        let mut app = Self::default();
        if let Some(path) = path {
            app.load_file(path);
        }
        app
    }

    fn open_file_dialog(&mut self) {
        let file = FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .pick_file();
        if let Some(path) = file {
            self.load_file(path);
        }
    }

    fn reload_current(&mut self) {
        if let Some(path) = self.current_file.clone() {
            self.load_file(path);
        }
    }

    fn load_file(&mut self, path: PathBuf) {
        match read_text_with_fallback(&path) {
            Ok(text) => {
                self.markdown_text = text.clone();
                self.saved_snapshot = text;
                self.blocks = parse_markdown(&self.markdown_text);
                self.dirty = false;
                self.current_file = Some(path.clone());
                self.status_message = Some(format!("已加载: {}", path.display()));
                self.last_error = None;
            }
            Err(err) => {
                self.status_message = None;
                self.last_error = Some(format!("{err}"));
            }
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        ctx.input(|input| {
            if input.modifiers.command && input.key_pressed(egui::Key::O) {
                self.open_file_dialog();
            }
            if input.modifiers.command && input.key_pressed(egui::Key::R) {
                self.reload_current();
            }
            if input.modifiers.command && input.key_pressed(egui::Key::S) {
                if input.modifiers.shift {
                    self.save_as_dialog();
                } else {
                    self.save_current();
                }
            }
        });
    }

    fn refresh_blocks(&mut self) {
        self.blocks = parse_markdown(&self.markdown_text);
    }

    fn refresh_dirty_state(&mut self) {
        self.dirty = self.markdown_text != self.saved_snapshot;
    }

    fn save_current(&mut self) {
        if let Some(path) = self.current_file.clone() {
            self.save_to_path(path);
        } else {
            self.save_as_dialog();
        }
    }

    fn save_as_dialog(&mut self) {
        let file = FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .set_file_name("untitled.md")
            .save_file();
        if let Some(path) = file {
            self.save_to_path(path);
        }
    }

    fn save_to_path(&mut self, path: PathBuf) {
        match write_text_utf8(&path, &self.markdown_text) {
            Ok(()) => {
                self.current_file = Some(path.clone());
                self.saved_snapshot = self.markdown_text.clone();
                self.refresh_blocks();
                self.refresh_dirty_state();
                self.last_error = None;
                self.status_message = Some(format!("已保存: {}", path.display()));
            }
            Err(err) => {
                self.status_message = None;
                self.last_error = Some(format!("save file failed: {err}"));
            }
        }
    }

    fn on_editor_changed(&mut self) {
        self.refresh_blocks();
        self.refresh_dirty_state();
        self.status_message = None;
    }

    fn render_editor_view(&mut self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::new()
            .fill(Color32::from_rgb(250, 252, 255))
            .stroke(Stroke::new(1.0, Color32::from_rgb(207, 215, 227)))
            .corner_radius(8.0)
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                ui.set_min_height(available.y.max(240.0));
                let editor = egui::TextEdit::multiline(&mut self.markdown_text)
                    .font(TextStyle::Monospace)
                    .desired_width(f32::INFINITY);
                let response = ui.add_sized([ui.available_width(), ui.available_height()], editor);
                if response.changed() {
                    self.on_editor_changed();
                }
            });
    }

    fn render_preview_body(&self, ui: &mut egui::Ui) {
        if self.blocks.is_empty() {
            ui.add_space(8.0);
            ui.label("Markdown 内容为空。");
            return;
        }

        let available = ui.available_width();
        let max_width = self.preview_mode.max_width(self.metrics.content_max_width);
        let content_width = available.min(max_width);
        let side = ((available - content_width) / 2.0).max(0.0);
        ui.horizontal(|ui| {
            ui.add_space(side);
            egui::Frame::new()
                .fill(Color32::from_rgb(255, 255, 255))
                .stroke(Stroke::new(1.0, Color32::from_rgb(212, 219, 228)))
                .corner_radius(10.0)
                .inner_margin(egui::Margin::symmetric(22, 18))
                .show(ui, |ui| {
                    ui.set_width(content_width);
                    for block in &self.blocks {
                        render_block(ui, block, &self.metrics);
                    }
                });
        });
    }

    fn render_preview_view(&self, ui: &mut egui::Ui) {
        let available = ui.available_size();
        egui::Frame::new()
            .fill(Color32::from_rgb(242, 246, 250))
            .stroke(Stroke::new(1.0, Color32::from_rgb(217, 224, 233)))
            .corner_radius(8.0)
            .inner_margin(egui::Margin::same(10))
            .show(ui, |ui| {
                ui.set_min_height(available.y.max(240.0));
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_preview_body(ui);
                });
            });
    }

    fn apply_theme(&mut self, ctx: &egui::Context, theme: ReaderTheme) {
        if self.theme != theme {
            self.theme = theme;
            self.metrics = metrics_for_theme(theme);
            apply_reader_style(ctx, &self.metrics);
        }
    }
}

pub fn default_cjk_font_candidates() -> Vec<PathBuf> {
    vec![
        PathBuf::from(r"C:\Windows\Fonts\msyh.ttc"),
        PathBuf::from(r"C:\Windows\Fonts\simhei.ttf"),
        PathBuf::from(r"C:\Windows\Fonts\simsunb.ttf"),
        PathBuf::from(r"C:\Windows\Fonts\Deng.ttf"),
        PathBuf::from(r"C:\Windows\Fonts\NotoSansSC-VF.ttf"),
    ]
}

pub fn default_ui_font_candidates() -> Vec<PathBuf> {
    vec![
        PathBuf::from(r"C:\Windows\Fonts\msyh.ttc"),
        PathBuf::from(r"C:\Windows\Fonts\segoeui.ttf"),
        PathBuf::from(r"C:\Windows\Fonts\Deng.ttf"),
        PathBuf::from(r"C:\Windows\Fonts\NotoSansSC-VF.ttf"),
    ]
}

pub fn default_monospace_font_candidates() -> Vec<PathBuf> {
    vec![
        PathBuf::from(r"C:\Windows\Fonts\consola.ttf"),
        PathBuf::from(r"C:\Windows\Fonts\simhei.ttf"),
    ]
}

pub fn first_existing_path(paths: &[PathBuf]) -> Option<PathBuf> {
    paths.iter().find(|p| p.exists()).cloned()
}

pub fn configure_cjk_fonts(ctx: &egui::Context) -> Option<PathBuf> {
    let mut fonts = FontDefinitions::default();

    let ui_path = first_existing_path(&default_ui_font_candidates());
    let cjk_path = first_existing_path(&default_cjk_font_candidates());
    let mono_path = first_existing_path(&default_monospace_font_candidates());

    let mut proportional_pref = Vec::new();
    let mut monospace_pref = Vec::new();

    let mut loaded_cjk = None;
    if let Some(path) = cjk_path
        && apply_font_path(&mut fonts, "cjk", &path).is_ok()
    {
        // Chinese first: prioritize CJK glyph style over Latin fallback.
        proportional_pref.push("cjk".to_owned());
        loaded_cjk = Some(path);
    }

    if let Some(path) = ui_path
        && apply_font_path(&mut fonts, "ui", &path).is_ok()
    {
        proportional_pref.push("ui".to_owned());
    }

    if let Some(path) = mono_path
        && apply_font_path(&mut fonts, "mono", &path).is_ok()
    {
        monospace_pref.push("mono".to_owned());
    }

    if loaded_cjk.is_some() {
        monospace_pref.push("cjk".to_owned());
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        for name in proportional_pref.into_iter().rev() {
            family.insert(0, name);
        }
    }

    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        for name in monospace_pref.into_iter().rev() {
            family.insert(0, name);
        }
    }

    ctx.set_fonts(fonts);
    loaded_cjk
}

fn apply_font_path(
    fonts: &mut FontDefinitions,
    name: &str,
    path: &Path,
) -> Result<(), std::io::Error> {
    let bytes = std::fs::read(path)?;
    fonts
        .font_data
        .insert(name.to_owned(), FontData::from_owned(bytes).into());
    Ok(())
}

impl eframe::App for MarkdownReaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.style_initialized {
            apply_reader_style(ctx, &self.metrics);
            self.style_initialized = true;
        }

        self.handle_shortcuts(ctx);

        egui::TopBottomPanel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(246, 249, 253))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(214, 221, 231)))
                    .inner_margin(egui::Margin::symmetric(12, 8)),
            )
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing = vec2(8.0, 8.0);
                    if ui.button("打开").clicked() {
                        self.open_file_dialog();
                    }
                    if ui.button("重载").clicked() {
                        self.reload_current();
                    }
                    if ui.button("保存").clicked() {
                        self.save_current();
                    }
                    if ui.button("另存为").clicked() {
                        self.save_as_dialog();
                    }

                    ui.separator();
                    ui.label("主题");
                    for preset in ReaderTheme::all() {
                        if ui
                            .selectable_label(self.theme == preset, preset.label())
                            .clicked()
                        {
                            self.apply_theme(ctx, preset);
                        }
                    }

                    ui.separator();
                    ui.label("模式");
                    for mode in WorkspaceMode::all() {
                        if ui
                            .selectable_label(self.mode == mode, mode.label())
                            .clicked()
                        {
                            self.mode = mode;
                        }
                    }

                    ui.separator();
                    ui.label("预览");
                    for mode in PreviewMode::all() {
                        if ui
                            .selectable_label(self.preview_mode == mode, mode.label())
                            .clicked()
                        {
                            self.preview_mode = mode;
                        }
                    }

                    ui.separator();
                    let edit_side_label = if self.edit_on_left {
                        "编辑在左"
                    } else {
                        "编辑在右"
                    };
                    if ui.button(edit_side_label).clicked() {
                        self.edit_on_left = !self.edit_on_left;
                    }
                });

                ui.add_space(4.0);
                ui.horizontal_wrapped(|ui| {
                    let file_label = match &self.current_file {
                        Some(path) => {
                            let mut path_label = path.display().to_string();
                            if self.dirty {
                                path_label.push_str(" *");
                            }
                            path_label
                        }
                        None => {
                            if self.dirty {
                                "未命名文件 *".to_owned()
                            } else {
                                "未打开文件".to_owned()
                            }
                        }
                    };
                    ui.colored_label(Color32::from_rgb(67, 78, 91), file_label);

                    if let Some(err) = &self.last_error {
                        ui.separator();
                        ui.colored_label(Color32::from_rgb(175, 32, 32), err);
                    }
                    if let Some(status) = &self.status_message {
                        ui.separator();
                        ui.colored_label(Color32::from_rgb(26, 122, 46), status);
                    }
                });
            });

        egui::TopBottomPanel::bottom("statusbar")
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(248, 250, 253))
                    .stroke(Stroke::new(1.0, Color32::from_rgb(219, 225, 234)))
                    .inner_margin(egui::Margin::symmetric(12, 6)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        "快捷键: Ctrl+O 打开 | Ctrl+R 重载 | Ctrl+S 保存 | Ctrl+Shift+S 另存为",
                    );
                    ui.separator();
                    ui.label(format!(
                        "当前: {} / {}",
                        self.mode.label(),
                        self.preview_mode.label()
                    ));
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(236, 242, 248))
                    .inner_margin(egui::Margin::same(12)),
            )
            .show(ctx, |ui| match self.mode {
                WorkspaceMode::Read => self.render_preview_view(ui),
                WorkspaceMode::Edit => self.render_editor_view(ui),
                WorkspaceMode::Split => {
                    ui.columns(2, |columns| {
                        if self.edit_on_left {
                            self.render_editor_view(&mut columns[0]);
                            self.render_preview_view(&mut columns[1]);
                        } else {
                            self.render_preview_view(&mut columns[0]);
                            self.render_editor_view(&mut columns[1]);
                        }
                    });
                }
            });
    }
}

fn apply_reader_style(ctx: &egui::Context, metrics: &ThemeMetrics) {
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        TextStyle::Body,
        FontId::new(metrics.body_size, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Monospace,
        FontId::new(metrics.code_size, FontFamily::Monospace),
    );
    style.text_styles.insert(
        TextStyle::Button,
        FontId::new(14.0, FontFamily::Proportional),
    );
    style.text_styles.insert(
        TextStyle::Heading,
        FontId::new(metrics.heading_sizes[0], FontFamily::Proportional),
    );
    style.spacing.item_spacing = vec2(8.0, 10.0);
    style.spacing.indent = 20.0;
    style.spacing.interact_size.y = 30.0;
    style.spacing.button_padding = vec2(10.0, 6.0);

    style.visuals.panel_fill = Color32::from_rgb(244, 248, 252);
    style.visuals.extreme_bg_color = Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(248, 251, 255);
    style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(248, 251, 255);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::from_rgb(205, 214, 226));
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(231, 240, 252);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(218, 232, 250);
    style.visuals.widgets.open.bg_fill = Color32::from_rgb(231, 240, 252);
    style.visuals.selection.bg_fill = Color32::from_rgb(198, 220, 249);
    style.visuals.selection.stroke = Stroke::new(1.0, Color32::from_rgb(74, 118, 196));

    ctx.set_style(style);
}

fn render_block(ui: &mut egui::Ui, block: &Block, metrics: &ThemeMetrics) {
    match block {
        Block::Heading { level, spans } => {
            let idx = (level.saturating_sub(1) as usize).min(5);
            let size = metrics.heading_sizes[idx];
            let line_height = metrics.heading_line_heights[idx];
            let space_after = metrics.heading_gaps[idx];
            render_spans_job(ui, spans, size, line_height, true);
            ui.add_space(space_after);
        }
        Block::Paragraph { spans } => {
            render_spans_job(
                ui,
                spans,
                metrics.body_size,
                metrics.body_line_height,
                false,
            );
            ui.add_space(metrics.paragraph_gap);
        }
        Block::List { kind, items } => {
            for (idx, item) in items.iter().enumerate() {
                ui.horizontal(|ui| {
                    let marker = match kind {
                        ListKind::Unordered => match item.checked {
                            Some(true) => "☑".to_owned(),
                            Some(false) => "☐".to_owned(),
                            None => "•".to_owned(),
                        },
                        ListKind::Ordered { start } => format!("{}.", start + idx as u64),
                    };
                    ui.add_sized(
                        [34.0, metrics.body_line_height],
                        egui::Label::new(
                            egui::RichText::new(marker)
                                .strong()
                                .size(metrics.body_size)
                                .color(Color32::from_gray(80)),
                        ),
                    );
                    render_spans_job(
                        ui,
                        &item.spans,
                        metrics.body_size,
                        metrics.body_line_height,
                        false,
                    );
                });
            }
            ui.add_space(metrics.list_gap);
        }
        Block::Quote { blocks } => {
            egui::Frame::new()
                .fill(Color32::from_gray(249))
                .corner_radius(6.0)
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.colored_label(Color32::from_rgb(125, 125, 125), "▌");
                        ui.vertical(|ui| {
                            for block in blocks {
                                render_block(ui, block, metrics);
                            }
                        });
                    });
                });
            ui.add_space(metrics.block_gap);
        }
        Block::CodeBlock { code } => {
            egui::Frame::new()
                .fill(Color32::from_gray(245))
                .corner_radius(6.0)
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    let mut job = LayoutJob::default();
                    let format = TextFormat {
                        font_id: FontId::new(metrics.code_size, FontFamily::Monospace),
                        line_height: Some(metrics.code_line_height),
                        color: Color32::from_rgb(31, 41, 55),
                        ..Default::default()
                    };
                    job.append(code, 0.0, format);
                    ui.add(egui::Label::new(job));
                });
            ui.add_space(metrics.block_gap);
        }
        Block::Divider => {
            ui.separator();
            ui.add_space(metrics.block_gap);
        }
        Block::Table { headers, rows } => {
            let id = ui.make_persistent_id(format!("table_{:p}", rows.as_ptr()));
            egui::Grid::new(id)
                .striped(true)
                .spacing(vec2(16.0, 10.0))
                .min_col_width(110.0)
                .show(ui, |ui| {
                    for header in headers {
                        render_spans_job(
                            ui,
                            header,
                            metrics.body_size - 0.5,
                            metrics.table_cell_line_height,
                            true,
                        );
                    }
                    ui.end_row();

                    for row in rows {
                        for cell in row {
                            render_spans_job(
                                ui,
                                cell,
                                metrics.body_size - 0.5,
                                metrics.table_cell_line_height,
                                false,
                            );
                        }
                        ui.end_row();
                    }
                });
            ui.add_space(metrics.block_gap);
        }
    }
}

fn render_spans_job(
    ui: &mut egui::Ui,
    spans: &[Span],
    base_size: f32,
    line_height: f32,
    heading: bool,
) {
    let job = spans_to_layout_job(spans, base_size, line_height, heading);
    ui.add(egui::Label::new(job).wrap());
}

fn spans_to_layout_job(
    spans: &[Span],
    base_size: f32,
    line_height: f32,
    heading: bool,
) -> LayoutJob {
    let mut job = LayoutJob::default();
    for span in spans {
        let mut format = TextFormat {
            font_id: FontId::new(base_size, FontFamily::Proportional),
            line_height: Some(line_height),
            color: Color32::from_rgb(27, 27, 27),
            ..Default::default()
        };
        if heading || span.bold {
            format.font_id = FontId::new(base_size, FontFamily::Proportional);
        }
        if span.italic {
            format.italics = true;
        }
        if span.strike {
            format.strikethrough = Stroke::new(1.0, Color32::from_gray(105));
        }
        if span.code {
            format.font_id = FontId::new((base_size - 1.5).max(12.0), FontFamily::Monospace);
            format.background = Color32::from_gray(240);
            format.color = Color32::from_rgb(127, 26, 177);
            format.line_height = Some((line_height - 1.0).max(18.0));
        }
        if span.link.is_some() {
            format.color = Color32::from_rgb(32, 94, 214);
            format.underline = Stroke::new(1.0, Color32::from_rgb(32, 94, 214));
        }

        let text = if let Some(url) = &span.image {
            if span.text.is_empty() {
                format!("[图片]({url})")
            } else {
                format!("[图片: {}]({url})", span.text)
            }
        } else {
            span.text.clone()
        };
        job.append(&text, 0.0, format);
    }
    job
}
