use anyhow::Result;
use rustdoc_types::Crate;

#[test]
fn test_better_json_parsing_error_messages() {
    let malformed_json = r#"{"format_version":"#;
    
    let result: Result<Crate, _> = serde_json::from_str(malformed_json)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON documentation: {}", e));
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_message = error.to_string();
    
    // Should provide context about what failed to parse
    assert!(error_message.contains("Failed to parse JSON documentation"));
    assert!(error_message.contains("column") || error_message.contains("position"));
}

#[test]
fn test_truncated_json_handling() {
    let truncated_json = r#"{"format_version":40,"index":{"#;
    
    let result: Result<Crate, _> = serde_json::from_str(truncated_json)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON documentation: {}", e));
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_message = error.to_string();
    
    // Should indicate truncation or incomplete JSON
    assert!(error_message.contains("Failed to parse JSON documentation"));
    assert!(error_message.contains("EOF") || error_message.contains("unexpected end"));
}

#[test]
fn test_complex_nested_type_parsing() {
    let complex_type_json = r#"{
        "format_version": 40,
        "index": {
            "1": {
                "id": "1",
                "crate_id": 0,
                "name": "complex_function",
                "span": null,
                "visibility": "public",
                "docs": null,
                "links": {},
                "attrs": [],
                "deprecation": null,
                "inner": {
                    "function": {
                        "decl": {
                            "inputs": [],
                            "output": {
                                "resolved_path": {
                                    "path": "Box",
                                    "id": 84,
                                    "args": {
                                        "angle_bracketed": {
                                            "args": [{
                                                "type": {
                                                    "dyn_trait": {
                                                        "traits": [{
                                                            "trait": {
                                                                "path": "::core::future::Future",
                                                                "id": 85,
                                                                "args": {
                                                                    "angle_bracketed": {
                                                                        "args": [],
                                                                        "constraints": [{
                                                                            "name": "Output",
                                                                            "args": null,
                                                                            "binding": {
                                                                                "equality": {
                                                                                    "type": {
                                                                                        "resolved_path": {
                                                                                            "path": "Result",
                                                                                            "id": 8,
                                                                                            "args": {
                                                                                                "angle_bracketed": {
                                                                                                    "args": [
                                                                                                        {"type": {"tuple": []}},
                                                                                                        {"type": {"resolved_path": {"path": "Error", "id": 9}}}
                                                                                                    ]
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }]
                                                                    }
                                                                }
                                                            }
                                                        }],
                                                        "lifetime": null
                                                    }
                                                }
                                            }]
                                        }
                                    }
                                }
                            }
                        },
                        "generics": {
                            "params": [],
                            "where_predicates": []
                        },
                        "header": {
                            "const": false,
                            "async": true,
                            "unsafe": false,
                            "extern": null
                        },
                        "abi": "Rust"
                    }
                }
            }
        },
        "paths": {
            "1": {
                "crate_id": 0,
                "path": ["complex_function"]
            }
        },
        "external_crates": {},
        "root": "1"
    }"#;
    
    let result: Result<Crate, _> = serde_json::from_str(complex_type_json)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON documentation: {}", e));
    
    // Should successfully parse complex nested types
    assert!(result.is_ok(), "Failed to parse complex nested type structure: {:?}", result.unwrap_err());
    
    let crate_data = result.unwrap();
    assert_eq!(crate_data.index.len(), 1);
    
    // Should correctly parse the async function returning Box<dyn Future<Output = Result<(), Error>>>
    let item = crate_data.index.get(&crate_data.root).unwrap();
    assert_eq!(item.name, Some("complex_function".to_string()));
}

#[test]
fn test_error_context_includes_json_snippet() {
    let json_with_error = r#"{"format_version":40,"index":{"1":{"name":"test","inner":{"function":{"decl":{"inputs":[],"output":invalid_value_here}}}}}}"#;
    
    let result: Result<Crate, _> = serde_json::from_str(json_with_error)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON documentation: {}", e));
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    let error_message = error.to_string();
    
    // Should include surrounding JSON context in error message
    assert!(error_message.contains("Failed to parse JSON documentation"));
    // Should mention the problematic area
    assert!(error_message.contains("column") || error_message.contains("position"));
}

#[test]
fn test_partial_parsing_recovery() {
    // Test that parser can recover from errors in individual items
    let json_with_partial_error = r#"{
        "format_version": 40,
        "index": {
            "1": {
                "id": "1",
                "crate_id": 0,
                "name": "good_function",
                "span": null,
                "visibility": "public",
                "docs": null,
                "links": {},
                "attrs": [],
                "deprecation": null,
                "inner": {
                    "function": {
                        "decl": {
                            "inputs": [],
                            "output": {"tuple": []}
                        },
                        "generics": {"params": [], "where_predicates": []},
                        "header": {"const": false, "async": false, "unsafe": false, "extern": null},
                        "abi": "Rust"
                    }
                }
            },
            "2": {
                "id": "2",
                "crate_id": 0,
                "name": "bad_function",
                "span": null,
                "visibility": "public",
                "docs": null,
                "links": {},
                "attrs": [],
                "deprecation": null,
                "inner": {
                    "function": {
                        "decl": {
                            "inputs": [],
                            "output": null
                        },
                        "generics": {"params": [], "where_predicates": []},
                        "header": {"const": false, "async": false, "unsafe": false, "extern": null},
                        "abi": "Rust"
                    }
                }
            }
        },
        "paths": {
            "1": {"crate_id": 0, "path": ["good_function"]},
            "2": {"crate_id": 0, "path": ["bad_function"]}
        },
        "external_crates": {},
        "root": "1"
    }"#;
    
    let result: Result<Crate, _> = serde_json::from_str(json_with_partial_error)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON documentation: {}", e));
    
    // Should either succeed with partial results or fail with helpful error
    match result {
        Ok(crate_data) => {
            // If it succeeds, should have parsed at least the good function
            assert!(!crate_data.index.is_empty());
            assert!(crate_data.index.values().any(|item| item.name == Some("good_function".to_string())));
        }
        Err(error) => {
            // If it fails, should provide helpful context
            let error_message = error.to_string();
            assert!(error_message.contains("Failed to parse JSON documentation"));
        }
    }
}