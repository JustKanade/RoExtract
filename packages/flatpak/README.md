### Building Flatpak package
# Building and installing
1. Download everything inside packages/flatpak
2. Make sure you're running the command below in the folder with the files
```
flatpak-builder --force-clean --user --install-deps-from=flathub --repo=repo --install builddir com.github.AeEn123.RoExtract.yml
```
# Building a .flatpak file
1. Do "Building and installing" and go inside the same folder
2. Run the command below
```
flatpak build-bundle repo RoExtract-linux.flatpak com.github.AeEn123.RoExtract --runtime-repo=https://flathub.org/repo/flathub.flatpakrepo
```
