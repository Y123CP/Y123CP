//! Loop-invariant condition detector — lightweight syntactic AST analysis.
//!
//! For each `while` / `for` / `loop` body, emit one `LoopInvariantCond`
//! per `if` / `match` whose condition expression reads NO identifier
//! that is written anywhere in the loop body (including nested loops),
//! and NO induction variable of the enclosing `for` loop.
//!
//! Conservative write model — false negatives over false positives:
//!   - `lhs = ...` and compound `+=` `-=` `*=` etc.   → leaf(lhs)
//!   - `&mut x`                                       → leaf(x)
//!   - **any** `recv.method(...)` call                → leaf(recv)
//!   - `let x = ...` rebinding inside the loop        → x
//!   - tokens inside a macro invocation (crude)       → every ident token
//!
//! Reads: every `ExprPath` first segment in the cond expression, minus
//! a small set of language keywords/variants (`Some`/`None`/`Ok`/`Err`/
//! `true`/`false`/`self`/`Self`).
//!
//! Output (one row per detected invariant cond):
//!   file, line, loop_kind {while|for|loop}, loop_line, containing_fn,
//!   cond_snippet (truncated), reads (sorted dedup list)

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use quote::ToTokens;
use serde::Serialize;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{
    BinOp, Block, Expr, ExprAssign, ExprBinary, ExprCall, ExprCast, ExprField,
    ExprForLoop, ExprIf, ExprIndex, ExprLet, ExprLoop, ExprMacro, ExprMatch,
    ExprMethodCall, ExprParen, ExprPath, ExprReference, ExprUnary, ExprWhile,
    ImplItemFn, ItemFn, Local, Pat, PatIdent, PatTuple, PatTupleStruct,
};

#[derive(Serialize, Debug)]
pub struct LoopInvariantCond {
    pub file:          String,
    pub line:          usize,
    pub loop_kind:     String,    // "while" | "for" | "loop"
    pub loop_line:     usize,
    pub containing_fn: String,
    pub cond_snippet:  String,
    pub reads:         Vec<String>,
}

pub struct Walker<'a> {
    file:     PathBuf,
    items:    Vec<LoopInvariantCond>,
    fn_stack: Vec<String>,
    _text:    &'a str,           // reserved for future use (e.g. extracting raw src)
}

impl<'a> Walker<'a> {
    pub fn new(file: &Path, text: &'a str) -> Self {
        Walker {
            file:     file.to_path_buf(),
            items:    Vec::new(),
            fn_stack: Vec::new(),
            _text:    text,
        }
    }

    pub fn into_items(self) -> Vec<LoopInvariantCond> {
        self.items
    }

    fn cur_fn(&self) -> String {
        self.fn_stack.last().cloned().unwrap_or_else(|| "<top-level>".to_string())
    }

    fn handle_loop(&mut self, kind: &str, body: &Block,
                   loop_var: Option<String>, header_line: usize) {
        let mut wc = WriteCollector::default();
        wc.visit_block(body);

        let mut cc = CondCollector::default();
        cc.visit_block(body);

        for cond in cc.conds {
            let mut rc = ReadCollector::default();
            rc.visit_expr(&cond.expr);

            if rc.reads.is_empty() {
                continue;                    // pure literal cond (e.g. `true`)
            }

            // Free function calls in the cond (e.g. `fputc(fp,c) == EOF`,
            // `myfeof(stream)`, `ferror(stream)`) almost always have
            // hidden side effects on their args that pure syntax can't
            // see — skip rather than emit a false-positive "invariant".
            let mut cc_chk = CallChecker::default();
            cc_chk.visit_expr(&cond.expr);
            if cc_chk.has_call { continue; }

            if let Some(lv) = &loop_var {
                if rc.reads.contains(lv) { continue; }
            }
            if rc.reads.iter().any(|r| wc.writes.contains(r)) {
                continue;                    // some read is written in loop body
            }

            let mut reads: Vec<String> = rc.reads.into_iter().collect();
            reads.sort();
            self.items.push(LoopInvariantCond {
                file:          self.file.display().to_string(),
                line:          cond.line,
                loop_kind:     kind.to_string(),
                loop_line:     header_line,
                containing_fn: self.cur_fn(),
                cond_snippet:  truncate_snippet(&cond.snippet),
                reads,
            });
        }
    }
}

