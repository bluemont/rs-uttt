/// Functions for intelligent behaviors.

use data::*;
use lru_time_cache::LruCache;
use std::cmp::Ordering;

// == data structures ==========================================================

/// The computed outcome of a game; either:
/// 1. A win for player X in some number of plays
/// 2. A win for player O in some number of plays
/// 3. A tie in some number of turns
/// 4. Not one of the above, so unknown.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Outcome {
    Win { player: Player, turns: Count },
    Tie { turns: Count },
    Unknown { depth: Count },
}

pub type Count = u8;

/// A solution to a game state, containing (a) the optimal next play (optional)
/// and (b) the resulting outcome.
///
/// For example, if the outcome is a 'win in N plays', this means that the
/// current player will beat an optimal opponent in N plays. However, the
/// current player may beat a non-optimal opponent in fewer than N plays.
///
/// Some specific examples, assuming optimal players:
///
/// 1. See turn 30 in the table below; it is X's move. Let's assume that X's
///    best move will ensure victory in 5 moves (counting the moves of both
///    players), so the 'outcome' of turn 30 is a 'win by X in 5'. In this kind
///    of situation (ie.e X is next to move and will guarantee victory in K
///    turns), K is always odd.
///
/// ```text
///       last                     next
/// turn  player  winner  outcome  player
/// ----  ------  ------  -------  ------
///    0  -       -       .        X
///    1  X       -       .        O
///   ..  .       -       .        .
///   ..  .       -       .        .
///   ..  .       -       .        .
///   30  O       -       Win X 5  X
///   31  X       -       Win X 4  O
///   32  O       -       Win X 3  X
///   33  X       -       Win X 2  O
///   34  O       -       Win X 1  X
///   35  X       X       Win X 0  -
/// ```
///
/// 2. If O reasons he cannot win or tie against an optimal opponent, then his
///    optimal sequence of moves will delay defeat as long as possible. For
///    example, look at the table above at turn 31; it is O's turn. The best O
///    can do is delay X's win by 4 moves, so the outcome of turn 31 is 'win by
///    X in 4'. Note that such a number must be even.
///
/// 3. If X's best move is to put O in a position so that a tie is optimal for O
///    (meaning that O can do no better than tie), then the outcome will be
///    'tie in 2', meaning that a tie is 2 plays away.
///
/// 4. If X is only willing/able to compute 10 moves, it is possible that a
///    solution will not be found. In such a situation, the optional next play
///    will be `None` and the outcome will be 'unknown for depth 10'.
#[derive(Clone, Copy, Debug)]
pub struct Solution {
    /// The optimal next play (optional)
    pub opt_play: Option<Play>,
    /// The resulting outcome
    pub outcome: Outcome,
}

// == solvers ==================================================================

impl Game {
    /// Returns the solution for depth == 0.
    fn solve_0(self) -> Solution {
        match self.state() {
            GameState::Won(player) => Solution {
                opt_play: None,
                outcome: Outcome::Win { player: player, turns: 0 },
            },
            GameState::Tied => Solution {
                opt_play: None,
                outcome: Outcome::Tie { turns: 0 },
            },
            GameState::Ongoing => Solution {
                opt_play: None,
                outcome: Outcome::Unknown { depth: 0 },
            },
        }
    }

    /// Returns the solution for depth == 1.
    #[allow(unused_variables)]
    fn solve_1(self) -> Solution {
        let solution = self.solve_0();
        match solution.outcome {
            Outcome::Win { .. } => solution,
            Outcome::Tie { .. } => solution,
            Outcome::Unknown { .. } => {
                let solutions = self.valid_plays().iter()
                    .map(|&play| self.play(play).unwrap().solve_0())
                    .collect::<Vec<Solution>>();
                if solutions.is_empty() {
                    panic!("Internal Error. The solutions vector is empty.");
                }
                let player = self.next_player().unwrap();
                best_solution_1(player, solutions)
            },
        }
    }

    /// Returns the solution for a given depth and game. This function looks
    /// ahead for a number of moves (specified by `depth`). If there are no
    /// valid moves, returns a solution where the optional play is `None` and
    /// the outcome is either a win or a tie.
    pub fn solve_for(self, depth: Count) -> Solution {
        match depth {
            0 => self.solve_0(),
            1 => self.solve_1(),
            2 => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

/// Returns the best solution for depth == 1.
fn best_solution_1(p: Player, solutions: Vec<Solution>) -> Solution {
    let mut ss = solutions;
    ss.sort_by(|a, b| Solution::compare(p, a, b));
    ss.first().unwrap().clone()
}

// == comparators ==============================================================

impl Outcome {
    /// Compare two outcomes from the point of view of the given player.
    #[allow(dead_code)]
    fn compare(p: Player, a: &Outcome, b: &Outcome) -> Ordering {
        let o = p.opponent();
        match (*a, *b) {
            (Outcome::Win { player: p1, turns: t1 },
             Outcome::Win { player: p2, turns: t2 }) => {
                if p1 == p && p2 == o { Ordering::Greater }
                else if p1 == o && p2 == p { Ordering::Less }
                else { t1.cmp(&t2) }
            },
            (Outcome::Win { player: p1, turns: _ },
             Outcome::Tie { turns: _ }) => {
                if p1 == p { Ordering::Greater }
                else { Ordering::Less }
            },
            (Outcome::Tie { turns: _ },
             Outcome::Win { player: p1, turns: _ }) => {
                if p1 == o { Ordering::Greater }
                else { Ordering::Less }
            },
            (Outcome::Win { player: p1, turns: _ },
             Outcome::Unknown { depth: _ }) => {
                if p1 == p { Ordering::Greater }
                else { Ordering::Less }
            },
            (Outcome::Unknown { depth: _ },
             Outcome::Win { player: p1, turns: _ }) => {
                if p1 == o { Ordering::Greater }
                else { Ordering::Less }
            },
            (Outcome::Tie { turns: t1 },
             Outcome::Tie { turns: t2 }) => t1.cmp(&t2),
            (Outcome::Tie { turns: _ },
             Outcome::Unknown { depth: _ }) => Ordering::Less,
            (Outcome::Unknown { depth: _ },
             Outcome::Tie { turns: _ }) => Ordering::Greater,
            (Outcome::Unknown { .. },
             Outcome::Unknown { .. }) => Ordering::Equal
        }
    }
}

impl Solution {
    /// Compare two solutions.
    #[allow(unused_variables)]
    fn compare(player: Player, a: &Solution, b: &Solution) -> Ordering {
        let o = player.opponent();
        Ordering::Equal
    }
}

// == cache ====================================================================

/// Constructs and returns a least-recently-used cache.
pub fn outcome_cache(capacity: usize) -> LruCache<Game, Outcome> {
    LruCache::<Game, Outcome>::with_capacity(capacity)
}
