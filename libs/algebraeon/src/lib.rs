mod NN;
mod ZZ;
use koto_runtime::prelude::*;

pub fn make_module() -> KMap {
    let mut result = KMap::default();

    let mut NN = KMap::default();
    NN.insert_meta(
        MetaKey::Call,
        KNativeFunction::new(|ctx| match ctx.args() {
            [KValue::Number(n)] => Ok(NN::NN::make_koto_object(*n).into()),
            unexpected => unexpected_args("|Number|", unexpected),
        })
        .into(),
    );

    NN.insert_meta(
        MetaKey::Named("display".into()),
        KNativeFunction::new(|_ctx| Ok("NN".into())).into(),
    );

    NN.insert_meta(
        MetaKey::Named("primes".into()),
        KNativeFunction::new(|_ctx| Ok("TO DO".into())).into(),
    );
    result.insert("NN", NN);

    result.add_fn("ZZ", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok(ZZ::ZZ::make_koto_object(*n).into()),
        unexpected => unexpected_args("|Number|", unexpected),
    });

    result
}
