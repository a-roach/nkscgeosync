// Compiler options

#![allow(unused_parens)]
#![allow(non_snake_case)]

// Import Identifers

use std::env;
use std::fs;
use std::fs::File;
use std::path::{Path,PathBuf};
use std::io::{BufReader,Write};
use std::io::prelude::*;

use data_encoding::BASE64_NOPAD;
use data_encoding::BASE64;
use ansi_term::Colour;
use exif::{ In, Value, Tag};

// Define Structures

struct LocationData
  {
    GPSLatitudeRef: String,
    GPSLatitude: String,
    GPSLongitudeRef: String,
    GPSLongitude: String,
    GPSAltitude: String,
    GPSDateStamp: String,
    GPSTimeStamp: String
  }

// Custom Macros

macro_rules! verbose
  {
    ($( $args:expr ),*) =>
      {
        unsafe
          {
            if (VERBOSE==true)
              {
                println!( $( $args ),* );
              }
          }
      }
  }

// Global Variables

static mut VERBOSE:bool=false;                                                  // Used to tell us if we are going to be verbose
const VERSION_STRING: &'static str = env!("VERSION_STRING");

fn main()
{
  let mut search_extension = ".".clone().to_owned()+&"nef".clone().to_owned(); // Default extension to search for
  let mut recursive:bool=false;                                                // Are going to do recursive parsing of directories?
  let mut i_want_to_save_changes:bool=true;                                    // Tells the program to save changes to the nksc file - if turned off you just get a listing
  let mut i_want_to_save_the_original_file:bool=true;                          // Tells the program to backup the nksc file before making changes
  let mut i_want_to_see_everything:bool=true;                                  // Tells the program to show all nksc/nef files, even if they are in sync
  let mut astro:bool=false;                                                    // Turn on astro noise reduction
  let mut best_quality:bool=false;                                             // Change the noise reduction from Fastest to Best
  let mut edge:bool=false;                                                     // Enable edge noise reduction  
  let mut enable_geo_sync:bool=true;                                           // Process the geo location data 
  let args: Vec<String> = wild::args().collect();                              // Command line arguments
  let mut file_names = Vec::new();                                             // File name pointers   
  let mut directory_names = Vec::new();                                        // Directory name pointers

  println!("{}", VERSION_STRING);
  
  /*
   * Parse the command line.
   * We'll use a while loop for this so we can skip given parameters
   */
  {
    let mut i=1;
    while i < args.len()
      {
        let argument=args[i].to_lowercase();

        if (argument == "-?")||
           (argument =="-h")
           {
              println!("{}","Program to insert location data stored in an NEF into the NKSC_PARAM if it is missing.\n\
                        \nUSAGE:\n\
                        \x20  nkscgeosync [OPTIONS] [<file names>]\n\
                        \nOPTIONS:\n\
                        \x20  -v              Verbose\n\
                        \x20  -r              Recursively search sub-directories\n\
                        \x20  -l              Look for NEF/NKSC files but do not sync them - just print the results to the screen.\n\
                        \x20  --astro         Set \"Astro Noise Reduction\" to \"On\".\n\
                        \x20  --best          Set noise reduction to \"Best\".\n\
                        \x20  --edge          Set \"Edge Noise Reduction\" to \"On\".\n\
                        \x20  --noback        Do not back up the original file\n\
                        \x20  --nosync        Only show the NKSC file which are out of sync with NEF files.\n\
                        \x20  --nogeo         Don't execute the geosync code (i.e. do only --astro and/or --best).\n\
                        \x20  -d <dir name>   Specify a directory to search, or additional directories to search.\n\
                        \x20                  If none are specified the current directory is used.\n\
                        \x20  -e              Change the extension to search on.\n\
                        \x20                  By default .NEF is used, but this could be changed to .JPG if desired.\n\
                        \nCommand line parameters can not be compounded, but can be specified individually, e.g. \"-vrl\" won't work, but \"-v -r -l\" will.\
                        ");

              println!("\n{}",Colour::Yellow.on(Colour::Red).paint("Yellow writing with a red background means there is no location data."));
              println!("{}",Colour::Blue.on(Colour::Green).paint("Blue writing with a green background means there is location data."));
              println!("{}",Colour::Black.on(Colour::Yellow).paint("Black on Yellow indicates a file is being processed."));

              quit::with_code(0);
           }

        else if (argument == "-r")
           {
             recursive = true;
           }
        else if (argument == "-v")
           {
             unsafe { VERBOSE = true;}
           }
        else if (argument == "--astro")
           {
             astro = true;
           }
        else if (argument == "--best")
           {
             best_quality = true;
           }
        else if (argument == "--edge")
           {
             edge = true;
           }
        else if (argument == "--noback")
           {
             i_want_to_save_the_original_file = false;
           }
        else if (argument == "--nosync")
           {
             i_want_to_see_everything = false;
           }
        else if (argument == "--nogeo")
           {
             enable_geo_sync = false;
           }
        else if (argument == "-l")
           {
             i_want_to_save_changes = false;
           }
        else if (argument == "-d")
           {
             directory_names.push(i+1);
             i+=1;
           }
        else if (argument == "-e")
           {
             i+=1;
             search_extension = ".".clone().to_owned()+&argument.clone().to_lowercase();
           }
        else
          {
            let test_Path=Path::new(&args[i]);

            /*
             * See if the file exists, if it does, add it to a list of files we will process later on
             */
            if test_Path.exists()
              {
                file_names.push(i);
              }
            else
              {
                println!("Unknown something ({}) given on command line. To be safe I'm gonna die.",args[i]);
                quit::with_code(0);
              }
          }
        i+=1;
      }
  }


  /*
   * If we have been given any file names on the command line, we will now walk through them and process each individually
   */
  if (file_names.len()>0)
    {
      for file_idx in file_names
        {
          let path = PathBuf::from(args[file_idx].to_lowercase());

          if enable_geo_sync
            {
              geo_sync_a_file(&path, &search_extension, i_want_to_save_changes,i_want_to_save_the_original_file,i_want_to_see_everything);
            }

          if astro==true || best_quality==true || edge==true
            {  
              set_astro_or_best_on_in_a_file(&path, &search_extension, i_want_to_save_changes,i_want_to_save_the_original_file,i_want_to_see_everything, astro, best_quality,edge);
            }
        }
    }


  /*
   * Now we will start the business. A few things can happen here, if no directories were specified on the command line, or indeed there was no command line, then the current
   * directory will be searched. Can be useful I guess.
   * When one or more directory names are given they will be parsed one at a time.
   * When the recursive flag is given, all subdirectories will also be searched. If you have a subdirectory also specified as one of the directories on the command line, the
   * behaviours will be unspecified, but possibly harmless. Maybe. Hopefully. Having symlinks nested within a recursive search could be another matter entirely, that probably
   * will be a very bad thing and will doubtless end in an endless loop.
   */

  if (directory_names.len()>0)
    {
      for dir_idx in directory_names
        {
          let SearchDirectory = Path::new(&args[dir_idx]).to_path_buf();

          WalkDirectory(&SearchDirectory, &search_extension, recursive,i_want_to_save_changes,i_want_to_save_the_original_file,i_want_to_see_everything,enable_geo_sync,
                        astro,best_quality,edge);
        }
    }
  else // We were not given any directory paths to process, so we'll use the current directory instead
    {
      let SearchDirectory = env::current_dir().expect("Could not find the starting directory to look for files.");

      WalkDirectory(&SearchDirectory, &search_extension, recursive,i_want_to_save_changes,i_want_to_save_the_original_file,i_want_to_see_everything,enable_geo_sync,
                    astro,best_quality,edge);
    }
}



