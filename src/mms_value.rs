
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

// Common Functions

pub fn mms_value_equals(value1: &MmsValue, value2: &MmsValue) -> bool {
    value1 == value2
}
pub fn mms_value_get_type(value: &MmsValue) -> MmsType {
    match value {
        MmsValue::Boolean{value: _} => MmsType::Boolean,
        MmsValue::Integer{value: _, size: _} => MmsType::Integer,
        MmsValue::Unsigned{value: _, size: _} => MmsType::Unsigned,
    }
}
pub fn mms_value_equal_types(value1: &MmsValue, value2: &MmsValue) -> bool {
    mms_value_get_type(value1) == mms_value_get_type(value2)
}
pub fn mms_value_update(value1: &mut MmsValue, value2: &MmsValue) -> bool {
    match value1 {
        MmsValue::Boolean{value: v1} => {
            if let MmsValue::Boolean{value: v2} = value2 {
                *v1 = *v2;
                true
            }
            else {
                false
            }
        },
        MmsValue::Integer{value: v1, size: s1} => {
            if let MmsValue::Integer{value: v2, size: s2} = value2 && *s1 >= *s2 {
                *v1 = *v2;
                *s1 = *s2;
                true
            }
            else {
                false
            }
        },
        MmsValue::Unsigned{value: v1, size: s1} => {
            if let MmsValue::Unsigned{value: v2, size: s2} = value2 && *s1 >= *s2 {
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

// Integer Functions

pub fn mms_value_new_integer_from_int32(integer: i32) -> MmsValue {
    MmsValue::Integer{value: integer as i64, size: 5}
}
pub fn mms_value_set_int32(value: &mut MmsValue, integer: i32) {
    if let MmsValue::Integer{value: v, size: s} = value && *s >= 4 {
        *v = integer as i64;
    }
}
pub fn mms_value_to_int32(value: &MmsValue) -> i32 {
    match value {
        MmsValue::Integer { value: v, size: _ } => {
            *v as i32
        },
        MmsValue::Unsigned { value: v, size: _ } => {
            *v as i32
        },
        _ => 0,
    }
}

// Unsigned Integer Functions

pub fn mms_value_new_unsigned_from_uint32(integer: u32) -> MmsValue {
    MmsValue::Unsigned{value: integer as u64, size: 5}
}
pub fn mms_value_set_uint32(value: &mut MmsValue, integer: u32) {
    if let MmsValue::Unsigned{value: v, size: s} = value && *s >= 4 {
        *v = integer as u64;
    }
}
pub fn mms_value_to_uint32(value: &MmsValue) -> u32 {
    match value {
        MmsValue::Integer { value: v, size: _ } => {
            *v as u32
        },
        MmsValue::Unsigned { value: v, size: _ } => {
            *v as u32
        },
        _ => 0,
    }
}

// Boolean Functions

pub fn mms_value_new_boolean(boolean: bool) -> MmsValue {
    MmsValue::Boolean { value: boolean }
}
pub fn mms_value_set_boolean(value: &mut MmsValue, boolean: bool) {
    if let MmsValue::Boolean { value: v } = value {
        *v = boolean;
    }
}
pub fn mms_value_get_boolean(value: &MmsValue) -> bool {
    if let MmsValue::Boolean { value: v } = value {
        *v
    }
    else {
        false
    }
}
