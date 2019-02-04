use crate::eval::{StackData, VM};
use crate::value::{RefValue, Value};

pub static SYNTAX: &[(&str, fn(&mut VM))] = &[
    ("define", |vm| match vm.pp.next().unwrap() {
        Value::Ident(ident) => {
            vm.stack.truncate(vm.sp as usize);
            vm.stack.push(StackData::Val(Value::Syntax("define2", |vm| {
                if let StackData::Val(value) = vm.stack.pop().unwrap() {
                    if let StackData::Val(Value::Ident(ident)) = vm.stack.pop().unwrap() {
                        vm.env.insert(ident, value);
                        vm.rr = Value::Bool(true);
                        vm.stack.truncate(vm.sp as usize);
                        vm.sp -= 1;
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            })));
            vm.stack.push(StackData::Val(Value::Ident(ident)));
        }
        Value::Cons(defun_ident, defun_args) => {
            let body = vm.pp.next().unwrap();
            let _ = vm.pp.clone().try_into_nil().unwrap();
            let defun_ident = defun_ident.to_value().try_into_ident().unwrap();
            let value = Value::Closure(defun_args, RefValue::new(body), vm.env.clone());
            vm.env.insert(defun_ident, value);
            vm.rr = Value::Bool(true);
            vm.stack.truncate(vm.sp as usize);
            vm.sp -= 1;
        }
        _ => panic!("syntax error"),
    }),
    ("quote", |vm| {
        vm.rr = vm.pp.next().unwrap();
        vm.stack.truncate(vm.sp as usize);
        vm.sp -= 1;
    }),
    ("lambda", |vm| {
        let args = vm.pp.next().unwrap();
        let body = vm.pp.next().unwrap();
        vm.rr = Value::Closure(RefValue::new(args), RefValue::new(body), vm.env.clone());
        vm.stack.truncate(vm.sp as usize);
        vm.sp -= 1;
    }),
    ("if", |vm| {
        vm.stack.truncate(vm.sp as usize);
        vm.stack.push(StackData::Val(Value::Syntax("if2", |vm| {
            if let StackData::Val(Value::Bool(false)) = vm.stack.pop().unwrap() {
                vm.pp.next();
            }
            vm.pp = vm.pp.next().unwrap();
            vm.stack.truncate(vm.sp as usize);
        })));
    }),
    ("call/cc", |vm| {
        vm.stack.truncate(vm.sp as usize);
        vm.stack
            .push(StackData::Val(Value::Syntax("call/cc2", |vm| {
                let cont = Value::Cont(Box::new(vm.clone()));
                let lambda = vm.stack.pop().unwrap();
                vm.stack.pop();
                vm.stack.push(lambda);
                vm.stack.push(StackData::Val(cont));
            })));
    }),
    ("print-env", |vm| {
        vm.env.print();
        vm.stack.pop();
        vm.rr = Value::Null;
        vm.sp -= 1;
    }),
];

// TODO: error handling
pub static SUBR: &[(&str, fn(&mut dyn Iterator<Item = Value>) -> Value)] = &[
    ("cons", |args| {
        let car = args.next().unwrap();
        let cdr = args.next().unwrap();
        Value::Cons(RefValue::new(car), RefValue::new(cdr))
    }),
    ("car", |args| {
        let cons = args.next().unwrap().try_into_cons().unwrap();
        cons.0
    }),
    ("cdr", |args| {
        let cons = args.next().unwrap().try_into_cons().unwrap();
        cons.1
    }),
    ("=", |args| {
        let first = args.next().unwrap();
        for val in args {
            if first != val {
                return Value::Bool(false);
            }
        }
        Value::Bool(true)
    }),
    ("+", |args| {
        let mut acc = 0.0;
        for val in args {
            acc += val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("-", |args| {
        let mut acc = args.next().unwrap().try_into_num().unwrap();
        for val in args {
            acc -= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("*", |args| {
        let mut acc = 1.0;
        for val in args {
            acc *= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("/", |args| {
        let mut acc = args.next().unwrap().try_into_num().unwrap();
        for val in args {
            acc /= val.try_into_num().unwrap();
        }
        Value::Num(acc)
    }),
    ("print", |args| {
        for val in args {
            println!("{:?}", val);
        }
        Value::Bool(true)
    }),
];
