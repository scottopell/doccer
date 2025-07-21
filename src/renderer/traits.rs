
/// Configuration context for rendering operations
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub depth: usize,
    pub show_private: bool,
    pub format: OutputFormat,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            depth: 0,
            show_private: false,
            format: OutputFormat::Text,
        }
    }

    pub fn with_depth(&self, depth: usize) -> Self {
        Self {
            depth,
            show_private: self.show_private,
            format: self.format,
        }
    }

    pub fn indent(&self) -> String {
        "  ".repeat(self.depth)
    }
}

impl Default for RenderContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Output format configuration
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Text,
    // Future: Html, Markdown, etc.
}

/// Core rendering trait for all parsed items
pub trait Render {
    fn render(&self, context: &RenderContext) -> String;
}

