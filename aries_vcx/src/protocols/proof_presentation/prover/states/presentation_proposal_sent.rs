use messages::msg_fields::protocols::present_proof::propose::{
    PresentationPreview, ProposePresentation, ProposePresentationContent,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PresentationProposalSent {
    pub proposal: ProposePresentation,
}

impl Default for PresentationProposalSent {
    fn default() -> Self {
        let id = Uuid::new_v4().to_string();
        let preview = PresentationPreview::new(Vec::new(), Vec::new());

        let content = ProposePresentationContent::builder()
            .presentation_proposal(preview)
            .build();

        Self {
            proposal: ProposePresentation::builder().id(id).content(content).build(),
        }
    }
}

impl PresentationProposalSent {
    pub fn new(proposal: ProposePresentation) -> Self {
        Self { proposal }
    }
}
