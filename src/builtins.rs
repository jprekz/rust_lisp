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
                        vm.ret(Value::Bool(true))
                    } else {
                        return Err("syntax error".to_string());
                    }
                } else {
                    return Err("internal error".to_string());
                }
            })));
            vm.stack.push(StackData::Val(Value::Ident(ident)));
            Ok(())
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
            vm.ret(Value::Bool(true))
        }
        _ => panic!("syntax error"),
    }
}

fn quote_syntax(vm: &mut VM) -> Result<(), String> {
    let quoted = vm.pp.next().ok_or("syntax error")?;
    vm.ret(quoted)
}

fn lambda_syntax(vm: &mut VM) -> Result<(), String> {
    let args = vm.pp.next().ok_or("syntax error")?;
    let body = vm.pp.next().ok_or("syntax error")?;
    vm.ret(Value::Closure(
        RefValue::new(args),
        RefValue::new(body),
        vm.env.clone(),
    ))
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

fn cons_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let car = args.next().ok_or("syntax error")??;
    let cdr = args.next().ok_or("syntax error")??;
    std::mem::drop(args);
    vm.ret(Value::Cons(RefValue::new(car), RefValue::new(cdr)))
}

fn car_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let cons = args.next().ok_or("syntax error")??.try_into_cons()?;
    std::mem::drop(args);
    vm.ret(cons.0)
}

fn cdr_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let cons = args.next().ok_or("syntax error")??.try_into_cons()?;
    std::mem::drop(args);
    vm.ret(cons.1)
}

fn eqv_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let first = args.next().ok_or("syntax error")??;
    let mut result = true;
    for val in args {
        if first != val? {
            result = false;
            break;
        }
    }
    vm.ret(Value::Bool(result))
}

fn equal_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let first = args.next().ok_or("syntax error")??;
    let mut result = true;
    for val in args {
        if first != val? {
            result = false;
            break;
        }
    }
    vm.ret(Value::Bool(result))
}

fn plus_subr(vm: &mut VM) -> Result<(), String> {
    let mut acc = 0.0;
    for val in vm.args() {
        acc += val?.try_into_num()?;
    }
    vm.ret(Value::Num(acc))
}

fn minus_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let mut acc = args.next().ok_or("syntax error")??.try_into_num()?;
    for val in args {
        acc -= val?.try_into_num()?;
    }
    vm.ret(Value::Num(acc))
}

fn multiply_subr(vm: &mut VM) -> Result<(), String> {
    let mut acc = 1.0;
    for val in vm.args() {
        acc *= val?.try_into_num()?;
    }
    vm.ret(Value::Num(acc))
}

fn divide_subr(vm: &mut VM) -> Result<(), String> {
    let mut args = vm.args();
    let mut acc = args.next().ok_or("syntax error")??.try_into_num()?;
    for val in args {
        acc /= val?.try_into_num()?;
    }
    vm.ret(Value::Num(acc))
}

fn print_subr(vm: &mut VM) -> Result<(), String> {
    for val in vm.args() {
        println!("{:?}", val?);
    }
    vm.ret(Value::Bool(true))
}

fn print_env_subr(vm: &mut VM) -> Result<(), String> {
    vm.env.print();
    vm.ret(Value::Bool(true))
}

pub static SYNTAX: &[(&str, fn(&mut VM) -> Result<(), String>)] = &[
    ("define", define_syntax),
    ("quote", quote_syntax),
    ("lambda", lambda_syntax),
    ("if", if_syntax),
    ("call/cc", call_cc_syntax),
];

pub static SUBR: &[(&str, fn(&mut VM) -> Result<(), String>)] = &[
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
    ("print-env", print_env_subr),
];
