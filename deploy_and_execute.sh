#!/bin/bash

export SHELLY_ADDR="192.168.33.1"

#create a script slot in shelly device
SHELLY_SCRIPT_ID=$(curl -X POST -d '{"id":1, "method":"Script.Create", "params":{"name":"update_frequency_booster"}}' http://${SHELLY_ADDR}/rpc)

echo "Your script id is: '${SHELLY_SCRIPT_ID}'"

#upload the script in chunks
python put_script.py ${SHELLY_ADDR} "$SHELLY_SCRIPT_ID" "main.js"

#Run the script
curl -X POST -d '{"id":1, "method":"Script.Start", "params":{"id" '"${SHELLY_SCRIPT_ID}"'}' http://${SHELLY_ADDR}/rpc