pub const ORDERED_ALCHEMICALS: [&str; 8] = ["R+", "G+", "B+", "R-", "G-", "B-", "++", "--"];

#[derive(Clone, Debug)]
pub struct IngredientAssignments {
    pub fern: &'static str,
    pub claw: &'static str,
    pub mushroom: &'static str,
    pub flower: &'static str,
    pub root: &'static str,
    pub scorpion: &'static str,
    pub toad: &'static str,
    pub feather: &'static str,
}

impl IngredientAssignments {
    pub fn values(&self) -> impl std::iter::ExactSizeIterator<Item = &'static str> {
        std::array::IntoIter::new([
            self.fern,
            self.claw,
            self.mushroom,
            self.flower,
            self.root,
            self.scorpion,
            self.toad,
            self.feather,
        ])
    }
}

pub fn from_lehmer(lehmer: &[u8]) -> crate::IngredientAssignments {
    if lehmer.len() != 7 {
        panic!(
            "wrong number of ingredient lehmer values ({})",
            lehmer.len()
        );
    }

    let mut options = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let mut choices = Vec::with_capacity(8);

    for i in lehmer {
        choices.push(options.remove(*i as usize));
    }

    choices.push(options.remove(0));

    IngredientAssignments {
        fern: ORDERED_ALCHEMICALS[choices[0]],
        claw: ORDERED_ALCHEMICALS[choices[1]],
        mushroom: ORDERED_ALCHEMICALS[choices[2]],
        flower: ORDERED_ALCHEMICALS[choices[3]],
        root: ORDERED_ALCHEMICALS[choices[4]],
        scorpion: ORDERED_ALCHEMICALS[choices[5]],
        toad: ORDERED_ALCHEMICALS[choices[6]],
        feather: ORDERED_ALCHEMICALS[choices[7]],
    }
}
