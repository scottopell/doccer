#[cfg(test)]
mod tests {
    use crate::{TextRenderer, Crate, Item, Visibility, Deprecation};
    use std::collections::HashMap;
    use serde_json::json;
    use anyhow::Result;

    fn create_test_crate() -> Crate {
        // Create a minimal crate with some items for testing
        let mut index = HashMap::new();
        
        // Add a root module
        index.insert("1".to_string(), Item {
            id: Some(1),
            crate_id: 0,
            name: Some("test_crate".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("This is a test crate".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"module": {"items": [2, 3, 4]}}),
        });
        
        // Add a function
        index.insert("2".to_string(), Item {
            id: Some(2),
            crate_id: 0,
            name: Some("test_function".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("This is a test function".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({
                "function": {
                    "sig": {
                        "inputs": [
                            ["param1", {"primitive": "i32"}],
                            ["param2", {"primitive": "String"}]
                        ],
                        "output": {"primitive": "bool"}
                    }
                }
            }),
        });
        
        // Add a struct
        index.insert("3".to_string(), Item {
            id: Some(3),
            crate_id: 0,
            name: Some("TestStruct".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("This is a test struct".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({
                "struct": {
                    "kind": "plain",
                    "fields": [5, 6],
                    "impls": []
                }
            }),
        });
        
        // Add a deprecated enum
        index.insert("4".to_string(), Item {
            id: Some(4),
            crate_id: 0,
            name: Some("TestEnum".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("This is a test enum".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: Some(Deprecation {
                since: Some("1.0.0".to_string()),
                note: Some("Use something else".to_string()),
            }),
            inner: json!({
                "enum": {
                    "variants": [7, 8]
                }
            }),
        });
        
        // Add struct fields
        index.insert("5".to_string(), Item {
            id: Some(5),
            crate_id: 0,
            name: Some("field1".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("First field".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"struct_field": {"primitive": "i32"}}),
        });
        
        index.insert("6".to_string(), Item {
            id: Some(6),
            crate_id: 0,
            name: Some("field2".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("Second field".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"struct_field": {"primitive": "String"}}),
        });
        
        // Add enum variants
        index.insert("7".to_string(), Item {
            id: Some(7),
            crate_id: 0,
            name: Some("Variant1".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("First variant".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"variant": {"kind": "plain"}}),
        });
        
        index.insert("8".to_string(), Item {
            id: Some(8),
            crate_id: 0,
            name: Some("Variant2".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("Second variant".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({
                "variant": {
                    "kind": {
                        "tuple": [9]
                    }
                }
            }),
        });
        
        // Add variant field
        index.insert("9".to_string(), Item {
            id: Some(9),
            crate_id: 0,
            name: None,
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: None,
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"struct_field": {"primitive": "String"}}),
        });
        
        Crate {
            root: 1,
            crate_version: Some("0.1.0".to_string()),
            includes_private: false,
            index,
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        }
    }

    #[test]
    fn test_render_function() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        let mut output = String::new();
        renderer.render_item("2", &mut output, 0)?;
        
        // Check that function signature is rendered correctly
        assert!(output.contains("pub fn test_function(param1: i32, param2: String) -> bool"));
        
        // Check that docs are rendered
        assert!(output.contains("This is a test function"));
        
        Ok(())
    }

    #[test]
    fn test_render_struct() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        let mut output = String::new();
        renderer.render_item("3", &mut output, 0)?;
        
        // Check that struct is rendered correctly
        assert!(output.contains("pub struct TestStruct { ... }"));
        
        // Check that docs are rendered
        assert!(output.contains("This is a test struct"));
        
        Ok(())
    }

    #[test]
    fn test_render_enum_with_deprecation() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        let mut output = String::new();
        renderer.render_item("4", &mut output, 0)?;
        
        // Check that enum is rendered correctly
        assert!(output.contains("pub enum TestEnum { ... }"));
        
        // Check that deprecation notice is rendered
        assert!(output.contains("DEPRECATED since 1.0.0"));
        
        // Check that docs are rendered
        assert!(output.contains("This is a test enum"));
        
        // Check that variants are rendered
        assert!(output.contains("Variant1"));
        assert!(output.contains("First variant"));
        assert!(output.contains("Variant2(String)"));
        assert!(output.contains("Second variant"));
        
        Ok(())
    }

    #[test]
    fn test_render_deprecation() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        // Get the deprecated enum item
        let enum_item = &crate_data.index["4"];
        
        let mut output = String::new();
        renderer.render_deprecation(enum_item, &mut output, "  ");
        
        // Check that deprecation notice is rendered correctly
        assert!(output.contains("  DEPRECATED since 1.0.0"));
        
        Ok(())
    }

    #[test]
    fn test_render_struct_field() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        let field_item = &crate_data.index["5"];
        let field_data = field_item.inner.get("struct_field").unwrap();
        
        let mut output = String::new();
        renderer.render_struct_field(field_item, field_data, &mut output, "  ")?;
        
        // Check that field is rendered correctly
        assert!(output.contains("  pub field1: i32"));
        
        // Check that docs are rendered
        assert!(output.contains("First field"));
        
        Ok(())
    }

    #[test]
    fn test_render_variant() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        // Test plain variant
        let variant_item = &crate_data.index["7"];
        let variant_data = variant_item.inner.get("variant").unwrap();
        
        let mut output = String::new();
        renderer.render_variant(variant_item, variant_data, &mut output, "  ")?;
        
        // Check that variant is rendered correctly
        assert!(output.contains("  Variant1"));
        
        // Check that docs are rendered
        assert!(output.contains("First variant"));
        
        // Test tuple variant
        let variant_item = &crate_data.index["8"];
        let variant_data = variant_item.inner.get("variant").unwrap();
        
        let mut output = String::new();
        renderer.render_variant(variant_item, variant_data, &mut output, "  ")?;
        
        // Check that variant is rendered correctly with tuple field
        assert!(output.contains("  Variant2(String)"));
        
        Ok(())
    }

