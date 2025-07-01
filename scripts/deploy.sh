#!/bin/bash

sshpass -p ${SFTP_PASSWORD} sftp -o StrictHostKeyChecking=no sftp://${SFTP_USER}@${SFTP_HOST}:${SFTP_PORT} <<EOL
cd ${REPO_PATH}
put ${PACKAGE_NAME}.aarch64-gnu
EOL

exit $?