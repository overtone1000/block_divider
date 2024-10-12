# default.nix
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [ 
        pkg-config #Needed by node to handle dependencies
        openssl  #Needed by node?
        #dbus 
        #glib 
        #gtk3 
        #libsoup_3
        #webkitgtk_4_1
        #appimagekit 
    ];
}