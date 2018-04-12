use math::Matrix4;
use helpers::Nums;
// use math::Vector;

use std::cmp::{Ord};
// use std::cmp::{ Eq, Ord, Ordering};
use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};


#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub struct Matrix3<T> {
    pub elements: [T; 9],
}


#[allow(dead_code)]
impl <T> Matrix3<T>
where T:Nums+MulAssign+AddAssign+SubAssign+Mul<Output=T>+Add<Output=T>+DivAssign+Sub<Output=T>+Ord+Neg<Output=T>+Clone+Div<Output=T>+Into<T>+From<f64>+From<f32>
{
    pub fn new() -> Self {
        Self {
            elements: [
				Nums::one(), Nums::zero(), Nums::zero(),
				Nums::zero(), Nums::one(), Nums::zero(),
				Nums::zero(), Nums::zero(), Nums::one()
		    ]
        }
    }

	pub fn get_normal_matrix (&mut self, matrix4: &Matrix4<T> ) -> &mut Self {
        let clone = &self.clone();
		self.set_from_matrix4( matrix4 ).get_inverse( clone, false ).unwrap().transpose()
	}


	pub fn set_from_matrix4 (&mut self,  m: &Matrix4<T> ) -> &mut Self {
		let me = m.elements;
		self.set(
			me[ 0 ], me[ 4 ], me[ 8 ],
			me[ 1 ], me[ 5 ], me[ 9 ],
			me[ 2 ], me[ 6 ], me[ 10 ]
		);
		self
	}

	pub fn set (&mut self, n11: T, n12: T, n13: T, n21: T, n22: T, n23: T, n31: T, n32: T, n33: T ) -> &mut Self {
		let mut te = self.elements;
		te[ 0 ] = n11; te[ 1 ] = n21; te[ 2 ] = n31;
		te[ 3 ] = n12; te[ 4 ] = n22; te[ 5 ] = n32;
		te[ 6 ] = n13; te[ 7 ] = n23; te[ 8 ] = n33;
		self
	}

	pub fn get_inverse (&mut self, matrix: &Matrix3<T>, throw_on_degenerate: bool ) -> Result<&mut Self, &str> {

		let me = matrix.elements;
		let mut te = self.elements;

		let n11 = me[ 0 ]; let n21 = me[ 1 ]; let n31 = me[ 2 ];
		let n12 = me[ 3 ]; let n22 = me[ 4 ]; let n32 = me[ 5 ];
		let n13 = me[ 6 ]; let n23 = me[ 7 ]; let n33 = me[ 8 ];

		let t11 = n33 * n22 - n32 * n23;
		let t12 = n32 * n13 - n33 * n12;
		let t13 = n23 * n12 - n22 * n13;

		let det = n11 * t11 + n21 * t12 + n31 * t13;

		if  det == Nums::zero()  {

			let msg = "THREE.Matrix3: .get_inverse() can't invert matrix, determinant is 0";
            eprintln!("{}", msg);

			if throw_on_degenerate == true {
				return Err(msg);
			}

			return Ok(self.identity());
		}

		let det_inv = Nums::one() / det;

		te[ 0 ] = t11 * det_inv;
		te[ 1 ] = ( n31 * n23 - n33 * n21 ) * det_inv;
		te[ 2 ] = ( n32 * n21 - n31 * n22 ) * det_inv;

		te[ 3 ] = t12 * det_inv;
		te[ 4 ] = ( n33 * n11 - n31 * n13 ) * det_inv;
		te[ 5 ] = ( n31 * n12 - n32 * n11 ) * det_inv;

		te[ 6 ] = t13 * det_inv;
		te[ 7 ] = ( n21 * n13 - n23 * n11 ) * det_inv;
		te[ 8 ] = ( n22 * n11 - n21 * n12 ) * det_inv;

		Ok(self)
	}

	pub fn identity (&mut self) -> &mut Self {
		self.set(
			Nums::one(), Nums::zero(), Nums::zero(),
			Nums::zero(), Nums::one(), Nums::zero(),
			Nums::zero(), Nums::zero(), Nums::one()
		);
		self
	}

    // 	transpose: function () {
    //
    // 		var tmp, m = this.elements;
    //
    // 		tmp = m[ 1 ]; m[ 1 ] = m[ 3 ]; m[ 3 ] = tmp;
    // 		tmp = m[ 2 ]; m[ 2 ] = m[ 6 ]; m[ 6 ] = tmp;
    // 		tmp = m[ 5 ]; m[ 5 ] = m[ 7 ]; m[ 7 ] = tmp;
    //
    // 		return this;
    //
    // 	},

	pub fn transpose (&mut self) -> &mut Self {
		let mut tmp; let mut m = self.elements;

		tmp = m[ 1 ]; m[ 1 ] = m[ 3 ]; m[ 3 ] = tmp;
		tmp = m[ 2 ]; m[ 2 ] = m[ 6 ]; m[ 6 ] = tmp;
		tmp = m[ 5 ]; m[ 5 ] = m[ 7 ]; m[ 7 ] = tmp;

        self
	}




}

