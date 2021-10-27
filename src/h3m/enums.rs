#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    S = 36,
    M = 72,
    L = 108,
    XL = 144,
    H = 180,
    XH = 216,
    G = 252,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Version {
    RoE,
    AB,
    SoD,
    Chr,
    WoG,
    HotA,
}
