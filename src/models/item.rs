use schema::item;

#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub stat: i16,
    pub amount: i32
}

#[derive(Serialize, Deserialize)]
pub struct ItemSpell {
    pub spellId: i32,
    pub nCharges: i16,
    pub consumable: bool,
    pub categoryId: i16,
    pub spell: Option<Spell>,
    pub trigger: String,
}

#[derive(Serialize, Deserialize)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub cooldown: Option<String>,
    pub castTime: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct WeaponInfo {
    pub weaponSpeed: f32,
    pub dps: f32
}

#[derive(Serialize, Deserialize)]
pub struct Damage {
    pub min: i32,
    pub max: i32,
    pub exactMin: f32,
    pub exactMax: f32
}

#[derive(Serialize, Deserialize)]
pub struct SocketInfo {
    #[serde(rename = "type")]
    pub socket_type: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct ItemSource {
    pub sourceId: i32,
    pub sourceType: String
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub description: String,
    pub name: String,
    pub icon: String,
    pub stackable: i16,
    #[serde(rename = "itemBind")]
    pub item_bind: i16,
    #[serde(rename = "bonusStats")]
    pub bonus_stats: Vec<Stat>,
    #[serde(rename = "itemSpells")]
    pub item_spells: Vec<ItemSpell>,
    #[serde(rename = "buyPrice")]
    pub buy_price: i64,
    #[serde(rename = "itemClass")]
    pub item_class: i16,
    #[serde(rename = "weaponInfo")]
    pub weapon_info: Option<WeaponInfo>,
    #[serde(rename = "inventoryType")]
    pub inventory_type: i16,
    pub equippable: bool,
    #[serde(rename = "itemLevel")]
    pub item_level: i16,
    #[serde(rename = "maxCount")]
    pub max_count: i16,
    #[serde(rename = "maxDurability")]
    pub max_durability: i16,
    #[serde(rename = "minFactionId")]
    pub min_faction_id: i16,
    #[serde(rename = "minReputation")]
    pub min_reputation: i32,
    pub quality: i16,
    #[serde(rename = "sellPrice")]
    pub sell_price: i64,
    #[serde(rename = "requiredSkill")]
    pub required_skill: i16,
    #[serde(rename = "requiredLevel")]
    pub required_level: i16,
    #[serde(rename = "requiredSkillRank")]
    pub required_skill_rank: i16,
    #[serde(rename = "socketInfo")]
    pub socket_info: Option<SocketInfo>,
    #[serde(rename = "itemSource")]
    pub item_source: ItemSource,
    #[serde(rename = "baseArmor")]
    pub base_armor: i32,
    #[serde(rename = "hasSockets")]
    pub has_sockets: bool,
    #[serde(rename = "isAuctionable")]
    pub is_auctionable: bool,
    pub armor: i32,
    #[serde(rename = "displayInfoId")]
    pub display_info_id: i32,
    #[serde(rename = "nameDescription")]
    pub name_description: String,
    #[serde(rename = "nameDescriptionColor")]
    pub name_description_color: String,
    pub upgradable: bool,
    #[serde(rename = "heroicTooltip")]
    pub heroic_tooltip: bool,
    pub context: String,
    #[serde(rename = "availableContexts")]
    pub available_contexts: Vec<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: i16
}

#[derive(Queryable, Insertable)]
#[table_name="item"]
pub struct NewItem {
    pub id: i32,
    pub description: String,
    pub name: String,
    pub icon: String,
    pub stackable: i16,
    pub item_bind: i16,
    pub buy_price: i64,
    pub item_class: i16,
    pub inventory_type: i16,
    pub equippable: bool,
    pub item_level: i16,
    pub max_count: i16,
    pub max_durability: i16,
    pub min_faction_id: i16,
    pub min_reputation: i32,
    pub quality: i16,
    pub sell_price: i64,
    pub required_skill: i16,
    pub required_level: i16,
    pub required_skill_rank: i16,
    pub base_armor: i32,
    pub has_sockets: bool,
    pub is_auctionable: bool,
    pub armor: i32,
    pub display_info_id: i32,
    pub name_description: String,
    pub name_description_color: String,
    pub upgradable: bool,
    pub heroic_tooltip: bool,
    pub context: String,
    pub artifact_id: i16
}

impl NewItem {
    pub fn from(item: &Item) -> NewItem {
        NewItem {    
            id: item.id,
            description: item.description.to_owned(),
            name: item.name.to_owned(),
            icon: item.icon.to_owned(),
            stackable: item.stackable,
            item_bind: item.item_bind,
            buy_price: item.buy_price,
            item_class: item.item_class,
            inventory_type: item.inventory_type,
            equippable: item.equippable,
            item_level: item.item_level,
            max_count: item.max_count,
            max_durability: item.max_durability,
            min_faction_id: item.min_faction_id,
            min_reputation: item.min_reputation,
            quality: item.quality,
            sell_price: item.sell_price,
            required_skill: item.required_skill,
            required_level: item.required_level,
            required_skill_rank: item.required_skill_rank,
            base_armor: item.base_armor,
            has_sockets: item.has_sockets,
            is_auctionable: item.is_auctionable,
            armor: item.armor,
            display_info_id: item.display_info_id,
            name_description: item.name_description.to_owned(),
            name_description_color: item.name_description_color.to_owned(),
            upgradable: item.upgradable,
            heroic_tooltip: item.heroic_tooltip,
            context: item.context.to_owned(),
            artifact_id: item.artifact_id
        }

    }

}
