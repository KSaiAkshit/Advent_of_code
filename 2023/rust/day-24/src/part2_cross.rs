use crate::custom_error::AocError;

use glam::{DVec2, DVec3, I64Vec2, I64Vec3, Vec3Swizzles};
use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
struct Hail {
    starting_position: I64Vec3, // (x1, y1, z2)
    direction: I64Vec3,         // (l,m,n)
}

// 18, 19, 22 @
fn ivec(input: &str) -> IResult<&str, I64Vec3> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = terminated(tag(","), space1)(input)?;
    let (input, z) = complete::i64(input)?;
    Ok((input, I64Vec3::new(x, y, z)))
}

fn parse_hails(input: &str) -> IResult<&str, Vec<Hail>> {
    separated_list1(
        line_ending,
        separated_pair(ivec, delimited(space1, tag("@"), space1), ivec).map(
            |(starting_position, direction)| Hail {
                starting_position,
                direction,
            },
        ),
    )(input)
}
// xi + t*vxi = xj + t*vyj
// xi - xj = t* (vyj-vxi)
// t = (xi - xj)/ (vyj-vxi)
// t = (yi - yj)/ (vyj-vxi)
//  (yi - yj)/ (vyj-vxi) = (xi - xj)/ (vyj-vxi)
// Hailstone A: 19, 13, 30 @ -2, 1, -2
// Hailstone B: 18, 19, 22 @ -1, -1, -2
// Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

// Rock - H1
// (xr, yr, zr)   (xh, yh, zh)
// (ur, vr, wr)   (uh, vh, wh)
// for time t

// t = (xr-xh)/(uh-ur) = (yr-yh)/(vh-vr) = (zr-zh)/(wh-wr)

// (xr-xh)(vh-vr) - (uh-ur)(yr-yh) = 0
// (yr-yh)(wh-wr) - (zr-zh)(vh-vr) = 0

// need 6 equations -> 6 unknowns.
// get other three equations from H1, H2, H3

// P - Pi = t ( V - Vi)
// Vector = Scalar * Vector -> Vectors are parallel, crossproduct is 0
// (P - Pi) x (V - Vi) = 0
// (Vi-Vj)×(Pi-Pj)⋅P = (Vi-Vj)⋅Pi×Pj
// Pi = Coords of hail i

/*
(V0-V1)×(P0-P1)⋅P = (V0-V1)⋅P0×P1
(V0-V2)×(P0-P2)⋅P = (V0-V2)⋅P0×P2
(V1-V2)×(P1-P2)⋅P = (V1-V2)⋅P1×P2
*/
fn gen_equations(h1: Hail, h2: Hail, h3: Hail) -> DVec3 {
    let hails = [h1, h2, h3];
    let mut sPos: Vec<I64Vec3> = Vec::new();
    let mut sVel: Vec<I64Vec3> = Vec::new();
    hails.iter().for_each(|h| {
        sPos.push(h.starting_position);
        sVel.push(h.direction);
    });
    // dbg!(sPos, sVel);

    let lhs: Vec<_> = hails
        .iter()
        .tuple_combinations()
        // .inspect(|(x, y)| {
        //     dbg!(x, y);
        // })
        .map(|(h1, h2)| {
            let p1 = h1.starting_position;
            let p2 = h2.starting_position;
            let v1 = h1.direction;
            let v2 = h2.direction;
            let diff_p = p2 - p1;
            let diff_v = v2 - v1;
            // dbg!(diff_p);
            // dbg!(diff_v);

            (v2 - v1).cross(p2 - p1).as_dvec3().to_array()
        })
        // .inspect(|cross| {
        //     dbg!(cross);
        // })
        .collect();
    let rhs: Vec<_> = hails
        .iter()
        .circular_tuple_windows()
        // .inspect(|(x, y)| {
        //     dbg!(x, y);
        // })
        .map(|(h1, h2)| {
            let p1 = h1.starting_position;
            let p2 = h2.starting_position;
            let v1 = h1.direction;
            let v2 = h2.direction;

            let diff_p = p2 - p1;
            let diff_v = v2 - v1;
            // dbg!(diff_p);
            // dbg!(diff_v);
            (v2 - v1).dot(p2.cross(p1)) as f64
        })
        // .inspect(|dot| {
        //     dbg!(dot);
        // })
        .collect();
    // dbg!(lhs, rhs);

    let coeff_matrix: Array2<f64> = Array2::from(lhs);
    let const_matrix: Array1<f64> = Array1::from(rhs);
    // dbg!(&coeff_matrix, &const_matrix);

    let result = coeff_matrix
        .solve(&const_matrix)
        .unwrap_or(Array::default(3));
    let xx = result[0];
    let yy = result[1];
    let zz = result[2];
    DVec3::new(xx, yy, zz)

    // dbg!(result);

    // let p1 = h1.starting_position;
    // let p2 = h2.starting_position;
    // let p3 = h3.starting_position;
    // let v1 = h1.direction;
    // let v2 = h2.direction;
    // let v3 = h3.direction;
    // let p: DVec3 = DVec3::default();
    // let v: DVec3 = DVec3::default();
    // let cross = p1.cross(p2);
    // // dbg!(cross);
    // // dbg!(p1, p2, p3, v1, v2, v3, p, v);
}

