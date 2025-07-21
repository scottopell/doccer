use anyhow::Result;
use rustdoc_types::{Crate, Id, Item, ItemEnum, Module, Visibility};
use crate::parser::types::*;

// Parser for converting raw JSON items to typed structures
pub struct ItemParser<'a> {
    crate_data: &'a Crate,
}

impl<'a> ItemParser<'a> {
    pub fn new(crate_data: &'a Crate) -> Self {
        Self { crate_data }
    }

    // Helper method to check if a trait implementation should be filtered out
    fn should_filter_trait_impl(&self, impl_item: &Item, impl_data: &serde_json::Value) -> bool {
        // Check for synthetic implementation marker to identify derived implementations
        if let Some(is_synthetic) = impl_data.get("is_synthetic").and_then(|v| v.as_bool()) {
            if is_synthetic {
                return true;
            }
        }

        // Check for derive attribute in item attributes
        if impl_item.attrs.iter().any(|attr| attr.contains("#[derive")) {
            return true;
        }

        // Filter out blanket implementations (they are usually generic auto-implementations)
        if impl_data.get("blanket_impl").is_some() && !impl_data.get("blanket_impl").unwrap().is_null() {
            return true;
        }

        // Filter out common auto-derived traits that typically shouldn't be shown
        if let Some(trait_ref) = impl_data.get("trait") {
            if let Some(trait_path) = trait_ref.get("path").and_then(|p| p.as_str()) {
                let filtered_traits = [
                    "Send",
                    "Sync",
                    "Freeze",
                    "Unpin",
                    "UnwindSafe",
                    "RefUnwindSafe",
                    "Borrow",
                    "BorrowMut",
                    "Into",
                    "From",
                    "TryInto",
                    "TryFrom",
                    "Any",
                    "CloneToUninit",
                    "ToOwned",
                    "StructuralPartialEq",
                    "ToString",
                    "IntoFuture",
                ];

                // Extract just the trait name (last part of the path)
                let trait_name = trait_path.split("::").last().unwrap_or(trait_path);

                if filtered_traits.contains(&trait_name) {
                    return true;
                }
            }
        }

        false
    }

    pub fn parse_crate(&self) -> Result<ParsedModule> {
        let root_id = &self.crate_data.root;
        if let Some(root_item) = self.crate_data.index.get(root_id) {
            let mut parsed_module = ParsedModule {
                name: root_item.name.as_deref().unwrap_or("unknown").to_string(),
                visibility: Visibility::Public,
                items: Vec::new(),
                docs: root_item.docs.clone(),
            };

            if let ItemEnum::Module(module) = &root_item.inner {
                for item_id in &module.items {
                    if let Some(parsed_item) = self.parse_item(item_id)? {
                        parsed_module.items.push(parsed_item);
                    }
                }
            }

            Ok(parsed_module)
        } else {
            Err(anyhow::anyhow!("Root module not found"))
        }
    }

