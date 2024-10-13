# default.nix

# Must restart VSCode to make this work
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [ 
        pkg-config #Needed by node to handle dependencies
        openssl  #Needed by node?
        openssl.dev #Needed by rust
        dbus
        glib 
        gtk3 
        libsoup_3
        webkitgtk_4_1
        appimagekit
    ];
}

{
    environment.systemPackages = with pkgs; [
        gcc
        #clang #c language tools for rust; should only need gcc or clang?
	    llvmPackages.bintools #llvm package for rust
        openssl
        openssl.dev
    ]
}