macro_rules! make_foo
{
    (
        $foobar: ident,
        $i_foobar: ident,
        (
            $(
                (
                    $element: ident: $ty: ty,
                    $getter: ident: $setter: ident
                )
            ),*)
    )
    =>
    {
        struct $foobar
        {
            id: u64,
            $($element: $ty),*
        }

        trait $i_foobar
        {
            $(
                fn $getter(&self) -> $ty;
                fn $setter(&mut self, $element: $ty);
            )*
        }

        impl $i_foobar for $foobar
        {
            $(
                fn $getter(&self) -> $ty{
                    self.$element
                }
                fn $setter(&mut self, $element: $ty) {
                    self.$element = $element;
                }
            )*
        }
    }
}

pub(crate) use make_foo;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
