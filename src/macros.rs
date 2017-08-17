#[macro_escape]
macro_rules! errno {
        ($s:expr) => { if $s == -1 { errno!() } else { Ok($s) }};
        () => { Err(format!("errno: {}", errno::errno()).into()) }
}
