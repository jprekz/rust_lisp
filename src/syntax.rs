use super::eval::{StackData, VM};
use super::value::{RefValue, Value};

pub static SYNTAX: &'static [(&'static str, fn(&mut VM))] = &[
    ("define", |vm| match vm.pp.next().unwrap() {
        Value::Ident(ident) => {
            while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
            vm.stack.push(StackData::Val(Value::Syntax("define2", |vm| {
                if let StackData::Val(value) = vm.stack.pop().unwrap() {
                    if let StackData::Val(Value::Ident(ident)) = vm.stack.pop().unwrap() {
                        vm.env.insert(ident, value);
                        vm.rr = Value::Bool(true);
                        while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
                        vm.sp -= 1;
                    } else { panic!() }
                } else { panic!() }
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
            while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
            vm.sp -= 1;
        }
        _ => panic!("syntax error"),
    }),
    ("quote", |vm| {
        vm.pp.next().unwrap();
    }),
    ("lambda", |vm| {
        let args = vm.pp.next().unwrap();
        let body = vm.pp.next().unwrap();
        vm.rr = Value::Closure(RefValue::new(args), RefValue::new(body), vm.env.clone());
        while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
        vm.sp -= 1;
    }),
    ("if", |vm| {
        while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
        vm.stack.push(StackData::Val(Value::Syntax("if2", |vm| {
            if let StackData::Val(Value::Bool(false)) = vm.stack.pop().unwrap() {
                vm.pp.next();
            }
            vm.pp = vm.pp.next().unwrap();
            while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
        })));
    }),
    ("call/cc", |vm| {
        while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
        vm.stack.push(StackData::Val(Value::Syntax("call/cc2", |vm| {
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
    })
];

pub static SUBR: &'static [(&'static str, fn(&mut Iterator<Item = Value>) -> Value)] = &[
    ("cons", |args| {
        let car = args.next().unwrap();
        let cdr = args.next().unwrap();
        Value::Cons(RefValue::new(car), RefValue::new(cdr))
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
];
