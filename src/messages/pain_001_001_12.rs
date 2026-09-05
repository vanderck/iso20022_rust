// Generated from pain.001.001.12.xsd
#![allow(unused_imports, non_snake_case, non_camel_case_types)]
use serde::{Serialize, Deserialize};
use validator::Validate;
pub type ActiveOrHistoricCurrencyAndAmountSimpleType = ::rust_decimal::Decimal;
pub type ActiveOrHistoricCurrencyCode = String;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum AdviceType1Code {
    #[serde(rename = "ADWD")]
    Adwd,
    #[serde(rename = "ADND")]
    Adnd,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type AnyBICDec2014Identifier = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum Authorisation1Code {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "FDET")]
    Fdet,
    #[serde(rename = "FSUM")]
    Fsum,
    #[serde(rename = "ILEV")]
    Ilev,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type BICFIDec2014Identifier = String;
pub type BaseOneRate = ::rust_decimal::Decimal;
pub type BatchBookingIndicator = bool;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum ChargeBearerType1Code {
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "SHAR")]
    Shar,
    #[serde(rename = "SLEV")]
    Slev,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum ChequeDelivery1Code {
    #[serde(rename = "MLDB")]
    Mldb,
    #[serde(rename = "MLCD")]
    Mlcd,
    #[serde(rename = "MLFA")]
    Mlfa,
    #[serde(rename = "CRDB")]
    Crdb,
    #[serde(rename = "CRCD")]
    Crcd,
    #[serde(rename = "CRFA")]
    Crfa,
    #[serde(rename = "PUDB")]
    Pudb,
    #[serde(rename = "PUCD")]
    Pucd,
    #[serde(rename = "PUFA")]
    Pufa,
    #[serde(rename = "RGDB")]
    Rgdb,
    #[serde(rename = "RGCD")]
    Rgcd,
    #[serde(rename = "RGFA")]
    Rgfa,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum ChequeType2Code {
    #[serde(rename = "CCHQ")]
    Cchq,
    #[serde(rename = "CCCH")]
    Ccch,
    #[serde(rename = "BCHQ")]
    Bchq,
    #[serde(rename = "DRFT")]
    Drft,
    #[serde(rename = "ELDR")]
    Eldr,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type CountryCode = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type DecimalNumber = ::rust_decimal::Decimal;
pub type Exact2NumericText = String;
pub type Exact4AlphaNumericText = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum ExchangeRateType1Code {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SALE")]
    Sale,
    #[serde(rename = "AGRD")]
    Agrd,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type ExternalAccountIdentification1Code = String;
pub type ExternalCashAccountType1Code = String;
pub type ExternalCategoryPurpose1Code = String;
pub type ExternalClearingSystemIdentification1Code = String;
pub type ExternalCreditorAgentInstruction1Code = String;
pub type ExternalCreditorReferenceType1Code = String;
pub type ExternalDateType1Code = String;
pub type ExternalDebtorAgentInstruction1Code = String;
pub type ExternalDocumentAmountType1Code = String;
pub type ExternalDocumentLineType1Code = String;
pub type ExternalDocumentType1Code = String;
pub type ExternalFinancialInstitutionIdentification1Code = String;
pub type ExternalGarnishmentType1Code = String;
pub type ExternalLocalInstrument1Code = String;
pub type ExternalMandateSetupReason1Code = String;
pub type ExternalOrganisationIdentification1Code = String;
pub type ExternalPersonIdentification1Code = String;
pub type ExternalProxyAccountType1Code = String;
pub type ExternalPurpose1Code = String;
pub type ExternalServiceLevel1Code = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum Frequency6Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "FRTN")]
    Frtn,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type IBAN2007Identifier = String;
pub type ISODate = ::chrono::NaiveDate;
pub type ISODateTime = ::chrono::DateTime<::chrono::Utc>;
pub type ISOYear = String;
pub type LEIIdentifier = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum MandateClassification1Code {
    #[serde(rename = "FIXE")]
    Fixe,
    #[serde(rename = "USGB")]
    Usgb,
    #[serde(rename = "VARI")]
    Vari,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type Max10KBinary = String;
pub type Max10Text = String;
pub type Max128Text = String;
pub type Max140Text = String;
pub type Max15NumericText = String;
pub type Max16Text = String;
pub type Max2048Text = String;
pub type Max256Text = String;
pub type Max34Text = String;
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
pub type Number = ::rust_decimal::Decimal;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum PaymentMethod3Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "TRA")]
    Tra,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type PercentageRate = ::rust_decimal::Decimal;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum Priority2Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum RegulatoryReportingType1Code {
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "BOTH")]
    Both,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum RemittanceLocationMethod2Code {
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "EDIC")]
    Edic,
    #[serde(rename = "URID")]
    Urid,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "SMSM")]
    Smsm,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum TaxRecordPeriod1Code {
    #[serde(rename = "MM01")]
    Mm01,
    #[serde(rename = "MM02")]
    Mm02,
    #[serde(rename = "MM03")]
    Mm03,
    #[serde(rename = "MM04")]
    Mm04,
    #[serde(rename = "MM05")]
    Mm05,
    #[serde(rename = "MM06")]
    Mm06,
    #[serde(rename = "MM07")]
    Mm07,
    #[serde(rename = "MM08")]
    Mm08,
    #[serde(rename = "MM09")]
    Mm09,
    #[serde(rename = "MM10")]
    Mm10,
    #[serde(rename = "MM11")]
    Mm11,
    #[serde(rename = "MM12")]
    Mm12,
    #[serde(rename = "QTR1")]
    Qtr1,
    #[serde(rename = "QTR2")]
    Qtr2,
    #[serde(rename = "QTR3")]
    Qtr3,
    #[serde(rename = "QTR4")]
    Qtr4,
    #[serde(rename = "HLF1")]
    Hlf1,
    #[serde(rename = "HLF2")]
    Hlf2,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type TrueFalseIndicator = bool;
pub type UUIDv4Identifier = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AccountIdentification4Choice {
    #[serde(rename = "IBAN")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban:
Option<        IBAN2007Identifier
>,
    #[serde(rename = "Othr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub othr:
Option<        GenericAccountIdentification1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AccountSchemeName1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalAccountIdentification1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy:
        ActiveOrHistoricCurrencyCode
,
    /// Content
    #[serde(rename = "$value")]
    pub value:
        ActiveOrHistoricCurrencyAndAmountSimpleType
,
}
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
pub struct AdviceType1 {
    #[serde(rename = "CdtAdvc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_advc:
Option<        AdviceType1Choice
>,
    #[serde(rename = "DbtAdvc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbt_advc:
Option<        AdviceType1Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AdviceType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        AdviceType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AmountType4Choice {
    #[serde(rename = "InstdAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "EqvtAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eqvt_amt:
Option<        EquivalentAmount2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Authorisation1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        Authorisation1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max128Text
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
pub struct CashAccount40 {
    #[serde(rename = "Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        AccountIdentification4Choice
>,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        CashAccountType2Choice
>,
    #[serde(rename = "Ccy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccy:
Option<        ActiveOrHistoricCurrencyCode
>,
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max70Text
>,
    #[serde(rename = "Prxy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prxy:
Option<        ProxyAccountIdentification1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CashAccountType2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCashAccountType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CategoryPurpose1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCategoryPurpose1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Cheque19 {
    #[serde(rename = "ChqTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chq_tp:
Option<        ChequeType2Code
>,
    #[serde(rename = "ChqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chq_nb:
Option<        Max35Text
>,
    #[serde(rename = "ChqFr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chq_fr:
Option<        NameAndAddress18
>,
    #[serde(rename = "DlvryMtd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dlvry_mtd:
Option<        ChequeDeliveryMethod1Choice
>,
    #[serde(rename = "DlvrTo")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dlvr_to:
Option<        NameAndAddress18
>,
    #[serde(rename = "InstrPrty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_prty:
Option<        Priority2Code
>,
    #[serde(rename = "ChqMtrtyDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chq_mtrty_dt:
Option<        ISODate
>,
    #[serde(rename = "FrmsCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frms_cd:
Option<        Max35Text
>,
    #[serde(rename = "MemoFld")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub memo_fld:
Vec<        Max35Text
>,
    #[serde(rename = "RgnlClrZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rgnl_clr_zone:
Option<        Max35Text
>,
    #[serde(rename = "PrtLctn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prt_lctn:
Option<        Max35Text
>,
    #[serde(rename = "Sgntr")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sgntr:
Vec<        Max70Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ChequeDeliveryMethod1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ChequeDelivery1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
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
pub struct CreditTransferMandateData1 {
    #[serde(rename = "MndtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mndt_id:
Option<        Max35Text
>,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        MandateTypeInformation2
>,
    #[serde(rename = "DtOfSgntr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr:
Option<        ISODate
>,
    #[serde(rename = "DtOfVrfctn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_of_vrfctn:
Option<        ISODateTime
>,
    #[serde(rename = "ElctrncSgntr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr:
Option<        Max10KBinary
>,
    #[serde(rename = "FrstPmtDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt:
Option<        ISODate
>,
    #[serde(rename = "FnlPmtDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fnl_pmt_dt:
Option<        ISODate
>,
    #[serde(rename = "Frqcy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frqcy:
Option<        Frequency36Choice
>,
    #[serde(rename = "Rsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsn:
Option<        MandateSetupReason1Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CreditTransferTransaction61 {
    #[serde(rename = "PmtId")]
    pub pmt_id:
        PaymentIdentification6
,
    #[serde(rename = "PmtTpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf:
Option<        PaymentTypeInformation26
>,
    #[serde(rename = "Amt")]
    pub amt:
        AmountType4Choice
,
    #[serde(rename = "XchgRateInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xchg_rate_inf:
Option<        ExchangeRate1
>,
    #[serde(rename = "ChrgBr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrg_br:
Option<        ChargeBearerType1Code
>,
    #[serde(rename = "MndtRltdInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf:
Option<        CreditTransferMandateData1
>,
    #[serde(rename = "ChqInstr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chq_instr:
Option<        Cheque19
>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr:
Option<        PartyIdentification272
>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IntrmyAgt1Acct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1_acct:
Option<        CashAccount40
>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IntrmyAgt2Acct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2_acct:
Option<        CashAccount40
>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IntrmyAgt3Acct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3_acct:
Option<        CashAccount40
>,
    #[serde(rename = "CdtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "Cdtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr:
Option<        PartyIdentification272
>,
    #[serde(rename = "CdtrAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_acct:
Option<        CashAccount40
>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr:
Option<        PartyIdentification272
>,
    #[serde(rename = "InstrForCdtrAgt")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub instr_for_cdtr_agt:
Vec<        InstructionForCreditorAgent3
>,
    #[serde(rename = "InstrForDbtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt:
Option<        InstructionForDebtorAgent1
>,
    #[serde(rename = "Purp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purp:
Option<        Purpose2Choice
>,
    #[serde(rename = "RgltryRptg")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rgltry_rptg:
Vec<        RegulatoryReporting3
>,
    #[serde(rename = "Tax")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax:
Option<        TaxData1
>,
    #[serde(rename = "RltdRmtInf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rltd_rmt_inf:
Vec<        RemittanceLocation8
>,
    #[serde(rename = "RmtInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmt_inf:
Option<        RemittanceInformation22
>,
    #[serde(rename = "SplmtryData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data:
Vec<        SupplementaryData1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CreditorReferenceInformation3 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        CreditorReferenceType3
>,
    #[serde(rename = "Ref")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#ref:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CreditorReferenceType2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCreditorReferenceType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CreditorReferenceType3 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry:
        CreditorReferenceType2Choice
,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CustomerCreditTransferInitiationV12 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr:
        GroupHeader114
,
    #[serde(rename = "PmtInf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pmt_inf:
Vec<        PaymentInstruction44
>,
    #[serde(rename = "SplmtryData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data:
Vec<        SupplementaryData1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DateAndDateTime2Choice {
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        ISODate
>,
    #[serde(rename = "DtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_tm:
Option<        ISODateTime
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
pub struct DateAndType1 {
    #[serde(rename = "Tp")]
    pub tp:
        DateType2Choice
,
    #[serde(rename = "Dt")]
    pub dt:
        ISODate
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DatePeriod2 {
    #[serde(rename = "FrDt")]
    pub fr_dt:
        ISODate
,
    #[serde(rename = "ToDt")]
    pub to_dt:
        ISODate
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DateType2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalDateType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Document {
    /// XML Namespace
    #[serde(rename = "@xmlns")]
    pub xmlns:
        String
,
    #[serde(rename = "CstmrCdtTrfInitn")]
    pub cstmr_cdt_trf_initn:
        CustomerCreditTransferInitiationV12
,
}
impl Document {
    pub fn new(body: CustomerCreditTransferInitiationV12) -> Self {
        Self {
            xmlns: "urn:iso:std:iso:20022:tech:xsd:pain.001.001.12".to_string(),
            cstmr_cdt_trf_initn: body,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentAdjustment1 {
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind:
Option<        CreditDebitCode
>,
    #[serde(rename = "Rsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsn:
Option<        Max4Text
>,
    #[serde(rename = "AddtlInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_inf:
Option<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentAmount1 {
    #[serde(rename = "Tp")]
    pub tp:
        DocumentAmountType1Choice
,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentAmountType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalDocumentAmountType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        DocumentLineType1
>,
    #[serde(rename = "Nb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb:
Option<        Max35Text
>,
    #[serde(rename = "RltdDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_dt:
Option<        ISODate
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentLineInformation2 {
    #[serde(rename = "Id")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub id:
Vec<        DocumentLineIdentification1
>,
    #[serde(rename = "Desc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc:
Option<        Max2048Text
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        RemittanceAmount4
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentLineType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry:
        DocumentLineType1Choice
,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentLineType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalDocumentLineType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry:
        DocumentType2Choice
,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct DocumentType2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalDocumentType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct EquivalentAmount2 {
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CcyOfTrf")]
    pub ccy_of_trf:
        ActiveOrHistoricCurrencyCode
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ExchangeRate1 {
    #[serde(rename = "UnitCcy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_ccy:
Option<        ActiveOrHistoricCurrencyCode
>,
    #[serde(rename = "XchgRate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xchg_rate:
Option<        BaseOneRate
>,
    #[serde(rename = "RateTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_tp:
Option<        ExchangeRateType1Code
>,
    #[serde(rename = "CtrctId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctrct_id:
Option<        Max35Text
>,
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
pub struct Frequency36Choice {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        Frequency6Code
>,
    #[serde(rename = "Prd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prd:
Option<        FrequencyPeriod1
>,
    #[serde(rename = "PtInTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pt_in_tm:
Option<        FrequencyAndMoment1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct FrequencyAndMoment1 {
    #[serde(rename = "Tp")]
    pub tp:
        Frequency6Code
,
    #[serde(rename = "PtInTm")]
    pub pt_in_tm:
        Exact2NumericText
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct FrequencyPeriod1 {
    #[serde(rename = "Tp")]
    pub tp:
        Frequency6Code
,
    #[serde(rename = "CntPerPrd")]
    pub cnt_per_prd:
        DecimalNumber
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Garnishment4 {
    #[serde(rename = "Tp")]
    pub tp:
        GarnishmentType1
,
    #[serde(rename = "Grnshee")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grnshee:
Option<        PartyIdentification272
>,
    #[serde(rename = "GrnshmtAdmstr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr:
Option<        PartyIdentification272
>,
    #[serde(rename = "RefNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_nb:
Option<        Max140Text
>,
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        ISODate
>,
    #[serde(rename = "RmtdAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmtd_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "FmlyMdclInsrncInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fmly_mdcl_insrnc_ind:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "MplyeeTermntnInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mplyee_termntn_ind:
Option<        TrueFalseIndicator
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GarnishmentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry:
        GarnishmentType1Choice
,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GarnishmentType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalGarnishmentType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GenericAccountIdentification1 {
    #[serde(rename = "Id")]
    pub id:
        Max34Text
,
    #[serde(rename = "SchmeNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schme_nm:
Option<        AccountSchemeName1Choice
>,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
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
pub struct GroupHeader114 {
    #[serde(rename = "MsgId")]
    pub msg_id:
        Max35Text
,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm:
        ISODateTime
,
    #[serde(rename = "Authstn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub authstn:
Vec<        Authorisation1Choice
>,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs:
        Max15NumericText
,
    #[serde(rename = "CtrlSum")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctrl_sum:
Option<        DecimalNumber
>,
    #[serde(rename = "InitgPty")]
    pub initg_pty:
        PartyIdentification272
,
    #[serde(rename = "FwdgAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fwdg_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "InitnSrc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initn_src:
Option<        PaymentInitiationSource1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct InstructionForCreditorAgent3 {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCreditorAgentInstruction1Code
>,
    #[serde(rename = "InstrInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_inf:
Option<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct InstructionForDebtorAgent1 {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalDebtorAgentInstruction1Code
>,
    #[serde(rename = "InstrInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_inf:
Option<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct LocalInstrument2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalLocalInstrument1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct MandateClassification1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        MandateClassification1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct MandateSetupReason1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalMandateSetupReason1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max70Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct MandateTypeInformation2 {
    #[serde(rename = "SvcLvl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svc_lvl:
Option<        ServiceLevel8Choice
>,
    #[serde(rename = "LclInstrm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lcl_instrm:
Option<        LocalInstrument2Choice
>,
    #[serde(rename = "CtgyPurp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctgy_purp:
Option<        CategoryPurpose1Choice
>,
    #[serde(rename = "Clssfctn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clssfctn:
Option<        MandateClassification1Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct NameAndAddress18 {
    #[serde(rename = "Nm")]
    pub nm:
        Max140Text
,
    #[serde(rename = "Adr")]
    pub adr:
        PostalAddress27
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
pub struct PaymentIdentification6 {
    #[serde(rename = "InstrId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_id:
Option<        Max35Text
>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id:
        Max35Text
,
    #[serde(rename = "UETR")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uetr:
Option<        UUIDv4Identifier
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PaymentInitiationSource1 {
    #[serde(rename = "Nm")]
    pub nm:
        Max140Text
,
    #[serde(rename = "Prvdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prvdr:
Option<        Max35Text
>,
    #[serde(rename = "Vrsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vrsn:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PaymentInstruction44 {
    #[serde(rename = "PmtInfId")]
    pub pmt_inf_id:
        Max35Text
,
    #[serde(rename = "PmtMtd")]
    pub pmt_mtd:
        PaymentMethod3Code
,
    #[serde(rename = "ReqdAdvcTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reqd_advc_tp:
Option<        AdviceType1
>,
    #[serde(rename = "BtchBookg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub btch_bookg:
Option<        BatchBookingIndicator
>,
    #[serde(rename = "NbOfTxs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_of_txs:
Option<        Max15NumericText
>,
    #[serde(rename = "CtrlSum")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctrl_sum:
Option<        DecimalNumber
>,
    #[serde(rename = "PmtTpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf:
Option<        PaymentTypeInformation26
>,
    #[serde(rename = "ReqdExctnDt")]
    pub reqd_exctn_dt:
        DateAndDateTime2Choice
,
    #[serde(rename = "PoolgAdjstmntDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poolg_adjstmnt_dt:
Option<        ISODate
>,
    #[serde(rename = "Dbtr")]
    pub dbtr:
        PartyIdentification272
,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct:
        CashAccount40
,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt:
        BranchAndFinancialInstitutionIdentification8
,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "InstrForDbtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt:
Option<        Max140Text
>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr:
Option<        PartyIdentification272
>,
    #[serde(rename = "ChrgBr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrg_br:
Option<        ChargeBearerType1Code
>,
    #[serde(rename = "ChrgsAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrgs_acct:
Option<        CashAccount40
>,
    #[serde(rename = "ChrgsAcctAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrgs_acct_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "CdtTrfTxInf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cdt_trf_tx_inf:
Vec<        CreditTransferTransaction61
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PaymentTypeInformation26 {
    #[serde(rename = "InstrPrty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_prty:
Option<        Priority2Code
>,
    #[serde(rename = "SvcLvl")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub svc_lvl:
Vec<        ServiceLevel8Choice
>,
    #[serde(rename = "LclInstrm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lcl_instrm:
Option<        LocalInstrument2Choice
>,
    #[serde(rename = "CtgyPurp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctgy_purp:
Option<        CategoryPurpose1Choice
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
pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        ProxyAccountType1Choice
>,
    #[serde(rename = "Id")]
    pub id:
        Max2048Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProxyAccountType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalProxyAccountType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Purpose2Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalPurpose1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ReferredDocumentInformation8 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        DocumentType1
>,
    #[serde(rename = "Nb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb:
Option<        Max35Text
>,
    #[serde(rename = "RltdDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_dt:
Option<        DateAndType1
>,
    #[serde(rename = "LineDtls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line_dtls:
Vec<        DocumentLineInformation2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max140Text
>,
    #[serde(rename = "Ctry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctry:
Option<        CountryCode
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind:
Option<        RegulatoryReportingType1Code
>,
    #[serde(rename = "Authrty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authrty:
Option<        RegulatoryAuthority2
>,
    #[serde(rename = "Dtls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtls:
Vec<        StructuredRegulatoryReporting3
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RemittanceAmount4 {
    #[serde(rename = "RmtAmtAndTp")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rmt_amt_and_tp:
Vec<        DocumentAmount1
>,
    #[serde(rename = "AdjstmntAmtAndRsn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adjstmnt_amt_and_rsn:
Vec<        DocumentAdjustment1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RemittanceInformation22 {
    #[serde(rename = "Ustrd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ustrd:
Vec<        Max140Text
>,
    #[serde(rename = "Strd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub strd:
Vec<        StructuredRemittanceInformation18
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RemittanceLocation8 {
    #[serde(rename = "RmtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmt_id:
Option<        Max35Text
>,
    #[serde(rename = "RmtLctnDtls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rmt_lctn_dtls:
Vec<        RemittanceLocationData2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RemittanceLocationData2 {
    #[serde(rename = "Mtd")]
    pub mtd:
        RemittanceLocationMethod2Code
,
    #[serde(rename = "ElctrncAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elctrnc_adr:
Option<        Max2048Text
>,
    #[serde(rename = "PstlAdr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pstl_adr:
Option<        NameAndAddress18
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ServiceLevel8Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalServiceLevel1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct StructuredRegulatoryReporting3 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        Max35Text
>,
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        ISODate
>,
    #[serde(rename = "Ctry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctry:
Option<        CountryCode
>,
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        Max10Text
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Inf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inf:
Vec<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct StructuredRemittanceInformation18 {
    #[serde(rename = "RfrdDocInf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rfrd_doc_inf:
Vec<        ReferredDocumentInformation8
>,
    #[serde(rename = "RfrdDocAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt:
Option<        RemittanceAmount4
>,
    #[serde(rename = "CdtrRefInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf:
Option<        CreditorReferenceInformation3
>,
    #[serde(rename = "Invcr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invcr:
Option<        PartyIdentification272
>,
    #[serde(rename = "Invcee")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invcee:
Option<        PartyIdentification272
>,
    #[serde(rename = "TaxRmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_rmt:
Option<        TaxData1
>,
    #[serde(rename = "GrnshmtRmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt:
Option<        Garnishment4
>,
    #[serde(rename = "AddtlRmtInf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addtl_rmt_inf:
Vec<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SupplementaryData1 {
    #[serde(rename = "PlcAndNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plc_and_nm:
Option<        Max350Text
>,
    #[serde(rename = "Envlp")]
    pub envlp:
        SupplementaryDataEnvelope1
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SupplementaryDataEnvelope1 {
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxAmount3 {
    #[serde(rename = "Rate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate:
Option<        PercentageRate
>,
    #[serde(rename = "TaxblBaseAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxbl_base_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "TtlAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Dtls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dtls:
Vec<        TaxRecordDetails3
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxAuthorisation1 {
    #[serde(rename = "Titl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub titl:
Option<        Max35Text
>,
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxData1 {
    #[serde(rename = "Cdtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr:
Option<        TaxParty1
>,
    #[serde(rename = "Dbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr:
Option<        TaxParty2
>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr:
Option<        TaxParty2
>,
    #[serde(rename = "AdmstnZone")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admstn_zone:
Option<        Max35Text
>,
    #[serde(rename = "RefNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ref_nb:
Option<        Max140Text
>,
    #[serde(rename = "Mtd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtd:
Option<        Max35Text
>,
    #[serde(rename = "TtlTaxblBaseAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_taxbl_base_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "TtlTaxAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_tax_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        ISODate
>,
    #[serde(rename = "SeqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_nb:
Option<        Number
>,
    #[serde(rename = "Rcrd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rcrd:
Vec<        TaxRecord3
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxParty1 {
    #[serde(rename = "TaxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id:
Option<        Max35Text
>,
    #[serde(rename = "RegnId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regn_id:
Option<        Max35Text
>,
    #[serde(rename = "TaxTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_tp:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxParty2 {
    #[serde(rename = "TaxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_id:
Option<        Max35Text
>,
    #[serde(rename = "RegnId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regn_id:
Option<        Max35Text
>,
    #[serde(rename = "TaxTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_tp:
Option<        Max35Text
>,
    #[serde(rename = "Authstn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authstn:
Option<        TaxAuthorisation1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxPeriod3 {
    #[serde(rename = "Yr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yr:
Option<        ISOYear
>,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        TaxRecordPeriod1Code
>,
    #[serde(rename = "FrToDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_to_dt:
Option<        DatePeriod2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxRecord3 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        Max35Text
>,
    #[serde(rename = "Ctgy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctgy:
Option<        Max35Text
>,
    #[serde(rename = "CtgyDtls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctgy_dtls:
Option<        Max35Text
>,
    #[serde(rename = "DbtrSts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr_sts:
Option<        Max35Text
>,
    #[serde(rename = "CertId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert_id:
Option<        Max35Text
>,
    #[serde(rename = "FrmsCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frms_cd:
Option<        Max35Text
>,
    #[serde(rename = "Prd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prd:
Option<        TaxPeriod3
>,
    #[serde(rename = "TaxAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_amt:
Option<        TaxAmount3
>,
    #[serde(rename = "AddtlInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_inf:
Option<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TaxRecordDetails3 {
    #[serde(rename = "Prd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prd:
Option<        TaxPeriod3
>,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
}
