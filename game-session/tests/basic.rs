use game_session_io::*;
use gstd::ActorId;
use gtest::{Program, System};

const WORDLE_PROGRAM_ADDRESS: u64 = 200;

const USER: u64 = 110;
const BALANCE: u128 = 450000000000000;

#[test]
fn test_initialization() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ADDRESS.into();

    game_session_program.send(USER, wordle_program_address);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.wordle_program, wordle_program_address);
    assert!(state.user_sessions.is_empty());
}

#[test]
fn test_start_game() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ADDRESS.into();

    game_session_program.send(USER, wordle_program_address);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.wordle_program, wordle_program_address);
    assert!(state.user_sessions.is_empty());

    game_session_program.send(USER, SessionAction::StartGame);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.user_sessions.len(), 1);
}

#[test]
fn test_check_word() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ADDRESS.into();

    game_session_program.send(USER, wordle_program_address);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.wordle_program, wordle_program_address);
    assert!(state.user_sessions.is_empty());

    game_session_program.send(USER, SessionAction::StartGame);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.user_sessions.len(), 1);

    game_session_program.send(
        USER,
        SessionAction::CheckWord {
            word: "huuuu".to_string(),
        },
    );
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.user_sessions.len(), 1);
}

#[test]
fn test_game_win() {
    let system = System::new();
    system.init_logger();
    system.mint_to(USER, BALANCE);
    let game_session_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/game_session.opt.wasm",
    );
    let wordle_program = Program::from_file(
        &system,
        "../target/wasm32-unknown-unknown/debug/wordle.opt.wasm",
    );
    wordle_program.send_bytes(USER, []);
    system.run_next_block();

    let wordle_program_address: ActorId = WORDLE_PROGRAM_ADDRESS.into();

    game_session_program.send(USER, wordle_program_address);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.wordle_program, wordle_program_address);
    assert!(state.user_sessions.is_empty());

    game_session_program.send(USER, SessionAction::StartGame);
    system.run_next_block();

    let state: State = game_session_program.read_state(()).unwrap();
    assert_eq!(state.user_sessions.len(), 1);

    game_session_program.send(
        USER,
        SessionAction::CheckWord {
            word: "house".to_string(),
        },
    );
    system.run_next_block();
    let state: State = game_session_program.read_state(()).unwrap();
    println!("{:?}", state);
}
