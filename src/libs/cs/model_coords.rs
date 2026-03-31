
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXy {
    pub x: f64,
    pub y: f64, 
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXyPolar {
    pub r_xy: f64,
    pub f_yx: f64, // w radianach
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXyCylindricalZ {
    pub r_xy: f64,
    pub f_yx: f64, // w radianach
    pub z: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXz {
    pub x: f64,
    pub z: f64, 
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dXzPolar {
    pub r_xz: f64,
    pub f_zx: f64, // w radianach
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXzCylindricalY {
    pub r_xz: f64,
    pub f_zx: f64, // w radianach
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dYz {
    pub y: f64,
    pub z: f64, 
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords2dYzPolar {
    pub r_yz: f64,
    pub f_zy: f64, // w radianach
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dYzCylindricalX {
    pub r_yz: f64,
    pub f_zy: f64, // w radianach
    pub x: f64, 
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXyz {
    pub x: f64,
    pub y: f64, 
    pub z: f64, 
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords3dXyzSpherical {
    pub r_xyz: f64,
    pub f_yx: f64, // w radianach
    pub t_zr: f64, // w radianach
}
