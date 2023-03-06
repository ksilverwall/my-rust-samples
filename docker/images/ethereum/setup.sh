#!/bin/bash
for I in "${@}"; do
  echo === setup $I ===
  rm -rf $I
  mkdir -p $I || exit -1
  geth init --datadir $I ${GENESIS_FILE:-./files/shared/genesis.json} || exit -1
done
