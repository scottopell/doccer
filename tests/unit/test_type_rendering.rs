#[cfg(test)]
mod tests {
    use crate::TextRenderer;
    use serde_json::json;

    fn create_test_renderer() -> TextRenderer {
        // Create a minimal Crate structure for testing
        let crate_data = crate::Crate {
            root: 1,
            crate_version: Some("0.1.0".to_string()),
            includes_private: false,
            index: std::collections::HashMap::new(),
            paths: serde_json::Value::Null,
            external_crates: serde_json::Value::Null,
            format_version: 0,
        };
        
        TextRenderer::new(crate_data)
    }

    #[test]
    fn test_primitive_types() {
        let renderer = create_test_renderer();
        
        // Test primitive types
        let types = [
            ("i32", json!({"primitive": "i32"})),
            ("String", json!({"primitive": "String"})),
            ("bool", json!({"primitive": "bool"})),
            ("f64", json!({"primitive": "f64"})),
        ];
        
        for (expected, json_value) in types {
            assert_eq!(renderer.type_to_string(&json_value), expected);
        }
    }

    #[test]
    fn test_generic_types() {
        let renderer = create_test_renderer();
        
        // Test generic type parameters
        let generic = json!({"generic": "T"});
        assert_eq!(renderer.type_to_string(&generic), "T");
        
        let generic_u = json!({"generic": "U"});
        assert_eq!(renderer.type_to_string(&generic_u), "U");
    }

    #[test]
    fn test_resolved_path_types() {
        let renderer = create_test_renderer();
        
        // Test resolved path types (like std::vec::Vec)
        let vec_type = json!({
            "resolved_path": {
                "path": "std::vec::Vec"
            }
        });
        assert_eq!(renderer.type_to_string(&vec_type), "std::vec::Vec");
        
        // Test resolved path with generic arguments
        let vec_string_type = json!({
            "resolved_path": {
                "path": "std::vec::Vec",
                "args": {
                    "angle_bracketed": {
                        "args": [
                            { "type": { "primitive": "String" } }
                        ]
                    }
                }
            }
        });
        assert_eq!(renderer.type_to_string(&vec_string_type), "std::vec::Vec<String>");
    }

    #[test]
    fn test_borrowed_ref_types() {
        let renderer = create_test_renderer();
        
        // Test borrowed references
        let ref_type = json!({
            "borrowed_ref": {
                "type": { "primitive": "str" }
            }
        });
        assert_eq!(renderer.type_to_string(&ref_type), "&str");
        
        // Test mutable borrowed references
        let ref_mut_type = json!({
            "borrowed_ref": {
                "is_mutable": true,
                "type": { "primitive": "String" }
            }
        });
        assert_eq!(renderer.type_to_string(&ref_mut_type), "&mut String");
        
        // Test borrowed references with lifetimes
        let ref_lifetime_type = json!({
            "borrowed_ref": {
                "lifetime": "a",
                "type": { "primitive": "str" }
            }
        });
        assert_eq!(renderer.type_to_string(&ref_lifetime_type), "&'a str");
    }

    #[test]
    fn test_tuple_types() {
        let renderer = create_test_renderer();
        
        // Test empty tuple (unit type)
        let unit_type = json!({
            "tuple": []
        });
        assert_eq!(renderer.type_to_string(&unit_type), "()");
        
        // Test tuple with elements
        let tuple_type = json!({
            "tuple": [
                { "primitive": "i32" },
                { "primitive": "String" }
            ]
        });
        assert_eq!(renderer.type_to_string(&tuple_type), "(i32, String)");
    }

    #[test]
    fn test_array_and_slice_types() {
        let renderer = create_test_renderer();
        
        // Test slice type
        let slice_type = json!({
            "slice": { "primitive": "u8" }
        });
        assert_eq!(renderer.type_to_string(&slice_type), "[u8]");
        
        // Test array type with length
        let array_type = json!({
            "array": {
                "type": { "primitive": "u8" },
                "len": 4
            }
        });
        assert_eq!(renderer.type_to_string(&array_type), "[u8; 4]");
        
        // Test array type without length
        let array_type_no_len = json!({
            "array": {
                "type": { "primitive": "u8" }
            }
        });
        assert_eq!(renderer.type_to_string(&array_type_no_len), "[u8; N]");
    }

    #[test]
    fn test_raw_pointer_types() {
        let renderer = create_test_renderer();
        
        // Test const raw pointer
        let const_ptr_type = json!({
            "raw_pointer": {
                "is_mutable": false,
                "type": { "primitive": "u8" }
            }
        });
        assert_eq!(renderer.type_to_string(&const_ptr_type), "*const u8");
        
        // Test mutable raw pointer
        let mut_ptr_type = json!({
            "raw_pointer": {
                "is_mutable": true,
                "type": { "primitive": "u8" }
            }
        });
        assert_eq!(renderer.type_to_string(&mut_ptr_type), "*mut u8");
    }

    #[test]
    fn test_qualified_path_types() {
        let renderer = create_test_renderer();
        
        // Test qualified paths like Self::Type
        let self_type = json!({
            "qualified_path": {
                "name": "Key"
            }
        });
        assert_eq!(renderer.type_to_string(&self_type), "Self::Key");
        
        let self_error = json!({
            "qualified_path": {
                "name": "Error"
            }
        });
        assert_eq!(renderer.type_to_string(&self_error), "Self::Error");
        
        let self_other = json!({
            "qualified_path": {
                "name": "OtherType"
            }
        });
        assert_eq!(renderer.type_to_string(&self_other), "Self::OtherType");
    }
    
    #[test]
    fn test_is_unit_type() {
        let renderer = create_test_renderer();
        
        // Test unit type detection
        let unit_type = json!({
            "tuple": []
        });
        assert!(renderer.is_unit_type(&unit_type));
        
        // Test null as unit type
        assert!(renderer.is_unit_type(&serde_json::Value::Null));
        
        // Test non-unit types
        let non_unit_types = [
            json!({"primitive": "i32"}),
            json!({"tuple": [{"primitive": "i32"}]}),
            json!({"resolved_path": {"path": "String"}}),
        ];
        
        for typ in non_unit_types {
            assert!(!renderer.is_unit_type(&typ));
        }
    }

    #[test]
    fn test_get_type_name() {
        let renderer = create_test_renderer();
        
        // Test type name extraction
        let resolved_path = json!({
            "resolved_path": {
                "path": "std::vec::Vec"
            }
        });
        assert_eq!(renderer.get_type_name(&resolved_path), Some("std::vec::Vec".to_string()));
        
        let generic = json!({"generic": "T"});
        assert_eq!(renderer.get_type_name(&generic), Some("T".to_string()));
        
        let primitive = json!({"primitive": "i32"});
        assert_eq!(renderer.get_type_name(&primitive), Some("i32".to_string()));
        
        // Test with unknown type
        let unknown = json!({"unknown_type": "something"});
        assert_eq!(renderer.get_type_name(&unknown), None);
    }
}