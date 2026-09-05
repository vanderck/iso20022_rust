// Generated from camt.053.001.13.xsd
#![allow(unused_imports, non_snake_case, non_camel_case_types)]
use serde::{Serialize, Deserialize};
use validator::Validate;
pub type ActiveCurrencyAndAmountSimpleType = ::rust_decimal::Decimal;
pub type ActiveCurrencyCode = String;
pub type ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType = ::rust_decimal::Decimal;
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum AttendanceContext1Code {
    #[serde(rename = "ATTD")]
    Attd,
    #[serde(rename = "SATT")]
    Satt,
    #[serde(rename = "UATT")]
    Uatt,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum AuthenticationEntity1Code {
    #[serde(rename = "ICCD")]
    Iccd,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum AuthenticationMethod1Code {
    #[serde(rename = "UKNW")]
    Uknw,
    #[serde(rename = "BYPS")]
    Byps,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "FPIN")]
    Fpin,
    #[serde(rename = "CPSG")]
    Cpsg,
    #[serde(rename = "PPSG")]
    Ppsg,
    #[serde(rename = "MANU")]
    Manu,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SNCT")]
    Snct,
    #[serde(rename = "SCNL")]
    Scnl,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type BICFIDec2014Identifier = String;
pub type BaseOneRate = ::rust_decimal::Decimal;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum CSCManagement1Code {
    #[serde(rename = "PRST")]
    Prst,
    #[serde(rename = "BYPS")]
    Byps,
    #[serde(rename = "UNRD")]
    Unrd,
    #[serde(rename = "NCSC")]
    Ncsc,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum CardDataReading1Code {
    #[serde(rename = "TAGC")]
    Tagc,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "BRCD")]
    Brcd,
    #[serde(rename = "MGST")]
    Mgst,
    #[serde(rename = "CICC")]
    Cicc,
    #[serde(rename = "DFLE")]
    Dfle,
    #[serde(rename = "CTLS")]
    Ctls,
    #[serde(rename = "ECTL")]
    Ectl,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum CardPaymentServiceType2Code {
    #[serde(rename = "AGGR")]
    Aggr,
    #[serde(rename = "DCCV")]
    Dccv,
    #[serde(rename = "GRTT")]
    Grtt,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "LOYT")]
    Loyt,
    #[serde(rename = "NRES")]
    Nres,
    #[serde(rename = "PUCO")]
    Puco,
    #[serde(rename = "RECP")]
    Recp,
    #[serde(rename = "SOAF")]
    Soaf,
    #[serde(rename = "UNAF")]
    Unaf,
    #[serde(rename = "VCAU")]
    Vcau,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum CardholderVerificationCapability1Code {
    #[serde(rename = "MNSG")]
    Mnsg,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "FCPN")]
    Fcpn,
    #[serde(rename = "FEPN")]
    Fepn,
    #[serde(rename = "FDSG")]
    Fdsg,
    #[serde(rename = "FBIO")]
    Fbio,
    #[serde(rename = "MNVR")]
    Mnvr,
    #[serde(rename = "FBIG")]
    Fbig,
    #[serde(rename = "APKI")]
    Apki,
    #[serde(rename = "PKIS")]
    Pkis,
    #[serde(rename = "CHDT")]
    Chdt,
    #[serde(rename = "SCEC")]
    Scec,
    #[serde(other)]
    #[default]
    Unknown,
}
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
pub type ChargeIncludedIndicator = bool;
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
pub type Exact1NumericText = String;
pub type Exact3NumericText = String;
pub type Exact4AlphaNumericText = String;
pub type ExternalAccountIdentification1Code = String;
pub type ExternalBalanceSubType1Code = String;
pub type ExternalBalanceType1Code = String;
pub type ExternalBankTransactionDomain1Code = String;
pub type ExternalBankTransactionFamily1Code = String;
pub type ExternalBankTransactionSubFamily1Code = String;
pub type ExternalCardTransactionCategory1Code = String;
pub type ExternalCashAccountType1Code = String;
pub type ExternalCategoryPurpose1Code = String;
pub type ExternalChargeType1Code = String;
pub type ExternalClearingSystemIdentification1Code = String;
pub type ExternalCorporateActionEventType1Code = String;
pub type ExternalCreditLineType1Code = String;
pub type ExternalCreditorReferenceType1Code = String;
pub type ExternalDateType1Code = String;
pub type ExternalDocumentAmountType1Code = String;
pub type ExternalDocumentLineType1Code = String;
pub type ExternalDocumentType1Code = String;
pub type ExternalEntryStatus1Code = String;
pub type ExternalFinancialInstitutionIdentification1Code = String;
pub type ExternalFinancialInstrumentIdentificationType1Code = String;
pub type ExternalGarnishmentType1Code = String;
pub type ExternalLocalInstrument1Code = String;
pub type ExternalOrganisationIdentification1Code = String;
pub type ExternalPersonIdentification1Code = String;
pub type ExternalProxyAccountType1Code = String;
pub type ExternalPurpose1Code = String;
pub type ExternalRePresentmentReason1Code = String;
pub type ExternalReportingSource1Code = String;
pub type ExternalReturnReason1Code = String;
pub type ExternalServiceLevel1Code = String;
pub type ExternalTechnicalInputChannel1Code = String;
pub type IBAN2007Identifier = String;
pub type ISINOct2015Identifier = String;
pub type ISO2ALanguageCode = String;
pub type ISODate = ::chrono::NaiveDate;
pub type ISODateTime = ::chrono::DateTime<::chrono::Utc>;
pub type ISOYear = String;
pub type ISOYearMonth = String;
pub type ImpliedCurrencyAndAmount = ::rust_decimal::Decimal;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum InterestType1Code {
    #[serde(rename = "INDY")]
    Indy,
    #[serde(rename = "OVRN")]
    Ovrn,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type LEIIdentifier = String;
pub type Max1025Text = String;
pub type Max105Text = String;
pub type Max128Text = String;
pub type Max140Text = String;
pub type Max15NumericText = String;
pub type Max15PlusSignedNumericText = String;
pub type Max16Text = String;
pub type Max20000Text = String;
pub type Max2048Text = String;
pub type Max256Text = String;
pub type Max30DecimalNumber = ::rust_decimal::Decimal;
pub type Max34Text = String;
pub type Max350Text = String;
pub type Max35Text = String;
pub type Max3NumericText = String;
pub type Max4Text = String;
pub type Max500Text = String;
pub type Max52Text = String;
pub type Max5NumericText = String;
pub type Max70Text = String;
pub type Min2Max3NumericText = String;
pub type Min3Max4NumericText = String;
pub type Min8Max28NumericText = String;
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
pub type NonNegativeDecimalNumber = ::rust_decimal::Decimal;
pub type Number = ::rust_decimal::Decimal;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum OnLineCapability1Code {
    #[serde(rename = "OFLN")]
    Ofln,
    #[serde(rename = "ONLN")]
    Onln,
    #[serde(rename = "SMON")]
    Smon,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum POIComponentType1Code {
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "EMVK")]
    Emvk,
    #[serde(rename = "EMVO")]
    Emvo,
    #[serde(rename = "MRIT")]
    Mrit,
    #[serde(rename = "CHIT")]
    Chit,
    #[serde(rename = "SECM")]
    Secm,
    #[serde(rename = "PEDV")]
    Pedv,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum PartyType3Code {
    #[serde(rename = "OPOI")]
    Opoi,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum PartyType4Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "TAXH")]
    Taxh,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type PercentageRate = ::rust_decimal::Decimal;
