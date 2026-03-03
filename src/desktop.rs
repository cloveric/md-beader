use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum IpcCommand {
    AppReady,
    NewFile,
    OpenFile,
    SaveFile { content: String },
    SaveAs { content: String },
}

impl IpcCommand {
    pub fn parse(raw: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(raw)
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum HostEvent {
    FileOpened { path: String, content: String },
    FileSaved { path: String },
    Error { message: String },
    Status { message: String },
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
        let cmd = IpcCommand::parse(r##"{"cmd":"save_file","content":"# hello"}"##)
            .expect("parse save command");
        assert_eq!(
            cmd,
            IpcCommand::SaveFile {
                content: "# hello".to_owned()
            }
        );
    }

    #[test]
    fn parses_new_file_command() {
        let cmd = IpcCommand::parse(r##"{"cmd":"new_file"}"##).expect("parse new_file command");
        assert_eq!(cmd, IpcCommand::NewFile);
    }

    #[test]
    fn encodes_script_payload() {
        let script = to_webview_script(&HostEvent::Status {
            message: "ok".to_owned(),
        })
        .expect("encode script");
        assert!(script.contains(r#""event":"status""#));
        assert!(script.contains(r#""message":"ok""#));
    }
}
