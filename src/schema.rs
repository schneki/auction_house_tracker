table! {
    auction (auc) {
        auc -> Int4,
        item -> Nullable<Int4>,
        owner -> Varchar,
        owner_realm -> Varchar,
        bid -> Nullable<Int8>,
        buyout -> Nullable<Int8>,
        quantity -> Int2,
        time_left -> Varchar,
        rand -> Int4,
        seed -> Int8,
        context -> Int2,
        pet_species_id -> Nullable<Int2>,
        pet_breed_id -> Nullable<Int2>,
        pet_level -> Nullable<Int2>,
        pet_quality_id -> Nullable<Int2>,
        time -> Timestamp,
    }
}

table! {
    item (id) {
        id -> Int4,
        description -> Varchar,
        name -> Varchar,
        icon -> Varchar,
        stackable -> Int2,
        item_bind -> Int2,
        buy_price -> Int8,
        item_class -> Int2,
        inventory_type -> Int2,
        equippable -> Bool,
        item_level -> Int2,
        max_count -> Int2,
        max_durability -> Int2,
        min_faction_id -> Int2,
        min_reputation -> Int4,
        quality -> Int2,
        sell_price -> Int8,
        required_skill -> Int2,
        required_level -> Int2,
        required_skill_rank -> Int2,
        base_armor -> Int4,
        has_sockets -> Bool,
        is_auctionable -> Bool,
        armor -> Int4,
        display_info_id -> Int4,
        name_description -> Varchar,
        name_description_color -> Varchar,
        upgradable -> Bool,
        heroic_tooltip -> Bool,
        context -> Varchar,
        artifact_id -> Nullable<Int2>,
    }
}

joinable!(auction -> item (item));

allow_tables_to_appear_in_same_query!(
    auction,
    item,
);
