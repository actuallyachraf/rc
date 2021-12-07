use crate::ast;

pub fn emit_ret() -> &'static str {
    "ret"
}

pub fn emit_bin_op(op: ast::BinOp) -> &'static str {
    match op {
        ast::BinOp::Add => "add",
        ast::BinOp::Sub => "sub",
        ast::BinOp::Mul => "mul",
        ast::BinOp::Div => "div",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_emit_bin_op() {
        let expected = "add";
        let input = ast::BinOp::Add;

        assert_eq!(expected,emit_bin_op(input))
    }
}