    #[test]
    fn test_render_full_crate() -> Result<()> {
        let crate_data = create_test_crate();
        let renderer = TextRenderer::new(crate_data);
        
        let output = renderer.render()?;
        
        // Check that crate header is rendered
        assert!(output.contains("# Crate: test_crate"));
        assert!(output.contains("Version: 0.1.0"));
        assert!(output.contains("This is a test crate"));
        
        // Check that function is rendered
        assert!(output.contains("pub fn test_function(param1: i32, param2: String) -> bool"));
        
        // Check that struct is rendered
        assert!(output.contains("pub struct TestStruct { ... }"));
        
        // Check that enum is rendered with deprecation notice
        assert!(output.contains("pub enum TestEnum { ... }"));
        assert!(output.contains("DEPRECATED since 1.0.0"));
        
        Ok(())
    }

    #[test]
    fn test_restricted_visibility() -> Result<()> {
        // Create a crate with a restricted visibility item
        let mut index = HashMap::new();
        
        // Add a root module
        index.insert("1".to_string(), Item {
            id: Some(1),
            crate_id: 0,
            name: Some("test_crate".to_string()),
            span: None,
            visibility: Visibility::Simple("public".to_string()),
            docs: Some("This is a test crate".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"module": {"items": [2]}}),
        });
        
        // Add a struct field with restricted visibility
        index.insert("2".to_string(), Item {
            id: Some(2),
            crate_id: 0,
            name: Some("private_field".to_string()),
            span: None,
            visibility: Visibility::Restricted {
                restricted: crate::RestrictedVisibility {
                    parent: "crate".to_string(),
                    path: "test_crate".to_string(),
                }
            },
            docs: Some("This field has crate visibility".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"struct_field": {"primitive": "i32"}}),
        });
        
        let crate_data = Crate {
            root: 1,
            crate_version: None,
            includes_private: false,
            index,
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        let renderer = TextRenderer::new(crate_data);
        
        let field_item = &renderer.crate_data.index["2"];
        let field_data = field_item.inner.get("struct_field").unwrap();
        
        let mut output = String::new();
        renderer.render_struct_field(field_item, field_data, &mut output, "  ")?;
        
        // Check that field is rendered with pub(crate) visibility
        assert!(output.contains("  pub(crate) private_field: i32"));
        
        Ok(())
    }
}