impl<'a, 'ast> Visit<'ast> for Walker<'a> {
    fn visit_item_fn(&mut self, f: &'ast ItemFn) {
        self.fn_stack.push(f.sig.ident.to_string());
        visit::visit_item_fn(self, f);
        self.fn_stack.pop();
    }
    fn visit_impl_item_fn(&mut self, f: &'ast ImplItemFn) {
        self.fn_stack.push(f.sig.ident.to_string());
        visit::visit_impl_item_fn(self, f);
        self.fn_stack.pop();
    }

    fn visit_expr_while(&mut self, w: &'ast ExprWhile) {
        let header_line = w.while_token.span().start().line;
        self.handle_loop("while", &w.body, None, header_line);
        visit::visit_expr_while(self, w);    // recurse — nested loops handled themselves
    }
    fn visit_expr_for_loop(&mut self, fr: &'ast ExprForLoop) {
        let lv = pat_first_name(&fr.pat);
        let header_line = fr.for_token.span().start().line;
        self.handle_loop("for", &fr.body, lv, header_line);
        visit::visit_expr_for_loop(self, fr);
    }
    fn visit_expr_loop(&mut self, l: &'ast ExprLoop) {
        let header_line = l.loop_token.span().start().line;
        self.handle_loop("loop", &l.body, None, header_line);
        visit::visit_expr_loop(self, l);
    }
}


// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pat_first_name(p: &Pat) -> Option<String> {
    match p {
        Pat::Ident(PatIdent { ident, .. }) => Some(ident.to_string()),
        Pat::Tuple(PatTuple { elems, .. }) =>
            elems.first().and_then(pat_first_name),
        Pat::TupleStruct(PatTupleStruct { elems, .. }) =>
            elems.first().and_then(pat_first_name),
        _ => None,
    }
}

/// Walk down a chain of field/index/method/cast/ref/paren wrappers to the
/// underlying identifier — the "leaf" name that the expression touches.
fn leaf_name(e: &Expr) -> Option<String> {
    match e {
        Expr::Path(ExprPath { path, .. }) =>
            path.segments.first().map(|s| s.ident.to_string()),
        Expr::Field(ExprField { base, .. })            => leaf_name(base),
        Expr::Index(ExprIndex { expr, .. })            => leaf_name(expr),
        Expr::MethodCall(ExprMethodCall { receiver, .. }) => leaf_name(receiver),
        Expr::Paren(ExprParen { expr, .. })            => leaf_name(expr),
        Expr::Unary(ExprUnary { expr, .. })            => leaf_name(expr),
        Expr::Reference(ExprReference { expr, .. })    => leaf_name(expr),
        Expr::Cast(ExprCast { expr, .. })              => leaf_name(expr),
        _ => None,
    }
}

fn truncate_snippet(s: &str) -> String {
    const MAX: usize = 160;
    if s.len() <= MAX { s.to_string() } else { format!("{} ...", &s[..MAX]) }
}


// ---------------------------------------------------------------------------
// Write collector — anything that may modify an identifier in the loop body
// ---------------------------------------------------------------------------

#[derive(Default)]
struct WriteCollector {
    writes: HashSet<String>,
}

impl<'ast> Visit<'ast> for WriteCollector {
    fn visit_expr_assign(&mut self, a: &'ast ExprAssign) {
        if let Some(n) = leaf_name(&a.left) { self.writes.insert(n); }
        visit::visit_expr_assign(self, a);
    }

    fn visit_expr_binary(&mut self, b: &'ast ExprBinary) {
        use BinOp::*;
        let compound = matches!(b.op,
            AddAssign(_)    | SubAssign(_)    | MulAssign(_)    |
            DivAssign(_)    | RemAssign(_)    | BitXorAssign(_) |
            BitAndAssign(_) | BitOrAssign(_)  | ShlAssign(_)    | ShrAssign(_)
        );
        if compound {
            if let Some(n) = leaf_name(&b.left) { self.writes.insert(n); }
        }
        visit::visit_expr_binary(self, b);
    }

