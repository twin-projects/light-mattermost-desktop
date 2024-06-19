use std::fmt;
use std::fmt::Debug;

use models::*;

#[derive(Debug)]
pub enum ApiEvent {
    Login(String, String),
    MyTeams,
    MyTeamMembers,
    MyChannels,
    PostThreads(PostId),
    ChannelPosts(ChannelId),
    UserUnseen {
        channel_id: ChannelId,
        user_id: UserId,
    },
}

#[derive(Debug)]
pub enum Response {
    Login {
        token: AccessToken,
        user_id: String,
        user_name: String,
    },
    /// teams
    MyTeams(Vec<Team>),
    /// team members
    MyTeamMembers(Vec<TeamMember>),
    MyChannels(Vec<Channel>),
    ChannelThreads(PostThread),
    ChannelPosts(PostThread),
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
