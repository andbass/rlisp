
use std::any::TypeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Number,
    Bool,
    
    Symbol,
    String,

    List,

    Nil,

    HardFunc,  
    Lambda,

    Type,
    Quote(Box<Type>),

    Foreign(TypeId),
}
