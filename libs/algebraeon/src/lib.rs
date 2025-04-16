mod NN;
mod ZZ;
use koto_runtime::{Result, prelude::*};

pub fn make_module() -> KMap {
    use KValue::{Number, Str};

    let mut result = KMap::default();

    result.add_fn("NN", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok(NN::NN::make_koto_object(*n).into()),
        unexpected => unexpected_args("|Number|", unexpected),
    });

    result.add_fn("ZZ", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok(ZZ::ZZ::make_koto_object(*n).into()),
        unexpected => unexpected_args("|Number|", unexpected),
    });

    result
}
