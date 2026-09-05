// Generated from camt.027.001.10.xsd
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
pub type AnyBICDec2014Identifier = String;
pub type BICFIDec2014Identifier = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum ClearingChannel2Code {
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "RTNS")]
    Rtns,
    #[serde(rename = "MPNS")]
    Mpns,
    #[serde(rename = "BOOK")]
    Book,
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
pub type ExternalAccountIdentification1Code = String;
pub type ExternalAgentInstruction1Code = String;
pub type ExternalCashAccountType1Code = String;
pub type ExternalCashClearingSystem1Code = String;
pub type ExternalCategoryPurpose1Code = String;
pub type ExternalClearingSystemIdentification1Code = String;
pub type ExternalCreditorReferenceType1Code = String;
pub type ExternalDateType1Code = String;
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
pub type Max1025Text = String;
pub type Max10KBinary = String;
pub type Max128Text = String;
pub type Max140Text = String;
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
pub enum PaymentMethod4Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "DD")]
    Dd,
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
pub enum SequenceType3Code {
    #[serde(rename = "FRST")]
    Frst,
    #[serde(rename = "RCUR")]
    Rcur,
    #[serde(rename = "FNAL")]
    Fnal,
    #[serde(rename = "OOFF")]
    Ooff,
    #[serde(rename = "RPRE")]
    Rpre,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum SettlementMethod1Code {
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "INGA")]
    Inga,
    #[serde(rename = "COVE")]
    Cove,
    #[serde(rename = "CLRG")]
    Clrg,
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
pub type YesNoIndicator = bool;
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
pub struct AmendmentInformationDetails15 {
    #[serde(rename = "OrgnlMndtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_mndt_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlCdtrSchmeId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_schme_id:
Option<        PartyIdentification272
>,
    #[serde(rename = "OrgnlCdtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "OrgnlCdtrAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "OrgnlDbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr:
Option<        PartyIdentification272
>,
    #[serde(rename = "OrgnlDbtrAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_acct:
Option<        CashAccount40
>,
    #[serde(rename = "OrgnlDbtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "OrgnlDbtrAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "OrgnlFnlColltnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_fnl_colltn_dt:
Option<        ISODate
>,
    #[serde(rename = "OrgnlFrqcy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_frqcy:
Option<        Frequency36Choice
>,
    #[serde(rename = "OrgnlRsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_rsn:
Option<        MandateSetupReason1Choice
>,
    #[serde(rename = "OrgnlTrckgDays")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_trckg_days:
Option<        Exact2NumericText
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
pub struct Case6 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "Cretr")]
    pub cretr:
        Party50Choice
,
    #[serde(rename = "ReopCaseIndctn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reop_case_indctn:
Option<        YesNoIndicator
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CaseAssignment6 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "Assgnr")]
    pub assgnr:
        Party50Choice
,
    #[serde(rename = "Assgne")]
    pub assgne:
        Party50Choice
,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm:
        ISODateTime
