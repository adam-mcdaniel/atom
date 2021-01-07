use std::{
    collections::BTreeMap,
    process::Command,
    path::PathBuf,
    fmt,
};

use super::{Environment, Error};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    bytes: usize
}

impl Size {
    pub fn from_bytes(bytes: usize) -> Self { Self { bytes } }
    pub fn from_kilobytes(kb: f64) -> Self { Self::from_bytes((kb * 1000.0) as usize) }
    pub fn from_megabytes(mb: f64) -> Self { Self::from_kilobytes(mb * 1000.0) }
    pub fn from_gigabytes(gb: f64) -> Self { Self::from_megabytes(gb * 1000.0) }

    fn as_bytes(&self) -> usize { self.bytes }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let decimal_places = (self.bytes as f64).log10() as u32;

        if decimal_places < 3 {
            write!(f, "{} bytes", self.bytes)
        } else if decimal_places < 6 {
            write!(f, "{}kb", self.bytes as f64 / 1000.0)
        } else if decimal_places < 9 {
            write!(f, "{}mb", self.bytes as f64 / 1000000.0)
        } else {
            write!(f, "{}gb", self.bytes as f64 / 1000000000.0)
        }
    }
}

#[derive(Clone)]
pub enum Value {
    Symbol(String),

    Builtin(String, fn(&Vec<Value>, &mut Environment) -> Result<Value, Error>),
    Apply(Box<Self>, Vec<Self>),
    
    Index(Box<Self>, Box<Self>),

    Run(Box<Self>, Vec<Self>),
    Macro(Vec<String>, Box<Self>),
    Lambda(Vec<String>, Box<Self>, Environment),

    Range(Box<Self>, Box<Self>),
    Define(String, Box<Self>),
    
    // For AST purposes
    Grouped(Box<Self>),

    Do(Vec<Self>),
    Scope(Vec<Self>),

    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Not(Box<Self>),
    
