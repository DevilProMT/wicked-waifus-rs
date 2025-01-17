use shorekeeper_protocol::{
    ErrorCode, Zih, InputSettingRequest, InputSettingResponse, InputSettingUpdateRequest,
    InputSettingUpdateResponse, LanguageSettingUpdateRequest, LanguageSettingUpdateResponse,
    ServerPlayStationPlayOnlyStateRequest, ServerPlayStationPlayOnlyStateResponse, VersionInfoPush,
    AccessPathTimeServerConfigRequest,  AccessPathTimeServerConfigResponse,
    GetDetectionLabelInfoRequest, GetDetectionLabelInfoResponse, EnergySyncRequest, EnergySyncResponse,
    TutorialInfoRequest, TutorialInfoResponse, MonthCardRequest, MonthCardResponse,
    InfluenceInfoRequest, InfluenceInfoResponse, ForgeInfoRequest, ForgeInfoResponse,
    AchievementInfoRequest, AchievementInfoResponse, ExchangeRewardRequest, ExchangeRewardResponse,
    LivenessRequest, LivenessResponse, WebSignRequest, WebSignResponse, 
    PhotoMemoryRequest, PhotoMemoryResponse, WeaponSkinRequest, WeaponSkinResponse,
    VisionEquipGroupInfoRequest, VisionEquipGroupInfoResponse, UpdatePlayStationBlockAccountRequest, UpdatePlayStationBlockAccountResponse,
    AdventureManualRequest, AdventureManualResponse, LordGymInfoRequest, LordGymInfoResponse, 
    MapTraceInfoRequest, MapTraceInfoResponse, TowerRequest, TowerResponse,
    ExploreProgressRequest, ExploreProgressResponse, ReportDataRequest, ReportDataResponse,
    UpdateVoxelEnvRequest, UpdateVoxelEnvResponse, SimpleTrackReportAsyncRequest, SimpleTrackReportAsyncResponse,
    TowerSeasonUpdateRequest, TowerSeasonUpdateResponse
};

use crate::logic::player::Player;
use std::collections::HashMap;

pub fn on_input_setting_request(
    _: &Player,
    _: InputSettingRequest,
    response: &mut InputSettingResponse,
) {
    response.zih = Some(Zih::default());
}

