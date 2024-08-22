/*
 * The purpose of this module is to find all function definitions inside of a directory.
 * 
 * Both functions generally do the same thing, but find_def_directory_rec will recursively search files, 
 * while the other does not.
 * 
 * both take in a directory as their only parameter
 */


use crate::utils::types::{Definition, Call};
use std::fs;
use std::thread;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread::JoinHandle;
use crate::finders::all_definitions;

use super::find_single::read_single_function;


/**
 * THIS IS THE NON-RECURSIVE VERSION
 * This function will find all of the function definitions in a given directory.
 * This function makes use of multithreading for quicker file I/O.
 * @param directory {&str} the directory to search
 * @return {Option<HashMap<String, Vec<Definition>>>} An option that holds a hashmap that uses filenames as keys and definitions vectors as values
 */
pub fn find_def_directory(directory: &str) -> Option<HashMap<String, Vec<Definition>>> {
    //define the path, get the files, and define output hash map
    let files = fs::read_dir(directory);
    let results: Arc<Mutex<HashMap<String, Vec<Definition>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // read every file
    for file in files.ok()? {
        let file = file.ok()?;
        let path = file.path().to_str()?.to_string();

        //skip if not a js file
        if !path.ends_with(".js") {
            continue;
        }

        //clone results so we can safelt access from all threads
        let results = Arc::clone(&results);

        //spawn a new thread to call all_definitions
        let handle = thread::spawn(move || {
            let definitions = all_definitions(&path).unwrap_or_else(Vec::new);
            let mut results = results.lock().unwrap();
            results.insert(path.to_string(), definitions);
        });

        //add to all handles
        handles.push(handle);
    } 

    //wait for each thread to finish
    for handle in handles {
        handle.join()
            .unwrap();
    }

    //unwrap the reults of reading files
    let results = Arc::try_unwrap(results) //unwrap the result
        .ok() //ok the Result<>
        .unwrap() //remove the option
        .into_inner() //get inner data from mutex
        .unwrap(); //remove the result


    if !results.is_empty() {
        Some (results)
    }
    else {
        None
    }
}


/**
 * THIS FUNCTION IS THE RECURSIVE VERSION
 * This function will find all of the function definitions in a given directory.
 * This function makes use of multithreading for quicker file I/O.
 * @param directory {&str} the directory to search
 * @return {Option<HashMap<String, Vec<Definition>>>} An option that holds a hashmap that uses filenames as keys and definitions vectors as values
 */
pub fn find_def_directory_rec(directory: &str) -> Option<HashMap<String, Vec<Definition>>> {
    //define the path, get the files, and define output hash map
    let files = fs::read_dir(directory);
    let results: Arc<Mutex<HashMap<String, Vec<Definition>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // read every file
    for file in files.ok()? {
        let file = file.ok()?;
        let path = file.path();

        if path.is_dir() {
            //make recursive call if we find a directory
            if let Some(rec_result) = find_def_directory_rec(path.to_str().unwrap()) {
                //unwrap the mutex so we can insert
                let mut results = results.lock().unwrap();

                for (key, value) in rec_result {
                    results.entry(key).or_insert_with(Vec::new).extend(value);
                }
            }
            continue;
        }

        //skip if not a js file
        if !path.to_str().unwrap().ends_with(".js") {
            continue;
        }

        //clone results so we can safely access from all threads
        let results = Arc::clone(&results);

        //spawn a new thread to call all_definitions
        let handle = thread::spawn(move || {
            let definitions = all_definitions(path.to_str().unwrap()).unwrap_or_else(Vec::new);
            let mut results = results.lock().unwrap();
            results.insert(path.to_str().unwrap().to_string(), definitions);
        });

        //add to all handles
        handles.push(handle);
    } 

    //wait for each thread to finish
    for handle in handles {
        handle.join()
            .unwrap();
    }

    //unwrap the reults of reading files
    let results = Arc::try_unwrap(results) //unwrap the result
        .ok() //ok the Result<>
        .unwrap() //remove the option
        .into_inner() //get inner data from mutex
        .unwrap(); //remove the result

    if !results.is_empty() {
        Some (results)
    }
    else {
        None
    }
}


