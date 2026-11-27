use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputLook {
    pub id: String,
    pub name: String,
    pub description: String,
    pub routes: Vec<OutputRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputRoute {
    pub output_id: String,
    pub label: String,
    pub visible_layers: Vec<String>,
    pub premultiplied_alpha: bool,
    pub target: OutputTargetKind,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputTargetKind {
    AudienceWindow,
    StageMonitor,
    NdiStream,
}

impl OutputLook {
    pub fn default_looks() -> Vec<Self> {
        vec![
            Self {
                id: "main-fullscreen".to_string(),
                name: "Main Full-Screen".to_string(),
                description: "Full-screen opaque composition for Main LED & Side Screens".to_string(),
                routes: vec![OutputRoute {
                    output_id: "output-1".to_string(),
                    label: "Main LED".to_string(),
                    visible_layers: vec![
                        "background".into(),
                        "media".into(),
                        "slide".into(),
                        "announcements".into(),
                        "props".into(),
                    ],
                    premultiplied_alpha: false,
                    target: OutputTargetKind::AudienceWindow,
                }],
            },
            Self {
                id: "ndi-lower-third".to_string(),
                name: "NDI Lower-Third Alpha".to_string(),
                description: "Premultiplied RGBA transparent overlay for vMix / OBS".to_string(),
                routes: vec![OutputRoute {
                    output_id: "output-ndi".to_string(),
                    label: "Livestream NDI".to_string(),
                    visible_layers: vec!["slide".into(), "announcements".into()],
                    premultiplied_alpha: true,
                    target: OutputTargetKind::NdiStream,
                }],
            },
            Self {
                id: "stage-confidence".to_string(),
                name: "Stage Confidence".to_string(),
                description: "Current/Next slide + countdown timer + stage message".to_string(),
                routes: vec![OutputRoute {
                    output_id: "output-stage".to_string(),
                    label: "Stage Monitor".to_string(),
                    visible_layers: vec!["slide".into(), "props".into()],
                    premultiplied_alpha: false,
                    target: OutputTargetKind::StageMonitor,
                }],
            },
        ]
    }
}
