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
pub fn serialize_delimspan<S>(a: &proc_macro2::extra::DelimSpan, serializer: S) -> std::result::Result<S::Ok, S::Error> where     S: serde::Serializer,{
    
    serializer.serialize_str(&format!("{:?}",a).to_string())
}
pub fn serialize_literal<S,T>(_a: T, serializer: S) -> std::result::Result<S::Ok, S::Error> where     S: serde::Serializer,{        serializer.serialize_str("unknown2")}

pub fn serialize_meta_list<S>(a: &crate::attr::MetaList, serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,{
	serializer.serialize_str(&format!("{:?}",a).to_string())
}

pub fn serialize_option_ident<S>(
    a: &std::option::Option<proc_macro2::Ident>,
    serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,{
    serializer.serialize_str(&format!("{:?}",a).to_string())
}
pub fn serialize_option_ident_token<S,T>(_a: T, serializer: S) -> std::result::Result<S::Ok, S::Error> where     S: serde::Serializer,{        serializer.serialize_str("unknown5")}
pub fn serialize_option_token_ident<S,T>(_a: T, serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,{        serializer.serialize_str("unknown6")}
pub fn serialize_path<S,T>(_a: T, serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,{        serializer.serialize_str("unknown7")}
pub fn serialize_punt_iter_field<S,T>(_a: T, serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,{        serializer.serialize_str("unknown8")}

pub fn serialize_span<S>(a: &proc_macro2::Span,
			 serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{:?}",a).to_string())
}

pub fn serialize_spans<S>(a: &[proc_macro2::Span], serializer: S) -> std::result::Result<S::Ok, S::Error> where     S: serde::Serializer,{
    serializer.serialize_str(&format!("{:?}",a).to_string())
}

pub fn serialize_token_stream<S,T>(_a: T, serializer: S) -> std::result::Result<S::Ok, S::Error> where    S: serde::Serializer,{        serializer.serialize_str("unknown11")}
pub fn serialize_unknown<S,T>(_a:T, serializer: S) -> std::result::Result<S::Ok, S::Error> where     S: serde::Serializer,{        serializer.serialize_str("unknown12")}

