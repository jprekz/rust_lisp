use crate::eval::VM;
use crate::value::{RefValue, Value};

fn define_syntax(vm: &mut VM) -> Result<(), String> {
    match vm.pop_pp().ok_or("syntax error")? {
        // (define ident value)
        Value::Ident(ident) => {
            vm.truncate_stack();
            vm.eval_then("define2", |vm| {
                if vm.pop_pp() != None {
                    return Err("syntax error".to_string());
                }
                let value = vm.pop_value()?;
                let ident = vm.pop_value()?.try_into_ident()?;
                vm.define(ident, value);
                vm.ret(Value::Bool(true))
            });
            vm.push_value(Value::Ident(ident));
            Ok(())
        }
        // (define (defun_ident defun_args...) body)
        Value::Cons(defun_ident, defun_args) => {
            let body = vm.pop_pp().ok_or("syntax error")?;
            if vm.pop_pp() != None {
                return Err("syntax error".to_string());
            }
            let defun_ident = defun_ident
                .to_value()
                .try_into_ident()
                .or(Err("syntax error"))?;
            let value = vm.new_closure(defun_args, RefValue::new(body));
            vm.define(defun_ident, value);
            vm.ret(Value::Bool(true))
        }
        _ => panic!("syntax error"),
    }
}

fn quote_syntax(vm: &mut VM) -> Result<(), String> {
    let quoted = vm.pop_pp().ok_or("syntax error")?;
    vm.ret(quoted)
}

fn lambda_syntax(vm: &mut VM) -> Result<(), String> {
    let args = vm.pop_pp().ok_or("syntax error")?;
    let body = vm.pop_pp().ok_or("syntax error")?;
    vm.ret(vm.new_closure(RefValue::new(args), RefValue::new(body)))
}

fn if_syntax(vm: &mut VM) -> Result<(), String> {
    vm.truncate_stack();
    vm.eval_then("if2", |vm| {
        let test = vm.pop_value()?.try_into_bool()?;
        let then_expr = vm.pop_pp().ok_or("syntax error")?;
        let else_expr = vm.pop_pp().unwrap_or(Value::Null);
        if test {
            vm.set_pp(then_expr);
        } else {
            vm.set_pp(else_expr);
        }
        vm.truncate_stack();
        Ok(())
    });
    Ok(())
}

fn call_cc_syntax(vm: &mut VM) -> Result<(), String> {
    vm.truncate_stack();
    vm.eval_then("call/cc2", |vm| {
        let cont = Value::Cont(Box::new(vm.clone()));
        let lambda = vm.pop_value()?;
        if let Value::Closure(_, _, _) = lambda {
        } else {
            return Err("syntax error".to_string());
        }
        vm.pop_value()?; // pop 'call/cc ?
        vm.push_value(lambda);
        vm.push_value(cont);
        Ok(())
    });
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
    vm.print_env();
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