// import { Vector3 } from './Vector3.js';
//
// /**
//  * @author alteredq / http://alteredqualia.com/
//  * @author WestLangley / http://github.com/WestLangley
//  * @author bhouston / http://clara.io
//  * @author tschw
//  */
//
// function Matrix3() {
//
// 	this.elements = [
//
// 		1, 0, 0,
// 		0, 1, 0,
// 		0, 0, 1
//
// 	];
//
// 	if ( arguments.length > 0 ) {
//
// 		console.error( 'THREE.Matrix3: the constructor no longer reads arguments. use .set() instead.' );
//
// 	}
//
// }
//
// Object.assign( Matrix3.prototype, {
//
// 	isMatrix3: true,
//


// 	clone: function () {
//
// 		return new this.constructor().fromArray( this.elements );
//
// 	},
//
// 	copy: function ( m ) {
//
// 		var te = this.elements;
// 		var me = m.elements;
//
// 		te[ 0 ] = me[ 0 ]; te[ 1 ] = me[ 1 ]; te[ 2 ] = me[ 2 ];
// 		te[ 3 ] = me[ 3 ]; te[ 4 ] = me[ 4 ]; te[ 5 ] = me[ 5 ];
// 		te[ 6 ] = me[ 6 ]; te[ 7 ] = me[ 7 ]; te[ 8 ] = me[ 8 ];
//
// 		return this;
//
// 	},
//
//
// 	applyToBufferAttribute: function () {
//
// 		var v1 = new Vector3();
//
// 		return function applyToBufferAttribute( attribute ) {
//
// 			for ( var i = 0, l = attribute.count; i < l; i ++ ) {
//
// 				v1.x = attribute.getX( i );
// 				v1.y = attribute.getY( i );
// 				v1.z = attribute.getZ( i );
//
// 				v1.applyMatrix3( this );
//
// 				attribute.setXYZ( i, v1.x, v1.y, v1.z );
//
// 			}
//
// 			return attribute;
//
// 		};
//
// 	}(),
//
// 	multiply: function ( m ) {
//
// 		return this.multiplyMatrices( this, m );
//
// 	},
//
// 	premultiply: function ( m ) {
//
// 		return this.multiplyMatrices( m, this );
//
// 	},
//
// 	multiplyMatrices: function ( a, b ) {
//
// 		var ae = a.elements;
// 		var be = b.elements;
// 		var te = this.elements;
//
// 		var a11 = ae[ 0 ], a12 = ae[ 3 ], a13 = ae[ 6 ];
// 		var a21 = ae[ 1 ], a22 = ae[ 4 ], a23 = ae[ 7 ];
// 		var a31 = ae[ 2 ], a32 = ae[ 5 ], a33 = ae[ 8 ];
//
// 		var b11 = be[ 0 ], b12 = be[ 3 ], b13 = be[ 6 ];
// 		var b21 = be[ 1 ], b22 = be[ 4 ], b23 = be[ 7 ];
// 		var b31 = be[ 2 ], b32 = be[ 5 ], b33 = be[ 8 ];
//
// 		te[ 0 ] = a11 * b11 + a12 * b21 + a13 * b31;
// 		te[ 3 ] = a11 * b12 + a12 * b22 + a13 * b32;
// 		te[ 6 ] = a11 * b13 + a12 * b23 + a13 * b33;
//
// 		te[ 1 ] = a21 * b11 + a22 * b21 + a23 * b31;
// 		te[ 4 ] = a21 * b12 + a22 * b22 + a23 * b32;
// 		te[ 7 ] = a21 * b13 + a22 * b23 + a23 * b33;
//
// 		te[ 2 ] = a31 * b11 + a32 * b21 + a33 * b31;
// 		te[ 5 ] = a31 * b12 + a32 * b22 + a33 * b32;
// 		te[ 8 ] = a31 * b13 + a32 * b23 + a33 * b33;
//
// 		return this;
//
// 	},
//
// 	multiplyScalar: function ( s ) {
//
// 		var te = this.elements;
//
// 		te[ 0 ] *= s; te[ 3 ] *= s; te[ 6 ] *= s;
// 		te[ 1 ] *= s; te[ 4 ] *= s; te[ 7 ] *= s;
// 		te[ 2 ] *= s; te[ 5 ] *= s; te[ 8 ] *= s;
//
// 		return this;
//
// 	},
//
// 	determinant: function () {
//
// 		var te = this.elements;
//
// 		var a = te[ 0 ], b = te[ 1 ], c = te[ 2 ],
// 			d = te[ 3 ], e = te[ 4 ], f = te[ 5 ],
// 			g = te[ 6 ], h = te[ 7 ], i = te[ 8 ];
//
// 		return a * e * i - a * f * h - b * d * i + b * f * g + c * d * h - c * e * g;
//
// 	},
//
//

