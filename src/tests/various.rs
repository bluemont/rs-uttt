/// Various tests.

use data::*;
use constants::*;

// == game =====================================================================

#[test]
fn test_empty_game() {
    assert!(EMPTY_GAME == EMPTY_GAME);
    assert!(EMPTY_GAME.board == EMPTY_BOARD);
    assert!(EMPTY_GAME.last_loc == None);
    assert!(EMPTY_GAME.last_player() == None);
}

#[test]
fn test_game_play() {
    let opt_game = EMPTY_GAME.play(Play {
        loc: Loc::new(RI::R6, CI::C3),
        player: Player::X,
    });
    assert!(opt_game != None);
    assert!(opt_game != Some(EMPTY_GAME));
}

#[test]
fn test_game_last_player() {
    assert!(EMPTY_GAME.last_player() == None);
    // TODO: test an example game
}

// == board ====================================================================

#[test]
fn test_empty_board() {
    assert!(EMPTY_BOARD == EMPTY_BOARD);
    assert!(EMPTY_BOARD.sboards == [
        EMPTY_SBOARD, EMPTY_SBOARD, EMPTY_SBOARD,
        EMPTY_SBOARD, EMPTY_SBOARD, EMPTY_SBOARD,
        EMPTY_SBOARD, EMPTY_SBOARD, EMPTY_SBOARD,
    ]);
    // Note: comparing slices (below) is a work-around, since Rust does not
    // currently allow direct comparison of arrays bigger than 32 elements:
    // https://github.com/rust-lang/rfcs/issues/1038
    assert!(&EMPTY_BOARD.slots()[..] == &[
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
        SE, SE, SE, SE, SE, SE, SE, SE, SE,
    ][..]);
    assert!(EMPTY_BOARD.slots_9x9() == [
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
        [SE, SE, SE, SE, SE, SE, SE, SE, SE],
    ]);
}

// == sub-board ================================================================

#[test]
fn test_empty_sboard() {
    let sb = EMPTY_SBOARD;
    assert!(sb.row_at(SRI::R0) == EMPTY_ROW);
    assert!(sb.row_at(SRI::R1) == EMPTY_ROW);
    assert!(sb.row_at(SRI::R2) == EMPTY_ROW);
    assert!(sb.rows() == [EMPTY_ROW, EMPTY_ROW, EMPTY_ROW]);
    assert!(sb.slots() == [
        SE, SE, SE,
        SE, SE, SE,
        SE, SE, SE
    ]);
    assert!(sb.slots_3x3() == [
        [SE, SE, SE],
        [SE, SE, SE],
        [SE, SE, SE],
    ]);
}

#[test]
fn test_sboard_from_slots() {
    assert!(SBoard::from_slots([
        SE, SE, SX,
        SO, SX, SE,
        SE, SE, SE,
    ]) == SBoard { encoding: 0b0000001010100001 });
    assert!(SBoard::from_slots([
        SE, SE, SE,
        SE, SE, SX,
        SO, SX, SE,
    ]) == SBoard { encoding: 0b0101010000100000 });
}

// == rows =====================================================================

// == row ======================================================================

#[test]
fn test_empty_row() {
    let er = EMPTY_ROW;
    assert!(er.slots() == [SE, SE, SE]);
}

// == board play ===============================================================

// == sub-board play ===========================================================

// == board location ===========================================================

// == sub-board location =======================================================

// == slots ====================================================================

// == slot =====================================================================

// == board indexes ============================================================

// == sub-board indexes ========================================================

// == player ===================================================================

// == u8 =======================================================================

// == bool =====================================================================
