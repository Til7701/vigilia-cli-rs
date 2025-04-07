UID=$(id -u)
GID=$(id -g)

docker run --rm \
  -u "${UID}":"${GID}" \
  -v "${PWD}":/local openapitools/openapi-generator-cli:v7.12.0 generate \
  -i https://raw.githubusercontent.com/schlunzis/vigilia/ba7e6e18e0a5a39078791259ed41e8d3949d5dc7/openapi.yaml \
  -g rust \
  --global-property models,apis \
  -o /local
