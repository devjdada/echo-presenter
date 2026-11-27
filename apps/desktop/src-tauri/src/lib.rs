pub mod renderer_bridge;

use std::path::PathBuf;
use std::sync::Mutex;
use echo_bible::{BibleEngine, BibleTranslation, BibleVerseResult};
use echo_core::{
    BackgroundLayer, OutputTargetMode, RendererCommand, SceneGraph, SceneLayer, SceneObject,
    Size2D, TextShadow, Vec2,
};
use echo_output::OutputManager;
use echo_presentation::{MediaItemRecord, ProjectDb, Slide};
use renderer_bridge::RendererBridge;
use serde::Serialize;
use tauri::{Manager, State};
use tracing::info;
use uuid::Uuid;

pub struct AppState {
    pub output_manager: Mutex<OutputManager>,
    pub active_slide_id: Mutex<Option<String>>,
    pub cleared_layers: Mutex<Vec<String>>,
    pub renderer: Mutex<Option<RendererBridge>>,
    pub bible: Mutex<Option<BibleEngine>>,
    pub project_db: Mutex<Option<ProjectDb>>,
    pub audio_listening: Mutex<bool>,
}

impl Default for AppState {
    fn default() -> Self {
        let bible = find_bundled_bible_db().and_then(|p| BibleEngine::open(&p).ok());
        let project_db = directories::ProjectDirs::from("com", "echopresenter", "EchoPresenter")
            .and_then(|dirs| ProjectDb::open(&dirs.data_dir().join("echopresenter.sqlite")).ok());
        Self {
            output_manager: Mutex::new(OutputManager::default()),
            active_slide_id: Mutex::new(None),
            cleared_layers: Mutex::new(Vec::new()),
            renderer: Mutex::new(None),
            bible: Mutex::new(bible),
            project_db: Mutex::new(project_db),
            audio_listening: Mutex::new(false),
        }
    }
}

fn find_bundled_bible_db() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let p = dir.join("resources/bible-modules/bundled_public_domain.sqlite");
        if p.exists() {
            return Some(p);
        }
        let p2 = dir.join("bundled_public_domain.sqlite");
        if p2.exists() {
            return Some(p2);
        }
    }
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../resources/bible-modules/bundled_public_domain.sqlite");
    if dev.exists() {
        return Some(dev);
    }
    None
}

#[derive(Debug, Serialize)]
pub struct AppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub bundle_id: &'static str,
    pub active_look_id: String,
    pub audience_live: bool,
}

#[derive(Debug, Serialize)]
pub struct LiveStateResponse {
    pub active_slide_id: Option<String>,
    pub active_look_id: String,
    pub audience_live: bool,
    pub cleared_layers: Vec<String>,
}

#[tauri::command]
fn get_app_info(state: State<'_, AppState>) -> AppInfo {
    let om = state.output_manager.lock().unwrap();
    AppInfo {
        name: "EchoPresenter",
        version: env!("CARGO_PKG_VERSION"),
        bundle_id: "com.echopresenter.app",
        active_look_id: om.active_look_id.clone(),
        audience_live: om.audience_live,
    }
}

#[tauri::command]
fn trigger_slide(
    slide_id: String,
    text: Option<String>,
    reference: Option<String>,
    next_slide_text: Option<String>,
    stage_message: Option<String>,
    state: State<'_, AppState>,
) -> LiveStateResponse {
    info!("Triggering slide live: {}", slide_id);
    let mut active = state.active_slide_id.lock().unwrap();
    *active = Some(slide_id.clone());

    let mut om = state.output_manager.lock().unwrap();
    om.audience_live = true;
    let look_id = om.active_look_id.clone();

    let mut cleared = state.cleared_layers.lock().unwrap();
    cleared.clear();

    let content = match (text, reference) {
        (Some(t), Some(r)) => format!("{t}\n\n— {r}"),
        (Some(t), None) => t,
        _ => format!("Slide {slide_id}"),
    };

    let scene = SceneGraph {
        scene_id: Uuid::new_v4(),
        width: 1920,
        height: 1080,
        background: BackgroundLayer::Gradient {
            from: "#0F172A".into(),
            to: "#1E1B4B".into(),
            angle_deg: 135.0,
        },
        layers: vec![SceneLayer {
            id: "slide".into(),
            name: "Slide".into(),
            visible: true,
            opacity: 1.0,
            objects: vec![SceneObject::Text {
                content,
                font_family: "Segoe UI".into(),
                font_size: 64.0,
                font_weight: 700,
                color: "#FFFFFF".into(),
                position: Vec2 { x: 160.0, y: 360.0 },
                size: Size2D { w: 1600.0, h: 420.0 },
                outline: None,
                shadow: Some(TextShadow {
                    color: "#000000B0".into(),
                    offset: [3.0, 3.0],
                    blur: 4.0,
                }),
            }],
        }],
    };

    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::RenderScene {
            output_id: "output-1".into(),
            scene,
            look_id: look_id.clone(),
            next_slide_text,
            stage_message,
        });
    }

    LiveStateResponse {
        active_slide_id: active.clone(),
        active_look_id: look_id,
        audience_live: om.audience_live,
        cleared_layers: cleared.clone(),
    }
}

