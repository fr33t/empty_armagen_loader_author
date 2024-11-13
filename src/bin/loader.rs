use armagen::Obfuscation;
mod z;

fn main() {
    #[cfg(feature = "separated")]
    {
        use armagen::{parese_z, Obfuscator};
        let o: Obfuscator = serde_json::from_str(&parese_z()).unwrap();
        o.exec();
    }
    #[cfg(not(feature = "separated"))]
    {
        use z::O;
        O.exec();
    }
}
