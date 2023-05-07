
pub struct TransportationProblem {
    start: u64,
    dest: u64,
}

#[derive(Debug)]

pub enum Action {
    Walk,
    Tram,
}

impl TransportationProblem {
    pub fn new(dest: u64) -> Self {
        Self { start: 1, dest }
    }

    pub fn is_end(&self, state: u64) -> bool {
        self.dest == state
    }

    pub fn next_act_cost(&self, state: u64) -> Vec<(Action, u64, u64)> {
        let mut ans = Vec::new();
        if state + 1 <= self.dest {
            ans.push((Action::Walk, state + 1, 1));
        }
        if state * 2 <= self.dest {
            ans.push((Action::Tram, state * 2, 2));
        }
        ans
    }
}