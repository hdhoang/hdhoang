#!/bin/sh
while IFS="" read -r p || [ -n "$p" ]
do
if [[ ${p:0:1} != "#" ]] && [[ $p != "" ]]
   then
     export "$p"
   fi
done < ~/.config/environment.d/${HOSTNAME}.conf
