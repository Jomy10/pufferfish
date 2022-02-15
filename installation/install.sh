echo "Installing pufferfish for your operating system..."

OS="unknown"
case "$OSTYPE" in
    linux*) OS="linux" ;;
    darwin*) OS="darwin" ;;
    msys*) OS="windows" ;;
    *) 
        echo "Unsuported OS type"
        exit 1
        ;;
esac

# Get the download url for the specific platform
URL=$(curl "https://api.github.com/repos/jomy10/pufferfish/releases/latest" \
    | grep "browser_download_url" \
    | grep "$OS" \
    | grep -v "sha256sum" \
    | sed 's/"browser_download_url": //' \
    | sed 's/"//' \
    | sed 's/"//' \
    | tr -d '[:space:]'
)

echo "Downloading pufferfish from $URL"

if [[ $OS == "darwin" ]]
then
    BASE_FOLDER="be.jonaseveraert.pufferfish/"
    BASE_PATH="$TMPDIR$BASE_FOLDER"
    mkdir $BASE_PATH

    TAR_NAME="pufferfish.tar.gz"
    TAR_FILE="$BASE_PATH$TAR_NAME"

    EXTRACTED_NAME="pufferfish/"
    EXTRACTED_PATH="$BASE_PATH$EXTRACTED_NAME"
    
    mkdir $EXTRACTED_PATH

    curl -L "$URL" --output "$TAR_FILE"
    echo "File downloaded to $TAR_FILE"
    tar -xf "$TAR_FILE" -C "$EXTRACTED_PATH"

    EXEC_NAME="puf"
    mv "$EXTRACTED_PATH$EXEC_NAME" "/usr/local/bin/puf"

    echo "Cleaning up"

    rm -r "$BASE_PATH"
elif [[ $OS == "linux" ]]
then
    echo "\x1b[36mHi, I do not own a Linux computer. \nIf after this installation you can type \"puf\" in the terminal and get output, please let me know through either opening an issue, or opening a pull request and removing this line in the install script.\nIf it doesn't work, please open an issue with the terminal output and I'll try to fix this!\x1b[0m"

    TMPDIR="/var/tmp"
    BASE_FOLDER="be.jonaseveraert.pufferfish/"
    BASE_PATH="$TMPDIR$BASE_FOLDER"
    mkdir $BASE_PATH

    TAR_NAME="pufferfish.tar.gz"
    TAR_FILE="$BASE_PATH$TAR_NAME"

    EXTRACTED_NAME="pufferfish/"
    EXTRACTED_PATH="$BASE_PATH$EXTRACTED_NAME"
    
    mkdir $EXTRACTED_PATH

    curl -L "$URL" --output "$TAR_FILE"
    echo "File downloaded to $TAR_FILE"
    tar -xf "$TAR_FILE" -C "$EXTRACTED_PATH"

    EXEC_NAME="puf"
    mv "$EXTRACTED_PATH$EXEC_NAME" "/usr/local/bin/$EXEC_NAME"

    echo "Cleaning up"

    rm -r "$BASE_PATH"
elif [[ $OS == "windows" ]]
then
    echo "\x1b[36mHi, I do not own a Windows computer. \nIf after this installation you can type \"puf\" in the terminal and get output, please let me know through either opening an issue, or opening a pull request and removing this line in the install script.\nIf it doesn't work, please open an issue with the terminal output and I'll try to fix this!\nNOTE: you might want to try typing \"puf.exe\" in the terminal? I'm really not sure how this works on Windows.\x1b[0m"

    TMPDIR="\Users\<username>\AppData\Local\Temp\\"
    BASE_FOLDER="be.jonaseveraert.pufferfish\\"
    BASE_PATH="$TMPDIR$BASE_FOLDER"
    mkdir $BASE_PATH

    ZIP_NAME="pufferfish.tar.gz"
    ZIP_FILE="$BASE_PATH$TAR_NAME"

    EXTRACTED_NAME="pufferfish\\"
    EXTRACTED_PATH="$BASE_PATH$EXTRACTED_NAME"

    mkdir $EXTRACTED_PATH

    curl -L "$URL" --output "$ZIP_FILE"
    echo "File downloaded to $ZIP_FILE"
    unzip -d "$EXTRACTED_PATH" "$ZIP_FILE"

    EXEC_NAME="puf.exe"
    mv "$EXTRACTED_PATH$EXEC_NAME" "\Windows\System32\$EXEC_NAME"
fi

echo "Done."