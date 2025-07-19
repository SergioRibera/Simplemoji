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

  standardLibs = [
    "libc.so" "libm.so" "libgcc_s.so" "libstdc++.so" "ld-linux"
    "libdl.so" "libpthread.so" "librt.so" "libutil.so"
    "libgomp.so" "libatomic.so" "libasan.so" "libtsan.so"
  ];

  isStandardLib = path:
    let
      basename = baseNameOf path;
    in
      builtins.any (libname: lib.hasInfix libname basename) standardLibs;

  copyNonStandardLibs = useHome: drv: destDir: let
     dir = if useHome then "$HOME" else "$out";
   in ''
    mkdir -p ${dir}${destDir}lib
    echo "Filtering libraries in ${drv}/lib..."

    if [[ -d ${drv}/lib ]]; then
      find ${drv}/lib -type f -name "*.so*" | while read lib; do
      if ${pkgs.coreutils}/bin/ldd "$lib" >/dev/null 2>&1 && \
        (${pkgs.binutils}/bin/objdump -p "$lib" | grep -q "NEEDED.*libc.so" || \
        ${if isStandardLib "$lib" then "true" else "false"}); then
          echo "Skipping standard library: $lib"
      else
        echo "Copying non-standard library: $lib"
        cp -d "$lib" ${dir}${destDir}lib/
        chmod +w ${dir}${destDir}lib/$(basename "$lib")
        ${pkgs.patchelf}/bin/patchelf --set-rpath '$ORIGIN/../lib' ${dir}${destDir}lib/$(basename "$lib")
      fi
      done
    fi
  '';

  handleSystemd = drv: useHome: meta: destDir: let
     dir = if useHome then "$HOME" else "$out";
   in ''
    ${
      if meta ? systemd then
        if lib.isList meta.systemd then
          lib.concatMapStringsSep "\n" (service: ''
            cp ${service} ${dir}${destDir}$(basename ${service})
            chmod 644 ${dir}${destDir}$(basename ${service})
          '') meta.systemd
        else
          ''
            cp ${meta.systemd} ${dir}${destDir}${drv.pname}.service
            chmod 644 ${dir}${destDir}${drv.pname}.service
          ''
      else ""
    }
  '';

  prepareOptStructure = drv: useHome: let
    dir = if useHome then "$HOME" else "$out";
  in ''
    mkdir -p ${dir}/opt/${pname}/bin
    mkdir -p ${dir}/opt/${pname}/lib

    for bin in ${drv}/bin/*; do
      cp "$bin" ${dir}/opt/${pname}/bin/
      chmod +w ${dir}/opt/${pname}/bin/$(basename "$bin")
      chmod +x ${dir}/opt/${pname}/bin/$(basename "$bin")

      ${pkgs.patchelf}/bin/patchelf \
        --set-rpath '/opt/${pname}/lib:/usr/lib:/lib' \
        ${dir}/opt/${pname}/bin/$(basename "$bin")
    done

    ${copyNonStandardLibs useHome drv "/opt/${pname}/"}
  '';

  compress = ext: drv: let
    meta = packageMeta // (target.packageMeta or {});
  in pkgs.stdenv.mkDerivation {
    dontUnpack = true;
    name = "${pname}-${ext}";
    buildInputs = with pkgs; [ ouch patchelf ];

    buildPhase = ''
      mkdir -p package/opt/${pname}/bin
      mkdir -p package/opt/${pname}/lib
      ${lib.optionalString (target.os == "linux") ''
        mkdir -p package/lib/systemd/system
      ''}

      echo "Copying binaries from ${drv}/bin"
      for bin in ${drv}/bin/*; do
        if [[ -f "$bin" ]]; then
          install -Dm755 "$bin" "package/opt/${pname}/bin/$(basename "$bin")"
        fi
      done

      ${if target.os == "windows" then ''
        find ${drv}/bin -name "*.dll" -exec install -Dm755 {} package/opt/${pname}/bin \;
      '' else ''
        # Linux: copiar y procesar bibliotecas
        ${copyNonStandardLibs false drv "/package/opt/${pname}/"}
      ''}

      if [[ -d "${drv}/resources" ]]; then
        cp -r ${drv}/resources package/opt/${pname}/
      fi

      ${lib.optionalString (target.os == "linux") ''
        ${handleSystemd drv false meta "/package/lib/systemd/system/"}
      ''}
    '';

    installPhase = ''
      mkdir -p $out
      echo "Creating ${ext} archive..."
      ouch compress package/* "$out/${pname}-${meta.version}-${target.arch}-${target.os}.${ext}"
      echo "Package created at $out/${pname}-${meta.version}-${target.arch}-${target.os}.${ext}"
    '';
  };

  deb = drv: let
    meta = packageMeta // (target.packageMeta or {});
  in pkgs.stdenv.mkDerivation {
    dontUnpack = true;
    name = "${target.pname}-${meta.version}-${target.arch}.deb";
    nativeBuildInputs = with pkgs; [ dpkg fakeroot patchelf ];

    buildCommand = ''
      mkdir -p $out/debian-package/DEBIAN
      mkdir -p $out/debian-package/opt/${pname}
      mkdir -p $out/debian-package/lib/systemd/system

      ${prepareOptStructure drv false}

      mv $out/opt/${pname} $out/debian-package/opt/

      ${handleSystemd drv false meta "/debian-package/lib/systemd/system/"}

      cat > $out/debian-package/DEBIAN/control <<EOF
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

      cat > $out/debian-package/DEBIAN/postinst <<EOF
      #!/bin/sh
      ${lib.optionalString (meta ? systemd) "systemctl daemon-reload"}
      EOF

      cat > $out/debian-package/DEBIAN/postrm <<EOF
      #!/bin/sh
      ${lib.optionalString (meta ? systemd) "systemctl daemon-reload"}
      EOF

      chmod +x $out/debian-package/DEBIAN/postinst
      chmod +x $out/debian-package/DEBIAN/postrm

      fakeroot dpkg-deb --build $out/debian-package $out/${target.pname}-${meta.version}-${if target.arch == "x86_64" then "amd64" else target.arch}.deb
    '';
  };

  rpm = drv: let
    meta = packageMeta // (target.packageMeta or {});
  in pkgs.stdenv.mkDerivation {
    dontUnpack = true;
    name = "${target.pname}-${meta.version}-${target.arch}.rpm";
    nativeBuildInputs = with pkgs; [ pkgs.rpm patchelf ];

    buildCommand = ''
      export HOME=$(mktemp -d)
      export buildbase=$HOME/rpmbuild
      export buildroot=$buildbase/BUILDROOT/${target.pname}-${meta.version}-1.${target.arch}
      export RPM_DB_PATH=$HOME/.rpmdb

      mkdir -p $RPM_DB_PATH
      mkdir -p $buildbase/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
      mkdir -p $buildroot/opt/${pname}
      mkdir -p $buildroot/lib/systemd/system

      rpm --initdb --dbpath "$RPM_DB_PATH"

      ${prepareOptStructure drv true}

      mv "$out/opt/${pname}" "$buildroot/opt/${pname}"

      ${handleSystemd drv true meta "/rpmbuild/BUILDROOT/${target.pname}-${meta.version}-1.${target.arch}/lib/systemd/system/"}

      cat > $buildbase/SPECS/${pname}.spec <<EOF
Name: ${pname}
Version: ${meta.version}
Release: 1%{?dist}
Summary: ${meta.description}
License: Proprietary
URL: ${meta.homepage}
Group: ${meta.rpm.group or "Applications/System"}
BuildArch: ${target.arch}
${lib.optionalString (meta.rpm.requires != []) "Requires: ${lib.concatStringsSep ", " meta.rpm.requires}\n"}\
${lib.optionalString ((meta.rpm.recommends or []) != []) "Recommends: ${lib.concatStringsSep ", " (meta.rpm.recommends or [])}\n"}\

%description
${meta.description}

%files
%attr(0755, root, root) /opt/${pname}/*
${lib.optionalString (meta ? systemd) "%attr(0644, root, root) /lib/systemd/system/*"}

%post
${lib.optionalString (meta ? systemd && meta.systemd != []) "systemctl daemon-reload >/dev/null 2>&1 || :"}

%postun
${lib.optionalString (meta ? systemd && meta.systemd != []) "systemctl daemon-reload >/dev/null 2>&1 || :"}

%clean
rm -rf \$RPM_BUILD_ROOT
EOF

      rpmbuild \
        --define "_topdir $buildbase" \
        --dbpath "$RPM_DB_PATH" \
        --target ${target.arch} \
        -bb $buildbase/SPECS/${pname}.spec

      mkdir -p $out/rpm
      cp $buildbase/RPMS/${target.arch}/${target.pname}-${meta.version}-1.${target.arch}.rpm $out/rpm/
    '';
  };
in
  if format == "deb" then
    deb drv
  else if format == "rpm" then
    rpm drv
  else
    compress format drv
