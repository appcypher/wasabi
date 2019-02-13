#!/bin/sh

# PATHS
# Get current working directory
current_dir=`pwd`

# Get the absolute path of where script is running from
script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd)"
script_path="$script_dir/setup.sh"

# RETURN VARIABLE
ret=""

# ARGUMENTS
args="${@:2}" # All arguments except the first

# DESCRIPTION:
#	Where execution starts
main() {
    case $1 in
        install )
            install
        ;;
        uninstall )
            uninstall
        ;;
        --help|help|-h )
            help
        ;;
    esac

    exit 0
}

# DESCRIPTION:
#	Prints helpful information about the setup script
help() {
    echo ""
    echo "=============================================================="
    echo "====================== WASABI MANAGER ========================"
    echo "=============================================================="
    echo "[USAGE] : wasabi [comand]"
    echo "[COMMAND] :"
    echo " > help               - prints this help message"
    echo " > install            - builds project and exposes relevant commands"
    echo " > uninstall          - removes build files and commands"
    echo "=============================================================="
    echo ""
}

# TODO: Refactor
# DESCRIPTION:
#	Installs the wasabi project
install() {
    local wasmception_dir="$script_dir/deps/wasmception"

    #--------------------------------------------------

    displayln "build dependecies"
    # Cd into wasmception directory
    cd $wasmception_dir
    # Build wasmception project
    make
    # Cd back into project rood directory
    cd $script_dir

    #--------------------------------------------------

    # TODO: Seperate release build.
    displayln "build wasabi project"
    # Build cargo project.
    cargo build

    #--------------------------------------------------

    displayln "make wasabi commands accessible system-wide"
    # Make setup script executable
    chmod u+x $script_path
    # Add links to commands in /usr/local/bin
    add_link "wacc" "$script_dir/target/debug/wacc"
    add_link "wa++" "$script_dir/target/debug/waxx"
    add_link "wasabi" "$script_path"
}

# TODO: Refactor
# DESCRIPTION:
#	Uninstalls the wasabi project
uninstall() {
    local wasmception_dir="$script_dir/deps/wasmception"

    if confirm "uninstall wasabi"; then
        echo "Exiting"
        exit 0
    fi

    # TODO
    #---------------- Remove built deps ---------------
    #---------------- Remove cargo build --------------
    #--------------------------------------------------

    displayln "remove command links"
    remove_link "wacc"
    remove_link "wa++"
    remove_link "wasabi"
}

# DESCRIPTION:
#	Adds a symbolic link to files in `/usr/local/bin`
add_link() {
    if [[ -z $1 ]]; then
        echo "You need to specify link name!"
        exit 1
    fi

    if [[ -z $2 ]]; then
        echo "You need to specify the file you want to link to!"
        exit 1
    fi

    # displayln "Add a link to specified file in /usr/local/bin"
    ln -s $2 /usr/local/bin/$1
}


# DESCRIPTION:
#   Removes a symbolic link from `/usr/local/bin`
remove_link() {
    if [[ -z $1 ]]; then
        echo "You need to provide the symbolic file to delete!"
        exit 1
    fi

    # displayln "Check that file is a link"
    if [[ ! -L "/usr/local/bin/$1" ]]; then
        echo "What you specified is not a symbolic link!"
        exit 1
    fi

    # displayln "Remove link `/usr/local/bin`"
    rm /usr/local/bin/$1
}


# DESCRIPTION:
#	Displays a message
displayln() {
    printf "\n::: $1 :::\n"
}

# DESCRIPTION:
#	Asks the user for confirmation befor proceeding
confirm() {
	printf "\n::: Are you sure you want to $1? [Y/n] "

	read response

	if [[ $response = "Y" ]]; then
		return 1
	else
		return 0
	fi
}
