use std::time::{Duration, Instant};
use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

// heuristic estimates how good the board is when we can't search all the way to the end
// positive score favors X, negative favors O
fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();
    let mut score: i32 = board.score() * 100; // multiply by 100 so completed triplets always outweigh partial bonuses

    let center = (n / 2) as i32;
    for i in 0..n {
        for j in 0..n {
            let dist = (i as i32 - center).abs() + (j as i32 - center).abs();
            // center pieces rewarded more because they can contribute to more triplets and quads in more directions than edge pieces
            Cell::X => score += position_bonus,
            let position_bonus = n as i32 - dist;
            match &cells[i][j] {
                Cell::X => score += position_bonus,
                Cell::O => score -= position_bonus,
                _ => {}
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            // scan in 4 directions for triplets: right, down, down-right diagonal, down-left diagonal
            let dirs: &[(i32, i32)] = &[(0, 1), (1, 0), (1, 1), (1, -1)];
            for (di, dj) in dirs {
                // calculate the coordinates of the 2nd and 3rd cells in the window
                // by stepping once and twice in the current direction
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                let i3 = i as i32 + 2 * di;
                let j3 = j as i32 + 2 * dj;

                // make sure all 3 cells in the window are actually on the board before accessing them
                // cells near the edge may step off the board when we move in a direction
                if i2 < 0 || j2 < 0 || i3 < 0 || j3 < 0 {
                    continue;
                }
                if i2 >= n as i32 || j2 >= n as i32 || i3 >= n as i32 || j3 >= n as i32 {
                    continue;
                }

                // get the 3 cells in this window and evaluate the
                let a = &cells[i][j];
                let b = &cells[i2 as usize][j2 as usize];
                let c = &cells[i3 as usize][j3 as usize];

                // add the window score to the total heuristic score
                score += eval_window(a, b, c);
            }
        }
    }

    score
}

fn eval_window(a: &Cell, b: &Cell, c: &Cell) -> i32 {
    let cells = [a, b, c];
    let x_count = cells.iter().filter(|&&c| c == &Cell::X).count();
    let o_count = cells.iter().filter(|&&c| c == &Cell::O).count();
    let empty_count = cells.iter().filter(|&&c| c == &Cell::Empty).count();

    // if both players have pieces in this window, neither can complete a triplet here
    if x_count > 0 && o_count > 0 {
        return 0;
    }

    // if the window contains a wall, it can never become a triplet
    if x_count + o_count + empty_count < 3 {
        return 0;
    }

    // two in a row with an open end is high priority — one move from completing a triplet
    if x_count == 2 && empty_count == 1 {
        return 20;
    }
    if o_count == 2 && empty_count == 1 {
        return -20;
    }

    // single piece with open space has some value but much less than two in a row
    // values kept below 20 and 100 to maintain hierarchy: triplet > two-in-a-row > single piece
    if x_count == 1 && empty_count == 2 {
        return 3;
    }
    if o_count == 1 && empty_count == 2 {
        return -3;
    }

    0
}

fn minimax(
    board: &mut Board,
    player: Player,
    depth: u32, // how many more levels to search
    mut alpha: i32, // best score X has found so far
    mut beta: i32, // best score O has found so far
    deadline: &Instant, // when we started, used to check if time is running out
    // returns Option so we can return None if we run out of time mid-search
    // Some(result) means search completed, None means time ran out
) -> Option<(i32, usize, usize)> {
    // If we're out of time, return None to signal the caller to stop
    // stop at 1800ms instead of 2000ms to leave a buffer for returning results back up the call stack
    if deadline.elapsed() > Duration::from_millis(1800) {
        return None;
    }

    if board.game_over() {
        return Some((board.score(), 0, 0));
    }

    // reached our search depth limit — can't look any further ahead
    // use heuristic to estimate the board value instead of knowing the exact outcome
    if depth == 0 {
        return Some((heuristic(board), 0, 0));
    }

    let moves = board.moves();
    let mut best_move = moves[0]; // initialize to first available move as a safe default — guaranteed to exist since game is not over
    // X starts at MIN and O starts at MAX so any real score (1, 0, -1) will replace the initial value
    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    for m in moves {
        board.apply_move(m, player);
        let result = minimax(board, player.flip(), depth - 1, alpha, beta, deadline); // flip to opponent's turn after each move
        board.undo_move(m, player); // undo instead of cloning the board for efficiency — modifies in place and restores

        // If we ran out of time mid-search, propagate None up
        let (score, _, _) = match result {
            Some(r) => r,
            None => return None,
        };

        match player {
            Player::X => {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }
                
                // update alpha — best score X has found, X will never accept lower
                if best_score > alpha {
                    alpha = best_score;
                }
            }
            Player::O => {
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }

                // update beta — best score O has found, O will never accept higher
                if best_score < beta {
                    beta = best_score;
                }
            }
        }

        // if alpha >= beta, O would never allow this branch since X already has a better option
        // no need to search further — prune this branch
        if alpha >= beta {
            break;
        }
    }

    Some((best_score, best_move.0, best_move.1)) // return (score, row, column) of the best move found
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, time_limit: u64) -> (i32, usize, usize) {
        let deadline = Instant::now(); // record start time so we can track how much time has elapsed during search
        let limit = Duration::from_millis(time_limit);
        let cutoff = limit.mul_f64(0.9); // use 90% of time limit as cutoff to leave buffer for returning results

        let mut best = (0, 0, 0);

        // Iterative deepening: try depth 1, 2, 3... until time runs out
        for depth in 1.. {
            match minimax(board, player, depth, i32::MIN, i32::MAX, &deadline) {
                Some(result) => {
                    best = result; // save completed result — if next depth runs out of time we can still return this
                    // If we finished this depth and time is already close, stop
                    if deadline.elapsed() >= cutoff {
                        break;
                    }
                }
                // Ran out of time mid-search, use best result from previous depth
                None => break,
            }
        }

        best
    }
}
