let connSettings = {
    topic: "shellyUpdateBooster/",
    qos: "1",
    retain: "false",
    postUrl: ""
}

// TODO: add an event handler to call publish NewSample instead of an infinite loop.
function checkSampleChange(){
    while(0){

    }
}

function publishNewSample() {

    let deviceStatus = Shelly.getComponentStatus("em", 0);
    if (MQTT.isConnected()){
        print("mqtt triggered");
        MQTT.publish(connSettings.topic, deviceStatus, connSettings.qos, connSettings.retain);
    }
    else if (!config.post_url){
        print("http triggered");
        Shelly.call("HTTP.POST", {url: connSettings.postUrl}, deviceStatus);
    }
    print(deviceStatus);
}

print (connSettings);