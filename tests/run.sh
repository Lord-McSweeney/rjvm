#!/bin/bash

function run_test () {
    javac Test.java

    printf "[runner] Running test ${test_name}..."

    rjvm_result=$(../../../target/release/rjvm_desktop Test 2> /dev/null)
    rjvm_exit=$?
    java_result=$(java Test 2> /dev/null)
    java_exit=$?

    rm *.class

    if [[ "$rjvm_result" == "$java_result" && $rjvm_exit == 0 && $java_exit == 0 ]]; then
        printf "\033[0;32mok\033[0m\n"
    else
        printf "\033[0;31mFAILED\033[0m\n"
    fi
}

cd ./tests/

if [[ $# == 1 ]]; then
    # run specified test
    test_name=$1

    cd $1

    run_test

    cd ..
else
    # run all tests
    for test_name in *
    do
        cd $test_name

        run_test

        cd ..
    done
fi

# just in case
rm -f */*.class
