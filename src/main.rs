use std::collections::VecDeque;

mod tram;

type Path = Vec<tram::Action>;
type Address = u64;
type Cost = u64;

/// different types of blind search:
/// - `BFS`: Breadth First Search
/// - `DFS`: Depth First Search
/// - `UCS`: Uniform Cost Search
enum SearchType {
    /// Breadth First Search
    BFS,

    /// Depth First Search
    DFS,

    /// Uniform Cost Search
    UCS,
}

fn main() {
    let my_instance = tram::TransportationProblem::new(17);
    let mut path: Path;
    let mut cost: Cost;

    (path, cost) = generic_blind_search(&my_instance, SearchType::BFS);
    println!(
        "The path found by Breadth First Search is {:?}, with time cost {}",
        path, cost
    );

    (path, cost) = generic_blind_search(&my_instance, SearchType::DFS);
    println!(
        "The path found by Depth First Search is {:?}, with time cost {}",
        path, cost
    );

    (path, cost) = generic_blind_search(&my_instance, SearchType::UCS);
    println!(
        "The path found by Uniform Cost Search is {:?}, with time cost {}",
        path, cost
    );
}

/// Generic blind search:
/// with different given `search_type`, perform different search
fn generic_blind_search(
    problem: &tram::TransportationProblem,
    search_type: SearchType,
) -> (Path, Cost) {
    let mut que: VecDeque<(Address, Cost, Path)> = VecDeque::new();
    que.push_back((problem.start(), 0, Vec::new()));

    let mut action: tram::Action;
    let mut new_state: Address;
    let mut cost: Cost;
    let mut history_cost: Cost;
    let mut path: Path;

    loop {
        let item = que_action(&mut que, &search_type);
        (new_state, history_cost, path) = item;
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

/// The major difference between blind search algorithms are
/// the difference of the order of new node exploring
fn que_action(
    que: &mut VecDeque<(Address, Cost, Path)>,
    search_type: &SearchType,
) -> (Address, Cost, Path) {
    match search_type {
        SearchType::BFS => match que.pop_front() {
            Some(x) => x,
            None => panic!("Unreachable destination!"),
        },
        SearchType::DFS => match que.pop_back() {
            Some(x) => x,
            None => panic!("Unreachable destination!"),
        },
        SearchType::UCS => {
            que.make_contiguous().sort_by_key(|tuple| tuple.1);
            match que.pop_front() {
                Some(x) => x,
                None => panic!("Unreachable destination!"),
            }
        }
    }
}
