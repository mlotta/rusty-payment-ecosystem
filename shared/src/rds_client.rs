//! RdsClient is use to communicate with an AWS Aurora DB
use crate::settings::RdsSettings;
use aws_config::SdkConfig;
use aws_sdk_rdsdata::operation::execute_statement::builders::ExecuteStatementFluentBuilder;
use secrecy::{ExposeSecret, Secret};

#[derive(Clone)]
pub struct RdsClient {
    client: aws_sdk_rdsdata::Client,
    secret_arn: Secret<String>,
    cluster_arn: String,
    db_instance: String,
}

impl RdsClient {
    pub fn new(settings: &RdsSettings, sdk_config: &SdkConfig) -> Self {
        RdsClient {
            client: aws_sdk_rdsdata::Client::new(sdk_config),
            secret_arn: settings.secretarn.clone(),
            cluster_arn: settings.clusterarn.clone(),
            db_instance: settings.dbinstance.clone(),
        }
    }

    pub fn execute_statement(&self) -> ExecuteStatementFluentBuilder {
        self.client
            .execute_statement()
            .secret_arn(self.secret_arn.expose_secret())
            .resource_arn(self.cluster_arn.as_str())
            .database(self.db_instance.as_str())
    }
}
