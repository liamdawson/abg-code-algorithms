pub const GOLEM_VALUES: [&str; 6] = ["R", "r", "G", "g", "B", "b"];

#[derive(Clone, Debug)]
pub struct GolemAssignments {
    pub head: &'static str,
    pub chest: &'static str,
}

impl GolemAssignments {
    pub fn values(&self) -> impl std::iter::ExactSizeIterator<Item = &'static str> {
        std::array::IntoIter::new([self.head, self.chest])
    }
}

pub fn from_lehmer(lehmer: &[u8]) -> crate::GolemAssignments {
    if lehmer.len() != 2 {
        panic!("wrong number of golem lehmer values ({})", lehmer.len());
    }

    let mut results = vec![];

    let mut choices = vec![0, 1, 2, 3, 4, 5];

    for i in 0..=1 {
        let current_val = lehmer[i];
        let ivar4 = choices[lehmer[i] as usize];

        results.push(ivar4);

        choices.remove(2 * (current_val / 2) as usize);
        choices.remove(2 * (current_val / 2) as usize);
    }

    GolemAssignments {
        chest: GOLEM_VALUES[results[0]],
        head: GOLEM_VALUES[results[1]],
    }
}
