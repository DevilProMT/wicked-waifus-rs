use shorekeeper_protocol::{
    FriendAllRequest, FriendAllResponse, ErrorCode, 
    
};

use crate::logic::player::Player;

pub fn on_friend_all_request(
    _player: &Player,
    _request: FriendAllRequest,
    response: &mut FriendAllResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.friend_info_list = vec![];
    response.friend_apply_list = vec![];
}