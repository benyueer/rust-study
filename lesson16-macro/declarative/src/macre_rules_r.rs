#[macro_export]
macro_rules! add {
    ( $x: expr ) => {
        {
            $x
        }
    };

    ( $a: expr, $b: expr ) => {
        {
            $a + $b
        }
    };

    ( $a: expr, $b: expr, $typ: ty) => {
        {
            $a as $typ + $b as $typ
        }
    };
    ($a: expr, $($b: tt)*) => {
        {
            $a+add!($($b)*)
        }
    }
}

#[macro_export]
macro_rules! myvec {
    ( $($x: expr), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! add_as {
    ($($x: expr), *) => {
        {
            0$(+$x)*
        }
    };
}

#[macro_export]
macro_rules! ok_or_return {
    ($a: ident($($b: tt)*)) => {
        {
            match $a($($b)*) {
                Ok(value) => value,
                Err(err) => {
                    return Err(err as String)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! ok_or_return_err {
    (@error $a: ident, $($b: tt)*) => {
        {
            match $a($($b)*) {
                Ok(value)=>value,
                Err(err)=>{
                    return Err(err);
                }
            }
        }
    };

    ($a: ident($($b: tt)*)) => {
        ok_or_return_err!(@error $a, $($b)*)
    };
}


#[macro_export]
macro_rules! make_public {
    (
        $(#[$meta: meta])*
        $vis: vis struct $struct_name: ident {
            $(
                $(#[$field_meta: meta])*
                $field_vis: vis $field_name: ident : $field_type: ty
            ),*$(,)+
        }
    ) => {
        
            $(#[$meta])*
            pub struct $struct_name {
                $(
                    $(#[$field_meta])*
                    pub $field_name: $field_type,
                )*
            }
        
    };
}