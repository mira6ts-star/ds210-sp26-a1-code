use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::{Board, Cell};
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

const MAX_DEPTH: u32 = 5;

fn heuristic(board: &Board) -> i32 {
    let cells = board.get_cells();
    let n = cells.len();
    let mut score: i32 = board.score() * 100;

    let center = (n / 2) as i32;
    for i in 0..n {
        for j in 0..n {
            let dist = (i as i32 - center).abs() + (j as i32 - center).abs();
            let position_bonus = (n as i32 - dist);
            match &cells[i][j] {
                Cell::X => score += position_bonus,
                Cell::O => score -= position_bonus,
                _ => {}
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            let dirs: &[(i32, i32)] = &[(0, 1), (1, 0), (1, 1), (1, -1)];
            for (di, dj) in dirs {
                let i2 = i as i32 + di;
                let j2 = j as i32 + dj;
                let i3 = i as i32 + 2 * di;
                let j3 = j as i32 + 2 * dj;

                if i2 < 0 || j2 < 0 || i3 < 0 || j3 < 0 {
                    continue;
                }
                if i2 >= n as i32 || j2 >= n as i32 || i3 >= n as i32 || j3 >= n as i32 {
                    continue;
                }

                let a = &cells[i][j];
                let b = &cells[i2 as usize][j2 as usize];
                let c = &cells[i3 as usize][j3 as usize];

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

    if x_count > 0 && o_count > 0 {
        return 0;
    }
    if x_count + o_count + empty_count < 3 {
        return 0;
    }

    if x_count == 2 && empty_count == 1 {
        return 20;  // X about to complete - high priority
    }
    if o_count == 2 && empty_count == 1 {
        return -20; // Block O - equally high priority
    }
    if x_count == 1 && empty_count == 2 {
        return 3;
    }
    if o_count == 1 && empty_count == 2 {
        return -3;
    }

    0
}

fn minimax(board: &mut Board, player: Player, depth: u32, mut alpha: i32, mut beta: i32) -> (i32, usize, usize) {
    if board.game_over() {
        return (board.score(), 0, 0);
    }

    if depth == 0 {
        return (heuristic(board), 0, 0);
    }

    let moves = board.moves();
    let mut best_move = moves[0];
    let mut best_score = match player {
        Player::X => i32::MIN,
        Player::O => i32::MAX,
    };

    for m in moves {
        board.apply_move(m, player);
        let (score, _, _) = minimax(board, player.flip(), depth - 1, alpha, beta);
        board.undo_move(m, player);

        match player {
            Player::X => {
                if score > best_score {
                    best_score = score;
                    best_move = m;
                }
                if best_score > alpha {
                    alpha = best_score;
                }
            }
            Player::O => {
                if score < best_score {
                    best_score = score;
                    best_move = m;
                }
                if best_score < beta {
                    beta = best_score;
                }
            }
        }

        // Prune: this branch can't possibly affect the result
        if alpha >= beta {
            break;
        }
    }

    (best_score, best_move.0, best_move.1)
}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        minimax(board, player, MAX_DEPTH, i32::MIN, i32::MAX)
    }
}