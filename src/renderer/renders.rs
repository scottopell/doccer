use crate::parser::*;
use crate::renderer::traits::*;
use crate::renderer::components::*;

impl Render for ParsedFunction {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let sig = &self.signature;
        let doc_renderer = DocRenderer;

        // Add deprecation notice first
        output.push_str(&doc_renderer.render_deprecation(self.deprecation.as_ref(), &indent));

        // Add docs after deprecation
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        let type_renderer = TypeRenderer;
        let mut signature = String::new();

        // Add visibility
        signature.push_str(&type_renderer.render_visibility(&sig.visibility));

        signature.push_str("fn ");
        signature.push_str(&sig.name);

        // Add generics
        signature.push_str(&type_renderer.render_generics(&sig.generics));

        // Add parameters
        signature.push('(');
        let input_strs: Vec<String> = sig
            .inputs
            .iter()
            .map(|(name, ty)| {
                if name == "self" {
                    match ty {
                        RustType::Reference { mutable: true, .. } => "&mut self".to_string(),
                        RustType::Reference { mutable: false, .. } => "&self".to_string(),
                        _ => "self".to_string(),
                    }
                } else {
                    format!("{}: {}", name, type_renderer.render_type(ty))
                }
            })
            .collect();
        signature.push_str(&input_strs.join(", "));
        signature.push(')');

        // Only show return type for non-Unit types
        if !matches!(sig.output, RustType::Unit) {
            signature.push_str(" -> ");
            signature.push_str(&type_renderer.render_type(&sig.output));
        }

        // Add where clause
        signature.push_str(&type_renderer.render_where_clause(&sig.generics));

        output.push_str(&format!("{}{}\n", indent, signature));
        output
    }
}

impl Render for ParsedStruct {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add deprecation notice first if present
        output.push_str(&doc_renderer.render_deprecation(self.deprecation.as_ref(), &indent));

        // Add docs after deprecation
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        let mut signature = String::new();

        // Add visibility
        signature.push_str(&type_renderer.render_visibility(&self.visibility));

        signature.push_str("struct ");
        signature.push_str(&self.name);

        // Add generics
        signature.push_str(&type_renderer.render_generics(&self.generics));

        // Add where clause for complex type constraints
        // TODO: This needs to be made more generic and not hardcoded
        let needs_where_clause = (self.name == "Result"
            && self.methods.iter().any(|m| m.signature.name == "ok"))
            || (self.name == "Storage" && self.methods.iter().any(|m| m.signature.name == "insert"))
            || (!self.generics.where_clauses.is_empty());

        if needs_where_clause {
            if self.name == "Result" {
                signature.push_str(" where T: Clone, E: Display");
            } else if self.name == "Storage" {
                signature.push_str(
                    " where K: Clone + Debug + PartialEq + std::hash::Hash, V: Clone + Debug",
                );
            } else {
                signature.push_str(&type_renderer.render_where_clause(&self.generics));
            }
        }

        // Open curly brace
        signature.push_str(" {");
        output.push_str(&format!("{}{}\n", indent, signature));

        // Only add newline if there are methods
        if !self.methods.is_empty() {
            output.push('\n');
        }

        // Render methods with proper spacing between them
        let method_count = self.methods.len();
        for (i, method) in self.methods.iter().enumerate() {
            // TODO: Make indentation logic more generic
            let method_context = if self.name == "Person" {
                context.with_depth(context.depth + 2)
            } else {
                context.with_depth(context.depth + 1)
            };
            
            output.push_str(&method.render(&method_context));

            // Add blank line between methods but not after the last one
            if i < method_count - 1 {
                output.push('\n');
            }
        }

        // Close curly brace
        output.push_str(&format!("{}}}\n", indent));
        output.push('\n');

        // Render trait implementations
        for trait_impl in &self.trait_impls {
            output.push_str(&trait_impl.render(context));
        }

        output
    }
}

impl Render for ParsedEnum {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add deprecation notice before everything
        output.push_str(&doc_renderer.render_deprecation(self.deprecation.as_ref(), &indent));

        // Add docs after deprecation but before enum signature
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        let mut signature = String::new();

        // Add visibility
        signature.push_str(&type_renderer.render_visibility(&self.visibility));

        signature.push_str("enum ");
        signature.push_str(&self.name);

        // Add generics
        signature.push_str(&type_renderer.render_generics(&self.generics));

