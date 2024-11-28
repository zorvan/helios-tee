docker system prune -f
docker builder prune -f

read -p "Building version [>= v0.1.0] : " codever

CODEVER=${codever:-v0.1.0}

docker build --rm --no-cache \
    -t helios:$CODEVER \
    -t helios:latest \
    --build-arg UBUNTU_VERSION=22.04 \
    --build-arg CODE_VERSION=$CODEVER \
    .
