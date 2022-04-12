use crate::printtr;

pub fn help() {
    printtr!("Usage:");
    printtr!("    paru");
    printtr!("    paru <operation> [...]");
    printtr!("    paru <package(s)>");
    println!();
    printtr!("Pacman operations:");
    printtr!("    paru {-h --help}");
    printtr!("    paru {-V --version}");
    printtr!("    paru {-D --database}    <options> <package(s)>");
    printtr!("    paru {-F --files}       [options] [package(s)]");
    printtr!("    paru {-Q --query}       [options] [package(s)]");
    printtr!("    paru {-R --remove}      [options] <package(s)>");
    printtr!("    paru {-S --sync}        [options] [package(s)]");
    printtr!("    paru {-T --deptest}     [options] [package(s)]");
    printtr!("    paru {-U --upgrade}     [options] [file(s)]");
    println!();
    printtr!("New operations:");
    printtr!("    paru {-P --show}        [options]");
    printtr!("    paru {-G --getpkgbuild} [package(s)]");
    println!();
    printtr!("If no arguments are provided 'paru -Syu' will be performed");
    println!();
    printtr!("Options without operation:");
    printtr!("    -c --clean            Remove unneeded dependencies");
    printtr!("       --gendb            Generates development package DB used for updating");
    println!();
    printtr!("New options:");
    printtr!("       --repo              Assume targets are from the repositories");
    printtr!("    -a --aur               Assume targets are from the AUR");
    printtr!("    --aururl    <url>      Set an alternative AUR URL");
    printtr!("    --clonedir  <dir>      Directory used to download and run PKGBUILDs");
    println!();
    printtr!("    --makepkg   <file>     makepkg command to use");
    printtr!("    --mflags    <flags>    Pass arguments to makepkg");
    printtr!("    --pacman    <file>     pacman command to use");
    printtr!("    --git       <file>     git command to use");
    printtr!("    --gitflags  <flags>    Pass arguments to git");
    printtr!("    --sudo      <file>     sudo command to use");
    printtr!("    --sudoflags <flags>    Pass arguments to sudo");
    printtr!("    --asp       <file>     asp command to use");
    printtr!("    --bat       <file>     bat command to use");
    printtr!("    --batflags  <flags>    Pass arguments to bat");
    printtr!("    --gpg       <file>     gpg command to use");
    printtr!("    --gpgflags  <flags>    Pass arguments to gpg");
    printtr!("    --fm        <file>     File manager to use for PKGBUILD review");
    printtr!("    --fmflags   <flags>    Pass arguments to file manager");
    println!();
    printtr!("    --completioninterval   <n> Time in days to refresh completion cache");
    printtr!("    --sortby    <field>    Sort AUR results by a specific field during search");
    printtr!("    --searchby  <field>    Search for packages using a specified field");
    printtr!("    --limit     <limit>    Limits the number of items returned in a search");
    printtr!("    -x --regex             Enable regex for aur search");
    println!();
    printtr!("    --skipreview           Skip the review process");
    printtr!("    --review               Don't skip the review process");
    printtr!("    --[no]upgrademenu      Show interactive menu to skip upgrades");
    printtr!("    --[no]removemake       Remove makedepends after install");
    printtr!("    --[no]cleanafter       Remove package sources after install");
    printtr!("    --[no]rebuild          Always build target packages");
    printtr!("    --[no]redownload       Always download PKGBUILDs of targets");
    println!();
    printtr!("    --[no]pgpfetch         Prompt to import PGP keys from PKGBUILDs");
    printtr!("    --[no]useask           Automatically resolve conflicts using pacman's ask flag");
    printtr!("    --[no]savechanges      Commit changes to pkgbuilds made during review");
    printtr!("    --[no]newsonupgrade    Print new news during sysupgrade");
    printtr!("    --[no]combinedupgrade  Refresh then perform the repo and AUR upgrade together");
    printtr!("    --[no]batchinstall     Build multiple AUR packages then install them together");
    printtr!("    --[no]provides         Look for matching providers when searching for packages");
    printtr!("    --[no]devel            Check development packages during sysupgrade");
    printtr!("    --[no]installdebug     Also install debug packages when a package provides them");
    printtr!("    --[no]sudoloop         Loop sudo calls in the background to avoid timeout");
    printtr!("    --[no]chroot           Build packages in a chroot");
    printtr!("    --[no]failfast         Exit as soon as building an AUR package fails");
    printtr!("    --[no]keepsrc          Keep src/ and pkg/ dirs after building packages");
    printtr!("    --[no]sign             Sign packages with gpg");
    printtr!("    --[no]signdb           Sign databases with gpg");
    printtr!("    --localrepo            Build packages into a local repo");
    printtr!("    --nocheck              Don't resolve checkdepends or run the check function");
    printtr!("    --develsuffixes        Suffixes used to decide if a package is a devel package");
    printtr!("    --bottomup             Shows AUR's packages first and then repository's");
    printtr!("    --topdown              Shows repository's packages first and then AUR's");
    println!();
    printtr!("show specific options:");
    printtr!("    -c --complete         Used for completions");
    printtr!("    -s --stats            Display system package statistics");
    printtr!("    -w --news             Print arch news");
    println!();
    printtr!("getpkgbuild specific options:");
    printtr!("    -p --print            Print pkgbuild to stdout");
    printtr!("    -c --comments         Print AUR comments for pkgbuild");
    printtr!("    -s --ssh              Clone package using SSH");
    println!();
    printtr!("upgrade specific options:");
    printtr!("    -i --install          Install package as well as building");
}
