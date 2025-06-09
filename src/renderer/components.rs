use crate::parser::*;
use crate::renderer::traits::*;
use rustdoc_types::Visibility;

/// Helper for rendering type signatures
pub struct TypeRenderer;

impl TypeRenderer {
    pub fn render_type(&self, ty: &RustType) -> String {
        ty.to_string()
    }

    pub fn render_visibility(&self, vis: &Visibility) -> String {
        match vis {
            Visibility::Public => "pub ".to_string(),
            Visibility::Crate => "pub(crate) ".to_string(),
            Visibility::Restricted { path, .. } => format!("pub({}) ", path),
            Visibility::Default => String::new(),
        }
    }

    pub fn render_generics(&self, generics: &Generics) -> String {
        if generics.params.is_empty() {
            return String::new();
        }

        let param_strs: Vec<String> = generics
            .params
            .iter()
            .map(|p| match &p.kind {
                GenericParamKind::Type { bounds } => {
                    if bounds.is_empty() {
                        p.name.clone()
                    } else {
                        format!("{}: {}", p.name, bounds.join(" + "))
                    }
                }
                GenericParamKind::Lifetime => {
                    if p.name.starts_with('\'') {
                        p.name.clone()
                    } else {
                        format!("'{}", p.name)
                    }
                }
                GenericParamKind::Const { ty } => format!("const {}: {}", p.name, ty),
            })
            .collect();

        format!("<{}>", param_strs.join(", "))
    }

    pub fn render_where_clause(&self, generics: &Generics) -> String {
        if generics.where_clauses.is_empty() {
            String::new()
        } else {
            format!(" where {}", generics.where_clauses.join(", "))
        }
    }
}

/// Helper for rendering documentation comments
pub struct DocRenderer;

impl DocRenderer {
    pub fn render_docs(&self, docs: Option<&String>, indent: &str) -> String {
        let Some(docs) = docs else {
            return String::new();
        };

        let mut output = String::new();
        for line in docs.lines() {
            if line.trim().is_empty() {
                output.push_str(&format!("{}///\n", indent));
            } else {
                output.push_str(&format!("{}/// {}\n", indent, line));
            }
        }
        output
    }

    pub fn render_deprecation(&self, deprecation: Option<&rustdoc_types::Deprecation>, indent: &str) -> String {
        let Some(deprecation) = deprecation else {
            return String::new();
        };

        if let Some(since) = &deprecation.since {
            format!("{}DEPRECATED since {}\n", indent, since)
        } else {
            format!("{}DEPRECATED\n", indent)
        }
    }
}

/// Helper for consistent indentation
pub struct IndentationHelper;

impl IndentationHelper {
    pub fn indent_for_depth(depth: usize) -> String {
        "  ".repeat(depth)
    }

    pub fn indent_from_context(context: &RenderContext) -> String {
        context.indent()
    }
}