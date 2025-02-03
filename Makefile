.PHONY: setup account_management order_card transaction

BUCKET_NAME := $(shell aws cloudformation describe-stacks --stack-name ecosystem-database --query "Stacks[0].Outputs[?ExportName=='ecosystem-database-EcosystemConfigBucketName'].OutputValue" --output text) 
DB_CLUSTER_ARN := $(shell aws cloudformation describe-stacks --stack-name ecosystem-database --query "Stacks[0].Outputs[?ExportName=='ecosystem-database-DatabaseClusterArn'].OutputValue" --output text) 
DB_SECRET_ARN := $(shell aws cloudformation describe-stacks --stack-name ecosystem-database --query "Stacks[0].Outputs[?ExportName=='ecosystem-database-DatabaseSecretArn'].OutputValue" --output text) 
DB_INSTANCE := $(shell aws cloudformation describe-stacks --stack-name ecosystem-database --query "Stacks[0].Outputs[?ExportName=='ecosystem-database-DatabaseClusterName'].OutputValue" --output text) 

setup:
	# Provision a DB Cluster and the ecosytem configuration in an S3 bucket
	sam deploy\
		--stack-name ecosystem-database \
		--template-file "deploy/aurora_cluster_template.yaml" 

	sleep 10 && aws s3 cp config/ecosystem-config.yaml s3://$(BUCKET_NAME)

	# Create databases for each agent
	cd deploy && \
	cargo build --bin init_db && \
	CONFIG_FILE_BUCKET=$(BUCKET_NAME) \
	CONFIG_FILE_KEY=ecosystem-config.yaml \
	DB_RDS_CLUSTERARN=$(DB_CLUSTER_ARN) \
	DB_RDS_SECRETARN=$(DB_SECRET_ARN) \
	DB_RDS_DBINSTANCE=$(DB_INSTANCE) \
	cargo run --bin init_db


account_management:
	# cd agents/bank && \
	# cargo lambda build --release --target aarch64-unknown-linux-gnu 

	# sam deploy \
	# 	--stack-name bank-1 \
	# 	--template-file "deploy/bank_agent_template.yaml" \
	# 	--parameter-overrides \
	# 		DBClusterStackName="ecosystem-database"

order_card:
	echo "Not implemented yet ..."

transaction:
	echo "Not implemented yet ..."
