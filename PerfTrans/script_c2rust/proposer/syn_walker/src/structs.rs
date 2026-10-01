//! struct walker — emits one `StructItem` per `struct` definition.
//!
//! Captures (per DESIGN.md §D.4 / EvidencePack v1.2 spec):
//!   - name, file, line
//!   - fields: list of (name, ty_str, line)
//!   - repr_attribute: "C" | "transparent" | "packed" | None
//!   - ffi_boundary  : true if `#[repr(C)]` OR has any `extern "C"` reference
//!                     (caller responsibility — proxy via repr(C) here)
//!   - is_pub        : visibility
//!   - derives       : list of derive macros (e.g. ["Clone", "Copy"])
//!
//! Deferred to v2 (need cross-file analysis):
//!   - usage_freq    (which hot fns reference this struct)
//!   - access_pattern (read-mostly / write-mostly / mixed)

use std::path::{Path, PathBuf};

use quote::ToTokens;
use serde::Serialize;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Attribute, Field, Fields, ItemStruct, Meta};

#[derive(Serialize, Debug)]
pub struct FieldItem {
    pub name: String,
    pub ty:   String,
    pub line: usize,
}

#[derive(Serialize, Debug)]
pub struct StructItem {
    pub name:           String,
    pub file:           String,
    pub line:           usize,
    pub fields:         Vec<FieldItem>,
    pub repr_attribute: Option<String>,
    pub ffi_boundary:   bool,
    pub is_pub:         bool,
    pub derives:        Vec<String>,
}

pub struct Walker<'a> {
    file:  PathBuf,
    text:  &'a str,
    items: Vec<StructItem>,
}

impl<'a> Walker<'a> {
    pub fn new(file: &Path, text: &'a str) -> Self {
        Walker { file: file.to_path_buf(), text, items: Vec::new() }
    }

    pub fn into_items(self) -> Vec<StructItem> {
        self.items
    }

    fn line_of_span(&self, span: proc_macro2::Span) -> usize {
        // syn 2 spans: only useful in proc-macro contexts; we work around
        // by calling .start().line which works for parsed-from-text spans.
        span.start().line
    }

    fn ty_to_string(ty: &syn::Type) -> String {
        ty.to_token_stream().to_string()
    }
}

impl<'a, 'ast> Visit<'ast> for Walker<'a> {
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        let mut item = StructItem {
            name:           node.ident.to_string(),
            file:           self.file.display().to_string(),
            line:           self.line_of_span(node.ident.span()),
            fields:         Vec::new(),
            repr_attribute: None,
            ffi_boundary:   false,
            is_pub:         matches!(node.vis, syn::Visibility::Public(_)),
            derives:        Vec::new(),
        };

        // Walk attributes.
        for attr in &node.attrs {
            decode_repr(attr, &mut item.repr_attribute);
            decode_derive(attr, &mut item.derives);
        }
        // Cheap proxy for ffi_boundary: repr(C) or repr(transparent).
        item.ffi_boundary = matches!(item.repr_attribute.as_deref(), Some("C") | Some("transparent"));

        // Fields.
        item.fields = fields_to_items(&node.fields);

        self.items.push(item);
        // No need to recurse into nested types — top-level structs only
        // for v1; nested struct definitions inside fn bodies are rare in
        // c2rust output.
    }
}

fn fields_to_items(fields: &Fields) -> Vec<FieldItem> {
    let mut out = Vec::new();
    match fields {
        Fields::Named(named) => {
            for f in &named.named {
                if let Some(name) = field_name(f) {
                    out.push(FieldItem {
                        name,
                        ty:   Walker::ty_to_string(&f.ty),
                        line: f.ident.as_ref().map(|i| i.span().start().line).unwrap_or(0),
                    });
                }
            }
        }
        Fields::Unnamed(unnamed) => {
            for (i, f) in unnamed.unnamed.iter().enumerate() {
                out.push(FieldItem {
                    name: format!("_{}", i),
                    ty:   Walker::ty_to_string(&f.ty),
                    line: f.ty.span().start().line,
                });
            }
        }
        Fields::Unit => {}
    }
    out
}

fn field_name(f: &Field) -> Option<String> {
    f.ident.as_ref().map(|i| i.to_string())
}

fn decode_repr(attr: &Attribute, slot: &mut Option<String>) {
    if !attr.path().is_ident("repr") {
        return;
    }
    // repr(C) / repr(transparent) / repr(packed) / repr(C, packed)
    if let Meta::List(ml) = &attr.meta {
        let s = ml.tokens.to_string();
        for tok in s.split(',').map(|t| t.trim()) {
            if matches!(tok, "C" | "transparent" | "packed" | "Rust") {
                if slot.is_none() {
                    *slot = Some(tok.to_string());
                } else if let Some(prev) = slot.as_mut() {
                    if !prev.contains(tok) {
                        prev.push(',');
                        prev.push_str(tok);
                    }
                }
            }
        }
    }
}

fn decode_derive(attr: &Attribute, slot: &mut Vec<String>) {
    if !attr.path().is_ident("derive") {
        return;
    }
    if let Meta::List(ml) = &attr.meta {
        let s = ml.tokens.to_string();
        for tok in s.split(',').map(|t| t.trim()) {
            if !tok.is_empty() {
                slot.push(tok.to_string());
            }
        }
    }
}
