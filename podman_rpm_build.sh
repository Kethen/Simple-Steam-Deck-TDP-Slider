podman run \
	-it --rm \
	-v ./:/workdir \
	-w /workdir \
	tumbleweed \
	bash -c '

zypper -n install cargo cmake gcc-c++ fontconfig-devel rpm-build

cp /workdir/rpmmacros ~/.rpmmacros
rpmbuild -bb deck-tdp-slider.spec
'
