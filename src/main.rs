
/*
Author: Hunter Treleaven
Date: June 23rd, 2026
Purpose: Loads symptom terms from a CSV file and checks a clinical note
for matching symptoms and aliases.
*/

use serde::Deserialize; // allows rust to convert each row into a rust struct
use std::error::Error; // allows functions to return different types of errors

// communicates with serde so that the struct can be created from data in a CSV file
#[derive(Debug, Deserialize)]
struct SymptomTerm {

    preferred_term: String, // stores symptom names
    category: String, // stores symptom categories
    aliases: String, // stores other potential phrases or spellings

}

// creates and loads all symptom terms from the file
fn load_symptoms(file_path: &str) -> Result<Vec<SymptomTerm>, Box<dyn Error>> {

    let mut reader = csv::Reader::from_path(file_path)?; // opens the CSV file path, function will stop and return error if it fails to open
    let mut symptoms = Vec::new(); // creates an empty vector that holds all the symptoms rows

    // iterates through each row in the CSV file until the end
    for result in reader.deserialize() {

        let symptom: SymptomTerm = result?; // converts current row into a struct for Symptom Term, returns error if the row cant be read correctly

        symptoms.push(symptom); // adds the symptom to the vector

    }

    Ok(symptoms)

}

// searches clinical note for symptoms
fn detect_symptoms<'a>(clinical_note: &str, symptom_terms: &'a [SymptomTerm],) -> Vec<&'a SymptomTerm> {

    let normalized_note = clinical_note.to_lowercase(); // converts entire note to lowercade
    let mut detected_symptoms = Vec::new(); // creates empty vector for symptoms that are found

    for symptom in symptom_terms {

        let preferred_term = symptom.preferred_term.to_lowercase(); // converts preferred term to lowercase
        let preferred_term_found = normalized_note.contains(&preferred_term); // confirms if the main term appears in the notes
        let alias_found = symptom
            .aliases
            .split('|') // splits the aliases whenever a | appears
            .map(str::trim) // removes white spaces
            .filter(|alias| !alias.is_empty()) // removes empty alias values
            .any(|alias| normalized_note.contains(&alias.to_lowercase())); // returns true if one of the aliases is in the note

        if preferred_term_found || alias_found {

            detected_symptoms.push(symptom); // add symptom to the detected list

        }

    }

    detected_symptoms // returns list of detected symptoms

}

// main func
fn main() -> Result<(), Box<dyn Error>> {

    let symptom_file = "data/symptoms.csv"; // path to CSV file
    let symptom_terms = load_symptoms(symptom_file)?; // loads all symptom rows
    let clinical_note = "Patient reports chest pressure, can faint, and has trouble breathing"; // default clinical note

    println!("Clinical Note:");
    println!("{clinical_note}");
    println!();

    let detected_symptoms = detect_symptoms(clinical_note, &symptom_terms); // searches clinical note for symptoms

    // checks to see if no symptoms were found
    if detected_symptoms.is_empty() {

        println!("No symptoms detected");

    // if symptoms are found
    } else {

        println!("Detected Symptoms:");

        // iterate through every symptom
        for symptom in detected_symptoms {

            // prints name and category
            println!("- {} ({})", symptom.preferred_term, symptom.category);

        }

    }

    // returns success from main func
    Ok(())

}