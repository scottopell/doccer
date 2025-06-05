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
    fn test_format_generics_empty() {
        let renderer = create_test_renderer();
        
        // Test empty generics
        let empty_generics = json!({
            "params": []
        });
        
        assert_eq!(renderer.format_generics(&empty_generics), "");
    }

    #[test]
    fn test_format_generics_with_type_params() {
        let renderer = create_test_renderer();
        
        // Test with single type parameter
        let single_type_param = json!({
            "params": [
                {
                    "name": "T",
                    "kind": {
                        "type": {}
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&single_type_param), "<T>");
        
        // Test with multiple type parameters
        let multiple_type_params = json!({
            "params": [
                {
                    "name": "T",
                    "kind": {
                        "type": {}
                    }
                },
                {
                    "name": "U",
                    "kind": {
                        "type": {}
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&multiple_type_params), "<T, U>");
    }

    #[test]
    fn test_format_generics_with_lifetimes() {
        let renderer = create_test_renderer();
        
        // Test with lifetime parameter
        let lifetime_param = json!({
            "params": [
                {
                    "name": "a",
                    "kind": {
                        "lifetime": {}
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&lifetime_param), "<'a>");
        
        // Test with lifetime parameter that already has a quote
        let quoted_lifetime_param = json!({
            "params": [
                {
                    "name": "'b",
                    "kind": {
                        "lifetime": {}
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&quoted_lifetime_param), "<'b>");
    }

    #[test]
    fn test_format_generics_with_bounds() {
        let renderer = create_test_renderer();
        
        // Test with type parameter that has bounds
        let type_with_bounds = json!({
            "params": [
                {
                    "name": "T",
                    "kind": {
                        "type": {
                            "bounds": [
                                {
                                    "trait_bound": {
                                        "trait": {
                                            "path": "std::fmt::Display"
                                        }
                                    }
                                }
                            ]
                        }
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&type_with_bounds), "<T: std::fmt::Display>");
        
        // Test with multiple bounds
        let type_with_multiple_bounds = json!({
            "params": [
                {
                    "name": "T",
                    "kind": {
                        "type": {
                            "bounds": [
                                {
                                    "trait_bound": {
                                        "trait": {
                                            "path": "std::fmt::Display"
                                        }
                                    }
                                },
                                {
                                    "trait_bound": {
                                        "trait": {
                                            "path": "std::clone::Clone"
                                        }
                                    }
                                }
                            ]
                        }
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&type_with_multiple_bounds), "<T: std::fmt::Display + std::clone::Clone>");
    }

    #[test]
    fn test_format_generics_with_where_clause() {
        let renderer = create_test_renderer();
        
        // Test with where clause
        let with_where_clause = json!({
            "params": [
                {
                    "name": "T",
                    "kind": {
                        "type": {}
                    }
                }
            ],
            "where_predicates": [
                {
                    "bound_predicate": {
                        "type": { "generic": "T" },
                        "bounds": [
                            {
                                "trait_bound": {
                                    "trait": {
                                        "path": "std::fmt::Display"
                                    }
                                }
                            }
                        ]
                    }
                }
            ]
        });
        
        assert_eq!(renderer.format_generics(&with_where_clause), "<T> where T: std::fmt::Display");
        
        // Test with multiple where predicates
        let with_multiple_where_predicates = json!({
            "params": [
                {
                    "name": "T",
                    "kind": {
                        "type": {}
                    }
                },
                {
                    "name": "U",
                    "kind": {
                        "type": {}
                    }
                }
            ],
            "where_predicates": [
                {
                    "bound_predicate": {
                        "type": { "generic": "T" },
                        "bounds": [
                            {
                                "trait_bound": {
                                    "trait": {
                                        "path": "std::fmt::Display"
                                    }
                                }
                            }
                        ]
                    }
                },
                {
                    "bound_predicate": {
                        "type": { "generic": "U" },
                        "bounds": [
                            {
                                "trait_bound": {
                                    "trait": {
                                        "path": "std::clone::Clone"
                                    }
                                }
                            }
                        ]
                    }
                }
            ]
        });
        
        assert_eq!(
            renderer.format_generics(&with_multiple_where_predicates), 
            "<T, U> where T: std::fmt::Display, U: std::clone::Clone"
        );
    }

    #[test]
    fn test_format_bound() {
        let renderer = create_test_renderer();
        
        // Test valid trait bound
        let trait_bound = json!({
            "trait_bound": {
                "trait": {
                    "path": "std::fmt::Display"
                }
            }
        });
        
        assert_eq!(renderer.format_bound(&trait_bound), Some("std::fmt::Display".to_string()));
        
        // Test invalid trait bound (missing path)
        let invalid_bound = json!({
            "trait_bound": {
                "trait": {}
            }
        });
        
        assert_eq!(renderer.format_bound(&invalid_bound), None);
        
        // Test non-trait bound
        let non_trait_bound = json!({
            "other_bound": {}
        });
        
        assert_eq!(renderer.format_bound(&non_trait_bound), None);
    }

    #[test]
    fn test_format_where_predicate() {
        let renderer = create_test_renderer();
        
        // Test valid where predicate
        let where_predicate = json!({
            "bound_predicate": {
                "type": { "generic": "T" },
                "bounds": [
                    {
                        "trait_bound": {
                            "trait": {
                                "path": "std::fmt::Display"
                            }
                        }
                    }
                ]
            }
        });
        
        assert_eq!(
            renderer.format_where_predicate(&where_predicate), 
            Some("T: std::fmt::Display".to_string())
        );
        
        // Test where predicate with multiple bounds
        let multiple_bounds_predicate = json!({
            "bound_predicate": {
                "type": { "generic": "T" },
                "bounds": [
                    {
                        "trait_bound": {
                            "trait": {
                                "path": "std::fmt::Display"
                            }
                        }
                    },
                    {
                        "trait_bound": {
                            "trait": {
                                "path": "std::clone::Clone"
                            }
                        }
                    }
                ]
            }
        });
        
        assert_eq!(
            renderer.format_where_predicate(&multiple_bounds_predicate), 
            Some("T: std::fmt::Display + std::clone::Clone".to_string())
        );
        
        // Test invalid where predicate (missing bounds)
        let invalid_predicate = json!({
            "bound_predicate": {
                "type": { "generic": "T" }
            }
        });
        
        assert_eq!(renderer.format_where_predicate(&invalid_predicate), None);
        
        // Test non-bound predicate
        let non_bound_predicate = json!({
            "other_predicate": {}
        });
        
        assert_eq!(renderer.format_where_predicate(&non_bound_predicate), None);
    }
}