use echo_core::OutputLook;

#[derive(Debug, Clone)]
pub struct OutputManager {
    pub looks: Vec<OutputLook>,
    pub active_look_id: String,
    pub audience_live: bool,
}

impl Default for OutputManager {
    fn default() -> Self {
        let looks = OutputLook::default_looks();
        let active_look_id = looks.first().map(|l| l.id.clone()).unwrap_or_default();
        Self {
            looks,
            active_look_id,
            audience_live: false,
        }
    }
}

impl OutputManager {
    pub fn set_active_look(&mut self, look_id: &str) -> bool {
        if self.looks.iter().any(|l| l.id == look_id) {
            self.active_look_id = look_id.to_string();
            true
        } else {
            false
        }
    }
}