    fn parse_item(&self, item_id: &Id) -> Result<Option<ParsedItem>> {
        let item = match self.crate_data.index.get(item_id) {
            Some(item) => item,
            None => return Ok(None),
        };

        match &item.inner {
            ItemEnum::Function(func_data) => {
                let json_value = serde_json::to_value(func_data)?;
                if let Some(parsed) = self.parse_function(item, &json_value)? {
                    return Ok(Some(ParsedItem::Function(parsed)));
                }
            }
            ItemEnum::Struct(struct_data) => {
                let json_value = serde_json::to_value(struct_data)?;
                if let Some(parsed) = self.parse_struct(item, &json_value)? {
                    return Ok(Some(ParsedItem::Struct(parsed)));
                }
            }
            ItemEnum::Enum(enum_data) => {
                let json_value = serde_json::to_value(enum_data)?;
                if let Some(parsed) = self.parse_enum(item, &json_value)? {
                    return Ok(Some(ParsedItem::Enum(parsed)));
                }
            }
            ItemEnum::Trait(trait_data) => {
                let json_value = serde_json::to_value(trait_data)?;
                if let Some(parsed) = self.parse_trait(item, &json_value)? {
                    return Ok(Some(ParsedItem::Trait(parsed)));
                }
            }
            ItemEnum::Constant { type_, const_ } => {
                let const_obj = serde_json::json!({
                    "type": type_,
                    "const": const_
                });
                if let Some(parsed) = self.parse_constant(item, &const_obj)? {
                    return Ok(Some(ParsedItem::Constant(parsed)));
                }
            }
            ItemEnum::Module(module_data) => {
                let json_value = serde_json::to_value(module_data)?;
                if let Some(parsed) = self.parse_module(item, &json_value)? {
                    return Ok(Some(ParsedItem::Module(parsed)));
                }
            }
            ItemEnum::Macro(macro_data) => {
                let json_value = serde_json::to_value(macro_data)?;
                if let Some(parsed) = self.parse_macro(item, &json_value)? {
                    return Ok(Some(ParsedItem::Macro(parsed)));
                }
            }
            ItemEnum::Impl(impl_data) => {
                let json_value = serde_json::to_value(impl_data)?;
                if let Some(parsed) = self.parse_trait_impl(item, &json_value)? {
                    return Ok(Some(ParsedItem::TraitImpl(parsed)));
                }
            }
            // ItemEnum::Import(import_data) => {
            //     if let Some(parsed) = self.parse_use(item, import_data)? {
            //         return Ok(Some(ParsedItem::ReExport(parsed)));
            //     }
            // }
            _ => {} // Skip other kinds for now
        }

        Ok(None)
    }


