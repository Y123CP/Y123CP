//! alloc-site walker — emits one `AllocSite` per heap-alloc call site.
//!
//! Recognised forms:
//!   Vec::new()                 kind = "Vec::new"           size_arg = "0"
//!   Vec::with_capacity(N)      kind = "Vec::with_capacity" size_arg = literal/expr/"unknown"
//!   Vec::from(...)             kind = "Vec::from"          size_arg = "unknown"
//!   Box::new(...)              kind = "Box::new"           size_arg = "1"
//!   String::new()              kind = "String::new"        size_arg = "0"
//!   String::with_capacity(N)   kind = "String::with_capacity"
//!   vec![]                     kind = "vec!"               size_arg = literal/"unknown"
//!
//! Output (per DESIGN.md §D.4 EvidencePack v1.2):
//!   - file, line
//!   - kind (string above)
//!   - size_arg (string repr — caller side does numeric extraction)
//!   - containing_fn (best-effort — set when the call sits inside `fn ...`)
//!   - alloc_freq_estimate is left at 0; needs cross-fn callgraph
//!     analysis (deferred to v2).

use std::path::{Path, PathBuf};

use quote::ToTokens;
use serde::Serialize;
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Expr, ExprCall, ExprMacro, ExprPath, ItemFn, Macro};

#[derive(Serialize, Debug)]
pub struct AllocSite {
    pub file:                String,
    pub line:                usize,
    pub kind:                String,
    pub size_arg:            String,
    pub containing_fn:       String,
    pub alloc_freq_estimate: u64,
}

pub struct Walker<'a> {
    file:        PathBuf,
    text:        &'a str,
    items:       Vec<AllocSite>,
    fn_stack:    Vec<String>,
}

impl<'a> Walker<'a> {
    pub fn new(file: &Path, text: &'a str) -> Self {
        Walker {
            file:     file.to_path_buf(),
            text,
            items:    Vec::new(),
            fn_stack: Vec::new(),
        }
    }

    pub fn into_items(self) -> Vec<AllocSite> {
        self.items
    }

    fn cur_fn(&self) -> String {
        self.fn_stack.last().cloned().unwrap_or_else(|| "<top-level>".to_string())
    }

    fn record(&mut self, line: usize, kind: &str, size_arg: String) {
        self.items.push(AllocSite {
            file:                self.file.display().to_string(),
            line,
            kind:                kind.to_string(),
            size_arg,
            containing_fn:       self.cur_fn(),
            alloc_freq_estimate: 0,
        });
    }
}

impl<'a, 'ast> Visit<'ast> for Walker<'a> {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.fn_stack.push(node.sig.ident.to_string());
        syn::visit::visit_item_fn(self, node);
        self.fn_stack.pop();
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        if let Expr::Path(ExprPath { path, .. }) = &*node.func {
            let path_str = path.to_token_stream().to_string().replace(' ', "");
            let line = node.span().start().line;

            // Match common heap-alloc forms.
            let kind_size: Option<(&str, String)> = match path_str.as_str() {
                "Vec::new" | "Vec::<_>::new" => Some(("Vec::new", "0".to_string())),
                "Vec::with_capacity" | "Vec::<_>::with_capacity" =>
                    Some(("Vec::with_capacity", first_arg_str(node))),
                "Vec::from" | "Vec::<_>::from" =>
                    Some(("Vec::from", first_arg_str(node))),
                "Box::new" =>
                    Some(("Box::new", "1".to_string())),
                "String::new" =>
                    Some(("String::new", "0".to_string())),
                "String::with_capacity" =>
                    Some(("String::with_capacity", first_arg_str(node))),
                _ => {
                    // Also catch generic-spelt forms like `Vec::<u8>::new`.
                    if path_str.starts_with("Vec::") && path_str.ends_with("::new") {
                        Some(("Vec::new", "0".to_string()))
                    } else if path_str.starts_with("Vec::") && path_str.ends_with("::with_capacity") {
                        Some(("Vec::with_capacity", first_arg_str(node)))
                    } else {
                        None
                    }
                }
            };
            if let Some((kind, size_arg)) = kind_size {
                self.record(line, kind, size_arg);
            }
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        if is_vec_macro(&node.mac) {
            let line = node.mac.span().start().line;
            let tokens = node.mac.tokens.to_string();
            let size_arg = if tokens.is_empty() {
                "0".to_string()
            } else if let Some(rest) = tokens.strip_prefix(';').or_else(|| tokens.split_once(';').map(|(_, r)| r)) {
                rest.trim().to_string()
            } else {
                // `vec![1, 2, 3]` form: count commas + 1
                let n = tokens.split(',').count();
                n.to_string()
            };
            self.record(line, "vec!", size_arg);
        }
        syn::visit::visit_expr_macro(self, node);
    }
}

fn is_vec_macro(m: &Macro) -> bool {
    m.path.segments.last().map(|s| s.ident == "vec").unwrap_or(false)
}

fn first_arg_str(call: &ExprCall) -> String {
    call.args.first()
        .map(|a| a.to_token_stream().to_string().replace(' ', ""))
        .unwrap_or_else(|| "unknown".to_string())
}