//

//
// 	transposeIntoArray: function ( r ) {
//
// 		var m = this.elements;
//
// 		r[ 0 ] = m[ 0 ];
// 		r[ 1 ] = m[ 3 ];
// 		r[ 2 ] = m[ 6 ];
// 		r[ 3 ] = m[ 1 ];
// 		r[ 4 ] = m[ 4 ];
// 		r[ 5 ] = m[ 7 ];
// 		r[ 6 ] = m[ 2 ];
// 		r[ 7 ] = m[ 5 ];
// 		r[ 8 ] = m[ 8 ];
//
// 		return this;
//
// 	},
//
// 	setUvTransform: function ( tx, ty, sx, sy, rotation, cx, cy ) {
//
// 		var c = Math.cos( rotation );
// 		var s = Math.sin( rotation );
//
// 		this.set(
// 			sx * c, sx * s, - sx * ( c * cx + s * cy ) + cx + tx,
// 			- sy * s, sy * c, - sy * ( - s * cx + c * cy ) + cy + ty,
// 			0, 0, 1
// 		);
//
// 	},
//
// 	scale: function ( sx, sy ) {
//
// 		var te = this.elements;
//
// 		te[ 0 ] *= sx; te[ 3 ] *= sx; te[ 6 ] *= sx;
// 		te[ 1 ] *= sy; te[ 4 ] *= sy; te[ 7 ] *= sy;
//
// 		return this;
//
// 	},
//
// 	rotate: function ( theta ) {
//
// 		var c = Math.cos( theta );
// 		var s = Math.sin( theta );
//
// 		var te = this.elements;
//
// 		var a11 = te[ 0 ], a12 = te[ 3 ], a13 = te[ 6 ];
// 		var a21 = te[ 1 ], a22 = te[ 4 ], a23 = te[ 7 ];
//
// 		te[ 0 ] = c * a11 + s * a21;
// 		te[ 3 ] = c * a12 + s * a22;
// 		te[ 6 ] = c * a13 + s * a23;
//
// 		te[ 1 ] = - s * a11 + c * a21;
// 		te[ 4 ] = - s * a12 + c * a22;
// 		te[ 7 ] = - s * a13 + c * a23;
//
// 		return this;
//
// 	},
//
// 	translate: function ( tx, ty ) {
//
// 		var te = this.elements;
//
// 		te[ 0 ] += tx * te[ 2 ]; te[ 3 ] += tx * te[ 5 ]; te[ 6 ] += tx * te[ 8 ];
// 		te[ 1 ] += ty * te[ 2 ]; te[ 4 ] += ty * te[ 5 ]; te[ 7 ] += ty * te[ 8 ];
//
// 		return this;
//
// 	},
//
// 	equals: function ( matrix ) {
//
// 		var te = this.elements;
// 		var me = matrix.elements;
//
// 		for ( var i = 0; i < 9; i ++ ) {
//
// 			if ( te[ i ] !== me[ i ] ) return false;
//
// 		}
//
// 		return true;
//
// 	},
//
// 	fromArray: function ( array, offset ) {
//
// 		if ( offset === undefined ) offset = 0;
//
// 		for ( var i = 0; i < 9; i ++ ) {
//
// 			this.elements[ i ] = array[ i + offset ];
//
// 		}
//
// 		return this;
//
// 	},
//
// 	toArray: function ( array, offset ) {
//
// 		if ( array === undefined ) array = [];
// 		if ( offset === undefined ) offset = 0;
//
// 		var te = this.elements;
//
// 		array[ offset ] = te[ 0 ];
// 		array[ offset + 1 ] = te[ 1 ];
// 		array[ offset + 2 ] = te[ 2 ];
//
// 		array[ offset + 3 ] = te[ 3 ];
// 		array[ offset + 4 ] = te[ 4 ];
// 		array[ offset + 5 ] = te[ 5 ];
//
// 		array[ offset + 6 ] = te[ 6 ];
// 		array[ offset + 7 ] = te[ 7 ];
// 		array[ offset + 8 ] = te[ 8 ];
//
// 		return array;
//
// 	}
//
// } );
//
//
// export { Matrix3 };
