let connSettings = {
    topic: 'shellyUpdateBooster/',
    qos: 1,
    retain: false,
    postUrl: '',
    updateTime: 500
};

let timerId;

function checkSampleChange() {
    timerId = Timer.set(connSettings.updateTime, true, publishNewSample, null);
}

function publishNewSample() {
    let timestamp = Shelly.getComponentStatus("sys").unixtime;
    let deviceStatus = Shelly.getComponentStatus('em', 0);
    deviceStatus.timestamp = timestamp;
      
    if (MQTT.isConnected()) {
        MQTT.publish(connSettings.topic, JSON.stringify(deviceStatus), connSettings.qos, connSettings.retain);
    } else if (connSettings.postUrl !== '') {
        print('http triggered');
        Shelly.call('HTTP.POST', {url: connSettings.postUrl, body: JSON.stringify(deviceStatus)}, function(result) {
            print(JSON.stringify(result));
        });
    } else {
        print("Connection problems... Clearing the timer and killing the script process.");
        Timer.clear(timerId);
        die("No connection stabilished!");
    }
}

checkSampleChange();
print(JSON.stringify(connSettings));