fn main()
{
    let age = 111u8;

    let majorité = match age {
        0..=17 => "Mineur",
        _x => "Majeur",
    };

    println!("{}", majorité);
}