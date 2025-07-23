{ pkgs, lib, format, target, pname, drv, ... }: let
  packageMeta = {
    description = "";
    version = "1.0.0";
    maintainer = "Sergio Ribera <support@sergioribera.rs>";
    homepage = "";
    deb = {
      depends = ["libc6"];
      recommends = [];
      section = "utils";
      priority = "optional";
    };
    rpm = {
      requires = ["glibc"];
      recommends = [];
      group = "Development/Tools";
    };
    systemd = []; # or []
  };

  copyNonStandardLibs = drv: destDir: ''
    echo "Filtering libraries in ${drv}/lib..."

    standard_libs=(
      "libc.so" "libm.so" "libgcc_s.so" "libstdc++.so" "ld-linux"
      "libdl.so" "libpthread.so" "librt.so" "libutil.so"
      "libgomp.so" "libatomic.so" "libasan.so" "libtsan.so"
    )

    for lib in "$(cat ${pkgs.referencesByPopularity drv})"
    do
      if ! [[ -d "$lib/lib" ]]; then
        continue
      fi

      if ! [[ -d "${destDir}lib" ]]; then
        mkdir -p ${destDir}lib
      fi

      for std_lib in "$\{standard_libs[@]}"; do
      if [[ "$libname" == *"$std_lib"* ]]; then
          is_standard=1
          break
      fi
      done

      if [[ $is_standard -eq 0 ]]; then
        echo "Copying non-standard library: $lib"
        cp -r $lib/lib ${destDir}lib/$(basename "$lib")
      fi
      # ${pkgs.patchelf}/bin/patchelf --set-rpath '$ORIGIN/../lib' ${destDir}lib/$(basename "$lib")
    done
  '';

  prepareOptStructure = drv: destDir: ''
    mkdir -p ${destDir}bin
    mkdir -p ${destDir}lib

    for bin in ${drv}/bin/*; do
      install -Dm755 "$bin" "${destDir}bin/$(basename "$bin")"

      if ! [ "${target.os}" == "windows" ]; then
        ${pkgs.patchelf}/bin/patchelf \
            --set-interpreter "/usr/lib/ld-linux-${pkgs.lib.strings.replaceStrings ["_"] ["-"] target.arch}.so.2" \
            --set-rpath "\$ORIGIN/../lib:/usr/lib:/lib" \
            "${destDir}bin/$(basename "$bin")"
      fi
    done
  '';

  compress = ext: drv: let
    meta = packageMeta // (target.packageMeta or {});
    path = if target.os != "windows" then "package/opt/${pname}/" else "";
    fromPath = if target.os != "windows" then "package/*" else "bin/* lib/*";
  in pkgs.stdenv.mkDerivation {
    dontUnpack = true;
    name = "${pname}-${ext}";
    buildInputs = with pkgs; [ ouch patchelf ];

    buildPhase = ''
      ${lib.optionalString (target.os != "windows") ''
        mkdir -p ${path}{bin,lib}
      ''}

      ${prepareOptStructure drv path}

      if [[ -d "${drv}/resources" ]]; then
        cp -r ${drv}/resources ${path}
      fi

      ${copyNonStandardLibs drv path}
    '';

    installPhase = ''
      mkdir -p $out
      ouch compress ${fromPath} "$out/${pname}-${meta.version}-${target.arch}-${target.os}.${ext}"
    '';
  };

  deb = drv: let
    meta = packageMeta // (target.packageMeta or {});
  in pkgs.stdenv.mkDerivation {
    dontUnpack = true;
    name = "${target.pname}-${meta.version}.deb";
    nativeBuildInputs = with pkgs; [ dpkg fakeroot patchelf ];

    buildCommand = ''
      mkdir -p debian-package/DEBIAN
      mkdir -p $out/debian-package/opt/${pname}
      mkdir -p debian-package/lib/systemd/system

      ${prepareOptStructure drv "debian-package/opt/${pname}/" }

      cat > debian-package/DEBIAN/control <<EOF
      Package: ${target.pname}
      Version: ${meta.version}
      Architecture: ${if target.arch == "x86_64" then "amd64" else target.arch}
      Maintainer: ${meta.maintainer}
      Description: ${meta.description}
      Homepage: ${meta.homepage}
      Section: ${meta.deb.section or "utils"}
      Priority: ${meta.deb.priority or "optional"}
      ${lib.optionalString (meta.deb.depends != []) "Depends: ${lib.concatStringsSep ", " meta.deb.depends}\n"}
      ${lib.optionalString ((meta.deb.recommends or []) != []) "Recommends: ${lib.concatStringsSep ", " (meta.deb.recommends or [])}\n"}
      EOF

      cat > debian-package/DEBIAN/postinst <<EOF
      #!/bin/sh
      ${lib.optionalString (meta ? systemd) "systemctl daemon-reload"}
      EOF

      cat > debian-package/DEBIAN/postrm <<EOF
      #!/bin/sh
      ${lib.optionalString (meta ? systemd) "systemctl daemon-reload"}
      EOF

      mkdir -p $out

      mv debian-package $out/debian-package

      chmod +x $out/debian-package/DEBIAN/postinst
      chmod +x $out/debian-package/DEBIAN/postrm

      fakeroot dpkg-deb --build "$out/debian-package" "$out/${target.pname}-${meta.version}.deb"
    '';
  };

  msi = drv: let
    meta = packageMeta // (target.packageMeta or {});
  in pkgs.stdenv.mkDerivation {
  };
in
  if format == "deb" then
    deb drv
  else if format == "msi" then
    msi drv
  else
    compress format drv
