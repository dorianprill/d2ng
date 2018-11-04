// Player struct
// implements the Entity trait

use world_entity::*;
use coordinate::*;

pub mod player;

struct Player {
        name:               &str;
        has_mercenary:      bool;
        directory_known:    bool;
        mercenary_id:       u32;
        level:              u32;
        portal_id:          u32;
        pub entity:         WorldEntity;
        //protected Globals.CharacterClassType m_class;
}

// TODO translate form c#
//public Globals.CharacterClassType Class { get { return m_class; } set { m_class = value; } }

impl Player {
    pub fn name(&self) -> &str {
        return self.name
    }

    pub fn has_mercenary(&self) -> bool {
        return self.has_mercenary
    }

    pub fn mercenary_id(&self) -> u32 {
        return self.mercenary_id
    }

    pub fn mercenary_id_set(&self, merc_id: u32) {
        self.has_mercenary  = true;
        self.mercenary_id   = merc_id;
    }

    pub fn directory_known(&self) -> bool {
        return self.directory_known
    }

    pub fn level(&self) -> u32 {
        return self.level
    }

    pub fn set_level(&self, lvl: u32) -> u32 {
        self.level = lvl
    }

    pub fn portal_id(&self) -> u32 {
        return self.portal_id
    }

    pub fn set_portal_id(&self, portal_id: u32) -> u32 {
        self.portal_id = portal_id
    }

}
