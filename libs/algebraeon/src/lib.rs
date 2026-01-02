mod NN;
mod ZZ;
use koto_runtime::prelude::*;

pub fn version_string() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn make_module() -> KMap {
    let result = KMap::default();

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
        MetaKey::UnaryOp(UnaryOp::Display),
        KNativeFunction::new(|_ctx| Ok("NN".into())).into(),
    );
    NN.add_fn("primes", |_ctx| Ok("TO DO".into()));

    result.insert("NN", NN);

    result.add_fn("ZZ", |ctx| match ctx.args() {
        [KValue::Number(n)] => Ok(ZZ::ZZ::make_koto_object(*n).into()),
        unexpected => unexpected_args("|Number|", unexpected),
    });

    result.add_fn("gcd", |ctx| match ctx.args() {
        [KValue::Object(n), KValue::Object(m)] => {
            // Check if both objects are NN::NN
            if let (Ok(nn_n), Ok(nn_m)) = (n.cast::<crate::NN::NN>(), m.cast::<crate::NN::NN>()) {
                // Call the gcd function on the Natural values
                let result_natural = algebraeon::nzq::gcd(nn_n.0.clone(), nn_m.0.clone());
                let result_nn = KObject::from(crate::NN::NN(result_natural));
                Ok(result_nn.into())
            } else {
                // If not both NN::NN, return the first object as before
                Ok(n.clone().into())
            }
        }
        unexpected => unexpected_args("|Object, Object|", unexpected),
    });

    result
}
