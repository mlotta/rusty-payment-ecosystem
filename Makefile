.PHONY: setup_db account_management order_card transaction



setup:
	sam deploy\
		--stack-name agent-database \
		--template-file "deploy/aurora_cluster_template.yaml" 

	BUCKET_NAME=$(aws cloudformation describe-stacks --stack-name agent-database --query 'Stacks[0].Outputs[0].OutputValue' | jq -r .) && \
	aws s3 cp config/ecosystem-config.yaml s3://${BUCKET_NAME}


account_management:
	sam deploy \
		--stack-name bank-1 \
		--template-file "deploy/bank_agent_template.yaml" \
		--parameter-overrides \
			DBClusterStackName="agent-database"
	# cd agents/bank && \
	# cargo lambda build --release --target aarch64-unknown-linux-gnu 
	# . "./deploy/bank_no_db_cfg.sh" && \
	# sam deploy \ 
	# 	--no-confirm-changeset \
	# 	--stack-name bank-1 \
	# 	--template-file "deploy/bank_no_db_template.yaml" \
	# 	--parameter-overrides \
	# 		LambdaLogLevel="$log_level" \
	# 		DBClusterArn="$cluster_arn" \
	# 		DBSecretArn="$secret_arn" \
	# 		DBInstance="$db_instance"


order_card:
	echo "Not implemented yet ..."

transaction:
	echo "Not implemented yet ..."
