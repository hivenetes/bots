
for V in 0.0.3 0.0.4 0.0.5 0.0.6
do
    pushd pkg && docker buildx build --build-arg=VERSION=$V --platform linux/amd64,linux/arm64,linux/arm . -t cnskunkworks/cats:$V --push && popd
done

