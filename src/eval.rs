use crate::env::Env;
use crate::value::Value;

#[derive(Clone, Debug)]
pub enum StackData {
    Frame(i64, Value),
    Val(Value),
    Env(Env),
}

#[derive(Clone)]
pub struct VM {
    pub pp: Value,
    pub sp: i64,
    pub rr: Value,
    pub stack: Vec<StackData>,
    pub env: Env,
}

pub fn eval(val: Value, env: Env, debug_mode: bool) -> Result<Value, String> {
    let mut vm = VM {
        pp: val,
        sp: 0i64,
        rr: Value::Null,
        stack: Vec::new(),
        env: env,
    };

    if debug_mode {
        use std::mem::size_of;
        eprintln!("[DEBUG] size of StackData: {:?}", size_of::<StackData>());
        eprintln!("[DEBUG] size of Value: {:?}", size_of::<Value>());
        eprintln!("[DEBUG] size of Env: {:?}", size_of::<Env>());
    }

    loop {
        if debug_mode {
            eprintln!(
                "[DEBUG] env:{:?}\tsp:{}\tpp:{:?}\trr:{:?}\n stack: {:?}",
                vm.env, vm.sp, vm.pp, vm.rr, vm.stack
            );
        }

        match vm.pp.clone() {
            Value::Null => {
                if vm.sp == vm.stack.len() as i64 {
                    vm.rr = Value::Null;
                    vm.sp -= 1;
                    continue;
                }
                if vm.sp < 0 {
                    return Ok(vm.rr);
                }
                match vm.stack[vm.sp as usize].clone() {
                    StackData::Val(Value::Closure(closure_args, closure_body, closure_env)) => {
                        let extended_env = closure_env.extend();
                        for (i, closure_arg) in closure_args.to_value().enumerate() {
                            let ident = closure_arg.try_into_ident().or(Err("syntax error".to_string()))?;
                            if let StackData::Val(value) = vm.stack[vm.sp as usize + 1 + i].clone()
                            {
                                extended_env.insert(ident, value);
                            } else {
                                return Err("internal error".to_string());
                            }
                        }
                        vm.stack.truncate(vm.sp as usize);
                        vm.pp = closure_body.to_value();
                        if vm.sp > 0 {
                            if let StackData::Env(_) = vm.stack[vm.sp as usize - 1] {
                                vm.env = extended_env;
                                continue;
                            }
                        }
                        vm.stack.push(StackData::Env(vm.env.clone()));
                        vm.env = extended_env;
                        vm.sp += 1;
                    }
                    StackData::Val(Value::Syntax(_name, f)) => {
                        f(&mut vm)?;
                    }
                    StackData::Val(Value::Subr(_name, f)) => {
                        let mut args = Vec::new();
                        for d in &vm.stack[vm.sp as usize + 1..] {
                            if let StackData::Val(v) = d {
                                args.push(v.clone());
                            } else {
                                return Err("internal error".to_string());
                            }
                        }
                        vm.rr = f(&mut args.into_iter())?;
                        vm.stack.truncate(vm.sp as usize);
                        vm.sp -= 1;
                    }
                    StackData::Val(Value::Cont(box_vm)) => {
                        if let Some(StackData::Val(arg)) = vm.stack.pop() {
                            vm = *box_vm;
                            vm.rr = arg;
                            vm.stack.truncate(vm.sp as usize);
                            vm.sp -= 1;
                        } else {
                            return Err("internal error".to_string());
                        }
                    }
                    StackData::Val(_) => {
                        return Err("invalid application".to_string());
                    }
                    StackData::Frame(sp, pp) => {
                        vm.pp = pp;
                        vm.sp = sp;
                        vm.stack.pop();
                        vm.stack.push(StackData::Val(vm.rr.clone()));
                        if let StackData::Val(Value::Syntax(_name, f)) =
                            vm.stack[vm.sp as usize].clone()
                        {
                            f(&mut vm)?;
                        }
                    }
                    StackData::Env(e) => {
                        vm.stack.pop();
                        vm.env = e;
                        vm.sp -= 1;
                    }
                }
            }
            Value::Cons(car, cdr) => {
                vm.stack.push(StackData::Frame(vm.sp, cdr.to_value()));
                vm.sp = vm.stack.len() as i64;
                vm.pp = car.to_value();
            }
            Value::Ident(ident) => {
                vm.rr = vm.env.get(ident.clone()).ok_or("unbound variable")?.clone();
                vm.pp = Value::Null;
                vm.sp -= 1;
            }
            other => {
                vm.rr = other;
                vm.pp = Value::Null;
                vm.sp -= 1;
            }
        }
    }
}
