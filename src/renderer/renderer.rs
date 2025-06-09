use crate::parser::*;
use crate::renderer::traits::*;
use rustdoc_types::Visibility;

pub struct ParsedRenderer;

impl ParsedRenderer {
    pub fn render(&self, module: &ParsedModule, crate_version: Option<&str>) -> String {
        let mut output = String::new();

        // Render crate header
        output.push_str(&format!("# Crate: {}\n\n", module.name));

        if let Some(version) = crate_version {
            output.push_str(&format!("Version: {}\n\n", version));
        }

        if let Some(docs) = &module.docs {
            output.push_str(&format!("{}\n\n", docs));
        }

        // Extract macros first to render them at the top (for compatibility with expected output)
        let (macros, other_items): (Vec<_>, Vec<_>) = module
            .items
            .iter()
            .partition(|item| matches!(item, ParsedItem::Macro(_)));

        let context = RenderContext::new().with_depth(1);

        // First, render all macros
        for item in &macros {
            output.push_str(&item.render(&context));
        }

        // Then render all other items
        for item in &other_items {
            output.push_str(&item.render(&context));
        }

        // Render re-exports section if any exist
        let reexports: Vec<_> = module
            .items
            .iter()
            .filter_map(|item| match item {
                ParsedItem::ReExport(re) => Some(re),
                _ => None,
            })
            .collect();

        if !reexports.is_empty() {
            output.push_str("# Re-exports\n\n");

            // Find if any re-export has documentation
            let doc_comment = reexports
                .iter()
                .find_map(|re| re.docs.as_ref())
                .map(|docs| format!("  /// {}\n", docs));

            // If we have a doc comment, use it for all re-exports
            if let Some(ref doc) = doc_comment {
                output.push_str(doc);
            }

            // Render all re-exports
            for reexport in reexports {
                output.push_str(&format!("  pub use {}\n", reexport.path));
            }
        }

        output
    }
}