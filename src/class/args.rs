use crate::alloc::VmRef;
use crate::class::{Class, FunctionArgs, Object};
use crate::error::{Throwables, VmResult};
use crate::types::DataValue;

use log::warn;
use std::any::type_name;

pub trait Arg: Sized {
    fn convert(val: DataValue) -> Result<Self, Throwables>;
}

struct ArgTaker<'a, 'b>(&'a mut FunctionArgs<'b>, usize);

impl<'a, 'b> ArgTaker<'a, 'b> {
    fn take_next(&mut self) -> Result<DataValue, Throwables> {
        self.1 -= 1;
        let val = self
            .0
            .try_take(self.1)
            .ok_or(Throwables::Other("java/lang/InternalError"))?;
        Ok(val)
    }
}

pub trait Args: Sized {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables>;
}

impl<T: TryFrom<DataValue, Error = DataValue>> Arg for T {
    fn convert(val: DataValue) -> Result<Self, Throwables> {
        Self::try_from(val).map_err(|val| {
            warn!("failed to convert {:?} to {:?}", val, type_name::<Self>());
            Throwables::Other("java/lang/InternalError")
        })
    }
}

impl Arg for String {
    fn convert(val: DataValue) -> Result<Self, Throwables> {
        let obj = <VmRef<Object> as Arg>::convert(val)?;

        obj.string_value_utf8()
            .ok_or(Throwables::NullPointerException)
    }
}

// -----
impl<A: Arg> Args for (A,) {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables> {
        let mut taker = ArgTaker(args, 1);
        Ok((taker.take_next().and_then(|v| A::convert(v))?,))
    }
}
impl<A: Arg, B: Arg> Args for (A, B) {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables> {
        let mut taker = ArgTaker(args, 2);
        Ok((
            taker.take_next().and_then(|v| A::convert(v))?,
            taker.take_next().and_then(|v| B::convert(v))?,
        ))
    }
}
impl<A: Arg, B: Arg, C: Arg> Args for (A, B, C) {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables> {
        let mut taker = ArgTaker(args, 3);
        Ok((
            taker.take_next().and_then(|v| A::convert(v))?,
            taker.take_next().and_then(|v| B::convert(v))?,
            taker.take_next().and_then(|v| C::convert(v))?,
        ))
    }
}
impl<A: Arg, B: Arg, C: Arg, D: Arg> Args for (A, B, C, D) {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables> {
        let mut taker = ArgTaker(args, 4);
        Ok((
            taker.take_next().and_then(|v| A::convert(v))?,
            taker.take_next().and_then(|v| B::convert(v))?,
            taker.take_next().and_then(|v| C::convert(v))?,
            taker.take_next().and_then(|v| D::convert(v))?,
        ))
    }
}
impl<A: Arg, B: Arg, C: Arg, D: Arg, E: Arg> Args for (A, B, C, D, E) {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables> {
        let mut taker = ArgTaker(args, 5);
        Ok((
            taker.take_next().and_then(|v| A::convert(v))?,
            taker.take_next().and_then(|v| B::convert(v))?,
            taker.take_next().and_then(|v| C::convert(v))?,
            taker.take_next().and_then(|v| D::convert(v))?,
            taker.take_next().and_then(|v| E::convert(v))?,
        ))
    }
}

impl<A: Arg, B: Arg, C: Arg, D: Arg, E: Arg, F: Arg> Args for (A, B, C, D, E, F) {
    fn try_from(args: &mut FunctionArgs) -> Result<Self, Throwables> {
        let mut taker = ArgTaker(args, 6);
        Ok((
            taker.take_next().and_then(|v| A::convert(v))?,
            taker.take_next().and_then(|v| B::convert(v))?,
            taker.take_next().and_then(|v| C::convert(v))?,
            taker.take_next().and_then(|v| D::convert(v))?,
            taker.take_next().and_then(|v| E::convert(v))?,
            taker.take_next().and_then(|v| F::convert(v))?,
        ))
    }
}
// ---

impl<'a> FunctionArgs<'a> {
    pub fn destructure<A: Args>(mut self) -> Result<A, Throwables> {
        A::try_from(&mut self)
    }
}

#[cfg(test)]
mod tests {
    use crate::class::FunctionArgs;
    use crate::types::DataValue;
    use itertools::Itertools;

    fn mk_args<const N: usize>(args: [DataValue; N]) -> FunctionArgs<'static> {
        let owned = args.into_iter().rev().collect_vec().into_boxed_slice();
        let leaked = Box::leak(owned);
        FunctionArgs::from(leaked)
    }

    #[test]
    fn single() {
        let (i,) = mk_args([DataValue::Int(5)])
            .destructure::<(i32,)>()
            .unwrap();
        assert_eq!(i, 5);

        assert!(mk_args([DataValue::Int(5)])
            .destructure::<(f32,)>()
            .is_err());
        assert!(mk_args([DataValue::Int(5)])
            .destructure::<(i32, i32)>()
            .is_err());
    }

    #[test]
    fn int_to_bool() {
        env_logger::builder().is_test(true).init();
        let (b1, l, f, b2) = mk_args([
            DataValue::Boolean(true),
            DataValue::Long(100),
            DataValue::Float(2.0),
            DataValue::Int(1),
        ])
        .destructure::<(bool, i64, f32, bool)>()
        .unwrap();
        assert!(b1);
        assert!(b2);
        assert_eq!(l, 100);
        assert_eq!(f, 2.0);
    }
}
