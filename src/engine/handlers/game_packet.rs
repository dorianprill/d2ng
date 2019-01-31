use engine::handlers::game_event::*;
use d2re::connection::d2gs::D2GSPacket;
/// Server->Client MessageId is determined by the first byte of its content i.e. packet[0]
#[allow(dead_code)]
#[repr(u8)]
enum MessageId {
    GameLoading         = 0x00,
    GameFlagsPing       = 0x01,
    SetLocale           = 0x02, // what is this really?
    LoadActData         = 0x03,
    NpcUpdate           = 0x0C,
    PlayerMove          = 0x0F,
    PlayerReassign      = 0x15, // this is used for self sent chat packets!?
    Experience1         = 0x1A,
    Experience2         = 0x1B,
    Experience3         = 0x1C,
    ItemSkillBonus1     = 0x21,
    ItemSkillBonus2     = 0x22,
    ChatMessage         = 0x26,
    NPCTransaction      = 0x2A,
    //This message should be used for manipulating the trading window,
    // the Horadric Cube item window, and the Stash window.
    // see https://bnetdocs.org/packet/98/d2gs-trade
    Trade           = 0x4F,
    WorldObject     = 0x51,
    PlayerInit      = 0x59, // Server->Client
    // e.g. 4711 Stones of Jordan Sold to Merchants
    EventMessage    = 0x5A,
    PlayerJoined    = 0x5B,
    PlayerLeft      = 0x5C,
    NPCMoveEntity   = 0x68, // ?
    NPCStateUpdate  = 0x69, // ?
    NPCMoveStop     = 0x6D, // in game i only ever see these received
    MercUpdate      = 0x81,
    PortalUpdate    = 0x82,
    // not sure why there are two types for hp/mp update
    LifeManaUpdate1 = 0x18,
    LifeManaUpdate2 = 0x95,
    // again, not sure why there are two types
    ItemAction1     = 0x9C,
    ItemAction2     = 0x9D,
    DelayedStated   = 0xA7,
    NPCAssignment   = 0xAC, // whatever this is
    // warden should'nt find anything since we're not modifying game memory :)
    WardenScan      = 0xAE
}


/// Packet handler calls the corresponding event handler functions in game_event.rs
pub fn game_packet_handler(packet: &D2GSPacket) {

    // println!(
    //     "recv d2gs packet len={:04} decompress={:?} {:x?}  {:?}",
    //     which.len(),
    //     decompress,
    //     which,
    //     String::from_utf8_lossy(which).into_owned()
    // );
    // how to get rid of this unsafe block?
    // enum has #[repr(u8)] so should'nt be a problem...
    let header: MessageId = unsafe { ::std::mem::transmute(which[0]) };
    match header {
        MessageId::SetLocale       => (),
        MessageId::PlayerReassign  => (),
        MessageId::ChatMessage     => chat_event_handler(&which),
        MessageId::NPCTransaction  => (),
        MessageId::EventMessage    => (),
        MessageId::LifeManaUpdate1
            | MessageId::LifeManaUpdate2
                                      => (),
        MessageId::ItemAction1
          | MessageId::ItemAction2 => (),
        MessageId::DelayedStated   => (),
        MessageId::WardenScan      => (),
        _ => (),
    }
}