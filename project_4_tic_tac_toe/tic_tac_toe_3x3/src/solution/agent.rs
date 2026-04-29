use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

pub struct SolutionAgent {}

impl Agent for SolutionAgent {
    fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
        if board.game_over() {
            return (board.score(), 0, 0); // base case: game is over, someone has won, return final score. Move coordinates don't matter here
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
            let (score, _, _) = SolutionAgent::solve(board, player.flip()); // flip to opponent's turn after each move
            board.undo_move(m, player); // undo instead of cloning the board for efficiency — modifies in place and restores

            match player {
                Player::X => {
                    if score > best_score {
                        best_score = score;
                        best_move = m;
                    }
                }
                Player::O => {
                    if score < best_score {
                        best_score = score;
                        best_move = m;
                    }
                }
            }
        }
        

        (best_score, best_move.0, best_move.1) // return (score, row, column) of the best move found
    }
}
