use schema::auction;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct BonusList {
    #[serde(rename = "bonusListId")]
    pub bonus_list_id: i32
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Modifier {
    #[serde(rename = "type")]
    modifier_type: i16,
    value: i32
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Auction {
    pub auc: i32,
    pub item: i32,
    pub owner: String,
    #[serde(rename = "ownerRealm")]
    pub owner_realm: String,
    pub bid: i64,
    pub buyout: i64,
    pub quantity: i16,
    #[serde(rename = "timeLeft")]
    pub time_left: String,
    pub rand: i32,
    pub seed: i64,
    pub context: i16,
    #[serde(rename = "bonusList")]
    pub bonus_list: Option<Vec<BonusList>>,
    pub modifiers: Option<Vec<Modifier>>,
    #[serde(rename = "petSpeciesId")]
    pub pet_species_id: Option<i16>,
    #[serde(rename = "petBreedId")]
    pub pet_breed_id: Option<i16>,
    #[serde(rename = "petLevel")]
    pub pet_level: Option<i16>,
    #[serde(rename = "petQualityId")]
    pub pet_quality_id: Option<i16>,
    #[serde(skip_deserializing)]
    pub time: i64
}


#[derive(Queryable, Insertable)]
#[table_name = "auction"]
pub struct NewAuction {
    pub auc: i32,
    pub item: i32,
    pub owner: String,
    pub owner_realm: String,
    pub bid: i64,
    pub buyout: i64,
    pub quantity: i16,
    pub time_left: String,
    pub rand: i32,
    pub seed: i64,
    pub context: i16,
    pub pet_species_id: Option<i16>,
    pub pet_breed_id: Option<i16>,
    pub pet_level: Option<i16>,
    pub pet_quality_id: Option<i16>,
    pub time: NaiveDateTime
}

impl NewAuction {
    pub fn from(auction: &Auction) -> NewAuction {
        NewAuction {
            auc: auction.auc,
            item: auction.item,
            owner: auction.owner.to_owned(),
            owner_realm: auction.owner_realm.to_owned(),
            bid: auction.bid,
            buyout: auction.buyout,
            quantity: auction.quantity,
            time_left: auction.time_left.to_owned(),
            rand: auction.rand,
            seed: auction.seed,
            context: auction.context,
            pet_species_id: auction.pet_species_id,
            pet_breed_id: auction.pet_breed_id,
            pet_level: auction.pet_level,
            pet_quality_id: auction.pet_quality_id,
            time: NaiveDateTime::from_timestamp(auction.time, 0)
        }
    }

}
