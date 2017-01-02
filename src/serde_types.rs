use serde_yaml::Value;
use std::collections::HashMap;





#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CargoToml: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CargoToml {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CargoToml, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __ignore, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "package" => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"package" => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    CargoToml;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoToml, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < CargoPackage > (
                                       )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(CargoToml{package: __field0,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoToml, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<CargoPackage> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("package"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  CargoPackage > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "package" )),
                            };
                        Ok(CargoToml{package: __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["package"];
                deserializer.deserialize_struct("CargoToml", FIELDS,
                                                __Visitor)
            }
        }
    };
pub struct CargoToml {
    pub package: CargoPackage,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CargoPackage: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CargoPackage {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CargoPackage, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __ignore, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "metadata" => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"metadata" => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    CargoPackage;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoPackage, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < CargoMetadata > (
                                       )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(CargoPackage{metadata: __field0,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoPackage, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<CargoMetadata> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("metadata"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  CargoMetadata > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "metadata" )),
                            };
                        Ok(CargoPackage{metadata: __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["metadata"];
                deserializer.deserialize_struct("CargoPackage", FIELDS,
                                                __Visitor)
            }
        }
    };
pub struct CargoPackage {
    pub metadata: CargoMetadata,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CargoMetadata: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CargoMetadata {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CargoMetadata, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __ignore, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "sls-distribution" => {
                                        Ok(__Field::__field0)
                                    }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"sls-distribution" => {
                                        Ok(__Field::__field0)
                                    }
                                    _ => Ok(__Field::__ignore),
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    CargoMetadata;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoMetadata, __V::Error> where
                     __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < CargoDistribution
                                       > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(CargoMetadata{sls_distribution: __field0,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoMetadata, __V::Error> where
                     __V: _serde::de::MapVisitor {
                        let mut __field0: Option<CargoDistribution> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("sls-distribution"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  CargoDistribution > (  )));
                                }
                                _ => {
                                    let _ =
                                        try!(visitor . visit_value :: < _serde
                                             :: de :: impls :: IgnoredAny > (
                                             ));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field (
                                     "sls-distribution" )),
                            };
                        Ok(CargoMetadata{sls_distribution: __field0,})
                    }
                }
                const FIELDS: &'static [&'static str] = &["sls_distribution"];
                deserializer.deserialize_struct("CargoMetadata", FIELDS,
                                                __Visitor)
            }
        }
    };
