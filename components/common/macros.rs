use std::sync::mpsc::{channel};

#[derive(Debug)]
pub enum ChanError {
    SendError(&'static str),
    RecvError(&'static str),
}

#[macro_export]
macro_rules! send {
    ($sender:expr, $path:path => ( $($arg:expr),* )) => {{
        let data = $path($($arg),*);
        $sender.send(data)
            .map_err(|e| {
                $crate::ChanError::SendError(stringify!($path))
            })
    }}
}

#[macro_export]
macro_rules! req_rep {
    ($sender:expr, $path:path => ( $($arg:expr),* )) => {{
        let (tx, rx) = channel();
        let data = $path(tx, $($arg),*);
        let path_str = stringify!($path);
        let ret = $sender.send(data)
            .map_err(|_e| {
                $crate::ChanError::SendError(stringify!($path))
            });
        let finalres: ::std::result::Result<_, $crate::ChanError>;
        if ret.is_err() {
            finalres = Err(ret.unwrap_err());
        } else {
            finalres = rx.recv()
            .map_err(|_e| {
                $crate::ChanError::RecvError(stringify!($path))
            });
        }
        finalres
    }}
}
