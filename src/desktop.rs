use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum IpcCommand {
    AppReady {
        #[serde(default)]
        tab_id: Option<String>,
    },
    NewFile {
        #[serde(default)]
        tab_id: Option<String>,
    },
    OpenFile {
        #[serde(default)]
        tab_id: Option<String>,
    },
    SaveFile {
        #[serde(default)]
        tab_id: Option<String>,
        #[serde(default)]
        path: Option<String>,
        content: String,
    },
    SaveAs {
        #[serde(default)]
        tab_id: Option<String>,
        #[serde(default)]
        path: Option<String>,
        content: String,
    },
}

impl IpcCommand {
    pub fn parse(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum HostEvent {
    FileOpened {
        tab_id: String,
        path: String,
        content: String,
    },
    FileSaved {
        tab_id: String,
        path: String,
    },
    Error {
        message: String,
    },
    Status {
        message: String,
    },
}

pub fn to_webview_script(event: &HostEvent) -> Result<String, serde_json::Error> {
    let payload = serde_json::to_string(event)?;
    Ok(format!(
        "window.__HOST__ && window.__HOST__.onMessage({payload});"
    ))
}

#[cfg(test)]
mod tests {
    use super::{HostEvent, IpcCommand, to_webview_script};

    #[test]
    fn parses_save_command() {
        let cmd = IpcCommand::parse(
            r##"{"cmd":"save_file","tab_id":"tab-1","path":"C:\\demo\\a.md","content":"# hello"}"##,
        )
        .expect("parse save command");
        assert_eq!(
            cmd,
            IpcCommand::SaveFile {
                tab_id: Some("tab-1".to_owned()),
                path: Some(r"C:\demo\a.md".to_owned()),
                content: "# hello".to_owned(),
            }
        );
    }

    #[test]
    fn parses_new_file_command() {
        let cmd = IpcCommand::parse(r##"{"cmd":"new_file"}"##).expect("parse new_file command");
        assert_eq!(cmd, IpcCommand::NewFile { tab_id: None });
    }

    #[test]
    fn parses_open_file_with_tab_id() {
        let cmd = IpcCommand::parse(r##"{"cmd":"open_file","tab_id":"tab-2"}"##)
            .expect("parse open_file command");
        assert_eq!(
            cmd,
            IpcCommand::OpenFile {
                tab_id: Some("tab-2".to_owned()),
            }
        );
    }

    #[test]
    fn parses_legacy_save_command_without_tab_context() {
        let cmd = IpcCommand::parse(r##"{"cmd":"save_file","content":"legacy"}"##)
            .expect("parse legacy save command");
        assert_eq!(
            cmd,
            IpcCommand::SaveFile {
                tab_id: None,
                path: None,
                content: "legacy".to_owned(),
            }
        );
    }

    #[test]
    fn encodes_script_payload() {
        let script = to_webview_script(&HostEvent::FileSaved {
            tab_id: "tab-9".to_owned(),
            path: "C:\\tmp\\a.md".to_owned(),
        })
        .expect("encode script");
        assert!(script.contains(r#""event":"file_saved""#));
        assert!(script.contains(r#""tab_id":"tab-9""#));
        assert!(script.contains(r#""path":"C:\\tmp\\a.md""#));
    }
}
