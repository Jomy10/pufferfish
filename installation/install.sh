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
    echo "tar_dir: $TAR_FILE"

    EXTRACTED_NAME="pufferfish/"
    EXTRACTED_PATH="$BASE_PATH$EXTRACTED_NAME"
    echo "Extracted: $EXTRACTED_PATH"
    
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
    echo "linux"
elif [[ $OS == "windows" ]]
then
    echo "windows is currently not supported with install script, please install manually"
fi

echo "Done."