use crate::dna::DNA;
use codec::{Decode, Encode};

#[derive(Encode, Decode, Debug, Clone)]
pub struct Parents {
    father: DNA,
    mother: DNA,
}

impl Parents {
    pub fn new() -> Self {
        Self {
            father: DNA::new(),
            mother: DNA::new(),
        }
    }
    pub fn set_father(self, father: DNA) -> Self {
        Self { father, ..self }
    }

    pub fn set_mother(self, mother: DNA) -> Self {
        Self { mother, ..self }
    }
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct Kitty {
    parents_dna: Parents,
    brothers_dna: DNA,
    children_dna: DNA,
    partner_dna: DNA,
    self_dna: DNA,
}

impl Kitty {
    pub fn new() -> Self {
        Self {
            parents_dna: Parents {
                father: DNA::new(),
                mother: DNA::new(),
            },
            brothers_dna: DNA::new(),
            children_dna: DNA::new(),
            partner_dna: DNA::new(),
            self_dna: DNA::new(),
        }
    }

    pub fn set_self_dna(self, dna: DNA) -> Self {
        Self {
            self_dna: dna,
            ..self
        }
    }
    pub fn set_parents_dna(self, parent_dna: Parents) -> Self {
        Self {
            parents_dna: parent_dna,
            ..self
        }
    }

    pub fn set_brothers_dna(self, brothers_dns: DNA) -> Self {
        Self {
            brothers_dna: brothers_dns,
            ..self
        }
    }

    pub fn set_partner_dna(self, partner_dna: DNA) -> Self {
        Self {
            partner_dna,
            ..self
        }
    }

    pub fn mutate_partner_dna(&mut self, partner_dna: DNA) {
        self.partner_dna = partner_dna;
    }

    pub fn mutate_children_dna(&mut self, children_dna: DNA) {
        self.children_dna = children_dna;
    }

    pub fn get_self_dna(&self) -> DNA {
        self.self_dna
    }
}