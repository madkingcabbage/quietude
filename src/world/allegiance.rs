use crate::ui::traits::{ChoiceAttribute, StringLookup, StringLookupDictionary};

pub struct Allegiance;

impl StringLookup for Allegiance {
    fn dictionary() -> &'static StringLookupDictionary<Self> {
        todo!()
    }
}

impl ChoiceAttribute for Allegiance {}
