#!/bin/bash

cd ./tests/

for test_name in *
do
    cd $test_name

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

    cd ..
done

# just in case
rm -f */*.class