pub type PhoneNumber = String;
pub type PositiveNumber = ::rust_decimal::Decimal;
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
pub enum PriceValueType1Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum TransactionChannel1Code {
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "TLPH")]
    Tlph,
    #[serde(rename = "ECOM")]
    Ecom,
    #[serde(rename = "TVPY")]
    Tvpy,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum TransactionEnvironment1Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "PRIV")]
    Priv,
    #[serde(rename = "PUBL")]
    Publ,
    #[serde(other)]
    #[default]
    Unknown,
}
pub type TrueFalseIndicator = bool;
pub type UUIDv4Identifier = String;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum UnitOfMeasure1Code {
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "ACRE")]
    Acre,
    #[serde(other)]
    #[default]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
#[derive(Default)]
pub enum UserInterface2Code {
    #[serde(rename = "MDSP")]
    Mdsp,
    #[serde(rename = "CDSP")]
    Cdsp,
    #[serde(other)]
    #[default]
    Unknown,
}
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
pub struct AccountInterest4 {
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        InterestType1Choice
>,
    #[serde(rename = "Rate")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rate:
Vec<        Rate4
>,
    #[serde(rename = "FrToDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_to_dt:
Option<        DateTimePeriod1
>,
    #[serde(rename = "Rsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsn:
Option<        Max35Text
>,
    #[serde(rename = "Tax")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax:
Option<        TaxCharges2
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
pub struct AccountStatement14 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "StmtPgntn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stmt_pgntn:
Option<        Pagination1
>,
    #[serde(rename = "ElctrncSeqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elctrnc_seq_nb:
Option<        Number
>,
    #[serde(rename = "RptgSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rptg_seq:
Option<        SequenceRange1Choice
>,
    #[serde(rename = "LglSeqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lgl_seq_nb:
Option<        Number
>,
    #[serde(rename = "CreDtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm:
Option<        ISODateTime
>,
    #[serde(rename = "FrToDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_to_dt:
Option<        DateTimePeriod1
>,
    #[serde(rename = "CpyDplctInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpy_dplct_ind:
Option<        CopyDuplicate1Code
>,
    #[serde(rename = "RptgSrc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rptg_src:
Option<        ReportingSource1Choice
>,
    #[serde(rename = "Acct")]
    pub acct:
        CashAccount43
,
    #[serde(rename = "RltdAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_acct:
Option<        CashAccount40
>,
    #[serde(rename = "Intrst")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub intrst:
Vec<        AccountInterest4
>,
    #[serde(rename = "Bal")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bal:
Vec<        CashBalance8
>,
    #[serde(rename = "TxsSummry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub txs_summry:
Option<        TotalTransactions6
>,
    #[serde(rename = "Ntry")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ntry:
Vec<        ReportEntry15
>,
    #[serde(rename = "AddtlStmtInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_stmt_inf:
Option<        Max500Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "@Ccy")]
    pub ccy:
        ActiveCurrencyCode
,
    /// Content
    #[serde(rename = "$value")]
    pub value:
        ActiveCurrencyAndAmountSimpleType
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "@Ccy")]
    pub ccy:
        ActiveOrHistoricCurrencyCode
,
    /// Content
    #[serde(rename = "$value")]
    pub value:
        ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType
,
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
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
    #[serde(rename = "Amt")]
    pub amt:
        ImpliedCurrencyAmountRange1Choice
,
    #[serde(rename = "CdtDbtInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind:
