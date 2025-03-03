use ark_grumpkin::Fr;

pub trait FieldId {
    const FIELD_ID: u32;
}

impl FieldId for Fr {
    const FIELD_ID: u32 = blitzar_sys::SXT_FIELD_GRUMPKIN;
}
