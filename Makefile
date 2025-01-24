.PHONY: account_management_light account_management_full order_card transaction


account_management_light:
# TODO
	aws s3 cp config/ecosystem.yaml s3://my-config-bucket/
	cd agents/bank && \
	cargo lambda build --release --target aarch64-unknown-linux-gnu 
	. "./deploy/bank_no_db_cfg.sh" && \
	sam deploy \ 
		--no-confirm-changeset \
		--stack-name bank-1 \
		--template-file "deploy/bank_no_db_template.yaml" \
		--parameter-overrides \
			LambdaLogLevel="$log_level" \
			DBClusterArn="$cluster_arn" \
			DBSecretArn="$secret_arn" \
			DBInstance="$db_instance"


account_management_full:
	echo "Not implemented yet ..."

order_card:
	echo "Not implemented yet ..."

transaction:
	echo "Not implemented yet ..."