pub fn on_input_setting_update_request(
    _: &Player,
    _: InputSettingUpdateRequest,
    response: &mut InputSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_language_setting_update_request(
    _: &Player,
    _: LanguageSettingUpdateRequest,
    response: &mut LanguageSettingUpdateResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_server_play_station_play_only_state_request(
    _: &Player,
    _: ServerPlayStationPlayOnlyStateRequest,
    response: &mut ServerPlayStationPlayOnlyStateResponse,
) {
    response.cross_play_enabled = false;
}


pub fn on_access_path_time_server_config_request(
    _: &Player,
    _: AccessPathTimeServerConfigRequest,
    response: &mut AccessPathTimeServerConfigResponse,
) {
    response.access_path_time_server_config = vec![];
}

pub fn on_energy_sync_request(
    _: &Player,
    _: EnergySyncRequest,
    response: &mut EnergySyncResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.sync_info = vec![];
}

pub fn on_get_detection_label_info_request(
    _: &Player,
    _: GetDetectionLabelInfoRequest,
    response: &mut GetDetectionLabelInfoResponse,
) {
    response.unlock_label_info = None;
}

pub fn on_tutorial_info_request(
    _: &Player,
    _: TutorialInfoRequest,
    response: &mut TutorialInfoResponse,
) {
    response.unlock_list = vec![];
}

pub fn on_month_card_request(
    _: &Player,
    _: MonthCardRequest,
    response: &mut MonthCardResponse,
) {
    response.days = 0;
    response.is_daily_got = false;
    response.error_code = ErrorCode::Success.into();
}

pub fn on_influence_info_request(
    _: &Player,
    _: InfluenceInfoRequest,
    response: &mut InfluenceInfoResponse,
) {
    response.mws = vec![];
}

pub fn on_forge_info_request(
    _: &Player,
    _: ForgeInfoRequest,
    response: &mut ForgeInfoResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.forge_info_list = vec![];
    response.forge_configs = vec![];
    response.limit_refresh_time = 0;
}

pub fn on_achievement_info_request(
    _: &Player,
    _: AchievementInfoRequest,
    response: &mut AchievementInfoResponse,
) {
    response.achievement_group_info_list = vec![];
    response.achievement_finished_star = 0;
    response.finished_achievement_num = 0;
}

pub fn on_exchange_reward_request(
    _: &Player,
    _: ExchangeRewardRequest,
    response: &mut ExchangeRewardResponse,
) {
    response.exchange_share_data = HashMap::new();
    response.exchange_reward_data = HashMap::new();
}

pub fn on_liveness_request(
    _: &Player,
    _: LivenessRequest,
    response: &mut LivenessResponse,
) {
    response.liveness_info = None;
}

pub fn on_web_sign_request(
    _: &Player,
    _: WebSignRequest,
    _response: &mut WebSignResponse,
) {}

pub fn on_photo_memory_request(
    _: &Player,
    _: PhotoMemoryRequest,
    response: &mut PhotoMemoryResponse,
) {
    response.item = vec![];
}

pub fn on_weapon_skin_request(
    _: &Player,
    _: WeaponSkinRequest,
    response: &mut WeaponSkinResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.equip_list = vec![];
}

pub fn on_vision_equip_group_info_request(
    _: &Player,
    _: VisionEquipGroupInfoRequest,
    response: &mut VisionEquipGroupInfoResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.vision_equip_list = vec![];
}

pub fn on_update_play_station_block_account_request(
    _: &Player,
    _: UpdatePlayStationBlockAccountRequest,
    _: &mut UpdatePlayStationBlockAccountResponse,
) {}

pub fn on_adventure_manual_request(
    _: &Player,
    _: AdventureManualRequest,
    response: &mut AdventureManualResponse,
) {
    response.error_code = ErrorCode::Success.into();
}

pub fn on_lord_gym_info_request(
    _: &Player,
    _: LordGymInfoRequest,
    response: &mut LordGymInfoResponse,
) {
    response.unlock_load_gym_ids = vec![];
    response.read_load_gym_ids = vec![];
    response.lord_gym_pass_records = vec![];
}

pub fn on_map_trace_info_request(
    _: &Player,
    _: MapTraceInfoRequest,
    response: &mut MapTraceInfoResponse,
) {
    response.error_code = 0;
    response.mark_id_list = vec![];
}

pub fn on_tower_request(
    _: &Player,
    _: TowerRequest,
    response: &mut TowerResponse,
) {
    response.tower_info = None;
}

pub fn on_explore_progress_request(
    _: &Player,
    _: ExploreProgressRequest,
    response: &mut ExploreProgressResponse,
) {
    response.area_progress = vec![];
}

pub fn on_report_data_request(
    _: &Player,
    _: ReportDataRequest,
    response: &mut ReportDataResponse,
) {
    response.error = None;
}

pub fn on_update_voxel_env_request(
    _: &Player,
    _: UpdateVoxelEnvRequest,
    response: &mut UpdateVoxelEnvResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.server_cave_mode = 0;
}

pub fn on_simple_track_report_async_request(
    _: &Player,
    _: SimpleTrackReportAsyncRequest,
    response: &mut SimpleTrackReportAsyncResponse,
) {
    response.error_code = ErrorCode::Success.into();
    response.po = vec![];
}

pub fn on_tower_season_update_request(
    _: &Player,
    _: TowerSeasonUpdateRequest,
    response: &mut TowerSeasonUpdateResponse,
) {
    response.max_unlock_difficulty = 0;
}

pub fn on_version_info_push(_player: &Player, push: VersionInfoPush) {
    // TODO: Shall we do safety check and ensure we have compatible versions?
    tracing::debug!(
        "Client versions: launcher: {}, app: {}, resources: {}",
        push.launcher_version,
        push.app_version,
        push.resource_version
    );
}