/** check_if_there_isnt_already_location_data_in
  fn check_if_there_isnt_already_location_data_in(file: &PathBuf) -> bool
    file: &PathBuf = path to an nksc file

  Takes a fully qualified path as a parameter and returns true if the file has GPS data in it,
  and false if it does not. I' sure there is an easier way to write this since it is basically a
  single line function, but off the top of my head I can't quite remember how.
**/
fn check_if_there_isnt_already_location_data_in(file: &PathBuf) -> bool
{
 if (fs::read_to_string(file).expect("Could not open nksc file to check its contents").find("GPSLatitude rdf:parseType")!=None)
   {
     return false;
   }
 true
}


/** check_if_there_is_location_data_in
  fn check_if_there_is_location_data_in(file: &Path) -> bool
   file: &Path = path to an exif file

  Takes a fully qualified path as a parameter and returns true if the file has GPS data in it,
  and false if it does not.
**/
fn check_if_there_is_location_data_in(file: &Path) -> bool
{
  let file = File::open(file).expect(format!("Could not open {:?}",file.file_name()).as_str());
  let exif = exif::Reader::new().read_from_container(&mut BufReader::new(&file)).expect("Could not read EXIF data");

  for f in exif.fields()
    {
      if (format!("{}",f.tag)=="GPSVersionID")
        {
          return true;
        }
    }
  false
}