    fn parse_type(&self, type_val: &serde_json::Value) -> RustType {
        // Handle null values as unit type
        if type_val.is_null() {
            return RustType::Unit;
        }

        if let Some(primitive) = type_val.get("primitive") {
            if let Some(prim_str) = primitive.as_str() {
                return RustType::Primitive(prim_str.to_string());
            }
        }

        if let Some(generic) = type_val.get("generic") {
            if let Some(gen_str) = generic.as_str() {
                return RustType::Generic(gen_str.to_string());
            }
        }

        if let Some(resolved_path) = type_val.get("resolved_path") {
            let path = resolved_path
                .get("path")
                .and_then(|p| p.as_str())
                .unwrap_or("unknown")
                .to_string();

            // Normalize $crate:: paths to their standard library equivalents
            let normalized_path = if path.starts_with("$crate::") {
                match path.as_str() {
                    "$crate::fmt::Formatter" => "std::fmt::Formatter".to_string(),
                    "$crate::fmt::Result" => "std::fmt::Result".to_string(),
                    "$crate::clone::Clone" => "Clone".to_string(),
                    "$crate::cmp::PartialEq" => "PartialEq".to_string(),
                    other => {
                        // For other $crate:: paths, replace with std::
                        other.replace("$crate::", "std::")
                    }
                }
            } else {
                path
            };

            let mut generics = Vec::new();
            if let Some(args) = resolved_path.get("args") {
                if let Some(angle_bracketed) = args.get("angle_bracketed") {
                    if let Some(args_array) = angle_bracketed.get("args").and_then(|a| a.as_array())
                    {
                        for arg in args_array {
                            if let Some(type_arg) = arg.get("type") {
                                generics.push(self.parse_type(type_arg));
                            }
                        }
                    }
                }
            }

            return RustType::Path { path: normalized_path, generics };
        }

        if let Some(borrowed_ref) = type_val.get("borrowed_ref") {
            let lifetime = borrowed_ref
                .get("lifetime")
                .and_then(|l| l.as_str())
                .map(|s| s.to_string());
            let mutable = borrowed_ref
                .get("is_mutable")
                .and_then(|m| m.as_bool())
                .unwrap_or(false);
            let inner = borrowed_ref
                .get("type")
                .map(|t| Box::new(self.parse_type(t)))
                .unwrap_or_else(|| Box::new(RustType::Unknown));

            return RustType::Reference {
                lifetime,
                mutable,
                inner,
            };
        }

        if let Some(tuple) = type_val.get("tuple") {
            if let Some(tuple_array) = tuple.as_array() {
                if tuple_array.is_empty() {
                    return RustType::Unit;
                } else {
                    let elements = tuple_array
                        .iter()
                        .map(|elem| self.parse_type(elem))
                        .collect();
                    return RustType::Tuple(elements);
                }
            }
        }

        if let Some(slice) = type_val.get("slice") {
            return RustType::Slice(Box::new(self.parse_type(slice)));
        }

        if let Some(array) = type_val.get("array") {
            if let Some(type_info) = array.get("type") {
                let inner = Box::new(self.parse_type(type_info));
                let size = array
                    .get("len")
                    .map(|l| l.to_string())
                    .unwrap_or_else(|| "N".to_string());
                return RustType::Array { inner, size };
            }
        }

        if let Some(raw_pointer) = type_val.get("raw_pointer") {
            let mutable = raw_pointer
                .get("is_mutable")
                .and_then(|m| m.as_bool())
                .unwrap_or(false);
            let inner = raw_pointer
                .get("type")
                .map(|t| Box::new(self.parse_type(t)))
                .unwrap_or_else(|| Box::new(RustType::Unknown));
            return RustType::RawPointer { mutable, inner };
        }

        if let Some(qualified_path) = type_val.get("qualified_path") {
            if let Some(name) = qualified_path.get("name").and_then(|n| n.as_str()) {
                return RustType::QualifiedPath {
                    base: "Self".to_string(),
                    name: name.to_string(),
                };
            }
        }

        if let Some(dyn_trait) = type_val.get("dyn_trait") {
            let lifetime = dyn_trait
                .get("lifetime")
                .and_then(|l| l.as_str())
                .map(|s| s.to_string());
            
            let mut traits = Vec::new();
            if let Some(traits_array) = dyn_trait.get("traits").and_then(|t| t.as_array()) {
                for trait_item in traits_array {
                    if let Some(trait_info) = trait_item.get("trait") {
                        if let Some(path) = trait_info.get("path").and_then(|p| p.as_str()) {
                            let mut trait_str = path.to_string();
                            
                            // Handle associated type constraints
                            if let Some(args) = trait_info.get("args") {
                                if let Some(angle_bracketed) = args.get("angle_bracketed") {
                                    if let Some(constraints) = angle_bracketed.get("constraints").and_then(|c| c.as_array()) {
                                        if !constraints.is_empty() {
                                            let mut constraint_strs = Vec::new();
                                            for constraint in constraints {
                                                if let Some(name) = constraint.get("name").and_then(|n| n.as_str()) {
                                                    if let Some(binding) = constraint.get("binding") {
                                                        if let Some(equality) = binding.get("equality") {
                                                            if let Some(ty) = equality.get("type") {
                                                                let constraint_type = self.parse_type(ty);
                                                                constraint_strs.push(format!("{} = {}", name, constraint_type));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if !constraint_strs.is_empty() {
                                                trait_str.push_str(&format!("<{}>", constraint_strs.join(", ")));
                                            }
                                        }
                                    }
                                }
                            }
                            
                            traits.push(trait_str);
                        }
                    }
                }
            }
            
            return RustType::DynTrait { traits, lifetime };
        }

        RustType::Unknown
    }

    fn parse_generics(&self, generics: &serde_json::Value) -> Generics {
        let mut params = Vec::new();
        let mut where_clauses = Vec::new();

        if let Some(params_array) = generics.get("params").and_then(|p| p.as_array()) {
            for param in params_array {
                if let Some(name) = param.get("name").and_then(|n| n.as_str()) {
                    if let Some(kind) = param.get("kind") {
                        if let Some(type_kind) = kind.get("type") {
                            let mut bounds = Vec::new();
                            
                            // Parse bounds from the type kind
                            if let Some(bounds_array) = type_kind.get("bounds").and_then(|b| b.as_array()) {
                                for bound in bounds_array {
                                    if let Some(trait_bound) = bound.get("trait_bound") {
                                        if let Some(trait_ref) = trait_bound.get("trait") {
                                            if let Some(path) = trait_ref.get("path").and_then(|p| p.as_str()) {
                                                bounds.push(path.to_string());
                                            }
                                        }
                                    }
                                }
                            }
                            
                            params.push(GenericParam {
                                name: name.to_string(),
                                kind: GenericParamKind::Type { bounds },
                            });
                        } else if kind.get("lifetime").is_some() {
                            params.push(GenericParam {
                                name: name.to_string(),
                                kind: GenericParamKind::Lifetime,
                            });
                        }
                    }
                }
            }
        }

        // Parse where clauses
        if let Some(where_predicates) = generics.get("where_predicates").and_then(|p| p.as_array()) {
            for predicate in where_predicates {
                if let Some(bound_predicate) = predicate.get("bound_predicate") {
                    if let Some(type_info) = bound_predicate.get("type") {
                        // Get the type being constrained
                        let type_name = if let Some(generic_name) = type_info.get("generic").and_then(|g| g.as_str()) {
                            generic_name.to_string()
                        } else {
                            // For more complex types, we'd need to parse them fully
                            "Self".to_string()
                        };
                        
                        // Parse the bounds
                        let mut bounds = Vec::new();
                        if let Some(bounds_array) = bound_predicate.get("bounds").and_then(|b| b.as_array()) {
                            for bound in bounds_array {
                                if let Some(trait_bound) = bound.get("trait_bound") {
                                    if let Some(trait_ref) = trait_bound.get("trait") {
                                        if let Some(path) = trait_ref.get("path").and_then(|p| p.as_str()) {
                                            bounds.push(path.to_string());
                                        }
                                    }
                                }
                            }
                        }
                        
                        if !bounds.is_empty() {
                            let where_clause = format!("{}: {}", type_name, bounds.join(" + "));
                            where_clauses.push(where_clause);
                        }
                    }
                }
            }
        }

        Generics {
            params,
            where_clauses,
        }
    }

    fn parse_function(
        &self,
        item: &Item,
        func_data: &serde_json::Value,
    ) -> Result<Option<ParsedFunction>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Function missing name"))?
            .clone();
        let visibility = item.visibility.clone();
        let generics = func_data
            .get("generics")
            .map(|g| self.parse_generics(g))
            .unwrap_or_else(|| Generics {
                params: Vec::new(),
                where_clauses: Vec::new(),
            });

        let mut inputs = Vec::new();
        let mut output = RustType::Unit;

        if let Some(sig) = func_data.get("sig") {
            if let Some(inputs_val) = sig.get("inputs") {
                if let Some(inputs_array) = inputs_val.as_array() {
                    for input in inputs_array {
                        if let Some(input_array) = input.as_array() {
                            if input_array.len() == 2 {
                                if let Some(param_name) = input_array[0].as_str() {
                                    let param_type = self.parse_type(&input_array[1]);
                                    inputs.push((param_name.to_string(), param_type));
                                }
                            }
                        }
                    }
                }
            }

            if let Some(output_val) = sig.get("output") {
                output = self.parse_type(output_val);
            }
        }

        let mut is_async = false;
        if let Some(header) = func_data.get("header") {
            if let Some(async_flag) = header.get("is_async") {
                is_async = async_flag.as_bool().unwrap_or(false);
            }
        }

        let signature = FunctionSignature {
            name,
            visibility,
            generics,
            inputs,
            output,
            is_async,
        };

        Ok(Some(ParsedFunction {
            signature,
            docs: item.docs.clone(),
            deprecation: item.deprecation.clone(),
        }))
    }

    fn parse_struct(
        &self,
        item: &Item,
        struct_data: &serde_json::Value,
    ) -> Result<Option<ParsedStruct>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Struct missing name"))?
            .clone();
        let visibility = item.visibility.clone();
        let generics = struct_data
            .get("generics")
            .map(|g| self.parse_generics(g))
            .unwrap_or_else(|| Generics {
                params: Vec::new(),
                where_clauses: Vec::new(),
            });

        let mut methods = Vec::new();
        let mut trait_impls = Vec::new();
        let mut fields = Vec::new();

        // Parse struct fields
        if let Some(kind) = struct_data.get("kind") {
            if let Some(plain) = kind.get("plain") {
                if let Some(field_ids) = plain.get("fields") {
                    if let Some(field_array) = field_ids.as_array() {
                        for field_id in field_array {
                            if let Some(field_id_num) = field_id.as_u64() {
                                let field_id = Id(field_id_num as u32);
                                if let Some(field_item) = self.crate_data.index.get(&field_id) {
                                    if let ItemEnum::StructField(field_type) = &field_item.inner {
                                        let field_name = field_item.name.clone().unwrap_or_else(|| "unnamed".to_string());
                                        let parsed_field = ParsedField {
                                            name: field_name,
                                            visibility: field_item.visibility.clone(),
                                            field_type: self.parse_type(&serde_json::to_value(field_type).unwrap_or_default()),
                                            docs: field_item.docs.clone(),
                                            deprecation: field_item.deprecation.clone(),
                                        };
                                        fields.push(parsed_field);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Parse methods from impl blocks
        if let Some(impls) = struct_data.get("impls") {
            if let Some(impl_ids) = impls.as_array() {
                for impl_id in impl_ids {
                    if let Some(impl_id_num) = impl_id.as_u64() {
                        let impl_id = Id(impl_id_num as u32);
                        if let Some(impl_item) = self.crate_data.index.get(&impl_id) {
                            if let ItemEnum::Impl(impl_inner) = &impl_item.inner {
                                let is_trait_impl = impl_inner.trait_.is_some();

                                if !is_trait_impl {
                                    // Inherent impl - collect methods
                                    for method_id in &impl_inner.items {
                                        if let Some(method_item) =
                                            self.crate_data.index.get(method_id)
                                        {
                                            if let ItemEnum::Function(func_data) =
                                                &method_item.inner
                                            {
                                                // Convert Function enum back to JSON for now
                                                let func_json = serde_json::to_value(func_data)?;
                                                if let Some(parsed_method) =
                                                    self.parse_function(method_item, &func_json)?
                                                {
                                                    methods.push(parsed_method);
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    // Trait impl - collect it only if it should not be filtered
                                    let impl_json = serde_json::to_value(impl_inner)?;
                                    if !self.should_filter_trait_impl(impl_item, &impl_json) {
                                        if let Some(parsed_impl) =
                                            self.parse_trait_impl(impl_item, &impl_json)?
                                        {
                                            if let ParsedItem::TraitImpl(trait_impl) =
                                                ParsedItem::TraitImpl(parsed_impl)
                                            {
                                                trait_impls.push(trait_impl);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(Some(ParsedStruct {
            name,
            visibility,
            generics,
            docs: item.docs.clone(),
            deprecation: item.deprecation.clone(),
            fields,
            methods,
            trait_impls,
        }))
    }

    fn parse_enum(&self, item: &Item, enum_data: &serde_json::Value) -> Result<Option<ParsedEnum>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Enum missing name"))?
            .clone();
        let visibility = item.visibility.clone();
        let generics = enum_data
            .get("generics")
            .map(|g| self.parse_generics(g))
            .unwrap_or_else(|| Generics {
                params: Vec::new(),
                where_clauses: Vec::new(),
            });

        let mut variants = Vec::new();

        if let Some(variant_ids) = enum_data.get("variants").and_then(|v| v.as_array()) {
            for variant_id in variant_ids {
                if let Some(variant_id_num) = variant_id.as_u64() {
                    let variant_id = Id(variant_id_num as u32);
                    if let Some(variant_item) = self.crate_data.index.get(&variant_id) {
                        if let Some(parsed_variant) = self.parse_variant(variant_item)? {
                            variants.push(parsed_variant);
                        }
                    }
                }
            }
        }

        Ok(Some(ParsedEnum {
            name,
            visibility,
            generics,
            variants,
            docs: item.docs.clone(),
            deprecation: item.deprecation.clone(),
        }))
    }

    fn parse_variant(&self, item: &Item) -> Result<Option<ParsedVariant>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Variant missing name"))?
            .clone();

        let kind = if let ItemEnum::Variant(variant_data) = &item.inner {
            match &variant_data.kind {
                rustdoc_types::VariantKind::Plain => VariantKind::Unit,
                rustdoc_types::VariantKind::Tuple(tuple_fields) => {
                    let mut field_types = Vec::new();
                    for field_id_opt in tuple_fields {
                        if let Some(field_id) = field_id_opt {
                            if let Some(field_item) = self.crate_data.index.get(field_id) {
                                if let ItemEnum::StructField(field_data) = &field_item.inner {
                                    let field_json = serde_json::to_value(field_data)?;
                                    let field_type = self.parse_type(&field_json);
                                    field_types.push(field_type);
                                }
                            }
                        }
                    }
                    VariantKind::Tuple(field_types)
                }
                rustdoc_types::VariantKind::Struct { fields, .. } => {
                    let mut named_fields = Vec::new();
                    for field_id in fields {
                        if let Some(field_item) = self.crate_data.index.get(field_id) {
                            if let ItemEnum::StructField(field_data) = &field_item.inner {
                                let field_name = field_item
                                    .name
                                    .as_ref()
                                    .unwrap_or(&"unknown".to_string())
                                    .clone();
                                let field_json = serde_json::to_value(field_data)?;
                                let field_type = self.parse_type(&field_json);
                                named_fields.push((field_name, field_type));
                            }
                        }
                    }
                    VariantKind::Struct(named_fields)
                }
            }
        } else {
            VariantKind::Unit
        };

        Ok(Some(ParsedVariant {
            name,
            kind,
            docs: item.docs.clone(),
        }))
    }

    fn parse_trait(
        &self,
        item: &Item,
        trait_data: &serde_json::Value,
    ) -> Result<Option<ParsedTrait>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Trait missing name"))?
            .clone();
        let visibility = item.visibility.clone();
        let generics = trait_data
            .get("generics")
            .map(|g| self.parse_generics(g))
            .unwrap_or_else(|| Generics {
                params: Vec::new(),
                where_clauses: Vec::new(),
            });

        let mut items = Vec::new();

        if let Some(trait_items) = trait_data.get("items").and_then(|i| i.as_array()) {
            for item_id in trait_items {
                if let Some(item_id_num) = item_id.as_u64() {
                    let item_id = Id(item_id_num as u32);
                    if let Some(trait_item) = self.crate_data.index.get(&item_id) {
                        if let Some(parsed_trait_item) = self.parse_trait_item(trait_item)? {
                            items.push(parsed_trait_item);
                        }
                    }
                }
            }
        }

        Ok(Some(ParsedTrait {
            name,
            visibility,
            generics,
            items,
            docs: item.docs.clone(),
            deprecation: item.deprecation.clone(),
        }))
    }

    fn parse_trait_item(&self, item: &Item) -> Result<Option<ParsedTraitItem>> {
        match &item.inner {
            ItemEnum::AssocType { .. } => {
                let name = item.name.as_ref().unwrap_or(&"unknown".to_string()).clone();
                let bounds = Vec::new(); // TODO: Parse bounds
                return Ok(Some(ParsedTraitItem::AssocType {
                    name,
                    bounds,
                    docs: item.docs.clone(),
                }));
            }
            ItemEnum::Function(func_data) => {
                let func_json = serde_json::to_value(func_data)?;
                if let Some(parsed_func) = self.parse_function(item, &func_json)? {
                    return Ok(Some(ParsedTraitItem::Method(parsed_func)));
                }
            }
            ItemEnum::AssocConst { type_, .. } => {
                let name = item.name.as_ref().unwrap_or(&"unknown".to_string()).clone();
                let const_json = serde_json::to_value(type_)?;
                let ty = self.parse_type(&const_json);
                return Ok(Some(ParsedTraitItem::AssocConst {
                    name,
                    ty,
                    docs: item.docs.clone(),
                }));
            }
            _ => {}
        }
        Ok(None)
    }

    fn parse_constant(
        &self,
        item: &Item,
        const_data: &serde_json::Value,
    ) -> Result<Option<ParsedConstant>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Constant missing name"))?
            .clone();
        let visibility = item.visibility.clone();
        let ty = const_data
            .get("type")
            .map(|t| self.parse_type(t))
            .unwrap_or(RustType::Unknown);

        Ok(Some(ParsedConstant {
            name,
            visibility,
            ty,
            docs: item.docs.clone(),
            deprecation: item.deprecation.clone(),
        }))
    }

    fn parse_module(
        &self,
        item: &Item,
        module_data: &serde_json::Value,
    ) -> Result<Option<ParsedModule>> {
        let name = item.name.as_ref().unwrap_or(&"unknown".to_string()).clone();
        let visibility = item.visibility.clone();

        let mut items = Vec::new();
        if let Ok(module) = serde_json::from_value::<Module>(module_data.clone()) {
            for item_id in &module.items {
                if let Some(parsed_item) = self.parse_item(item_id)? {
                    items.push(parsed_item);
                }
            }
        }

        Ok(Some(ParsedModule {
            name,
            visibility,
            items,
            docs: item.docs.clone(),
        }))
    }

    fn parse_macro(
        &self,
        item: &Item,
        macro_data: &serde_json::Value,
    ) -> Result<Option<ParsedMacro>> {
        let name = item
            .name
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Macro missing name"))?
            .clone();

        let signature = if let Some(macro_str) = macro_data.as_str() {
            if let Some(start) = macro_str.find('(') {
                if let Some(end) = macro_str.find(')') {
                    let params_part = &macro_str[start + 1..end];
                    format!("macro_rules! {}({})", name, params_part)
                } else {
                    format!("macro_rules! {}(...)", name)
                }
            } else {
                format!("macro_rules! {}", name)
            }
        } else {
            format!("macro_rules! {}", name)
        };

        Ok(Some(ParsedMacro {
            signature,
            docs: item.docs.clone(),
        }))
    }

    fn parse_trait_impl(
        &self,
        item: &Item,
        impl_data: &serde_json::Value,
    ) -> Result<Option<ParsedTraitImpl>> {
        if let Some(trait_ref) = impl_data.get("trait") {
            if !trait_ref.is_null() {
                let trait_path = trait_ref
                    .get("path")
                    .and_then(|p| p.as_str())
                    .unwrap_or("unknown")
                    .to_string();

                let for_type = impl_data
                    .get("for")
                    .map(|t| self.parse_type(t))
                    .unwrap_or(RustType::Unknown);

                let mut items = Vec::new();
                if let Some(impl_items) = impl_data.get("items").and_then(|i| i.as_array()) {
                    for item_id in impl_items {
                        if let Some(item_id_num) = item_id.as_u64() {
                            let item_id = Id(item_id_num as u32);
                            if let Some(impl_item) = self.crate_data.index.get(&item_id) {
                                if let Some(parsed_impl_item) =
                                    self.parse_trait_impl_item(impl_item)?
                                {
                                    items.push(parsed_impl_item);
                                }
                            }
                        }
                    }
                }

                return Ok(Some(ParsedTraitImpl {
                    trait_path,
                    for_type,
                    items,
                    docs: item.docs.clone(),
                }));
            }
        }
        Ok(None)
    }


    fn parse_trait_impl_item(&self, item: &Item) -> Result<Option<ParsedTraitImplItem>> {
        match &item.inner {
            ItemEnum::AssocType { type_, .. } => {
                let name = item.name.as_ref().unwrap_or(&"unknown".to_string()).clone();
                let type_json = if let Some(ty) = type_ {
                    serde_json::to_value(ty)?
                } else {
                    serde_json::Value::Null
                };
                let ty = self.parse_type(&type_json);
                return Ok(Some(ParsedTraitImplItem::AssocType { name, ty }));
            }
            ItemEnum::Function(func_data) => {
                let func_json = serde_json::to_value(func_data)?;
                if let Some(parsed_func) = self.parse_function(item, &func_json)? {
                    return Ok(Some(ParsedTraitImplItem::Method(parsed_func)));
                }
            }
            _ => {}
        }
        Ok(None)
    }
}