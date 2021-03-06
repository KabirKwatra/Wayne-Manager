//    Copyright (C) 2020  Kabir Kwatra
//    Full notice in main.rs file

use serenity::model::voice::VoiceState;

/// A Wayne Event
pub enum Event {
    WayneJoin,
    WayneLeave,
    WayneMove,
    NonWayne,
}

/// Detects and returns the Wayne Event described by the provided states
///
/// # Arguments
///
/// * `old` - An optional voice state describing the voice channel before the event.
/// * `new_state` - The new voice state describing the voice channel after the event.
/// * `wayne_id` - Wayne's discord id.
pub fn event(old: &Option<VoiceState>, new_state: &VoiceState, wayne_id: &u64) -> Event {
    if new_state.user_id.as_u64() != wayne_id {
        return Event::NonWayne;
    }

    if old.is_none() {
        return Event::WayneJoin;
    }
    let old_state = old.clone().expect("Unexpected None Old Voice State");

    if new_state.channel_id.is_none() {
        return Event::WayneLeave;
    }
    if old_state
        .channel_id
        .expect("Unexpected None Old State Channel ID")
        != new_state
            .channel_id
            .expect("Unexpected None New State Channel ID")
    {
        return Event::WayneMove;
    }

    Event::NonWayne
}
