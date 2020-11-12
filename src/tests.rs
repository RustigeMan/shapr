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
    let circle = Shp::oval([width, height]);
    let square = Shp::rect([width, height]);

    let union = Shp::union(vec![circle.clone(), square.clone()]);

    assert_eq!(union, Shp::Op(Arc::new(Op::Union(vec![circle, square]))));
    assert_eq!(union.size(), 3);
    assert_eq!(
        union.to_float_vector(),
        vec![UNION, 1.0, 0.0, OVAL, width, height, RECT, width, height, UNION, 0.0, 0.0]
    );
}
