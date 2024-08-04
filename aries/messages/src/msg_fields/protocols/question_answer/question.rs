use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{
    decorators::{thread::Thread, timing::Timing},
    msg_parts::MsgParts,
};

pub type Question = MsgParts<QuestionContent, QuestionDecorators>;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct QuestionContent {
    #[builder(default)]
    #[serde(default)]
    pub response_requested: bool,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, TypedBuilder)]
pub struct QuestionDecorators {
    #[builder(default, setter(strip_option))]
    #[serde(rename = "~thread")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<Thread>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "~timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{
        decorators::thread::tests::make_extended_thread, misc::test_utils,
        msg_types::question_answer::QuestionAnswerTypeV1_0,
    };

    #[test]
    fn test_minimal_ping() {
        let content = QuestionContent::default();

        let decorators = QuestionDecorators::default();

        let expected = json!({
            "response_requested": false,
        });

        test_utils::test_msg(
            content,
            decorators,
            QuestionAnswerTypeV1_0::Question,
            expected,
        );
    }

    #[test]
    fn test_extended_ping() {
        let content = QuestionContent::builder()
            .comment("test_comment".to_owned())
            .build();

        let decorators = QuestionDecorators::builder()
            .thread(make_extended_thread())
            .build();

        let expected = json!({
            "response_requested": false,
            "comment": content.comment,
            "~thread": decorators.thread
        });

        test_utils::test_msg(
            content,
            decorators,
            QuestionAnswerTypeV1_0::Question,
            expected,
        );
    }
}