/** get_location_data_from_exif
  fn get_location_data_from_exif(file: &Path, LocationData: &mut LocationData)
  
    file: &Path = path to the NEF (or possibly JPEG) file with exif data containing locations
    LocationData: &mut LocationData = pointer to a structure to hold our location data

  Function opens up a given NEF file and then uses the KAMADAK EXIF reader to interrogate the file and get out the raw exif data for the GPSLocation data.
  After that it munges the data for each of the types of data into the different formats which are required. Some of the data is encoded in plain ASCII,
  some in un-padded Base64 and others in padded Base64.

**/
fn get_location_data_from_exif(file: &Path, LocationData: &mut LocationData)
{
  let file = File::open(file).expect(format!("Could not open {:?}",file.file_name()).as_str());
  let exif = exif::Reader::new().read_from_container(&mut BufReader::new(&file)).expect("Could not read the exif data from an image file");
  
  
  /*
   * Read the GPSLatitudeRef from the exif data and convert it to a pre-computed un-padded BASE64 value
   */
  if let Some(field) = exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY)
    {
      if format!("{}",field.display_value())=="N"  // 0
        {
          verbose!("North");
          LocationData.GPSLatitudeRef=String::from("AAAAAA==");
        }
      else // South = 1
        {
          verbose!("South");
          LocationData.GPSLatitudeRef=String::from("AQAAAA==");
        }
    }


  /*
   * Read the GPSLongitudeRef from the exif data and convert it to a pre-computed un-padded BASE64 value
   */
  if let Some(field) = exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY)
    {
      if format!("{}",field.display_value())=="E" // 2
        {
          verbose!("East");
          LocationData.GPSLongitudeRef=String::from("AgAAAA==");
        }
      else // W = 3
        {
          verbose!("West");
          LocationData.GPSLongitudeRef=String::from("AwAAAA==");
        }
    }


  /*
   * Read the Latitude from the exif data which is stored a 3 f64 values and convert it to an un-padded BASE64 value
   * Unfortunately to do the conversion, we have to do it in a super convoluted way, namely we have to map each of our three variables to three buffers
   * using the transmute function, then we have to in-turn copy those three individual mapped buffers into a continuous memory block to get Base64 encoded.
   * Finally we reformat the output as a string and copy it into our location data structure.
   */
  if let Some(field) = exif.get_field(Tag::GPSLatitude, In::PRIMARY)
    {
      match field.value
        {
          Value::Rational(ref latitude) =>
            {
              let mut raw_bytes: [u8; 24]= [0;24];
              let raw_bytes0: [u8; 8] = unsafe { std::mem::transmute(latitude[0].to_f64()) };
              let raw_bytes1: [u8; 8] = unsafe { std::mem::transmute(latitude[1].to_f64()) };
              let raw_bytes2: [u8; 8] = unsafe { std::mem::transmute(latitude[2].to_f64()) };

              for i in 0..8
                {
                  raw_bytes[i]=raw_bytes0[i];
                  raw_bytes[i+8]=raw_bytes1[i];
                  raw_bytes[i+16]=raw_bytes2[i];
                }

              let b64=BASE64_NOPAD.encode(&raw_bytes);
              LocationData.GPSLatitude=String::from(format!("{}", b64));

              unsafe
                {
                  if (VERBOSE==true)
                    {
                      let Latitude: f64=latitude[0].to_f64()+(latitude[1].to_f64()/60.0)+(latitude[2].to_f64()/3600.0);

                      println!("Field Value: {:?}\nDecimal Degrees: {}", field.value,Latitude);
                      println!("GPSLatitude BASE64: {}", LocationData.GPSLatitude);
                    }
                }
            },
          _ => {},
        }
    }


  /*
   * Read the Longitude from the exif data and convert it to an un-padded BASE64 value
   */
  if let Some(field) = exif.get_field(Tag::GPSLongitude, In::PRIMARY)
    {
      match field.value
        {
          Value::Rational(ref longitude) =>
            {
              let mut raw_bytes: [u8; 24]= [0;24];
              let raw_bytes0: [u8; 8] = unsafe { std::mem::transmute(longitude[0].to_f64()) };
              let raw_bytes1: [u8; 8] = unsafe { std::mem::transmute(longitude[1].to_f64()) };
              let raw_bytes2: [u8; 8] = unsafe { std::mem::transmute(longitude[2].to_f64()) };

              for i in 0..8
                {
                  raw_bytes[i]=raw_bytes0[i];
                  raw_bytes[i+8]=raw_bytes1[i];
                  raw_bytes[i+16]=raw_bytes2[i];
                }

              let b64=BASE64_NOPAD.encode(&raw_bytes);
              LocationData.GPSLongitude=String::from(format!("{}", b64));

              unsafe
                {
                  if (VERBOSE==true)
                    {
                      let Longitude: f64=longitude[0].to_f64()+(longitude[1].to_f64()/60.0)+(longitude[2].to_f64()/3600.0);
                      println!("Field Value: {:?}\nGPSLongitude Decimal Degrees: {}", field.value,Longitude);
                      println!("GPSLongitude BASE64: {}", LocationData.GPSLongitude);
                    }
                }
            },
          _ => {},
        }
    }


  /*
   * Read the GPSAltitude from the exif data and convert it from a single f64 into a padded BASE64 value
   */
  if let Some(field) = exif.get_field(Tag::GPSAltitude, In::PRIMARY)
    {
      match field.value
        {
          Value::Rational(ref Altitude) =>
            {
              let raw_bytes0: [u8; 8] = unsafe { std::mem::transmute(Altitude[0].to_f64()) };
              let b64_00 = BASE64.encode(&raw_bytes0);
              LocationData.GPSAltitude=String::from(format!("{}", b64_00));
              unsafe
                {
                  if (VERBOSE==true)
                    {
                      let GPSAltitude: f64=Altitude[0].to_f64();
                      println!("Field Value: {:?}\nGPSAltitude: {}", field.value,GPSAltitude);
                      println!("GPSAltitude BASE64: {}", LocationData.GPSAltitude);
                    }
                }
            },
          _ => {},
        }
    }


  /*
   * Read the GPSDateStamp
   */
  if let Some(field) = exif.get_field(Tag::GPSDateStamp, In::PRIMARY)
    {
      LocationData.GPSDateStamp=String::from(format!("{}",field.display_value()).replace("-",":"));
      verbose!("\nGPSDateStamp: {}", LocationData.GPSDateStamp);
    }


  /*
   * Read the GPSTimeStamp from the exif data and convert it to an un-padded BASE64 value
   */
  if let Some(field) = exif.get_field(Tag::GPSTimeStamp, In::PRIMARY)
    {
      match field.value
        {
          Value::Rational(ref TimeStamp) =>
            {
              let mut raw_bytes: [u8; 24]= [0;24];
              let raw_bytes0: [u8; 8] = unsafe { std::mem::transmute(TimeStamp[0].to_f64()) };
              let raw_bytes1: [u8; 8] = unsafe { std::mem::transmute(TimeStamp[1].to_f64()) };
              let raw_bytes2: [u8; 8] = unsafe { std::mem::transmute(TimeStamp[2].to_f64()) };

              for i in 0..8
                {
                  raw_bytes[i]=raw_bytes0[i];
                  raw_bytes[i+8]=raw_bytes1[i];
                  raw_bytes[i+16]=raw_bytes2[i];
                }

              let b64=BASE64_NOPAD.encode(&raw_bytes);
              LocationData.GPSTimeStamp=String::from(format!("{}",b64));

              unsafe
                {
                  if (VERBOSE==true)
                    {
                      println!("GPSTimeStamp BASE64: {}", LocationData.GPSTimeStamp);
                    }
                }
            },
          _ => {},
        }
    }

}


