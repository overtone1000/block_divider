# default.nix

# Must restart VSCode to make this work
with import <nixpkgs> {};
stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [ 
        pkg-config #Needed by rust to find other packges. node too?
        openssl  #Needed by rust networking crates. node too?
        postgresql #Needed by rust diesel crate with postgres feature. Just needs libpq, but no such package is currently available on Nix.
        #dbus #Needed by tauri?
        #glib #Needed by tauri? 
        #gtk3 #Needed by tauri?
        #libsoup_3 #Needed by tauri?
        #webkitgtk_4_1 #Needed by tauri?
        #appimagekit #Needed by tauri?
    ];
}