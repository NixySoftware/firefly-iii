use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    ReqwestMiddleware(reqwest_middleware::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::ReqwestMiddleware(e) => ("reqwest-middleware", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::ReqwestMiddleware(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<reqwest_middleware::Error> for Error<T> {
    fn from(e: reqwest_middleware::Error) -> Self {
        Error::ReqwestMiddleware(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String),
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            Self::Json
        } else if content_type.starts_with("text/plain") {
            Self::Text
        } else {
            Self::Unsupported(content_type.to_string())
        }
    }
}

pub mod about_api;
pub mod accounts_api;
pub mod attachments_api;
pub mod autocomplete_api;
pub mod available_budgets_api;
pub mod bills_api;
pub mod budgets_api;
pub mod categories_api;
pub mod charts_api;
pub mod configuration_api;
pub mod currencies_api;
pub mod currency_exchange_rates_api;
pub mod data_api;
pub mod insight_api;
pub mod links_api;
pub mod object_groups_api;
pub mod piggy_banks_api;
pub mod preferences_api;
pub mod recurrences_api;
pub mod rule_groups_api;
pub mod rules_api;
pub mod search_api;
pub mod summary_api;
pub mod tags_api;
pub mod transactions_api;
pub mod user_groups_api;
pub mod users_api;
pub mod webhooks_api;

pub mod configuration;

use std::sync::Arc;

pub trait Api {
    fn about_api(&self) -> &dyn about_api::AboutApi;
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi;
    fn attachments_api(&self) -> &dyn attachments_api::AttachmentsApi;
    fn autocomplete_api(&self) -> &dyn autocomplete_api::AutocompleteApi;
    fn available_budgets_api(&self) -> &dyn available_budgets_api::AvailableBudgetsApi;
    fn bills_api(&self) -> &dyn bills_api::BillsApi;
    fn budgets_api(&self) -> &dyn budgets_api::BudgetsApi;
    fn categories_api(&self) -> &dyn categories_api::CategoriesApi;
    fn charts_api(&self) -> &dyn charts_api::ChartsApi;
    fn configuration_api(&self) -> &dyn configuration_api::ConfigurationApi;
    fn currencies_api(&self) -> &dyn currencies_api::CurrenciesApi;
    fn currency_exchange_rates_api(
        &self,
    ) -> &dyn currency_exchange_rates_api::CurrencyExchangeRatesApi;
    fn data_api(&self) -> &dyn data_api::DataApi;
    fn insight_api(&self) -> &dyn insight_api::InsightApi;
    fn links_api(&self) -> &dyn links_api::LinksApi;
    fn object_groups_api(&self) -> &dyn object_groups_api::ObjectGroupsApi;
    fn piggy_banks_api(&self) -> &dyn piggy_banks_api::PiggyBanksApi;
    fn preferences_api(&self) -> &dyn preferences_api::PreferencesApi;
    fn recurrences_api(&self) -> &dyn recurrences_api::RecurrencesApi;
    fn rule_groups_api(&self) -> &dyn rule_groups_api::RuleGroupsApi;
    fn rules_api(&self) -> &dyn rules_api::RulesApi;
    fn search_api(&self) -> &dyn search_api::SearchApi;
    fn summary_api(&self) -> &dyn summary_api::SummaryApi;
    fn tags_api(&self) -> &dyn tags_api::TagsApi;
    fn transactions_api(&self) -> &dyn transactions_api::TransactionsApi;
    fn user_groups_api(&self) -> &dyn user_groups_api::UserGroupsApi;
    fn users_api(&self) -> &dyn users_api::UsersApi;
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi;
}

pub struct ApiClient {
    about_api: Box<dyn about_api::AboutApi>,
    accounts_api: Box<dyn accounts_api::AccountsApi>,
    attachments_api: Box<dyn attachments_api::AttachmentsApi>,
    autocomplete_api: Box<dyn autocomplete_api::AutocompleteApi>,
    available_budgets_api: Box<dyn available_budgets_api::AvailableBudgetsApi>,
    bills_api: Box<dyn bills_api::BillsApi>,
    budgets_api: Box<dyn budgets_api::BudgetsApi>,
    categories_api: Box<dyn categories_api::CategoriesApi>,
    charts_api: Box<dyn charts_api::ChartsApi>,
    configuration_api: Box<dyn configuration_api::ConfigurationApi>,
    currencies_api: Box<dyn currencies_api::CurrenciesApi>,
    currency_exchange_rates_api: Box<dyn currency_exchange_rates_api::CurrencyExchangeRatesApi>,
    data_api: Box<dyn data_api::DataApi>,
    insight_api: Box<dyn insight_api::InsightApi>,
    links_api: Box<dyn links_api::LinksApi>,
    object_groups_api: Box<dyn object_groups_api::ObjectGroupsApi>,
    piggy_banks_api: Box<dyn piggy_banks_api::PiggyBanksApi>,
    preferences_api: Box<dyn preferences_api::PreferencesApi>,
    recurrences_api: Box<dyn recurrences_api::RecurrencesApi>,
    rule_groups_api: Box<dyn rule_groups_api::RuleGroupsApi>,
    rules_api: Box<dyn rules_api::RulesApi>,
    search_api: Box<dyn search_api::SearchApi>,
    summary_api: Box<dyn summary_api::SummaryApi>,
    tags_api: Box<dyn tags_api::TagsApi>,
    transactions_api: Box<dyn transactions_api::TransactionsApi>,
    user_groups_api: Box<dyn user_groups_api::UserGroupsApi>,
    users_api: Box<dyn users_api::UsersApi>,
    webhooks_api: Box<dyn webhooks_api::WebhooksApi>,
}

impl ApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            about_api: Box::new(about_api::AboutApiClient::new(configuration.clone())),
            accounts_api: Box::new(accounts_api::AccountsApiClient::new(configuration.clone())),
            attachments_api: Box::new(attachments_api::AttachmentsApiClient::new(
                configuration.clone(),
            )),
            autocomplete_api: Box::new(autocomplete_api::AutocompleteApiClient::new(
                configuration.clone(),
            )),
            available_budgets_api: Box::new(available_budgets_api::AvailableBudgetsApiClient::new(
                configuration.clone(),
            )),
            bills_api: Box::new(bills_api::BillsApiClient::new(configuration.clone())),
            budgets_api: Box::new(budgets_api::BudgetsApiClient::new(configuration.clone())),
            categories_api: Box::new(categories_api::CategoriesApiClient::new(
                configuration.clone(),
            )),
            charts_api: Box::new(charts_api::ChartsApiClient::new(configuration.clone())),
            configuration_api: Box::new(configuration_api::ConfigurationApiClient::new(
                configuration.clone(),
            )),
            currencies_api: Box::new(currencies_api::CurrenciesApiClient::new(
                configuration.clone(),
            )),
            currency_exchange_rates_api: Box::new(
                currency_exchange_rates_api::CurrencyExchangeRatesApiClient::new(
                    configuration.clone(),
                ),
            ),
            data_api: Box::new(data_api::DataApiClient::new(configuration.clone())),
            insight_api: Box::new(insight_api::InsightApiClient::new(configuration.clone())),
            links_api: Box::new(links_api::LinksApiClient::new(configuration.clone())),
            object_groups_api: Box::new(object_groups_api::ObjectGroupsApiClient::new(
                configuration.clone(),
            )),
            piggy_banks_api: Box::new(piggy_banks_api::PiggyBanksApiClient::new(
                configuration.clone(),
            )),
            preferences_api: Box::new(preferences_api::PreferencesApiClient::new(
                configuration.clone(),
            )),
            recurrences_api: Box::new(recurrences_api::RecurrencesApiClient::new(
                configuration.clone(),
            )),
            rule_groups_api: Box::new(rule_groups_api::RuleGroupsApiClient::new(
                configuration.clone(),
            )),
            rules_api: Box::new(rules_api::RulesApiClient::new(configuration.clone())),
            search_api: Box::new(search_api::SearchApiClient::new(configuration.clone())),
            summary_api: Box::new(summary_api::SummaryApiClient::new(configuration.clone())),
            tags_api: Box::new(tags_api::TagsApiClient::new(configuration.clone())),
            transactions_api: Box::new(transactions_api::TransactionsApiClient::new(
                configuration.clone(),
            )),
            user_groups_api: Box::new(user_groups_api::UserGroupsApiClient::new(
                configuration.clone(),
            )),
            users_api: Box::new(users_api::UsersApiClient::new(configuration.clone())),
            webhooks_api: Box::new(webhooks_api::WebhooksApiClient::new(configuration.clone())),
        }
    }
}