/** create_new_nksc_file
  fn create_new_nksc_file(file: &Path, LocationData: &mut LocationData)

    file: &Path = path to the NKSC file we wish to amend
    LocationData: &mut LocationData = pointer to a structure with our location data in it

  Function will open up an existing nksc file and insert into it the update location data.
  Although the nksc is an XML file and I could probably have used an XML library for writing the data, it is such a basic and small file format
  that we are just going to open it up into memory and make the changes there then save it back to disk.
**/
fn create_new_nksc_file(file: &Path, Location: &mut LocationData, i_want_to_save_the_original_file: bool)
{
  let mut fr = File::open(file).expect("Could not open file.");
  let mut nksc = String::new();

  fr.read_to_string(&mut nksc).expect("Unable to read from the file");
  drop(fr);

  let idx = nksc.find("</rdf:Description>").expect("Could not find a vlad XML tag to hook in to."); // find the end of the nksc XML data, where we will insert our new fragment

  if (idx>0)
    {
      for i in (0..idx).rev() // The tag we found before will be indented, so walk back to the start of the line and then insert out new stuff there
        {
          if (nksc.chars().nth(i)==Some('\n'))
            {
              let mut xml:String=format!("\n         <ast:GPSVersionID rdf:parseType=\"Resource\">\n\
                                          \x20           <rdf:value>AgIAAA==</rdf:value>\n\
                                          \x20           <astype:Type>Binary</astype:Type>\n\
                                          \x20        </ast:GPSVersionID>\n\
                                          \x20        <ast:GPSLatitudeRef rdf:parseType=\"Resource\">\n\
                                          \x20           <rdf:value>{}</rdf:value>\n\
                                          \x20           <astype:Type>Long</astype:Type>\n\
                                          \x20        </ast:GPSLatitudeRef>\n\
                                          \x20        <ast:GPSLatitude rdf:parseType=\"Resource\">\n\
                                          \x20           <rdf:value>{}</rdf:value>\n\
                                          \x20           <astype:Type>Double</astype:Type>\n\
                                          \x20        </ast:GPSLatitude>\n\
                                          \x20        <ast:GPSLongitudeRef rdf:parseType=\"Resource\">\n\
                                          \x20           <rdf:value>{}</rdf:value>\n\
                                          \x20           <astype:Type>Long</astype:Type>\n\
                                          \x20        </ast:GPSLongitudeRef>\n\
                                          \x20        <ast:GPSLongitude rdf:parseType=\"Resource\">\n\
                                          \x20           <rdf:value>{}</rdf:value>\n\
                                          \x20           <astype:Type>Double</astype:Type>\n\
                                          \x20        </ast:GPSLongitude>\n\
                                          \x20        <ast:GPSMapDatum rdf:parseType=\"Resource\">\n\
                                          \x20           <rdf:value>WGS-84</rdf:value>\n\
                                          \x20           <astype:Type>Ascii</astype:Type>\n\
                                          \x20        </ast:GPSMapDatum>",
                                          Location.GPSLatitudeRef,
                                          Location.GPSLatitude,
                                          Location.GPSLongitudeRef,
                                          Location.GPSLongitude);

              if (Location.GPSAltitude!="")
                {
                  xml.push_str(&format!("\n         <ast:GPSAltitudeRef rdf:parseType=\"Resource\">\n\
                                        \x20           <rdf:value>AA==</rdf:value>\n\
                                        \x20           <astype:Type>Binary</astype:Type>\n\
                                        \x20        </ast:GPSAltitudeRef>\n\
                                        \x20        <ast:GPSAltitude rdf:parseType=\"Resource\">\n\
                                        \x20           <rdf:value>{}</rdf:value>\n\
                                        \x20           <astype:Type>Double</astype:Type>\n\
                                        \x20        </ast:GPSAltitude>",Location.GPSAltitude));
                }

              if (Location.GPSDateStamp!="")
                {
                  xml.push_str(&format!("\n         <ast:GPSDateStamp rdf:parseType=\"Resource\">\n\
                                        \x20           <rdf:value>{}</rdf:value>\n\
                                        \x20           <astype:Type>Ascii</astype:Type>\n\
                                        \x20        </ast:GPSDateStamp>",Location.GPSDateStamp));
                }

              if (Location.GPSTimeStamp!="")
                {
                  xml.push_str(&format!("\n        <ast:GPSTimeStamp rdf:parseType=\"Resource\">\n\
                                        \x20           <rdf:value>{}</rdf:value>\n\
                                        \x20           <astype:Type>Double</astype:Type>\n\
                                        \x20        </ast:GPSTimeStamp>",Location.GPSTimeStamp));
                }

              nksc.insert_str(i+1,&xml);
              break;
            }
        }
    }

  /*
   * Back up the old file next
   * But if there is already a backup, we won't. The logic is the original backup will be the original file, and I don't really
   * want to loose the original original file. Besides, this isn't the sort of thing that we'd be doing more than once anyway.
   */

  if i_want_to_save_the_original_file
    {
      let backup:String=format!("{}.original",file.display());
      let backup_Path=Path::new(&backup);
      if (!backup_Path.exists())
        {
          fs::rename(file.to_str().unwrap(), backup).expect("backing up a file failed");
        }
    }

  /*
   * Write the contents of our reformatted buffer to disk
   */
  let mut output = File::create(format!("{}",file.display())).expect("Create file failed");
  output.write_all(nksc.as_bytes()).expect("write failed");
  drop(output);
}


