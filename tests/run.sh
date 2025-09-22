#!/bin/bash

cd ./tests/

for test_name in *
do
    cd $test_name

    javac Test.java

    printf "[runner] Running test ${test_name}..."
    rjvm_result=$(../../../target/release/rjvm_desktop Test.class 2> /dev/null)
    java_result=$(java Test 2> /dev/null)

    if [[ "$rjvm_result" == "$java_result" ]]; then
        printf "\033[0;32mok\033[0m\n"
    else
        printf "\033[0;31mFAILED\033[0m\n"
        echo ""
        echo "rjvm:"
        echo $rjvm_result
        echo ""
        echo "java:"
        echo $java_result
        echo ""
    fi

    cd ..
done


rm */*.class