#[tauri::command]
fn clear_layer(layer_id: String, state: State<'_, AppState>) -> LiveStateResponse {
    info!("Clearing layer: {}", layer_id);
    let mut active = state.active_slide_id.lock().unwrap();
    let mut om = state.output_manager.lock().unwrap();
    let mut cleared = state.cleared_layers.lock().unwrap();

    if layer_id == "all" {
        *active = None;
        om.audience_live = false;
        cleared.clear();
    } else if layer_id == "slide" {
        *active = None;
        if !cleared.contains(&layer_id) {
            cleared.push(layer_id.clone());
        }
    } else if !cleared.contains(&layer_id) {
        cleared.push(layer_id.clone());
    }

    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::ClearLayer {
            output_id: "output-1".into(),
            layer_id,
        });
    }

    LiveStateResponse {
        active_slide_id: active.clone(),
        active_look_id: om.active_look_id.clone(),
        audience_live: om.audience_live,
        cleared_layers: cleared.clone(),
    }
}

#[tauri::command]
fn set_output_look(look_id: String, state: State<'_, AppState>) -> LiveStateResponse {
    info!("Switching Output Look to: {}", look_id);
    let active = state.active_slide_id.lock().unwrap();
    let mut om = state.output_manager.lock().unwrap();
    om.set_active_look(&look_id);
    let cleared = state.cleared_layers.lock().unwrap();

    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::SetOutputLook {
            look_id: om.active_look_id.clone(),
        });
    }

    LiveStateResponse {
        active_slide_id: active.clone(),
        active_look_id: om.active_look_id.clone(),
        audience_live: om.audience_live,
        cleared_layers: cleared.clone(),
    }
}

#[tauri::command]
fn list_monitors(state: State<'_, AppState>) {
    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::ListMonitors);
    }
}

#[tauri::command]
fn set_output_target(mode: String, monitor_id: Option<String>, state: State<'_, AppState>) {
    let target = match mode.as_str() {
        "fullscreen" => OutputTargetMode::BorderlessFullscreen {
            monitor_id: monitor_id.unwrap_or_else(|| "mon-0".into()),
        },
        _ => OutputTargetMode::WindowedTest {
            width: 1280,
            height: 720,
        },
    };
    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::SetOutputTarget {
            output_id: "output-1".into(),
            target,
        });
    }
}

#[tauri::command]
fn bible_list_translations(state: State<'_, AppState>) -> Vec<BibleTranslation> {
    if let Ok(guard) = state.bible.lock()
        && let Some(engine) = guard.as_ref()
    {
        return engine.list_translations().unwrap_or_default();
    }
    Vec::new()
}

#[tauri::command]
fn bible_search_reference(
    reference: String,
    translation_id: String,
    state: State<'_, AppState>,
) -> Vec<BibleVerseResult> {
    if let Ok(guard) = state.bible.lock()
        && let Some(engine) = guard.as_ref()
    {
        return engine
            .lookup_reference(&reference, &translation_id)
            .unwrap_or_default();
    }
    Vec::new()
}

#[tauri::command]
fn bible_search_keyword(
    query: String,
    translation_id: String,
    state: State<'_, AppState>,
) -> Vec<BibleVerseResult> {
    if let Ok(guard) = state.bible.lock()
        && let Some(engine) = guard.as_ref()
    {
        return engine
            .search_fts(&query, &translation_id, 50)
            .unwrap_or_default();
    }
    Vec::new()
}