/** fit_name_in
  fn fit_name_in(path: &String,nChars: usize) -> String
    path: &String = string to truncate or pad
    nChars: usize = number of characters to pad or truncate to

  Function takes a path and number of character as a parameter and either truncate the file name to
  fit with the space, or pads it out.
**/
fn fit_name_in(path: &String,nChars: usize) -> String
{
  let mut me = path.clone().to_owned();
  
  if (me.len()>(nChars))
    {
      me.replace_range(..(me.len()-nChars+3), "...");
      return me;
    }
  else if (me.len()<(nChars))
    {
      for _i in 0..(nChars)-me.len()
        {
          me.push(' ');
        }
      return me;
    }
    return path.to_string()
}


/**  WalkDirectory
  fn WalkDirectory(WhichDirectory: &PathBuf, search_extension: &str, recursive: bool, i_want_to_save_changes: bool, i_want_to_save_the_original_file: bool,
                 i_want_to_see_everything: bool, enable_geo_sync: bool, astro: bool, best_quality: bool, edge: bool)

    WhichDirectory: &PathBuf = directory to search in
    search_extension: &str, = the file extension to look for (default .nef)
    recursive: bool = flag to indicate if a recursive search is to be performed
    i_want_to_save_changes: bool = flag to indicate if we are doing a "listing" run without actually processing teh file
    i_want_to_save_the_original_file: bool = flag to indicate if we wish to save a back up of the original file before modifying it
    i_want_to_see_everything: bool = flag which basically says if I want to see all files, whether they are in sync or not.
    enable_geo_sync: bool = let it do the geo sync
    astro: bool = process astro flag
    best_quality: bool = process best_quality flag
    edge: bool = set edge noise reduction on

  Function which walks through a given directory and basically does all of the work.
*/
fn WalkDirectory(WhichDirectory: &PathBuf, search_extension: &str, recursive: bool, i_want_to_save_changes: bool, i_want_to_save_the_original_file: bool,
                 i_want_to_see_everything: bool, enable_geo_sync: bool,astro: bool, best_quality: bool, edge: bool)
{
  if WhichDirectory.is_dir() // sanity check, probably not necessary, but this is Rust and Rust is all about "safety"
    {
      let paths = fs::read_dir(WhichDirectory).expect("Could not scan the directory");

      println!("Processing: {}", WhichDirectory.display());
      for each_path in paths
        {
          let nef_path = each_path.unwrap();

          if (nef_path.path().is_file())
            {
              if (enable_geo_sync==true)
                {
                  geo_sync_a_file(&nef_path.path().to_path_buf(), search_extension, i_want_to_save_changes,i_want_to_save_the_original_file,i_want_to_see_everything);
                }
              
              if (astro==true || best_quality==true || edge==true)
                {
                  set_astro_or_best_on_in_a_file(&nef_path.path().to_path_buf(), search_extension, i_want_to_save_changes,
                                                  i_want_to_save_the_original_file,i_want_to_see_everything, astro, best_quality, edge);
                }
            }
          else // Directory
            {
              if (recursive == true)
                {
                  verbose!("DIR: {}", nef_path.path().display());
                  WalkDirectory(&nef_path.path().to_path_buf(), search_extension, recursive, i_want_to_save_changes,i_want_to_save_the_original_file,
                                i_want_to_see_everything,enable_geo_sync,astro,best_quality, edge);
                }
            }
        }
    }
  else
    {
      println!("Something went gravely wrong: {:?}", WhichDirectory.file_name());
    }
}

