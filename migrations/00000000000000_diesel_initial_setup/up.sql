-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.




-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
CREATE TABLE IF NOT EXISTS item (
      id  integer PRIMARY KEY,
      description VARCHAR NOT NULL,
      name VARCHAR NOT NULL,
      icon VARCHAR NOT NULL,
      stackable smallint NOT NULL,
      item_bind smallint NOT NULL,
      buy_price BIGINT NOT NULL,
      item_class smallint NOT NULL,
      inventory_type smallint NOT NULL,
      equippable boolean NOT NULL,
      item_level smallint NOT NULL,
      max_count smallint NOT NULL,
      max_durability smallint NOT NULL,
      min_faction_id smallint NOT NULL,
      min_reputation integer NOT NULL,
      quality smallint NOT NULL,
      sell_price BIGINT NOT NULL,
      required_skill smallint NOT NULL,
      required_level smallint NOT NULL,
      required_skill_rank smallint NOT NULL,
      base_armor integer NOT NULL,
      has_sockets boolean NOT NULL,
      is_auctionable boolean NOT NULL,
      armor integer NOT NULL,
      display_info_id integer NOT NULL,
      name_description VARCHAR NOT NULL,
      name_description_color VARCHAR NOT NULL,
      upgradable boolean NOT NULL,
      heroic_tooltip boolean NOT NULL,
      context VARCHAR NOT NULL,
      artifact_id smallint
);
CREATE TABLE IF NOT EXISTS auction (
        auc integer PRIMARY KEY,
        item integer REFERENCES item(id),
        owner VARCHAR(12) NOT NULL,
        owner_realm VARCHAR(30) NOT NULL,
        bid BIGINT,
        buyout BIGINT,
        quantity smallint NOT NULL,
        time_left VARCHAR(15) NOT NULL,
        rand integer NOT NULL,
        seed BIGINT NOT NULL,
        context smallint NOT NULL,
        pet_species_id smallint,
        pet_breed_id smallint,
        pet_level smallint,
        pet_quality_id smallint,
        time timestamp NOT NULL
);

--
-- ```
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
