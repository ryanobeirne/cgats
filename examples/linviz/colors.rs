// // Be careful here... the alpha prefix must match `SOLID`
// pub const CYAN:    &str = "#40009FE3";
// pub const MAGENTA: &str = "#40E5007E";
// pub const YELLOW:  &str = "#40FFED00";
// pub const BLACK:   &str = "#401C1B1A";
// pub const RED:     &str = "#FF0000";
// pub const GREEN:   &str = "#00FF00";
// pub const BLUE:    &str = "#0000FF";

// The level of transparency we want to use
const SOLID: &str = "#40";
const TRANS: &str = "#CC";

// Make a color transparent for gnuplot
pub fn trans(color: &str) -> String {
    color.replace(SOLID, TRANS)
}

// An RGB color object
#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    // Convert RGB to hexadecimal
    pub fn to_hex(&self) -> String {
        format!("{}{:02X}{:02X}{:02X}", SOLID, self.red, self.green, self.blue)
    }
}

impl Default for Rgb {
    fn default() -> Self {
        Rgb {
            red: 255,
            green: 255,
            blue: 255,
        }
    }
}

impl From<&[u8; 3]> for Rgb {
    // Convert an array of u8 to Rgb
    fn from(a: &[u8; 3]) -> Rgb {
        Rgb {
            red: a[0], green: a[1], blue: a[2] 
        }
    }
}

#[test]
fn rgbhex() {
    let rgb = Rgb { red: 0, green: 128, blue: 255 };
    assert_eq!(rgb.to_hex(), format!("{}0080FF", SOLID));
}