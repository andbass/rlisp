
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    Bool,
    
    Symbol,
    Str,

    List,

    Nil,

    HardFunc,  
    Lambda,

    Type,
    Quote(Box<Type>),
}