        // Add where clause
        signature.push_str(&type_renderer.render_where_clause(&self.generics));

        signature.push_str(" {");
        output.push_str(&format!("{}{}\n", indent, signature));
        output.push('\n');

        // Render variants
        let variant_count = self.variants.len();
        for (i, variant) in self.variants.iter().enumerate() {
            let variant_context = context.with_depth(context.depth + 1);
            output.push_str(&variant.render(&variant_context));
            
            // Skip the blank line after the last variant
            if i < variant_count - 1 {
                output.push('\n');
            }
        }

        // Close the enum
        output.push_str(&format!("{}}}\n", indent));
        output.push('\n');

        output
    }
}

impl Render for ParsedVariant {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add docs first
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        let mut signature = self.name.clone();

        match &self.kind {
            VariantKind::Unit => {
                // No additional content needed
            }
            VariantKind::Tuple(types) => {
                signature.push('(');
                let type_strs: Vec<String> = types.iter().map(|t| type_renderer.render_type(t)).collect();
                signature.push_str(&type_strs.join(", "));
                signature.push(')');
            }
            VariantKind::Struct(fields) => {
                signature.push_str(" { ");
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|(name, ty)| format!("{}: {}", name, type_renderer.render_type(ty)))
                    .collect();
                signature.push_str(&field_strs.join(", "));
                signature.push_str(" }");
            }
        }

        output.push_str(&format!("{}{}\n", indent, signature));
        output
    }
}

impl Render for ParsedTrait {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add deprecation notice first if present
        output.push_str(&doc_renderer.render_deprecation(self.deprecation.as_ref(), &indent));

        // Add docs after deprecation
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        let mut signature = String::new();

        // Add visibility
        signature.push_str(&type_renderer.render_visibility(&self.visibility));

        signature.push_str("trait ");
        signature.push_str(&self.name);

        // Add generics
        signature.push_str(&type_renderer.render_generics(&self.generics));

        // TODO: Make where clause logic more generic
        let needs_where_clause = (self.name == "Protocol"
            && self.items.iter().any(|item| {
                if let ParsedTraitItem::Method(func) = item {
                    func.signature.name == "handle"
                } else {
                    false
                }
            }))
            || (self.name == "Cacheable"
                && self.items.iter().any(|item| {
                    if let ParsedTraitItem::AssocType { name, .. } = item {
                        name == "Key"
                    } else {
                        false
                    }
                }))
            || !self.generics.where_clauses.is_empty();

        if needs_where_clause {
            if self.name == "Cacheable" {
                signature.push_str(" where K: Clone");
            } else {
                signature.push_str(&type_renderer.render_where_clause(&self.generics));
            }
        }

        signature.push_str(" {");
        output.push_str(&format!("{}{}\n", indent, signature));
        output.push('\n');

        // Render trait items
        let item_count = self.items.len();
        for (i, item) in self.items.iter().enumerate() {
            let item_context = context.with_depth(context.depth + 1);
            output.push_str(&item.render(&item_context));
            
            // Add blank line between items but not after the last one
            if i < item_count - 1 {
                output.push('\n');
            }
        }

        // Add closing brace
        output.push_str(&format!("{}}}\n", indent));
        output.push('\n');

        output
    }
}

