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
    pub fn bitcount(&self) -> KValue {
        KValue::from(self.0.bitcount())
    }

    #[koto_method]
    pub fn factorial(&self) -> KValue {
        KValue::Object(KObject::from(NN::from(NN(self.0.factorial()))))
    }

    #[koto_method]
    pub fn is_square(&self) -> KValue {
        self.0.is_square().into()
    }

    #[koto_method]
    pub fn sqrt_ceil(&self) -> KValue {
        KValue::Object(KObject::from(NN::from(NN(self.0.sqrt_ceil()))))
    }

    #[koto_method]
    pub fn sqrt_floor(&self) -> KValue {
        KValue::Object(KObject::from(NN::from(NN(self.0.sqrt_floor()))))
    }

    pub fn generator(_ctx: &mut CallContext) -> Result<KValue> {
        // Create a KMap that will act as our generator
        let mut generator_map = KMap::default();

        // Initialize the counter in the map's data using ValueKey
        use koto_runtime::ValueKey;
        generator_map.data_mut().insert(ValueKey::from("counter"), KValue::Number(0.into()));

        // Add the @next function to the metamap
        generator_map.insert_meta(
            MetaKey::Named("next".into()),
            KNativeFunction::new(|ctx| -> Result<KValue> {
                // Get the current counter value from the instance (which should be our KMap)
                let counter_value = if let KValue::Map(map) = ctx.instance() {
                    match map.data().get(&ValueKey::from("counter")) {
                        Some(KValue::Number(n)) => n.try_into().unwrap_or(0u64),
                        _ => 0,
                    }
                } else {
                    0
                };

                // Create the result (current counter as NN)
                let result = KValue::Object(KObject::from(NN(Natural::from(counter_value))));

                // Update the counter in the instance data for the next call
                if let KValue::Map(map) = ctx.instance() {
                    map.data_mut().insert(ValueKey::from("counter"), KValue::Number((counter_value + 1).into()));
                }

                Ok(result)
            }).into(),
        );

        Ok(generator_map.into())
    }
}

impl KotoObject for NN {
    fn add(&self, other: &KValue) -> Result<KValue> {
        match other {
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
        match other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                Ok(self.0 == other.0)
            }
            unexpected => unexpected_type("Number", unexpected),
        }
    }

    fn multiply(&self, rhs: &KValue) -> Result<KValue> {
        match rhs {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() * other.0.clone();
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }

    fn subtract(&self, rhs: &KValue) -> Result<KValue> {
        match rhs {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                let result = self.0.clone() - other.0.clone();
                Ok(KValue::Object(KObject::from(Self(result))))
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }

    fn add_assign(&mut self, other: &KValue) -> Result<()> {
        match other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                self.0 += other.0.clone();
                Ok(())
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }

    fn multiply_assign(&mut self, other: &KValue) -> Result<()> {
        match other {
            KValue::Object(other) if other.is_a::<Self>() => {
                let other = other.cast::<Self>().unwrap();
                self.0 *= other.0.clone();
                Ok(())
            }
            unexpected => unexpected_type("NN", unexpected),
        }
    }
}
