#[macro_export]
macro_rules! go {
    // types
    (*string) => {&str};
    (*$type:ty) => {&mut $type};
    (*[]$type:ty) => {&[$type]};
    ([]$type:ty) => {Vec<$type>};
    ([$len:literal]$type:ty) => {[$type; $len]};
    (map [$key:ty]&val:ty) => {std::collections::HashMap<$key, $val>};

    // statements
    ($var:ident++) => {$var += 1};
    ($var:ident--) => {$var -= 1};
    ({
        $($body:stmt)*
    }) => {
        {
            $($body;)*
        }
    };
    (type $name:ident struct {
        $($prop:ident $type:ty),* $(,)?
    }) => {
        pub struct $name {
            $(pub $prop: $type,)*
        }
    };
    (type $name:ident $type:ty) => {
        pub type $name = $type;
    };
    (func $name:ident (
        $($param:ident $param_type:ty),*
    ) $($return_type:ty)? {
        $($body:stmt)*
    }) => {
        pub fn $name(
            $($param: $param_type,)*
        ) $(-> $return_type)? {
            go!({
                $($body)*
            })
        }
    };
    (func ($self:ident *$impl:ty) $name:ident (
        $($param:ident $param_type:ty),*
    ) $($return_type:ty)? {
        $($body:stmt)*
    }) => {
        impl $impl {
            pub fn $name(
                &mut self,
                $($param: $param_type,)*
            ) $(-> $return_type)? {
                let $self = self;
                go!({
                    $($body)*
                })
            }
        }
    };
    (func ($self:ident $impl:ty) $name:ident (
        $($param:ident $param_type:ty),*
    ) $($return_type:ty)? {
        $($body:stmt)*
    }) => {
        impl $impl {
            pub fn $name(
                self,
                $($param: $param_type,)*
            ) $(-> $return_type)? {
                let mut $self = self;
                go!({
                    $($body)*
                })
            }
        }
    };
    (return $($expr:expr),*) => {
        return (
            $($expr:expr),*
        )
    };
    (for $var:ident := range($val:expr) {
        $($body:stmt)*
    }) => {
        for $var in $val {
            go!({
                $($body)*
            })
        }
    };
    (for $(;;)? {
        $($body:stmt)*
    }) => {
        loop {
            go!({
                $($body)*
            })
        }
    };
    (for ($condition:expr) {
        $($body:stmt)*
    }) => {
        while $condition {
            go!({
                $($body)*
            })
        }
    };
    (for $var:ident := $($start:stmt)?; $condition:expr; $inc_var:ident++ {
        $($body:stmt)*
    }) => {
        go! {
            for let mut $var = $start; $condition; (go!($inc_var++)) {
                $($body)*
            }
        }
    };
    (for $var:ident := $($start:stmt)?; $condition:expr; $inc_var:ident-- {
        $($body:stmt)*
    }) => {
        go! {
            for let mut $var = $start; $condition; (go!($inc_var--)) {
                $($body)*
            }
        }
    };
    (for $($init:stmt)?; $condition:expr; ($($inc:stmt)?) {
        $($body:stmt)*
    }) => {
        {
            $($init;)?
            while $condition {
                go!({
                    $($body)*
                })
                $($inc;)?
            }
        }
    };

    // expressions
    ([]$type:ty {
        $($expr:expr),* $(,)?
    }) => {
        {
            let vec: Vec<$type> = vec![$($expr),*];
            vec
        }
    };
    ([$len:literal]$type:ty {
        $($expr:expr),* $(,)?
    }) => {
        {
            let arr: [$type; $len] = [$($expr),*];
            arr
        }
    };
    (map [$key_type:ty]$val_type:ty {
        $(($key:expr): $val:expr),* $(,)?
    }) => {
        {
            let mut map = std::collections::HashMap::<$key_type, $val_type>::new();
            $(map.insert($key, $val);)*
            map
        }
    };
    (func (
        $($param:ident $param_type:ty),*
    ) $($return_type:ty)? {
        $($body:stmt)*
    }) => {
        {
            |$($param: $param_type),*| $(-> $return_type)? {
                go!({$($body)*});
            }
        }
    };
    (($type:ty)($val:expr)) => {$val as $type};
    (len($expr:expr)) => {$expr.len()};
    (append(*$arr:expr, $($elem:expr),+)) => {
        {
            let arr: &[_] = $arr;
            let mut vec = arr.to_vec();
            $(vec.push($elem);)+
            vec
        }
    };
    (append($vec:expr, $($elem:expr),+)) => {
        {
            let vec: Vec<_> = $vec;
            let mut vec = vec.clone();
            $(vec.push($elem);)+
            vec
        }
    };
}
pub mod prelude {
    #![allow(non_camel_case_types)]
    pub type string = String;
    pub type int = isize;
    pub type int8 = i8;
    pub type int32 = i32;
    pub type int64 = i64;
    pub type int128 = i128;
    pub type uint = usize;
    pub type uint8 = u8;
    pub type uint32 = u32;
    pub type uint64 = u64;
    pub type uint128 = u128;
    pub type float32 = f32;
    pub type float64 = f64;
    pub type byte = u8;
    pub type rune = i32;
}