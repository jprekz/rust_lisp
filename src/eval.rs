use super::env::Env;
use super::value::Value;

#[derive(Clone, Debug)]
pub enum StackData {
    Val(Value),
    PP(Value),
    SP(i64),
    Env(Env),
}
pub type Stack = Vec<StackData>;
pub struct VM {
    pub pp: Value,
    pub sp: i64,
    pub rr: Value,
    pub stack: Stack,
    pub env: Env,
}

pub fn eval(val: Value, env: Env) -> Value {
    println!("eval start");
    let mut vm = VM {
        pp: val,
        sp: 0i64,
        rr: Value::Nil,
        stack: Vec::new(),
        env: env,
    };
    loop {
        println!("rr:{:?} sp:{} pp:{:?} {:?}", vm.rr, vm.sp, vm.pp, vm.stack);
        match vm.pp.clone() {
            Value::Nil => {
                if vm.sp == vm.stack.len() as i64 {
                    vm.rr = Value::Nil;
                    vm.sp -= 1;
                    continue;
                }
                if vm.sp < 0 {
                    return vm.rr;
                }
                match vm.stack[vm.sp as usize].clone() {
                    StackData::Val(Value::Closure(closure_args, closure_body, closure_env)) => {
                        let mut extended_env = closure_env.extend();
                        for (i, closure_arg) in closure_args.to_value().enumerate() {
                            let ident = closure_arg.try_into_ident().expect("syntax error");
                            if let StackData::Val(value) = vm.stack[vm.sp as usize + 1 + i].clone() {
                                extended_env.insert(ident, value);
                            } else {
                                panic!();
                            }
                        }
                        while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
                        vm.stack.push(StackData::Env(vm.env.clone()));
                        vm.sp += 1;
                        vm.pp = closure_body.to_value();
                        vm.env = extended_env;
                    }
                    StackData::Val(Value::Syntax(name, f)) => {
                        println!("  exec syntax {}", name);
                        f(&mut vm);
                    }
                    StackData::Val(Value::Subr(_name, f)) => {
                        vm.rr = f(&mut vm.stack[vm.sp as usize + 1 ..].iter().map(|d| {
                            if let StackData::Val(v) = d { v.clone() } else { panic!() }
                        }));
                        while vm.stack.len() > vm.sp as usize { vm.stack.pop(); }
                        vm.sp -= 1;
                    }
                    StackData::Val(_) => {
                        panic!("invalid application");
                    }
                    StackData::PP(v) => {
                        vm.stack.pop();
                        vm.pp = v;
                        vm.sp -= 1;
                        if let Some(StackData::SP(v)) = vm.stack.pop() {
                            vm.sp = v;
                            vm.stack.push(StackData::Val(vm.rr.clone()));
                            if let StackData::Val(Value::Syntax(name, f)) = vm.stack[vm.sp as usize].clone() {
                                println!("  exec syntax {}", name);
                                f(&mut vm);
                            }
                        } else {
                            unreachable!();
                        }
                    }
                    StackData::SP(_) => {
                        unreachable!();
                    }
                    StackData::Env(e) => {
                        vm.stack.pop();
                        vm.env = e;
                        vm.sp -= 1;
                    }
                }
            }
            Value::Cons(car, cdr) => {
                vm.stack.push(StackData::SP(vm.sp));
                vm.stack.push(StackData::PP(cdr.to_value()));
                vm.sp = vm.stack.len() as i64;
                vm.pp = car.to_value();
            }
            Value::Quoted(value) => {
                vm.rr = value.to_value();
                vm.pp = Value::Nil;
                vm.sp -= 1;
            }
            Value::Ident(ident) => {
                vm.rr = vm.env.get(ident.clone()).expect("unbound variable");
                vm.pp = Value::Nil;
                vm.sp -= 1;
            }
            other => {
                vm.rr = other;
                vm.pp = Value::Nil;
                vm.sp -= 1;
            }
        }
    }
}

