#![feature(mspc_select)]
pub use schedule_recv::oneshot_ms;

#[macro_export]
macro_rules! select2_timeout {
    ($time_ms:expr => $code:expr, $($args:tt)*) => ({
        let timeout_rx = $crate::select2::oneshot_ms($time_ms);
        select2!(
            _ = timeout_rx => $code,
            $($args)*
        )
    });
}

#[macro_export]
macro_rules! select2 {
    (@internal
        [$($parsed:tt)*]
        $name:pat = $handle:expr => $code:expr, 
        $($rest:tt)*
    ) => ({
        select2!(@internal [$($parsed)* rx rx2 $name = $handle => $code,]
                         $($rest)*)
    });
    
    (@internal
        [$($parsed:tt)*]
        $name:pat = $handle:expr => $code:expr
    ) => ({
        select2!([$($parsed)* rx rx2 $name = $handle => $code,])
    });

    (@internal
        [$($rx:ident $output:ident $name:pat = $handle:expr => $code:expr,)*]
    ) => ({
        $( let mut $output = None; )+
        {
            use std::sync::mpsc::Select;
            let sel = Select::new();
            $( let mut $rx = sel.handle(&$handle); )+
            unsafe {
                $( $rx.add(); )+
            }
            let ret = sel.wait();
            $( if ret == $rx.id() { $output = Some($rx.recv()); } )+ 
        }
        $( if let Some($name) = $output { $code } else )+
        { unreachable!() }
    });
    
    ($($args:tt)*) => ( select2!(@internal [] $($args)* ) );
}

