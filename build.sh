SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

DISPLAY="" pyinstaller --clean -y $SCRIPT_DIR/RailOSConsist.spec
