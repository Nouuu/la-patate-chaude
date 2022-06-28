use serde::{Deserialize, Serialize};

use crate::{
    public_player::PublicPlayer,
    challenge::{MD5HashCash, ChallengeAnswer, ReportedChallengeResult},
    subscribe::SubscribeResult
};

#[derive(Serialize, Deserialize, Debug)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput),
    RecoverSecret(RecoverSecretOutput),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportedChallengeResult {
    pub name: String,
    pub value: ChallengeValue,
}

pub type PublicLeaderBoard = Vec<PublicPlayer>;

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretInput {
    pub letters: String,
    pub tuple_sizes: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecretOutput {
    pub secret_sentence: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashOutput {
    pub seed: u64,
    pub hashcode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCashInput {
    pub(crate) complexity: u32,
    pub(crate) message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecoverSecret(pub(crate) RecoverSecretInput);

#[derive(Serialize, Deserialize, Debug)]
pub struct MD5HashCash(pub(crate) MD5HashCashInput);

#[derive(Serialize, Deserialize, Debug)]
pub enum ChallengeType {
    MD5HashCash(MD5HashCash),
    RecoverSecret(RecoverSecret),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Hello,
    Welcome {
        version: u8,
    },
    Subscribe {
        name: String,
    },
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(ChallengeType),
    ChallengeResult {
        answer: ChallengeAnswer,
        next_target: String,
    },
    RoundSummary {
        challenge: String,
        chain: Vec<ReportedChallengeResult>,
    },
  StartGame {},
    EndOfGame {
        leader_board: Vec<PublicPlayer>,
    },
  EndOfCommunication,
}

#[derive(Debug, Clone)]
pub enum ResponseType {
  Broadcast,
  Unicast,
}

#[derive(Debug, Clone)]
pub struct MessageType {
    pub message: Message,
    pub message_type: ResponseType,
}

impl MessageType {
    pub fn boardcast(message: Message) -> MessageType {
        MessageType { message, message_type: ResponseType::Broadcast }
    }
    pub fn unicast(message: Message) -> MessageType {
        MessageType { message, message_type: ResponseType::Unicast }
    }
    
}

#[cfg(test)]
mod tests {
    use crate::subscribe::SubscribeError;

    use super::*;

    #[test]
    fn test_message_hello_serialization() {
        let message = Message::Hello;
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "\"Hello\"");
    }

    #[test]
    fn test_welcome_serialization() {
        let message = Message::Welcome { version: 1 };
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"Welcome\":{\"version\":1}}");
    }

    #[test]
    fn test_subscribe_serialization() {
        let message = Message::Subscribe {
            name: "test".to_string(),
        };
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"Subscribe\":{\"name\":\"test\"}}");
    }

    #[test]
    fn test_subscribe_result_success_serialization() {
        let message = Message::SubscribeResult(SubscribeResult::Ok);
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(serialized, "{\"SubscribeResult\":\"Ok\"}");
    }

    #[test]
    fn test_subscribe_result_failure_serialization() {
        let message = Message::SubscribeResult(SubscribeResult::Err(SubscribeError::InvalidName));
        let serialized = serde_json::to_string(&message).unwrap();
        assert_eq!(
            serialized,
            "{\"SubscribeResult\":{\"Err\":\"InvalidName\"}}"
        );
    }
}
