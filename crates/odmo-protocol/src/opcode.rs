pub const CHECKSUM_VALIDATION: i16 = 6716;

pub mod account {
    pub const KEEP_CONNECTION: i16 = -3;
    pub const CONNECTION: i16 = -1;
    pub const CONNECTED_EVENT: i16 = -1;
    pub const CONNECTION_RESPONSE: i16 = -2;
    pub const LOGIN_REQUEST: i16 = 3301;
    pub const LOAD_SERVER_LIST: i16 = 1701;
    pub const CONNECT_CHARACTER_SERVER: i16 = 1702;
}

pub mod character {
    pub const KEEP_CONNECTION: i16 = -3;
    pub const CONNECTION: i16 = -1;
    pub const CONNECTION_RESPONSE: i16 = -2;
    pub const CREATE_CHARACTER: i16 = 1303;
    pub const DELETE_CHARACTER: i16 = 1304;
    pub const CHECK_NAME_DUPLICITY: i16 = 1302;
    pub const GET_CHARACTER_POSITION: i16 = 1305;
    pub const REQUEST_CHARACTERS: i16 = 1706;
    pub const CONNECT_GAME_SERVER: i16 = 1703;
    pub const CHARACTER_LIST: i16 = 1301;
    pub const CHARACTER_CREATED: i16 = 1306;
    pub const CHARACTER_CREATION_FAILED: i16 = 1307;
    pub const CHARACTER_DELETED: i16 = 1304;
    pub const CONNECT_GAME_SERVER_INFO: i16 = 1308;
}

