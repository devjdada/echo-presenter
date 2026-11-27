use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use crate::{EchoError, SceneGraph};

pub const RENDERER_PIPE_NAME: &str = r"\\.\pipe\echopresenter-renderer";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MonitorInfo {
    pub id: String,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum OutputTargetMode {
    WindowedTest { width: u32, height: u32 },
    BorderlessFullscreen { monitor_id: String },
}

/// IPC messages sent from `src-tauri` to `echo-renderer` over Named Pipes (MessagePack).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum RendererCommand {
    Ping,
    ListMonitors,
    SetOutputTarget {
        output_id: String,
        target: OutputTargetMode,
    },
    RenderScene {
        output_id: String,
        scene: SceneGraph,
        look_id: String,
        next_slide_text: Option<String>,
        stage_message: Option<String>,
    },
    ClearLayer {
        output_id: String,
        layer_id: String,
    },
    SetOutputLook {
        look_id: String,
    },
    SetTransition {
        kind: String,
        duration_ms: u32,
    },
    UpdateVisualLayer {
        opacity: f32,
        blend_mode: String,
        audio_reactive: bool,
    },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum RendererEvent {
    Pong {
        adapter_name: String,
        backend: String,
    },
    MonitorsList {
        monitors: Vec<MonitorInfo>,
    },
    PreviewFrame {
        output_id: String,
        width: u32,
        height: u32,
        png_base64: String,
    },
    FrameReady {
        output_id: String,
        width: u32,
        height: u32,
    },
    Error {
        message: String,
    },
}

/// Write a length-prefixed MessagePack frame: `[u32 little-endian length][rmp-serde payload]`.
pub async fn write_framed<W, T>(writer: &mut W, msg: &T) -> Result<(), EchoError>
where
    W: AsyncWrite + Unpin,
    T: Serialize,
{
    let payload = rmp_serde::to_vec_named(msg)
        .map_err(|e| EchoError::Ipc(format!("MessagePack encode failed: {e}")))?;
    let len = u32::try_from(payload.len())
        .map_err(|_| EchoError::Ipc("Payload exceeds u32::MAX".into()))?;
    writer
        .write_all(&len.to_le_bytes())
        .await
        .map_err(|e| EchoError::Ipc(format!("Pipe write length failed: {e}")))?;
    writer
        .write_all(&payload)
        .await
        .map_err(|e| EchoError::Ipc(format!("Pipe write payload failed: {e}")))?;
    writer
        .flush()
        .await
        .map_err(|e| EchoError::Ipc(format!("Pipe flush failed: {e}")))?;
    Ok(())
}

/// Read a length-prefixed MessagePack frame: `[u32 little-endian length][rmp-serde payload]`.
pub async fn read_framed<R, T>(reader: &mut R) -> Result<T, EchoError>
where
    R: AsyncRead + Unpin,
    T: for<'de> Deserialize<'de>,
{
    let mut len_buf = [0u8; 4];
    reader
        .read_exact(&mut len_buf)
        .await
        .map_err(|e| EchoError::Ipc(format!("Pipe read length failed: {e}")))?;
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > 64 * 1024 * 1024 {
        return Err(EchoError::Ipc(format!("Frame too large: {len} bytes")));
    }
    let mut payload = vec![0u8; len];
    reader
        .read_exact(&mut payload)
        .await
        .map_err(|e| EchoError::Ipc(format!("Pipe read payload failed: {e}")))?;
    rmp_serde::from_slice(&payload)
        .map_err(|e| EchoError::Ipc(format!("MessagePack decode failed: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BackgroundLayer, SceneGraph};
    use uuid::Uuid;

    #[tokio::test]
    async fn test_length_prefixed_msgpack_roundtrip() {
        let (mut client, mut server) = tokio::io::duplex(4096);
        let cmd = RendererCommand::RenderScene {
            output_id: "output-1".into(),
            look_id: "main-fullscreen".into(),
            next_slide_text: Some("Next verse".into()),
            stage_message: None,
            scene: SceneGraph {
                scene_id: Uuid::new_v4(),
                width: 1920,
                height: 1080,
                background: BackgroundLayer::Solid {
                    color: "#0F172A".into(),
                },
                layers: vec![],
            },
        };

        write_framed(&mut client, &cmd).await.unwrap();
        let decoded: RendererCommand = read_framed(&mut server).await.unwrap();
        match decoded {
            RendererCommand::RenderScene {
                output_id, look_id, ..
            } => {
                assert_eq!(output_id, "output-1");
                assert_eq!(look_id, "main-fullscreen");
            }
            _ => panic!("Unexpected command variant"),
        }
    }
}
