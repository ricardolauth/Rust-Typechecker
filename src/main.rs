use std::fmt;

pub enum Exp {
    Int {
        val: i32
    },
    Bool {
        val: bool
    },
    Plus {
        left: Box<Exp>,
        right: Box<Exp>
    },
    Mult{
        left: Box<Exp>,
        right: Box<Exp>
    },
    Or{
        left: Box<Exp>,
        right: Box<Exp>
    },
}

#[derive(PartialEq, Eq, Debug)]
enum Type{
    Int,
    Bool,
    None
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Bool => write!(f, "Bool"),
            Type::Int => write!(f, "Int"),
            Type::None => write!(f, "None"),
        }
    }
}

fn typecheck(exp: Exp) -> Type{
    fn assert_type(assert: Type, left: Type, right : Type) -> Type{
        return if assert == left && left == right {assert} else {Type::None}
    }

    match exp {
        Exp::Int { val: _ } => return Type::Int,
        Exp::Bool { val: _ } => return Type::Bool,
        Exp::Plus { left, right } => return assert_type(Type::Int, typecheck(*left), typecheck(*right)),
        Exp::Mult { left, right } => return assert_type(Type::Int, typecheck(*left), typecheck(*right)),
        Exp::Or { left, right } => return assert_type(Type::Bool, typecheck(*left), typecheck(*right)), 
        }
}

fn main() {
    let ex: Exp = Exp::Plus { 
        left: (Box::new(Exp::Mult { 
            left : Box::new(Exp::Int { val : 1 }), 
            right: Box::new(Exp::Int { val : 2 })})), 
        right: (Box::new(Exp::Plus { 
            left : Box::new(Exp::Int { val : 1 }), 
            right: Box::new(Exp::Int { val : 2 })}))};

    let result = typecheck(ex);
    print!("Type: {}\n", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_exp() {
        let ex: Exp = Exp::Plus { 
            left: (Box::new(Exp::Mult { 
                left : Box::new(Exp::Int { val : 1 }), 
                right: Box::new(Exp::Int { val : 2 })})), 
            right: (Box::new(Exp::Plus { 
                left : Box::new(Exp::Int { val : 1 }), 
                right: Box::new(Exp::Int { val : 2 })}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::Int, result);
    }

    #[test]
    fn test_int_exp2() {
        let ex: Exp = Exp::Mult { 
            left: (Box::new(Exp::Int { val: 4 })), 
            right: (Box::new(Exp::Plus { 
                left : Box::new(Exp::Int { val : 1 }), 
                right: Box::new(Exp::Int { val : 2 })}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::Int, result);
    }

    #[test]
    fn test_bool_exp() {
        let ex: Exp = Exp::Or { 
            left: (Box::new(Exp::Or { 
                left : Box::new(Exp::Bool { val : true }), 
                right: Box::new(Exp::Bool { val : false })})), 
            right: (Box::new(Exp::Or { 
                left : Box::new(Exp::Bool { val : false }), 
                right: Box::new(Exp::Bool { val : true })}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::Bool, result);
    }

    #[test]
    fn test_bool_exp2() {
        let ex: Exp = Exp::Or { 
            left: (Box::new(Exp::Or { 
                left : Box::new(Exp::Bool { val : true }), 
                right: Box::new(Exp::Bool { val : false })})), 
            right: (Box::new(Exp::Bool { val: false}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::Bool, result);
    }

    #[test]
    fn test_int_none_exp() {
        let ex: Exp = Exp::Plus { 
            left: (Box::new(Exp::Mult { 
                left : Box::new(Exp::Int { val : 1 }), 
                right: Box::new(Exp::Int { val : 2 })})), 
            right: (Box::new(Exp::Or { 
                left : Box::new(Exp::Bool { val : true }), 
                right: Box::new(Exp::Bool { val : false })}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::None, result);
    }

    #[test]
    fn test_int_none2_exp() {
        let ex: Exp = Exp::Mult { 
            left: (Box::new(Exp::Bool { val: true })), 
            right: (Box::new(Exp::Plus { 
                left : Box::new(Exp::Int { val : 1 }), 
                right: Box::new(Exp::Int { val : 2 })}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::None, result);
    }

    #[test]
    fn test_bool_none_exp() {
        let ex: Exp = Exp::Or { 
            left: (Box::new(Exp::Or { 
                left : Box::new(Exp::Bool { val : true }), 
                right: Box::new(Exp::Int { val : 2 })})), 
            right: (Box::new(Exp::Or { 
                left : Box::new(Exp::Bool { val : false }), 
                right: Box::new(Exp::Bool { val : true })}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::None, result);
    }

    #[test]
    fn test_bool_none2_exp2() {
        let ex: Exp = Exp::Or { 
            left: (Box::new(Exp::Mult { 
                left : Box::new(Exp::Int { val : 1 }), 
                right: Box::new(Exp::Int { val : 3 })})), 
            right: (Box::new(Exp::Bool { val: false}))};
        
        let result = typecheck(ex);

        assert_eq!(Type::None, result);
    }
}