use crate::eval::{StackData, VM};
use crate::value::{RefValue, Value};

fn define_syntax(vm: &mut VM) -> Result<(), String> {
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
            let defun_ident = defun_ident
                .to_value()
                .try_into_ident()
                .or(Err("syntax error"))?;
            let value = Value::Closure(defun_args, RefValue::new(body), vm.env.clone());
            vm.env.insert(defun_ident, value);
            vm.rr = Value::Bool(true);
            vm.stack.truncate(vm.sp as usize);
            vm.sp -= 1;
        }
        _ => panic!("syntax error"),
    }
    Ok(())
}

fn quote_syntax(vm: &mut VM) -> Result<(), String> {
    vm.rr = vm.pp.next().ok_or("syntax error")?;
    vm.stack.truncate(vm.sp as usize);
    vm.sp -= 1;
    Ok(())
}

fn lambda_syntax(vm: &mut VM) -> Result<(), String> {
    let args = vm.pp.next().ok_or("syntax error")?;
    let body = vm.pp.next().ok_or("syntax error")?;
    vm.rr = Value::Closure(RefValue::new(args), RefValue::new(body), vm.env.clone());
    vm.stack.truncate(vm.sp as usize);
    vm.sp -= 1;
    Ok(())
}

fn if_syntax(vm: &mut VM) -> Result<(), String> {
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
}

fn call_cc_syntax(vm: &mut VM) -> Result<(), String> {
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
}

fn print_env_syntax(vm: &mut VM) -> Result<(), String> {
    vm.env.print();
    vm.stack.pop();
    vm.rr = Value::Null;
    vm.sp -= 1;
    Ok(())
}

pub static SYNTAX: &[(&str, fn(&mut VM) -> Result<(), String>)] = &[
    ("define", define_syntax),
    ("quote", quote_syntax),
    ("lambda", lambda_syntax),
    ("if", if_syntax),
    ("call/cc", call_cc_syntax),
    ("print-env", print_env_syntax),
];

fn cons_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let car = args.next().ok_or("syntax error")?;
    let cdr = args.next().ok_or("syntax error")?;
    Ok(Value::Cons(RefValue::new(car), RefValue::new(cdr)))
}

fn car_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let cons = args.next().ok_or("syntax error")?.try_into_cons()?;
    Ok(cons.0)
}

fn cdr_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let cons = args.next().ok_or("syntax error")?.try_into_cons()?;
    Ok(cons.1)
}

fn eqv_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let first = args.next().ok_or("syntax error")?;
    for v in args {
        if v != first {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn equal_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let first = args.next().ok_or("syntax error")?;
    for val in args {
        if first != val {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn plus_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let mut acc = 0.0;
    for val in args {
        acc += val.try_into_num()?;
    }
    Ok(Value::Num(acc))
}

fn minus_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let mut acc = args.next().ok_or("syntax error")?.try_into_num()?;
    for val in args {
        acc -= val.try_into_num()?;
    }
    Ok(Value::Num(acc))
}

fn multiply_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let mut acc = 1.0;
    for val in args {
        acc *= val.try_into_num()?;
    }
    Ok(Value::Num(acc))
}

fn divide_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    let mut acc = args.next().ok_or("syntax error")?.try_into_num()?;
    for val in args {
        acc /= val.try_into_num()?;
    }
    Ok(Value::Num(acc))
}

fn print_subr(args: &mut dyn Iterator<Item = Value>) -> Result<Value, String> {
    for val in args {
        println!("{:?}", val);
    }
    Ok(Value::Bool(true))
}

pub static SUBR: &[(
    &str,
    fn(&mut dyn Iterator<Item = Value>) -> Result<Value, String>,
)] = &[
    ("cons", cons_subr),
    ("car", car_subr),
    ("cdr", cdr_subr),
    ("eqv?", eqv_subr),
    ("=", equal_subr),
    ("+", plus_subr),
    ("-", minus_subr),
    ("*", multiply_subr),
    ("/", divide_subr),
    ("print", print_subr),
];