pub struct CargoMetadata {
    pub sls_distribution: CargoDistribution,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_CargoDistribution: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for CargoDistribution {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<CargoDistribution, __D::Error> where
             __D: _serde::de::Deserializer {
                #[allow(non_camel_case_types)]
                enum __Field { __field0, __field1, __field2, __field3, }
                impl _serde::de::Deserialize for __Field {
                    #[inline]
                    fn deserialize<__D>(deserializer: &mut __D)
                     -> ::std::result::Result<__Field, __D::Error> where
                     __D: _serde::de::Deserializer {
                        struct __FieldVisitor;
                        impl _serde::de::Visitor for __FieldVisitor {
                            type
                            Value
                            =
                            __Field;
                            fn visit_usize<__E>(&mut self, value: usize)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    0usize => { Ok(__Field::__field0) }
                                    1usize => { Ok(__Field::__field1) }
                                    2usize => { Ok(__Field::__field2) }
                                    3usize => { Ok(__Field::__field3) }
                                    _ =>
                                    Err(_serde::de::Error::invalid_value("expected a field")),
                                }
                            }
                            fn visit_str<__E>(&mut self, value: &str)
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    "group" => { Ok(__Field::__field0) }
                                    "manifest_extensions" => {
                                        Ok(__Field::__field1)
                                    }
                                    "args" => { Ok(__Field::__field2) }
                                    "git_version" => { Ok(__Field::__field3) }
                                    _ =>
                                    Err(_serde::de::Error::unknown_field(value)),
                                }
                            }
                            fn visit_bytes<__E>(&mut self, value: &[u8])
                             -> ::std::result::Result<__Field, __E> where
                             __E: _serde::de::Error {
                                match value {
                                    b"group" => { Ok(__Field::__field0) }
                                    b"manifest_extensions" => {
                                        Ok(__Field::__field1)
                                    }
                                    b"args" => { Ok(__Field::__field2) }
                                    b"git_version" => {
                                        Ok(__Field::__field3)
                                    }
                                    _ => {
                                        let value =
                                            ::std::string::String::from_utf8_lossy(value);
                                        Err(_serde::de::Error::unknown_field(&value))
                                    }
                                }
                            }
                        }
                        deserializer.deserialize_struct_field(__FieldVisitor)
                    }
                }
                struct __Visitor;
                impl _serde::de::Visitor for __Visitor {
                    type
                    Value
                    =
                    CargoDistribution;
                    #[inline]
                    fn visit_seq<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoDistribution, __V::Error>
                     where __V: _serde::de::SeqVisitor {
                        let __field0 =
                            match try!(visitor . visit :: < String > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(0usize));
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit :: < HashMap < String ,
                                       Value > > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(1usize));
                                }
                            };
                        let __field2 =
                            match try!(visitor . visit :: < Vec < String > > (
                                        )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(2usize));
                                }
                            };
                        let __field3 =
                            match try!(visitor . visit :: < bool > (  )) {
                                Some(value) => { value }
                                None => {
                                    try!(visitor . end (  ));
                                    return Err(_serde::de::Error::invalid_length(3usize));
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(CargoDistribution{group: __field0,
                                             manifest_extensions: __field1,
                                             args: __field2,
                                             git_version: __field3,})
                    }
                    #[inline]
                    fn visit_map<__V>(&mut self, mut visitor: __V)
                     -> ::std::result::Result<CargoDistribution, __V::Error>
                     where __V: _serde::de::MapVisitor {
                        let mut __field0: Option<String> = None;
                        let mut __field1: Option<HashMap<String, Value>> =
                            None;
                        let mut __field2: Option<Vec<String>> = None;
                        let mut __field3: Option<bool> = None;
                        while let Some(key) =
                                  try!(visitor . visit_key :: < __Field > (
                                       )) {
                            match key {
                                __Field::__field0 => {
                                    if __field0.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("group"));
                                    }
                                    __field0 =
                                        Some(try!(visitor . visit_value :: <
                                                  String > (  )));
                                }
                                __Field::__field1 => {
                                    if __field1.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("manifest_extensions"));
                                    }
                                    __field1 =
                                        Some(try!(visitor . visit_value :: <
                                                  HashMap < String , Value > >
                                                  (  )));
                                }
                                __Field::__field2 => {
                                    if __field2.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("args"));
                                    }
                                    __field2 =
                                        Some(try!(visitor . visit_value :: <
                                                  Vec < String > > (  )));
                                }
                                __Field::__field3 => {
                                    if __field3.is_some() {
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::duplicate_field("git_version"));
                                    }
                                    __field3 =
                                        Some(try!(visitor . visit_value :: <
                                                  bool > (  )));
                                }
                            }
                        }
                        try!(visitor . end (  ));
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                try!(visitor . missing_field ( "group" )),
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None => ::std::default::Default::default(),
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None => ::std::default::Default::default(),
                            };
                        let __field3 =
                            match __field3 {
                                Some(__field3) => __field3,
                                None => ::std::default::Default::default(),
                            };
                        Ok(CargoDistribution{group: __field0,
                                             manifest_extensions: __field1,
                                             args: __field2,
                                             git_version: __field3,})
                    }
                }
                const FIELDS: &'static [&'static str] =
                    &["group", "manifest_extensions", "args", "git_version"];
                deserializer.deserialize_struct("CargoDistribution", FIELDS,
                                                __Visitor)
            }
        }
    };
pub struct CargoDistribution {
    pub group: String,
    pub manifest_extensions: HashMap<String, Value>,
    pub args: Vec<String>,
    pub git_version: bool,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Manifest: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::ser::Serialize for Manifest {
            fn serialize<__S>(&self, _serializer: &mut __S)
             -> ::std::result::Result<(), __S::Error> where
             __S: _serde::ser::Serializer {
                let mut __serde_state =
                    try!(_serializer . serialize_struct (
                         "Manifest" , 0 + 1 + 1 + 1 + 1 + 1 + 1 ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "manifest-version" , & self .
                     manifest_version ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "product-type" , & self .
                     product_type ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "product-group" , & self .
                     product_group ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "product-name" , & self .
                     product_name ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "product-version" , & self .
                     product_version ));
                try!(_serializer . serialize_struct_elt (
                     & mut __serde_state , "extensions" , & self . extensions
                     ));
                _serializer.serialize_struct_end(__serde_state)
            }
        }
    };
pub struct Manifest {
    pub manifest_version: String,
    pub product_type: String,
    pub product_group: String,
    pub product_name: String,
    pub product_version: String,
    pub extensions: HashMap<String, Value>,
}
