use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub gated: bool,
}

pub fn list_mcp_tools() -> Vec<McpToolInfo> {
    vec![
        McpToolInfo { name: "create_presentation", description: "Create a new presentation", gated: false },
        McpToolInfo { name: "create_slides", description: "Add slides to a presentation", gated: false },
        McpToolInfo { name: "import_document_to_slides", description: "Import PDF/DOCX/PPTX/MD to slides", gated: false },
        McpToolInfo { name: "generate_story_slides", description: "Generate slides from story text", gated: false },
        McpToolInfo { name: "search_bible", description: "Search Bible by reference or keyword", gated: false },
        McpToolInfo { name: "get_bible_verse", description: "Get a specific Bible verse", gated: false },
        McpToolInfo { name: "extract_quotes", description: "Extract quotable quotes from transcript", gated: false },
        McpToolInfo { name: "present_quote_lower_third", description: "Present quote as RGBA lower third", gated: true },
        McpToolInfo { name: "present_quote_full_page", description: "Present quote as full-page graphic", gated: true },
        McpToolInfo { name: "set_output_look", description: "Switch active Output Look", gated: true },
        McpToolInfo { name: "get_live_state", description: "Get current live production state", gated: false },
        McpToolInfo { name: "go_live", description: "Send slide live to audience", gated: true },
        McpToolInfo { name: "next_slide", description: "Advance to next slide", gated: true },
        McpToolInfo { name: "clear_output", description: "Clear audience output", gated: true },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_tool_registry_and_gates() {
        let tools = list_mcp_tools();
        assert_eq!(tools.len(), 14);
        assert!(tools.iter().find(|t| t.name == "go_live").unwrap().gated);
        assert!(!tools.iter().find(|t| t.name == "search_bible").unwrap().gated);
    }
}