//========================================================================================
//Directory finders for function CALLS


/**
 * THIS IS THE NON-RECURSIVE VERSION
 * Finds all occurences of some function in a directory
 * @param directory {&str} The given directory to search
 * @param function {&str} the function we want to find all occurences of
 * @return {Option<HashMap<String, Vec<Call>>>} A hashmap where keys are filenames and values are Call vecs
 */
pub fn find_call_directory(directory: &str, function: &str) -> Option<HashMap<String, Vec<Call>>> {
    //define the path, get the files, and define output hash map
    let files = fs::read_dir(directory);
    let results: Arc<Mutex<HashMap<String, Vec<Call>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // read every file
    for file in files.ok()? {
        let file = file.ok()?;
        let path = file.path();

        //skip if not a js file
        if !path.ends_with(".js") {
            continue;
        }

        //clone results so we can safely access from all threads
        let results = Arc::clone(&results);
        let function_clone = String::from(function);

        //spawn a new thread to call all_definitions
        let handle = thread::spawn(move || {
            let path = path.to_str()
                .unwrap();
            match read_single_function(path, &function_clone) {
                Some((_, calls)) => {
                    let mut results = results.lock().unwrap();
                    results.insert(String::from(path), calls);
                },
                None => {
                    let mut results = results.lock().unwrap();
                    results.insert(String::from(path), Vec::new());
                }
            }
        });

        //add to all handles
        handles.push(handle);
    } 

    //wait for each thread to finish
    for handle in handles {
        handle.join()
            .unwrap();
    }

    //unwrap the reults of reading files
    let results = Arc::try_unwrap(results) //unwrap the result
        .ok() //ok the Result<>
        .unwrap() //remove the option
        .into_inner() //get inner data from mutex
        .unwrap(); //remove the result


    if !results.is_empty() {
        Some (results)
    }
    else {
        None
    }
}


/**
 * THIS IS THE RECURSIVE VERSION
 * Finds all occurences of some function in a directory
 * @param directory {&str} The given directory to search
 * @param function {&str} the function we want to find all occurences of
 * @return {Option<HashMap<String, Vec<Call>>>} A hashmap where keys are filenames and values are Call vecs
 */
pub fn find_call_directory_rec(directory: &str, function: &str) -> Option<HashMap<String, Vec<Call>>> {
    //define the path, get the files, and define output hash map
    let files = fs::read_dir(directory);
    let results: Arc<Mutex<HashMap<String, Vec<Call>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // read every file
    for file in files.ok()? {
        let file = file.ok()?;
        let path = file.path();

        if path.is_dir() {
            //make recursive call if we find a directory
            if let Some(rec_result) = find_call_directory_rec(path.to_str().unwrap(), function) {
                //unwrap the mutex so we can insert
                let mut results = results.lock().unwrap();

                for (key, value) in rec_result {
                    results.entry(key).or_insert_with(Vec::new).extend(value);
                }
            }
            continue;
        }

        //skip if not a js file
        if !path.ends_with(".js") {
            continue;
        }

        //clone results so we can safely access from all threads
        let results = Arc::clone(&results);
        let function_clone = String::from(function);

        //spawn a new thread to call all_definitions
        let handle = thread::spawn(move || {
            let path = path.to_str()
                .unwrap();
            match read_single_function(path, &function_clone) {
                Some((_, calls)) => {
                    let mut results = results.lock().unwrap();
                    results.insert(String::from(path), calls);
                },
                None => {
                    let mut results = results.lock().unwrap();
                    results.insert(String::from(path), Vec::new());
                }
            }
        });

        //add to all handles
        handles.push(handle);
    } 

    //wait for each thread to finish
    for handle in handles {
        handle.join()
            .unwrap();
    }

    //unwrap the reults of reading files
    let results = Arc::try_unwrap(results) //unwrap the result
        .ok() //ok the Result<>
        .unwrap() //remove the option
        .into_inner() //get inner data from mutex
        .unwrap(); //remove the result


    if !results.is_empty() {
        Some (results)
    }
    else {
        None
    }
}