pub const CYAN:    &str = "#40009FE3";
pub const MAGENTA: &str = "#40E5007E";
pub const YELLOW:  &str = "#40FFED00";
pub const BLACK:   &str = "#401C1B1A";
// pub const RED:     &str = "#FF0000";
// pub const GREEN:   &str = "#00FF00";
// pub const BLUE:    &str = "#0000FF";

pub const TRANS: &str ="#C0";

pub fn trans(color: &str) -> String {
    color.replace("#40", TRANS)
}

#[derive(Debug)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn to_hex(&self) -> String {
        format!("#40{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl From<&[u8; 3]> for Rgb {
    fn from(a: &[u8; 3]) -> Rgb {
        Rgb {
            red: a[0], green: a[1], blue: a[2] 
        }
    }
}

#[test]
fn rgbhex() {
    let rgb = Rgb { red: 0, green: 128, blue: 255 };
    assert_eq!(rgb.to_hex(), "#0080FF");
}