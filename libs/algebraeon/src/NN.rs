use koto_runtime::{Result, derive::*, prelude::*};

use algebraeon::nzq::Natural;

#[derive(PartialEq, Clone, KotoCopy, KotoType, Eq, Debug)]
pub struct NN(pub Natural);

#[koto_impl]
impl NN {
    pub fn make_koto_object(n: KNumber) -> KObject {
        let my_int = Natural::from(u64::from(n));
        KObject::from(Self(my_int))
    }

    #[koto_method]
    pub fn factorial(&self) -> KValue {
        KValue::Object(KObject::from(NN::from(NN(self.0.factorial()))))
    }
}

impl KotoObject for NN {
    fn add(&self, other: &KValue) -> Result<KValue> {
        match &other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() + other.0.clone();
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }

    fn display(&self, ctx: &mut DisplayContext) -> koto_runtime::Result<()> {
        ctx.append(self.0.to_string());
        Ok(())
    }

    fn equal(&self, other: &KValue) -> Result<bool> {
        match &other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                Ok(self.0 == other.0)
            }
            unexpected => unexpected_type("Number", unexpected),
        }
    }

    fn multiply(&self, rhs: &KValue) -> Result<KValue> {
        match &rhs {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() * other.0.clone();
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }

    fn subtract(&self, rhs: &KValue) -> Result<KValue> {
        match &rhs {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() - other.0.clone();
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }
}