Option<        CreditDebitCode
>,
    #[serde(rename = "Ccy")]
    pub ccy:
        ActiveOrHistoricCurrencyCode
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
pub struct AmountAndCurrencyExchange4 {
    #[serde(rename = "InstdAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_amt:
Option<        AmountAndCurrencyExchangeDetails5
>,
    #[serde(rename = "TxAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_amt:
Option<        AmountAndCurrencyExchangeDetails5
>,
    #[serde(rename = "CntrValAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cntr_val_amt:
Option<        AmountAndCurrencyExchangeDetails5
>,
    #[serde(rename = "AnncdPstngAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anncd_pstng_amt:
Option<        AmountAndCurrencyExchangeDetails5
>,
    #[serde(rename = "PrtryAmt")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prtry_amt:
Vec<        AmountAndCurrencyExchangeDetails6
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AmountAndCurrencyExchangeDetails5 {
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CcyXchg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccy_xchg:
Option<        CurrencyExchange24
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AmountAndCurrencyExchangeDetails6 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CcyXchg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ccy_xchg:
Option<        CurrencyExchange24
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AmountAndDirection35 {
    #[serde(rename = "Amt")]
    pub amt:
        NonNegativeDecimalNumber
,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind:
        CreditDebitCode
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct AmountRangeBoundary1 {
    #[serde(rename = "BdryAmt")]
    pub bdry_amt:
        ImpliedCurrencyAndAmount
,
    #[serde(rename = "Incl")]
    pub incl:
        YesNoIndicator
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BalanceSubType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalBalanceSubType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BalanceType10Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalBalanceType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BalanceType13 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry:
        BalanceType10Choice
,
    #[serde(rename = "SubTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_tp:
Option<        BalanceSubType1Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BankToCustomerStatementV13 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr:
        GroupHeader116
,
    #[serde(rename = "Stmt")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stmt:
Vec<        AccountStatement14
>,
    #[serde(rename = "SplmtryData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data:
Vec<        SupplementaryData1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BankTransactionCodeStructure4 {
    #[serde(rename = "Domn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domn:
Option<        BankTransactionCodeStructure5
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        ProprietaryBankTransactionCodeStructure1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BankTransactionCodeStructure5 {
    #[serde(rename = "Cd")]
    pub cd:
        ExternalBankTransactionDomain1Code
,
    #[serde(rename = "Fmly")]
    pub fmly:
        BankTransactionCodeStructure6
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BankTransactionCodeStructure6 {
    #[serde(rename = "Cd")]
    pub cd:
        ExternalBankTransactionFamily1Code
,
    #[serde(rename = "SubFmlyCd")]
    pub sub_fmly_cd:
        ExternalBankTransactionSubFamily1Code
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct BatchInformation2 {
    #[serde(rename = "MsgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_id:
Option<        Max35Text
>,
    #[serde(rename = "PmtInfId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id:
Option<        Max35Text
>,
    #[serde(rename = "NbOfTxs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_of_txs:
Option<        Max15NumericText
>,
    #[serde(rename = "TtlAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "CdtDbtInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind:
Option<        CreditDebitCode
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
pub struct CardAggregated2 {
    #[serde(rename = "AddtlSvc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_svc:
Option<        CardPaymentServiceType2Code
>,
    #[serde(rename = "TxCtgy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_ctgy:
Option<        ExternalCardTransactionCategory1Code
>,
    #[serde(rename = "SaleRcncltnId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id:
Option<        Max35Text
>,
    #[serde(rename = "SeqNbRg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_nb_rg:
Option<        CardSequenceNumberRange1
>,
    #[serde(rename = "TxDtRg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_dt_rg:
Option<        DateOrDateTimePeriod1Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardEntry5 {
    #[serde(rename = "Card")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card:
Option<        PaymentCard4
>,
    #[serde(rename = "POI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poi:
Option<        PointOfInteraction1
>,
    #[serde(rename = "AggtdNtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggtd_ntry:
Option<        CardAggregated2
>,
    #[serde(rename = "PrePdAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct:
Option<        CashAccount40
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardIndividualTransaction2 {
    #[serde(rename = "ICCRltdData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data:
Option<        Max1025Text
>,
    #[serde(rename = "PmtCntxt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_cntxt:
Option<        PaymentContext3
>,
    #[serde(rename = "AddtlSvc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_svc:
Option<        CardPaymentServiceType2Code
>,
    #[serde(rename = "TxCtgy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_ctgy:
Option<        ExternalCardTransactionCategory1Code
>,
    #[serde(rename = "SaleRcncltnId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id:
Option<        Max35Text
>,
    #[serde(rename = "SaleRefNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sale_ref_nb:
Option<        Max35Text
>,
    #[serde(rename = "RePresntmntRsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub re_presntmnt_rsn:
Option<        ExternalRePresentmentReason1Code
>,
    #[serde(rename = "SeqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_nb:
Option<        Max35Text
>,
    #[serde(rename = "TxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id:
Option<        TransactionIdentifier1
>,
    #[serde(rename = "Pdct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdct:
Option<        Product2
>,
    #[serde(rename = "VldtnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vldtn_dt:
Option<        ISODate
>,
    #[serde(rename = "VldtnSeqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vldtn_seq_nb:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardSecurityInformation1 {
    #[serde(rename = "CSCMgmt")]
    pub csc_mgmt:
        CSCManagement1Code
,
    #[serde(rename = "CSCVal")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csc_val:
Option<        Min3Max4NumericText
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardSequenceNumberRange1 {
    #[serde(rename = "FrstTx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frst_tx:
Option<        Max35Text
>,
    #[serde(rename = "LastTx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_tx:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardTransaction18 {
    #[serde(rename = "Card")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card:
Option<        PaymentCard4
>,
    #[serde(rename = "POI")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poi:
Option<        PointOfInteraction1
>,
    #[serde(rename = "Tx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx:
Option<        CardTransaction3Choice
>,
    #[serde(rename = "PrePdAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct:
Option<        CashAccount40
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardTransaction3Choice {
    #[serde(rename = "Aggtd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggtd:
Option<        CardAggregated2
>,
    #[serde(rename = "Indv")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indv:
Option<        CardIndividualTransaction2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CardholderAuthentication2 {
    #[serde(rename = "AuthntcnMtd")]
    pub authntcn_mtd:
        AuthenticationMethod1Code
,
    #[serde(rename = "AuthntcnNtty")]
    pub authntcn_ntty:
        AuthenticationEntity1Code
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
pub struct CashAccount43 {
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
    #[serde(rename = "Ownr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ownr:
Option<        PartyIdentification272
>,
    #[serde(rename = "Svcr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svcr:
Option<        BranchAndFinancialInstitutionIdentification8
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
pub struct CashAvailability1 {
    #[serde(rename = "Dt")]
    pub dt:
        CashAvailabilityDate1Choice
,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind:
        CreditDebitCode
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CashAvailabilityDate1Choice {
    #[serde(rename = "NbOfDays")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_of_days:
Option<        Max15PlusSignedNumericText
>,
    #[serde(rename = "ActlDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actl_dt:
Option<        ISODate
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CashBalance8 {
    #[serde(rename = "Tp")]
    pub tp:
        BalanceType13
,
    #[serde(rename = "CdtLine")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cdt_line:
Vec<        CreditLine3
>,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind:
        CreditDebitCode
,
    #[serde(rename = "Dt")]
    pub dt:
        DateAndDateTime2Choice
,
    #[serde(rename = "Avlbty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub avlbty:
Vec<        CashAvailability1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CashDeposit1 {
    #[serde(rename = "NoteDnmtn")]
    pub note_dnmtn:
        ActiveCurrencyAndAmount
,
    #[serde(rename = "NbOfNotes")]
    pub nb_of_notes:
        Max15NumericText
,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveCurrencyAndAmount
,
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
pub struct ChargeType3Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalChargeType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        GenericIdentification3
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Charges15 {
    #[serde(rename = "TtlChrgsAndTaxAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_chrgs_and_tax_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Rcrd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rcrd:
Vec<        ChargesRecord8
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ChargesRecord8 {
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind:
Option<        CreditDebitCode
>,
    #[serde(rename = "ChrgInclInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrg_incl_ind:
Option<        ChargeIncludedIndicator
>,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        ChargeType3Choice
>,
    #[serde(rename = "Rate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate:
Option<        PercentageRate
>,
    #[serde(rename = "Br")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub br:
Option<        ChargeBearerType1Code
>,
    #[serde(rename = "Agt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "Tax")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax:
Option<        TaxCharges2
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
pub struct CorporateAction82 {
    #[serde(rename = "EvtTp")]
    pub evt_tp:
        CorporateActionEventType104Choice
,
    #[serde(rename = "CorpActnEvtId")]
    pub corp_actn_evt_id:
        Max35Text
,
    #[serde(rename = "OffclCorpActnEvtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offcl_corp_actn_evt_id:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CorporateActionEventType104Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCorporateActionEventType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        GenericIdentification30
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CreditLine3 {
    #[serde(rename = "Incl")]
    pub incl:
        TrueFalseIndicator
,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        CreditLineType1Choice
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        DateAndDateTime2Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct CreditLineType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalCreditLineType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
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
pub struct CurrencyExchange24 {
    #[serde(rename = "SrcCcy")]
    pub src_ccy:
        ActiveOrHistoricCurrencyCode
,
    #[serde(rename = "TrgtCcy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trgt_ccy:
Option<        ActiveOrHistoricCurrencyCode
>,
    #[serde(rename = "UnitCcy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_ccy:
Option<        ActiveOrHistoricCurrencyCode
>,
    #[serde(rename = "XchgRate")]
    pub xchg_rate:
        BaseOneRate
,
    #[serde(rename = "CtrctId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctrct_id:
Option<        Max35Text
>,
    #[serde(rename = "QtnDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qtn_dt:
Option<        ISODateTime
>,
    #[serde(rename = "XchgRateBase")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub xchg_rate_base:
Option<        PositiveNumber
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
pub struct DateOrDateTimePeriod1Choice {
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        DatePeriod2
>,
    #[serde(rename = "DtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt_tm:
Option<        DateTimePeriod1
>,
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
pub struct DateTimePeriod1 {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm:
        ISODateTime
,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm:
        ISODateTime
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
pub struct DisplayCapabilities1 {
    #[serde(rename = "DispTp")]
    pub disp_tp:
        UserInterface2Code
,
    #[serde(rename = "NbOfLines")]
    pub nb_of_lines:
        Max3NumericText
,
    #[serde(rename = "LineWidth")]
    pub line_width:
        Max3NumericText
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Document {
    /// XML Namespace
    #[serde(rename = "@xmlns")]
    pub xmlns:
        String
,
    #[serde(rename = "BkToCstmrStmt")]
    pub bk_to_cstmr_stmt:
        BankToCustomerStatementV13
,
}
impl Document {
    pub fn new(body: BankToCustomerStatementV13) -> Self {
        Self {
            xmlns: "urn:iso:std:iso:20022:tech:xsd:camt.053.001.13".to_string(),
            bk_to_cstmr_stmt: body,
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
pub struct EntryDetails14 {
    #[serde(rename = "Btch")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub btch:
Option<        BatchInformation2
>,
    #[serde(rename = "TxDtls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tx_dtls:
Vec<        EntryTransaction15
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct EntryStatus1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalEntryStatus1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct EntryTransaction15 {
    #[serde(rename = "Refs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refs:
Option<        TransactionReferences6
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "CdtDbtInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind:
Option<        CreditDebitCode
>,
    #[serde(rename = "AmtDtls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt_dtls:
Option<        AmountAndCurrencyExchange4
>,
    #[serde(rename = "Avlbty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub avlbty:
Vec<        CashAvailability1
>,
    #[serde(rename = "BkTxCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bk_tx_cd:
Option<        BankTransactionCodeStructure4
>,
    #[serde(rename = "Chrgs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrgs:
Option<        Charges15
>,
    #[serde(rename = "Intrst")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrst:
Option<        TransactionInterest4
>,
    #[serde(rename = "RltdPties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_pties:
Option<        TransactionParties12
>,
    #[serde(rename = "RltdAgts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_agts:
Option<        TransactionAgents6
>,
    #[serde(rename = "LclInstrm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lcl_instrm:
Option<        LocalInstrument2Choice
>,
    #[serde(rename = "PmtTpInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf:
Option<        PaymentTypeInformation27
>,
    #[serde(rename = "Purp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purp:
Option<        Purpose2Choice
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
    #[serde(rename = "RltdDts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_dts:
Option<        TransactionDates3
>,
    #[serde(rename = "RltdPric")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_pric:
Option<        TransactionPrice4Choice
>,
    #[serde(rename = "RltdQties")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rltd_qties:
Vec<        TransactionQuantities4Choice
>,
    #[serde(rename = "FinInstrmId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id:
Option<        SecurityIdentification19
>,
    #[serde(rename = "Tax")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax:
Option<        TaxData1
>,
    #[serde(rename = "RtrInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtr_inf:
Option<        PaymentReturnReason8
>,
    #[serde(rename = "RltdCorpActn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rltd_corp_actn:
Option<        CorporateAction82
>,
    #[serde(rename = "SfkpgAcct")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct:
Option<        SecuritiesAccount19
>,
    #[serde(rename = "UndrlygAllcn")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub undrlyg_allcn:
Vec<        TransactionAllocation1
>,
    #[serde(rename = "CshDpst")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub csh_dpst:
Vec<        CashDeposit1
>,
    #[serde(rename = "CardTx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_tx:
Option<        CardTransaction18
>,
    #[serde(rename = "InstrCpy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_cpy:
Option<        Max20000Text
>,
    #[serde(rename = "AddtlTxInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_tx_inf:
Option<        Max500Text
>,
    #[serde(rename = "SplmtryData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data:
Vec<        SupplementaryData1
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
pub struct FinancialInstrumentQuantity33Choice {
    #[serde(rename = "Unit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit:
Option<        DecimalNumber
>,
    #[serde(rename = "FaceAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face_amt:
Option<        ImpliedCurrencyAndAmount
>,
    #[serde(rename = "AmtsdVal")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amtsd_val:
Option<        ImpliedCurrencyAndAmount
>,
    #[serde(rename = "DgtlTknUnit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dgtl_tkn_unit:
Option<        Max30DecimalNumber
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct FromToAmountRange1 {
    #[serde(rename = "FrAmt")]
    pub fr_amt:
        AmountRangeBoundary1
,
    #[serde(rename = "ToAmt")]
    pub to_amt:
        AmountRangeBoundary1
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
pub struct GenericIdentification1 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "SchmeNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schme_nm:
Option<        Max35Text
>,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct GenericIdentification3 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
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
pub struct GenericIdentification32 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        PartyType3Code
>,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        PartyType4Code
>,
    #[serde(rename = "ShrtNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shrt_nm:
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
pub struct GroupHeader116 {
    #[serde(rename = "MsgId")]
    pub msg_id:
        Max35Text
,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm:
        ISODateTime
,
    #[serde(rename = "MsgRcpt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_rcpt:
Option<        PartyIdentification272
>,
    #[serde(rename = "MsgPgntn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_pgntn:
Option<        Pagination1
>,
    #[serde(rename = "OrgnlBizQry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_biz_qry:
Option<        OriginalBusinessQuery1
>,
    #[serde(rename = "AddtlInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_inf:
Option<        Max500Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct IdentificationSource3Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalFinancialInstrumentIdentificationType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ImpliedCurrencyAmountRange1Choice {
    #[serde(rename = "FrAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_amt:
Option<        AmountRangeBoundary1
>,
    #[serde(rename = "ToAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_amt:
Option<        AmountRangeBoundary1
>,
    #[serde(rename = "FrToAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_to_amt:
Option<        FromToAmountRange1
>,
    #[serde(rename = "EQAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eq_amt:
Option<        ImpliedCurrencyAndAmount
>,
    #[serde(rename = "NEQAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neq_amt:
Option<        ImpliedCurrencyAndAmount
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct InterestRecord2 {
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind:
        CreditDebitCode
,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        InterestType1Choice
>,
    #[serde(rename = "Rate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate:
Option<        Rate4
>,
    #[serde(rename = "FrToDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_to_dt:
Option<        DateTimePeriod1
>,
    #[serde(rename = "Rsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsn:
Option<        Max35Text
>,
    #[serde(rename = "Tax")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax:
Option<        TaxCharges2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct InterestType1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        InterestType1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
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
pub struct MessageIdentification2 {
    #[serde(rename = "MsgNmId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_nm_id:
Option<        Max35Text
>,
    #[serde(rename = "MsgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_id:
Option<        Max35Text
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
pub struct NumberAndSumOfTransactions1 {
    #[serde(rename = "NbOfNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries:
Option<        Max15NumericText
>,
    #[serde(rename = "Sum")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum:
Option<        DecimalNumber
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct NumberAndSumOfTransactions4 {
    #[serde(rename = "NbOfNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries:
Option<        Max15NumericText
>,
    #[serde(rename = "Sum")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum:
Option<        DecimalNumber
>,
    #[serde(rename = "TtlNetNtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_net_ntry:
Option<        AmountAndDirection35
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
pub struct OriginalAndCurrentQuantities1 {
    #[serde(rename = "FaceAmt")]
    pub face_amt:
        ImpliedCurrencyAndAmount
,
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val:
        ImpliedCurrencyAndAmount
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct OriginalBusinessQuery1 {
    #[serde(rename = "MsgId")]
    pub msg_id:
        Max35Text
,
    #[serde(rename = "MsgNmId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_nm_id:
Option<        Max35Text
>,
    #[serde(rename = "CreDtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm:
Option<        ISODateTime
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
pub struct OtherIdentification1 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "Sfx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sfx:
Option<        Max16Text
>,
    #[serde(rename = "Tp")]
    pub tp:
        IdentificationSource3Choice
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Pagination1 {
    #[serde(rename = "PgNb")]
    pub pg_nb:
        Max5NumericText
,
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind:
        YesNoIndicator
,
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
pub struct PaymentCard4 {
    #[serde(rename = "PlainCardData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plain_card_data:
Option<        PlainCardData1
>,
    #[serde(rename = "CardCtryCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd:
Option<        Exact3NumericText
>,
    #[serde(rename = "CardBrnd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_brnd:
Option<        GenericIdentification1
>,
    #[serde(rename = "AddtlCardData")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_card_data:
Option<        Max70Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PaymentContext3 {
    #[serde(rename = "CardPres")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_pres:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "CrdhldrPres")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crdhldr_pres:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "OnLineCntxt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_line_cntxt:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "AttndncCntxt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attndnc_cntxt:
Option<        AttendanceContext1Code
>,
    #[serde(rename = "TxEnvt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_envt:
Option<        TransactionEnvironment1Code
>,
    #[serde(rename = "TxChanl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_chanl:
Option<        TransactionChannel1Code
>,
    #[serde(rename = "AttndntMsgCpbl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attndnt_msg_cpbl:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "AttndntLang")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attndnt_lang:
Option<        ISO2ALanguageCode
>,
    #[serde(rename = "CardDataNtryMd")]
    pub card_data_ntry_md:
        CardDataReading1Code
,
    #[serde(rename = "FllbckInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fllbck_ind:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "AuthntcnMtd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authntcn_mtd:
Option<        CardholderAuthentication2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PaymentReturnReason8 {
    #[serde(rename = "OrgnlBkTxCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_bk_tx_cd:
Option<        BankTransactionCodeStructure4
>,
    #[serde(rename = "Orgtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgtr:
Option<        PartyIdentification272
>,
    #[serde(rename = "Rsn")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsn:
Option<        ReturnReason5Choice
>,
    #[serde(rename = "AddtlInf")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub addtl_inf:
Vec<        Max105Text
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
pub struct PlainCardData1 {
    #[serde(rename = "PAN")]
    pub pan:
        Min8Max28NumericText
,
    #[serde(rename = "CardSeqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_seq_nb:
Option<        Min2Max3NumericText
>,
    #[serde(rename = "FctvDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fctv_dt:
Option<        ISOYearMonth
>,
    #[serde(rename = "XpryDt")]
    pub xpry_dt:
        ISOYearMonth
,
    #[serde(rename = "SvcCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub svc_cd:
Option<        Exact3NumericText
>,
    #[serde(rename = "TrckData")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trck_data:
Vec<        TrackData1
>,
    #[serde(rename = "CardSctyCd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_scty_cd:
Option<        CardSecurityInformation1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PointOfInteraction1 {
    #[serde(rename = "Id")]
    pub id:
        GenericIdentification32
,
    #[serde(rename = "SysNm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sys_nm:
Option<        Max70Text
>,
    #[serde(rename = "GrpId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grp_id:
Option<        Max35Text
>,
    #[serde(rename = "Cpblties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpblties:
Option<        PointOfInteractionCapabilities1
>,
    #[serde(rename = "Cmpnt")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cmpnt:
Vec<        PointOfInteractionComponent1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PointOfInteractionCapabilities1 {
    #[serde(rename = "CardRdngCpblties")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub card_rdng_cpblties:
Vec<        CardDataReading1Code
>,
    #[serde(rename = "CrdhldrVrfctnCpblties")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub crdhldr_vrfctn_cpblties:
Vec<        CardholderVerificationCapability1Code
>,
    #[serde(rename = "OnLineCpblties")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_line_cpblties:
Option<        OnLineCapability1Code
>,
    #[serde(rename = "DispCpblties")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disp_cpblties:
Vec<        DisplayCapabilities1
>,
    #[serde(rename = "PrtLineWidth")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prt_line_width:
Option<        Max3NumericText
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PointOfInteractionComponent1 {
    #[serde(rename = "POICmpntTp")]
    pub poi_cmpnt_tp:
        POIComponentType1Code
,
    #[serde(rename = "ManfctrId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manfctr_id:
Option<        Max35Text
>,
    #[serde(rename = "Mdl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mdl:
Option<        Max35Text
>,
    #[serde(rename = "VrsnNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vrsn_nb:
Option<        Max16Text
>,
    #[serde(rename = "SrlNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub srl_nb:
Option<        Max35Text
>,
    #[serde(rename = "ApprvlNb")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub apprvl_nb:
Vec<        Max70Text
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
pub struct Price7 {
    #[serde(rename = "Tp")]
    pub tp:
        YieldedOrValueType1Choice
,
    #[serde(rename = "Val")]
    pub val:
        PriceRateOrAmount3Choice
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct PriceRateOrAmount3Choice {
    #[serde(rename = "Rate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate:
Option<        PercentageRate
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        ActiveOrHistoricCurrencyAnd13DecimalAmount
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct Product2 {
    #[serde(rename = "PdctCd")]
    pub pdct_cd:
        Max70Text
,
    #[serde(rename = "UnitOfMeasr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measr:
Option<        UnitOfMeasure1Code
>,
    #[serde(rename = "PdctQty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdct_qty:
Option<        DecimalNumber
>,
    #[serde(rename = "UnitPric")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_pric:
Option<        ImpliedCurrencyAndAmount
>,
    #[serde(rename = "PdctAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pdct_amt:
Option<        ImpliedCurrencyAndAmount
>,
    #[serde(rename = "TaxTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_tp:
Option<        Max35Text
>,
    #[serde(rename = "AddtlPdctInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_inf:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryAgent5 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Agt")]
    pub agt:
        BranchAndFinancialInstitutionIdentification8
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryBankTransactionCodeStructure1 {
    #[serde(rename = "Cd")]
    pub cd:
        Max35Text
,
    #[serde(rename = "Issr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryDate3 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Dt")]
    pub dt:
        DateAndDateTime2Choice
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryParty6 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Pty")]
    pub pty:
        Party50Choice
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryPrice2 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Pric")]
    pub pric:
        ActiveOrHistoricCurrencyAndAmount
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryQuantity1 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Qty")]
    pub qty:
        Max35Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ProprietaryReference1 {
    #[serde(rename = "Tp")]
    pub tp:
        Max35Text
,
    #[serde(rename = "Ref")]
    pub r#ref:
        Max35Text
,
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
pub struct Rate4 {
    #[serde(rename = "Tp")]
    pub tp:
        RateType4Choice
,
    #[serde(rename = "VldtyRg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vldty_rg:
Option<        ActiveOrHistoricCurrencyAndAmountRange2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct RateType4Choice {
    #[serde(rename = "Pctg")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pctg:
Option<        PercentageRate
>,
    #[serde(rename = "Othr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub othr:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct References74Choice {
    #[serde(rename = "SctiesSttlmTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scties_sttlm_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "IntraPosMvmntId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intra_pos_mvmnt_id:
Option<        Max35Text
>,
    #[serde(rename = "IntraBalMvmntId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intra_bal_mvmnt_id:
Option<        Max35Text
>,
    #[serde(rename = "AcctSvcrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "MktInfrstrctrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "CtrPtyMktInfrstrctrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ctr_pty_mkt_infrstrctr_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "PoolId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool_id:
Option<        Max35Text
>,
    #[serde(rename = "CmonId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cmon_id:
Option<        Max35Text
>,
    #[serde(rename = "TradId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trad_id:
Option<        Max52Text
>,
    #[serde(rename = "OthrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub othr_tx_id:
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
pub struct ReportEntry15 {
    #[serde(rename = "NtryRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ntry_ref:
Option<        Max35Text
>,
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind:
        CreditDebitCode
,
    #[serde(rename = "RvslInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rvsl_ind:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "Sts")]
    pub sts:
        EntryStatus1Choice
,
    #[serde(rename = "BookgDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookg_dt:
Option<        DateAndDateTime2Choice
>,
    #[serde(rename = "ValDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub val_dt:
Option<        DateAndDateTime2Choice
>,
    #[serde(rename = "AcctSvcrRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref:
Option<        Max35Text
>,
    #[serde(rename = "Avlbty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub avlbty:
Vec<        CashAvailability1
>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd:
        BankTransactionCodeStructure4
,
    #[serde(rename = "ComssnWvrInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comssn_wvr_ind:
Option<        YesNoIndicator
>,
    #[serde(rename = "AddtlInfInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_inf_ind:
Option<        MessageIdentification2
>,
    #[serde(rename = "AmtDtls")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt_dtls:
Option<        AmountAndCurrencyExchange4
>,
    #[serde(rename = "Chrgs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chrgs:
Option<        Charges15
>,
    #[serde(rename = "TechInptChanl")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tech_inpt_chanl:
Option<        TechnicalInputChannel1Choice
>,
    #[serde(rename = "Intrst")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrst:
Option<        TransactionInterest4
>,
    #[serde(rename = "CardTx")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_tx:
Option<        CardEntry5
>,
    #[serde(rename = "NtryDtls")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ntry_dtls:
Vec<        EntryDetails14
>,
    #[serde(rename = "AddtlNtryInf")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addtl_ntry_inf:
Option<        Max500Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ReportingSource1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalReportingSource1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct ReturnReason5Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalReturnReason1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SecuritiesAccount19 {
    #[serde(rename = "Id")]
    pub id:
        Max35Text
,
    #[serde(rename = "Tp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tp:
Option<        GenericIdentification30
>,
    #[serde(rename = "Nm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nm:
Option<        Max70Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isin:
Option<        ISINOct2015Identifier
>,
    #[serde(rename = "OthrId")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub othr_id:
Vec<        OtherIdentification1
>,
    #[serde(rename = "Desc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desc:
Option<        Max140Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SequenceRange1 {
    #[serde(rename = "FrSeq")]
    pub fr_seq:
        Max35Text
,
    #[serde(rename = "ToSeq")]
    pub to_seq:
        Max35Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct SequenceRange1Choice {
    #[serde(rename = "FrSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_seq:
Option<        Max35Text
>,
    #[serde(rename = "ToSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_seq:
Option<        Max35Text
>,
    #[serde(rename = "FrToSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fr_to_seq:
Option<Vec<        SequenceRange1
>>,
    #[serde(rename = "EQSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eq_seq:
Option<Vec<        Max35Text
>>,
    #[serde(rename = "NEQSeq")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neq_seq:
Option<Vec<        Max35Text
>>,
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
pub struct TaxCharges2 {
    #[serde(rename = "Id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id:
Option<        Max35Text
>,
    #[serde(rename = "Rate")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate:
Option<        PercentageRate
>,
    #[serde(rename = "Amt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amt:
Option<        ActiveOrHistoricCurrencyAndAmount
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
pub struct TechnicalInputChannel1Choice {
    #[serde(rename = "Cd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cd:
Option<        ExternalTechnicalInputChannel1Code
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        Max35Text
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TotalTransactions6 {
    #[serde(rename = "TtlNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_ntries:
Option<        NumberAndSumOfTransactions4
>,
    #[serde(rename = "TtlCdtNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_cdt_ntries:
Option<        NumberAndSumOfTransactions1
>,
    #[serde(rename = "TtlDbtNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_dbt_ntries:
Option<        NumberAndSumOfTransactions1
>,
    #[serde(rename = "TtlNtriesPerBkTxCd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ttl_ntries_per_bk_tx_cd:
Vec<        TotalsPerBankTransactionCode5
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TotalsPerBankTransactionCode5 {
    #[serde(rename = "NbOfNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries:
Option<        Max15NumericText
>,
    #[serde(rename = "Sum")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum:
Option<        DecimalNumber
>,
    #[serde(rename = "TtlNetNtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_net_ntry:
Option<        AmountAndDirection35
>,
    #[serde(rename = "CdtNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdt_ntries:
Option<        NumberAndSumOfTransactions1
>,
    #[serde(rename = "DbtNtries")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbt_ntries:
Option<        NumberAndSumOfTransactions1
>,
    #[serde(rename = "FcstInd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fcst_ind:
Option<        TrueFalseIndicator
>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd:
        BankTransactionCodeStructure4
,
    #[serde(rename = "Avlbty")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub avlbty:
Vec<        CashAvailability1
>,
    #[serde(rename = "Dt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dt:
Option<        DateAndDateTime2Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TrackData1 {
    #[serde(rename = "TrckNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trck_nb:
Option<        Exact1NumericText
>,
    #[serde(rename = "TrckVal")]
    pub trck_val:
        Max140Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionAgents6 {
    #[serde(rename = "InstgAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instg_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "InstdAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instd_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "DbtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dbtr_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "CdtrAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdtr_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "RcvgAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rcvg_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "DlvrgAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dlvrg_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "IssgAgt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issg_agt:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "SttlmPlc")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sttlm_plc:
Option<        BranchAndFinancialInstitutionIdentification8
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prtry:
Vec<        ProprietaryAgent5
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionAllocation1 {
    #[serde(rename = "Amt")]
    pub amt:
        ActiveOrHistoricCurrencyAndAmount
,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind:
        CreditDebitCode
,
    #[serde(rename = "Acct")]
    pub acct:
        CashAccount40
,
    #[serde(rename = "Purp")]
    pub purp:
        Purpose2Choice
,
    #[serde(rename = "Ref")]
    pub r#ref:
        Max35Text
,
    #[serde(rename = "RltdRefs")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rltd_refs:
Vec<        References74Choice
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionDates3 {
    #[serde(rename = "AccptncDtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm:
Option<        ISODateTime
>,
    #[serde(rename = "TradActvtyCtrctlSttlmDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trad_actvty_ctrctl_sttlm_dt:
Option<        ISODate
>,
    #[serde(rename = "TradDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trad_dt:
Option<        ISODate
>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt:
Option<        ISODate
>,
    #[serde(rename = "StartDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_dt:
Option<        ISODate
>,
    #[serde(rename = "EndDt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_dt:
Option<        ISODate
>,
    #[serde(rename = "TxDtTm")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_dt_tm:
Option<        ISODateTime
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prtry:
Vec<        ProprietaryDate3
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionIdentifier1 {
    #[serde(rename = "TxDtTm")]
    pub tx_dt_tm:
        ISODateTime
,
    #[serde(rename = "TxRef")]
    pub tx_ref:
        Max35Text
,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionInterest4 {
    #[serde(rename = "TtlIntrstAndTaxAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_intrst_and_tax_amt:
Option<        ActiveOrHistoricCurrencyAndAmount
>,
    #[serde(rename = "Rcrd")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rcrd:
Vec<        InterestRecord2
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionParties12 {
    #[serde(rename = "InitgPty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initg_pty:
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
    #[serde(rename = "UltmtDbtr")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr:
Option<        Party50Choice
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
    #[serde(rename = "TradgPty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tradg_pty:
Option<        Party50Choice
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prtry:
Vec<        ProprietaryParty6
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionPrice4Choice {
    #[serde(rename = "DealPric")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deal_pric:
Option<        Price7
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<Vec<        ProprietaryPrice2
>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionQuantities4Choice {
    #[serde(rename = "Qty")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qty:
Option<        FinancialInstrumentQuantity33Choice
>,
    #[serde(rename = "OrgnlAndCurFaceAmt")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgnl_and_cur_face_amt:
Option<        OriginalAndCurrentQuantities1
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prtry:
Option<        ProprietaryQuantity1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct TransactionReferences6 {
    #[serde(rename = "MsgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub msg_id:
Option<        Max35Text
>,
    #[serde(rename = "AcctSvcrRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref:
Option<        Max35Text
>,
    #[serde(rename = "PmtInfId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id:
Option<        Max35Text
>,
    #[serde(rename = "InstrId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instr_id:
Option<        Max35Text
>,
    #[serde(rename = "EndToEndId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_to_end_id:
Option<        Max35Text
>,
    #[serde(rename = "UETR")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uetr:
Option<        UUIDv4Identifier
>,
    #[serde(rename = "TxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_id:
Option<        Max35Text
>,
    #[serde(rename = "MndtId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mndt_id:
Option<        Max35Text
>,
    #[serde(rename = "ChqNb")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chq_nb:
Option<        Max35Text
>,
    #[serde(rename = "ClrSysRef")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref:
Option<        Max35Text
>,
    #[serde(rename = "AcctOwnrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acct_ownr_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "AcctSvcrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "MktInfrstrctrTxId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id:
Option<        Max35Text
>,
    #[serde(rename = "PrcgId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prcg_id:
Option<        Max35Text
>,
    #[serde(rename = "Prtry")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prtry:
Vec<        ProprietaryReference1
>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
#[derive(Default)]
pub struct YieldedOrValueType1Choice {
    #[serde(rename = "Yldd")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yldd:
Option<        YesNoIndicator
>,
    #[serde(rename = "ValTp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub val_tp:
Option<        PriceValueType1Code
>,
}
