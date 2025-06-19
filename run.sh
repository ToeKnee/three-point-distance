#!/bin/bash

function measure_memory {
    if [ -z "$1" ]; then
        echo "Usage: measure_memory <command>"
        return 1
    fi

    # Run the command and capture its PID
    $1 &
    CMD_PID=$!
    CONTINUE=true
    MAX_MEM=0
    while $CONTINUE; do
        top -p $CMD_PID -b -n1 | grep $CMD_PID > /dev/null 2>&1
        if [ $? -ne 0 ]; then
            CONTINUE=false
        fi
        MEM_USAGE=$(top -p $CMD_PID -b -n1 | grep $CMD_PID | awk '{print "Memory Usage: " $6 }') > /dev/null 2>&1
        if [[ "$MEM_USAGE" > "$MAX_MEM" ]]; then
            MAX_MEM=$MEM_USAGE
        fi
        sleep 0.1
    done
    echo $MAX_MEM
    return 0
}

./compile.sh && clear || { echo "compile.sh failed"; exit 1; }

clear

./test.sh && clear || { echo "test.sh failed"; exit 1; }

clear

echo ""
echo "Running the plain language implementations..."
echo ""


echo ""
echo "🔻 Running Ruby project..."
cd ruby
measure_memory "ruby main.rb"
cd ..

echo ""
echo "🐍 Running Python project..."
cd python
measure_memory "python3 main.py"
cd ..

echo ""
echo "⚙️ Running Rust project..."
cd rust
# We have already built the project, so we can run the binary directly
measure_memory "./target/release/three-point-distance"
# Uncomment the line below if you want to run it with cargo instead
# cargo run --release
cd ..

echo ""
echo ""
echo "Running the language + rust implementations..."
echo ""

echo ""
echo "🔻⚙️ Running Ruby-Magnus project..."
cd ruby-magnus
measure_memory "ruby less_efficient.rb"
echo ""
measure_memory "ruby more_efficient.rb"
cd ..

echo ""
echo "🐍⚙️ Running Python Py03 project..."
cd py03
measure_memory "pipenv run python less_efficient.py"
echo ""
measure_memory "pipenv run python more_efficient.py"
cd ..

echo ""
echo ""
echo "Running the Python generator implementation -- no rust..."

echo ""
echo "🐍 Running Pure Python project, but using a generator instead of a list comprehension..."
cd python_generator
measure_memory "python3 main.py"
cd ..

echo ""
echo "All tasks completed successfully."
