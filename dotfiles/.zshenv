typeset -U path
PYENV_ROOT=~/.pyenv/
EDITOR=/bin/vim
LC_ALL="$LANG"
TERM=xterm-256color
K3D_FIX_MOUNTS=1
QML_IMPORT_PATH=/usr/lib/qt6/qml
# Gcloud
# CLOUDSDK_ROOT_DIR=/opt/google-cloud-cli
# CLOUDSDK_PYTHON=/usr/bin/python
# CLOUDSDK_PYTHON_ARGS='-S -W ignore'
# GOOGLE_CLOUD_SDK_HOME=/opt/google-cloud-cli
###
path=($path[@] ~/.poetry/bin/ ~/.local/bin/ ~/.pyenv/bin ~/.cargo/bin ~/.krew/bin /opt/sonar-scanner/bin ~/.go/bin /opt/google-cloud-cli/bin)
fpath=($fpath[@] ~/.zfunc)
