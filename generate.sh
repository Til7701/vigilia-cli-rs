UID=$(id -u)
GID=$(id -g)

docker run --rm \
  -u "${UID}":"${GID}" \
  -v "${PWD}":/local openapitools/openapi-generator-cli:v7.12.0 generate \
  -i https://raw.githubusercontent.com/schlunzis/vigilia/d668156de5649769954678d6d1a44d3567d12aa0/openapi.yaml \
  -g rust \
  -o /local/target/openapi
