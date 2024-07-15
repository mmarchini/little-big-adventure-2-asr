use asr::signature::Signature;

// Look up the "No CD" strings, find the adddress where both of them are next to
// each other,
pub const SIGNATURE: Signature<255> = Signature::new(
    "59 6f 75 20 6e 65 65 64 20 4c 42 41 32 20 43 44 2c 20 73 6f 72 \
        72 79 21 00 54 57 49 4e 53 45 4e 00 59 6f 75 20 6e 65 65 64 20 54 77 \
        69 6e 73 65 6e 27 73 20 4f 64 79 73 73 65 79 20 43 44 2c 20 73 6f 72 \
        72 79 21 00 42 75 66 4d 65 6d 6f 53 65 65 6b 00 42 75 66 66 65 72 20 \
        41 6e 69 6d 00 72 65 73 73 2e 68 71 72 00 72 65 73 73 2e 68 71 72 00 \
        72 65 73 73 2e 68 71 72 00 72 65 73 73 2e 68 71 72 00 72 65 73 73 2e \
        68 71 72 00 72 65 73 73 2e 68 71 72 00 72 65 73 73 2e 68 71 72 00 72 \
        65 73 73 2e 68 71 72 00 72 65 73 73 2e 68 71 72 00 72 65 73 73 2e 68 \
        71 72 00 72 65 73 73 2e 68 71 72 00 72 65 73 73 2e 68 71 72 00 73 70 \
        72 69 74 65 73 2e 68 71 72 00 73 70 72 69 74 65 73 2e 68 71 72 00 73 \
        70 72 69 72 61 77 2e 68 71 72 00 73 70 72 69 72 61 77 2e 68 71 72 00
        61 6e 69 6d"
);

// After figuring out address above, find out where money is stored in memory.
// Subtract both, and you have the offset from our signature to where money is every time, regardless if things are moved around or not
pub const STRING_TO_INVENTORY_OFFSET: i64 = 0x1DD26;

// not sure why "+ 3" was needed here
pub const STRING_TO_NUM_CUBE_OFFSET: i64 = 0x1BD8F + 3;

pub const FLAG_THE_END: i64 = 157;

// Locations

// Twinsun -> Moon
pub const EMERALD_MOON_NEAR_CIRCLE_ENTRANCE: i32 = 75;

// Moon -> Otringal
pub const OTRINGAL_CRASH_SITE: i32 = 138;

// Otringal -> Francos
pub const FRANCOS_VILLAGE: i32 = 109;

// Francos -> Wannies
pub const UNDERGAS_ELEVATOR_OUTSIDE: i32 = 98;
// const UNDERGAS_ELEVATOR_INSIDE: i32 = 123;

// Wannies -> Mosquibees
pub const MOSQUIBEES_FERRY: i32 = 105;

// Mosquibees -> CX
pub const CX_OUTSIDE: i32 = 110;

// CX -> Palace
pub const OTRINGAL_OUTSIDE_PALACE_SHUTTLE: i32 = 88;

// Palace -> Dark Monk
pub const DARK_MONK_STATUE_FIRST: i32 = 185;
