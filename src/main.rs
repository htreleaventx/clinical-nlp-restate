fn detect_symptoms(note: &str) -> Vec<&str> {
    let note_lowercase = note.to_lowercase();
    let mut symptoms = Vec::new();

    if note_lowercase.contains("chest pain") {
        symptoms.push("chest pain");
    }

    if note_lowercase.contains("shortness of breath") {
        symptoms.push("shortness of breath");
    }

    if note_lowercase.contains("fever") {
        symptoms.push("fever");
    }

    symptoms
}

fn main() {
    let clinical_note =
        "Patient reports chest pain and shortness of breath.";

    println!("Clinical note:");
    println!("{clinical_note}\n");

    let symptoms = detect_symptoms(clinical_note);

    println!("Detected symptoms:");

    for symptom in symptoms {
        println!("- {symptom}");
    }
}