.PHONY: setup account_management order_card transaction

BUCKET_NAME := $(shell aws cloudformation describe-stacks --stack-name ecosystem-database --query "Stacks[0].Outputs[?ExportName=='ecosystem-database-EcosystemConfigBucketName'].OutputValue" --output text) 

setup:
	sam deploy\
		--stack-name ecosystem-database \
		--template-file "deploy/aurora_cluster_template.yaml" 

	aws s3 cp config/ecosystem-config.yaml s3://$(BUCKET_NAME)


account_management:
	cd agents/bank && \
	cargo lambda build --release --target aarch64-unknown-linux-gnu 

	sam deploy \
		--stack-name bank-1 \
		--template-file "deploy/bank_agent_template.yaml" \
		--parameter-overrides \
			DBClusterStackName="ecosystem-database"

order_card:
	echo "Not implemented yet ..."

transaction:
	echo "Not implemented yet ..."
