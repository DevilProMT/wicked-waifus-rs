use shorekeeper_protocol::{
    GetFormationDataRequest, GetFormationDataResponse, ErrorCode
};

use crate::logic::player::Player;

pub fn on_get_formation_data_request(
    _player: &Player,
    _request: GetFormationDataRequest,
    response: &mut GetFormationDataResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.formations = vec![];
}