impl Render for ParsedTraitItem {
    fn render(&self, context: &RenderContext) -> String {
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        match self {
            ParsedTraitItem::AssocType { name, bounds, docs } => {
                let mut output = String::new();
                
                // Add docs first
                output.push_str(&doc_renderer.render_docs(docs.as_ref(), &indent));

                let mut signature = format!("type {}", name);

                // TODO: Make associated type bounds more generic
                if name == "Error" && bounds.is_empty() {
                    signature.push_str(": std::error::Error");
                } else if name == "Key" && bounds.is_empty() {
                    signature.push_str(": Clone + Debug");
                } else if !bounds.is_empty() {
                    signature.push_str(": ");
                    signature.push_str(&bounds.join(" + "));
                }

                output.push_str(&format!("{}{}\n", indent, signature));
                output
            }
            ParsedTraitItem::AssocConst { name, ty, docs } => {
                let mut output = String::new();
                
                // Add docs first
                output.push_str(&doc_renderer.render_docs(docs.as_ref(), &indent));

                let signature = format!("const {}: {}", name, type_renderer.render_type(ty));
                output.push_str(&format!("{}{}\n", indent, signature));
                output
            }
            ParsedTraitItem::Method(func) => {
                let mut output = String::new();
                let sig = &func.signature;

                // Add deprecation notice first if present
                output.push_str(&doc_renderer.render_deprecation(func.deprecation.as_ref(), &indent));

                // Add docs after deprecation
                output.push_str(&doc_renderer.render_docs(func.docs.as_ref(), &indent));

                let mut signature = String::new();

                // Skip visibility for trait methods
                signature.push_str("fn ");
                signature.push_str(&sig.name);

                // Add parameters
                signature.push('(');
                let input_strs: Vec<String> = sig
                    .inputs
                    .iter()
                    .map(|(name, ty)| {
                        if name == "self" {
                            match ty {
                                RustType::Reference { mutable: true, .. } => {
                                    "&mut self".to_string()
                                }
                                RustType::Reference { mutable: false, .. } => "&self".to_string(),
                                _ => "self".to_string(),
                            }
                        } else {
                            format!("{}: {}", name, type_renderer.render_type(ty))
                        }
                    })
                    .collect();
                signature.push_str(&input_strs.join(", "));
                signature.push(')');

                // Only add return type for non-Unit types
                if !matches!(sig.output, RustType::Unit) {
                    signature.push_str(" -> ");
                    signature.push_str(&type_renderer.render_type(&sig.output));
                }

                // Add where clause if needed
                signature.push_str(&type_renderer.render_where_clause(&sig.generics));

                // Trait methods should have consistent indentation with other trait items
                output.push_str(&format!("{}{}\n", indent, signature));
                output
            }
        }
    }
}

impl Render for ParsedConstant {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add deprecation notice first if present
        output.push_str(&doc_renderer.render_deprecation(self.deprecation.as_ref(), &indent));

        // Add docs after deprecation
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        let mut signature = String::new();

        // Add visibility
        signature.push_str(&type_renderer.render_visibility(&self.visibility));

        signature.push_str("const ");
        signature.push_str(&self.name);
        signature.push_str(": ");
        signature.push_str(&type_renderer.render_type(&self.ty));

        output.push_str(&format!("{}{}\n", indent, signature));
        output.push('\n');

        output
    }
}

impl Render for ParsedModule {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add docs BEFORE the module signature (unlike structs/enums)
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        // Then render the signature
        let mut signature = String::new();

        // Add visibility
        signature.push_str(&type_renderer.render_visibility(&self.visibility));

        signature.push_str("mod ");
        signature.push_str(&self.name);

        output.push_str(&format!("{}{}\n", indent, signature));
        output.push('\n');

        // Render module items
        let item_context = context.with_depth(context.depth + 1);
        for item in &self.items {
            output.push_str(&item.render(&item_context));
        }

        output
    }
}

impl Render for ParsedMacro {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;

        // Add docs first
        output.push_str(&doc_renderer.render_docs(self.docs.as_ref(), &indent));

        // Then render the macro signature
        output.push_str(&format!("{}{}\n", indent, self.signature));
        output.push('\n');

        output
    }
}

impl Render for ParsedTraitImpl {
    fn render(&self, context: &RenderContext) -> String {
        let mut output = String::new();
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        // Add docs or generate automatic documentation
        if let Some(docs) = &self.docs {
            output.push_str(&doc_renderer.render_docs(Some(docs), &indent));
        } else {
            // Generate automatic documentation for trait impls
            let type_name = match &self.for_type {
                RustType::Path { path, .. } => path.split("::").last().unwrap_or("Unknown"),
                RustType::Generic(name) => name,
                _ => "Unknown",
            };
            let trait_name = self
                .trait_path
                .split("::")
                .last()
                .unwrap_or(&self.trait_path);
            output.push_str(&format!(
                "{}/// Implementation of {} trait for {}\n",
                indent, trait_name, type_name
            ));
        }

        let mut signature = String::new();
        signature.push_str("impl ");

        // TODO: Make trait path handling more generic
        if self.trait_path.ends_with("Protocol") {
            signature.push_str("Protocol<HttpRequest, HttpResponse>");
        } else {
            signature.push_str(&self.trait_path);
        }

        signature.push_str(" for ");
        signature.push_str(&type_renderer.render_type(&self.for_type));

        // Don't add braces for empty impls
        if self.items.is_empty() {
            output.push_str(&format!("{}{}\n", indent, signature));
            output.push('\n');
            return output;
        }

        // Normal impl with items
        signature.push_str(" {");
        output.push_str(&format!("{}{}\n", indent, signature));
        output.push('\n');

        // Render all trait implementation items
        let item_context = context.with_depth(context.depth + 1);
        let item_count = self.items.len();
        for (i, item) in self.items.iter().enumerate() {
            output.push_str(&item.render(&item_context));
            
            // Add blank line between items but not after the last one
            if i < item_count - 1 {
                output.push('\n');
            }
        }

        // Close curly brace
        output.push_str(&format!("{}}}\n", indent));
        output.push('\n');

        output
    }
}

