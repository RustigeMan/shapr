use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos(pub f32, pub f32); // Position

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vel(pub f32, pub f32); // Velocity

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Len(pub f32); // Length

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dim(pub Len, pub Len); // Dimensions (width, height)

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rad(pub f32); // Radius

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rot(pub f32); // Rotation

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scl(pub f32); // Scale

#[derive(Debug, Clone, PartialEq)]
pub struct Clr(pub u8, pub u8, pub u8); // Color

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dlt<T>(pub T); // Delta

impl<T> Dlt<T> {
    pub fn as_delta(val: T) -> Dlt<T> {
        Dlt(val)
    }
}

pub trait ToPos {
    fn to_pos(self) -> Pos;
}
impl ToPos for (f32, f32) {
    fn to_pos(self) -> Pos {
        let (x, y) = self;
        Pos(x, y)
    }
}
impl ToPos for [f32; 2] {
    fn to_pos(self) -> Pos {
        Pos(self[0], self[1])
    }
}
impl ToPos for Pos {
    fn to_pos(self) -> Pos {
        self
    }
}

pub trait ToLen {
    fn to_len(self) -> Len;
}
impl ToLen for f32 {
    fn to_len(self) -> Len {
        Len(self)
    }
}
impl ToLen for Len {
    fn to_len(self) -> Len {
        self
    }
}

pub trait ToDim {
    fn to_dim(self) -> Dim;
}
impl ToDim for Dim {
    fn to_dim(self) -> Dim {
        self
    }
}
impl ToDim for (f32, f32) {
    fn to_dim(self) -> Dim {
        let (w, h) = self;
        Dim(Len(w), Len(h))
    }
}
impl ToDim for [f32; 2] {
    fn to_dim(self) -> Dim {
        Dim(Len(self[0]), Len(self[1]))
    }
}

pub trait ToRot {
    fn to_rot(self) -> Rot;
}
impl ToRot for f32 {
    fn to_rot(self) -> Rot {
        Rot(self)
    }
}
impl ToRot for Rot {
    fn to_rot(self) -> Rot {
        self
    }
}

pub trait ToScl {
    fn to_scl(self) -> Scl;
}
impl ToScl for f32 {
    fn to_scl(self) -> Scl {
        Scl(self)
    }
}
impl ToScl for Scl {
    fn to_scl(self) -> Scl {
        self
    }
}

pub trait ToClr {
    fn to_clr(self) -> Clr;
}
impl ToClr for (u8, u8, u8) {
    fn to_clr(self) -> Clr {
        let (r, g, b) = self;
        Clr(r, g, b)
    }
}
impl ToClr for [u8; 3] {
    fn to_clr(self) -> Clr {
        Clr(self[0], self[1], self[2])
    }
}

impl ToClr for [f32; 3] {
    fn to_clr(self) -> Clr {
        let [r, g, b] = self;
        let r = if r > 1.0 { 1.0 } else { r };
        let g = if r > 1.0 { 1.0 } else { g };
        let b = if b > 1.0 { 1.0 } else { b };

        let max = u8::MAX as f32;
        let r = r * max;
        let g = g * max;
        let b = b * max;

        Clr(r as u8, g as u8, b as u8)
    }
}
impl ToClr for Clr {
    fn to_clr(self) -> Clr {
        self
    }
}

pub trait ToDlt<T> {
    fn to_delta(self) -> Dlt<T>;
}

impl<T> ToDlt<T> for T {
    fn to_delta(self) -> Dlt<T> {
        Dlt(self)
    }
}
/*
impl<T> ToDlt<T> for Dlt<T> {
    fn to_dlt(self) -> Dlt<T> {
        self
    }
}
*/
impl ToDlt<Pos> for (f32, f32) {
    fn to_delta(self) -> Dlt<Pos> {
        let (dx, dy) = self;
        Dlt(Pos(dx, dy))
    }
}
impl ToDlt<Pos> for [f32; 2] {
    fn to_delta(self) -> Dlt<Pos> {
        Dlt(Pos(self[0], self[1]))
    }
}

impl ToDlt<Rot> for f32 {
    fn to_delta(self) -> Dlt<Rot> {
        Dlt(Rot(self))
    }
}

impl ToDlt<Scl> for f32 {
    fn to_delta(self) -> Dlt<Scl> {
        Dlt(Scl(self))
    }
}

impl Pos {
    pub fn translate(&mut self, trn: &Dlt<Pos>) {
        let Dlt(Pos(dx, dy)) = trn;
        self.0 += dx;
        self.1 += dy;
    }
}

impl Mul<f32> for Pos {
    type Output = Pos;

    fn mul(self, other: f32) -> Pos {
        Pos(self.0 * other, self.1 * other)
    }
}

impl Div<f32> for Pos {
    type Output = Pos;

    fn div(self, other: f32) -> Pos {
        Pos(self.0 / other, self.1 / other)
    }
}

impl<T> Mul<f32> for Dlt<T>
where
    T: Mul<f32>,
{
    type Output = Dlt<T::Output>;

    fn mul(self, other: f32) -> Dlt<T::Output> {
        Dlt(self.0 * other)
    }
}

impl<T> Div<f32> for Dlt<T>
where
    T: Div<f32>,
{
    type Output = Dlt<T::Output>;

    fn div(self, other: f32) -> Dlt<T::Output> {
        Dlt(self.0 / other)
    }
}

impl Add<Dlt<Pos>> for Pos {
    type Output = Pos;

    fn add(self, other: Dlt<Pos>) -> Pos {
        let Dlt(Pos(dx, dy)) = other;
        Pos(self.0 + dx, self.1 + dy)
    }
}

impl Rot {
    pub fn rotate(&mut self, drot: &Dlt<Rot>) {
        let Dlt(Rot(drot)) = drot;
        self.0 += drot;
    }
}

impl Scl {
    pub fn scale(&mut self, dscale: &Dlt<Scl>) {
        let Dlt(Scl(dscale)) = dscale;
        self.0 *= dscale;
    }
}
