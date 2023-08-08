Name:           deck-tdp-slider
Version:        0.1
Release:        0
License:        MIT
Summary:        Simple slider gui for adjusting steamdeck TDP
BuildRoot:      %{_tmppath}/%{name}-%{version}-build
Source0: .
BuildRequires: cargo
BuildRequires: cmake
BuildRequires: gcc-c++
BuildRequires: fontconfig-devel


%description
Tweaks for running opensuse on the steam deck, includes saving and restoring TDP, setting /sys node permissions and a gamescope session

%build
cd %{SOURCE0}
cargo build -r

%install
mkdir -p %{buildroot}/usr/bin/
mkdir -p %{buildroot}/usr/share/applications
cp %{SOURCE0}/target/release/deck_tdp_slider %{buildroot}/usr/bin/
cp %{SOURCE0}/deck_tdp_slider.desktop %{buildroot}/usr/share/applications/

%clean
cd %{SOURCE0}
rm -r target

%files
%defattr(-,root,root)
/usr/bin/deck_tdp_slider
/usr/share/applications/deck_tdp_slider.desktop

%changelog
* Tue Aug 08 2023 Katharine Chui <kwchuiaa@connect.ust.hk> - 0.1
- nothing for now
