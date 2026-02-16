.PHONY: gen_backend run_db gen_db

gen_backend:
	openapi-generator generate -i swagger.json -g rust-server -o backend/crates/openapi
	# openapi-generator generate -i swagger.json -g rust-server -o backend
run_db:
	docker run --name mysql -e MYSQL_ROOT_PASSWORD=MYSQL_ROOT_PASSWORD -d -p 3306:3306 mysql:8
gen_db:
	cd backend/crates/db_model && diesel setup && diesel migration run && cd -