fn solve_linalg(h1: Hail, h2: Hail, h3: Hail) -> DVec2 {
    let a = [(h1.direction.y), (h2.direction.y)];
    let b = [(-h1.direction.x), (-h2.direction.x)];
    let c = [
        (h1.direction.y * h1.starting_position.x - h1.direction.x * h1.starting_position.y),
        (h2.direction.y * h2.starting_position.x - h2.direction.x * h2.starting_position.y),
    ];
    // dbg!(a, b, c);
    let coeff_matrix: Array2<f64> = array!([a[0] as f64, b[0] as f64], [a[1] as f64, b[1] as f64]);
    // let coeff_matrix: Array2<f64> = array![
    //     [h1.direction.y as f64, -h1.direction.x as f64],
    //     [h2.direction.y as f64, -h2.direction.x as f64]
    // ];
    let const_matrix: Array1<f64> = array!(c[0] as f64, c[1] as f64);
    let result = coeff_matrix
        .solve(&const_matrix)
        .unwrap_or(Array::default(2));
    // dbg!(coeff_matrix, const_matrix);
    let xx = result[0];
    let yy = result[1];
    if (h1.direction.x as f64 * (xx - h1.starting_position.x as f64) >= 0.0)
        && (h1.direction.y as f64 * (yy - h1.starting_position.y as f64) >= 0.0)
        && (h2.direction.x as f64 * (xx - h2.starting_position.x as f64) >= 0.0)
        && (h2.direction.y as f64 * (yy - h2.starting_position.y as f64) >= 0.0)
    {
        DVec2::new(result[0], result[1])
    } else {
        DVec2::default()
    }
}

fn check_parallel(h1: Hail, h2: Hail, h3: Hail) -> Option<DVec2> {
    let intersection = solve_linalg(h1, h2, h3);

    if (h1.direction.y * -h2.direction.x) != (h2.direction.y * -h1.direction.x) {
        Some(intersection)
    } else {
        None
    }
}

// fn solve_matrix(h1: Hail, h2: Hail) {
//     let coef_arr: Array2<f64> = array![
//         [h1.direction.x as f64, h2.direction.x as f64,],
//         [h1.direction.y as f64, h2.direction.y as f64,]
//     ];
//     dbg!(coef_arr);
// }

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, hail) = parse_hails(input).expect("Should pass");
    let bounds = 200000000000000f64..=400000000000000f64;

    let h1 = hail[0];
    let h2 = hail[1];
    let h3 = hail[2];
    // dbg!(h1, h2);
    // gen_equations(h1, h2, h3);

    // let res = solve_linalg(h1, h2);
    // dbg!(res);

    let result: Vec<_> = hail
        .iter()
        .tuple_combinations()
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .map(|(h1, h2, h3)| gen_equations(*h1, *h2, *h3))
        // .filter(|res| res.x != 0.0 && res.y != 0.0)
        // .filter(|dvec| {
        //     dvec.x != 0.0 && dvec.y != 0.0 && bounds.contains(&dvec.x) && bounds.contains(&dvec.y)
        // })
        // .inspect(|res| {
        //     dbg!(res);
        // })
        .collect();
    dbg!(result);
    // Ok(result.len().to_string())
    Ok("result".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // bounds of [7, 27] (inclusive)
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
