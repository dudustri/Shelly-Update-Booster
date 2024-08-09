# Shelly-Update-Booster

Shelly Data Frequency Update Booster

## Script Deploy and Execution

To deploy the script in your Shelly EM Device you need to be connected to its WiFi STA and execute the deplot_script.sh file as follows:

```bash
.\deploy_and_execute.sh
```

## Script Stop and Deletion

The script id will be printed in the terminal and a new environment variable will be created. If you want to stop the script you can call:

```bash
.\stop_and_delete.sh {your_script_id}
```

You can call without arguments if you have the environment variable set. You can check it with the following command:

```bash
printenv | grep 'SHELLY_SCRIPT_ID'
```
