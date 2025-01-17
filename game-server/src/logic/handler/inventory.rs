use shorekeeper_protocol::{
    NormalItemRequest, NormalItemResponse, WeaponItemRequest, WeaponItemResponse,
    PhantomItemRequest, PhantomItemResponse, ValidTimeItemRequest, ValidTimeItemResponse,
    ItemExchangeInfoRequest, ItemExchangeInfoResponse,
};

use crate::logic::player::Player;

pub fn on_normal_item_request(
    _player: &Player,
    _request: NormalItemRequest,
    response: &mut NormalItemResponse,
) {
    response.normal_item_list = vec![];
}

pub fn on_weapon_item_request(
    _player: &Player,
    _request: WeaponItemRequest,
    response: &mut WeaponItemResponse,
) {
    response.weapon_item_list = vec![];
}

pub fn on_phantom_item_request(
    _player: &Player,
    _request: PhantomItemRequest,
    response: &mut PhantomItemResponse,
) {
    response.phantom_item_list = vec![];
    response.equip_info_list = vec![];
    response.ows2 = vec![];
    response.phantom_skin_list = vec![];
    response.max_cost = 12;
}

pub fn on_valid_time_item_request(
    _player: &Player,
    _request: ValidTimeItemRequest,
    response: &mut ValidTimeItemResponse,
) {
    response.item_list = vec![];
}

pub fn on_item_exchange_info_request(
    _player: &Player,
    _request: ItemExchangeInfoRequest,
    response: &mut ItemExchangeInfoResponse,
) {
    response.item_exchange_infos = vec![];
}
