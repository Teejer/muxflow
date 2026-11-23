//! The assistant engine, out of the desktop app and where a host can reach it.
//!
//! This is the conversation half of the assistant: the OpenAI-compatible
//! stream reader, the round budget, the tool table, the approval policy, and
//! the pane-resolution helpers — everything except who owns the settings, the
//! context snapshot, and the UI. The desktop panel ([`muxflow_gpui::ai`]) is
//! one client. The plan is a second: the host runs a conversation on behalf of
//! a thin client — the phone — that sends a question and receives the stream,
//! so the endpoint, the key, and the tools stay where the tmux server is and
//! the client stays a proxy's client. Hence the name.
//!
//! Nothing here knows about gpui, connections, or storage. [`engine`] talks to
//! a configured endpoint over blocking threads; [`tools`] describes and gates
//! the calls a model may make and resolves them against a [`tmux_control`]
//! snapshot, leaving execution to the caller that holds the panes.

pub mod engine;
pub mod tools;
