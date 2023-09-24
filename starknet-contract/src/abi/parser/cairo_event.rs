//! Event parsing.
use starknet_core::types::contract::{AbiEvent, AbiNamedMember, EventFieldKind, TypedAbiEvent};

use super::abi_types::{AbiType, AbiTypeAny};
use super::{CairoEnum, CairoStruct};

#[derive(Debug, Clone)]
pub enum CairoEventInner {
    Enum(CairoEnum),
    Struct(CairoStruct),
}

#[derive(Debug, Clone)]
pub struct CairoEvent {
    pub abi: AbiTypeAny,
    pub inner: CairoEventInner,
    pub fields_kinds: Vec<EventFieldKind>,
}

impl CairoEvent {
    /// Gets the name of the struct type.
    pub fn get_name(&self) -> String {
        self.abi.get_cairo_type_name()
    }

    /// Gets the count for each field kind (keys, data).
    pub fn count_fields_kinds(&self) -> (usize, usize) {
        let mut k = 0;
        let mut d = 0;

        for fk in &self.fields_kinds {
            match fk {
                EventFieldKind::Key => k += 1,
                EventFieldKind::Data => d += 1,
                _ => continue,
            }
        }

        (k, d)
    }

    /// Initializes a new instance from the abi name and it's members.
    pub fn new(abi_event: &AbiEvent) -> Option<CairoEvent> {
        match abi_event {
            AbiEvent::Typed(typed_e) => match typed_e {
                TypedAbiEvent::Struct(s) => {
                    if s.members.is_empty() {
                        return None;
                    }

                    let name = &s.name;
                    let mut kinds = vec![];
                    let members = s
                        .members
                        .iter()
                        .map(|m| {
                            kinds.push(m.kind.clone());
                            AbiNamedMember {
                                name: m.name.clone(),
                                r#type: m.r#type.clone(),
                            }
                        })
                        .collect();

                    let cs = CairoStruct::new(name, &members);

                    Some(CairoEvent {
                        abi: AbiTypeAny::from_string(name),
                        inner: CairoEventInner::Struct(cs),
                        fields_kinds: kinds,
                    })
                }
                TypedAbiEvent::Enum(e) => {
                    if e.variants.is_empty() {
                        return None;
                    }

                    let name = &e.name;
                    let mut kinds = vec![];
                    let variants = e
                        .variants
                        .iter()
                        .map(|v| {
                            kinds.push(v.kind.clone());
                            AbiNamedMember {
                                name: v.name.clone(),
                                r#type: v.r#type.clone(),
                            }
                        })
                        .collect();

                    let ce = CairoEnum::new(name, &variants);

                    Some(CairoEvent {
                        abi: AbiTypeAny::from_string(name),
                        inner: CairoEventInner::Enum(ce),
                        fields_kinds: kinds,
                    })
                }
            },
            AbiEvent::Untyped(_) => {
                // Can we support this..?
                //panic!("Untyped events are not supported");
                None
            }
        }
    }
}
