./deploy.sh
[ ! -f .env ] || export $(grep -v '^#' .env | xargs)
ssh -t ${PI_USERNAME}@${PI_HOSTNAME} "./music-player"
