#!/bin/bash
#
# Copyright 2016 Palantir Technologies
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# <http://www.apache.org/licenses/LICENSE-2.0>
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#

is_process_active() {
   local PID=$1
   ps $PID > /dev/null;
   echo $?
}

is_process_service() {
  local PID=$1
  local SERVICE_NAME=$2
  # trailing '=' prevents a header line
  ps -o command= $PID | grep -q "$SERVICE"
  return $?
}

# Everything in this script is relative to the base directory of an SLSv2 distribution
pushd "`dirname \"$0\"`/../.." > /dev/null

BIN="@bin@"
START_ARGS=(@start_args@)
CHECK_ARGS=(@check_args@)

ACTION=$1
SCRIPT_DIR="service/bin"
SERVICE="@service@"
PIDFILE="var/run/$SERVICE.pid"

case $ACTION in
start)
    if service/bin/init.sh status &> /dev/null; then
        echo "Process is already running"
        exit 0
    fi
    printf "%-50s" "Running '$SERVICE'..."

    # ensure log and pid directories exist
    mkdir -p "var/log"
    mkdir -p "var/run"
    PID=$($BIN "${START_ARGS[@]}" > var/log/$SERVICE-startup.log 2>&1 & echo $!)
    # always write $PIDFILE so that `init.sh status` for a service that crashed when starting will return 1, not 3
    echo $PID > $PIDFILE
    sleep 1
    if is_process_service $PID $SERVICE; then
        printf "%s\n" "Started ($PID)"
        exit 0
    else
        printf "%s\n" "Failed"
        exit 1
    fi
;;
status)
    printf "%-50s" "Checking '$SERVICE'..."
    if [ -f $PIDFILE ]; then
        PID=$(cat $PIDFILE)
        if is_process_service $PID $SERVICE; then
            printf "%s\n" "Running ($PID)"
            exit 0
        elif is_process_active $PID; then
            printf "%s\n" "Warning, Pid $PID appears to not correspond to service $SERVICE"
            # fallthrough to generic 'process dead but pidfile exists'
        fi

        printf "%s\n" "Process dead but pidfile exists."
        exit 1
    else
        printf "%s\n" "Service not running"
        exit 3
    fi
;;
stop)
    printf "%-50s" "Stopping '$SERVICE'..."
    if service/bin/init.sh status &> /dev/null; then
        PID=$(cat $PIDFILE)
        kill $PID
        COUNTER=0
        while is_process_service $PID $SERVICE && [ "$COUNTER" -lt "240" ]; do
            sleep 1
            let COUNTER=COUNTER+1
            if [ $((COUNTER%5)) == 0 ]; then
                if [ "$COUNTER" -eq "5" ]; then
                    printf "\n" # first time get a new line to get off Stopping printf
                fi
                printf "%s\n" "Waiting for '$SERVICE' ($PID) to stop"
            fi
        done
        if is_process_service $PID $SERVICE; then
            printf "%s\n" "Failed"
            exit 1
        else
            rm -f $PIDFILE
            printf "%s\n" "Stopped ($PID)"
            exit 0
        fi
    else
        rm -f $PIDFILE
        printf "%s\n" "Service not running"
        exit 0
    fi
;;
console)
    if service/bin/init.sh status &> /dev/null; then
        echo "Process is already running"
        exit 1
    fi
    trap "service/bin/init.sh stop &> /dev/null" SIGTERM EXIT
    mkdir -p "$(dirname $PIDFILE)"

    $BIN "${START_ARGS[@]}" &
    echo $! > $PIDFILE
    wait
;;
restart)
    service/bin/init.sh stop
    service/bin/init.sh start
;;
check)
    printf "%-50s" "Checking health of '$SERVICE'..."
    $BIN "${CHECK_ARGS[@]}" > var/log/$SERVICE-check.log 2>&1
    RESULT=$?
    if [ $RESULT -eq 0 ]; then
        printf "%s\n" "Healthy"
        exit 0
    else
        printf "%s\n" "Unhealthy"
        exit $RESULT
    fi
;;
*)
    # Support arbitrary additional actions; e.g. init-reload.sh will add a "reload" action
    if [[ -f "$SCRIPT_DIR/init-$ACTION.sh" ]]; then
        export LAUNCHER_CMD
        shift
        /bin/bash "$SCRIPT_DIR/init-$ACTION.sh" "$@"
        exit $?
    else
        COMMANDS=$(ls $SCRIPT_DIR | sed -ne '/init-.*.sh/ { s/^init-\(.*\).sh$/|\1/g; p; }' | tr -d '\n')
        echo "Usage: $0 {status|start|stop|console|restart|check${COMMANDS}}"
        exit 1
    fi
esac
