// use derive_more::From;
// use koto::{derive::*, prelude::*, runtime};
use koto_runtime::{Result, derive::*, prelude::*};

use algebraeon::{nzq::Integer, rings::structure::MetaFactorableStructure};

#[derive(PartialEq, Clone, KotoCopy, KotoType, Eq, Debug)]
pub struct ZZ(Integer);

#[koto_impl]
impl ZZ {
    pub fn make_koto_object(n: KNumber) -> KObject {
        let my_int = Integer::from(i64::from(n));
        KObject::from(Self(my_int))
    }

    #[koto_method]
    pub fn is_irreducible(&self) -> KValue {
        self.0.is_irreducible().into()
    }
}

impl KotoObject for ZZ {
    fn add(&self, other: &KValue) -> Result<KValue> {
        match &other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() + other.0.clone(); // suma de `Integer`
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("ZZ", unexpected),
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

    fn negate(&self, _vm: &mut KotoVm) -> Result<KValue> {
        Ok(KValue::Object(KObject::from(Self(-self.0.clone()))))
    }

    fn multiply(&self, rhs: &KValue) -> Result<KValue> {
        match &rhs {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() * other.0.clone(); // suma de `Integer`
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("ZZ", unexpected),
        }
    }

    fn subtract(&self, rhs: &KValue) -> Result<KValue> {
        match &rhs {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() - other.0.clone(); // suma de `Integer`
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("ZZ", unexpected),
        }
    }
}
