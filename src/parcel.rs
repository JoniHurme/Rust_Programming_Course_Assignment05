// 1. Define Parcel Statuses
pub enum ParcelStatus {
    Preparing,
    InTransit {courier: String},
    Delivered {recipient: String},
    Returned {reason: String},
}

// 2. Describe Every Status
pub fn status_message(status: ParcelStatus) -> String {
    match status {
        ParcelStatus::Preparing => "Parcel is preparing".to_string(),
        ParcelStatus::InTransit {courier} => format!("Parcel is in transit with courier {}", courier),
        ParcelStatus::Delivered {recipient} => format!("Parcel is delivered to {}", recipient),
        ParcelStatus::Returned {reason} => format!("Parcel is returned due to {}", reason),
    }
}

// 3. Print an Optional Delivery Note
pub fn print_delivery_note(note: Option<&str>) {

    if let Some(delivery_note) = note {
        println!("Delivery note: {}", delivery_note)
    } else {
        println!("Parcel not delivered. Can't print delivery note.")
    }
}

// 4. Announce Delivered Parcels
pub fn announce_delivery(status: &ParcelStatus) {

    if let ParcelStatus::Delivered {recipient} = status {
        println!("Parcel delivered to: {}", recipient)
    } else {
        println!("Can not announce delivery. Parcel has not been delivered yet.")
    }
}

// 5. Create a Delivery Receipt
pub fn create_receipt(status: ParcelStatus) {

    let ParcelStatus::Delivered {recipient} = status else {
        println!("Can not create receipt. Parcel has not been delivered yet.");
        return;
    };
    println!("Parcel delivered. Receipt printed to: {}", recipient);
}