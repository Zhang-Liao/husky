use crate::*;

// downcast primitives
impl RegularValue {
    pub unsafe fn downcast_unit(self) -> () {
        match self {
            RegularValue::Unit(_) => (),
            _ => unreachable!(),
        }
    }
    pub unsafe fn downcast_bool(self) -> bool {
        match self {
            RegularValue::Bool(b) => b,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_char(self) -> char {
        match self {
            RegularValue::Char(c) => c,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_i8(self) -> i8 {
        match self {
            RegularValue::I8(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_i16(self) -> i16 {
        match self {
            RegularValue::I16(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_i32(self) -> i32 {
        match self {
            RegularValue::I32(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_i64(self) -> i64 {
        match self {
            RegularValue::I64(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_i128(self) -> i128 {
        match self {
            RegularValue::I128(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_isize(self) -> isize {
        match self {
            RegularValue::ISize(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_u8(self) -> u8 {
        match self {
            RegularValue::U8(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_u16(self) -> u16 {
        match self {
            RegularValue::U16(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_u32(self) -> u32 {
        match self {
            RegularValue::U32(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_u64(self) -> u64 {
        match self {
            RegularValue::U64(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_u128(self) -> u128 {
        match self {
            RegularValue::U128(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_usize(self) -> usize {
        match self {
            RegularValue::USize(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_r8(self) -> u8 {
        match self {
            RegularValue::R8(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_r16(self) -> u16 {
        match self {
            RegularValue::R16(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_r32(self) -> u32 {
        match self {
            RegularValue::R32(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_r64(self) -> u64 {
        match self {
            RegularValue::R64(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_r128(self) -> u128 {
        match self {
            RegularValue::R128(val) => val,
            _ => unreachable!(),
        }
    }

    pub unsafe fn downcast_rsize(self) -> usize {
        match self {
            RegularValue::RSize(val) => val,
            _ => unreachable!(),
        }
    }
}

// downcast any
impl RegularValue {
    unsafe fn downcast_intrinsic_trivial<T>(self) -> T
    where
        T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
    {
        let RegularValue::Intrinsic(box_value) = self else {
            unreachable!()
        };
        let box_value = box_value as Box<dyn std::any::Any>;
        box_value
            .downcast::<__RegularStandTrivialImpl<T>>()
            .expect("type right")
            .downcast()
    }

    unsafe fn downcast_sized_ref<'a, T>(self) -> &'a T
    where
        T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
    {
        let RegularValue::SizedRef(ref_value) = self else {
            unreachable!()
        };
        let ref_value = ref_value as *const dyn std::any::Any;
        wild_utils::arb_ref(
            (*ref_value)
                .downcast_ref::<__RegularStandTrivialImpl<T>>()
                .expect("type right")
                .downcast_ref(),
        )
    }

    unsafe fn downcast_sized_mut<'a, T>(self) -> &'a mut T
    where
        T: std::fmt::Debug + Clone + UnwindSafe + RefUnwindSafe + 'static,
    {
        let RegularValue::SizedMut(mut_value) = self else {
            unreachable!()
        };
        let mut_value = mut_value as *mut dyn std::any::Any;
        wild_utils::arb_ref(
            (*mut_value)
                .downcast_mut::<__RegularStandTrivialImpl<T>>()
                .expect("type right")
                .downcast_mut(),
        )
    }
}