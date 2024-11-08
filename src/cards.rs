use super::structs::*;

pub const RACES: [Race; 3] = [
    Race {
        id: 0,
        name: "Royal",
    },
    Race {
        id: 1,
        name: "Humanoid",
    },
    Race {
        id: 2,
        name: "Undead",
    },
];

pub const ROYAL_DECK: [Card; 30] = [
    Card {
        id: 0,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "royal_knight",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 1,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "royal_knight",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 2,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "royal_knight",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 6,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "royal_priest",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 7,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "royal_priest",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 8,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "royal_priest",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 9,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "royal_soldier",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 10,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "royal_soldier",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 11,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "royal_soldier",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 12,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "royal_shield",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 13,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "royal_shield",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 14,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "royal_shield",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 18,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "royal_cleric",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 19,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "royal_cleric",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 20,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "royal_cleric",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 21,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "royal_swordman",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 22,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "royal_swordman",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 23,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "royal_swordman",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 3,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "royal_champion",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 4,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "royal_champion",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 5,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "royal_champion",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 15,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "royal_paladin",
        race: 0,
        rarity: 2,
    },
    Card {
        id: 16,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "royal_paladin",
        race: 0,
        rarity: 2,
    },
    Card {
        id: 17,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "royal_paladin",
        race: 0,
        rarity: 2,
    },
    Card {
        id: 24,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "royal_angel",
        race: 0,
        rarity: 2,
    },
    Card {
        id: 25,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "royal_angel",
        race: 0,
        rarity: 2,
    },
    Card {
        id: 26,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "royal_angel",
        race: 0,
        rarity: 2,
    },
    Card {
        id: 27,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "royal_crusade",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 28,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "royal_crusade",
        race: 0,
        rarity: 0,
    },
    Card {
        id: 29,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "royal_crusade",
        race: 0,
        rarity: 0,
    },
];

pub const HUMANOID_DECK: [Card; 30] = [
    Card {
        id: 0,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_archer",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 1,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_archer",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 2,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_archer",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 6,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_assasin",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 7,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_assasin",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 8,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_assasin",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 9,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_fanatic",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 10,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_fanatic",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 11,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_fanatic",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 12,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "humanoid_grunt",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 13,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "humanoid_grunt",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 14,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "humanoid_grunt",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 18,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_occultist",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 19,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_occultist",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 20,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "humanoid_occultist",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 21,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "humanoid_pikeman",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 22,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "humanoid_pikeman",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 23,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "humanoid_pikeman",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 3,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_tinker",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 4,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_tinker",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 5,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_tinker",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 15,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_wanderer",
        race: 1,
        rarity: 2,
    },
    Card {
        id: 16,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_wanderer",
        race: 1,
        rarity: 2,
    },
    Card {
        id: 17,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_wanderer",
        race: 1,
        rarity: 2,
    },
    Card {
        id: 24,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_fox",
        race: 1,
        rarity: 2,
    },
    Card {
        id: 25,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_fox",
        race: 1,
        rarity: 2,
    },
    Card {
        id: 26,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "humanoid_fox",
        race: 1,
        rarity: 2,
    },
    Card {
        id: 27,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "humanoid_wolfrider",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 28,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "humanoid_wolfrider",
        race: 1,
        rarity: 0,
    },
    Card {
        id: 29,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "humanoid_wolfrider",
        race: 1,
        rarity: 0,
    },
];

pub const UNDEAD_DECK: [Card; 30] = [
    Card {
        id: 0,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "undead_archer",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 1,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "undead_archer",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 2,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "undead_archer",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 6,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "undead_bones",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 7,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "undead_bones",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 8,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "undead_bones",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 9,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "undead_crawler",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 10,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "undead_crawler",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 11,
        mana: 2,
        hp: 2,
        attack: 1,
        curr_hp: 0,
        name: "undead_crawler",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 12,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "undead_eye",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 13,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "undead_eye",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 14,
        mana: 3,
        hp: 5,
        attack: 0,
        curr_hp: 0,
        name: "undead_eye",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 18,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "undead_feader",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 19,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "undead_feader",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 20,
        mana: 3,
        hp: 3,
        attack: 1,
        curr_hp: 0,
        name: "undead_feader",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 21,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "undead_revenant",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 22,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "undead_revenant",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 23,
        mana: 2,
        hp: 1,
        attack: 2,
        curr_hp: 0,
        name: "undead_revenant",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 3,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "undead_scarab",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 4,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "undead_scarab",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 5,
        mana: 4,
        hp: 2,
        attack: 3,
        curr_hp: 0,
        name: "undead_scarab",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 15,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "undead_stumbler",
        race: 2,
        rarity: 2,
    },
    Card {
        id: 16,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "undead_stumbler",
        race: 2,
        rarity: 2,
    },
    Card {
        id: 17,
        mana: 5,
        hp: 4,
        attack: 3,
        curr_hp: 0,
        name: "undead_stumbler",
        race: 2,
        rarity: 2,
    },
    Card {
        id: 24,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "undead_ghoul",
        race: 2,
        rarity: 2,
    },
    Card {
        id: 25,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "undead_ghoul",
        race: 2,
        rarity: 2,
    },
    Card {
        id: 26,
        mana: 4,
        hp: 3,
        attack: 3,
        curr_hp: 0,
        name: "undead_ghoul",
        race: 2,
        rarity: 2,
    },
    Card {
        id: 27,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "undead_hand",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 28,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "undead_hand",
        race: 2,
        rarity: 0,
    },
    Card {
        id: 29,
        mana: 6,
        hp: 1,
        attack: 6,
        curr_hp: 0,
        name: "undead_hand",
        race: 2,
        rarity: 0,
    },
];
