#!/bin/bash

cnt=1
until [ $cnt -gt 10 ]
do
	/usr/bin/time --verbose ../target/release/client-test
	((cnt++))
done
