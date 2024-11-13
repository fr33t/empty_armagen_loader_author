use armagen::*;

fn main() {
    let payload = r2sc();

    let mut obf = Obfuscator::new();

    obf.obfuscate(&payload);

    gen(&obf);
}
