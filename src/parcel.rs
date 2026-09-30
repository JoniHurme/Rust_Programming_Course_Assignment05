

pub enum ParcelStatus {
    Preparing,
    InTransit {courier: String},
    Delivered {recipient: String},
    Returned {reason: String},
}

pub fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => "Parcel is preparing".to_string(),
        ParcelStatus::InTransit {courier} => format!("Parcel is in transit with courier {}", courier),
        ParcelStatus::Delivered {recipient} => format!("Parcel is delivered to {}", recipient),
        ParcelStatus::Returned {reason} => format!("Parcel is returned due to {}", reason),
    }
}

pub fn print_delivery_note(note: Option<&str>) {


    if let Some(ParcelStatus) = note{
        println!("The note is some")
    } else {
        println!("The note is none")
    }
}


pub fn announce_delivery(status: &ParcelStatus) {


}

// pub fn create_receipt(status: ParcelStatus) {
//
// }