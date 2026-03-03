#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::path::PathBuf;
use std::sync::OnceLock;

use markdown_reader::desktop::{HostEvent, IpcCommand, to_webview_script};
use markdown_reader::io::{read_text_with_fallback, write_text_utf8};
use rfd::FileDialog;
use tao::dpi::LogicalSize;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop, EventLoopBuilder};
use tao::window::WindowBuilder;
use wry::{WebView, WebViewBuilder};

#[derive(Debug)]
enum UserEvent {
    Ipc(String),
}

const INDEX_HTML_TEMPLATE: &str = include_str!("../assets/editor_shell.html");
const ENGINE_JS_B64: &str = include_str!("../assets/vendor/engine.js.b64");
const ENGINE_CSS_B64: &str = include_str!("../assets/vendor/engine.css.b64");
const ENGINE_ICON_B64: &str = include_str!("../assets/vendor/icon_ant.js.b64");
const I18N_ZH_CN_B64: &str = include_str!("../assets/vendor/i18n_zh_cn.js.b64");

static INIT_SCRIPT: OnceLock<String> = OnceLock::new();

fn certutil_base64_body(data: &str) -> String {
    data.lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>()
}

fn initialization_script() -> &'static str {
    INIT_SCRIPT
        .get_or_init(|| {
            let js_b64 = certutil_base64_body(ENGINE_JS_B64);
            let css_b64 = certutil_base64_body(ENGINE_CSS_B64);
            let icon_b64 = certutil_base64_body(ENGINE_ICON_B64);
            let i18n_b64 = certutil_base64_body(I18N_ZH_CN_B64);
            format!(
                "window.__ENGINE_JS_B64__ = {}; window.__ENGINE_CSS_B64__ = {}; window.__ENGINE_ICON_B64__ = {}; window.__ENGINE_I18N_B64__ = {};",
                serde_json::to_string(&js_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(&css_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(&icon_b64).unwrap_or_else(|_| "\"\"".to_owned()),
                serde_json::to_string(&i18n_b64).unwrap_or_else(|_| "\"\"".to_owned())
            )
        })
        .as_str()
}

fn send_event(webview: &WebView, event: HostEvent) {
    if let Ok(script) = to_webview_script(&event) {
        let _ = webview.evaluate_script(&script);
    }
}

fn open_file_into_editor(webview: &WebView, path: PathBuf, current_file: &mut Option<PathBuf>) {
    match read_text_with_fallback(&path) {
        Ok(content) => {
            *current_file = Some(path.clone());
            send_event(
                webview,
                HostEvent::FileOpened {
                    path: path.display().to_string(),
                    content,
                },
            );
        }
        Err(err) => {
            send_event(
                webview,
                HostEvent::Error {
                    message: format!("打开失败: {err}"),
                },
            );
        }
    }
}

fn save_content_to_path(
    webview: &WebView,
    path: PathBuf,
    content: &str,
    current_file: &mut Option<PathBuf>,
) {
    match write_text_utf8(&path, content) {
        Ok(()) => {
            *current_file = Some(path.clone());
            send_event(
                webview,
                HostEvent::FileSaved {
                    path: path.display().to_string(),
                },
            );
        }
        Err(err) => {
            send_event(
                webview,
                HostEvent::Error {
                    message: format!("保存失败: {err}"),
                },
            );
        }
    }
}

fn main() -> wry::Result<()> {
    let initial_file = std::env::args().nth(1).map(PathBuf::from);
    let event_loop: EventLoop<UserEvent> = EventLoopBuilder::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("Markdown Studio")
        .with_inner_size(LogicalSize::new(1280.0, 860.0))
        .build(&event_loop)
        .map_err(|_| wry::Error::InitScriptError)?;

    let webview = WebViewBuilder::new()
        .with_initialization_script(initialization_script())
        .with_html(INDEX_HTML_TEMPLATE.to_owned())
        .with_ipc_handler(move |request| {
            let _ = proxy.send_event(UserEvent::Ipc(request.body().to_owned()));
        })
        .build(&window)?;

    let mut current_file: Option<PathBuf> = None;
    let mut pending_initial_file = initial_file;

    event_loop.run(move |event, _window_target, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::UserEvent(UserEvent::Ipc(payload)) => match IpcCommand::parse(&payload) {
                Ok(IpcCommand::AppReady) => {
                    send_event(
                        &webview,
                        HostEvent::Status {
                            message: "编辑器已就绪".to_owned(),
                        },
                    );
                    if let Some(path) = pending_initial_file.take() {
                        open_file_into_editor(&webview, path, &mut current_file);
                    }
                }
                Ok(IpcCommand::NewFile) => {
                    current_file = None;
                    send_event(
                        &webview,
                        HostEvent::FileOpened {
                            path: String::new(),
                            content: String::new(),
                        },
                    );
                }
                Ok(IpcCommand::OpenFile) => {
                    let file = FileDialog::new()
                        .add_filter("Markdown", &["md", "markdown", "txt"])
                        .pick_file();
                    if let Some(path) = file {
                        open_file_into_editor(&webview, path, &mut current_file);
                    }
                }
                Ok(IpcCommand::SaveFile { content }) => {
                    if let Some(path) = current_file.clone() {
                        save_content_to_path(&webview, path, &content, &mut current_file);
                    } else {
                        let file = FileDialog::new()
                            .add_filter("Markdown", &["md", "markdown", "txt"])
                            .set_file_name("untitled.md")
                            .save_file();
                        if let Some(path) = file {
                            save_content_to_path(&webview, path, &content, &mut current_file);
                        }
                    }
                }
                Ok(IpcCommand::SaveAs { content }) => {
                    let default_name = current_file
                        .as_ref()
                        .and_then(|path| path.file_name().and_then(|name| name.to_str()))
                        .unwrap_or("untitled.md");
                    let file = FileDialog::new()
                        .add_filter("Markdown", &["md", "markdown", "txt"])
                        .set_file_name(default_name)
                        .save_file();
                    if let Some(path) = file {
                        save_content_to_path(&webview, path, &content, &mut current_file);
                    }
                }
                Err(err) => {
                    send_event(
                        &webview,
                        HostEvent::Error {
                            message: format!("无法识别指令: {err}"),
                        },
                    );
                }
            },
            _ => {}
        }
    })
}