    Negate(Box<Self>),
    Add(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
    Divide(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Remainder(Box<Self>, Box<Self>),
    
    Greater(Box<Self>, Box<Self>),
    Less(Box<Self>, Box<Self>),
    GreaterEqual(Box<Self>, Box<Self>),
    LessEqual(Box<Self>, Box<Self>),
    Equal(Box<Self>, Box<Self>),
    NotEqual(Box<Self>, Box<Self>),

    Conditional(Box<Self>, Box<Self>, Box<Self>),
    If(Box<Self>, Box<Self>, Box<Self>),
    While(Box<Self>, Box<Self>),
    For(String, Box<Self>, Box<Self>),

    Path(PathBuf),
    Size(Size),

    Boolean(bool),
    String(String),
    Integer(i32),
    Float(f64),
    List(Vec<Self>),
    Table(BTreeMap<String, Self>),
    Nil,

    Error(Box<Error>),
}

impl Value {
    pub fn symbol(s: impl ToString) -> Self {
        Self::Symbol(s.to_string())
    }

    pub fn string(s: impl ToString) -> Self {
        Self::String(s.to_string())
    }

    pub fn builtin(name: impl ToString, body: fn(&Vec<Value>, &mut Environment) -> Result<Value, Error>) -> Self {
        Self::Builtin(name.to_string(), body)
    }

    fn get_used_symbols(&self) -> Vec<String> {
        match self {
            Self::Symbol(name) => vec![name.clone()],
            Self::Apply(x, args) | Self::Run(x, args) => {
                let mut result = x.get_used_symbols();
                for arg in args {
                    result.extend(arg.get_used_symbols());
                }
                result
            }

            Self::Define(_, value) => value.get_used_symbols(),

            Self::Macro(_, body)     => body.get_used_symbols(),
            Self::Lambda(_, body, _) => body.get_used_symbols(),

            Self::While(cond, x) => {
                let mut result = cond.get_used_symbols();
                result.extend(x.get_used_symbols());
                result
            }

            Self::For(_, list, x) => {
                let mut result = list.get_used_symbols();
                result.extend(x.get_used_symbols());
                result
            }

            Self::If(a, b, c) | Self::Conditional(a, b, c) => {
                let mut result = a.get_used_symbols();
                result.extend(b.get_used_symbols());
                result.extend(c.get_used_symbols());
                result
            }

            Self::List(vals) | Self::Do(vals) | Self::Scope(vals) => {
                let mut result = vec![];
                for val in vals {
                    result.extend(val.get_used_symbols());
                }
                result
            }

            Self::And(a, b)
            | Self::Or(a, b)
            | Self::Index(a, b)
            | Self::Range(a, b)
            | Self::Add(a, b)
            | Self::Subtract(a, b)
            | Self::Multiply(a, b)
            | Self::Divide(a, b)
            | Self::Remainder(a, b)
            | Self::Greater(a, b)
            | Self::GreaterEqual(a, b)
            | Self::Less(a, b)
            | Self::LessEqual(a, b)
            | Self::Equal(a, b)
            | Self::NotEqual(a, b) => {
                let mut result = a.get_used_symbols();
                result.extend(b.get_used_symbols());
                result
            }

            Self::Grouped(x) | Self::Not(x) | Self::Negate(x) => x.get_used_symbols(),

            Self::Builtin(_, _)
            | Self::Path(_)
            | Self::Size(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::String(_)
            | Self::Boolean(_)
            | Self::Error(_)
            | Self::Nil => vec![],
            Self::Table(items) => {
                let mut result = vec![];
                for (_, val) in items {
                    result.extend(val.get_used_symbols());
                }
                result
            }
        }
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self {
            Self::Index(val, idx) => {
                match (val.eval(env)?, idx.eval(env)?) {
                    (Self::String(s), Self::Integer(i)) if i < 0 => {
                        Self::Index(
                            Box::new(Self::String(s.chars().rev().collect())),
                            Box::new(Self::Integer(-i - 1))
                        ).eval(env)
                    }
                    (Self::List(s), Self::Integer(i)) if i < 0 => {
                        Self::Index(
                            Box::new(Self::List(s.into_iter().rev().collect())),
                            Box::new(Self::Integer(-i - 1))
                        ).eval(env)
                    }

                    (Self::String(s), Self::Integer(i)) => match s.chars().nth(i as usize) {
                        Some(ch) => Ok(Self::String(ch.to_string())),
                        None => Err(Error::IndexNotFound(Self::String(s.clone()), Self::Integer(i)))
                    },
                    (Self::List(list), Self::Integer(i)) => {
                        if 0 <= i && i < list.len() as i32 {
                            Ok(list[i as usize].clone())
                        } else {
                            Err(Error::IndexNotFound(Self::List(list), Self::Integer(i)))
                        }
                    },
                    (Self::Table(table), Self::String(x)) => {
                        if let Some(val) = table.get(&x) {
                            Ok(val.clone())
                        } else {
                            Err(Error::IndexNotFound(Self::Table(table), Self::String(x)))
                        }
                    }
                    (Self::Error(e), _) | (_, Self::Error(e)) => Ok(Self::Error(e)),
                    (x, y) => Err(Error::CannotIndexWith(x, y)),
                }
            }

            Self::Range(from, to) => {
                match (from.eval(env)?, to.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => {
                        Ok(Self::List((x..y).map(Self::Integer).collect::<Vec<Self>>()))
                    }
                    (Self::Integer(x), Self::Float(y)) => {
                        Ok(Self::List((x..y as i32).map(Self::Integer).collect::<Vec<Self>>()))
                    }
                    (Self::Float(x), Self::Integer(y)) => {
                        Ok(Self::List((x as i32..y).map(Self::Integer).collect::<Vec<Self>>()))
                    }
                    (Self::Float(x), Self::Float(y)) => {
                        Ok(Self::List((x as i32..y as i32).map(Self::Integer).collect::<Vec<Self>>()))
                    }
                    (Self::Error(e), _) | (_, Self::Error(e)) => Ok(Self::Error(e)),
                    (x, y) => Err(Error::CannotRange(x, y))
                }
            }

            Self::Symbol(name) => Ok(if let Ok(val) = env.get(name) {
                val
            } else {
                self.clone()
            }),

            Self::While(cond, body) => {
                let mut acc = Value::Nil;
                loop {
                    match cond.eval(env)? {
                        Self::Boolean(true) => acc = body.eval(env)?,
                        Self::Boolean(false) => break,
                        x => return Err(Error::CannotIterateOver(x.clone()))
                    }
                }
                Ok(acc)
            }

            Self::For(name, iter, body) => {
                match iter.eval(env)? {
                    Self::List(list) => {
                        let mut acc = Value::Nil;
                        for item in list {
                            env.define(name, item);
                            acc = body.eval(env)?;
                        }
                        Ok(acc)
                    }
                    Self::Error(e) => Ok(Self::Error(e)),
                    x => Err(Error::CannotIterateOver(x.clone()))
                }
            }

            Self::Do(vals) => {
                let mut acc = Value::Nil;
                for val in vals {
                    acc = val.eval(env)?;
                }
                Ok(acc)
            }
            
            Self::Scope(vals) => {
                let mut scope = env.clone();
                let mut acc = Value::Nil;
                for val in vals {
                    acc = val.eval(&mut scope)?;
                }
                Ok(acc)
            }

            Self::Conditional(cond, a, b) | Self::If(cond, a, b) => {
                match cond.eval(env)? {
                    Self::Boolean(x) => Ok(if x { a.eval(env)? } else { b.eval(env)? }),
                    Self::Error(e) => Ok(Self::Error(e)),
                    x => Err(Error::InvalidCondition(x))
                }
            }

            Self::Grouped(x) => x.eval(env),

            Self::Run(program, arguments) => {
                let program = program.eval(env)?;

                match &program {
                    Self::Path(_) | Self::String(_) | Self::Symbol(_) => {
                        let mut result = Command::new(&program.to_string());
                        result.current_dir(env.get_cwd()?);

                        for arg in arguments {
                            result.arg(&arg.eval(env)?.to_string());
                        }

                        if let Ok(status) = result.status() {
                            if status.success() {
                                Ok(Self::Integer(0))
                            } else if let Some(code) = status.code() {
                                Ok(Self::Integer(code))
                            } else {
                                Ok(Self::Integer(1))
                            }
                        } else {
                            Err(Error::CannotExecuteProgram(program.clone()))
                        }
                    }

                    Self::Error(e) => Ok(Self::Error(e.clone())),

                    other => Self::Apply(Box::new(other.clone()), arguments.clone()).eval(env),
                }
            }

            Self::Define(name, value) => {
                let result = value.eval(env)?;
                env.define(name.clone(), result.clone());
                Ok(result)
            }

            Self::GreaterEqual(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Boolean(x>=y),
                    (Self::Integer(x), Self::Float(y))   => Self::Boolean((x as f64)>=y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Boolean(x>=(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Boolean(x>=y),

                    (Self::Size(x), Self::Size(y)) => Self::Boolean(x.as_bytes() >= y.as_bytes()),
                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotOrder(x, y))
                })
            }

            Self::Greater(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Boolean(x>y),
                    (Self::Integer(x), Self::Float(y))   => Self::Boolean((x as f64)>y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Boolean(x>(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Boolean(x>y),

                    (Self::Size(x), Self::Size(y)) => Self::Boolean(x.as_bytes() > y.as_bytes()),
                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotOrder(x, y))
                })
            }

            Self::Less(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Boolean(x<y),
                    (Self::Integer(x), Self::Float(y))   => Self::Boolean((x as f64)<y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Boolean(x<(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Boolean(x<y),

                    (Self::Size(x), Self::Size(y)) => Self::Boolean(x.as_bytes() < y.as_bytes()),
                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotOrder(x, y))
                })
            }

            Self::LessEqual(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Boolean(x<=y),
                    (Self::Integer(x), Self::Float(y))   => Self::Boolean((x as f64)<=y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Boolean(x<=(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Boolean(x<=y),

                    (Self::Size(x), Self::Size(y)) => Self::Boolean(x.as_bytes() <= y.as_bytes()),

                    (x, y) => return Err(Error::CannotOrder(x, y))
                })
            }

            Self::Equal(a, b) => Ok(Self::Boolean(a.eval(env)? == b.eval(env)?)),
            Self::NotEqual(a, b) => Ok(Self::Boolean(a.eval(env)? != b.eval(env)?)),

            Self::Apply(func, args) => {
                match func.eval(env)? {
                    Self::Macro(params, body) => {
                        if params.len() > args.len() {
                            Err(Error::TooFewArguments(*func.clone(), args.clone()))
                        } else if params.len() < args.len() {
                            Err(Error::TooManyArguments(*func.clone(), args.clone()))
                        } else {
                            let mut save_vars = BTreeMap::new();
                            for param in &params {
                                if env.is_defined(param) {
                                    save_vars.insert(param.clone(), env.get(param)?);
                                }
                            }

                            for (param, arg) in params.iter().zip(args) {
                                let val = arg.eval(env)?;
                                env.define(param.clone(), val);
                            }

                            let result = body.eval(env);

                            for (key, val) in save_vars {
                                env.define(key, val);
                            }

                            result
                            // let mut tmp = env.clone();
                            // for (param, arg) in params.iter().zip(args) {
                            //     let val = arg.eval(env)?;
                            //     tmp.define(param.clone(), val);
                            // }
                            // let result = body.eval(&mut tmp);
                            // for (key, val) in tmp.get_symbols() {
                            //     if !params.contains(key) {
                            //         env.define(key.clone(), val.clone());
                            //         println!("DEFINING {} to `{}`", key, val);
                            //     }
                            // }
                        }
                    }

                    Self::Lambda(params, body, captured) => {
                        if params.len() > args.len() {
                            Err(Error::TooFewArguments(*func.clone(), args.clone()))
                        } else if params.len() < args.len() {
                            Err(Error::TooManyArguments(*func.clone(), args.clone()))
                        } else {
                            let mut tmp = env.combine(&captured);

                            for (param, arg) in params.iter().zip(args) {
                                let val = arg.eval(env)?;
                                tmp.define(param.clone(), val);
                            }

                            body.eval(&mut tmp)
                        }
                    }

                    Self::Builtin(_, builtin) => builtin(args, env),
                    Self::Error(e) => Ok(Self::Error(e)),

                    _ => Err(Error::CannotApply(*func.clone()))
                }
            }

            Self::Lambda(params, body, old_env) => {
                let mut tmp_env = old_env.clone();
                for symbol in body.get_used_symbols() {
                    if env.is_defined(&symbol) && !old_env.is_defined(&symbol) {
                        tmp_env.define(symbol.clone(), env.get(&symbol)?);
                    }
                }

                Ok(Self::Lambda(params.clone(), body.clone(), tmp_env))
            }

            Self::And(a, b) => {
                match (a.eval(env)?, b.eval(env)?) {
                    (Self::Boolean(x), Self::Boolean(y)) => Ok(Self::Boolean(x && y)),
                    (Self::Error(e), _) | (_, Self::Error(e)) => Ok(Self::Error(e)),
                    (x, y) => Err(Error::CannotAnd(x, y))
                }
            }

            Self::Or(a, b) => {
                match (a.eval(env)?, b.eval(env)?) {
                    (Self::Boolean(x), Self::Boolean(y)) => Ok(Self::Boolean(x || y)),
                    (Self::Error(e), _) | (_, Self::Error(e)) => Ok(Self::Error(e)),
                    (x, y) => Err(Error::CannotOr(x, y))
                }
            }

            Self::Not(a) => {
                match a.eval(env)? {
                    Self::Boolean(x) => Ok(Self::Boolean(!x)),
                    Self::Error(e) => Ok(Self::Error(e)),
                    x => Err(Error::CannotNot(x))
                }
            }

            Self::Negate(a) => {
                Ok(match a.eval(env)? {
                    Self::Float(x)   => Self::Float(-x),
                    Self::Integer(x) => Self::Integer(-x),

                    Self::Error(e) => Self::Error(e),

                    x => return Err(Error::CannotNegate(x))
                })
            }

            Self::Add(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Integer(x+y),
                    (Self::Integer(x), Self::Float(y))   => Self::Float((x as f64)+y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Float(x+(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Float(x+y),

                    (Self::Path(x), Self::Path(y))   => Self::Path(x.join(y)),
                    (Self::Path(x), Self::Symbol(y)) | (Self::Path(x), Self::String(y)) => Self::Path(x.join(y)),

                    (Self::String(x), Self::String(y)) => Self::String(x+&y),

                    (Self::List(x), Self::List(y)) => {
                        let mut z = x.clone();
                        z.extend(y);
                        Self::List(z)
                    }

                    (Self::Table(x), Self::Table(y)) => {
                        let mut z = x.clone();
                        z.extend(y);
                        Self::Table(z)
                    }

                    (Self::Size(x), Self::Size(y)) => Self::Size(Size::from_bytes(x.as_bytes() + y.as_bytes())),

                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotAdd(x, y))
                })
            }

            Self::Subtract(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Integer(x-y),
                    (Self::Integer(x), Self::Float(y))   => Self::Float((x as f64)-y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Float(x-(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Float(x-y),

                    (Self::Size(x), Self::Size(y)) => Self::Size(Size::from_bytes(x.as_bytes() - y.as_bytes())),

                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotSubtract(x, y))
                })
            }

            Self::Multiply(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Integer(x*y),
                    (Self::Integer(x), Self::Float(y))   => Self::Float((x as f64)*y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Float(x*(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Float(x*y),

                    (Self::String(x),  Self::Integer(y)) => Self::String(x.repeat(y as usize)),
                    (Self::List(x),    Self::Integer(y)) => {
                        let mut result = vec![];
                        for _ in 0..y {
                            result.extend(x.clone());
                        }
                        Self::List(result)
                    }

                    (Self::Float(x), Self::Size(y))   => Self::Size(Size::from_bytes((x * y.as_bytes() as f64) as usize)),
                    (Self::Integer(x), Self::Size(y)) => Self::Size(Size::from_bytes(x as usize * y.as_bytes())),

                    (Self::Size(x), Self::Float(y))   => Self::Size(Size::from_bytes((x.as_bytes() as f64 * y) as usize)),
                    (Self::Size(x), Self::Integer(y)) => Self::Size(Size::from_bytes(x.as_bytes() * y as usize)),

                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotMultiply(x, y))
                })
            }

            Self::Divide(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Integer(x/y),
                    (Self::Integer(x), Self::Float(y))   => Self::Float((x as f64)/y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Float(x/(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Float(x/y),

                    (Self::Size(x), Self::Integer(y)) => Self::Size(Size::from_bytes(x.as_bytes() / y as usize)),
                    (Self::Size(x), Self::Float(y)) => Self::Size(Size::from_bytes((x.as_bytes() as f64 / y) as usize)),

                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotDivide(x, y))
                })
            }

            Self::Remainder(a, b) => {
                Ok(match (a.eval(env)?, b.eval(env)?) {
                    (Self::Integer(x), Self::Integer(y)) => Self::Integer(x%y),
                    (Self::Integer(x), Self::Float(y))   => Self::Float((x as f64)%y),
                    (Self::Float(x),   Self::Integer(y)) => Self::Float(x%(y as f64)),
                    (Self::Float(x),   Self::Float(y))   => Self::Float(x%y),

                    (Self::Error(e), _) | (_, Self::Error(e)) => Self::Error(e),

                    (x, y) => return Err(Error::CannotRemainder(x, y))
                })
            }

            Self::List(items) => {
                Ok(Self::List(items.iter().map(|x| x.eval(env)).collect::<Result<Vec<Self>, Error>>()?))
            }

            Self::Table(items) => {
                let mut result = BTreeMap::new();
                for (key, val) in items {
                    result.insert(key.clone(), val.eval(env)?);
                }
                Ok(Self::Table(result))
            }

            Self::Builtin(_, _)
            | Self::Macro(_, _)
            | Self::Path(_)
            | Self::Size(_)
            | Self::Boolean(_)
            | Self::String(_)
            | Self::Integer(_)
            | Self::Float(_)
            | Self::Error(_)
            | Self::Nil => Ok(self.clone())
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(x), Self::Integer(y)) => x == y,
            (Self::Integer(x), Self::Float(y))   => (*x as f64) == *y,
            (Self::Float(x),   Self::Integer(y)) => *x == (*y as f64),
            (Self::Float(x),   Self::Float(y))   => x == y,
    
            (Self::Symbol(x),  Self::Symbol(y))  => x == y,
            (Self::Path(x),    Self::Path(y))    => x == y,
            (Self::Size(x),    Self::Size(y))    => x == y,
            (Self::String(x),  Self::String(y))  => x == y,
            (Self::Boolean(x), Self::Boolean(y)) => x == y,
            (Self::Table(x),   Self::Table(y))   => x == y,
            (Self::List(x),    Self::List(y))    => x == y,

            (Self::Builtin(x, _), Self::Builtin(y, _)) => x == y,
            (Self::Error(x), Self::Error(y)) => x == y,

            (Self::Nil, Self::Nil) => true,

            (_, _) => false,
        }
    }
}


impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::While(cond, body) => write!(f, "while {} {}", cond, body),
            Self::For(name, iter, body) => write!(f, "for {} in {} {}", name, iter, body),

            Self::Do(vals) => {
                let mut result = String::from("{");
                for item in vals {
                    result += &format!("{}; ", item);
                }
                if !vals.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}}}", result)
            }
            
            Self::Scope(vals) => {
                let mut result = String::from("{");
                for item in vals {
                    result += &format!("{}; ", item);
                }
                if !vals.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}}}", result)
            }

            Self::Symbol(name) => write!(f, "{}", name),
            Self::Builtin(name, _) => write!(f, "{}", name),

            Self::Grouped(x) => write!(f, "({})", x),

            Self::Define(name, value) => write!(f, "{} := {}", name, value),

            Self::Index(a, b) => write!(f, "{}[{}]", a, b),
            Self::Range(a, b) => write!(f, "{} to {}", a, b),

            Self::And(a, b) => write!(f, "{} and {}", a, b),
            Self::Or(a, b) => write!(f, "{} or {}", a, b),
            Self::Not(x) => write!(f, "not {}", x),

            Self::Negate(x) => write!(f, "-{}", x),
            Self::Add(a, b) => write!(f, "{} + {}", a, b),
            Self::Subtract(a, b) => write!(f, "{} - {}", a, b),
            Self::Multiply(a, b) => write!(f, "{} * {}", a, b),
            Self::Divide(a, b) => write!(f, "{} / {}", a, b),
            Self::Remainder(a, b) => write!(f, "{} % {}", a, b),

            Self::Greater(a, b) => write!(f, "{} > {}", a, b),
            Self::GreaterEqual(a, b) => write!(f, "{} ≥ {}", a, b),
            Self::Less(a, b) => write!(f, "{} < {}", a, b),
            Self::LessEqual(a, b) => write!(f, "{} ≤ {}", a, b),
            Self::Equal(a, b) => write!(f, "{} = {}", a, b),
            Self::NotEqual(a, b) => write!(f, "{} ≠ {}", a, b),

            Self::If(cond, a, b) => write!(f, "if ({}) {} else {}", cond, a, b),
            Self::Conditional(cond, a, b) => write!(f, "{}? {} : {}", cond, a, b),

            Self::Lambda(params, body, _) => {
                let mut result = String::from("fn(");
                for param in params {
                    result += &format!("{}, ", param);
                }
                if !params.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}) -> {}", result, body)
            }

            Self::Macro(params, body) => {
                let mut result = String::from("macro(");
                for param in params {
                    result += &format!("{}, ", param);
                }
                if !params.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}) -> {}", result, body)
            }

            Self::Run(program, args) => {
                write!(f, "{}'", program)?;
                for arg in args {
                    write!(f, " {}", arg)?;
                }
                Ok(())
            }

            Self::Apply(func, args) => {
                let mut result = format!("{}(", func);
                for arg in args {
                    result += &format!("{}, ", arg);
                }
                if !args.is_empty() {
                    result.pop();
                    result.pop();
                }
                write!(f, "{})", result)
            }

            Self::Path(path) => write!(f, "{}", path.as_path().display().to_string()),
            Self::Size(size) => write!(f, "{}", size),
            Self::Integer(x) => write!(f, "{}", x),
            Self::Float(x)   => write!(f, "{}", x),
            Self::String(x)  => write!(f, "{}", x),
            Self::Boolean(x) => write!(f, "{}", x),
            Self::Nil => write!(f, "nil"),
            Self::List(x) => {
                let mut result = String::from("[");
                for item in x {
                    result += &format!("{:?}, ", item);
                }
                if !x.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}]", result)
            }
            Self::Table(x) => {
                let mut result = String::from("{ ");
                for (key, val) in x {
                    result += &format!("\"{}\": {:?}, ", key, val);
                }
                if !x.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{} }}", result)
            }

            Self::Error(e) => write!(f, "{}", e)
        }
    }
}


impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::While(cond, body) => write!(f, "while {:?} {:?}", cond, body),
            Self::For(name, iter, body) => write!(f, "for {:?} in {:?} {:?}", name, iter, body),

            Self::Do(vals) => {
                let mut result = String::from("{ ");
                for item in vals {
                    result += &format!("{:?}; ", item);
                }
                if !vals.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{} }}", result)
            }
            
            Self::Scope(vals) => {
                let mut result = String::from("{ ");
                for item in vals {
                    result += &format!("{:?}; ", item);
                }
                if !vals.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{} }}", result)
            }

            Self::Symbol(name) => write!(f, "{}", name),
            Self::Builtin(name, func) => write!(f, "<{} at {:p}>", name, func),

            Self::Grouped(x) => write!(f, "({:?})", x),

            Self::Define(name, value) => write!(f, "{} := {:?}", name, value),

            Self::Index(a, b) => write!(f, "{:?}[{:?}]", a, b),

            Self::Range(a, b) => write!(f, "{:?} to {:?}", a, b),

            Self::And(a, b) => write!(f, "{:?} and {:?}", a, b),
            Self::Or(a, b) => write!(f, "{:?} or {:?}", a, b),
            Self::Not(x) => write!(f, "not {:?}", x),

            Self::Negate(x) => write!(f, "-{:?}", x),
            Self::Add(a, b) => write!(f, "{:?} + {:?}", a, b),
            Self::Subtract(a, b) => write!(f, "{:?} - {:?}", a, b),
            Self::Multiply(a, b) => write!(f, "{:?} * {:?}", a, b),
            Self::Divide(a, b) => write!(f, "{:?} / {:?}", a, b),
            Self::Remainder(a, b) => write!(f, "{:?} % {:?}", a, b),

            Self::Greater(a, b) => write!(f, "{:?} > {:?}", a, b),
            Self::GreaterEqual(a, b) => write!(f, "{:?} ≥ {:?}", a, b),
            Self::Less(a, b) => write!(f, "{:?} < {:?}", a, b),
            Self::LessEqual(a, b) => write!(f, "{:?} ≤ {:?}", a, b),
            Self::Equal(a, b) => write!(f, "{:?} = {:?}", a, b),
            Self::NotEqual(a, b) => write!(f, "{:?} ≠ {:?}", a, b),

            Self::If(cond, a, b) => write!(f, "if ({:?}) {:?} else {:?}", cond, a, b),
            Self::Conditional(cond, a, b) => write!(f, "{:?}? {:?} : {:?}", cond, a, b),

            Self::Lambda(params, body, _) => {
                let mut result = String::from("fn(");
                for param in params {
                    result += &format!("{}, ", param);
                }
                if !params.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}) -> {:?}", result, body)
            }

            Self::Macro(params, body) => {
                let mut result = String::from("macro(");
                for param in params {
                    result += &format!("{}, ", param);
                }
                if !params.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}) -> {:?}", result, body)
            }

            Self::Run(program, args) => {
                write!(f, "{}'", program)?;
                for arg in args {
                    write!(f, " {:?}", arg)?;
                }
                Ok(())
            }

            Self::Apply(func, args) => {
                let mut result = format!("{:?}(", func);
                for arg in args {
                    result += &format!("{:?}, ", arg);
                }
                if !args.is_empty() {
                    result.pop();
                    result.pop();
                }
                write!(f, "{})", result)
            }

            Self::Path(path) => write!(f, "{}", path.as_path().display().to_string()),
            Self::Size(size) => write!(f, "{}", size),
            Self::Integer(x) => write!(f, "{:?}", x),
            Self::Float(x)   => write!(f, "{:?}", x),
            Self::String(x)  => write!(f, "{:?}", x),
            Self::Boolean(x) => write!(f, "{:?}", x),
            Self::Nil => write!(f, "nil"),
            Self::List(x) => {
                let mut result = String::from("[");
                for item in x {
                    result += &format!("{:?}, ", item);
                }
                if !x.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{}]", result)
            }
            Self::Table(x) => {
                let mut result = String::from("{ ");
                for (key, val) in x {
                    result += &format!("\"{}\": {:?}, ", key, val);
                }
                if !x.is_empty() {
                    result.pop();
                    result.pop();
                }

                write!(f, "{} }}", result)
            }

            Self::Error(e) => write!(f, "{}", e)
        }
    }
}