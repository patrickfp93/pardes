mod struture;
pub(crate) mod utilities {
    use syn::{parse::Parse, parse_str};

    pub fn to_items<'a, T>(strs: &[&'a str]) -> Result<Vec<T>, syn::Error>
    where
        T: Parse,
    {
        strs.iter()
            .map(|s| parse_str::<T>(s))
            .collect::<Result<Vec<_>, _>>()
    }
}
/*pub mod macro_utilities {

    #[macro_export]
    macro_rules! expose_for_tests {

        ($i:item) =>{
            $i

            #[cfg(test)]
            pub $i
        };
    }
}*/
