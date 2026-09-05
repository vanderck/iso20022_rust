// Generated from xmldsig-core-schema.xsd
#![allow(unused_imports, non_snake_case, non_camel_case_types)]
use serde::{Serialize, Deserialize};
use validator::Validate;
pub type CryptoBinary = String;
pub type DigestValueType = String;
pub type HMACOutputLengthType = i64;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
#[serde(rename = "ds:Signature")]
pub struct SignatureType {
    /// XMLDSIG Namespace
    #[serde(rename = "@xmlns:ds")]
    pub xmlns_ds:
        String
,
    /// Default NS
    #[serde(rename = "@xmlns")]
    pub xmlns:
        String
,
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "ds:SignedInfo", alias = "SignedInfo")]
    pub signed_info:
        SignedInfoType
,
    #[serde(rename = "ds:SignatureValue", alias = "SignatureValue")]
    pub signature_value:
        SignatureValueType
,
    #[serde(rename = "ds:KeyInfo", alias = "KeyInfo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_info:
Option<        KeyInfoType
>,
    #[serde(rename = "ds:Object", alias = "Object")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub object:
Vec<        ObjectType
>,
}
impl SignatureType {
    pub fn new() -> Self {
        Self {
            xmlns: "http://www.w3.org/2000/09/xmldsig#".to_string(),
            xmlns_ds: "http://www.w3.org/2000/09/xmldsig#".to_string(),
            ..Default::default()
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SignatureValueType {
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:Option<String>,
    /// Content
    #[serde(rename = "$value")]
    pub value:
        String
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[serde(rename = "ds:SignedInfo")]
pub struct SignedInfoType {
    /// XMLDSIG Namespace
    #[serde(rename = "@xmlns:ds")]
    pub xmlns_ds:
        String
,
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "ds:CanonicalizationMethod", alias = "CanonicalizationMethod")]
    pub canonicalization_method:
        CanonicalizationMethodType
,
    #[serde(rename = "ds:SignatureMethod", alias = "SignatureMethod")]
    pub signature_method:
        SignatureMethodType
,
    #[serde(rename = "ds:Reference", alias = "Reference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference:
Vec<        ReferenceType
>,
}
impl Default for SignedInfoType {
    fn default() -> Self {
        Self {
            xmlns_ds: "http://www.w3.org/2000/09/xmldsig#".to_string(),
            id: Default::default(),
            canonicalization_method: Default::default(),
            signature_method: Default::default(),
            reference: Default::default(),
        }
    }
}
impl SignedInfoType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CanonicalizationMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm:
        String
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SignatureMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm:
        String
,
    #[serde(rename = "HMACOutputLength")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hmac_output_length:
Option<        HMACOutputLengthType
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ReferenceType {
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "@URI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri:
Option<        String
>,
    #[serde(rename = "@Type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type:
Option<        String
>,
    #[serde(rename = "ds:Transforms", alias = "Transforms")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transforms:
Option<        TransformsType
>,
    #[serde(rename = "ds:DigestMethod", alias = "DigestMethod")]
    pub digest_method:
        DigestMethodType
,
    #[serde(rename = "ds:DigestValue", alias = "DigestValue")]
    pub digest_value:
        DigestValueType
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransformsType {
    #[serde(rename = "ds:Transform", alias = "Transform")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transform:
Vec<        TransformType
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransformType {
    #[serde(rename = "@Algorithm")]
    pub algorithm:
        String
,
    #[serde(rename = "XPath")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x_path:
Option<        String
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DigestMethodType {
    #[serde(rename = "@Algorithm")]
    pub algorithm:
        String
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
#[serde(rename = "ds:KeyInfo")]
pub struct KeyInfoType {
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "ds:KeyName", alias = "KeyName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_name:
Option<        String
>,
    #[serde(rename = "ds:KeyValue", alias = "KeyValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_value:
Option<        KeyValueType
>,
    #[serde(rename = "ds:RetrievalMethod", alias = "RetrievalMethod")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_method:
Option<        RetrievalMethodType
>,
    #[serde(rename = "ds:X509Data", alias = "X509Data")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub x509_data:
Option<        X509DataType
>,
    #[serde(rename = "ds:PGPData", alias = "PGPData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pgp_data:
Option<        PGPDataType
>,
    #[serde(rename = "ds:SPKIData", alias = "SPKIData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spki_data:
Option<        SPKIDataType
>,
    #[serde(rename = "ds:MgmtData", alias = "MgmtData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mgmt_data:
Option<        String
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct KeyValueType {
    #[serde(rename = "ds:DSAKeyValue", alias = "DSAKeyValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dsa_key_value:
Option<        DSAKeyValueType
>,
    #[serde(rename = "ds:RSAKeyValue", alias = "RSAKeyValue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsa_key_value:
Option<        RSAKeyValueType
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RetrievalMethodType {
    #[serde(rename = "@URI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri:
Option<        String
>,
    #[serde(rename = "@Type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type:
Option<        String
>,
    #[serde(rename = "ds:Transforms", alias = "Transforms")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transforms:
Option<        TransformsType
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct X509DataType {
    #[serde(rename = "X509IssuerSerial")]
    pub x509_issuer_serial:
        X509IssuerSerialType
,
    #[serde(rename = "X509SKI")]
    pub x509_ski:
        String
,
    #[serde(rename = "X509SubjectName")]
    pub x509_subject_name:
        String
,
    #[serde(rename = "X509Certificate")]
    pub x509_certificate:
        String
,
    #[serde(rename = "X509CRL")]
    pub x509_crl:
        String
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct X509IssuerSerialType {
    #[serde(rename = "X509IssuerName")]
    pub x509_issuer_name:
        String
,
    #[serde(rename = "X509SerialNumber")]
    pub x509_serial_number:
        i64
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PGPDataType {
    #[serde(rename = "PGPKeyID")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pgp_key_id:
Option<        String
>,
    #[serde(rename = "PGPKeyPacket")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pgp_key_packet:
Option<        String
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SPKIDataType {
    #[serde(rename = "SPKISexp")]
    pub spki_sexp:
        String
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ObjectType {
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "@MimeType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mime_type:
Option<        String
>,
    #[serde(rename = "@Encoding")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding:
Option<        String
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ManifestType {
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "ds:Reference", alias = "Reference")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference:
Vec<        ReferenceType
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SignaturePropertiesType {
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
    #[serde(rename = "ds:SignatureProperty", alias = "SignatureProperty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature_property:
Vec<        SignaturePropertyType
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SignaturePropertyType {
    #[serde(rename = "@Target")]
    pub target:
        String
,
    #[serde(rename = "@Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        String
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DSAKeyValueType {
    #[serde(rename = "P")]
    pub p:
        CryptoBinary
,
    #[serde(rename = "Q")]
    pub q:
        CryptoBinary
,
    #[serde(rename = "G")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub g:
Option<        CryptoBinary
>,
    #[serde(rename = "Y")]
    pub y:
        CryptoBinary
,
    #[serde(rename = "J")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub j:
Option<        CryptoBinary
>,
    #[serde(rename = "Seed")]
    pub seed:
        CryptoBinary
,
    #[serde(rename = "PgenCounter")]
    pub pgen_counter:
        CryptoBinary
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RSAKeyValueType {
    #[serde(rename = "Modulus")]
    pub modulus:
        CryptoBinary
,
    #[serde(rename = "Exponent")]
    pub exponent:
        CryptoBinary
,
}