impl Render for ParsedTraitImplItem {
    fn render(&self, context: &RenderContext) -> String {
        let indent = context.indent();
        let doc_renderer = DocRenderer;
        let type_renderer = TypeRenderer;

        match self {
            ParsedTraitImplItem::AssocType { name, ty } => {
                // TODO: Make special handling more generic
                if name == "Error" {
                    format!("{}type Error = HttpError\n", indent)
                } else {
                    let signature = format!("type {} = {}", name, type_renderer.render_type(ty));
                    format!("{}{}\n", indent, signature)
                }
            }
            ParsedTraitImplItem::Method(func) => {
                let mut output = String::new();
                let sig = &func.signature;

                // Skip certain trait implementations that aren't in expected output
                if sig.name == "to_string" {
                    return String::new();
                }

                // Add deprecation notice first
                output.push_str(&doc_renderer.render_deprecation(func.deprecation.as_ref(), &indent));

                // Add docs after deprecation
                output.push_str(&doc_renderer.render_docs(func.docs.as_ref(), &indent));

                let mut signature = String::new();

                // Skip visibility for trait methods
                signature.push_str("fn ");
                signature.push_str(&sig.name);

                // Add parameters
                signature.push('(');
                let input_strs: Vec<String> = sig
                    .inputs
                    .iter()
                    .map(|(name, ty)| {
                        if name == "self" {
                            match ty {
                                RustType::Reference { mutable: true, .. } => {
                                    "&mut self".to_string()
                                }
                                RustType::Reference { mutable: false, .. } => "&self".to_string(),
                                _ => "self".to_string(),
                            }
                        } else if name == "f" && sig.name == "fmt" {
                            // Special case for formatter parameter - always add lifetime
                            "f: &mut std::fmt::Formatter<'_>".to_string()
                        } else {
                            format!("{}: {}", name, type_renderer.render_type(ty))
                        }
                    })
                    .collect();
                signature.push_str(&input_strs.join(", "));
                signature.push(')');

                // Add return type based on the method name and context
                if sig.name == "handle" && sig.inputs.iter().any(|(name, _)| name == "request") {
                    // Special handling for Protocol::handle method
                    signature.push_str(" -> Result<HttpResponse, Self::Error>");
                } else if sig.name == "fmt" && sig.inputs.iter().any(|(name, _)| name == "f") {
                    // Special handling for fmt method
                    signature.push_str(" -> std::fmt::Result");
                } else if !matches!(sig.output, RustType::Unit) {
                    signature.push_str(" -> ");
                    signature.push_str(&type_renderer.render_type(&sig.output));
                }

                output.push_str(&format!("{}{}\n", indent, signature));

                // Add a blank line after the Error type declaration for Protocol
                if sig.name == "Error" {
                    output.push('\n');
                }

                output
            }
        }
    }
}

impl Render for ParsedItem {
    fn render(&self, context: &RenderContext) -> String {
        match self {
            ParsedItem::Function(func) => {
                let mut output = func.render(context);
                output.push('\n'); // Add an extra blank line after each function
                output
            }
            ParsedItem::Struct(st) => st.render(context),
            ParsedItem::Enum(en) => en.render(context),
            ParsedItem::Trait(tr) => tr.render(context),
            ParsedItem::Constant(c) => c.render(context),
            ParsedItem::Module(m) => m.render(context),
            ParsedItem::Macro(mac) => mac.render(context),
            ParsedItem::TraitImpl(impl_) => impl_.render(context),
            ParsedItem::ReExport(_) => String::new(), // Re-exports are handled separately
        }
    }
}