pub mod game {
    pub const KEEP_CONNECTION: i16 = -3;
    pub const CONNECTION: i16 = -1;
    pub const CONNECTION_RESPONSE: i16 = -2;
    pub const COMPLEMENTAR_INFORMATION: i16 = 1001;
    pub const TAMER_MOVIMENTATION: i16 = 1004;
    // pGame::Sync (1006) is a multiplexed entity channel. The leading byte
    // discriminates the sub-packet: 1=mob, 3=tamer, 4=unload, 5=tamer-walk,
    // 6=digimon-walk, 16=buffs. Walk, load/unload and buff packets all ride
    // opcode 1006; the client switches on that first byte. (pGame::HpRate=1007
    // is a different, unrelated packet — do not put entity loads there.)
    pub const MAP_ENTITY: i16 = 1006;
    pub const LOAD_UNLOAD_ENTITY: i16 = 1006;
    pub const CONSIGNED_SHOP_ENTITY: i16 = 1008;
    pub const LOAD_BUFFS: i16 = 1006;
    pub const NOTICE_MESSAGE: i16 = 1010;
    pub const CHAT_MESSAGE: i16 = 1008;
    pub const CHAT_MESSAGE_RESPONSE: i16 = 1006;
    pub const INITIAL_INFORMATION: i16 = 1706;
    pub const INITIAL_INFO_RESPONSE: i16 = 1003;
    // Target HP-rate notice {u4 handler_id, u1 rate} -> event 42
    // (44996 registry 1007/0xDA6EB0, builder case pGame::HpRate).
    pub const GAME_HP_RATE: i16 = 1007;
    // Level-up notice {u4 handler_id, u1 level} -> event 667
    // (44996 registry 1019/0xDA6C60, builder case pGame::LevelUp).
    pub const GAME_LEVEL_UP: i16 = 1019;
    // Battle-state notices {u4 handler_id} resolved through the entity table
    // (44996 registry 1034/0xDA6770 and 1035/0xDA67E0).
    pub const GAME_BATTLE_ON: i16 = 1034;
    pub const GAME_BATTLE_OFF: i16 = 1035;
    // Partner stat-restore notice {u4 handler_id; u4 x3} hitting three
    // sequential status setters (44996 registry 1023/0xDA6D70).
    pub const GAME_HEAL_UP: i16 = 1023;
    // Ting visual notice {u4 handler_id} (44996 registry 1053/0xDD0780).
    pub const GAME_TING: i16 = 1053;
    // Emoticon notice {u4 handler_id; u4 emotion_id; u4 flag} resolved through
    // table 17 (CEmotionTable); flag 1 selects the aura variant
    // (44996 registry 1058/0xDA48A0).
    pub const GAME_EMOTICON: i16 = 1058;
    // Start-message UI notice {u1 count; count x {u2 len; len x bytes}} feeding
    // event 42 per message (44996 registry 1047/0xDACC50).
    pub const GAME_START_MESSAGE: i16 = 1047;
    // Death-state reset notice; client advances a single u2 and clears the
    // corpse/party-requeue state (44996 registry 1052/0xDA4B30).
    pub const GAME_DIED_ALREADY: i16 = 1052;
    // Advert-boss notices: 1059 count/upsert {u4; u4; u4} (lookup-then-create
    // dispatch, 0xDAF960), 1061 die {u4; u4} (direct-removal dispatch
    // 0xDAF8E0). Create 1060 {u4 map; u4; u4 monster} resolves CMapTable(33)
    // and CMonsterTable(27) rows, pings event 121 and registers the boss
    // entry conditionally (44996 registry 1060/0xDAFAD0).
    pub const ADVERT_BOSS_COUNT: i16 = 1059;
    pub const ADVERT_BOSS_CREATE: i16 = 1060;
    pub const ADVERT_BOSS_DIE: i16 = 1061;
    // Sparse channel-status map fill: n x {u1 offset; u1 byte}, terminated by
    // an 0xFF offset; builds the 32-byte slot buffer for event 118
    // (44996 registry 1713/0xDA0CD0).
    pub const SVR_CHANNEL_INFO: i16 = 1713;
    // Buddy-notice family. Modern parsers read modern UTF-8 strings
    // ({u2 len; len x bytes}) before dispatching into the friends UI:
    // 2401 add ({u1 flag; str}), 2403 ban (str), 2407 not-found (str),
    // 2408 on (str; see FRIEND_CONNECT), 2409 off (str),
    // 2410 name change ({u1 flag; str; str}).
    pub const BUDDY_ADD: i16 = 2401;
    pub const BUDDY_BAN: i16 = 2403;
    pub const BUDDY_NOT_FOUND: i16 = 2407;
    pub const BUDDY_OFF: i16 = 2409;
    pub const BUDDY_CHANGE_NAME: i16 = 2410;
    // Item-scan notice on the receive table: {u4; i64; u2 count;
    // count x u2 rows} feeding the scan list through event 96
    // (44996 registry 3988/0xDBE370). The fork's C2S ItemScan request uses
    // 3987 on the send table, so this notice keeps a distinct constant.
    pub const ITEM_SCAN_NOTICE: i16 = 3988;
    pub const UPDATE_STATUS: i16 = 1043;
    pub const SERVER_EXPERIENCE: i16 = 1054;
    pub const SEALS: i16 = 1333;
    pub const AVAILABLE_RELATIONS: i16 = 2404;
    pub const FRIEND_CONNECT: i16 = 2408;
    pub const AVAILABLE_CHANNELS: i16 = 1713;
    pub const GUILD_INFORMATION: i16 = 2113;
    pub const GUILD_RANK: i16 = 2127;
    pub const GUILD_HISTORIC: i16 = 2128;
    pub const TIME_REWARD: i16 = 3106;
    pub const TAMER_ATTENDANCE: i16 = 3133;
    pub const CASH_SHOP_COINS: i16 = 3404;
    pub const MEMBERSHIP: i16 = 3414;
    pub const PICK_BITS: i16 = 3911;
    pub const PICK_ITEM_FAIL: i16 = 3913;
    pub const LOOT_ITEM: i16 = 3910;
    pub const UPDATE_MOVEMENT_SPEED: i16 = 9905;
    pub const LOAD_INVENTORY: i16 = 16009;
    pub const TAMER_XAI_RESOURCES: i16 = 16032;
    pub const XAI_INFO: i16 = 16033;
    pub const SYNC_CONDITION: i16 = 1070;
    pub const WARP_GATE: i16 = 1709;
    pub const LOCAL_MAP_SWAP: i16 = 1711;
    pub const CONSUME_ITEM: i16 = 3901;
    pub const MOVE_ITEM: i16 = 3904;
    pub const MOVE_ITEM_FAILURE: i16 = 3905;
    pub const SPLIT_ITEM: i16 = 3907;
    pub const ITEM_REMOVE: i16 = 3909;
    pub const NPC_PURCHASE: i16 = 3915;
    // C2S sell request; the client also parses 3916 frames on GS2C as the
    // sell result {i64 money; u1 count; count x {u1 kind; u4 serial; u2
    // stack rows}} pinging ev89 (44996 receive 0xDA3E50).
    pub const NPC_SELL: i16 = 3916;
    // Event notices: 3135 hot-time {r4 value} -> ev454 {3135, value};
    // 3139 item term {u2} -> ev527 {3139, value}
    // (0xD089C0 / 0xD09000).
    pub const EVENT_HOT_TIME: i16 = 3135;
    pub const EVENT_ITEM_TERM: i16 = 3139;
    // Gotcha {r4 value} -> ev492 {value} (44996 0xDB0D10).
    pub const ITEM_GOTCHA: i16 = 3959;
    // Guild member join {r4 value} -> ev261 {value} (44996 0xD95560).
    pub const GUILD_INC_MEMBER: i16 = 2130;
    // Tamer option {r4 target handler; r4 value}; the client looks the
    // target up and applies the option through it (44996 0xDA3D00).
    pub const TAMER_OPTION: i16 = 1313;
    // Skill-chip result {r4 handler; r4 skill_id}: the CSkillTable row keyed
    // by skill_id with kind 1 or 47 pops the buff message (44996 0xDC8B40).
    pub const SKILL_RESULT_CHIP: i16 = 1122;
    // Cross-inventory slot-size update {r4; u2; r4; r4} -> ev491
    // (44996 0xDB0D80).
    pub const ITEM_CROSS_INVEN_SLOT_SIZE: i16 = 3966;
    // Move-clear removal {r4 handler; r4 item_id}: the client checks the
    // CItemTable row (+204) and pops the "Event\%s.nif" visual via 901112
    // (44996 0xDB0E90).
    pub const ITEM_MOVE_CLEAR: i16 = 3962;
    // Cash trade timeout {u2; r4; r4} -> ev467 (44996 0xD9BEE0).
    pub const ITEM_CASH_TRADE_TIMEOUT: i16 = 3984;
    // Make result {u1 kind; u1 slot}: 0 clears a digimon skill-table row
    // (offset 65 flag = -1), 1/3/4 fire ev586 with 1000/3000/4000 + slot, 2
    // fires ev68 {slot}, 5 fires ev586 {5000} and 9 fires ev68 {1}
    // (44996 0xD9B730).
    pub const ITEM_MAKE_RESULT: i16 = 3983;
    // Tamer item-skill flow: 1330 apply {r4 index; r4 skill ref} through the
    // client list at +1092 plus a CSkillTable row check; 1331 expire and 1332
    // remove are {u1 count; count x r4 skill ids} batches
    // (44996 0xDC6050 / 0xDC6160 / 0xDC62B0).
    pub const TAMER_ITEM_SKILL_APPLY: i16 = 1330;
    pub const TAMER_ITEM_SKILL_EXPIRE: i16 = 1331;
    pub const TAMER_ITEM_SKILL_REMOVE: i16 = 1332;
    // Pinochi vote {u2 count; count x {u2 slot; r4 value}}: each entry
    // decrements a digimon skill-table row (+4, zeroing the 69-byte row on
    // underflow) and ends with the 10019 event window
    // (44996 0xDB4A70).
    pub const EVENT_PINOCHI_VOTE: i16 = 3127;
    // Store-summon use {u1 count; count x {r4; r4}} -> ev466
    // (44996 0xD9BC10).
    pub const ITEM_STORE_SUMMON_USE: i16 = 3953;
    // Skill error {u1 kind; u2 code; dead u2 when kind = 255}: kind 95
    // resets only on code 0, kind 100 closes the cast window on code 1 and
    // other kinds fall through to the digimon vtable reset
    // (44996 0xDC5420).
    pub const SKILL_ERROR: i16 = 1106;
    // Event active banner {r4; u1 x4; u1 count; count x 71-byte rows} ->
    // ev130; each row is 68 opaque bytes plus a u2 and a u1 tail
    // (44996 0xD092D0).
    pub const EVENT_ACTIVE: i16 = 3138;
    // Tamer skill success {r4 skill index; r4 argument}: the client looks up
    // the slide-list row and rebuilds the skill presentation struct (reads
    // stop at the two u4s) (44996 0xDC56F0).
    pub const TAMER_SKILL_SUCCESS: i16 = 1328;
    // Partner scale {r4 handler; u2 scale; r4 arg}: entity lookup + scale
    // application through the resolved digimon (44996 0xDCAB10).
    pub const MASTER_PARTNER_SCALE: i16 = 9942;
    // Master/GM debug notices: 9963 conn count {r4} pops "Connect\n%d"; 9964
    // cash payout {i64; i64} pops the premium text; 9972 GM quest copies a
    // fixed 0x598-byte blob straight into the client table at +0x766C
    // (44996 0xD910A0 / 0xD91140 / 0xD91250).
    pub const MASTER_CONN_COUNT: i16 = 9963;
    pub const MASTER_PAY_CASH: i16 = 9964;
    pub const MASTER_GM_QUEST: i16 = 9972;
    // Guild name-flow notices: 2122 member level-up {u8 dead; s2 string} ->
    // ev268 with the string; 2123 tamer-name change {s2; s2} -> ev275 with
    // the first string; 2124 deleted tamer {s2} -> ev274
    // (44996 0xD93790 / 0xD94080 / 0xD93FE0, raw asm).
    pub const GUILD_MEMBER_LEVEL_UP: i16 = 2122;
    pub const GUILD_CHANGE_TNAME: i16 = 2123;
    pub const GUILD_DELETED_TAMER: i16 = 2124;
    // GM hatch stat table {r4 x6 succ; r4 x5 fail; r4 x5 break; r4 x5
    // succ-average floats}: the client renders _HatchTry.txt and opens
    // notepad (44996 0xD90DB0, raw asm).
    pub const GM_HATCH_TRY: i16 = 9966;
    // Pinochi get-info {r4 x11}: one value lands at container +0x48, five at
    // +0x60..+0x70 and five at +0x4C..+0x5C; the render list is refreshed
    // through the 0x3138 slots and the fallback path pings ev42
    // (44996 0xDB47E0, raw asm).
    pub const EVENT_PINOCHI_GET_INFO: i16 = 3126;
    // Party exp split {i64 x3; r4; i64 x3; r4; s2 label} -> ev196; the label
    // string is consumed but never reaches the event struct
    // (44996 0xD9D670, raw asm).
    pub const PARTY_EXP: i16 = 2311;
    // Equip flow {u2 handler; u1 kind; r4 x3; u1 dead; u1}: kind 5 runs the
    // equip slot push through the entity slot and kind 10 swaps the digimon
    // aura (item-table row kind 31 with the T_Aura visual pair)
    // (44996 0xDD03E0, raw asm).
    pub const TAMER_EQUIP_ITEM: i16 = 1310;
    // Master user-info lookup miss {s2 name}: the client answers the GM
    // console with the "(name) is not found" line on ev42
    // (44996 0xD8F990, raw asm).
    pub const MASTER_USER_INFO_NOT_FOUND: i16 = 9921;
    // Master second-password reset {u2 code}: 0 resolves the misc table id 34
    // (CHAT_SECOND_PASSWORD_RESET_SUCCESS) and 1 id 17 (CHAT_NOT_EXIST_ID),
    // both popped as an ev42 console line (44996 0xD8F190, raw asm).
    pub const MASTER_INIT_2ND_PASS: i16 = 9945;
    // Master kill-stat dump {u4 count; count x i64}: per-entry money totals
    // are rendered into _KillMon.txt and popped open in notepad
    // (44996 0xD90B50, raw asm).
    pub const MASTER_LIE_KILL_MONSTER: i16 = 9958;
    // Pinochi item delete {r4 key; x3 {u2 count; count x {u2 slot; r4
    // value}}}: block 1 decrements digimon skill-table rows (zeroing the
    // 69-byte row on underflow), blocks 2/3 are limit-checked and popped to
    // ev59/ev60, and the tail walks the special-event NPC list
    // (44996 0xDB4B80).
    pub const EVENT_PINOCHI_ITEM_DELETE: i16 = 3128;
    // Master user list {u2 count; count x {s2; s2}}: rows render into
    // uList.txt and ping ev42 per line; the tail opens the file in notepad
    // (44996 0xD8FB80, raw asm).
    pub const MASTER_USER_LIST: i16 = 9922;
    pub const REPURCHASE_ITEM: i16 = 3978;
    pub const LOAD_NPC_REPURCHASE_LIST: i16 = 3979;
    pub const DIGI_SUMMON_SYNC_REQUEST: i16 = 3652;
    pub const DIGI_SUMMON_SYNC_RESPONSE: i16 = 3652;
    pub const EXITEM_MOVE: i16 = 0x0ED9;
    pub const EXITEM_BATCH_MOVE: i16 = 0x0EDA;
    pub const EXITEM_SORT: i16 = 0x0EDB;
    pub const EXITEM_USE: i16 = 0x0EDC;
    pub const EMOTICON: i16 = 1076;
    pub const WHISPER_MESSAGE: i16 = 1009;
    pub const SHOUT_MESSAGE: i16 = 1056;
    pub const MEGAPHONE_MESSAGE: i16 = 1074;
    pub const TAMER_REACTION: i16 = 1058;
    pub const PARTNER_STOP: i16 = 1033;
    pub const PARTNER_STOP_RESPONSE: i16 = 1033;
    pub const PARTNER_ATTACK: i16 = 1013;
    pub const PARTNER_ATTACK_RESPONSE: i16 = 1013;
    pub const ATTACK_MISS: i16 = 1014;
    pub const PARTNER_SKILL: i16 = 1015;
    pub const PARTNER_SKILL_RESPONSE: i16 = 1015;
    pub const KILL_ON_HIT: i16 = 1020;
    pub const KILL_ON_SKILL: i16 = 1021;
    pub const PARTNER_SKILL_ERROR: i16 = 1105;
    pub const EVOLUTION: i16 = 1028;
    pub const PARTNER_EVOLUTION: i16 = 1028;
    pub const EVOLUTION_FAILURE: i16 = 1029;
    pub const PARTNER_SWITCH: i16 = 1041;
    pub const PARTNER_SWITCH_RESPONSE: i16 = 1041;
    pub const PARTNER_DELETE: i16 = 1042;
    pub const PARTNER_DELETE_RESPONSE: i16 = 1042;
    pub const CAPSULE_EVOLUTION_SLOT_RESULT: i16 = 3238;
    pub const EVOLUTION_ARMOR_UNLOCKED: i16 = CAPSULE_EVOLUTION_SLOT_RESULT;
    pub const EVOLUTION_UNLOCK: i16 = 1055;
    pub const EVOLUTION_UNLOCK_RESPONSE: i16 = 1055;
    pub const RIDE_MODE_START: i16 = 1325;
    pub const RIDE_MODE_STOP: i16 = 1326;
    pub const OPEN_RIDE_MODE: i16 = 1063;
    pub const SET_TARGET: i16 = 1016;
    pub const STAT_UP: i16 = 1030;
    pub const REFRESH_SCREEN: i16 = 1046;
    pub const AWAY_TIME: i16 = 1069;
    pub const DIGIMON_CHANGE_NAME: i16 = 3206;
    pub const HATCH_INSERT_EGG: i16 = 1036;
    pub const HATCH_INCREASE: i16 = 1037;
    pub const HATCH_FINISH: i16 = 1038;
    pub const HATCH_REMOVE_EGG: i16 = 1039;
    pub const HATCH_FAILURE: i16 = 1040;
    pub const HATCH_BACKUP_INSERT: i16 = 3946;
    pub const HATCH_BACKUP_CANCEL: i16 = 3947;
    pub const INCUBATOR_CLOSE: i16 = 3948;
    pub const DIGIMON_ARCHIVE_MOVE: i16 = 3201;
    pub const DIGIMON_ARCHIVE_LIST: i16 = 3204;
    pub const DIGIMON_ARCHIVE_SWAP: i16 = 3243;
    pub const INVENTORY_SORT: i16 = 3980;
    pub const ITEM_IDENTIFY: i16 = 3968;
    pub const ITEM_CRAFT: i16 = 3982;
    pub const ITEM_REROLL: i16 = 3969;
    pub const ITEM_SOCKET_IN: i16 = 3926;
    pub const ITEM_SOCKET_OUT: i16 = 3927;
    pub const ITEM_SOCKET_IDENTIFY: i16 = 3929;
    pub const ITEM_RETURN: i16 = 3923;
    pub const ITEM_SCAN: i16 = 3987;
    pub const LOAD_GIFT_STORAGE: i16 = 3935;
    pub const GIFT_STORAGE_RETRIEVE: i16 = 3936;
    pub const LOAD_REWARD_STORAGE: i16 = 16001;
    pub const RECOMPENSE_GAIN: i16 = 16002;
    pub const TAMER_SHOP_OPEN: i16 = 1511;
    pub const TAMER_SHOP_CLOSE: i16 = 1512;
    pub const TAMER_SHOP_BUY: i16 = 1513;
    pub const CONSIGNSHOP_OPEN: i16 = 1516;
    pub const CONSIGNSHOP_VIEW: i16 = 1520;
    pub const CONSIGNSHOP_PURCHASE: i16 = 1518;
    pub const CONSIGNSHOP_RETRIEVE: i16 = 1517;
    pub const CASHSHOP_BUY: i16 = 3413;
    pub const CASHSHOP_RELOAD: i16 = 3404;
    pub const QUEST_AVAILABLE_LIST: i16 = 11009;
    pub const QUEST_ACCEPT: i16 = 11002;
    pub const QUEST_DELIVER: i16 = 11004;
    pub const QUEST_GIVE_UP: i16 = 11003;
    pub const QUEST_UPDATE: i16 = 11001;
    pub const QUEST_GOAL_UPDATE: i16 = 11001;
    pub const QUEST_DAILY_UPDATE: i16 = 11006;
    pub const DIE_CONFIRM: i16 = 1022;
    pub const REMOVE_BUFF: i16 = 4005;
    pub const DAMAGE_SKIN_CHANGE: i16 = 3153;
    pub const SEAL_OPEN: i16 = 3971;
    pub const SEAL_CLOSE: i16 = 3972;
    // Seal leader assignment: request carries a u2 card code;
    // confirmation returns {u4 tamer_id, u2 leader_id}.
    pub const SEAL_SET_LEADER: i16 = 3232;
    // Seal leader removal: request carries a u2 card code; server confirms
    // with {u4 tamer_id, u2 card_id}. Opcode 3233 is shared: an empty
    // payload is the encyclopedia open request instead.
    pub const SEAL_REMOVE_LEADER: i16 = 3233;
    pub const SEAL_SET_FAVORITE: i16 = 1334;
    // Encyclopedia open request: opcode 3233 with an empty payload.
    pub const ENCYCLOPEDIA_LOAD: i16 = 3233;
    // Encyclopedia list: a 6-byte payload {u4, u2} is a level update;
    // otherwise {u4 count, count × record{u4 id, u2 level, u8 slot_mask,
    // 5×u2 enchant(AT,BL,CT,EV,HP), s2 size, u1 rewarded}}, 27-byte records.
    pub const ENCYCLOPEDIA_LOAD_RESPONSE: i16 = 3234;
    // Encyclopedia reward claim {u4 digimon_id};
    // response {u4 item_id, u2 item_count}.
    pub const ENCYCLOPEDIA_GET_REWARD: i16 = 3235;
    // Encyclopedia deck use {u4 deck_id} (1001..).
    pub const ENCYCLOPEDIA_DECK_USE: i16 = 3236;
    // Encyclopedia deck buff {u2 option_index}, ENCY_MAX_OPTION == 4.
    pub const ENCYCLOPEDIA_DECK_BUFF: i16 = 3237;
    // Care slot size notice {u2 slot_count} -> event 407 (44996 registry 3205/0xDAEF50).
    pub const DIGIMON_CARE_SLOT_SIZE: i16 = 3205;
    // Battle tag result notices; both payloadless (44996 registry 3209/0xDAD980,
    // 3210/0xDAD9A0).
    pub const DIGIMON_BATTLE_TAG_SUCCESS: i16 = 3209;
    pub const DIGIMON_BATTLE_TAG_FAILURE: i16 = 3210;
    // Skill point init {u1 count, count × 28-byte record} -> event 477
    // (44996 registry 3213/0xDA0DA0).
    pub const DIGIMON_SKILL_POINT_INIT: i16 = 3213;
    // Incubator facilitation reply {u1 kind, u4 arg} -> event 412
    // (44996 registry 3227/0xDB29C0).
    pub const DIGIMON_FACILITATION: i16 = 3227;
    // Incubator error notice; parser consumes a u4 and drops it
    // (44996 registry 3228/0xD866F0). Field meaning unresolved.
    pub const DIGIMON_INCUBATER_ERROR: i16 = 3228;
    // Incubator list rows: exactly 3 rows of {u4; u4; u4}, each row handed to
    // the slot builder, then an empty event-408 dispatch
    // (44996 registry 3226/0xDB2830).
    pub const DIGIMON_INCUBATER_LIST: i16 = 3226;
    // Incubator extraction result {u1} -> event 413
    // (44996 registry 3229/0xDB2A70).
    pub const DIGIMON_EXTRACTION: i16 = 3229;
    // Incubator slot change {u4 slot; u4 value} -> event 411; slot -1 makes
    // the client close the incubator view instead
    // (44996 registry 3231/0xDB2AE0).
    pub const DIGIMON_INCUBATER_SLOT_CHANGE: i16 = 3231;
    pub const OTHER_TAMER_DETAIL_INFO_REQUEST: i16 = 16070;
    pub const OTHER_TAMER_DETAIL_INFO_RESPONSE: i16 = 16071;
    pub const ARENA_DAILY_POINTS: i16 = 4131;
    pub const ARENA_DAILY_RANKING: i16 = 4130;
    pub const ARENA_RANKING_ALL: i16 = 16023;
    pub const ARENA_REQUEST_RANK: i16 = 16025;
    pub const ARENA_REQUEST_OLD_RANK: i16 = 16026;
    pub const DUNGEON_NEXT_STAGE: i16 = 4126;
    pub const DUNGEON_SURRENDER: i16 = 4127;
    pub const BURNING_EVENT: i16 = 3132;
    pub const DAILY_CHECK_EVENT: i16 = 3136;
    pub const DAILY_CHECK_EVENT_REQUEST: i16 = 3137;
    pub const JOIN_EVENT_QUEUE: i16 = 3124;
    pub const REGION_UNLOCK: i16 = 1051;
    pub const SET_TITLE: i16 = 15;
    pub const CHANGE_TAMER_MODEL: i16 = 1314;
    pub const TAMER_NAME_CHANGE: i16 = 1311;
    pub const RARE_MACHINE_OPEN: i16 = 3956;
    pub const RARE_MACHINE_RUN: i16 = 3957;
    pub const PARTY_INVITE: i16 = 2301;
    pub const PARTY_INVITE_RESPONSE: i16 = 2302;
    pub const PARTY_CHAT: i16 = 2304;
    pub const PARTY_JOIN: i16 = 2305;
    pub const PARTY_KICK: i16 = 2306;
    pub const PARTY_LEAVE: i16 = 2307;
    pub const PARTY_CHANGE_MASTER: i16 = 2308;
    pub const PARTY_CHANGE_LOOT: i16 = 2309;
    pub const PARTY_MEMBER_LIST: i16 = 2310;
    pub const PARTY_MEMBER_DISCONNECTED: i16 = 2312;
    pub const PARTY_MEMBER_INFO: i16 = 2313;
    pub const PARTY_MEMBER_MAP_CHANGE: i16 = 2315;
    pub const PARTY_MEMBER_DIGIMON_CHANGE: i16 = 2316;
    pub const PARTY_DISMISS: i16 = 2317;
    pub const PARTY_CREATED: i16 = 2319;
    pub const PARTY_MEMBER_POSITION: i16 = 2320;
    pub const PARTY_MEMBER_BUFF_CHANGE: i16 = 2323;
    pub const GUILD_CREATE: i16 = 2101;
    pub const GUILD_DELETE: i16 = 2102;
    pub const GUILD_INVITE: i16 = 2109;
    pub const GUILD_INVITE_ACCEPT: i16 = 2103;
    pub const GUILD_INVITE_RESPONSE_ACCEPT: i16 = 2108;
    pub const GUILD_INVITE_DENY: i16 = 2105;
    pub const GUILD_INVITE_FAIL: i16 = 2110;
    pub const GUILD_MEMBER_CONNECT: i16 = 2111;
    pub const GUILD_MEMBER_DISCONNECT_NOTIFY: i16 = 2112;
    pub const GUILD_KICK: i16 = 2106;
    pub const GUILD_LEAVE: i16 = 2107;
    pub const GUILD_MESSAGE: i16 = 2114;
    pub const GUILD_NOTICE: i16 = 2126;
    pub const GUILD_HISTORY: i16 = 2128;
    pub const GUILD_SET_TITLE: i16 = 2129;
    pub const TRADE_REQUEST: i16 = 1501;
    pub const TRADE_REQUEST_SUCCESS: i16 = 1501;
    pub const TRADE_REQUEST_ERROR: i16 = 1507;
    pub const TRADE_ACCEPT: i16 = 1502;
    pub const TRADE_CANCEL: i16 = 1506;
    pub const TRADE_ADD_ITEM: i16 = 1508;
    pub const TRADE_REMOVE_ITEM: i16 = 1531;
    pub const TRADE_REMOVE_ITEM_RESPONSE: i16 = 1519;
    pub const TRADE_ADD_MONEY: i16 = 1509;
    pub const TRADE_CONFIRM: i16 = 1503;
    pub const TRADE_FINAL_CONFIRMATION: i16 = 1504;
    pub const TRADE_LOCK: i16 = 1532;
    pub const TRADE_UNLOCK: i16 = 1505;
    pub const SEASON_PASS_DETAILS: i16 = 641;
    pub const SEASON_PASS_PURCHASE_EXP: i16 = 643;
    pub const SEASON_PASS_MISSION_REWARD: i16 = 644;
    pub const SEASON_PASS_SEASON_REWARD: i16 = 645;
    pub const CHANGE_CHANNEL: i16 = 1050;
    pub const CHANNEL_SWITCH_CONFIRM: i16 = 1703;
    pub const TAMER_SHOP_LIST: i16 = 1515;
    pub const CONSIGNSHOP_WAREHOUSE: i16 = 1523;
    pub const CONSIGNSHOP_WAREHOUSE_RETRIEVE: i16 = 1521;
    pub const CASHSHOP_BUY_HISTORY: i16 = 3412;
    pub const ADD_FRIEND: i16 = 2401;
    pub const GUILD_AUTHORITY_MASTER: i16 = 2119;
    pub const GUILD_AUTHORITY_SUBMASTER: i16 = 2118;
    pub const GUILD_AUTHORITY_MEMBER: i16 = 2116;
    pub const GUILD_AUTHORITY_NEW_MEMBER: i16 = 2115;
    pub const GUILD_AUTHORITY_DATS: i16 = 2117;
    pub const SPIRIT_TO_DIGIMON: i16 = 3239;
    pub const DIGIMON_TO_SPIRIT: i16 = 3240;
    pub const UNKNOWN_DIGIMON_3241: i16 = 3241;
    pub const DIGI_SUMMON_PURCHASE: i16 = 3651;
    pub const DIGI_SUMMON_PURCHASE_RESPONSE: i16 = 3651;
    // Digivice combine and union combine flows, plus the random box draw.
    pub const DIGI_COMBINE_SYNC: i16 = 3661;
    pub const DIGI_COMBINE: i16 = 3662;
    pub const DIGI_COMBINE_REWARD: i16 = 3663;
    pub const UNION_COMBINE_SYNC: i16 = 4301;
    pub const UNION_COMBINE: i16 = 4302;
    pub const UNION_COMBINE_REWARD: i16 = 4303;
    // Modern D-Unit / Union hacking and init flows.
    pub const UNION_HACK_OPEN_REQUEST: i16 = 4311;
    pub const UNION_HACK_OPEN_RESPONSE: i16 = 4311;
    pub const UNION_HACK_MODIFY_REQUEST: i16 = 4312;
    pub const UNION_HACK_MODIFY_RESPONSE: i16 = 4312;
    pub const UNION_INIT_DATA: i16 = 4313;
    pub const RANDOM_BOX_LIST: i16 = 16067;
    pub const RANDOM_BOX_PURCHASE: i16 = 16068;
    pub const LOAD_ACCOUNT_WAREHOUSE: i16 = 3930;
    pub const RETRIEVE_ACCOUNT_WAREHOUSE: i16 = 3931;
    pub const PARTY_MEMBER_DISCONNECT: i16 = 2312;
    pub const MONSTER_RESPAWN_TIMER: i16 = 16064;
    pub const JUMP_BOOSTER: i16 = 1057;
    pub const SKILL_LEVEL_UP: i16 = 1104;
    pub const TAMER_CHARGE_XCRYSTAL: i16 = 16030;
    pub const TAMER_CONSUME_XCRYSTAL: i16 = 16031;
    pub const TAMER_SUMMON: i16 = 1114;
    pub const TAMER_SKILL_REQUEST: i16 = 1327;
    pub const TRANSCENDENCE_RECEIVE_EXP: i16 = 16039;
    pub const TRANSCENDENCE_SUCCESS: i16 = 16040;
    pub const TIME_CHARGE_RESULT: i16 = 16042;
    pub const WARP_GATE_DUNGEON: i16 = 4119;
    pub const SKILL_UPDATE_COOLDOWN: i16 = 3246;
    // Skill-limit open notice {u4 flag; u4 digimon_row; 28-byte skill blob;
    // u4 skill_index; u4 expected} — direct digimon-table mutation plus an
    // empty event-477 ping (44996 registry 3245/0xDCA180).
    pub const SKILL_LIMIT_OPEN: i16 = 3245;
    // Enchant/tactics decrease notice applied to the local digimon:
    // {u2 anchor (client stores value - 1); u2 x14 in two runs of seven};
    // the loop pairs run one with run two into seven info-list entries,
    // decrements a 69-byte skill-table row and pings event 718 plus an
    // empty event-476 (44996 registry 1083/0xD96B90).
    pub const DIGIMON_DECREASE_INCHANT: i16 = 1083;
    // Enchant notice {u4 digimon handler; u2 anchor (client stores value - 1);
    // u2 x14 in two runs of seven}; same pairing loop as the decrease variant,
    // then a full stat recompute through the base-stats/evolution/digimon
    // tables and events 41, 167 and 476 (44996 registry 1075/0xD95DF0).
    pub const DIGIMON_INCHANT: i16 = 1075;
    // Coat/vanity slot notices: 2201 all {u4 handler} (entity type 14 calls a
    // vanity slot setter), 2202 savers {u4 value; u4 consumed}, 2203
    // adventure {u4 value; u4 handler} with type 13 routing through the tamer
    // vanity table and type 14 through the digimon one
    // (44996 registry 2201/0xDAF300, 2202/0xDAF380, 2203/0xDAF3F0).
    pub const ITEM_COAT_ALL: i16 = 2201;
    pub const ITEM_COAT_SAVERS: i16 = 2202;
    pub const ITEM_COAT_ADVENTURE: i16 = 2203;
    // Slot-size updates: 3924 inventory {u2 size} -> ev458 + generic ev353
    // node {103, size}; 3925 warehouse {u2 size} -> ev61 + ev353 node
    // {104, size} (44996 registry 3924/0xDAE3A0, 3925/0xDAE280).
    pub const ITEM_INVEN_SLOT_SIZE: i16 = 3924;
    pub const ITEM_WAREHOUSE_SLOT_SIZE: i16 = 3925;
    // Use-failure notice {u2 code; u4 item serial; u4 failure kind}: kind 1
    // opens a failure message box driven by the CItemTable row, kind 13 pings
    // ev154, kind 19 beeps, other kinds just reset the client
    // (44996 registry 3902/0xDAD060).
    pub const ITEM_USE_FAILURE: i16 = 3902;
    // Portable-item failure {u4 code}: switch 7701 pings ev212, 7702 and 7703
    // gate on prior portable-item usage (44996 registry 3943/0xDAFE70).
    pub const ITEM_PORTABLE_FAIL: i16 = 3943;
    // Crop-money flow {u4; u4; u4; u4}: the first two u4s form one i64 and
    // the trailing pair feeds the coin delta sums; closes three fixed UI
    // handlers first (23100/22900/23000), plays the coin sound and pings
    // ev671 (44996 registry 3911/0xDA5280).
    pub const ITEM_CROP_MONEY: i16 = 3911;
    // Limited-attribute change {u2; u1} -> event 585 with an u1 gate slot
    // (44996 registry 3934/0xDACE80).
    pub const ITEM_CHANGE_LIMITED_ATTRIBUTE: i16 = 3934;
    // Enchant-info notice: {u4 digimon handler; u2 base; five u2 values} —
    // the five u2s insert into fixed positions 0, 1, 2, 4 and 6 of the
    // digimon's enchant-info list, then the client refreshes that digimon
    // (44996 registry 3214/0xDCDA00).
    pub const DIGIMON_INCHANT_INFO: i16 = 3214;
    // Macro-result family: 1066 failure {u4} -> ev359, 1067 success with no
    // payload (local zeroed struct -> ev359), 1068 result {u4} -> ev360
    // (44996 registry 1066/0xDB63D0, 1067/0xDB6390, 1068/0xDB6170).
    pub const MACRO_FAILURE: i16 = 1066;
    pub const MACRO_SUCCESS: i16 = 1067;
    pub const MACRO_RESULT: i16 = 1068;
    // Experience-gain notice: five i64 quantity pairs plus two u4 fields, the
    // fourth read being consumed as u16 and the last one feeding a float exp
    // path; every quantity enters the same x100 fixed-point conversion
    // (44996 registry 1018/0xDA6890).
    pub const GAME_GET_EXP: i16 = 1018;
    // Target-tracking notice {u4 target handler; u1 flag; u4; u4} resolved
    // through the entity lookup and applied only to non-filtered types
    // (44996 registry 1017/0xDA6210).
    pub const GAME_TARGET_INFO: i16 = 1017;
    // Stat-change notice {u4 handler; u2 count; count x {u2 stat; u4 value}};
    // stat selector 1..10 and 12 dispatch into distinct setters, then an
    // empty event-590/476 ping depending on entity type
    // (44996 registry 1084/0xDAC0F0).
    pub const GAME_CHANGE_STAT: i16 = 1084;
    // Silent-state notice {u2; u4}: both fields are consumed with no dispatch
    // (44996 registry 1062/0xD085B0).
    pub const GAME_EVENT: i16 = 1062;
    // Message-all failure and incubator error share parser 0xD866F0: one u4
    // is consumed and dropped, no dispatch.
    pub const MSG_ALL_FAILURE: i16 = 1071;
    pub const INCUBATER_ERROR: i16 = 3228;
    pub const FRIENDLY_INFO: i16 = 3129;
}
