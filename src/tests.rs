use super::repr::*;
use super::units::*;
use super::{Op, Prim, Shp};
use std::sync::Arc;

#[test]
fn primitives() {
    let size = 0.5;
    let circle = Shp::circ(size);
    assert_eq!(circle, Shp::Pr(Prim::Oval(Dim(Len(size), Len(size)))));
    assert_eq!(circle.size(), 1);
    assert_eq!(circle.to_float_vector(), vec![OVAL, size, size]);

    let width = 1.0;
    let height = 0.5;
    let oval = Shp::oval([width, height]);
    assert_eq!(oval, Shp::Pr(Prim::Oval(Dim(Len(width), Len(height)))));
    assert_eq!(oval.size(), 1);
    assert_eq!(oval.to_float_vector(), vec![OVAL, width, height]);

    let arch = Shp::arch([width, height]);
    assert_eq!(arch, Shp::Pr(Prim::Arch(Dim(Len(width), Len(height)))));
    assert_eq!(arch.size(), 1);
    assert_eq!(arch.to_float_vector(), vec![ARCH, width, height]);

    let square = Shp::squa(size);
    assert_eq!(square, Shp::Pr(Prim::Rect(Dim(Len(size), Len(size)))));
    assert_eq!(square.size(), 1);
    assert_eq!(square.to_float_vector(), vec![RECT, size, size]);

    let triangle = Shp::tria([width, height]);
    assert_eq!(triangle, Shp::Pr(Prim::Tria(Dim(Len(width), Len(height)))));
    assert_eq!(triangle.size(), 1);
    assert_eq!(triangle.to_float_vector(), vec![TRIA, width, height]);
}

#[test]
fn operations_on_primitives() {
    let width = 0.5;
    let height = 1.0;
    let oval = Shp::oval([width, height]);
    let rectangle = Shp::rect([width, height]);

    let union = Shp::union(vec![oval.clone(), rectangle.clone()]);

    assert_eq!(union, Shp::Op(Arc::new(Op::Union(vec![oval, rectangle]))));
    assert_eq!(union.size(), 3);
    assert_eq!(
        union.to_float_vector(),
        vec![UNION, 1.0, 0.0, OVAL, width, height, RECT, width, height, UNION, 0.0, 0.0]
    );
}

#[test]
fn fill_operations() {
    let circle = Shp::circ(0.5).fill([1.0, 0.5, 0.0]);
    let r = u8::MAX;
    let g = u8::MAX / 2;
    let b = 0;

    assert_eq!(
        circle,
        Shp::Op(Arc::new(Op::Fill(
            Clr(r, g, b),
            Shp::Pr(Prim::Oval(Dim(Len(0.5), Len(0.5))))
        )))
    );

    assert_eq!(
        circle.to_float_vector(),
        vec![FILL, 0xFF7F00 as f32, 0.0, OVAL, 0.5, 0.5]
    );
}
