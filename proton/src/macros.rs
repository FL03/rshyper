/*
    Appellation: macros <module>
    Contrib: @FL03
*/

#[macro_export]
macro_rules! ruleset {
    ($(($state:expr, $symbol:expr) -> $dir:ident($next_state:expr, $next_symbol:expr)),* $(,)?) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert(
                    rstm::Head::new(rstm::State($state), $symbol), 
                    rstm::Tail::new(rstm::Direction::$dir, rstm::State($next_state), $next_symbol)
                );
            )*
            map
        }
    };
}