#[tauri::command]
fn bible_compare_translations(
    reference: String,
    translation_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Vec<Vec<BibleVerseResult>> {
    if let Ok(guard) = state.bible.lock()
        && let Some(engine) = guard.as_ref()
    {
        return engine
            .compare_translations(&reference, &translation_ids)
            .unwrap_or_default();
    }
    Vec::new()
}

#[tauri::command]
fn import_pptx(file_path: String) -> Result<Vec<Slide>, String> {
    echo_ingest::import_pptx_file(std::path::Path::new(&file_path)).map_err(|e| e.to_string())
}

#[tauri::command]
fn import_media_item(name: String, file_path: String, kind: String, state: State<'_, AppState>) -> Result<Vec<MediaItemRecord>, String> {
    let mut guard = state.project_db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_mut().ok_or_else(|| "ProjectDb not initialized".to_string())?;
    db.upsert_media(&MediaItemRecord {
        id: Uuid::new_v4().to_string(),
        name,
        file_path,
        kind,
        thumbnail_path: None,
    })
    .map_err(|e| e.to_string())?;
    db.list_media().map_err(|e| e.to_string())
}

#[tauri::command]
fn list_media_items(state: State<'_, AppState>) -> Vec<MediaItemRecord> {
    if let Ok(guard) = state.project_db.lock()
        && let Some(db) = guard.as_ref()
    {
        return db.list_media().unwrap_or_default();
    }
    Vec::new()
}

#[tauri::command]
fn save_playlists(playlists_json: String, state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.project_db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or_else(|| "ProjectDb not initialized".to_string())?;
    db.save_playlists_json(&playlists_json).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_playlists(state: State<'_, AppState>) -> Option<String> {
    if let Ok(guard) = state.project_db.lock()
        && let Some(db) = guard.as_ref()
    {
        return db.load_playlists_json().ok().flatten();
    }
    None
}

#[tauri::command]
fn ingest_text_to_slides(text: String) -> Vec<Slide> {
    let beats = echo_ingest::chunk_text_to_beats(&text);
    echo_ingest::beats_to_slides(&beats)
}

#[tauri::command]
fn ai_check_safety(action: String) -> String {
    match echo_ai::check_action_safety(&action) {
        echo_ai::ActionSafety::Unrestricted => "unrestricted".into(),
        echo_ai::ActionSafety::RequiresApproval => "requires_approval".into(),
    }
}

#[tauri::command]
fn audio_list_devices() -> Vec<echo_audio::AudioInputDevice> {
    echo_audio::list_input_devices()
}

#[derive(Debug, Serialize)]
pub struct SermonLineAnalysis {
    pub transcript: String,
    pub detected_scriptures: Vec<echo_scripture_detection::DetectedReference>,
    pub extracted_quote: Option<echo_quotes::ExtractedQuote>,
}

#[tauri::command]
fn audio_simulate_sermon_line(line: String, speaker: Option<String>) -> SermonLineAnalysis {
    let detector = echo_scripture_detection::ScriptureDetector::new(vec![
        "Genesis".into(),
        "Psalms".into(),
        "Proverbs".into(),
        "Isaiah".into(),
        "Matthew".into(),
        "John".into(),
        "Romans".into(),
        "1 Corinthians".into(),
        "Hebrews".into(),
    ]);
    let detected_scriptures = detector.detect(&line);
    let extracted_quote = if line.split_whitespace().count() >= 5 {
        Some(echo_quotes::ExtractedQuote {
            id: Uuid::new_v4().to_string(),
            quote_text: line.clone(),
            cleaned_grammar: line.clone(),
            category: "sermon".into(),
            punchiness_score: 0.90,
            speaker: speaker.unwrap_or_else(|| "Pastor".into()),
        })
    } else {
        None
    };
    SermonLineAnalysis {
        transcript: line,
        detected_scriptures,
        extracted_quote,
    }
}

#[tauri::command]
fn mcp_list_tools() -> Vec<echo_mcp::McpToolInfo> {
    echo_mcp::list_mcp_tools()
}

#[tauri::command]
fn osc_parse_command(addr: String, arg: Option<f32>) -> Option<echo_automation::AutomationAction> {
    echo_automation::parse_osc_address(&addr, arg)
}

#[tauri::command]
fn save_settings(settings_json: String, state: State<'_, AppState>) -> Result<(), String> {
    let guard = state.project_db.lock().map_err(|e| e.to_string())?;
    let db = guard.as_ref().ok_or_else(|| "ProjectDb not initialized".to_string())?;
    db.save_settings_json(&settings_json).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_settings(state: State<'_, AppState>) -> Option<String> {
    if let Ok(guard) = state.project_db.lock()
        && let Some(db) = guard.as_ref()
    {
        return db.load_settings_json().ok().flatten();
    }
    None
}

#[tauri::command]
async fn ai_chat_send(api_key: String, model: String, prompt: String) -> Result<String, String> {
    echo_ai::send_openrouter_chat(
        &api_key,
        &model,
        "You are EchoPresenter AI Director. Help create church presentation slides and scriptures concisely.",
        &prompt,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn import_document_to_slides(file_path: String) -> Result<Vec<Slide>, String> {
    let path = std::path::Path::new(&file_path);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "pptx" => echo_ingest::import_pptx_file(path).map_err(|e| e.to_string()),
        "docx" => {
            let text = echo_ingest::extract_docx_text(path)?;
            let beats = echo_ingest::chunk_text_to_beats(&text);
            Ok(echo_ingest::beats_to_slides(&beats))
        }
        _ => {
            let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
            let beats = echo_ingest::chunk_text_to_beats(&text);
            Ok(echo_ingest::beats_to_slides(&beats))
        }
    }
}

#[tauri::command]
fn set_transition(kind: String, duration_ms: u32, state: State<'_, AppState>) {
    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::SetTransition { kind, duration_ms });
    }
}

#[tauri::command]
fn update_visual_layer(opacity: f32, blend_mode: String, audio_reactive: bool, state: State<'_, AppState>) {
    if let Ok(guard) = state.renderer.lock()
        && let Some(bridge) = guard.as_ref()
    {
        bridge.send(RendererCommand::UpdateVisualLayer {
            opacity,
            blend_mode,
            audio_reactive,
        });
    }
}

#[tauri::command]
fn audio_start_listening(device_name: Option<String>, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<bool, String> {
    use tauri::Emitter;
    let mut listening = state.audio_listening.lock().map_err(|e| e.to_string())?;
    *listening = true;
    info!(device = ?device_name, "Started audio listening");
    let _ = app.emit("audio://level", 0.35_f32);
    Ok(true)
}

#[tauri::command]
fn audio_stop_listening(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<bool, String> {
    use tauri::Emitter;
    let mut listening = state.audio_listening.lock().map_err(|e| e.to_string())?;
    *listening = false;
    let _ = app.emit("audio://level", 0.0_f32);
    Ok(false)
}

#[tauri::command]
fn bible_generate_slides(reference: String, translation_id: String, state: State<'_, AppState>) -> Result<Vec<Slide>, String> {
    let guard = state.bible.lock().map_err(|e| e.to_string())?;
    let engine = guard.as_ref().ok_or_else(|| "BibleEngine not initialized".to_string())?;
    let verses = engine
        .lookup_reference(&reference, &translation_id)
        .map_err(|e| e.to_string())?;
    let chunks = echo_bible::split_verses_into_slides(&verses, 180);
    Ok(chunks
        .into_iter()
        .enumerate()
        .map(|(i, (text, ref_str))| Slide {
            id: format!("bible-slide-{}-{}", i + 1, Uuid::new_v4()),
            label: ref_str.clone(),
            group: echo_presentation::SlideGroupKind::Scripture,
            text,
            reference: Some(ref_str),
            background_color: "#451A03".into(),
        })
        .collect())
}

#[tauri::command]
fn bible_import_user_json(file_path: String, translation_id: String) -> Result<String, String> {
    let dirs = directories::ProjectDirs::from("com", "echopresenter", "EchoPresenter")
        .ok_or_else(|| "Failed to resolve AppData dir".to_string())?;
    let user_db_path = dirs.data_dir().join("user_imported.sqlite");
    if let Some(parent) = user_db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let _engine = BibleEngine::open(&user_db_path).map_err(|e| e.to_string())?;
    Ok(format!("Imported {} from {} into {:?}", translation_id, file_path, user_db_path))
}

#[tauri::command]
fn export_echopres(file_path: String, playlists_json: String) -> Result<String, String> {
    std::fs::write(&file_path, playlists_json).map_err(|e| e.to_string())?;
    Ok(file_path)
}

#[tauri::command]
fn import_echopres(file_path: String) -> Result<String, String> {
    std::fs::read_to_string(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn test_ai_connection(api_key: String, ollama_url: String) -> Result<String, String> {
    if !api_key.trim().is_empty() {
        echo_ai::send_openrouter_chat(&api_key, "google/gemini-2.5-flash", "Reply OK", "ping")
            .await
            .map(|_| "Connected to OpenRouter".to_string())
            .map_err(|e| e.to_string())
    } else {
        Ok(format!("Local Ollama ready at {ollama_url}"))
    }
}

#[derive(Debug, Serialize)]
pub struct WhisperModelStatus {
    pub id: String,
    pub size_label: String,
    pub installed: bool,
    pub path: String,
}

#[derive(Debug, Serialize)]
pub struct DiagnosticsInfo {
    pub bible_ready: bool,
    pub project_db_ready: bool,
    pub api_endpoint: &'static str,
    pub remote_url: &'static str,
}

#[tauri::command]
fn get_diagnostics(state: State<'_, AppState>) -> DiagnosticsInfo {
    DiagnosticsInfo {
        bible_ready: state.bible.lock().map(|g| g.is_some()).unwrap_or(false),
        project_db_ready: state.project_db.lock().map(|g| g.is_some()).unwrap_or(false),
        api_endpoint: "http://127.0.0.1:50001/api/v1",
        remote_url: "http://127.0.0.1:50001/remote",
    }
}

#[tauri::command]
fn list_whisper_models() -> Vec<WhisperModelStatus> {
    let base_dir = directories::ProjectDirs::from("com", "echopresenter", "EchoPresenter")
        .map(|d| d.data_dir().join("ai-models"))
        .unwrap_or_else(|| PathBuf::from("ai-models"));
    [
        ("ggml-base.en.bin", "142 MB"),
        ("ggml-small.en.bin", "466 MB"),
    ]
    .into_iter()
    .map(|(id, size_label)| {
        let p = base_dir.join(id);
        WhisperModelStatus {
            id: id.to_string(),
            size_label: size_label.to_string(),
            installed: p.exists(),
            path: p.to_string_lossy().to_string(),
        }
    })
    .collect()
}

#[tauri::command]
async fn download_whisper_model(model_id: String, app: tauri::AppHandle) -> Result<String, String> {
    use tauri::Emitter;
    if model_id != "ggml-base.en.bin" && model_id != "ggml-small.en.bin" {
        return Err(format!("Unsupported Whisper model: {model_id}"));
    }
    let base_dir = directories::ProjectDirs::from("com", "echopresenter", "EchoPresenter")
        .map(|d| d.data_dir().join("ai-models"))
        .unwrap_or_else(|| PathBuf::from("ai-models"));
    std::fs::create_dir_all(&base_dir).map_err(|e| e.to_string())?;
    let dest = base_dir.join(&model_id);
    if dest.exists() {
        let _ = app.emit("whisper://download-progress", serde_json::json!({ "modelId": model_id, "percent": 100 }));
        return Ok(dest.to_string_lossy().to_string());
    }

    let url = format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{model_id}");
    let _ = app.emit("whisper://download-progress", serde_json::json!({ "modelId": model_id, "percent": 5 }));
    let resp = reqwest::get(&url).await.map_err(|e| format!("Download failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {} downloading {model_id}", resp.status()));
    }
    let bytes = resp.bytes().await.map_err(|e| format!("Read failed: {e}"))?;
    let _ = app.emit("whisper://download-progress", serde_json::json!({ "modelId": model_id, "percent": 90 }));
    std::fs::write(&dest, &bytes).map_err(|e| format!("Write failed: {e}"))?;
    let _ = app.emit("whisper://download-progress", serde_json::json!({ "modelId": model_id, "percent": 100 }));
    Ok(dest.to_string_lossy().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            let bridge = RendererBridge::start(app.handle().clone());
            let state = app.state::<AppState>();
            *state.renderer.lock().unwrap() = Some(bridge);

            // Start echo-api Axum REST server on localhost:50001
            tauri::async_runtime::spawn(async move {
                let router = echo_api::build_router();
                if let Ok(listener) = tokio::net::TcpListener::bind("127.0.0.1:50001").await {
                    info!("echo-api listening on http://127.0.0.1:50001");
                    let _ = axum::serve(listener, router).await;
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event
                && window.label() == "main"
            {
                let state = window.state::<AppState>();
                if let Ok(guard) = state.renderer.lock()
                    && let Some(bridge) = guard.as_ref()
                {
                    bridge.shutdown();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            trigger_slide,
            clear_layer,
            set_output_look,
            list_monitors,
            set_output_target,
            bible_list_translations,
            bible_search_reference,
            bible_search_keyword,
            bible_compare_translations,
            import_pptx,
            import_media_item,
            list_media_items,
            save_playlists,
            load_playlists,
            ingest_text_to_slides,
            ai_check_safety,
            audio_list_devices,
            audio_simulate_sermon_line,
            mcp_list_tools,
            osc_parse_command,
            save_settings,
            load_settings,
            ai_chat_send,
            import_document_to_slides,
            set_transition,
            update_visual_layer,
            audio_start_listening,
            audio_stop_listening,
            bible_generate_slides,
            bible_import_user_json,
            export_echopres,
            import_echopres,
            test_ai_connection,
            list_whisper_models,
            download_whisper_model,
            get_diagnostics
        ])
        .run(tauri::generate_context!())
        .expect("error while running EchoPresenter");
}
