macro_rules! polynom {
    [$value:expr; $count:expr] => {{
        let array = [$value; $count];
        $crate::gf::poly::Polynom::from(&array[..])
    }}; 

    [$( $value:expr ),* ] => {{
        let array = [$($value, )*];
        $crate::gf::poly::Polynom::from(&array[..])
    }};
}
