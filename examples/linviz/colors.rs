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
#[allow(unused)]
pub fn trans(color: &str) -> String {
    let bytes = color.as_bytes();
    let base = match bytes.len() {
        // FFF
        3 => format!("0{}0{}0{}", bite(&bytes, 0), bite(&bytes, 1), bite(&bytes, 2)),
        // #FFF
        4 => format!("0{}0{}0{}", bite(&bytes, 1), bite(&bytes, 2), bite(&bytes, 3)),
        // #FFFFFF
        6 => color.to_string(),
        // #FFFFFF
        7 => String::from_utf8_lossy(&bytes[1..]).to_string(),
        // CCFFFFFF
        8 => String::from_utf8_lossy(&bytes[2..]).to_string(),
        // #CCFFFFFF
        9 => String::from_utf8_lossy(&bytes[3..]).to_string(),
        _ => panic!(format!("'{}': Not valid hexadecimal color!", color))
    };

    if base.len() != 6 || !i64::from_str_radix(&base, 16).is_ok() {
        panic!("'{} => {}': Not a valid hexadecimal color!", color, base);
    }

    format!("{}{}", TRANS, base)
}

fn bite(bytes: &[u8], index: usize) -> String {
    String::from_utf8_lossy(&bytes[index..=index]).to_string()
}

#[test]
fn trans_color() {
    let base = "0080FF";
    let expected = format!("{}{}", TRANS, base);

    let color0 = "#0080FF";
    let color0a = "0080FF";
    let color1 = "#CC0080FF";
    let color1a = "CC0080FF";
    let color2 = "#400080FF";
    let color2a = "400080FF";
    let color3 = "#FFF";
    let color3a = "FFF";

    assert_eq!(expected, trans(color0));
    assert_eq!(expected, trans(color0a));
    assert_eq!(expected, trans(color1));
    assert_eq!(expected, trans(color1a));
    assert_eq!(expected, trans(color2));
    assert_eq!(expected, trans(color2a));
    assert_eq!("#CC0F0F0F", trans(color3));
    assert_eq!("#CC0F0F0F", trans(color3a));
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
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    pub fn to_hex_solid(&self) -> String {
        let hex = self.to_hex().trim_start_matches("#").to_string();
        format!("{}{}", SOLID, hex)
    }

    pub fn to_hex_trans(&self) -> String {
        let hex = self.to_hex().trim_start_matches("#").to_string();
        format!("{}{}", TRANS, hex)
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
    assert_eq!(rgb.to_hex(), "#0080FF");
    assert_eq!(rgb.to_hex_solid(), format!("{}0080FF", SOLID));
    assert_eq!(rgb.to_hex_trans(), format!("{}0080FF", TRANS));
}