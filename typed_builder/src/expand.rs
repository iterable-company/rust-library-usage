#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod stuff {
    use derive_new::new;
    use typed_builder::TypedBuilder;
    pub struct Stuff {
        name: String,
        age: u8,
        address: String,
    }
    impl Stuff {
        /**
                    Create a builder for building `Stuff`.
                    On the builder, call `.name(...)`, `.age(...)`, `.address(...)` to set the values of the fields.
                    Finally, call `.build()` to create the instance of `Stuff`.
                    */
        #[allow(dead_code, clippy::default_trait_access)]
        pub fn builder() -> StuffBuilder<((), (), ())> {
            StuffBuilder {
                fields: ((), (), ()),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[must_use]
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub struct StuffBuilder<TypedBuilderFields> {
        fields: TypedBuilderFields,
        phantom: (),
    }
    impl<TypedBuilderFields> Clone for StuffBuilder<TypedBuilderFields>
    where
        TypedBuilderFields: Clone,
    {
        #[allow(clippy::default_trait_access)]
        fn clone(&self) -> Self {
            Self {
                fields: self.fields.clone(),
                phantom: ::core::default::Default::default(),
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub trait StuffBuilder_Optional<T> {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T;
    }
    impl<T> StuffBuilder_Optional<T> for () {
        fn into_value<F: FnOnce() -> T>(self, default: F) -> T {
            default()
        }
    }
    impl<T> StuffBuilder_Optional<T> for (T,) {
        fn into_value<F: FnOnce() -> T>(self, _: F) -> T {
            self.0
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__address, __age> StuffBuilder<((), __age, __address)> {
        pub fn name(self, name: String) -> StuffBuilder<((String,), __age, __address)> {
            let name = (name,);
            let (_, age, address) = self.fields;
            StuffBuilder {
                fields: (name, age, address),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum StuffBuilder_Error_Repeated_field_name {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__address, __age> StuffBuilder<((String,), __age, __address)> {
        #[deprecated(note = "Repeated field name")]
        pub fn name(
            self,
            _: StuffBuilder_Error_Repeated_field_name,
        ) -> StuffBuilder<((String,), __age, __address)> {
            self
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__address, __name> StuffBuilder<(__name, (), __address)> {
        pub fn age(self, age: u8) -> StuffBuilder<(__name, (u8,), __address)> {
            let age = (age,);
            let (name, _, address) = self.fields;
            StuffBuilder {
                fields: (name, age, address),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum StuffBuilder_Error_Repeated_field_age {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__address, __name> StuffBuilder<(__name, (u8,), __address)> {
        #[deprecated(note = "Repeated field age")]
        pub fn age(
            self,
            _: StuffBuilder_Error_Repeated_field_age,
        ) -> StuffBuilder<(__name, (u8,), __address)> {
            self
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__age, __name> StuffBuilder<(__name, __age, ())> {
        pub fn address(
            self,
            address: String,
        ) -> StuffBuilder<(__name, __age, (String,))> {
            let address = (address,);
            let (name, age, _) = self.fields;
            StuffBuilder {
                fields: (name, age, address),
                phantom: self.phantom,
            }
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum StuffBuilder_Error_Repeated_field_address {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl<__age, __name> StuffBuilder<(__name, __age, (String,))> {
        #[deprecated(note = "Repeated field address")]
        pub fn address(
            self,
            _: StuffBuilder_Error_Repeated_field_address,
        ) -> StuffBuilder<(__name, __age, (String,))> {
            self
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum StuffBuilder_Error_Missing_required_field_name {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl<__address, __age> StuffBuilder<((), __age, __address)> {
        #[deprecated(note = "Missing required field name")]
        pub fn build(self, _: StuffBuilder_Error_Missing_required_field_name) -> Stuff {
            { ::std::rt::begin_panic("explicit panic") };
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum StuffBuilder_Error_Missing_required_field_age {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl<__address> StuffBuilder<((String,), (), __address)> {
        #[deprecated(note = "Missing required field age")]
        pub fn build(self, _: StuffBuilder_Error_Missing_required_field_age) -> Stuff {
            { ::std::rt::begin_panic("explicit panic") };
        }
    }
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, non_snake_case)]
    pub enum StuffBuilder_Error_Missing_required_field_address {}
    #[doc(hidden)]
    #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
    impl StuffBuilder<((String,), (u8,), ())> {
        #[deprecated(note = "Missing required field address")]
        pub fn build(
            self,
            _: StuffBuilder_Error_Missing_required_field_address,
        ) -> Stuff {
            { ::std::rt::begin_panic("explicit panic") };
        }
    }
    #[allow(dead_code, non_camel_case_types, missing_docs)]
    impl StuffBuilder<((String,), (u8,), (String,))> {
        #[allow(clippy::default_trait_access)]
        pub fn build(self) -> Stuff {
            let (name, age, address) = self.fields;
            let name = name.0;
            let age = age.0;
            let address = address.0;
            Stuff { name, age, address }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Stuff {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Stuff {
        #[inline]
        fn eq(&self, other: &Stuff) -> bool {
            self.name == other.name && self.age == other.age
                && self.address == other.address
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Stuff {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Stuff",
                "name",
                &&self.name,
                "age",
                &&self.age,
                "address",
                &&self.address,
            )
        }
    }
    impl Stuff {
        ///Constructs a new `Stuff`.
        pub fn new(name: String, age: u8, address: String) -> Self {
            Stuff {
                name: name,
                age: age,
                address: address,
            }
        }
    }
}
use stuff::Stuff;
fn main() {
    let stuff = Stuff::builder()
        .name(String::from("Someone"))
        .age(11)
        .address(String::from("Somewhere"))
        .build();
    match (&stuff, &Stuff::new(String::from("Someone"), 11, String::from("Somewhere"))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    }
}
