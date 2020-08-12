#[macro_export]
macro_rules! call_mod_througth_bus {
    ($bus_addr: ident, $mod_name: expr, $fn_name: expr, $param: expr) => {
        $bus_addr
            .send(ewf_core::CallQuery {
                module: $mod_name.to_string(),
            })
            .await??
            .send(ewf_core::Call {
                method: $fn_name.to_string(),
                args: $param,
            })
            .await??
    };
}

#[macro_export]
macro_rules! call_self {
    ($self_addr: ident, $fn_name: expr, $param: expr) => {
        $self_addr
            .send(ewf_core::Call {
                method: $fn_name.to_string(),
                args: $param,
            })
            .await??
    };
}

#[macro_export]
macro_rules! async_parse_check {
    ($input: expr, $err: expr) => {
        match serde_json::from_value($input) {
            Ok(ans) => ans,
            Err(_) => return Err($err),
        }
    };
}

#[macro_export]
macro_rules! sync_parse_check {
    ($input: expr, $err: expr) => {
        match serde_json::from_value($input) {
            Ok(ans) => ans,
            Err(_) => return Box::pin(async move { Err($err) }),
        }
    };
}

#[macro_export]
macro_rules! sync_ewf_error {
    ($expression: expr) => {
        match $expression {
            Ok(ans) => ans,
            Err(err) => return Box::pin(async move { Err(err.to_ewf_error()) }),
        }
    };
}

#[macro_export]
macro_rules! sync_ewf_error_spec {
    ($expression: expr, $err: expr) => {
        match $expression {
            Ok(ans) => ans,
            Err(_) => return Box::pin(async move { Err($err.to_ewf_error()) }),
        }
    };
}

#[macro_export]
macro_rules! async_ewf_error_spec {
    ($expression: expr, $err: expr) => {
        match $expression {
            Ok(ans) => ans,
            Err(_) => return Err($err.to_ewf_error()),
        }
    };
}
