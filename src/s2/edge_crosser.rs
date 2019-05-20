// Copyright 2017 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License

use ::s2::point::Point;
use ::s2::predicates::Direction;

// EdgeCrosser allows edges to be efficiently tested for intersection with a
// given fixed edge AB. It is especially efficient when testing for
// intersection with an edge chain connecting vertices v0, v1, v2, ...
//
// Example usage:
//
//	func CountIntersections(a, b Point, edges []Edge) int {
//		count := 0
//		crosser := NewEdgeCrosser(a, b)
//		for _, edge := range edges {
//			if crosser.CrossingSign(&edge.First, &edge.Second) != DoNotCross {
//				count++
//			}
//		}
//		return count
//	}
//
pub struct EdgeCrosser {
    a: Point,
    b: Point,
    aXb: Point,

    // To reduce the number of calls to expensiveSign, we compute an
	// outward-facing tangent at A and B if necessary. If the plane
	// perpendicular to one of these tangents separates AB from CD (i.e., one
	// edge on each side) then there is no intersection.
    a_tangent: Point, // Outward-facing tangent at A. 
    b_tangent: Point, // Outward-facing tangent at B. 

   // The fields below are updated for each vertex in the chain
    c: Point,  // Previous vertex in the vertex chain.
    acb: ::s2::predicates::Direction,
}

impl EdgeCrosser {
    // new returns an EdgeCrosser with the fixed edge AB.
    pub fn new(a: Point, b: Point) -> EdgeCrosser {
        let norm = a.point_cross(&b);
        EdgeCrosser {
            a: a.clone(),
            b: b.clone(),
            aXb: a.cross(&b),
            a_tangent: a.cross(&norm),
            b_tangent: norm.cross(&b),
            c: Point(::r3::vector::Vector::new(0., 0., 0.)),
            acb: Direction::Indeterminate,
        }
    }

    pub fn crossing_sign(&self, c: Point, d: Point) -> Crossing {
        unimplemented!()
    }
}

