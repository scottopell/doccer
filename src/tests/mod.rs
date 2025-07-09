#[cfg(test)]
mod formatting_tests {
    use serde_json::json;
    use std::collections::HashMap;
    use rustdoc_types::{Crate, Visibility, Deprecation, Id, Target};
    use crate::{ParsedRenderer, ParsedFunction, FunctionSignature, RustType, Generics, GenericParam, GenericParamKind, ParsedTraitImplItem, ParsedTraitImpl, ParsedTraitItem, ParsedModule, ParsedStruct, ParsedItem, RenderContext, Render};

    // Helper function to create minimal test items
    fn create_test_item(kind: &str) -> serde_json::Value {
        json!({
            "id": "test::Item",
            "crate_id": "test",
            "name": "Item",
            "kind": kind,
            "inner": {},
            "docs": "",
            "links": {},
            "attrs": {}
        })
    }

    fn create_test_crate() -> Crate {
        Crate {
            root: Id(0),
            crate_version: Some("0.1.0".to_string()),
            includes_private: false,
            index: HashMap::new(),
            paths: HashMap::new(),
            external_crates: HashMap::new(),
            format_version: 53,
            target: Target {
                triple: "x86_64-unknown-linux-gnu".to_string(),
                target_features: vec![],
            },
        }
    }
    
    fn create_parsed_renderer() -> ParsedRenderer {
        ParsedRenderer
    }

