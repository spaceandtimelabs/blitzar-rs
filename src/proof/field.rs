use ark_grumpkin::Fq;

pub trait FieldId {
    const FIELD_ID: u32;
}

impl FieldId for Fq {
    const FIELD_ID: u32 = blitzar_sys::SXT_FIELD_GRUMPKIN;
}
