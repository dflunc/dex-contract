use classic_dfluncswap::asset::PairInfoRaw;
use cw_storage_plus::Item;

pub const PAIR_INFO: Item<PairInfoRaw> = Item::new("pair_info");
