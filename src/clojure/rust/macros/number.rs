//! # All numbers and `Number` protocol
//!
//! Here will be all the number management, casting, operations
//! and functions.

#[macro_export]
macro_rules! number_def {
    ($name:ident, $def:ty) => {
        #[derive(Debug)]
        pub struct $name {
            value: $def,
        }

        castable_to!($name => [sync] TObject, Number);

        impl $name {
            pub fn new(value: $def) -> Object {
                Object::new(Some(Arc::new($name {value})))
            }
        }

        impl TObject for $name {
            fn get_class<'a>(&self) -> &'a SClass {
                todo!()
            }

            fn get_hash(&self) -> usize {
                todo!()
            }

            fn equals(&self, other: &Object) -> bool {
                todo!()
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "$Name {:?}", self.value)
            }
        }

        /// Implementation of Number trait
        impl Number for $name {
            fn big_integer_value_o(&self) -> Object {
                BigInteger::new(self.big_integer_value())
            }

            fn long_value_o(&self) -> Object {
                Long::new(self.long_value())
            }

            fn int_value_o(&self) -> Object {
                Integer::new(self.int_value())
            }

            fn short_value_o(&self) -> Object {
                Short::new(self.short_value())
            }

            fn byte_value_o(&self) -> Object {
                Byte::new(self.byte_value())
            }

            fn double_value_o(&self) -> Object {
                Double::new(self.double_value())
            }

            fn float_value_o(&self) -> Object {
                Float::new(self.float_value())
            }

            fn usize_value_o(&self) -> Object {
                Usize::new(self.usize_value())
            }
            fn big_integer_value(&self) -> i128 {
                self.value as i128
            }

            fn long_value(&self) -> i64 {
                self.value as i64
            }

            fn int_value(&self) -> i32 {
                self.value as i32
            }

            fn short_value(&self) -> i16 {
                self.value as i16
            }

            fn byte_value(&self) -> i8 {
                self.value as i8
            }

            fn double_value(&self) -> f64 {
                self.value as f64
            }

            fn float_value(&self) -> f32 {
                self.value as f32
            }

            fn usize_value(&self) -> usize {
                self.value as usize
            }
        }

        /// default value of $name is default of $def
        impl Default for $name {
            fn default() -> Self {
                $name {
                    value: <$def>::default(),
                }
            }
        }
    };
}
