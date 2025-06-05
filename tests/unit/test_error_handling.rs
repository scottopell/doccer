#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::collections::HashMap;
    use serde_json::json;
    
    // Test error handling in TextRenderer's render method
    #[test]
    fn test_render_missing_root_item() -> Result<()> {
        // Create a crate with a root ID that doesn't exist in the index
        let crate_data = crate::Crate {
            root: 999, // This ID doesn't exist in the index
            crate_version: None,
            includes_private: false,
            index: HashMap::new(),
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        let renderer = crate::TextRenderer::new(crate_data);
        let result = renderer.render();
        
        // The render method should still succeed, even with a missing root item
        assert!(result.is_ok());
        
        // The output should be empty or very minimal
        let output = result?;
        assert!(output.contains("# Crate: unknown"));
        
        Ok(())
    }
    
    // Test error handling in render_item_with_trait_control method
    #[test]
    fn test_render_item_missing_from_index() -> Result<()> {
        // Create a minimal crate with some items
        let mut index = HashMap::new();
        
        // Add one item to the index
        index.insert("1".to_string(), crate::Item {
            id: Some(1),
            crate_id: 0,
            name: Some("test_item".to_string()),
            span: None,
            visibility: crate::Visibility::Simple("public".to_string()),
            docs: None,
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"module": {"items": [2]}}), // Reference to item 2, which doesn't exist
        });
        
        let crate_data = crate::Crate {
            root: 1,
            crate_version: None,
            includes_private: false,
            index,
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        let renderer = crate::TextRenderer::new(crate_data);
        
        // Test rendering an item that doesn't exist in the index
        let mut output = String::new();
        let result = renderer.render_item_with_trait_control("999", &mut output, 0, false);
        
        // The method should handle the missing item gracefully and return Ok
        assert!(result.is_ok());
        assert_eq!(output, ""); // Output should be empty for missing item
        
        Ok(())
    }
    
    // Test handling of malformed inner JSON in items
    #[test]
    fn test_malformed_inner_json() -> Result<()> {
        // Create a minimal crate with a malformed item
        let mut index = HashMap::new();
        
        // Add a malformed item to the index (inner is not an object)
        index.insert("1".to_string(), crate::Item {
            id: Some(1),
            crate_id: 0,
            name: Some("malformed_item".to_string()),
            span: None,
            visibility: crate::Visibility::Simple("public".to_string()),
            docs: None,
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!("not an object"), // This should be an object, not a string
        });
        
        let crate_data = crate::Crate {
            root: 1,
            crate_version: None,
            includes_private: false,
            index,
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        let renderer = crate::TextRenderer::new(crate_data);
        
        // The render method should not panic with malformed JSON
        let result = renderer.render();
        assert!(result.is_ok());
        
        Ok(())
    }
    
    // Test handling deprecation with empty fields
    #[test]
    fn test_deprecation_with_empty_fields() -> Result<()> {
        // Create a minimal crate with an item that has empty deprecation fields
        let mut index = HashMap::new();
        
        // Add an item with empty deprecation fields
        index.insert("1".to_string(), crate::Item {
            id: Some(1),
            crate_id: 0,
            name: Some("deprecated_item".to_string()),
            span: None,
            visibility: crate::Visibility::Simple("public".to_string()),
            docs: None,
            links: HashMap::new(),
            attrs: vec![],
            deprecation: Some(crate::Deprecation {
                since: None,
                note: None,
            }),
            inner: json!({"function": {}}),
        });
        
        let crate_data = crate::Crate {
            root: 1,
            crate_version: None,
            includes_private: false,
            index,
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        let renderer = crate::TextRenderer::new(crate_data);
        let mut output = String::new();
        
        // Test rendering deprecation notice with empty fields
        renderer.render_deprecation(&index["1"], &mut output, "  ");
        
        // Should show DEPRECATED without a since version
        assert!(output.contains("  DEPRECATED\n"));
        assert!(!output.contains("since"));
        
        Ok(())
    }

    // Test handling of unknown item types
    #[test]
    fn test_unknown_item_type() -> Result<()> {
        // Create a minimal crate with an item of unknown type
        let mut index = HashMap::new();
        
        // Add an item with an unknown inner type
        index.insert("1".to_string(), crate::Item {
            id: Some(1),
            crate_id: 0,
            name: Some("unknown_item".to_string()),
            span: None,
            visibility: crate::Visibility::Simple("public".to_string()),
            docs: Some("Documentation for unknown item".to_string()),
            links: HashMap::new(),
            attrs: vec![],
            deprecation: None,
            inner: json!({"unknown_type": {}}),
        });
        
        let crate_data = crate::Crate {
            root: 1,
            crate_version: None,
            includes_private: false,
            index,
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        let renderer = crate::TextRenderer::new(crate_data);
        let result = renderer.render();
        
        // The render method should succeed even with unknown item types
        assert!(result.is_ok());
        
        // The output should contain the item name and documentation
        let output = result?;
        assert!(output.contains("unknown_item"));
        assert!(output.contains("Documentation for unknown item"));
        
        Ok(())
    }
}