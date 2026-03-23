
#[derive(Debug, Clone, PartialEq)]
pub enum MmsValue {
    Boolean{ value: bool },
    Integer{ value: i64, size: u8 },
    Unsigned{ value: u64, size: u8 },
    // TODO: other enum values out of scope for initial POC
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MmsType {
    Boolean,
    Integer,
    Unsigned,
}

impl MmsValue {
    //////////////////////
    // Common Functions //
    //////////////////////

    // MmsValue_equals
    pub fn equals(&self, other: &Self) -> bool {
        self == other
    }

    // MmsValue_getType
    pub fn get_type(&self) -> MmsType {
        match self {
            MmsValue::Boolean{value: _} => MmsType::Boolean,
            MmsValue::Integer{value: _, size: _} => MmsType::Integer,
            MmsValue::Unsigned{value: _, size: _} => MmsType::Unsigned,
        }
    }

    // MmsValue_equalTypes
    pub fn equal_types(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
    }

    // MmsValue_update
    pub fn update(&mut self, other: &MmsValue) -> bool {
        match self {
            MmsValue::Boolean{value: v1} => {
                if let MmsValue::Boolean{value: v2} = other {
                    *v1 = *v2;
                    true
                }
                else {
                    false
                }
            },
            MmsValue::Integer{value: v1, size: s1} => {
                if let MmsValue::Integer{value: v2, size: s2} = other && *s1 >= *s2 {
                    *v1 = *v2;
                    *s1 = *s2;
                    true
                }
                else {
                    false
                }
            },
            MmsValue::Unsigned{value: v1, size: s1} => {
                if let MmsValue::Unsigned{value: v2, size: s2} = other && *s1 >= *s2 {
                    *v1 = *v2;
                    *s1 = *s2;
                    true
                }
                else {
                    false
                }
            },
        }
    }

    ///////////////////////
    // Integer Functions //
    ///////////////////////

    // MmsValue_newIntegerFromInt32
    pub fn new_integer_from_int32(integer: i32) -> MmsValue {
        MmsValue::Integer{value: integer as i64, size: 5}
    }

    // MmsValue_setInt32
    pub fn set_int32(&mut self, integer: i32) {
        if let MmsValue::Integer{value: v, size: s} = self && *s >= 4 {
            *v = integer as i64;
        }
    }

    // MmsValue_toInt32
    pub fn to_int32(&self) -> i32 {
        match self {
            MmsValue::Integer { value: v, size: _ } => {
                *v as i32
            },
            MmsValue::Unsigned { value: v, size: _ } => {
                *v as i32
            },
            _ => 0,
        }
    }

    ////////////////////////////////
    // Unsigned Integer Functions //
    ////////////////////////////////

    // MmsValue_newIntegerFromUint32
    pub fn new_unsigned_from_uint32(integer: u32) -> MmsValue {
        MmsValue::Unsigned{value: integer as u64, size: 5}
    }

    // MmsValue_setUint32
    pub fn set_uint32(&mut self, integer: u32) {
        if let MmsValue::Unsigned{value: v, size: s} = self && *s >= 4 {
            *v = integer as u64;
        }
    }

    // MmsValue_toUint32
    pub fn to_uint32(&self) -> u32 {
        match self {
            MmsValue::Integer { value: v, size: _ } => {
                *v as u32
            },
            MmsValue::Unsigned { value: v, size: _ } => {
                *v as u32
            },
            _ => 0,
        }
    }

    ///////////////////////
    // Boolean Functions //
    ///////////////////////

    // MmsValue_newBoolean
    pub fn new_boolean(boolean: bool) -> MmsValue {
        MmsValue::Boolean { value: boolean }
    }

    // MmsValue_setBoolean
    pub fn set_boolean(&mut self, boolean: bool) {
        if let MmsValue::Boolean { value: v } = self {
            *v = boolean;
        }
    }

    // MmsValue_getBoolean
    pub fn get_boolean(&self) -> bool {
        if let MmsValue::Boolean { value: v } = self {
            *v
        }
        else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_changes_up() {
        let mut v = MmsValue::new_integer_from_int32(0);
        assert_eq!(v.to_int32(), 0);

        v.set_int32(5);
        assert_eq!(v.to_int32(), 5);
    }
    #[test]
    fn integer_changes_down() {
        let mut v = MmsValue::new_integer_from_int32(2);
        assert_eq!(v.to_int32(), 2);

        v.set_int32(1);
        assert_eq!(v.to_int32(), 1);
    }

    #[test]
    fn boolean_changes_true_false() {
        let mut v = MmsValue::new_boolean(true);
        assert_eq!(v.get_boolean(), true);

        v.set_boolean(false);
        assert_eq!(v.get_boolean(), false);
    }
    #[test]
    fn boolean_changes_false_true() {
        let mut v = MmsValue::new_boolean(false);
        assert_eq!(v.get_boolean(), false);

        v.set_boolean(true);
        assert_eq!(v.get_boolean(), true);
    }

    #[test]
    fn type_mismatch_set_fails_boolean() {
        let mut v = MmsValue::new_boolean(true);
        assert_eq!(v.get_boolean(), true);

        v.set_int32(0);
        assert_eq!(v.get_boolean(), true);
    }
    #[test]
    fn type_mismatch_set_fails_integer() {
        let mut v = MmsValue::new_integer_from_int32(10);
        assert_eq!(v.to_int32(), 10);

        v.set_boolean(false);
        assert_eq!(v.to_int32(), 10);
    }

    #[test]
    fn integer_and_unsigned_matching() {
        let i = MmsValue::new_integer_from_int32(12);
        let u = MmsValue::new_unsigned_from_uint32(12);

        assert_eq!(i.to_int32(), u.to_int32());
        assert_eq!(i.to_uint32(), u.to_uint32());
    }
    #[test]
    fn integer_and_unsigned_set_mismatch() {
        let mut i = MmsValue::new_integer_from_int32(8);
        assert_eq!(i.to_int32(), 8);

        i.set_uint32(20);
        assert_eq!(i.to_int32(), 8);
    }
    #[test]
    fn unsigned_and_integer_set_mismatch() {
        let mut u = MmsValue::new_unsigned_from_uint32(8);
        assert_eq!(u.to_uint32(), 8);

        u.set_int32(20);
        assert_eq!(u.to_uint32(), 8);
    }

    #[test]
    fn equals_check() {
        let a = MmsValue::new_integer_from_int32(10);
        let b = MmsValue::new_integer_from_int32(10);
        let c = MmsValue::new_integer_from_int32(20);
        let d = MmsValue::new_boolean(true);

        assert!(a.equals(&b));
        assert!(!a.equals(&c));
        assert!(!a.equals(&d));
    }

    #[test]
    fn equal_types_check() {
        let a = MmsValue::new_integer_from_int32(10);
        let b = MmsValue::new_integer_from_int32(20);
        let c = MmsValue::new_boolean(true);

        assert!(a.equal_types(&b));
        assert!(!a.equal_types(&c));
    }

    #[test]
    fn update_int() {
        let mut a = MmsValue::new_integer_from_int32(10);
        let b = MmsValue::new_integer_from_int32(20);

        assert!(a.update(&b));
        assert_eq!(a.to_int32(), 20);
    }
    #[test]
    fn update_mismatch() {
        let mut a = MmsValue::new_integer_from_int32(10);
        let b = MmsValue::new_unsigned_from_uint32(20);
        let c = MmsValue::new_boolean(true);

        assert!(!a.update(&b));
        assert_eq!(a.to_int32(), 10);
        assert!(!a.update(&c));
        assert_eq!(a.to_int32(), 10);
    }
}
