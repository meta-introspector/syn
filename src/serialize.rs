//pub fn serialize_ident(){}
use serde::Serializer;
//use crate::serialize::serialize_ident;
// impl Serialize for proc_macro2::Ident {
//     fn serialize<S>(&self,serializer:S)-> Result<S>
//     where
// 	S: Serializer,
//     {
// 	serializer.serialize_str(self.ident);
//     }
// }


//pub fn serialize_ident<S:Serializer>(id: & proc_macro2::Ident,serializer:S)->Result<serde::Serializer>{
//
//}


pub fn serialize_ident<S>(ident: &proc_macro2::Ident, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let str_repr = ident.to_string();
    serializer.serialize_str(&str_repr)
}

