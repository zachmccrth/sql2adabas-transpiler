use crate::ast_transformer::ast_to_adabas::*;
use std::fmt;
use std::fmt::Display;

// ───────────────────────────────
// 🌲 Trait for Tree Display
// ───────────────────────────────
pub trait TreeDisplay {
    fn fmt_tree(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result;
}

impl TreeDisplay for BooleanExpr {
    fn fmt_tree(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        let pad = "  ".repeat(indent);
        match self {
            BooleanExpr::LogicOp(logic) => {
                writeln!(f, "{}LogicOp: {:?}", pad, logic.op)?;
                logic.left.fmt_tree(f, indent + 1)?;
                logic.right.fmt_tree(f, indent + 1)
            }
            BooleanExpr::Comparison(comp) => {
                writeln!(
                    f,
                    "{}Comparison: {:?} {} {:?}",
                    pad, comp.op, comp.left.value, comp.right
                )
            }
        }
    }
}

// ───────────────────────────────
// 📦 Wrapper Struct to Use with `Display`
// ───────────────────────────────
pub struct TreeFormatter<'a>(pub &'a dyn TreeDisplay);

impl<'a> Display for TreeFormatter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_tree(f, 0)
    }
}
