extern crate nodrop_union;
use nodrop_union::NoDrop;

#[macro_export]
macro_rules! scope_exit {
    ($($t:tt)*) => (
        let _guard = $crate::ScopeExitGuard::new(|| { $($t)* });
    )
}

#[macro_export]
macro_rules! scope_exit_move {
    ($($t:tt)*) => (
        let _guard = $crate::ScopeExitGuard::new(move || { $($t)* });
    )
}

#[macro_export]
macro_rules! scope_success {
    ($($t:tt)*) => (
        let _guard = $crate::ScopeSuccessGuard::new(|| { $($t)* });
    )
}

#[macro_export]
macro_rules! scope_success_move {
    ($($t:tt)*) => (
        let _guard = $crate::ScopeSuccessGuard::new(move || { $($t)* });
    )
}

#[macro_export]
macro_rules! scope_failure {
    ($($t:tt)*) => (
        let _guard = $crate::ScopeFailureGuard::new(|| { $($t)* });
    )
}

#[macro_export]
macro_rules! scope_failure_move {
    ($($t:tt)*) => (
        let _guard = $crate::ScopeFailureGuard::new(move || { $($t)* });
    )
}

macro_rules! impl_scope_guard {
    ($name:ident, $condition:expr) => (
        pub struct $name<F: FnOnce()> {
            func: NoDrop<F>,
        }

        impl<F: FnOnce()> $name<F> {
            pub fn new(f: F) -> Self {
                $name { func: NoDrop::new(f) }
            }
        }

        impl<F: FnOnce()> Drop for $name<F> {
            fn drop(&mut self) {
                // SAFE: This is safe because `self.func` is wrapped in `NoDrop`, so the drop glue
                // that executes after this function will not try to drop that field.
                //
                // We also don't leak, since `self.func` is moved onto the local stack and will
                // either be consumed by the following conditional call or dropped implicitly.
                let func = unsafe { ::std::ptr::read(&self.func).into_inner() };
                if $condition { (func)(); }
            }
        }
    )
}

impl_scope_guard!(ScopeExitGuard, true);
impl_scope_guard!(ScopeSuccessGuard, !::std::thread::panicking());
impl_scope_guard!(ScopeFailureGuard, ::std::thread::panicking());

#[cfg(test)]
mod tests {
    fn catch_panics<F: FnOnce()>(f: F) {
        use std::panic::AssertUnwindSafe;
        use std::panic::catch_unwind;
        let _ = catch_unwind(AssertUnwindSafe(f));
    }

    #[test]
    fn scope_exit_when_it_succeeds() {
        let mut worked = false;
        catch_panics(|| {
            scope_exit! { worked = true; }
            /* Sucess (no panics). */
        });
        assert!(worked);
    }

    #[test]
    fn scope_exit_when_it_panics() {
        let mut worked = false;
        catch_panics(|| {
            scope_exit! { worked = true; }
            panic!();
        });
        assert!(worked);
    }

    #[test]
    fn scope_success_when_it_succeeds() {
        let mut worked = false;
        catch_panics(|| {
            scope_success! { worked = true; }
            /* Sucess (no panics). */
        });
        assert!(worked);
    }

    #[test]
    fn scope_success_when_it_panics() {
        let mut worked = true;
        catch_panics(|| {
            scope_success! { worked = false; }
            panic!();
        });
        assert!(worked);
    }

    #[test]
    fn scope_failure_when_it_succeeds() {
        let mut worked = true;
        catch_panics(|| {
            scope_failure! { worked = false; }
            /* Sucess (no panics). */
        });
        assert!(worked);
    }

    #[test]
    fn scope_failure_when_it_panics() {
        let mut worked = false;
        catch_panics(|| {
            scope_failure! { worked = true; }
            panic!();
        });
        assert!(worked);
    }
}
