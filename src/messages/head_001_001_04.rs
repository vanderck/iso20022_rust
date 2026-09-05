// Generated from head.001.001.04.xsd
#![allow(unused_imports, non_snake_case, non_camel_case_types)]
use serde::{Serialize, Deserialize};
use validator::Validate;
// Imported from xmldsig_core_schema module
use crate::messages::dsig::SignatureType;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type AnyBICDec2014Identifier = String;
pub type BICFIDec2014Identifier = String;
pub type BusinessMessagePriorityCode = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum CopyDuplicate1Code {
    #[serde(rename = "CODU")]
    Codu,
    #[serde(rename = "COPY")]
    Copy,
    #[serde(rename = "DUPL")]
    Dupl,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type CountryCode = String;
pub type Exact4AlphaNumericText = String;
pub type ExternalClearingSystemIdentification1Code = String;
pub type ExternalFinancialInstitutionIdentification1Code = String;
pub type ExternalOrganisationIdentification1Code = String;
pub type ExternalPersonIdentification1Code = String;
pub type ISODate = ::chrono::NaiveDate;
pub type ISODateTime = ::chrono::DateTime<::chrono::Utc>;
pub type LEIIdentifier = String;
pub type Max128Text = String;
pub type Max140Text = String;
pub type Max16Text = String;
pub type Max2048Text = String;
pub type Max256Text = String;
pub type Max350Text = String;
pub type Max35Text = String;
pub type Max4Text = String;
pub type Max70Text = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type PhoneNumber = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum PreferredContactMethod2Code {
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "CELL")]
    Cell,
    #[serde(rename = "ONLI")]
    Onli,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type UnicodeChartsCode = String;
pub type YesNoIndicator = bool;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AddressType3Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        AddressType2Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        GenericIdentification30
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BranchAndFinancialInstitutionIdentification8 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id:
        FinancialInstitutionIdentification23
,
    #[serde(rename = "BrnchId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brnch_id:
