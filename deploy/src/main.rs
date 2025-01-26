// use aws_sdk_cloudformation::Error;
use shared::utils::setup_tracing;
use shared::settings::{get_settings, init_environment, BankSettings};
use thiserror::Error;
use aws_sdk_rds::error::SdkError;
use aws_sdk_rds::types::{DbCluster, DbClusterMember};
use aws_sdk_rds::operation::{
    describe_db_clusters::DescribeDBClustersError,
    delete_db_instance::DeleteDBInstanceError,
};
use tracing::{error, info};


#[derive(Debug, Error)]
enum AuroraError {
    #[error("Failed to describe clusters : {0}")]
    DescribeClusters(SdkError<DescribeDBClustersError>),
    #[error("Failed to delete instance : {0}")]
    DeleteInstance(SdkError<DeleteDBInstanceError>),
    #[error("Number of clusters found: {0}")]
    InvalidNumberOfClusters(usize),
}


async fn get_cluster(client: &aws_sdk_rds::Client) -> Result<DbCluster, AuroraError> {
    let clusters = client
        .describe_db_clusters()
        .send()
        .await
        .map_err(|err| AuroraError::DescribeClusters(err.into()))?
        .db_clusters
        .unwrap();
    
    let nb_clusters = clusters.len() ;
    if nb_clusters > 1 || nb_clusters == 0 {
        return Err(AuroraError::InvalidNumberOfClusters(nb_clusters));
    }

    Ok(clusters.first().cloned().unwrap())
}

async fn reset_cluster_instances(client: &aws_sdk_rds::Client, cluster: &DbCluster) -> Result<(), AuroraError> {
    let members = cluster.db_cluster_members.clone();

    let members = match members {
        Some(members) => {
            info!("Found {} cluster member", members.len());
            members
        },
        None => {
            info!("No members for cluster {:?}", cluster.db_cluster_identifier);
            Vec::<DbClusterMember>::new()
        }
    };

    for member in members{
        client
        .delete_db_instance()
        .db_instance_identifier(member.db_instance_identifier.clone().unwrap())
        .skip_final_snapshot(true)
        .send()
        .await
        .map_err(|err|AuroraError::DeleteInstance(err.into()))?;
        info!("Successfully deleted db instance: {}", member.db_instance_identifier.unwrap());
    }
    
    Ok(())
}

async fn create_bank_db_instance(client: &aws_sdk_rds::Client, cluster: &DbCluster, bank_name: String) -> Result<(), AuroraError>{
    client
        .create_db_instance()
        .db_name(bank_name)
        .db_instance_identifier(format!("db-instance-{}", bank_name))
        .engine("aurora-postgresql")
        .
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AuroraError> {
    setup_tracing();

    // Get AWS Config
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

    // Load settings
    let environment = init_environment().expect("Failed to initialize environment");
    let settings = get_settings(&environment).expect("Failed to load configuration");
    // dbg!(&settings.agents.bank.len());

    let client = aws_sdk_rds::Client::new(&sdk_config);
    let cluster = get_cluster(&client).await?;
    // reset_cluster_instances(&client, &cluster).await?;

    for (bank_name, _) in settings.agents.bank {
        create_bank_db_instance(&client, &cluster, bank_name).await?;

    }

    Ok(())

}