/**  geo_sync_a_file
  fn geo_sync_a_file(nef_path: &PathBuf, search_extension: &str, i_want_to_save_changes: bool, i_want_to_save_the_original_file: bool, i_want_to_see_everything: bool)

    nef_path = path to file 
    search_extension = file extension 
    i_want_to_save_changes = save changes, as opposed to just walking through the files and seeing what is going on inside them
    i_want_to_save_the_original_file = make a back up of the original file before making changes
    i_want_to_see_everything = be quite verbose in the information we print out

  Function which processes an individual file.
*/
fn geo_sync_a_file(nef_path: &PathBuf, search_extension: &str, i_want_to_save_changes: bool, i_want_to_save_the_original_file: bool, i_want_to_see_everything: bool)
{
  let mut column_width:usize = 39;
  
  /*
   * Work out how wide our terminal is, so we can use as much real estate as we possibly can 
   */

    if let Some((w, _h)) = term_size::dimensions() 
      {
        column_width=(w-2)/2;
      } 

  /*
   * Process the Nikon Sidecar File and the NEF to see if there is any GEO data and sync them if there is
   */
  if (format!(".{:?}",nef_path.extension().expect("Hoped to find some NEF files, but I could not.")).to_lowercase().replace("\\\\","\\").replace("\"","")==search_extension)
    {
      let mut nksc_path:String=format!("{:?}\\\\NKSC_PARAM\\\\{:?}.nksc",nef_path.parent().unwrap(), nef_path.file_name().unwrap());
      nksc_path=nksc_path.replace("\\\\","\\").replace("\"","");
      let nksc_Path=Path::new(&nksc_path);

      if nksc_Path.exists()
        {
          let mut Location = LocationData{GPSLatitudeRef: "".to_string(),GPSLatitude: "".to_string(),GPSLongitudeRef: "".to_string(),GPSLongitude: "".to_string(),
                                          GPSAltitude:"".to_string(),GPSDateStamp: "".to_string(),GPSTimeStamp: "".to_string()};
          let nef:String=format!("{}",nef_path.display());
                    
          /*
            * Check to see if the nksc file doesn't already have location data, if it doesn't, then we will
            * see if there is any location data in the NEF file, if there is, we will then try to extract
            * that data and update the nksc file with the data from the exif tag in the NEF.
            */

          let there_isnt_location_data_in_nksc:bool = check_if_there_isnt_already_location_data_in(&nksc_Path.to_path_buf());
          let there_is_location_data_in_nef:bool = check_if_there_is_location_data_in(&nef_path);

          if there_isnt_location_data_in_nksc
            {
              if (there_is_location_data_in_nef)||(i_want_to_see_everything)
                  {
                  print!("{}  ",Colour::Yellow.on(Colour::Red).paint(fit_name_in(&nksc_path,column_width)));
                  }

              if there_is_location_data_in_nef
                {
                  if i_want_to_save_changes
                    {
                      print!("{}",Colour::Black.on(Colour::Yellow).paint(fit_name_in(&nef,column_width)));

                      get_location_data_from_exif(&nef_path,&mut Location);
                      create_new_nksc_file(&nksc_Path,&mut Location, i_want_to_save_the_original_file);

                      for _i in 0..(column_width*2)+2 {print!("\x08")}; // Erase the contents of the line from the screen
                      println!("{}  {}",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nksc_path,column_width)),Colour::Blue.on(Colour::Green).paint(fit_name_in(&nef,column_width)));
                    }
                  else
                    {
                      get_location_data_from_exif(&nef_path,&mut Location);
                      println!("{}",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nef,column_width)));
                    }
                }
              else
                {
                  if (there_is_location_data_in_nef)||(i_want_to_see_everything)
                      {
                      println!("{}",Colour::Yellow.on(Colour::Red).paint(fit_name_in(&nef,column_width)));
                      }
                }
            }
          else  // there IS already location data in the NKSC_PARAM file
            {
              if (!there_is_location_data_in_nef)||(i_want_to_see_everything)
                {
                  print!("{}  ",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nksc_path,column_width)));
                  get_location_data_from_exif(&nef_path,&mut Location);
                  if there_is_location_data_in_nef
                    {
                      println!("{}",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nef,column_width)));
                    }
                  else
                    {
                      println!("{}",Colour::Yellow.on(Colour::Red).paint(fit_name_in(&nef,column_width)));
                    }
                }
            }
        }
    }
}

