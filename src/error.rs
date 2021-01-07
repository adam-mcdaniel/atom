use super::Value;
use std::{fmt, path::PathBuf};

#[derive(Clone, PartialEq)]
pub enum Error {
    SymbolNotDefined(String),

    CannotNegate(Value),
    CannotAdd(Value, Value),
    CannotSubtract(Value, Value),
    CannotMultiply(Value, Value),
    CannotDivide(Value, Value),
    CannotRemainder(Value, Value),

    CannotOrder(Value, Value),
    CannotCompare(Value, Value),

    CannotRange(Value, Value),

    CannotAnd(Value, Value),
    CannotOr(Value, Value),
    CannotNot(Value),

    InvalidArguments(Value, Vec<Value>),
    TooFewArguments(Value,  Vec<Value>),
    TooManyArguments(Value, Vec<Value>),

    CannotApply(Value),

    CannotIndexWith(Value, Value),
    IndexNotFound(Value, Value),

    InvalidCondition(Value),
    CannotIterateOver(Value),
    CannotExecuteProgram(Value),

    CannotChangeDir(PathBuf),

    ReadInputError,
    CouldNotParseFloat(Value),
    CouldNotParseInteger(Value),

    HomeDirectoryNotFound,
    DocumentsDirectoryNotFound,
    DesktopDirectoryNotFound,
    DownloadsDirectoryNotFound,
    VideosDirectoryNotFound,
    PicturesDirectoryNotFound,

    SyntaxError(String),
    CustomError(String),
}



impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::SymbolNotDefined(name) => write!(f, "symbol `{}` not defined", name),
            
            Self::CannotNegate(val)     => write!(f, "cannot negate `{:?}`", val),
            Self::CannotAdd(a, b)       => write!(f, "cannot add `{:?}` and `{:?}`", a, b),
            Self::CannotSubtract(a, b)  => write!(f, "cannot subtract `{:?}` and `{:?}`", a, b),
            Self::CannotMultiply(a, b)  => write!(f, "cannot multiply `{:?}` and `{:?}`", a, b),
            Self::CannotDivide(a, b)    => write!(f, "cannot divide `{:?}` and `{:?}`", a, b),
            Self::CannotRemainder(a, b) => write!(f, "cannot remainder `{:?}` and `{:?}`", a, b),
            
            Self::CannotOrder(a, b)   => write!(f, "cannot order `{:?}` and `{:?}`", a, b),
            Self::CannotCompare(a, b) => write!(f, "cannot compare `{:?}` and `{:?}`", a, b),
            
            Self::CannotRange(a, b) => write!(f, "cannot range from `{:?}` to `{:?}`", a, b),

            Self::IndexNotFound(a, b) => write!(f, "index `{:?}` not found in `{:?}`", b, a),

            Self::CannotAnd(a, b) => write!(f, "cannot logical and `{:?}` and `{:?}`", a, b),
            Self::CannotOr(a, b) => write!(f, "cannot logical or `{:?}` and `{:?}`", a, b),
            Self::CannotNot(x) => write!(f, "cannot logical not `{:?}`", x),
            
            Self::InvalidArguments(func, args) => write!(f, "invalid arguments in call `{:?}`", Value::Apply(Box::new(func.clone()), args.clone())),
            Self::TooFewArguments(func, args) => write!(f, "too few arguments in call `{:?}`", Value::Apply(Box::new(func.clone()), args.clone())),
            Self::TooManyArguments(func, args) => write!(f, "too many arguments in call `{:?}`", Value::Apply(Box::new(func.clone()), args.clone())),

            Self::CannotIndexWith(val, idx) => write!(f, "cannot index `{:?}` with `{:?}`", val, idx),
            Self::CannotApply(val) => write!(f, "cannot apply `{:?}`", val),

            Self::InvalidCondition(val) => write!(f, "invalid condition `{:?}`", val),
            Self::CannotIterateOver(val) => write!(f, "cannot iterate over `{:?}`", val),
            Self::CannotExecuteProgram(prog) => write!(f, "cannot execute program `{:?}`", prog),

            Self::CannotChangeDir(dir) => write!(f, "cannot change dir {:?}", dir),
            Self::ReadInputError => write!(f, "could not get user input"),

            Self::CouldNotParseFloat(x) => write!(f, "could not parse `{}` as an float", x),
            Self::CouldNotParseInteger(x) => write!(f, "could not parse `{}` as an integer", x),

            Self::HomeDirectoryNotFound => write!(f, "home directory not found"),
            Self::DocumentsDirectoryNotFound => write!(f, "documents directory not found"),
            Self::DesktopDirectoryNotFound => write!(f, "desktop directory not found"),
            Self::DownloadsDirectoryNotFound => write!(f, "downloads directory not found"),
            Self::VideosDirectoryNotFound => write!(f, "videos directory not found"),
            Self::PicturesDirectoryNotFound => write!(f, "pictures directory not found"),

            Self::SyntaxError(s) => write!(f, "{}", s),
            Self::CustomError(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::SymbolNotDefined(name) => write!(f, "symbol `{}` not defined", name),
            
            Self::CannotNegate(val)     => write!(f, "cannot negate `{:?}`", val),
            Self::CannotAdd(a, b)       => write!(f, "cannot add `{:?}` and `{:?}`", a, b),
            Self::CannotSubtract(a, b)  => write!(f, "cannot subtract `{:?}` and `{:?}`", a, b),
            Self::CannotMultiply(a, b)  => write!(f, "cannot multiply `{:?}` and `{:?}`", a, b),
            Self::CannotDivide(a, b)    => write!(f, "cannot divide `{:?}` and `{:?}`", a, b),
            Self::CannotRemainder(a, b) => write!(f, "cannot remainder `{:?}` and `{:?}`", a, b),
            
            Self::CannotOrder(a, b)   => write!(f, "cannot order `{:?}` and `{:?}`", a, b),
            Self::CannotCompare(a, b) => write!(f, "cannot compare `{:?}` and `{:?}`", a, b),
            
            Self::CannotRange(a, b) => write!(f, "cannot range from `{:?}` to `{:?}`", a, b),
            
            Self::IndexNotFound(a, b) => write!(f, "index `{:?}` not found in `{:?}`", b, a),
            
            Self::CannotAnd(a, b) => write!(f, "cannot logical and `{:?}` and `{:?}`", a, b),
            Self::CannotOr(a, b) => write!(f, "cannot logical or `{:?}` and `{:?}`", a, b),
            Self::CannotNot(x) => write!(f, "cannot logical not `{:?}`", x),
            
            Self::InvalidArguments(func, args) => write!(f, "invalid arguments in call `{:?}`", Value::Apply(Box::new(func.clone()), args.clone())),
            Self::TooFewArguments(func, args) => write!(f, "too few arguments in call `{:?}`", Value::Apply(Box::new(func.clone()), args.clone())),
            Self::TooManyArguments(func, args) => write!(f, "too many arguments in call `{:?}`", Value::Apply(Box::new(func.clone()), args.clone())),

            Self::CannotApply(val) => write!(f, "cannot apply `{:?}`", val),
            Self::CannotIndexWith(val, idx) => write!(f, "cannot index `{:?}` with `{:?}`", val, idx),

            Self::InvalidCondition(val) => write!(f, "invalid condition `{:?}`", val),
            Self::CannotIterateOver(val) => write!(f, "cannot iterate over `{:?}`", val),
            Self::CannotExecuteProgram(prog) => write!(f, "cannot execute program `{:?}`", prog),
            
            Self::CannotChangeDir(dir) => write!(f, "cannot change dir {:?}", dir),
            Self::ReadInputError => write!(f, "could not get user input"),

            Self::CouldNotParseFloat(x) => write!(f, "could not parse `{}` as an float", x),
            Self::CouldNotParseInteger(x) => write!(f, "could not parse `{}` as an integer", x),
         
            Self::HomeDirectoryNotFound => write!(f, "home directory not found"),
            Self::DocumentsDirectoryNotFound => write!(f, "documents directory not found"),
            Self::DesktopDirectoryNotFound => write!(f, "desktop directory not found"),
            Self::DownloadsDirectoryNotFound => write!(f, "downloads directory not found"),
            Self::VideosDirectoryNotFound => write!(f, "videos directory not found"),
            Self::PicturesDirectoryNotFound => write!(f, "pictures directory not found"),

            Self::SyntaxError(s) => write!(f, "{}", s),
            Self::CustomError(s) => write!(f, "{}", s),
        }
    }
}