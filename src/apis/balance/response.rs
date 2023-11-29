use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BalanceRawResponse {
    #[serde(alias = "an")]
    pub year: usize,

    #[serde(alias = "cui")]
    pub unique_registration_code: usize,

    #[serde(alias = "deni")]
    pub name: String,

    #[serde(alias = "caen")]
    pub activity_code: usize,

    #[serde(alias = "den_caen")]
    pub activity_name: String,

    pub balance: Vec<RawBalance>,
}

#[derive(Debug, Clone)]
pub struct BalanceResponse {
    pub year: usize,
    pub unique_registration_code: usize,
    pub name: String,
    pub activity_code: usize,
    pub activity_name: String,
    pub balance: BTreeMap<BalanceIndicator, RawBalance>,
}

impl From<BalanceRawResponse> for BalanceResponse {
    fn from(value: BalanceRawResponse) -> Self {
        let is_ngo = value.balance.iter().any(|it| it.code == "I46");

        let balance = match is_ngo {
            true => value
                .balance
                .into_iter()
                .map(|it| (BalanceIndicator::Ngo(it.clone().code.into()), it))
                .collect(),
            false => value
                .balance
                .into_iter()
                .map(|it| (BalanceIndicator::Company(it.clone().code.into()), it))
                .collect(),
        };

        Self {
            year: value.year,
            unique_registration_code: value.unique_registration_code,
            name: value.name,
            activity_code: value.activity_code,
            activity_name: value.activity_name,
            balance,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RawBalance {
    #[serde(alias = "indicator")]
    pub code: String,

    #[serde(alias = "val_den_indicator")]
    pub name: String,

    #[serde(alias = "val_indicator")]
    pub value: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BalanceIndicator {
    Company(CompanyBalanceIndicatorKind),
    Ngo(NgoBalanceIndicatorKind),
}

impl BalanceIndicator {
    pub fn company(self) -> CompanyBalanceIndicatorKind {
        match self {
            BalanceIndicator::Company(it) => it,
            _ => panic!("Expected Company BalanceIndicator"),
        }
    }

    pub fn ngo(self) -> NgoBalanceIndicatorKind {
        match self {
            BalanceIndicator::Ngo(it) => it,
            _ => panic!("Expected Ngo BalanceIndicator"),
        }
    }

    pub fn is_company(&self) -> bool {
        matches!(self, BalanceIndicator::Company(_))
    }

    pub fn is_ngo(&self) -> bool {
        matches!(self, BalanceIndicator::Ngo(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompanyBalanceIndicatorKind {
    AverageNumberOfEmployees, // I20
    NetLoss,                  // I19
    NetProfit,                // I18
    GrossLoss,                // I17
    GrossProfit,              // I16
    TotalExpenditures,        // I15
    TotalIncome,              // I14
    NetTurnorver,             // I13
    HeritageOfTheKingdom,     // I12
    PaidSubscribedCapital,    // I11
    CapitalTotal,             // I10
    Provisions,               // I9
    IncomeInAdvance,          // I8
    Liabilities,              // I7
    PrePayments,              // I6
    CashAndBankAccounts,      // I5
    Debt,                     // I4
    Inventories,              // I3
    CurrentAssets,            // I2
    FixedAssets,              // I1
}

impl From<String> for CompanyBalanceIndicatorKind {
    fn from(value: String) -> Self {
        match &*value {
            "I20" => CompanyBalanceIndicatorKind::AverageNumberOfEmployees,
            "I19" => CompanyBalanceIndicatorKind::NetLoss,
            "I18" => CompanyBalanceIndicatorKind::NetProfit,
            "I17" => CompanyBalanceIndicatorKind::GrossLoss,
            "I16" => CompanyBalanceIndicatorKind::GrossProfit,
            "I15" => CompanyBalanceIndicatorKind::TotalExpenditures,
            "I14" => CompanyBalanceIndicatorKind::TotalIncome,
            "I13" => CompanyBalanceIndicatorKind::NetTurnorver,
            "I12" => CompanyBalanceIndicatorKind::HeritageOfTheKingdom,
            "I11" => CompanyBalanceIndicatorKind::PaidSubscribedCapital,
            "I10" => CompanyBalanceIndicatorKind::CapitalTotal,
            "I9" => CompanyBalanceIndicatorKind::Provisions,
            "I8" => CompanyBalanceIndicatorKind::IncomeInAdvance,
            "I7" => CompanyBalanceIndicatorKind::Liabilities,
            "I6" => CompanyBalanceIndicatorKind::PrePayments,
            "I5" => CompanyBalanceIndicatorKind::CashAndBankAccounts,
            "I4" => CompanyBalanceIndicatorKind::Debt,
            "I3" => CompanyBalanceIndicatorKind::Inventories,
            "I2" => CompanyBalanceIndicatorKind::CurrentAssets,
            "I1" => CompanyBalanceIndicatorKind::FixedAssets,
            _ => panic!("Unknown BalanceIndicator: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NgoBalanceIndicatorKind {
    PersonnelEffectiveInEconomicActivity,             //I46
    PersonnelEffectiveInNonProfitActivity,            //I45
    DeficitOrLoss,                                    //I44
    DeficitOrLossProjection,                          //I43
    SurplusOrProfit,                                  //I42
    SurplusOrProfitProjection,                        //I41
    TotalExpenditures,                                //I40
    TotalExpendituresProjection,                      //I39
    TotalIncome,                                      //I38
    TotalIncomeProjection,                            //I37
    LossFromEconomicActivity,                         //I36
    LossFromEconomicActivityProjection,               //I35
    ProfitFromEconomicActivity,                       //I34
    ProfitFromEconomicActivityProjection,             //I33
    ExpendituresForEconomicActivity,                  //I32
    ExpendituresForEconomicActivityProjection,        //I13
    IncomeFromEconomicActivity,                       //I30
    IncomeFromEconomicActivityProjection,             //I29
    LossFromSpecialPurposesActivity,                  //I28
    LossFromSpecialPurposesActivityProjection,        //I27
    SurplusFromSpecialPurposesActivity,               //I26
    SurplusFromSpecialPurposesActivityProjection,     //I25
    ExpendituresForSpecialPurposesActivity,           //I24
    ExpendituresForSpecialPurposesActivityProjection, //I23
    IncomeFromSpecialPurposesActivity,                //I22
    IncomeFromSpecialPurposesActivityProjection,      //I21
    DeficitFromNonProfitActivity,                     //I20
    DeficitFromNonProfitActivityProjection,           //I19
    SurplusFromNonProfitActivity,                     //I18
    SurplusFromNonProfitActivityProjection,           //I17
    ExpendituresForNonProfitActivity,                 //I16
    ExpendituresForNonProfitActivityProjection,       //I15
    IncomeFromNonProfitActivity,                      //I14
    IncomeFromNonProfitActivityProjection,            //I13
    CapitalTotal,                                     //I12
    FundsForNonProfitActivities,                      //I11
    OwnedCapitalTotal,                                //I10
    IncomeInAdvance,                                  //I9
    Provisions,                                       //I8
    LiabilitiesToBePaidInMoreThanOneYear,             //I7
    TotalAssetsMinusCurrentLiabilities,               //I6
    NetCurrentAssetsAndNetCurrentLiabilities,         //I5
    LiabilitiesToBePaidInLessThanOneYear,             //I4
    PrePayments,                                      //I3
    CurrentAssetsTotal,                               //I2
    FixedAssetsTotal,                                 //I1
}

impl From<String> for NgoBalanceIndicatorKind {
    fn from(value: String) -> Self {
        match &*value {
            "I46" => NgoBalanceIndicatorKind::PersonnelEffectiveInEconomicActivity,
            "I45" => NgoBalanceIndicatorKind::PersonnelEffectiveInNonProfitActivity,
            "I44" => NgoBalanceIndicatorKind::DeficitOrLoss,
            "I43" => NgoBalanceIndicatorKind::DeficitOrLossProjection,
            "I42" => NgoBalanceIndicatorKind::SurplusOrProfit,
            "I41" => NgoBalanceIndicatorKind::SurplusOrProfitProjection,
            "I40" => NgoBalanceIndicatorKind::TotalExpenditures,
            "I39" => NgoBalanceIndicatorKind::TotalExpendituresProjection,
            "I38" => NgoBalanceIndicatorKind::TotalIncome,
            "I37" => NgoBalanceIndicatorKind::TotalIncomeProjection,
            "I36" => NgoBalanceIndicatorKind::LossFromEconomicActivity,
            "I35" => NgoBalanceIndicatorKind::LossFromEconomicActivityProjection,
            "I34" => NgoBalanceIndicatorKind::ProfitFromEconomicActivity,
            "I33" => NgoBalanceIndicatorKind::ProfitFromEconomicActivityProjection,
            "I32" => NgoBalanceIndicatorKind::ExpendituresForEconomicActivity,
            "I31" => NgoBalanceIndicatorKind::ExpendituresForEconomicActivityProjection,
            "I30" => NgoBalanceIndicatorKind::IncomeFromEconomicActivity,
            "I29" => NgoBalanceIndicatorKind::IncomeFromEconomicActivityProjection,
            "I28" => NgoBalanceIndicatorKind::LossFromSpecialPurposesActivity,
            "I27" => NgoBalanceIndicatorKind::LossFromSpecialPurposesActivityProjection,
            "I26" => NgoBalanceIndicatorKind::SurplusFromSpecialPurposesActivity,
            "I25" => NgoBalanceIndicatorKind::SurplusFromSpecialPurposesActivityProjection,
            "I24" => NgoBalanceIndicatorKind::ExpendituresForSpecialPurposesActivity,
            "I23" => NgoBalanceIndicatorKind::ExpendituresForSpecialPurposesActivityProjection,
            "I22" => NgoBalanceIndicatorKind::IncomeFromSpecialPurposesActivity,
            "I21" => NgoBalanceIndicatorKind::IncomeFromSpecialPurposesActivityProjection,
            "I20" => NgoBalanceIndicatorKind::DeficitFromNonProfitActivity,
            "I19" => NgoBalanceIndicatorKind::DeficitFromNonProfitActivityProjection,
            "I18" => NgoBalanceIndicatorKind::SurplusFromNonProfitActivity,
            "I17" => NgoBalanceIndicatorKind::SurplusFromNonProfitActivityProjection,
            "I16" => NgoBalanceIndicatorKind::ExpendituresForNonProfitActivity,
            "I15" => NgoBalanceIndicatorKind::ExpendituresForNonProfitActivityProjection,
            "I14" => NgoBalanceIndicatorKind::IncomeFromNonProfitActivity,
            "I13" => NgoBalanceIndicatorKind::IncomeFromNonProfitActivityProjection,
            "I12" => NgoBalanceIndicatorKind::CapitalTotal,
            "I11" => NgoBalanceIndicatorKind::FundsForNonProfitActivities,
            "I10" => NgoBalanceIndicatorKind::OwnedCapitalTotal,
            "I9" => NgoBalanceIndicatorKind::IncomeInAdvance,
            "I8" => NgoBalanceIndicatorKind::Provisions,
            "I7" => NgoBalanceIndicatorKind::LiabilitiesToBePaidInMoreThanOneYear,
            "I6" => NgoBalanceIndicatorKind::TotalAssetsMinusCurrentLiabilities,
            "I5" => NgoBalanceIndicatorKind::NetCurrentAssetsAndNetCurrentLiabilities,
            "I4" => NgoBalanceIndicatorKind::LiabilitiesToBePaidInLessThanOneYear,
            "I3" => NgoBalanceIndicatorKind::PrePayments,
            "I2" => NgoBalanceIndicatorKind::CurrentAssetsTotal,
            "I1" => NgoBalanceIndicatorKind::FixedAssetsTotal,
            _ => panic!("Unknown BalanceIndicator: {}", value),
        }
    }
}
