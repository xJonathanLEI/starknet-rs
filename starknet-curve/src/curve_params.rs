use starknet_ff::FieldElement;

use crate::ec_point::AffinePoint;

pub const EC_ORDER: FieldElement = FieldElement::from_mont([
    8939893405601011193,
    1143265896874747514,
    9,
    369010039416812937,
]);

pub const ALPHA: FieldElement = FieldElement::from_mont([
    18446744073709551585,
    18446744073709551615,
    18446744073709551615,
    576460752303422960,
]);

pub const BETA: FieldElement = FieldElement::from_mont([
    3863487492851900874,
    7432612994240712710,
    12360725113329547591,
    88155977965380735,
]);

pub const GENERATOR: AffinePoint = AffinePoint {
    x: FieldElement::from_mont([
        14484022957141291997,
        5884444832209845738,
        299981207024966779,
        232005955912912577,
    ]),
    y: FieldElement::from_mont([
        6241159653446987914,
        664812301889158119,
        18147424675297964973,
        405578048423154473,
    ]),
    infinity: false,
};

pub const SHIFT_POINT: AffinePoint = AffinePoint {
    x: FieldElement::from_mont([
        1933903796324928314,
        7739989395386261137,
        1641324389046377921,
        316327189671755572,
    ]),
    y: FieldElement::from_mont([
        14252083571674603243,
        12587053260418384210,
        4798858472748676776,
        81375596133053150,
    ]),
    infinity: false,
};
pub const PEDERSEN_P0: AffinePoint = AffinePoint {
    x: FieldElement::from_mont([
        3602345268353203007,
        13758484295849329960,
        518715844721862878,
        241691544791834578,
    ]),
    y: FieldElement::from_mont([
        13441546676070136227,
        13001553326386915570,
        433857700841878496,
        368891789801938570,
    ]),
    infinity: false,
};
pub const PEDERSEN_P1: AffinePoint = AffinePoint {
    x: FieldElement::from_mont([
        16491878934996302286,
        12382025591154462459,
        10043949394709899044,
        253000153565733272,
    ]),
    y: FieldElement::from_mont([
        13950428914333633429,
        2545498000137298346,
        5191292837124484988,
        285630633187035523,
    ]),
    infinity: false,
};
pub const PEDERSEN_P2: AffinePoint = AffinePoint {
    x: FieldElement::from_mont([
        1203723169299412240,
        18195981508842736832,
        12916675983929588442,
        338510149841406402,
    ]),
    y: FieldElement::from_mont([
        12352616181161700245,
        11743524503750604092,
        11088962269971685343,
        161068411212710156,
    ]),
    infinity: false,
};
pub const PEDERSEN_P3: AffinePoint = AffinePoint {
    x: FieldElement::from_mont([
        1145636535101238356,
        10664803185694787051,
        299781701614706065,
        425493972656615276,
    ]),
    y: FieldElement::from_mont([
        8187986478389849302,
        4428713245976508844,
        6033691581221864148,
        345457391846365716,
    ]),
    infinity: false,
};