Option<        BranchData5
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BranchData5 {
    #[serde(rename = "Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        Max35Text
>,
    #[serde(rename = "LEI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lei:
Option<        LEIIdentifier
>,
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max140Text
>,
    #[serde(rename = "PstlAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pstl_adr:
Option<        PostalAddress27
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
#[serde(rename = "AppHdr")]
pub struct BusinessApplicationHeader8 {
    /// AppHdr Namespace
    #[serde(rename = "@xmlns")]
    pub xmlns:
        String
,
    #[serde(rename = "CharSet")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_set:
Option<        UnicodeChartsCode
>,
    #[serde(rename = "Fr")]
    pub fr:
        Party51Choice
,
    #[serde(rename = "To")]
    pub to:
        Party51Choice
,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr:
        Max35Text
,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr:
        Max35Text
,
    #[serde(rename = "BizSvc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_svc:
Option<        Max35Text
>,
    #[serde(rename = "MktPrctc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mkt_prctc:
Option<        ImplementationSpecification1
>,
    #[serde(rename = "CreDt")]
    pub cre_dt:
        ISODateTime
,
    #[serde(rename = "BizPrcgDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_prcg_dt:
Option<        ISODateTime
>,
    #[serde(rename = "CpyDplct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpy_dplct:
Option<        CopyDuplicate1Code
>,
    #[serde(rename = "PssblDplct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct:
Option<        YesNoIndicator
>,
    #[serde(rename = "Prty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prty:
Option<        BusinessMessagePriorityCode
>,
    #[serde(rename = "Sgntr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sgntr:
Option<        SignatureEnvelope
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
#[serde(rename = "AppHdr")]
pub struct BusinessApplicationHeaderV04 {
    /// AppHdr Namespace
    #[serde(rename = "@xmlns")]
    pub xmlns:
        String
,
    #[serde(rename = "CharSet")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_set:
Option<        UnicodeChartsCode
>,
    #[serde(rename = "Fr")]
    pub fr:
        Party51Choice
,
    #[serde(rename = "To")]
    pub to:
        Party51Choice
,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr:
        Max35Text
,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr:
        Max35Text
,
    #[serde(rename = "BizSvc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_svc:
Option<        Max35Text
>,
    #[serde(rename = "MktPrctc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mkt_prctc:
Option<        ImplementationSpecification1
>,
    #[serde(rename = "CreDt")]
    pub cre_dt:
        ISODateTime
,
    #[serde(rename = "BizPrcgDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub biz_prcg_dt:
Option<        ISODateTime
>,
    #[serde(rename = "CpyDplct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpy_dplct:
Option<        CopyDuplicate1Code
>,
    #[serde(rename = "PssblDplct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct:
Option<        YesNoIndicator
>,
    #[serde(rename = "Prty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prty:
Option<        BusinessMessagePriorityCode
>,
    #[serde(rename = "Sgntr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sgntr:
Option<        SignatureEnvelope
>,
    #[serde(rename = "Rltd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rltd:
Vec<        BusinessApplicationHeader8
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ClearingSystemIdentification2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalClearingSystemIdentification1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clr_sys_id:
Option<        ClearingSystemIdentification2Choice
>,
    #[serde(rename = "MmbId")]
    pub mmb_id:
        Max35Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Contact13 {
    #[serde(rename = "NmPrfx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm_prfx:
Option<        NamePrefix2Code
>,
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max140Text
>,
    #[serde(rename = "PhneNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phne_nb:
Option<        PhoneNumber
>,
    #[serde(rename = "MobNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mob_nb:
Option<        PhoneNumber
>,
    #[serde(rename = "FaxNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fax_nb:
Option<        PhoneNumber
>,
    #[serde(rename = "URLAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url_adr:
Option<        Max2048Text
>,
    #[serde(rename = "EmailAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_adr:
Option<        Max256Text
>,
    #[serde(rename = "EmailPurp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_purp:
Option<        Max35Text
>,
    #[serde(rename = "JobTitl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_titl:
Option<        Max35Text
>,
    #[serde(rename = "Rspnsblty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rspnsblty:
Option<        Max35Text
>,
    #[serde(rename = "Dept")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dept:
Option<        Max70Text
>,
    #[serde(rename = "Othr")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub othr:
Vec<        OtherContact1
>,
    #[serde(rename = "PrefrdMtd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd:
Option<        PreferredContactMethod2Code
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DateAndPlaceOfBirth1 {
    #[serde(rename = "BirthDt")]
    pub birth_dt:
        ISODate
,
    #[serde(rename = "PrvcOfBirth")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth:
Option<        Max35Text
>,
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth:
        Max35Text
,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth:
        CountryCode
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct FinancialIdentificationSchemeName1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalFinancialInstitutionIdentification1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct FinancialInstitutionIdentification23 {
    #[serde(rename = "BICFI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bicfi:
Option<        BICFIDec2014Identifier
>,
    #[serde(rename = "ClrSysMmbId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id:
Option<        ClearingSystemMemberIdentification2
>,
    #[serde(rename = "LEI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lei:
Option<        LEIIdentifier
>,
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max140Text
>,
    #[serde(rename = "PstlAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pstl_adr:
Option<        PostalAddress27
>,
    #[serde(rename = "Othr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub othr:
Option<        GenericFinancialIdentification1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GenericFinancialIdentification1 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "SchmeNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schme_nm:
Option<        FinancialIdentificationSchemeName1Choice
>,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    pub id:
        Exact4AlphaNumericText
,
    #[serde(rename = "Issr")]
    pub issr:
        Max35Text
,
    #[serde(rename = "SchmeNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schme_nm:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GenericOrganisationIdentification3 {
    #[serde(rename = "Id")]
    pub id:
        Max256Text
,
    #[serde(rename = "SchmeNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schme_nm:
Option<        OrganisationIdentificationSchemeName1Choice
>,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GenericPersonIdentification2 {
    #[serde(rename = "Id")]
    pub id:
        Max256Text
,
    #[serde(rename = "SchmeNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schme_nm:
Option<        PersonIdentificationSchemeName1Choice
>,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ImplementationSpecification1 {
    #[serde(rename = "Regy")]
    pub regy:
        Max350Text
,
    #[serde(rename = "Id")]
    pub id:
        Max2048Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct OrganisationIdentification39 {
    #[serde(rename = "AnyBIC")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub any_bic:
Option<        AnyBICDec2014Identifier
>,
    #[serde(rename = "LEI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lei:
Option<        LEIIdentifier
>,
    #[serde(rename = "Othr")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub othr:
Vec<        GenericOrganisationIdentification3
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct OrganisationIdentificationSchemeName1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalOrganisationIdentification1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    pub chanl_tp:
        Max4Text
,
    #[serde(rename = "Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        Max128Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Party51Choice {
    #[serde(rename = "OrgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id:
Option<        PartyIdentification272
>,
    #[serde(rename = "FIId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fi_id:
Option<        BranchAndFinancialInstitutionIdentification8
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Party52Choice {
    #[serde(rename = "OrgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id:
Option<        OrganisationIdentification39
>,
    #[serde(rename = "PrvtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prvt_id:
Option<        PersonIdentification18
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PartyIdentification272 {
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max140Text
>,
    #[serde(rename = "PstlAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pstl_adr:
Option<        PostalAddress27
>,
    #[serde(rename = "Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        Party52Choice
>,
    #[serde(rename = "CtryOfRes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctry_of_res:
Option<        CountryCode
>,
    #[serde(rename = "CtctDtls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctct_dtls:
Option<        Contact13
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PersonIdentification18 {
    #[serde(rename = "DtAndPlcOfBirth")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth:
Option<        DateAndPlaceOfBirth1
>,
    #[serde(rename = "Othr")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub othr:
Vec<        GenericPersonIdentification2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PersonIdentificationSchemeName1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalPersonIdentification1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PostalAddress27 {
    #[serde(rename = "AdrTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adr_tp:
Option<        AddressType3Choice
>,
    #[serde(rename = "CareOf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub care_of:
Option<        Max140Text
>,
    #[serde(rename = "Dept")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dept:
Option<        Max70Text
>,
    #[serde(rename = "SubDept")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_dept:
Option<        Max70Text
>,
    #[serde(rename = "StrtNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strt_nm:
Option<        Max140Text
>,
    #[serde(rename = "BldgNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bldg_nb:
Option<        Max16Text
>,
    #[serde(rename = "BldgNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bldg_nm:
Option<        Max140Text
>,
    #[serde(rename = "Flr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flr:
Option<        Max70Text
>,
    #[serde(rename = "UnitNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_nb:
Option<        Max16Text
>,
    #[serde(rename = "PstBx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pst_bx:
Option<        Max16Text
>,
    #[serde(rename = "Room")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub room:
Option<        Max70Text
>,
    #[serde(rename = "PstCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pst_cd:
Option<        Max16Text
>,
    #[serde(rename = "TwnNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twn_nm:
Option<        Max140Text
>,
    #[serde(rename = "TwnLctnNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm:
Option<        Max140Text
>,
    #[serde(rename = "DstrctNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dstrct_nm:
Option<        Max140Text
>,
    #[serde(rename = "CtrySubDvsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn:
Option<        Max35Text
>,
    #[serde(rename = "Ctry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctry:
Option<        CountryCode
>,
    #[serde(rename = "AdrLine")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adr_line:
Vec<        Max70Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SignatureEnvelope {
    /// Digital Signature (XMLDSIG)
    #[serde(rename = "ds:Signature", alias = "Signature")]
    pub signature:
        SignatureType
,
}
