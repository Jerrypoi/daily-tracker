.PHONY: gen_backend frontend_install frontend_dev frontend_build frontend_generate_api cli_help

gen_backend:
	openapi-generator generate -i swagger.json -g rust-server -o backend

run_backend:
	cd backend && cargo run

frontend_install:
	npm --prefix frontend install

frontend_dev:
	npm --prefix frontend run dev

frontend_build:
	npm --prefix frontend run build

frontend_generate_api:
	npm --prefix frontend run generate:api

cli_help:
	npx --yes github:Jerrypoi/daily-tracker daily-tracker help

