pub mod units;

#[cfg(test)]
mod tests;

pub use units::{Clr, Dim, Dlt, Len, Pos, Rad, Rot, Scl};
pub use units::{ToClr, ToDim, ToDlt, ToLen, ToRot, ToScl};

//use std::iter::IntoIterator;
use std::sync::Arc;

// So an AST for simple shapes?
#[derive(Debug, PartialEq)]
pub enum Shp {
    Pr(Prim),
    Op(Arc<Op>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Prim {
    Nil,
    Oval(Dim),
    Arch(Dim),
    Rect(Dim),
    Tria(Dim),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Trans(Dlt<Pos>, Shp),
    Rotat(Dlt<Rot>, Shp),
    Origi(Dlt<Pos>, Shp),
    Scale(Dlt<Scl>, Shp),
    Compl(Shp),
    Fill(Clr, Shp),
    Outl(Len, Shp),
    Union(Vec<Shp>),
    Inter(Vec<Shp>),
}

mod repr {
    pub const NIL: f32 = 0.0;
    pub const OVAL: f32 = 1.0;
    pub const ARCH: f32 = 2.0;
    pub const RECT: f32 = 3.0;
    pub const TRIA: f32 = 4.0;

    pub const TRANS: f32 = 5.0;
    pub const ROTAT: f32 = 6.0;
    pub const ORIGI: f32 = 7.0;
    pub const SCALE: f32 = 8.0;
    pub const COMPL: f32 = 9.0;
    pub const FILL: f32 = 10.0;
    pub const OUTL: f32 = 11.0;
    pub const UNION: f32 = 12.0;
    pub const INTER: f32 = 13.0;
}

impl Shp {
    pub fn nil() -> Self {
        Shp::Pr(Prim::Nil)
    }
    pub fn circ<T>(len: T) -> Self
    where
        T: ToLen,
    {
        let len = len.to_len();
        Shp::Pr(Prim::Oval(Dim(len, len)))
    }
    pub fn oval<T>(dim: T) -> Self
    where
        T: ToDim,
    {
        Shp::Pr(Prim::Oval(dim.to_dim()))
    }
    pub fn arch<T>(dim: T) -> Self
    where
        T: ToDim,
    {
        Shp::Pr(Prim::Arch(dim.to_dim()))
    }
    pub fn squa<T>(len: T) -> Self
    where
        T: ToLen,
    {
        let len = len.to_len();
        Shp::Pr(Prim::Rect(Dim(len, len)))
    }
    pub fn rect<T>(dim: T) -> Self
    where
        T: ToDim,
    {
        Shp::Pr(Prim::Rect(dim.to_dim()))
    }
    pub fn tria<T>(dim: T) -> Self
    where
        T: ToDim,
    {
        Shp::Pr(Prim::Tria(dim.to_dim()))
    }

    pub fn trans<T>(self, trn: T) -> Self
    where
        T: ToDlt<Pos>,
    {
        let trn = trn.to_delta();
        Shp::Op(Arc::new(Op::Trans(trn, self)))
    }
    pub fn rotat<T>(self, rot: T) -> Self
    where
        T: ToDlt<Rot>,
    {
        let rot = rot.to_delta();
        Shp::Op(Arc::new(Op::Rotat(rot, self)))
    }
    pub fn origi<T>(self, origin_trn: T) -> Self
    where
        T: ToDlt<Pos>,
    {
        let origin_trn = origin_trn.to_delta();
        Shp::Op(Arc::new(Op::Origi(origin_trn, self)))
    }
    pub fn scale<T>(self, scale: T) -> Self
    where
        T: ToDlt<Scl>,
    {
        let scale = scale.to_delta();
        Shp::Op(Arc::new(Op::Scale(scale, self)))
    }

    pub fn union(shapes: Vec<Shp>) -> Self {
        Shp::Op(Arc::new(Op::Union(shapes)))
    }
    pub fn inter(shapes: Vec<Shp>) -> Self {
        Shp::Op(Arc::new(Op::Inter(shapes)))
    }
    pub fn compl(self) -> Self {
        Shp::Op(Arc::new(Op::Compl(self)))
    }

    pub fn fill<T>(self, color: T) -> Self
    where
        T: ToClr,
    {
        let color = color.to_clr();
        Shp::Op(Arc::new(Op::Fill(color, self)))
    }

    pub fn size(&self) -> usize {
        match self {
            Shp::Pr(_) => 1,
            Shp::Op(operation) => {
                use Op::*;
                1 + match &**operation {
                    Union(shapes) | Inter(shapes) => {
                        let mut size = 0;
                        for shape in shapes {
                            size += shape.size();
                        }
                        size
                    }
                    Trans(_, shape)
                    | Rotat(_, shape)
                    | Origi(_, shape)
                    | Scale(_, shape)
                    | Compl(shape)
                    | Fill(_, shape)
                    | Outl(_, shape) => shape.size(),
                }
            }
        }
    }

    pub fn to_float_vector(&self) -> Vec<f32> {
        let mut floats = Vec::with_capacity(self.size() * 3);
        self.add_to_float_vector(&mut floats);
        floats
    }

    fn add_to_float_vector(&self, floats: &mut Vec<f32>) {
        match self {
            Shp::Pr(primitive) => {
                use repr::*;
                use Prim::*;
                floats.push(match primitive {
                    Nil => NIL,
                    Arch(_) => ARCH,
                    Oval(_) => OVAL,
                    Rect(_) => RECT,
                    Tria(_) => TRIA,
                });

                match primitive {
                    Nil => {
                        floats.push(0.0);
                        floats.push(0.0);
                    }
                    Arch(dim) | Oval(dim) | Rect(dim) | Tria(dim) => {
                        let Dim(Len(width), Len(height)) = dim;
                        floats.push(*width);
                        floats.push(*height);
                    }
                }
            }

            Shp::Op(operation) => {
                use repr::*;
                use Op::*;

                match &**operation {
                    Union(shapes) => {
                        Self::push_instr(floats, UNION, 1.0, 0.0); // 1.0 indicates start of union

                        for shape in shapes {
                            shape.add_to_float_vector(floats);
                        }

                        Self::push_instr(floats, UNION, 0.0, 0.0); // 0.0 indicates end of union
                    }
                    Inter(shapes) => {
                        Self::push_instr(floats, INTER, 1.0, 0.0); // 1.0 indicates start of intersection

                        for shape in shapes {
                            shape.add_to_float_vector(floats);
                        }

                        Self::push_instr(floats, INTER, 0.0, 0.0); // 0.0 indicates end of intersection
                    }
                    Trans(translation, shape) => {
                        let Dlt(Pos(dx, dy)) = translation;
                        Self::push_instr(floats, TRANS, *dx, *dy);

                        shape.add_to_float_vector(floats);
                    }
                    Rotat(rotation, shape) => {
                        let Dlt(Rot(rotation)) = rotation;
                        Self::push_instr(floats, ROTAT, *rotation, 0.0);

                        shape.add_to_float_vector(floats);
                    }
                    Origi(origin, shape) => {
                        let Dlt(Pos(x, y)) = origin;
                        Self::push_instr(floats, ORIGI, *x, *y);

                        shape.add_to_float_vector(floats);
                    }
                    Scale(scaling, shape) => {
                        let Dlt(Scl(scale)) = scaling;
                        Self::push_instr(floats, SCALE, *scale, 0.0);

                        shape.add_to_float_vector(floats);
                    }
                    Compl(shape) => {
                        Self::push_instr(floats, COMPL, 0.0, 0.0);

                        shape.add_to_float_vector(floats);
                    }
                    Fill(color, shape) => {
                        Self::push_instr(floats, FILL, Self::pack_color_to_float(color), 0.0);

                        shape.add_to_float_vector(floats);
                    }
                    Outl(outline, shape) => {
                        let Len(width) = outline;
                        Self::push_instr(floats, OUTL, *width, 0.0);

                        shape.add_to_float_vector(floats);
                    }
                }
            }
        }
    }

    fn push_instr(floats: &mut Vec<f32>, instruction: f32, arg1: f32, arg2: f32) {
        floats.push(instruction);
        floats.push(arg1);
        floats.push(arg2);
    }

    fn pack_color_to_float(color: &Clr) -> f32 {
        let r = color.0 as u32;
        let g = color.1 as u32;
        let b = color.2 as u32;
        let packed_color = (r << 16) + (g << 8) + b;
        packed_color as f32
    }
}

// Make sure only the top RC gets cloned if a complex shape is cloned:
impl Clone for Shp {
    fn clone(&self) -> Self {
        match self {
            Shp::Pr(primitive) => Shp::Pr(*primitive),
            Shp::Op(shape_arc) => Shp::Op(shape_arc.clone()),
        }
    }
}
