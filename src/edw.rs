use redox_ecc::edwards::{Curve as EdCurve, Ell2};
use redox_ecc::ellipticcurve::{EllipticCurve, MapToCurve, RationalMap};
use redox_ecc::field::{FromFactory, Sgn0Endianness};
use redox_ecc::instances::{
    edwards25519_to_curve25519, edwards448_to_curve448, EdCurveID, GetCurve, EDWARDS25519,
    EDWARDS448,
};
use redox_ecc::montgomery::Curve as MtCurve;

use crate::api::{Encoding, HashID, HashToCurve, MapID, Suite};

impl Suite<EdCurveID> {
    pub fn get(&self, dst: &[u8]) -> impl HashToCurve<E = EdCurve> {
        let dst = dst.to_vec();
        let e = self.curve.get();
        let f = e.get_field();
        let cofactor = e.new_scalar(e.get_cofactor());
        let ratmap: Option<Box<dyn RationalMap<E0 = EdCurve, E1 = MtCurve>>> =
            if self.curve == EDWARDS25519 {
                Some(Box::new(edwards25519_to_curve25519()))
            } else if self.curve == EDWARDS448 {
                Some(Box::new(edwards448_to_curve448()))
            } else {
                None
            };
        let map_to_curve: Box<dyn MapToCurve<E = EdCurve>> = match self.map {
            MapID::ELL2(z, s) => Box::new(Ell2::new(e, f.from(z), s, ratmap)),
            _ => unimplemented!(),
        };
        Encoding {
            hash_to_field: Box::new(f),
            dst,
            map_to_curve,
            cofactor,
            h: self.h,
            l: self.l,
            ro: self.ro,
        }
    }
}

pub static EDWARDS25519_SHA256_EDELL2_NU_: Suite<EdCurveID> = Suite {
    name: "edwards25519-SHA256-EDELL2-NU-",
    curve: EDWARDS25519,
    h: HashID::SHA256,
    map: MapID::ELL2(2, Sgn0Endianness::LittleEndian),
    l: 48,
    ro: false,
};
pub static EDWARDS25519_SHA256_EDELL2_RO_: Suite<EdCurveID> = Suite {
    name: "edwards25519-SHA256-EDELL2-RO-",
    ro: true,
    ..EDWARDS25519_SHA256_EDELL2_NU_
};

pub static EDWARDS25519_SHA512_EDELL2_NU_: Suite<EdCurveID> = Suite {
    name: "edwards25519-SHA512-EDELL2-NU-",
    curve: EDWARDS25519,
    map: MapID::ELL2(2, Sgn0Endianness::LittleEndian),
    h: HashID::SHA512,
    l: 48,
    ro: false,
};
pub static EDWARDS25519_SHA512_EDELL2_RO_: Suite<EdCurveID> = Suite {
    name: "edwards25519-SHA512-EDELL2-RO-",
    ro: true,
    ..EDWARDS25519_SHA512_EDELL2_NU_
};

pub static EDWARDS448_SHA512_EDELL2_NU_: Suite<EdCurveID> = Suite {
    name: "edwards448-SHA512-EDELL2-NU-",
    curve: EDWARDS448,
    map: MapID::ELL2(-1, Sgn0Endianness::LittleEndian),
    h: HashID::SHA512,
    l: 84,
    ro: false,
};
pub static EDWARDS448_SHA512_EDELL2_RO_: Suite<EdCurveID> = Suite {
    name: "edwards448-SHA512-EDELL2-RO-",
    ro: true,
    ..EDWARDS448_SHA512_EDELL2_NU_
};