/**  set_astro_or_best_on_in_a_file
  fn set_astro_or_best_on_in_a_file(nef_path: &PathBuf, search_extension: &str, i_want_to_save_changes: bool, i_want_to_save_the_original_file: bool, 
                                    i_want_to_see_everything: bool,astro: bool, best_quality: bool)

    nef_path = path to file 
    search_extension = file extension 
    i_want_to_save_changes = save changes, as opposed to just walking through the files and seeing what is going on inside them
    i_want_to_save_the_original_file = make a back up of the original file before making changes
    i_want_to_see_everything = be quite verbose in the information we print out
    astro = set the astro noise reduction to on
    best_quality = change the noise reduction from fastest to best quality
    edge = set edge noise reduction to on

  Function which processes an individual file settings its astro flag or best quality flag for noise reduction
*/
fn set_astro_or_best_on_in_a_file(nef_path: &PathBuf, search_extension: &str, i_want_to_save_changes: bool, i_want_to_save_the_original_file: bool, 
                                  i_want_to_see_everything: bool, astro: bool, best_quality: bool, edge: bool)
{
  let mut column_width:usize = 39;
  
  /*
   * Work out how wide our terminal is, so we can use as much real estate as we possibly can 
   */

  if let Some((w, _h)) = term_size::dimensions() 
    {
      column_width=w-24;
    } 

  /*
   * Process the Nikon Sidecar File
   */
  if (format!(".{:?}",nef_path.extension().expect("Hoped to find some NEF files, but I could not.")).to_lowercase().replace("\\\\","\\").replace("\"","")==search_extension)
    {
      let mut nksc_path:String=format!("{:?}\\\\NKSC_PARAM\\\\{:?}.nksc",nef_path.parent().unwrap(), nef_path.file_name().unwrap());
      nksc_path=nksc_path.replace("\\\\","\\").replace("\"","");
      let nksc_Path=Path::new(&nksc_path);
      let mut go_astro:bool = false;
      let mut go_best_quality:bool = false;
      let mut go_edge:bool = false;

      if nksc_Path.exists() // See if the file actually exists, which it should
        {
          if astro==true
            {
              if is_what_already_set_in(nksc_Path,"NoiseReduction.chkSpike\"&gt;1&lt;")
                {
                  if i_want_to_see_everything
                    {
                      println!("Astro:                {} ",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nksc_path,column_width))); 
                    }
                }
              else
                {
                  println!("Astro:                {} ",Colour::Yellow.on(Colour::Red).paint(fit_name_in(&nksc_path,column_width)));
                  go_astro=true;
                }  
            }

          if best_quality==true
            {
              if is_what_already_set_in(nksc_Path,"NoiseReduction.cbMethod\"&gt;1&lt;")
                {
                  if i_want_to_see_everything
                    {
                      println!("Best Quality:         {} ",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nksc_path,column_width))); 
                    }
                }
              else
                {
                  println!("Best Quality:         {} ",Colour::Yellow.on(Colour::Red).paint(fit_name_in(&nksc_path,column_width)));
                  go_best_quality=true;
                }  
            }
          
          if edge==true
            {
              if is_what_already_set_in(nksc_Path,"NoiseReduction.chkEdge\"&gt;1&lt;")
                {
                  if i_want_to_see_everything
                    {
                      println!("Edge Noise Reduction: {} ",Colour::Blue.on(Colour::Green).paint(fit_name_in(&nksc_path,column_width))); 
                    }
                }
              else
                {
                  println!("Edge Noise Reduction: {} ",Colour::Yellow.on(Colour::Red).paint(fit_name_in(&nksc_path,column_width)));
                  go_edge=true;
                }  
            }

            if (go_best_quality==true || go_astro==true || go_edge ==true) && (i_want_to_save_changes==true)
              {
                
                /*
                 * Read the file into memory, once there we'll do a rather grotesque replacement of the strings which control the astro setting
                 * or the best_quality quality setting. After that we will save the buffer back to disk.
                 */

                let mut fr = File::open(nksc_Path).expect(format!("Could not open {:?}",nksc_Path.file_name()).as_str());
                let mut body = String::new();

                fr.read_to_string(&mut body).expect("Unable to read from the file");
                drop(fr);
                
               /*
                * Back up the old file next
                * But if there is already a backup, we won't. The logic is the original backup will be the original file, and I don't really
                * want to loose the original original file. Besides, this isn't the sort of thing that we'd be doing more than once anyway.
                */

                if i_want_to_save_the_original_file
                  {
                    let backup:String=format!("{}.original",nksc_Path.display());
                    let backup_Path=Path::new(&backup);
                    if (!backup_Path.exists())
                      {
                        fs::rename(nksc_Path.to_str().unwrap(), backup).expect("backing up a file failed");
                      }
                  }

                if go_astro
                  {
                    body=body.replace("NoiseReduction.chkSpike\"&gt;0&lt;","NoiseReduction.chkSpike\"&gt;1&lt;");
                  }

                if go_best_quality
                  {
                    body=body.replace("NoiseReduction.cbMethod\"&gt;0&lt;","NoiseReduction.cbMethod\"&gt;1&lt;");
                  }
    
                if go_edge
                  {
                    body=body.replace("NoiseReduction.chkEdge\"&gt;0&lt;","NoiseReduction.chkEdge\"&gt;1&lt;");
                  }

                /*
                 * Write the contents of our reformatted buffer to disk
                 */
                let mut output = File::create(format!("{}",nksc_Path.display())).expect("Create file failed");
                output.write_all(body.as_bytes()).expect("Write failed");
                drop(output);
              }
          }
    }
}

/** is_what_already_set_in
  fn is_what_already_set_in(file: &Path, what: &str) -> bool
   file: &Path = path to an sidecar file
   what: &str = string to look for in the sidecar file to see if it is set

  Takes a fully qualified path as a parameter and returns true if the file has a matching string inside of it
**/
fn is_what_already_set_in(file: &Path, what: &str) -> bool
{
  let mut fr = File::open(file).expect(format!("Could not open {:?}",file.file_name()).as_str());
  let mut body = String::new();

  fr.read_to_string(&mut body).expect("Unable to read from the file");
  drop(fr);  

  if body.contains(what)
    {
      return true;
    }
  false
}