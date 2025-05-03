# Build Flatpak
Download all the files inside packages/flatpak and put them in your build folder, then run:
"flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir com.github.AeEn123.RoExtract.yml"
or if you want to build a .flatpak file:
"flatpak build-bundle repo RoExtract-linux.flatpak com.github.AeEn123.RoExtract --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo"

