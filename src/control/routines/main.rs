use std::time::Duration;

use strum::IntoEnumIterator;

use crate::control::actions::{express::{Express, Expression}, general::{Sequence, WaitFor}};

pub fn main_sequence() -> Sequence {
    let mut sequence  = Sequence::new("Main Sequence");
    Expression::iter().for_each(|expression|  {
        sequence.enqueue(Express::new(expression));
        sequence.enqueue(WaitFor::new(Duration::from_millis(500)));
    });

    sequence
}