impl Api for ApiClient {
    fn about_api(&self) -> &dyn about_api::AboutApi {
        self.about_api.as_ref()
    }
    fn accounts_api(&self) -> &dyn accounts_api::AccountsApi {
        self.accounts_api.as_ref()
    }
    fn attachments_api(&self) -> &dyn attachments_api::AttachmentsApi {
        self.attachments_api.as_ref()
    }
    fn autocomplete_api(&self) -> &dyn autocomplete_api::AutocompleteApi {
        self.autocomplete_api.as_ref()
    }
    fn available_budgets_api(&self) -> &dyn available_budgets_api::AvailableBudgetsApi {
        self.available_budgets_api.as_ref()
    }
    fn bills_api(&self) -> &dyn bills_api::BillsApi {
        self.bills_api.as_ref()
    }
    fn budgets_api(&self) -> &dyn budgets_api::BudgetsApi {
        self.budgets_api.as_ref()
    }
    fn categories_api(&self) -> &dyn categories_api::CategoriesApi {
        self.categories_api.as_ref()
    }
    fn charts_api(&self) -> &dyn charts_api::ChartsApi {
        self.charts_api.as_ref()
    }
    fn configuration_api(&self) -> &dyn configuration_api::ConfigurationApi {
        self.configuration_api.as_ref()
    }
    fn currencies_api(&self) -> &dyn currencies_api::CurrenciesApi {
        self.currencies_api.as_ref()
    }
    fn currency_exchange_rates_api(
        &self,
    ) -> &dyn currency_exchange_rates_api::CurrencyExchangeRatesApi {
        self.currency_exchange_rates_api.as_ref()
    }
    fn data_api(&self) -> &dyn data_api::DataApi {
        self.data_api.as_ref()
    }
    fn insight_api(&self) -> &dyn insight_api::InsightApi {
        self.insight_api.as_ref()
    }
    fn links_api(&self) -> &dyn links_api::LinksApi {
        self.links_api.as_ref()
    }
    fn object_groups_api(&self) -> &dyn object_groups_api::ObjectGroupsApi {
        self.object_groups_api.as_ref()
    }
    fn piggy_banks_api(&self) -> &dyn piggy_banks_api::PiggyBanksApi {
        self.piggy_banks_api.as_ref()
    }
    fn preferences_api(&self) -> &dyn preferences_api::PreferencesApi {
        self.preferences_api.as_ref()
    }
    fn recurrences_api(&self) -> &dyn recurrences_api::RecurrencesApi {
        self.recurrences_api.as_ref()
    }
    fn rule_groups_api(&self) -> &dyn rule_groups_api::RuleGroupsApi {
        self.rule_groups_api.as_ref()
    }
    fn rules_api(&self) -> &dyn rules_api::RulesApi {
        self.rules_api.as_ref()
    }
    fn search_api(&self) -> &dyn search_api::SearchApi {
        self.search_api.as_ref()
    }
    fn summary_api(&self) -> &dyn summary_api::SummaryApi {
        self.summary_api.as_ref()
    }
    fn tags_api(&self) -> &dyn tags_api::TagsApi {
        self.tags_api.as_ref()
    }
    fn transactions_api(&self) -> &dyn transactions_api::TransactionsApi {
        self.transactions_api.as_ref()
    }
    fn user_groups_api(&self) -> &dyn user_groups_api::UserGroupsApi {
        self.user_groups_api.as_ref()
    }
    fn users_api(&self) -> &dyn users_api::UsersApi {
        self.users_api.as_ref()
    }
    fn webhooks_api(&self) -> &dyn webhooks_api::WebhooksApi {
        self.webhooks_api.as_ref()
    }
}
