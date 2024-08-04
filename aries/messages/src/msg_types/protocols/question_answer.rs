use derive_more::From;
use messages_macros::MessageType;
use strum_macros::{AsRefStr, EnumString};
use transitive::Transitive;

use super::Protocol;
use crate::msg_types::{role::Role, MsgKindType};

#[derive(Copy, Clone, Debug, From, PartialEq, MessageType)]
#[msg_type(protocol = "questionanswer")]
pub enum QuestionAnswerType {
    V1(QuestionAnswerTypeV1),
}

#[derive(Copy, Clone, Debug, From, PartialEq, Transitive, MessageType)]
#[transitive(into(QuestionAnswerType, Protocol))]
#[msg_type(major = 1)]
pub enum QuestionAnswerTypeV1 {
    #[msg_type(minor = 0, roles = "Role::Questioner, Role::Responder")]
    V1_0(MsgKindType<QuestionAnswerTypeV1_0>),
}

#[derive(Copy, Clone, Debug, AsRefStr, EnumString, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum QuestionAnswerTypeV1_0 {
    Question,
    Answer,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::misc::test_utils;

    #[test]
    fn test_protocol_question_answer() {
        test_utils::test_serde(
            Protocol::from(QuestionAnswerTypeV1::new_v1_0()),
            json!("https://didcomm.org/questionanswer/1.0"),
        )
    }

    #[test]
    fn test_version_resolution_question_answer() {
        test_utils::test_msg_type_resolution(
            "https://didcomm.org/questionanswer/1.255",
            QuestionAnswerTypeV1::new_v1_0(),
        )
    }

    #[test]
    #[should_panic]
    fn test_unsupported_version_question_answer() {
        test_utils::test_serde(
            Protocol::from(QuestionAnswerTypeV1::new_v1_0()),
            json!("https://didcomm.org/questionanswer/2.0"),
        )
    }

    #[test]
    fn test_msg_type_question() {
        test_utils::test_msg_type(
            "https://didcomm.org/questionanswer/1.0",
            "question",
            QuestionAnswerTypeV1::new_v1_0(),
        )
    }

    #[test]
    fn test_msg_type_answer() {
        test_utils::test_msg_type(
            "https://didcomm.org/questionanswer/1.0",
            "answer",
            QuestionAnswerTypeV1::new_v1_0(),
        )
    }
}
