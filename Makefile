.PHONY: gen_backend

gen_backend:
	openapi-generator generate -i swagger.json -g rust-server -o backend
