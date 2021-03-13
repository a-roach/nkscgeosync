# nkscgeosync
A program to sync location data from Nikon .NEF files to Nikon sidecar files when it is missing. As a bonus, the program also lets you set various noise reduction settings en-mass.
## Rational
Like apparently a few people out there (for example [here](https://exiftool.org/forum/index.php?topic=10067.0) or [here](https://www.dpreview.com/forums/post/61818470)), I was "hand geolocating" my NEF files using [GeoSetter](https://geosetter.de/), but then discovered that if edits were made in Capture NX-D or the new NX-Studio the location data would disappear. It was fine if the data was originally set in View NX-I prior to editing the images, but if you used a 3rd party program to set it within the NEF it was quite possible to get badly out of sync. I had hundreds of these, and didn't really fancy hand editing them to fix the problem, but thought in my infinite wisdom a much better approach might be to take ten times the time and write a program to do it, such is my mind 😉. 

Actually, there was a second and more compelling reason - I wanted to learn Rust, and didn't quite dig the whole "hello world" path, and feeling that I needed to project to through my attentions to eventually choose this (normally I'd just pick something at work to do it, but I've been on sick leave for over a year). I could have banged this out in C in a fraction of the time, but it was a learning process, and I have to confess, quite a fun one. I am starting to be won over by many of Rust's charms. But if you look at the code, please be a little kind - it literally is my first Rust program, and I've mostly come from a C world so there are probably a lot of stylistic hangovers from this, not least of which is my default code formatting which is pretty much straight gnu C style and most definitely not Rust.

## Usage
Currently the program is strictly a CLI program. As part of my "Rust learning curve" I am thinking of adding either a GUI or Wizard97 interface to the program eventually as well, but that is for another day.

Run the program in the directory where the .NEF files are saved, or specify individual .NEF files on the directory - it will poke its head into the sidecar directory to find what it needs.

Running the program without any parameters will execute it in the current directory with default settings (geosync, backup). I'm thinking of removing this, but for now it stays because it is the way I am running it, and as far as I can tell I am currently the only person using it. 😉

Command line parameters can not be compounded, but can be specified individually, e.g. \"-vrl\" won't work, but \"-v -r -l\" will.

### Command Line Options

#### -? or -h
Help. 
#### -v
Verbose
#### -r
Recursively search sub-directories.

In theory this works, but to be honest, I've never been quite game enough to try it in a "production setting", so use at your own risk. In my much mangled system I fear the command would hit a symbolic link and end up in an endless loop or something. That is not to say it will happen, I just live with that fear. At any rate, I think that usually you (as is the case for me) would be running it over a single directory at a time anyway.
#### -l
Look for NEF/NKSC files but do not sync them - just print the results to the screen.
#### --astro
Set "Astro Noise Reduction" to "On".

I wish there were a way to just set this up to come through turned on by default. My D7100 has a few hot pixels, my old D80 has quite a few more, and this command turns on what I consider a missnamed function which nicely gets rid of hot pixels. Nikon, why didn't you call it "Remove hot pixels" ?
#### --best
Set noise reduction to "Best".

Why Nikon, with modern CPUs, default it to fastest and ugly? My now, somewhat vintage i5, is more than capable of doing "Best", and why not have the best? This will set it by default!
#### --edge
Set "Edge Noise Reduction" to "On".

Never been 100% sold on the old Edge noise reduction, there are times when I've seen it make a difference, but that's on a case-by-case basis, but in case someone out there likes this for _everything_ you can turn it on here.
#### --noback
Do not back up the original file. If there already is an "original file", then it wont attempt a backup.
#### --nosync
Only show the NKSC file which are out of sync with NEF files.
#### --nogeo
Don't execute the geosync code (i.e. do only --astro and/or --best etc.). I know this is inverse logic from the other options and I am considering changing it in the future, but for now it will remain the way it is.
#### -d
Specify a directory to search, or additional directories to search. If none are specified the current directory is used.
#### -e
Change the extension to search on. By default .NEF is used, but this could be changed to .JPG if desired. Confession time - I've never actually tested this. I never shoot JPEG, and I am not sure in any context it would be practical to want to run the program off of JPEGs, but it is here if you want.

### Examples
`nkscgeosync -l -d c:\test_data`  
will parse `c:\test_data`, listing all NEF and their associated sidecar files and indicate if they have location data.

`nkscgeosync -l --nosync -d c:\test_data`  
will parse `c:\test_data`, listing NEF and their associated sidecar files which are not synchronised.

`nkscgeosync -l`  
will parse the current directory, listing all NEF and their associated sidecar files and indicate if they have location data.

`nkscgeosync -d c:\test_data --astro`  
will parse `c:\test_data`, finding any files for which there is missing location data, then update the location data. At the same time, it will set Astro noise reduction to on if it is turned off.

`nkscgeosync -d c:\test_data --best --edge --nogeo`  
will parse `c:\test_data`, finding any files which don't have the edge noise reduction set and at the same time it will set the noise reduction from fastest to best. Location data will be ignored.

`nkscgeosync wedding*.nef --astro --best`  
will search the current directory for all files matching `wedding*.nef`, syncing location data if necessary, finding any files which don't have astro noise reduction set and at the same time it will set the noise reduction from fastest to best.

## License
Copyright © 2021 Andrew Roach. All rights reserved.

GNU General Public License version 3

nkscgeosync is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

nkscgeosync is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.