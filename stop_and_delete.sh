#!/bin/bash

#check env var and user input
if [ -z "$1" ]; then 
    if [ "$SHELLY_SCRIPT_ID" -eq 0 ]; then
        echo "The script id needs to be inputed"
        exit 1
    fi
    SCRIPT_ID=$SHELLY_SCRIPT_ID
else    
SCRIPT_ID=$1
fi

#stop the script
curl -X POST -d "{\"id\":1, \"method\":\"Script.Stop\", \"params\":{\"id\":\"${SCRIPT_ID}\"}}" http://"${SHELLY_ADDR}"/rpc

#delete the script
curl -X POST -d "{\"id\":1, \"method\":\"Script.Delete\", \"params\":{\"id\":\"${SCRIPT_ID}\"}}" http://"${SHELLY_ADDR}"/rpc