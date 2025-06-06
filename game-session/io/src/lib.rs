#![no_std]
#![allow(clippy::redundant_field_names)]

use gmeta::{InOut, Metadata, Out};
use gstd::{collections::HashMap, prelude::*, ActorId, MessageId};
use wordle_io::Event;

pub struct GameSessionMetadata;

impl Metadata for GameSessionMetadata {
    type Init = InOut<ActorId, SessionEvent>;
    type Handle = InOut<SessionAction, SessionEvent>;
    type Reply = InOut<Event, SessionEvent>;
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionAction {
    StartGame,
    CheckWord { word: String },
    CheckGameStatus { user: ActorId },
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
pub enum SessionEvent {
    Initialized,
    GameStarted,
    WordChecked {
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameOver {
        result: SessionResult,
    },
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
pub enum SessionResult {
    Ongoing,
    Win,
    Lose,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
pub enum SessionStatus {
    StartGameWaiting,
    StartGameSent,
    CheckWordWaiting,
    CheckWordSent,
    ReplyReceived(SessionEvent),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum StateQuery {
    WordleProgram,
    UserSession { user: ActorId },
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct GameSessionState {
    pub wordle_program: ActorId,
    pub user_to_session: HashMap<ActorId, Session>,
}

#[derive(Debug, Encode, Decode, TypeInfo, Default)]
pub struct State {
    pub wordle_program: ActorId,
    pub user_sessions: Vec<(ActorId, Session)>,
}

impl From<GameSessionState> for State {
    fn from(state: GameSessionState) -> Self {
        let GameSessionState {
            wordle_program,
            user_to_session,
        } = state;

        let user_sessions = user_to_session
            .iter()
            .map(|(user, session)| (*user, session.clone()))
            .collect();

        Self {
            wordle_program,
            user_sessions,
        }
    }
}

type SentMessageId = MessageId;
type OriginalMessageId = MessageId;

#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
pub struct Session {
    pub start_block: u32,
    pub check_count: u8,
    pub msg_ids: (SentMessageId, OriginalMessageId),
    pub status: SessionStatus,
    pub result: SessionResult,
}
