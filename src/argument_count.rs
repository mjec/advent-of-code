#[allow(unused_imports)]
use return_codes;

macro_rules! require_exactly_arguments {
    ( $args:ident, $count:expr, $help:path ) => {
        let argument_count = $args.len();
        if argument_count != $count {
            return match $help($args) {
                Ok(()) => Err(match argument_count {
                    x if x > $count => return_codes::TOO_MANY_ARGUMENTS,
                    _ => return_codes::NOT_ENGOUH_ARGUMENTS,
                }),
                e => e,
            };
        }
    };
}
