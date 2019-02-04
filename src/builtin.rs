use crate::eval::{StackData, VM};
use crate::value::{RefValue, Value};

pub static SYNTAX: &[(&str, fn(&mut VM) -> Result<(), String>)] = &[
    ("define", |vm| {
        match vm.pp.next().ok_or("syntax error")? {
            Value::Ident(ident) => {
                vm.stack.truncate(vm.sp as usize);
                vm.stack.push(StackData::Val(Value::Syntax("define2", |vm| {
                    if vm.pp != Value::Null {
                        return Err("syntax error".to_string());
                    }
                    if let Some(StackData::Val(value)) = vm.stack.pop() {
                        if let Some(StackData::Val(Value::Ident(ident))) = vm.stack.pop() {
                            vm.env.insert(ident, value);
                            vm.rr = Value::Bool(true);
                            vm.stack.truncate(vm.sp as usize);
                            vm.sp -= 1;
                        } else {
                            return Err("syntax error".to_string());
                        }
                    } else {
                        return Err("internal error".to_string());
                    }
                    Ok(())
                })));
                vm.stack.push(StackData::Val(Value::Ident(ident)));
            }
            Value::Cons(defun_ident, defun_args) => {
                let body = vm.pp.next().ok_or("syntax error")?;
                let _ = vm.pp.clone().try_into_nil().or(Err("syntax error"))?;
                let defun_ident = defun_ident.to_value().try_into_ident().or(Err("syntax error"))?;
                let value = Value::Closure(defun_args, RefValue::new(body), vm.env.clone());
                vm.env.insert(defun_ident, value);
                vm.rr = Value::Bool(true);
                vm.stack.truncate(vm.sp as usize);
                vm.sp -= 1;
            }
            _ => panic!("syntax error"),
        }
        Ok(())
    }),
    ("quote", |vm| {
        vm.rr = vm.pp.next().ok_or("syntax error")?;
        vm.stack.truncate(vm.sp as usize);
        vm.sp -= 1;
        Ok(())
    }),
    ("lambda", |vm| {
        let args = vm.pp.next().ok_or("syntax error")?;
        let body = vm.pp.next().ok_or("syntax error")?;
        vm.rr = Value::Closure(RefValue::new(args), RefValue::new(body), vm.env.clone());
        vm.stack.truncate(vm.sp as usize);
        vm.sp -= 1;
        Ok(())
    }),
    ("if", |vm| {
        vm.stack.truncate(vm.sp as usize);
        vm.stack.push(StackData::Val(Value::Syntax("if2", |vm| {
            if let Some(StackData::Val(Value::Bool(b))) = vm.stack.pop() {
                if !b {
                    vm.pp.next();
                }
            } else {
                return Err("syntax error".to_string());
            }
            vm.pp = vm.pp.next().unwrap_or(Value::Null);
            vm.stack.truncate(vm.sp as usize);
            Ok(())
        })));
        Ok(())
    }),
    ("call/cc", |vm| {
        vm.stack.truncate(vm.sp as usize);
        vm.stack
            .push(StackData::Val(Value::Syntax("call/cc2", |vm| {
                let cont = Value::Cont(Box::new(vm.clone()));
                let lambda = vm.stack.pop().ok_or("internal error")?;
                if let StackData::Val(Value::Closure(_, _, _)) = lambda {
                } else {
                    return Err("syntax error".to_string());
                }
                vm.stack.pop();
                vm.stack.push(lambda);
                vm.stack.push(StackData::Val(cont));
                Ok(())
            })));
        Ok(())
    }),
    ("print-env", |vm| {
        vm.env.print();
        vm.stack.pop();
        vm.rr = Value::Null;
        vm.sp -= 1;
        Ok(())
    }),
];

pub static SUBR: &[(&str, fn(&mut dyn Iterator<Item = Value>) -> Result<Value, String>)] = &[
    ("cons", |args| {
        let car = args.next().ok_or("syntax error")?;
        let cdr = args.next().ok_or("syntax error")?;
        Ok(Value::Cons(RefValue::new(car), RefValue::new(cdr)))
    }),
    ("car", |args| {
        let cons = args.next().ok_or("syntax error")?.try_into_cons()?;
        Ok(cons.0)
    }),
    ("cdr", |args| {
        let cons = args.next().ok_or("syntax error")?.try_into_cons()?;
        Ok(cons.1)
    }),
    ("eqv?", |args| {
        let first = args.next().ok_or("syntax error")?;
        for v in args {
            if v != first {
                return Ok(Value::Bool(false));
            }
        }
        Ok(Value::Bool(true))
    }),
    ("=", |args| {
        let first = args.next().ok_or("syntax error")?;
        for val in args {
            if first != val {
                return Ok(Value::Bool(false));
            }
        }
        Ok(Value::Bool(true))
    }),
    ("+", |args| {
        let mut acc = 0.0;
        for val in args {
            acc += val.try_into_num()?;
        }
        Ok(Value::Num(acc))
    }),
    ("-", |args| {
        let mut acc = args.next().ok_or("syntax error")?.try_into_num()?;
        for val in args {
            acc -= val.try_into_num()?;
        }
        Ok(Value::Num(acc))
    }),
    ("*", |args| {
        let mut acc = 1.0;
        for val in args {
            acc *= val.try_into_num()?;
        }
        Ok(Value::Num(acc))
    }),
    ("/", |args| {
        let mut acc = args.next().ok_or("syntax error")?.try_into_num()?;
        for val in args {
            acc /= val.try_into_num()?;
        }
        Ok(Value::Num(acc))
    }),
    ("print", |args| {
        for val in args {
            println!("{:?}", val);
        }
        Ok(Value::Bool(true))
    }),
];