    fn visit_expr_reference(&mut self, r: &'ast ExprReference) {
        if r.mutability.is_some() {
            if let Some(n) = leaf_name(&r.expr) { self.writes.insert(n); }
        }
        visit::visit_expr_reference(self, r);
    }

    fn visit_expr_method_call(&mut self, m: &'ast ExprMethodCall) {
        // Conservative: any method call writes its receiver.
        if let Some(n) = leaf_name(&m.receiver) { self.writes.insert(n); }
        visit::visit_expr_method_call(self, m);
    }

    fn visit_local(&mut self, l: &'ast Local) {
        // `let x = ...` inside the loop re-binds x every iteration.
        if let Some(n) = pat_first_name(&l.pat) { self.writes.insert(n); }
        visit::visit_local(self, l);
    }

    fn visit_expr_macro(&mut self, m: &'ast ExprMacro) {
        // Macros are opaque — pessimistically mark every ident token as written.
        let toks = m.mac.tokens.to_string();
        for tok in toks.split(|c: char| !c.is_alphanumeric() && c != '_') {
            if let Some(first) = tok.chars().next() {
                if first.is_alphabetic() || first == '_' {
                    self.writes.insert(tok.to_string());
                }
            }
        }
    }
}


// ---------------------------------------------------------------------------
// Cond collector — `if` / `match` at the current loop level only.
// Nested loops are skipped (they get their own handle_loop pass).
// ---------------------------------------------------------------------------

struct CondItem {
    expr:    Expr,
    line:    usize,
    snippet: String,
}

#[derive(Default)]
struct CondCollector {
    conds: Vec<CondItem>,
}

impl<'ast> Visit<'ast> for CondCollector {
    fn visit_expr_if(&mut self, ie: &'ast ExprIf) {
        // `if let PAT = EXPR` → use EXPR as the cond
        let (cond_expr, line) = match &*ie.cond {
            Expr::Let(ExprLet { expr, let_token, .. }) =>
                ((**expr).clone(), let_token.span().start().line),
            other =>
                ((*other).clone(), ie.if_token.span().start().line),
        };
        let snippet = cond_expr.to_token_stream().to_string();
        self.conds.push(CondItem { expr: cond_expr, line, snippet });
        visit::visit_expr_if(self, ie);
    }

    fn visit_expr_match(&mut self, m: &'ast ExprMatch) {
        let snippet = m.expr.to_token_stream().to_string();
        let line    = m.match_token.span().start().line;
        let expr    = (*m.expr).clone();
        self.conds.push(CondItem { expr, line, snippet });
        visit::visit_expr_match(self, m);
    }

    // STOP at nested loops — their conds are handled by Walker's outer pass.
    fn visit_expr_while(&mut self, _w: &'ast ExprWhile) {}
    fn visit_expr_for_loop(&mut self, _f: &'ast ExprForLoop) {}
    fn visit_expr_loop(&mut self, _l: &'ast ExprLoop) {}
}


// ---------------------------------------------------------------------------
// Call checker — does the cond contain a free function call?
// ---------------------------------------------------------------------------

#[derive(Default)]
struct CallChecker {
    has_call: bool,
}

impl<'ast> Visit<'ast> for CallChecker {
    fn visit_expr_call(&mut self, _c: &'ast ExprCall) {
        self.has_call = true;
    }
}


// ---------------------------------------------------------------------------
// Read collector — every ExprPath's first segment in the cond expression.
// ---------------------------------------------------------------------------

#[derive(Default)]
struct ReadCollector {
    reads: HashSet<String>,
}

impl<'ast> Visit<'ast> for ReadCollector {
    fn visit_expr_path(&mut self, p: &'ast ExprPath) {
        if let Some(seg) = p.path.segments.first() {
            let name = seg.ident.to_string();
            if !matches!(name.as_str(),
                "Some" | "None" | "Ok" | "Err" |
                "true" | "false" |
                "self" | "Self" | "super" | "crate"
            ) {
                self.reads.insert(name);
            }
        }
    }
}
