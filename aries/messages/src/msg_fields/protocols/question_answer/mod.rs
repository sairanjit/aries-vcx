//! Module containing the `trust ping` protocol messages, as defined in the [RFC](<https://github.com/hyperledger/aries-rfcs/blob/main/features/0048-trust-ping/README.md>).

pub mod answer;
pub mod question;

use derive_more::From;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use self::{
    answer::{Answer, AnswerContent, AnswerDecorators},
    question::{Question, QuestionContent, QuestionDecorators},
};

use crate::{
    misc::utils::{into_msg_with_type, transit_to_aries_msg},
    msg_fields::traits::DelayedSerde,
    msg_types::{
        protocols::question_answer::{
            QuestionAnswerType as QuestionAnswerKind, QuestionAnswerTypeV1, QuestionAnswerTypeV1_0,
        },
        MsgWithType,
    },
};

#[derive(Clone, Debug, From, PartialEq)]
pub enum QuestionAnswer {
    Question(Question),
    Answer(Answer),
}

impl DelayedSerde for QuestionAnswer {
    type MsgType<'a> = (QuestionAnswerKind, &'a str);

    fn delayed_deserialize<'de, D>(
        msg_type: Self::MsgType<'de>,
        deserializer: D,
    ) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (protocol, kind_str) = msg_type;

        let kind = match protocol {
            QuestionAnswerKind::V1(QuestionAnswerTypeV1::V1_0(kind)) => {
                kind.kind_from_str(kind_str)
            }
        };

        match kind.map_err(D::Error::custom)? {
            QuestionAnswerTypeV1_0::Question => Question::deserialize(deserializer).map(From::from),
            QuestionAnswerTypeV1_0::Answer => Answer::deserialize(deserializer).map(From::from),
        }
    }

    fn delayed_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Question(v) => MsgWithType::from(v).serialize(serializer),
            Self::Answer(v) => MsgWithType::from(v).serialize(serializer),
        }
    }
}

transit_to_aries_msg!(QuestionContent: QuestionDecorators, QuestionAnswer);
transit_to_aries_msg!(AnswerContent: AnswerDecorators, QuestionAnswer);

into_msg_with_type!(Question, QuestionAnswerTypeV1_0, Question);
into_msg_with_type!(Answer, QuestionAnswerTypeV1_0, Answer);
