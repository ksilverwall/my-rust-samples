#!/bin/bash
if [ ! -n "${DATA_DIR}" ]; then
  echo DATA_DIR is not set
  exit -1
fi

if [ ! -d "${DATA_DIR}" ]; then
  echo ${DATA_DIR} is not found, initialize by ${GENESIS_FILE}
  /opt/scripts/setup.sh ${DATA_DIR}
fi

geth --datadir "${DATA_DIR}" $@