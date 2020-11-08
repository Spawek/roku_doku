use crate::Board;

pub struct ResolveResult {
    pub board: Board,
    pub blocks_removed: i32,
}

pub fn resolve_board(board: Board) -> ResolveResult {
    let mut resolved_board = board.clone();
    let mut blocks_removed = 0;

    blocks_removed = blocks_removed + resolve_rows(board, &mut resolved_board);
    blocks_removed = blocks_removed + resolve_columns(board, &mut resolved_board);
    blocks_removed = blocks_removed + resolve_blocks(board, &mut resolved_board);

    ResolveResult {
        board: resolved_board,
        blocks_removed,
    }
}

fn resolve_rows(base_board: Board, resolved_board: &mut Board) -> i32 {
    let mut blocks_removed = 0;

    for y in 0..9 {
        let mut full_row = true;
        for x in 0..9 {
            if !base_board[x][y] {
                full_row = false;
                break;
            }
        }

        if full_row {
            blocks_removed = blocks_removed + 1;

            for x in 0..9 {
                resolved_board[x][y] = false;
            }
        }
    }

    blocks_removed
}

fn resolve_columns(base_board: Board, resolved_board: &mut Board) -> i32 {
    let mut blocks_removed = 0;

    for x in 0..9 {
        let mut full_col = true;
        for y in 0..9 {
            if !base_board[x][y] {
                full_col = false;
                break;
            }
        }

        if full_col {
            blocks_removed = blocks_removed + 1;

            for y in 0..9 {
                resolved_board[x][y] = false;
            }
        }
    }

    blocks_removed
}

fn resolve_blocks(base_board: Board, resolved_board: &mut Board) -> i32 {
    let mut blocks_removed = 0;

    for block_x in 0..3 {
        for block_y in 0..3 {
            let mut full_block = true;
            for x in 0..3 {
                for y in 0..3 {
                    if !base_board[block_x * 3 + x][block_y * 3 + y] {
                        full_block = false;
                        break;
                    }
                }
            }

            if full_block {
                blocks_removed = blocks_removed + 1;

                for x in 0..3 {
                    for y in 0..3 {
                        resolved_board[block_x * 3 + x][block_y * 3 + y] = false;
                    }
                }
            }
        }
    }

    blocks_removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_empty_board() {
        let ret = resolve_board(Board::default());
        assert_eq!(ret.blocks_removed, 0);

        for x in 0..9 {
            for y in 0..9 {
                assert_eq!(ret.board[x][y], false);
            }
        }
    }

    #[test]
    fn test_resolve_row() {
        let mut board = Board::default();
        for x in 0..9 {
            board[x][4] = true;
        }

        let ret = resolve_board(board);
        assert_eq!(ret.blocks_removed, 1);
        for x in 0..9 {
            assert_eq!(ret.board[x][4], false);
        }
    }

    #[test]
    fn test_resolve_2_rows() {
        let mut board = Board::default();
        for x in 0..9 {
            board[x][4] = true;
            board[x][7] = true;
        }

        let ret = resolve_board(board);
        assert_eq!(ret.blocks_removed, 2);
        for x in 0..9 {
            assert_eq!(ret.board[x][4], false);
            assert_eq!(ret.board[x][7], false);
        }
    }

    #[test]
    fn test_resolve_column() {
        let mut board = Board::default();
        for y in 0..9 {
            board[3][y] = true;
        }

        let ret = resolve_board(board);
        assert_eq!(ret.blocks_removed, 1);
        for y in 0..9 {
            assert_eq!(ret.board[3][y], false);
        }
    }

    #[test]
    fn test_resolve_2_columns() {
        let mut board = Board::default();
        for y in 0..9 {
            board[3][y] = true;
            board[5][y] = true;
        }

        let ret = resolve_board(board);
        assert_eq!(ret.blocks_removed, 2);
        for y in 0..9 {
            assert_eq!(ret.board[3][y], false);
            assert_eq!(ret.board[5][y], false);
        }
    }

    #[test]
    fn test_resolve_block() {
        let mut board = Board::default();
        for x in 3..6 {
            for y in 3..6 {
                board[x][y] = true;
            }
        }

        let ret = resolve_board(board);
        assert_eq!(ret.blocks_removed, 1);
        for x in 3..6 {
            for y in 3..6 {
                assert_eq!(ret.board[x][y], false);
            }
        }
    }

    #[test]
    fn test_resolve_column_row_and_block() {
        let mut board = Board::default();
        for x in 3..6 {
            for y in 3..6 {
                board[x][y] = true;
            }
        }

        for x in 0..9 {
            board[x][2] = true;
        }

        for y in 0..9 {
            board[1][y] = true;
        }

        let ret = resolve_board(board);
        assert_eq!(ret.blocks_removed, 3);
        for x in 0..9 {
            for y in 0..9 {
                assert_eq!(ret.board[x][y], false);
            }
        }
    }
}
