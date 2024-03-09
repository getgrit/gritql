#[macro_export]
macro_rules! resolve {
    ( $x:expr ) => {{
        if let Some(x) = $x {
            x
        } else {
            return Ok(false);
        }
    }};
}

#[macro_export]
macro_rules! resolve_opt {
    ( $x:expr ) => {{
        if let Some(x) = $x {
            x
        } else {
            return Ok(None);
        }
    }};
}
