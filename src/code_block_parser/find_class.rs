use std::marker::Tuple;

struct Closure {
    s : String,
    t : String,
}

impl<T> FnOnce<(T,)> for Closure {
    type Output = String;

    extern "rust-call" fn call_once(self, _: (T,)) -> Self::Output {
        let x = self.s + &*self.t;
        x
    }
}