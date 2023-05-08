use std::collections::VecDeque;

mod tram;

type Path = Vec<tram::Action>;
type Address = u64;
type Cost = u64;

fn main() {
    let my_instance = tram::TransportationProblem::new(100);
    let path: Path;
    let cost: Cost;
    (path, cost) = breadth_first_search(&my_instance);
    println!("Find pash is {:?}", path);
    println!("With time cost {:?}", cost);
}

/// implement breadth first search for tram problem
fn breadth_first_search(problem: &tram::TransportationProblem) -> (Path, Cost) {
    let mut que: VecDeque<(Address, Cost, Path)> = VecDeque::new();
    que.push_back((problem.start(), 0, Vec::new()));

    let mut action: tram::Action;
    let mut new_state: Address;
    let mut cost: Cost;
    let mut history_cost: Cost;
    let mut path: Path;

    loop {
        let item = que.pop_front();
        match item {
            Some(x) => (new_state, history_cost, path) = x,
            None => {
                panic!("Unreachable destination!");
            }
        }
        if problem.is_end(new_state) {
            return (path, history_cost);
        }
        for item in problem.next_act_cost(new_state) {
            (action, new_state, cost) = item;
            let mut new_path = path.clone();
            new_path.push(action);
            que.push_back((new_state, history_cost + cost, new_path));
        }
    }
}
