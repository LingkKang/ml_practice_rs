type Address = u64;
type Cost = u64;

/// Simply transport from `start` to `dest`
/// with possible actions defined in `Action`
pub struct TransportationProblem {
    start: Address,
    dest: Address,
}

/// Available action in transportation problem
/// - `Walk` take you from `k` to `k + 1` with time cost `1`
/// - `Tram` take you from `k` to `2 * k` with time cost `2`
#[derive(Debug, Clone)]
pub enum Action {
    Walk,
    Tram,
}

impl TransportationProblem {
    pub fn new(dest: Address) -> Self {
        Self { start: 1, dest }
    }

    pub fn start(&self) -> Address {
        self.start
    }

    pub fn is_end(&self, state: Address) -> bool {
        self.dest == state
    }

    pub fn next_act_cost(&self, state: Address) -> Vec<(Action, Address, Cost)> {
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
