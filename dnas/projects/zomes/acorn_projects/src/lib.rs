use hdk3::prelude::*;

mod project;

// use project::{
//   ArchiveGoalResponse, Edge, EntryPoint, EntryPointResponse, GetHistoryResponse, GetResponse, Goal,
//   GoalComment, GoalMaybeWithEdge, GoalMember, GoalVote, Member, ProjectMeta, GoalEdgeInput,
// };

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // Path::from(AGENTS_PATH).ensure()?;
    Ok(InitCallbackResult::Pass)
}

entry_defs!(ProjectMeta::)

    // project::projectmeta_def()
    // project::entry_point_def()
    // project::member_def()
    // project::edge_def()
    // project::goal_def()
    // project::goal_comment_def()
    // project::goal_member_def()
    // project::goal_vote_def()


// #[receive]
//   pub fn receive(from: Address, msg_json: JsonString) -> String {
//     hdk::debug(format!("New direct message from: {:?}", from)).ok();
//     let maybe_message: Result<DirectMessage, _> = JsonString::from_json(&msg_json).try_into();
//     match maybe_message {
//       Err(err) => format!("Err({})", err),
//       Ok(message) => {
//         signal_ui(&message);
//         String::from("Ok")
//       }
//     }
//   }

// these types will come straight through signals to the UI,
// so they will actually be referenced there. Be mindful of this

// pub const NEW_MEMBER_SIGNAL_TYPE: &str = "new_member";
// pub const ENTRY_POINT_SIGNAL_TYPE: &str = "entry_point";
// pub const ENTRY_POINT_ARCHIVED_SIGNAL_TYPE: &str = "entry_point_archived";
// pub const GOAL_MAYBE_WITH_EDGE_SIGNAL_TYPE: &str = "goal_maybe_with_edge";
// pub const GOAL_ARCHIVED_SIGNAL_TYPE: &str = "goal_archived";
// pub const EDGE_SIGNAL_TYPE: &str = "edge";
// pub const EDGE_ARCHIVED_SIGNAL_TYPE: &str = "edge_archived";
// pub const GOAL_COMMENT_SIGNAL_TYPE: &str = "goal_comment";
// pub const GOAL_COMMENT_ARCHIVED_SIGNAL_TYPE: &str = "goal_comment_archived";
// pub const GOAL_MEMBER_SIGNAL_TYPE: &str = "goal_member";
// pub const GOAL_MEMBER_ARCHIVED_SIGNAL_TYPE: &str = "goal_member_archived";
// pub const GOAL_VOTE_SIGNAL_TYPE: &str = "goal_vote";
// pub const GOAL_VOTE_ARCHIVED_SIGNAL_TYPE: &str = "goal_vote_archived";

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct NewMemberSignalPayload {
//   member: Member,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct EntryPointSignalPayload {
//   entry_point: EntryPointResponse,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct GoalMaybeWithEdgeSignalPayload {
//   goal: GoalMaybeWithEdge,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct EdgeSignalPayload {
//   edge: GetResponse<Edge>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct GoalArchivedSignalPayload {
//   archived: ArchiveGoalResponse,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct GoalCommentSignalPayload {
//   goal_comment: GetResponse<GoalComment>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct GoalMemberSignalPayload {
//   goal_member: GetResponse<GoalMember>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct GoalVoteSignalPayload {
//   goal_vote: GetResponse<GoalVote>,
// }

// // Used for GoalComment, GoalMember, GoalVote, and EntryPoint
// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct EntryArchivedSignalPayload {
//   address: Address,
// }

// /// Fully typed definition of the types of direct messages.
// /// All of which exist for the purposes of UI signals
// /// at this time.
// #[derive(Clone, Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
// pub(crate) enum DirectMessage {
//   NewMemberNotification(NewMemberSignalPayload),
//   EntryPointNotification(EntryPointSignalPayload),
//   GoalMaybeWithEdgeNotification(GoalMaybeWithEdgeSignalPayload),
//   EdgeNotification(EdgeSignalPayload),
//   EdgeArchivedNotification(EntryArchivedSignalPayload),
//   GoalArchivedNotification(GoalArchivedSignalPayload),
//   GoalCommentNotification(GoalCommentSignalPayload),
//   GoalMemberNotification(GoalMemberSignalPayload),
//   GoalVoteNotification(GoalVoteSignalPayload),
//   EntryPointArchivedNotification(EntryArchivedSignalPayload),
//   GoalCommentArchivedNotification(EntryArchivedSignalPayload),
//   GoalMemberArchivedNotification(EntryArchivedSignalPayload),
//   GoalVoteArchivedNotification(EntryArchivedSignalPayload),
// }

// // send a signal to the UI
// pub(crate) fn signal_ui(message: &DirectMessage) {
//   match message {
//     // Members
//     DirectMessage::NewMemberNotification(signal_payload) => {
//       hdk::emit_signal(NEW_MEMBER_SIGNAL_TYPE, signal_payload).ok();
//     }
//     // EntryPoints
//     DirectMessage::EntryPointNotification(signal_payload) => {
//       hdk::emit_signal(ENTRY_POINT_SIGNAL_TYPE, signal_payload).ok();
//     }
//     DirectMessage::EntryPointArchivedNotification(signal_payload) => {
//       hdk::emit_signal(ENTRY_POINT_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
//     }
//     // Edges
//     DirectMessage::EdgeNotification(signal_payload) => {
//       hdk::emit_signal(EDGE_SIGNAL_TYPE, signal_payload).ok();
//     }
//     DirectMessage::EdgeArchivedNotification(signal_payload) => {
//       hdk::emit_signal(EDGE_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
//     }
//     // Goals
//     DirectMessage::GoalMaybeWithEdgeNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_MAYBE_WITH_EDGE_SIGNAL_TYPE, signal_payload).ok();
//     }
//     DirectMessage::GoalArchivedNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
//     }
//     // Goal Comments
//     DirectMessage::GoalCommentNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_COMMENT_SIGNAL_TYPE, signal_payload).ok();
//     }
//     DirectMessage::GoalCommentArchivedNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_COMMENT_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
//     }
//     // Goal Members
//     DirectMessage::GoalMemberNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_MEMBER_SIGNAL_TYPE, signal_payload).ok();
//     }
//     DirectMessage::GoalMemberArchivedNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_MEMBER_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
//     }
//     // Goal Votes
//     DirectMessage::GoalVoteNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_VOTE_SIGNAL_TYPE, signal_payload).ok();
//     }
//     DirectMessage::GoalVoteArchivedNotification(signal_payload) => {
//       hdk::emit_signal(GOAL_VOTE_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
//     }
//   };
// }