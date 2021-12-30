use std::collections::HashSet;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Eq, PartialEq, Debug)]
pub enum Op {
    Input(u8),
    Value(i64),
    Mod(Rc<Op>, Rc<Op>),
    Div(Rc<Op>, Rc<Op>),
    Add(Rc<Op>, Rc<Op>),
    Mul(Rc<Op>, Rc<Op>),
    Eql(Rc<Op>, Rc<Op>),
    Neql(Rc<Op>, Rc<Op>),
    If(Rc<Op>, Rc<Op>),
}

impl Op {
    fn type_string(&self) -> String {
        match self {
            Op::Input(i) => format!("input{i}"),
            Op::Value(value) => format!("{value}"),
            Op::Mod(_, _) => format!("%"),
            Op::Div(_, _) => format!("/"),
            Op::Add(_, _) => format!("+"),
            Op::Mul(_, _) => format!("*"),
            Op::Eql(_, _) => format!("="),
            Op::Neql(_, _) => format!("!="),
            Op::If(_, _) => format!("if"),
        }
    }

    pub fn to_dot(&self) -> String {
        let mut nodes = HashSet::new();
        let mut edges = Vec::new();
        let mut stack = Vec::new();
        stack.push(self);
        while let Some(node) = stack.pop() {
            let label = node.type_string();
            if nodes.insert(format!("n{:p}[label=\"{label}\"]", node)) {
                match &*node {
                    Op::Mod(a, b)
                    | Op::Add(a, b)
                    | Op::Mul(a, b)
                    | Op::Eql(a, b)
                    | Op::Neql(a, b) => {
                        edges.push(format!("n{:p} -> n{:p}", *a, node));
                        stack.push(&**a);
                        edges.push(format!("n{:p} -> n{:p}", *b, node));
                        stack.push(&**b);
                    }
                    Op::If(a, b) => {
                        edges.push(format!("n{:p} -> n{:p} [label=\"condition\"]", &**a, node));
                        stack.push(&**a);
                        edges.push(format!("n{:p} -> n{:p}", *b, node));
                        stack.push(&**b);
                    }
                    Op::Div(a, b) => {
                        edges.push(format!("n{:p} -> n{:p}", *a, node));
                        stack.push(&**a);
                        edges.push(format!("n{:p} -> n{:p} [label=\"divide by\"]", &**b, node));
                        stack.push(&**b);
                    }
                    _ => (),
                }
            }
        }
        let mut rtn = String::new();
        rtn.push_str("digraph N {\n");
        for n in nodes {
            rtn.push_str(&n);
            rtn.push_str("\n");
        }
        for e in edges {
            rtn.push_str(&e);
            rtn.push_str("\n");
        }
        rtn.push_str("}\n");
        rtn
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Input(number) => f.write_fmt(format_args!("input{}", number)),
            Op::Value(value) => f.write_fmt(format_args!("{}", value)),
            Op::Mod(a, b) => f.write_fmt(format_args!("({a} % {b})")),
            Op::Div(a, b) => f.write_fmt(format_args!("({a} / {b})")),
            Op::Add(a, b) => f.write_fmt(format_args!("({a} + {b})")),
            Op::Mul(a, b) => f.write_fmt(format_args!("({a} * {b})")),
            Op::Eql(a, b) => f.write_fmt(format_args!("(({a} == {b}) as i64)")),
            Op::If(a, b) => f.write_fmt(format_args!("(if {a} {{ {b} }} else {{ 0 }})")),
            Op::Neql(a, b) => f.write_fmt(format_args!("(({a} != {b}) as i64)")),
        }
    }
}
