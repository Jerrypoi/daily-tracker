.PHONY: gen_backend frontend_install frontend_dev frontend_build frontend_generate_api

gen_backend:
	openapi-generator generate -i swagger.json -g rust-server -o backend

frontend_install:
	npm --prefix frontend install

frontend_dev:
	npm --prefix frontend run dev

frontend_build:
	npm --prefix frontend run build

frontend_generate_api:
	npm --prefix frontend run generate:api

