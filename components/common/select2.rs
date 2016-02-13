#![feature(mspc_select)]

macro_rules! as_expr { ($x:expr) => ($x) }

#[macro_export]
macro_rules! select_internal {
    (
        [$($parsed:tt)*]
        $name:pat = $handle:expr => $code:expr, 
        $($rest:tt)*
    ) => ({
        select_internal!([$($parsed)* rx $name = $handle => $code,]
                         $($rest)*)
    });
    
    (
        [$($parsed:tt)*]
        $name:pat = $handle:expr => $code:expr
    ) => ({
        select_internal!([$($parsed)* rx $name = $handle => $code,])
    });

    ([$($rx:ident $name:pat = $handle:expr => $code:expr,)*]) => ({
        use std::sync::mpsc::Select;
        let sel = Select::new();
        $( let mut $rx = sel.handle(&$handle); )+
        unsafe {
            $( $rx.add(); )+
        }
        let ret = sel.wait();
        $( if ret == $rx.id() { let $name = $rx.recv(); $code } else )+
        { unreachable!() }
    });
}
#[macro_export]
macro_rules! select2 {
    ($($args:tt)*) => ( select_internal!([] $($args)* ) )
}
