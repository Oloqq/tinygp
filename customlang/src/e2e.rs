use crate::params::Params;
use crate::tinygp::TinyGP;
use std::{io::{self, Write}, cell::RefCell};

#[test]
#[ignore]
fn test_e2e_identity() {
    let identity = "1 | 3
1 | 1
2 | 2
3 | 3";
    assert!(true);
    let (params, cases) = Params::from_string(identity.to_string()).unwrap();
    let writer = RefCell::new(Box::new(io::stdout()));

    let mut tgp = TinyGP::new(params, cases, Some(1), writer);
    tgp.evolve(2);
    assert!(false);
}