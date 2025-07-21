use rustdoc_types::{Deprecation, Visibility};


// Parsed data structures - representing items in a more structured way

#[derive(Debug, Clone)]
pub enum RustType {
    Primitive(String),
    Generic(String),
    Reference {
        lifetime: Option<String>,
        mutable: bool,
        inner: Box<RustType>,
    },
    Tuple(Vec<RustType>),
    Slice(Box<RustType>),
    Array {
        inner: Box<RustType>,
        size: String,
    },
    Path {
        path: String,
        generics: Vec<RustType>,
    },
    RawPointer {
        mutable: bool,
        inner: Box<RustType>,
    },
    QualifiedPath {
        base: String,
        name: String,
    },
    DynTrait {
        traits: Vec<String>,
        lifetime: Option<String>,
    },
    Unit,
    Unknown,
}

impl std::fmt::Display for RustType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RustType::Primitive(name) => write!(f, "{}", name),
            RustType::Generic(name) => write!(f, "{}", name),
            RustType::Reference {
                lifetime,
                mutable,
                inner,
            } => {
                let mut result = "&".to_string();
                if let Some(lifetime_str) = lifetime {
                    result.push_str(lifetime_str);
                    result.push(' ');
                }
                if *mutable {
                    result.push_str("mut ");
                }
                result.push_str(&inner.to_string());
                write!(f, "{}", result)
            }
            RustType::Tuple(elements) => {
                if elements.is_empty() {
                    write!(f, "()")
                } else {
                    let element_strs: Vec<String> =
                        elements.iter().map(|e| e.to_string()).collect();
                    write!(f, "({})", element_strs.join(", "))
                }
            }
            RustType::Slice(inner) => write!(f, "[{}]", inner),
            RustType::Array { inner, size } => write!(f, "[{}; {}]", inner, size),
            RustType::Path { path, generics } => {
                if generics.is_empty() {
                    write!(f, "{}", path)
                } else {
                    let generic_strs: Vec<String> =
                        generics.iter().map(|g| g.to_string()).collect();
                    write!(f, "{}<{}>", path, generic_strs.join(", "))
                }
            }
            RustType::RawPointer { mutable, inner } => {
                if *mutable {
                    write!(f, "*mut {}", inner)
                } else {
                    write!(f, "*const {}", inner)
                }
            }
            RustType::QualifiedPath { base, name } => write!(f, "{}::{}", base, name),
            RustType::DynTrait { traits, lifetime } => {
                let mut result = "dyn ".to_string();
                if let Some(lifetime_str) = lifetime {
                    result.push_str(lifetime_str);
                    result.push(' ');
                }
                result.push_str(&traits.join(" + "));
                write!(f, "{}", result)
            }
            RustType::Unit => write!(f, "()"),
            RustType::Unknown => write!(f, "..."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenericParam {
    pub name: String,
    pub kind: GenericParamKind,
}

#[derive(Debug, Clone)]
pub enum GenericParamKind {
    Type { bounds: Vec<String> },
    Lifetime,
}

#[derive(Debug, Clone)]
pub struct Generics {
    pub params: Vec<GenericParam>,
    pub where_clauses: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Generics,
    pub inputs: Vec<(String, RustType)>,
    pub output: RustType,
    pub is_async: bool,
}

#[derive(Debug, Clone)]
pub struct ParsedFunction {
    pub signature: FunctionSignature,
    pub docs: Option<String>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone)]
pub struct ParsedField {
    pub name: String,
    pub visibility: Visibility,
    pub field_type: RustType,
    #[allow(dead_code)]
    pub docs: Option<String>,
    #[allow(dead_code)]
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone)]
pub struct ParsedStruct {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Generics,
    pub docs: Option<String>,
    pub deprecation: Option<Deprecation>,
    pub fields: Vec<ParsedField>,
    pub methods: Vec<ParsedFunction>,
    pub trait_impls: Vec<ParsedTraitImpl>,
}

#[derive(Debug, Clone)]
pub struct ParsedEnum {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Generics,
    pub variants: Vec<ParsedVariant>,
    pub docs: Option<String>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone)]
pub struct ParsedVariant {
    pub name: String,
    pub kind: VariantKind,
    pub docs: Option<String>,
}

#[derive(Debug, Clone)]
pub enum VariantKind {
    Unit,
    Tuple(Vec<RustType>),
    Struct(Vec<(String, RustType)>),
}

#[derive(Debug, Clone)]
pub struct ParsedTrait {
    pub name: String,
    pub visibility: Visibility,
    pub generics: Generics,
    pub items: Vec<ParsedTraitItem>,
    pub docs: Option<String>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone)]
pub enum ParsedTraitItem {
    AssocType {
        name: String,
        bounds: Vec<String>,
        docs: Option<String>,
    },
    AssocConst {
        name: String,
        ty: RustType,
        docs: Option<String>,
    },
    Method(ParsedFunction),
}

#[derive(Debug, Clone)]
pub struct ParsedTraitImpl {
    pub trait_path: String,
    pub for_type: RustType,
    pub items: Vec<ParsedTraitImplItem>,
    pub docs: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ParsedTraitImplItem {
    AssocType { name: String, ty: RustType },
    Method(ParsedFunction),
}

#[derive(Debug, Clone)]
pub struct ParsedConstant {
    pub name: String,
    pub visibility: Visibility,
    pub ty: RustType,
    pub docs: Option<String>,
    pub deprecation: Option<Deprecation>,
}

#[derive(Debug, Clone)]
pub struct ParsedModule {
    pub name: String,
    pub visibility: Visibility,
    pub items: Vec<ParsedItem>,
    pub docs: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParsedMacro {
    pub signature: String,
    pub docs: Option<String>,
}


#[derive(Debug, Clone)]
pub enum ParsedItem {
    Function(ParsedFunction),
    Struct(ParsedStruct),
    Enum(ParsedEnum),
    Trait(ParsedTrait),
    Constant(ParsedConstant),
    Module(ParsedModule),
    Macro(ParsedMacro),
    TraitImpl(ParsedTraitImpl),
}