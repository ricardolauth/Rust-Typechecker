# Rust-Typechecker
## Comparison between Haskell and Rust
Haskell's expression syntax makes it perfect for this kind of use case. 
This results in an easy to read and short solution to this problem. 

```Haskell
data Type = TInt | TBool deriving Show

typecheck :: E -> Maybe Type
typecheck Zero = Just TInt
typecheck One = Just TInt
typecheck ETrue = Just TBool
typecheck EFalse = Just TBool
typecheck (Plus e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TInt, Just TInt) -> Just TInt
       (_, _) -> Nothing
typecheck (Mult e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TInt, Just TInt) -> Just TInt
       (_, _) -> Nothing
typecheck (Or e1 e2) =
     case (typecheck e1, typecheck e2) of
       (Just TBool, Just TBool) -> Just TBool
       (_, _) -> Nothing
```

Rust has a bit more "boilerplate" for defining an appropriate enum, 
```Rust
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
```
but in addition Rust's enums are very powerful, which also results in a readable and short solution. Rust's enum match expression combined with a simple control flow statement is well suited to this problem. 
```Rust
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
```
Rust brings the best of the imperative world to a memory-safe option.
I think most people get faster in Rust because they are probably familiar with other imperative programming languages.



       