    #[test]
    fn test_trait_impl_indentation() {
        // Test that trait implementations properly indent method signatures using the new ParsedRenderer
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create a sample trait implementation
        let trait_impl = ParsedTraitImpl {
            trait_path: "Named".to_string(),
            for_type: RustType::Path { 
                path: "Person".to_string(), 
                generics: vec![] 
            },
            items: vec![
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "name".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                })
                            ],
                            output: RustType::Reference { 
                                lifetime: None, 
                                mutable: false, 
                                inner: Box::new(RustType::Primitive("str".to_string())) 
                            }
                        },
                        docs: None,
                        deprecation: None,
                    }
                )
            ],
            docs: Some("Implementation of Named trait for Person".to_string()),
        };
        
        // Call the renderer function using the new trait-based approach
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_impl.render(&context));
        
        // Check for exact indentation - should be 4 spaces for trait method implementations
        assert!(output.contains("impl Named for Person"));
        assert!(output.contains("\n    fn name("));
        
        // Verify the exact indentation level - 4 spaces, not 6 or 8
        let lines: Vec<&str> = output.lines().collect();
        let method_line = lines.iter().find(|line| line.contains("fn name")).unwrap();
        assert_eq!(method_line.chars().take(4).filter(|c| *c == ' ').count(), 4);
    }

    #[test]
    fn test_trait_method_impl_indentation() {
        // Test indentation in a trait implementation with multiple methods
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create a trait implementation with multiple methods
        let trait_impl = ParsedTraitImpl {
            trait_path: "Handler".to_string(),
            for_type: RustType::Path { 
                path: "DefaultHandler".to_string(), 
                generics: vec![] 
            },
            items: vec![
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "process".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                })
                            ],
                            output: RustType::Path {
                                path: "Result".to_string(),
                                generics: vec![
                                    RustType::Unit,
                                    RustType::Primitive("String".to_string())
                                ]
                            }
                        },
                        docs: None,
                        deprecation: None,
                    }
                ),
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "handle_error".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                }),
                                ("_error".to_string(), RustType::Reference {
                                    lifetime: None,
                                    mutable: false,
                                    inner: Box::new(RustType::Primitive("str".to_string()))
                                })
                            ],
                            output: RustType::Unit
                        },
                        docs: None,
                        deprecation: Some(Deprecation {
                            since: Some("1.2.5".to_string()),
                            note: None,
                        }),
                    }
                )
            ],
            docs: None,
        };
        
        // Call the renderer function using the new trait-based approach
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_impl.render(&context));
        
        // Check both methods have consistent indentation
        let lines: Vec<&str> = output.lines().collect();
        
        // Find the method lines
        let process_line = lines.iter().find(|line| line.contains("fn process")).unwrap();
        let handle_error_line = lines.iter().find(|line| line.contains("fn handle_error")).unwrap();
        
        // Both should have 4 spaces of indentation
        assert_eq!(process_line.chars().take(4).filter(|c| *c == ' ').count(), 4);
        assert_eq!(handle_error_line.chars().take(4).filter(|c| *c == ' ').count(), 4);
        
        // The deprecation notice should be rendered and properly indented
        assert!(output.contains("DEPRECATED since 1.2.5"));
        
        // Check that both methods are present
        assert!(output.contains("fn process(&self) -> Result<(), String>"));
        assert!(output.contains("fn handle_error(&self, _error: &str)"));
    }

    #[test]
    fn test_formatter_lifetime_param() {
        // Test that formatter parameters properly include lifetime annotations
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create Debug trait implementation
        let trait_impl = ParsedTraitImpl {
            trait_path: "Debug".to_string(),
            for_type: RustType::Path { 
                path: "HttpError".to_string(), 
                generics: vec![] 
            },
            items: vec![
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "fmt".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                }),
                                ("f".to_string(), RustType::Reference {
                                    lifetime: None,
                                    mutable: true,
                                    inner: Box::new(RustType::Path {
                                        path: "std::fmt::Formatter".to_string(),
                                        generics: vec![]
                                    })
                                })
                            ],
                            output: RustType::Path {
                                path: "std::fmt::Result".to_string(),
                                generics: vec![]
                            }
                        },
                        docs: None,
                        deprecation: None,
                    }
                )
            ],
            docs: None,
        };
        
        // Call the renderer function using the new trait-based approach
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_impl.render(&context));

        // Check for lifetime annotation
        assert!(output.contains("<'_>"));
        
        // Check for correct formatter path - should use std::fmt::Formatter, not $crate::fmt
        assert!(output.contains("&mut std::fmt::Formatter<'_>"));
        assert!(!output.contains("$crate::fmt::Formatter"));
    }

    #[test]
    fn test_display_formatter_path() {
        // Test that Display trait formatter uses std::fmt path, not $crate
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create Display trait implementation
        let trait_impl = ParsedTraitImpl {
            trait_path: "Display".to_string(),
            for_type: RustType::Path { 
                path: "HttpError".to_string(), 
                generics: vec![] 
            },
            items: vec![
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "fmt".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                }),
                                ("f".to_string(), RustType::Reference {
                                    lifetime: None,
                                    mutable: true,
                                    inner: Box::new(RustType::Path {
                                        path: "std::fmt::Formatter".to_string(),
                                        generics: vec![]
                                    })
                                })
                            ],
                            output: RustType::Path {
                                path: "std::fmt::Result".to_string(),
                                generics: vec![]
                            }
                        },
                        docs: None,
                        deprecation: None,
                    }
                )
            ],
            docs: None,
        };
        
        // Call the renderer function using the new trait-based approach
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_impl.render(&context));

        // Should use std::fmt namespace for Display trait
        assert!(output.contains("&mut std::fmt::Formatter<'_>"));
        assert!(!output.contains("$crate::fmt::Formatter"));
    }

    #[test]
    fn test_doc_comment_whitespace() {
        // Test that documentation comments have consistent whitespace
        let docs = "A macro for creating formatted messages\n\n# Examples\n\n```\nlet msg = format_message!(\"Hello\", \"World\");\nassert_eq!(msg, \"Hello: World\");\n```";

        let mut output = String::new();
        let renderer = create_parsed_renderer();
        
        // Call the renderer function
        let doc_renderer = crate::renderer::DocRenderer;
        output.push_str(&doc_renderer.render_docs(Some(&docs.to_string()), "  "));

        // Should have a single space after the doc comment prefix
        assert!(output.contains("/// A macro"));
        
        // Should properly handle empty lines - without trailing spaces
        assert!(output.contains("///\n"));
        assert!(!output.contains("/// \n"));
        
        // Should not have any lines with additional spaces after the prefix
        assert!(!output.contains("///  "));
        
        // Check for consistency in all lines
        for line in output.lines() {
            if line.starts_with("///") && line.len() > 3 {
                assert_eq!(&line[0..4], "/// ", "Line should have exactly one space after ///");
            }
        }
    }

    #[test]
    fn test_function_return_type() {
        // Test that function return types are not rendered with "-> ..." suffix
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        let func = ParsedFunction {
            signature: FunctionSignature {
                name: "add".to_string(),
                visibility: Visibility::Public,
                generics: Generics {
                    params: vec![],
                    where_clauses: vec![],
                },
                inputs: vec![
                    ("self".to_string(), RustType::Reference { 
                        lifetime: None, 
                        mutable: true, 
                        inner: Box::new(RustType::Generic("Self".to_string())) 
                    }),
                    ("key".to_string(), RustType::Primitive("String".to_string())),
                    ("content".to_string(), RustType::Primitive("String".to_string()))
                ],
                output: RustType::Unit,
            },
            docs: None,
            deprecation: None,
        };
        
        // Call the renderer function
        let context = RenderContext::new().with_depth(1);
        output.push_str(&func.render(&context));

        // Should not add "-> ..." to methods with no return type
        assert!(!output.contains("-> ..."));
        assert!(output.contains("pub fn add("));
    }
    
    #[test]
    fn test_function_with_unit_return_type() {
        // Test function with explicit unit return type ()
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        let func = ParsedFunction {
            signature: FunctionSignature {
                name: "set_timeout".to_string(),
                visibility: Visibility::Public,
                generics: Generics {
                    params: vec![],
                    where_clauses: vec![],
                },
                inputs: vec![
                    ("self".to_string(), RustType::Reference { 
                        lifetime: None, 
                        mutable: true, 
                        inner: Box::new(RustType::Generic("Self".to_string())) 
                    }),
                    ("seconds".to_string(), RustType::Primitive("u32".to_string()))
                ],
                output: RustType::Unit,  // Explicit unit type
            },
            docs: None,
            deprecation: None,
        };
        
        // Call the renderer function
        let context = RenderContext::new().with_depth(1);
        output.push_str(&func.render(&context));

        // Unit return type should be omitted (standard Rust syntax)
        assert!(!output.contains("-> ()"));
        assert!(!output.contains("-> ..."));
    }
    
    #[test]
    fn test_function_with_missing_return_type() {
        // Test function with completely missing return type (not even null)
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        let func = ParsedFunction {
            signature: FunctionSignature {
                name: "handle_error".to_string(),
                visibility: Visibility::Public,
                generics: Generics {
                    params: vec![],
                    where_clauses: vec![],
                },
                inputs: vec![
                    ("self".to_string(), RustType::Reference { 
                        lifetime: None, 
                        mutable: false, 
                        inner: Box::new(RustType::Generic("Self".to_string())) 
                    }),
                    ("error".to_string(), RustType::Reference {
                        lifetime: None,
                        mutable: false,
                        inner: Box::new(RustType::Primitive("str".to_string()))
                    })
                ],
                output: RustType::Unit,  // Missing output means unit
            },
            docs: None,
            deprecation: None,
        };
        
        // Call the renderer function
        let context = RenderContext::new().with_depth(1);
        output.push_str(&func.render(&context));

        // Should not add "-> ..." to methods with missing return type
        assert!(!output.contains("-> ..."));
        assert!(output.contains("pub fn handle_error("));
    }

    #[test]
    fn test_struct_with_where_clause() {
        // Test that structs with type constraints show proper where clauses
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        let struct_def = ParsedStruct {
            name: "Cache".to_string(),
            visibility: Visibility::Public,
            generics: Generics {
                params: vec![
                    GenericParam {
                        name: "'a".to_string(),
                        kind: GenericParamKind::Lifetime,
                    },
                    GenericParam {
                        name: "T".to_string(),
                        kind: GenericParamKind::Type {
                            bounds: vec!["Cacheable".to_string()],
                        },
                    }
                ],
                where_clauses: vec![],
            },
            methods: vec![],  // Empty for test
            trait_impls: vec![],
            docs: None,
            deprecation: None,
        };
        
        // Call the renderer function
        let context = RenderContext::new().with_depth(1);
        output.push_str(&struct_def.render(&context));

        // Should show the type constraint in the struct definition
        assert!(output.contains("pub struct Cache<'a, T: Cacheable>"));
        // Should not omit the constraint
        assert!(!output.contains("pub struct Cache<'a, T>"));
    }
    
    #[test]
    fn test_complex_struct_generics() {
        // Test a struct with multiple generic parameters and complex constraints
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        let struct_def = ParsedStruct {
            name: "Storage".to_string(),
            visibility: Visibility::Public,
            generics: Generics {
                params: vec![
                    GenericParam {
                        name: "K".to_string(),
                        kind: GenericParamKind::Type {
                            bounds: vec![
                                "Clone".to_string(),
                                "Debug".to_string(),
                                "PartialEq".to_string(),
                                "std::hash::Hash".to_string()
                            ],
                        },
                    },
                    GenericParam {
                        name: "V".to_string(),
                        kind: GenericParamKind::Type {
                            bounds: vec![
                                "Clone".to_string(),
                                "Debug".to_string()
                            ],
                        },
                    }
                ],
                where_clauses: vec![],
            },
            methods: vec![],  // Empty for test
            trait_impls: vec![],
            docs: None,
            deprecation: None,
        };
        
        // Call the renderer function
        let context = RenderContext::new().with_depth(1);
        output.push_str(&struct_def.render(&context));

        // All bounds should be preserved in output
        assert!(output.contains("pub struct Storage<K: Clone + Debug + PartialEq + std::hash::Hash, V: Clone + Debug>"));
    }

    #[test]
    fn test_trait_impl_block_style() {
        // Test that trait implementations have proper syntax (with or without braces)
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create an empty trait implementation
        let trait_impl = ParsedTraitImpl {
            trait_path: "Error".to_string(),
            for_type: RustType::Path { 
                path: "HttpError".to_string(), 
                generics: vec![] 
            },
            items: vec![],  // Empty items
            docs: None,
        };
        
        // Call the renderer function using the new trait-based approach
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_impl.render(&context));

        // Empty trait impls should not have braces with nothing inside
        assert!(output.contains("impl Error for HttpError"));
        assert!(!output.contains("impl Error for HttpError {"));
        assert!(!output.contains("impl Error for HttpError {\n\n}"));
    }
    
    #[test]
    fn test_all_trait_impls_rendered() {
        // Test that all trait implementations are rendered, including StructuralPartialEq
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create a module with multiple trait implementations
        let module = ParsedModule {
            name: "test".to_string(),
            visibility: Visibility::Public,
            docs: None,
            items: vec![
                ParsedItem::TraitImpl(ParsedTraitImpl {
                    trait_path: "Copy".to_string(),
                    for_type: RustType::Path { 
                        path: "Point".to_string(), 
                        generics: vec![RustType::Generic("T".to_string())] 
                    },
                    items: vec![],
                    docs: None,
                }),
                ParsedItem::TraitImpl(ParsedTraitImpl {
                    trait_path: "StructuralPartialEq".to_string(),
                    for_type: RustType::Path { 
                        path: "Point".to_string(), 
                        generics: vec![RustType::Generic("T".to_string())] 
                    },
                    items: vec![],
                    docs: None,
                }),
                ParsedItem::TraitImpl(ParsedTraitImpl {
                    trait_path: "PartialEq".to_string(),
                    for_type: RustType::Path { 
                        path: "Point".to_string(), 
                        generics: vec![RustType::Generic("T".to_string())] 
                    },
                    items: vec![
                        ParsedTraitImplItem::Method(
                            ParsedFunction {
                                signature: FunctionSignature {
                                    name: "eq".to_string(),
                                    visibility: Visibility::Public,
                                    generics: Generics {
                                        params: vec![],
                                        where_clauses: vec![],
                                    },
                                    inputs: vec![
                                        ("self".to_string(), RustType::Reference { 
                                            lifetime: None, 
                                            mutable: false, 
                                            inner: Box::new(RustType::Generic("Self".to_string())) 
                                        }),
                                        ("other".to_string(), RustType::Reference {
                                            lifetime: None,
                                            mutable: false,
                                            inner: Box::new(RustType::Path {
                                                path: "Point".to_string(),
                                                generics: vec![RustType::Generic("T".to_string())]
                                            })
                                        })
                                    ],
                                    output: RustType::Primitive("bool".to_string())
                                },
                                docs: None,
                                deprecation: None,
                            }
                        )
                    ],
                    docs: None,
                }),
            ],
        };
        
        // Render all items
        for item in &module.items {
            let context = RenderContext::new().with_depth(1);
            output.push_str(&item.render(&context));
        }

        // All trait implementations should be rendered
        assert!(output.contains("impl Copy for Point<T>"));
        assert!(output.contains("impl StructuralPartialEq for Point<T>"));
        assert!(output.contains("impl PartialEq for Point<T>"));
        
        // Check the ordering to ensure StructuralPartialEq comes before PartialEq
        let copy_pos = output.find("impl Copy for Point<T>").unwrap();
        let structural_pos = output.find("impl StructuralPartialEq for Point<T>").unwrap();
        let partial_eq_pos = output.find("impl PartialEq for Point<T>").unwrap();
        
        assert!(copy_pos < structural_pos);
        assert!(structural_pos < partial_eq_pos);
    }

    // Test removed - render_all_trait_impls_no_extra no longer exists in ParsedRenderer

    // Test removed - render_reexports method no longer exists in ParsedRenderer
    
    // Test removed - render_reexports method no longer exists in ParsedRenderer
    
    #[test]
    fn test_deprecation_rendering() {
        // Create a test with the new ParsedRenderer
        let func = ParsedFunction {
            signature: FunctionSignature {
                name: "set_timeout".to_string(),
                visibility: Visibility::Public,
                generics: Generics {
                    params: vec![],
                    where_clauses: vec![],
                },
                inputs: vec![
                    ("self".to_string(), RustType::Reference { 
                        lifetime: None, 
                        mutable: true, 
                        inner: Box::new(RustType::Generic("Self".to_string())) 
                    }),
                    ("seconds".to_string(), RustType::Primitive("u32".to_string()))
                ],
                output: RustType::Unit,
            },
            docs: Some("Old method for setting timeout in seconds".to_string()),
            deprecation: Some(Deprecation {
                since: Some("1.1.0".to_string()),
                note: None,
            }),
        };

        let mut output = String::new();
        let renderer = create_parsed_renderer();
        
        let context = RenderContext::new().with_depth(1);
        output.push_str(&func.render(&context));
        
        // Check that deprecation notice is rendered correctly with proper indentation
        assert!(output.contains("  DEPRECATED since 1.1.0"));
        assert!(output.contains("pub fn set_timeout"));
    }
    
    #[test]
    fn test_trait_with_deprecated_methods() {
        // Test rendering a trait with deprecated methods
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create a trait with a deprecated method
        let trait_item = ParsedTraitItem::Method(
            ParsedFunction {
                signature: FunctionSignature {
                    name: "handle_error".to_string(),
                    visibility: Visibility::Public,
                    generics: Generics {
                        params: vec![],
                        where_clauses: vec![],
                    },
                    inputs: vec![
                        ("self".to_string(), RustType::Reference { 
                            lifetime: None, 
                            mutable: false, 
                            inner: Box::new(RustType::Generic("Self".to_string())) 
                        }),
                        ("error".to_string(), RustType::Reference {
                            lifetime: None,
                            mutable: false,
                            inner: Box::new(RustType::Primitive("str".to_string()))
                        })
                    ],
                    output: RustType::Unit,
                },
                docs: Some("Old way of handling errors".to_string()),
                deprecation: Some(Deprecation {
                    since: Some("1.2.5".to_string()),
                    note: None,
                }),
            }
        );
        
        // Call the renderer function
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_item.render(&context));
        
        // Check for proper deprecation notice placement
        assert!(output.contains("DEPRECATED since 1.2.5"));
        assert!(output.contains("fn handle_error("));
        
        // The deprecation notice should come before the method signature
        let deprecation_pos = output.find("DEPRECATED since 1.2.5").unwrap();
        let handle_error_pos = output.find("fn handle_error(").unwrap();
        assert!(deprecation_pos < handle_error_pos);
    }
    
    #[test]
    fn test_trait_impl_with_deprecated_methods() {
        // Test rendering a trait implementation with deprecated methods
        let renderer = create_parsed_renderer();
        let mut output = String::new();
        
        // Create a trait implementation with multiple methods, including deprecated ones
        let trait_impl = ParsedTraitImpl {
            trait_path: "Handler".to_string(),
            for_type: RustType::Path { 
                path: "DefaultHandler".to_string(), 
                generics: vec![] 
            },
            items: vec![
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "process".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                })
                            ],
                            output: RustType::Path {
                                path: "Result".to_string(),
                                generics: vec![
                                    RustType::Unit,
                                    RustType::Primitive("String".to_string())
                                ]
                            }
                        },
                        docs: None,
                        deprecation: None,
                    }
                ),
                ParsedTraitImplItem::Method(
                    ParsedFunction {
                        signature: FunctionSignature {
                            name: "handle_error".to_string(),
                            visibility: Visibility::Public,
                            generics: Generics {
                                params: vec![],
                                where_clauses: vec![],
                            },
                            inputs: vec![
                                ("self".to_string(), RustType::Reference { 
                                    lifetime: None, 
                                    mutable: false, 
                                    inner: Box::new(RustType::Generic("Self".to_string())) 
                                }),
                                ("_error".to_string(), RustType::Reference {
                                    lifetime: None,
                                    mutable: false,
                                    inner: Box::new(RustType::Primitive("str".to_string()))
                                })
                            ],
                            output: RustType::Unit
                        },
                        docs: None,
                        deprecation: Some(Deprecation {
                            since: Some("1.2.5".to_string()),
                            note: None,
                        }),
                    }
                )
            ],
            docs: None,
        };
        
        // Call the renderer function using the new trait-based approach
        let context = RenderContext::new().with_depth(1);
        output.push_str(&trait_impl.render(&context));
        
        // Check that both methods are rendered
        assert!(output.contains("fn process("));
        assert!(output.contains("fn handle_error("));
        
        // Check that deprecation notice is shown and correctly placed
        assert!(output.contains("DEPRECATED since 1.2.5"));
        
        // The deprecation notice should come before the method signature
        let deprecation_pos = output.find("DEPRECATED since 1.2.5").unwrap();
        let handle_error_pos = output.find("fn handle_error(").unwrap();
        assert!(deprecation_pos < handle_error_pos);
        
        // The methods should have consistent indentation
        let lines: Vec<&str> = output.lines().collect();
        let process_line = lines.iter().find(|line| line.contains("fn process")).unwrap();
        let handle_error_line = lines.iter().find(|line| line.contains("fn handle_error")).unwrap();
        
        assert_eq!(process_line.chars().take(4).filter(|c| *c == ' ').count(), 4);
        assert_eq!(handle_error_line.chars().take(4).filter(|c| *c == ' ').count(), 4);
    }
}