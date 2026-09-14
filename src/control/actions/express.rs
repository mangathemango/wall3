use std::fmt::Display;

use strum::{EnumIter, EnumString};

use crate::{ROBOT, control::actions::Action};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Express {
    expression: Expression,
}

impl Express {
    pub fn new(expression: Expression) -> Self {
        Self {expression}
    }
}

impl Action for Express {
    fn start(&mut self) {
        ROBOT.stm32_controller().set_display_text(self.expression.to_string());
    }
}

impl Display for Express {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expressing {:?}", self.expression)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumString)]
pub enum Expression {
    Happy,     
    Curious,    
    Surprise,   
    Sad,       
    Idle,      
    Sleep,      
    Smug,      
    Dead,
    Blush,
    Bruh,
    Disappointed 
}

impl Expression {
    pub fn all() {

    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expression_str = match &self {
            Expression::Happy =>        "  'V'  ",
            Expression::Curious =>      "  '3'? ",
            Expression::Surprise =>     "  'o'! ",
            Expression::Sad =>          "  '^'  ",
            Expression::Idle =>         "  '-'  ",
            Expression::Sleep =>        "  -.-zz",
            Expression::Smug =>         "  -v-  ",
            Expression::Dead =>         "  x-x  ",
            Expression::Blush =>        " -///- ",
            Expression::Bruh =>         "  -~-  ",
            Expression::Disappointed => "  =.=? ",
        };
        write!(f, "{}", expression_str)
    }
}