#!/bin/bash

# THIS SCRIPT IS FOR FETCHING GLUCOSE FROM ONLINE AND PATCHING TO EXPOSE SOME FIELD TO API
# IF THE VENDOR FOLDER IS AVAILABLE DUE TO LICENCING.

GLUCOSE_FOLDER="glucose-syrup-4.1"
GLUCOSE_TAR="${GLUCOSE_FOLDER}.tgz"
GLUCOSE_LINK="https://www.labri.fr/perso/lsimon/downloads/softwares/${GLUCOSE_TAR}"

if [ ! -d "vendor" ]; then
    wget $GLUCOSE_LINK
    tar -xzf $GLUCOSE_TAR
    rm "${GLUCOSE_TAR}"
    mv $GLUCOSE_FOLDER "vendor_wip"
    # Create patch if you already have the modified vendor
    # diff -ruN vendor_wip/ vendor/ > glucose.patch
    # Apply the patch for vendor_wip
    patch -s -p0 < opt_glucose.patch
    mv vendor_wip/ vendor/
fi