,
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
pub struct ClaimNonReceiptV10 {
    #[serde(rename = "Assgnmt")]
    pub assgnmt:
        CaseAssignment6
,
    #[serde(rename = "Case")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case:
Option<        Case6
>,
    #[serde(rename = "Undrlyg")]
    pub undrlyg:
        UnderlyingTransaction8Choice
,
    #[serde(rename = "CoverDtls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_dtls:
Option<        MissingCover6
>,
    #[serde(rename = "InstrForAssgne")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_for_assgne:
Option<        InstructionForAssignee1
>,
    #[serde(rename = "SplmtryData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data:
Vec<        SupplementaryData1
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
pub struct ClearingSystemIdentification3Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCashClearingSystem1Code
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
    #[serde(rename = "ClmNonRct")]
    pub clm_non_rct:
        ClaimNonReceiptV10
,
}
impl Document {
    pub fn new(body: ClaimNonReceiptV10) -> Self {
        Self {
            xmlns: "urn:iso:std:iso:20022:tech:xsd:camt.027.001.10".to_string(),
            clm_non_rct: body,
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
pub struct InstructionForAssignee1 {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalAgentInstruction1Code
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
pub struct MandateRelatedData3Choice {
    #[serde(rename = "DrctDbtMndt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drct_dbt_mndt:
Option<        MandateRelatedInformation16
>,
    #[serde(rename = "CdtTrfMndt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_trf_mndt:
Option<        CreditTransferMandateData1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct MandateRelatedInformation16 {
    #[serde(rename = "MndtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mndt_id:
Option<        Max35Text
>,
    #[serde(rename = "DtOfSgntr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr:
Option<        ISODate
>,
    #[serde(rename = "AmdmntInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "AmdmntInfDtls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amdmnt_inf_dtls:
Option<        AmendmentInformationDetails15
>,
    #[serde(rename = "ElctrncSgntr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr:
Option<        Max1025Text
>,
    #[serde(rename = "FrstColltnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_colltn_dt:
Option<        ISODate
>,
    #[serde(rename = "FnlColltnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fnl_colltn_dt:
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
    #[serde(rename = "TrckgDays")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trckg_days:
Option<        Exact2NumericText
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
pub struct MissingCover6 {
    #[serde(rename = "MssngCoverInd")]
    pub mssng_cover_ind:
        YesNoIndicator
,
    #[serde(rename = "CoverCrrctn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_crrctn:
Option<        SettlementInstruction16
>,
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
pub struct OriginalGroupInformation29 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id:
        Max35Text
,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id:
        Max35Text
,
    #[serde(rename = "OrgnlCreDtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm:
Option<        ISODateTime
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct OriginalTransactionReference42 {
    #[serde(rename = "IntrBkSttlmAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        AmountType4Choice
>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt:
Option<        ISODate
>,
    #[serde(rename = "ReqdColltnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reqd_colltn_dt:
Option<        ISODate
>,
    #[serde(rename = "ReqdExctnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt:
Option<        DateAndDateTime2Choice
>,
    #[serde(rename = "CdtrSchmeId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_schme_id:
Option<        PartyIdentification272
>,
    #[serde(rename = "SttlmInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sttlm_inf:
Option<        SettlementInstruction15
>,
    #[serde(rename = "PmtTpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf:
Option<        PaymentTypeInformation27
>,
    #[serde(rename = "PmtMtd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_mtd:
Option<        PaymentMethod4Code
>,
    #[serde(rename = "MndtRltdInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf:
Option<        MandateRelatedData3Choice
>,
    #[serde(rename = "RmtInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rmt_inf:
Option<        RemittanceInformation22
>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr:
Option<        Party50Choice
>,
    #[serde(rename = "Dbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr:
Option<        Party50Choice
>,
    #[serde(rename = "DbtrAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr_acct:
Option<        CashAccount40
>,
    #[serde(rename = "DbtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct:
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
Option<        Party50Choice
>,
    #[serde(rename = "CdtrAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_acct:
Option<        CashAccount40
>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr:
Option<        Party50Choice
>,
    #[serde(rename = "Purp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purp:
Option<        Purpose2Choice
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
pub struct Party50Choice {
    #[serde(rename = "Pty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pty:
Option<        PartyIdentification272
>,
    #[serde(rename = "Agt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agt:
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
pub struct PaymentTypeInformation27 {
    #[serde(rename = "InstrPrty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_prty:
Option<        Priority2Code
>,
    #[serde(rename = "ClrChanl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clr_chanl:
Option<        ClearingChannel2Code
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
    #[serde(rename = "SeqTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_tp:
Option<        SequenceType3Code
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
pub struct SettlementInstruction15 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd:
        SettlementMethod1Code
,
    #[serde(rename = "SttlmAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sttlm_acct:
Option<        CashAccount40
>,
    #[serde(rename = "ClrSys")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clr_sys:
Option<        ClearingSystemIdentification3Choice
>,
    #[serde(rename = "InstgRmbrsmntAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "InstgRmbrsmntAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "InstdRmbrsmntAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "InstdRmbrsmntAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "ThrdRmbrsmntAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "ThrdRmbrsmntAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt_acct:
Option<        CashAccount40
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SettlementInstruction16 {
    #[serde(rename = "InstgRmbrsmntAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "InstgRmbrsmntAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt_acct:
Option<        CashAccount40
>,
    #[serde(rename = "InstdRmbrsmntAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "InstdRmbrsmntAgtAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt_acct:
Option<        CashAccount40
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct UnderlyingGroupInformation1 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id:
        Max35Text
,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id:
        Max35Text
,
    #[serde(rename = "OrgnlCreDtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm:
Option<        ISODateTime
>,
    #[serde(rename = "OrgnlMsgDlvryChanl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_msg_dlvry_chanl:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct UnderlyingPaymentInstruction9 {
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf:
Option<        UnderlyingGroupInformation1
>,
    #[serde(rename = "OrgnlPmtInfId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_pmt_inf_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlInstrId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlEndToEndId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlUETR")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr:
Option<        UUIDv4Identifier
>,
    #[serde(rename = "OrgnlInstdAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_instd_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "ReqdExctnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt:
Option<        DateAndDateTime2Choice
>,
    #[serde(rename = "ReqdColltnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reqd_colltn_dt:
Option<        ISODate
>,
    #[serde(rename = "OrgnlTxRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref:
Option<        OriginalTransactionReference42
>,
    #[serde(rename = "OrgnlSvcLvl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_svc_lvl:
Option<        ServiceLevel8Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct UnderlyingPaymentTransaction8 {
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf:
Option<        UnderlyingGroupInformation1
>,
    #[serde(rename = "OrgnlInstrId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlEndToEndId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlUETR")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr:
Option<        UUIDv4Identifier
>,
    #[serde(rename = "OrgnlIntrBkSttlmAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "OrgnlIntrBkSttlmDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_dt:
Option<        ISODate
>,
    #[serde(rename = "OrgnlTxRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref:
Option<        OriginalTransactionReference42
>,
    #[serde(rename = "OrgnlSvcLvl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_svc_lvl:
Option<        ServiceLevel8Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct UnderlyingStatementEntry3 {
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf:
Option<        OriginalGroupInformation29
>,
    #[serde(rename = "OrgnlStmtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_stmt_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlNtryId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_ntry_id:
Option<        Max35Text
>,
    #[serde(rename = "OrgnlUETR")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr:
Option<        UUIDv4Identifier
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct UnderlyingTransaction8Choice {
    #[serde(rename = "Initn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initn:
Option<        UnderlyingPaymentInstruction9
>,
    #[serde(rename = "IntrBk")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intr_bk:
Option<        UnderlyingPaymentTransaction8
>,
    #[serde(rename = "StmtNtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stmt_ntry:
Option<        UnderlyingStatementEntry3
>,
}
